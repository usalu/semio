use super::ChangeDesignSituation;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;
pub fn inverse(_payload: &ChangeDesignSituation, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeDesignSituation(ChangeDesignSituation { new_design_situation: base.design_situation.clone() })]
}
