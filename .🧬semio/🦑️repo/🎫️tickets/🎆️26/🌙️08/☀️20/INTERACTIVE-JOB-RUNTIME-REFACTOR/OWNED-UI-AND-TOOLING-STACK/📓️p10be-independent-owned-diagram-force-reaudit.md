# P10be Independent Owned Diagram Force Reaudit

## Verdict

**REJECT.** `d3-force`/`d3-quadtree` removal and the owned engine's internal cursors are materially improved, and all required non-browser gates below are green. However, the live rAF path still performs unbounded user/React publication after a completed 20,000-node projection, all three pointer callbacks synchronously invoke arbitrary user code, the public `tick()` adapter runs initialization/drag phases with an infinite deadline, and the bounded sampled identity is used as a unique `Map` key. The last point aliases valid distinct long IDs and can resolve an edge to the wrong node. These are acceptance blockers for the requested `<8 ms` hard ceiling and deterministic correctness.

The requested dependency boundary remains **observed** at **141 = 78 JavaScript + 63 Rust**, but is not accepted while these blockers remain.

## Evidence Read

Read the master plan and the preceding packet reports:

- `📓️p10ba-next-live-dependency-scout.md`
- `📓️p10bb-owned-diagram-force.md`
- `📓️p10bc-independent-owned-diagram-force-audit.md`
- `📓️p10bd-owned-diagram-force-repair.md`

Live implementation and tests audited:

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️component.tsx` (1,335 lines)
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🧪️component.test.tsx`
- UI React manifest and `bun.lock`

No production source, manifest, lock, Cargo file, or Git state was modified. No Cargo command or real-browser test was run.

## Reattack Findings

### 1. Cursorized construction and internal force work — partial pass

`OwnedDiagramForceSimulation` construction only stores references/configuration (`component.tsx:343-354`). The live effect constructs a lazy source and starts rAF without reading an array item (`812-829`, `1105-1143`). Initialization is generation-tagged and cursorizes nodes, incremental merge sorting, links, degree calculation, and link strength/bias (`560-620`). Its loop rechecks the deadline before the next unit. Charge, link, collision, and integration retain phase/cursor state and use a five-millisecond owned-work deadline in a six-millisecond frame budget (`656-779`).

The focused tests genuinely exercise this live source with Proxy numeric-index counters, fake rAF, and a synthetic `performance.now`; they include controlled/uncontrolled 20,000-node cases, a 5,000-node/5,000-edge full-force case, 3,001 selected nodes, stale frames, and oversized IDs. This is useful source-level evidence, not a tautology.

It is not sufficient for acceptance because the tests replace `HostReactFlow` whenever `nodes.length > 1,000` (`component.test.tsx:29-31`), and use trivial callback mocks. Thus their timings exclude the actual ReactFlow mount/reconciliation and arbitrary consumer callback work described below.

### 2. Notification publication violates the hard ceiling — blocker

The projection loop is cursorized and deadline-checked (`1115-1134`), but after it has assembled the entire array it calls either:

- `onNodesChangeProp?.(positionedNodes)` for controlled mode (`1137`), or
- `setInternalNodes(positionedNodes)` for uncontrolled mode (`1138`).

Both occur synchronously inside the listener invoked by `notify()` in `advanceFrame()` (`735-779`), after a frame's projection completes. A controlled callback is arbitrary external code and may synchronously scan 20,000 nodes or block; no deadline/cancellation boundary surrounds it. The uncontrolled state update similarly hands a new 20,000-node array to React on the frame path, and the large tests intentionally mock the actual host, so no timing evidence covers ReactFlow reconciliation/rendering.

There is exactly one listener invocation rather than a subscriber iteration loop, but it is still an unbounded external subscriber call hidden after projection. This fails the specified hard ceiling and the notification condition. Add a bounded publication handoff/acknowledgement protocol, or move this boundary outside the deadline-owned frame path, and prove it using a deliberately expensive controlled subscriber plus the real host.

### 3. Pointer callbacks do more than O(1) enqueue/coalesce — blocker

The simulation `drag()` method itself retains an array reference and coalesces in O(1); scheduled `stepDrag()` cursorizes pin/release (`401-417`, `622-654`). But every live handler also synchronously invokes external callback code:

- `onNodeDragStartPropRef.current?.(event, node, nodes)` (`1067`)
- `onNodeDragPropRef.current?.(event, node, nodes)` (`1076`)
- `onNodeDragStopPropRef.current?.(event, node, nodes)` (`1087`)

These receive the full selected-node array. They are not bounded by the Diagram callback and can scan 3,001+ selected nodes or block before the host callback returns. The existing test records only `events.push(...)`; it proves no local numeric-index reads before scheduling, but does not test an adversarial external handler. This contradicts the requirement that pointer start/move/stop only enqueue/coalesce O(1).

### 4. Public force `tick()` is explicitly unbounded — blocker

`DiagramForceSimulation.tick()` is public and `createDiagramForceSimulation()` is exported. `tick()` calls all three state machines with `Number.POSITIVE_INFINITY` and loops to completion (`445-449`). A 20,000-node direct call synchronously reads/recover/indexes/sorts the entire graph and processes every pending selected drag. The p10bd report labels it a batch adapter, but the current public contract does not limit it to tests or batch entry points. This conflicts with the re-audit requirement that every force tick phase observe the hard ceiling. The focused timing tests only drive scheduled rAF work; they do not bound a large public `tick()` call.

### 5. Bounded identity is not a safe unique identity — blocker

`forceIdentity()` samples at most 256 UTF-16 code units and produces `key = length:hashA:hashB:hashC:hashD` (`230-247`). That key is used as the only node lookup key (`347`, `571`, `582-583`, `646`). Consequently the implementation is bounded, but it does not preserve identity for valid distinct IDs.

For example, two 100,000-code-unit IDs that are identical except at index 1 have equal length and equal sampled values: for 256 samples the first two sampled indices are 0 and `floor(99999 / 255) = 392`, so index 1 is not sampled. They produce the same four deterministic hashes and key. The later node overwrites the earlier `nodeByIdentity` entry; a link endpoint for either ID resolves to the later node. This is not merely a negligible 128-bit hash collision: it is a constructible collision induced by the sampling rule. The oversized-ID test checks bounded rereads but does not construct colliding IDs or verify endpoint resolution.

Hot recovery and zero-distance jiggle do use stored numeric metadata (`recover` at `452-460`, `forceJiggle` at `261-266`) and do not reread IDs. The residual must nevertheless be stated as semantic aliasing, not only theoretical hash collision. A fixed-cost identity index needs deterministic collision disambiguation that retains exact identity semantics without rereading an arbitrary ID in the hot path.

### 6. Other contract checks

- Restart/stop callbacks are generation-bound, sole-handle scheduling is present, and the focused stale-frame/restart test passes (`401-475`, `711-779`).
- Force projection has no `find`/whole sort after initialization. Unrelated focus handling still uses `find` (`1153-1164`), but it is outside the force notification path.
- Controlled input is not mutated in the covered test; uncontrolled uses the existing internal state route. Publication timing remains the blocker above.
- `d3-force` and `d3-quadtree` have zero matches in the live Diagram declaration, UI manifest, and `bun.lock`. `d3-dispatch` and `d3-timer` remain reachable in `bun.lock` via `@xyflow/system` → `d3-zoom` → `d3-transition`/`d3-drag`; this shared XYFlow chain remains intact.
- No real browser was run. This audit does not claim native pointer capture/cancellation, hardware pointer cadence, real 20,000-node ReactFlow rendering, or subjective force-layout quality.

## Independently Run Gates

| Command | Result |
| --- | --- |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- --run 🧱️elements/📊️Diagram/🧪️component.test.tsx` | PASS — 1 file, 12 tests, 2.83 s. |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | PASS — 20 files, 684 tests, 6.10 s. |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | PASS. |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | PASS; only the Bun `NO_COLOR`/`FORCE_COLOR` environment warning. |
| `bun nx run @semio-tech/ui-react:check-ui-primitives --skip-nx-cache` | PASS — 0 violations, 2 allowlisted files. |
| `bun nx run @semio-tech/framework-renderer-react:test-quick --skip-nx-cache` | PASS — 4 files, 439 tests, 3.79 s. |
| `bun nx run @semio-tech/framework-renderer-react:lint --skip-nx-cache` | PASS — region/host-contract lint passed. |
| `bun nx format:check --files='…Diagram/🟦️component.tsx,…Diagram/🧪️component.test.tsx'` | PASS. |
| `bun install --lockfile-only --ignore-scripts --no-progress --no-summary --frozen-lockfile` | PASS (exit 0; output `Saved lockfile`); subsequent targeted diff was empty. |
| `bun ./📜️script.ts verify dependencies` | PASS — baseline 238, current 141, 97 removed, no additions. |
| `bun ./📜️script.ts verify dependencies list js` | PASS — 78 identities. |
| `bun ./📜️script.ts verify dependencies list rust` | PASS — 63 identities. |
| `bun ./📜️script.ts verify dependencies parity js` | PASS — 83 manifests, 263 external rows, 114 evidenced, 149 unowned advisory rows, 0 undeclared imports, 44 lock workspaces, 0 lock mismatches, 5 fixtures. |
| `git diff --check -- <Diagram source/test> bun.lock` | PASS — no output. |
| Targeted retired-identity, rejected-pattern, and `[DEBUG]` scans | PASS in live Diagram/UI manifest/lock scope; no `[DEBUG]` in Diagram source/tests. |

The initially attempted project name `@semio-tech/renderer-react` was invalid (`Cannot find project`); `bun nx show projects` identified and the audit then ran the actual `@semio-tech/framework-renderer-react` target above.

## Required Repair Before Reaudit

1. Remove unbounded external calls from rAF pointer and projection paths, with a bounded queued handoff that preserves controlled/uncontrolled semantics and cancellation/generation behavior.
2. Remove or restrict the public infinite `tick()` adapter so every exposed interactive force phase is deadline/fuel bounded.
3. Replace sampled-hash-as-identity lookup with exact, collision-safe deterministic identity handling; add a long same-length unsampled-position collision/end-point test.
4. Run a real-host 20,000-node test or browser pass that includes publication/reconciliation, and add adversarial slow/read-count subscriber tests. Do not claim browser pointer/cadence/visual results until they are actually run.
