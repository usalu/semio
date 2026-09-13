# Retained Exact Window-Config Pack Load Capability Design

## Scope and Order

This is the first shared production slice required after Generation2d exact-window acceptance. It establishes a retained, exact, failure-atomic `WindowConfigPack` load. It does not add lifecycle events, change eager capture callers, or provision Generation3d owners. Those later changes depend on a load operation that can prove restored-owner priority without mutating live authority during decode.

## Current Authority Violation

`WindowConfigOwnerRegistry::load` selects only the outer `window_kind_id` and calls `TypedWindowConfigStoreOwner::load`. The typed load calls the monolithic async `store::parse_document_pack`, checks only `parsed.envelope.schema`, then calls `partition(window_id)`. `partition` creates and installs a default-valued live `ConfigStore` for an absent target before `ConfigStore::reset` is known to succeed.

The parsed document envelope also owns the persisted partition id in `parsed.envelope.id`. The loader does not compare it with `window-config:{outer-window-kind-id}:{outer-window-id}`. A pack whose outer target is `target` but whose inner id remains `window-config:identity-window:source` therefore changes the target. The delimiter case does the same from `scene:one` to `scene:two`. Schema equality cannot establish instance identity.

This order has two unacceptable results: a rejected load can leave a newly materialized default partition, and a valid same-kind foreign partition can replace a target. Calling `reset` on the live store also makes the live store the decode transaction owner, so cancellation cannot preserve the old authority and retire a rejected candidate independently.

## Required Store Capability

Store needs a retained `ConfigStore` load candidate that is built beside the live store. Its public contract should have the following shape, with names resolved during implementation:

- `begin` accepts owned, pre-admitted Pack and SPR pages plus the typed owner catalogs. Capacity or factory rejection returns every input owner unchanged.
- `advance` performs one bounded step through Pack decode, SPR/history decode, edit validation, cursor replay, and candidate-store construction. It receives explicit item, byte, fuel, and wall-time grants and never drives itself to readiness.
- the ready state retains exactly one fully initialized `ConfigStore<C, M>` and exposes immutable candidate identity and cursor facts before handoff;
- `take_ready` transfers the candidate once, without changing another store;
- `request_cancel`, `begin_close`, `close_step`, and `terminal_is_empty` retain and physically retire decoded envelopes, replay snapshots, history rows, pages, a partly built store, or a ready rejected store; and
- `Drop` fails closed unless candidate ownership was transferred or every retained owner reached terminal emptiness.

This cannot be a wrapper around the current `parse_document_pack(...).await` followed by `ConfigStore::reset`. The parser and store construction are the expensive work that must become retained. Current reusable lower seams are `RetainedPackSourceCursor` with its page demand/reserve/actual ledger, `RetainedPackAnchorCursor`, `RetainedPackSegmentCursor`, `RetainedPackCatalogCursor` with the shared `RetainedPackSymbolTable`, and `RetainedPackCloseStep`. OS Pack value also provides `RetainedValueCursor` and `RetainedRecordBodyCursor` with pre-admitted finite stacks, structural tokens, allocation ledgers, and exact close.

Two lower gaps remain explicit. Segment inflation is not yet retained, and no generic typed `ArtifactDsl` or WindowConfig factory replays `RetainedValueToken`; Generation2d and Generation3d snapshot/mutation consumers are domain-specific state machines and cannot serve as a generic factory. Store still needs retained SPR/history decode, bounded typed replay, and fresh-store construction. The operation may reuse the document replacement initializer concepts, but it must not import the document-only child/composition lifecycle into a scalar config store or hide a monolithic parser behind a retained shell. Inflater retention remains a separately reviewed lower slice.

Resource grants remain typed by what they retire. `O::MAXIMUM_PUBLICATION_BYTES = 4,096` bounds one WindowConfig mutation preparation and publication. It does not size load decode or physical job cleanup. Mounted job payload retirement uses the shared `semio_framework_job::JOB_PAYLOAD_PAGE_BYTES` value, currently 16 KiB; Store decode pages use Store's declared decode-page bound. The implementation must not substitute any of these constants for another.

## Exact Window Load Candidate

The plugin registry owns a fixed-capacity operation table. Each entry is keyed by app lifetime, operation generation, `window_kind_id`, and `window_id`, and retains:

- the outer `WindowConfigPack` identity and owned file-page ingress;
- the expected inner partition id `window-config:{window_kind_id}:{window_id}`;
- one erased typed Store candidate;
- the live partition generation observed at begin, or an exact absence reservation;
- cancellation and close state;
- a typed disposer for a rejected candidate and, after commit, for the displaced live store; and
- a terminal receipt that must be acknowledged before the slot is reusable.

The erased typed owner supplies begin, advance, identity inspection, candidate transfer, and close operations. It must return rejected ingress or candidate ownership intact. A second load for the same exact key reports Pending or replaces through an explicit latest-wins cancellation sequence; it cannot overwrite the first operation entry.

The candidate phases are `Ingress`, `Decoding`, `ValidatingIdentity`, `PreparingStore`, `Ready`, `Cancelling`, `RetiringRejectedCandidate`, `RetiringDisplacedStore`, and `Complete`. Every transition consumes at most one admitted unit. `MoreWork` remains true for runnable decode and every cancellation or retirement phase.

## Validation Before Authority Change

After typed Pack and SPR decode and before candidate-store construction or live partition lookup that materializes state, validate all of these facts:

1. the outer kind is still registered and exactly matches the selected erased owner;
2. the typed initial state Pack has its exact state schema, `Pack` component, and version 1;
3. `parsed.envelope.schema == O::SCHEMA`;
4. `parsed.envelope.id == format!("window-config:{}:{}", O::WINDOW_KIND_ID, outer.window_id)`;
5. the SPR document id and schema agree with that decoded envelope;
6. cursor edit ids and all durable history invariants are valid; and
7. the app lifetime, operation generation, exact-key reservation, and observed live generation remain current.

The `source → target` and `scene:one → scene:two` inputs fail item 4 even though kind and schema match. A missing envelope, wrong component, wrong state schema, or wrong version fails during typed state-Pack decode. No rejection calls `partition`, creates a default owner, resets the live store, changes its generation, or changes its Pack/SPR bytes.

## Commit and Races

Only a fully ready candidate enters the non-suspending commit boundary. The registry revalidates the exact key, app lifetime, operation generation, and absence/live-generation witness, then either inserts the fresh candidate into an absent reserved slot or swaps it with the exact live partition. The displaced store moves back into the operation and is retired with the owner-provided disposer. Completion is published only after displaced ownership reaches terminal emptiness.

A command publication and a load for the same exact owner cannot both commit from one base witness. Whichever reaches its authoritative commit first invalidates the other's generation. The stale operation requests cancellation and retires its prepared or ready candidate. A provisioning candidate racing an accepted restore follows the same rule: the restored load owns priority during the restoration barrier; the seed remains retained until cancellation and close finish.

Cancellation never removes an operation entry immediately. App close, a stale generation, load replacement, or explicit cancellation only changes its phase. Fixed maintenance advances one entry per turn, and close revisits entries until the exact terminal proof passes. Rejected decoded candidates remain owned; no synchronization-time drop is added.

## Native RED First

Before production code, extend the existing actual-registry `window_config_pack_identity_` law and its `311.222` route. It already proves `source → target` and `scene:one → scene:two` against an existing sentinel target; extend it to prove both cases against an absent target without creating a default partition. Keep that exact identity coverage in one law.

Then add one focused retained-load lifecycle RED at reserved launch order `311.241`. It defines the new operation contract without duplicating the identity fixture:

- reject missing-envelope, wrong-component, wrong-schema, and wrong-version state Packs before authority change;
- cancel once during decode and once after a ready candidate, proving each owner remains retained and every close step respects its item and byte grant before terminal emptiness;
- race a load with a publication on the same exact owner and prove only the current generation commits while the stale candidate is physically retired;
- round-trip two saved same-kind window packs and one other-kind pack through close and a new app lifetime without cross-instance or cross-kind changes; and
- use a normal 2 MiB test thread. Typed mutation publication remains bounded at 4,096 bytes, while job and Store cleanup use their own shared physical constants.

The RED must exercise `WindowConfigOwnerRegistry` and the actual typed erased owner. A source-text assertion or a direct state decoder does not prove the authority boundary. The registered root and ticket Nx target and both launch files should be added with the native law, after the neutral oracle at `311.240`.

## Independent Implementation Boundary

The identity-law extension, retained lifecycle RED, fixed plugin window-load operation registry, exact outer/inner validation gate, stale-generation policy, atomic candidate swap, and plugin-owned cancellation/retirement state machine can be implemented in the window-config module without changing lower Pack ownership. The Store candidate API and typed replay must be coordinated at the Store boundary. The lower Pack owner retains its source, anchor, segment, catalog, symbol-table, and value cursor files; this slice consumes those public contracts and does not edit their domain-specific Generation2d/Generation3d binary call sites.

## Existing Store Runtime Boundary

Store already contains two useful lower pieces, but neither is the required ConfigStore load operation. `ArtifactStoreInitializationRuntime<P>` is domain-neutral after a caller has produced and validated the typed initial snapshot, envelope, cursor identities, replayed edit facts, and revision records. It retains the candidate current snapshot and initialization ledgers, provides bounded exact rejection cleanup through the supplied typed retirement factory, and can be adopted once through `ArtifactStore::from_initialized_runtime_with_owners`. The new ConfigStore candidate should reuse this runtime and adoption boundary.

`RetainedPersistedDocumentHydration<P, M>` cannot be reused as the ConfigStore operation. Its `from_pack` accepts a fully materialized `Vec<u8>` and an already decoded `HistoryLog`; its scan phase advances by pages, but `DecodePack` still invokes `P::decode_pack` monolithically in one step. Its `Begin` phase requires document-only `ArtifactRef`, dialect, composition, child owner, cursor, and document schema facts. Reusing it would either import document composition semantics into scalar window configuration or conceal the unbounded typed Pack decoder behind a retained phase name.

SPR has the separate bounded `RetainedHistoryDecode` and `RetainedSprVerification`, and the document composition-open operation demonstrates how to retain and retire that decoder. That history decoder is reusable after owned SPR ingress is established, but it does not solve typed state-Pack replay. The missing lower dependency remains a bounded generic `ArtifactPack` factory over the retained Pack/value token cursors, including exact `Pack` component, state schema, version, envelope schema, and inner partition-id evidence. Until that exists, plugin can prepare operation keys, generations, reservations, cancellation, and commit witnesses, while Store can expose the candidate shell around the existing initialization runtime; neither layer can truthfully make `311.241` retained acceptance green.

The frozen lower fault boundary carries no borrowed source context. Byte-source allocation rejection is `RetainedPackSourceAllocationError { allocated_bytes, reason: &'static str }`; catalog rejection is Copy `RetainedPackCatalogFault { code: &'static str, offset: u64 }`; anchor and segment currently return public `PackError`. Retained value/body parsing also returns `PackError`; its compact hot-path change is confined to a dedicated static-detail retained-malformed variant without changing cursor method signatures. The WindowConfig operation owns the first returned diagnostic value, refuses later ingress, and still closes every retained cursor. Cold `Display` or `to_string` materialization belongs at the external fault boundary after retained cleanup rather than inside replay turns.

On rejection, the operation first retires any pending source/body event under its item grant, then retires each logical token/parser/candidate owner, and finally satisfies every cursor's exact `next_release_allocation_bytes` demand before `terminal_is_empty`. It sums the structured item and byte releases and never infers physical release from logical phase progress.

## Handoff to Lifecycle Work

After the exact load slice is green, the next schema-first slice can add `RestoreComplete`, `WindowOpened`, and `WindowClosed`, exact absence reservations, and explicit per-kind default or document-dependent provisioning policies. Capture, generation, render, panels, sections, context menus, immediate dispatch, and retained commands then consume Existing, Pending, or Unregistered without materializing state.

`SurfaceHidden` must not cancel a provisioning candidate for a logically open window. It removes only that surface-generation waiter. A retained command or reserved-section request for the still-open exact window continues to observe Pending, and bounded maintenance may finish the seed without any visible surface. A later `SurfaceVisible` binds to the existing owner or the same candidate. Only `WindowClosed`, app close, stale document authority, or an authoritative restored load cancels the logical owner operation.


## Prepared Test Boundary

The existing `311.222` identity fixture now contains five cases. Both foreign same-kind addresses—`source → target` and `scene:one → scene:two`—run once with a pre-existing target and once with the target absent. The native law records actual `WindowConfigOwnerRegistry::packs()` identities before and after every load. Existing-target refusal must preserve generation, revision, snapshot backing, and the Pack identity set; absent-target refusal must preserve the exact Pack identity set and prove no target Pack appeared. Every case drains the registry through the existing bounded close loop even when admission fails.

The reserved `311.241` route is registered as `workspace:window-config-retained-pack-load-native`. Its closed schema-first fixture names nine required outcomes and distinguishes `ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES`, `JOB_PAYLOAD_PAGE_BYTES`, the 4,096-byte typed mutation bound, and the 2 MiB native test stack. Its executor accepts an already heap-pinned future before polling on that stack. The compiling native baseline uses only current concrete APIs: it materializes two same-kind partitions and one other-kind partition, saves their actual Pack/SPR pairs, closes the source registry, restores the three pairs into a new registry lifetime, rejects one malformed state Pack against an absent target, proves the registry Pack set unchanged, and drains the reopened registry with item/byte grant checks.

The same fixture fixes the candidate protocol vocabulary before production code: nine ordered phases from ingress through exact retirement, structural `retained-value-tokens` as the typed replay input, first-fault ownership until terminal close, a ready-and-current-generation-only commit boundary, eight exact state/SPR/partition/lifetime identity checks, and physical close order from pending event through logical owners and allocation demands to terminal emptiness.

The registered `workspace:window-config-pack-identity` neutral route passes all five cases through Ajv 2020 and the independent JSON Patch oracle. Its durable ticket log records `[DEBUG] Window Pack exact identity: 5 Ajv/JSON Patch cases; native admission is a separate gate` and the successful Nx target result. This is schema/oracle evidence only; current native registry admission remains the expected RED until the exact retained loader validates the inner partition id.

That baseline is not retained-load acceptance. It does not prove cancellation during decode, cancellation of a ready candidate, stale publication/load generation races, typed missing/component/schema/version envelope rejection, or physical retirement of rejected and displaced candidates. Those remain required RED cases when the Store candidate and registry operation APIs have concrete ownership-returning signatures.
