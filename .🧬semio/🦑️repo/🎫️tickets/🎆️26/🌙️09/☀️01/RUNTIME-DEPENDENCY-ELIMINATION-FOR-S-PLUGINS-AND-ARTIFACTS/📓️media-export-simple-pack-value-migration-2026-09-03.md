# media_export_simple: serde_json::Value → semio_framework_os_kernel::json::Value

## Target
`🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs`, module `media_export_simple` (was lines 3434-3516).

## Key finding: the import path
The ticket assumed `use semio_framework_pack::json::Value;` directly, with `semio-framework-pack`
as a **direct** dependency of the os-host crate. That is false: os-host's Cargo.toml
(`🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml`) does NOT list
`semio-framework-pack`. It only reaches pack **transitively** through
`semio-framework-os-kernel` (already a direct dep, line 26), which re-exports the whole
`pack::json` module at its own crate root:
`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs:143` (`pub use pack::json;` inside
`os_pack`) plus `:303` (`pub use crate::os_pack::*;`).

So the correct, already-established repo alias is `semio_framework_os_kernel::json::Value` — this
exact pattern (`semio_framework_os_kernel::json::{Value, object, to_json_string, from_json_str,
from_dsl_value, parse, value_eq_ignoring_object_order}`) is already used throughout
`✏️s/🔌️plugins/🏭️process/…` (process3d plugin), confirming it as precedent, not invention. No
Cargo.toml was touched — this works purely because os-host already has a direct dep on
`semio-framework-os-kernel`, and animate's own Cargo.toml already has the same direct dep
(`✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml:37`).

## Changes made
1. `🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs:3438` —
   `use serde_json::Value;` → `use semio_framework_os_kernel::json::Value;`.
   No other line in `media_export_simple` needed changes: every `.get("...")` call is an
   object-key lookup (ports unchanged), every `.get(1)`/`.first()` call is native `Vec<Value>::get`
   on a `&Vec<Value>` obtained via `.as_array()` first (not `Value::get`), and every numeric read
   goes through `.as_f64()` which the pack `Number` type handles across its Int/UInt/Float arms —
   so no `.get(0)`-on-array bug and no int/float defect existed in this module to begin with.

2. `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs`
   — the only production caller of `title_card_svg` in animate or raster:
   - `animate_presentation_document_json_to_svg(value: &serde_json::Value)` →
     `&semio_framework_os_kernel::json::Value`.
   - Its two `#[test]`s used `serde_json::json!({...})` to build the value passed in — this
     file's own tests ARE the source (no external production caller exists), so they were
     converted to the pack builder: `use semio_framework_os_kernel::json::{object, Object, Value};`,
     `object([("title".to_string(), Value::from("My Deck"))])` and `Value::Object(Object::new())`
     for the empty case (avoided `object([])` deliberately — an empty array literal against a
     generic `impl IntoIterator<Item = (String, Value)>` parameter risks a "type annotations
     needed" inference failure; `Value::Object(Object::new())` sidesteps that ambiguity entirely).
   - `animate_presentation_document_json_from_dwg`'s return type and its downstream test call
     sites were ALSO found converted from `Result<serde_json::Value, String>` to
     `Result<dsl::DslValue, String>` in this same file — this was NOT my edit; a concurrent
     session/agent on this same ticket touched the same file (confirmed via `git diff`, which
     showed changes I did not make). Left as-is per "ignore unrelated concurrent changes, keep
     focus." It is compatible with and complementary to my change (removes another serde_json
     reference from the same file).

## Raster and gis: no live call sites, only stale comments
Ticket asked to check `✏️s/🔌️plugins/🖨️raster` for 2 call sites of these three fns. Grep found
**zero live calls** — `title_card_svg`/`pages_rects_svg`/`map_points_svg` appear only inside doc
comments (`…/🚪️io/🦀️.rs:102,237`) describing what the *old* placeholder used to be, before raster
was migrated to its own real `SemioDrawingSnapshot`-based SVG path
(`drawing_snapshot_from_raster` → `raster_document_json_to_svg`). A repo-wide grep for these three
function names also turned up `✏️s/🔌️plugins/🌍️gis/…/🧬️schema/🦀️.rs:518` — likewise only a stale
doc comment ("replaces the old hand-rolled `map_points_svg` delegate"), no live call. Nothing to
change in either plugin; no files touched there. Since gis is out of ticket scope (not
animate/raster) and has no live call site anyway, this is moot, not a blocker.

## Verification (no cargo run)
```
grep -n 'serde_json' '🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs' | sed -n '/34[3-9][0-9]\|35[01][0-9]/p'
# -> empty (0 refs in the module's line range)

grep -rn 'serde_json' ✏️s/🔌️plugins/🎞️animate --include='*.rs' | grep -vE '🧪|🏭|🔬' | grep -vE ':\s*(///|//!|//|\*)'
# -> 3 refs, all inside ✏️editor/🦀️.rs's `retained_command_fixture_matches_exact_routes_and_serde_json_boundaries`
#    test — a deliberate third-party-library oracle comparison (CLAUDE.md's own "same output with
#    at least one third-party library" test rule), unrelated to media_export_simple/title_card_svg,
#    left untouched (out of this task's scope).

grep -n '\.get([0-9]' <touched files>
# -> only 🖥️host/🦀️.rs:3491,3505 -- both `coords.get(1)` where coords: &Vec<Value> from
#    .as_array(), i.e. Vec::get, not Value::get. No array-index bug.
```

## Ref counts
- `media_export_simple` module (lines ~3434-3516): serde_json refs 1 → 0.
- `🎞️animate` plugin non-comment serde_json refs (ticket's grep methodology): 6 (ticket-reported
  starting point) → 3 (all in one pre-existing, in-scope third-party-oracle test, `✏️editor/🦀️.rs`,
  not part of this change).

## Files touched (git diff --name-only, my own edits)
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs` (1 line)
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs`
  (fn signature + test bodies)

Note: `git diff --name-only` against the live tree shows many more files across the repo/ticket —
this is concurrent work by other sessions on the same ticket (confirmed: some of it lands inside
the exact same io/🦀️.rs file I edited, verified via `git diff` showing hunks I did not author).
Per CLAUDE.md, that is expected simultaneous multi-dev editing and was left alone.

## No cargo run, no sub-agents
Zero `cargo` commands were executed. No sub-agents were spawned — all investigation and edits were
done directly in this session.
