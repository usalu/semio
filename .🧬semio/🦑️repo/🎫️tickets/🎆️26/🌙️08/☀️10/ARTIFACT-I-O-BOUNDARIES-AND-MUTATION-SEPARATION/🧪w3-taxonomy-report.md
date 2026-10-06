# 🧪 W3 Taxonomy Report — Artifact Io Facets

Ticket `26/08/10/ARTIFACT-IO-FACETS` · Wave 3 (taxonomy + four twins + expand policy)

## Verdict

WAVE 3 complete. Vocabulary declares `🚪️io`; discovery / registry / Rust / root-policy twins agree on the nested shape; `validateTaxonomy()` is clean; facet-completeness policy fires until W5/W6 land leaves (expected).

## Changes

### 1. `🔣️taxonomy.json`

- `🚪️io` in both `artifactComponentDirs` and `artifactChildDirs`
- `ioFormatChildDirs`: `["📥️import", "📤️export"]`
- `mediaFormatDirs`: 26 entries copied from owner-table `catalog_formats`
- `📥️import` / `📤️export` (and `🚪️io`) in `taxonomyLeafParentDirs`
- `_comment` / `_engineListComment` / `_ioFacetComment` cite ticket `26/08/10/ARTIFACT-IO-FACETS`

### 2. Discovery twin (`🔍️discovery/🟦️component.ts`)

- `Taxonomy` interface: `ioFormatChildDirs`, `mediaFormatDirs`
- `artifactFacetChildDirs`: `🚪️io` → any `mediaFormatDirs` value; each format → `ioFormatChildDirs`
- `validateTaxonomy()` `IoFacetContract` region

### 3. Registry twin (`📔️registry/📜️script.ts`)

- `IO_FACET_DIR` + `TAXONOMY_IO_FORMAT_CHILD_DIRS`
- `validateTaxonomyTree`: for `🚪️io` require dir + rust root leaf only (no TS leaf / no format leaves until W6)
- `taxonomyLeafParents` includes `ioFormatChildDirs`

### 4. Rust twin (`🔌️plugin/🦀️component.rs`)

- `assert_taxonomy_components`: `io_dir` special-case — require `🚪️io/` dir + `🦀️component.rs`; do not walk format import/export leaves

### 5. Root policy (`📜️script.ts`)

- `policyTaxonomyDirsBreaches` NestedFacetWalk allows `🚪️io` → `mediaFormatDirs` → `ioFormatChildDirs`
- `policyArtifactIoBreaches` (exported) = catalog-parity + `artifact-io/facet-completeness`
- `policyAppSchemaBreaches` unchanged (smoke: 0 breaches)

## Verification

| Check | Result |
| --- | --- |
| `validateTaxonomy()` | **0** problems |
| `policyArtifactIoBreaches` | **52** (`artifact-io/facet-completeness` only; catalog-parity 0) |
| `policyAppSchemaBreaches` | **0** |
| mediaFormatDirs ↔ owner-table | **26/26** match |

Facet-completeness non-zero is expected until W5/W6 create `🚪️io/` on every artifact. Do not gate CI on full policy for this wave.

## Out of scope (later waves)

- Per-format import/export leaf completeness (W5 pilots, W6 fan-out)
- SDK traits / OS registry (W4)
- Engine `io::register()` migration (W5/W6)
