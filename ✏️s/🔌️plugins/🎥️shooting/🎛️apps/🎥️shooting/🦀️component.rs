//! 🖥️ Shooting play app — the `ArtifactApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, chrome measures in those windows' `🎚️options/*`, panel trees in `📌️panels/*`,
//! labels in `🦀️terminology.rs`, view state in `🦀️config.rs`, shared compute in the artifact's `⚙️engine`.
//! This file is a routing table: `handle` → `ShootingCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

use crate::apps::shooting::commands::{asset, camera, export, fixture, gumball, locale, scene, selection, shot};
use crate::apps::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::apps::shooting::presence::{ShootingPresence, ShootingPresenceMutation};
use crate::apps::shooting::modes::edit;
use crate::apps::shooting::modes::edit::windows::icon as icon_window;
use crate::apps::shooting::modes::edit::windows::scene as scene_window;
use crate::apps::shooting::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::apps::shooting::terminology::shooting_play_labels;
use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::{ShootingSnapshot, SHOOTING_DOCUMENT_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView,
    tree_item_with_action, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, AppIo, ConfigView, ArtifactApp, ArtifactView, DslValue, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm,
    MediaPayload, MediaType, OsMediaCapability, UiNode, UiTreeItemNode, UtilityDefinition, WindowEngagement, WindowMeasure,
};
use store::EngineHandles;
use std::collections::HashMap;

//#region 🔖️Constants
pub const SHOOTING_PLAY_APP_ID: &str = "shooting-play";
const SHOOTING_PLAY_CONTROLLER_ID: &str = "shooting-play";
pub use icon_window::SHOOTING_PLAY_BODY_ICON;
pub use icon_window::SHOOTING_PLAY_WINDOW_ICON;
pub use scene_window::SHOOTING_PLAY_BODY_SCENE;
pub use scene_window::SHOOTING_PLAY_WINDOW_SCENE;
pub use catalogue_panel::SHOOTING_PLAY_BODY_CATALOGUE;
pub use document_panel::SHOOTING_PLAY_BODY_DOCUMENT;
pub use inspection_panel::SHOOTING_PLAY_BODY_INSPECTION;
pub use crate::apps::shooting::commands::fixture::set_active_example::SHOOTING_EXAMPLE_DEFAULT_ID;
//#endregion 🔖️Constants

//#region 🔖️Utilities
/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`🎚️options/*`, `📌️panels/*`) builds its `on_change`/item actions with.
pub fn shooting_action(action: &str, args: Option<serde_json::Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(SHOOTING_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🌳️ Layers an `icon_id` onto the SDK's `tree_item_with_action` skeleton — the SDK primitive's third
/// parameter is `description`, not an icon, so the shooting-specific icon assignment stays local. Shared
/// by the document and catalogue panels (two consumers).
pub fn tree_item_with_icon(id: impl Into<String>, label: impl Into<Label>, icon_id: &str, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode { icon_id: Some(icon_id.into()), menu: None, ..tree_item_with_action(id, label, None, action) }
}
//#endregion 🔖️Utilities

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `crate::artifacts::shooting::artifact_kind` already declares (schema/media type/presentation fields
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
/// them drops no live behavior. `crate::artifacts::shooting::artifact_kind()`'s `export_stdio_kinds`/
/// `import_stdio_kinds` remain the live source of truth for this artifact's real format list.
pub fn shooting_io() -> AppIo {
    AppIo {
        document_schema: "shooting.scene".into(),
        document_media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
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
/// port — reuses the same SVG-then-rasterize pipeline (`crate::artifacts::shooting::schema::shooting_scene_svg` +
/// `rasterize_svg_to_png_base64`) as the `exportActiveShot`/PNG shell action, so there is exactly one
/// photo renderer.
pub fn shooting_photo_media(snapshot: &ShootingSnapshot) -> Result<Media, MediaError> {
    let (svg, width, height) = crate::artifacts::shooting::schema::shooting_scene_svg(snapshot).map_err(|error| MediaError::Payload("photos:out".into(), error))?;
    let png_base64 = semio_framework_os::rasterize_svg_to_png_base64(&svg, width, height).map_err(|error| MediaError::Payload("photos:out".into(), error))?;
    Ok(Media {
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        payload: MediaPayload::Structured { schema: "2d.image".into(), json: png_base64 },
    })
}
//#endregion 🔖️Io

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `ShootingPlayApp::Command` — the SOLE dispatch surface for shooting's own behavior, assembled
    /// from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the
    /// kebab-case `#[dsl(key = ..)]` the binary/text codec uses) — different vocabularies, and
    /// `setLocale`/`locale` is the row that proves it. **Row order is the binary variant ordinal:
    /// appending is safe, reordering is a wire-format break.**
    pub enum ShootingCommand for ShootingSnapshot, ShootingMutation, ShootingConfig, ShootingConfigMutation {
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
        "setActiveUtility" as "active-utility" => set_active_utility::SetActiveUtility,
        "setLocale" as "locale" => set_locale::SetLocale,
        "setSelection" as "set-selection" => set_selection::SetSelection,
        "setSelectionMethod" as "selection-method" => set_selection_method::SetSelectionMethod,
        "worldSelect" as "world-select" => world_select::WorldSelect,
        "setHover" as "set-hover" => set_hover::SetHover,
        "worldPick" as "world-pick" => world_pick::WorldPick,
        "worldPointerDown" as "world-pointer-down" => world_pointer_down::WorldPointerDown,
        "worldPointerMove" as "world-pointer-move" => world_pointer_move::WorldPointerMove,
        "saveDownload" as "save-download" => save_download::SaveDownload,
        "loadRequest" as "load-request" => load_request::LoadRequest,
        "importAssetRequest" as "import-asset-request" => import_asset_request::ImportAssetRequest,
        // 🎯️ command_id() is overridden below (payload-dependent: exportActiveShot/exportAllShots) — the
        // row literal here is never actually consulted, see `ShootingPlayApp::command_id`.
        "exportActiveShot" as "export-shots" => export_shots::ExportShots,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use asset::{add_asset, import_asset, import_asset_request, patch_assets, set_active_asset};
use camera::{load_saved_camera, save_camera, set_camera, set_camera_draft_label, set_shot_camera};
use export::export_shots;
use fixture::{load_request, reset_snapshot, save_download, set_active_example, import_snapshot_json};
use gumball::{rotate_selection, scale_selection, translate_selection};
use locale::set_locale;
use scene::{set_ambient_intensity, set_material_roughness, set_shadow_enabled, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use selection::{set_active_utility, set_center_model, set_hover, set_selection, set_selection_method, world_pick, world_pointer_down, world_pointer_move, world_select};
use shot::{add_shot, patch_shots, set_active_shot, set_active_shot_format, set_active_shot_label, set_active_shot_shape};
//#endregion 🔖️Commands

//#region 🔖️ShootingPlayApp
/// 🧪️ B1: unit struct — every former runtime field now lives in `ShootingConfig`, written through
/// `ShootingConfigMutation`s.
#[derive(Default)]
pub struct ShootingPlayApp;

impl ArtifactApp for ShootingPlayApp {
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

    const APP_ID: &'static str = SHOOTING_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = SHOOTING_DOCUMENT_SCHEMA;

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::apps::shooting::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> ShootingSnapshot {
        crate::artifacts::shooting::schema::default_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(shooting_io())
    }

    /// 🎞️ `photos:out` (see `shooting_photo_media`) plus the
    /// inherited `document:out` default (the pack of `doc.snapshot`, replicated inline — overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new
    /// one).
    fn export_media(port: &str, doc: &ArtifactView<'_, ShootingSnapshot>) -> Result<Media, MediaError> {
        match port {
            "photos:out" => shooting_photo_media(doc.snapshot),
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = store::ArtifactPack::encode_pack(doc.snapshot);
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🧬️ No `whole_document_operation` override — per `📓️taxonomy.md`, whole-document replace
    /// (the retired whole-document-replace variant) is banned outright with NO replacement mutation, so this falls back to the
    /// trait's own default (`None`); `import_media`'s `"document:in"` override below handles the
    /// real gesture via `reset_document_effect` instead.
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, ShootingSnapshot>) -> Result<Emit<ShootingMutation, ShootingConfigMutation, NoDraftMutation>, MediaError> {
        if port != "document:in" {
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
    /// View/Shell kind-discipline check. Every row delegates to the macro-generated `command_id()`
    /// EXCEPT `ExportShots`, whose real manifest id is payload-dependent (`exportActiveShot` when
    /// `all == false`, `exportAllShots` when `all == true`) — `app_commands!`'s generated method is a
    /// static 1:1 row→literal mapping with no per-payload escape hatch, so this is the one case that
    /// needs a manual override.
    fn command_id(command: &ShootingCommand) -> &'static str {
        match command {
            ShootingCommand::ExportShots(export_shots::ExportShots { all }) => {
                if *all {
                    "exportAllShots"
                } else {
                    "exportActiveShot"
                }
            }
            other => other.command_id(),
        }
    }

    fn handle(command: &ShootingCommand, doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<ShootingMutation, ShootingConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
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

    fn render(body_key: &str, doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>) -> UiNode {
        let snapshot = doc.snapshot;
        let labels = shooting_play_labels(cfg.snapshot);
        match body_key {
            SHOOTING_PLAY_BODY_SCENE => scene_window::render(snapshot, cfg.snapshot),
            SHOOTING_PLAY_BODY_ICON => icon_window::render(snapshot, cfg.snapshot),
            SHOOTING_PLAY_BODY_DOCUMENT => document_panel::render(snapshot, labels),
            SHOOTING_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
            SHOOTING_PLAY_BODY_INSPECTION => inspection_panel::render(snapshot, cfg.snapshot, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_engagements(doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>) -> HashMap<String, WindowEngagement> {
        let labels = shooting_play_labels(cfg.snapshot);
        HashMap::from([(SHOOTING_PLAY_WINDOW_SCENE.into(), scene_window::engagement(doc.snapshot, cfg.snapshot, labels)), (SHOOTING_PLAY_WINDOW_ICON.into(), icon_window::engagement(doc.snapshot, labels))])
    }

    fn window_measures(doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = shooting_play_labels(cfg.snapshot);
        HashMap::from([(SHOOTING_PLAY_WINDOW_SCENE.into(), scene_window::window_measures(doc.snapshot, labels)), (SHOOTING_PLAY_WINDOW_ICON.into(), icon_window::window_measures(doc.snapshot, labels))])
    }
}
//#endregion 🔖️ShootingPlayApp

//#region 🔖️ResetDocument
/// 🌱️ Builds a `HostEffect::LoadDocument` that swaps the live document to `scene` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (file import,
/// load-example, dev fixture load). Per `📓️taxonomy.md`, whole-document replace is banned outright with NO
/// replacement mutation: whole-document replace is not expressible as an in-history `Mutation` at
/// all. Every former "replace the whole document" gesture in this package (`import_media`'s
/// `"document:in"` above, `commands::fixture::{import_snapshot_json,set_active_example,reset_snapshot}`)
/// builds this effect instead of an `Emit::mutations([...])`. The spr is a fresh, edit-free op-log
/// for `scene` — a genesis envelope with no history to encode.
pub fn reset_document_effect(scene: &ShootingSnapshot) -> semio_framework_plugin::HostEffect {
    let pack = <ShootingSnapshot as store::ArtifactPack>::encode_pack(scene);
    let envelope = store::create_document_envelope::<ShootingSnapshot, ShootingMutation>(SHOOTING_DOCUMENT_SCHEMA, "shooting", scene.clone(), None);
    let spr = store::print_document_spr(&envelope).expect("shooting document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework_plugin::HostEffect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_shooting_app() -> App {
    App::from_builder(
        App::builder(SHOOTING_PLAY_APP_ID, LocalizedLabel::native("Shooting", "Shooting"))
            .document(["semio", "shooting"])
            .artifact_kind(crate::artifacts::shooting::artifact_kind())
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
                    export_stdio_kinds: vec!["stdio.png"],
        import_stdio_kinds: vec!["stdio.png"],
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
            // 🛠️ Dev-only whole-fixture import — kept out of the command palette.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("importSnapshotJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"), ActionKind::Mutation) })
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .mutation("setActiveShot", LocalizedLabel::native("Set Active Shot", "Aktive Aufnahme festlegen"))
            .mutation("setActiveAsset", LocalizedLabel::native("Set Active Asset", "Aktives Objekt festlegen"))
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .mutation("setShotCamera", LocalizedLabel::native("Set Shot Camera", "Aufnahmekamera festlegen"))
            .mutation("saveCamera", LocalizedLabel::native("Save Camera", "Kamera speichern"))
            .view_action("loadSavedCamera", LocalizedLabel::native("Load Saved Camera", "Gespeicherte Kamera laden"))
            .mutation("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"))
            .mutation("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"))
            .mutation("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"))
            .mutation("setAmbientIntensity", LocalizedLabel::native("Set Ambient Intensity", "Umgebungslichtintensität festlegen"))
            .mutation("setMaterialRoughness", LocalizedLabel::native("Set Material Roughness", "Materialrauheit festlegen"))
            .mutation("setShadowEnabled", LocalizedLabel::native("Set Shadow Enabled", "Schatten aktivieren"))
            .mutation("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"))
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
            // 👁️ Ephemeral view state — selection, camera draft label, world picking.
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("setCameraDraftLabel", LocalizedLabel::native("Set Camera Draft Label", "Kamera-Entwurfsbezeichnung festlegen"))
            .view_action("setCenterModel", LocalizedLabel::native("Set Center Model", "Modellzentrierung festlegen"))
            .view_action("worldSelect", LocalizedLabel::native("World Select", "Welt auswählen"))
            .view_action("setHover", LocalizedLabel::native("Set Hover", "Überfahren festlegen"))
            .view_action("worldPick", LocalizedLabel::native("World Pick", "Welt-Auswahl (Pick)"))
            .view_action("setSelectionMethod", LocalizedLabel::native("Set Selection Method", "Auswahlmethode festlegen"))
            .view_action("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"))
            .view_action("worldPointerMove", LocalizedLabel::native("World Pointer Move", "Welt-Zeiger bewegt"))
            // 🐚️ Shell effects — export/import round-trips through the host.
            .shell_action("saveDownload", LocalizedLabel::native("Save Download", "Download speichern"))
            .shell_action("loadRequest", LocalizedLabel::native("Load Request", "Ladeanfrage"))
            .shell_action("importAssetRequest", LocalizedLabel::native("Import Asset Request", "Objekt-Importanfrage"))
            .shell_action("exportActiveShot", LocalizedLabel::native("Export Active Shot", "Aktive Aufnahme exportieren"))
            .shell_action("exportAllShots", LocalizedLabel::native("Export All Shots", "Alle Aufnahmen exportieren"))
            // 📝️ Staged argument forms for the panel-visible create actions (defaults materialized host-side).
            .action_args("addShot", vec![
                ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), vec![ActionArgOption::new("svg", LocalizedLabel::native("SVG", "SVG")), ActionArgOption::new("png", LocalizedLabel::native("PNG", "PNG"))]).default_value("png"),
                ActionArgDef::select("shape", LocalizedLabel::native("Shape", "Form"), vec![ActionArgOption::new("rectangle", LocalizedLabel::native("Rectangle", "Rechteck")), ActionArgOption::new("ellipse", LocalizedLabel::native("Ellipse", "Ellipse"))]).default_value("rectangle"),
            ])
            .action_args("addAsset", vec![
                ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), vec![ActionArgOption::new("glb", LocalizedLabel::native("GLB", "GLB"))]).default_value("glb"),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new(SHOOTING_EXAMPLE_DEFAULT_ID, LocalizedLabel::native("Default Base Icon", "Standard-Basissymbol")),
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
            .io(shooting_io()),
    )
    .example(SHOOTING_EXAMPLE_DEFAULT_ID, LocalizedLabel::native("Default Base Icon", "Standard-Basissymbol"), crate::artifacts::shooting::schema::default_snapshot_json(), "camera")
    .workflow("shooting", "Shooting", "icon")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsArtifactApp, ViewModel, WindowMeasure};

    pub type ShootingApp = VcsArtifactApp<ShootingPlayApp>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn shooting_app() -> ShootingApp {
        new_app::<ShootingPlayApp>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn shooting_app_with_registry() -> ShootingApp {
        new_app_with_registry::<ShootingPlayApp>(create_shooting_app)
    }

    pub fn dispatch(app: &mut ShootingApp, command: ShootingCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut ShootingApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    pub fn scene_window_measures(app: &mut ShootingApp) -> Vec<WindowMeasure> {
        app.window_measures().get(SHOOTING_PLAY_WINDOW_SCENE).cloned().expect("scene window measures")
    }

    pub fn icon_window_measures(app: &mut ShootingApp) -> Vec<WindowMeasure> {
        app.window_measures().get(SHOOTING_PLAY_WINDOW_ICON).cloned().expect("icon window measures")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::shooting::testkit::{dispatch, shooting_app, shooting_app_with_registry, ShootingApp};
    use semio_framework_plugin::testkit;
    use semio_framework_plugin::{ActionKind, HostEffect, PluginApp, ViewModel};
    use serde_json::{json, Value};

    fn default_camera(position: [f64; 3]) -> crate::artifacts::shooting::ShootingCamera {
        crate::artifacts::shooting::ShootingCamera { position, target: [0.0, 0.0, 0.0], zoom: 1.0, fov: 50.0, up: None, projection: None }
    }

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
    /// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to
    /// hold.
    #[test]
    fn command_ids_are_unique_across_every_row() {
        let app = ShootingPlayApp;
        let ids: Vec<&str> = every_command().iter().map(|command| ShootingPlayApp::command_id(command)).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 42, "every ShootingCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) fn every_command() -> Vec<ShootingCommand> {
        vec![
            ShootingCommand::ImportSnapshotJson(import_snapshot_json::ImportSnapshotJson { json: "{\"schema\":\"shooting.shooting\"}".into() }),
            ShootingCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "base-icon".into() }),
            ShootingCommand::SetActiveShot(set_active_shot::SetActiveShot { shot_id: Some("s1".into()) }),
            ShootingCommand::SetActiveAsset(set_active_asset::SetActiveAsset { asset_id: Some("a1".into()) }),
            ShootingCommand::SetShotCamera(set_shot_camera::SetShotCamera { shot_id: "s1".into(), camera: default_camera([1.0, 2.0, 3.0]) }),
            ShootingCommand::SaveCamera(save_camera::SaveCamera {}),
            ShootingCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 45.0 }),
            ShootingCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 35.0 }),
            ShootingCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 2.4 }),
            ShootingCommand::SetAmbientIntensity(set_ambient_intensity::SetAmbientIntensity { value: 1.15 }),
            ShootingCommand::SetMaterialRoughness(set_material_roughness::SetMaterialRoughness { value: 0.5 }),
            ShootingCommand::SetShadowEnabled(set_shadow_enabled::SetShadowEnabled { value: true }),
            ShootingCommand::ToggleSun(toggle_sun::ToggleSun { value: false }),
            ShootingCommand::SetActiveShotLabel(set_active_shot_label::SetActiveShotLabel { value: "Overview".into() }),
            ShootingCommand::SetActiveShotFormat(set_active_shot_format::SetActiveShotFormat { value: "png".into() }),
            ShootingCommand::SetActiveShotShape(set_active_shot_shape::SetActiveShotShape { value: "ellipse".into() }),
            ShootingCommand::PatchShots(patch_shots::PatchShots { shot_ids: vec!["s1".into(), "s2".into()], field: "label".into(), value: "Hero".into() }),
            ShootingCommand::PatchAssets(patch_assets::PatchAssets { asset_ids: vec!["a1".into()], field: "name".into(), value: "Renamed".into() }),
            ShootingCommand::AddShot(add_shot::AddShot { format: "svg".into(), shape: "rectangle".into() }),
            ShootingCommand::AddAsset(add_asset::AddAsset { format: "glb".into() }),
            ShootingCommand::ImportAsset(import_asset::ImportAsset { payload: "data:model/gltf-binary;base64,AAA=".into(), name: Some("Imported".into()) }),
            ShootingCommand::ResetSnapshot(reset_snapshot::ResetSnapshot {}),
            ShootingCommand::TranslateSelection(translate_selection::TranslateSelection { asset_ids: vec!["a1".into(), "a2".into()], dx: 1.0, dy: -2.0, dz: 3.5 }),
            ShootingCommand::RotateSelection(rotate_selection::RotateSelection { asset_ids: vec!["a1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.5 }),
            ShootingCommand::ScaleSelection(scale_selection::ScaleSelection { asset_ids: vec!["a1".into()], sx: 2.0, sy: 2.0, sz: 2.0 }),
            ShootingCommand::SetCamera(set_camera::SetCamera { camera: default_camera([9.0, 9.0, 9.0]) }),
            ShootingCommand::LoadSavedCamera(load_saved_camera::LoadSavedCamera { id: "cam1".into() }),
            ShootingCommand::SetCameraDraftLabel(set_camera_draft_label::SetCameraDraftLabel { value: "Hero".into() }),
            ShootingCommand::SetCenterModel(set_center_model::SetCenterModel { pressed: Some(true) }),
            ShootingCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "rotate".into() }),
            ShootingCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            ShootingCommand::SetSelection(set_selection::SetSelection { shot_ids: vec!["s1".into()], asset_ids: vec!["a1".into(), "a2".into()] }),
            ShootingCommand::SetSelectionMethod(set_selection_method::SetSelectionMethod { method: "rectangle".into() }),
            ShootingCommand::WorldSelect(world_select::WorldSelect { ids: vec!["a1".into()], merge: "replace".into() }),
            ShootingCommand::SetHover(set_hover::SetHover { asset_id: Some("a1".into()) }),
            ShootingCommand::WorldPick(world_pick::WorldPick { asset_id: Some("a1".into()), asset_index: Some(2), merge: "toggle".into() }),
            ShootingCommand::WorldPointerDown(world_pointer_down::WorldPointerDown {}),
            ShootingCommand::WorldPointerMove(world_pointer_move::WorldPointerMove {}),
            ShootingCommand::SaveDownload(save_download::SaveDownload {}),
            ShootingCommand::LoadRequest(load_request::LoadRequest {}),
            ShootingCommand::ImportAssetRequest(import_asset_request::ImportAssetRequest {}),
            ShootingCommand::ExportShots(export_shots::ExportShots { all: true }),
        ]
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_shooting_app().definition).expect("app definition json");
        for id in [SHOOTING_PLAY_WINDOW_SCENE, SHOOTING_PLAY_WINDOW_ICON] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        for body in [SHOOTING_PLAY_BODY_DOCUMENT, SHOOTING_PLAY_BODY_CATALOGUE, SHOOTING_PLAY_BODY_INSPECTION] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("2d.shooting"), "artifact kind missing from the manifest");
    }

    #[test]
    fn utility_registry_scopes_transform_gumball_and_actions_are_declared() {
        let definition = create_shooting_app().definition;
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(utility_ids, ["move", "rotate", "scale"], "gumball utilities declared in registry order");
        assert!(definition.utilities.iter().all(|utility| utility.group.as_deref() == Some("transform")), "one exclusive transform group");
        let scene = definition.window_kinds.iter().find(|window| window.id == SHOOTING_PLAY_WINDOW_SCENE).expect("scene window");
        let scoped: Vec<&str> = scene.utilities.iter().map(|utility| utility.as_str()).collect();
        assert_eq!(scoped, ["move", "rotate", "scale"], "utilities scoped to the scene window kind");
        for command in ["loadRequest", "importAssetRequest", "saveDownload", "exportActiveShot", "exportAllShots", "resetFixture", "saveCamera"] {
            assert!(definition.actions.iter().any(|action| action.id == command), "registry declares {command}");
        }
        let mut app = shooting_app();
        let engagements = app.window_engagements();
        assert!(engagements[SHOOTING_PLAY_WINDOW_SCENE].options.is_none(), "the gumball selector moved to the host-derived utility bar");
        assert!(engagements[SHOOTING_PLAY_WINDOW_SCENE].status.as_ref().unwrap()[0].text.contains("assets"));
        assert!(engagements[SHOOTING_PLAY_WINDOW_ICON].status.as_ref().unwrap()[0].text.contains("256×256"));
    }

    #[test]
    fn world_pick_is_declared_as_a_view_action_and_emits_no_operations() {
        let definition = create_shooting_app().definition;
        let world_pick_action = definition.actions.iter().find(|action| action.id == "worldPick").expect("worldPick declared");
        assert!(matches!(world_pick_action.kind, ActionKind::View), "worldPick is a View action");
        let mut app = shooting_app_with_registry();
        let result = dispatch(&mut app, ShootingCommand::WorldPick(world_pick::WorldPick { asset_id: None, asset_index: Some(0), merge: "replace".into() }));
        assert!(result.mutations.is_empty(), "worldPick (View) emits no operations even under registry enforcement");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️Locale
    #[test]
    fn shooting_labels_resolve_native_english_by_default() {
        let mut app = shooting_app();
        let document_json = crate::apps::shooting::testkit::render(&mut app, SHOOTING_PLAY_BODY_DOCUMENT);
        assert!(document_json.contains("Shots"));
        assert!(document_json.contains("Assets"));
        let catalogue_json = crate::apps::shooting::testkit::render(&mut app, SHOOTING_PLAY_BODY_CATALOGUE);
        assert!(catalogue_json.contains("Add Shot"));
        assert!(catalogue_json.contains("SVG Rectangle"));
        let engagements = app.window_engagements();
        assert_eq!(engagements[SHOOTING_PLAY_WINDOW_SCENE].input.as_ref().unwrap().placeholder.as_deref(), Some("Camera label"));
        assert_eq!(engagements[SHOOTING_PLAY_WINDOW_ICON].input.as_ref().unwrap().placeholder.as_deref(), Some("Shot label"));
    }

    /// 🗣️ B1: locale is now `cfg.locale`, set via the typed `SetLocale` config command.
    #[test]
    fn shooting_labels_resolve_native_german() {
        let mut app = shooting_app();
        dispatch(&mut app, ShootingCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
        let document_json = crate::apps::shooting::testkit::render(&mut app, SHOOTING_PLAY_BODY_DOCUMENT);
        assert!(document_json.contains("Aufnahmen"));
        assert!(document_json.contains("Objekte"));
        let engagements = app.window_engagements();
        assert_eq!(engagements[SHOOTING_PLAY_WINDOW_SCENE].input.as_ref().unwrap().placeholder.as_deref(), Some("Kamera-Bezeichnung"));
        assert_eq!(engagements[SHOOTING_PLAY_WINDOW_ICON].input.as_ref().unwrap().placeholder.as_deref(), Some("Aufnahme-Bezeichnung"));
    }
    //#endregion 🔖️Locale

    //#region 🔖️CrossCutting
    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = shooting_app();
        testkit::assert_undo_redo_round_trip(&mut app, ShootingCommand::AddShot(add_shot::AddShot { format: "png".into(), shape: "rectangle".into() }), |app| app.snapshot().expect("snapshot").shots.len(), 2, 3);
    }

    /// 🎥️ `SetCamera` is config-only — dragging the viewport camera through several ticks must never
    /// create a VCS edit/undo step on the DOCUMENT store at all.
    #[test]
    fn camera_drag_never_creates_a_document_undo_step() {
        let mut app = shooting_app();
        for position in [[1.0, 0.0, 0.0], [2.0, 0.0, 0.0], [3.0, 0.0, 0.0]] {
            dispatch(&mut app, ShootingCommand::SetCamera(set_camera::SetCamera { camera: default_camera(position) }));
        }
        let camera_position = |app: &mut ShootingApp| -> Value {
            let node = app.render(SHOOTING_PLAY_BODY_SCENE, None, &ViewModel::default()).expect("render");
            let payload: Value = serde_json::to_value(&node).unwrap();
            let camera: Value = serde_json::from_str(payload["world3d"]["cameraJson"].as_str().unwrap()).unwrap();
            camera["position"].clone()
        };
        assert_eq!(camera_position(&mut app), json!([3.0, 0.0, 0.0]), "config camera reflects the last drag tick");
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo (no-op: nothing on the document store to undo)");
        assert_eq!(camera_position(&mut app), json!([3.0, 0.0, 0.0]), "document undo has nothing to revert — the drag never touched the document");
    }

    /// 🧪️ The definitional regression proof: two independent instances start from the same fixture,
    /// apply DISJOINT edits, and exchanging operations over a `MemoryBackbone` converges both sides to
    /// contain BOTH edits.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<ShootingPlayApp, (String, [f64; 3])>(
            "mem://shooting-convergence",
            ShootingCommand::SetActiveShotLabel(set_active_shot_label::SetActiveShotLabel { value: "Renamed By A".into() }),
            ShootingCommand::TranslateSelection(translate_selection::TranslateSelection { asset_ids: vec!["base".into()], dx: 5.0, dy: 6.0, dz: 7.0 }),
            |app| {
                let snapshot = app.snapshot().expect("snapshot");
                (crate::artifacts::shooting::schema::active_shot(&snapshot).unwrap().label.clone(), snapshot.assets[0].origin)
            },
        );
    }

    #[test]
    fn ingest_operations_is_idempotent_for_shooting() {
        testkit::assert_ingest_idempotent::<ShootingPlayApp, String>(ShootingCommand::SetActiveShotLabel(set_active_shot_label::SetActiveShotLabel { value: "Hero".into() }), |app| {
            crate::artifacts::shooting::schema::active_shot(&app.snapshot().expect("snapshot")).unwrap().label.clone()
        });
    }

    #[test]
    fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        let mut app = shooting_app();
        assert!(crate::apps::shooting::testkit::render(&mut app, "shooting.play.nope").contains("Unknown body"));
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️Io
    #[test]
    fn shooting_io_mirrors_the_declared_artifact_kind() {
        let io = shooting_io();
        assert_eq!(io.document_schema, "shooting.scene");
        assert_eq!(io.artifact.id, "2d.shooting");
        // 🗂️ `AppIo` has no `export_stdio_kinds`/`import_stdio_kinds` string peer (see `shooting_io`'s
        // doc comment) — the real format list lives on `artifact_kind()` instead, asserted below.
        assert_eq!(io.export_formats.len(), 0);
        assert_eq!(io.import_formats.len(), 0);
        let kind = crate::artifacts::shooting::artifact_kind();
        assert_eq!(kind.export_stdio_kinds, kind.import_stdio_kinds);
        assert!(kind.export_stdio_kinds.contains(&"stdio.svg"));
        assert!(kind.export_stdio_kinds.contains(&"stdio.png"));
    }

    /// 🔌️ WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE Wave 2 port recipe:
    /// `photos:out` is declared, optional/`Many`, and pinned to the `2d.image` kind.
    #[test]
    fn shooting_io_declares_the_photos_out_port() {
        let io = shooting_io();
        let port = io.ports.iter().find(|port| port.id == "photos:out").expect("photos:out declared");
        assert_eq!(port.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(port.kind_id.as_deref(), Some("2d.image"));
        assert!(!port.required);
        assert_eq!(port.multiplicity, semio_framework::PortMultiplicity::Many);
        assert_eq!(port.media_type.class, MediaClass::TwoD);
        assert_eq!(port.media_type.form, MediaForm::Raster);
    }

    /// 🖼️ `shooting_photo_media` renders the same scene as `exportActiveShot`'s PNG (base64, non-empty).
    #[test]
    fn shooting_photo_media_exports_a_raster_2d_image() {
        let snapshot = crate::artifacts::shooting::schema::default_snapshot();
        let media = shooting_photo_media(&snapshot).expect("photo export succeeds");
        assert_eq!(media.media_type.class, MediaClass::TwoD);
        assert_eq!(media.media_type.form, MediaForm::Raster);
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "2d.image");
                assert!(!json.is_empty());
            }
            MediaPayload::Binary { .. } => panic!("expected a Structured payload"),
        }
    }
    //#endregion 🔖️Io

    //#region 🔖️Export
    #[test]
    fn export_import_and_download_operations() {
        let mut app = shooting_app();
        let result = dispatch(&mut app, ShootingCommand::LoadRequest(load_request::LoadRequest {}));
        match &result.requested_effects[0] {
            HostEffect::RequestFileOpen { import_action, .. } => assert_eq!(import_action, "importSnapshotJson"),
            other => panic!("expected RequestFileOpen, got {other:?}"),
        }
        let result = dispatch(&mut app, ShootingCommand::SaveDownload(save_download::SaveDownload {}));
        match &result.requested_effects[0] {
            HostEffect::DownloadMediaExport { filename, data, .. } => {
                assert_eq!(filename, "shooting.shooting.ops");
                let round_trip: ShootingSnapshot = serde_json::from_str(data).unwrap();
                assert_eq!(round_trip.schema, SHOOTING_DOCUMENT_SCHEMA);
            }
            other => panic!("expected DownloadMediaExport, got {other:?}"),
        }
    }
    //#endregion 🔖️Export
}
//#endregion 🧪️Tests
