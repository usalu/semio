STATUS: DONE. semio-framework-plugin-extension GREEN (wasm32-wasip2 --lib, hub manifest, `C/🗑️generated/1101/plugext-1.rc`=0). Gate `plugext-gate-1` rc=1 with exactly one red crate, plugjob's plugin-runtime-ops (1 error); nothing else red. Log: `C/🗑️generated/1101/plugext-gate-1.txt`. `framework-green` not written.

## Edits (plugin/💡️inference)
- 🫴️receiving: `close_original_retained_step` closures now return `RetainedWorkOutcome` (`owner.step(grant)` wrapped with `owner.last_turn_work()`; copy closure `work: 1`).
- 🚪️gateway: `close_factory_ticket(..)` and `admit_owned_retirement(..)` consumed via `.result` (4 + 1 sites).
