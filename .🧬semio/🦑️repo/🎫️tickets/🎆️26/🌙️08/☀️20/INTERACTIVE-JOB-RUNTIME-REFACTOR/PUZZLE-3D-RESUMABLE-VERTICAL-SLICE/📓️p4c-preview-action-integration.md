# P4c Typed Preview and Action Integration

## Outcome

Puzzle 3D fill planning now crosses the shared plugin job boundary. `fillBuildTick` performs only two bounded UI-safe operations: poll the latest coalesced observation and, when no live fill job exists, emit one isolated `Effect::SpawnJob`. The registered `semio.puzzle3d.fill` job advances exactly one existing `FillBuilder: InteractiveJob` slice per `JobCtx::tick`, publishes typed progress, and stores the latest lossless checkpoint. It creates no worker pool or thread.

`FillBuildPreview` is revisioned and typed. It contains operation, base revision, generation, sequence, stage, target slot, candidate ghost (`BrushPreviewState`, including mesh URL and transform), broad-phase IDs, current pair, colliding IDs, bounded collision sample points, sample/inside counts, rejection reason, target/candidate cursors, accepted prefix/count, and search/rejection counts. The main Puzzle 3D surface projects the candidate as a translucent ghost and retains the complete typed preview alongside it for collision overlays.

## Runtime route

1. `fillBuildTick` polls `Puzzle3dPrecomputeSession::poll_fill_job`.
2. `enqueue_fill_job` enforces single-flight and captures operation/generation freshness.
3. The action emits `Effect::SpawnJob { kind: "semio.puzzle3d.fill", placement: Isolated }`.
4. The plugin registers that kind through `PluginBuilder::job`.
5. Every host grant resolves one `JobCtx::tick`, drives one bounded fill slice, publishes progress, and latest-keeps the fill checkpoint.
6. A scene generation change makes the old request stale; the existing fill cancellation/generation checks remain authoritative.

## Determinism and checkpoint state

- Preview fields are derived only from persistent `FillBuilder` state and its seeded RNG/collision state.
- The accepted prefix is the authoritative ordered `BrushPlacePayload` prefix, never a renderer reconstruction.
- Search and rejection counters are stored in `FillJobCheckpoint`.
- Candidate ghost, collision IDs/samples, and operation identity are stored in the preview inside the checkpoint.
- The existing checkpoint byte-identity, cancellation, stale-generation, step-budget parity, replay, and sub-8-ms tests remain the reference drivers; P4c adds typed preview encode/decode/checkpoint coverage and an enqueue-only test driver for native UI tests.

## Static safety census

Command:

```text
rg -n 'precompute_step|drive_step|run_to_completion|block_on|WorkerPool::new|std::thread|thread::spawn|rayon|spawn_blocking' <fill-build-tick component>
rg -n 'WorkerPool::new|std::thread|thread::spawn|rayon::|block_on\(|run_to_completion\(' <puzzle3d precompute subtree> --glob '*.rs'
```

Result: zero matches. Positive route census finds only the plugin registration, `SpawnJob`, and `JobCtx::{tick,progress,checkpoint}`.

## Gates

- Individual P4c Rust source parse/rustfmt checks: exit 0 for schema, collision state, controller, fill state machine, action, editor bridge, and main-window projection; logs `📝️p4c-rustfmt-check.txt` and `📝️p4c-rustfmt-check2.txt`.
- Static executor census: zero forbidden direct stepping/private-pool/thread matches in the action and precompute subtree. The positive route is exactly the plugin `.job(...)` registration, one `Effect::SpawnJob`, and `JobCtx::{tick,progress,checkpoint}`; log `📝️p4c-static-census.txt`.
- Focused native command: `CARGO_TARGET_DIR=<ticket>/🧪️target-p4 CARGO_INCREMENTAL=0 cargo check -p semio-s-plugin-puzzle --lib --message-format=json`.
- First log: `📝️p4c-native-check1.json`; exit 101 with exactly three upstream E0425 diagnostics in framework pack's included async component (`semio_framework_async::yield_once` at line 251 and `race2` at lines 254/256). Puzzle was not reached. The async-kernel owner then restored those exact primitives.
- Second log: `📝️p4c-native-check2.json`; Puzzle was reached with 2,704 structured errors from the repository-wide decorative-async migration. It identified two P4c action residues and two pure preview-helper residues. Those four pure functions were made synchronous rather than adding awaits.
- Final native boundary: `📝️p4c-native-check3.json` / `📝️p4c-native-check3-errors.tsv`; Cargo reports 2,701 total crate errors (2,699 primary structured rows). The P4c controller/fill/collision subtree has **zero** diagnostics, `fill-build-tick` has **zero**, the added schema range has **zero**, and the typed preview helpers have **zero**. Remaining diagnostics are the pre-existing Puzzle-wide async/value migration wall, headed by unresolved root `PuzzleApps` and thousands of unrelated pure-helper call-shape errors.
- Focused debug/release tests and wasm cannot be truthfully executed because all are compiled through the same monolithic Puzzle crate and stop at that native source wall before test code or target-specific P4c code is reached. No green result is claimed. The deterministic replay/batch-size/checkpoint/cancellation/stale-generation/watchdog tests remain in the owned source and will become runnable as soon as the broader Puzzle crate compiles.

## Files

- Puzzle 3D schema `FillBuildPreview`
- Puzzle 3D fill state machine/checkpoint
- Puzzle 3D precompute session and plugin-job bridge
- `fill-build-tick` action
- Puzzle 3D main-world preview projection
- Puzzle plugin job registration

## 2026-08-22 Current-tree verification

### Product and target evidence

- `📝️p4-suggestion-timing-focused-final-3.txt`: focused suggestion/session slice passes 10/10, including async accept, live preview, and keyed utility/session preservation.
- `📝️p4-full-j4-final-4.txt`: authoritative bounded native suite passes 1,124/1,124 with four test threads in 15.53 s. The unchanged watchdog tests, collision checkpoint/cancellation tests, fill checkpoint/replay tests, and deterministic drive-batch tests all execute in this run.
- `📝️p4-wasm32-unknown-final-2.txt`: `cargo check -p semio-s-plugin-puzzle --lib --target wasm32-unknown-unknown` exits 0. This exposed and repaired two Puzzle-owned WASM constructors so `Puzzle3dStore::new` and `Puzzle5dStore::new` propagate `VcsError`, plus a dropped 2D resize future.
- `📝️p4-wasm32-wasip2-final.txt`: `cargo check -p semio-s-plugin-puzzle --lib --target wasm32-wasip2` exits 0 after 3m48s.
- `📝️p4-release-lib-final.txt`: native release is **not green**. The check reaches Puzzle and fails with six E0425 diagnostics in `⏳️precompute/🪣️fill/🦀️component.rs`: missing release-scope imports for target enumeration/order, candidate enumeration/order, and fixture application. These spans are within the audited production-fill repair packet and were intentionally not patched during this evidence-only boundary.
- `📝️p4-nx-test-quick-final.txt` and `📝️p4-nx-test-quick-final-2.txt`: Nx is not green. Both invocations are killed by the repository wrapper's fixed 15,000 ms budget while `cargo nextest run --no-tests` rebuilds concurrently changed shared dependencies. This is a real required-gate failure even though it is not a Puzzle test assertion failure.

### Interaction repair in this verification slice

The suggestion precompute path now preserves the keyed session cache and advances brush candidate collision work with a persistent 2 ms cursor slice. The previous full-suite measurements of 12.559 ms for `openVortexSuggestions` and 11.740 ms for `acceptSuggestion` are gone; the focused 10-test slice and full 1,124-test j4 suite pass with the existing 8 ms assertions unchanged.

### Closure reconciliation

Phase 4 remains open. The independent current-tree audit in `📓️p4-closure-audit-20260822.md` is authoritative over the older outcome text above:

1. `fillBuildTick` still calls `precompute_step_lane` directly instead of being poll/enqueue-only and routing execution solely through the registered isolated job.
2. Target preparation, candidate preparation, broad-phase materialization, and acceptance still contain whole-collection/whole-application stages without persistent item cursors.
3. The existing 3D parity test varies drive batch sizes, not actual worker counts 1/2/4/default.
4. No 3D test establishes first substantive preview below 50 ms.
5. Nx quick remains red by its command budget.
6. Native release remains red with the six compiler diagnostics recorded above.

Accordingly, the narrow green timing tests and full suite do not establish the full adversarial Phase 4 contract, and `📌️important.md` must remain unchecked. No ticket lifecycle change is justified.
