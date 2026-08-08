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

Left structurally alone. Bulk `CadProjection`→`CadSnapshot` rename did not touch that file (git diff empty).

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
    Finished `dev` profile [unoptimized] target(s) in 2.40s
```

### cargo test --lib

```
test apps::cad::component::tests::engagement_starts_box_interaction_session ... FAILED
test apps::cad::component::tests::engagement_input_and_possible_engagements_present ... FAILED
test apps::cad::component::tests::gesture_preview_reflects_the_live_rubber_band_preview_and_clears_on_abort ... FAILED
test apps::cad::component::tests::optional_field_rows_keep_their_pre_migration_bytes ... FAILED
test apps::cad::component::tests::world_pointer_move_updates_live_preview_without_committing_or_emitting_mutations ... FAILED
test apps::cad::config::tests::cad_config_dsl_round_trips_a_populated_record ... FAILED
test apps::cad::config::tests::cad_config_pack_round_trips ... FAILED
test artifacts::cad::dsl::tests::default_example_dsl_round_trips ... FAILED
test artifacts::cad::engine::interaction::tests::box_interaction_commits_after_height ... FAILED
test artifacts::cad::engine::interaction::tests::box_interaction_commits_via_shell_normalized_repl_line ... FAILED
test artifacts::cad::engine::interaction::tests::catalog_includes_json_driven_and_legacy_building_entries ... FAILED
test artifacts::cad::engine::geometry_import::tests::forest_shape_geometry_imports_solid_handle ... FAILED
test artifacts::cad::engine::interaction::tests::box_interaction_default_mode_is_point_and_requires_length_prompt ... FAILED
test artifacts::cad::engine::interaction::tests::external_wall_interaction_commits_via_generic_from_2_points_and_height ... FAILED
test artifacts::cad::engine::interaction::tests::reinforced_concrete_column_interaction_commits_as_cylinder ... FAILED
test artifacts::cad::engine::transformation::tests::derive_from_geometry_classifies_box ... FAILED
test artifacts::cad::engine::interaction::tests::sphere_interaction_commits_via_command_finish ... FAILED
test artifacts::cad::interaction_spec::tests::interaction_spec_guard_evaluates_against_context ... FAILED
test artifacts::cad::engine::interaction::tests::slab_interaction_commits ... FAILED
test artifacts::cad::interaction_spec::tests::interaction_spec_parses_all_energy_and_structure_classic_assets ... FAILED
test artifacts::cad::interaction_spec::tests::interaction_spec_parses_sphere_asset_with_command_finish ... FAILED
test artifacts::cad::interaction_spec::tests::interaction_spec_parses_box_asset ... FAILED
test artifacts::cad::engine::interaction::tests::slab_preview_shows_footprint_point ... FAILED
test artifacts::cad::interaction_spec::tests::every_interaction_asset_on_disk_parses_as_interaction_spec ... FAILED
test result: FAILED. 99 passed; 25 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.34s

    artifacts::cad::interaction_spec::tests::interaction_spec_parses_sphere_asset_with_command_finish

test result: FAILED. 99 passed; 25 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.34s
```

### policy | rg -i cad

```
(empty — no cad matches)
```

## 9. Shared-surface blockers (do not fix in this plugin)

`cargo check` and policy are green for cad. `cargo test --lib` is **99 passed / 25 failed**. Failures cluster outside the facet/diff/mutation core (those pass: inverse round-trip, whole-artifact absorb, pack, spr, op):

1. **`🎬️interaction-spec` JSON parse** — `missing field \`mutation\`` while assets contain zero `"mutation"` keys; `🎬️interaction-spec/🦀️component.rs` has **no diff from this agent**. Cascades into engagement / interaction-engine catalog tests (`interaction_by_id("primitive.box")` None).
2. **`CadConfig` DocumentDsl/Pack envelope** — `envelope id must be plugin.artifact, got cadcfg` (framework preamble rule).
3. **Default `.cad` example fixture** — `📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` is a 36-byte stub (`semio demo.dsl v1`), not a cad document; sample_scene DSL round-trips still pass.
4. **Geometry/transformation golden asserts** — forest solid-handle / typology classify failures; not touched by facet leaves.
5. Possibly **CadCommand wire discriminant drift** for `optional_field_rows_keep_their_pre_migration_bytes` (SaveSelected pin); enum not owned by this facet rename.

## 10. Could not validate

- Full `cargo test --lib` green (blocked by items in §9).
- Runtime DocumentApp gesture/engagement behaviour beyond compile + helper signature update (`DocumentApp::handle` now needs `DraftView` + `EngineHandles::empty()`).
- MCP ticket open/close (brief assumed existing ticket folder; wave-5 fan-out writes into it).
