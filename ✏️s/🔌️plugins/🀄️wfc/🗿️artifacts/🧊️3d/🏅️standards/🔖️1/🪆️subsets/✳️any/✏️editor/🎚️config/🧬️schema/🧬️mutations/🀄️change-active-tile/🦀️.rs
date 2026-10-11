//! 🀄️ Change Active Tile in the WFC 3D config facet — which tile a pin gesture in THIS pane assigns.

use super::{Wfc3dConfig, Wfc3dConfigDiff, Wfc3dConfigMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "change-active-tile")]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeActiveTile {
    pub tile_id: String,
}

impl protocol::MutationKind<Wfc3dConfig, Wfc3dConfigMutation> for ChangeActiveTile {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "active-tile", kind: "change-active-tile", record: "ChangeActiveTile" };
    fn diff(&self, base: &Wfc3dConfig) -> protocol::MutationOutcome<Wfc3dConfigDiff> {
        if base.active_tile_id == self.tile_id {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The active tile is already set.");
        }
        protocol::MutationOutcome::new(Wfc3dConfigDiff { active_tile_id: Some(self.tile_id.clone()), ..Default::default() })
    }
    fn inverse(&self, base: &Wfc3dConfig) -> Result<Vec<Wfc3dConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Wfc3dConfigMutation::ChangeActiveTile(ChangeActiveTile { tile_id: base.active_tile_id.clone() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change Active Tile", "Aktive Kachel ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["active-tile".into()]
    }
}
