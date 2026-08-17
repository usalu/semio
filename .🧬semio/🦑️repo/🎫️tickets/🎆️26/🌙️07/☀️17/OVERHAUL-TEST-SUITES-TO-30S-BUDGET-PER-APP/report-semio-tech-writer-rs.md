# @semio-tech/writer-rs — Test Suite Overhaul Report

## Unit
- Nx project: `@semio-tech/writer-rs`
- Root: `writer/rs`
- Crate: `writer` (Cargo.toml `[package] name = "writer"`)

## What ran today
- `writer/rs/script.ts` `TestScript` called the un-budgeted `runCargo(["test", "-p", "writer", ...segments], this.repoRoot)` — no wall-clock kill, build time folded into the same invocation as test execution.
- `writer/rs/project.json` `test` target already just called `bun ./📜️script.ts test` (no change needed there).

## Test inventory (in-source `#[cfg(test)] mod writer_vcs_tests` in `writer/rs/lib.rs`)
Only 3 tests exist, all inside `document_vcs` module:
1. `writer_document_vcs_replays_text_ops` — dispatches a `SetText` op through the VCS store, asserts the projection's text field updated.
2. `writer_document_vcs_replays_camera_and_document_ops` — dispatches `SetCamera` then a full `SetDocument` replacement, asserts partial-diff apply (camera) vs. whole-replace (document) semantics both work.
3. `writer_document_vcs_undoes_text_op` — dispatches `SetText` then `Undo`, asserts the reducer's `backwards()`/undo path restores prior state.

### Classification
All 3 are KEEP — they exercise the `Operation`/`OperationDiff` reducer implementation (`diff`, `apply`, `absorb`, `backwards`) for `WriterOp`/`WriterDiff`, i.e. real state-machine/undo-log branching logic per the task's explicit "reducers/state machines" keep-category. None are export-exists checks, getter/identity assertions, CSS/string-list comparisons, or duplicate loop-generated cases. Nothing was deleted.

## Runner migration
Mechanical swap, matching the pattern already used by `animate/core/rs`, `animate/video/rs`, `trinity/jack/lsp`, `fem/3d/rs`, `framework/graph/rs`, etc.:

    - import { BundleScript, ScriptRouter, runBundleScriptMain, runCargo, runWasmPackWebBuild } from "../../repo/lib/js/index.ts";
    + import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../repo/lib/js/index.ts";

      class TestScript extends BundleScript {
        run(segments: string[]): void {
    -     runCargo(["test", "-p", "writer", ...segments], this.repoRoot);
    +     runCargoTestBudgeted(["writer"], this.repoRoot, segments);
        }
      }

`runCargoTestBudgeted` (from `repo/lib/js/index.ts`) does an un-budgeted `cargo build --tests -p writer` followed by a budgeted `cargo test -p writer` (30s wall-clock kill via `SEMIO_TEST_BUDGET_MS`/`runTestBudgeted`). No other part of `script.ts` (the `WasmScript` class, `ScriptRouter` registrations, `runBundleScriptMain` call) was touched.

Nothing about this unit is genuinely e2e/integration (no server, container, or browser spun up), so no `test-e2e` target was added — the default `test` target is the right place for this.

## Timing measurement — could not be completed this session
I could not obtain a verified before/after wall-clock number for `cargo test -p writer` execution. Root cause was environmental, not code-related:

- The repo-wide `.cargo/config.toml` routes every `rustc` invocation through a single shared `sccache` server (`rustc-wrapper = "sccache"`). At the time of this ticket, dozens of concurrent sessions in this sandbox (visible via `ps aux`: builds for `raster`, `sequence_core`, `layout_rs`, `gis_2d`, `fem_2d`, `framework_graph`, `kernel_3d_brepkit`, a `NORM-TECHNOLOGY-FULL-FAMILY-CRATES` from-scratch build, a long-running `wgpu` wasm32 build, plus several `vitest` workers at 60-80% CPU) had driven load average to 22-38 on the machine.
- `sccache --show-stats` showed a growing backlog throughout the attempt: it went from ~2,190 pending compile requests to ~2,288 pending over roughly 30 minutes of observation, with system-wide throughput of only ~4 compiles/minute — i.e. the queue was not draining, consistent with the repo's own documented "Concurrent Cargo Workspace Churn" condition (expect 30-90+ minutes, poll rather than chase).
- I tried both the shared `target/` directory (blocked for 38+ minutes behind another session's in-progress wasm32 build holding the directory lock, 0 child processes / 0 CPU progress the whole time) and an isolated `CARGO_TARGET_DIR` under this ticket's scratch folder (to sidestep the directory-lock queue) — the isolated attempt still funnels every `rustc` call through the same contended `sccache` server and stalled for 20+ minutes on a single dependency (`adler2` v2.0.1) with its child processes pinned at ~0.01s CPU each.
- I deliberately did not modify the shared `.cargo/config.toml` (e.g. removing `rustc-wrapper`) since that's infrastructure shared by every concurrent session and out of scope for this ticket.
- I killed my own stray isolated build attempt once it was clear it would not finish in reasonable time, so as to stop adding to the shared backlog; the log of what it did compile is kept at `writer-rs-scratch/build.log` in this ticket folder as evidence.

baselineSec / finalSec: not measured (null) — I will not report a fabricated number for either. Structurally, however, I'm confident the actual `cargo test -p writer` execution phase (once built) is trivial: 3 in-memory, no-I/O reducer-state assertions on a tiny struct, the same shape of test that other already-migrated crates in this repo (e.g. `animate/core/rs`) run in well under a second. The risk this unit poses to the 30s budget is effectively zero; the only open question was the exact number, which the environment did not let me capture today.

## Files touched
- `writer/rs/script.ts` — migrated `TestScript` from `runCargo` to `runCargoTestBudgeted`; import list updated accordingly.
- `writer/rs/lib.rs` — reviewed only, not modified (all 3 existing tests kept as-is).
- `writer/rs/project.json` — reviewed only, not modified (already `bun ./📜️script.ts test`, no `test-e2e` needed).
- `.repo/🎫️/26/07/17/OVERHAUL-TEST-SUITES-TO-30S-BUDGET-PER-APP/writer-rs-scratch/build.log` — left in place as evidence of the stalled isolated-build attempt (the isolated `target/` directory itself was deleted to reclaim disk space).
- `.repo/🎫️/26/07/17/OVERHAUL-TEST-SUITES-TO-30S-BUDGET-PER-APP/report-semio-tech-writer-rs.md` — this report.
