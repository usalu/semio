#!/usr/bin/env python3
"""🧫 H14 test-only (rule 22, coordinator 07:4x): kernel-db laws that depend on process-global state become deterministic under
the default runner. (1) `db_engine::vcs_integration::retained_tests`: every law that claims from the process-global
`VCS_ADMISSION` table (64 slots) — directly or through a graph's `record_change` — runs as the only law of its own process
(`process_isolated_law`, a no-op under nextest); the module-local `TEST_LOCK` could not exclude the engine laws that commit
through the default `vcs` graph at the same time (hold 8: the 70 016-commit law held a slot while the aggregate law claimed all 64
→ "vcs operation capacity exhausted", then its poisoned lock failed six more). (2) `artifact_history_empty_and_two_batch_replay_…`
and `full_submit_durable_query_round_trip_…` dropped their `Database` without `shutdown`: whichever thread dropped the last
`Arc<VersionGraphs>` dropped a live vcs `ArtifactStore`, and the store's Drop witness panicked when that was the test thread
(4/40 and 5/24 isolated runs under 8-way load) → both laws shut their database down. Idempotent; `--dry-run` reports."""
import re
import sys

R = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests"
VCS = f"{R}/🔬️vcs-integration-retained/🦀️.rs"
UNIT = f"{R}/🔬️unit/🦀️.rs"
DRY = "--dry-run" in sys.argv
files = {path: open(path, encoding="utf-8").read() for path in (VCS, UNIT)}
problems, states = [], []


def edit(path, old, new, label):
    text = files[path]
    if new in text and old not in text:
        states.append("done")
        return
    if text.count(old) != 1:
        problems.append(f"{label}: expected 1, found {text.count(old)}")
        states.append("problem")
        return
    files[path] = text.replace(old, new)
    states.append("replace")


edit(VCS, """mod retained_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEST_LOCK: Mutex<()> = Mutex::new(());
""", """/// 🧫️ Every law here that claims from the process-global `VCS_ADMISSION` table — directly or through a graph's
/// `record_change` — runs as the only law of its own process ([`crate::db_storage::process_isolated_law`]): the engine laws
/// commit through the default `vcs` graph at the same time, so a slot count, a slot identity or an exhausted table is only
/// exact when nothing else in the process holds a claim.
mod retained_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
""", "module isolation doc, lock removed")

ISOLATED = [
    "vcs_graph_rolls_full_windows_and_retires_them_in_bounded_steps",
    "vcs_store_keeps_exact_history_owners_through_changes_checkpoint_and_bounded_close",
    "vcs_shutdown_error_reinstalls_exact_store_and_retry_reaches_terminal",
    "vcs_retained_item_cap_plus_one_and_nested_bytes_plus_one_return_without_mutation",
    "vcs_checkpoint_derived_item_boundary_admits_31_rejects_32_and_preserves_exact_owners",
    "vcs_derived_owner_process_aggregate_plus_one_rejects_without_consuming_input",
    "vcs_retained_pending_wake_is_fifo_one_shot_and_quiet_without_release",
    "vcs_retained_cancel_clears_waiter_and_slot_aba_stays_stale",
]
for name in ISOLATED:
    guard = f"""        if !crate::db_storage::process_isolated_law("db_engine::vcs_integration::retained_tests::{name}") {{
            return;
        }}
"""
    match = re.search(rf"fn {name}\(\) \{{\n        let _guard = TEST_LOCK\.lock\(\)\.unwrap\(\);\n", files[VCS])
    if match:
        files[VCS] = files[VCS][: match.start()] + f"fn {name}() {{\n" + guard + files[VCS][match.end():]
        states.append("replace")
    elif f"fn {name}() {{\n{guard}" in files[VCS]:
        states.append("done")
    else:
        problems.append(f"isolate {name}: anchor not found")
        states.append("problem")

if "TEST_LOCK" in files[VCS]:
    problems.append("TEST_LOCK still referenced")

edit(UNIT, """    let _history_capacity = db_artifact::history_capacity_test_lock();
    let root = tempdir("history-order").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();""", """    let _history_capacity = db_artifact::history_capacity_test_lock();
    let root = tempdir("history-order").await;
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();""", "history mut database")

edit(UNIT, """    assert!(second.operation_id_eq(1, 0, "history-2"));
    while first.close_step() {}
    while second.close_step() {}
}""", """    assert!(second.operation_id_eq(1, 0, "history-2"));
    while first.close_step() {}
    while second.close_step() {}
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
}""", "history shutdown")

edit(UNIT, """    let _history_capacity = db_artifact::history_capacity_test_lock();
    let root = tempdir("round-trip").await;
    let database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();""", """    let _history_capacity = db_artifact::history_capacity_test_lock();
    let root = tempdir("round-trip").await;
    let mut database = Database::open_at(test_worker_pool(), &root, Profile::Test).await.unwrap();""", "round trip mut database")

edit(UNIT, """    let mut history = handle.history().await.unwrap();
    assert_eq!(history.entries().len(), 1);
    assert!(history.operation_id_eq(0, 0, "op-1"));
    while history.close_step() {}
}""", """    let mut history = handle.history().await.unwrap();
    assert_eq!(history.entries().len(), 1);
    assert!(history.operation_id_eq(0, 0, "op-1"));
    while history.close_step() {}
    drop(handle);
    database.shutdown(&DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(30))).await.unwrap();
}""", "round trip shutdown")

print(f"states {states}")
if problems:
    print("PROBLEMS:\n  " + "\n  ".join(problems))
    sys.exit(1)
if DRY:
    print("dry-run clean")
elif "replace" in states:
    for path, text in files.items():
        open(path, "w", encoding="utf-8").write(text)
    print("applied")
else:
    print("nothing to apply")
