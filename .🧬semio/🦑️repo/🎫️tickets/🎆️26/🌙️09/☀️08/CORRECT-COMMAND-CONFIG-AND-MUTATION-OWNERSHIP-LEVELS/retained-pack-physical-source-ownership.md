# Retained Pack Physical Source Ownership

## Scope

This implementation completes the first lower-Package ownership slice described by the retained artifact architecture audit. It repairs the byte-source owner, strengthens the shared paged owner that backs it, fixes chunk observation cardinality, and updates the existing Generation2d and Generation3d mounted consumers. It does not switch the common Store hydration/factory boundary.

## Source ownership contract

`RetainedPackSourceCursor` is allocation-free at construction. It owns its input with `PagedList<RetainedPackPage, RETAINED_PACK_MAXIMUM_PAGES>` rather than an up-front `ManuallyDrop<Vec<RetainedPackPage>>`. `RETAINED_PACK_MAXIMUM_PAGES` is derived from the signed addressable byte range and `size_of::<RetainedPackPage>()`; the source never instantiates `PagedList<_, usize::MAX>` or tries to reserve a gigantic metadata capacity.

The constructor separates two limits:

- `maximum_payload_bytes` limits admitted canonical Pack bytes.
- `maximum_allocation_bytes` limits cumulative actual source backing, including list metadata and payload vectors. Values above `isize::MAX` are rejected.

Each `reserve_page` call performs at most one `PagedList` allocation and reports the allocator's actual byte count. A refused grant leaves the owner unchanged. If an allocator returns a larger backing than admitted or crosses the cumulative physical limit, the backing remains owned, the error reports the actual allocation, and a sticky fault prevents page admission, sealing, or parsing. Cleanup remains available under exact grants.

The physical ledger is:

1. Metadata and payload backings increase `allocated_bytes` by their actual capacities.
2. Admitting a page changes the logical page and payload-byte ledgers without allocating.
3. Popping one admitted page reports `released_items: 1, released_bytes: 0`; its backing is still owned.
4. Releasing one empty backing requires `next_release_allocation_bytes()` and reports that exact physical count with `released_items: 0`.
5. An insufficient release grant preserves the backing pointer, reserved capacity, initialized length, and allocation ledger.
6. A zero item/byte close grant changes no source state.
7. Terminal empty requires the source logical byte ledger to be zero and the `PagedList` root capacity, length, logical capacity, and allocated-byte ledger all to be zero.

`PagedList::terminal_is_empty` now checks the complete four-field ledger, and `PagedList::next_release_allocation_bytes` exposes the exact next backing size. Existing allocator-overgrant tests now close each retained backing with that exact query and verify all four terminal fields.

## Consumers and catalog

Generation2d and Generation3d mounted sessions now construct the source with separate payload and physical limits. `admit_byte` refuses and returns the unchanged producer until the page slot exists. The sessions expose the exact next allocation and a one-backing reserve step; their snapshot authorities spend a separate job-fuel step on that allocation before reading or advancing the next encoded input byte. They preflight before moving their inline page, restore that producer on unexpected refusal, and forward the actual close grant. Their snapshot authorities and mounted laws ask the source for the exact next release size. The mounted laws measure the source allocation ledger through cleanup and require aggregate released bytes to equal aggregate admitted source allocation bytes. A zero-item session close leaves its phase unchanged.

`RetainedPackCatalogCursor` records a `KIND_CHUNK` segment once when its `Begin` event arrives. Raw bytes no longer duplicate the segment header. A valid five-byte chunk law compares the retained result with `PackFile` under full verification and requires one observed chunk.

Compressed Pack behavior remains enabled. The focused retained law writes and reads `CodecId(1)` and checks byte-identical document output. The native Pack target must run this law after the source change.

## Neutral and native evidence

The schema-first corpus is defined by `🧰️framework/🔨️modules/🎒️pack/🧬️schema/🔣️.json` and `🧰️framework/🔨️modules/🎒️pack/🧫️fixtures/🔣️.json`. The independent TypeScript oracle validates it with Ajv 2020 and applies the terminal ledger with fast-json-patch. It covers the distinct payload/physical limits, metadata and payload phases, a page item larger than 4 KiB, zero-grant stability, logical-versus-physical release, allocation overgrant as a required hostile case, the all-zero terminal witness, and one `Begin` observation for a multi-byte chunk.

The ticket-local isolated Nx target `abstraction-ownership-validation:framework-retained-pack-ownership` passed on 2026-09-12. Its observable result was:

```text
[DEBUG] Retained Pack physical source fixture agrees with Ajv 2020 and fast-json-patch; chunk observations=1 raw-byte-events=5 terminal-ledger=zero
NX Successfully ran target framework-retained-pack-ownership for project abstraction-ownership-validation
```

Strict TypeScript ran within the same target. `git diff --check` passed for the owned source/configuration paths before the final zero-grant additions and must be repeated after native feedback.

The registered native target runs these focused filters in order:

1. `semio-framework-replication` `value::list::` laws, including actual allocator overgrant and exact release.
2. `semio-framework-pack` `retained_pack_source_laws`, including pointer/capacity preservation, allocation refusal, exact aggregate source release, cancellation after a consumed byte, the multi-byte chunk, hostile CRC, and the existing `CodecId(1)` retained route.
3. `semio-framework-pack` `write_then_read_round_trip_with_compressed_segment_and_chunk`, the existing full-verification `CodecId(1)` PackFile/PackWriter regression.
4. Generation2d `retained_mounted_laws`.
5. Generation2d `retained_pack_outer_` laws, covering exact demand delegation through Store/VCS/snapshot/mounted source, a non-mutating subexact grant, aggregate physical release conservation, cancelled terminal state, and owner-preserving rejection for oversized nested single-demand and cumulative-retained ceilings.
6. Generation3d `retained_mounted_laws`.

The 2026-09-12 focused native rerun completed through the ticket facade in 7m33s. It passed the 5 replication list laws, 8 Pack retained-source laws, the existing compressed `CodecId(1)` round trip, 2 Generation2d mounted laws, and 2 Generation3d mounted laws. Its durable output is `🗑️generated/retained-pack-native-coherence.log`.

The schema-first outer-close contract then passed its four Ajv/JSON Patch and strict TypeScript cases in 4.5s; its durable output is `🗑️generated/retained-pack-outer-neutral.log`. The focused native outer run completed through the registered ticket facade in 2m41s. Both laws passed in 0.01s with 169 filtered tests: the live cancellation route refused a subexact external grant without changing the nested owner, then retired exactly the aggregate admitted source allocation and reached `Cancelled`; the construction laws returned both oversized single-demand and cumulative-retained nested owners for exact close. Its durable output is `🗑️generated/retained-pack-outer-native-1.log`.

## Remaining ownership work

The catalog still owns `Vec` and `String` backings that are allocated eagerly and released without a physical byte ledger. The retained value stack and the deflate inflater also retain dynamic backing outside this source contract. The Generation mounted typed authorities contain further dynamic domain collections. Their source integration is real, but this report does not claim that the complete catalog, typed decoder, compressed path, mounted session, Store hydrator, or recursive archive path is physically retained.

The normal Generation snapshot decode and cancellation paths ask the mounted source for each exact close grant. Store now requires each field, VCS, and snapshot authority to report its next close-byte demand, maximum single demand, and maximum cumulative retained close bytes. Constructors validate the nested ceilings before admission and return an oversized owner intact. The active decoder consumes one work opportunity per close call, keeps physical released bytes in a separate cumulative ledger, and preserves the first close fault. The external close path refuses a caller grant below the current demand without mutating the nested owner. The focused outer native laws satisfy the acceptance gate for this propagation.

The Store persisted-document capability boundary, semio envelope owner, common history hydrator, custom Semio Flow factory, archive ingress owner, and recursive publication laws remain later slices. No Store factory constructor was changed here.

## Files

- Shared owner: `🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs` and its list/counter laws.
- Pack source/catalog: `🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs`, public exports, retained source laws, schema, and fixture.
- Store exports: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`.
- Real consumers: Generation2d and Generation3d mounted snapshot binary owners, focused laws, and snapshot mutation close call sites.
- Commands: root `📜️script.ts`, `📋️project.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, and the ticket-local validation facade.
