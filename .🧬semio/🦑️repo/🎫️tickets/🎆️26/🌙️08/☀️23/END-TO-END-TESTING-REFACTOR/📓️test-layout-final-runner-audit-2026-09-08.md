# Canonical Test Layout and Runner Audit

> **Superseded migration snapshot.** This report records the 21:58 CEST migration queue, including the then-current 30 findings and 480 non-feature directories. Those residuals were repaired. The final full scanner inspected 29,224 authored sources with zero findings. Read [the current final audit](./📓️test-layout-final-current-audit-2026-09-08.md) for the resolved source result and non-feature runner samples; a separate current runner-classification inventory will replace this pointer when its TypeScript audit is published.


## Scope and Evidence Time

This is a read-only audit of JavaScript, TypeScript, Python, Go, test fixtures, helpers, and discovery paths. It was taken on 2026-09-08 at 21:58 CEST while the TypeScript and Rust migration agents were still changing sources. It is a hand-off queue and runner-boundary record, not a green gate claim.

The audit first read `📓️test-layout-enforcement.md`, `📓️test-layout-framework-files-2026-09-08.md`, `📓️test-layout-source-evidence-integration-2026-09-08.md`, `📓️test-layout-source-evidence-execution-2026-09-08.md`, `📓️test-layout-native-followup-2026-09-08.md`, and `📓️test-layout-script-audit-2026-09-08.md`.

The permanent contract is implemented by `inspectTestLayoutSources` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`. Its path rule permits only a direct `<semantic owner>/🧪️tests/<emoji-kebab-case>/<kind-only implementation>` leaf and rejects owners below `📦️packages`, `⚡️implementations`, or `🎯️targets`.

The audit's read-only input script is `🧑‍💻final-layout-audit/📜️script.ts`. It used the same exported parser on all JS/TS/Python/Go case sources and canonical implementations plus no-ignore executable candidates. The completed invocation inspected 3,449 sources, including 606 canonical implementation leaves and 2,955 executable candidates. It returned 30 current findings: 28 inline self-test declarations, one delivery-scope violation, and one scan-visible untracked temporary source. No Python or Go finding was returned by this invocation.

`📓️test-layout-current-snapshot-2026-09-08.md` has a 20:29 CEST intermediate full-scan timestamp and explicitly identifies itself as a migration queue. Its previously reported Flow and Python paths were already moved before this audit; they are not repeated below.

## Current JavaScript and TypeScript Residuals

The following named declarations were reported as `inline-self-test-declaration`. They remain executable test bodies outside a canonical implementation at this audit time.

| Source | Declaration(s) |
| --- | --- |
| `✏️s/🔌️plugins/🗄️stdio/📜️script.ts` | `testStdioArtifactPackageGraph` (333); `testTypeScriptPackage` (401) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/📜️script.ts` | `testFlowBrowserDeclaration` (88) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🔎️scalar-witness/📜️script.ts` | `testScalarRecordWireFixture` (33) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️script.ts` | `testGroupVisibilityFixtures` (10) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧵️send/📜️script.ts` | `testNativeCodecSendFixture` (8) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/✂️detach/📜️script.ts` | `testBackboneDetachFixture` (8) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌱️initial/🪪️identity/📜️script.ts` | `testInitialChildIdentityFixture` (10) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/📜️script.ts` | `testDurableOwnedGroupDecisionFixture` (14) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🪪️member-dialect/📜️script.ts` | `testMemberDialectFixture` (9) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/📜️script.ts` | `testCanonicalEditFixtures` (12) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🪪️runtime/📜️script.ts` | `testDirectoryRuntimeIdentityFixture` (8) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts` | `testClosedBrowserComponentFactory` (653); `testBrowserCodegenCapsule` (744); `testBrowserCodegenSources` (784); `testBrowserCodegenPolicy` (832); `testBrowserActorCodegenManifest` (882); `testClosedBrowserActorBundle` (897); `testBrowserHostActivation` (1107) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌊️actor-import/📜️script.ts` | `testCanonicalActorAsyncImport` (35) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌐️wasi-activation/📜️script.ts` | `testBrowserWasiActivation` (9) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts` | `pluginTestRunnerSelfTests` (21) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🔣️codec/🧵️send/📜️script.ts` | `testPluginCodecCallerSource` (277) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts` | `testFreshComponentSourceEpochV1` (847); `testFreshComponentStagingV1` (947); `testFreshComponentProcessV1` (1160) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📜️script.ts` | `testFixtureSweepExtraction` (58) |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts` | `testCacheContracts` (338) |

`🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/🧪️tests/🖼️surface/🟨️.js` is a full top-level assertion program. Its first assertion helper is at line 16. Its case is structurally direct, but its owner is below `🎯️targets`, producing `test-owner-delivery-scope`. A no-ignore reference search found no script, package manifest, project manifest, or Vitest configuration that names this path or its unique `a2Composition`/`surfaceTrace` output. It is therefore both wrongly scoped and unregistered by the runners searched.

`temp/merge/mit-bestand/präsentation/33.projektetage/js/index.ts` is scan-visible and untracked (`git ls-files --error-unmatch` returned 1). At line 615 it has `import.meta.vitest` wiring which the parser reported as lacking a canonical import. It names `../🧪️tests/📽️deck/🟦️.ts` at line 622, but the layout parser still rejects this source wiring. The temporary source is outside this ticket; no cleanup was performed in this read-only audit.

The renderer `PluginRuntime` inline suite that was present earlier is absent from the 21:58 parser results. Its prior presence must not be used as a current finding.

## Python, Go, and Fixture Helper Check

No Python or Go path was returned by the no-ignore parser invocation. The previously reported native stage helper paths no longer exist as separate fixtures. The live parity bodies are canonical:

- `♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/data/🧪️tests/🔬️stage-one-parity/🐍️.py` exposes `run(check_fn)`.
- `♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/model/🧪️tests/🔬️stage-two-parity/🐍️.py` exposes `run(check_fn)`.
- `♻️mit-bestand/🔎️recherche/_neo4j/netz/netz/🧪️tests/🪜️stage-regression/🐍️.py` loads those two canonical files with `run_stage_case` and executes their `run(check)` methods.

The retained stage-regression fixture producer `🧫️fixtures/dump_snapshot.py` writes its golden snapshot only when explicitly invoked; this audit did not run it.

## Runner Discovery and Masking

The test-domain discovery code (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`, `discoverTestCases` at line 548) creates a domain case only when `🥒️.feature` exists. The Nx plugin repeats the same boundary with `createNodesV2: ["**/*.feature", testCaseProjects]` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs` line 178.

At audit time there were 602 canonical JavaScript/TypeScript/Python/Go case directories: 122 held `🥒️.feature`; 480 did not. The latter 480 are not generated as Nx test-case projects by this feature-based mechanism. This is an execution-coverage boundary, not evidence that each lacks a package-native runner. Each must retain an explicit package/script/host execution path; the webgpu surface case above is the concrete one for which no such reference was found.

The root `vitest.config.ts` deliberately has `include: []` and no `projects` key, documenting that it is not a repository test aggregator. This avoids a root double-collection mask. Package-level configs can run wrappers through `includeSource`; the framework OS config deliberately sets `include: []`, and the renderer React config separates canonical `include` suites from in-source `includeSource` suites by test level. The latter previously included `PluginRuntime`; the current source parser no longer sees an inline body there.

## Required Follow-up

Move each listed self-test body to a semantic-owner canonical case and replace the caller with explicit dispatch to that canonical implementation. Move the webgpu surface case above the `🎯️targets` delivery directory and add its explicit runner path. Resolve or relocate the scan-visible `temp/merge` source so it does not participate in the repository layout contract. Re-run the full `scanTestLayout`/test-contract gate after the active TypeScript and Rust migrations settle; this audit does not substitute for that final gate.
