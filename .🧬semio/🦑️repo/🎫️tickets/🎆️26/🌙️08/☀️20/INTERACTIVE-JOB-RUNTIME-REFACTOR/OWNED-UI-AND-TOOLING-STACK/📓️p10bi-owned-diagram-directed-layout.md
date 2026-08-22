# P10bi Owned Diagram Directed Layout

## Status

RED. The owned deterministic layout implementation and the shared-Worker source route are implemented and covered, but the dependency-removal packet is not accepted. Dagre remains installed because a real browser runtime round-trip has not been demonstrated, the UI publication authority has no explicit cursorized close/terminal-empty handshake, and the shared port has no reactive readiness signal.

## Implemented

- Added a dependency-free persistent directed-layout job with finite staged admission, paged storage, generation/deadline/fuel/cancel checks, stable merge ordering, duplicate detection, deterministic cycle breaking/rank assignment, crossing reduction, coordinate projection, and TB/BT/LR/RL transforms.
- Added exact bounded wire admission: at most 64 items and 16 KiB per input page, IDs limited to 512 UTF-16 code units with exact UTF-8 accounting, finite/safe-integer descriptor checks, hostile-input fault ownership, and at most 128 fixed-width output positions per page.
- Added a registry-owned worker factory used by the existing process-wide browser frame Worker. No Diagram subsystem Worker and no UI-thread layout fallback were added.
- Added the React hook route through `interactiveJobPort`; render/effect setup reads no source items, input capture occurs only in timed bounded host callbacks, and terminal publication is an O(1) paged-array proxy handoff.
- Kept the exact batch/test adapter on the same persistent job and removed it from the Diagram product barrel.
- Removed the obsolete Dagre declaration adapter and all Dagre source imports.
- Covered DAGs, cycles, parallel/disconnected/self edges, all four directions, variable dimensions, reversed-order determinism, source mutation/stale generation, more than 256 preview positions, 20k/20k setup/ingress/step/cancel, zero budget, finite close, malformed numeric/identity payloads, public-barrel absence, and animation-frame-stack exclusion.

## Exact Packet Files

- Added `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️layout.ts`.
- Updated `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️component.tsx`.
- Updated `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🧪️component.test.tsx`.
- Removed `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️implementation.d.ts`.
- Updated `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` for the test-only batch import and owned-result assertion.

The coordinated shared-Worker transport/registry files are owned and reported by the Phase 3 browser-worker packet, not this packet.

## Verification

- `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache`: PASS.
- `bun nx run @semio-tech/ui-react:lint --skip-nx-cache`: PASS.
- `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- --run '🧱️elements/📊️Diagram/🧪️component.test.tsx'`: PASS, 1 file / 31 tests.
- `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache`: PASS, 20 files / 703 tests.
- `bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker --skip-nx-cache`: PASS, 2 files / 22 tests.
- `bun nx run @semio-tech/framework-renderer-wgpu:check-browser-worker --skip-nx-cache`: PASS, UI boot bundle 4 modules and Worker bundle 30 modules.
- `git diff --check -- <five packet source paths>`: PASS.
- `rg -n 'dagre' 🧰️framework/🔨️modules/🖱️ui`: only the retained UI package-manifest dependency; no source import remains.
- No Cargo command was run.

## Dependency Census

- Accepted starting boundary: 141 exact identities = 63 Rust + 78 JavaScript.
- This packet's current delta: 0.
- Current boundary: 141 = 63 + 78.
- `dagre`, its `graphlib` reachability, and Dagre's private lodash reachability remain in `bun.lock` intentionally.
- The expected 140 = 63 + 77 boundary is not claimed.

## Blocking Residuals

1. Source tests and both UI/Worker bundles prove the route is statically connected, but they do not prove a live browser Worker round-trip through Wasm boot, OffscreenCanvas ownership, the shared frame Worker, output publication, cancellation, and teardown. The controlled browser window was not available and Cargo/Wasm construction was explicitly out of scope.
2. The UI `DiagramLayoutPublication` owns paged captured nodes/edges and positions. Completion performs an O(1) proxy handoff, but cancellation/fault/replacement can eventually abandon that authority through garbage collection because the shared port consumer contract has no explicit incremental UI-side `closeStep` and no terminal-empty acknowledgement. Dagre cannot be removed until that ownership handshake is explicit and tested.
3. `useDiagramLayout` reads `interactiveJobPort.status` only when its effect runs. `setInteractiveJobPort` and later `ready()` transitions do not publish a React-observable generation, so a Diagram mounted before browser boot can remain on its source positions forever unless its nodes, edges, or options identity changes. The shared port needs a bounded `subscribe`/`getSnapshot` readiness generation (consumed through `useSyncExternalStore` or an equivalent owned signal), including notifications for install, ready, quarantine, and close transitions.
4. Product preview pages are intentionally suppressed at the wire adapter while final positions drain one bounded page per Worker turn. The exact job's replaceable preview ring is covered, but live preview publication should remain disabled until the UI close handshake exists.

## Removal Condition

Remove Dagre from the UI manifest and remove the now-unreachable `dagre`/`graphlib` lock rows only after all blockers are closed by a real browser gate, an explicit bounded UI publication-authority drain protocol, and a reactive shared-port readiness generation. Then rerun frozen lock verification, the JavaScript identity census, UI gates, Worker gates, and the browser runtime harness.
