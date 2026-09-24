"""📌️ One-off: the native wgpu shell's hub Check In owner (request on a landed checkpoint, pump, footer status)."""
p = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
s = open(p, encoding="utf-8").read()
def rep(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:120], s.count(old))
    s = s.replace(old, new)

rep('''    pub checkin_dialog_draft: Option<String>,
    //#endregion 🔖️CheckIn''', '''    pub checkin_dialog_draft: Option<String>,
    /// 📌️ The one hub Check In this shell drives for its mounted hub document: requested the moment
    /// a checkpoint this shell asked for lands, submitted once the document's sync status names an
    /// acknowledged head, then polled to a terminal status (the React twin is the worker's
    /// `driveDocumentCheckIn`). Its status is the footer's hub badge suffix.
    #[cfg(not(target_arch = "wasm32"))]
    pub hub_check_in: Option<ShellHubCheckInV1>,
    //#endregion 🔖️CheckIn''')
rep('''            checkin_dialog_draft: None,
''', '''            checkin_dialog_draft: None,
            #[cfg(not(target_arch = "wasm32"))]
            hub_check_in: None,
''')

# landed branch: request the hub check-in (native)
rep('''        #[cfg(not(target_arch = "wasm32"))]
        {
            let Some(space_id) = self.open_space_id.clone() else { return };
            let Some(document_id) = self.sync_channel.as_ref().map(|channel| channel.document_id.clone()) else { return };
            if document_id != S_SPACE_INDEX_DOCUMENT_ID {
                self.touch_space_index_artifact(&space_id, &document_id).await;
            }
        }''', '''        #[cfg(not(target_arch = "wasm32"))]
        {
            let Some(space_id) = self.open_space_id.clone() else { return };
            let Some(document_id) = self.sync_channel.as_ref().map(|channel| channel.document_id.clone()) else { return };
            self.request_hub_check_in(&space_id, &document_id);
            if document_id != S_SPACE_INDEX_DOCUMENT_ID {
                self.touch_space_index_artifact(&space_id, &document_id).await;
            }
        }''')

owner = r'''
    /// 📌️ Arms one hub Check In of the mounted hub document; a running one for the same document is
    /// kept (its head is whatever the hub acknowledges once it is submitted).
    #[cfg(not(target_arch = "wasm32"))]
    fn request_hub_check_in(&mut self, space_id: &str, document_id: &str) {
        if self.hub_check_in.as_ref().is_some_and(|current| !current.status.as_ref().is_some_and(|status| status.phase.is_terminal()) && current.space_id == space_id && current.document_id == document_id) {
            return;
        }
        let now_ms = Self::directory_now_ms();
        self.hub_check_in = Some(ShellHubCheckInV1 {
            space_id: space_id.to_string(),
            document_id: document_id.to_string(),
            request_id: mint_directory_command_request_id(),
            head: None,
            quiescent_by_ms: now_ms.saturating_add(HUB_CHECK_IN_QUIESCENCE_MS),
            deadline_at_ms: now_ms.saturating_add(HUB_CHECK_IN_DEADLINE_MS),
            next_poll_at_ms: now_ms,
            submitted: false,
            cancel_requested: false,
            cancel_sent: false,
            status: None,
        });
    }

    /// 📌️ Drives the Check In by at most one bounded hub request per frame: waits (bounded) for the
    /// document's sync status to name an acknowledged head, submits it, then polls or cancels to a
    /// terminal status. Answers whether the footer's status changed.
    #[cfg(not(target_arch = "wasm32"))]
    async fn pump_hub_check_in(&mut self) -> bool {
        use semio_framework_os_kernel::os_directory::{DocumentCheckInPhaseV1, DocumentCheckInRefusalV1, DocumentCheckInV1, DOCUMENT_CHECK_IN_SCHEMA_V1};
        let Some(operation) = self.hub_check_in.clone() else { return false };
        if operation.status.as_ref().is_some_and(|status| status.phase.is_terminal()) {
            return false;
        }
        let now_ms = Self::directory_now_ms();
        let local_refusal = |refusal| Some(hub_check_in_local_status(&operation.request_id, DocumentCheckInPhaseV1::Failed, Some(refusal)));
        let Some(client) = self.directory_client.clone() else {
            self.hub_check_in.as_mut().expect("armed check-in").status = local_refusal(DocumentCheckInRefusalV1::AuthorityChanged);
            return true;
        };
        if now_ms >= operation.deadline_at_ms {
            self.hub_check_in.as_mut().expect("armed check-in").status = local_refusal(DocumentCheckInRefusalV1::Unavailable);
            return true;
        }
        let mounted = self.sync_channel.as_ref().is_some_and(|channel| channel.document_id == operation.document_id) && self.open_space_id.as_deref() == Some(operation.space_id.as_str());
        let Some(head) = operation.head.clone() else {
            if !mounted || now_ms >= operation.quiescent_by_ms {
                self.hub_check_in.as_mut().expect("armed check-in").status = local_refusal(DocumentCheckInRefusalV1::Unavailable);
                return true;
            }
            let Some(head) = self.sync_status.as_ref().and_then(|status| status.acknowledged_head.clone()).filter(|head| head.document_id == operation.document_id) else { return false };
            let current = self.hub_check_in.as_mut().expect("armed check-in");
            current.head = Some(head);
            current.status = Some(hub_check_in_local_status(&current.request_id, DocumentCheckInPhaseV1::Accepted, None));
            return true;
        };
        let cancel_due = operation.cancel_requested && !operation.cancel_sent;
        if operation.submitted && !cancel_due && now_ms < operation.next_poll_at_ms {
            return false;
        }
        let context = self.directory_command_ctx();
        let answer = if !operation.submitted {
            let request = DocumentCheckInV1 { schema: DOCUMENT_CHECK_IN_SCHEMA_V1.to_string(), request_id: operation.request_id.clone(), head };
            client.document_check_in(&context, &operation.space_id, &request).await
        } else if cancel_due {
            client.cancel_document_check_in(&context, &operation.space_id, &operation.document_id, &operation.request_id).await
        } else {
            client.document_check_in_status(&context, &operation.space_id, &operation.document_id, &operation.request_id).await
        };
        let Some(current) = self.hub_check_in.as_mut().filter(|current| current.request_id == operation.request_id) else { return false };
        current.next_poll_at_ms = Self::directory_now_ms().saturating_add(HUB_CHECK_IN_POLL_MS);
        current.submitted = true;
        current.cancel_sent |= cancel_due;
        match answer {
            Ok(status) => current.status = Some(status),
            Err(DirectoryClientError::Unauthorized) | Err(DirectoryClientError::Http { status: 403, .. }) => current.status = Some(hub_check_in_local_status(&current.request_id, DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::AuthorityChanged))),
            Err(DirectoryClientError::Http { status, .. }) if (400..500).contains(&status) => current.status = Some(hub_check_in_local_status(&current.request_id, DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::Unavailable))),
            Err(_) => {}
        }
        true
    }

    /// 📌️ The footer's hub badge, followed by the current Check In's status while one is shown.
    fn hub_footer_label(&self) -> String {
        let is_de = self.locale_id == "de";
        let hub = self.hub_connection_state().text(is_de);
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(status) = self.hub_check_in.as_ref().and_then(|operation| operation.status.as_ref()) {
            return format!("{hub} · {}", hub_check_in_status_text(status, is_de));
        }
        hub
    }
'''
rep('''    //#endregion 🔖️CheckIn

    /// @emoji 🔗️ The sync card's manual attach''', owner + '''    //#endregion 🔖️CheckIn

    /// @emoji 🔗️ The sync card's manual attach''')
rep('''        changed |= self.pump_hub_artifact_creation().await;
        let runner = self.directory_home.as_ref().and_then(|home| home.stream.clone());''', '''        changed |= self.pump_hub_artifact_creation().await;
        changed |= self.pump_hub_check_in().await;
        let runner = self.directory_home.as_ref().and_then(|home| home.stream.clone());''')
rep('''        let hub_label = hub.text(self.locale_id == "de");''', '''        let hub_label = self.hub_footer_label();''')
rep('''                let label = hub.text(self.locale_id == "de");
                let actionable = hub == ShellHubConnectionState::SignedOut;''', '''                let label = self.hub_footer_label();
                let actionable = hub == ShellHubConnectionState::SignedOut;''')

free = r'''
//#region 📌️HubCheckIn
/// ⏳️ How long a requested Check In waits for the document's edits to be acknowledged.
#[cfg(not(target_arch = "wasm32"))]
const HUB_CHECK_IN_QUIESCENCE_MS: u64 = 15_000;
/// ⏱️ The whole Check In, from request to a terminal status.
#[cfg(not(target_arch = "wasm32"))]
const HUB_CHECK_IN_DEADLINE_MS: u64 = 180_000;
#[cfg(not(target_arch = "wasm32"))]
const HUB_CHECK_IN_POLL_MS: u64 = 250;

/// 📌️ One native hub Check In owner (see `ShellState::hub_check_in`).
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug)]
pub struct ShellHubCheckInV1 {
    pub space_id: String,
    pub document_id: String,
    pub request_id: String,
    pub head: Option<semio_framework_os_kernel::os_directory::EditedArtifactFrontierV1>,
    pub quiescent_by_ms: u64,
    pub deadline_at_ms: u64,
    pub next_poll_at_ms: u64,
    pub submitted: bool,
    pub cancel_requested: bool,
    pub cancel_sent: bool,
    pub status: Option<semio_framework_os_kernel::os_directory::DocumentCheckInStatusV1>,
}

#[cfg(not(target_arch = "wasm32"))]
fn hub_check_in_local_status(request_id: &str, phase: semio_framework_os_kernel::os_directory::DocumentCheckInPhaseV1, refusal: Option<semio_framework_os_kernel::os_directory::DocumentCheckInRefusalV1>) -> semio_framework_os_kernel::os_directory::DocumentCheckInStatusV1 {
    semio_framework_os_kernel::os_directory::DocumentCheckInStatusV1 {
        schema: semio_framework_os_kernel::os_directory::DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1.to_string(),
        request_id: request_id.to_string(),
        phase,
        progress: semio_framework_os_kernel::os_directory::DocumentCheckInProgressV1 { completed_units: 0, total_units: 8 },
        ready: None,
        refusal,
    }
}

/// 📌️ One sentence for a hub Check In status (the React twin is `checkinStatusText`).
#[cfg(not(target_arch = "wasm32"))]
fn hub_check_in_status_text(status: &semio_framework_os_kernel::os_directory::DocumentCheckInStatusV1, is_de: bool) -> String {
    use semio_framework_os_kernel::os_directory::{DocumentCheckInPhaseV1 as Phase, DocumentCheckInRefusalV1 as Refusal};
    let key = match (status.phase, status.refusal) {
        (Phase::Ready, _) => "checkIn.ready",
        (Phase::Cancelled, _) => "checkIn.cancelled",
        (Phase::Failed, Some(Refusal::UnknownHead)) => "checkIn.unknownHead",
        (Phase::Failed, Some(Refusal::StaleHead)) => "checkIn.staleHead",
        (Phase::Failed, Some(Refusal::ActiveCheckpointChanged)) => "checkIn.activeCheckpointChanged",
        (Phase::Failed, Some(Refusal::LedgerNotReplayable)) => "checkIn.ledgerNotReplayable",
        (Phase::Failed, Some(Refusal::CodecRefused)) => "checkIn.codecRefused",
        (Phase::Failed, Some(Refusal::AuthorityChanged)) => "checkIn.authorityChanged",
        (Phase::Failed, _) => "checkIn.unavailable",
        (Phase::Accepted | Phase::Materializing | Phase::Publishing, _) => return format!("{} {}/{}", shell_chrome_string("checkIn.running", is_de), status.progress.completed_units, status.progress.total_units),
    };
    shell_chrome_string(key, is_de).to_string()
}
//#endregion 📌️HubCheckIn
'''
rep('''#[allow(clippy::too_many_arguments, reason = "one retained footer status item")]
fn render_footer_status_step(''', free.lstrip("\n") + '''
#[allow(clippy::too_many_arguments, reason = "one retained footer status item")]
fn render_footer_status_step(''')
rep('''        ("command.checkIn", false) => "Check In",
        ("command.checkIn", true) => "Einchecken",''', '''        ("command.checkIn", false) => "Check In",
        ("command.checkIn", true) => "Einchecken",
        ("checkIn.running", false) => "Checking in…",
        ("checkIn.running", true) => "Wird eingecheckt…",
        ("checkIn.ready", false) => "Checked in",
        ("checkIn.ready", true) => "Eingecheckt",
        ("checkIn.cancelled", false) => "Check-in cancelled",
        ("checkIn.cancelled", true) => "Einchecken abgebrochen",
        ("checkIn.unknownHead", false) => "Check-in refused: the hub does not know this version",
        ("checkIn.unknownHead", true) => "Einchecken abgelehnt: Der Hub kennt diesen Stand nicht",
        ("checkIn.staleHead", false) => "Already checked in: a newer check-in exists",
        ("checkIn.staleHead", true) => "Bereits eingecheckt: Es gibt einen neueren Check-in",
        ("checkIn.activeCheckpointChanged", false) => "Check-in collided with another check-in; try again",
        ("checkIn.activeCheckpointChanged", true) => "Einchecken kollidierte mit einem anderen Check-in; bitte erneut versuchen",
        ("checkIn.ledgerNotReplayable", false) => "Check-in refused: an approval in this range is checked in on its own",
        ("checkIn.ledgerNotReplayable", true) => "Einchecken abgelehnt: Eine Freigabe in diesem Bereich wird separat eingecheckt",
        ("checkIn.codecRefused", false) => "Check-in refused: the document could not be rebuilt",
        ("checkIn.codecRefused", true) => "Einchecken abgelehnt: Das Dokument konnte nicht wiederhergestellt werden",
        ("checkIn.authorityChanged", false) => "Check-in stopped: you can no longer edit this document",
        ("checkIn.authorityChanged", true) => "Einchecken gestoppt: Sie dürfen dieses Dokument nicht mehr bearbeiten",
        ("checkIn.unavailable", false) => "Check-in unavailable; try again",
        ("checkIn.unavailable", true) => "Einchecken nicht verfügbar; bitte erneut versuchen",''')
open(p, "w", encoding="utf-8").write(s)
print("ok")
