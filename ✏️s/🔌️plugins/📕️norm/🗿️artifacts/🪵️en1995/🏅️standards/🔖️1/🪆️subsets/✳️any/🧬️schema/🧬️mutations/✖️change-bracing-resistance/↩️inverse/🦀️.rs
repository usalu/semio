use crate::change_bracing_resistance::ChangeBracingResistance;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeBracingResistance, base: &En1995Snapshot) -> Option<crate::change_bracing_resistance::ChangeBracingResistance> {
    match base.bracings.iter().find(|item| item.id == payload.bracing_id) {
        Some(item) => Some(ChangeBracingResistance { bracing_id: payload.bracing_id.clone(), new_value: item.resistance_n }),
        None => None,
    }
}
