# WP4 — `🌎️hub` scope-owned schema contracts

Partition: `🌎️hub/**`. Executed by one coordinator (scope `hub.inference`, the shared
`🌎️hub/📦️packages/🦀️rust/📜️script.ts` resolver, verification) plus four concurrent workers whose own
reports are `📓️wp4-hub-directory.md`, `📓️wp4-hub-artifact-authority.md`, `📓️wp4-hub-trusted-catalog.md`,
`📓️wp4-hub-bootstrap-auth-admin.md`. Repo MCP was down for the whole session — the ticket was managed on
disk, no ticket tool was called, `🗑️generated/` was never deleted.

## 1. Result

`🌎️hub` now has **ten** scope-owned schema modules carrying **143** `$defs` exports, all draft-07, all under
`https://semio.tech/schema/hub/…`. **Zero** fixture-owned schemas remain anywhere in the partition, and the
three `🧬️schema/` directories that were illegally nested inside `🧪️fixtures` leaves are gone.

```
$ cd /Users/ueli/Documents/semio && find "🌎️hub" -name '*schema.json' -not -path '*/node_modules/*'
$ git ls-files "🌎️hub" | grep -E 'schema\.json$|🧬️schema.json$' | while read -r f; do [ -e "$f" ] && echo "ON DISK: $f"; done
$ find "🌎️hub" -type d -name '🧬️schema' | grep -E '🧪️|🧫️'
```
(all three gates return nothing; the `git ls-files` rows are index entries for files deleted on disk — no
git-modifying command was run, so the index still lists them.)

## 2. Scope → export table

| Scope id | Module path | Formats present | Exports | Export ids |
|---|---|---|---|---|
| `hub.admin` | `🌎️hub/🔨️modules/🛡️admin/🧬️schema/` | `🔣️.json` | 2 | AdminEntryGraphV1, AdminStylesheetGraphV1 |
| `hub.artifact-authority` | `🌎️hub/🗿️artifact-authority/🧬️schema/` | `🔣️.json` | 4 | ArtifactCasObjectKeyV1, ArtifactCasManifestPlanV1, ArtifactCasRetentionLedgerEventV1, ArtifactCasSweepCursorV1 |
| `hub.artifact-authority.creation` | `🌎️hub/🗿️artifact-authority/🌱️creation/🧬️schema/` | `🔣️.json` `🦀️.rs` | 12 | ArtifactCreationPhaseV1, ArtifactCreationFactKindV1, ArtifactCreationOperationStateV1, ArtifactCreationTransitionV1, ArtifactCreationCancellationDecisionV1, ArtifactCreationTransactionKindV1, ArtifactCreationTransactionOutcomeV1, ArtifactCreationAcceptedRecoveryV1, ArtifactCreationHttpLimitsV1, ArtifactCreationHttpRouteV1, ArtifactCreationHttpAuthorityV1, ArtifactCreationHttpResponseV1 |
| `hub.artifact-authority.native-openable-provider` | `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧬️schema/` | `🔣️.json` | 7 | NativeArtifactProviderHeadlessProfileV1, NativeCodecProviderSetIdentityV1, NativeCodecProviderSelectionCaseV1, NativeCodecUnconsumedProfileV1, NativeOpenableAttestationV1, NativeOpenableOpenTargetV1, NativeOpenableHostileExpectationV1 |
| `hub.artifact-authority.trusted-catalog` | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/` | `🔣️.json` `🦀️.rs` | 17 | TrustedCatalogRelativePathV1, TrustedBundleIdentityV1, TrustedBundleCodecV1, TrustedBundleParentDialectV1, TrustedBundleGrantV1, TrustedBundleOpenTargetV1, TrustedBundleFileV1, TrustedBundleComponentV1, TrustedBundleExecutionProtocolV1, TrustedBundleBrowserActorV1, TrustedBundlePackageV1, TrustedBundleProfileOpenTargetV1, TrustedBundleProfileV1, TrustedBundleV1, TrustedCatalogCurrentPointerV1, TrustedCatalogPublicationCommandV1, TrustedCatalogPublicationReceiptV1 |
| `hub.auth` | `🌎️hub/🔐️auth/🧬️schema/` | `🔣️.json` `🟦️.ts` | 7 | InviteCapabilityV1, SessionCapabilityV1, ShareCapabilityV1, SocketGrantCapabilityV1, SocketGrantReceiptV1, AuthLimitsV1, AuthCapabilityVectorsV1 |
| `hub.directory` | `🌎️hub/📇️directory/🧬️schema/` | `🔣️.json` | 30 | DirectoryDocumentScopeV1, DirectorySocketScopeV1, DirectorySocketMessageRoutingV1, DocumentOpenIntentV1, ExecutionTargetAssetRouteV1, SocketGrantReceiptV1, DirectoryCommandReceiptV1, DirectorySessionBindingV1, DirectorySpaceSessionBindingV1, SpaceAdministrationSpaceV1, SpaceAdministrationMemberRowV1, SpaceAdministrationInviteRowV1, PublicSpaceDetailV1, MemberSpaceDetailV1, AuthorSpaceDetailV1, DirectoryDocumentDescriptorV1, DirectoryEventPageCursorV1, DirectoryProfileV1, LocalizedTextV1, AdminPrincipalV1, AdminCreateSpaceIntentV1, AdminOperationIntentV1, AdminActorV1, AdminRecordedConnectionV1, AdminIntentOutcomeV1, InviteRedemptionCallV1, InviteAcceptanceMarkerV1, PresenceAdmissionV1, PresenceLeaseOperationV1, PresenceScopeSummaryV1 |
| `hub.inference` | `🌎️hub/💡️inference/🧬️schema/` | `🔣️.json` `🦀️.rs` `🟦️.ts` | 45 | see §3 |
| `hub.lag-rebootstrap` | `🌎️hub/🛰️lag-rebootstrap/🧬️schema/` | `🔣️.json` | 7 | CanonicalCheckpointPairBaselineV1, CanonicalCheckpointPairBlobV1, CanonicalCheckpointPairSelectionV1, CanonicalCheckpointPairLimitsV1, CanonicalPairPartV1, CanonicalPairTerminalV1, RebootstrapTransferLimitsV1 |
| `hub.local-bootstrap` | `🌎️hub/🚀️local-bootstrap/🧬️schema/` | `🔣️.json` | 12 | LocalBootstrapProfileV1, LocalBootstrapPipeInitializeV1, LocalBootstrapPipeHelloV1, LocalBootstrapPipeHelloAcceptedV1, LocalBootstrapPipeIssueV1, LocalBootstrapPipeRejectV1, LocalBootstrapPipeCancelV1, LocalBootstrapPipeShutdownV1, LocalBootstrapPipeV1, LocalBootstrapCredentialEnvelopeV1, LocalBootstrapReadinessV1, LocalBootstrapIdleAdmissionV1 |

**`🛰️.proto` / `🔗️.graphql`: none added, deliberately.** No contract in this partition is transported as
protobuf or GraphQL. Every hub contract crosses either HTTP+JSON (`💡️inference`, `📇️directory`, `🔐️auth`,
`🗿️artifact-authority`), the framework's own binary replication/WAL framing (`✉️inference-command`,
`⛓️inference-wal-chain` — a length-prefixed varint envelope, not protobuf), or a local pipe carrying JSON
(`🚀️local-bootstrap`). Repo-wide there is no `.proto`/`.graphql` producer or consumer under `🌎️hub`
(`git ls-files 🌎️hub | grep -E '\.(proto|graphql)$'` → empty, both before and after this work). Adding them
would create formats nothing reads.

## 3. `hub.inference` — the complete module (task 1)

`🌎️hub/💡️inference/🧬️schema/` is now `{🔣️.json, 🟦️.ts, 🦀️.rs, ✅️approval/🦀️.rs}`.
`🔣️.json` is draft-07 with `$id https://semio.tech/schema/hub/inference/schema.json` and 45 PascalCase
exports (plus six lower-camel private helpers `id`, `hash`, `serverId`, `safeInteger`, `generation`,
`hexBytes`, which are not exports):

`InferenceServerIdV1`, `InferenceDocumentScopeV1`, `InferenceRequestV1`, `InferenceParentDialectV1`,
`InferenceBindingIdentityV1`, `InferenceIdentityV1`, `InferenceJobStateV1`, `InferenceProposalStateV1`,
`InferenceLifecycleKindV1`, `InferenceLimitsV1`, `InferenceApprovalRequestV1`, `InferenceApprovalReceiptV1`,
`InferenceApprovalOutboxV1`, `GisMapDocumentFrontierV1`, `GisMapApprovalUndoHandleV1`,
`GisMapApprovalUndoRequestV1`, `GisMapApprovalUndoReceiptV1`, `GisMapApprovalUndoTargetV1`,
`GisInferenceCheckpointControlFrameV1`, `GisInferenceCheckpointControlDirectionV1`,
`GisMapInferencePreviewV1`, `InferenceMapBoundsV1`, `InferenceMapSummaryV1`, `InferenceProgressV1`,
`InferenceEventV1`, `InferenceJobReceiptV1`, `InferenceEventPageV1`, `InferenceHybridLogicalTimestampV1`,
`InferenceCommandPayloadV1`, `InferenceCommandV1`, `InferenceCommandLimitsV1`, `InferenceWalChainPolicyV1`,
`InferenceWalTargetV1`, `InferenceCatalogOwnerV1`, `InferenceCatalogFrontierV1`,
`InferenceCatalogDescriptorV1`, `InferenceCatalogPackageV1`, `InferenceCatalogServiceV1`,
`InferenceCatalogSelectionV1`, `GisMapFrozenExecutionProtocolV1`, `GisMapFrozenPackageV1`,
`GisMapFrozenArtifactV1`, `GisMapFrozenSurfaceV1`, `GisMapFrozenGrantV1`, `GisMapFrozenBindingV1`.

`🟦️.ts` exports the matching type **and** a `parse<Export>()` for all 45 (verified by a run, §6), built on a
handful of private structural helpers — no external runtime dependency.

`🦀️.rs` stays the decoder authority. It gained the module identity (`SCHEMA_SCOPE`, `SCHEMA_ID`), a
re-export of the four approval-undo types that physically live in the os kernel
(`directory::os_directory::{CheckpointPublicationFrontierV1 as GisMapDocumentFrontierV1,
GisMapApprovalUndoHandleV1, GisMapApprovalUndoReceiptV1, GisMapApprovalUndoRequestV1}`) so the module is the
single Rust entry point, and three `#[cfg(test)]` laws:

- `hub_inference_module_declares_every_export_as_a_closed_draft_07_contract` — dialect, `$id`, the export set
  is exactly the 45 names, every object export is `additionalProperties: false`, and `required` equals
  `properties` except for one declared-optional pair (`InferenceEventPageV1.preview`, which the Rust DTO
  marks `skip_serializing_if`).
- `hub_inference_exports_agree_with_the_rust_decoders_field_for_field` — decodes the real fixtures through
  `InferenceIdentityV1`, `InferenceRequestV1`, `InferenceBindingIdentityV1`, `InferenceParentDialectV1`,
  `InferenceApprovalRequestV1::decode`, `GisMapApprovalUndoRequestV1`, `GisMapApprovalUndoHandleV1` and
  `GisMapApprovalUndoTargetV1`, then asserts the serialized camelCase key set equals the schema's `required`
  list for each.
- `hub_inference_approval_hostiles_declare_their_rejection_stage` — every hostile fixture row carries
  `{stage:"contract", result:"rejected", code}` and is actually refused by the Rust decoder.

A third law uses the **framework validator**, which became usable mid-session: when this work started
`🧰️framework/🔨️modules/🧬️schema/✅️validator.rs` implemented no `pattern` keyword and
`validate_schema_node` *rejects* every keyword it does not implement, so a module built on hex/`serverId`
patterns could not compile through it at all. WP2 has since landed `PatternMatcher` (`✅️validator.rs:233`,
`:528`, ECMA subset incl. `(?:…)`) and the `items` array form, so `hub.inference` now also carries:

- `hub_inference_fixtures_validate_through_the_owned_draft_07_validator` — compiles each export by pinning
  the module document's root `$ref` at it, runs `OwnedJsonSchemaValidator::validate_json` over twelve real
  fixture members, and asserts the validator refuses all 14 approval hostiles. `semio-framework-schema` was
  added to hub's `[dev-dependencies]` only (test-only; it never enters a runtime build, per CLAUDE.md).

To make that possible, `InferenceIdentityV1`'s conditional rule (`headEditId == "" ⇒ headOrdinal == 0`) was
re-expressed from the unsupported `if`/`then` pair into an equivalent `anyOf`:
```json
"anyOf": [
  { "properties": { "headEditId": { "$ref": "#/$defs/serverId" } } },
  { "properties": { "headEditId": { "const": "" }, "headOrdinal": { "const": 0 } } }
]
```
Equivalence was checked against the four boundary cases rather than assumed — `("0"*32, 1) → accept`,
`("", 1) → reject`, `("", 0) → accept`, `("a b", 1) → reject` — and the whole
`GisInferenceLedgerOracleScript` server-identity parity loop (11 cases × 5 fields) still passes. See §7.7:
four sibling hub modules still use `if`/`then`/`else` and therefore cannot be compiled by the owned validator
yet.

The third-party AJV cross-check that CLAUDE.md's dual-implementation rule requires is done from TypeScript
against the same module file (§6.2), so each contract is now checked by three independent implementations:
hand-written Rust decoders, the owned Rust draft-07 validator, and AJV.

## 4. Oracle rewiring (task 3)

A new region `//#region 🧬️Scope-owned schema resolution` at the top of
`🌎️hub/📦️packages/🦀️rust/📜️script.ts` is the single binding point for the whole partition:

- `HUB_SCHEMA_SCOPES` — the ten scope ids → module paths.
- `hubSchemaModule(repoRoot, scope)` — loads a module once, **asserts** it declares the draft-07 dialect and
  a `https://semio.tech/schema/hub/…` `$id`, and keeps its compiled `Ajv` (draft-07 `ajv`, not
  `ajv/dist/2020`).
- `hubSchemaExport(repoRoot, "schema://<scope id>/<ExportId>")` → a validator. The URI form of §A of the
  contract is used **now**; when the harness worker lands `schema://` in `resolveFixtures`, fixture
  descriptors can carry the identical string and this helper becomes the resolver's hub adapter.
- `HubFixtureExpectationV1` = `{stage, result, code}` and `assertHubFixtureExpectation(name, expectation,
  admitted)` — the replacement for wrapper-schema hostile lists.

No `prove*` function in `🌎️hub` reads a fixture-local schema any more. The remaining
`readFileSync(… "🧬️.schema.json")` calls in that file all point **outside** `🌎️hub` (stdio/gis plugin
registry, `🧰️framework/…/📇️directory`, `🧰️framework/…/🛢️db`, `🧰️framework/🔨️modules/🌱️value`) and are
listed in §7 as cross-partition requests.

`hub.inference`'s thirteen functions, rewired by this coordinator:

| Function | Binds | Envelope replaced by |
|---|---|---|
| `proveInferenceWalProofFixture` | `InferenceCommandV1`, `InferenceWalTargetV1` | schema-const + inventory + trace-name uniqueness assertions |
| `proveInferenceWalChainFixture` | `InferenceWalChainPolicyV1` | 14 unique case names, 3 ownership rows, 2 retained boundaries |
| `proveInferenceCatalogSelectionFixture` | `InferenceCatalogSelectionV1` | 12 unique case names |
| `proveGisMapApprovalUndoFixture` | `GisMapApprovalUndoTargetV1`, `GisMapApprovalUndoRequestV1` | history dispatch tag/order law + request↔target binding law (replaces the `prefixItems` tuple) |
| `proveGisInferenceRetainedRuntimeFixture` | `InferenceLimitsV1` (cross-checked against the canonical limits fixture) | limits identity law, 9 traces, 10 unique source hostiles |
| `proveGisInferenceCheckpointControlFixture` | `GisInferenceCheckpointControlFrameV1`, `GisInferenceCheckpointControlDirectionV1` | direction order law |
| `proveGisMapProposalApprovalFixture` | `InferenceBindingIdentityV1`, `InferenceLimitsV1`, `GisMapInferencePreviewV1`, `InferenceMapSummaryV1`, `InferenceLifecycleKindV1` | **49 explicit committer laws** (38 scalar + 11 ordered) replacing the 16 wrapper `const` mutations, plus an error-code namespace law over `errors`/`approvalRejections`/`visibility` |
| `GisInferenceLedgerOracleScript.run` | `InferenceRequestV1`, `InferenceIdentityV1`, `InferenceApprovalOutboxV1`, `InferenceMapSummaryV1`, `InferenceServerIdV1` | ledger + server-identity envelope/inventory laws; every server-identity case now also runs through `parseInferenceServerIdV1` and `assertHubFixtureExpectation` |
| `proveInferenceApprovalRequestFixture` | `InferenceApprovalRequestV1` | fixture-declared `{stage,result,code}` per hostile, plus a TypeScript-parser parity check |
| `proveInferenceAuthorFixture` | — (pure SQLite oracle, no wire contract existed) | 16 unique operation names |
| `proveInferenceCommandFixture` | `InferenceCommandLimitsV1`, `InferenceCommandV1` | envelope + 20 unique vector names/changes |
| `proveGisMapFrozenBindingFixture` | `GisMapFrozenBindingV1` | per-hostile `assertHubFixtureExpectation`; the substitution now runs against the module export instead of a fixture wrapper |
| `proveGisMapTwoAuthorCompositionFixture` | — (e2e scenario spec, no wire contract) | 16 declarative laws + step/hostile uniqueness (replaces 10 wrapper mutations) |

## 5. Files created / moved / deleted (this coordinator's slice)

Created / rewritten
- `🌎️hub/💡️inference/🧬️schema/🔣️.json` — draft-07, new `$id`, 45 exports (was 2020-12, `urn:semio:hub:inference-job:v1`, 3 contract defs).
- `🌎️hub/💡️inference/🧬️schema/🟦️.ts` — 45 types + 45 `parse*` (was 1 type + 1 parse).
- `🌎️hub/💡️inference/🧬️schema/🦀️.rs` — module identity, undo re-exports, 3 test laws.

Moved
- `🌎️hub/💡️inference/🧬️schema/🤝️two-author-shell-v1/🔣️.json` → `🌎️hub/🧪️fixtures/🤝️two-author-shell-v1/🔣️.json` (audit decision 2: an e2e scenario fixture never belonged inside a `🧬️schema/` module).

Deleted
- `🌎️hub/💡️inference/🧬️schema/🤝️two-author-shell-v1/🧬️.schema.json` (and the directory).
- 13 fixture-owned schemas: `🌎️hub/🧪️fixtures/{✅️inference-approval-v1, ↩️gis-map-approval-undo-v1, ⏸️gis-inference-checkpoint-control-v1, ⛓️inference-wal-chain-v1, ✉️inference-command-v1, 🎯️inference-catalog-selection-v1, 🖥️inference-server-identity-v1, 🗳️gis-map-proposal-approval-v1, 🗺️gis-inference-job-v1, 🗺️gis-inference-retained-runtime-v1, 🛂️inference-author-v1, 🧊️gis-map-frozen-binding-v1, 🧾️inference-wal-proof-v1}/🧬️.schema.json`.

Fixture data edited (data only)
- `🌎️hub/🧪️fixtures/✅️inference-approval-v1/🔣️.json` — each of the 14 hostile rows gained `stage`/`result`/`code`.
- `🌎️hub/🧪️fixtures/🗺️gis-inference-job-v1/🔣️.json` and `🌎️hub/🧪️fixtures/🧾️inference-wal-proof-v1/🔣️.json` — see §5a.

Edited
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — the resolver region plus the thirteen functions above.

Nothing outside `🌎️hub/**` was modified by this coordinator. No `include_str!`, `📋️project.json`,
`.vscode/launch.json`, `package.json` or nx input referenced any path that moved
(`grep -rn '🤝️two-author-shell-v1'` now resolves only to the new fixture path and the WP0 audit JSON).

### 5a. Pre-existing red repaired: the ledger identity digest chain

`gis-inference-ledger-oracle` was already failing at `HEAD` (`0a0bb74380`) with
`neutral input/identity hash mismatch`, unrelated to this ticket. Provenance, established with
`git log --date=iso` rather than commit-message text:

- `860e015bf6` (2026-09-08 04:50:36 +0200, i.e. before this session) changed exactly one line of
  `🌎️hub/🧪️fixtures/🗺️gis-inference-job-v1/🔣️.json`: `identity.binding.digest`
  `435e0206…` → `24a934f5…`.
- At the previous commit `b0dfa0f09b` the fixture was self-consistent (verified by recomputing
  `identityDigest` and `outbox.jobId` from that blob — both matched).
- The new value equals `🧊️gis-map-frozen-binding-v1/🔣️.json`'s `expectedDigest`, so the change was correct
  and simply not propagated to the four derived values.

The propagation is fully determined by the fixture's own laws, so it was completed rather than reported:
`identityDigest` = sha256(`semio.hub.inference-identity/v1\0` + canonical identity), `outbox.jobId` =
sha256(`semio.hub.inference-job-id/v1\0` + identityDigest)[..32], `outbox.mutationId` =
sha256(`semio.hub.inference-approval-mutation/v1\0` + jobId + `\0` + proposalHash)[..32], and the canonical
command bytes (`encodedHex`, `commandHash`) which embed `mutationId` — mirrored into
`🧾️inference-wal-proof-v1/🔣️.json` (`jobId`, `command.mutationId`, `encodedHex`, `commandHash`). No other
fixture cross-references these values. `outbox.proposalHash` was already correct and is unchanged.

## 6. Verification — real output

### 6.1 All ten modules load as draft-07 and every export compiles (third-party AJV)
```
hub.admin | draft-07 | ok | 2 | 🔣️.json
hub.artifact-authority | draft-07 | ok | 4 | 🔣️.json
hub.artifact-authority.creation | draft-07 | ok | 12 | 🔣️.json 🦀️.rs
hub.artifact-authority.native-openable-provider | draft-07 | ok | 7 | 🔣️.json
hub.artifact-authority.trusted-catalog | draft-07 | ok | 17 | 🔣️.json 🦀️.rs
hub.auth | draft-07 | ok | 7 | 🔣️.json 🟦️.ts
hub.directory | draft-07 | ok | 30 | 🔣️.json
hub.inference | draft-07 | ok | 45 | 🔣️.json 🦀️.rs 🟦️.ts
hub.lag-rebootstrap | draft-07 | ok | 7 | 🔣️.json
hub.local-bootstrap | draft-07 | ok | 12 | 🔣️.json
TOTAL EXPORTS 143
```
(`$id` column `ok` means the `$id` is exactly `https://semio.tech/schema/<scope path>/schema.json`.)

### 6.2 `hub.inference` — AJV ⟷ TypeScript ⟷ fixture agreement
```
exports=45 parse-functions=45 missing=[]
hub.inference module verified: exports=45 checks=107 hostile=49
```
107 positive checks (every contract-shaped member of all 13 fixtures, each validated by AJV **and** parsed by
the hand-written TS parser) and 49 hostile rows, none admitted by either implementation.

### 6.3 `bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts gis-inference-ledger-oracle`
```
inference-server-identity-oracle: cases=11 fields=5 composite-bounds=2 ajv+node=1
gis-inference-ledger-oracle: traces=9 hostile=13 identity-hostile=17 sqlite-integers=6 ajv+typescript=1 hashes=6 independent-bounds=1; no executor/route/approval claim
inference-wal-proof-oracle: traces=17 ownership=3 binding-hostile=2 protocol-envelope=1 node-sha256=1; committed-WAL runtime still required
inference-command-oracle: vectors=20 ajv=1 node+webcrypto-hash=2; no GIS execution authority
inference-approval-request-oracle: valid=1 hostile=14 byte-boundaries=2 ajv+node=1; no approval authority
inference-author-oracle: cases=16 accepted=1 ajv+sqlite=1; no retained grant or submit authority
inference-wal-chain-oracle: exact=14 hashing-ownership=3 retained-boundaries=2 ajv=1 crc-valid=14 blake3-known-answer=1; Rust replay and third-party blake3 parity pending
inference-catalog-projection-oracle: exact=12; no native provider or route authority
trusted-catalog-identity-oracle: exact=6 canonical-kind=1 descriptor-sha256=1 package-ref-blake3=distinct; no GIS provider activation
gis-native-codec-oracle: receipts=2 hostile=8 ajv+node+webcrypto=1; no catalog activation or GIS execution claim
gis-controlled-proposal-oracle: literal=1 bounds=1 interruption=3 rejection=7 ajv=1; no hub approval authority
gis-native-provider-selection-oracle: cases=8 accepted=1 scope-exports=1; no native or catalog activation claim
memory-backing-oracle: tables=8 admission=3 hostile=5 timer-retry=1 sequential=128; runtime ABI sizes and worker wake are checked by the Rust owner laws
native-deficit-oracle: lanes=6 maximum-rounds=8 checks=8; shared independent oracle, cooperative host turns remain distinct
```
Exit 0. (Before §5a this command stopped at `neutral input/identity hash mismatch`; every rewired assertion
above that point already passed.)

A re-run 20 minutes later reproduced all nine `hub.inference` lines plus `trusted-catalog-identity-oracle`
unchanged, then aborted **inside another partition's script** at
`✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts:54` with
`ENOENT … 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️fixtures/🌱️artifact-document-id-v1/🧬️.schema.json`
— the os/framework partition deleted that fixture schema between the two runs without repointing the gis
reader. Both files are outside `🌎️hub/**`; see §7.4. Nothing in `🌎️hub` regressed: `hub.inference`'s own
contract agreement is independently re-proved by §6.2, which does not enter foreign scripts at all.

### 6.4 `bun … gis-map-frozen-binding-check` and `bun … gis-map-proposal-check --source`
```
gis-map-frozen-binding-check: checks=54 clean; no route, provider, inference execution, or publication claim
gis-inference-checkpoint-control-oracle: ajv=1 exact=2 hostile=8 fd=4 test-support-only=1 passed
gis-map-approval-undo-oracle: AJV=1 exact=6 genesis-first=1 history=2 hostile=8
gis-map-proposal-oracle: scope-exports=5 committer-laws=49 node-sha256=2 independent-bounds=2 preview=1 lifecycle=9 checkpoint-control=8 retained-runtime=9 durable-undo=17 committer=7 visibility=7 errors=11 approval-rejections=11 cross-fixture=1 process-source=26; no external model provider, no WGPU rendering
gis-map-proposal-check: neutral source oracle passed with hostile=49; native laws and the two-user process journey remain unclaimed. No external model provider, no WGPU rendering.
```
Exit 0.

### 6.5 `bun … trusted-stdio-gis-bundle-check --two-author-source`
```
[DEBUG] GIS Map witness socket retirement: neutral=5 Bun=1 third-party-ws=1 exact-close-events=2; no Hub restart claim
GIS Map two-author composition fixture laws=16 hostile=10 current-coordinates=5; real mounted journey remains separately required
```
Exit 0 (run with `SEMIO_TEST_ARTIFACT_DIR=<ticket>/🗑️generated/wp4-hub`, which the socket-retirement oracle
requires).

### 6.6 `bunx vitest run --root 🌎️hub/📦️packages/🟦️typescript` (re-run after every edit above)
```
 RUN  v4.1.10 /Users/ueli/Documents/semio/🌎️hub/📦️packages/🟦️typescript

 Test Files  1 passed (1)
      Tests  11 passed | 1 skipped (12)
   Duration  18.80s
```
(the skipped one is the `HUB_E2E`-gated journey.)

### 6.6b Worker commands
Each worker pasted its own real output in its own report. Summarised: `hub.directory` — module harness
`assertions=304 exports=30 fixtures=21` plus 14 green router commands; `hub.artifact-authority` —
`checks=159 passed=159 mutations-admitted=0`, `space-artifact-creation-check source` and
`execution-target-relay-check` green, `bunx vitest run --root 🌎️hub/📦️packages/🟦️typescript` 11 passed /
1 skipped; `hub.artifact-authority.trusted-catalog` — 86 contract checks, `trusted-catalog-opened-root-check`
green, all 7 rewired functions green; `hub.local-bootstrap`/`lag-rebootstrap`/`auth`/`admin` — 34 fixture
members accepted, 12 hostile members rejected, `os-hub-ts test long` 11 passed / 1 skipped, both `os-hub-admin`
oracles green.

### 6.7 `cargo check -p semio-hub`

**Default features — blocked by a concurrent peer refactor, not by this work.**
```
error: could not compile `semio-s-artifact-gis-gismap` (lib) due to 87 previous errors
```
All 87 errors are in `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/**` (`E0433 cannot find 'artifacts' in 'crate'`,
`E0432 unresolved import 'protocol'`, `cannot find attribute 'value'/'dsl'`, `cannot find module 'store'`),
none in `🌎️hub`. That tree is being split into per-artifact crates by another session right now — untracked
`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/{🏔️gisterrain,🗺️gismap}/📦️packages/` directories, mtime 16:44 today.
`semio-hub`'s default features pull `semio-s-plugin-gis`, so the full check cannot complete until that lands.

**`cargo check -p semio-hub --no-default-features --features sqlite --all-targets`** (the same check with
that one dependency edge dropped) — every Rust line this ticket touched lives in the lib, and the lib and its
test target compile:
```
warning: `semio-hub` (lib) generated 15 warnings
warning: `semio-hub` (lib test) generated 24 warnings (12 duplicates)
warning: `semio-hub` (bin "os-hub") generated 28 warnings
error: could not compile `semio-hub` (bin "os-hub") due to 1 previous error; 28 warnings emitted
warning: `semio-hub` (bin "os-hub" test) generated 63 warnings (25 duplicates)
error: could not compile `semio-hub` (bin "os-hub" test) due to 5 previous errors; 63 warnings emitted
```
- **lib: 0 errors. lib test: 0 errors.** That covers `🌎️hub/💡️inference/🧬️schema/🦀️.rs` (module identity,
  the four os re-exports, the three new `#[cfg(test)]` laws incl. the `semio-framework-schema` dev-dependency),
  `🌎️hub/🗿️artifact-authority/🌱️creation/🧬️schema/🦀️.rs` (moved production module, `#[path]` updated),
  `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🦀️.rs` (new `pub` module + call-site renames),
  and every `include_str!` retarget.
- **bin `os-hub`: 6 × `E0308`, none from this ticket.** All are in `🚀️bin.rs` at `:3817`, `:11953`,
  `:12622`, `:12637`, `:12722`, and all report the same two os-kernel type changes:
  `DirectoryStreamMessage::Event` now carries `event: Box<DirectoryEvent>`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:2712`, committed in `0a0bb74380`,
  2026-09-08 14:30) and `authority.checkpoint` became `Option<DocumentOpenCheckpointV1>`. `🚀️bin.rs` is
  currently **staged-modified by another session** (`git status --porcelain` → `M  🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`),
  i.e. a peer is already catching it up, so it was deliberately not edited here. The compiler's own fixes are
  `Box::new(event)` at the four construction/comparison sites and `Some(authority.checkpoint)` at `:3817`.
- Warning hygiene: the six `unused_qualifications` warnings that the new
  `pub use directory::os_directory::{…}` in `hub.inference` introduced (via `use super::{… schema::*}` in
  `💡️inference/🪶️sqlite/🦀️.rs`) were fixed by dropping the now-redundant `directory::os_directory::`
  prefixes at `:158 :164 :824 :925 :956 :1034`; two more in
  `🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:1344,1350` and three in
  `📇️directory/🪶️sqlite/🦀️.rs:586,592,598` were fixed the same way. All eleven are gone from the re-run.
  The warnings that remain under `--no-default-features` are dead-code/unused-import artifacts of the reduced
  feature set plus `🚀️bin.rs`, which this ticket did not touch.

Still to run once the GIS artifact split lands (nothing in this report depends on it):
```
CARGO_TARGET_DIR=<private> RUSTC_WRAPPER="" cargo check -p semio-hub --all-targets
CARGO_TARGET_DIR=<private> RUSTC_WRAPPER="" cargo test -p semio-hub --lib hub_inference
```

## 7. Cross-partition requests

1. **os/mcp duplicates of `semio.hub.inference-approval/v1`** (contract §D row 1). Three independent
   re-implementations remain outside this partition and nothing keeps them in step with `hub.inference`:
   - `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🦀️.rs:1995`
     (`GisMapInferenceApprovalRequestV1`) + `…/📇️directory/🧬️schema/🟦️.ts:1626-2020`
     (`sealGisMapInferenceApprovalRequestV1`).
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🦀️.rs:654-661,986` — a second
     `GisMapInferenceApprovalRequestV1` with its own `GIS_MAP_INFERENCE_APPROVAL_SCHEMA` const.
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/💡️inference-bridge.ts:61,259`
     — an inline AJV-shaped object literal.
   Requested change: each becomes a conformance-checked mirror that `$ref`s
   `https://semio.tech/schema/hub/inference/schema.json#/$defs/InferenceApprovalRequestV1` (and
   `…/InferenceApprovalReceiptV1`), or re-exports the hub type. The same applies to
   `GisMapInferenceApprovalReceiptV1` (`…/📇️directory/🧬️schema/🦀️.rs:2091`) versus
   `hub.inference/InferenceApprovalReceiptV1`.
2. **`GisMapApprovalUndoHandleV1` / `…RequestV1` / `…ReceiptV1` / `CheckpointPublicationFrontierV1` live in
   `os.directory`, but hub decodes them** (`🌎️hub/💡️inference/🏃️runtime/🦀️.rs:3468`). `hub.inference` now
   owns the JSON-Schema half and re-exports the Rust structs. When `os.directory` becomes a draft-07 module,
   its schema for these four should `$ref` the hub `$defs` (or the structs should move to hub) so there is
   exactly one authority. Note `CheckpointPublicationFrontierV1` is exported here as
   `GisMapDocumentFrontierV1`; if os prefers the other name, say so and this side will follow.
3. **`hubSchemaExport` builds one `Ajv` per scope, so cross-scope `$ref` does not resolve yet.** That is why
   `hub.artifact-authority.trusted-catalog/TrustedBundleBrowserActorV1` restates the os browser-actor identity
   fields instead of `$ref`-ing them. Once `os.directory`'s `🌐️browser-actor` module is draft-07 with a
   `semio.tech` `$id`, WP2's catalog should feed `dependsOn` into a shared `Ajv` and these restatements become
   `$ref`s.
4. **Schemas still read from outside `🌎️hub` by the hub script**, which the owning partitions must repoint at
   their own scope modules (they are untouched here):
   `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/🧬️native-codec-factories.schema.json`, the five
   `🗄️stdio/📇️registry/🧪️fixtures/📇️native-catalog-surface/*.schema.json`,
   `🗄️stdio/…/🧾️claim-authority/🧬️.schema.json`, `✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🧬️.schema.json`,
   `🧰️framework/🔨️modules/🌱️value/🔁️codec/🧪️fixtures/🧬️.schema.json`,
   `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️fixtures/🧮️memory-backing/🧬️.schema.json`,
   and the `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/{🌐️browser-actor/🔣️.schema.json,
   📣️checkpoint-publication-command-v1/🧬️.schema.json, 🌱️space-artifact-creation-v1/🧬️.schema.json}` group.
   Several of these have already been **deleted** by their owning partitions without the hub reader being
   rewired, which is what currently reds `native-openable-catalog-provider-check` and three trusted-catalog
   commands (details in the two worker reports). One more appeared during the final verification pass:
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️fixtures/🌱️artifact-document-id-v1/🧬️.schema.json`
   was deleted while `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/📜️script.ts:54` still reads it, which now
   aborts `gis-inference-ledger-oracle` **after** every hub oracle in it has printed green. Neither file is
   in this partition.
5. **`os/dev`** — `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/…/📜️script.ts:5269-5277` still reads the
   deleted `🛰️lag-rebootstrap/🧪️fixtures/🪢️canonical-pair/🧬️.schema.json`; the exact replacement is in
   `📓️wp4-hub-bootstrap-auth-admin.md` §E.1.
6. **WP2 catalog/taxonomy** — the ten scope ids above, with their module paths and export lists, are ready to
   be emitted into `🔣️schema-catalog.json`. `HUB_SCHEMA_SCOPES` in the hub script is a hand-maintained
   duplicate of that catalog and should be replaced by a read of the generated file once it exists.
7. **`✅️validator.rs` needs `if`/`then`/`else`.** Contract §C now names
   `semio_framework_schema::structural_validator_for` / `✅️validator.rs` as *the* draft-07 structural
   validator. Its keyword table (`validate_schema_node`) rejects any keyword it does not implement, and
   `if`/`then`/`else` are not implemented — so four of the ten hub modules cannot be compiled by it today:
   `hub.artifact-authority` (`if`,`then`,`else`), `hub.artifact-authority.creation` (`if`,`then`,`else`),
   `hub.artifact-authority.native-openable-provider` (`if`,`then`), `hub.local-bootstrap`
   (`if`,`then`,`else`). Every other keyword used by all ten modules is inside the supported subset — verified
   by walking all ten documents against the table in `✅️validator.rs:206-262`:
   ```
   UNSUPPORTED 🌎️hub/🗿️artifact-authority/🧬️schema/🔣️.json if, then, else
   UNSUPPORTED 🌎️hub/🗿️artifact-authority/🌱️creation/🧬️schema/🔣️.json if, then, else
   UNSUPPORTED 🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🧬️schema/🔣️.json if, then
   UNSUPPORTED 🌎️hub/🚀️local-bootstrap/🧬️schema/🔣️.json if, then, else
   ```
   Requested change: implement draft-07 `if`/`then`/`else` in `✅️validator.rs` (compile-side keyword entry +
   an `validate_value_inner` branch that runs `then` when `if` matches and `else` otherwise, both treated as
   non-failing subschema probes exactly like `anyOf` branches). Until then those four scopes are covered by
   AJV only, and `hub.inference` deliberately avoids the keywords so it can be held by both validators.

## 8. Open questions

1. **`🌎️hub/🧪️fixtures/🤝️two-author-shell-v1`** — the WP0 audit asked whether "two-author GIS Map
   composition" is live or planned. It is exercised only by
   `trusted-stdio-gis-bundle-check --two-author-source|--two-author-shell`; no application implementation
   exists. Kept as a fixture with 16 declarative laws; a dev decision is still needed on whether it stays.
2. **`hub.artifact-authority.creation` fixture directory name** — the worker used `🧫️fixtures` (matching
   taxonomy `testFixturesDirName` and the collection `🌱️creation` already had) rather than `🧪️fixtures`.
   Flagged so WP2's taxonomy work can settle which of the two names is canonical for hub.
3. **`hub.admin`** — `AdminEntryGraphV1`/`AdminStylesheetGraphV1` describe a dev tool's own graph output, not
   a hub wire contract. They were given a scope module because `*.schema.json` must cease to exist, but if
   WP2 decides dev-tool output shapes are not scope contracts, this module should be reconsidered.
   Note the worker found real drift the old file had been hiding: it pinned `shared.export` to `"./🎨️.css"`
   while the fixture and the production resolver both use `"./🧵️.css"` — the schema was never actually run
   through a validator. `AdminStylesheetGraphV1` pins the real value.
4. **`🔣️bundle.schema.json` was not orphaned.** WP0 decision item 8 is wrong: it had three file readers and
   two cross-schema `$ref` consumers (evidence in `📓️wp4-hub-trusted-catalog.md`). It has been folded into
   `hub.artifact-authority.trusted-catalog` as the 13 `TrustedBundle*V1` exports and deleted; all readers were
   rewired. No dev confirmation was needed for deletion since the content survives.
5. **Contract §D row 2** says "all `🌎️hub/🧪️fixtures/*-v1` contracts → `hub.inference`". That was written
   about the seed cluster; applied literally it would put directory, execution-target and native-provider
   contracts in the inference module. Routed by real owner instead (`hub.directory` for
   `🧪️fixtures/📇️directory/**` and `🪪️execution-target-relay-v1`,
   `hub.artifact-authority.native-openable-provider` for `🧭️native-artifact-provider-frontier-v1`), per the
   task brief's "their real owner" instruction. Flagged as a deviation from §D as written.
