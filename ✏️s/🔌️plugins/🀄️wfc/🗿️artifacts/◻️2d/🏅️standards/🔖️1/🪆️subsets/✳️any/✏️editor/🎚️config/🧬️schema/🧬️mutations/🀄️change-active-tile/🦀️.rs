//! 🀄️ Change Active Tile in the WFC 2D config facet — which tile a pin gesture in THIS pane assigns.

use super::{Wfc2dConfig, Wfc2dConfigMutation};

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[dsl(keyword = "change-active-tile")]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeActiveTile {
    pub tile_id: String,
}

impl protocol::MutationKind<Wfc2dConfig, Wfc2dConfigMutation> for ChangeActiveTile {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "active-tile", kind: "change-active-tile", record: "ChangeActiveTile" };
    fn diff(&self, base: &Wfc2dConfig) -> protocol::MutationOutcome<Wfc2dConfig> {
        protocol::MutationOutcome::new(Wfc2dConfig { active_tile_id: self.tile_id.clone(), ..base.clone() })
    }
    fn inverse(&self, base: &Wfc2dConfig) -> Vec<Wfc2dConfigMutation> {
        vec![Wfc2dConfigMutation::ChangeActiveTile(ChangeActiveTile { tile_id: base.active_tile_id.clone() })]
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Change Active Tile", "Aktive Kachel ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["active-tile".into()]
    }
}
