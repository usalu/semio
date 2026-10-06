#!/usr/bin/env python3
"""🩹️ S5-AGNOSTIC harness v9 (found by the first family on law v8, raster 01:17: "the edit scenario: the root's accepted draft is a
withdrawal, not its edited input"). The edit half of the conflict law resolved a blocked review by withdrawing whatever `nextProblem`
named — also when it named the ROOT: an input change that makes the edited mutation itself fail is no downstream conflict, and
withdrawing it replaced the edit under test. Now such a candidate is taken back (restore, zero trace) and the next one is drafted; a
review that can only be resolved by withdrawing the root is left through `historyEditExit` and the next candidate is tried.
Anchored on the exact post-v8 text, count-asserted, one write; idempotent. Usage: [--apply]"""
import sys

PATH = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️history-edit-acceptance/🦀️.rs"
OLD = """            let (classified, blocked) = acceptance_classified(&app, &ids)?;
            if !blocked {
                acceptance_verb(&mut app, HISTORY_EDIT_RESTORE_ACTION_ID, begin.clone()).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("an accepted draft of the root is not restored: {refusal}")))?;
                acceptance_pump(&mut app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await.map_err(AcceptanceVerdict::Fail)?;
                continue;
            }
            let resolved = acceptance_resolve(&mut app, &ids).await?;
"""
NEW = """            let (classified, blocked) = acceptance_classified(&app, &ids)?;
            let own = app.time_travel.status().and_then(|status| status.next_problem).is_some_and(|problem| problem.mutation_id == root);
            if !blocked || own {
                acceptance_verb(&mut app, HISTORY_EDIT_RESTORE_ACTION_ID, begin.clone()).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("an accepted draft of the root is not restored: {refusal}")))?;
                acceptance_pump(&mut app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await.map_err(AcceptanceVerdict::Fail)?;
                continue;
            }
            let resolved = acceptance_resolve(&mut app, &ids).await?;
            if resolved.contains(&root) {
                acceptance_verb(&mut app, HISTORY_EDIT_EXIT_ACTION_ID, Vec::new()).await.map_err(|refusal| AcceptanceVerdict::Fail(format!("a review that only withdrawing the edited root resolves does not exit: {refusal}")))?;
                acceptance_pump(&mut app, |app| app.time_travel.status().is_none() && !app.time_travel.has_pending_work()).await.map_err(AcceptanceVerdict::Fail)?;
                continue;
            }
"""
DOC_OLD = """/// fresh fold of the edited root followed by the mutations left. `Ok(None)`: no input change of the root conflicts downstream."""
DOC_NEW = """/// fresh fold of the edited root followed by the mutations left. An input change that makes the root itself fail, or that only
/// withdrawing the root resolves, is no downstream conflict: it is taken back and the next one drafted. `Ok(None)`: no input change of
/// the root conflicts downstream."""

with open(PATH, encoding="utf-8") as handle:
    before = handle.read()
if "a review that only withdrawing the edited root resolves does not exit" in before:
    sys.exit("SKIP: already carries harness v9")
if before.count(OLD) != 1 or before.count(DOC_OLD) != 1:
    sys.exit(f"ANCHOR: {before.count(OLD)} / {before.count(DOC_OLD)}")
after = before.replace(OLD, NEW).replace(DOC_OLD, DOC_NEW)
if "--apply" not in sys.argv:
    print(f"WOULD apply harness v9 (+{after.count(chr(10)) - before.count(chr(10))} lines)")
    sys.exit(0)
with open(PATH, encoding="utf-8") as handle:
    if handle.read() != before:
        sys.exit("RACE")
with open(PATH, "w", encoding="utf-8") as handle:
    handle.write(after)
print("WROTE")
