//! 👯 Puzzle 5d document VCS on `framework_vcs`.

use framework_vcs::{
    create_document_vcs_envelope, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff,
};
use serde::{Deserialize, Serialize};

pub const PUZZLE_5D_SCHEMA: &str = "puzzle.5d/v1";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dDocument {
    pub revision: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dDiff {
    pub revision: Option<i64>,
}

impl OperationDiff<Puzzle5dDocument> for Puzzle5dDiff {
    fn apply(&self, projection: &Puzzle5dDocument) -> Puzzle5dDocument {
        Puzzle5dDocument {
            revision: self.revision.unwrap_or(projection.revision),
        }
    }

    fn absorb(&mut self, other: Self) {
        if other.revision.is_some() {
            self.revision = other.revision;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum Puzzle5dOp {
    SetRevision { revision: i64 },
}

impl Operation<Puzzle5dDocument> for Puzzle5dOp {
    type Diff = Puzzle5dDiff;

    fn diff(&self, _projection: &Puzzle5dDocument) -> Puzzle5dDiff {
        match self {
            Puzzle5dOp::SetRevision { revision } => Puzzle5dDiff {
                revision: Some(*revision),
            },
        }
    }

    fn backwards(&self, projection: &Puzzle5dDocument) -> Vec<Self> {
        vec![Puzzle5dOp::SetRevision {
            revision: projection.revision,
        }]
    }
}

pub type Puzzle5dEnvelope = DocumentVcsEnvelope<Puzzle5dDocument, Puzzle5dOp>;
pub type Puzzle5dStore = DocumentVcsStore<Puzzle5dDocument, Puzzle5dOp>;

pub fn empty_puzzle5d_projection() -> Puzzle5dDocument {
    Puzzle5dDocument { revision: 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn puzzle5d_document_vcs_replays_ops() {
        let mut store = Puzzle5dStore::new(create_document_vcs_envelope(
            PUZZLE_5D_SCHEMA,
            "puzzle5d",
            empty_puzzle5d_projection(),
            None,
        ));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![Puzzle5dOp::SetRevision { revision: 5 }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").revision, 5);
    }
}
