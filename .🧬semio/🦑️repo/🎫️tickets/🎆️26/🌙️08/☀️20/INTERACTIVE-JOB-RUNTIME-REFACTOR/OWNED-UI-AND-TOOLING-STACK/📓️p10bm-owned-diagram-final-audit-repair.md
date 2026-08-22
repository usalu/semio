# P10bm Owned Diagram Final-Audit Repair

## Outcome

The two Diagram-owned blockers from the P10bl independent audit are repaired. Worker ingress is a total, no-throw, transactional boundary over unknown input, and proxy-backed published results are retired only after a React commit acknowledges a different displayed authority or unmount. Dagre remains installed. This packet did not edit shared Port/browser-worker files, invoke Cargo, run a real browser, call ticket APIs, or use git-modifying commands.

## Owned Files

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️layout.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️component.tsx`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🧪️component.test.tsx`
- This report.

## Hostile Ingress Totality

`DiagramLayoutWorkerJob.ingest` now accepts `unknown` and contains every discriminator/property read inside one no-throw boundary. It rejects nullish values, primitives, arrays/functions as pages, invalid kinds/generations/booleans, missing/null/non-array `values`, oversized arrays, throwing/revoked accessors, non-record node/edge entries, and throwing record fields. Reentrant ingress is rejected while the current page owns validation.

Node and edge records are copied once into bounded temporary arrays. Record identity, numeric fields, offsets, item/page bytes, exact UTF-8 bytes, declared completion, cancellation, and fault state are all validated before a paged store, received counter, or persistent layout job changes. Only a fully admitted page commits its copied records and counters. A valid record followed by a throwing record and a valid early-complete partial page both fault with `close(... fuel: 1) === true`, proving no first-record store mutation or job construction escaped validation.

## Commit-Acknowledged Result Retirement

Terminal callbacks now only register ownership and request `setPublished`; they never retire the prior authority. The hook derives the authority actually displayed by the render and reconciles an owned-authority ledger in a dependency-scoped post-commit effect. When a replacement or source fallback commits, that effect asynchronously retires every no-longer-displayed authority one fixed page per macrotask. Unmount cleanup retires both committed and produced-but-never-committed authorities.

The regression uses `React.startTransition` plus a suspending successor. The old committed proxy remains readable after eight macrotasks while the replacement is uncommitted. Resolving the suspension commits the successor; eight bounded retirement turns then make the old proxy empty. This fails the prior callback-time disposal design and establishes commit-before-retirement ownership.

## Added Adversarial Coverage

- Null, undefined, boolean, numeric, string, symbol, array, function, and prototype-free pages.
- Missing, null, object, string, typed-array, and throwing-array `values`.
- Invalid optional completion and unknown kind.
- Nullish/primitive/array node entries.
- Throwing generation, values, values-length, node-id, and edge-target accessors.
- Valid-first/throwing-second node and edge pages with no partial store commit.
- Valid early-complete partial page with no commit or job construction.
- Suspended concurrent replacement, pre-commit proxy readability, post-commit bounded retirement, and unmount ownership.

## Final Gates

| Command | Result |
| --- | --- |
| `bun nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- --run '🧱️elements/📊️Diagram/🧪️component.test.tsx'` | PASS — 1 file, 40 tests |
| `bun nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | PASS |
| `bun nx run @semio-tech/ui-react:lint --skip-nx-cache` | PASS |
| `bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker --skip-nx-cache` | PASS — 2 files, 32 tests |
| `bun nx run @semio-tech/framework-renderer-wgpu:check-browser-worker --skip-nx-cache` | PASS — boot and frame-worker bundles |

The Worker tests remain fake/static protocol coverage. No live browser Worker, Wasm, or OffscreenCanvas lifecycle was executed or claimed.

## Dagre Decision

`dagre@0.8.5`, `graphlib@2.1.8`, and Dagre's lodash edge remain in the manifest and lockfile exactly as requested. No dependency-removal claim is part of this repair.
