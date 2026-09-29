#!/usr/bin/env python3
"""🔇️ R10 session 15 (guest set, window 5): the SDK's `[TRACE] typed-operation slots …` line prints only when runtime
diagnostics are armed (`SEMIO_RUNTIME_DIAGNOSTICS`, `semio_framework_trace::runtime_diagnostics_enabled`), like every other
per-turn runtime trace. Before: every new occupancy peak and its first full drain printed unconditionally, so every page
printed two lines per guest (C12 row 28, audit-s15 §2 UNOWNED). The peak keeps being tracked (one atomic `fetch_max`), so a
run that arms diagnostics mid-session still reports the true peak; the drain-witness flag goes (armed runs print every
transition anyway). The slot-release laws of the SDK (`typed-operation slots were never released`, `left … typed-operation
slots live`) keep guarding the leak on every test run. The program matrix's `NOISE` alternative for the line is dead
(the line never matched `FAULT`) and goes with it.

usage: python3 trace-slots-default-off.py --dry-run | --write | --revert [--root <tree>]
Backups (byte-exact) under `.🧬semio/🌐hub/s14-r10-state/trace-slots-backup/<root-hash>/`.
"""
import hashlib
import os
import sys

TREE = sys.argv[sys.argv.index("--root") + 1] if "--root" in sys.argv else "/Users/ueli/Documents/semio"
BACKUP = f"/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-r10-state/trace-slots-backup/{hashlib.sha256(TREE.encode()).hexdigest()[:12]}"
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
MATRIX = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts"

SDK_OLD = """    /// 🎰️ Highest typed-operation slot occupancy any instance of this guest has reached, and whether that
    /// peak still owes its drain witness — the two values that bound the diagnostic below to at most two
    /// console lines per distinct peak, i.e. at most `2 × ARTIFACT_LIVE_OUTPUT_SLOTS` for a whole session.
    static PEAK_TYPED_OPERATION_SLOTS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    static TYPED_OPERATION_SLOT_DRAIN_PENDING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

    /// 🎰️ Reports the guest's live typed-operation slot occupancy on the console channel the rest of this
    /// runtime's `[TRACE]` diagnostics already use — no wire, no protocol frame, and nothing the host may
    /// branch on. A browser probe reads `live=0` as the proof every admitted slot reached its release path;
    /// a `peak` that rises and never drains is the retirement leak that refused
    /// `engagementAbort`/`setCamera`/`openVortexSuggestions` 39 times in the 2026-09-11 puzzle3d battery
    /// (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B8). Every new peak and its first full drain print
    /// unconditionally because that pair IS the leak signal and is self-limiting; every intermediate
    /// transition prints only under [`semio_framework_trace::runtime_diagnostics_enabled`].
    fn trace_typed_operation_slot_occupancy<PA: PluginApp>(app: &PA, instance: u32) {
        let live = app.live_typed_operation_slots() as u64;
        let previous = PEAK_TYPED_OPERATION_SLOTS.fetch_max(live, Ordering::Relaxed);
        let peak = previous.max(live);
        let raised = live > previous;
        if raised {
            TYPED_OPERATION_SLOT_DRAIN_PENDING.store(true, Ordering::Relaxed);
        }
        let drained = live == 0 && TYPED_OPERATION_SLOT_DRAIN_PENDING.swap(false, Ordering::Relaxed);
        if !raised && !drained && !semio_framework_trace::runtime_diagnostics_enabled() {
            return;
        }
        eprintln!("[TRACE] typed-operation slots instance={instance} live={live}/{} peak={peak}", crate::app::ARTIFACT_LIVE_OUTPUT_SLOTS);
    }
"""
SDK_NEW = """    /// 🎰️ Highest typed-operation slot occupancy any instance of this guest has reached — tracked on every
    /// turn, so a run that arms runtime diagnostics mid-session still reports the true peak.
    static PEAK_TYPED_OPERATION_SLOTS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    /// 🎰️ Reports the guest's live typed-operation slot occupancy on the console channel the rest of this
    /// runtime's `[TRACE]` diagnostics use — no wire, no protocol frame, and nothing the host may branch on —
    /// only under [`semio_framework_trace::runtime_diagnostics_enabled`], like every other per-turn runtime
    /// trace: an interactive page never prints it. A leak run arms `SEMIO_RUNTIME_DIAGNOSTICS` and reads `live=0`
    /// as the proof every admitted slot reached its release path; a `peak` that rises and never drains is the
    /// retirement leak that refused `engagementAbort`/`setCamera`/`openVortexSuggestions` 39 times in the
    /// 2026-09-11 puzzle3d battery (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B8), which the slot-release laws
    /// of this crate guard on every test run.
    fn trace_typed_operation_slot_occupancy<PA: PluginApp>(app: &PA, instance: u32) {
        let live = app.live_typed_operation_slots() as u64;
        let peak = PEAK_TYPED_OPERATION_SLOTS.fetch_max(live, Ordering::Relaxed).max(live);
        if semio_framework_trace::runtime_diagnostics_enabled() {
            eprintln!("[TRACE] typed-operation slots instance={instance} live={live}/{} peak={peak}", crate::app::ARTIFACT_LIVE_OUTPUT_SLOTS);
        }
    }
"""
MATRIX_OLD = "|Download the (React|Vue) DevTools|typed-operation slots/;"
MATRIX_NEW = "|Download the (React|Vue) DevTools/;"
EDITS = {SDK: [(SDK_OLD, SDK_NEW)], MATRIX: [(MATRIX_OLD, MATRIX_NEW)]}


def main():
    mode = next((flag for flag in ("--dry-run", "--write", "--revert") if flag in sys.argv), None)
    if mode is None:
        print(__doc__)
        sys.exit(2)
    if mode == "--revert":
        for path in EDITS:
            saved = os.path.join(BACKUP, path)
            if os.path.isfile(saved):
                open(os.path.join(TREE, path), "w", encoding="utf-8").write(open(saved, encoding="utf-8").read())
                print("restored", path)
        return
    problems, staged, present = [], {}, 0
    for path, pairs in EDITS.items():
        before = open(os.path.join(TREE, path), encoding="utf-8").read()
        after = before
        for old, new in pairs:
            if new in after and old not in after:
                present += 1
                continue
            count = after.count(old)
            if count != 1:
                problems.append(f"{path}: expected 1 anchor, found {count}")
                continue
            after = after.replace(old, new)
        if after != before:
            staged[path] = (before, after)
    for problem in problems:
        print("PROBLEM", problem)
    for path in staged:
        print("edit", path)
    print(f"{len(staged)} files to change, {present} already applied, {len(problems)} problems")
    if mode == "--write" and not problems:
        for path, (before, after) in staged.items():
            saved = os.path.join(BACKUP, path)
            os.makedirs(os.path.dirname(saved), exist_ok=True)
            if not os.path.exists(saved):
                open(saved, "w", encoding="utf-8").write(before)
            open(os.path.join(TREE, path), "w", encoding="utf-8").write(after)
        print("written; backups under", BACKUP)
    sys.exit(1 if problems else 0)


main()
