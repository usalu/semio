//! 🏨️ `create-curtain-panel-override` payload. Overrides the panel of one cell of a curtain wall, named by its indices along the wall and up it; at most one override per cell. A door panel belongs in the base row.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{CurtainPanelOverride, ModelSnapshot};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateCurtainPanelOverride {
    pub id: String,
    pub curtain_panel_override: CurtainPanelOverride,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateCurtainPanelOverride
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateCurtainPanelOverride> + From<semio_s_artifact_bim_model_mut_any_107::standards::v1::subsets::any::schema::mutations::delete_curtain_panel_override::DeleteCurtainPanelOverride>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "curtain-panel-override", kind: "create-curtain-panel-override", record: "CreateCurtainPanelOverride" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Override cell ({}, {}) of curtain wall \"{}\"", self.curtain_panel_override.u, self.curtain_panel_override.v, self.curtain_panel_override.curtain), &format!("Feld ({}, {}) von Vorhangfassade \"{}\" abweichend füllen", self.curtain_panel_override.u, self.curtain_panel_override.v, self.curtain_panel_override.curtain))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for CreateCurtainPanelOverride
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<CreateCurtainPanelOverride> + From<semio_s_artifact_bim_model_mut_any_107::standards::v1::subsets::any::schema::mutations::delete_curtain_panel_override::DeleteCurtainPanelOverride>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_curtain_panel_override::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_curtain_panel_override::inverse::inverse(self, base))
    }
}
