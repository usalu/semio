# W1 Composer Report — Stdio Policy Rules and Launch Gates

Agent: Composer (W1 policy slice only; taxonomy/discovery owned by Grok).

## Scope delivered

- Replaced `//#region 🔧️PolicyRuleArtifactIo` in root `📜️script.ts` with seven stdio policy scanners + `policyStdioArtifactsBreaches` aggregator.
- Removed `policyArtifactSchemaPackRelocationBreaches` and its spread from `policyArtifactSchemaBreaches`.
- Removed legacy exports: `policyArtifactIoBreaches`, `policyMediaFormatCatalogBreaches`, `policyArtifactIoFacetCompletenessBreaches`, `policyArtifactIoLeafParityBreaches`, `policyArtifactIoNoEngineIoBreaches`.
- Wired `export const policy` to `policyStdioArtifactsBreaches`.
- Replaced single `⚖️gate🚪️artifact-io` in `.vscode/🧩️launch.seed.jsonc` with seven gates (orders 410–410.6).

## Facet tokens (from `🧪tokens.json`)

| Key | Dir / file |
|-----|------------|
| builder | `🏗️builder` |
| decomposer | `🪓️decomposer` |
| text | `📝️text` |
| binary | `💾️binary` |
| deserializers | `🧩️deserializers` |
| serializers | `🧵️serializers` |
| stdio_plugin | `🗄️stdio` |

## Defensive taxonomy reads

At run time `🔣️taxonomy.json` still lacks `schemaChildDirs`, `representationDirs`, and builder/decomposer in `artifactComponentDirs` (Grok W1 in flight). Policies use:

- `taxonomy.schemaChildDirs ?? ["📸️snapshot", "🔺️diff", "🧬️mutations"]`
- `taxonomy.representationDirs ?? ["📝️text", "💾️binary"]`
- `policyStdioArtifactFacets()` merges `artifactComponentDirs` while filtering legacy root facets (`🗣️dsl`, `🔧️op`, `📡️spr`, `🔺️diff`, `📸️snapshot`) and always requiring `🏗️builder` / `🪓️decomposer`.

Stdio roster, DAG, and IO matrix load from ticket `🧪owner-table.json` (not taxonomy).

## Gate breach counts (2026-08-10, post-implementation)

| Export | Breaches | Notes |
|--------|----------|--------|
| `policyStdioCatalogBreaches` | 1 | `🗄️stdio` plugin not scaffolded yet (W2) |
| `policyArtifactBuilderBreaches` | 54 | one per domain artifact |
| `policyArtifactDecomposerBreaches` | 54 | one per domain artifact |
| `policySchemaRepresentationBreaches` | 162 | new `🧬️schema` tree + text/binary leaves |
| `policyIoSerializerMatrixBreaches` | 1140 | deserializer/serializer matrix rows |
| `policyIoTerminalityBreaches` | 0 | owner-table DAG valid |
| `policyCodecFidelityBreaches` | 2 | `SRAS` + `IFCCARTOONMESH` in `🔺️mesh` (expected until W3/W4) |
| `policyStdioArtifactsBreaches` | 1413 | sum of above |

## Launch gates registered

1. `⚖️gate🗄️stdio-catalog` → `policyStdioCatalogBreaches`
2. `⚖️gate🏗️artifact-builder` → `policyArtifactBuilderBreaches`
3. `⚖️gate🪓️artifact-decomposer` → `policyArtifactDecomposerBreaches`
4. `⚖️gate🧬️schema-representation` → `policySchemaRepresentationBreaches`
5. `⚖️gate🚪️io-serializer-matrix` → `policyIoSerializerMatrixBreaches`
6. `⚖️gate🗄️io-terminality` → `policyIoTerminalityBreaches`
7. `⚖️gate💾codec-fidelity` → `policyCodecFidelityBreaches`

## Files touched

- `📜️script.ts` (policy region + schema aggregator + policy export)
- `.vscode/🧩️launch.seed.jsonc`
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/STDIO-ARTIFACTS-AND-IO/🔧️policy-rule-artifact-io.region.ts` (splice source, kept in ticket)
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/STDIO-ARTIFACTS-AND-IO/🧪w1-composer-report.md` (this file)

## Not touched (Grok / other waves)

- `🔣️taxonomy.json`, `🔍️discovery/🟦️component.ts`, registry twins, `launch.json` (generated from seed).

## Verification

```bash
bun -e 'const m = await import("./📜️script.ts"); console.log(m.policyStdioArtifactsBreaches(process.cwd()).length);'
```

Completed without git modify commands.
