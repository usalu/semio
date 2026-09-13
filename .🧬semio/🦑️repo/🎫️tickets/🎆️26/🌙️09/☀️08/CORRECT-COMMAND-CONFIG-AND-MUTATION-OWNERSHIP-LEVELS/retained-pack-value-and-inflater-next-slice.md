# Retained Pack Value and Inflater Ownership Plan

## Scope

The next Pack-level work should be implemented as two bounded, schema-first slices. The first owns the retained value expectation stack and record-body symbol registry. The second owns the retained DEFLATE history and connects that physical owner to the retained segment and mounted snapshot cursors. Both slices preserve the cold `PackFile` and one-shot codec paths, keep codec 1 supported, and stop below the Store capability/factory boundary.

These slices extend the accepted source and catalog work. They do not claim physical ownership for typed Generation collections, mutation builders, archive ingress, recursive members, or the common persisted-document factory.

## Current Physical Ledger

### Value stack

`RetainedValueCursor` in `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs` owns `Vec<Expect>`. `try_new` synchronously reserves `max_depth * 8` frames and pushes the two root frames. Parsing rejects a push at capacity, but the caller never admits the allocation. `close_step(maximum_items)` removes logical frames and eventually marks the cursor closed while retaining the vector capacity. `terminal_is_empty` checks logical frames and pending input only.

`Expect` and its nested cursors are fixed inline values. The vector backing is the value cursor's only physical allocation. Its direct mounted consumers are the Generation2d and Generation3d snapshot Pack sessions, and it is nested in `RetainedRecordBodyCursor` for mutation bodies.

### Record-body symbols

`RetainedRecordBodyCursor` in the same file owns `Vec<String>` plus an optional `RetainedValueCursor`. The symbol-count transition synchronously reserves the string roster. Every symbol-length transition synchronously reserves another string, and decoded scalar values grow that string. `symbol_chars` and `symbol_char` repeatedly scan UTF-8 with `chars().count()` and `chars().nth(index)`. Close pops strings and the nested value cursor as logical items but neither reports nor bounds their physical releases.

The direct production consumers are the Generation2d and Generation3d mutation binary sessions. Those sessions currently expose no retained allocation demand for their record body, and their close API returns a Boolean without physical release bytes. Their typed mutation owners also allocate strings and vectors independently. That separate typed-owner work remains open after this lower record-body slice.

### Retained inflater

`Inflater` in `🧰️framework/🔨️modules/🗜️deflate/🦀️.rs` owns every byte ever produced in `output: Vec<u8>` so LZ77 matches can index prior output. It grows through ordinary `push`, even though RFC 1951 requires at most a 32,768-byte history window. Dynamic-Huffman phases also move a `Vec<u8>` of code lengths created by `Vec::with_capacity(hlit + hdist)`. These allocations have no admission, actual-capacity ledger, exact close demand, or bounded release.

`DeflateRetainedCursor` in `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs` wraps that inflater. Its constructor is allocation-free only until output or a dynamic tree is decoded. `close` drops the inflater synchronously and reports no physical bytes. `RetainedPackSegmentCursor` creates one inflater at every compressed segment Begin, drops it during ordinary parsing at compressed completion, and drops any live inflater synchronously during close.

The compressed mounted consumers are the Generation2d and Generation3d snapshot sessions through `RetainedPackSegmentCursor`. Their identity-segment path must remain allocation-free. The cold `deflate_decompress`, `PackFile`, and `PackWriter` paths must remain enabled and verified.

### Concrete consumer inventory

| Physical owner | Direct consumer | Current boundary |
| --- | --- | --- |
| `RetainedValueCursor::stack` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs` | Snapshot session constructs it synchronously, omits it from `next_retained_allocation_bytes`/`retained_allocated_bytes`, and closes logical frames with one item while reporting zero physical bytes. |
| `RetainedValueCursor::stack` | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs` | Same gap as Generation2d. |
| `RetainedRecordBodyCursor::symbols` and nested value stack | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` | Mutation session constructs after the ordinal, offers no body allocation demand, and closes through `bool close_step(maximum_items)`. |
| `RetainedRecordBodyCursor::symbols` and nested value stack | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs` | Same gap as Generation2d. |
| `Inflater::output` and dynamic code lengths | `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs` `DeflateRetainedCursor` | Byte admission/output is retained, but allocation and close are not. |
| `DeflateRetainedCursor` | `🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs` `RetainedPackSegmentCursor` | A new inflater is created per compressed Begin and synchronously dropped at segment completion or cancellation. |
| `RetainedPackSegmentCursor` | Both Generation snapshot files above | Mounted allocation totals currently include only source and catalog; segment close reports one logical item and zero physical bytes. |

The Store file reexports these owners through its narrow mounted Pack module, but it does not directly own their backing. That reexport is not a reason to change Store lifecycle or factory capabilities.

## Slice One: Value and Record-Body Owners

### Schema and limits

Extend the existing retained Pack schema and fixture before source changes. Specify independent fields for maximum stack frames, maximum symbol count, cumulative symbol UTF-8 bytes, cumulative symbol scalar count, and maximum actual allocation bytes. `max_depth` remains a grammar/depth limit; it is not a physical byte grant. Per-symbol `max_segment_len` remains a wire extent; it does not replace cumulative symbol limits.

Derive the signed stack-frame bound once from `max_depth`, reject overflow, and avoid the `usize::MAX` capacity trap. The constructor must not reserve. A cursor can advertise only a finite request that fits its signed construction-time ceiling.

### Value stack API

Replace the hidden `Vec<Expect>` reserve with an allocation-free retained owner. The cleanest first slice can pre-admit the complete finite `max_depth * 8` frame capacity before accepting a value byte. This keeps the current parser's push semantics and avoids consuming an input event before discovering that a later control-frame push needs allocation.

The cursor should expose:

- `next_allocation_bytes() -> Result<Option<usize>, PackError>` for the exact next backing request;
- `reserve_allocation(maximum_bytes)` with zero/subexact refusal and returned-owner preservation;
- `allocated_bytes()` using actual capacity times `size_of::<Expect>()` plus any real owner metadata;
- `next_release_allocation_bytes() -> Option<usize>` only after all logical frames and pending parser state have retired;
- `close_step(maximum_items, maximum_bytes)` returning logical and physical release categories separately;
- a terminal witness requiring pending input, logical frame count, capacity and allocation ledger all to be zero.

An allocator overgrant that breaches the construction-time physical ceiling must remain owned for cleanup and establish the first sticky fault. That cursor cannot accept or parse a byte after the breach. A zero or subexact grant must preserve the pointer, capacity, initialized frames, pending byte and allocation ledger.

Root frames are initialized only after the admitted backing exists. Allocation is one job opportunity; pushing and popping one inline frame is one logical item opportunity. Physical bytes are never charged as job fuel.

### Shared symbol owner

Extract a retained symbol table owner in the Pack format module so both the full Pack catalog and OS record body can use it without reversing dependency direction. It owns paged Unicode scalars and inline spans containing scalar start, scalar count and exact UTF-8 byte count. It provides indexed `symbol_chars` and `symbol_char` without repeated UTF-8 scans. The owner must accept a pending scalar only after its next backing is admitted, enforce cumulative UTF-8 and scalar limits across all symbols, and preserve its first fault through close.

`RetainedRecordBodyCursor` should own this symbol table plus the retained value cursor. Its constructor remains allocation-free. Its deterministic allocation order is the symbol table demand needed by the pending symbol event, followed by the value stack demand before the cursor enters Value. A pending symbol byte or the transition to Value cannot be consumed until that exact demand is admitted.

Close first retires the nested value owner's logical frames, then symbol spans/scalars, and finally exposes each exact empty backing release. An item-only grant cannot release physical backing. A byte-only grant cannot skip nonempty logical owners. `terminal_is_empty` includes the nested value terminal witness and every symbol logical/physical ledger.

### Mounted mutation propagation

Generation2d and Generation3d mutation sessions must expose the same three-level ownership protocol already used by mounted snapshots: next retained allocation demand, exact reserve, and actual allocated bytes; then next retained release demand and a structured close result. The envelope decoder consumes one admitted work opportunity for an allocation or logical retirement and records actual released bytes separately.

The session-level physical ceiling is aggregate and construction-time: record-body symbols plus the nested value stack, not a maximum single allocation. Demand routing and reserve routing must use the same owner order. Cancellation retains a pending body byte until its owner is selected, preserves the first fault, and cannot invent a cleanup grant.

The existing typed Generation mutation builder allocations are outside this slice. Its current `String` and `Vec` fields must remain called out in the completion ledger, even after the record body becomes physically retained.

## Slice Two: Inflater and Segment Owners

### Decoder split

Separate the DEFLATE state machine from its history storage. Keep the parsing state, bit reader and Huffman tables inline. Use an internal first-party history interface so the one-shot path can preserve its ordinary output result while the retained path supplies a bounded ring owner. No external runtime dependency crosses the public API.

The retained history is one contiguous ring with a logical capacity of `min(maximum_raw_segment_bytes, 32_768)`. Its physical ledger uses the allocator's actual capacity, which may exceed the requested logical capacity. It allocates at most once from empty under an explicit grant, never grows while decoding, and supports overlapping matches by reading the byte `distance` behind the current output position before writing the next byte.

Replace the dynamic-Huffman code-length `Vec` with a fixed inline array and length. The RFC bounds are finite (`HLIT <= 286`, `HDIST <= 32`, at most 318 combined code lengths), so this state requires no heap owner. Before relying on that replacement, add a dynamic-Huffman oracle law that covers literal lengths and repeat symbols 16, 17 and 18. This also guards the existing parser while its phase representation changes.

### Retained inflater API

The retained inflater starts without allocation and receives its finite logical history target and maximum actual allocation bytes at construction. It exposes exact `next_allocation_bytes`, `reserve_allocation`, `allocated_bytes`, `next_release_allocation_bytes`, and `close_step(items, bytes)` operations. Parsing remains one admitted input byte and at most one emitted output byte per grant.

Allocation is required after a compressed Begin and before consuming compressed payload or producing output. Identity segments never request inflater backing. Refused and allocator-overgrant cases preserve the input event, ring pointer/capacity, logical history and ledger; overgrant establishes a sticky fault and prevents later decode admission while keeping the backing available for exact release.

Completion validates the declared raw length, trailing input and decoder terminal state without dropping the ring. A logical reset clears the inline decoder and ring history length while retaining the same backing for the next compressed segment. This lets one `RetainedPackSegmentCursor` reuse one 32 KiB maximum backing across any number of sequential compressed segments. The pointer and physical ledger remain stable across resets, and close releases the backing exactly once.

### Segment and mounted snapshot propagation

`RetainedPackSegmentCursor` should own the reusable retained inflater for its entire lifetime. It exposes allocation demand/reserve/ledger while a compressed Begin waits for history, and exposes exact release demand during close. Its close becomes `close_step(maximum_items, maximum_bytes)` and delegates logical inflater retirement before physical backing release. Terminal empty includes the pending source event, segment parser state and inflater's all-zero witness.

Generation2d and Generation3d mounted snapshot sessions then include source, catalog, value stack and segment inflater in one aggregate physical ceiling. During Drive, the query/reserve pair uses a stable owner order that matches the parser transition: value-stack initialization first, then a pending segment inflater demand, then a catalog event demand. The exact order can differ if source inspection proves another event dependency, but query and reserve must be identical and the native law must exercise simultaneous demands.

Mounted close keeps its current logical owner order but propagates exact segment/value physical release before advancing to later inline anchor and source owners. `next_retained_release_allocation_bytes` must remain `None` while an earlier owner still has logical work; it cannot expose a later allocation that the same close step cannot release. Aggregate released bytes must equal source plus catalog plus value-stack plus inflater actual allocation.

No Store trait, decoder-factory, lifecycle-owner or window-config API changes are needed for this slice. The existing outer snapshot close demand already transports an exact nested byte demand; the mounted session supplies the new value/inflater demand through that boundary.

## Schema-First Laws

The neutral fixture and TypeScript oracle should add these language-neutral cases before Rust implementation:

1. Empty and shallow values begin with zero allocation, refuse a subexact stack grant without mutation, admit the exact stack allocation, parse, then require logical retirement before exact physical release.
2. Nested values at the signed maximum depth succeed; maximum plus one and derived-frame arithmetic overflow fail with the pending byte and first fault preserved.
3. Record-body empty, ASCII and multibyte symbol tables report exact UTF-8 and scalar totals and indexed scalar values. Maximum and maximum-plus-one symbol, cumulative UTF-8 and cumulative scalar cases remain distinct.
4. Record-body allocation refusal, allocator overgrant, cancellation during a partial multibyte symbol, item-only close, byte-only close and exact terminal release preserve pointer/capacity/ledger invariants.
5. Stored, fixed-Huffman and dynamic-Huffman raw DEFLATE vectors match independent `miniz_oxide` output. Dynamic repeat codes 16/17/18 and malformed/truncated variants are included.
6. A raw payload beyond 32 KiB with back-references crossing ring wrap decodes exactly. Distance 32,768 succeeds when history exists; distance zero or beyond available history fails without consuming a subsequent input byte.
7. Zero and subexact ring grants do not mutate the decoder. An actual allocator overgrant is retained, faults sticky, and releases its actual bytes exactly.
8. Two compressed segments reuse one retained ring pointer and allocation ledger. An intervening identity segment neither allocates nor frees it. Cancellation while the second segment owns pending input releases the ring once.
9. Declared raw length exact, minus one and plus one cases preserve the existing length/trailing-input contract.
10. A canonical codec 1 Pack reaches a typed Generation2d and Generation3d field and matches the cold `PackFile` result. A cancellation cut while the inflater is live proves aggregate mounted allocation/release conservation.

The JavaScript/TypeScript oracle should continue to use Ajv 2020 and the platform `TextEncoder`/fatal `TextDecoder`. `miniz_oxide` remains a Rust dev-only third-party codec oracle; it is already a dev dependency of the deflate package. No runtime dependency is added.

## Implementation and Validation Order

1. Extend the retained Pack JSON schema and fixtures, then make the neutral Ajv/TypeScript/codec oracle red for the new value and inflater cases.
2. Implement the value stack owner and its native allocation/release laws in the OS Pack value package.
3. Extract the shared symbol table owner, migrate the full catalog without changing its accepted external receipt, then migrate `RetainedRecordBodyCursor` and both mutation-session protocols.
4. Implement the inline dynamic-code-length representation and retained ring history in the deflate package. Prove stored/fixed/dynamic/wrap behavior against `miniz_oxide` before changing the Pack segment consumer.
5. Migrate `DeflateRetainedCursor`, `RetainedPackSegmentCursor`, and both mounted snapshot sessions. Preserve the cold one-shot API and compressed PackFile law.
6. Run focused lower owners first, then Pack source/catalog/codec 1, Generation2d and Generation3d mounted snapshot and mutation filters, and the actual outer cancellation filters.

Each source switch must remain API-coherent before yielding Cargo to unrelated consumers. Permanent commands remain branches in the root `📜️script.ts`, delegated by root `📋️project.json`, with matching launch and seed entries. Generated logs stay under this ticket's `🗑️generated` directory.

## Acceptance Boundary

Acceptance requires allocation-free retained constructors, finite construction-time physical ceilings, exact demand/reserve pairing, actual-capacity ledgers, non-mutating refusal, retained overgrant faults, exact release, sticky first fault, all-zero terminal witnesses, and real outer cancellation for both uncompressed and codec 1 mounted paths.

After acceptance, typed snapshot/mutation collections, archive ingress, recursive member ownership, cold `PackFile` allocations, and Store persisted-document capability composition remain explicitly open. The result may be described as retained value/catalog/source/inflater ownership for the Generation2d/Generation3d mounted routes, not as complete retained document persistence.

## Implementation Status

Slice One is source-complete for the value stack, shared symbol span/scalar storage, record-body composition and both Generation mounted snapshot/mutation protocols. The public API switch ended after all four real callsites compiled together; later corrections retained those signatures. Schema, fixture and independent platform UTF-8 oracle are green. Focused native red/fix evidence and the current final rerun are recorded in `retained-pack-value-physical-ownership.md`.

The compact diagnostic work is source-complete for Slice One and the upper retained pipeline. Value, record-body, anchor, segment, retained-varint and retained-DEFLATE grants now use sticky allocation-free hot variants; cold APIs retain their owned diagnostics. The registered pipeline diagnostic target passes three new fault laws plus the real codec-1 retained pipeline and hostile anchor-close laws. Registered native run 6 still ended without an Nx footer after Generation2d mutation 1/1, so its three later caller filters produced no result.

Slice Two is implemented and accepted at its bounded source/segment/mounted boundary. The retained decoder owns one actual-capacity 32,768-byte ring, uses inline maximum-288 Huffman symbols and maximum-318 dynamic lengths, reuses the same pointer across compressed segments, leaves identity at zero allocation, and propagates exact allocation/release through Segment and both mounted snapshot sessions. The selected native route passed deflate 3/3, Pack physical 2/2, compact deflate diagnostic 1/1, cold codec 1 1/1 and both Generation mounted suites 2/2. Exact implementation and test evidence is in `retained-pack-inflater-physical-implementation.md`.

Typed snapshot collections, mutation builders and retained SPR/history are now the next lower-to-upper ownership chain. Their source-first design and neutral contract are in `retained-typed-snapshot-mutation-and-spr-history-design.md`.
