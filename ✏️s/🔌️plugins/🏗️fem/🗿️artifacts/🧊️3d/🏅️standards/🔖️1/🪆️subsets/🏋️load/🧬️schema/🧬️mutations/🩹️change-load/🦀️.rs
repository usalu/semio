//! 🧩️ Typed load field intent shared by local edits, replay and collaboration.
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dLoadPatch;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use protocol::MutationKind;
use protocol::SemanticDescriptor;
use protocol::ColdMutationKind;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

/// 🩹️ Changes only supplied properties of an existing load.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "change-load")]
pub struct ChangeLoad {
    pub case_id: String,
    pub id: String,
    pub patch: Fem3dLoadPatch,
}

impl<Op> MutationKind<Fem3dSnapshot, Op> for ChangeLoad where Op: protocol::Mutation<Fem3dSnapshot, Diff = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff> + From<ChangeLoad> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "load", kind: "change-load", record: "ChangedLoad" };
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change load \"{}\"", self.id), &format!("Last \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> { vec![self.case_id.clone(), self.id.clone()] }
}
impl<Op> protocol::ColdMutationKind<Fem3dSnapshotValue, Op> for ChangeLoad where Op: protocol::ColdMutation<Fem3dSnapshotValue, ReferenceDiff = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff> + From<ChangeLoad> {
    fn diff(&self, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff> { super::diff::diff(self, base) }

    fn inverse(&self, base: &Fem3dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> { super::inverse::inverse(self, base) }
}
