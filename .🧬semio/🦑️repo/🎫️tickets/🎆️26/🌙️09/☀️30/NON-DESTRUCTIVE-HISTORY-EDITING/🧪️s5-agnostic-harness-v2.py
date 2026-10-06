#!/usr/bin/env python3
"""🧬️ S5-AGNOSTIC law v2 (coordinator 09:30: "begin edit → draft input → preview without downstream → accept → replay report →
withdraw / restore → finalize overwrite + new alternative; `editable === false` → withdraw-only"). Adds to the cross-plugin harness
`🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs`, on top of P1 (leafless + child reload):
1. PREVIEW AS OF THE MUTATION — right after `historyEditBegin`, what every render seam reads (`TimeTravelLedger::render_snapshot_or`)
   must equal the document folded up to and including the edited mutation (a fresh instance seeded with that mutation alone when a
   downstream edit exists, else the committed head): downstream is not applied while editing.
2. WITHDRAW / RESTORE — before the edit session, the row action `historyEditWithdraw{mutationId}` opens the session on a withdrawn
   draft, `historyEditAccept` replays to a review (ready or blocked), `historyEditRestore{mutationId}` takes it back: the session
   closes, the head is the one before and the store holds no supersession of the mutation (zero trace). A withdrawal the store's
   supersede law does not admit (`timeTravel.not-withdrawable`) is stated in the summary instead.
3. VISIBLE-ROW RULE (design §22.20) — a committed operation whose payload schema shows no input row is not a case (the runtime refuses
   its editor by design; the inputs gate's `inputless` rule owns such a leaf).
4. WITHDRAW-ONLY BY DECLARATION (design §22.20/§22.23) — leaves whose descriptor says `"editable": false` are read from the artifact
   tree: the input-schema law does not judge them (and prints how many), and an artifact whose EVERY leaf is declared so answers
   "withdraw-only artifact" instead of failing for want of an editable case; any artifact with at least one editable leaf keeps the
   strict search.
Anchored on the exact post-P1 text, count-asserted, docstring emojis checked unique, one write, re-read before; idempotent.
Usage: [--apply]"""
import collections, sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs"
MARKER = "async fn acceptance_withdraw_restore"

EDITS = [
    # imports
    ("""    HISTORY_EDIT_FINALIZE_ACTION_ID, HISTORY_EDIT_INPUT_ACTION_ID,
};
""", """    HISTORY_EDIT_FINALIZE_ACTION_ID, HISTORY_EDIT_INPUT_ACTION_ID, HISTORY_EDIT_RESTORE_ACTION_ID, HISTORY_EDIT_WITHDRAW_ACTION_ID,
};
"""),
    # module doc
    ("""//! `historyEditBegin` → `historyEditInput` (a schema-valid change derived from the leaf's input descriptors) → `historyEditAccept`
//! → the Report replay → `historyEditFinalize` → `historyEditCommit` as an overwrite and, on a second instance, as a new
//! alternative; each head must equal a fresh fold of the edited log, and the edited mutation's history row must carry its leaf
//! label in every locale.""", """//! `historyEditWithdraw` → `historyEditAccept` → `historyEditRestore` (a withdrawal taken back leaves no trace), then
//! `historyEditBegin` (the preview is the document as of the edited mutation: downstream is not applied) → `historyEditInput` (a
//! schema-valid change derived from the leaf's input descriptors) → `historyEditAccept` → the Report replay →
//! `historyEditFinalize` → `historyEditCommit` as an overwrite and, on a second instance, as a new alternative; each head must
//! equal a fresh fold of the edited log, and the edited mutation's history row must carry its leaf label in every locale. A leaf
//! whose descriptor declares `"editable": false` is withdraw-only (design §22.20) and never a case."""),
    # visible-row rule
    ("""/// 🦠️ The operation a committed wire names, when it decodes as `A::Mutation` and the generic editor edits it (an input schema
/// and no foreign-step capability).
fn acceptance_operation<A: ArtifactApp>(wire: &DslValue) -> Option<A::Mutation> {
    let value = if acceptance_record(wire) { wire.get("mutation").cloned().unwrap_or(DslValue::Null) } else { wire.clone() };
    let op = <A::Mutation as semio_framework_value::FromValue>::from_value(value).ok()?;
    if ::protocol::Mutation::<A::Snapshot>::input_schema(&op).is_none() || ::protocol::Mutation::<A::Snapshot>::may_emit_foreign_steps(&op) {""",
     """/// 🦠️ The operation a committed wire names, when it decodes as `A::Mutation` and the generic editor edits it (an input schema
/// that shows at least one input row — design §22.20: an editor with zero rows never opens — and no foreign-step capability).
fn acceptance_operation<A: ArtifactApp>(wire: &DslValue) -> Option<A::Mutation> {
    let value = if acceptance_record(wire) { wire.get("mutation").cloned().unwrap_or(DslValue::Null) } else { wire.clone() };
    let op = <A::Mutation as semio_framework_value::FromValue>::from_value(value).ok()?;
    if ::protocol::Mutation::<A::Snapshot>::input_schema(&op).is_none_or(|schema| !time_travel::time_travel_schema_shows_inputs(schema)) || ::protocol::Mutation::<A::Snapshot>::may_emit_foreign_steps(&op) {"""),
    # drive: preview parameter and check
    ("""/// review, finalize, and commit as an overwrite or as the new alternative `alternative`. Answers the change drafted.
async fn acceptance_drive<A, M>(app: &mut VcsArtifactApp<A, M>, target: &str, store: Option<&str>, change: Option<&(String, DslValue)>, alternative: Option<&str>) -> Result<(String, DslValue), AcceptanceVerdict>""",
     """/// review, finalize, and commit as an overwrite or as the new alternative `alternative`. With `as_of` (the document folded up to
/// and including `target`), what every render seam reads right after the begin must be exactly that: the preview applies no
/// downstream mutation. Answers the change drafted.
async fn acceptance_drive<A, M>(
    app: &mut VcsArtifactApp<A, M>,
    target: &str,
    store: Option<&str>,
    change: Option<&(String, DslValue)>,
    alternative: Option<&str>,
    as_of: Option<&(DslValue, String)>,
) -> Result<(String, DslValue), AcceptanceVerdict>"""),
    ("""        Some(editor) => (editor.inputs.clone(), editor.value.clone()),
    };
    let changes = change.map_or_else(|| acceptance_changes(&inputs, &original), |change| vec![change.clone()]);""",
     """        Some(editor) => (editor.inputs.clone(), editor.value.clone()),
    };
    if let Some(as_of) = as_of {
        let committed = app.store.snapshot_owner();
        let shown = app.time_travel.render_snapshot_or(&app.tool_runs, &committed);
        let shown = (semio_framework_value::ToValue::to_value(shown.as_ref()), store::ArtifactDsl::print_dsl(shown.as_ref()));
        if shown != *as_of {
            let _ = acceptance_verb(app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new()).await;
            return Err(AcceptanceVerdict::Fail(format!("while editing, the preview is not the document as of the edited mutation (downstream not applied) at {}", acceptance_text_difference(&shown.1, &as_of.1))));
        }
    }
    let changes = change.map_or_else(|| acceptance_changes(&inputs, &original), |change| vec![change.clone()]);"""),
    # withdraw / restore + session parameter
    ("""/// ✏️ [`acceptance_drive`] over `app`'s first document mutation, then the store's own account of it: the superseding input decoded
/// as the edited operation, unscoped for an overwrite, scoped to the new active alternative otherwise. Answers the change drafted
/// and the edited operation the store now folds.
async fn acceptance_session<A, M>(app: &mut VcsArtifactApp<A, M>, change: Option<&(String, DslValue)>, alternative: Option<&str>) -> Result<((String, DslValue), A::Mutation), AcceptanceVerdict>""",
     """/// 🧯️ The withdraw → restore round trip over `app`'s first document mutation (design §22.1, §22.16): the row action
/// `historyEditWithdraw{mutationId}` opens the session on a withdrawn draft, accepting it replays to a review (ready, or blocked by
/// a downstream mutation that needed it), and `historyEditRestore{mutationId}` takes the withdrawal back — the session closes, the
/// head is the one before and the store holds no supersession of the mutation. `Ok(false)`: the store's supersede law admits no
/// withdrawal of this operation (`timeTravel.not-withdrawable`), nothing was changed.
async fn acceptance_withdraw_restore<A, M>(app: &mut VcsArtifactApp<A, M>) -> Result<bool, AcceptanceVerdict>
where
    A: ArtifactApp + Default,
    M: SpaceMember + MemberFactory + Send + 'static,
{
    let target = app
        .store
        .mutation_ops()
        .map_err(|error| AcceptanceVerdict::Fail(format!("the applied operations do not fold: {error:?}")))?
        .first()
        .map(|op| op.mutation_id.0.clone())
        .ok_or_else(|| AcceptanceVerdict::Fail("the seeded edit left no applied operation".into()))?;
    let before = acceptance_head(app).map_err(AcceptanceVerdict::Fail)?;
    let named = || vec![(HISTORY_EDIT_ARG_MUTATION_ID, DslValue::String(target.clone()))];
    match acceptance_verb(app, HISTORY_EDIT_WITHDRAW_ACTION_ID, named()).await {
        Ok(()) => {}
        Err(refusal) if refusal.ends_with(semio_framework_time_travel::TIME_TRAVEL_NOT_WITHDRAWABLE_CODE) => return Ok(false),
        Err(refusal) => return Err(AcceptanceVerdict::Fail(format!("the row's withdraw does not open: {refusal}"))),
    }
    acceptance_verb(app, HISTORY_EDIT_ACCEPT_ACTION_ID, Vec::new()).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("the withdrawn draft is not accepted: {refusal}")))?;
    acceptance_pump(app, |app| app.time_travel.status().is_some_and(|status| status.stage != HistoryTimeTravelStage::Replaying)).await.map_err(AcceptanceVerdict::Fail)?;
    let status = app.time_travel.status().ok_or_else(|| AcceptanceVerdict::Fail("accepting the withdrawal closed the session".into()))?;
    if status.stage != HistoryTimeTravelStage::Reviewing || !matches!(status.review, Some(HistoryTimeTravelReview::Ready | HistoryTimeTravelReview::Blocked)) {
        return Err(AcceptanceVerdict::Fail(format!("the replay of the withdrawal ends at {:?}/{:?}, not a review with a report", status.stage, status.review)));
    }
    acceptance_verb(app, HISTORY_EDIT_RESTORE_ACTION_ID, named()).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("the withdrawal is not restored: {refusal}")))?;
    acceptance_pump(app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await.map_err(AcceptanceVerdict::Fail)?;
    if app.store.supersessions().iter().any(|(id, _)| id.0 == target) {
        return Err(AcceptanceVerdict::Fail("restoring the only withdrawal left a supersession of the mutation in the store".into()));
    }
    let after = acceptance_head(app).map_err(AcceptanceVerdict::Fail)?;
    if after != before {
        return Err(AcceptanceVerdict::Fail(format!("after withdraw and restore the head differs from the one before at {}", acceptance_text_difference(&after.1, &before.1))));
    }
    Ok(true)
}

/// ✏️ [`acceptance_drive`] over `app`'s first document mutation (its preview checked against `as_of`), then the store's own account
/// of it: the superseding input decoded as the edited operation, unscoped for an overwrite, scoped to the new active alternative
/// otherwise. Answers the change drafted and the edited operation the store now folds.
async fn acceptance_session<A, M>(app: &mut VcsArtifactApp<A, M>, change: Option<&(String, DslValue)>, alternative: Option<&str>, as_of: &(DslValue, String)) -> Result<((String, DslValue), A::Mutation), AcceptanceVerdict>"""),
    ("""    let drafted = acceptance_drive(app, &target, None, change, alternative).await?;""",
     """    let drafted = acceptance_drive(app, &target, None, change, alternative, Some(as_of)).await?;"""),
    # scenario: as-of head, round trip, parameters
    ("""    let session = acceptance_session(&mut overwrite, None, None).await;
    let (change, edited) = match session {""",
     """    let as_of = match downstream {
        None => acceptance_head(&overwrite),
        Some(_) => match acceptance_seeded::<A, M>(manifest, &case.base, &[&case.op], true).await {
            Ok(mut alone) => {
                let head = acceptance_head(&alone);
                acceptance_close(&mut alone).await;
                head
            }
            Err(fault) => Err(format!("the edited mutation alone does not seed: {}", fault.reason())),
        },
    };
    let as_of = match as_of {
        Ok(head) => head,
        Err(reason) => {
            acceptance_close(&mut overwrite).await;
            return AcceptanceVerdict::Fail(reason);
        }
    };
    let (trail, session) = match acceptance_withdraw_restore(&mut overwrite).await {
        Ok(withdrawn) => (if withdrawn { "withdrawn and restored without a trace" } else { "withdrawal not admitted by the store's law" }, acceptance_session(&mut overwrite, None, None, &as_of).await),
        Err(verdict) => ("", Err(verdict)),
    };
    let (change, edited) = match session {"""),
    ("""                    Ok(mut branched) => {
                        let verdict = acceptance_alternative(&mut branched, manifest, &change, &fold, &label, downstream.is_some()).await;
                        acceptance_close(&mut branched).await;
                        verdict
                    }
                }
            }
            Ok(mut branched) => {
                let verdict = acceptance_alternative(&mut branched, manifest, &change, &fold, &label, downstream.is_some()).await;""",
     """                    Ok(mut branched) => {
                        let verdict = acceptance_alternative(&mut branched, manifest, &change, &fold, &label, downstream.is_some(), &as_of, trail).await;
                        acceptance_close(&mut branched).await;
                        verdict
                    }
                }
            }
            Ok(mut branched) => {
                let verdict = acceptance_alternative(&mut branched, manifest, &change, &fold, &label, downstream.is_some(), &as_of, trail).await;"""),
    ("""/// 🌳️ The new-alternative half of a scenario on `branched`: the change the overwrite accepted, committed as a new alternative, must
/// reach the same head as the fresh fold `fold`, live and after the document reloads through the app's own store initializer.
async fn acceptance_alternative<A, M>(branched: &mut VcsArtifactApp<A, M>, manifest: fn() -> App, change: &(String, DslValue), fold: &(DslValue, String), label: &str, downstream: bool) -> AcceptanceVerdict""",
     """/// 🌳️ The new-alternative half of a scenario on `branched`: the change the overwrite accepted, committed as a new alternative, must
/// reach the same head as the fresh fold `fold`, live and after the document reloads through the app's own store initializer; its
/// preview is checked against `as_of` like the overwrite's, and `trail` (the overwrite's withdraw round trip) joins the summary.
#[allow(clippy::too_many_arguments)]
async fn acceptance_alternative<A, M>(
    branched: &mut VcsArtifactApp<A, M>,
    manifest: fn() -> App,
    change: &(String, DslValue),
    fold: &(DslValue, String),
    label: &str,
    downstream: bool,
    as_of: &(DslValue, String),
    trail: &str,
) -> AcceptanceVerdict"""),
    ("""    match acceptance_session(branched, Some(change), Some(ACCEPTANCE_ALTERNATIVE)).await {""",
     """    match acceptance_session(branched, Some(change), Some(ACCEPTANCE_ALTERNATIVE), as_of).await {"""),
    ("""                (Ok(_), Ok(_)) => AcceptanceVerdict::Pass(format!("{} = {:?}, row \\"{label}\\"{}, both reloads fold alike", change.0, change.1, if downstream { ", one downstream edit replayed" } else { "" })),""",
     """                (Ok(_), Ok(_)) => AcceptanceVerdict::Pass(format!(
                    "{} = {:?}, row \\"{label}\\"{}, previewed as of the mutation, {trail}, both reloads fold alike",
                    change.0,
                    change.1,
                    if downstream { ", one downstream edit replayed" } else { "" }
                )),"""),
    # child lane: no preview check (the member's preview reaches the seams through the children view)
    ("""        let change = match acceptance_drive(&mut overwrite, &target, Some(&store), None, None).await {""",
     """        let change = match acceptance_drive(&mut overwrite, &target, Some(&store), None, None, None).await {"""),
    ("""        match acceptance_drive(&mut branched, &target, Some(&store), Some(&change), Some(ACCEPTANCE_ALTERNATIVE)).await {""",
     """        match acceptance_drive(&mut branched, &target, Some(&store), Some(&change), Some(ACCEPTANCE_ALTERNATIVE), None).await {"""),
    # withdraw-only by declaration
    ("""/// 🧑‍⚖️ LAW (design §16.3): a representative editable leaf of the app is edited in history end to end through the generic""",
     """/// 🚫️ The semantic kinds whose leaf descriptor under `root` (a `🔣️.json` naming `semanticKind` and `aggregateVariant`, fixtures and
/// tests aside) declares `"editable": false`: withdraw-only by declaration (design §22.20, §22.23).
pub fn acceptance_withdraw_only_kinds(root: &std::path::Path) -> BTreeSet<String> {
    fn walk(directory: &std::path::Path, found: &mut BTreeSet<String>) {
        let Ok(entries) = std::fs::read_dir(directory) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            if !path.is_dir() || name.starts_with('.') || ["target", "dist", "node_modules", "🗑️generated", "🧫️fixtures", "🧪️tests"].contains(&name.as_str()) {
                continue;
            }
            if let Some(descriptor) = acceptance_json(&path.join("🔣️.json")).filter(|descriptor| descriptor.get("aggregateVariant").is_some()) {
                if descriptor.get("editable").and_then(DslValue::as_bool) == Some(false) {
                    found.extend(descriptor.get("semanticKind").and_then(DslValue::as_str).map(str::to_string));
                }
                continue;
            }
            walk(&path, found);
        }
    }
    let mut kinds = BTreeSet::new();
    walk(root, &mut kinds);
    kinds
}

/// 🧑‍⚖️ LAW (design §16.3): a representative editable leaf of the app is edited in history end to end through the generic"""),
    ("""/// case's summary; an app whose aggregate has no leaf at all says so; panics naming `plugin`, the leaf and the case otherwise.""",
     """/// case's summary; an app whose aggregate has no leaf at all says so, and so does one whose every leaf is declared withdraw-only
/// (`"editable": false`, design §22.20); panics naming `plugin`, the leaf and the case otherwise."""),
    ("""    if <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS.is_empty() {
        return format!("{plugin}: no parent-lane leaf (a composed parent edits its content on the child lane)");
    }
""", """    let descriptors = <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS;
    if descriptors.is_empty() {
        return format!("{plugin}: no parent-lane leaf (a composed parent edits its content on the child lane)");
    }
    let withdraw_only = acceptance_withdraw_only_kinds(fixtures);
    if descriptors.iter().all(|descriptor| withdraw_only.contains(descriptor.semantic_kind)) {
        return format!("{plugin}: withdraw-only artifact (all {} leaves declare editable: false, design §22.20)", descriptors.len());
    }
"""),
    ("""/// (transitively) that nothing published, or a payload validator that does not compile. One line per breach, naming the leaf.
pub fn input_schema_resolution_failures<A: ArtifactApp>() -> Vec<String> {""",
     """/// (transitively) that nothing published, or a payload validator that does not compile. One line per breach, naming the leaf. The
/// kinds of `withdraw_only` (declared `"editable": false`: no editor ever opens on them, design §22.20) are not judged.
pub fn input_schema_resolution_failures<A: ArtifactApp>(withdraw_only: &BTreeSet<String>) -> Vec<String> {"""),
    ("""        let kind = descriptor.semantic_kind;
        if let Err(error) = semio_framework::mutation_input_defs(schema, &semio_framework::registered_input_schema_document) {""",
     """        let kind = descriptor.semantic_kind;
        if withdraw_only.contains(kind) {
            continue;
        }
        if let Err(error) = semio_framework::mutation_input_defs(schema, &semio_framework::registered_input_schema_document) {"""),
    ("""/// leaves reference, as in production — every document leaf's payload schema resolves completely. Answers the number of leaves
/// read; panics naming `plugin` and every breaching leaf otherwise.
pub async fn assert_input_schemas_resolve<A, M>(plugin: &str, manifest: fn() -> App) -> usize""",
     """/// leaves reference, as in production — every editable document leaf's payload schema resolves completely (a leaf under `fixtures`
/// declared `"editable": false` is withdraw-only and not judged). Answers the number of leaves read and the number declared
/// withdraw-only; panics naming `plugin` and every breaching leaf otherwise.
pub async fn assert_input_schemas_resolve<A, M>(plugin: &str, manifest: fn() -> App, fixtures: &std::path::Path) -> (usize, usize)"""),
    ("""    let failures = input_schema_resolution_failures::<A>();
    acceptance_close(&mut app).await;
    assert!(failures.is_empty(), "{plugin}: {} leaf payload schema(s) do not resolve through the runtime resolver:\\n{}", failures.len(), failures.join("\\n"));
    <A::Mutation as ::protocol::Mutation<A::Snapshot>>::INPUT_SCHEMAS.len()""",
     """    let declared = acceptance_withdraw_only_kinds(fixtures);
    let failures = input_schema_resolution_failures::<A>(&declared);
    acceptance_close(&mut app).await;
    assert!(failures.is_empty(), "{plugin}: {} leaf payload schema(s) do not resolve through the runtime resolver:\\n{}", failures.len(), failures.join("\\n"));
    let withdraw_only = <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS.iter().filter(|descriptor| declared.contains(descriptor.semantic_kind)).count();
    (<A::Mutation as ::protocol::Mutation<A::Snapshot>>::INPUT_SCHEMAS.len() - withdraw_only, withdraw_only)"""),
    ("""        fn history_edit_inputs_resolve() {
            let leaves = $crate::app::history_edit_acceptance::block_on_acceptance($crate::app::history_edit_acceptance::assert_input_schemas_resolve::<$crate::EditorApp<$editor>, <$editor as $crate::ArtifactEditor>::Members>($plugin, $manifest));
            ::std::println!("[history-edit-inputs] {}: {leaves} leaf payload schema(s) resolve", $plugin);""",
     """        fn history_edit_inputs_resolve() {
            let fixtures = ::std::path::Path::new(::core::env!("CARGO_MANIFEST_DIR")).join($fixtures);
            let (leaves, withdraw_only) =
                $crate::app::history_edit_acceptance::block_on_acceptance($crate::app::history_edit_acceptance::assert_input_schemas_resolve::<$crate::EditorApp<$editor>, <$editor as $crate::ArtifactEditor>::Members>($plugin, $manifest, &fixtures));
            ::std::println!("[history-edit-inputs] {}: {leaves} leaf payload schema(s) resolve, {withdraw_only} withdraw-only leaf(s) not judged", $plugin);"""),
]


def emojis(text):
    found, previous = [], False
    for line in text.splitlines():
        stripped = line.strip()
        doc = stripped.startswith("///") or stripped.startswith("//!")
        if doc and not previous:
            found.append(stripped[3:].strip().split(" ")[0])
        previous = doc
    return found


def main() -> None:
    with open(PATH, encoding="utf-8") as handle:
        before = handle.read()
    if MARKER in before:
        print("SKIP: already carries law v2")
        return
    if "DESCRIPTORS.is_empty()" not in before or "acceptance_child_reload_difference" not in before:
        sys.exit("ORDER: apply P1 (leafless + child reload) first")
    for index, (old, _) in enumerate(EDITS):
        if before.count(old) != 1:
            sys.exit(f"ANCHOR #{index}: {before.count(old)} matches: {old[:90]!r}")
    after = before
    for old, new in EDITS:
        after = after.replace(old, new)
    repeated = {token: count for token, count in collections.Counter(emojis(after)).items() if count > 1}
    if repeated:
        sys.exit(f"EMOJI: repeated docstring emojis {repeated}")
    if "--preview" in sys.argv:
        open(sys.argv[sys.argv.index("--preview") + 1], "w", encoding="utf-8").write(after)
    if "--apply" not in sys.argv:
        print(f"WOULD apply law v2 ({len(EDITS)} hunks, +{after.count(chr(10)) - before.count(chr(10))} lines)")
        return
    with open(PATH, encoding="utf-8") as handle:
        if handle.read() != before:
            sys.exit("RACE: the harness changed while staging")
    with open(PATH, "w", encoding="utf-8") as handle:
        handle.write(after)
    print("WROTE")


if __name__ == "__main__":
    main()
