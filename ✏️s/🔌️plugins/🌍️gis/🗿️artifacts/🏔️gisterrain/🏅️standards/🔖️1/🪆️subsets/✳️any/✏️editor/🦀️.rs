//! ⛰️ GIS 3D play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the World3d
//! viewport in `🎭️modes/👁️view/🪟️windows/🏔️terrain`, view state in `🦀️config.rs`, fixture-scenery
//! compute in `crate::schema::inferences` (`parse_descriptor`), and this app's
//! typed media I/O surface (`map:in` overlay, ports, scene media) below in `🔖️Io` — relocated from
//! the artifact's `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).

use crate::op::GisTerrainMutation;
use crate::schema::default_terrain_document;
use crate::{GisTerrainSnapshot, GIS_3D_TERRAIN_SCHEMA};
use crate::editor::gis3d::commands::{exaggeration, view};
use crate::editor::gis3d::config::{Gis3dConfig, Gis3dConfigMutation, SetCamera};
use crate::editor::gis3d::modes::view as view_mode;
use crate::editor::gis3d::modes::view::windows::terrain;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppIo, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, Dialect, DraftView, Editor, EditorApp, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec,
    InteractionDefinition, InteractionRef, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec,
};
use serde_json::Value;
use store::ArtifactPack;
use store::EngineHandles;

//#region 🔖️Constants
pub const GIS3D_PLAY_APP_ID: &str = "gis3d-play";
//#endregion 🔖️Constants

//#region 🔖️Io
/// 🧭️ Relocated from the artifact's `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this app's typed media I/O surface
/// (`AppDefinition.io`), plus the two app-specific workflow ports
/// (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE Wave 2 port recipe): `map:in`
/// (a `2d.map` producer — gis2d's `map:out` — feeds an overlay pin layer, see
/// `GisTerrainSnapshot::imported_features_json`) and `scene:out` (this terrain as `3d.mesh`).
/// `document_media_type` is Data×Value (the document is a scalar "exaggeration + imported overlay"
/// record, not itself mesh geometry — `scene:out` is the actual renderable mesh/terrain surface).
pub fn gis3d_io() -> AppIo {
    AppIo {
        document_schema: GIS_3D_TERRAIN_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        ports: vec![gis3d_map_in_port(), gis3d_scene_out_port()],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: crate::GISTERRAIN_DIALECT.artifact_kind.into(), name: "GIS Terrain".into(), dimension: "3d".into(), component_kind: "gisterrain".into() },
    }
}

/// 🔌️ `map:in` — a `2d.map` producer (gis2d's `map:out`) feeding an overlay pin layer into this
/// terrain (see `GisTerrainSnapshot::imported_features_json`). `One`/optional: exactly one map may
/// be draped onto a terrain at a time, and a terrain with no upstream edge is valid.
pub fn gis3d_map_in_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "map:in".into(),
        label: "Map".into(),
        direction: semio_framework_plugin::MediaPortDirection::In,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        kind_id: Some(semio_s_artifact_gis_gismap::GISMAP_DIALECT.artifact_kind.into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}

/// 🔌️ `scene:out` — this terrain as `3d.mesh` (kind already registered by lowpoly; reused verbatim,
/// not redeclared — WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe). `Many`/optional: several
/// downstream consumers may fan out from one terrain, and a terrain with no downstream edge is valid.
pub fn gis3d_scene_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "scene:out".into(),
        label: "Scene".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        kind_id: Some("3d.mesh".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::Many,
    }
}

/// 🎞️ `scene:out`'s `Media` value. First pass (mirrors this app's own "deliberately minimal" module
/// doc): gis3d has no CPU-side heightmap tessellator yet (rendering is scene-descriptor driven, see the
/// 🏔️terrain window's `render`/`build_terrain_scene_json`), so this exports the same terrain descriptor
/// fields (exaggeration + imported overlay) as a structured `3d.mesh` payload rather than a real
/// triangulated mesh — an honest placeholder for the day a tessellator lands, not a silent fake.
pub fn gis3d_scene_media(document: &GisTerrainSnapshot) -> Media {
    Media {
        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        payload: MediaPayload::Structured {
            schema: "3d.mesh".into(),
            json: serde_json::json!({
                "exaggeration": document.exaggeration,
                "importedFeatures": serde_json::from_str::<Value>(&document.imported_features_json).unwrap_or(serde_json::json!(null)),
            })
            .to_string(),
        },
    }
}
//#endregion 🔖️Io

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Gis3dPlayApp::Command` — the SOLE dispatch surface for gis3d's own behavior, covering every
    /// action `create_gis3d_app` declares. Row order is the binary variant ordinal: appending is safe,
    /// reordering is a wire-format break.
    pub enum Gis3dCommand for GisTerrainSnapshot, GisTerrainMutation, Gis3dConfig, Gis3dConfigMutation {
        "setExaggeration" as "exaggeration" => set_exaggeration::SetExaggeration,
        "setCamera" as "camera" => set_camera::SetCamera,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier.
use exaggeration::set_exaggeration;
use view::set_camera;
//#endregion 🔖️Commands

//#region 🔖️Gis3dPlayApp
/// ⛰️ GIS 3D terrain play app. The document holds exaggeration plus the `map:in` overlay layer;
/// the camera and pin selection are [`Gis3dConfig`] — session-only but real, undoable config state.
#[derive(Default)]
pub struct Gis3dPlayApp;

//#region 🧵️RetainedCommands
const GIS3D_RETAINED_TOOL_IDS: &[&str] = &["setExaggeration", "setCamera", ];
const GIS3D_RETAINED_PAYLOAD_SCHEMA: &str = "gis.terrain.tool-command.v1";
const GIS3D_RETAINED_RAW_BYTES: usize = 8_192;
const GIS3D_RETAINED_WORK_ITEMS: usize = 1;

fn gis3d_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(GIS3D_RETAINED_RAW_BYTES, 32, 32, 16_384, 7_500)
}

fn gis3d_retained_extent(command: &Gis3dCommand, _snapshot: &GisTerrainSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let bytes = match command {
        Gis3dCommand::SetExaggeration(payload) if payload.exaggeration.is_finite() => 0,
        Gis3dCommand::SetCamera(payload) if serde_json::from_str::<Value>(&payload.camera_json).is_ok_and(|camera| camera.is_object()) => payload.camera_json.len(),
        _ => return None,
    };
    (bytes <= GIS3D_RETAINED_RAW_BYTES).then_some(GIS3D_RETAINED_WORK_ITEMS)
}

fn gis3d_retained_reduce(
    command: &Gis3dCommand,
    snapshot: &GisTerrainSnapshot,
    config: &Gis3dConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Gis3dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<GisTerrainMutation, Gis3dConfigMutation, NoDraftMutation>, Fault> {
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config })
}

struct Gis3dCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Gis3dCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: GIS3D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
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
        if input.declared_bytes() > GIS3D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("GIS terrain bounded command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for Gis3dCommandJobFactory {
    type Owner = EditorApp<Gis3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = GIS3D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = GIS_3D_TERRAIN_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "setExaggeration", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
const GIS3D_STORE_MAXIMUM_BYTES: usize = 32_768;

type Gis3dPrepareOne<P, M> = fn(&P, M) -> Result<(P, Vec<M>, M, usize), String>;

struct Gis3dOneItemPreparation<P, M> {
    base: Option<store::SnapshotRead<P>>,
    mutation: Option<M>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(P, Vec<M>, M, usize)>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<P, M>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    prepare: Gis3dPrepareOne<P, M>,
    phase: u8,
    cancelled: bool,
    closing: bool,
}

fn gis3d_store_edit<M>(prefix: &str, forward: M, inverse: Vec<M>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<M> {
    let id = format!("{prefix}-{}", authority.next_sequence_number());
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

fn gis3d_bounded_serialized_bytes<T: dsl::ToValue>(value: &T) -> Result<usize, String> {
    let bytes = dsl::os_pack::json::to_json_string(value).len();
    if bytes > GIS3D_STORE_MAXIMUM_BYTES {
        return Err("GIS terrain Store root exceeds its fixed envelope".to_string());
    }
    Ok(bytes)
}

fn prepare_gis3d_artifact(base: &GisTerrainSnapshot, mutation: GisTerrainMutation) -> Result<(GisTerrainSnapshot, Vec<GisTerrainMutation>, GisTerrainMutation, usize), String> {
    use protocol::{Mutation as _, MutationDiff as _};
    if !matches!(&mutation, GisTerrainMutation::ChangeExaggeration(payload) if payload.new_exaggeration.is_finite()) {
        return Err("GIS terrain Artifact preparation only admits ChangeExaggeration".into());
    }
    let retained_bytes = gis3d_bounded_serialized_bytes(base)?;
    let inverse = mutation.inverse(base);
    let post = mutation.diff(base).into_parts().0.apply(base).map_err(|_| "GIS terrain Artifact mutation could not produce its post root".to_string())?;
    Ok((post, inverse, mutation, retained_bytes))
}

fn prepare_gis3d_config(base: &Gis3dConfig, mutation: Gis3dConfigMutation) -> Result<(Gis3dConfig, Vec<Gis3dConfigMutation>, Gis3dConfigMutation, usize), String> {
    use protocol::{Mutation as _, MutationDiff as _};
    let valid = match &mutation {
        Gis3dConfigMutation::SetCamera(SetCamera { camera_json }) => camera_json.len() <= GIS3D_RETAINED_RAW_BYTES && serde_json::from_str::<Value>(camera_json).is_ok_and(|camera| camera.is_object()),
    };
    if !valid {
        return Err("GIS terrain Config preparation rejected its exact mutation envelope".into());
    }
    let retained_bytes = gis3d_bounded_serialized_bytes(base)?;
    let inverse = mutation.inverse(base);
    let post = mutation.diff(base).into_parts().0.apply(base).map_err(|_| "GIS terrain Config mutation could not produce its post root".to_string())?;
    Ok((post, inverse, mutation, retained_bytes))
}

impl<P, M> store::ArtifactStoreOneItemPreparation<P, M> for Gis3dOneItemPreparation<P, M>
where
    P: Send + Sync + 'static,
    M: dsl::ToValue + Send + 'static,
{
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() || self.phase >= 2 {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        match self.phase {
            0 => {
                let base = self.base.as_ref().ok_or_else(|| "GIS terrain preparation lost its exact base root".to_string())?;
                let mutation = self.mutation.take().ok_or_else(|| "GIS terrain preparation lost its mutation owner".to_string())?;
                self.candidate = Some((self.prepare)(base.get(), mutation)?);
                self.phase = 1;
                let completed_bytes = self.candidate.as_ref().map_or(0, |candidate| candidate.3);
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: completed_bytes as u64, digest: [0; 32] };
                Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint))
            }
            1 => {
                let (post, inverse, forward, completed_bytes) = self.candidate.take().ok_or_else(|| "GIS terrain preparation lost its semantic candidate".to_string())?;
                let authority = self.authority.as_ref().ok_or_else(|| "GIS terrain preparation lost its Store authority".to_string())?;
                let prepared = authority.prepare_one_item(gis3d_store_edit("gis-terrain-retained", forward, inverse, self.description.take(), authority), std::sync::Arc::new(post))?;
                self.phase = 2;
                self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: completed_bytes as u64, digest: prepared.edit_digest() };
                self.prepared = Some(prepared);
                Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
            }
            _ => Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)),
        }
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
        if self.prepared.take().is_some() || self.candidate.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("GIS terrain preparation could not return its exact base root".into());
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
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.candidate.is_none() && self.prepared.is_none()
    }
}

struct Gis3dArtifactStorePreparationFactory;
struct Gis3dConfigStorePreparationFactory;

fn begin_gis3d_preparation<P, M>(
    request: store::ArtifactStoreOneItemPreparationRequest<P, M>,
    prepare: Gis3dPrepareOne<P, M>,
) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, store::ArtifactStoreOneItemPreparationRequest<P, M>>
where
    P: Send + Sync + 'static,
    M: dsl::ToValue + Send + 'static,
{
    if request.lane != store::HistoryLane::Document
        || request.operation != request.authority.operation()
        || request.generation != request.authority.generation()
        || request.base_revision != request.authority.base_revision()
        || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
    {
        return Err(request);
    }
    Ok(Box::new(Gis3dOneItemPreparation {
        base: Some(request.base),
        mutation: Some(request.mutation),
        description: request.description,
        authority: Some(request.authority),
        candidate: None,
        prepared: None,
        checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
        prepare,
        phase: 0,
        cancelled: false,
        closing: false,
    }))
}

impl store::ArtifactStoreOneItemPreparationFactory<GisTerrainSnapshot, GisTerrainMutation> for Gis3dArtifactStorePreparationFactory {
    fn preflight(&self, mutation: &GisTerrainMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document
            || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES)
            || !matches!(mutation, GisTerrainMutation::ChangeExaggeration(payload) if payload.new_exaggeration.is_finite())
        {
            return Err("GIS terrain Artifact preparation rejected its lane, description, or mutation".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: 8 })
    }
    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<GisTerrainSnapshot, GisTerrainMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<GisTerrainSnapshot, GisTerrainMutation>>, store::ArtifactStoreOneItemPreparationRequest<GisTerrainSnapshot, GisTerrainMutation>> {
        begin_gis3d_preparation(request, prepare_gis3d_artifact)
    }
}

impl store::ArtifactStoreOneItemPreparationFactory<Gis3dConfig, Gis3dConfigMutation> for Gis3dConfigStorePreparationFactory {
    fn preflight(&self, mutation: &Gis3dConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("GIS terrain Config preparation rejected its lane or description".into());
        }
        let retained_bytes = match mutation {
            Gis3dConfigMutation::SetCamera(SetCamera { camera_json }) if camera_json.len() <= GIS3D_RETAINED_RAW_BYTES && serde_json::from_str::<Value>(camera_json).is_ok_and(|camera| camera.is_object()) => camera_json.len(),
            _ => return Err("GIS terrain Config preparation rejected its exact mutation".into()),
        };
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes })
    }
    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<Gis3dConfig, Gis3dConfigMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Gis3dConfig, Gis3dConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<Gis3dConfig, Gis3dConfigMutation>> {
        begin_gis3d_preparation(request, prepare_gis3d_config)
    }
}
//#endregion 📬️StorePreparation

impl ArtifactEditor for Gis3dPlayApp {
    type Snapshot = GisTerrainSnapshot;
    type Mutation = GisTerrainMutation;
    type Config = Gis3dConfig;
    type ConfigMutation = Gis3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::gis3d::presence::Gis3dPresence;
    type PresenceMutation = crate::editor::gis3d::presence::Gis3dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Gis3dCommand;

    const DIALECT: Dialect = crate::GISTERRAIN_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GIS_3D_TERRAIN_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Gis3dArtifactStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Gis3dConfigStorePreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Gis3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.gis.gisterrain@1/*#editor",
        document_schema: "gis.terrain",
        factory: "Gis3dCommandJobFactory",
        factory_type: Gis3dCommandJobFactory,
        tools: {
            "setExaggeration" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
            "setCamera" => ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
        }
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Gis3dCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !GIS3D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("gis3d-command-tool-mismatch"));
        }
        if gis3d_retained_extent(&request.command, &request.snapshot, &request.interaction_state).is_none() {
            return Err(Fault::from("gis3d-command-payload-too-large"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(BoundedArtifactCommandWork::new(tool_id, gis3d_retained_reduce, gis3d_retained_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation: operation_context, completion: request.completion },
            Gis3dCommand::command_id,
            GIS3D_RETAINED_RAW_BYTES,
            GIS3D_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::gis3d::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> GisTerrainSnapshot {
        default_terrain_document()
    }

    /// 🔌️ `map:in`/`scene:out` (WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2 port recipe) plus the implicit
    /// document ports.
    fn io() -> Option<AppIo> {
        Some(gis3d_io())
    }

    /// 🎞️ `scene:out` (see `gis3d_scene_media` in `🔖️Io` above) plus the inherited
    /// `document:out` default (the pack of `doc.snapshot`, replicated inline — overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, GisTerrainSnapshot>) -> Result<Media, MediaError> {
        match port {
            "scene:out" => Ok(gis3d_scene_media(doc.snapshot)),
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ `map:in` writes the incoming `2d.map` descriptor JSON verbatim into
    /// `GisTerrainSnapshot::imported_features_json` (rendered as an extra pin layer, see the
    /// 🏔️terrain window) via `change-imported-features`. `document:in` (whole-document replace) is
    /// deliberately unimplemented — per the semantic-mutations taxonomy, whole-document replace has
    /// no in-history mutation; it goes through `ArtifactStore::reset` (file-open/import/load-example),
    /// entirely outside this method.
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, GisTerrainSnapshot>) -> Result<Emit<GisTerrainMutation, Gis3dConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "map:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "map:in only accepts a Structured JSON payload".into()));
                };
                use crate::mutations::change_imported_features::ChangeImportedFeatures;
                Ok(Emit::mutations(vec![GisTerrainMutation::ChangeImportedFeatures(ChangeImportedFeatures { new_imported_features_json: json.clone() })]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn command_id(command: &Gis3dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Gis3dCommand` — React/wgpu still speak the stringly
    /// `{action,args}` wire; this is the typed-command bridge until those call sites send `OpBinary`
    /// bytes directly. Mirrors `crate::editor::gis2d`'s arg-key tolerance (camelCase + snake_case + the
    /// nested `camera` object form).
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        let args = args.map_or(Value::Null, Value::from);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        match action {
            "setExaggeration" => Ok(Gis3dCommand::SetExaggeration(set_exaggeration::SetExaggeration { exaggeration: ["exaggeration", "value"].iter().find_map(|key| args.get(key).and_then(Value::as_f64)).unwrap_or(1.0) })),
            "setCamera" => {
                let camera_json = str_arg(&["cameraJson", "camera_json"]).or_else(|| args.get("camera").map(|value| if value.is_string() { value.as_str().unwrap_or("{}").to_string() } else { value.to_string() })).unwrap_or_else(|| "{}".into());
                if camera_json.len() > GIS3D_RETAINED_RAW_BYTES {
                    return Err(Fault::from("gis3d-command-payload-too-large"));
                }
                Ok(Gis3dCommand::SetCamera(set_camera::SetCamera { camera_json }))
            }
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    fn handle(
        command: &Gis3dCommand,
        doc: &ArtifactView<'_, GisTerrainSnapshot>,
        cfg: &ConfigView<'_, Gis3dConfig>,
        _interaction: &InteractionView<'_>, _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<GisTerrainMutation, Gis3dConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🧮️ Empty — gis3d's `Config` is session view state (camera), not a user-facing settings
    /// record; `ConfigSpec::empty()` (the trait default) is correct as-is.
    fn config_spec() -> semio_framework_plugin::ConfigSpec {
        semio_framework_plugin::ConfigSpec::default()
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, GisTerrainSnapshot>, cfg: &ConfigView<'_, Gis3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            terrain::GIS3D_PLAY_BODY_COMPOSITE => terrain::render(doc.snapshot, cfg.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Gis3dPlayApp

//#region 🔖️Manifest
pub fn create_gis3d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::GISTERRAIN_DIALECT)
            .document(["semio", "gis", "3d"])
            // 🔌️ Declared for clarity on both sides of the `map:in` edge (WORKFLOWS-END-TO-END-TYPED-PORTS
            // Wave 2 port recipe) — the canonical declaration is the gismap artifact's;
            // identical-shape duplicates are harmless (registry dedupes by id).
            .artifact_kind(semio_s_artifact_gis_gismap::artifact_kind())
            // 🧱️ `.artifact_kind(mesh_artifact_kind())` REMOVED — `3d.mesh` duplicate kind deleted
            // repo-wide (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`); mesh is now
            // canonically `s.stdio.semio@v1/mesh`, composed via `GisTerrainSnapshot.mesh`.
            .media_input(gis3d_map_in_port())
            .media_output(gis3d_scene_out_port())
            .icon_id("gis3d")
            .mode_def(view_mode::definition())
            .default_mode_id(view_mode::GIS3D_PLAY_MODE_VIEW)
            .window_kind_def(terrain::definition())
            .default_layout(view_mode::layout())
            // 🕹️ The framework-owned "features" interaction domain (ticket
            // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — the imported-overlay pin
            // selection; auto-injects interactionSelect/interactionHover/clearSelection/selectAll/
            // setSelectionMode/setInteractionGranularity, replacing the deleted bespoke
            // setSelection/worldSelect view actions below.
            .interaction(InteractionDefinition {
                id: "features".into(),
                label: LocalizedLabel::native("Features", "Objekte"),
                granularities: vec![GranularityDefinition { id: "pin".into(), label: LocalizedLabel::native("Pin", "Stift"), icon_id: "map-pin".into() }],
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
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .mutation("setExaggeration", LocalizedLabel::native("Set Exaggeration", "Überhöhung festlegen"))
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setExaggeration", InteractiveJobClassification::Migrated)
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .config(Gis3dPlayApp::config_spec())
            .io(gis3d_io())
            .interactive_jobs(InteractiveJobClassification::Migrated)
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder::build_definition` has no `.example(...)`/
            // `.workflow(...)` — the old `"reuse-terrain"` app-level example registration and the
            // no-op `.workflow("gis3d", …)` call are dropped here (not silently: reported in the
            // migration notes). The subset's own `📚️examples/🎬️demo` facet is the modern replacement.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
#[cfg(test)]
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
