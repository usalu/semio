//! 🔁️ Fem2d mutation — `ReplaceRegion` payload + `MutationKind` impl.

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemRegion;
use protocol::MutationKind;
use protocol::SemanticDescriptor;
use protocol::ColdMutationKind;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

//#region 🔖️Mutation
/// 🔁️ Whole-value swap of an existing meshed region's payload (structured geometry + properties, no
/// field-by-field editor gesture exists for this entity — every real caller sets the whole record at
/// once).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-region")]
pub struct ReplaceRegion {
    pub id: String,
    pub new_region: FemRegion,
}

impl<Op> MutationKind<Fem2dSnapshot, Op> for ReplaceRegion
where
    Op: protocol::Mutation<Fem2dSnapshot, Diff = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff> + From<crate::standards::v1::subsets::any::schema::mutations::replace_region::ReplaceRegion>, {

    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "region", kind: "replace-region", record: "ReplacedRegion" };
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace region \"{}\"", self.id), &format!("Bereich \"{}\" ersetzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }

}
impl<Op> protocol::ColdMutationKind<Fem2dSnapshotValue, Op> for ReplaceRegion
where
    Op: protocol::ColdMutation<Fem2dSnapshotValue, ReferenceDiff = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff> + From<crate::standards::v1::subsets::any::schema::mutations::replace_region::ReplaceRegion>, {


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
