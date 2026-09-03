# `Menu::action_args` serde_json::Value → DslValue

## Signature converted

Yes. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12215` (real crate root is the 21-line
stub at `.../📦️packages/🦀️rust/🦀️.rs` which `#[path]`-includes this file as `mod component`):

```rust
// before
pub async fn action_args(self, action_id: impl Into<String>, args: Value) -> Self {
    self.action_with_args(action_id, Some(DslValue::from(&args))).await
}
// after
pub async fn action_args(self, action_id: impl Into<String>, args: DslValue) -> Self {
    self.action_with_args(action_id, Some(args)).await
}
```

`Value` there was `serde_json::Value` (module-scope `use serde_json::Value;` at line 288, same
`pub mod app { }` block `Menu` lives in). `DslValue` is `protocol::value::DslValue` (the
`🌱️value` module's enum, reached in this file via `use dsl::{to_dsl_value, DslValue};` at line 247)
— **not** the same type as `pack::json::Value`/`dsl::os_pack::json::Value`. Deletion-not-translation
applied: the old body bridged forward via `DslValue::from(&args)`; the new body just forwards `args`.

## Caller survey — only two real callers existed

`grep -rn "\.action_args("` matches ~30 plugin files, but almost all of them call a *different*
method with the same name: `ActionDefinition::action_args(id, Vec<ActionArgDef>)` (the declared-args
builder, e.g. fem/remodel/architect/lowpoly/puzzle/sourcing — confirmed by reading each call site's
second argument). Those are untouched; they never depended on `Menu`'s `Value` param and are
unaffected by this signature.

The **only** production call sites of `Menu::action_args(id, Value)` (via `Menu::of(registry)` builder
chains) in the whole repo:

- `✏️s/🔌️plugins/🕸️dag/…/✏️editor/🦀️.rs:152,157` (`dag_context_menu_items`)
- `✏️s/🔌️plugins/🌍️gis/…/✏️editor/🦀️.rs:556,558,561` (`gis2d_context_menu_items` +
  `select_feature_action_args` helper)

Both converted to build `dsl::DslValue` directly via `dsl::DslValue::object([...])` /
`dsl::DslValue::String(...)` — no `serde_json::json!`/`Value` bridge. `dsl` is already
`extern crate semio_framework_os_kernel as dsl;` at both plugins' crate roots (pre-existing, not
added by this pass). GIS's `select_feature_action_args` still needs one JSON-text sub-field
(`"targets"`, a stringified array per the existing wire contract `parse_interaction_targets` reads
in the framework) — built with `dsl::os_pack::json!([...]).to_string()` (the first-party
`pack::json::Value` literal macro + its `Display` impl), then wrapped as `DslValue::String(...)`. No
`serde_json` involved anywhere in either conversion.

## Per-plugin ref counts (`python3 /tmp/prodserde.py <plugin-dir>`)

- **dag**: 5 → **0** (all 5 were the two `action_args` call sites plus the `use serde_json::Value;`
  import; a concurrent session removed that now-dead import in the same working tree — see below).
- **gis**: 152 → **152** (unchanged at the plugin level). The three GIS `action_args` call sites used
  *unqualified* `json!`/`Value` (imported via `use serde_json::{json, Value};`), which
  `prodserde.py`'s `serde_json`-substring regex never counted in the first place — so the count
  couldn't move on this metric. The plugin's `editor/🦀️.rs` file still legitimately needs
  `serde_json` for unrelated code (lines 61, 759-760, 1014, 1217 — a bare `Option<Value>` command-arg
  type, `serde_json::to_vec`/`json!` in a `#[cfg(test)]` block). Those are out of this ticket's named
  scope (only `action_args` was authorized) and were left alone.

## `git diff --name-only` (files this pass touched)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (the `action_args` signature, 2 lines)
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

## Hunks in those files NOT written by this pass (concurrent sessions — left untouched)

- Framework plugin file: large unrelated diff already present before/alongside this edit —
  `MeshDwgDocumentImporter`/`MeshDwgBridgeResult` (`Value` → `dsl::os_pack::json::Value`),
  `tree_item_with_action_draggable` (`&Value` → `&dsl::os_pack::json::Value` + key-iteration fix),
  its test fixture, `InteractionConfigMutation` (dropped `Serialize`/`Deserialize` derive, routed
  `OpText`/`OpBinary` through `dsl::os_pack::json::to_json_string`/`from_json_str`), the
  `plugin_runtime` `effects_to_value` bridge removal (`Effect` now derives `ToValue` directly), and
  `world3d_host::apply_world3d_sun_action` (`Option<&Value>` → `Option<&store::json::Value>`). All at
  file offsets far from `Menu::action_args` (line 12215) — verified via `git diff` that only that one
  2-line hunk is mine.
- `✏️s/🔌️plugins/🕸️dag/…/✏️editor/🦀️.rs`: removal of the (now provably dead, post-conversion) top-level
  `use serde_json::Value;` import — not made by this pass, but confirmed correct: no bare `Value`
  reference remains in the file after the `action_args` conversion.
- `✏️s/🔌️plugins/🌍️gis/…/✏️editor/🦀️.rs`: one doc-comment emoji fix (`◻2d` → `◻️2d`, unrelated line 211).

## Puzzle sites

None — puzzle's `.action_args(` calls are all the `ActionDefinition::action_args(id, Vec<ActionArgDef>)`
builder, never `Menu::action_args(id, Value)`. Nothing to report for another session there.

## Confirmations

- Zero `cargo` commands run.
- Zero sub-agents spawned (`Agent` tool never called).
