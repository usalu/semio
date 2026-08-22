# P10bb Owned Diagram Force

**AUDIT-READY.** Diagram now runs a repository-owned deterministic force engine behind its existing owned contracts. The retired `d3-force` identity and its orphaned `d3-quadtree` resolution are absent from executable source, manifests, declarations, and `bun.lock`. The dependency ratchet is at the required **141 total / 78 JavaScript / 63 Rust identities**. All bounded source, focused behavior, real Diagram/renderer consumer, type, lint, primitive-policy, formatting, frozen-lock, dependency, parity, manifest-source, exact-scan, and diff gates pass.

## Owned Runtime

- The existing `DiagramForceConfig`, `DiagramForceNode`, `DiagramForceLink`, and `DiagramForceSimulation` interfaces remain the only force boundary. No external implementation type is exported or referenced.
- Node processing, link processing, and pair processing use stable code-point identity order. Missing or non-finite positions and velocities recover from repository-owned deterministic identity hashes.
- The engine implements the consumed link distance/degree strength and bias, many-body charge, pair collision, center spring, velocity retention, alpha decay/target, manual tick, restart, stop, and exact pinned-axis behavior.
- Browser work is cooperative and supersedable: at most four ticks, six milliseconds, 2,048 node integrations, 2,048 links, and 2,048 pairs per tick. A simulation owns at most one animation-frame or timeout handle; stop and effect cleanup cancel the handle and reject stale callbacks.
- `updateIntervalMs`, formerly inert, now throttles Diagram position projections while preserving an initial and completion notification. Manual `tick()` remains listener-silent.
- Server/static rendering does not schedule work. The browser fallback uses a cancellable timeout only when animation frames are unavailable.
- Diagram no longer performs the former synchronous 300-tick mount loop. Controlled consumers receive proposals without input mutation; uncontrolled consumers commit through the existing internal state path. Drag start/reheat, selected-node pins, and drag-stop unpin remain on the existing Diagram callbacks.

## Behavioral Evidence

The focused real-DOM suite records and checks the pre-removal three-node reference fixture after 30 ticks:

| Node | Retired reference position |
| ---- | -------------------------- |
| `a`  | `(-11.574003, -31.790995)` |
| `b`  | `(28.684336, 7.358646)`    |
| `c`  | `(-24.941755, 22.616164)`  |

The owned replay is deterministic across reversed node/link input order and remains inside the bounded fixture tolerance. Separate fixtures prove link attraction, charge repulsion, collision separation, center movement, finite recovery, multi-node drag pins and unpin movement, exact-once scheduling, notification throttling, stale-frame cancellation, and a 20,000-node cooperative frame budget. Real Diagram coverage proves controlled-lag projection, unmount cancellation, and SSR/static-render safety. The renderer React suite exercises the real downstream NodeGraph Diagram consumer.

## Changed Paths

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🧪️component.test.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️implementation.d.ts`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`
- `bun.lock`
- regenerated `📊️p10-manifest-source-parity.json`
- regenerated `📓️p10-manifest-source-parity.md`
- this report

The shared manifest, lock, test configuration, and parity artifacts also contain concurrent Phase 10 work; this packet preserved those edits. Dagre, XYFlow, DnD, graph public APIs, i18n, router, resizable, graphics, PDF, and `xstate` were not altered by this packet.

## Dependency And Lock Outcome

Bun lock-only reconciliation removed the direct workspace row and the two now-unreachable resolutions. `d3-dispatch` and `d3-timer` remain in `bun.lock`; their reachability is explicit through `@xyflow/system` → `d3-zoom` → `d3-transition`/`d3-drag`. No lifecycle script ran during reconciliation or frozen validation.

## Final Gates

| Gate                                                                       | Result                                                                                                                                                         |
| -------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Bun lock-only reconciliation with `--ignore-scripts`                       | PASS                                                                                                                                                           |
| Bun frozen install with `--ignore-scripts`                                 | PASS                                                                                                                                                           |
| UI React `test-quick --skip-nx-cache`                                      | PASS — 20 files, 679 tests                                                                                                                                     |
| UI React `typecheck --skip-nx-cache`                                       | PASS                                                                                                                                                           |
| UI React `lint --skip-nx-cache`                                            | PASS; only Bun's existing color-environment warning                                                                                                            |
| UI primitive policy                                                        | PASS — 0 violations, 2 existing allowlisted files                                                                                                              |
| Renderer React `test-quick --skip-nx-cache`                                | PASS — 4 files, 439 tests                                                                                                                                      |
| Renderer React lint                                                        | PASS — region/host-contract lint                                                                                                                               |
| Exact changed-file Nx format check                                         | PASS                                                                                                                                                           |
| Dependency freeze                                                          | PASS — historical 238, current 141, removed 97, no additions                                                                                                   |
| JavaScript dependency list                                                 | PASS — 78 identities                                                                                                                                           |
| Rust dependency list                                                       | PASS — 63 identities; no Cargo command ran                                                                                                                     |
| JavaScript dependency parity                                               | PASS — 83 manifests, 263 external rows, 114 evidenced rows, 149 advisory unowned rows, 0 undeclared imports, 0 lock mismatches, 5 fixtures, 44 lock workspaces |
| Manifest/source audit regeneration                                         | PASS — 64 manifests, 575 direct rows, 263 external rows, 74 no-package-scope-evidence candidates                                                               |
| Executable/config/manifest scan for the retired identity and adapter names | PASS — zero matches outside historical dependency evidence                                                                                                     |
| Exact lock scan for the retired identity and orphan                        | PASS — zero matches                                                                                                                                            |
| Explicit retained dispatch/timer and XYFlow reachability scan              | PASS                                                                                                                                                           |
| Packet `[DEBUG]` scan                                                      | PASS — zero matches                                                                                                                                            |
| Targeted `git diff --check`                                                | PASS                                                                                                                                                           |

Per the scout, renderer-wide typecheck is not a packet gate because its independently documented graphics, shell, WASM, and worker baseline is red. The bounded UI typecheck plus real renderer consumer test are green. No Cargo command, modifying Git command, ticket metadata edit, or browser run occurred.

## Browser-Only Residual

JSDOM and fake animation frames prove the owned contracts, ordering, pinning, cooperative budget, throttling, cleanup, and SSR guard. They cannot certify native pointer capture/touch gesture delivery, production animation cadence under load, or subjective visual force quality. A real-browser drag/cancellation and large-graph visual-cadence pass remains before those browser-specific properties can be called closed.
