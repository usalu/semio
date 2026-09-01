# The `command_from_action` seam — migrated to `DslValue`

## Headline

**`ArtifactEditor`/`ArtifactApp::command_from_action` and `host_configuration_mutation` are now
`serde_json`-free — PROVEN BY A PASSING CHECK.** `semio-framework-plugin` compiles clean (0 errors)
both natively and for `wasm32-wasip2`, and `semio-framework-os-kernel` is unchanged at 0 errors — the
green baseline this ticket promised held. `🏭️process`'s own `command_from_action`/
`host_configuration_mutation` bodies, its inspector-patch command, its media-export codec and its
`🚪️io` export struct are converted too. `🏭️process`'s `serde_json` dependency is **NOT** at zero —
WRITTEN AND JUSTIFIED, not an oversight — see "Why `serde_json` cannot go to zero" below.

## Measured first

```
grep -rn "command_from_action" --include='*.rs' 🧰️framework ✏️s | wc -l   → 143 occurrences
grep -rl "command_from_action" --include='*.rs' 🧰️framework ✏️s | wc -l   → 30 files
grep -rl "command_from_action" --include='*.rs' 🧰️framework              → 6 files
grep -rl "command_from_action" --include='*.rs' ✏️s                       → 24 files
```

All 6 framework hits outside the actual seam file turned out to be unrelated: `🌉️mcp/*` only
mentions the name in doc comments, `📺️renderer/…/Shell/🧊️component.rs`'s `directory_command_from_action`
is an unrelated same-named local helper (shell chrome directory commands, not the trait method), and
`🌱️value/🦀️component.rs`'s hit is a doc comment on the pre-existing `serde_json::Value → DslValue`
bridge. **The entire real seam lives in one file**:
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (19 of the 143 hits, the trait
declarations plus every function that threads the same `args` value down to them).

Of the 24 `✏️s/` files, only `🏭️process`'s own editor file was in this session's scope (per the
ticket). The other 23 (cad, puzzle ×3, block ×3, space ×3, procedural ×2, stdio, gis ×2, sourcing,
architect, vcs, demonstrator, norm, remodel ×2, playbook extension) still implement the trait against
the OLD `serde_json::Value` signature and now fail to compile against the new one — **expected**,
per the same pattern every prior seam move in this ticket produced (see
`📓️serde-fanout-playbook.md`'s own line: "This breaks every plugin that hasn't converted yet —
expected"). Checked which of these are the ticket's wasm32-wasip2 guardrails (animate/flow/draw-fsm):
**none of them implement `command_from_action`** (grep-confirmed) — animate/flow default to the
trait's provided body, so this seam move cannot regress their own compile by itself (see "Guardrail"
below for what DID, and did not, regress them).

## The signature change

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`:

```rust
// ArtifactApp (was: Option<&Value>, i.e. serde_json::Value)
fn host_configuration_mutation(_action: &str, _args: Option<&DslValue>) -> Result<Option<Self::ConfigMutation>, Fault>
async fn command_from_action(action: &str, _args: Option<&DslValue>) -> Result<Self::Command, Fault>

// ArtifactEditor — identical shape
fn host_configuration_mutation(_action: &str, _args: Option<&DslValue>) -> Result<Option<Self::ConfigMutation>, Fault>
fn command_from_action(action: &str, _args: Option<&DslValue>) -> Result<Self::Command, Fault>
```

`DslValue` was already bare-imported in this module (`use dsl::{to_dsl_value, DslValue};`, pre-existing
line). `use serde_json::Value;` was **deliberately kept** — the module legitimately uses
`serde_json::Value` for ~60 unrelated things (MCP dispatch fixtures, mesh/SVG document importers,
world3d config UI helpers, media JSON, wire-format tests) that are not part of this seam and stay
serde-based (framework is exempt from the plugin serde ban). Only the functions that actually thread
`command_from_action`/`host_configuration_mutation`'s `args` were converted.

## Following it outward — the full pipeline, function by function

Every function whose `args`/`value` parameter is the *same* value that ultimately reaches
`command_from_action`/`host_configuration_mutation` (not just the two trait methods themselves) had to
move together, because Rust requires one static type per binding threaded through a call chain:

- `merge_ui_values` / `ui_value_to_json_retained` (renamed `ui_value_to_dsl_retained`) /
  `UiCommandJsonProducer` / `UiCommandJsonFrame` — the retained-`UiValue`-to-JSON bridge that feeds
  `command_from_intent`. Rewritten to build `DslValue` trees directly instead of `serde_json::Value`
  (no JSON-text round trip — this is in-memory only, per the playbook's own "JSON text vs DslValue"
  rule). Added `dsl_object_insert` (replace-if-present-else-append on `DslValue::Object`'s
  `Vec<(String, DslValue)>`) to replace `serde_json::Map::insert`'s map semantics.
- `dispatch_action`, `dispatch_command`, `dispatch_framework_reserved_action`,
  `dispatch_interaction_action`, `commit_framework_history_route`, `commit_framework_revert_route`,
  `commit_framework_shared_host_route`, `framework_reserved_work_items`, `history_command`,
  `history_command_authors`, `admit_command_json_with_proof`, `admit_command_json`,
  `admit_host_configuration_json`, `build_artifact_reserved_action_job`, `bounded_json_items`,
  `interaction_domain_id_arg`, `parse_interaction_targets`, `parse_merge_mode`, `handle_action`,
  `handle_action_invocation` — all converted signature + body. `InverseAction.args` and
  `ArtifactReservedToolInput::Action.args` (both public SDK types) moved to `Option<DslValue>` too,
  since they carry the identical value.
- `plugin_runtime::plugin_exchange`'s `ArtifactCommand` arm (the actual WASI-guest wire-decode
  boundary — `envelope: Value = decode_wire_serialized_or(...)` stays `serde_json::Value` since it is
  a genuine wire-text decode, then `args = envelope.get("args").map(DslValue::from)` bridges once at
  the boundary using the existing `From<&serde_json::Value> for DslValue` impl).
- `assert_declared_actions_bridge_to_commands` (the test harness every app's own action-coverage test
  calls): used to build `DslValue` via `effective_action_args`, then explicitly convert back to
  `serde_json::Value` via `store::pack_rt::dsl_value_to_json` just to satisfy the old signature — that
  round trip is now gone; the staged `DslValue` is passed straight through.

### `.as_u64()` has no `DslValue` equivalent

`DslValue::Number` is a single `f64` (matching `pack::json`'s own number-widening convention). Every
`Value::as_u64()`/`as_i64()` call site (`entrySeq` fields, `unsigned_field`/`signed_field` helpers)
became `.and_then(DslValue::as_f64).map(|n| n as u64)` (or `as i64`).

### JSON-text serialization boundaries — bridged, not left as a serde escape hatch

Two spots genuinely still need JSON **text** (not just an in-memory value) and route through the
already-existing `serde_json::Value ↔ DslValue` bridge (`🧰️framework/🔨️modules/🌱️value/🦀️component.rs`,
built in an earlier session) rather than reintroducing a parallel decode path:

- `admit_command_json_with_proof` writes the admitted command onto a raw wire byte page via
  `serde_json::to_writer` — `args: Option<&DslValue>` is bridged once (`args.map(serde_json::Value::from)`)
  immediately before the write.
- `dispatch_framework_reserved_action` does the equivalent for its `raw` wire fixture.

Both are framework code (exempt from the plugin serde ban) and both bridge exactly once at the
text-boundary, matching the ticket's "route through the first-party JSON… do NOT keep a
`serde_json::Value` alive by converting back and forth just to avoid touching a call site" rule — the
call sites themselves (`bounded_json_items`, the reserved-action dispatch tree) all operate on the
real `DslValue` now; only the wire-serialize step touches `serde_json`.

### Test call sites (60+ in this one file)

Every `Some(&json!({…}))` / `Some(&serde_json::json!({…}))` passed into `handle_action`/
`dispatch_action`/`command_from_action` in this file's own test module was converted with a small
test-only bridge:

```rust
fn dv(value: serde_json::Value) -> DslValue { DslValue::from(&value) }
```

(inside `mod plugin_builder_contract_tests`, which names every import explicitly rather than
`use super::*`). This keeps the fixtures readable as JSON literals — framework tests using
`serde_json::json!` are fine, framework is exempt from the ban — while feeding the trait boundary the
real `DslValue` it now requires. `interaction_target_args` (built `serde_json::Value`, used by 8 call
sites) was changed to return `DslValue` directly instead, since none of its 8 callers needed anything
else.

## Guardrail — `semio-framework-plugin` itself: PROVEN clean, both targets

```
cargo check -p semio-framework-plugin --message-format=short
   ...
warning: `semio-framework-plugin` (lib) generated 203 warnings (run `cargo fix --lib -p semio-framework-plugin` to apply 103 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 2m 29s
```
exit 0, **0 errors**, re-run twice for stability (6.13s and 12.80s on incremental re-checks).

```
cargo check --target wasm32-wasip2 -p semio-framework-plugin --message-format=short
   ...
warning: `semio-framework-plugin` (lib) generated 210 warnings (run `cargo fix --lib -p semio-framework-plugin` to apply 103 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 1m 32s
```
exit 0, **0 errors**, wasm32-wasip2 target.

```
cargo check -p semio-framework-os-kernel --message-format=short
   ...
warning: `semio-framework-os-kernel` (lib) generated 33 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 33 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.75s
```
exit 0, **0 errors, 33 warnings** — identical to this ticket's own recorded baseline. Not regressed.

### Plugin-level guardrail — contaminated by unrelated concurrent work, not by this seam

`cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-flow` fails, but **not** on anything this
seam touched:

```
🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/.../🕸️dag/🧬️schema/🧬️mutations/🔗️connect-nodes/🦀️.rs:5:59: error[E0277]: the trait bound `EdgeRouteStyle: ToValue` is not satisfied
🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/.../🕸️dag/🦀️component.rs:8833:68: error[E0277]: the trait bound `DagDelta: ToValue` is not satisfied
...6 more, same shape (DagNodeKind/DagNodeSpec × ToValue/FromValue)...
error: could not compile `semio-framework-os-infinite` (lib) due to 8 previous errors
```

then separately fails again in `semio-s-plugin-stdio` (2218 errors, the ticket's own documented,
in-flight, another-agent's wave). Neither error set names anything from this seam's own files
(`🔌️plugin/🦀️component.rs`, `🏭️process`'s files) — grep-confirmed. `semio-framework-plugin` itself
appears in that same build log with **zero errors** (`warning: semio-framework-plugin (lib) generated
176 warnings`, no error line) — it compiled clean as flow's dependency; `semio-framework-os-infinite`
(an unrelated crate — DAG board mutation types, not touched by this session at all) is what broke the
overall build. This matches the ticket's own documented pattern ("Concurrent Cargo Workspace Churn" —
repo-wide failures from another session's in-progress refactor) and its own stdio warning verbatim.
**Not fixed here** — out of scope, actively owned by other concurrent work.

## `🏭️process` — its own conversion

`command_from_action`/`host_configuration_mutation`
(`✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`):
converted `string_field`/`number_field` to `DslValue::as_str`/`as_f64`; `unsigned_field`/`signed_field`
dropped their dead `as_u64`/`as_i64` branches (DslValue has none) and now go straight through
`as_f64`; `vec3_field`/`vec2_field` unchanged (slice indexing + `as_f64`, same shape either way);
`"setSnapshot"`/`"updateStep"`'s `serde_json::to_string(value)` JSON-text re-encode became
`semio_framework_os_kernel::json::to_json_string(value)` (the `pack::json` `T: ToValue` helper,
`DslValue` implements `ToValue` identically per its own blanket impl); `"updateWorkshopMachine"`'s
`value.into()` (the old `serde_json::Value → DslValue` bridge call) is now a no-op removed, since
`value` is already the `DslValue` `args.get("machine")` handed over. The `testkit::action` test helper
and 3 direct `command_from_action`/`host_configuration_mutation` test call sites converted
(`Some(&serde_json::json!(…))` → `Some(&DslValue::from(&serde_json::json!(…)))`).

Beyond the seam itself, converted three more genuinely trait-adjacent sites while in the file,
confirmed by grep that all their own callers were updated:

- **`🎮️commands/🔎️inspector/🦀️component.rs`** (inspector field-patch dispatcher) — `Value`/`json!`
  usage there was entirely self-contained (never crossed a trait boundary, just unified two local
  patch-application functions' shared type) — converted fully to `DslValue`, `serde_json` count in
  this file: 0.
- **`🎮️commands/📤️media/🦀️component.rs`** + **`🚪️io/🦀️component.rs`** — `Process3dModelExport.data`
  (process's own struct, always constructed as `Value::String(...)`, never Object/Array/Number)
  converted to `DslValue`; the media command's consumer match converted to match. `serde_json` count
  in both files after: 0 (io file's one remaining `serde_json` mention is a comment).
- **`✏️editor/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🦀️component.rs`** — `process3d_window_action`'s
  `args: Option<serde_json::Value>` (bridged via `semio_framework::optional_json_to_dsl`) changed to
  take `Option<DslValue>` directly; all 4 call sites already passed `None` (grep-confirmed), so no
  further call-site changes needed. `process3d_selection_json` (parses/mutates/re-emits a JSON string)
  rerouted through `semio_framework_os_kernel::json::{from_json_str, to_json_string}` on `DslValue`
  instead of `serde_json::from_str`/`.to_string()`.

## Why `serde_json` cannot go to zero for `🏭️process` — a NEW, concrete, measured reason

The earlier session's doc named the trait boundary as the reason. That reason is now closed. A
**different**, harder blocker remains, found by reading the actual dependency, not assumed:

`✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`'s
`deserialize_bytes` parses raw JSON bytes into `semio_s_plugin_stdio::artifacts::json::JsonSnapshot`
— a type **owned by `🗄️stdio`**, not by process. `JsonSnapshot::from_value` takes a real
`serde_json::Value`; that type has not been converted (stdio is its own ~563-file wave, currently
mid-flight at ~2218 errors from a different concurrent agent, explicitly out of scope here). Process
cannot decode this bridge without depending on stdio's own `serde_json`-shaped API, so
`serde_json.workspace = true` stays in `🏭️process`'s `Cargo.toml` — correctly, not an oversight.
Beyond that: `evaluated_preview_payload` (mesh/instance preview JSON for the render pipeline,
`json!([{...}])` embedding a `Serialize` mesh value) and several test-only layout/definition JSON
serializations remain `serde_json` too — genuinely orthogonal to `command_from_action`, unconverted
this session for lack of budget, not because they're blocked.

## Verification — verbatim tails

```
$ cargo check -p semio-framework-plugin --message-format=short
warning: `semio-framework-plugin` (lib) generated 203 warnings (run `cargo fix --lib -p semio-framework-plugin` to apply 103 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 2m 29s
```
PROVEN BY A PASSING CHECK. exit 0, 0 errors.

```
$ cargo check --target wasm32-wasip2 -p semio-framework-plugin --message-format=short
warning: `semio-framework-plugin` (lib) generated 210 warnings (run `cargo fix --lib -p semio-framework-plugin` to apply 103 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 1m 32s
```
PROVEN BY A PASSING CHECK. exit 0, 0 errors.

```
$ cargo check -p semio-framework-os-kernel --message-format=short
warning: `semio-framework-os-kernel` (lib) generated 33 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 33 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.75s
```
PROVEN BY A PASSING CHECK. exit 0, 0 errors, 33 warnings — unchanged from ticket baseline.

```
$ cargo check -p semio-s-plugin-process --message-format=short
...
error: could not compile `semio-s-plugin-stdio` (lib) due to 2218 previous errors; 1325 warnings emitted
```
NOT PROVEN — blocked by `semio-s-plugin-stdio` (confirmed by `grep -ic "process3d\|semio-s-plugin-process" <output>` → `0`, i.e. zero errors name anything process-owned; every error is inside `🗄️stdio`,
re-run twice for stability, 2217→2218 as stdio's own concurrent wave progressed between runs).
Process's own source is believed correct (hand-reviewed against the real `DslValue`/`pack::json` API,
matching this file's own pre-existing usage of the same helpers) but unverified by a passing compile
for the same structural reason the prior session recorded: stdio fails first, so cargo never reaches
process's own type-check.

```
$ cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-process --message-format=short
...
error: could not compile `semio-s-plugin-stdio` (lib) due to 2218 previous errors; 1326 warnings emitted
```
NOT PROVEN — identical blocker, wasm32-wasip2 target.

```
$ cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-flow --message-format=short
...
error: could not compile `semio-framework-os-infinite` (lib) due to 8 previous errors; 18 warnings emitted
...
error: could not compile `semio-s-plugin-stdio` (lib) due to 2218 previous errors; 1326 warnings emitted
```
Guardrail contaminated by unrelated concurrent work (see "Guardrail" section above) — `semio-framework-plugin`
itself compiles clean inside this same log.

## Files touched this session

**Framework** (one file, the seam itself):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — trait signatures ×4, the
  UI-intent-to-command JSON bridge (`merge_ui_values`/`ui_value_to_dsl_retained`/
  `UiCommandJsonProducer`/`UiCommandJsonFrame`, +`dsl_object_insert`), the entire framework-reserved
  dispatch tree (~20 functions), `InverseAction`/`ArtifactReservedToolInput::Action`'s `args` field,
  the wasm-guest wire-decode arm in `plugin_runtime::plugin_exchange`, ~70 test call sites, +1
  test-only `dv()` helper.

**`🏭️process`**:
- `.../✏️editor/🦀️component.rs` — `command_from_action`/`host_configuration_mutation` bodies, the
  `export_media` brep-export text match, 3 test call sites, dropped an unused `use serde_json::Value;`.
- `.../🚪️io/🦀️component.rs` — `Process3dModelExport.data: DslValue`, its 3 construction sites.
- `.../✏️editor/🎮️commands/📤️media/🦀️component.rs` — export-data consumer match.
- `.../✏️editor/🎮️commands/🔎️inspector/🦀️component.rs` — fully converted, 0 `serde_json` left.
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🦀️component.rs` — `process3d_window_action`
  signature, `process3d_selection_json` rerouted through `pack::json`.

**Not touched, correctly left alone**: `Cargo.toml`'s `serde_json.workspace = true` (still needed, see
above); `world3d_sun_measures`/`apply_world3d_sun_action`/`world3d_projection_*` helpers in the seam
file (used by cad/puzzle/procedural/lowpoly's own COMMAND handlers for UI descriptor construction, not
by `command_from_action`'s trait parameter itself — grep-confirmed process never calls the `apply_*`
variants, only the display-only `*_measures` builders); ~25 remaining `serde_json` call sites in
`🏭️process` genuinely orthogonal to this seam (render/preview JSON, layout/definition test
serialization, the stdio `JsonSnapshot` bridge).

## Summary for whoever picks this up next

- The seam is closed and proven. Every plugin implementing `ArtifactEditor`/`ArtifactApp` now decodes
  `command_from_action`/`host_configuration_mutation` args as first-party `DslValue`, no exceptions,
  no compat shim.
- `🏭️process` cannot reach zero third-party dependencies until `🗄️stdio`'s own wave lands (its
  `JsonSnapshot` bridge is a hard, measured blocker, not a preference) — re-run
  `cargo check -p semio-s-plugin-process --message-format=short` once stdio is green; if it's not
  clean immediately, the remaining error list will be small and process-specific now, not fleet noise.
- Every OTHER plugin implementing the trait (cad, puzzle ×3, block ×3, space ×3, procedural ×2,
  stdio, gis ×2, sourcing, architect, vcs, demonstrator, norm, remodel ×2, playbook extension) now
  fails to compile against the new signature — expected, matching every prior seam in this ticket;
  each needs the identical mechanical treatment `🏭️process` got here (`Value::as_str` →
  `DslValue::as_str`, drop `as_u64`/`as_i64`, bridge JSON-text serialization through `pack::json`).
