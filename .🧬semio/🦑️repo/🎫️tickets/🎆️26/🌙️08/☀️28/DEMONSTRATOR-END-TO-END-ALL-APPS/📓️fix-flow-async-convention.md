# Fix compile errors — `semio-s-plugin-flow`

Scope given: two files, 20 claimed errors total (10 + 10). Actual finding: only
1 real compile error lived at the crate root, and the artifacts file's 10
errors were **not** the async-convention family described in the brief — they
were a separate, already-documented repo migration (serde → `ToValue`/
`FromValue`). Details below.

## 1. Crate root — `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/🦀️.rs:614`

```
error[E0308]: mismatched types
  semio_framework_plugin::plugin_exports!(plugin::plugin, plugin::FlowApps);
  expected `Result<Plugin<FlowApps>, ...>`, found future
```

Only **one** genuine error attributed to this file (the other 9 hits the
coordinator's grep found at `../../🦀️.rs:9/12/12/13/13/24/28` were compiler
**warnings** — unused doc comment, unnecessary qualification — not errors).

Root cause traced through `#[path = "../../🦀️.rs"] mod plugin;`: the actual
callee is `pub async fn plugin()` in **`✏️s/🔌️plugins/🌊️flow/🦀️.rs`** (the
plugin-root file, one directory up from the two files listed in scope — not
itself in the given file list, but not owned by any other agent either, and
it is the only place the fix can land per "fix the callee"). Its body is a
single synchronous builder chain (`Plugin::<FlowApps>::builder(...)....
try_build()`) with no `.await` anywhere — confirmed `PluginBuilder::try_build`
(`🧰️framework/…/🔌️plugin/🏗️builder/🦀️.rs:591`) returns
`Result<Plugin<PA>, PluginAssemblyError>` directly, not a future, and the
macro's expansion calls `install_plugin_bundle_result(runtime, ($bundle_fn)())`
— a direct synchronous call, no `.await`.

**Fix**: removed `async` from `pub async fn plugin()` →
`pub fn plugin() -> Result<Plugin<FlowApps>, semio_framework_plugin::PluginAssemblyError>`.

**Working sibling model**: `✏️s/🔌️plugins/📐️cad/🦀️.rs:25` and
`✏️s/🔌️plugins/🪵️sourcing/🦀️.rs:33` both declare `pub fn plugin() -> Result<Plugin<...>, ...>`
(non-async) with the identical builder-chain shape — confirms this is the
established convention, not a one-off.

No other call site in the repo invokes `plugin::plugin()` with `.await` or
otherwise depends on it being async (checked via grep across
`✏️s/🔌️plugins/🌊️flow/` and `🧰️framework/…/🔌️plugin/`), so this is a
self-contained, non-cascading fix.

**Deviation from stated scope**: this required editing
`✏️s/🔌️plugins/🌊️flow/🦀️.rs`, which was not one of the two listed files. I
made this call because (a) the assigned crate-root error is unfixable any
other way — the type mismatch originates entirely at the async fn's
definition, not at the macro call site — and (b) this file is not claimed by
any of the four excluded areas (`🧬️schema/🧬️mutations/`,
`✏️editor/🎭️modes/`, `✏️editor/📌️panels/`, `✏️editor/🧵️retained/`). Flagging
this explicitly in case another agent is also touching this file.

## 2. Artifacts file — `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs`

All 10 errors here are `E0277` **not** `E0308`/opaque-future errors, and none
of them are async-convention fallout. They are all
`the trait bound X: serde::Serialize`/`Deserialize<'de>` is not satisfied` for
four types used inside `Widget` variant fields:

- `Widget::Neuron.params: Dictionary` (line 80 encode, line 107 decode)
- `Widget::OutputPreview.preview: Dictionary`, `.expanded: OrderedSet`
  (line 89 both encode; line 116 both decode)
- `Widget::Cluster.tree: Tree`, `.flow: FlowGui` (= `FlowUi` alias)
  (line 92 both encode; line 119 both decode)

`widget_params`/`widget_from_node` called `serde_json::to_string`/
`from_str` directly on these fields. `Dictionary`, `OrderedSet`, `Tree`, and
`FlowUi` all deliberately dropped their non-test `Serialize`/`Deserialize`
impls in ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`
(see the doc comment on `Dictionary` in
`🧰️framework/…/🧠️neural/⚙️engine/🦀️.rs:26-34`, and on `FlowUi` in
`🧰️framework/…/🌊️flow/📄️artifact/🦀️.rs:83-88`) — production code now routes
through hand-written/derived `ToValue`/`FromValue` instead, with the
`serde_json`-shaped replacement being
`pack::json::to_json_string`/`from_json_str` (`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:1418-1427`,
doc-linked to the same ticket). This same plugin crate's own
`✏️editor/🦀️.rs:2030` already calls this exact replacement as
`flow::os_pack::json::to_json_string(&manifest)` — confirmed `flow` (the
`semio-framework-os-flow` crate dependency) re-exports `os_pack::json` with
both functions.

**Fix** — replaced the 6 failing call sites (all `serde_json::to_string(...).unwrap_or_default()`
/ `serde_json::from_str(...).unwrap_or_default()` on the 4 named types) with
`flow::os_pack::json::to_json_string(x)` (returns `String` directly, no
`unwrap_or_default` needed) and `flow::os_pack::json::from_json_str(&text).unwrap_or_default()`
(returns `Result<T, ValueError>`, `T: Default` holds for all four types —
confirmed each derives/has `Default`). Left `input_ports`/`output_ports`
(`Vec<String>`) and the `#[cfg(test)] mod tests` block's `serde_json` usage
untouched — those are not implicated by any of the 10 errors and `Vec<String>`
has ordinary `serde` support; the test module uses `serde_json` only under
`#[cfg(test)]`, matching this repo's own established pattern of test-only
serde.

## Unresolved / unverified

- **Not run `cargo check`** per instructions — all fixes are statically
  reasoned from reading callee definitions, trait impls, and a working
  sibling plugin. Compilation success is unverified.
- Both `Dictionary::from_value`/`Tree::from_value`/`OrderedSet::from_value`/
  `FlowUi::from_value` round-tripping through `pack::json::from_json_str`
  exactly matches the old `serde_json` wire shape byte-for-byte is asserted
  by the framework's own doc comments (`from_dsl_value`/`to_dsl_value`
  bridge) but not independently re-verified here.
- If another agent's fix to `plugin_exports!`, `PluginBuilder`, or the
  `Widget`/`Dictionary`/`Tree`/`OrderedSet`/`FlowUi` type definitions lands
  concurrently, these two edits may need re-checking against the new shapes.
- The `✏️s/🔌️plugins/🌊️flow/🦀️.rs` edit (outside originally stated scope) —
  flagging again here for visibility.

## Files touched

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/🦀️.rs` — read only, no edit needed (the one real error there was fixed at its actual definition site).
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs` — edited, 6 call sites.
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌊️flow/🦀️.rs` — edited, 1 line (`async fn plugin` → `fn plugin`); outside originally stated scope, see deviation note above.
