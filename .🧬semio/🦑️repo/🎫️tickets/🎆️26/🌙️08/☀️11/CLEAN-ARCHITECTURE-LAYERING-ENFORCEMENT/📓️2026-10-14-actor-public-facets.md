# Defining Actor Public Facet Inventory

The current Actor TypeScript package publishes only its root turn-scheduler wildcard export. The complete neutral graph retains29 cross-package relative Actor findings, including lifetime/patch/wire/shard and schema consumers. Defining source/test Product dependencies were removed in the preceding context report. This inventory reads actual exported declarations with the independent TypeScript compiler before changing public interfaces.

## 📃️page/🟦️.ts

Runtime: `ACTOR_BYTE_PAGE_BYTES`, `createActorBytePage`, `readActorBytePage`

Types: `ActorBytePageBlock`, `ActorBytePage`

## 🚪️lifetime/🟦️.ts

Runtime: `ACTOR_INSTANCE_LIFECYCLE_MAXIMUM_BYTES`, `encodeActorInstanceLifecycle`, `decodeActorInstanceLifecycle`, `actorInstanceLifetimeEquals`, `actorInstanceLifecycleReceiptEquals`, `actorInstanceCapturedReceiptMatches`, `actorInstanceCloseReceiptMatches`

Types: `ActorInstanceLifetime`, `ActorInstanceOpenRequest`, `ActorInstanceCloseRequest`, `ActorInstanceLifecycleReceipt`, `ActorInstanceLifecycleAck`, `ActorInstanceLifecycleWire`

## 🚪️lifetime/🩹️patch/🟦️.ts

Runtime: `ACTOR_UI_PATCH_RECEIPT_MAXIMUM_BYTES`, `encodeActorUiPatchReceipt`, `decodeActorUiPatchReceipt`, `actorUiPatchReceiptEquals`, `validateActorUiPatchPairing`

Types: `ActorUiPatchReceipt`

## 📮️shard-client/🟦️.ts

Runtime: `SHARD_COMMAND_MAXIMUM_PAGES`, `createShardCommandIngressPages`, `MAINTENANCE_LANE_DEFAULT_BUDGET`, `SHARD_FRAME_VARIANT_FIELDS`, `orderEnvelopesByLane`, `createGrantedBudgetTracker`, `interpretShardFrame`, `evaluateShardLiveness`, `isShardLostError`, `describeShardWorkerError`, `describeShardSilence`, `settleFailedInstanceOpen`, `describeShardMessageError`, `formatShardWorkerFault`, `SHARD_JSPI_FAULT_CODE`, `SHARD_JSPI_FAULT_TEXT`, `shardJspiAvailable`, `ShardJspiUnavailableError`, `assertShardJspiAvailable`, `RETRYABLE_LIFECYCLE_TURN`, `isRetryableLifecycleTurn`, `RETRYABLE_LIFECYCLE_TURN_ATTEMPTS`, `OwnedShardReturn`, `OwnedShardReturnPage`, `OwnedNativeUiPatchAuthority`, `OwnedNativeUiPatchSubmissionReceipt`, `ShardClient`, `shardClientTestDependenciesV1`, `shardClientRetryableLifecycleTestDependenciesV1`

Types: `ShardBudget`, `ShardJobBudget`, `ShardCapabilityGrant`, `ShardEventEnvelope`, `ShardCommandPageCursor`, `ShardCommandIngressPage`, `ShardAsset`, `ShardJobStep`, `ShardMetrics`, `ShardMetricsSample`, `ShardOrigin`, `ShardEnvelope`, `ShardFrame`, `GrantedBudgetTracker`, `ShardFrameDispatch`, `ShardQuotaBreach`, `HostEffectHandler`, `ShardWorkerLike`, `CreateShardWorker`, `ShardCarriedBeat`, `ShardLivenessWindow`, `ShardLivenessDecision`, `ShardSilenceReport`, `ShardJspiScope`, `ShardInstanceOpenInput`, `ShardInstanceLifecycleLease`, `ShardActorActivationLease`, `ShardReturnReport`, `ShardReturnAdmission`, `OwnedNativeUiPatchValue`, `OwnedUiPatchAcknowledgementEntry`, `ShardClientOptions`, `ShardClientTestDependenciesV1`, `ShardClientRetryableLifecycleTestDependenciesV1`

## 📮️shard-client/🧬️component-codec/🟦️.ts

Runtime: `ACTOR_CODEC_REFUSAL`, `actorCodecAnswer`

Types: `ShardCodecRequest`, `ShardCodecDocumentPair`, `ShardCodecAnswer`, `ActorCodecExports`

## 📮️shard-client/🫀️liveness/🟦️.ts

Runtime: `SHARD_LIVENESS_POLICY`

Types: ``

## 🖼️wire-turn/🟦️.ts

Runtime: `coerceWireBytes`, `coerceTurnResult`, `shellFrameBytes`, `shellMessageKind`, `leftoverShellInvocationFrames`, `wireSendMessageTargetTag`, `WIRE_SEND_MESSAGE_ROUTED_TARGETS`, `isRoutedWireSendMessage`, `TYPED_OPERATION_PAGE_MAGIC`, `TYPED_OPERATION_ACK_MAGIC`, `TYPED_OPERATION_PAGE_HEADER_BYTES`, `TYPED_OPERATION_TOKEN_BYTES`, `TYPED_OPERATION_RESULT_PAGE_BYTES`, `TYPED_OPERATION_RESULT_LANE_MAX`, `TYPED_OPERATION_LANE_ARTIFACT`, `TYPED_OPERATION_LANE_TERMINAL`, `TYPED_OPERATION_LANE_FAULT`, `typedOperationResult`, `typedOperationAcknowledgements`, `scanTypedOperationPages`, `typedOperationPageAnswerV1`, `drainTypedOperationTurns`, `decodeWirePatchOps`, `applyUiPatchToRetained`, `wireExtensionInvocation`, `wireRespondAnswer`, `wireOptionValue`, `wireOptionText`, `wireMediaExportEncoding`, `wireDownloadMediaExport`, `wireIconRenderExport`, `wireVideoRenderExport`, `wireSpawnJob`, `wireCancelJob`, `wireJobIdentity`, `wireTurnStatusTag`, `shardTurnCarriesNothingV1`, `shardTurnDriveStepsV1`, `shardTurnDriveBudgetMsV1`, `driveShardTurnMoreWorkV1`, `INBOUND_REQUEST_TURN_BUDGET`, `driveInboundRequest`, `wireJobStep`, `driveSpawnedJob`, `spawnedJobCompletedEvent`, `wireSpawnedJobs`, `wireEffectToFriendly`

Types: `WireVariant`, `WireUiPatch`, `WireTurnResult`, `TypedOperationToken`, `TypedOperationAckEvent`, `TypedOperationPage`, `TypedOperationPageScan`, `PatchOp`, `RetainedSurface`, `ShardTurnCarriers`, `InboundRequestDrive`, `InboundRequestTurn`, `InboundRequestAnswer`, `SpawnedJobPort`, `SpawnedJobDrive`

## 🎟️retained-turn/🟦️.ts

Runtime: `KernelOriginalResourcePool`, `KernelOriginalOperationIssuer`, `OriginalOperationReceiptRefusal`, `OriginalOperationSource`, `originalTurnWit`, `originalOperationWorkerSource`, `originalReceiptFromWit`

Types: `OriginalRetainedGrant`, `OriginalRetainedProgress`, `OriginalRetainedInput`, `OriginalOperationResources`, `OriginalOperationAdmission`, `OriginalOperationTurn`, `OriginalOperationReceipt`, `OriginalWireGrant`, `OriginalWireInput`, `OriginalWireReceipt`, `OriginalOperationPhysicalSpent`

## 🧵️turn-scheduler/🟦️.ts

Runtime: `TurnScheduler`

Types: `QueuedTurn`, `TurnBudgetProvider`, `TurnSchedulerOptions`

Planned defining package facets use explicit named first-party exports, corresponding closed schema/corpus ownership, and direct client adoption. Test fixture sources need an explicit testing facet or consumer fixture ownership; they will not be silently promoted to production API. A facet removal/declaration/package-export law must precede consumer changes, followed by the actual defining source suite and original whole direction receiver. No compatibility aliases or legacy alternative imports will be introduced.

## Bounded Facet Implementation

The first closed public API corpus covers only three bounded codecs and the existing scheduler root. It does not promote the whole large wire helper or transport implementation to public API. New first-party facets publish exact page/lifetime/patch runtime and type names plus their same defining schemas; the root scheduler wildcard is replaced with its existing exact four-name runtime/type roster. The package exports are explicit. The owner’s canonical source command now includes the new API law in both strict and portable phases.

The actual law `actor-public-facets-current1.log` is terminal genuine red: the authored package has no ./page export. Facets were implemented only after this observation. The law independently validates the closed corpus with Ajv, parses exact named export/type modifiers with the TypeScript compiler, compares the unique rosters through SQLite, and loads actual defining facet modules. Native pathToFileURL supplies portable module addresses; there is no custom file-URL constructor. The actual second execution is pending. Consumer imports have not yet changed.

## Current Defining Public API and Consumer Adoption

The four named facets are implemented and package-resolved. The original second individual law passed before this continuation; its generated log was reclaimed externally. This continuation therefore requalified current bytes rather than treating a missing handle or removed output as success. `actor-public-facets-strict2.log` was a genuine strict red because the fixture did not provide the existing in-source Vitest ImportMeta declaration. The permanent defining source command now explicitly supplies `vitest/importMeta` and ESNext/DOM in its compile phase; no production types or facade definitions were weakened.

The current actual permanent source receiver `actor-public-api-source-current2.log` terminated successfully in17.6s: strict types; all104 actual Vitest behavior laws; seven Bun schema/typegen/owner/public-facet laws with4260 assertions. The API law now independently checks explicit compiler declaration rosters, each exact defining implementation or schema reexport, package-manifest resolution through the real workspace package, and actual loaded runtime object identity. Ajv and SQLite remain the independent corpus/roster oracles. No wildcard, default compatibility alias, foreign library type, or duplicate schema owner was introduced.

The General product-denial law now covers13 required source owners and61 actual discovered TypeScript files; its actual production bundler follows83 modules and builds all seven defining/public entries while refusing product resolution. The controlled negative probe still has to fail at the exact forbidden path. This is current TypeScript product-independence evidence, not whole native architecture closure.

The current compiler inventory found61 private implementation/schema references in22 clients. They were all adopted directly to the three first-party facets. All14 schema imports now select their named defining schema while retaining their original local binding. Curated AST span changes preserved surrounding current source and checked for concurrent changes before writing. A second full current literal scan of framework, S, Hub, Mit, teaching, Semio Tech, Leutwiler, tests and schemas found zero remaining external relative references to those six bounded private implementation/schema files. Nonliteral computed imports and the remaining Actor transport/resource/wire APIs require the whole architecture receiver.

### Actual Client Roster

- ✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/🟦️.ts (2 imports)
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts (2 imports)
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx (11 imports)
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧺️turn-patch-batch/🟦️.ts (2 imports)
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx (2 imports)
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🧪️tests/🧪️typedwire/🟦️.tsx (8 imports)
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts (2 imports)
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📤️return/🟦️.ts (1 imports)
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📤️return/🧪️tests/🧪️pluginreturnwit-maps-every-canonical-drive-to-the-exact-wit-nesting-and/🟦️.ts (3 imports)
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts (2 imports)
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🧪️tests/🧪️browser-actor-patch-handoff-validates-the-neutral-schema-and-exact-owner/🟦️.ts (1 imports)
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️shardclient-reserved-response-settlement/🟦️.ts (3 imports)
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️actorreturnresponseframing-uses-canonical-vectors-with-no-payload-copies/🟦️.ts (8 imports)
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🎭️actor-transport/🟦️.ts (1 imports)
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts (2 imports)
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🧪️tests/🧪️cold-pair-loading/🟦️.ts (1 imports)
- 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🩹️operations/📥️wire/📃️pages/🟦️.ts (1 imports)
- 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/💾️resident/🟦️.ts (1 imports)
- 🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🏘️instance/🟦️.ts (2 imports)
- 🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🟦️.ts (1 imports)
- 🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/🧪️tests/🧪️kernelreturncontentframing-matches-the-shared-stream-and-independent-fra/🟦️.ts (3 imports)
- 🧰️framework/🔨️modules/🎠️kernel/📤️return/📦️content/📥️input/🟦️.ts (2 imports)

### Current Whole-Goal Limits

The first current downstream Kernel permanent receiver correctly refused an absent explicit Vitest policy before collecting any laws (`actor-facet-kernel-client-current1.log`); it is not a behavior pass. Its next execution supplies the actual repository policy factory for the real Kernel owner, preserves the original factory budget, and redirects only cache/coverage output into this ticket. The exact caller metadata is retained in `actor-facet-kernel-caller2.json`. The actual Kernel framing oracle and fresh whole neutral architecture receiver are in progress; their terminal results will be recorded below. The complete clean-architecture goal remains active.

## Current Downstream and Whole Neutral Receipts

The actual permanent Kernel receiver `actor-facet-kernel-client-current2.log` terminated GREEN9.5s: five selected real KernelReturnContentFraming laws,77unselected laws explicitly skipped. The selected laws include the actual three named Actor schema facets and independent WebAssembly LEB128 frame oracle. This is a five-law receipt, not a whole Kernel test pass. Its actual original repository Vitest factory policy was supplied after the first missing-policy refusal; only output storage was redirected.

The fresh actual permanent whole neutral dependency receiver `actor-facet-neutral-graph-current1.log` terminated correctly RED85s:1319 inventoried source files,1570strict rules,7081 ownership patterns;2494 modules,7749 dependencies;1090rule-qualified findings across709 distinct source-target pairs. `framework-no-products` has zero findings. The Actor-target subset is30findings/15distinct pairs, compared with the previous29paired targets. None of the bounded page/lifetime/patch/schema clients remains in that subset. This whole capture includes concurrent edits and is not an isolated benchmark; it proves the broader architecture remains incomplete. Remaining Actor targets are shard-client types/runtime, wire-turn helper clients, bootstrap test supply, original retained-turn source, component codec/liveness materialization, and one private scheduler consumer. The scheduler already has a closed defining root facet and is being adopted directly as the final member of this bounded API cohort.

## Retained Turn Defining Public Contract

A fresh full workspace compiler inventory found four external original retained-turn consumers in renderer/browser materialization. The next closed facet adds the existing seven first-party runtime exports, eleven first-party types, and the single defining numeric JSON receipt schema as `originalRetainedReceiptSchema`. It will resolve ./retained-turn directly; no alternate wrapper or second source implementation is introduced. The exact API corpus/schema are authored before the missing-public-facet law runs. The wider transport inventory is74references/40files and includes private test dependency types and private nonexported implementation types, so publishing the entire transport module would conceal real API design work; it remains an explicit next frontier.

The retained-turn law first failed genuinely at the absent ./retained-turn manifest entry (83assertions). Only then were the explicit first-party facet, manifest export and four direct clients implemented. The product-denial corpus now requires14owners/eightactualproductionentries. The existing scheduler root was also adopted by the one current Kernel root consumer.

The first full retained source gate passed its strict phase but failed a test tool path resolved under the Actor package node_modules. A fresh actual workspace-root/router probe returned the repository root; the capture does not prove the earlier root selection cause. The attempted bare vitest/vitest.mjs resolver also failed because it is not a published package subpath; that failed invocation is retained as current2 and is not accepted. The final source command resolves the actual installed vitest/package.json export, validates its declared bin.vitest string, and executes that bin path. Both original finite phase budgets and invocation remain unchanged. The actual manifest probe resolves the root installed vitest.mjs. Current3 must still terminate successfully before acceptance.

Actual downstream renderer original-source, emitted browser worker lifetime and materializer bootstrap receiver: sixlaws/79assertionsGREEN741ms (`actor-retained-client-current1.log`). The worker fixture really starts/terminates its emitted Node worker and preserves two genuine root admissions/fourclose receipts across stale disposal. Renderer native-scalar behavior here uses a declared TypeScript fixture, not GPU/native or heap qualification. No consumer was redirected to a copied source implementation.

## Current Retained Public API Acceptance

The actual permanent defining source receiver `actor-retained-api-source-current3.log` is terminal GREEN9.7s after the manifest-declared executable correction: strict compile;104actualVitestlaws;7Bunlaws/4293assertions. The current actual product-denial receiver requires14ownerfiles, discovers62actualActorTypeScriptfiles, follows85modules and builds all eight defining/publicentries. The five closed facets package-resolve to their exact defining files; all runtime values match the same implementation object identity. No original grant, source identity, clock or receipt was recreated by publishing this interface.

Retained source clients adopted:

- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🎟️original/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎞️frame-worker/🎟️original/🧪️tests/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧪️tests/🚪️lifetime/🟦️.ts

The current whole neutral capture predates only the final scheduler/retained-source adoption, so its1090/709complete-graph counts remain the last actual whole receiver result and must not be extrapolated into a whole pass. These final adoptions are independently source/runtime qualified, but the entire transport/wire boundary, native producer frontier and other architecture findings remain active work.

The final independent TypeScript compiler literal inventory inspected11189 externalTS/TSX/MTSfiles and found zero relative references to all eight bounded private sources/schema files covered by the fivefacets. ThreeunrelatedSfiles have existing parser diagnostics (Puzzle5dDiff422, Puzzle3dDiff482, Layoutdocumentlaw1), so this is not a full syntax/type pass. A separate raw reference check on those threefiles also found no bounded-facet references. Computed imports remain outside that literal claim. A new complete neutral receiver is running after final scheduler/retained source adoption; its result will be reported without extrapolating prior counts.

## Final Current Whole Neutral Receiver

The final actual permanent whole neutral receiver `actor-public-final-graph-current1.log` terminated correctly RED66s after all fivefacet adoptions. Actual graph: 2501modules/7771dependencies;1091rule-qualified findings/710distinct source-targetpairs;framework-no-products=0. Actor-target findings=26/13pairs. Remaining Actor targets:

- 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts
- 🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts
- 🧰️framework/🔨️modules/🎭️actor/🎟️retained-turn/🏛️bootstrap/🧪️tests/🧫️fixtures/🟦️.ts
- 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧬️component-codec/🟦️.ts
- 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🫀️liveness/🟦️.ts

The bounded page/lifetime/patch/scheduler/retainedSource facets have no remaining private consumer targets in that complete capture. Those claims apply to this actual graph; no complete-goal claim follows. Future transport/wire/test supply and all other architecture findings remain required work.
