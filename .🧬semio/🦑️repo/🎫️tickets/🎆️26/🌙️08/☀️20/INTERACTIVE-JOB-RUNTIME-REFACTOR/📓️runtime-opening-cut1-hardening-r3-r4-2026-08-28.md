# Runtime Opening CUT1 Ticket Hardening R3–R4

## Scope and Current Evidence

This packet supersedes only the recovery/probe/test declaration in the earlier CUT1 R1–R2 report. It does not change native resident, Plugin, runtime, Store, the allocator, canonical includes, capacities, or the original Opening7. Six CUT1 native bodies are staged and uncompiled. Resident R11 remains the separately executed25/25 snapshot; this work adds no native result.

R3 is an actual schema-first source RED. The existing one-project Nx/Bun ticket command exited1 in strict Ajv validation: missing cut1.pinning, missing cut1.probe, and fewer than six cases. No model or native body ran. Complete tool output is preserved in 🧪️runtime-opening-cut1-hardening-source-r3-2026-08-28.json. The later pinCases declaration was added after that RED, not independently executed as a RED. R4 execution and selected source capture are recorded below after the single source-only run.

The isolated64MiB total/control8MiB fixture is unchanged. It is not a live composition choice, a replacement for the native32MiB UI subdivision, or proof that a complete app fits. All granted calls retain one item/4096 bytes. Exhaustion or an oversized actual node/write/free is an explicit failure after attempted exact cleanup, never a larger budget.

## Exact Original-Root Pin Representation

The neutral primary anchor remains owned by the original ResidentLedgerRoot before its first allocation or fallible return. It retains the original registration stamp independently of prepared_consumer, later unrelated consumers, and later same-type consumers. No Plugin type, external lookup table, movable-facade backlink, public key constructor, or capacity-equality identity is added to value-resident.

The proposed physical additions are:

- ConsumerHeader.recovery_pins: AtomicUsize on every actual consumer allocation.
- ResidentRecoveryPin { pointer: NonNull<ConsumerHeader>, registration: NonZeroU64 }. This is private root-owned counted metadata, not a public raw-pointer capability, heap allocation, Arc, or automatic payload destructor.
- ResidentRecoveryCursor { stamp: ResidentRegistrationStamp, closing: bool, revoked: bool, next: Option<ResidentRecoveryPin>, found: Option<ResidentRecoveryPin> }.
- LedgerState retains the previously declared next_consumer_registration, primary: Option<ResidentPrimaryAnchor>, and recovery: Option<ResidentRecoveryCursor>. The full original-root Layout includes these fields, the largest Pending ConsumerPage anchor variant, and the existing single Release.
- No extra root, queue, Box, general registration table or independently funded recovery object is introduced. At most two counted pins can exist in a cursor operation; the ordinary stable states contain one next OR one found pin. A transient successor pin stays within the same admitted, nonfallible metadata permutation.

PrimaryBacking::Pending owns the exact pending ConsumerPage descriptor. Publication links that page into the original list before the anchor becomes Published. Original detach must change the anchor to its pointerless release association before consumer Destroy/Free; Released is retained through Refund/Clear. A stale published pointer is never consulted after physical free.

No pointer may be copied into a local continuation surviving release of the root access gate. A caller keeps only its borrowed root reference and ordinary scalar progress. Each resumed call first acquires the same root gate and checks root closing/recovery revoked/stamp before reading next/found or the consumer. The original root reference remains live and funded even if its child page is freed while the caller is paused; this packet does not prove final root-memory retirement or callback-tail quiescence.

## Pin Acquisition, Transfer and Close Frontiers

All count and cursor mutations are checked and their work is admitted before the first write. Counts use checked arithmetic under the original root gate; no strong_count sampling or busy-loop acquisition.

| Operation | Exact ordering under the same gate | Actual write/move census |
| --- | --- | --- |
| Begin scan | Validate original stamp and live root; increment first page pin; install next lease | Consumer pin usize; Option next; complete cursor/stamp installation |
| Nonmatch advance | Acquire successor pin before releasing current; replace next; release old pin | Successor pin usize; old descriptor read/move; next Option write; current pin usize |
| Matching node | Move the exact next lease to found without changing its pin count | Both Option fields and scalar phase if used |
| Capture | Validate found; increment payload alias before removing found lease/decrementing pin | Alias usize; exact facade descriptor move; found Option; pin usize |
| Close revoke | Set original root/primary close and recovery-revoked state, no pin release | Actual bool fields only |
| Close next | Clear exact next Option and decrement its exact page pin | Option field plus usize |
| Close found | Clear exact found Option and decrement its exact page pin | Option field plus usize |
| Close cursor | Require both pin slots empty, then clear original cursor | Full Option<ResidentRecoveryCursor> extent |
| Node retirement | Require empty C, zero payload aliases, zero admissions and zero recovery pins | Existing original detach→Destroy→Free→Refund→Clear; no new release queue |

The successor transition has no allocator, callback, user Drop, unchecked arithmetic, fallible conversion, or panic hook between its preflight and completed metadata permutation. Observation hooks in that region are fixed scalar writes only, cannot unwind, and are not external callbacks. Any synthetic panic/pause hook is after the gate is released and all remaining pins are structurally in the original cursor. This avoids a lost local counted pin on an injected unwind.

Each table row must be priced from actual field/descriptor extents; the table is not a numeric price. Refusal with one byte below the complete row's required grant leaves every field/count unchanged. Both a0-byte short close and an exact one-short pin-clear are staged. test_cut1_next_pin_clear_write_regions returns intrinsic typed field/descriptor extents for the next original close transition, not its required_bytes value. The test sums these, refuses an oversized sum before mutation, executes one-short, checks the complete cursor snapshot unchanged, and later compares the actual FieldWrite sum in the real successful clear call to that independently derived sum. Its successor/capture/frontier short laws remain required for native implementation beyond this specific next/found clear test; no constant TS logical price establishes native work.

Root close may revoke and clear the pins while a pointerless recovery caller is parked. It need not wait forever for that caller. No Free or Refund may precede the last exact pin clear. Once pins and other owners are empty, physical retirement is permitted. Resumption must refuse without a payload/header pointer load or fresh alias. Sticky poison still permits only the existing pointerless Release phases/empty-root observation, never new live cursor traversal.

## Actual Native Layout Comparison

The staged layout law directly calls Layout::new for all three actual T:

1. Arc<RuntimeAppCell<OpeningApp>>;
2. RuntimeCloseEntry<OpeningApp>;
3. RuntimeActorAuthority.

It also measures RuntimeRegistryPage<T> for EACH of those T, RuntimeResidentState<OpeningApp>, RuntimeActorByteChunk, and ResidentLedgerRoot. All runtime registry capacities remain1024 logical slots, represented by64 pages of16. The old actor inline [u8;4096]+u16 cannot be credited as a4096-byte node. The proposed actor header references four1024-byte chunks without increasing maximum actor bytes.

The planned allocation rows are checked against the actual global-allocator request, not a second call to the plan helper. Every observed allocation must have exactly one declared row; each expected row must occur at its declared count. The three registry page classes must each have64 real allocation entries and64 matching concrete-T admission entries. A primary consumer must occur once.

For each actual reservation, the test checks original registration/partition/all-axis charge before allocator entry, one requested Layout, actual initialization extent, actual typed empty-destruction extent, actual System-deallocation-return Layout, and separate later refund. The global allocator receives Layout directly from the actual allocation/free call. Domain payload Layouts use the real monomorphized type independently in the test; neutral internal node/payload Layout is observed at its actual initialization/destruction call site rather than synthesized by the external pricing helper.

The current empty-runtime CUT1 constructor does not allocate actor chunks. The law explicitly expects zero actual ActorChunk allocations. Direct chunk Layout and Buffer partition vectors therefore do NOT prove RecordNode<ActorChunk> allocation/admission/Destroy/Free or full actor ingestion. That required actual-source follow-on remains separate; no synthetic chunk producer is substituted.

Actual FieldWrite observations carry role, containing allocation/frame extent, offset and intrinsic written byte count. Source hooks derive these from the typed place (addr_of!/size_of_val or the exact generic Layout), not from required_bytes. Address containment is checked only as diagnostic integrity, never as registered-field/funding authority. Native test-side aggregation verifies per-call intrinsic initialize/destroy/free/write totals against actual granted/reported work and4096. These are desired tests, not measured values yet.

## Concrete cfg-Only Probe Data and Call Sites

The new staged child 🔎️probe/🦀️.rs contains fixed data types only; no runtime or allocator implementation. Cut1ObservedEvent records sequence/call, phase, class, concrete TypeId, original registration/reservation/partition, requested and payload Layout, original all-axis charge and partition usage, allocated bytes, allocation flags, diagnostic address, actual field write, pins/aliases, and granted/reported bytes. Cut1ProbeBatch has exactly512 optional events, a recorded count, and dropped counter. Cut1NullSelection copies only observed original reservation metadata. Cut1RecoverySnapshot records whether the selected real consumer is otherwise empty/freeable.

Proposed source-owned cfg boundaries, all unmounted:

| Boundary | Exact observation/hook |
| --- | --- |
| Root primary reservation commits | Reservation event AFTER original charge/descriptor/stamp are installed and BEFORE any allocation. Expose test_cut1_reserved_primary_allocation<C>() only for that original pending primary, returning copied diagnostic selection. |
| Actual std::alloc::alloc request | Existing sole global allocator receives the actual Layout. AllocatorEnter reads a fixed TLS reservation context installed by the real caller; no plan lookup to fabricate a call. |
| Delegation/null return | Record SystemDelegation only if System.alloc is actually invoked. Record AllocationReturn with actual address/null result and unchanged original charge. |
| Node initialization | At actual typed initialize, record full initialized node region and concrete payload Layout. Do not duplicate that region as FieldWrite events; metadata writes outside it are separate. |
| Actual metadata mutation | Record exact typed place/extent after its write; PinAcquire/PinRelease/Revoke/Clear are diagnostic markers, not extra work credits. |
| Empty destruction | Monomorphized destroy_empty records DestroyReturn only after its actual typed destructor returns, with that concrete node Layout, no caller-provided price. |
| Physical free | Existing sole allocator records DeallocatorEnter and SystemDeallocationReturn around the actual System.dealloc; the latter is strictly after it returns. No unwind injection is allowed in allocator hooks. |
| Original refund | Refund observes the original Release descriptor and exact partition/all-axis charge; it cannot manufacture a freed-pointer witness. |
| Method return | One CallReturn per captured call, including refusal/error, captures actual grant and actual reported work. Test aggregation cannot silently omit a call with writes. |
| Recovery pause | Fixed test rendezvous is AFTER completed cursor-pin publication and gate release. It retains no payload pointer. The resumed call reacquires the gate and records refusal/payload-read/new-alias counts. |

RuntimeResidentEventProbe::begin/capture/finish is a proposed test collector, not a runtime permit. capture drains each fixed buffer only after the measured call returns; the test's aggregate Vec may allocate outside that call. The paused thread has its own fixed buffer; sequence numbers allow post-join ordering. A dropped event fails the law. The test fixture owns all observation buffers and thread handles before the measured callback; no logger, heap allocation, mutex, formatting or panic is allowed in the allocator/gate hook itself.

The original node/descriptor layouts and all probe setup cost must be separately declared when these hooks are mounted. No public probe/feature API or cross-crate cfg exposure is implemented here. The eventual neutral instrumentation facade must remain cfg-only and must not duplicate the ledger or its allocator. Current Rust still references unimplemented test hooks and proposed parent APIs; it is not a compiler-ready missing-API packet.

## Exact Selected Allocator-Null Law

The test first drives actual primary reservation, then reads its exact selected Layout/registration/reservation/partition/charge. Only that pending request is armed. The next actual prepare call must enter std::alloc::alloc and the existing global allocator once. Deterministic injection returns null before delegation, so its required flags are allocatorEntered=true, systemDelegated=false, nullReturned=true. This does NOT claim that System itself returned null.

The result must be Err AND the exact reservation event must precede one matching AllocatorEnter and one matching AllocationReturn. Layout, type, partition and all-axis charge must match. Partition usage and allocated bytes must be unchanged before/after the refused allocation; the original anchor remains, factory count is0, dropped events is0. No System delegation or deallocation may occur for this selected injected request. Any unrelated preparation error, missing factory, capacity refusal, wrong identity, zero attempts, repeated attempt or dropped observation fails. No allocate/free/report-null trick is permitted.

Failure is disarmed before cleanup. All intended assertions follow attempted exact runtime/root close. No strict Store/app source is created by CUT1. Unknown unexpected cleanup faults remain explicit failures with original owners retained, not cleared to make assertions safe.

## Sixth Native Law and Remaining Boundary

runtime_cut1_paused_recovery_pins_block_close_and_refund runs NextPinned and FoundPinned against the ACTUAL primary consumer stopped after publication and before RuntimeResidentState installation. It also prepares an ordinary empty consumer, not a synthetic successful runtime. The selected consumer must report C empty, payload aliases0, admissions0 and a positive actual pin count; otherwise the test fails after cleanup.

The test parks actual recovery after gate release, attempts a zero-byte close, drains the original close under declared work bounds while parked, resumes/joins before any assertions, and checks the exact pin/free/refund event order plus resumed refusal/no read/no alias. The original root value stays alive for the whole scoped reference. This is staged desired behavior only.

The original five CUT1 goals remain; the weak anyErr allocation check has been replaced by exact allocator evidence. Original Opening7/resident25 are preserved. Opening app allocation, Store field/FIFO destination funding, full actor ingestion, SyncSession's unrelated original parent and channel/request retirement, and scheduler-tail quiescence are NOT mounted or proved by this packet.

## Source-Only R4

Actual one-project command (no Cargo):
```sh
bun x nx exec --projects=@semio-tech/framework-plugin -- bun '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️runtime-opening-parent/📜️script.ts'
```

Terminal Nx/Bun exit0:
```text
[DEBUG] Opening cut1 reference=immer cases=6 transitions=32 actorBufferVectors=10 nativeRoster=6 pinCases=3 pinTransitions=25 pinOrderNegatives=6 nullObservationNegatives=8 schemaHostile=11 nativeExecuted=0 nativeLayoutMeasured=0
[DEBUG] Opening parent reference=immer cases=7 transitions=39 nativeRoster=7 hostile=2 nativeExecuted=0 liveMounted=0
```

All eight selected pre/post SHA256 rows are byte-identical:
```text
40569087a914bd4147dbb1d992393a2109b64a158a1575a5b2d7069fd9dcf327  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️runtime-opening-parent/🧬️schema/🔣️.json
12837937b1de1a136dc8a158a262f28630def28498f74420be7aa7679aea229c  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️runtime-opening-parent/🔣️.json
e2b612ad29d252df852a03b54b90e261071943d543104a1073372fb3875790ac  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️runtime-opening-parent/📜️script.ts
4c6b34304b5d9c0746b922d65d2c570bf31679f21ff0747c4b8ff6363f0b3217  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️runtime-opening-parent/🧪️cut1/🦀️.rs
dd1475d1f0123fac5f29f57b7aeccfb7e5717cade2931830e52c69daf8675a75  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️runtime-opening-parent/🧪️cut1/🔎️probe/🦀️.rs
01d75c62a738771d492b9619f8d02e87057958975a88ca1d62c415aa2d9e27e1  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️runtime-opening-parent/🦀️.rs
e23ec4068c261ef56020e4aaafd97e3bd304a6503a58e9dc1b7a3c6de576dbd3  🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs
e81bcca1121f724891f75f007322c61e61d46ea9dc71a601c413aeb6afbba175  🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs
```

Full command/tool/pre/post records are in 🧪️runtime-opening-cut1-hardening-source-r4-2026-08-28.json. This source command executes strict Ajv, Immer state references, Node Buffer vectors, explicit negative declarations, and lexical native-name inventory. It does NOT compile/typecheck the staged Rust or prove its future hooks/native Layouts. The pin reference uses logical ordering/counts, not Rust memory prices. The null negative table validates the declared observation predicate, not an actual allocator.

Only the five staged CUT1/fixture/controller/probe inputs changed for this packet. The original Opening7 body and native resident authority/tests retain their exact earlier hashes. Reports/raw evidence were added separately; no native command or compiler request was issued.

## Exact Remaining Hook Return Contracts

The proposed probe collector returns copied diagnostics, never a permit:

- finish() returns { root_layout: Cut1ObservedLayout, domain_layouts: [(Cut1NativeClass, TypeId, Cut1ObservedLayout);8], events: Vec<Cut1ObservedEvent>, dropped: u64 }. The Vec is test-owned and populated only after each captured production turn. Root/domain observations come from actual typed call sites; allocation/free Layout comes from the allocator, independently of the pricing helper.
- capture(f) returns f's exact Result, recording one CallReturn and draining the512-event buffer afterward. It preserves the original error, not a substitute observation error. Unexpected unwind is captured only by the specific outer law that requested it, never by the allocator.
- test_cut1_native_layout_plan returns root_layout, domain_layouts, actor_maximum_bytes/chunks/chunk_bytes, logical_slots[3], pages_per_registry[3], allocated_nodes rows { class, concrete_type, count, node_layout, payload_layout }, and frontiers with checked bytes. page_type(class) selects the actual monomorphized Page<T> TypeId. These declarative rows alone cannot satisfy the actual event law.
- test_cut1_next_pin_clear_write_regions returns exact Cut1ObservedWrite regions, including all descriptor/field changes of that one next/found clear call. Scalar count/bool/generation sizes are independently checked using usize/bool/u64 in the test; cursor/lease region offsets and extents are observed from actual typed private places.
- The pause handle owns only the test rendezvous/worker join, not the consumer page. wait_until_parked returns Cut1RecoverySnapshot. resume_and_join returns { refused: bool, payload_reads: usize, new_aliases: usize }; payload_reads counts EVERY consumer/header/payload pointer load in the resumed attempt, not merely accesses to C. It must remain0 after revocation. take_observations transfers its fixed-buffer observations only after join.
- Reserved-null selection is diagnostic copied data; arm_exact_null can target only the already active exact test allocator context, not confer any runtime registration or allocation authority. The real runtime call must independently have performed valid capacity/admission checks.

These are proposed cfg-only contracts. Their cross-crate visibility, physical test-buffer setup, source hooks and allocator integration remain unmounted and require a separate narrow review. Native17/25, original Opening7, Store funding/detach and SyncSession ownership are not broadened by these signatures.
