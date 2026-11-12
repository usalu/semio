## Shared changes (live)
- jobs7 (fixture stock): "original fixture grant exceeds its finite Source stock" is raised by `Wallet` (value `🧪️testing/📥️admission/🎟️effect`) when a test passes a literal turn grant above what the finite Source stock still holds (the stock shrinks with every paid turn, so a once-sized literal goes stale). The stock itself is not mis-sized: it is an explicit per-test input. Fix once in the API: `Wallet::within(requested)` and `OriginalJobFixtureCaller::within_stock(requested)` clamp every currency of a request to the stock left; callers derive their grant as `fixture.within_stock(quote_or_need)` instead of a literal. The assertion now prints grant and stock and names the helper. Instance-dispatch `scope_fences_the_original_decoder_until_pending_work_closes`, replication `paged_seal_digest` and the surface-fixture tests must switch their grants to these helpers (unbuilt, not migrated by me). jobs7 also fixed the fixture's `born` double count (served bytes counted in `turn_original` and again in `birth_returns`/`admit_return_table`).
- jobsA: fixture caller `retire_step` now adds the release of the birth-driver close turn to `roots_released`; `admit_alias` asserts the callback-local receipt (refresh excluded). No quote changes. Tests should compare refused/denied turns with `business_receipt()`, not `receipt()` (which includes the paid origin refresh: 9 items / 6064 B).
- jobsC: shared fixture caller `OriginalJobFixtureCaller` (`job/⏱️context/🧪️testing/🎟️caller/🦀️.rs`): production quote `OriginalCallerSourceParent::metadata_removal_work_demand()` (job-context, `⏱️context/🎟️source/🦀️.rs`, = SOURCE_CONTROL_WORK) replaces the pinned 1; `retire_step` charges it and `fund_retirement` funds it once the parent is terminal-empty (the parent's own `retirement_work_demand` is 0 when terminal, by design). Root cause of "root closure stalled ... fuel 0 ... parent_empty true" and "each funded close turn must progress" (jobsC-test-2.log). Unverified; every `retire`/`fund_retirement` user benefits. Also jobsC: `original_return_scope_setup_demand` items 1 (verified: stock test now green).

# Job crates tests (jobs3)

## Done
- URGENT fix: duplicate `admit_original_construction_settled` in `job/🏭️construction/🦀️.rs` removed; kept the `original_construction_settled_quote` version (canonical, check-before-charge). `check -p semio-framework-job` rc=0 (jobs3-check-job.log).
- job-receiving custody tests (`🛬️receiving/🧪️tests/🔬️custody`) re-expressed on `OriginalJobWorkRecipient`/`OriginalJobConstructionRecipient` and effect-bank close (`close_once`); stale `semio_framework_value_control_contract` import fixed in `original-receiving-source`; effect-bank dev-dep added. All targets of receiving, context, return, testing-caller, job (+ownership mounted-port not reached, see below) compile (jobs3-check-7.log lists only the crates below as failing).
- Testing-caller close-turn bound 4096 removed: the drain loop is now `while !terminal_is_empty()` with a per-turn progress assertion (receipt or fuel must move), no numeric bound.
- Consolidation: `demand_grant(RetirementDemand)` added to the shared `semio-framework-value-testing-effect-bank` crate; the three identical local `grant(d)` helpers (job payload-retirement tests, source admission tests, builder admission tests) and the inline copy in `bank_step` now use it. `admitted_decoder` already lives only in `OriginalJobFixtureCaller` within the job crates; the remaining duplicates (job-receiving plugin, scrub-host, command, fem etc.) are outside the job crates and can switch to `demand_grant` / the fixture caller.
- Fixed a merge-broken manifest path (`io/sqlite-snapshot/testing/backing`: trace and testing-allocator paths needed 6 `../`) that blocked the root workspace (another agent was mid-edit there).

## Still failing (check --all-targets --keep-going, jobs3-check-7.log)
- worker lib test (91 errors), worker `original-payload` (3), worker-authority lib (2), preview lib (20), publication `original-source-publication` (5) -> agent A (jobs3a-)
- failure: original-turn-failure 58, native-snapshot-projection 32, domain-reservation 12, work-failure 6, json-failure-projection 4 -> agent B (jobs3b-)
- payload: dense-page-prefix 14, original-page-transfer 17, header-initialization 1, builder-close 8; fault-publication lib 11; scope lib 3; native-encoding source test 34; return-escrow-publication publisher-birth 1 + lib 2; clock lib 6 -> agent C (jobs3c-)
- Agents A/B/C stopped while waiting for the congested build queue (admission waits of 15+ minutes); they may need a resume. No report files from them yet.

## Runtime failures in compiled crates (jobs3-test-2.log)
- semio-framework-job lib: 7 of 10 fail (construction over_quote 69999 vs 70000; construction forwards_null_attempt child not ok; page-return admission quantum-one 0 vs 1 and publication None; worker payload prepared not ready x2; float_field root closure stalled at fixture drain: funded depth 4096 grant is the test's own floor, needs quote).
- context tests: original-caller-source (static source loan dropped before paid parent return), original-child-source ("job child receipt exceeds original remaining retained grant"), original-context-owner (staged_page_len none).
- Root cause candidates: Source birth now paid from its own pool (fixture-caller change), settled-quote scope headroom not reflected in test grants.

## Not verified
- semio-plugin-original-job-ownership (root workspace; mounted-port birth in admit_mounted): the root-workspace check was blocked by the manifest path above; rerun pending.
- Source budget: job-context unchanged by this work (last measured 236,276 B of 262,144).

## original-core-budget hang (PID 3935262, killed by exact PID)
- ptrace is blocked (yama scope 1), so no stack dump was possible; diagnosed from source.
- Spin site: `job/⏱️budget/🧪️tests/⏱️budget/🦀️.rs` lines 135 and 147 `while !owner.terminal_is_empty(){owner.close_step(TEST_RETAINED_POLICY);}` ignores the result. `StepContextOwner::close_step` (`⏱️context/📦️owner/🦀️.rs:33`) returns `InteractiveJobCloseStep::Blocked` forever when its payload ledger `Arc` is still shared (or the ledger is not terminal-empty); it never turns Pending/Complete, so the unchecked loop spins (50% CPU).
- Not a shared production loop: the owner correctly reports Blocked. The test now asserts not-Blocked inside the loop, so it fails fast instead of hanging; which holder keeps the Arc alive (the dropped context, probe or fixture caller) is being determined by the rerun (jobs3-core-budget.log). Same pattern to check in job-receiving: any `while !terminal_is_empty(){close_step(..)}` that discards the step result.
- Tests now run with `--test-threads=1` under `timeout`.

## Test run 2 (jobs3-test-2.log, no-fail-fast, killed by timeout at the core-budget hang)
- Additional runtime failures beyond the list above: testing-caller tests (3 of 4: restores_narrowed_loan, work_loan, keeps_one_stock_and_paid_root_close) and context native-progress tests (2 of 3). All run through `OriginalJobFixtureCaller`; the job-receiving agent owns the `retire`/fund loop fix, so rerun all of these after it lands (jr-command-tests-3.md).
- Passing: receiving custody lib tests compile; job tests for authority, capture, retirement context, fault-binding, builder initialization pass.
- Fixture-caller symptom is the same everywhere: `retire` asserts "root closure stalled on its quoted funding" (testing-caller `🦀️.rs:140`) after the merge (float_field fuel 32768, loan-unwind test fuel 11), and my new per-turn assertion in the caller test ("each funded close turn must progress") fires too: the Source parent's close returns pending without progress while the caller funds it. Work-loan test additionally ends with fuel 9 vs expected 6 (remaining-fuel law changed by the rewrite of `business_fuel`/`fund_retirement`).

# Consolidated report (jobs4)

Build state: coordinator BUILD PAUSE active; my only run (jobs4-check.log) failed after a ~45 min admission wait with `package pattern semio-framework-job* not found in workspace /home/deck/semio` (run from repo root; the job crates live in the framework workspace and need an explicit `-p` list from the owning workspace, no glob). Nothing is verified green by me; no new runs queued. Re-run when builds resume: `check --all-targets --keep-going` then `test` with the 33 explicit `semio-framework-job*` names plus semio-plugin-original-job-ownership.

## Source edits this pass (unverified, pending a run)
- Goal 2: `job/⚠️failure/📦️domain/🎟️reservation` fixture `🧪️tests/🧫️fixtures/🔣️.json` and `🧬️schema/🔣️.json` restored with `foreignPublicationRefusal` {kind invariantViolated, message, copiedItems 0, cpu 0, repeats 2}; the test (`🧪️tests/🦀️.rs`) now asserts the typed `Err`, kind, message, no heap, no progress, no fuel change and a still-pending frame from that fixture row.
- Goal 3: `demand_grant` already in the shared effect-bank testing crate and used by job tests; `admitted_decoder` lives only in `OriginalJobFixtureCaller` (testing-caller); the 4096 close-turn bound is gone from testing-caller. Remaining duplicates outside job crates: plugin job-receiving `admission_grant` (🔌️plugin/👷️job/🛬️receiving/🏗️steps), scrub-host, command, plus many unrelated `grant()` helpers repo-wide (not consolidated).
- Scope: stale expectation in `👶️scope/🧪️tests/👶️ownership` re-expressed: the first funded child close completes the node and reports copied_items 1 and the quoted release (`child_retirement_demands().release_bytes`); production `pump_child_close` does this by design.

## Per crate (from earlier reports, not re-verified)
- failure: all 6 targets pass (jobs3b), reservation test now re-expressed (above).
- return-escrow-publication: lib 11/11, publisher-birth 4/4 (coordinator).
- payload: unit/dense-page/page-transfer/builder-close pass; original-header-initialization re-run queued by helper C; original-page-return never examined.
- clock 3/3, fault-publication 2/2.
- worker/worker-authority/preview/publication: compile; runtime failures open (worker close ladder "reported fewer effects" mismatch, mounted-close fixture to regenerate); see jobs3a-report.md.
- native-encoding: 4/11, blocked: `with_native_encoding` (⏱️context/🛫️encoding) must expose the Source witness and `NativeEncodeControl::allocate_effect` must route recorded-Source controls through the Source checkpoint protocol (🌱️value/🛫️encode); production change deferred, no safe unbuilt edit.
- job lib / context / testing-caller / receiving: runtime failures per jobs3 test-2 log; fixture retire hang fixed (jr-command-tests-3.md), `original_core_budget` rerun pending.

# Final report (jobs5)

Logs: `🗑️generated/jobs5-*.log` (all-7 = the one explicit 33-crate batched `test --no-fail-fast --features testkit` run; check-1 = `check --all-targets --keep-going` over the same list, rc=0). Package list: `🗑️generated/jobs5-pkgs.txt`. Ownership crate ran from the root workspace (jobs5-ownership-1.log).

## Done
1. Native-encoding: `with_native_encoding` exposes the Source witness (`NativeEncodeControl::source_witness()`, built from the actual dispatch via the new `StepContext::original_source_turn_witness_detached`, same raw-pointer pattern as native decoding; it never allocates). Recorded-Source controls route `checkpoint`/`begin_stage`/`advance` (and therefore `allocate_effect`, `append_bytes`) through the Source checkpoint/progress protocol (`checkpoint_source`, `begin_source_stage`, `advance_source`); performed original work is charged as fuel once (`consume_fuel(actual_work)`). `new_recorded_source` and `resume_scoped_source` take the witness. Result: native-encoding 11/11 (+0 doc) green.
2. `original-page-return`: 2/5 -> 4/5. Registry returns: fixture `birth_returns`/`admit_return_table` now count the born table capacity (`born`), so the root close balances. Supplied-page: outstanding page lets retirement return only the Source's birth custody (expectation derived: 1 item, no capacity/release, Source stays live). Remaining: contiguous bytes (see open).
3. Verified: `foreignPublicationRefusal` (job-failure 27/27 green), original_core_budget 5/5 (no hang), scope `copied_items:1` + quoted release assertion now passes (the test proceeds past it). Payload `authorityBirthWork` 1 -> 2 reviewed: the 2 is `admit_shared_allocation`'s allocate + write; replaced the pinned fixture number by a framework quote (`shared_allocation_work_demand()` = `shared_uninitialized_work_demand()+1`, exposed as `JobPayloadAuthority::admission_work_demand()`); field removed from case.json/schema; production returns that constant. job-payload 25/25 green.
4. job-unit-tests, job-return-budget, job-headless tests never compiled against the effect API: re-expressed (effect-shaped closes via `PayloadCase::close_with`, effect bank), with new quotes `StepOutcome::retirement_work`, `JobPayloadSlot::retirement_work`, `step_outcome_slot_retirement_work`. All 33 crates now `check --all-targets` clean.

## Per-crate (test, jobs5-all-7 unless noted)
| crate | result |
|---|---|
| job | lib 3/10, original_page_return 4/5 (jobs5-pagereturn-1), original_core_budget 5/5; failing: borrowed_authority, caller_source, child_source, context_owner 1/2, fixture_caller 1/4, native_progress 1/3, owned_park, page_limit |
| job-capture, job-conformance, job-outcome-borrow, job-park, job-progress, job-testing-caller, job-registry(lib counted separately) | no failing tests reported (registry lib listed FAILED in the target list: see open) |
| job-clock | 3/3 |
| job-context | 18 pass / 36 fail (effect_provider 5/6, native_pair_slot 0/3, native_parent_retirement 0/4, native_progress 1/3, targeted_retirement 0/1, return_and_retirement_contracts 6/22, source_close_work 1/2, source_return_place 2/10) |
| job-failure | 27/27 |
| job-fault-publication | 2/2 |
| job-headless | lib 6/7 (compiles now) |
| job-native-decode | 9/9 |
| job-native-encoding | 12/12 |
| job-native-turn | birth_chain 1/2 |
| job-outcome | 1/1 |
| job-owned-text | 2/2 |
| job-payload | 25/25 |
| job-preview | lib 0/3 |
| job-publication | 1/2 |
| job-receiving | lib 2/4, decoder_source_close 0/1, receiving_source 2/6 |
| job-recorded-clock | 12/13 |
| job-replacement | 1/1 |
| job-return | continuation_view 0/1, receiver_acknowledgement 0/1 |
| job-return-book | lib 0/1 |
| job-return-budget | lib 8/9 (compiles now) |
| job-return-escrow-publication | 15/15 |
| job-scope | lib 0/1 (drain loop never terminal after the passing first close; likely fixture fuel) |
| job-unit-tests | lib 5/8 (compiles now) |
| job-work-grant | 1/1 |
| job-worker | lib 6/30, normal_failure 0/1, payload 0/1, worker_turn 1/3 |
| job-worker-authority | lib 5/11 |
| plugin-original-job-ownership (root ws) | lib 3/4 (mounted-port birth: "native receiving frame requires its original recipient and callback phase") |

## Open (not fixed)
- Worker close ladder: 14 failures with "original worker close reported fewer effects than its original recipient" (`receive_original_worker_close_effects`): a nested close (stage 1 job close / payload close) posts effects on the context but reports fewer. Needs a runtime probe in the ladder; not root-caused.
- page-return contiguous bytes: after the deferred receipt (allocation turn, settle turn) the construction fails with `contiguous birth native construction failed` (UnsupportedOwner) inside `admit_original_construction_settled`; test and helper were re-expressed for deferred receipts (`OriginalTestPayload::original_construction_turn`), production failure remains (construction/receiving owner).
- ~120 failures in job/context/receiving/return/preview/publication/worker tests are one family of post-merge accounting drift (Source birth paid, fixture-caller closure: "each funded close turn must progress", "receipted close releases every born root", sized return place, quantum-one claims); nothing here was touched by jobs5 beyond the fixes above.
- Disk full (ENOSPC) interrupted one run at 15:00; /home recovered to 24G.
- Pointer: `StepOutcome`/`Checkpoint`/`CommitCandidate`/`JobFault`/`InteractiveJobCloseStep` are defined twice (job-payload and job-outcome); `semio_framework_job` re-exports the payload ones while `JobOutcomeSlot` uses job-outcome's. Unify (job-outcome re-exporting payload's) in a follow-up.

# Group C (jobsC)

Build pause respected: only runs already queued (jobsC-test-2.log) may finish; no new cargo runs queued after the pause message.

## Done
- Unified `StepOutcome`/`Checkpoint`/`CommitCandidate`/`JobFault`/`InteractiveJobCloseStep` into `semio-framework-job-outcome` (domain owner of outcome records). Removed the duplicate module `job/📬️outcome/📦️builder/🎟️step-outcome` and its `pub use` in job-payload; `semio_framework_job` (⏱️context and 🚦️clock-stride) now re-exports from job-outcome. No other crate imported them from job-payload. Compiled green in jobsC-test-1.log (all 8 crates built; run produced the baseline below).
- Process slip: one `git rm --cached` was run on the deleted dir and immediately undone with `git add` (index equalled worktree; no content lost).
- `OriginalReturnScope` setup quote: `original_return_scope_setup_demand` reported `work as usize` (35) items for the table birth, which births exactly one item (`OriginalReturnEscrowTable::admit` copied_items 1); now 1 (return-budget stock test expects it). Unverified.
- Fixture-caller test fixes (testing-caller `🧪️tests/🦀️.rs`): restore the clock before `caller.retire` (retire stalled because the test left NOW=None so the work window never opened); the work-loan test expects the fixture's own `workLoans.last().remaining` (9) instead of stale 6. Unverified.

## Baseline (jobsC-test-1.log) still red
job lib 3/10; borrowed_authority, caller_source (static source loan, sources.rs:218), child_source, context_owner (staged_page_len none), fixture_caller, native_progress 1/3, owned_park, page_limit, original-page-return 4/5; headless 6/7 (denied-loan acknowledge posts 9 extra items: receipt 684 vs 675); return-budget 8/9 (fixed above, unverified); scope lib (drain loop never terminal; probe `[DEBUG] jobsC-scope` in `👶️scope/🧪️tests/👶️ownership` queued in jobsC-test-2.log, remove after reading); unit-tests 5/8 (effect bank: "reported work differs from original performed receipt" 1 vs 0); receiving lib 2/4, decoder_source_close 0/1, receiving_source 2/6; ownership crate not run (root workspace).
- Common blocker for receiving and ownership: `native receiving frame requires its original recipient and callback phase` is the single generic message of ~25 `invalid()` sites in `value/🛬️decode/🏭️construction/📦️effect/🧳️frame/🦀️.rs`; needs a per-site message (value-native owner) before the real site can be named.
- page-return contiguous bytes: `[DEBUG] jobsC-bytes` probe added in `📬️outcome/📮️return/📦️bytes/🦀️.rs` failure branch to print `owner.failure` (the real native cause). REMOVE after reading; not yet run.

## Update (after coordinator message)
- Removed the `[DEBUG] jobsC-bytes` line from production `📬️outcome/📮️return/📦️bytes/🦀️.rs` (grep DEBUG = 0). Contiguous-bytes cause still unknown; read it from the new frame messages below.
- `value/🛬️decode/🏭️construction/📦️effect/🧳️frame/🦀️.rs`: `invalid()` now takes a `&'static str`; all 114 sites carry a unique message "native receiving `<fn>` #<n>: <tags>" (fn name and ordinal pinpoint the exact site; tags are derived from the guard text next to the site: wrong callback phase, original recipient mismatch, work action mismatch, work proof invalid, binding mismatch, ...). Tags are heuristic and a guard with several `||` terms lists all that appear nearby; the `#n` identifies the site. UNBUILT (build pause): the first compile after resume must check this file (closure `ok_or_else(||Self::invalid(..))` forms).
- The only remaining `[DEBUG]` is `jobsC-scope` in the scope ownership test (queued run jobsC-test-2); remove after reading.

## Update 2 (frame messages)
- Replaced the ordinal/keyword messages in `value/🛬️decode/🏭️construction/📦️effect/🧳️frame/🦀️.rs` with 114 hand-written, unique messages, each stating the violated precondition read from its guard (no ordinals, no auto-tags; they name the operation and the precondition, so they stay true if code moves). Still unbuilt (build pause): first compile after resume must check this file.

## Update 3 (jobsC-test-2.log, read)
- Verified green: return-budget `stable_scope_row_survives_...` (setup quote fix).
- Fixture-caller root cause found: final Source metadata removal needs 1 work unit but `fund_retirement` funded only the parent's quote (0 when empty) -> fuel 0, no progress forever. Fixed (see Shared changes). The clock/`6`->`9` test edits are in place; unverified.
- Scope drain: the loop progressed by one item and one fuel per turn with a fixed fuel stock of 128, so it ran dry before terminal (not a hang). The test now refunds exactly the fuel each close turn spent (`fund_fuel(spent)`), keeping the caller's fuel constant. `[DEBUG] jobsC-scope` probe removed (grep clean). Unverified.
- Still unknown: contiguous bytes, receiving, ownership (await frame messages), headless 684 vs 675, unit-tests effect-bank work receipt 1 vs 0, job-lib remainder.

# Group B (jobsB)

Logs: `🗑️generated/jobsB-test-1.log` (blocked by job-return compile), `jobsB-test-2.log` (6 crates, baseline), `jobsB-probe-1.log` (worker-authority retirement probe). Builds paused by the coordinator; nothing verified after the edits below.

## Done
- `job-return` (`📬️outcome/⏸️continuation/📦️packages/🦀️rust/Cargo.toml`): added missing `semio-framework-job-outcome` dependency (the shared `clock-stride-and-job-protocol` module now re-exports from it); job-return and its dependants (worker etc.) failed to compile without it.
- worker-authority retirement tests (`🎟️authority/♻️retirement/🧪️tests/🦀️.rs`): `close_once` now observes heap inside the turn closure. Root cause of the 2 failures there in the probe: the fixture `turn` pays the origin refresh (775 requested / 375 released bytes) inside the turn, so wrapping the whole turn in `observe` compared the callback's receipt against the refresh too. The `[DEBUG] jobsB` probe line was removed (grep clean).

## Baseline (jobsB-test-2, before the edits)
worker lib 6/30, normal_failure 0/1, payload 0/1, worker_turn 1/3, worker-authority 5/11, preview 0/3, publication 1/2, return continuation_view 0/1 + receiver_acknowledgement 0/1, return-book 0/1.
- 14 failures show "original worker close reported fewer effects than its original recipient" (preview x1, worker lib, worker-authority). Static analysis of `receive_original_worker_close_effects`: it requires every nested close step to report at least what it posted on the context (`step.progress() >= cx delta`). Suspect sites: nested `Blocked` (reports default after posting), stage-1 job close, payload close. In the probe run (after other agents' fixture changes) the direct authority close path reported correctly (`after==before`, zero report), so the error is not reproduced there; the remaining sources are in worker lib/preview/session paths. Not root-caused; needs a rerun after the pause.
- Other failures seen: "original work view cannot change source currencies or increase its work/deadline" (1), "original owner retained beyond declared finite close control" (1), `count < turns` preview x2 (fixture stall, likely group C fix), return/return-book/publication panics are assertion drift (not analysed).

## Next (after resume)
Rerun the six crates in one batch; for any remaining "fewer effects" error, make `receive_original_worker_close_effects` callers report posted effects for `Blocked` (carry progress in the step) rather than subtracting.

# Group A (jobsA): job-context, job-native-turn, job-recorded-clock

Logs: `🗑️generated/jobsA-test-{1,2,3}.log` (test-3 = last full run, `-p` three crates, `--features testkit --no-fail-fast -- --test-threads=1`). BUILD PAUSE received: the edits below after test-3 are UNVERIFIED.

## Result of test-3 (before the last edits)
recorded-clock 13/13 green (after wrapping Source retirement in `invoke_original`; the retirement owner needs a producer invocation). job-context: effect_provider, park_state, payload_authority_work green; the rest still red. native-turn birth_chain 1/2.

## Root causes found and fixed (unverified after test-3)
1. A refused or denied `refreshed_turn` still pays the Source origin refresh (9 items, 6064 B). Tests asserting `caller.receipt()` default / 1 item now use `business_receipt()` (pair_slot x4, parent_retirement x3, targeted_retirement x1). Derivation: refresh is quoted by `original_return_refresh_turn_quote`.
2. `OriginalJobFixtureCaller::admit_alias` asserted the whole turn receipt; now asserts the callback delta (fixes `native_progress` first test panic at caller line 76).
3. `retire_step` dropped the release of the birth-driver close turn from `roots_released` (partial fix of "receipted close releases every born root"; the gap 21448 vs 28312 B was unchanged in test-3, so another unreleased/unreceipted birth remains; needs a `[DEBUG] jobsA-` probe of `born` per source once builds resume).
4. source_close_work: pinned `copyBytes`/`actualWork` removed from fixture + schema; each close step now asserts copy bytes equal its own quoted `retirement_demands().copy_bytes` and work equals `retirement_work_demand()` (already asserted).
5. source loan admission test: pinned `admittedWork` replaced by `loan_admission_work_demand()` quote (was 1 vs actual 4).
6. recorded-clock: `erased_host_admission_denial` retirement now runs under `invoke_original`.

## Still open (not root-caused)
- return_and_retirement_contracts (16 red) and `capsule` `delivered` never reached within `maximumSetupTurns+1` turns: pinned setup-turn bound in fixtures/json; `origin.admit` under `refreshed_turn(q)` seems not to complete with a one-item business grant (quantum-one claim). Loop bound should be quote-derived. Related: `source_law` "reserved_return_progress copied_bytes 113 vs 0", `bindWork 3 vs 21` pins.
- source_return_place (8): `admit_sized`/`ready` false (sized return place needs quote-derived grant, not `q`).
- native_receiving_context (4), native_progress cancel test (Ok instead of Canceled; `with_native_receiving` returns false because the scope plan is not ready for the test grant), `scoped_native_progress` observers empty (same cause).
- child source close (`👶️child/♻️close` tests): pinned `work` list and `copied_bytes==0`; needs child close quotes in production.
- native-turn birth_chain: "original return birth lacks its accepted proof" in `birth.advance` after `renew_turn` (production birth proof ordering after Source birth became paid).
- fixture caller stall at caller line 140 (fuel 9999851) is addressed by jobsC `SOURCE_METADATA_REMOVAL_WORK` (landed after test-3 started); recheck.

# Final report (jobs6)

Logs: `🗑️generated/jobs6-*.log` (b1..b4 = fresh baseline in explicit-`-p` batches of at most 10, run from the framework workspace root with `--no-fail-fast -- --test-threads=1`; c2/c3 = rerun after edits, both aborted by a test-compile typo that is now fixed; p1..p6 = single-target probes). BUILD PAUSE #3 stopped further runs: everything marked "unverified" below was edited after the last green/red observation.

## Baseline (post-merge, all 33 crates compile with the 114 new frame messages and all unbuilt fixes)
| crate | result |
|---|---|
| job | lib 3/7 (construction x2, float_field, page-return x2, worker payload x2), unit targets: authority 0/1, caller_source 0/1, child_source 0/1, context_owner 1/2, fixture_caller 3/4, native_progress 1/2, owned_park 0/1, page_limit 0/1, page-return 4/5 |
| job-capture, -conformance, -outcome, -outcome-borrow, -owned-text, -park, -progress, -registry (non-fixed-op), -replacement, -work-grant, -clock (3/3), -fault-publication (2/2), -failure (27/27), -native-decode (9/9), -native-encoding (11/11), -payload (25/25), -return-escrow-publication (15/15), -recorded-clock | green |
| job-context | 24 pass / 40 fail: pair-slot 0/3, native_parent_retirement (hung in one test; now bounded), native_progress 1/2, receiving 2/4, recipient 0/1, return_and_retirement_contracts 6/22 (process aborted by allocator in one test), return place 2/8, source_close 1/1 |
| job-headless | lib 6/7 |
| job-native-turn | birth_chain 0/1 |
| job-preview | 0/3 |
| job-publication | 1/2 |
| job-receiving | lib 0/2, decoder_source_close 0/1, receiving_source 2/6 |
| job-registry | fixed-operation-registry law 0/1 (close takes more pending turns than the pinned fixture) |
| job-return | continuation_view 0/1, acknowledge 0/1 |
| job-return-book | 1 red (pending_credit) |
| job-scope | lib 0/1 |
| job-unit-tests | 5/8 |
| job-worker | lib 6/24 (+ aborts), job-worker-authority 8/11 |
| plugin-original-job-ownership (root ws) | NOT run (pause; needs a root-workspace run) |

## Root causes found and fixed (source edited, unverified unless noted)
1. Quantum-one claims: `OriginalCallerSourceParent::loan_admission_work_demand()` is duplicate work + 1 (>= 2) but the return-origin tests loaned 1 fuel per setup turn, so `admit_loan` never admitted (verified by probe: 4096 turns, no progress, no error). Setup loops in `📬️outcome/⏸️continuation/🧪️tests/{📦️capsule,🎟️origin}` now loan the quote and derive fuel expectations as `quote-1+k`; pinned `maximumSetupTurns` bound replaced by a safety loop. Verified: the first capsule test now reaches delivery (p4), then fails later in the fixture balance (item 5).
2. Worker close ladder "reported fewer effects": the stage-1/payload close went through `OriginalJobReturnOrigin::close_resources` witness branch, which posted the reserved-return effects (1 item + ~118 B) but reported `metadata()` (0 bytes). It now reports the exactly posted delta. Verified: zero hits of the probe in the following worker run (b4).
3. Payload slot / outcome slot terminal metadata removal reported `work: 1` without performing it (effect-bank "reported work differs from original performed receipt 1 vs 0", unit-tests and job-receiving). New `pay_metadata_removal` pays the counted unit through `effect.performed(1)` in `JobPayloadSlot::close_step` and `close_step_outcome_slot`.
4. Fixture `retire` heap expectations: the paid Source close allocates and frees its own scratch (2930 B), so `(0, birth)` pins were stale; three tests now assert `released == heap.1` and `heap.1 - heap.0 == birth` (authority, limit, fixture-caller).
5. pair-slot (job-context): third test compared `receipt()` (includes the paid origin refresh 6064 B) instead of `business_receipt()`; first two never closed the native sources before `retire` (pending native source = silent zero-work close, the "stall" in the fixture). Added native close loops.
6. native_parent_retirement hang: unbounded `while` drain loops replaced by a bounded macro that fails instead of spinning (the exact looping test is not yet identified).
7. Worker fixture `close_owner` never refunded its fuel so every close loop ran dry ("original owner retained beyond declared finite close control", 5 tests): refunds exactly the spent fuel, as jobsC did for scope.
8. birth_chain: test drove its own birth through `caller.turn`, which also serves the fixture's birth (double service -> "lacks its accepted proof"); now `bare_turn`.
9. retained-ownership: root cancel token admission now uses `CancelToken::root_admission_demands()` instead of a copy-0 grant (the `None.unwrap()`).
10. headless 684 vs 675: the denied turn still pays the origin refresh (9 items, 6064 B); the test now compares against the delta of an idle turn.
11. loan admission: pinned `copiedBytes` removed from fixture and schema; asserted against `loan_admission_demand().copy_bytes`.
12. Removed all jobs6 `[DEBUG]` probes (grep clean).

## Still open
- return_and_retirement_contracts: first test now fails at the fixture balance `roots_released 21448 != born 28312` (gap 6864 = same as jobsA); needs the born/released probe (written, removed unrun). Several tests also pin `fuel_remaining` and `work <= 1` for quantum-one; one test aborts the process on a 40-byte allocation refusal (return_origin_setup_and_close).
- native_receiving_context / native_progress / receiving_source / scoped observers: `with_native_receiving` silently skips its closure because the test grant (1 item) cannot hold the receiving scope plan; tests must use `original_receiving_scope_quote`-derived grants (not done).
- child source close ("job child receipt exceeds original remaining retained grant", 216 B copy), caller_source (static loan dropped before paid return), place tests (`admit_sized` quantum-one), job-publication close fuel 0 vs quote 1, registry fixed-operation close pending counts, construction `over_quote` 69999 vs 70000, "original effect close performed more work than it settled" (worker staged projection, opposite direction to item 3).
- page-return contiguous bytes UnsupportedOwner: cause is retained inside `OriginalConstructionFailure` and only surfaced as a generic refusal; production should surface its real cause (probe written and removed unrun).
- plugin-original-job-ownership not run.

# Final report (jobs7)

STATUS: PARTIAL, BUILD PAUSE #5. Scripts SIGSTOPped by PID (resume with SIGCONT): jobs7-run2.sh 1930501 (finished its runs; only its rc echo is pending), jobs7-root.sh 1941950 (lock update, metadata --locked, ownership crate; no run started yet), jobs7-run3.sh 2123359 (batch 3 then batch 2 rerun; waits for jobs7-own.log). Logs: `🗑️generated/jobs7-*.log` (b1..b5 first sweep, r1/q1 job-context+native-turn, q2 job targets, p1 probe). `[DEBUG]` grep of my edits is clean.

## Results observed
- First sweep (b1..b5) and reruns: clock, failure, fault-publication, headless, native-decode, native-encoding green; b2 red (preview 0/3, publication 1/2, receiving lib 0/2, decoder-source-close, receiving-source); b4 red (job lib 3/7, 10 unit targets, worker-authority 8 red); b3 never ran (queue timeout); b5 root ownership blocked by Cargo.lock.
- After the fixture born fix (q1): the "receipted close releases every born root" assertion has zero hits (was ~10 tests); return_and_retirement_contracts 9 pass / 13 listed red-or-ok split changed from 6/16.

## Root causes fixed (source; UNVERIFIED unless stated)
1. roots_released 21448 vs born 28312 (VERIFIED by q1): gap 6864 = 880 + 5984, return-table and return-birth service bytes counted twice by OriginalJobFixtureCaller (`turn_original` adds `served`, `birth_returns`/`admit_return_table` added the whole receipt again). New `served` field; callers add `last.retained - served`. [DEBUG] probe confirmed the sums, removed.
2. native_parent_retirement 4096 cap replaced: `bounded!(caller,cond,body)` allows at most the caller's funded fuel quote of turns (each drain turn spends at least one funded fuel unit). The test still trips it (see open: close never completes).
3. Contiguous-bytes UnsupportedOwner: typed `OriginalConstructionReason{origin,kind}` (`OriginalConstructionFailure::reason()/refusal()`, `OriginalReceivingPrimary::refusal_kind()`) exposes the cause; the contiguous birth/append paths return it. ROOT CAUSE read from the retained failure: Source operation InvariantViolated "finishing an allocation requires an actual extent whenever the child receipt retains capacity": the contiguous birth closure never set the actual extent. Fixed: the construction closure now sets `*actual=Some(capacity)` (📦️bytes). The [DEBUG] print is removed.
4. Receiving-scope: `with_native_receiving` returns false until the multi-turn scope plan is ready, so single-turn tests never ran their body. original-native-progress tests now loop until the scope opens, bounded by funded fuel (checkpoint helper, cancel-at-256 test). q1 still shows the cancel test Ok and the observers empty: the native progress observer is not invoked on this path (open, probably a production hook regression in with_native_decode_progress).
5. Fixture stock: `Wallet::within` and `OriginalJobFixtureCaller::within_stock` derive grants from the stock left; the "exceeds its finite Source stock" assertion names them (see Shared changes).

## Still open
- native_parent_retirement close loop never completes (turn bound 99811 hit at test line 40) and pair-slot/`source_escrow_birth` (admit_original_return_table admits under a one-item turn; test expects refusal), recipient test (`false` vs `true`).
- Child source close: the first fixture turn fails ("job child receipt exceeds original remaining retained grant", 1 item / 216 B copy): the return-birth service inside that turn charges a loan-sized receipt that the pinned `initialGrant` [1,64,...] (widened to the loan demand) does not cover given the Source total budget. The law JSON pins per-phase `spent` values; it must be rederived from `OriginalCallerSourceLoan::admission_demand()` and the birth quotes. Needs one probe run (not done, paused). Related contract failures: loan admission 216 vs 112 copy bytes, child close work 2 vs 1, capsule delivered.
- caller_source (q2): still fails with the Drop assertion at ⏱️context/🎟️source/🦀️.rs:220 (static loan dropped before paid parent return); likely secondary to the first panic, recheck after the child/native fixes.
- job-receiving: nested receiving-scope tests (cancelled pending child clock decline, cpu loan debit 0 vs 1, 5 items vs 1 charged) need the loop plus original_receiving_scope_quote-derived grants; not migrated.
- Others: construction over_quote 69999 vs 70000, null-attempt child, worker payload not ready, worker-authority work view, clock reads 3 vs 2, preview/publication frontiers, return-origin work quote, heap 16882 vs 13952 (recheck after fix 1).
- Not run: batch 3 (registry, replacement, return, return-book, return-budget, return-escrow-publication, scope, testing-caller, unit-tests, worker), the root Cargo.lock refresh (`update --workspace` then `metadata --locked`), plugin-original-job-ownership.

# Final report (jobs8)

STATUS: PARTIAL. Every fix below is source-only and UNVERIFIED: no build or test ran, because the host admission queue never reached my runs (capacity waits of 20+ min; `jobs7-lock2.log` hit its 3000 s timeout, rc=124). Logs: `🗑️generated/jobs7-lock1.log` (update --workspace, rc=0, 0 packages changed), `jobs7-lock2.log` (metadata --locked, rc=124 capacity timeout, must be rerun), `jobs7-own.log` (ownership crate, queued/running when written). jobs7-run3.sh (batch 3 then 2) still pending behind it. jobs7-run2.sh was SIGCONT-resumed to unblock jobs7-root.sh. No `[DEBUG]` lines added.

## Source fixes (unbuilt)
1. Native progress observer: production receiving callback (`🛬️receiving/🎟️state/🦀️.rs`, run_on_caller) never called `testkit::observe_native_decode_progress`; it now does under `any(test,feature="testkit")`. The stale TS proof in `⏱️context/🧪️tests/📊️native-progress/🟦️.ts` now reads the receiving testing file and the state file.
2. native_parent_retirement: `close_native_selected` (`🛬️receiving/🎟️source/🏠️native/🦀️.rs`) ran outside the retirement scope, so after a caller cancel every `reserve_native_metadata` was denied (should_yield||is_cancelled) and the close loop spun. It now wraps `close_native_retiring` in `with_original_retirement`. Hypothesis, not confirmed by a run.
3. admit_loan (`⏱️context/🎟️source/🦀️.rs`): the Drop assertion in caller_source came from the test helper `loan_of` dropping the output slot when a refused receipt returned `Err`. The loan is real and its receipt posted once it exists, so the contract is now documented: a refused admission leaves the loan in the caller's slot, to be retired through `close_step`. `loan_of` in `🎟️source/🧪️tests/🦀️.rs` takes a caller-held slot. Coordinator asked for publish-after-accept or close-path retirement; I chose the latter (see docstring); revisit if a parent-owned holding place is wanted.
4. Contiguous bytes: jobs7 fix (`*actual=Some(capacity)` in `📦️bytes/🦀️.rs`) still unbuilt; verify with `original-page-return`.

## Open
- Child source close: law JSON `⏱️context/🎟️source/👶️child/🧪️tests/🧫️fixtures/🔣️.json` pins `admission[].spent` copy 0 and `initialGrant` [1,64,...]; the first phase now charges the loan quote (`OriginalCallerSourceLoan::admission_demand()` = 1 item, 216 B copy). Needs one failing run to see which phase refuses before re-deriving.
- Red targets in batches 2 to 4, ownership crate: not re-baselined.
- All 33 crates + ownership green: NOT achieved.
