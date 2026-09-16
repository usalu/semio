//! 🖥️ Shooting editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch. Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: moved here verbatim from
//! the retired `🎛️apps/🎥️shooting/🦀️.rs`, `impl ArtifactApp` → `impl ArtifactEditor`.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, chrome measures in those windows' `☑️options/*`, panel trees in `📌️panels/*`,
//! labels in `🦀️terminology.rs`, view state in `🦀️config.rs`, shared compute in the artifact's `⚙️engine`.
//! This file is a routing table: `handle` → `ShootingCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

use crate::editor::shooting::commands::{asset, camera, export, document, gumball, scene, selection, shot};
use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::modes::edit;
use crate::editor::shooting::modes::edit::windows::icon as icon_window;
use crate::editor::shooting::modes::edit::windows::scene as scene_window;
use crate::editor::shooting::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::shooting::presence::{ShootingPresence, ShootingPresenceMutation};
use crate::editor::shooting::terminology::shooting_play_labels;
use crate::op::ShootingMutation;
use crate::{ShootingSnapshot, SHOOTING_DOCUMENT_SCHEMA};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    tree_item_with_action, ActionArgDef, ActionArgOption, ActionDefinition, ActionKind, AppIo, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract,
    ArtifactToolPublicationLane, ArtifactView, ConfigView, Dialect, DraftView, DslValue,
    Editor, EditorApp, Emit, Fault, FaultCode, FaultOrigin, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractiveJobClassification, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm,
    MediaPayload, MediaType, MergeMode, NoDraft, NoDraftMutation, OsMediaCapability, SelectionMethod, SelectionMode, SelectionSpec, UtilityDefinition, WindowEngagement, WindowMeasure,
};
use std::collections::HashMap;
use store::EngineHandles;

//#region 🔖️Constants
pub const SHOOTING_PLAY_APP_ID: &str = "s.shooting.shooting@1/*#editor";
const SHOOTING_PLAY_CONTROLLER_ID: &str = SHOOTING_PLAY_APP_ID;
/// 🕹️ The framework-owned interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM)?
/// covering asset pick/marquee selection and pointer hover in the 3d scene window — granularity `"asset"`
/// only, `HierarchyProvider::Flat`. Shot selection is NOT part of this domain — see
/// `ShootingConfig::selected_shot_ids`'s doc comment.
pub const SHOOTING_INTERACTION_DOMAIN: &str = "assets";
pub use crate::editor::shooting::commands::document::set_active_example::{SHOOTING_EXAMPLE_DEFAULT_ID, SHOOTING_EXAMPLE_HEXAGONAL_CUT_CONCRETE_FOREST_LEFT};
pub use catalogue_panel::SHOOTING_PLAY_BODY_CATALOGUE;
pub use document_panel::SHOOTING_PLAY_BODY_ARTIFACT;
pub use icon_window::SHOOTING_PLAY_BODY_ICON;
pub use icon_window::SHOOTING_PLAY_WINDOW_ICON;
pub use inspection_panel::SHOOTING_PLAY_BODY_INSPECTION;
pub use scene_window::SHOOTING_PLAY_BODY_SCENE;
pub use scene_window::SHOOTING_PLAY_WINDOW_SCENE;
//#endregion 🔖️Constants

//#region 🔖️Utilities
/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`☑️options/*`, `📌️panels/*`) builds its `on_change`/item actions with.
pub fn shooting_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(SHOOTING_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🎛️ Addresses host window chrome with its native action descriptor.
pub fn shooting_window_action(action: &str, args: Option<DslValue>) -> semio_framework_plugin::ActionDescriptor {
    semio_framework_plugin::ActionDescriptor { controller_id: SHOOTING_PLAY_CONTROLLER_ID.into(), action: action.into(), args }
}

/// 🏷️ Admits localized text into the semantic UI contract.
pub fn ui_label(value: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    value.try_into().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.label.capacity", "fixed UI label admission failed"))
}

/// 📝️ Admits a fixed shooting UI string.
pub fn ui_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::UiText> {
    semio_framework_ui_contract::UiText::try_from_str(value.as_ref()).ok_or_else(ui_capacity_error)
}

/// 🧱️ Finalizes a shooting UI node with explicit identity.
pub fn ui_node<B: semio_framework_ui_contract::HasBase + semio_framework_ui_contract::Buildable>(builder: B, id: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    builder.try_id(id).map_err(|_| ui_capacity_error())?.try_build().map_err(|_| ui_capacity_error())
}

/// 👶️ Admits a complete collection of shooting UI children.
pub fn ui_children<B: semio_framework_ui_contract::HasChildren>(builder: B, children: impl IntoIterator<Item = semio_framework_plugin::BuiltNode>) -> semio_framework_plugin::UiAssemblyResult<B> {
    builder.try_children(children).map_err(|_| ui_capacity_error())
}

/// 🚧️ Reports fixed-capacity UI admission failure.
pub fn ui_capacity_error() -> semio_framework_plugin::PluginAssemblyError {
    semio_framework_plugin::PluginAssemblyError::new("shooting.ui.capacity", "shooting UI admission failed")
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

/// 🌳️ Layers an `icon_id` onto the SDK's `tree_item_with_action` skeleton — the SDK primitive's third
/// parameter is `description`, not an icon, so the shooting-specific icon assignment stays local. Shared
/// by the document and catalogue panels (two consumers)?.
pub fn tree_item_with_icon(
    id: impl AsRef<str>,
    label: impl TryInto<Label>,
    icon_id: &str,
    action: semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let label: Label = label.try_into().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.tree-item.label", "tree-item label conversion failed"))?;
    let mut node = tree_item_with_action(id, label.as_str(), None, action?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.icon = Some(semio_framework_plugin::UiText::try_from_str(icon_id).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.tree-item.icon", "fixed tree-item icon admission failed"))?);
    }
    Ok(node)
}
//#endregion 🔖️Utilities

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `crate::artifact_kind` already declares (schema/media type/presentation fields
/// copied verbatim); the sole app-specific port is `photos:out` (see `shooting_photos_out_port` below)
/// — the implicit document in/out ports cover the rest.
///
/// ⚠️ `export_formats`/`import_formats` stay empty: `AppIo` (unlike `ArtifactKindSpec`) carries no
/// `export_stdio_kinds`/`import_stdio_kinds` string peer to hold the real `["s.stdio.svg",
/// "s.stdio.png"]` list, and its field type (a `Vec` of the framework's closed media-format enum) is
/// framework-owned (`🧰️framework/🔨️modules/🛂️manifest`), out of this plugin's write scope. Confirmed
/// dead as of this migration —
/// `app.io.export_formats`/`import_formats` have no framework reader (`app.io.all_ports()`/
/// `document_schema`/`artifact.component_kind` are the only fields anything consumes) — so emptying
/// them drops no live behavior. `crate::artifact_kind()`'s `export_stdio_kinds`/
/// `import_stdio_kinds` remain the live source of truth for this artifact's real format list.
pub fn shooting_io() -> AppIo {
    AppIo {
        artifact_schema: "shooting.scene".into(),
        artifact_media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        ports: vec![shooting_photos_out_port()],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "2d.shooting".into(), name: "2D Shooting".into(), dimension: "2d".into(), component_kind: "shooting".into() },
    }
}

/// 🔌️ `photos:out` — the shooting document's captured photo(s), as `2d.image` raster media (workflow
/// port surface; WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE Wave 2 port recipe).
/// `Many`/optional: a shooting document may carry several shots, and downstream consumers (e.g.
/// remodel's `photos:in`) may connect before any shot exists.
pub fn shooting_photos_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "photos:out".into(),
        label: "Photos".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        kind_id: Some("2d.image".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🖼️ Exports the active shot's rendered scene as a `2d.image` `Media` payload for the `photos:out`
/// port — reuses the same SVG-then-rasterize pipeline (`crate::standards::v1::subsets::any::schema::shooting_scene_svg` +
/// `rasterize_svg_to_png_base64`) as the `exportActiveShot`/PNG shell action, so there is exactly one
/// photo renderer.
pub fn shooting_photo_media(snapshot: &ShootingSnapshot) -> Result<Media, MediaError> {
    let (svg, width, height) = crate::standards::v1::subsets::any::schema::shooting_scene_svg(snapshot).map_err(|error| MediaError::Payload("photos:out".into(), error))?;
    let png_base64 = semio_framework_os::rasterize_svg_to_png_base64(&svg, width, height).map_err(|error| MediaError::Payload("photos:out".into(), error))?;
    Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: png_base64 } })
}
//#endregion 🔖️Io

//#region 🔖️Commands
/// 🕹️ Per-dispatch app-struct state that is neither document nor config: a read-only snapshot of the
/// `"assets"` interaction domain's current selection ids (see [`SHOOTING_INTERACTION_DOMAIN`]). The
/// `semio_framework_plugin::app_commands!`-generated `dispatch` has no way to thread `InteractionView`
/// itself (see that macro's own doc comment on `ctx`), so `ArtifactApp::handle` reads it once and hands
/// it down through this app-owned context instead — used by the retained `translate/rotate/scale-
/// Selection` gumball verbs (`🎮️commands/🧭️gumball`) as their fallback-to-current-selection source.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ShootingDispatchCtx {
    pub selected_asset_ids: Vec<String>,
}

semio_framework_plugin::app_commands! {
    /// 🎯️ `ShootingPlayApp::Command` — the SOLE dispatch surface for shooting's own behavior, assembled
    /// from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the
    /// kebab-case `#[dsl(key = ..)]` the binary/text codec uses) — different vocabularies, and
    /// appending is safe, reordering is a wire-format break.**
    pub enum ShootingCommand for ShootingSnapshot, ShootingMutation, ShootingConfig, ShootingConfigMutation, ctx = ShootingDispatchCtx {
        "importSnapshotJson" as "import-snapshot-json" => import_snapshot_json::ImportSnapshotJson,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "setActiveShot" as "active-shot" => set_active_shot::SetActiveShot,
        "setActiveAsset" as "active-asset" => set_active_asset::SetActiveAsset,
        "setShotCamera" as "shot-camera" => set_shot_camera::SetShotCamera,
        "saveCamera" as "save-camera" => save_camera::SaveCamera,
        "setSunAzimuth" as "sun-azimuth" => set_sun_azimuth::SetSunAzimuth,
        "setSunElevation" as "sun-elevation" => set_sun_elevation::SetSunElevation,
        "setSunIntensity" as "sun-intensity" => set_sun_intensity::SetSunIntensity,
        "setAmbientIntensity" as "ambient-intensity" => set_ambient_intensity::SetAmbientIntensity,
        "setMaterialRoughness" as "material-roughness" => set_material_roughness::SetMaterialRoughness,
        "setShadowEnabled" as "shadow-enabled" => set_shadow_enabled::SetShadowEnabled,
        "toggleSun" as "toggle-sun" => toggle_sun::ToggleSun,
        "setActiveShotLabel" as "active-shot-label" => set_active_shot_label::SetActiveShotLabel,
        "setActiveShotFormat" as "active-shot-format" => set_active_shot_format::SetActiveShotFormat,
        "setActiveShotShape" as "active-shot-shape" => set_active_shot_shape::SetActiveShotShape,
        "patchShots" as "patch-shots" => patch_shots::PatchShots,
        "patchAssets" as "patch-assets" => patch_assets::PatchAssets,
        "addShot" as "add-shot" => add_shot::AddShot,
        "addAsset" as "add-asset" => add_asset::AddAsset,
        "importAsset" as "import-asset" => import_asset::ImportAsset,
        "resetFixture" as "reset-snapshot" => reset_snapshot::ResetSnapshot,
        "translateSelection" as "translate-selection" => translate_selection::TranslateSelection,
        "rotateSelection" as "rotate-selection" => rotate_selection::RotateSelection,
        "scaleSelection" as "scale-selection" => scale_selection::ScaleSelection,
        "setCamera" as "camera" => set_camera::SetCamera,
        "loadSavedCamera" as "load-saved-camera" => load_saved_camera::LoadSavedCamera,
        "setCameraDraftLabel" as "camera-draft-label" => set_camera_draft_label::SetCameraDraftLabel,
        "setCenterModel" as "center-model" => set_center_model::SetCenterModel,
        "setShotSelection" as "set-shot-selection" => set_shot_selection::SetShotSelection,
        "worldPointerDown" as "world-pointer-down" => world_pointer_down::WorldPointerDown,
        "worldPointerMove" as "world-pointer-move" => world_pointer_move::WorldPointerMove,
        "saveDownload" as "save-download" => save_download::SaveDownload,
        "loadRequest" as "load-request" => load_request::LoadRequest,
        "importAssetRequest" as "import-asset-request" => import_asset_request::ImportAssetRequest,
        "exportActiveShot" as "export-active-shot" => export_active_shot::ExportActiveShot,
        "exportAllShots" as "export-all-shots" => export_all_shots::ExportAllShots,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use asset::{add_asset, import_asset, import_asset_request, patch_assets, set_active_asset};
use camera::{load_saved_camera, save_camera, set_camera, set_camera_draft_label, set_shot_camera};
use export::{export_active_shot, export_all_shots};
use document::{import_snapshot_json, load_request, reset_snapshot, save_download, set_active_example};
use gumball::{rotate_selection, scale_selection, translate_selection};
use scene::{set_ambient_intensity, set_material_roughness, set_shadow_enabled, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use selection::{set_center_model, set_shot_selection, world_pointer_down, world_pointer_move};
use shot::{add_shot, patch_shots, set_active_shot, set_active_shot_format, set_active_shot_label, set_active_shot_shape};
//#endregion 🔖️Commands

//#region 🔖️ActionBridge
/// 🎯️ Host-action bridge into the closed `ShootingCommand` enum (ticket
/// 26/09/16/SHOOTING-PLUGIN-END-TO-END). The React/wgpu shells still speak `{action, args}` with
/// camelCase argument keys and the host's control contracts (`value` for sliders/selects/inputs,
/// `pressed` for toggles, `{mode, ids, dx…}` for the gumball, `{windowId, camera}` for the viewport);
/// every `🎮️commands/*` payload derives `FromValue` over its own snake_case field names, so this
/// boundary folds the keys, applies the per-verb aliases and decodes — the default trait impl refuses
/// every app action outright, which left every panel/measure/viewport gesture dead in the shell.
mod args_bridge {
    use super::*;
    use semio_framework_plugin::{FaultCode, FaultOrigin};

    fn snake(key: &str) -> String {
        let mut out = String::with_capacity(key.len() + 4);
        for ch in key.chars() {
            if ch.is_ascii_uppercase() {
                out.push('_');
                out.push(ch.to_ascii_lowercase());
            } else {
                out.push(ch);
            }
        }
        out
    }

    fn put(entries: &mut Vec<(String, DslValue)>, key: &str, value: DslValue) {
        entries.retain(|(existing, _)| existing != key);
        entries.push((key.to_string(), value));
    }

    /// 🔢️ The host's JSON round trip turns every integer into `Number::Float`; the `u64`/`i64` codecs
    /// decode EXACT integers only, so whole finite floats get their integer variant back (`f64` fields
    /// accept any `Number` variant, so nothing else changes).
    fn integral(value: DslValue) -> DslValue {
        match value {
            DslValue::Number(dsl::Number::Float(float)) if float.is_finite() && float.fract() == 0.0 && float.abs() < 9.007_199_254_740_992e15 => {
                if float >= 0.0 { DslValue::Number(dsl::Number::UInt(float as u64)) } else { DslValue::Number(dsl::Number::Int(float as i64)) }
            }
            DslValue::Array(items) => DslValue::Array(items.into_iter().map(integral).collect()),
            DslValue::Object(entries) => DslValue::Object(entries.into_iter().map(|(key, value)| (key, integral(value))).collect()),
            other => other,
        }
    }

    /// 📝️ Prints a host control value into the `String` field the patch verbs carry (`"512"`, `"true"`,
    /// or the text itself) — the reducers re-parse per field.
    fn stringify(value: DslValue) -> DslValue {
        match value {
            DslValue::String(_) => value,
            DslValue::Null => DslValue::String(String::new()),
            other => DslValue::String(dsl::json::to_json_string(&other)),
        }
    }

    /// 🔁️ Snake-cases every key of `args`, applies `aliases` (snake_case source → destination, first
    /// present source wins and never overwrites a present destination) and seeds `defaults` for keys
    /// still absent.
    fn fold(args: Option<&DslValue>, aliases: &[(&str, &str)], defaults: &[(&str, DslValue)]) -> DslValue {
        let mut entries: Vec<(String, DslValue)> = Vec::new();
        if let Some(DslValue::Object(object)) = args {
            for (key, value) in object {
                put(&mut entries, &snake(key), integral(value.clone()));
            }
        }
        for (from, into) in aliases {
            if entries.iter().any(|(key, _)| key == into) {
                continue;
            }
            if let Some((_, value)) = entries.iter().find(|(key, _)| key == from).cloned() {
                put(&mut entries, into, value);
            }
        }
        for (key, value) in defaults {
            if !entries.iter().any(|(existing, _)| existing == key) {
                entries.push(((*key).to_string(), value.clone()));
            }
        }
        DslValue::Object(entries)
    }

    /// 📷️ The viewport nests its pose under `camera` (`worldCameraSetCameraDispatchArgs`); a flat
    /// `{position, target, …}` payload is admitted as the pose itself.
    fn nest_camera(mut folded: DslValue) -> DslValue {
        if let DslValue::Object(entries) = &mut folded {
            if !entries.iter().any(|(key, _)| key == "camera") {
                let pose = DslValue::Object(entries.iter().filter(|(key, _)| matches!(key.as_str(), "position" | "target" | "zoom" | "fov" | "up" | "projection")).cloned().collect());
                entries.push(("camera".into(), pose));
            }
        }
        folded
    }

    fn with_text_value(mut folded: DslValue) -> DslValue {
        if let DslValue::Object(entries) = &mut folded {
            if let Some(slot) = entries.iter_mut().find(|(key, _)| key == "value") {
                slot.1 = stringify(slot.1.clone());
            }
        }
        folded
    }

    fn decode<T: dsl::FromValue>(action: &str, value: DslValue) -> Result<T, Fault> {
        T::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("shooting action '{action}' arguments do not decode: {error}")))
    }

    pub fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<ShootingCommand, Fault> {
        const IDS: &[(&str, &str)] = &[("ids", "asset_ids"), ("asset_id", "asset_ids")];
        const SHOT_IDS: &[(&str, &str)] = &[("ids", "shot_ids"), ("shot_id", "shot_ids")];
        const PRESSED: &[(&str, &str)] = &[("pressed", "value")];
        let zero = || DslValue::Number(dsl::Number::Float(0.0));
        let one = || DslValue::Number(dsl::Number::Float(1.0));
        let text = |value: &str| DslValue::String(value.into());
        let plain = || fold(args, &[], &[]);
        Ok(match action {
            "importSnapshotJson" => ShootingCommand::ImportSnapshotJson(decode(action, fold(args, &[("value", "json"), ("document", "json")], &[]))?),
            "setActiveExample" => ShootingCommand::SetActiveExample(decode(action, fold(args, &[("id", "example_id"), ("value", "example_id")], &[("example_id", text(SHOOTING_EXAMPLE_DEFAULT_ID))]))?),
            "setActiveShot" => ShootingCommand::SetActiveShot(decode(action, fold(args, &[("value", "shot_id"), ("id", "shot_id")], &[]))?),
            "setActiveAsset" => ShootingCommand::SetActiveAsset(decode(action, fold(args, &[("value", "asset_id"), ("id", "asset_id")], &[]))?),
            "setShotCamera" => ShootingCommand::SetShotCamera(decode(action, nest_camera(fold(args, &[("id", "shot_id")], &[])))?),
            "saveCamera" => ShootingCommand::SaveCamera(decode(action, plain())?),
            "setSunAzimuth" => ShootingCommand::SetSunAzimuth(decode(action, plain())?),
            "setSunElevation" => ShootingCommand::SetSunElevation(decode(action, plain())?),
            "setSunIntensity" => ShootingCommand::SetSunIntensity(decode(action, plain())?),
            "setAmbientIntensity" => ShootingCommand::SetAmbientIntensity(decode(action, plain())?),
            "setMaterialRoughness" => ShootingCommand::SetMaterialRoughness(decode(action, plain())?),
            "setShadowEnabled" => ShootingCommand::SetShadowEnabled(decode(action, fold(args, PRESSED, &[]))?),
            "toggleSun" => ShootingCommand::ToggleSun(decode(action, fold(args, PRESSED, &[]))?),
            "setActiveShotLabel" => ShootingCommand::SetActiveShotLabel(decode(action, with_text_value(plain()))?),
            "setActiveShotFormat" => ShootingCommand::SetActiveShotFormat(decode(action, with_text_value(plain()))?),
            "setActiveShotShape" => ShootingCommand::SetActiveShotShape(decode(action, with_text_value(plain()))?),
            "patchShots" => ShootingCommand::PatchShots(decode(action, with_text_value(fold(args, SHOT_IDS, &[])))?),
            "patchAssets" => ShootingCommand::PatchAssets(decode(action, with_text_value(fold(args, IDS, &[])))?),
            "addShot" => ShootingCommand::AddShot(decode(action, fold(args, &[], &[("format", text("png")), ("shape", text("rectangle"))]))?),
            "addAsset" => ShootingCommand::AddAsset(decode(action, fold(args, &[], &[("format", text("glb"))]))?),
            "importAsset" => ShootingCommand::ImportAsset(decode(action, fold(args, &[("value", "payload")], &[]))?),
            "resetFixture" => ShootingCommand::ResetSnapshot(decode(action, plain())?),
            "translateSelection" => ShootingCommand::TranslateSelection(decode(action, fold(args, IDS, &[("asset_ids", DslValue::Array(Vec::new())), ("dx", zero()), ("dy", zero()), ("dz", zero())]))?),
            "rotateSelection" => ShootingCommand::RotateSelection(decode(action, fold(args, IDS, &[("asset_ids", DslValue::Array(Vec::new())), ("ax", zero()), ("ay", zero()), ("az", one()), ("angle", zero())]))?),
            "scaleSelection" => ShootingCommand::ScaleSelection(decode(action, fold(args, IDS, &[("asset_ids", DslValue::Array(Vec::new())), ("sx", one()), ("sy", one()), ("sz", one())]))?),
            "setCamera" => ShootingCommand::SetCamera(decode(action, nest_camera(plain()))?),
            "loadSavedCamera" => ShootingCommand::LoadSavedCamera(decode(action, fold(args, &[("value", "id"), ("camera_id", "id")], &[]))?),
            "setCameraDraftLabel" => ShootingCommand::SetCameraDraftLabel(decode(action, with_text_value(plain()))?),
            "setCenterModel" => ShootingCommand::SetCenterModel(decode(action, fold(args, &[("value", "pressed")], &[]))?),
            "setShotSelection" => ShootingCommand::SetShotSelection(decode(action, fold(args, SHOT_IDS, &[("shot_ids", DslValue::Array(Vec::new()))]))?),
            "worldPointerDown" => ShootingCommand::WorldPointerDown(decode(action, plain())?),
            "worldPointerMove" => ShootingCommand::WorldPointerMove(decode(action, plain())?),
            "saveDownload" => ShootingCommand::SaveDownload(decode(action, plain())?),
            "loadRequest" => ShootingCommand::LoadRequest(decode(action, plain())?),
            "importAssetRequest" => ShootingCommand::ImportAssetRequest(decode(action, plain())?),
            "exportActiveShot" => ShootingCommand::ExportActiveShot(decode(action, plain())?),
            "exportAllShots" => ShootingCommand::ExportAllShots(decode(action, plain())?),
            _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the shooting editor has no command for action '{action}'"))),
        })
    }
}
//#endregion 🔖️ActionBridge

//#region 🔖️ShootingPlayApp
/// 🧪️ B1: unit struct — every former runtime field now lives in `ShootingConfig`, written through
/// `ShootingConfigMutation`s.
#[derive(Default)]
pub struct ShootingPlayApp;

//#region 🧵️RetainedCommands
/// 🧵️ Every shell-reachable verb is a bounded first-step tool (ticket 26/09/16/SHOOTING-PLUGIN-END-TO-END):
/// the framework refuses UI dispatch of any command not classified `Migrated`, so a partial roster left
/// every panel/measure/viewport gesture dead. Order mirrors the `ShootingCommand` rows.
const SHOOTING_BOUNDED_TOOL_IDS: &[&str] = &[
    "importSnapshotJson",
    "setActiveExample",
    "setActiveShot",
    "setActiveAsset",
    "setShotCamera",
    "saveCamera",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setAmbientIntensity",
    "setMaterialRoughness",
    "setShadowEnabled",
    "toggleSun",
    "setActiveShotLabel",
    "setActiveShotFormat",
    "setActiveShotShape",
    "patchShots",
    "patchAssets",
    "addShot",
    "addAsset",
    "importAsset",
    "resetFixture",
    "translateSelection",
    "rotateSelection",
    "scaleSelection",
    "setCamera",
    "loadSavedCamera",
    "setCameraDraftLabel",
    "setCenterModel",
    "setShotSelection",
    "worldPointerDown",
    "worldPointerMove",
    "saveDownload",
    "loadRequest",
    "importAssetRequest",
    "exportActiveShot",
    "exportAllShots",
];
const SHOOTING_RETAINED_PAYLOAD_SCHEMA: &str = "shooting.shooting.tool-command.v1";
const SHOOTING_BOUNDED_RAW_BYTES: usize = 65_536;
const SHOOTING_BOUNDED_WORK_ITEMS: usize = 1;

fn shooting_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(SHOOTING_BOUNDED_RAW_BYTES, 64, SHOOTING_BOUNDED_WORK_ITEMS as u64, 262_144, 7_500)
}

fn shooting_bounded_extent(command: &ShootingCommand, _snapshot: &ShootingSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    SHOOTING_BOUNDED_TOOL_IDS.contains(&command.command_id()).then_some(SHOOTING_BOUNDED_WORK_ITEMS)
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
fn shooting_bounded_reduce(
    command: &ShootingCommand,
    snapshot: &ShootingSnapshot,
    config: &ShootingConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<ShootingPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<ShootingMutation, ShootingConfigMutation, NoDraftMutation>, Fault> {
    if !SHOOTING_BOUNDED_TOOL_IDS.contains(&command.command_id()) {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("shooting.retained.route"), "the bounded Shooting reducer rejects resumable routes"));
    }
    let mut ctx = ShootingDispatchCtx::default();
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config, window: None }, &mut ctx)
}

struct ShootingCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl ShootingCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: SHOOTING_BOUNDED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for ShootingCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<ShootingPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<ShootingPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        SHOOTING_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        shooting_bounded_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > SHOOTING_BOUNDED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("bounded Shooting command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for ShootingCommandJobFactory {
    type Owner = EditorApp<ShootingPlayApp>;
    const TOOL_IDS: &'static [&'static str] = SHOOTING_BOUNDED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SHOOTING_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "importSnapshotJson", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setActiveShot", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setActiveAsset", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setShotCamera", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "saveCamera", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSunAzimuth", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSunElevation", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setSunIntensity", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setAmbientIntensity", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setMaterialRoughness", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setShadowEnabled", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "toggleSun", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setActiveShotLabel", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setActiveShotFormat", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setActiveShotShape", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchShots", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchAssets", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "addShot", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "addAsset", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "importAsset", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "resetFixture", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "loadSavedCamera", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setCameraDraftLabel", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setCenterModel", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setShotSelection", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "worldPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "worldPointerMove", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "saveDownload", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "loadRequest", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "importAssetRequest", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "exportActiveShot", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "exportAllShots", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
}
//#endregion 🧵️RetainedCommands

impl ArtifactEditor for ShootingPlayApp {
    type Snapshot = ShootingSnapshot;
    type Mutation = ShootingMutation;
    type Config = ShootingConfig;
    type ConfigMutation = ShootingConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = ShootingPresence;
    type PresenceMutation = ShootingPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = ShootingCommand;

    const DIALECT: Dialect = crate::SHOOTING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SHOOTING_DOCUMENT_SCHEMA;

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<ShootingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.shooting.shooting@1/*#editor",
        artifact_schema: "shooting.shooting",
        factory: "ShootingCommandJobFactory",
        factory_type: ShootingCommandJobFactory,
        tools: {
            "importSnapshotJson" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setActiveExample" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setActiveShot" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setActiveAsset" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setShotCamera" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "saveCamera" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setSunAzimuth" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setSunElevation" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setSunIntensity" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setAmbientIntensity" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setMaterialRoughness" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setShadowEnabled" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "toggleSun" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setActiveShotLabel" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setActiveShotFormat" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setActiveShotShape" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "patchShots" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "patchAssets" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "addShot" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "addAsset" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "importAsset" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "resetFixture" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "translateSelection" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "rotateSelection" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "scaleSelection" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setCamera" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "loadSavedCamera" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setCameraDraftLabel" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setCenterModel" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "setShotSelection" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "worldPointerDown" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "worldPointerMove" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "saveDownload" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "loadRequest" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "importAssetRequest" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "exportActiveShot" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "exportAllShots" => ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
        }
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(ShootingCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !SHOOTING_BOUNDED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("shooting.retained.tool-mismatch"), "Shooting command does not match its exact registered tool"));
        }
        if shooting_bounded_extent(&request.command, &request.snapshot, &request.interaction_state).is_none() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("shooting.retained.extent"), "Shooting bounded route exceeded its declared work extent"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(BoundedArtifactCommandWork::new(tool_id, shooting_bounded_reduce, shooting_bounded_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            ShootingCommand::command_id,
            SHOOTING_BOUNDED_RAW_BYTES,
            SHOOTING_BOUNDED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::shooting::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> ShootingSnapshot {
        crate::standards::v1::subsets::any::schema::default_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(shooting_io())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("shooting-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Config, Self::ConfigMutation>("shooting-config-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    /// 👥️ Shooting presence is a shot-id list plus one camera, so the default root is its exact empty terminal.
    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |_| true).expect("default shooting presence is the exact empty terminal")))
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    /// 🎞️ `photos:out` (see `shooting_photo_media`) plus the
    /// inherited `document:out` default (the pack of `doc.snapshot`, replicated inline — overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new
    /// one).
    fn export_media(port: &str, doc: &ArtifactView<'_, ShootingSnapshot>) -> Result<Media, MediaError> {
        match port {
            "photos:out" => shooting_photo_media(doc.snapshot),
            "artifact:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
                let bytes = store::ArtifactPack::encode_pack(doc.snapshot);
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🧬️ No `whole_document_operation` override — per `📓️taxonomy.md`, whole-document replace
    /// (the retired whole-document-replace variant) is banned outright with NO replacement mutation, so this falls back to the
    /// trait's own default (`None`); `import_media`'s `"artifact:in"` override below handles the
    /// real gesture via `reset_document_effect` instead.
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, ShootingSnapshot>) -> Result<Emit<ShootingMutation, ShootingConfigMutation, NoDraftMutation>, MediaError> {
        if port != "artifact:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
        };
        let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let snapshot = <ShootingSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        Ok(Emit { effects: vec![reset_document_effect(&snapshot)], ..Default::default() })
    }

    /// 🏷️ Maps each `ShootingCommand` variant back to the action id it was declared under in
    /// `create_shooting_app` — used by `VcsArtifactApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check.
    fn command_id(command: &ShootingCommand) -> &'static str {
        command.command_id()
    }

    fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<Self::Command, Fault> {
        args_bridge::command_from_action(action, args)
    }

    fn handle(
        command: &ShootingCommand,
        doc: &ArtifactView<'_, ShootingSnapshot>,
        cfg: &ConfigView<'_, ShootingConfig>,
        interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<ShootingMutation, ShootingConfigMutation, Self::DraftMutation>, Fault> {
        let mut ctx = ShootingDispatchCtx { selected_asset_ids: interaction.selection(SHOOTING_INTERACTION_DOMAIN).ids.clone() };
        command.dispatch(doc, cfg, &mut ctx)
    }

    /// 🧮️ This app's typed configuration spec — mirrors `ShootingConfig`'s three sticky-default fields,
    /// each grounded in an existing `.action_args` default (see that struct's doc).
    fn config_spec() -> semio_framework_plugin::ConfigSpec {
        semio_framework_plugin::ConfigSpec {
            fields: vec![
                semio_framework_plugin::ConfigFieldSpec {
                    key: "defaultShotFormat".into(),
                    label: "Default Shot Format".into(),
                    shape: semio_framework_plugin::ConfigFieldShape::Select { options: vec!["svg".into(), "png".into()] },
                    default: Some(DslValue::String("png".into())),
                },
                semio_framework_plugin::ConfigFieldSpec {
                    key: "defaultShotShape".into(),
                    label: "Default Shot Shape".into(),
                    shape: semio_framework_plugin::ConfigFieldShape::Select { options: vec!["rectangle".into(), "ellipse".into()] },
                    default: Some(DslValue::String("rectangle".into())),
                },
                semio_framework_plugin::ConfigFieldSpec {
                    key: "defaultAssetFormat".into(),
                    label: "Default Asset Format".into(),
                    shape: semio_framework_plugin::ConfigFieldShape::Select { options: vec!["glb".into()] },
                    default: Some(DslValue::String("glb".into())),
                },
            ],
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let snapshot = doc.snapshot;
        let labels = shooting_play_labels(view_state);
        match body_key {
            SHOOTING_PLAY_BODY_SCENE => scene_window::render(snapshot, cfg.snapshot, view_state.active_utility_id.as_deref().unwrap_or("move")),
            SHOOTING_PLAY_BODY_ICON => icon_window::render(snapshot, cfg.snapshot),
            SHOOTING_PLAY_BODY_ARTIFACT => document_panel::render(snapshot, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, SHOOTING_PLAY_BODY_ARTIFACT)),
            SHOOTING_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels, &semio_framework_plugin::TreeWindows::for_body(view_state, SHOOTING_PLAY_BODY_CATALOGUE)),
            SHOOTING_PLAY_BODY_INSPECTION => inspection_panel::render(snapshot, cfg.snapshot, labels),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| ui_capacity_error()),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }

    fn window_engagements(doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let labels = shooting_play_labels(view_state);
        HashMap::from([(SHOOTING_PLAY_WINDOW_SCENE.into(), scene_window::engagement(doc.snapshot, cfg.snapshot, labels)), (SHOOTING_PLAY_WINDOW_ICON.into(), icon_window::engagement(doc.snapshot, labels))])
    }

    fn window_measures(doc: &ArtifactView<'_, ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = shooting_play_labels(view_state);
        HashMap::from([(SHOOTING_PLAY_WINDOW_SCENE.into(), scene_window::window_measures(doc.snapshot, labels)), (SHOOTING_PLAY_WINDOW_ICON.into(), icon_window::window_measures(doc.snapshot, labels))])
    }
}
//#endregion 🔖️ShootingPlayApp

//#region 🔖️ResetDocument
/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `scene` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (file import,
/// load-example, dev document load). Per `📓️taxonomy.md`, whole-document replace is banned outright with NO
/// replacement mutation: whole-document replace is not expressible as an in-history `Mutation` at
/// all. Every former "replace the whole document" gesture in this package (`import_media`'s
/// `"artifact:in"` above, `commands::document::{import_snapshot_json,set_active_example,reset_snapshot}`)
/// builds this effect instead of an `Emit::mutations([...])`. The spr is a fresh, edit-free op-log
/// for `scene` — `store::empty_document_spr`, never a minted `create_document_envelope` (an envelope
/// dropped without its bounded retirement authority traps the guest on Drop).
pub fn reset_document_effect(scene: &ShootingSnapshot) -> semio_framework_plugin::Effect {
    let pack = <ShootingSnapshot as store::ArtifactPack>::encode_pack(scene);
    let spr = semio_framework_plugin::resolve_ready(store::empty_document_spr("shooting", SHOOTING_DOCUMENT_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_shooting_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::SHOOTING_DIALECT)
            .document(["semio", "shooting"])
            .artifact_kind(crate::artifact_kind())
            // 🖼️ `2d.image` — the interchange kind `photos:out` produces (WORKFLOWS-END-TO-END-TYPED-PORTS
            // Wave 2 port recipe); a sibling agent may declare the identical shape on the raster app too
            // — identical-shape duplicates are harmless (registry dedupes by id).
            .artifact_kind(semio_framework_plugin::ArtifactKindSpec {
                id: "2d.image".into(),
                name: "2D Image".into(),
                source_format: "2d.image".into(),
                component_kind: "image".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
                schema: "2d.image".into(),
                export_formats: vec![],
                import_formats: vec![],
                    export_stdio_kinds: vec!["stdio.png".into()],
        import_stdio_kinds: vec!["stdio.png".into()],
    })
            .media_output(shooting_photos_out_port())
            .icon_id("camera")
            .mode_def(edit::definition())
            .default_mode_id(edit::SHOOTING_PLAY_MODE_EDIT)
            .window_kind_def(scene_window::definition())
            .window_kind_def(icon_window::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
            // 🛠️ Dev-only whole-document import — kept out of the command palette.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importSnapshotJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"), ActionKind::Mutation) })
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .mutation("setActiveShot", LocalizedLabel::native("Set Active Shot", "Aktive Aufnahme festlegen"))
            .mutation("setActiveAsset", LocalizedLabel::native("Set Active Asset", "Aktives Objekt festlegen"))
            .action_with(ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera"))
            .mutation("setShotCamera", LocalizedLabel::native("Set Shot Camera", "Aufnahmekamera festlegen"))
            .mutation("saveCamera", LocalizedLabel::native("Save Camera", "Kamera speichern"))
            .view_action("loadSavedCamera", LocalizedLabel::native("Load Saved Camera", "Gespeicherte Kamera laden"))
            .action_with(ActionDefinition::new("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), ActionKind::Mutation, "sun"))
            .action_with(ActionDefinition::new("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), ActionKind::Mutation, "sun"))
            .action_with(ActionDefinition::new("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), ActionKind::Mutation, "sun"))
            .mutation("setAmbientIntensity", LocalizedLabel::native("Set Ambient Intensity", "Umgebungslichtintensität festlegen"))
            .mutation("setMaterialRoughness", LocalizedLabel::native("Set Material Roughness", "Materialrauheit festlegen"))
            .mutation("setShadowEnabled", LocalizedLabel::native("Set Shadow Enabled", "Schatten aktivieren"))
            .action_with(ActionDefinition::new("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), ActionKind::Mutation, "sun"))
            .mutation("setActiveShotLabel", LocalizedLabel::native("Set Active Shot Label", "Bezeichnung der aktiven Aufnahme festlegen"))
            .mutation("setActiveShotFormat", LocalizedLabel::native("Set Active Shot Format", "Format der aktiven Aufnahme festlegen"))
            .mutation("setActiveShotShape", LocalizedLabel::native("Set Active Shot Shape", "Form der aktiven Aufnahme festlegen"))
            .mutation("patchShots", LocalizedLabel::native("Patch Shots", "Aufnahmen aktualisieren"))
            .mutation("patchAssets", LocalizedLabel::native("Patch Assets", "Objekte aktualisieren"))
            .mutation("addShot", LocalizedLabel::native("Add Shot", "Aufnahme hinzufügen"))
            .mutation("addAsset", LocalizedLabel::native("Add Asset", "Objekt hinzufügen"))
            .mutation("importAsset", LocalizedLabel::native("Import Asset", "Objekt importieren"))
            .mutation("resetFixture", LocalizedLabel::native("Reset Fixture", "Vorgabe zurücksetzen"))
            .mutation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .mutation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .mutation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            // 👁️ Ephemeral view state — shot gallery selection, camera draft label, transform utility.
            .view_action("setShotSelection", LocalizedLabel::native("Set Shot Selection", "Aufnahmeauswahl festlegen"))
            .view_action("setCameraDraftLabel", LocalizedLabel::native("Set Camera Draft Label", "Kamera-Entwurfsbezeichnung festlegen"))
            .view_action("setCenterModel", LocalizedLabel::native("Set Center Model", "Modellzentrierung festlegen"))
            .action_with(ActionDefinition::new("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"), ActionKind::View, "mouse-pointer"))
            .view_action("worldPointerMove", LocalizedLabel::native("World Pointer Move", "Welt-Zeiger bewegt"))
            // 🕹️ The framework-owned "assets" interaction domain (ticket
            // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — the 3d scene's asset pick/marquee
            // selection and pointer hover; auto-injects interactionSelect/interactionHover/
            // clearSelection/selectAll/setSelectionMode/setInteractionGranularity, replacing the deleted
            // bespoke setSelection/setSelectionMethod/worldSelect/setHover/worldPick actions above.
            .interaction(InteractionDefinition {
                id: SHOOTING_INTERACTION_DOMAIN.into(),
                label: LocalizedLabel::native("Assets", "Objekte"),
                granularities: vec![GranularityDefinition { id: "asset".into(), label: LocalizedLabel::native("Asset", "Objekt"), icon_id: "box".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(SHOOTING_PLAY_WINDOW_SCENE, vec![InteractionRef::new(SHOOTING_INTERACTION_DOMAIN)])
            // 🐚️ Shell effects — export/import round-trips through the host.
            .shell_action("saveDownload", LocalizedLabel::native("Save Download", "Download speichern"))
            .shell_action("loadRequest", LocalizedLabel::native("Load Request", "Ladeanfrage"))
            .shell_action("importAssetRequest", LocalizedLabel::native("Import Asset Request", "Objekt-Importanfrage"))
            .shell_action("exportActiveShot", LocalizedLabel::native("Export Active Shot", "Aktive Aufnahme exportieren"))
            .shell_action("exportAllShots", LocalizedLabel::native("Export All Shots", "Alle Aufnahmen exportieren"))
            // 🧵️ Every verb is a bounded first-step tool with an exact publication contract
            // (`ShootingCommandJobFactory::PUBLICATION_CONTRACTS`) — UI dispatch refuses anything else.
            .action_interactive_job("importSnapshotJson", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveShot", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveAsset", InteractiveJobClassification::Migrated)
            .action_interactive_job("setShotCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("saveCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunAzimuth", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunElevation", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunIntensity", InteractiveJobClassification::Migrated)
            .action_interactive_job("setAmbientIntensity", InteractiveJobClassification::Migrated)
            .action_interactive_job("setMaterialRoughness", InteractiveJobClassification::Migrated)
            .action_interactive_job("setShadowEnabled", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSun", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveShotLabel", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveShotFormat", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveShotShape", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchShots", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchAssets", InteractiveJobClassification::Migrated)
            .action_interactive_job("addShot", InteractiveJobClassification::Migrated)
            .action_interactive_job("addAsset", InteractiveJobClassification::Migrated)
            .action_interactive_job("importAsset", InteractiveJobClassification::Migrated)
            .action_interactive_job("resetFixture", InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("loadSavedCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCameraDraftLabel", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCenterModel", InteractiveJobClassification::Migrated)
            .action_interactive_job("setShotSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("worldPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("worldPointerMove", InteractiveJobClassification::Migrated)
            .action_interactive_job("saveDownload", InteractiveJobClassification::Migrated)
            .action_interactive_job("loadRequest", InteractiveJobClassification::Migrated)
            .action_interactive_job("importAssetRequest", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportActiveShot", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportAllShots", InteractiveJobClassification::Migrated)
            // 📝️ Staged argument forms for the panel-visible create actions (defaults materialized host-side).
            .action_args("addShot", vec![
                ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), vec![ActionArgOption::new("svg", LocalizedLabel::native("SVG", "SVG")), ActionArgOption::new("png", LocalizedLabel::native("PNG", "PNG"))]).default_value(&"png"),
                ActionArgDef::select("shape", LocalizedLabel::native("Shape", "Form"), vec![ActionArgOption::new("rectangle", LocalizedLabel::native("Rectangle", "Rechteck")), ActionArgOption::new("ellipse", LocalizedLabel::native("Ellipse", "Ellipse"))]).default_value(&"rectangle"),
            ])
            .action_args("addAsset", vec![
                ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), vec![ActionArgOption::new("glb", LocalizedLabel::native("GLB", "GLB"))]).default_value(&"glb"),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new(SHOOTING_EXAMPLE_DEFAULT_ID, LocalizedLabel::native("Default Base Icon", "Standard-Basissymbol")),
                    ActionArgOption::new(
                        SHOOTING_EXAMPLE_HEXAGONAL_CUT_CONCRETE_FOREST_LEFT,
                        LocalizedLabel::native("Hexagonal Cut Concrete Forest Left", "Sechseckig geschnittener Betonwald links"),
                    ),
                ]).required(),
            ])
            // 🧰️ Transform gumball — an exclusive utility group scoped to the scene window (active utility is host-owned).
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("move", LocalizedLabel::native("Move", "Verschieben"), "move") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("rotate", LocalizedLabel::native("Rotate", "Drehen"), "rotate-cw") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("scale", LocalizedLabel::native("Scale", "Skalieren"), "maximize-2") })
            .window_kind_utilities(SHOOTING_PLAY_WINDOW_SCENE, vec!["move".into(), "rotate".into(), "scale".into()])
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS Wave 1) —
            // `config_spec()`/`shooting_io()` are this same information's single source of truth, reused
            // here rather than duplicated (`command_grammar` stays `CommandGrammar::empty()`: this app's
            // typed commands are dispatched via `ShootingCommand`'s `OpBinary` codec directly, not a
            // keyword-parsed text grammar).
            .config(ShootingPlayApp::config_spec())
            .io(shooting_io())
            // 🚧️ SDK GAP (contract §2.4): `Editor::builder(...).build_definition()` returns a bare
            // `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example(...)`/`.workflow(...)` on this builder, so the old app-level
            // `SHOOTING_EXAMPLE_DEFAULT_ID` example registration and the no-op `.workflow("shooting",
            // …)` call are dropped here, not silently: reported in this packet's migration notes. The
            // subset's own `📚️examples/🎬️demo` facet is the modern, role-agnostic replacement surface.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
//#endregion 🧪️UnitTests


#[cfg(test)]
#[path = "🧪️tests/🔬️window-action-contract/🦀️.rs"]
mod window_action_contract;
