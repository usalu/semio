# Funded Publisher Body Control Handoff

Source inspection only. No Native operation, live payload path, protected census, timing run, pending cleanup or runtime edit. This traces current P/W/Original source; it does not reconstruct the failed process's exact wall-clock cutoff instant.

## Actual reference chain

`publishNativeImagePublicationV1` calls `withAcceptedPhysicalCleanupV1(options.control,{version:1,kind:"native-image",resource:"native-fixed-image:"+options.directory,reserveMilliseconds:10000},...)`. The callback receives `body`, and the actual exclusive lease acquisition and captured writer proof use that exact body reference and its signal. The private acceptance receives `body`, checks it is the accepted cleanup control/refinement, and retains it as `scope.body`.

`capturePublicationPayload` calls actual W with `scope.body`, never `scope.options.control`. Its observer calls `checkBacking` and `requireNativeImagePublicationV1`, whose body path checks `scope.body` and validates its exact body-bound writer proof. Every W `check`, I/O/cursor check, descriptor checkpoint, progress and continuation closes over the passed `control`. The POSIX event-service observer also receives that control. Body child commands derive actual requested controls from `scope.body`; they inherit its parent deadline. No handoff in this current path widens W/observer back to the broad incoming control.

## Reserve mechanism

`withAcceptedPhysicalCleanupV1` takes current actual remaining, bounds a whole-operation deadline by the inherited deadline, requires more than the reserve before effects, and computes bodyDeadline=wholeDeadline-10000. Its actual issued body control is a child with that finite requested duration, inherited parent control and a body cancellation signal. The private body scope's absolute deadline is set to that cutoff. Both its timer and every real remaining call enforce it; remaining walks the actual parent chain as well as Original validity. Cleanup separately retains the unchanged whole deadline and cleanup signal.

`beginAcceptedPhysicalCleanupV1` closes body admission, drains its accepted children, retires the body, and enters cleanup. P's cleanup admission uses `checkAcceptedPhysicalCleanupV1` plus `requireAcceptedPhysicalCleanupWriterProofV1`, which verifies the retained original writer and actual accepted body ancestry under cleanup funding. It does not require a live ordinary body check after retirement. The current Source therefore retains the authored10000ms reserve; no lost-funded-control bug was found.

The generic text `Original resource loan deadline exhausted` can come from the finite body control's inherited/deadline arithmetic. It is not evidence that W was passed the broad Original. The saved Tools log lacks per-control cutoff/remaining timestamps, so this inspection does not claim the refusal happened exactly at110000ms or that cleanup completed within the reserve. Scheduling and actual cleanup work remain runtime observations; no budget was increased.

## Meaning of custody false and pending

P's initial witness runs before effective detach, while its `effects` remains false and `physicalSettled` initially true. On a body failure it still enters funded cleanup and attempts private publication closure; the actual guard releases only if physicalSettled remains proven. None of this completes qualification or calls the pending-finish path without witnesses.

The asset-publication ticket helper is additionally conservative: after any publication-started failure it sets `uncertain=true` and retains its infrastructure handle/remaining guards rather than claiming aggregate closure. Thus PID20908's ownedCustodyClosedfalse does not identify a particular leaked guard, prove a reserve handoff error, or authorize clearing pending. Its actual aggregate false result must remain false; individual closure would need fresh authorized evidence. Durable pending is cleared only by the canonical qualified owner path described in `pending-publication-owner-readiness.md`.

## Current Source hashes

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/🎛️owner/🟦️.ts`: `e08a96bd367f146d6f35d1c70abd6da9dff51f4c7062108e5a815e4ef22b96ce`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🟦️.ts`: `c823a977d7644c46898e9789246e96db4a863608fb690a5d35d2ee795ffdcf5d`
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`: `f88e958a0d55610a5a6b8b4cad8b87bd0b3d16f9d85c84c01a54f78fdf47f173`
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts`: `efeb9ff3819acdd475ebeee416ae04b776ceeb8d482f14e258da23e16df40512`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/asset-publication/📜️script.ts`: `233b406c77bae55cb37dc8f5fc94c8ea72d0acfd59691b91fdc4ceff095b944a`
