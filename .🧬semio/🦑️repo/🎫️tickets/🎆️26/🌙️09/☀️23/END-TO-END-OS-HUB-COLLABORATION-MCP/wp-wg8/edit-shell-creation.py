import pathlib
p = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
t = p.read_text()
def swap(old, new):
    global t
    assert t.count(old) == 1, old[:100]
    t = t.replace(old, new)
swap('''use crate::hub_connection::{FRAMEWORK_HUB_PANEL_ID, HubDocumentRemote, HubSessionPresence};
''', '''use crate::hub_connection::{
    FRAMEWORK_HUB_PANEL_ID, HUB_ARTIFACT_CREATION_DEADLINE_MS, HUB_ARTIFACT_CREATION_POLL_MS, HubArtifactCatalogPhase, HubArtifactCreation, HubArtifactCreationState, HubArtifactOpening, HubDocumentRemote, HubSessionPresence, hub_artifact_creation_intent,
    hub_artifact_creation_terminal,
};
''')
swap('''            hub_action::OPEN_SPACE => {
                self.hub_workspace.open_space_id = (!space_id.is_empty()).then(|| space_id.clone());
                self.reload_hub_members(&space_id).await;
''', '''            hub_action::OPEN_SPACE => {
                self.hub_workspace.open_space_id = (!space_id.is_empty()).then(|| space_id.clone());
                self.reload_hub_members(&space_id).await;
                self.open_hub_artifact_creation(&space_id).await;
''')
swap('''            hub_action::CREATE_INVITE => self.run_hub_create_invite_turn(&space_id).await,
            hub_action::REDEEM_INVITE => self.run_hub_redeem_turn().await,
            _ => {}
        }
        let _ = self.refresh_ui(UiDirtyScope::Full).await;
    }
''', '''            hub_action::CREATE_INVITE => self.run_hub_create_invite_turn(&space_id).await,
            hub_action::REDEEM_INVITE => self.run_hub_redeem_turn().await,
            hub_action::SELECT_ARTIFACT_KIND => {
                let kind_id = args.as_ref().and_then(|args| args.get("kindId")).and_then(DslValue::as_str).unwrap_or_default();
                let creation = &mut self.hub_workspace.creation;
                if creation.catalog.as_ref().is_some_and(|catalog| catalog.kinds.iter().any(|kind| kind.kind_id == kind_id)) {
                    creation.kind_id = Some(kind_id.to_string());
                }
            }
            hub_action::SET_ARTIFACT_NAME => self.hub_workspace.creation.name_draft = value,
            hub_action::CREATE_ARTIFACT => {
                self.begin_hub_artifact_creation();
                self.pump_hub_artifact_creation().await;
            }
            hub_action::CANCEL_ARTIFACT_CREATION => {
                if let Some(operation) = self.hub_workspace.creation.operation.as_mut().filter(|operation| !hub_artifact_creation_terminal(operation.phase)) {
                    operation.cancel_requested = true;
                }
                self.pump_hub_artifact_creation().await;
            }
            hub_action::OPEN_CREATED_ARTIFACT => {
                self.open_created_hub_artifact().await;
            }
            _ => {}
        }
        let _ = self.refresh_ui(UiDirtyScope::Full).await;
    }

    /// 🌱️ Opens the creation door for `space_id`: a fresh door holding a minted idempotency key, then
    /// the space's selected current catalog. A hub with no ready catalog leaves the door
    /// `Unavailable`, saying so in both languages; local work never waits on it.
    async fn open_hub_artifact_creation(&mut self, space_id: &str) {
        self.hub_workspace.creation = HubArtifactCreationState { next_request_id: mint_directory_command_request_id(), ..HubArtifactCreationState::default() };
        let Some(client) = self.directory_client.clone().filter(|_| !space_id.is_empty()) else { return };
        self.hub_workspace.creation.catalog_phase = HubArtifactCatalogPhase::Loading;
        match client.space_artifact_creation_catalog(&self.directory_command_ctx(), space_id).await {
            Ok(catalog) => {
                self.hub_workspace.creation.catalog = Some(catalog);
                self.hub_workspace.creation.catalog_phase = HubArtifactCatalogPhase::Ready;
            }
            Err(_) => self.hub_workspace.creation.catalog_phase = HubArtifactCatalogPhase::Unavailable,
        }
    }

    /// 📥️ Seals the door's intent under its minted key and mints the next one at once, so no later
    /// click can resubmit this request id as a second creation. The frame pump submits it.
    fn begin_hub_artifact_creation(&mut self) {
        let Some(space_id) = self.hub_workspace.open_space_id.clone() else { return };
        let Some(intent) = hub_artifact_creation_intent(&self.hub_workspace) else { return };
        let now_ms = Self::directory_now_ms();
        let creation = &mut self.hub_workspace.creation;
        creation.next_request_id = mint_directory_command_request_id();
        creation.operation = Some(HubArtifactCreation {
            intent,
            space_id,
            phase: SpaceArtifactCreationPhaseV1::Accepted,
            submitted: false,
            cancel_requested: false,
            cancel_sent: false,
            ready: None,
            opening: HubArtifactOpening::Idle,
            deadline_at_ms: now_ms.saturating_add(HUB_ARTIFACT_CREATION_DEADLINE_MS),
            next_poll_at_ms: now_ms,
        });
    }

    /// 🌱️ Drives the one creation by at most one bounded hub request per frame — its submission, its
    /// cancellation, or a status poll no sooner than `HUB_ARTIFACT_CREATION_POLL_MS` after the last —
    /// and opens the artifact once the receipt is `Ready`. Past `HUB_ARTIFACT_CREATION_DEADLINE_MS` the
    /// outcome is `Indeterminate` (the hub may still finish it), never `Failed`. A `409` on the
    /// submission means the catalog generation moved: the door re-reads the catalog instead of
    /// retrying. Answers whether the door changed.
    async fn pump_hub_artifact_creation(&mut self) -> bool {
        let Some(operation) = self.hub_workspace.creation.operation.clone() else { return false };
        if hub_artifact_creation_terminal(operation.phase) {
            if operation.ready.is_some() && operation.opening == HubArtifactOpening::Idle {
                self.open_created_hub_artifact().await;
                return true;
            }
            return false;
        }
        let Some(client) = self.directory_client.clone() else { return false };
        let now_ms = Self::directory_now_ms();
        if now_ms >= operation.deadline_at_ms {
            if let Some(current) = self.hub_workspace.creation.operation.as_mut() {
                current.phase = SpaceArtifactCreationPhaseV1::Indeterminate;
            }
            return true;
        }
        let request_id = operation.intent.request_id.as_str();
        let context = self.directory_command_ctx();
        let (receipt, submission, cancellation) = if !operation.submitted {
            (client.create_space_artifact(&context, &operation.space_id, &operation.intent).await, true, false)
        } else if operation.cancel_requested && !operation.cancel_sent {
            (client.cancel_space_artifact_creation(&context, &operation.space_id, request_id).await, false, true)
        } else if now_ms >= operation.next_poll_at_ms {
            (client.space_artifact_creation_status(&context, &operation.space_id, request_id).await, false, false)
        } else {
            return false;
        };
        let mut refresh_catalog = false;
        {
            let Some(current) = self.hub_workspace.creation.operation.as_mut().filter(|current| current.intent.request_id == operation.intent.request_id) else { return false };
            current.submitted = true;
            current.cancel_sent |= cancellation;
            current.next_poll_at_ms = Self::directory_now_ms().saturating_add(HUB_ARTIFACT_CREATION_POLL_MS);
            match receipt {
                Ok(status) if status.catalog_generation_id == current.intent.expected_catalog_generation_id && status.ready.as_ref().is_none_or(|ready| ready.kind_id == current.intent.kind_id) => {
                    current.phase = status.phase;
                    current.ready = status.ready;
                }
                Ok(_) => current.phase = SpaceArtifactCreationPhaseV1::Failed,
                Err(DirectoryClientError::Http { status: 409, .. }) if submission => {
                    current.phase = SpaceArtifactCreationPhaseV1::Failed;
                    refresh_catalog = true;
                }
                Err(DirectoryClientError::Http { status, .. }) if (400..500).contains(&status) => current.phase = SpaceArtifactCreationPhaseV1::Failed,
                Err(DirectoryClientError::Unauthorized) => current.phase = SpaceArtifactCreationPhaseV1::Failed,
                Err(_) => {}
            }
        }
        if refresh_catalog {
            let space_id = operation.space_id.clone();
            let operation = self.hub_workspace.creation.operation.take();
            self.open_hub_artifact_creation(&space_id).await;
            self.hub_workspace.creation.operation = operation;
        }
        true
    }

    /// 🚪️ Opens a ready artifact through the ordinary document-open relay, bound to its space, with
    /// the dialect and schema the hub's receipt names; the relay resolves and mounts the owning app.
    /// The opening is `Opened` only when this shell's document binding names the created artifact.
    async fn open_created_hub_artifact(&mut self) {
        let Some((ready, space_id)) = self.hub_workspace.creation.operation.as_ref().and_then(|operation| operation.ready.clone().map(|ready| (ready, operation.space_id.clone()))) else { return };
        if let Some(operation) = self.hub_workspace.creation.operation.as_mut() {
            operation.opening = HubArtifactOpening::Opening;
        }
        let dialect = semio_framework::ArtifactDialect { artifact_kind: ready.parent_dialect.artifact_kind.clone(), standard: ready.parent_dialect.standard.clone(), subset: ready.parent_dialect.subset.clone() };
        let args = serde_json::json!({ "artifactRef": dialect.to_coordinate(), "documentId": ready.artifact_id, "schema": ready.artifact_schema, "spaceId": space_id });
        self.handle_open_artifact_relay("os.open-artifact", Some(&args)).await;
        let opened = self.sync_channel.as_ref().is_some_and(|channel| channel.document_id == ready.artifact_id);
        if let Some(operation) = self.hub_workspace.creation.operation.as_mut() {
            operation.opening = if opened { HubArtifactOpening::Opened } else { HubArtifactOpening::Failed };
        }
    }
''')
swap('''    pub async fn pump_directory_events(&mut self) -> bool {
        let identity_changed = self.poll_browser_identity().await;
        let administration_changed = self.pump_space_administration().await;
        self.flush_pending_directory_commands().await;
        // 🌉️ Packet W15e: the agent bridge's socket rides the SAME 100 ms slot rather than a timer of
        // its own — one pump cadence for every out-of-process conversation this shell holds.
        let bridge_changed = self.pump_agent_bridge();
        identity_changed || administration_changed || bridge_changed
    }''', '''    pub async fn pump_directory_events(&mut self) -> bool {
        let identity_changed = self.poll_browser_identity().await;
        let administration_changed = self.pump_space_administration().await;
        self.flush_pending_directory_commands().await;
        let creation_changed = self.pump_hub_artifact_creation().await;
        // 🌉️ Packet W15e: the agent bridge's socket rides the SAME 100 ms slot rather than a timer of
        // its own — one pump cadence for every out-of-process conversation this shell holds.
        let bridge_changed = self.pump_agent_bridge();
        identity_changed || administration_changed || creation_changed || bridge_changed
    }''')
swap('''        let mut changed = self.pump_space_administration().await || inference_changed || bridge_changed;''', '''        let mut changed = self.pump_space_administration().await || inference_changed || bridge_changed;
        changed |= self.pump_hub_artifact_creation().await;''')
p.write_text(t)
print("shell creation door wired")
