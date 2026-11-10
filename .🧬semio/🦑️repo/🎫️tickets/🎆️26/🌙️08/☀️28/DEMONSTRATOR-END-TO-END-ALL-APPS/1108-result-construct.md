# 1108 construct result

## Step 1 (W-construct-200): table-birth allowance, clean option chosen
- No manifest change, no LOCK: job-context already depends on job-payload. Fixture caller (job/⏱️context/🧪️testing/🎟️caller) now auto-records the Source escrow table's born capacity (`OriginalReturnEscrowTable::birth_demand().1`) in the turn where the table first appears, via new testkit query `testkit::return_table_present(cx)` (one line in job-context 🦀️.rs testkit module, test-only). Removed the manual `record_born_capacity` (method + 3 callers: source/native tests x2, worker tests x1).
- W-construct-200 (action-bus lib): refused by peer mid-edit in `semio-framework-value-retirement` (OriginalRetirementCloseLoan missing, RetirementCursor close_step signature). Not mine; retry.
- Rerun: `"$C/1105b-tools/runtime/tst.sh" W-construct-NN "🧰️framework/🔨️modules/🎯️action-bus/📦️packages/🦀️rust/Cargo.toml" native -p semio-framework-action-bus --lib`
- Open: wire heap witnesses, CreditedOutput adoption in remaining loops, cancellation Ready/Failed, retire helper, retained-command wasm check.

## Step 2 (W-construct-201..204)
- value-retirement Box<dyn RetirementCursor> stale impl was fixed by declint (201->203). W-construct-203/204 now stop in `semio-framework-value-shared-issuance` (value/♻️retirement/🔗️arc-custody/🔒️sealed 🦀️.rs:59,90,108,109: `SharedOriginalBirth::publication` missing, fn arity 1 vs 0, `SharedOriginalStorage::duplicate` missing): peer mid-port, nobody claimed. My edits not yet compiled by cargo (they sit downstream of that crate).
- Unproven edits: testkit::return_table_present (job-context 🦀️.rs), fixture caller auto table_born, removed record_born_capacity + 3 callers.
- Next on green: rerun command above; expect the 5 `18160 != 12176` retire asserts to clear; then wire heap witnesses, adoption in remaining loops, cancellation Ready/Failed, retire helper, retained-command wasm check.
STATUS: BLOCKED shared-issuance arc-custody sealed compile errors (peer), W-construct-204

## Step 3 (W-construct-205/206)
- shared-issuance now compiles. W-construct-205/206 stop in value decode crates (foundation lane mid-port): `NativeReceiver::advance` missing (decode/owned/list 55), `crate::native_decoding` missing and `OriginalRetirementFailure` not at crate root (decode/owned/value/bulk 65, 118), construction 49/67/73/102 arity. Not mine.
- Prepared: CreditedOutput adoption is already in source()/drive()/close() of admission tests; remaining items wait for a compiling tree.
- Rerun: same command, label W-construct-NN (next 207).
STATUS: BLOCKED value decode crates (foundation) W-construct-206

## Step 4 (W-construct-207/208)
- 207: `semio-framework-value` errors (admit_shared_retirement import, RetainedCloneCursor::advance / RetirementCursor::close_step no longer trait members). 208: refused before compiler, `semio-framework-value` Source budget exceeded at value/📝️text/🚶️utf8/🦀️.rs (foundation's file; budget must not be raised). Action-bus tests unrun: still 13/32.
- Rerun: label W-construct-209+.
STATUS: BLOCKED semio-framework-value Source budget (foundation), W-construct-208

## Step 5 (W-construct-209..211): action-bus lib FIXED-verified
- 2 errors on the 733 contract fixed: registered `RegisteredSourceRetirement` now implements `close_into` (inherent `close_registered` holds the old body); wire original `step_granted(body, effect)`. Native W-construct-210 rc 0, wasm W-construct-211 rc 0.
- Next: tests (W-construct-212).

## Step 6 (W-construct-212/213): action-bus tests compile and run: 13 passed / 19 failed
- Test-code ports to the 733 contract (all compile): `step`/`step_granted` take `body` (= `effect.grant().maximum_copy_bytes`) at 6 sites (admission tests x4, registered tests, wire original tests); unit-test Job builder close goes through `cx.with_original_effect(g,|_,effect|owner.close_step(effect))`; removed action-bus's own `#[global_allocator]` (job testing-caller now links the testing allocator).
- Remaining 19 failures are the same root cause as before, not an accounting bug in the allocation: 12x "source/identity/multi-turn construction must settle", wire `original effect producer requires transfer or paid retirement`, plus the dependent asserts (`admitted && output.is_some()`, `settled`, Option::unwrap None, 40 vs 0, Pending vs Complete). Per 1108-result-birth.md: "until the driver exists, `cx.admit_original_return_table()` returns Ok(false) forever: every test/fixture that needs a table is red", and 1108-result-driver.md: `OriginalJobReturnBirth` exists but cannot run (no sourced-Work bootstrap: 'original Source clock is absent'; hosts do not call `advance`). My table-birth allowance (auto born capacity in the fixture caller) stays correct and waits for a table that can be born.
- Not fabricated: wiring a birth chain into the fixture/test callers or hosts is the driver lane's open platform decision (who owns the first Source-aware Work effect). Once `admit_original_return_table` can return true, rerun W-construct-214 (same command) and work the remaining list (cancellation Ready vs Failed, retire helper, wire heap witnesses, retained-command wasm check).
STATUS: BLOCKED return-table birth driver (sourced-Work bootstrap, driver/birth lanes), W-construct-213

## Step 7 (W-construct-214, after driver wiring): 13 passed / 19 failed, new first cause
- 12 failures: `original Source clock decision stayed pending` (OriginalTurnFailure InvariantViolated/refused) raised by `OriginalJobReturnBirth::clock_begin` (job/🛬️receiving/🎟️native-turn/🌱️birth/🦀️.rs): it polls `poll_recorded_clock_decision` 4 times and turns a still-Pending decision into a hard error in the fixture turn, instead of yielding to the next turn. 1 failure: `pending decoder requires original recovery`. 1: `original construction action retains an unsettled reply` (value construction 21:61, original_command_wire_law).
- Remaining accounting asserts unchanged in kind: wire heap `(14136,192)` vs `(13944,0)` (192 B allocated+released not posted), `40 vs 0`, Pending vs Complete (4104 B released not completed).
- Not mine to patch: native-turn birth is driver's. Needed from driver: clock decision Pending must resume next turn (return Idle), or the fixture caller must admit a clock that decides within 4 polls (action-bus fixture uses `clock()->Some(1)`, deadline 100000, fuel 100000).
- Rerun: W-construct-215 (same command).
STATUS: BLOCKED native-turn birth clock decision stays Pending (driver), W-construct-214

## Step 8 (W-construct-215, after driver clock-resume fix): still 13 passed / 19 failed, new first cause
- 12 failures: `original clock Source or action changed` (generic `invalid()` of job/🕰️clock-identity-and-budget/⏱️recorded/🦀️.rs). Hypothesis: the clock decision now resumed on the next turn is polled under a different Source turn / action identity than the one that started it (the recorded reading/decision is owned per Source turn), so the identity check rejects it. Owner: driver (OriginalJobReturnBirth::clock_begin/clock_end cross-turn resume), not action-bus.
- 1 `pending decoder requires original recovery`, 1 `original construction action retains an unsettled reply` (original_command_wire_law), wire heap (14136,192) vs (13944,0) x3, `40 vs 0`, Pending vs Complete: unchanged.
- Rerun: W-construct-216.
STATUS: BLOCKED native-turn birth clock decision identity across turns (driver), W-construct-215

## Step 9 (W-construct-216, after birth-chain law): 13 passed / 19 failed
- Clock errors are gone. Causes now: 12x construct loops exhaust 16 turns ("source/identity construction must settle" at admission tests 17:583 / unit tests 47:507), `admitted && output.is_some()` (79), `settled` (107), `Option::unwrap None` (87, 95: no table/output after the loop), plus unchanged: wire heap (14136,192) vs (13944,0) x3, `40 vs 0`, Pending vs Complete (4104 B released), `pending decoder requires original recovery` x1, `unsettled reply` (original_command_wire_law).
- Matches the earlier root cause in 1107-result-runtime2.md (settled helper's first turn: admit_scope consumes the turn's whole remaining fuel, scope only ready on a later call, later turns see fuel 0 / retirement_should_yield). That is driver2's "construction extent yield" fault. Reported via claims line; action-bus code unchanged.
- Rerun: W-construct-217 after driver2.
STATUS: BLOCKED construction extent yield / closing a born table (driver2), W-construct-216
