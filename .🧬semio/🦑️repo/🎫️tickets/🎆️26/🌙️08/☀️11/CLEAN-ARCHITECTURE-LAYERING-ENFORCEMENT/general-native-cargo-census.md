# General Native Task Cargo Product Census

Read-only static census of `🧰️framework/🔨️modules/**/📜️script.ts`; excludes Specific products, ticket/generated contents, and the concurrently edited Actor Rust script from other-module totals. No scripts imported or executed; no tests or native commands run. Source content is observed concurrently and can change after this census. SHA below means SHA-256 of each observed source, not a Git revision. No Git command was used.

Scanned 105 scripts. Product import references: 62 files including Actor; **61 other files**, **90 lexical import declarations**, **105 named import bindings**. Counts describe syntax, not runtime instances or users.

## Actual API Groups

| Product export | Other file/import-binding count | Required neutral request boundary |
|---|---:|---|
| `buildRepositoryCargoArtifacts` | 3 | Artifact build/cache orchestration remains product-owned; inject build endpoint returning neutral receipt. |
| `configuredRepositoryCargoPreparationPolicyV1` | 11 | Product preparation default belongs to publisher; inject resolved preparation policy. |
| `repositoryCargoNetworkV1` | 3 | Repository network policy belongs to publisher; inject explicit resolved network capability. |
| `runRepositoryCargoCommandV1` | 13 | Closed command request with argv/cwd/env/budget and original invocation; preserve cancellation/deadline. |
| `runRepositoryCargoLintV1` | 4 | Lint request + policy + original invocation; same admission family. |
| `runRepositoryCargoTestsV1` | 60 | Standard Cargo test request + policy + original invocation; caller supplies repository admission endpoint. |
| `runRepositoryExactCargoLaws` | 11 | Exact law request with manifest map, target/groups/laws, preparation, policy, progress; endpoint supplied by publisher. |

## Source Classes and Registered Commands

Each row records actual class ownership and router registrations from lexical source. A dash indicates a call outside a class span. Line references locate the call. Classes without a product API call are omitted.

| Source | SHA-256 | Imports | Affected class → registered command → API call lines |
|---|---|---:|---|
| `🧰️framework/🔨️modules/⏪️time-travel/📦️packages/🦀️rust/📜️script.ts` | `8690a7dd4b894b4c7e6add80b1a5beb28a48bba2205bcf19471ccf51c7ba852c` | 2 | `CheckScript` → `check` → `runRepositoryCargoCommandV1` L29<br>`CheckScript` → `check` → `runRepositoryCargoCommandV1` L30<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L23 |
| `🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/📜️script.ts` | `49aacaa70c54a50225450f420f6b5a2fafc67397c5e4bd18d5a58a8d418b1d95` | 2 | `CheckScript` → `check` → `runRepositoryCargoCommandV1` L104<br>`CheckScript` → `check` → `runRepositoryCargoCommandV1` L105<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L37<br>`OriginalProgressTestScript` → `test-original-progress` → `runRepositoryCargoTestsV1` L47<br>`OriginalEntitiesTestScript` → `test-original-entities` → `runRepositoryCargoTestsV1` L57<br>`OriginalReceivingLimitsTestScript` → `test-original-receiving-limits` → `runRepositoryCargoTestsV1` L64<br>`OriginalIdentityReceivingTestScript` → `test-original-identity-receiving` → `runRepositoryCargoTestsV1` L71<br>`OriginalStepReceivingTestScript` → `test-original-step-receiving` → `runRepositoryCargoTestsV1` L78<br>`OriginalNativeTickTestScript` → `test-original-native-tick` → `runRepositoryCargoTestsV1` L85<br>`OriginalEntityColumnTestScript` → `test-original-entity-column` → `runRepositoryCargoTestsV1` L92<br>`OriginalProgressReceivingTestScript` → `test-original-progress-receiving` → `runRepositoryCargoTestsV1` L99 |
| `🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust/📜️script.ts` | `c01ba534f3f5ab1cf0d22d6dc29af3d06a5120c3ec41cc9573fc818ea32e0b8d` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L15 |
| `🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust/📜️script.ts` | `d31976b2e52111618fca5c2aa565594bf0b48f2c4f13d40b04c390c2753dbfd9` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L15 |
| `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/📜️script.ts` | `f066b43ba17e25ae28ab9cc1c8c86fc6f9e269c29bfb676d2ebea3e97d9f0fc7` | 3 | `WorkerMaintenanceCheckScript` → `worker-maintenance-check` → `runRepositoryExactCargoLaws` L62<br>`WorkerParkingCheckScript` → `worker-parking-check` → `runRepositoryExactCargoLaws` L99<br>`WorkerDeferredWakeCheckScript` → `worker-deferred-wake-check` → `runRepositoryExactCargoLaws` L144<br>`WorkerPoolUseCheckScript` → `worker-pool-use-check` → `runRepositoryExactCargoLaws` L193<br>`CancelReturnScript` → `test-cancel-return` → `runRepositoryExactCargoLaws` L285<br>`WorkerMaintenanceCheckScript` → `worker-maintenance-check` → `configuredRepositoryCargoPreparationPolicyV1` L62<br>`WorkerParkingCheckScript` → `worker-parking-check` → `configuredRepositoryCargoPreparationPolicyV1` L99<br>`WorkerDeferredWakeCheckScript` → `worker-deferred-wake-check` → `configuredRepositoryCargoPreparationPolicyV1` L144<br>`WorkerPoolUseCheckScript` → `worker-pool-use-check` → `configuredRepositoryCargoPreparationPolicyV1` L193<br>`CancelReturnScript` → `test-cancel-return` → `configuredRepositoryCargoPreparationPolicyV1` L285<br>`CheckScript` → `check` → `runRepositoryCargoCommandV1` L205<br>`PublicationDocsTestScript` → `test-publication-docs` → `runRepositoryCargoCommandV1` L217<br>`TestScript` → `test` → `runRepositoryCargoCommandV1` L239<br>`PreviewGeneratedScript` → `preview-generated` → `runRepositoryCargoCommandV1` L260<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L224 |
| `🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust/📜️script.ts` | `5b50a9a443d11cca07b3df9bd88695d7d18484b64192352dbffa3829246ab730` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L17 |
| `🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust/📜️script.ts` | `8e23bf4d98825c675870c8c74923e0391a7e19c9a91e5b0893e2cc541503ebe0` | 1 | `NativeScript` → `test-native` → `runRepositoryCargoTestsV1` L17<br>`OriginalDomainNative` → `test-original-domain-native` → `runRepositoryCargoTestsV1` L52 |
| `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/📜️script.ts` | `68e5283731c810ea9583d8dc61a3062eff9116394d45af9f5c5535b5113fc54e` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L38 |
| `🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/📜️script.ts` | `15e603584b37f4e69f8e82b62d623fde338f814b732ebe126ffd9b1d6b1b1da9` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L15 |
| `🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/📜️script.ts` | `775cfaa7d104de8c231430892747dcc1140e762d6d699d27e1208163e98fc2cf` | 2 | `CheckWasmScript` → `check-wasm` → `runRepositoryCargoCommandV1` L26<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L17 |
| `🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/📜️script.ts` | `16de09fd2d16f91cfca521408fd92208b1e3f89b933a64f6aac81c6e6d9abfa0` | 1 | `NativeRecipientTestScript` → `test-native-recipient` → `runRepositoryCargoTestsV1` L26<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L38<br>`ControlledValueTestScript` → `test-controlled-construction` → `runRepositoryCargoTestsV1` L64<br>`ControlledEncodingTestScript` → `test-controlled-encoding` → `runRepositoryCargoTestsV1` L69 |
| `🧰️framework/🔨️modules/🎒️pack/⚠️error/📦️packages/🦀️rust/📜️script.ts` | `8819e62358f6f2d303eba2156d88b1a6c9207ed6a642b30c702cb9235ae26b1d` | 1 | `NativeScript` → `test-native` → `runRepositoryCargoTestsV1` L25 |
| `🧰️framework/🔨️modules/🎒️pack/🌱️value/📦️packages/🦀️rust/📜️script.ts` | `a27747fa920c809debe0fd0c1b289e31961b1f0332a4fff030c8b9e205e2795d` | 1 | `NativeScript` → `test-native` → `runRepositoryCargoTestsV1` L36 |
| `🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/📜️script.ts` | `281753dfcc9416b6cf5c747813ae6de70c5fa70460efc3b5da07574d4ebc813f` | 4 | `TestScript` → `test` → `runRepositoryExactCargoLaws` L30<br>`TestScript` → `test` → `configuredRepositoryCargoPreparationPolicyV1` L30<br>`BuildScript` → `build` → `repositoryCargoNetworkV1` L95<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L37<br>`AbsoluteVarintNativeScript` → `test-absolute-varint-native` → `runRepositoryCargoTestsV1` L55<br>`BorrowedPreflightNativeScript` → `test-borrowed-preflight-native` → `runRepositoryCargoTestsV1` L73<br>`SchemaHashNativeScript` → `test-schema-hash-native` → `runRepositoryCargoTestsV1` L81<br>`SchemaStorageNativeScript` → `test-schema-storage-native` → `runRepositoryCargoTestsV1` L89<br>`OriginalStorageScript` → `test-original-storage` → `runRepositoryCargoTestsV1` L106<br>`RetainedIntrinsicNativeScript` → `test-retained-intrinsic-native` → `runRepositoryCargoTestsV1` L114<br>`IntrinsicSchemaNativeScript` → `test-intrinsic-schema-native` → `runRepositoryCargoTestsV1` L122<br>`OriginalRecordReceivingScript` → `test-original-record-receiving` → `runRepositoryCargoTestsV1` L166<br>`BuildScript` → `build` → `buildRepositoryCargoArtifacts` L95 |
| `🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/📜️script.ts` | `ccdddb96981bd1b5686580509fe78c55b59d1b88851af4d5e3bff0b3b641da00` | 2 | `NativeScript` → `test-native` → `runRepositoryExactCargoLaws` L56<br>`NativeScript` → `test-native` → `runRepositoryExactCargoLaws` L59<br>`NativeScript` → `test-native` → `configuredRepositoryCargoPreparationPolicyV1` L56<br>`NativeScript` → `test-native` → `configuredRepositoryCargoPreparationPolicyV1` L59<br>`NativeScript` → `test-native` → `runRepositoryCargoTestsV1` L62 |
| `🧰️framework/🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust/📜️script.ts` | `bf9ab24bfa2717bb697dd8dc7ee63e1a9930837d8b4b33f81f23e25cd1e7ba08` | 1 | `Native` → `test-native` → `runRepositoryCargoTestsV1` L11 |
| `🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust/📜️script.ts` | `2ed3d2425e09c7ec917ec603b27d0542df314471e454b13244fce164f884e679` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L18 |
| `🧰️framework/🔨️modules/📚️compiler/📦️packages/🦀️rust/📜️script.ts` | `1a94ac84cfa13f0617c8a0a521bd1be6764fc278113afda0b78fa7cc37a37034` | 1 | `NativeScript` → `test-native` → `runRepositoryCargoTestsV1` L13 |
| `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts` | `c33de67c81f498aa68f6764f6dad42268bdad9ee51d37f204dd46ef1e582b862` | 4 | `RetainedVerificationScript` → `retained-verification-check` → `runRepositoryExactCargoLaws` L299<br>`RetainedRecordObservationScript` → `retained-record-observation-check` → `runRepositoryExactCargoLaws` L343<br>`PresencePeerCodecScript` → `presence-peer-codec-check` → `runRepositoryExactCargoLaws` L388<br>`RetainedVerificationScript` → `retained-verification-check` → `configuredRepositoryCargoPreparationPolicyV1` L299<br>`RetainedRecordObservationScript` → `retained-record-observation-check` → `configuredRepositoryCargoPreparationPolicyV1` L343<br>`PresencePeerCodecScript` → `presence-peer-codec-check` → `configuredRepositoryCargoPreparationPolicyV1` L388<br>`BuildScript` → `build` → `repositoryCargoNetworkV1` L35<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L29<br>`LocalInteractionNativeTestScript` → `test-local-interaction-native` → `runRepositoryCargoTestsV1` L61<br>`CommandIngressNativeScript` → `test-command-ingress-native` → `runRepositoryCargoTestsV1` L407<br>`BuildScript` → `build` → `buildRepositoryCargoArtifacts` L35 |
| `🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust/📜️script.ts` | `d6d20da94528eb3bdad376bc3d2610566d30609c412f5322c45a49a09d63ed69` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L15 |
| `🧰️framework/🔨️modules/🔄️machine/✨️derive/📦️packages/🦀️rust/📜️script.ts` | `6e61fbca51acbde350c78cfe2809c88c2a06dc6f1641640904bb773ee03ae8f5` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L15 |
| `🧰️framework/🔨️modules/🔄️machine/📦️packages/🦀️rust/📜️script.ts` | `64615554c950f4380900a457a8b22320ac8b73e32e001bb8da95d5bd26f04a82` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L15 |
| `🧰️framework/🔨️modules/🔤️typeset/📦️packages/🦀️rust/📜️script.ts` | `81f1e35636184db7b1066165477a788246970de061cc30d609c93f393ced033a` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L15<br>`LintScript` → `lint` → `runRepositoryCargoLintV1` L22 |
| `🧰️framework/🔨️modules/🔲️pixels/📦️packages/🦀️rust/📜️script.ts` | `3daa0e65a753052bff99c78ca44ca1491bcf75db8f358c62e794becfce08ab37` | 2 | `ImageFundingScript` → `test-image-funding` → `runRepositoryExactCargoLaws` L17<br>`ImageFundingScript` → `test-image-funding` → `configuredRepositoryCargoPreparationPolicyV1` L17<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L22 |
| `🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/📦️packages/🦀️rust/📜️script.ts` | `3909ecf6a0674b62c66e3532f29b3ad4f1bb0214945b29a2101e5cced19d044f` | 2 | `CheckScript` → `check` → `runRepositoryCargoCommandV1` L29<br>`CheckScript` → `check` → `runRepositoryCargoCommandV1` L30<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L23 |
| `🧰️framework/🔨️modules/🖌️raster/📦️packages/🦀️rust/📜️script.ts` | `2865e10c509532ddac3df8be291311ca22e8018f57b102a4e4c251e7b0d81f20` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L15<br>`LintScript` → `lint` → `runRepositoryCargoLintV1` L22 |
| `🧰️framework/🔨️modules/🖱️ui/🌐️locale/📦️packages/🦀️rust/📜️script.ts` | `d7d8609f642ff9e916c94386ef8b08c5e5ea1aa3a082a5bb9219f07353d2e8b4` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L16<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L20 |
| `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/📜️script.ts` | `035e6b9747d308c19c11d1ad94e9c6910a7b3e6880bcc21891959b775c9bfab8` | 2 | `CheckWasmScript` → `check-wasm` → `runRepositoryCargoCommandV1` L37<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L28 |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts` | `90591579cd5a82f53fc75016166aedeb58a81d25f78f759c6bf6026273c348b4` | 2 | `CheckWasmScript` → `wasm` → `runRepositoryCargoCommandV1` L107<br>`CheckWgpuEngineWasmScript` → `wasm` → `runRepositoryCargoCommandV1` L116<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L22<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L27<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L72<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L88<br>`TestWgpuEngineScript` → `wgpu-engine` → `runRepositoryCargoTestsV1` L101 |
| `🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust/📜️script.ts` | `edc34b17a6736b6ccb8585a0164c12c71d513f24e7338c9d9caaddc76e4e5aa1` | 2 | `CheckWasmScript` → `check-wasm` → `runRepositoryCargoCommandV1` L46<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L37 |
| `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📜️script.ts` | `3e7a93780eb0fb78ab7039b35e1d350b325f29689792a8c3df314a035ad969d2` | 2 | `CheckScript` → `check` → `runRepositoryCargoCommandV1` L40<br>`CheckWasmScript` → `check-wasm` → `runRepositoryCargoCommandV1` L48<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L32 |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/📜️script.ts` | `5dbaaf1ab08e3ae1ceed3a880f9b5efd9efbad13cc752de38b24709fa6c5c4f1` | 3 | `TreeRetirementScript` → `tree-retirement-check` → `runRepositoryExactCargoLaws` L35<br>`NativeResourceScript` → `test-native-resource-custody` → `runRepositoryExactCargoLaws` L53<br>`TreeRetirementScript` → `tree-retirement-check` → `configuredRepositoryCargoPreparationPolicyV1` L35<br>`NativeResourceScript` → `test-native-resource-custody` → `configuredRepositoryCargoPreparationPolicyV1` L53<br>`CheckWasmScript` → `check-wasm` → `runRepositoryCargoCommandV1` L70<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L61 |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts` | `690128dac5494b6afd415e66addf1fc3bc1b59894ecf9a76eb0e76c7084b185f` | 3 | `BuiltTreeRetirementScript` → `built-tree-retirement-check` → `runRepositoryExactCargoLaws` L45<br>`BuiltTreeRetirementScript` → `built-tree-retirement-check` → `configuredRepositoryCargoPreparationPolicyV1` L45<br>`CheckWasmScript` → `check-wasm` → `runRepositoryCargoCommandV1` L90<br>`CheckWasmScript` → `check-wasm` → `runRepositoryCargoCommandV1` L108<br>`PreviewGeneratedScript` → `preview-generated` → `runRepositoryCargoCommandV1` L128<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L64<br>`ConformanceScript` → `conformance` → `runRepositoryCargoTestsV1` L81 |
| `🧰️framework/🔨️modules/🖱️ui/🪟️viewport/📦️packages/🦀️rust/📜️script.ts` | `e73477d179b284a1229ffae449e11ea491212e5b036bdd8dbb44b7c4a76f4dc1` | 1 | `NativeScript` → `test-native` → `runRepositoryCargoTestsV1` L17 |
| `🧰️framework/🔨️modules/🖼️canvas/📦️packages/🦀️rust/📜️script.ts` | `b0f9e24c9bc95943ffe388217532c47552928b37b83fa838ebf5c5937b611ba3` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L13 |
| `🧰️framework/🔨️modules/🖼️canvas/🔤️fonts/📦️packages/🦀️rust/📜️script.ts` | `cd7fc85eb5a77f2fd009e09b6887f22fe63ef5c76d869dca6bebb3262c5d06af` | 2 | `BuildScript` → `build` → `repositoryCargoNetworkV1` L13<br>`BuildScript` → `build` → `buildRepositoryCargoArtifacts` L13 |
| `🧰️framework/🔨️modules/🗜️deflate/📦️packages/🦀️rust/📜️script.ts` | `63aa21f35a3fbbc31939072c07bcc640ec977f28efb027142b32e05c82e04dd6` | 1 | `NativeScript` → `test-native` → `runRepositoryCargoTestsV1` L25 |
| `🧰️framework/🔨️modules/🗣️dsl/📦️packages/🦀️rust/📜️script.ts` | `cc3f82f8eaebedf32875fce53e84b5196349d1db61dfc89c6138fab952b47ad6` | 1 | `NativeScript` → `test-native` → `runRepositoryCargoTestsV1` L39 |
| `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust/📜️script.ts` | `6c2e08c941b41a7c77f5659b58ca5a07544d5ed2e2b233bd19f4f2f288a1d939` | 2 | `CustodyNativeScript` → `test-custody-native` → `runRepositoryExactCargoLaws` L36<br>`CustodyNativeScript` → `test-custody-native` → `configuredRepositoryCargoPreparationPolicyV1` L36<br>`NativeScript` → `test-native` → `runRepositoryCargoTestsV1` L27 |
| `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust/📜️script.ts` | `1ca2820178faa1e21b5f4a5a4f19f62f976798bb84e2c9c0642b5b08fc4bde9a` | 1 | `NativeScript` → `test-native` → `runRepositoryCargoTestsV1` L25<br>`OriginalReceivingScript` → `test-original-receiving` → `runRepositoryCargoTestsV1` L37 |
| `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📜️script.ts` | `b8929f54572cb5f7d3a948683d3f390519ec16fbbaf06c49c6dcaa852459df78` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L42<br>`SceneWireNativeScript` → `test-scene-wire-native` → `runRepositoryCargoTestsV1` L59 |
| `🧰️framework/🔨️modules/🗿️artifact-reference/📦️packages/🦀️rust/📜️script.ts` | `f52714541eefd8191ea5b1ed357b4f54d700857abbdf79ddb0191031b09a7ba5` | 1 | `NativeScript` → `test-native` → `runRepositoryCargoTestsV1` L17 |
| `🧰️framework/🔨️modules/🚪️io/⏱️control/📦️packages/🦀️rust/📜️script.ts` | `a1e1abac38ed0f3907d9f3fd4cba7e5c19bb8a47a06fb10eed176b175479a6c3` | 1 | `NativeScript` → `test-native` → `runRepositoryCargoTestsV1` L24<br>`ReceivingScript` → `test-snapshot-receiving-native` → `runRepositoryCargoTestsV1` L31 |
| `🧰️framework/🔨️modules/🚪️io/📦️codec/📦️packages/🦀️rust/📜️script.ts` | `a149e29a67fa5e8d144d0da9c225eabd22c2b21ed23b95af8d8efad102b3c047` | 1 | `NativeScript` → `test-native` → `runRepositoryCargoTestsV1` L10 |
| `🧰️framework/🔨️modules/🚪️io/🔢️binary/📥️source/📜️script.ts` | `c72eae0bc0434a79c01e06a2d540ed6a72e1b70cb21923fcdee498a7734fb216` | 2 | `TestScript` → `test` → `runRepositoryExactCargoLaws` L22<br>`TestScript` → `test` → `configuredRepositoryCargoPreparationPolicyV1` L22<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L24 |
| `🧰️framework/🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust/📜️script.ts` | `fe0a23bbe83f4b891bcf40429f0d29750f17a988a92296451d211a40a229ce87` | 1 | `NativeScript` → `test-native` → `runRepositoryCargoTestsV1` L17 |
| `🧰️framework/🔨️modules/🚪️io/🧬️schema/📦️packages/🦀️rust/📜️script.ts` | `7e23164f4688bbfab99ef6e4984c5b25fd69ca996fa3b0e564fc74a79001e9f4` | 1 | `NativeScript` → `test-native` → `runRepositoryCargoTestsV1` L27 |
| `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🏛️model/📦️packages/🦀️rust/📜️script.ts` | `601e7b4ec1cdab3436934707f58c9fe5804aff4788038690dce8681e04d7b54d` | 1 |  |
| `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust/📜️script.ts` | `9c5c5fa670b50bd762b7f64594ee4848c4d1a3767876ac92b84aaca515059cc4` | 1 | `NativeScript` → `test-native` → `runRepositoryCargoTestsV1` L27<br>`ReceivingSchemaScript` → `test-receiving-schema` → `runRepositoryCargoTestsV1` L37<br>`OriginalFixedLimitsScript` → `test-original-fixed-limits` → `runRepositoryCargoTestsV1` L68<br>`OriginalPhysicalScript` → `test-original-physical` → `runRepositoryCargoTestsV1` L80 |
| `🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust/📜️script.ts` | `5158ff355ee99ac67f139228e8a2854db08f525f1445793576c398026770ad79` | 2 | `CheckScript` → `check` → `runRepositoryCargoCommandV1` L29<br>`CheckScript` → `check` → `runRepositoryCargoCommandV1` L30<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L23 |
| `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/📜️script.ts` | `629d4a98a0edd369b5891012ba3de020254ec1be61043123c19f9e052df0f64d` | 3 | `TestScript` → `test` → `runRepositoryExactCargoLaws` L31<br>`CanonicalArchitectureScript` → `canonical-architecture` → `runRepositoryExactCargoLaws` L41<br>`TestScript` → `test` → `configuredRepositoryCargoPreparationPolicyV1` L31<br>`CanonicalArchitectureScript` → `canonical-architecture` → `configuredRepositoryCargoPreparationPolicyV1` L41<br>`BenchScript` → `bench` → `runRepositoryCargoCommandV1` L50<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L34<br>`LintScript` → `lint` → `runRepositoryCargoLintV1` L58 |
| `🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/📦️packages/🦀️rust/📜️script.ts` | `68a5ccb5d6cfff13acd229a7aa5278b5520ef59022a00154f48725ac5ce4fa64` | 1 | `—` → `—` → `runRepositoryCargoTestsV1` L12 |
| `🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🦀️rust/📜️script.ts` | `9d9b7432fb76a592d3acf3a3356c0dbe02acde4eb5fce0d616fbb11d5042df22` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L19 |
| `🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/📜️script.ts` | `82f9dab90636b8e7b495973b3e5f55d944ed3fca25c4a5dffad6e1fef6719e01` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L15 |
| `🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust/📜️script.ts` | `b98e657ae0a82affe66ef09c639ce5c1f568d79d952595818e8ced37a6f31f26` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L16 |
| `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts` | `a07debd6da8e5d1981255c668b6a0f11fa982c19cffce88b1aabfc1e30a86b14` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L19 |
| `🧰️framework/🔨️modules/🧬️schema/📶️state/📦️packages/🦀️rust/📜️script.ts` | `01ec39763eb0cbafa04c50e04afa0b97efeb69df41b658297403e3f6e474d141` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L15 |
| `🧰️framework/🔨️modules/🧬️schema/🧩️composition/📦️packages/🦀️rust/📜️script.ts` | `a15340d8aeca2500f747955bbb7a444af321161886eff0da71af43ce5130d5b7` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L15 |
| `🧰️framework/🔨️modules/🧮️expression/📦️packages/🦀️rust/📜️script.ts` | `384982d87790759282e6abbe31dc27c70494d2e24394257f3b31a41cecc45f95` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L18 |
| `🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/📜️script.ts` | `11f3a23577b2246235d771fa1e856114b5a017f8a1c30260ae02909485c5c6de` | 1 | `TestScript` → `test` → `runRepositoryCargoTestsV1` L17<br>`LintScript` → `lint` → `runRepositoryCargoLintV1` L24 |
| `🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/📜️script.ts` | `1bb3ef8a8730d42c777823775add362ac90faec171e11229877037d539639c33` | 2 | `TestScript` → `test` → `runRepositoryExactCargoLaws` L27<br>`CanonicalArchitectureScript` → `canonical-architecture` → `runRepositoryExactCargoLaws` L40<br>`OriginalTurnFailureScript` → `test-original-turn-failure` → `runRepositoryExactCargoLaws` L75<br>`OriginalCloseContextNativeScript` → `test-original-close-context-native` → `runRepositoryExactCargoLaws` L89<br>`OriginalRetainedLoanNativeScript` → `test-original-retained-loan-native` → `runRepositoryExactCargoLaws` L99<br>`TestScript` → `test` → `configuredRepositoryCargoPreparationPolicyV1` L27<br>`CanonicalArchitectureScript` → `canonical-architecture` → `configuredRepositoryCargoPreparationPolicyV1` L40<br>`OriginalTurnFailureScript` → `test-original-turn-failure` → `configuredRepositoryCargoPreparationPolicyV1` L75<br>`OriginalCloseContextNativeScript` → `test-original-close-context-native` → `configuredRepositoryCargoPreparationPolicyV1` L89<br>`OriginalRetainedLoanNativeScript` → `test-original-retained-loan-native` → `configuredRepositoryCargoPreparationPolicyV1` L99<br>`TestScript` → `test` → `runRepositoryCargoTestsV1` L32 |

## Alias and Dynamic Import Coverage

No `@repo` package specifiers found.
Dynamic import call sites across the scanned scripts: 47.
- `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️script.ts:22`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📜️script.ts:33`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📜️script.ts:46`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📜️script.ts:50`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📜️script.ts:65`
- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📜️script.ts:68`
- `🧰️framework/🔨️modules/🎒️pack/🌱️value/📜️script.ts:6`
- `🧰️framework/🔨️modules/🎒️pack/🌱️value/📜️script.ts:6`
- `🧰️framework/🔨️modules/🎒️pack/🌱️value/📜️script.ts:7`
- `🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/📦️packages/🦀️rust/📜️script.ts:10`
- `🧰️framework/🔨️modules/🧪️test/🎮️mutation/🏭️inventory/📦️packages/🦀️rust/📜️script.ts:11`
- `🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/📜️script.ts:38`
- `🧰️framework/🔨️modules/🏃️process/📜️script.ts:28`
- `🧰️framework/🔨️modules/🏃️process/📜️script.ts:35`
- `🧰️framework/🔨️modules/🏃️process/📜️script.ts:37`
- `🧰️framework/🔨️modules/🏃️process/📜️script.ts:39`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:41`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:46`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:47`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:48`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:54`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:204`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:205`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:206`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:307`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:308`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:352`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts:358`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/📜️script.ts:19`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/📜️script.ts:24`
- `🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript/📜️script.ts:28`
- `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📜️script.ts:14`
- `🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🦀️rust/📜️script.ts:15`
- `🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🦀️rust/📜️script.ts:17`
- `🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🦀️rust/📜️script.ts:25`
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts:28`
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts:38`
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts:40`
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/📜️script.ts:41`
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📜️script.ts:21`
- `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/📜️script.ts:25`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts:48`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts:57`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts:66`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/📜️script.ts:44`
- `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/📜️script.ts:46`
- `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/📜️script.ts:46`

All counted product edges above are named static imports through relative paths, including local aliases such as `runCargoTestsV1`, `runCargoLintV1`, and `runExactCargoLaws`. Lexical matching does not prove module resolution: relative path correctness is outside this bounded census.

## Bounded Future Canonicalization Batch

1. Define General-owned schema-first request/receipt contracts for exact laws, standard tests/lint, closed Cargo commands, and artifact build. Keep repository preparation, network, cache publisher, and admission implementation in Specific Repo. Original invocation/control/progress must remain continuous across the injected endpoint; do not recreate unrelated process controls.
2. Migrate exact-law consumers plus preparation defaults as one family. Actor is actively being changed by the parent; do not edit it in this inventory. Other exact-law files appear in the table. Their selected law names, manifests, native targets, cargo arguments and progress receipts remain authored at each owning module.
3. Migrate standard tests/lint consumers using the same publisher-injected admission shape. Preserve class/command selection and native test policy; do not turn the general process module into repository orchestration.
4. Migrate command consumers with an explicit closed schema (check/bench/build/test use cases and target flags); Wasm target selection is caller intent, not a new provider. The table and call line distinguish commands; inspect argv at those source lines during implementation.
5. Migrate the three artifact-build/network consumers (Pack, Replication, Canvas Fonts) as a distinct endpoint family. Keep cache preparation and network admission Specific. Do not emulate repository storage/cache in General.

The common seam is publisher injection: General owns task intent and cancellation/progress contract; Repo supplies actual Cargo admission/preparation/artifact orchestration. These are future implementation batches, not source changes or executed validation. No compatibility layer or migration script is proposed.

## Dynamic Import Review and Product Endpoint Digests

Reviewed all listed dynamic import expressions textually. None names Repo. Replication computes one local codec target from its own root; Process contains an import expression inside child command text whose argument comes from its local function-suite mapping. These are lexical expressions, not runtime loads. Actor dynamic imports are concurrent observations only. General Graph dynamically imports the General Cargo test helper and does not add a direct product import to this census.

The following digests identify the resolved imported endpoint content when present; missing paths are recorded as unresolved rather than assumed valid.

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts`: SHA-256 `d9be87ceeb58b7c45edfe51f3bf0c2ae7d4d497f81c1f4e407d20387f091d593`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎟️admission/🟦️.ts`: SHA-256 `0a70358e1611fde26a39a5d3cde6988dcb5f17a1d3f9c646842f7b24843f68d8`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🟦️.ts`: SHA-256 `ab86ffcbfad80afeab23717ff01da547c645ae6512a42742c4a59d5262d25fdb`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts`: SHA-256 `e6dd55bc381337d00f21368b2d27ec00741eb43c152a4d0d95c230ae423456fe`
