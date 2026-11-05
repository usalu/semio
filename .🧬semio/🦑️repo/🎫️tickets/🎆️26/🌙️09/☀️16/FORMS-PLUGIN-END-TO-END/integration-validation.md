# Forms Integration Validation

## Current Outcome

- UI contract mirror: actual Rust exporter and canonical freshness comparison passed (one test each). Generated sheet/touch/unnamedControl rows are present.
- Canonical routing prerequisite: seven independent-oracle tests passed; bounded capability diagnostics: two tests passed.
- Schema index: moved production paths and try-partial-change are installed. New try-value-target is absent; actual generator session76837 is live, with freshness check required afterward.
- Semantic descriptors: no final emission yet. Fresh declared owner19405 is live in graph closure verification after a corrected canonical generated-output export. Earlier emitter completed/admitted source-digest mismatch remains undiagnosed at actual compile; bounded diagnostics are installed and their SQLite/Ajv suite passed17tests/117assertions. No speculative source/cache/receipt correction was applied.
- Specific catalog generation and catalog-check have not run after final descriptor changes.
- No complete Forms native/browser integration pass is claimed by this report. See the root completion audit for other lanes.

Historical command attempts and their evidence follow. Current blocker details are in [pack-churn-audit.md](pack-churn-audit.md). All active owned handles remain monitored; no source/receipt/capacity bypass was performed.

## Scope

Regenerate semantic Forms and procedural-provider descriptor pairs from compiled source, regenerate the specific installed catalog, and run its declared catalog check. No descriptor or generated catalog rows are patched by hand. Root owns Forms native checks; browser and retained-provider lanes own their execution checks.

## Commands and Evidence

- First descriptor attempt: `repo run workspace:components -- describe forms playbook-module-procedural --profile prototype`. Full output retained in `🗑️generated/integration-describe.log`. Capacity wait lasted approximately 660 seconds before repository source admission and graph verification. At 241 of 1583 graph input closures, the command was cooperatively cancelled (exit 130) because final concurrent source edits were still pending. The log explicitly records `Cancelled (SIGINT); leases released.` No compiler result or regenerated descriptor is claimed from this attempt.
- The first bounded read accidentally requested the default Python capacity reservation; it was cooperatively cancelled before execution. Subsequent bounded reads use the supported `repo tools exec --light bun` query path.

## Registration Audit Before Regeneration

The procedural-provider Rust source declares the question kinds `buildingComponent`, `flow`, and `procedural3d`, with body keys `params`, `preview`, `flowParams`, `flowPreview`, `proceduralParams`, and `proceduralPreview`. The existing committed descriptor still contained only the building-component contribution and the original two body keys. This establishes why generation is required; it is not a final validation result.

The installed-catalog integration fixture and test already require all three kinds, each referenced provider app and declared body key, contributor discovery on the Forms topic, and dependency-before-provider activation order.

## Current Gate

All three source owners confirmed stable changes. The final descriptor attempt is running through `repo run workspace:components --parallel=1 -- describe forms playbook-module-procedural --profile prototype`, with complete output in `🗑️generated/integration-describe-final.log`. It is currently waiting under the repository capacity gate. Shared dependency sync has completed. No cache receipts, images, mounts, partitions, or capacity reservations were bypassed or manually changed.

## Changed Files in This Lane

- This report.

Final descriptor generation, catalog generation, and catalog check remain pending.

## Direct Cargo Validation Route

The supported `repo cargo --store=shared --configuration=prototype --parallelism=1 -- test --manifest-path <Forms-package-Cargo.toml> -p semio-s-artifact-forms-forms-config-tests --lib forms_try_window_` route avoids the Nx task graph: `build/command/runtime` branches on `command === cargo`, calls `launchRepositoryCargoV1`, and returns before dependency/build-graph dispatch. The launcher retains compiler-store custody, Cargo lock and source-boundary admission, tool registry invocation, capacity/jobserver controls, and storage reservation. It therefore removes Nx whole-workspace closure verification while preserving owned native compilation controls. An explicit config-tests manifest path is needed because Forms is in its own workspace; the root workspace does not supply the package automatically. Root subsequently confirmed that the concrete window tests belong to the split `semio-s-artifact-forms-forms-config-tests` package; the facade package would run zero matching window tests. The supported launcher conclusion remains the same. This route was inspected only; no duplicate native command was launched by this lane.

## Direct Descriptor Script Probe

After all source owners confirmed stable, the second task-based attempt remained capacity queued and was cooperatively cancelled (exit 130). The manifest-backed root Script command `repo ./📜️script.ts components describe forms playbook-module-procedural --profile prototype` was then attempted once. It failed before compilation (exit 1): `Original native caller capabilities required`, raised by the capability validator through `repositoryCargoNetworkV1` in `ComponentsScript`. The full refusal is retained in `🗑️generated/integration-describe-owned.log`. No capability environment was fabricated and no private function was called. The declared task-based owner has been restarted, retaining output in `🗑️generated/integration-describe-admitted.log`.

## Initial-Value Schema Index Ownership

The global mutation schema-document index scans every production JSON `$id` below schema directories under plugin/module roots, independent of whether it is referenced by a runtime derive. The Forms `try-initial-value` schema therefore belongs in that projection. The owner is `@semio-tech/dsl-derive-rs:generate-schema-index`; its `GenerateSchemaIndexScript` projects and writes only changed bytes, using the existing Ajv oracle. The plain package Script path refuses without an Original parent envelope, so the declared owner target has been started instead. No index row was manually edited. Full owner output is retained in `🗑️generated/integration-schema-index-owner.log`; generation and freshness verification remain pending.

## Current Build-Graph Refusal

The fresh descriptor task reached dependency verification and graph planning, then exited 1 before compilation. Its causal error is `build.test.aggregate-refused` with `{owner: repo, reason: route-targets:private-native-birth:0}`, raised by the canonical cache-test aggregate resolver. The policy selects the caching owner manifest; its new `private-native-birth` canonical route has no matching declared target there. A similarly named target exists only in the repo-lib package manifest. The refusal and full causal stack are in `🗑️generated/integration-describe-retry.log`; no wrapper-only diagnosis is used. Root was notified of the exact owner mismatch. This lane has not edited unrelated build infrastructure.

An earlier fresh task failed at bootstrap with `Build native caller capabilities refused`. A bounded read-only validation using the current canonical environment, native execution policy and caller schema returned no errors; no capability payload was injected or changed.

## Canonical Private-Native Routing Correction

Inspection found the actual private-native protocol fixes its witness target to `@semio-tech/repo-lib:test-private-native-birth`. The actual owner checks that selected target, the Native orchestration parent, finite original control, a real child process, unchanged limits, independent Ajv receipt validation, and retirement. The repo-lib target already declared its exact owner and child runtime entries, but its router had no matching branch; the branch and canonical code-suite attribution were incorrectly placed in the caching owner.

Moved that function dispatch to the actual repo-lib runtime/path test router under the existing finite `withRepositoryTestArtifactsV1` wrapper. Removed the misplaced caching branch and `codeSuites` entry. The schema-fixed target and all Native authority, real-process, retirement and oracle checks are unchanged. No canonical aggregate was weakened or hand-rewritten.

The actual `requireCanonicalTestAggregateV1` now succeeds; its derived 193 dependencies exactly equal the committed aggregate, with zero missing or extra dependencies. The registered repo-lib route rejects malformed arguments before native authority access. This bounded verification passed through repo-owned Bun; output is retained in `🗑️generated/integration-private-routing.log`. Existing language-agnostic route fixtures, independent SQLite/Ajv aggregate tests, and the actual declared private Native witness target have also been launched. Their results remain pending under repository admission.

Changed source files in this lane:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️typescript-package-commands/🛤️runtime-and-path-routes/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🔬️script-commands/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🗂️suites/🔣️.json`

The descriptor target was retried after this concrete prerequisite correction; complete output is in `🗑️generated/integration-describe-routing.log`.

## Native Capability Diagnostics

Added permanent bounded refusal details to the actual native build caller constructor: at most eight validator messages of at most 256 characters, with no serialized capability object or raw caller values. The existing schema refusal remains enforced for execution-policy and caller-schema failures. A concurrent temporary debug line had already added unbounded messages; this change retains that diagnostic intent as a bounded permanent error.

Added language-agnostic diagnostic cases for invalid build mode and tool, fixture-schema validation through Ajv, and negative constructor tests requiring the schema path, the existing refusal, no sensitive caller value, and a bounded message. The prechange observation did not include the failing schema path; the current observation identifies `/buildMode` without its value. Actual admitted Bun test output is pending in `🗑️generated/integration-capability-tests.log`.

Additional changed files:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🧪️tests/🧷️capabilities/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🧪️tests/🧷️capabilities/🧬️schema.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🧪️tests/🧷️capabilities/🟦️.ts`

## Passing Aggregate Verification

The repository-owned Bun run of the canonical aggregate tests passed: **7 tests, 0 failures, 24 assertions, 310 ms**, exit 0. It validates the shared route fixture with independent Ajv and SQLite, exact route identity/refusal, actual planner cycles, and current aggregate freshness. Output: `🗑️generated/integration-aggregate-retry.log`. The first launch was refused before any test because its file operand omitted the explicit `./` prefix; the corrected invocation uses the exact file path.

## Passing Capability and Schema-Index Evidence

The actual repository-owned capability suite passed: **2 tests, 0 failures, 117 assertions, 147 ms**, exit 0. Existing JSON-patch capability vectors agree with Ajv, and the new constructor diagnostics identify paths while omitting caller values. Output: `🗑️generated/integration-capability-tests.log`.

The actual declared `@semio-tech/dsl-derive-rs:generate-schema-index` target completed successfully (exit 0, `schema-document-index: written`, Nx success). Read-only inspection confirms the generated index contains both `https://semio.tech/s/forms/try-initial-value` and `https://semio.tech/ui/text-chunks` mapped to their actual production schemas. No index row was hand-edited. The declared freshness-check owner is now running; output: `🗑️generated/integration-schema-index-check.log`.

Generated source changed by this owner: `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json`.

## Private Witness Priority

At root coordination request, cooperatively cancelled only this lane's actual private-native witness invocation to prioritize Forms descriptor/native/browser execution. It had completed 1464 closure verification, refreshed and completed 1275 verification, and reached a whole-workspace inference refresh; no real witness child completion was observed. The invocation exited 130. Its full log is retained in `🗑️generated/integration-private-native.log`. Actual witness execution remains pending after Forms checks. The 193-route aggregate, existing independent-oracle suite, and malformed repo-lib dispatch checks have already passed. Descriptor and schema-index freshness invocations remain live.

## UI Contract Mirror Prerequisite

Root found browser TypeScript blockers from a stale generated UI contract mirror missing already-authored schema variants (`sheet`, `touch`, `unnamedControl`). Started the declared `@semio-tech/ui-contract-rs:generate` owner with `--parallel=1`, retaining full output in `🗑️generated/integration-ui-contract-generate.log`. Its GenerateScript runs the actual Rust `semio-framework-ui-contract-typegen` `typegen_export` test into the manifest mirror. The exact freshness owner is `@semio-tech/ui-contract-rs:check`, whose exporter byte-compares against the committed mirror; it will run after generation settles. No generated TypeScript enum or mirror row is hand-patched.

## Passing Schema-Index Freshness

The declared `@semio-tech/dsl-derive-rs:check-schema-index` owner completed successfully (exit 0), reporting `schema-document-index: fresh` and Nx target success. This checks the final production-schema `$id` mapping, including both new Forms initial-value and UI text-chunks schemas; later edits to existing schema contents do not alter the index projection. Full evidence: `🗑️generated/integration-schema-index-check.log`.

## Index Freshness Superseded by Owner Moves

After the passing index generation/check, root moved common Forms extensions and configuration into their shared `any` owners. Production schema paths changed while IDs stayed the same. The prior passing check is historical evidence and no longer proves current freshness. A new canonical generation plus freshness check will run once the path-audit owner confirms moves are settled. Root also began a readonly artifact Viewer contribution mount; descriptors must reflect the actual final compiled exports. No descriptor pair or index path is hand-edited.

## UI Exporter Direct Native Route

The declared UI generator was admitted but remained in a peer build-graph wait without running its exporter. At root coordination request, cooperatively cancelled only that invocation (exit 130) and started the identical authored exporter through the supported native `repo cargo` ingress: shared store, prototype configuration, one worker, UI contract manifest, package `semio-framework-ui-contract-typegen`, test `typegen_export`. The ordinary authored `SEMIO_TYPEGEN_OUT` output setting points to the unchanged GenerateScript destination, `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts`. No proof, capability, private store, or compiler-directory environment was set. The owning Cargo ingress retains source, storage, capacity, tool and Original controls. Full output: `🗑️generated/integration-ui-contract-native.log`. Exact freshness comparison will run sequentially after actual emission. Primary descriptor invocation remains live.

## Passing UI Contract Export

The supported direct native exporter completed **exit 0**: prototype compilation **5.98 s**, actual `exports_typescript_bindings` test **1 passed, 0 failed**. The generated mirror now contains `sheet`, `touch`, and `unnamedControl`; its write timestamp is 2026-11-04T22:59:54.071Z. No TypeScript mirror was edited by hand. Confirmed the parent environment does not retain `SEMIO_TYPEGEN_OUT`, then started the identical native test without that output setting to exercise its canonical byte-comparison freshness branch. Full emission evidence: `🗑️generated/integration-ui-contract-native.log`; comparison evidence will be `🗑️generated/integration-ui-contract-native-check.log`. Browser owner was notified to run its renderer TypeScript check.

Generated source changed by this owner: `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts`.

## Passing UI Contract Freshness and Final Path Gate

The identical supported direct Cargo `typegen_export` test **without** `SEMIO_TYPEGEN_OUT` completed **exit 0**, prototype compile **0.80 s**, **1 passed, 0 failed**. This exercises the canonical exporter comparison against the generated mirror, so UI contract emission and freshness are now both verified. Full evidence: `🗑️generated/integration-ui-contract-native-check.log`.

Root confirmed readonly Viewer and shared Forms path cleanup are settled, including removal of the obsolete empty Viewer config schemas. Started final canonical schema-index generation after these moves; output: `🗑️generated/integration-schema-index-final.log`. A fresh check will follow sequentially. No new Viewer command schema ID was introduced; it reuses shared configuration.

## Final Schema-Index Generation After Moves

The final declared generation completed **exit 0** with `schema-document-index: unchanged` and Nx success; the actual generator ran. A concurrent owned generation had already reconciled moved paths before this owner executed. Read-only audit of the Forms root finds **36 schema mappings, zero missing paths, shared config mappings under `any/🎚️config`, and zero obsolete Viewer config mappings**. The final declared freshness check is now running sequentially; output: `🗑️generated/integration-schema-index-final-check.log`.

## Partial-Change Schema Addition

Root added production schema `https://semio.tech/s/forms/try-partial-change` after final path generation. Read-only inspection confirmed that ID was not yet in the index. Cooperatively cancelled only the pending freshness-check invocation (exit 130, no stale check result claimed), then started canonical generation again with output in `🗑️generated/integration-schema-index-partial.log`. Verification of the new ID and a sequential actual freshness check remain required. Primary descriptor execution was kept live.

## Pack JSON Snapshot Refusal Audit

The emitter prerequisite exhausted owned input-snapshot retries and the repository automatically restarted the existing descriptor command while retaining its capacity grant. No Rust compiler error is claimed. The bounded read-only identity/stat/content observations and the precise completed-provenance comparison behind this refusal are recorded in [pack-churn-audit.md](pack-churn-audit.md). They show current file stability and explain why the generic failure cannot establish repeated writes or a metadata bug.

## Latest Index Attempt

The schema-index generation captured in integration-schema-index-partial.log exited 1 before the generator: graph project-nodes called undefined readBuildProducedCompileOutputV1. The browser owner is actively changing this canonical producer-resolution interface and was notified; no fallback or generated-file patch was applied. The new try-partial-change ID remains unverified until a fresh owner succeeds. Descriptor session 93488 and digest-diagnostic tests 38337 remain live.

## Digest Diagnostic Test And Descriptor Terminal

The actual artifact-plan suite exited 0: 17 pass, 0 fail, 117 assertions, 196 ms. The added neutral vectors distinguish exact matching bytes, changed bytes, missing observation, explicit null observation, a distinct lookup spelling, and Windows paths. SQLite exact lookup and Ajv independently validate the comparison and closed diagnostic DTO. Full log: integration-source-diagnostics-tests.log.

Descriptor session 93488 exited 1 during its automatic retry graph refresh, with the same undefined readBuildProducedCompileOutputV1 interface as the index. No descriptor or catalog emission is claimed. Browser owner corrected the export and is completing resolver changes; fresh descriptor/index invocation must follow that stable notice. Neither failed owner remains live.

## Stable Cold-Wasm Retry

Browser owner confirmed canonical resolver source stable. Fresh declared descriptor session 19405 is captured in integration-describe-cold-stable.log. Fresh exact schema-index check session 36797 is captured in integration-schema-index-cold-check.log; read-only inventory confirms all new IDs are installed, so check precedes any necessary regeneration. These are live invocations, not success claims.

## Concurrent Native Prerequisite Ownership

The final graph must include semio-s-artifact-forms-forms-viewer at Forms any/👁️viewer/📦️packages/🦀️rust, whose dedicated source owner avoids the prior config-tests source-file capacity overflow. The provider owner authored that crate and rewiring; this integration lane did not edit it.

The root owner’s native-prerequisite-files.md records the 15 stdio ArtifactEditor codec placement fixes and the explicit HistoryFoldIndexRead reexports in framework/modules/replication/packages/rust. The neural-cache parent import correction was a separate collaborator’s change. These are prerequisite evidence, not source changes attributed to this integration lane. Their actual fresh native compile remains required.

## Remaining Exact Native Routing Witness

The minimal declared witness is repo run @semio-tech/repo-lib:test-private-native-birth --parallel=1. Its manifest has dependsOn:[], orchestration execution, live/uncached policy, and the actual owner and child in runtimeSourceEntries. Its exact authored command is repo ./📜️script.ts test private-native-birth. The corrected repo-lib route executes testPrivateNativeBirthV1 inside the existing finite TestArtifacts owner under the Original invocation. No aggregate or caching-suite invocation is needed. The previous session46469 was cancelled before the real witness completed; the witness remains required after descriptor/index capacity allows.

## Resolver Repair Validation

Browser owner reports actual cold-Wasm helper suite exit 0, 3 tests and 64 assertions in 384 ms. It covers all five real owners/ten exact generated modules with generated bytes absent and independent Ajv, SQLite, and TypeScript oracles. Production resolver source remains stable; later browser acceptance edits are tests only. Fresh descriptor19405 and index36797 are still retained.

## New Target Schema Requires Generation

A bounded exact-ID lookup confirms https://semio.tech/s/forms/try-value-target is absent from the generated index while try-partial-change is present. The freshness owner36797 was still door-capacity queued and was cooperatively cancelled (terminal130). Actual generation76837 now owns reconciliation, captured in integration-schema-index-target.log; a fresh check must follow its success. Descriptor19405 remains live.

## Runtime Source Entry Prerequisite Fix

Corrected only the archive-ownership target runtimeSourceEntries literal in framework-renderer-react project manifest by adding required {workspaceRoot}/. Existing canonical schema and strict Ajv both rejected the actual old value and accept the corrected physical entry; the unprefixed value still refuses. See nested-admission-audit.md and runtime-entry-red/green.json logs. The earlier structural audit missed this explicit schema prefix and is corrected. No generic source/capacity guard changed. Live owners19405/76837 retained.

## Native Fixture Budget Prerequisite

See native-source-budget.md for the actual native byte refusal, complete included-fixture inventory, schema-first compact recipes, all consumer updates, and original byte/hash equivalence. Diagnostic editor closure now253884B with8260Bheadroom; actual fresh native admission remains root-owned. Normal Formsconsumer91085 and browser69935 are pending. Descriptor19405/index76837 retained.

## Existing Witness Reuse Criteria

The genuine private-native-birth owner consumes the child receipt internally and asserts independent Ajv/schema validity, distinct process/workspace, actual retirement, bounded transport/deadline/network, and unchanged Original envelope. It does not print that receipt itself. The declared target is live/uncached with no output files, so a queue disappearance or cache artifact alone is insufficient. Reuse requires authoritative successful output/terminal status for the exact current repo-lib:test-private-native-birth owner and source revision. No duplicate witness has been launched; no such actual successful owner evidence has yet been received.

## Descriptor Pass-Two Boundary

Descriptor 19405 completed its second 1,397/1,397 closure verification and entered graph inference refresh for 550 projects at generation 897. It reported 3,896 stale facts, including the new PatchQuestions, mounted TextEditor/archive, and cold-Wasm source owners. No third full pass or emitter compiler was observed at this boundary; the same owner remains live.

## Forms Consumers Passed

Session 91085 is terminal exit 0: 25 pass, 0 fail, 78 assertions, 1.038 s. Descriptor 19405 moved beyond graph refresh into Nx source-index and SDK integrity verification; generation 898 is published. No descriptor emission or final catalog result is claimed yet.

## Renderer Result Freshness

Browser run 69935 passed 13 post-recipe tests, but a subsequent full renderer check exposed TS2367 and required a fresh-read liveness helper correction. That earlier focused pass does not validate the latest helper source. Browser owns full renderer 19028 and focused 45721 for the correction; authoritative results remain pending here. This lane has not edited that production helper.

## Emitter Lock Refusal And Owned Automatic Retry

The live descriptor invocation reached Nx prerequisite execution. Four prerequisites passed; os-plugin-describe-rs:build failed after 14m17s because the root Cargo.lock bytes changed across four guarded snapshot attempts. Components did not run. The repository automatically restarted the same invocation with the current implementation (attempt 1 of 6), then began verification of 2,043 closures. No agent restart/cancellation, lock edit, receipt edit, or guard relaxation was performed. The latest refusal is Cargo.lock input freshness, not the earlier completed/admitted pack-source comparison. Full evidence remains in integration-describe-cold-stable.log.
