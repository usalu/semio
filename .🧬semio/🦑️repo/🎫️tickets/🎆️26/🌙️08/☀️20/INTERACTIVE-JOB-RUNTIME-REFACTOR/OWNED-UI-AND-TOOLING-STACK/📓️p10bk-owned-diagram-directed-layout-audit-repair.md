# P10bk Owned Diagram Directed-Layout Audit Repair

## Outcome

The Diagram-owned directed-layout packet now discharges B1–B5 from the P10bj independent audit against the finalized shared closeable-consumer contract. Dagre remains installed. No Cargo command, real-browser claim, ticket API, git-modifying command, or shared Port/browser-worker edit was made by this packet.

## Owned Files

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️layout.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🧪️component.test.tsx`
- This report.

## Repair

### Reactive Port Lifecycle

`useDiagramLayout` consumes the stable `{ status, revision }` snapshot through `reactHostPort.useSyncExternalStore`. A mounted Diagram now resubmits or cancels on unavailable → ready, ready → ready installed-port replacement, quarantine, and close. Test ports implement the full current `getSnapshot`/`subscribe`/`observeConsumerTurn` interface.

### Exact Count And Credit Contract

Admission is computed from counts only; render/effect performs no identity scan.

| Credit | Exact declared maximum |
| --- | ---: |
| Identifier | 512 Unicode characters |
| Node record | `64 + 512 × 4 = 2,112` bytes |
| Edge record | `64 + 3 × 512 × 4 = 6,208` bytes |
| Input items | `nodeCount + edgeCount <= 65,536` |
| Reserved bytes | `nodeCount × 2,112 + edgeCount × 6,208 + nodeCount × 32 <= 256 MiB` |
| Input page | 64 items and 16 KiB |
| Output page | 128 items and 16 KiB |

`diagramLayoutCredits` returns an explicit `items` or `bytes` rejection, and the hook exposes `layoutStatus: "rejected"` plus `layoutRejection`. The same count/byte shape is rejected by the Diagram Worker job constructor. Maximal astral identifiers are accepted and accounted as 2,048 UTF-8 bytes; a 513th Unicode character is rejected during bounded ingress.

### Exact Output And Terminal Publication

The publication boundary requires sequence numbers beginning at one, exact monotonic source indices, one position per node, no duplicates, holes, no-progress pages, early completion, extra pages, or inconsistent host/payload completion. Unknown or hostile payloads fault without throwing. Result transfer occurs only after a matching terminal-complete and complete exact position coverage.

The zero-node Diagram Worker job emits exactly one empty terminal-complete position page before exposing its complete terminal. The host accepts that explicit empty output and publishes empty nodes/edges.

### Explicit Ownership Retirement

`DiagramLayoutPublication` implements the shared retained consumer's `closeStep()` and `terminalIsEmpty()` witnesses. Each close turn releases at most one fixed 128-item page and advances through positions, untransferred edges, untransferred nodes, retained terminal, and complete stages. Cancellation, fault, replacement, port close/quarantine, rejected submit, and terminal completion therefore have explicit finite retirement.

On success, only the result node/edge stores transfer into `DiagramLayoutPublishedResult`; ingress positions and the retained terminal remain with the publication and are drained by the shared port. Superseded and unmounted published results retire asynchronously one bounded page per macrotask, never through a synchronous React cleanup loop.

## Added Coverage

- Full current test-port contract and restoration of the prior 31-test suite.
- Reactive unavailable/ready/install-replacement/quarantine/close lifecycle.
- Exact 65,536/65,537 item boundaries and exact 256 MiB reservation boundary.
- Maximal UTF-8 node/edge records and overlong identities.
- Duplicate, hole, out-of-order, no-progress, hostile-payload, and early-complete output.
- Terminal-gated success, explicit item/byte hook rejection, cancelled close, transferred success close, replacement cancellation, and terminal-empty acknowledgement.
- Explicit zero-node result page and terminal semantics.

## Final Gates

| Command | Result |
| --- | --- |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- --run '🧱️elements/📊️Diagram/🧪️component.test.tsx'` | PASS — 1 file, 38 tests |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | PASS |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | PASS |
| `bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker --skip-nx-cache` | PASS — 2 files, 30 tests |
| `bun nx run @semio-tech/framework-renderer-wgpu:check-browser-worker --skip-nx-cache` | PASS — boot and frame-worker bundles |

The Worker tests remain fake/static protocol coverage. No live browser Worker, Wasm, or OffscreenCanvas lifecycle was executed or claimed.

## Dependency Decision

`dagre@0.8.5`, `graphlib@2.1.8`, and Dagre's lodash edge remain in the UI manifest/lock exactly as requested. Dependency removal remains a separate acceptance packet after any required real-browser and parity gates.
