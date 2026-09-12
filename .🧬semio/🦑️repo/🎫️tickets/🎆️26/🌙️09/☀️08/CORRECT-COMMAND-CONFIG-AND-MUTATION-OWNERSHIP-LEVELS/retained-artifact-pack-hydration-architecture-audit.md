# Retained Artifact Pack Hydration Architecture Audit

## Scope and Evidence

Current implementation refinement (2026-09-12): `store-owner-capability-level-audit.md` and the root review supersede the broad required-DocumentStoreOwners decoder wording in slices 4–5 below. Generic lifecycle and identity-bearing persisted loading are separate capabilities. `retained-pack-physical-source-ownership.md` records the source/backing and once-per-chunk repairs; its focused native results remain the authority for completion. Existing codec 1 admission stays enabled. The next catalog slice is specified in `retained-pack-catalog-next-slice.md`.

This is a read-only audit of `retained-artifact-pack-hydration-architecture.md`, the suite-52 progress review, and the currently checked-out Rust sources. No source, Cargo manifest, generated output, ticket state, or goal state was changed. The repository MCP and its `repo://goals` resource are not exposed in this session; the supplied open ticket is therefore the only ticket context used.

The suite-52 result remains functional evidence only. The common hydrator still advances `pack_scanned` without reading input, invokes whole `P::decode_pack` in one step, and later treats `Vec::truncate` as physical release (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:263-287,646-659`). It cannot support an end-to-end retained decode claim.

The proposal has the right end state: a factory-owned, incremental typed decode and an ingress owner that survives cancellation and rejection. It is not ready to implement as written because several proposed dependencies are not physically safe yet, and one lower Pack catalog behavior is semantically wrong for chunked Packs.

## Blocking Findings

### 1. `RetainedPackPage` is not a 4 KiB physical allocation

`RetainedPackPage` contains `[u8; 4096]` **and** `usize len` (`🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs:1087-1110`). Its Rust layout is therefore larger than the 4 KiB payload (on a usual 64-bit target, at least 4,104 bytes before any allocator rounding). The current source nevertheless reserves a `Vec<RetainedPackPage>` up front and reports/releases exactly `RETAINED_PACK_PAGE_BYTES` per logical page (`:1133-1160,1226-1239`). The vector backing itself is wrapped in `ManuallyDrop`, so popping every page leaves its backing neither released nor reported.

This is also a defect in the proposed `PagedList<RetainedPackPage>` wording if its scheduler continues to cap a close grant at 4,096 bytes. `PagedList` would correctly report the actual `Vec<RetainedPackPage>` capacity times `size_of::<RetainedPackPage>()`; a 4,096-byte close grant could never release one page. All source admission, close, and operation-credit code must ask the owner for its next/actual allocation size. They must never substitute the payload extent for a backing allocation.

Choose one explicit representation before lower-Package work:

1. Store full pages as `PagedList<RetainedPackPage, N>` and define source credits/close grants in actual allocation bytes, including list metadata and `size_of::<RetainedPackPage>()`; or
2. Make the payload item exactly `[u8; 4096]` and retain the final-page length in separately accounted metadata. This permits a 4 KiB payload-page grant but still requires separate metadata grants.

The first option is narrower and preserves the current page type. It must use `PagedList::next_allocation_bytes` and the returned `allocated_bytes`/`released_allocation_bytes`; it must not hard-code `4096` in the retained owner.

### 2. The existing Pack catalog is neither a safe owner nor correct for chunk segments

`RetainedPackCatalogCursor::try_new` synchronously reserves all symbol, chunk, and observed-chunk vector capacity (`🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs:1852-1909`). Symbol lengths then reserve independent `String` allocations. The only checks are individual cardinality limits and one `maximum_string_bytes <= max_total_alloc` comparison; there is no aggregate accounting for roster backing, all strings, chunks, observed headers, or allocator rounding. `close_step` pops logical entries and returns zero released bytes (`:2077-2091`), after which ordinary `Vec`/`String` drops release unreported backings.

There is a separate semantic blocker at `:1961-1966`: every `RawByte` from a `KIND_CHUNK` segment pushes the same segment header into `observed_chunks`. The chunk table later indexes this list one entry per chunk (`:1943-1951`) and completion requires its length to equal the number of chunk entries. A multi-byte chunk consequently produces duplicate observations or reaches capacity. Record one header at `Begin(segment)` when `segment.kind == KIND_CHUNK`, not once per payload byte, and add a valid multi-byte chunk corpus row before treating this cursor as reusable.

The proposed symbol byte pool plus span table is the right direction, but it needs an explicit total allocation ledger and a cumulative symbol-byte/cardinality check. A `PagedList<String, N>` alone is insufficient: it accounts for the roster only, while each `String` owns a second backing. Use a paged UTF-8 byte pool plus a paged span table, or a retained exact-contiguous string owner whose allocation and release are separately reported. Apply the same rule to typed snapshot strings, lists, maps, dictionaries, presence vectors, and custom-format fields.

### 3. Value cursors and mounted typed sessions are grammar references, not retained owner implementations

`RetainedValueCursor::try_new` reserves the whole expectation stack from `max_depth * 8` synchronously (`🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:791-838`). Its close path pops stack elements but reports no backing bytes (`:1186-1208`). `RetainedRecordBodyCursor` reserves a symbol roster and each `String` during semantic transitions, then closes only logical elements (`:1239-1419`). Both must receive allocation/retirement owners before they are evidence for bounded Store hydration.

Generation2d and Generation3d mounted sessions have the desired event sequencing, but `allocate_after_discriminator` creates the source, catalog, value cursor, and typed owner directly while admitting a discriminator byte. It therefore performs several unreported allocations on one parser transition. Their current 4 KiB close gate repeats the physical-page mismatch. They can supply grammar/typed-field decomposition only; do not transplant their ownership logic.

Semio Flow makes the factory requirement real, but its current decoder still owns `MemberOpenRequest`, `String`s, vectors, and `try_reserve_exact` transitions directly. It must be converted to the same allocation/close law, not merely fitted behind the new trait.

### 4. Refusing compressed input can only be a lower-layer staging state

The source currently enables `DeflateRetainedCursor` for the deflate feature and accepts codec 1 incrementally (`🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs:428-500`; used by the Pack segment cursor). Its `close` drops an opaque inflater without a physical allocation ledger. The proposal correctly identifies that gap but proposes `UnsupportedCodec(1)` until repair.

That refusal is acceptable only while no common persisted-document route is switched. It cannot be the final Store behavior: codec 1 documents are already admitted by the current Pack parser, and the objective requires support for all existing admitted valid documents. Before activation, add a first-party retained inflater ownership contract exposing construction allocation, any working backing allocation, one-output-byte progress, and bounded close. The catalog/factory inventory must include a compressed canonical fixture that reaches a typed field and has a cancellation cut while the inflater is live. A final gate that excludes compressed valid documents would silently reduce the intended scope.

### 5. The common API must remove member-only diagnostics without duplicating identity/owner data

The current common hydrator imports `MemberOpenDiagnostic` at document scope and returns it for Pack decode and Store initialization (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:1-8,223-240`). `MemberOpenDiagnostic`, `MemberOpenPhase`, and `MemberOpenProgress` live at the composition/open boundary and describe member framing (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs:16-75`). They must not be the new root-and-member document decoder's fault/progress vocabulary.

Add a Store-document-level `PersistedSnapshotDecodeDiagnostic` and `PersistedSnapshotDecodeProgress`. It must distinguish identity/envelope, parser, codec, typed-shape, allocation, stale/cancel/expiry, and retirement-fault outcomes while preserving the first primary fault through close. Composition adapts this diagnostic to `MemberOpenDiagnostic` only at its public boundary.

`ArtifactRef` is already the canonical document identity (`🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs:170-187`) and should stay in the request without a look-alike type. `OwnerRef` already expresses the persisted parent/slot/child relation (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🏠️owner/🧬️schema/🦀️.rs:1-14`); retain `Option<OwnerRef>` directly for root versus child documents. Do not introduce aliases or string copies. The factory request must retain those exact values through parsing and validate them before publication.

### 6. Factory boundary: Store owns operation/input lifecycle; domain owns binary grammar and typed construction

The narrow reusable Store boundary is:

```text
channel ingress owner
  -> RetainedDocumentPackInput (Store document owner)
  -> ArtifactPersistedSnapshotDecodeFactory<P>::begin(request, input)
  -> ArtifactPersistedSnapshotDecode<P>
  -> common history hydration/publication
```

`RetainedDocumentPackInput` owns physical raw pages and only exposes borrowed, bounded source events. `DocumentStoreOwners<P, Mutation>` owns a required decode factory alongside the existing snapshot-retirement factory. The common hydrator never has an `ArtifactPack` bound and never calls `P::decode_pack`, `P::from_value`, `decode_document`, or `decode_record_body`.

The factory must consume the input because standard canonical Pack and custom Flow have different binary grammar. Store can aggregate a fixed progress shape (input bytes, parser events, typed events, admitted/released allocation bytes), while the factory fills parser-specific counters. A factory must not return `Ready` while retaining raw pages, parser state, or a partial typed candidate. A rejecting factory returns the original sealed input to the common physical retirement path or owns it until terminal close; it does not reduce the common route to a synchronous fallback.

The standard semio-binary envelope cursor belongs with the standard factory/parser package, not in every domain. It can be exposed through the Store factory contract because Store already depends on the mounted Pack surface. The generated/domain typed authority remains at the schema owner. This prevents Store-to-plugin dependencies while avoiding a generic JSON/`DslValue` detour.

### 7. `PagedList` is usable, with two contract strengthenings

The current `PagedList` implementation does **not** double-charge allocation. At `🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs:261-299`, line 267 is a non-mutating preflight of `allocated + requested`; metadata and payload branches subsequently record actual `Vec` capacity exactly once. An over-grant allocation stays owned and is returned as `PagedListAllocationError { allocated_bytes: actual }`, so the caller can enter bounded retirement. Existing counter laws cover the metadata and payload over-allocation cases.

Strengthen, rather than replace, this owner:

1. Make `terminal_is_empty` require `root.capacity() == 0`, `length == 0`, `capacity == 0`, and `allocated == 0`.
2. Add an independent normal-success law asserting that admitted bytes equal released bytes and all four terminal ledger fields are zero. Existing normal list law checks `released == admitted` but not `allocated_bytes() == 0` after terminal (`🧪️tests/📋️list/🦀️.rs:4-37`). Repeat the assertion for the metadata and payload allocator-overrun paths after exact release; the present counter law proves retained pointer/exact release but its terminal witnesses do not uniformly assert an all-zero ledger (`🧪️tests/🔬️counter/🦀️.rs:24-82`).

This is defense-in-depth, not a claimed current `PagedList` accounting defect. It makes the physical invariant executable before Pack adopts the type.

### 8. Fuel is a saturated work budget, so current combined item/byte accounting overspends

`StepContext::consume_fuel` saturates at zero (`🧰️framework/🔨️modules/🧵️job/🦀️.rs:978-993`). In both common active retirement and `CloseEnvelopeRuntime`, the hydrator passes `maximum_bytes = min(fuel_remaining, 4096)`, accepts one released item plus `maximum_bytes` released bytes, then consumes `released_items + released_bytes` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs:223-240,665-680`). With one item and `released_bytes == fuel_remaining`, the operation performs one unit more work than its advertised fuel; saturation hides it.

The new contract needs a non-ambiguous grant. Either reserve one unit of fuel for an item before deriving a byte grant, or define independent `work_units`, `maximum_items`, and `maximum_bytes` in the retirement grant and require each owner to report all three. Do not use `fuel_remaining` as a byte grant and then add an item charge afterward. The retained Pack page mismatch makes the current 4 KiB cap inappropriate in the new route regardless.

## Executable Implementation Slices

1. **Establish physical collection and grant law.** Strengthen `PagedList` terminal witness and add the normal/all-zero plus over-grant/all-zero laws. Define one Store-independent allocation/retirement grant that separates work, item, and physical-byte limits. Its law includes an insufficient grant preserving pointer, initialized length, capacity, and allocation ledger.

2. **Repair Pack primitive owners before Store integration.** Replace source's up-front `ManuallyDrop<Vec<RetainedPackPage>>` with a real page owner driven by actual allocation sizes; repair catalog roster/symbol/chunk owners and the once-per-chunk header observation; repair value and record-body stacks/symbols. Add aggregate cardinality/allocation limits. The lower corpus must include multi-page input, multi-byte chunk, nested symbols, cancellation, malformed prefix, and exact physical retirement.

3. **Implement retained deflate before activation.** Give the existing first-party inflater a physical allocation/close contract and exercise codec 1 alongside uncompressed Pack. A temporary parser-only rejection can remain below the activation boundary; the production factory cannot reject codec 1 valid documents.

4. **Add the common factory contract.** Add Store document types for sealed raw input, exact request identity, generic diagnostic/progress, decoder/factory, and required `DocumentStoreOwners` factory. Update all constructors directly. Implement a rejecting factory that preserves the owner; there is no optional compatibility field and no generic synchronous fallback.

5. **Add factories in dependency order.** Implement a small standard typed authority first, then extract Generation2d/Generation3d grammar while replacing their dynamic owners, then convert Flow with its custom grammar. For every archive-reachable schema, install a real factory before switching the shared route. The factory output is tested byte-for-byte against the existing codec through an independent language-neutral corpus, but does not invoke that codec in retained production code.

6. **Switch hydration and remove duplicate member decoding.** Replace `ScanPack`/`DecodePack`/`Vec` retirement in the common hydrator with one decoder phase. Preserve Store versus Envelope handoff and app-owned initialization. Remove the member opener's unreachable second typed hydration path only after both parent and member use the factory contract.

7. **Replace archive ingress last.** Build a move-only `RetainedDocumentArchiveIngress` incrementally in the channel decoder, including parent Pack, SPR, member identity fields, and envelope Pack. Do not call `read_bounded_bytes` for these load fields. Transfer exact sealed owners to active archive load and close them under the same physical grant protocol. The cold `DocumentArchivePack` remains an export/read DTO only.

8. **Final activation gate.** Run the language-neutral and native laws for uncompressed and compressed canonical documents, Flow, parent/branch/leaf histories, malformed-after-valid-prefix, all cancellation cuts, stale/expiry/owner mismatch, exact physical release, normal-stack watchdog, and atomic publication. Only this gate supports the retained persisted-archive claim.

## Acceptance Checks for the First Sol Slice

- `RetainedPackPage` allocation/release byte counts come from the owner, include page-list metadata, and are not assumed to equal 4,096.
- A valid canonical Pack with a multi-byte chunk reaches catalog completion with exactly one observed header per chunk.
- Symbols and nested typed strings cannot exceed cumulative allocation/cardinality limits even when every individual string is valid.
- A compressed valid Pack is accepted by the final standard factory.
- Each progress increment observes a consumed byte, parser event, allocation, typed token, or actual release; no counter advances independently.
- An insufficient close grant leaves pointer/capacity/initialized length/ledger unchanged; the exact sufficient grant releases one physical backing and reports its actual byte count.
- `Ready` is impossible while raw input, parser state, or partial candidate has retained backing.

No test command was run: the assigned task is read-only and explicitly excludes Cargo. The findings are source inspection results, not a passing validation claim.
