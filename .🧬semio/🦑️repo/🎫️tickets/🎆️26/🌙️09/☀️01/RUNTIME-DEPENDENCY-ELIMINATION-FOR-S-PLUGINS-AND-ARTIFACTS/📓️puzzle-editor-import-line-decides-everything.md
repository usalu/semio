# 🎯️ One import line per editor decides whether a puzzle artifact family is converted

Found by a peer session, verified by me. Each artifact family's `✏️editor/🦀️.rs` has ONE import that
determines which `Value` type the whole family's handlers are typed against:

| family | editor import | `Value` | `json!` | state |
|---|---|---|---|---|
| `🧊️3d` | `use dsl::os_pack::json::{from_json_str, object, parse, to_json_string, to_string, Object, Value};` (:55) | 275 | 101 | ✅️ converted, coherent |
| `◻️2d` | `use serde_json::{json, Value};` (:41) | 117 | 68 | ⚠️ UN-converted, coherent |
| `🖐️5d` | `use serde_json::{json, Value};` (:43) | 389 | 46 | ❌️ INCOHERENT — handlers converted, editor not |

## The trap: 2d is green because nothing was touched
`◻️2d` compiles at 0 errors and is NOT done — it is uniformly serde_json, so it is internally
consistent. `🖐️5d` is red precisely BECAUSE it was half-converted: its `🎮️commands/*` handlers now
take `dsl::os_pack::json::Value` while the helper they call
(`puzzle5d_resolve_number_edit`, `✏️editor/🦀️.rs:512`, `value: Option<&Value>`) is still typed against
serde_json's `Value` via that one import.
**Converting more handlers WIDENS the mismatch until the editor import moves.** Convert the editor
first, or not at all.

This is the third "green ≠ done" on this ticket:
1. `semio-framework-surface` read 0 all night — it had never been compiled behind a red chain.
2. `🧊️3d` read 0 while still linking serde_json (19 real refs, since removed → 0).
3. `◻️2d` reads 0 because it is entirely un-converted.

## Decision taken: FORWARD, not backward
`🧊️3d` proves the conversion is tractable and carries MORE `json!` sites (101) than 5d (46), so there
is a working in-tree template rather than a design to invent. Reverting 5d's handlers to serde_json
would restore green while moving AWAY from the goal (no serde in a plugin's wasm component).
`◻️2d` is deliberately left un-converted and coherent tonight rather than opening a third
half-finished middle — recorded here as known remaining work.
