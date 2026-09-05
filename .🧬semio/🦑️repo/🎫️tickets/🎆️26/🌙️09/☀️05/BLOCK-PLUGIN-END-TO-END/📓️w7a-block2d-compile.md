# 📓️ W7a — block2d compiles against the current framework API

Scope: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/**` only
(the 17 `◻️2d/` lines of `🗑️generated/check-lib-1.txt`). `🧊️3d/**` and `🖐️5d/**` untouched
(W7b/W7c). The crate entry `📦️packages/🦀️rust/🦀️.rs` needed no edit — all 60 errors in
`check-lib-1.txt` sit inside the three subsets.

Oracle: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/**`, plus
`🌀️procedural` and `📕️norm` where puzzle has no equivalent surface.

## Error classes, precedents and changes

### 1 — E0433 `cannot find mutation in move_camera2d` (2 files)

Precedent: `🧩️puzzle/…/✳️any/🧬️schema/🧬️mutations/📍move-node/↩️inverse/🦀️.rs:9` — the inverse
addresses the sibling builder by its absolute module path
(`crate::artifacts::puzzle2d::mutations::move_node::move_node`), never through a `mutation`
submodule (no such module exists in either plugin).

Changed — the two block2d inverses now use the same absolute path:

- `🧬️schema/🧬️mutations/🎥️move-camera2d/↩️inverse/🦀️.rs:8`
  `super::super::move_camera2d::mutation::move_camera2d` →
  `crate::artifacts::block2d::mutations::move_camera2d::move_camera2d`
- `🧬️schema/🧬️mutations/🔍️scale-camera2d/↩️inverse/🦀️.rs:8` — same shape for `scale_camera2d`.

### 2 — E0053 `command_from_action` expected `dsl::DslValue`, found `dsl::JsonValue`

Precedent: `🧩️puzzle/…/✏️editor/🦀️.rs:1881-1884` — the host wire value is a `dsl::DslValue`
and field reads go through `dsl::DslValue::get` / `dsl::DslValue::as_str`
(`🧰️framework/🔨️modules/🌱️value/🦀️.rs:166,187`).

Changed in `✏️editor/🦀️.rs`:

- signature `args: Option<&Value>` → `args: Option<&dsl::DslValue>`;
- `str_field`'s `.and_then(Value::as_str)` → `.and_then(dsl::DslValue::as_str)`;
- dropped the now-unused `use dsl::os_pack::json::Value;`.

### 3 — E0046 `DESCRIPTORS`/`descriptor` missing on Config/Presence

Precedent: `🧩️puzzle/…/✏️editor/🎚️config/🦀️.rs` (`DESCRIPTORS` + `descriptor`, one
`MutationLeafDescriptor` per aggregate variant) and `…/✏️editor/👥️presence/🦀️.rs:98-106`.
These enums carry no `dsl::Mutations` derive, so `protocol::Mutation`'s two associated items
are hand-written; the `owner` strings are registry metadata, not on-disk leaf directories
(the same "provisional" note puzzle carries).

Changed:

- `✏️editor/🎚️config/🦀️.rs` — 2 descriptors (`Snapshot`, `SetLocale`) + `descriptor` match;
- `✏️editor/👥️presence/🦀️.rs` — 1 descriptor (`Snapshot`) + `descriptor` match.

### 4 — E0053/E0308 `render` must return `Result<ComponentTree, …>`

The trait declarations are `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:26841`
(`ArtifactEditor::render`) and `:27063` (`ArtifactViewer::render`) — both sync, both
`UiAssemblyResult<ComponentTree>`. Precedent for the body shape:
`🧩️puzzle/…/✏️editor/🦀️.rs:2089-2112` and `🧩️puzzle/…/👁️viewer/🦀️.rs:68-74` — every arm
yields a `BuiltNode`, the unknown-body arm goes through
`semio_framework_plugin::built_text_node`, and the whole thing is wrapped once with
`built_to_component_tree`.

Changed:

- `✏️editor/🦀️.rs` render — arms now `?`-propagate, unknown body via `built_text_node`,
  result wrapped with `built_to_component_tree`;
- `👁️viewer/🦀️.rs` render — same shape; `Label`/`UiNode` dropped from the import list.

The two window bodies had to move off the retired `UiNode` tree helpers
(`ui_stack_vertical`/`ui_text`) onto the fixed-capacity contract builders. Precedent:
`📕️norm/🖥️app-surface/🦀️.rs:66-83` (`ui::column().try_children(children)?.try_build()`,
one `text(label).try_build()` per line) and `🌊️flow/…/🪟️windows/📝️form/🦀️.rs:142`.

- `✏️editor/🎭️modes/✏️edit/🪟️windows/📋️board/🦀️.rs` — `render` returns
  `UiAssemblyResult<BuiltNode>`, built from `column()` + a `line()` helper.
- `👁️viewer/🎭️modes/👁️view/🪟️windows/📋️board/🦀️.rs` — same, with the label/list helpers
  written locally rather than imported from the sibling editor module (viewer purity —
  see puzzle's viewer board docstring). Its test now unwraps the `Result` before
  serializing.

### 5 — E0599 `with_ports` on `impl Future<Output = AppIo>`

The `AppIo` builder is async now. Precedent: `🧩️puzzle/…/✏️editor/🦀️.rs:2055-2057` — puzzle
drives both stages with the framework's own sync poller
`semio_framework::io::resolve_ready` (`🧰️framework/🔨️modules/🚪️io/🦀️.rs:891`), because
`ArtifactEditor::io()` itself is sync.

Changed `✏️editor/🦀️.rs::block2d_io` to `resolve_ready(AppIo::from_document(…))` then
`resolve_ready(io.with_ports(vec![…]))`. The `"catalog:out"` port spec is unchanged.

### 6 — E0308 `ActionDescriptor` vs `Result<(ActionId, Option<UiValue>), …>` (inspection panel)

`block2d_action` had already been migrated to the `ActionFactory` shape
(`✏️editor/🦀️.rs:46`, mirroring `ActionFactory::action` at
`🧰️framework/…/🔌️plugin/🦀️.rs:6572`), but `📌️panels/🔍️inspection` still built raw
`UiNode::Field { child: UiNode::Input { on_change: … } }` values.

Precedent for an editable inspector in the contract API:
`🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/📌️panels/🔍️inspection/🦀️.rs:44-73` —
`ActionFactory::action(...)?` destructured into `(action, args)`, then
`input(InputKind::…).value(…).try_id(…)?.try_on_with(Trigger::Change, action, args)?.try_build()?`
wrapped in `field(label)?.try_child(control)?.try_build()?`, all pushed into a
`UiFixedList<BuiltNode>` and closed with `PanelTreeBuilder::…section(…)?.build()`.

`📌️panels/🔍️inspection/🦀️.rs` was rewritten to that shape: four `blur`-committed text
fields (`name`/`label`/`variant`/`description`, each bound to `patchNodeKind` with its
document field name) plus a read-only handle-count `tree_item`, inside one
`PanelTreeBuilder` section. The existing test (tree, not stack; contains "Name") is kept
verbatim.

### 7 — E0277 `Label: From<Label>` / `From<LabelText>` (artifact panel)

The prelude `Label` reachable as `semio_framework_plugin::Label` is still the renderer's
unbounded `ui_wgpu::wgpu::Label`; every builder/`tree_item*` helper takes the contract's
fixed-capacity `Label` (`semio_framework_plugin::plugin_app_close_prelude::Label`). There is
no `From` between them, and the terminology macro's `LabelText`
(`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🎗️label.rs:62`) converts to
neither.

Precedent: `🧩️puzzle/…/✏️editor/🦀️.rs:940-942` defines a subset-local `ui_label` that admits
a `String` into the contract `Label`, and every panel call site passes
`ui_label(labels.x.as_str())?` (`🧩️puzzle/…/📌️panels/🗿️artifact/🦀️.rs:78-80`,
`…/📌️panels/🔍️inspection/🦀️.rs:35-38`). `📕️norm` reaches the contract types without a new
Cargo dependency via `use semio_framework_plugin::plugin_app_close_prelude as ui;`
(`📕️norm/🖥️app-surface/🦀️.rs:16`) — block does the same, since block's `Cargo.toml` (unlike
puzzle's) has no direct `semio-framework-ui-contract` dependency and CLAUDE.md forbids
adding one.

Changed:

- `✏️editor/🦀️.rs` — added `ui_label` (contract `Label`) and `ui_text` (contract `UiText`),
  next to the existing `ui_value_*` / `ui_node_list` helpers.
- `📌️panels/🗿️artifact/🦀️.rs` — `Label` now imported from `plugin_app_close_prelude`; the
  four `Label::data(…)` / `labels.*.into()` / bare `LabelText` arguments replaced with
  `ui_label(…)?`.

### 8 — E0277 `[MutationMessage]: ToValue` (`🧬️schema/🧬️mutations/🦀️.rs:163`)

`ToValue` is implemented for `Vec<T>` and `[T; N]`
(`🧰️framework/🔨️modules/🌱️value/🔁️codec/🦀️.rs:218,251`) but not for the unsized slice, and
`MutationOutcome::messages()` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1171`)
returns `&[MutationMessage]`. Puzzle ships no test bridge, so the framework impl set is the
oracle. The sibling `inverseMessages` entry in the same object already passes a `Vec`.

Changed: `to_value(forward.messages())` → `to_value(&forward.messages().to_vec())`, making
both message entries structurally identical.

## Verification

Command (foreground, shared warm `target-block`):

```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-block \
CARGO_BUILD_JOBS=4 RUSTFLAGS=-Awarnings \
cargo check -p semio-s-plugin-block --lib --message-format=short
```

PENDING_CHECK

```
cargo test -p semio-s-plugin-block --lib block2d
```

PENDING_TEST

Tails saved to `🗑️generated/w7a-check.txt` and `🗑️generated/w7a-test.txt`.

## Files touched (all under `🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/`)

- `🧬️schema/🧬️mutations/🎥️move-camera2d/↩️inverse/🦀️.rs`
- `🧬️schema/🧬️mutations/🔍️scale-camera2d/↩️inverse/🦀️.rs`
- `🧬️schema/🧬️mutations/🦀️.rs`
- `✏️editor/🦀️.rs`
- `✏️editor/🎚️config/🦀️.rs`
- `✏️editor/👥️presence/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/📋️board/🦀️.rs`
- `✏️editor/📌️panels/🗿️artifact/🦀️.rs`
- `✏️editor/📌️panels/🔍️inspection/🦀️.rs`
- `👁️viewer/🦀️.rs`
- `👁️viewer/🎭️modes/👁️view/🪟️windows/📋️board/🦀️.rs`

Retained-factory, boot-snapshot and io work already in these files was left intact — no
shims, adapters or compatibility layers were added.
