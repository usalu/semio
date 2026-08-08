//! 🌐️ GIS 2D play app command — the Shell-kind effect that opens a picked feature's source URL
//! through the host.

use crate::apps::gis2d::config::{Gis2dConfig, Gis2dConfigMutation};
use crate::apps::gis2d::maphost::map_host_from;
use crate::artifacts::gismap::op::GisMapMutation;
use crate::artifacts::gismap::GisMapDocument;
use semio_framework_plugin::kernel::HostEffect;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️OpenSource
pub mod open_source {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "open-source")]
    pub struct OpenSource {
        pub feature_id: String,
    }

    pub fn handle(payload: &OpenSource, doc: &DocumentView<'_, GisMapDocument>, cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        let host = map_host_from(doc.projection, cfg.projection);
        match host.features.positions.get(&payload.feature_id).and_then(|row| row.source_url.clone()) {
            Some(url) => Ok(Emit::effect(HostEffect::OpenExternalUrl { url })),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️OpenSource

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::gis2d::testkit::{app, dispatch};
    use crate::apps::gis2d::Gis2dCommand;

    #[test]
    fn open_source_on_an_unknown_feature_emits_no_effect() {
        let mut app = app();
        let result = dispatch(&mut app, Gis2dCommand::OpenSource(open_source::OpenSource { feature_id: "nope".into() }));
        assert!(result.mutations.is_empty());
        assert!(!result.requested_effects.iter().any(|effect| matches!(effect, HostEffect::OpenExternalUrl { .. })));
    }

    /// 🌐️ A Shell action never emits document operations — the registry's kind-discipline guard
    /// rejects one that does.
    #[test]
    fn open_source_is_a_shell_action_that_emits_no_operations() {
        let definition = crate::apps::gis2d::create_gis2d_app().definition;
        let action = definition.actions.iter().find(|action| action.id == "openSource").expect("openSource declared");
        assert!(matches!(action.kind, semio_framework_plugin::ActionKind::Shell));
        let mut app = crate::apps::gis2d::testkit::app_with_registry();
        assert!(dispatch(&mut app, Gis2dCommand::OpenSource(open_source::OpenSource { feature_id: "nope".into() })).operations.is_empty());
    }
}
//#endregion 🧪️Tests
