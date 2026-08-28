# Paged Wire Decoder Join

## Inspected Boundary

The existing `RetainedUiWireValueCursor` is the shared per-node/component PACK grammar. It owns a whole transferred Uint8Array, reads one varint byte at a time, but reads text and float payloads directly from that array. Text is bounded by the actual 512-byte native scalar envelope and normal collections by 256 fields. Surface document byte arrays have the separate exact path-bound allowance. Previous symbols and map keys currently retain subarray views into the whole source. The grammar must be reused rather than duplicated or replaced with JSON.

The new operation payload builder already owns fixed destination pages and one strongly registered scalar reader. Its copied token and the peer's release receipt do not certify whole-field framing, raw page ACK or UI publication. The peer's `field.advance` consumes one already-copied source byte into its original framing. A partially available field currently stops with no next fragment. No UI method can manufacture that missing range.

## Continuation Phase

After copied receipt settlement, the implemented builder services a dedicated source-continuation phase through the actual `field.advance(grant, exactBuilder)`. It forwards the child's exact blocked/rejected/accounting result and retains its field on faults. A separate 128-byte observation phase reads the completion/consumption/next-fragment metadata after the child's work. A complete field requires equality with the privately captured declared length before payload-ready. A nonterminal consumed range permits a later actual `field.fragment`, but null means retained backpressure, never success. Source continuation is a separate step from receipt clearing or wrapper transitions.

The executed first-fragment bound is `3*n + 4*ceil(n/256) + 16`: one destination-copy step, one source-framing step and one separate observation step per byte, plus fixed page and receipt transitions. The member R15 and independent coordinator fifteen-law gates include that phase split and reader-constructor recovery. Cancellation before copied proof uses Cancelled; cancellation after copied proof retains the copied release obligation. Source framing progress does not reopen the detached byte reader.

## One Decoder, Two Concrete Sources

The decoder will retain its current transfer-owned native-buffer source for the explicitly separate original per-operation entry, and gain an exact privately branded payload-reader entry. No public structural byte provider, callback or arbitrary `{byteAt,length}` object is accepted. The paged entry remains bound to the original builder and exact field. Cumulative input positions use checked u64 bigint; bounded scratch/page indices remain numbers.

The semantic switch remains one implementation. An input-fill phase requests one scalar byte from the concrete source; a page-hop/pending/refusal is forwarded without pretending a byte arrived. Varints require one byte, floats eight, and native text at most 512. Source bytes are consumed only after a successful read. No whole field concatenation or source subarray survives a page release. End-of-input compares exact consumed length, and trailing/truncated fields reject explicitly.

Any scratch and parser-owner allocation must have an actual shared resident reservation before allocation. Existing maxResidentBytes only charges destination pages, while maxOwners charges concrete retained owners; neither proves all parser heap allocations. The implementation must add exact fixed scratch/frame reservations under the same supplied pool rather than quietly treating the 512-byte scalar cap as memory permission. Composition chooses capacity separately; no per-operation pool, maxPatchBytes-derived budget or new default is introduced.

## Decoder and Reader Retirement

Before exposure, the original builder must strongly register the concrete decoder and reserve its owner slot, using the same constructor-before-finalization discipline as the repaired builder. The decoder exclusively claims its issued reader before consuming it. Public standalone reads must be rejected once that exact consumer owns the reader; a previously advanced reader cannot become a fresh wire source.

The close order is decoded frames/values/index tasks, decoder source link, issued reader, destination pages, then wrapper owner slots. The decoder must not recursively ask the parent to close itself, and the parent must not wait on a reader while failing to service the decoder that owns it. Captured Surface bytes must move into independently owned typed-node fields before decoder retirement; current raw JS references do not substitute for that capture.

## Required Executed Laws

The next schema/oracle packet must cover real native upsert and set-component PACK values across every scalar/page split, bounded UTF-8 keys, ordered symbols, normal nested values and Surface byte documents. The same native/Node oracle values must result from both concrete source entries. It also needs exclusive-reader refusal, caller loss during decoder construction, zero grants, source child partial faults, cancellation at input-fill/semantic/retirement boundaries, trailing/truncated values, and exact source lifetime after activation revocation. Successful source-page release remains distinct from later typed publication and native UI ACK.

The continuation and registered-reader construction portions are implemented through the fifteen-law paged checkpoint. The one-decoder/two-source entry, metadata admission and exclusive decoder-to-reader ownership remain an implementation plan, not a live decoder claim. Shared ledger and exact domain-record foundations are now available; UI adoption is tracked in `📓️renderer-shared-ledger-adoption-2026-08-27.md`.
