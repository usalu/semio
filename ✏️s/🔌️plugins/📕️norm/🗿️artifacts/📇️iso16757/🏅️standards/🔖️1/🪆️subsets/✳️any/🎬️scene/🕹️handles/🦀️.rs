//! 🕹️ Handles: the drag grips of a selection (the primitive dimensions of the shape and the space envelopes). Each grip is a one-parameter `s`, the coordinate along its axis in object space, written to one leaf of a space envelope (`geometry.objects.ID.spaces[id=SPACE].bounds.max[axis]`) and committed as one undoable `setFields`.

use super::kit::*;
use super::layout::{Layout, Placement, Pose};
use super::model::part_2::{GeometryNode, GeometryObject, SpaceEnvelope};
use super::model::Iso16757Snapshot;
use super::scene::projection_kit::handle_kit::{grid, handle, field_binding as binding, round, HandleSpec as Spec};
use super::scene::*;

const AXES: [(&str, usize, &str, &str); 3] = [("width", 0, "Width of {} envelope", "Breite der Hülle {}"), ("height", 1, "Height of {} envelope", "Höhe der Hülle {}"), ("depth", 2, "Depth of {} envelope", "Tiefe der Hülle {}")];

fn space_handles(id: &str, space: &SpaceEnvelope, pose: &Pose) -> Vec<Handle> {
    let entity = sub_ref(EntityKind::Solid, &format!("{id}:{SPACE_TAG}{}", space.id), format!("geometry.objects.{id}.spaces[id={}]", space.id));
    let (lo, hi) = (space.bounds.min, space.bounds.max);
    let mid = [0.5 * (lo[0] + hi[0]), 0.5 * (lo[1] + hi[1]), 0.5 * (lo[2] + hi[2])];
    AXES.iter()
        .map(|(tag, axis, en, de)| {
            let direction = Pose::dir(*axis);
            let mut anchor = mid;
            anchor[*axis] = hi[*axis];
            handle(Spec {
                entity: &entity,
                tag,
                kind: HandleKind::ScalarAlong { axis: direction },
                anchor: pose.at(anchor[0], anchor[1], anchor[2]),
                grip: GripStyle::Arrow,
                writes: vec![binding(format!("geometry.objects.{id}.spaces[id={}].bounds.max[{axis}]", space.id), Affine::IDENTITY, "m", round(0.001))],
                snap: grid(0.005),
                limits: (lo[*axis] + 0.001, lo[*axis] + 100.0),
                name: loc(&en.replace("{}", &space.id), &de.replace("{}", &space.id)),
            })
        })
        .collect()
}

/// 📐️ One dimension grip of a primitive: the parameter written, the object axis it moves along, its anchor and the affine `value = (s − origin) / scale` of that parameter.
struct Grip {
    name: &'static str,
    axis: usize,
    anchor: [f64; 3],
    origin: f64,
    scale: f64,
    en: &'static str,
    de: &'static str,
}

/// 📐️ The dimension grips of a primitive standing at `o` (translation only), in the schema convention of [`model::part_2::solid`]: box width x, height y, depth z from `o`; cylinder and sphere extend `2r` from `o` in the plane of their radius.
fn dimension_grips(kind: &str, p: &dyn Fn(&str) -> f64, o: [f64; 3]) -> Vec<Grip> {
    let at = |dx: f64, dy: f64, dz: f64| [o[0] + dx, o[1] + dy, o[2] + dz];
    let (w, h, d, r) = (p("width"), p("height"), p("depth"), p("radius"));
    match kind {
        "box" => vec![
            Grip { name: "width", axis: 0, anchor: at(w, 0.5 * h, 0.5 * d), origin: o[0], scale: 1.0, en: "Width of {}", de: "Breite von {}" },
            Grip { name: "height", axis: 1, anchor: at(0.5 * w, h, 0.5 * d), origin: o[1], scale: 1.0, en: "Height of {}", de: "Höhe von {}" },
            Grip { name: "depth", axis: 2, anchor: at(0.5 * w, 0.5 * h, d), origin: o[2], scale: 1.0, en: "Depth of {}", de: "Tiefe von {}" },
        ],
        "cylinder" => vec![
            Grip { name: "radius", axis: 0, anchor: at(2.0 * r, 0.5 * h, r), origin: o[0], scale: 2.0, en: "Radius of {}", de: "Radius von {}" },
            Grip { name: "height", axis: 1, anchor: at(r, h, r), origin: o[1], scale: 1.0, en: "Height of {}", de: "Höhe von {}" },
        ],
        "sphere" => vec![Grip { name: "radius", axis: 0, anchor: at(2.0 * r, r, r), origin: o[0], scale: 2.0, en: "Radius of {}", de: "Radius von {}" }],
        _ => Vec::new(),
    }
}

fn shape_handles(object: &GeometryObject, pose: &Pose) -> Vec<Handle> {
    let (mut path, mut offset, mut node) = (String::from("shape"), [0.0; 3], object.shape.as_ref());
    while let Some(GeometryNode::Transform { translation, rotation_deg, child }) = node {
        if rotation_deg.iter().any(|a| *a != 0.0) {
            return Vec::new();
        }
        path.push_str(".child");
        (0..3).for_each(|axis| offset[axis] += translation[axis]);
        node = Some(child);
    }
    let Some(GeometryNode::Primitive { kind, parameters }) = node else { return Vec::new() };
    let value = |name: &str| parameters.get(name).copied().unwrap_or(0.0).max(0.0);
    let entity = object_ref(&object.id);
    dimension_grips(kind, &value, offset)
        .into_iter()
        .map(|g| {
            handle(Spec {
                entity: &entity,
                tag: &format!("shape-{}", g.name),
                kind: HandleKind::ScalarAlong { axis: Pose::dir(g.axis) },
                anchor: pose.at(g.anchor[0], g.anchor[1], g.anchor[2]),
                grip: GripStyle::Arrow,
                writes: vec![binding(format!("geometry.objects.{}.{path}.parameters.{}", object.id, g.name), Affine { base: -g.origin / g.scale, k: 1.0 / g.scale }, "m", round(0.001))],
                snap: grid(0.005),
                limits: (g.origin + 0.001 * g.scale, g.origin + 100.0 * g.scale),
                name: loc(&g.en.replace("{}", &object.id), &g.de.replace("{}", &object.id)),
            })
        })
        .collect()
}

fn object_handles(object: &GeometryObject, pose: &Pose) -> Vec<Handle> {
    shape_handles(object, pose).into_iter().chain(object.spaces.iter().flat_map(|space| space_handles(&object.id, space, pose))).collect()
}

/// 🕹️ The grips of the selected entities: the space envelopes of a selected object, or of the object a selected space belongs to.
pub fn handles(document: &Iso16757Snapshot, placements: &[Placement], selection: &[EntityRef]) -> Vec<Handle> {
    let layout = Layout::of(document, placements);
    let mut out: Vec<Handle> = Vec::new();
    for entity in selection {
        let base = base_id(&entity.id);
        let (Some(object), Some(pose)) = (document.geometry.objects.get(base), layout.pose(base)) else { continue };
        let all = object_handles(object, pose);
        let wanted = match entity.id.split_once(':') {
            Some((_, tag)) if tag.starts_with(SPACE_TAG) => all.into_iter().filter(|h| h.entity.id == entity.id).collect(),
            _ => all,
        };
        for h in wanted {
            if !out.iter().any(|known| known.id == h.id) {
                out.push(h);
            }
        }
    }
    out
}
