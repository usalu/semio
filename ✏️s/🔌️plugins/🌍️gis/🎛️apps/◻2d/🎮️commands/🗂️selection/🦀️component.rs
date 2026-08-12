//! 🗂️ GIS 2D play app commands — layer selection, feature selection and the marquee vocabulary.
//! Every command here is config-only: it emits `config_mutations`, never document operations.

use crate::apps::gis2d::config::{Gis2dConfig, Gis2dConfigMutation};
use crate::apps::gis2d::maphost::map_host_from;
use crate::artifacts::gismap::op::GisMapMutation;
use crate::artifacts::gismap::GisMapSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;

//#region 🔖️SelectionHelpers
/// 🖱️ Folds an incoming `{positions,routes}` pick into the current feature selection under the given
/// marquee mode (`default` replaces, `additive`/`subtractive`/`invertive` combine).
fn merge_feature_selection(current_json: &str, positions: Vec<String>, routes: Vec<String>, mode: &str) -> Value {
    let current: Value = serde_json::from_str(current_json).unwrap_or(json!({"positions":[],"routes":[]}));
    let current_positions: Vec<String> = current.get("positions").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
    let current_routes: Vec<String> = current.get("routes").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
    let mut next_positions: HashSet<String> = current_positions.iter().cloned().collect();
    let mut next_routes: HashSet<String> = current_routes.iter().cloned().collect();
    let incoming_positions: HashSet<String> = positions.into_iter().collect();
    let incoming_routes: HashSet<String> = routes.into_iter().collect();
    match mode {
        "additive" => {
            next_positions.extend(incoming_positions);
            next_routes.extend(incoming_routes);
        }
        "subtractive" => {
            next_positions.retain(|id| !incoming_positions.contains(id));
            next_routes.retain(|id| !incoming_routes.contains(id));
        }
        "invertive" => {
            for id in incoming_positions {
                if !next_positions.insert(id.clone()) {
                    next_positions.remove(&id);
                }
            }
            for id in incoming_routes {
                if !next_routes.insert(id.clone()) {
                    next_routes.remove(&id);
                }
            }
        }
        _ => {
            next_positions = incoming_positions;
            next_routes = incoming_routes;
        }
    }
    json!({
        "positions": next_positions.into_iter().collect::<Vec<_>>(),
        "routes": next_routes.into_iter().collect::<Vec<_>>(),
    })
}
//#endregion 🔖️SelectionHelpers

//#region 🔖️SetSelection
pub mod set_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "selection")]
    pub struct SetSelection {
        pub ids: Vec<String>,
    }

    pub fn handle(payload: &SetSelection, _doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis2dConfigMutation::SetSelection { ids: payload.ids.clone() }]))
    }
}
//#endregion 🔖️SetSelection

//#region 🔖️SetFeatureSelection
pub mod set_feature_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "feature-selection")]
    pub struct SetFeatureSelection {
        pub positions: Vec<String>,
        pub routes: Vec<String>,
        pub mode: String,
    }

    pub fn handle(payload: &SetFeatureSelection, doc: &ArtifactView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        let config = cfg.snapshot;
        let selection = merge_feature_selection(&config.feature_selection_json, payload.positions.clone(), payload.routes.clone(), &payload.mode);
        let mut host = map_host_from(doc.snapshot, config);
        if host.set_selection_json(&selection.to_string()).is_ok() {
            Ok(Emit::config(vec![Gis2dConfigMutation::SetFeatureSelection { value_json: selection.to_string() }]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️SetFeatureSelection

//#region 🔖️ClearSelection
pub mod clear_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "clear-selection")]
    pub struct ClearSelection {}

    pub fn handle(_payload: &ClearSelection, _doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis2dConfigMutation::SetFeatureSelection { value_json: Gis2dConfig::default().feature_selection_json }]))
    }
}
//#endregion 🔖️ClearSelection

//#region 🔖️SelectAll
pub mod select_all {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "select-all")]
    pub struct SelectAll {}

    pub fn handle(_payload: &SelectAll, doc: &ArtifactView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        let host = map_host_from(doc.snapshot, cfg.snapshot);
        let selection = json!({
            "positions": host.features.positions.keys().cloned().collect::<Vec<_>>(),
            "routes": host.features.routes.keys().cloned().collect::<Vec<_>>(),
        });
        Ok(Emit::config(vec![Gis2dConfigMutation::SetFeatureSelection { value_json: selection.to_string() }]))
    }
}
//#endregion 🔖️SelectAll

//#region 🔖️Deselect
pub mod deselect {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "deselect")]
    pub struct Deselect {
        pub feature_id: String,
        pub feature_kind: String,
    }

    pub fn handle(payload: &Deselect, _doc: &ArtifactView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        let mut selection: Value = serde_json::from_str(&cfg.snapshot.feature_selection_json).unwrap_or(json!({"positions":[],"routes":[]}));
        let bucket = if payload.feature_kind == "position" { "positions" } else { "routes" };
        if let Some(rows) = selection.get_mut(bucket).and_then(|value| value.as_array_mut()) {
            rows.retain(|row| row.as_str() != Some(payload.feature_id.as_str()));
        }
        Ok(Emit::config(vec![Gis2dConfigMutation::SetFeatureSelection { value_json: selection.to_string() }]))
    }
}
//#endregion 🔖️Deselect

//#region 🔖️FocusFeature
pub mod focus_feature {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "focus-feature")]
    pub struct FocusFeature {
        pub feature_id: String,
        pub feature_kind: String,
    }

    pub fn handle(payload: &FocusFeature, doc: &ArtifactView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        let mut host = map_host_from(doc.snapshot, cfg.snapshot);
        if host.focus_feature(&payload.feature_kind, &payload.feature_id) {
            Ok(Emit::config(vec![Gis2dConfigMutation::SetCamera { camera_json: host.camera_json() }]))
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️FocusFeature

//#region 🔖️SetSelectionMethod
pub mod set_selection_method {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "selection-method")]
    pub struct SetSelectionMethod {
        pub value: String,
    }

    pub fn handle(payload: &SetSelectionMethod, _doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis2dConfigMutation::SetSelectionMethod { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetSelectionMethod

//#region 🔖️SetSelectionMode
pub mod set_selection_mode {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "selection-mode")]
    pub struct SetSelectionMode {
        pub value: String,
    }

    pub fn handle(payload: &SetSelectionMode, _doc: &ArtifactView<'_, GisMapSnapshot>, _cfg: &ConfigView<'_, Gis2dConfig>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation>, Fault> {
        Ok(Emit::config(vec![Gis2dConfigMutation::SetSelectionMode { value: payload.value.clone() }]))
    }
}
//#endregion 🔖️SetSelectionMode

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::gis2d::testkit::{app, app_with_registry, dispatch};
    use crate::apps::gis2d::Gis2dCommand;

    #[test]
    fn set_selection_is_view_state_and_emits_no_operations() {
        let mut app = app();
        let result = dispatch(&mut app, Gis2dCommand::SetSelection(set_selection::SetSelection { ids: vec!["roads".into()] }));
        assert!(result.mutations.is_empty(), "selection must not produce document operations");
    }

    /// 👁️ A representative View action mutates only config state, so under the real registry it
    /// emits no operations and never trips the View → emits-operations guard.
    #[test]
    fn selection_actions_emit_no_ops_under_registry_kind_discipline() {
        let mut app = app_with_registry();
        assert!(dispatch(&mut app, Gis2dCommand::SelectAll(select_all::SelectAll {})).mutations.is_empty());
        assert!(dispatch(&mut app, Gis2dCommand::ClearSelection(clear_selection::ClearSelection {})).mutations.is_empty());
        assert!(dispatch(&mut app, Gis2dCommand::SetFeatureSelection(set_feature_selection::SetFeatureSelection { positions: vec!["p1".into()], routes: Vec::new(), mode: "default".into() })).mutations.is_empty());
    }

    /// 🗂️ Probes the emitted `SetFeatureSelection` payload directly rather than the rendered scene:
    /// a feature id appears in the scene descriptor whether or not it is selected, so a substring
    /// check on the render output is not a selection probe.
    #[test]
    fn select_all_then_deselect_drops_just_that_feature() {
        const PIN: &str = "p_institut_de_botanique_ulg_liege";
        let document = crate::artifacts::gismap::schema::default_document();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView { snapshot: &document, history: &history };

        let base = Gis2dConfig::default();
        let all = select_all::handle(&select_all::SelectAll {}, &doc, &ConfigView { snapshot: &base }).expect("selectAll");
        let Some(Gis2dConfigMutation::SetFeatureSelection { value_json }) = all.config_mutations.first().cloned() else {
            panic!("selectAll emits one SetFeatureSelection");
        };
        assert!(value_json.contains(PIN), "select-all writes every position id into the selection");

        let selected = Gis2dConfig { feature_selection_json: value_json, ..Gis2dConfig::default() };
        let dropped = deselect::handle(&deselect::Deselect { feature_id: PIN.into(), feature_kind: "position".into() }, &doc, &ConfigView { snapshot: &selected }).expect("deselect");
        let Some(Gis2dConfigMutation::SetFeatureSelection { value_json }) = dropped.config_mutations.first().cloned() else {
            panic!("deselect emits one SetFeatureSelection");
        };
        assert!(!value_json.contains(PIN), "the deselected feature is gone from the selection");
    }

    #[test]
    fn merge_feature_selection_honours_every_marquee_mode() {
        let base = r#"{"positions":["a"],"routes":[]}"#;
        let sorted = |value: Value| {
            let mut ids: Vec<String> = serde_json::from_value(value.get("positions").cloned().unwrap_or(json!([]))).unwrap_or_default();
            ids.sort();
            ids
        };
        assert_eq!(sorted(merge_feature_selection(base, vec!["b".into()], Vec::new(), "default")), vec!["b".to_string()]);
        assert_eq!(sorted(merge_feature_selection(base, vec!["b".into()], Vec::new(), "additive")), vec!["a".to_string(), "b".to_string()]);
        assert_eq!(sorted(merge_feature_selection(base, vec!["a".into()], Vec::new(), "subtractive")), Vec::<String>::new());
        assert_eq!(sorted(merge_feature_selection(base, vec!["a".into()], Vec::new(), "invertive")), Vec::<String>::new());
    }

    #[test]
    fn focus_feature_on_an_unknown_id_emits_nothing() {
        let mut app = app();
        let result = dispatch(&mut app, Gis2dCommand::FocusFeature(focus_feature::FocusFeature { feature_id: "nope".into(), feature_kind: "position".into() }));
        assert!(result.mutations.is_empty());
    }
}
//#endregion 🧪️Tests
