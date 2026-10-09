//! 🔺️ Diff constructor for `DeletePropertyTemplate`: one deleted template entry. A template is library data and owns no entries: the property sets of elements are keyed by element id and stay as they are, they only lose
//! their definition (the effective properties are inferred, so nothing derived is written).

use super::DeletePropertyTemplate;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeletePropertyTemplate, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.property_templates.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Property template \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::property_templates(payload.id.clone(), Entry::Deleted))
}
