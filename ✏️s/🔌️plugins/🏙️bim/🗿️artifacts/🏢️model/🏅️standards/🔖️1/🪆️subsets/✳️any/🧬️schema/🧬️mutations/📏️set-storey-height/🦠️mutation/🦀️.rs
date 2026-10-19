//! 📏️ `set-storey-height` payload. Sets a storey's floor-to-floor height in metres; every elevation above and every wall resolved by it follows by inference.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetStoreyHeight {
    pub id: String,
    pub height: f64,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetStoreyHeight
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetStoreyHeight>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "storey", kind: "set-storey-height", record: "SetStoreyHeight" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_storey_height::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_storey_height::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set storey height to {} m", self.height), &format!("Geschosshöhe auf {} m setzen", self.height))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
