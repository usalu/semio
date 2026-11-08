//! 🕹️ Handles: the drag grips of a selection. Each grip is a one-parameter `s`, the coordinate along its axis in object space, written to one leaf of a space envelope (`geometry.objects.ID.spaces[id=SPACE].bounds.max[axis]`) and committed as one undoable `setFields`.

use super::kit::*;
use super::layout::{Layout, Placement, Pose};
use super::model::part_2::{GeometryObject, SpaceEnvelope};
use super::model::Iso16757Snapshot;
use super::scene::projection_kit::handle_kit::{grid, handle, field_binding as binding, round, HandleSpec as Spec};
use super::scene::*;

const AXES: [(&str, usize, &str, &str); 3] = [("width", 0, "Width of {} envelope", "Breite der Hülle {}"), ("depth", 1, "Depth of {} envelope", "Tiefe der Hülle {}"), ("height", 2, "Height of {} envelope", "Höhe der Hülle {}")];

fn space_handles(id: &str, space: &SpaceEnvelope, pose: &Pose) -> Vec<Handle> {
    let entity = sub_ref(EntityKind::Solid, &format!("{id}:{SPACE_TAG}{}", space.id), format!("geometry.objects.{id}.spaces[id={}]", space.id));
    let (lo, hi) = (space.bounds.min, space.bounds.max);
    let mid = [0.5 * (lo[0] + hi[0]), 0.5 * (lo[1] + hi[1]), 0.5 * (lo[2] + hi[2])];
    AXES.iter()
        .map(|(tag, axis, en, de)| {
            let direction = [Vec3::X, Vec3::Y, Vec3::Z][*axis];
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

fn object_handles(object: &GeometryObject, pose: &Pose) -> Vec<Handle> {
    object.spaces.iter().flat_map(|space| space_handles(&object.id, space, pose)).collect()
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
