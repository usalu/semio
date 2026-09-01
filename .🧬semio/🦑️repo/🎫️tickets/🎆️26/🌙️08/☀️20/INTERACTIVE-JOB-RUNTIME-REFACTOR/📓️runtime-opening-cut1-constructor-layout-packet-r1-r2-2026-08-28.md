# Runtime Opening CUT1 — Constructor, Actor Layout And Exact Recovery

## Status

Ticket-only schema, reference controller and five additional staged native laws. No production/ABI/dependency/include change, native compiler, Wasm check or Store detach implementation. Resident25 and original Opening7 Rust are unchanged. The five new native functions have not been compiled or run and intentionally refer to proposed actual-runtime/neutral APIs and observation seams.

The existing Opening schema/controller/fixture were extended, not replaced by another test domain or script. The source oracle first failed on the newly required missing `cut1` declaration, then passed after its declaration/reference implementation. Original seven cases/39 transitions/two hostile checks still execute. This is declaration/reference TDD, **not** a native actor-layout or constructor behavioral RED/GREEN.

## Same-Capacity Actor Representation

Current `RuntimeActorAuthority { len:u16, bytes:[u8;4096] }` exceeds4096 bytes before padding, registry tuple or resident metadata. Paging only the1024-entry registry cannot make this owner fit.

Choose these actual private runtime shapes for CUT1; all remain unimplemented:

```rust
struct RuntimeActorByteChunk { bytes: [u8; 1_024] }
struct RuntimeActorAuthority {
    len: u16,
    chunks: [Option<RuntimeActorChunkKey>; 4],
    initialized_chunks: u8,
}
struct RuntimeRegistryPage<T> {
    slots: [MaybeUninit<(u32, T)>; 16],
    occupied: u16,
    next: Option<RuntimeRegistryPageKey<T>>,
}
```

`RuntimeActorChunkKey` and `RuntimeRegistryPageKey<T>` are private associations to original canonical resident records, not external Boxes, allocator handles or pointers to a facade. They do not make unknown records accessible. Their actual native Layout belongs in the containing header/page. The three existing runtime registries each retain1024 logical slots using64 pages of16. No new pool/queue or changed hash/collision semantics is proposed. Occupancy/initialization remain structural in those original registries.

An occupied actor reserves exactly four1024-byte chunks in the same ledger before copying its maximum4096-byte input. Empty registry slots do not already contain actors or payload chunks. Header initialization, each chunk initialization/copy and publication are separate bounded work; partial construction retains initialized chunks and the source input. The actor's byte limit remains4096, including UTF-8 encoded length; chunk boundaries may split a UTF-8 code point. Individual chunks are byte storage, not independently decoded strings. The ten Buffer reference vectors include a four-byte code point split after byte1023. The present whole `to_string` allocation is not claimed bounded or adopted by this packet.

Native pricing is requested Layout, not `payload_bytes` alone:

| Owner | Required native extent |
| --- | --- |
| Actor chunk | Actual `RecordNode<RuntimeActorByteChunk>` including Option/alias metadata, plus its admission node and exact root descriptor metadata. |
| Actor header | Actual containing registry slot/page Layout, including all four key fields, length, initialization state and alignment. No second independent charge for inline fields. |
| Registry page | Actual `RecordNode<RuntimeRegistryPage<T>>`, for each of the three actual T types;64 pages per existing registry, not a new1024-slot authority. |
| Runtime state/cell | Actual registered control/consumer node with original preparation cursors and source-owned key slots. No inline PA assumption. |
| Destroy/Free | Actual node Layout plus actual R11 release-slot and write extents, not merely1024 bytes or the header size. |

Each separately allocated node is charged its exact requested bytes and one slot/owner in the original Control partition before allocation. Payload moved from an existing separately funded source keeps its original charge until that source is retired; a Control shell is not a refund. Inline metadata is counted only in its containing Layout. The source model computes logical storage arithmetic only; no native fit measurement has occurred. If even a16-slot page or any node's actual Destroy/Free/write frontier exceeds4096, the native law must fail/refuse after cleanup. Do not increase the grant or capacity to pass. Single AppInstance backing and full application-field layout remain outside CUT1 and retain the earlier explicit no-fit prerequisite.

## Original Key Owner Before The First Return

The neutral role is **one primary consumer anchor**, not a Plugin-specific field/type inside value-resident. Its only purpose is retaining the original top-level consumer registration through lost construction/capture returns. It is an inline field in the one original `ResidentLedgerRoot`; no external registry, returned-local-key owner, per-runtime ledger or movable facade backlink is introduced.

Concrete proposed neutral fields:

```rust
struct ResidentRegistrationStamp { generation: NonZeroU64, type_id: TypeId }
enum ResidentPrimaryBacking {
    Pending(ConsumerPage),
    Published(NonNull<ConsumerHeader>),
    Released,
}
struct ResidentPrimaryAnchor {
    stamp: ResidentRegistrationStamp,
    partition: ResidentPartition,
    closing: bool,
    backing: ResidentPrimaryBacking,
}
struct ResidentRecoveryCursor {
    stamp: ResidentRegistrationStamp,
    closing: bool,
    next: Option<NonNull<ConsumerHeader>>,
    found: Option<NonNull<ConsumerHeader>>,
}
```

Add `next_consumer_registration:u64`, `primary:Option<ResidentPrimaryAnchor>` and `recovery:Option<ResidentRecoveryCursor>` to LedgerState. Add the checked registration generation to ConsumerPage and initialized ConsumerHeader. The existing ordinary `pending_consumer`/`prepared_consumer` path remains distinct. No Plugin type, numeric instance ID or app factory enters these neutral declarations.

The primary Pending variant owns the **existing ConsumerPage descriptor itself** before allocation; it is not a second allocation protocol. Before its first possible return, the reserve step checks generation overflow, exact consumer Layout/capacity and work, debits the original partition, then installs the stamp and pending descriptor in this original root slot. Failure before admission leaves no anchor or allocation; failure/return loss after admission leaves that exact pending descriptor and charge owned by the root. This pending primary can coexist with ordinary consumer preparation, so later same-type/unrelated registrations do not overwrite it. On publication, the page moves once into the existing consumer list, while the anchor retains the exact stamp and a checked association to that live header.

Capture returns only a borrowed, phase-qualified facade. A local copy of the stamp is not authority or its durable owner: the root slot already owns the original stamp. Dropping/losing the first creation result, the first capture result, or a later facade does not remove it. Recovery begins from the anchored stamp, not TypeId or `prepared_consumer`; its one inline cursor examines at most one original list node under each admitted turn and compares root membership, exact generation and type. Pending recovery refers to the owned Pending descriptor without reading uninitialized C memory. The cursor itself is root-retained if its return is lost.

The facade's original root association is checked **before** reserve/allocation. Supplying a second equal-capacity root, an ordinary same-type registration, a stale facade or a closed stamp cannot move authority. No public stamp/key constructor exists. TypeId is a casting qualification after exact identity, not selection. Checked generations never wrap or reuse within the root's lifetime.

One primary owner per root is an explicit **CUT1 limitation**, not a new general host-composition policy. A second primary bind refuses while this anchor is occupied. Ordinary consumers, including the same Rust type, remain legal. This first-cut packet must not be installed as a blanket restriction on existing multi-runtime native hosts, or evaded by creating independent per-runtime ledgers. Their actual composition-owned multi-runtime slot backing is a later funded integration. No live host/guest cutover is claimed here.

## Neutral And Runtime Call Boundary

Proposed neutral operations are `reserve_primary_consumer<C>(partition,grant)`, `prepare_primary_consumer<C>(grant)`, `begin_primary_recovery<C>(closing,grant)`, `advance_primary_recovery<C>(grant)`, `primary_consumer<C>(closing)`, and `begin_primary_consumer_close(grant)`. They operate on the root-owned primary slot and return progress or temporary access, never move its stamp ownership into the caller. A repeated preparation resumes only that exact anchored registration; it does not reserve another consumer. Closing/replayed forward admission refuses before allocation.

The actual `PluginRuntime::new` becomes an unbound allocation-free facade; it does not allocate its three registries or create an app. `prepare_resident(root,grant)` uses the neutral anchored path to construct the **actual** retained `RuntimeResidentState<PA>` and original registry pages. `RuntimeResidentState` is the moved storage of the existing PluginRuntime fields, not another runtime or scheduler. The original facade contains only the checked original association after capture; the retained root remains the owner. `recover_original_resident`/`capture_original_resident` use the primary recovery path and can resume a Pending constructor without fabricating an initialized runtime value.

No app factory, Store, ActionBus or bundle producer is invoked during these five CUT1 laws. This is the actual runtime's empty/root constructor, not a substitute successful PluginApp. The staged actor native law independently measures header/chunk/page shapes and verifies actual empty-registry allocation events; it does not yet execute full actor input consumption or host Open. The Buffer source vectors are byte-reference tests only. Actual actor-slot reservation and original incoming input custody must join the ordinary Open path before production actor use.

## Root Census, Cancellation And Close

The original root's fixed bootstrap Layout must now include the full Pending ConsumerPage variant, anchor/stamp, recovery cursor and generation counter. This is the one explicit bounded root metadata exclusion, not an unpriced table. Every consumer/runtime/registry/actor descendant remains separately charged. The native inventory must report `Layout::new::<ResidentLedgerRoot>()`, actual Option/enum discriminants/padding, ConsumerPage/Header deltas, all node Layouts and each mutated field extent. No TS296/264 price or guessed byte count is used.

Cancel the anchored primary first: revoke forward capture/preparation and preserve its Pending/Published source for exact close recovery. An active recovery cursor is either advanced in close mode or retired under its own grant; it cannot retain a stale pointer after the node is freed. The root's ordinary consumers are not silently cancelled merely by primary cancellation. Global root close eventually revokes/closes them explicitly.

Before actual primary consumer Free, the anchor must lose its live-node pointer under the same original close authority and become a pointerless Released marker. Retain its stamp and exact release association until that consumer's R11 Free→Refund→Clear completes. Only then, under a separate grant, retire anchor/recovery metadata and permit original root terminal emptiness. The new fields must be part of structural-empty/final-root accounting; leaving a dormant anchor and returning Complete is forbidden. Live poison is not bypassed; R11's pointerless-only poisoned progress remains the limit. No bound-child/live Store release API is mounted as part of CUT1.

## Five Staged Actual Native Laws

`🧪️runtime-opening-parent/🧪️cut1/🦀️.rs` is intended for the same existing Plugin runtime test scope as Opening7, after its fixture/helper definitions. It is **not included**. The module has no replacement app, fake terminal disposer, direct state clear or actual production implementation. It intentionally references the proposed actual types and cfg observations.

1. Observe actual `PluginRuntime::new` and zero-grant preparation: no allocation/factory call or root anchor mutation; close exact originals before assertions. Current eager new is expected to violate this law when selected, not assumed repaired.
2. Independently measure actual header/chunk/page native Layouts and compare with every planned write/Destroy/Free frontier. Inspect original allocator request/reservation events and exact all-axis charge before System allocation. This is not a boolean “admitted” helper. Actor input bytes are separately covered only by the source reference at this stage.
3. Inject a panic after actual original anchor placement before its creation return, drop the original facade, prepare an unrelated and an ordinary same-type consumer, then recover the original primary. Separately lose capture return. Hooks run after committing the original source and releasing the gate; they do not manufacture a poison-recovery success or move C into the panic payload.
4. Cancel original primary, attempt forward/foreign-root/replayed/stale access, observe zero allocations and unchanged original registration, then close both exact roots before assertions. The same-capacity foreign root is not accepted as identity.
5. Make the actual selected runtime-backing System allocation return null after root reservation; retain the exact anchor/partial registries and close them. No allocator unwind injection, generic allocation failure in the test driver or quota increase is permitted.

`RuntimeResidentAllocationProbe` and the `test_cut1_*` seams are proposed cfg observations, not implemented success helpers. They must be backed by the actual allocator call boundary and original reservation state, use fixed nonallocating event storage, and expose dropped-event counts. Any overflow invalidates the evidence. If a Plugin test allocator is required, it must be the sole allocator in that test binary and preserve System's non-unwinding contract; this packet does not install one or reuse the resident test binary's allocator implicitly. Exact hook/include review remains required before a native compile request.

Drivers reuse the existing captured-layout/slot/transition work bound, not100000 retries. Error, oversize and refusal observations are collected while the original runtime/root remains retained. Cleanup is attempted before intended assertions and nonterminal cleanup is itself an explicit failure. Final staged-test hardening after the source GREEN retained the close-start Result in its debug/assertion path and required the lost-capture panic followed by another unrelated/same-type registration and exact recovery; no source rerun was claimed for those bodies. The source controller reads only the five native function names, never executes or typechecks these tests.

## Actual Source Execution

Both commands used exactly one existing Nx project and the existing ticket controller:

```text
bun x nx exec --projects=@semio-tech/framework-plugin -- bun '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️runtime-opening-parent/📜️script.ts'
```

R1 exited1: strict Ajv reported required `cut1` missing in the old declaration. No model/native cases executed in that run. Complete original tool output is preserved in `🧪️runtime-opening-cut1-source-r1-r2-2026-08-28.json`.

R2 exited0 with actual output:

```text
[DEBUG] Opening cut1 reference=immer cases=5 transitions=26 actorBufferVectors=10 nativeRoster=5 hostile=5 nativeExecuted=0 nativeLayoutMeasured=0
[DEBUG] Opening parent reference=immer cases=7 transitions=39 nativeRoster=7 hostile=2 nativeExecuted=0 liveMounted=0
```

Ajv/Immer/Node Buffer are test-only third-party/reference dependencies. The model explicitly rejects the latest-pointer result after same-type registration and validates original anchored identity. Five hostile declaration mutations reject actor cap/chunk changes, latest-pointer authority, returned-local-key ownership and a new external table. No native private identity, allocation fit, producer timing or live liveness is inferred from this model.

No pre-run hash inventory was captured for these two source runs, so there is no pre/post stability claim. Current final source hashes are reported separately below; the source GREEN native roster remained five names after the small cleanup-result hardening. Original report R3 and root's prior Opening7 reference remain historical and unchanged, not rewritten as CUT1 evidence.

| Current file | SHA256 |
| --- | --- |
| Original Opening7 Rust, unchanged | 01d75c62a738771d492b9619f8d02e87057958975a88ca1d62c415aa2d9e27e1 |
| New CUT1 staged Rust, after post-GREEN hardening | 5056adca25acb16cd89ec28e41ec1cbcb46adc25fd91c992b822dae810c512fa |
| Opening fixture | 8dd5171a1d5c5340e945f07fa3a8018154c8e01e9b43a8929e918a6e23c3e67a |
| Opening schema | 696d8784498c184182af6b8a642c96f02052dd6a47c46779ef3231cd280fa298 |
| Existing Opening controller, extended | 79a1ad43b753af9304560ad1b969daed2ada690bf3d22b513d49407ad194ec2c |
| Exact R1/R2 tool-output JSON | 0cc2f7b635d14f0cadecf7f56ad3bd8ebcffc14e917a090c641759bbb0141b51 |
| Native resident authority, unchanged | e23ec4068c261ef56020e4aaafd97e3bd304a6503a58e9dc1b7a3c6de576dbd3 |
| Native resident tests, unchanged | e81bcca1121f724891f75f007322c61e61d46ea9dc71a601c413aeb6afbba175 |

## Boundary For Review

Only existing ticket schema/fixture/controller plus the new ticket native file, this report and exact tool JSON changed. No canonical include, runtime parent, Store/FIFO, native value authority, UI/WGPU, async/job or SyncSession source changed. The existing SyncSession detach remains blocked on its own real original parent/request/channel retirement; this primary consumer anchor is not lent to an unrelated SyncSession.

Next approval must review this neutral primary-anchor API/actual Layout census and the five test observation hooks before a sole-executor missing-API/behavior RED. This report is not a compiler-ready production release. The later same-root single-Release bound-child proposal remains separate.
