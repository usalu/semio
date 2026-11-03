//! 🎞️ Tracks: the fire-time track of the EN 1996 scene; each keyframe binds the field `fire-demand@R`, the tabulated minimum thickness of EN 1996-1-2 for the fire class R over the actual wall thickness, on a thin face overlay of every wall with a fire requirement.

use super::kit::*;
use super::layout::{Layout, WallPose};
use super::masonry as m;
use super::model::MasonryWall;
use super::scene::projection_kit::tracks as kit_tracks;
use super::scene::*;
use super::SceneInput;

pub const FIRE_CLASSES: [u32; 6] = [30, 60, 90, 120, 180, 240];
pub const MU_FI: f64 = 0.7;
const COLUMNS: usize = 9;
const ROWS: usize = 5;
const LIFT: f64 = 0.004;

/// 🔥️ Ratio of the tabulated minimum wall thickness for fire class `rei` to the wall thickness; above one the wall is too thin for that class.
pub fn fire_demand(wall: &MasonryWall, rei: u32) -> f64 {
    m::fire_min_thickness_m(rei, wall.unit_material) / wall.thickness_m
}

fn overlay(wall: &MasonryWall, pose: &WallPose, out: &mut Out) {
    let (l, h) = (wall.length_m, wall.height_m);
    let points: Vec<Vec3> = (0..ROWS).flat_map(|r| (0..COLUMNS).map(move |c| (r, c))).map(|(r, c)| pose.at(l * c as f64 / (COLUMNS - 1) as f64, -LIFT, h * r as f64 / (ROWS - 1) as f64)).collect();
    let grid = PointGrid { rows: ROWS as u32, columns: COLUMNS as u32, points: points.clone() };
    let fields: Vec<ScalarField> = FIRE_CLASSES.iter().map(|&rei| ScalarField { id: format!("fire-demand@{rei}"), unit: QuantityKind::Dimensionless, samples: FieldSamples::Vertex { values: vec![fire_demand(wall, rei); ROWS * COLUMNS] } }).collect();
    let name = loc(&format!("{} – fire thickness demand", wall.label_en), &format!("{} – Brandschutz-Dickenbedarf", wall.label_de));
    let mut spec = Spec::new(EntityKind::Surface, format!("{}:fire-field", wall.id), Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid } }), MaterialRole::Neutral, LAYER_FIRE, name);
    spec.detail = loc("tabulated minimum thickness over wall thickness for the fire class R (EN 1996-1-2)", "tabellierte Mindestdicke bezogen auf die Wanddicke für die Feuerwiderstandsklasse R (EN 1996-1-2)");
    spec.channel = Some(ColourChannel::Fields { active: fields[0].id.clone(), fields });
    spec.opacity = 0.75;
    out.push(spec, &points);
}

/// 🎬️ Adds the fire overlay of every wall with a fire class and the `fire` track that scrubs the class R.
pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let mut any = false;
    for wall in input.document.walls.iter().filter(|w| w.fire_rei_min > 0 && w.thickness_m > 1e-6) {
        if let Some(pose) = layout.pose(&wall.id) {
            any = true;
            if out.wants(&wall.id) {
                overlay(wall, pose, out);
            }
        }
    }
    if !any {
        return;
    }
    out.global();
    let stops: Vec<(f64, String, String)> = FIRE_CLASSES.iter().map(|r| (f64::from(*r) * 60.0, format!("fire-demand@{r}"), format!("R {r}"))).collect();
    let rows: Vec<(f64, &str, &str, &str)> = stops.iter().map(|(at, id, name)| (*at, id.as_str(), name.as_str(), name.as_str())).collect();
    if let Some(track) = kit_tracks::timed("fire", TrackKind::Time, ("Fire resistance class", "Feuerwiderstandsklasse"), &rows) {
        out.track(track);
    }
}
