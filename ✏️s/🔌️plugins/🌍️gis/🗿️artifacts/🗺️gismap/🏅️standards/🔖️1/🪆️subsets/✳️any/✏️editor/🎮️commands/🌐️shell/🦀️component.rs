//! 🌐️ GIS 2D play app command — the Shell-kind effect that opens a picked feature's source URL
//! through the host.

use crate::artifacts::gismap::op::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;
use crate::editor::gis2d::config::{Gis2dConfig, Gis2dConfigMutation};
use crate::editor::gis2d::maphost::map_host_from;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️OpenSource
pub mod open_source {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "open-source")]
    pub struct OpenSource {
        pub feature_id: String,
    }

    pub async fn handle(payload: &OpenSource, doc: &ArtifactView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        let host = map_host_from(doc.snapshot, cfg.snapshot);
        match host.features.positions.get(&payload.feature_id).and_then(|row| row.source_url.clone()) {
            Some(url) => Ok(Emit::effect(Effect::OpenExternalUrl { url })),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️OpenSource

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::gis2d::testkit::{app, dispatch};
    use crate::editor::gis2d::Gis2dCommand;

    #[semio_framework_async_macros::async_test]
    async fn open_source_on_an_unknown_feature_emits_no_effect() {
        let mut app = app();
        let result = dispatch(&mut app, Gis2dCommand::OpenSource(open_source::OpenSource { feature_id: "nope".into() }));
        assert!(result.mutations.is_empty());
        assert!(!result.requested_effects.iter().any(|effect| matches!(effect, Effect::OpenExternalUrl { .. })));
    }

    /// 🌐️ A Shell action never emits document operations — the registry's kind-discipline guard
    /// rejects one that does.
    #[semio_framework_async_macros::async_test]
    async fn open_source_is_a_shell_action_that_emits_no_operations() {
        let definition = crate::editor::gis2d::create_gis2d_app().definition;
        let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "openSource").expect("openSource declared");
        assert!(matches!(action.kind, semio_framework_plugin::ActionKind::Shell));
        let mut app = crate::editor::gis2d::testkit::app_with_registry();
        assert!(dispatch(&mut app, Gis2dCommand::OpenSource(open_source::OpenSource { feature_id: "nope".into() })).mutations.is_empty());
    }
}
//#endregion 🧪️Tests
