# P10bg Owned Diagram Force Second Repair

## Verdict

**AUDIT-READY, NOT ACCEPTED.** This implementation pass repairs the P10be blockers in the owned Diagram force boundary and preserves the earlier P10bc setup/tick/projection/drag repairs. The focused, full UI, renderer, type, lint, primitive, format, frozen-lock, dependency, parity, and source-pattern gates recorded below are green after the final production-source change.

The provisional removal of `d3-force` and `d3-quadtree` remains **REJECTED** at **141 total dependencies / 78 JavaScript + 63 Rust** until another independent Terra audit inspects the final source and reruns its own focused/full suites. This report is implementation evidence and does not grant acceptance.

No Dagre replacement work, Cargo command, modifying Git command, ticket lifecycle mutation, framework Layout/shared Rust edit, manifest edit, or lockfile repair was performed in this pass.

## Repair-Owned Files

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️component.tsx` — owned force runtime, live projection/virtualization, handoff queue, pointer boundary, and local DOM query.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🧪️component.test.tsx` — 15 focused force/live-host tests.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` — the directly related in-package force test now drives bounded `step({ deadline, fuel })`; it no longer calls removed `tick()`.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/OWNED-UI-AND-TOOLING-STACK/🌐️p10bg-diagram-force-harness.htm` — temporary Vite-served real-browser shell, React refresh preamble, visible boot diagnostics, and explicit host geometry.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/OWNED-UI-AND-TOOLING-STACK/🟦️p10bg-diagram-force-harness.tsx` — temporary actual Diagram/HostReactFlow 20,000-node/20,000-edge browser harness and telemetry.
- this report.

The cumulative current three-production-file diff is **1,413 insertions / 283 deletions**. The current files are 1,775 Diagram source lines, 600 focused-test lines, and one direct package-test line change. Those cumulative numbers include the preserved P10bb/P10bd force replacement already present in the shared worktree.

## P10be Rejection Repair Map

| P10be blocker                                                                                               | Final repair                                                                                                                                                                                                                                                                                                                                                                                                                 | Permanent proof                                                                                                                                                                                                                                                                                                                                                                                                  |
| ----------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| rAF directly invoked arbitrary `onNodesChange`/React publication after a 20k projection.                    | The rAF listener performs only deadline/fuel cursor work, freezes a generation-tagged snapshot handle, and does O(1) enqueue/coalescing. A fixed-seven-kind macrotask handoff performs bounded Host publication and state/consumer delivery later. Every task has a generation validity predicate; cleanup invalidates stale generations.                                                                                    | Controlled and uncontrolled actual-Diagram 20k tests see no callback in the rAF stack, no synchronous setup reads, a complete 20k semantic snapshot, and a bounded actual Host page of at most 128 nodes/256 edges.                                                                                                                                                                                              |
| A later subscriber/React callback could still monopolize the UI isolate.                                    | Every handoff consumer is watched with an 8 ms contract. A consumer taking at least 8 ms is recorded once and quarantined by function identity; later calls are skipped. The last valid controlled publication remains retained.                                                                                                                                                                                             | A scanning 20k subscriber intentionally blocks for 10 ms, produces one `consumer-publication` violation, retains the prior generation, then receives no third call. A scanning 3,001-node drag consumer blocks for 10 ms, produces one `drag-move` violation, then receives no second move.                                                                                                                      |
| A throwing callback escaped the queue and could break later drain semantics.                                | `DiagramHandoffQueue.consume` wraps `task.run()` in `try/catch`, normalizes the fault diagnostic, applies the same quarantine path, does not replace the last valid snapshot, and leaves the already-scheduled queue drain intact. The violation log is a deterministic capped ring of 16 entries.                                                                                                                           | The throwing-consumer test records exactly `fault: "blocked Diagram consumer"`, retains the earlier last-valid generation, proves a later drag-stop task drains, and proves the throwing publication consumer is skipped thereafter.                                                                                                                                                                             |
| Pointer start/move/stop called arbitrary props and exposed/scanned the full selection on the pointer stack. | Host pointer callbacks now perform O(1) capture/coalesce/enqueue only. Force drag lookup, selection processing, pin/release, and semantic callbacks happen after the pointer stack. For a virtualized host, the initialization cursor builds the selected-node page authority; a lazy O(1)-created offset view expands a visible-page move to the complete selection while scheduled processing consumes one item at a time. | Start+move and stop each assert less than 8 synthetic ms and exactly zero selected-array numeric reads in the pointer callback. The 3,001-node test later observes exactly 3,001 scheduled reads, exact pinning at indices 0 and 2,500, ordered start/move/stop notifications, and post-stop movement. The virtual-host test starts from at most 128 Host nodes yet delivers and moves all 3,001 selected nodes. |
| Public `tick()` exposed an unbounded/infinite interactive route.                                            | `tick()` is removed. Every public interactive force step requires explicit finite `{deadline, fuel}` and returns `{initialized, remainingFuel, tickComplete}`. Tests and the package-local consumer drive the same state machine through bounded `step`.                                                                                                                                                                     | The exact-ID test asserts `"tick" in simulation === false`; a zero-fuel step performs zero identity reads and yields incomplete; source scan finds no production `Number.POSITIVE_INFINITY` or `tick()` route.                                                                                                                                                                                                   |
| Sampled identifier hashes were constructibly colliding Map keys.                                            | Nodes are initialized one UTF-16 code unit per work unit. Exact stable bottom-up merge comparison and exact lookup walk one UTF-16 unit per fuel/deadline check. String identity is never a native Map key; resolved links and hot force phases use numeric node indices and precomputed numeric seeds/fallbacks.                                                                                                            | Two 100,000-code-unit same-length IDs that differ only at index 1 resolve as distinct nodes and only the linked node moves. Initialization reads each ID once; ten hot frames add zero ID reads. Duplicate exact IDs fault deterministically in two separate bounded runs.                                                                                                                                       |
| Setup/callback limits were 50/250 ms and large tests replaced the real host.                                | Permanent thresholds are now less than 8 ms. Setup gates use the same deterministic monotonic budget clock as the scheduler plus exact zero-read input proxies, avoiding suite-preemption false failures while detecting synchronous graph work. No large-graph Host mock/bypass exists: the capture wrapper renders the actual `HostReactFlow`.                                                                             | Actual ReactFlow DOM is present for controlled/uncontrolled 20k integration; first render starts from an empty Host page and later publishes only the bounded page. Full UI stays green with the real host.                                                                                                                                                                                                      |

## Preserved P10bc Repairs

- Live constructor/effect setup captures source references and counts only. Node recovery, full UTF-16 identity preparation, deterministic stable sorting, numeric indexing, edge endpoint lookup, link sorting, degree scans, and strength/bias resolution are persistent initialization cursors under the same deadline/fuel.
- Force ticks remain persistent `alpha → charge → links → collision → nodes` cursors. Pair/link/node progress survives yields, and every unit checks deadline/fuel before execution.
- Live 20k proposal construction is a generation-tagged persistent cursor with direct same-index projection; there is no `map × find`, string-map lookup, or O(N²) notification path.
- Viewport page membership and resolved visible edges are built under the projection cursor. `onMoveEnd` updates the current viewport bounds in O(1); the next scheduled projection rebuilds a bounded current page.
- Restart still schedules a sole generation-bound handle, resets initial-notification state, emits immediately after restart, and rejects stale stop/restart callbacks.
- Hot recovery uses precomputed finite fallback coordinates. Zero-distance jiggle mixes precomputed numeric identity lanes; it does not concatenate or reread IDs.
- The browser harness uncovered a real Diagram import-time cycle: importing `queryElement` from ContextMenu pulled the public UI barrel into an `InfoIcon` temporal-dead-zone fault. Diagram now owns a local typed SSR-safe query helper and has no ContextMenu cross-element dependency.

## Runtime Bounds

| Boundary                                        | Production bound                  |
| ----------------------------------------------- | --------------------------------- |
| public frame deadline                           | 6 ms                              |
| owned force deadline before publication reserve | 5 ms                              |
| fuel per frame                                  | 32,768 units                      |
| completed ticks per frame                       | 4                                 |
| charge/collision pairs, links, nodes per tick   | 2,048 each                        |
| proposal/Host projection items per frame        | 2,048                             |
| real Host page                                  | 128 nodes / 256 edges             |
| handoff kinds scanned per dequeue               | fixed 7                           |
| handoff tasks consumed per macrotask            | 1                                 |
| external/React consumer watchdog                | violation and quarantine at ≥8 ms |
| retained diagnostics                            | latest 16 violations              |

The rAF callback can enqueue/swap only immutable snapshot handles. It cannot call an external consumer or React setter. The separately scheduled handoff consumer can still spend one violating call on the UI isolate; after that measured return or caught fault, production quarantines it and retains the last valid controlled snapshot.

## Focused Test Inventory and Assertions

The final focused file contains **15 tests**:

1. deterministic reversed-input/reference replay;
2. independent link, charge, collision, and center forces;
3. finite recovery and pin/unpin;
4. sole-handle restart/throttle/stale cancellation and initial restart notification;
5. direct 20,000-node cooperative run;
6. full 5,000-node/5,000-link resumable tick phases;
7. small controlled proposal separation/cleanup;
8. actual-host controlled 20,000-node/20,000-edge setup/projection/watchdog;
9. actual-host uncontrolled 20,000-node projection/commit;
10. 3,001-node multi-drag enqueue/coalesce/pin/unpin and slow-handler quarantine;
11. virtualized ≤128-node Host-page drag expanded to all 3,001 selected nodes;
12. throwing consumer diagnostic/quarantine/last-valid/continued drain;
13. exact two-100k-ID endpoint resolution and hot-path read stability;
14. duplicate exact-ID deterministic bounded fault;
15. actual static/SSR surface without browser scheduling.

Permanent exact ceilings/count assertions:

- every synthetic scheduled frame interval: **≤6.1 ms**;
- controlled 20k/20k setup: **<8 ms**, **0 node numeric reads**, **0 edge numeric reads**;
- controlled first frame: notification count **0**, node reads **>0 and <1,200**, edge reads **0**;
- controlled semantic publication length/read scan: **20,000 / 20,000**; one slow violation; last-valid generation unchanged; third subscriber call skipped;
- uncontrolled 20k setup: **<8 ms**, **0 node numeric reads**; committed publication length **20,000**;
- actual Host page: **1–128 nodes**, **1–256 edges**;
- pointer start+move: **<8 ms**, **0 start-array reads**, **0 move-array reads**;
- scheduled large move: exactly **3,001 reads**; quarantined second move leaves the count at **3,001**;
- pointer stop: **<8 ms**, numeric read delta **0**;
- virtualized-host pointer capture: **<8 ms**, Host input **≤128**, semantic selection **3,001**;
- oversized-ID constructor: **<8 ms**, identity getter reads **0**; completed initialization **3**; ten hot frames keep **3**;
- public zero-fuel step returns exactly `{ initialized: false, remainingFuel: 0, tickComplete: false }`.

The sub-8 setup/pointer values in Vitest are deterministic scheduler-clock assertions, intentionally paired with zero-read proxies. Real wall-clock cadence and callback costs are exposed separately by the browser harness rather than conflated with concurrently scheduled Vitest workers.

## Real-Browser Harness

Vite URL:

`http://127.0.0.1:6070/@fs/Users/ueli/Documents/semio/.%F0%9F%A7%ACsemio/%F0%9F%A6%91%EF%B8%8Frepo/%F0%9F%8E%AB%EF%B8%8Ftickets/%F0%9F%8E%86%EF%B8%8F26/%F0%9F%8C%99%EF%B8%8F08/%E2%98%80%EF%B8%8F20/INTERACTIVE-JOB-RUNTIME-REFACTOR/OWNED-UI-AND-TOOLING-STACK/%F0%9F%8C%90%EF%B8%8Fp10bg-diagram-force-harness.htm`

The ticket-local harness:

- chunk-builds 20,000 nodes and 20,000 edges over native rAF frames;
- mounts the actual Diagram and actual HostReactFlow with the first 3,001 nodes selected;
- tracks dataset-frame maximum, native animation-frame gap, Host DOM node/edge counts, full publication lengths/reads, callback stack provenance, semantic drag-selection length, consumer costs, and live handoff diagnostics;
- exports `window.__P10_DIAGRAM_FORCE__.snapshot()`, `armSlowConsumer()`, and `armSlowPointer()`;
- blocks each armed adversarial consumer for 12 real milliseconds;
- exposes pre-module `error`/`unhandledrejection` diagnostics in `window.__P10_DIAGRAM_FORCE_BOOT_ERRORS__` and visibly in `#p10-boot-status`;
- includes the standard Vite React-refresh preamble and explicit `/@vite/client`, because the `.htm` route intentionally avoids the OS dev server's full-document replacement plugin;
- provides explicit 100% Host geometry independent of unavailable Tailwind utility CSS, with the telemetry overlay on the right and the left drag surface unobstructed.

Final in-app Browser/CUA observations after reloading the repaired harness:

- `mounted=true`, `datasetReady=true`, `hostReady=true`, `firstPublicationReady=true`;
- dataset: **20,000 nodes / 20,000 edges**; visible fixed Host page: **87 nodes / 87 edges**;
- dataset-frame maximum: **0.19999998807907104 ms**;
- `callbackInAnimationFrame=false` and `callbackInPointerStack=false`;
- both 12 ms adversarial buttons were armed, then CUA dragged the real visible `rf__node-node-00004`;
- after the first drag: `dragCalls=1`, `dragSelectionLength=3001`, `dragStopCalls=1`, `pointerCaptureCallbacks=1`;
- violations were exactly `[{ elapsedMs: 13, generation: 40, kind: "drag-move" }, { elapsedMs: 13, generation: 43, kind: "consumer-publication" }]`;
- maximum pointer-consumer cost: **13.399999976158142 ms**; maximum publication-consumer cost: **12.600000023841858 ms**;
- `publicationCalls=20` and `publicationReads=400000`; both armed flags returned to false;
- a second real drag without rearming left `dragCalls=1`, `publicationCalls=20`, `publicationReads=400000`, and the violations unchanged, proving both violating consumers stayed quarantined;
- the second drag advanced `pointerCaptureCallbacks` from 1 to 2 and `dragStopCalls` from 1 to 2. This is intentional: quarantine is by exact consumer function identity. The slow `onNodeDrag` consumer is suppressed, while the separately supplied fast O(1) start/stop consumers remain active so drag lifecycle cleanup and independent semantic notification are not discarded.

## Final Non-Cargo Gate Evidence

All commands below ran after the final production-source change unless the row explicitly says otherwise.

| Gate                                  | Exact command                                                                                                   | Observed result                                                                                                                                                                      |
| ------------------------------------- | --------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| focused Diagram, uncached             | `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- --run 🧱️elements/📊️Diagram/🧪️component.test.tsx` | PASS — **1 file / 15 tests**, 6.89 s                                                                                                                                                 |
| full UI, uncached                     | `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache`                                                    | PASS — **20 files / 687 tests**, 11.54 s                                                                                                                                             |
| UI typecheck, uncached                | `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`                                                     | PASS                                                                                                                                                                                 |
| UI lint, uncached                     | `bun nx run @semio-tech/ui-react:lint --skip-nx-cache`                                                          | PASS; only existing Bun `NO_COLOR`/`FORCE_COLOR` warning                                                                                                                             |
| UI primitive policy                   | `bun nx run @semio-tech/ui-react:check-ui-primitives --skip-nx-cache`                                           | PASS — **0 violations / 2 allowlisted files**                                                                                                                                        |
| renderer consumer suite, uncached     | `bun nx run @semio-tech/framework-renderer-react:test-quick --skip-nx-cache`                                    | PASS — **4 files / 439 tests**, 3.85 s                                                                                                                                               |
| renderer lint, uncached               | `bun nx run @semio-tech/framework-renderer-react:lint --skip-nx-cache`                                          | PASS — region/host contract                                                                                                                                                          |
| exact owned formatting                | `bun nx format:check --files='<component>,<test>,<package index>,<harness html>,<harness tsx>,<report>'`        | PASS after final harness geometry/evidence formatting                                                                                                                                |
| frozen lock validation                | `bun install --lockfile-only --ignore-scripts --no-progress --no-summary --frozen-lockfile`                     | PASS, exit 0; Bun printed `Saved lockfile`; `git diff -- bun.lock` remained empty, so the pre-existing staged lock state gained no repair-owned unstaged change                      |
| dependency freeze                     | `bun ./📜️script.ts verify dependencies`                                                                         | PASS — baseline **238**, current **141**, removed **97**, additions **0**                                                                                                            |
| JavaScript identity census            | `bun ./📜️script.ts verify dependencies list js`                                                                 | PASS — **78**                                                                                                                                                                        |
| Rust identity census through Bun only | `bun ./📜️script.ts verify dependencies list rust`                                                               | PASS — **63**; no Cargo process                                                                                                                                                      |
| JavaScript dependency parity          | `bun ./📜️script.ts verify dependencies parity js`                                                               | PASS — manifests **83**, external rows **263**, evidenced **114**, unowned advisory **149**, undeclared imports **0**, lock workspaces **44**, lock mismatches **0**, fixtures **5** |

Source scans after the final production-source change:

- production Diagram/package retired `d3-force`, `d3-quadtree`, and `@types/d3-force`: **0**;
- production `Number.POSITIVE_INFINITY` and public `tick()`: **0** (the sole infinity occurrence is the finite-recovery test input);
- rejected `finalNodes.map`, `finalEdges.map`, `simulation.nodes().find`, `originalById`, sampled string-map identity keys, and direct drag-prop optional calls: **0**;
- `[DEBUG]` in owned source/test: **0**;
- JSX `onNodeDragStart={handleNodeDragStart}`: exactly **1**;
- scoped `git diff --check`: clean.

## Honest Residuals

- No real mobile touch, pointer-cancel, stylus, high-frequency hardware coalescing, background-tab throttling, sustained pan/zoom, or accessibility/visual-quality pass ran.
- The virtual Host page uses a cursorized scan, not a separate logarithmic spatial tree. It is deadline/fuel bounded and caps ReactFlow reconciliation to 128/256, but a fresh page still requires a resumable O(N+E) scan.
- An arbitrary consumer can block once before the watchdog can observe and quarantine it. This is the explicit production quarantine contract, not hard preemption of JavaScript.
- Renderer-wide typecheck was not run; the focused UI typecheck and real renderer consumer tests/lint are green.
- No independent audit has rerun this exact final state.

## Required Independent Verdict

Keep removal rejected at **141 / 78+63** until a fresh Terra audit:

1. inspects rAF, handoff, viewport-page, pointer, exact-ID, exception, and ContextMenu-cycle paths;
2. drives the repaired browser harness through native publication and pointer scenarios;
3. confirms one overrun/fault observation followed by quarantine and last-valid retention;
4. reruns focused 15, full UI 687, and renderer 439 suites;
5. independently repeats the rejected-pattern/dependency/parity/diff gates.
