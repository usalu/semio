# H-CAD-DRAW Reference Surface Audit

## Outcome

The requested live reference surface is exactly **99 path-bearing occurrences: 76 CAD and 23 Draw**. Against the current normalization adapters, the partition is:

| Classification | CAD | Draw | Total |
| --- | ---: | ---: | ---: |
| Exact supported structured edit | 61 | 20 | **81** |
| Projection-owned structural rewrite | 0 | 0 | **0** |
| Irrelevant token | 0 | 0 | **0** |
| Unresolved adapter gap | 15 | 3 | **18** |
| **Total** | **76** | **23** | **99** |

All 99 occurrences carry repository path meaning. Even the two comments describe authoritative path grammar and must change with the transaction; neither is dismissed as an irrelevant coincidental token. The live normalizer has no generic CAD/Draw projection-reference adapter, so no occurrence qualifies as a projection-owned structural rewrite. Its only special structural projection grammar is mutation-specific.

One additional executable CAD root token exists immediately beside the counted surface at `🎬️interaction-spec/🦀️component.rs:638`. It was not included in the stated 76 because the census anchor was `🖼️assets/🏗️modelDefinitions/` with a trailing slash while this string ends at `modelDefinitions`. It is an additional unresolved adapter gap, making **19 gaps to repair in code** while preserving the requested 99-occurrence partition.

No production, taxonomy, test, physical-tree, Compose, temp/Compose, AGENTS, or Git state was modified. This report is the only write.

## Frozen path grammar

The exact mapping oracle is:

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json`

Its SHA-256 during this audit was `52f2a3c5b7e44114b0eff260c8f7f5116acbf7e969bf0a5e13e24021284ca820`.

CAD roots:

```text
C0 = ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions
C1 = ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📚️examples/🪆️1-any/🏗️models
```

Every exact CAD target below follows the golden's exact `C0/<source suffix> → C1/<destination suffix>` pair. Renameable semantic JSON leaves become semantic directories with physical `🔣️.json` leaves; model manifests become `<model>/🔣️.json`.

Draw roots:

```text
D0 = ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️canvas-pointer-down
D1 = ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down
```

Every Draw directory reference replaces the exact `D0` prefix with `D1`. The three renameable Rust source leaves use `🦀️component.rs → 🦀️.rs`; fixed Cargo, Nx, permanent-script, and package-glue basenames remain unchanged.

## Adapter evidence

The audited normalizer was `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`, SHA-256 `e47ae8a37a160baf9e7cf55b779314ff86b4c00bb909b4313c1505f840d85d79`.

- `jsonTokens` at line 1939 emits exact raw JSON string tokens. Project/package/launch embedded-argument recognition delegates to `embeddedArgumentTokens` at line 1830, which accepts `${workspaceRoot}/…`, not Nx's `{workspaceRoot}/…` form.
- `tomlTokens` at line 1963 emits every quoted TOML string. This covers workspace members and Cargo path dependencies.
- `rustTokens` at line 1978 covers `#[path = "…"]` and `include_str!`/`include!`/`include_bytes!`. Its comment projection grammar only recognizes the old mutation-test shape.
- `typescriptTokens` at line 2026 covers imports, selected URL/worker calls, path-like variable assignments, and selected path/read calls. It does not cover `import.meta.glob`, arbitrary array strings, or `Path::join` in Rust.
- `resolveReferencePath` at line 2120 rejects any token containing `*` or `{}`. Consequently wildcard and Nx placeholder paths cannot resolve to a moving inventory entry.
- `unsupportedReferenceTokens` at line 2217 only fails closed when a token resolves to an exact moving target. It detects the exact root-script array string, but wildcard, fragment, and prose tokens remain silent.
- `buildReferenceEdits` at line 2301 uses the moving reference file's final path when rendering a relative edit. It therefore supports the eight Draw `cwd` strings even though their two Nx manifests move.

## CAD occurrence ledger

### CAD editor interaction Rust component — 49 supported

Reference file:

`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🕹️interaction/🦀️component.rs`

All 49 are exact `include_str!` literals, are recognized by `rustTokens`, resolve to a unique source file, and have an exact golden destination. The table uses `C0`/`C1`-relative suffixes.

| Line | Source suffix | Destination suffix |
| ---: | --- | --- |
| 67 | `📐️spatial.shape/🎬️interactions/🔣️arc.json` | `📐️spatial.shape/🎬️interactions/🎬️arc/🔣️.json` |
| 68 | `📐️spatial.shape/🎬️interactions/🔣️area.json` | `📐️spatial.shape/🎬️interactions/🎬️area/🔣️.json` |
| 69 | `📐️spatial.shape/🎬️interactions/🔣️booleanDifference.json` | `📐️spatial.shape/🎬️interactions/🎬️booleanDifference/🔣️.json` |
| 70 | `📐️spatial.shape/🎬️interactions/🔣️booleanIntersection.json` | `📐️spatial.shape/🎬️interactions/🎬️booleanIntersection/🔣️.json` |
| 71 | `📐️spatial.shape/🎬️interactions/🔣️booleanUnion.json` | `📐️spatial.shape/🎬️interactions/🎬️booleanUnion/🔣️.json` |
| 72 | `📐️spatial.shape/🎬️interactions/🔣️box.json` | `📐️spatial.shape/🎬️interactions/🎬️box/🔣️.json` |
| 73 | `📐️spatial.shape/🎬️interactions/🔣️chamfer.json` | `📐️spatial.shape/🎬️interactions/🎬️chamfer/🔣️.json` |
| 74 | `📐️spatial.shape/🎬️interactions/🔣️circle.json` | `📐️spatial.shape/🎬️interactions/🎬️circle/🔣️.json` |
| 75 | `📐️spatial.shape/🎬️interactions/🔣️constructCurve.json` | `📐️spatial.shape/🎬️interactions/🎬️constructCurve/🔣️.json` |
| 76 | `📐️spatial.shape/🎬️interactions/🔣️constructSurface.json` | `📐️spatial.shape/🎬️interactions/🎬️constructSurface/🔣️.json` |
| 77 | `📐️spatial.shape/🎬️interactions/🔣️controlPointCurve.json` | `📐️spatial.shape/🎬️interactions/🎬️controlPointCurve/🔣️.json` |
| 78 | `📐️spatial.shape/🎬️interactions/🔣️copy.json` | `📐️spatial.shape/🎬️interactions/🎬️copy/🔣️.json` |
| 79 | `📐️spatial.shape/🎬️interactions/🔣️createAnchor.json` | `📐️spatial.shape/🎬️interactions/🎬️createAnchor/🔣️.json` |
| 80 | `📐️spatial.shape/🎬️interactions/🔣️cylinder.json` | `📐️spatial.shape/🎬️interactions/🎬️cylinder/🔣️.json` |
| 81 | `📐️spatial.shape/🎬️interactions/🔣️explode.json` | `📐️spatial.shape/🎬️interactions/🎬️explode/🔣️.json` |
| 82 | `📐️spatial.shape/🎬️interactions/🔣️extrudeCrv.json` | `📐️spatial.shape/🎬️interactions/🎬️extrudeCrv/🔣️.json` |
| 83 | `📐️spatial.shape/🎬️interactions/🔣️extrudeWire.json` | `📐️spatial.shape/🎬️interactions/🎬️extrudeWire/🔣️.json` |
| 84 | `📐️spatial.shape/🎬️interactions/🔣️fillet.json` | `📐️spatial.shape/🎬️interactions/🎬️fillet/🔣️.json` |
| 85 | `📐️spatial.shape/🎬️interactions/🔣️interpolateCurve.json` | `📐️spatial.shape/🎬️interactions/🎬️interpolateCurve/🔣️.json` |
| 86 | `📐️spatial.shape/🎬️interactions/🔣️join.json` | `📐️spatial.shape/🎬️interactions/🎬️join/🔣️.json` |
| 87 | `📐️spatial.shape/🎬️interactions/🔣️length.json` | `📐️spatial.shape/🎬️interactions/🎬️length/🔣️.json` |
| 88 | `📐️spatial.shape/🎬️interactions/🔣️line.json` | `📐️spatial.shape/🎬️interactions/🎬️line/🔣️.json` |
| 89 | `📐️spatial.shape/🎬️interactions/🔣️loft.json` | `📐️spatial.shape/🎬️interactions/🎬️loft/🔣️.json` |
| 90 | `📐️spatial.shape/🎬️interactions/🔣️mirror.json` | `📐️spatial.shape/🎬️interactions/🎬️mirror/🔣️.json` |
| 91 | `📐️spatial.shape/🎬️interactions/🔣️move.json` | `📐️spatial.shape/🎬️interactions/🎬️move/🔣️.json` |
| 92 | `📐️spatial.shape/🎬️interactions/🔣️networkSrf.json` | `📐️spatial.shape/🎬️interactions/🎬️networkSrf/🔣️.json` |
| 93 | `📐️spatial.shape/🎬️interactions/🔣️offsetSurface.json` | `📐️spatial.shape/🎬️interactions/🎬️offsetSurface/🔣️.json` |
| 94 | `📐️spatial.shape/🎬️interactions/🔣️plane.json` | `📐️spatial.shape/🎬️interactions/🎬️plane/🔣️.json` |
| 95 | `📐️spatial.shape/🎬️interactions/🔣️polyline.json` | `📐️spatial.shape/🎬️interactions/🎬️polyline/🔣️.json` |
| 96 | `📐️spatial.shape/🎬️interactions/🔣️rotate.json` | `📐️spatial.shape/🎬️interactions/🎬️rotate/🔣️.json` |
| 97 | `📐️spatial.shape/🎬️interactions/🔣️scale1d.json` | `📐️spatial.shape/🎬️interactions/🎬️scale1d/🔣️.json` |
| 98 | `📐️spatial.shape/🎬️interactions/🔣️scale3d.json` | `📐️spatial.shape/🎬️interactions/🎬️scale3d/🔣️.json` |
| 99 | `📐️spatial.shape/🎬️interactions/🔣️sphere.json` | `📐️spatial.shape/🎬️interactions/🎬️sphere/🔣️.json` |
| 100 | `📐️spatial.shape/🎬️interactions/🔣️split.json` | `📐️spatial.shape/🎬️interactions/🎬️split/🔣️.json` |
| 101 | `📐️spatial.shape/🎬️interactions/🔣️sweep1.json` | `📐️spatial.shape/🎬️interactions/🎬️sweep1/🔣️.json` |
| 102 | `📐️spatial.shape/🎬️interactions/🔣️sweep2.json` | `📐️spatial.shape/🎬️interactions/🎬️sweep2/🔣️.json` |
| 103 | `📐️spatial.shape/🎬️interactions/🔣️trim.json` | `📐️spatial.shape/🎬️interactions/🎬️trim/🔣️.json` |
| 104 | `🔥️aec.building.energy/🎬️interactions/🔣️constructBasePlate.json` | `🔥️aec.building.energy/🎬️interactions/🎬️constructBasePlate/🔣️.json` |
| 105 | `🔥️aec.building.energy/🎬️interactions/🔣️constructExternalWall.json` | `🔥️aec.building.energy/🎬️interactions/🎬️constructExternalWall/🔣️.json` |
| 106 | `🔥️aec.building.energy/🎬️interactions/🔣️constructHull.json` | `🔥️aec.building.energy/🎬️interactions/🎬️constructHull/🔣️.json` |
| 107 | `🔥️aec.building.energy/🎬️interactions/🔣️constructRoof.json` | `🔥️aec.building.energy/🎬️interactions/🎬️constructRoof/🔣️.json` |
| 108 | `🔥️aec.building.energy/🎬️interactions/🔣️constructWindows.json` | `🔥️aec.building.energy/🎬️interactions/🎬️constructWindows/🔣️.json` |
| 111 | `🏛️aec.building.structure.classic/🎬️interactions/🔣️constructOneWayReinforcedConcreteSlab.json` | `🏛️aec.building.structure.classic/🎬️interactions/🎬️constructOneWayReinforcedConcreteSlab/🔣️.json` |
| 115 | `🏛️aec.building.structure.classic/🎬️interactions/🔣️constructReinforcedConcreteColumn.json` | `🏛️aec.building.structure.classic/🎬️interactions/🎬️constructReinforcedConcreteColumn/🔣️.json` |
| 119 | `🏛️aec.building.structure.classic/🎬️interactions/🔣️constructReinforcedConcreteExternalWall.json` | `🏛️aec.building.structure.classic/🎬️interactions/🎬️constructReinforcedConcreteExternalWall/🔣️.json` |
| 123 | `🏛️aec.building.structure.classic/🎬️interactions/🔣️constructReinforcedConcreteInternalWall.json` | `🏛️aec.building.structure.classic/🎬️interactions/🎬️constructReinforcedConcreteInternalWall/🔣️.json` |
| 125 | `📏️aec.building.structure.fem.line/🎬️interactions/🔣️constructLineElement.json` | `📏️aec.building.structure.fem.line/🎬️interactions/🎬️constructLineElement/🔣️.json` |
| 128 | `🧊️aec.building.structure.fem.solid/🎬️interactions/🔣️constructSolidElement.json` | `🧊️aec.building.structure.fem.solid/🎬️interactions/🎬️constructSolidElement/🔣️.json` |
| 132 | `🗺️aec.building.structure.fem.surface/🎬️interactions/🔣️constructSurfaceElement.json` | `🗺️aec.building.structure.fem.surface/🎬️interactions/🎬️constructSurfaceElement/🔣️.json` |

The normalizer must render each new relative literal from the unchanged reference file to `C1`; for this file the common relative destination prefix is `../../../../../../../📚️examples/🪆️1-any/🏗️models/`.

### CAD interaction-spec Rust component — 12 supported, 1 gap

Reference file:

`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🎬️interaction-spec/🦀️component.rs`

| Line | Classification | Source suffix / token | Expected direction |
| ---: | --- | --- | --- |
| 2 | **Unresolved adapter gap** | Prose wildcard `🖼️assets/🏗️modelDefinitions/*/🎬️interactions/*.json` | Rewrite authoritative prose to `📚️examples/🪆️1-any/🏗️models/*/🎬️interactions/*/🔣️.json`. Rust comment projection parsing only recognizes mutation paths; wildcard resolution is rejected, so this is silent today. |
| 589 | Exact supported | `📐️spatial.shape/🎬️interactions/🔣️box.json` | `📐️spatial.shape/🎬️interactions/🎬️box/🔣️.json` |
| 604 | Exact supported | `📐️spatial.shape/🎬️interactions/🔣️sphere.json` | `📐️spatial.shape/🎬️interactions/🎬️sphere/🔣️.json` |
| 614 | Exact supported | `🔥️aec.building.energy/🎬️interactions/🔣️constructBasePlate.json` | `🔥️aec.building.energy/🎬️interactions/🎬️constructBasePlate/🔣️.json` |
| 615 | Exact supported | `🔥️aec.building.energy/🎬️interactions/🔣️constructExternalWall.json` | `🔥️aec.building.energy/🎬️interactions/🎬️constructExternalWall/🔣️.json` |
| 616 | Exact supported | `🔥️aec.building.energy/🎬️interactions/🔣️constructHull.json` | `🔥️aec.building.energy/🎬️interactions/🎬️constructHull/🔣️.json` |
| 617 | Exact supported | `🔥️aec.building.energy/🎬️interactions/🔣️constructRoof.json` | `🔥️aec.building.energy/🎬️interactions/🎬️constructRoof/🔣️.json` |
| 618 | Exact supported | `🔥️aec.building.energy/🎬️interactions/🔣️constructWindows.json` | `🔥️aec.building.energy/🎬️interactions/🎬️constructWindows/🔣️.json` |
| 619 | Exact supported | `🏛️aec.building.structure.classic/🎬️interactions/🔣️constructOneWayReinforcedConcreteSlab.json` | `🏛️aec.building.structure.classic/🎬️interactions/🎬️constructOneWayReinforcedConcreteSlab/🔣️.json` |
| 620 | Exact supported | `🏛️aec.building.structure.classic/🎬️interactions/🔣️constructReinforcedConcreteColumn.json` | `🏛️aec.building.structure.classic/🎬️interactions/🎬️constructReinforcedConcreteColumn/🔣️.json` |
| 621 | Exact supported | `🏛️aec.building.structure.classic/🎬️interactions/🔣️constructReinforcedConcreteExternalWall.json` | `🏛️aec.building.structure.classic/🎬️interactions/🎬️constructReinforcedConcreteExternalWall/🔣️.json` |
| 622 | Exact supported | `🏛️aec.building.structure.classic/🎬️interactions/🔣️constructReinforcedConcreteInternalWall.json` | `🏛️aec.building.structure.classic/🎬️interactions/🎬️constructReinforcedConcreteInternalWall/🔣️.json` |
| 695 | Exact supported | `🔥️aec.building.energy/🎬️interactions/🔣️constructExternalWall.json` | `🔥️aec.building.energy/🎬️interactions/🎬️constructExternalWall/🔣️.json` |

The 12 exact literals use `include_str!`; their rendered common relative destination prefix is `../📚️examples/🪆️1-any/🏗️models/`.

### CAD editor runtime TypeScript component — 13 gaps

Reference file:

`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏃️runtime/🟦️component.ts`

The destination glob root from this unchanged file is exactly:

`../../../../../../../📚️examples/🪆️1-any/🏗️models`

All 13 are unresolved adapter gaps. `typescriptTokens` has no `import.meta.glob` adapter, `resolveReferencePath` rejects `*`, and the unsupported scanner cannot associate the patterns with exact moving entries. The current four-`..` strings also normalize to a nonexistent duplicated `…/🪆️subsets/🗿️artifacts/…` path; the repair must render from the schema projection destination, not do substring replacement.

| Line | Occurrences | Token role | Expected direction |
| ---: | ---: | --- | --- |
| 161 | 1 | Authoritative model catalog grammar comment | Replace the old root with `📚️examples/🪆️1-any/🏗️models`; describe category members as semantic directories containing `🔣️.json`, and model manifests as `<model>/🔣️.json`. |
| 164 | 1 | `typologies` glob | `<root>/**/🗂️typologies/**/🔣️.json` |
| 168 | 1 | `actions` glob | `<root>/**/🎬️actions/*/🔣️.json` |
| 169 | 1 | `interactions` glob | `<root>/**/🎬️interactions/*/🔣️.json` |
| 170 | 1 | Model manifest glob | `<root>/*/🔣️.json` |
| 171 | 1 | `extensions` glob | The projection owns zero matching source files. Remove this dead selector unless a separate schema authority is added; do not fabricate a destination category. |
| 172 | 1 | Attribute-definition glob | `<root>/**/🏷️attributeDefinitions/*/🔣️.json` |
| 173 | 2 | Property-definition and property-kind globs | `<root>/**/🔧️propertyDefinitions/*/🔣️.json` and `<root>/**/🏷️propertyKinds/*/🔣️.json`. |
| 174 | 2 | Two `properties` globs | The projection owns zero matching source files for both selectors. Remove the dead selectors unless separately authorized; do not fabricate destinations. |
| 175 | 1 | Stat-definition glob | `<root>/**/📊️statDefinitions/*/🔣️.json` |
| 176 | 1 | Transformation glob | `<root>/**/🔀️transformations/**/🔣️.json` |

### Spatial-kernel geometry component — 1 gap

Reference file and location:

`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/📐️geometry/🟦️component.ts:2692`

The array marker `🖼️assets/🏗️modelDefinitions/` is executable parsing grammar used by `modelDefinitionAssetPathRest`. It is not captured by `typescriptTokens`, and as a path fragment it cannot resolve to an exact moving entry. Add/replace the canonical marker with `📚️examples/🪆️1-any/🏗️models/`; if old ASCII markers are no longer externally accepted identifiers, remove them rather than preserving compatibility.

## Draw occurrence ledger

### Root dependency graph — 8 supported

Reference file: `🔒️dependencies.json`.

Lines `2444, 2771, 2830, 3031, 3152, 3278, 3497, 3536` are exact Cargo manifest strings. `jsonTokens` recognizes every raw string, and each target maps uniquely. Replace `D0` with `D1`; preserve the remainder (`/🔄️fsm[/✨️macros]/📦️packages/🦀️rust/Cargo.toml`). This file is not excluded by a generator output contract in the audited taxonomy, so the normalizer will plan structured JSON edits.

### Moving Nx manifests — 8 supported, 2 gaps

Reference files:

- `D0/🔄️fsm/📦️packages/🦀️rust/📋️project.json`
- `D0/🔄️fsm/✨️macros/📦️packages/🦀️rust/📋️project.json`

For each file:

| Line | Classification | Expected direction |
| ---: | --- | --- |
| 5 | **Unresolved adapter gap** | Rewrite `{workspaceRoot}/D0/🔄️fsm[\/✨️macros]/**/*.rs` to `{workspaceRoot}/D1/🔄️fsm[\/✨️macros]/**/*.rs`. `embeddedArgumentTokens` only recognizes `${workspaceRoot}`, while the generic resolver rejects both `{}` and `*`; the gap is silent. |
| 11 | Exact supported | Replace the exact `cwd` directory prefix `D0 → D1`. |
| 19 | Exact supported | Replace the exact `cwd` directory prefix `D0 → D1`. |
| 27 | Exact supported | Replace the exact `cwd` directory prefix `D0 → D1`. |
| 35 | Exact supported | Replace the exact `cwd` directory prefix `D0 → D1`. |

The eight `cwd` edits remain supported even though the two manifest files move: `buildReferenceEdits` computes the relative/new value against each reference file's final projected path.

### Root Cargo workspace — 2 supported

`Cargo.toml:77` and `Cargo.toml:78` are exact workspace-member directories. `tomlTokens` recognizes both. Replace `D0 → D1` and preserve respectively `/🔄️fsm/📦️packages/🦀️rust` and `/🔄️fsm/✨️macros/📦️packages/🦀️rust`.

### Draw package Cargo dependency — 1 supported

`✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/Cargo.toml:27` contains the exact relative FSM package directory in a quoted TOML path dependency. `tomlTokens` recognizes it. Render the new relative value as:

`../../🗿️artifacts/🖍️draw/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🔄️fsm/📦️packages/🦀️rust`

### Draw package glue — 1 supported

`✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/📦️glue.rs:558` contains an exact Rust `#[path = "…"]` attribute. `rustTokens` recognizes it. Render:

`../../🗿️artifacts/🖍️draw/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs`

### Root permanent policy script — 1 detected gap

`📜️script.ts:7799` contains the exact command Rust source path as an element of `drawEditorSources`, later consumed by `policyReadFileSafe(root, file)`. It must change from `D0/🦀️component.rs` to `D1/🦀️.rs`.

This is outside `typescriptTokens`: the literal is neither an import nor a direct path-like variable assignment nor an argument of a recognized read/path function. Unlike the wildcard gaps, `unsupportedReferenceTokens` can resolve this exact source and should emit `reference-syntax-unsupported`; therefore the planner fails closed rather than silently missing it.

## Adjacent uncounted executable gap

`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🎬️interaction-spec/🦀️component.rs:638` contains:

```text
Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🏗️modelDefinitions")
```

Rust reference parsing has no `Path::join` adapter. The string resolves exactly to `C0`, so the unsupported scanner should fail closed. The expected relative destination from `🎬️interaction-spec` is `../📚️examples/🪆️1-any/🏗️models`. This occurrence must be repaired atomically despite not being part of the 76-count anchor.

## Required adapter closure

The production implementation should remain schema-owned and fail closed:

1. Add exact `import.meta.glob` structured token support that parses a string or string-array first argument, resolves its non-wildcard owner prefix through the selected projection contract, and renders destination category grammar. Do not perform global substring replacement. Require every nonempty source selector to map to at least one exact projected pair; explicitly delete or separately authorize the three zero-match selectors.
2. Add exact Nx `{workspaceRoot}/…` wildcard token support in project JSON. Match the placeholder literally, resolve the longest non-wildcard directory prefix, and preserve the glob suffix.
3. Add a schema-owned CAD/Draw prose/fragment structural adapter only for the selected projection contracts and known reference locations/forms. Arbitrary comment/path-fragment rewriting would be unsafe.
4. Add Rust `Path::new(…).join("…")` recognition for literal join arguments, or replace this call with an already-supported exact construct. The adapter must resolve the full literal relative to the reference file.
5. Add TypeScript support for strings passed indirectly through a proven path collection/sink pair such as `drawEditorSources → policyReadFileSafe`, or refactor the value into a directly recognized exact path binding. Preserve fail-closed detection for unsupported exact paths.

## Acceptance checks

- Assert the frozen partition `CAD 61/0/0/15`, `Draw 20/0/0/3`, total `81/0/0/18` in the four classification buckets.
- Assert the adjacent line-638 root token is independently detected and rewritten, without changing the requested 76 CAD census.
- For every one of the 61 CAD exact literals, resolve the preimage to the exact golden `sourcePath` and the edit target to its exact `destinationPath`; all 61 must be accounted for, including duplicates.
- For every Draw exact token, resolve the `D0` source file/directory and its `D1` destination; verify moving Nx manifests use their final destination path for edit rendering.
- Assert the two Nx named-input wildcards and 12 CAD globs are no longer silently skipped. Negative tests must reject unregistered owner prefixes, ambiguous projection owners, escaped placeholders, and unmatched nonempty selectors.
- Assert the three zero-source CAD selectors are deleted or backed by an explicit independent authority; they must not be made valid through fallback globs.
- Assert both authoritative comments and the spatial parser marker contain no old `modelDefinitions` grammar after apply.
- Assert `📜️script.ts:7799` and the Rust `Path::join` root produce structured edits, not `reference-syntax-unsupported` violations.
- Run the full 220-pair CAD/Draw golden, normalization dry plan, reference-preimage hashes, Cargo metadata, Nx project discovery, Rust compile/check for both Draw packages and CAD interaction consumers, and Vite glob discovery in one atomic acceptance boundary.

## Commands and evidence

All filesystem scans used explicit production paths or `rg --files -g '!compose/**' -g '!temp/compose/**'`; neither forbidden tree was traversed or read.

```text
rg -n '🖼️assets/🏗️modelDefinitions/' <four exact CAD reference files>
rg -n 'canvas-pointer-down|🔄️fsm|✨️macros' <seven exact Draw reference files>
rg -n 'function (jsonTokens|tomlTokens|rustTokens|typescriptTokens|embeddedArgumentTokens|resolveReferencePath|unsupportedReferenceTokens|buildReferenceEdits)' <normalization component>
sed -n '1830,2075p;2120,2265p;2301,2388p' <normalization component>
bun -e <resolve every include_str literal against the 209-pair golden and print line/source-suffix/destination-suffix>
jq <projection mapping and category census> <CAD/Draw golden>
shasum -a 256 <normalization component> <CAD/Draw golden>
```

Observed reconciliation:

```json
{
  "cad": { "supported": 61, "projectionOwned": 0, "irrelevant": 0, "gaps": 15, "total": 76 },
  "draw": { "supported": 20, "projectionOwned": 0, "irrelevant": 0, "gaps": 3, "total": 23 },
  "combined": { "supported": 81, "projectionOwned": 0, "irrelevant": 0, "gaps": 18, "total": 99 },
  "adjacentUncountedExecutableGaps": 1
}
```
