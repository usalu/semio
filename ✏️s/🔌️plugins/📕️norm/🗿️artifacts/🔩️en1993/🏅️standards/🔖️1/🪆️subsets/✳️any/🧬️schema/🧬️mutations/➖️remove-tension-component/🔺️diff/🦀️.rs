use super::RemoveTensionComponent;
use crate::diff::En1993TensionList;
use crate::{En1993Diff, En1993Snapshot};
pub fn diff(payload: &RemoveTensionComponent, base: &En1993Snapshot) -> protocol::MutationOutcome<En1993Diff> {
    if payload.index >= base.tension_components.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("tension-component index {} out of range.", payload.index), Vec::<String>::new());
    }
    let mut values = base.tension_components.clone();
    values.remove(payload.index);
    protocol::MutationOutcome::new(En1993Diff { tension_components: Some(En1993TensionList { values }), ..Default::default() })
}
