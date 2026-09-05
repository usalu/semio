# 📓️ W7b — block3d compile repair (framework API drift)

Subset: `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/**` only. Oracles: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/**`
(same layout, compiles), plus `💠️lowpoly` and `🌊️flow` where puzzle has no equivalent (editable inspector
fields, dual action factories). No shims, no adapters, no compat layers — every fix moves block3d onto the
current framework shape.

## Error classes and the precedent mirrored

### 1 · `↩️inverse` E0433 `cannot find mutation in move_camera3d` (2 sites)

Both camera inverse files reached their sibling builder through a `mutation` submodule that block3d's leaves
do not have (block3d declares the payload struct + builder inline in the leaf's own `🦀️.rs`; puzzle3d splits
them into a `mutation` submodule — see `🧩️puzzle/…/🧬️mutations/🔃rotate-object/↩️inverse/🦀️.rs:9`, whose leaf
DOES have that submodule). Every one of block3d's other 33 inverse leaves already used the flat spelling
`super::super::<leaf>::<builder>(…)` — the two camera files were the outliers.

- `🧬️schema/🧬️mutations/🎥move-camera3d/↩️inverse/🦀️.rs:8` — `move_camera3d::mutation::move_camera3d` → `move_camera3d::move_camera3d`
- `🧬️schema/🧬️mutations/🔍scale-camera3d/↩️inverse/🦀️.rs:8` — same, `scale_camera3d`

### 2 · `command_from_action` takes `dsl::DslValue`, not `dsl::JsonValue` (E0053)

Precedent: `🧩️puzzle/…/✏️editor/🦀️.rs:6862` — the trait's wire arg is a `&dsl::DslValue` now, and puzzle
converts once at the boundary with `dsl::os_pack::json::from_dsl_value` before its own body reads JSON
accessors.

- `✏️editor/🦀️.rs:777` — signature changed to `args: Option<&dsl::DslValue>`; two added lines convert to the
  owned `Value` the existing body (`str_field`, `f64_vec3_field`, `window_id_from_args`, ~25 arms) already
  speaks. No arm bodies touched, so the `setCamera` forward-fix and every `worldSurface*` id divergence stay
  exactly as landed.

### 3 · `Mutation` impls missing `DESCRIPTORS` / `descriptor` (E0046, 2 sites)

Precedent: `🧩️puzzle/…/✏️editor/🎚️config/🦀️.rs:471-513` and `…/👥️presence/🦀️.rs:99-107` — hand-written
`MutationLeafDescriptor` tables (these enums predate `#[derive(dsl::Mutations)]`; their `diff`/`inverse` is a
plain `match`, not the derive's per-leaf `MutationKind` shape), one row per variant in declaration order, and
a `descriptor()` that indexes `Self::DESCRIPTORS` by variant. Puzzle marks its `owner` paths
`⚠️ PROVISIONAL` (no leaf directory exists on disk); block3d's are provisional in exactly the same way and
carry the same note.

- `✏️editor/🎚️config/🦀️.rs` — 14 rows (`Snapshot`, `SetActiveRepresentation`, `SetWantedTags`, `SetLocale`,
  `SetWindowRepresentations`, `ToggleWindowRepresentation`, `SetWindowArrangement`, `SetWindowSpacing`,
  `SetActiveUtility`, `SetBrushVortexKind`, `SetBrushRadius`, `SetBrushFlip`, `SetBrushPreview`, `SetCamera`)
  + the 14-arm `descriptor()`.
- `✏️editor/👥️presence/🦀️.rs` — 1 row (`Snapshot`) + its `descriptor()`.

### 4 · `render` returns `UiAssemblyResult<ComponentTree>`, not `UiNode` (E0053/E0308)

Precedent: `🧩️puzzle/…/👁️viewer/🦀️.rs:65-71` (viewer) and `…/✏️editor/🦀️.rs:6988-7027` (editor) — every body
render returns `UiAssemblyResult<BuiltNode>`, the unknown-body arm is `built_text_node(..)` mapped to a
`PluginAssemblyError`, and the trait method closes with `Ok(built_to_component_tree(node))`. The legacy
`ui_wgpu` `UiNode` tree and its `build_world_3d_scene`/`ui_text` constructors are gone from every plugin
render path; scenes go through `scene_surface(surface_id, SurfaceKind::World3d, &scene)`
(`🧩️puzzle/…/🪟️windows/🧊️main/🦀️.rs:497`).

- `✏️editor/🦀️.rs:878-889` — match now binds a `BuiltNode` and wraps once at the end.
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🦀️.rs` — `render` → `UiAssemblyResult<BuiltNode>` via
  `scene_surface(BLOCK3D_PLAY_SURFACE_ID, SurfaceKind::World3d, &scene)`; `build_world_3d_scene` and the now
  unused `BLOCK3D_PLAY_APP_ID` import dropped (the surface contract no longer carries a controller id —
  puzzle's world window does not pass one either).
- `👁️viewer/🦀️.rs` — same shape as puzzle's viewer, unknown-body arm included.
- `👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️world/🦀️.rs` — `scene_surface(SURFACE_ID, …)`;
  `BLOCK3D_VIEW_CONTROLLER_ID` is retained because `vortices_json` still stamps it into each `fullId`.

### 5 · `with_ports` on `impl Future<Output = AppIo>` (E0599)

`AppIo::from_document` and `AppIo::with_ports` are both `async fn` now (`🛂️manifest/🦀️.rs:5337,5342`).
Precedent: `🧩️puzzle/…/✏️editor/🦀️.rs:7106-7123` wraps each await point in
`semio_framework::io::resolve_ready(..)` — nested twice, because the builder call and the port call are two
separate futures.

- `✏️editor/🦀️.rs:164` — `block3d_io()` now `resolve_ready(resolve_ready(AppIo::from_document(..)).with_ports(..))`.
  The `"catalog:out"` port spec is unchanged, so `block3d_io_declares_the_catalog_out_port` and
  `export_media`'s puzzle3d seam keep their contract.

### 6 · `ActionDescriptor` vs `Result<(ActionId, Option<UiValue>), …>` (E0308, 9 sites)

There are now TWO action factories, not one, and block3d only had the contract-node flavour. Precedent:
`💠️lowpoly/…/✏️editor/🦀️.rs:50` (`lowpoly_action` → `UiAssemblyResult<(ActionId, Option<UiValue>)>`, for
contract-built nodes) alongside `:55` (`lowpoly_window_action` → `ActionDescriptor`, "bridges window chrome,
which still carries the retained WGPU action descriptor"). `WindowMeasure::{Select,Slider,Toggle}.on_change`
is an `ActionDescriptor` (`🧊️wgpu/🧩️component.rs:1026`), whose `args` is `Option<DslValue>`.

- `✏️editor/🦀️.rs:71` — added `block3d_window_action(action, args: Option<dsl::DslValue>) -> ActionDescriptor`
  next to the existing `block3d_action`; args are built as `dsl::DslValue::object([..])` (cad/lowpoly
  spelling), with no `serde_json` on the runtime path.
- The 5 `🪟️windows/🌐️world/☑️options/*` measures (`↔️arrangement`, `📏️spacing`, `🔀️quick-representation`,
  `🧱️representations`, `🖌️brush` ×3) now call `block3d_window_action`. Same action ids, same arg keys
  (`windowId`/`representationId`/`visible`/`flip`) — only the value vocabulary moved from `UiValue` to
  `DslValue`, and the `.expect("… fits ui map capacity")` panics disappear (the retained descriptor is
  infallible).
- `📌️panels/🔍️inspection/🦀️.rs` keeps `block3d_action` (contract nodes) — see class 4/7 below.

### 7 · `Label: From<Label>` / `From<LabelText>` in `📌️panels/🗿️artifact` (E0277, 5 sites)

Two distinct `Label` types exist: the SDK's retained `ui_wgpu::wgpu::Label` (re-exported as
`semio_framework_plugin::Label`) and the contract's fixed-capacity
`plugin_app_close_prelude::Label`. `tree_item_desc`/`PanelTreeBuilder::section*` want the contract one, and
`app_labels!` fields are a third type (`LabelText`). Precedent: every panel in puzzle/lowpoly/flow funnels
through a tiny per-plugin `ui_label(impl AsRef<str>) -> UiAssemblyResult<contract::Label>`
(`🧩️puzzle/…/✏️editor/🦀️.rs:1998`, `💠️lowpoly/…/✏️editor/🦀️.rs:59`) and passes `labels.x.as_str()`.

- `✏️editor/🦀️.rs:77` — added block3d's `ui_label`.
- `📌️panels/🗿️artifact/🦀️.rs` — `icon_item` now takes `&str` and admits through `ui_label`; both
  `section_or_placeholder` label + placeholder args go through `ui_label(labels.…​.as_str())?`. The
  `semio_framework_plugin::Label` import is gone.

### Follow-on (not in the original list, forced by class 4)

`📌️panels/🔍️inspection/🦀️.rs` was a `UiNode`-tree inspector (`ui_inspector_groups_to_tree`,
`UiFieldNode`/`UiInputNode`/`UiSelectNode`, `ActionDescriptor` on_change). Its `render` had to become
`UiAssemblyResult<BuiltNode>`, and it is the one editable inspector in block3d, so puzzle (a read-only
tree) is not the oracle here — `💠️lowpoly/…/📌️panels/🔍️inspection/🦀️.rs:29-68` is: `ui::input(InputKind::…)`
+ `ui::field(..)` rows bound with `try_on_with(Trigger::Change, action, args)` from the contract action
factory. Rewritten on that pattern, keeping every id, the `commit: "blur"` hint, the `patchObjectKind`
`{field}` args, the `setActiveRepresentation` select and the read-only vortex count, and keeping a
`PanelTreeBuilder` root so the existing test's `"type":"tree"` / not-`"type":"stack"` assertions still hold.
The block crate has no direct `semio-framework-ui-contract` dependency (puzzle/lowpoly/flow do), so the
builders are reached via `semio_framework_plugin::plugin_app_close_prelude` — noted inline as an SDK gap
rather than by adding a dependency.

## Files changed (all under `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/`)

- `🧬️schema/🧬️mutations/🎥move-camera3d/↩️inverse/🦀️.rs`
- `🧬️schema/🧬️mutations/🔍scale-camera3d/↩️inverse/🦀️.rs`
- `✏️editor/🦀️.rs`
- `✏️editor/🎚️config/🦀️.rs`
- `✏️editor/👥️presence/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/☑️options/↔️arrangement/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/☑️options/📏️spacing/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/☑️options/🔀️quick-representation/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/☑️options/🧱️representations/🦀️.rs`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/☑️options/🖌️brush/🦀️.rs`
- `✏️editor/📌️panels/🗿️artifact/🦀️.rs`
- `✏️editor/📌️panels/🔍️inspection/🦀️.rs`
- `👁️viewer/🦀️.rs`
- `👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️world/🦀️.rs`

The crate entry `📦️packages/🦀️rust/🦀️.rs` was NOT touched (no module added or removed), and neither was
`◻️2d/**` or `🖐️5d/**` (W7a/W7c's subsets). Retained-factory, boot-snapshot and io work already in these
files is intact.

## Verification

<!--VERIFICATION-->
