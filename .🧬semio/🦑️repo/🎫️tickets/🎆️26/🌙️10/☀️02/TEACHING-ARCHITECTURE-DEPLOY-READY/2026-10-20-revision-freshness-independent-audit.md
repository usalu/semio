# Independent Revision Freshness Audit

Read-only source/schema review after High confirmed neutral 4 terminal exit 1. No native execution, tests, Source modifications, pin refresh or terminal polling performed.

## Concrete Causal Boundary

The final stack enters `capture` through `current` through `captureBoundedSourceRevisionV1`, then the Graph storage consumer/writer. Thus the observed refusal is inside the revision reader self-check before the caller-target capture, rather than evidence that the storage target itself violates the finite bound. `current()` captures its own physical module using the originally loaded `byteLength`; if its live bytes grow, `before.size > maximumBytes` produces exactly `Bounded physical Source revision required` before SHA comparison. Its privately captured schema follows the same route only when compiler-captured sources exist. Other possible causes at that same guard are a nonregular/symlink leaf, nlink != 1 or invalid size. The stack does not identify which self-check path or original byte length.

Current observations: revision module 8090 bytes, ordinary single-link file, mtime/ctime 21:45:33.794 UTC; schema 4248 bytes, ordinary single-link file, mtime/ctime 21:45:16.627 UTC. Both changed after retry launch 21:31:15 UTC according to executor pin drift. Growth/self-revision mismatch is a concrete mechanism consistent with this refusal; the original executing byte length and exact loaded revision are not present in the log, so exact attribution is not proven. Earlier graph-settle conflict paths cannot supply that missing attribution.

## Contract and Repeat Safety

The schema retains finite 8 MiB Source, 64 KiB text/chunk, 256 path-unit and 64 ancestor limits; compiler context is closed version/root/sources with 1–1024 hash rows. New captured-read diagnostics describe workspace/outside scope, reducing path and source count; they do not grant read authority. Current reader checks closed context keys, canonical sorted paths, digest pattern and physical ancestry/descriptors. Captured execution refuses undeclared reads and digest drift. A fresh invocation can legitimately recapture settled current revisions, but it cannot accept the old d474 manifest whose six declared pins drifted. Review each substantive drift, regenerate the manifest only for deliberately accepted current input, rehash all pins/closed shape and reserve a fresh log before Root dispatch. Existing refusals must remain fail-closed; no loosened maximum, skipped freshness check or stale-result reuse is justified.

A separate current-source caveat: uncaptured settled target cache hits return before `current()`. Consequently repeated capture of an unchanged cached target can bypass the reader self-check on that path. This does not explain the emitted failing stack (which did enter `current`) and is not fixed or tested by this read-only audit; it should be considered during owner review.

No course/neutral phase acceptance follows from these observations.
