# P10bh Independent Owned Diagram Force Final Audit

## Verdict

**ACCEPT — the P10 Diagram force dependency wave may remove `d3-force` and `d3-quadtree`.**

The independently observed live boundary is **141 total third-party dependencies = 78 JavaScript + 63 Rust**. This verdict accepts that dependency wave only. It does **not** close, accept, or otherwise assert phase-wide completion of P10; unrelated P10 work remains outside this audit.

No production source, manifest, or Git command was modified or run by this audit. The frozen-lock validation printed `Saved lockfile`, but the immediate `git diff --numstat -- bun.lock` was empty. The final scoped status still reports a staged `bun.lock` change, which this audit does not attribute; it has no resulting unstaged lock diff. No Cargo, native, or Wasm command was run.

## Scope And Method

This was a fresh audit of the final working tree, not a confirmation of `p10bg-owned-diagram-force-second-repair.md`. I read that implementation report only to determine claimed gates, then inspected the production force/runtime code, its 15-test suite, the direct package consumer, and the ticket-local real-browser harness. The runtime exercise used the already-running `127.0.0.1:6070` Vite page and real coordinate-based browser pointer drags.

## Source Acceptance

| Requirement                                                                      | Independent final-source evidence                                                                                                                                                                                                                                                                                                                         | Result |
| -------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ |
| Bounded rAF delivery                                                             | The rAF callback is sole-handle/generation guarded at `component.tsx:869-942`; it creates a 5 ms working deadline from the 6 ms public frame bound and 32,768 fuel. The listener performs cursor work and only enqueues immutable snapshot publication; React setters and external consumers run from the macrotask handoff at `1073-1144` / `1521-1537`. | Pass   |
| O(1) drag capture and deferred semantic delivery                                 | `handleNodeDragStart`, `handleNodeDrag`, and `handleNodeDragStop` only capture/coalesce state, enqueue consumers, and schedule the simulation at `1388-1451`. The 3,001-node selection is a lazy proxy (`998-1008`); actual ID lookup/pinning is cursorized at `769-810`.                                                                                 | Pass   |
| Exact UTF-16 identity and collision handling                                     | Each code unit is processed by `charCodeAt` under fuel (`648-667`). Ordering and lookup use resumable exact character comparison (`342-390`, `625-645`); duplicate exact IDs throw deterministically. Force links resolve to numeric runtimes, so no sampled string-hash or string-key `Map` is used in the hot boundary.                                 | Pass   |
| Persistent bounded setup/tick/projection                                         | Initialization phases, tick cursors, per-phase 2,048 caps, and deadline/fuel checks are at `311-314`, `648-766`, `812-862`. Projection is separately cursorized and Host output is capped at 128 nodes / 256 edges at `1480-1538`.                                                                                                                        | Pass   |
| Fixed Host page                                                                  | `diagramForceHostPage` fixes extent 2,048, nodes 128, edges 256 at `984`; only `hostNodes`/`hostEdges` render while force is enabled above the node cap at `1326-1330`.                                                                                                                                                                                   | Pass   |
| Generation and stale invalidation                                                | Simulation schedule/restart/stop generation checks are at `461-501`, `869-942`. Publication validity closes over active simulation and generation, and cleanup invalidates all publication kinds at `1524`, `1535`, `1544-1551`. Drag handoff generation is separately invalidated at `1393-1406`.                                                        | Pass   |
| Slow/throwing consumer quarantine and retained last valid controlled publication | The seven-kind queue coalesces one task/kind, drains one task per timeout, catches faults, measures every consumer, quarantines at `>=8 ms`, retains a 16-entry ring, and updates `lastValidPublication` only after a valid consumer publication (`1071-1143`).                                                                                           | Pass   |
| ContextMenu import-cycle removal                                                 | Diagram owns its SSR-safe `queryDiagramElement` helper (`986-988`), and its import section contains no ContextMenu/public-barrel import. The public index’s independent ContextMenu exports remain unrelated.                                                                                                                                             | Pass   |
| No public infinite `tick()`                                                      | Public `DiagramForceSimulation` exposes finite `step({ deadline, fuel })` and no `tick` member (`183-193`). Production scan found no `Number.POSITIVE_INFINITY` or `.tick(` route.                                                                                                                                                                        | Pass   |

The single `new Map` remaining in the Diagram file is the separate Dagre layout adapter at `93`; it is not part of the owned force identity/resolution path and is not either removed dependency.

## Runtime Browser Acceptance

Fresh browser telemetry after the harness settled:

- Mounted actual Diagram and actual HostReactFlow: `mounted`, `datasetReady`, `hostReady`, and `firstPublicationReady` were all true.
- Dataset and semantic publication: 20,000 nodes, 20,000 edges, `lastPublicationLength=20000`; no consumer callback ran in an animation-frame or pointer stack.
- Fixed real Host page: 88 rendered nodes and 88 rendered edges, within 128/256.
- One real CUA drag of a visible selected ReactFlow node after arming both probes resulted in `dragCalls=1`, `dragSelectionLength=3001`, `dragStopCalls=1`, and `pointerCaptureCallbacks=1`.
- The actual 12 ms adversarial functions produced exactly one `drag-move` violation (13 ms) and one `consumer-publication` violation (12 ms). The observed maximum costs were 13.7 ms and 12.7 ms respectively, proving the watchdog measures real consumer cost after it is safely off the rAF/pointer stacks.
- A second real CUA drag without rearming left `dragCalls=1`, `publicationCalls=21`, `publicationReads=420000`, and the two violation records unchanged. `dragStopCalls` and `pointerCaptureCallbacks` advanced to 2, which is correct: the slow drag consumer was quarantined by identity while distinct fast lifecycle handlers retained cleanup. The retained 20,000-node publication remained intact.
- Dataset construction’s observed maximum frame work was 0.2 ms. No boot error or unhandled rejection appeared in the visible harness.

## Fresh Automated Gates

| Gate                       | Exact command                                                                                                   | Result                                                                                                                                |
| -------------------------- | --------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| Focused Diagram            | `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- --run 🧱️elements/📊️Diagram/🧪️component.test.tsx` | Pass: 1 file, 15 tests                                                                                                                |
| Full UI                    | `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache`                                                    | Pass: 20 files, 687 tests                                                                                                             |
| Renderer consumer          | `bun nx run @semio-tech/framework-renderer-react:test-quick --skip-nx-cache`                                    | Pass: 4 files, 439 tests                                                                                                              |
| UI typecheck               | `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`                                                     | Pass                                                                                                                                  |
| UI lint                    | `bun nx run @semio-tech/ui-react:lint --skip-nx-cache`                                                          | Pass; only the existing Bun `NO_COLOR`/`FORCE_COLOR` warning                                                                          |
| UI primitive policy        | `bun nx run @semio-tech/ui-react:check-ui-primitives --skip-nx-cache`                                           | Pass: 0 violations, 2 allowlisted files                                                                                               |
| Renderer lint              | `bun nx run @semio-tech/framework-renderer-react:lint --skip-nx-cache`                                          | Pass                                                                                                                                  |
| Frozen lock                | `bun install --lockfile-only --ignore-scripts --no-progress --no-summary --frozen-lockfile`                     | Pass; no resulting `bun.lock` diff                                                                                                    |
| Dependency freeze          | `bun ./📜️script.ts verify dependencies`                                                                         | Pass: baseline 238, current 141, no new third-party dependencies                                                                      |
| JavaScript list            | `bun ./📜️script.ts verify dependencies list js`                                                                 | Pass: 78 JavaScript dependencies                                                                                                      |
| Rust list through Bun only | `bun ./📜️script.ts verify dependencies list rust`                                                               | Pass: 63 Rust dependencies; no Cargo process                                                                                          |
| JavaScript parity          | `bun ./📜️script.ts verify dependencies parity js`                                                               | Pass: manifests 83, external rows 263, evidenced 114, unowned 149, undeclared imports 0, lock workspaces 44, mismatches 0, fixtures 5 |

## Rejected-Pattern And Diff Audit

- No production `d3-force`, `d3-quadtree`, or `@types/d3-force` occurrence in the Diagram source or direct UI package consumer.
- No production `Number.POSITIVE_INFINITY` or `.tick(` route.
- No rejected `finalNodes.map`, `finalEdges.map`, `simulation.nodes().find`, `originalById`, or direct optional drag-consumer call in the owned force source.
- Diagram has no ContextMenu or imported `queryElement` reference.
- `git diff --check` completed cleanly over the current shared worktree.

## Residuals And Boundary

This audit deliberately does not claim a hard-preemptive JavaScript budget: any external callback can block once before the watchdog observes and quarantines it. The acceptance criterion is therefore the verified queue boundary, one measured violation, deterministic quarantine, continued draining, and preservation of the last valid controlled publication. Real mobile touch/pointer-cancel/stylus and sustained pan/zoom testing were not part of this dependency-wave acceptance.
