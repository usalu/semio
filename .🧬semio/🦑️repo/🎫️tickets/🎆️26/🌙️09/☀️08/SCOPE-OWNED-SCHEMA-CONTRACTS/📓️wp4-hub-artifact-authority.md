# WP4 — `hub.artifact-authority` scope-owned schema contracts

Three owner modules created, eight fixture-owned / misplaced schema documents eliminated, seven
prove-function regions rewired onto `hubSchemaExport`. Every fixture directory in this partition now
contains data only.

## 1. Scope → export table

### `hub.artifact-authority`
`🌎️hub/🗿️artifact-authority/🧬️schema/🔣️.json` — `$id` `https://semio.tech/schema/hub/artifact-authority/schema.json`

| Export id | Source fixture (wrapper it was extracted from) | Implementation it mirrors |
|---|---|---|
| `ArtifactCasManifestPlanV1` | `🧪️fixtures/🧱️artifact-chunk-cas/🧬️schema/🔣️.json` → `$defs.vector` | `ArtifactCasManifestPlan` / `ArtifactCasManifestV1`, `🌎️hub/🗿️artifact-authority/🧱️chunk-cas/🦀️.rs:47,57` |
| `ArtifactCasObjectKeyV1` | same, `$defs.objectToken` | `ArtifactCasObjectKey` + `ArtifactCasObjectKind`, `🧱️chunk-cas/🦀️.rs:65,96` |
| `ArtifactCasRetentionLedgerEventV1` | same, `$defs.ledgerEvent` | reserve/publish/retention/space-delete ledger rows folded by `DirectoryService::sweep_artifact_cas`, `🌎️hub/📇️directory/🦀️.rs` |
| `ArtifactCasSweepCursorV1` | same, `properties.sweepContinuation.expectedFirstCursor` | `ArtifactCasSweepPosition`, `🌎️hub/📇️directory/🦀️.rs:2066` |

Private helpers (lower-camel, not exports): `hash`, `safeInteger`, `chunkOrdinal`.

### `hub.artifact-authority.creation`
`🌎️hub/🗿️artifact-authority/🌱️creation/🧬️schema/🔣️.json` — `$id` `https://semio.tech/schema/hub/artifact-authority/creation/schema.json`

| Export id | Source fixture | Implementation it mirrors |
|---|---|---|
| `ArtifactCreationPhaseV1` | `📚️operation-v1/🛑️cancel.schema.json`, `🧫️fixtures/🌐️http-owner-v1/🧬️.schema.json` | `SpaceArtifactCreationPhaseV1` (os `📇️directory/🧬️schema/🌱️space-artifact-creation-v1`) |
| `ArtifactCreationFactKindV1` | `📚️operation-v1/🧬️.schema.json` (`fact`) | `ArtifactCreationFactBodyV1` tag, `🌱️creation/🧬️schema/🦀️.rs:56` |
| `ArtifactCreationOperationStateV1` | `📚️operation-v1/🧬️.schema.json` (`state`) | folded read state of `ArtifactCreationOperationV1::fold` plus the pre-history `absent` |
| `ArtifactCreationTransitionV1` | `📚️operation-v1/🧬️.schema.json` (root law) | `ArtifactCreationOperationV1::fold` legality, `🌱️creation/🧬️schema/🦀️.rs` |
| `ArtifactCreationCancellationDecisionV1` | `📚️operation-v1/🛑️cancel.schema.json` | `decide_artifact_creation_fact_append_v1`, `🌱️creation/🧬️schema/🦀️.rs` |
| `ArtifactCreationTransactionKindV1` | `🧪️transaction-v1/🧬️.schema.json` (`kind`) | genesis-transaction fault taxonomy (`DocumentGenesisCommitV1` call sites, `🌎️hub/📇️directory/🪶️sqlite/🌱️creation-v1/🦀️.rs`) |
| `ArtifactCreationTransactionOutcomeV1` | `🧪️transaction-v1/🧬️.schema.json` | `DocumentGenesisCommitV1`, `🌱️creation/🧬️schema/🦀️.rs:127` |
| `ArtifactCreationAcceptedRecoveryV1` | `🧪️transaction-v1/🧯️accepted-recovery.schema.json` | `genesis_accepted_only_recovery_has_no_prepared_or_public_side_effects` law, `🌎️hub/📇️directory/🪶️sqlite/🌱️creation-v1/🦀️.rs:156` |
| `ArtifactCreationHttpLimitsV1` | `🧫️fixtures/🌐️http-owner-v1/🧬️.schema.json` | `ARTIFACT_CREATION_DEADLINE_MS` + owner HTTP body/live-operation bounds |
| `ArtifactCreationHttpRouteV1` | same | `localRelayUpstreamPath` route surface (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`, `🚀️bin.rs` routes) |
| `ArtifactCreationHttpAuthorityV1` | same | author/spectator/member/session admission ladder in `🚀️bin.rs` |
| `ArtifactCreationHttpResponseV1` | same | phase → 202/200 mapping in `🚀️bin.rs` |

Private helpers: `safeInteger`, `identity`, `openTransitions`.

### `hub.artifact-authority.native-openable-provider`
`🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧬️schema/🔣️.json` — `$id` `https://semio.tech/schema/hub/artifact-authority/native-openable-provider/schema.json`

| Export id | Source fixture | Implementation it mirrors |
|---|---|---|
| `NativeArtifactProviderHeadlessProfileV1` | `🌎️hub/🧪️fixtures/🧭️native-artifact-provider-frontier-v1/🧬️.schema.json` (`headless`) | headless `semio-hub` Cargo profile (`🌎️hub/📦️packages/🦀️rust/Cargo.toml`) |
| `NativeCodecProviderSetIdentityV1` | same (`production`) | `NativeCodecProviderSetV1`, `NATIVE_OPENABLE_PROVIDER_SET_V1_ID/_RECEIPTS`, `📇️native-openable-provider/🦀️.rs:9,11,22` |
| `NativeCodecProviderSelectionCaseV1` | `🧪️fixtures/🌍️gis-v1/🧬️.schema.json`, `🧪️fixtures/🌿️vcs-v1/🧬️.schema.json` (`cases`) | `NativeCodecProviderSetV1::preview` identity + cancel/deadline fences, `📇️native-openable-provider/🦀️.rs:28` |
| `NativeCodecUnconsumedProfileV1` | `🧪️fixtures/🌿️vcs-v1/🧬️.schema.json` (`unconsumedProfiles`) | `linked_provider_set_previews_only_the_selected_packages_…` law, `📇️native-openable-provider/🦀️.rs:288` |
| `NativeOpenableAttestationV1` | `🧪️fixtures/🪪️v1/🧬️.schema.json` (`attestation`) | component + descriptor-projection SHA-256 attestation checked by `NativeOpenableCatalogProviderV1` |
| `NativeOpenableOpenTargetV1` | same (`$defs.openTarget`) | published open target of `NativeOpenableCatalogProviderV1::into_bindings`, `📇️native-openable-provider/🦀️.rs:127,138` |
| `NativeOpenableHostileExpectationV1` | same (`hostileCases`) | fixture-expectation shape (`stage`/`result`/`code`), consumed by `assertHubFixtureExpectation` |

Private helpers: `digest`, `hexBytes`, `identity`, `packageId`, `cargoFeature`, `linkedPackageId`.

## 2. Files created / moved / deleted

Created
- `🌎️hub/🗿️artifact-authority/🧬️schema/🔣️.json`
- `🌎️hub/🗿️artifact-authority/🌱️creation/🧬️schema/🔣️.json`
- `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧬️schema/🔣️.json`

Moved
- `🌱️creation/📚️operation-v1/🦀️.rs` → `🌱️creation/🧬️schema/🦀️.rs` (production reducer becomes the module's Rust facet)
- `🌱️creation/📚️operation-v1/🔣️.json` → `🌱️creation/🧫️fixtures/📚️operation-v1/🔣️.json`
- `🌱️creation/📚️operation-v1/🛑️cancel.json` → `🌱️creation/🧫️fixtures/📚️operation-v1/🛑️cancel.json`
- `🌱️creation/🧪️transaction-v1/🔣️.json` → `🌱️creation/🧫️fixtures/🧪️transaction-v1/🔣️.json`
- `🌱️creation/🧪️transaction-v1/🧯️accepted-recovery.json` → `🌱️creation/🧫️fixtures/🧪️transaction-v1/🧯️accepted-recovery.json`

Deleted (directories `📚️operation-v1/` and `🧪️transaction-v1/` no longer exist)
- `🗿️artifact-authority/🧪️fixtures/🧱️artifact-chunk-cas/🧬️schema/` (whole illegally nested module directory)
- `🌱️creation/📚️operation-v1/🧬️.schema.json`, `🌱️creation/📚️operation-v1/🛑️cancel.schema.json`
- `🌱️creation/🧪️transaction-v1/🧬️.schema.json`, `🌱️creation/🧪️transaction-v1/🧯️accepted-recovery.schema.json`
- `🌱️creation/🧫️fixtures/🌐️http-owner-v1/🧬️.schema.json`
- `📇️native-openable-provider/🧪️fixtures/🌍️gis-v1/🧬️.schema.json`
- `📇️native-openable-provider/🧪️fixtures/🌿️vcs-v1/🧬️.schema.json`
- `📇️native-openable-provider/🧪️fixtures/🪪️v1/🧬️.schema.json`
- `🌎️hub/🧪️fixtures/🧭️native-artifact-provider-frontier-v1/🧬️.schema.json`

Fixture data edited (expectations only, no schema)
- `📇️native-openable-provider/🧪️fixtures/🌍️gis-v1/🔣️.json`, `…/🌿️vcs-v1/🔣️.json` — each `cases[]` row gained
  `stage` + `code`; `accepted` is unchanged because Rust reads it (`📇️native-openable-provider/🦀️.rs:255`,
  `🔏️trusted-catalog/🦀️.rs:2230`).
- `📇️native-openable-provider/🧪️fixtures/🪪️v1/🔣️.json` — `hostileCases[].outcome: "denied"` replaced by
  `stage` / `result` / `code`; `publishedTargets: 0` kept (it is a separate no-partial-publication claim).
- `🗿️artifact-authority/🧪️fixtures/🧱️artifact-chunk-cas/🔣️.json`, `🌱️creation/🧫️fixtures/**`,
  `🌎️hub/🧪️fixtures/🧭️native-artifact-provider-frontier-v1/🔣️.json` — byte-identical, data only.

Reference fixes
- `🌱️creation/🦀️.rs` — `#[path = "📚️operation-v1/🦀️.rs"]` → `#[path = "🧬️schema/🦀️.rs"]` (module still named
  `operation`, so `artifact_authority::creation::operation::tests::*` law ids are unchanged).
- `🌱️creation/🧬️schema/🦀️.rs` — both `include_str!` now `../🧫️fixtures/📚️operation-v1/…`.
- `🌎️hub/📇️directory/🦀️.rs:4252`, `🌎️hub/📇️directory/🪶️sqlite/🌱️creation-v1/🦀️.rs:60,156,194,299`,
  `🌎️hub/📇️directory/🐘️postgres/🌱️creation-v1/🦀️.rs:79`, `🌎️hub/📇️directory/🌐️neo4j/🌱️creation-v1/🦀️.rs:79`
  — `include_str!` paths repointed at `🧫️fixtures/`.
- `🌎️hub/📦️packages/🟦️typescript/🤝️index.test.ts` — the artifact chunk-CAS case now binds
  `hubSchemaExport(root, "🌎️hub/🗿️artifact-authority/🧬️schema/🔣️.json")` (the helper a sibling worker added
  to that file) and asserts the envelope constants (`version`, `chunkBytes`, the four maxima,
  `largePair.totalLength`, barrier order ids) explicitly instead of by wrapper schema.

No `📋️project.json`, `package.json`, `.vscode/launch.json` or nx-input reference named any moved path
(`grep` over those file kinds returned nothing), and no command was added or renamed.

## 3. Prove-function rewiring

| Function (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`) | Change |
|---|---|
| `proveNativeArtifactProviderFrontier` | dropped the dynamic `Ajv2020` + `🧬️.schema.json` compile; validates `fixture.headless` / `fixture.production` against `NativeArtifactProviderHeadlessProfileV1` / `NativeCodecProviderSetIdentityV1`, then asserts the retired consts literally (`schema`, `configuredWithoutProvider`, `defaultFeatures`, `features == ["sqlite"]`, `directPluginDependencies.length === 0`, `feature`, `providerId`, `receiptCount === 29`, `pluginDependencies` array equality) |
| `proveGisNativeProviderSelectionFixture` | `NativeCodecProviderSelectionCaseV1` per row + `assertHubFixtureExpectation`; envelope asserted explicitly (`schema`, 8 cases, unique names) |
| `proveVcsNativeProviderSelectionFixture` | same, plus `NativeCodecUnconsumedProfileV1` for the two profiles; envelope (`schema`, 8 cases, 2 profiles, unique names) |
| `proveNativeOpenableCatalogProviderFixture` | fixture-local schema compile removed; `NativeOpenableOpenTargetV1` / `NativeOpenableAttestationV1` / `NativeOpenableHostileExpectationV1` bound from the module; envelope asserted (`schema`, `receiptCount === 26`, 13 unique mutations, projection/definition-root types); hostile loop drives `assertHubFixtureExpectation`, using the module export as the contract-stage witness for `wrong-surface` and the bijection oracle otherwise. The stdio-owned schema reads in this function were left alone (see §5). |
| `proveSpaceArtifactCreationContractV1` (http-owner block) | four scope exports (`ArtifactCreationHttpLimitsV1/RouteV1/AuthorityV1/ResponseV1`) plus explicit envelope + limit literals (`4096`, `8`, `30000`, exact row counts, id uniqueness) |
| `proveSpaceArtifactCreationContractV1` (transaction block) | `ArtifactCreationTransactionOutcomeV1` and `ArtifactCreationAcceptedRecoveryV1`; the `committed` / `facts+1` mutation ratchets are preserved because the exports carry the kind→outcome implications |
| `proveSpaceArtifactCreationContractV1` (operation block) | `ArtifactCreationCancellationDecisionV1` and `ArtifactCreationTransitionV1`; the `accepted` and `next` flip ratchets are preserved (`accepted ⟺ phase ≠ null`; legal-pair `oneOf` vs `next === null ∧ ¬legal-pair`) |

Fixture-count guards added where the deleted wrappers' `minItems`/`maxItems` used to hold the closure:
32 transaction kinds, 3 recovery kinds, 9 cancellation races, 30 transitions (6×5, unique `state/fact`),
10 routes / 6 authorities / 6 responses, 8 selection cases per plugin, 13 hostile mutations, 26 receipts.

## 4. Verification (real output)

### 4.1 Draft-07 `ajv` module load + fixture validation

`bun <scratchpad>/verify.ts` — loads each module with `new Ajv({strict:true,allErrors:true})` from the
`ajv` package (draft-07, not `ajv/dist/2020`), `addSchema`, `getSchema($id + "#/$defs/" + export)`, and
validates every rewired fixture member plus the mutation ratchets:

```
hub.artifact-authority => https://semio.tech/schema/hub/artifact-authority/schema.json
  exports: ArtifactCasObjectKeyV1, ArtifactCasManifestPlanV1, ArtifactCasRetentionLedgerEventV1, ArtifactCasSweepCursorV1
hub.artifact-authority.creation => https://semio.tech/schema/hub/artifact-authority/creation/schema.json
  exports: ArtifactCreationPhaseV1, ArtifactCreationFactKindV1, ArtifactCreationOperationStateV1, ArtifactCreationTransitionV1, ArtifactCreationCancellationDecisionV1, ArtifactCreationTransactionKindV1, ArtifactCreationTransactionOutcomeV1, ArtifactCreationAcceptedRecoveryV1, ArtifactCreationHttpLimitsV1, ArtifactCreationHttpRouteV1, ArtifactCreationHttpAuthorityV1, ArtifactCreationHttpResponseV1
hub.artifact-authority.native-openable-provider => https://semio.tech/schema/hub/artifact-authority/native-openable-provider/schema.json
  exports: NativeArtifactProviderHeadlessProfileV1, NativeCodecProviderSetIdentityV1, NativeCodecProviderSelectionCaseV1, NativeCodecUnconsumedProfileV1, NativeOpenableAttestationV1, NativeOpenableOpenTargetV1, NativeOpenableHostileExpectationV1
contract rejects wrong-surface: true
contract rejects cancelled-before-preview: true
checks=159 passed=159 mutations-admitted=0
```

### 4.2 `bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts space-artifact-creation-check source`

```
[DEBUG] space artifact creation contract: cases=34 raw-json=14 relay=10 authority=6 responses=6 AJV=1 scope-exports=4 TypeScript=1 Pack=1 ready-only-coordinate=1 ordinary-bridge=3; runtime genesis remains a separate gate
[DEBUG] document index fixture: ordered-client=10 AJV=1 independent-node-SHA256=10; backend transactions not executed
[DEBUG] required committed checkpoint: TypeScript/AJV plan+lease cases=6; protected HTTP endpoints not executed
[DEBUG] genesis publication current: TypeScript=10 AJV=1; no Directory lineage or backend transaction executed
[DEBUG] creation transaction oracle: scope-export=32; backend transactions not executed
[DEBUG] creation accepted recovery oracle: scope-export=3; real deadline/backend not executed
[DEBUG] creation cancellation oracle: scope-export=9; backend concurrency not executed
[DEBUG] creation operation: scope-export transitions=30 independent-node-SHA256=1; native reducer/backend not executed
[DEBUG] genesis frontier: TypeScript=8 AJV=1 open/lease=8 independent-node-SHA256=5; native factory/publication not executed
```

### 4.3 `bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts execution-target-relay-check`

```
hub-native-artifact-provider-frontier-oracle: scope-exports=2 headless=sqlite plugin-deps=0 production-receipts=29 configured-no-provider=reject
execution-target-provider-frontier: checks=12
execution-target relay runtime routes=21 responses=9 bounded-capacity=2 exact-bytes=true
execution-target-relay-check: checks=39
execution-target-relay-check: existing browser proof-ratchet runtime regression clean
```

### 4.4 `bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts native-openable-catalog-provider-check --oracle-only`

Run with `SEMIO_TEST_ARTIFACT_DIR` / `CARGO_TARGET_DIR` under this ticket's `🗑️generated`.

```
headless-stdio-metadata-capture-oracle: private-commands=2 replacement-stable=1 replacement-during-capture-denied=1 second-dependency-root-denied=1 outside-target-denied=1
vcs-native-codec-oracle: receipts=1 hostile=9 ajv+node+webcrypto=1 dependency-coherence=3; no catalog activation or VCS execution claim
vcs-native-provider-selection-oracle: cases=8 accepted=1 unconsumed-profiles=2 linked-receipts=29 scope-exports=2; no native or catalog activation claim
4746 |   const projectionSchema = JSON.parse(readFileSync(join(projectionPath, "../🧬️native-codec-factories.schema.json"), "utf8"));
                                             ^
ENOENT: no such file or directory, open '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/🧬️native-codec-factories.schema.json'
```

`proveVcsNativeProviderSelectionFixture` is fully green. `proveNativeOpenableCatalogProviderFixture`
executes its whole rewired prologue (envelope guards, `NativeOpenableOpenTargetV1`,
`NativeOpenableAttestationV1`, all 13 `NativeOpenableHostileExpectationV1` rows — none throws) and then
fails on a **stdio-plugin-partition** file that a concurrent worker has already deleted
(`git status` shows `D ✏️s/🔌️plugins/🗄️stdio/📇️registry/🧪️fixtures/…/*.schema.json`). Not this partition;
see §5.

### 4.5 `bunx vitest run --root 🌎️hub/📦️packages/🟦️typescript`

```
 Test Files  1 passed (1)
      Tests  11 passed | 1 skipped (12)
   Duration  36.83s
```

(The one skip is the `HUB_E2E=1`-gated case; the rewired artifact chunk-CAS case is among the 11 passes —
it also passes alone under `-t "artifact chunk-CAS"`.)

### 4.6 No fixture-owned schema left in this partition

```
$ git ls-files 🌎️hub/🗿️artifact-authority | grep -E 'schema\.json$'
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🌐️browser-actor/🧬️.schema.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/📌️current.schema.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/📡️transport.schema.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/📬️command.schema.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/🔄️cas.schema.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/🔒️owner.schema.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/🧬️.schema.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/📤️publication/🧾️receipt.schema.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🔗️compiled-dependencies/🧬️.schema.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🛡️opened-root/🧬️.schema.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🤝️gis-map-collaboration/🧬️.schema.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧊️codec-source/🧬️.schema.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧬️retained-gis-browser/🧬️.schema.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧬️stdio-gis-bootstrap/🧬️.schema.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧱️generation-stage/🧬️.schema.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🪪️identity-roles/🧬️.schema.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️bundle.schema.json
```

Only `🔏️trusted-catalog` (another worker's scope) remains, and on disk
`find 🌎️hub/🗿️artifact-authority -name '*.schema.json'` already returns nothing — that worker has deleted
theirs too, the listing above is the still-tracked HEAD state.
`🌎️hub/🧪️fixtures/🧭️native-artifact-provider-frontier-v1/` and
`🗿️artifact-authority/🧪️fixtures/🧱️artifact-chunk-cas/` now hold exactly one file each: `🔣️.json`.

## 5. Cross-partition requests

1. **stdio plugin registry** (`✏️s/🔌️plugins/🗄️stdio/📇️registry/**`) — `proveNativeOpenableCatalogProviderFixture`
   still compiles seven stdio-owned schema documents that partition has already deleted from disk:
   `🧬️schema/🧬️native-codec-factories.schema.json`, `🧪️fixtures/📇️native-catalog-surface/{🧬️.schema.json,
   🧬️commitment-cases.schema.json,📌️commitment.schema.json,🧬️imports.schema.json,🧬️budget.schema.json}` and
   `🧪️fixtures/🧾️claim-authority/🧬️.schema.json`. Those `readFileSync`/`ajv.compile` call sites are inside the
   function I own but read files that partition owns; they must be repointed at the stdio scope module
   (`schema://s.stdio.registry/<Export>` or equivalent) by whoever creates it. The command is red until
   then. Also `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧪️fixtures/📇️topic-contributions/🧬️.schema.json`
   (framework/os partition) is compiled from the same function.
2. **`hub.inference`** — `GisInferenceLedgerOracleScript` (`📜️script.ts` ≈ L7671) still builds `Ajv2020` and
   `compile`s `🌎️hub/💡️inference/🧬️schema/🔣️.json`, which is now draft-07; it dies with
   `no schema with key or ref "http://json-schema.org/draft-07/schema#"`. That blocks reaching
   `proveGisNativeProviderSelectionFixture` through its own command, so the GIS rewiring is proven only
   by §4.1 (same module export, same fixture rows, same contract-stage witness) and by its identical VCS
   twin in §4.4. The inference owner should switch that call site to `hubSchemaExport`.
3. **`framework.actor`** — during this work `🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts` imported a
   `../📃️pageSchema/🟦️.ts` that the rename to `📃️page` had already removed, which made every hub script
   command unloadable for ~2.5 minutes. It has since been fixed by that partition; recorded only so the
   coordinator knows the red window was not this change.

## 6. Decisions and open questions

- **`🧪️transaction-v1` was moved, not merely stripped.** It sat at module level (`🌱️creation/🧪️transaction-v1/`)
  rather than inside a fixture collection, so leaving it there would have kept a `🧪️*` directory as a direct
  module member. Its two data files now live in `🌱️creation/🧫️fixtures/🧪️transaction-v1/`.
- **`🧫️fixtures`, not `🧪️fixtures`, for the two new creation collections.** The coordinator brief named
  `🧪️fixtures`, but `🌱️creation` already owns `🧫️fixtures/🌐️http-owner-v1/` and taxonomy
  `testFixturesDirName` is `🧫️fixtures`. Creating a second, differently-named collection in the same
  directory would have been the ugly outcome. The pre-existing `🧪️fixtures` collections under
  `🗿️artifact-authority/` and `📇️native-openable-provider/` were left as they are — renaming them touches
  `🔏️trusted-catalog` include_str! paths and other partitions' script.ts reads.
- **`📚️operation-v1/🦀️.rs` moved into `🧬️schema/`.** It declares `ArtifactCreationIntentV1`,
  `ArtifactCreationPreparedV1`, `ArtifactCreationReceiptV1`, `ArtifactCreationFactV1`,
  `ArtifactCreationOperationV1` and the fact reducer — i.e. it *is* the Rust facet of this scope. The
  `#[path]` in `🌱️creation/🦀️.rs` was updated; the Rust module is still named `operation`, so every
  `artifact_authority::creation::operation::tests::*` law id referenced from `📜️script.ts` and
  `.vscode/launch.json` is unchanged.
- **Envelope consts were not recreated.** Everything the deleted wrappers asserted with `const`/`minItems`
  is now either a module-level domain law (kind→outcome implications, `accepted ⟺ phase ≠ null`,
  legal-transition pairs, phase→status) or an explicit literal/`Set`-size assertion in the prove function.
- **Hostile stages are honest about what a draft-07 schema can decide.** Only `wrong-surface`
  (surface-role suffix is a closed vocabulary) and `cancelled-before-preview` are `stage: "contract"`; the
  deadline rows are `stage: "bounds"` and the identity/bijection rows are `stage: "domain"`, because
  draft-07 cannot compare `nowMs` with `deadlineMs` or check cross-file receipt identity. The independent
  oracles (`valid()` bijection, package-receipt comparison) remain the real judge for those, and
  `assertHubFixtureExpectation` enforces the contract-stage subset.
- **Open question for WP5/WP7.** The three modules ship only the `🔣️jsonschema` facet plus, for
  `hub.artifact-authority.creation`, the pre-existing `🦀️.rs`. `🛰️.proto`, `🔗️.graphql` and `🟦️.ts` facets
  for these scopes are not in this work package; the derived catalog generator will report them missing.
- **Open question.** `hub.artifact-authority` currently exports only the artifact-CAS contracts. The
  `🧪️fixtures/🏛️canonical-authority` and `🧪️fixtures/🔌️authority-adapter` fixtures in the same scope have
  never had a schema (Rust-only oracles); whether they should also gain exports is a separate decision.
