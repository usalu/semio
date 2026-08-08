//! 🔺️ Diff fragment for `Operations`.
use crate::artifacts::program::mutations::ProgramMutation;
use crate::artifacts::program::Program;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OperationsDiff { pub mutation: Option<ProgramMutation> }
impl OperationsDiff {
    pub fn from_mutation(mutation: ProgramMutation) -> Self { Self { mutation: Some(mutation) } }
}
impl MutationDiff<Program> for OperationsDiff {
    fn apply(&self, projection: &Program) -> Program {
        match &self.mutation {
            Some(m) => { let mut next = projection.clone(); crate::artifacts::program::mutations::apply_program_mutation(&mut next, m); next }
            None => projection.clone(),
        }
    }
    fn absorb(&mut self, other: Self) { if other.mutation.is_some() { *self = other; } }
}
//#endregion 🔖️Diff
