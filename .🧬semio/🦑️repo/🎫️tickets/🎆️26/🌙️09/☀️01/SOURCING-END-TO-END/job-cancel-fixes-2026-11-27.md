# Job Cancellation Fixes (2026-11-27)

Follows `job-cancel-audit-2026-11-27.md`. Rule applied: every `step` checks cancellation before any work or state change and returns a typed Cancelled; release/retire is left to `close_step`.

## Shared helper
- `StepContext::cancelled_turn()` in `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs`: `Some(Result<Option<JobOutcomeBorrow>,OriginalTurnFailure>)` = the Cancelled admission when cancelled, `None` otherwise. Call sites are one line: `if let Some(turn)=cx.cancelled_turn(){return turn}`.
- Shared test driver `semio_framework_job::cancel_probe::{CancelProbe,ProbeTurn}` (feature `testkit`, `🧰️framework/🔨️modules/🧵️job/🧪️testing/🛑️cancel-probe/🦀️.rs`, wired in job `🦀️.rs`). It prepares the receiving place first (a source cancelled from birth can never prepare it), then reports the kind of each turn (`cancel`, `step`, `step_to_outcome`, `turn`, `release`, `close`).

## Fixes
| Job | Change | Test | Status |
|---|---|---|---|
| `ArtifactReservedToolJob` (`FP/📜️declaration/👷️job/🧱️ownership/📦️producer`) | entry gate before `try_lock`/lending when no inner job is held (a lent job delegates to its inner, which checks itself) | `lending_tests::a_cancelled_step_cancels_before_lending_the_job_and_a_lent_job_cancels_at_its_next_step` (state still holds the job, `lent=false`; lent job cancels at next step). Stub now checks cancel via the helper | pass (`semio-plugin-original-job-ownership`, 3/3) |
| `SearchJob` (`🀄️wfc/⚙️engine/🔍️search/🏃️batch`) | gate before the `yield_next` branch | `a_search_cancelled_before_its_first_step_...` (owed yield stays owed, no extra Yield) and `a_search_cancelled_mid_run_...` | pass (`semio-s-plugin-wfc-engine-solve`, 3/3 incl. old law) |
| `EchoJob` (`🧩️puzzle/🎮️commands/📤️jobs`, test-module job) | gate before `outbox.phase` demand quoting / wire take | `a_cancelled_echo_step_cancels_before_any_work_and_mid_run_at_the_next_step` (wire untouched when cancelled first) | pass (`semio-s-plugin-puzzle-jobs --features testing`) |
| `LayoutExportJob` (`📏️layout/.../⚙️engine/📤️export/⏭️advance-commit-and-codec`) | gate moved to first statement, before `slot.settle` and before driving a staged publication (previously a staged publication was delivered under cancel); the old later check removed | `a_cancelled_step_returns_cancelled_before_it_settles_or_drives_a_staged_publication`, `a_job_cancelled_mid_run_cancels_at_the_next_step_without_advancing_its_stage` in `🧪️tests/🔬️unit/🎟️harness-pdf-and-dispatch-laws` | lib `cargo check` ok; tests written but NOT run (see Open) |
| `GuestRelayLifecycleProbeJob` (`FP/🖥️host/🔀️post-turn-relay/🔬️lifecycle-probe-and-refused-source`) | gate at top of `step_callback` | none | file is orphaned (see Open) |

## Verify-only results (read, no change needed)
- `MountedLayoutJob::worker_one`: `close_requested || is_cancelled` is the first statement after a test-only observation; ok.
- `FemJobGraph`, `AssemblyJob`, `LdltJob`, `SubspaceIterationJob`, `PcgJob`, `MeshJob`: every `run()` checks `is_cancelled` on entry and per bounded unit; `step_plain` only retires an already delivered outcome / re-admits a pending run before that; ok.
- `ArtifactStoreInitializationJob`: replay `turn_inner` reads `is_cancelled` first, then only moves to `RetireCancelled` (release); ok.
- `Stub` (producer) is `cfg(test)`; now cancel-aware.

## Extra edits needed to build/test (not part of the fix list)
- `🀄️wfc/⚙️engine/🍰️chunk/🧪️tests/🔬️unit/🦀️.rs`: added `settled()` unwrapping `SearchResult` (tests did not compile against the new `SearchResult` return); `🔧️repair/🧪️tests/🔬️unit/🦀️.rs`: added `use crate::outcome::SolveOutcome;`.
- `📏️layout/.../📤️export/🧱️primitives/🧪️testing/🦀️.rs:65`: trailing `;` after `caller.retire(..)` (now returns `usize`).
- Dev-dependency `semio-framework-job` with `testkit` added to `🀄️wfc/⚙️engine/🧗️solve` Cargo.toml.

## Wasm impact (sourcing hub / extension)
Sourcing has no wfc/puzzle/layout dependency, so only the framework fixes can reach its wasm:
- `StepContext::cancelled_turn` (job crate `⏱️context`): compiled into every wasm that links `semio-framework-job` (hub and extension).
- `ArtifactReservedToolJob` entry gate (plugin `📜️declaration/👷️job/🧱️ownership`): behaviour change in the hub/extension wasm (reserved-route jobs, retained-command reserved producers).
- Not in sourcing wasm: `SearchJob` (wfc), `EchoJob` (puzzle test module), `LayoutExportJob` (layout), `cancel_probe` (testkit only), `GuestRelayLifecycleProbeJob` (not compiled at all).

## Open
- `LayoutExportJob` tests are unverified: the layout export crate's `testkit` feature no longer compiles because of concurrent peer changes (`🧪️testing/🦀️.rs` calls `BatchJobSession::try_admit_owned` and `RetainedCloneProgress`, neither resolves). The non-test lib compiles. Re-run: `cargo test -p semio-s-artifact-layout-layout-engine-export --features testkit -- a_cancelled_step_returns a_job_cancelled_mid_run` from `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout`.
- `GuestRelayLifecycleProbeJob` (and `GuestColdRelayJob`, mounted-session files) sit in `🖥️host/🔀️post-turn-relay/` files that no `mod`/`#[path]` includes; the one-line gate is applied but nothing compiles or tests it. No test added (would be dead).
- Tests run one at a time with `cargo test -j 2` in each plugin's own workspace (plugins are excluded from the root workspace); target dir `/Users/ueli/t/src-cancel` deleted, logs removed. Helper script kept: `🗑️generated/cancel-fix-cargo.sh` (`WD=<workspace dir>`).
