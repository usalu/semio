# P10bl Independent Owned Diagram Directed-Layout Final Audit

## Verdict

**REJECT.** The repaired packet has strong bounded-layout and lifecycle coverage, but it still
does not fail closed at its worker ingress boundary and may retire a published proxy-backed result
before React has committed the replacement. Either defect invalidates the owned Diagram acceptance.
The current shared browser-port suite also fails, so global browser-runtime acceptance is separately
unavailable.

## Scope And Method

Read-only source audit of the owned Diagram packet, its direct shared-port/worker integration, and
the repair report `📓️p10bk-owned-diagram-directed-layout-audit-repair.md`. No Cargo command,
browser run, ticket API, or git mutation was used. This audit did not claim a real browser Worker,
Wasm, or OffscreenCanvas result.

## Owned Strengths Confirmed

| Requirement | Evidence | Result |
| --- | --- | --- |
| Count, UTF-8, and process credits | `diagramLayoutCredits` derives a count-only worst-case reservation; ingress recomputes each admitted page's UTF-8 byte size. Maxima are 512 Unicode characters, 2,112-byte nodes, 6,208-byte edges, 65,536 items, and 256 MiB. | Pass |
| No render/effect whole-input scan | `useDiagramLayout` reads only array `.length` during render and constructs the publication in its effect; item identity traversal is deferred to `readInputPage`. | Pass |
| Readiness revision and replacement | The stable facade's snapshot contains a monotonically renewed revision; `setInteractiveJobPort` publishes after every installation. The hook depends on `portSnapshot`, cancels its old lease in cleanup, and resubmits. Focused coverage includes ready-to-ready replacement. | Pass |
| Exact output coverage | `DiagramLayoutPublication.acceptOutputPage` enforces sequence one upward, exact source index progression, finite coordinates, byte/item equality, no early/extra/empty-progress pages, and terminal-complete coverage before transfer. | Pass |
| Zero-node completion | `DiagramLayoutWireJob.takeResultPage` emits exactly one `{ complete: true, sequence: 1, values: [] }`; terminal publication is withheld until it is emitted. | Pass |
| Bounded consumer retirement | Publication `closeStep` releases positions, edges, nodes, then the retained terminal; it is wired for cancellation, fault, port replacement/close, and success. Superseded/unmounted result cleanup is one asynchronous bounded page step per macrotask. | Conditionally pass; see React-commit defect below |
| Public API and deterministic layout | Worker construction remains internal to the registry; product test coverage asserts it is absent from the public Diagram module. The persistent job sorts identity deterministically and projects positions back by source index. | Pass |

## Blocking Defect 1: Hostile Worker Ingress Throws Or Is Accepted

`DiagramLayoutWireJob.ingest` dereferences `page.generation` before a type guard or `try` block
at `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️layout.ts:1266`, and reads
`page.values.length` before the subsequent `try` at line 1277. It never requires
`Array.isArray(page.values)`.

Fresh direct execution against a zero-node descriptor produced:

| Payload | Observed result |
| --- | --- |
| `null` / `undefined` | throws on `page.generation` |
| missing `values` / `values: null` | throws on `page.values.length` |
| proxy getter for `generation` or `values` | getter exception escapes |
| `values: {}` / `values: ""` | returns `true` and creates the zero-count job |
| numeric/string/array primitive page | returned false in the exercised cases, but this is not a complete hostile-object guard |

The packet explicitly requires hostile payloads to fault without throwing. This is an owned worker
constructor/ingress boundary, not merely an outer transport concern. It needs an object discriminator,
an array discriminator, and all hostile property reads within a no-throw fault boundary before any
state transition.

## Blocking Defect 2: Published Result Can Be Retired Before React Commit

In `useDiagramLayout`, terminal handling calls `setPublished(next)` at
`🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️component.tsx:1710` and immediately schedules
`retireDiagramLayoutResult(previous.authority)` at line 1711. The source-change branch does the
same at lines 1665-1669. `DiagramLayoutPublishedResult` exposes proxy arrays backed by its paged
stores; retirement releases those stores. React is permitted to defer a concurrent state commit, so
the old committed tree can still read the old proxy after its timer begins draining it. The existing
tests use synchronous `act` behavior and do not establish commit-before-retirement ownership.

Retirement must be acknowledged only from a committed replacement/unmount effect (or equivalent
post-commit ownership ledger), not from the terminal callback that merely requested a state update.

## Current Shared Browser-Port Residual: Separate From Diagram Ownership

The shared command below presently fails with two tests:

```text
bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker --skip-nx-cache
FAIL browser interactive job port > serves two concurrent instances and rejects late and future messages
FAIL interactive Worker scheduler > cancels between output pages without publishing the remainder
2 failed, 28 passed
```

The first has an identifiable shared-port defect: before branching on `"job-output-page"`,
`BrowserInteractiveJobPort.receive` validates `message.status` as if every worker message were a
terminal (`browser-interactive-job-port.ts:157-160`). Output messages have no status, so the port
quarantines before its output branch. This code is outside Diagram ownership. It means **global
browser-runtime acceptance is REJECT independently**, even if the Diagram-specific defects were
fixed. The second failure was recorded as a current scheduler residual; no Diagram conclusion relies
on attributing it.

## Gates Re-run

| Command | Result |
| --- | --- |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- --run '🧱️elements/📊️Diagram/🧪️component.test.tsx'` | Pass: 1 file, 38 tests |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | Pass (Nx also emitted a flaky-task notice) |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | Pass |
| `bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker --skip-nx-cache` | Fail: 2 of 30 tests, detailed above |
| `bun nx run @semio-tech/framework-renderer-wgpu:check-browser-worker --skip-nx-cache` | Pass: boot and frame-worker bundles |

## Dagre Decision

`rg` found no `dagre` or `graphlib` product-source import under
`🧰️framework/🔨️modules/🖱️ui`; `dagre` appears only in the React target manifest and the lockfile.
The owned deterministic layout is implemented locally in `🟦️layout.ts`. Therefore Dagre, graphlib,
and their lodash edge are technically removable from this packet's runtime dependency graph. This
audit made no manifest/lock modification, and dependency removal must be rechecked after the two
blocking correctness defects and the global shared-port gates are repaired.

## Required Acceptance Evidence

1. Add adversarial ingress tests for nullish, primitive, missing/non-array values, and throwing
   accessors; demonstrate all fault without throw or premature job construction.
2. Add a concurrent/deferred-commit test proving the outgoing published result stays readable until
   React has committed ownership of its successor, then retire it in bounded turns.
3. Restore the two shared browser-worker tests and rerun the four gates above.
4. Only then remove Dagre in its own verified dependency packet if no other workspace consumer uses it.
