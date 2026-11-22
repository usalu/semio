# Publication lease order: concurrent publisher of one shared artifact

## Symptom
The 58-minute Demonstrator release component build linked, then failed in `executeArtifactPublicationV1` with `LeaseOrderErrorV1 kind: "hold-and-wait"`. It held `artifact-scratch-private:<hash>` (exclusive, rank 6) and had waited 15 s for `artifact:<components/c42e...>` (exclusive, rank 6), held by a concurrent publisher of the same artifact.

## Root cause
`checkHoldAndWaitV1` refuses a wait longer than the 15 s grace whenever the task holds an exclusive lease of a non-inert class, unless a declared `allowed` edge covers the pair (class `artifact` to class `artifact` had none). A publication always holds two leases while it waits for the destination:

1. its private scratch claim (`artifact-scratch-private:`, named after one invocation's random directory, so nobody else can ever request it), and
2. on the general path (`stageArtifacts`, `withArtifactSourceCaptureV1`) the exclusive `artifact-capture:<destination>`, acquired before `artifact:<destination>`. The fresh-writers path takes them in the same order.

The policy already declares the private scratch exemption for the toolchain (`artifact` to `toolchain`, `held-prefix`), but not for a wait on another artifact lease. The scratch claim was the oldest hold, so it was named. After exempting it alone, the capture lease would have been named next and refused, so both holds need declared edges.

## Fix (the design's prescribed mechanism: declared, bound edges; enforcement and the 15 s budget untouched)
`🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🔣️policy.json`: two bilingual `allowed` edges, class `artifact` to class `artifact`:

- `held-prefix artifact-scratch-private:`. Private claim, no other requester, so no cycle can pass through it.
- `same-directory-suffix artifact-capture:` to `artifact:`. Capture then destination of the same path, the one order every publisher uses. The destination holder never requests its own capture.

The shared `artifact-scratch:` prefix, the capture of another path, and the reverse direction (destination held, capture requested) still refuse. The cycle detector still runs on the wait. The wait is unbounded in time only as long as the other publisher lives.

Not changed: the earlier recorded out-of-order observations of the compiler-writer image (rank 7) held while requesting `artifact:` and `artifact-capture:`. Those are a separate family (observe mode), addressed by the component-stage-writer-order ticket's driver change.

## Tests
- `🔒️leases/🧭️order/🧪️tests/🟦️.ts`: new test "a private scratch claim and the capture of the same destination may wait ...". It checks the allowed and refused combinations above, the edge bindings, reasons, and schema validity. Whole file: 78 pass, 0 fail (run directly with the pinned bun, because the resident repo service died in graph slicing under the current load).
- `📦️artifacts/📤️publication/🧪️tests/🟦️.ts`: new `testConcurrentPublishersOfOneArtifactV1`, called first in `testArtifactPublication`. It uses real leases and a test policy with 200 ms grace.
  1. Publisher A holds a shared `artifact-scratch:` claim, publisher B holds `artifact:<dest>` for 900 ms, and A publishes. A must be refused with hold-and-wait.
  2. Same with the authored policy minus the two edges and a private scratch. This reproduces the failure: hold-and-wait on `artifact:<dest>`.
  3. Same with the authored policy and a private scratch. A waits past the grace, then publishes. Bytes are verified by the Python oracle, with no leftover stage directories.
  Steps 1 to 3 ran green. A later assertion of the existing suite (line 30, `stageArtifacts` called with an undefined option object expecting the message `Explicit absolute publication lease`) fails with a bun `TypeError` thrown inside the capture module. It is not related to this change and was not touched.
- `repo run @semio-tech/framework-process:test-resource-lease-order` and `:test-artifact-publication` could not complete: the resident service died ("Slice worker exited") after long graph reads under load (1h45m and 49m, no test output). No cargo was started.

Runner: `run-publication-lease-order.ts` in this ticket (needs `SEMIO_PRIVATE_REPOSITORY` under `/tmp/sg`).
