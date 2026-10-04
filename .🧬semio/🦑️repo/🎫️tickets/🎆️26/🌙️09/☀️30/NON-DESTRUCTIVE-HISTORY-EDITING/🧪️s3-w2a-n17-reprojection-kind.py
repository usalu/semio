"""🧪️ S3-W2A gap N17: `HistoryReprojection.local: bool` becomes `kind: remote | step | load` (a remote change, this
replica's own deferred history step, a whole-document load), across the kernel wire, its twin, schema, fixture, the
runtime status/section and the laws — one pass, written only when every anchor matches."""

import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
K = ROOT / "🧰️framework/🔨️modules/🎠️kernel"
P = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
PZ = ROOT / "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️history-edit-runtime/🦀️.rs"

EDITS = {
    K / "🦀️.rs": [
        ("""/// 📡️ The replay a history change needs before this replica adopts it, stepped per reactor turn (design §16.6, gap N17):
/// a remote change (another replica's supersession, or the undo or redo of one) or, `local`, this replica's own history step
/// (an interior undo or redo, a checkout, an alternative switch). Replayed operations of the total, whether the user paused
/// a remote one (`historyEditCancelReplay` while no session is open; `historyEditRerun` resumes — cancelling a local step
/// drops it instead) and the code of a refused adoption.""",
         """/// 📡️ Which history change replays before this replica adopts it ([`HistoryReprojection`]): another replica's change
/// (`remote`), this replica's own deferred history step (`step`: an interior undo or redo, a checkout, an alternative
/// switch) or a whole-document load (`load`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum HistoryReprojectionKind {
    #[default]
    Remote,
    Step,
    Load,
}

/// 📡️ The replay a history change needs before this replica adopts it, stepped per reactor turn (design §16.6, gap N17),
/// by `kind`: replayed operations of the total, whether the user paused a remote one (`historyEditCancelReplay` while no
/// session is open; `historyEditRerun` resumes — cancelling a step or a load drops it with zero trace instead) and the code
/// of a refused adoption.""", 1),
        ("""    #[serde(default)]
    #[value(default)]
    pub local: bool,""", """    #[serde(default)]
    #[value(default)]
    pub kind: HistoryReprojectionKind,""", 1),
    ],
    K / "🟦️.ts": [
        ("""/** 📡️ The replay a history change needs before this replica adopts it, mirrored from Rust `HistoryReprojection`: a remote
 * change or, `local`, this replica's own history step (interior undo/redo, checkout, alternative switch); replayed operations
 * of the total, whether the user paused a remote one, and the code of a refused adoption. */
export type HistoryReprojection = {
  readonly done: number;
  readonly total: number;
  readonly local?: boolean;""",
         """/** 📡️ Which history change replays before adoption, mirrored from Rust `HistoryReprojectionKind`: another replica's change,
 * this replica's own deferred history step (interior undo/redo, checkout, alternative switch) or a whole-document load. */
export type HistoryReprojectionKind = "remote" | "step" | "load";

/** 📡️ The replay a history change needs before this replica adopts it, mirrored from Rust `HistoryReprojection`: its
 * `kind` (absent = remote), replayed operations of the total, whether the user paused a remote one, and the code of a
 * refused adoption. */
export type HistoryReprojection = {
  readonly done: number;
  readonly total: number;
  readonly kind?: HistoryReprojectionKind;""", 1),
    ],
    K / "🧬️schema/🔣️history-patch/🔣️.json": [
        ('''        "local": { "type": "boolean" },''', '''        "kind": { "enum": ["remote", "step", "load"] },''', 1),
    ],
    K / "🧫️fixtures/🧫️history-patch/🔣️.json": [
        ('''"reprojection": { "done": 256, "total": 600, "local": true } },
      "keys": []
    },''', '''"reprojection": { "done": 256, "total": 600, "kind": "step" } },
      "keys": []
    },
    {
      "id": "a-whole-document-load-replays-before-it-is-adopted",
      "patch": { "cursor": 15, "commandFilter": "all", "reprojection": { "done": 12, "total": 241, "kind": "load" } },
      "keys": []
    },''', 1),
        ('''{ "id": "reprojection-local-not-a-boolean", "patch": { "cursor": 1, "reprojection": { "done": 0, "total": 3, "local": "yes" } }, "reason": "whether a reprojection is this replica's own step is a boolean" }''',
         '''{ "id": "reprojection-kind-unknown", "patch": { "cursor": 1, "reprojection": { "done": 0, "total": 3, "kind": "local" } }, "reason": "a reprojection is a remote change, this replica's own step or a whole-document load" }''', 1),
    ],
    P / "⏪️time-travel/🦀️.rs": [
        ("HistoryMutationMessage, HistoryReprojection, HistoryTimeTravel,", "HistoryMutationMessage, HistoryReprojection, HistoryReprojectionKind, HistoryTimeTravel,", 1),
        ("""    reprojection_fault: Option<(String, bool)>,""", """    reprojection_fault: Option<(String, HistoryReprojectionKind)>,""", 1),
        ("""    /// 📡️ The history change the document store replays before adopting it, as the history wire and body show it: a remote
    /// change or this replica's own history step (`local`), its progress, whether the user paused a remote one and the code
    /// of a refused adoption; `None` while none waits.
    pub(crate) fn reprojection_status(&self) -> Option<HistoryReprojection> {
        let local = self.store.local_step_pending();
        match (self.store.reprojection_progress(), self.time_travel.reprojection_fault.as_ref()) {
            (Some(progress), fault) => Some(HistoryReprojection { done: progress.done, total: progress.total, local, paused: !local && self.time_travel.reprojection_paused, fault: fault.map(|(code, _)| code.clone()) }),
            (None, Some((code, local))) => Some(HistoryReprojection { done: 0, total: 0, local: *local, paused: false, fault: Some(code.clone()) }),
            (None, None) => None,
        }
    }""",
         """    /// 📡️ The history change this replica replays before adopting it, as the history wire and body show it: a whole-document
    /// load first (its archive operation's progress), else the document store's waiting reprojection — this replica's own
    /// history step or a remote change — with whether the user paused a remote one and the code of a refused adoption;
    /// `None` while nothing waits.
    pub(crate) fn reprojection_status(&self) -> Option<HistoryReprojection> {
        if let Some(load) = self.live_document_load().and_then(|operation| self.document_archive_loads.get(operation)).map(ActiveDocumentArchiveLoad::status) {
            return Some(HistoryReprojection { done: u32::try_from(load.completed).unwrap_or(u32::MAX), total: u32::try_from(load.total).unwrap_or(u32::MAX), kind: HistoryReprojectionKind::Load, paused: false, fault: None });
        }
        let kind = if self.store.local_step_pending() { HistoryReprojectionKind::Step } else { HistoryReprojectionKind::Remote };
        match (self.store.reprojection_progress(), self.time_travel.reprojection_fault.as_ref()) {
            (Some(progress), fault) => Some(HistoryReprojection { done: progress.done, total: progress.total, kind, paused: kind == HistoryReprojectionKind::Remote && self.time_travel.reprojection_paused, fault: fault.map(|(code, _)| code.clone()) }),
            (None, Some((code, kind))) => Some(HistoryReprojection { done: 0, total: 0, kind: *kind, paused: false, fault: Some(code.clone()) }),
            (None, None) => None,
        }
    }

    /// 🛬️ The whole-document load (document archive operation) still running on this instance, if any
    /// (`📓️api-stepped-document-load.md`).
    pub(crate) fn live_document_load(&self) -> Option<u64> {
        let mut live = None;
        self.document_archive_loads.each_id(|operation| {
            if live.is_none() && self.document_archive_loads.get(operation).is_some_and(|load| !load.terminal()) {
                live = Some(operation);
            }
        });
        live
    }""", 1),
        ("""                self.time_travel.reprojection_fault = Some((code, local));""",
         """                self.time_travel.reprojection_fault = Some((code, if local { HistoryReprojectionKind::Step } else { HistoryReprojectionKind::Remote }));""", 1),
        ("""    /// ⏹️ `historyEditCancelReplay` while no session is open: a local history step that still replays is dropped with zero
    /// trace (`discard_local_step`); a waiting remote change's running replay is dropped (the store keeps the change) and
    /// driver turns leave it until `historyEditRerun`.
    fn cancel_reprojection_turns(&mut self) -> TimeTravelActionOutcome {
        if self.store.discard_local_step() {""",
         """    /// ⏹️ `historyEditCancelReplay` while no session is open: a whole-document load still running is cancelled (the previous
    /// document stays, zero trace); a local history step that still replays is dropped with zero trace
    /// (`discard_local_step`); a waiting remote change's running replay is dropped (the store keeps the change) and driver
    /// turns leave it until `historyEditRerun`.
    fn cancel_reprojection_turns(&mut self) -> TimeTravelActionOutcome {
        if let Some(operation) = self.live_document_load() {
            return match PluginApp::cancel_document_archive_load(self, operation) {
                Ok(()) => {
                    self.note_time_travel_changed(true, true);
                    TimeTravelActionOutcome::Applied(TimeTravelStage::Inactive)
                }
                Err(_) => TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal)),
            };
        }
        if self.store.discard_local_step() {""", 1),
        ("""    LocalReplay,
    LocalReplayProgress,
    LocalReplayRefused,
}""", """    LocalReplay,
    LocalReplayProgress,
    LocalReplayRefused,
    DocumentLoad,
    DocumentLoadProgress,
}""", 1),
        ("""                Self::LocalReplayRefused => "History step refused: {reason}",
            },""", """                Self::LocalReplayRefused => "History step refused: {reason}",
                Self::DocumentLoad => "Document load",
                Self::DocumentLoadProgress => "Loading document: {done} of {total}",
            },""", 1),
        ("""                Self::LocalReplayRefused => "Verlaufsschritt abgelehnt: {reason}",
            },""", """                Self::LocalReplayRefused => "Verlaufsschritt abgelehnt: {reason}",
                Self::DocumentLoad => "Dokument laden",
                Self::DocumentLoadProgress => "Dokument wird geladen: {done} von {total}",
            },""", 1),
        ("""    let (title, progress, refused) = match remote.local {
        true => (HistoryPanelText::LocalReplay, HistoryPanelText::LocalReplayProgress, HistoryPanelText::LocalReplayRefused),
        false => (HistoryPanelText::RemoteReplay, HistoryPanelText::RemoteReplayProgress, HistoryPanelText::RemoteReplayRefused),
    };""",
         """    let (title, progress, refused) = match remote.kind {
        HistoryReprojectionKind::Load => (HistoryPanelText::DocumentLoad, HistoryPanelText::DocumentLoadProgress, HistoryPanelText::LocalReplayRefused),
        HistoryReprojectionKind::Step => (HistoryPanelText::LocalReplay, HistoryPanelText::LocalReplayProgress, HistoryPanelText::LocalReplayRefused),
        HistoryReprojectionKind::Remote => (HistoryPanelText::RemoteReplay, HistoryPanelText::RemoteReplayProgress, HistoryPanelText::RemoteReplayRefused),
    };""", 1),
    ],
    P / "🧪️tests/🧪️time-travel/🦀️.rs": [
        ("""!status.paused && !status.local && status.fault.is_none(), "{status:?}");
    assert_eq!(head(&remote).1, "a",""", """!status.paused && status.kind == semio_framework::kernel::HistoryReprojectionKind::Remote && status.fault.is_none(), "{status:?}");
    assert_eq!(head(&remote).1, "a",""", 1),
        ("""/// - the wire carries `reprojection.local`, the body reads""", """/// - the wire carries `reprojection.kind = step`, the body reads""", 1),
        ("""        assert!(status.local && status.total > 0 && !status.paused && status.fault.is_none(), "{status:?}");""",
         """        assert!(status.kind == semio_framework::kernel::HistoryReprojectionKind::Step && status.total > 0 && !status.paused && status.fault.is_none(), "{status:?}");""", 1),
    ],
    PZ: [
        ("""&& !waiting.paused && !waiting.local && waiting.fault.is_none(), "{waiting:?}");""", """&& !waiting.paused && waiting.kind == semio_framework::kernel::HistoryReprojectionKind::Remote && waiting.fault.is_none(), "{waiting:?}");""", 1),
        ("""/// - Its progress rides the history wire (`reprojection`, not local).""", """/// - Its progress rides the history wire (`reprojection`, kind `remote`).""", 1),
        ("""/// - The wire carries `reprojection.local`.""", """/// - The wire carries `reprojection.kind = step`.""", 1),
        ("""        assert!(waiting.local && waiting.total >= 360 && waiting.fault.is_none(), "{waiting:?}");""", """        assert!(waiting.kind == semio_framework::kernel::HistoryReprojectionKind::Step && waiting.total >= 360 && waiting.fault.is_none(), "{waiting:?}");""", 1),
        ("""reprojection.is_some_and(|replay| replay.local)""", """reprojection.is_some_and(|replay| replay.kind == semio_framework::kernel::HistoryReprojectionKind::Step)""", 1),
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
    print(f"reprojection kind pass: {sum(len(edits) for edits in EDITS.values())} edits over {len(EDITS)} files")


if __name__ == "__main__":
    main()
