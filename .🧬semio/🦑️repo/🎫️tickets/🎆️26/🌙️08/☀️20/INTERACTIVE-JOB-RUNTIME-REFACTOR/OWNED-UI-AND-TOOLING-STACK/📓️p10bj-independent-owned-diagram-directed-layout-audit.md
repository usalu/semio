# P10bj Independent Owned Diagram Directed-Layout Audit

## Verdict

**REJECT.** The worker-side directed-layout authority is mostly well-bounded, but the landed UI/port packet cannot yet replace Dagre safely. Its focused UI gate is currently red, mounted Diagrams do not react to port readiness, wire credits reject valid bounded input and silently exclude the documented maximum graph shape, output integrity is not exact, and UI publication ownership has no explicit close protocol. No real-browser execution was performed or claimed.

## Scope And Method

- Read the owned Diagram implementation, directed-layout codec/job, React hook and tests; the shared browser interactive-job port, scheduler, frame transport, worker source and tests; package/lock dependency rows; and the existing P10bi packet note.
- Did not modify product source, invoke Cargo, use ticket APIs, or run any git-modifying command.
- Worktree is concurrent and dirty. Findings below describe the files as inspected on 2026-08-22.

## Blocking Findings

### B1 — Focused Diagram Gate Is Red

`InteractiveJobPort` now requires `getSnapshot` and `subscribe` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔌️Ports/🟦️interactive-job.ts:36-47`), and `setInteractiveJobPort` unconditionally calls `port.subscribe` (`:74-81`). The Diagram test cleanup and its ready mock still provide only `status` and `submit` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🧪️component.test.tsx:118-120`, `:287-294`). Consequently the focused command fails all 31 tests before their assertions with `TypeError: port.subscribe is not a function` from `interactive-job.ts:79`.

This is a real test/fixture contract break, not a passing gate. It also means the claimed behavioral coverage is not executable in the current packet.

### B2 — React Does Not Subscribe To Readiness

The global port correctly forwards install and concrete-port status notifications (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔌️Ports/🟦️interactive-job.ts:62-85`), and the concrete browser port notifies on ready/quarantine/close (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🟦️typescript/🧵️browser-interactive-job-port.ts:56-72`, `:146-165`, `:206-225`). But `useDiagramLayout` only reads `interactiveJobPort.status` inside an effect whose dependencies are the node, edge and option identities (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️component.tsx:1645-1685`).

A Diagram mounted while the port is unavailable never reruns merely because the shared worker later becomes ready. It permanently returns source positions unless another input identity changes. The hook must consume `subscribe/getSnapshot` through `reactHostPort.useSyncExternalStore` (or an equally reactive owned signal) and prove install, ready, quarantine and close transitions.

### B3 — Descriptor Credits Are Neither Exact Nor Compatible With Advertised Bounds

The UI declares fixed aggregate input credits of `1600 * nodeCount + 4672 * edgeCount` (`Diagram/component.tsx:1651-1665`), while actual admitted wire records are up to `64 + UTF-8(id)` for nodes and `64 + UTF-8(id) + UTF-8(source) + UTF-8(target)` for edges (`Diagram/layout.ts:91-112`). A valid 512-UTF-16-unit astral identifier is 2,048 UTF-8 bytes, so valid one-record maxima are **2,112 bytes/node** and **6,208 bytes/edge**. The UI port aggregates actual `page.byteLength` and quarantines the entire port when it exceeds descriptor credit (`browser-interactive-job-port.ts:175-196`). Thus a valid high-byte identity can quarantine the process-wide worker route instead of producing an owned per-job rejection.

There is a second incompatible limit: the codec accepts 65,536 nodes and 65,536 edges independently (`Diagram/layout.ts:210`, `:1124-1129`), and the hook only individually rejects counts above those values (`Diagram/component.tsx:1648`). It nevertheless submits `inputItems = nodes + edges` (`:1651`), while the port rejects descriptor input items above 65,536 (`browser-interactive-job-port.ts:77`). A 65,536-node graph with even one edge, or the stated 65,536/65,536 shape, silently receives no lease and retains the unlaid-out source graph.

The contract must either calculate/validate an exact bounded aggregate before submission without violating UI-turn bounds, or publish a smaller coherent admissible graph/byte contract and reject it locally with explicit result ownership. Add boundary tests for one maximal UTF-8 node/edge and count totals of 65,536 and 65,537.

### B4 — Output Completion Does Not Prove Exact Position Coverage

`DiagramLayoutPublication.acceptOutputPage` accepts every individually valid position and overwrites the paged store without rejecting a duplicate index; `publish` only checks that ingress captures have their expected lengths (`Diagram/layout.ts:468-485`). It does not require every source index to appear exactly once, a monotonic page sequence, or output aggregate count equality. A duplicate-index `complete` output page followed by a complete terminal can therefore publish nodes for which positions were never supplied (they retain source coordinates).

The scheduler's normal implementation happens to emit sequential pages, but the UI side is the validation boundary for Worker output. Track exact output sequence/index ownership, reject duplicates/holes/out-of-order `complete`, and only publish after a terminal-complete plus a fully verified position set. Cover corrupted but per-page-credit-valid output.

### B5 — UI Publication Authority Has No Bounded Explicit Close Or Terminal-Empty Handshake

The worker job owns incremental close (`Diagram/layout.ts:1209-1226`) and the shared port cursor-drops worker consumers on close (`browser-interactive-job-port.ts:146-159`). In contrast, `DiagramLayoutPublication` owns three paged stores but exposes no `closeStep`/terminal-empty operation (`Diagram/layout.ts:448-551`), and the React effect's cancellation only calls `lease.cancel()` then drops its local reference (`Diagram/component.tsx:1680-1684`). Fault, cancellation, replacement, and terminal paths therefore rely on eventual garbage collection to reclaim UI-side captured pages. This does not meet explicit bounded ownership/disposal requirements and makes preview publication unsafe.

Add a cursorized UI publication close protocol invoked for cancellation, fault, replacement and post-handoff retirement; retain the authority until terminal-empty acknowledgement and test zero-fuel, finite-fuel and concurrent replacement paths.

## Other Audit Results

- **Worker boundedness/cancellation:** the scheduler and codec have finite slots, item/page credits, generation checks, output pagination and incremental worker close (`interactive-job-registry.ts:87-269`, `Diagram/layout.ts:1107-1234`). The static worker protocol suite passes, but this cannot discharge the UI findings above.
- **Determinism/parity:** source has an identity-sort layout and a reversed-input unit assertion (`Diagram/layout.ts:321-352`, `Diagram/component.test.tsx:193-203`). There is no current executable Dagre parity corpus or live browser round-trip; the historic three-node reference is not sufficient removal proof.
- **Public API:** concrete job classes and batch adapters are absent from the Diagram component/barrel assertion (`Diagram/component.test.tsx:396-402`). No direct Dagre/graphlib source import was found in the UI packet. This is non-blocking, subject to restored tests.
- **Browser runtime:** not run. The passing Worker tests use a fake Worker (`browser-frame-transport.test.ts:16-34`) and do not construct Wasm, transfer a real OffscreenCanvas, boot a browser Worker, or verify the complete layout lifecycle.

## Dependency-Removal Decision

`dagre` is still a direct UI dependency (`🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json:30-47`) and remains in the lockfile (`bun.lock:510-528`, `:2532`), retaining `graphlib` (`bun.lock:2894`) and Dagre's lodash edge. The source implementation no longer imports it, but the dependency packet cannot claim removal now. Other lodash-family rows also have independent tooling/transitive owners, so only Dagre's specific reachability may be considered after a fresh frozen-lock census.

## Commands Run

| Command | Result |
| --- | --- |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- --run '🧱️elements/📊️Diagram/🧪️component.test.tsx'` | **FAIL** — 1 file, 31 failed, all rooted at missing `port.subscribe` test mocks. |
| `bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker --skip-nx-cache` | PASS — 2 files, 22 tests; fake/static protocol coverage only. |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | PASS. |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | PASS. |

## Exact Acceptance Gates

1. Repair all Diagram test ports/mocks to satisfy the current interface, add reactive readiness tests, and make the focused Diagram suite pass.
2. Subscribe `useDiagramLayout` to the installed shared port status and demonstrate submit/recovery behavior across unavailable → ready, install replacement, quarantine and close.
3. Make descriptor item/byte credits mathematically exact for every accepted source shape, or explicitly and locally reject a coherent smaller shape before worker submission. Test maximal UTF-8 records and count/byte boundaries without quarantining unrelated jobs.
4. Enforce exact worker-output sequence and one-position-per-node coverage before publication; add malformed duplicate/hole/early-complete cases.
5. Implement and test explicit incremental UI-publication disposal with terminal-empty/close acknowledgement for cancellation, fault, replacement and success handoff.
6. Run the focused UI suite, full relevant Worker suite, typecheck and lint after the repair; add a real browser Worker + Wasm + OffscreenCanvas lifecycle gate covering layout output, cancellation and teardown. Do not call it passed until it is actually run.
7. Only then remove `dagre` from the UI manifest, regenerate/freeze the lockfile, prove `dagre`/`graphlib` and the Dagre-only lodash reachability are gone, and rerun the identity census plus all gates above.
