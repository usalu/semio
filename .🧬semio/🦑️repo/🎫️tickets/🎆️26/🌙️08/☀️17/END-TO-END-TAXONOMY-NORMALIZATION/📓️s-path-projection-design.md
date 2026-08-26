# S-PATH-PROJECTION-DESIGN — artifact mutation test projection

## Decision

Adopt one schema-owned projection for **all** committed artifact mutation specification vectors, not only the vectors currently over the path budget:

```text
source
<artifact>/🏅️standards/<🔖️standard>/🪆️subsets/<✳️subset>/🧬️schema/🧬️mutations/
<mutation>/🧪️tests/<scenario>/<physical fixture>

destination
<artifact>/🧪️tests/<🪆️standard-subset>/<mutation>/<🧪️scenario>/<physical fixture>
```

For the dominant profile this is exactly:

```text
<artifact>/🧪️tests/🪆️1-any/<mutation>/<🧪️scenario>/<physical fixture>
```

The canonical inventory proves that the complete population is **20,215 nodes / 1,555 scenarios / 53 exact profiles**. Of those, **14,367 nodes in 1,524 scenarios** currently exceed 240 bytes. Moving only those 1,524 would create two physical ownership rules for the same contract and make a formerly short scenario move when a later semantic edit lengthened it. Moving all 1,555 is deterministic and leaves one invariant.

The projection produces **20,215 distinct destinations**, no byte/NFC/case/VS16-fold collision, no destination occupied by an outside inventory entry, and no pre-existing `<artifact>/🧪️tests/🪆️<profile>` root. It leaves five over-budget records, all beneath three DIN 16798 scenarios. The three exact scenario changes in [DIN 16798 residuals](#din-16798-residuals) resolve them without changing `collisionPolicy.maxPathBytes = 240`.

This packet is design-only. It did not modify production, schema, engine, tests, manifests, Git state, Compose, or the removed/relocated opaque tree.

## Canonical evidence

Source artifact:

```text
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/
📊️taxonomy-inventory/🔣️.json
```

The artifact has taxonomy schema v7 and inventory digest `68166b9fdcf70c4ad85d3a521803c4f0e460c5a27a28c0c0cf24f73521878934`.

### Exact fixture contract

Every one of the 1,555 scenario groups has exactly 13 nodes: seven directories and six files. There are no other descendant shapes.

| Relative normalized node beneath `<🧪️scenario>` | Count | Contract |
|---|---:|---|
| scenario directory | 1,555 | `test-case` |
| `🦀️.rs` | 1,555 | `rust-source` |
| `🦠️mutation/` | 1,555 | `mutation` directory |
| `🦠️mutation/🔣️.json` | 1,555 | `json` |
| `📸️snapshot/` | 1,555 | `snapshot` directory |
| `📸️snapshot/⬅️before/` | 1,555 | `comparison-before` |
| `📸️snapshot/⬅️before/🔣️.json` | 1,555 | `json` |
| `📸️snapshot/➡️after/` | 1,555 | `comparison-after` |
| `📸️snapshot/➡️after/🔣️.json` | 1,555 | `json` |
| `🔺️diff/` | 1,555 | `diff` directory |
| `🔺️diff/🔣️.json` | 1,484 | `json`, exclusive alternative A |
| `🔺️diff/🚫️.absent` | 71 | `absence-marker`, exclusive alternative B |
| `🎯️outcome/` | 1,555 | `comparison-outcome` |
| `🎯️outcome/🔣️.json` | 1,555 | `json` |

The corresponding current source leaves are `🦀️component.rs`, four or five `🔣️component.json` files, and either `🔣️component.json` or `🚫️component.absent` under `🔺️diff`. Physical-leaf normalization changes them to the kind-only leaves above. A group with both diff alternatives, neither alternative, another child, a missing child, a symlink, or a non-file leaf is unresolved and must not project.

The move population is therefore **9,330 file moves** (`1,555 × 6`). Directory destinations are created by the transaction and the 10,885 empty source directories are pruned only after verification.

### Exact profiles

The profile directory is a forward rendering of the captured standard and subset. It must never be reverse-parsed by splitting on `-`: both source slugs may contain hyphens. Verification uses the captured tuple or the owner manifest. The current 53 renderings are collision-free:

| Profile | Standard | Subset | Scenarios | Currently over-budget scenarios | Entries | Long entries |
|---|---|---|---:|---:|---:|---:|
| `🪆️1-any` | `🔖️1` | `✳️any` | 1,278 | 1,247 | 16,614 | 11,346 |
| `🪆️1.0-any` | `🔖️1.0` | `✳️any` | 4 | 4 | 52 | 46 |
| `🪆️1.1-any` | `🔖️1.1` | `✳️any` | 1 | 1 | 13 | 7 |
| `🪆️1.2-any` | `🔖️1.2` | `✳️any` | 1 | 1 | 13 | 10 |
| `🪆️1.4-any` | `🔖️1.4` | `✳️any` | 1 | 1 | 13 | 11 |
| `🪆️1.7-any` | `🔖️1.7` | `✳️any` | 1 | 1 | 13 | 12 |
| `🪆️2.0-any` | `🔖️2.0` | `✳️any` | 121 | 121 | 1,573 | 1,336 |
| `🪆️2.1-any` | `🔖️2.1` | `✳️any` | 1 | 1 | 13 | 12 |
| `🪆️2x3-any` | `🔖️2x3` | `✳️any` | 1 | 1 | 13 | 7 |
| `🪆️3.0-any` | `🔖️3.0` | `✳️any` | 1 | 1 | 13 | 12 |
| `🪆️4-any` | `🔖️4` | `✳️any` | 1 | 1 | 13 | 2 |
| `🪆️5-any` | `🔖️5` | `✳️any` | 1 | 1 | 13 | 12 |
| `🪆️6.0-any` | `🔖️6.0` | `✳️any` | 1 | 1 | 13 | 12 |
| `🪆️87a-any` | `🔖️87a` | `✳️any` | 1 | 1 | 13 | 11 |
| `🪆️89a-any` | `🔖️89a` | `✳️any` | 1 | 1 | 13 | 12 |
| `🪆️ac1018-any` | `🔖️ac1018` | `✳️any` | 1 | 1 | 13 | 7 |
| `🪆️ac1024-any` | `🔖️ac1024` | `✳️any` | 1 | 1 | 13 | 12 |
| `🪆️ap214-any` | `🔖️ap214` | `✳️any` | 1 | 1 | 13 | 7 |
| `🪆️ascii-any` | `🔖️ascii` | `✳️any` | 1 | 1 | 13 | 13 |
| `🪆️commonmark-any` | `🔖️commonmark` | `✳️any` | 1 | 1 | 13 | 10 |
| `🪆️ecma-376-any` | `🔖️ecma-376` | `✳️any` | 3 | 3 | 39 | 36 |
| `🪆️energyplus-any` | `🔖️energyplus` | `✳️any` | 1 | 1 | 13 | 13 |
| `🪆️iana-any` | `🔖️iana` | `✳️any` | 1 | 1 | 13 | 10 |
| `🪆️isobmff-any` | `🔖️isobmff` | `✳️any` | 1 | 1 | 13 | 12 |
| `🪆️jfif-1.01-any` | `🔖️jfif-1.01` | `✳️any` | 1 | 1 | 13 | 12 |
| `🪆️mpeg1-layer3-any` | `🔖️mpeg1-layer3` | `✳️any` | 1 | 1 | 13 | 7 |
| `🪆️r12-any` | `🔖️r12` | `✳️any` | 1 | 1 | 13 | 7 |
| `🪆️raw-any` | `🔖️raw` | `✳️any` | 1 | 1 | 13 | 7 |
| `🪆️rfc1950-any` | `🔖️rfc1950` | `✳️any` | 1 | 1 | 13 | 12 |
| `🪆️rfc4180-any` | `🔖️rfc4180` | `✳️any` | 1 | 1 | 13 | 9 |
| `🪆️rfc8259-any` | `🔖️rfc8259` | `✳️any` | 1 | 1 | 13 | 12 |
| `🪆️riff-pcm-any` | `🔖️riff-pcm` | `✳️any` | 1 | 1 | 13 | 13 |
| `🪆️utf-8-any` | `🔖️utf-8` | `✳️any` | 1 | 1 | 13 | 9 |
| `🪆️v1-animation` | `🔖️v1` | `✳️animation` | 1 | 1 | 13 | 12 |
| `🪆️v1-any` | `🔖️v1` | `✳️any` | 1 | 1 | 13 | 12 |
| `🪆️v1-audio` | `🔖️v1` | `✳️audio` | 1 | 1 | 13 | 12 |
| `🪆️v1-brep` | `🔖️v1` | `✳️brep` | 13 | 13 | 169 | 146 |
| `🪆️v1-cad` | `🔖️v1` | `✳️cad` | 1 | 1 | 13 | 10 |
| `🪆️v1-document` | `🔖️v1` | `✳️document` | 1 | 1 | 13 | 12 |
| `🪆️v1-drawing` | `🔖️v1` | `✳️drawing` | 17 | 17 | 221 | 192 |
| `🪆️v1-flow` | `🔖️v1` | `✳️flow` | 1 | 1 | 13 | 12 |
| `🪆️v1-graph` | `🔖️v1` | `✳️graph` | 11 | 11 | 143 | 132 |
| `🪆️v1-image` | `🔖️v1` | `✳️image` | 12 | 12 | 156 | 120 |
| `🪆️v1-kit` | `🔖️v1` | `✳️kit` | 15 | 15 | 195 | 163 |
| `🪆️v1-mesh` | `🔖️v1` | `✳️mesh` | 17 | 17 | 221 | 193 |
| `🪆️v1-model` | `🔖️v1` | `✳️model` | 1 | 1 | 13 | 12 |
| `🪆️v1-object` | `🔖️v1` | `✳️object` | 9 | 9 | 117 | 102 |
| `🪆️v1-presentation` | `🔖️v1` | `✳️presentation` | 1 | 1 | 13 | 13 |
| `🪆️v1-table` | `🔖️v1` | `✳️table` | 8 | 8 | 104 | 82 |
| `🪆️v1-text` | `🔖️v1` | `✳️text` | 7 | 7 | 91 | 51 |
| `🪆️v1-value` | `🔖️v1` | `✳️value` | 1 | 1 | 13 | 12 |
| `🪆️v1-video` | `🔖️v1` | `✳️video` | 1 | 1 | 13 | 13 |
| `🪆️v3-any` | `🔖️v3` | `✳️any` | 1 | 1 | 13 | 10 |

## Exact matching and reversible identity

### Source matcher

A directory is a projection source only when all conditions hold:

1. It is below an exact registered artifact member of `🗿️artifacts`.
2. Its ancestor chain, in order, is exactly `🏅️standards/<🔖️standard>/🪆️subsets/<✳️subset>/🧬️schema/🧬️mutations/<mutation>/🧪️tests/<scenario>`.
3. `standard` matches the registered `standard` kind and `subset` the registered `subset` kind; neither is inferred from the destination profile text.
4. `<mutation>` is an exact NFC/VS16 member that exists under the captured subset's `🧬️schema/🧬️mutations`; the matcher does not accept an arbitrary emoji-leading directory or the global union of schema members.
5. `<scenario>` matches `testCaseSlugPattern`, is the scenario registered for that mutation vector, and its descendant set equals `mutation-scenario-bundle-v1` above.
6. Every node is a Git-index inventory node, not an unindexed filesystem fallback; symlinks fail the bundle contract.
7. The path is outside every opaque exclusion. An absent opaque prefix remains lexically excluded and is never traversed.

The match is rooted at the artifact member, not found by a substring search. Thus framework mutation fixtures such as OS configuration tests, aggregate artifact cases already under `<artifact>/🧪️tests/mutate-*`, examples, CAD model definitions, ticket evidence, and prose mentioning the path are not move sources.

### Destination renderer

Given the captured identity, render:

```text
profileDirectory = "🪆️" + standardSlug + "-" + subsetSlug
scenarioDirectory = "🧪️" + canonicalScenarioSlug

destinationScenarioRoot =
  artifactRoot + "/🧪️tests/" + profileDirectory + "/" +
  mutationDirectoryName + "/" + scenarioDirectory
```

All segments are NFC and carry canonical VS16. The mutation directory is copied byte-for-byte from its registered source identity. The physical suffix is rendered from the fixed bundle contract, so source generic leaves become physical kind-only leaves. Before accepting a plan, group by byte, NFC, case-fold, VS16-fold, and same-kind keys, and compare against every non-source inventory destination.

### Identity tuple

The stable semantic identity is:

```text
(artifactId, standardVersion, subsetId, mutationId, scenarioId)
```

Its concrete record is:

```ts
type MutationScenarioIdentity = Readonly<{
  artifactId: string;
  artifactRoot: string;
  standardVersion: string;
  standardDirectoryName: string;
  subsetId: string;
  subsetDirectoryName: string;
  mutationId: string;
  mutationDirectoryName: string;
  scenarioId: string;
}>;
```

`artifactId`, `mutationId`, and `scenarioId` are semantic IDs from their owner registries/manifests; directory names are their exact canonical renderings. A transaction can reverse the projection from its captured tuple and `TaxonomyMove.sourcePath`/`destinationPath`. It must not reverse by splitting `🪆️mpeg1-layer3-any` or another profile string.

Current evidence has **1,555 mutation identities and exactly one scenario per identity**; the maximum is one. The contract must nevertheless model `vectors` as an array so a future mutation can own multiple separately registered scenarios without changing the path grammar.

## Schema-first contract

### Taxonomy v7 additions

Add the following concepts to `🔣️taxonomy.json`, its `Taxonomy`/`TaxonomyV7` types, strict loader, and validator:

```ts
semanticDirectoryKinds["mutation-test-profile"] = {
  emoji: "🪆️",
  slugPattern: "^[a-z0-9][a-z0-9.\\-]*-[a-z0-9][a-z0-9.\\-]*$",
  allowEmojiOnly: false,
  parentKindIds: ["tests"]
};

semanticProjectedMemberKinds["mutation-test-subject"] = {
  ownerKindIds: ["mutation-test-profile"],
  projectionContractId: "artifact-mutation-tests-v1",
  sourceMemberKindId: "members-of-schema",
  identityField: "mutationDirectoryName"
};

semanticDirectoryKinds["test-case"].parentKindIds = [
  "tests",
  "mutation-test-subject"
];
```

`semanticProjectedMemberKinds` is deliberately not another broad regex and not a copy of the 1,319-member `members-of-schema` union. It resolves the exact mutation member through the captured artifact/standard/subset source owner. Strict validation must allow projected member kind IDs in `parentKindIds`, require their referenced projection/source kind to exist, and reject cycles.

Add:

```ts
semanticPathProjectionContracts["artifact-mutation-tests-v1"] = {
  sourceOwnerKindId: "members-of-artifacts",
  sourceSegments: [
    { kindId: "standards", literal: "🏅️standards" },
    { kindId: "standard", capture: "standardVersion" },
    { kindId: "subsets", literal: "🪆️subsets" },
    { kindId: "subset", capture: "subsetId" },
    { kindId: "schema", literal: "🧬️schema" },
    { kindId: "schema", literal: "🧬️mutations" },
    { projectedMemberKindId: "mutation-test-subject", capture: "mutationId" },
    { kindId: "tests", literal: "🧪️tests" },
    { kindId: "test-case", capture: "scenarioId" }
  ],
  profileRenderer: "standard-subset-v1",
  destinationOwnerKindId: "members-of-artifacts",
  destinationSegments: [
    { kindId: "tests", literal: "🧪️tests" },
    { kindId: "mutation-test-profile", render: "profile" },
    { projectedMemberKindId: "mutation-test-subject", copy: "mutationId" },
    { kindId: "test-case", copy: "scenarioId" }
  ],
  descendantContractId: "mutation-scenario-bundle-v1",
  rationaleRule: "artifact-mutation-test-projection-v1"
};
```

The JSON shape should use records/arrays rather than storing the TypeScript-like shorthand above. `standard-subset-v1` renders forward from two captured values and performs a collision check over tuples; it does not expose a string-splitting reverse operation.

`mutation-scenario-bundle-v1` encodes the exact 13-node contract, including the exclusive diff alternatives and the descendant reserve. The reserve is computed from the longest allowed suffix rather than retained as an unexplained `42` constant.

### Mutation catalog additions

The owner manifest is the correct source of the mutation/scenario association. Extend `MutationCatalog` in the test-domain schema with required profile and vector data:

```ts
type MutationVector = Readonly<{
  kind: string;
  mutationDirectoryName: string;
  scenarios: readonly Readonly<{ id: string; directoryName: string }>[];
}>;

type MutationCatalog = Readonly<{
  id: string;
  capability: string;
  standardDirectoryName: string;
  subsetDirectoryName: string;
  kinds: readonly string[];
  vectors: readonly MutationVector[];
  deferredKinds?: readonly string[];
}>;
```

Validation rules:

- the contribution manifest must be owned by the same subset named by `standardDirectoryName` and `subsetDirectoryName`;
- `vectors[].kind` is unique and is a bijection with `kinds`;
- every `mutationDirectoryName` is exact NFC/VS16, exists beneath that subset's `🧬️schema/🧬️mutations`, and exposes the same semantic mutation kind;
- every scenario ID is unique within its mutation, matches `testCaseSlugPattern`, and `directoryName` equals `🧪️${id}`;
- every projected scenario has one vector and every vector has one exact bundle;
- no source alias or compatibility name is retained after migration.

This consolidates facts currently duplicated in Rust `#[path]` registries, Python `VECTORS`, Gherkin examples, and filesystem names. Generators may consume the vector registry; they may not become its owner.

## DIN 16798 residuals

The projection has five residual **path records**, not five distinct scenario directories. They are the before/after JSON leaves beneath three scenarios. Remove the article `the` from each canonical scenario ID:

| Mutation | Current scenario | Canonical scenario | Saving |
|---|---|---|---:|
| `🌾change-humidification-required-kg-h` | `raises-the-required-humidification-to-3-point-5-kg-per-hour` | `raises-required-humidification-to-3-point-5-kg-per-hour` | 4 bytes |
| `🍀change-humidification-provided-kg-h` | `drops-the-provided-humidification-to-1-point-25-kg-per-hour` | `drops-provided-humidification-to-1-point-25-kg-per-hour` | 4 bytes |
| `🌴change-infiltration-allowance-m3-h` | `raises-the-infiltration-allowance-to-52-point-5-m3-per-hour` | `raises-infiltration-allowance-to-52-point-5-m3-per-hour` | 4 bytes |

Exact affected destination records:

| Record | Before | After |
|---|---:|---:|
| required humidification `📸️snapshot/⬅️before/🔣️.json` | 242 | 238 |
| required humidification `📸️snapshot/➡️after/🔣️.json` | 241 | 237 |
| provided humidification `📸️snapshot/⬅️before/🔣️.json` | 242 | 238 |
| provided humidification `📸️snapshot/➡️after/🔣️.json` | 241 | 237 |
| infiltration allowance `📸️snapshot/⬅️before/🔣️.json` | 241 | 237 |

Record the three new values as the canonical `vectors[].scenarios[].id` values before planning. Because each current mutation identity has exactly one scenario, the source vector maps unambiguously to the newly registered canonical ID. Update all Rust module identifiers/assertion messages, Python `VECTORS`, Gherkin rows, and textual references atomically. Do not retain an old-to-new alias in production.

## Production owners and consumers

### Core schema, discovery, and planner

| File/region | Required change |
|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` around `semanticDirectoryKinds` (current lines 226–313), `semanticDirectoryMemberKinds` (314 onward), and test vocabulary (around 751–883) | Add the profile kind, projected-member kind, projection contract, fixture contract, and rationale comments. Keep `maxPathBytes = 240`. |
| `…/📚️library/🔍️discovery/🟦️component.ts` taxonomy interfaces around 226–245, `semanticDirectoryKindId` around 500–522, and strict validation around 862–884 | Type/validate the new contracts and resolve projected members with the full artifact/profile context. |
| `…/📚️library/🧹️normalization/🟦️.ts` taxonomy interfaces/loader around 260–590 | Load the new shapes strictly; no default/fallback contract. |
| same file, `matchDirectoryKind`/`canonicalDirectory` around 690–755 | At a matched scenario root, replace the deep canonical parent with the artifact projection, then let descendants inherit that projected canonical parent. Normal directory normalization remains unchanged. |
| same file, inventory API around 1,706 onward | Build the five-field identity, validate the exact bundle/catalog, and report `projection-source-unregistered`, `projection-bundle-invalid`, or `projection-profile-collision` rather than guessing. |
| same file, `planTaxonomy` around 1,932–1,985 | Produce file moves with rationale `artifact-mutation-test-projection-v1`; resolved source path-length errors disappear because `normalizedPath` is projected before the path-policy check. Preserve the frozen plan shape. |
| same file, reference token/graph code around 1,220–1,671 | Add the structured forms below. Current Rust `#[path]` and `include_str!` support is reusable. |
| same file, apply/rollback around 2,243–2,445 | Reuse staging/journal semantics, but add post-verify projection bundle/identity checks before pruning. Sort regenerations and validate output hashes. |

### Test domain and virtual Nx discovery

| File/region | Required change |
|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️component.json` `$defs.MutationCatalog` around 774–814 | Require profile identity and vector registry fields. No legacy optional form. |
| `…/🧪️test/📦️packages/🟦️typescript/📦️index.ts` `MutationCatalog`/contribution loading around 620–710 and completeness around 1,098–1,135 | Validate vector/catalog bijection and expose the registered vector lookup used by projection validation. |
| same file, fixture URI parser/resolver around 733–790 | URI resolution already works against the artifact owner; add projected URI tests and structured Gherkin tokenization, not a second resolver. |
| `…/🧪️test/🔌️nx-plugin.mjs` `testCaseProjects` around 100–165 and `discoverCaseDirs` around 170–196 | Keep executable case discovery at immediate `🧪️tests/<case>/<feature>` only. Add a regression proving `🪆️<profile>/<mutation>/<scenario>` is a fixture projection and never becomes an Nx project. Inputs already include the artifact owner tree. |
| `…/🧪️test/📜️script.ts` host routing/generation | No path literal was found. Confirm generated hosts consume `ResolvedFixture.path`; do not add profile-specific routing. |

### Actual tracked reference owners

A direct old-hierarchy token census found **196 files**:

| Owner form | Files |
|---|---:|
| plugin Rust package glue | 30 |
| artifact aggregate Rust adapters under `🧪️tests/mutate-*` | 48 |
| artifact Gherkin features under `🧪️tests/mutate-*` | 53 |
| artifact Python adapters under `🧪️tests/mutate-*` | 9 |
| subset `🧪️oracle/🔣️component.json` manifests | 23 |
| subset snapshot Rust owners | 30 |
| case-local JSON fixtures containing literal paths | 3 |

The 196 files contain 614 Rust `#[path]` occurrences, 2,439 `include_str!` occurrences, 210 `asset://` occurrences, nine JSON path strings, 74 prose/documentation path occurrences, and one other direct occurrence. Separately, **78** mutation-root `🦀️component.rs` registries contain **828** local `#[path = "<mutation>/🧪️tests/<scenario>/🦀️component.rs"]` occurrences; those do not contain the full deep prefix and therefore are not part of the 196-file census.

The inventory graph over projected nodes records 7,730 internal outgoing edges, 53 cross-boundary outgoing edges, and 13,377 cross-boundary incoming memberships from 140 unique outside origin files. Internal sibling `include_str!` paths retain their relative topology but still need physical basename edits. Every external incoming edge must be accounted for by a structured edit or an owned regeneration.

Representative exact owners:

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — local test-module `#[path]` registry;
- `…/📗️din16798/🧪️tests/mutate-din16798-1/🦀️component.rs` — aggregate `include_str!` table;
- `…/📗️din16798/🧪️tests/mutate-din16798-1/component.feature` — templated `asset://` vectors;
- `…/📗️din16798/🧪️tests/mutate-din16798-1/🐍️component.py` — `VECTORS` scenario registry;
- `…/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json` — catalog and oracle rationale;
- `✏️s/🔌️plugins/📕️norm/🧪️oracle/📦️packages/🐍️python/semio_norm_vocabulary.py` lines 85–87 and 775–780 — shared `VECTOR_ROOT` plus constructed vector stem;
- every matching plugin `📦️packages/🦀️rust/📦️glue.rs` — crate-level test reachability.

No permanent mutation-vector generator was found, and no `generatorContracts` output root owns these checked-in fixtures/registries. The subset manifests describe catalogs as generated from semantic descriptors, but no registered Bun/Nx command materializes the catalog, Rust module map, Python vector table, or feature rows. Therefore this migration must treat them as authoritative tracked inputs and edit them with preimage guards. If a generator is introduced, register it schema-first with exact inputs/outputs/check target before putting a `plan.regenerations` entry in the transaction.

The virtual Nx test project generator is a runtime consumer, not a checked-in output owner. The plugin-registry generator consumes `✏️s/🔌️plugins/**`; its check should run after the move, but the projection must not claim it regenerates mutation vectors unless its declared output actually changes.

## Structured reference requirements

### Already reusable

The normalization Rust adapter already parses and rewrites `#[path = "…"]`, `include!`, `include_str!`, and `include_bytes!` with byte offsets and preimage hashes. Relative paths must be recomputed from the reference file's **final** destination, which the current planner already does. JSON exact values and physical leaf renames are also reusable when the string is a resolvable whole path.

### Gherkin `asset://` templates

`.feature` is not currently admitted by `textualPath`, and `FIXTURE_URI_RE` lives in the test domain rather than the normalization graph. Add a `gherkin` reference adapter that recognizes this exact grammar in descriptions, steps, doc strings, and tables:

```text
asset://🏅️standards/🔖️<standard>/🪆️subsets/✳️<subset>/🧬️schema/🧬️mutations/
<mutation>/🧪️tests/<scenario>/<suffix>
```

Rewrite to:

```text
asset://🧪️tests/🪆️<standard>-<subset>/<mutation>/🧪️<scenario>/<suffix>
```

Angle-bracket placeholders are captures, not glob wildcards. Preserve them byte-for-byte. Emit `adapter: "gherkin"`, `structuredLocation: "gherkin:<line>:<column>@<rawOffset>"`, exact `oldValue`/`newValue`, and the containing file's `preimageHash`. Reject an asset template whose profile or mutation cannot be resolved to one catalog; do not silently ignore it as an external URI.

### Python `VECTOR_ROOT` and constructed stems

The shared norm oracle currently has:

```python
VECTOR_ROOT = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
stem = "%s/%s/🧪️tests/%s" % (subset.vector_root, directory, fixture)
```

Its final form is:

```python
VECTOR_ROOT = "asset://🧪️tests/🪆️1-any"
stem = "%s/%s/🧪️%s" % (subset.vector_root, directory, fixture)
```

Add `python-path-template` tokens for path-like constant assignments and `%`/f-string construction. Required locations are `python-string:VECTOR_ROOT:<line>:<column>@<rawOffset>` and `python-format:stem:<line>:<column>@<rawOffset>`. Resolve the composed value against each owning artifact vector; a partial rewrite of only `VECTOR_ROOT` would leave the stale `/🧪️tests/` segment and must be rejected. Each token carries the file preimage hash.

### Prose path forms

The census has 74 old-hierarchy occurrences in Rust comments, Gherkin descriptions, and JSON rationale strings. Add a narrowly scoped `prose-path` token recognized only when the entire old structural grammar occurs inside:

- Rust `//!`, `///`, or `//` comments;
- a Gherkin description/doc string;
- a decoded JSON/JSONC string value.

Emit a subspan location such as `rust-comment:<line>:<column>@<rawOffset>`, `gherkin-description:…`, or `/@value[n]/prose@<rawOffset>`. Preserve Markdown backticks and surrounding prose. For JSON escapes, maintain a decoded-to-raw offset map; if it cannot be proven, emit `reference-syntax-unsupported` rather than applying a decoded offset to raw bytes. This is an exact structural-token edit, never repository-wide string replacement.

### Required stale-token gate

After edits/regenerations, scan every non-excluded text input for the exact old structural sequence:

```text
/🏅️standards/<standard>/🪆️subsets/<subset>/🧬️schema/🧬️mutations/<mutation>/🧪️tests/
```

Allowed hits are zero for projected artifact vectors. A prose hit is stale documentation, not an exemption. References to mutation implementation code that stop before `/🧪️tests/` remain valid.

## Collision, budget, and reference policy

Before emitting moves:

1. Build all 1,555 identities and destination roots, including the three canonical DIN IDs.
2. Reject two distinct `(standardVersion, subsetId)` tuples rendering the same profile within one artifact.
3. Reject two different identities rendering the same destination under byte, NFC, case-fold, VS16-fold, or same-kind comparison.
4. Reject any destination occupied by a node outside the projection source set.
5. Validate Windows reserved names and trailing dot/space on every destination segment.
6. Validate `Buffer.byteLength(destination, "utf8") <= 240` on every directory and file destination.
7. Require every incoming reference to be a structured edit/regeneration; generated/binary unresolved edges remain blocking.

The current inventory passes items 2–4 with zero groups. After the three DIN canonical names it passes item 6 with zero projected violations.

## Deterministic apply and rollback

### Plan order

1. Verify baseline commit, source-tree digest, taxonomy/catalog schema, and opaque digests without traversing excluded roots.
2. Discover and sort identities by UTF-8 source root bytes; validate every 13-node bundle and owner vector.
3. Compute the three DIN canonical IDs, every destination, collisions, budgets, and the complete reference graph before writing.
4. Emit only file `TaxonomyMove` records, sorted by `(sourcePath, destinationPath)`. Use the existing operation ID digest over `move\0source\0destination` and rationale `artifact-mutation-test-projection-v1`.
5. Emit structured edits sorted by `(final path, structured location, old value, new value)`. References in moved files use their final path as the relative-path base.
6. Emit regenerations only for schema-registered owners, sorted by regeneration ID, with exact output hashes. The current fixture/vector population has no registered regeneration owner, so its plan must use edits and have an empty regeneration list.
7. Hash canonical plan bytes with an empty/omitted self digest as already specified by `taxonomyPlanDigest`.

### Apply order

1. Preflight all source hashes, edit preimages, destination occupancy, catalog hashes, and cancellation.
2. Stage all 9,330 files in plan order before installing any destination. This preserves the existing two-phase collision safety.
3. Install all staged files in plan order; create profile/mutation/scenario/fixture parents as needed without replacing the artifact's existing executable `🧪️tests/mutate-*` cases.
4. Apply grouped structured edits in final-path order, then registered regenerations in ID order.
5. Verify every destination hash/mode, exact bundle membership, identity/catalog bijection, reference target, path policy, old-token absence, and expected post-state digest.
6. Recheck opaque digests. Prune only empty source `🧪️tests` descendants; mutation implementation directories remain non-empty and must never be pruned.
7. Commit the journal and remove only transaction staging/backups.

Cancellation is checked before each identity validation, move, edit group, regeneration, and verification group. Failure injection at `after-staging`, `after-moves`, `after-edits`, and `before-verify` must cover this projection.

### Rollback order

1. Mark the journal `rolling-back`.
2. Restore edited/generated output backups in reverse final-path order.
3. Move installed destinations back to their operation staging slots in reverse move order.
4. Move staged files back to exact source paths in reverse move order, restoring modes and hashes.
5. Remove only transaction-created empty destination directories, deepest first; never remove an existing `🧪️tests` root or aggregate case.
6. Verify the pre-transaction affected-tree digest and opaque digests, then mark `rolled-back`.

The second inventory/plan after a successful apply must contain no projection moves, no reference edits, no stale old path token, and no path-policy violation for these vectors.

## CAD and other non-mutation remnants

- CAD mutation vectors that match the exact source grammar are included in the 1,555-scenario projection, including profiles `🪆️1-any`, `🪆️v1-cad`, `🪆️ac1018-any`, `🪆️ac1024-any`, and `🪆️r12-any` where owned by their artifacts.
- The **102** CAD example entries under `…/📚️examples/🖼️assets/🏗️modelDefinitions/…` do not match and remain unresolved. They require the separate artifact-level example/model-catalog projection proposed by H-PATH-BUDGET; reusing this mutation contract would falsify their ownership and fixture shape.
- Artifact aggregate executable cases such as `<artifact>/🧪️tests/mutate-cad-1` remain where they are. The new profile directories are specification-vector storage, not executable case owners.
- Framework/OS mutation fixtures lack the artifact/standard/subset identity and do not match. They need their own owner contract if budget remediation is later required.
- Ticket/governance paths, Draw's fixed Nx manifest, and all other non-mutation path remnants remain separate work. The projection must not lower their severity or increase the path limit.

## Test plan

### Language-agnostic golden

Add a JSON golden fixture containing only strings and records:

- identities from `🪆️1-any`, `🪆️2.0-any`, `🪆️v1-drawing`, `🪆️ecma-376-any`, and a hyphenated standard such as `🪆️mpeg1-layer3-any`;
- both diff alternatives;
- the three DIN canonical scenario IDs and five exact deepest-path byte counts;
- one collision fixture with two source tuples rendering the same profile;
- one foreign/unregistered mutation member, malformed bundle, symlink, stale asset URI, and unsupported prose escape.

The same golden bytes must be consumable from Bun/TypeScript and a language-neutral JSON parser; no TypeScript-only object snapshot is the authority.

### Third-party parity

`fast-glob@3.3.3` is already a test-only dependency of `@semio-tech/repo-lib`. In an isolated disposable Git fixture:

1. Materialize representative old trees plus unrelated examples, aggregate cases, framework mutations, and a tiny opaque `compose/` fixture.
2. `git init`, configure a fixture-local identity, `git add`, and commit only inside that disposable root.
3. Use `fast-glob` to enumerate the old source grammar and the six physical file alternatives.
4. Independently transform those captures with a test-only pure reference renderer.
5. Assert the engine's source membership and exact sorted destination bytes equal the fast-glob/reference set.
6. Assert fast-glob's negative controls are absent and the opaque bytes/digest are unchanged.

This gives third-party parity for language-agnostic path selection while the production engine remains dependency-free.

### Focused behavioral coverage

- all 53 profile renderings and all 1,555 identity records are unique;
- 1,524 currently long scenarios are distinguished from 31 currently short scenarios, but all project;
- exact 13-node bundle validation and 9,330 file moves;
- zero collision/folded collision/outside occupancy;
- Gherkin `asset://`, Python `VECTOR_ROOT`/format, Rust paths/includes, JSON rationale, and prose structured locations/preimages;
- aggregate Nx case discovery ignores profile storage while still discovering `mutate-*` features;
- cancellation and every failure stage restore source bytes, modes, references, catalogs, and opaque digest;
- successful apply yields an empty second plan and zero old structural tokens;
- focused command: `bun nx run @semio-tech/repo-lib:test -- --test-name-pattern=artifact-mutation-test-projection` (selector passed in the package script's supported equal/no-space form if required by Bun), followed by the repository test-domain focused contract target.

No test was run for this read-only design packet.

## Acceptance checks

1. Exactly 1,555 registered identities / 20,215 projected nodes are discovered from the canonical population; exactly 9,330 file moves are planned.
2. Exactly 53 profile tuple renderings exist and no destination is reverse-parsed by hyphen splitting.
3. Every projected scenario has exactly the 13-node contract and one exclusive diff alternative.
4. The three DIN scenario IDs are the canonical shortened values; all five formerly residual paths are at most 238 bytes.
5. No projected path exceeds 240 bytes; `collisionPolicy.maxPathBytes` is unchanged.
6. Byte/NFC/case/VS16/same-kind collisions and outside destination occupancy are zero.
7. Every mutation member resolves through its captured subset owner; no global emoji/slug guess is accepted.
8. Every cross-boundary reference is a structured edit or registered regeneration; stale old hierarchy tokens are zero.
9. Existing aggregate executable cases remain discoverable and projected profile storage creates no Nx project.
10. Apply, cancellation, all four failure stages, rollback, resume, and empty-second-plan convergence preserve exact bytes/modes and opaque digests.
11. CAD model-definition examples and all non-artifact mutation remnants remain explicit unresolved work, not silently folded into this contract.

## Reproducible read-only commands

Representative command forms used against explicit non-opaque roots:

```text
sed -n '1,260p' <ticket>/📓️h-path-budget.md
bun -e '<load canonical inventory; match exact artifact/standard/subset/schema/mutation/tests grammar; group profiles/scenarios/suffixes>'
bun -e '<render destinations; count byte/NFC/case/VS16 collisions and outside occupancy>'
bun -e '<partition inventory reference edges into internal and cross-boundary memberships>'
rg -n '🧬️mutations/.*/🧪️tests/' ✏️s 🧰️framework --glob '!**/compose/**' --glob '!**/temp/compose/**'
rg -n 'VECTOR_ROOT|asset://|🧪️tests/.*/🦀️component.rs' <explicit plugin/test-domain files>
sed -n '<regions>' <taxonomy/discovery/normalization/test-domain files>
```

No search or read traversed `compose/**` or `temp/compose/**`.

## Contract correction from implementation census

The implementation census supersedes the proposed `vectors[].kind ↔ kinds[]` bijection above. Runtime `kinds` and physical projection vectors are independent owner facts:

- `kinds` remains the capability vocabulary an executable mutation feature covers;
- required `vectors` is the complete physical projection registry and may be `[]` when that catalog owns no physical bundle;
- each vector is `{ mutationId, mutationDirectoryName, scenarios }`, with unique exact mutation identity and one or more canonical scenario identities;
- the physical registry, not runtime dispatch membership, must biject with the actual 13-node scenario bundles.

This distinction is required by current evidence. The 144 non-Compose catalogs declare 2,173 runtime kinds but own 1,555 physical vector directories/scenarios; 30 catalogs own no physical vector. glTF deliberately exposes seven dispatchable runtime kinds while its checked-in schema contains 120 exact physical vectors. Expanding runtime kinds, dropping physical vectors, or assigning invented emoji would falsify one of those contracts.

The implemented invariant is therefore:

`registered physical (mutationDirectoryName, canonical scenario id) ↔ physical source/projected 13-node bundle`

exactly once, independent of `kinds`. The three DIN canonical IDs remain registered before their transactional physical/reference rename and surface as explicit `mutation-vector-source-id-mismatch` findings until apply; no source alias is accepted.
