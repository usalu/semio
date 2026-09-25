import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
text = path.read_text()


def replace(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:100])
    text = text.replace(old, new)


replace(
    """        if let Some((plugin_id, app)) = switch {
            if let Err(error) = self.switch_to_app(&plugin_id, app).await {
                Self::debug_log(&format!("[DEBUG] wgpu shell os.open-artifact could not switch to {plugin_id}: {error}"));
                return;
            }
        }
        let (Some(document_id), Some(schema)) = (target.document_id, target.schema) else {
            return;
        };
        let (bindings, surface) = self.default_bindings_for_current_session();
        if let Err(error) = self.open_document(document_id, schema, bindings, surface, None).await {
            Self::debug_log(&format!("[DEBUG] wgpu shell os.open-artifact relay failed: {error}"));
            let is_de = self.locale_id == "de";
            self.show_transient_notice(shell_chrome_string("open-artifact.document-failed", is_de), semio_framework::Severity::Warning, Some("open-artifact.document-failed"));
        }
    }
""",
    """        let document = match (target.document_id, target.schema) {
            (Some(document_id), Some(schema)) => Some(ShellDocumentOpenTarget { document_id, schema }),
            _ => None,
        };
        match switch {
            Some((plugin_id, app)) if !self.session.as_ref().is_some_and(|session| session.plugin_id == plugin_id && session.app.id == app.id) => {
                let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == plugin_id).cloned() else {
                    Self::debug_log(&format!("[DEBUG] wgpu shell os.open-artifact could not switch to {plugin_id}: program missing"));
                    return;
                };
                let label = app.label.resolve(self.active_terminology(), self.active_locale()).to_string();
                let app_id = app.id.clone();
                let pending = ShellDetached::spawn(async move { program.create_app(&app_id).await.map(ShellDocumentOpenAnswer::Instantiated) });
                self.document_opening = Some(ShellDocumentOpening { label, phase: ShellDocumentOpenPhase::Instantiating, cancel_requested: false, started_at_ms: chrome_now_ms(), plugin_id, app, document, pending: Some(pending), prepared: None });
            }
            _ => {
                if let Some(document) = document {
                    self.begin_document_seed(document).await;
                }
            }
        }
    }

    /// 🌱️ Starts the document half of an open in the mounted session: the host half is prepared on
    /// this turn, and the guest's genesis load runs detached; [`Self::advance_document_opening`] binds the
    /// document once it answers. A preparation that fails settles the open as failed.
    async fn begin_document_seed(&mut self, document: ShellDocumentOpenTarget) {
        let Some(session) = self.session.clone() else {
            self.fail_document_opening("document open has no mounted session".to_string());
            return;
        };
        let label = session.app.label.resolve(self.active_terminology(), self.active_locale()).to_string();
        let opening = self.document_opening.take();
        let (bindings, surface) = self.default_bindings_for_current_session();
        let prepared = match self.prepare_document_open(document.document_id, document.schema, bindings, surface, None).await {
            Ok(prepared) => prepared,
            Err(error) => {
                self.document_opening = opening;
                self.fail_document_opening(error);
                return;
            }
        };
        let pending = ShellDetached::spawn({
            let (plugin, instance_id, schema, document_id, hub_bound) = (prepared.plugin.clone(), prepared.session.instance_id, prepared.schema.clone(), prepared.document_id.clone(), prepared.hub_bound);
            async move { seed_document_genesis(plugin, instance_id, schema, document_id, hub_bound).await.map(|()| ShellDocumentOpenAnswer::Seeded) }
        });
        let mut opening = opening.unwrap_or_else(|| ShellDocumentOpening { label, phase: ShellDocumentOpenPhase::Seeding, cancel_requested: false, started_at_ms: chrome_now_ms(), plugin_id: session.plugin_id.clone(), app: session.app.clone(), document: None, pending: None, prepared: None });
        opening.phase = ShellDocumentOpenPhase::Seeding;
        opening.pending = Some(pending);
        opening.prepared = Some(prepared);
        self.document_opening = Some(opening);
    }

    /// ⏭️ Advances the retained open by the one detached step that answered, if any: an instance mounts
    /// as the session (or is destroyed when the open was cancelled), a seeded document binds, and every
    /// render the open owes goes to the settle lane. Called once per frame from [`Self::pump_sync_events`];
    /// answers whether the open moved.
    async fn advance_document_opening(&mut self) -> bool {
        let Some(answer) = self.document_opening.as_ref().and_then(|opening| opening.pending.as_ref()).and_then(ShellDetached::take) else { return false };
        let Some(mut opening) = self.document_opening.take() else { return false };
        opening.pending = None;
        match answer {
            Ok(ShellDocumentOpenAnswer::Instantiated(instance_id)) => {
                if opening.cancel_requested {
                    if let Some(program) = self.plugins.iter().find(|entry| entry.plugin_id == opening.plugin_id) {
                        program.destroy_app(instance_id);
                    }
                    opening.phase = ShellDocumentOpenPhase::Cancelled;
                    self.document_opening = Some(opening);
                    return true;
                }
                self.install_app_session(&opening.plugin_id, opening.app.clone(), instance_id);
                self.owe_refresh(UiDirtyScope::Full);
                self.owe_settle();
                match opening.document.take() {
                    Some(document) => {
                        self.document_opening = Some(opening);
                        self.begin_document_seed(document).await;
                    }
                    None => self.document_opening = None,
                }
            }
            Ok(ShellDocumentOpenAnswer::Seeded) => {
                let Some(prepared) = opening.prepared.take() else {
                    self.document_opening = Some(opening);
                    self.fail_document_opening("seeded document open lost its prepared owner".to_string());
                    return true;
                };
                if opening.cancel_requested {
                    opening.phase = ShellDocumentOpenPhase::Cancelled;
                    self.document_opening = Some(opening);
                    return true;
                }
                match self.finish_document_open(prepared).await {
                    Ok(()) => {
                        self.owe_refresh(UiDirtyScope::Full);
                        self.owe_settle();
                    }
                    Err(error) => {
                        self.document_opening = Some(opening);
                        self.fail_document_opening(error);
                        return true;
                    }
                }
            }
            Err(error) => {
                self.document_opening = Some(opening);
                self.fail_document_opening(error);
            }
        }
        true
    }

    /// 🧯️ Settles the open as failed, out loud: its band keeps the reason and the notice names it.
    fn fail_document_opening(&mut self, error: String) {
        Self::debug_log(&format!("[DEBUG] wgpu shell os.open-artifact relay failed: {error}"));
        let is_de = self.locale_id == "de";
        self.show_transient_notice(shell_chrome_string("open-artifact.document-failed", is_de), semio_framework::Severity::Warning, Some("open-artifact.document-failed"));
        if let Some(opening) = self.document_opening.as_mut() {
            opening.phase = ShellDocumentOpenPhase::Failed(error);
            opening.pending = None;
            opening.prepared = None;
        }
    }

    /// 🛑️ Cancels the open in flight, or clears a settled record. The phase moves only when the step in
    /// flight answers: an instance created after the cancel is destroyed, a seeded document never binds.
    pub fn cancel_document_opening(&mut self) {
        if self.document_opening.as_ref().is_some_and(|opening| !opening.running()) {
            self.document_opening = None;
            return;
        }
        if let Some(opening) = self.document_opening.as_mut() {
            opening.cancel_requested = true;
        }
    }
""",
)

# relay busy guard: refuse a second open while one runs
replace(
    """        if let Some(space_id) = target.space_id {
            self.open_space_id = Some(space_id);
        }
        // 🎬️ The relay used to open every artifact inside whatever session happened to be mounted, so""",
    """        if self.document_opening.as_ref().is_some_and(ShellDocumentOpening::running) {
            let is_de = self.locale_id == "de";
            self.show_transient_notice(shell_chrome_string("document.open.busy", is_de), semio_framework::Severity::Info, Some("document.open.busy"));
            return;
        }
        if let Some(space_id) = target.space_id {
            self.open_space_id = Some(space_id);
        }
        // 🎬️ The relay used to open every artifact inside whatever session happened to be mounted, so""",
)

# pump hook
replace(
    """        self.poll_auto_checkin().await;
        if let Some(error) = self.sync_terminal_fault.take() {""",
    """        self.poll_auto_checkin().await;
        let directory_changed = self.advance_document_opening().await || directory_changed;
        if let Some(error) = self.sync_terminal_fault.take() {""",
)

# door: resolve opening after the relay's open settles
replace(
    """        if hub_artifact_creation_terminal(operation.phase) {
            if operation.ready.is_some() && operation.opening == HubArtifactOpening::Idle {
                self.open_created_hub_artifact().await;
                return true;
            }
            return false;
        }""",
    """        if hub_artifact_creation_terminal(operation.phase) {
            if operation.ready.is_some() && operation.opening == HubArtifactOpening::Idle {
                self.open_created_hub_artifact().await;
                return true;
            }
            if operation.opening == HubArtifactOpening::Opening && !self.document_opening.as_ref().is_some_and(ShellDocumentOpening::running) {
                let opened = operation.ready.as_ref().is_some_and(|ready| self.sync_channel.as_ref().is_some_and(|channel| channel.document_id == ready.artifact_id));
                if let Some(current) = self.hub_workspace.creation.operation.as_mut() {
                    current.opening = if opened { HubArtifactOpening::Opened } else { HubArtifactOpening::Failed };
                }
                return true;
            }
            return false;
        }""",
)
replace(
    """        self.handle_open_artifact_relay("os.open-artifact", Some(&args)).await;
        let opened = self.sync_channel.as_ref().is_some_and(|channel| channel.document_id == ready.artifact_id);
        if let Some(operation) = self.hub_workspace.creation.operation.as_mut() {
            operation.opening = if opened { HubArtifactOpening::Opened } else { HubArtifactOpening::Failed };
        }
    }""",
    """        self.handle_open_artifact_relay("os.open-artifact", Some(&args)).await;
    }""",
)
replace(
    """    /// 🚪️ Opens a ready artifact through the ordinary document-open relay, bound to its space, with
    /// the dialect and schema the hub's receipt names; the relay resolves and mounts the owning app.
    /// The opening is `Opened` only when this shell's document binding names the created artifact.""",
    """    /// 🚪️ Opens a ready artifact through the ordinary document-open relay, bound to its space, with
    /// the dialect and schema the hub's receipt names; the relay resolves and mounts the owning app.
    /// The relay's open is frame-pumped, so the door stays `Opening` until it settles, and is `Opened`
    /// only when this shell's document binding then names the created artifact
    /// ([`Self::pump_hub_artifact_creation`]).""",
)
path.write_text(text)
print("ok")
