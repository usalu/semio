# 🧪 Wave 1 Report — Taxonomy + Nested Walkers

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Wave W1 owns the vocabulary + the four consumers that must move with it
(except root `📜️script.ts` / `policyTaxonomyDirsBreaches`, which W2 owns).

## Taxonomy keys (`🔣️taxonomy.json`)

### Added
- `schemaFormats` — five entries (`🦀️rust`, `🟦️typescript`, `🔗️graphql`, `🔣️jsonschema`, `🛰️protobuf`), each `{ leafFilename, extension, fieldCasing }` exactly as §3.
- `snapshotChildDirs: ["🧬️schema", "🎒️pack"]`
- `diffChildDirs: ["🧬️schema"]`
- `artifactSchemaSpecFilenames` — three facet paths → `🔣️component.json`
- `taxonomyLeafParentDirs` member `🧬️schema`
- `artifactComponentDirs` / `artifactChildDirs` members `🧬️schema`, `📸️snapshot`

### Changed
- `artifactSpecFilenames`: rekeyed `"🎒️pack"` → `"📸️snapshot/🎒️pack"` (value `📡️component.protocol.semio` unchanged)
- `_comment` / `_engineListComment`: prose now documents the three schema facets and that `🎒️pack` nests under `📸️snapshot`

### Removed
- bare `"🎒️pack"` from `artifactComponentDirs` and `artifactChildDirs`
- bare `"🎒️pack"` key from `artifactSpecFilenames`

## `validateTaxonomy` clauses (discovery)

New `SchemaFacetContract` region (mirrors `MutationFacetContract`):
- `snapshotChildDirs` / `diffChildDirs` present, non-empty, every member emoji-prefixed (U+FE0F)
- `🧬️schema` + `📸️snapshot` required in both completeness and structural sets
- bare `"🎒️pack"` forbidden in `artifactComponentDirs`, `artifactChildDirs`, and as an `artifactSpecFilenames` key
- `schemaFormats` non-empty; every entry has non-empty `leafFilename` / `extension` / `fieldCasing`; casing ∈ `{snake,camel}`; leaf ends with its extension; leaf filenames distinct
- `artifactSchemaSpecFilenames` keys exactly the three §2 facet paths; every value equals `schemaFormats["🔣️jsonschema"].leafFilename`
- `taxonomyLeafParentDirs` must include `🧬️schema`
- `artifactSpecFilenames` nested keys (`parent/child`) validate the parent against `artifactComponentDirs` (`.semio` suffix invariant kept; JSON Schema leaf stays out of that map)

Accessors exported: `schemaFormats()`, `snapshotChildDirs()`, `diffChildDirs()`, `artifactSchemaSpecFilename()` (+ existing `artifactSpecFilename` / `loadTaxonomy`).

## `validateTaxonomyTree` nested walk (plugin registry)

After the flat `artifactComponentDirs` pass:
- `📸️snapshot` is treated as a nested container (dir must exist; no rust/ts leaf required at its root)
- descends into `snapshotChildDirs` (pack requires rust+ts+protocol.semio; schema dir presence checked here, leaves below)
- descends into `diffChildDirs`
- for every schema facet path (`🧬️schema`, `📸️snapshot/🧬️schema`, `🔺️diff/🧬️schema`) requires all five `schemaFormats` leaf filenames
- `taxonomyLeafParents` now includes `snapshotChildDirs` + `diffChildDirs`

## Rust twin (`assert_taxonomy_components`)

No seventh hardcoded facet entry. At test time the function:
1. walks up from `CARGO_MANIFEST_DIR` until it finds repo-root marker `nx.json`
2. parses `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` with the crate's existing `serde_json` dependency
3. drives `artifactComponentDirs`, `snapshotChildDirs`, `diffChildDirs`, `schemaFormats` leaf filenames, `forbiddenExamplePluralDirs`, `exampleAssetKindPrefixes`, `exampleAssetsDirName` / `exampleTestsDirName`, `pluginDirName` / `pluginChildDirs` from that JSON

Stale `EXAMPLE_KINDS` plurals (`🗣️dsls` / …) are gone: plurals must **not** exist; assets/tests dirs must exist (aligned with the registry walker and `forbiddenExamplePluralDirs`).

## Gates

### `bun nx run @semio-tech/plugin-registry:check`
- Exit: **1** (expected)
- Malformed-taxonomy / missing-key / validator-crash errors: **none**
- Artifacts reported missing new schema/snapshot facets: **54** (unique plugin∶artifact pairs)
- Full log: `🧪wave1-registry-check.log`

Verbatim tail:

```
  - 🪵️sourcing: artifact "🗂️curate" is missing 🧬️schema/🔣️component.json
  - 🪵️sourcing: artifact "🗂️curate" is missing 🧬️schema/🛰️component.proto
  - 🪵️sourcing: artifact "🗂️curate" is missing 📸️snapshot/🧬️schema/🦀️component.rs
  - 🪵️sourcing: artifact "🗂️curate" is missing 📸️snapshot/🧬️schema/🟦️component.ts
  - 🪵️sourcing: artifact "🗂️curate" is missing 📸️snapshot/🧬️schema/🔗️component.graphql
  - 🪵️sourcing: artifact "🗂️curate" is missing 📸️snapshot/🧬️schema/🔣️component.json
  - 🪵️sourcing: artifact "🗂️curate" is missing 📸️snapshot/🧬️schema/🛰️component.proto
  - 🪵️sourcing: artifact "🗂️curate" is missing 🔺️diff/🧬️schema/🦀️component.rs
  - 🪵️sourcing: artifact "🗂️curate" is missing 🔺️diff/🧬️schema/🟦️component.ts
  - 🪵️sourcing: artifact "🗂️curate" is missing 🔺️diff/🧬️schema/🔗️component.graphql
  - 🪵️sourcing: artifact "🗂️curate" is missing 🔺️diff/🧬️schema/🔣️component.json
  - 🪵️sourcing: artifact "🗂️curate" is missing 🔺️diff/🧬️schema/🛰️component.proto
  - 🪵️sourcing: 🔌️plugin/🎛️apps/🦀️component.rs is not declared by any #[path] in 📦️glue.rs
  - 🪵️sourcing: 🔌️plugin/🛂️manifest/🦀️component.rs is not declared by any #[path] in 📦️glue.rs
  - 🪵️sourcing: 🔌️plugin/🔧️setup/🦀️component.rs is not declared by any #[path] in 📦️glue.rs
  - 🪵️sourcing: 🔌️plugin/🎟️capabilities/🦀️component.rs is not declared by any #[path] in 📦️glue.rs
  - 🪵️sourcing: 🧩️extensions/🪵️beams/🦀️component.rs is not declared by any #[path] in 📦️glue.rs
  - 🪵️sourcing: 🧩️extensions/🧱️slabs/🦀️component.rs is not declared by any #[path] in 📦️glue.rs
  - 🪵️sourcing: 🧩️extensions/🪟️windows/🦀️component.rs is not declared by any #[path] in 📦️glue.rs
Warning: command "bun ./📜️script.ts check" exited with non-zero status code1371 |  * throws on non-zero exit, signal, or budget exceed (the `[budget]` line is printed
1372 |  * to stderr first so it survives a caller's try/catch, e.g. [[tryRun]]).
1373 |  */
1374 | export function runCmd(cmd: string, args: string[], opts: RunCmdOpts = {}): void {
1375 |   const status = runCmdInternal(cmd, args, opts);
1376 |   if (status !== 0) throw new Error(`${cmd} ${args.join(" ")} exited with status ${status}`);
                                     ^
error: node /Users/ueli/Documents/semio/node_modules/nx/bin/nx.js run @semio-tech/plugin-registry:check exited with status 1
      at runCmd (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:1376:31)
      at run (/Users/ueli/Documents/semio/📜️script.ts:599:5)
      at run (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:815:71)
      at runWorkspaceScriptMain (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:850:16)
      at /Users/ueli/Documents/semio/📜️script.ts:5435:11

Bun v1.3.14 (macOS arm64)
error: script "nx" exited with code 1
```

### `bun ./📜️script.ts test` (`@semio-tech/repo-lib`)
- All `loadTaxonomy` / `validateTaxonomy` tests **pass** (including six new clause tests)
- Suite overall: 133 pass / 16 fail — failures are pre-existing unrelated suites (`dependency-boundary`, `command budgets`, `discoverPackages`, `areaOf` expecting `mixed` while taxonomy already declares plugins `clean`, etc.)
- Full log: `🧪wave1-repo-lib-test.log`

Taxonomy-related lines:

```
(pass) loadTaxonomy > parses 🔣️taxonomy.json into the expected shape [0.12ms]
(pass) loadTaxonomy > keeps the artifact completeness set and the artifact structural set as two separate lists [0.06ms]
(pass) loadTaxonomy > describes the per-example assets/tests shape instead of plural facet dirs [0.07ms]
(pass) loadTaxonomy > forbids both the plural and singular implementations spellings [0.01ms]
(pass) loadTaxonomy > describes every declared lang with a manifest/marker/leaf contract [0.06ms]
(pass) loadTaxonomy > encodes Shape V2 entry location and both valid rust #[path] base conventions [0.05ms]
(pass) loadTaxonomy > declares the area-state enum that replaces LEGACY_LAYOUT_TOLERANT [0.01ms]
(pass) validateTaxonomy > the shipped vocabulary is internally consistent [0.45ms]
(pass) validateTaxonomy > reports an area state outside the declared enum [0.06ms]
(pass) validateTaxonomy > reports a completeness dir missing from the structural set [0.05ms]
(pass) validateTaxonomy > reports mutationChildDirs missing from taxonomyLeafParentDirs [0.04ms]
(pass) validateTaxonomy > reports empty mutationChildDirs [0.14ms]
(pass) validateTaxonomy > rejects plural example component dirs in taxonomyLeafParentDirs [0.15ms]
(pass) validateTaxonomy > rejects missing example slug pattern [0.14ms]
(pass) validateTaxonomy > reports empty snapshotChildDirs [0.13ms]
(pass) validateTaxonomy > reports empty diffChildDirs [0.13ms]
(pass) validateTaxonomy > rejects bare pack in artifactComponentDirs [0.14ms]
(pass) validateTaxonomy > rejects bare pack key in artifactSpecFilenames [0.16ms]
(pass) validateTaxonomy > reports schemaFormats leafFilename/extension mismatch [0.10ms]
(pass) validateTaxonomy > reports artifactSchemaSpecFilenames drift from jsonschema leaf [0.11ms]
```

### Rust `cargo check -p semio-framework-plugin --tests`
- Crate has many pre-existing compile errors unrelated to the taxonomy twin (no diagnostics naming `load_taxonomy_json` / `assert_taxonomy_components` / `schemaFormats`)
- Confirmed `serde_json` was already a dependency — no new crate added

## Left to later waves
- W2: root `📜️script.ts` `policyTaxonomyDirsBreaches` nested walk + schema policy rules
- W3+: handcrafted facet leaves per artifact, pack relocation on disk, glue `#[path]` remounts, framework `🧬️schema` module, Projection→Snapshot renames
- Physical migration of the 54 artifacts' `🎒️pack` → `📸️snapshot/🎒️pack` and creation of the 15 schema leaves each
