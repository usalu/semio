//! ➖️ Fem3d mutation — `RemoveLoad` payload + `MutationKind` impl.

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use protocol::MutationKind;
use protocol::SemanticDescriptor;
use protocol::ColdMutationKind;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

//#region 🔖️Mutation
/// ➖️ Detaches a load from an existing load case's `loads` member collection by id, capturing
/// nothing itself (the removed payload is recovered from `base` inside `↩️inverse`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-load")]
pub struct RemoveLoad {
    pub case_id: String,
    pub load_id: String,
}

impl<Op> MutationKind<Fem3dSnapshot, Op> for RemoveLoad
where
    Op: protocol::Mutation<Fem3dSnapshot, Diff = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff> + From<crate::standards::v1::subsets::any::schema::mutations::add_load::AddLoad>, {

    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "load", kind: "remove-load", record: "RemovedLoad" };
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove load \"{}\" from case \"{}\"", self.load_id, self.case_id), &format!("Last \"{}\" aus Lastfall \"{}\" entfernen", self.load_id, self.case_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.case_id.clone()]
    }

}
impl<Op> protocol::ColdMutationKind<Fem3dSnapshotValue, Op> for RemoveLoad
where
    Op: protocol::ColdMutation<Fem3dSnapshotValue, ReferenceDiff = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff> + From<crate::standards::v1::subsets::any::schema::mutations::add_load::AddLoad>, {


    fn diff(&self, base: &Fem3dSnapshotValue) -> protocol::MutationOutcome<semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Fem3dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

}
//#endregion 🔖️Mutation
