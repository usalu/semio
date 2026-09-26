use crate::mutations::change_design_working_life::ChangeDesignWorkingLife;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(_payload: &ChangeDesignWorkingLife, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeDesignWorkingLife(ChangeDesignWorkingLife { new_years: base.design_working_life_years })]
}
