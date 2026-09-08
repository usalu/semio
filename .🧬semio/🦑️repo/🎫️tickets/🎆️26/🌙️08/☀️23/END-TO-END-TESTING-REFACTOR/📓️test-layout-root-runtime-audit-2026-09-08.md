# Extracted Root Policy Runtime Audit

The public `bun nx exec --projects=workspace` probe loaded all 54 canonical implementation modules. It invoked functions without arguments, which was valid for 37 functions: 27 passed and 10 failed. The other 17 functions require source/context arguments; their observed errors are **probe invocation errors**, not evidence that their tests fail. Those 17 must be verified through their existing command dispatchers or with the exact dispatcher arguments.

Nine of the ten valid failures first reached stale schema lookups (including the aggregate coverage function); the other reported a microsecond worker binding mismatch. Targeted schema repairs and subsequent results follow. No assertions are weakened.

```json
[
  {
    "name": "toolJobOwnerFactoryResolutionSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-owner-factory-resolution/🟦️.ts",
    "status": "failed",
    "error": "Error: ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧬️schema/🏭️owner-factory-resolution.schema.json'"
  },
  {
    "name": "toolJobFactoryProofJoinSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-factory-proof-join/🟦️.ts",
    "status": "failed",
    "error": "Error: ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📐️tool-factory-proof.schema.json'"
  },
  {
    "name": "toolJobCooperativeMaintenanceSelfTests",
    "path": "🧰️framework/🔨️modules/⏳️async/🤝️cooperative/🧪️tests/🔬️tool-job-cooperative-maintenance/🟦️.ts",
    "status": "passed",
    "count": 8
  },
  {
    "name": "toolJobTelemetryContentionSelfTests",
    "path": "🧰️framework/🔨️modules/⏱️trace/⏱️clock/🧪️tests/🔬️tool-job-telemetry-contention/🟦️.ts",
    "status": "passed",
    "count": 11
  },
  {
    "name": "toolJobMicrosecondBudgetSelfTests",
    "path": "🧰️framework/🔨️modules/🧵️job/⏱️budget/🧪️tests/🔬️tool-job-microsecond-budget/🟦️.ts",
    "status": "failed",
    "error": "Error: microsecond exact worker binding: none"
  },
  {
    "name": "cadPresenceRetirementSelfTests",
    "path": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts",
    "status": "failed",
    "error": "Error: ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🔣️.schema.json'"
  },
  {
    "name": "toolJobLatestWinsSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts",
    "status": "failed",
    "error": "Error: ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📏️tool-latest-wins.schema.json'"
  },
  {
    "name": "storeCanonicalEditSealerSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️store-canonical-edit-sealer/🟦️.ts",
    "status": "failed",
    "error": "Error: ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧬️schema/🔏️canonical-edit-sealer.schema.json'"
  },
  {
    "name": "canonicalErrorProgressSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️canonical-error-progress/🟦️.ts",
    "status": "failed",
    "error": "Error: ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧬️schema/🚧️canonical-error-progress.schema.json'"
  },
  {
    "name": "proceduralGenerationRootSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🧬️generation/🧪️tests/🔬️procedural-generation-root/🟦️.ts",
    "status": "passed",
    "count": 12
  },
  {
    "name": "flowTypedRetirementSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🧪️tests/🔬️flow-typed-retirement/🟦️.ts",
    "status": "passed",
    "count": 9
  },
  {
    "name": "flowSelectedCopySelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/🔬️flow-selected-copy/🟦️.ts",
    "status": "passed",
    "count": 15
  },
  {
    "name": "toolJobScalarConfigCohortSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-scalar-config-cohort/🟦️.ts",
    "status": "failed",
    "error": "Error: ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧬️schema/🎚️scalar-config-cohort.schema.json'"
  },
  {
    "name": "toolJobArtifactEnvelopeRejectionTransferSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️tool-job-artifact-envelope-rejection-transfer/🟦️.ts",
    "status": "incorrect-probe-invocation",
    "parameters": "store: string"
  },
  {
    "name": "toolJobPuzzleReservedRoutesSelfTests",
    "path": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️tool-job-puzzle-reserved-routes/🟦️.ts",
    "status": "passed",
    "count": 14
  },
  {
    "name": "toolJobLiveFixedReplaySelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-live-fixed-replay/🟦️.ts",
    "status": "incorrect-probe-invocation",
    "parameters": "actor: string, shard: string, executor: string, wgpu: string, actionBus: string, pluginApp: string, pluginHost: string, programBridge: string"
  },
  {
    "name": "toolJobFemNumericalMicrocursorSelfTests",
    "path": "✏️s/🔌️plugins/🏗️fem/🧪️tests/🔬️tool-job-fem-numerical-microcursor/🟦️.ts",
    "status": "incorrect-probe-invocation",
    "parameters": "sparse: string, mesh: string, analyses: string, model: string, elements: string, session: string, runtime: string"
  },
  {
    "name": "toolJobCheckpointSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-checkpoint/🟦️.ts",
    "status": "failed",
    "error": "Error: ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧬️schema/📸️artifact-command-checkpoint.schema.json'"
  },
  {
    "name": "toolJobCoverageSelfTests",
    "path": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-coverage/🟦️.ts",
    "status": "failed",
    "error": "Error: ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧬️schema/📸️artifact-command-checkpoint.schema.json'"
  },
  {
    "name": "toolJobFixedOperationRegistrySelfTests",
    "path": "🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️tool-job-fixed-operation-registry/🟦️.ts",
    "status": "incorrect-probe-invocation",
    "parameters": "source: string"
  },
  {
    "name": "toolJobDrawingGestureOperationOwnerSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-drawing-gesture-operation-owner/🟦️.ts",
    "status": "passed",
    "count": 18
  },
  {
    "name": "toolJobFemLiveVisualPublicationSelfTests",
    "path": "✏️s/🔌️plugins/🏗️fem/🧪️tests/🔬️tool-job-fem-live-visual-publication/🟦️.ts",
    "status": "incorrect-probe-invocation",
    "parameters": "fem2dModel: string,\n  fem2dSession: string,\n  fem3dSession: string,\n  fem3dEditor: string,\n  fem3dModel: string,\n  fem3dResults: string,\n  fem3dViewer: string,\n  fem3dViewerModel: string,\n  femPluginRoot: string,\n  femGlue: string,\n  femSparse: string,\n  frameworkPlugin: string,\n  frameworkWorld: string,\n  worldSnapshot: string,\n  canvasSnapshot: string,\n  canvasRenderer: string,\n  femAnalyses: string,\n  femMesh: string,"
  },
  {
    "name": "toolJobArtifactRetainedCommandSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-artifact-retained-command/🟦️.ts",
    "status": "incorrect-probe-invocation",
    "parameters": "source: string, runtimeLawSource: string"
  },
  {
    "name": "interactivityAllAppDiscoverySelfTests",
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🔬️interactivity-all-app-discovery/🟦️.ts",
    "status": "passed",
    "count": 25
  },
  {
    "name": "interactivityRuntimeSourceSelfTests",
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🔬️interactivity-runtime-source/🟦️.ts",
    "status": "passed",
    "count": null
  },
  {
    "name": "interactivityPuzzleFillEnvelopeSelfTests",
    "path": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-envelope/🟦️.ts",
    "status": "incorrect-probe-invocation",
    "parameters": "repoRoot: string"
  },
  {
    "name": "interactivityPuzzleFillP4eSelfTests",
    "path": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-p4e/🟦️.ts",
    "status": "incorrect-probe-invocation",
    "parameters": "repoRoot: string"
  },
  {
    "name": "interactivityPuzzleFillPreviewJsonSelfTests",
    "path": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-preview-json/🟦️.ts",
    "status": "incorrect-probe-invocation",
    "parameters": "repoRoot: string"
  },
  {
    "name": "interactivityLiveReconcileSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-live-reconcile/🟦️.ts",
    "status": "incorrect-probe-invocation",
    "parameters": "repoRoot: string"
  },
  {
    "name": "interactivityMountedLayoutTextSelfTests",
    "path": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️interactivity-mounted-layout-text/🟦️.ts",
    "status": "incorrect-probe-invocation",
    "parameters": "repoRoot: string"
  },
  {
    "name": "interactivityMountedFrameTransactionSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-mounted-frame-transaction/🟦️.ts",
    "status": "incorrect-probe-invocation",
    "parameters": "repoRoot: string"
  },
  {
    "name": "interactivityMountedEngineSurfaceLifetimeSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-mounted-engine-surface-lifetime/🟦️.ts",
    "status": "incorrect-probe-invocation",
    "parameters": "repoRoot: string"
  },
  {
    "name": "interactivityMountedPreparedRenderSelfTests",
    "path": "🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️interactivity-mounted-prepared-render/🟦️.ts",
    "status": "incorrect-probe-invocation",
    "parameters": "repoRoot: string"
  },
  {
    "name": "interactivityMountedSurfaceLaneSelfTests",
    "path": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️interactivity-mounted-surface-lane/🟦️.ts",
    "status": "incorrect-probe-invocation",
    "parameters": "repoRoot: string"
  },
  {
    "name": "interactivityPreparedRasterProducerSelfTests",
    "path": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️interactivity-prepared-raster-producer/🟦️.ts",
    "status": "passed",
    "count": null
  },
  {
    "name": "interactivityShardExecutorSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧪️tests/🔬️interactivity-shard-executor/🟦️.ts",
    "status": "passed",
    "count": null
  },
  {
    "name": "interactivityStoreSyncSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️interactivity-store-sync/🟦️.ts",
    "status": "passed",
    "count": null
  },
  {
    "name": "interactivityDbIoB1B6SelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️tests/🔬️interactivity-db-io-b1-b6/🟦️.ts",
    "status": "passed",
    "count": null
  },
  {
    "name": "interactivityP1qR4SelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-p1q-r4/🟦️.ts",
    "status": "passed",
    "count": null
  },
  {
    "name": "interactivityDbIoCallerMigrationSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-db-io-caller-migration/🟦️.ts",
    "status": "passed",
    "count": null
  },
  {
    "name": "interactivityDbIoDirectWriterSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-db-io-direct-writer/🟦️.ts",
    "status": "passed",
    "count": null
  },
  {
    "name": "interactivityDbIoSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️tests/🔬️interactivity-db-io/🟦️.ts",
    "status": "passed",
    "count": null
  },
  {
    "name": "interactivityArtifactSubmitSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️interactivity-artifact-submit/🟦️.ts",
    "status": "passed",
    "count": null
  },
  {
    "name": "interactivityDatabaseCapabilityOpenSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️interactivity-database-capability-open/🟦️.ts",
    "status": "passed",
    "count": null
  },
  {
    "name": "interactivityDatabaseCatalogReadSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️interactivity-database-catalog-read/🟦️.ts",
    "status": "passed",
    "count": null
  },
  {
    "name": "interactivityDatabaseCatalogBootstrapSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️interactivity-database-catalog-bootstrap/🟦️.ts",
    "status": "passed",
    "count": null
  },
  {
    "name": "interactivityDatabaseCreateCatalogSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️interactivity-database-create-catalog/🟦️.ts",
    "status": "passed",
    "count": null
  },
  {
    "name": "interactivityDatabaseCompactionSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-database-compaction/🟦️.ts",
    "status": "incorrect-probe-invocation",
    "parameters": "compact: string, engine: string, snapshot: string, index: string, contract: string"
  },
  {
    "name": "interactivityDatabaseSyncHelloSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-database-sync-hello/🟦️.ts",
    "status": "incorrect-probe-invocation",
    "parameters": "sync: string, engine: string, hub: string, wal: string, protocol: string, contract: string"
  },
  {
    "name": "interactivityArtifactHistorySelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-artifact-history/🟦️.ts",
    "status": "passed",
    "count": null
  },
  {
    "name": "interactivityVcsBridgeSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-vcs-bridge/🟦️.ts",
    "status": "passed",
    "count": null
  },
  {
    "name": "interactivityMcpHttpTransportSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚚️transport/🧪️tests/🔬️interactivity-mcp-http-transport/🟦️.ts",
    "status": "passed",
    "count": null
  },
  {
    "name": "dependencyJsLockParitySelfTests",
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧪️tests/🔬️dependency-js-lock-parity/🟦️.ts",
    "status": "passed",
    "count": 5
  },
  {
    "name": "dependencyTruthSelfTests",
    "path": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧪️tests/🔬️dependency-truth/🟦️.ts",
    "status": "passed",
    "count": 18
  }
]
```

## Schema Reference Follow-up

Eight canonical suites were updated to the observed owner component schema and its named `$defs` entry. The checkpoint oracle now uses the current component schema draft (draft-07). The rerun passed owner factory resolution (21), factory proof join (28), and checkpoint (15). The other results below reached deeper current-schema/source checks and are not reported as passes.

```json
[
  {
    "name": "toolJobOwnerFactoryResolutionSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-owner-factory-resolution/🟦️.ts",
    "passed": true,
    "count": 21
  },
  {
    "name": "toolJobFactoryProofJoinSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-factory-proof-join/🟦️.ts",
    "passed": true,
    "count": 28
  },
  {
    "name": "cadPresenceRetirementSelfTests",
    "path": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts",
    "passed": false,
    "error": "Error: strict mode: unknown keyword: \"x-semio-state\"",
    "stack": "Error: strict mode: unknown keyword: \"x-semio-state\"\n    at checkStrictMode (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/util.js:174:19)\n    at checkUnknownRules (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/util.js:32:13)\n    at alwaysValidSchema (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/util.js:19:5)\n    at <anonymous> (/Users/ueli/Documents/semio/node_modules/ajv/dist/vocabularies/applicator/properties.js:23:63)\n    at filter (native:1:11)\n    at code (/Users/ueli/Documents/semio/node_modules/ajv/dist/vocabularies/applicator/properties.js:23:37)\n    at keywordCode (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/validate/index.js:464:13)\n    at <anonymous> (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/validate/index.js:222:17)\n    at code (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/codegen/index.js:439:13)\n    at block (/Users/ueli/Documents/semio/node_modules/ajv/dist/compile/codegen/index.js:568:18)"
  },
  {
    "name": "toolJobLatestWinsSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts",
    "passed": false,
    "error": "Error: latest-wins production admission/publication authority is incomplete",
    "stack": "Error: latest-wins production admission/publication authority is incomplete\n    at toolJobLatestWinsSelfTests (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts:87:33)\n    at /Users/ueli/Documents/semio/[eval]:1:480"
  },
  {
    "name": "storeCanonicalEditSealerSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️store-canonical-edit-sealer/🟦️.ts",
    "passed": false,
    "error": "Error: borrowed map schema accepted hostile lifetime shape",
    "stack": "Error: borrowed map schema accepted hostile lifetime shape\n    at storeCanonicalEditSealerSelfTests (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️store-canonical-edit-sealer/🟦️.ts:109:80)\n    at /Users/ueli/Documents/semio/[eval]:1:480"
  },
  {
    "name": "canonicalErrorProgressSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️canonical-error-progress/🟦️.ts",
    "passed": false,
    "error": "Error: canonical error-progress schema admitted forged credit or completion",
    "stack": "Error: canonical error-progress schema admitted forged credit or completion\n    at canonicalErrorProgressSelfTests (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️canonical-error-progress/🟦️.ts:17:68)\n    at /Users/ueli/Documents/semio/[eval]:1:480"
  },
  {
    "name": "toolJobScalarConfigCohortSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-scalar-config-cohort/🟦️.ts",
    "passed": false,
    "error": "Error: ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs'",
    "stack": "Error: ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs'\n    at readFileSync (unknown)\n    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-scalar-config-cohort/🟦️.ts:17:86)\n    at map (native:1:11)\n    at toolJobScalarConfigCohortSelfTests (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-scalar-config-cohort/🟦️.ts:17:57)\n    at /Users/ueli/Documents/semio/[eval]:1:480"
  },
  {
    "name": "toolJobCheckpointSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-checkpoint/🟦️.ts",
    "passed": true,
    "count": 15
  }
]
```

## Correct Repository-context Invocation

The nine functions declaring a `repoRoot: string` argument were rerun with the actual workspace root through public Bun/Nx. These are valid invocation results. Failures need separation between source-evidence changes caused by extraction and unrelated concurrent runtime changes; none are reported as passing.

```json
[
  {
    "name": "interactivityPuzzleFillEnvelopeSelfTests",
    "path": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-envelope/🟦️.ts",
    "passed": false,
    "error": "Error: [verify interactivity] Puzzle fill envelope baseline was falsely rejected: Puzzle fill admission does not advance one fixed nested allocation/entry backed by the exact credited slot pages before reservation; Puzzle fill semantic identity/cap/ABA/accounting/terminal/retirement fixtures are missing",
    "stack": "Error: [verify interactivity] Puzzle fill envelope baseline was falsely rejected: Puzzle fill admission does not advance one fixed nested allocation/entry backed by the exact credited slot pages before reservation; Puzzle fill semantic identity/cap/ABA/accounting/terminal/retirement fixtures are missing\n    at interactivityPuzzleFillEnvelopeSelfTests (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-envelope/🟦️.ts:74:40)\n    at /Users/ueli/Documents/semio/[eval]:1:465"
  },
  {
    "name": "interactivityPuzzleFillP4eSelfTests",
    "path": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-p4e/🟦️.ts",
    "passed": false,
    "error": "Error: [verify interactivity] Puzzle fill P4e baseline was falsely rejected: P4e preparation preflight/storage omits a fixture, mesh, catalog, or compatibility fixed owner or lacks exact cap/+1 handback evidence; P4e capacity refusal does not publish an active generation-qualified no-ghost diagnostic before terminal fault/removal; P4e spatial owner is not fixed, resumable, generation-bound, and used by the production broad phase; P4e preview publication is not the canonical bounded diagnostic page; P4e preparation fixture missing constructor_cap_and_plus_one_take_bounded_turns_and_refuse_permanently; P4e preparation fixture missing stale_generation_stops_preparation_before_installing_any_entry; P4e spatial fixture missing spatial_resumable_query_narrows_sparse_cells_without_visiting_distant_population; P4e spatial fixture missing spatial_capacity_plus_one_refusal_preserves_exact_old_state; P4e spatial fixture missing spatial_stale_owner_cannot_finish_partial_replacement; P4e spatial fixture missing spatial_multi_cell_oversized_replacement_and_removal_make_bounded_progress",
    "stack": "Error: [verify interactivity] Puzzle fill P4e baseline was falsely rejected: P4e preparation preflight/storage omits a fixture, mesh, catalog, or compatibility fixed owner or lacks exact cap/+1 handback evidence; P4e capacity refusal does not publish an active generation-qualified no-ghost diagnostic before terminal fault/removal; P4e spatial owner is not fixed, resumable, generation-bound, and used by the production broad phase; P4e preview publication is not the canonical bounded diagnostic page; P4e preparation fixture missing constructor_cap_and_plus_one_take_bounded_turns_and_refuse_permanently; P4e preparation fixture missing stale_generation_stops_preparation_before_installing_any_entry; P4e spatial fixture missing spatial_resumable_query_narrows_sparse_cells_without_visiting_distant_population; P4e spatial fixture missing spatial_capacity_plus_one_refusal_preserves_exact_old_state; P4e spatial fixture missing spatial_stale_owner_cannot_finish_partial_replacement; P4e spatial fixture missing spatial_multi_cell_oversized_replacement_and_removal_make_bounded_progress\n    at interactivityPuzzleFillP4eSelfTests (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-p4e/🟦️.ts:59:40)\n    at /Users/ueli/Documents/semio/[eval]:1:465"
  },
  {
    "name": "interactivityPuzzleFillPreviewJsonSelfTests",
    "path": "✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-preview-json/🟦️.ts",
    "passed": false,
    "error": "Error: [verify interactivity] Puzzle fill preview self-test mutation missing-oracle-law no longer reaches production source.",
    "stack": "Error: [verify interactivity] Puzzle fill preview self-test mutation missing-oracle-law no longer reaches production source.\n    at interactivityPuzzleFillPreviewJsonSelfTests (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-preview-json/🟦️.ts:53:54)\n    at /Users/ueli/Documents/semio/[eval]:1:465"
  },
  {
    "name": "interactivityLiveReconcileSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-live-reconcile/🟦️.ts",
    "passed": false,
    "error": "Error: [verify interactivity] live reconcile self-test per-surface-credit-cap made no source mutation.",
    "stack": "Error: [verify interactivity] live reconcile self-test per-surface-credit-cap made no source mutation.\n    at interactivityLiveReconcileSelfTests (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-live-reconcile/🟦️.ts:110:111)\n    at /Users/ueli/Documents/semio/[eval]:1:465"
  },
  {
    "name": "interactivityMountedLayoutTextSelfTests",
    "path": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️interactivity-mounted-layout-text/🟦️.ts",
    "passed": false,
    "error": "Error: [verify interactivity] P5c mutation unbounded-renderer-budget did not alter source.",
    "stack": "Error: [verify interactivity] P5c mutation unbounded-renderer-budget did not alter source.\n    at interactivityMountedLayoutTextSelfTests (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️interactivity-mounted-layout-text/🟦️.ts:49:52)\n    at /Users/ueli/Documents/semio/[eval]:1:465"
  },
  {
    "name": "interactivityMountedFrameTransactionSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-mounted-frame-transaction/🟦️.ts",
    "passed": false,
    "error": "Error: [verify interactivity] P5a mutation missing-max-identity did not alter source.",
    "stack": "Error: [verify interactivity] P5a mutation missing-max-identity did not alter source.\n    at interactivityMountedFrameTransactionSelfTests (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-mounted-frame-transaction/🟦️.ts:114:52)\n    at /Users/ueli/Documents/semio/[eval]:1:465"
  },
  {
    "name": "interactivityMountedEngineSurfaceLifetimeSelfTests",
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-mounted-engine-surface-lifetime/🟦️.ts",
    "passed": false,
    "error": "Error: [verify interactivity p3mn] mutation flow-whole-store-drop did not bind live source",
    "stack": "Error: [verify interactivity p3mn] mutation flow-whole-store-drop did not bind live source\n    at interactivityMountedEngineSurfaceLifetimeSelfTests (/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-mounted-engine-surface-lifetime/🟦️.ts:46:52)\n    at /Users/ueli/Documents/semio/[eval]:1:465"
  },
  {
    "name": "interactivityMountedPreparedRenderSelfTests",
    "path": "🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️interactivity-mounted-prepared-render/🟦️.ts",
    "passed": false,
    "error": "Error: [verify interactivity p5d] mutation missing-input-drop-law did not bind live source",
    "stack": "Error: [verify interactivity p5d] mutation missing-input-drop-law did not bind live source\n    at interactivityMountedPreparedRenderSelfTests (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests/🔬️interactivity-mounted-prepared-render/🟦️.ts:63:52)\n    at /Users/ueli/Documents/semio/[eval]:1:465"
  },
  {
    "name": "interactivityMountedSurfaceLaneSelfTests",
    "path": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️interactivity-mounted-surface-lane/🟦️.ts",
    "passed": false,
    "error": "Error: [verify interactivity p5e] mutation wide-worker-deadline did not bind live source",
    "stack": "Error: [verify interactivity p5e] mutation wide-worker-deadline did not bind live source\n    at interactivityMountedSurfaceLaneSelfTests (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️interactivity-mounted-surface-lane/🟦️.ts:28:52)\n    at /Users/ueli/Documents/semio/[eval]:1:465"
  }
]
```
