# ✅️ `dsl::os_pack::json::Value` is FIRST-PARTY — converting to it is a real migration, not a bridge

Verified directly, because an agent's finding hinged on it and it changes the definition of done:

- `pub enum Value` is DEFINED in our own source at `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:232`.
- `semio-framework-pack`'s `[dependencies]` contains **NO serde and NO serde_json**.

So `dsl::os_pack::json::Value` is an in-house JSON value type — a reimplementation, not a re-export.

## Why this matters for the ticket
Code reading `dsl::os_pack::json::Value` LOOKS serde-shaped and is not. Converting a handler from
`serde_json::Value` to `os_pack::json::Value` genuinely removes the third-party dependency; it does
NOT merely rename it. Several already-converted command files in 🧩️puzzle use exactly this target,
and an agent confirmed it is the conventional one for command-handler args in this app — `DslValue`
is not always the right destination.

## The distinction that actually matters
| shape | verdict |
|---|---|
| `os_pack::json::Value` / `pack::json::to_json_string` / `from_json_str` | ✅️ first-party, ships no serde |
| `dsl::DslValue`, `ToValue`, `FromValue` | ✅️ first-party |
| `serde_json::Value`, `serde_json::from_value`, `serde_json::to_string` | ❌️ third-party, links serde_json |
| `From<&DslValue> for serde_json::Value` used to satisfy a bound | ❌️ a BRIDGE — compiles, still links serde_json |

## Known remaining bridges (recorded, not fixed)
1. ~~`🧊️3d`'s `from_dsl_value` is a bridge~~ — **RETRACTED, this was wrong.** I recorded a peer's
   characterisation without checking the signature. `pack::json::from_dsl_value(&DslValue) -> Value`
   returns the FIRST-PARTY `pack::json::Value` (🎒️pack/🔤️json/🦀️.rs:525), so
   `args.map(dsl::os_pack::json::from_dsl_value)` at 🧊️3d editor:6745 links NO serde_json. It is a
   legitimate conversion, not a bridge. ~55 handlers taking `os_pack::json::Value` are FINE as-is.
   **The real 3d work is different**: 19 genuine `serde_json` code refs remain in
   `🧊️3d/…/✏️editor/🦀️.rs` (32 total minus 13 in comments), centred on
   `scene_from_projection(projection: &serde_json::Value, …)` :295 and
   `puzzle3d_operations_from_fixture_change(before: &serde_json::Value, …)` :325, plus
   `serde_json::Value::from(&dsl::ToValue::to_value(...))` round-trips at :312, :314, :329.
   Only 3 of 55 command files mention serde_json.
2. `🖐️5d/…/✏️editor/🦀️.rs` — `impl dsl::FromValue for Puzzle5dDocument` routes through
   `serde_json::from_value(Value::from(&value))`. Added deliberately because the struct's
   `Option<serde_json::Value>` fields block derive expansion. That file still has **77** `serde_json`
   refs; an audit found many are a documented boundary (`Puzzle5dPlaySnapshot` genuinely wraps
   `serde_json::Value`) rather than leftovers.

**A plugin at 0 errors is not necessarily dependency-free.** Both crates above compile clean and still
link serde_json. The authoritative check is the manifest plus
`cargo tree --target wasm32-wasip2 | grep '^serde'`, never the error count.
