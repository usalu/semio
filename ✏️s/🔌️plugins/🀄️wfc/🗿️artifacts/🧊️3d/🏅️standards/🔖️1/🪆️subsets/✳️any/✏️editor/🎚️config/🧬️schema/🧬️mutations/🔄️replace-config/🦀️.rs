//! 🔄️ Replace Config in the WFC 3D config facet — the whole-record swap a host restore performs.

use super::{ChangeActiveTile, ChangeCamera, Wfc3dConfig, Wfc3dConfigDiff};

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: Wfc3dConfig,
}

impl<Op> protocol::MutationKind<Wfc3dConfig, Op> for ReplaceConfig
where
    Op: protocol::Mutation<Wfc3dConfig, Diff = Wfc3dConfigDiff> + From<ReplaceConfig> + From<crate::editor::wfc3d::config::mutations::change_active_tile::ChangeActiveTile> + From<crate::editor::wfc3d::config::mutations::change_camera::ChangeCamera>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplaceConfig" };
    fn diff(&self, base: &Wfc3dConfig) -> protocol::MutationOutcome<Wfc3dConfigDiff> {
        let config = &self.config;
        protocol::MutationOutcome::new(Wfc3dConfigDiff {
            camera_x: (base.camera_x != config.camera_x).then_some(config.camera_x),
            camera_y: (base.camera_y != config.camera_y).then_some(config.camera_y),
            camera_zoom: (base.camera_zoom != config.camera_zoom).then_some(config.camera_zoom),
            active_tile_id: (base.active_tile_id != config.active_tile_id).then(|| config.active_tile_id.clone()),
        })
    }
    fn inverse(&self, base: &Wfc3dConfig) -> Result<Vec<Op>, semio_framework_value::ValueError> {
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
