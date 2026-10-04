"""🧪️ S3-W2A gap N17, runtime half, applied as ONE pass over every file it touches (kernel wire + twin + schema + fixtures,
plugin runtime, laws) so the tree compiles before and after:
- `HistoryRemoteReplay`/`HistoryPatch.remoteReplay` become `HistoryReprojection`/`HistoryPatch.reprojection`, with `local`
  (a deferred LOCAL history step — interior undo/redo, checkout, alternative switch — next to the remote change).
- The history notice `history.replaying` (en/de) joins the kernel table, its twin and fixture.
- The runtime defers local replays like remote ones, drives both in one turn step, names a local step "Replaying history",
  cancels it by `discard_local_step` (zero trace) instead of pausing, shows a blocked local finalize with the blocking copy,
  and answers `historyEditBegin` busy while a local step waits.
Every anchor must match exactly once (or as stated); nothing is written unless every file's anchors match."""

import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
K = ROOT / "🧰️framework/🔨️modules/🎠️kernel"
P = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
PUZZLE_LAW = ROOT / "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️history-edit-runtime/🦀️.rs"

NOTICE_EN = "History is still replaying — wait for it or cancel it first."
NOTICE_DE = "Der Verlauf wird noch neu angewendet — abwarten oder zuerst abbrechen."

EDITS = {
    K / "🦀️.rs": [
        ("""/// 📡️ The replay a remote history change (another replica's supersession, or the undo or redo of one) needs before this
/// replica adopts it, stepped per reactor turn (design §16.6): replayed operations of the total, whether the user paused
/// it (`historyEditCancelReplay` while no session is open; `historyEditRerun` resumes) and the code of a refused adoption.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HistoryRemoteReplay {
    pub done: u32,
    pub total: u32,
    #[serde(default)]
    #[value(default)]
    pub paused: bool,""",
         """/// 📡️ The replay a history change needs before this replica adopts it, stepped per reactor turn (design §16.6, gap N17):
/// a remote change (another replica's supersession, or the undo or redo of one) or, `local`, this replica's own history step
/// (an interior undo or redo, a checkout, an alternative switch). Replayed operations of the total, whether the user paused
/// a remote one (`historyEditCancelReplay` while no session is open; `historyEditRerun` resumes — cancelling a local step
/// drops it instead) and the code of a refused adoption.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HistoryReprojection {
    pub done: u32,
    pub total: u32,
    #[serde(default)]
    #[value(default)]
    pub local: bool,
    #[serde(default)]
    #[value(default)]
    pub paused: bool,""", 1),
        ("""    /// 📡️ The remote history change waiting for its replay; absent while none waits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub remote_replay: Option<HistoryRemoteReplay>,""",
         """    /// 📡️ The history change (remote, or this replica's own step) waiting for its replay; absent while none waits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub reprojection: Option<HistoryReprojection>,""", 1),
        ("""pub const HISTORY_NOTICE_LABELS: [(&str, &str, &str); 3] = [""", """pub const HISTORY_NOTICE_LABELS: [(&str, &str, &str); 4] = [""", 1),
        ("""    ("history.full", "This document's history is full ({n} edits).", "Der Verlauf dieses Dokuments ist voll ({n} Bearbeitungen)."),
];""",
         f"""    ("history.full", "This document's history is full ({{n}} edits).", "Der Verlauf dieses Dokuments ist voll ({{n}} Bearbeitungen)."),
    ("history.replaying", "{NOTICE_EN}", "{NOTICE_DE}"),
];""", 1),
    ],
    K / "🟦️.ts": [
        ("""  { code: "history.full", en: "This document's history is full ({n} edits).", de: "Der Verlauf dieses Dokuments ist voll ({n} Bearbeitungen)." },
""",
         f"""  {{ code: "history.full", en: "This document's history is full ({{n}} edits).", de: "Der Verlauf dieses Dokuments ist voll ({{n}} Bearbeitungen)." }},
  {{ code: "history.replaying", en: "{NOTICE_EN}", de: "{NOTICE_DE}" }},
""", 1),
        ("""/** 📡️ The replay a remote history change needs before this replica adopts it, mirrored from Rust `HistoryRemoteReplay`:
 * replayed operations of the total, whether the user paused it, and the code of a refused adoption. */
export type HistoryRemoteReplay = {
  readonly done: number;
  readonly total: number;""",
         """/** 📡️ The replay a history change needs before this replica adopts it, mirrored from Rust `HistoryReprojection`: a remote
 * change or, `local`, this replica's own history step (interior undo/redo, checkout, alternative switch); replayed operations
 * of the total, whether the user paused a remote one, and the code of a refused adoption. */
export type HistoryReprojection = {
  readonly done: number;
  readonly total: number;
  readonly local?: boolean;""", 1),
        ("""  /** 📡️ The remote history change waiting for its replay; absent while none waits. */
  readonly remoteReplay?: HistoryRemoteReplay;""",
         """  /** 📡️ The history change (remote, or this replica's own step) waiting for its replay; absent while none waits. */
  readonly reprojection?: HistoryReprojection;""", 1),
    ],
    K / "🧬️schema/🔣️history-patch/🔣️.json": [
        ('''        "remoteReplay": { "$ref": "#/definitions/HistoryRemoteReplay" }''', '''        "reprojection": { "$ref": "#/definitions/HistoryReprojection" }''', 1),
        ('''    "HistoryRemoteReplay": {
      "type": "object",
      "additionalProperties": false,
      "required": ["done", "total"],
      "properties": {
        "done": { "$ref": "#/definitions/U32" },
        "total": { "$ref": "#/definitions/U32" },''',
         '''    "HistoryReprojection": {
      "type": "object",
      "additionalProperties": false,
      "required": ["done", "total"],
      "properties": {
        "done": { "$ref": "#/definitions/U32" },
        "total": { "$ref": "#/definitions/U32" },
        "local": { "type": "boolean" },''', 1),
    ],
    K / "🧫️fixtures/🧫️history-patch/🔣️.json": [
        ('''      "patch": { "cursor": 12, "commandFilter": "all", "remoteReplay": { "done": 40, "total": 200 } },
      "keys": []
    },''',
         '''      "patch": { "cursor": 12, "commandFilter": "all", "reprojection": { "done": 40, "total": 200 } },
      "keys": []
    },
    {
      "id": "a-local-history-step-replays-before-it-is-adopted",
      "patch": { "cursor": 14, "commandFilter": "all", "reprojection": { "done": 256, "total": 600, "local": true } },
      "keys": []
    },''', 1),
        ('''"remoteReplay": { "done": 0, "total": 200, "paused": true, "fault": "history.transition-refused" }''', '''"reprojection": { "done": 0, "total": 200, "paused": true, "fault": "history.transition-refused" }''', 1),
        ('''{ "id": "remote-replay-without-total", "patch": { "cursor": 1, "remoteReplay": { "done": 3 } }, "reason": "a remote replay names how many operations it replays" }''',
         '''{ "id": "reprojection-without-total", "patch": { "cursor": 1, "reprojection": { "done": 3 } }, "reason": "a reprojection names how many operations it replays" },
    { "id": "reprojection-local-not-a-boolean", "patch": { "cursor": 1, "reprojection": { "done": 0, "total": 3, "local": "yes" } }, "reason": "whether a reprojection is this replica's own step is a boolean" }''', 1),
    ],
    K / "🧫️fixtures/🧫️history-notices/🔣️.json": [
        ("an edit while a streamed tool transaction is open, a commit or abort naming no open transaction, and an exhausted edit history.",
         "an edit while a streamed tool transaction is open, a commit or abort naming no open transaction, an exhausted edit history, and a history step while another one still replays (gap N17).", 1),
        ('''    { "code": "history.full", "en": "This document's history is full ({n} edits).", "de": "Der Verlauf dieses Dokuments ist voll ({n} Bearbeitungen)." }
''',
         f'''    {{ "code": "history.full", "en": "This document's history is full ({{n}} edits).", "de": "Der Verlauf dieses Dokuments ist voll ({{n}} Bearbeitungen)." }},
    {{ "code": "history.replaying", "en": "{NOTICE_EN}", "de": "{NOTICE_DE}" }}
''', 1),
    ],
    P / "⏪️time-travel/🦀️.rs": [
        ("HistoryMutationMessage, HistoryRemoteReplay, HistoryTimeTravel,", "HistoryMutationMessage, HistoryReprojection, HistoryTimeTravel,", 1),
        ("""/// 📡️ Operations one reactor turn replays of a remote history change before the document store adopts it (design §16.6,
/// `ArtifactStore::defer_remote_replays`).
pub const TIME_TRAVEL_REMOTE_REPLAY_OPERATIONS: usize = 256;""",
         """/// 📡️ Operations one reactor turn replays of a history change before the document store adopts it — a remote change or
/// this replica's own interior undo/redo, checkout or alternative switch (design §16.6, gap N17;
/// `ArtifactStore::defer_remote_replays`, `ArtifactStore::defer_local_replays`).
pub const TIME_TRAVEL_REPLAY_OPERATIONS: usize = 256;""", 1),
        ("""    remote_paused: bool,
    remote_fault: Option<String>,
    remote_refreshed_ms: u64,""",
         """    reprojection_paused: bool,
    reprojection_fault: Option<(String, bool)>,
    reprojection_refreshed_ms: u64,""", 1),
        ("""            remote_paused: false,
            remote_fault: None,
            remote_refreshed_ms: 0,""",
         """            reprojection_paused: false,
            reprojection_fault: None,
            reprojection_refreshed_ms: 0,""", 1),
        ("""    /// ⏸️ Whether the user paused the replay of a waiting remote history change.
    pub fn remote_replay_paused(&self) -> bool {
        self.remote_paused
    }""",
         """    /// ⏸️ Whether the user paused the replay of a waiting remote history change.
    pub fn reprojection_paused(&self) -> bool {
        self.reprojection_paused
    }""", 1),
        ("""        self.remote_paused = false;
        self.remote_fault = None;
        self.authoring = None;""",
         """        self.reprojection_paused = false;
        self.reprojection_fault = None;
        self.authoring = None;""", 1),
        ("""        self.time_travel.has_pending_work() || (!self.time_travel.remote_paused && self.store.reprojection_progress().is_some()) || (self.time_travel.is_active() && self.time_travel_base() != Some(self.time_travel.session.base))
    }

    /// 📡️ The remote history change the document store replays before adopting it, as the history wire and body show it:
    /// its progress, whether the user paused it and the code of a refused adoption; `None` while none waits.
    pub(crate) fn remote_replay_status(&self) -> Option<HistoryRemoteReplay> {
        match (self.store.reprojection_progress(), self.time_travel.remote_fault.as_ref()) {
            (Some(progress), fault) => Some(HistoryRemoteReplay { done: progress.done, total: progress.total, paused: self.time_travel.remote_paused, fault: fault.cloned() }),
            (None, Some(fault)) => Some(HistoryRemoteReplay { done: 0, total: 0, paused: false, fault: Some(fault.clone()) }),
            (None, None) => None,
        }
    }

    /// ⏭️ One budget of the waiting remote history change (design §16.6): progress refreshes the history at most every
    /// [`TIME_TRAVEL_PROGRESS_REFRESH_MS`]; the adoption swaps the document in, delivers `BaseMoved` and refreshes every
    /// body; a refused adoption (the store drops the change) keeps its code on the remote row.
    async fn step_remote_replay(&mut self) -> Result<(), Fault> {
        let generation = self.store.generation();
        match self.store.step_reprojection().await {
            Ok(Some(_)) => {
                self.time_travel.remote_fault = None;
                let now_ms = semio_framework_job::default_now_ms().unwrap_or(0);
                if now_ms.saturating_sub(self.time_travel.remote_refreshed_ms) >= TIME_TRAVEL_PROGRESS_REFRESH_MS {
                    self.time_travel.remote_refreshed_ms = now_ms;
                    self.note_time_travel_changed(false, false);
                }
            }
            Ok(None) => {
                self.time_travel.remote_fault = None;
                if self.store.generation() != generation {
                    self.cache = None;
                    self.deliver_base_moved().await?;
                }
                self.note_time_travel_changed(true, true);
            }
            Err(error) => {
                self.time_travel.remote_fault = Some(error.into_fault().code.0);
                self.cache = None;
                self.note_time_travel_changed(true, true);
            }
        }
        Ok(())
    }

    /// ⏸️ `historyEditCancelReplay` while no session is open: drops the running replay of the waiting remote history change
    /// (the store keeps the change) and stops driving it until `historyEditRerun`.
    fn pause_remote_replay(&mut self) -> TimeTravelActionOutcome {
        if self.time_travel.remote_paused || self.store.reprojection_progress().is_none() {
            return TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal));
        }
        self.store.cancel_reprojection();
        self.time_travel.remote_paused = true;
        self.note_time_travel_changed(false, false);
        TimeTravelActionOutcome::Applied(TimeTravelStage::Inactive)
    }

    /// ▶️ `historyEditRerun` while no session is open: drives the paused remote replay again from its start.
    fn resume_remote_replay(&mut self) -> TimeTravelActionOutcome {
        if !self.time_travel.remote_paused {
            return TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal));
        }
        self.time_travel.remote_paused = false;
        self.note_time_travel_changed(false, false);
        TimeTravelActionOutcome::Applied(TimeTravelStage::Inactive)
    }""",
         """        self.time_travel.has_pending_work() || self.reprojection_drives() || (self.time_travel.is_active() && self.time_travel_base() != Some(self.time_travel.session.base))
    }

    /// 🚜️ Whether driver turns step the document store's waiting reprojection: a local history step always (it cannot be
    /// paused, only dropped), a remote change unless the user paused it.
    fn reprojection_drives(&self) -> bool {
        self.store.reprojection_progress().is_some() && (self.store.local_step_pending() || !self.time_travel.reprojection_paused)
    }

    /// 📡️ The history change the document store replays before adopting it, as the history wire and body show it: a remote
    /// change or this replica's own history step (`local`), its progress, whether the user paused a remote one and the code
    /// of a refused adoption; `None` while none waits.
    pub(crate) fn reprojection_status(&self) -> Option<HistoryReprojection> {
        let local = self.store.local_step_pending();
        match (self.store.reprojection_progress(), self.time_travel.reprojection_fault.as_ref()) {
            (Some(progress), fault) => Some(HistoryReprojection { done: progress.done, total: progress.total, local, paused: !local && self.time_travel.reprojection_paused, fault: fault.map(|(code, _)| code.clone()) }),
            (None, Some((code, local))) => Some(HistoryReprojection { done: 0, total: 0, local: *local, paused: false, fault: Some(code.clone()) }),
            (None, None) => None,
        }
    }

    /// ⏭️ One budget of the waiting reprojection (design §16.6, gap N17), a remote change or this replica's own history
    /// step: progress refreshes the history at most every [`TIME_TRAVEL_PROGRESS_REFRESH_MS`]; the adoption swaps the
    /// document in, delivers `BaseMoved` and refreshes every body; a refused adoption (the store drops the change) keeps its
    /// code on the row — a local finalize whose report blocks reads the blocking copy (`timeTravel.blocked`).
    async fn step_reprojection_turn(&mut self) -> Result<(), Fault> {
        let generation = self.store.generation();
        let local = self.store.local_step_pending();
        match self.store.step_reprojection().await {
            Ok(Some(_)) => {
                self.time_travel.reprojection_fault = None;
                let now_ms = semio_framework_job::default_now_ms().unwrap_or(0);
                if now_ms.saturating_sub(self.time_travel.reprojection_refreshed_ms) >= TIME_TRAVEL_PROGRESS_REFRESH_MS {
                    self.time_travel.reprojection_refreshed_ms = now_ms;
                    self.note_time_travel_changed(false, false);
                }
            }
            Ok(None) => {
                self.time_travel.reprojection_fault = None;
                if self.store.generation() != generation {
                    self.cache = None;
                    self.deliver_base_moved().await?;
                }
                self.note_time_travel_changed(true, true);
            }
            Err(error) => {
                let code = match (&error, local) {
                    (store::VcsError::Rejected { .. }, true) => TimeTravelRefusal::Blocked.code().to_string(),
                    _ => error.into_fault().code.0,
                };
                self.time_travel.reprojection_fault = Some((code, local));
                self.cache = None;
                self.note_time_travel_changed(true, true);
            }
        }
        Ok(())
    }

    /// ⏹️ `historyEditCancelReplay` while no session is open: a local history step that still replays is dropped with zero
    /// trace (`discard_local_step`); a waiting remote change's running replay is dropped (the store keeps the change) and
    /// driver turns leave it until `historyEditRerun`.
    fn cancel_reprojection_turns(&mut self) -> TimeTravelActionOutcome {
        if self.store.discard_local_step() {
            self.time_travel.reprojection_fault = None;
            self.note_time_travel_changed(true, true);
            return TimeTravelActionOutcome::Applied(TimeTravelStage::Inactive);
        }
        if self.time_travel.reprojection_paused || self.store.reprojection_progress().is_none() {
            return TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal));
        }
        self.store.cancel_reprojection();
        self.time_travel.reprojection_paused = true;
        self.note_time_travel_changed(false, false);
        TimeTravelActionOutcome::Applied(TimeTravelStage::Inactive)
    }

    /// ▶️ `historyEditRerun` while no session is open: drives the paused remote replay again from its start.
    fn resume_reprojection_turns(&mut self) -> TimeTravelActionOutcome {
        if !self.time_travel.reprojection_paused {
            return TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal));
        }
        self.time_travel.reprojection_paused = false;
        self.note_time_travel_changed(false, false);
        TimeTravelActionOutcome::Applied(TimeTravelStage::Inactive)
    }""", 1),
        ("""                HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID => return Ok(self.pause_remote_replay()),
                HISTORY_EDIT_RERUN_ACTION_ID => return Ok(self.resume_remote_replay()),""",
         """                HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID => return Ok(self.cancel_reprojection_turns()),
                HISTORY_EDIT_RERUN_ACTION_ID => return Ok(self.resume_reprojection_turns()),""", 1),
        ("""        if self.tool_runs.holds_mutating_run() || self.pending_transaction.is_some() || self.store.open_transaction().is_some() || self.time_travel.authoring.is_some() {""",
         """        if self.tool_runs.holds_mutating_run() || self.pending_transaction.is_some() || self.store.open_transaction().is_some() || self.time_travel.authoring.is_some() || self.store.local_step_pending() {""", 1),
        ("""        if !self.time_travel.remote_paused && self.store.reprojection_progress().is_some() {
            self.step_remote_replay().await?;
        }""",
         """        if self.reprojection_drives() {
            self.step_reprojection_turn().await?;
        }""", 1),
        ("""    RemoteReplay,
    RemoteReplayProgress,
    RemoteReplayPaused,
    RemoteReplayRefused,
}""",
         """    RemoteReplay,
    RemoteReplayProgress,
    RemoteReplayPaused,
    RemoteReplayRefused,
    LocalReplay,
    LocalReplayProgress,
    LocalReplayRefused,
}""", 1),
        ("""                Self::RemoteReplayRefused => "Remote history change refused: {code}",
            },""",
         """                Self::RemoteReplayRefused => "Remote history change refused: {reason}",
                Self::LocalReplay => "History step",
                Self::LocalReplayProgress => "Replaying history: {done} of {total} mutations",
                Self::LocalReplayRefused => "History step refused: {reason}",
            },""", 1),
        ("""                Self::RemoteReplayRefused => "Entfernte Verlaufsänderung abgelehnt: {code}",
            },""",
         """                Self::RemoteReplayRefused => "Entfernte Verlaufsänderung abgelehnt: {reason}",
                Self::LocalReplay => "Verlaufsschritt",
                Self::LocalReplayProgress => "Verlauf wird neu angewendet: {done} von {total} Mutationen",
                Self::LocalReplayRefused => "Verlaufsschritt abgelehnt: {reason}",
            },""", 1),
        ("""/// 📡️ The remote history change waiting for its replay as the history body's tree section `framework.history.remoteReplay`
/// (design §16.6): one status row — progress in words, paused, or the code of a refused adoption — then, when `controls`,
/// Cancel replay while it runs or Replay again while paused (no controls while a session is open, whose own Cancel replay
/// and Replay again address the session, nor for a viewer).
pub(crate) fn time_travel_remote_section(remote: &HistoryRemoteReplay, controls: bool, controller_id: &str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    let scope = "framework.history.remoteReplay";
    let (line, tone) = match (remote.fault.as_deref(), remote.paused) {
        (Some(code), _) if remote.total == 0 => (HistoryPanelText::RemoteReplayRefused.text(locale).replace("{code}", code), Tone::Danger),
        (_, true) => (HistoryPanelText::RemoteReplayPaused.text(locale).to_string(), Tone::Warning),
        _ => (HistoryPanelText::RemoteReplayProgress.text(locale).replace("{done}", &remote.done.to_string()).replace("{total}", &remote.total.to_string()), Tone::Info),
    };""",
         """/// 📡️ The history change waiting for its replay as the history body's tree section `framework.history.reprojection`
/// (design §16.6, gap N17) — a remote change, or this replica's own history step ("Replaying history"): one status row —
/// progress in words, paused, or the reason of a refused adoption (a local finalize whose report blocks reads the blocking
/// copy) — then, when `controls`, Cancel replay while it runs (a local step is dropped with zero trace) or Replay again
/// while a remote one is paused (no controls while a session is open, whose own Cancel replay and Replay again address the
/// session, nor for a viewer).
pub(crate) fn time_travel_reprojection_section(remote: &HistoryReprojection, controls: bool, controller_id: &str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    let scope = "framework.history.reprojection";
    let (title, progress, refused) = match remote.local {
        true => (HistoryPanelText::LocalReplay, HistoryPanelText::LocalReplayProgress, HistoryPanelText::LocalReplayRefused),
        false => (HistoryPanelText::RemoteReplay, HistoryPanelText::RemoteReplayProgress, HistoryPanelText::RemoteReplayRefused),
    };
    let reason = |code: &str| TimeTravelLabel::for_code(code).map_or_else(|| code.to_string(), |label| label.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string());
    let (line, tone) = match (remote.fault.as_deref(), remote.paused) {
        (Some(code), _) if remote.total == 0 => (refused.text(locale).replace("{reason}", &reason(code)), Tone::Danger),
        (_, true) => (HistoryPanelText::RemoteReplayPaused.text(locale).to_string(), Tone::Warning),
        _ => (progress.text(locale).replace("{done}", &remote.done.to_string()).replace("{total}", &remote.total.to_string()), Tone::Info),
    };""", 1),
        ("""    tree_section(ui_label(HistoryPanelText::RemoteReplay.text(locale), "time-travel-panel.remote-label")?)""",
         """    tree_section(ui_label(title.text(locale), "time-travel-panel.remote-label")?)""", 1),
    ],
    P / "🦀️.rs": [
        ("""            store.defer_remote_replays(Some(time_travel::TIME_TRAVEL_REMOTE_REPLAY_OPERATIONS));""",
         """            store.defer_remote_replays(Some(time_travel::TIME_TRAVEL_REPLAY_OPERATIONS));
            store.defer_local_replays(Some(time_travel::TIME_TRAVEL_REPLAY_OPERATIONS));""", 1),
        ("""                self.store.defer_remote_replays(Some(time_travel::TIME_TRAVEL_REMOTE_REPLAY_OPERATIONS));""",
         """                self.store.defer_remote_replays(Some(time_travel::TIME_TRAVEL_REPLAY_OPERATIONS));
                self.store.defer_local_replays(Some(time_travel::TIME_TRAVEL_REPLAY_OPERATIONS));""", 1),
        ("""remote_replay: Option<&semio_framework::kernel::HistoryRemoteReplay>, pages: &HistoryMutationPages,""",
         """reprojection: Option<&semio_framework::kernel::HistoryReprojection>, pages: &HistoryMutationPages,""", 1),
        ("""        let remote_section = match remote_replay {
            Some(remote) => Some(time_travel::time_travel_remote_section(remote, time_travel.is_none() && !read_only, controller_id, locale)?),""",
         """        let remote_section = match reprojection {
            Some(remote) => Some(time_travel::time_travel_reprojection_section(remote, time_travel.is_none() && !read_only, controller_id, locale)?),""", 1),
        ("""                remote_replay: self.remote_replay_status(),""", """                reprojection: self.reprojection_status(),""", 1),
        ("""                let remote_replay = self.remote_replay_status();""", """                let reprojection = self.reprojection_status();""", 1),
        ("""ui_history_panel(history, time_travel.as_ref(), remote_replay.as_ref(), &pages,""", """ui_history_panel(history, time_travel.as_ref(), reprojection.as_ref(), &pages,""", 1),
    ],
    P / "🧪️tests/🧪️time-travel/🦀️.rs": [
        ("""/// the history before it, its progress rides the history wire (`remoteReplay`) and the body's remote section; Cancel replay""",
         """/// the history before it, its progress rides the history wire (`reprojection`) and the body's reprojection section; Cancel replay""", 1),
        ("""    let status = remote.remote_replay_status().expect("the remote history change waits for its replay");""",
         """    let status = remote.reprojection_status().expect("the remote history change waits for its replay");""", 1),
        ("""    assert!(status.total > 0 && status.done < status.total && !status.paused && status.fault.is_none(), "{status:?}");
    assert_eq!(head(&remote).1, "a", "the replica shows the history before the change");
    assert_eq!(remote.history_patch(true).await.expect("history patch").remote_replay.as_ref().map(|remote| remote.total), Some(status.total), "the wire carries the remote replay");""",
         """    assert!(status.total > 0 && status.done < status.total && !status.paused && !status.local && status.fault.is_none(), "{status:?}");
    assert_eq!(head(&remote).1, "a", "the replica shows the history before the change");
    assert_eq!(remote.history_patch(true).await.expect("history patch").reprojection.as_ref().map(|remote| remote.total), Some(status.total), "the wire carries the remote replay");""", 1),
        ("""framework.history.remoteReplay.status""", """framework.history.reprojection.status""", 1),
        ("""    assert!(remote.time_travel.remote_replay_paused() && !remote.time_travel_has_pending_work(), "a paused remote replay is no driver work");""",
         """    assert!(remote.time_travel.reprojection_paused() && !remote.time_travel_has_pending_work(), "a paused remote replay is no driver work");""", 1),
        ("""framework.history.remoteReplay.rerun""", """framework.history.reprojection.rerun""", 1),
        ("""    assert!(remote.remote_replay_status().is_none(), "nothing waits any more");""", """    assert!(remote.reprojection_status().is_none(), "nothing waits any more");""", 1),
    ],
    PUZZLE_LAW: [
        ("""/// (`TIME_TRAVEL_REMOTE_REPLAY_OPERATIONS`) waits while the replica shows the history before it. Meanwhile:
/// - Its progress rides the history wire (`remoteReplay`).""",
         """/// (`TIME_TRAVEL_REPLAY_OPERATIONS`) waits while the replica shows the history before it. Meanwhile:
/// - Its progress rides the history wire (`reprojection`, not local).""", 1),
        ("""    let waiting = block_on(remote.history_snapshot()).expect("history").remote_replay.expect("the remote history change waits for its replay");
    assert!(waiting.total > 0 && waiting.done < waiting.total && !waiting.paused && waiting.fault.is_none(), "{waiting:?}");""",
         """    let waiting = block_on(remote.history_snapshot()).expect("history").reprojection.expect("the remote history change waits for its replay");
    assert!(waiting.total > 0 && waiting.done < waiting.total && !waiting.paused && !waiting.local && waiting.fault.is_none(), "{waiting:?}");""", 1),
        ("""remote.time_travel_ledger().remote_replay_paused()""", """remote.time_travel_ledger().reprojection_paused()""", 1),
        ("""expect("history").remote_replay.map(|replay| replay.paused), Some(true)""", """expect("history").reprojection.map(|replay| replay.paused), Some(true)""", 1),
        ("""block_on(app.history_snapshot()).expect("history").remote_replay.is_none()""", """block_on(app.history_snapshot()).expect("history").reprojection.is_none()""", 1),
    ],
}


def main():
    staged = {}
    for path, edits in EDITS.items():
        text = path.read_text(encoding="utf-8")
        for old, new, count in edits:
            if text.count(old) != count:
                sys.exit(f"{path.name} ({path.parent.name}): anchor count {text.count(old)} != {count}: {old[:110]!r}")
            text = text.replace(old, new)
        staged[path] = text
    for path, text in staged.items():
        path.write_text(text, encoding="utf-8")
    print(f"N17 runtime pass: {sum(len(edits) for edits in EDITS.values())} edits over {len(EDITS)} files")


if __name__ == "__main__":
    main()
