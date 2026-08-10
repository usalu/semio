//! 🔺️ Diff fragment yielded by `SetText`.
use crate::artifacts::writer::schema::diff::text::WriterDiff;
use crate::artifacts::writer::schema::mutations::WriterMutation;
use crate::artifacts::writer::WriterSnapshot;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SetTextDiff {
    pub mutation: Option<WriterMutation>,
}

impl SetTextDiff {
    pub fn from_mutation(mutation: WriterMutation) -> Self {
        Self { mutation: Some(mutation) }
    }

    pub fn into_writer_diff(self) -> WriterDiff {
        WriterDiff { text: self.mutation.and_then(|m| match m { WriterMutation::SetText { text } => Some(text), _ => None }), document: None }
    }
}

impl MutationDiff<WriterSnapshot> for SetTextDiff {
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
