//! 🏷️ Fem2d mutation — `ChangeLoadCaseName` payload + `MutationKind` impl.

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;
use protocol::MutationKind;
use protocol::SemanticDescriptor;
use protocol::ColdMutationKind;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

//#region 🔖️Mutation
/// 🏷️ Sets an existing load case's human-readable `name`. The `id` is the identity every
/// combination term resolves through and is untouched, so a rename orphans nothing.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-load-case-name")]
pub struct ChangeLoadCaseName {
    pub case_id: String,
    pub new_name: String,
}

impl<Op> MutationKind<Fem2dSnapshot, Op> for ChangeLoadCaseName
where
    Op: protocol::Mutation<Fem2dSnapshot, Diff = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff> + From<crate::standards::v1::subsets::any::schema::mutations::change_load_case_name::ChangeLoadCaseName>, {

    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "load-case", kind: "change-load-case-name", record: "ChangedLoadCaseName" };
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename case \"{}\" to \"{}\"", self.case_id, self.new_name), &format!("Lastfall \"{}\" in \"{}\" umbenennen", self.case_id, self.new_name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.case_id.clone()]
    }

}
impl<Op> protocol::ColdMutationKind<Fem2dSnapshotValue, Op> for ChangeLoadCaseName
where
    Op: protocol::ColdMutation<Fem2dSnapshotValue, ReferenceDiff = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff> + From<crate::standards::v1::subsets::any::schema::mutations::change_load_case_name::ChangeLoadCaseName>, {


    fn diff(&self, base: &Fem2dSnapshotValue) -> protocol::MutationOutcome<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Fem2dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

}
//#endregion 🔖️Mutation
