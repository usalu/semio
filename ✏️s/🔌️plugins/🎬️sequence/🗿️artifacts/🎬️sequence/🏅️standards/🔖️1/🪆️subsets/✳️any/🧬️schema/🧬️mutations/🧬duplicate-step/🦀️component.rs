//! 🧬️ Sequence mutation — `DuplicateStep`: copies a step to a new identity/position.
use crate::artifacts::sequence::diff::SequenceDiff;
use crate::artifacts::sequence::mutations::SequenceMutation;
use crate::artifacts::sequence::SequenceSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧬️ `duplicate-step` payload — source step id, the copy's new id, and its FINAL position.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "duplicate-step")]
pub struct DuplicateStep {
    pub source_id: String,
    pub new_id: String,
    pub x: f64,
    pub y: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn duplicate_step(source_id: String, new_id: String, x: f64, y: f64) -> SequenceMutation {
    SequenceMutation::DuplicateStep(DuplicateStep { source_id, new_id, x, y })
}

impl protocol::MutationKind<SequenceSnapshot, SequenceMutation> for DuplicateStep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "duplicate", entity: "step", kind: "duplicate-step", record: "DuplicatedStep" };

    async fn diff(&self, base: &SequenceSnapshot) -> protocol::MutationOutcome<SequenceDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SequenceSnapshot) -> Vec<SequenceMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Duplicate step \"{}\" as \"{}\"", self.source_id, self.new_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.new_id.clone()]
    }
}
//#endregion 🔖️Mutation

//#region 🧪️MutationLaws
#[cfg(test)]
mod mutation_law_tests {
    use super::*;
    use crate::artifacts::sequence::default_snapshot;
    use protocol::{
        testkit::{assert_fatal_never_applies, assert_missing_target_is_error, assert_mutation_inverse_law},
        Mutation,
    };

    #[semio_framework_async_macros::async_test]
    async fn duplicate_step_inverse_law() {
        let base = default_snapshot();
        assert_mutation_inverse_law(&base, &duplicate_step("step-1".into(), "step-1-copy".into(), 10.0, 10.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn duplicate_family_missing_target_is_error() {
        let base = default_snapshot();
        assert_missing_target_is_error(&base, &duplicate_step("missing".into(), "step-1-copy".into(), 0.0, 0.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn duplicate_family_fatal_never_applies() {
        let base = default_snapshot();
        let outcome = duplicate_step("step-1".into(), "step-2".into(), 0.0, 0.0).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Fatal));
        assert_fatal_never_applies(&outcome);
    }
}
//#endregion 🧪️MutationLaws
