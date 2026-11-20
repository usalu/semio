# Note Relayed by the Architecture Course Session (2026-11-18)

Not measured or verified by g1-m02; copied from a peer session's message so the fleet coordinator can see it. g1-m02 is a worker, owns no code and changes none.

Area: `🧰️framework/🔨️modules/🏃️process/🧭️routing`.

## Said to be fixed and landed in the live tree (by that session)

- `📥️invocation/🏃️process/🟦️.ts`, `🧪️tests/🟦️.ts`, new `🧪️tests/⏳️admission/fresh-envelope.*`.
- Expired telemetry-sampler envelope now throws the typed `ResourceLoanDeadlineExhaustedV1` and reads the clock once (before: plain "Original Script process deadline refused", turning long `repo cargo … test` runs into exit 1 "Internal error").
- Child epoch carries the parent's epoch plus admission waits credited since birth; `remaining()` re-samples late admission credits.
- Details: `cargo-deadline-refusal.md` in ticket 2026/11/13/ARCHITECTURE-COURSE-REDEPLOY-READINESS.

## Said to be failing on HEAD and with the fix alike (each file alone, `SEMIO_TEST_ARTIFACT_DIR` set)

- 22 tests in `🪢️lifetime`; `🎛️command` boundary/cold; `🏠️workspace`; `📤️settlement`; `⏲️expiry/⏳️progress`; `📊️profile`; `📥️drain`.
- 3 General Nx entry/router-bundle tests ("all General Nx executable entries and router imports remain inside their defining owner", "every actual General router and public export independently bundles with Specific sources refused").
- That session is not touching these and asks for them to go to the owner of that tooling.
