STATUS: CHECKPOINT. All edits done and compile-atomic; NOT yet compile-verified (first check hit ENOSPC in repo cargo headroom lock, rc=1; rerun also rc=1 for fw, energy check pending/failed to start).

# 1029 policysweep result

| # (policy29) | item | outcome |
|---|---|---|
| 1-5 | text-splice native tests `println!("[DEBUG] ...")` x5 | fixed: removed, dropped now-unused `index`/`ready` bindings |
| 6 | tool-machine typing edit `owners.key/request .unwrap()` (demands) | fixed: `ok_or_else(invalid)?` |
| 7 | tool-machine transfer `checked_sub(1).expect(..)` | fixed: computed up front as `live` with `InvariantViolated` ValueError, before the slot is mutated |
| 8 | energy inspector `#![allow(clippy::too_many_arguments)]` | fixed: no fn has >7 args; allow removed |
| 9-11 | energy inspector 3x `.expect("... arm parsed ...")` | fixed: `ok_or_else(|| invalid_value(property, value))?` |
| 12 | energy editor commands allow | fixed: removed `async_fn_in_trait` + `too_many_arguments` (no async fn / long signature in file) |
| (task) | energy mutations aggregate `#![allow(long_running_const_eval)]` | removed as an experiment; the lint is generated for 540+ aggregates by `derive(Mutations)`; if the check shows the const-eval error it is re-added and listed under SKIPPED |
| 13-14 | puzzle task-resume `[DEBUG]` panic/format | fixed: `.expect("the registered typed operation settles")` |
| 15-17 | puzzle action-producer `eprintln!("[DEBUG]..")` | already gone |
| 18 | render staging tool-machine `original_typing_settlement.as_mut().unwrap()` | fixed with `let Some(..) else { return Err(InvariantViolated) }`; same for the twin `original_press_settlement` |
| 19-20 | typed-operations typing original unwraps | fixed (`window`, `row`, `owners`, `row.take()`, `verb`) via `ok_or_else(invalid)?` |
| 21-22 | typed/runtime `/// [DEBUG]` doc | already gone |
| 23-24 | op-binary `eprintln!("[DEBUG]")` | already gone |
| 25 | ownership producer `state.take().unwrap()` | fixed: typed InvariantViolated ValueError |

Extra [DEBUG] removals (files older than 20 min, modified 10/29): pack value decode `receiving` (3 eprintln in non-test code), fem2d remove-selection probe test + log_phase, tests/helpers in energy snapshot, hub stdio shipped-fleet, pack-json receiving test, typing-edit and typed-ops original tests, job emission host test (prefix dropped from panic/expect messages or print removed).

SKIPPED (active, modified < 20 min): `pack/🌱️value/🛬️decode/📜️record-body-cursor/🦀️.rs:480` `eprintln!("[DEBUG] body close ...")` (peer editing).

Not touched: remaining pre-existing unwraps in the same files (tool-machine typing edit lines 38/76-100, typed-ops emit original etc.), ~540 other `long_running_const_eval` allows.

Tools: `1029-tools/policysweep/{strip_debug.py,crates.py,check.sh}`. Outputs: `🗑️generated/1029/policysweep-*`.

## Next step (exact)
`bash C/1029-tools/policysweep/check.sh` (detached, writes `C/🗑️generated/1029/policysweep-{fw,energy}.{txt,rc}`). Check disk (>40 GiB free) first. Read the errors in `policysweep-*.txt` (the previous failure was the repo CLI crashing on ENOSPC, not a compiler error; verify the second run's txt for the real cause).
If the energy check shows a `long_running_const_eval` error, re-add `#![allow(long_running_const_eval)]` to the energy mutations aggregate (`.../🧬️schema/🧬️mutations/🦀️.rs`) and list it as SKIPPED (needs derive(Mutations) redesign).
