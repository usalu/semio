# 📓️ W4 — raster compiles against the current framework API

Slice: the framework-API drift block of `🗑️generated/check-native-1-errors.txt` (26 of the 29 baseline
errors), plus the two `🧬️schema/🧬️mutations/💾️binary/🦀️.rs` errors. Untouched by design: W1's
`create_raster_app` builder region and `🔏️publication-authority`, W2's `initial_snapshot`/examples/
`🎬️set-active-example`/crate-root mounts (the duplicate `set_active_example` E0428 is theirs), W3's
`🚪️io/**` and per-format artifact leaves.

Oracles: `BLOCK-PLUGIN-END-TO-END/📓️w7a-block2d-compile.md` (identical classes 3, 4, 6, 7),
`✏️s/🔌️plugins/🧩️puzzle/…/◻️2d/**`, `✏️s/🔌️plugins/🌊️flow/…/🌊️main/**`,
`✏️s/🔌️plugins/🌍️gis/…/🧬️mutations/💾️binary/🦀️.rs`, and the framework definitions themselves
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`,
`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🏗️builder.rs`).

No shim, `From` impl, adapter or compatibility layer was added anywhere; no new Cargo dependency.

## Error classes, framework change, and fix

### 1 — E0277 `Label: From<LabelText> / From<&str> / From<semio_framework_plugin::Label>` (15 sites)

**Framework change.** Two distinct `Label` types are reachable from the plugin crate: the renderer's
unbounded `ui_wgpu::wgpu::Label` (re-exported at the crate root as `semio_framework_plugin::Label`,
built with `Label::data(..)`), and the semantic UI contract's fixed-capacity
`semio_framework_plugin::plugin_app_close_prelude::Label` (`Label(pub UiText)`,
`🧬️contract/…/🧩️component.rs:36`). Every `PanelKit` helper — `tree_item`, `tree_item_desc`,
`tree_item_with_action`, `PanelTreeBuilder::section` / `::section_or_placeholder`
(`🔌️plugin/🦀️.rs:5824-5860`, `:5944-5960`) — now takes the **contract** `Label`. There is no `From`
between the two, and the `app_labels!` macro's `LabelText` converts to neither.

**Fix.** A subset-local admitting helper (puzzle's `ui_label` / block's `ui_label` precedent), reached
through the prelude so no `semio-framework-ui-contract` dependency is added:

- `✏️editor/🦀️.rs:126` — new `pub fn ui_label(value: impl AsRef<str>) -> UiAssemblyResult<plugin_app_close_prelude::Label>`
  (`Label::try_from(String)` is the contract's only admission).

Call sites, all now `ui_label(…)?` (`LabelText` reaches it via `.as_str()`):

- `✏️editor/📌️panels/🗿️artifact/🦀️.rs:44` (layer row label), `:60` (add-layer rows), `:70` (section label)
- `✏️editor/📌️panels/🔍️inspection/🦀️.rs:33,34,36`
- `✏️editor/📌️panels/🛍️catalogue/🦀️.rs:26,27,28,30`
- `✏️editor/📌️panels/🎭️masks/🦀️.rs:24,50` (section + placeholder)

`Label` was dropped from the `semio_framework_plugin::{…}` import list in the artifact and masks panels.

### 2 — E0046 `DESCRIPTORS` / `descriptor` missing on `RasterConfig` / `RasterPresence`

**Framework change.** `protocol::Mutation` gained two associated items, `const DESCRIPTORS: &[MutationLeafDescriptor]`
and `fn descriptor(&self)`. The `dsl::Mutations` derive supplies them; these two enums carry no derive,
so they are hand-written — exactly as block2d/puzzle2d do, with `owner` strings that are registry
metadata rather than on-disk leaf directories.

- `✏️editor/🎚️config/🦀️.rs:186` — 7 descriptors (`Snapshot`, `SetBrushSize`, `SetBrushOpacity`,
  `SetCompositeViewport`, `SetCamera`, `SetActiveUtility`, `SetLocale`) + the `descriptor` match.
- `✏️editor/👥️presence/🦀️.rs:97` — 1 descriptor (`Snapshot`) + the `descriptor` match.

### 3 — E0053/E0308 `render` must return `Result<ComponentTree, …>`

**Framework change.** `ArtifactEditor::render` and `ArtifactViewer::render` are
`UiAssemblyResult<ComponentTree>` now; window/panel bodies yield `BuiltNode`, the unknown-body arm goes
through `built_text_node` (`🔌️plugin/🦀️.rs:357`), and the whole thing is closed once with
`built_to_component_tree` (`:346`).

- `👁️viewer/🦀️.rs:73` — signature + body reshaped to the block2d/puzzle2d shape; `UiNode` dropped from
  the import list.
- `✏️editor/🦀️.rs:830` — same reshape (arms `?`-propagate, `built_text_node` fallback,
  `built_to_component_tree` at the end); `UiNode` dropped from the import list.

That forced the four window bodies off the retired `UiNode` builders:

- `✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/🦀️.rs:47` and `…/🧭️navigator/🦀️.rs:39` — the wgpu
  `build_paint_2d_scene(surface_id, controller_id, scene) -> UiNode`
  (`🖱️ui/…/🧊️wgpu/🧩️component.rs:3935`) is the legacy `UiNode` path no other plugin still uses; both now
  call `scene_surface(surface_id, ContractSurfaceKind::Paint2d, &scene) -> UiAssemblyResult<BuiltNode>`
  (`🔌️plugin/🦀️.rs:351`), the same call flow/procedural/trinity/block3d use. The controller id is
  dropped because the semantic surface node carries none — the host resolves the owning app instance.
  `Paint2dScene` is the *same* struct on both paths (`ui_scene` re-exported through `ui_wgpu::wgpu`), so
  the payload is byte-identical; only the envelope changed. `SurfaceKind::Paint2d` in the two
  `WindowKindDefinition`s is untouched (that field is still the wgpu enum) — hence the
  `ContractSurfaceKind` alias for the surface-encode call only.
- `👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️composite/🦀️.rs:36` and `…/🧭️navigator/🦀️.rs:47` — see class 5.

### 4 — E0308 `ActionDescriptor` vs `Result<(ActionId, Option<UiValue>), …>` (4 sites)

**Framework change.** `raster_action` had already been migrated to `ActionFactory::action`, which returns
the semantic-UI pair `UiAssemblyResult<(ActionId, Option<UiValue>)>`. But `WindowMeasure::Slider.on_change`
is a **renderer-side** record field of type `Option<ActionDescriptor>` (`🧊️wgpu/🧩️component.rs:28`) —
window chrome is not a semantic-UI node, so the two surfaces genuinely need different builders (flow's
`grid_action`, block3d's `block3d_window_action` are the precedent).

- `✏️editor/🦀️.rs:118` — new `pub fn raster_measure_action(action: &str) -> Option<ActionDescriptor>`.
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🖌️brush/🦀️.rs:36,50` and
  `…/🧽️eraser/🦀️.rs:37,51` — `raster_action("setBrushSize"/"setBrushOpacity", None)` →
  `raster_measure_action("…")`; the import switches with it.

### 5 — `BuiltNode` vs `Result<BuiltNode, PluginAssemblyError>`, and `no field base on BuiltNode`

**Framework change.** (a) `WindowKit::render` is fallible now — `ImageWindowKit::render(&ImageView)`
returns `UiAssemblyResult<BuiltNode>` (`🔌️plugin/🦀️.rs:26212`). (b) The subset-local `ui_node_list`
takes an iterator of *`Result`s* and does the `?` itself, so call sites must not pre-`?` their items.
(c) `BuiltNode` was flattened: there is no `base` field any more, and `children` is a
retained-page-backed `BuiltChildren` (`🏗️builder.rs:151`) whose only writer is `try_push`.

- `👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️composite/🦀️.rs:36` and `…/🧭️navigator/🦀️.rs:47` — signatures
  `UiNode` → `UiAssemblyResult<BuiltNode>` (bodies unchanged, they just forward the kit's result); both
  window tests now `.expect("bounded fixture")`.
- `✏️editor/📌️panels/🔍️inspection/🦀️.rs:31-34` and `✏️editor/📌️panels/🛍️catalogue/🦀️.rs:24-28` — the
  `?` after each `tree_item_desc(...)` inside the `ui_node_list([...])` array removed.
- `✏️editor/📌️panels/🗿️artifact/🦀️.rs:34-55` — `layer_tree_item` keeps building the row with
  `tree_item_desc` + the `Component::TreeItem` chrome patch, but the group recursion now pushes each
  nested row into `node.children` with `BuiltChildren::try_push` instead of assigning
  `node.base.children`. A small `row_text` helper admits the icon/description `UiText`s.

### 6 — `🧬️schema/🧬️mutations/💾️binary/🦀️.rs` (2 errors, raster-internal)

**What broke.** `ArtifactStoreInitializationAuthority::close_step(&mut self, maximum_items, maximum_bytes)`
(`🔌️plugin/🦀️.rs:13678`) carries **no** `StepContext` — but raster's `pump_terminal_retirement` /
`pump_active` had been given one (they spend step fuel via `raster_reserve_unit`), so `close_step`
could not call them (E0061 at `:3988`). Separately `ArtifactOwnedDisposer::close_step`
(`:13542`) returns `Result<_, Fault>`, which `?` cannot lift into these pumps' `String` error (E0277 at
`:3480`). The gis oracle has no context on either pump at all, so there is no framework-side answer to
copy — only raster reserves fuel here.

**Fix.**

- `:1112` — new `raster_reserve_granted_unit(cx: Option<&mut StepContext>)`: `Some` spends the caller's
  step fuel exactly as before, `None` reserves unconditionally because on the `close_step` path the
  caller's own `maximum_items`/`maximum_bytes` grant *is* the unit (already checked at the top of
  `close_step`). No fabricated context, no second budget.
- `:3455` / `:3480` — `pump_active` and `pump_terminal_retirement` take `Option<&mut StepContext<'_>>`
  and thread it with `.as_deref_mut()`; the five internal reservation sites move to the new helper.
- `:3480` — the disposer's `Fault` is mapped to the pumps' `String` verbatim
  (`format!("{}: {}", fault.code.0, fault.message)`) rather than dropped.
- Call sites: `:3610` `pump_active(Some(cx))`, `:3945` `pump_terminal_retirement(Some(cx))`,
  `:3999` `pump_terminal_retirement(None)`.

## Preserved decisions (`📓️explore-raster-history-and-prior-tickets.md` §5)

Camera stays session-only (`RasterConfig.camera`, still `SetCamera`/`SetCameraZoom` config mutations —
descriptors added, semantics untouched); `RasterPlayApp` keeps its name and its single-controller
`app_commands!` channel; both editor windows keep `SurfaceKind::Paint2d` in their
`WindowKindDefinition`s.

## Verification

Command (foreground, shared `target-s-e2e`, shared cargo lock):

```
cd /Users/ueli/Documents/semio && CARGO_TARGET_DIR=target-s-e2e RUSTC_WRAPPER="" \
cargo check -p semio-s-plugin-raster --lib --message-format short
```

PENDING_CHECK

## Files touched

- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `…/✳️any/✏️editor/🎚️config/🦀️.rs`
- `…/✳️any/✏️editor/👥️presence/🦀️.rs`
- `…/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs`
- `…/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs`
- `…/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs`
- `…/✳️any/✏️editor/📌️panels/🎭️masks/🦀️.rs`
- `…/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/🦀️.rs`
- `…/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🖌️brush/🦀️.rs`
- `…/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️composite/☑️options/🧽️eraser/🦀️.rs`
- `…/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧭️navigator/🦀️.rs`
- `…/✳️any/👁️viewer/🦀️.rs`
- `…/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️composite/🦀️.rs`
- `…/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧭️navigator/🦀️.rs`
- `…/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs`
