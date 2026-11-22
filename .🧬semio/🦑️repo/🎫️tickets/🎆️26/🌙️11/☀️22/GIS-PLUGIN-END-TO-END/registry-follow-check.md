# Registry follow check

- Confirmed: `observeOwnerWithRetryV1` (owner-isolation) only passed foreign failures through; a `BUILD_SOURCE_AUTHORITY` "Canonical native dependency owner required" fault was not foreign, so it became a `BUILD_OWNER_FAILED` (stable on held rows). `withOwnerRegistryCustodyV1` only follows the raw codes, so `settleOwnerRegistryV1` never saw it, and `stableOwnerFailureHoldsV1` threw it at once.
- Fix in `build/🕸️graph/📥️load/🛟️owner-isolation/🟦️.ts`: new `registryPendingFailureV1` (raw fault, or named owner failure whose code and reason are a fault of the schema list `OwnerRegistryWorkerFailures`, all four prefixes). `isForeignFailureV1` treats it as foreign (thrown unchanged, so custody and registry adoption follow it); `stableOwnerFailureHoldsV1` is false for it.
- Test: appended to `owner-isolation/🧪️tests/🟦️.ts` (all three fault shapes pass through unchanged after one attempt, named form never stable, unrelated codes unaffected).
- Run with `repo test-file`: owner-isolation 21 pass, registry-follow 8 pass, 0 fail.
- Not verified: the GIS `run-many` itself was not rerun.
