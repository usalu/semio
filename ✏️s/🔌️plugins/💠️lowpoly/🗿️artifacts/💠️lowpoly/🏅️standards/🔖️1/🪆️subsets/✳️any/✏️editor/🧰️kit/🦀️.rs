//! 🧰️ lowpoly editor kit: identifiers, the dispatch context and the ui-value and helper plumbing every command, mode
//! and panel builds on.

extern crate semio_framework_os_kernel as store;

use semio_s_artifact_lowpoly_lowpoly_model::{snapshot::LOWPOLY_DOCUMENT_SCHEMA, snapshot::LowpolySnapshot};
use semio_s_artifact_lowpoly_lowpoly_config::LowpolyConfig;
use semio_s_artifact_lowpoly_lowpoly_view::{editor::lowpoly::terminology::LowpolyLabels, LowpolyView, utility_param_f64};
use semio_framework_plugin::{ActionDescriptor, WindowEngagement, WindowEngagementInput, WindowEngagementOption, WindowEngagementPossible, WindowEngagementStatus, WindowMeasure};
use semio_framework_ui_locale::LabelText;
use pack::snapshot::ArtifactPack;

//#region 🔖️Constants
pub const LOWPOLY_PLAY_APP_ID: &str = "lowpoly-play";
pub const LOWPOLY_PLAY_CONTROLLER_ID: &str = "lowpoly-play";
/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`🛠️options/*`, `📌️panels/*`, window/engagement builders) builds its `on_change`/item actions with.
pub fn lowpoly_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(LOWPOLY_PLAY_CONTROLLER_ID).action(action, args)
}
/// 🪟️ Bridges window chrome, which still carries the retained WGPU action descriptor.
pub fn lowpoly_window_action(action: &str, args: Option<semio_framework::DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: LOWPOLY_PLAY_CONTROLLER_ID.into(), action: action.into(), args }
}
/// 🏷️ Admits resolved Lowpoly text into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "lowpoly UI label admission failed"))
}
/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}
/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}
/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}
/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}
/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}
//#endregion 🔖️Constants
//#region 🔖️SharedMeasures

/// 🧮️ Shared leaf builder for one utility-param slider — used by the `🧲️snap` option and by
/// `paint_utility_params_group` below.
#[allow(clippy::too_many_arguments, reason = "one WindowMeasure::Slider literal per call site; a params struct would only move the same 8 fields around for this single builder")]
pub fn utility_param_slider(id: &str, label: LabelText, key: &str, params: &serde_json::Value, default: f64, min: f64, max: f64, step: f64) -> WindowMeasure {
    WindowMeasure::Slider {
        id: format!("lowpoly-measure-{id}"),
        label: Some(label.into()),
        value: utility_param_f64(params, key, default),
        min,
        max,
        step: Some(step),
        ready: None,
        loading: None,
        disabled: None,
        on_change: lowpoly_window_action("setUtilityParam", Some(semio_framework_value::DslValue::object([("key".to_string(), semio_framework_value::DslValue::String(key.to_string()))]))),
        waiting: None,
    }
}
/// 🖌️ Utility Options for a stamping paint utility (`brush`/`eraser`) — the live brush size/opacity/
/// hardness sliders, tagged `active_utility_id: Some(utility)` so `partition_window_measures` surfaces
/// them in the Utility Options rail only while that exact utility is active. Both utilities stamp
/// through the same `stamp_brush` path, so they share an identical param set.
pub fn paint_utility_params_group(utility: &str, params: &serde_json::Value, labels: &LowpolyLabels) -> WindowMeasure {
    let slider = |suffix: &str, label: LabelText, key: &str, default: f64, min: f64, max: f64, step: f64| WindowMeasure::Slider {
        id: format!("lowpoly-measure-{utility}-{suffix}"),
        label: Some(label.into()),
        value: utility_param_f64(params, key, default),
        min,
        max,
        step: Some(step),
        ready: None,
        loading: None,
        disabled: None,
        on_change: lowpoly_window_action("setUtilityParam", Some(semio_framework_value::DslValue::object([("key".to_string(), semio_framework_value::DslValue::String(key.to_string()))]))),
        waiting: None,
    };
    WindowMeasure::Group {
        id: format!("lowpoly-measure-paint-params-{utility}"),
        label: labels.brush_group.into(),
        default_open: Some(true),
        active_utility_id: Some(utility.into()),
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            slider("size", labels.brush_size, "brushSize", 16.0, 1.0, 128.0, 1.0),
            slider("opacity", labels.brush_opacity, "brushOpacity", 1.0, 0.0, 1.0, 0.05),
            slider("hardness", labels.brush_hardness, "brushHardness", 0.5, 0.0, 1.0, 0.05),
        ],
    }
}
//#endregion 🔖️SharedMeasures
//#region 🔖️ResetDocument
/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `scene` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (mesh import, file
/// open, dev fixture load). Per `📓️taxonomy.md`, whole-document replace is banned outright with NO
/// replacement mutation: whole-document replace is not expressible as an in-history `Mutation` at
/// all. Every former "replace the whole document" gesture in this package (`import_media`'s
/// `"mesh:in"`/`"artifact:in"` above, `commands::document::{set_snapshot_json,replace_snapshot_json}`)
/// builds this effect instead of an `Emit::mutations([...])`. The spr is a fresh, edit-free op-log
/// (`store::os_store::text_format::empty_document_spr`, the `🏗️fem`/process3d shape) — never a live `ArtifactEnvelope`
/// minted just to print it: such an envelope owns a bounded retirement authority, and dropping it at
/// the end of this function trapped the guest (`artifact envelope terminal shell reached Drop before
/// its app-owned bounded retirement authority detached every nested owner`) on every mesh import and
/// `replaceSnapshotJson` (ticket 26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS, 2026-09-17).
pub fn reset_document_effect(scene: &LowpolySnapshot) -> semio_framework_plugin::Effect {
    let pack = <LowpolySnapshot as ArtifactPack>::encode_pack(scene);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("lowpoly", LOWPOLY_DOCUMENT_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument
