# L28 board fill admission (unverified, source-only)

Root cause: `BoardFillJob::admit_original` required `snapshot.is_some() && output.is_none()` on every call. `admit_original_construction_settled` (framework job construction) documents `Ok(false)` as "continue next turn with the same admission and output": the constructor may already have run (snapshot taken, output set) while settlement is pending. The re-entry then failed with "board fill admission requires its original snapshot and empty output". Yesterday it settled in one turn, so no re-entry occurred; the grant/fuel dependent turn split made it visible.

Fix (framework, correct invariant is "exactly one of snapshot/output is held"):
- 🧰️framework/.../♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🪣️fill/🦀️.rs: guard now `snapshot.is_some() == output.is_some()`; message updated.
- Puzzle 2D editor caller ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/.../✏️editor/⏳️precompute/🪣️fill/🏃️run-job-stepping-and-finalize/🦀️.rs: the re-entry gates skipped admission once output/ingress was Some, which would strand a pending settlement; now they also re-enter while `!construction.terminal_is_empty()` (job admission and snapshot ingress capture).
Test setups (line 362 test, puzzle fill-fixtures) already loop until true; unchanged.
Not built or run.

## Follow-up: caller ledger assertion (l23-lasso2.log)

The panic is at `🎟️caller/🦀️.rs:13:226`, inside `OriginalJobFixtureCaller::admit` (test line 355, `close_source::caller(1,1)`), i.e. before any fill admission runs. It asserts that the CancelToken root receipt `(retained_capacity_bytes, released_bytes)` equals the thread-local allocator observation around `CancelToken::admit_root`. Receipt `(64,0)` is correct; the observation read `(234076,132740)`, which is a single realloc 132740 -> 234076 bytes on the test thread inside `admit_root`, which only does one 64-byte `Arc::try_new_uninit`. The same log shows `[DEBUG] big alloc 64/0` plus an unresolved backtrace printed on this test's stdout: a temporary allocator debug probe (not in source now; no `big alloc` remains under 🧰️framework or ✏️s) was active in that build, and its own printing/backtrace allocations (libtest capture buffer growth) landed inside the observed scope. Fill admission, construction retirement and turn release are not involved: the assertion fires before `admit_original` is called, and the earlier run (before the l28 fix) passed this line.

Conclusion: no source defect found; no change made for this. Rerun without the debug probe; if it still fails at line 13, the thing to inspect is any allocation hook on the test thread, not BoardFillJob. Unverified (no builds).
