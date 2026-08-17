# 🧪 Wave 2 Report — Artifact Schema Policy Scanners

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Wave W2 owns root `📜️script.ts` only:
`policyTaxonomyDirsBreaches` nested walk + region `🔧️PolicyRuleArtifactSchemas` + registration.

## Region `🔧️PolicyRuleArtifactSchemas`

Placed immediately after `🔧️PolicyRuleMutationArtifactEngines`. Contents:

### Extractors (exported; these are the compiler)

| Function | Format | Top-level type | Fields |
| --- | --- | --- | --- |
| `policyExtractRustSchemaFields` | 🦀️rust | `pub struct X` | `pub` fields; `#[state(…)]`; `Option`/`Vec`/`[T;k]`/`BTreeMap`/`HashMap`; snake→camel |
| `policyExtractTypescriptSchemaFields` | 🟦️typescript | `export interface X` | members; `/** @state … */`; `?` / `T[]` / tuple / `Record<string,T>` |
| `policyExtractGraphqlSchemaFields` | 🔗️graphql | `type X` | fields; `@state(class:…)`; trailing `!` required; `[T!]!` list; `[XEntry!]!` map |
| `policyExtractJsonSchemaFields` | 🔣️jsonschema | `title` | `properties`+`required`; `x-semio-state`; array/fixedList/map via `additionalProperties` — **normative** |
| `policyExtractProtobufSchemaFields` | 🛰️protobuf | `message X` | fields; `// @state …`; `optional`/`repeated`/`map<string,T>`; snake→camel |

Leaf filenames and `fieldCasing` come from `taxonomy.schemaFormats` (never hardcoded).

Normalised field shape: `{ name /* camelCase */, optional, cardinality: scalar|list|fixedList|map, scalar, state /* kebab */ }`.

### Scanners → breach ids

| Scanner | `kind` | Breach `id` patterns |
| --- | --- | --- |
| `policyArtifactSchemaFacetCompletenessBreaches` | `artifact-schema/facet-completeness` | `artifact-schema-facet-missing-${facetAbs}`, `artifact-schema-leaf-missing-${leafRel}` |
| (same + normative) | `artifact-schema/normative-leaf` | `artifact-schema-normative-missing-${normativeRel}` |
| `policyArtifactSchemaFieldParityBreaches` | `artifact-schema/field-parity` | `artifact-schema-field-parity-missing|shape|extra-${leafRel}-${name}` |
| `policyArtifactSchemaStateParityBreaches` | `artifact-schema/state-parity` | `artifact-schema-state-parity-missing|shape|extra-${artRel}-${name}` |
| `policyArtifactSchemaDiffCoverageBreaches` | `artifact-schema/diff-coverage` | `artifact-schema-diff-coverage|effect|artifact-entry-${artRel}[-${name}]` |
| `policyArtifactSchemaTypeNameParityBreaches` | `artifact-schema/type-name-parity` | `artifact-schema-prefix-unknown-${artRel}`, `artifact-schema-type-name[-missing]-${leafRel}` |
| `policyArtifactSchemaPackRelocationBreaches` | `artifact-schema/pack-relocation` | `artifact-schema-pack-root-${artRel}` |

Aggregator: `export function policyArtifactSchemaBreaches(repoRoot)`.

§10 prefixes live in `POLICY_ARTIFACT_SCHEMA_PREFIXES` keyed by `stripEmoji(plugin)/stripEmoji(artifact)`.

### Registration

- `export const policy` pushes `policyArtifactSchemaBreaches(repoRoot)` next to `policyHandcraftedSpecP3Breaches`.
- `VerifyScript.runGate()` logs `[verify] artifact-schema facet policies…` and throws on any of those breaches, next to the handcrafted block.

### Nested taxonomy walk (item C)

`policyTaxonomyDirsBreaches` now, when a recognized artifact child is `📸️snapshot` or `🔺️diff`, descends one level and accepts only `taxonomy.snapshotChildDirs` / `taxonomy.diffChildDirs`. Bare `🎒️pack` on the artifact root remains an unknown child (and is also flagged by pack-relocation).

## Nx target / launch confirmation

- Root `📋️project.json` has **no** `policy` target (only `verify` / `verify-gate`, which call `bun ./📜️script.ts verify …`).
- `🟨️nx-plugin.mjs` (`@repo/policy-scripts-file`) synthesizes `breach-<slug>:lint` for every `📜️script.ts` that `export`s `policy`, with `command: bun "<rel>" policy`. Confirmed in source. Current `nx show projects` lists three nested `breach-*_script_ts` projects; the root emoji script is not currently materialized in the graph (pre-existing emoji-glob quirk) — **no new target was added**.
- `.vscode/launch.json`: **0** configurations mentioning verify / policy / breach (verified). No `🧩️launch.seed.jsonc` in repo.

## Policy CLI

Real invocation is `bun ./📜️script.ts policy` → `dispatchPolicyArgv` → `runPolicyExit` → `runPolicyScript`. Stdout is DEBUG-only; breach payload is written to `.🦑️repo/⚡️cache/breaches/compose.json` (entity resolves as technology `compose` for the root script). Counts below are from that cache / direct `policyArtifactSchemaBreaches` calls.

## Before / after

| | Total | Notes |
| --- | --- | --- |
| Before | **1769** | `🧪wave2-policy-before.txt` / `…-ids.txt` |
| After | **1983** | `🧪wave2-policy-after.txt` |
| Delta | **+214 artifact-schema** | 162 facet-completeness (54 artifacts × 3 missing facets) + 52 pack-relocation |
| Churn | 4 `budget/no-budget-null` ids | Same pre-existing null-budget sites in `📜️script.ts`; line numbers shifted because the file grew — not a new logical breach class |

No field-parity / state-parity / diff-coverage / type-name breaches yet: no facet leaves exist on disk for any of the 54 artifacts.

## Extractor coverage

No existing `*.test.ts` covers root `📜️script.ts` policy helpers. Self-check probe: `🧪wave2-extractor-probe.ts` (PASS). Region draft mirror: `🧬️policy-rule-artifact-schemas.region.ts`.

## Verify-gate

`bun nx run workspace:verify-gate` exits non-zero **early** at `@semio-tech/plugin-registry:check` (same unmigrated 54-artifact facet findings as W1), before later gate steps including the new artifact-schema hook. Full log: `🧪wave2-verify-gate.log`.

Verbatim tail:

```
Warning: command "bun ./📜️script.ts check" exited with non-zero status code

 NX   Running target check for project @semio-tech/plugin-registry failed
Failed tasks:
- @semio-tech/plugin-registry:check
…
error: bun nx run @semio-tech/plugin-registry:check exited with status 1
…
NX   Running target verify-gate for project workspace failed
Failed tasks:
- workspace:verify-gate
```

Direct exercise of the new verify hook reports **214** breaches (54 artifacts missing all three facets + 52 root packs).

## Left to later waves

- Handcrafted 15 leaves × 54 artifacts, pack move under `📸️snapshot`, glue remounts (W3+/fan-out).
- Field/state/diff/type-name scanners will start firing once leaves land.
