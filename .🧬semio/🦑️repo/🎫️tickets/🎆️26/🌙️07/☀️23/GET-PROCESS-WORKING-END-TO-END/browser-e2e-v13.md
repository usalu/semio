# Browser E2E v13: role-switch memory leak and empty viewport, two root causes fixed

## Result
Two product fixes, both verified in Firefox on the v11 hub before the rebuild:
1. Shard workers are recycled when their last actor is gone (a role switch no longer leaves about 1 GB resident).
2. The 3D surface is edgeless, so an opened Actions window no longer pushes the canvas out of view (this is the "empty dark viewport" of v11).

Scenarios on the v11 hub with these fixes, canvas pixel check on: `process-interactions.json` exit 0 (canvas 1366x558, 43363 non-background px), `process-load-trace.json` exit 0 (1366x596, 44725 px), Demo to Viewer to Editor cycle exit 0 (42689 px; `e2e-process-v13-cycle.png` viewed: grid, gizmo, workpiece). `process-cancel.json` exit 1, see Open.

Not green end to end: cancel fails on a guest error independent of my changes, and the hub rebuilt afterwards (as requested) does not boot because of other agents' in-flight Rust changes, so the served hub is currently broken.

## Root cause 1: leak (about 1 GB per role switch)
`footprint -p` of the content process, Demo, before: 1.7 GB loaded, 2.7 GB after Editor to Viewer and flat for 60 s, then 3.1 to 4.5 GB on Viewer to Editor and the browser was killed (about 15 of 15 attempts).
Each actor runs in a shard worker. The worker drops the instance on `dispose`, but wasm linear memory only returns to the OS when the worker ends, and round robin puts the next actor on another worker, so each switch grew another worker. Fix: `ShardRouting.recycleIdle(slot, closed?)` replaces a worker that has no actor, no pending request, no activation and no other pending original closure with a fresh worker at the same index. The transport calls it when the worker reports `original-source-close` (also when `sourceClosed` is false, because the worker dropped the instance anyway).
After: Demo, Viewer, Editor, Viewer, Editor: 1.6, 2.8 (switching), 1.57, 2.7, 1.64 GB (`footprint-role-cycle.txt`); the run exits 0. No JS-side retention (archive bytes, scene, GL) was found; the whole step was the workers.

Law: `shard-client/tests/shard-recycle/recycle.test.ts` (bun test, 4 pass): idle shard replaced and old worker terminated; a shard with an actor, pending request, pending original closure or activation is never touched; the just-reported closure is ignored but another pending one blocks; a stale slot is not recycled twice. A FinalizationRegistry check was not possible because the guest lives in a worker, not on the page heap.

## Root cause 2: empty viewport with Actions open
Opening Actions moved the 3D canvas to y=654 (below the viewport). Reproduced on a fresh Demo with only "open Actions". `useWindowContentDeadLineInset` pads the scroller by the bottom of the floating chrome unless the content has `data-window-content-layout="edgeless"`; only `canvas-2d` surfaces had it, so world-3d was pushed down by the whole expanded Actions panel (596 px). Fix: `EDGELESS_SURFACE_KINDS` (canvas-2d, world-3d, board-2d, tiled-map) in the surface shell. After: canvas at y=58, 596 px high, 44725 px drawn with Actions open.

## Harness
Canvas pixel check (exit 1 if no canvas or fewer than 200 differing px), `LIVE_LOG`, crash-data capture (v12). New: `run-forest-render.sh`, `run-render-retry.sh`, `run-three.sh`, scenarios `process-role-cycle.json`, `process-actions-open.json`. `publish-hub.ts` now also writes the shard worker (`shardWorkerSource`), because the generated bridge needs the worker to pass the core-module loader.

## Open
1. `process-cancel.json`: about 28 s after the cancelled load, `runtime live cleanup faulted for instance 1: a prior step outcome faulted or was cancelled` (guest, `original-turn.native-return-refused`), then the Viewer switch fails. Same with `recycleIdle` disabled, so not caused by the shard change; a guest Rust fault in the cancel path of the v11 hub.
2. Rebuild: `rebuild-and-boot.sh` (gated, about 55 min) built and published, boot check failed. First `actor-core-module.loader-required` (stale shard worker; fixed by publishing the worker), then `InvariantViolated: Deflate restart requires complete output custody` in the guest at boot (deflate encode cursor, untouched today). The hub is built from uncommitted Rust changes of other agents (process3d, brep). The previously working hub was overwritten, so the dev page at 6022 currently fails to boot. Needed: the owner of that change fixes the invariant, then `hub-build.sh` and `bun publish-hub.ts`, then `run-three.sh` again (cancel should be rechecked on the new hub).
3. The final screenshot of the interactions run (`e2e-process-v13-interactions.png`) could not be opened at the end (viewer hook timeout); pixel counts above. Viewed: `e2e-process-v13-cycle.png`.

## Changed files
Product (under `🧰️framework/`):
- `🔨️modules/🎭️actor/📮️shard-client/🧭️routing/🟦️.ts` (`recycleIdle`)
- `🔨️modules/🎭️actor/📮️shard-client/🧩️hub/🟦️.ts` (role)
- `🔨️modules/🎭️actor/📮️shard-client/📬️request-transport/🟦️.ts` (recycle on `original-source-close`)
- `🔨️modules/🎭️actor/📮️shard-client/🧪️tests/♻️shard-recycle/recycle.test.ts` (new law)
- `🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🖼️surface-views/🟦️.tsx` (edgeless surfaces)

Ticket folder: `verify-process-firefox-bidi.ts`, `publish-hub.ts`, `run-forest-render.sh`, `run-render-retry.sh`, `run-when-quiet.sh`, `run-three.sh`, `light-prefs.js`, `wasm-mem-preload.js`, `process-*.json` scenarios above, `footprint-role-cycle.txt`, `e2e-process-v13-*.png`. No `[DEBUG]` lines remain in product code; temporary outputs in `🗑️generated` were removed.
