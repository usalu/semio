//! 🔺️ Diff constructor for `DeleteMaterial`: one deleted material entry. A material that any layer, profile type, railing, ramp or curtain
//! wall type, panel override or space finish still names is refused as `mutation.target-referenced`.

use super::DeleteMaterial;
use crate::{CurtainPanel, Entry, Layer, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

fn layered(layers: &[Layer], id: &str) -> bool {
    layers.iter().any(|layer| layer.material == id)
}

fn solid(panel: &CurtainPanel, id: &str) -> bool {
    matches!(panel, CurtainPanel::Solid { material } if material == id)
}

fn user(base: &ModelSnapshot, id: &str) -> Option<&'static str> {
    [
        ("a wall type", base.wall_types.values().any(|row| layered(&row.layers, id))),
        ("a slab type", base.slab_types.values().any(|row| layered(&row.layers, id))),
        ("a ceiling type", base.ceiling_types.values().any(|row| layered(&row.layers, id))),
        ("a roof type", base.roof_types.values().any(|row| layered(&row.layers, id))),
        ("a column type", base.column_types.values().any(|row| row.material == id)),
        ("a beam type", base.beam_types.values().any(|row| row.material == id)),
        ("a window type", base.window_types.values().any(|row| row.material == id)),
        ("a door type", base.door_types.values().any(|row| row.material == id)),
        ("a curtain wall type", base.curtain_wall_types.values().any(|row| row.panel_material == id || row.mullion_material == id || solid(&row.panel, id))),
        ("a curtain panel override", base.curtain_panel_overrides.values().any(|row| solid(&row.panel, id))),
        ("a railing", base.railings.values().any(|row| row.material == id)),
        ("a space finish", base.spaces.values().any(|row| [&row.floor_finish, &row.wall_finish, &row.ceiling_finish].into_iter().any(|finish| finish.as_deref() == Some(id)))),
        ("a ramp", base.ramps.values().any(|row| row.material == id)),
    ]
    .into_iter()
    .find_map(|(kind, present)| present.then_some(kind))
}

pub fn diff(payload: &DeleteMaterial, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.materials.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Material \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if let Some(kind) = user(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Material \"{}\" is still used by {kind}.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::materials(payload.id.clone(), Entry::Deleted))
}
