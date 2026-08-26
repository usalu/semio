# Independent CAD and Draw Projection Audit

## Result

The live non-Compose physical trees independently derive exactly **209 CAD** and **11 Draw** source-to-destination file pairs. Source authority, destination rendering, identity tuples, descendant closure, normalization/collision keys, occupancy, path budgets, fixed roles, and incoming references all close without an unresolved source.

After freezing the independent derivation, I compared it with the implementation golden. The two pair sets are identical: **0 missing from the golden, 0 extra in the golden**, and both per-contract digests match. No production, test, physical-tree, Compose/temp-Compose, AGENTS, or Git-state change was made.

The exhaustive machine-readable 220-pair evidence is:

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json`

That file was opened only after the independent physical mapping and digests were frozen.

## Frozen projection identities

| Contract | Owner tuple | Source root | Destination root |
| --- | --- | --- | --- |
| `artifact-example-model-catalog-v1` | `📐️cad / 1 / any` | `…/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions` | `…/📚️examples/🪆️1-any/🏗️models` |
| `artifact-editor-command-bundle-v1` | `🖍️draw / 1 / any / 🖱️canvas-pointer-down` | `…/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down` | `…/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down` |

The shared profile renderer is forward-only: `(standardVersion, subsetId) = (1, any)` renders exact directory `🪆️1-any`. Neither projection reverse-splits that slug.

## Independently derived grammar

### CAD

The model-definition root has nine exact semantic model directories. Each requires `🔣️modelDefinition.json` with schema `spatial.modelDefinition`, version `1.0.0`, and an `id` rendered by its owning model directory. That manifest becomes the model directory's physical JSON leaf:

```text
<source-model>/🔣️modelDefinition.json
→ 📚️examples/🪆️1-any/🏗️models/<model>/🔣️.json
```

Direct semantic JSON categories move the semantic source filename stem into a category-owned directory. The manifest `id` remains intact inside JSON and remains the catalog identity; it is not shortened, hashed, or substituted for the physical semantic stem:

```text
<model>/<category>/🔣️<semantic-stem>.json
→ 📚️examples/🪆️1-any/🏗️models/<model>/<category>/<category-emoji><semantic-stem>/🔣️.json
```

Nested fixed JSON categories retain their already-semantic member directory and replace only the externally semantic fixed filename with a physical JSON-kind leaf:

```text
<model>/🗂️typologies/<semantic-member>/🔣️typology.json
→ …/🏗️models/<model>/🗂️typologies/<semantic-member>/🔣️.json

<model>/🔀️transformations/<semantic-member>/🔣️transformation.json
→ …/🏗️models/<model>/🔀️transformations/<semantic-member>/🔣️.json
```

This distinction matters for path budget and semantic correctness. For example, source `🔣️constructReinforcedConcreteExternalWallFrom2PointsAndHeight.json` becomes directory `🎬️constructReinforcedConcreteExternalWallFrom2PointsAndHeight`; the JSON keeps the full manifest ID `structure.structure.constructReinforcedConcreteExternalWallFrom2PointsAndHeight`. Rendering the full manifest ID into the directory would incorrectly produce a 257-byte path, while the contract's source-stem rendering produces the verified 237-byte maximum.

### Draw

The exact owner vector selects one command bundle. Its internal directory shape is copied beneath the profile destination. Three Rust implementation files become physical `🦀️.rs` leaves. Cargo, Nx, permanent script, and package glue identities remain exact:

```text
🦀️component.rs → 🦀️.rs
Cargo.toml       → Cargo.toml
📋️project.json  → 📋️project.json
📜️script.ts     → 📜️script.ts
📦️glue.rs       → 📦️glue.rs
```

The source bundle is exactly 18 nodes: 7 directories and 11 files. File roles are 3 Rust physical leaves, 2 Cargo manifests, 2 Nx manifests, 2 permanent scripts, and 2 Rust package-glue files. The Cargo packages are `semio-s-plugin-draw-fsm` and `semio-s-plugin-draw-fsm-macros`; the Nx projects are `@semio-tech/draw-fsm` and `@semio-tech/draw-fsm-macros`.

## Source closure

All 209 CAD source files are valid JSON and match one exact declared shape. There are no unknown categories, unknown model directories, unexpected files, missing fixed manifests, schema mismatches, version mismatches, or inferred model authorities.

| Model directory | Files/pairs |
| --- | ---: |
| `🏛️aec.building.structure` | 13 |
| `🏛️aec.building.structure.classic` | 22 |
| `🏢️aec.building` | 14 |
| `📏️aec.building.structure.fem.line` | 7 |
| `📐️spatial.shape` | 101 |
| `🔥️aec.building.energy` | 29 |
| `🗺️aec.building.structure.fem.surface` | 7 |
| `🧊️aec.building.structure.fem.solid` | 7 |
| `🧱️aec.building.concrete` | 9 |
| **Total** | **209** |

The concrete model is not inferred. Its exact manifest declares:

```json
{
  "schema": "spatial.modelDefinition",
  "id": "aec.building.concrete",
  "version": "1.0.0",
  "kinds": ["action"]
}
```

That manifest and its eight sibling `spatial.action` records account for all nine concrete pairs.

CAD file roles are:

| Role | Count |
| --- | ---: |
| model manifests | 9 |
| actions | 88 |
| interactions | 49 |
| attribute definitions | 10 |
| stat definitions | 3 |
| property kinds | 1 |
| property definitions | 1 |
| typologies | 38 |
| transformations | 10 |
| **Total** | **209** |

The nine model tuples and all 200 member tuples are unique under `(artifactId, standardVersion, subsetId, modelId, category, memberId)`: **209 tuples, 209 unique, 0 duplicates**. The Draw owner tuple is singular and unique; its 11 descendant file identities are also unique.

## Destination closure and occupancy

Counts rooted at each golden `destinationRoot` are:

| Projection | Directories | Files | Nodes |
| --- | ---: | ---: | ---: |
| CAD `🏗️models` subtree | 244 | 209 | 453 |
| Draw command subtree | 7 | 11 | 18 |
| **Total contract subtrees** | **251** | **220** | **471** |

The CAD directory count is `1 models root + 9 models + 34 model/category instances + 200 semantic members`. The Draw directory count is `1 command root + 6 bundle descendants`.

Full closure from each artifact owner also includes destination parents. CAD adds `📚️examples/🪆️1-any`, and Draw adds `✏️editor/🪆️1-any/🎮️commands`:

| Scope | Directories | Files | Nodes |
| --- | ---: | ---: | ---: |
| Full CAD owner-relative destination | 246 | 209 | 455 |
| Full Draw owner-relative destination | 10 | 11 | 21 |
| **Full combined destination** | **256** | **220** | **476** |

All 220 destination files and 255 of 256 destination directories are absent. The sole existing ancestor is CAD `…/📐️cad/📚️examples`, which currently owns `🔣️machine.json`; adding child `🪆️1-any` is non-colliding. There is no occupied destination root, descendant, or leaf.

## Normalization, collision, and path-budget proof

All 220 sources and destinations are NFC. The destination pair set has:

- 0 exact collisions;
- 0 NFC collisions;
- 0 Unicode case-fold collisions;
- 0 VS16-stripped plus case-fold collisions;
- 0 same-parent/same-file-kind collisions;
- 0 source-to-destination identity tuple collisions.

Exact UTF-8 maxima:

| Projection | Source maximum | Destination maximum | Budget margin |
| --- | ---: | ---: | ---: |
| CAD | 293 bytes | 237 bytes | 3 bytes under 240 |
| Draw | 242 bytes | 204 bytes | 36 bytes under 240 |

The 293-byte CAD source maximum is the surface-FEM transformation `…/🔀️transformations/🔀️from_aec.building.structure/🔣️transformation.json`. The 237-byte CAD destination maximum is the classic reinforced-concrete external-wall action. The Draw maximum is the macros package's fixed Nx manifest: 242 bytes at source and 204 at destination.

The schema reserves exactly match the realized maxima:

```text
CAD destination root 104 bytes + CAD reserve 133 bytes = 237
Draw destination root 132 bytes + Draw reserve 72 bytes = 204
```

The projection therefore repairs the two over-budget source trees without abbreviation or hashing.

## Incoming live reference surface

The scan covered 104,676 filesystem files under `✏️s`, `🧰️framework`, and root files, lexically skipping every `compose` directory, excluding the CAD/Draw golden during independent derivation, and never reading Git state.

### CAD

There are **76 source-root/path occurrences in 4 live files**. Of these, 61 are exact references to 49 source files and 15 are root/glob/parser references:

| Referencing file | Total | Exact source file | Root/glob only |
| --- | ---: | ---: | ---: |
| CAD editor interaction Rust component | 49 | 49 | 0 |
| CAD artifact `🎬️interaction-spec` Rust component | 13 | 12 | 1 |
| CAD editor runtime TypeScript component | 13 | 0 | 13 |
| spatial-kernel geometry TypeScript component | 1 | 0 | 1 |

The atomic transaction must rewrite Rust `include_str!` paths, TypeScript `import.meta.glob` patterns and their authoritative path comments, the interaction-spec path root, and the spatial-kernel model-definition path recognizer. Merely moving the 209 JSON files would leave compile-time and runtime discovery stale.

### Draw

There are **23 source-root/path occurrences in 7 live files**. Ten are exact references to three source files; thirteen reference source directories:

| Referencing file | Occurrences | Form |
| --- | ---: | --- |
| root `🔒️dependencies.json` | 8 | exact Cargo manifest paths |
| two moving `📋️project.json` files | 5 each | self-owned input/cwd directory paths |
| root `Cargo.toml` | 2 | workspace member directories |
| Draw package `📦️glue.rs` | 1 | exact command Rust source path |
| Draw package `Cargo.toml` | 1 | FSM package directory dependency |
| root `📜️script.ts` | 1 | exact command Rust source path |

The transaction must rewrite/regenerate all 23 occurrences together with the 11 moves. In particular, preserving the two fixed Nx manifests without rewriting their `namedInputs` and four `cwd` values each would retain valid filenames but invalid project metadata.

## Exact pair digests and golden comparison

Digest algorithm `sha256-source-nul-destination-lines-v1` is:

1. render each full repo-relative pair as `source + U+0000 + destination`;
2. sort the rendered strings by deterministic code-unit order;
3. join with `U+000A`, with no trailing newline;
4. SHA-256 the UTF-8 bytes.

| Pair set | Count | Independent digest | Golden digest | Match |
| --- | ---: | --- | --- | --- |
| CAD | 209 | `a09f60c5de5718394ddb856052444b306de7443b2d4ecd546e1e911dc44d40a6` | same | yes |
| Draw | 11 | `2341b92ad57c7e9103a7a4ee40e47d99fe561a21641c51a1a87fa2197fa76814` | same | yes |
| Combined | 220 | `9d97fb4cd002509e119fa1fde4a851f605d964441f588b28c8f65f4cb5661b70` | n/a | n/a |

Exact set comparison returned:

```json
{
  "derived": 220,
  "golden": 220,
  "missingFromGolden": 0,
  "extraInGolden": 0
}
```

The golden's directory/node counts use each declared `destinationRoot` as the closure root (244/453 CAD and 7/18 Draw). The larger 246/455 and 10/21 figures above are the requested full artifact-owner closure including destination parents; this is a scope distinction, not a mapping discrepancy.

## Atomic acceptance checks

- Load and validate all nine model manifests before planning any pair; concrete remains exact authored authority.
- Validate every direct/nested member schema, version, source shape, and scoped identity tuple; unknown files/categories fail closed.
- Produce exactly the two digests above before apply.
- Require all 220 destinations and all non-ancestor destination directories to be unoccupied under exact, NFC, case-fold, and VS16-fold keys.
- Keep fixed Draw Cargo/Nx/script/glue basenames unchanged while moving their containing directories and rewriting all structured references.
- Apply 220 file moves, destination directory creation, and 99 live path-reference rewrites as one rollback-capable transaction. The 99 count is 76 CAD plus 23 Draw textual occurrences; it does not imply 99 distinct files.
- After apply, re-run schema validation, pair-set/digest validation, Cargo metadata, Nx project discovery, Rust path compilation, TypeScript glob discovery, and the CAD/Draw golden.
