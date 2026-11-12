# 1112 flowext
- Ported semio-os-flow-extensions (flow/extensions/wasm) from grant-shaped to effect-shaped retirement: ladders return `Result<RetainedCloneStep,RetirementPollExit>` (settled once via `RetirementPollExit::settle`), leaf steps run in `cx.with_original_effect` (hub helpers `evaluation_effect`, `evaluation_effect_cx`, `evaluation_effect_step`), work counters/`take_work` removed (effect charges the turn), cursors use `retirement_seed`/`close_into`.
- W-flowext-1 native: 117 -> 45 errors (in progress).
- W-flowext-3 native -p semio-os-flow-extensions rc 0 (lib). Next: wasm flow-bim-component.
- W-flowext-4 wasm (flow-bim-component): semio-os-flow-extensions compiles; next red is semio-os-flow-host-read (98 errors, unclaimed, last touched 11-11): claimed, porting.

## Final
- semio-os-flow-extensions: native W-flowext-3 rc0; wasm passes in W-flowext-4/7 (no errors from it).
- semio-os-flow-host-read (flow/host/evaluation-source/read), claimed 07:48: native W-flowext-6 rc0; wasm W-flowext-7 passes it. Edits: compare (reserve_one/release_empty_page effect + Ready/Denied outcomes), root (`.0` dropped, imports of source helpers, comparison steps take effect), request (resume/pause tuples), authoring (`Construction` in storage wraps `admit_original_construction_settled`; text closes via `close_with_poll`/`poll_outcome`; storage reserve/put/take/release through `cx.with_original_effect`; BudgetedOwnedEntry begin_close removed; reorganize reserve takes admission).
- Bim wasm check W-flowext-7 rc101: next red is semio-os-flow-host-session (40 errors, `FlowEvalSession` / `FlowSnapshotIntakeFailure` not found) - flowhost lane's crate, not touched.
- Debts: cfg(test) code in request/🦀️.rs and 🧪️tests still grant-shaped (--tests not checked); close-cursor work quotes (`next_computational_work`) are simple (1 / delegate), not runtime tested; Pending close polls map to zero-progress retry; BorrowedProjectedPackFailure on envelope close collapses to its ValueError reason.
STATUS: BLOCKED bim wasm W-flowext-7 rc101 behind semio-os-flow-host-session (flowhost); flow-extensions + flow-host-read FIXED-verified (W-flowext-3, W-flowext-6)
