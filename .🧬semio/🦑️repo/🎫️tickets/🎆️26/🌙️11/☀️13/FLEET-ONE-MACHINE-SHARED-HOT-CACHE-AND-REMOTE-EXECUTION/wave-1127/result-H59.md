# H59 result: process.testing.capabilities-differ was a local test/suite drift, not a worker capability

## Finding
The failure is not remote-specific and not a capability of the worker: both files fail identically with SEMIO_TEST_FILE_OFFLOAD=never. A test file declares its birth with `declareBuildTestEnvelopeV1({... capabilities:{artifacts:true}})`; beneath a parent envelope (`repo test-file` / suite runner) the issued capabilities must equal exactly the declared ones (`🧪️testing/📥️invocation/🟦️.ts` line 58). The suite rows `owned-execution` and `native-build` author no `capabilities.artifacts`, so the inherited envelope has no artifacts directory and the check throws. The worker derives the identical row from the replicated table, so there is nothing per-worker to pin or advertise; no offload-gate change needed (a pin already follows the replica cut).

## Fix (test-only, no table or project.json change, so no seal-window conflict)
- `🎛️owned-execution/📥️capture/🚪️entry/🧪️tests/🟦️.ts`: declared artifacts but never used the directory -> `capabilities:{}`.
- `📦️artifacts/🏗️native-build/📥️source/📎️retained/🧪️tests/🟦️.ts`: used SEMIO_TEST_ARTIFACT_DIR for scratch -> `mkdtemp(join(tmpdir(),"semio-retained-"))`, `capabilities:{}`.
- Tried a dedicated suite row `cargo-retained-source` with artifacts:true; the suites law test requires a project target per suite (needs project.json, sealed window), so reverted at once (lock process, atomic). Final table diff of H59: none.

## Verified
- Both files: SEMIO_TEST_FILE_OFFLOAD=never and =always pass (owned-execution 2/0 on g1-m12 earlier and locally; retained 2/0 local 18 s, remote g1-m13 7.8 s).
- `🧪️testing/🗂️suites/🧪️tests` 7/0 after revert; dangling-imports clean.
- Unverified: the first always-run after the revert hit g1-m01 whose replica still held the temporary row (3 fail); repeat passed. Replica lag, transient.
