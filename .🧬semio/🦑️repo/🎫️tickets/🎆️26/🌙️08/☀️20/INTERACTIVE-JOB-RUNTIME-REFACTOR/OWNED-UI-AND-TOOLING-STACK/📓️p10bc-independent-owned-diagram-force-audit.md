# P10bc Independent Owned Diagram Force Audit

## Verdict

**REJECT.** `d3-force` and its orphaned `d3-quadtree` resolution are cleanly removed, the owned API has no D3 public/type leak, and every prescribed automated gate is green. The claimed **strict four-tick / six-millisecond cooperative browser budget is false for the live Diagram integration**: each throttled force notification performs an unbounded, quadratic position projection after the engine's budget loop. The supplied 20,000-node test exercises the simulation alone and therefore cannot establish the real Diagram frame contract.

This is a packet blocker, not a browser-only residual. The production path can freeze a large controlled or uncontrolled Diagram immediately after an otherwise budgeted force frame.

## Scope And Removal Findings

| Area | Independent result | Evidence |
| --- | --- | --- |
| Closed force boundary | PASS | The owned structural `DiagramForceConfig`, node/link, and simulation interfaces are in `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️component.tsx`. The public UI barrel re-exports only owned config/Diagram surface; its internal test-only import of `createDiagramForceSimulation` is not re-exported. |
| D3 public/type leak | PASS | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️implementation.d.ts` contains only the retained Dagre declaration. Exact source/config/package/lock scans found no live `d3-force`, `d3-quadtree`, `forceImplementation`, or `DiagramForceImplementation`. The only broad-repository hits are historical `🔒️dependencies.json` records. |
| Manifest and lock | PASS | The UI React manifest no longer declares `d3-force`; frozen Bun install succeeds. `bun.lock` has neither retired resolution. The dependency census is **141 total / 78 JS / 63 Rust**, a 97-identity reduction from the 238 baseline. |
| Legitimate retained D3 transitives | PASS | `@xyflow/react` → `@xyflow/system` remains active. The latter requires `d3-zoom`; its lock path retains `d3-dispatch` and `d3-timer` through `d3-drag`/`d3-transition`. They are not D3 Force remnants. |
| Excluded scope | PASS | The packet leaves active Dagre, XYFlow, DnD, i18n, routing, resizable, graphics/PDF, and `xstate` identities intact. |

## Blocking Scheduler Finding

The engine limits only the work before the notification:

- `advanceFrame` ticks up to four times and tests elapsed time only **between whole ticks** at `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️component.tsx:458-475`.
- It invokes the tick listener after that loop at lines 469-472, with no item/time budget around listener work.
- In uncontrolled Diagram mode, the listener maps all state nodes and runs `simulation.nodes().find(...)` for each at lines 833-845. This is **O(N²)** per notification.
- In controlled mode, it maps all simulation nodes and runs `finalNodes.find(...)` for each at lines 846-855. This is likewise **O(N²)** per notification.

The focused 20,000-node test at `🧪️component.test.tsx:145-158` creates a simulation directly; it never renders `Diagram`, so neither production projection runs. The test consequently proves the engine's cursored node integration only, not the scheduler claimed by P10bb. It also uses zero charge/link/collision/center forces, so it does not establish the time limit for a full 2,048-pair/link/node tick. Since the deadline is checked only after `tick()` returns, even engine-only work may overrun six milliseconds before the next check.

Required repair before acceptance:

1. Make projection itself cooperative: remove the `map` × `find` listener paths, use pre-indexed identity lookup, and do not whole-array-project a large graph outside the per-frame budget. Expose/consume bounded dirty batches or an equivalent resumable projection path.
2. Enforce elapsed-time checks inside tick work as well as tick-count/item limits; do not claim a hard six-millisecond ceiling otherwise.
3. Add real-Diagram 20,000-node controlled and uncontrolled tests that run scheduled frames and account for all notification/projection work, plus a full-force time/budget test.

## Behavior Reattack

| Requirement | Result | Audit evidence |
| --- | --- | --- |
| Deterministic endpoint resolution, fallback, finite recovery | Partial PASS | Constructor sorts nodes/links by identity and resolves string/object endpoints (`component.tsx:252-267`); hash jitter/fallback repairs zero-distance, overlap, and non-finite values (`215-224`, `325-368`, `397-416`). Tests cover input reversal and a small finite/pinned fixture. |
| Alpha/decay, degree bias, charge/link/collision/center math | Partial PASS | Implementation has the expected owned calculations (`316-440`) and directional two/three-node tests pass. The captured retired-D3 comparison is only one three-node fixture with a broad distance `< 35` tolerance (`test:57-72`), so it is not a complete differential proof for degree or endpoint variants. |
| Pinned nodes and drag semantics | NOT CLOSED | Direct simulation fixtures mutate `fx`/`fy` (`test:102-119`) but do not invoke live `handleNodeDragStart`, `handleNodeDrag`, or `handleNodeDragStop` (`component:733-796`). No test proves multi-select pin/move/unpin order through the real Diagram, controlled lag, or a selected dragged node beyond the 2,048 node cursor. The live multi-select pointer path itself repeatedly scans selected nodes (`763-769`), an additional large-selection responsiveness risk. |
| Controlled/uncontrolled proposal separation | Partial PASS | One-node controlled input remains unmutated and receives a proposal (`test:160-181`). There is no uncontrolled live force test or large controlled-lag projection test; both are required by the blocker repair. |
| rAF, stop, supersession | PASS for covered unit contract | A single handle is scheduled/cancelled and stale callback returns while `running` is false (`298-313`, `446-475`); fake-frame test covers stop and unmount cancellation. This does not cure the listener budget breach. |
| `updateIntervalMs` | NOT CLOSED | First-run throttling is covered (`test:121-143`), but `lastNotification` is initialized once (`241`) and is not reset in `restart()` (`298-303`). A reheat/restart within the interval can suppress the claimed initial notification. Add a restart-after-prior-notification test and define/repair this public option's intended initial-notification behavior. |
| SSR/static safety | PASS for covered server path | Browser APIs are guarded in `scheduleFrame`/`stop` and static render schedules no work (`446-455`, `test:183-188`). |

## Gates Re-run Independently

| Gate | Result |
| --- | --- |
| Focused Diagram force suite, uncached | PASS — 1 file, 7 tests |
| Full UI React suite, uncached | PASS — 20 files, 679 tests |
| UI React typecheck, uncached | PASS |
| UI React lint, uncached | PASS — only the existing Bun color-environment warning |
| UI primitive policy | PASS — 0 violations, 2 allowlisted files |
| Renderer React consumer suite, uncached | PASS — 4 files, 439 tests |
| Exact changed-file format check | PASS |
| Frozen Bun install with lifecycle scripts disabled | PASS |
| Dependency freeze | PASS — baseline 238, current 141, no additions |
| JS/Rust identity census | PASS — 78 / 63 |
| JavaScript manifest/lock parity | PASS — 83 manifests, 263 external rows, 114 evidenced, 149 advisory-unowned, 0 undeclared imports, 0 lock mismatches, 5 fixtures, 44 lock workspaces |
| Regenerated manifest/source evidence | PASS — 64 manifests, 575 direct rows, 263 external rows, 74 no-package-scope-evidence candidates |
| Targeted `git diff --check` and `[DEBUG]` scan | PASS |

No Cargo command was run. Renderer-wide typecheck remains intentionally outside this packet because of its separately documented unrelated baseline; the uncached real renderer consumer test is green and no changed-Diagram/removal-identity diagnostic was observed in the UI gate.

## Browser Residuals After Repair

Native pointer capture/touch gesture delivery, production animation cadence under load, and visual layout quality still need a real-browser large-graph drag/cancellation pass. These are residuals only **after** the deterministic live integration budget and drag-projection tests above pass; they do not downgrade the current rejection.
