//! 🖥️ Shooting editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch. Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: moved here verbatim from
//! the retired `🎛️apps/🎥️shooting/🦀️.rs`, `impl ArtifactApp` → `impl ArtifactEditor`.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, chrome measures in those windows' `🎚️options/*`, panel trees in `📌️panels/*`,
//! labels in `🦀️terminology.rs`, view state in `🦀️config.rs`, shared compute in the artifact's `⚙️engine`.
//! This file is a routing table: `handle` → `ShootingCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::{ShootingSnapshot, SHOOTING_DOCUMENT_SCHEMA};
use crate::editor::shooting::commands::{asset, camera, export, fixture, gumball, locale, scene, selection, shot};
use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::modes::edit;
use crate::editor::shooting::modes::edit::windows::icon as icon_window;
use crate::editor::shooting::modes::edit::windows::scene as scene_window;
use crate::editor::shooting::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::shooting::presence::{ShootingPresence, ShootingPresenceMutation};
use crate::editor::shooting::terminology::shooting_play_labels;
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    tree_item_with_action, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppIo, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactView, ConfigView, Dialect, DraftView,
    DslValue, Editor, EditorApp, Emit, Fault, FaultCode, FaultOrigin, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractiveJobClassification, Label, LocalizedLabel, Media, MediaClass, MediaError,
    MediaForm, MediaPayload, MediaType, MergeMode, NoDraft, NoDraftMutation, OsMediaCapability, SelectionMethod, SelectionMode, SelectionSpec, UiNode, UiTreeItemNode, UtilityDefinition, WindowEngagement, WindowMeasure,
};
use std::collections::HashMap;
use store::EngineHandles;

//#region 🔖️Constants
pub const SHOOTING_PLAY_APP_ID: &str = "shooting-play";
const SHOOTING_PLAY_CONTROLLER_ID: &str = "shooting-play";
/// 🕹️ The framework-owned interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM)?
/// covering asset pick/marquee selection and pointer hover in the 3d scene window — granularity `"asset"`
/// only, `HierarchyProvider::Flat`. Shot selection is NOT part of this domain — see
/// `ShootingConfig::selected_shot_ids`'s doc comment.
pub const SHOOTING_INTERACTION_DOMAIN: &str = "assets";
pub use crate::editor::shooting::commands::fixture::set_active_example::SHOOTING_EXAMPLE_DEFAULT_ID;
pub use catalogue_panel::SHOOTING_PLAY_BODY_CATALOGUE;
pub use document_panel::SHOOTING_PLAY_BODY_DOCUMENT;
pub use icon_window::SHOOTING_PLAY_BODY_ICON;
pub use icon_window::SHOOTING_PLAY_WINDOW_ICON;
pub use inspection_panel::SHOOTING_PLAY_BODY_INSPECTION;
pub use scene_window::SHOOTING_PLAY_BODY_SCENE;
pub use scene_window::SHOOTING_PLAY_WINDOW_SCENE;
//#endregion 🔖️Constants

//#region 🔖️Utilities
/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`🎚️options/*`, `📌️panels/*`) builds its `on_change`/item actions with.
pub fn shooting_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(SHOOTING_PLAY_CONTROLLER_ID).action(action, args)
}


/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref())
        .map(semio_framework_plugin::UiValue::Text)
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
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
    let mut builder = semio_framework_plugin::UiListBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder
            .push(value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder
            .push(key.to_owned(), value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes
            .try_push(node)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
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
    let mut node = tree_item_with_action(id, label, None, action?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.icon = Some(
            semio_framework_plugin::UiText::try_from_str(icon_id)
                .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.tree-item.icon", "fixed tree-item icon admission failed"))?,
        );
    }
    Ok(node)
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
pub async fn shooting_io() -> AppIo {
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
pub async fn shooting_photos_out_port() -> semio_framework_plugin::MediaPortSpec {
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
pub async fn shooting_photo_media(snapshot: &ShootingSnapshot) -> Result<Media, MediaError> {
    let (svg, width, height) = crate::artifacts::shooting::schema::shooting_scene_svg(snapshot).map_err(|error| MediaError::Payload("photos:out".into(), error))?;
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
    /// `setLocale`/`locale` is the row that proves it. **Row order is the binary variant ordinal:
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
        "setActiveUtility" as "active-utility" => set_active_utility::SetActiveUtility,
        "setLocale" as "locale" => set_locale::SetLocale,
        "setShotSelection" as "set-shot-selection" => set_shot_selection::SetShotSelection,
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
use fixture::{import_snapshot_json, load_request, reset_snapshot, save_download, set_active_example};
use gumball::{rotate_selection, scale_selection, translate_selection};
use locale::set_locale;
use scene::{set_ambient_intensity, set_material_roughness, set_shadow_enabled, set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use selection::{set_active_utility, set_center_model, set_shot_selection, world_pointer_down, world_pointer_move};
use shot::{add_shot, patch_shots, set_active_shot, set_active_shot_format, set_active_shot_label, set_active_shot_shape};
//#endregion 🔖️Commands

//#region 🔖️ShootingPlayApp
/// 🧪️ B1: unit struct — every former runtime field now lives in `ShootingConfig`, written through
/// `ShootingConfigMutation`s.
#[derive(Default)]
pub struct ShootingPlayApp;

//#region 🧵️RetainedCommands
const SHOOTING_BOUNDED_TOOL_IDS: &[&str] = &["loadRequest", "importAssetRequest"];
const SHOOTING_RETAINED_PAYLOAD_SCHEMA: &str = "shooting.shooting.tool-command.v1";
const SHOOTING_BOUNDED_RAW_BYTES: usize = 65_536;
const SHOOTING_BOUNDED_WORK_ITEMS: usize = 1;

fn shooting_command_id(command: &ShootingCommand) -> &'static str {
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

fn shooting_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(SHOOTING_BOUNDED_RAW_BYTES, 64, SHOOTING_BOUNDED_WORK_ITEMS, 262_144, 7_500)
}

fn shooting_bounded_extent(command: &ShootingCommand, _snapshot: &ShootingSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    SHOOTING_BOUNDED_TOOL_IDS.contains(&shooting_command_id(command)).then_some(SHOOTING_BOUNDED_WORK_ITEMS)
}

fn shooting_bounded_reduce(
    command: &ShootingCommand,
    snapshot: &ShootingSnapshot,
    config: &ShootingConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &AppOperationContext,
) -> Result<Emit<ShootingMutation, ShootingConfigMutation, NoDraftMutation>, Fault> {
    if !SHOOTING_BOUNDED_TOOL_IDS.contains(&shooting_command_id(command)) {
        return Err(Fault::new(FaultOrigin::App, FaultCode::new("shooting.retained.route"), "the bounded Shooting reducer rejects resumable routes"));
    }
    let mut ctx = ShootingDispatchCtx::default();
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config }, &mut ctx)
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

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
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
    type Owner = semio_framework_plugin::EditorApp<ShootingPlayApp>;
    const TOOL_IDS: &'static [&'static str] = SHOOTING_BOUNDED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SHOOTING_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "loadRequest", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "importAssetRequest", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
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

    const DIALECT: Dialect = crate::artifacts::shooting::SHOOTING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SHOOTING_DOCUMENT_SCHEMA;

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<ShootingPlayApp>,
        owner_file: "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.shooting.shooting@1/*#editor",
        document_schema: "shooting.shooting",
        factory: "ShootingCommandJobFactory",
        factory_type: ShootingCommandJobFactory,
        tools: {
            "loadRequest" => semio_framework::ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
            "importAssetRequest" => semio_framework::ToolExecutionContract::bounded_first_step(65_536, 64, 1, 262_144, 7_500),
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
        if shooting_command_id(&request.command) != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("shooting.retained.tool-mismatch"), "Shooting command does not match its exact registered tool"));
        }
        if shooting_bounded_extent(&request.command, &request.snapshot, &request.interaction_state).is_none() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("shooting.retained.extent"), "Shooting bounded route exceeded its declared work extent"));
        }
        let tool_id = shooting_command_id(&request.command);
        let work = Box::new(BoundedArtifactCommandWork::new(tool_id, shooting_bounded_reduce, shooting_bounded_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new_with_context(
            *request.command,
            request.snapshot,
            request.config,
            request.history,
            request.interaction_state,
            request.interaction_hover,
            request.context,
            operation_context,
            request.completion,
            shooting_command_id,
            SHOOTING_BOUNDED_RAW_BYTES,
            SHOOTING_BOUNDED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::shooting::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> ShootingSnapshot {
        crate::artifacts::shooting::schema::default_snapshot()
    }

    async fn io() -> Option<AppIo> {
        Some(shooting_io())
    }

    /// 🎞️ `photos:out` (see `shooting_photo_media`) plus the
    /// inherited `document:out` default (the pack of `doc.snapshot`, replicated inline — overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new
    /// one).
    async fn export_media(port: &str, doc: &ArtifactView<'_, ShootingSnapshot>) -> Result<Media, MediaError> {
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
    async fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, ShootingSnapshot>) -> Result<Emit<ShootingMutation, ShootingConfigMutation, NoDraftMutation>, MediaError> {
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
    async fn command_id(command: &ShootingCommand) -> &'static str {
        shooting_command_id(command)
    }

    async fn handle(
        command: &ShootingCommand,
        doc: &ArtifactView<'_, ShootingSnapshot>,
        cfg: &ConfigView<'_, ShootingConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<ShootingMutation, ShootingConfigMutation, Self::DraftMutation>, Fault> {
        let mut ctx = ShootingDispatchCtx { selected_asset_ids: interaction.selection(SHOOTING_INTERACTION_DOMAIN).ids.clone() };
        command.dispatch(doc, cfg, &mut ctx)
    }

    /// 🧮️ This app's typed configuration spec — mirrors `ShootingConfig`'s three sticky-default fields,
    /// each grounded in an existing `.action_args` default (see that struct's doc).
    async fn config_spec() -> semio_framework_plugin::ConfigSpec {
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

    async fn render(body_key: &str, doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
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

    async fn window_engagements(doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>) -> HashMap<String, WindowEngagement> {
        let labels = shooting_play_labels(cfg.snapshot);
        HashMap::from([(SHOOTING_PLAY_WINDOW_SCENE.into(), scene_window::engagement(doc.snapshot, cfg.snapshot, labels)), (SHOOTING_PLAY_WINDOW_ICON.into(), icon_window::engagement(doc.snapshot, labels))])
    }

    async fn window_measures(doc: &ArtifactView<'_, ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = shooting_play_labels(cfg.snapshot);
        HashMap::from([(SHOOTING_PLAY_WINDOW_SCENE.into(), scene_window::window_measures(doc.snapshot, labels)), (SHOOTING_PLAY_WINDOW_ICON.into(), icon_window::window_measures(doc.snapshot, labels))])
    }
}
//#endregion 🔖️ShootingPlayApp

//#region 🔖️ResetDocument
/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `scene` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (file import,
/// load-example, dev fixture load). Per `📓️taxonomy.md`, whole-document replace is banned outright with NO
/// replacement mutation: whole-document replace is not expressible as an in-history `Mutation` at
/// all. Every former "replace the whole document" gesture in this package (`import_media`'s
/// `"document:in"` above, `commands::fixture::{import_snapshot_json,set_active_example,reset_snapshot}`)
/// builds this effect instead of an `Emit::mutations([...])`. The spr is a fresh, edit-free op-log
/// for `scene` — a genesis envelope with no history to encode.
pub async fn reset_document_effect(scene: &ShootingSnapshot) -> semio_framework_plugin::Effect {
    let pack = <ShootingSnapshot as store::ArtifactPack>::encode_pack(scene);
    let envelope = store::create_document_envelope::<ShootingSnapshot, ShootingMutation>(SHOOTING_DOCUMENT_SCHEMA, "shooting", scene.clone(), None);
    let spr = store::print_document_spr(&envelope).expect("shooting document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub async fn create_shooting_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::shooting::SHOOTING_DIALECT)
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
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("importSnapshotJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"), ActionKind::Mutation) })
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
            // 👁️ Ephemeral view state — shot gallery selection, camera draft label, transform utility.
            .view_action("setShotSelection", LocalizedLabel::native("Set Shot Selection", "Aufnahmeauswahl festlegen"))
            .view_action("setCameraDraftLabel", LocalizedLabel::native("Set Camera Draft Label", "Kamera-Entwurfsbezeichnung festlegen"))
            .view_action("setCenterModel", LocalizedLabel::native("Set Center Model", "Modellzentrierung festlegen"))
            .view_action("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"))
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
            // 🧵️ These two reducers emit exactly one fixed host file-open request. Every document,
            // config, codec, renderer, selection, and placeholder route remains fail-closed until its
            // completion lane has an installed bounded preparation owner or a real resumable cursor.
            .action_interactive_job("loadRequest", InteractiveJobClassification::Migrated)
            .action_interactive_job("importAssetRequest", InteractiveJobClassification::Migrated)
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

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::app::EditorApp;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsArtifactApp, ViewModel, WindowMeasure};

    pub type ShootingApp = VcsArtifactApp<EditorApp<ShootingPlayApp>>;

    /// ✏️ `ShootingPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<ShootingPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<ShootingPlayApp>` builds it.
    ///
    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn shooting_app() -> ShootingApp {
        new_app::<EditorApp<ShootingPlayApp>>()
    }

    /// ✏️ Adapts `create_shooting_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `new_app_with_registry`/`assert_declared_actions_bridge_to_commands` still
    /// expect — framework testkit gap, not modifiable here (`🧰️framework/**` is outside this packet's
    /// lease).
    pub async fn shooting_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_shooting_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn shooting_app_with_registry() -> ShootingApp {
        new_app_with_registry::<EditorApp<ShootingPlayApp>>(shooting_app_manifest_for_testkit)
    }

    pub async fn dispatch(app: &mut ShootingApp, command: ShootingCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub async fn render(app: &mut ShootingApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    pub async fn scene_window_measures(app: &mut ShootingApp) -> Vec<WindowMeasure> {
        app.window_measures().get(SHOOTING_PLAY_WINDOW_SCENE).cloned().expect("scene window measures")
    }

    pub async fn icon_window_measures(app: &mut ShootingApp) -> Vec<WindowMeasure> {
        app.window_measures().get(SHOOTING_PLAY_WINDOW_ICON).cloned().expect("icon window measures")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::testkit::{dispatch, shooting_app, shooting_app_with_registry, ShootingApp};
    use semio_framework_plugin::app::EditorApp;
    use semio_framework_plugin::testkit;
    use semio_framework_plugin::{ActionKind, Effect, PluginApp, ViewModel};
    use serde_json::{json, Value};

    async fn default_camera(position: [f64; 3]) -> crate::artifacts::shooting::ShootingCamera {
        crate::artifacts::shooting::ShootingCamera { position, target: [0.0, 0.0, 0.0], zoom: 1.0, fov: 50.0, up: None, projection: None }
    }

    //#region 🧵️RetainedCatalogOracle
    #[derive(Debug, PartialEq, Eq)]
    struct ShootingRetainedCatalogSummary {
        routes: usize,
        bounded: usize,
        resumable: usize,
        migrated: usize,
        fail_closed: usize,
        unique: bool,
        route_ids: std::collections::BTreeSet<String>,
        bounded_ids: std::collections::BTreeSet<String>,
        host_only_ids: std::collections::BTreeSet<String>,
    }

    trait ShootingRetainedCatalogOracle {
        fn summarize(&self, fixture: &str) -> ShootingRetainedCatalogSummary;
    }

    struct SerdeJsonShootingRetainedCatalogOracle;

    impl ShootingRetainedCatalogOracle for SerdeJsonShootingRetainedCatalogOracle {
        fn summarize(&self, fixture: &str) -> ShootingRetainedCatalogSummary {
            let document: serde_json::Value = serde_json::from_str(fixture).expect("language-neutral retained catalog fixture");
            let routes = document.get("routes").and_then(serde_json::Value::as_array).expect("routes array");
            let bounded = routes.iter().filter(|route| route.get("execution").and_then(serde_json::Value::as_str) == Some("bounded")).count();
            let resumable = routes.iter().filter(|route| route.get("execution").and_then(serde_json::Value::as_str) == Some("resumable")).count();
            let migrated = routes.iter().filter(|route| route.get("admission").and_then(serde_json::Value::as_str) == Some("migrated")).count();
            let fail_closed = routes.iter().filter(|route| route.get("admission").and_then(serde_json::Value::as_str) == Some("failClosed")).count();
            let route_ids = routes
                .iter()
                .filter_map(|route| route.get("id").and_then(serde_json::Value::as_str).map(str::to_string))
                .collect::<std::collections::BTreeSet<_>>();
            let bounded_ids = routes
                .iter()
                .filter(|route| route.get("execution").and_then(serde_json::Value::as_str) == Some("bounded"))
                .filter_map(|route| route.get("id").and_then(serde_json::Value::as_str).map(str::to_string))
                .collect::<std::collections::BTreeSet<_>>();
            let host_only_ids = document
                .get("publicationContracts")
                .and_then(serde_json::Value::as_array)
                .expect("publication contracts array")
                .iter()
                .filter(|contract| contract.get("lanes").and_then(serde_json::Value::as_array).is_some_and(|lanes| lanes.as_slice() == [serde_json::Value::String("hostOnly".into())]))
                .filter_map(|contract| contract.get("toolId").and_then(serde_json::Value::as_str).map(str::to_string))
                .collect::<std::collections::BTreeSet<_>>();
            ShootingRetainedCatalogSummary { routes: routes.len(), bounded, resumable, migrated, fail_closed, unique: route_ids.len() == routes.len(), route_ids, bounded_ids, host_only_ids }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_command_catalog_matches_the_serde_json_oracle() {
        let oracle = SerdeJsonShootingRetainedCatalogOracle.summarize(include_str!("🧪️fixtures/🧫️retained-command-limits/🔣️.json"));
        let mut command_ids = every_command()
            .iter()
            .map(|command| shooting_command_id(command).to_string())
            .collect::<std::collections::BTreeSet<_>>();
        command_ids.insert("exportActiveShot".to_string());
        let bounded_ids = SHOOTING_BOUNDED_TOOL_IDS.iter().map(|id| (*id).to_string()).collect::<std::collections::BTreeSet<_>>();
        let host_only_ids = <ShootingCommandJobFactory as semio_framework_plugin::ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS
            .iter()
            .filter(|contract| contract.lanes == [semio_framework_plugin::ArtifactToolPublicationLane::HostOnly])
            .map(|contract| contract.tool_id.to_string())
            .collect::<std::collections::BTreeSet<_>>();
        let subject = ShootingRetainedCatalogSummary {
            routes: command_ids.len(),
            bounded: bounded_ids.len(),
            resumable: command_ids.difference(&bounded_ids).count(),
            migrated: bounded_ids.len(),
            fail_closed: command_ids.difference(&bounded_ids).count(),
            unique: bounded_ids.len() == SHOOTING_BOUNDED_TOOL_IDS.len() && bounded_ids.is_subset(&command_ids),
            route_ids: command_ids.clone(),
            bounded_ids: bounded_ids.clone(),
            host_only_ids,
        };
        assert_eq!(
            oracle,
            ShootingRetainedCatalogSummary {
                routes: 39,
                bounded: 2,
                resumable: 37,
                migrated: 2,
                fail_closed: 37,
                unique: true,
                route_ids: command_ids,
                bounded_ids: bounded_ids.clone(),
                host_only_ids: bounded_ids,
            }
        );
        assert_eq!(subject, oracle);
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_publication_oracle_rejects_hostile_tool_and_lane_fixtures() {
        let fixture = include_str!("🧪️fixtures/🧫️retained-command-limits/🔣️.json");
        let expected = SHOOTING_BOUNDED_TOOL_IDS.iter().map(|id| (*id).to_string()).collect::<std::collections::BTreeSet<_>>();
        let wrong_lane = fixture.replacen("\"hostOnly\"", "\"artifact\"", 1);
        let wrong_tool = fixture.replacen("\"loadRequest\"", "\"forgedRequest\"", 1);
        assert_ne!(SerdeJsonShootingRetainedCatalogOracle.summarize(&wrong_lane).host_only_ids, expected);
        assert_ne!(SerdeJsonShootingRetainedCatalogOracle.summarize(&wrong_tool).host_only_ids, expected);
    }
    //#endregion 🧵️RetainedCatalogOracle

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
    /// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to
    /// hold.
    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique_across_every_row() {
        let app = ShootingPlayApp;
        let ids: Vec<&str> = every_command().iter().map(shooting_command_id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 38, "every ShootingCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) async fn every_command() -> Vec<ShootingCommand> {
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
            ShootingCommand::SetShotSelection(set_shot_selection::SetShotSelection { shot_ids: vec!["s1".into()] }),
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
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_shooting_app()).expect("app definition json");
        for id in [SHOOTING_PLAY_WINDOW_SCENE, SHOOTING_PLAY_WINDOW_ICON] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        for body in [SHOOTING_PLAY_BODY_DOCUMENT, SHOOTING_PLAY_BODY_CATALOGUE, SHOOTING_PLAY_BODY_INSPECTION] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("2d.shooting"), "artifact kind missing from the manifest");
    }

    #[semio_framework_async_macros::async_test]
    async fn utility_registry_scopes_transform_gumball_and_actions_are_declared() {
        let definition = create_shooting_app();
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(utility_ids, ["move", "rotate", "scale"], "gumball utilities declared in registry order");
        assert!(definition.utilities.iter().all(|utility| utility.group.as_deref() == Some("transform")), "one exclusive transform group");
        let scene = definition.window_kinds.iter().find(|window| window.id == SHOOTING_PLAY_WINDOW_SCENE).expect("scene window");
        let scoped: Vec<&str> = scene.utilities.iter().map(|utility| utility.as_str()).collect();
        assert_eq!(scoped, ["move", "rotate", "scale"], "utilities scoped to the scene window kind");
        for command in ["loadRequest", "importAssetRequest", "saveDownload", "exportActiveShot", "exportAllShots", "resetFixture", "saveCamera"] {
            assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == command), "registry declares {command}");
        }
        let mut app = shooting_app();
        let engagements = app.window_engagements();
        assert!(engagements[SHOOTING_PLAY_WINDOW_SCENE].options.is_none(), "the gumball selector moved to the host-derived utility bar");
        assert!(engagements[SHOOTING_PLAY_WINDOW_SCENE].status.as_ref().unwrap()[0].text.contains("assets"));
        assert!(engagements[SHOOTING_PLAY_WINDOW_ICON].status.as_ref().unwrap()[0].text.contains("256×256"));
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: replaces the deleted
    /// `world_pick_is_declared_as_a_view_action_and_emits_no_operations` test — asset pick/select is the
    /// framework-injected `interactionSelect` verb now (no app-declared action id), asserted here
    /// instead of a bespoke `worldPick` action.
    #[semio_framework_async_macros::async_test]
    async fn interaction_select_is_reachable_as_a_framework_injected_action_under_registry_enforcement() {
        let mut app = shooting_app_with_registry();
        let asset_id = app.snapshot().expect("snapshot").assets[0].id.clone();
        let targets = serde_json::to_string(&serde_json::json!([{ "granularity": "asset", "id": asset_id }])).unwrap();
        app.handle_action("interactionSelect", Some(&json!({ "domainId": SHOOTING_INTERACTION_DOMAIN, "targets": targets, "merge": "replace" })), &testkit::meta("local")).expect("interactionSelect");
    }

    #[semio_framework_async_macros::async_test]
    async fn assets_interaction_domain_is_declared_and_scoped_to_the_scene_window() {
        let definition = create_shooting_app();
        let domain = definition.interactions.iter().find(|interaction| interaction.id == SHOOTING_INTERACTION_DOMAIN).expect("assets interaction domain declared");
        assert_eq!(domain.granularities.len(), 1);
        assert_eq!(domain.granularities[0].id, "asset");
        assert!(matches!(domain.hierarchy, HierarchyProvider::Flat));
        let scene = definition.window_kinds.iter().find(|window| window.id == SHOOTING_PLAY_WINDOW_SCENE).expect("scene window");
        assert!(scene.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == SHOOTING_INTERACTION_DOMAIN));
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️Locale
    #[semio_framework_async_macros::async_test]
    async fn shooting_labels_resolve_native_english_by_default() {
        let mut app = shooting_app();
        let document_json = crate::editor::shooting::testkit::render(&mut app, SHOOTING_PLAY_BODY_DOCUMENT);
        assert!(document_json.contains("Shots"));
        assert!(document_json.contains("Assets"));
        let catalogue_json = crate::editor::shooting::testkit::render(&mut app, SHOOTING_PLAY_BODY_CATALOGUE);
        assert!(catalogue_json.contains("Add Shot"));
        assert!(catalogue_json.contains("SVG Rectangle"));
        let engagements = app.window_engagements();
        assert_eq!(engagements[SHOOTING_PLAY_WINDOW_SCENE].input.as_ref().unwrap().placeholder.as_deref(), Some("Camera label"));
        assert_eq!(engagements[SHOOTING_PLAY_WINDOW_ICON].input.as_ref().unwrap().placeholder.as_deref(), Some("Shot label"));
    }

    /// 🗣️ B1: locale is now `cfg.locale`, set via the typed `SetLocale` config command.
    #[semio_framework_async_macros::async_test]
    async fn shooting_labels_resolve_native_german() {
        let mut app = shooting_app();
        dispatch(&mut app, ShootingCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }));
        let document_json = crate::editor::shooting::testkit::render(&mut app, SHOOTING_PLAY_BODY_DOCUMENT);
        assert!(document_json.contains("Aufnahmen"));
        assert!(document_json.contains("Objekte"));
        let engagements = app.window_engagements();
        assert_eq!(engagements[SHOOTING_PLAY_WINDOW_SCENE].input.as_ref().unwrap().placeholder.as_deref(), Some("Kamera-Bezeichnung"));
        assert_eq!(engagements[SHOOTING_PLAY_WINDOW_ICON].input.as_ref().unwrap().placeholder.as_deref(), Some("Aufnahme-Bezeichnung"));
    }
    //#endregion 🔖️Locale

    //#region 🔖️CrossCutting
    #[semio_framework_async_macros::async_test]
    async fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = shooting_app();
        testkit::assert_undo_redo_round_trip(&mut app, ShootingCommand::AddShot(add_shot::AddShot { format: "png".into(), shape: "rectangle".into() }), |app| app.snapshot().expect("snapshot").shots.len(), 2, 3);
    }

    /// 🎥️ `SetCamera` is config-only — dragging the viewport camera through several ticks must never
    /// create a VCS edit/undo step on the DOCUMENT store at all.
    #[semio_framework_async_macros::async_test]
    async fn camera_drag_never_creates_a_document_undo_step() {
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
    #[semio_framework_async_macros::async_test]
    async fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<EditorApp<ShootingPlayApp>, (String, [f64; 3])>(
            "mem://shooting-convergence",
            ShootingCommand::SetActiveShotLabel(set_active_shot_label::SetActiveShotLabel { value: "Renamed By A".into() }),
            ShootingCommand::TranslateSelection(translate_selection::TranslateSelection { asset_ids: vec!["base".into()], dx: 5.0, dy: 6.0, dz: 7.0 }),
            |app| {
                let snapshot = app.snapshot().expect("snapshot");
                (crate::artifacts::shooting::schema::active_shot(&snapshot).unwrap().label.clone(), snapshot.assets[0].origin)
            },
        );
    }

    #[semio_framework_async_macros::async_test]
    async fn ingest_operations_is_idempotent_for_shooting() {
        testkit::assert_ingest_idempotent::<EditorApp<ShootingPlayApp>, String>(ShootingCommand::SetActiveShotLabel(set_active_shot_label::SetActiveShotLabel { value: "Hero".into() }), |app| {
            crate::artifacts::shooting::schema::active_shot(&app.snapshot().expect("snapshot")).unwrap().label.clone()
        });
    }

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        let mut app = shooting_app();
        assert!(crate::editor::shooting::testkit::render(&mut app, "shooting.play.nope").contains("Unknown body"));
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️Io
    #[semio_framework_async_macros::async_test]
    async fn shooting_io_mirrors_the_declared_artifact_kind() {
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
    #[semio_framework_async_macros::async_test]
    async fn shooting_io_declares_the_photos_out_port() {
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
    #[semio_framework_async_macros::async_test]
    async fn shooting_photo_media_exports_a_raster_2d_image() {
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
    #[semio_framework_async_macros::async_test]
    async fn export_import_and_download_operations() {
        let mut app = shooting_app();
        let result = dispatch(&mut app, ShootingCommand::LoadRequest(load_request::LoadRequest {}));
        match &result.requested_effects[0] {
            Effect::RequestFileOpen { import_action, .. } => assert_eq!(import_action, "importSnapshotJson"),
            other => panic!("expected RequestFileOpen, got {other:?}"),
        }
        let result = dispatch(&mut app, ShootingCommand::SaveDownload(save_download::SaveDownload {}));
        match &result.requested_effects[0] {
            Effect::DownloadMediaExport { filename, data, .. } => {
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
