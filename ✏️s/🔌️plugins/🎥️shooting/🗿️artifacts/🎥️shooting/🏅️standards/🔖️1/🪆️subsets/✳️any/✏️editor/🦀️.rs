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
use crate::editor::shooting::ui_helpers_and_io::*;
use crate::standards::v1::subsets::any::schema::mutations::ShootingMutation;
use crate::{ShootingSnapshot, SHOOTING_DOCUMENT_SCHEMA};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::DslValue;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultCode;
use semio_framework_plugin::FaultOrigin;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::OsMediaCapability;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use std::collections::HashMap;
use semio_framework_2d::compute::EngineHandles;

//#region 🔖️Constants
pub use crate::editor::shooting::commands::document::set_active_example::{SHOOTING_EXAMPLE_DEFAULT_ID, SHOOTING_EXAMPLE_HEXAGONAL_CUT_CONCRETE_FOREST_LEFT};
pub use catalogue_panel::SHOOTING_PLAY_BODY_CATALOGUE;
pub use document_panel::SHOOTING_PLAY_BODY_ARTIFACT;
pub use icon_window::SHOOTING_PLAY_BODY_ICON;
pub use icon_window::SHOOTING_PLAY_WINDOW_ICON;
pub use inspection_panel::SHOOTING_PLAY_BODY_INSPECTION;
pub use scene_window::SHOOTING_PLAY_BODY_SCENE;
pub use scene_window::SHOOTING_PLAY_WINDOW_SCENE;
//#endregion 🔖️Constants

//#region 🔖️Commands

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
        "resetSnapshot" as "reset-snapshot" => reset_snapshot::ResetSnapshot,
        "translateSelection" as "translate-selection" => translate_selection::TranslateSelection,
        "rotateSelection" as "rotate-selection" => rotate_selection::RotateSelection,
        "scaleSelection" as "scale-selection" => scale_selection::ScaleSelection,
        "setCamera" as "camera" => set_camera::SetCamera,
        "loadSavedCamera" as "load-saved-camera" => load_saved_camera::LoadSavedCamera,
        "setCenterModel" as "center-model" => set_center_model::SetCenterModel,
        "setShotSelection" as "set-shot-selection" => set_shot_selection::SetShotSelection,
        "worldPointerDown" as "world-pointer-down" => world_pointer_down::WorldPointerDown,
        "worldPointerMove" as "world-pointer-move" => world_pointer_move::WorldPointerMove,
        "saveDownload" as "save-download" => save_download::SaveDownload,
        "importAssetRequest" as "import-asset-request" => import_asset_request::ImportAssetRequest,
        "exportActiveShot" as "export-active-shot" => export_active_shot::ExportActiveShot,
        "exportAllShots" as "export-all-shots" => export_all_shots::ExportAllShots,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::editor::shooting::commands::asset::{add_asset, import_asset, import_asset_request, patch_assets, set_active_asset};
use crate::editor::shooting::commands::camera::{load_saved_camera, save_camera, set_camera, set_shot_camera};
use crate::editor::shooting::commands::export::{export_active_shot, export_all_shots};
use crate::editor::shooting::commands::document::{import_snapshot_json, reset_snapshot, save_download, set_active_example};
use crate::editor::shooting::commands::gumball::{rotate_selection, scale_selection, translate_selection};
use crate::editor::shooting::commands::scene::{set_ambient_intensity, set_material_roughness, set_shadow_enabled, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use crate::editor::shooting::commands::selection::{set_center_model, set_shot_selection, world_pointer_down, world_pointer_move};
use crate::editor::shooting::commands::shot::{add_shot, patch_shots, set_active_shot, set_active_shot_format, set_active_shot_label, set_active_shot_shape};
//#endregion 🔖️Commands

//#region 🔖️ActionBridge

/// 🎯️ Host-action bridge into the closed `ShootingCommand` enum (ticket
/// 26/09/16/SHOOTING-PLUGIN-END-TO-END). The React/wgpu shells still speak `{action, args}` with
/// camelCase argument keys and the host's control contracts (`value` for sliders/selects/inputs,
/// `pressed` for toggles, `{mode, ids, dx…}` for the gumball, `{windowId, camera}` for the viewport);
/// every `🎮️commands/*` payload derives `FromValue` over its own snake_case field names, so this
/// boundary folds the keys, applies the per-verb aliases and decodes — the default trait impl refuses
/// every app action outright, which left every panel/measure/viewport gesture dead in the shell.
#[path = "🔀️args-bridge/🦀️.rs"]
mod args_bridge;

//#endregion 🔖️ActionBridge

#[path = "🕹️play-app/🦀️.rs"]
mod play_app;
pub use play_app::*;

#[path = "🧵️retained-commands/🦀️.rs"]
mod retained_commands;
pub use retained_commands::*;


#[path = "📋️manifest/🦀️.rs"]
mod manifest;
pub use manifest::*;

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

//#region 🪢️TaxonomyMounts


//#endregion 🪢️TaxonomyMounts

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
use crate::{SHOOTING_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::shooting::create_shooting_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.shooting".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("2D Shooting", "2D-Aufnahme"),
        source_format: "shooting.scene".into(),
        component_kind: "shooting".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        schema: SHOOTING_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into()],
        import_stdio_kinds: vec!["stdio.json".into()],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems3
