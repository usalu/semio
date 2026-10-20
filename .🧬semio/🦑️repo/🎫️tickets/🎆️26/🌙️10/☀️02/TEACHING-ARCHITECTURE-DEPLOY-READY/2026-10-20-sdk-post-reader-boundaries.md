# Exact Post-Final-Marker Reader Boundaries

Bounded read-only Source audit only. The retained `🕳️require/📜️script.ts --post-reader` command (session76480) terminated0 after reading seven exact private Source/schema files/137739bytes against base+24/active diagnostic provenance. Metadata/log are `2026-10-20-runtime-sdk-post-reader-source.json` and `...source1.log`. Three host implementations have drifted (physical Source reader, leases, invocation); conclusions below use actual private bytes. No Source module/API was imported/evaluated, no Native authority or inventory was invoked, no process sample or managed/private mutation occurred. Namespace helper3ded0… remains unchanged.

## Remaining Boundary Sequence

1. After final-recheck-after, the SDK reader immediately checks the same current child control.
2. It awaits readNxSdkSourceMetadataReadV1 for the held base's `.semio-sdk-source.json`, hashes those returned bytes synchronously with SHA256, and compares material.sourceIndex. The final read/hash has no SDK-stage marker.
3. If the body failed, it throws that exact retained primary error, even if a later final recheck/index operation also failed. Otherwise it returns the body value or final failure.
4. withResourceLeases catches/preserves that primary and synchronously reverse-releases its held resource leases in finally. For this reader the retained snapshot resource is shared. The concrete SQL close is synchronous, without a JavaScript await/progress loop.
5. The requested child control's settle catches that result/error, closes child admission and awaits its accepted pending-operation drain, then retires the child and runs synchronous retirement callbacks. Its wrapper promise is tracked on the parent, not on the child being drained. Further caller/Store/Native finalizers are outside this reader boundary map.

These are ordered possible remaining operations, not the observed current site after a marker. A quiet interval may occur before any of the following return points; no live accepted-operation set was read.

## Exact Bounded Final Index API

SDK index module90284063d28cdb3f3305f5f685a5b31868f279707c70b7f7af22f4042616251b/21758B, read functionline35, delegates to readPhysicalCargoSourceV1 with version1, actual enforcing root, expectedBytesnull, remainingFiles1 and the default maximumIndexBytes16777216. The schema92715311…/7710B owns that16MiB ceiling. A23MiB index would refuse before buffer allocation under this call; no actual23MiB index is asserted. The last measured earlier-generation sealed index was2017181B; this audit does not silently promote that old extent to a fresh current-generation observation.

The actual physical reader isabda94bf5cd996ab8de396ce9d3129f8cd0c4f5d322c09e69f4428f1df2a178e/5485B, lines12–18. Its protocol schema9df1d997…/2018B fixes maximumChunkBytes65536 and maximumAncestors64. It validates the closed request twice, canonical bounded path and containment, captures nofollow directory ancestry and exact regular single-link file extent, then opens a nofollow descriptor. It validates descriptor identity before allocation. ENOENT returnsnull, which the SDK wrapper converts to its missing-metadata refusal.

It allocates exactly the admitted size, reads the complete extent in positive-progress chunks, yields the same child control after each chunk, replays the entire extent against a second bounded verification buffer, checks one-byte EOF, and rechecks ancestry/path/descriptor identities throughout. The actual frozen reader rechecks at the start of every chunk in both passes; newer host code differs here, so host timing/count assumptions are excluded. It awaits descriptor.close before returning and preserves read/close failures (aggregating when both fail). The SDK then copies the returned bytes into a Buffer and hashes it. This is finite byte/ancestor work with inherited cancellation/deadline, not a new unbounded Source traversal.

For a successful extent B with full-sized primary reads and C=ceil(B/65536), the two data passes yield2Ctimes, perform2C+4complete ancestry/descriptor rechecks, then close. Primary reads can legally be shorter but positive; this count is a conditional full-chunk model, not a measured count or strict maximum. For the old2017181B observation, C=31 gives62yields and66rechecks; no current extent/duration is inferred.

## Exact Lease Callback Return and Close

Actual private leases26415d2697d125b9c8447648d8cbdf159a540f7bfc060cdf4195ccba705ccf4b/58540B (host differs) has withResourceLeases at377–385. It awaits operation, preserves arbitrary primary failure includingundefined, then calls finishResourceLeaseCleanupV1 over reverse-held releases. That helper ise60723e186afa25781ab96d2248e05636fcf29c74e91c2f9bc03ed5a1f49ee4a/7550B, line20: a finite synchronous loop attempting every operation and retaining the first/primary error. No accepted-promise drain is hidden in this helper.

The concrete lease release at208–210 is idempotent: phase→closing, database.close, releasedtrue, phase→closed. The Bun database adapter at134–140 calls database.close(true) synchronously; Node calls DatabaseSync.close synchronously. Statement reads finalize their prepared statements. There is no release retry/spin/await in these exact JavaScript branches. This does not rule out time spent or a blocking/failure behavior in the native SQLite close binding; it simply locates that as a synchronous API boundary rather than a JS pending drain. No live SQL connection was accessed by this audit.

## Exact Requested-Control Settlement

Actual invocation4fc8e1d6c25707bfc0d99570b2f5c7b496dee5beb41e76e112896b5a6dcfa215/34678B (host differs): withRequestedIssuedResourceControlV1(line102) tracks the child settlement on child.parent. settle(line54) awaits run then drain regardless of a retained failure. drain(line53) sets acceptingfalse and loops while child.pending.size, awaiting allSettled of accepted snapshots. track(lines47–51) rejects new work through closed ancestry and registers pending-deletion callbacks on each admitted ancestor. The wrapper's own promise is excluded from the child's set, so no intrinsic self-enrollment is present. No numerical drain timeout/periodic marker is added here; settlement depends on actual accepted descendants completing. Their runtime identities or dependency graph remain unknown. Retiring live state/timers happens after drain in finally.

If another root-authorized observation becomes necessary, the smallest markers are final-index-before/after, raw lease-close-before/after, then requested-child-drain-before/after with a numeric pending count. They should be considered only after the first actual body-error metadata and existing stage sequence are recorded; no such instrumentation is authored/applied by this audit. Exact primary/custody/error semantics must remain unchanged.
