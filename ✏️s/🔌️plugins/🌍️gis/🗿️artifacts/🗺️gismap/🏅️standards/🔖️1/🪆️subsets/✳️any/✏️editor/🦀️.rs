//! 🗺️ GIS 2D play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the map canvas
//! and its chrome in `🎭️modes/✏️edit/🪟️windows/🗺️map` (+ its `🎚️options/*`), panel trees in
//! `📌️panels/*`, labels in `🦀️terminology.rs`, view state in `🦀️config.rs`, the shared `MapHost`
//! projection in `🦀️maphost.rs`, and document-side compute in `crate::artifacts::gismap::schema`.
//! This app's typed media I/O surface (`gis2d_io`/ports/`gis2d_map_media`) lives below in `🔖️Io` —
//! relocated from the artifact's `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES),
//! since an `AppIo` surface is app behaviour, not artifact data.

use crate::artifacts::gismap::op::GisMapMutation;
use crate::artifacts::gismap::schema::{gis_map_document_from_descriptor_json, positions_operations, regions_operations, routes_operations};
use crate::artifacts::gismap::{artifact_kind, GisMapSnapshot, GIS_MAP_SCHEMA};
use crate::editor::gis2d::commands::{example, features, inference, locale, shell, view};
use crate::editor::gis2d::config::{Gis2dConfig, Gis2dConfigMutation};
use crate::editor::gis2d::modes::edit;
use crate::editor::gis2d::modes::edit::windows::map;
use crate::editor::gis2d::panels::{artifact as document_panel, catalogue as catalogue_panel, inspection as inspection_panel};
use crate::editor::gis2d::terminology::gis2d_labels;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    tree_item, tree_item_with_action, ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppIo, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry,
    ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, Dialect, DraftView, Editor, EditorApp, Emit, Fault, GranularityDefinition,
    HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, Menu, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode,
    SelectionSpec, UiNode, UiTreeItemNode, WindowMeasure, INTERACTION_SELECT_ACTION_ID,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use store::ArtifactPack;
use store::EngineHandles;

//#region 🔖️Constants
pub const GIS2D_PLAY_APP_ID: &str = "gis2d-play";

/// 🗂️ The app-wide map layer stack: `(id, native English name, icon id)?`. Every chrome node (document
/// and catalogue trees, the layers/weights window options, the inspector summary) enumerates it.
pub const GIS_MAP_LAYER_IDS: &[(&str, &str, &str)] = &[
    ("raster", "Raster", "layers"),
    ("water", "Water", "cloud"),
    ("land", "Land", "trees"),
    ("roads", "Roads", "compass"),
    ("buildings", "Buildings", "building"),
    ("borders", "Borders", "square-dashed"),
    ("labels", "Labels", "type"),
    ("positions", "Positions", "crosshair"),
    ("positionLabels", "Position Labels", "tags"),
    ("routes", "Routes", "git-branch"),
    ("regions", "Regions", "layers"),
];

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🎚️options/*`) builds its `on_change`/item actions with.
pub fn gis2d_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(GIS2D_PLAY_APP_ID).action(action, args)
}

/// 🪟️ Bridges semantic app actions into the retained window-measure transport.
pub fn gis2d_window_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: GIS2D_PLAY_APP_ID.into(), action: action.into(), args: semio_framework::optional_json_to_dsl(args) }
}

/// 🏷️ Admits resolved app text into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::Label> {
    semio_framework_plugin::plugin_app_close_prelude::Label::try_from(value.as_ref())
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "GIS UI label admission failed"))
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


/// 🌳️ A layer tree item — `tree_item_with_action`/`tree_item` plus the icon that identifies each map
/// layer, since the SDK's `PanelKit` family has no icon-carrying constructor. Shared by the document
/// panel (`action: None` — the tree is `interaction_domain`-bound now, so the framework's renderer
/// translates clicks into injected `interactionSelect`) and the catalogue panel (`action: Some(..)` —
/// a real, non-selection click that toggles layer visibility).
pub fn gis2d_layer_tree_item(
    id: impl AsRef<str>,
    label: semio_framework_plugin::plugin_app_close_prelude::Label,
    description: Option<String>,
    icon_id: &str,
    action: Option<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut node = match action {
        Some(action) => tree_item_with_action(id, label, description.clone(), action)?,
        None => tree_item(id, label)?,
    };
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        if props.description.is_none() {
            props.description = match description {
                Some(value) => Some(semio_framework_plugin::UiText::try_from_string(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "gis layer description admission failed"))?),
                None => None,
            };
        }
        props.icon = Some(semio_framework_plugin::UiText::try_from_str(icon_id).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "gis layer icon admission failed"))?);
    }
    Ok(node)
}
//#endregion 🔖️Constants

//#region 🔖️Io
/// 🧭️ Relocated from the artifact's `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this app's typed media I/O surface
/// (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` `crate::artifacts::gismap::artifact_kind()`
/// declares (schema/media type/export+import formats/presentation fields copied verbatim), plus the
/// two app-specific workflow ports (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE
/// Wave 2 port recipe): `features:in` (any TwoD×Vector producer feeds new/patched
/// positions/routes/regions) and `map:out` (this document's own feature layers, the `2d.map`
/// interchange kind gis3d's `map:in` consumes).
pub fn gis2d_io() -> AppIo {
    AppIo {
        document_schema: GIS_MAP_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        ports: vec![gis2d_features_in_port(), gis2d_map_out_port()],
        // 🚮️ V7 deprecated-codec-enum retirement (SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT):
        // `AppIo.{export,import}_formats` stays framework-owned and carries no `&'static str`
        // stdio-kind-id peer field to move real values onto (unlike `ArtifactKindSpec`) — emptied,
        // mirroring the sibling `gis3d_io()` (gis3d app) fix already applied in this ticket.
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: crate::artifacts::gismap::GISMAP_DIALECT.artifact_kind.into(), name: "2D Map".into(), dimension: "2d".into(), component_kind: "gismap".into() },
    }
}

/// 🔌️ `features:in` — accepts any TwoD×Vector producer (draw's `vector:out`, another gis2d's
/// `map:out`, …); no `kind_id` pin since it's a generic vector-features sink, not one specific kind.
/// `Many`/optional: several producers may fan into one map, and a map with no upstream edge is valid.
pub fn gis2d_features_in_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "features:in".into(),
        label: "Features".into(),
        direction: semio_framework_plugin::MediaPortDirection::In,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        kind_id: None,
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🔌️ `map:out` — this document's positions/routes/regions as the `2d.map` interchange kind (gis3d's
/// `map:in` consumes it). `Many`/optional: several downstream consumers may fan out from one map, and a
/// map with no downstream edge is valid.
pub fn gis2d_map_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "map:out".into(),
        label: "Map".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        kind_id: Some(crate::artifacts::gismap::GISMAP_DIALECT.artifact_kind.into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🎞️ `map:out`'s `Media` value — this document's positions/routes/regions as a `2d.map` structured
/// payload; reuses the exact descriptor JSON shape the ◻️2d window's renderer/`MapHost` already consume,
/// so there is exactly one "gis map as JSON" shape in the whole app.
pub fn gis2d_map_media(document: &GisMapSnapshot) -> Media {
    Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "2d.map".into(), json: crate::artifacts::gismap::schema::gis_map_descriptor_json(document) } }
}
//#endregion 🔖️Io

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Gis2dPlayApp::Command` — the SOLE dispatch surface for gis2d's own behavior, covering every
    /// action `create_gis2d_app` declares. Row order is the binary variant ordinal: appending is safe,
    /// reordering is a wire-format break.
    pub enum Gis2dCommand for GisMapSnapshot, GisMapMutation, Gis2dConfig, Gis2dConfigMutation {
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "patchPositions" as "patch-positions" => patch_positions::PatchPositions,
        "patchRoutes" as "patch-routes" => patch_routes::PatchRoutes,
        "patchRoute" as "patch-route" => patch_route::PatchRoute,
        "toggleLayerVisibility" as "toggle-layer-visibility" => toggle_layer_visibility::ToggleLayerVisibility,
        "fitWorld" as "fit-world" => fit_world::FitWorld,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setRenderMode" as "render-mode" => set_render_mode::SetRenderMode,
        "setVectorStyle" as "vector-style" => set_vector_style::SetVectorStyle,
        "setLodMode" as "lod-mode" => set_lod_mode::SetLodMode,
        "focusFeature" as "focus-feature" => focus_feature::FocusFeature,
        "setLayerStrokeScale" as "layer-stroke-scale" => set_layer_stroke_scale::SetLayerStrokeScale,
        "setLocale" as "locale" => set_locale::SetLocale,
        "openSource" as "open-source" => open_source::OpenSource,
        "proposeBoundsRegion" as "propose-bounds-region" => propose_bounds_region::ProposeBoundsRegion,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier.
use example::set_active_example;
use features::{patch_positions, patch_route, patch_routes};
use inference::propose_bounds_region;
use locale::set_locale;
use shell::open_source;
use view::{fit_world, focus_feature, set_camera, set_layer_stroke_scale, set_lod_mode, set_render_mode, set_vector_style, toggle_layer_visibility};
//#endregion 🔖️Commands

//#region 🔖️Gis2dPlayApp
/// 🗺️ GIS 2D map play app. The document holds positions/routes/regions; everything else (camera,
/// render mode, style, LOD, layer visibility, stroke weights, locale) is [`Gis2dConfig`] — a
/// session-only but real, undoable config artifact. Layer AND feature selection/hover now live in
/// the framework-owned `"features"` interaction domain (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
#[derive(Default)]
pub struct Gis2dPlayApp;

//#region 🧵️RetainedCommands
const GIS2D_RETAINED_TOOL_IDS: &[&str] = &[
    "setActiveExample",
    "patchPositions",
    "patchRoutes",
    "patchRoute",
    "toggleLayerVisibility",
    "fitWorld",
    "setCamera",
    "setRenderMode",
    "setVectorStyle",
    "setLodMode",
    "focusFeature",
    "setLayerStrokeScale",
    "openSource",
    "setLocale",
    "proposeBoundsRegion",
];
const GIS2D_RETAINED_PAYLOAD_SCHEMA: &str = "gis.map.tool-command.v1";
const GIS2D_RETAINED_RAW_BYTES: usize = 8_192;
const GIS2D_RETAINED_WORK_ITEMS: usize = 64;

const GIS2D_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "patchPositions", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchRoutes", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchRoute", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "toggleLayerVisibility", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "fitWorld", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setRenderMode", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setVectorStyle", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setLodMode", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "focusFeature", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setLayerStrokeScale", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "openSource", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "proposeBoundsRegion", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];

fn gis2d_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GIS2D_RETAINED_RAW_BYTES, 64, GIS2D_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

fn gis2d_retained_extent(command: &Gis2dCommand, _snapshot: &GisMapSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    GIS2D_RETAINED_TOOL_IDS.contains(&command.command_id()).then_some(1)
}

fn gis2d_retained_reduce(
    command: &Gis2dCommand,
    snapshot: &GisMapSnapshot,
    config: &Gis2dConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &AppOperationContext,
) -> Result<Emit<GisMapMutation, Gis2dConfigMutation, NoDraftMutation>, Fault> {
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config })
}

struct Gis2dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Gis2dRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GIS2D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for Gis2dRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Gis2dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Gis2dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GIS2D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        gis2d_retained_contract()
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
        if input.declared_bytes() > GIS2D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("GIS map retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for Gis2dRetainedCommandJobFactory {
    type Owner = EditorApp<Gis2dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = GIS2D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GIS_MAP_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = GIS2D_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 📬️OneItemPreparation
struct Gis2dOneItemPreparationFactory<P, M> {
    marker: std::marker::PhantomData<fn() -> (P, M)>,
}

impl<P, M> Default for Gis2dOneItemPreparationFactory<P, M> {
    fn default() -> Self {
        Self { marker: std::marker::PhantomData }
    }
}

struct Gis2dOneItemPreparation<P, M> {
    base: Option<store::SnapshotRead<P>>,
    mutation: Option<M>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<P, M>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

fn gis2d_one_item_edit<M>(forward: M, inverse: Vec<M>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<M> {
    let id = format!("gis2d-retained-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

impl<P, M> store::ArtifactStoreOneItemPreparationFactory<P, M> for Gis2dOneItemPreparationFactory<P, M>
where
    P: Clone + Send + Sync + 'static,
    M: protocol::Mutation<P> + Send + Sync + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn preflight(&self, _mutation: &M, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("GIS map retained preparation rejected its lane or description envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<P, M>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, store::ArtifactStoreOneItemPreparationRequest<P, M>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Gis2dOneItemPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl<P, M> store::ArtifactStoreOneItemPreparation<P, M> for Gis2dOneItemPreparation<P, M>
where
    P: Clone + Send + Sync + 'static,
    M: protocol::Mutation<P> + Send + 'static,
    M::Diff: protocol::MutationDiff<P>,
{
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "GIS map retained preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "GIS map retained preparation lost its mutation owner".to_string())?;
        let inverse = mutation.inverse(base.get());
        let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "GIS map retained preparation lost its Store authority".to_string())?;
        let edit = gis2d_one_item_edit(mutation, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("GIS map retained preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️OneItemPreparation

/// 🕹️ `interactionSelect` args for a single-feature pick against the `"features"` domain's
/// `"feature"` granularity — the generic replacement for the deleted bespoke `setFeatureSelection`
/// action (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
fn select_feature_action_args(feature_id: &str) -> dsl::DslValue {
    let targets = dsl::os_pack::json!([{ "granularity": "feature", "id": feature_id }]).to_string();
    dsl::DslValue::object([
        ("domainId".to_string(), dsl::DslValue::String("features".to_string())),
        ("targets".to_string(), dsl::DslValue::String(targets)),
        ("merge".to_string(), dsl::DslValue::String("replace".to_string())),
        ("method".to_string(), dsl::DslValue::String("pick".to_string())),
    ])
}

/// 🖱️ On-demand GIS tiled-map context menu from feature hit-test and selection — grouped
/// disclosure via `Menu::of(registry)`; `organize_context_menu` (run automatically at the
/// `VcsArtifactApp::context_menu` funnel) sorts the declared `.group(...)` rows into
/// `RIBBON_PARENT_CATEGORIES` taxonomy order and inserts the pre-destructive separator itself.
///
/// 🕳️ `selected_ids` is always empty for now: `ArtifactEditor::context_menu` carries no
/// `InteractionView` (the SDK's B1 breaking pass threaded it only into `handle`/`copy_fragment`/
/// `cut_operations` — see `w3c-summary.md`'s own flagged gap on `open_context_menu`'s `selection`
/// field), so the "already selected" branch can never fire and `clearSelection` always renders
/// disabled until a future wave wires interaction state through here too.
async fn gis2d_context_menu_items(registry: &semio_framework_plugin::AppActionRegistry, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>, selected_ids: &[String]) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    let hits = surface.map_or(&[][..], |s| s.hits.as_slice());
    let feature = hits.iter().find(|h| h.domain == "feature" || h.domain == "position" || h.domain == "route");
    if let Some(feature) = feature {
        let kind = if feature.domain == "route" { "route" } else { "position" };
        let mut menu = Menu::of(registry)
            .await
            .action_args(INTERACTION_SELECT_ACTION_ID, select_feature_action_args(&feature.id))
            .await
            .action_args(
                "focusFeature",
                dsl::DslValue::object([("featureId".to_string(), dsl::DslValue::String(feature.id.clone())), ("featureKind".to_string(), dsl::DslValue::String(kind.to_string()))]),
            )
            .await;
        if kind == "position" {
            menu = menu.action_args("openSource", dsl::DslValue::object([("featureId".to_string(), dsl::DslValue::String(feature.id.clone()))])).await;
        }
        return menu.build().await;
    }
    let mut items = Menu::of(registry).await.action("selectAll").await.action("fitWorld").await.destructive("clearSelection").await.build().await;
    if let Some(clear) = items.iter_mut().find(|entry| entry.id == "clearSelection") {
        clear.disabled = selected_ids.is_empty().then_some(true);
    }
    items
}

impl ArtifactEditor for Gis2dPlayApp {
    type Snapshot = GisMapSnapshot;
    type Mutation = GisMapMutation;
    type Config = Gis2dConfig;
    type ConfigMutation = Gis2dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::gis2d::presence::Gis2dPresence;
    type PresenceMutation = crate::editor::gis2d::presence::Gis2dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Gis2dCommand;

    const DIALECT: Dialect = crate::artifacts::gismap::GISMAP_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GIS_MAP_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Gis2dOneItemPreparationFactory::<Self::Snapshot, Self::Mutation>::default()))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Gis2dOneItemPreparationFactory::<Self::Config, Self::ConfigMutation>::default()))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<Gis2dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.gis.gismap@1/*#editor",
        document_schema: "gis.map",
        factory: "Gis2dRetainedCommandJobFactory",
        factory_type: Gis2dRetainedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 64, 16_384, 7_500),
        tools: [
            "setActiveExample", "patchPositions", "patchRoutes", "patchRoute", "toggleLayerVisibility", "fitWorld", "setCamera", "setRenderMode", "setVectorStyle", "setLodMode", "focusFeature", "setLayerStrokeScale", "openSource", "setLocale",
            "proposeBoundsRegion",
        ]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller_id = registry.controller_id().to_string();
        registry.register(Gis2dRetainedCommandJobFactory::new(&controller_id))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !GIS2D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id || gis2d_retained_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
            return Err(Fault::from("gis2d-retained-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(BoundedArtifactCommandWork::new(tool_id, gis2d_retained_reduce, gis2d_retained_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
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
            Gis2dCommand::command_id,
            GIS2D_RETAINED_RAW_BYTES,
            GIS2D_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::gismap::spr::gis_map_envelope_decode_owner_bundle())
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::artifacts::gismap::spr::gis_map_document_store_owners())
    }

    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::artifacts::gismap::spr::gis_map_document_store_initialization_job(envelope, operation, generation))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::gis2d::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> GisMapSnapshot {
        crate::artifacts::gismap::schema::default_document()
    }

    /// 🔌️ `features:in`/`map:out` (WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe) plus the
    /// implicit document ports.
    fn io() -> Option<AppIo> {
        Some(gis2d_io())
    }

    // 🧬️ No `whole_document_operation` override: per the taxonomy's banned-vocabulary rule, whole-
    // document replace has no in-history mutation — `document:in` falls back to the trait's own
    // default (`None`) and returns `MediaError::NotImplemented`. `setActiveExample` (this app's real
    // document-replacing gesture) goes through `positions_operations`/`routes_operations`/
    // `regions_operations` instead, diffing into batched create/delete/replace-data operations.

    /// 🎞️ `map:out` (see `gis2d_map_media` in `🔖️Io` below) plus the inherited
    /// `document:out` default (the pack of `doc.snapshot`, replicated inline — overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, GisMapSnapshot>) -> Result<Media, MediaError> {
        match port {
            "map:out" => Ok(gis2d_map_media(doc.snapshot)),
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `features:in` normalizes an incoming `{positions,routes,regions}` descriptor into granular
    /// add/patch/remove operations against every collection (a generic vector-features sink — not
    /// pinned to `2d.map`, so a `draw`/another `gis2d`'s producer both work) plus the inherited
    /// `document:in` default (replicated inline for the same reason as `export_media`).
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, GisMapSnapshot>) -> Result<Emit<GisMapMutation, Gis2dConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "features:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "features:in only accepts a Structured JSON payload".into()));
                };
                let incoming = gis_map_document_from_descriptor_json(json);
                let document = doc.snapshot;
                let mut operations = positions_operations(&document.positions, &incoming.positions);
                operations.extend(routes_operations(&document.routes, &incoming.routes));
                operations.extend(regions_operations(&document.regions, &incoming.regions));
                Ok(Emit::mutations(operations))
            }
            "document:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let snapshot = <GisMapSnapshot as ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                match Self::whole_document_operation(snapshot) {
                    Some(operation) => Ok(Emit::mutations(vec![operation])),
                    None => Err(MediaError::NotImplemented),
                }
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn command_id(command: &Gis2dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Gis2dCommand` — React/wgpu still speak the
    /// stringly `{action,args}` wire; this is the typed-command bridge until those call sites send
    /// `OpBinary` bytes directly.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        let args = args.map(Value::from).unwrap_or(Value::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let string_list = |key: &str| -> Vec<String> { args.get(key).and_then(|value| value.as_array()).map(|rows| rows.iter().filter_map(|row| row.as_str().map(str::to_string)).collect()).unwrap_or_default() };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_f64())) };
        match action {
            "setActiveExample" => Ok(Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_arg(&["exampleId", "example_id", "value"]).unwrap_or_default() })),
            "patchPositions" => {
                Ok(Gis2dCommand::PatchPositions(patch_positions::PatchPositions { positions_json: str_arg(&["positionsJson", "positions_json"]).or_else(|| args.get("positions").map(ToString::to_string)).unwrap_or_else(|| "[]".into()) }))
            }
            "patchRoutes" => Ok(Gis2dCommand::PatchRoutes(patch_routes::PatchRoutes {
                route_ids: {
                    let mut ids = string_list("routeIds");
                    if ids.is_empty() {
                        ids = string_list("route_ids");
                    }
                    ids
                },
                field: str_arg(&["field"]).unwrap_or_default(),
                value: str_arg(&["value"]).unwrap_or_default(),
            })),
            "patchRoute" => Ok(Gis2dCommand::PatchRoute(patch_route::PatchRoute { route_id: str_arg(&["routeId", "route_id"]).unwrap_or_default(), field: str_arg(&["field"]).unwrap_or_default(), value: str_arg(&["value"]).unwrap_or_default() })),
            "toggleLayerVisibility" => Ok(Gis2dCommand::ToggleLayerVisibility(toggle_layer_visibility::ToggleLayerVisibility { layer_id: str_arg(&["layerId", "layer_id"]).unwrap_or_default() })),
            "fitWorld" => Ok(Gis2dCommand::FitWorld(fit_world::FitWorld {})),
            "setCamera" => {
                let camera_json = str_arg(&["cameraJson", "camera_json"]).or_else(|| args.get("camera").map(|value| if value.is_string() { value.as_str().unwrap_or("{}").to_string() } else { value.to_string() })).unwrap_or_else(|| "{}".into());
                Ok(Gis2dCommand::SetCamera(set_camera::SetCamera { camera_json }))
            }
            "setRenderMode" => Ok(Gis2dCommand::SetRenderMode(set_render_mode::SetRenderMode { value: str_arg(&["value", "renderMode", "render_mode"]).unwrap_or_default() })),
            "setVectorStyle" => Ok(Gis2dCommand::SetVectorStyle(set_vector_style::SetVectorStyle { value: str_arg(&["value", "vectorStyle", "vector_style"]).unwrap_or_default() })),
            "setLodMode" => Ok(Gis2dCommand::SetLodMode(set_lod_mode::SetLodMode { value: str_arg(&["value", "lodMode", "lod_mode"]).unwrap_or_default() })),
            "focusFeature" => {
                Ok(Gis2dCommand::FocusFeature(focus_feature::FocusFeature { feature_id: str_arg(&["featureId", "feature_id"]).unwrap_or_default(), feature_kind: str_arg(&["featureKind", "feature_kind"]).unwrap_or_else(|| "position".into()) }))
            }
            "setLayerStrokeScale" => Ok(Gis2dCommand::SetLayerStrokeScale(set_layer_stroke_scale::SetLayerStrokeScale { layer_id: str_arg(&["layerId", "layer_id"]).unwrap_or_default(), value: f64_arg(&["value"]).unwrap_or(1.0) })),
            "setLocale" => Ok(Gis2dCommand::SetLocale(set_locale::SetLocale { value: str_arg(&["value", "locale"]).unwrap_or_default() })),
            "openSource" => Ok(Gis2dCommand::OpenSource(open_source::OpenSource { feature_id: str_arg(&["featureId", "feature_id"]).unwrap_or_default() })),
            "proposeBoundsRegion" => Ok(Gis2dCommand::ProposeBoundsRegion(propose_bounds_region::ProposeBoundsRegion {})),
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    fn handle(
        command: &Gis2dCommand,
        doc: &ArtifactView<'_, GisMapSnapshot>,
        cfg: &ConfigView<'_, Gis2dConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<GisMapMutation, Gis2dConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🧮️ Empty — gis2d's `Config` is session view state (camera/render/layer visibility/…), not a
    /// user-facing settings record; `ConfigSpec::empty()` (the trait default) is correct as-is.
    fn config_spec() -> semio_framework_plugin::ConfigSpec {
        semio_framework_plugin::ConfigSpec::default()
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, Gis2dConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let config = cfg.snapshot;
        let labels = gis2d_labels(config);
        match body_key {
            map::GIS2D_PLAY_BODY_COMPOSITE => map::render(doc.snapshot, config).map(semio_framework_plugin::built_to_component_tree),
            document_panel::GIS2D_PLAY_BODY_DOCUMENT => document_panel::render(config, labels).map(semio_framework_plugin::built_to_component_tree),
            catalogue_panel::GIS2D_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels).map(semio_framework_plugin::built_to_component_tree),
            inspection_panel::GIS2D_PLAY_BODY_INSPECTION => inspection_panel::render(config, labels).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_measures(_doc: &ArtifactView<'_, GisMapSnapshot>, cfg: &ConfigView<'_, Gis2dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        HashMap::from([(map::GIS2D_PLAY_WINDOW_MAIN.into(), map::window_measures(config, gis2d_labels(config)))])
    }

    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        _doc: &ArtifactView<'_, GisMapSnapshot>,
        _cfg: &ConfigView<'_, Gis2dConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        semio_framework_plugin::resolve_ready(async { gis2d_context_menu_items(registry, request.surface.as_ref(), &[]).await })
    }
}
//#endregion 🔖️Gis2dPlayApp

//#region 🔖️Manifest
pub fn create_gis2d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::gismap::GISMAP_DIALECT).document(["semio", "gis", "2d"])
            .artifact_kind(artifact_kind())
            // 🔌️ Typed workflow ports (WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe) — same
            // constructor fns `gis2d_io()` embeds, so `AppIo.all_ports()` and these declarations can
            // never drift apart. `map:out`'s `2d.map` kind is declared above; `features:in` pins no kind.
            .media_input(gis2d_features_in_port())
            .media_output(gis2d_map_out_port())
            .icon_id("gis2d")
            .mode_def(edit::definition())
            .default_mode_id(edit::GIS2D_PLAY_MODE_EDIT)
            .window_kind_def(map::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            // 🕹️ The framework-owned "features" interaction domain (ticket
            // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — covers both the document tree's
            // layer selection (granularity "layer") and the map's feature pick/marquee selection
            // (granularity "feature"); auto-injects interactionSelect/interactionHover/clearSelection/
            // selectAll/setSelectionMode/setInteractionGranularity, replacing every deleted bespoke
            // setSelection/setFeatureSelection/setHover/setSelectionMethod/setSelectionMode/
            // clearSelection/selectAll action below.
            .interaction(InteractionDefinition {
                id: "features".into(),
                label: LocalizedLabel::native("Features", "Objekte"),
                granularities: vec![
                    GranularityDefinition { id: "layer".into(), label: LocalizedLabel::native("Layer", "Ebene"), icon_id: "layers".into() },
                    GranularityDefinition { id: "feature".into(), label: LocalizedLabel::native("Feature", "Objekt"), icon_id: "crosshair".into() },
                ],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle, SelectionMethod::Lasso],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(map::GIS2D_PLAY_WINDOW_MAIN, vec![InteractionRef::new("features")])
            // ✏️ Mutation actions — flow through the document store with true inverses. `setActiveExample`
            // replaces document content by diffing every collection into batched create/delete/
            // replace-data operations (never a whole-document snapshot swap — that vocabulary is
            // retired by the taxonomy), so it is a Mutation, not a View action.
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .mutation("patchPositions", LocalizedLabel::native("Patch Positions", "Positionen aktualisieren"))
            .mutation("patchRoutes", LocalizedLabel::native("Patch Routes", "Routen aktualisieren"))
            .mutation("patchRoute", LocalizedLabel::native("Patch Route", "Route aktualisieren"))
            // 👁️ View actions — mutate ephemeral config state (camera, render config, layer
            // visibility, stroke weights), never the document.
            .view_action("toggleLayerVisibility", LocalizedLabel::native("Toggle Layer Visibility", "Ebenensichtbarkeit umschalten"))
            .action_with(ActionDefinition { category: Some("view".into()), ..ActionDefinition::bounded_catalog("fitWorld", LocalizedLabel::native("Fit World", "Welt einpassen"), ActionKind::View) })
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .view_action("setRenderMode", LocalizedLabel::native("Set Render Mode", "Darstellungsmodus festlegen"))
            .view_action("setVectorStyle", LocalizedLabel::native("Set Vector Style", "Vektorstil festlegen"))
            .view_action("setLodMode", LocalizedLabel::native("Set LOD Mode", "LOD-Modus festlegen"))
            .action_with(ActionDefinition { category: Some("view".into()), ..ActionDefinition::bounded_catalog("focusFeature", LocalizedLabel::native("Focus Feature", "Objekt fokussieren"), ActionKind::View) })
            .view_action("setLayerStrokeScale", LocalizedLabel::native("Set Layer Stroke Scale", "Ebenenstrichstärke festlegen"))
            // 🌐️ Shell action — opens the picked feature's source URL through the host.
            .action_with(ActionDefinition { category: Some("open".into()), ..ActionDefinition::bounded_catalog("openSource", LocalizedLabel::native("Open Source", "Quelle öffnen"), ActionKind::Shell) })
            .view_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"))
            // 💡️ Shell action — asks the host to open its own ephemeral inference port and offer a
            // reviewable bounds region. It never writes the document; only the hub's server-stamped
            // approval command can.
            .action_with(ActionDefinition { category: Some("inference".into()), ..ActionDefinition::bounded_catalog("proposeBoundsRegion", LocalizedLabel::native("Propose Bounds Region", "Begrenzungsregion vorschlagen"), ActionKind::Shell) })
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchPositions", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchRoutes", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchRoute", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleLayerVisibility", InteractiveJobClassification::Migrated)
            .action_interactive_job("fitWorld", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setRenderMode", InteractiveJobClassification::Migrated)
            .action_interactive_job("setVectorStyle", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLodMode", InteractiveJobClassification::Migrated)
            .action_interactive_job("focusFeature", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLayerStrokeScale", InteractiveJobClassification::Migrated)
            .action_interactive_job("openSource", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLocale", InteractiveJobClassification::Migrated)
            .action_interactive_job("proposeBoundsRegion", InteractiveJobClassification::Migrated)
            // 📝️ Argument schemas for the discrete-choice actions so the command palette can stage them
            // and the registry validates the vocabulary. The arg id matches the key each handler reads.
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new("reuse-map", LocalizedLabel::native("Reuse Map", "Karte wiederverwenden")),
                ]).default_value("reuse-map"),
            ])
            .action_args("setRenderMode", vec![
                ActionArgDef::select("value", LocalizedLabel::native("Render Mode", "Darstellungsmodus"), vec![
                    ActionArgOption::new("image", LocalizedLabel::native("Image", "Bild")),
                    ActionArgOption::new("vector", LocalizedLabel::native("Vector", "Vektor")),
                    ActionArgOption::new("combined", LocalizedLabel::native("Combined", "Kombiniert")),
                ]).default_value("combined"),
            ])
            .action_args("setVectorStyle", vec![
                ActionArgDef::select("value", LocalizedLabel::native("Vector Style", "Vektorstil"), vec![
                    ActionArgOption::new("colored", LocalizedLabel::native("Colored", "Farbig")),
                    ActionArgOption::new("figureGround", LocalizedLabel::native("Figure Ground", "Figur-Grund")),
                    ActionArgOption::new("invertedFigure", LocalizedLabel::native("Inverted Figure", "Invertierte Figur")),
                ]).default_value("colored"),
            ])
            .action_args("setLodMode", vec![
                ActionArgDef::select("value", LocalizedLabel::native("LOD Mode", "LOD-Modus"), map::options::lod_mode::lod_arg_options()).default_value(framework_surface::tiled_map::GIS_MAP_LOD_MODE_AUTOMATIC),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .config(Gis2dPlayApp::config_spec())
            .io(gis2d_io())
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder::build_definition` has no `.example(...)`/
            // `.workflow(...)` — the old `"reuse-map"` app-level example registration and the no-op
            // `.workflow("gis2d", …)` call are dropped here (not silently: reported in the migration
            // notes). The subset's own `📚️examples/🎬️demo` facet is the modern replacement surface.
            .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated)
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type Gis2dApp = VcsArtifactApp<EditorApp<Gis2dPlayApp>>;

    pub fn app() -> Gis2dApp {
        new_app::<EditorApp<Gis2dPlayApp>>()
    }

    /// ✏️ Adapts `create_gis2d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `testkit::assert_declared_actions_bridge_to_commands` still expects —
    /// framework testkit gap, not modifiable here.
    pub fn gis2d_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_gis2d_app(), examples: Vec::new() }
    }

    /// 🧬️ A wrapper carrying the real registry so kind discipline (View/Shell-emits-operations rejection) runs.
    pub fn app_with_registry() -> Gis2dApp {
        new_app_with_registry::<EditorApp<Gis2dPlayApp>>(gis2d_app_manifest_for_testkit)
    }

    pub fn dispatch(app: &mut Gis2dApp, command: Gis2dCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut Gis2dApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    pub fn main_window_measures(app: &mut Gis2dApp) -> Vec<WindowMeasure> {
        app.window_measures().get(map::GIS2D_PLAY_WINDOW_MAIN).cloned().unwrap_or_default()
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::gis2d::testkit::{app, app_with_registry, gis2d_app_manifest_for_testkit, render};
    use semio_framework_plugin::{ContextMenuRequest, EditorApp, PluginApp, VcsArtifactApp};

    fn gis_map_envelope_wire() -> Vec<u8> {
        use store::ArtifactPack;

        let snapshot = crate::artifacts::gismap::schema::empty_gis_map_snapshot();
        let snapshot_pack = snapshot.encode_pack();
        let snapshot_hex = snapshot_pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        let wire = serde_json::to_vec(&serde_json::json!({
            "schema": GIS_MAP_SCHEMA,
            "id": "gis-map-live-load",
            "vcs": {
                "initialSnapshot": snapshot_hex,
                "edits": [],
                "changes": [],
                "checkpoints": [],
                "alternatives": []
            },
            "editMessages": [],
            "conflicts": []
        }))
        .expect("schema-first GIS fixture envelope");
        let envelope = store::create_document_envelope(GIS_MAP_SCHEMA, "gis-map-live-load", snapshot, None);
        let mut retirement = crate::artifacts::gismap::spr::gis_map_envelope_decode_owner_bundle().retire_envelope(envelope);
        for _ in 0..100_000 {
            match retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("GIS fixture envelope retirement") {
                store::SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    drop(retirement);
                    return wire;
                }
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
                }
                store::SnapshotRetirementStep::Blocked => panic!("unshared GIS fixture envelope retirement blocked"),
            }
        }
        panic!("GIS fixture envelope retirement did not reach terminal")
    }

    fn admit_gis_map_envelope(app: &mut VcsArtifactApp<EditorApp<Gis2dPlayApp>>, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
        let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
        let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("GIS live envelope ingress credits");
        for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
            let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
            bytes[..chunk.len()].copy_from_slice(chunk);
            let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded GIS live envelope page");
            app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("GIS live envelope page admission failed: {fault}"));
        }
        assert!(app.seal_artifact_envelope_ingress(handle).expect("GIS live envelope seal/submit"));
        handle
    }

    fn drive_gis_map_live_load(app: &mut VcsArtifactApp<EditorApp<Gis2dPlayApp>>, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
        for _ in 0..100_000 {
            app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one GIS live maintenance turn");
            let poll = app.advance_artifact_envelope_load(handle).expect("GIS live load advancement");
            if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
                return poll;
            }
            std::thread::yield_now();
        }
        panic!("GIS live envelope load did not reach terminal")
    }

    #[semio_framework_async_macros::async_test]
    async fn gis_map_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed() {
        let mut app = app();
        let base_generation = app.artifact_generation_now();
        let handle = admit_gis_map_envelope(&mut app, &gis_map_envelope_wire());
        assert_eq!(handle.generation, base_generation);
        assert_eq!(drive_gis_map_live_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
        assert_eq!(app.artifact_generation_now().0, base_generation.0 + 1);
        assert!(app.acknowledge_artifact_store_replacement(handle).expect("first exact GIS load acknowledgement"));
        assert!(!app.acknowledge_artifact_store_replacement(handle).expect("duplicate GIS load acknowledgement is a no-op"));
    }

    #[semio_framework_async_macros::async_test]
    async fn gis_map_live_envelope_cancel_closes_retained_pages_without_publication() {
        let mut app = app();
        let base_generation = app.artifact_generation_now();
        let wire = gis_map_envelope_wire();
        let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
        let handle = app.begin_artifact_envelope_ingress(pages, wire.len()).expect("cancelled GIS ingress credits");
        let first = &wire[..wire.len().min(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)];
        let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..first.len()].copy_from_slice(first);
        let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, first.len()).expect("cancelled GIS first page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("cancelled GIS page admission failed: {fault}"));
        app.cancel_artifact_envelope_load(handle).expect("cancel exact GIS ingress");
        assert_eq!(drive_gis_map_live_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
        assert_eq!(app.artifact_generation_now(), base_generation);
    }

    //#region 🔖️CommandSurface
    /// 🎯️ One value per `app_commands!` row, in row order — the wire-law loop below and the id
    /// uniqueness check both run off this list, so a new row that forgets to appear here fails the
    /// coverage assertion.
    fn every_command() -> Vec<Gis2dCommand> {
        vec![
            Gis2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "reuse-map".into() }),
            Gis2dCommand::PatchPositions(patch_positions::PatchPositions { positions_json: r#"[{"id":"p1","lon":1.0,"lat":2.0}]"#.into() }),
            Gis2dCommand::PatchRoutes(patch_routes::PatchRoutes { route_ids: vec!["r1".into(), "r2".into()], field: "label".into(), value: "Home".into() }),
            Gis2dCommand::PatchRoute(patch_route::PatchRoute { route_id: "r1".into(), field: "label".into(), value: "Home".into() }),
            Gis2dCommand::ToggleLayerVisibility(toggle_layer_visibility::ToggleLayerVisibility { layer_id: "water".into() }),
            Gis2dCommand::FitWorld(fit_world::FitWorld {}),
            Gis2dCommand::SetCamera(set_camera::SetCamera { camera_json: r#"{"x":0,"y":0,"zoom":1}"#.into() }),
            Gis2dCommand::SetRenderMode(set_render_mode::SetRenderMode { value: "vector".into() }),
            Gis2dCommand::SetVectorStyle(set_vector_style::SetVectorStyle { value: "colored".into() }),
            Gis2dCommand::SetLodMode(set_lod_mode::SetLodMode { value: "automatic".into() }),
            Gis2dCommand::FocusFeature(focus_feature::FocusFeature { feature_id: "p1".into(), feature_kind: "position".into() }),
            Gis2dCommand::SetLayerStrokeScale(set_layer_stroke_scale::SetLayerStrokeScale { layer_id: "roads".into(), value: 1.5 }),
            Gis2dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            Gis2dCommand::OpenSource(open_source::OpenSource { feature_id: "p1".into() }),
            Gis2dCommand::ProposeBoundsRegion(propose_bounds_region::ProposeBoundsRegion {}),
        ]
    }

    /// 🏷️ The wire keyword each row prints under — the kebab `as` literal, independent of the camelCase
    /// manifest action id. Pinned so a reordered/renamed row is caught here, not in production.
    const WIRE_KEYWORDS: &[&str] =
        &["active-example", "patch-positions", "patch-routes", "patch-route", "toggle-layer-visibility", "fit-world", "camera", "render-mode", "vector-style", "lod-mode", "focus-feature", "layer-stroke-scale", "locale", "open-source", "propose-bounds-region"];

    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique_and_cover_every_row() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(Gis2dCommand::command_id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 15, "every Gis2dCommand row must be covered by every_command()");
    }

    #[test]
    fn retained_factory_owns_every_migrated_command_and_exact_publication_lane() {
        assert_eq!(<Gis2dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::TOOL_IDS, GIS2D_RETAINED_TOOL_IDS);
        assert_eq!(<Gis2dRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS, GIS2D_RETAINED_PUBLICATION_CONTRACTS);
        assert_eq!(GIS2D_RETAINED_PUBLICATION_CONTRACTS.iter().find(|row| row.tool_id == "setActiveExample").map(|row| row.lanes), Some(&[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config][..]));
        assert_eq!(GIS2D_RETAINED_PUBLICATION_CONTRACTS.iter().find(|row| row.tool_id == "setCamera").map(|row| row.lanes), Some(&[ArtifactToolPublicationLane::Config][..]));
        assert_eq!(GIS2D_RETAINED_PUBLICATION_CONTRACTS.iter().find(|row| row.tool_id == "openSource").map(|row| row.lanes), Some(&[ArtifactToolPublicationLane::HostOnly][..]));
        assert_eq!(GIS2D_RETAINED_PUBLICATION_CONTRACTS.iter().find(|row| row.tool_id == "proposeBoundsRegion").map(|row| row.lanes), Some(&[ArtifactToolPublicationLane::HostOnly][..]));
    }

    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_text_and_binary_under_its_declared_wire_keyword() {
        assert_eq!(every_command().len(), WIRE_KEYWORDS.len());
        for (command, keyword) in every_command().iter().zip(WIRE_KEYWORDS) {
            store::os_store::test_support::assert_op_text_binary_equivalence(command);
            let printed = protocol::OpText::print_op(command);
            assert!(printed == *keyword || printed.starts_with(&format!("{keyword} ")), "row {} printed {printed:?}, expected the {keyword:?} wire keyword", command.command_id());
        }
    }

    /// 🧷️ `PatchRoutes`' empty-`Vec` shape round-trips identically to its non-empty shape — the one
    /// `Vec`-carrying optional-field case left after the interaction-mechanism migration deleted every
    /// other optional-field row (`setSelection`/`setFeatureSelection`/`clearSelection`/`selectAll`).
    #[semio_framework_async_macros::async_test]
    async fn patch_routes_empty_route_ids_round_trips_text_and_binary() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&Gis2dCommand::PatchRoutes(patch_routes::PatchRoutes { route_ids: Vec::new(), field: "label".into(), value: String::new() }));
    }

    /// 🎯️ Every app-declared action must bridge through `command_from_action` and round-trip
    /// `command_id`. Uses the framework's own harness, which stages each action's declared args and
    /// knows the framework-injected ids to skip (`undo`/`copy`/`recordTutorial`/…).
    #[semio_framework_async_macros::async_test]
    async fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<EditorApp<Gis2dPlayApp>>(gis2d_app_manifest_for_testkit);
        assert!(Gis2dPlayApp::command_from_action("noSuchAction", None).is_err());
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️Manifest
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let definition = create_gis2d_app().definition;
        assert_eq!(definition.modes.len(), 1);
        assert_eq!(definition.window_kinds.len(), 1);
        // 🧷️ The framework injects its own panel tabs on top of the app's three, so assert the app's
        // own tabs are stitched in rather than pinning a total.
        for body_key in [document_panel::GIS2D_PLAY_BODY_DOCUMENT, catalogue_panel::GIS2D_PLAY_BODY_CATALOGUE, inspection_panel::GIS2D_PLAY_BODY_INSPECTION] {
            assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
        }
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == crate::artifacts::gismap::GISMAP_DIALECT.artifact_kind));
    }

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_falls_back_to_a_text_node() {
        let mut app = app();
        assert!(render(&mut app, "gis2d.play.nope").contains("Unknown body"));
    }
    //#endregion 🔖️Manifest

    //#region 🔖️Media
    #[semio_framework_async_macros::async_test]
    async fn export_media_map_out_produces_a_2d_map_structured_payload() {
        let app = app();
        let document = app.snapshot().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let media = semio_framework_plugin::resolve_ready(Gis2dPlayApp::export_media("map:out", &doc)).expect("map:out export");
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "2d.map");
        assert!(json.contains("positions"));
    }

    #[semio_framework_async_macros::async_test]
    async fn import_media_features_in_adds_new_positions_as_operations() {
        let app = app();
        let document = app.snapshot().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let incoming = json!({ "positions": [{ "id": "imported-1", "lon": 1.0, "lat": 2.0 }] }).to_string();
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "2d.map".into(), json: incoming } };
        let emit = Gis2dPlayApp::import_media("features:in", &media, &doc).expect("features:in import");
        assert!(emit.artifact_mutations.iter().any(|operation| matches!(operation, GisMapMutation::CreatePosition(payload) if payload.item.id == "imported-1")));
    }

    #[semio_framework_async_macros::async_test]
    async fn media_ports_declare_features_in_and_map_out() {
        let app = Gis2dPlayApp;
        let ports = Gis2dPlayApp::media_ports();
        assert!(ports.iter().any(|port| port.id == "features:in"));
        assert!(ports.iter().any(|port| port.id == "map:out"));
    }

    /// 🧭️ Relocated from the artifact's `⚙️engine` tests (ticket
    /// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) alongside `gis2d_io`/`gis2d_map_media`.
    #[semio_framework_async_macros::async_test]
    async fn gis2d_io_declares_the_features_in_and_map_out_ports() {
        let io = gis2d_io();
        assert_eq!(io.document_schema, GIS_MAP_SCHEMA);
        assert_eq!(io.artifact.id, crate::artifacts::gismap::GISMAP_DIALECT.artifact_kind);
        let ports = io.all_ports();
        assert!(ports.iter().any(|port| port.id == "features:in" && port.direction == semio_framework_plugin::MediaPortDirection::In));
        let map_out = ports.iter().find(|port| port.id == "map:out").expect("map:out declared");
        assert_eq!(map_out.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(map_out.kind_id.as_deref(), Some(crate::artifacts::gismap::GISMAP_DIALECT.artifact_kind));
    }

    #[semio_framework_async_macros::async_test]
    async fn gis2d_map_media_exports_the_document_descriptor() {
        let document = crate::artifacts::gismap::schema::default_document();
        let media = gis2d_map_media(&document);
        let semio_framework_plugin::MediaPayload::Structured { schema, json } = media.payload else {
            panic!("expected a structured map:out payload");
        };
        assert_eq!(schema, "2d.map");
        assert!(json.contains("positions"));
    }
    //#endregion 🔖️Media

    //#region 🔖️ContextMenu
    /// 🖱️ Grouped disclosure: the empty-canvas context menu (no feature under the pointer) stays
    /// within the row budget and keeps the known destructive `clearSelection` last, matching the
    /// canonical migration pattern.
    #[semio_framework_async_macros::async_test]
    async fn context_menu_stays_within_budget_and_keeps_clear_selection_destructive_last() {
        let mut app = app_with_registry();
        let request = ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "gis2dMap".into(), args: None }, surface: None, window_instance_id: None, point: None };
        let menu = app.context_menu(&request);
        assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
        let last = menu.last().expect("empty-canvas context menu should not be empty");
        assert_eq!(last.id, "clearSelection", "known destructive clearSelection must be last: {menu:?}");
        assert_eq!(last.destructive, Some(true), "clearSelection must be marked destructive: {menu:?}");
    }
    //#endregion 🔖️ContextMenu
}
//#endregion 🧪️Tests
