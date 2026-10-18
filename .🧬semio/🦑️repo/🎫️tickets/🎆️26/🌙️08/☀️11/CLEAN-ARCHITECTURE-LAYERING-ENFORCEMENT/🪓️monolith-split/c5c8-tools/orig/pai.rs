//! 🔌️ The `PluginApp` implementation of the mounted artifact instance with the framework-reserved settle helpers and the maintenance stage constants.

use semio_framework_plugin_ownership::bounded_ownership::{ARTIFACT_LIVE_OUTPUT_SLOTS, PluginLifecycleStep};
use semio_framework_plugin_decl_views::media_contract::{ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES, ArtifactDownloadOutput, ArtifactMediaExportHandle, ArtifactMediaExportPoll};
use semio_framework::{ActionKind, AppRole, Fault, FaultCode, FaultFrom, FaultOrigin, Media, MediaClass, MediaError, MediaFingerprint, MediaForm, MediaPayload, MediaType, MediaWireFormat, ViewModel};
use semio_framework_plugin_decl_emission::{ActionMeta, ChildEmit, Emit};
use semio_framework_plugin_document_media::{ActiveDocumentArchiveLoad, ActiveDocumentArchiveLoadState, MediaArtifact, copy_media_text_parts, whole_document_media};
use semio_framework::kernel::{AppEvent, Effect, FRAMEWORK_RESERVED_JOB_KIND, FixedCommandPage, HistoryPatch, InvocationResult, UiDirtyScope};
#[cfg(any(test, feature = "artifact-app-testing"))]
use std::sync::Arc;
use semio_framework_plugin_decl_app::{ArtifactApp, DOCUMENT_LOAD_BUSY_CODE, DOCUMENT_LOAD_CHANGED_CODE, DOCUMENT_LOAD_FAILED_CODE, DOCUMENT_LOAD_HISTORY_INVALID_CODE, DOCUMENT_LOAD_INCOMPLETE_CODE, DOCUMENT_LOAD_MEMBER_INVALID_CODE, DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, DOCUMENT_LOAD_TOO_LARGE_CODE, DOCUMENT_LOAD_TOO_MANY_MEMBERS_CODE, EphemeralSnapshot, MediaArtifactDescriptor, MediaArtifactError, MediaConsumption, NATURAL_FILE_PORT, TransactionPrepareOutcome, document_load_fault, media_projection_error, natural_file_input_media, whole_document_archive};
use store::os_store::store_schema::{ArtifactCommand, ArtifactEnvelope};
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_operation::original_catalog_constructor::{ArtifactStoreConstructorKind, ArtifactStoreConstructorPhase};
use semio_framework_plugin_decl_app::tool_contract::{ArtifactToolCompletionValue, ArtifactToolPublicContract, ToolOwnerWitness, TypedOperationResultLane};
use semio_framework_plugin_decl_views::{ArtifactView, ConfigView, InteractionView};
use std::collections::{BTreeMap, HashMap};
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_ui_runtime::ComponentTree;
use semio_framework_plugin_composition_publication::{ComposedGestureResult, MountedTypedCommandFullOperationStage};
use semio_framework_ui_layout::layout::{ContextMenuItemSpec, ContextMenuRequest, FRAMEWORK_HISTORY_BODY_KEY, WindowEngagement, WindowMeasure};
use semio_framework_value::{DslValue, RetirementDemand, ValueError};
use semio_framework_plugin_job::{EmitWire, PendingTransaction, TransactionProposalDraft, TypedOperationCompletion, TypedOperationResultPage, TypedOperationResultToken, TypedOperationUiProgress};
use semio_framework_plugin_job::reserved_jobs::FrameworkReservedCommitProgress;
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use crate::{InstanceExtensions, VcsArtifactApp};
use semio_framework_plugin_instance_stores::stores_components::InstanceStores;
use semio_framework::manifest::CommandAddress as ManifestCommandAddress;
use std::mem::ManuallyDrop;
use semio_framework_plugin_instance_docs::docs_components::MediaTransfers;
use store::{MemberFactory, Mutation, SpaceMember};
use semio_framework_plugin_mounted_owner::{MountedOwnerPhaseV1, MountedOwnerTurnV1};
use protocol::OpText;
use semio_framework_plugin_command::PURE_HISTORY_UNAVAILABLE_CODE;
use semio_framework_plugin_peer::{PeerRosterPublication, PresenceRosterAdmission, PresenceRosterOutcome};
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_tool_run::ToolRunTraceCursor;
use semio_framework_plugin_window_transient::window_transient::TransientView;
use semio_framework_plugin_window_config::WindowConfigPack;
use semio_framework_plugin_window_transient::built_to_component_tree;
use store::os_store::document_materialize::create_document_envelope;
use semio_framework_plugin_instance_dispatch::dispatch_emit;
use semio_framework_plugin_reactor_jobs::reserved::drive_framework_reserved_registry_job;
#[cfg(test)]
use semio_framework_plugin_reactor_jobs::reserved::drive_framework_reserved_registry_job_counted;
use semio_framework_plugin_window_kits::{merge_ui_values, validate_ui_dispatch_classification};
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_plugin_instance_render as render;
use semio_framework_plugin_time_travel_panel::command_history_panel_lift::ui_history_panel;
use semio_framework_ui_contract::*;
use semio_framework_plugin_tool_run::TOOL_RUN_TRACE_DELTA_BYTES;

use semio_framework_plugin_instance_commands::commands_instance_root::framework_reserved_action_kind;
use semio_framework_plugin_instance_jobs::jobs_instance_root::OriginalRenderTreeIdentity;

/// 🧪 Guest laws in dependent crates (puzzle 3d) settle Isolated reserved admits the host would.
pub async fn settle_framework_reserved_admission<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, admitted: InvocationResult) -> Result<InvocationResult,semio_framework_job::OriginalTurnFailure> {
    let Some((job, input)) = admitted.requested_effects.iter().find_map(|effect| match effect {
        Effect::SpawnJob { job, kind, input, .. } if kind == FRAMEWORK_RESERVED_JOB_KIND => Some((*job, input.clone())),
        _ => None,
    }) else {
        return Ok(admitted);
    };
    let output=drive_framework_reserved_registry_job(job,input,app.mounted_policy.preparation)?;
    settle_framework_reserved_commit(app,job,Ok(output)).await.map_err(Into::into)
}

/// 🪜️ Most units one reserved commit may take in a settle law — one per composed child or undone
/// edit, well above anything a fixture composes.
pub const FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS: usize = 4_096;

/// 🪜️ Hands a finished reserved spawn-job to `app`'s commit queue and drives the commit one unit
/// per call — the same units the reactor's typed-operation continuation runs across turns — to its
/// outcome.
pub async fn settle_framework_reserved_commit<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, job: u64, output: Result<Vec<u8>, Fault>) -> Result<InvocationResult, Fault> {
    if !app.pending.admit_framework_reserved_commit(&mut app.history, job, output)? {
        return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-spawn-missing"), "framework reserved spawn-job had no pending admit"));
    }
    for _ in 0..FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS {
        app.step_framework_reserved_commit().await?;
        if let Some(outcome) = app.pending.take_framework_reserved_commit_outcome() {
            return outcome;
        }
    }
    Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-commit-stall"), format!("framework reserved commit did not finish within {FRAMEWORK_RESERVED_COMMIT_SETTLE_UNITS} units")))
}

/// 🧪 Browser `driveSpawnedJob` contract: the isolated reserved dummy job is admitted from its original request and driven
/// by funded `step_job` turns to its terminal output, which is then settled through the reserved commit; the turn count is returned.
#[cfg(any(test, feature = "artifact-app-testing"))]
pub async fn drive_framework_reserved_spawn_like_host<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static>(app: &mut VcsArtifactApp<A, M>, admitted: InvocationResult) -> Result<(InvocationResult,u32),semio_framework_job::OriginalTurnFailure> {
    let Some((job, input)) = admitted.requested_effects.iter().find_map(|effect| match effect {
        Effect::SpawnJob { job, kind, input, .. } if kind == FRAMEWORK_RESERVED_JOB_KIND => Some((*job, input.clone())),
        _ => None,
    }) else {
        return Ok((admitted, 0));
    };
    let (output, steps) = drive_framework_reserved_registry_job_counted(job, input, app.mounted_policy.preparation)?;
    let settled = settle_framework_reserved_commit(app, job, Ok(output)).await?;
    Ok((settled, steps))
}

/// 🪪️ A document loaded from its text or pack belongs to the app that loads it: a pack that names no dialect is stamped
/// with the app's own, exactly as every admission path (`try_adopt_completed`, the constructor) stamps it — without it the
/// first child restore faulted "child restore requires the parent's exact declared dialect".
fn loaded_with_app_dialect<A: ArtifactApp>(mut envelope: ArtifactEnvelope<A::Snapshot, A::Mutation>) -> ArtifactEnvelope<A::Snapshot, A::Mutation> {
    if envelope.dialect.is_none() {
        envelope.dialect = Some(A::DIALECT.into());
    }
    envelope
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> PluginApp for VcsArtifactApp<A, M> {
    fn admit_cancellation_issuer(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let parent=cx.original_source_parent()?;self.tools.tool_cancellations.admit_step(parent,cx)}

    fn mounted_owner_maintenance_phase(&self) -> Result<MountedOwnerPhaseV1, Fault> {
        let instance_id = self.tools.live_runtime_instance_id.ok_or_else(|| plugin_sdk_fault("mounted phase requires the live instance"))?;
        if A::mounted_jobs_terminal_is_empty(instance_id) { return Ok(MountedOwnerPhaseV1::Maintenance); }
        A::mounted_job_maintenance_phase(instance_id).ok_or_else(|| plugin_sdk_fault("retained mounted owner did not declare its preparation or maintenance phase"))
    }

    fn mounted_owner_maintenance_step(&mut self,turn:&mut MountedOwnerTurnV1<'_,'_>)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let identity=turn.identity();if self.tools.live_runtime_instance_id!=Some(identity.instance_id)||!matches!(turn.phase(),MountedOwnerPhaseV1::Preparation|MountedOwnerPhaseV1::Maintenance){return Err(plugin_sdk_fault("mounted maintenance does not own this live instance and phase").into())}
        let step=turn.advance_lifecycle(|cx,loan|if !A::mounted_jobs_terminal_is_empty(identity.instance_id){let demand=A::mounted_job_maintenance_demands(identity.instance_id,loan.maximum_copy_bytes)?;semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,demand,|same,actual|Ok((A::mounted_job_maintenance_step(identity.instance_id,same,actual)?,true)))}else{self.maintenance_step(cx,loan)})?;
        Ok(match step{PluginLifecycleStep::Complete(progress)if !self.maintenance_terminal_is_empty()=>PluginLifecycleStep::Progress(progress),step=>step})
    }

    fn mounted_owner_close_step(&mut self,turn:&mut MountedOwnerTurnV1<'_,'_>)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        let identity=turn.identity();if self.tools.live_runtime_instance_id!=Some(identity.instance_id)||turn.phase()!=MountedOwnerPhaseV1::Close{return Err(plugin_sdk_fault("mounted close does not own this live instance and phase").into())}
        let step=turn.advance_lifecycle(|cx,loan|if !A::mounted_jobs_terminal_is_empty(identity.instance_id){let demand=A::mounted_job_close_demands(identity.instance_id,loan.maximum_copy_bytes)?;semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,demand,|same,actual|Ok((A::mounted_job_close_step(identity.instance_id,same,actual)?,A::mounted_jobs_terminal_is_empty(identity.instance_id))))}else{self.close_step(cx,loan)})?;
        Ok(match step{PluginLifecycleStep::Complete(progress)if !self.close_terminal_is_empty()=>PluginLifecycleStep::Progress(progress),step=>step})
    }

    fn close_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.constructor.has_constructor_receipt() { return Ok(Default::default()); }
        if !self.constructor.constructor_terminal_is_empty() { return self.stores.constructor_close_demands(&self.constructor, body); }
        self.close_ladder_demand(body)
    }

    fn maintenance_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.ladder_ref().maintenance_ladder_demands(body)
    }

    async fn bind_instance_id(&mut self, instance_id: u32) {
        self.tools.live_runtime_instance_id = Some(instance_id);
    }

    async fn bind_actor(&mut self, actor: &str) {
        assert_eq!(self.stores.store.local_actor_id().0.as_str(), actor, "opening actor must own the document store from construction");
        assert_eq!(self.stores.config_store.local_actor_id().0.as_str(), actor, "opening actor must own the config store from construction");
        assert_eq!(self.stores.draft_store.local_actor_id().0.as_str(), actor, "opening actor must own the draft store from construction");
        assert_eq!(self.stores.interaction_store.local_actor_id().0.as_str(), actor, "opening actor must own the interaction store from construction");
        assert_eq!(self.stores.window_config_store.local_actor_id().0.as_str(), actor, "opening actor must own the window config registry from construction");
    }

    /// 🎟️ Advances one genuine original owner under the complete caller grant.
    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        if self.constructor.has_constructor_receipt(){return Ok(PluginLifecycleStep::AwaitingInput{reason:"original constructor receipt awaits its phase recipient"})}
        let demand=self.close_retirement_demands(loan.maximum_copy_bytes)?;
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,loan,demand,|same,grant|{
            if !self.constructor.constructor_terminal_is_empty(){self.stores.close_store_constructor_unit(&mut self.constructor, grant)?;let progress=self.constructor.constructor_receipt().map_or(Default::default(),|(_,_,_,progress)|*progress);self.constructor.take_constructor_receipt();return Ok((PluginLifecycleStep::Progress(progress),false))}
            let step=self.close_ladder_step(same,grant)?;let terminal=self.close_terminal_is_empty();Ok((PluginLifecycleStep::retained(step,terminal),terminal))
        })
    }

    fn close_terminal_is_empty(&self) -> bool {
        self.constructor.constructor_terminal_is_empty()
            && self.close.close_started
            && self.history.pending_command_prune.is_none()
            && !self.history.command_prune_requested()
            && self.history.command_log.pruned_terminal_is_empty()
            && InstanceExtensions::terminal_is_empty(&self.time_travel, &self.tool_runs)
            && self.stores.store.close_owned_store_terminal_is_empty()
            && self.stores.config_store.close_owned_store_terminal_is_empty()
            && self.stores.draft_store.close_owned_store_terminal_is_empty()
            && self.stores.interaction_store.close_owned_store_terminal_is_empty()
            && self.close.close_presence_disposer.is_none()
            && self.close.close_transient_disposer.is_none()
            && self.stores.window_config_store.terminal_is_empty()
            && self.stores.window_transient_store.terminal_is_empty()
            && self.stores.retired_window_transient_stores.is_empty()
            && self.stores.retired_window_transient_stores.empty_backing_byte_demand()==Some(0)
            && self.close.close_interaction_disposer.is_none()
            && self.tools.tool_cancellations.terminal_is_empty()
            && self.pending.pending_document_window_admission.is_none()
            && self.tools.pending_tool_lease_admission.is_none()
            && self.media.pending_media_import_permit.is_none()
            && self.tools.tool_operations.is_empty()
            && self.child_composition.private_child_groups_terminal_is_empty()
            && self.tools.latest_wins_commands.is_empty()
            && self.tools.typed_operation_reservations.iter().all(Option::is_none)
            && self.tools.latest_wins_order.len() == 0
            && self.tools.latest_wins_keys.terminal_is_empty()
            && self.close.close_instance_operation_owner_drained
            && self.tools.instance_operation_owner.close_terminal_is_empty().unwrap_or(false)
            && self.pending.render_failure_terminal_is_empty()
            && self.tools.agent_lane_preview.is_none()
            && self.media.media_submission_refusal_terminal_is_empty()
            && self.ingress.action_ingress_sources_closed()
            && self.media.media_exports.is_empty()
            && self.pending.pending_reserved.is_empty()
            && self.pending.reserved_commits.is_empty()
            && self.pending.reserved_commit_outcome.is_none()
            && self.media.media_closures.is_empty()
            && self.media.snapshot_retirements.is_empty()
            && self.media.segmented_downloads.is_empty()
            && self.media.segmented_closures.is_empty()
            && self.child_composition.child_content_retirements.is_empty()
            && self.child_composition.child_content_retirements.empty_backing_byte_demand()==Some(0)
            && self.close.close_child_member.is_none()
            && self.child_composition.child_member_retirements.is_empty()
            && self.child_composition.child_member_retirements.empty_backing_byte_demand()==Some(0)
            && self.child_composition.children.is_empty()
            && self.roster.peer_roster_publications.is_empty()
            && self.roster.peer_roster_outcomes.is_empty()
            && self.roster.peer_presence_retirements.is_empty()
            && self.roster.presence_peer_retirements.is_empty()
            && self.ingest.envelope_ingress.is_empty()
            && self.ingest.envelope_decode_jobs.is_empty()
            && self.ingest.store_replacement_jobs.is_empty()
            && self.ingest.document_archive_loads.is_empty()
            && self.ingest.document_archive_loads.empty_backing_byte_demand()==Some(0)
            && self.ingest.envelope_field_decoder_retirements.is_empty()
            && self.ingest.envelope_completed_record_retirements.is_empty()
            && self.ladder_ref().retained_fields_terminal_is_empty()
    }

    /// 🎡️ Visits the live-maintenance units in fair order and forwards the unchanged grant to the first that releases something.
    fn maintenance_step(&mut self, cx:&mut semio_framework_job::StepContext<'_>, loan:RetainedCloneGrant) -> Result<PluginLifecycleStep,semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        self.maintenance_ladder_step(cx,loan)
    }

    fn maintenance_terminal_is_empty(&self) -> bool {
        self.ladder_ref().maintenance_ladder_terminal_is_empty()
    }

    fn maintenance_under_pressure(&self) -> bool {
        self.ladder_ref().maintenance_ladder_under_pressure()
    }

    async fn app_id(&self) -> &str {
        self.app.instance_id().await
    }

    async fn artifact_schema(&self) -> &str {
        A::DOCUMENT_SCHEMA
    }

    async fn tool_owner_witness(&self) -> ToolOwnerWitness {
        ToolOwnerWitness::of::<A>()
    }

    async fn tool_public_contracts(&self) -> Vec<ArtifactToolPublicContract> {
        self.tools.bounded_tool_contracts
            .iter()
            .cloned()
            .chain(self.tools.framework_tool_registrations.values().map(|registration| ArtifactToolPublicContract {
                owner: registration.owner,
                controller_id: registration.key.controller_id.clone(),
                tool_id: registration.key.tool_id.clone(),
                schema_id: registration.schema_id.clone(),
                max_raw_wire_bytes: registration.contract.max_raw_wire_bytes,
                publication_lanes: registration.publication_lanes,
            }))
            .chain(self.tools.app_tool_registrations.values().map(|registration| ArtifactToolPublicContract {
                owner: registration.owner,
                controller_id: registration.key.controller_id.clone(),
                tool_id: registration.key.tool_id.clone(),
                schema_id: registration.schema_id.clone(),
                max_raw_wire_bytes: registration.contract.max_raw_wire_bytes,
                publication_lanes: registration.publication_lanes,
            }))
            .collect()
    }

    async fn advance_typed_operation_publication(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let mut retained=cx.retained_grant();if cx.should_yield(){return Ok(())}
        if self.interaction.local_interaction_query_receipt.is_some(){return Ok(())}
        if self.constructor.has_constructor_receipt() { return Ok(()); }
        if self.constructor.store_constructor_is_pending() { ({ let __a0 = semio_framework_plugin_reactor_app::original_plugin_turn_grant(self.mounted_policy.preparation,retained); self.stores.drive_store_constructor_unit(&mut self.constructor, __a0) })?; return Ok(()); }
        if InstanceExtensions::holds_receipt(&self.time_travel, &self.tool_runs) || (0..ARTIFACT_LIVE_OUTPUT_SLOTS).any(|index|self.tools.tool_operations.entry(index).is_some_and(|(_,owner)|owner.publication_ownership_progress.is_some()||owner.original_retirement_receipt.is_some())){return Ok(());}
        if !self.pending.reserved_commits.is_empty() && self.pending.reserved_commit_outcome.is_none() {
            self.command_ctx().step_framework_reserved_commit().await?;
            return Ok(());
        }
        if InstanceExtensions::drive_turn(&mut self.extension_ctx(), &mut retained, identity, cx).await? {
            return Ok(());
        }
        if self.interaction.local_interaction_query.as_ref().is_some_and(|query| query.has_pending_work()) {
            self.interaction.local_interaction_query_turn = !self.interaction.local_interaction_query_turn;
            if self.interaction.local_interaction_query_turn {
                self.interaction.advance_local_interaction_query_one(store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(retained), semio_framework_plugin_interaction::query::LOCAL_INTERACTION_QUERY_PAGE_BYTES)?;
                return Ok(());
            }
        }
        self.typed_ctx().advance_typed_operation_publication_one(cx).await.map_err(Into::into)
    }

    fn take_typed_operation_preparation_receipt(&mut self) -> Option<(semio_framework_job::OperationId, RetainedCloneGrant, RetainedCloneProgress)> {
        for index in 0..ARTIFACT_LIVE_OUTPUT_SLOTS{
            let Some((_,owner))=self.tools.latest_wins_commands.entry_mut(index)else{continue;};
            if let Some((grant,progress))=owner.receipt.take(){return Some((owner.operation.operation,grant,progress));}
        }

        for index in 0..ARTIFACT_LIVE_OUTPUT_SLOTS{
            let Some((_,owner))=self.tools.tool_operations.entry_mut(index)else{continue;};
            if let Some((grant,progress))=owner.publication_ownership_progress.take(){return Some((owner.operation.operation,grant,progress));}
        }
        None
    }

    fn take_typed_operation_retirement_receipt(&mut self) -> Option<(semio_framework_job::OperationId, RetainedCloneGrant, RetainedCloneProgress)> {
        for index in 0..ARTIFACT_LIVE_OUTPUT_SLOTS{let Some((_,owner))=self.tools.tool_operations.entry_mut(index)else{continue;};if let Some((grant,progress))=owner.original_retirement_receipt.take(){return Some((owner.operation.operation,grant,progress));}}None
    }

    fn take_history_command_receipt(&mut self) -> Option<(RetainedCloneGrant, RetainedCloneProgress)> {
        self.time_travel.take_command_receipt()
    }

    fn take_tool_run_receipt(&mut self) -> Option<(RetainedCloneGrant, RetainedCloneProgress)> {
        self.tool_runs.take_original_receipt()
    }

    fn take_local_interaction_query_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.interaction.local_interaction_query_receipt.take()}

    fn store_constructor_is_pending(&self) -> bool { self.constructor.store_constructor_is_pending() }

    fn take_constructor_receipt(&mut self) -> Option<(ArtifactStoreConstructorKind, ArtifactStoreConstructorPhase, RetainedCloneGrant, RetainedCloneProgress)> { self.constructor.take_constructor_receipt() }

    fn tool_run_presence(&self) -> Option<protocol::PresenceToolRun> {
        self.tool_runs.presence()
    }

    fn tool_run_trace_delta(&self, cursor: Option<ToolRunTraceCursor>) -> Option<String> {
        self.tool_runs.trace_delta(cursor, TOOL_RUN_TRACE_DELTA_BYTES)
    }

    fn typed_operation_refusal(&self) -> Option<String> {
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|index| {
            let (_, operation) = self.tools.tool_operations.entry(index)?;
            operation.retained_structural_refusal().map(|reason| format!("typed operation '{}' {reason}", operation.verb))
        })
    }

    fn close_diagnostic(&self) -> String {
        self.ladder_ref().close_rung_name()
    }

    fn typed_operation_diagnostic(&self) -> String {
        (0..ARTIFACT_LIVE_OUTPUT_SLOTS).filter_map(|index| {
            let (id, operation) = self.tools.tool_operations.entry(index)?;
            Some(format!("operation {id} '{}' stage={:?} session={} terminal={} refusal={}", operation.verb, operation.stage, operation.session.is_some(), operation.terminal_seen, operation.retained_structural_refusal().unwrap_or_else(|| "none".into())))
        }).collect::<Vec<_>>().join("; ")
    }

    fn has_pending_typed_operations(&self) -> bool {
        self.constructor.store_constructor_is_pending() || self.tools.operation_progress_retired
            || self.child_composition.derivable_follow_awaits_retirement(&self.stores)
            || InstanceExtensions::has_pending_work(&self.extension_ref())
            || !self.pending.reserved_commits.is_empty()
            || self.pending.reserved_commit_outcome.is_some()
            || !self.tools.tool_operations.is_empty()
            || !self.tools.latest_wins_commands.is_empty()
            || self.tools.typed_effect_outbox.len() != 0
            || self.tools.typed_event_outbox.len() != 0
            || self.tools.typed_ui_outbox.len() != 0
            || self.tools.typed_completion_outbox.len() != 0
            || self.interaction.local_interaction_query.as_ref().is_some_and(|query| query.has_pending_work())
    }

    fn live_typed_operation_slots(&self) -> usize {
        VcsArtifactApp::<A, M>::live_typed_operation_slots(self)
    }

    fn has_runnable_typed_operations(&self) -> bool {
        self.constructor.store_constructor_is_pending() || self.tools.operation_progress_retired
            || self.child_composition.derivable_follow_awaits_retirement(&self.stores)
            || InstanceExtensions::has_pending_work(&self.extension_ref())
            || !self.pending.reserved_commits.is_empty()
            || self.pending.reserved_commit_outcome.is_some()
            || !self.tools.tool_operations.is_empty()
            || self.ingest.has_runnable_artifact_envelope_decode_worker_step()
            || !self.tools.latest_wins_commands.is_empty()
            || self.tools.typed_effect_outbox.len() != 0
            || self.tools.typed_event_outbox.len() != 0
            || self.tools.typed_ui_outbox.len() != 0
            || self.tools.typed_completion_outbox.len() != 0
            || self.interaction.local_interaction_query.as_ref().is_some_and(|query| query.has_pending_work())
    }

    fn begin_local_interaction_query(&mut self, request_id: u64, query_generation: u64) -> Option<protocol::LocalInteractionQueryReply> {
        if self.close.close_started || self.tools.live_runtime_instance_id.is_none() {
            return Some(protocol::LocalInteractionQueryReply::Rejected { request_id, code: protocol::LocalInteractionQueryRejection::Closed });
        }
        if self.interaction.local_interaction_query.is_some() {
            return Some(protocol::LocalInteractionQueryReply::Rejected { request_id, code: protocol::LocalInteractionQueryRejection::Busy });
        }
        let document_revision = self.stores.store.content_revision_now();
        let config_revision = self.stores.config_store.content_revision_now();
        let topology_revision = match self.interaction.local_interaction_authority.revision(document_revision, config_revision) {
            Ok(revision) => revision,
            Err(_) => return Some(protocol::LocalInteractionQueryReply::Rejected { request_id, code: protocol::LocalInteractionQueryRejection::Closed }),
        };
        let identity = protocol::LocalInteractionIdentity {
            app_instance_id: self.tools.live_runtime_instance_id.expect("live instance checked"),
            generation: self.stores.interaction_store.generation_now(),
            revision: self.stores.interaction_store.content_revision_now(),
            document_revision,
            topology_revision,
        };
        self.interaction.local_interaction_query =
            Some(semio_framework_plugin_interaction::live::LocalInteractionLiveQuery::new(request_id, query_generation, identity, self.stores.store.snapshot_read().ok(), self.stores.config_store.snapshot_read().ok(), self.stores.interaction_store.snapshot_read().ok()));
        self.interaction.local_interaction_emitted_bytes = 0;
        self.interaction.local_interaction_retired_bytes = 0;
        None
    }

    fn acknowledge_local_interaction_query(&mut self, token: &protocol::LocalInteractionQueryToken) -> bool {
        self.interaction.local_interaction_query.as_mut().is_some_and(|query| query.acknowledge(token))
    }

    fn cancel_local_interaction_query(&mut self, token: &protocol::LocalInteractionQueryToken) -> bool {
        self.interaction.local_interaction_query.as_mut().is_some_and(|query| query.cancel_authorized(token))
    }

    fn take_local_interaction_query_reply(&mut self) -> Option<protocol::LocalInteractionQueryReply> {
        let query = self.interaction.local_interaction_query.as_mut()?;
        let reply = query.take_reply();
        if query.terminal_is_empty() {
            self.interaction.local_interaction_query = None;
        }
        reply
    }

    fn publish_local_interaction_query_reply(&mut self, frames: &mut Vec<Vec<u8>>, maximum_frames: usize) -> bool {
        let Some(query) = self.interaction.local_interaction_query.as_mut() else {
            return false;
        };
        if !query.reply_ready() || frames.len() >= maximum_frames {
            return false;
        }
        let mut encoded = Vec::new();
        if frames.try_reserve_exact(1).is_err() || encoded.try_reserve_exact(512).is_err() {
            return false;
        }
        let mut encoded = Some(encoded);
        let delivered = query
            .take_reply_admitted(|reply| {
                if protocol::encode_local_interaction_query_frame_into(reply, encoded.as_mut().expect("one admitted reply buffer")).is_err() {
                    return false;
                }
                frames.push(encoded.take().expect("one admitted reply buffer"));
                true
            })
            .is_some();
        if query.terminal_is_empty() {
            self.interaction.local_interaction_query = None;
        }
        delivered
    }

    fn take_typed_operation_result_page(&mut self, receiver: u32) -> Option<TypedOperationResultPage> {
        let (index, operation_id) = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find_map(|offset| {
            let index = (self.tools.typed_result_cursor + offset) % ARTIFACT_LIVE_OUTPUT_SLOTS;
            self.tools.tool_operations.entry(index).filter(|(_, operation)| operation.meta.instance_id == receiver && !operation.result_page_presented && operation.result_page.is_some()).map(|(id, _)| (index, *id))
        })?;
        self.tools.typed_result_cursor = (index + 1) % ARTIFACT_LIVE_OUTPUT_SLOTS;
        self.tools.tool_operations.get_mut(operation_id)?.take_result_page()
    }

    fn acknowledge_typed_operation_result(&mut self, token: TypedOperationResultToken) -> Result<bool, Fault> {
        if token.receiver != self.tools.live_runtime_instance_id.unwrap_or(u32::MAX) {
            return Ok(false);
        }
        let download_ack = self.tools.tool_operations.get(token.operation).and_then(|operation| operation.result_page.as_ref().map(|page| (operation, page))).is_some_and(|(operation, page)| {
            page.token == token
                && page.lane == TypedOperationResultLane::Download
                && operation.stage == MountedTypedCommandFullOperationStage::AwaitingAck
                && matches!(operation.publication.as_ref(), Some(ArtifactToolCompletionValue::Download(Ok(_), _)))
        });
        if download_ack && (!self.media.segmented_downloads.can_insert(token.operation) || !self.media.segmented_closures.can_insert(token.operation)) {
            return Ok(false);
        }
        let Some(operation) = self.tools.tool_operations.get_mut(token.operation) else { return Ok(false) };
        let accepted = operation.acknowledge_result_page(token)?;
        if !accepted || !download_ack {
            return Ok(accepted);
        }
        let publication = operation.publication.take().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-download-owner"), "download ACK lost its exact retained publication owner"))?;
        let ArtifactToolCompletionValue::Download(Ok(download), ephemeral) = publication else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-download-owner"), "download ACK resolved a non-download retained publication owner"));
        };
        if !ephemeral.presence.is_empty() || !ephemeral.transient.is_empty() {
            operation.publication = Some(ArtifactToolCompletionValue::Download(Ok(download), ephemeral));
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-download-ephemeral"), "download ACK preceded retained ephemeral publication"));
        }
        self.media.segmented_downloads.insert_admitted(token.operation, download);
        Ok(true)
    }

    fn take_typed_operation_effect(&mut self) -> Option<Effect> {
        self.tools.typed_effect_outbox.pop()
    }

    fn take_typed_operation_event(&mut self) -> Option<AppEvent> {
        self.tools.typed_event_outbox.pop()
    }

    fn take_typed_operation_ui_scope(&mut self) -> Option<UiDirtyScope> {
        self.tools.typed_ui_outbox.pop().or_else(|| self.tools.take_operation_progress_scope())
    }

    fn take_typed_operation_ui_progress(&mut self) -> Option<TypedOperationUiProgress> {
        let ui_scope = match self.tools.typed_ui_outbox.pop().or_else(|| self.tools.take_operation_progress_scope()) {
            Some(scope) => scope,
            None if self.time_travel.has_patch() => UiDirtyScope::None,
            None => return None,
        };
        Some(TypedOperationUiProgress { ui_scope, leftover: self.tools.typed_inline_interaction_leftover.take(), history_patch: self.time_travel.take_patch() })
    }

    /// 🧾️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B21: the dirty set is only authoritative AFTER the
    /// backfill. A typed operation publishes its edit through
    /// [`Self::publish_mounted_typed_operation_unit`], which stages a batched `Edit` and never calls
    /// `record_command` — that edit reaches the command log exclusively through
    /// [`Self::backfill_command_log`], which runs inside [`Self::refresh_cache`], which
    /// [`Self::history_patch`] runs FIRST. Reading `history_dirty_sequences` before that backfill
    /// therefore answered "nothing dirty" on the very call whose publication had just landed, so the
    /// terminal completion carried `history_patch: None` and the row surfaced on whatever later call
    /// happened to refresh the cache — the one-command lag measured in the browser on wasm #47
    /// (`📓️2026-09-12-wave-B19-mutation-lane-regression.md` §3).
    ///
    /// 🧾️ Lane-less rows (a View, a Mutation that emitted nothing) join this patch. Retirement
    /// still records them, but only after this function has already read `history_dirty_sequences`.
    async fn take_typed_operation_completion(&mut self) -> Result<Option<TypedOperationCompletion>, Fault> {
        let Some(witness) = self.tools.typed_completion_outbox.pop() else { return Ok(None) };
        self.tools.record_settled_typed_operation_command(&mut self.history, &mut self.host_events, witness.operation);
        self.render_ctx().refresh_cache().await?;
        let history_patch = if self.history.history_dirty_sequences.is_empty() { None } else { Some(self.render_ctx().history_patch(false).await?) };
        let revision = self.stores.store.content_revision_now();
        Ok(Some(TypedOperationCompletion { operation: witness.operation, revision: u64::from_be_bytes(revision[..8].try_into().expect("revision lane width")), ui_scope: witness.ui_scope, history_patch }))
    }

    fn take_typed_operation_composed_result(&mut self) -> Option<ComposedGestureResult> {
        self.tools.typed_composed_outbox.pop()
    }

    async fn begin_dispatch_report(&mut self) {
        self.pending.reset_dispatch_report(&mut self.stores).await;
    }

    async fn dispatch_report(&self) -> &protocol::DispatchReport {
        &self.pending.dispatch_report
    }

    async fn set_merge_policy(&mut self, policy: protocol::MergePolicy) {
        self.pending.apply_merge_policy(&mut self.stores, policy).await;
    }

    async fn resolve_conflict(&mut self, conflict_id: &str, resolution: protocol::ConflictResolution) -> Result<protocol::MergeReport, Fault> {
        self.pending.resolve_store_conflict(&mut self.stores, &mut self.history, conflict_id, resolution).await
    }

    async fn open_conflicts(&self) -> Vec<protocol::Conflict> {
        self.stores.current_open_conflicts().await
    }

    async fn handle_action(&mut self, action: &str, args: &mut Option<DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        let log_generation_before = self.history.log_generation;
        let result = self.command_ctx().dispatch_action(action, args, &mut None, meta, identity, cx).await?;
        let Some(result)=result else{return Ok(None)};
        self.child_composition.follow_derivable_children(&mut self.stores, identity).await?;
        Ok(Some(self.render_ctx().finish_recorded(log_generation_before, action, result).await))
    }

    fn admit_reserved_spawned_job(&mut self, job: u64, output: Result<Vec<u8>, Fault>) -> Result<bool, Fault> {
        self.pending.admit_framework_reserved_commit(&mut self.history, job, output)
    }

    fn take_reserved_commit_outcome(&mut self) -> Option<Result<InvocationResult, Fault>> {
        self.pending.take_framework_reserved_commit_outcome()
    }

    fn reserved_commit_progress(&self) -> Option<FrameworkReservedCommitProgress> {
        self.pending.framework_reserved_commit_progress()
    }

    async fn handle_action_invocation(&mut self, address: &semio_framework::manifest::ActionAddress, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, active_mode_id: Option<&str>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let owner_app_id = self.app.instance_id().await;
        if address.app_id != owner_app_id {
            return Err((plugin_sdk_fault(format!("action app owner {} does not match {}", address.app_id, owner_app_id))).into());
        }
        if address.window_instance_id.trim().is_empty() {
            return Err((plugin_sdk_fault("action window instance id must be non-empty")).into());
        }
        let reserved_kind = framework_reserved_action_kind(&address.action_id);
        if reserved_kind.is_none() {
            if !self.tools.registry.has_mode(&address.mode_id).await {
                return Err((plugin_sdk_fault(format!("unknown action mode owner {}", address.mode_id))).into());
            }
            if active_mode_id != Some(address.mode_id.as_str()) {
                return Err((plugin_sdk_fault(format!("action {} belongs to inactive mode {}", address.action_id, address.mode_id))).into());
            }
        }
        let kind = match self.tools.registry.window_action(&address.window_kind_id, &address.action_id).await {
            Some(definition) => definition.kind,
            None => reserved_kind.ok_or_else(|| plugin_sdk_fault(format!("window kind {} does not own action {}", address.window_kind_id, address.action_id)))?,
        };
        if !self.ingress.action_source_capture_ready(&address.action_id, cx){return Ok(None)}
        let log_generation_before = self.history.log_generation;
        let previous_kind = self.host_events.invocation_kind.replace(kind);
        let result = self.command_ctx().dispatch_action(&address.action_id, source_args, original_fault, meta, identity, cx).await;
        self.host_events.invocation_kind = previous_kind;
        let Some(result) = result? else{return Ok(None)};
        Ok(Some(self.render_ctx().finish_recorded(log_generation_before, &address.action_id, result).await))
    }

    async fn handle_response_action(&mut self, action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        if !self.ingress.action_source_capture_ready(action, cx){return Ok(None)}
        let definition = self.tools.registry.app_command(action).await.ok_or_else(|| plugin_sdk_fault(format!("continuation command '{action}' is not owned by the receiving app")))?;
        validate_ui_dispatch_classification("receiving app", action, definition.semantics.execution.interactive_job)?;
        let kind = definition.kind;
        let Some(command) = self.ingress.receive_action_command(action, args, original_fault, cx).await? else { return Ok(None) };
        let wire = <A::Command as ::protocol::OpBinary>::encode_op(&command).map_err(|error| error.into_fault())?;
        let admission = self.tools.admit_command_wire(&self.stores, action, &wire, 1).await?;
        self.media.require_complete_tool_operation_pipeline(&admission)?;
        let log_generation_before = self.history.log_generation;
        let previous_kind = self.host_events.invocation_kind.replace(kind);
        let result = self.dispatch_ctx().dispatch_typed_command_inner(Box::new(command), admission, meta, identity, cx).await;
        self.host_events.invocation_kind = previous_kind;
        let result = result?;
        Ok(Some(self.render_ctx().finish_recorded(log_generation_before, action, result).await))
    }

    async fn handle_command(&mut self, address: &ManifestCommandAddress, source_args: &mut Option<DslValue>, original_fault: &mut Option<Fault>, active_mode_id: Option<&str>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let log_generation_before = self.history.log_generation;
        let result = self.command_ctx().dispatch_command(address, source_args, original_fault, active_mode_id, meta, identity, cx).await?;
        let Some(result)=result else{return Ok(None)};
        Ok(Some(self.render_ctx().finish_recorded(log_generation_before, &address.command_id, result).await))
    }

    fn original_command_frame_pending(&self)->bool{self.tools.agent_lane_preview.is_some()}

    async fn handle_command_frame(&mut self, command_bytes: &[u8], meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let log_generation_before = self.history.log_generation;
        let result = self.command_ctx().dispatch_command_frame(command_bytes, meta, identity, cx).await?;
        match result{Some(result)=>Ok(Some(self.render_ctx().finish_recorded(log_generation_before,"typed-command",result).await)),None=>Ok(None)}
    }

    /// 🎯️ M1: resolves `A::command_from_intent(intent)` then dispatches through the SAME
    /// `dispatch_typed_command_inner` `dispatch_typed_command`/`dispatch_typed` use — kind
    /// discipline, the command log, undo grouping and `AsyncTask` spawning apply unchanged.
    async fn handle_intent_frame(&mut self, intent: &UiIntent, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure> {
        let verb = intent.action.name.as_str();
        if intent.action.version != 1 {
            return Err((Fault::new(FaultOrigin::App, FaultCode::new("app.intent.version-mismatch"), format!("intent action '{verb}' targets version {} but the intent bridge only resolves version-1 actions", intent.action.version))).into());
        }
        let _ = self.tools.qualified_tool_proof(&self.stores, verb)?;
        if !self.ingress.action_source_capture_ready(verb, cx){return Ok(None)}
        let mut merged = if self.ingress.action_source_pending(){None}else{merge_ui_values(intent.args.as_ref(), intent.input.as_ref()).await?};
        let Some(command)=self.ingress.receive_action_command(verb, &mut merged, &mut None, cx).await?else{return Ok(None)};
        self.dispatch_ctx().dispatch_typed(command, meta, identity, cx).await.map(Some).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: decodes the task resolution's own
    /// `OpBinary`-encoded `A::Command` and re-enters `dispatch_typed` with it — see the trait
    /// method's own doc for why this is the applying lane and not `handle_command_frame`.
    async fn resume_task_command(&mut self, command_bytes: Vec<u8>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let command = <A::Command as ::protocol::OpBinary>::decode_op(&command_bytes)
            .map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.task-resume-command"), format!("an AsyncTask resolved TaskResolution::Command with bytes that are not this app's typed Command: {error}")))?;
        self.dispatch_ctx().dispatch_typed(command, meta, identity, cx).await
    }

    /// 🧵️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME: decodes each lane's wire ops back into this
    /// app's concrete `A::Mutation`/`A::ConfigMutation`/`A::DraftMutation` via `OpBinary::
    /// decode_op` (the SAME decode idiom `dispatch_typed_command` uses for `A::Command`) and
    /// dispatches them directly — see the trait method's own doc for why this skips `handle`.
    async fn resume_task_emit(&mut self, artifact_ops: Vec<u8>, config_ops: Vec<u8>, draft_ops: Vec<u8>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        async fn decode_lane<T: ::protocol::OpBinary>(bytes: &[u8]) -> Result<Vec<T>, Fault> {
            let op_bytes_list = protocol::decode_ops_vec(bytes).map_err(|error| error.into_fault())?;
            let mut decoded = Vec::with_capacity(op_bytes_list.len());
            for op_bytes in op_bytes_list.iter() {
                decoded.push(<T as ::protocol::OpBinary>::decode_op(op_bytes).map_err(|error| error.into_fault())?);
            }
            Ok(decoded)
        }
        let log_generation_before = self.history.log_generation;
        let artifact_mutations = decode_lane::<A::Mutation>(&artifact_ops).await?;
        let config_mutations = decode_lane::<A::ConfigMutation>(&config_ops).await?;
        let draft_mutations = decode_lane::<A::DraftMutation>(&draft_ops).await?;
        let result = self.dispatch_ctx().dispatch_emit("task-resume-emit", Emit { artifact_mutations, config_mutations, draft_mutations, ..Default::default() }, meta, identity, cx).await?;
        Ok(self.render_ctx().finish_recorded(log_generation_before, "task-resume-emit", result).await)
    }

    async fn take_last_emit_wire(&mut self) -> Option<EmitWire> {
        self.pending.last_emit_wire.take()
    }

    async fn take_pending_presence(&mut self) -> Vec<PresenceUpdate> {
        std::mem::take(&mut self.roster.pending_presence)
    }

    async fn take_pending_transaction_proposal(&mut self) -> Option<TransactionProposalDraft> {
        self.txn.pending_transaction_proposal.take()
    }

    /// 🔀️ Decodes EITHER wire form (contract §2/§5.3): pre-planned (`prepared_ops`
    /// non-empty) carries its own `origin`; owner-mutation (`prepared_ops` empty)
    /// decodes the single `payload` op and has no origin on the wire — see the 🚧️ note below.
    ///
    /// 🚧️ The owner-mutation form carries no `origin` on the wire (contract §2/§5.3)
    /// — this member is by construction a foreign target of someone else's
    /// transaction, so the best available provenance is `Transaction` with an
    /// initiator identity the wire does not convey. Flagged in `📓️w1-b-report.md`
    /// as a contract gap for the coordinator/host lanes to close (e.g. by always
    /// preferring the pre-planned form, which does carry `origin`).
    async fn transaction_prepare(&mut self, txn_id: &str, _mutation_id: &str, payload: &[u8], prepared_ops: &[Vec<u8>], prepared_child_ops: &[u8], origin: Option<protocol::MutationOrigin>) -> TransactionPrepareOutcome {
        if let Some(rejection) = InstanceExtensions::transaction_refusal(&self.time_travel, &self.tool_runs, txn_id) {
            return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(rejection) };
        }
        if self.txn.pending_transaction.is_some() {
            return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.instance-busy", "a transaction is already pending on this instance")) };
        }
        let children = match ChildEmit::decode_groups(prepared_child_ops) {
            Ok(children) => children,
            Err(fault) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(fault) },
        };
        if let Some(missing) = children.iter().find(|child| !child.owner.is_empty() || self.child_composition.children.get(&(child.slot.clone(), child.child_id.clone())).is_none()) {
            return TransactionPrepareOutcome {
                foreign: Vec::new(),
                rejection: Some(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.member-rejected", format!("owned child {:?} in slot {:?} is not held by this instance", missing.child_id, missing.slot))),
            };
        }
        let (ops, resolved_origin): (Vec<A::Mutation>, protocol::MutationOrigin) = if !prepared_ops.is_empty() || !children.is_empty() {
            let mut decoded = Vec::with_capacity(prepared_ops.len());
            for op_bytes in prepared_ops {
                match <A::Mutation as ::protocol::OpBinary>::decode_op(op_bytes) {
                    Ok(op) => decoded.push(op),
                    Err(error) => {
                        return TransactionPrepareOutcome {
                            foreign: Vec::new(),
                            rejection: Some(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.unknown-mutation", format!("prepared op did not decode as a {} mutation: {error:?}", A::DOCUMENT_SCHEMA))),
                        };
                    }
                }
            }
            (decoded, origin.unwrap_or_default())
        } else {
            match <A::Mutation as ::protocol::OpBinary>::decode_op(payload) {
                Ok(op) => (vec![op], protocol::MutationOrigin::Transaction { initiator: protocol::ForeignTarget { artifact_id: String::new(), artifact_kind: String::new(), dialect: None } }),
                Err(error) => {
                    return TransactionPrepareOutcome {
                        foreign: Vec::new(),
                        rejection: Some(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.unknown-mutation", format!("owner-mutation payload did not decode as a {} mutation: {error:?}", A::DOCUMENT_SCHEMA))),
                    };
                }
            }
        };
        let mut running = match self.stores.store.snapshot() {
            Ok(snapshot) => snapshot,
            Err(error) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.member-rejected", format!("{error:?}"))) },
        };
        let foreign = Vec::new();
        for op in &ops {
            match op.foreign_step_source(&running, 0) { Ok(None) => {}, Ok(Some(_)) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(plugin_sdk_fault("foreign transaction output awaits its original staged preparation recipient")) }, Err(error) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(error.into_fault()) } }
            let outcome = op.diff(&running);
            if !outcome.is_applicable(protocol::MergePolicy::default()) {
                return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages()))) };
            }
            running = match protocol::apply_diff(outcome.diff(), &running) {
                Ok(next) => next,
                Err(error) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string())) },
            };
        }
        let base_generation = self.stores.store.generation();
        self.txn.pending_transaction = Some(PendingTransaction { txn_id: txn_id.to_string(), ops, children, origin: resolved_origin, base_generation });
        TransactionPrepareOutcome { foreign, rejection: None }
    }

    /// 🔀️ Contract §5.8: reject commit when the base generation observed at prepare time no
    /// longer matches the current one — the pending state is restored (not discarded) so an
    /// explicit `TransactionRollback`/retry can still act on it.
    ///
    /// 🔀️ Contract §5.6: this member's prepared ops land as exactly ONE `Edit` — the
    /// one-edit-per-member invariant `CompositionCoordinator::undo_group`'s tail-based group
    /// undo depends on — stamped with `group_id = txn_id` and every `MutationMeta.origin`.
    async fn transaction_commit(&mut self, txn_id: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, Fault> {
        let pending = match self.txn.pending_transaction.take() {
            Some(pending) if pending.txn_id == txn_id => pending,
            Some(other) => {
                let message = format!("transaction_commit: txn_id {txn_id:?} does not match the pending transaction {:?}", other.txn_id);
                self.txn.pending_transaction = Some(other);
                return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", message));
            }
            None => return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", format!("transaction_commit: no transaction is pending (requested txn_id {txn_id:?})"))),
        };
        if pending.base_generation != self.stores.store.generation() {
            let message = format!("transaction {txn_id:?}'s base generation {} no longer matches the current generation {}", pending.base_generation, self.stores.store.generation());
            self.txn.pending_transaction = Some(pending);
            return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.generation-mismatch", message));
        }
        let PendingTransaction { txn_id, ops, children, origin, .. } = pending;
        if !children.is_empty() {
            return self.dispatch_ctx().commit_transaction_group(&txn_id, ops, children, origin, meta, identity).await;
        }
        let label = ops.first().map(|op| protocol::SemanticMutation::<A::Snapshot>::label(op));
        self.stores.store.dispatch(ArtifactCommand::Apply { mutations: ops, transaction: None }, identity).await.map_err(|error| MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", format!("{error:?}")))?;
        self.history.cache = None;
        self.stores.store.stamp_tail_group_id(&txn_id).await.map_err(|error| MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", format!("{error:?}")))?;
        self.stores.store.stamp_tail_origin(origin).await.map_err(|error| MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", format!("{error:?}")))?;
        let edit_id = self.stores.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()).unwrap_or_default();
        self.tools.record_command(&mut self.history, &format!("transaction:{txn_id}"), ActionKind::Mutation, label, Some(edit_id.clone()), Vec::new(), None);
        Ok(edit_id)
    }

    async fn transaction_rollback(&mut self, txn_id: &str) -> Result<(), Fault> {
        match &self.txn.pending_transaction {
            Some(pending) if pending.txn_id == txn_id => {
                self.txn.pending_transaction = None;
                Ok(())
            }
            _ => Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.rollback-unknown"), format!("transaction_rollback: no pending transaction matches txn_id {txn_id:?}"))),
        }
    }

    async fn transaction_undo(&mut self, group_id: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if !self.child_composition.children.is_empty() {
            return self.dispatch_ctx().transaction_group_history("undo", group_id, identity).await;
        }
        if self.stores.store.tail_group_id().await.as_deref() != Some(group_id) {
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.undo-foreign-tail"), format!("transaction_undo: this instance's tail edit does not belong to group {group_id:?}")));
        }
        self.stores.store.undo(identity).await.map_err(|error| Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.undo-failed"), format!("{error:?}")))?;
        self.history.cache = None;
        Ok(())
    }

    async fn transaction_redo(&mut self, group_id: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        if !self.child_composition.children.is_empty() {
            return self.dispatch_ctx().transaction_group_history("redo", group_id, identity).await;
        }
        match self.stores.store.redo_tail().await {
            Some((_, Some(tail_group))) if tail_group == group_id => {
                self.stores.store.redo(identity).await.map_err(|error| Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.redo-failed"), format!("{error:?}")))?;
                self.history.cache = None;
                Ok(())
            }
            _ => Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("transaction.redo-foreign-tail"), format!("transaction_redo: this instance's redo-tail edit does not belong to group {group_id:?}"))),
        }
    }

    async fn history_snapshot(&mut self) -> Result<HistoryPatch, Fault> {
        if self.time_travel.history_unavailable() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new(PURE_HISTORY_UNAVAILABLE_CODE), "this head-only pure evaluation holds no history; use a live instance"));
        }
        self.render_ctx().history_patch(true).await
    }

    /// 🫥️ A pure command's head is the HEAD snapshot without history (design §20.8, `📓️api-stepped-document-load.md` §4):
    /// it hydrates in O(snapshot) — no history fold — and marks the document head-only, so history verbs and queries answer
    /// `pure.history-unavailable`. An empty head keeps the live document.
    async fn hydrate_pure_head(&mut self,head:&[u8],cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if head.is_empty() {
            return Ok(true);
        }
        if self.pending.pending_document_window_admission.as_ref().is_some_and(|admission|admission.committed){return self.pending.close_document_window_admission(cx, false).map(|step|matches!(step,PluginLifecycleStep::Complete(_))).map_err(Into::into)}
        if !self.tools.admit_document_window_reset(&mut self.pending, cx)?{return Ok(false)}
        let head = <A::Snapshot as ArtifactPack>::decode_pack(head).map_err(|error| plugin_sdk_fault(format!("the pure head snapshot does not decode: {error:?}")))?;
        let id = self.stores.store.envelope().id.clone();
        let envelope = create_document_envelope::<A::Snapshot, A::Mutation>(A::DOCUMENT_SCHEMA, &id, head, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        let mut window_reset=Some(self.ladder_ref().prepare_document_window_reset()?);
        let actor = self.stores.store.local_actor_id().clone();
        self.stores.store.reset(loaded_with_app_dialect::<A>(envelope)).await.map_err(|error| error.into_fault())?;
        self.pending.commit_document_window_reset(&mut self.stores, &mut self.history, &mut window_reset, cx)?;
        assert_eq!(self.stores.store.local_actor_id(), &actor, "pure head retains its opened actor");
        self.history.cache = None;
        self.history.retire_displaced_document_rows();
        self.time_travel.set_history_unavailable(true);
        Ok(false)
    }

    /// 🎯️ Empty domains ⇒ empty bytes (contract-freeze §C7.6) — `encode_presence_interaction`
    /// itself always writes at least `app_id` + a domain count, so an explicit short-circuit
    /// is required rather than trusting the codec's own "nothing to say" shape.
    ///
    /// 👤️ Generation 0 ⇒ empty bytes, the presence twin of the interaction short-circuit above: a
    /// local presence root that was never published has nothing to say, and `encode_pack` always
    /// writes a record header, so an untouched lane would otherwise ship a pack on every frame.
    async fn ephemeral_snapshot(&self) -> EphemeralSnapshot {
        let hover_specs: BTreeMap<String, protocol::HoverSpec> = self.tools.registry.interactions().await.map(|def| (def.id.clone(), def.hover.clone())).collect();
        let selection_specs: BTreeMap<String, protocol::SelectionSpec> = self.tools.registry.interactions().await.map(|def| (def.id.clone(), def.selection.clone())).collect();
        let interaction_state = self.interaction.interaction_state(&self.stores).await;
        let interaction = protocol::assemble_presence_interaction(self.app_id().await, &interaction_state, &hover_specs, &selection_specs).await;
        let interaction_bytes = if interaction.domains.is_empty() {
            Vec::new()
        } else {
            let mut bytes = Vec::new();
            protocol::encode_presence_interaction(&interaction, &mut bytes).await;
            bytes
        };
        let presence_generation = self.stores.presence_store.generation().await;
        EphemeralSnapshot {
            presence: if presence_generation == 0 { Vec::new() } else { self.stores.presence_store.local().encode_pack() },
            presence_generation,
            transient_generation: self.stores.transient_store.generation().await,
            interaction: interaction_bytes,
            tool_run: self.tool_runs.presence(),
            history_edit: self.time_travel.presence(),
        }
    }

    fn reserve_presence_ingress(&mut self, seq: u64) -> Result<PresenceRosterAdmission, Fault> {
        let generation = self.roster.peer_roster_admission_generation.checked_add(1).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-generation"), "peer roster admission generation exhausted"))?;
        let slot = InstanceStores::<A>::peer_roster_slot(generation);
        if self.roster.peer_roster_reservations[slot].is_some()
            || !self.roster.peer_roster_publications.can_insert(generation)
            || !self.roster.peer_roster_outcomes.can_insert(generation)
            || !self.roster.peer_presence_retirements.can_insert(generation)
            || !self.roster.presence_peer_retirements.can_insert(generation)
            || !self.stores.presence_store.peer_retirement_available()
        {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-saturated"), "peer roster ingress, outcome, or retirement authority is saturated before payload decode"));
        }
        self.roster.peer_roster_reservations[slot] = Some((generation, seq));
        self.roster.peer_roster_admission_generation = generation;
        Ok(PresenceRosterAdmission { seq, generation, cancel: self.roster.peer_roster_scope.child_now() })
    }

    fn admit_presence_ingress(&mut self, admission: PresenceRosterAdmission, command: protocol::PresenceCommandCursor, now_ms: i64) {
        let slot = InstanceStores::<A>::peer_roster_slot(admission.generation);
        assert_eq!(command.seq(), admission.seq, "Presence cursor sequence is bound by its unforgeable reservation");
        assert_eq!(self.roster.peer_roster_reservations[slot], Some((admission.generation, admission.seq)), "Presence reservation remains live under the app's exclusive owner");
        assert!(self.roster.peer_roster_publications.can_insert(admission.generation), "Presence publication slot was reserved before cursor construction");
        let generation = admission.generation;
        self.roster.peer_roster_publications.insert_admitted(generation, PeerRosterPublication::<A>::new(admission, now_ms, command));
    }

    fn push_presence_ingress(&mut self, generation: u64, page_index: u32, page: FixedCommandPage) -> Result<(), (Fault, FixedCommandPage)> {
        let Some(publication) = self.roster.peer_roster_publications.get_mut(generation) else {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-owner"), "Presence page targets no retained publication owner"), page));
        };
        publication.push_page(page_index, page)
    }

    fn reject_presence_ingress(&mut self, admission: PresenceRosterAdmission, command: FixedCommandPage, fault: Fault) {
        let slot = InstanceStores::<A>::peer_roster_slot(admission.generation);
        assert_eq!(self.roster.peer_roster_reservations[slot], Some((admission.generation, admission.seq)), "malformed Presence owner retains its exact reservation");
        assert!(self.roster.peer_roster_publications.can_insert(admission.generation), "malformed Presence cleanup uses its pre-reserved publication slot");
        let generation = admission.generation;
        self.roster.peer_roster_publications.insert_admitted(generation, PeerRosterPublication::<A>::rejected(admission, command, fault));
    }

    fn take_presence_outcome(&mut self) -> Option<PresenceRosterOutcome> {
        let generation = self.roster.peer_roster_outcome_generation.checked_add(1)?;
        let outcome = self.roster.peer_roster_outcomes.remove(generation)?;
        self.roster.peer_roster_outcome_generation = generation;
        Some(outcome)
    }

    async fn config_pack(&self) -> Result<store::os_store::serialization::ArtifactPackFiles, Fault> {
        store::os_store::text_format::print_document_pack(self.stores.config_store.envelope()).await.map_err(|error| error.into_fault())
    }

    async fn window_config_packs(&self) -> Result<Vec<WindowConfigPack>, Fault> {
        self.stores.window_config_store.packs().await
    }

    async fn load_window_config_pack(&mut self, pack: WindowConfigPack) -> Result<(), Fault> {
        self.stores.window_config_store.load(pack).await
    }

    async fn load_child_pack(&mut self, slot: &str, child_id: &str, dialect: ArtifactDialect, envelope_pack: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        self.child_composition.open_child(&mut self.stores, slot, child_id, dialect, envelope_pack, identity).await?;
        self.history.cache = None;
        Ok(())
    }

    /// 🔢️ Sorted by `(slot, child_id)`: the map's iteration order is not stable, and a
    /// persisted child list that reshuffles between reads would make every save look like a
    /// change to anything diffing it.
    async fn child_packs(&self) -> Result<Vec<protocol::ChildPackEntry>, Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let mut entries: Vec<protocol::ChildPackEntry> = Vec::with_capacity(self.child_composition.children.len());
        for (key, entry) in self.child_composition.children.keyed_entries() {
            let envelope_pack = entry.member.envelope_pack_bytes().await.map_err(|error| error.into_fault())?;
            let owner = self.child_composition.children.path_of(key).map(|path| path.owner_text()).unwrap_or_default();
            entries.push(protocol::ChildPackEntry { slot: entry.owner.slot.clone(), child_id: entry.owner.child_id.clone(), dialect: entry.reference.dialect.to_coordinate(), envelope_pack, owner });
        }
        entries.sort_by(|left, right| (&left.owner, &left.slot, &left.child_id).cmp(&(&right.owner, &right.slot, &right.child_id)));
        Ok(entries)
    }

    async fn child_head_packs(&self) -> Result<Vec<protocol::ChildHeadPackEntry>, Fault> {

        let mut entries: Vec<protocol::ChildHeadPackEntry> = Vec::with_capacity(self.child_composition.children.len());
        for (key, entry) in self.child_composition.children.keyed_entries() {
            let head_pack = entry.member.document_pack_bytes().await.map_err(|error| error.into_fault())?;
            let owner = self.child_composition.children.path_of(key).map(|path| path.owner_text()).unwrap_or_default();
            entries.push(protocol::ChildHeadPackEntry { slot: entry.owner.slot.clone(), child_id: entry.owner.child_id.clone(), dialect: entry.reference.dialect.to_coordinate(), head_pack, owner });
        }
        entries.sort_by(|left, right| (&left.owner, &left.slot, &left.child_id).cmp(&(&right.owner, &right.slot, &right.child_id)));
        Ok(entries)
    }

    fn begin_document_archive_load(&mut self, operation: u64, mut archive: protocol::DocumentArchivePack) -> Result<(), Fault> {
        if archive.members.len() > protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS {
            return Err(document_load_fault(DOCUMENT_LOAD_TOO_MANY_MEMBERS_CODE, "document archive exceeds its fixed 1024-member authority").with_param("count", archive.members.len().to_string()).with_param("maximum", protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS.to_string()));
        }
        let archive_payload_bytes = archive.members.iter().fold(archive.parent_pack.len().checked_add(archive.parent_spr.len()), |total, entry| total.and_then(|total| total.checked_add(entry.envelope_pack.len())));
        if archive_payload_bytes.is_none_or(|bytes| bytes > protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES) {
            return Err(document_load_fault(DOCUMENT_LOAD_TOO_LARGE_CODE, "document archive typed payload exceeds its fixed byte authority").with_param("bytes", archive_payload_bytes.unwrap_or(usize::MAX).to_string()).with_param("maximum", protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES.to_string()));
        }
        if !self.ingest.document_archive_loads.can_insert(operation) {
            return Err(document_load_fault(DOCUMENT_LOAD_BUSY_CODE, "document archive operation identity is already live or its fixed slot is occupied"));
        }
        if archive.parent_pack.is_empty() || archive.parent_spr.is_empty() {
            return Err(document_load_fault(DOCUMENT_LOAD_INCOMPLETE_CODE, "document archive parent pack and SPR must both be present"));
        }
        archive.members.sort_by_key(|entry| entry.ordinal);
        for (ordinal, entry) in archive.members.iter().enumerate() {
            let fields = [
                entry.reference.artifact_id.as_str(),
                entry.reference.artifact_kind.as_str(),
                entry.reference.standard.as_str(),
                entry.reference.subset.as_str(),
                entry.owner.parent.artifact_id.as_str(),
                entry.owner.parent.artifact_kind.as_str(),
                entry.owner.parent.standard.as_str(),
                entry.owner.parent.subset.as_str(),
                entry.owner.slot.as_str(),
                entry.owner.child_id.as_str(),
            ];
            if usize::try_from(entry.ordinal).ok() != Some(ordinal)
                || entry.envelope_pack.is_empty()
                || fields.iter().any(|field| field.is_empty() || field.len() > store::MEMBER_OPEN_IDENTITY_BYTES || field.chars().any(char::is_control))
            {
                return Err(document_load_fault(DOCUMENT_LOAD_MEMBER_INVALID_CODE, "document archive member ordinal, identity, owner, or envelope is invalid").with_param("ordinal", (ordinal + 1).to_string()));
            }
        }
        archive.members.reverse();
        let limits =
            protocol::RetainedSprLimits { file_bytes: protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES as u64, frame_body_bytes: store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES as u64 * 256, records: protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS as u64 * 8 };
        let history = protocol::RetainedHistoryDecode::new_persisted_document(archive.parent_spr.len(), limits).map_err(|error| document_load_fault(DOCUMENT_LOAD_HISTORY_INVALID_CODE, error.to_string()))?;
        self.ingest.document_archive_loads.insert_admitted(operation, ActiveDocumentArchiveLoad::new(operation, archive, history));
        Ok(())
    }

    fn begin_document_archive_merge(&mut self, operation: u64, archive: protocol::DocumentArchivePack) -> Result<(), Fault> {
        PluginApp::begin_document_archive_load(self, operation, archive)?;
        self.ingest.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive merge admission changed before it was flagged"))?.merge = true;
        Ok(())
    }

    /// 📬️ Observes the original mounted load; its registered maintenance job drives retained work.
    async fn poll_document_archive_load(&mut self, operation: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<protocol::DocumentArchiveLoadStatus, Fault> {
        if self.ingest.document_archive_loads.get(operation).is_some_and(ActiveDocumentArchiveLoad::awaits_merge) {
            self.command_ctx().merge_document_archive(operation, identity, cx).await?;
        }
        self.ingest.document_archive_loads.get(operation).map(ActiveDocumentArchiveLoad::status).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "recursive document archive authority changed before status publication"))
    }

    fn cancel_document_archive_load(&mut self, operation: u64) -> Result<(), Fault> {
        let active = self.ingest.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "unknown recursive document archive operation"))?;
        match active.state {
            ActiveDocumentArchiveLoadState::Pending | ActiveDocumentArchiveLoadState::Running => {
                active.request_terminal(ActiveDocumentArchiveLoadState::Cancelled);
                Ok(())
            }
            ActiveDocumentArchiveLoadState::Cancelled => Ok(()),
            ActiveDocumentArchiveLoadState::Ready | ActiveDocumentArchiveLoadState::Fault => Err(document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "terminal recursive document archive operation cannot be cancelled")),
        }
    }

    fn acknowledge_document_archive_load(&mut self, operation: u64) -> Result<(), Fault> {
        let active = self.ingest.document_archive_loads.get(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "unknown recursive document archive operation"))?;
        if !active.terminal() {
            return Err(document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive operation cannot be acknowledged before terminal status"));
        }
        if !active.inputs_are_empty() {
            return Err(document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "terminal recursive document archive operation still owns retained input"));
        }
        let mut active = self.ingest.document_archive_loads.remove(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "terminal recursive document archive authority changed before acknowledgement"))?;
        drop(std::mem::take(&mut active.fault));
        drop(active);
        Ok(())
    }

    fn document_load_archive(&self, files: &store::os_store::serialization::ArtifactPackFiles) -> Result<protocol::DocumentArchivePack, Fault> {
        let Effect::LoadDocument { pack, spr } = self.stores.stamp_load_document_identity(Effect::LoadDocument { pack: files.pack.clone(), spr: files.spr.clone() })? else {
            return Err(plugin_sdk_fault("the identity stamp answered something other than a document load"));
        };
        Ok(protocol::DocumentArchivePack { parent_pack: pack, parent_spr: spr, members: Vec::new() })
    }

    fn document_identity(&self) -> Option<String> {
        let id = &self.stores.store.envelope().id;
        (1..=512).contains(&id.chars().count()).then(|| id.clone())
    }

    async fn document_archive(&self) -> Result<protocol::DocumentArchivePack, Fault> {
        let parent_generation = self.stores.store.generation_now();
        let child_generation = self.child_composition.child_content_generation;
        let parent = store::os_store::text_format::print_document_pack(self.stores.store.envelope()).await.map_err(|error| error.into_fault())?;
        let members = self.child_composition.archive_member_entries().await?;
        if self.stores.store.generation_now() != parent_generation || self.child_composition.child_content_generation != child_generation {
            return Err(document_load_fault(DOCUMENT_LOAD_CHANGED_CODE, "document archive authority changed during generation-fenced export"));
        }
        Ok(protocol::DocumentArchivePack { parent_pack: parent.pack, parent_spr: parent.spr, members })
    }

    async fn load_config_pack(&mut self, files: &store::os_store::serialization::ArtifactPackFiles) -> Result<(), Fault> {
        let parsed: store::os_store::text_format::ParsedDocumentText<A::Config, A::ConfigMutation> = store::os_store::text_format::parse_document_pack(&files.pack, &files.spr, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| error.into_fault())?;
        self.stores.config_store.reset(parsed.into_envelope()).await.map_err(|error| error.into_fault())?;
        self.history.cache = None;
        Ok(())
    }

    async fn dispatch_config_command(&mut self, command_bytes: &mut Vec<u8>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<InvocationResult>, semio_framework_job::OriginalTurnFailure> {
        let log_generation_before = self.history.log_generation;
        let Some(result)=self.dispatch_ctx().dispatch_config_command_inner(command_bytes,meta,identity,cx).await? else{return Ok(None)};
        Ok(Some(self.render_ctx().finish_recorded(log_generation_before,"configCommand",result).await))
    }

    async fn ingest_operations(&mut self, mutations: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Vec<protocol::MergeReport>, Fault> {
        let envelopes = protocol::decode_envelopes(mutations).map_err(|error| error.into_fault())?;
        let mut reports = Vec::with_capacity(envelopes.len());
        let generation = self.stores.store.generation();
        for envelope in envelopes {
            reports.push(self.stores.store.ingest_remote(envelope).await.map_err(|error| error.into_fault())?);
        }
        self.history.cache = None;
        if self.stores.store.generation() != generation {
            self.dispatch_ctx().deliver_base_moved(identity, cx).await?;
        }
        Ok(reports)
    }

    async fn ingest_operations_text(&mut self, operations_text: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let mut mutations: Vec<A::Mutation> = Vec::new();
        for line in operations_text.lines().map(str::trim).filter(|line| !line.is_empty()) {
            mutations.push(A::Mutation::parse_op(line).map_err(|error| error.into_fault())?);
        }
        if mutations.is_empty() {
            return Ok(());
        }
        self.stores.store.dispatch(ArtifactCommand::Apply { mutations, transaction: None }, identity).await.map_err(|error| error.into_fault())?;
        self.history.cache = None;
        Ok(())
    }

    async fn document_text(&self) -> Result<store::os_store::text_format::ArtifactTextFiles, Fault> {
        store::os_store::text_format::print_document_text(self.stores.store.envelope()).await.map_err(|error| error.into_fault())
    }

    async fn document_pack(&self) -> Result<store::os_store::serialization::ArtifactPackFiles, Fault> {
        store::os_store::text_format::print_document_pack(self.stores.store.envelope()).await.map_err(|error| error.into_fault())
    }

    async fn attach_backbone(&mut self, backbone: store::os_store::backbone::Backbones) -> Result<(), Fault> {
        self.stores.store.attach_backbone(backbone).await.map_err(|error| error.into_fault())?;
        self.child_composition.announce_member_event_logs(&mut self.stores).await?;
        self.history.cache = None;
        Ok(())
    }

    async fn attach_hot_backbone(&mut self, backbone: store::os_store::backbone::Backbones) -> Result<(), Fault> {
        self.stores.store.attach_hot_backbone(backbone).await.map_err(|error| error.into_fault())?;
        self.history.cache = None;
        Ok(())
    }

    async fn tick_backbone(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<Vec<protocol::MergeReport>, Fault> {
        let generation = self.stores.store.generation();
        let reports = self.stores.store.tick_backbone_reports().await.map_err(|error| error.into_fault())?;
        if self.stores.store.generation() != generation {
            self.dispatch_ctx().deliver_base_moved(identity, cx).await?;
        }
        self.child_composition.follow_derivable_children(&mut self.stores, identity).await?;
        let folded_member_lanes = self.child_composition.fold_member_inbound(&mut self.stores, &mut self.history).await?;
        if !reports.is_empty() || folded_member_lanes != 0 {
            self.history.cache = None;
        }
        Ok(reports)
    }

    async fn detach_backbone(&mut self) -> Result<(), Fault> {
        self.stores.store.detach_backbone().map_err(|error| error.into_fault())?;
        self.history.cache = None;
        Ok(())
    }

    /// 🕰️ Framework-owned, snapshot-independent — served before any app body-key match.
    ///
    /// 🕹️ Task 5: materialized once, before either branch, then used to stamp EVERY
    /// `interaction_domain`-bound `UiTree` this render produces — see `stamp_and_cache_interaction_ui`.
    ///
    /// 🕹️ Materialized owned BEFORE the field-wise destructure below (same reasoning as
    /// `stamp_and_cache_interaction_ui`'s own comment): `interaction_hover`/`peer_presence`
    /// are only ever READ here, so a snapshot avoids an aliasing conflict against `&mut self`.
    async fn render(&mut self, body_key: &str, snapshot_override_json: Option<&str>, view_state: &ViewModel,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<()>,semio_framework_job::OriginalTurnFailure> {
        if self.pending.receive_render_failure(cx)?||cx.should_yield(){return Ok(None)}
        if self.pending.pending_render_tree.is_none()&&self.pending.pending_render_tree_close.is_none()&&self.pending.pending_render_tree_ready{self.pending.pending_render_tree_identity=None;self.pending.pending_render_tree_ready=false;}
        if let Some(identity)=self.pending.pending_render_tree_identity.as_mut(){identity.validate(cx)?;}else{self.pending.pending_render_tree_identity=Some(OriginalRenderTreeIdentity::capture(cx)?);}
        if self.pending.pending_render_tree.is_some(){return Ok(self.pending.pending_render_tree_ready.then_some(()));}
        let original_render_result:crate::OriginalRenderResult<()>=async {
        if let Some(section)=semio_framework::UiRefreshSection::from_body_key(body_key){
            let tree=semio_framework_plugin_reactor_app::original_render_section(self,section,view_state)?;
            self.pending.retain_pending_render_tree(tree);
            self.pending.pending_render_tree_ready=true;
            return Ok(Some(()));
        }
        self.render_ctx().refresh_cache().await?;
        if body_key == FRAMEWORK_HISTORY_BODY_KEY {
            let Some((_, _, _, history)) = self.history.cache.as_ref() else {
                return Err(plugin_sdk_fault("render cache unavailable after refresh").into());
            };
            let mut time_travel = self.time_travel.panel();
            if let Some(panel) = time_travel.as_mut() {
                self.time_travel_ref().resolve_time_travel_snaps(panel, view_state);
                self.time_travel_ref().resolve_time_travel_options(panel);
                self.time_travel_ref().resolve_time_travel_reference_labels(panel);
            }
            let reprojection = self.render_ref().reprojection_status();
            let pages = self.child_composition.history_mutation_pages(&self.stores, history, time_travel.as_ref(), view_state);
            let root =
                ui_history_panel(history, time_travel.as_ref(), reprojection.as_ref(), &pages, &self.tools.registry.controller_id, view_state.locale, A::ROLE == AppRole::Viewer, self.time_travel_ref().time_travel_busy(), view_state).await?;
            self.pending.retain_pending_render_tree(built_to_component_tree(root));
            self.pending.pending_render_tree_ready=true;
            return Ok(Some(()));
        }
        if let Some(root) = InstanceExtensions::render_body(&mut self.extension_ctx(), body_key, view_state) {
            self.pending.retain_pending_render_tree(built_to_component_tree(root?));
            self.pending.pending_render_tree_ready=true;
            return Ok(Some(()));
        }
        let interaction_state = self.interaction.interaction_state(&self.stores).await;
        let interaction_hover = self.interaction.interaction_hover.clone();
        let interaction_peers = std::sync::Arc::clone(&self.roster.peer_presence);
        let draft_references = self.time_travel.draft_references();
        let interaction = InteractionView { state: &interaction_state, hover: &interaction_hover, peers: interaction_peers.as_ref(), draft_references: &draft_references };
        let window_config = self.stores.window_config_store.capture(Some(view_state)).await?;
        let window_transient = self.stores.window_transient_store.capture(Some(view_state))?;
        if let Some(json) = snapshot_override_json {
            let snapshot: A::Snapshot = semio_framework_pack_json::from_json_str(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|plugin_sdk_fault(error.to_string()))?;
            let history = std::sync::Arc::clone(&self.history.cache.as_ref().ok_or_else(|| plugin_sdk_fault("render cache unavailable after refresh"))?.3);
            let doc = ArtifactView::new(&snapshot, history.as_ref()).with_instance(self.tools.instance_operation_owner.clone());
            let config = self.stores.config_store.snapshot().unwrap_or_else(|_| A::Config::default());
            let cfg = ConfigView { snapshot: &config, window: window_config.as_ref().map(|authority| &authority.snapshot) };
            let transient = self.stores.transient_store.current_root();
            let transient = TransientView { snapshot: transient.as_ref(), window: window_transient.as_ref().map(|authority| &authority.snapshot) };
            let Some(node) = A::render_with_request_context(&self.tools.instance_operation_owner, body_key, &doc, &cfg, view_state, &transient, &interaction,cx).await? else{return Ok(None)};
            self.pending.retain_pending_render_tree(node);
            self.render_ctx().stamp_pending_render_interaction_ui(&interaction_state, body_key).await?;
            self.pending.pending_render_tree_ready=true;
            return Ok(Some(()));
        }
        let operation_progress = self.tools.operation_progress_snapshot();
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let Some(node) = ({
            let VcsArtifactApp { app: _, cache, child_content_root, tool_runs, time_travel, tool_machines, .. } = self;
            let Some((_, snapshot, config, history)) = cache.as_ref() else {
                return Err(plugin_sdk_fault("render cache unavailable after refresh").into());
            };
            let doc = ArtifactView::with_render_context(
                time_travel.render_snapshot_or(tool_runs, tool_machines.overlay_or(snapshot)).as_ref(),
                history.as_ref(),
                ChildContentView::clone(time_travel.render_children_or(tool_runs, child_content_root)),
                render_operation,
                parent_document_id,
                None,
            )
            .await
            .with_tool_run(tool_runs.view_for(view_state.window_id.as_deref()))
            .with_instance(self.tools.instance_operation_owner.clone())
            .with_operations(&operation_progress);
            let cfg = ConfigView { snapshot: tool_machines.config_overlay_or(config).as_ref(), window: window_config.as_ref().map(|authority| tool_machines.window_config_overlay_or(&authority.snapshot)) };
            let transient = self.stores.transient_store.current_root();
            let transient = TransientView { snapshot: transient.as_ref(), window: window_transient.as_ref().map(|authority| &authority.snapshot) };
            A::render_with_request_context(&self.tools.instance_operation_owner, body_key, &doc, &cfg, view_state, &transient, &interaction,cx).await?
        }) else{return Ok(None)};
        self.pending.retain_pending_render_tree(node);
        InstanceExtensions::after_app_render(&mut self.extension_ctx(), body_key, view_state)?;
        self.render_ctx().stamp_pending_render_interaction_ui(&interaction_state, body_key).await?;
        self.pending.pending_render_tree_ready=true;
        Ok(Some(()))
        }.await;
        match original_render_result{Ok(tree)=>Ok(tree),Err(semio_framework_plugin_ownership::sdk_fault::OriginalRenderFailure::Original(failure))=>self.pending.retain_render_failure(semio_framework_plugin_ownership::sdk_fault::OriginalRenderFailure::Original(failure), cx),Err(source)=>self.pending.retain_render_failure(source, cx)}
    }

    fn original_render_tree_source<'a>(&'a mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<&'a mut Option<ComponentTree>,semio_framework_job::OriginalTurnFailure>{
        self.pending.pending_render_tree_identity.as_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original render source has no initiating identity"))?.validate(cx)?;
        if !self.pending.pending_render_tree_ready || self.pending.pending_render_failure.is_some() || self.pending.pending_render_compound.is_some() || self.pending.pending_render_close_failure.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original render source is not ready for donation").into());}
        Ok(&mut self.pending.pending_render_tree)
    }

    /// 🕹️ Same materialize-before-destructure shape the `render`/`window_measures`/`context_menu`
    /// paths use: `interaction_hover`/`peer_presence` are read-only here, so an owned snapshot
    /// avoids aliasing the `&mut self` field-wise destructure below.
    async fn window_engagements(&mut self, view_state: &ViewModel) -> HashMap<String, WindowEngagement> {
        if self.render_ctx().refresh_cache().await.is_err() {
            return HashMap::new();
        }
        let interaction_state = self.interaction.interaction_state(&self.stores).await;
        let interaction_hover = self.interaction.interaction_hover.clone();
        let interaction_peers = std::sync::Arc::clone(&self.roster.peer_presence);
        let draft_references = self.time_travel.draft_references();
        let interaction = InteractionView { state: &interaction_state, hover: &interaction_hover, peers: interaction_peers.as_ref(), draft_references: &draft_references };
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let VcsArtifactApp { window_config_store, window_transient_store, cache, child_content_root, transient_store, tool_runs, time_travel, tool_machines, .. } = self;
        let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(
            time_travel.render_snapshot_or(tool_runs, tool_machines.overlay_or(snapshot)).as_ref(),
            history.as_ref(),
            ChildContentView::clone(time_travel.render_children_or(tool_runs, child_content_root)),
            render_operation,
            parent_document_id,
            None,
        )
        .await
        .with_tool_run(tool_runs.view_for(view_state.window_id.as_deref()))
        .with_instance(self.tools.instance_operation_owner.clone());
        let mut engagements = HashMap::new();
        for window in view_state.window_instances.iter().take(UI_RESIDENT_SLOTS) {
            let Some(window_view_state) = view_state.for_window_instance(&window.id) else { continue };
            let window_config = match window_config_store.capture(Some(&window_view_state)).await {
                Ok(snapshot) => snapshot,
                Err(_) => continue,
            };
            let window_transient = match window_transient_store.capture(Some(&window_view_state)) {
                Ok(snapshot) => snapshot,
                Err(_) => continue,
            };
            let cfg = ConfigView { snapshot: tool_machines.config_overlay_or(config).as_ref(), window: window_config.as_ref().map(|authority| tool_machines.window_config_overlay_or(&authority.snapshot)) };
            let transient_root = transient_store.current_root();
            let transient = TransientView { snapshot: transient_root.as_ref(), window: window_transient.as_ref().map(|authority| &authority.snapshot) };
            let mut projected = A::window_engagements_with_request_context(&doc, &cfg, &window_view_state, &transient, &interaction).await;
            if let Some(engagement) = projected.remove(&window.id).or_else(|| projected.remove(&window.window_kind_id)) {
                engagements.insert(window.id.clone(), engagement);
            }
        }
        engagements
    }

    async fn window_measures(&mut self, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        if self.render_ctx().refresh_cache().await.is_err() {
            return HashMap::new();
        }
        let interaction_state = self.interaction.interaction_state(&self.stores).await;
        let interaction_hover = self.interaction.interaction_hover.clone();
        let interaction_peers = std::sync::Arc::clone(&self.roster.peer_presence);
        let draft_references = self.time_travel.draft_references();
        let interaction = InteractionView { state: &interaction_state, hover: &interaction_hover, peers: interaction_peers.as_ref(), draft_references: &draft_references };
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let VcsArtifactApp { window_config_store, cache, child_content_root, tool_runs, time_travel, tool_machines, .. } = self;
        let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(
            time_travel.render_snapshot_or(tool_runs, tool_machines.overlay_or(snapshot)).as_ref(),
            history.as_ref(),
            ChildContentView::clone(time_travel.render_children_or(tool_runs, child_content_root)),
            render_operation,
            parent_document_id,
            None,
        )
        .await
        .with_tool_run(tool_runs.view_for(view_state.window_id.as_deref()))
        .with_instance(self.tools.instance_operation_owner.clone());
        let mut measures = HashMap::new();
        for window in view_state.window_instances.iter().take(UI_RESIDENT_SLOTS) {
            let Some(window_view_state) = view_state.for_window_instance(&window.id) else { continue };
            let window_config = match window_config_store.capture(Some(&window_view_state)).await {
                Ok(snapshot) => snapshot,
                Err(_) => continue,
            };
            let cfg = ConfigView { snapshot: tool_machines.config_overlay_or(config).as_ref(), window: window_config.as_ref().map(|authority| tool_machines.window_config_overlay_or(&authority.snapshot)) };
            let mut projected = A::window_measures_with_request_context(&doc, &cfg, &window_view_state, &interaction).await;
            if let Some(window_measures) = projected.remove(&window.id).or_else(|| projected.remove(&window.window_kind_id)) {
                measures.insert(window.id.clone(), window_measures);
            }
        }
        measures
    }

    async fn tool_measures(&mut self, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        if self.render_ctx().refresh_cache().await.is_err() {
            return HashMap::new();
        }
        let window_config = match self.stores.window_config_store.capture(Some(view_state)).await {
            Ok(snapshot) => snapshot,
            Err(_) => return HashMap::new(),
        };
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let VcsArtifactApp { app: _, cache, child_content_root, tool_runs, time_travel, tool_machines, .. } = self;
        let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(
            time_travel.render_snapshot_or(tool_runs, tool_machines.overlay_or(snapshot)).as_ref(),
            history.as_ref(),
            ChildContentView::clone(time_travel.render_children_or(tool_runs, child_content_root)),
            render_operation,
            parent_document_id,
            None,
        )
        .await
        .with_tool_run(tool_runs.view_for(view_state.window_id.as_deref()))
        .with_instance(self.tools.instance_operation_owner.clone());
        let cfg = ConfigView { snapshot: tool_machines.config_overlay_or(config).as_ref(), window: window_config.as_ref().map(|authority| tool_machines.window_config_overlay_or(&authority.snapshot)) };
        A::tool_measures(&doc, &cfg, view_state).await
    }

    async fn app_catalogue_json(&mut self) -> String {
        A::app_catalogue_json().await
    }

    /// 🔒️ The app instance's RETAINED operation owner, handed to the poll for the same
    /// reason `render_with_request_context` gets it: window-scoped background work an app
    /// arms from here is latched on state only that owner holds, and a poll that cannot see
    /// it re-arms an already-pending chain once per refresh
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// ⏯️ The poll sees the LIVE RUN, exactly as the four render and measure passes above do.
    /// `ArtifactApp::pending_effects` is where a surface starts a run once its work is owed and
    /// finalizes a complete one, and that whole ladder is written over `doc.tool_run()` — handed a
    /// view that always answered `None` it could only ever read "no run": the generation3d preview
    /// run started, ticked, completed and was never finalized, so the next `toolRunStart` a gesture
    /// asked for was a no-op against a run the framework still held and the 3d preview stopped
    /// re-evaluating for the rest of the session
    /// (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`, contract §3.7). The document itself
    /// stays the COMMITTED snapshot: a poll decides about work over what has landed, never over a
    /// run's own provisional overlay.
    async fn pending_effects(&mut self, view: Option<&ViewModel>) -> Vec<Effect> {
        if self.render_ctx().refresh_cache().await.is_err() {
            return Vec::new();
        }
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let snapshot_is_admitted = {
            let (_, snapshot, _, _) = self.history.cache.as_ref().expect("cache refreshed above");
            A::mounted_job_prepare_snapshot_read(render_operation, snapshot.as_ref())
        };
        let snapshot_read = if snapshot_is_admitted {
            match self.stores.store.snapshot_read() {
                Ok(snapshot_read) => Some(snapshot_read),
                Err(_) => return Vec::new(),
            }
        } else {
            None
        };
        let instance_operation_owner = self.tools.instance_operation_owner.clone();
        let effects = {
            let VcsArtifactApp { app: _, cache, child_content_root, tool_runs, tool_machines, .. } = self;
            let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
            let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(child_content_root), render_operation, parent_document_id, snapshot_read)
                .await
                .with_instance(instance_operation_owner.clone())
                .with_tool_run(tool_runs.view_for(view.and_then(|view| view.window_id.as_deref())))
                .with_provisional_generation(tool_machines.provisional_generation());
            let cfg = ConfigView { snapshot: config.as_ref(), window: None };
            A::pending_effects(&instance_operation_owner, &doc, &cfg, view).await
        };
        self.stores.stamp_polled_load_document_effects(effects)
    }

    /// 🗂️ Every context menu is organized (D2 of the grouped-context-menu mechanism design) at this
    /// single funnel — a raw-vec emitter is grouped for free, and an emitter that already built its own
    /// `Menu::group(...)` rows is never re-flattened (`organize_context_menu` is idempotent on already-organized input).
    async fn context_menu(&mut self, request: &ContextMenuRequest, view_state: &ViewModel) -> Vec<ContextMenuItemSpec> {
        if self.render_ctx().refresh_cache().await.is_err() {
            return Vec::new();
        }
        let interaction_state = self.interaction.interaction_state(&self.stores).await;
        let interaction_hover = self.interaction.interaction_hover.clone();
        let interaction_peers = std::sync::Arc::clone(&self.roster.peer_presence);
        let draft_references = self.time_travel.draft_references();
        let interaction = InteractionView { state: &interaction_state, hover: &interaction_hover, peers: interaction_peers.as_ref(), draft_references: &draft_references };
        let window_config = match self.stores.window_config_store.capture(Some(view_state)).await {
            Ok(snapshot) => snapshot,
            Err(_) => return Vec::new(),
        };
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let VcsArtifactApp { app: _, cache, registry, child_content_root, instance_operation_owner, .. } = self;
        let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(child_content_root), render_operation, parent_document_id, None).await.with_instance(instance_operation_owner.clone());
        let cfg = ConfigView { snapshot: config.as_ref(), window: window_config.as_ref().map(|authority| &authority.snapshot) };
        let items = A::context_menu_with_request_context(request, &doc, &cfg, view_state, &interaction, registry).await;
        semio_framework_ui_layout::layout::organize_context_menu(items, &|id| registry.category_of(id))
    }

    /// 📤️ ABI-level media delivery for ONE port — the route `AppCommand::MediaOut` reaches
    /// through `plugin_produce_media`.
    ///
    /// 🎞️ Produces one document or declared media output while preserving the initiating pending turn.
    async fn produce_media(&mut self, port: &str, owner: &mut semio_framework_os_kernel::io::control::NativeSnapshotEncodeOwner<'_,'_>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<Option<MediaArtifact>,semio_framework_plugin_decl_app::MediaArtifactOperationFailure> {
        if port == NATURAL_FILE_PORT {
            let codec = A::natural_file_codec().await.ok_or_else(|| MediaArtifactError::Payload("natural file exporter is not mounted".into()))?;
            self.render_ctx().refresh_cache().await?;
            let snapshot = self.history.cache.as_ref().expect("cache refreshed above").1.as_ref();
            let data = A::encode_natural_file(snapshot).await.map_err(MediaArtifactError::Export)?;
            return Ok(Some(MediaArtifact {
                descriptor: MediaArtifactDescriptor {
                    edge_id: None,
                    port_id: Some(port.to_string()),
                    kind_id: Some(codec.format_kind.to_string()),
                    media_type: None,
                    wire: MediaWireFormat::Binary { format_kind: codec.format_kind.to_string() },
                    blob_hash: None,
                },
                data,
            }));
        }
        if port != "artifact:out" {
            let exported = owner.receive_nested::<Media, Result<Option<MediaArtifact>,semio_framework_plugin_decl_app::MediaArtifactOperationFailure>>(|slot, original| {
                let media = match ::semio_framework_async::poll::resolve_ready(self.export_media(port,cx)) {
                    Ok(Some(media))=>media,
                    Ok(None)=>return Ok(Ok(None)),
                    Err(error)=>return Ok(Err(error.into())),
                };
                *slot = Some(media);
                let media = slot.as_ref().unwrap();
                let artifact = match &media.payload {
                    MediaPayload::Intrinsic { schema, value } => MediaArtifact::encode_intrinsic(port, media.media_type, schema, value, original).map_err(|error| match error { MediaArtifactError::Value(error) => error, _ => media_projection_error() })?,
                    MediaPayload::Structured { schema, json } => {
                        let (port, schema, text) = copy_media_text_parts(port, schema, json, original)?;
                        original.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<String>(), ..Default::default() })?;
                        MediaArtifact { descriptor: MediaArtifactDescriptor { edge_id: None, port_id: Some(port), kind_id: None, media_type: Some(media.media_type), wire: MediaWireFormat::Binary { format_kind: schema }, blob_hash: None }, data: text.into_bytes() }
                    }
                    MediaPayload::Binary { format_kind, blob_hash } => {
                        let (port, format_kind, blob_hash) = copy_media_text_parts(port, format_kind, blob_hash, original)?;
                        MediaArtifact { descriptor: MediaArtifactDescriptor { edge_id: None, port_id: Some(port), kind_id: None, media_type: Some(media.media_type), wire: MediaWireFormat::Binary { format_kind }, blob_hash: Some(blob_hash) }, data: Vec::new() }
                    }
                };
                Ok(Ok(Some(artifact)))
            })??;
            return Ok(exported);
        }
        let archive = PluginApp::document_archive(self).await?;
        whole_document_media(port, self.artifact_schema().await.to_string(), &archive).await.map(Some).map_err(Into::into)
    }

    async fn prepare_media_import(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let source=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?);
        if let Some((_,original))=self.media.pending_media_import_permit.as_ref(){
            if *original!=source{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media import requires its retained original receiving source").into())}
            return Ok(!cx.should_yield()&&cx.retained_grant().maximum_items>0&&cx.retained_grant().maximum_depth>0)
        }
        let meta=ActionMeta{actor:self.stores.store.local_actor_id().0.clone(),instance_id:self.tools.live_runtime_instance_id.ok_or_else(||plugin_sdk_fault("media import requires its mounted live instance"))?,view_state:None};
        let Some(permit)=self.dispatch_ctx().admit_reserved_emit_producer("import-media",&meta,cx)?else{return Ok(false)};
        *self.media.pending_media_import_permit=Some((permit,source));Ok(false)
    }

    async fn consume_media(&mut self, port: &str, artifact: MediaArtifact, cx:&mut semio_framework_job::StepContext<'_>) -> Result<MediaConsumption, MediaArtifactError> {
        if port != NATURAL_FILE_PORT {
            let schema_now = self.artifact_schema().await.to_string();
            return match artifact.descriptor.wire {
                MediaWireFormat::Document { schema } if schema == schema_now => whole_document_archive(&artifact.data).await.map(MediaConsumption::DocumentLoad),
                MediaWireFormat::Document { schema } => Err(MediaArtifactError::SchemaMismatch { expected: schema_now, found: schema }),
                MediaWireFormat::Binary { format_kind } => Err(MediaArtifactError::NoImporter(format_kind)),
                MediaWireFormat::Intrinsic { .. } => {
                    let media = {
                        let mut accept = |_| true;
                        let mut control = semio_framework_value::NativeDecodeControl::new(ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES, &mut accept);
                        artifact.decode_intrinsic(&mut control)?
                    };
                    let instance_id = self.tools.live_runtime_instance_id.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "intrinsic media import requires a bound live app instance"))?;
                    let actor = self.stores.store.local_actor_id().0.clone();
                    self.dispatch_ctx().dispatch_import_media(port, media, &ActionMeta { actor, instance_id, view_state: None }, cx).await.map(|_| MediaConsumption::Applied).map_err(MediaArtifactError::Import)
                }
            };
        }
        let codec = A::natural_file_codec().await.ok_or_else(|| MediaArtifactError::NoImporter(port.to_string()))?;
        let MediaWireFormat::Binary { format_kind } = artifact.descriptor.wire else {
            return Err(MediaArtifactError::NoImporter(codec.format_kind.to_string()));
        };
        if format_kind != codec.format_kind {
            return Err(MediaArtifactError::NoImporter(format_kind));
        }
        let instance_id = self.tools.live_runtime_instance_id.ok_or_else(|| MediaArtifactError::Payload("natural file import requires a bound live app instance".into()))?;
        let actor = self.stores.store.local_actor_id().0.clone();
        let media_type = A::io().await.map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
        let media = natural_file_input_media(codec, media_type, artifact.data);
        self.dispatch_ctx().dispatch_import_media(port, media, &ActionMeta { actor, instance_id, view_state: None }, cx)
            .await
            .map(|_| MediaConsumption::Applied)
            .map_err(|fault| MediaArtifactError::Payload(fault.message))
    }

    /// 🎞️ Batch-only adapter. Registered interactive outputs are driven through the same
    /// externally addressable submission API; this loop is not used by UI/workflow entry points.
    async fn export_media(&mut self,port:&str,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<Media>,semio_framework_job::OriginalTurnFailure>{
        if self.media.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(None)}
        if self.tools.app_tool_registrations.keys().any(|verb|verb.strip_prefix("export-media:")==Some(port)){
            let operation_id=if let Some(id)=self.media.current_media_export{if self.media.media_exports.get(id).is_none_or(|active|active.port!=port){return self.media.retain_media_submission_refusal(MediaError::Payload(port.to_string(),"current export belongs to a different original port".into()), cx)}id}else{let Some(instance)=self.tools.live_runtime_instance_id else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"batch media requires its mounted instance").into())};if !self.media.receive_owned_media_export(&mut self.stores, &mut self.tools, port, instance, None, cx).await?{return Ok(None)}self.media.current_media_export.ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"batch media publication lost its live operation"))?};
            if self.media.media_export_preparation.is_some(){
                self.media.snapshot_retirements.get(operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"batch media lost its original supplier"))?.validate(cx)?;
                let grant=cx.retained_grant();self.media.close_media_submission_refusal(&mut self.tools, cx, grant)?;return Ok(None)
            }
            return match self.media.receive_owned_media_poll(&mut self.stores, operation_id, cx).await?{None|Some(ArtifactMediaExportPoll::Running{..})=>Ok(None),Some(ArtifactMediaExportPoll::Complete(result))=>match result.into_batch_media(){Ok(media)=>Ok(Some(media)),Err(source)=>self.media.retain_media_submission_refusal(source, cx)},Some(ArtifactMediaExportPoll::Cancelled)=>self.media.retain_media_submission_refusal(MediaError::Payload(port.to_string(),"media export cancelled".into()), cx),Some(ArtifactMediaExportPoll::Failed(detail))=>self.media.retain_media_submission_refusal(MediaError::Payload(port.to_string(),detail), cx)};
        }
        self.render_ctx().refresh_cache().await?;
        let render_operation=self.tools.live_render_operation(&self.mounted_policy, &self.stores);let parent_document_id=self.stores.store.envelope().id.clone();
        let VcsArtifactApp{app:_,cache,child_content_root,transient_store,instance_operation_owner,..}=self;
        let(_,snapshot,config,history)=cache.as_ref().expect("cache refreshed above");
        let doc=ArtifactView::with_render_context(snapshot.as_ref(),history.as_ref(),ChildContentView::clone(child_content_root),render_operation,parent_document_id,None).await.with_instance(instance_operation_owner.clone());let _cfg=ConfigView{snapshot:config.as_ref(),window:None};let transient=transient_store.current_root();
        match A::export_media_with_request_context(instance_operation_owner,port,&doc,&TransientView{snapshot:transient.as_ref(),window:None}).await{Ok(media)=>Ok(Some(media)),Err(source)=>self.media.retain_media_submission_refusal(source, cx)}
    }

    async fn submit_media_export(&mut self,port:&str,receiver:&mut Option<ArtifactMediaExportHandle>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.media.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(false)}
        let Some(app_instance_id)=self.tools.live_runtime_instance_id else{return self.media.retain_media_submission_refusal::<()>(MediaError::Payload(port.to_string(),"resumable media export requires a bound live app instance".into()), cx).map(|_|false)};
        self.media.receive_owned_media_export(&mut self.stores, &mut self.tools, port, app_instance_id, Some(receiver), cx).await
    }

    async fn submit_media_export_checked(&mut self,port:&str,expected_parent_document_id:&str,expected_base_revision:u64,receiver:&mut Option<ArtifactMediaExportHandle>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.media.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(false)}
        let parent_document_id=self.stores.store.envelope().id.as_str();let content_revision=self.stores.store.content_revision();let base_revision=u64::from_be_bytes(content_revision[..8].try_into().expect("revision lane width"));
        if parent_document_id!=expected_parent_document_id||base_revision!=expected_base_revision{return self.media.retain_media_submission_refusal::<()>(MediaError::Payload(port.to_string(),"media export resource authority is stale".into()), cx).map(|_|false)}
        let Some(app_instance_id)=self.tools.live_runtime_instance_id else{return self.media.retain_media_submission_refusal::<()>(MediaError::Payload(port.to_string(),"resumable media export requires a bound live app instance".into()), cx).map(|_|false)};
        self.media.receive_owned_media_export(&mut self.stores, &mut self.tools, port, app_instance_id, Some(receiver), cx).await
    }

    async fn poll_media_export(&mut self,handle:semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportHandleLoan<'_>,receiver:&mut Option<ArtifactMediaExportPoll>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if self.media.receive_media_submission_refusal(cx)?||cx.should_yield(){return Ok(false)}
        self.media.snapshot_retirements.get(handle.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media polling lost its original source"))?.validate(cx)?;
        if receiver.is_some(){return Ok(true)}
        if self.media.media_exports.get(handle.operation_id.0).is_none_or(|active|active.handle.original_loan()!=handle){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"media polling requires its exact original live handle").into())}
        let grant=cx.retained_grant();let bytes=size_of::<ArtifactMediaExportPoll>()+size_of::<bool>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
        let Some(poll)=self.media.receive_owned_media_poll(&mut self.stores, handle.operation_id.0, cx).await?else{return Ok(false)};
        if matches!(poll,ArtifactMediaExportPoll::Complete(_)){self.media.media_closures.get_mut(handle.operation_id.0).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"terminal media poll lost its original close owner"))?.download_pending=true}
        *receiver=Some(poll);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }

    async fn cancel_media_export(&mut self, handle: &ArtifactMediaExportHandle) -> Result<(), MediaError> {
        self.media.cancel_owned_media_export(handle).await
    }

    async fn retain_media_export_result(&mut self,handle:semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportHandleLoan<'_>,source:&mut Option<ArtifactMediaExportPoll>,status:&mut Option<semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportPublishedStatus>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let operation_id=handle.operation_id.0;let identity=(cx.operation(),cx.generation(),cx.original_source_parent()?.source_identity()?);
        if handle.parent_document_id.len()>semio_framework_plugin_decl_views::media_contract::ARTIFACT_OUTPUT_METADATA_BYTES{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"download handle document metadata exceeds its original limit").into())}
        self.media.snapshot_retirements.get(operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving lost its original source"))?.validate(cx)?;
        if self.media.media_download_preparation.is_none()&&status.is_some()&&source.is_none(){return if self.media.segmented_downloads.get(operation_id).is_some_and(|output|output.owns_media_export_loan(handle)){Ok(true)}else{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"published download lost its exact registry recipient").into())}}
        if self.media.media_download_preparation.is_none(){
            let active=self.media.media_closures.get(operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving lost its original completed operation"))?;
            if active.handle.original_loan()!=handle{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving requires its exact completed business handle").into())}
            if let Some(ArtifactMediaExportPoll::Complete(result))=source.as_ref(){if !result.chunks.same_operation(&active.output_chunks){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving requires its original supplied chunks").into())}}
        }
        let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled(){return Ok(false)}
        if self.media.media_download_preparation.is_none(){
            if status.is_some()||!self.media.segmented_downloads.can_insert(operation_id){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving slot is occupied").into())}
            let Some(ArtifactMediaExportPoll::Complete(result))=source.as_ref()else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving requires its retained completion").into())};
            let bytes=size_of::<semio_framework_plugin_document_media::media_export::MediaDownloadPreparation>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
            *self.media.media_download_preparation=Some(semio_framework_plugin_document_media::media_export::MediaDownloadPreparation{operation_id,source_identity:identity,document_identity:(handle.parent_document_id.as_ptr()as usize,handle.parent_document_id.len()),mime_identity:(result.mime_type.as_ptr()as usize,result.mime_type.len()),chunks_identity:result.chunks.original_identity(),coordinates:(handle.app_instance_id,handle.base_revision,handle.generation),buffers:ManuallyDrop::new(Some(std::array::from_fn(|_|Vec::new()))),buffers_close:ManuallyDrop::new(None),stage:0,handle:ManuallyDrop::new(None),handle_close:ManuallyDrop::new(None),output:ManuallyDrop::new(None),output_close:ManuallyDrop::new(None),published:false});
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        let original=self.media.media_download_preparation.as_mut().unwrap();if original.operation_id!=operation_id||original.source_identity!=identity||original.coordinates!=(handle.app_instance_id,handle.base_revision,handle.generation)||original.document_identity!=(handle.parent_document_id.as_ptr()as usize,handle.parent_document_id.len()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download publication requires its unchanged original source and handle").into())}
        if let Some(ArtifactMediaExportPoll::Complete(result))=source.as_ref(){if original.chunks_identity!=result.chunks.original_identity(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download receiving lost its same actual result root").into())}}
        if original.published{
            if source.is_some()||status.is_none(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"published download has inconsistent original receiving slots").into())}
            let bytes=size_of::<semio_framework_plugin_document_media::media_export::MediaDownloadPreparation>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}self.media.media_download_preparation.take();cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(true)
        }
        if original.stage<2{
            let Some(ArtifactMediaExportPoll::Complete(result))=source.as_ref()else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"partial download metadata lost its retained result").into())};if original.mime_identity!=(result.mime_type.as_ptr()as usize,result.mime_type.len()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"partial download requires its unchanged original MIME source").into())}
            let input=if original.stage==0{handle.parent_document_id.as_bytes()}else{result.mime_type.as_bytes()};let output=&mut original.buffers.as_mut().unwrap()[original.stage];
            if grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
            if output.capacity()==0&&!input.is_empty(){if grant.maximum_capacity_bytes<input.len(){return Ok(false)}output.try_reserve_exact(input.len()).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"download metadata allocation refused"))?;cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:output.capacity(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
            if output.len()<input.len(){let count=(input.len()-output.len()).min(grant.maximum_copy_bytes);if count==0{return Ok(false)}let start=output.len();output.extend_from_slice(&input[start..start+count]);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
            if grant.maximum_copy_bytes<size_of::<usize>(){return Ok(false)}original.stage+=1;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<usize>(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        if original.handle.is_none()&&status.is_none(){
            let Some(ArtifactMediaExportPoll::Complete(result))=source.as_ref()else{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download metadata donation lost its original result").into())};let total_bytes=result.chunks.bytes()as u64;let bytes=size_of::<ArtifactMediaExportHandle>()+size_of::<semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportPublishedStatus>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
            let buffers=original.buffers.as_mut().unwrap();let document=String::from_utf8(std::mem::take(&mut buffers[0])).expect("copied original document remains UTF-8");let mime_type=String::from_utf8(std::mem::take(&mut buffers[1])).expect("copied original MIME remains UTF-8");*original.handle=Some(ArtifactMediaExportHandle{app_instance_id:handle.app_instance_id,parent_document_id:document,operation_id:handle.operation_id,base_revision:handle.base_revision,generation:handle.generation});*status=Some(semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportPublishedStatus{mime_type,total_bytes});cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        if original.output.is_none(){let _=ArtifactDownloadOutput::receive_media_export_original(&mut original.handle,source,&mut original.output,cx)?;return Ok(false)}
        let bytes=size_of::<ArtifactDownloadOutput>()+2*size_of::<bool>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}if !self.media.segmented_downloads.can_insert(operation_id){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download registry receiving slot changed before publication").into())}
        self.media.segmented_downloads.insert_admitted(operation_id,original.output.take().unwrap());original.published=true;if let Some(active)=self.media.media_closures.get_mut(operation_id){active.download_pending=false}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(false)
    }

    async fn take_media_export_chunk(&mut self,handle:semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportHandleLoan<'_>,receiver:&mut Option<Vec<u8>>,terminal:&mut bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        cx.original_source_parent()?;if receiver.is_some(){return Ok(true)}let operation_id=handle.operation_id.0;
        if *terminal&&self.media.segmented_downloads.get(operation_id).is_none(){return Ok(true)}
        self.media.snapshot_retirements.get(operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download chunk receiving lost its original source"))?.validate(cx)?;
        let output=self.media.segmented_downloads.get(operation_id).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download chunk receiving has no exact output"))?;
        if !output.owns_media_export_loan(handle){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download chunk receiving requires its exact handle").into())}
        if !*terminal{if !output.chunks.take_chunk_original(receiver,terminal,cx)?{return Ok(false)}return Ok(receiver.is_some())}
        if !output.terminal_is_empty(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"download EOF retains unread original chunks").into())}
        let grant=cx.retained_grant();let bytes=size_of::<ArtifactDownloadOutput>()+size_of::<semio_framework_plugin_document_media::media_export::MediaDownloadRetirement>();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
        if !self.media.segmented_closures.can_insert(operation_id){return Ok(false)}let output=self.media.segmented_downloads.remove(operation_id).expect("checked exact original download");self.media.segmented_closures.insert_admitted(operation_id,semio_framework_plugin_document_media::media_export::MediaDownloadRetirement::pending(output));cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }

    async fn take_segmented_download_chunk(&mut self, operation_id: u64) -> Result<Option<Vec<u8>>, Fault> {
        let output = self.media.segmented_downloads.get(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-segmented-download"), format!("unknown segmented download operation {operation_id}")))?;
        let chunk = output.chunks.take_chunk()?;
        if chunk.is_none() {
            let output =
                self.media.segmented_downloads.remove(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-download-authority"), "terminal segmented download changed before exact removal"))?;
            if !output.terminal_is_empty() {
                self.media.segmented_downloads.insert_admitted(operation_id, output);
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.segmented-download-terminal-not-empty"), "terminal segmented download retained unread chunks"));
            }
            drop(output);
        }
        Ok(chunk)
    }

    async fn import_media(&mut self, port: &str, media: Media, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let log_generation_before = self.history.log_generation;
        let result = self.dispatch_ctx().dispatch_import_media(port, media, meta, cx).await?;
        Ok(self.render_ctx().finish_recorded(log_generation_before, &format!("import-media:{port}"), result).await)
    }

    async fn media_fingerprint(&mut self, port: &str) -> Result<MediaFingerprint, MediaError> {
        self.render_ctx().refresh_cache().await.map_err(|error| MediaError::Payload(port.to_string(), error.message))?;
        let render_operation = self.tools.live_render_operation(&self.mounted_policy, &self.stores);
        let parent_document_id = self.stores.store.envelope().id.clone();
        let VcsArtifactApp { app: _, cache, child_content_root, instance_operation_owner, .. } = self;
        let (_, snapshot, config, history) = cache.as_ref().expect("cache refreshed above");
        let doc = ArtifactView::with_render_context(snapshot.as_ref(), history.as_ref(), ChildContentView::clone(child_content_root), render_operation, parent_document_id, None).await.with_instance(instance_operation_owner.clone());
        let _cfg = ConfigView { snapshot: config.as_ref(), window: None };
        A::media_fingerprint(port, &doc).await
    }
}
