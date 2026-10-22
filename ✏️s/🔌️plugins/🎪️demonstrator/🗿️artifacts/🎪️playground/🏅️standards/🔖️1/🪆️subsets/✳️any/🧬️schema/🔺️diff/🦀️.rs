//! 🧬️ Playground diff schema — sparse field delta over the artifact.

use schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the playground artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.demonstrator.playground")]
pub struct PlaygroundDiff {
    #[state(artifact)]
    pub schema: Option<String>,
}
//#endregion 🔖️Diff

use crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use protocol::MutationDiff;

impl MutationDiff<PlaygroundSnapshot> for PlaygroundDiff {}

impl protocol::ColdMutationDiff<PlaygroundSnapshot> for PlaygroundDiff {
    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
    }

    fn apply(&self, unpublished: &mut PlaygroundSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let snapshot = &*unpublished;
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        *unpublished = next;
        Ok(())
    }
}

impl protocol::DiffAlgebra<PlaygroundSnapshot> for PlaygroundDiff {
    fn is_empty(&self) -> bool {
        self.schema.is_none()
    }
}

impl protocol::ColdDiffAlgebra<PlaygroundSnapshot> for PlaygroundDiff {
    fn inverse(&self, base: &PlaygroundSnapshot) -> Self {
        Self { schema: self.schema.as_ref().map(|_| base.schema.clone()) }
    }
}


