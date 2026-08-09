//! 🔺️ Diff fragment yielded by `SetSnapshot`.
use crate::artifacts::writer::diff::WriterDiff;
use crate::artifacts::writer::mutations::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SetSnapshotDiff {
    pub mutation: Option<WriterMutation>,
}

impl SetSnapshotDiff {
    pub fn from_mutation(mutation: WriterMutation) -> Self {
        Self { mutation: Some(mutation) }
    }

    pub fn into_writer_diff(self) -> WriterDiff {
        WriterDiff {
            text: None,
            document: self.mutation.and_then(|m| match m { WriterMutation::SetSnapshot { snapshot } => Some(document), _ => None }),
        }
    }
}

impl MutationDiff<WriterSnapshot> for SetSnapshotDiff {
    fn apply(&self, projection: &WriterSnapshot) -> WriterSnapshot {
        self.clone().into_writer_diff().apply(projection)
    }

    fn absorb(&mut self, other: Self) {
        if other.mutation.is_some() {
            *self = other;
        }
    }
}
//#endregion 🔖️Diff
