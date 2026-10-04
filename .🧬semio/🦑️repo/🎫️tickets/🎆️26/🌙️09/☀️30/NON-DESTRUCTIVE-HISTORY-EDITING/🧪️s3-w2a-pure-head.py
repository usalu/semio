"""🧪️ S3-W2A, decision §4 of `📓️api-stepped-document-load.md`: a pure command is head-only. Its document lane
hydrates the HEAD snapshot with no history: O(snapshot), no fold, no suspension. A lane that carries history is refused
`pure.history-unavailable`, and so is every history verb or history query in that mode. Applied as one pass over the
kernel notice table, its twin and fixture, the history-edit ledger, the runtime and the toy law."""

import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
K = ROOT / "🧰️framework/🔨️modules/🎠️kernel"
P = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
EN = "This is a head-only evaluation without history — open the document in a live instance to use its history."
DE = "Dies ist eine Auswertung nur des aktuellen Stands ohne Verlauf — für den Verlauf das Dokument in einer laufenden Instanz öffnen."
LOAD_EN = "The document is still loading — wait for it or cancel it first."
LOAD_DE = "Das Dokument wird noch geladen — abwarten oder zuerst abbrechen."

EDITS = {
    K / "🦀️.rs": [
        ("pub const HISTORY_NOTICE_LABELS: [(&str, &str, &str); 5] = [", "pub const HISTORY_NOTICE_LABELS: [(&str, &str, &str); 6] = ["),
        (f'    ("document.loading", "{LOAD_EN}", "{LOAD_DE}"),\n];', f'    ("document.loading", "{LOAD_EN}", "{LOAD_DE}"),\n    ("pure.history-unavailable", "{EN}", "{DE}"),\n];'),
    ],
    K / "🟦️.ts": [
        (f'  {{ code: "document.loading", en: "{LOAD_EN}", de: "{LOAD_DE}" }},\n', f'  {{ code: "document.loading", en: "{LOAD_EN}", de: "{LOAD_DE}" }},\n  {{ code: "pure.history-unavailable", en: "{EN}", de: "{DE}" }},\n'),
    ],
    K / "🧫️fixtures/🧫️history-notices/🔣️.json": [
        (f'    {{ "code": "document.loading", "en": "{LOAD_EN}", "de": "{LOAD_DE}" }}\n', f'    {{ "code": "document.loading", "en": "{LOAD_EN}", "de": "{LOAD_DE}" }},\n    {{ "code": "pure.history-unavailable", "en": "{EN}", "de": "{DE}" }}\n'),
        ("and any command while a whole-document load is live (gap N17).", "any command while a whole-document load is live (gap N17), and a history verb or query on a head-only pure evaluation."),
    ],
    P / "⏪️time-travel/🦀️.rs": [
        ("    authoring: Option<SupersedeAuthoring<A::Snapshot, A::Mutation>>,\n", "    authoring: Option<SupersedeAuthoring<A::Snapshot, A::Mutation>>,\n    history_unavailable: bool,\n"),
        ("            authoring: None,\n", "            authoring: None,\n            history_unavailable: false,\n"),
        ("""    /// ⏸️ Whether the user paused the replay of a waiting remote history change.
    pub fn reprojection_paused(&self) -> bool {""",
         """    /// 🫥️ Whether the document holds no history because a pure command hydrated its head alone
    /// (`📓️api-stepped-document-load.md` §4): history verbs and queries answer `pure.history-unavailable`.
    pub fn history_unavailable(&self) -> bool {
        self.history_unavailable
    }

    /// 🫥️ Marks the document as hydrated head-only (`true`) or loaded with its history (`false`).
    pub(crate) fn set_history_unavailable(&mut self, unavailable: bool) {
        self.history_unavailable = unavailable;
    }

    /// ⏸️ Whether the user paused the replay of a waiting remote history change.
    pub fn reprojection_paused(&self) -> bool {"""),
    ],
    P / "🦀️.rs": [
        ("""    /// ⏳️ The fault code every verb but a view verb answers while a whole-document load still runs (gap N17); its en/de
    /// notice is the kernel's `history_notice("document.loading")`.
    pub const DOCUMENT_LOADING_CODE: &str = "document.loading";""",
         """    /// ⏳️ The fault code every verb but a view verb answers while a whole-document load still runs (gap N17); its en/de
    /// notice is the kernel's `history_notice("document.loading")`.
    pub const DOCUMENT_LOADING_CODE: &str = "document.loading";

    /// 🫥️ The fault code of a history verb, a history query or a history-carrying document lane on a head-only pure
    /// evaluation (`📓️api-stepped-document-load.md` §4); its en/de notice is the kernel's
    /// `history_notice("pure.history-unavailable")`.
    pub const PURE_HISTORY_UNAVAILABLE_CODE: &str = "pure.history-unavailable";"""),
        ("""            if let Some(fault) = self.document_loading_refusal(action) {
                return Err(fault);
            }""",
         """            if let Some(fault) = self.document_loading_refusal(action) {
                return Err(fault);
            }
            if self.time_travel.history_unavailable() && (HISTORY_ACTION_IDS.contains(&action) || action == REVERT_TO_COMMAND_ACTION_ID || is_time_travel_action_id(action)) {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new(PURE_HISTORY_UNAVAILABLE_CODE), format!("'{action}' needs the document's history, which this head-only pure evaluation does not hold; use a live instance")));
            }"""),
        ("""        async fn history_snapshot(&mut self) -> Result<HistoryPatch, Fault> {
            self.history_patch(true).await
        }

        async fn hydrate_document_lane(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), Fault> {
            if pack.is_empty() && spr.is_empty() {
                return Ok(());
            }
            self.load_document_pack(&store::ArtifactPackFiles { pack: pack.to_vec(), spr: spr.to_vec(), ops: String::new() }).await
        }""",
         """        async fn history_snapshot(&mut self) -> Result<HistoryPatch, Fault> {
            if self.time_travel.history_unavailable() {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new(PURE_HISTORY_UNAVAILABLE_CODE), "this head-only pure evaluation holds no history; use a live instance"));
            }
            self.history_patch(true).await
        }

        /// 🫥️ A pure command's document lane is the HEAD snapshot without history (`📓️api-stepped-document-load.md` §4):
        /// it hydrates in O(snapshot) — no history fold, no suspension — and marks the document head-only, so history verbs
        /// and queries answer `pure.history-unavailable`; a lane carrying history is refused with that code. Empty lanes keep
        /// the live document.
        async fn hydrate_document_lane(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), Fault> {
            if pack.is_empty() && spr.is_empty() {
                return Ok(());
            }
            if !spr.is_empty() {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new(PURE_HISTORY_UNAVAILABLE_CODE), "a pure command carries the head snapshot without history; load a document with history through a live instance"));
            }
            let head = <A::Snapshot as ArtifactPack>::decode_pack(pack).map_err(|error| plugin_sdk_fault(format!("the pure head snapshot does not decode: {error:?}")))?;
            let id = self.store.envelope().id.clone();
            let envelope = store::create_document_envelope::<A::Snapshot, A::Mutation>(A::DOCUMENT_SCHEMA, &id, head, None);
            let window_reset = self.prepare_document_window_reset()?;
            self.store.reset(loaded_with_app_dialect::<A>(envelope)).await.map_err(|error| error.into_fault())?;
            self.commit_document_window_reset(window_reset);
            self.cache = None;
            self.retire_displaced_document_rows();
            self.time_travel.set_history_unavailable(true);
            Ok(())
        }"""),
        ("""            let parsed: store::ParsedDocumentText<A::Snapshot, A::Mutation> = store::parse_document_text(&files.dsl, &files.ops).await.map_err(|error| error.into_fault())?;
            let window_reset = self.prepare_document_window_reset()?;
            self.store.reset(loaded_with_app_dialect::<A>(parsed.into_envelope())).await.map_err(|error| error.into_fault())?;
            self.commit_document_window_reset(window_reset);
            self.cache = None;
            self.retire_displaced_document_rows();
            Ok(())""",
         """            let parsed: store::ParsedDocumentText<A::Snapshot, A::Mutation> = store::parse_document_text(&files.dsl, &files.ops).await.map_err(|error| error.into_fault())?;
            let window_reset = self.prepare_document_window_reset()?;
            self.store.reset(loaded_with_app_dialect::<A>(parsed.into_envelope())).await.map_err(|error| error.into_fault())?;
            self.commit_document_window_reset(window_reset);
            self.cache = None;
            self.retire_displaced_document_rows();
            self.time_travel.set_history_unavailable(false);
            Ok(())"""),
        ("""            let parsed: store::ParsedDocumentText<A::Snapshot, A::Mutation> = store::parse_document_pack(&files.pack, &files.spr).await.map_err(|error| error.into_fault())?;
            let window_reset = self.prepare_document_window_reset()?;
            self.store.reset(loaded_with_app_dialect::<A>(parsed.into_envelope())).await.map_err(|error| error.into_fault())?;
            self.commit_document_window_reset(window_reset);
            self.cache = None;
            self.retire_displaced_document_rows();
            Ok(())""",
         """            let parsed: store::ParsedDocumentText<A::Snapshot, A::Mutation> = store::parse_document_pack(&files.pack, &files.spr).await.map_err(|error| error.into_fault())?;
            let window_reset = self.prepare_document_window_reset()?;
            self.store.reset(loaded_with_app_dialect::<A>(parsed.into_envelope())).await.map_err(|error| error.into_fault())?;
            self.commit_document_window_reset(window_reset);
            self.cache = None;
            self.retire_displaced_document_rows();
            self.time_travel.set_history_unavailable(false);
            Ok(())"""),
    ],
    P / "🧪️tests/🧪️time-travel/🦀️.rs": [
        ("//#endregion 🧭️AcceptanceLaws", """/// ⚖️ LAW (decision §4 of `📓️api-stepped-document-load.md`): a pure command's document lane hydrates the HEAD snapshot of
/// a 240-edit document without its history — the store holds no edit, the head is the source head, nothing folds — while a
/// lane carrying the history refuses `pure.history-unavailable`; in that head-only mode undo, a history edit and the history
/// query refuse with the same code, and a load with history lifts it.
#[semio_framework_async_macros::async_test]
async fn a_pure_lane_hydrates_the_head_without_history_and_history_verbs_refuse() {
    let fixture = fixture();
    let actor = text(&fixture["actor"]).to_string();
    let mut source = seeded_app(&fixture).await;
    for value in 0..240 {
        source.store.dispatch(ArtifactCommand::Apply { mutations: vec![SetCount { value }.into()], description: None, transaction: None }).await.expect("a source edit");
    }
    let head = source.store.snapshot().expect("the source head");
    let files = source.document_pack().await.expect("the source pair");
    let mut pure = artifact_app_laws::new_registered_app::<ToyHistoryApp, _>(toy_manifest()).await;
    let code = |fault: Fault| fault.code.0;
    assert_eq!(pure.hydrate_document_lane(&files.pack, &files.spr).await.err().map(code).as_deref(), Some(PURE_HISTORY_UNAVAILABLE_CODE), "a pure lane never carries history");
    pure.hydrate_document_lane(&head.encode_pack(), &[]).await.expect("the head-only lane hydrates");
    assert_eq!(pure.store.snapshot().expect("the pure head"), head, "the pure document is the source head");
    assert!(pure.store.envelope().vcs.edits.is_empty() && pure.time_travel.history_unavailable(), "no history was folded or kept");
    assert_eq!(history_verb_refusal(&mut pure, &actor, "undo").await.as_deref(), Some(PURE_HISTORY_UNAVAILABLE_CODE));
    let meta = ActionMeta { view_state: Some(ViewModel::default()), ..artifact_app_laws::meta(&actor) };
    let begin = pure.handle_action("historyEditBegin", Some(&DslValue::object([("mutationId".to_string(), DslValue::String("any".into()))])), &meta).await;
    assert_eq!(begin.err().map(code).as_deref(), Some(PURE_HISTORY_UNAVAILABLE_CODE), "a history edit needs the history");
    assert_eq!(pure.history_snapshot().await.err().map(code).as_deref(), Some(PURE_HISTORY_UNAVAILABLE_CODE), "so does the history query");
    pure.load_document_pack(&files).await.expect("a load with history");
    assert!(!pure.time_travel.history_unavailable() && pure.history_snapshot().await.is_ok(), "a load with history lifts head-only mode");
    close(&mut pure);
    close(&mut source);
}

//#endregion 🧭️AcceptanceLaws"""),
    ],
}


def main():
    staged = {}
    for path, edits in EDITS.items():
        text = path.read_text(encoding="utf-8")
        for old, new in edits:
            if text.count(old) != 1:
                sys.exit(f"{path.name} ({path.parent.name}): anchor count {text.count(old)}: {old[:110]!r}")
            text = text.replace(old, new)
        staged[path] = text
    for path, text in staged.items():
        path.write_text(text, encoding="utf-8")
    print(f"pure head pass: {sum(len(edits) for edits in EDITS.values())} edits over {len(EDITS)} files")


if __name__ == "__main__":
    main()
