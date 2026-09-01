# Store Backbone Retirement In The Original Native Parent

## Status

Ticket-only contract/design. Read in full: 📓️runtime-opening-parent-red-packet-2026-08-28.md and 📓️native-resident-private-consumer-candidate-2026-08-28.md. Dag directly confirmed reuse of the canonical ResidentRecord family and an original privately issued parent-field binding; current generic Option handoff is insufficient.

No production, test, package, or resident17 input was edited in this continuation. All14 files captured by 📓️os-kernel-r17-native-test-first-source-2026-08-28.md were rehashed and matched exactly. Resident source remains508b78726ae6747f476fdb7d60938b3d2349ea300ef8fc55d555502a3500c49f and tests ebde45c9d5ff7f5276e7a33f464601c23b6018d3e412c67616beaeea488f297e. No Cargo, native test, or new source oracle was run.

Ticket contract: 🧪️store-backbone-parent/🔣️.json and 🧬️schema/🔣️.json. They declare the proposed phases and refusals only; no model or native execution is claimed.

## Exact Existing Child

Store/🦀️component.rs currently owns:

- ArtifactStore<P, Mutation>::backbone: ManuallyDrop<Option<Backbones>>.
- ArtifactStore<P, Mutation>::displaced_retirements: ManuallyDrop<ArtifactStoreDisplacedRetirements>.
- ArtifactStoreDisplacedRetirements::owners: ManuallyDrop<VecDeque<Box<dyn ErasedSnapshotRetirement>>>.
- reserve_owner_slots returns the exact private slot:u8, generation:u64 and remaining:usize identity. Its fixed1024 owner capacity and8 reservation entries are logical storage, not additional shell funding.
- replace_backbone_retained currently allocates Box<ArtifactStoreBackboneRetirement> only after mem::replace removes the old source.
- ArtifactStoreBackboneRetirement contains ManuallyDrop<Option<Backbones>>, ManuallyDrop<Option<VecDeque<BackboneMessage>>>, ManuallyDrop<Option<BackboneMessage>>, and ManuallyDrop<Option<Vec<u8>>>. Its Drop only asserts terminal emptiness.

The envelope's detached ArtifactBackboneRef also owns its URI String. It must transfer into the same typed retirement shell (a new exact descriptor field) instead of being cleared/dropped. That changes the shell Layout and is included before admission.

## Selected Representation, Not A Second Retirement Pool

Use one typed Backbone entry in the SAME Store FIFO. Its backing is the existing resident family's ResidentRecord<ArtifactStoreBackboneRetirement>. This replaces the backbone-specific Box allocation rather than adding a resident record plus a second Box. Other already-existing erased retirement owners remain separate variants in that same FIFO; there is no old-Box fallback for backbone retirement.

The physical child allocation is the canonical RecordNode<ArtifactStoreBackboneRetirement> Layout, including Option, alias counters and padding, not merely size_of<ArtifactStoreBackboneRetirement>. Query the family's native_layout<C,S>().record_page_bytes and actual requested Layout after the final shell definition. No numeric value is guessed without a native layout observation. The typed FIFO entry, reservation descriptor, original parent/consumer/admission record and any pending-key storage also need actual Layout accounting.

A retained Store cannot store a borrow of a movable stack facade as its lifetime authority. Its FIFO stores a private original-record key; each turn recovers a narrow typed facade through the original registered parent. The key alone is not permission. It is valid only with the original registration, source consumer, record and phase. No new owning Arc root, integer budget, allocator receipt inside S, address-containment proof, serialized key or public projection closure is proposed.

The FIFO's changed entry layout means its backing must be registered/charged during the original Opening before any producer. Existing1024×Box capacity cannot be relabeled as1024×new-entry funding. Logical capacities remain unchanged. Any backing that cannot be initialized/admitted under the existing grant requires retained preparation; no large implicit initialization or quota increase.

## Private Receiver And Required Typed Operations

For VcsArtifactApp, only its funded RuntimeAppCell/Opening resolves the closed document/config/draft/interaction Store selector. It issues a proposed StoreBackboneRetirementField<P,Mutation> binding:

1. Original native composition registration and actual parent allocation.
2. Original selected Store consumer/record, not equal schema/document/instance IDs.
3. Exact Store displaced reservation (slot, generation, count=1).
4. Exact pending/final FIFO field, vacancy, source backbone generation and allowed phase.
5. Actual record/shell Layout and reserved parent resources.

These are required private bindings, not a currently implemented public Rust type. The cross-crate family extension is after immutable resident17 and must expose narrow typed allocation/init/transfer/retire operations, not &mut Option, &mut S, DerefMut, an arbitrary field-selector string, or a caller-provided projection callback. Lost borrowed facades must be recoverable from the original parent without releasing the reservation or the source.

Proposed authored operation shapes are prepare_backbone_detach(original_parent_access, grant), advance_backbone_detach(original_parent_access, grant), and close_backbone_detach(original_parent_close_access, grant), returning typed progress/fault only. All roots stay installed; no method returns Backbones or a new whole future owning the transition. Maintenance/SpaceMember/Plugin forwarding must pass or recover that same original access context. A plain old method cannot silently obtain a global/default permit.

## Preadmission And Commit Order

| Phase | Required exact state and action |
| --- | --- |
| ParentAndFifoRegistered | Original Opening has funded Store, FIFO entry backing, operation metadata and recovery/error destination. |
| ExactSlotReserved | Validate original Store, checked next generation and descriptor/source state; reserve one exact displaced slot. No source take or descriptor mutation. |
| RecordAllocated | Same original parent/family charges the full record Layout BEFORE allocator invocation; null refusal retains source and the original preparation state. |
| EmptyShellInitialized | Initialize only the empty typed shell in already-retained record backing under its actual work grant. No whole original owner in an unwind-local constructor. |
| Prepared | Revalidate original parent/source/slot/record, full move bytes and exact mailbox reservation if session detach; all source roots remain installed. |
| Committed | One non-fallible, non-allocating guarded transition moves original backbone and descriptor into the shell, installs the same typed FIFO record entry, and publishes checked Store generation with unchanged semantic cursor revision. No whole cursor clone/reconcile or arbitrary callback. |
| DescendantsRetiring | Exact original shell advances its URI/queue/message/byte/channel owners; retained aliases block release. Existing close semantics need their own physical-accounting laws before a bounded close claim. |
| TypedShellEmpty | Actual typed terminal witness, not an empty source Option or semantic byte counter. |
| TypedShellDestroyed | Separately granted destruction of the verified empty typed shell. Receipt remains outside S. |
| BackingFreed | Original native record pointer/Layout is deallocated exactly once. Parent record retains the post-free state. |
| CreditRefunded | Refund only after actual deallocation; poison/refusal cannot reconstruct or double-free backing. |
| EntryReleased | Remove the empty logical FIFO entry/reservation and its metadata under its own admitted work. |

Current resident close_step refunds charge before record.release/deallocation in the same branch. Dag was notified: this remains an explicit follow-on limitation, not a reason to edit or retroactively reject the unchanged17 structural test snapshot. Current record handoff_into accepts arbitrary Option and cannot implement this private field transition.

Current Store bump performs unrelated cursor cloning/revision reconciliation and unchecked generation addition. A detach changes connectivity, not cursor semantics; the narrow commit must preserve the existing content revision and publish a checked generation through the original snapshot-read authority. It must not borrow bump as a bounded transaction. Refusal at any precommit phase preserves descriptor, generation, backbone, payload pointer/capacity, session channels and unsent request. After the irreversible commit, a later tail fault is recorded as postcommit fault, never rollback.

## SyncSession Original-Owner Forwarding

Current SyncSession owns its Store, Option<ArtifactMailboxSender>, Option<broadcast::Receiver<ArtifactEvent>>, and status. It sends Detach before Store admission, then clears both channel owners. The full exact constructor census found only two cfgtest SyncSession::new calls; there is no demonstrated live RuntimeAppCell→SyncSession construction link. Do not invent one. A future native standalone session needs its actual native parent registration from the SAME family; if a Store already comes from RuntimeAppCell, reuse that original Store binding rather than reserve a second owner.

Proposed SyncSessionDetachState stays inline in its original registered session and retains request/reservation/progress/first fault. prepare_detach must obtain BOTH the Store receiver and the exact actor-mailbox reservation before commit. It leaves cmd_tx/events in their original fields while preparing. Merely holding a mailbox MutexGuard across turns is not reservation authority.

The actual ArtifactMailboxSender::send uses a blocking mutex, consumes the message, has no reservation API, and invokes wakers/callbacks after its write. A narrow private detach reservation is therefore necessary: original mailbox authority+generation, reserved capacity/entry, and an exact commit_from the installed request. All existing sends must respect that capacity; no second mailbox. Prepared commit reacquires all actual guards nonblocking and revalidates closure/generation/vacancy before any Store or mailbox mutation. It performs fixed writes only; no waker/callback or allocation inside the transaction. Wake dispatch is a later retained tail with an explicit postcommit outcome, not a success/rollback shortcut.

Store connectivity transition and exact one Detach publication must share that prepared commit. If the request slot is full, byte-limited, stale, closed, busy or poisoned before commit, the Store and entire session remain unchanged. If a later wake fails or unwinds, the accepted request cannot be resent; its committed state stays in the original session. cmd_tx is not taken until its original close receiver is prepared and the actor's exact completion/handback permits it.

### Actual Event-Receiver Limitation

Cargo.lock selects Tokio1.52.3. Its pinned source at /Users/ueli/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.52.3/src/sync/broadcast.rs1548 shows Receiver::Drop locking the tail and looping unread slots until the captured tail. RecvGuard::Drop1713 sets slot.val=None when the last receiver releases it. try_recv clones the stored T through RecvGuard::clone_value.

Here T is ArtifactEvent, with Vec-backed mutations/snapshot/presence/preview and other owned values. Consequently events=None can drop multiple payloads, and a try_recv loop both clones and may drop the last stored payload. Neither is bounded typed cleanup. An actor-close flag or retaining its Sender does not prevent this last-reader path.

Keep the exact receiver installed and report pending until the native actor/event owner offers a privately issued retirement destination. The eventual canonical event lane must own typed payloads independently of broadcast Receiver destruction (e.g. bounded event-record handles with an original parent-owned typed queue); that is a separately reviewed necessary join, not implemented here. No fake detach-complete result, leak-as-success, unbounded drain or generic Drop workaround.

## Authored Forwarders Required Before Cutover

Store and its Space wrapper must stop returning raw Backbones. SyncSession, OS-host workflow wrapper, PluginApp/VcsArtifactApp and plugin_detach_backbone must propagate the typed retained progress/refusal/fault instead of async void or resolve_ready discard. Existing renderer ProgramBridge/Shell dispatch must retain original instance authority and the returned outcome. These callers are documented, not edited or implicitly granted scope. Dag owns original RuntimeAppCell/Opening and private parent receiver; Retained owns Store shell/FIFO/transition and coordinated sync forwarding; root coordinates broader Plugin/host signatures.

## Future Genuine Test Matrix

After taxonomy release and unchanged resident17, preserve the original six-law OS-kernel packet and obtain its actual compiler/semantic outcomes. Then schema/native tests for the real private parent join must cover foreign/equal-ID parents, wrong Store selector, stale reservation generation, occupied target, zero/short grants, parent capacity, null allocation, partial initialization/unwind, exact source transfer, parent/Store loss recovery, revocation, final alias, post-free/pre-refund interruption and exact once refund.

Session laws must use the actual mailbox interlock and actual retained request, not two independent calls: refusal before shared commit, same-parent commit exactly once, wake-tail unwind, actor close/ACK and event receiver last-owner preservation. Tests must close exact original roots before asserting the intended failure. None is executed or production-ready merely because this contract names it.

## Exact SyncSession Constructor Inventory

```text
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:866:pub struct SyncSession<P, Mutation>
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:877:impl<P, Mutation> SyncSession<P, Mutation>
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3887:        let mut session = SyncSession::new(store).await;
./🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs:3897:        let mut session = SyncSession::new(store).await;
```
