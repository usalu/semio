//! 🔁️ Fem3d mutation — `ReplaceNode` payload + `MutationKind` impl.

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_s_fem_node::volumetric::FemNodeValue;
use protocol::MutationKind;
use protocol::SemanticDescriptor;
use protocol::ColdMutationKind;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;

//#region 🔖️Mutation
/// 🔁️ Whole-value swap of an existing node's payload — the one gesture in this vocabulary that
/// MOVES a node (the inspector's coordinate edits and the transform gumball both spell a move this
/// way). Moving a node an element, a support or a nodal load still names is deliberately allowed:
/// every referrer addresses it by `id`, and a replacement may not change that id, so the geometry
/// travels while the topology stays exactly where it was.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-node")]
pub struct ReplaceNode {
    pub id: String,
    pub new_node: FemNodeValue,
}

impl<Op> MutationKind<Fem3dSnapshot, Op> for ReplaceNode
where
    Op: protocol::Mutation<Fem3dSnapshot, Diff = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff> + From<crate::standards::v1::subsets::any::schema::mutations::replace_node::ReplaceNode>, {

    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "node", kind: "replace-node", record: "ReplacedNode" };
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace node \"{}\"", self.id), &format!("Knoten \"{}\" ersetzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }

}
impl<Op> protocol::ColdMutationKind<Fem3dSnapshotValue, Op> for ReplaceNode
where
    Op: protocol::ColdMutation<Fem3dSnapshotValue, ReferenceDiff = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff> + From<crate::standards::v1::subsets::any::schema::mutations::replace_node::ReplaceNode>, {


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
