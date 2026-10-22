//! 🛷️ `move-opening` payload. Moves an opening along its host axis (centre offset from the host start) and optionally re-hosts it to another wall or curtain wall.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveOpening {
    pub id: String,
    pub offset: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

impl<Op> MutationKind<ModelSnapshot, Op> for MoveOpening
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<MoveOpening>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "opening", kind: "move-opening", record: "MoveOpening" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move opening \"{}\" to {} m", self.id, self.offset), &format!("Öffnung \"{}\" auf {} m verschieben", self.id, self.offset))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for MoveOpening
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<MoveOpening>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::move_opening::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::move_opening::inverse::inverse(self, base))
    }
}
