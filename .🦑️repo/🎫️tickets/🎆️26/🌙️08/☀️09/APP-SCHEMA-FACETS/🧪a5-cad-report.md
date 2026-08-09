# 🧪 A5 Cad Report — APP-SCHEMA-FACETS

## Summary

Wave A5 for `📐️cad` / `semio-s-plugin-cad` is complete for the single owner
`✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🎚️config` (`CadConfig` ↔ `CadPresence`).

- Config facet (`🎚️config/🧬️schema`) documents all 30 top-level `CadConfig` fields with `local-ui`,
  including nested helpers (`CadHoverTarget`, `CadComponentSelection`, `CadSunConfig`, `CadCamera`,
  `CadDislocateOptions`) via `$ref` / companion types.
- Presence runtime + schema (`👥️presence`) ships shareable live CAD state (selection, hover,
  component selection, main-pane camera, active utility, engagement step/pane) with
  `CadPresenceMutation::Snapshot` (lowpoly/NoPresence pattern).
- `📦️glue.rs` nests `config { component; schema }` and `presence { component; schema }`.
- `CadPlayApp` binds `type Presence = CadPresence` / `type PresenceMutation = CadPresenceMutation`.

## Files touched

### Created
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🎚️config/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🎚️config/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🎚️config/🧬️schema/🔗️component.graphql`
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🎚️config/🧬️schema/🔣️component.json`
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🎚️config/🧬️schema/🛰️component.proto`
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/👥️presence/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/👥️presence/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/👥️presence/🧬️schema/🔗️component.graphql`
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/👥️presence/🧬️schema/🔣️component.json`
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/👥️presence/🧬️schema/🛰️component.proto`

### Updated
- `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/📐️cad/🎛️apps/📐️cad/🦀️component.rs`

## Gate tails

### 1. Scoped `policyAppSchemaBreaches` (cad)

```
0
```

Scoped cad breaches: **0**.

### 2. `cargo check -p semio-s-plugin-cad`

```
warning: unnecessary qualification
   --> ✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/./././../../🗿️artifacts/📐️cad/⚙️engine/🦀️component.rs:766:21
    |
766 |     type Snapshot = crate::artifacts::cad::CadSnapshot;
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
766 -     type Snapshot = crate::artifacts::cad::CadSnapshot;
766 +     type Snapshot = CadSnapshot;
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

warning: `semio-s-plugin-cad` (lib) generated 9 warnings (run `cargo fix --lib -p semio-s-plugin-cad` to apply 8 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 51.79s
```

### 3. `cargo test -p semio-s-plugin-cad --lib`

```
test artifacts::cad::interaction_spec::tests::interaction_spec_parses_all_energy_and_structure_classic_assets ... ok
test artifacts::cad::interaction_spec::tests::interaction_spec_parses_box_asset ... ok
test artifacts::cad::interaction_spec::tests::interaction_spec_parses_sphere_asset_with_command_finish ... ok
test artifacts::cad::mutations::component::tests::inverse_inverts_every_variant_against_a_populated_scene ... ok
test artifacts::cad::op::tests::cad_mutation_print_op_round_trips_every_variant_as_one_line ... ok
test artifacts::cad::op::tests::optional_field_rows_keep_their_pre_migration_bytes ... ok
test artifacts::cad::snapshot::pack::tests::cad_scene_round_trips_through_pack ... ok
test artifacts::cad::snapshot::pack::tests::cad_scene_with_all_geometry_panes_round_trips_through_pack ... ok
test artifacts::cad::snapshot::pack::tests::command_envelope_round_trip_holds_for_an_applied_operation ... ok
test artifacts::cad::spr::tests::add_object_round_trips_through_store ... ok
test artifacts::cad::spr::tests::cad_projection_defaults ... ok
test artifacts::cad::spr::tests::encode_decode_op_round_trips_a_representative_operation ... ok
test artifacts::cad::spr::tests::set_scene_replaces_projection_and_inverts ... ok
test artifacts::cad::spr::tests::translate_objects_updates_origin ... ok
test artifacts::cad::interaction_spec::tests::every_interaction_asset_on_disk_parses_as_interaction_spec ... ok
test artifacts::cad::engine::interaction::tests::reinforced_concrete_column_interaction_commits_as_cylinder ... ok
test artifacts::cad::engine::interaction::tests::slab_interaction_commits ... ok
test artifacts::cad::engine::interaction::tests::external_wall_interaction_commits_via_generic_from_2_points_and_height ... ok
test artifacts::cad::engine::interaction::tests::sphere_interaction_commits_via_command_finish ... ok
test apps::cad::component::tests::switching_utility_emits_no_operations_and_no_history_entry ... ok
test apps::cad::component::tests::engagement_repeat_last_restarts_the_last_finalized_interaction ... ok
test apps::cad::component::tests::forest_example_uses_per_object_brep_meshes ... ok
test apps::cad::component::tests::renders_world_scene_for_each_pane ... ok

test result: ok. 124 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.26s
```

**124 passed; 0 failed.**

## Unverified

- TS package `📦️index.ts` was not extended with app config/presence schema re-exports (dag/lowpoly
  A5 peers likewise left artifact-only TS exports; rust glue is the load-bearing mount).
- Presence fields are a designed shareable subset of the CAD surface; no runtime path yet feeds
  `CadPresence` into the SPR peer encoder (kernel A3 owns that wiring).
- Nested helper stubs in the config schema rust leaf are documentation shapes (not the live
  `CadConfig` DSL types); fidelity is enforced on top-level field name/optional/cardinality only.
