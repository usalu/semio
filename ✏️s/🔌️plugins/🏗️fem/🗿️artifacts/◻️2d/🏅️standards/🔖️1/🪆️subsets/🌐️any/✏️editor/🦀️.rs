//! 🖥️ Fem2d play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, view state in `🎚️config`, shared compute in the artifact's `⚙️engine`.
//! This file is a routing table: `handle` → `Fem2dCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one passthrough per node (fem2d's mode/window declarations stay
//! scalar/inline — no `mode_def`/`window_kind_def` object is built anywhere in the pre-migration code).

use crate::app_surface::{DisplayMode, ResultDisplay};
use crate::artifacts::fem2d::op::Fem2dMutation;
use crate::artifacts::fem2d::Fem2dSnapshot;
use crate::editor::fem2d::commands::{
    add_area_load, add_bar, add_beam, add_combination, add_load_case, add_material, add_member_udl, add_nodal_load, add_node, add_region, add_section, add_support, remove_selection, set_active_example, set_analysis_settings, set_camera, set_locale,
    set_result_display, set_self_weight,
};
use crate::editor::fem2d::config::{Fem2dConfig, Fem2dConfigMutation};
use crate::editor::fem2d::modes::edit;
use crate::editor::fem2d::modes::edit::windows::model as model_window;
use crate::editor::fem2d::modes::edit::windows::results as results_window;
use crate::model::{Dof, ElementResult};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::{
    built_text_node, create_default_layout, ActionArgDef, ActionArgOption, AppIo, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract,
    ArtifactToolPublicationLane, ArtifactView, ConfigSpec, ConfigView,
    DraftView, Editor, EditorApp, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, NoDraft, NoDraftMutation, PluginCloseStep,
};
use dsl::json::Value;
use std::collections::HashMap;
use store::EngineHandles;

//#region 🔖️Constants
pub const FEM2D_APP_ID: &str = "fem2d-play";

/// 📦️ The `fem2d-play` "default" example — read directly by the `setActiveExample` handler
/// (`crate::editor::fem2d::commands::set_active_example`) and every test fixture (`EditorBuilder` has
/// no `.example(...)` registration — see the SDK-gap doc comment on `create_fem2d_app` below).
pub const FEM2D_EXAMPLE_DSL: &str = crate::artifacts::fem2d::dsl::FEM2D_EXAMPLE_TEXT;
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Fem2dPlayApp::Command` — the SOLE dispatch surface for fem2d's own behavior, assembled from
    /// the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`,
    /// the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the codec uses) — genuinely different vocabularies; `"setActiveExample" as
    /// "active-example"` and `"setCamera" as "camera"` are two of the rows that prove it. **Row order is
    /// the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum Fem2dCommand for Fem2dSnapshot, Fem2dMutation, Fem2dConfig, Fem2dConfigMutation {
        "addNode" as "add-node" => add_node::AddNode,
        "addBar" as "add-bar" => add_bar::AddBar,
        "addBeam" as "add-beam" => add_beam::AddBeam,
        "addMaterial" as "add-material" => add_material::AddMaterial,
        "addSection" as "add-section" => add_section::AddSection,
        "addSupport" as "add-support" => add_support::AddSupport,
        "addNodalLoad" as "add-nodal-load" => add_nodal_load::AddNodalLoad,
        "addMemberUdl" as "add-member-udl" => add_member_udl::AddMemberUdl,
        "addAreaLoad" as "add-area-load" => add_area_load::AddAreaLoad,
        "addRegion" as "add-region" => add_region::AddRegion,
        "addLoadCase" as "add-load-case" => add_load_case::AddLoadCase,
        "addCombination" as "add-combination" => add_combination::AddCombination,
        "setSelfWeight" as "set-self-weight" => set_self_weight::SetSelfWeight,
        "setAnalysisSettings" as "set-analysis-settings" => set_analysis_settings::SetAnalysisSettings,
        "removeSelection" as "remove-selection" => remove_selection::RemoveSelection,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setResultDisplay" as "result-display" => set_result_display::SetResultDisplay,
        "setLocale" as "locale" => set_locale::SetLocale,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
/// 🧵️ Every `Fem2dCommand` row, without exception — fem2d declares no host-only verb, so the retained
/// route table and `create_fem2d_app`'s `Migrated` classification list are the same nineteen ids
/// (pinned by `retained_routes_cover_every_command_exactly_once`).
const FEM2D_RETAINED_TOOL_IDS: &[&str] = &[
    "addNode", "addBar", "addBeam", "addMaterial", "addSection", "addSupport", "addNodalLoad", "addMemberUdl", "addAreaLoad", "addRegion", "addLoadCase", "addCombination", "setSelfWeight", "setAnalysisSettings", "removeSelection", "setActiveExample", "setCamera",
    "setResultDisplay", "setLocale",
];
const FEM2D_RETAINED_PAYLOAD_SCHEMA: &str = "fem.2d.tool-command.v1";
const FEM2D_RETAINED_RAW_BYTES: usize = 65_536;
const FEM2D_RETAINED_WORK_ITEMS: usize = 4_096;
/// 🧮️ The largest document a retained fem2d route will reduce over in one bounded first step — the
/// same number the artifact-lane store preparation admits, so a document too large for the reducer is
/// rejected before any authority is claimed rather than mid-publication.
const FEM2D_MAXIMUM_DOCUMENT_ITEMS: usize = 4_096;
/// 🛣️ Publication lanes per route: the fifteen structural editors emit `Fem2dMutation`s only, while
/// `setActiveExample` replaces the whole document through a non-history `Effect::LoadDocument` and
/// publishes ONLY its two granular config resets (`SetResultDisplay`, `SetCamera`) — effects are not a
/// store lane, so its contract is `Config`, exactly like the three view actions.
const FEM2D_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "addNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addBar", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addBeam", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addMaterial", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addSection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addSupport", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addNodalLoad", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addMemberUdl", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addAreaLoad", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addRegion", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addLoadCase", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addCombination", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setSelfWeight", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setAnalysisSettings", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setResultDisplay", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[ArtifactToolPublicationLane::Config] },
];

fn fem2d_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(FEM2D_RETAINED_RAW_BYTES, FEM2D_RETAINED_WORK_ITEMS, 1, 262_144, 7_500)
}

/// 🧮️ Every fem2d route reduces in exactly one bounded step, admitted only while the live document
/// stays inside `FEM2D_MAXIMUM_DOCUMENT_ITEMS` — an oversized document faults as "exceeds semantic
/// work capacity" instead of silently blowing the step budget.
fn fem2d_document_items(snapshot: &Fem2dSnapshot) -> Option<usize> {
    [snapshot.nodes.len(), snapshot.elements.len(), snapshot.regions.len(), snapshot.materials.len(), snapshot.sections.len(), snapshot.supports.len(), snapshot.load_cases.len(), snapshot.combinations.len()]
        .into_iter()
        .try_fold(1usize, |total, count| total.checked_add(count))
}

fn fem2d_retained_extent(command: &Fem2dCommand, snapshot: &Fem2dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if !FEM2D_RETAINED_TOOL_IDS.contains(&command.command_id()) {
        return None;
    }
    fem2d_document_items(snapshot).filter(|items| *items <= FEM2D_MAXIMUM_DOCUMENT_ITEMS).map(|_| 1)
}

fn fem2d_retained_reduce(
    command: &Fem2dCommand,
    snapshot: &Fem2dSnapshot,
    config: &Fem2dConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &AppOperationContext,
) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation, NoDraftMutation>, Fault> {
    if !FEM2D_RETAINED_TOOL_IDS.contains(&command.command_id()) { return Err(Fault::from("fem2d-command-retained-route-rejected")); }
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config })
}

struct Fem2dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Fem2dRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: FEM2D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for Fem2dRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<Fem2dPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<Fem2dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        FEM2D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        fem2d_retained_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > FEM2D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("FEM2D retained command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for Fem2dRetainedCommandJobFactory {
    type Owner = EditorApp<Fem2dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FEM2D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::fem2d::FEM_2D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = FEM2D_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 📬️ConfigStorePreparation
const FEM2D_CONFIG_TEXT_MAXIMUM_BYTES: usize = 128;
const FEM2D_CONFIG_PUBLICATION_MAXIMUM_BYTES: usize = 4_096;

//#region 🎟️Admission
fn fem2d_config_text_bytes(config: &Fem2dConfig) -> usize {
    [config.result_source_id.as_ref().map_or(0, String::len), config.result_mode.len(), config.locale.len()].into_iter().fold(0usize, usize::saturating_add)
}

fn fem2d_config_publication_bytes(mutation: &Fem2dConfigMutation) -> Result<usize, String> {
    let bytes = match mutation {
        Fem2dConfigMutation::Snapshot { config } => fem2d_config_text_bytes(config),
        Fem2dConfigMutation::SetResultDisplay { source_id, mode, .. } => source_id.as_ref().map_or(0, String::len).saturating_add(mode.len()),
        Fem2dConfigMutation::SetCamera { .. } => 0,
        Fem2dConfigMutation::SetLocale { value } => value.len(),
    };
    if bytes > FEM2D_CONFIG_TEXT_MAXIMUM_BYTES { return Err("fem2d-config-text-envelope".into()); }
    Ok(FEM2D_CONFIG_PUBLICATION_MAXIMUM_BYTES)
}

struct Fem2dConfigPreparationFactory;

impl store::ArtifactStoreOneItemPreparationFactory<Fem2dConfig, Fem2dConfigMutation> for Fem2dConfigPreparationFactory {
    fn preflight(&self, mutation: &Fem2dConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > 64) {
            return Err("fem2d-config-lane-or-description-envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: fem2d_config_publication_bytes(mutation)? })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<Fem2dConfig, Fem2dConfigMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Fem2dConfig, Fem2dConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<Fem2dConfig, Fem2dConfigMutation>> {
        if request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > 64 || self.preflight(&request.mutation, request.description.as_deref(), request.lane).is_err() || fem2d_config_text_bytes(request.base.get()) > FEM2D_CONFIG_TEXT_MAXIMUM_BYTES {
            return Err(request);
        }
        Ok(Box::new(Fem2dConfigPreparation {
            base: Some(request.base), mutation: Some(request.mutation), description: request.description, authority: Some(request.authority), prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false,
        }))
    }
}
//#endregion 🎟️Admission

//#region 🧵️Preparation
struct Fem2dConfigPreparation {
    base: Option<store::SnapshotRead<Fem2dConfig>>,
    mutation: Option<Fem2dConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Fem2dConfig, Fem2dConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparation<Fem2dConfig, Fem2dConfigMutation> for Fem2dConfigPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || grant.maximum_bytes < FEM2D_CONFIG_PUBLICATION_MAXIMUM_BYTES || self.cancelled || self.closing { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.checkpoint.cursor != 0 { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint)); }
        let base = self.base.as_ref().ok_or_else(|| "fem2d-config-base-owner-missing".to_string())?;
        let mutation = self.mutation.as_ref().ok_or_else(|| "fem2d-config-mutation-owner-missing".to_string())?;
        let mut next = base.get().clone();
        let inverse = match mutation {
            Fem2dConfigMutation::Snapshot { config } => {
                next = config.clone();
                Fem2dConfigMutation::Snapshot { config: base.get().clone() }
            }
            Fem2dConfigMutation::SetResultDisplay { source_id, mode, mode_index } => {
                next.result_source_id = source_id.clone(); next.result_mode = mode.clone(); next.result_mode_index = *mode_index;
                Fem2dConfigMutation::SetResultDisplay { source_id: base.get().result_source_id.clone(), mode: base.get().result_mode.clone(), mode_index: base.get().result_mode_index }
            }
            Fem2dConfigMutation::SetCamera { camera } => { next.camera = camera.clone(); Fem2dConfigMutation::SetCamera { camera: base.get().camera.clone() } }
            Fem2dConfigMutation::SetLocale { value } => { next.locale = value.clone(); Fem2dConfigMutation::SetLocale { value: base.get().locale.clone() } }
        };
        if fem2d_config_text_bytes(&next) > FEM2D_CONFIG_TEXT_MAXIMUM_BYTES { return Err("fem2d-config-post-text-envelope".into()); }
        let authority = self.authority.as_ref().ok_or_else(|| "fem2d-config-authority-missing".to_string())?;
        let id = format!("fem2d-config-{}", authority.next_sequence_number());
        let edit = protocol::Edit {
            id: id.clone(), actor: Some(authority.actor().to_string()), forwards: vec![mutation.clone()], inverse: vec![inverse],
            mutation_meta: vec![protocol::MutationMeta {
                mutation_id: Some(protocol::MutationId(format!("{id}#0"))), dependencies: Vec::new(), base_version: authority.base_applied_edit_count() as u64,
                author_id: Some(protocol::ActorId(authority.actor().to_string())), timestamp: authority.next_clock(), undo_policy: protocol::UndoPolicy::ExactBaseOnly,
                payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default(),
            }],
            description: self.description.clone(), coalesce_key: None, sequence_number: authority.next_sequence_number(), started_at: String::new(), finished_at: None,
        };
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(next))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: FEM2D_CONFIG_PUBLICATION_MAXIMUM_BYTES as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Fem2dConfig, Fem2dConfigMutation>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Fem2dConfig, Fem2dConfigMutation>> { self.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 || grant.maximum_bytes < FEM2D_CONFIG_PUBLICATION_MAXIMUM_BYTES { return Ok(store::SnapshotRetirementStep::Blocked); }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: FEM2D_CONFIG_PUBLICATION_MAXIMUM_BYTES });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() { return Err("fem2d-config-base-retirement-rejected".into()); }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.authority.take().is_some() { return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES }); }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 🧵️Preparation
//#region 🧪️PreparationLaws
#[cfg(test)]
mod fem2d_config_preparation_laws {
    use super::*;
    use store::{ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory};

    #[test]
    fn admitted_maximum_and_production_grant_make_bounded_progress() {
        let factory = Fem2dConfigPreparationFactory;
        let maximum = Fem2dConfigMutation::SetLocale { value: "x".repeat(FEM2D_CONFIG_TEXT_MAXIMUM_BYTES) };
        assert_eq!(factory.preflight(&maximum, None, store::HistoryLane::Document).expect("maximum admission").retained_bytes, 4_096);
        let overflow = Fem2dConfigMutation::SetLocale { value: "x".repeat(FEM2D_CONFIG_TEXT_MAXIMUM_BYTES + 1) };
        assert!(factory.preflight(&overflow, None, store::HistoryLane::Document).is_err());
        assert!(factory.preflight(&maximum, Some(&"x".repeat(65)), store::HistoryLane::Document).is_err());
        let mut work = Fem2dConfigPreparation {
            base: None, mutation: Some(maximum), description: None, authority: None, prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false,
        };
        assert!(matches!(work.advance(store::ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 4_096 }), Ok(store::ArtifactStoreOneItemPreparationStep::Blocked)));
        assert!(matches!(work.advance(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_095 }), Ok(store::ArtifactStoreOneItemPreparationStep::Blocked)));
        work.cancel();
        assert!(matches!(work.advance(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 }), Ok(store::ArtifactStoreOneItemPreparationStep::Blocked)));
        work.begin_close();
        assert!(matches!(work.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 0 }), Ok(store::SnapshotRetirementStep::Blocked)));
        assert!(work.mutation.is_some());
        assert!(matches!(work.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 }), Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 4_096 })));
        assert!(work.terminal_is_empty());
        assert!(matches!(work.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 }), Ok(store::SnapshotRetirementStep::Complete)));
    }
}
//#endregion 🧪️PreparationLaws
//#endregion 📬️ConfigStorePreparation

//#region 📬️ArtifactStorePreparation
/// 📬️ The document lane's one-item publication authority — the counterpart of
/// `Fem2dConfigPreparationFactory` above. Without it every `Artifact`-lane route is refused at
/// registration with `interactive-job.publication-authority-missing`, which is why fem2d's fifteen
/// structural editors could not be retained before.
struct Fem2dArtifactPreparationFactory;

struct Fem2dArtifactPreparation {
    base: Option<store::SnapshotRead<Fem2dSnapshot>>,
    mutation: Option<Fem2dMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Fem2dSnapshot, Fem2dMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Fem2dSnapshot, Fem2dMutation> for Fem2dArtifactPreparationFactory {
    fn preflight(&self, _mutation: &Fem2dMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("fem2d-artifact-lane-or-description-envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Fem2dSnapshot, Fem2dMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Fem2dSnapshot, Fem2dMutation>>, store::ArtifactStoreOneItemPreparationRequest<Fem2dSnapshot, Fem2dMutation>> {
        let admitted = fem2d_document_items(request.base.get()).is_some_and(|items| items <= FEM2D_MAXIMUM_DOCUMENT_ITEMS);
        if !admitted
            || request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || self.preflight(&request.mutation, request.description.as_deref(), request.lane).is_err()
        {
            return Err(request);
        }
        Ok(Box::new(Fem2dArtifactPreparation {
            base: Some(request.base), mutation: Some(request.mutation), description: request.description, authority: Some(request.authority), prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Fem2dSnapshot, Fem2dMutation> for Fem2dArtifactPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "fem2d-artifact-base-owner-missing".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "fem2d-artifact-mutation-owner-missing".to_string())?;
        let inverse = mutation.inverse(base.get());
        let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "fem2d-artifact-authority-missing".to_string())?;
        let id = format!("fem2d-retained-{}", authority.next_sequence_number());
        let edit = protocol::Edit {
            id: id.clone(), actor: Some(authority.actor().to_string()), forwards: vec![mutation], inverse,
            mutation_meta: vec![protocol::MutationMeta {
                mutation_id: Some(protocol::MutationId(format!("{id}#0"))), dependencies: Vec::new(), base_version: authority.base_applied_edit_count() as u64,
                author_id: Some(protocol::ActorId(authority.actor().to_string())), timestamp: authority.next_clock(), undo_policy: protocol::UndoPolicy::ExactBaseOnly,
                payload_hash: None, semantic_kind: None, label: None, group_id: None, origin: Default::default(),
            }],
            description: self.description.take(), coalesce_key: None, sequence_number: authority.next_sequence_number(), started_at: String::new(), finished_at: None,
        };
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Fem2dSnapshot, Fem2dMutation>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Fem2dSnapshot, Fem2dMutation>> { self.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() { return Err("fem2d-artifact-base-retirement-rejected".into()); }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() { return Ok(store::SnapshotRetirementStep::Blocked); }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}

//#region 🧪️ArtifactPreparationLaws
#[cfg(test)]
mod fem2d_artifact_preparation_laws {
    use super::*;
    use store::{ArtifactStoreOneItemPreparation, ArtifactStoreOneItemPreparationFactory};

    #[test]
    fn admitted_document_mutations_make_bounded_progress_and_retire() {
        let factory = Fem2dArtifactPreparationFactory;
        let mutation = Fem2dMutation::CreateNode(crate::artifacts::fem2d::mutations::create_node::CreateNode { node: crate::artifacts::fem2d::FemNode { id: "n1".into(), x: 0.0, y: 0.0 } });
        assert_eq!(factory.preflight(&mutation, None, store::HistoryLane::Document).expect("document admission").work_items, 1);
        assert!(factory.preflight(&mutation, Some(&"x".repeat(store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES + 1)), store::HistoryLane::Document).is_err());
        let mut work = Fem2dArtifactPreparation {
            base: None, mutation: Some(mutation), description: None, authority: None, prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(), cancelled: false, closing: false,
        };
        assert!(matches!(work.advance(store::ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 1_048_576 }), Ok(store::ArtifactStoreOneItemPreparationStep::Blocked)));
        work.cancel();
        assert!(matches!(work.advance(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1_048_576 }), Ok(store::ArtifactStoreOneItemPreparationStep::Blocked)));
        work.begin_close();
        assert!(matches!(work.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 1_048_576 }), Ok(store::SnapshotRetirementStep::Blocked)));
        assert!(matches!(work.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1_048_576 }), Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })));
        assert!(work.terminal_is_empty());
        assert!(matches!(work.close_step(store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1_048_576 }), Ok(store::SnapshotRetirementStep::Complete)));
    }
}
//#endregion 🧪️ArtifactPreparationLaws
//#endregion 📬️ArtifactStorePreparation

//#region 🔖️ExportImportHelpers
/// 👁️ B1: `cfg`-driven counterpart of the deleted `ResultDisplay` `RefCell` — converts the flat
/// `Fem2dConfig` result-display fields back into `crate::app_surface::ResultDisplay`/`DisplayMode` so
/// the results window's render pipeline (built around those shared types) needs no changes.
fn config_result_display(cfg: &Fem2dConfig) -> ResultDisplay {
    let mode = match cfg.result_mode.as_str() {
        "modal" => DisplayMode::Modal(cfg.result_mode_index as usize),
        "buckling" => DisplayMode::Buckling(cfg.result_mode_index as usize),
        _ => DisplayMode::Static,
    };
    ResultDisplay { source_id: cfg.result_source_id.clone(), mode }
}

/// 🎨️ Manual `crate::model::StaticResult` -> JSON bridge for `"results:out"` (see `export_media` below)
/// — `crate::model::StaticResult`/`ElementResult`/`Dof` don't derive `Serialize` (out of this ticket's
/// scope: `🫀️core` is a shared crate), so this hand-rolls the same shape `dsl::json::to_json_string` would
/// have produced, using `Dof`'s existing `{:?}` formatting (already used for the reaction-label layers
/// in the results window's render).
fn dof_json(dof: Dof) -> Value {
    dsl::json!(format!("{dof:?}"))
}

fn element_result_json(result: &ElementResult) -> Value {
    match result {
        ElementResult::Bar { n } => dsl::json!({ "kind": "bar", "n": n }),
        ElementResult::Beam { stations } => {
            dsl::json!({ "kind": "beam", "stations": stations.iter().map(|s| dsl::json!({ "x": s.x, "n": s.n, "v": s.v, "m": s.m })).collect::<Vec<_>>() })
        }
        ElementResult::Plane { gauss } => {
            dsl::json!({ "kind": "plane", "gauss": gauss.iter().map(|g| dsl::json!({ "sxx": g.sxx, "syy": g.syy, "sxy": g.sxy, "vonMises": g.von_mises })).collect::<Vec<_>>() })
        }
        ElementResult::Plate { gauss } => {
            dsl::json!({ "kind": "plate", "gauss": gauss.iter().map(|g| dsl::json!({ "mx": g.mx, "my": g.my, "mxy": g.mxy })).collect::<Vec<_>>() })
        }
        ElementResult::Solid { gauss } => dsl::json!({
            "kind": "solid",
            "gauss": gauss.iter().map(|g| dsl::json!({ "sxx": g.sxx, "syy": g.syy, "szz": g.szz, "sxy": g.sxy, "syz": g.syz, "sxz": g.sxz, "vonMises": g.von_mises })).collect::<Vec<_>>(),
        }),
        ElementResult::Shell { gauss } => dsl::json!({
            "kind": "shell",
            "gauss": gauss.iter().map(|g| dsl::json!({ "nxx": g.nxx, "nyy": g.nyy, "nxy": g.nxy, "mxx": g.mxx, "myy": g.myy, "mxy": g.mxy, "vonMisesTop": g.von_mises_top, "vonMisesBottom": g.von_mises_bottom })).collect::<Vec<_>>(),
        }),
    }
}

fn static_result_json(result: &crate::model::StaticResult) -> Value {
    dsl::json!({
        "displacements": result.displacements.iter().map(|d| dsl::json!({ "nodeId": d.node_id, "values": d.values })).collect::<Vec<_>>(),
        "reactions": result.reactions.iter().map(|r| dsl::json!({ "nodeId": r.node_id, "dof": dof_json(r.dof), "value": r.value })).collect::<Vec<_>>(),
        "elements": result.elements.iter().map(|(id, element_result)| dsl::json!({ "id": id, "result": element_result_json(element_result) })).collect::<Vec<_>>(),
        "checks": { "residualNorm": result.checks.residual_norm, "reactionSum": result.checks.reaction_sum },
    })
}

fn results_map_json(results: &HashMap<String, crate::model::StaticResult>) -> Value {
    Value::Object(results.iter().map(|(id, result)| (id.clone(), static_result_json(result))).collect())
}
//#endregion 🔖️ExportImportHelpers

//#region 🔌️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document port pair
/// (`fem.2d` × 2D-Vector) plus `geometry:in` (importing an externally authored 2D outline as a new
/// `FemRegion` — see `import_media` above) and `results:out` (every load case/combination's solved
/// `crate::model::StaticResult`, pinned to the `computation.fem2d` artifact kind declared in
/// `crate::artifacts::fem2d::computation_artifact_kind` — see `export_media` above). Moved out of the
/// (now deleted) artifact `⚙️engine`: it returns `AppIo`, an app type, so it belongs here.
pub fn fem2d_io() -> AppIo {
    AppIo {
        document_schema: crate::artifacts::fem2d::FEM_2D_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        ports: vec![fem2d_geometry_in_port(), fem2d_results_out_port()],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "2d.fem".into(), name: "FEM 2D".into(), dimension: "2d".into(), component_kind: "fem2d".into() },
    }
}

/// 🔌️ `geometry:in` — an externally authored 2D polygon-with-holes outline, imported as a new
/// `FemRegion`.
pub fn fem2d_geometry_in_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "geometry:in".into(),
        label: "Geometry".into(),
        direction: semio_framework_plugin::MediaPortDirection::In,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
        kind_id: None,
        required: true,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}

/// 🔌️ `results:out` — every load case/combination's solved `crate::model::StaticResult`, pinned to the
/// `computation.fem2d` artifact kind.
pub fn fem2d_results_out_port() -> semio_framework_plugin::MediaPortSpec {
    semio_framework_plugin::MediaPortSpec {
        id: "results:out".into(),
        label: "Results".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        kind_id: Some("computation.fem2d".into()),
        required: false,
        multiplicity: semio_framework::PortMultiplicity::One,
    }
}
//#endregion 🔌️Io

//#region 🔖️ActionArgHelpers
/// 🛡️ Reads one `FemDof` out of an action arg's free text (`"tx"`, `"Ty"`, `"rz"`, …) — the shells
/// stage every declared arg as a JSON scalar, so the typed enum is resolved here rather than by the
/// stringly action wire.
fn fem2d_dof(value: Option<&str>) -> Option<crate::artifacts::fem2d::FemDof> {
    use crate::artifacts::fem2d::FemDof;
    match value?.trim().to_ascii_lowercase().as_str() {
        "tx" => Some(FemDof::Tx),
        "ty" => Some(FemDof::Ty),
        "tz" => Some(FemDof::Tz),
        "rx" => Some(FemDof::Rx),
        "ry" => Some(FemDof::Ry),
        "rz" => Some(FemDof::Rz),
        _ => None,
    }
}

/// 🛡️ Reads `addSupport`'s separator-delimited `fixed` list, defaulting to the pinned support
/// (`tx,ty`) every fem2d fixture starts from when nothing is staged.
fn fem2d_dofs(value: Option<&str>) -> Vec<crate::artifacts::fem2d::FemDof> {
    use crate::artifacts::fem2d::FemDof;
    let parsed: Vec<FemDof> = value.map(|text| text.split([',', ' ', ';']).filter_map(|token| fem2d_dof(Some(token))).collect()).unwrap_or_default();
    if parsed.is_empty() {
        vec![FemDof::Tx, FemDof::Ty]
    } else {
        parsed
    }
}
//#endregion 🔖️ActionArgHelpers

//#region 🔖️Fem2dPlayApp
/// 🧪️ B1: unit struct — every former `Fem2dPlayApp` `RefCell` field (`result_display`, `camera`) plus
/// the deleted `ViewModel::locale` now live in `crate::editor::fem2d::config::Fem2dConfig`, written
/// through `Fem2dConfigMutation`s. v0 design unchanged: results are never persisted or cached —
/// `fem2d_solve`/`fem2d_solve_all` run fresh inside `render()`/`export_media` whenever the results
/// window is drawn or the `"results:out"` port is read.
#[derive(Default)]
pub struct Fem2dPlayApp;

impl ArtifactEditor for Fem2dPlayApp {
    type Snapshot = Fem2dSnapshot;
    type Mutation = Fem2dMutation;
    type Config = Fem2dConfig;
    type ConfigMutation = Fem2dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::fem2d::presence::Fem2dPresence;
    type PresenceMutation = crate::editor::fem2d::presence::Fem2dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Fem2dCommand;

    const DIALECT: Dialect = crate::artifacts::fem2d::FEM2D_DIALECT;

    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::fem2d::FEM_2D_SCHEMA;

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Fem2dConfigPreparationFactory))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Fem2dArtifactPreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Fem2dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs",
        controller: "s.fem.fem2d@1/*#editor",
        document_schema: "fem.2d",
        factory: "Fem2dRetainedCommandJobFactory",
        factory_type: Fem2dRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "addNode",
            "addBar",
            "addBeam",
            "addMaterial",
            "addSection",
            "addSupport",
            "addNodalLoad",
            "addMemberUdl",
            "addAreaLoad",
            "addRegion",
            "addLoadCase",
            "addCombination",
            "setSelfWeight",
            "setAnalysisSettings",
            "removeSelection",
            "setActiveExample",
            "setCamera",
            "setResultDisplay",
            "setLocale"
        ]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Fem2dRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !FEM2D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("fem2d-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(semio_framework_plugin::retained_command::BoundedArtifactCommandWork::new(tool_id, fem2d_retained_reduce, fem2d_retained_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: None, operation: operation_context, completion: request.completion },
            Fem2dCommand::command_id,
            FEM2D_RETAINED_RAW_BYTES,
            FEM2D_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::fem2d::config::schema::app_schema_descriptor())
    }

    /// 🌱️ Boots on the bundled `📚️examples/🎬️demo` document so the playground paints a real structure
    /// at first frame instead of an empty canvas — the same snapshot `Fem2dViewer` already booted on.
    fn initial_snapshot() -> Fem2dSnapshot {
        crate::artifacts::fem2d::schema::default_fem2d_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(fem2d_io())
    }

    fn mounted_job_maintenance_step(instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        Ok(crate::editor::fem2d::session::maintenance_step(instance_id, maximum_items, maximum_bytes))
    }

    fn mounted_job_close_step(instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        Ok(crate::editor::fem2d::session::close_step(instance_id, maximum_items, maximum_bytes))
    }

    fn mounted_jobs_terminal_is_empty(instance_id: u32) -> bool {
        crate::editor::fem2d::session::terminal_is_empty(instance_id)
    }

    fn mounted_job_prepare_snapshot_read(operation: semio_framework_plugin::AppRenderOperationContext, snapshot: &Self::Snapshot) -> bool {
        crate::editor::fem2d::session::prepare_snapshot_read(operation, snapshot)
    }

    /// 🎞️ `"document:out"` reproduces the trait's default whole-document pack (overriding `export_media`
    /// shadows the trait's provided body for every port on this app, not just the new one). `"results:out"`
    /// runs every load case/combination's analysis fresh and returns them as plain JSON text in a
    /// `Structured` payload — `MediaPayload::Structured.json` doesn't require a `pack`-encoded value. A
    /// document with no load cases, or a solve failure, is reported as `MediaError::Payload` rather than
    /// an empty/panicking export.
    fn export_media(port: &str, doc: &ArtifactView<'_, Fem2dSnapshot>) -> Result<Media, MediaError> {
        match port {
            "document:out" => {
                let media_type = fem2d_io().document_media_type;
                let bytes = <Fem2dSnapshot as store::ArtifactPack>::encode_pack(doc.snapshot);
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            "results:out" => {
                if doc.snapshot.load_cases.is_empty() {
                    return Err(MediaError::Payload("results:out".into(), "no load cases defined".into()));
                }
                let results = crate::fem2d_engine::fem2d_solve_all(doc.snapshot).map_err(|error| MediaError::Payload("results:out".into(), error.to_string()))?;
                let json = results_map_json(&results).to_string();
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.fem2d".into(), json } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🧬️ No `whole_document_operation` override on this impl — per `📓️taxonomy.md`, whole-document
    /// replace (`SetSnapshot`) is banned outright with NO replacement mutation, so this falls back to
    /// the trait's own default (`None`).
    ///
    /// 🎞️ `"document:in"` swaps the whole live document via `reset_document_effect` (a
    /// `Effect::LoadDocument`, the sanctioned non-history whole-doc-replace path — see
    /// `reset_document_effect`'s own doc comment) instead of routing through `whole_document_operation`.
    /// `"geometry:in"` decodes a minimal, app-owned `{"outline": [[f64;2]...], "holes": [[[f64;2]...]...]}`
    /// polygon-with-holes contract into a new `FemRegion` via `create-region`, defaulted to the
    /// document's first existing material if any, else an `"unassigned"` placeholder id.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, Fem2dSnapshot>) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "document:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let snapshot = <Fem2dSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                Ok(Emit { effects: vec![reset_document_effect(&snapshot)], ..Default::default() })
            }
            "geometry:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "geometry:in only accepts a Structured JSON payload".into()));
                };
                let value = dsl::json::parse(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let outline: Vec<[f64; 2]> = dsl::FromValue::from_value(dsl::json::to_dsl_value(&value.get("outline").cloned().unwrap_or(Value::Null))).map_err(|error| MediaError::Payload(port.to_string(), format!("outline: {error}")))?;
                let holes: Vec<Vec<[f64; 2]>> = match value.get("holes").cloned() {
                    Some(holes_value) => dsl::FromValue::from_value(dsl::json::to_dsl_value(&holes_value)).map_err(|error| MediaError::Payload(port.to_string(), format!("holes: {error}")))?,
                    None => Vec::new(),
                };
                let material_id = doc.snapshot.materials.first().map_or_else(|| "unassigned".into(), |material| material.id.clone());
                let id = crate::app_surface::next_id(doc.snapshot.regions.iter().map(|r| r.id.clone()), "r");
                let region = crate::artifacts::fem2d::FemRegion { id, name: "Imported Geometry".into(), outline, holes, thickness: 0.02, material_id, mesh_size: 0.25 };
                Ok(Emit::mutations(vec![Fem2dMutation::CreateRegion(crate::artifacts::fem2d::mutations::create_region::CreateRegion { region })]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &Fem2dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + staged args onto `Fem2dCommand` — the React/wgpu shells still speak the
    /// stringly `{action, args}` wire (`ShellHost`'s action pane and its example switcher both do), and
    /// the trait's default rejects every app action outright, so without this bridge none of fem2d's
    /// nineteen declared actions can reach `dispatch`. Every key here is the `ActionArgDef.id` declared
    /// for that action in `🔖️Manifest` below.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        let text = |key: &str| args.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_str).map(str::to_string);
        let number = |key: &str| args.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_f64);
        let flag = |key: &str| args.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_bool);
        let list = |key: &str| args.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_array).map(|items| items.iter().filter_map(dsl::DslValue::as_str).map(str::to_string).collect::<Vec<_>>());
        match action {
            "addNode" => Ok(Fem2dCommand::AddNode(add_node::AddNode { x: number("x").unwrap_or_default(), y: number("y").unwrap_or_default() })),
            "addBar" => Ok(Fem2dCommand::AddBar(add_bar::AddBar { start: text("start").unwrap_or_default(), end: text("end").unwrap_or_default(), material_id: text("materialId").unwrap_or_default(), section_id: text("sectionId").unwrap_or_default() })),
            "addBeam" => Ok(Fem2dCommand::AddBeam(add_beam::AddBeam { start: text("start").unwrap_or_default(), end: text("end").unwrap_or_default(), material_id: text("materialId").unwrap_or_default(), section_id: text("sectionId").unwrap_or_default() })),
            "addMaterial" => Ok(Fem2dCommand::AddMaterial(add_material::AddMaterial { name: text("name").unwrap_or_default(), e: number("e").unwrap_or(2.1e11) })),
            "addSection" => Ok(Fem2dCommand::AddSection(add_section::AddSection { name: text("name").unwrap_or_default(), area: number("area").unwrap_or_default(), iy: number("iy").unwrap_or_default() })),
            "addSupport" => Ok(Fem2dCommand::AddSupport(add_support::AddSupport { node_id: text("nodeId").unwrap_or_default(), fixed: fem2d_dofs(text("fixed").as_deref()) })),
            "addNodalLoad" => Ok(Fem2dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad {
                node_id: text("nodeId").unwrap_or_default(),
                dof: fem2d_dof(text("dof").as_deref()).unwrap_or(crate::artifacts::fem2d::FemDof::Ty),
                value: number("value").unwrap_or_default(),
                case_id: text("caseId").filter(|id| !id.is_empty()),
            })),
            "addMemberUdl" => Ok(Fem2dCommand::AddMemberUdl(add_member_udl::AddMemberUdl {
                element_id: text("elementId").unwrap_or_default(),
                wx: number("wx").unwrap_or_default(),
                wy: number("wy").unwrap_or_default(),
                case_id: text("caseId").filter(|id| !id.is_empty()),
            })),
            "addAreaLoad" => Ok(Fem2dCommand::AddAreaLoad(add_area_load::AddAreaLoad {
                region_id: text("regionId").unwrap_or_default(),
                pressure: number("pressure").unwrap_or_default(),
                case_id: text("caseId").filter(|id| !id.is_empty()),
            })),
            "addRegion" => Ok(Fem2dCommand::AddRegion(add_region::AddRegion {
                x: number("x").unwrap_or_default(),
                y: number("y").unwrap_or_default(),
                width: number("width").unwrap_or_default(),
                height: number("height").unwrap_or_default(),
                material_id: text("materialId").unwrap_or_default(),
                thickness: number("thickness"),
                mesh_size: number("meshSize"),
            })),
            "addLoadCase" => Ok(Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: text("name").unwrap_or_default(), self_weight: flag("selfWeight").unwrap_or(false) })),
            "addCombination" => Ok(Fem2dCommand::AddCombination(add_combination::AddCombination { name: text("name").unwrap_or_default(), terms: Vec::new() })),
            "setSelfWeight" => Ok(Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: text("caseId").unwrap_or_default(), enabled: flag("enabled").unwrap_or(false) })),
            "setAnalysisSettings" => Ok(Fem2dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings {
                modal_count: number("modalCount").map(|value| value.max(0.0) as u32),
                buckling_count: number("bucklingCount").map(|value| value.max(0.0) as u32),
                deformation_scale: number("deformationScale"),
            })),
            "removeSelection" => Ok(Fem2dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: list("ids").unwrap_or_default() })),
            "setActiveExample" => Ok(Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: text("exampleId").or_else(|| text("id")).unwrap_or_default() })),
            "setCamera" => Ok(Fem2dCommand::SetCamera(set_camera::SetCamera { x: number("x").unwrap_or_default(), y: number("y").unwrap_or_default(), zoom: number("zoom").unwrap_or(1.0) })),
            "setResultDisplay" => Ok(Fem2dCommand::SetResultDisplay(set_result_display::SetResultDisplay {
                source_id: text("sourceId").filter(|id| !id.is_empty()),
                mode: text("mode").unwrap_or_else(|| "static".into()),
                mode_index: number("modeIndex").map(|value| value.max(0.0) as u32).unwrap_or_default(),
            })),
            "setLocale" => Ok(Fem2dCommand::SetLocale(set_locale::SetLocale { value: text("value").unwrap_or_else(|| "en-US".into()) })),
            other => Err(Fault::from(format!("action '{other}' is not a declared fem2d action — every app action is dispatched through the typed command channel (see `dispatch_typed_command`)"))),
        }
    }

    fn handle(
        command: &Fem2dCommand,
        doc: &ArtifactView<'_, Fem2dSnapshot>,
        cfg: &ConfigView<'_, Fem2dConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Fem2dMutation, Fem2dConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn pending_effects(doc: &ArtifactView<'_, Fem2dSnapshot>, _cfg: &ConfigView<'_, Fem2dConfig>) -> Vec<semio_framework::kernel::Effect> {
        crate::editor::fem2d::session::reconcile(doc)
    }

    /// 🎯️ Fem2d has no user-visible config defaults to expose (all of `addRegion`'s
    /// `thickness`/`meshSize` defaults are baked directly into its handler, not user-configurable
    /// settings) — declaring `ConfigSpec::empty()` explicitly keeps the typed channel surface
    /// consistent with the sibling apps' convention.
    fn config_spec() -> ConfigSpec {
        ConfigSpec::default()
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Fem2dSnapshot>, cfg: &ConfigView<'_, Fem2dConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let camera = &cfg.snapshot.camera;
        match body_key {
            model_window::BODY_KEY => crate::editor::fem2d::session::with_live_visual(doc.render_operation(), |visual| model_window::render_with_progress(doc.snapshot, camera, visual)),
            results_window::BODY_KEY => results_window::render(doc.snapshot, &config_result_display(cfg.snapshot), camera),
            _ => built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fem2d unknown-body label admission failed")),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }
}
//#endregion 🔖️Fem2dPlayApp

//#region 🔖️ResetDocument
/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `scene` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (file import,
/// load-example). Per `📓️taxonomy.md`, `SetSnapshot` is banned outright with NO replacement
/// mutation: whole-document replace is not expressible as an in-history `Mutation` at all. Every
/// former "replace the whole document" gesture in this package (`import_media`'s `"document:in"`,
/// `commands::set_active_example`) builds this effect instead of an `Emit::mutations([...])`.
/// The spr is a fresh, edit-free op-log for `scene` — a genesis envelope with no history to encode.
pub fn reset_document_effect(scene: &Fem2dSnapshot) -> semio_framework::kernel::Effect {
    let pack = <Fem2dSnapshot as store::ArtifactPack>::encode_pack(scene);
    let envelope = store::create_document_envelope::<Fem2dSnapshot, Fem2dMutation>(crate::artifacts::fem2d::FEM_2D_SCHEMA, "fem2d", scene.clone(), None);
    let spr = semio_framework_plugin::resolve_ready(store::print_document_spr(&envelope)).expect("fem2d document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔖️Manifest
/// 📚️ `AppBuilder` carries no `.example(...)`: an example is declared ONCE as a definition leaf
/// (`📚️examples/🎬️demo`'s `ExampleSource`) and reaches `PluginManifest.examples` through the subset
/// root's `SubsetDeclaration.examples` — see `🪆️subsets/🌐️any/🦀️.rs`. The shell's navbar switcher
/// reads that list and dispatches `setActiveExample { exampleId }` back into this app, which is why
/// `setActiveExample`'s select option below is that same `demo::ID` and why the action is `Migrated`
/// (a `BatchOnlyPendingRewrite` classification would make the switcher a dead control). `.workflow(...)`
/// stays dropped: `WorkflowDefinition` was deleted from framework-core, with no replacement surface.
pub fn create_fem2d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::fem2d::FEM2D_DIALECT)
            .document(["semio", "fem", "fem2d"])
            // 🔌️ The computed-results output artifact (`results:out`'s `kind_id`, see
            // `fem2d_io` above) — deliberately a different `media_type`
            // (`Computation`×`Value`) than the PORT's wire-level `Data`×`Value`.
            .artifact_kind(crate::artifacts::fem2d::computation_artifact_kind())
            .icon_id("fem-app")
            .mode(edit::MODE_ID, LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id(edit::MODE_ID)
            .window_kind(model_window::WINDOW_KIND_ID, LocalizedLabel::native("Model", "Modell"), model_window::BODY_KEY, semio_framework_ui_contract::SurfaceKind::Canvas2d, "fem-model")
            .window_kind(results_window::WINDOW_KIND_ID, LocalizedLabel::native("Results", "Ergebnisse"), results_window::BODY_KEY, semio_framework_ui_contract::SurfaceKind::Canvas2d, "bar-chart-3")
            .default_layout(create_default_layout(
                &[model_window::WINDOW_KIND_ID.into(), results_window::WINDOW_KIND_ID.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Model".into(), "Results".into()]),
            ))
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .action_args("addNode", vec![
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required(),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required(),
            ])
            .mutation("addBar", LocalizedLabel::native("Add Bar", "Stab hinzufügen"))
            .action_args("addBar", vec![
                ActionArgDef::text("start", LocalizedLabel::native("Start Node", "Startknoten")).required(),
                ActionArgDef::text("end", LocalizedLabel::native("End Node", "Endknoten")).required(),
                ActionArgDef::text("materialId", LocalizedLabel::native("Material", "Material")).required(),
                ActionArgDef::text("sectionId", LocalizedLabel::native("Section", "Querschnitt")).required(),
            ])
            .mutation("addBeam", LocalizedLabel::native("Add Beam", "Balken hinzufügen"))
            .action_args("addBeam", vec![
                ActionArgDef::text("start", LocalizedLabel::native("Start Node", "Startknoten")).required(),
                ActionArgDef::text("end", LocalizedLabel::native("End Node", "Endknoten")).required(),
                ActionArgDef::text("materialId", LocalizedLabel::native("Material", "Material")).required(),
                ActionArgDef::text("sectionId", LocalizedLabel::native("Section", "Querschnitt")).required(),
            ])
            .mutation("addMaterial", LocalizedLabel::native("Add Material", "Material hinzufügen"))
            .action_args("addMaterial", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required(),
                ActionArgDef::number("e", LocalizedLabel::native("Young's Modulus", "Elastizitätsmodul")).default_value(&2.1e11),
            ])
            .mutation("addSection", LocalizedLabel::native("Add Section", "Querschnitt hinzufügen"))
            .action_args("addSection", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required(),
                ActionArgDef::number("area", LocalizedLabel::native("Area", "Fläche")).required(),
                ActionArgDef::number("iy", LocalizedLabel::native("Second Moment of Area", "Flächenträgheitsmoment")).required(),
            ])
            .mutation("addSupport", LocalizedLabel::native("Add Support", "Lager hinzufügen"))
            // 🛡️ `fixed` is `Vec<FemDof>`; no `ActionArgDef` control maps to a typed enum list, so the
            // staged form takes the separator-delimited spelling `fem2d_dofs` reads (`"tx,ty"`).
            .action_args("addSupport", vec![
                ActionArgDef::text("nodeId", LocalizedLabel::native("Node", "Knoten")).required(),
                ActionArgDef::text("fixed", LocalizedLabel::native("Fixed Degrees of Freedom", "Gesperrte Freiheitsgrade")).default_value(&"tx,ty"),
            ])
            .mutation("addNodalLoad", LocalizedLabel::native("Add Nodal Load", "Knotenlast hinzufügen"))
            .action_args("addNodalLoad", vec![
                ActionArgDef::text("nodeId", LocalizedLabel::native("Node", "Knoten")).required(),
                ActionArgDef::select("dof", LocalizedLabel::native("Degree of Freedom", "Freiheitsgrad"), vec![
                    ActionArgOption::new("tx", LocalizedLabel::native("Tx", "Tx")),
                    ActionArgOption::new("ty", LocalizedLabel::native("Ty", "Ty")),
                    ActionArgOption::new("rz", LocalizedLabel::native("Rz", "Rz")),
                ])
                .default_value(&"ty"),
                ActionArgDef::number("value", LocalizedLabel::native("Value", "Wert")).required(),
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")),
            ])
            .mutation("addMemberUdl", LocalizedLabel::native("Add Member UDL", "Streckenlast hinzufügen"))
            .action_args("addMemberUdl", vec![
                ActionArgDef::text("elementId", LocalizedLabel::native("Element", "Element")).required(),
                ActionArgDef::number("wx", LocalizedLabel::native("Wx", "Wx")).default_value(&0.0),
                ActionArgDef::number("wy", LocalizedLabel::native("Wy", "Wy")).required(),
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")),
            ])
            .mutation("addAreaLoad", LocalizedLabel::native("Add Area Load", "Flächenlast hinzufügen"))
            .action_args("addAreaLoad", vec![
                ActionArgDef::text("regionId", LocalizedLabel::native("Region", "Bereich")).required(),
                ActionArgDef::number("pressure", LocalizedLabel::native("Pressure", "Druck")).required(),
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")),
            ])
            .mutation("addRegion", LocalizedLabel::native("Add Region", "Bereich hinzufügen"))
            .action_args("addRegion", vec![
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")).required(),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")).required(),
                ActionArgDef::number("width", LocalizedLabel::native("Width", "Breite")).required(),
                ActionArgDef::number("height", LocalizedLabel::native("Height", "Höhe")).required(),
                ActionArgDef::text("materialId", LocalizedLabel::native("Material", "Material")).required(),
                ActionArgDef::number("thickness", LocalizedLabel::native("Thickness", "Dicke")).default_value(&0.02),
                ActionArgDef::number("meshSize", LocalizedLabel::native("Mesh Size", "Netzgröße")).default_value(&0.25),
            ])
            .mutation("addLoadCase", LocalizedLabel::native("Add Load Case", "Lastfall hinzufügen"))
            .action_args("addLoadCase", vec![
                ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required(),
                ActionArgDef::toggle("selfWeight", LocalizedLabel::native("Self Weight", "Eigengewicht")).default_value(&false),
            ])
            // 🎯️ `terms` is `Fem2dCommand::AddCombination`'s typed `Vec<FemCombinationTerm>` — no single
            // `ActionArgDef` control maps to that shape, so the staged form declares `name` only and the
            // action bridge opens an empty combination the term rows are added to afterwards.
            .mutation("addCombination", LocalizedLabel::native("Add Combination", "Kombination hinzufügen"))
            .action_args("addCombination", vec![ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")).required()])
            .mutation("setSelfWeight", LocalizedLabel::native("Set Self Weight", "Eigengewicht festlegen"))
            .action_args("setSelfWeight", vec![
                ActionArgDef::text("caseId", LocalizedLabel::native("Case", "Lastfall")).required(),
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).required(),
            ])
            .mutation("setAnalysisSettings", LocalizedLabel::native("Set Analysis Settings", "Analyseeinstellungen festlegen"))
            .action_args("setAnalysisSettings", vec![
                ActionArgDef::number("modalCount", LocalizedLabel::native("Modal Count", "Anzahl Eigenformen")),
                ActionArgDef::number("bucklingCount", LocalizedLabel::native("Buckling Count", "Anzahl Knickformen")),
                ActionArgDef::number("deformationScale", LocalizedLabel::native("Deformation Scale", "Verformungsmaßstab")),
            ])
            .mutation("removeSelection", LocalizedLabel::native("Remove Selection", "Auswahl entfernen"))
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            // 📚️ The option id is the bundled example's own `ExampleSource` id, because the shell's
            // navbar switcher dispatches `setActiveExample { exampleId }` straight from
            // `PluginManifest.examples` (`ShellHost`'s `dispatchActiveExample`) — a select option that
            // did not match that id could never be reached from the switcher.
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new(
                    crate::artifacts::fem2d::examples::demo::ID,
                    crate::artifacts::fem2d::examples::demo::label(),
                )])
                .default_value(&crate::artifacts::fem2d::examples::demo::ID),
            ])
            .view_action("setResultDisplay", LocalizedLabel::native("Set Result Display", "Ergebnisanzeige festlegen"))
            .action_args("setResultDisplay", crate::app_surface::result_display_action_args())
            .view_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"))
            // 🧵️ Every row is `Migrated`: each one is an owned retained route on
            // `Fem2dRetainedCommandJobFactory` (`FEM2D_RETAINED_TOOL_IDS`) with a real reducer
            // (`fem2d_retained_reduce` → the `🎮️commands/*` handler) and a real publication authority
            // (`Fem2dArtifactPreparationFactory` for the document lane, `Fem2dConfigPreparationFactory`
            // for the config lane) — pinned by `retained_routes_cover_every_command_exactly_once`.
            .action_interactive_job("addNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("addBar", InteractiveJobClassification::Migrated)
            .action_interactive_job("addBeam", InteractiveJobClassification::Migrated)
            .action_interactive_job("addMaterial", InteractiveJobClassification::Migrated)
            .action_interactive_job("addSection", InteractiveJobClassification::Migrated)
            .action_interactive_job("addSupport", InteractiveJobClassification::Migrated)
            .action_interactive_job("addNodalLoad", InteractiveJobClassification::Migrated)
            .action_interactive_job("addMemberUdl", InteractiveJobClassification::Migrated)
            .action_interactive_job("addAreaLoad", InteractiveJobClassification::Migrated)
            .action_interactive_job("addRegion", InteractiveJobClassification::Migrated)
            .action_interactive_job("addLoadCase", InteractiveJobClassification::Migrated)
            .action_interactive_job("addCombination", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelfWeight", InteractiveJobClassification::Migrated)
            .action_interactive_job("setAnalysisSettings", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setResultDisplay", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLocale", InteractiveJobClassification::Migrated)
            // 🎯️ Typed channel surface — `config_spec()`/`fem2d_io()` are this same information's single
            // source of truth, reused here rather than duplicated.
            .config(Fem2dPlayApp::config_spec())
            .io(fem2d_io())
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type Fem2dApp = VcsArtifactApp<EditorApp<Fem2dPlayApp>>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn fem2d_app() -> Fem2dApp {
        semio_framework_plugin::resolve_ready(new_app::<EditorApp<Fem2dPlayApp>>())
    }

    pub async fn dispatch(app: &mut Fem2dApp, command: Fem2dCommand) -> InvocationResult {
        let result = app.dispatch_typed(command, &meta("local")).await.expect("dispatch");
        for effect in &result.requested_effects {
            if let semio_framework_plugin::Effect::LoadDocument { pack, spr } = effect {
                let files = store::ArtifactPackFiles { pack: pack.clone(), spr: spr.clone(), ops: String::new() };
                app.load_document_pack(&files).await.expect("test host applies load-document effect");
            }
        }
        result
    }

    pub fn render(app: &mut Fem2dApp, body_key: &str) -> String {
        semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::resolve_ready(app.render(body_key, None, &ViewModel::default())).expect("render")).expect("fixture projection")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::fem2d::testkit::{fem2d_app, render};
    use semio_framework_plugin::{ArtifactEditor, EditorApp, PluginApp};
    use store::ArtifactDsl;

    //#region 🔖️CommandSurface
    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) fn every_command() -> Vec<Fem2dCommand> {
        vec![
            Fem2dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0 }),
            Fem2dCommand::AddBar(add_bar::AddBar { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "rod".into() }),
            Fem2dCommand::AddBeam(add_beam::AddBeam { start: "n1".into(), end: "n2".into(), material_id: "steel".into(), section_id: "ipe300".into() }),
            Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 }),
            Fem2dCommand::AddSection(add_section::AddSection { name: "HEA200".into(), area: 0.00538, iy: 0.0000369 }),
            Fem2dCommand::AddSupport(add_support::AddSupport { node_id: "n1".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Tx, crate::artifacts::fem2d::FemDof::Ty] }),
            Fem2dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: crate::artifacts::fem2d::FemDof::Ty, value: -5000.0, case_id: Some("live".into()) }),
            Fem2dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: -500.0, case_id: None }),
            Fem2dCommand::AddAreaLoad(add_area_load::AddAreaLoad { region_id: "r1".into(), pressure: 5000.0, case_id: Some("dead".into()) }),
            Fem2dCommand::AddRegion(add_region::AddRegion { x: 0.0, y: 0.0, width: 4.0, height: 2.0, material_id: "steel".into(), thickness: Some(0.02), mesh_size: None }),
            Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false }),
            Fem2dCommand::AddCombination(add_combination::AddCombination {
                name: "ULS".into(),
                terms: vec![crate::artifacts::fem2d::FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, crate::artifacts::fem2d::FemCombinationTerm { case_id: "live".into(), factor: 1.5 }],
            }),
            Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "dead".into(), enabled: true }),
            Fem2dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings { modal_count: Some(5), buckling_count: None, deformation_scale: Some(30.0) }),
            Fem2dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: vec!["n1".into(), "e1".into()] }),
            Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() }),
            Fem2dCommand::SetCamera(set_camera::SetCamera { x: 1.0, y: 2.0, zoom: 1.5 }),
            Fem2dCommand::SetResultDisplay(set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 0 }),
            Fem2dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        ]
    }

    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
    /// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to
    /// hold.
    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 19, "every Fem2dCommand row must be covered by every_command()");
        // 🏷️ Unlike flow's setLocale/flowEvalTick, every one of fem2d's 19 commands (including
        // setLocale) has a real manifest action declaration — see `create_fem2d_app`'s `.view_action`
        // calls.
        let definition = create_fem2d_app();
        for id in ids {
            assert!(definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == id), "command_id {id} must be a declared action");
        }
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 📌️ LAW: the pre-migration command wire format, row for row, INCLUDING both `Option` shapes of
    /// every row whose `None`/`Some` cases encode differently. Every hex string was dumped from the old
    /// `📡️protocol` crate's `Fem2dCommand` before `app_commands!` rebuilt the enum (ticket
    /// `26/08/05/FEM-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`,
    /// `🧪️wire-baseline-before-2d.txt`). Row order is the binary variant ordinal, so a reordering — which
    /// no round-trip law can catch — shows up here as a leading-byte mismatch.
    #[semio_framework_async_macros::async_test]
    async fn every_command_keeps_its_pre_migration_bytes() {
        use protocol::OpBinary;
        let rows: Vec<(&str, Fem2dCommand)> = vec![
            ("010000020005000000000000f03f01050000000000000040", Fem2dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0 })),
            ("010104026d31026e31026e3202733104000601010602020600030603", Fem2dCommand::AddBar(add_bar::AddBar { start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into() })),
            ("010204026d31026e31026e3202733104000601010602020600030603", Fem2dCommand::AddBeam(add_beam::AddBeam { start: "n1".into(), end: "n2".into(), material_id: "m1".into(), section_id: "s1".into() })),
            ("01030105737465656c020006000105000000da7c724842", Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "steel".into(), e: 210e9 })),
            ("01040106697065333030030006000105a4005130630a763f020509c577de9de7153f", Fem2dCommand::AddSection(add_section::AddSection { name: "ipe300".into(), area: 0.005381, iy: 8.356e-5 })),
            ("010501026e31020006000116020002", Fem2dCommand::AddSupport(add_support::AddSupport { node_id: "n1".into(), fixed: vec![crate::artifacts::fem2d::FemDof::Tx, crate::artifacts::fem2d::FemDof::Ty] })),
            ("010602046c697665026e3104000601010a010205000000000088b3c0030600", Fem2dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: crate::artifacts::fem2d::FemDof::Ty, value: -5000.0, case_id: Some("live".into()) })),
            ("010601026e3103000600010a010205000000000088b3c0", Fem2dCommand::AddNodalLoad(add_nodal_load::AddNodalLoad { node_id: "n1".into(), dof: crate::artifacts::fem2d::FemDof::Ty, value: -5000.0, case_id: None })),
            ("010701026531030006000105000000000000000002050000000000407fc0", Fem2dCommand::AddMemberUdl(add_member_udl::AddMemberUdl { element_id: "e1".into(), wx: 0.0, wy: -500.0, case_id: None })),
            ("0108020464656164027231030006010105000000000088b340020600", Fem2dCommand::AddAreaLoad(add_area_load::AddAreaLoad { region_id: "r1".into(), pressure: 5000.0, case_id: Some("dead".into()) })),
            (
                "01090105737465656c060005000000000000000001050000000000000000020500000000000010400305000000000000004004060005057b14ae47e17a943f",
                Fem2dCommand::AddRegion(add_region::AddRegion { x: 0.0, y: 0.0, width: 4.0, height: 2.0, material_id: "steel".into(), thickness: Some(0.02), mesh_size: None }),
            ),
            ("010a01044c697665020006000101", Fem2dCommand::AddLoadCase(add_load_case::AddLoadCase { name: "Live".into(), self_weight: false })),
            (
                "010b0303554c530464656164046c69766502000600010c020d0200060101059a9999999999f53f0d020006020105000000000000f83f",
                Fem2dCommand::AddCombination(add_combination::AddCombination {
                    name: "ULS".into(),
                    terms: vec![crate::artifacts::fem2d::FemCombinationTerm { case_id: "dead".into(), factor: 1.35 }, crate::artifacts::fem2d::FemCombinationTerm { case_id: "live".into(), factor: 1.5 }],
                }),
            ),
            ("010c010464656164020006000102", Fem2dCommand::SetSelfWeight(set_self_weight::SetSelfWeight { case_id: "dead".into(), enabled: true })),
            ("010d000200040502050000000000003e40", Fem2dCommand::SetAnalysisSettings(set_analysis_settings::SetAnalysisSettings { modal_count: Some(5), buckling_count: None, deformation_scale: Some(30.0) })),
            ("010e02026531026e3101000c0206010600", Fem2dCommand::RemoveSelection(remove_selection::RemoveSelection { ids: vec!["n1".into(), "e1".into()] })),
            ("010f010764656661756c7401000600", Fem2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() })),
            ("011000030005000000000000f03f010500000000000000400205000000000000f83f", Fem2dCommand::SetCamera(set_camera::SetCamera { x: 1.0, y: 2.0, zoom: 1.5 })),
            ("0111020464656164056d6f64616c03000600010601020400", Fem2dCommand::SetResultDisplay(set_result_display::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 0 })),
            ("0112010564652d444501000600", Fem2dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() })),
        ];
        for (expected, command) in rows {
            let bytes = command.encode_op().expect("encode");
            assert_eq!(bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>(), expected, "wire bytes changed for {}", command.command_id());
        }
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️RetainedRoutes
    /// 🧵️ LAW: the retained route table, the bounded-first-step proof roster, the publication-lane
    /// contracts and the manifest's `Migrated` classifications are FOUR views of one set — every
    /// declared command, exactly once, with no `BatchOnlyPendingRewrite` survivor. A route missing from
    /// any one of them is a dead action at runtime (`interactive-job.missing-owned-reducer` or
    /// `interactive-job.publication-authority-missing`).
    #[semio_framework_async_macros::async_test]
    async fn retained_routes_cover_every_command_exactly_once() {
        use std::collections::BTreeSet;
        let commands: BTreeSet<&str> = every_command().iter().map(Fem2dCommand::command_id).collect();
        let routes: BTreeSet<&str> = FEM2D_RETAINED_TOOL_IDS.iter().copied().collect();
        assert_eq!(routes.len(), FEM2D_RETAINED_TOOL_IDS.len(), "no duplicate retained route ids");
        assert_eq!(commands, routes, "every declared command is a retained route and vice versa");
        assert_eq!(<Fem2dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), FEM2D_RETAINED_TOOL_IDS.len());
        assert_eq!(Fem2dRetainedCommandJobFactory::PUBLICATION_CONTRACTS.len(), FEM2D_RETAINED_TOOL_IDS.len());
        let definition = create_fem2d_app();
        for tool_id in FEM2D_RETAINED_TOOL_IDS {
            let contract = Fem2dRetainedCommandJobFactory::PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == *tool_id).unwrap_or_else(|| panic!("publication contract for {tool_id}"));
            assert!(!contract.lanes.is_empty(), "{tool_id} publishes into at least one lane");
            let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == *tool_id).unwrap_or_else(|| panic!("action {tool_id} declared"));
            assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{tool_id} must be Migrated to dispatch interactively");
        }
    }

    /// 🛣️ LAW: a route's declared lanes cover exactly what its own handler emits — the framework
    /// refuses a completion whose emitted lane is absent from the contract.
    #[semio_framework_async_macros::async_test]
    async fn every_route_declares_the_lane_its_handler_emits() {
        let snapshot = crate::artifacts::fem2d::schema::default_fem2d_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let config = Fem2dConfig::default();
        let cfg = ConfigView { snapshot: &config };
        for command in every_command() {
            let tool_id = command.command_id();
            let emit = command.dispatch(&doc, &cfg).unwrap_or_else(|error| panic!("{tool_id} dispatches: {error:?}"));
            let lanes = Fem2dRetainedCommandJobFactory::PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == tool_id).expect("publication contract").lanes;
            assert!(emit.artifact_mutations.is_empty() || lanes.contains(&ArtifactToolPublicationLane::Artifact), "{tool_id} emits document mutations without the Artifact lane");
            assert!(emit.config_mutations.is_empty() || lanes.contains(&ArtifactToolPublicationLane::Config), "{tool_id} emits config mutations without the Config lane");
        }
    }

    /// 🎯️ LAW: the stringly `{action, args}` wire every shell speaks resolves to the typed command
    /// with the same id — the trait's default rejects app actions outright, so this bridge is the only
    /// path from a rendered button to `dispatch`.
    #[semio_framework_async_macros::async_test]
    async fn command_from_action_resolves_every_declared_action() {
        let args = dsl::DslValue::Object(vec![
            ("x".into(), dsl::DslValue::float(1.0)),
            ("y".into(), dsl::DslValue::float(2.0)),
            ("exampleId".into(), dsl::DslValue::String(crate::artifacts::fem2d::examples::demo::ID.into())),
        ]);
        for tool_id in FEM2D_RETAINED_TOOL_IDS {
            let command = <Fem2dPlayApp as ArtifactEditor>::command_from_action(*tool_id, Some(&args)).unwrap_or_else(|error| panic!("action {tool_id} must resolve: {error:?}"));
            assert_eq!(command.command_id(), *tool_id);
        }
        assert!(<Fem2dPlayApp as ArtifactEditor>::command_from_action("nope", None).is_err());
        let node = <Fem2dPlayApp as ArtifactEditor>::command_from_action("addNode", Some(&args)).expect("addNode resolves");
        assert_eq!(node, Fem2dCommand::AddNode(add_node::AddNode { x: 1.0, y: 2.0 }));
    }
    //#endregion 🔖️RetainedRoutes

    //#region 🔖️ManifestSanity
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = dsl::json::to_json_string(&create_fem2d_app());
        for id in [model_window::WINDOW_KIND_ID, results_window::WINDOW_KIND_ID] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        assert!(json.contains(edit::MODE_ID), "mode {} missing from the manifest", edit::MODE_ID);
        assert!(json.contains("computation.fem2d"), "artifact kind missing from the manifest");
    }

    #[semio_framework_async_macros::async_test]
    async fn config_spec_declares_no_fields() {
        assert!(Fem2dPlayApp::config_spec().fields.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn manifest_declares_config_io_and_computation_artifact_kind() {
        let definition = create_fem2d_app();
        assert!(definition.config.fields.is_empty());
        assert_eq!(definition.io.document_schema, crate::artifacts::fem2d::FEM_2D_SCHEMA);
        let computation_kind = definition.artifact_kinds.iter().find(|kind| kind.id == "computation.fem2d").expect("computation.fem2d artifact kind declared");
        assert_eq!(computation_kind.media_type.class, MediaClass::Computation);
        assert_eq!(computation_kind.media_type.form, MediaForm::Value);
    }

    #[semio_framework_async_macros::async_test]
    async fn app_io_forwards_the_engine_declared_ports() {
        let io = Fem2dPlayApp::io().expect("io declared");
        assert!(io.ports.iter().any(|port| port.id == "geometry:in"));
        assert!(io.ports.iter().any(|port| port.id == "results:out"));
    }

    /// 🔌️ Wave-1's `required: true` unwired-input enforcement (`validate_edge_kinds`) lives in the run
    /// crate, not here — this test only proves the port DECLARATION is correct; the cross-crate
    /// enforcement is exercised at the run-crate level.
    #[semio_framework_async_macros::async_test]
    async fn fem2d_io_declares_geometry_in_and_results_out_ports() {
        let io = fem2d_io();
        assert_eq!(io.document_schema, crate::artifacts::fem2d::FEM_2D_SCHEMA);
        assert_eq!(io.document_media_type.class, MediaClass::TwoD);
        assert_eq!(io.document_media_type.form, MediaForm::Vector);
        assert_eq!(io.artifact.id, "2d.fem");
        assert_eq!(io.artifact.component_kind, "fem2d");

        let geometry_in = io.ports.iter().find(|port| port.id == "geometry:in").expect("geometry:in declared");
        assert_eq!(geometry_in.direction, semio_framework_plugin::MediaPortDirection::In);
        assert!(geometry_in.required, "geometry:in is a required input port");
        assert_eq!(geometry_in.media_type.class, MediaClass::TwoD);
        assert_eq!(geometry_in.media_type.form, MediaForm::Vector);
        assert_eq!(geometry_in.multiplicity, semio_framework::PortMultiplicity::One);

        let results_out = io.ports.iter().find(|port| port.id == "results:out").expect("results:out declared");
        assert_eq!(results_out.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert!(!results_out.required, "results:out is optional");
        assert_eq!(results_out.kind_id.as_deref(), Some("computation.fem2d"));
        assert_eq!(results_out.media_type.class, MediaClass::Data);
        assert_eq!(results_out.media_type.form, MediaForm::Value);
    }

    /// 🗣️ B1: the manifest itself (not a runtime `cfg.locale`-driven overlay) now carries every
    /// locale's translation via `LocalizedLabel`.
    #[semio_framework_async_macros::async_test]
    async fn manifest_labels_resolve_german_locale_2d() {
        use semio_framework_plugin::{Locale, Terminology};
        let definition = create_fem2d_app();
        let window_model = definition.window_kinds.iter().find(|window| window.id == model_window::WINDOW_KIND_ID).expect("model window kind declared");
        assert_eq!(window_model.label.resolve(Terminology::Native, Locale::En), "Model");
        assert_eq!(window_model.label.resolve(Terminology::Native, Locale::De), "Modell");
        let add_node_action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "addNode").expect("addNode action declared");
        assert_eq!(add_node_action.label.resolve(Terminology::Native, Locale::En), "Add Node");
        assert_eq!(add_node_action.label.resolve(Terminology::Native, Locale::De), "Knoten hinzufügen");
        let set_locale_action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == "setLocale").expect("setLocale action declared");
        assert_eq!(set_locale_action.label.resolve(Terminology::Native, Locale::En), "Set Locale");
        assert_eq!(set_locale_action.label.resolve(Terminology::Native, Locale::De), "Sprache festlegen");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️CrossCutting
    /// 🌱️ LAW: a fresh editor opens on the bundled example, not on an empty canvas — the model window
    /// must have something to paint at first frame.
    #[semio_framework_async_macros::async_test]
    async fn the_editor_boots_on_the_bundled_example_document() {
        let boot = <Fem2dPlayApp as ArtifactEditor>::initial_snapshot();
        assert!(!boot.nodes.is_empty(), "expected the bundled example fixture's nodes");
        assert!(!boot.elements.is_empty(), "expected the bundled example fixture's elements");
        assert_ne!(boot, crate::artifacts::fem2d::schema::empty_fem2d_snapshot());
        assert_eq!(boot, Fem2dSnapshot::parse_dsl(FEM2D_EXAMPLE_DSL).expect("the bundled example parses"));
        let app = fem2d_app();
        assert!(!app.snapshot().expect("snapshot").nodes.is_empty(), "a booted app renders a non-empty document");
    }

    #[semio_framework_async_macros::async_test]
    async fn undo_restores_document_after_add_node() {
        let mut app = fem2d_app();
        let before = app.snapshot().expect("snapshot").nodes.len();
        semio_framework_plugin::testkit::assert_undo_redo_round_trip(
            &mut app,
            Fem2dCommand::AddNode(add_node::AddNode { x: 1.0, y: 1.0 }),
            |app| app.snapshot().expect("snapshot").nodes.len(),
            before,
            before + 1,
        )
        .await;
    }

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        let mut app = fem2d_app();
        assert!(render(&mut app, "fem2d.play.nope").contains("Unknown body"));
    }

    #[semio_framework_async_macros::async_test]
    async fn two_instances_converge_on_disjoint_edits() {
        let (mut instance_a, mut instance_b) = semio_framework_plugin::resolve_ready(semio_framework_plugin::testkit::paired_apps::<EditorApp<Fem2dPlayApp>>("mem://fem2d-convergence"));

        instance_a.dispatch_typed(Fem2dCommand::AddMaterial(add_material::AddMaterial { name: "Steel".into(), e: 2.1e11 }), &semio_framework_plugin::testkit::meta("actor-a")).await.expect("a adds a material");
        instance_b.dispatch_typed(Fem2dCommand::AddNode(add_node::AddNode { x: 5.0, y: 5.0 }), &semio_framework_plugin::testkit::meta("actor-b")).await.expect("b adds a node");

        // A neutral history action always dispatches through the store, which pumps inbound operations first.
        semio_framework_plugin::resolve_ready(instance_a.handle_action("commitCheckpoint", None, &semio_framework_plugin::testkit::meta("actor-a"))).expect("pump a");
        semio_framework_plugin::resolve_ready(instance_b.handle_action("commitCheckpoint", None, &semio_framework_plugin::testkit::meta("actor-b"))).expect("pump b");

        let projection_a = instance_a.snapshot().expect("snapshot a");
        let projection_b = instance_b.snapshot().expect("snapshot b");
        assert!(projection_a.materials.iter().any(|m| m.name == "Steel"), "A keeps its material");
        assert!(projection_a.nodes.iter().any(|n| n.x == 5.0), "A absorbs B's node");
        assert_eq!(projection_a.nodes.len(), projection_b.nodes.len(), "both instances converge to the same node set");
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️ConfigIo
    // (see `🔖️ManifestSanity` above for config/io declaration checks)
    //#endregion 🔖️ConfigIo

    //#region 🔖️ExportImportMedia
    /// 🧬️ Whole-document replace is not an in-history mutation (`SetSnapshot` is banned outright —
    /// see `📓️taxonomy.md`'s forbidden vocabulary), so `import_media("document:in")` now surfaces as a
    /// `Effect::LoadDocument` carrying the replacement document's pack bytes, not an
    /// `artifact_mutations` entry — asserted directly on `Emit` rather than through `app.snapshot()`.
    #[semio_framework_async_macros::async_test]
    async fn export_media_document_out_round_trips_via_import_media_document_in() {
        let _app = Fem2dPlayApp;
        let snapshot: Fem2dSnapshot = Fem2dSnapshot::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let media = Fem2dPlayApp::export_media("document:out", &doc).expect("document:out exports");
        assert_eq!(media.media_type.class, MediaClass::TwoD);
        assert_eq!(media.media_type.form, MediaForm::Vector);
        let empty_projection = crate::artifacts::fem2d::schema::empty_fem2d_snapshot();
        let empty_history = semio_framework_plugin::HistoryView::empty();
        let empty_doc = ArtifactView::new(&empty_projection, &empty_history);
        let emit = Fem2dPlayApp::import_media("document:in", &media, &empty_doc).expect("document:in imports");
        assert!(emit.artifact_mutations.is_empty(), "whole-document replace must not be an artifact_mutations entry");
        let semio_framework::kernel::Effect::LoadDocument { pack, .. } = emit.effects.first().expect("document:in must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <Fem2dSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode loaded document pack");
        assert_eq!(loaded, snapshot);
    }

    #[semio_framework_async_macros::async_test]
    async fn export_media_results_out_returns_json_with_every_case_and_combination() {
        let _app = Fem2dPlayApp;
        let snapshot: Fem2dSnapshot = Fem2dSnapshot::parse_dsl(FEM2D_EXAMPLE_DSL).unwrap();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let media = Fem2dPlayApp::export_media("results:out", &doc).expect("results:out exports");
        assert_eq!(media.media_type.class, MediaClass::Data);
        assert_eq!(media.media_type.form, MediaForm::Value);
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "computation.fem2d");
                let value = dsl::json::parse(&json).expect("results:out payload is valid JSON");
                for case_id in ["dead", "live", "uls"] {
                    let result = value.get(case_id).unwrap_or_else(|| panic!("missing {case_id} in results:out payload: {value}"));
                    assert!(result.get("displacements").is_some());
                    assert!(result.get("reactions").is_some());
                    assert!(result.get("checks").is_some());
                }
            }
            MediaPayload::Binary { .. } => panic!("expected a Structured payload"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn export_media_results_out_errors_when_no_load_cases_are_defined() {
        let _app = Fem2dPlayApp;
        let snapshot = crate::artifacts::fem2d::schema::empty_fem2d_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let error = Fem2dPlayApp::export_media("results:out", &doc).expect_err("no load cases means no results to export");
        match error {
            MediaError::Payload(port, _) => assert_eq!(port, "results:out"),
            other => panic!("expected MediaError::Payload, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn export_media_unknown_port_is_not_implemented() {
        let _app = Fem2dPlayApp;
        let snapshot = crate::artifacts::fem2d::schema::empty_fem2d_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        assert!(matches!(Fem2dPlayApp::export_media("bogus:out", &doc), Err(MediaError::NotImplemented)));
    }

    #[semio_framework_async_macros::async_test]
    async fn import_media_geometry_in_builds_a_new_region_from_the_first_material() {
        let _app = Fem2dPlayApp;
        let mut snapshot = crate::artifacts::fem2d::schema::empty_fem2d_snapshot();
        snapshot.materials.push(crate::artifacts::fem2d::FemMaterial { id: "steel".into(), name: "Steel".into(), e: 2.1e11, nu: 0.3, rho: 7850.0 });
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let payload = dsl::json::to_string(&dsl::json!({ "outline": [[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], "holes": [] }));
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "geometry".into(), json: payload } };
        let emit = Fem2dPlayApp::import_media("geometry:in", &media, &doc).expect("geometry:in imports");
        assert_eq!(emit.artifact_mutations.len(), 1);
        match &emit.artifact_mutations[0] {
            Fem2dMutation::CreateRegion(crate::artifacts::fem2d::mutations::create_region::CreateRegion { region }) => {
                assert_eq!(region.outline, vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]]);
                assert!(region.holes.is_empty());
                assert_eq!(region.material_id, "steel");
            }
            _ => panic!("expected CreateRegion"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn import_media_geometry_in_falls_back_to_unassigned_material_when_none_exists() {
        let _app = Fem2dPlayApp;
        let snapshot = crate::artifacts::fem2d::schema::empty_fem2d_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let payload = dsl::json::to_string(&dsl::json!({ "outline": [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]] }));
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: "geometry".into(), json: payload } };
        let emit = Fem2dPlayApp::import_media("geometry:in", &media, &doc).expect("geometry:in imports");
        match &emit.artifact_mutations[0] {
            Fem2dMutation::CreateRegion(crate::artifacts::fem2d::mutations::create_region::CreateRegion { region }) => assert_eq!(region.material_id, "unassigned"),
            _ => panic!("expected CreateRegion"),
        }
    }
    //#endregion 🔖️ExportImportMedia
}
//#endregion 🧪️Tests
