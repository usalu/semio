#!/usr/bin/env python3
"""🤖️ P8 post-publish patch set: an agent and a human run the SAME code for the same verb.

The agent lane's prepare phase (`VcsArtifactApp::preview_addressed_action`, reached by the MCP gateway's typed command
frame) called `ArtifactApp::handle`, while the shell lane runs the verb's retained tool job. For every retained route the two
differ: flow's `handle` refuses retained routes by design (`flow.retained.legacy-dispatch`), cad's needs an operation-bound
artifact view, sequence the job's child context, and every lane `handle` does not see (children, effects, window config) is
lost. The preview now builds the SAME job the shell builds (`A::build_tool_job` over the SAME captured roots), runs its own
preflight and work to their emit without publishing, and closes it through its own close protocol. An emit whose effect
the agent transaction cannot carry — owned children, a whole-document replacement, a file download or request, extension
calls, follow-up tasks — is refused by name instead of committing nothing; presentation-only lanes (window config, a
selection follow-up, notices) have no meaning for an agent and are dropped exactly as before.

Parts (anchored, each exactly once; ABI-neutral — no WIT, export, pack-schema, codec or frame change):
  A  action-bus `ToolPayload::into_inner` — takes back the typed payload a builder put in.
  B  retained-command `ArtifactRetainedCommandJob::preview_emit` — preflight + work to the emit, nothing published.
  C  SDK `capture_typed_command_roots` — the ONE capture both lanes run against; `start_typed_command_operation` uses it.
  D  SDK `preview_addressed_action` runs the retained job; refuses uncarried lanes by name.
  E  the architect law's pinned agent divergences move with it (exports + import picker are refused by name now).
Usage: p8-agent-lane.py --dry-run | --write"""
from p8_patch import ROOT, finish, replace

SDK = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
RETAINED = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs"
BUS = ROOT / "🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs"

replace(
    "A",
    BUS,
    """    fn downcast<T: Send + 'static>(self) -> Result<T, ToolJobFactoryError> {
        self.value.downcast::<T>().map(|value| *value).map_err(|_| ToolJobFactoryError::new(format!("tool payload '{}' has the wrong Rust payload type", self.schema_id)))
    }
}""",
    """    fn downcast<T: Send + 'static>(self) -> Result<T, ToolJobFactoryError> {
        self.value.downcast::<T>().map(|value| *value).map_err(|_| ToolJobFactoryError::new(format!("tool payload '{}' has the wrong Rust payload type", self.schema_id)))
    }

    /// 🔓️ Takes back the typed payload a builder put in, or hands this payload back unchanged when it carries
    /// another type — the agent lane's prepare phase reads a retained route's work without dispatching it.
    pub fn into_inner<T: Send + 'static>(self) -> Result<T, Self> {
        let schema_id = self.schema_id;
        self.value.downcast::<T>().map(|value| *value).map_err(|value| Self { schema_id, value })
    }
}""",
)

replace(
    "B",
    RETAINED,
    """    pub fn from_wire_with_checkpoint(payload: ArtifactRetainedCommandPayload<A>, input: RetainedToolWireInput, checkpoint: RetainedToolWireInput) -> Self {
        Self::from_payload(payload, Some(input), Some(checkpoint))
    }
""",
    """    pub fn from_wire_with_checkpoint(payload: ArtifactRetainedCommandPayload<A>, input: RetainedToolWireInput, checkpoint: RetainedToolWireInput) -> Self {
        Self::from_payload(payload, Some(input), Some(checkpoint))
    }

    /// 👁️ The agent lane's prepare phase: runs this job's own preflight and work to the emit the shell lane
    /// would publish, and publishes nothing. The job stays owned by the caller, which closes it through its
    /// ordinary close protocol ([`InteractiveJob::begin_close`] / [`InteractiveJob::close_step`]).
    pub fn preview_emit(&mut self) -> Result<(Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, EphemeralEmit<A>), Fault> {
        let (Some(command), Some(snapshot), Some(config), Some(history), Some(interaction), Some(hover), Some(operation), Some(work)) =
            (self.command.as_ref(), self.snapshot.as_ref(), self.config.as_ref(), self.history.as_ref(), self.interaction_state.as_ref(), self.interaction_hover.as_ref(), self.operation.as_ref(), self.work.as_mut())
        else {
            return Err(Fault::from("retained command preview owner is absent"));
        };
        let extent = work.extent(command, snapshot, interaction, self.context.as_deref()).ok_or_else(|| Fault::from("retained command work refused the command before any capacity was measured"))?;
        if extent == 0 || extent > self.maximum_work_items {
            return Err(Fault::from("retained command exceeds semantic work capacity"));
        }
        loop {
            match work.step(&ArtifactCommandInputs { command, snapshot, config, history, interaction, hover, context: self.context.as_deref(), operation })? {
                ArtifactCommandWorkStep::Replay { .. } | ArtifactCommandWorkStep::Progress { .. } => {}
                ArtifactCommandWorkStep::Complete(emit) => return Ok((emit, EphemeralEmit::default())),
                ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral } => return Ok((emit, ephemeral)),
            }
        }
    }
""",
)

CAPTURE_OLD = """            let verb = admission.verb.clone();
            self.refresh_cache().await?;
            let draft_snapshot = self.draft_store.snapshot_root();
            let interaction_state = self.interaction_store.snapshot_root();
            let interaction_hover = self.interaction_hover.clone();
            let (snapshot, config, history) = self.command_cache_inputs();
            let children = std::sync::Arc::new(ChildContentView::clone(&self.child_content_root));
            let presence_peers = self.presence_store.peers_root();
            let transient = self.transient_store.current_root();
            let peer_presence = std::sync::Arc::clone(&self.peer_presence);
            let canonical_base_revision = self.store.content_revision();
            let base_revision = semio_framework_job::RevisionId(u64::from_be_bytes(canonical_base_revision[..8].try_into().expect("revision lane width")));
            let generation = semio_framework_job::Generation(self.store.generation());
            let config_generation = self.config_store.generation();
            let draft_generation = self.draft_store.generation();
            let presence_generation = self.presence_store.generation().await;
            let transient_generation = self.transient_store.generation().await;
            let window_config_authority = self.window_config_store.capture(meta.view_state.as_ref()).await?;
            let targeted_window_transient_view = match A::retained_window_transient_target(command.as_ref()) {
                Some((window_id, expected_kind)) => {
                    let view = meta.view_state.as_ref().ok_or_else(|| plugin_sdk_fault("targeted window transient capture requires an exact ViewModel roster"))?;
                    let window = view.window_instances.iter().find(|window| window.id == window_id).ok_or_else(|| plugin_sdk_fault("targeted window transient capture requires an attached window instance"))?;
                    if window.window_kind_id != expected_kind {
                        return Err(plugin_sdk_fault("targeted window transient capture does not match the command owner's expected window kind"));
                    }
                    Some(view.for_window_instance(window_id).ok_or_else(|| plugin_sdk_fault("targeted window transient capture lost its attached window instance"))?)
                }
                None => None,
            };
            let window_transient_authority = self.window_transient_store.capture(targeted_window_transient_view.as_ref().or(meta.view_state.as_ref()))?;
"""
CAPTURE_NEW = """            let verb = admission.verb.clone();
            let TypedCommandRoots {
                snapshot,
                config,
                history,
                children,
                draft_snapshot,
                interaction_state,
                interaction_hover,
                presence_peers,
                transient,
                peer_presence,
                canonical_base_revision,
                base_revision,
                generation,
                config_generation,
                draft_generation,
                presence_generation,
                transient_generation,
                window_config_authority,
                window_transient_authority,
            } = self.capture_typed_command_roots(command.as_ref(), meta).await?;
"""
replace("C", SDK, CAPTURE_OLD, CAPTURE_NEW)

replace(
    "C",
    SDK,
    """        async fn start_typed_command_operation(
            &mut self,""",
    """        /// 📸️ The roots one typed command runs against, captured once and identically for the shell lane
        /// that mounts the operation ([`Self::start_typed_command_operation`]) and for the agent lane's prepare
        /// phase that only previews it ([`Self::preview_addressed_action`]) — so the two lanes can never run
        /// the same verb against different state.
        async fn capture_typed_command_roots(&mut self, command: &A::Command, meta: &ActionMeta) -> Result<TypedCommandRoots<A>, Fault> {
            self.refresh_cache().await?;
            let (snapshot, config, history) = self.command_cache_inputs();
            let canonical_base_revision = self.store.content_revision();
            let window_config_authority = self.window_config_store.capture(meta.view_state.as_ref()).await?;
            let targeted_window_transient_view = match A::retained_window_transient_target(command) {
                Some((window_id, expected_kind)) => {
                    let view = meta.view_state.as_ref().ok_or_else(|| plugin_sdk_fault("targeted window transient capture requires an exact ViewModel roster"))?;
                    let window = view.window_instances.iter().find(|window| window.id == window_id).ok_or_else(|| plugin_sdk_fault("targeted window transient capture requires an attached window instance"))?;
                    if window.window_kind_id != expected_kind {
                        return Err(plugin_sdk_fault("targeted window transient capture does not match the command owner's expected window kind"));
                    }
                    Some(view.for_window_instance(window_id).ok_or_else(|| plugin_sdk_fault("targeted window transient capture lost its attached window instance"))?)
                }
                None => None,
            };
            let window_transient_authority = self.window_transient_store.capture(targeted_window_transient_view.as_ref().or(meta.view_state.as_ref()))?;
            Ok(TypedCommandRoots {
                snapshot,
                config,
                history,
                children: std::sync::Arc::new(ChildContentView::clone(&self.child_content_root)),
                draft_snapshot: self.draft_store.snapshot_root(),
                interaction_state: self.interaction_store.snapshot_root(),
                interaction_hover: self.interaction_hover.clone(),
                presence_peers: self.presence_store.peers_root(),
                transient: self.transient_store.current_root(),
                peer_presence: std::sync::Arc::clone(&self.peer_presence),
                canonical_base_revision,
                base_revision: semio_framework_job::RevisionId(u64::from_be_bytes(canonical_base_revision[..8].try_into().expect("revision lane width"))),
                generation: semio_framework_job::Generation(self.store.generation()),
                config_generation: self.config_store.generation(),
                draft_generation: self.draft_store.generation(),
                presence_generation: self.presence_store.generation().await,
                transient_generation: self.transient_store.generation().await,
                window_config_authority,
                window_transient_authority,
            })
        }

        async fn start_typed_command_operation(
            &mut self,""",
)

replace(
    "C",
    SDK,
    """    /// 📬️ Operation-owned completion cell shared by an app factory's job and the exact framework
    /// commit path. It is single-assignment and consumed exactly once after freshness validation.
    pub struct ArtifactToolCompletion<A: ArtifactApp> {""",
    """    /// 📸️ Every root one typed command is reduced against — see `VcsArtifactApp::capture_typed_command_roots`.
    struct TypedCommandRoots<A: ArtifactApp> {
        snapshot: std::sync::Arc<A::Snapshot>,
        config: std::sync::Arc<A::Config>,
        history: std::sync::Arc<HistoryView>,
        children: std::sync::Arc<ChildContentView>,
        draft_snapshot: std::sync::Arc<A::Draft>,
        interaction_state: std::sync::Arc<protocol::InteractionState>,
        interaction_hover: InteractionHoverState,
        presence_peers: std::sync::Arc<store::PresencePeersRoot<A::Presence>>,
        transient: std::sync::Arc<A::Transient>,
        peer_presence: std::sync::Arc<PeerPresenceRoot>,
        canonical_base_revision: [u8; 32],
        base_revision: semio_framework_job::RevisionId,
        generation: semio_framework_job::Generation,
        config_generation: u64,
        draft_generation: u64,
        presence_generation: u64,
        transient_generation: u64,
        window_config_authority: Option<super::window_config::WindowConfigAuthority>,
        window_transient_authority: Option<super::window_transient::WindowTransientAuthority>,
    }

    /// 📬️ Operation-owned completion cell shared by an app factory's job and the exact framework
    /// commit path. It is single-assignment and consumed exactly once after freshness validation.
    pub struct ArtifactToolCompletion<A: ArtifactApp> {""",
)

PREVIEW_OLD = """            let command = A::command_from_action(&address.action_id, Some(&args)).await?;
            let emit = {
                self.refresh_cache().await?;
                let (snapshot, config, history) = self.command_cache_inputs();
                let draft = self.draft_store.snapshot_root();
                let interaction_state = self.interaction_store.snapshot_root();
                let interaction_hover = self.interaction_hover.clone();
                let interaction_peers = std::sync::Arc::clone(&self.peer_presence);
                let doc = ArtifactView::new(snapshot.as_ref(), history.as_ref());
                let cfg = ConfigView { snapshot: config.as_ref(), window: None };
                let draft = DraftView { snapshot: draft.as_ref() };
                let interaction = InteractionView { state: interaction_state.as_ref(), hover: &interaction_hover, peers: interaction_peers.as_ref() };
                A::handle(&command, &doc, &cfg, &interaction, Some(&view), &draft, &store::EngineHandles::empty()).await?
            };
"""
PREVIEW_NEW = """            let command = A::command_from_action(&address.action_id, Some(&args)).await?;
            let emit = self.preview_retained_command(Box::new(command), &proof, &ActionMeta { view_state: Some(view.clone()), ..meta.clone() }).await?;
            let uncarried = [
                ("owned children", !emit.child_emits.is_empty()),
                ("a whole-document replacement", emit.effects.iter().any(|effect| matches!(effect, Effect::LoadDocument { .. }))),
                ("a file download", emit.effects.iter().any(|effect| matches!(effect, Effect::DownloadMediaExport { .. } | Effect::IconRenderExport { .. }))),
                ("a file request", emit.effects.iter().any(|effect| matches!(effect, Effect::RequestFileOpen { .. } | Effect::RequestMediaFrames { .. }))),
                ("extension calls", !emit.extension_invocations.is_empty()),
                ("follow-up tasks", !emit.tasks.is_empty()),
            ]
            .into_iter()
            .filter(|(_, published)| *published)
            .map(|(lane, _)| lane)
            .collect::<Vec<_>>();
            if !uncarried.is_empty() {
                return Err(Fault::new(
                    FaultOrigin::Framework,
                    FaultCode::new("interactive-job.agent-lane-uncarried"),
                    format!("action '{}' publishes {} that an agent transaction cannot carry; it runs only from the shell", address.action_id, uncarried.join(", ")),
                ));
            }
"""
replace("D", SDK, PREVIEW_OLD, PREVIEW_NEW)

replace(
    "D",
    SDK,
    """        /// 🧩️ A typed frame IS an owner-qualified invocation — the exact wire `AppCommand::Command`""",
    """        /// 👁️ Builds the SAME retained job the shell lane builds for this command (`A::build_tool_job` over
        /// [`Self::capture_typed_command_roots`]), runs its own preflight and work to the emit without publishing
        /// anything, and closes it through its own close protocol. A route whose builder answers anything but a
        /// retained command payload is refused by name: the agent lane never substitutes other code for it.
        async fn preview_retained_command(&mut self, command: Box<A::Command>, proof: &QualifiedToolProof, meta: &ActionMeta) -> Result<Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, Fault> {
            use semio_framework_job::InteractiveJob;
            let verb = A::command_id(&command).await.to_string();
            let roots = self.capture_typed_command_roots(command.as_ref(), meta).await?;
            let operation_id = self.admit_typed_operation_slot().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-capacity"), "every fixed typed-operation and segmented-output slot already owns a live operation"))?;
            let seed_handle = artifact_handle_of(&format!("{}/{}/{verb}/{}", meta.instance_id, self.tool_job_controller_id, roots.base_revision.0)).await;
            let operation = semio_framework_job::Operation::new(operation_id, roots.base_revision, roots.generation, (seed_handle.0 as u64) ^ ((seed_handle.0 >> 64) as u64));
            let completion = ArtifactToolCompletion::<A>::new();
            let context = std::sync::Arc::new(
                ArtifactOwnedToolJobContext::new(
                    meta.instance_id,
                    meta.view_state.clone(),
                    roots.canonical_base_revision,
                    roots.draft_generation,
                    roots.transient_generation,
                    ArtifactOwnedToolJobSnapshots {
                        children: std::sync::Arc::clone(&roots.children),
                        draft: std::sync::Arc::clone(&roots.draft_snapshot),
                        transient: std::sync::Arc::clone(&roots.transient),
                        window_config: roots.window_config_authority.as_ref().map(|authority| authority.snapshot.clone()),
                        window_transient: roots.window_transient_authority.as_ref().map(|authority| authority.snapshot.clone()),
                    },
                )
                .with_tool_run(self.tool_runs.view_for(meta.view_state.as_ref().and_then(|view| view.window_id.as_deref()))),
            );
            let key = proof.key();
            let spec = A::build_tool_job(ArtifactOwnedToolJobRequest {
                command,
                raw_wire: ArtifactToolRawInput::transferred_to_factory(),
                operation,
                controller_id: key.controller_id,
                tool_id: key.tool_id,
                payload_schema_id: proof.schema_id(),
                contract: proof.contract(),
                decoded_items: 1,
                app_instance_id: meta.instance_id,
                parent_document_id: self.store.envelope().id.clone(),
                canonical_base_revision: roots.canonical_base_revision,
                snapshot: roots.snapshot,
                config: roots.config,
                window_config: roots.window_config_authority.as_ref().map(|authority| authority.snapshot.clone()),
                history: roots.history,
                interaction_state: roots.interaction_state,
                interaction_hover: std::sync::Arc::new(roots.interaction_hover),
                context,
                instance_operation_owner: self.instance_operation_owner.clone(),
                output_chunks: ArtifactOutputChunks::new(proof.contract().max_output_bytes),
                completion: completion.clone(),
            })
            .await?
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-owned-builder"), format!("app-owned tool '{verb}' registered a factory but supplied no exact payload builder")))?;
            let payload = spec.payload.into_inner::<crate::retained_command::ArtifactRetainedCommandPayload<A>>().map_err(|payload| {
                Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.preview-unsupported"), format!("action '{verb}' runs a '{}' job the agent lane cannot preview; it runs only from the shell", payload.schema_id))
            })?;
            let mut job = crate::retained_command::ArtifactRetainedCommandJob::new(payload);
            let previewed = job.preview_emit();
            job.begin_close();
            while !job.terminal_is_empty() {
                if let semio_framework_job::InteractiveJobCloseStep::Blocked = job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.preview-close"), format!("the previewed job of action '{verb}' could not close")));
                }
            }
            drop(completion);
            previewed.map(|(emit, _)| emit)
        }

        /// 🧩️ A typed frame IS an owner-qualified invocation — the exact wire `AppCommand::Command`""",
)

replace("E", ROOT / '✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/⚖️declared-verbs/🦀️.rs', '//! ⚖️ Every verb the architect editor declares honours its declaration — the framework\'s declared-verb\n//! law (`artifact_app_laws::assert_declared_verbs_honour_their_declarations`) over this surface, on the\n//! sample program the editor boots with.\n\nuse super::*;\nuse semio_framework_plugin::artifact_app_laws::{assert_declared_verbs_honour_their_declarations, declared_verb_agent_divergences, declared_verb_ever_wrote_document, declared_verb_probe};\n\n#[semio_framework_async_macros::async_test]\nasync fn every_declared_architect_verb_honours_its_declaration() {\n    let probes = assert_declared_verbs_honour_their_declarations::<semio_framework_plugin::EditorApp<ArchitectPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(create_architect_app, None).await;\n    assert_eq!(probes.len(), 22, "the architect editor declares twenty-two verbs");\n    for verb in ["runAnalysis", "runReport"] {\n        assert!(declared_verb_ever_wrote_document(declared_verb_probe(&probes, verb)), "{verb} stores its record in the program");\n    }\n    assert_eq!(declared_verb_agent_divergences(&probes), ["setActiveExample"], "the agent lane previews no operation for a LoadDocument example switch");\n}\n', '//! ⚖️ Every verb the architect editor declares honours its declaration — the framework\'s declared-verb\n//! law (`artifact_app_laws::assert_declared_verbs_honour_their_declarations`) over this surface, on the\n//! sample program the editor boots with.\n\nuse super::*;\nuse semio_framework_plugin::artifact_app_laws::{assert_declared_verbs_honour_their_declarations, declared_verb_agent_divergences, declared_verb_ever_wrote_document, declared_verb_probe};\n\n/// ⚖️ LAW: all 22 declared verbs honour their declarations. The agent lane runs the same retained job the shell runs\n/// and refuses by name (`interactive-job.agent-lane-uncarried`) the four whose result an agent transaction cannot carry\n/// yet: the example switch (a host `LoadDocument`), the two exports (a file download) and the import picker (a host file\n/// request) — routed to the MCP gateway (ticket 26/09/23 `wp-p8.md` § routed). The pin turns red when a carrier lands.\n#[semio_framework_async_macros::async_test]\nasync fn every_declared_architect_verb_honours_its_declaration() {\n    let probes = assert_declared_verbs_honour_their_declarations::<semio_framework_plugin::EditorApp<ArchitectPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(create_architect_app, None).await;\n    assert_eq!(probes.len(), 22, "the architect editor declares twenty-two verbs");\n    for verb in ["runAnalysis", "runReport"] {\n        assert!(declared_verb_ever_wrote_document(declared_verb_probe(&probes, verb)), "{verb} stores its record in the program");\n    }\n    assert_eq!(declared_verb_agent_divergences(&probes), ["setActiveExample", "exportProgram", "exportRegistersCsv", "importProgramRequest"], "agent-lane divergences");\n}\n')

finish(__doc__)
