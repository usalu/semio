# Genuine Single-Enqueue Observer RED Seam

## Existing Single Operation

RuntimeMailboxInner::enqueue in WGPU glue11825–11836 already performs one operation: create completion; acquire actual completions mutex; enqueue; release guard; mark scene changed; inspect waker/call it. The interval after releasing the actual completion guard but before scene invalidation is inside this one operation. This is the valid place for a composite-publication test. It is not the separate mark_scene_changed/observe_input_generation sequence, which remains legitimate.

## Minimal Semantics-Preserving Test Extraction

Propose extracting only the existing body after completion creation into a private free function enqueue_runtime_completion(completions: &Mutex<RuntimeCompletionQueue>, presentation: &RuntimePresentationAuthority, waker: &Mutex<Option<RuntimeHostWaker>>, completion: RuntimeCompletion) -> bool. RuntimeMailboxInner::enqueue constructs its existing completion then calls this helper. The helper initially retains the exact old lock/enqueue/drop/mark/wake behavior. No production fix or new authority is introduced by that extraction.

A cfg(test)-only thread-local interlock immediately after the actual completion guard is dropped pauses the writer before scene invalidation. The native law constructs actual RuntimeCompletionQueue, RuntimePresentationAuthority and a real waker mutex with None; it calls the same production helper with a Resize completion. No GPU/window/AppRuntime construction or fake queue is needed. The reader takes the actual queue try_lock once and reads presentation state. Current behavior exposes one ready completion with the old scene revision. The test resumes/joins the actual producer and drains the scalar Resize completion before its unchanged no-half-operation assertion.

After the guarded-state repair, the same internal interlock moves to between the actual first and final writes while the composite guard remains held. A reader must get Busy, not a partial tuple, and after release it gets the completion plus new scene revision. It must not alter observed build input. Poison/full/refused/held-reader laws then use that same real storage, not substitute atomic counters or a mock verdict.

This first RED covers completion publication plus scene invalidation only. It does not certify revision reservation, input queue, surface lane, callback timing, physical funding, superseded-owner retirement, waker scheduling, or the full metrics transaction. Those remain necessary later joins.

## Reviewed Canonical Packet

Parent approved the extraction and taxonomy acknowledged the exact glue preimage notice without refreshing or publishing any pinned preimages. The native leaf is now staged at ui/host/📥️input/🎟️admission/🔗️commit/📥️enqueue/🧪️tests/🦀️.rs, with its own strict schema and fixture. The earlier staged semantic filename was moved to this canonical leaf. The older independent-update diagnostic remains unmounted and is not an acceptance test.

The dedicated fixture has exactly three observed fields: ready completion count, scene revision, and unchanged observed build input. Source revision11 and scalar Resize640×480×1 are checked after cleanup. It is not the separate seven-field metrics model. Node Buffer independently encodes three24-byte tuples; strict Ajv rejects five invariant changes.

Both writer and reader run in separate scoped threads. The coordinator waits only bounded durations for publication and observation, always releases the writer, joins both threads, and drains the actual queue before assertions. This permits a mistakenly blocking reader to finish after writer release rather than preventing cleanup. The read-before-release timeout is a test failure, not a terminal or progress claim.

That pre-mount review boundary is now superseded: the approved private helper extraction, thread-local interlock and canonical native test module are mounted. Before-edit glue SHA256 was214c5ece5918ed0c3255828da5ac0f9441ddc164b7b2efa88cd879b5f6c01c28. The existing independent scene/build-input test remains unchanged. Native execution is pending the held dependency snapshot. Current WGPU TestScript invokes Cargo then Vitest using the same arguments; a RED terminates before Vitest, while a future native GREEN requires an honest pure-native route.
