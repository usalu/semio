# Coordinator Independent P5b Live Reactor Reconcile Final Audit

Date: 2026-08-23  
Verdict: **REJECT — the mounted synchronous reconcile seam is gone, but source ownership, bounded-work, and instance-close invariants are not yet complete.**

## Scope

This is an independent read-only audit of the P5b source packet after the author marked it audit-ready. No Cargo, Nx, Wasm, browser, or runtime command was run while other Rust source packets remained active.

## Confirmed Source Progress

- The mounted reactor no longer calls `PatchTracker::diff`, `SurfaceReconciler::reconcile`, or `snapshot().revision`.
- `PatchTracker` owns fixed arrays for surface, rejected, terminal, deferred, unadmitted, and closing-instance records. The reactor calls exactly one `drive_one` opportunity per actor turn.
- The reconcile job carries generation, cancellation, operation, and exact source/reconciler authority by value; ready publication is ordered by admission generation.
- Active reconciliation traverses the presented tree with persistent cursors and retires cursor/previous state incrementally before publication.
- Rejected and terminal public types expose retry/resume/take/close APIs, and ordinary job/rejected/terminal drop hands the retained state to the fixed global terminal registry.
- The author-recorded rustfmt, parser, verifier, forbidden-scan, and diff gates are internally consistent with the inspected source. Executable gates remain pending.

## Blocking Findings

### 1. Dynamic semantic payload is neither measured nor rejected before it is cloned

`SurfaceReconcileCursor::step` computes `node_page_bytes` from the `FlatPresentedNode` shell, key, and child-id array only. It excludes dynamic payload inside the component, accessibility value, labels, data attributes, and other record fields. During `DiffRecords`, `build_record`, `record.clone`, and `diff_record` can therefore clone an arbitrarily large semantic payload before `estimate_record_bytes` runs. That estimate still counts only the record shell, key, and child-id array.

This permits one grant to copy unadmitted dynamic data and makes the claimed 16 KiB page / 2 MiB operation bound false for the live payload. Admission and the cap/+1 fixture must cover every dynamically owned field before any clone or candidate mutation, with a controlled maximum and over-maximum identity witness.

### 2. The mounted unadmitted path can discard the exact tree and has no progress or close consumer

The reactor calls `retain_unadmitted(surface, tree)` after `begin` returns the exact owner. `retain_unadmitted` returns only a generation; when all 65 entries are occupied, it silently drops `surface` and `tree`. Even below saturation, no production path takes, retries, resumes, or incrementally closes an unadmitted entry. `has_work` observes these entries forever, while `close_step` never visits them.

The cap/+1 path must return or retain the exact pointer-identical owner without a recursive drop. Mounted drive must retry or expose it, and app/instance/realm close must retire it one bounded owner at a time.

### 3. Instance close does not cover every retained effect/owner class

`begin_close_instance`/`close_step` scans only `slots`, then local terminals marked `close`. It does not cancel or retire matching `ready` patches, `deferred` surface keys, or `unadmitted` trees. A patch made ready before close can therefore be published after the instance is closed. `terminal_is_empty` also omits both `ready` and `deferred`, so it can report empty while those owners remain.

Instance close must generation-filter and incrementally retire all surface slots, rejected owners, terminal owners, unadmitted trees, deferred requests, and ready effects. A fixture must close with each class populated, prove no stale patch publishes, and reach exact terminal emptiness.

### 4. Local terminal saturation hands ownership to an unmounted registry

Both `mark_rejected` and instance `close_step` use an `if let (Some(terminal), Some(target))` insertion. If the local terminal array is full, the terminal is dropped and handed to the global `SURFACE_RECONCILE_TERMINALS` registry. The mounted tracker never calls `take_surface_reconcile_terminal`, so the actor can lose its close authority while later reporting local emptiness.

Local saturation must keep the source slot/close cursor retryable, or mounted code must explicitly drain the global handback using generation-tagged ownership. Add terminal-cap/+1, ordinary-drop, and ABA fixtures covering the mounted route.

## Required Repair Gate

1. Measure every dynamic semantic owner before copying it; reject page/item/byte cap +1 with exact identity and no large pre-rejection clone.
2. Make unadmitted saturation lossless and give retained unadmitted work a mounted retry/take/close route.
3. Extend app/instance/realm close over ready, deferred, rejected, terminal, active, idle, and unadmitted classes, one bounded retirement opportunity per grant.
4. Eliminate the unmounted local-terminal overflow handoff or explicitly drain it from the actor.
5. Add permanent live-source predicates and mutations for each repaired failure mode, then rerun rustfmt, verifier self-tests, plain deny, and scoped diff checks before another independent source audit.

Phase 5 and P5b remain RED. The distinct upstream `plugin_render` materialization and the full serialized executable matrix also remain open after these source defects are repaired.
