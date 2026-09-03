🐛️ Puzzle 5d/2d `editor/🎮️commands/**` — `serde_json::Value` → dsl value conversion

## Scope
Converted the 16 `🖐️5d` and 2 `◻️2d` `editor/🎮️commands/*` handler files flagged in the captured
231-line error list (`🖐️5d 214` / `◻️2d 12` buckets; `🧊️3d 17` untouched — owned by another session).
Did **not** run cargo at any point (per instruction); verified every change by re-reading the file
from disk after editing.

## Key finding that reshaped the plan
The task brief said "convert to `dsl::DslValue`", but the actual expected type at every 5d command
call site is `dsl::os_pack::json::Value` (aka `JsonValue`, the in-house `serde_json::Value`
replacement at `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs`) — confirmed both by the compiler's own
"expected `JsonValue`, found `Value`" messages and by the already-fixed `🧊️3d` reference handlers
(`use dsl::os_pack::json::Value;`). `puzzle5d`'s `command_from_action` converts the trait's
`Option<&DslValue>` to `Option<JsonValue>` via `dsl::json::from_dsl_value` before dispatch, so
handlers work in `JsonValue` space, not raw `DslValue`, except where a field/mutation genuinely
types itself as `DslValue` (e.g. `Puzzle2dPlayRuntime.brush_candidates: Vec<dsl::DslValue>`).

Second finding: `◻️2d`'s own `editor/🦀️.rs` dispatcher (unlike 5d's) is **not yet migrated** — it
still does `use serde_json::{json, Value};` and bridges via the forbidden
`args.map(Value::from)` (`DslValue → serde_json::Value`) at its `command_from_action`. So for the
two 2d command files, `args: Option<&Value>` genuinely means `serde_json::Value` still (matches
what the unmigrated dispatcher passes) — changing that import would have broken the whole file
against `Puzzle2dPlaySnapshot(pub Value)` (also still serde_json, in `🧬️schema/🧬️mutations/🦀️.rs`,
out of scope). Left that plumbing alone; fixed only the two genuinely-broken spots where a struct
switched from serde-derive to value-derive-only.

## Files edited (18)

### 🖐️5d/editor/🎮️commands — all fixed by swapping `use serde_json::Value` → `use dsl::os_pack::json::Value`
(method surface — `.get`/`.as_str`/`.as_bool`/`.as_f64`/`.as_array`/`Value::from` fn-pointer args —
is identical on the new type, so no body changes needed beyond the import):
- 🔄️rotate-selection, 🔄️translate-selection — pure f64 arg extraction, import swap only.
- 🔗️retarget-fastener, 🔗️create-fastener, 🔗️proximity-connect — the reported E0631 closure
  mismatches were `Value::as_array`/`as_str`/`as_bool` fn-pointers against the wrong `Value`; import
  swap alone resolves them (`document.kind_compatibility: Option<Value>` already uses the new type
  in `editor/🦀️.rs`).
- ✏️patch-fastener, ✏️patch-grip, ✏️patch-part, 🔗️edit-fastener — call `puzzle5d_resolve_number_edit`
  (already `Value`-typed in `editor/🦀️.rs`); import swap only.

### 🖐️5d — import swap + body fix
- 🔄️scale-selection — `part.part_3d.scale` is now `Option<Puzzle5dScale>` (an enum,
  `Uniform(f64)`/`Vec3([f64;3])`), not a raw JSON array. Replaced
  `Some(json!([...]))` with `Some(Puzzle5dScale::Vec3([...]))`
  (`crate::artifacts::puzzle5d::Puzzle5dScale`).
- 🎥️set-camera, 🎥️set-camera-2d, 🎥️set-camera-3d — `Puzzle5dCamera2d`/`Puzzle5dCamera3d`
  (`editor/🎚️config/🦀️.rs`) derive only `value_derive::ToValue`/`FromValue`, no serde at all.
  Replaced `serde_json::from_value::<T>(camera.clone())` with
  `T::from_value(dsl::os_pack::json::to_dsl_value(camera))` (`use dsl::FromValue;`).
- 🖌️add-brush-part, 🧩️add-part-kind — `use dsl::json;` (macro, matches the already-converted
  `🧊️3d` pattern) + `use dsl::os_pack::json::Value;`. `add-brush-part`'s rename-key step
  (`object.remove("partKind")` → `insert("objectKindId", ...)`) had to drop the `remove` since
  `pack::json::Object` has no `remove` method (only `get`/`get_mut`/`insert`/`contains_key`) — now
  reads-then-inserts, leaving the stale `partKind` key present. Harmless: the downstream decode
  target (`BrushPlacePayload`) has no `deny_unknown_fields`, so the extra key is ignored.
- 🛍️set-fixture-json — `Puzzle5dDocument` has a **manual** `impl dsl::FromValue`
  (`editor/🦀️.rs:448`), not derived. Replaced `serde_json::from_str::<Puzzle5dDocument>(text)` with
  `dsl::os_pack::json::from_json_str::<Puzzle5dDocument>(text)`.

### ◻️2d/editor/🎮️commands — narrow fixes only, `Value` alias left as `serde_json::Value`
- 🛍️set-active-example — two spots where a struct dropped its serde derive:
  - `Puzzle2dKindCompatibility::from_value(dsl::DslValue::from(row))` replaces
    `serde_json::from_value::<Puzzle2dKindCompatibility>(row.clone())` (`row: &serde_json::Value`;
    uses the **sanctioned reverse bridge** `impl From<&serde_json::Value> for DslValue` at
    `🧰️framework/🔨️modules/🌱️value/🦀️.rs`, whose own doc comment names this exact
    still-serde-dispatcher transitional case — not the forbidden forward direction).
  - Replaced the `serde_json::to_value(catalogs)` vs `current: Option<&serde_json::Value>`
    comparison with both sides converted **forward** into `DslValue`
    (`current.map(dsl::DslValue::from)` / `target.meta.kind_catalogs.as_ref().map(dsl::ToValue::to_value)`)
    instead of bridging `DslValue` back into `serde_json::Value` — avoids adding a 9th forbidden
    `serde_json::Value::from(&DslValue)` site (constraint said 8 already exist in 3d, add no more).
  - Everything else (the `json!`/`Value` for `doc.0`, `queue()`'s `optional_json_to_dsl` call, the
    two `LazyLock` example statics) left untouched — genuinely still serde_json-typed via the
    unmigrated `Puzzle2dPlaySnapshot(pub Value)`.
- 🎲️apply-board-events — single fix: `envelope.runtime.brush_candidates` is `Vec<dsl::DslValue>`
  (`editor/🎚️config/🦀️.rs:203`); replaced `candidates.clone()` (`Vec<serde_json::Value>`) with
  `candidates.iter().map(dsl::DslValue::from).collect()` (same sanctioned reverse bridge).

## Not converted / left alone
- `editor/🦀️.rs` dispatcher files (5d and 2d), `panels/🛍️catalogue`, `🧠️precompute`,
  `🎭️modes/✏️edit/*`, `🪟️windows/◻️2d` (nested inside 5d) — outside the literal
  `editor/🎮️commands/**` scope given in the brief; left for whatever wave owns those (156+ of the
  captured 5d errors are there, mostly cascading E0631/E0308 from callers of files I did fix — some
  will likely resolve on their own now).
- `🧊️3d/**` — untouched per hard constraint (another session).
- No `serde_json::Value::from(&DslValue)` (forward) bridges added anywhere — grepped all 18 touched
  files, zero hits except the two intentionally-untouched 2d files' pre-existing serde_json usage.

## Verification performed (no cargo)
- Re-read every edited file from disk after editing.
- Grepped all 18 touched files for `.get([0-9]` (array-index trap) — zero hits.
- Grepped for leftover `serde_json` — zero hits in the 16 5d files; exactly the expected
  intentionally-untouched lines in the 2 2d files.
- Grepped for `unwrap_or_else` beside `to_json_string`/`from_json_str` — zero hits (no stale
  infallible-conversion fallbacks introduced).
- Traced every helper/field type (`mesh_selection_ids`, `part_scale_json`,
  `apply_engine_brush_placement`, `apply_board_brush_place`, `puzzle5d_resolve_number_edit`,
  `Puzzle5dDocument`'s manual `FromValue`/`ToValue`, `Puzzle5dScale`, `Puzzle2dPlaySnapshot`,
  `Puzzle2dKindCompatibility`/`Puzzle2dKindCatalogs` derives) by reading its actual definition
  rather than assuming from the brief's translation table.
