//! 📏 Procedural 2d document VCS on `framework_vcs`.

use framework_vcs::{
    create_document_vcs_envelope, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff,
};
use serde::{Deserialize, Serialize};

pub const PROCEDURAL_2D_SCHEMA: &str = "procedural.2d/v1";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural2dDocument {
    pub revision: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural2dDiff {
    pub revision: Option<i64>,
}

impl OperationDiff<Procedural2dDocument> for Procedural2dDiff {
    fn apply(&self, projection: &Procedural2dDocument) -> Procedural2dDocument {
        Procedural2dDocument {
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
pub enum Procedural2dOp {
    SetRevision { revision: i64 },
}

impl Operation<Procedural2dDocument> for Procedural2dOp {
    type Diff = Procedural2dDiff;

    fn diff(&self, _projection: &Procedural2dDocument) -> Procedural2dDiff {
        match self {
            Procedural2dOp::SetRevision { revision } => Procedural2dDiff {
                revision: Some(*revision),
            },
        }
    }

    fn backwards(&self, projection: &Procedural2dDocument) -> Vec<Self> {
        vec![Procedural2dOp::SetRevision {
            revision: projection.revision,
        }]
    }
}

pub type Procedural2dEnvelope = DocumentVcsEnvelope<Procedural2dDocument, Procedural2dOp>;
pub type Procedural2dStore = DocumentVcsStore<Procedural2dDocument, Procedural2dOp>;

pub fn empty_procedural2d_projection() -> Procedural2dDocument {
    Procedural2dDocument { revision: 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn procedural2d_document_vcs_replays_ops() {
        let mut store = Procedural2dStore::new(create_document_vcs_envelope(
            PROCEDURAL_2D_SCHEMA,
            "procedural2d",
            empty_procedural2d_projection(),
            None,
        ));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![Procedural2dOp::SetRevision { revision: 2 }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").revision, 2);
    }
}
