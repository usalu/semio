# Retained Pack Inflater Physical Design

## Boundary

The next slice owns only the retained DEFLATE history and its segment/mounted propagation. Cold `Inflater::new`, `inflate`, `deflate_decompress`, `PackFile` and `PackWriter` remain available. Identity segments do not allocate inflater state. Store capability and WindowConfig APIs do not change.

## Shared Decoder Core

Keep one RFC 1951 state machine and give it two history modes. The cold mode retains its ordinary complete-output vector. The retained mode owns `ManuallyDrop<Vec<u8>>` as a circular history with a logical capacity of `min(max_segment_len, 32_768)`, a write cursor and retained length. LZ77 distance lookup uses checked circular coordinates and rejects zero, beyond-retained and beyond-window distances.

The current Huffman `symbols: Vec<u16>` and dynamic `lengths: Vec<u8>` must not remain hidden allocations. Move the three active decode tables into fixed inline decoder fields with at most 288 symbols each. Replace dynamic code lengths with an inline `[u8; 318]` plus initialized length, the RFC maximum of 286 literal/length and 32 distance entries. Decode phases then carry only scalar state; the hot retained decoder has one physical allocation: its history ring.

## Allocation Protocol

`DeflateRetainedCursor` construction receives the declared raw length, logical segment limit and a separate maximum actual allocation ceiling. It constructs the decoder without allocation. Before compressed input is accepted it exposes:

- `next_allocation_bytes() -> Result<Option<usize>, PackError>`;
- `reserve_allocation(maximum_bytes) -> Result<DeflateRetainedAllocationStep, DeflateRetainedAllocationError>`;
- `allocated_bytes() -> usize`.

The exact demand is the ring target in bytes. Zero and subexact grants preserve the pending segment event, cursor, ring pointer/capacity and ledger. `try_reserve_exact` records actual capacity. Allocator rejection becomes the first sticky static fault. Allocator overgrant above the constructor ceiling also becomes sticky while retaining the real backing for close.

`RetainedPackSegmentCursor::try_new` gains the separate physical ceiling and exposes the same aggregate query/reserve/ledger surface. A compressed Begin creates an allocation-free retained decoder, publishes Begin once, and blocks compressed payload admission until the exact ring grant is admitted. The one retained ring is reused across later compressed segments; completion resets logical history and parser state without releasing the backing. Identity-only streams never create a demand.

## Close Protocol

`DeflateRetainedCursor::close_step(maximum_items, maximum_bytes)` retires one pending compressed byte first, then bounded logical history/parser state, then advertises the exact actual ring capacity. A zero grant changes nothing. A subexact byte grant changes no pointer, capacity or ledger. Exact release replaces and drops the backing once, reports the actual bytes and reaches an all-zero terminal witness.

`RetainedPackSegmentCursor::close_step(maximum_items, maximum_bytes)` retires its pending source event, delegates inflater logical and physical close, retires its inline segment state, then becomes closed. `next_release_allocation_bytes` returns no later demand while an earlier logical owner blocks it.

Generation2d and Generation3d mounted snapshot sessions add segment demand after value and catalog demand in both query and reserve order. Their aggregate allocation ledger includes source + catalog + value + segment. Close routes segment physical release before anchor and source, and aggregate released bytes must equal the actual admitted aggregate. Existing outer close byte-demand forwarding remains unchanged.

## Laws

Schema and native laws cover zero/subexact refusal, allocator actual capacity, optional overgrant fault injection, stored/fixed/dynamic blocks, a stream larger than 32 KiB that wraps the ring, maximum distance, invalid distance, truncated input, cancellation with pending compressed input, exact ring release and mounted aggregate cancellation for both Generation snapshots. The existing `miniz_oxide` dev-only differential oracle remains the independent byte-result oracle. Codec 1 stays enabled throughout.

## Acceptance Limit

This slice can establish physical ownership for the retained source, catalog, value stack and segment inflater in the Generation2d/Generation3d mounted snapshot routes. Typed snapshot collections, record mutation builders, retained SPR/history, archive ingress, recursive members, Store factories and WindowConfig candidates remain separate work. It cannot be described as complete retained document persistence.

## Implementation Status

Implemented and accepted at the bounded inflater/segment boundary. The final selected route passes deflate 3/3, Pack physical 2/2, compact deflate diagnostic 1/1, cold codec 1 1/1, Generation2d mounted 2/2 and Generation3d mounted 2/2. It observes exact 32,768-byte ring allocation/release, distance-1 overlap, distance 32,768 across wrap, repeat codes 16/17/18, three compressed segments on one pointer, identity zero demand and mounted aggregate allocation/release equality. See `retained-pack-inflater-physical-implementation.md` and `🗑️generated/retained-pack-inflater-native-callers-1.log`.

The next typed snapshot, mutation and SPR/history boundary is designed in `retained-typed-snapshot-mutation-and-spr-history-design.md` and remains unimplemented.
