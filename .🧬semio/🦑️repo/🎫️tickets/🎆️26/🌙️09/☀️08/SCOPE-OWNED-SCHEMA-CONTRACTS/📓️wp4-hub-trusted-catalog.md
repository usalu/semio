# WP4 — `hub.artifact-authority.trusted-catalog`

Owner module: `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/`
Scope id: `hub.artifact-authority.trusted-catalog`
Module `$id`: `https://semio.tech/schema/hub/artifact-authority/trusted-catalog/schema.json`
Dialect: `http://json-schema.org/draft-07/schema#` (no `prefixItems`, no `unevaluated*`, no 2020-12 `$schema`)
Root: `$ref: #/$defs/TrustedBundleV1`

## 1. Scope → export table

17 PascalCase `$defs` exports, 6 lower-camel private helpers (`identity`, `digest`, `nonzeroDigest`,
`requestId`, `relativePath`, `publicationRevision`).

| Export id | Source (fixture / former schema) | Rust struct it mirrors |
|---|---|---|
| `TrustedBundleV1` | `🧬️schema/🔣️bundle.schema.json` root | `Bundle` (`🔏️trusted-catalog/🦀️.rs`) |
| `TrustedBundleProfileV1` | `…bundle.schema.json#/$defs/profile` | `BundleProfile` |
| `TrustedBundleProfileOpenTargetV1` | `…#/$defs/profile.openTarget` | `BundleProfileOpenTarget` |
| `TrustedBundlePackageV1` | `…#/$defs/package` | `BundlePackage` |
| `TrustedBundleIdentityV1` | `…#/$defs/root`, `…#/$defs/dependency`; also `🔗️compiled-dependencies#/$defs/identity`, `🧬️stdio-gis-bootstrap#/$defs/identity` | `BundleIdentity` |
| `TrustedBundleCodecV1` | `…#/$defs/codec` | `BundleCodec` |
| `TrustedBundleOpenTargetV1` | `…#/$defs/openTarget` | `BundleOpenTarget` |
| `TrustedBundleParentDialectV1` | `…#/$defs/parentDialect`; `🧬️stdio-gis-bootstrap.openTarget.parentDialect` | `semio_framework::ArtifactDialect` as embedded in `BundleOpenTarget` |
| `TrustedBundleGrantV1` | `…#/$defs/grant`; `🧬️stdio-gis-bootstrap.openTarget.grant` | `BundleGrant` |
| `TrustedBundleFileV1` | `…#/$defs/file` | `BundleFile` |
| `TrustedBundleComponentV1` | `…#/$defs/component` | `BundleComponent` |
| `TrustedBundleExecutionProtocolV1` | `…#/$defs/package.executionProtocol`; `🧱️generation-stage.executionProtocol`; `🧬️stdio-gis-bootstrap#/$defs/package.executionProtocol` | `semio_framework::ExecutionProtocol` as embedded in `BundlePackage` |
| `TrustedBundleBrowserActorV1` | `…#/$defs/browserActor`; `🌐️browser-actor#/$defs/closed`; `🧬️stdio-gis-bootstrap#/$defs/package.browserActor` | `BundleBrowserActor` (`🔏️trusted-catalog/🌐️browser-actor/🦀️.rs`) |
| `TrustedCatalogCurrentPointerV1` | `📤️publication/📌️current.schema.json`; `🛡️opened-root#/$defs/current` | `TrustedCatalogCurrentPointerV1` (moved, now `pub`) |
| `TrustedCatalogPublicationCommandV1` | `📤️publication/📬️command.schema.json` | `TrustedCatalogPublicationCommandV1` (moved, now `pub`) |
| `TrustedCatalogPublicationReceiptV1` | `📤️publication/🧾️receipt.schema.json` | `TrustedCatalogPublicationReceiptV1` (moved, now `pub`) |
| `TrustedCatalogRelativePathV1` | `🛡️opened-root#/$defs/relativePaths[].value` (was untyped fixture data) | `TrustedCatalogRelativePath::parse` (`🛡️opened-root/🦀️.rs:19`) |

## 2. Three-implementation consolidation (publication receipt/command)

**Before — three competing implementations of the same contract:**

1. **Rust, private, non-`pub`** — `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`
   - `struct TrustedCatalogCurrentPointer` (line 177) + `decode`/`encode`
   - `fn publication_revision` (line 203)
   - `struct TrustedCatalogPublicationCommand` (line 211)
   - `struct TrustedCatalogPublicationReceipt` (line 222)
   None were `pub`; the schema strings `semio.hub.trusted-catalog-publication/v1`,
   `…-receipt/v1`, the `4_096` bound and the `"durable"` / `"replaced-unconfirmed"` tokens were
   inline literals inside `TrustedCatalogPublisher::publish_current`.
2. **TypeScript, hand-written** — `trustedBootstrapPublicationReceipt` defined *inside* the test
   script `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (was line 9939, verified by grep). It re-stated
   the schema const and the outcome enum in a single boolean expression.
3. **Six AJV micro-schemas** under `🧪️fixtures/📤️publication/` — `📬️command.schema.json`,
   `🧾️receipt.schema.json`, `📌️current.schema.json` (real contract) plus `🧬️.schema.json`,
   `📡️transport.schema.json`, `🔄️cas.schema.json`, `🔒️owner.schema.json` (pure `const` envelopes
   duplicating the fixture data byte for byte).

**After — one `pub` module:**

- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️.json` is the single normative authority.
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🦀️.rs` (new) holds the Rust projection:
  `pub struct TrustedCatalogCurrentPointerV1` (with `pub` fields and `pub fn decode`/`encode`),
  `pub struct TrustedCatalogPublicationCommandV1`, `pub struct TrustedCatalogPublicationReceiptV1`,
  `pub fn publication_revision`, and the named constants
  `TRUSTED_CATALOG_PUBLICATION_SCHEMA`, `TRUSTED_CATALOG_PUBLICATION_RECEIPT_SCHEMA`,
  `TRUSTED_CATALOG_PUBLICATION_MAX_BYTES`, `TRUSTED_CATALOG_PUBLICATION_OUTCOME_DURABLE`,
  `TRUSTED_CATALOG_PUBLICATION_OUTCOME_UNCONFIRMED`, and
  `TRUSTED_CATALOG_SCHEMA_JSON = include_str!("🔣️.json")` binding the Rust module to its own schema.
- Wired from the crate root exactly the way `🌎️hub/💡️inference/🦀️.rs` does it:
  ```rust
  #[path = "🧬️schema/🦀️.rs"]
  pub mod schema;
  use schema::{publication_revision, TrustedCatalogCurrentPointerV1, TrustedCatalogPublicationCommandV1, TrustedCatalogPublicationReceiptV1, TRUSTED_CATALOG_PUBLICATION_MAX_BYTES, …};
  ```
  All inline literals in `publish_current` now reference the module constants.
- The script's hand-written decoder is a **thin consumer** of the module export. It keeps only the
  bytes/canonical-framing and request-binding checks it alone can do, and delegates the shape:
  ```ts
  if (!trustedCatalogPublicationReceiptContract()(row)) throw new Error("trusted publication receipt does not satisfy its scope-owned contract; reconcile current");
  ```
  `trustedCatalogPublicationReceiptContract()` resolves
  `schema://hub.artifact-authority.trusted-catalog/TrustedCatalogPublicationReceiptV1` via
  `hubSchemaExport`, whose independent third-party AJV (`ajv`, draft-07) compilation is the
  cross-check CLAUDE.md requires — and it is compiled **from the module**, not from a fixture path.
  The decoder's signature was deliberately left unchanged (it is asserted verbatim by the
  source-conformance laws in `proveTrustedGisPublicationFixture` and at `📜️script.ts:12455`), so it
  derives the repo root from `import.meta.dir`.

## 3. `🧬️schema/🔣️bundle.schema.json` consumer finding

WP0 (`📓️wp0-hub.md:196,219` and `📊️wp0-hub.json:211,220`) flagged this file as **orphaned — "zero
consumers found anywhere in the repo"**. **That finding is wrong.** Proven by:

```
$ grep -rn 'bundle.schema.json' . --exclude-dir=target --exclude-dir=node_modules --exclude-dir=.git
🌎️hub/📦️packages/🦀️rust/📜️script.ts:8191:  const schema = JSON.parse(readFileSync(join(root, "🧬️schema/🔣️bundle.schema.json"), "utf8"));
🌎️hub/📦️packages/🦀️rust/📜️script.ts:8476:  ajv.addSchema(JSON.parse(readFileSync(join(root, "../../🧬️schema/🔣️bundle.schema.json"), "utf8")));
🌎️hub/📦️packages/🟦️typescript/🤝️index.test.ts:235:    const schema = JSON.parse(readFileSync(join(catalogRoot, "🧬️schema", "🔣️bundle.schema.json"), "utf8"));
(remaining hits are ticket notes under .🧬semio/🦑️repo/🎫️tickets)

$ grep -rn 'trusted-catalog-bundle-v2' . --exclude-dir=target --exclude-dir=node_modules --exclude-dir=.git
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️bundle.schema.json:3   (its own $id)
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🧬️stdio-gis-bootstrap/🧬️.schema.json:261  ($ref .../browserActor)
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🌐️browser-actor/🧬️.schema.json:10       ($ref .../browserActor)
```

So it had **three file readers and two cross-schema `$ref` consumers**. WP0's evidence line
(`grep -rl 'trusted-catalog-bundle-v2' repo-wide returns only this file itself`) missed both `$ref`
sites and did not grep the filename at all.

Action taken: its contents were folded into `🧬️schema/🔣️.json` as the 13 `TrustedBundle*V1` exports
(2020-12 → draft-07: the `browserActor` `oneOf` branch that used `$ref` + `unevaluatedProperties:false`
over the os.directory `identity` def is now a self-contained closed variant mirroring this scope's own
`BundleBrowserActor` Rust enum field-for-field), the file was deleted, and all three readers were
rewired.

## 4. Files created / moved / deleted

**Created**
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️.json` — the scope module (17 exports).
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🦀️.rs` — the Rust projection.

**Deleted (18 files, all superseded — no aliases, redirects or compatibility layers left)**
- `🧬️schema/🔣️bundle.schema.json`
- `🧪️fixtures/📤️publication/{🧬️,📌️current,📡️transport,📬️command,🔄️cas,🔒️owner,🧾️receipt}.schema.json` (7)
- `🧪️fixtures/{🌐️browser-actor,🔗️compiled-dependencies,🛡️opened-root,🤝️gis-map-collaboration,🧊️codec-source,🧬️retained-gis-browser,🧬️stdio-gis-bootstrap,🧱️generation-stage,🪪️identity-roles}/🧬️.schema.json` (9)
- (`🧪️fixtures/📤️publication/🔣️.json` stays as data; `🧪️fixtures/👥️two-package/🔣️.json` untouched.)

**Fixture data updated (negative cases became declared expectations)**
- `🧪️fixtures/🌐️browser-actor/🔣️.json` — every `cases[]` entry lost its `shape: boolean` and gained
  `expectation: { stage: "contract", result, code }`; every `rawLengths[]` entry gained the same.
  26 + 8 expectations, codes e.g. `path-escape`, `zero-digest`, `unknown-field`,
  `duplicate-interface`, `length-out-of-bounds`, `non-integer-length`, `foreign-actor-schema`.
  All fields the Rust test `trusted_browser_actor_metadata_and_generation_match_neutral_corpus` reads
  (`id`/`kind`/`renderer`/`set`/`remove`/`accepted`/`token`) are preserved.
- `🧪️fixtures/📤️publication/🔣️.json` — gained `commandCases` (7) and `receiptCases` (6), each with
  `expectation`. The publication command/receipt hostiles that used to be an inline TS array are now
  fixture data. Three receipt hostiles are `stage: "domain"` (they are contract-valid but
  request-/pointer-/revision-mismatched) and three are `stage: "contract"`.

**Rust rewired**
- `🔏️trusted-catalog/🦀️.rs` — publication family removed and imported from `schema`; `use serde::{Deserialize, Serialize}` → `use serde::Deserialize` (Serialize moved with the receipt).
- `🔏️trusted-catalog/🧪️tests/📤️publication/🦀️.rs:96` — `TrustedCatalogCurrentPointer::decode` → `TrustedCatalogCurrentPointerV1::decode`.
- No `include_str!` in the partition ever pointed at a `.schema.json` (verified by grep) — only at
  fixture **data** `🔣️.json` / `📡️transport.json` / `🔒️owner.json` / `🔄️cas.json`, all of which survive.
- No `📋️project.json`, `.vscode/launch.json` or `package.json` reference to any moved path (verified by grep).

## 5. Prove-function rewiring (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`)

Every function below no longer reads a fixture-local schema; each binds `schema://` exports and
replaces the deleted envelope with explicit literal/`Set`/exhaustive-enumeration assertions.

| Prove function | Was | Now |
|---|---|---|
| `proveTrustedBrowserActorCatalogFixture` | `🔣️bundle.schema.json` + os browser-actor `addSchema` + `🌐️browser-actor/🧬️.schema.json` | `TrustedBundleBrowserActorV1` + `assertHubFixtureExpectation` × 34; envelope replaced by exact key order, schema const, hex patterns, `26/8/8` counts, unique-id `Set` checks, exhaustive `kind`/`renderer` enumeration |
| `proveTrustedCatalogOpenedRootFixture` | `🛡️opened-root/🧬️.schema.json` | `TrustedCatalogCurrentPointerV1` + `TrustedCatalogRelativePathV1` (15 paths cross-checked against the production predicate); `limits` deep-equal, `openedHandle` key order + bounds, denial list already exhaustive |
| `proveTrustedGisRetainedBrowserFixture` | `🧬️retained-gis-browser/🧬️.schema.json` | pinned `contract` object deep-equal, pinned `nonclaims` list, key-order assertion (no contract shape in this fixture) |
| `proveTrustedGisMapCollaborationContractFixture` | `🤝️gis-map-collaboration/🧬️.schema.json` (456 lines, 100 % `const`) | one pinned specification literal + key-order guard; the 8 existing hostiles still must fail it |
| `proveTrustedStdioGisBootstrapFixture` | `🧬️stdio-gis-bootstrap/🧬️.schema.json` + 2 `addSchema` | `TrustedBundleIdentityV1`, `TrustedBundleBrowserActorV1`, `TrustedBundleExecutionProtocolV1`, `TrustedBundleParentDialectV1`, `TrustedBundleGrantV1`; pinned `limits`, `framing` (incl. NUL domains), 8 cancellation stages, 19 hostile ids, 6 rotation source cases, 7 rotation write cases |
| `proveTrustedCompiledDependenciesFixture` | `🔗️compiled-dependencies/🧬️.schema.json` | `TrustedBundleIdentityV1` × 6; pinned `publicationFiles`, `descriptorPreviewCases`, `consumerCatalogCases`, `atomicCases`, exhaustive 24-value `change` enumeration over 25 rows with a `change/owner` uniqueness `Set` |
| `proveTrustedGenerationStageFixture` | `🧱️generation-stage/🧬️.schema.json` | `TrustedBundleExecutionProtocolV1` × 3; pinned `descriptors`, `componentHex`, exhaustive 28-case `change` list, accepted-subset assertion |
| `proveTrustedBootstrapCodecCaptureFixture` (`🧊️codec-source`) | `🧊️codec-source/🧬️.schema.json` | pinned key order, bounds, exhaustive 15-case `change` list, per-row key-order guard |
| `proveTrustedCatalogIdentityRolesFixture` | `🪪️identity-roles/🧬️.schema.json` | `TrustedBundleV1` applied to the real `👥️two-package` bundle it dereferences; pinned source path, hash-role names, exhaustive 6-case `change` list |
| `proveTrustedPublicationFixture` | 6 AJV schema files | `TrustedCatalogPublicationCommandV1` / `…ReceiptV1` / `TrustedCatalogCurrentPointerV1`; `transport`, `owner`, `cas` and the top-level fixture pinned by deep-equal; command/receipt hostiles driven from fixture `expectation`s |
| `trustedBootstrapPublicationReceipt` | inline schema-const + outcome-enum literals | thin consumer of `TrustedCatalogPublicationReceiptV1` (see §2) |

Also rewired: `🌎️hub/📦️packages/🟦️typescript/🤝️index.test.ts` — "validates the neutral immutable
trusted-catalog bundle" now uses the file's `hubSchemaExport(root, …/🧬️schema/🔣️.json)("TrustedBundleV1")`
helper (added there by the concurrent `hub.lag-rebootstrap` worker) instead of `Ajv2020` + the deleted
bundle schema + the os.directory `addSchema`.

## 6. Verification (real output)

### 6.1 Standalone AJV draft-07 (`ajv`, **not** `ajv/dist/2020`), `{strict:true, allErrors:true}`, `addSchema` + `getSchema(id + "#/$defs/" + export)`

```
$ cd /Users/ueli/Documents/semio && bun <scratchpad>/verify.mjs
module https://semio.tech/schema/hub/artifact-authority/trusted-catalog/schema.json
dialect http://json-schema.org/draft-07/schema#
exports(17) TrustedCatalogRelativePathV1 TrustedBundleIdentityV1 TrustedBundleCodecV1 TrustedBundleParentDialectV1 TrustedBundleGrantV1 TrustedBundleOpenTargetV1 TrustedBundleFileV1 TrustedBundleComponentV1 TrustedBundleExecutionProtocolV1 TrustedBundleBrowserActorV1 TrustedBundlePackageV1 TrustedBundleProfileOpenTargetV1 TrustedBundleProfileV1 TrustedBundleV1 TrustedCatalogCurrentPointerV1 TrustedCatalogPublicationCommandV1 TrustedCatalogPublicationReceiptV1
private-helpers(6) identity digest nonzeroDigest requestId relativePath publicationRevision
dialect-leaks none (prefixItems/unevaluated*/2020-12)

👥️two-package -> TrustedBundleV1
  PASS bundle

🌐️browser-actor -> TrustedBundleBrowserActorV1
  PASS closed member
  PASS 26 cases + 8 rawLengths match declared expectations (mismatches=0)

🛡️opened-root -> TrustedCatalogCurrentPointerV1 / TrustedCatalogRelativePathV1
  PASS current pointer
  PASS 15 relative paths match (mismatches=0)

📤️publication -> TrustedCatalogPublicationCommandV1 / ReceiptV1
  PASS command member
  PASS 7 command expectations (mismatches=0)
  PASS 6 receipt expectations (mismatches=0)
  PASS cas pointer projection

🧬️stdio-gis-bootstrap -> Identity/BrowserActor/ExecutionProtocol/ParentDialect/Grant
  PASS selectedClosure[0]
  PASS selectedClosure[1]
  PASS gis.browserActor
  PASS gis.executionProtocol
  PASS gis.dependencies[0]
  PASS stdio.browserActor
  PASS stdio.executionProtocol
  PASS openTarget.parentDialect
  PASS openTarget.grant

🔗️compiled-dependencies -> TrustedBundleIdentityV1
  PASS identity[0]
  PASS identity[1]
  PASS identity[2]
  PASS identity[3]
  PASS identity[4]
  PASS identity[5]

🧱️generation-stage -> TrustedBundleExecutionProtocolV1
  PASS executionProtocol.root
  PASS executionProtocol.gis
  PASS executionProtocol.stdio

🪪️identity-roles -> TrustedBundleV1 (via ../👥️two-package)
  PASS source bundle

total contract checks: 86; exit=0
```

### 6.2 Hub oracle command (real router command, no shim)

```
$ cd /Users/ueli/Documents/semio && SEMIO_TEST_ARTIFACT_DIR=<ticket>/🗑️generated/wp4-trusted-catalog \
    bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts trusted-catalog-opened-root-check
trusted-catalog-opened-root: scope-exports=2 paths=15 denials=7 same-handle=1 source=14 startup=no-ambient-path; source contract only, native platform laws require --native
```

The other three router commands that reach my functions
(`gis-inference-ledger-oracle`, `document-browser-actor-identity-check`,
`native-catalog-selection-check`, `native-document-open-check`, `gis-map-proposal-check`) currently
**abort before reaching my code** on other partitions' in-flight WP4 edits (§8). To verify the real
production functions anyway, I ran them directly out of the real `📜️script.ts` through a scratchpad
Bun `onLoad` shim whose only effect is `export { … }` of the prove functions and stripping the
top-level `runBundleScriptMain` call — no code substitution:

```
$ cd /Users/ueli/Documents/semio && SEMIO_TEST_ARTIFACT_DIR=<ticket>/🗑️generated/wp4-trusted-catalog \
    bun --preload <scratchpad>/harness-shim.ts <scratchpad>/harness.ts <7 functions>
trusted-catalog-opened-root: scope-exports=2 paths=15 denials=7 same-handle=1 source=14 startup=no-ambient-path; source contract only, native platform laws require --native
OK   proveTrustedCatalogOpenedRootFixture
trusted-browser-actor-catalog: scope-export=TrustedBundleBrowserActorV1 cases=26 bodies=8 raw-lengths=8 WebCrypto=1; metadata/framing oracle only, no native loader/activation claim
OK   proveTrustedBrowserActorCatalogFixture
trusted-catalog-identity-oracle: exact=6 canonical-kind=1 descriptor-sha256=1 package-ref-blake3=distinct; no GIS provider activation
OK   proveTrustedCatalogIdentityRolesFixture
[DEBUG] trusted-retained-gis-browser: pinned-tables=3 deep-equal=6 SHA256=node+webcrypto source-hostiles=8; neutral bytes only, no browser execution claim
OK   proveTrustedGisRetainedBrowserFixture
[DEBUG] trusted-gis-collaboration-contract: pinned-specification=1 hostile=8 cross-fixture-equality=8; specification only, no observed collaboration trace
OK   proveTrustedGisMapCollaborationContractFixture
trusted-generation-stage: scope-export=TrustedBundleExecutionProtocolV1 WebCrypto=1 cases=28 evidence=…/wp4-trusted-catalog/generation-stage-9tBdxR
OK   proveTrustedGenerationStageFixture
[DEBUG] trusted publication final-byte fence: scope-exports=3 WebCrypto=1 Node-replace-oracle=1 cases=5 command-cases=7 receipt-cases=6 revision-cases=8 transport-outcomes=3 real-child-cases=12; native-owner/CAS remain unqualified; evidence=…/wp4-trusted-catalog/publication-fence-wl4Ws7
OK   proveTrustedPublicationFixture
harness: ran=7 failed=0
```

`proveTrustedPublicationFixture` above really spawns all 12 publication child processes and runs the
CAS/owner/final-byte-fence oracles; `proveTrustedGenerationStageFixture` really stages 28 generation
directories. Both pass.

`proveTrustedCompiledDependenciesFixture` and `proveTrustedBootstrapCodecCaptureFixture` execute all
of **my** new assertions and then abort inside other partitions' code (§8). Their pinned tables were
therefore replayed standalone:

```
$ cd /Users/ueli/Documents/semio && bun <scratchpad>/tables.mjs
compiled-dependencies pinned tables: PASS (11 keys, 25 cases, 7 atomic, 5 consumer, 2 preview, 4 raw, 2 encoding, ordering=3)
codec-source pinned tables: PASS (4 keys, 15 cases, exhaustive change enumeration)
stdio-gis-bootstrap pinned tables: PASS (9 keys, limits, NUL framing, 8 cancellation stages, 19 hostile, 6 source + 7 write rotation cases)
```

### 6.3 No `*.schema.json` left in the partition

```
$ cd /Users/ueli/Documents/semio && find 🌎️hub/🗿️artifact-authority/🔏️trusted-catalog -name '*.schema.json'
(no output)

$ cd /Users/ueli/Documents/semio && git ls-files 🌎️hub/🗿️artifact-authority/🔏️trusted-catalog | grep -E 'schema\.json$'
(no output — index-hits=0)

$ cd /Users/ueli/Documents/semio && git ls-files 🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🔣️.json
🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧬️schema/🦀️.rs
```

### 6.4 Not verified

- **Rust compilation.** My instructions forbid running cargo at all, so `🧬️schema/🦀️.rs`, the
  `#[path]` wiring, the `use schema::{…}` import list and the three call-site renames are
  **unverified by a compiler**. They follow the exact `🌎️hub/💡️inference/🦀️.rs` +
  `🌎️hub/💡️inference/🧬️schema/🦀️.rs` pattern and the sibling `🌐️browser-actor/🦀️.rs` `use super::{…}`
  pattern, and `use serde::{Deserialize, Serialize}` was narrowed to `Deserialize` because the only
  `Serialize` derive moved out — but a `cargo check -p semio-hub` is still owed by whoever is allowed
  to run one.
- `native-catalog-selection-check`, `native-document-open-check`, `gis-map-proposal-check` and
  `document-browser-actor-identity-check --native` additionally require a compiled binary
  (`runExactCargoLaws` / `runCargo`) and were not attempted.

## 7. Cross-partition requests

These schema paths are read by prove functions I rewired but are owned by **other** partitions. I
left every one of them exactly as it was:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌐️browser-actor/🔣️.schema.json`
  (scope `os.directory`) — previously `addSchema`'d alongside the trusted-catalog bundle schema in
  `proveTrustedBrowserActorCatalogFixture` and `proveTrustedStdioGisBootstrapFixture`. Both
  `addSchema` calls are gone from **my** functions; the file is still read at `📜️script.ts:2369`,
  `:5620`, `:8193` (should read `:8193`'s remaining hit — see below), `:14170` by os-partition
  functions. **Request to WP4-os:** when this becomes `os.directory`'s `🧬️schema/🔣️.json` module,
  export the public actor identity as `DocumentOpenBrowserActorV1` and tell me — the trusted-catalog
  scope currently re-states the identity fields inside `TrustedBundleBrowserActorV1` because
  (a) `hubSchemaExport` builds a per-scope Ajv with only that scope's document added, so a cross-scope
  `$ref` cannot resolve today, and (b) the os document is still 2020-12 and uses
  `unevaluatedProperties`, which draft-07 cannot `$ref` into. This duplication is legitimate today —
  the trusted-catalog `BundleBrowserActor` Rust enum declares every field itself with
  `deny_unknown_fields`, and cross-checks against os.directory at runtime via
  `identity().validate(source, renderer)` — but it should become a cross-scope `$ref` once both sides
  are draft-07 modules and `hubSchemaExport` can resolve dependencies.
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/🧬️native-codec-factories.schema.json` and
  `✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🧬️.schema.json` — read by
  `proveTrustedBootstrapCodecCaptureFixture` (a trusted-catalog prove function reading **plugin**
  schemas). Left untouched. **Request to WP4-plugins:** the stdio one is already deleted on disk
  while `📜️script.ts:8821` still reads it — see §8.
- `🧰️framework/🔨️modules/🛂️manifest/🧪️fixtures/🗄️artifact-kind-formats.schema.json` — read by
  `proveTrustedCompiledDependenciesFixture`. Left untouched; already deleted on disk — see §8.
- `🧰️framework/🔨️modules/🌱️value/🔁️codec/🧪️fixtures/🧬️.schema.json` — read by
  `proveDocumentBrowserActorIdentityFixture` (framework partition), which is the only caller of my
  `proveTrustedBrowserActorCatalogFixture`. Left untouched; already deleted on disk — see §8.
- `🌎️hub/💡️inference/🧬️schema/🔣️.json` is now draft-07 but `📜️script.ts:7676` still compiles it with
  `Ajv2020` — see §8.
- `🌎️hub/📦️packages/🟦️typescript/🤝️index.test.ts` is not listed in any worker's partition, but it read
  `🔣️bundle.schema.json` directly, so I rewired that one `it(...)` block (only) to the module. If that
  file belongs to another worker, this is the change I made.

## 8. Peer in-flight breakage observed (NOT mine, NOT fixed by me)

Snapshot at the time of writing; peers are moving, one of these already self-healed:

| Location | Symptom | Owner |
|---|---|---|
| `🧰️framework/🔨️modules/🎭️actor/📤️return/🟦️.ts:2` | imported `../📃️pageSchema/🟦️.ts`, directory is `📃️page` → every `bun 🌎️hub/…/📜️script.ts <cmd>` failed to load | framework — **self-healed during this session** |
| `📜️script.ts:8173` reads `🧰️framework/🔨️modules/🌱️value/🔁️codec/🧪️fixtures/🧬️.schema.json` | file deleted on disk, reader not rewired → `document-browser-actor-identity-check` ENOENT before reaching `proveTrustedBrowserActorCatalogFixture` | framework |
| `📜️script.ts:8616`-ish reads `🧰️framework/🔨️modules/🛂️manifest/🧪️fixtures/🗄️artifact-kind-formats.schema.json` | file deleted on disk, reader not rewired → `proveTrustedCompiledDependenciesFixture` ENOENT after my assertions | framework |
| `📜️script.ts:8821` reads `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️schema/🧬️native-codec-factories.schema.json` | file deleted on disk, reader not rewired → `proveTrustedBootstrapCodecCaptureFixture` ENOENT after my assertions | plugins |
| `📜️script.ts:7676` compiles `🌎️hub/💡️inference/🧬️schema/🔣️.json` with `Ajv2020` | module is now draft-07 → `no schema with key or ref "http://json-schema.org/draft-07/schema#"`, `gis-inference-ledger-oracle` aborts before `proveTrustedCatalogIdentityRolesFixture` | hub.inference |

## 9. Open questions

1. **Bundle Rust struct names.** Execution contract §A wants the Rust `struct` name to equal the
   export id. `Bundle`, `BundleProfile`, `BundlePackage`, `BundleFile`, `BundleComponent`,
   `BundleIdentity`, `BundleCodec`, `BundleOpenTarget`, `BundleGrant`, `BundleProfileOpenTarget` and
   `BundleBrowserActor` are still spelled without the `TrustedBundle…V1` prefix and still live in
   `🦀️.rs` / `🌐️browser-actor/🦀️.rs` rather than `🧬️schema/🦀️.rs`. I did not rename them: it is ~11
   types across a 2868-line file plus two submodules and `🏗️test-support/🦀️.rs`, and I am forbidden to
   run cargo, so the change would be unverifiable. The JSON Schema is nonetheless the single authority
   now. Needs a follow-up by someone who can compile.
2. **`relativePath` vs `TrustedCatalogRelativePathV1`.** The bundle's `path` fields keep the former
   bundle-schema pattern (1024 bytes, rejects `\`, leading `/`, `X:/`, `..` segments) rather than the
   stricter `TrustedCatalogRelativePathV1` that mirrors `TrustedCatalogRelativePath::parse`
   (additionally rejects `.` segments, empty segments, NUL and >64 segments). Behaviour on every
   existing fixture is identical; unifying them would tighten the bundle contract to match what Rust
   actually enforces downstream, but it is a semantic change I did not make unilaterally.
3. **`importInterfaces` vocabulary.** The old bundle `browserActor` inherited the closed 16-value
   interface `enum` from os.directory's `#/$defs/interfaces`. `TrustedBundleBrowserActorV1` uses a
   bounded `uniqueItems` string array instead, matching this scope's `Vec<String>` Rust field, which
   delegates the vocabulary to `DocumentOpenBrowserActorV1::validate`. All 26 corpus cases behave
   identically; the enum should come back as a cross-scope `$ref` per §7.
4. **`stage` vocabulary.** `HubFixtureExpectationV1` allows `contract | domain | bounds`. I used
   `contract` and `domain`; nothing in this scope needed `bounds`.
