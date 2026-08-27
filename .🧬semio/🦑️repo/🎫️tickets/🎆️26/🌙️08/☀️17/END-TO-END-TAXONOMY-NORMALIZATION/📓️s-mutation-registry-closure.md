# S-Mutation Registry Closure

## Outcome

The version-7 projection catalog is closed over the complete live corpus. All 144 MutationCatalogs and all 1,555 physical vectors now carry an exact `sourceMutationDirectoryName`; canonical `mutationDirectoryName` remains the forward-only destination identity. The 126 changed rows (125 distinct source mutation names) are registered explicitly, with no compatibility key, default, runtime-kind inference, arbitrary emoji, or wildcard membership.

The physical fixture trees were not moved. The normalization transaction planner owns the atomic source-to-canonical move and reference rewrite. No normalization-engine, Compose/temp-Compose, AGENTS, or Git-state change was made in this lane.

## Frozen contract

```ts
type MutationVector = Readonly<{
  mutationId: string;
  sourceMutationDirectoryName: string;
  mutationDirectoryName: string;
  scenarios: readonly Readonly<{ id: string; directoryName: string }>[];
}>;
```

The catalog contract requires these exact fields:

```json
{
  "mutationIdField": "mutationId",
  "sourceMutationDirectoryNameField": "sourceMutationDirectoryName",
  "mutationDirectoryNameField": "mutationDirectoryName",
  "scenariosField": "scenarios",
  "scenarioIdField": "id",
  "scenarioDirectoryNameField": "directoryName",
  "sourceBundleUniquenessFields": ["mutationId", "sourceMutationDirectoryName", "scenarioId"],
  "canonicalBundleUniquenessFields": ["mutationId", "mutationDirectoryName", "scenarioId"]
}
```

`sourceMutationDirectoryName` is the exact current physical basename. `mutationDirectoryName` is the exact canonical registry member and destination basename. Both must be NFC, single basenames, and render the same kebab-case `mutationId`. Scenario records remain canonical and exact (`directoryName === "🧪️" + id`); the complete corpus has one registered physical scenario per vector, so source association is a strict one-to-one coverage relation rather than a name alias. `semanticProjectionCatalogProblems` rejects unknown keys, missing source identity, unregistered canonical members, duplicate source or canonical tuples, normalized destination collisions, and projected paths over the byte limit.

Mutation vectors remain an independent required physical registry. They are not bijective with runtime `kinds`, and an empty vectors array remains legal.

## Canonical vocabulary

`members-of-schema` contains every canonical mutation identity. The glTF vocabulary composes an explicit operation emoji with an explicit entity emoji:

| Dimension | Registered identities |
| --- | --- |
| Operations | bind `🔗️`, change `✏️`, create `🌱️`, declare `📣️`, delete `🗑️`, move `🚚️`, reorder `🔀️`, reparent `👪️`, require `✅️`, transform `🔄️`, unbind `✂️`, unrequire `🚫️`, withdraw `🔙️` |
| Entities | accessor `📐️`, animation `🎞️`, asset `📦️`, buffer `💾️`, buffer-view `👁️`, camera `🎥️`, default-scene/scene `🎬️`, document `📄️`, extension `🧩️`, image `🖼️`, material `💎️`, mesh `🕸️`, morph-target `🧬️`, node `🔘️`, primitive `🔺️`, sampler `🎛️`, skin `🧥️`, texture `🎨️` |

This closes 120 glTF physical mutation names. The five non-glTF source mappings are:

| Physical source | Canonical destination |
| --- | --- |
| `🔖add-representation-tag` | `➕️🏷️add-representation-tag` |
| `🔖️change-handle-kind-label` | `✏️🏷️change-handle-kind-label` |
| `🔖️rename-block` | `✏️🧱️rename-block` |
| `🔖rename-machine` | `✏️🏭️rename-machine` |
| `🔖️rename-paint-layer` | `✏️🎨️rename-paint-layer` |

`add-representation-tag` occurs in two catalog rows, producing 126 changed rows from 125 distinct source names. `💾️binary` is now an exact member under artifacts.

## Path-budget corrections

The three pre-existing 241-byte projected cases were shortened semantically:

| Previous scenario ID | Canonical scenario ID |
| --- | --- |
| `drops-provided-humidification-to-1-point-25-kg-per-hour` | `provided-humidification-becomes-1-point-25-kg-per-hour` |
| `raises-required-humidification-to-3-point-5-kg-per-hour` | `required-humidification-becomes-3-point-5-kg-per-hour` |
| `removes-the-selected-generation-2-and-falls-back-to-generation-1` | `removes-generation-2-and-selects-generation-1` |

Five glTF scenarios also required semantic shortening after adding their operation/entity prefixes:

| Previous scenario ID | Canonical scenario ID |
| --- | --- |
| `restamps-generator-copyright-and-min-version-together` | `restamps-generator-copyright-and-min-version` |
| `attaches-a-punctual-lights-extension-to-the-document-root` | `attaches-punctual-lights-extension-to-document-root` |
| `switches-the-primitive-from-implicit-triangles-to-triangle-strip` | `switches-primitive-from-triangles-to-triangle-strip` |
| `demotes-the-unlit-requirement-behind-the-transform-requirement` | `moves-unlit-requirement-behind-transform-requirement` |
| `flips-normal-ahead-of-position-inside-the-morph-target` | `orders-normal-before-position-in-morph-target` |

The language-neutral golden records source and canonical identities for the changed examples. Final maximum projected length is exactly 240 bytes including the schema-owned 42-byte descendant reserve; the maximal destination is:

```text
✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1993/🧪️tests/🪆️1-any/🌬️update-through-thickness-inputs/🧪️upgrades-the-subgrade-to-k2-for-a-thicker-plate-at-minus-20c
```

## Production and permanent test changes

- `🔣️taxonomy.json`: strict source/canonical catalog contract, 125 canonical mutation members, and `💾️binary`.
- `🔍️discovery/🟦️component.ts`: strict types plus fail-closed catalog validation, membership, uniqueness, collision, and path-budget checks.
- 144 `✏️s/🔌️plugins/**/🧪️oracle/🔣️.json` files: all 1,555 vectors populated with the required source identity and affected canonical names/scenarios.
- Test-domain JSON schema and TypeScript validator: required source identity with no default, exact-key enforcement, and source-tree coverage.
- Repo-library and test-domain permanent tests: strict negatives, source/canonical bijection, native recursive-walk versus `fast-glob` parity, and exact 13-node bundle behavior.
- Language-neutral golden `🧫️fixtures/🧪️mutation-path-projection/🔣️.json`: exact corpus counts and representative source/destination projections.

## Evidence

TDD red was observed before the discovery API landed:

```text
bun test .../repo-lib/.../🧪️index.test.ts --test-name-pattern 'all physical mutation catalogs close'
SyntaxError: Export named 'semanticProjectionCatalogProblems' not found
```

Strict load:

```text
bun -e 'import { loadTaxonomy, validateTaxonomy } ...'
{"problems":[],"members":1444,"binary":true}
```

Permanent shared tests, including native recursion/`fast-glob` parity and strict negatives:

```text
bun test '.../repo-lib/.../🧪️index.test.ts' --test-name-pattern 'all physical mutation catalogs close|strict catalog helper rejects|language-agnostic mutation projection golden'
3 pass, 0 fail, 16 assertions
```

Test-domain schema/13-node assertions:

```text
bun test '.../repo-test-domain/.../🧪️index.test.ts' --test-name-pattern 'physical vectors are strict|a source or projected 13-node bundle'
2 pass, 0 fail, 7 assertions
```

A read-only full-corpus census returned:

```json
{
  "catalogFiles": 166,
  "catalogs": 144,
  "vectors": 1555,
  "scenarios": 1555,
  "changedRows": 126,
  "changedSources": 125,
  "sourceTupleCount": 1555,
  "canonicalTupleCount": 1555,
  "projectedDestinationCount": 1555,
  "projectedCollisions": 0,
  "exact13": 1555,
  "badBundles": 0,
  "missingRoots": 0,
  "maxProjectedBytesWithReserve": 240,
  "reserve": 42,
  "maxPathBytes": 240
}
```

The focused TypeScript compiler command was also run:

```text
bunx tsc --noEmit -p .../repo-lib/.../tsconfig.json
exit 2: pre-existing cross-root TS6059 errors and missing ImportMeta.env/glob ambient declarations; no diagnostic named a changed mutation-registry file or line.
```

Boundary hashes at the successful strict-load/test point:

```text
ff5721e386bdade4dc0dcaff2942581dc79c5dba82a221c0a5848a35ce8fd126  taxonomy.json
8d1c67d2bc83fa4708871ed4a3d26d934f3b4c551b862bf71bd319e32746a4ce  discovery/component.ts
94a33b769a75f5ccca1c8f65bc7fb7e50980a4900cad8e8e7700d0b27e7fd6ed  mutation-path-projection golden
0754b3aabc9c68668a07ed5237998d4d99f8eb57c3da9b55e2fc72307c76605b  aggregate of the 144 catalog-bearing files (sorted path + NUL + bytes)
```

## Acceptance and atomic-apply requirements

- Every catalog vector must retain the required physical `sourceMutationDirectoryName`; equality with canonical remains explicit on all 1,429 unchanged rows.
- The planner must read source identity only for source occupancy and canonical identity only for destination rendering; it must never derive either from runtime kinds or strip/reapply emoji.
- The planner must apply the registered mutation/scenario directory moves, all 13 descendants, and incoming structured references in one transaction or roll back the whole projection.
- Physical fixture trees remain the pre-transaction source of truth until that transaction runs; manually moving them independently would invalidate the source registry.
- Any new catalog row must close exact schema membership, source and canonical tuple uniqueness, physical 13-node coverage, VS16/case-fold destination uniqueness, and the 240-byte budget before it is admissible.

One broader test-domain discovery path remains outside this lane: the v7 contribution file kind resolves `🧪️oracle/🔣️.json` while some live contributions still occupy `🧪️oracle/🔣️.json`. Direct catalog census, strict discovery validation, permanent focused tests, and physical coverage are green; the separate atomic location migration must close that full-suite loader path without adding an alias.
