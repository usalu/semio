Vcs And Dag Serde Elimination (2026-09-03)

## Scope
Targets: `✏️s/🔌️plugins/🌿️vcs` and `✏️s/🔌️plugins/🕸️dag`.
Measured with `python3 /tmp/prodserde.py <plugin> <n>` (strips `#[cfg(test)]` blocks/items,
`cfg_attr(test`, comments — counts only TRUE production refs).

## Before/after

| plugin | before | after |
|---|---|---|
| 🌿️vcs | 60 | **0** |
| 🕸️dag | 60 | **5** (all reported blockers below, not bridged) |

## Pattern applied (mirrors 🧱️block/📖️playbook/🪵️sourcing templates)
- Schema/mutation/config/presence structs: `derive(..., dsl::ToValue, dsl::FromValue, ...)` +
  `#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]`; `#[value(rename_all = "camelCase"[, default])]`
  + `#[cfg_attr(test, serde(rename_all = "camelCase"[, default]))]`. `dsl::DslRecord`/`DslArtifact`/
  `DslEnum`/`Mutations`/`DslOps` coexist fine with `ToValue`/`FromValue` in the same derive list
  (confirmed against 🧱️block precedent).
- `#[serde(default)]` field attr -> `#[value(default)]` (value-derive supports it, confirmed in
  `🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs` doc).
- JSON bridge functions: `serde_json::to_string`/`to_value` -> `dsl::json::to_json_string`
  (infallible — deleted every `.unwrap_or_else`/`.expect("...serialization is infallible")` fallback
  wrapping it); `serde_json::from_str`/`from_value` -> `dsl::json::from_json_str` (fallible, kept
  existing `.map_err`/`.unwrap_or_else` fallback shapes since decode genuinely can fail).
- csv/xlsx/zip lossy stub serializers/deserializers: replaced the
  `serde_json::to_value(from)` -> `serde_json::from_value(...)` double-hop with a direct
  `<Target>::from_value(from.to_value())` (both `VcsSnapshot`/`DagSnapshot` and the stdio
  `CsvSnapshot`/`XlsxSnapshot`/`ZipSnapshot` targets already derive `value_derive::ToValue`/
  `FromValue`).
- json rfc8259 export/import: switched to `JsonSnapshot::from_value(dsl::json::from_dsl_value(&x.to_value()))`
  / `write_json_pretty` (export) and `parse_json_text` + `X::from_value(dsl::json::to_dsl_value(&from.to_pack_value()))`
  (import), matching 🪵️sourcing's already-converted `curation` leaf exactly.
- UI action-arg literals (`serde_json::json!({...})` fed into a plugin-local `foo_action(...)`):
  replaced with the plugin's own `ui_value_map([("key", ui_value_text(...)/ui_value_list(...))])`
  helpers (already defined per-plugin in each `✏️editor/🦀️.rs`).
- `command_from_action`'s `Option<&Value>` param: `use serde_json::Value` -> `use dsl::os_pack::json::Value`
  (mirrors 🧱️block's editor root exactly — `Value::get`/`as_str`/`as_array`/`Null` all exist on
  `pack::json::Value` with the same shape). vcs's own test module explicitly re-qualifies its 4
  `let fixture: Value = serde_json::from_str(...)` oracle lines as `serde_json::Value` (mirrors an
  identical explicit qualification already present in 🧱️block's own test at that file's line ~712) so
  the import swap doesn't silently break the test-only serde oracle.
- dag's unused `use serde_json::Value;` in its editor root (no `command_from_action` override exists
  in that file) was simply deleted — dead import, not a bridge.

## REAL before/after counts (re-run today, both 0-cargo)
- 🌿️vcs: 60 -> 0
- 🕸️dag: 60 -> 5

## Framework-signature blockers — reported, NOT bridged (per ticket instruction)
1. **`PropertyBag` (`BTreeMap<String, graph::manifest::PropertyValue>`) has no `ToValue`/`FromValue`
   anywhere.** `graph::manifest::PropertyValue` (`🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs:57`)
   derives neither, and `🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs` has no blanket `BTreeMap` impl
   (only `Vec<T>` at line 218). Blocks 2 refs in
   `🕸️dag/…/🚪️io/🧬️mutations/📝️text/🦀️.rs:127,130` (`ReplaceNodeProperties`/`ConnectNodes`'s
   `new_properties_json`/`properties_json` fields) plus the file's own `properties_json_of` helper at
   line 93 that's forced to stay on `serde_json::to_string`. `DagNodeSpec`/`DagNodeKind` in the SAME
   file (`node_json`/`new_kind_json`) already had hand-written/derived `ToValue`/`FromValue` and were
   converted cleanly — split into a `json_of<T: dsl::ToValue>` (used for those two) vs a dedicated
   `properties_json_of` (documented, still serde) so the fix didn't block the convertible fields.
2. **`semio_framework_plugin::Menu::action_args` takes `args: Value` where `Value` = `serde_json::Value`**
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12215`, via that mod's own
   `use serde_json::Value;` at line 288). Blocks 2 refs in
   `🕸️dag/…/✏️editor/🦀️.rs:152,157` (`addNode`/`disconnect` context-menu action args) — the call
   site has no first-party alternative until that framework signature changes.

## git diff --name-only (both plugins) — mine vs concurrent
Every `.rs` file under `🧬️schema/`, `🚪️io/`, `✏️editor/` I edited matches the list below. Three
categories showed up as modified that I did **not** write (confirmed by `git diff` content, flagged
per ticket instruction rather than silently claimed):
- `🌿️vcs/📦️packages/🦀️rust/🦀️.rs` and `🕸️dag/📦️packages/🦀️rust/🦀️.rs` — a concurrent session's
  `📄txt` -> `📄️txt` directory-emoji rename ripple (two `#[path]` string edits each, unrelated to serde).
- `🕸️dag/…/✏️editor/🎮️commands/🗂️graph-pointer-down/🦀️.rs` — a concurrent one-line doc-comment
  edit (`procedural3d` -> `generation3d`), unrelated to serde.
- `🧪️oracle/🔣️.json`, `🧪️tests/mutate-{vcs,dag}-1/🐍️.py`, `🧪️tests/mutate-{vcs,dag}-1/🥒️.feature`,
  and the `📄txt`->`📄️txt` `🟦️.ts`/`🦀️.rs` renames themselves — all under paths this ticket's
  instructions forbid touching (`🧪️test/🧪️tests/`) or that I never opened; a concurrent session's work.

## Zero cargo, zero sub-agents
No `cargo` command was run at any point. No `Agent`/sub-agent tool was invoked — all edits made
directly with Read/Edit/Write/Bash by this session.
