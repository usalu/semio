# Third Independent P1y Release-Err Source/Static Acceptance Re-Audit

Date: 2026-08-24  
Auditor: Codex, independent read-only source/static audit  
Verdict: **GREEN — no concrete P1y A3 release-Err source/static counterexample found.**

## Scope And Method

Read the repository and OS instructions; the post-RED report, P1y caller census/clarification,
and current implementation report; the live compaction, index, snapshot, engine-facade, CLI, and
root P1y verifier sources. The selected graph is:

`db CLI cmd_compact` → `Database::compact_document` →
`DatabaseCompactionFuture::try_submit` → typed `Lane::Io` driver.

This audit is limited to A3 backend lease-release `Err` recovery and P1y/P1x/P1w/P1q regression
confirmation. No source or root-verifier edits were made. No Cargo, Nx, build, Wasm, browser, or
runtime tests were run.

## A3 Static Trace

`DatabaseCompactionLeaseRecovery::release_future` owns exact `storage`, `resource`, `holder`, and
the mutex-held `fence`. It consumes that fence and sets `released` only inside `if result.is_ok()`.
On every backend `Err`, it resets only the in-flight `releasing` claim. Thus the retained fence and
the exact storage/resource/holder identity remain discoverable.

The first retained retry failure occupies `core.release_fault`; later failures occupy
`core.release_retry_fault`. A completed execution is parked in `core.release_waiting` until the
successful release witness exists. `schedule` refuses terminal callback-close while a future,
release-waiting output, either release-fault owner, or armed release retry remains. `terminal_is_empty`
also requires each of those owners absent before admission can be empty.

Release retry is a real worker-loop path: `arm_release_retry` uses `WorkerPool::callback_at`, whose
callback mounts the typed `release_future` and calls `schedule`; `submit_exact` submits that exact
job through `Lane::Io`; only `drive_one` calls `poll_one`. Callback terminal maintenance never calls
`poll_one`, so it cannot repoll the main compaction future or backend release directly.

After release `Ok(())`, the callback retires exactly one retained release-fault owner per callback
opportunity. It publishes the parked output only after both fault slots have been retired. For a
panic path, quarantine retirement and `release_terminal` remain gated on `released`; only then is
`panic_retired` set and public fault completion permitted. A sustained `Err` therefore retains the
fence, first fault, admission, and registry entry while blocking a public terminal.

## Faithful Harmful-Mutation Reproduction

The root P1y verifier first accepts the actual source, then mutates in-memory copies of the exact
live source and requires each mutation to produce at least one static failure. It rejected all six
A3 release-Err regressions:

| Harmful mutation | Bound live construct | Gate result |
| --- | --- | --- |
| Unconditionally consume fence | `if result.is_ok()` → `if true` | rejected |
| Mark released on `Err` | reset `releasing` → set `released` | rejected |
| Drop first retained fault | `core.release_fault = Some(error)` → `drop(error)` | rejected |
| Bypass release callback registration | `callback_at(... release_retry_callback())` → `drop(state)` | rejected |
| Remove release-waiting terminal guard | `core.release_waiting.is_none()` → `true` | rejected |
| Repoll live work from callback maintenance | inject `self.poll_one()` into close callback | rejected |

The root verifier also requires both actual production-path law bodies: one injected release failure
then success through the worker loop, and perpetual injected release failure retaining the fence,
fault, admission, registry, and leased backend state. This is source/static evidence only; their Rust
runtime bodies were not executed in this audit.

## Regression And Static Checks

| Command/check | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity p1y` | PASS — live source and hostile mutations clean |
| `bun ./📜️script.ts verify interactivity p1x` | PASS |
| `bun ./📜️script.ts verify interactivity p1w` | PASS |
| `bun ./📜️script.ts verify interactivity p1q-b1-b6` | PASS |
| Scoped `rustfmt --edition 2021 --check` (compact/index/snapshot/engine/CLI) | PASS |
| Scoped `git diff --check` (P1y sources/verifier/census) | PASS |
| `rg` selected facade/CLI and retained-region census | PASS — selected path reaches retained authority; release polling is scheduled through `Lane::Io`; no callback-maintenance `poll_one` path |

## Acceptance Basis

The previous RED trace (failed release retry treated as released) no longer follows the live source:
the fence and released witness are success-conditional, release faults and parked output remain
retained, the only release retry path is callback-at → schedule → `Lane::Io`, and terminal/public
completion remains closed until success plus incremental retirement. No concrete source/static A3
release-Err counterexample remains in the audited scope.
