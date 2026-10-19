//! 🌱️ Fem2d mutation — `CreateMaterial` payload + `MutationKind` impl.

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_s_fem_material::planar::FemMaterialValue;
use protocol::MutationKind;
use protocol::SemanticDescriptor;
use protocol::ColdMutationKind;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemMaterialValue`] into existence.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-material")]
pub struct CreateMaterial {
    pub material: FemMaterialValue,
    /// 📍 Zero-based insertion position among the siblings; `None` or past the end appends.
    pub index: Option<usize>,
}

impl<Op> MutationKind<Fem2dSnapshot, Op> for CreateMaterial
where
    Op: protocol::Mutation<Fem2dSnapshot, Diff = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff> + From<crate::standards::v1::subsets::any::schema::mutations::delete_material::DeleteMaterial>, {

    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "material", kind: "create-material", record: "CreatedMaterial" };
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create material \"{}\"", self.material.id), &format!("Material \"{}\" erstellen", self.material.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.material.id.clone()]
    }

}
impl<Op> protocol::ColdMutationKind<Fem2dSnapshotValue, Op> for CreateMaterial
where
    Op: protocol::ColdMutation<Fem2dSnapshotValue, ReferenceDiff = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff> + From<crate::standards::v1::subsets::any::schema::mutations::delete_material::DeleteMaterial>, {


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
