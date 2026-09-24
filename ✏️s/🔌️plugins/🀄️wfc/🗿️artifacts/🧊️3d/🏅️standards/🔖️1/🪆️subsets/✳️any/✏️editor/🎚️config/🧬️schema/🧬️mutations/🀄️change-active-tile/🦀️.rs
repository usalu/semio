//! 🀄️ Change Active Tile in the WFC 3D config facet — which tile a pin gesture in THIS pane assigns.

use super::{Wfc3dConfig, Wfc3dConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "change-active-tile")]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeActiveTile {
    pub tile_id: String,
}

impl protocol::MutationKind<Wfc3dConfig, Wfc3dConfigMutation> for ChangeActiveTile {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "active-tile", kind: "change-active-tile", record: "ChangeActiveTile" };
    fn diff(&self, base: &Wfc3dConfig) -> protocol::MutationOutcome<Wfc3dConfig> {
        protocol::MutationOutcome::new(Wfc3dConfig { active_tile_id: self.tile_id.clone(), ..base.clone() })
    }
    fn inverse(&self, base: &Wfc3dConfig) -> Vec<Wfc3dConfigMutation> {
        vec![Wfc3dConfigMutation::ChangeActiveTile(ChangeActiveTile { tile_id: base.active_tile_id.clone() })]
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Change Active Tile", "Aktive Kachel ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["active-tile".into()]
    }
}
