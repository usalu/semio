# Renderer Source Consistency

The current WGPU wasm owner `🏗️compiler/🌐️wasm/📜️script.ts` calls `buildCargoArtifacts` directly inside `withRepositoryCompilerLaneV1`; it does not call `publishRustArtifactFromPackage`. Cargo receives the live workspace manifest and working directory. The compiler lane is an artifact/resource lease, not a source lock or sealed workspace. The owner then validates the Cargo observation, invokes the matching wasm-bindgen, and publishes the resulting real files through the repository destination and storage gates.

`native-build/🟦️.ts` lines 339–375 already retries typed concurrent edits, re-planning and re-admitting the graph and source on each attempt. It checks successful captured observations against admitted source digests before the consumer runs. However an ordinary failed Cargo invocation is thrown as a generic error at line 1144. Thus mixed revisions that produce a Rust compiler error do not reach the concurrent-edit retry. The ticket wrapper retries only exit status 75 and does not make source coherent.

The existing optional `retainCompilationBytes` enables a source drift watcher during Cargo (lines 1074–1079), causing detected drift to become a typed concurrent edit and use the existing retry. It must be a finite positive value within the admitted storage allocation and policy maximum (line 324). This is an owner code option, not a discovered environment switch; the current renderer owner omits it. It retains completed compilation evidence too. The watcher is capped and event/metadata based, so it is not a sealed snapshot and does not guarantee detection before watcher startup or diagnose every mixed cached dependency failure.

The component driver has stronger failure-specific machinery: `🧩️component-pass/🔁️failure-churn/🟦️.ts` lines 153–175 runs bounded attempts after a new dependency-byte witness, while `🧩️component-pass/🟦️.ts` lines 896–933 diagnoses failed compiler units, re-admits source and a replacement unit graph, checks that it fits the held lane, then recompiles before publication. It is implemented in that driver, not exposed as a renderer build configuration. Reusing it robustly requires wiring equivalent failure evidence and graph admission into the artifact build path, or a supported sealed-source facility; no existing config was found that provides either for this owner. Preserve the existing wasm-bindgen transformations and destination publication after a verified successful Cargo capture. No build was run.

## Cargo failure recheck

Cargo's artifact runner now returns an observed nonzero compiler exit through its writer and capacity callbacks. It rechecks the admitted plan while the target recipe lease is still held. A typed concurrent edit passes through the tracked production callback as data, preserving the issued lifetime until the existing bounded retry receives it. An unchanged source plan keeps the original compiler failure. Cancellation, a failed launch, a signalled child, and output validation failures retain their existing paths.

The new language-neutral case reproduced the defect before the change. The first compiler fixture process changed `main.rs` and exited 19. The runner reported a compiler failure after one compilation and did not recheck. With the fix, it admitted the changed source, compiled twice, and published bytes verified by the independent WebCrypto digest. The unchanged-source nonzero case ran once and rechecked exactly once after the failure. Both cancellation after child launch and successful exit with absent deliverables kept the original three graph captures, with no failure recheck. All five capture vectors passed.

The Cargo plan, controlled plan and component failure fixtures passed 46 additional checks. These include independent Ajv, SQLite, Lodash and BLAKE3 comparisons against neutral data. The verification used the repository runner and fixture processes and did not start a real Cargo compile or publisher.

### Changed paths

- [Native artifact runner](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts)
- [Native artifact tests](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧪️tests/🟦️.ts)
- [Language-neutral Cargo fixture](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧪️tests/🧫️fixtures/🔣️.json)
- [Focused verification preload](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/LAYOUT-FEATURE-COMPLETE/cargo-failure-test-invocation.ts)

### Verification commands

```sh
ticket='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/LAYOUT-FEATURE-COMPLETE'
native='🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build'
repo tools exec --light python -- -c 'from pathlib import Path; import sys; Path(sys.argv[1]).mkdir(exist_ok=True, parents=True)' "$ticket/🗑️generated/n"
SEMIO_TEST_ARTIFACT_DIR="$PWD/$ticket/🗑️generated/n" repo tools exec --light bun -- test --preload "./$ticket/cargo-failure-test-invocation.ts" --timeout=120000 --test-name-pattern='captured native artifacts:' "./$native/🧪️tests/🟦️.ts"

SEMIO_TEST_ARTIFACT_DIR="$PWD/$ticket/🗑️generated/n" repo tools exec --light bun -- test --timeout=30000 "./$native/📋️plan/🧪️tests/🟦️.ts" "./$native/📋️plan/🧪️tests/🧵️control/🟦️.ts" "./$native/🧩️component-pass/🔁️failure-churn/🧪️tests/🟦️.ts"
```
