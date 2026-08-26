# Coordinator Serialized Final Verification Matrix Contract — 2026-08-24

## Status

**Prepared, not executed.** Rust source packets are active, so Cargo, Nx, Wasm, browser, stress,
replay, allocation, and timing commands remain deferred. This contract defines the single-owner
order to use after source quiescence. A result from an earlier or overlapping tree is not evidence
for the final tree.

## Launch Registration Gap

The existing launch file registers native, `wasm32-wasip2`, and
`wasm32-unknown-unknown` strict-warning gates, the native plugin scale benchmark, Storybook,
collaboration end-to-end, test discovery/doctor/contract, oracle/subject/parity levels, affected
quick tests, exhaustive parity, and metrics enforcement.

It does not currently register the permanent `verify interactivity`,
`verify interactivity tool-jobs`, or dependency-freeze commands. Before final execution, a
file-disjoint packet must add these executable commands to `.vscode/launch.json` in the existing
`4_gate` order and naming taxonomy. The commands must call `bun ./📜️script.ts`; no second script is
allowed. The user's later all-app requirement supersedes the plan's earlier Compose exclusion, so
dependency and runtime verification must cover every currently declared application surface without
rewriting the baseline.

## One-owner Order

One build owner performs these stages serially and writes every transcript inside the master ticket.
It stops at the first red stage, records the exact failure, returns the source tree to remediation,
then restarts the matrix at stage 1 on the new quiescent tree.

1. **Tree and static hygiene**
   - working-tree `git diff --check` plus a separately reported cached-index check;
   - `bun ./📜️script.ts verify interactivity`;
   - `bun ./📜️script.ts verify interactivity tool-jobs`;
   - Rust and JavaScript dependency lists plus the all-ecosystem dependency freeze;
   - artifact builder/decomposer, schema representation, I/O serializer, I/O terminality, codec
     fidelity, standards coverage, analyzer, composer, migrated-builder, plugin-parity,
     contribution-target, and layering gates already registered in launch configuration.
2. **Compiler and target closure**
   - registered native strict-warning gate;
   - registered `wasm32-wasip2` plugin-library strict-warning gate;
   - registered `wasm32-unknown-unknown` actor-library strict-warning gate;
   - all relevant debug and release test targets through Bun/Nx-owned commands, never concurrent
     Cargo processes.
3. **Protocol and deterministic replay**
   - discovery, doctor, and contract gates;
   - oracle, subject, and parity quick gates;
   - affected quick tests followed by exhaustive parity;
   - the mounted torture/replay fixture at worker counts 1, 2, 4, and default, requiring identical
     commit bytes and preview/sequence semantics.
4. **Runtime interactivity and resilience**
   - native plugin scale benchmark and all mounted phase fixtures;
   - cancellation, stale generation, close/drain, queue saturation, memory pressure, worker fault,
     lost wake, resize/effect storms, multi-window invalidation, and device-loss paths;
   - metrics enforcement with every UI callback and worker step below 8 ms, UI callback p99 at or
     below 2 ms, cancel observation p99 below 8 ms, first substantive preview below 50 ms, and live
     preview cadence at or below 33 ms under load.
5. **Wasm and real-browser parity**
   - build every framework/API-affected Wasm target;
   - exercise worker scheduling, cancellation, replay, progress, resize, surface replacement,
     accessibility, locale, route, and renderer parity in the real browser;
   - preserve one externally driven opportunity per job step; no browser-only run-to-completion
     adapter may be accepted.
6. **Repository-wide final gates**
   - Storybook, collaboration end-to-end, exhaustive parity, and final metrics enforcement;
   - rerun dependency and Phase 8 classification gates after every generated artifact is present;
   - final tree/status/diff evidence and phase-by-phase acceptance ledger.

## Evidence Rules

- Each command records start/end timestamps, command identity, exit code, and complete output in the
  ticket folder. Temporary diagnostics retain the `[DEBUG] ` prefix and are not erased.
- A green exit without the hostile fixture/mutation required by its packet is insufficient.
- Timing evidence must come from runtime instrumentation, not source constants or unit-test names.
- Browser evidence must identify the exact route, locale, renderer, worker count, and observed
  callback/step maxima.
- The matrix does not authorize modifying unrelated peer work, the dependency baseline, or the shared
  Git index. Compose is in scope only where it remains a currently declared application surface under
  the user's later all-app requirement.

## Closure Consequence

No later phase or master ticket closes until this entire matrix is green on one final tree and the
repository ticket API is available for genuine closure. Manual metadata edits are not a substitute.
