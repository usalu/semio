# Block + Shooting: TRUE production serde refs driven to 0

Counter used throughout: `python3 /tmp/prodserde.py <plugin>`.

## Before/after

| Plugin | Before | After |
|---|---|---|
| `✏️s/🔌️plugins/🧱️block` | 29 | **0** |
| `✏️s/🔌️plugins/🎥️shooting` | 38 | **0** |

Zero `cargo` commands run. Zero sub-agents spawned. All verification was `python3 /tmp/prodserde.py` re-runs plus manual re-reads of every edited region.

## Destination types used

- `dsl::os_pack::json::{Value, Object, json!, parse, parse_bytes, to_string, to_string_pretty, to_json_string, from_json_str, from_dsl_value, to_dsl_value, array}` — dropped in everywhere a file built/parsed ad-hoc JSON (`serde_json::{json, Value}` imports, `serde_json::to_string`/`from_str`/`from_slice`).
- `semio_framework_plugin::UiValue` via each editor root's local `ui_value_map`/`ui_value_text`/`ui_value_bool` builders — used everywhere an `on_change`/action arg was being built (`*_action("cmd", Some(json!({...})))`). This is NOT the same type as `dsl::os_pack::json::Value`; `UiValue` has no `From<Value>` bridge, so every one of these call sites was a **real, pre-existing type mismatch**, not just a serde smell — `json!(...)` piped into an `Option<UiValue>` parameter would not have compiled either way. Fixed by building `UiValue` directly with the map/text/bool constructors (matching the established pattern already live in `🏛️architect`/`🧩️puzzle`/shooting's own `📌️panels/📄️artifact`).
- Numbers: added local `vec3`/`vec4` closures/fns (`[f64;3]`/`[f64;4]` → `Value` via `.iter().map(Value::from).collect()`) everywhere a coordinate/quaternion field was interpolated into a `json!{...}` literal — `[f64;N]` has no blanket `Into<Value>`, unlike `serde_json`'s.
- `pack::JsonValue` → stdio's own `JsonValue` bridge (`impl From<pack::JsonValue> for JsonValue`, already landed by a peer wave on this same ticket) used in shooting's rfc8259 json serializer leaf — no `serde_json::Value` needed there at all now.

## Fallible UiValue construction in non-Result functions

`ui_value_map`/`ui_value_text` return `UiAssemblyResult<UiValue>` (fixed-capacity admission can fail). Several call sites live in functions that return a bare `UiNode`/`WindowMeasure` (not `Result`) — e.g. block's `text_field`, the block3d/5d/2d inspector panels, block3d's world-window option measures, shooting's inspector panel and scene-window engagement. Rather than re-plumbing `Result` through every render call chain (out of scope — a much bigger, unrelated refactor), these use `.expect("...")` on the single/two-entry map construction, which cannot fail in practice (fixed literal key counts, well under capacity). Flagging this choice explicitly per the "no pragmatism" rule — the alternative (Result-ifying `text_field`/`render` chains across 6+ files) was judged out of scope for a serde-ref-count ticket and risks touching files other sessions may be mid-edit on.

## Test-only serde_json left in place (all inside `#[cfg(test)] mod tests`, correctly excluded by the counter)

- Block: `🧊️3d/✏️editor/🦀️.rs`, `🖐️5d/✏️editor/🦀️.rs`, `◻️2d/✏️editor/🦀️.rs` each had one test doing `let value: Value = serde_json::from_str(&json)...` where `Value` used to alias `serde_json::Value` via the file's top-level import. Since that import now aliases `dsl::os_pack::json::Value` (no serde derive), each was re-qualified to `let value: serde_json::Value = serde_json::from_str(...)` — still a real oracle-style check of the exported JSON text, unchanged behavior, just no longer riding on the production alias.
- Same pattern + same fix in shooting's edit-mode scene window test (`parse` from our own module used instead where the alias would otherwise have broken; `serde_json::to_string(&node)` for `UiNode` left untouched — unrelated framework type, not part of this migration).

## Files touched (this session only — see caveat below)

Block:
- `🗿️artifacts/🧊️3d/🦀️.rs`
- `🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs`
- `🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs`
- `🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs`
- `🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️world/🦀️.rs`
- `🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs`
- `🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌍️world/🦀️.rs`
- `🗿️artifacts/🧊️3d/.../🎚️options/{🧱️representations,↔️arrangement,🔀️quick-representation,📏️spacing,🖌️brush}/🦀️.rs`
- `🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs`
- `🗿️artifacts/◻️2d/🏅️标准s/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs`

Shooting:
- `🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs`
- `🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs`
- `🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🎥️scene/🦀️.rs`
- `🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/📄️artifact/🦀️.rs`
- `🗿️artifacts/🎥️shooting/🏅️标准s/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs`
- `🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🎥️scene/🦀️.rs`
- `🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/{📦️asset,🖨️export,📷️shot,🗃️fixture}/🦀️.rs`

No `Cargo.toml` edited (as instructed) — both plugins keep `serde`/`serde_json` deps in the manifest so the parent session can drop them after compiling.

## ⚠️ Concurrent-session hunks observed in `git diff --name-only` under block/shooting that this session did NOT write

Flagging per "verify without compiling" instructions — these appeared in the diff of the two target trees but were never touched by any tool call in this session:

- `✏️s/🔌️plugins/🎥️shooting/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/mutate-shooting-1/🥒️.feature`
- `✏️s/🔌️plugins/🧱️block/📦️packages/🟦️typescript/🟦️.ts`
- `✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/🦀️.rs`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🟦️.ts`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🦀️.rs`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️标准s/🔖️1/🪆️subsets/✳️any/🦀️.rs` (artifact-root file — distinct from the `✏️editor/🦀️.rs`/`🧬️schema/💡️inferences/🦀️.rs` leaves this session edited)
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️标准s/🔖️1/🪆️subsets/✳️any/🦀️.rs` (same — artifact-root, not any file this session edited)
- ~30 `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️标准s/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/*/🔣️.json` fixture files

These read like a fixture/codegen regeneration pass (or a peer session) touching the same plugins concurrently — did not inspect their content, did not attribute cause, did not touch them.
