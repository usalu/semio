# Handoff: Procedural 3d End To End (2026-11-23 ~01:15 CET)

Supersedes the runtime state in [📓️wave-2026-11-22-night.md](./📓️wave-2026-11-22-night.md) and [📓️handoff-2026-11-22-evening.md](./📓️handoff-2026-11-22-evening.md). Ticket `26/09/09/PROCEDURAL-3D-END-TO-END` is still `open` (`ticket_reopen` refuses because it is already open; just keep working in it). `ticket_close` needs `📌️important/📝️.md`, which is still missing. Write it at close as a curated list of the key notes.

## State at handoff
- Ports `6018` (React) and `6118` (WGPU) were both down (`000`).
- The `repo run …:dev-generation3d-{react,wgpu}-prototype` launches v14 were SIGTERMed after ~3 h in "Reading the project graph slice … checking the file inventory … 0 of 600" ([react-v14.txt](./🗑️generated/react-v14.txt), [wgpu-v14.txt](./🗑️generated/wgpu-v14.txt)). The graph read never converges, and that is the critical blocker. Every lane therefore falls back to native tools.
- Lane audit of 2026-11-23: [📓️lane-state-audit-2026-11-23.md](./📓️lane-state-audit-2026-11-23.md).

## Lanes still running at handoff (these were not stopped; check their notes before restarting a lane)
| Lane | Expected result note | Restart only if the note is missing or partial |
| --- | --- | --- |
| Native React serve on 6018 (PDF-native pattern, `serve-generation3d-native.ts/.sh`, launch config `generation3d-react-native`) + `🐍️react-battery.mjs` reds | `📓️react-native-serve-2026-11-23.md` | yes |
| Native WGPU serve on 6118 (wasm-dev build, boot/frame-worker, staged plugin modules, `serve-generation3d-wgpu-native.sh`, launch config `generation3d-wgpu-native`) + `🐍️wgpu-battery.mjs` reds | `📓️wgpu-native-serve-2026-11-23.md` | yes |
| Root cause and fix of the graph-slice read stall (repo tool, with a test), then a confirming launch into `🗑️generated/react-v15.txt` | `📓️graph-slice-stall-2026-11-23.md` | yes |
| `semio-framework-job-context`: all targets green (binding decisions a/b/c in the evening handoff, lane 4) | section "2026-11-23" in `📓️job-context-completion-2026-11-22.md` | yes |
| Procedural crate tests (io-text, preview-eval, generation3d), retained-commands reds (`typed_input_admission_*`, SIGABRT in `original_native_param_patch_*`, E0432 `app::named_scene_surface`), then `world_host_unit_3` | `📓️procedural-and-retained-tests-2026-11-23.md` | yes |
| P5a law: 9 real Rust violations (expect/unwrap in frame-transaction, saturating_add, lock().expect, unbounded clear/measure_text/footer loop/clone/sibling walks per grant) | section "2026-11-23 p5a Rust fixes" in `📓️wgpu-frame-loop-fix.md` | yes |

Done on 2026-11-23: graph-slice stall root cause fixed. Owner discovery now records only the taxonomy contracts it consults (`🔎️owner-discovery/🟨️.mjs`, test `📜️contract-rows`). The inventory now takes about 1 min instead of 9000 s, but a full seal and target start are not yet confirmed; watch `🗑️generated/react-v15.txt`. Still open: rediscovery on target growth, single-threaded discovery, and the registry follow dropping the memo. See [📓️graph-slice-stall-2026-11-23.md](./📓️graph-slice-stall-2026-11-23.md). React native lane: 6018 is NOT serving, and the battery was not run. The native vite host starts but needs the generation3d activation receipt, which needs the staged `procedural` wasm component. That component can only be built through `repo` (script invocation envelope). The build run (door pid 96971, `semio-hub-procedural-component:component`) was waiting on the `.cache/b/d` lease, held exclusively for over 90 min by pid 17436 (`tools setup all`, another lane's). After it stages: activate, restart `serve-generation3d-native.sh`, run the battery. Source fix: new dev owner `✏️s/🧑‍💻dev/🎭️variants/🌀️procedural/` plus `devContribution` rows. Without it, any generation3d serve throws "Unknown playground module identity". The 3 generated catalog files were hand-edited, so regenerate them and compare. See [📓️react-native-serve-2026-11-23.md](./📓️react-native-serve-2026-11-23.md). P5a: all 9 Rust findings were fixed and the law test is green (1 pass), but only the `🖌️paint` crate was type-checked. The frame-transaction check was blocked by a peer's `[DEBUG]` line (unimported `NativeRecordKey` in `🌱️value/🌳️value/🔀️variant/🦀️.rs`) and by `wasmtime-wasi-io` failing to find its WIT directory. Rerun `p5a-rust-check.sh` once those compile, which also covers runtime-mailbox and Shell. No Rust unit tests were run. See "2026-11-23 p5a Rust fixes" in [📓️wgpu-frame-loop-fix.md](./📓️wgpu-frame-loop-fix.md). Also done: P5a law drift fixed (it read the current partition crates; mutation harness now counts only new failures against the baseline). See "2026-11-23 p5a" in [📓️wgpu-frame-loop-fix.md](./📓️wgpu-frame-loop-fix.md).

## Still open after those lanes
1. wgpu shell tests `shell-tests-input-3`, `shell-tests-docs-1/-2` (unresolved `operation_publications_from_json`, `take_media_actions`, `push_media_action`, `pinned_app_for`, `relay_open_plan`), `engine-attach-tests-2` (incl. `fit_graph_action_persists_…`), `frame-job`, `runtime-mailbox-tests`. Start these once the WGPU lane no longer compiles those crates.
2. wgpu renderer merge compile-check: `WORLD3D_POINTS_SHADER` import, `render_widget` missing, `BlockListRole` mismatch, `registry-and-node-graph` tuple mismatch (lines 210, 232), shell source-boundary overruns ([📓️replica-stale-overwrite.md](./📓️replica-stale-overwrite.md)).
3. Neighbour laws `interactivity-live-reconcile` and `interactivity-mounted-engine-surface-lifetime` register 0 bun tests and fail with their own drift (`per-surface-credit-cap made no source mutation`, `wrapping-cpu-generation did not bind live source`).
4. End-to-end browser verification of both prototypes: all examples, hover and selection, editor actions, no console errors. Then `📌️important/📝️.md` and `ticket_close` with every changed file.

## Job-context lane result (~03:45)
Whole-crate run `🗑️generated/jc-run12.txt` (`--no-fail-fast`): every target is green except `original-native-receiving-context`, which is 4/6. The two reds:
- `cancelled_source_recovers_actual_pending_child_with_same_construction_issuer`: the work delta is 4 where 3 is expected. Find where the recovery call debits the extra unit.
- `original_context_retirement_after_cancel_keeps_ledger_deadline_fuel_and_exact_heap`: the callback runs (`visited` true) for a case the fixture expects to be denied. Find the row in `🛬️receiving/🧪️tests/🧫️fixtures/🔣️.json`.
- Suspect the placement-posting change and the invocation wrapper on `close_step`, then do one plain whole-crate run.
- Open design item: rebinding a pending child's construction action to recover it in a later Source turn. The cancelled-source test was rewritten to a single turn.
See the "2026-11-23" section of [📓️job-context-completion-2026-11-22.md](./📓️job-context-completion-2026-11-22.md).

## WGPU lane result (~03:40)
6118 serves HTTP 200 (`screen -S g3dwgpu`). The renderer wasm, boot, renderer-boot and frame-worker bundles build natively, and the activation receipt is published. Boot still fails with `plugin-registry-invalid: selected module dependency graph`, because `🌀️procedural` is NOT staged: `.cache/b/p/m/browser/prototype/modules` holds only raster, shard and vendor. The battery has not run.
- Next: if `🌀️procedural` appears there, run `bun wgpu-native-activate.ts`, reload 6118, then run `bun 🐍️wgpu-battery.mjs`. Otherwise run `wgpu-native-components-build.sh` (about 25 GB free needed; the disk now has 56 GiB), then `bun wgpu-native-modules.ts`, then `wgpu-native-activate.ts`.
- The same staged component unblocks the React native serve on 6018.
- Many compile fixes were applied in the renderer engine (missing deps, imports, visibility, `MailboxRuntime` → `JobThreadTransfer`, and `naturalFileMediaInputV1` moved to `🛠️ShellHelpers/🎞️media-import`). Several look like regressions from the replica stale-overwrite between commits 751 and 752.
- See [📓️wgpu-native-serve-2026-11-23.md](./📓️wgpu-native-serve-2026-11-23.md).

## Disk blocker (03:03; resolved: 56 GiB free after the WGPU lane removed its own scratch)
The host data volume has 3.1 GiB free of 228 GiB. The compiler image `.cache/b/r/m` is at 0 bytes free (100 %). Every cargo build fails with "No space left on device". The procedural/retained tests lane ([📓️procedural-and-retained-tests-2026-11-23.md](./📓️procedural-and-retained-tests-2026-11-23.md)) ran no tests because of this. Reclaim space through the repo's own image-reclaim mechanism ([📓️rust-image-auto-reclaim.md](./📓️rust-image-auto-reclaim.md)) before any compile. Do not delete the shared unit cache by hand while others compile. Then run io-text, preview-eval and generation3d, then the retained-commands lib with output captured, then `world_host_unit_3`. `named_scene_surface` was merged into `scene_surface` by a peer, so the E0432 is gone. The retained-commands SIGABRT is a destructor assertion after an earlier panic; fix that first panic.

## Host discipline
- At most about 3 concurrently compiling lanes. Run cargo one package per call at `-j 1..2`. Wait while `memory_pressure` free is below 40 % or the 1-minute load is above 20.
- Never delete `🗑️generated`. Give `bun test` absolute paths (the wrapper refuses relative ones).
- The `semio` MCP server timed out on connect in this session. The `repo` MCP works (ticket tools, goals).
