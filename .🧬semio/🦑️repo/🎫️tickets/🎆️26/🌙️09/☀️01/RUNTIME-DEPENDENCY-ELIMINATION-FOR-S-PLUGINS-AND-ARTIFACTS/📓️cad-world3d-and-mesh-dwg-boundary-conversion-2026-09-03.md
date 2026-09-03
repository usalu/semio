# cad: world3d sun/projection + MeshDwgDocumentImporter boundary conversion

## Signatures converted (framework, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`)

All three took `Option<&Value>` (`Value` = `serde_json::Value` via the module's
`use serde_json::{json, Value};`). Converted to `Option<&store::json::Value>`
(first-party `pack::json::Value`, reached in this module via the crate's
`extern crate semio_framework_os_kernel as store;` alias):

- `world3d_host::apply_world3d_sun_action` (line ~36371)
- `world3d_host::apply_world3d_projection_action` (line ~36654)
- `world3d_host::world3d_projection_action_moves_pose` (line ~36711) — the third
  `world3d_*` sibling a prior agent flagged; confirmed by grepping `Option<&Value>`
  inside `world3d_host`.

Internal bodies: `Value::as_f64`/`Value::as_str` → `store::json::Value::as_f64`/`as_str`.
`.get("key")` calls needed no change (object-key lookup, identical signature on the
first-party type). In-file test block (`apply_action_switches_kind_and_leaves_other_kinds_untouched_for_later_recall`)
updated from `json!({...})` (serde_json macro) to `store::json!({...})` (first-party
macro, `#[macro_export]` in `pack::json`, reachable via the same `store` alias).

Also converted, same ticket, same module (`app` — not `world3d_host`):
- `MeshDwgDocumentImporter` type alias (line ~3525): `fn(&MeshData) -> Result<Value, String>`
  → `Result<dsl::os_pack::json::Value, String>` (bare `Value` there is a *different*,
  outer `use serde_json::Value;` at line 288, so fully-qualified rather than re-aliased).
- `MeshDwgBridgeResult.document` field: `Value` → `dsl::os_pack::json::Value` (direct
  consequence — this field is populated straight from the importer's return value).
- `TwoDSvgDocumentRenderer`/`TwoDSvgExportRequest`/`TwoDSvgExportResult` left untouched
  — out of scope, not named in the ticket, no forced consequence from the above.

## cad production-serde count

`python3 /tmp/prodserde.py ✏️s/🔌️plugins/📐️cad 40`

- Before: **7** (2× `MeshDwgDocumentImporter`-return in `🚪️io/🦀️.rs`, 3× sun-args
  bridge in `🌞️sun/🦀️.rs`, 2× projection-args bridge in `🎥️camera/🦀️.rs`)
- After: **0**

## Bridge pattern used everywhere

Call sites already built a `DslValue` (first-party DSL value) and bridged it to
`serde_json::Value` via `serde_json::Value::from(&dsl_args)` purely to satisfy the old
framework signature. Replaced with the existing first-party bridge function
`{dsl,protocol,store}::os_pack::json::from_dsl_value(&DslValue) -> pack::json::Value`
(equivalently `{dsl,protocol}::json::from_dsl_value`, all aliases of the same
`semio_framework_os_kernel` crate). No `serde_json::Value::from(...)` bridging remains
at any of these call sites.

`cad_document_from_mesh` (`🚪️io/🦀️.rs`) and `generation3d_document_from_mesh`
(procedural, see below): previously `Ok(serde_json::Value::from(protocol::ToValue::to_value(&doc)))`,
now `Ok(protocol::json::from_dsl_value(&protocol::ToValue::to_value(&doc)))`.

## Non-cad callers also updated (forced by the signature change; none are cad, none are puzzle)

Changing the three `world3d_host` functions' shared parameter type breaks every caller
that isn't already `DslValue`-only. Updated the callers reachable outside the
DO-NOT-TOUCH list:

- **procedural** (`✏️s/🔌️plugins/🌀️procedural/...`):
  - `generation3d/.../✏️editor/🦀️.rs`: `generation3d_document_from_mesh` return type +
    body (same bridge pattern as cad's `cad_document_from_mesh`).
  - `.../🎮️commands/🌞️set-sun-azimuth/🦀️.rs`, `.../🌞️set-sun-elevation/🦀️.rs`,
    `.../🌞️set-sun-intensity/🦀️.rs`: `serde_json::json!({...})` → `dsl::json!({...})`
    (first-party macro), dropped the now-unused `use serde_json::json;`.
  - `.../🌞️toggle-sun/🦀️.rs`: unchanged — its call passes `None`, which needs no source
    edit for either `Option<&T>`.
  - These are **new/untracked files** (never committed) — `git diff` shows them as `??`,
    not as tracked diffs.
- **lowpoly** (`✏️s/🔌️plugins/💠️lowpoly/.../🌞️sun/🦀️.rs`): `apply_sun_command`'s
  `args` builder used `dsl::DslValue::object([...]).into()` (implicit `Into<serde_json::Value>`,
  which doesn't exist for the first-party type) → switched to explicit
  `dsl::os_pack::json::from_dsl_value(&dsl::DslValue::object([...]))`.
- **framework test-only fixtures** (`🧰️framework/.../🔌️plugin/🏗️builder/🦀️.rs`,
  `#[cfg(test)] mod plugin_builder_dependency_tests`): `counting_mesh_dwg_importer`/
  `alternate_mesh_dwg_importer` (implement the `MeshDwgDocumentImporter` fn-pointer type)
  and one `assert_eq!(result.document, ...)` — updated to the first-party type/macro so
  the fn-pointer type still matches. These are stripped by `prodserde.py` either way
  (test module), so this doesn't move any plugin's counted number.

Neither procedural nor lowpoly reaches 0 production serde refs — both have large,
unrelated pre-existing serde surfaces (119 and 77 refs respectively, mostly binary
snapshot/mutation codecs) that this ticket's target (cad → 0) does not cover. Only the
exact call sites forced by the three converted framework signatures were touched there;
procedural's count dropped 119 → 115 (the four `serde_json::` refs removed: one
`Value::from` + three `json!` macro calls). Lowpoly's count is unchanged (77) because
its prior code used `.into()`, which `prodserde.py`'s grep-based counter never counted
as a `serde_json::` reference in the first place — the fix was still required for the
type to line up with the new signature.

## Left untouched — 🧩️puzzle (DO NOT TOUCH)

Five files call the converted `world3d_host` functions and were **not** edited, per
explicit instruction:

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/.../✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/.../✏️editor/🎮️commands/☀️apply-sun/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/.../✏️editor/🎮️commands/🎥️set-projection/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/.../✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/.../✏️editor/🎮️commands/☀️apply-sun/🦀️.rs`

All five already pass `serde_json::Value`-shaped args (`bridged.as_ref()`/`sun_args.as_ref()`
built via a comment-documented "framework helper still bound to serde_json::Value"
pattern) — they will not compile against the new signature until whoever owns puzzle
(two sessions reportedly active there) converts them the same way. Flagging this
explicitly rather than silently leaving it implied.

## Hunks observed but not written by me

`git diff` on `🧰️framework/.../🔌️plugin/🦀️.rs` shows several unrelated hunks already
present before I started (concurrent session): `tree_item_with_action_draggable`'s
signature/body, `InteractionConfigMutation`'s serde derive removal + op codec rewrite,
and the `effects_to_value`/`requested_effects` `ToValue` derive change. Left exactly as
found, not touched, not reverted.

## Verification without compiling

Re-read every edited region on disk after editing; re-ran `python3 /tmp/prodserde.py`
before/after on cad (7→0), procedural (119→115), lowpoly (77→77, expected per above).
No `cargo` command was run at any point.
