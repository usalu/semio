# serde_json Removal — Lowpoly Editor Slice (7 files)

Pattern: `dsl` = `semio_framework_os_kernel` (extern-crate alias in the plugin root). Built JSON
objects directly as `dsl::DslValue::object([...])`/`DslValue::float`/`uint`/`Bool`/`String` instead
of `serde_json::json!`; produced/parsed JSON **text** via `dsl::json::to_json_string`/`from_json_str`
(the `pack::json` bridge, zero serde_json) instead of `serde_json::to_string`/`from_str`.

**🧭️view/🦀️.rs**: removed `use serde_json::Value`. `mesh_select_action` now uses
`dsl::json::to_json_string` (was fallible `serde_json::to_string`, now infallible). `utility_params_value`
now parses via `dsl::json::from_json_str::<DslValue>` bridged to `serde_json::Value` at the return
boundary — signature kept as `serde_json::Value` (fully qualified) since 5+ files outside this
ticket's ownership (`🎮️commands/🔷️mesh-edit`, `🖌️session`, `🛠️options/🧲️snap`/paint-params-*) still
consume that exact type; changing it would ripple beyond the 11-file split.

**📌️panels/🔍️inspection/🦀️.rs**: removed the `use`; one `&Value` param fully qualified to
`&serde_json::Value` (unavoidable, same cross-file reason as above).

**🛠️options/🗂️select/🦀️.rs**: both `json!({...})` call sites rebuilt as `DslValue::object([...])`.

**🎭️modes/🌐️model/🦀️.rs** and **🎭️modes/🖼️uv/🦀️.rs**: fully converted — zero serde_json left, no
fully-qualified references either. All scene JSON (`world_selection_json_for`, `world_meshes_json`,
`world_instances_json`, `uv_canvas_layers_json`) now builds `DslValue` and emits via
`dsl::json::to_json_string`. Note: model.rs's old `mesh_data_from_transfer(...)` call sat directly
inside a `json!()` macro, which requires `Serialize` — `MeshData`'s `Serialize` is now `#[cfg(test)]`-
only (ticket 26/09/01), so this file would not have compiled as-is; the fix was necessary, not optional.

**⚙️engine/🦀️.rs**: removed `use`, the `LowpolyCoreError::Json(serde_json::Error)` variant, its
Display/source arms, and `impl From<serde_json::Error>` — all unused once `tessellate_transfer_json`/
`tessellate_all_json` stopped calling fallible `serde_json::to_string`/`json!`. `tessellate_transfer_json`
builds a `DslValue` then bridges once to `serde_json::Value` at return: signature kept as
`serde_json::Value` since the peer-owned `mesh_data_from_transfer` (`🧬️schema/🦀️.rs`) and model.rs's
tessellation-export path both consume that exact type (confirmed via the peer's own concurrent
docstring on `mesh_data_from_transfer`, which independently states the same constraint).

**editor/🦀️.rs**: removed the `use`; `lowpoly_window_action`'s `args: Option<Value>` kept as
`Option<serde_json::Value>` fully qualified — genuinely forced by the framework API
`world3d_sun_measures(..., action: impl Fn(&str, Option<Value>) -> ActionDescriptor)`
(`🔌️plugin/🦀️.rs`), called by name at `🛠️options/🌞️sun/🦀️.rs` (outside this ticket). Both
`json!({"key": key})` call sites and both mesh export/import JSON conversions now route through
`DslValue`/`dsl::json`.

All `#[cfg(test)]` oracle usage (round-trips, fixture comparisons) left untouched; two test-only
`Vec<Value>` bindings fully qualified to `Vec<serde_json::Value>` since their file's `use` was removed.

**Breach count**: `bun ./📜️script.ts test | grep 💠️lowpoly | grep testing/dependency` — 11 → 0 lines
(all 7 owned files gone; confirmed by a completed full run, not a partial one).

**Not verified**: `cargo check -p semio-s-plugin-lowpoly` did not complete in this session — stuck
"Blocking waiting for file lock on build directory" for 10+ minutes behind heavy concurrent lock
contention (20+ other cargo processes system-wide). Every API used (`DslValue::object/float/uint/Bool`,
`dsl::ToValue::to_value`, `dsl::FromValue::from_value`, `dsl::json::to_json_string/from_json_str`,
the `DslValue`↔`serde_json::Value` `From` impls) was confirmed to exist with the exact signature used
by reading `🌱️value/🦀️.rs` and `🎒️pack/🔤️json/🦀️.rs` directly, and every cross-file boundary type
decision was cross-checked against the peer's own already-migrated `🧬️schema/🦀️.rs`, which documents
the identical constraints independently.

## Follow-up: `--all-targets` verification (isolated CARGO_TARGET_DIR, full run to completion)

`cargo check -p semio-s-plugin-lowpoly --lib`: **0 errors** (coordinator-confirmed).
`cargo check -p semio-s-plugin-lowpoly --all-targets`: **39 errors, 0 in my 7 files.**

All 39 are `E0277: the trait bound ...: serde::Serialize`/`Deserialize` is not satisfied`, on
`LowpolyObject`/`LowpolyObjectPatch`/`CreateMesh`, in: `🖌️session/🦀️.rs` (1), `🧬️schema/🦀️.rs`,
`🧬️schema/📸️snapshot/🦀️.rs`, `🧬️schema/🔺️diff/🦀️.rs`, `🧬️schema/🧬️mutations/🦀️.rs`, and ~20
`🧬️schema/🧬️mutations/*/🧪️tests/*/🦀️.rs` fixture files — none of my seven files appear anywhere in
the error list, and grepping the full log for `DslValue`/`dsl::json`/`to_json_string`/`from_json_str`
finds zero matches. Root cause, confirmed by reading the source directly: `🗿️artifacts/💠️lowpoly/🦀️.rs`
(a file neither agent owns) defines `LowpolyObject`/`LowpolyObjectPatch` with
`#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]` — no
`Serialize`/`Deserialize`, not even `#[cfg_attr(test, ...)]`-gated. Test/fixture code elsewhere still
calls `serde_json::to_string`/`json!` on these types directly, which cannot compile regardless of
serde_json vs DslValue in my 7 files. Not introduced by my pass; out of scope for me to fix (would
mean editing `🗿️artifacts/💠️lowpoly/🦀️.rs` and the peer's `🧬️schema/` tree). Left untouched.
