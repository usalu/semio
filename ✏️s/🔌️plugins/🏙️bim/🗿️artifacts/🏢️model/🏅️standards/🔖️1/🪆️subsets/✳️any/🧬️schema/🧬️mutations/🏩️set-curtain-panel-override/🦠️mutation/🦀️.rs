//! 🏩️ `set-curtain-panel-override` payload. Changes the panel of an existing override; the cell it addresses stays the same (to address another cell, delete the override and create it again).

use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::CurtainPanel;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetCurtainPanelOverride {
    pub id: String,
    pub panel: CurtainPanel,
}

impl<Op> MutationKind<ModelSnapshot, Op> for SetCurtainPanelOverride
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetCurtainPanelOverride>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "curtain-panel-override", kind: "set-curtain-panel-override", record: "SetCurtainPanelOverride" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change the panel of override \"{}\"", self.id), &format!("Füllung der Abweichung \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for SetCurtainPanelOverride
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<SetCurtainPanelOverride>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::set_curtain_panel_override::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::set_curtain_panel_override::inverse::inverse(self, base))
    }
}
