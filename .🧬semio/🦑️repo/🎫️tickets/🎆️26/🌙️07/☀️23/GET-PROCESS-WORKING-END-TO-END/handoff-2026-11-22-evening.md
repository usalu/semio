# Handoff 2026-11-22 Evening

## State at halt (21:xx)
- Ticket `26/07/23/GET-PROCESS-WORKING-END-TO-END` is still open (goal `🎯️r2602`). Do not reopen it; just continue.
- Fleet is up (11 machines). `repo test @semio-tech/process-js` failed after 2h10m. The resident service crashed while reading the graph slice: `BUILD_SOURCE_UNPARSEABLE`, missing `…/⚡️caching/🦀️cargo/🗃️unit-cache/🏗️buildscript/☁️offload/🧬️schema/🔣️.json` (the fleet work is in progress). `repo cargo` refuses multi-crate calls across workspaces. The process3d crates are not in the root workspace, so they need native cargo (`domain-cargo.sh` / `replay-cargo.sh`).
- Receiving floor and struct seed: see `receiving-floor-verification.md`.
  - The libraries of all five crates compile. The process3d editor and host pass `check --tests`.
  - Failures:
    - job: 8 tests (unrelated? no baseline yet)
    - value-derive: 2 tests
    - store-member-open-store: all 3 record-open tests
  - The value-native-decoding and pack-record-retained tests don't compile (stale signatures).
  - Open decision: the struct seed plan fails with "native partial template changed its original Work" across Source turns.
- History replay grant: see `history-replay-grant.md`.
  - `INTERACTIVE_COMPONENT` copy is now 512 KiB, derived from the largest indivisible step. The law passes.
  - The editor `history_edits_end_to_end` tests have not run yet.
  - A native build may still be running (log `🗑️generated/replay-hist.txt`, build dir `build-replay`).
- Browser:
  - Present: the jco output in `.cache/b/p/m/browser/prototype/modules/🏭️process` and the activation receipt (both from 17:05).
  - Vite was running on :6022 (pid in `🗑️generated/vite.pid`).
  - The last BiDi run failed at the stage-24 archive load (0 canvases). The hub wasm must be rebuilt after the framework fixes.

## Prompt for new worker

```
/goal Get process plugin working end to end.
Continue ticket 26/07/23/GET-PROCESS-WORKING-END-TO-END (already open, goal 🎯️r2602). Start by reading in its folder: handoff-2026-11-22-evening.md, status-2026-11-22.md, receiving-floor-verification.md, history-replay-grant.md, archive-load-analysis.md, native-rust-editor-tests.md (tail).
Remaining work, in order:
1. Decide and finish the archive-load stall fix. Either keep the struct-derive seed plan by letting the pinned Work rebind across Source turns, or drop the plan and rely on the receiving floor alone. Prefer the clean long-term option. Make all 3 record-open tests in semio-os-store-member-open-store pass.
2. Port the stale tests of semio-framework-value-native-decoding and semio-framework-pack-record-retained to the current signatures. Baseline and fix the 8 semio-framework-job failures and the 2 semio-framework-value-derive failures.
3. Run the process3d editor tests (package semio-s-artifact-process-process3d-editor, native cargo via the ticket's domain-cargo.sh / replay-cargo.sh, since it is not in the root workspace for repo cargo) and fix all remaining failures: history replay, reserved commit, workpiece hover, camera gesture, extensions replay, stl stock, add_step, HeapWitness. Run the host, extensions (metal, wood, robotic, concrete), bridge and process-js tests.
4. Rebuild the hub wasm, republish (jco) and activate (🗑️generated/publish-modules.ts, activate-native.ts). Serve the react dev route on :6022 and run verify-process-firefox-bidi.ts until the Process 3D app renders a canvas with no stage-24 fault.
5. Delete 🗑️generated (it includes the large native target), keep the reports and scripts, and close the ticket with a summary and every file touched.
At the same time, the shared work fleet mechanism is being developed in parallel. If it doesn't work, fall back to native tools. Never get stuck or hold your work. The machine has 16 GB RAM: run at most one or two cargo builds at once.
Use the main chat with Grok 4.7 256k High Non-Fast for main plan coordination. Use multiple Grok 4.7 256k Medium Non-Fast agents for medium to high complexity task execution. Use multiple Grok 4.7 256k Light Non-Fast Medium agents for read-only exploration, audits and low complexity tasks. Finish the goal as fast as you can (no fast LLM agent modes, only non-fast models) with all the resources you can get (agents, cpu, memory, etc). Never idle; always stay saturated. When you find fundamental issues unrelated to the task, fix them as well. Treat the goal completely end to end. Other agents are working on other logical parts. Even if they work on the same files, don't stop: keep working in conjunction with them and focus on your task.
```
