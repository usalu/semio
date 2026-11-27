//! ⛰️ GIS 3D terrain editor: `ArtifactEditor` implementation, typed command dispatch, media I/O, and app manifest.

use crate::editor::gis3d::commands::{contours, edit_elevation, elevation_source, example, exaggeration, grade_region, view};
use crate::editor::gis3d::modes::view as view_mode;
use crate::editor::gis3d::modes::view::windows::terrain;
use crate::editor::gis3d::terrain_data;
use crate::standards::v1::subsets::any::schema::mutations::GisTerrainMutation;
use crate::standards::v1::subsets::any::io::text::snapshot::default_terrain_document;
use crate::{GisTerrainSnapshot, GIS_3D_TERRAIN_SCHEMA};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_plugin::app::InteractionView;
use semio_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_plugin::AppIo;
use semio_plugin::AppOperationContext;
use semio_plugin::ArtifactEditor;
use semio_plugin::ArtifactOwnedToolJobRequest;
use semio_plugin::ArtifactToolFactoryRegistry;
use semio_plugin::ArtifactToolPublicationContract;
use semio_plugin::ArtifactToolPublicationLane;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_plugin::DraftView;
use semio_plugin::Editor;
use semio_plugin::EditorApp;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_plugin::GranularityDefinition;
use semio_plugin::HierarchyProvider;
use semio_plugin::HoverSpec;
use semio_plugin::InteractionDefinition;
use semio_plugin::InteractionRef;
use semio_ui_locale::Label;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::Media;
use semio_plugin::MediaClass;
use semio_plugin::MediaError;
use semio_plugin::MediaForm;
use semio_plugin::MediaPayload;
use semio_plugin::MediaType;
use semio_plugin::MergeMode;
use semio_plugin::NoConfig;
use semio_plugin::NoConfigMutation;
use semio_plugin::NoDraft;
use semio_plugin::NoDraftMutation;
use semio_plugin::SelectionMethod;
use semio_plugin::SelectionMode;
use semio_plugin::SelectionSpec;
use semio_framework_pack_json::{parse as parse_json, to_json_string, JsonMemberPolicy};
use pack::snapshot::ArtifactPack;
use semio_framework_2d_compute::EngineHandles;

#[path = "📥️media-import/🦀️.rs"]
mod media_import;
#[path = "📤️media-export/🦀️.rs"]
mod media_export;

//#region 🔖️Constants
pub const GIS3D_PLAY_APP_ID: &str = "gis3d-play";
const GIS_ELEVATION_GRID_SCHEMA: &str = "gis.elevation-grid";
const GIS_ELEVATION_MEDIA_TYPE: MediaType = MediaType { class: MediaClass::Data, form: MediaForm::Value };
const GIS_ELEVATION_JSON_MAX_BYTES: usize = 64 * 1024;
const GIS_MAP_MEDIA_TYPE: MediaType = MediaType { class: MediaClass::TwoD, form: MediaForm::Vector };
//#endregion 🔖️Constants

//#region 🔖️Io
/// 🧭️ Declares the Semio document medium and the typed map, elevation, and mesh workflow ports.
pub fn gis3d_io() -> AppIo {
    AppIo {
        artifact_schema: GIS_3D_TERRAIN_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        ports: vec![
            gis3d_map_in_port(),
            gis3d_elevation_port("elevation:in", semio_plugin::MediaPortDirection::In, semio_framework::PortMultiplicity::One),
            gis3d_elevation_port("elevation:out", semio_plugin::MediaPortDirection::Out, semio_framework::PortMultiplicity::Many),
            gis3d_scene_out_port(),
        ],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_plugin::ArtifactPresentation { id: crate::GISTERRAIN_DIALECT.artifact_kind.into(), name: "GIS Terrain".into(), dimension: "3d".into(), component_kind: "gisterrain".into() },
    }
}

/// 🔌️ `map:in` — a `2d.map` producer (gis2d's `map:out`) feeding an overlay pin layer into this
/// terrain (see `GisTerrainSnapshot::imported_map`). `One`/optional: exactly one map may
/// be draped onto a terrain at a time, and a terrain with no upstream edge is valid.
pub fn gis3d_map_in_port() -> semio_plugin::MediaPortSpec {
    semio_plugin::MediaPortSpec {
        id: "map:in".into(),
        label: LocalizedLabel::native("Map", "Karte"),
        direction: semio_plugin::MediaPortDirection::In,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        kind_id: Some(semio_s_artifact_gis_gismap::GISMAP_DIALECT.artifact_kind.into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}

fn gis3d_elevation_port(id: &str, direction: semio_plugin::MediaPortDirection, multiplicity: semio_framework::PortMultiplicity) -> semio_plugin::MediaPortSpec {
    semio_plugin::MediaPortSpec {
        id: id.into(),
        label: LocalizedLabel::native("Elevation source", "Höhenquelle"),
        direction,
        media_type: GIS_ELEVATION_MEDIA_TYPE,
        kind_id: Some(GIS_ELEVATION_GRID_SCHEMA.into()),
        required: false,
        multiplicity,
    }
}

/// 🎭️ Publishes terrain geometry to the workflow mesh media class.
pub fn gis3d_scene_out_port() -> semio_plugin::MediaPortSpec {
    semio_plugin::MediaPortSpec {
        id: "scene:out".into(),
        label: LocalizedLabel::native("Scene", "Szene"),
        direction: semio_plugin::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        kind_id: None,
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🎞️ Publishes tessellated terrain using the shared workflow mesh document carrier.
pub fn gis3d_scene_media(document: &GisTerrainSnapshot) -> Result<Media, MediaError> {
    use semio_framework_value::{DslValue, ToValue};
    let surface = crate::schema::imported_map::gis_terrain_mesh_buffers(document)
        .map_err(|error| MediaError::Payload("scene:out".into(), error.into()))?;
    let mesh = semio_plugin::MeshData {
        positions: surface.positions,
        normals: surface.normals,
        indices: surface.indices,
        uvs: surface.uvs,
        ..Default::default()
    };
    let schema = "mesh.document";
    let payload = DslValue::object([("schema".into(), schema.to_value()), ("mesh".into(), mesh.to_value())]);
    Ok(Media {
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        payload: MediaPayload::Structured {
            schema: schema.into(),
            json: semio_framework_pack_json::to_json_string(&payload),
        },
    })
}

fn gis3d_elevation_media(document: &GisTerrainSnapshot) -> Result<Media, MediaError> {
    use semio_framework_value::ToValue;
    let source = document.elevation_source.as_ref().filter(|source| source.is_valid())
        .ok_or_else(|| MediaError::Payload("elevation:out".into(), "no authored elevation source".into()))?;
    Ok(Media {
        media_type: GIS_ELEVATION_MEDIA_TYPE,
        payload: MediaPayload::Intrinsic { schema: GIS_ELEVATION_GRID_SCHEMA.into(), value: source.to_value() },
    })
}
//#endregion 🔖️Io

//#region 🔖️Commands
semio_plugin::app_commands! {
    /// 🎯️ `Gis3dPlayApp::Command` — the SOLE dispatch surface for gis3d's own behavior, covering every
    /// action `create_gis3d_app` declares. Row order is the binary variant ordinal: appending is safe,
    /// reordering is a wire-format break.
    pub enum Gis3dCommand for GisTerrainSnapshot, GisTerrainMutation, NoConfig, NoConfigMutation {
        "setExaggeration" as "exaggeration" => set_exaggeration::SetExaggeration,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "clearElevationSource" as "clear-elevation-source" => clear_elevation_source::ClearElevationSource,
        "editElevation" as "edit-elevation" => edit_elevation::EditElevation,
        "addOverlayPosition" as "add-overlay-position" => add_overlay_position::AddOverlayPosition,
        "removeOverlayFeature" as "remove-overlay-feature" => remove_overlay_feature::RemoveOverlayFeature,
        "clearOverlays" as "clear-overlays" => clear_overlays::ClearOverlays,
        "importElevationGrid" as "import-elevation-grid" => import_elevation::ImportElevationGrid,
        "importElevationGridFile" as "import-elevation-grid-file" => import_elevation_file::ImportElevationGridFile,
        "editOverlayShape" as "edit-overlay-shape" => edit_overlay_shape::EditOverlayShape,
        "moveOverlayPosition" as "move-overlay-position" => move_overlay_position::MoveOverlayPosition,
        "createElevationGrid" as "create-elevation-grid" => create_elevation_grid::CreateElevationGrid,
        "exportElevationGrid" as "export-elevation-grid" => export_elevation_grid::ExportElevationGrid,
        "gradeTerrainRegion" as "grade-terrain-region" => grade_region::GradeTerrainRegion,
        "importGeoJson" as "import-geojson" => geojson::ImportGeoJson,
        "importGeoJsonFile" as "import-geojson-file" => import_geojson_file::ImportGeoJsonFile,
        "exportGeoJson" as "export-geojson" => export_geojson::ExportGeoJson,
        "generateTerrainContours" as "generate-terrain-contours" => contours::GenerateTerrainContours,
        "configureTerrainBrush" as "configure-terrain-brush" => configure_terrain_brush::ConfigureTerrainBrush,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier.
use example::set_active_example;
use exaggeration::set_exaggeration;
use elevation_source::{clear_elevation_source, create_elevation_grid};
use crate::editor::gis3d::commands::overlays::{self, add_overlay_position, clear_overlays, edit_overlay_shape, move_overlay_position, remove_overlay_feature};
use crate::editor::gis3d::commands::import_elevation::{self, export as export_elevation_grid, file as import_elevation_file};
use crate::editor::gis3d::commands::{geojson::{self, export as export_geojson, file as import_geojson_file}, local_file};
use view::{configure_terrain_brush, set_camera};
//#endregion 🔖️Commands

//#region 🔖️Gis3dPlayApp
/// 🎮️ GIS 3D terrain play app. The document holds exaggeration plus the `map:in` overlay layer;
/// the camera is persisted by the exact terrain-window owner while pin selection is interaction state.
#[derive(Default)]
pub struct Gis3dPlayApp;

//#region 🧵️RetainedCommands
/// ↔️ The vertical-exaggeration range the Actions rail stages: flattened terrain through a five-fold
/// relief. `GIS3D_EXAGGERATION_STAGED_DEFAULT` is deliberately NOT the document's own default, so a
/// one-click dispatch from the rail produces a real edit instead of a zero-operation re-apply.
pub const GIS3D_EXAGGERATION_MINIMUM: f64 = 0.5;
pub const GIS3D_EXAGGERATION_MAXIMUM: f64 = 5.0;
pub const GIS3D_EXAGGERATION_STAGED_DEFAULT: f64 = 2.5;

pub const GIS3D_RETAINED_TOOL_IDS: &[&str] = &[
    "setExaggeration", "setCamera", "setActiveExample", "clearElevationSource", "editElevation",
    "addOverlayPosition", "removeOverlayFeature", "clearOverlays",
    "importElevationGrid", "editOverlayShape", "moveOverlayPosition", "createElevationGrid", "gradeTerrainRegion",
    "importGeoJson", "generateTerrainContours", "configureTerrainBrush",
];
const GIS3D_RETAINED_PAYLOAD_SCHEMA: &str = "gis.terrain.tool-command.v1";
pub const GIS3D_RETAINED_RAW_BYTES: usize = 8_192;
pub const GIS3D_RETAINED_WORK_ITEMS: usize = 1;

fn gis3d_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(
        GIS3D_RETAINED_RAW_BYTES, 32,
        semio_s_artifact_gis_gisterrain_inferences::elevation_edit::MAX_STROKE_WORK,
        16_384, 7_500,
    )
}

pub fn gis3d_retained_extent(command: &Gis3dCommand, _snapshot: &GisTerrainSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let bytes = match command {
        Gis3dCommand::SetExaggeration(payload) if payload.exaggeration.is_finite() => 0,
        Gis3dCommand::SetCamera(payload) if payload.camera_json.len() <= GIS3D_RETAINED_RAW_BYTES
            && parse_json(&payload.camera_json, JsonMemberPolicy::Reject).is_ok_and(|camera| camera.as_object().is_some()) => payload.camera_json.len(),
        Gis3dCommand::SetActiveExample(payload) => payload.example_id.len(),
        Gis3dCommand::ClearElevationSource(_) => 0,
        Gis3dCommand::CreateElevationGrid(payload) if payload.is_valid() => 0,
        Gis3dCommand::GradeTerrainRegion(payload) if !payload.id.trim().is_empty() && payload.id.len() <= 128 => payload.id.len(),
        Gis3dCommand::GenerateTerrainContours(payload) if payload.is_valid() => 0,
        Gis3dCommand::ConfigureTerrainBrush(payload) if payload.settings().is_ok() => 0,
        Gis3dCommand::AddOverlayPosition(payload) => payload.id.len() + payload.label.as_ref().map_or(0, String::len),
        Gis3dCommand::RemoveOverlayFeature(payload) => payload.id.len(),
        Gis3dCommand::ClearOverlays(_) => 0,
        Gis3dCommand::EditOverlayShape(payload) if payload.coordinates.len() <= overlays::MAX_COORDINATES_JSON_BYTES => {
            payload.id.len() + payload.kind.len() + payload.mode.len() + payload.coordinates.len() + payload.label.as_ref().map_or(0, String::len)
        }
        Gis3dCommand::MoveOverlayPosition(payload) => payload.id.len(),
        Gis3dCommand::ImportElevationGrid(_) => 0,
        Gis3dCommand::ExportElevationGrid(_) => 0,
        Gis3dCommand::ImportGeoJson(_) | Gis3dCommand::ExportGeoJson(_) => 0,
        Gis3dCommand::ImportElevationGridFile(file) => return local_file::valid_length(&file.payload, import_elevation::MAX_FILE_BYTES).then_some(GIS3D_RETAINED_WORK_ITEMS),
        Gis3dCommand::ImportGeoJsonFile(file) => return local_file::valid_length(&file.payload, local_file::MAX_FILE_BYTES).then_some(GIS3D_RETAINED_WORK_ITEMS),
        Gis3dCommand::EditElevation(payload) if payload.is_valid() => {
            payload.operation.len() + payload.stroke.as_ref().map_or(0, |samples| samples.len() * std::mem::size_of::<[u32; 2]>())
        }
        _ => return None,
    };
    (bytes <= GIS3D_RETAINED_RAW_BYTES).then_some(GIS3D_RETAINED_WORK_ITEMS)
}

#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
fn gis3d_retained_reduce(
    command: &Gis3dCommand,
    snapshot: &GisTerrainSnapshot,
    config: &NoConfig,
    history: &semio_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_plugin::app::InteractionHoverState,
    context: Option<&semio_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Gis3dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<GisTerrainMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    if let Some(mutation) = gis3d_window_mutation(command, snapshot, context.and_then(|context| context.view_state.as_deref()))? {
        return Ok(Emit { window_config_mutations: vec![mutation], ..Default::default() });
    }
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config, window: context.and_then(|context| context.window_config.as_ref()) })
}

fn gis3d_window_mutation(command: &Gis3dCommand, snapshot: &GisTerrainSnapshot, view: Option<&semio_plugin::ViewModel>) -> Result<Option<semio_plugin::WindowConfigMutation>, Fault> {
    use terrain::config::{GisTerrainWindowConfigMutation, SetTerrainBrush};
    let view = || view.ok_or_else(terrain::config::missing_window);
    let mutation = match command {
        Gis3dCommand::SetCamera(payload) => terrain::config::addressed_camera(view()?, snapshot, &payload.camera_json)?,
        Gis3dCommand::ConfigureTerrainBrush(payload) => {
            terrain::config::addressed(view()?, GisTerrainWindowConfigMutation::SetTerrainBrush(SetTerrainBrush { brush: Some(payload.settings()?) }))?
        }
        _ => return Ok(None),
    };
    Ok(Some(mutation))
}

pub struct Gis3dCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Gis3dCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GIS3D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }

    semio_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Gis3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.gis.gisterrain@1/*#editor",
        artifact_schema: "gis.terrain",
        factory: "Gis3dCommandJobFactory",
        factory_type: Gis3dCommandJobFactory,
        contract: gis3d_retained_contract(),
        tools: [
            "setExaggeration", "setCamera", "setActiveExample", "clearElevationSource", "editElevation",
            "addOverlayPosition", "removeOverlayFeature", "clearOverlays", "editOverlayShape",
            "moveOverlayPosition", "importElevationGrid", "createElevationGrid", "gradeTerrainRegion",
            "importGeoJson", "generateTerrainContours", "configureTerrainBrush",
        ]
    }
}

impl ToolJobFactory for Gis3dCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Gis3dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Gis3dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        GIS3D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        gis3d_retained_contract()
    }

    semio_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: GIS3D_RETAINED_RAW_BYTES, checkpoint_resume: false);

}

impl semio_plugin::ArtifactOwnedToolJobFactory for Gis3dCommandJobFactory {
    type Owner = EditorApp<Gis3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = GIS3D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GIS_3D_TERRAIN_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] =
        &[
            ArtifactToolPublicationContract { tool_id: "setExaggeration", lanes: &[ArtifactToolPublicationLane::Artifact] },
            ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
            ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact] },
            ArtifactToolPublicationContract { tool_id: "clearElevationSource", lanes: &[ArtifactToolPublicationLane::Artifact] },
            ArtifactToolPublicationContract { tool_id: "createElevationGrid", lanes: &[ArtifactToolPublicationLane::Artifact] },
            ArtifactToolPublicationContract { tool_id: "gradeTerrainRegion", lanes: &[ArtifactToolPublicationLane::Artifact] },
            ArtifactToolPublicationContract { tool_id: "addOverlayPosition", lanes: &[ArtifactToolPublicationLane::Artifact] },
            ArtifactToolPublicationContract { tool_id: "removeOverlayFeature", lanes: &[ArtifactToolPublicationLane::Artifact] },
            ArtifactToolPublicationContract { tool_id: "clearOverlays", lanes: &[ArtifactToolPublicationLane::Artifact] },
            ArtifactToolPublicationContract { tool_id: "editOverlayShape", lanes: &[ArtifactToolPublicationLane::Artifact] },
            ArtifactToolPublicationContract { tool_id: "moveOverlayPosition", lanes: &[ArtifactToolPublicationLane::Artifact] },
            ArtifactToolPublicationContract { tool_id: "editElevation", lanes: &[ArtifactToolPublicationLane::Artifact] },
            ArtifactToolPublicationContract { tool_id: "importElevationGrid", lanes: &[ArtifactToolPublicationLane::HostOnly] },
                ArtifactToolPublicationContract { tool_id: "importGeoJson", lanes: &[ArtifactToolPublicationLane::HostOnly] },
            ArtifactToolPublicationContract { tool_id: "generateTerrainContours", lanes: &[ArtifactToolPublicationLane::Artifact] },
            ArtifactToolPublicationContract { tool_id: "configureTerrainBrush", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ];
}
//#endregion 🧵️RetainedCommands

fn media_import_emit(port: &str, media: &Media) -> Result<Emit<GisTerrainMutation, NoConfigMutation>, MediaError> {
    if port == "elevation:in" {
        use semio_framework_value::FromValue;
        let source = match &media.payload {
            MediaPayload::Intrinsic { schema, value } if Gis3dPlayApp::accepts_original_structured_media(port, schema, &media.media_type) =>
                crate::schema::snapshot::ElevationSource::from_value(value.clone()),
            MediaPayload::Structured { schema, json } if Gis3dPlayApp::accepts_original_structured_media(port, schema, &media.media_type) && json.len() <= GIS_ELEVATION_JSON_MAX_BYTES =>
                semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject),
            _ => return Err(MediaError::Payload(port.into(), "elevation media identity or size differs".into())),
        }.map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
        return Ok(Emit::mutations(vec![GisTerrainMutation::ReplaceElevationSource(crate::mutations::ReplaceElevationSource { new_elevation_source: Some(source) })]));
    }
    if port != "map:in" {
        return Err(MediaError::NotImplemented);
    }
    use crate::schema::imported_map::ImportedMap;
    let maximum = store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;
    let imported_map = match &media.payload {
        MediaPayload::Intrinsic { schema, value } if Gis3dPlayApp::accepts_original_structured_media(port, schema, &media.media_type)
            && semio_framework_pack_json::to_json_string(value).len() <= maximum => ImportedMap::from_media_value(value),
        MediaPayload::Structured { schema, json } if Gis3dPlayApp::accepts_original_structured_media(port, schema, &media.media_type)
            && json.len() <= maximum => ImportedMap::from_json(json),
        _ => return Err(MediaError::Payload(port.into(), "map media identity or size differs".into())),
    };
    let imported_map = imported_map.map_err(|error| MediaError::Payload(port.into(), error))?;
    use crate::mutations::change_imported_features::ChangeImportedFeatures;
    Ok(Emit::mutations(vec![GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_map: Some(imported_map) })]))
}

impl ArtifactEditor for Gis3dPlayApp {
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        static NOTICES: std::sync::LazyLock<Vec<(&'static str, LocalizedLabel)>> = std::sync::LazyLock::new(|| {
            import_elevation::fault_notices().iter().chain(geojson::fault_notices()).chain(contours::fault_notices())
                .chain(overlays::fault_notices()).cloned().chain(configure_terrain_brush::fault_notices()).collect()
        });
        NOTICES.as_slice()
    }

    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    type Snapshot = GisTerrainSnapshot;
    type Mutation = GisTerrainMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::gis3d::presence::Gis3dPresence;
    type PresenceMutation = crate::editor::gis3d::presence::Gis3dPresenceMutation;
    type Transient = semio_plugin::NoTransient;
    type TransientMutation = semio_plugin::NoTransientMutation;

    type Command = Gis3dCommand;

    const DIALECT: Dialect = crate::GISTERRAIN_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GIS_3D_TERRAIN_SCHEMA;

    fn document_store_owners_source_demands() -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        semio_plugin::cold_replay_document_store_owners_source_demands::<Self::Snapshot, Self::Mutation>()
    }

    fn build_document_store_owners(grant: semio_framework_value::RetainedCloneGrant) -> Option<Result<(
        store::os_store::document_store_owners::DocumentStoreOwners<Self::Snapshot, Self::Mutation>,
        semio_framework_value::RetainedCloneProgress,
    ), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<Self::Snapshot, Self::Mutation>>> {
        Some(semio_plugin::cold_replay_document_store_owners::<Self::Snapshot, Self::Mutation>(grant).map(|(mut owners, progress)| {
            owners.operation_wire = owners.operation_wire.with_original_target(|operation| Some(operation)).with_original_receiving();
            (owners, progress)
        }))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::os_store::cold_mutation_apply_preparation_factory())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_plugin::no_transient_local_root_retirement_factory())
    }

    fn register_window_config_owners(registry: &mut semio_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<terrain::config::GisTerrainWindowConfigOwner>()
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = Gis3dCommandJobFactory::bounded_first_step_tool_proofs();
        proofs.extend(local_file::LocalFileImportJobFactory::proofs());
        proofs.extend(export_geojson::LocalFileExportJobFactory::proofs());
        proofs
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Gis3dCommandJobFactory::new(&controller))?;
        registry.register(local_file::LocalFileImportJobFactory::new(&controller))?;
        registry.register(export_geojson::LocalFileExportJobFactory::new(&controller))?;
        registry.register(media_export::Factory::new(&controller))
    }

    fn build_media_export_job(request: &mut Option<semio_plugin::ArtifactMediaExportJobRequest<EditorApp<Self>>>, output: &mut Option<Box<dyn semio_plugin::ArtifactReservedJob + Send>>, construction: &mut semio_plugin::MediaConstruction, cx: &mut semio_plugin::__job::StepContext<'_>) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
        media_export::Export::admit(request, output, construction, cx)
    }

    fn accepts_original_structured_media(port: &str, schema: &str, media_type: &MediaType) -> bool {
        match port {
            "elevation:in" => schema == GIS_ELEVATION_GRID_SCHEMA && *media_type == GIS_ELEVATION_MEDIA_TYPE,
            "map:in" => schema == "2d.map" && *media_type == GIS_MAP_MEDIA_TYPE,
            _ => false,
        }
    }

    fn build_reserved_tool_job(request: &mut Option<semio_plugin::ArtifactReservedToolJobRequest<EditorApp<Self>>>, output: &mut Option<Box<dyn semio_plugin::ArtifactReservedJob + Send>>, construction: &mut semio_plugin::__job::OriginalConstructionAdmission, cx: &mut semio_plugin::__job::StepContext<'_>) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
        if output.is_none() && request.as_ref().is_none_or(|request| request.tool_id != "import-media") {
            return Ok(false);
        }
        semio_plugin::plugin_app_close_prelude::admit_reserved_work::<EditorApp<Self>, media_import::MediaImport>(request, output, construction, cx)
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_plugin::ErasedToolJob>, _construction:&mut semio_plugin::__job::OriginalConstructionAdmission,cx: &mut semio_plugin::__job::StepContext<'_>) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        let file_import = local_file::FILE_ACTIONS.contains(&original.tool_id.as_str());
        let file_export = matches!(original.tool_id.as_str(), export_geojson::ACTION | export_elevation_grid::ACTION);
        if !file_import && !file_export && !GIS3D_RETAINED_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if original.command.command_id() != original.tool_id {
            return Err(semio_plugin::__value::ValueError::literal(semio_plugin::__value::ValueRefusalKind::InvariantViolated, "the gis terrain command does not match its exact registered tool").into());
        }
        if gis3d_retained_extent(&original.command, &original.snapshot, &original.interaction_state).is_none() {
            return Err(Fault::from("gis3d-command-payload-too-large").into());
        }
        let tool_id = original.command.command_id();
        if file_export {
            return semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, Gis3dCommand::command_id, GIS3D_RETAINED_RAW_BYTES, GIS3D_RETAINED_WORK_ITEMS, false, move |_| export_geojson::Work::for_action(tool_id));
        }
        if matches!(&*original.command, Gis3dCommand::EditElevation(payload) if payload.stroke.is_some()) {
            return semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, Gis3dCommand::command_id, GIS3D_RETAINED_RAW_BYTES, GIS3D_RETAINED_WORK_ITEMS, false, |_| edit_elevation::StrokeWork::default());
        }
        let maximum_raw_bytes = if file_import { local_file::MAX_WIRE_BYTES } else { GIS3D_RETAINED_RAW_BYTES };
        if tool_id == import_elevation::FILE_ACTION {
            return semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, Gis3dCommand::command_id, maximum_raw_bytes, GIS3D_RETAINED_WORK_ITEMS, false, |_| import_elevation::work::Work::default());
        }
        if tool_id == contours::ACTION {
            return semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, Gis3dCommand::command_id, maximum_raw_bytes, GIS3D_RETAINED_WORK_ITEMS, false, |_| contours::Work::default());
        }
        semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, Gis3dCommand::command_id, maximum_raw_bytes, GIS3D_RETAINED_WORK_ITEMS, false, |_| BoundedArtifactCommandWork::new(tool_id, gis3d_retained_reduce, gis3d_retained_extent))
    }

    fn reference_birth()->Option<fn()->Result<GisTerrainSnapshot,semio_plugin::Fault>>{Some(||{
        Ok(default_terrain_document())
    })}

    /// 🗺️ Declares workflow ports and document media contract.
    fn io() -> Option<AppIo> {
        Some(gis3d_io())
    }

    /// 🌀️ Exports `scene:out`, `elevation:out`, and `artifact:out` media.
    fn export_media(port: &str, doc: &ArtifactView<'_, GisTerrainSnapshot>) -> Result<Media, MediaError> {
        match port {
            "scene:out" => gis3d_scene_media(doc.snapshot),
            "elevation:out" => gis3d_elevation_media(doc.snapshot),
            "artifact:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// ✔️ Admits `elevation:in` and `map:in` workflow media into document mutations.
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, GisTerrainSnapshot>) -> Result<Emit<GisTerrainMutation, NoConfigMutation, Self::DraftMutation>, MediaError> {
        media_import_emit(port, media)
    }

    fn command_id(command: &Gis3dCommand) -> &'static str {
        command.command_id()
    }

    fn original_action_command_producer() -> Option<&'static dyn semio_plugin::ArtifactOriginalCommandProducerFactory<Self::Command>> {
        Some(&semio_plugin::OriginalActionCommandBridge(Self::command_from_action, local_file::command_wire_maximum))
    }

    /// 🏠️ Bridges host action identifiers and JSON arguments onto `Gis3dCommand`.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        use semio_framework_value::DslValue;

        if let Some(fault) = original_fault.take() { return Err(fault); }
        if action == import_elevation::FILE_ACTION {
            return import_elevation_file::ImportElevationGridFile::receive(args).map(Gis3dCommand::ImportElevationGridFile);
        }
        if action == geojson::FILE_ACTION {
            return import_geojson_file::ImportGeoJsonFile::receive(args).map(Gis3dCommand::ImportGeoJsonFile);
        }
        let args = args.as_ref().unwrap_or(&DslValue::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(DslValue::as_str).map(str::to_string)) };
        let number = |key| args.get(key).and_then(DslValue::as_f64).ok_or_else(|| overlays::OverlayFault::number(key).fault());
        let feature_id = || str_arg(&["id"]).ok_or_else(|| overlays::OverlayFault::IdentifierMissing.fault());
        let label = || -> Result<Option<String>, Fault> {
            let Some(value) = args.get("label").filter(|value| !matches!(value, DslValue::Null)) else { return Ok(None); };
            let label = value.as_str().ok_or_else(|| overlays::OverlayFault::LabelInvalid.fault())?.trim();
            Ok((!label.is_empty()).then(|| label.to_owned()))
        };
        match action {
            "setExaggeration" => Ok(Gis3dCommand::SetExaggeration(set_exaggeration::SetExaggeration { exaggeration: ["exaggeration", "value"].iter().find_map(|key| args.get(key).and_then(DslValue::as_f64)).unwrap_or(1.0) })),
            "setCamera" => {
                let camera_json = str_arg(&["cameraJson", "camera_json"]).or_else(|| args.get("camera").map(|value| {
                    value.as_str().map_or_else(|| to_json_string(value), str::to_owned)
                })).unwrap_or_else(|| "{}".into());
                if camera_json.len() > GIS3D_RETAINED_RAW_BYTES {
                    return Err(Fault::from("gis3d-command-payload-too-large").into());
                }
                Ok(Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json }))
            }
            "setActiveExample" => Ok(Gis3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_arg(&["exampleId", "example_id", "value"]).unwrap_or_default() })),
            "clearElevationSource" => Ok(Gis3dCommand::ClearElevationSource(clear_elevation_source::ClearElevationSource {})),
            "createElevationGrid" => Ok(Gis3dCommand::CreateElevationGrid(create_elevation_grid::CreateElevationGrid::from_action(args)?)),
            "gradeTerrainRegion" => Ok(Gis3dCommand::GradeTerrainRegion(grade_region::GradeTerrainRegion::from_action(args)?)),
            "generateTerrainContours" => Ok(Gis3dCommand::GenerateTerrainContours(contours::GenerateTerrainContours::from_action(args)?)),
            "configureTerrainBrush" => Ok(Gis3dCommand::ConfigureTerrainBrush(configure_terrain_brush::ConfigureTerrainBrush::from_action(args)?)),
            "addOverlayPosition" => {
                Ok(Gis3dCommand::AddOverlayPosition(add_overlay_position::AddOverlayPosition {
                    id: feature_id()?,
                    longitude: number("longitude")?,
                    latitude: number("latitude")?,
                    label: label()?,
                    elevation: match args.get("elevation") {
                        None | Some(DslValue::Null) => None,
                        Some(_) => Some(number("elevation")?),
                    },
                }))
            }
            "removeOverlayFeature" => Ok(Gis3dCommand::RemoveOverlayFeature(remove_overlay_feature::RemoveOverlayFeature {
                id: feature_id()?,
            })),
            "clearOverlays" => Ok(Gis3dCommand::ClearOverlays(clear_overlays::ClearOverlays {})),
            "editOverlayShape" => Ok(Gis3dCommand::EditOverlayShape(edit_overlay_shape::EditOverlayShape {
                id: feature_id()?,
                kind: str_arg(&["kind"]).unwrap_or_else(|| "route".into()),
                mode: str_arg(&["mode"]).unwrap_or_else(|| "replace".into()),
                coordinates: str_arg(&["coordinates"]).ok_or_else(|| overlays::OverlayFault::VerticesMissing.fault())?,
                label: label()?,
            })),
            "moveOverlayPosition" => Ok(Gis3dCommand::MoveOverlayPosition(move_overlay_position::MoveOverlayPosition {
                id: feature_id()?, longitude: number("longitude")?, latitude: number("latitude")?,
            })),
            "importElevationGrid" => Ok(Gis3dCommand::ImportElevationGrid(import_elevation::ImportElevationGrid {})),
            "exportElevationGrid" => Ok(Gis3dCommand::ExportElevationGrid(export_elevation_grid::ExportElevationGrid {})),
            "importGeoJson" => Ok(Gis3dCommand::ImportGeoJson(geojson::ImportGeoJson {})),
            "exportGeoJson" => Ok(Gis3dCommand::ExportGeoJson(export_geojson::ExportGeoJson {})),
            "editElevation" => Ok(Gis3dCommand::EditElevation(edit_elevation::EditElevation::from_action(args)?)),
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    fn handle(
        command: &Gis3dCommand,
        doc: &ArtifactView<'_, GisTerrainSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        _interaction: &InteractionView<'_>,
        view_state: Option<&semio_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<GisTerrainMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        if let Some(mutation) = gis3d_window_mutation(command, doc.snapshot, view_state)? {
            return Ok(Emit { window_config_mutations: vec![mutation], ..Default::default() });
        }
        command.dispatch(doc, cfg)
    }

    /// 🧮️ Empty app config; camera state belongs to the exact registered terrain-window owner.
    fn config_spec() -> semio_plugin::ConfigSpec {
        semio_plugin::ConfigSpec::default()
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, GisTerrainSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_plugin::ViewModel) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {
        match body_key {
            terrain::GIS3D_PLAY_BODY_COMPOSITE => terrain::render(doc.snapshot, terrain::config::current(cfg), view_state).map(semio_plugin::built_to_component_tree),
            terrain_data::BODY => terrain_data::render(doc.snapshot, view_state).map(semio_plugin::built_to_component_tree),
            _ => semio_plugin::built_text_to_component_tree(Label::data(LocalizedLabel::native(&format!("Unknown body: {body_key}"), &format!("Unbekannter Body: {body_key}")).resolve(view_state.terminology, view_state.locale))),
        }
    }
}
//#endregion 🔖️Gis3dPlayApp

//#region 🔖️Manifest
pub fn create_gis3d_app() -> semio_plugin::AppDefinition {
    use semio_plugin::{ActionArgDef, ActionDefinition, ActionKind, CapabilityAudience, UtilityDefinition};
    use InteractiveJobClassification::Migrated;
    let label = LocalizedLabel::native;
    let mut editor = Editor::builder(crate::GISTERRAIN_DIALECT)
        .document(["semio", "gis", "3d"])
        .artifact_kind(semio_s_artifact_gis_gismap::artifact_kind())
        .media_input(gis3d_map_in_port())
        .media_output(gis3d_scene_out_port())
        .icon_id("gis3d")
        .mode_def(view_mode::definition())
        .default_mode_id(view_mode::GIS3D_PLAY_MODE_VIEW)
        .window_kind_def(terrain::definition());
    for (id, en, de, icon) in terrain::UTILITIES {
        let mut utility = UtilityDefinition::new(id, label(en, de), icon);
        utility.allows_actions_while_active = true;
        utility.cursor = (id != "select").then(|| "crosshair".into());
        editor = editor.utility(utility);
    }
    editor = editor.window_kind_utilities(terrain::GIS3D_PLAY_WINDOW_MAIN, terrain::UTILITIES.iter().map(|utility| utility.0.into()).collect())
        .panel_tab_def(terrain_data::definition())
        .default_layout(view_mode::layout())
        .interaction(InteractionDefinition {
            id: "features".into(),
            label: label("Features", "Objekte"),
            granularities: vec![GranularityDefinition { id: "pin".into(), label: label("Pin", "Stift"), icon_id: "map-pin".into() }],
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
        .window_kind_interactions(terrain::GIS3D_PLAY_WINDOW_MAIN, vec![InteractionRef::new("features")])
        .action_with(ActionDefinition::new("setCamera", label("Set Camera", "Kamera festlegen"), ActionKind::View, "camera"))
        .action_with(ActionDefinition::new("configureTerrainBrush", label("Configure Terrain Brush", "Geländepinsel einstellen"), ActionKind::View, "configure-terrain-brush"))
        .action_args("configureTerrainBrush", configure_terrain_brush::action_arguments())
        .mutation("setExaggeration", label("Set Exaggeration", "Überhöhung festlegen"))
        .mutation("clearElevationSource", label("Clear Elevation Source", "Höhenquelle entfernen"))
        .mutation("createElevationGrid", label("Create Flat Grid", "Flaches Höhenraster erstellen"))
        .action_args("createElevationGrid", create_elevation_grid::action_arguments())
        .mutation("gradeTerrainRegion", label("Grade Region", "Region nivellieren"))
        .action_args("gradeTerrainRegion", grade_region::action_arguments())
        .mutation("generateTerrainContours", label("Generate Terrain Contours", "Geländehöhenlinien erzeugen"))
        .action_args("generateTerrainContours", contours::action_arguments())
        .mutation("addOverlayPosition", label("Add Position", "Position hinzufügen"))
        .mutation("removeOverlayFeature", label("Remove Map Feature", "Kartenobjekt entfernen"))
        .mutation("clearOverlays", label("Clear Map Features", "Kartenobjekte entfernen"))
        .mutation("editOverlayShape", label("Edit Route or Region", "Route oder Fläche bearbeiten"))
        .mutation("moveOverlayPosition", label("Move Position", "Position verschieben"))
        .mutation("editElevation", label("Edit Elevation", "Höhe bearbeiten"))
        .shell_action(import_elevation::REQUEST_ACTION, label("Import Elevation Grid…", "Höhenraster importieren…"))
        .shell_action(export_elevation_grid::ACTION, label("Export Elevation Grid…", "Höhenraster exportieren…"))
        .shell_action(geojson::REQUEST_ACTION, label("Import GeoJSON Overlays…", "GeoJSON-Overlays importieren…"))
        .shell_action(export_geojson::ACTION, label("Export GeoJSON Overlays…", "GeoJSON-Overlays exportieren…"))
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(geojson::FILE_ACTION, label("Load GeoJSON Overlays", "GeoJSON-Overlays laden"), ActionKind::Mutation) })
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(import_elevation::FILE_ACTION, label("Load Elevation Grid", "Höhenraster laden"), ActionKind::Mutation) })
        .action_args("editElevation", edit_elevation::action_arguments())
        .action_with(ActionDefinition::new("setActiveExample", label("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
        .action_destructive("setActiveExample")
        .action_args("setActiveExample", vec![ActionArgDef::select("exampleId", label("Example", "Beispiel"), example::example_arg_options()).default_value(&example::DEFAULT_EXAMPLE_ID)])
        .action_args("setExaggeration", vec![ActionArgDef::slider("value", label("Exaggeration", "Überhöhung"), GIS3D_EXAGGERATION_MINIMUM, GIS3D_EXAGGERATION_MAXIMUM).default_value(&GIS3D_EXAGGERATION_STAGED_DEFAULT)])
        .action_args("addOverlayPosition", overlays::add_args())
        .action_args("removeOverlayFeature", vec![overlays::id_arg()])
        .action_args("editOverlayShape", overlays::shape_args())
        .action_args("moveOverlayPosition", overlays::move_args());
    for action_id in [
        "setCamera", "configureTerrainBrush", "setExaggeration", "setActiveExample", "clearElevationSource",
        "createElevationGrid", "gradeTerrainRegion", "generateTerrainContours", "addOverlayPosition",
        "removeOverlayFeature", "clearOverlays", "editOverlayShape", "moveOverlayPosition", "editElevation",
        import_elevation::REQUEST_ACTION, import_elevation::FILE_ACTION, export_elevation_grid::ACTION,
        geojson::REQUEST_ACTION, geojson::FILE_ACTION, export_geojson::ACTION,
    ] {
        editor = editor.action_interactive_job(action_id, Migrated);
    }
    editor
        .keybinding("mod+z", "undo")
        .keybinding("mod+shift+z", "redo")
        .config(Gis3dPlayApp::config_spec())
        .io(gis3d_io())
        .interactive_jobs(Migrated)
        .action_describe("setActiveExample", label("Replaces the whole terrain with one of the plugin's bundled terrain examples, by example id.", "Ersetzt das gesamte Gelände durch eines der mitgelieferten Geländebeispiele, anhand der Beispiel-Id."))
        .action_describe("setExaggeration", label("Sets the vertical exaggeration factor the terrain surface is drawn with.", "Legt den Überhöhungsfaktor fest, mit dem die Geländeoberfläche gezeichnet wird."))
        .action_describe("clearElevationSource", label("Clears the saved elevation grid.", "Entfernt das gespeicherte Höhenraster."))
        .action_describe("addOverlayPosition", label("Adds a position using WGS84 coordinates.", "Fügt eine Position mit WGS84-Koordinaten hinzu."))
        .action_describe("removeOverlayFeature", label("Removes a position, route or region by its feature ID.", "Entfernt eine Position, Route oder Fläche anhand ihrer Objekt-ID."))
        .action_describe("clearOverlays", label("Clears the map's positions, routes and regions.", "Entfernt die Positionen, Routen und Flächen der Karte."))
        .action_describe("editElevation", label("Edits stored heights in meters. Use a negative value to lower terrain or enter 0–100 to smooth.", "Bearbeitet gespeicherte Höhen in Metern. Negative Werte senken das Gelände ab. Zum Glätten 0–100 eingeben."))
        .action_describe("gradeTerrainRegion", label("Sets terrain samples inside an existing GIS region to one height and preserves the holes.", "Setzt Rasterpunkte in einer vorhandenen GIS-Region auf eine Höhe und erhält die Aussparungen."))
        .action_describe("generateTerrainContours", label("Replaces generated contours with lines at multiples of the interval in metres, using the saved elevation grid.", "Ersetzt erzeugte Höhenlinien durch Linien in Vielfachen des Höhenabstands in Metern, anhand des gespeicherten Höhenrasters."))
        .action_describe("configureTerrainBrush", label("Sets the radius, elevation step and smoothing strength for this terrain window.", "Legt Radius, Höhenänderung und Glättungsstärke für dieses Geländefenster fest."))
        .action_describe(import_elevation::REQUEST_ACTION, label("Imports a WGS84 Esri ASCII Grid (.asc, up to 64 KiB) with elevations in metres. Replaces the current elevation grid.", "Importiert ein WGS84-Esri-ASCII-Raster (.asc, bis zu 64 KiB) mit Höhen in Metern. Ersetzt das aktuelle Höhenraster."))
        .action_describe(export_elevation_grid::ACTION, label("Downloads the saved heights as a WGS84 Esri ASCII Grid (.asc). Requires equal latitude and longitude spacing.", "Lädt die gespeicherten Höhen als WGS84-Esri-ASCII-Raster (.asc) herunter. Die Abstände in Längen- und Breitengraden müssen gleich sein."))
        .action_describe(geojson::REQUEST_ACTION, label("Replaces the current overlays with points, lines, and polygons from a GeoJSON FeatureCollection (up to 24 KiB).", "Ersetzt die aktuellen Overlays durch Punkte, Linien und Polygone aus einer GeoJSON-FeatureCollection mit höchstens 24 KiB."))
        .action_describe(export_geojson::ACTION, label("Downloads the current overlay features and their properties as GeoJSON.", "Lädt die aktuellen Overlay-Objekte und ihre Eigenschaften als GeoJSON herunter."))
        .action_audience("setCamera", CapabilityAudience::Chrome)
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub mod unit_tests;
//#endregion 🧪️UnitTests
