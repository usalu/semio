# S-Path-Projection-Schema

## Outcome

Implemented the strict version-7 semantic path-projection schema and discovery interface in the two assigned production files. The contract projects mutation test bundles from artifact standard/subset schema ownership into artifact test profiles without exposing a reverse profile parser, a global mutation-name union, or any compatibility alias.

MutationCatalog `vectors` is an independent required physical registry. Empty vectors are valid, runtime `kinds` are unrelated, and physical bundle identity is exactly `(mutationId, mutationDirectoryName, scenarioId)`.

## Production changes

### `🔣️taxonomy.json`

- Added contextual `mutation-test-profile` and made `test-case` legal below the projected `mutation-test-subject`.
- Added `semanticProjectedMemberKinds.mutation-test-subject`, sourced from the exact schema-member registry and owned only below a rendered mutation-test profile.
- Added forward-only `semanticPathProjectionProfileRenderers.standard-subset-v1`, rendering `🪆️{standardVersion}-{subsetId}` and retaining `(artifactId, standardVersion, subsetId)` as the collision tuple.
- Added `semanticPathProjectionContracts.artifact-mutation-tests-v1` with the exact source and destination segment grammars and rationale `artifact-mutation-test-projection-v1`.
- Added `semanticDescendantContracts.mutation-scenario-bundle-v1`: 12 required nodes plus exactly one of `🔺️diff/🔣️.json` and `🔺️diff/🚫️.absent`, for 13 realized nodes.
- Added the suffix-derived path reserve: `/📸️snapshot/⬅️before/🔣️.json` is 42 UTF-8 bytes. Workspace `maxPathBytes` remains exactly 240.
- Added `semanticPathProjectionCatalogContracts.mutation-catalog-vectors-v1` and exact catalog-facing IDs in `mutationCatalogProjection`.
- Preserved all generator contracts unchanged.

### `🔍️discovery/🟦️component.ts`

- Added strict public types for projected members, source/destination segments, profile renderers, descendant nodes and alternatives, independent catalog contracts, projections, and catalog-facing IDs.
- Added strict validation for exact record keys; reference integrity; projected-owner acyclicity; literal/capture/copy/render exclusivity; one source and destination reference for each projected kind; exact source/destination grammar; exact 13-node bundle; exactly-one diff alternatives; catalog independence; old alias rejection; and the derived path-budget reserve.
- Extended contextual directory-parent validation to projected parents without treating projected kinds as global structural matches.
- Added `semanticProjectedMemberKindId`, which resolves only with complete artifact, standard, subset, mutation, scenario, and vector registration context. Empty or mismatched physical vectors fail closed.
- Added forward-only `renderSemanticProjectionProfile` and collision-checking `renderSemanticProjectionProfiles`; no reverse split/parser exists.
- Added `semanticDescendantNodeRelativePath` for canonical physical bundle realization.

## Frozen live identifiers

| Registry | Identifier |
| --- | --- |
| Projected member | `mutation-test-subject` |
| Profile renderer | `standard-subset-v1` |
| Path projection | `artifact-mutation-tests-v1` |
| Descendant bundle | `mutation-scenario-bundle-v1` |
| Catalog contract | `mutation-catalog-vectors-v1` |
| Profile directory kind | `mutation-test-profile` |

## Verification evidence

All commands ran from `/Users/ueli/Documents/semio` on 2026-08-26.

1. Strict runtime load and validator:

   `bun -e 'import { loadTaxonomy, validateTaxonomy } from "./…/discovery/🟦️component.ts"; const t=loadTaxonomy(); console.log(validateTaxonomy(t));'`

   Result: `[]`; schema version 7 loaded with `artifact-mutation-tests-v1`.

2. Eight focused negative assertions mutated independent clones and required the precise rejection for catalog bijection, projected-owner cycle, reverse profile renderer, invalid source literal, changed bundle node, incorrect path reserve, removed alias, and `maxPathBytes=241`.

   Result: all eight printed `negative:<case>:ok`.

3. Full-context helper and bundle/diff assertion:

   Result: `{"projected":"mutation-test-subject","profile":"🪆️v1-base","requiredNodes":12,"diffAlternatives":["🔺️diff/🔣️.json","🔺️diff/🚫️.absent"],"pathReserve":42}`. The same lookup with `vectors: []` returned `null` as required.

4. Static JSON assertion:

   `jq -e '<v7, 240 bytes, 12+2 bundle, independent empty-vector assertions>' 🔣️taxonomy.json`

   Result: `true`.

5. Repository library compile target:

   `bun nx run @semio-tech/repo-lib:lint`

   Result: failed on unrelated existing TypeScript configuration/import errors in styling and OS plugin sources (`ImportMeta.env`, `ImportMeta.glob`, and files outside repo-lib `rootDir`). No diagnostic referenced either assigned production file. The stricter Bun module-load/runtime assertions above passed.

## Canonical hashes

- `🔣️taxonomy.json`: `7bd14633f3c3d6e1d17d12c38c7a3f4a26a700ebe869d2a335e621577089bf55`
- `🔍️discovery/🟦️component.ts`: `569fac4505acb5818fb7f2341d5e3dd8ae682e073a79509de9bc24f33177b7eb`

## Boundaries and blockers

- No normalization engine, catalog, test-domain, root script, generator owner, Compose/temp-Compose, AGENTS, or Git state was modified.
- This slice defines and validates the physical-vector uniqueness contract but does not edit MutationCatalog data; catalog/engine consumers must enforce every discovered physical bundle exactly once using the frozen fields.
- A clean repo-lib TypeScript target remains blocked by the unrelated diagnostics recorded above.
