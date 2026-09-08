# WP0 — The Existing Owner/Schema Mechanism (Mechanism Surface Audit)

Read-only audit. Every claim below is cited to an exact path and, where the source is code, an exact
line number as of this session's HEAD. Emoji-prefixed paths are copy-pasteable; cwd was
`/Users/ueli/Documents/semio` for every command.

## 0. TL;DR — what exists today, in one paragraph

There is **no single "scope owner" registry**. There are at least **four independent, overlapping
ownership mechanisms**, each with its own eligibility rule, each hand-rolled in TypeScript, none
derived from a declared taxonomy list of "this scope owns schema X":

1. **Artifact schema parity** (`policyArtifactSchemaBreaches`, 📜️script.ts) — eligibility is a
   **hand-maintained literal table** (`POLICY_ARTIFACT_SCHEMA_PREFIXES`) mapping `plugin/artifact` to a
   PascalCase type-name prefix. One schema per facet (🧬️schema, 📸️snapshot/🧬️schema, 🔺️diff/🧬️schema),
   mirrored across up to 5 language files, JSON Schema normative.
2. **App/surface schema parity** (`policyAppSchemaBreaches`, `policyDiscoverAppSchemaOwners`) —
   eligibility is **derived from source**: it parses `type Config = XConfig;` out of each surface's Rust
   component and walks to find where `XConfig` is actually declared. No hand-maintained table.
3. **Mutation payload schema** (`mutationPayloadSchemaAuthority`/`mutationPayloadSchemaLocation` in
   taxonomy.json + `derivePayloadSchema`/`scaffoldLeafDescriptor` in the test harness) — a **repo-wide
   heuristic Rust-type index** (`rustTypeIndex`) that scans every `.rs` file, indexes `pub struct`/`pub
   enum` by bare name, and resolves same-named collisions by "longest common path-prefix with the
   owner" — a full nearest-parent heuristic.
4. **stdio format artifacts** (`stdioDefinitionCatalog`, 📜️script.ts ~19519) — an **explicit catalog**
   (`📇️registry/🔣️.json` → `artifact_definition_paths[]`) cross-checked against a full-tree walk so the
   catalog can never silently drift from disk, with canonical dotted, versioned identities
   (`s.stdio.<slug>.v<N>`). This is the closest existing precedent to a "(scope, export id)" identity
   scheme, but it is stdio-only and describes container/codec/mutation *definitions*, not schema field
   contracts.

The taxonomy vocabulary (`schemaFormats`, `schemaFacetKinds`, `schemaChildDirs`,
`artifactSchemaSpecFileKinds`, `surfaceSchemaSpecFileKinds`) defines **where a `🧬️schema/` facet's
files must physically sit and what filename each language gets**, and nothing else. It has **no concept
of "named export"** — every schema leaf file (`🔣️.json`, `🦀️.rs`, `🟦️.ts`, `🔗️.graphql`,
`🛰️.proto`) is expected to declare **exactly one** top-level type, found by
`policyFindSchemaDeclaration`/`policyExpectedSchemaTypeNameForFacetPath` (script.ts:30013-30022) using
an *expected type name*, not a catalog of exports. "Named export resolved by (scope, export id,
format)" as described in the ticket goal **does not exist as a general mechanism today** — but its
skeleton is closer than it first appears: `ArtifactSchemaDescriptor`/`FacetLeaves`
(`🧰️framework/🔨️modules/🧬️schema/⚛️component.rs:130-172`, §3.7.1) already carry a dotted **scope id**
(verified example `"s.trinity.jack"`) and a fixed-cardinality `(facet name, format) → body` map — it
just caps the facet dimension at exactly 4 hardcoded names (`artifact`/`snapshot`/`diff`/`mutations`)
rather than allowing arbitrary named exports. Widening that struct is the shortest path to the ticket's
target model; see §3.7.1 for the verified detail.

Fixtures, by contrast, **already** do exactly what the goal describes for schemas: `resolveFixtures`
(test harness 🟦️.ts:996-1015) binds `shared://`/`local://`/`asset://` URIs **explicitly** against a
case's fixture directories — no nearest-parent search, no glob. This is the pattern to generalize to
schema exports, not something that needs building from scratch.

---

## 1. Taxonomy vocabulary

File: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (1,064,258 bytes, 1
top-level JSON object). Full list of top-level keys is in the companion JSON deliverable
(`taxonomySchemaKeys.topLevelKeys`); only schema-relevant families are narrated here.

### 1.1 The five-format schema vocabulary

```json
"schemaFormats": {
  "🦀️rust":       { "fileKindId": "rust-source",       "fieldCasing": "snake" },
  "🟦️typescript": { "fileKindId": "typescript-source", "fieldCasing": "camel" },
  "🔗️graphql":    { "fileKindId": "graphql",            "fieldCasing": "camel" },
  "🔣️jsonschema": { "fileKindId": "json",               "fieldCasing": "camel" },
  "🛰️protobuf":   { "fileKindId": "protobuf",           "fieldCasing": "snake" },
  "📜️wit":        { "fileKindId": "wit",                "fieldCasing": "kebab" }
}
```
`schemaFormats` is the global list of "a schema can be expressed in these languages, each with its
canonical filename (via `fileKindId` → `canonicalFilenameForKind`) and its expected field-name casing."
It says nothing about ownership or which facet needs which subset of formats — that's
`schemaFacetKinds`:

```json
"schemaFacetKinds": {
  "🧬️data":      { "normativeFormat": "🔣️jsonschema", "formats": ["🔣️jsonschema","🦀️rust","🟦️typescript","🔗️graphql","🛰️protobuf"] },
  "📜️interface": { "normativeFormat": "📜️wit",         "formats": ["📜️wit"] }
}
```
A `🧬️schema/` directory's **kind** (which of these two rows applies) is resolved *from disk*, not
declared: `resolveSchemaFacetKind` (🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:2670-2679)
checks, for each `schemaFacetKinds` entry, whether the facet directory already contains that kind's
*normative* format's canonical filename (e.g. does `🔣️.schema.json` exist?) — first match wins, and if
none matches it falls back to `"🧬️data"`. `schemaFacetFormatEntries` (same file, 2684-2691) is the
function everything downstream calls to get "the `[formatId, formatSpec]` pairs a facet is expected to
carry."

**Extension point**: `resolveSchemaFacetKind`/`schemaFacetFormatEntries` is the one already-generic
place that turns "a facet path" into "the formats/filenames it should carry." A named-export catalog
would plug in here or immediately downstream of it.

### 1.2 Where a `🧬️schema/` facet may physically sit

- `artifactComponentDirs[0]`, `artifactChildDirs[0]`, `subsetComponentDirs[0]`, `subsetChildDirs[0]` =
  `"🧬️schema"` — every artifact and every subset gets a `🧬️schema` child.
- `configChildDirs[0]`, `presenceChildDirs[0]`, `transientChildDirs[0]` = `"🧬️schema"` — every surface's
  `🎚️config`/`👥️presence`/`🫧️transient` facet also gets one.
- `mutationOrganizationalFacetDirs[3]` = `"🧬️schema"` — optional per-mutation-leaf schema facet.
- `schemaChildDirs` = `["📸️snapshot", "🔺️diff", "🧬️mutations", "💡️inferences"]` — what a `🧬️schema/`
  directory may itself contain as children (the four "representation" facets).
- `taxonomyLeafParentDirs[0]` = `"🧬️schema"`.
- `semanticDirectoryKinds.schema` — `{ emoji: "🧬️", slugPattern: "^(schema|mutations|contract|wire)$" }`;
  several other kinds (`schema-artifact-subject`, `retained`, `ui-contract-copy`, `ui-contract-typed`,
  `ui-contract-document`) declare `parentKindIds: ["schema"]`, i.e. they're things that may live *inside*
  a schema facet.

### 1.3 Normative-leaf declarations (which format is "the" source of truth, per facet shape)

```json
"artifactSchemaSpecFileKinds": {
  "🧬️schema": "json", "🧬️schema/📸️snapshot": "json", "🧬️schema/🔺️diff": "json",
  "🧬️schema/💡️inferences": "json",
  "🚪️io/📸️snapshot/📝️text": "json", "🚪️io/🔺️diff/📝️text": "json",
  "🚪️io/💡️inferences/📝️text": "json", "🚪️io/🧬️mutations/📝️text": "json"
}
```
```json
"surfaceSchemaSpecFileKinds": {
  "🎚️config/🧬️schema": "json", "👥️presence/🧬️schema": "json", "🫧️transient/🧬️schema": "json"
}
```
Both map a facet-relative path to a `fileKindId` (always `json` today) — i.e. "this facet's normative,
source-of-truth leaf is JSON Schema." Consumed by `policyArtifactSchemaFacetCompletenessBreaches`
(script.ts:30381) and `policyAppSchemaFacetCompletenessBreaches` (script.ts:30834).

### 1.4 Mutation payload schema — the one place taxonomy.json declares a *contract-linking* rule

```json
"mutationPayloadSchemaLocation": { "directoryKindId": "schema", "directoryName": "🧬️schema", "fileKindId": "json" }
"mutationPayloadSchemaAuthority": {
  "contractKind": "descriptor-linked-mutation-payload-schema",
  "ownerAuthority": "mutationOwnerIdentity",
  "descriptorFileKindId": "json",
  "descriptorField": "payloadSchema",
  "descriptorSchemaVersion": 1,
  "descriptorCardinality": "one-canonical-no-competing-descriptor",
  "descriptorOwnerField": "owner",
  "descriptorIdentityField": "semanticKind",
  "jsonSchemaDialect": "http://json-schema.org/draft-07/schema#",
  "targetAuthority": "owner-relative-regular-json-schema"
}
```
This is the **only** taxonomy-level statement that looks like "an export is referenced by a named field
on a descriptor" — a mutation leaf's own `🔣️.json` descriptor (elsewhere `testContributionFileKindId`)
carries a `payloadSchema` field that is supposed to point at the mutation's owner-relative JSON Schema
file. In practice (see §2.4) the actual test-harness code that *resolves* `payloadSchema` accepts **two
different conventions** for where that file lives — this is exactly the kind of ambiguity WP1+ needs to
collapse into one.

### 1.5 Test-module-specific schema location

```json
"testSchemaLocation": { "directoryPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema", "fileKindId": "json" }
```
Points at the test-protocol descriptor schema itself (§3.1) — a schema *about* test descriptors, not a
domain schema.

### 1.6 fileKinds / fileKindResolutionRules (schema-role file kinds)

`fileKinds` entries with `"role": "schema"`: `graphql` (`.graphql`), `json` (`.json`), `json-expect`
(`.expect.json`), `json-fixture` (`.fixture.json`), `json-patch-fixture` (`.patch.json`), `json-schema`
(`.schema.json`), `json-snapshot` (`.snapshot.json`), `protobuf` (`.proto`), `wit` (`.wit`).
`fileKindResolutionRules.physical-json-schema-json` = `{ extensionChain: ".schema.json", fileKindId:
"json-schema", priority: 0 }` — this is the **only** kind resolution rule scoped to schema, and it is
what lets a bare `🔣️.schema.json` (as opposed to a canonical schema-facet leaf) exist at all as a
recognized kind elsewhere in the tree (used by the harness's flat-`.schema.json` convention, §2.4).

### 1.7 What "scopedFileKinds", "fixedDirectoryContracts", "fixedFilenameContracts",
"fixedFilenameRejectionContracts", "configurableEntryContracts", "packageBoundaryRules" actually
contain w.r.t. schema

- **`scopedFileKinds`** (8 entries) — none are schema-related; all are internal taxonomy-*transaction*
  bookkeeping (backup/edit/restore staging files for the taxonomy tool's own crash-safety). Not part of
  the owner/schema mechanism at all.
- **`fixedDirectoryContracts`** — **zero** entries mention "schema" anywhere in their JSON (checked by
  full-text scan of the ~55KB value). Directory-shape enforcement for schema facets happens entirely
  through `schemaChildDirs`/`artifactChildDirs`/`subsetChildDirs` + the policy functions, not through
  `fixedDirectoryContracts`.
- **`fixedFilenameContracts`** — 2 entries total, neither schema-specific (`ui-components-config` for
  shadcn's `components.json`; `cargo-cache-tag` for Cargo's `CACHEDIR.TAG`). One's `reason` string
  happens to use the word "schema" colloquially ("uses shadcn's schema").
- **`fixedFilenameRejectionContracts`** — 1 entry (`nested-ticket-manifests`), unrelated to schema; a
  ticket-relocation rule.
- **`configurableEntryContracts`** — package entry-point conventions (Cargo `lib.path`/`bin[].path`,
  package.json `exports`/`main`/`types`) — describes how *code* entry points are declared, not schema.
  Relevant only as a sibling pattern: entry points are resolved via `configurationSources` (an explicit
  string naming the config key), which is a possible template for "how a scope declares its schema
  export catalog."
- **`packageBoundaryRules`** — per-ecosystem (rust/typescript/javascript/go/python/dotnet) allow-lists
  of file/directory kinds a package boundary may contain (`allowedFileKindIds`,
  `allowedDirectoryKindIds`). None reference schema directly; they gate what can live *inside*
  `📦️packages/<lang>/`, which is a sibling of `🧬️schema/`, not a container for it.

### 1.8 Catalog/projection mechanisms already in taxonomy.json (prior art for "generate a catalog from owned files")

- **`mutationCatalogProjection`** — `{ projectionContractId: "artifact-mutation-tests-v1",
  projectedMemberKindId: "mutation-test-subject", descendantContractId: "mutation-scenario-bundle-v1",
  catalogContractId: "mutation-catalog-vectors-v1" }`. Ties together three other contract families
  (`semanticPathProjectionContracts["artifact-mutation-tests-v1"]`,
  `semanticDescendantContracts["mutation-scenario-bundle-v1"]`,
  `semanticPathProjectionCatalogContracts["mutation-catalog-vectors-v1"]`) into one named pipeline: walk
  a fixed path shape under every subset, find scenario-bundle descendants, and assemble them into one
  catalog with declared uniqueness fields (`mutationId`, `mutationDirectoryName`, `scenarioId`) and a
  `coverage: "every-physical-bundle-exactly-once"` completeness contract. **This is the strongest
  existing precedent for "declare a catalog contract in taxonomy.json, generate/verify it from a
  script.ts command."**
- **`semanticPathProjectionCatalogContracts`** also holds `cad-model-catalog-v1` (a hand-authored,
  domain-specific manifest catalog for the CAD plugin: `categoryRules[]` map source directory names to
  `manifestSchema` string ids like `"spatial.action"`, `"spatial.typology"` — **these
  `manifestSchema` strings are dotted identifiers with no `$defs`/file backing found in this audit**;
  they look like intended (scope, export-id) identity strings but nothing currently resolves them to an
  actual schema file) and `draw-editor-command-vectors-v1` (an `exact-owner-vectors` contract: a
  literal, hand-maintained `vectors: [{artifactId, standardVersion, subsetId, commandDirectoryName}]`
  array — the taxonomy-JSON equivalent of `POLICY_ARTIFACT_SCHEMA_PREFIXES`, i.e. another
  hand-maintained enumeration standing in for what would ideally be scope-declared/discovered data).
- **`semanticOwnedFileProjectionContracts`** (5 entries: `artifact-empty-facet-primary-markdown-v1`,
  `readme-license-owner-leaves-v1`, `ticket-document-primary-markdown-v1`,
  `ticket-important-history-markdown-v1`, `ticket-important-markdown-v1`) and
  **`semanticPathProjectionReferenceConsumerContracts`** (9 entries, all CAD/draw-specific — e.g.
  `cad-editor-interaction`, `draw-package-cargo`) exist but none of the 14 entries across both are
  schema-export related; they're markdown-ownership and reference-consumer-path contracts for specific
  plugins.
- **`mutationDomainOwners`** (2 entries: the `architect/program` artifact and the stdio `gltf` artifact)
  and **`mutationCatalogSourceOwners`** (8 entries, all gltf sub-subsets pointing at the gltf `♾️any`
  subset as their catalog source) are hand-authored per-owner **verb vocabularies** (which
  create/delete/rename/... mutation ids exist for which noun) — a different axis from schema field
  contracts, but the same "hand-maintained JSON table keyed by owner path" shape recurs here for a third
  time.

---

## 2. Tooling

### 2.1 Root `📜️script.ts` — command surface (35,456 lines; `.` prefixed subcommands below are
`bun ./📜️script.ts <command> <subcommand>`)

Dispatch table (script.ts:22716-22750, `const router = new ScriptRouter(...)`): top-level commands are
`os`, `semio`, `examples`, `nx`, `setup`, `start`, `dev`, `generate`, `scale-fixture`, `new`, `lint`,
`verify`, `format`, `test`, `bench`, `stdio`, `build`, `cpp`, `publish`, `purge`, `clean`,
`micro-commit`, `commit`. CLI entry: `if (import.meta.main) { if
(!(await dispatchPolicyArgv(...))) { await runWorkspaceScriptMain(router); } }` (script.ts, end of
file).

**`GenerateScript`** (script.ts:1054-1153):
- `generate taxonomy census|duplicates --ticket <id>` (`generateTaxonomy`, 1101-1120) — writes
  `📊️semantic-census.json`/`📓️semantic-census.md` (or `…-duplicates…`) via `buildSemanticCensus` — a
  general taxonomy-inventory report, not schema-specific, but it's the pattern any new "schema census"
  command would follow (writes into the ticket dir, from `--ticket`).
  - `generate taxonomy package-adapter [preview|apply]` → `runNestedCargoPackageAdapter` — unrelated to
    domain schema (jco/Cargo adapter).
- `generate neo4j [<db-name>]` / bare `generate` — Neo4j graph export, unrelated to schema.
- `generate plugin-glue [--dry] [<plugin>]` (`generatePluginGlue`, 1123-1153) — emits
  `🤖️generated-subset-inventory.txt` per plugin, listing subset paths; explicitly a discoverability
  probe, not a schema catalog ("this command validates discoverability").
- **There is no `generate schema` command.** Schema *derivation* (payload schema from Rust structs)
  lives only in the test module (§2.2), and *checking* schema parity lives only in `lint`/`verify gate`
  (below), never as a `generate`.

**`VerifyScript`** (script.ts:10685-…): `verify taxonomy report|enforce [--scope]` (calls the shared
`verifyTaxonomy`), `verify mutation-outcome-law`, `verify rust-warnings [--target] [-p]`, `verify
interactivity [tool-jobs|apps|p1q-b1-b6|p1w|p1x|p1y|p1z|p5d|p5e|p3mn]`, `verify dependencies`, `verify
layering`, bare `verify` / `verify gate` (runs `runGate()`, which is the umbrella that ends up calling
the `policy` linter — see below). **There is no `verify schema` subcommand.** Schema-contract
verification happens only as part of the giant `policy` lint export (next item) or via `verify gate`.

**The `policy` lint export** (script.ts, end of file, `export const policy = defineLint(...)`, ~35380
to EOF) is the actual enforcement surface: it's a single `TechnologyLinter` that calls ~70
`policy*Breaches` functions in sequence and concatenates their `BreachRecord[]`. The two schema-relevant
calls are:
```
breaches.push(...policyArtifactSchemaBreaches(repoRoot));   // script.ts ~end, in the policy export
breaches.push(...policyAppSchemaBreaches(repoRoot));
```
Run via `bun ./📜️script.ts lint` (`LintScript`, script.ts:1159) or as part of `verify gate`. There is
**no way to run only the schema checks** from the CLI today — they're two calls buried inside one
70-call linter.

#### 2.1.1 `policyArtifactSchemaBreaches` (script.ts:30676-30684) — the artifact-schema mechanism

Composed of 5 rule functions, all iterating `policyListPluginArtifactDirs(repoRoot)`
(script.ts:27913-27924 — walks `✏️s/🔌️plugins/<plugin>/🗿️artifacts/<artifact>`, nothing more):

| Function | Lines | What it checks |
|---|---|---|
| `policyArtifactSchemaFacetCompletenessBreaches` | 30381-30422 | every artifact has all 3 `POLICY_SCHEMA_FACET_RELS` dirs, each with every `schemaFormats` leaf + the normative leaf from `artifactSchemaSpecFileKinds` |
| `policyArtifactSchemaFieldParityBreaches` | 30442-30507 (start) | all 5 leaves of one facet declare the identical field set/optionality/cardinality; JSON Schema wins disagreements |
| `policyArtifactSchemaStateParityBreaches` | 30509-30567 | state-class tags (`#[state(...)]` / `@state` JSDoc / GraphQL `@state` directive) agree across languages |
| `policyArtifactSchemaDiffCoverageBreaches` | 30568-30622 | 🔺️diff facet fields are a subset of the base 🧬️schema facet's fields |
| `policyArtifactSchemaTypeNameParityBreaches` | 30623-30675 | the declared type name in each language equals `POLICY_ARTIFACT_SCHEMA_PREFIXES[plugin/artifact]` + `Artifact`/`Snapshot`/`Diff` |

**Owner-eligibility mechanism**: `POLICY_ARTIFACT_SCHEMA_PREFIXES` (script.ts:29889-29948) is a
**literal, hand-maintained `Record<string,string>`** with ~65 entries keyed
`"<plugin>/<artifact>"` (emoji-stripped) → PascalCase prefix (e.g. `"cad/cad": "Cad"`, `"fem/2d":
"Fem2d"`). `policyArtifactSchemaPrefix` (script.ts:29974-29981) looks an artifact path up in this table;
if absent, the artifact is **silently excluded** from type-name-parity checking (though facet
completeness/field parity still run, since those don't need the prefix). The breach message for a
missing entry literally says: *"Add a prefix entry for this artifact to POLICY_ARTIFACT_SCHEMA_PREFIXES
in 📜️script.ts (see normative-spec §10)."* (script.ts:30635) — i.e. **the documented extension
procedure today is "hand-edit a TypeScript source file,"** which is precisely the kind of
non-scope-declared eligibility the ticket wants replaced.

Per-format field extraction (each parses ONE top-level declaration per file, chosen by expected-name
match, first-declaration fallback via `policyFindSchemaDeclaration`, script.ts:29996-30011):
- `policyExtractRustSchemaFields` (30052-30100) — `pub struct` with `#[state(...)]` attrs.
- `policyExtractTypescriptSchemaFields` (30124-30157) — `export interface` with `@state` JSDoc.
- `policyExtractGraphqlSchemaFields` (30158-…) — `type X { }` with `@state` directive.
- `policyExtractJsonSchemaFields` (30256-30299) — `$defs`/`properties` of the JSON Schema file.
- `policyExtractProtobufSchemaFields` (30300-30331) — protobuf `message`.
- `policyLoadSchemaFacetLeaves` (30332-30370) — orchestrates all 5, using `schemaFacetFormatEntries`
  (from the discovery library, §1.1) to know which formats to expect and `canonicalFilenameForKind` to
  know each format's exact filename.

**Structural finding for the ticket**: today's model is "**one type per schema file**," discovered by
*expected name* (derived from the hardcoded prefix table) with a same-file positional fallback. There is
no notion of multiple named exports living in one `🔣️.json`/`🦀️.rs` file, and no notion of a
scope-to-export catalog — the "catalog" is implicitly "whatever facet directories exist on disk," walked
fresh on every lint run.

#### 2.1.2 `policyAppSchemaBreaches` (script.ts:31145-…) — the app/surface-schema mechanism (the

**better precedent to generalize from**

```
/** Owners are derived from each surface's `type Config = …` binding — never a hand-maintained prefix table. */
```
(script.ts, comment directly above `policyDiscoverAppSchemaOwners`, ~30687-30693)

`policyDiscoverAppSchemaOwners` (script.ts:30728-30768): walks every plugin's `👁️viewer`/`✏️editor`
surface roots (`policySurfaceRoots`, script.ts:25163), reads each surface's Rust component file, regex
matches `type Config = ([A-Za-z_][A-Za-z0-9_]*);`, and then resolves the **owner directory** for that
config type by checking, in order: (1) sibling `🎚️config/` dir, (2) legacy `🧮️config/` dir, (3) the
*plugin-level* `🎚️config/🦀️.rs` if it textually declares `pub struct <ConfigType>`. Presence owner is
derived as the sibling `👥️presence/` of whichever config owner was found, and the presence type name is
mechanically `<ConfigType minus "Config">Presence` (`policyAppPresenceTypeName`, script.ts:30712-30716).
**No hand-maintained table anywhere** — eligibility and the expected type name both come from parsing
the surface's own source. `policyLoadAppSchemaFacetLeaves` (30780-30823) reuses the same 5
`policyExtract*SchemaFields` functions as the artifact mechanism, but with `expectedTypeName` supplied
directly from the discovered `configType`/`presenceType` rather than a table lookup.

This is the pattern to hold up as "the existing mechanism already knows how to derive an owner and an
expected export name from source, without a hardcoded table" — extend *this* shape to artifacts (or
retire `POLICY_ARTIFACT_SCHEMA_PREFIXES` in favor of an equivalent source-derived or explicitly-declared
mechanism), rather than inventing a third pattern.

#### 2.1.3 Mutation-taxonomy inventory system (script.ts:20859-21286, `mutationTaxonomy*`/`policyMutation*`)

A separate, large (~450-line) subsystem — `inventoryMutationTaxonomy`, `planMutationTaxonomy`,
`runMutationTaxonomyCli` (wired to `verify taxonomy`/a dedicated CLI path via
`taxonomyCliOptions`/`TaxonomyCliOperation`) — that tracks which *source files* (Rust/TS) declare which
mutation kind, detects drift between `🧬️mutations/<kind>` directory names and Rust module structure, and
enforces `mutationPayloadSchemaLocation`/`mutationPayloadSchemaAuthority` placement
(`policyMutationPayloadSchemaProblems`, script.ts:28259-28266). Distinct from the artifact/app schema
mechanisms above; scoped to the mutation-verb layer, not general schema field contracts. Worth knowing
about because any new scope-owner mechanism for mutation payload schemas specifically should reconcile
with this rather than add a fifth parallel system.

### 2.2 Test module — `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/`

Files (all read this session):
- `📜️script.ts` (1,571 lines) — CLI classes.
- `📦️packages/🟦️typescript/🟦️.ts` (6,039 lines) — the harness library (discovery, contract validation,
  fixture resolution, schema derivation).
- `🟨️.mjs` (200 lines) — Nx cache-input hashing only; not schema-relevant beyond hashing fixture bytes
  (lines 61-96).
- `🧬️schema/🔣️.json` (930 lines) — the **test-protocol descriptor schema** (JSON Schema draft
  2020-12), `$id: "https://semio-tech.com/schema/repo/test/v2"`, carries an `x-semio` annotation block
  (`owner`, `schemaVersion`, `baselineSha`) — this is a real, working example of the
  `x-semio-*`-annotation pattern the ticket goal alludes to, but it describes the **test harness's own
  wire format** (features/fixtures/oracles), not a domain artifact's data schema.
- `📇️registry/🔣️.json` (34 lines) — the framework's own (empty-by-default) oracle registry, with the
  explicit contract: *"Nothing in this file may name a plugin, a product or a format... Every reference
  implementation... belongs to that domain's owner and reaches the platform through
  `<owner>/🔣️oracle.json`, which is discovered by convention."* — i.e. **per-owner oracle registries are
  discovered by filename convention** (`🔣️oracle.json` beside the owner), not centrally cataloged. A
  real precedent for "convention over central catalog," worth weighing against the ticket's "consumers
  resolve named exports via (scope id, export id, format)" goal, which is closer to a central catalog.

Pipeline stages (from `taxonomy.json testPhases`): `discover → dsl → doctor → contract → oracle →
subject → parity → run → report → metrics → clean → dependency → nx → inventory`
(`testLevellessPhases` — the subset that ignores `--level`: `discover, doctor, report, metrics, nx,
inventory`). CLI classes implementing these, script.ts: `DiscoverScript` (825), `ContractScript` (838,
runs `validateAllContracts` → writes `⚡️cache/breaches/testing.json`), `OracleScript` (851),
`SubjectScript` (858), `ParityScript` (865), `RunScript` (872), `ReportScript` (885), `CleanScript`
(897), `DependencyScript` (920), `DslScript` (964), `MetricsScript` (971), `NxScript` (990),
`DoctorScript` (997), `InventoryScript` (1028), plus non-phase utility classes `FixtureScript` (1078,
`fixture generate|reproduce|verify|audit`), `ProbeScript` (1198), `MatrixScript` (1219), `GcScript`
(1245), `GapScript` (1352), `ManifestScript` (1439, `manifest scaffold|payload-schema`, and bare
`manifest` = readiness-per-owner report).

Note: `validateAllContracts` (harness 🟦️.ts:1869) performs **hand-written structural checks**, not
JSON-Schema-validator-driven ones — no `Ajv`/`ajv` import exists anywhere in the harness `.ts` file (the
draft-2020-12 test-protocol schema at `🧬️schema/🔣️.json` documents the wire shape but nothing in this
file runs it through a validator at execution time).

#### 2.2.1 Explicit fixture binding (the pattern to generalize) — harness 🟦️.ts:996-1032

`resolveFixtures(repoRoot, discovered, uris)`: each fixture reference is a URI with an explicit scheme —
`shared://`, `local://`, or `asset://` — extracted from feature text by `fixtureUrisIn`
(🟦️.ts:982-987, regex `FIXTURE_URI_RE`). Resolution is a straight join against
`discovered.sharedFixtureDir`/`localFixtureDir`/`owner`, with a path-escape guard
(`resolve(abs).startsWith(guard + sep)`) and a digest pin (`fileDigest`). **No shadowing, no nearest-dir
search** — the doc comment is explicit: *"a `local://` name never shadows a `shared://` one, so adding a
case-local file can never silently change what an existing scenario reads."* This is the existing,
working analog of "fixtures bind explicitly to exports" from the ticket goal — schema exports should get
an equivalently explicit reference scheme (e.g. `owner-relative export id` instead of a bare filename).

#### 2.2.2 Heuristic/fallback schema discovery to note (see §5 for the consolidated list)

Two conventions accepted interchangeably for a mutation leaf's payload schema
(`scaffoldLeafDescriptor`, harness 🟦️.ts:3579-3596):
```ts
const schemaCandidates: readonly [string, string][] = [
  [`🧬️schema/${testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId)}`, join(leafAbs, "🧬️schema", testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId))],
  ["🔣️.schema.json", join(leafAbs, "🔣️.schema.json")],
];
const payloadSchema = schemaCandidates.find(([, abs]) => existsSync(abs))?.[0] ?? "";
```
And the repo-wide `rustTypeIndex` heuristic scan used by `derivePayloadSchema` (harness 🟦️.ts:3985 ff.)
— see §5 for full detail; this is the single largest heuristic-discovery mechanism found in the audit.

### 2.3 `.mjs` / registry / other test files
Already covered inline above (§2.2).

---

## 3. Build/launch integration, runtime validation, packaging

Full raw detail lives in `📊️wp0-mechanism.json` (`buildIntegration`, `launchEntries`,
`runtimeValidators`, `productionFixtureReads`, `packagingEmbeds`); narrative summary here.

### 3.1 nx.json / root project.json / package.json

`nx.json` has **no** schema/fixture/taxonomy `namedInputs` — its only "schema" hit is its own
`"$schema": "./node_modules/nx/schemas/nx-schema.json"` pointer (nx's config schema, unrelated). The
root `📋️project.json` (1,011 lines) carries the `metadata.semio.taxonomy` pointer (lines 6-8, pointing
at `🔣️taxonomy.json`) and the actual nx targets that wrap `📜️script.ts`: `generate`,
`generate-taxonomy-census`/`-duplicates`, `clean-taxonomy-{inventory,plan,apply,verify}`,
`new-taxonomy-mutation`, **`stdio-schema-parity`** (lines 568-575, `dependsOn: ["stdio-quick"]` — the
field-parity checker runs as its own first-class nx target, not only buried inside `lint`),
`mutation-outcome-law`, `verify`, `verify-taxonomy-report`/`-enforce`,
`verify-dependencies-freeze[-write-baseline]`, `verify-layering[-write-baseline]`. `package.json` has no
schema/fixture/taxonomy-named script; every relevant script is a thin `bun nx run
workspace:<target>` wrapper around the `project.json` targets above (some dev scripts pass `fixture` as
a CLI *argument value*, not a script name, e.g. `dev:puzzle:3d:concrete-forest`).

### 3.2 `.vscode/launch.json` (26,771 lines) — naming/grouping convention

Every entry: `{"name", "type": "node-terminal", "request": "launch", "command": "bun nx run
workspace:<target>" (or "<project>:<target>"), "cwd": "${workspaceFolder}", "presentation": {"group",
"order"}}`. `name` is emoji-segmented `<verb-emoji>verb<noun-emoji>noun<detail-emoji>detail`, e.g.
`📦️generate🧩️taxonomy📊️census`. Groups: `0_dev, 1_keyboard, 2_mouse, 3_dev, 4_build, 4_gate,
5_publish`. `order` is a **float**, deliberately, so a new entry inserts between two existing ones
(`209`, `209.1`, `209.15`, `209.2`, `209.3`, ...) without renumbering siblings. Four verbatim examples
(lines 10185-10259):

```json
{ "name": "📦️generate🧩️taxonomy📊️census", "type": "node-terminal", "request": "launch",
  "command": "bun nx run workspace:generate-taxonomy-census -- --ticket 26/08/12/...",
  "cwd": "${workspaceFolder}", "presentation": { "group": "4_build", "order": 207 } }

{ "name": "📦️verify🧩️taxonomy📋️report", "type": "node-terminal", "request": "launch",
  "command": "bun nx run workspace:verify-taxonomy-report",
  "cwd": "${workspaceFolder}", "presentation": { "group": "4_build", "order": 209 } }

{ "name": "📦️verify🧩️taxonomy🚦️enforce", "type": "node-terminal", "request": "launch",
  "command": "bun nx run workspace:verify-taxonomy-enforce",
  "cwd": "${workspaceFolder}", "presentation": { "group": "4_build", "order": 209.1 } }

{ "name": "📦️new🧩️taxonomy🗿️artifact", "type": "node-terminal", "request": "launch",
  "command": "bun nx run workspace:new -- artifact",
  "cwd": "${workspaceFolder}", "presentation": { "group": "4_build", "order": 209.3 } }
```

Other schema/taxonomy/fixture-named entries confirmed: `⚖️gate📕️norm🧬️mutation-leaf-taxonomy` (3871),
`📦️build🦀️@semio-tech/framework-schema` / `🧪️test🦀️@semio-tech/framework-schema` (11494/11505),
`📦️build🦀️@semio-tech/schema-derive-rs` / `🔎️check🦀️@semio-tech/schema-derive-rs` (12220/12231),
`⚖️gate🧬️schema-representation` (8735), `⚖️gate🕸️dag🧬️payload-schema` (8746), ~50
`🧹clean🧩️taxonomy🧪️*` shrink-only fixture-migration gates (4744-8724), and several
`⚖️gate*🧪️direct-fixtures` entries (8768-9021). **A new schema/taxonomy dev command belongs in group
`4_build` (or `4_gate` for verification-only entries), placed right after the sibling
`verify-taxonomy-*`/`clean-taxonomy-*` block, at the next available fractional order.**

### 3.3 Root ratchet files

- **`🔒️dependencies.json`** — generated flat array of every external dependency
  (ecosystem/name/version/kinds/users/productionReachable). References the schema module's own
  `Cargo.toml` files as tracked "users" of external deps (lines 2019/2073/2369/2492), and tracks the
  `schemars` crate itself as a dependency (line 2203).
- **`🧅️layering.json`** — a shrink-only cross-area-reference ratchet (max-count per file pattern,
  regenerated only via `verify layering write-baseline`). The schema component file itself is a
  ratcheted violation source, capped at 1 reference each (lines 16, 27:
  `🧰️framework/🔨️modules/🧬️schema/⚛️component.rs`, `…📇️directory/🧬️schema/🦀️component.rs`).
- **`🚚️migration.json`** — a shrink-only unmanaged-test-count ratchet by area (`{total: 48, byArea:
  {...}}`). **Not schema-related at all** beyond its own `schemaVersion` key.

### 3.4 Cargo `[package.metadata.semio]` — confirmed field vocabulary

`role` (`"plugin"`/`"tool"`/`"extension"`), `extends`, `depends-on`, `consumes`, `contributes`, plus
repeatable `[[package.metadata.semio.playground]]` tables (`variant`, `app`, `aliases`, `ports`,
`engines`). **No `subset` field exists in this TOML block anywhere** — "subset" is a taxonomy
filesystem-path segment (`🪆️subsets/✳️any`), never a Cargo metadata key. Example
(`✏️s/🔌️plugins/🎪️demonstrator/📦️packages/🦀️rust/Cargo.toml:23-26`):
```toml
[package.metadata.semio]
role = "plugin"
consumes = ["forms.questionKind", "flow.extension", "process.machines"]
depends-on = ["cad", "gis", "procedural", "process", "puzzle", "sourcing"]
```
(4 more examples, including `extends`/`contributes` for an extension package, in the JSON deliverable.)

### 3.5 Runtime JSON-Schema validation — genuinely two different worlds

**Test/conformance-only** (not shipped): the harness `validateAllContracts`
(`🧰️framework/.../🧪️test/📦️packages/🟦️typescript/🟦️.ts:1869`) has **no Ajv import** — hand-written
structural checks only. `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧬️schema-validation.ts`
(19 lines, `isValidJsonSchema2020_12`) is explicitly a "conformance suite" oracle, called only from
`.test.ts` files. `📜️script.ts`'s own self-tests use `new Ajv({strict:true,...})`/`Ajv2020` dozens of
times to validate fixtures against `🧬️schema/*.schema.json` — this is policy self-test tooling, not
shipped runtime.

**Genuine production runtime (Rust)**: `🧰️framework/🔨️modules/🧬️schema/✅️validator.rs` (475 lines) is
a **hand-rolled JSON-Schema compiler/validator** (`OwnedJsonSchemaValidator`, `compile`/
`validate_json`/`is_valid_json`, with cancellation support) — **no external `jsonschema` crate
dependency**, matching CLAUDE.md's "no runtime deps on external libraries" rule. Wired into production
via `⚛️component.rs:40,60` (`SchemaCatalog.register_json`) and
`…/🌉️mcp/🧬️schema/🦀️.rs:289,295`. Separately, `schemars::JsonSchema` derives generate real MCP tool
`inputSchema`/`outputSchema` documents served to live MCP clients
(`…/🌉️mcp/🦀️.rs:384,524` — `schemars::schema_for!(PreparedActionReport)` etc.) — also genuine
production runtime.

### 3.6 Production code reading `🧪️fixtures/` at runtime

**None found.** Every `🧪️fixtures` grep hit outside test/script code was individually checked and is
either inside a `#[cfg(test)]`-reachable function (`🧰️framework/🔨️modules/🧵️job/🦀️.rs:485`) or is a
descriptor *label string*, not a filesystem read
(`🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs:981`). **Conclusion: shipping a plugin without
its `🧪️fixtures/` directory would not break runtime behavior** — only its own test/verify targets.

### 3.7 Packaging — the actual production mechanism

`include_str!` embedding of the sibling `🧬️schema/` files is **the** packaging mechanism, and it is
genuine, massive, production code: **3,804 distinct files** repo-wide contain a schema-related
`include_str!` call. Representative example
(`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs:201-230`,
function `jack_artifact_schema_descriptor()`, not inside `#[cfg(test)]`):
```rust
pub fn jack_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
  ...artifact: FacetLeaves {
    rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"),
    graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"),
    proto: include_str!("🛰️.proto"),
  },
  snapshot: FacetLeaves { rust: include_str!("📸️snapshot/🦀️.rs"), ... },
  diff: FacetLeaves { rust: include_str!("🔺️diff/🦀️.rs"), ... },
  mutations: FacetLeaves { rust: include_str!("🧬️mutations/🦀️.rs"), ... },
}
```
`ArtifactSchemaDescriptor` is defined at `🧰️framework/🔨️modules/🧬️schema/⚛️component.rs:166-172`
(`{id, artifact: FacetLeaves, snapshot: FacetLeaves, diff: FacetLeaves, mutations: FacetLeaves}`),
registered into a runtime `HashMap<&'static str, ArtifactSchemaDescriptor>` via
`register_artifact_schema_descriptor[s]` (lines 319, 346), and **every plugin's declared artifact
registration takes one as an argument** via
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:2992,3050,3095`
(`ArtifactDeclarationBuilder<DeclarationReady>.schema(descriptor)`). **So: every plugin embeds its own
five-language schema siblings directly into its compiled wasm/native binary at build time via
`include_str!`, and registers them at plugin-init runtime — no asset-copy step, no `build.rs`
involvement.** Confirmed by checking all 3 `build.rs` files in the repo (none mention "schema") and
every Vite config (all are emoji-prefixed `⚙️vite.config.ts`, not bare `vite.config.ts` — a naive glob
finds none of them; checked the real filenames, none reference "schema"). Schema packaging is entirely a
Rust-side `include_str!` embed.

---

## 4. "Owner" is an overloaded term — disambiguation for whoever designs WP1+

The word "owner" is used for at least **five unrelated things** in this codebase; conflating them will
produce a design that silently breaks something:

1. **Package owner** (`discoverOwners`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts:9294-9296`)
   — any directory that carries a `📦️packages/` child; used for the clean-mechanism/packaging-purity
   burndown, nothing to do with schema.
2. **Artifact-schema owner** (`policyListPluginArtifactDirs` + `POLICY_ARTIFACT_SCHEMA_PREFIXES`) — a
   `plugin/artifact` pair, hand-listed.
3. **App/surface-schema owner** (`policyDiscoverAppSchemaOwners`) — a `🎚️config`/`🧮️config` directory,
   derived from a `type Config = X` binding.
4. **Mutation owner** (`mutationOwnerIdentity` in `mutationPayloadSchemaAuthority`,
   `mutationDomainOwners`, `mutationCatalogSourceOwners` in taxonomy.json) — a subset path that declares
   the create/delete/rename/... verb vocabulary for one mutation family.
5. **Test-contribution owner** (`registry.contributions[].owner` in the harness, discovered via
   `🔮️oracle`/`testContributionDirName` + `testContributionDirectoryOverrides`) — a subset path that
   contributes oracle/fixture/mutation-manifest data to the test registry.

A "scope owner" for schema contracts, as the ticket frames it, most closely maps to (2)+(3) unified,
with the taxonomy vocabulary in §1 (`artifactComponentDirs`/`subsetComponentDirs`/
`schemaChildDirs`/`schemaFacetKinds`) as the physical-placement layer underneath all of them.

---

## 5. Heuristic / nearest-parent / fallback discovery to REMOVE (or explicitly supersede)

All confirmed by reading the source (not inferred from comments alone):

| # | Mechanism | File : lines | Behavior |
|---|---|---|---|
| 1 | `POLICY_ARTIFACT_SCHEMA_PREFIXES` table lookup | `📜️script.ts:29889-29948`, used at `29974-29981` | Hand-maintained `plugin/artifact → PascalCase prefix` map; artifacts absent from it are silently skipped for type-name parity. Documented fix today is "hand-edit script.ts." |
| 2 | Dual payload-schema convention | `🧰️framework/.../🧪️test/📦️packages/🟦️typescript/🟦️.ts:3579-3596` | Accepts either `🧬️schema/<canonical-json-filename>` **or** flat `🔣️.schema.json` beside a mutation leaf — whichever exists first wins, no single source of truth. |
| 3 | Repo-wide Rust type index (`rustTypeIndex`) | same file, `:3825-3931` (index build) | Walks **every** `.rs` file in the repo (`walkDirectories(repoRoot, ...)`), indexing `pub struct`/`pub enum`/tagged-enum/newtype declarations by **bare name only**. |
| 4 | Nearest-owner disambiguation for ambiguous struct names | same file, `:4130-4165` (`composite` resolution in `resolve`) | When a struct name is declared in >1 place, picks the definition whose file path shares the **longest common path-prefix** with the mutation leaf's owner path — a literal nearest-parent walk. |
| 5 | Nearest-owner disambiguation for ambiguous enum names | same file, `:4102-4119` (`nearest` helper) | Same longest-common-prefix heuristic, applied to `placedEnums`. |
| 6 | Nearest-owner disambiguation for `set-snapshot` composite resolution | same file, `:4028-4060` | Same heuristic again, a third call site, inlined separately inside `resolveSnapshot`. |
| 7 | First-declaration positional fallback | `📜️script.ts:29996-30011` (`policyFindSchemaDeclaration`) | When a schema leaf's declared type name doesn't match the *expected* name (from the prefix table or the derived config type), it silently falls back to **the first declaration found in the file** rather than failing. |
| 8 | Facet-kind resolution by disk probing | `🧰️framework/.../📚️library/🔍️discovery/🟦️.ts:2670-2679` (`resolveSchemaFacetKind`) | Determines whether a `🧬️schema/` dir is "🧬️data" or "📜️interface" kind by checking which normative filename already exists on disk, defaulting to `"🧬️data"` if neither is found — i.e. kind is inferred from current file layout, not declared. |
| 9 | Per-owner directory-name override table for oracle contributions | `🔣️taxonomy.json` → `testContributionDirectoryOverrides` (~140 entries) | A hand-maintained per-subset-path override list, almost all mapping to the same default value (`🔮️oracle`) the un-overridden case already uses — a sign the override mechanism is mostly vestigial/copy-pasted rather than load-bearing. |

Items 3-6 are one connected mechanism (`rustTypeIndex` + its three nearest-parent resolution call sites)
and are almost certainly the single biggest thing a "scope-owned, explicitly-resolved (scope, export
id, format)" design needs to retire: today, a JSON Schema for a mutation payload is **derived**, at
generation time, by grep-scanning the entire Rust tree for a same-named struct and guessing which one is
"the" owner by directory proximity — the opposite of an explicit, scope-declared export catalog.

---

## 6. Extension points (file + function/key + line) for WP1+

| Concern | Where to hook | Why |
|---|---|---|
| Declare a scope's eligible schema exports | New taxonomy.json key, sibling to `schemaFacetKinds`/`artifactSchemaSpecFileKinds` (§1.1/1.3) | Keeps the "what formats/filenames are expected" layer (already generic) next to a new "what exports exist" layer. |
| Resolve (scope, export id) → formats/filenames | `schemaFacetFormatEntries`/`resolveSchemaFacetKind`, `🧰️framework/.../📚️library/🔍️discovery/🟦️.ts:2670-2691` | Already the one general-purpose choke point every schema policy check calls through; extending its signature (or adding a sibling `resolveNamedSchemaExport`) reaches every consumer at once. |
| Replace `POLICY_ARTIFACT_SCHEMA_PREFIXES` | `📜️script.ts:29889-29981` (table + `policyArtifactSchemaPrefix`) | Direct swap target: replace table lookup with a scope-declared/derived lookup, mirroring `policyDiscoverAppSchemaOwners`'s already-source-derived approach (§2.1.2). |
| Generalize explicit fixture binding to schema exports | `resolveFixtures`, harness 🟦️.ts:996-1015, and `fixtureUrisIn`, 982-987 | The URI-scheme + explicit-directory-join + digest-pin pattern is the template; a `schema://<scope>/<exportId>` scheme could be added the same way, replacing the `schemaCandidates.find(existsSync)` fallback at 3579-3596. |
| Retire the `rustTypeIndex` nearest-parent heuristic | harness 🟦️.ts:3825-3931 (index), 4028-4060/4102-4119/4130-4165 (resolution) | Once payload schemas are scope-declared exports resolved by explicit id, this whole repo-wide scan-and-guess mechanism becomes unnecessary for the cases it's meant to serve; keep only if something else still needs "find any struct anywhere" (unlikely). |
| New catalog contract shape | `mutationCatalogProjection` + its 3 referenced contract families in taxonomy.json (§1.8) | Existing, working precedent for "declare {projection, descendant, catalog} contract ids, generate/verify via script.ts" — clone this shape for a schema-export catalog instead of inventing a new contract taxonomy. |
| Named/versioned identity precedent | `stdioDefinitionCatalog`, `📜️script.ts:19519-19544`, dotted ids `s.stdio.<slug>.v<N>` (`stdioDefinitionIdentity`/`stdioVersionedLeaf`, `19472-19479`) | Working example of an explicit catalog (`artifact_definition_paths[]`) cross-checked against a full-tree walk for completeness, with canonical versioned identities — closest existing model for "(scope id, export id)" naming, though currently stdio-only and describes container/mutation definitions rather than JSON-Schema field contracts. |
| Test-harness oracle "discovered by convention" precedent (tension to resolve) | `🧰️framework/.../🧪️test/📇️registry/🔣️.json:4` (comment) | Explicitly states owners are found by **filename convention** (`<owner>/🔣️oracle.json`), not central cataloging — decide deliberately whether the new schema-export mechanism should be catalog-based (like `mutationCatalogProjection`) or convention-based (like this), since both patterns coexist today for different concerns. |

---

## 3.7.1 The nearest existing skeleton for "(scope id, export id, format)" — verified directly, not
just inferred

`🧰️framework/🔨️modules/🧬️schema/⚛️component.rs:130-172` already declares almost exactly the shape the
ticket goal describes, at reduced cardinality:

```rust
pub struct FacetLeaves {                    // (export id, format) -> body, but fixed to 5 known formats
    pub rust: &'static str,
    pub typescript: &'static str,
    pub graphql: &'static str,
    pub json_schema: &'static str,
    pub proto: &'static str,
}

pub struct ArtifactSchemaDescriptor {       // scope id + exactly 4 fixed "exports"
    pub id: &'static str,                   // e.g. "s.trinity.jack" — the scope id, ALREADY dotted
    pub artifact: FacetLeaves,
    pub snapshot: FacetLeaves,
    pub diff: FacetLeaves,
    pub mutations: FacetLeaves,
}
```

Registered/deduped via `register_artifact_schema_descriptor[s]`/`preflight_artifact_schema_descriptors`
(lines 317-347) into `with_artifact_schema_registry`'s `by_id: HashMap<&'static str,
ArtifactSchemaDescriptor>` (line 196) — **id collisions with a differing body are already a hard
registration error** (`SchemaDescriptorRegistryError`), so exact-duplicate-safe, differing-conflict-fatal
registration is already implemented.

**What this means for the ticket**: the "scope id" half of "(scope id, export id, format)" already
exists, verified, as `ArtifactSchemaDescriptor.id`, in the dotted form the stdio catalog also uses
(`"s.trinity.jack"`, cf. `"s.stdio.<slug>.v<N>"` in §6). What does **not** exist is an "export id"
dimension — `FacetLeaves` hard-codes exactly 5 fixed format fields for exactly 4 fixed facet names
(`artifact`/`snapshot`/`diff`/`mutations`); there is no way today to register an arbitrary *named*
export beyond those 4, and no per-facet way to have more than one JSON Schema/Rust/TS/GraphQL/proto
body. Extending `FacetLeaves`/`ArtifactSchemaDescriptor` to a keyed collection (`exports:
BTreeMap<&'static str, FacetLeaves>` or similar) rather than inventing a new type is the most direct
path from today's mechanism to the ticket's target model.

## 3.8 Packaging implication for WP1+

Because every plugin's schema siblings are `include_str!`-embedded at compile time (§3.7), **any new
"named export" catalog mechanism must resolve at build/generation time, not at plugin-init runtime** —
there is no load-bearing runtime file-read to intercept or redirect; the binary already contains the
bytes. A scope-owned export catalog would most naturally live as an addition to
`ArtifactSchemaDescriptor`/`FacetLeaves` (or a sibling struct) and to the `include_str!` call sites
themselves (or to codegen that emits them), not as a new runtime lookup path.

## Files read in full or in relevant part during this audit

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `📜️script.ts` (root) — targeted regions: 1-160 (imports), 847-1153 (Generate/taxonomy helpers),
  10685-10900 (VerifyScript), 19428-19545 (stdio catalog types), 20202-21286 (taxonomy/mutation-taxonomy
  CLI), 22629-22960 (dispatch + Neo4j export), 27900-27945 (`policyListPluginArtifactDirs`),
  29880-31150 (all `policy*Schema*` rule functions), end-of-file (`policy` export, CLI entrypoint)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts` — targeted regions:
  2650-2710 (`resolveSchemaFacetKind`/`schemaFacetFormatEntries`), 9230-9300 (`discoverOwners`/
  `discoverPackages`)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📇️registry/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts` — targeted
  regions: 960-1120 (fixture resolution/oracle host packages), 3250-3630 (mutation leaf descriptors +
  scaffold), 3790-4165 (Rust type index + payload schema derivation)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts` — targeted regions: 825-1200 (phase
  CLI classes), 1260-1315 (`payloadSchemaCommand`), 1439-1470 (`ManifestScript`)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs` (full, 200 lines)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/SCOPE-OWNED-SCHEMA-CONTRACTS/🎫️ticket.json`

Additionally read/grepped by the parallel build/launch/runtime/packaging sub-audit (§3): `nx.json`,
`📋️project.json` (targeted regions), `package.json`, `.vscode/launch.json` (26,771 lines, targeted
regions), `🔒️dependencies.json`, `🧅️layering.json`, `🚚️migration.json`, five plugin `Cargo.toml`
files under `✏️s/🔌️plugins/`, `🧰️framework/🔨️modules/🧬️schema/✅️validator.rs`,
`🧰️framework/🔨️modules/🧬️schema/⚛️component.rs`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🦀️.rs`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧬️schema-validation.ts`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🧩️pair/🧪️oracle/🟦️.ts`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs`,
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`,
`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`,
`🧰️framework/🔨️modules/🧵️job/🦀️.rs`, `🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️.rs`, all 3
`build.rs` files in the repo, and every `⚙️vite.config.ts` in the repo.
