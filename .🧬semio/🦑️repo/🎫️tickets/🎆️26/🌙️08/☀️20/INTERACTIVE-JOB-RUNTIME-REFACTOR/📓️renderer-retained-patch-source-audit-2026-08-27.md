# Retained Renderer Patch Source Audit

## Read Boundary

Read the coordinator packet and the complete current `UiDocumentStore` implementation/tests. Read the live `PluginRuntime` patch decoder, retained apply, snapshot reconstruction/hash, recursive BuiltNode reconstruction, patch-ACK flow and actual call sites. No implementation changes in these regions yet.

## Confirmed Work Frontiers

1. Wire decode currently maps the whole patch and calls whole `decodePackValue` for each encoded field. `set-children` also maps the complete ID list. Fixing only `applyUiPatch` would leave this synchronous ingress work intact.
2. Patch accounting traverses all operations and constructs a whole `TextEncoder` output for each string. Text quota accounting separately traverses component collections and strings.
3. Apply constructs `new Map(state.nodes)`. Remove spreads all children onto its work stack in one step. Fixed-size record replacement itself shares unchanged component/child values; it need not recursively copy them.
4. Validation walks all nodes/children, stores long sibling keys in native `Set`, and emits all violation objects synchronously. The current walk pushes children in forward order and pops them in reverse order; the exact violation ordering must be preserved against the reference tests.
5. `PluginRuntime` copies patch operations, then later materializes the entire node array, recursively rebuilds BuiltNode children, and stringifies/encodes/hashes the whole snapshot for each requested refresh body. These are genuine second-pass costs after apply.
6. Store notification compares every old/new record to rediscover touched IDs. Current listener iteration is synchronous and native `Set` iteration admits reentrant additions; a retained contract must explicitly define its replacement delivery frontier.
7. ACK currently follows synchronous retained acceptance. A future asynchronous cursor must not ACK receipt or mere decoding; exact accepted root/revision publication must precede ACK.

## Preservation Requirements

- Existing `loadSnapshot` is intentionally unchecked. Native rejection fixtures load invalid snapshots and assert the subsequent patch rejection. Do not introduce a new hydration rejection or silently normalize those fixtures.
- The current TypeScript table uses insertion-ordered `Map`; retained snapshot hashing observes that order. A sorted persistent tree alone is not drop-in equivalent. An insertion-ordered persistent numeric index needs either a second ordinal index or an explicitly tested common canonical-order contract change.
- `UiNodeId` is generated as JavaScript `number`; actual wire ingress rejects values outside nonnegative safe integers. Do not silently truncate to u32 in a radix implementation.
- The old reference applicator belongs in tests only once the interactive cursor is wired. It is not an acceptable completion callback for a retained wrapper.
- Snapshot captures must keep old roots readable while a candidate is prepared. Root, revision and hash publication must be one acceptance operation. Supersession before acceptance must retire private candidates without notifying.
- Existing language-neutral rejection categories/order, numeric finiteness, section rules, quotas, cycle handling and unknown-node behavior remain authoritative.

## Proposed Checkpoint Order

1. Schema-first retained numeric ordered-index law fixture, preserving insertion order and old captured readers. Use fixed-size immutable nodes and one spine/rotation action per grant, with explicit cancelled-cursor release. Compare with native JavaScript `Map` as the existing platform oracle. No whole-array root copy or unbounded overlay chain.
2. Typed patch transaction accounting/apply/validation/retirement cursors using that index. Long UTF-16/UTF-8 fields and sibling-key comparison need byte-level progress; a native string `Set` insertion is not a certified byte-bounded substitute.
3. Pack ingress and exact canonical JSON/FNV streaming share the accepted root. Reconcile authored BuiltNode input with explicit traversal frames. The renderer callback consumes already-prepared state instead of reconstructing it.
4. Store-owned publication and notification frontier, then actual PluginRuntime ACK/refresh integration. Add reentrant subscription, cancellation at every phase, stale base and cross-surface identity fixtures before wiring.
5. Full existing corpus and React tests, native parity where applicable, fresh Wasm/browser latency/interaction verification. GC/allocation and actual browser eight-millisecond constraints require runtime evidence; a logical item counter does not certify them.

## Current Dependency

The Board/loader packet is source-coherent and verified separately. Seven tutorial TypeScript errors await the other executor's actual registered full-local interaction capture/restore runtime, not merely schema declarations. This audit does not claim that the retained patch implementation or the tutorial API is complete.
