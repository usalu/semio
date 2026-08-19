//! 🔺️ `change-humidification-provided-kg-h` sparse diff construction — writes only `Din16798Diff.humidification_provided_kg_h` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_humidification_provided_kg_h::mutation::ChangeHumidificationProvidedKgH;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeHumidificationProvidedKgH, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if !payload.new_humidification_provided_kg_h.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Provided humidification rate must be a finite number, got {}.", payload.new_humidification_provided_kg_h), Vec::<String>::new());
    }
    if base.humidification_provided_kg_h == payload.new_humidification_provided_kg_h {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Provided humidification rate is already {}.", payload.new_humidification_provided_kg_h));
    }
    protocol::MutationOutcome::new(Din16798Diff { humidification_provided_kg_h: Some(payload.new_humidification_provided_kg_h.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
