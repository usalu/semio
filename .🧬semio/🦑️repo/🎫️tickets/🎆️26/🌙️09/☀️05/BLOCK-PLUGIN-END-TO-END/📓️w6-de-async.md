# 🧯️ W6 — De-async `semio-s-plugin-block` back to the framework/peer convention

## 🎯️ Problem

`✏️s/🔌️plugins/🧱️block/**/🦀️.rs` carried **2125 `async fn`** — the fallout of a blanket
"make everything async" codemod that block alone received. Every framework trait block implements is
**sync**, so 214 `async fn diff`, 214 `async fn inverse`, 47 `async fn handle`, the whole `OpText`/
`OpBinary` codec surface and hundreds of free constructors were `E0053` (method signature mismatch)
or produced un-awaited `Future`s inside otherwise-sync bodies.

Measured evidence before the fix:

| metric | value |
|---|---|
| `async fn` in block | **2125** |
| distinct `async fn` names | 382 |
| `.await` sites in the whole crate | **21** (all in the three `🚪️io/🦀️.rs` test modules) |
| non-test `async fn` | **1173** |
| test `async fn` (`#[semio_framework_async_macros::async_test]`) | 952 |
| `async fn diff` in `🧩️puzzle` / `🌀️procedural` / `🗄️stdio` | **0** |

The 21-vs-1173 ratio is the proof the codemod was mechanical: essentially nothing in block ever
awaited anything, so no async body was load-bearing.

## 🧭️ Decision rule

Applied per distinct name, in this precedence order:

1. **Framework trait method** → follow the trait declaration verbatim.
2. **Free / inherent fn** → follow the peer plugins (`🧩️puzzle`, `🌀️procedural`, `🗄️stdio`),
   counting only their **non-test** declarations.
3. **Test fn** → left untouched (see "Tests" below).

### Authoritative trait readings

| trait | method(s) | file:line | async? |
|---|---|---|---|
| `protocol::Mutation` | `diff`, `inverse` | `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:154-155` | **sync** |
| `protocol::MutationDiff` | `apply`, `absorb` | `…/🎮️mutation/🦀️.rs:98+` | **sync** |
| `protocol::OpText` | `print_op`, `parse_op` | `…/🎮️mutation/🦀️.rs:1257-1260` | **sync** |
| `protocol::OpBinary` | `encode_op`, `decode_op` | `…/🎮️mutation/🦀️.rs:1266-1269` | **sync** |
| `store::ArtifactDsl` | `parse_dsl`, `print_dsl` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:4891` | **sync** |
| `store::ArtifactPack` | `encode_pack_with`, `decode_pack_with` | `…/🏪️store/🦀️.rs:9326` | **sync** |
| `plugin::ArtifactEditor` | `initial_snapshot`, `handle`, `render`, `io`, `export_media`, `command_id`, `command_from_action`, `interaction_topology`, `window_measures`, `app_schema` | `…/🔌️plugin/🦀️.rs:26586+` | **sync** (only `command_from_intent` and `media_ports` are async — block implements neither) |
| `plugin::ArtifactViewer` | same surface | `…/🔌️plugin/🦀️.rs:26935+` | **sync** (only `media_ports` async) |
| `plugin::ArtifactComposition` | `reads`, `compose` | `…/🔌️plugin/🦀️.rs:997-999` | **sync** |
| `plugin::ArtifactAnalysis` | `sniff`, `analyze` | `…/🔌️plugin/🦀️.rs:981+` | **sync** |
| `plugin::ArtifactInferrer` | `infer` | `…/🔌️plugin/🦀️.rs:1258` | **sync** (`infer_cached` is async — not implemented by block) |
| `protocol::Inference` | `infer` | `…/📡️spr/🎮️command/🦀️.rs:33` | **sync** |
| `protocol::InferenceSpec` | `inference_schema_id`, `schema_version`, `fields` | `…/📡️spr/🎮️command/🦀️.rs:99+` | **sync** |
| **`io::Serializer`** | `serialize` | `🧰️framework/🔨️modules/🚪️io/🦀️.rs:2376` | **ASYNC** (`-> impl Future<…> + Send`) |
| **`io::Deserializer`** | `deserialize`, `sniff` | `🧰️framework/🔨️modules/🚪️io/🦀️.rs:2391,2394` | **ASYNC** (`-> impl Future<…> + Send`) |

The io traits are the only async ones block touches, and they are exactly the 21 `.await` sites —
so the io surface was already correct and was left alone.

## 📋️ NAME table (non-test occurrences only)

| async fn NAME | block count (non-test) | decision | oracle |
|---|---|---|---|
| `diff` | 214 | → sync | trait `protocol::Mutation::diff` at 📡️replication/🎮️mutation/🦀️.rs:154 — sync; peers async=0 sync=1319 |
| `inverse` | 214 | → sync | trait `protocol::Mutation::inverse` at 📡️replication/🎮️mutation/🦀️.rs:155 — sync; peers async=0 sync=1514 |
| `label` | 111 | → sync | free/inherent fn, no async caller; peers async=0 sync=1138 |
| `target` | 72 | → sync | free/inherent fn, no async caller; peers async=0 sync=1034 |
| `handle` | 47 | → sync | trait `ArtifactEditor::handle` / `ArtifactViewer::handle` at 🔌️plugin/🦀️.rs:26586+/26935+ — sync; peers async=0 sync=241 |
| `definition` | 22 | → sync | free/inherent fn, no async caller; peers async=0 sync=423 |
| `render` | 22 | → sync | trait `ArtifactEditor::render` / `ArtifactViewer::render` at 🔌️plugin/🦀️.rs — sync; peers async=2 sync=406 |
| `deserialize` | 18 | **stay async** | `io::Serializer::serialize` / `Deserializer::deserialize` return `impl Future` (🚪️io/🦀️.rs:2376,2394) |
| `serialize` | 18 | **stay async** | `io::Serializer::serialize` / `Deserializer::deserialize` return `impl Future` (🚪️io/🦀️.rs:2376,2394) |
| `encode_op` | 15 | → sync | trait `protocol::OpBinary::encode_op` at 📡️replication/🎮️mutation/🦀️.rs:1268 — sync; peers async=0 sync=268 |
| `decode_op` | 15 | → sync | trait `protocol::OpBinary::decode_op` at 📡️replication/🎮️mutation/🦀️.rs:1269 — sync; peers async=0 sync=268 |
| `sniff` | 12 | **split**: 9 stay async (`impl Deserializer`), 3 → sync (`impl ArtifactAnalysis`) | `Deserializer::sniff` → `impl Future` (🚪️io/🦀️.rs:2391); `ArtifactAnalysis::sniff` sync (🔌️plugin/🦀️.rs:981) |
| `absorb` | 9 | → sync | trait `protocol::MutationDiff::absorb` at 📡️replication/🎮️mutation/🦀️.rs:98+ — sync; peers async=0 sync=204 |
| `parse_op` | 9 | → sync | trait `protocol::OpText::parse_op` at 📡️replication/🎮️mutation/🦀️.rs:1260 — sync; peers async=0 sync=106 |
| `print_op` | 9 | → sync | trait `protocol::OpText::print_op` at 📡️replication/🎮️mutation/🦀️.rs:1259 — sync; peers async=0 sync=106 |
| `from_snapshot` | 6 | → sync | free/inherent fn, no async caller; peers async=0 sync=156 |
| `apply` | 6 | → sync | trait `protocol::MutationDiff::apply` at 📡️replication/🎮️mutation/🦀️.rs:98+ — sync; peers async=0 sync=321 |
| `initial_snapshot` | 6 | → sync | trait `ArtifactEditor::initial_snapshot` / viewer at 🔌️plugin/🦀️.rs — sync; peers async=0 sync=188 |
| `layout` | 6 | → sync | free/inherent fn, no async caller; peers async=0 sync=190 |
| `replace_document_operations` | 6 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `measure` | 5 | → sync | free/inherent fn, no async caller; peers async=0 sync=12 |
| `reads` | 3 | → sync | trait `ArtifactComposition::reads` at 🔌️plugin/🦀️.rs:997 — sync; peers async=0 sync=104 |
| `compose` | 3 | → sync | trait `ArtifactComposition::compose` at 🔌️plugin/🦀️.rs:999 — sync; peers async=0 sync=130 |
| `to_snapshot` | 3 | → sync | free/inherent fn, no async caller; peers async=1 sync=62 |
| `set_snapshot` | 3 | → sync | free/inherent fn, no async caller; peers async=1 sync=61 |
| `empty` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=98 |
| `from_text` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=93 |
| `from_binary` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=92 |
| `mutate` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=200 |
| `build` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=103 |
| `analyze` | 3 | → sync | trait `ArtifactAnalysis::analyze` at 🔌️plugin/🦀️.rs:981+ — sync; peers async=0 sync=92 |
| `next_id` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=3 |
| `encode` | 3 | → sync | trait `store::ArtifactPack`-adjacent codec at peers sync 150×; peers async=0 sync=150 |
| `decode` | 3 | → sync | trait `store::ArtifactPack`-adjacent codec at peers sync 159×; peers async=0 sync=159 |
| `round_trip` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=81 |
| `remove_author` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `add_attribute` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `add_author` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `remove_compatibility_rule` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `remove_attribute` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_meta_description` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `add_compatibility_rule` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `apply_identified_delta` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=3 |
| `apply_to_artifact` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=5 |
| `diff_set_compatibility_rule` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `diff_remove_compatibility_rule` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `diff_set_attribute` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `diff_remove_attribute` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `diff_set_snapshot` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=46 |
| `infer` | 3 | → sync | trait `protocol::Inference::infer` at 📡️spr/🎮️command/🦀️.rs:33 — sync; peers async=5 sync=125 |
| `inference_schema_id` | 3 | → sync | trait `protocol::InferenceSpec::inference_schema_id` at 📡️spr/🎮️command/🦀️.rs:99+ — sync; peers async=0 sync=62 |
| `schema_version` | 3 | → sync | trait `protocol::InferenceSpec::schema_version` at 📡️spr/🎮️command/🦀️.rs:99+ — sync; peers async=0 sync=62 |
| `fields` | 3 | → sync | trait `protocol::InferenceSpec::fields` at 📡️spr/🎮️command/🦀️.rs:99+ — sync; peers async=0 sync=63 |
| `io` | 3 | → sync | trait `ArtifactEditor::io` / viewer at 🔌️plugin/🦀️.rs — sync; peers async=0 sync=7 |
| `command_id` | 3 | → sync | trait `ArtifactEditor::command_id` at 🔌️plugin/🦀️.rs — sync; peers async=0 sync=5 |
| `command_from_action` | 3 | → sync | trait `ArtifactEditor::command_from_action` at 🔌️plugin/🦀️.rs — sync; peers async=0 sync=7 |
| `interaction_topology` | 3 | → sync | trait `ArtifactEditor::interaction_topology` at 🔌️plugin/🦀️.rs — sync; peers async=0 sync=4 |
| `export_media` | 3 | → sync | trait `ArtifactEditor::export_media` at 🔌️plugin/🦀️.rs — sync; peers async=0 sync=2 |
| `new_app` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `app_with_registry` | 3 | → sync | free/inherent fn, no async caller; peers async=2 sync=3 |
| `dispatch` | 3 | → sync | free/inherent fn, no async caller; peers async=2 sync=10 |
| `every_command` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=2 |
| `app_schema_descriptor` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=5 |
| `text_field` | 3 | → sync | free/inherent fn, no async caller; peers async=0 sync=6 |
| `seeded_snapshot` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=4 |
| `change_representation_description` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_representation_lod` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `add_representation_tag` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `rename_representation` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `add_representation_attribute` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `remove_representation_attribute` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `create_representation` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `remove_representation_tag` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `move_camera3d` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `scale_camera3d` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_representation_mesh_url` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `delete_representation` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `absorb_col` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `diff_set_representation` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `diff_remove_representation` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `vortex` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=3 |
| `representation_mesh_id` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `vortex_kind_color` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `window_measures` | 2 | → sync | trait `ArtifactEditor::window_measures` at 🔌️plugin/🦀️.rs — sync; peers async=0 sync=12 |
| `hexagonal_cut_concrete_forest_left` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `move_camera2d` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `scale_camera2d` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `grip` | 2 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `app_schema` | 2 | → sync | trait `ArtifactEditor::app_schema` at 🔌️plugin/🦀️.rs — sync; peers async=0 sync=5 |
| `block_one_f64` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `kit_type_from_vortex_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `vortex_kind_extra_from_vortex_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `vortex_kind_from_parts` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `catalog_snapshot_from_vortex_kinds` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `vortex_kind_extra_list_from_vortex_kinds` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `vortex_kinds_from_catalog_and_extra` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `catalog_child_handle` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `vortex_kinds_of_parts` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `vortex_kinds_of` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `set_vortex_kinds_parts` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `set_vortex_kinds` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `validate_vortex_kind_catalog` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `default_arrangement` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `default_spacing` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `default_active_utility` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `for_window` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `empty_block3d_snapshot` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `block3d_boot_snapshot` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `nakagin_capsule` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `apply_block3d_mutation` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `inverse_block3d_mutation` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_object_kind_description` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `rename_vortex_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_object_kind_icon` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_object_kind_label` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_vortex_kind_color` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `resize_vortex` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `create_vortex` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `delete_vortex` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `move_vortex` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_object_kind_unit` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `rename_object_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_vortex_vortex_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_vortex_kind_label` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_object_kind_variant` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_vortex_label` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `delete_vortex_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_vortex_kind_default_cable_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `create_vortex_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `block3d_index_of` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `diff_set_vortex_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `diff_remove_vortex_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `diff_set_vortex` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `diff_remove_vortex` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `resolve_active_mesh_url` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `puzzle3d_catalog_fragment` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `snapshot_with_vortices` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `compute_block3d_bounds` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `meshes_json` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=1 |
| `instances_json` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=1 |
| `vortices_json` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `block3d_resolve_world_body` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `f64_vec3_field` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `window_id_from_args` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `block3d_io` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `block3d_app_manifest_for_testkit` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `main_window_measures` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `block3d_is_de_locale` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `block3d_locale` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `block3d_labels` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `default_brush_radius` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `block3d_window_view` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `block3d_active_utility` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `upsert_window_view_index` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `visible_representations` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `arrangement_offset` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `instance_offset_for_representation` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `effective_camera` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `world_meshes_json` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=3 |
| `world_instances_json` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=80 |
| `block3d_vortex_full_id` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `world_vortices_json` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=1 |
| `world_camera_json` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `world_selection_json` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=1 |
| `world_interaction_json` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=2 |
| `world_hit_to_local_vortex` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `default_vortex_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `resolve_brush_vortex_kind_id` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `empty_block5d_snapshot` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `default_block5d_snapshot` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `apply_block5d_mutation` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `inverse_block5d_mutation` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `rename_part_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `rename_grip_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_part_kind_variant` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `update_part_2d` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `delete_grip_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_grip_kind_color` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `resize_grip_3d` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `delete_grip` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_part_kind_description` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_part_kind_unit` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `move_grip_2d` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `create_grip_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_grip_grip_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `create_grip` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `move_grip_3d` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `update_part_3d` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_grip_kind_label` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_part_kind_label` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_grip_kind_default_rope_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_part_kind_icon` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `block5d_index_of` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `diff_set_grip_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `diff_remove_grip_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `diff_set_grip` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `diff_remove_grip` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `puzzle5d_catalog_fragment` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `snapshot_with_grips` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `compute_block5d_bounds` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `block5d_io` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `block5d_app_manifest_for_testkit` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `block5d_labels` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `empty_block2d_snapshot` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `default_block2d_snapshot` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `apply_block2d_mutation` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `inverse_block2d_mutation` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `delete_handle` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_node_kind_variant` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `move_handle` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_node_kind_unit` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `delete_handle_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `rename_handle_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `create_handle_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `rename_node_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `create_handle` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_node_kind_label` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `update_presentation` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_handle_kind_label` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_node_kind_description` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_node_kind_icon` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_handle_kind_default_wire_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_handle_handle_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `change_handle_kind_color` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `apply_handle_kinds_delta` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `apply_handles_delta` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `apply_compatibility_delta` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `apply_attributes_delta` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `absorb_delta` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `id` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=2 |
| `block2d_index_of` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `diff_set_handle_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `diff_remove_handle_kind` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `diff_set_handle` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `diff_remove_handle` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `puzzle2d_manifest_fragment` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `snapshot_with_handles` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `compute_block2d_bounds` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `block2d_io` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `block2d_app_manifest_for_testkit` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |
| `block2d_labels` | 1 | → sync | free/inherent fn, no async caller; peers async=0 sync=0 |

## 🧪️ Tests — deliberately NOT changed

952 `async fn` are test functions, every one carrying
`#[semio_framework_async_macros::async_test]` (the macro **requires** an `async fn` and rejects a
sync one — `🧰️framework/🔨️modules/⏳️async/✨️macros/🦀️.rs:32`). That is exactly how `🗄️stdio`
declares its own mutation-law tests (`produces_committed_diff`, `inverse_restores_before`, …), i.e.
the convention of the plugin whose migration wave produced this codemod. `🧩️puzzle` still uses plain
`#[test] fn`; both spellings compile, and `#[async_test]` + `async fn` is internally consistent, so
rewriting 952 test signatures would have been churn with zero compile effect and a real risk of
colliding with W1/W2/W4. Verified there is **no** `#[test]` directly above an `async fn` anywhere in
block (the one combination that genuinely cannot compile).

The tests were the *victims* of the codemod, not a second bug: their bodies call `mutation.diff(&base)`
with **no** `.await`, which only type-checks once `diff` is sync again.

## ✂️ What was changed

- **1127 signature lines** rewritten `async fn NAME(` → `fn NAME(` across **435 files**.
  Nothing else was touched: no reformatting, no body edits, no whole-file re-serialisation.
- **45 `async fn` deliberately kept**: 18 `serialize` (`impl Serializer<…>`), 18 `deserialize` and
  9 `sniff` (`impl Deserializer<…>`) in the three `🪆️subsets/✳️any/🚪️io/🦀️.rs`.
- 3 further `sniff` — the ones in `impl ArtifactAnalysis for BlockXdAnalyzerAnalysis` — **were**
  de-asynced: same name, different (sync) trait. This is the only name that split.
- One line was already fixed by a peer between the scan and the apply
  (`block3d_boot_snapshot` in `🧊️3d/…/🧬️schema/📸️snapshot/📝️text/🦀️.rs`) — skipped, not re-applied.

### Call-site fallout: none

`.await` count is **21 before and 21 after** — identical, and all 21 are on `Serializer::serialize` /
`Deserializer::deserialize`, which stayed async. **No `.await` had to be removed and no now-sync
function was left calling an async callee**, so the `AGENTS.md` / `🧰️framework/🔨️modules/⏳️async`
rule (never `block_on` in plugin code; fix the callee, not the call site) never had to be invoked.
Block also contains zero `async move` / `async {}` blocks, zero `impl Future` returns and zero
`BoxFuture`, so there is no residual async plumbing.

### Residual async in block after the fix

