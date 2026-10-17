//! 🔄️ Replace Config in the WFC 2D config facet.

use crate::editor::wfc2d::config::mutations::{ChangeActiveTile, ChangeCamera, Wfc2dConfig, Wfc2dConfigDiff};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: Wfc2dConfig,
}

impl<Op> protocol::MutationKind<Wfc2dConfig, Op> for ReplaceConfig
where
    Op: protocol::Mutation<Wfc2dConfig, Diff = Wfc2dConfigDiff> + From<ReplaceConfig> + From<crate::editor::wfc2d::config::mutations::change_active_tile::ChangeActiveTile> + From<crate::editor::wfc2d::config::mutations::change_camera::ChangeCamera>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplaceConfig" };
    fn diff(&self, base: &Wfc2dConfig) -> protocol::MutationOutcome<Wfc2dConfigDiff> {
        let config = &self.config;
        protocol::MutationOutcome::new(Wfc2dConfigDiff {
            camera_x: (base.camera_x != config.camera_x).then_some(config.camera_x),
            camera_y: (base.camera_y != config.camera_y).then_some(config.camera_y),
            camera_zoom: (base.camera_zoom != config.camera_zoom).then_some(config.camera_zoom),
            active_tile_id: (base.active_tile_id != config.active_tile_id).then(|| config.active_tile_id.clone()),
        })
    }
    fn inverse(&self, base: &Wfc2dConfig) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![
            Op::from(ChangeActiveTile { tile_id: base.active_tile_id.clone() }),
            Op::from(ChangeCamera { x: base.camera_x, y: base.camera_y, zoom: base.camera_zoom }),
        ]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Replace Config", "Konfiguration ersetzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["config".into()]
    }
}
