//! 🔺️ Diff fragment yielded by `SetCursor`.
use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dDocument;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// @emoji 🔺️ Diff produced by one `SetCursor` mutation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SetCursorDiff {
    pub mutation: Option<Process3dMutation>,
}

impl SetCursorDiff {
    pub fn from_mutation(mutation: Process3dMutation) -> Self {
        Self { mutation: Some(mutation) }
    }
}

impl MutationDiff<Process3dDocument> for SetCursorDiff {
    fn apply(&self, projection: &Process3dDocument) -> Process3dDocument {
        match &self.mutation {
            Some(mutation) => {
                let diff = <Process3dMutation as protocol::Mutation<Process3dDocument>>::diff(mutation, projection);
                <Process3dDiff as MutationDiff<Process3dDocument>>::apply(&diff, projection)
            }
            None => projection.clone(),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.mutation.is_some() {
            *self = other;
        }
    }
}
//#endregion 🔖️Diff
