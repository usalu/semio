//! 🏦️ `set-curtain-wall-type-of` payload. Builds a curtain wall as another curtain wall type; its grid, mullions and panels follow by inference.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCurtainWallTypeOf {
    pub id: String,
    pub curtain_wall_type: String,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetCurtainWallTypeOf
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetCurtainWallTypeOf>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "curtain-wall", kind: "set-curtain-wall-type-of", record: "SetCurtainWallTypeOf" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set type of curtain wall \"{}\" to \"{}\"", self.id, self.curtain_wall_type), &format!("Typ von Vorhangfassade \"{}\" auf \"{}\" setzen", self.id, self.curtain_wall_type))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetCurtainWallTypeOf
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetCurtainWallTypeOf>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall_type_of::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall_type_of::inverse::inverse(self, base))
    }
}
