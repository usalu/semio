# 🤝️ Handoff: dev wfc for all artifact editors (2026-11-19, ~23:20)

## Goal (unchanged)
Get `repo run @semio-tech/framework-os-dev:dev-<variant>-react-prototype --parallel=1 --lang=en` working end to end for bitmap (6041), grid2d (6042), wfc2d (6043), grid3d (6044) and wfc3d (6045). For each one: it serves, Load Example works, the editor renders, and Solve publishes a preview and finishes. Each editor's Rust and TS tests must also pass. Fix fundamental issues met along the way.

## Hard rules (repo `CLAUDE.md` plus lessons learned today)
- Use the `repo` binary. Plain `cargo` is refused by the gate. Run `SEMIO_OFFLOAD=never repo cargo --store=shared --configuration=prototype --parallelism=2 -- …`.
  - **`SEMIO_OFFLOAD=never` is mandatory.** The work fleet hangs cargo runs ("flow to the fleet").
  - **`--store=shared` is mandatory.** Isolated stores (12–16 GiB each) filled the 67 GiB compiler image.
- No git commands that change anything, no worktrees, don't edit AGENTS.md, and no `repo dashboard`.
- Clean root fixes only; never weaken tests. Docstrings start with an emoji, and temporary logs use the `[DEBUG] ` prefix (not `[PAGEDBG]` or `[bitmap-diag]`).
- Before queueing a long run, check that the package and filter match at least one test. Wrong packages and filters cost hours today (0-test runs).
- After stopping a run, confirm its process is gone (`lsof -t <log>`, `ps -p`). Six orphaned runs blocked the memory queue today.
- Component build: `native-component.sh` (it retries on rc=75 from peer edits), then `🗑️generated/last-mile.sh` and a restart of the servers. It takes about 25 min plus queue time.
- Logs go in `🗑️generated/`. Record each fix in `fundamental-fixes.md`.

## Live status
- **Serving:** all 5 serve (HTTP 200 on 6041–6045).
- **Before the 20:10 builds:** Load Example → render worked on bitmap, grid2d, wfc2d and grid3d. grid3d's Preview is empty until a solve, by design. The wfc3d Preview had an RGBA colour bug, now fixed in source.
- **Regression since the 20:10 builds:** bitmap Load Example fails with `original posted publication receipt lacks its unique family or actual recipient binding` (`🔌️plugin/⚛️reactor/🔄️turn/🩹️reconcile-drive-and-publication-receipts/🦀️.rs:~160`).
  - **Debug line:** I added a temporary `[DEBUG] receipt-refusal …` eprintln there, and the component with it built rc=0 at about 23:10. Next: load bitmap and read that line via `read_console_messages` with the pattern `receipt-refusal`. It names which condition fails: unique family, recipient, or `before+posted==retained`.
  - **Prime suspect:** the earlier paid `copied_items:1` units in the worker authority. The teardown agent has since replaced them with `consume_fuel(1)` (22:48), which posts no retained progress, so the next component rebuild may fix it.
  - **Cleanup:** remove the debug line afterwards.
- **Solve:** not yet verified on any variant.

## Test status (latest)
| Editor | Last full run | Notes |
|---|---|---|
| bitmap | test22: 263/22 | The rest are the drive stall, teardown and brush settle. Test23 is held for go-ahead. The brush settle guard is applied, unverified. |
| grid2d | test-01: 240/24 | Store replay fixed. The frontier test passes after `admit_sqlite_values`. |
| wfc2d | build26: 266/21 | The drag settle guard is applied, unverified. Awaiting go-ahead for a full run. |
| grid3d | final13: 244/21 | 16 teardown, 2 history, 3 fill/preview stalls. |
| wfc3d | test28: 327/8 | Teardown 0. Test30 is running (settle guard, contradiction rule, store-retire probe). |
| window-config apply | apply-fix31: 5/6 | The paging test ties at 90 vs 90. `apply-pagedbg32` is queued to decide whether the tight grant isn't tight, or whether to assert on text-page turns with a strict `tight>large`. |
| animate | presentation tests compile (test8-norun) | `animate-test9.log` runs the full presentation tests, queued. After that, run the full workspace tests. |
| headless (`semio-framework-job-headless`) | t11: 7/7 | Verified the fuel-visibility fix. |

## Root causes fixed today (see `fundamental-fixes.md`)
- **Load Example speed:** a 25 ms host sleep between turns (now ready-turn re-entry plus back-off), `step_slice` for manifest receiving, and the event-batch cut at patch-producing events.
- **Headless teardown and drive "advanced but Blocked":** the caller's fuel read hid spent work (`original_turn_fuel_remaining`). Verified 7/7.
- **History edits:**
  - a native-work handoff unit (`with_allocated_effect_reported` / `with_effect_reported`);
  - the hydration fold-close ladder;
  - harness identity lender `CallerIdentityWork` (unverified).
- **SQLite decode cancel:** replay queued events. Plus grid2d's `admit_sqlite_values`.
- **Build infrastructure:**
  - the component-pass failure-diagnosis and unit-killed retry;
  - the EPIPE guard;
  - the wit-bindgen OUT_DIR lane neutralisation;
  - the host-capacity lock EINVAL;
  - the lease claim ENOENT;
  - cache quota capped at image size, and the component-run collector (freed 17 GiB).
- **Peer half-edits repaired:**
  - `close_step_unpaid`;
  - `lend_reason`;
  - instances cleanup-turns abi log → eprintln;
  - ops ingress imports;
  - UI re-export of removed classes.

## Open work (in priority order)
1. **Bitmap Load Example:** read the `receipt-refusal` line, fix it, then do one combined component rebuild that includes the fuel fix. Then verify Load Example → render → Solve on all 5.
2. **Drive-phase stock exhaustion** (rooms-16 runs out of its 512 MiB copy stock while the job copies only about 2 MB). Re-test after the fuel fix (`bitmap-f1.log`, queued). If it persists, find where about 16 KiB per step is charged outside the job step.
3. **Give-the-go list:**
   - bitmap test23;
   - wfc2d full run;
   - grid2d, grid3d and wfc3d reruns after the fixes land;
   - history `chain4.sh` / `chain5.sh` runs: wfc3d, grid3d, grid2d, then bitmap `history_edits_end_to_end`.
4. **`semio-plugin-tests-05-2` doesn't compile** (45 errors, facade host lost the turn crate's items). An agent is fixing it, which is needed to verify the receipt-collect change.
5. **Window-config:** settle the paging test, then run the full suite (`window_genesis`, `window_pair_receiving_outcomes`).
6. **Animate:** finish the test port, then run the full workspace tests.
7. **Cleanup of temporary debug code:**
   - `[DEBUG] receipt-refusal` (mine);
   - `[DEBUG] pagedbg` (window-config apply test);
   - the wfc3d store-retire probe;
   - peers' `stall5d`, `lane5d`, `birth mismatch` and the `gate ingress` per-turn flood. Coordinate before deleting peer lines.
8. **Follow-ups:**
   - a protocol-level "advanced vs blocked" signal;
   - the typed-operation publication `EntityIdentityAuthority::resume` with no lender;
   - about 21 `with_allocated_effect` callers that under-report one handoff unit;
   - the churn refusals test referencing a missing graph message;
   - `🪟️native-memory` TS errors (another agent's);
   - the fleet offload hang (fall back locally when there's no progress);
   - the gc "Would remove N" line ignoring owner reclaim;
   - lease volume sizing;
   - the metadata phase enum.
9. **Close the ticket:** `repo ticket close` once all 5 variants are verified and the tests pass.

## Background agents and runs that were in flight at handoff (they keep running)
Fix agents (Sonnet), in this session:
- Load Example runtime;
- headless teardown and fuel (also owns drive and stock);
- history edits;
- bitmap, wfc2d, grid2d, grid3d and wfc3d editors;
- window-config;
- animate;
- tests-05-2 compile.

The new session cannot message them. Read their logs in `🗑️generated/`, and check `ps` / `lsof` before starting duplicate runs.

## Continuation after 23:30

The earlier five servers exited on SIGTERM. The generated component dated 23:04 did contain the diagnostic, but the served bitmap activation was from 21:30. Re-preparing and activating the diagnostic component on port 6041 reproduced auto-load failure: `families=[] receiving_family=None recipient_ok=true`, with one directly charged item and 112 bytes. See line 2350 of `🗑️generated/[DEBUG] bitmap-receipt-2.log`. The producer now attaches already-paid progress only to an emitted producer receipt. The debug print is removed. The constructor/receipt source contract passed 1/1.

At 23:49 a combined component build was running (`component.log`), with a watcher set to prepare/activate all five once the build succeeds. Five canonical `repo run` invocations are waiting for graph loading or machine capacity; their logs are named `[DEBUG] canonical-serve-<variant>.log`. Use ps and lsof before starting a replacement. HTTP and Solve verification on the fixed build are pending.

The shared TS tests passed 183/183, grid2d 27/27 and wfc2d 117/117. The package provider is being fixed to bind the package before running public-export tests; two concurrent grid2d package runs passed. The missing actor and UI contract bindings were regenerated. The manifest exporter and the three affected package reruns are in progress. Native Rust workspace runs have resumed; consult their logs for results newer than this note.

## After midnight on November 20

All five TypeScript package suites passed. Bitmap had 30 tests, grid2d 27, wfc2d 117, grid3d 5, and wfc3d 3. The separate WFC examples/schema suite passed 183. See `typescript-test-audit.md` for commands and per-suite logs. The native grid2d and grid3d Rust runs passed 262 and 263 tests respectively and each exposed two history teardown failures. Focused diagnosis and other full-suite runs continue.

Four queued public serve processes were stopped and their exits confirmed while the grid2d public command waited for host capacity. Its PID was 40204. Restart the other canonical targets after the graph can run. The new combined component, serial prepare/activate, five native servers, and serial real-browser checks have been scheduled. Use the `[DEBUG] latest-activate-*.log`, `[DEBUG] serve-fixed-*.log`, and `[DEBUG] fixed-*-browser.log` evidence before classifying browser outcomes. Prior successful HTTP status or baseline editor activations do not establish Solve.

At 00:19 the first fixed browser component reached the published module and was prepared, activated, and served for all five variants. Its final source freshness check requested a rebuild because a peer had changed the headless caller after source capture. Actual browser runs on the published component are recorded in `[DEBUG] fixed-<variant>-browser.log`. Bitmap, grid2d and grid3d failed on a second receipt issue during Load Example. Wfc2d and wfc3d panicked while dropping live construction authority during the same physical receipt refusal. Wasm breakpoint evidence identified the original constructor and the next Window store call. The job constructor now settles its physical scope on the originating turn. The subsequent combined build, the focused constructor test and the Rust suite work are running; the newer browser build still needs verification. Native port 6042 was freed for the canonical process.

At 00:49 all five native servers returned HTTP 200 on their assigned ports. The exact public grid2d command cleared its memory wait and is checking the graph inventory. The temporary native process is serving 6042 until the canonical target reaches serve. The rebuild includes immediate physical receipt settlement and the retained producer candidate fix. A local watcher checks for a new wasm digest, runs prepare and activate for each variant, then drives the browser on that port. Its logs use the prefix `[DEBUG] settled-`. The previous receipt starts with `7cd52328`. The UI smoke selects an example and records the Solve control, windows, surfaces, visible status and final screenshot. Inspect the resulting preview and completion before marking any editor verified.

At 01:15 the b26221c6 browser build had cleared the first trap and refusal. All five still failed to load. The independent reserved-return payment described in fundamental-fixes.md was incorrectly charged to the worker's semantic receipt. A fresh combined build and five real UI checks are scheduled under the log prefix `[DEBUG] reserved-`. The native window-config paging target passed all six tests. The five TypeScript package scripts passed 182 tests in total. A full wfc2d native run passed 280 tests and failed seven; the native editor owners are following through on the remaining failures.

Component 4211a226 got past the b26221c6 receipt error in all five browsers. Initial Load Example then hit another error before any window or Solve. At 01:50 physical accounting and the caller PC showed the original return claim was attempting its 242-byte row after consuming the last item of its phase loan. The repaired source is rebuilding. Use `[DEBUG] claim-browser-*.log`, `[DEBUG] claim-fault-bitmap.log`, and `[DEBUG] acknowledged-row-quantum.log` for its results. A transient stale repo schema interrupted several queued Rust tests before Cargo started. They were resubmitted from the coherent checkout after the exited PIDs/log writers were confirmed gone. Bitmap's native solver now reached a verdict for every example in 2.89 seconds.

At 02:30 local time fdef25ae had reached typed page acknowledgement in bitmap, wfc2d and wfc3d, which exposed the string shell endpoint. The browser passed that point after the TypeScript correction but then refused retirement. Grid2d separately exposed its missing disposer demand. Both grid sources and the shared typed-operation retirement window were fixed. A combined build and automatic prepare, activate and browser checks are in progress under `[DEBUG] gridfix-`. See fundamental-fixes.md. The shared native headless suite passed all seven cases, and the actor wire suite passed all 34.

At 03:00 the one-item return-claim regression and the reserved-return publication regression each passed their focused Rust run. The renderer accepted a numeric shell acknowledgement in its focused host lifecycle test, and all 13 diagnostic typed-page tests passed. The live fdef25ae component still stopped before the editor windows. The combined build now compiles the grid window disposer quotes, staged follower fields and isolated granted retirement turn. The five public commands restarted after the shared cargo lease import was repaired. Grid2d has reached graph file inventory. The five native fallback listeners continue to serve during that work.

The 794c59dd build reached bitmap epoch 8963 and all five browsers advanced past the prior receipt problem. Each still refused the initial example before rendering. The actual retained failure in bitmap and grid2d was “runtime live cleanup faulted for instance 1: the cleanup pump made no progress for its whole stall credit”. Three independent Chromium runs read the actual last maintenance stage as 104 (ToolOverlay) on 6041, 6042 and 6045. This matches the focused wfc3d drag result. That test reached a one-byte Window preview epoch transition. The responsible owner replaced construction of those scalar transitions with directly paid writes. The component retry is compiling with that edit. Physical constructor accounting and its neutral TypeScript fixture now each pass the focused regression. See `[DEBUG] cleanup-stage-6041.log`, `[DEBUG] cleanup-stage-6042.log`, `[DEBUG] cleanup-stage-6045.log`, and `[DEBUG] constructor-fresh-scope.log`.

At 03:44 CET the browser debugger confirmed 16 repeated attempts at the same indexed epoch partition in bitmap and grid2d. Mounted maintenance had lent the follower one item for the entire operation, including its receiving scope setup. The active original follower now borrows the remaining mounted turn and reports its physical delta after accounting for independently reserved returns. The window epoch also advances scalar ordinal and ready flags directly. The component build wpALkx5 is capturing these repairs. The previous captured build (wxe3EDo) failed on an earlier incomplete tool-machine dependency and missing DagHoverFacts.edge retirement, both fixed in the live source. The five canonical commands and fallback listeners remain active. The browser harness now requires both a changed rendered preview and the Tool runs terminal status.

Component wsLKllq started at 04:02 with the new receiving API and its callers present before capture. It is still compiling as of 04:20. The active checker waits for a replacement of the 794c59dd6ec0 browser receipt. Its final evidence requires a newly started Tool Run to reach Finalized and a visible preview canvas to change. The focused Window epoch test now selects the actual package manifest in the repository workspace and is waiting for host admission.

At 04:42 wsLKllq reached 936 compiled units and failed in Grid 2D and Grid 3D against an older ViewModel. Their source called contextual_utility, which the current ViewModel now defines. Provenance also confirmed a concurrent edit in WGPU draw scalar encoding. The driver returned 75 and the existing component script immediately began another attempt. The watched browser receipt is still 794c59dd6ec0.

The warmed retry wYsyk6X has compiled 399 units and reused 478 as of 05:06 CET. It compiled the matching ViewModel, Document Media and Instance Ladders units. Three Rust owners resumed at 05:02 and confirmed their existing runs before continuing. On macOS the browser checker now uses the Chromium rendering flags and Tool runs tab selector from the repository's Tool Run matrix. It still waits on the new WFC component receipt.

The WFC 3D focused native run reached all nine drag laws. Seven passed. Both mounted cases moved past the ToolOverlay phase 104 stall. Their new failures were a missing history row and an action fixture dispatch bound. Grid 2D completed all eight packages with 260 passes and four failures in grid2d-v1. The earlier missing replay issuer is repaired. History now reaches the wrong operation wire or the Report input loan. The artifact owners are tracing those new outcomes.

## November 20 afternoon

Component run `wIRDTOG` linked and staged digest `abe27718cd8f`. Native fallback servers on all five assigned ports returned HTTP 200 and Chromium mounted each default editor with canvases. Every alternate Load Example stalled. A separate bitmap run also stalled on the shell history command ahead of Solve. The browser capture shows one command page followed by 9,031 continuations without a terminal acknowledgement. The guest named command ingress as its only remaining work. Pausing inside the guest showed it advancing slowly through surface refresh and displaced-view retirement. See [browser-native-recovery.md](browser-native-recovery.md).

The component rebuild `w8p13di` is compiling the paid refresh batching under `[DEBUG] batched-surface-component.log`. A watcher will prepare and activate the new receipt and run the browser matrix. The graph cache now seals the stable union of exact observations, with 45 focused tests passing. All five exact public commands are waiting inside the resident checkout graph service. See [canonical-graph-publication.md](canonical-graph-publication.md).

The native Rust roster runs all named members for each editor. The focused Window fixture and WFC2D ordinal codec checks use `[DEBUG] native-window-original-current.log` and `[DEBUG] wfc2d-authored-ordinal-receiving.log`. The five TypeScript suites already passed 182 tests. Browser Solve and the current full Rust test matrix still need passing evidence before ticket closure.

The first batched-refresh component build exited 75. Its typed command called `ArtifactPreparedOperationCursor::next_depth_demand`, which the captured Store dependency lacked and the current checkout defines. The rerun in `[DEBUG] batched-surface-component-r2.log` passed fresh source admission. The existing browser watcher will run the matrix as soon as the new artifact stages.

The current Grid 2D roster passed 261 of 264 tests. One inference test stalled while receiving a mutation. The ordinary history edit stayed in Report replay. The conflict edit's close stayed on Document Archive, repeatedly quoting 1,872 capacity bytes at depth 5. The Report caller now supplies its actual available Work. Archive close now releases the buffer of an unacknowledged terminal fault under the caller's release grant. The next close assertion reports the effective physical grant and remaining fuel. The focused rerun of both history laws is queued under `[DEBUG] editor-rust-grid2d-history-r2.log`.

The second batched-refresh component staged digest `13777a1fc8c0`. Bitmap's alternate Load Example sent `setActiveExample` with `flowers-24`, then `LoadDocumentArchive` and its polls. Its load reached 113/113 and the input pixels changed. The picker still showed Rooms 16. Bitmap's direct Solve reached `toolRunStart` and trapped on a live original alias. The Wasm stack identified the default retargetable job builder dropping a temporary `ToolRunJobRequest` that owned a trace-key alias. The run now owns its atomic counter directly and both job builders borrow it. WFC 3D's alternate load exhausted the actor's copy stock. The actor retained turn now keeps one original Source and its return origin while binding each incoming shell epoch separately. R3 includes both repairs and is running under `[DEBUG] wfc-browser-r3-component.log`. The browser watcher will activate the new receipt and repeat the matrix.

The focused native Window saved-load law compiled and failed before replay. Its loader quoted a contiguous 91,536-byte copy under the same 32 KiB copy ceiling used by the mounted owner policy. See `[DEBUG] native-window-original-current.log`. The five exact public commands still wait inside the resident graph service and the native fallback listeners remain available on 6041–6045.
