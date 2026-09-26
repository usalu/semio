use super::ChangeDesignSituation;
use crate::mutations::En1996Mutation;
use crate::En1996Snapshot;
pub fn inverse(payload: &ChangeDesignSituation, base: &En1996Snapshot) -> Vec<En1996Mutation> {
    vec![En1996Mutation::ChangeDesignSituation(ChangeDesignSituation { new_design_situation: base.design_situation })]
}
