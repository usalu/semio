# H36: Go -buildvcs=false

Root cause: workers have no `.git`, so `go build` VCS stamping failed (exit 128). Stamping also made outputs checkout-dependent (cache-hostile).
Fix (the only two places composing go build/test): `goBuildArgumentsV1` (go-build owner) emits `-buildvcs=false`, and rejects caller `-buildvcs`; `runCanonicalGoTests` passes `-buildvcs=false`.
Tests: go-build-mode suite passed (2/2, fixture `common` asserts the flag, oracle `go run` uses it); typecheck passed (13 files).
Unverified: go-plan `lease` test fails on the events list (missing build/recheck), in code this change does not touch (likely pre-existing); m12/m14/m15 ready.json after the next workspace re-run.
