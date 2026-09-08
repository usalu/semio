# TypeScript Test Layout Execution Audit

## Scope

This is a bounded read-only snapshot of authored JavaScript and TypeScript paths. The arrays were generated directly from this checkout in this audit turn; every path is repository-relative and literal. The parent lane owns the repository-wide contract gate, so it was not duplicated here.

No authored file currently uses a legacy `*.test.*` or `*.spec.*` basename. Stale runner strings and invalid locations remain.

## Parser blocker and duplicate suites

The parser pass found only the two library leaves below: `🔬️index` begins cascading diagnostics at line 6440 and `🔬️workspace-contract` at line 6354. Their damaged values are escaped synthetic fixture source, not paths to rebase. Restore fixture bytes and semantics before moving or deleting: they model package/export, Rust path, TypeScript, JSON, JSONC, TOML, XML, and CMake source.

The leaves have identical registered test labels and assertion structure. The comparison found 17 fixture-data-only differences, including malformed rewrites. Retain `🔬️workspace-contract` after it parses and validates; remove generic `🔬️index` only then.

```json
{
  "libraryContractCases": [
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️index/🟦️.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts"
  ]
}
```

## Structural findings

Every `flatTestImplementations` path lacks a named-case segment. `configurations` is the active `defineConfig` subset and must move out of `🧪️tests`. The WGPU tests need to move from delivery scope to an engine semantic owner.

```json
{
  "flatTestImplementations": [
    "🌎️hub/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️tests/🟦️.ts",
    "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🧪️tests/🟦️.ts",
    "🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧪️tests/🟦️.ts"
  ],
  "configurations": [
    "🌎️hub/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🟦️.ts",
    "🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🧪️tests/🟦️.ts",
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️tests/🟦️.ts"
  ],
  "wgpuDeliveryTests": [
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/📨️browser-frame-transport/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🧩️package-integration/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎮️browser-interactive-job-port/🟦️.ts"
  ]
}
```

The focused layout scanner found inline UI React test registration/assertion bodies at lines 10858 and 10933. Extract them into a UI owner case, then remove the `includeSource` entry that collects the source body. The current source locations are recorded below. The generic inventory intentionally excludes production `🟦️.tsx` files, which is why its UI subscan was empty.

```json
{
  "uiInlineBodies": [
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx",
      "line": 10858
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx",
      "line": 10933
    }
  ]
}
```

## Runner and configuration wiring

The engine React Vitest configuration includes `engine/🧪️tests/*/🟦️.{ts,tsx}`. It will collect command-only exported leaves with no Vitest registrar as empty suites. Enumerate Vitest-registered leaves and run command-only leaves only through their command runner.

The engine React `📜️script.ts` has a nonexistent old `🔬️index` import at lines 67 and 85. Root `📜️script.ts:8313` retains `INTERACTIVITY_AUDIT_PUZZLE_FILL_RENDERER_TEST_FILE` ending in `🧪️index.test.ts`. Recalculate each moved configuration's includes from its declared Vitest `root`; fixture strings need classification before changing them.

## Scoped script-body extraction lane

Each record is a non-root `📜️script.ts` named `test*` or `*SelfTests` function whose syntactic body contains `assert` or `expect`. Extract its executable body to `<semantic-owner>/🧪️tests/<named-case>/🟦️.ts` (or JS equivalent), leaving only guarded import/dependency-injection command wiring. The list is conservative: helper and non-test-named assertion functions require the follow-up full audit.

```json
{
  "scriptBodyCandidates": [
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/📜️script.ts",
      "line": 332,
      "name": "testStdioArtifactPackageGraph"
    },
    {
      "path": "✏️s/🔌️plugins/🗄️stdio/📜️script.ts",
      "line": 400,
      "name": "testTypeScriptPackage"
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts",
      "line": 329,
      "name": "testCacheContracts"
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔒️leases/🧪️tests/📜️script.ts",
      "line": 9,
      "name": "testResourceLeases"
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts",
      "line": 13,
      "name": "testCommandInputs"
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts",
      "line": 69,
      "name": "testBrowserModuleRelocation"
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts",
      "line": 94,
      "name": "testDemonstratorRuntime"
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts",
      "line": 161,
      "name": "testRuntimeComponents"
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts",
      "line": 199,
      "name": "testWorkspaceRoots"
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts",
      "line": 220,
      "name": "testBunDependencies"
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts",
      "line": 274,
      "name": "testNativePreparation"
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts",
      "line": 314,
      "name": "testNxDaemonDiagnostics"
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts",
      "line": 344,
      "name": "testNxDaemonTaskEnvironment"
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts",
      "line": 368,
      "name": "testNxDaemonRetention"
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🚀️bootstrap/📜️script.ts",
      "line": 7,
      "name": "testNxBootstrap"
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🖥️services/📜️script.ts",
      "line": 13,
      "name": "testContinuousServiceScenario"
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🖥️services/🌐️readiness/📜️script.ts",
      "line": 7,
      "name": "testServiceReadiness"
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️daemon/📜️script.ts",
      "line": 8,
      "name": "testGraphCoalescing"
    },
    {
      "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌐️browser/📜️script.ts",
      "line": 7,
      "name": "testBrowserDistribution"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/📜️script.ts",
      "line": 58,
      "name": "testFixtureSweepExtraction"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️script.ts",
      "line": 10,
      "name": "testGroupVisibilityFixtures"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/📜️script.ts",
      "line": 12,
      "name": "testCanonicalEditFixtures"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧵️send/📜️script.ts",
      "line": 8,
      "name": "testNativeCodecSendFixture"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/✂️detach/📜️script.ts",
      "line": 8,
      "name": "testBackboneDetachFixture"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/📜️script.ts",
      "line": 14,
      "name": "testDurableOwnedGroupDecisionFixture"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🪪️member-dialect/📜️script.ts",
      "line": 9,
      "name": "testMemberDialectFixture"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌱️initial/🪪️identity/📜️script.ts",
      "line": 10,
      "name": "testInitialChildIdentityFixture"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🔎️scalar-witness/📜️script.ts",
      "line": 33,
      "name": "testScalarRecordWireFixture"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️script.ts",
      "line": 21,
      "name": "pluginTestRunnerSelfTests"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🔣️codec/🧵️send/📜️script.ts",
      "line": 277,
      "name": "testPluginCodecCallerSource"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts",
      "line": 653,
      "name": "testClosedBrowserComponentFactory"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts",
      "line": 744,
      "name": "testBrowserCodegenCapsule"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts",
      "line": 784,
      "name": "testBrowserCodegenSources"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts",
      "line": 832,
      "name": "testBrowserCodegenPolicy"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts",
      "line": 882,
      "name": "testBrowserActorCodegenManifest"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts",
      "line": 897,
      "name": "testClosedBrowserActorBundle"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📜️script.ts",
      "line": 1107,
      "name": "testBrowserHostActivation"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌐️wasi-activation/📜️script.ts",
      "line": 9,
      "name": "testBrowserWasiActivation"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧪️fixtures/🌊️actor-import/📜️script.ts",
      "line": 35,
      "name": "testCanonicalActorAsyncImport"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/📜️script.ts",
      "line": 7,
      "name": "testFixtureProjectionRetirement"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts",
      "line": 847,
      "name": "testFreshComponentSourceEpochV1"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts",
      "line": 947,
      "name": "testFreshComponentStagingV1"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts",
      "line": 1160,
      "name": "testFreshComponentProcessV1"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/📜️script.ts",
      "line": 88,
      "name": "testFlowBrowserDeclaration"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🪪️runtime/📜️script.ts",
      "line": 8,
      "name": "testDirectoryRuntimeIdentityFixture"
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/📜️script.ts",
      "line": 11,
      "name": "testInputAdmissionFixture"
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/📜️script.ts",
      "line": 8,
      "name": "testInputCommitObserverFixture"
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/📥️enqueue/📜️script.ts",
      "line": 7,
      "name": "testSingleEnqueuePublicationFixture"
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🪪️root/📜️script.ts",
      "line": 7,
      "name": "testInputRootFixture"
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/✍️writer/📜️script.ts",
      "line": 7,
      "name": "testInputWriterFixture"
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/♻️retirement/🌲️built/📜️script.ts",
      "line": 6,
      "name": "testBuiltTreeRetirementFixture"
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts",
      "line": 20,
      "name": "fixedListStorageSelfTests"
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts",
      "line": 184,
      "name": "conformanceCorpusSelfTests"
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/📜️script.ts",
      "line": 7,
      "name": "testRuntimeTreeRetirement"
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts",
      "line": 20,
      "name": "surfaceOwnershipSelfTests"
    },
    {
      "path": "🧰️framework/🔨️modules/⏱️trace/⏱️clock/🏁️tail/📜️script.ts",
      "line": 7,
      "name": "testWatchdogTailFixture"
    },
    {
      "path": "🧰️framework/🔨️modules/🎯️action-bus/🧹️wire-retirement/📜️script.ts",
      "line": 8,
      "name": "testWireRetirementFixture"
    }
  ]
}
```

The following scripts are already inside a test tree with an invalid `📜️script.ts` implementation name. The caching root script also lacks a named case. Move test bodies and retain custom command wrappers outside the test tree, or make them explicitly import their canonical leaf. The current public `bun nx ...` bootstrap now points to the owner-level caching `🚀️bootstrap/📜️script.ts`; do not recreate root `bun ./📜️script.ts nx`.

```json
{
  "scriptFilesInsideTests": [
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔒️leases/🧪️tests/📜️script.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/📜️script.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🚀️bootstrap/📜️script.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🖥️services/📜️script.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🖥️services/🌐️readiness/📜️script.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🕸️daemon/📜️script.ts",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🌐️browser/📜️script.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🛂️mutation-source-authority/📜️script.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/📤️macro-exports/📜️script.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🌲️fixture-projection/📜️script.ts"
  ]
}
```

## Execution order

1. Restore and parse the library fixture sources; validate `🔬️workspace-contract`, then remove duplicate `🔬️index`.

2. Move flat configurations and correct includes from each declared Vitest root.

3. Move WGPU delivery tests, extract UI React inline bodies, and narrow the engine glob before adding command-only canonical leaves.

4. Extract the script candidates by owner, beginning with the library caching subtree because it has invalid test-tree scripts and public Nx bootstrap wiring.

5. Rerun focused parser/layout scans, then the repository contract gate after concurrent mutation lanes finish.
