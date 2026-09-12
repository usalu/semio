# Retained Artifact Pack Hydration Architecture

## Decision Status

This is the bounded implementation proposal requested after the suite 52 checkpoint. No referenced production source was edited while preparing it, and no new native result is claimed. The earlier proposed one-byte Pack scan/cleanup test was withdrawn because it would have accepted bookkeeping progress and logical `Vec` truncation as if they were parser progress and physical release.

`🗑️generated/recursive-replacement-suite-native-52.log` remains useful functional evidence: the current common hydrator reconstructs nonempty parent, branch and leaf histories, preserves the old archive on malformed-after-valid-prefix input, and reaches the existing atomic replacement boundary on the normal stack. It does not prove retained Pack parsing, retained typed snapshot construction, or physical backing retirement.

## Current Failure Boundary

The current Store-owned cursor at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs` has two unsound proof boundaries:

1. `Phase::ScanPack` advances `pack_scanned` without reading or parsing a byte. `Phase::DecodePack` later invokes the complete `P::decode_pack` under one fuel unit. Precharging an extent does not make the work that consumes it resumable or cancellable.
2. Its Pack owner is a `Vec<u8>`. Cleanup truncates length and reports those removed elements as released bytes even though capacity and the physical allocation remain unchanged; the allocation is later dropped without reporting its physical size.

The defect starts upstream. `PagedDocumentArchiveDecode` in `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs` uses `PagedCommandReader::read_bounded_bytes` to reserve and fill complete `Vec<u8>` fields synchronously. `MemberOpenRequest` receives `OwnedSchemaDecodePages`, but that type reserves a complete boxed `MaybeUninit<OwnedSchemaDecodePage>` registry in `try_with_credits`; `close_take_page` retires initialized page values without releasing or accounting for the box backing. Replacing only the hydrator's `Vec` would leave both ingress owners physically unbounded.

## Reusable First-Party Components

### Canonical Pack event pipeline

`🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs` already provides the semantic state machines needed for the canonical Pack body:

- `RetainedPackPage` is the fixed inline 4 KiB input unit.
- `RetainedPackSegmentCursor::{try_new, preflight, admit, grant, close_step, terminal_is_empty}` validates the fixed header, segment flags, minimal varints, payload lengths, CRC, trailer and complete extent. It emits at most one `RetainedPackSegmentEvent` per grant.
- `RetainedPackAnchorCursor::{new, grant, take, close_step, terminal_is_empty}` verifies the header, footer and superblock from retained source events.
- `RetainedPackCatalogCursor::{try_new, admit, grant, symbol_chars, symbol_char, document_bytes, take, close_step, terminal_is_empty}` validates manifest spans, symbol and chunk counts, observed chunk framing and content hash while emitting document/schema/field-index bytes.
- `RetainedPackSourceProgress` and the source/segment/catalog event enums already define useful observable progress vocabulary.

The canonical retained laws live at `🧰️framework/🔨️modules/🎒️pack/📐️format/🧪️tests/🔬️retained-pack-source-laws/🦀️.rs`; their language-neutral corpus is `🧰️framework/🔨️modules/🎒️pack/🧫️fixtures/🔣️.json`.

The fixed segment and anchor logic is directly reusable. The source and catalog owners require physical-allocation repair before Store can use them as acceptance evidence:

- `RetainedPackSourceCursor::try_new` reserves a `Vec<RetainedPackPage>` to maximum capacity up front. `close_step` pops inline pages and reports 4 KiB for each, but the `Vec` roster allocation is held in `ManuallyDrop` and is neither released nor accounted.
- `RetainedPackCatalogCursor::try_new` reserves symbol, chunk and observed-chunk vectors up front. Symbols also own independent `String` allocations. `close_step` pops logical items with zero released bytes; the capacities are later dropped outside the grant ledger.
- `RetainedPackSegmentCursor` delegates compressed input to `DeflateRetainedCursor`. That cursor drops `semio_framework_deflate::Inflater` synchronously in `close` without a physical grant. Compressed Pack admission must remain explicitly refused until the inflater exposes retained allocation and close semantics.

### Canonical value pipeline

`🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs` provides:

- `RetainedValueCursor::{try_new, admit_byte, seal, grant, ingress_ready, close_step, terminal_is_empty}` and its role/container/token vocabulary.
- `RetainedRecordBodyCursor::{try_new, admit_byte, seal, symbol_chars, symbol_char, grant, ingress_ready, close_step, terminal_is_empty}`.

These state machines already emit bounded semantic tokens, so their grammar should be retained. Their internal `Vec` stack, symbol registry and `String` owners have the same physical-retirement gap as the Pack catalog and must be replaced with grant-accounted owners before reuse.

Store reexports this deliberately narrow surface through `mounted_pack_rt` in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`. Batch `decode_document`, `decode_record_body`, `RecordValue`, `P::from_value` and `P::decode_pack` are intentionally absent from that mounted reachability graph. The new persisted path must preserve that boundary.

### Physical collection owner

`PagedList<T, N>` at `🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs` is the reusable physical owner:

- `next_allocation_bytes`, `reserve_one` and `reserve_capacity_one` admit one backing allocation and report the allocator's actual capacity.
- `push_reserved` places a value only after a slot exists.
- `pop` retires logical values independently of backing pages.
- `release_empty_page` preserves the page under an insufficient byte grant, releases one actual backing allocation under a sufficient grant, and reports its actual byte count.
- `allocated_bytes`, `terminal_is_empty` and the test-only `backing_ptr`/`initialized_len` expose the physical witness needed by the new laws.

The Pack crate already depends on `semio-framework-replication`, which exports this collection, so the lower Pack cursors can use it without adding a runtime dependency or creating a Store-to-Pack dependency cycle.

### Existing typed retained decoders

Two artifacts already compose the retained canonical cursors with direct token-to-domain construction:

- `Generation2dMountedPackSession` and `Generation2dMountedTypedSnapshotOwner` in `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs`.
- `Generation3dMountedPackSession` and `Generation3dMountedTypedSnapshotOwner` in `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs`.

They prove the desired pipeline shape: source, anchor, segment, catalog, value tokens, typed fields, publication and cancellation. They are not yet the common solution. Each typed owner performs `Vec::try_reserve_exact` or `String::try_reserve_exact` inside semantic transitions and closes logical values without releasing/accounting their backings. Their reusable contribution is the typed grammar and state decomposition, not the present allocation implementation.

`SemioFlowSnapshotDecode` in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/📸️snapshot/💾️binary/🦀️.rs` is the custom-format reference. It reads one byte at a time from `MemberOpenRequest`, validates its own discriminator and constructs `SemioFlowSnapshot` directly. Its parser must be detached from the member-only request and its vector/string allocations must become grant-accounted, but it demonstrates why persisted admission needs a domain factory rather than a generic `ArtifactPack::decode_pack` fallback.

### Owned schema factories

The Store-owned schema machinery supplies reusable ownership patterns:

- `ArtifactEnvelopeSnapshotFieldAuthority<P>` accepts tokens, retains a domain-owned candidate and publishes only after exact completion.
- `ArtifactEnvelopeOwnedFieldCatalog<P, Mutation>` supplies domain decoders.
- `ArtifactEnvelopeDecodeOwnerBundle` and `ArtifactOwnedValueRetirementFactory<P>` preserve rejected and partially constructed values until bounded close.

This is a JSON-envelope path. It cannot decode binary Pack by wrapping bytes in JSON. `OwnedSchemaDecodePages` also cannot serve as the physical Pack owner unchanged because its boxed slot allocation is not grant-accounted on close.

## Required Common Interfaces

The permanent Store-level source should be `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/🎒️pack/💧️decode/🦀️.rs`. The existing history hydrator remains at the document/history boundary and consumes the new typed result. Nothing common belongs beneath composition/member-open.

The common API should add these owner-level concepts, with final names reviewed before implementation:

```rust
pub struct PersistedSnapshotDecodeRequest {
    pub operation: OperationId,
    pub generation: Generation,
    pub expires_at_us: u64,
    pub expected: ArtifactRef,
    pub owner: Option<OwnerRef>,
    pub expected_envelope: SemioEnvelope,
    pub pack_limits: PackLimits,
    pub allocation_bytes: usize,
}

pub enum PersistedSnapshotDecodeStep {
    Pending(PersistedSnapshotDecodeProgress),
    Ready,
    Rejected(PersistedSnapshotDecodeDiagnostic),
}

pub trait ArtifactPersistedSnapshotDecode<P>: Send {
    fn step(&mut self, cx: &mut StepContext<'_>) -> PersistedSnapshotDecodeStep;
    fn take_ready(&mut self, cx: &StepContext<'_>) -> Option<P>;
    fn request_cancel(&mut self);
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String>;
    fn terminal_is_empty(&self) -> bool;
}

pub trait ArtifactPersistedSnapshotDecodeFactory<P>: Send + Sync {
    fn begin(
        &self,
        request: PersistedSnapshotDecodeRequest,
        input: RetainedDocumentPackInput,
    ) -> Result<Box<dyn ArtifactPersistedSnapshotDecode<P>>, PersistedSnapshotDecodeRejected>;
}
```

`RetainedDocumentPackInput` is a move-only sealed owner backed by `PagedList<RetainedPackPage, OWNED_DOCUMENT_PACK_MAXIMUM_PAGES>`. It exposes borrowed bytes or source events without concatenating them. Its admission methods allocate at most one metadata or payload page from explicit operation credit. Its close first retires initialized inline page values, then calls `release_empty_page`; an insufficient byte grant preserves the exact `backing_ptr`, capacity and `allocated_bytes`, while a sufficient grant reports the allocator's actual backing size.

The decode operation owns its input, parser states and partial typed candidate. `Ready` is observable only after all parser and raw-input backings are terminal empty; the typed `P` is then the sole live owner and `take_ready` makes the decoder terminal. Fault and cancellation retain the primary diagnostic while closing the partial typed candidate, catalog/value state, envelope token owner and raw pages separately.

`PersistedSnapshotDecodeProgress` must describe actual work: admitted pages/bytes, consumed Pack bytes, verified segments, admitted document-body bytes, emitted value tokens, constructed typed fields and released physical allocation bytes. A counter that can advance without reading a byte is forbidden.

`DocumentStoreOwners<P, Mutation>` should own a required `Arc<dyn ArtifactPersistedSnapshotDecodeFactory<P>>` beside the initial-snapshot retirement factory. Owners that cannot be persisted install an explicit rejecting factory which returns the exact unconsumed `RetainedDocumentPackInput`; the common archive path has no synchronous fallback. Because the repository is greenfield, all constructor consumers should be updated directly and no optional compatibility capability or old-name alias should remain.

The operation's allocation credit is admitted with the archive/member operation before parsing. Each `step` performs at most one parser event, typed token or backing allocation and subtracts the actual allocation from that credit. An allocator result larger than the available credit enters retained rejection with the newly allocated owner still attached; it cannot be dropped to report failure. Close continues to use the scheduler's explicit item/byte grant.

## Standard Binary Factory

The standard factory needs two retained layers:

1. `RetainedSemioBinaryEnvelopeCursor` parses the eight-byte `BINARY_MAGIC`, little-endian token length and UTF-8 token incrementally. It validates exact plugin, artifact, component and version against `PersistedSnapshotDecodeRequest` before semantic payload allocation. This replaces synchronous `semio_format::unwrap_binary`, which currently copies the entire payload into a new `Vec`.
2. `RetainedCanonicalArtifactSnapshotDecode<P>` feeds payload source events through the repaired anchor, segment, catalog and value cursors and then into a domain typed authority. It never constructs a complete `DslValue`/`RecordValue` tree and never calls `P::from_value` or `P::decode_pack`.

The lower Pack repairs should keep the existing event grammar while replacing physical owners:

- Source pages become `PagedList<RetainedPackPage, N>` with allocation and release phases.
- Symbol text becomes one paged UTF-8 byte pool plus a paged span table rather than one allocation per `String`.
- Chunk entries and observed segment headers become paged lists.
- The value expectation/container stack becomes fixed bounded storage or a paged list admitted one backing at a time.
- Catalog completion validates in place. Typed authorities borrow verified symbols/spans until their fields are complete; the catalog no longer needs to transfer dynamic `Vec` registries merely to prove completion.
- Deflate input is refused with `UnsupportedCodec(1)` until `Inflater` gains the same allocation/close contract.

The Generation2d/Generation3d typed grammars should be extracted behind the common typed authority and their dynamic stacks, lists, presence vectors, dictionaries, JSON/DSL frames and strings moved to paged or exact contiguous owners. A standard derived artifact needs a generated typed authority at its schema owner; generating a factory that calls the synchronous `ArtifactPack` implementation would reproduce the defect.

Custom formats such as Semio Flow install their own `ArtifactPersistedSnapshotDecodeFactory`. They receive the same input owner and authority request, so root and member documents share operation identity, cancellation and physical retirement even when their binary grammar differs.

The inventory found 60 `ArtifactPack` implementation occurrences across 38 Rust source files, but only the two Generation mounted sessions provide canonical retained typed construction today. Many occurrences are config, presence, transient or test state rather than persisted documents. The implementation inventory must classify actual document owners first; unsupported state lanes must reject before consuming bytes, and every document kind reachable from archive/member admission must install a real factory before the common path is switched.

## Hydrator And Composition Integration

`RetainedPersistedDocumentHydration<P, Mutation>` should replace `from_pack(Vec<u8>)`, `ScanPack` and `DecodePack` with a single `DecodeInitialSnapshot` phase holding `Box<dyn ArtifactPersistedSnapshotDecode<P>>`. It must retain the existing `ArtifactStoreInitializationRuntime`, history replay, `Store`/`Envelope` output modes and app-owned initializer semantics. No domain initializer moves into Store.

Parent and member paths pass the same sealed input owner and exact request into the factory. The parent continues from the common `Envelope` output into `begin_persisted_document_store_replacement`; member opening continues from `Store` output into the existing closed-member handoff. Atomic parent/member/content/coordinator/window publication remains downstream and unchanged.

After the switch, remove the unreachable second typed hydration implementation from `InitialMemberStoreOpen` in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs`: `BeginEdit`, `DecodeForward`, `DecodeInverse`, `DecodeMetadata`, `FinishEdit`, the five history-record hydration phases, `Replay`, `SeedApplied`, `SeedRedo`, `RetireHistory` and `Initialize`, together with their dead envelope/runtime/edit/operation/record/pin fields. Keep only framing, retained history decode, factory selection, common hydration, source retirement and ready/rejected handoff.

The recursive fixture's custom snapshot opener currently copies the whole snapshot into a `Vec`, calls synchronous `ArtifactPack::decode_pack`, and logically truncates that vector on close. It must install a real persisted decoder factory and exercise the same common path. The test fixture cannot be a special synchronous exception.

## Channel Ingress Integration

`DocumentArchivePack` remains suitable as the cold output/read DTO used by constitutional codecs. Load admission needs a distinct move-only `RetainedDocumentArchiveIngress` whose parent Pack, parent SPR and member envelopes are physically owned page collections. The v17 paged command decoder must build those owners incrementally instead of calling `read_bounded_bytes` for complete vectors.

Length-prefixed archive fields can cross channel input pages. The command decoder therefore reserves at most one destination page under an admitted allocation opportunity, copies a bounded byte extent, and retains both source and destination until the copy advances. Completion moves the sealed ingress into `AppCommand::LoadDocumentArchive`; plugin admission then moves the exact owner into `ActiveDocumentArchiveLoad`. Refusal, cancellation and out-of-limit input return or retire that owner without first converting it to `DocumentArchivePack`.

This load-only owner is a new protocol type, not an alias or a binary-to-JSON wrapper. Read/export may still produce the cold DTO through its own retained writer. If `AppCommand` derives conflict with a move-only owner, its equality/debug contract should be implemented explicitly around command identity and observable fields rather than making the retained owner cloneable.

## Physical Retirement Invariants

Every dynamic owner in the path must satisfy all of these rules:

1. Allocation is a visible retained step. One maintenance opportunity performs at most one allocation and reports its actual bytes.
2. Logical element retirement and physical backing release are separate events.
3. Before release, an insufficient grant preserves exact pointer, capacity, initialized length and allocation ledger.
4. A successful backing release reports `capacity * size_of::<T>()` or the exact allocation size returned by its owner, plus the correct allocation item count.
5. Nested strings, token stacks, symbol pools, chunk tables, typed vectors and custom decoder owners obey the same rule; a terminal witness cannot ignore empty-but-allocated capacity.
6. `terminal_is_empty` means every candidate, parser, source page, metadata page and rejected owner has transferred or physically released its backing.
7. A primary stale/cancel/malformed/capacity diagnostic survives cleanup. A cleanup fault is retained separately and cannot overwrite it.

## Language-Neutral Laws

Add a schema-first corpus at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/🎒️pack/🧫️fixtures/🔣️.json` with schema id `semio.retained-artifact-pack-hydration-laws.v1`. It should include:

- A standard semio-wrapped canonical Pack larger than two physical pages and one custom-format document.
- Expected progress after each admitted event: input offset, verified segment count, document byte count, value-token count and typed-field count.
- Cancellation cuts in the semio magic, token, segment header, document body, nested allocation, typed field and post-construction/pre-handoff phases.
- CRC mismatch, nonminimal varint, bad UTF-8/envelope identity, bad field shape, limit plus one and trailing payload after valid prefixes.
- Stale operation, stale generation, expiry and owner/reference mismatch.
- A physical ledger for metadata and payload allocations, including an insufficient close grant with unchanged pointer/capacity and a sufficient grant with the exact released allocation bytes.
- A recursive archive whose parent, branch and leaf each contain nonempty history, plus a CRC-valid malformed history after a valid prefix.

The ticket's permanent `validation/📜️script.ts` should validate the fixture with Ajv, reproduce CRC values with the already resolved third-party `crc-32` package after declaring it as a direct development dependency, and validate the recursive closure with the existing Graphlib reference check. The TypeScript byte tracer should independently parse the semio header, little-endian token length, canonical segment varints and page boundaries and emit the same progress trace as Rust. Strict TypeScript remains mandatory. These are test-only dependencies; production receives no external runtime dependency.

## Native Laws

The focused Store/native route must prove behavior rather than counters:

- A multi-page standard document changes `consumed_pack_bytes`, segment/catalog state, value-token count and typed-field count under one-event grants. A zero/insufficient allocation opportunity leaves every progress field and backing witness unchanged.
- Each step consumes at most one parser event, allocation or typed token. The watchdog runs on the normal configured 2 MiB stack.
- Cancellation at every neutral cut closes a partially constructed typed candidate and all parser/input owners to exact terminal empty.
- After a valid typed prefix, malformed CRC/shape/history input retains the primary fault and leaves the previously published Store byte-identical.
- Physical close records pointer, capacity, initialized length and `allocated_bytes`; an insufficient grant preserves all four, and the exact grant releases one backing and reports its actual bytes.
- Stale operation/generation/expiry returns the exact unconsumed owner or closes it through the same physical ledger.
- Standard Generation2d/Generation3d and custom Semio Flow factories produce byte-identical snapshots to their existing synchronous codecs without calling those codecs from the retained implementation.
- The recursive plugin suite repeats parent/branch/leaf nonempty history, cancellation, malformed prefix and atomic publication on the normal stack through the production app initializer.

The withdrawn one-byte scan/cleanup test is not part of this plan. No source-ready or pending-pass claim remains for it.

## Implementation Slices And Gates

1. Repair the lower Pack source/catalog/value physical owners and add their neutral/native physical laws. Keep compressed input refused. Gate: actual parse progress and exact backing release pass independently of Store.
2. Add the Store-level request, progress, decoder/factory traits and retained semio envelope cursor. Add a direct standard test factory whose typed authority constructs a small record without `DslValue` or synchronous Pack decode. Gate: Store parser/candidate cancellation and physical cleanup laws pass.
3. Convert Generation2d, Generation3d and Semio Flow to the common factory and repair their typed dynamic owners. Inventory every archive-reachable document factory. Gate: retained output matches the existing synchronous codec through independent fixtures.
4. Add the required factory to `DocumentStoreOwners`, update every constructor directly, switch the common history hydrator, and delete the old member typed path plus recursive synchronous opener. Gate: Store history laws and all 87 catalog consumers compile.
5. Replace load-side channel vectors with `RetainedDocumentArchiveIngress` and wire exact owner transfer into the plugin registry. Gate: channel neutral/Ajv/CRC/strict-TypeScript laws and native cancel/refusal/ACK laws pass.
6. Run the five recursive production laws plus standard/custom decoder laws on the normal stack. Only this final gate can support an end-to-end bounded persisted archive claim.

This proposal intentionally leaves production unchanged until architecture review. The current source remains at the suite 52 functional checkpoint, and the fleet Cargo slot is yielded.
