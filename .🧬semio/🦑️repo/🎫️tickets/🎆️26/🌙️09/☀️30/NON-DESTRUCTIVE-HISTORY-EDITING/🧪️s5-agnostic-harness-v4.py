#!/usr/bin/env python3
"""🩹️ S5-AGNOSTIC harness fix (found by the first law-v3 run, puzzle 5d 17:25: `create-part [option] … the session never settled; it
rests at None`). The reducer closes time travel with zero trace when an accepted draft REBUILDS THE SAME OPERATION
(`TimeTravelSession::accept` → `unchanged` → `resume`): drafting `/part/anchor = "fixed"` on a part that omits its anchor (the
default) changes the editor's value but not the operation. The harness took "the editor's value differs" for a change and then waited
60 s for a review. Now a candidate whose acceptance closes the session is counted as unchanged — it must leave no supersession of the
mutation — and the next candidate is drafted on a re-opened session; only when no candidate changes the operation is the case skipped.
Also: the pump's timeout names whether work is still pending, and the census line names the leaves that have cases but were not
exercised, each with its first skip reason. Anchored on the exact post-v3 text, count-asserted, one write; idempotent.
Usage: [--apply] [--preview <file>]"""
import collections, sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs"
MARKER = "left the operation unchanged"

EDITS = [
    ("""    Err(format!("the session never settled; it rests at {:?}", app.time_travel.status().map(|status| status.stage)))""",
     """    Err(format!("the session never settled within 60 s; it rests at {:?} (pending work: {})", app.time_travel.status().map(|status| status.stage), app.time_travel.has_pending_work()))"""),
    ("""    acceptance_verb(app, HISTORY_EDIT_BEGIN_ACTION_ID, begin).await.map_err(AcceptanceVerdict::Fail)?;
    let (inputs, original) = match app.time_travel.editor() {""",
     """    acceptance_verb(app, HISTORY_EDIT_BEGIN_ACTION_ID, begin.clone()).await.map_err(AcceptanceVerdict::Fail)?;
    let (inputs, original) = match app.time_travel.editor() {"""),
    ("""    let mut drafted = None;
    for candidate in changes {
        if acceptance_verb(app, HISTORY_EDIT_INPUT_ACTION_ID, vec![("path", DslValue::String(candidate.0.clone())), ("value", candidate.1.clone())]).await.is_ok()
            && app.time_travel.editor().is_some_and(|editor| editor.refused.is_none() && editor.value != original)
        {
            drafted = Some(candidate);
            break;
        }
    }
    let Some(drafted) = drafted else {
        let _ = acceptance_verb(app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new()).await;
        return Err(AcceptanceVerdict::Skip(format!("no schema-valid{} change of its {} input(s) is accepted", kind.map_or(String::new(), |kind| format!(" {}", ACCEPTANCE_CONTROL_KINDS[kind])), inputs.len())));
    };
    acceptance_verb(app, HISTORY_EDIT_ACCEPT_ACTION_ID, Vec::new()).await.map_err(AcceptanceVerdict::Fail)?;
    acceptance_pump(app, |app| app.time_travel.status().is_some_and(|status| status.stage != HistoryTimeTravelStage::Replaying)).await.map_err(AcceptanceVerdict::Fail)?;
    let status = app.time_travel.status().ok_or_else(|| AcceptanceVerdict::Fail("accepting closed the session".into()))?;""",
     """    let (mut drafted, mut unchanged) = (None, 0);
    for candidate in changes {
        if app.time_travel.status().is_none() {
            acceptance_verb(app, HISTORY_EDIT_BEGIN_ACTION_ID, begin.clone()).await.map_err(AcceptanceVerdict::Fail)?;
        }
        if acceptance_verb(app, HISTORY_EDIT_INPUT_ACTION_ID, vec![("path", DslValue::String(candidate.0.clone())), ("value", candidate.1.clone())]).await.is_err()
            || !app.time_travel.editor().is_some_and(|editor| editor.refused.is_none() && editor.value != original)
        {
            continue;
        }
        acceptance_verb(app, HISTORY_EDIT_ACCEPT_ACTION_ID, Vec::new()).await.map_err(AcceptanceVerdict::Fail)?;
        acceptance_pump(app, |app| app.time_travel.status().is_none_or(|status| status.stage != HistoryTimeTravelStage::Replaying)).await.map_err(AcceptanceVerdict::Fail)?;
        if app.time_travel.status().is_some() {
            drafted = Some(candidate);
            break;
        }
        if store.is_none() && app.store.supersessions().iter().any(|(id, _)| id.0 == target) {
            return Err(AcceptanceVerdict::Fail(format!("accepting {} = {:?}, which rebuilds the same operation, closed the session but left a supersession of the mutation", candidate.0, candidate.1)));
        }
        unchanged += 1;
    }
    let Some(drafted) = drafted else {
        if app.time_travel.status().is_some() {
            let _ = acceptance_verb(app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new()).await;
        }
        return Err(AcceptanceVerdict::Skip(format!(
            "no schema-valid{} change of its {} input(s) is accepted ({unchanged} left the operation unchanged)",
            kind.map_or(String::new(), |kind| format!(" {}", ACCEPTANCE_CONTROL_KINDS[kind])),
            inputs.len()
        )));
    };
    let status = app.time_travel.status().ok_or_else(|| AcceptanceVerdict::Fail("accepting closed the session".into()))?;"""),
    ("""/// one editor accepts (the editor's value differs)""", """/// one editor accepts (the editor's value differs)"""),
    ("""    let census = format!(
        "{plugin}: {} of {} editable leaves exercised ({} withdraw-only by declaration, {} without a committed editable case{}{}); control kinds: {}",
        exercised.len(),
        leaves.len(),
        <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS.len() - leaves.len(),
        uncased.len(),
        if uncased.is_empty() { "" } else { ": " },
        uncased.iter().take(12).copied().collect::<Vec<_>>().join(", "),
        kinds.join(", ")
    );""",
     """    let idle: Vec<String> = leaves
        .iter()
        .copied()
        .filter(|leaf| !exercised.contains(leaf) && !uncased.contains(leaf))
        .map(|leaf| {
            let reason = skipped.iter().chain(&broken).find(|entry| entry.starts_with(&format!("{leaf} "))).and_then(|entry| entry.rsplit_once("): ")).map_or("no reason recorded", |(_, reason)| reason);
            format!("{leaf} [{}]", reason.chars().take(110).collect::<String>())
        })
        .collect();
    let census = format!(
        "{plugin}: {} of {} editable leaves exercised ({} withdraw-only by declaration, {} without a committed editable case{}{}, {} with cases but not exercised{}{}); control kinds: {}",
        exercised.len(),
        leaves.len(),
        <A::Mutation as ::protocol::Mutation<A::Snapshot>>::DESCRIPTORS.len() - leaves.len(),
        uncased.len(),
        if uncased.is_empty() { "" } else { ": " },
        uncased.iter().take(12).copied().collect::<Vec<_>>().join(", "),
        idle.len(),
        if idle.is_empty() { "" } else { ": " },
        idle.iter().take(12).cloned().collect::<Vec<_>>().join("; "),
        kinds.join(", ")
    );"""),
]
EDITS = [edit for edit in EDITS if edit[0] != edit[1]]
DOC = ("""/// store): begin, draft `change` (else the first change of [`acceptance_changes`] the editor accepts), accept, replay to a clean""",
       """/// store): begin, draft `change` (else the first change of [`acceptance_changes`] the editor accepts AND that changes the operation:
/// accepting a draft that rebuilds the same operation closes time travel without a trace, by design, and the next one is tried on a
/// re-opened session), accept, replay to a clean""")


def main() -> None:
    with open(PATH, encoding="utf-8") as handle:
        before = handle.read()
    if MARKER in before:
        print("SKIP: already carries the unchanged-draft fix")
        return
    edits = EDITS + ([DOC] if before.count(DOC[0]) == 1 else [])
    for index, (old, _) in enumerate(EDITS):
        if before.count(old) != 1:
            sys.exit(f"ANCHOR #{index}: {before.count(old)} matches: {old[:90]!r}")
    after = before
    for old, new in edits:
        after = after.replace(old, new)
    if "--preview" in sys.argv:
        open(sys.argv[sys.argv.index("--preview") + 1], "w", encoding="utf-8").write(after)
    if "--apply" not in sys.argv:
        print(f"WOULD apply the unchanged-draft fix ({len(edits)} hunks, +{after.count(chr(10)) - before.count(chr(10))} lines)")
        return
    with open(PATH, encoding="utf-8") as handle:
        if handle.read() != before:
            sys.exit("RACE: the harness changed while staging")
    with open(PATH, "w", encoding="utf-8") as handle:
        handle.write(after)
    print("WROTE")


if __name__ == "__main__":
    main()
