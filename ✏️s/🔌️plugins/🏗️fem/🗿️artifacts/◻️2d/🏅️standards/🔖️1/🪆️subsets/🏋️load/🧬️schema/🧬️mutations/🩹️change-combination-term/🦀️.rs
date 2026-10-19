//! 🧬️ Independent combination term intent.
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;
use protocol::MutationKind;
use protocol::SemanticDescriptor;
use protocol::ColdMutationKind;
use semio_framework_value_derive::ToValue;
use semio_framework_value_derive::FromValue;
/// 🩹️ Changes only the addressed combination term.
#[derive(Clone,Debug,PartialEq,ToValue,FromValue,semio_framework_dsl_record_derive::DslRecord,dsl::MutationLeaf,semio_framework_value::RetireOwned,semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract=::protocol)]
#[value(rename_all="camelCase",deny_unknown_fields)]
#[dsl(keyword="change-combination-term")]
pub struct ChangeCombinationTerm {
    pub id: String,
    pub case_id: String,
    #[value(required)]
    pub factor: Option<f64>,
    #[value(skip_serializing_if="Option::is_none",deserialize_with="protocol::list_delta::supplied_value",deserialize_controlled_with="protocol::list_delta::supplied_value_controlled",default_controlled="protocol::list_delta::supplied_value_default",retire_with="protocol::list_delta::supplied_value_retire")]
    pub index: Option<usize>,
}
impl<Op> MutationKind<Fem2dSnapshot, Op> for ChangeCombinationTerm where Op: protocol::Mutation<Fem2dSnapshot, Diff = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff> + From<ChangeCombinationTerm> {
    const SEMANTICS:SemanticDescriptor=SemanticDescriptor{verb:"change",entity:"combination-term",kind:"change-combination-term",record:"ChangedCombinationTerm"};
    fn label(&self)->semio_framework_ui_locale::LocalizedLabel{semio_framework_ui_locale::LocalizedLabel::native(&format!("Change combination term {}",self.id),&format!("Kombination {} ändern",self.id))}
    fn target(&self)->Vec<String>{vec![self.id.clone()]}
}
impl<Op> protocol::ColdMutationKind<Fem2dSnapshotValue, Op> for ChangeCombinationTerm where Op: protocol::ColdMutation<Fem2dSnapshotValue, ReferenceDiff = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff> + From<ChangeCombinationTerm> {
    fn diff(&self,base:&Fem2dSnapshotValue)->protocol::MutationOutcome<semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::diff::Fem2dDiff>{super::diff::diff(self,base)}

    fn inverse(&self,base:&Fem2dSnapshotValue)->Result<Vec<Op>,semio_framework_value::ValueError>{super::inverse::inverse(self,base)}
}
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
