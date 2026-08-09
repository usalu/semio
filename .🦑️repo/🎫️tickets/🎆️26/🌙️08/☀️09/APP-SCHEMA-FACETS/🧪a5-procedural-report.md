# A5 Procedural Report

## Summary

Implemented app schema facets for both procedural owners (`Procedural2dConfig` / `Procedural2dPresence`, `Procedural3dConfig` / `Procedural3dPresence`):

- Config `🧬️schema` five-leaf facets matching real `XConfig` fields (`local-ui`), including nested `CameraJson` / `Procedural3dPreviewCamera`.
- Presence runtime (`XPresence` + `XPresenceMutation` Snapshot pattern) plus five-leaf presence schemas (`shared-ui`).
- Wired `📦️glue.rs` as nested `config { component; schema }` / `presence { component; schema }`.
- Replaced `DocumentApp` `NoPresence` bindings with real presence types on `◻2d` and `🧊️3d`.

Scoped policy breaches: **0**. `cargo check` and `cargo test --lib` both green.

## Files touched

### Plugin production
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🎚️config/🧬️schema/🔗️component.graphql`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🎚️config/🧬️schema/🔣️component.json`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🎚️config/🧬️schema/🛰️component.proto`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🎚️config/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🎚️config/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/👥️presence/🧬️schema/🔗️component.graphql`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/👥️presence/🧬️schema/🔣️component.json`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/👥️presence/🧬️schema/🛰️component.proto`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/👥️presence/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/👥️presence/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/◻2d/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎚️config/🧬️schema/🔗️component.graphql`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎚️config/🧬️schema/🔣️component.json`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎚️config/🧬️schema/🛰️component.proto`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎚️config/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🎚️config/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/👥️presence/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/👥️presence/🧬️schema/🔗️component.graphql`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/👥️presence/🧬️schema/🔣️component.json`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/👥️presence/🧬️schema/🛰️component.proto`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/👥️presence/🧬️schema/🟦️component.ts`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/👥️presence/🧬️schema/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🎛️apps/🧊️3d/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📦️glue.rs`

### Ticket
- `gen_a5_procedural.py` (generator)
- `🧪a5-procedural-report.md` (this report)

## Gate tails

### 1. Scoped `policyAppSchemaBreaches` (procedural)

```
0
```

### 2. `cargo check -p semio-s-plugin-procedural`

```
   --> ✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/././../../🗿️artifacts/🧊️procedural3d/⚙️engine/🦀️component.rs:908:21
    |
908 |     type Snapshot = crate::artifacts::procedural3d::Procedural3dSnapshot;
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
help: remove the unnecessary path segments
    |
908 -     type Snapshot = crate::artifacts::procedural3d::Procedural3dSnapshot;
908 +     type Snapshot = Procedural3dSnapshot;
    |

warning: `semio-s-plugin-procedural` (lib) generated 26 warnings (run `cargo fix --lib -p semio-s-plugin-procedural` to apply 26 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 2m 25s
warning: the following packages contain code that will be rejected by a future version of Rust: block v0.1.6
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
```

### 3. `cargo test -p semio-s-plugin-procedural --lib`

```
test artifacts::procedural3d::spr::tests::op_text_round_trip_generation ... ok
test artifacts::procedural3d::spr::tests::op_text_round_trip_remove_layout ... ok
test artifacts::procedural3d::spr::tests::op_text_round_trip_remove_synapse ... ok
test artifacts::procedural3d::snapshot::pack::tests::pack_round_trips_the_hex_column_example ... ok
test artifacts::procedural3d::spr::tests::op_text_round_trip_remove_widget ... ok
test artifacts::procedural3d::spr::tests::op_text_round_trip_set_camera ... ok
test artifacts::procedural3d::spr::tests::op_text_round_trip_set_layout ... ok
test apps::procedural3d::panels::inspection::tests::inspector_shows_no_selection_by_default ... ok
test artifacts::procedural3d::spr::tests::op_text_round_trip_set_schema ... ok
test artifacts::procedural3d::spr::tests::op_text_round_trip_set_widget ... ok
test artifacts::procedural3d::spr::tests::op_text_round_trip_set_synapse ... ok
test artifacts::procedural3d::spr::tests::document_text_round_trip_with_operation_applied ... ok
test artifacts::procedural3d::engine::tests::all_bundled_examples_emit_preview_meshes ... ok
test artifacts::procedural3d::engine::tests::document_from_mesh_returns_valid_default_snapshot ... ok
test artifacts::procedural3d::engine::tests::preview_payload_has_meshes_and_instances ... ok
test artifacts::procedural3d::engine::tests::procedural3d_mesh_bridges_round_trip_through_obj_glb_stl_codecs ... ok
test artifacts::procedural3d::engine::tests::rectangle_wire_preview_emits_edge_only_mesh ... ok
test artifacts::procedural3d::engine::tests::wireframe_show_mode_strips_shaded_triangles ... ok

test result: ok. 193 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 15.89s
```

## Unverified

- End-to-end multiplayer presence pack round-trip in a running host (types compile and pack codecs exist; no live peer session exercised).
- TS package re-exports for the new app schema leaves were not added (prompt required Rust `📦️glue.rs` only; lowpoly TS glue likewise omits them).
- Proto package segments `semio.app.procedural.2d` / `semio.app.procedural.3d` follow owner-slug stripping; not validated by `protoc` (leaves are documentation mirrors, same as lowpoly pilot).
