# H-CAD-EXAMPLE-PATH — CAD catalog and Draw fixed-manifest projection audit

## Decision

Two schema-first, forward-only projection contracts are required.

1. `artifact-example-model-catalog-v1` projects the CAD distributed JSON model catalog. It is a manifest-owned recursive catalog whose semantic JSON stems must become directories and whose leaves must become physical JSON-kind leaves.
2. `artifact-editor-command-bundle-v1` projects one exact Draw command bundle. It contains two Rust packages and a fixed Nx manifest; it is not a model catalog and must not share the CAD descendant contract.

Both contracts may share one new forward-only profile renderer, `artifact-standard-subset-v1`, but not the mutation-only `standard-subset-v1` renderer whose live `directoryKindId` is `mutation-test-profile`. Neither contract may reverse-split `🪆️1-any`, abbreviate an identity, hash an identity, raise `collisionPolicy.maxPathBytes`, or reuse `artifact-mutation-tests-v1`.

The CAD projection is not ready to apply until the owner of `🧱️aec.building.concrete` supplies an authoritative model-definition manifest or relocates its eight action records to their actual manifest owner. The current tree and reachable Git history contain no `🔣️modelDefinition.json` for that directory. Guessing `aec.building.concrete` from its path would make the filesystem, not a manifest, the identity authority.

## Baseline and scope

- Workspace: `/Users/ueli/Documents/semio`
- Observed `HEAD`: `a03e259755a2448dea999fc9e621139b5b480881`
- Audit population: 64,726 Git-visible non-Compose files from `git ls-files -z`, plus their derived directories.
- Compose and `temp/compose` were excluded lexically before every inventory/reference operation and were not read.
- CAD source files and the Draw command bundle are present and have no worktree overlap. Relevant concurrent overlap exists in `Cargo.toml` (`MM`), `Cargo.lock` (`M`), root `📜️script.ts` (`M`), taxonomy (`MM`), and discovery (`MM`). An implementation must use current-content preimage guards and preserve those concurrent edits.
- Current taxonomy SHA-256: `0ae9ba190562362a6b3abb107cd03dddb6cadf724b549f93f80006180e4c5d18`.
- Current discovery SHA-256: `15c88ec0d4fa6a75af1ca5f4dea13f9946026aa02cab4985967f993e1deb647a`.
- CAD Git-index bundle digest over sorted `mode\0blobOid\0path`: `eea3f29103a38eafd556d89d3eb7ad29fbdeff10bf21f454b189f1bb0f60aa92`.
- Draw command Git-index bundle digest over the same encoding: `31a4e99ea9a9db62af91675acecde524f84e0f74d7f5a1e784277b3f136c22ed`.
- The 102-row CAD offender digest over sorted `nodeType\0utf8Bytes\0path`: `2f509ba67e4472eedd13c7cece1bdcf43af9b752c19bcc204f125496035bdd10`.

No production test or mutating command was run. All commands in this packet were read-only.

## CAD inventory and authority

### Exact owner and profile

| Dimension | Exact identity |
|---|---|
| plugin | `📐️cad` |
| artifact | `📐️cad` |
| standard directory / value | `🔖️1` / `1` |
| subset directory / value | `✳️any` / `any` |
| rendered profile | `🪆️1-any` |
| historical source catalog root | [Frozen `/projections/0/sourceRoot` coordinate](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json#/projections/0/sourceRoot) |

The current population gives no evidence that this catalog is profile-independent: it is physically owned by standard `1` and subset `any`, and no catalog manifest declares independence. The destination must therefore retain the compact profile.

### Physical population

- 208 tracked JSON files, 92 derived directories, 300 current nodes.
- Current file-path range: 214–293 UTF-8 bytes. Current all-node range: 166–293 bytes.
- Exactly 91 files and 11 directories exceed 240 bytes: 102 entries and 91 minimal frontiers.
- Longest current path: 293 bytes, ending in `🗺️aec.building.structure.fem.surface/🔀️transformations/🔀️from_aec.building.structure/🔣️transformation.json`.
- File categories: 88 actions, 49 interactions, 10 transformations, 8 model-definition roots, 38 typologies, 10 attribute definitions, 3 stat definitions, 1 property kind, and 1 property definition.
- Schema counts: `spatial.action` 88, `spatial.interaction` 49, `spatial.transformation` 10, `spatial.modelDefinition` 8, `spatial.typology` 38, `spatial.attribute` 10, `spatial.stat` 3, and `spatial.property` 2.
- Every file has top-level string `id`, `version: "1.0.0"`, and a recognized `schema`. There are no outgoing filesystem references in their JSON values; the six slash-containing strings are human text or units.

### Model owners

| Directory | Files | Manifest `id` |
|---|---:|---|
| `🏛️aec.building.structure.classic` | 22 | `aec.building.structure.classic` |
| `🏛️aec.building.structure` | 13 | `aec.building.structure` |
| `🏢️aec.building` | 14 | `aec.building` |
| `📏️aec.building.structure.fem.line` | 7 | `aec.building.structure.fem.line` |
| `📐️spatial.shape` | 101 | `spatial.shape` |
| `🔥️aec.building.energy` | 29 | `aec.building.energy` |
| `🗺️aec.building.structure.fem.surface` | 7 | `aec.building.structure.fem.surface` |
| `🧊️aec.building.structure.fem.solid` | 7 | `aec.building.structure.fem.solid` |
| `🧱️aec.building.concrete` | 8 | **missing** |

The missing owner accounts for eight of the 102 long entries. No matching manifest was found in reachable Git history using the exact and historical filename forms.

### Identity cannot be only `id`

The 208 records contain 192 unique top-level IDs and 14 intentional duplicate-ID groups (30 records):

| Duplicate `id` | Contexts |
|---|---|
| `from_aec.building.structure` | four transformation owners: classic, FEM line, FEM surface, FEM solid |
| `spatial.shape.gvalue` | structure and energy attributes |
| `spatial.shape.material` | structure and building attributes |
| `spatial.shape.opening` | structure and building attributes |
| `spatial.shape.uvalue` | structure and energy attributes |
| `curve.line`, `curve.polyline`, `entity.createAnchor` | action and interaction under `spatial.shape` |
| `transform.copy`, `transform.mirror`, `transform.move`, `transform.rotate`, `transform.scale1d`, `transform.scale3d` | action and interaction under `spatial.shape` |

The lossless catalog identity is therefore:

```text
(artifactMemberName, standardVersion, subsetId,
 modelDirectoryName, modelDefinitionId,
 categoryDirectoryName, memberLocalName, manifestSchema, manifestId)
```

`modelDefinitionId` must come from the model manifest. `memberLocalName` is copied exactly from the current semantic file stem or existing semantic member directory; `manifestId` remains byte-identical in JSON. The tuple, not any one field, is unique.

## CAD target contract

### Exact grammar

```text
source:
<artifactRoot>/🏅️standards/🔖️<standardVersion>/🪆️subsets/✳️<subsetId>/
📚️examples/🖼️assets/🏗️modelDefinitions/<distributed catalog>

destination:
<artifactRoot>/📚️examples/🪆️<standardVersion>-<subsetId>/🏗️models/
<canonical distributed catalog>
```

`models` is the complete English plural, not an abbreviation. The redundant `assets` layer is absent because `models` is the exact asset kind inside the artifact's examples profile. Retaining `🖼️assets/🏗️models` leaves nine paths over budget; retaining `🏗️modelDefinitions` without `assets` leaves one path over budget. A simple byte-for-byte descendant copy to `.../🏗️models` reaches 231 bytes but retains semantic JSON filenames and is therefore not the normalization target.

### Canonical descendant rendering

| Source shape | Destination shape | Count |
|---|---|---:|
| `<model>/🔣️modelDefinition.json` | `<model>/🔣️.json` | 8 current; the ninth owner is blocked |
| `<model>/🎬️actions/🔣️<localName>.json` | `<model>/🎬️actions/🎬️<localName>/🔣️.json` | 88 |
| `<model>/🎬️interactions/🔣️<localName>.json` | `<model>/🎬️interactions/🎬️<localName>/🔣️.json` | 49 |
| `<model>/🏷️attributeDefinitions/🔣️<localName>.json` | `<model>/🏷️attributeDefinitions/🏷️<localName>/🔣️.json` | 10 |
| `<model>/📊️statDefinitions/🔣️<localName>.json` | `<model>/📊️statDefinitions/📊️<localName>/🔣️.json` | 3 |
| `<model>/🏷️propertyKinds/🔣️<localName>.json` | `<model>/🏷️propertyKinds/🏷️<localName>/🔣️.json` | 1 |
| `<model>/🔧️propertyDefinitions/🔣️<localName>.json` | `<model>/🔧️propertyDefinitions/🔧️<localName>/🔣️.json` | 1 |
| `<model>/🗂️typologies/<existingMember>/🔣️typology.json` | `<model>/🗂️typologies/<existingMember>/🔣️.json` | 38 |
| `<model>/🔀️transformations/<existingMember>/🔣️transformation.json` | `<model>/🔀️transformations/<existingMember>/🔣️.json` | 10 |

The member names are copied, never shortened. The content's full `id` remains unchanged. Existing JSON content and modes remain unchanged; only paths change.

For the 208 current files this renders 244 directories and 452 nodes. The longest destination is 237 bytes:

```text
✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📚️examples/🪆️1-any/🏗️models/🏛️aec.building.structure.classic/🎬️actions/🎬️constructReinforcedConcreteExternalWallFrom2PointsAndHeight/🔣️.json
```

The other tied 237-byte path is the corresponding internal-wall action. The destination root is 104 bytes; the realized longest descendant suffix including its separator is 133 bytes, leaving a three-byte reserve at the current catalog population. New catalog members must be rejected before write when their fully rendered path exceeds 240; a constant `3` must not become a generic future allowance.

Mapping SHA-256 over sorted `source\0destination` for the 208 canonical file moves: `2098704ee35021e7ef1a116aca7b75e071e0e05f693529dabaead3cde2a35451`.

### Required schema shape

The live exact-bundle descendant shape is insufficient for a recursive manifest catalog. Add a strict tagged catalog variant; do not flatten these 452 nodes into a global emoji union.

```json
{
  "semanticDirectoryKinds": {
    "standard-subset-profile": {
      "emoji": "🪆️",
      "slugPattern": "^[a-z0-9][a-z0-9.\\-]*-[a-z0-9][a-z0-9.\\-]*$",
      "allowEmojiOnly": false,
      "parentKindIds": ["examples", "editor"]
    },
    "models": {
      "emoji": "🏗️",
      "slugPattern": "^models$",
      "allowEmojiOnly": false,
      "parentKindIds": ["standard-subset-profile"]
    }
  },
  "semanticPathProjectionProfileRenderers": {
    "artifact-standard-subset-v1": {
      "direction": "forward-only",
      "captureFields": ["standardVersion", "subsetId"],
      "directoryKindId": "standard-subset-profile",
      "template": "🪆️{standardVersion}-{subsetId}",
      "tupleCollisionFields": ["artifactId", "standardVersion", "subsetId"]
    }
  },
  "semanticPathProjectionCatalogContracts": {
    "cad-model-catalog-v1": {
      "contractKind": "distributed-json-manifest-catalog",
      "ownerArtifactMemberName": "📐️cad",
      "modelManifestSchema": "spatial.modelDefinition",
      "modelManifestSourceFilename": "🔣️modelDefinition.json",
      "modelIdentityField": "id",
      "memberIdentityField": "id",
      "memberVersionField": "version",
      "requiredModelManifest": true,
      "coverage": "every-source-file-and-destination-node-exactly-once",
      "unknownCategoryPolicy": "problem",
      "unownedModelPolicy": "problem"
    }
  },
  "semanticDescendantContracts": {
    "cad-model-catalog-bundle-v1": {
      "contractKind": "catalog",
      "rootDirectoryKindId": "models",
      "catalogContractId": "cad-model-catalog-v1",
      "leafFileKindId": "json",
      "rendering": "semantic-member-directory-and-physical-kind-leaf",
      "pathBudgetReserve": {
        "derivation": "longest-rendered-catalog-descendant-suffix",
        "bytes": 133
      }
    }
  },
  "semanticPathProjectionContracts": {
    "artifact-example-model-catalog-v1": {
      "sourceOwnerKindId": "members-of-artifacts",
      "sourceArtifactMemberName": "📐️cad",
      "sourceSegments": [
        {"kindId": "standards", "literal": "🏅️standards"},
        {"kindId": "standard", "capture": "standardVersion"},
        {"kindId": "subsets", "literal": "🪆️subsets"},
        {"kindId": "subset", "capture": "subsetId"},
        {"kindId": "examples", "literal": "📚️examples"},
        {"kindId": "assets", "literal": "🖼️assets"},
        {"memberKindId": "members-of-assets", "literal": "🏗️modelDefinitions"}
      ],
      "profileRendererId": "artifact-standard-subset-v1",
      "destinationOwnerKindId": "members-of-artifacts",
      "destinationSegments": [
        {"kindId": "examples", "literal": "📚️examples"},
        {"kindId": "standard-subset-profile", "render": "profile"},
        {"kindId": "models", "literal": "🏗️models"}
      ],
      "descendantContractId": "cad-model-catalog-bundle-v1",
      "catalogContractId": "cad-model-catalog-v1",
      "rationaleRule": "artifact-example-model-catalog-projection-v1"
    }
  }
}
```

Strict validation must additionally require exact category records for the nine evidenced category shapes and their schema/emoji/rendering shown above; require every distributed model/member manifest exactly once; check model-manifest `id` against the registered model directory; reject extra/missing/symlink nodes; reject catalog cycles and duplicate identity tuples; and resolve projected member directories only with the full artifact/profile/model/category context. A source segment `memberKindId` is an exact source-only registry match, not an alias for the canonical `models` kind.

The literal `pathBudgetReserve.bytes` above is the **suffix cost** (133), consistent with the live mutation contract's usage; the remaining capacity for this exact live root is `240 - 104 - 133 = 3`.

## Draw inventory and target contract

### Exact owner and fixed manifest

Historical source command root: [frozen Draw `/projections/1/sourceRoot` coordinate](../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json#/projections/1/sourceRoot).

```text
242-byte fixed Nx manifest:
<source command root>/🔄️fsm/✨️macros/📦️packages/🦀️rust/📋️project.json
```

- plugin/artifact: `🖍️draw` / `🖍️draw`.
- standard/subset/profile: `1` / `any` / `🪆️1-any`.
- surface/command: `✏️editor` / `🖱️canvas-pointer-down`.
- nested registered identities: `🔄️fsm`, then `✨️macros`.
- fixed filename contract: `nx-project-manifest`; the basename remains exactly `📋️project.json`.
- Nx identity: `@semio-tech/draw-fsm-macros`.
- sibling package identity: `@semio-tech/draw-fsm`.
- Cargo identities: `semio-s-plugin-draw-fsm-macros` and `semio-s-plugin-draw-fsm`.
- 11 files, 7 derived directories, 18 nodes. The Nx manifest is the only current node over 240 bytes.

The coherent move unit is the whole command bundle, not the fixed manifest or macros directory alone. It contains:

```text
🦀️component.rs
🔄️fsm/🦀️component.rs
🔄️fsm/📦️packages/🦀️rust/{Cargo.toml,📋️project.json,📜️script.ts,📦️glue.rs}
🔄️fsm/✨️macros/🦀️component.rs
🔄️fsm/✨️macros/📦️packages/🦀️rust/{Cargo.toml,📋️project.json,📜️script.ts,📦️glue.rs}
```

### Exact grammar

```text
source:
<artifactRoot>/🏅️standards/🔖️<standardVersion>/🪆️subsets/✳️<subsetId>/
✏️editor/🎮️commands/<registeredCommand>/<exact command bundle>

destination:
<artifactRoot>/✏️editor/🪆️<standardVersion>-<subsetId>/
🎮️commands/<registeredCommand>/<canonical exact command bundle>
```

The three `🦀️component.rs` files become physical Rust-kind leaves `🦀️.rs`. Cargo manifests, Nx manifests, permanent scripts, and package glue retain their exact schema/tool contracts. All other relative topology is copied.

For the current command this renders 11 files, 7 directories, 18 nodes, with a maximum of 204 bytes and 36 bytes of remaining capacity. Byte, NFC, case-fold, VS16-fold, and outside-occupancy groups are all zero. Mapping SHA-256 over the 11 canonical file moves: `2341b92ad57c7e9103a7a4ee40e47d99fe561a21641c51a1a87fa2197fa76814`.

The exact descendant contract must enumerate the 18-node shape and use a strict file-node union:

```text
kindId: rust-source                       for 🦀️.rs
fixedFilenameContractId: cargo-manifest  for Cargo.toml
fixedFilenameContractId: nx-project-manifest for 📋️project.json
fixedFilenameContractId: root-script     for 📜️script.ts
packageGlue: { ecosystemId: 🦀️rust, filename: 📦️glue.rs } for package glue
```

Exactly one of `kindId`, `fixedFilenameContractId`, or `packageGlue` must be present on a file node. This is a required strict extension to the current descendant-node shape; classifying fixed manifests as ordinary file kinds would violate the physical/tool contract.

The projection record is:

```json
{
  "semanticProjectedMemberKinds": {
    "projected-editor-command": {
      "ownerKindIds": ["commands"],
      "projectionContractId": "artifact-editor-command-bundle-v1",
      "sourceMemberKindId": "members-of-commands",
      "identityField": "commandDirectoryName"
    }
  },
  "semanticPathProjectionCatalogContracts": {
    "draw-editor-command-vectors-v1": {
      "contractKind": "exact-owner-vectors",
      "required": true,
      "allowEmpty": false,
      "identityFields": ["artifactId", "standardVersion", "subsetId", "commandDirectoryName"],
      "coverage": "every-physical-command-bundle-exactly-once",
      "vectors": [
        {
          "artifactId": "🖍️draw",
          "standardVersion": "1",
          "subsetId": "any",
          "commandDirectoryName": "🖱️canvas-pointer-down"
        }
      ]
    }
  },
  "semanticPathProjectionContracts": {
    "artifact-editor-command-bundle-v1": {
      "sourceOwnerKindId": "members-of-artifacts",
      "sourceArtifactMemberName": "🖍️draw",
      "sourceSegments": [
        {"kindId": "standards", "literal": "🏅️standards"},
        {"kindId": "standard", "capture": "standardVersion"},
        {"kindId": "subsets", "literal": "🪆️subsets"},
        {"kindId": "subset", "capture": "subsetId"},
        {"kindId": "editor", "literal": "✏️editor"},
        {"kindId": "commands", "literal": "🎮️commands"},
        {"projectedMemberKindId": "projected-editor-command", "capture": "commandDirectoryName"}
      ],
      "profileRendererId": "artifact-standard-subset-v1",
      "destinationOwnerKindId": "members-of-artifacts",
      "destinationSegments": [
        {"kindId": "editor", "literal": "✏️editor"},
        {"kindId": "standard-subset-profile", "render": "profile"},
        {"kindId": "commands", "literal": "🎮️commands"},
        {"projectedMemberKindId": "projected-editor-command", "copy": "commandDirectoryName"}
      ],
      "descendantContractId": "draw-editor-command-bundle-v1",
      "catalogContractId": "draw-editor-command-vectors-v1",
      "rationaleRule": "artifact-editor-command-projection-v1"
    }
  }
}
```

## Collision, Unicode, VS16, and occupancy evidence

The collision census compared all projected files and derived directories with every non-source tracked file and derived directory. Compose remained lexically excluded.

| Projection | Files | Destination dirs | Destination nodes | Max bytes | Over 240 | byte internal/occupied | NFC | case-fold | VS16-fold |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| canonical CAD catalog | 208 | 244 | 452 | 237 | 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 |
| canonical Draw command | 11 | 7 | 18 | 204 | 0 | 0 / 0 | 0 / 0 | 0 / 0 | 0 / 0 |

All source and destination nodes are NFC. No source or destination contains VS15. All newly rendered emoji literals include canonical VS16. Collision checks must remain separate byte, NFC, case-fold, VS16-fold, and same-kind checks in production; the audit also used a combined NFC/case/VS16 key and found zero groups.

## Incoming references and required adapters

### CAD

The exact deep source hierarchy occurs 1,671 times in ten non-Compose tracked files: 74 occurrences in three live production files and 1,597 occurrences in seven ticket/governance evidence files. Historical ticket evidence records old paths and must not be rewritten. The live path/reference contract must reach zero old structural tokens; the historical evidence set is outside that stale-token gate.

The live semantic-path census contains 78 relevant occurrences in five owners:

| Owner | Occurrences | Required treatment |
|---|---:|---|
| `.../📐️cad/🎬️interaction-spec/🦀️component.rs` | 14 | 12 `include_str!` targets, one runtime catalog-root join, one doc path |
| `.../✏️editor/⚙️engine/🕹️interaction/🦀️component.rs` | 49 | 49 `include_str!` targets |
| `.../✏️editor/⚙️engine/🏃️runtime/🟦️component.ts` | 13 | 12 Vite `import.meta.glob` strings and one doc path |
| `.../🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts` | 1 | replace the old `🖼️assets/🏗️modelDefinitions/` parser marker with the schema-owned projected models root; no compatibility markers |
| taxonomy JSON | 1 | remove `🏗️modelDefinitions` from the ordinary asset-member registry and register the source literal/canonical projection contracts |

Required structured adapters:

- reuse Rust `include_str!` token edits with final-target resolution;
- reuse TypeScript string/glob tokenization, but render the new canonical patterns (`🎬️actions/*/🔣️.json`, etc.) rather than substituting only the root;
- update the Rust disk-walk assertion from `parent == 🎬️interactions` to the exact canonical grandparent/member/`🔣️.json` shape;
- update the catalog-root `Path.join` as a structured Rust string;
- update documentation subspans without renaming conceptual APIs such as `modelDefinitions` variables;
- update the spatial-kernel parser through an exact projected-root helper; remove its old ASCII/singular/plural marker aliases;
- update the taxonomy/catalog registry in the same transaction.

The runtime currently declares empty globs for extensions and properties that have zero catalog files. They must not become wildcard allowances. Either remove those empty source patterns or add future categories schema-first when an authoritative manifest exists.

### Draw

The exact command source prefix occurs 313 times in 75 non-Compose tracked files: 21 occurrences in five live/root files and 292 occurrences in 70 historical ticket/governance files. Historical evidence remains unchanged.

External incoming live edges are exactly 13:

- root `Cargo.toml`: two workspace-member paths;
- root `📜️script.ts`: one direct command component path;
- root `🔒️dependencies.json`: eight manifest-user paths (five FSM, three macro);
- Draw outer Rust package `Cargo.toml`: one relative dependency on the FSM package;
- Draw outer Rust package `📦️glue.rs`: one relative `#[path]` to the command component.

Moved-file edits include ten self-paths in the two Nx manifests (two workspace globs and eight target `cwd` values), two `$schema` paths, three external Cargo dependency paths, and two internal Rust `#[path]` values for the renamed `🦀️.rs` leaves. The internal FSM-to-macros Cargo path and both package-glue paths retain relative topology but still require resolution verification against the move map.

Required adapters are the existing JSON/JSONC string and JSON-pointer edits, TOML strings including Cargo paths and workspace arrays, Rust `#[path]`, and TypeScript exact strings. `🔒️dependencies.json` should receive eight preimage-guarded structured JSON edits; do not run the hand-ratcheted `write-baseline` command merely to update paths. Run its non-writing verification afterward.

The two destination Nx `$schema` values must resolve from the **final** project directories. The external Cargo dependency paths similarly lose three ancestor traversals. Do not perform textual `../` trimming: resolve old targets, apply the complete move map, then render relative paths from the final reference path.

## Atomic apply requirements

1. Quiesce writers for taxonomy, discovery, normalization, root `Cargo.toml`, root `📜️script.ts`, and affected CAD/Draw owners. Preserve current concurrent changes; never reset/stash/checkout.
2. Preflight current-content hashes/modes, taxonomy strict load, owner vectors, catalog schemas, exact bundle membership, symlinks, opaque digests, and cancellation before staging.
3. Resolve the `🧱️aec.building.concrete` authority gap. The acceptable outcomes are an owner-authored model manifest with a validated ID or relocation of all eight actions under their real manifest owner. There is no inferred-ID fallback.
4. Land both projection contracts, the forward-only renderer, directory/category kinds, catalog contracts, descendant contracts, and strict validators atomically. No optional/default/legacy parser is allowed.
5. Build the entire plan before writes: 208 current CAD file moves plus 11 Draw file moves, the owner-authorized concrete correction, all structured edits, zero collisions, zero occupied destinations, and every destination at most 240 bytes.
6. Stage every existing source file with preimage and mode before installing destinations. Create destination directories deepest only as required; never move a fixed manifest alone.
7. Apply reference edits using final-path bases. All moved existing files must retain content hashes unless they themselves own a structured reference edit. The 208 CAD JSON contents must remain byte-identical.
8. Run schema-registered regenerations only. No evidenced generator owns the CAD catalog or Draw command manifests; the plan should contain no guessed regeneration.
9. Verify owner/catalog bijection, exact descendant bundles, JSON schema/id/version, Nx fixed filenames, Cargo/Nx discovery, every reference target, byte/NFC/case/VS16/same-kind collisions, zero live old-hierarchy tokens, maximum path bytes, and opaque digests.
10. Prune only transaction-created or now-empty source directories, deepest first. On failure/cancellation, restore edits and moves in reverse order and prove the affected-tree pre-state digest. A second inventory/plan must be empty.

## Required tests and acceptance checks

### Language-agnostic golden

Add one canonical JSON golden with:

- the exact CAD and Draw source/destination grammars;
- all 208 CAD source/destination file pairs and 11 Draw pairs;
- expected mapping digests `2098704e...5451` and `2341b92a...6814`;
- all nine model-directory records, with concrete explicitly missing an owner manifest in the negative fixture;
- the eight category render rules and manifest schema/ID fields;
- the fixed Nx/Cargo/script/package-glue node roles;
- expected counts, maximum bytes, Unicode form, and collision/occupancy results.

Consume the same JSON and compare the same sorted UTF-8 destination bytes in Bun/TypeScript and an existing Rust JSON implementation. No TypeScript object snapshot is the authority.

### Third-party parity

Use existing test-only `fast-glob@3.3.3` in a disposable Git fixture to enumerate the two source grammars, then independently render destinations from the golden. Compare exact sorted pairs, negative controls, file contents, modes, and opaque digest with the production planner. The production engine remains dependency-free.

### Required negatives

- missing/unregistered model manifest (`🧱️aec.building.concrete`);
- duplicate distributed identity tuple or duplicate catalog membership;
- unknown schema/category, missing/extra node, symlink, or partial Draw package;
- source profile tuple collision and any attempted reverse split of a rendered profile;
- non-NFC/VS15 source, VS16-fold collision, case-fold collision, occupied destination, and a 241-byte rendered path;
- attempt to rename `📋️project.json`, `Cargo.toml`, `📜️script.ts`, or package glue as an ordinary file kind;
- unresolved Rust/TOML/JSON/TypeScript reference, stale live path token, and historical ticket evidence incorrectly selected as an edit target;
- cancellation/failure after staging, moves, edits, and before verify, each restoring exact hashes and modes.

### Focused verification after implementation

```text
bun nx run @semio-tech/repo-lib:test -- --test-name-pattern=artifact-example-model-catalog-projection
bun nx run @semio-tech/repo-lib:test -- --test-name-pattern=artifact-editor-command-projection
bun nx show project @semio-tech/draw-fsm --json
bun nx show project @semio-tech/draw-fsm-macros --json
bun nx run @semio-tech/draw-fsm:test-quick
bun nx run @semio-tech/draw-fsm-macros:test-quick
bun nx run @semio-tech/cad-plugin:test-quick
bun nx run @semio-tech/cad-js:test-quick
bun ./📜️script.ts verify dependencies
cargo metadata --no-deps --format-version 1
```

Acceptance is: two and only two new projection contracts; all current 219 existing files covered exactly once; the concrete owner gap resolved without inference; zero destination paths above 240; maxima 237 and 204 for the evidenced population; zero byte/NFC/case/VS16/same-kind collisions and outside occupancy; fixed Nx name retained; all live incoming edges resolved; CAD IDs/content preserved; historical evidence unchanged; rollback/resume/empty-second-plan green.

## Exhaustive 102-entry CAD offender inventory

All paths below are relative to the exact CAD source catalog root stated above. `F` is a tracked file and `D` is a derived directory. The list has 91 `F` and 11 `D` rows.

```text
289 F 🏛️aec.building.structure.classic/🎬️actions/🔣️constructOneWayReinforcedConcreteSlabFrom2PointsAndHeight.json
287 F 🏛️aec.building.structure.classic/🎬️actions/🔣️constructOneWayReinforcedConcreteSlabFromCurveAndHeight.json
280 F 🏛️aec.building.structure.classic/🎬️actions/🔣️constructOneWayReinforcedConcreteSlabFromSurface.json
285 F 🏛️aec.building.structure.classic/🎬️actions/🔣️constructReinforcedConcreteColumnFrom2PointsAndHeight.json
283 F 🏛️aec.building.structure.classic/🎬️actions/🔣️constructReinforcedConcreteColumnFromCurveAndHeight.json
276 F 🏛️aec.building.structure.classic/🎬️actions/🔣️constructReinforcedConcreteColumnFromSurface.json
291 F 🏛️aec.building.structure.classic/🎬️actions/🔣️constructReinforcedConcreteExternalWallFrom2PointsAndHeight.json
289 F 🏛️aec.building.structure.classic/🎬️actions/🔣️constructReinforcedConcreteExternalWallFromCurveAndHeight.json
282 F 🏛️aec.building.structure.classic/🎬️actions/🔣️constructReinforcedConcreteExternalWallFromSurface.json
291 F 🏛️aec.building.structure.classic/🎬️actions/🔣️constructReinforcedConcreteInternalWallFrom2PointsAndHeight.json
289 F 🏛️aec.building.structure.classic/🎬️actions/🔣️constructReinforcedConcreteInternalWallFromCurveAndHeight.json
282 F 🏛️aec.building.structure.classic/🎬️actions/🔣️constructReinforcedConcreteInternalWallFromSurface.json
274 F 🏛️aec.building.structure.classic/🎬️interactions/🔣️constructOneWayReinforcedConcreteSlab.json
270 F 🏛️aec.building.structure.classic/🎬️interactions/🔣️constructReinforcedConcreteColumn.json
276 F 🏛️aec.building.structure.classic/🎬️interactions/🔣️constructReinforcedConcreteExternalWall.json
276 F 🏛️aec.building.structure.classic/🎬️interactions/🔣️constructReinforcedConcreteInternalWall.json
262 D 🏛️aec.building.structure.classic/🔀️transformations/🔀️from_aec.building.structure
289 F 🏛️aec.building.structure.classic/🔀️transformations/🔀️from_aec.building.structure/🔣️transformation.json
254 D 🏛️aec.building.structure.classic/🗂️typologies/🏛️ReinforcedConcreteColumn
275 F 🏛️aec.building.structure.classic/🗂️typologies/🏛️ReinforcedConcreteColumn/🔣️typology.json
258 D 🏛️aec.building.structure.classic/🗂️typologies/🧱️OneWayReinforcedConcreteSlab
279 F 🏛️aec.building.structure.classic/🗂️typologies/🧱️OneWayReinforcedConcreteSlab/🔣️typology.json
260 D 🏛️aec.building.structure.classic/🗂️typologies/🧱️ReinforcedConcreteExternalWall
281 F 🏛️aec.building.structure.classic/🗂️typologies/🧱️ReinforcedConcreteExternalWall/🔣️typology.json
260 D 🏛️aec.building.structure.classic/🗂️typologies/🧱️ReinforcedConcreteInternalWall
281 F 🏛️aec.building.structure.classic/🗂️typologies/🧱️ReinforcedConcreteInternalWall/🔣️typology.json
245 F 🏛️aec.building.structure/🏷️attributeDefinitions/🔣️bondable.json
245 F 🏛️aec.building.structure/🏷️attributeDefinitions/🔣️exposure.json
243 F 🏛️aec.building.structure/🏷️attributeDefinitions/🔣️gvalue.json
245 F 🏛️aec.building.structure/🏷️attributeDefinitions/🔣️material.json
244 F 🏛️aec.building.structure/🏷️attributeDefinitions/🔣️opening.json
243 F 🏛️aec.building.structure/🏷️attributeDefinitions/🔣️uvalue.json
241 F 🏛️aec.building.structure/📊️statDefinitions/🔣️stability.json
261 F 🏛️aec.building.structure/🔀️transformations/🏛️classic/🔣️transformation.json
261 F 🏛️aec.building.structure/🔀️transformations/📏️linefem/🔣️transformation.json
267 F 🏛️aec.building.structure/🔀️transformations/🔀️from_building/🔣️transformation.json
264 F 🏛️aec.building.structure/🔀️transformations/🗺️surfacefem/🔣️transformation.json
262 F 🏛️aec.building.structure/🔀️transformations/🧊️solidfem/🔣️transformation.json
243 F 🏢️aec.building/🗂️typologies/🪨️Foundation/🔣️typology.json
273 F 📏️aec.building.structure.fem.line/🎬️actions/🔣️constructLineElementFrom2PointsAndHeight.json
271 F 📏️aec.building.structure.fem.line/🎬️actions/🔣️constructLineElementFromCurveAndHeight.json
264 F 📏️aec.building.structure.fem.line/🎬️actions/🔣️constructLineElementFromSurface.json
258 F 📏️aec.building.structure.fem.line/🎬️interactions/🔣️constructLineElement.json
263 D 📏️aec.building.structure.fem.line/🔀️transformations/🔀️from_aec.building.structure
290 F 📏️aec.building.structure.fem.line/🔀️transformations/🔀️from_aec.building.structure/🔣️transformation.json
242 D 📏️aec.building.structure.fem.line/🗂️typologies/📏️LineElement
263 F 📏️aec.building.structure.fem.line/🗂️typologies/📏️LineElement/🔣️typology.json
241 F 📐️spatial.shape/🎬️actions/🔣️setCubeHeightFromFootprint.json
250 F 📐️spatial.shape/🗂️typologies/〰️ControlPointCurve/🔣️typology.json
245 F 📐️spatial.shape/🗂️typologies/➡️ExtrudeCurve/🔣️typology.json
250 F 📐️spatial.shape/🗂️typologies/🌊️InterpolateCurve/🔣️typology.json
242 F 📐️spatial.shape/🗂️typologies/📐️Polyline/🔣️typology.json
248 F 📐️spatial.shape/🗂️typologies/🕸️NetworkSurface/🔣️typology.json
242 F 📐️spatial.shape/🗂️typologies/🥫️Cylinder/🔣️typology.json
250 F 🔥️aec.building.energy/🎬️actions/🔣️constructBasePlateFromSurface.json
262 F 🔥️aec.building.energy/🎬️actions/🔣️constructExternalWallFrom2PointsAndHeight.json
260 F 🔥️aec.building.energy/🎬️actions/🔣️constructExternalWallFromCurveAndHeight.json
253 F 🔥️aec.building.energy/🎬️actions/🔣️constructExternalWallFromSurface.json
254 F 🔥️aec.building.energy/🎬️actions/🔣️constructHullFrom2PointsAndHeight.json
252 F 🔥️aec.building.energy/🎬️actions/🔣️constructHullFromCurveAndHeight.json
245 F 🔥️aec.building.energy/🎬️actions/🔣️constructHullFromSurface.json
254 F 🔥️aec.building.energy/🎬️actions/🔣️constructRoofFrom2PointsAndHeight.json
252 F 🔥️aec.building.energy/🎬️actions/🔣️constructRoofFromCurveAndHeight.json
245 F 🔥️aec.building.energy/🎬️actions/🔣️constructRoofFromSurface.json
257 F 🔥️aec.building.energy/🎬️actions/🔣️constructWindowsFrom2PointsAndHeight.json
255 F 🔥️aec.building.energy/🎬️actions/🔣️constructWindowsFromCurveAndHeight.json
248 F 🔥️aec.building.energy/🎬️actions/🔣️constructWindowsFromSurface.json
244 F 🔥️aec.building.energy/🎬️interactions/🔣️constructBasePlate.json
247 F 🔥️aec.building.energy/🎬️interactions/🔣️constructExternalWall.json
242 F 🔥️aec.building.energy/🎬️interactions/🔣️constructWindows.json
241 F 🔥️aec.building.energy/📊️statDefinitions/🔣️energydemand.json
264 F 🔥️aec.building.energy/🔀️transformations/🔀️from_geometry/🔣️transformation.json
245 F 🔥️aec.building.energy/🔧️propertyDefinitions/🔣️heatedvolume.json
244 F 🔥️aec.building.energy/🗂️typologies/🏠️Roof/🔣️typology.json
244 F 🔥️aec.building.energy/🗂️typologies/🐚️Hull/🔣️typology.json
252 F 🔥️aec.building.energy/🗂️typologies/🧱️ExternalWall/🔣️typology.json
247 F 🔥️aec.building.energy/🗂️typologies/🪟️Windows/🔣️typology.json
249 F 🔥️aec.building.energy/🗂️typologies/🟫️BasePlate/🔣️typology.json
279 F 🗺️aec.building.structure.fem.surface/🎬️actions/🔣️constructSurfaceElementFrom2PointsAndHeight.json
277 F 🗺️aec.building.structure.fem.surface/🎬️actions/🔣️constructSurfaceElementFromCurveAndHeight.json
270 F 🗺️aec.building.structure.fem.surface/🎬️actions/🔣️constructSurfaceElementFromSurface.json
264 F 🗺️aec.building.structure.fem.surface/🎬️interactions/🔣️constructSurfaceElement.json
266 D 🗺️aec.building.structure.fem.surface/🔀️transformations/🔀️from_aec.building.structure
293 F 🗺️aec.building.structure.fem.surface/🔀️transformations/🔀️from_aec.building.structure/🔣️transformation.json
248 D 🗺️aec.building.structure.fem.surface/🗂️typologies/🗺️SurfaceElement
269 F 🗺️aec.building.structure.fem.surface/🗂️typologies/🗺️SurfaceElement/🔣️typology.json
275 F 🧊️aec.building.structure.fem.solid/🎬️actions/🔣️constructSolidElementFrom2PointsAndHeight.json
273 F 🧊️aec.building.structure.fem.solid/🎬️actions/🔣️constructSolidElementFromCurveAndHeight.json
266 F 🧊️aec.building.structure.fem.solid/🎬️actions/🔣️constructSolidElementFromSurface.json
260 F 🧊️aec.building.structure.fem.solid/🎬️interactions/🔣️constructSolidElement.json
264 D 🧊️aec.building.structure.fem.solid/🔀️transformations/🔀️from_aec.building.structure
291 F 🧊️aec.building.structure.fem.solid/🔀️transformations/🔀️from_aec.building.structure/🔣️transformation.json
244 D 🧊️aec.building.structure.fem.solid/🗂️typologies/🧊️SolidElement
265 F 🧊️aec.building.structure.fem.solid/🗂️typologies/🧊️SolidElement/🔣️typology.json
260 F 🧱️aec.building.concrete/🎬️actions/🔣️constructConcreteWallFromBottomAndTop.json
254 F 🧱️aec.building.concrete/🎬️actions/🔣️constructExtrudedMushroomColumn.json
260 F 🧱️aec.building.concrete/🎬️actions/🔣️constructFullyQuadraticMushroomColumn.json
246 F 🧱️aec.building.concrete/🎬️actions/🔣️constructMushroomColumn.json
274 F 🧱️aec.building.concrete/🎬️actions/🔣️constructRectangularMushroomColumnWithQuadraticSlab.json
252 F 🧱️aec.building.concrete/🎬️actions/🔣️constructVerticalConcreteWall.json
264 F 🧱️aec.building.concrete/🎬️actions/🔣️constructWallFromHorizontalPathAndProfile.json
265 F 🧱️aec.building.concrete/🎬️actions/🔣️constructWallFromHorizontalPathAndProfiles.json
```

## Reproducible read-only evidence commands

```text
git rev-parse HEAD
git ls-files -z
git status --short -- <exact CAD root> <exact Draw root> Cargo.toml Cargo.lock 🔒️dependencies.json 📜️script.ts
git grep -n -I -F 'modelDefinitions' -- '✏️s/**' '🧰️framework/**' ':!compose/**' ':!temp/compose/**'
git grep -n -I -F '🎮️commands/🖱️canvas-pointer-down' -- ':!compose/**' ':!temp/compose/**'
git log --all -- ':(glob)**/🧱️aec.building.concrete/🔣️modelDefinition.json'
shasum -a 256 <taxonomy> <discovery>
bun -e '<derive tracked directories; parse 208 JSON manifests; enumerate duplicate identities>'
bun -e '<render both canonical mappings; measure UTF-8 bytes; compare byte/NFC/case/VS16 occupancy>'
bun -e '<hash sorted index bundle, offender inventory, and source/destination pairs>'
```
