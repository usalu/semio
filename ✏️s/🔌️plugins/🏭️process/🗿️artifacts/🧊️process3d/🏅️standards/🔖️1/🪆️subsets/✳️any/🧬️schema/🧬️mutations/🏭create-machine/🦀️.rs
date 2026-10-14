//! 🛠️ Process3d mutation — `CreateMachine` (repurposes the pre-migration `🛠️machines/` triad dir —
//! glue.rs path-includes this exact directory outside this facet's writable boundary, so the
//! directory name stays `🛠️machines`; see the migration report's `sharedFileRequests` for the
//! rename once a later pass can touch `🦀️.rs`).

use semio_s_artifact_process_process3d_model_diff::Process3dDiff;
use semio_s_artifact_process_process3d_model::{snapshot::Process3dSnapshot, snapshot::WorkshopMachine};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️CreateMachine
/// 🛠️ Full initial payload for a new [`WorkshopMachine`] installed into the document's workshop.
/// `index` is carried for label/provenance purposes only — the workshop's `machines` list has no
/// user-meaningful order, so the diff always appends.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateMachine {
    pub index: usize,
    pub machine: WorkshopMachine,
}

impl<Op> protocol::MutationKind<Process3dSnapshot, Op> for CreateMachine
where
    Op: protocol::Mutation<Process3dSnapshot, Diff = semio_s_artifact_process_process3d_model_diff::Process3dDiff> + From<crate::delete_machine::DeleteMachine>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "machine", kind: "create-machine", record: "CreatedMachine" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create machine \"{}\"", self.machine.label), &format!("Maschine \"{}\" erstellen", self.machine.label))
    }

    fn target(&self) -> Vec<String> {
        vec![self.machine.id.clone()]
    }
}
//#endregion 🔖️CreateMachine
