use super::InsertTensionComponent;
use crate::diff::En1993TensionList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &InsertTensionComponent, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if base.tension_components.iter().any(|existing| existing.id == payload.tension_component.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Tension component id {} already exists.", payload.tension_component.id), [payload.tension_component.id.clone()]);
    }
    let mut values = base.tension_components.clone();
    let at = payload.index.min(values.len());
    values.insert(at, payload.tension_component.clone());
    protocol::MutationOutcome::new(En1993Diff { tension_components: Some(En1993TensionList { values }), ..Default::default() })
}
