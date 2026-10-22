//! 💧️ `create-mep-element` payload. Routes a duct, pipe or cable tray: a storey, a system, a cross-section in metres and a path of at least two distinct points whose height is measured above the storey.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{MepElement, ModelSnapshot};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateMepElement {
    pub id: String,
    pub mep: MepElement,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateMepElement
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateMepElement> + From<semio_s_artifact_bim_model_mut_any_123::standards::v1::subsets::any::schema::mutations::delete_mep_element::DeleteMepElement>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "mep-element", kind: "create-mep-element", record: "CreateMepElement" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Route MEP element \"{}\"", self.mep.name), &format!("TGA-Element \"{}\" verlegen", self.mep.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for CreateMepElement
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<CreateMepElement> + From<semio_s_artifact_bim_model_mut_any_123::standards::v1::subsets::any::schema::mutations::delete_mep_element::DeleteMepElement>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_mep_element::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_mep_element::inverse::inverse(self, base))
    }
}
