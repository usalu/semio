use super::ChangeInvestigationDepth;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;
pub fn inverse(_payload: &ChangeInvestigationDepth, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeInvestigationDepth(ChangeInvestigationDepth { new_investigation_depth: base.investigation_depth })]
}
