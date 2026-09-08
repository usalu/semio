# WP4c — `🌎️hub` export completeness and Rust scope registration

Partition: `🌎️hub/**`. Input: the 68 `schema-export-incomplete` findings of
`🗑️generated/wp1c-test-schema.json` whose `path` starts with `🌎️hub`. Repo MCP was down for the whole
session — no ticket tool was called, `🗑️generated/` was never deleted, no git-modifying command was run.

## 0. How the 68 rows were measured (the brief's command does not measure them)

`bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts test schema --under 🌎️hub --json` returns
`{"diagnostics": [], "fixtures": []}` **and always has**, before and after this work:

```
🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts:4861
  return [...schemaPlacementDiagnostics(…), …, ...(under.length === 0 ? schemaResolutionDiagnostics(repoRoot) : [])];
```

`schema-export-incomplete` is emitted by `schemaExportCompletenessDiagnostics`, which only runs inside
`schemaResolutionDiagnostics`, which `--under` **skips** — resolution is a whole-catalog question. So
`--under 🌎️hub` is structurally incapable of reporting this code, and a green `--under` run is not
evidence. The partition is measured instead by `wp4c-hub-probe.ts` (in this ticket folder), which calls
the harness's own `schemaResolutionDiagnostics(repoRoot)` and keeps the rows whose `scope` starts with
`hub.` or whose `path` starts with `🌎️hub`. It is the same code path the full-tree gate runs.

```
$ bun <ticket>/wp4c-hub-probe.ts .            # before
[wp4c-hub] 68 finding(s) of 3141 repo-wide
[wp4c-hub]     68 × schema-export-incomplete
```

68 = 39 `hub.inference` + 14 `hub.artifact-authority.trusted-catalog` +
12 `hub.artifact-authority.creation` + 3 `hub.auth`. No other hub scope had a finding, because the other
six hub scopes provide only `🔣️jsonschema` and a single-format scope is complete by construction.

## 1. Result

```
$ bun <ticket>/wp4c-hub-probe.ts .            # after
[wp4c-hub] 0 finding(s) of 4258 repo-wide
```

(the repo-wide total moved 3141 → 4694 → 4258 during the session; every one of those rows is in another
partition, and none is in `🌎️hub`.)

| Scope | findings | added the format entity | annotated `x-semio-formats` |
|---|---|---|---|
| `hub.inference` | 39 | 19 | 20 |
| `hub.artifact-authority.trusted-catalog` | 14 | 14 | 0 |
| `hub.artifact-authority.creation` | 12 | 0 | 12 |
| `hub.auth` | 3 | 0 | 3 |

Nothing was deleted and no export was added or removed: the catalog is generated and lists the export
set, so adding or dropping a `$defs` key here would only produce a `schema-export-unknown` until WP2
regenerates. Every decision below is therefore "declare the entity that already exists" or "state the
truth about the formats it exists in".

## 2. The rule used, stated once

An export gets the missing format entity when **a Rust type that IS that contract already exists** in the
crate — the contract is transported, and the only defect is that the scope module does not name it. It is
annotated when Rust holds **no type of that shape**: the contract is validated (fixtures, oracles, laws)
but never encoded or decoded by a Rust struct. A same-named Rust type of a *different* shape (a SQLite
row, a borrowed binary cursor) is not the export and was never used to justify a Rust entity — that would
have been the fabrication the brief forbids.

Every annotation was checked in both directions, which the harness enforces
(`📓️wp1c-harness-rules.md` §2.1): an annotated export must exist in **exactly** the named formats, so
each of the 35 annotated rows was also verified absent from the formats it does not name.

## 3. `hub.inference` — 39 rows (module provides `🔣️jsonschema` `🦀️rust` `🟦️typescript`; all 39 were `🦀️rust`)

### 3.1 Rust entity added — 19

| Export | Rust entity it now names | how |
|---|---|---|
| `InferenceApprovalRequestV1` | `✅️approval/🦀️.rs:9` `pub struct` (`decode` is the approval decoder) | `pub use` → `pub type` (see §3.3) |
| `GisMapDocumentFrontierV1` | `os.directory` `CheckpointPublicationFrontierV1` | `pub use` → `pub type` |
| `GisMapApprovalUndoHandleV1` | `os.directory` same-named `pub struct` | `pub use` → `pub type` |
| `GisMapApprovalUndoRequestV1` | ″ | `pub use` → `pub type` |
| `GisMapApprovalUndoReceiptV1` | ″ | `pub use` → `pub type` |
| `GisMapFrozenExecutionProtocolV1` | was `struct GisMapFrozenExecutionProtocolV1` (private) in `📇️catalog/🦀️.rs` | moved into the scope module as `pub`, catalog imports it |
| `GisMapFrozenPackageV1` | ″ | moved |
| `GisMapFrozenArtifactV1` | ″ | moved |
| `GisMapFrozenSurfaceV1` | ″ | moved |
| `GisMapFrozenGrantV1` | ″ | moved |
| `GisMapFrozenBindingV1` | was `struct GisMapFrozenBindingProjectionV1` | moved **and renamed to the export id** |
| `InferenceCatalogServiceV1` | was `struct GisMapFrozenServiceV1` (identical 12 fields, same order) | moved and renamed to the export id |
| `InferenceJobReceiptV1` | was `pub struct InferenceJobReceiptDtoV1` in `🏃️runtime/🦀️.rs` | moved, `Dto` dropped |
| `InferenceProgressV1` | was `InferenceProgressDtoV1` | moved |
| `InferenceEventV1` | was `InferenceEventDtoV1` | moved |
| `InferenceEventPageV1` | was `InferenceEventPageDtoV1` | moved |
| `GisMapInferencePreviewV1` | was `GisMapInferencePreviewDtoV1` | moved |
| `InferenceApprovalReceiptV1` | was `InferenceApprovalReceiptDtoV1` | moved |
| `GisInferenceCheckpointControlFrameV1` | was the private `#[cfg(feature = "test-support")] struct InferenceCheckpointControlFrameV1` in `🏃️runtime/🦀️.rs` | moved and renamed; the `cfg` is kept (see §8.8) |

Each move preserves the derive list, the serde attributes and the **field order** verbatim, because
`GisMapFrozenBindingV1`'s canonical serialization is the binding digest
(`GIS_MAP_BINDING_DOMAIN = b"semio.hub.gis-map-frozen-binding/v1\0"`, `📇️catalog/🦀️.rs`). The six wire
DTOs are `Serialize`-only structs whose `schema` const is the contract identity, which is what proves the
export and the Rust type are the same contract:

```
🏃️runtime/🦀️.rs  InferenceJobReceiptV1     { schema: "semio.hub.inference-job-receipt/v1",    … }
                 GisMapInferencePreviewV1  { schema: "semio.hub.gis-map-inference-preview/v1", … }
                 InferenceApprovalReceiptV1{ schema: "semio.hub.inference-approval-receipt/v1",… }
                 InferenceEventPageV1      { schema: "semio.hub.inference-job-events/v1",      … }
```

Two consequential renames fell out of the moves, both inside `🌎️hub/💡️inference/`:

- `🪶️sqlite/🦀️.rs`'s **different** row types that happened to share two of those names became
  `InferenceLedgerJobRowV1` (was `InferenceJobReceiptV1`: `{job_id, identity_digest, expires_at_ms}` — a
  ledger row, not the wire receipt) and `InferenceLedgerEventPageV1` (was `InferenceEventPageV1`: rows,
  not the wire page). `🪶️sqlite/🦀️.rs` glob-imports `super::schema::*`, so leaving both names would have
  meant two different types of the same name shadowing each other in one file.
- `📇️catalog/🦀️.rs`'s `GisMapFrozenDialectV1` is deleted: it was a field-for-field duplicate of the
  module's own `InferenceParentDialectV1`, which the frozen binding now uses directly.

### 3.2 Annotated `"x-semio-formats": ["🔣️jsonschema", "🟦️typescript"]` — 20

All 20 exist in the module's `🟦️.ts` as an exported type **and** a `parse<Export>()` (verified export by
export, §7.5). None exists as a Rust type of that shape.

| Export | why Rust holds no type of this shape |
|---|---|
| `InferenceServerIdV1` | a string law; Rust's counterpart is the predicate `fn server_id(&str) -> bool`, not a type |
| `InferenceDocumentScopeV1` | Rust uses `os.directory`'s `DocumentScope`; contract §A forbids a scope restating another scope's `$defs`, and hub must not mint a second Rust shape for it |
| `InferenceLifecycleKindV1` | the ten event kinds; the Rust wire field is `InferenceEventV1.kind: String` — no enum exists |
| `InferenceLimitsV1` | a limits table; Rust carries the same numbers as `const`s (`🧬️schema/🦀️.rs`, `🏃️runtime/🦀️.rs`), never as a struct |
| `InferenceCommandLimitsV1` | ditto, `✉️command/🦀️.rs:5-8` `COMMAND_MAX_BYTES`/`TEXT_MAX_BYTES`/`DEPENDENCY_MAX_COUNT`/`PAYLOAD_MAX_BYTES` |
| `InferenceApprovalOutboxV1` | the JSON export is an **oracle expectation** (`proposal: const "ledger-only-proposal"`, `preparedCount: const 1`, `reconciledCount: const 1`, `commandHex`). `🪶️sqlite/🦀️.rs`'s same-named `pub struct` is `{job_id, mutation_id, command_hash, proposal_hash, prepared_at_ms, command}` — a different shape, and it is never serialized to this document |
| `GisMapApprovalUndoTargetV1` | a durable private record. `🪶️sqlite/🦀️.rs:135` has a `pub(crate)` struct with different fields (`user_id`/`session_id` vs `ownerUserId`/`ownerSessionId`, plus `original_command` bytes) and no `schema` field; the export's `schema` const is used by Rust only as a hash domain string (`🪶️sqlite/🦀️.rs:823`) |
| `GisInferenceCheckpointControlDirectionV1` | the two direction tokens; Rust has no enum — direction is implicit in which end reads the descriptor |
| `InferenceMapBoundsV1`, `InferenceMapSummaryV1` | the expected-inference projection the ledger oracle validates; no Rust struct anywhere in `🌎️hub` |
| `InferenceHybridLogicalTimestampV1` | Rust uses the framework's `protocol::HybridLogicalTimestamp`; hub restating it would be a second authority |
| `InferenceCommandV1`, `InferenceCommandPayloadV1` | the **logical** command the oracle validates. Rust's counterpart is `✉️command/🦀️.rs`'s `pub(super) struct CanonicalInferenceCommandV1<'a>` — a borrowed cursor over the length-prefixed **binary** envelope, with `dependencies: [&str; 64] + dependency_count` and flattened `diff_schema`/`diff_payload`/`inverse_schema`/`inverse_payload`. It decodes the same bytes but it is not this shape, so naming it as the export would have been a false claim. Flagged in §8.1 |
| `InferenceWalChainPolicyV1` | a chain policy law (`hashAlgorithm`, `requiredFlags`, `recordDigest`, `commitDigest`); Rust uses `db::wal` constants |
| `InferenceWalTargetV1` | the JSON export is `{scope, documentKey, generation, jobId, proposalHash, maximumRecords}`; `🧾️wal/🦀️.rs:37`'s same-named `pub struct` is a different 9-field shape (`mutation_id`, `command_hash`, `actor`, `receipt`, no `documentKey`) and is never serialized |
| `InferenceCatalogOwnerV1`, `…FrontierV1`, `…DescriptorV1`, `…PackageV1`, `…SelectionV1` | the catalog-projection oracle's fixture contract. Rust builds identity from `os.directory`'s `DocumentDescriptor` and the framework's `PackageDescriptor`; there is no hub struct for the projection. (`InferenceCatalogServiceV1` is the one member of this family that *does* exist in Rust and is therefore NOT annotated) |

### 3.3 `pub use` vs `pub type` — a three-way vocabulary disagreement, flagged

The five re-exports were already `pub use` before this work and were still reported incomplete. The three
authorities disagree:

- `📋️execution-contract.md` §A, export presence: Rust `pub struct|enum <Export>`.
- `📋️execution-contract.md` §A (amended during this session, line 26): "a Rust re-export is `pub use`".
- the harness `declaresSchemaExport` (`…/🧪️test/📦️packages/🟦️typescript/🟦️.ts`):
  `^\s*pub\s+(?:struct|enum|type)\s+<Name>\b` — `pub use` does **not** match.

`pub type X = <real path>;` is the only form all three instruments accept as present *and* that is a
genuine re-export rather than a restatement, so the five became `pub type`. See the cross-partition
request in §8.4.

### 3.4 One real drift repaired: `InferenceEventPageV1.schema`

`🔣️.json` and `🟦️.ts` pinned `"semio.hub.inference-event-page/v1"`. The Rust that actually writes the
wire pins `"semio.hub.inference-job-events/v1"` (`🏃️runtime/🦀️.rs`, two construction sites), and the
binary's own law asserts the same value against a real HTTP response
(`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:1388`). No fixture carries the field, so nothing caught it. The JSON
and the TypeScript were corrected to the transported value; the runtime was not touched.

## 4. `hub.artifact-authority.trusted-catalog` — 14 rows, all given the Rust entity

The module docstring already claimed to be the "sole owner of the trusted-catalog … contract" while the
whole bundle format was decoded by **private** `Bundle*` structs in the parent
`🔏️trusted-catalog/🦀️.rs`. Those structs are the contract — `Bundle` is what `serde` parses the trusted
bundle file into at hub start-up — so they were renamed to their export ids and moved into the scope
module as `pub`:

| Export | was | now |
|---|---|---|
| `TrustedBundleV1` | `struct Bundle` | moved to `🧬️schema/🦀️.rs`, `pub` |
| `TrustedBundleProfileV1` | `BundleProfile` | ″ |
| `TrustedBundleProfileOpenTargetV1` | `BundleProfileOpenTarget` | ″ |
| `TrustedBundlePackageV1` | `BundlePackage` | ″ |
| `TrustedBundleIdentityV1` | `BundleIdentity` | ″ |
| `TrustedBundleCodecV1` | `BundleCodec` | ″ |
| `TrustedBundleOpenTargetV1` | `BundleOpenTarget` | ″ |
| `TrustedBundleGrantV1` | `BundleGrant` | ″ |
| `TrustedBundleFileV1` | `BundleFile` | ″ |
| `TrustedBundleComponentV1` | `BundleComponent` | ″ |
| `TrustedBundleBrowserActorV1` | `pub(super) enum BundleBrowserActor` in `🌐️browser-actor/🦀️.rs` | renamed + `pub`, re-exported by the module as `pub type` |
| `TrustedCatalogRelativePathV1` | `pub(super) struct TrustedCatalogRelativePath` in `🛡️opened-root/🦀️.rs` | renamed + `pub`, re-exported as `pub type` |
| `TrustedBundleParentDialectV1` | the field type `semio_framework::ArtifactDialect` (`{artifact_kind, standard, subset}`) | `pub type` re-export — the framework owns the shape, hub must not restate it |
| `TrustedBundleExecutionProtocolV1` | `semio_framework::ExecutionProtocol` (`{app_channel_version}`) | `pub type` re-export |

Two enum helpers that are **not** exports (`role`, `rendererTarget` are inline enums in the JSON) moved
with them and were renamed out of the `Bundle*` namespace: `TrustedBundlePackageRole`,
`TrustedBundleOpenRole`, `TrustedBundleRendererTarget`. Nothing was annotated in this scope: every one of
its 17 exports now exists in both formats it provides.

## 5. `hub.artifact-authority.creation` (12) and `hub.auth` (3) — annotated

### 5.1 `hub.artifact-authority.creation` — all 12 → `["🔣️jsonschema"]`

The module provides `🔣️jsonschema` + `🦀️rust`, and its `🦀️.rs` is the **production** creation state
machine (`ArtifactCreationIntentV1`, `…PreparedV1`, `…ReceiptV1`, `…FactBodyV1`, `…FactV1`,
`…OperationV1`, `DocumentGenesisAppendV1`, …). The twelve `$defs` are a different thing: law tables the
hub oracle validates fixtures against, bound as `schema://hub.artifact-authority.creation/<Export>` at
`📜️script.ts:14620-14740` and nowhere else.

- `ArtifactCreationPhaseV1` is the wire phase vocabulary, and its Rust enum is `os.directory`'s
  `SpaceArtifactCreationStatusV1` (`…/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🦀️.rs`) — another
  scope's export, which hub must not restate.
- `ArtifactCreationFactKindV1` / `…OperationStateV1` are discriminator vocabularies; Rust's
  `ArtifactCreationFactBodyV1` is a data-carrying enum, not the token list.
- `…TransitionV1`, `…CancellationDecisionV1`, `…TransactionKindV1`, `…TransactionOutcomeV1`,
  `…AcceptedRecoveryV1` are fixture-row shapes for `🧫️fixtures/{📚️operation-v1, 🧪️transaction-v1}`.
- `…HttpLimitsV1`, `…HttpRouteV1`, `…HttpAuthorityV1`, `…HttpResponseV1` are the route/authority matrix
  of `🧫️fixtures/🌐️http-owner-v1` — a table of expectations, not a transported message.

### 5.2 `hub.auth` — all 3 → `["🔣️jsonschema"]`

The module provides `🔣️jsonschema` + `🟦️typescript`; its `🟦️.ts` is 18 lines carrying the four opaque
capability string types and their `parse*`.

- `AuthLimitsV1` — a five-constant limits table.
- `AuthCapabilityVectorsV1` — the test-vector corpus shape of `🧪️fixtures/🔑️capability-v1`.
- `SocketGrantReceiptV1` — genuinely transported, but **not by this scope**: the TypeScript parser is
  `parseSocketGrantReceiptV1` in `🧰️framework/🛍️products/💻️os/🟦️.ts` and the Rust struct is in
  `…/📇️directory/🔌️client/🦀️.rs` and `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs`. Adding a second parser here
  would be exactly the dead duplicate the brief forbids. `hub.directory` also declares a
  `SocketGrantReceiptV1` — two hub scopes restating one contract. Flagged in §8.2; it cannot be fixed by
  deleting an export while the catalog is generated and stale.

## 6. Rust registration — `register_scope_schema_exports`

`semio-framework-schema-registry` is now a **normal** dependency of `semio-hub`
(`🌎️hub/📦️packages/🦀️rust/Cargo.toml`, `[dependencies]`) — registration is production code, so it may
not ride on the existing test-only `semio-framework-schema` dev-dependency. The registry crate is the
os-kernel-free leaf (contract §C), so this adds no os-kernel edge.

| Scope | declaration site | exports | `ALL_LEAVES` | `VALIDATED_ONLY` |
|---|---|---|---|---|
| `hub.inference` | `🌎️hub/💡️inference/🧬️schema/🦀️.rs` | 45 | 25 (`rust` + `typescript` + `json_schema`) | 20 (`typescript` + `json_schema`) |
| `hub.artifact-authority.creation` | `…/🌱️creation/🧬️schema/🦀️.rs` | 12 | 0 | 12 (`json_schema` only) |
| `hub.artifact-authority.trusted-catalog` | `…/🔏️trusted-catalog/🧬️schema/🦀️.rs` | 17 | 17 (`rust` + `json_schema`) | 0 |

Each is a sibling declaration in the scope's own `🧬️schema/🦀️.rs` with `include_str!` of the sibling
leaves, following the `framework.ui.contract` precedent
(`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs`), and each carries the same runtime law:
`registers_and_resolves_exactly_the_annotated_formats` registers, asserts
`scope_schema_exports_registered(SCHEMA_SCOPE)`, and asserts that for every export and every one of the
five formats, `resolve_schema_export` succeeds **iff** the module document's own `"x-semio-formats"` (or,
unannotated, the set of formats the module carries a file for) names it. The law reads the annotation
from `include_str!("🔣️.json")` rather than restating it, so the registration and the annotation cannot
drift apart silently.

**Only three of the ten hub scopes are registered, and that is deliberate.** The other seven
(`hub.admin`, `hub.artifact-authority`, `hub.artifact-authority.native-openable-provider`, `hub.auth`,
`hub.directory`, `hub.lag-rebootstrap`, `hub.local-bootstrap`) carry no `🦀️.rs` in their `🧬️schema/`
module. Adding one purely to host a registration call would make the catalog record `🦀️rust` as a format
those scopes *provide* (contract §B: "a scope provides the formats whose files exist in its module"), and
every one of their 62 exports would then need `"x-semio-formats": ["🔣️jsonschema"]` (or
`["🔣️jsonschema","🟦️typescript"]` for `hub.auth`) to stay honest — a module declaring a Rust file that
declares nothing. That is a format-set change with a `schema generate` consequence, so it is a
coordinator call, not a worker call: §8.3.

## 7. Verification — real output

All runs `cd /Users/ueli/Documents/semio`, `SEMIO_TEST_ARTIFACT_DIR` and `CARGO_TARGET_DIR` under
`<ticket>/🗑️generated/wp4c-hub`.

### 7.1 The partition gate

```
$ bun <ticket>/wp4c-hub-probe.ts .
[wp4c-hub] 0 finding(s) of 4258 repo-wide

$ bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts test schema --under 🌎️hub
[test schema] 0 invariant finding(s) over 🌎️hub
[test schema] 0/0 schema-bound fixture(s) reached their declared stage
```

(the second is the brief's command; §0 explains why it cannot see this code class either way.)

### 7.2 `cargo check -p semio-hub --no-default-features --features sqlite --lib`

**0 errors, 15 warnings — exactly the baseline count, with no warning this work introduced.**

```
$ CARGO_TARGET_DIR=<scratchpad>/target-w4 RUSTC_WRAPPER="" \
    cargo check -p semio-hub --no-default-features --features sqlite --lib
warning: `semio-hub` (lib) generated 15 warnings (run `cargo fix --lib -p semio-hub` to apply 2 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 5m 26s
$ grep -c '^error' cargo-4.txt
0
$ diff <(grep '^warning: ' cargo-baseline.txt | sort) <(grep '^warning: ' cargo-4.txt | sort)
2d1
< warning: `semio-framework-plugin` (lib) generated 1 warning …
41d39
< warning: unused import: `DocumentBackboneBindingStateV1`
```

The baseline was taken on the unmodified tree at the start of this session (same command, same private
target dir): `warning: semio-hub (lib) generated 15 warnings … Finished in 2m 46s`, 0 errors. The two
lines the diff shows are warnings that **disappeared** (peer fixes landing during the session); nothing
was added. Four intermediate runs were needed — each of the three warnings this work briefly introduced
(`unused import: serde::Deserialize` after the bundle structs moved out of `🔏️trusted-catalog/🦀️.rs`,
`constant ALL_LEAVES is never used` in the all-annotated creation scope, and three
`constant MODULE_JSON is never used` because it is read only by the `#[cfg(test)]` law) was fixed rather
than left. Full log: `🗑️generated/wp4c-hub/cargo-check.txt`.

**The three registration laws were written but NOT executed.** `--lib` does not build `#[cfg(test)]`
code, and the brief caps cargo at exactly this command, so `registers_and_resolves_exactly_the_annotated_formats`
is unrun in all three scopes. Nothing in this report claims it passes. (A concurrent repo-wide test-layout
tool extracted each `#[test]` body into a sibling
`🧪️tests/🔬️scope-schema-export-law-standalone/🦀️.rs` and `include!`s it back — that peer edit was left in
place, and it is also unrun.)

**What this command does and does not cover.** `--no-default-features --features sqlite` compiles the
scope modules (`🧬️schema/🦀️.rs` of all three registered scopes, `🪶️sqlite`, `🧾️wal`,
`🔏️trusted-catalog` and its four leaves) but **not** `💡️inference/📇️catalog` or `💡️inference/🏃️runtime`,
which are `#[cfg(feature = "native-artifact-execution")]`. Those two files were edited (the frozen-binding
structs and the six wire DTOs moved out of them, with their imports rewritten), and the brief caps cargo
at exactly this command, so their compilation is **not** claimed here. Every edit in them is an import
list plus a mechanical identifier rename; the moved definitions themselves are in the scope module and are
compiled above. Re-run when the budget allows:

```
CARGO_TARGET_DIR=<private> RUSTC_WRAPPER="" cargo check -p semio-hub --lib
CARGO_TARGET_DIR=<private> RUSTC_WRAPPER="" cargo test -p semio-hub --lib scope_schema_export_law
```


### 7.3 Hub oracle commands

Sequence run with `SEMIO_TEST_ARTIFACT_DIR=<ticket>/🗑️generated/wp4c-hub` and
`CARGO_TARGET_DIR=$SEMIO_TEST_ARTIFACT_DIR/cargo`; full log in `🗑️generated/wp4c-hub/oracles.txt`.

```
gis-map-frozen-binding-check                        exit=0
  gis-map-frozen-binding-check: checks=54 clean; no route, provider, inference execution, or publication claim
trusted-catalog-opened-root-check                   exit=0
  trusted-catalog-opened-root: scope-exports=2 paths=15 denials=7 same-handle=1 source=14 startup=no-ambient-path
document-browser-actor-identity-check               exit=0
  trusted-browser-actor-catalog: scope-export=TrustedBundleBrowserActorV1 cases=26 bodies=8 raw-lengths=8 WebCrypto=1
  browser-document-open-oracle: ajv=1 paths=3 installed-target=1 scope-keys=2 authority=1 exchange=1 websocket=1 rust-worker-bypass=denied hostile=25 bound=65536 redaction=6 passed
  execution-target-lease-oracle: ajv=1 positive=1 manifest-fields=49 byte-vectors=13 lifecycle=11 hostile=73 … status=5 passed
checkpoint-publication-check                        exit=0
  checkpoint-publication-command-oracle: valid=2 rejected=12 ajv=1 typescript=1 sha256=2 …
space-administration-check                          exit=0
  space-administration-oracle: AJV=2 vectors=4 cursors=8 hostiles=9 source-hostiles=9 component-schema=9 sha256=1 binding=1
gis-map-proposal-check --source                     exit=0
  gis-inference-checkpoint-control-oracle: ajv=1 exact=2 hostile=8 fd=4 test-support-only=1 passed
  gis-map-approval-undo-oracle: AJV=1 exact=6 genesis-first=1 history=2 hostile=8
  gis-map-proposal-oracle: scope-exports=5 committer-laws=49 node-sha256=2 … process-source=26
space-artifact-creation-check                       exit=0
  [DEBUG] creation transaction oracle: scope-export=32 · accepted recovery scope-export=3 · cancellation scope-export=9 · operation transitions=30
trusted-stdio-gis-bundle-check --two-author-source  exit=0
  [DEBUG] GIS Map witness socket retirement: neutral=5 Bun=1 third-party-ws=1 exact-close-events=2
  GIS Map two-author composition fixture laws=16 hostile=10 current-coordinates=5
gis-inference-ledger-oracle                         exit=1  (blocked in os.db.storage, §8.2)
  inference-server-identity-oracle: cases=11 fields=5 composite-bounds=2 ajv+node=1
  gis-inference-ledger-oracle: traces=9 hostile=13 identity-hostile=17 sqlite-integers=6 ajv+typescript=1 hashes=6 independent-bounds=1
  inference-wal-proof-oracle: traces=17 ownership=3 binding-hostile=2 protocol-envelope=1 node-sha256=1
  inference-command-oracle: vectors=20 ajv=1 node+webcrypto-hash=2
  inference-approval-request-oracle: valid=1 hostile=14 byte-boundaries=2 ajv+node=1
  inference-author-oracle: cases=16 accepted=1 ajv+sqlite=1
  inference-wal-chain-oracle: exact=14 hashing-ownership=3 retained-boundaries=2 ajv=1 crc-valid=14 blake3-known-answer=1
  inference-catalog-projection-oracle: exact=12
  trusted-catalog-identity-oracle: exact=6 canonical-kind=1 descriptor-sha256=1 package-ref-blake3=distinct
  gis-native-codec-oracle / gis-controlled-proposal-oracle / gis-native-provider-selection-oracle: green
  error: memory backing schema accepted altered bounds
        at proveMemoryBackendBackingFixture (…/🌎️hub/📦️packages/🦀️rust/📜️script.ts:10826:59)
```

Every `hub.*` oracle in that last command prints green; it then aborts on an **os-partition** contract,
see §8.2. `trusted-stdio-gis-bundle-check` failed once mid-sweep with
`ENOENT … mkdtemp '<ticket>/🗑️generated/wp4c-hub/gis-socket-retirement-…'` — a concurrent agent had
deleted `🗑️generated/` under a running command; recreating the directory and re-running gave the green
output above. (This session deleted nothing.)

Two hub laws had to be repaired on the way, both readers in this partition and both broken by the
concurrent test-layout refactor or by this work's own move:

- `proveGisInferenceCheckpointControlFixture` asserted `serde(deny_unknown_fields, rename_all =
  "camelCase")` against `💡️inference/🏃️runtime/🦀️.rs`. The frame TYPE is now a scope export declared in
  `🧬️schema/🦀️.rs`, so the law was split: the type and its serde closure are asserted against the scope
  module, the framing (`open_inherited`, the two frame functions, the 256-byte bound, the checkpoint call)
  against the runtime. This is this work's own consequence and was fixed here.
- `proveGisMapProposalApprovalFixture` asserted two `#[cfg(test)]` law **names**
  (`gis_map_approval_ingress_holds_sorted_hub_authority_without_outer_document_write`,
  `gis_map_applied_checkpoint_notifies_two_peers_with_one_exact_rebootstrap_pair`) against `🚀️bin.rs`; the
  END-TO-END-TESTING-REFACTOR ticket has since moved them into `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`. The two
  law names now read the whole `🌎️hub/🧪️tests/**` tree through the existing `moduleRustSource()` helper,
  so a further test-layout move cannot red them; the six production symbols still read `🚀️bin.rs`. Same
  remedy `📓️wp4b-hub.md` §5.3 used for the previous two occurrences of this hazard.


### 7.4 Hub vitest

```
$ cd 🌎️hub/📦️packages/🟦️typescript && bun ./📜️script.ts test long
 RUN  v4.1.10 /Users/ueli/Documents/semio/🌎️hub/📦️packages/🟦️typescript
 Test Files  1 passed (1)
      Tests  11 passed | 1 skipped (12)
   Duration  41.96s
```

(the skip is the `HUB_E2E`-gated journey.) This needed a one-token repair first, and the cause is not
this ticket: `runVitest`'s default config path in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:2578` changed to
`🧪️tests/🟦️.ts` (file mtime 19:48, before this session), while the hub package's config is
`vitest.config.ts` and no package in the repo has moved yet:

```
$ bun ./📜️script.ts test quick
✘ [ERROR] Could not resolve ".../🌎️hub/📦️packages/🟦️typescript/🧪️tests/🟦️.ts"
$ grep -rn "runVitest(" --include=📜️script.ts … | sed 's/.*runVitest(/runVitest(/' | sort | uniq -c
  35 runVitest(this.root, rest, "vitest.config.ts");
   4 runVitest(this.root, rest);
```

The hub script now passes `"vitest.config.ts"` explicitly like the other 35 callers. The remaining three
default callers are in other partitions and are red for the same reason — §8.6.

`test quick` still exceeds its 30 s budget on this loaded machine (`[budget] … exceeded 30000ms — killed`)
and passes in 42 s at `long`; `📓️wp4b-hub.md` §7.8 already flagged that this target's budget level is
wrong. Not changed here — the budget level is a repo-wide policy call.


### 7.5 Per-export presence, checked mechanically

`wp4c-hub-presence.sh` (kept in this ticket folder) re-derives the harness's own presence rules
(`declaresSchemaExport`: Rust `^\s*pub\s+(?:struct|enum|type)\s+<Name>\b`, TypeScript
`^\s*export\s+(?:interface|type|const|class)\s+<Name>\b`) over the four scopes and checks the
annotation in **both** directions, plus `parse<Export>()` for every export that claims TypeScript:

```
$ bash <ticket>/wp4c-hub-presence.sh
hub.inference: 45 exports, annotated=20, formats=['🔣️jsonschema', '🟦️typescript', '🦀️rust']
hub.auth: 7 exports, annotated=3, formats=['🔣️jsonschema', '🟦️typescript']
hub.artifact-authority.creation: 12 exports, annotated=12, formats=['🔣️jsonschema', '🦀️rust']
hub.artifact-authority.trusted-catalog: 17 exports, annotated=0, formats=['🔣️jsonschema', '🦀️rust']
mismatches = 0
```

And the annotated modules still compile through the strict shared Ajv, which had to be taught the
annotation keyword (`.addKeyword({ keyword: "x-semio-formats", metaSchema: … })` in
`🌎️hub/📦️packages/🟦️typescript/🟦️.ts`, the same way
`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts:189` does). The validator was
**taught the keyword, not loosened** — `strict: true` still rejects every other unknown keyword, which is
what keeps `📓️wp4b-hub.md` §5.2's `discriminator` finding alive. The owned Rust draft-07 validator needed
no change: `✅️validator.rs:287` already has `extension if extension.starts_with("x-") => {}`.

```
$ bun -e 'import("./🌎️hub/📦️packages/🟦️typescript/🟦️.ts").then(m => …)'
OK schema://hub.inference/InferenceServerIdV1           (a $ref export with the annotation as a sibling key)
OK schema://hub.inference/InferenceCommandV1
OK schema://hub.inference/GisMapFrozenBindingV1
OK schema://hub.inference/InferenceEventPageV1
OK schema://hub.auth/AuthLimitsV1
OK schema://hub.auth/AuthCapabilityVectorsV1
OK schema://hub.artifact-authority.creation/ArtifactCreationPhaseV1
OK schema://hub.artifact-authority.trusted-catalog/TrustedBundleV1
```


## 8. Cross-partition requests and open questions

### 8.1 Coordinator — is a differently-shaped Rust decoder "the export in Rust"?

Three `hub.inference` exports were annotated validated-only although Rust **does** encode or decode the
same bytes under a different shape and a different name: `InferenceCommandV1` /
`InferenceCommandPayloadV1` (`✉️command/🦀️.rs`'s borrowed `CanonicalInferenceCommandV1<'a>`),
`InferenceWalTargetV1` (`🧾️wal/🦀️.rs`'s 9-field struct), `InferenceApprovalOutboxV1` and
`GisMapApprovalUndoTargetV1` (`🪶️sqlite/🦀️.rs`'s durable rows). Naming any of them as the export would
have made the module claim a field-for-field agreement that
`hub_inference_exports_agree_with_the_rust_decoders_field_for_field` would immediately disprove, so they
are annotated and reported instead of quietly asserted. If the coordinator rules that "Rust round-trips
this contract's bytes" is enough, the fix is a rename plus widening the JSON to the real Rust shape — a
contract change, not an annotation change. **Recommendation either way:** rename the four sibling Rust
types out of the export ids, so a same name never means two shapes inside one crate.

### 8.2 W5 os (`os.db.storage`) — `MemoryBackingV1` stopped pinning its bounds

`🌎️hub/📦️packages/🦀️rust/📜️script.ts:10826` (`proveMemoryBackendBackingFixture`) asserts the owning
scope's export rejects four altered-bound mutations. It admits all four today. Reproduced with a **stock**
strict Ajv, no hub code and no `x-semio-formats` keyword:

```
$ bun -e 'ajv.addSchema(os.db.storage module); v = getSchema("…#/$defs/MemoryBackingV1")'
fixture: true
sequentialTasks 65: true
timerDelayMs 0:     true
```

`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧬️schema/🔣️.json` now declares
`tables[].slots {minimum:1}`, `sequentialTasks {minimum:1}`, `retry.timerDelayMs {minimum:0}`,
`maximumInlineBytes {minimum:1}` — no upper bounds, no exact values. File mtime 17:39, clean in
`git status`, i.e. it predates this session. Requested change: restore the exact bounds the memory
backing actually has (or tell the hub which of the four are not contract, and the hub law will be
narrowed to the rest). This is the only thing keeping `gis-inference-ledger-oracle` red; every
`hub.*` oracle inside that command is green (§7.3). Note this is *progress*: `📓️wp4b-hub.md` §7.1 had
the same command aborting earlier, inside the gis plugin — that blocker is gone.

### 8.3 Coordinator — `register_scope_schema_exports` for the seven Rust-less hub scopes

See §6. Registering them needs a `🦀️.rs` in each `🧬️schema/` module, which makes the catalog record
`🦀️rust` as a format those scopes provide and forces `"x-semio-formats"` on all 62 of their exports for a
Rust file that declares no type. The alternative — one non-sibling registration site for all seven — is
what contract §C's "sibling declaration" wording rules out. Please pick; the hub will follow either way.

### 8.4 W1c / harness — `pub use` is a Rust re-export the instrument cannot see

`📋️execution-contract.md` §A line 26 (amended during this session) says "a Rust re-export is `pub use`",
while §A's presence list says `pub struct|enum <Export>` and `declaresSchemaExport` matches
`pub struct|enum|type`. A module that follows the amended §A literally is reported
`schema-export-incomplete`, which is exactly what five `hub.inference` rows were. Requested change: make
`declaresSchemaExport`'s Rust arm also accept `^\s*pub use .*\b<Name>\s*[,;}]` (covering
`… as <Name>` and brace lists), and align §A's presence list with whatever the harness ends up accepting.
Until then hub uses `pub type`, which all three instruments accept.

### 8.5 WP2 — `UNCATALOGUED_SCHEMA_MODULES` retired itself, as designed

The regenerated catalog now carries `s.stdio.registry`, so the bridge table in
`🌎️hub/📦️packages/🟦️typescript/🟦️.ts` hard-errored on its next read
(`s.stdio.registry is catalogued now; delete its UNCATALOGUED_SCHEMA_MODULES entry`) and has been deleted
along with its fallback branch in `schemaModulePath`. `📓️wp4b-hub.md` §6.1 and §9.2 are closed.

### 8.6 Repo tooling — `runVitest`'s default config path moved ahead of its callers

See §7.4. Four `📜️script.ts` callers rely on the default and are red since 19:48; hub's is fixed here, the
other three are in other partitions. Either the default should go back to `vitest.config.ts` until the
configs actually move, or all four callers need the explicit argument.

### 8.7 Two hub scopes still restate one contract

`hub.auth` and `hub.directory` both declare `SocketGrantReceiptV1`, and the transported twins live in
`os.directory` (`🔌️client/🦀️.rs`, `🚀️bin.rs`) and in os's TypeScript (`parseSocketGrantReceiptV1`). Contract
§A forbids a scope restating another scope's `$defs`. Fixing it means **deleting** an export, which cannot
be done independently while the catalog is generated: the harness reads the export set from
`🔣️schema-catalog.json` and would report `schema-export-unknown` until the next `schema generate`. Needs to
be one change with WP2.

### 8.8 Open — `GisInferenceCheckpointControlFrameV1` is feature-gated

It is a `#[cfg(feature = "test-support")] pub struct` in the scope module: the text grep the harness and
the registry leaf both use finds it, but under `--no-default-features` no Rust type exists. It is
unannotated (so the rule requires it in all three formats). Is a feature-gated type "present in Rust"? If
not, the honest annotation is `["🔣️jsonschema","🟦️typescript"]` and the `test-support` framing keeps its
own private type.

### 8.9 Carried forward, unchanged

`📓️wp4-hub.md` §7.1 (three os/mcp duplicates of the approval contract), §7.2
(`GisMapDocumentFrontierV1` is still `pub type … = os.directory::CheckpointPublicationFrontierV1`; if os
prefers the other name, hub will follow), §7.7 (`✅️validator.rs` still has no `if`/`then`/`else`, so four
hub modules are AJV-only), §8.1 (`🤝️two-author-shell-v1` live or planned?), §8.2 (`🧫️fixtures` vs
`🧪️fixtures` for `hub.artifact-authority.creation`), §8.3 (is `hub.admin` a scope at all?).


## 9. Files changed

### Rust — scope modules (the format entities and the registrations)

- `🌎️hub/💡️inference/🧬️schema/🦀️.rs` — 5 `pub use` → `pub type`; 7 frozen-binding types +
  `InferenceCatalogServiceV1` moved in from `📇️catalog`; 6 wire DTOs moved in from `🏃️runtime`;
  the checkpoint frame moved in from `🏃️runtime`; `//#region 🔖️ScopeSchemaExports` (45 exports) + law.
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🦀️.rs` — the 10 `Bundle*` structs moved in and
  renamed to their export ids, 4 `pub type` re-exports (browser actor, relative path, and the two
  framework-owned shapes), 3 helper enums, `//#region 🔖️ScopeSchemaExports` (17 exports) + law.
- `🌎️hub/🗿️artifact-authority/🌱️creation/🧬️schema/🦀️.rs` — `//#region 🔖️ScopeSchemaExports`
  (12 exports, all `VALIDATED_ONLY`) + law.

### Rust — consumers rewired

- `🌎️hub/💡️inference/📇️catalog/🦀️.rs` — 8 private structs deleted, imported from the scope module;
  `GisMapFrozenBindingProjectionV1` → `GisMapFrozenBindingV1`, `GisMapFrozenServiceV1` →
  `InferenceCatalogServiceV1`, `GisMapFrozenDialectV1` → the module's `InferenceParentDialectV1`;
  the now-unused `serde` import dropped.
- `🌎️hub/💡️inference/📇️catalog/🧪️tests/🔬️unit/🦀️.rs` — the renamed projection type.
- `🌎️hub/💡️inference/🏃️runtime/🦀️.rs` — 6 DTO definitions and the checkpoint frame deleted, imported
  from the scope module, `Dto` dropped from ~24 identifiers.
- `🌎️hub/💡️inference/🪶️sqlite/🦀️.rs` — `InferenceJobReceiptV1` → `InferenceLedgerJobRowV1`,
  `InferenceEventPageV1` → `InferenceLedgerEventPageV1` (the ledger rows, not the wire types).
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` — struct block moved out, imports rewritten,
  every `Bundle*` identifier renamed, the now-unused `serde::Deserialize` import dropped.
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🌐️browser-actor/🦀️.rs` (+ its `🧪️tests/🔬️unit/🦀️.rs`),
  `…/🛡️opened-root/🦀️.rs`, `…/🧪️tests/🔬️unit/🦀️.rs`, `…/🏗️test-support/🦀️.rs` — the same renames;
  `BundleBrowserActor` and `TrustedCatalogRelativePath` raised to `pub` under their export ids.
- `🌎️hub/📦️packages/🦀️rust/Cargo.toml` — `semio-framework-schema-registry` added to `[dependencies]`.

### JSON Schema and TypeScript

- `🌎️hub/💡️inference/🧬️schema/🔣️.json` — 20 `"x-semio-formats"` annotations; the
  `InferenceEventPageV1.schema` const corrected to the transported value.
- `🌎️hub/💡️inference/🧬️schema/🟦️.ts` — the same const, in the type and both parser sites.
- `🌎️hub/🔐️auth/🧬️schema/🔣️.json` — 3 annotations.
- `🌎️hub/🗿️artifact-authority/🌱️creation/🧬️schema/🔣️.json` — 12 annotations.

### Scripts

- `🌎️hub/📦️packages/🟦️typescript/🟦️.ts` — `x-semio-formats` taught to the shared strict Ajv;
  `UNCATALOGUED_SCHEMA_MODULES` and its fallback branch deleted (§8.5).
- `🌎️hub/📦️packages/🟦️typescript/📜️script.ts` — explicit `"vitest.config.ts"` (§7.4).
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — the checkpoint-frame law split between scope module and
  runtime; the two bin law names read `🌎️hub/🧪️tests/**`; `TrustedCatalogRelativePathV1` in the
  opened-root source law.

### Ticket folder (inputs, kept)

- `wp4c-hub-probe.ts` — the partition measurement §0 explains.
- `wp4c-hub-presence.sh` — the per-export both-directions presence check of §7.5.
- `📓️wp4c-hub.md` — this report.
- `🗑️generated/wp4c-hub/{cargo-check.txt, oracles.txt}` — the run artifacts quoted above. The folder was
  created by these runs and was **not** deleted (a concurrent agent deleted it once mid-sweep; it was
  recreated, not removed, here).

