# Pending Effects Original Caller

This read-only audit identifies the existing physical owner and continuation driver. It changes no production source, authority, stock, deadline, native process or scheduling policy. The proposed pending-effects integration remains unimplemented and untested.

Let P be framework OS plugin modules and A be framework actor modules.

## Physical caller and decoder holder

P/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs:15–18 declares component-persistent-local ORIGINAL_ACTOR_ENTRY: RefCell<OriginalActorGuestEntry>. The with_entry function at25 mutably borrows that same owner for each actual export. poll_owned and poll_wit dispatch through entry.dispatch_value_step and pass its genuine cx to poll_kernel/poll_kernel_output. No new fixture caller or reconstructed context is used.

P/⚛️reactor/🎟️context/🌉️call/🦀️.rs:22 stores OriginalActorContextSlot in OriginalActorGuestEntry.context. Its dispatch_value_step at35–38 delegates to that existing slot and retains original typed failures.

A/🎟️retained-turn/🎟️context/🦀️.rs:18 defines OriginalActorContext<T> with the actual retained OriginalCallerSourceParent, OriginalJobDecodeOwner, cancellation, payload authority, physical spending, work recipient and sequence fields. OriginalActorContextSlot at63 retains this actual context in ManuallyDrop<Option<...>>. This is the physical source/decoder owner, not the operation/generation tuple.

At35 borrow_context borrows the existing self.source and calls with_original_decoder(&mut self.decoder), then binds the supplied epoch, existing clock binding, conserved remaining stock/work and receipts. At69 dispatch_step advances the admitted next epoch and borrows that same context. A new StepContext stack value is therefore a loan from the same actual retained source and decoder; it does not construct another caller. Same-epoch callbacks use borrow_pending_context at73; later kernel turns must enter the next admitted epoch through dispatch_step, not replay the old scope.

Job ⏱️context/🪪️binding/🦀️.rs:5–10 defines OriginalSourceBinding from operation, generation, actual payload authority identity, clock binding, clock callback and original absolute deadline. It deliberately excludes the per-turn epoch and local recipient stack address. Thus a pending owner can match this binding across valid subsequent loans from the same retained actor context. OriginalSurfaceViewRead separately checks actual native decoder ledger/retirement-recipient identity and source parent, so operation/generation equality alone cannot resume it.

The actual retained context is destroyed only through explicit paid close. That close must remain blocked until pending ViewRead and its native source/loan are fully returned; this is an integration obligation, not a newly executed proof.

## Existing host continuation

A/🏛️kernel-core/🦀️.rs:104 passes result.status==MoreWork into receive_original_call. At86–94 that function verifies the original receipt, subtracts actual spending from the original loan and retained grant/fuel, and stores continuation. No fresh grant or deadline is minted.

A/⏱️scheduler-and-scene/🦀️.rs:29–30 can_issue recognizes continuation without a mailbox event. At248–261 drain_original_call prioritizes continuation, increments the original epoch, and emits Payload::ContinueOriginalCall with the same retained work/remaining grant and loan. New mailbox mutation is not required for another turn. This preserves the physical guest entry and decoder above while the host issues the next valid epoch.

The native renderer pool receives raw OriginalCall returns in kernel-runtime/kernel-pool/turn-dispatch-and-pool-runner at169 and normal Turn results at173; normal completion reaches the actor complete path above.

## Correct guest driver mount

P/⚛️reactor/🔄️turn/🌀️poll-kernel-turn/🦀️.rs takes cx from that real export. Current order at43–54 resumes typed operations and result publication, then renders dirty surfaces; it has no retained pending-effects continuation driver.

A pending-effects owner needs explicit retained runtime state holding OriginalSurfaceViewRead and the actual callback work/output/close state. It must be advanced every kernel poll with this incoming cx, after command/response intake and before dependent render/publication; returning None from the product callback must retain that owner and arm the turn. It cannot be a local finish_exchange future or a new clone of ViewModel.

The existing scheduling seam is TurnWorkInputs and settle_turn_work in P/⚛️reactor/🔄️turn/🔦️more-work-sources-and-tracing/🦀️.rs:184–192,232–255. Add a truthful pending-effects runnable/closing source to the MoreWork result and trace. A busy lock alone is not runnable work: the existing documented typed-operation contention rule at224–230 must remain intact. Pending ownership that needs its next paid event or closure must not disappear into Some(empty). Once no retained pending work remains, it must stop arming MoreWork.

finish_exchange at runtime/exchange/plugin-exchange/exchange-finish is the admission/start point after mutation, not a sufficient resume point by itself. A later no-command poll skips that mutated-only branch. The central poll driver above is required to resume the retained owner.

## Refresh UI limitation and required proof

The RefreshUI runtime caller currently borrows a locally decoded request.view_state. That local value does not become a cross-turn pin merely because its cx came from ORIGINAL_ACTOR_ENTRY. It must donate/use the actual registered SurfaceContexts source via OriginalSurfaceViewRead before a pending continuation escapes. Returning or storing its borrowed local ViewModel pointer is invalid.

OriginalSurfaceViewRead holds the real cached_source owner plus registry mutation lease and returns the original to its exact slot before unlocking. Integration should retain this existing pin, not duplicate capture/clone machinery.

Required native integration proof: enter an actual retained actor context; admit the ViewRead; end the kernel loan; begin the next original epoch through the same context slot; verify same source/decoder identities and successful read resume; refuse a second genuine caller with identical business IDs; exercise None→MoreWork→ContinueOriginalCall→resume; then close pin/output/failure and the real actor context with unchanged aggregate stock/fuel/deadline and observed heap receipts. This audit does not claim that proof has run.

## Actual export close gap

The current close_wit/close_owned guards in plugin/reactor/context/call/exports at41/49 check only held outputs and registered owned jobs. They do not call original_loans_are_returned. OriginalActorGuestEntry exposes that predicate at call/root171 but no current production plugin caller was found.

The predicate forwards through ContextSlot to OriginalActorContext.source, and OriginalCallerSourceParent reports !borrowed_source plus SealedShared.original_aliases_are_returned; the latter compares the actual Job root's alias count to1. It is a Job-root alias predicate, not a registry ViewRead/pending-work query.

OriginalSurfaceViewRead's actual fields are scalar OriginalSourceBinding, decoder identity, registry identity/lease state and RetainedCloneSourceAdmission<ViewModel,ViewModel,()>. It does not store an OriginalCallerSourceLoan. The Job construction recipient grants/accepts physical receipts, while OriginalJobDecodeOwner retains its independent native decoder continuation and recipient; neither field list establishes an additional Job-root strong alias for this pin. Therefore the current predicate must not be assumed to cover the pending ViewRead. A native observation of its actual value while pinned was added to the new actor counterproof, with a DEBUG-prefixed diagnostic emitted only after complete closure; it remains NOTRUN.

Even if an alias is present indirectly, exports do not consult the predicate. Actual Context.close_inner at46 sets closing=true before decoder retirement; borrow_context at35 then rejects further live loans. The new pending owner needs a pre-close busy/cleanup gate covering its actual pin and output/close state before this transition. Held outputs alone do not supply that gate.

Per explicit Root authorization, only OriginalActorContext and OriginalActorContextSlot Drop guards were changed to unconditional terminal checks, removing their two panic exemptions. Their source package is outside both captured lower roots. No unrelated Drop or lower source was edited.
