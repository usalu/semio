# P10bn Independent Owned Diagram Directed-Layout Final Audit 2

## Verdict

**REJECT.** The two P10bl blockers are genuinely repaired and all focused gates now pass, but the replacement-retirement design still leaks a proxy-backed result when React abandons an uncommitted replacement. That is an explicit required hostile lifecycle case and violates the bounded ownership contract.

## Scope And Method

Audited only the P10 Diagram-owned repair in `📓️p10bm-owned-diagram-final-audit-repair.md`, the preceding rejection `📓️p10bl-independent-owned-diagram-final-audit.md`, the governing Phase 10 plan, and the Diagram implementation/tests plus direct worker-protocol integration. No product source, shared P3 worker/scheduler files, manifests, lockfiles, Cargo commands, ticket APIs, or git-mutating commands were used.

## P10bl Blockers Rechecked

| Prior blocker | Independent evidence | Result |
| --- | --- | --- |
| Hostile worker ingress throws or accepts invalid pages | `DiagramLayoutWireJob.ingest` accepts `unknown`, places every page discriminator and property read in the `try` boundary, requires a non-array object and an actual `Array` for `values`, and captures temporary arrays before changing stores/counters/constructing a job (`layout.ts:1266-1301`). The focused suite exercises nullish values, primitives, missing/null/non-array `values`, throwing getters, a throwing values-length proxy, malformed node/edge entries, early completion, and valid-first/throwing-second records. Every case faults without throw; commit occurs only after capture/complete validation. | **Pass** |
| Published result retired before React commit | Terminal callbacks now only add the result to the ownership set and call `setPublished` (`component.tsx:1689-1704`). Retirement is performed from the post-commit effect keyed by the actually displayed authority (`:1721-1728`). The suspended-transition test holds the old proxy readable for eight macrotasks before resolving/committing the successor, then confirms the old proxy drains. | **Pass, subject to the new abandoned-work defect** |

## New Blocking Finding: Abandoned Concurrent Replacement Is Never Retired

`onTerminal` adds every live completed result to `ownedResultsRef` before requesting state publication (`component.tsx:1702-1704`). A result is removed only by the effect at `:1721-1728`, which runs only after a render commits, or by unmount cleanup at `:1729-1734`.

For an interrupted/abandoned concurrent replacement, React does not commit the render that selected the successor. The old authority remains committed, so the retirement effect does not run for the new result. The new proxy-backed stores remain in `ownedResultsRef` and are not incrementally drained until a later displayed-authority transition or unmount. A long-lived screen can retain every abandoned result indefinitely.

The regression at `component.test.tsx:456-505` covers only a suspended replacement that is subsequently released and committed (`:496-503`). It never abandons/interrupts the transition, asserts the uncommitted successor drains, or checks the ledger after an abort. There is no other post-request acknowledgement/cancellation path that could initiate close for that result.

Required repair evidence:

1. Add a durable ownership protocol that distinguishes requested, committed, abandoned, and unmounted authorities without releasing the previous committed proxy early.

2. Add a concurrent regression that starts a suspended successor, verifies the old proxy stays readable, abandons/replaces the work without committing the successor, and proves the uncommitted successor retires one bounded `closeStep` turn at a time.

3. Include stale-terminal and cancellation races so only the matching live generation can become owned, then rerun all gates below.

## Other Required Properties Reattacked

| Property | Result |
| --- | --- |
| Output integrity | **Pass.** Publication enforces exact expected sequence and source-index progression, exact 32-byte positions, no zero-progress page before coverage, and exact terminal-complete coverage (`layout.ts:515-548`). Worker terminal is withheld until every output page is emitted, including one empty sequence-1 page for zero nodes (`:1325-1345`, `:1367-1373`). |
| Credits and page bounds | **Pass.** Count-only reserve credits are bounded at 65,536 total items / 256 MiB and ingress recomputes actual UTF-8 record bytes. Focused tests cover maximum Unicode records and bounded pages. |
| Explicit close and terminal acknowledgement | **Pass for paths reaching close.** Publication close stages positions, edges, nodes, terminal; the consumer exposes `closeStep` and `terminalIsEmpty`. Published result close releases at most one page from each store per invocation. The abandoned-result finding is the remaining ownership gap. |
| No extra Diagram Worker/scheduler/UI fallback | **Pass in directed-layout path.** `useDiagramLayout` only submits to the installed process-wide interactive port; worker construction is internal to the registry. Existing force-simulation timers are separate pre-existing Diagram functionality, not a directed-layout fallback. |
| No synchronous layout product API | **Pass.** The product component/barrel does not export worker constructors, the concrete job, or the batch helper. The only non-test-tree batch-helper use is within an `import.meta.vitest` block in the React target index. |
| External type exposure | **Residual pre-existing scope limitation, not introduced by P10bm.** Diagram's broader public React surface re-exports `@xyflow/react` types and `useDiagramLayout` uses its `Node`/`Edge` inputs. P10bm's added layout wire types are owned. The residual conflicts with the repository-wide zero-external-type end state but is outside this narrow repair. |
| Dagre | **Not removed, as required.** No `dagre` or `graphlib` source import exists in the Diagram packet; manifest/lock rows remain untouched. |

## Commands Executed

| Command | Result |
| --- | --- |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- --run '🧱️elements/📊️Diagram/🧪️component.test.tsx'` | **PASS** — 1 file, 40 tests. |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | **PASS** — Nx emitted its flaky-task notice. |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | **PASS**. |
| `bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker --skip-nx-cache` | **PASS** — 2 files, 32 tests; fake/static worker protocol coverage only. |
| `bun nx run @semio-tech/framework-renderer-wgpu:check-browser-worker --skip-nx-cache` | **PASS** — boot and frame-worker bundles built. |

No live browser Worker, Wasm, or OffscreenCanvas lifecycle was run or claimed.

## Acceptance Condition

Do not accept this packet until the abandoned/interrupted concurrent-replacement ownership leak is fixed, demonstrated under React concurrency, and the gates above are green again. The current repair resolves P10bl's two reported defects, but does not satisfy the requested full lifecycle proof.
