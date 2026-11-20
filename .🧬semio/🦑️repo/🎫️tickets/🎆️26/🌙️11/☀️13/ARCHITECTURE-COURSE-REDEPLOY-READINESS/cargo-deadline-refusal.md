# Cargo test ends with exit 1 after every test passed

## Symptom

`repo cargo --store=shared --configuration=prototype -- test --manifest-path 🎓️teaching/Cargo.toml --workspace` printed `test result: ok` for every binary and doc-test, then `Internal error: Original Script process deadline refused` and exit 1 (about 1340 s on a heavily loaded host).

## Reproduction

The refusal text is only thrown by `readScriptProcessEnvelope` (`🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`). A temporary `[DEBUG]` stack in that message and a rerun of the exact command on the loaded host (load 60 to 115) reproduced it after about 1600 s:

```
Internal error: Original Script process deadline refused remaining=-318 max=2000 owner=repository:build
  at readScriptProcessEnvelope
  at scriptProcessEnvironment
  at scriptResourceProcessEnvironmentV1
  at captureOwnedCommandLiveV1
```

So it is none of the three suspected candidates (not the 4 h / 1 h `native-child` progress policy, not a parent deadline carried into a smaller policy, not publication or cleanup). It is a child envelope with a 2000 ms ceiling.

## Root cause

1. The 2000 ms envelopes belong to the native telemetry sampler (`📦️artifacts/🏗️native-build/📈️telemetry/⏱️sampling`, `maximumSampleMilliseconds: 2000`): every sample runs under a 2 s requested control and spawns a child (memory counters). A run makes hundreds of them (164 in a short run).
2. `scriptResourceProcessEnvironmentV1` composed the child envelope against one clock reading (`now`, deadline `now + min(left, ceiling)`) and validated it, but its hand-over `scriptProcessEnvironment` read `Date.now()` a second time and validated again. On a starved host (swap, load above 100) the clock moved past the 2 s deadline between the two readings: `remaining = -318`. `scriptInvocationEnvironment` had the same double reading, and neither clamped the deadline to the policy maximum.
3. `readScriptProcessEnvelope` reported an expired deadline as a plain `Error`. The sampler is advisory and by design skips a sample whose loan expired (`ResourceLoanDeadlineExhaustedV1`, name `ResourceLoanDeadlineExhausted`, rendered by `refuseInternalV1` as the multilingual `execution.loan-expired` notice). A plain `Error` is not skipped: the sampler recorded it as its failure, `sampler.stop()` rethrew it during the retirement of the compiler store (after the tests had finished) and the CLI printed `Internal error` with exit 1.

## Fix

All in `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`:

- `readScriptProcessEnvelope`: an expired deadline throws `ResourceLoanDeadlineExhaustedV1` (typed expiry, en and de notice, skipped by advisory holders). A deadline beyond the policy maximum stays the structural `Original Script process deadline refused`.
- `scriptProcessEnvironment` takes the validating clock reading (`now`, default `Date.now()`); `scriptResourceProcessEnvironmentV1` and `scriptInvocationEnvironment` read the clock once and hand over with that reading, so a child envelope is never judged by a second clock.
- `scriptInvocationEnvironment` clamps the projected deadline to the policy maximum, so credited remaining time can never produce an envelope above its own maximum.
- `Original child phase deadline exhausted` is now the same typed `ResourceLoanDeadlineExhaustedV1`.

Existing expectation updated: `🏗️build/🧪️tests/🕸️graph/📌️pinned-publication/🟦️.ts` (expired envelope now throws the typed expiry).

## Tests (schema-first, test-driven)

`🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/⏳️admission/fresh-envelope.{json,schema.json,ts}` (imported by the directory entry `🟦️.ts`), with a fake clock that stalls every `Date.now()` reading by 10 to 1000 ms:

- typed expiry versus structural refusal of `readScriptProcessEnvelope`;
- the en and de notice of a typed expiry (never `Internal error` / `Interner Fehler`);
- stalled projection of a 2 s control ends as a projection or a typed expiry;
- a telemetry sample whose projection stalls is skipped and the observed work stays successful.

Red before the fix (3 failures with `Original Script process deadline refused`), green after.

## Real run with the fix

The exact command, rerun on the same loaded host with `SEMIO_PROGRESS_DEADLINE_TRACE=1`, ran 3192 s (well past the 1346 s where it used to fail) and exited 0 with every test result ok.

Unit suites: the new `fresh-envelope.ts` and the whole `⏳️admission` directory pass. Three tests of `🧭️routing/🧪️tests/🟦️.ts` and `⏳️admission/freshness.ts` ("owned child environment carries the caller's current original invocation" off by one ms, "fractional finite child ceilings" listener count, "fresh cutoff evaluation" read count) fail identically on the unchanged HEAD version of the file, so they are not caused by this fix. `pinned-publication` needs the `repo test` runner (`SEMIO_TEST_ARTIFACT_DIR`) and cannot run under plain `bun test`.

## The three remaining failures (product or test)

Run with `bun test` of `🧭️routing/🧪️tests/🟦️.ts` and `⏳️admission/🟦️.ts` (separately: an in-process SIGINT arms a 10 s exit guard that would cut a long combined run); both are now green (48 and 35 pass).

1. **"owned child environment carries the caller's current original invocation": product.** `scriptInvocationEnvironment` is documented as handing over the caller's current original invocation with its remaining deadline, and `scriptProcessEnvironment` as carrying the same admitted deadline. It projected a fresh `now + floor(remaining)` and lost the parent's exact epoch by up to 1 ms (the remaining is a float from a monotonic term). It now carries the parent's own epoch plus the whole milliseconds the process waited for admission since its birth (the birth records `waitedAtStart`), so an uncredited child carries the identical epoch and a credited one the credited epoch (clamped to the policy maximum); only an invocation alive past its epoch with no credit falls back to a fresh projection.
2. **"fractional finite child ceilings" (SIGINT listener missing): test.** The listener was not removed by the product: the runner process holds a `once("SIGINT")` scratch-cleanup listener (`🧹️scratch/🟦️.ts`), and the test's global `process.emit("SIGINT")` fired and consumed it (and would have signalled the process). The tests now call only the listeners the envelope under test installed (`emitOwnInterrupt` in `🧪️tests/🟦️.ts`, also used by "derived signal stop", which had the same global emit).
3. **"fresh cutoff evaluation preserves an open authenticated admission wait": product.** `remaining()` sampled the admission-wait ledger before its final clock readings, so a wait that opened during the last reading (the fixture stalls read 4 for 90 ms inside an open wait) was not credited and an alive loan reported exhausted. The ledger is now sampled again right after the final clock readings and the late credit is added to the bound, as the fixture's `expectedWaitAuthorized: true` and the docstring ("a deadline starts when the work is admitted") require.

Other files under `🧭️routing` need the `repo test` environment (`SEMIO_TEST_ARTIFACT_DIR`); they were run with that variable set to a scratch directory.

Regression check: the whole `🧭️routing` tree was run file by file with `SEMIO_TEST_ARTIFACT_DIR` set, against the unchanged HEAD implementation and against the fix. Every remaining failure (22 `🪢️lifetime` laws, `🎛️command` boundary/cold, `🏠️workspace`, `📤️settlement`, `⏲️expiry/⏳️progress`, `📊️profile`, `📥️drain`) fails identically on HEAD and is unrelated; `🧪️tests/🟦️.ts` (48), `⏳️admission` (35), `🪆️projected-parent`, `⏲️expiry/⏳️following` and `📤️fractional` are green.

## Triage of the General Nx / router-bundle failures

`🎛️command/🧪️tests/🚧️boundary`, `🏠️workspace/🧪️tests` and `🎛️command/🧪️tests/❄️cold` sit under `🧭️routing` but are not routing logic and fail identically on the unchanged HEAD implementation. They are layering and environment checks, so they were left alone:
- "all General Nx executable entries and router imports remain inside their defining owner" and "every actual General router and public export independently bundles with Specific sources refused": `🔨️modules` tests (churn, state, cargo exact, leases and others) and wasm package outputs import Specific `🛍️products/🦑️repo` sources or files, a repository content violation for whoever owns those tests.
- "native Rust, Python, Go and C# hosts resolve the same neutral workspace vectors": the `go` toolchain exits 1 on this host.
- "an actual configured General repo:script command starts with all Specific sources absent": the same layering violation seen through the cold copy.
