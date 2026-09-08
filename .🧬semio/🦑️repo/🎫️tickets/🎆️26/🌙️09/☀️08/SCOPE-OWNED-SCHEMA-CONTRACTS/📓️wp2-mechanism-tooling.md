# WP2 — Mechanism and tooling (taxonomy vocabulary, scope catalog, `schema` commands)

Partition: root `📜️script.ts`, root `📋️project.json`, `package.json`, `.vscode/launch.json`, `nx.json`,
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/**`. Nothing outside it was edited.

## 1. Files changed

| File | Change |
|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | New keys `schemaJsonDialect`, `schemaDefaultFacetKind`, `schemaScopeOwnerLevels`, `schemaExportResolution`; `schemaFacetKinds.*` gained `facetPathIdentities`; `pluginChildDirs` gained `🧬️schema`; 174 vestigial `testContributionDirectoryOverrides` rows dropped (2 real overrides kept). |
| `…/📚️library/🔍️discovery/🟦️.ts` | `Taxonomy` fields for the new keys; `validateSchemaScopeVocabulary`; `resolveSchemaFacetKind`/`schemaFacetFormatEntries` no longer probe disk (signature dropped `repoRoot`); new region `🧬️SchemaScopeCatalog` (walk, document read, `$ref` classification, catalog build, renderers). |
| `…/📚️library/📜️script.ts` | The 1 MiB folder lint now measures **authored** children and skips the two taxonomy-declared generated catalog outputs. |
| `…/📚️library/📦️packages/🟦️typescript/🔬️index.test.ts` | New `describe("schema scope catalog")` block: 8 language-agnostic fixture cases, identity/URI/facet-kind assertions, ajv draft-07 oracle. |
| `…/📚️library/🧪️tests/🧬️schema-scope-catalog/🧫️fixtures/🔣️.json` | The 8 language-agnostic cases (new). |
| `…/📚️library/🧪️tests/🧬️schema-scope-catalog/🛂️schema/🔣️.json` | Draft-07 authority for the case file, compiled by ajv as the third-party oracle (new). |
| `…/📚️library/🔣️schema-catalog.json` | **Generated** derived catalog (new). |
| `…/📚️library/📓️schema-catalog.md` | **Generated** Markdown scope/export index (new). |
| `📜️script.ts` | New `SchemaScript` (`schema audit\|check\|docs\|generate\|test\|verify`) registered on the router; retired `POLICY_ARTIFACT_SCHEMA_PREFIXES` + `policyArtifactSchemaPrefix` + `policyExpectedSchemaTypeName` + `policyExpectedSchemaTypeNameForFacetPath`; `policyFindSchemaDeclaration` lost its first-declaration fallback; `policyPluginRootShapeBreaches` now reads `pluginRequiredChildDirs`; nine `canonicalFilenameForKind` calls in the schema paths became `canonicalPrimaryFilenameForKind`. |
| `📋️project.json` | Targets `schema-generate`, `schema-docs`, `schema-check`, `schema-verify`, `schema-audit`, `schema-test`. |
| `package.json` | Scripts `schema:generate\|docs\|check\|verify\|audit\|test` → `bun nx run workspace:schema-*`. |
| `nx.json` | New named input `schemaSources`. |
| `.vscode/launch.json` | Six entries, group `4_build`, orders 208.1–208.6. |

Ticket-owned inputs (kept, per the rules): `wp2-taxonomy-edit.ts` (the idempotent taxonomy edit),
`wp2-schema-cases.ts` (a standalone runner for the same fixture cases, written while a peer's in-flight
`📃️pageSchema` rename broke the root `📜️script.ts` import graph the library spec pulls in).

## 2. Taxonomy keys added (verbatim JSON)

`schemaJsonDialect` — the single dialect, validated to equal `mutationPayloadSchemaAuthority.jsonSchemaDialect`:

```json
"schemaJsonDialect": "http://json-schema.org/draft-07/schema#",
"schemaDefaultFacetKind": "🧬️data"
```

`schemaScopeOwnerLevels` — the eligible owner levels from contract §A, plus the never-eligible shapes:

```json
"schemaScopeOwnerLevels": {
  "facetDirName": "🧬️schema",
  "directoryKindId": "schema",
  "levels": {
    "plugin-root": {
      "pathPatterns": ["✏️s/🔌️plugins/*"],
      "reason": "A plugin root owns its own identity contracts (plugin-identity, artifact-identity) that no single artifact may claim."
    },
    "artifact-standard-subset": {
      "pathPatterns": ["✏️s/🔌️plugins/*/🗿️artifacts/*/🏅️standards/*/🪆️subsets/*"],
      "reason": "A subset is the only level that owns artifact data: standards own subsets, subsets own schema, io and examples."
    },
    "surface-state-lane": {
      "pathPatterns": ["**/✏️editor/🎚️config", "**/✏️editor/👥️presence", "**/✏️editor/🫧️transient"],
      "reason": "Persisted local-only, ephemeral shared and ephemeral local-only surface state each own their own contract."
    },
    "framework-module": {
      "pathPatterns": ["🧰️framework/🔨️modules/**"],
      "reason": "A domain-neutral framework module owns the contracts its two independent consumers share."
    },
    "product-module": {
      "pathPatterns": ["🧰️framework/🛍️products/*/🔨️modules/**"],
      "reason": "A product module owns the contracts of the product surface it implements."
    },
    "hub-area-module": {
      "pathPatterns": ["🌎️hub/**"],
      "reason": "A hub area owns its protocol contracts; os and mcp mirrors are consumers, never co-owners."
    },
    "mutation-leaf": {
      "pathPatterns": ["**/🧬️schema/🧬️mutations/*"],
      "reason": "mutationPayloadSchemaAuthority makes the leaf the authority for its own payload; aggregates are $ref unions."
    }
  },
  "excludedOwnerPathPatterns": [
    "**/🧱️elements", "**/🧱️elements/**",
    "**/🎯️targets", "**/🎯️targets/**",
    "**/📦️packages", "**/📦️packages/**",
    "**/🧪️*", "**/🧪️*/**",
    "**/🧫️*", "**/🧫️*/**",
    "**/🧬️contracts", "**/🧬️contracts/**"
  ]
}
```

`schemaExportResolution` — the `schema://` scheme, the `$id` grammar the scope id is derived from, the
catalog paths and the retired placements:

```json
"schemaExportResolution": {
  "uriScheme": "schema",
  "uriPattern": "^schema://(?<scope>[a-z0-9]+(?:-[a-z0-9]+)*(?:\\.[a-z0-9]+(?:-[a-z0-9]+)*)*)/(?<export>[A-Z][A-Za-z0-9]*)$",
  "idBase": "https://semio.tech/schema/",
  "idFacetFilenamePattern": "^[a-z0-9]+(?:-[a-z0-9]+)*\\.json$",
  "scopeIdSeparator": ".",
  "scopeIdPattern": "^[a-z0-9]+(?:-[a-z0-9]+)*(?:\\.[a-z0-9]+(?:-[a-z0-9]+)*)*$",
  "exportIdPattern": "^[A-Z][A-Za-z0-9]*$",
  "rootExportKeyword": "title",
  "exportsKeyword": "$defs",
  "catalogPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json",
  "catalogDocumentPath": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📓️schema-catalog.md",
  "catalogContractId": "schema-scope-catalog-v1",
  "generator": "bun ./📜️script.ts schema generate",
  "forbiddenPlacementPatterns": ["**/*.schema.json", "**/🧬️schema.json", "**/🧬️contracts", "**/🧬️contracts/**"],
  "placementExceptions": {}
}
```

`schemaFacetKinds` gained a declared (never probed) claim list; the default kind must claim nothing:

```json
"schemaFacetKinds": {
  "🧬️data": { "normativeFormat": "🔣️jsonschema", "formats": ["🔣️jsonschema", "🦀️rust", "🟦️typescript", "🔗️graphql", "🛰️protobuf"], "facetPathIdentities": [] },
  "📜️interface": { "normativeFormat": "📜️wit", "formats": ["📜️wit"], "facetPathIdentities": ["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema"] }
}
```

`pluginChildDirs` now admits the plugin-root scope: `["🎮️commands", "🔨️modules", "🧬️schema"]`.

### Decisions inside the taxonomy work

- **`schemaVersion` stays 7.** The file's own rule is a hard pin (`validateTaxonomy`
  `schemaVersion !== 7`, `🧹️normalization/🟦️.ts:905`) with no stated increment policy for additive
  vocabulary, and frozen taxonomy snapshots under `…/26/08/12/SEMANTIC-MUTATIONS-OVERHAUL/📸️source-index-capture-66/**`
  embed `"schemaVersion": 7` as historical evidence. Nothing in the file requires a bump for additions,
  so none was made; the catalog records `taxonomySchemaVersion: 7` as provenance.
- **The plugin-root `🧬️schema` slot is `pluginChildDirs`, not `scopedFileKinds`/`fixedDirectoryContracts`.**
  Contract §C names those two families, but on inspection neither is the mechanism: all 8 `scopedFileKinds`
  entries are taxonomy-transaction staging *evidence* files (`**/💾️backup/🚧️restore-*/*.backup` etc.), and
  `fixedDirectoryContracts` has zero schema-related entries — plugin-root directory admission is decided by
  `policyPluginClosedShapeBreaches`, which derives its allowed set from `pluginChildDirs ∪ artifactsDirName ∪
  packagesDirName ∪ rootDataDirNames`. `semanticDirectoryKinds.schema` already admits `🧬️schema` at any
  level, so the one real change was `pluginChildDirs`.
- **`pluginChildDirs` vs `pluginRequiredChildDirs` were conflated in `📜️script.ts`.**
  `policyPluginRootShapeBreaches` iterated `pluginChildDirs` and demanded a `🦀️.rs` leaf for each, i.e. it
  read the *admitted* set as the *required* set while `pluginRequiredChildDirs` (`["🎮️commands"]`) sat
  unused. Adding `🧬️schema` to the admitted set would otherwise have demanded a schema module from all 33
  plugins on day one. Fixed: the shape rule now reads `pluginRequiredChildDirs`, the closed-shape rule keeps
  reading `pluginChildDirs`.

## 3. Commands

`bun ./📜️script.ts schema <sub>`, registered on the root router between `new` and `lint`.

| Subcommand | Behaviour | Exit |
|---|---|---|
| `generate [--check]` | Walks every `🧬️schema` module and rewrites `🔣️schema-catalog.json`. `--check` compares instead of writing. | 1 on `--check` drift |
| `docs` | Rewrites `📓️schema-catalog.md` (scope, level, module, exports, dependsOn). | 0 |
| `check [--report <path>]` | Every invariant as one stable JSON line (`{code, path, detail}`) plus a per-code summary. Without `--report` the lines go to stdout. | 1 when any finding |
| `verify` | Catalog + index freshness against the current sources, plus the catalog's own `generator`/`taxonomySchemaVersion` provenance. | 1 on drift |
| `audit --out <dir>` | Writes `📊️schema-audit.json` (catalog + module roster + findings) and `📓️schema-audit.md` (modules per level, findings per code). | 0 |
| `test [...]` | Delegates to `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts test schema …`. | harness's |

Nx targets (`📋️project.json`): `schema-generate`, `schema-docs`, `schema-check`, `schema-verify` all
declare `"inputs": ["schemaSources"]`; `schema-generate`/`schema-docs` declare their tracked output;
`schema-audit`/`schema-test` are `"cache": false`. `package.json` adds `schema:generate|docs|check|verify|audit|test`,
each a `bun nx run workspace:schema-*` wrapper.

`nx.json` named input (so any schema source, the taxonomy or the discovery library invalidates the catalog):

```json
"schemaSources": [
  "sharedGlobals",
  "{workspaceRoot}/**/🧬️schema/**/*",
  "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
  "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts"
]
```

`.vscode/launch.json`, group `4_build`, inserted directly after `📦️generate🧩️taxonomy🧬️duplicates` (order 208):

| Order | Name | Command |
|---|---|---|
| 208.1 | `📦️generate🧬️schema📇️catalog` | `bun nx run workspace:schema-generate` |
| 208.2 | `📦️generate🧬️schema📓️docs` | `bun nx run workspace:schema-docs` |
| 208.3 | `📦️verify🧬️schema🚦️check` | `bun nx run workspace:schema-check` |
| 208.4 | `📦️verify🧬️schema📇️catalog` | `bun nx run workspace:schema-verify` |
| 208.5 | `📦️generate🧬️schema📋️audit` | `bun nx run workspace:schema-audit -- --out .🧬semio/…/SCOPE-OWNED-SCHEMA-CONTRACTS/🗑️generated` |
| 208.6 | `📦️test🧬️schema🧪️harness` | `bun nx run workspace:schema-test` |

## 4. Catalog shape (contract §C)

```json
{
  "contractId": "schema-scope-catalog-v1",
  "catalogVersion": 1,
  "generator": "bun ./📜️script.ts schema generate",
  "taxonomySchemaVersion": 7,
  "jsonSchemaDialect": "http://json-schema.org/draft-07/schema#",
  "scopes": {
    "s.trinity.jack": {
      "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
      "level": "artifact-standard-subset",
      "facetKind": "🧬️data",
      "formats": { "🦀️rust": "🦀️.rs", "🟦️typescript": "🟦️.ts", "🔗️graphql": "🔗️.graphql", "🔣️jsonschema": "🔣️.json", "🛰️protobuf": "🛰️.proto" },
      "exports": ["Camera", "Edge", "JackArtifact", "JackDiff", "…"],
      "dependsOn": [],
      "hashes": { "🔣️.json": "…", "📸️snapshot/🔣️.json": "…" }
    }
  }
}
```

Resolution rules the generator applies, all declaration-driven:

- **Scope id** comes only from a document `$id` under `idBase`, minus the trailing facet filename, joined
  with `.`. A module's scope is its **root** `🔣️.json`'s id; a facet child (`📸️snapshot/🔣️.json`,
  `🧬️mutations/📝️text/🔣️.json`, …) may extend it (`s.trinity.jack.snapshot`) but never contradict it.
- **Exports** = `$defs` keys plus the root `title` when the root is an object schema (a `oneOf`/`$ref`
  root contributes no root export, per contract §B).
- **`$ref` resolution is by `$id`, never by scope-id prefix.** A cross-scope `$ref` must be
  `<target $id>#/$defs/<ExportId>`; the target is looked up in a `$id → document` index, and `dependsOn` is
  that document's owning scope. Anything else is `ref-not-catalog-addressable`, and an unknown `$id` or an
  export the target does not declare is `ref-unresolved`. No nearest-parent search anywhere.
- **`formats`** are module-relative leaf filenames (join with `path`); **`hashes`** are sha256 over the
  module's declared format leaves along declared facet chains (`schemaChildDirs ∪ representationDirs`),
  excluding nested scopes (a mutation leaf's own module is a separate scope) and every `🧪️*`/`🧫️*` subtree.
- Ordering is byte-order throughout (scopes, exports, dependsOn, hashes), so the rendering is deterministic;
  the fixture suite asserts `render(inventory) === render(inventory')` per case.
- **Diagnostics are not in the tracked catalog** — they belong to `check`/`audit`. Keeping them in made the
  file 24.9 MB; the catalog is now ~0.95 MB.

## 5. Heuristics retired (WP0 §5)

| Row | Was | Now |
|---|---|---|
| 1 | `POLICY_ARTIFACT_SCHEMA_PREFIXES`, a 65-row hand-maintained `plugin/artifact → PascalCase` table; artifacts absent from it were silently skipped, and the documented fix was "hand-edit `📜️script.ts`" | `policyDeclaredSchemaExportName(repoRoot, facetRel)` reads the facet's own normative JSON Schema `title` — the same root export the catalog publishes. A facet that declares none is a breach (`artifact-schema-export-undeclared-…`), never a skip. The table and both helpers are deleted. |
| 7 | `policyFindSchemaDeclaration` fell back to the *first* declaration in the file when the expected name was absent or unmatched | Fails instead: a null `expected` returns null, and a non-matching file returns null. The parity rules then report the leaf as missing its declared export. |
| 8 | `resolveSchemaFacetKind` probed disk (`does 🔣️.json exist? does 📜️.wit exist?`, default `🧬️data`) | Pure declaration: `schemaFacetKinds[<kind>].facetPathIdentities` claims exact modules, everything else is `schemaDefaultFacetKind`. `resolveSchemaFacetKind`/`schemaFacetFormatEntries` dropped their `repoRoot` parameter; all six call sites updated. |
| 9 | 176 `testContributionDirectoryOverrides` rows, 174 of which restated the default `🔮️oracle` | 2 rows (the real `⚖️oracle` overrides). Every consumer already used `?? testContributionDirName`, so the removal is behaviour-preserving. |

Rows 2–6 are the harness worker's (`rustTypeIndex` + its three nearest-parent call sites, and the dual
payload-schema convention).

Incidental fix in the same family: nine `canonicalFilenameForKind(...)` calls in the schema policy paths
threw `File kind "typescript-source" must have exactly one extension chain, got 5`, which made
`policyArtifactSchemaBreaches` abort before producing any record. They now use
`canonicalPrimaryFilenameForKind`, and the rule set runs (582 records, below).

## 6. Real command output

Taxonomy still validates (0 problems) and `loadTaxonomy()` now succeeds — the pre-existing
`generatorContracts["print-latex-tokens"/"report-actor-network"].previewTarget` failure the WP0 auditor
reported was real (it made **every** `loadTaxonomy()` consumer throw) and was fixed by another worker
during this session; it was never papered over here. The schema tooling deliberately reads
`loadCatalogTaxonomy()`, the loader documented as "validates catalog vocabulary without reading unrelated
generator outputs", because generator contracts have nothing to do with schema scopes.

```
$ bun -e 'validateTaxonomy(loadCatalogTaxonomy())'
loadTaxonomy OK
validateTaxonomy problems: 0

$ bun test …/🔬️index.test.ts -t "schema scope catalog"
 3 pass
 584 filtered out
 0 fail
 47 expect() calls

$ bun …/SCOPE-OWNED-SCHEMA-CONTRACTS/wp2-schema-cases.ts
[wp2-schema-cases] all 8 fixture cases and 13 identity checks pass.
```

```
$ bun ./📜️script.ts schema generate
[schema generate] 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json: 478 scopes, 8179 diagnostics.
$ bun ./📜️script.ts schema docs
[schema docs] 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📓️schema-catalog.md: 479 scopes.
$ bun ./📜️script.ts schema check --report <ticket>/🗑️generated/schema-check.jsonl
[schema check] wrote 8395 findings to …/🗑️generated/schema-check.jsonl
[schema check] modules=3259 scopes=479 findings=8395
[schema check] catalog-stale=1
[schema check] document-dialect-unexpected=1020
[schema check] document-id-duplicate=11
[schema check] document-id-missing=84
[schema check] document-id-unaddressable=91
[schema check] document-not-object=1
[schema check] export-id-invalid=502
[schema check] module-level-ineligible=571
[schema check] module-scope-id-inconsistent=135
[schema check] module-scope-id-missing=133
[schema check] placement-retired-location=215
[schema check] ref-not-catalog-addressable=2771
[schema check] ref-not-export-addressed=537
[schema check] ref-unresolved=74
[schema check] scope-id-duplicate=2249
check exit=1
$ bun ./📜️script.ts schema verify
[schema verify] stale generated output: …/🔣️schema-catalog.json. Run bun ./📜️script.ts schema generate && bun ./📜️script.ts schema docs.
verify exit=1
```

`catalog-stale`/`verify` staleness inside a single chained run is **not** non-determinism: the scope count
moved 478 → 479 between `generate` and `docs` because other workers are writing `$id`s into the tree right
now. Determinism is asserted instead on a frozen tree — every fixture case checks
`render(inventory) === render(inventory')`.

```
$ bun ./📜️script.ts schema test          # delegates to the harness module
[test schema] 4573 invariant finding(s) over the repository
[test schema]    1691 × schema-export-unknown
[test schema]    1001 × schema-export-incomplete
[test schema]     565 × schema-owner-ineligible
[test schema]     424 × schema-ref-unresolved
[test schema]     307 × schema-dialect-not-draft-07
[test schema]     298 × schema-fixture-defines-schema
[test schema]     228 × schema-placement-forbidden-filename
[test schema]      57 × schema-placement-outside-module
[test schema]       2 × schema-contracts-directory-forbidden
```

```
$ bun -e 'policyArtifactSchemaBreaches(cwd)'      # the rows 1/7 replacement, running
artifact-schema breaches: 582
{ "artifact-schema/facet-completeness": 480, "artifact-schema/normative-leaf": 51, "artifact-schema/type-name-parity": 51 }
first type-name-parity summaries:
  "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🧬️schema" declares no export id in its normative JSON Schema
```

Generated evidence in `🗑️generated/` (this WP's files only; other workers write there too):
`schema-check.jsonl`, `schema-run.txt` (the chained generate/docs/check/verify transcript quoted above),
`schema-test.txt`, `📊️schema-audit.json`, `📓️schema-audit.md`.

Final state left on disk: `schema generate` + `schema docs` re-run last, at 480 scopes.

## 7. Cross-partition requests

1. **Harness (`…/🧪️test/**`) — already delivered, keep the contract.** `bun …/🧪️test/📜️script.ts test schema`
   exists and reads `🔣️schema-catalog.json` cleanly (`readSchemaCatalog` returned 0 diagnostics against the
   current catalog). Two notes for that owner:
   - The harness's `readSchemaCatalog` requires `path`, `formats`, `exports`, `dependsOn` per scope and
     rejects a `formats` value that is absolute or contains `..`. The generator now emits **module-relative
     leaf filenames** (`"🔣️.json"`), which is what the harness's own message ("must name a file relative to
     `scope.path`") asks for. If the harness ever joins `formats` against the repo root instead of against
     `scope.path`, that is the side to change.
   - A run of `schema test` taken while `schema generate` was mid-write produced 460 spurious
     `schema-catalog-malformed` rows. Consider reading the catalog once, or tolerating a torn read, if the
     gate is ever run concurrently with generation.
2. **Harness — fixture cases.** The six cases the assignment named are implemented in *my* partition as
   language-agnostic JSON at `…/📚️library/🧪️tests/🧬️schema-scope-catalog/🧫️fixtures/🔣️.json`
   (`valid-module`, `cross-scope-reference`, `duplicate-scope-id`, `unresolved-reference`, `wrong-dialect`,
   `fixture-owned-schema`, `per-contract-directory`, `ineligible-level`). If the harness wants the same
   cases under its own fixture convention, reuse that file rather than re-authoring the expectations.
3. **Rust framework worker.** The catalog's `formats` are module-relative filenames and its scope ids are
   exactly the dotted ids `ArtifactSchemaDescriptor.id` already carries (`s.trinity.jack` verified). When
   `register_scope_schema_exports` lands, the export ids it registers should equal the catalog's `exports`
   array for that scope; `schema check` can then gain a Rust-vs-catalog parity code.
4. **Everyone migrating `$id`s.** `scope-id-duplicate=2249` and `module-scope-id-inconsistent=135` are the
   two largest structural blockers. The dominant pattern is many modules whose facet documents resolve to a
   scope id that another module's root already owns (`$id` paths that encode the *facet* chain into the scope
   path, e.g. `…/s/energy/model/1/any/mutation/…`). The per-scope rows are in
   `🗑️generated/schema-check.jsonl`.
5. **Nobody, but worth knowing.** The `📚️library` folder lint (1 MiB per child) now skips the two
   taxonomy-declared generated catalog outputs. `🔣️schema-catalog.json` is 0.95 MB today and will exceed
   1 MiB as migration adds scopes; without that exemption the lint would have started failing on a
   generated artifact.

## 8. Open questions

- **`hub-area-module` breadth.** Contract §A says `🌎️hub/<area>`; the declared pattern is `🌎️hub/**` so
  that nested hub modules (`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog`, an owner §D names) are eligible.
  Narrow it if hub modules are meant to be exactly one level deep.
- **Facet sub-scopes.** `📸️snapshot/📝️text/🔣️.json` declares `$id …/s/trinity/jack/snapshot/text.json`,
  which resolves to the scope id `s.trinity.jack.snapshot`. The generator treats such a document as a facet
  of the module's root scope (allowed when it extends the root scope id) rather than as its own scope. If
  facet documents are meant to be addressable as separate scopes, the `$id` grammar needs a rule that
  distinguishes "deeper scope" from "deeper facet"; today the facet is everything after the scope path, and
  only the last segment is treated as the facet filename.
- **`schema check` in `verify gate`.** The command is deliberately *not* wired into `verify gate` while
  8 395 findings stand. WP7's closing step is to add it once the count reaches zero.
- **Walk cost.** One `inventorySchemaScopes` pass walks ~53 k directories / ~79 k files and hashes ~5 k
  schema leaves: ~8 s of CPU, but 60–90 s wall on the currently saturated machine. The nx `schemaSources`
  input caches it; a cold `schema check` is not fast.
