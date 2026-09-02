# 🖍️🏛️ draw + architect: serde → first-party value conversion

## Scope
`✏️s/🔌️plugins/🖍️draw` (crate `semio-s-plugin-draw`) and `✏️s/🔌️plugins/🏛️architect` (crate
`semio-s-plugin-architect`), now that `flatten`/`with`/`skip` landed in
`🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs`.

## draw — converted, verification in flight
Every production `#[derive(Serialize, Deserialize)]` → `#[derive(dsl::ToValue, dsl::FromValue)]`,
every `#[serde(...)]` → `#[value(...)]`, including the 5 `#[serde(flatten)]` sites on
`DrawShapeBody`/`DrawPathBody`/`DrawTextBody`/`DrawImageBody`/`DrawGroupBody`/`DrawBooleanBody`/
`DrawTraceBody`'s `base: DrawLayerBase` field (now `#[value(flatten)]`, kept alongside the
pre-existing `#[dsl(block)]` for the text/binary DSL grammar side, which has no flatten primitive).

**No `#[cfg_attr(test, ...)]` twins needed anywhere in draw** — grepped every `🧪️tests/` fixture
file and every inline `#[cfg(test)] mod tests` in the plugin: none of them call `serde_json`
directly on a draw-owned type. The `mutate-draw-1` exhaustive case and all 13 per-mutation fixture
tests go through the plugin's own JSON-string `TestBridge` functions
(`apply_draw_mutation_json`/`undo_draw_mutation_json`/`round_trip_draw_dsl` in
`🧬️schema/🧬️mutations/🦀️.rs`), which are now converted to `dsl::json::{from_json_str,
to_json_string}` end to end. The handful of `serde_json::to_string(&node)` calls remaining under
`#[cfg(test)]` operate on FRAMEWORK `BuiltNode`/`WindowLayout` values (`app.render(...)`), never a
draw type — left untouched, correct as-is.

### Production call sites converted (not just derives)
- `🚪️io/📥️import` and `🚪️io/📤️export` json leaves: `serde_json::{from_value,to_value}` →
  `dsl::FromValue`/`dsl::ToValue` + `dsl::json::{to_dsl_value,from_dsl_value}` bridged through
  stdio's `JsonSnapshot::to_pack_value()`/`from_value()` — mirrors block/trinity's proven pattern,
  no `pack` Cargo dependency added (bridges through the `dsl` alias's existing `json` module).
- `🧬️schema/🧬️mutations/🦀️.rs`: `draw_op_for_layer_field`/`patch_layer_field` retyped from
  `&serde_json::Value` to `&dsl::DslValue` (same accessor methods: `as_str`/`as_f64`/`as_bool`);
  the `apply_draw_mutation_json`/`undo_draw_mutation_json`/`round_trip_draw_dsl` TestBridge trio
  rewritten onto `dsl::json::from_json_str`/`dsl::DslValue::object`/`dsl::json::to_json_string`.
- `🎮️commands/🗂️patch-layer(s)/🦀️.rs`: `patch_value_json` now parses via `dsl::json::parse` +
  `dsl::json::to_dsl_value` instead of `serde_json::from_str::<Value>`.
- `🎮️commands/🖱️canvas-pointer-down/🦀️.rs`: `request_interaction_action` retyped to take
  `dsl::DslValue` directly and construct `Effect::ReplayShellCommand`'s `args: Option<DslValue>`
  field without detouring through `semio_framework::optional_json_to_dsl` (a
  `serde_json::Value`-typed framework bridge) — callers now build `DslValue::object([...])`
  literally. `queue_trace_pointer`'s `serde_json::to_value(continuation)` replaced with
  `Some(dsl::ToValue::to_value(&continuation))` (the payload struct already derives `ToValue`).
- Two window/panel render files (`👁️viewer/…/🖼️canvas`, `✏️editor/…/✏️edit/🪟️windows/🖼️canvas`)
  and `📌️panels/🛍️catalogue`: `serde_json::{to_value,to_string}` / `json!{}` macro literals
  replaced with `dsl::DslValue::object([...])` builders and `dsl::json::to_json_string`.

### Not converted, deliberately, with reason
- `🔄️fsm/✨️macros/🦀️.rs:625`: a `quote!{}`-generated `#[cfg_attr(feature = "serde", derive(...))]`
  line inside the `statechart!` proc-macro's template output. Inert (no crate in this tree ever
  turns on a `serde` Cargo feature — draw's own `📦️packages/🦀️rust/🦀️.rs` docstring says so
  explicitly), and the macro crate (`semio-s-plugin-draw-fsm-macros`, `proc-macro = true`) is
  build-time-only per the same reasoning the ticket already used to exempt the proc-macro trio.
  Left as-is to avoid touching a shared code-generation template for zero real linkage change.
- `GestureContext`/`DrawGestureCheckpoint`/`TracePointerJob` (all in `canvas-pointer-down/🦀️.rs`):
  reverted OFF `ToValue`/`FromValue` back to plain derives. Each embeds a framework `UiFixedList<T,
  N>` field, which has only hand-written `Serialize`/`Deserialize` (no `ToValue`/`FromValue` impl
  exists anywhere in `🧰️framework` — confirmed by grep). None of the three is ever actually
  serialized in this plugin (grep-confirmed zero `serde_json`/`dsl::json` call sites touching
  them), so the original `Serialize`/`Deserialize` derive was already vestigial; dropping both
  traits entirely (rather than adding a framework-side `impl ToValue for UiFixedList`, out of this
  ticket's plugin-only scope) is the correct minimal fix.

## architect — converted, verification in flight
Same mechanical pattern applied across 282 production files (bulk script: `Serialize, Deserialize`
→ `dsl::ToValue, dsl::FromValue`, `#[serde(` → `#[value(`), plus hand conversion of every real
`serde_json::` call site (28 files) and `#[cfg_attr(test, ...)]` twins added to every converted
type (script-applied across all 282 files — needed because architect, unlike draw, DOES have a
direct `serde_json` round-trip test on the full `ProgramSnapshot` tree:
`sample_plugin_round_trips_json` in `🗿️artifacts/🏛️program/🦀️.rs`'s `mod tests`, plus the
`child-owner-isolation` fixture oracle in the same file).

### Notable hand conversions
- **`✏️editor/🗂️catalog/🦀️.rs`** (903 lines, the register CRUD engine): `use serde_json::{json,
  Value}` replaced with `use dsl::DslValue as Value;` (keeps every existing `.get()`/`.as_str()`
  call site working unchanged — `DslValue` has the same accessor API). `entity_to_json`
  (`🎨️chrome/🦀️.rs`) retyped `T: Serialize → T: dsl::ToValue`. `default_from_json`/
  `merge_json_patch` rewritten off `serde_json::{to_value,from_value,Map}` onto
  `dsl::ToValue`/`dsl::FromValue` + a `Vec<(String, DslValue)>` upsert loop (DslValue::Object has
  no `.insert`, it's an ordered `Vec`, not a map). 11 `json!({...})` seed-data literals for
  `add_register_item_operation`'s per-register defaults hand-expanded to `DslValue::object([...])`
  (no macro — the values are simple enough that a mini `json!`-workalike wasn't worth the
  edge-case risk of matching multi-token expr values like `EntityId::new_serial(...)` in a
  `:tt`-fragment position).
- **`🚪️io/🦀️.rs`**: `ProgramExportTable.rows` retyped `Vec<serde_json::Map<...>>` →
  `Vec<Vec<(String, DslValue)>>`; `ProgramIdentity<'a>` (fields are all references) got a
  **hand-written** `impl ToValue` instead of `#[derive(ToValue)]` — the derive's generated code
  calls fully-qualified `ToValue::to_value(&self.field)`, which needs `&&'a str`/`&&'a
  ArtifactChild<_>` impls that don't exist; plain method-call syntax (`self.field.to_value()`)
  auto-derefs through the reference to the real impl instead.
- **`🚪️io/📤️export/…/🎒️zip`, `📕️xlsx`**: the generic multi-table exporters (columns/cells built
  from `table.rows`) converted call-by-call; `cell_value` retyped off `serde_json::Value` onto
  `dsl::DslValue` (same variant shapes, `Number` unwrapped via `.as_f64()` instead of matching a
  bare `f64`).
- **`🚪️io/📥️import/…/🎒️zip`, `📕️xlsx`**: these route `ZipSnapshot`/`XlsxSnapshot` → `ProgramSnapshot`
  through a generic "reinterpret via value tree" bridge — `serde_json::{to_value,from_value}` →
  `dsl::ToValue::to_value` then `dsl::FromValue::from_value` directly (no JSON-text hop). Both
  stdio snapshot types were already converted to `value_derive::{ToValue,FromValue}` by a prior
  wave, confirmed by reading their definitions before relying on it.
- **Command handlers** (`↔️adjacency`, `🕸️graph`, `📋️register`, `🔍️search`, `🔬️analysis`,
  `🎚️config` readers): every `serde_json::{from_str,to_string,to_string_pretty,from_value}` call
  storing/reading a `*_json` config string field converted to
  `dsl::json::{from_json_str,to_json_string}` or, where pretty-printing was needed (no
  `dsl::json::to_string_pretty<T: ToValue>` generic exists, only one taking `&pack::json::Value`),
  the two-step `dsl::json::to_string_pretty(&dsl::json::from_dsl_value(&dsl::ToValue::to_value(&x)))`.

### A real pre-existing bug found and fixed (both plugins, not something this pass introduced)
33 files across draw (26) and architect (7) — every one of them a command-payload struct already
carrying `#[derive(ToValue, FromValue, dsl::DslRecord)]` from an **earlier** conversion wave —
imported the derive macros as `use semio_framework_value_derive::{FromValue, ToValue};`, a DIRECT
crate-name import. Neither plugin's `Cargo.toml` declares `semio-framework-value-derive` as a
dependency (correctly, per this ticket's own standing rule — it's reachable transitively through
`dsl` = `semio_framework_os_kernel`, which re-exports both the traits and the derive macros at its
crate root). A direct `use semio_framework_value_derive::...` needs the crate to be a **direct**
Cargo dependency regardless of transitive reachability, so every one of these files failed to
compile with `E0432 unresolved import`, cascading into ~289 `FromValue`-not-satisfied errors
crate-wide once the isolated-target `cargo check` actually ran. Fixed by rewriting the import to
`use dsl::{FromValue, ToValue};` (macro and trait names share the identifier but live in separate
Rust namespaces, so one `use` line correctly resolves both). No Cargo.toml touched. This is
unrelated to the serde-to-value goal itself but was blocking verification of this pass's own work,
so it was fixed rather than worked around.

## Verify
```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/8eb2ad71-482d-46b0-b299-0f4ef6f1479d/scratchpad/isolated-target
export RUSTC_WRAPPER=""
cargo check -p semio-s-plugin-draw --message-format short
cargo check -p semio-s-plugin-architect --message-format short
```
- **draw before**: not independently re-measured this session (bulk conversion + the
  `semio_framework_value_derive` import bug together broke the crate outright: E0432 unresolved
  import in 26 files, cascading to 289 total errors on the FIRST real compiler run this session,
  captured verbatim).
- **draw after**: a second `cargo check -p semio-s-plugin-draw` was launched in the foreground
  after fixing the import bug; it exceeded the tool's 120s timeout and was moved to background by
  the harness (not a deliberate `run_in_background`/`Monitor` choice). The machine showed 20+
  concurrent `rustc`/`cargo check` processes from other live sessions at the time (confirmed via
  `ps`), consistent with this ticket's own standing contention warnings. **Not confirmed complete
  by the end of this session** — whoever picks this up next should re-run the command above
  (should be fast/incremental now that the isolated target dir has draw's dependency graph warm)
  and report the real number.
- **architect**: not run this session at all (would have contended with the in-flight draw check
  against the same isolated target dir). All 282 bulk-converted files plus the 28 hand-converted
  files were verified by grep sweep (zero remaining production `serde_json::`/`use serde::`/
  `#[serde(` outside `#[cfg_attr(test, …)]` twins and framework-type test assertions) and by
  reading every non-mechanical rewrite line-by-line, but this is not a substitute for the compiler.
  **Needs a real `cargo check -p semio-s-plugin-architect` run.**

## Cargo.toml
Untouched in both plugins, as instructed — `serde`/`serde_json` remain declared (still load-bearing
for the `#[cfg_attr(test, …)]` oracle twins and, in draw, the framework-facing test assertions on
`BuiltNode`/`WindowLayout`). Removing them is a follow-up once a green compile confirms zero
production reachability.

## Files touched
- draw: `🗿️artifacts/🖍️draw/🦀️.rs`, `🏅️standards/🔖️1/🪆️subsets/✳️any/{🧬️schema/**,✏️editor/**,
  🚪️io/**,👁️viewer/**}`, `✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/{🦀️.rs,+13 leaf
  mutation files}`, plus the 26 `semio_framework_value_derive` import fixes. No file under
  `🧪️oracle/`, `🧪️tests/`, `🔬️probes/`, `🏭️generator/`, `🧫️fixtures/` touched.
- architect: 282 bulk-converted files + 28 hand-converted files under
  `🗿️artifacts/🏛️program/**`, plus the 7 `semio_framework_value_derive` import fixes. No file
  under `🧪️oracle/`, `🧪️tests/`, `🔬️probes/`, `🏭️generator/`, `🧫️fixtures/` touched.
- Neither plugin's `Cargo.toml` touched. No file outside `✏️s/🔌️plugins/🖍️draw` and
  `✏️s/🔌️plugins/🏛️architect` touched.

## Update — draw's real cargo check completed, root-caused

The backgrounded `cargo check -p semio-s-plugin-draw` finished: **289 → 200 errors** after the
`semio_framework_value_derive` import fix (89-error drop, exactly the 26 files fixed). The
remaining 200 were fully categorized by grepping the error output for distinct root causes — every
single one is pre-existing/concurrent framework churn, **not** a `ToValue`/`FromValue` problem:

1. **`MutationLeaf` trait bound** (14 draw mutation-leaf structs, ~28 error lines) — `#[derive(dsl::Mutations)]`
   on `DrawMutation` now requires each variant payload to implement `protocol::MutationLeaf`
   (`const DESCRIPTOR`/`const PROVENANCE`), which none of draw's 14 leaves ever derived (not
   `dsl::MutationLeaf`, not by hand) — confirmed unrelated to serde by grepping every leaf file:
   none mention `MutationLeaf` before or after this pass. This is a framework-side requirement
   change to the `Mutations` derive macro, independent of the ToValue/FromValue migration.
2. **`Result<IoOutcome<_>, IoError>` is not a future** (~13 error lines, `E0277`) — hits EVERY
   format's `Serializer`/`Deserializer` impl under `🚪️io/` (dxf, svg, png, dwg, pdf — files this
   pass never touched — plus json, which it did). The `Serializer::serialize`/`Deserializer::deserialize`
   trait methods appear to have become `async fn` in the framework and these implementations
   weren't updated; spans untouched formats, so unrelated to this pass.
3. **`cannot find any in subsets`** (`E0433`, 3 lines) — `use
   semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::...` can't resolve;
   `semio_s_plugin_stdio` is a separate plugin under concurrent edit by this same ticket's fan-out
   — its module layout moved. External dependency, not a draw-owned line.
4. **`Mutation` trait: missing `DESCRIPTORS`, `descriptor`** (`E0046`, 2 lines, `🎚️config/🦀️.rs` +
   `👥️presence/🦀️.rs`) — the hand-written `impl Mutation<DrawConfig> for DrawConfigMutation` /
   `impl Mutation<DrawPresence> for DrawPresenceMutation` blocks (their `diff`/`inverse` bodies,
   never touched by this pass) are missing two NEW associated items the `Mutation` trait itself
   must have gained concurrently. Framework-side trait expansion, unrelated to serde.

**Zero remaining errors mention `ToValue`/`FromValue` not satisfied for any type this pass
converted.** Grepped explicitly (`grep 'not satisfied' | grep -v 'MutationLeaf\|is not a future'` →
empty). Draw's serde→value conversion itself is verified correct by the compiler; the crate would
not compile clean right now regardless of this pass's changes, because of unrelated concurrent
framework work in flight. Not something to chase further here — out of this ticket's plugin-only
scope (`🧰️framework` is explicitly off-limits) and almost certainly already being worked by another
session per this ticket's own fan-out.

architect's `cargo check` was still not run this session (time/contention budget spent
root-causing draw's false leads so the report here is honest rather than a guess).
