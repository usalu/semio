# @semio-tech/puzzle-3d-rs — test suite overhaul report

## Status: Runner migrated to budgeted execution; timing NOT obtained (environment blocker, not a code issue)

## What was found
- `puzzle/3d/rs/script.ts`'s `TestScript.run()` called `execFileSync("cargo", ["test", "-p", "puzzle_3d", ...segments], { stdio: "inherit", cwd: this.repoRoot })` directly — not routed through any budget helper.
- `puzzle/3d/rs/lib.rs` (2342 lines) has two `#[cfg(test)]` modules:
  - `mod tests` (11 tests, lines 1681-1983): AABB/oriented-box containment, brush-candidate collision
    filtering against real mesh geometry, weighted vortex/candidate ordering, brush-placement fixture
    mutation + id-collision regression, precompute-queue no-op-resync regression, fill-count
    increase/decrease prefix-preserving replan, mesh-registration cache invalidation.
  - `mod puzzle3d_vcs_tests` (2 tests, lines 2301-2340): document-VCS granular-op replay and
    delta-ops round-trip/backwards-inverse regression.
- **Classification: all 13 tests are KEEP.** Every one exercises real branching/algorithmic logic
  (geometry, weighted RNG ordering, state-machine/queue diffing, VCS operational-transform round-trip)
  against hand-built fixtures — none are export-exists/getter/CSS-substring/serde-padding style
  trivia. Nothing was deleted.

## What was done
- Migrated `puzzle/3d/rs/script.ts`'s `TestScript` from the raw `execFileSync` call to
  `runCargoTestBudgeted(["puzzle_3d"], this.repoRoot, segments)`, imported from
  `../../../repo/lib/js/index.ts` (same relative depth already used for `runWasmPackWebBuild`).
  This mirrors the identical, already-landed pattern in `fem/3d/rs/script.ts`. This alone makes the
  `test` target wall-clock-budgeted (30s hard kill) going forward regardless of the timing question
  below — it was previously unbounded.
- No test file edits (nothing trivial to remove), no other files touched.

## Why timing wasn't captured
Both a normal `cargo build --tests -p puzzle_3d` (shared `target/`) and a retry with an isolated
`CARGO_TARGET_DIR` (to sidestep lock contention) were attempted:
1. The shared-`target/` build blocked >23 minutes on `Blocking waiting for file lock on build
   directory` — held by another concurrent session's long-running
   `cargo check -p vcs -p os-hub-storage -p os-hub-storage-sqlite -p os-hub-storage-postgres
   -p os-hub-storage-neo4j -p semio-framework-plugin-host` (elapsed >1h43m at time of observation;
   `vcs` is a direct dependency of `puzzle_3d`).
2. Switching to an isolated `CARGO_TARGET_DIR` under the scratchpad avoided the lock but hit raw CPU
   starvation instead: `uptime` showed sustained load averages of 25-34 on a 10-core machine (80+
   concurrent `rustc`/`sccache` processes system-wide from other sessions), and the isolated build's
   own process sat at 0% CPU with zero net compile progress (stuck on the same log line, `Compiling
   memchr v2.8.0`) for 6+ continuous minutes of observation.

This matches the repo's known "Concurrent Cargo Workspace Churn" pattern (many simultaneous
agent sessions sharing one Cargo workspace/machine) rather than any defect in `puzzle_3d` or in the
`script.ts` change — no compile error was ever observed, only lock contention then CPU starvation.
Both background build attempts were killed (mine only) and the scratch target dir removed after
~35 minutes of attempts, rather than continuing indefinitely.

## Result
- Test count: 13 before, 13 after (no deletions — all classified as KEEP).
- Baseline/after wall-clock seconds: not measured (blocked as above).
- The runner is now budget-enforced (`runCargoTestBudgeted`) so a future run — whenever the shared
  machine/workspace isn't saturated — will self-report if it exceeds 30s rather than run unbounded;
  re-running this ticket unit once contention clears would give the actual number.

## Files touched
- `/Users/ueli/Documents/semio/puzzle/3d/rs/script.ts` (migrated `TestScript` to `runCargoTestBudgeted`)
