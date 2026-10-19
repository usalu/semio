//! 🧩️ Typed section field intent shared by local edits, replay and collaboration.
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dSectionPatch;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;
use protocol::MutationKind;
use protocol::SemanticDescriptor;
use protocol::ColdMutationKind;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

/// 🩹️ Changes only supplied properties of an existing section.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "change-section")]
pub struct ChangeSection {
    pub id: String,
    pub patch: Fem2dSectionPatch,
}

impl<Op> MutationKind<Fem2dSnapshot, Op> for ChangeSection where Op: protocol::Mutation<Fem2dSnapshot, Diff = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff> + From<ChangeSection> {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "section", kind: "change-section", record: "ChangedSection" };
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change section \"{}\"", self.id), &format!("Querschnitt \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
impl<Op> protocol::ColdMutationKind<Fem2dSnapshotValue, Op> for ChangeSection where Op: protocol::ColdMutation<Fem2dSnapshotValue, ReferenceDiff = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff> + From<ChangeSection> {
    fn diff(&self, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff> { super::diff::diff(self, base) }

    fn inverse(&self, base: &Fem2dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> { super::inverse::inverse(self, base) }
}
