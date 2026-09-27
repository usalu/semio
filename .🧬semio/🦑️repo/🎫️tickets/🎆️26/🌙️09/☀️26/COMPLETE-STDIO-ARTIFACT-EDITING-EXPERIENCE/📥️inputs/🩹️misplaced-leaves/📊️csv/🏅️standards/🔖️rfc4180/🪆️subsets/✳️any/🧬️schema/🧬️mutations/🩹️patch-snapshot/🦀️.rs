//! 🩹️ Compact typed CSV snapshot patch with an exact inverse captured from the publication base.

use crate::schema::diff::CsvDiff;
use crate::schema::mutations::CsvMutation;
use crate::CsvSnapshot;
use protocol::command::DiffAlgebra;
use protocol::{Mutation, MutationDiff};
use semio_s_artifact_stdio_contract::editing;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct PatchSnapshot {
    pub patch: editing::SnapshotPatch,
}

#[path = "💾️binary/🦀️.rs"]
pub mod binary;
#[path = "📝️text/🦀️.rs"]
pub mod text;

impl protocol::MutationKind<CsvSnapshot, CsvMutation> for PatchSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "snapshot", kind: "patch-snapshot", record: "PatchSnapshot" };

    fn diff(&self, base: &CsvSnapshot) -> protocol::MutationOutcome<<CsvMutation as Mutation<CsvSnapshot>>::Diff> {
        match editing::apply_snapshot_patch(base, &self.patch) {
            Ok(next) => protocol::MutationOutcome::new(<CsvDiff as DiffAlgebra<CsvSnapshot>>::between(base, &next)),
            Err(error) => protocol::MutationOutcome::error(error.code, error.message, [error.path]),
        }
    }

    fn inverse(&self, base: &CsvSnapshot) -> Vec<CsvMutation> {
        editing::inverse_snapshot_patch(base, &self.patch).map(|patch| vec![CsvMutation::PatchSnapshot(Self { patch })]).unwrap_or_default()
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Patch snapshot", "Momentaufnahme bearbeiten")
    }

    fn target(&self) -> Vec<String> {
        self.patch.edits.first().map(|edit| edit.path.clone()).unwrap_or_default()
    }
}

#[cfg(test)]
pub(crate) fn test_case() -> CsvMutation {
    let base = CsvSnapshot::default();
    let event = editing::SnapshotEditEvent::SetValue { path: "/hasHeader".into(), value: dsl::DslValue::Bool(false) };
    CsvMutation::PatchSnapshot(PatchSnapshot { patch: editing::prepare_snapshot_patch(&base, &event).expect("prepare CSV patch fixture") })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::snapshot::{CsvField, CsvRecord};
    use protocol::{OpBinary, OpText};

    #[test]
    fn patch_snapshot_diff_inverse_and_codecs_preserve_large_unrelated_fields() {
        let mut base = CsvSnapshot::default();
        base.records.push(CsvRecord { fields: vec![CsvField { value: "x".repeat(2 * 1_024 * 1_024), quoted: true }] });
        let event = editing::SnapshotEditEvent::SetValue { path: "/hasHeader".into(), value: dsl::DslValue::Bool(false) };
        let patch = editing::prepare_snapshot_patch(&base, &event).expect("prepare header patch");
        let mutation = CsvMutation::PatchSnapshot(PatchSnapshot { patch });
        let next = MutationDiff::apply(mutation.diff(&base).diff(), &base).expect("apply header patch");
        assert!(!next.has_header);
        assert_eq!(next.records, base.records);
        let inverse = mutation.inverse(&base);
        assert_eq!(inverse.len(), 1);
        assert_eq!(MutationDiff::apply(inverse[0].diff(&next).diff(), &next).expect("apply inverse"), base);
        assert_eq!(CsvMutation::parse_op(&mutation.print_op()).expect("text round trip"), mutation);
        assert_eq!(CsvMutation::decode_op(&mutation.encode_op().expect("encode patch")).expect("binary round trip"), mutation);
    }
}
