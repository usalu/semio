# Neutral Original-Root Primary and Recovery-Pin Split

## Boundary

This is a ticket-only API/test-boundary proposal. No production Rust, native test include, allocator, Cargo metadata, Plugin, Store or CUT1 file changed. No command was run for the new reference and no native command was requested. Resident R11 remains25PASS, not an execution of this proposal. The new neutral vectors/controller are staged, not evidence of a passing gate.

The smallest next native dependency is the EXISTING semio-framework-value-resident crate alone. Its tests can directly inspect the actual private root, ConsumerPage/ConsumerHeader, recovery cursor and single Release. No public cross-crate probe facade, second copy of the ledger, second allocator, Plugin type, Runtime registry or actor-chunk fixture is needed.

The original Opening7 and full CUT1 six-law packet remain untouched. The new slice proves, at most, retention/recovery of one neutral primary consumer on its original root. It cannot establish original Runtime construction, app factory custody, private parent-field funding, Store FIFO handoff, complete callback retirement, or a SyncSession parent.

## Actual Current Source

Read the original root/Release/access implementation1–410, consumer/private storage410–630, reserve_record664–706, existing allocator/test root helpers, and existing same-crate release child. Current authority remains e23ec4068c261ef56020e4aaafd97e3bd304a6503a58e9dc1b7a3c6de576dbd3; test module e81bcca1121f724891f75f007322c61e61d46ea9dc71a601c413aeb6afbba175.

prepared_consumer is only a latest pointer. prepare_consumer currently clears it before reservation, allocates the exact ConsumerNode<C> in a later call, initializes in another call, then publishes to the existing list. The original root owns the pending page. The new primary path must not overwrite/use prepared_consumer as recovery authority.

The current Root close owns one Release and separate Destroy/Free/Refund/Clear turns. New primary custody must transfer into that exact Release, preserving every old pending/admission/record/consumer owner and all existing25 assertions.

## Exact Neutral Fields

Proposed private shapes, not mounted:

```rust
struct ResidentRegistrationStamp {
    generation: std::num::NonZeroU64,
    type_id: std::any::TypeId,
}
enum ResidentPrimaryBacking {
    Pending(ConsumerPage),
    Published(std::ptr::NonNull<ConsumerHeader>),
    Releasing,
}
struct ResidentPrimaryAnchor {
    stamp: ResidentRegistrationStamp,
    partition: ResidentPartition,
    backing: ResidentPrimaryBacking,
}
struct ResidentRecoveryPin {
    pointer: std::ptr::NonNull<ConsumerHeader>,
    registration: std::num::NonZeroU64,
}
struct ResidentRecoveryCursor {
    stamp: ResidentRegistrationStamp,
    mode: ResidentRecoveryMode,
    revoked: bool,
    next: Option<ResidentRecoveryPin>,
    found: Option<ResidentRecoveryPin>,
}
```

LedgerState adds last_consumer_registration:u64, primary:Option<ResidentPrimaryAnchor>, recovery:Option<ResidentRecoveryCursor>. ConsumerPage and ConsumerHeader both retain the actual nonzero registration; ConsumerHeader also adds recovery_pins:AtomicUsize. The initialize function receives the already reserved registration; it must not mint another identity inside allocation/init.

All ordinary consumers also receive a checked monotonic registration at original reservation. This is a narrow necessary join to the SAME list, not another registry. TypeId plus equal capacity or equal numeric generation on another root is never authority. Original root identity comes from the borrowed root plus its own structural page/stamp, not a public number. The new field/layout and write costs apply to ordinary preparation too; existing25 must be rerun unchanged, not exempted.

The root's complete Layout includes the largest primary Pending descriptor, both counted pin slots, cursor/stamp, existing Release and all gate fields. Each consumer allocation charges the new complete ConsumerNode<C> Layout. No extra heap allocation is created for a stamp, cursor or pin. The existing fixed-root bootstrap exclusion is explicit and finite: it does NOT fund any consumer page or prove that a future Runtime root was admitted before allocation. A real original parent must later fund that actual root extent.

One primary anchor per root is a first-cut API limitation only. No multi-runtime host restriction or per-runtime duplicate capacity is introduced. A second reserve returns Rejected, never silently replaces the anchor. Root closure is final; its historical generation counter is not reset/reused.

## Exact Method Surface

These methods live on the existing ResidentLedgerRoot. There is no new scheduler, parent class, facade lifetime or transport ABI.

```rust
pub enum ResidentRecoveryMode { Forward, Closing }

pub fn reserve_primary_consumer<C: Send + 'static>(
    &self, partition: ResidentPartition, grant: ResidentGrant,
) -> Result<ResidentStep, ResidentFault>;

pub fn prepare_primary_consumer<C: Send + 'static>(
    &self, grant: ResidentGrant,
) -> Result<ResidentStep, ResidentFault>;

pub fn begin_primary_recovery<C: Send + 'static>(
    &self, mode: ResidentRecoveryMode, grant: ResidentGrant,
) -> Result<ResidentStep, ResidentFault>;

pub fn advance_primary_recovery<C: Send + 'static>(
    &self, grant: ResidentGrant,
) -> Result<ResidentStep, ResidentFault>;

pub fn capture_primary_consumer<C: Send + 'static>(
    &self, mode: ResidentRecoveryMode, grant: ResidentGrant,
) -> Result<(ResidentStep, Option<ResidentConsumer<'_, C>>), ResidentFault>;

pub fn begin_primary_consumer_close(
    &self, grant: ResidentGrant,
) -> Result<ResidentStep, ResidentFault>;
```

reserve installs original stamp, exact partition charge and Pending page descriptor before it returns; it does not allocate. prepare performs only the next allocate/init/publish phase and never mints a different stamp. The original root retains failed/null allocation reservations. No default-zero stamp or implicit reserve from prepare.

Recovery begins only after Published and with the requested C matching the original stamp. Forward requires an open root. Closing requires the original root/primary close latch. An occupied cursor cannot be overwritten or restarted; any stale/foreign/mismatched phase refuses without mutation. Pending primary cancellation needs no consumer capture, because no C has entered that pending source slot.

Capture is explicitly granted: it creates an alias of the existing ResidentConsumer, not a new source owner. Blocked/not-found returns no consumer. It validates mode/stamp while holding the gate, acquires the payload alias before releasing the exact found pin and clears the completed cursor. Dropping a returned facade follows the existing scalar alias decrement; this slice does not claim that the caller's entire return/unwind/clock tail is measured by the capture call.

begin_primary_consumer_close latches whole-root close for this first cut. It may revoke the current cursor but cannot free anything. Later Closing recovery may be admitted once a revoked cursor has been cleared, so an actual live C can be recovered and handed back before root retirement. close_step may revoke that later cursor again; a suspended pointerless caller cannot prevent root close forever. This is not a live per-child release API.

## Checked Intrinsic Work Sums

The same-crate tests compute the following directly from private actual Layout/size_of types, NOT a public required_bytes helper. Checked sums reject overflow and any required turn above4096. Symbols:

- A = size_of::<Option<ResidentPrimaryAnchor>>()
- B = size_of::<ResidentPrimaryBacking>()
- P = size_of::<Option<ConsumerPage>>()
- Q = size_of::<Option<ResidentRecoveryCursor>>()
- K = size_of::<Option<ResidentRecoveryPin>>()
- U = size_of::<AtomicUsize>(), G = size_of::<u64>(), F = size_of::<bool>()
- V = size_of::<ResidentResources>(), L = size_of::<Option<ResidentRelease>>()
- N = Layout::new::<ConsumerNode<C>>().size()
- H = Layout::new::<ConsumerHeader>().size()
- T = size_of::<ResidentConsumer<'_, C>>()

| Actual phase | Declared work sum before mutation |
| --- | --- |
| Original primary reservation | A + G + V |
| Allocate pending primary | N + size_of::<Option<NonNull<ConsumerHeader>>>() + G |
| Initialize pending primary | N + F |
| Publish Pending into same list | B + 3P |
| Begin scan and first pin | Q + U |
| Advance to exact successor | H + 2U + 2K |
| Match exact next into found | H + 2K |
| Capture exact found alias | H + 2U + Q + T |
| Latch close/revoke | sum of each actually changed bool field |
| Clear next or found pin | K + U |
| Clear empty revoked cursor | Q |
| Detach original pending into Release | B + L |
| Detach original published node | existing same-list detach fields + B + L; no double-count of an already listed L |
| Destroy | actual original node Layout + L |
| Free | actual original allocation Layout + L + G |
| Refund | L + V |
| Clear primary Release and anchor | L + A |
| Final empty root observation/close | existing actual Root Layout |

3P at publication covers the existing head take, header.next installation and final head installation; B replaces the original Pending variant with Published. The tests' recorded mutations must match that actual source sequence. If implementation uses a different sequence, this table and independent test census must be reviewed before claiming an exact grant; no hidden descriptor transfer or upward budget adjustment is permitted.

H charges bounded actual header inspection separately from the write sum during scan/match/capture. The successor metadata permutation retains at most two counted pins: acquire successor before releasing current under the same gate, with no allocator, user destructor, fallible conversion or panic hook between its checked preflight and completed state. Fixed observation hooks cannot unwind. All returned continuations are pointerless; they retain only a root reference, never a saved node pointer beyond the gate.

Every mutating frontier gets (items0, exact bytes), (items1, exact bytes−1), and exact-grant desired tests. Refused calls must preserve the complete private snapshot and allocation/free counters. In particular capture short refusal cannot mint an alias that is merely dropped afterward. The previous CUT10-byte and one-short clear tests become all-frontier same-crate tests here, rather than relying on an exported layout probe.

## Same Root Close and Release

close_step keeps existing release precedence. Before any consumer selection it revokes an active cursor, clears next, clears found, and clears the cursor in separately granted turns. Each pin clear decrements the exact node stored in that slot; a positive count on another same-type node is irrelevant. Node detach requires C empty, aliases0, admissions0 and recovery_pins0.

Pending primary custody remains in the anchor until it transfers to the SAME Release. Published primary custody lives in the existing consumer list; selecting it verifies original pointer AND registration, then changes the anchor to pointerless Releasing before detach/free.

Extend the existing Release origin with private PrimaryPending{registration} and PrimaryConsumer{registration} variants, not another slot. Clear checks the original root-held anchor stamp and clears both the pointerless anchor and Release under L+A work. This avoids leaving a new Released marker outside Release that would require broader poisoned-root recovery. The registered stamp survives actual free/refund until this clear.

Sticky poison remains unchanged in principle: only already pointerless Refund/Clear and actual empty-root observation can advance. No live primary, pin, source or list traversal is allowed through poison. The source review must verify the enlarged origin/anchor do not make a pointer-bearing marker look terminal. No new unsafe Send/Sync blanket is proposed; the existing erased-root Send justification must explicitly cover these same-root counted pointers before implementation.

## Exact Same-Crate Test and Probe Placement

Proposed eventual include: one new child primary_recovery in the EXISTING resident/🧪️tests/🦀️.rs, alongside release_baseline/release_phases. The initial child may follow the existing ticket-source include pattern at this ticket's 🧪️resident-primary/🦀️.rs. No include or Rust child body is mounted/authored in this proposal; the seven exact bodies are the next test-writing boundary after API/TDD review. No new crate, feature, public cfg API or path-loaded production source is required.

The child uses crate::ConsumerNode, ConsumerHeader, ConsumerPage, LedgerState, ResidentRelease and the proposed neutral private anchor/cursor types directly. Test snapshots copy only scalar state while holding the actual root gate. They never retain a borrowed node pointer after the gate. All pointer reads after physical free are forbidden, including diagnostic reads.

The EXISTING ObservedAllocator remains the sole global allocator. Proposed minimal test-only hooks:

1. Enter: copy actual Layout plus the child’s fixed selected context into a scalar event before the current failure/delegation branch.
2. Return: record allocatorEntered=true, actual null, and whether the current branch actually delegated to System. Existing FAIL_NEXT_ALLOCATION semantics and previous counters remain unchanged.
3. Free: add the child observer AFTER the existing System.dealloc call returns, alongside the existing two release observers.
4. Before a recovery node/header load: a fixed cfg(test) child counter using the already root-owned pin registration, before dereferencing it. A resumed revoked call must produce0 loads.
5. Optional completed metadata event: fixed copied registration/pin counts only, no callback/unwind under the gate. Phase boundaries can otherwise be observed after actual methods return from the same-crate test.

The child owns a const-initialized TLS Cell of a small fixed event array (32 entries PER selected call, with overflow counter), not another allocator or runtime registration table. Selected reservation context is copied from the ACTUAL original Pending anchor under the gate, then the gate is dropped before calling actual prepare. It is diagnostic selection, never allocation authority. Existing allocator receives Layout independently; the test requires exactly one entered matching request. Observation buffers are reset/drained only outside measured calls. Any overflow fails, no dropped-event exemption.

The same-crate boundary removes the full CUT1 cross-crate512-event facade requirement. It does not change that historical packet or claim that its hooks exist.

## Seven Proposed Native Laws

| Exact native name | Actual source/test sequence and required observation |
| --- | --- |
| resident_primary_prepare_layout_and_all_short_frontiers | Root::new under existing allocation counter; measure full private root/header/node/anchor/cursor/origin Layouts; reserve/allocate/init/publish with zero and one-short at EACH phase; snapshots/allocations unchanged on refusal; exact-grant progresses; original stamp and page reside in root before first allocation. |
| resident_primary_lost_returns_keep_original_among_same_types | Unit panic after actual reserve returns, with root outside catch; ordinary unrelated and same-type consumers prepare on the same root; finish original primary; install a bounded neutral tagged/drop-counted C through the actual private consumer; lose capture after actual alias return; later same-type prepare; recover exact original tag/registration, with0 C drops during loss and cleanup before assertions. |
| resident_primary_partial_cancel_conserves_original_partition | Cancel after reserve, allocation, init, publication, plus active recovery; repeat Data and Control with a separately charged ordinary consumer in the other partition; actual System events and exact all-axis usage before/free/refund/clear; every original charge restored once, unaffected partition unchanged. |
| resident_primary_selected_allocator_null_keeps_reservation | Actual original pending reservation snapshot then existing selected failure; exactly one real allocator entry/Layout, delegated=false/null=true, exact registration/partition/all-axis charge retained, allocated bytes unchanged,0 events lost; disarm before actual cleanup; any unrelated Err fails. |
| resident_primary_recovery_short_grants_keep_exact_node_pins | Two same-type consumers with different actual registrations; exercise head→successor→match→capture and zero/one-short/exact grants from independent private Layout sums; compare PER-NODE pins, original root/stamp and unchanged alias/source counts. |
| resident_primary_paused_next_and_found_close_before_resume | Scoped worker executes real recovery call then parks AFTER return/gate release with only root reference; main closes exact next/found pins and actual original page; last pin clear precedes System free then refund; root reference remains alive; resume returns refusal with0 node loads and0 alias creation. Both next/found selected nodes are empty with aliases/admissions0, so the pin is the actual barrier. |
| resident_primary_busy_foreign_wrong_type_replay_and_stale | Hold actual gate and try from another thread: Blocked0 and no allocation; use two equal-capacity roots with equal numeric generations and current ledger.prepare_admission foreign-consumer rejection; wrong C, occupied primary replay, wrong mode, new forward capture under close, and post-terminal stale calls refuse before writes/loads. No public stamp constructor is introduced. |

The neutral C is a small test-owned tag plus bounded drop observer, not a fake Runtime/App shell. A retained external Option<C> may be used ONLY for structural test cleanup via the existing consumer handoff, never as evidence of funded parent-field authority. Keep source/consumer/capture results outside fallible closures; resume and join threads before assertions; disarm allocator hooks before cleanup. Derive close bounds from actual fixture-owned node counts, their Layout units and enumerated phase counts. Exhaustion is an explicit retained-owner failure, not evidence of liveness or permission to clear wrappers.

No live poison fixture is “cleaned” by clearing poison. Existing pointerless-poison25 coverage remains required; any additional live-poison observation must retain the exact original root and cannot claim generic completion.

## Schema and Reference Reuse

New staged files are 🧪️resident-primary/🔣️.json, 🧬️schema/🔣️.json, and 📜️script.ts. They reuse the existing CUT1 pin policy and the canonical native capacity fixture; no new runtime schema or capacity choice is created. Seven native names are declarations only, not a run roster.

The new identity vectors distinguish A:1 primary and A:2 ordinary, both Probe, plus B:1 with equal numeric registration on a foreign root. Pins are stored PER NODE. Positive vectors cover exact successor/capture, next-close, found-close. Eight negatives cover foreign/self successor substitution, wrong positive-count node release, foreign positive-count pin release, wrong same-type match, foreign capture, wrong found clear and free while pinned.

The staged Immer reference rejects each invalid transition without modifying the immutable original snapshot. B:1 keeps its own positive pin throughout A's work. These are proposed language-neutral identity tests, not native pointer authority. The earlier R4 remains only aggregate ordering/count evidence. The new script has NOT been executed, and neither its schema nor its assertions are claimed passing.

## Record Envelope Correction and Downstream Boundaries

Actual reserve_record charges envelope.checked_add(ResidentResources { bytes: Layout::new::<RecordNode<S>>().size(), slots:1, owners:1 }). Its physical allocated_bytes accounts only the node Layout; its original charge/refund includes the actual domain envelope on ALL axes.

The full CUT1 staged layout law's every-allocation node-only charge assertion is not valid for Record owners with nonzero envelopes. That historical uncompiled body remains unchanged and is explicitly NOT accepted as the future cross-crate join. Actual Runtime registry envelopes have not been selected/mounted; do not choose zero to make that assertion pass.

This neutral primary slice allocates only ConsumerNode<C> backings, whose canonical current intrinsic charge has no caller record envelope. That narrow fact cannot be exported to Store records, registry pages or FIFO entries. Before the full Runtime join the allocation observation must carry domainEnvelope and intrinsicNodeCharge separately and compare totalOriginalCharge=their checked sum; free subtracts physical node bytes, refund returns the exact total original charge to the exact partition.

Retained's additional Store finding remains mandatory: snapshot_read_leases.publish_authority is a real sequence CAS which may refuse. Any future backbone/FIFO commit must reserve/revalidate that same original witness BEFORE taking the backbone/descriptor. Recovery pins neither fund that shell nor authorize a publish-after-take assertion. SyncSession's original request/channel/runner owners remain separate, with no invented RuntimeAppCell identity.

## Next Review/Execution Boundary

1. Review these exact neutral fields/methods, sums, seven laws and same-crate hook placement.
2. Only then author the small same-crate Rust child and stage the exact include/allocator-hook delta for review; preserve all25 existing tests.
3. The sole Retained executor may then run one canonical resident-only missing-API RED on explicit GO. No Plugin graph, WGPU, Store, new target or duplicate source is required.
4. Only after genuine RED may the neutral implementation be considered; after that its exact native result and unchanged25 regression remain separate from any later Runtime/Opening integration.

No new native source or execution authority is inferred from this proposal.

