# W1 Report — Vocabulary, Twins, Policies, Gates

Ticket: `26/08/10/STDIO-ARTIFACTS-AND-IO`  
Date: 2026-08-10

## Evidence: `validateTaxonomy()` returns []

Ran `generators/w1-validate-taxonomy.mjs` (absolute import of discovery component):

```
PROBLEMS []
COUNT 0
OK schema/snapshot/text → declared
OK schema/mutations/<slug>/mutation → declared
OK io/import/deserializers/artifacts/<slug> → declared
OK builder → declared
OK not-a-facet → undeclared
VALIDATE_TAXONOMY_OK
```

Evidence JSON: `generators/w1-validate-evidence.json`.

## Tokens used (from `tokens.json`, exact, with U+FE0F)

- `builder`: `🏗️builder`
- `decomposer`: `🪓️decomposer`
- `text`: `📝️text`
- `binary`: `💾️binary`
- `deserializers`: `🧩️deserializers`
- `serializers`: `🧵️serializers`
- `ksy`: `🥋️component.ksy`
- `abnf`: `🔠️component.abnf`
- `spicy`: `🌶️component.spicy`
- `ebnf`: `🔤️component.ebnf`
- `g4`: `🅰️component.g4`
- `stdio_plugin`: `🗄️stdio`
- `grammar`: `📖️component.grammar.semio`
- `protocol`: `📡️component.protocol.semio`

## Files changed

1. `framework/.../library/taxonomy.json` — completeness = schema + engine + io + builder + decomposer; added schemaChildDirs, representationDirs, ioDirectionDirs, ioDirectionChildDirs, textSpecFilenames (8), binarySpecFilenames (6); rekeyed exampleAssetKindPrefixes / artifactSpecFilenames / artifactSchemaSpecFilenames / taxonomyLeafParentDirs; **deleted** mediaFormatDirs, ioFormatChildDirs, snapshotChildDirs, diffChildDirs.
2. `framework/.../library/discovery/component.ts` — Taxonomy interface; level-descriptor `artifactFacetChildLevel` with `*` wildcards; `artifactFacetPathIsDeclared`; `validateTaxonomy()` requires new keys and rejects deleted ones.
3. `framework/.../plugin/.../registry/script.ts` — `validateTaxonomyTree` schema-tree + io-direction shape; soft completeness for builder/decomposer.
4. `framework/.../plugin/component.rs` — `assert_taxonomy_components` new completeness set; soft builder/decomposer; stop requiring root dsl/op/spr/diff/snapshot/mutations leaves.
5. `script.ts` — PolicyRuleArtifactIo replaced by seven stdio rules + `policyStdioArtifactsBreaches`; pack-relocation deleted; `policyTaxonomyDirsBreaches` NestedFacetWalk accepts schemaChildDirs/representationDirs/ioDirection shape (no mediaFormatDirs).
6. `.vscode/launch.seed.jsonc` — replaced `gate artifact-io` with seven gates (stdio-catalog, artifact-builder, artifact-decomposer, schema-representation, io-serializer-matrix, io-terminality, codec-fidelity).

## Policy export smoke

All eight exports are functions; `policyIoTerminalityBreaches` = 0 (owner-table DAG valid).

## Known follow-ups (expected; not W1)

- W2: scaffold `stdio` plugin (catalog policy currently red).
- W3/W4: remove SRAS/IFCCARTOONMESH stubs (codec-fidelity).
- W5/W6: migrate 54 domain artifacts to schema tree + builder/decomposer + io matrix (builder/decomposer/schema/io-matrix policies will report many breaches until then).
- Registry/Rust soft-skip missing builder/decomposer so existing crates are not hard-blocked pre-migration; vocabulary remains strict.
- Out-of-ownership library tests may still mention deleted taxonomy keys.
- **No plugin artifact migration performed in W1.**

## Done criteria

- [x] `validateTaxonomy()` returns `[]`
- [x] Deleted keys absent from taxonomy object; discovery rejects them if reintroduced; `policyTaxonomyDirs` has zero `mediaFormatDirs` references
- [x] New policy functions exported
- [x] Seven launch gates registered
- [x] `w1-report.md` written
