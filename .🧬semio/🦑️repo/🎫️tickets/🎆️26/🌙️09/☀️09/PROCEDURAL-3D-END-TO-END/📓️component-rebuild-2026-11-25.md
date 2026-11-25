# Component Rebuild After the Escrow Slot Reclaim (2026-11-25)

Logs: `🗑️generated/component-rebuild/` (`run.txt` holds the real exit of `repo`). Scripts: `component-rebuild-run.sh`, `component-rebuild-wasm.sh`, `component-rebuild-law.sh`, `component-rebuild-dwg-laws.sh`.

| Step | Result |
|---|---|
| Laws `successive_` (owner-succession, component-owner-succession) | 2 passed (`succession-law.txt`) |
| `semio-hub-procedural-component:component` | exit 0 at 10:09:25 (first try failed on a missing peer `debug-ledger.rs`; a local stand-in fixed it) |
| `semio-framework-os-flow-browser-package:wasm` | run-7 exit 0 at 14:18:42; flow_core_bg.wasm 6,472,804 B, 7 `flow_bridge_*` exports in the d.ts. Earlier failures: graph inference crash (`reached.keys`, fixed by a peer), read-bytes budget on the reactor crate, SIGTERM by the memory warden (run-6) |
| `semio-s-plugin-flow-math-component:component` | exit 0 at 14:36:59 |
| `semio-hub-flow-component:component` | pass 1: final cdylib unit SIGKILLed (4.36 GiB, memory); pass 2 interrupted on request (exit 130). Not rebuilt |
| dwg write laws | not run (aborted twice for the one-compile rule) |
| brep component, shard `:support`, procedural rebuild | not started (ordered after the brep law and flow attach fix) |

Lesson: a status echo placed after `$(date)` reports the wrong exit; the scripts now record `$?` first.
