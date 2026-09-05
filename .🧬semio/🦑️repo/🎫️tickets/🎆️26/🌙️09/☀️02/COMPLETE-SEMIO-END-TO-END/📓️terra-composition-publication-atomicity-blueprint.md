# Composition Publication Atomicity Frontier

Status: RED by current source inspection. This is a read-only design audit; no Cargo/Nx/native law was run.

## Decisive finding

`VcsArtifactApp::commit_child_member` claims to publish the ownership graph, child map, and immutable child-content root together, but it awaits between the first two mutations. `ChildGenesis` is materially less safe: it stamps each new child's owner and inserts its graph edge before the next genesis construction, then returns strict owners in a plain `GroupReceipt` for a later, independently fallible map/root absorption. The current composition route is not all-or-nothing at either runtime publication or durable/recovery scope.

The smallest safe first packet is **not** a generic distributed transaction. It is a private, per-`VcsArtifactApp` prepared-child publication that completes with a short, allocation-preflighted, non-`await` linearization under the existing exclusive `&mut self` app actor. Child genesis remains excluded from public `ChildEmit` until it can hand one retained batch to that same publication boundary.

## Current write order and concrete hazards

| Path | Current source sequence | Observable/ownership hazard |
| --- | --- | --- |
| `open_child` / `register_child` | `admit_child_member` reserves immutable-root retirement, graph `OwnsAdmission`, and map slot; async open/checkout/capture follow; `commit_child_member` then awaits `member.set_owner`, awaits `graph_mut`, commits graph, inserts map, swaps root, advances generation. | Cancellation or a suspended future after preparation can direct-drop the strict member. A suspension between owner stamp and graph commit exposes owner-only state; a suspension after graph commit exposes graph-only state. The root and map are not yet published. |
| `dispatch_relation_group` genesis | For each `ChildGenesis`: async `Mc::create`, async `set_owner`, immediate `graph.insert_owns`, push live member to `created_children`; only then dispatch child/parent operations. | A later `create`, owner stamp, graph admission, child dispatch, or parent dispatch error leaves earlier created owners and graph edges. The comment that earlier strict members “simply get dropped” is invalid for `ArtifactStore`, whose `Drop` asserts terminal disposal. |
| `absorb_created_children` | For each returned receipt member: reserve content publication, obtain slot from already-mutated graph, reserve map slot, insert map, async capture/swap immutable root. | Graph is already visible. Any slot/hash/root/snapshot/retirement failure leaves graph without map/root; an error after map insertion can strand a strict member. Multiple genesis children become visible one at a time. |
| ordinary `dispatch_emit_group` | `dispatch_group` mutates member histories; then each child root is separately published with an `await`; `absorb_created_children` is called only afterwards. | Existing group compensation covers failed `dispatch_wire`/tail stamping, not failed view publication. A child event can be applied while the app continues showing the stale immutable snapshot. |

Exact anchors:

- [`plugin/🦀️.rs`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19956) reserves the child admission; [`commit_child_member`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19993) awaits owner/graph before map/root insertion.
- [`open_child`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20046) only moves a failed *preparation* member into `child_admission_abort_retirements`; the success path still crosses an async commit.
- [`ChildMemberRetirement`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8861) provides the existing bounded strict-owner close path. [`ChildContentRetirement`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:8685) can retire a published historical root only while it can locate the live member that owns its snapshot-read lease.
- [`CompositionGraph`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18848) uses an allocating `HashMap`; [`admit_owns`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18918) records authority/generation but reserves no map capacity, and [`commit_owns_admitted`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18932) inserts after the owner could have been stamped.
- [`dispatch_relation_group`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19452) performs genesis owner/graph publication before all later fallible work. [`GroupReceipt`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18807) exposes naked live members.
- [`absorb_created_children`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20864) separately inserts map/root. [`dispatch_emit_group`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:20958) presently passes `Vec::new()` genesis, so no claimed application route currently exercises a safe public child-genesis flow.

`ArtifactStore` confirms the ownership risk: its [`Drop`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:16742) requires its envelope, snapshot leases, disposer, history, and retirements to be terminal. A direct drop of a partially created member is a fail-closed bug, not cleanup.

## Existing authorities that should be reused

- `ChildMemberRegistry::admit` / `insert_admitted` / `cancel_admission` already give a fixed-slot, generation-bearing map admission.
- `ChildContentView::capture_member_admitted` builds the immutable candidate before publication; `child_content_retirements` already has a fixed admission for the prior live root.
- `CompositionGraph::OwnsAdmission` already binds an `Arc` authority and graph generation. It needs capacity reservation, not replacement with a global lock.
- `child_admission_abort_retirements` + `ChildMemberRetirement` are the correct post-open failure owner path. The candidate snapshot read must be returned to *that same member* before its close completes; it cannot be handed to `ChildContentRetirement` unless that member is installed in the live map.
- `ErasedSnapshotRead` returns its lease on direct drop, but the returned owner is later drained through the member’s `take_returned_snapshot_read_retirement` path. That does **not** make direct dropping the enclosing strict member safe.
- `VcsArtifactApp` already serializes its mutation path with `&mut self`; no new mutex, queue, or cross-app lock is justified.

## Smallest ordered implementation packet

### P0 — atomically publish one already-constructed child

1. In `store/🦀️.rs`, make `SpaceMember::set_owner` a synchronous metadata operation, and make `TransactionCoordinator::{graph,graph_mut}` synchronous accessors. Every current concrete `set_owner` implementation only assigns `ArtifactEnvelope.owner`; the macro and test wrapper delegate. Preserve async member I/O elsewhere.

2. Replace `admit_owns(&self)` with a preparation API that, while the graph is exclusively borrowed, validates identity/cycle/generation **and reserves capacity for the exact `owns` insertion**. The resulting `OwnsAdmission` must carry that reservation and release it on abort. Without this, `HashMap::insert` remains an allocation/failure point after owner mutation and the advertised all-or-nothing linearization is false.

3. Add private `PreparedChildPublication<M>` in `plugin/🦀️.rs`. It owns exactly:
   - `ChildStoreAdmission` (map reservation, root slot, graph ticket, root-retirement generation),
   - the `M` member,
   - fully captured `ChildContentView`, and
   - the exact parent store generation/projection and optional pending-pin identity.

   The current abort registry is typed only as `ChildMemberRetirement<M>`, so this requires a private fixed-slot `ChildAdmissionAbort<M>` sum/owner in that registry—not a claim that the current type can retain the candidate root. It must be admitted before any future that can be cancelled after taking `M`. Its abort state first drops/returns the candidate snapshot lease, then drives the member through the existing `ChildMemberRetirement`; it cancels the map reservation and releases the graph reservation only after no live candidate retains the member’s snapshot. Do not invent a generic disposer or raw `Drop` cleanup.

4. After all async work, cancellation checks, parent-generation/projection revalidation, root-retirement capacity checks, and graph capacity reservation are complete, invoke one non-async `commit_prepared_child`. Under the current app’s `&mut self`, it must have no `Result` branch and no allocation. Its only sequence is: stamp exact owner → commit graph admission → insert exact map reservation → swap prebuilt root → place prior root in its already-admitted retirement slot → advance generation → remove the exact matched pending pin. The commit receipt is emitted only after this sequence.

5. Keep `open_child` and `register_child` as the first consumers. The former routes a cancelled/denied prepared member through the existing abort-retirement registry; the latter returns the original member only before its snapshot lease/candidate has been taken. Do not promise caller return after preparation without a new retained return contract.

This delivers runtime all-or-nothing visibility for one child. It does **not** claim a cross-store durable transaction.

### P1 — retained child-genesis batch, after P0

`ChildGenesis` must not create/owner-stamp/graph-insert inside `TransactionCoordinator::dispatch_relation_group`. Replace the public `GroupReceipt.created_children: Vec<(ArtifactRef,M)>` with a private/retained `PreparedGenesisBatch<M>` whose entries have initial pack, full parent/slot/child coordinates, member candidate, and uncommitted graph/map/root admissions. It must be accepted only by the owning `VcsArtifactApp` transaction boundary.

The coordinator may still preview and construct candidates, but a failed later candidate, child operation, or parent operation must close each candidate using the bounded member owner before returning. No graph edge or live owner is written until the app has preflighted **every** member/map/root/graph/retirement capacity and can perform one no-await batch commit. A batch must validate `ChildSlotSpec.many`: multiple distinct child ids in a many slot are legal; the same child under different slots is not.

For durable CQRS, the parent event must contain one authenticated initial-composition receipt sufficient to recover the deterministic child relations and initial packs. The live batch commit follows the durable parent event while the app actor remains exclusive; restart recovery reconstructs the relation from that parent event. This is an event-sourced recovery protocol, not an unproven atomic transaction across independent child VCS stores. A read-only viewer only opens an already-created relation; it must never mint genesis.

### P2 — grouped mutation view publication

For ordinary `ChildEmit`, pre-capture every new immutable child root and reserve all displaced-root slots *before* `dispatch_group`. If group dispatch fails, release only candidates/reservations. If it succeeds, swap all roots synchronously before returning an invocation result. This separate packet is necessary because current post-dispatch `publish_child_content_member(...).await` can fail after durable child edits. It is not required to make P0 `open_child/register_child` truthful.

## Required laws

### Language-neutral fixture and independent oracle

Add a schema-first transition corpus for `prepared-child-publication-v1`, interpreted by an independent Bun/AJV state model. It must not emulate Rust retirement internals.

Required cases:

1. `open` and `register` success: owner, graph edge, registry entry, immutable root entry, and generation `N+1` become visible together.
2. Every pre-commit refusal—cancel, stale parent generation, changed projection, wrong owner/dialect, map/root/retirement/graph-capacity saturation—leaves graph/map/root/generation unchanged; it declares whether the original member is returned or retained-for-close.
3. Cancel at each pre-commit progress boundary: no direct strict-member drop; exact member/snapshot cleanup reaches terminal under bounded grants.
4. Replacing an existing root creates exactly one prior-root retirement and never releases the new snapshot.
5. Batch genesis: second-candidate failure, duplicate child, foreign owner, cycle, and full capacity publish no subset; a valid `many` slot with distinct children accepts only as a complete batch.
6. P2 group failure after child preparation has no root publication; P2 success makes all selected roots observable in one generation advance/batch receipt.
7. Recovery vector: a parent’s durable initial-composition receipt rebuilds the relation; a viewer-open vector cannot request author genesis.

### Real Rust laws

- Add a focused plugin law beside the current composed-parent child admission tests near [`plugin/🦀️.rs`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:35020). Use an instrumented real `SpaceMember` preparation gate and the existing bounded close fixtures. At each cancellation point inspect the existing test-only `ChildAdmissionTestState` plus graph owner/map/root state; assert all absent or all present, then drive existing maintenance to terminal.
- Add a store graph law beside current `CompositionGraph` laws: an `OwnsAdmission` cannot commit after generation/authority/capacity invalidation, and an aborted prepared admission releases exact capacity without an edge.
- Add a P1 real plugin+store integration law only after the retained batch type exists: force the second genesis candidate and the parent dispatch to fail; assert no graph/map/root visibility and drain every candidate. Do not fake this with `GroupReceipt.created_children` or synthetic direct drops.

## Acceptance and nonclaims

Accepted only when the neutral oracle and exact Rust laws execute, and when every normal/cancel/rejection path reaches existing bounded retirement terminal witnesses. Current source has no such full publication proof.

This packet deliberately does not claim: a database-wide atomic commit across separate child histories, a public `ChildEmit` genesis route, process-crash durability before the parent event carries an initial-composition receipt, or multi-user authorization. Those require the subsequent initial-composition authorization/event packet.
