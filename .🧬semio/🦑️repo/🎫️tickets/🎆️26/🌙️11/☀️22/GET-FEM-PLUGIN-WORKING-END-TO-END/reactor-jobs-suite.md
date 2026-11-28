# reactor-jobs-suite: `semio-plugin-reactor-jobs --lib` green

Crate: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs` (package `semio-plugin-reactor-jobs`).
Run natively with `reactor-jobs-run.sh <tag>` (private `C:\t\rj`, `C:\t\rjtmp`, `-j 3`, detached). Per-test isolation: `reactor-jobs-each.sh`, `reactor-jobs-one.sh`.

## Result
- Before: 24 passed, 31 failed.
- After: **59 passed, 0 failed** (55 existing + 3 new `step_coarse` laws + 1 restore law). Last run after the checkpoint change below. `a_selection_or_tool_switch_reserved_job_settles_in_a_bounded_number_of_crossings` stays green throughout.
- `step_coarse` itself needed no change. Production code is untouched except the test-only driver `🎛️registry-driver/🦀️.rs`.

## Root cause: stale fixtures, not a defect (plus four stale expectations)
The original-caller contract moved on: every turn must come from a real `OriginalJobFixtureCaller`/`OriginalWorkFixture` that owns a Source parent, and every owner must be funded with the grant and fuel it quotes. The suite still built bare `StepContext::new(.., ||Some(1), ..)` contexts (no Source parent: "original caller has no parent source supplier") and funded turns with one-unit grants. No guard was weakened; the fixtures were rewritten to the current contract with the shared helpers.

Concrete contract changes the laws now follow:
1. **Caller:** bare contexts replaced by `OriginalWorkFixture` turns (`loan` helper = turn plus `with_original_work_loan`) or by `RegistryJobDriver` (one caller per job, closes its job on drop).
2. **Funded demands:** retirement, construction and reply steps quote a demand (for example copy 424, capacity 392, depth 3) that is no longer adaptive to a tiny copy grant. Turns are widened to exactly the quoted demand (`fund_demand`, `source_grant`, `slot_grant` in the driver) and `original_construction_settled_turn_quote` for construction.
3. **Retirement mode:** `close_original` runs inside `with_original_retirement`. Retirement ignores the deadline by design (`retirement_should_yield`), so the "expired deadline denies a source close" row became "retirement still advances by exactly one unit at an expired deadline"; the fuel-0 denial row is kept.
4. **Loan guard:** a work loan cannot be replaced (`narrow_original_work_budget` now fails with "replaced its initiating context"); the expired-budget row uses a nested `with_original_work_loan(fuel, 0, ..)`. A fault reply pack step costs effect work plus its receipt debit, so the reply law issues `REPLY_STEP_FUEL` and asserts spent fuel never exceeds it (the loan guard enforces that) and that no-progress turns spend nothing.
5. **Lazy registry:** `LazySlots` materialises a ~10 MB slot table on first dereference. That allocation is cold setup, so the measured admission scopes call `warm_registry()` first (heap equality stays asserted at `(0,0)`).
6. **Resident construction:** the reserved body keeps its construction admission until a paid close, so "shallow at Done" became "source and source close empty at Done", and `close_body` asserts full shallowness afterwards.
7. **Failure value:** a failing body keeps its whole `OriginalTurnFailure` (`ExtensionInvocationCause::Turn`), and failure replies are canonical wire values, not `Fault` packs; tests read them through `fault_view` (`code`, `message`).
8. **Expired-clock fixtures:** the Source's own deadline of 100 us with a clock at 1 is declined at decoder admission ("original Source clock declined new construction"). The reserved expiry laws use `RESERVED_DEADLINE = 1_000_000` and step the clock to one past it. I did not isolate the exact minimum margin the Source requires (see open points).

## Files edited
- `🎛️registry-driver/🦀️.rs` (test driver, `#[doc(hidden)]`): `turn`, `start_source`, `step_with_fuel`, `step_via`; `slot_grant` now always funds a bounded body's quoted demand (an admission-failure reply or a cancelled body is stepped, not only closed); extracted `closing_grant`/`slot_grant`/`fund_demand`/`source_grant`/`widest` as `pub(crate)`.
- `🧪️tests/🔬️unit/🦀️.rs`: `Fixture` (per-job `RegistryJobDriver`), `loan`, `owner_for`, `delta`, `warm_registry`, `fault_view`; `FixedOriginalFixture::retirement_demands` quotes the source cursor birth demand; checkpoint, source-close, failure-projection and body-failure laws rewritten.
- `🧪️tests/🎟️admission/🦀️.rs`: all four laws on `OriginalWorkFixture` turns.
- `🏛️reserved/🧪️tests/🦀️.rs`: construction and close turns funded by quotes, expiry laws, plus the three new `step_coarse` laws below.
- `🧬️mutation-plan/🧪️tests/🔬️unit/🦀️.rs`, `🔀️migrate/🧪️tests/🔬️unit/🦀️.rs`: use `Fixture` and `fault_view`.
- `🧬️mutation-plan/🧪️testing/🧬️job-test-mutations/🔣️.json`: inverse vector `zero` stores `[]`. `AddValue(0)` has an empty diff, and the portable inverse-admission law (`🎮️mutation/🧪️tests/↩️inverse-admission`) admits no inverse for a non-participating operation. The corpus was stale, not the implementation (no other consumer of this corpus outside the crate).
- Scripts here: `reactor-jobs-run.sh`, `reactor-jobs-each.sh`, `reactor-jobs-one.sh`.

## `step_coarse` witnesses
No test in the repo is literally named a progress, stall or deadline witness for the guest crossing; the production caller (`step_job_original`, `step_job_owned` in `🎟️context/🌉️call/🎭️exports`) is only reached through the component. I therefore ran every existing law that exercises step, progress receipts, stall bounds and deadlines through the registry (all green), and added three laws in `🏛️reserved/🧪️tests/🦀️.rs` that drive the real reserved job through `abi::step_coarse`:
- `a_coarse_crossing_runs_no_micro_step_at_an_expired_deadline`: an expired work-loan deadline retains nothing and spends no fuel; the same job then settles with the exact bytes.
- `a_coarse_crossing_returns_when_its_micro_step_makes_no_progress`: a copy-starved crossing stops after the stalled step (no byte progress, bounded fuel) and the job later settles.
- `a_coarse_crossing_is_capped_and_a_long_payload_spans_several_crossings`: a payload beyond `COARSE_STEP_CAP` micro steps takes at least two crossings and delivers the exact bytes.
Existing coverage: progress receipts (`reserved_registry_original_body_bytes_and_actual_receipts_share_one_source`, work-denial law), deadline (`..._expired_clock_...` x3, admission `jobWorkDenials`), stall bounds (`original fixture exceeded its finite step bound` guards in `Fixture::terminal`/`slice`).

## Checkpoint replay input (coordinator decision: event-sourced, a checkpoint must restore the job)
Earlier I had adapted the checkpoint law to `None` after admission; that is reverted. The law is back to its strict form: the entry carries the input before and after body admission.
- `BoundedJob::replay_input(&self) -> Option<&[u8]>` (required, no default): a body borrows the input it owns until its close, so nothing is cloned per step. Implemented for the two-phase builtins (migrate, mutation-plan, io-sniff, ...), the inference job, the reserved job and the test fixture; `AdmissionFailedJob` returns `None` (it is a terminal fault reply, not a restorable job).
- `checkpoint_jobs` takes `source.input` before admission and the body's `replay_input()` after, and copies it once per call, plus the body checkpoint.
- New law `restoring_a_mid_run_checkpoint_matches_the_uninterrupted_run_in_output_and_receipts` (`🧪️tests/🔬️unit/🦀️.rs`): checkpoint taken after the first progress page, the job cancelled, restored from the entry's kind, input and checkpoint; the final output and the final step's receipt equal the uninterrupted run. The migrate and mutation-plan checkpoint laws now restore from `entry.input` instead of the test's own copy and assert it.
- `RegistryJobDriver::receipt()` added for the receipt comparison.

## Open points for the owners
- **Source deadline margin:** the minimum distance between the fixture clock and the Source deadline at decoder admission is unmeasured; worth a one-line law next to `OriginalCallerSourceAdmission`.
- **Peers' concurrent edits** (`🧵️job/⏱️context`, store init escrow) did not break this crate during the final runs; the crate does not depend on the escrow hang in `store-init-escrow-hang.md`.
