# Birth Chain Test Fixes

## Root causes
1. `birth_chain_quotes_equal_spend_and_a_grant_one_below_is_refused`, "original return birth lacks its accepted proof": the authorities-admission loop ran `advance` with fuel 64 and no publisher requested. The table-only chain costs less than 64, so the table was born there. The later turn then requested the publisher; the Publisher stage needs the table's accepted commit proof, which only exists inside the chain that births the table (Release clears it). The law (`birth-chain.json` fresh = 65 = origin, table and publisher in one atomic chain) requires the publisher to be requested before the loop. Test drift, not a driver bug.
2. `a_source_with_a_born_table_and_refreshed_origin_closes_terminal_empty`, "table never born": the test used `bare_turn`, which does not serve the resident birth driver. `admit_original_return_table` only requests birth; births run through the driver that `refreshed_turn`/`turn` serve. Test drift.
3. "fuel 65: refresh spend" left [3,0,0,0,0,0,4]: `StepContext::fuel_remaining` is `min(turn fuel, budget remaining)`. Another agent's `commit_origin` now resets the budget `spent`/`work` on a renewable refresh, so budget remaining jumps up during the origin commit and the fuel delta hid the commit's 2 Work. The refresh law ([3,2,0,0,0,0,4]) is right; the measurement was wrong. Stage spend is now measured with the monotone `original_work_units_spent`.

## Changed files (under `🧰️framework/🔨️modules/🧵️job/🛬️receiving/🎟️native-turn/`)
- `🧪️tests/birth-chain.rs`: loop calls `testkit::request_return_publisher`; second test uses `refreshed_turn`; spend measured with `original_work_units_spent`.
- `🌱️birth/🦀️.rs`: chain's per-stage meter `fuel` replaced by `done` (`original_work_units_spent`); `mark-fuel()` became `done()-mark`.
- Not changed: `clock_begin`, `birth-chain.json`.
- Sibling usages: `testkit::renew_turn` and `bare_turn` were left as is; only these two tests needed the change.

## Commands
- `repo cargo --store=shared --configuration=prototype --lang=en -- test -p semio-framework-job-native-turn --test birth-chain` (birthchain-1: 1 pass, 1 fail at refresh spend; birthchain-2 killed by session end; birthchain-3: 2 passed, 0 failed).
- `... test -p semio-framework-job-native-turn -p semio-framework-job-testing-caller --no-fail-fast` (birthchain-4): birth-chain 2 passed, 0 failed, both unit-test binaries 0 tests, no failures. The native-turn crate has only the birth-chain integration binary.
