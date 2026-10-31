# Live Setup Publication Review

Read-only review of root's actual canonical setup-all run 24933, using generated/setup-live-1.log and current owning source. No source edit, live publication/cache operation, cleanup, Native job, controller signal or peer-process intervention was performed.

## Finding

The repeating .cache/b/d/m shared activity is **active publication witness progress**, not evidence of a shared dependency lease being acquired. The witness explicitly publishes this exact root and mode as telemetry. The source and log do not support the proposed publisher EX-to-ordinary-dependency-SH recursion at this point.

This does not establish completion or an ETA. The run has not reached the witnessed publication stage in the inspected log. The full real payload census can be expensive, and this owner repeats substantial custody/ancestry checks. No runtime optimization or cap change is proposed or authorized by this review.

## Actual Log Evidence

One bounded log read contained:

- 12,404 lines / 1,821,089 bytes;
- native-fixed-image:/Users/ueli/Documents/semio/.cache/b/d:accepted, then :observed;
- 6,408 typed shared activity rows for /Users/ueli/Documents/semio/.cache/b/d/m;
- first shared elapsedMs 35, last shared elapsedMs 287489;
- zero decreases of that elapsed counter;
- no lease-queue, queued, or waiting-for-lease line matching the inspected diagnostic patterns;
- no witnessed, detached, staged, published, attached, verified or settled publication stage.

These are a read-time snapshot of a growing log, not a terminal result or a global lease inventory. No process stack sampler was run. The call chain below is derived from the actual emitted resource and the owning source, not claimed as a sampled OS stack.

## Derived Call Chain And Custody

1. The production setup owner waits for publishObservedNativePlatformImageV1 before ordinary dependency opening.
2. Native publishNativeImagePublicationV1 at publication/owner source line 88 acquires the real native-fixed-image exclusive writer with timeoutMs 0, captures its writer proof, captures current observed request facts, accepts the token, and emits accepted.
3. It awaits capturePublicationBodyCommand(information), then emits observed. Thus the owning information child has returned before the shown shared root activity begins.
4. It prepares the canonical publication candidate, then calls capturePublicationPayload(before) before emitting witnessed.
5. capturePublicationPayload at owner line 79 awaits captureNativeImagePublicationWitnessV1(root, body, maximumLogicalBytes, observer, exact-payload binding, bounded-logical-observation).
6. Witness source line 24 first runs captureNativeLogicalFrameV1 in bounded-logical-observation mode. It then performs two complete content censuses at lines 25-51.
7. The metadata frame's progress at witness line 69 and content census progress at line 30 publish {resource:root, mode:"shared", elapsedMs:...}. For this call root is precisely /Users/ueli/Documents/semio/.cache/b/d/m, matching the real log.
8. Issued control.publish at process invocation source line 278 wraps the supplied resource/mode into a Script running activity and forwards it to invocation.control.publish. At line 237 that invokes the progress presentation writer. This path does not call acquireResourceLease.
9. The live observer calls checkBacking (owner line 61), which calls requireNativeImagePublicationV1 (line 49), rechecks the same existing exclusive writer proof and physical ancestry, then verifies backing facts and mounted root identity. This retains EX; it does not downgrade to a new SH opener.

The metadata/content descriptor interfaces also do not open a repository store: filesystem descriptor source lines 45/47 use actual pinned openSync/opendirSync and descriptor close under checkpoints. No ordinary withRepositoryCacheStore or dependency receiver is called by these witness functions.

Because the exact root telemetry only originates inside the witness on this inspected path, publication has advanced beyond candidate preparation. The source's effective detach occurs only after the witness returns and the witnessed stage is emitted. The inspected log therefore supports pre-detach witness activity under retained EX, with the candidate intent already prepared by its owner. It does not support a completed detach or published receipt.

## Why This Can Take Substantial Time

The witness is deliberately complete and bounded:

- the preliminary logical frame observes all entries, with metadata work capped at 67,108,864, resident rows capped at 4,160, maximum depth 64, path 256 and a 64-name window;
- each named/descriptor operation calls the observer before and after relevant boundaries;
- each observer rechecks real writer custody plus the publication directory ancestors, backing and mounted root;
- metadata lineage verification repeats across the current chain before and after visits;
- lexicographic paging rereads a directory to select the next bounded 64 names, so a broad directory is enumerated more than once;
- the following content witness performs two full streams and validates that both complete censuses agree.

The shared activity is published after work increments (progressWorkUnits 256), and its elapsed value rises. That is useful evidence of ongoing observer work, unlike an unchanging queued lease report. The log contains no entry/byte count or frame identifier, so it cannot establish current file count, exact fraction complete, or conclusively identify preliminary logical frame versus a content census. Zero elapsed resets is consistent with remaining inside one frame/census; it is not a proof of which one.

## Protocol Limit

Root retains sole ownership of controller 24933 and its existing cleanup. Nothing in these observations permits killing a holder, detaching an image, rewriting intent/receipt data, manufacturing a phase, weakening a witness, accepting a stale reader, or starting another publication. Preserve the owner and await its real terminal/stage evidence. Any later performance work belongs to the witness/custody owner under the same caps and complete payload laws, after root coordinates the live run.
