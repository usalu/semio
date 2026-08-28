# Canonical Retained Return Paging

## Coordinated Neutral Storage Boundary

The runtime coordinator approved one replacement return schema and explicitly rejected an interim whole-copy WIT handoff. Dag owns native admission, the canonical return authority schema, and the neutral storage schema/Rust/WIT extraction. Demonstrator owns TypeScript storage and subsequent transport adoption.

Released names are `actor/📄️page/ActorBytePage`, `ActorBytePageBlock`, and `ACTOR_BYTE_PAGE_BYTES = 4096`. One page has length 0–4096 and 64 named blocks of eight unsigned u64 little-endian words. Bytes at or beyond the used length are zero. WIT uses the type-only `byte-page` interface; command input becomes `{cursor, page}`. No command kind, factory, cursor, 64-page command cap, or eager whole-page-array builder conveys return authority.

At the initial inspection, the released directory was not yet present. The three actual shared files subsequently appeared and were read in full before TypeScript tests and implementation. Storage RED3/GREEN98 and command-shape RED2/GREEN98 are now recorded in `📓️actor-byte-page-storage-2026-08-27.md`. No parallel schema or WIT declaration was authored.

The authored TypeScript consumer census found direct block construction/access only in ShardClient and its inline tests. PluginRuntime and the WGPU plugin bridge call the command-page builder and forward its result; the materializer forwards the dedicated command-page argument. None of those callers independently reconstructs block fields.

TypeScript validation of arbitrary foreign object fields requires own-key reflection, which is not bounded for hostile wrappers. Fixed native page conversion and strict unknown-wrapper rejection must remain distinct from an 8ms/allocation certificate. Unknown original wrappers remain caller-owned; selected field validation is not whole-record retirement. Independent tests will use Node Buffer for exact LE bytes, lengths, zero padding, missing/foreign fields, unsigned bounds, and the maximum page payload.

## Observed Boundary

The actual Actor TurnResult still owns opaque Vec byte fields for UI patches, effects and command ingress. Its receipt fields are now bounded, but pack_encode writes the whole opaque bodies. The typed WIT result still lifts whole UI patch and effect lists. Both the runtime coordinator and native owner confirmed that no canonical general return pager exists.

The existing ActionBus ToolWirePage has fixed 4096-byte storage and RetainedToolWireInput has explicit admission, seal and close. Its admission is factory-bound and its Vec capacity is reserved from the whole declared extent. These are useful storage/retirement references, not permission to reuse ToolWireAdmission, factory witnesses or AppCommand opcodes for return authority. A neutral storage extraction would require coordinated source ownership and an explicit backing-allocation bound; merely renaming the current container is insufficient.

The tested host output cell is the correct pre-dispatch response reservation. It does not make jco lifting, structured clone, raw-object enumeration or whole-array release bounded. The worker also needs an exact owner before normalization can fail.

## Proposed Canonical Direction — Not Mounted

The production path should retain variable return content before it crosses WIT, not first turn a whole result into arbitrary JavaScript objects and later try to discover a bounded retirement plan. Fixed control receipts stay inline. Variable UI operation bytes, effect payloads, command-ingress details and fault/status payloads must cross the existing reactor protocol in bounded pages.

The same canonical reactor poll/event protocol must own page advancement and exact input-release acknowledgement. No second guest export, host-only alternate wire, command-factory witness, or compatibility result union is proposed. The eventual schema must replace the variable result fields consistently in Actor, Kernel, WIT, generated worker and both host consumers.

## Required Authority And Ownership Laws

1. Before executing a semantic turn, reserve its exact native return owner and the host response cell. Admission failure preserves all existing roots and does not execute new work.
2. Bind the return owner to the captured activation, exact originating request and a checked native issuance serial. Never infer this identity from a current actor-name lookup, UI revision or page ordinal. UI content additionally retains the existing guest lifetime and issued patch receipt.
3. One returned page has a fixed maximum of 4096 payload bytes and fixed-width bounded metadata. A page ordinal is not itself ownership. Repeated transport delivery must identify the same source page without consuming it twice.
4. The producer retains the original typed result and its descendant cursors before normalization or encoding. A returned page has one retained source slot; it is not released by a raw poll callback, successful postMessage, surface map deletion or a semantic UI ACK.
5. Host input release requires the exact page to have been consumed or cancelled and its decoder/borrowed-input obligations to be terminal. It is distinct from the existing UI publication ACK, which remains bound to the actual patch receipt and private paired-publication token.
6. Page-release control turns must not recursively create another variable output needing another page-release ACK. Their empty response is a fixed control result; any genuine new semantic payload gets a separately admitted native return owner.
7. Close revokes new semantic work but retains the captured worker API for issued page/UI/lifecycle acknowledgements. Final lifecycle retirement joins the native result cursor, every source page, host decoded inputs, UI reads and existing native descendants.
8. Faults before or after handoff retain the exact original owner. Unknown JavaScript wrapper fields remain retained faults, not a fabricated whole-record retirement certificate. Well-formed production return roots need actual bounded terminal progress.
9. Counter exhaustion, full source/host capacity, stale generation, wrong lifetime, duplicate or out-of-order page ACK, interrupted copy, and worker loss preserve recoverable ownership. No cap increase or synthetic clock is allowed.

## Implementation Order

### Exact Poll Origin Join

The current `ShardClient.nextRequestId` allocates a checked positive safe-number sequence and formats its transport key as `rN`. Open and Close reserve their semantic lifecycle request sequences separately; every subsequent poll and ACK gets another transport request. The worker currently calls `api.poll(events, commandPage, budget)` without carrying that transport identity into native poll.

The native return origin must therefore bind the actual dispatched poll's typed request sequence, not the Open/Close semantic sequence and not a parsed actor name. Demonstrator will forward that exact captured typed authority when the single canonical poll request schema is released. This finding was sent to the native owner through the coordinator; no guessed field or parallel ABI was mounted.

First agree the native return-owner admission and existing-poll attachment with the native owner while its current UI patch handoff is still changing. Then author one canonical schema plus language-neutral success, refusal, cancellation, duplicate and exhaustion vectors. Implement the neutral storage/encoder cursor and exact page input-release contract in Rust and TypeScript before changing production consumers.

After actual native and JS RED/GREEN gates, replace the whole variable WIT/Actor return fields and all authored consumers together. Only then mount the host pre-admitted response cells into public lifecycle/operation dispatch, and require terminal raw-output ownership before final lifecycle ACK. The UI's released OwnedUiPatchIntake continues to own publication/read obligations; any incremental operation decoding changes must be coordinated with that owner, not implemented as a parallel tree or store.

This document is an implementation proposal and records inspected current source. It does not define a released ABI, grant native return authority, or claim live content/close readiness. No shared schema, native source, generated output or compiler cache was changed by this proposal.
