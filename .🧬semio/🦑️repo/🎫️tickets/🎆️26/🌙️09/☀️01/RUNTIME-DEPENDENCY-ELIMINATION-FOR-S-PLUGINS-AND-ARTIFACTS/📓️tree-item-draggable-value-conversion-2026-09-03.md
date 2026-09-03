# tree_item_with_action_draggable — drag_data serde_json → first-party Value

## Scope
Converted `drag_data: &serde_json::Value` on
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:5796`
(`tree_item_with_action_draggable`) to `&dsl::os_pack::json::Value` (first-party, `dsl` = crate-self
alias for `semio_framework_os_kernel`, which re-exports `pack::json` as `os_pack`). Updated every
caller of this exact signature, and only the code paths that feed it — not a wider serde sweep of
either plugin.

## Files touched (3)
1. `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
   - Line 5796: signature `drag_data: &Value` → `&dsl::os_pack::json::Value`.
   - Lines 5812-5813: the internal `entries.iter()` walk — our `Object::iter()` yields `(&str,
     &Value)` directly (unlike `serde_json::Map`'s `(&String, &Value)`), so `key.as_str()` became
     `*key`, and `key.clone()` (String clone) became `key.to_string()` (str → owned String).
   - Line 5963 (test `tree_item_with_action_draggable_maps_json_object_to_string_drag_data`):
     `&serde_json::json!({...})` → `&dsl::os_pack::json::object([(...)])`.
2. `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs`
   - `use serde_json::json;` → `use dsl::os_pack::json::{object, Value};`
   - The one call site's `drag_data` construction (nested `json!({MIME: json!({"kind":kind}).to_string()})`)
     rewritten with `object([...])` + `Value::String(...)`, preserving the exact same wire shape
     (outer object keyed by the MIME const, string value holding the inner object's JSON text).
   - This was forms' LAST non-test-gated serde reference (ticket baseline: 16 refs, 15 test-gated,
     1 production — this one).
3. `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs`
   - `flow_widget_descriptor` and `flow_widget_drag_json` (the only producers feeding this file's one
     `tree_item_with_action_draggable(...)` call) converted return/param types to
     `dsl::os_pack::json::Value` and their `json!{...}` bodies to `dsl::os_pack::json::object([...])`.
   - `use serde_json::{json, Value};` → `use serde_json::Value;` (the `json!` macro import is now
     unused in this file; `Value` stays — it's still serde_json's own type, used unqualified
     throughout `render()` for parsing the *unrelated* host `catalogue_json()` payload, which is
     out of this ticket's scope and untouched).
   - Flow plugin overall still carries ~147 serde refs elsewhere (schema, mutations, io, config,
     etc.) — those are pre-existing and NOT part of this pass; only the drag-data producer path
     that this framework signature change would otherwise have broken was touched.

## Ref counts (ticket's own verify commands)
- `tree_item_with_action_draggable` grep across framework+s: 14 hits total — framework (4: def,
  converted test, re-export, docstring mention), flow catalogue (4: import + 2 docstring mentions +
  the converted call), forms catalogue (1: import; the call itself doesn't re-mention the name inline),
  puzzle 3d catalogue (2: import + call — untouched, DO NOT TOUCH) and puzzle 2d catalogue (2: import
  + call — untouched, DO NOT TOUCH), plus os-kernel's own re-export line. None of these are stray
  serde-typed signatures anymore outside puzzle's already-first-party callers.
- forms serde grep (`serde_json\|use serde`, non-🧪🏭🔬, non-comment): 12 → 11 (the one production
  `use serde_json::json;` import removed; remaining 11 are all `#[cfg(test)]`-gated per ticket
  baseline).
- flow serde grep: 147 → 147 (unchanged by design — only the drag-data producer functions were
  converted; the file's `render()` still legitimately parses serde_json for the host catalogue JSON,
  well outside this ticket's target).

## Stopped on / left alone
`git diff` on the framework file (`🔌️plugin/🦀️.rs`) shows additional hunks I did NOT write —
`InteractionConfigMutation`'s `#[derive(Serialize, Deserialize)]` removal, its `encode_op`/`decode_op`
bodies switched to `dsl::os_pack::json::to_json_string`/`from_json_str`, and an `effects_to_value`
bridge function removed around line ~30206-30240. These are a concurrent session's edits to the same
file (per CLAUDE.md's standing warning on concurrent editing) — left untouched, not mine, not
reviewed.

Puzzle's two remaining callers (`🧩️puzzle/🗿️artifacts/🧊️3d/…/🛍️catalogue/🦀️.rs`,
`🧩️puzzle/🗿️artifacts/◻️2d/…/🛍️catalogue/🦀️.rs`) still pass `&drag_data` built via puzzle's own
already-first-party `Value` (they were already off serde per the DO-NOT-TOUCH instruction and use
`dsl::os_pack::json::Value` already) — confirmed compatible with the new signature by inspection,
not modified.

## Verification method
No cargo. Re-read every edited region on disk after editing (confirmed via Read/sed of the exact
line ranges). Cross-checked type API (`Object::iter()` item shape, `Value::as_object`/`as_array`/
`Display`, `object()` helper signature) by reading
`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs` directly rather than assuming serde_json parity.
Confirmed the `dsl` extern-crate-self alias exists in all three touched crates' root files
(`extern crate semio_framework_os_kernel as dsl;` for forms and flow; `extern crate self as dsl;`
inside os-kernel's own root for the framework file).

Zero cargo commands run. Zero sub-agents spawned.
