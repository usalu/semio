# Job return-place and retained-context native tests (root causes, fixes)

Verdict: not caused by the new reservation code (`reserve_rows` is only called from `lend_step`, and none of these tests reach it). They are drift from the 733/746 per-turn origin-refresh chain, plus one open product question (test 8).

Run: plain cargo, `CARGO_BUILD_RUSTC=rustc RUSTC_WRAPPER=`, private `CARGO_TARGET_DIR` and also a private `CARGO_BUILD_BUILD_DIR` (the repo config's shared build dir `C:\semio\.cache/b/...` is NOT moved by CARGO_TARGET_DIR alone; sharing it with the running fem build produced "failed to write ... invoked.timestamp" errors).

## 1. `original-source-return-place` (job-context): 10 tests, now 8 pass, 1 deleted, 1 open

Root causes (all fixture drift):
- Every Source turn now pays an origin refresh (9 items, 6064 bytes, 9 fuel, quote `original_return_refresh_turn_quote`), and the return table must exist before a 1-item business grant can claim (the old `admit_sized` birthed it inside a big-grant turn). Tests that never birthed the table saw `admit_sized` answer false forever (return budget "has not admitted its lifetime stock"). Stock items 64 and fuel 128 were also exhausted by the table birth (58 items) and refreshes.
- `born_bytes()` already counts the refresh bytes served per turn (746 `served` accounting), so `retire_source` double counted them via `receipt().retained_capacity_bytes`, and it compared against `born` before the retire scratch bytes were added.
- Core-wire tests funded the table-birth turn with `table_birth()` (no publisher in the quote); a wanted publisher makes the chain demand larger, so `grant_admits` answered Narrow forever.
- `sized_return_receipt...`: the old calibration (stock copy = minimal admitting stock for 1 byte, minus 1, plus 64) is meaningless now: the minimal stock is set by the refresh-gate slack, so admission is flat up to at least 32 KiB. Boundary now measured at 256 KiB (`grantBytes` 262144, refused row 262145 in `sized-case.json`).

Changes:
- `🧪️tests/🦀️.rs` (place tests): table birth first via `admit_return_table()`; stock items 4096, fuel 1024; `retire_source` compares released against post-retire `born_bytes()+extra`; extras use new `unborn_bytes()`; core-wire loops fund with `stock`; sized measurement parameterised by requested bytes.
- `sized-case.json`: `grantBytes` 64 -> 262144, refused `requested` 65 -> 262145, `remainingDuring` adjusted (schema unchanged).
- Fixture (test-only crate, not product): `🧪️testing/🎟️caller/🦀️.rs` gained `pub fn unborn_bytes()` (last turn capacity beyond what `born_bytes` already counts).
- Deleted `sized_return_publisher_failed_post_is_fenced`: its mechanism (publisher placement via `admit_original_escrow_publisher` posting through a loan receipt, injected with `set_loan_receipt_refusal`) no longer exists; admission now only requests the publisher and the refresh chain births it. The remaining fence (`post_publisher` in header begin/write) is unreachable without a new test hook and now has no test. Finding: requesting a publisher AFTER the table is Live makes the next chain fail with "original return birth lacks its accepted proof" (`birth_action`, Stage::Publisher requires a `table` proof that the chain only has when it births the table itself). Late publisher request is unsupported; owner should decide (product gap, not touched).

Open: `original_return_place_keeps_foreign_busy_cancelled_and_recorded_custody_under_one_bank` (line 13, `before-held` overflow). Measured: lifetime total fuel seen by the callback RISES across turns (16314 in the turn before the first claim, 16352 in the turn holding it, expected -3). Cause: the staged (not yet committed) change in `📬️outcome/🪙️budget/🦀️.rs` (`origin_prior.is_none()||self.renewable()` resets `spent`/`work` at every refresh when no claims are live) makes the return budget re-issue fuel (budget 16364 vs true lifetime 16323), and the Source parent persists the budget figure, so the table-birth/refresh work is refunded at the first renewal. This looks like a regression of that staged renewal (fuel leak), not fixture drift; a fixture-side clamp had no effect and was reverted. Not fixed here (product code, owned by the ingress-slot work).

## 2. retained-context lib

`actor_reserved_redemption_preserves_aggregate_receipt_and_repeated_fresh_borrow`: test bug, not product. After the last `close_step` the context stays `Some` with `identity == None` (dropped only by the next call), and the test's close loop called `next_input(context, ..)` which unwraps `identity`. Fixed in `🎟️context/🧪️tests/🦀️.rs`: the loop now derives the next input from the previous one plus `remaining_work/grant`. Passes. (Whole-lib run still aborts in another test.)

Not in scope but found: `tests::actor_source_birth_respects_actual_scoped_work_before_moving_its_parents` panics at tests line 287 (`owner.cancel.as_ref().unwrap()` is `None` after the first 1-fuel admission step; the cancel token is no longer present yet) and then aborts the process via the Drop assertion, which hides the remaining tests in a full `--lib` run. Pre-existing drift, untouched.

## Pass counts
- job-context `--features testkit --test original-source-return-place`: 8 passed, 1 failed (open, above) (9 tests after deletion).
- retained-context lib: the target test passes; `actor_return` 2/2, observer, retirement-binding, closure-error, actor_ingress all pass individually.

Product code changed: none. Test-support crate changed: `unborn_bytes()` in the job testing caller (additive).
