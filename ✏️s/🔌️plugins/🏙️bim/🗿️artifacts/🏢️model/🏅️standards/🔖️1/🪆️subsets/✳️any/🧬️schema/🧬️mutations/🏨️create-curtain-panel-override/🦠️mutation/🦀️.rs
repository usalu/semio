//! 🏨️ `create-curtain-panel-override` payload. Overrides the panel of one cell of a curtain wall, named by its indices along the wall and up it; at most one override per cell. A door panel belongs in the base row.

use crate::{CurtainPanelOverride, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateCurtainPanelOverride {
    pub id: String,
    pub curtain_panel_override: CurtainPanelOverride,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateCurtainPanelOverride {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "curtain-panel-override", kind: "create-curtain-panel-override", record: "CreateCurtainPanelOverride" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Override cell ({}, {}) of curtain wall \"{}\"", self.curtain_panel_override.u, self.curtain_panel_override.v, self.curtain_panel_override.curtain), &format!("Feld ({}, {}) von Vorhangfassade \"{}\" abweichend füllen", self.curtain_panel_override.u, self.curtain_panel_override.v, self.curtain_panel_override.curtain))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
