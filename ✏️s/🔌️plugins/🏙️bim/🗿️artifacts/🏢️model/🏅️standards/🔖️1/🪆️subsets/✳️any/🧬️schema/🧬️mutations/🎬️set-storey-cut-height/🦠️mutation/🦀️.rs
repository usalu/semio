//! 🎬️ `set-storey-cut-height` payload. Sets the height above its elevation at which the plan view cuts a storey; an assigned null returns to the 1.2 m default. Pure view convention: nothing else in the model depends on it.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Assigned;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetStoreyCutHeight {
    pub id: String,
    pub cut_height: Assigned<Option<f64>>,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetStoreyCutHeight
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetStoreyCutHeight>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "storey", kind: "set-storey-cut-height", record: "SetStoreyCutHeight" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_storey_cut_height::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_storey_cut_height::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set plan cut height of storey \"{}\"", self.id), &format!("Schnitthöhe von Geschoss \"{}\" setzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
