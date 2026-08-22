# P10bd Owned Diagram Force Repair

## Verdict

**AUDIT-READY, NOT ACCEPTED.** The P10bc production blockers and the subsequent setup, pointer-callback, stale-generation, and identifier hot-path reattacks are repaired in the owned Diagram force boundary. The focused and proportionate non-Cargo gates below are green after the final source change.

The provisional removal of `d3-force` and `d3-quadtree` remains **REJECTED** until a fresh independent Terra audit inspects the final live effect and pointer paths and reruns the focused/full suites. This report is implementation evidence, not independent acceptance.

## Scope

Repair-owned paths:

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🧪️component.test.tsx`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/OWNED-UI-AND-TOOLING-STACK/📓️p10bd-owned-diagram-force-repair.md`

No framework Layout/shared Rust file, manifest, lockfile, generated implementation declaration, or dependency artifact was edited by this repair. The cumulative Diagram files also contain the preserved P10bb owned-force replacement and concurrent/staged work.

## Rejection Repair Map

| Finding | Final repair | Proof |
| --- | --- | --- |
| Live projection used whole-array `map` × `find`, making notifications O(N²). | The live simulation builds its force output in input index order. A persistent `DiagramForceProjection` advances one index at a time, copies directly from the same input index, and has neither a scan nor nested identity lookup. | Real controlled and uncontrolled 20,000-node Diagram fixtures each publish one complete 20,000-node proposal/commit after resumable scheduled work. Rejected-pattern scan is zero. |
| Projection ran after the engine budget. | Projection persists `{cursor, generation, nodes}`, checks the simulation frame's shared deadline before each item, caps a frame at 2,048 projection items, and resumes a pending projection before additional force work. | Both live 20,000-node fixtures require multiple scheduled frames and assert every synthetic interval is at most 6.1 ms. |
| Tick phases could overrun before a post-hoc frame check. | A tick is a persistent `alpha → charge → links → collision → nodes` cursor. Pair, link, node, phase, and per-phase progress survive frame yields. Each unit is preceded by a deadline check; tick ceilings remain 2,048 units and four completed ticks per frame. | The 5,000-node/5,000-link full-force fixture enables charge, links, collision, center, and integration, requires more than 20 frames, and observes a maximum 5.099999999995362 synthetic ms. |
| The live effect still synchronously cloned/mapped/sorted/scanned/recovered a large graph before scheduling. | Live construction now captures only array references/counts and allocates an empty output. A generation-tagged initialization cursor performs node materialization/recovery/indexing, stable bottom-up merge sorting, edge endpoint resolution, stable link sorting, degree scans, and link strength/bias resolution under the same five-millisecond work deadline. No input element is read by constructor/effect setup. | Controlled 20,000-node + 20,000-edge setup observed 1 ms and exactly zero numeric node/edge reads. Uncontrolled 20,000-node setup observed 1 ms and zero numeric reads. Permanent assertions cap setup at 250 ms and retain the zero-read requirement. |
| Setup sort needed deterministic ordering without a synchronous whole-array `sort`. | Stable bottom-up merge cursors emit at most one sorted output item per step and persist merge width/range/cursors across frames for both nodes and links. Runtime identity ordering remains deterministic. | Deterministic reversal/reference fixture remains green, while first controlled large-graph frame reads only 500 nodes and zero edges instead of consuming either source. |
| Drag callbacks synchronously scanned the selected collection. | Start/move/stop callbacks enqueue or coalesce one pending drag batch in O(1), update generation/reheat state, and return. Scheduled `stepDrag` advances one selected entry or one pinned-release entry per deadline-checked unit. A move replaces an unfinished prior batch, so pointer delivery coalesces instead of accumulating work. | Real 3,001-selected-node Diagram fixture observes start+move callback time 0 ms, exactly zero start-array and move-array numeric reads, stop time 0 ms, and zero stop-time read delta. Scheduled work later reads all 3,001 move entries, pins node 2,500, and later proves it moves after cursored unpin. |
| Multi-select drag semantics were unproven. | Scheduled drag processing synchronizes x/y/fx/fy for the latest batch and tracks owned pins; start/stop release prior owned pins cooperatively. The public start/drag/stop callback order is preserved. | The live fixture verifies exact positions for indices 0 and 2,500, controlled-input immutability, callback order `start, drag, stop`, and post-stop movement of index 2,500. |
| Restart could suppress its initial notification and stale scheduled callbacks could disturb the live handle. | Restart resets the notification origin, increments generation, cancels/reschedules the sole live handle, and generation-binds callbacks. Stop invalidates generation. A stale callback returns before clearing the current handle. | Scheduling fixture proves one handle after repeated restart, notification at time 0, throttle at time 10, fresh notification at time 20 after stop/restart inside a 50 ms interval, stale callback inertness, and exact cancellation. |
| Fallback recovery and zero-distance jiggle repeatedly hashed/concatenated IDs in charge/link/collision/node units. | Initialization samples at most 256 evenly distributed UTF-16 code units into four stored deterministic hash lanes, length, key, and fallback coordinates. Hot `recover` uses stored coordinates; hot jiggle mixes stored integers only. | Maximum/oversized-ID fixture uses one 256-character ID and two 100,002-character IDs. Constructor reads zero IDs; initialization reads exactly three; ten subsequent hot frames reread none and keep all positions finite. |
| Real controlled/uncontrolled large Diagram integration was absent. | Both modes render the real `Diagram` and `ReactFlowProvider`. Only the Flow DOM host is replaced above 1,000 nodes to isolate Diagram scheduling/projection from 20,000 unrelated DOM nodes. Small controlled, cleanup, and SSR tests continue through the actual Flow host. | Controlled input is not mutated and receives a complete ordered proposal; uncontrolled state commits the identical complete array observed by the Flow host boundary. |

## Runtime Budget Contract

- The public frame ceiling is six milliseconds. Owned work uses a five-millisecond deadline, leaving one millisecond of publication reserve.
- Initialization, queued/coalesced drag, force tick phases, and notification projection all use that same deadline and preserve cursors across frames.
- A pending projection resumes before force work. At most one completed notification is published per scheduled frame.
- Initialization and projection are generation-tagged. Restart/drag supersession invalidates stale notification/projection work; stop and restart also generation-bind the scheduled callback.
- Manual `tick()` is the explicit unbounded synchronous adapter and drives the same initialization, drag, and tick state machines with an infinite deadline. The live Diagram path uses scheduled bounded work.
- Identity work is capped at 256 sampled code units per identifier and occurs only in resumable initialization or resumable drag lookup, never in steady-state force units.

## Focused Test Evidence

The final focused suite contains **12 tests**:

1. deterministic reversed-input/reference replay;
2. independent link, charge, collision, and center behavior;
3. finite recovery and pinned/unpinned movement;
4. sole-handle restart/throttle/stale-generation cancellation;
5. direct 20,000-node cooperative initialization/integration;
6. resumable 5,000-node/5,000-link full force;
7. small controlled proposal separation;
8. controlled live 20,000-node + 20,000-edge cursorized setup/projection;
9. uncontrolled live 20,000-node cursorized setup/commit;
10. real 3,001-node selected drag enqueue/coalescing/pin/unpin;
11. capped maximum/oversized identifier initialization and hot-path read stability;
12. SSR/static scheduling safety.

Permanent timing/read assertions in the final test source:

| Path | Final assertion |
| --- | --- |
| Every fake-clock scheduled interval | `<= 6.1 ms` |
| Controlled 20k/20k synchronous effect setup | `< 250 ms`; node numeric reads `= 0`; edge numeric reads `= 0` |
| Controlled first scheduled frame | no notification; node reads `> 0` and `< 600`; edge reads `= 0` |
| Uncontrolled 20k synchronous effect setup | `< 250 ms`; node numeric reads `= 0` |
| Selected start+move callbacks | `< 50 ms`; start-array reads `= 0`; move-array reads `= 0` |
| Selected stop callback | `< 50 ms`; move-array read delta `= 0` |
| Oversized-ID constructor | `< 50 ms`; ID getter reads `= 0` |
| Oversized-ID completed initialization | ID getter reads `= 3`; ten hot frames keep the count unchanged |

A temporary `[DEBUG]` diagnostic run recorded exact synthetic values before the final stale-generation handle hardening; those logs were removed, and the permanent assertions plus the entire final gate set were rerun after that hardening:

| Diagnostic path | Exact observation |
| --- | --- |
| Direct 20,000-node cooperative run | 681 frames; maximum 5.0400000001100125 ms |
| Full 5,000-node/5,000-link force run | 666 frames; maximum 5.099999999995362 ms |
| Controlled live setup | 1 ms; 0 node reads; 0 edge reads |
| Controlled first frame | 500 node reads; 0 edge reads |
| Controlled live completion | 1,483 frames; maximum 5.0400000001100125 ms; 40,000 node reads; 20,000 edge reads |
| Uncontrolled live setup | 1 ms; 0 node reads |
| Uncontrolled live completion | 721 frames; maximum 5.5800000001217995 ms; 40,000 node reads |
| Drag start+move callbacks | 0 ms; 0 start-array reads; 0 move-array reads |
| Drag scheduled pin/projection | 98 frames; maximum 5.049999999995407 ms; 3,001 move-array reads |
| Drag stop callback | 0 ms; 0 additional move-array reads |
| Oversized identity run | 1 frame; maximum 0.8400000000000005 ms; 3 total ID reads |

## Final Non-Cargo Gates

All rows below are final observed command outputs after the last implementation change.

| Gate | Command | Final observed output |
| --- | --- | --- |
| Focused Diagram suite, uncached | `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- --run 🧱️elements/📊️Diagram/🧪️component.test.tsx` | PASS — 1 file, 12 tests, 2.81 s |
| Full UI React quick suite, uncached | `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | PASS — 20 files, 684 tests |
| UI React typecheck, uncached | `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | PASS |
| UI React lint, uncached | `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | PASS; only Bun's existing `NO_COLOR`/`FORCE_COLOR` warning |
| UI primitive policy | `bun nx run @semio-tech/ui-react:check-ui-primitives --skip-nx-cache` | PASS — 0 violations, 2 allowlisted files |
| Renderer React consumer suite, uncached | `bun nx run @semio-tech/renderer-react:test-quick --skip-nx-cache` | PASS — 4 files, 439 tests |
| Renderer React lint, uncached | `bun nx run @semio-tech/renderer-react:lint --skip-nx-cache` | PASS |
| Exact owned-source format | `bun nx format:check --files='<component.tsx>,<component.test.tsx>'` | PASS |
| Frozen lock validation | `bun install --lockfile-only --ignore-scripts --no-progress --no-summary --frozen-lockfile` | PASS; exit 0; no repair-owned lock edit |
| Dependency freeze | existing Bun/Nx dependency-freeze gate | PASS — baseline 238, current 141, removed 97, additions 0 |
| JavaScript/Rust identity census | existing Bun/Nx list gates | PASS — JavaScript 78; Rust 63 |
| JavaScript dependency parity | existing Bun/Nx parity gate | PASS — 83 manifests, 263 external rows, 114 evidenced rows, 149 advisory-unowned rows, 0 undeclared imports, 0 lock mismatches, 5 fixtures, 44 lock workspaces |

Final source scans:

- retired `d3-force`/`d3-quadtree` identities and adapters in the Diagram implementation/declaration, UI manifest, and `bun.lock`: zero;
- rejected `finalNodes.map`, `finalEdges.map`, simulation-node `find`, `originalById`, and equivalent old projection/setup patterns: zero;
- `[DEBUG]` in both owned code/test files: zero;
- exact two-code-file `git diff --check`: clean.

No Cargo command and no modifying Git command ran.

## Explicitly Unrun Browser-Native Work

The following were not run and remain for the independent browser audit:

- native `requestAnimationFrame` cadence and main-thread contention under a real 20,000-node browser workload;
- native pointer capture, pointer cancellation, touch gesture delivery, and rapid/coalesced hardware pointer streams;
- real ReactFlow DOM rendering/measurement of 20,000 nodes and edges, because the deterministic large fixtures replace only that host boundary;
- subjective force-layout stability and visual quality during sustained drag/reheat;
- arbitrary consumer callback cost inside controlled `onNodesChange`, which the owned scheduler cannot bound after dispatch.

The capped 256-sample identity is deterministic and makes oversized-ID work bounded, but two distinct same-length identifiers that differ only at unsampled positions can theoretically alias. That is an explicit bounded-identity residual for the fresh audit, not a hidden claim of full-string uniqueness.

Renderer-wide typecheck was not rerun because P10bc records an unrelated graphics/shell/WASM baseline; the bounded UI typecheck and real renderer NodeGraph consumer tests are green.

## Required Next Verdict

Keep dependency removal rejected until a fresh Terra audit:

1. inspects the final live effect for synchronous whole-graph reads/work;
2. inspects pointer callbacks for selected-set iteration;
3. reattacks stale generation/sole-handle behavior;
4. reruns the focused 12-test suite and full 684-test UI suite;
5. evaluates the browser-native residuals above.

Only that independent result may change the packet from audit-ready to accepted.
