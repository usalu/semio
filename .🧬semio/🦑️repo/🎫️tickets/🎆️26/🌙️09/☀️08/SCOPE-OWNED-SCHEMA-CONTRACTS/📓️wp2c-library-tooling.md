# WP2c — library tooling (catalog generator, taxonomy vocabulary, library consumers)

Partition: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/**` only. Nothing outside it was edited.
Predecessor W2b left partial work on disk (`mutation-leaf-id-grammar`, `schemaRustEntryDiagnostics`,
`formatIds`/`rustEntriesContractId`/`mutationScopeSegment`/`mutationLeafFacetFilename`, the `♻️mit-bestand`
product-module pattern, the `testDemonstratorRuntime` rewrite, three of the catalog fixture cases). All of
that was kept; the rest of this report is what was finished on top of it.

## 1. Files changed

| File | Change |
|---|---|
| `…/📚️library/🔍️discovery/🟦️.ts` | Catalog `exports` shape, facet derivation, collection rule, `inertSchemaData`, mutation-leaf/aggregate grammar, repo-wide `$id` uniqueness, `dependsOn` verification, `enum: []`, `x-semio-formats` completeness, module-internal `$ref` rule, `AreaState` + `exempt` area validation. |
| `…/📚️library/🔣️taxonomy.json` | `schemaScopeOwnerLevels`: `plugin-extension`, `plugin-submodule`, two-segment mutation leaves, `moduleMemberDirName`, `mutationsFacetDirName`, `inertSchemaDataKeyword`, rewritten `fixtureOwnerReason`, `**/🗿️artifacts` + `**/🧩️extensions` exclusions; `schemaExportResolution.mutationAggregateFacetFilename`; `areas[".🧬semio/🦑️repo/🎫️tickets"] = "exempt"`; `members-of-fixtures.memberNames` −3, `members-of-members-of-members-of-modules.memberNames` −1; `generatorContracts.schema-entity-catalog.outputRoots` Go path. |
| `…/📚️library/🔣️schema-catalog.json` | **Generated.** 480 → 3 021 scopes, new `exports: { <ExportId>: { file, facet } }` shape. |
| `…/📚️library/📓️schema-catalog.md` | **Generated.** Export column now names the declaring file. |
| `…/📚️library/🧪️tests/🧬️schema-scope-catalog/🧫️fixtures/🔣️.json` | 11 → 14 language-agnostic cases; `exports` expectations became maps; `fixture-owned-schema` → `fixture-defines-schema` with findings; new `inert-schema-data`, `restricted-format-support`, `module-internal-definitions`. |
| `…/📚️library/🧪️tests/🧬️schema-scope-catalog/🛂️schema/🔣️.json` | Case authority: `Scope.exports` is an object of `{file, facet}`. |
| `…/📚️library/🧪️tests/🧬️schema-rust-entries/🧫️fixtures/🔣️.json` + `🛂️schema/🔣️.json` | Same shape change for the Rust-registry cross-check cases (7 scopes converted). |
| `…/📚️library/🧪️tests/🔬️index/🟦️.ts` | New `exports` shape in the catalog spec; **repaired 9 corrupted string literals** and 8 stale fixture base paths left by the suite relocation (see §7). |
| `…/📚️library/🧪️tests/🗺️testing-readme-coordinates/🟦️.ts:127` | `$id` expectation → `https://semio.tech/schema/repo/test/schema.json` (row 61). |
| `…/📚️library/🐹️.go:345` | `/api/v1/events` → `/api/v1/event` (row 17). |
| `…/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json` | Dropped the two rows for deleted coordinator files and recomputed the derived census counts (row 16). |
| ticket-owned `wp2-schema-cases.ts` | Runner updated to the new `exports` shape (kept as an input file). |

## 2. Row-by-row result

| Row | Result |
|---|---|
| **2** | Kept W2b's `schemaRustEntryDiagnostics(catalog, dump, taxonomy, complete)`; adapted it and its 7 fixture cases to the map-shaped `exports`. `schema verify --rust-entries <file>` unchanged. Green (`4 pass` in the `schema scope catalog` block). |
| **11 / 78** | A `🧬️mutations/<leaf>/🧬️schema/` module is a scope of its own (`level: mutation-leaf`, catalogued). `mutation-leaf-id-grammar` now enumerates **leaves from their descriptors**, not from the modules that happen to exist, so coverage is total: every directory under `<module>/🧬️schema/🧬️mutations/` at depth 1 **or 2** whose `🔣️.json` declares a `semanticKind`. Expected `$id` = `<root module $id scope path>/mutation/<semanticKind>/schema.json`, where the scope path is read from the enclosing module's **real root `$id`** (never a dotted artifact-level id, never the filesystem path). A leaf with no `🧬️schema/🔣️.json` yields `mutation-leaf-schema-absent` (currently 0). No double counting: `module-scope-id-inconsistent` is suppressed for documents inside a mutation leaf, and the aggregate `🧬️mutations/🔣️.json` is governed by a dedicated `mutation-aggregate-id-grammar` (expected `<root scope path>/mutations.json`) instead. |
| **13** | The 14 fixture cases stay the shared vector at `…/🧪️tests/🧬️schema-scope-catalog/🧫️fixtures/🔣️.json` with their draft-07 authority in `🛂️schema/`. Reuse them; do not re-author. |
| **14** | Already landed by W2b (`product-module` pattern `♻️mit-bestand/*/🔨️modules/**`); verified: both `♻️mit-bestand` modules are now catalogued, 0 `module-level-ineligible` under `♻️mit-bestand`. |
| **15** | Already applied on disk by W2b. Verified by running it: `testDemonstratorRuntime PASS` (reads `🧩️runtime/🧬️schema/🔣️.json`, plain `ajv`, pipeline at `$ref: "#/$defs/DemonstratorPipelineContract"`). |
| **16** | Two stale rows removed; `census.counts` and `blockingSummary.exactBlockedSources` recomputed from the surviving rows (candidates 229→227, implementation 209→207, authored 181→179, movedDomainImplementations 113→112, movedTests 27→26, exactDestinationDecisions 228→226). `🔬️schema.test.ts` / `🔬️server-persistence.test.ts` were **not** added: neither exists on disk. |
| **17** | Done. |
| **42 / 60** | The blanket fixture exemption is gone. `schemaScopeOwnerFixture` was replaced by `schemaScopeCollectionPath`, which mirrors the harness's `isFixtureOwnedPath` exactly — ancestor-wise match of the self patterns with the `🔨️modules/<m>` carve-out. A collection is never a scope owner **and** every normative JSON leaf inside a collection's schema module is a `fixture-defines-schema` finding; retired placements inside collections are no longer silenced either. The only escape is the enclosing case's `🔣️.json` declaring the file under `inertSchemaData` (declared as `schemaScopeOwnerLevels.inertSchemaDataKeyword`). Case `fixture-owned-schema` was rewritten to expect the finding, and `inert-schema-data` was added as its counterpart. |
| **43 / 63** | Catalog `exports` is now `{ "<ExportId>": { "file": "<module-relative path>", "facet": "<facet id>" } }`. `file` carries nested facet directories verbatim (`🔺️diff/📝️text/🔣️.json`, `📸️snapshot/💾️binary/🔣️.json`). `facet` is derived from the declaring document's `$id`: the scope-path suffix beyond the module's root scope path plus the facet filename (`artifact`, `snapshot`, `snapshot/text`, `diff`). Byte-order first declaring file wins; a second file in the same scope declaring the same id is `export-id-duplicate`. `dependsOn` is derived from cross-document `$ref`s resolved through the `$id` index (never by scope-id prefix). **R-8: the repository test module is emitted** — `repo.test` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema` (the `🔨️modules/<m>` carve-out is what makes it eligible). |
| **53** | `plugin-extension` level added (`✏️s/🔌️plugins/*/🧩️extensions/*`); `plugin-submodule` added per the updated contract §B (`✏️s/🔌️plugins/*/*`, with `🗿️artifacts` and `🧩️extensions` containers added to `excludedOwnerPathPatterns` so only real submodules match). `x-semio-formats` restricted-support rule implemented (below). `memberNames` pruned: the three deleted `*.schema.json` names, verified absent from disk. |
| **57** | Declared `areas[".🧬semio/🦑️repo/🎫️tickets"] = "exempt"`. `AreaState` is now `"clean" \| "exempt"`, and `validateTaxonomy` requires an exempt area to lie inside a `pathEmojiPolicy.reservedSubtreeDirectoryNames` entry or a declared `pathExclusions` path — i.e. a subtree discovery already skips, not an opt-out anyone can write. **Verified:** `bun test …/🧪️test-platform/🟦️.ts -t "exempt"` → `2 pass / 0 fail`. See the blocking request R-A in §6. |
| **58** | **No override needs restoring.** Checked all 176 pre-drop owners against disk: every one has the directory its override declared (`os.path.isdir(owner + "/" + value)` true for 176/176; 0 owners whose declared directory is absent, 0 owners with no oracle directory). All 174 dropped rows restated the default `🔮️oracle` verbatim, so the removal is behaviour-preserving. The 22 `🧪️oracle` directories on disk belong to owners that never had an override and resolve to `🔮️oracle` by default — `🧪️oracle` is a different concept, not a contribution directory. |
| **61** | Done. |
| **69** | `$id` uniqueness is now enforced over **every** module's documents, not only catalogued ones (`document-id-duplicate`, 15 findings). Each module is compiled against exactly its declared `dependsOn`: a cross-document `$ref` whose target document belongs to no catalogued scope is `dependency-uncataloged`, and one that crosses into a scope the owner's `dependsOn` does not list is `dependency-undeclared` (21 findings, all aggregates whose module lost the catalog row to a scope-id duplicate). `enum: []` is rejected everywhere with the hint (`enum-empty`; 0 today). |
| **71** | `generatorContracts.schema-entity-catalog.outputRoots` now names `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️entity_kinds.g.go`, which is where the file actually is. Catalog regenerated (`repo.client.sqlite` is present). `🔗️graphql` removed from `members-of-members-of-members-of-modules.memberNames` — the directory no longer exists anywhere in the tree, so the entry was stale, not enabling. The launch.json entry is the root-script worker's (§6, R-B). |
| **80** | `mutation-leaf` `pathPatterns` gained `**/🧬️schema/🧬️mutations/*/*`; the grammar's leaf enumeration accepts depth 1 and 2 and keys the scope on the descriptor's `semanticKind`. |

## 3. Generator rules and diagnostic codes

Everything is declaration-driven; no path heuristics, no nearest-parent search, no disk probing.

**Identity.** Scope id comes only from a document `$id` under `schemaExportResolution.idBase`, minus the
trailing facet filename. A module's scope is its root `🔣️.json`'s id. A facet child keeps the root scope
path and varies only the facet filename; one that resolves elsewhere is `module-scope-id-inconsistent`
and never deepens the catalog. A mutation leaf is a separate module and therefore a separate scope.

**References.** A `#`-local pointer is resolved against the document itself: `#/definitions/<helper>` and
`#/$defs/<Export>` are both legal and are never findings (contract row 77); a pointer that addresses no
existing member is `ref-unresolved`. A cross-document `$ref` must start with `idBase`; it may be a bare
target `$id` (aggregate → leaf, row 79) or `<$id>#/$defs/<ExportId>`. Anything else is
`ref-not-catalog-addressable`.

**Format coverage (contract §B).** A scope provides the formats whose leaves exist in its module. For
every provided non-normative format, each export must be declared in that format's leaf — `struct/enum/
type/trait/fn` or `pub use …` (Rust), `interface/type/class/enum/const/function`, `parse<Export>` or
`export { … }` (TypeScript), `type/input/enum/union/scalar/interface` (GraphQL), `message/enum/service`
(protobuf). An export carrying `"x-semio-formats": [...]` is checked only against the formats it names;
a malformed annotation (not a non-empty subset of the declared format ids) is
`export-formats-annotation-invalid`. Missing declarations are `export-format-missing`.

**Codes** (`schema check`, one stable JSON line each):
`catalog-absent` · `catalog-malformed` · `catalog-stale` · `dependency-uncataloged` ·
`dependency-undeclared` · `document-dialect-unexpected` · `document-id-duplicate` · `document-id-missing` ·
`document-id-unaddressable` · `document-not-object` · `document-unparseable` · `enum-empty` ·
`export-format-missing` · `export-formats-annotation-invalid` · `export-id-duplicate` ·
`export-id-invalid` · `fixture-defines-schema` · `module-level-ineligible` ·
`module-scope-id-inconsistent` · `module-scope-id-missing` · `mutation-aggregate-id-grammar` ·
`mutation-aggregate-kinds-redundant` · `mutation-leaf-id-grammar` · `mutation-leaf-schema-absent` ·
`placement-retired-location` · `ref-not-catalog-addressable` · `ref-unresolved` · `scope-id-duplicate`.
Rust cross-check (`schema verify --rust-entries`): `rust-entries-contract-unknown` ·
`rust-entry-format-absent` · `rust-entry-format-unknown` · `rust-entry-missing` · `rust-export-unknown` ·
`rust-scope-unknown` · `rust-scope-unregistered`.

Retired: `ref-not-export-addressed` (it only ever fired on legal module-internal pointers).

## 4. Taxonomy diff summary

```
schemaScopeOwnerLevels.levels.mutation-leaf.pathPatterns  += "**/🧬️schema/🧬️mutations/*/*"
schemaScopeOwnerLevels.levels.plugin-extension            = ["✏️s/🔌️plugins/*/🧩️extensions/*"]
schemaScopeOwnerLevels.levels.plugin-submodule            = ["✏️s/🔌️plugins/*/*"]
schemaScopeOwnerLevels.excludedOwnerPathPatterns          += "**/🗿️artifacts", "**/🧩️extensions"
schemaScopeOwnerLevels.fixtureOwnerReason                 rewritten: a collection is a finding, not a silencer
schemaScopeOwnerLevels.moduleMemberDirName                = "🔨️modules"
schemaScopeOwnerLevels.mutationsFacetDirName              = "🧬️mutations"
schemaScopeOwnerLevels.inertSchemaDataKeyword             = "inertSchemaData"
schemaExportResolution.mutationAggregateFacetFilename     = "mutations.json"
areas[".🧬semio/🦑️repo/🎫️tickets"]                        = "exempt"
semanticDirectoryMemberKinds.members-of-fixtures.memberNames                    124 → 121
semanticDirectoryMemberKinds.members-of-members-of-members-of-modules.memberNames 39 → 38
generatorContracts.schema-entity-catalog.outputRoots[2].path → …/💻️client/⌨️cli/🐹️entity_kinds.g.go
```

Level order matters: `plugin-root` → … → `mutation-leaf` → `plugin-extension` → `plugin-submodule`,
because `schemaScopeOwnerLevel` returns the first matching level and `✏️s/🔌️plugins/*/*` also matches an
extension directory.

`validateTaxonomy(loadCatalogTaxonomy())` → **0 problems** after every edit above.

## 5. Verification (real output)

```
$ bun ./📜️script.ts schema generate
[schema generate] 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json: 3021 scopes, 10380 diagnostics.
$ bun ./📜️script.ts schema docs
[schema docs] 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📓️schema-catalog.md: 3021 scopes.
$ bun ./📜️script.ts schema check --report <ticket>/🗑️generated/schema-check-w2t.jsonl
[schema check] wrote 10390 findings to …/🗑️generated/schema-check-w2t.jsonl
[schema check] modules=3203 scopes=3021 findings=10390
[schema check] catalog-stale=1
[schema check] dependency-undeclared=21
[schema check] document-dialect-unexpected=825
[schema check] document-id-duplicate=15
[schema check] document-id-missing=10
[schema check] document-id-unaddressable=55
[schema check] export-format-missing=6583
[schema check] export-id-duplicate=649
[schema check] export-id-invalid=366
[schema check] fixture-defines-schema=110
[schema check] module-level-ineligible=63
[schema check] module-scope-id-inconsistent=215
[schema check] module-scope-id-missing=50
[schema check] mutation-aggregate-id-grammar=52
[schema check] mutation-aggregate-kinds-redundant=59
[schema check] mutation-leaf-id-grammar=1048
[schema check] placement-retired-location=17
[schema check] ref-not-catalog-addressable=121
[schema check] ref-unresolved=50
[schema check] scope-id-duplicate=80
check exit=1
```

`catalog-stale=1` inside one chained run is concurrency, not non-determinism: peers write `$id`s while the
walk runs (documented in `📓️wp2-mechanism-tooling.md` §6). Determinism is asserted per fixture case
(`render(inventory) === render(inventory')`).

The tree was left with `schema generate` + `schema docs` re-run last (3 021 scopes, 10 313 diagnostics —
67 fewer than the `check` run minutes earlier, because the mutation workers are migrating `$id`s right
now). `schema verify` immediately afterwards still reports both outputs stale for the same reason; it will
only be quiet once the migration waves stop writing. That is the expected state for this wave, not a
generator defect.

```
$ bun test ./…/📚️library/🧪️tests/🔬️index/🟦️.ts -t "schema scope catalog"
 4 pass / 0 fail / 84 expect() calls

$ bun test ./…/📚️library/🧪️tests/🔬️index/🟦️.ts -t "schema"
 24 pass / 5 fail / 390 expect() calls        # the 5 are not this partition's, see §7

$ bun test ./…/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts -t "exempt"
 2 pass / 0 fail

$ bun <ticket>/wp2-schema-cases.ts
[wp2-schema-cases] all 14 fixture cases and 13 identity checks pass.

$ bun -e 'testDemonstratorRuntime(cwd)'
[DEBUG] Demonstrator runtime catalog, full component union and 11-file pure import boundary PASS
testDemonstratorRuntime PASS
```

Catalog row shape, as generated:

```json
"s.trinity.jack": {
  "path": "✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema",
  "level": "artifact-standard-subset", "facetKind": "🧬️data",
  "formats": { "🦀️rust": "🦀️.rs", "🟦️typescript": "🟦️.ts", "🔗️graphql": "🔗️.graphql", "🔣️jsonschema": "🔣️.json", "🛰️protobuf": "🛰️.proto" },
  "exports": {
    "Camera":       { "file": "📸️snapshot/🔣️.json", "facet": "snapshot" },
    "JackArtifact": { "file": "🔣️.json",            "facet": "artifact" },
    "JackDiff":     { "file": "🔺️diff/🔣️.json",     "facet": "diff" }
  },
  "dependsOn": ["s.trinity.jack.mutation.change-data-property", "…", "s.trinity.jack.mutation.rename-node"]
}
```

## 6. Remaining findings per partition owner

Attribution: `mutations` = any path containing `/🧬️mutations/`; then `plugins` = `✏️s/🔌️plugins`,
`hub` = `🌎️hub`, `os` = `…/💻️os`, `repo` = `…/🦑️repo` ∪ `♻️mit-bestand`, `framework` = the rest of `🧰️framework`.

| code | plugins | mutations | os | hub | framework | repo | total |
|---|---|---|---|---|---|---|---|
| `catalog-stale` | 0 | 0 | 0 | 0 | 0 | 1 | 1 |
| `dependency-undeclared` | 0 | 21 | 0 | 0 | 0 | 0 | 21 |
| `document-dialect-unexpected` | 821 | 0 | 4 | 0 | 0 | 0 | 825 |
| `document-id-duplicate` | 15 | 0 | 0 | 0 | 0 | 0 | 15 |
| `document-id-missing` | 9 | 0 | 1 | 0 | 0 | 0 | 10 |
| `document-id-unaddressable` | 53 | 0 | 2 | 0 | 0 | 0 | 55 |
| `export-format-missing` | 6278 | 0 | 240 | 63 | 2 | 0 | 6583 |
| `export-id-duplicate` | 649 | 0 | 0 | 0 | 0 | 0 | 649 |
| `export-id-invalid` | 220 | 43 | 34 | 69 | 0 | 0 | 366 |
| `fixture-defines-schema` | 2 | 43 | 16 | 0 | 0 | 49 | 110 |
| `module-level-ineligible` | 58 | 0 | 4 | 0 | 1 | 0 | 63 |
| `module-scope-id-inconsistent` | 191 | 24 | 0 | 0 | 0 | 0 | 215 |
| `module-scope-id-missing` | 47 | 0 | 3 | 0 | 0 | 0 | 50 |
| `mutation-aggregate-id-grammar` | 0 | 52 | 0 | 0 | 0 | 0 | 52 |
| `mutation-aggregate-kinds-redundant` | 0 | 59 | 0 | 0 | 0 | 0 | 59 |
| `mutation-leaf-id-grammar` | 0 | 1048 | 0 | 0 | 0 | 0 | 1048 |
| `placement-retired-location` | 0 | 0 | 0 | 0 | 0 | 17 | 17 |
| `ref-not-catalog-addressable` | 53 | 1 | 0 | 0 | 67 | 0 | 121 |
| `ref-unresolved` | 20 | 30 | 0 | 0 | 0 | 0 | 50 |
| `scope-id-duplicate` | 78 | 0 | 2 | 0 | 0 | 0 | 80 |
| **total** | **8494** | **1321** | **306** | **132** | **70** | **67** | **10390** |

**`scope-id-duplicate` is 80, not 0, and the cause is a single upstream `$id` bug, not the generator.**
All 40 duplicated ids are surface-state lanes: `🎚️config/🧬️schema/🔣️.json` declares
`…/app/writer/writer/config.json` and `👥️presence/🧬️schema/🔣️.json` declares
`…/app/writer/writer/presence.json`. Both resolve to the scope path `app/writer/writer`, because under the
settled grammar the last `$id` segment is the **facet filename**, not a scope segment. Surface lanes are
three *separate* eligible scopes, so each must own its own scope path:
`…/app/writer/writer/config/schema.json`, `…/presence/schema.json`, `…/transient/schema.json`
(scope ids `app.writer.writer.config` / `.presence` / `.transient`). Owners: `plugins` ×78, `os` ×2
(`app.equation.equation`, `app.procedural.2d/3d`, `app.flow.flow`, `app.gis.gis2d/3d`, `app.vcs.vcs`,
`app.animate.presentation`, `app.shooting.shooting`, `app.sequence.sequence`, `app.fem.2d`, …).
The 21 `dependency-undeclared` rows are a direct consequence: the aggregate lives in the lane that lost the
catalog row to the duplicate, so its `dependsOn` was computed from the other claimant. Both codes go to
zero together when the lane `$id`s are fixed.

`mutation-leaf-id-grammar` fell 2 637 → 1 048 between two runs an hour apart while the mutation workers
migrate; the remaining rows are leaves that still carry the old
`…/<artifact>/<standard>/<subset>/…/mutation/<kind>.json` shape.

**In this partition (67 rows), the honest reading is that they are real and mine — but they are the
consequence of row 42, not one of my rows, and the fix is a rename wave:** 48 `fixture-defines-schema` +
16 `placement-retired-location` rows are the library's own per-case authority schemas at
`🧪️tests/<case>/🧬️schema/🔣️.json` and `⚡️caching/🧫️fixtures/<case>/🧬️schema.json`. The taxonomy already
declares the correct home — `semanticDirectoryKinds.test-fixture-schema-authority` = `🛂️schema`, the
convention `📋️mutation-inventory`, `📡️mutation-reachability`, `🪪️mutation-metadata`,
`🧬️schema-rust-entries` and `🧬️schema-scope-catalog` already use. Fix = rename ~25 `🧬️schema/` case
directories to `🛂️schema/` and ~14 `🧬️schema.json` case files to `🛂️schema.json`, then rewire the readers
in `🧪️tests/🔬️index/🟦️.ts` and `⚡️caching/🧪️tests/📜️script.ts`. Deliberately **not** done in this pass:
two peers are rewriting `🧪️tests/🔬️index/🟦️.ts` right now (§7) and a 40-path rename inside it would
collide. Recommend a dedicated follow-up row.

## 7. Peer state encountered (not this partition's work, evidence attached)

1. **`🧪️tests/🔬️index/🟦️.ts` was syntactically invalid at `HEAD`.** Nine string literals carried the known
   codemod corruption (`…/🟦️subject.ts/"` where `…\"` belongs), so `bun build --no-bundle` failed and the
   whole library suite could not load. A peer had already repaired three sibling lines in the index. I
   repaired the remaining nine (restoring `./<sibling>` + the escaped quote, matching the peer's own
   repairs and the uncorrupted go/yaml/markdown rows in the same fixture). The file parses now.
2. **The suite relocation (`📦️packages/🟦️typescript/🔬️index.test.ts` → `🧪️tests/🔬️index/🟦️.ts`) left 8
   fixture paths behind**, all `join(import.meta.dir, "🧫️fixtures", …)`. Repointed at
   `../../📦️packages/🟦️typescript/🧫️fixtures`; three of them also named flat files
   (`🔣️transaction-protocol.json`) that are directories today (`🤝️transaction-protocol/🔣️.json`).
3. **`🧪️tests/🔬️workspace-contract/🟦️.ts` is a byte-identical 7 871-line duplicate of
   `🧪️tests/🔬️index/🟦️.ts`**, corrupted in a *different* set of lines, and it does not parse. It looks
   like an in-flight copy of the same relocation. Left untouched — whoever owns the move should delete one
   of the two rather than have both discovered.
4. **5 failures remain under `bun test … -t "schema"`, none from this partition:**
   `current JCO destination authority` (reads the deleted
   `💻️os/🧫️fixtures/🧩️jcoprobe/📐️destination.schema.json`, os partition) · two
   `artifact path projection authority` failures (`🖍️draw` schema and the CAD scenario source set, plugins
   partition) · `taxonomy normalization` (the worktree taxonomy's
   `semanticDescendantContracts["draw-editor-command-bundle-v1"]` lost both `configurableEntry` nodes —
   `realizedNodeCount` 20→16 and `pathBudgetReserve.bytes` 78→72 were recomputed with them, so this is a
   deliberate peer edit in flight, not a round-trip loss) · `direct mutation ownership` (root
   `📜️script.ts:15789` reads `entry.modulePath.length` on an entry that has no `modulePath`).

## 8. Cross-partition requests

**R-A — root `📜️script.ts`, blocking, same wave (row 57 fallout).** `policyRepositoryOwnedRoots()`
(`📜️script.ts:23511`) builds its walk roots from `Object.keys(loadTaxonomy().areas)` and filters only the
compose prefixes. Now that `.🧬semio/🦑️repo/🎫️tickets` is a declared area, it becomes a walk root, and
`policyListSemanticVocabularyScanFiles` would scan **20 ticket-scratch `.rs` files** under
`/🧬️mutations/` and `/🎮️commands/` as if they were production sources. Exact change:

```ts
-  const configured = Object.keys(loadTaxonomy().areas ?? {});
+  const configured = Object.entries(loadTaxonomy().areas ?? {}).filter(([, state]) => state === "clean").map(([area]) => area);
```

**R-B — root `📜️script.ts` / `📋️project.json` / `package.json` / `.vscode/launch.json` (row 71, row 59, row 64).**
- launch.json: `🪶️sqlite schema test` → `bun nx run @semio-tech/repo-sqlite:test`, group `3_dev`, directly after `🦑️mcp test`.
- Row 59: the root script must not contain the literal test-domain path; `SchemaScript.testDomainPath()` already reads it from the taxonomy, but the literal `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test` still appears elsewhere in the file.
- Row 64: `SCHEMA_DRAFT07_ORACLE_SPEC` → `🧬️schema/🧪️tests/✅️draft07-oracle/🟦️.ts`, vitest include for `🟦️.ts`, `ajv` + `ajv-formats` in devDependencies.
- No change is needed for the catalog shape: the root script never reads `scope.exports` (verified by grep); only the library and the harness do.

**R-C — plugins + os (`scope-id-duplicate`, `dependency-undeclared`).** Give each surface-state lane its own
scope path, as in §6. 40 ids, 80 rows; both codes reach zero together.

**R-D — harness (`🧪️test/📦️packages/🟦️typescript/🟦️.ts`).** `isFixtureOwnedPath` hard-codes the
`🔨️modules` carve-out segment; the taxonomy now declares it as
`schemaScopeOwnerLevels.moduleMemberDirName`. Read it from there so the carve-out has one declaration.
The library's `schemaScopeCollectionPath` already does.

**R-E — plugins (row 42 fallout, 2 rows) / os (16) / mutations (43).** `fixture-defines-schema` outside this
partition: move the contract into the owner module, or declare the file under `inertSchemaData` in the
enclosing case's `🔣️.json` if it really is a parser input.

**R-F — coordinator.** Row 20 (`📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts:287-288, 331-332`) is
addressed to "W2c tooling" in the ledger but was not in this worker's row list, so it was left alone.
Line 264 already reads `mutationPayloadSchemaRelativePath(loadCatalogTaxonomy())`; the remaining three
`"🧬️.schema.json"` occurrences are *synthetic inputs* to `mutationPayloadSchemaProblems` /
`validateMutationPayloadSchemas`, not assertions that the location must be that filename. If they should
use the taxonomy location, it is a three-literal change in this partition — say so and it is done.

## 9. Open questions

1. **`export-format-missing` = 6 583 is the single largest code and it is a genuine contract question.**
   The dominant pattern (e.g. `app.writer.writer`) is a lane whose JSON Schema restates `$defs` that
   another scope owns, while its `🦀️.rs` merely `use crate::{…}`s them. Private `use` is not a
   declaration and is not counted; `pub use` is. Either those `$defs` become cross-scope `$ref`s to the
   owning scope (which also removes 649 `export-id-duplicate` rows), or the leaves must really declare
   them. This is a per-partition decision, not a tooling one.
2. **`export-id-duplicate` = 649.** The new catalog shape forces one file per export id per scope. Today a
   `🔺️diff/🔣️.json` routinely restates the root's `$defs` verbatim. Byte-order-first wins deterministically
   and the rest are reported; the real fix is `$ref`ing the root instead of copying.
3. **Facet ids for documents with no resolvable `$id`.** They fall back to the module-relative directory
   chain (or `🧬️data` at the module root). Once `document-id-missing` reaches zero the fallback is dead
   code and should be deleted.
4. **`plugin-submodule` breadth.** `✏️s/🔌️plugins/*/*` also admits `🎮️commands` and `🔨️modules` as owners.
   No schema module sits in either today, so nothing changed; narrow it if that is not intended.
5. **`schema check` is still not in `verify gate`** — 10 390 findings stand. WP7's closing step.
