//! 🗣️ GIS 2D play app command — the host-pushed locale switch (undeclared in the manifest, never in
//! the command palette; host/test infra dispatches it directly).

use crate::apps::gis2d::config::{Gis2dConfig, Gis2dConfigMutation};
use crate::artifacts::gismap::op::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️SetLocale
pub mod set_locale {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "locale")]
    pub struct SetLocale {
        pub value: String,
    }

    pub fn handle(payload: &SetLocale, _doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis2dConfigMutation::SetLocale { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetLocale

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::gis2d::modes::edit::windows::map::GIS2D_PLAY_WINDOW_MAIN;
    use crate::apps::gis2d::panels::inspection::GIS2D_PLAY_BODY_INSPECTION;
    use crate::apps::gis2d::testkit::{app, dispatch, main_window_measures, render};
    use crate::apps::gis2d::Gis2dCommand;

    #[test]
    fn gis2d_labels_resolve_native_by_default() {
        let mut app = app();
        let json = render(&mut app, GIS2D_PLAY_BODY_INSPECTION);
        assert!(json.contains("\"Map View\""));
        assert!(json.contains("\"Render Mode\""));
        assert!(json.contains("\"Map Layer\""));
        assert!(!json.contains("Kartenansicht"));
    }

    /// 🗣️ Locale is `cfg.locale`, set via the typed `SetLocale` config command — no `ViewModel`-pushed
    /// locale anywhere.
    #[test]
    fn gis2d_labels_translate_inspector_and_layers_in_german() {
        let mut app = app();
        let result = dispatch(&mut app, Gis2dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
        assert!(result.mutations.is_empty(), "locale is config state, not a document edit");

        let inspector_json = render(&mut app, GIS2D_PLAY_BODY_INSPECTION);
        assert!(inspector_json.contains("Kartenansicht"));
        assert!(inspector_json.contains("Darstellungsmodus"));
        assert!(inspector_json.contains("Kartenebene"));
        assert!(!inspector_json.contains("\"Map View\""));

        let document_json = render(&mut app, crate::apps::gis2d::panels::artifact::GIS2D_PLAY_BODY_DOCUMENT);
        assert!(document_json.contains("Wasser"));
        assert!(!document_json.contains("\"Water\""));

        let window_json = serde_json::to_string(&main_window_measures(&mut app)).expect("measures json");
        assert!(window_json.contains("Ebenen"));
        assert!(window_json.contains("Ebenengewichte"));
        assert_eq!(GIS2D_PLAY_WINDOW_MAIN, "gis2d-main");
    }
}
//#endregion 🧪️Tests
