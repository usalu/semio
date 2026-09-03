# Fix flow `✏️editor/🎭️modes/` compile errors — serde_json → DSL migration

Scope: `semio-s-plugin-flow`, only under
`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/`.

## Files touched

- `🧬️generate/🎮️commands/🧬️update-generation-values/🦀️.rs`
- `🧬️generate/🎮️commands/🧬️select-generation/🦀️.rs`
- `🧬️generate/🎮️commands/🧬️rename-generation/🦀️.rs`
- `🧬️generate/🎮️commands/🧬️remove-generation/🦀️.rs`
- `🧬️generate/🎮️commands/🧬️add-generation/🦀️.rs`
- `✏️edit/🪟️windows/🌊️main/🎚️options/🌐️grid/🦀️.rs`
- `✏️edit/🪟️windows/🌊️main/🎚️options/🔭️lod/🦀️.rs`
- `✏️edit/🪟️windows/🌊️main/🦀️.rs`

## Conversion helpers found and used

- `dsl::json::to_json_string<T: ToValue>(value: &T) -> String` —
  `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:1418`. Replaces `serde_json::to_string(fixture)`
  wherever `fixture: &FlowSnapshot` (which no longer derives `serde::Serialize`); call it on
  `fixture.to_fixture()` (`FlowFixture` derives `ToValue`/`FromValue`,
  `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📄️artifact/🦀️.rs:303`).
- `dsl::json::from_dsl_value(value: &DslValue) -> dsl::json::Value` —
  `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:539`. Per-entry bridge used to build a
  `dsl::json::Object` (= `pack::JsonObject`, aliased at `🧰️framework/🔨️modules/🎒️pack/🦀️.rs:42`)
  out of a `HashMap<String, DslValue>`.
- `dsl::json::Object: FromIterator<(String, dsl::json::Value)>` —
  `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:232`. Lets `values.iter().map(...).collect()`
  produce the `&JsonObject` that `apply_generation_values_to_fixture` wants.
- `dsl::DslValue::object(entries) -> DslValue` and `DslValue::String(...)` /
  `DslValue::Null` — `🧰️framework/🔨️modules/🌱️value/🦀️.rs:194` / enum at line 100. Used to build
  command args directly as `DslValue` instead of `serde_json::json!`.
- `semio_framework::optional_json_to_dsl(Option<serde_json::Value>) -> Option<DslValue>` —
  `🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs:803`, re-exported at
  `🧰️framework/📦️packages/🦀️rust/🦀️.rs:2035`. Used once, in `lod/🦀️.rs`, to bridge the
  `serde_json::json!({"value": …})` payload (still needed because the value being wrapped —
  `config.lod_mode`, a plain `String`) into `ActionDescriptor.args: Option<DslValue>`.
- All five discovered by reading the exact sibling pattern at
  `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/…/🧬️schema/🦀️.rs:363-370`
  (`evaluate_generation_preview`) and
  `…/generation2d/…/✏️editor/🎮️commands/🧬️update-generation-values/🦀️.rs` (fully DSL-native
  command `handle`), and the working
  `✏️s/🔌️plugins/🏭️process/…/🎚️options/☀️sun/🦀️.rs` for the `ActionDescriptor` construction
  pattern.

## Generate-command chain — one value type end to end

All five `🧬️generate/🎮️commands/*` files duplicate the same `//#region 🔖️SharedDispatch` block
(`evaluate_generation_preview` + `handle_generation`). In every file:

- `evaluate_generation_preview`'s `values` parameter changed from
  `&serde_json::Map<String, Value>` to `&crate::playbook::PlaybookValues`
  (`= HashMap<String, DslValue>`, re-exported from `flow::playbook`). Body now serializes the
  fixture via `dsl::json::to_json_string`, converts each `DslValue` entry via
  `dsl::json::from_dsl_value`, and passes the resulting `dsl::json::Object` to
  `apply_generation_values_to_fixture` — matching that function's real signature
  (`fixture_json: &str, values: &crate::os_pack::json::Object`,
  `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs:2756`).
- `handle_generation`'s `args` parameter changed from `Option<&serde_json::Value>` to
  `Option<&dsl::DslValue>`, matching `handle_generation_action`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️.rs:468`).
- `serde_json::to_string(&generation)` at the end of `handle_generation` was **left untouched** —
  `GenerationPlayState` still derives `serde::Serialize` (playbook.rs:391-392), so that call is
  not part of the mismatch and needed no change.
- Each command's own `handle()` now builds its action args directly as `dsl::DslValue::object([…])`
  instead of `serde_json::json!({…})`, so no serde_json value ever crosses the
  `handle_generation`/`handle_generation_action` boundary:
  - `add-generation`: no args, unchanged (`None`).
  - `select-generation`, `remove-generation`: `[("id", DslValue::String(...))]`.
  - `rename-generation`: `[("id", ...), ("name", ...)]`.
  - `update-generation-values`: previously called `dsl::from_dsl_value(payload.value.clone())`
    (the *generic* `from_dsl_value<T: FromValue>` at `🧰️framework/🔨️modules/🌱️value/🦀️.rs:319`,
    which cannot target `serde_json::Value` since it doesn't implement the DSL `FromValue` trait —
    this was the file's 5th error). Replaced entirely: `payload.value` is already a `DslValue`, so
    it's spliced straight into `DslValue::object([...])` alongside `generationId`/`questionId`,
    with no conversion call at all.

## `FlowSnapshot: serde::Serialize` replacement

`✏️edit/🪟️windows/🌊️main/🦀️.rs:87`: `serde_json::to_string(fixture).ok()` (where
`fixture: &FlowSnapshot`, which no longer derives `Serialize`) became
`Some(dsl::json::to_json_string(&fixture.to_fixture()))` — same `to_fixture()` conversion the rest
of the file already uses (e.g. `flow_backed_node_graph_extras(&fixture.to_fixture(), …)` two lines
below), and `NodeGraphScene.fixture_json` is `Option<String>`
(`🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️scenes.rs:828`), matching `Some(...)`.

## `✏️edit/🪟️windows/` option files — same family, different shape

`grid/🦀️.rs` and `lod/🦀️.rs` errors were **not** the JSON/DSL value-type mismatch but a stale
caller of `flow_action` (owned by `✏️editor/🦀️.rs`, out of my scope — untouched). `flow_action`
still legitimately returns `UiAssemblyResult<(ActionId, Option<UiValue>)>` for its other caller
(`✏️editor/📌️panels/🛍️catalogue/🦀️.rs`, out of scope), but `WindowMeasure::{Toggle,Slider,Select}.on_change`
now wants a bare `ActionDescriptor { controller_id, action, args: Option<DslValue> }` directly
(confirmed against the struct at
`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs:28` and the
already-clean sibling `✏️s/🔌️plugins/🏭️process/…/🎚️options/☀️sun/🦀️.rs`). Fixed by constructing
`ActionDescriptor` directly at each call site instead of going through `flow_action`:

- `grid/🦀️.rs`: added a local `fn grid_action(action: &str) -> ActionDescriptor` (args always
  `None` for the three grid toggles/slider), replacing the three `flow_action(..., None)` calls.
- `lod/🦀️.rs`: built the `on_change` inline, using
  `optional_json_to_dsl(Some(json!({ "value": config.lod_mode })))` for `args` (kept the existing
  `serde_json`/`json!` import — only the `on_change` wiring was broken, not the
  `serde_json::from_str::<Vec<Value>>(&dag_lod_scale_json())` parse a few lines above, which had no
  error and was left as-is).

`📏️proximity/🦀️.rs` has the identical `flow_action` mismatch (1 error) but is **not** in my file
list, so it was left untouched.

## Unresolved / unverified

- **Not compiled.** Per instructions I did not run `cargo` at any point; every fix above is based
  on reading the real definitions (`dsl::json`, `DslValue`, `PlaybookValues`,
  `apply_generation_values_to_fixture`, `ActionDescriptor`, `NodeGraphScene`) and copying an
  already-consistent working sibling pattern (`generation2d`'s DSL-native generate commands,
  `process3d`'s `☀️sun` options file) line-for-line where the shapes matched exactly. The
  coordinator should verify centrally.
- `📏️proximity/🦀️.rs` (same `flow_action`/`ActionDescriptor` family, 1 error) is untouched —
  out of my assigned file list.
- `flow_action` itself (`✏️editor/🦀️.rs`, owned by another agent) was not touched; it still
  returns the old `(ActionId, Option<UiValue>)` tuple form, which is correct for its other caller
  in `✏️editor/📌️panels/🛍️catalogue/🦀️.rs` (also out of scope) but is no longer used by any file
  in my scope.
