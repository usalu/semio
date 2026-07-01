//! 📐 Procedural 3d document VCS on `framework_vcs`.

use framework_vcs::{
    create_document_vcs_envelope, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff,
};
use serde::{Deserialize, Serialize};

pub const PROCEDURAL_3D_SCHEMA: &str = "procedural.3d/v1";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dDocument {
    pub revision: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dDiff {
    pub revision: Option<i64>,
}

impl OperationDiff<Procedural3dDocument> for Procedural3dDiff {
    fn apply(&self, projection: &Procedural3dDocument) -> Procedural3dDocument {
        Procedural3dDocument {
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
pub enum Procedural3dOp {
    SetRevision { revision: i64 },
}

impl Operation<Procedural3dDocument> for Procedural3dOp {
    type Diff = Procedural3dDiff;

    fn diff(&self, _projection: &Procedural3dDocument) -> Procedural3dDiff {
        match self {
            Procedural3dOp::SetRevision { revision } => Procedural3dDiff {
                revision: Some(*revision),
            },
        }
    }

    fn backwards(&self, projection: &Procedural3dDocument) -> Vec<Self> {
        vec![Procedural3dOp::SetRevision {
            revision: projection.revision,
        }]
    }
}

pub type Procedural3dEnvelope = DocumentVcsEnvelope<Procedural3dDocument, Procedural3dOp>;
pub type Procedural3dStore = DocumentVcsStore<Procedural3dDocument, Procedural3dOp>;

pub fn empty_procedural3d_projection() -> Procedural3dDocument {
    Procedural3dDocument { revision: 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn procedural3d_document_vcs_replays_ops() {
        let mut store = Procedural3dStore::new(create_document_vcs_envelope(
            PROCEDURAL_3D_SCHEMA,
            "procedural3d",
            empty_procedural3d_projection(),
            None,
        ));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![Procedural3dOp::SetRevision { revision: 4 }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").revision, 4);
    }
}
