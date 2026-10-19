//! 🧬️ Independent combination name intent.
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use protocol::MutationKind;
use protocol::SemanticDescriptor;
use protocol::ColdMutationKind;
use semio_framework_value_derive::ToValue;
use semio_framework_value_derive::FromValue;
/// 🩹️ Changes only the addressed combination name.
#[derive(Clone,Debug,PartialEq,ToValue,FromValue,semio_framework_dsl_record_derive::DslRecord,dsl::MutationLeaf,semio_framework_value::RetireOwned,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract=::protocol)]
#[value(rename_all="camelCase",deny_unknown_fields)]
#[dsl(keyword="change-combination-name")]
pub struct ChangeCombinationName {
    pub id: String,
    pub name: String,
}
impl<Op> MutationKind<Fem3dSnapshot, Op> for ChangeCombinationName where Op: protocol::Mutation<Fem3dSnapshot, Diff = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff> + From<ChangeCombinationName> {
    const SEMANTICS:SemanticDescriptor=SemanticDescriptor{verb:"change",entity:"combination-name",kind:"change-combination-name",record:"ChangedCombinationName"};
    fn label(&self)->semio_framework_ui_locale::LocalizedLabel{semio_framework_ui_locale::LocalizedLabel::native(&format!("Change combination name {}",self.id),&format!("Kombination {} ändern",self.id))}
    fn target(&self)->Vec<String>{vec![self.id.clone()]}
}
impl<Op> protocol::ColdMutationKind<Fem3dSnapshotValue, Op> for ChangeCombinationName where Op: protocol::ColdMutation<Fem3dSnapshotValue, ReferenceDiff = semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff> + From<ChangeCombinationName> {
    fn diff(&self,base:&Fem3dSnapshotValue)->protocol::MutationOutcome<semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::diff::Fem3dDiff>{super::diff::diff(self,base)}

    fn inverse(&self,base:&Fem3dSnapshotValue)->Result<Vec<Op>,semio_framework_value::ValueError>{super::inverse::inverse(self,base)}
}