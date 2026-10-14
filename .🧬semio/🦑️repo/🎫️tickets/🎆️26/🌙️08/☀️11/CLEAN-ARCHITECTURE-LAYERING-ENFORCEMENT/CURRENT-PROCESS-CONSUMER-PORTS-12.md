# Current Process Consumer Ports 12

Read-only exact literal inventory derived from the fresh SHA-bound AST census. Closest authored semantic-owner classification is used. Literal source edges are not an actual graph verdict. No implementation source bodies are included. Graph diagnostics and full owner gates must precede mutations.

| Actual Foreign Consumed Facet | References | Files | Named Contracts |
| --- | ---: | ---: | --- |
| `🧭️routing/🟦️.ts` | 561 | 561 | `BundleScript`, `Script`, `ScriptCommand`, `ScriptInvocation`, `ScriptRouter`, `checkScriptInvocation`, `findWorkspaceRoot`, `readScriptPolicy`, `scriptInvocationBudget` |
| `🧭️routing/📥️invocation/🏃️process/🟦️.ts` | 400 | 400 | `SCRIPT_PROCESS_INVOCATION_ENV`, `ScriptProcessEnvelope`, `createScriptProcessEnvelope`, `readScriptProcessEnvelope`, `receiveScriptProcessInvocation`, `scriptInvocationEnvironment`, `scriptProcessEnvironment`, `withScriptProcessEnvelope` |
| `🧭️routing/🚪️entrypoint/🟦️.ts` | 391 | 391 | `RunScriptMainOptions`, `runScriptMain` |
| `🧪️testing/🎚️budget/🟦️.ts` | 313 | 297 | `TEST_LEVELS`, `TEST_LEVEL_BUDGET_MS`, `TestLevel`, `activeTestLevel`, `atTestLevel`, `isTestLevel`, `resolveTestLevel`, `testLevelAtLeast`, `testLevelBudgetMs`, `testLevelBudgetSeconds`, `testLevelRank` |
| `⏱️budget/🟦️.ts` | 108 | 107 | `BUILD_BUDGET_MS`, `CMD_BUDGET_MS`, `DAEMON_BUDGET_MS`, `ORCHESTRATOR_BUDGET_MS`, `buildBudgetMs`, `cmdBudgetMs`, `daemonBudgetMs`, `orchestratorBudgetMs` |
| `🎛️owned-execution/🟦️.ts` | 105 | 100 | `OwnedCommandOptions`, `captureOwnedCommandV1`, `runOwnedCommand`, `startNativeProgress` |
| `🧪️testing/🦀️cargo/🟦️.ts` | 76 | 75 | `CargoTestPolicyV1`, `partitionNextestExecutionFilters`, `readCargoTestPolicyV1`, `runCargoLintV1`, `runCargoTestsV1` |
| `🧪️testing/🎛️execution/🟦️.ts` | 52 | 45 | `runBudgetedTestCommand` |
| `🧪️testing/🦀️cargo/🎯️exact/🟦️.ts` | 47 | 40 | `ExactCargoLawGroup`, `ExactCargoLawOptions`, `ExactCargoLawPort`, `ExactCargoLawReceipt`, `configuredExactCargoLawPolicyV1`, `exactCargoGeneratedOutputHasLiveLease`, `exactExecutableFingerprint`, `generatedOutputHasLiveLease`, `readExactCargoLawPolicyV1`, `runExactCargoLaws` |
| `🧪️testing/🧪️vitest/🟦️.ts` | 25 | 25 | `VitestPolicyV1`, `readVitestPolicyV1`, `runVitestV1`, `testCacheDirectoryV1`, `vitestArgumentsV1` |
| `🧭️routing/📥️invocation/🟦️.ts` | 23 | 19 | `ScriptControl`, `ScriptInvocation`, `advanceScriptInvocation`, `checkScriptInvocation`, `readScriptPolicy`, `scriptInvocationBudget` |
| `📦️artifacts/🏗️native-build/🟦️.ts` | 21 | 15 | `CARGO_BUILD_OWNER_PID_ENV`, `CargoArtifactBuildOptionsV1`, `CargoArtifactBuildPolicyV1`, `buildCargoArtifacts`, `cargoBinarySourcesFreshnessV1`, `cargoDirectoryEntriesV1`, `cargoInputDigestV1`, `cargoProvenancePhysicalControlV1`, `cargoProvenanceProgressSinkV1`, `packageNativeRelease`, `parseCargoBinarySourcesV1`, `readCargoArtifactBuildPolicyV1`, `signExecutableForDistribution`, `writeCompletedCargoInvocationProvenanceV1` |
| `📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts` | 18 | 18 | `acquireCargoBuildLeaseV1`, `cargoBuildLeaseIdentityV1` |
| `🔒️leases/🟦️.ts` | 15 | 15 | `acquireQueuedResourceLease`, `acquireResourceLease`, `withResourceLeases` |
| `🪓️termination/🟦️.ts` | 15 | 15 | `terminateOwnedChildTree`, `terminateOwnedProcessTree` |
| `📦️artifacts/🗂️files/🟦️.ts` | 13 | 13 | `collectArtifactFiles`, `seedGeneratedFile`, `writeGeneratedFileIfChanged` |
| `📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts` | 11 | 10 | `CargoArtifactObservationV1`, `CargoInputObservationV1`, `CargoProvenanceRootsV1`, `CargoProvenanceV1`, `CargoResourceWitnessV1`, `cargoBuildRunLeafV1`, `cargoDepInfoChecksumsV1`, `cargoDepInfoResolvedChecksumsV1`, `cargoDepInfoResolvedSourcesV1`, `cargoDepInfoSourcesV1`, `cargoProvenanceMerkleRootV1`, `cargoProvenanceRowsDigestV1`, `cargoProvenanceSourceV1`, `cargoUnitLeafV1`, `decodeCargoProvenanceV1`, `encodeCargoProvenanceV1` |
| `📥️capture/🟦️.ts` | 10 | 10 | `captureOwnedProcess` |
| `📋️context/🟦️.ts` | 7 | 6 | `ProcessOwnerContextV1`, `processCacheDirectoryV1`, `readProcessOwnerContextV1` |
| `📦️artifacts/🕸️wasm-build/🟦️.ts` | 7 | 7 | `WasmBuildPolicyV1`, `WasmPackWebBuildOptions`, `buildWasmWebV1`, `readWasmBuildPolicyV1`, `wasmBuildArguments`, `wasmOutputDirectory` |
| `📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts` | 5 | 5 | `LinkedComponentV1`, `RELEASE_DRIVER_POLICY_V1`, `ReleaseGenerationV1`, `ReleaseJobV1`, `ReleaseOutcomeV1`, `ReleasePipelineV1`, `ReleasePlanV1`, `ReleaseStageV1`, `admitReleasePipelineV1`, `admitReleasePlanV1`, `runReleaseDriverV1` |
| `📦️artifacts/📤️publication/🟦️.ts` | 5 | 5 | `ArtifactPublicationOptions`, `pinExecutableArtifact`, `stageArtifacts` |
| `🏗️build/🎚️mode/🟦️.ts` | 2 | 2 | `BuildMode`, `buildModeEnvironment`, `readBuildMode` |
| `📦️artifacts/🏗️native-build/🗃️stage-cache/🟦️.ts` | 2 | 2 | `STAGE_CACHE_POLICY_V1`, `StageCacheV1`, `fileSha256V1`, `stageCacheV1` |
| `🧭️routing/🎛️command/🟦️.ts` | 2 | 2 | `executeCommandV1` |
| `📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🟦️.ts` | 1 | 1 |  |
| `📦️artifacts/🏗️native-build/🧪️tests/📦️publication/🟦️.ts` | 1 | 1 | `testCargoArtifactPublication` |
| `📦️artifacts/📤️publication/🧪️tests/🟦️.ts` | 1 | 1 | `testArtifactPublication` |
| `🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts` | 1 | 1 | `testResourceLeases` |
| `🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🟦️.ts` | 1 | 1 |  |

Observed foreign deep reference total: 2239; distinct target facets: 30. Private test targets stay private and must be invoked through their actual owner command, retaining all assertions. Namespace or computed members require actual source review before explicit public contract decisions.

Dynamic imports record literal targets, while named/destructured consumers of those promises need manual source review. Named contracts in the table do not claim complete dynamic-member attribution.

Domain prefix source observations (overlap Framework includes Repo): ✏️s/🔌️plugins/ 369, 🧰️framework/🛍️products/🦑️repo/ 419, 🧰️framework/ 1482, 🌎️hub/ 258, 🎓️teaching/ 13, ♻️mit-bestand/ 18, 🏢️semio-tech/ 11. Five private-test literals are confined to four actual Library consumer files.

## Actual Authored Consumer Ports

After all3 original bounded fresh8 attempts physicallyclosed, authored2,197 focused import changes across675 actual consumer files in5,404.530ms over79,301,335 source bytes. Each choice uses handcrafted actual Process/public SDK target metadata, closest present authored source owner, exact manifest export target and explicit current named exports, validated with existing TypeScript AST. Plugin consumers use actual SDK facades; other foreign consumers use actual Process entries. Every file was reread before targeted write to preserve concurrent work; original parse diagnostic counts retained. No source bodies were stored, private test published, alias added or owner rule relaxed. Removed only unused nonexistent root exactCargoGeneratedOutputHasLiveLease import after actual source/AST confirmed no calls; no legacy name introduced.

This is authored receiving state, not runtime acceptance or finished whole graph.118 literal edges require explicit new named/semantic owner contracts, dynamic member attribution or actual private test commands. No remaining edge is exempted; all original suites and whole graph still required. Initial endpoint-check helper refused before any writes because its physical owner lookup was one directory short; that input mistake was corrected before the successful observation/mutation.

- `🗑️generated/pa12-inventory/public-consumer-ports-current.json` FILE SHA256 `636570eff7c2c7b86721669e9ece3b33d03f96c02fe91a8a8013cd042c0a4e77`.

| Required Actual Remaining Facet | Literal References |
| --- | ---: |
| `🏗️build/🎚️mode/🟦️.ts` | 43 |
| `🧪️testing/🦀️cargo/🎯️exact/🟦️.ts` | 34 |
| `📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts` | 11 |
| `📦️artifacts/🏗️native-build/🟦️.ts` | 7 |
| `📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts` | 5 |
| `🏗️build/🎟️capacity/🟦️.ts` | 5 |
| `🎛️owned-execution/🟦️.ts` | 3 |
| `📦️artifacts/🏗️native-build/🗃️stage-cache/🟦️.ts` | 2 |
| `📦️artifacts/🐹️go-build/🟦️.ts` | 1 |
| `🧭️routing/🟦️.ts` | 1 |
| `🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🟦️.ts` | 1 |
| `📦️artifacts/🗂️files/🟦️.ts` | 1 |
| `📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🟦️.ts` | 1 |
| `🔒️leases/🧪️tests/🔒️resource-leases/🟦️.ts` | 1 |
| `📦️artifacts/🏗️native-build/🧪️tests/📦️publication/🟦️.ts` | 1 |
| `📦️artifacts/📤️publication/🧪️tests/🟦️.ts` | 1 |

After genuine red/publication, remaining named imports were ported to new explicit actual APIs. Initial incremental attempt stopped at an unrelated removed Next config after partial valid writes; its early partial writes lack complete original perfile receipt, so no exactcustody claim. Current reread preserved absence and completed35 morefiles/44 literal edits with SHA-only write journal. Five Promise.all destructures were then manually reviewed for actual consumed functions and ported individually to actual public targets. Original private-test5 and a native-metadata3-name frontier remain visible. Full fresh census/runtime/wholegraph still required.


Current39 remaining four private capacity helper clients already declare actual SDK workspace dependency in their closest packages. Explicit public helper/type publication and literal ports remain pending defining red; no additional dependency alias or manual node_modules linkage required for these four observed consumers. New whole source census/graph remains required.
