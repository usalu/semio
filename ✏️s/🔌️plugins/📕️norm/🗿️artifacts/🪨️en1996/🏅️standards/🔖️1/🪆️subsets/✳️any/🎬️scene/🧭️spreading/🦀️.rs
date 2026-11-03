//! 🧭️ Spreading: the load spread of every concentrated load as a 3D wedge through the wall (EN 1996-1-1, 6.1.3): the bearing length at the top widens at 60° to the horizontal, to the effective length l_efm at mid-height, clipped to the wall, extruded through the thickness (the flat face view is `spread-…` in the loads module); the wedge carries the utilisation of the bearing check.

use super::kit::*;
use super::layout::{Layout, WallPose, EPS};
use super::masonry as m;
use super::model::MasonryWall;
use super::scene::*;
use super::SceneInput;

const SPREAD_OPACITY: f64 = 0.3;
const BEARING_DEPTH: f64 = 0.05;

/// 📐️ Horizontal extent of the spread at the depth `drop` below the bearing: the bearing length plus `2 drop / tan 60°` on both sides, clipped to the wall length.
pub fn spread_extent(centre: f64, bearing: f64, drop: f64, length: f64) -> (f64, f64) {
    let reach = bearing * 0.5 + drop / 3f64.sqrt();
    ((centre - reach).max(0.0), (centre + reach).min(length))
}

/// 📐️ The bearing area A_b, the effective area A_ef = l_efm·t (60° spread to mid-height, clipped to the effective wall length) and the enhancement β of Eq. 6.10 of a concentrated load, the same terms the check evaluates.
pub fn bearing_terms(wall: &MasonryWall, c: &super::model::ConcentratedLoad) -> (f64, f64, f64) {
    let ab = if c.bearing_area_m2 > 0.0 { c.bearing_area_m2 } else { c.bearing_length_m * wall.thickness_m };
    let a_ef = (c.bearing_length_m + wall.height_m / 3f64.sqrt()).min(m::effective_length_m(wall)) * wall.thickness_m;
    (ab, a_ef, m::concentrated_beta(c.distance_a1_m.max(0.0), wall.height_m, ab, a_ef))
}

fn wedge(wall: &MasonryWall, pose: &WallPose, centre: f64, bearing: f64) -> Option<(MeshBuilder, f64)> {
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    let level = |drop: f64| spread_extent(centre, bearing, drop, l);
    let mut mesh = MeshBuilder::default();
    mesh.prism(pose, level(0.5 * h), level(0.0), (0.0, t), (0.5 * h, h));
    let mid = level(0.5 * h);
    (!mesh.is_empty()).then_some((mesh, mid.1 - mid.0))
}

pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for wall in &input.document.walls {
        if !out.wants(&wall.id) {
            continue;
        }
        let Some(pose) = layout.pose(&wall.id) else { continue };
        if wall.length_m <= EPS || wall.height_m <= EPS || wall.thickness_m <= EPS {
            continue;
        }
        for lc in &wall.load_cases {
            for c in lc.concentrated.iter().filter(|c| c.bearing_length_m > EPS) {
                let centre = c.position_m.clamp(0.0, wall.length_m);
                let Some((mesh, mid)) = wedge(wall, pose, centre, c.bearing_length_m) else { continue };
                let points = mesh.points().to_vec();
                let id = format!("{}:{WEDGE_TAG}{}-{}", wall.id, lc.id, c.id);
                let mut spec = Spec::new(EntityKind::Solid, id.clone(), mesh.solid(), MaterialRole::Load { kind: LoadKind::Permanent }, LAYER_PATTERNS, loc("Load spreading", "Lastausbreitung"));
                spec.nested = format!(".loadCases[id={}].concentrated[id={}]", lc.id, c.id);
                let (ab, a_ef, beta) = bearing_terms(wall, c);
                spec.detail = loc(&format!("Load {} spread at 60° from the {:.0} mm bearing to l_efm = {:.2} m at mid-height: A_b = {ab:.4} m², A_ef = {a_ef:.4} m², A_b/A_ef = {:.2}, a1 = {:.2} m, β = {beta:.3}", c.id, c.bearing_length_m * 1000.0, a_ef / wall.thickness_m, ab / a_ef.max(1e-9), c.distance_a1_m), &format!("Last {} breitet sich unter 60° vom {:.0} mm langen Auflager auf l_efm = {:.2} m in Wandmitte aus: A_b = {ab:.4} m², A_ef = {a_ef:.4} m², A_b/A_ef = {:.2}, a1 = {:.2} m, β = {beta:.3}", c.id, c.bearing_length_m * 1000.0, a_ef / wall.thickness_m, ab / a_ef.max(1e-9), c.distance_a1_m));
                spec.binding = binding_of(input.report, &format!("en1996.6.1.3.concentrated.{}.{}.{}", wall.id, lc.id, c.id));
                spec.opacity = SPREAD_OPACITY;
                spec.channel = Some(ColourChannel::Utilisation);
                spec.handles = super::handles::ids(input.document, layout, &id);
                out.push(spec, &points);
                let width = (ab / c.bearing_length_m).min(wall.thickness_m);
                let (x0, x1) = ((centre - 0.5 * c.bearing_length_m).max(0.0), (centre + 0.5 * c.bearing_length_m).min(wall.length_m));
                let mut bearing = MeshBuilder::default();
                bearing.prism(pose, (x0, x1), (x0, x1), (0.5 * (wall.thickness_m - width), 0.5 * (wall.thickness_m + width)), (wall.height_m, wall.height_m + BEARING_DEPTH));
                let bearing_points = bearing.points().to_vec();
                let bearing_id = format!("{}:{BEARING_TAG}{}-{}", wall.id, lc.id, c.id);
                let mut area = Spec::new(EntityKind::Solid, bearing_id.clone(), bearing.solid(), MaterialRole::Load { kind: LoadKind::Permanent }, LAYER_PATTERNS, loc("Bearing area", "Auflagerfläche"));
                area.nested = format!(".loadCases[id={}].concentrated[id={}]", lc.id, c.id);
                area.detail = loc(&format!("Bearing area A_b = {ab:.4} m² ({:.0} mm × {:.0} mm) against 0.45 A_ef = {:.4} m²", c.bearing_length_m * 1000.0, width * 1000.0, 0.45 * a_ef), &format!("Auflagerfläche A_b = {ab:.4} m² ({:.0} mm × {:.0} mm) gegen 0,45 A_ef = {:.4} m²", c.bearing_length_m * 1000.0, width * 1000.0, 0.45 * a_ef));
                area.binding = binding_of(input.report, &format!("en1996.6.1.3.area.{}.{}.{}", wall.id, lc.id, c.id));
                area.channel = Some(ColourChannel::Utilisation);
                area.handles = super::handles::ids(input.document, layout, &bearing_id);
                out.push(area, &bearing_points);
                let from = pose.at(centre - 0.5 * mid, 0.5 * wall.thickness_m, 0.5 * wall.height_m);
                out.annotate(Annotation::Dimension { id: format!("{}:{WEDGE_TAG}{}-{}-lefm", wall.id, lc.id, c.id), from: from.clone(), to: pose.at(centre + 0.5 * mid, 0.5 * wall.thickness_m, 0.5 * wall.height_m), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, mid), binding: None }, &from);
            }
        }
    }
}

/// 🕹️ The handles of a spread wedge: the bearing length (the spread follows at 60°), the load position along the wall and the distance a1 to the wall end.
pub(crate) fn handles(wall: &MasonryWall, pose: &WallPose, tag: &str) -> Vec<Handle> {
    let key = tag.trim_start_matches(WEDGE_TAG).trim_start_matches(BEARING_TAG);
    for lc in &wall.load_cases {
        for c in &lc.concentrated {
            if key != format!("{}-{}", lc.id, c.id) {
                continue;
            }
            let entity = EntityRef::new(EntityKind::Solid, format!("{}:{tag}", wall.id), format!("walls[id={}].loadCases[id={}].concentrated[id={}]", wall.id, lc.id, c.id));
            let path = format!("walls[id={}].loadCases[id={}].concentrated[id={}].positionM", wall.id, lc.id, c.id);
            let edge = (c.position_m - 0.5 * c.bearing_length_m).max(0.0);
            let x = c.position_m.clamp(0.0, wall.length_m);
            let length_path = format!("walls[id={}].loadCases[id={}].concentrated[id={}].bearingLengthM", wall.id, lc.id, c.id);
            let a1_path = format!("walls[id={}].loadCases[id={}].concentrated[id={}].distanceA1M", wall.id, lc.id, c.id);
            return vec![
                super::members::grip(&entity, "bearing-length", HandleKind::ScalarAlong { axis: pose.axis_x() }, pose.at(x + 0.5 * c.bearing_length_m, 0.5 * wall.thickness_m, wall.height_m), GripStyle::Arrow, super::members::field(length_path, Affine { base: -2.0 * x, k: 2.0 }, "m", 0.01), 0.05, (x + 0.005, wall.length_m.max(x + 0.01)), loc("Bearing length of the concentrated load", "Auflagerlänge der Einzellast")),
                super::members::grip(&entity, "position", HandleKind::ScalarAlong { axis: pose.axis_x() }, pose.at(c.position_m, 0.5 * wall.thickness_m, wall.height_m), GripStyle::Diamond, super::members::field(path, Affine::IDENTITY, "m", 0.01), 0.05, (0.0, wall.length_m), loc("Load position", "Lastlage")),
                super::members::grip(&entity, "distance-a1", HandleKind::ScalarAlong { axis: pose.axis_x() }, pose.at((edge - c.distance_a1_m).max(0.0), 0.5 * wall.thickness_m, wall.height_m), GripStyle::Arrow, super::members::field(a1_path, Affine { base: -edge, k: -1.0 }, "m", 0.01), 0.05, (0.0, edge.max(0.01)), loc("Distance a1 to the wall end", "Abstand a1 zum Wandende")),
            ];
        }
    }
    Vec::new()
}
