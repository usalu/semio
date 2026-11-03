//! 🕹️ Handles: the drag grips of a selection. Each handle is a one-parameter `s`, the coordinate along its axis measured from the wall origin, mapped to document leaves; releasing it commits all its writes as one undoable `setFields`.
//!
//! 📏️ Wall length, height, thickness and held sides; opening width, height and sill; floor bearing depth and span; load eccentricities at top and bottom; concentrated bearing length; chase position, length and depth; movement joint position and panel spacing; the diaphragm stiffness on a logarithmic law; concentrated load position, lintel bearing length and flange thickness; and one gauge ([`gauges`]) for every other numeric field of the walls, load cases, concentrated loads, ties, tendons, confining columns and bars.

use super::kit::*;
use super::layout::{openings, Layout, WallPose, EPS};
use super::model::{En1996Snapshot, MasonryWall};
use super::scene::*;
use super::scene::projection_kit::handle_kit::{grid, handle, HandleSpec as Spec, field_binding as binding, round};
use super::{diaphragm, members, profile, walls};

#[path = "🎚️gauges/🦀️.rs"]
pub mod gauges;

fn wall_path(wall: &MasonryWall, leaf: &str) -> String {
    format!("walls[id={}].{leaf}", wall.id)
}

fn wall_handles(wall: &MasonryWall, pose: &WallPose) -> Vec<Handle> {
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    let entity = wall_ref(&wall.id);
    let label = &wall.label_en;
    let label_de = &wall.label_de;
    vec![
        handle(Spec { entity: &entity, tag: "length", kind: HandleKind::ScalarAlong { axis: pose.axis_x() }, anchor: pose.at(l, 0.5 * t, 0.5 * h), grip: GripStyle::Arrow, writes: vec![binding(wall_path(wall, "lengthM"), Affine::IDENTITY, "m", round(0.01))], snap: grid(0.05), limits: (0.3, 60.0), name: loc(&format!("Length of {label}"), &format!("Länge von {label_de}")) }),
        handle(Spec { entity: &entity, tag: "height", kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: pose.at(0.5 * l, 0.5 * t, h), grip: GripStyle::Arrow, writes: vec![binding(wall_path(wall, "heightM"), Affine::IDENTITY, "m", round(0.01))], snap: grid(0.05), limits: (0.5, 12.0), name: loc(&format!("Height of {label}"), &format!("Höhe von {label_de}")) }),
        handle(Spec { entity: &entity, tag: "thickness", kind: HandleKind::FaceOffset { normal: pose.axis_y() }, anchor: pose.at(0.5 * l, t, 0.5 * h), grip: GripStyle::Diamond, writes: vec![binding(wall_path(wall, "thicknessM"), Affine::IDENTITY, "m", round(0.005))], snap: grid(0.005), limits: (0.05, 1.2), name: loc(&format!("Thickness of {label}"), &format!("Dicke von {label_de}")) }),
        handle(Spec { entity: &entity, tag: "sides", kind: HandleKind::CountStepper, anchor: pose.at(-0.3, 0.5 * t, 0.5 * h), grip: GripStyle::Ring, writes: vec![binding(wall_path(wall, "supportSides"), Affine::IDENTITY, "1", Some(Quantise::Int))], snap: SnapRule::NONE, limits: (2.0, 4.0), name: loc("Number of held wall sides", "Anzahl gehaltener Wandränder") }),
    ]
}

fn opening_handles(wall: &MasonryWall, pose: &WallPose, opening: &str) -> Vec<Handle> {
    let Some(o) = openings(wall).into_iter().find(|o| o.id == opening) else { return Vec::new() };
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    let entity = EntityRef::new(EntityKind::Plate, format!("{}:{OPENING_TAG}{opening}", wall.id), format!("walls[id={}].openings[id={opening}]", wall.id));
    let path = |leaf: &str| format!("walls[id={}].openings[id={opening}].{leaf}", wall.id);
    let (mid_x, mid_z) = (0.5 * (o.x0 + o.x1), 0.5 * (o.z0 + o.z1));
    vec![
        handle(Spec { entity: &entity, tag: "width", kind: HandleKind::ScalarAlong { axis: pose.axis_x() }, anchor: pose.at(o.x1, 0.5 * t, mid_z), grip: GripStyle::Arrow, writes: vec![binding(path("widthM"), Affine { base: -o.x0, k: 1.0 }, "m", round(0.01))], snap: grid(0.05), limits: (o.x0 + 0.2, l), name: loc("Opening width", "Öffnungsbreite") }),
        handle(Spec { entity: &entity, tag: "height", kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: pose.at(mid_x, 0.5 * t, o.z1), grip: GripStyle::Arrow, writes: vec![binding(path("heightM"), Affine { base: -o.z0, k: 1.0 }, "m", round(0.01))], snap: grid(0.05), limits: (o.z0 + 0.2, h), name: loc("Opening height", "Öffnungshöhe") }),
        handle(Spec { entity: &entity, tag: "position", kind: HandleKind::ScalarAlong { axis: pose.axis_x() }, anchor: pose.at(o.x0, 0.5 * t, mid_z), grip: GripStyle::Diamond, writes: vec![binding(path("positionM"), Affine::IDENTITY, "m", round(0.01))], snap: grid(0.05), limits: (0.0, l), name: loc("Opening position", "Öffnungslage") }),
        handle(Spec { entity: &entity, tag: "sill", kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: pose.at(mid_x, 0.5 * t, o.z0), grip: GripStyle::Diamond, writes: vec![binding(path("sillHeightM"), Affine::IDENTITY, "m", round(0.01))], snap: grid(0.05), limits: (0.0, (h - 0.2).max(0.0)), name: loc("Sill height", "Brüstungshöhe") }),
    ]
}

fn slab_handles(wall: &MasonryWall, pose: &WallPose) -> Vec<Handle> {
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    let a = wall.slab_bearing_depth_m.clamp(0.0, t);
    if a <= EPS {
        return Vec::new();
    }
    let entity = EntityRef::new(EntityKind::Plate, format!("{}:slab-top", wall.id), format!("walls[id={}]", wall.id));
    let mut out = vec![handle(Spec { entity: &entity, tag: "bearing-depth", kind: HandleKind::ScalarAlong { axis: pose.axis_y() }, anchor: pose.at(0.5 * l, t - a, h + 0.1), grip: GripStyle::Arrow, writes: vec![binding(wall_path(wall, "slabBearingDepthM"), Affine { base: t, k: -1.0 }, "m", round(0.005))], snap: grid(0.005), limits: (0.0, t - 0.02), name: loc("Floor bearing depth", "Deckenauflagertiefe") })];
    if let Some(lc) = wall.load_cases.first() {
        let stub = (0.5 * lc.slab_span_m.clamp(0.0, 6.0)).max(0.1);
        out.push(handle(Spec { entity: &entity, tag: "span", kind: HandleKind::ScalarAlong { axis: pose.axis_y() }, anchor: pose.at(0.5 * l, t + stub, h + 0.1), grip: GripStyle::Arrow, writes: vec![binding(format!("walls[id={}].loadCases[id={}].slabSpanM", wall.id, lc.id), Affine { base: -2.0 * t, k: 2.0 }, "m", round(0.05))], snap: grid(0.05), limits: (t, t + 3.0), name: loc("Floor span", "Deckenspannweite") }));
    }
    out
}

fn resultant_handles(document: &En1996Snapshot, wall: &MasonryWall, pose: &WallPose, top: bool) -> Vec<Handle> {
    let Some(p) = profile::governing(document, wall) else { return Vec::new() };
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    let tag = if top { "resultant-top" } else { "resultant-bottom" };
    let entity = EntityRef::new(EntityKind::Load, format!("{}:{tag}", wall.id), format!("walls[id={}]", wall.id));
    let (e, offset, z, leaf, low) = if top { (p.e_top, 0.5 * t + p.e_slab + p.e_theta, h, "eccentricityTopM", 0.5 * t + p.e_slab + p.e_theta) } else { (p.e_bottom, 0.5 * t, 0.0, "eccentricityBottomM", 0.5 * t) };
    let name = if top { loc("Eccentricity of the load at the top", "Ausmitte der Last am Kopf") } else { loc("Eccentricity of the load at the base", "Ausmitte der Last am Fuß") };
    vec![handle(Spec { entity: &entity, tag: "eccentricity", kind: HandleKind::ScalarAlong { axis: pose.axis_y() }, anchor: pose.at(0.5 * l, 0.5 * t + e.max(0.05 * t).min(0.5 * t), z), grip: GripStyle::Diamond, writes: vec![binding(wall_path(wall, leaf), Affine { base: -offset, k: 1.0 }, "m", round(0.001))], snap: grid(0.005), limits: (low.min(t), t), name })]
}

fn concentrated_handles(wall: &MasonryWall, pose: &WallPose, tag: &str) -> Vec<Handle> {
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    for lc in &wall.load_cases {
        for c in lc.concentrated.iter() {
            if tag != format!("{CONCENTRATED_TAG}{}-{}", lc.id, c.id) {
                continue;
            }
            let x = c.position_m.clamp(0.0, l);
            let entity = EntityRef::new(EntityKind::Load, format!("{}:{tag}", wall.id), format!("walls[id={}].loadCases[id={}].concentrated[id={}]", wall.id, lc.id, c.id));
            let path = format!("walls[id={}].loadCases[id={}].concentrated[id={}].bearingLengthM", wall.id, lc.id, c.id);
            let mut out = vec![handle(Spec { entity: &entity, tag: "bearing-length", kind: HandleKind::ScalarAlong { axis: pose.axis_x() }, anchor: pose.at(x + 0.5 * c.bearing_length_m, 0.5 * t, h), grip: GripStyle::Arrow, writes: vec![binding(path, Affine { base: -2.0 * x, k: 2.0 }, "m", round(0.01))], snap: grid(0.05), limits: (x + 0.005, l.max(x + 0.01)), name: loc("Bearing length of the concentrated load", "Auflagerlänge der Einzellast") })];
            let at = format!("walls[id={}].loadCases[id={}].concentrated[id={}].positionM", wall.id, lc.id, c.id);
            out.push(handle(Spec { entity: &entity, tag: "position", kind: HandleKind::ScalarAlong { axis: pose.axis_x() }, anchor: pose.at(x, 0.5 * t, h + 0.1), grip: GripStyle::Diamond, writes: vec![binding(at, Affine::IDENTITY, "m", round(0.01))], snap: grid(0.05), limits: (0.0, l), name: loc("Position of the concentrated load", "Lage der Einzellast") }));
            out.extend(gauges::of_concentrated(wall, pose, lc, c, &entity));
            return out;
        }
    }
    Vec::new()
}

fn chase_handles(document: &En1996Snapshot, wall: &MasonryWall, pose: &WallPose, chase_id: &str) -> Vec<Handle> {
    let Some(chase) = document.chases.iter().find(|c| c.id == chase_id && c.wall_id == wall.id) else { return Vec::new() };
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    let (x0, x1, y0, y1) = {
        let (x0, x1, y0, y1) = super::masonry::chase_extent(chase);
        (x0.clamp(0.0, l), x1.clamp(0.0, l), y0.clamp(0.0, h), y1.clamp(0.0, h))
    };
    let entity = EntityRef::new(EntityKind::Solid, format!("{}:{CHASE_TAG}{}", wall.id, chase.id), format!("chases[id={}]", chase.id));
    let path = |leaf: &str| format!("chases[id={}].{leaf}", chase.id);
    let (mid_x, mid_z) = (0.5 * (x0 + x1), 0.5 * (y0 + y1));
    let horizontal = chase.kind == super::model::ChaseKind::Horizontal;
    let length = if horizontal {
        handle(Spec { entity: &entity, tag: "length", kind: HandleKind::ScalarAlong { axis: pose.axis_x() }, anchor: pose.at(x1, -0.02, mid_z), grip: GripStyle::Arrow, writes: vec![binding(path("lengthM"), Affine { base: -chase.x_m, k: 1.0 }, "m", round(0.01))], snap: grid(0.05), limits: (chase.x_m + 0.05, l.max(chase.x_m + 0.1)), name: loc("Chase length", "Schlitzlänge") })
    } else {
        handle(Spec { entity: &entity, tag: "length", kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: pose.at(mid_x, -0.02, y1), grip: GripStyle::Arrow, writes: vec![binding(path("lengthM"), Affine { base: -chase.y_m, k: 1.0 }, "m", round(0.01))], snap: grid(0.05), limits: (chase.y_m + 0.05, h.max(chase.y_m + 0.1)), name: loc("Chase length", "Schlitzlänge") })
    };
    let width = if horizontal {
        handle(Spec { entity: &entity, tag: "width", kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: pose.at(mid_x, -0.02, y1), grip: GripStyle::Ring, writes: vec![binding(path("widthM"), Affine { base: -chase.y_m, k: 1.0 }, "m", round(0.005))], snap: grid(0.005), limits: (chase.y_m + 0.005, h.max(chase.y_m + 0.01)), name: loc("Chase width", "Schlitzbreite") })
    } else {
        handle(Spec { entity: &entity, tag: "width", kind: HandleKind::ScalarAlong { axis: pose.axis_x() }, anchor: pose.at(x1, -0.02, mid_z), grip: GripStyle::Ring, writes: vec![binding(path("widthM"), Affine { base: -chase.x_m, k: 1.0 }, "m", round(0.005))], snap: grid(0.005), limits: (chase.x_m + 0.005, l.max(chase.x_m + 0.01)), name: loc("Chase width", "Schlitzbreite") })
    };
    vec![
        handle(Spec { entity: &entity, tag: "position-x", kind: HandleKind::ScalarAlong { axis: pose.axis_x() }, anchor: pose.at(x0, -0.02, mid_z), grip: GripStyle::Diamond, writes: vec![binding(path("xM"), Affine::IDENTITY, "m", round(0.01))], snap: grid(0.05), limits: (0.0, l), name: loc("Chase position x", "Schlitzlage x") }),
        handle(Spec { entity: &entity, tag: "position-y", kind: HandleKind::ScalarAlong { axis: Vec3::Z }, anchor: pose.at(mid_x, -0.02, y0), grip: GripStyle::Diamond, writes: vec![binding(path("yM"), Affine::IDENTITY, "m", round(0.01))], snap: grid(0.05), limits: (0.0, h), name: loc("Chase position y", "Schlitzlage y") }),
        handle(Spec { entity: &entity, tag: "depth", kind: HandleKind::ScalarAlong { axis: pose.axis_y() }, anchor: pose.at(mid_x, chase.depth_m.max(0.01), mid_z), grip: GripStyle::Arrow, writes: vec![binding(path("depthM"), Affine::IDENTITY, "m", round(0.005))], snap: grid(0.005), limits: (0.005, t), name: loc("Chase depth", "Schlitztiefe") }),
        length,
        width,
    ]
}

fn joint_handles(document: &En1996Snapshot, wall: &MasonryWall, pose: &WallPose, joint_id: &str) -> Vec<Handle> {
    let Some(joint) = document.movement_joints.iter().find(|j| j.id == joint_id && j.wall_id == wall.id) else { return Vec::new() };
    let (l, h, t) = (wall.length_m, wall.height_m, wall.thickness_m);
    let x = joint.position_m.clamp(0.0, l);
    let before = document.movement_joints.iter().filter(|j| j.wall_id == wall.id && j.id != joint.id && j.position_m < joint.position_m).map(|j| j.position_m).fold(0.0, f64::max);
    let after = document.movement_joints.iter().filter(|j| j.wall_id == wall.id && j.id != joint.id && j.position_m >= joint.position_m).map(|j| j.position_m).fold(l, f64::min);
    let entity = EntityRef::new(EntityKind::Solid, format!("{}:{JOINT_TAG}{}", wall.id, joint.id), format!("movementJoints[id={}]", joint.id));
    let path = format!("movementJoints[id={}].positionM", joint.id);
    vec![
        handle(Spec { entity: &entity, tag: "position", kind: HandleKind::ScalarAlong { axis: pose.axis_x() }, anchor: pose.at(x, 0.5 * t, h), grip: GripStyle::Diamond, writes: vec![binding(path.clone(), Affine::IDENTITY, "m", round(0.01))], snap: grid(0.05), limits: (0.0, l), name: loc("Movement joint position", "Dehnfugenlage") }),
        handle(Spec { entity: &entity, tag: "spacing", kind: HandleKind::ScalarAlong { axis: pose.axis_x() }, anchor: pose.at(x, 0.5 * t, 0.5 * h), grip: GripStyle::Arrow, writes: vec![binding(path, Affine::IDENTITY, "m", round(0.01))], snap: grid(0.05), limits: (before + 0.2, (after - 0.2).max(before + 0.2)), name: loc(&format!("Panel spacing to the previous joint: {:.2} m", x - before), &format!("Feldweite bis zur vorigen Fuge: {:.2} m", x - before)) }),
    ]
}

/// 🧲️ The move and rotate grips of a wall; a drag writes `placements[id=WALL].x|y|yaw`.
fn move_handles(document: &En1996Snapshot, wall: &MasonryWall, pose: &WallPose) -> Vec<Handle> {
    let stored: Vec<_> = document.placements.iter().map(|p| super::Placement { entity_id: p.id.clone(), origin: Vec3::new(p.x, p.y, p.z), yaw: p.yaw }).collect();
    super::scene::projection_kit::handle_kit::move_handles(&wall_ref(&wall.id), &wall.id, &pose.origin, &stored, pose.yaw, true)
}

/// 🕹️ The handles of one document entity or derived scene id, anchored in the given layout.
pub(crate) fn build(document: &En1996Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    if let (Some(row), Some(pose)) = (document.cases.iter().find(|c| c.id == base_id(id)), layout.case(base_id(id))) {
        return super::scene::projection_kit::design_case::forms::handles::<super::schema::checks::masonry::En1996Cases>(row, pose, "cases");
    }
    let Some(wall) = document.walls.iter().find(|w| w.id == base_id(id)) else { return Vec::new() };
    let Some(pose) = layout.pose(&wall.id) else { return Vec::new() };
    let tag = id.split_once(':').map(|(_, tag)| tag);
    match tag {
        None => [wall_handles(wall, pose), move_handles(document, wall, pose), gauges::of_wall(wall, pose)].concat(),
        Some(tag) if walls::ZONES.iter().any(|z| z.0 == tag) => wall_handles(wall, pose),
        Some(tag) if tag.starts_with(OPENING_TAG) => opening_handles(wall, pose, &tag[OPENING_TAG.len()..]),
        Some(tag) if [LINTEL_TAG, PIER_TAG, TIE_TAG, TENDON_TAG, COLUMN_TAG, super::lateral::FLANGE_TAG, BAR_TAG].iter().any(|prefix| tag.starts_with(prefix)) => [members::handles(wall, pose, tag), gauges::of_member(wall, pose, tag)].concat(),
        Some("slab-top") => slab_handles(wall, pose),
        Some("resultant-top") => resultant_handles(document, wall, pose, true),
        Some("resultant-bottom") => resultant_handles(document, wall, pose, false),
        Some(tag) if tag.starts_with(CONCENTRATED_TAG) => concentrated_handles(wall, pose, tag),
        Some(tag) if tag.starts_with(CHASE_TAG) => chase_handles(document, wall, pose, &tag[CHASE_TAG.len()..]),
        Some(tag) if tag.starts_with(JOINT_TAG) => joint_handles(document, wall, pose, &tag[JOINT_TAG.len()..]),
        Some(tag) if tag.starts_with(DIAPHRAGM_TAG) => diaphragm::handles(document, wall, pose, tag),
        Some(tag) if tag.starts_with(WEDGE_TAG) || tag.starts_with(BEARING_TAG) => super::spreading::handles(wall, pose, tag),
        Some(ISOTHERM_TAG) => super::isotherms::handles(wall, pose),
        Some(PHI_TAG | YIELD_TAG | ARCH_TAG) => wall_handles(wall, pose),
        Some(_) => Vec::new(),
    }
}

/// 🆔️ The handle ids a scene entity advertises.
pub(crate) fn ids(document: &En1996Snapshot, layout: &Layout, id: &str) -> Vec<String> {
    build(document, layout, id).into_iter().map(|h| h.id).collect()
}

/// 🕹️ The handles of every selected entity (derived scene ids resolve to their own handle set), in selection order, without duplicates.
pub fn handles(document: &En1996Snapshot, placements: &[super::Placement], selection: &[EntityRef]) -> Vec<Handle> {
    let layout = Layout::of(document, placements);
    let mut out: Vec<Handle> = Vec::new();
    for entity in selection {
        for handle in build(document, &layout, &entity.id) {
            if !out.iter().any(|h| h.id == handle.id) {
                out.push(handle);
            }
        }
    }
    out
}
