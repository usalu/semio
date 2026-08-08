//! 🔺️ Diff fragment yielded by `SetDocument`.
use crate::artifacts::writer::diff::WriterDiff;
use crate::artifacts::writer::mutations::WriterMutation;
use crate::artifacts::writer::WriterProjection;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SetDocumentDiff {
    pub mutation: Option<WriterMutation>,
}

impl SetDocumentDiff {
    pub fn from_mutation(mutation: WriterMutation) -> Self {
        Self { mutation: Some(mutation) }
    }

    pub fn into_writer_diff(self) -> WriterDiff {
        WriterDiff {
            text: None,
            document: self.mutation.and_then(|m| match m { WriterMutation::SetDocument { document } => Some(document), _ => None }),
        }
    }
}

impl MutationDiff<WriterProjection> for SetDocumentDiff {
    fn apply(&self, projection: &WriterProjection) -> WriterProjection {
        self.clone().into_writer_diff().apply(projection)
    }

    fn absorb(&mut self, other: Self) {
        if other.mutation.is_some() {
            *self = other;
        }
    }
}
//#endregion 🔖️Diff
