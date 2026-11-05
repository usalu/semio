# Accepted Store Receipt Reuse

PendingArtifactStorePublication (composition/publication) has seven variants. Artifact, Config and Draft retain ArtifactStoreBatchPublication with public phase and receipt: Option<LaneItemReceipt>. Presence and Transient retain ArtifactEphemeralOneItemPublication with public phase and ManuallyDrop<Option<LaneItemReceipt>>. LaneItemReceipt is Copy two-u64 metadata; a synchronous accessor checking phase AwaitingAck can return the exact original accepted receipt without re-entering the store or consuming another item.

WindowConfig and WindowTransient erased publication traits expose phase, acknowledge and close methods but no receipt accessor. Their typed implementations retain the underlying durable/ephemeral publication. Add a required scalar accepted_receipt accessor delegating to that original owner, rather than inventing a new receipt, downcasting at the caller, replaying publication, or using the current store generation.

Durable advance_apply_batch returns AwaitingAck(original receipt) with turn_work0. It checks original issuer and durable-group idle before that phase branch, so direct receipt borrowing avoids unrelated new checks. Presence/transient advance methods return retained AwaitingAck before cancellation/staleness. None of these observations acknowledges or removes the receipt; ACK starts the existing closing phase.

For split commit/capture under maximum_items1, preserve the original Published ownership receipt on the commit turn, then on a later independently funded turn inspect accepted AwaitingAck before freshness/cancellation and capture the PendingPublicationPage. Do not consume a second item on the commit turn or re-run commit. The current typed operation AwaitingAck match errors when no host page exists; it must become the explicit deferred capture route.

Current Published metadata semantics matter: Artifact sets artifact_generation, canonical revision, operation.base_revision and operation.generation from the accepted receipt before next_token. Config/Draft/Presence/Transient update their own lane generations; WindowConfig retains its scalar receipt while refreshing; WindowTransient updates its captured authority generation. Freeze the token after these original metadata updates. next_token reads receiver instance, original operation, current operation.generation, result_sequence and attempt1; later framing must use the frozen token rather than a later store generation. No source changes or builds were made.


## Schema-first regression inputs

Enriched existing page-admission corpus and schema with seven acceptedReceipts lane rows. Each has original11→12 receipt, preparing/AwaitingAck/closing/complete expected getter results, and denied page-capture preserving AwaitingAck and the same receipt. New sibling receipt.test.ts uses SQLite phase-filtered selection as an independent oracle and reads both erased trait plus actual typed accessor bodies. Source law requires delegation to the retained publication rather than current store fields.

Actual pre-production run after matching the existing schema's Ajv options: 1 pass, 1 fail, 44 assertions, 393 ms. Model passes; source fails at erased transient's missing required accessor. The test body check was then tightened to the balanced actual getter body, rather than finding receipt fields elsewhere in the file. No production changes or heavy jobs. Actual native getter/capture tests remain required.


## Accessor implementation recheck

After five-file production port, exact receipt test completed 2 pass, 0 fail, 53 assertions, 340 ms. A test binder initially selected the trait/struct instead of the implementation and was corrected to target the actual erased-trait implementation header; the invariant still requires original-publication delegation. Both concrete erased implementations now pass. Native execution is not implied by this source/model green.


## Retained committed revision seam

Current recovery checks accepted receipt before cancellation and freshness, as required. A source-level incomplete fallback was identified: reading current revision only when current generation equals receipt.generation_after otherwise preserves mounted's previous revision, which can still be g0 if the committed store action returned before its outer metadata fold.

The lower owner already retains the exact solution: ArtifactStoreBatchPublication.expected_revision:[u8;32] and expected_generation:u64. Both successful commit branches in store/engine/bundle update them from the just-committed cursor immediately before installing receipt and AwaitingAck (around lines343–346 and413–416). An accepted_frontier accessor gated on AwaitingAck can return original LaneItemReceipt plus expected_revision, optionally checking expected_generation equals receipt.generation_after. No additional field or live store read is needed. Root was sent this precise seam.

The meaningful native law is commit g1, preserve its pending publication, advance the live store to g2, then read the accepted g1 receipt/revision without advancing or acknowledging the publication. Also exercise outer receipt failure before mounted metadata fold. The page token must use g1 receipt generation and retained g1 revision; neither g2 nor precommit g0 is acceptable.

WindowConfig recovery retains its scalar receipt before authority refresh; refresh progress occupies its own turn and page intent is reserved only after refresh returns ready without outstanding ownership progress. The exact current Published branch had one obsolete two-argument refresh call; runtime owner was notified while actively editing. No source edits or repeat tests were performed in this review.


## Neutral g1 recovery law

Added two frontierRecovery rows to the existing page-admission corpus/schema, normal and outer-receipt-post-failure. SQLite records the original commit exactly once, retains its g1 frontier separately, then advances live state to g2. Recovery selects only retained g1 and asserts page generation1/revisiong1, explicitly excluding g0 and g2. Strict Ajv validates the new exact neutral section and rejects an added field. Final exact receipt test: 3 pass, 0 fail, 61 assertions, 328 ms. This independently validates the neutral trace, not execution of Rust recovery.
