# Activation Freshness

## Investigation

The activation execution warm shortcut reads its own previous source marker and receipt without observing the current component owner tree or current staged module bytes. `readStagedSourceContentHash` reads only `.source-content-sha256`; normal activation writes that runtime marker only in the completion loop the shortcut skips. The lazy Vite activation publisher is the other runtime marker writer, after its explicit materialization.

The authored registry host-activation contract declares activation depending on preparation. Preparation validates module descriptors, bridge and artifact-marker presence; it does not update the runtime source marker. Even assuming preparation has compiled current bytes, the warm comparison therefore cannot establish current staged artifact identity: fresh module bytes can coexist with the previous unchanged runtime marker.

The existing completion path already streams every module/support byte with cancellation, hashes the bounded current owner source tree asynchronously, and delegates unchanged receipt/timestamp preservation to `nextActivationReceipt` and `publishActivationReceipt`. The narrow correction is to remove the shortcut and retain that path for every activation. This does not assert that activation compiles sources itself: freshness evidence still requires the owning prepare/build producer to have completed for the current source.

## Sources

- OS dev `♻️activation/🏃️execution/🟦️.ts`: warm branch and completion loop.
- OS dev `♻️activation/📥️installation/🟦️.ts`: cancellation-aware streamed artifact digest.
- Plugin `🏗️build/🔍️freshness/🟦️.ts`: marker reader, bounded source content/stat index hashing.
- OS dev `♻️activation/🧰️preparation/🟦️.ts`: prepared-set checks.
- Registry `🧪️tests/🎬️host-activation/🟦️.ts`: authored producer dependency assertion.
- OS dev activation Vite runtime: lazy producer publishes marker after build closure.

## Validation

Pending focused regression and source correction. No activation receipt, marker, guest output, cache state, stale policy or running compile has been manually changed.

## Changes

Removed the marker-only warm shortcut. Extracted the existing streamed digest and actual current-artifact completion loop into private activation `🔖️identity/🟦️.ts`; execution always calls it before receipt reuse. Installation retains only extension publication. Added a neutral activation scenario to the existing staging-root fixture and a behavioral regression using that same private production helper: stale unchanged marker plus changed module bytes, changed support, changed source, unchanged timestamp/receipt preservation, receipt publication/readback, and cancellation. The independent artifact-hash oracle is the already installed `crypto-browserify` bundled by Next, behind the test-only require boundary. No package exports, dependencies, receipt schema, freshness policy, artifact outputs or cache receipts changed.

The test was authored before the production edit, but an actual failing test run was deferred to coordinate machine admission with the native gate. Focused runner validation remains pending.

## Completed Focused Validation

`repo tools exec bun -- test ./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔌️staging-root/🟦️.ts` completed successfully: 57 passed, 0 failed, 650 assertions. Repeated once to retain the complete command output at ticket `🗑️generated/activation-freshness-test.log` (1.044 s). Existing source-tree tests also verified bounded 20,000-entry walks, 32-file / 4-component concurrency limits, asynchronous yield and cancellation against WebCrypto and picomatch oracles. The new production completion helper uses those existing selected owner-tree inputs; it never walks the repository.

`sourceContentSha256` remains the owner-source aggregate consumed by freshness reporting. `artifactSha256` remains SHA-256 of support digest concatenated with the existing framed module-files digest, consumed by receipts, extension packageHash and reload observation. It is not a raw Wasm digest; end-to-end acceptance must retain the producer current-source identity and canonical staged Wasm/materialization evidence separately. The private helper exposes only repository-owned `ActivationArtifact` and system-library types.

Updated the existing live-activation contract test definition extractor to read the moved streamed digest and new collector from the private identity owner. Direct Bun invocation of that broader fixture collected zero tests; no integration failure or pass has been established. No extra Cargo, dependency sync, fresh serve or global graph run was started.

## Live Integration Boundary

Ran `repo tools exec bun -- test ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️live-activation/🟦️.ts`: exit 0, 0 pass, 0 fail, 0 tests. Full output: `🗑️generated/activation-live-test.log`. It exports a function requiring an issued ScriptInvocation and ResourceLeaseControl, rather than registering a test. Its only caller is the broad caching command-inputs contract; no dedicated issued route exists. Adding one requires the caching codeSuite table, project target and command router. The fixture itself extracts production classes into a synthetic script whose BundleScript contains only `this.root`; actual activation requires issued `this.invocation`, `this.repoRoot`, catalog/font readers and destination controls. These are concrete inspection findings, not an established failing runtime test. No environment aliases or synthetic authority were introduced to run it. Production extension installation integration therefore remains unverified by this focused run.

## Exact Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🔖️identity/🟦️.ts` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/📥️installation/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🔌️staging-root/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/🔌️staging-root.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️live-activation/🟦️.ts`
- This ticket audit Markdown.

## Narrow Fixture Reconciliation

The live fixture now imports the actual private identity and installation modules instead of extracting their definitions without their dependency closure. Only the two preexisting command classes remain extracted. The cancellation call now receives the actual process invocation using the first-party `receiveScriptProcessInvocation` boundary and passes the current workspace/invocation/target/source/installRoot/hash/timestamp/signal signature. No invocation object, authority field or envelope is synthesized. This repairs the moved-helper closure and outdated cancellation signature; it does not supply the missing genuine private child birth or make the synthetic command classes integration-ready. The zero-test file run remains collection evidence only.

## Component Producer Evidence Pointers

Plugin `🖨️describe/🏷️component-receipt/🟦️.ts` owns componentDeliverableV1, readComponentReceiptV1, componentReceiptFindingsV1 and componentProvenanceDigestsV1. A component receipt binds pluginId/packageId/profile, wasmSha256/wasmBytes, semanticDescriptorSha256, sourceTreeDigest and provenanceSha256. The source digest derives from sorted repository-relative path/hash Cargo input rows; provenance is the RFC6962 Merkle root of observed units and captured build-script resources.

Browser `🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts` publishComponentModuleV1 verifies the probed module semantic descriptor against the receipt, copies COMPONENT_RECEIPT_FILE beside descriptor and bridge, and stages under componentModuleOwnerV1(target.cratePath). Its transpile stage binds debug-stripped component Wasm bytes and jco/materialization identities. Current-source acceptance therefore needs actual Cargo-input provenance plus canonical deliverable bytes/receipt and this copied staged module receipt, followed by activation/install identities. Activation source markers alone cannot prove that chain.
