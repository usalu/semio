//! 🌱️ Fem3d mutation — `CreateSolid` payload + `MutationKind` impl.

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemSolid;
use protocol::MutationKind;
use protocol::SemanticDescriptor;
use protocol::ColdMutationKind;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemSolid`] meshed solid into existence.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-solid")]
pub struct CreateSolid {
    pub solid: FemSolid,
    /// 📍 Zero-based insertion position among the siblings; `None` or past the end appends.
    pub index: Option<usize>,
}

impl<Op> MutationKind<Fem3dSnapshot, Op> for CreateSolid
where
    Op: protocol::Mutation<Fem3dSnapshot, Diff = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff> + From<crate::standards::v1::subsets::any::schema::mutations::delete_solid::DeleteSolid>, {

    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "solid", kind: "create-solid", record: "CreatedSolid" };
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create solid \"{}\"", self.solid.id), &format!("Körper \"{}\" erstellen", self.solid.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.solid.id.clone()]
    }

}
impl<Op> protocol::ColdMutationKind<Fem3dSnapshotValue, Op> for CreateSolid
where
    Op: protocol::ColdMutation<Fem3dSnapshotValue, ReferenceDiff = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff> + From<crate::standards::v1::subsets::any::schema::mutations::delete_solid::DeleteSolid>, {


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
