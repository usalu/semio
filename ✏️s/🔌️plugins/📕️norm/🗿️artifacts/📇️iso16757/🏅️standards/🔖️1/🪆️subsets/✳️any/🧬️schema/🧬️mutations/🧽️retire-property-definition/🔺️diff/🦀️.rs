//! 🔺️ `retire-property-definition` — sparse diff construction.

use crate::retire_property_definition::mutation::RetirePropertyDefinition;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use semio_s_artifact_norm_iso16757_diff::{Iso16757Diff, Iso16757PropertyDefinitionsRows};

//#region 🔖️Diff

pub fn diff(payload: &RetirePropertyDefinition, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    let Some(index) = base.catalogue.property_definitions.iter().position(|definition| definition.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Property definition \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Iso16757Diff { property_definitions: Some(Iso16757PropertyDefinitionsRows::removal(&base.catalogue.property_definitions, index)), ..Default::default() })
}
