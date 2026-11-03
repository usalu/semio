//! 🧩️ Bond: the explicit running bond of units and mortar joints of every wall (courses of height `hu + tbed`, perpends offset by half a unit); a unit meeting an opening is cut to the remaining wall rectangles.
//! The overlap is checked against `max(0.4 hu, 45 mm)`; an invalid bond is drawn in error style. Both entities live on the hidden `bond` layer and fall back to the joint lines when the unit budget is spent.

use super::kit::*;
use super::layout::{openings, solid_rects, Layout, EPS};
use super::model::MasonryWall;
use super::scene::*;
use super::SceneInput;

const MIN_OVERLAP: f64 = 0.045;

/// 🧩️ The minimum overlap of units in adjacent courses, `max(0.4 hu, 45 mm)`.
pub fn overlap_need(unit_height: f64) -> f64 {
    (0.4 * unit_height).max(MIN_OVERLAP)
}

fn hits_opening(wall: &MasonryWall, x: (f64, f64), z: (f64, f64)) -> bool {
    openings(wall).iter().any(|o| x.0 < o.x1 - EPS && x.1 > o.x0 + EPS && z.0 < o.z1 - EPS && z.1 > o.z0 + EPS)
}

fn course_count(wall: &MasonryWall) -> usize {
    let (hu, j) = (wall.unit_height_m, wall.bed_joint_thickness_m.max(0.0));
    if hu <= EPS { 0 } else { ((wall.height_m + j) / (hu + j)).floor().max(0.0) as usize }
}

fn units_per_course(wall: &MasonryWall) -> usize {
    let (lu, j) = (wall.unit_length_m, wall.bed_joint_thickness_m.max(0.0));
    if lu <= EPS { 0 } else { (wall.length_m / (lu + j)).ceil() as usize + 1 }
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for wall in &input.document.walls {
        if !out.wants(&wall.id) {
            continue;
        }
        let Some(pose) = layout.pose(&wall.id) else { continue };
        let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
        if l <= EPS || h <= EPS || t <= EPS {
            continue;
        }
        let (hu, lu, j) = (wall.unit_height_m, wall.unit_length_m, wall.bed_joint_thickness_m.max(0.0));
        let courses = course_count(wall);
        if courses == 0 || lu <= EPS {
            continue;
        }
        let inset = (0.25 * t).min(0.01);
        let y = (inset, t - inset);
        let with_units = courses * units_per_course(wall) <= super::UNIT_BUDGET;
        let mut units = MeshBuilder::default();
        let mut mortar = MeshBuilder::default();
        for k in 0..courses {
            let (z0, z1) = (k as f64 * (hu + j), k as f64 * (hu + j) + hu);
            if k + 1 < courses && j > EPS {
                for (a, b, c, d) in solid_rects(wall, z1, z1 + j) {
                    mortar.cuboid(pose, (a, b), y, (c, d));
                }
            }
            if !with_units {
                continue;
            }
            let strips = solid_rects(wall, z0, z1);
            let mut x = if k % 2 == 1 { -0.5 * (lu + j) } else { 0.0 };
            while x < l - EPS {
                let (x0, x1) = (x.max(0.0), (x + lu).min(l));
                if x1 - x0 > 1e-3 {
                    for &(a, b, c, d) in &strips {
                        let (p, q) = (x0.max(a), x1.min(b));
                        if q - p > 1e-3 {
                            units.cuboid(pose, (p, q), (0.0, t), (c, d));
                        }
                    }
                    if x + lu + j < l - EPS && j > EPS && !hits_opening(wall, (x1, x1 + j), (z0, z1)) {
                        mortar.cuboid(pose, (x1, x1 + j), y, (z0, z1));
                    }
                }
                x += lu + j;
            }
        }
        let valid = 0.5 * lu >= overlap_need(hu) - EPS;
        if with_units && !units.is_empty() {
            let points = units.points().to_vec();
            let mut spec = Spec::new(EntityKind::Solid, format!("{}:units", wall.id), units.solid(), MaterialRole::Masonry, LAYER_BOND, loc("Masonry units in running bond", "Mauersteine im Läuferverband"));
            spec.detail = loc(&format!("Units {:.0} × {:.0} mm, overlap {:.0} mm against the required {:.0} mm", lu * 1000.0, hu * 1000.0, 0.5 * lu * 1000.0, overlap_need(hu) * 1000.0), &format!("Steine {:.0} × {:.0} mm, Überbindung {:.0} mm bei geforderten {:.0} mm", lu * 1000.0, hu * 1000.0, 0.5 * lu * 1000.0, overlap_need(hu) * 1000.0));
            spec.binding = binding_of(input.report, &format!("en1996.8.1.5.unit-fit.{}", wall.id));
            spec.channel = Some(if valid { ColourChannel::Role } else { ColourChannel::Status });
            spec.error = !valid;
            out.push(spec, &points);
        }
        if !mortar.is_empty() {
            let points = mortar.points().to_vec();
            let spec = Spec::new(EntityKind::Solid, format!("{}:mortar", wall.id), mortar.solid(), MaterialRole::Neutral, LAYER_BOND, loc("Mortar joints", "Mörtelfugen"));
            out.push(spec, &points);
        }
    }
}
