# Pair Failure Custody Review

Read-only current source; no code or compiler jobs.

OriginalReceivingPrimary at value/🛬️decode/🏭️construction/🧾️result/🛬️receiving/🦀️.rs owns primary, quote_refusal, allocation_refusal separately plus progress. Its RetireOwned implementation quotes and retires all three; message() is only a borrowed display projection. Preserve this whole owner.

OriginalArtifactPairCause currently holds only Value(ValueError) or History(RetainedHistoryOpMetaFault), with derived RetireOwned. PairFailure holds cause+progress. Add a Native(OriginalReceivingPrimary) cause and conversion deriving progress from that complete owner; do not choose an inner ValueError.

Pair's boxed birth demand quotes size_of<Self> plus box transfers and actual capacity. Enlarging persistent pair fields therefore affects that genuine quote automatically. However returning a larger failure into a separate parent's slot is a distinct capture, not covered simply by pair construction.

Window typed-pack-load/🛬️native/🦀️.rs Sources:20 already owns failure:Option<PairFailure> and its paid retirement covers that field. pair_error:34 moves the complete error into that slot and returns a static diagnostic. It has no vacancy check and no distinct failure capture header quote added to the selected child demand. A second failure can overwrite prior custody. Reserve/capture the actual error slot before the fallible action, require vacant ownership, then retain every cause for paid close; refusing capture after the child has already produced owning errors is too late.

## Concrete work loss

- Snapshot erased trait drops inherent outcome.work.
- IFC custom snapshot receiver repeats child .result flattening.
- Pair step returns Result; phase1 forwards only that.
- WindowConfigNativePairReceiving::step returns Result.
- Window native load receive helper :29 wraps every callback in RetainedWorkOutcome{work:1}.
- Its reconcile :31 charges fuel using progress.copied_items, losing meaningful work when copied_items differs.

Use RetainedWorkOutcome<Progress,CompleteFailure> across each actual step boundary. Retain child-posted vs newly posted status exactly as current inherent receiver already does; do not repost the same physical receipt. Parent own actions must count real attempted work, while denied pre-action turns return zero. Expose actual work through reconciliation rather than deriving it from physical progress.

## Finite genuine caller regression

Extend the existing original-snapshot receiving test caller with its actual OriginalJobFixtureCaller and OriginalJobDecodeOwner. Run the same original Pack through the trait, pair, and window-erased consumer under a single-item grant. Force a real attempted validation whose work is positive while copied_items is zero, then a failure retaining all three distinct causes. Assert work consumed once, actual heap matches physical receipt, all cause owners remain held, and cancellation drains them under the same recipient. Include denied capture currencies before the attempt and verify unchanged state/no implicit Drop. Finish actual partial cursors and source/decoder closure; a model-only or constant-work wrapper cannot prove this law.
