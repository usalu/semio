//! 📍️ Placement drag: the release of a grip bound to `placements[id=ENTITY].leaf` is ONE typed commit, an insert of the row when the entity stands in its derived layout pose, else one set. The preview carries the complete pose, so the live scene paints the same row the release inserts. See [`super`] and the shared [`PlacementCommit`](semio_s_plugin_norm_structural_scene::projection_kit::PlacementCommit).

use crate::{is_view_write, resolve_set_fields, validate_writes, FieldWrite, NormEdit};
use semio_framework_value::{DslValue, ToValue};
use semio_s_plugin_norm_structural_scene::projection_kit::{placed, placement_target, placements_of, Placement, PlacementCommit, PlacementOp, PlacementWrite, PLACEMENTS_FIELD};
use semio_s_plugin_norm_structural_scene::{Geometry, Handle, StructuralScene, Vec3};

/// 🔎️ Whether a write addresses a placement leaf (`placements[id=ENTITY].x|y|z|yaw`).
pub fn is_placement_write(write: &FieldWrite) -> bool {
    placement_target(&write.path).is_some()
}

/// 🧭️ The yaw (rad about `+Z`) the scene derives for entity `id` when no placement row says otherwise: the heading of its frame `x` axis on the ground plane (members and plates); `None` when the scene has no oriented body for it or the axis is vertical.
pub fn derived_yaw(scene: &StructuralScene, id: &str) -> Option<f64> {
    let frame = scene.entities.iter().find(|entity| entity.entity.id == id).and_then(|entity| match &entity.geometry {
        Geometry::Member(member) => Some(&member.frame),
        Geometry::Plate(plate) => Some(&plate.frame),
        _ => None,
    })?;
    (frame.x.x.hypot(frame.x.y) > 1.0e-9).then(|| frame.x.y.atan2(frame.x.x))
}

/// 🧍️ The pose each entity stands in when it has no row: the anchor of its placement grips with the grip bases laid over (the persisted value, else the derived layout value), and the yaw of its derived projection (`derived_yaw`, else zero) unless a yaw grip says otherwise, so the first drag never resets a derived yaw.
pub fn standing_pose(handles: &[Handle], id: &str, derived_yaw: &dyn Fn(&str) -> Option<f64>) -> Option<Placement> {
    let mut pose: Option<Placement> = None;
    for handle in handles {
        for binding in &handle.writes {
            let Some((target, leaf)) = placement_target(&binding.path) else { continue };
            if target != id {
                continue;
            }
            let row = pose.get_or_insert_with(|| Placement { entity_id: id.to_string(), origin: handle.anchor.clone(), yaw: derived_yaw(id).unwrap_or(0.0) });
            match leaf {
                "x" => row.origin.x = binding.map.base,
                "y" => row.origin.y = binding.map.base,
                "z" => row.origin.z = binding.map.base,
                "yaw" => row.yaw = binding.map.base,
                _ => {}
            }
        }
    }
    pose
}

fn pose_leaf(row: &Placement, leaf: &str) -> f64 {
    match leaf {
        "x" => row.origin.x,
        "y" => row.origin.y,
        "z" => row.origin.z,
        _ => row.yaw,
    }
}

/// 🧾️ The row value an insert writes (`{id, x, y, z, yaw}`).
pub fn placement_row_value(row: &Placement) -> DslValue {
    DslValue::object(vec![
        ("id".to_string(), DslValue::String(row.entity_id.clone())),
        ("x".to_string(), DslValue::float(row.origin.x)),
        ("y".to_string(), DslValue::float(row.origin.y)),
        ("z".to_string(), DslValue::float(row.origin.z)),
        ("yaw".to_string(), DslValue::float(row.yaw)),
    ])
}

fn placement_writes(writes: &[FieldWrite]) -> Vec<PlacementWrite> {
    writes.iter().filter_map(|write| write.value.as_f64().filter(|_| is_placement_write(write)).map(|value| PlacementWrite { path: write.path.clone(), value })).collect()
}

/// ✍️ The typed edits of one placement release against the `rows` of the document: an entity without a row is ONE `insertItem` carrying its standing pose with the dragged leaves, an entity with a row gets one `setField` per changed leaf, a pose equal to the row writes nothing.
pub fn placement_edits(rows: &[Placement], standing: &dyn Fn(&str) -> Option<Placement>, writes: &[FieldWrite]) -> Vec<NormEdit> {
    let mut next = rows.len();
    PlacementCommit::of_writes(rows, standing, &placement_writes(writes))
        .ops
        .into_iter()
        .flat_map(|op| match op {
            PlacementOp::Insert(row) => {
                next += 1;
                vec![NormEdit::InsertItem { path: PLACEMENTS_FIELD.to_string(), index: next - 1, value: Some(placement_row_value(&row)) }]
            }
            PlacementOp::Set(leaves) => leaves
                .into_iter()
                .filter(|leaf| placement_target(&leaf.path).is_some_and(|(id, name)| placed(rows, id).is_none_or(|row| pose_leaf(row, name) != leaf.value)))
                .map(|leaf| NormEdit::SetField { path: leaf.path, value: DslValue::float(leaf.value) })
                .collect(),
            PlacementOp::Remove(id) => rows.iter().position(|row| row.entity_id == id).map(|index| NormEdit::RemoveItem { path: PLACEMENTS_FIELD.to_string(), index }).into_iter().collect(),
        })
        .collect()
}

/// 📦️ [`resolve_set_fields`] with the placement leaves routed through [`placement_edits`], all-or-nothing: the placement edits come first, then the plain leaves, resolved against the same document so the bundle stays one undo step.
pub fn resolve_placed_fields<D, M, F>(document: &D, writes: &[FieldWrite], standing: &dyn Fn(&str) -> Option<Placement>, mut resolve: F) -> Result<Vec<M>, String>
where
    D: ToValue,
    F: FnMut(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    validate_writes(writes)?;
    let (placed_writes, plain): (Vec<FieldWrite>, Vec<FieldWrite>) = writes.iter().filter(|write| !is_view_write(write)).cloned().partition(is_placement_write);
    let mut mutations = Vec::new();
    for edit in placement_edits(&placements_of(document), standing, &placed_writes) {
        mutations.extend(resolve(document, &edit)?);
    }
    if !plain.is_empty() {
        mutations.extend(resolve_set_fields(document, &plain, &mut resolve)?);
    }
    Ok(mutations)
}

/// 📤️ [`resolve_placed_fields`] as ONE emit: the release of a drag is one undo step, rows inserted and leaves set together.
pub fn handle_placed_fields<D, M, F>(document: &D, writes: &[FieldWrite], standing: &dyn Fn(&str) -> Option<Placement>, resolve: F) -> Result<semio_framework_plugin::Emit<M, semio_framework_plugin::NoConfigMutation>, semio_framework_plugin::Fault>
where
    D: ToValue,
    F: FnMut(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    let mutations = resolve_placed_fields(document, writes, standing, resolve).map_err(|error| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("norm.edit-unresolved"), error))?;
    crate::commit_snapshot_fields(mutations)
}

/// 🫧️ A drag preview made self-contained: the writes of an entity without a row gain the missing leaves of its standing pose, so the overlay document inserts the same row the release commits.
pub fn complete_preview(rows: &[Placement], standing: &dyn Fn(&str) -> Option<Placement>, writes: Vec<FieldWrite>) -> Vec<FieldWrite> {
    let mut out = writes.clone();
    for id in writes.iter().filter_map(|write| placement_target(&write.path).map(|(id, _)| id.to_string())).collect::<std::collections::BTreeSet<_>>() {
        if placed(rows, &id).is_some() {
            continue;
        }
        let Some(pose) = standing(&id) else { continue };
        for leaf in ["x", "y", "z", "yaw"] {
            let path = format!("{PLACEMENTS_FIELD}[id={id}].{leaf}");
            if !out.iter().any(|write| write.path == path) {
                out.push(FieldWrite::number(path, pose_leaf(&pose, leaf)));
            }
        }
    }
    out
}

/// 👁️ Lays `writes` over `tree`, inserting the row of an entity that has none from its four leaves (a partial leaf set of a missing row is an error).
pub fn overlay_writes(tree: &mut DslValue, writes: &[FieldWrite]) -> Result<(), String> {
    for write in writes {
        let Some((id, _)) = placement_target(&write.path) else {
            crate::set_value_at_path(tree, &write.path, write.value.clone())?;
            continue;
        };
        if crate::get_value_at_path(tree, &write.path).is_err() {
            let leaf = |name: &str| writes.iter().find(|w| w.path == format!("{PLACEMENTS_FIELD}[id={id}].{name}")).and_then(|w| w.value.as_f64()).ok_or_else(|| format!("the preview of '{id}' lacks the leaf '{name}' of its first placement row"));
            let row = Placement { entity_id: id.to_string(), origin: Vec3::new(leaf("x")?, leaf("y")?, leaf("z")?), yaw: leaf("yaw")? };
            let count = tree.get(PLACEMENTS_FIELD).and_then(DslValue::as_array).map_or(0, |items| items.len());
            crate::insert_value_at_path(tree, PLACEMENTS_FIELD, count, Some(placement_row_value(&row)))?;
        }
        crate::set_value_at_path(tree, &write.path, write.value.clone())?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
