use super::ChangeDesignApproach;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;
pub fn inverse(_payload: &ChangeDesignApproach, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeDesignApproach(ChangeDesignApproach { new_design_approach: base.design_approach.clone() })]
}
