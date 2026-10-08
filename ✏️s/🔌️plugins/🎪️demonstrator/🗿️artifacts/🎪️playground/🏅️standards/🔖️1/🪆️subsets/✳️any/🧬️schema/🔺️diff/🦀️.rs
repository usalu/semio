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

impl MutationDiff<PlaygroundSnapshot> for PlaygroundDiff {
    fn apply(&self, snapshot: &PlaygroundSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<PlaygroundSnapshot> {
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
    }
}

impl protocol::DiffAlgebra<PlaygroundSnapshot> for PlaygroundDiff {
    fn inverse(&self, base: &PlaygroundSnapshot) -> Self {
        Self { schema: self.schema.as_ref().map(|_| base.schema.clone()) }
    }
    fn between(base: &PlaygroundSnapshot, other: &PlaygroundSnapshot) -> Self {
        Self { schema: (base.schema != other.schema).then(|| other.schema.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
