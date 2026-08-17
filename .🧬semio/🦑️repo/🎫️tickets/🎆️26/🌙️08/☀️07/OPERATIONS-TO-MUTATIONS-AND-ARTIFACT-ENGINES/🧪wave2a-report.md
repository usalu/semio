# Wave 2a Report — Taxonomy / Discovery / Registry Mutations Facet

**Ticket:** `26/08/07/OPERATIONS-TO-MUTATIONS-AND-ARTIFACT-ENGINES`  
**Wave:** 2a (vocabulary + validators only)  
**Not edited:** root `📜️script.ts` (Wave 2b), plugins, `AGENTS.md`

## Goal of this wave

Make `🧬️mutations` + `⚙️engine` first-class required artifact facets in the shared taxonomy, and teach discovery / registry / Rust twin to enforce them (including the per-mutation triad under `🧬️mutations/`).

## Files touched

| File | Change |
|------|--------|
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | Completeness + structural lists, `mutationChildDirs`, leaf parents, comment |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts` | `Taxonomy.mutationChildDirs`, `validateTaxonomy` MutationFacetContract |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/📜️script.ts` | CONSTITUTIONAL_SLOTS mutations; `validateTaxonomyTree` MutationTriad + engine |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` | `ARTIFACT_COMPONENTS` += mutations/engine; triad documented as registry-side |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts` | Exact facet expectations + mutationChildDirs negative cases |

## Taxonomy snapshot

```
artifactComponentDirs = ['🧬️mutations', '🔺️diff', '🗣️dsl', '🎒️pack', '🔧️op', '📡️spr', '⚙️engine']
artifactChildDirs     = ['🧬️mutations', '🔺️diff', '🗣️dsl', '🎒️pack', '🔧️op', '📡️spr', '⚙️engine', '📚️examples']
mutationChildDirs     = ['🦠️mutation', '🔺️diff', '↩️inverse']
```

Leaf parents now include: `🧬️mutations`, `🦠️mutation`, `↩️inverse` (plus existing `🔺️diff`).  
Op brand unchanged: `🔧️op` remains in completeness; `🔧️ops` remains in `forbiddenExamplePluralDirs`.

## Registry behaviour

- Completeness loop (from `artifactComponentDirs`) requires rust+ts leaves for `🧬️mutations` and `⚙️engine` (no `.semio` spec — not in `artifactSpecFilenames`).
- After that, walks every `🧬️mutations/<mutation>/` child (skips `packagesDirName`) and requires rust+ts leaves for each of `mutationChildDirs`.
- Explicit finding if `⚙️engine/` dir is absent.
- Legacy `CONSTITUTIONAL_SLOTS` gains `mutations` → `🧬️mutations` (silent while plugin area is `clean`).

## Rust twin

Hard-requires the seven facet `🦀️component.rs` leaves including `🧬️mutations` and `⚙️engine`.  
Full triad walk left to registry/policy (documented on `assert_taxonomy_components`).

## Verification

Standalone import of discovery only (`🧪wave2a-verify.ts`):

```
PASS wave2a taxonomy+validateTaxonomy
{
  "artifactComponentDirs": [
    "🧬️mutations",
    "🔺️diff",
    "🗣️dsl",
    "🎒️pack",
    "🔧️op",
    "📡️spr",
    "⚙️engine"
  ],
  "mutationChildDirs": [
    "🦠️mutation",
    "🔺️diff",
    "↩️inverse"
  ],
  "validateTaxonomy": []
}

---STDERR---

---CODE 0---
```

Full `bun test ./🧪️index.test.ts` could not run: root `📜️script.ts` has an unterminated string ~line 4820 (`PolicyRuleMutationArtifactEngines` region inserted mid-template). That file is Wave 2b-owned; not modified here. See `🧪wave2a-test.txt`.

## Downstream impact

- Migrated plugins will start reporting missing `🧬️mutations` / `⚙️engine` (and incomplete mutation triads) from registry / Rust twin until Waves 3–4 land.
- Wave 2b must repair root `📜️script.ts` and wire policy scanners to the new vocabulary.
