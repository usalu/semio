# 🧪 Wave 5 Report — CAD (`semio-s-plugin-cad`)

Ticket `26/08/08/ARTIFACT-SCHEMA-FACETS`. Plugin `✏️s/🔌️plugins/📐️cad/`. Artifact key `cad`, prefix `Cad`, schema id `s.cad.cad`.

## 1. Fifteen facet leaves

| Facet | Dir | Type |
| --- | --- | --- |
| artifact | `🗿️artifacts/📐️cad/🧬️schema/` (5) | `CadArtifact` |
| snapshot | `🗿️artifacts/📐️cad/📸️snapshot/🧬️schema/` (5) | `CadSnapshot` |
| diff | `🗿️artifacts/📐️cad/🔺️diff/🧬️schema/` (5) | `CadDiff` |

## 2. Field inventory (state classes)

**Persistent** (= `CadSnapshot`):
- `schema`
- `id`
- `objects`
- `buildingObjects`
- `energyObjects`
- `structureClassicObjects`
- `referencesByModelDefinitionId`
- `nodes`
- `shapeGeometry`
- `buildingGeometry`
- `energyGeometry`
- `structureClassicGeometry`
- `activeModelDefinitionId`

**SharedUi**:
- `selectedObjectIds`
- `selectedNodeIds`
- `activeObjectId`
- `componentSelection`
- `selectedReferenceModelDefinitionId`
- `selectedReferenceId`
- `selectedPrimitiveId`
- `selectedPrimitiveKind`
- `activeUtilityId`
- `activeExampleId`

**LocalUi**:
- `selectionMethod`
- `engagementInput`
- `engagementStep`
- `engagementPane`
- `engagementSessionJson`
- `lastFinalizedInteractionId`
- `sunEnabled`
- `sunAzimuth`
- `sunElevation`
- `sunIntensity`
- `sunColor`
- `camera`
- `cameraBuilding`
- `cameraEnergy`
- `cameraStructureClassic`
- `dislocateShape`
- `dislocateBuilding`
- `dislocateEnergy`
- `dislocateStructureClassic`
- `locale`
- `terminology`
- `contributionsJson`

**Preview**:
- `hoveredObjectId`
- `hoveredTargetObjectId`
- `hoveredTargetMode`
- `hoveredTargetId`

**Effect**: none.

## 3. Diff-delta shape

`CadDiff` is a sparse field delta (not a mutation list):

- `artifact: Option<Box<CadArtifact>>` — whole-replacement (renamed from former `scene`)
- optional entry per non-effect artifact field
- object panes / nodes: `CadObjectsDelta` / `CadNodesDelta` (`added`/`removed`/`patched`/`reordered`)
- `referencesByModelDefinitionId: Option<BTreeMap<String, CadReferenceList>>` (key-wise replace; cad never used inner-`None` remove-key)
- optional lists wrapped as `CadStringList` where needed
- `MutationDiff<CadSnapshot>` applies persistent entries; `apply_to_artifact` applies all
- `absorb` merges field-wise; later `artifact` clears everything

`CadReferenceList` is a `type` alias for `Vec<CadReference>` so map scalars agree across the five formats without rewriting call sites.

## 4. Pack + set-snapshot

- `🎒️pack/` moved under `📸️snapshot/🎒️pack/`
- whole-document mutation folder `🎬️set-scene` → `🖼️set-snapshot` (`SetSnapshot` / `snapshot` payload field)
- pack protocol had no `Projection` segment name to rename

## 5. Engine

`CadEngine` owns real `CadArtifact` + cached `CadSnapshot` (`type Artifact = CadArtifact`, never collapsed). `apply` diffs against snapshot, writes snapshot, then `artifact.set_snapshot(...)`.

`register()` calls `register_artifact_schema()` which `include_str!`s all fifteen leaves into `ArtifactSchemaRegistry`.

## 6. Glue convention

Leaf-prefixed + grouping `#[path = "."]` (same as lowpoly pilot / existing cad glue). Nested `snapshot` / `diff` keep `../../`. Diff runtime: `pub use super::schema::*;`.

TypeScript index mirrors: `cad_schema`, `cad_snapshot_schema`, `cad_diff_schema`, pack under snapshot.

## 7. Interaction-spec

Effect serde tag is `mutation`. All on-disk interaction JSON under `🖼️assets/🏗️modelDefinitions/**/🎬️interactions/` had Effect tags renamed `"operation"` → `"mutation"` (Expr `binop`/`fold` `"operation"` fields left alone). Cascading engagement/catalog failures cleared once parse succeeded.

## 8. Gate tails (verbatim)

### cargo check

```
773 +     type Snapshot = CadSnapshot;
    |

warning: unused doc comment
   --> ✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/././../../🎛️apps/📐️cad/🦀️component.rs:908:1
    |
908 | / /// 📐️ B1/WORKFLOWS-END-TO-END-TYPED-PORTS: unit-struct-shaped pure `DocumentApp` — every former
909 | | /// `CadPlayRuntime`/`self.runtime` field now lives in `CadConfig`, written through
910 | | /// `CadConfigMutation`s (real `backwards`, no ad hoc `InverseAction`). `preview_seq` is the sole
911 | | /// surviving interior-mutable field — it backs `gesture_preview`'s never-VCS'd, never-config'd live
912 | | /// rubber-band tick counter, not app state.
    | |_-------------------------------------------^
    |   |
    |   rustdoc does not generate documentation for macro invocations
    |
    = help: to document an item produced by a macro, the macro must produce the documentation as part of its expansion
    = note: `#[warn(unused_doc_comments)]` (part of `#[warn(unused)]`) on by default

warning: `semio-s-plugin-cad` (lib) generated 13 warnings (run `cargo fix --lib -p semio-s-plugin-cad` to apply 12 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.25s
```

### cargo test --lib

```
test apps::cad::component::tests::undo_redo_round_trips_added_object_through_wrapper ... ok
test apps::cad::component::tests::forest_example_uses_per_object_brep_meshes ... ok
test apps::cad::component::tests::renders_world_scene_for_each_pane ... ok

test result: ok. 124 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.26s
```

### policy | rg -i cad

```
```

## 9. Four failure clusters — resolved in-plugin

All three gates green: `cargo check`, `cargo test --lib` **124 / 0**, policy|rg cad empty.

1. **interaction-spec `missing field mutation`** — fixtures still used Effect tag `"operation"`; renamed to `"mutation"` across interaction JSON assets. Cleared parse + engagement/catalog cascade.
2. **`CadConfig` envelope `cadcfg` rejected** — added `#[dsl(id = "cad.config")]` beside the `cadcfg` extension (same `<artifact>.config` form as draw).
3. **stub `🗣️example.dsl.semio`** — replaced 36-byte stub with a real printed `CadSnapshot` DSL from `sample_scene()` so `default_example_dsl_round_trips` passes.
4. **CadCommand wire / SPR goldens + geometry** —
   - SPR/pack protocols: `tag N` → `tag=N`; set-snapshot field `scene` → `snapshot`; SaveSelected/LoadRawRequest wire pins updated.
   - `face_*_sync` now tessellates Face handles (kernel `surface_*` requires Surface, not Face) so classify yields roof/baseplate/walls/windows.
   - `tessellate_geometry_handle` strips zero-area fan triangles from forest solid meshes.

## 10. Could not validate

- Runtime DocumentApp gesture/engagement behaviour beyond unit tests + helper signature update (`DocumentApp::handle` needs `DraftView` + `EngineHandles::empty()`).
- MCP ticket open/close (wave-5 fan-out writes into the existing ticket folder).
