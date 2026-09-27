//! 🩹️ Compact typed JSON snapshot patch with an exact inverse captured from the publication base.

use crate::schema::diff::JsonDiff;
use crate::schema::mutations::JsonMutation;
use crate::JsonSnapshot;
use protocol::os_spr::command::DiffAlgebra;
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

impl protocol::MutationKind<JsonSnapshot, JsonMutation> for PatchSnapshot {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "edit", entity: "snapshot", kind: "patch-snapshot", record: "PatchSnapshot" };

    fn diff(&self, base: &JsonSnapshot) -> protocol::MutationOutcome<<JsonMutation as Mutation<JsonSnapshot>>::Diff> {
        match editing::apply_snapshot_patch(base, &self.patch) {
            Ok(next) => protocol::MutationOutcome::new(<JsonDiff as DiffAlgebra<JsonSnapshot>>::between(base, &next)),
            Err(error) => protocol::MutationOutcome::error(error.code, error.message, [error.path]),
        }
    }

    fn inverse(&self, base: &JsonSnapshot) -> Vec<JsonMutation> {
        editing::inverse_snapshot_patch(base, &self.patch).map(|patch| vec![JsonMutation::PatchSnapshot(Self { patch })]).unwrap_or_default()
    }

    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Patch snapshot", "Momentaufnahme bearbeiten")
    }

    fn target(&self) -> Vec<String> {
        self.patch.edits.first().map(|edit| edit.path.clone()).unwrap_or_default()
    }
}

#[cfg(test)]
pub(crate) fn test_case() -> JsonMutation {
    let base = JsonSnapshot::default();
    let event = editing::SnapshotEditEvent::SetValue { path: "/value".into(), value: dsl::DslValue::Object(vec![("kind".into(), dsl::DslValue::String("bool".into())), ("value".into(), dsl::DslValue::Bool(true))]) };
    JsonMutation::PatchSnapshot(PatchSnapshot { patch: editing::prepare_snapshot_patch(&base, &event).expect("prepare JSON patch fixture") })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::snapshot::{JsonMember, JsonValue};
    use protocol::{OpBinary, OpText};

    #[test]
    fn patch_snapshot_diff_inverse_and_codecs_preserve_large_unrelated_members() {
        let base = JsonSnapshot::from_value(JsonValue::Object {
            members: vec![
                JsonMember { key: "payload".into(), value: JsonValue::String { value: "x".repeat(2 * 1_024 * 1_024) } },
                JsonMember { key: "status".into(), value: JsonValue::String { value: "draft".into() } },
            ],
        });
        let event = editing::SnapshotEditEvent::SetValue { path: "/value/members/1/value/value".into(), value: dsl::DslValue::String("review".into()) };
        let patch = editing::prepare_snapshot_patch(&base, &event).expect("prepare status patch");
        let mutation = JsonMutation::PatchSnapshot(PatchSnapshot { patch });
        let next = MutationDiff::apply(mutation.diff(&base).diff(), &base).expect("apply status patch");
        let JsonValue::Object { members } = &next.value else { panic!("object remains an object") };
        assert_eq!(members[0], match &base.value { JsonValue::Object { members } => members[0].clone(), _ => unreachable!() });
        assert_eq!(members[1].value, JsonValue::String { value: "review".into() });
        let inverse = mutation.inverse(&base);
        assert_eq!(inverse.len(), 1);
        assert_eq!(MutationDiff::apply(inverse[0].diff(&next).diff(), &next).expect("apply inverse"), base);
        assert_eq!(JsonMutation::parse_op(&mutation.print_op()).expect("text round trip"), mutation);
        assert_eq!(JsonMutation::decode_op(&mutation.encode_op().expect("encode patch")).expect("binary round trip"), mutation);
    }
}
