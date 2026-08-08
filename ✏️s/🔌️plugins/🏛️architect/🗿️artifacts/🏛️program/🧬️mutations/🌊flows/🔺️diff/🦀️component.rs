//! 🔺️ Diff fragment for `Flows`.
use crate::artifacts::program::mutations::ProgramMutation;
use crate::artifacts::program::Program;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FlowsDiff { pub mutation: Option<ProgramMutation> }
impl FlowsDiff {
    pub fn from_mutation(mutation: ProgramMutation) -> Self { Self { mutation: Some(mutation) } }
}
impl MutationDiff<Program> for FlowsDiff {
    fn apply(&self, projection: &Program) -> Program {
        match &self.mutation {
            Some(m) => { let mut next = projection.clone(); crate::artifacts::program::mutations::apply_program_mutation(&mut next, m); next }
            None => projection.clone(),
        }
    }
    fn absorb(&mut self, other: Self) { if other.mutation.is_some() { *self = other; } }
}
//#endregion 🔖️Diff
