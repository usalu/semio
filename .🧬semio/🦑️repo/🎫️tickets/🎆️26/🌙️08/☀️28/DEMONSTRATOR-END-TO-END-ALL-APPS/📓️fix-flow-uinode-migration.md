# 📓️ Flow plugin — retiring `UiNode` from the six remaining Generate/Edit-mode renderers

Scope: `semio-s-plugin-flow`, base `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/`.
Method: static reading only — no `cargo` was run (a `wasm-release` build holds the global target-dir
lock). Everything below is my best reasoning from the actual source, not a confirmed compile.

## Template copied

`👁️viewer/🎭️modes/👁️view/🪟️windows/🌊️main/🦀️.rs::render` — the node-graph viewer window already on
`semio_framework_plugin::scene_surface(id, SurfaceKind::NodeGraph, &scene) -> UiAssemblyResult<BuiltNode>`.
Also read `✏️editor/📌️panels/🗿️artifact/🦀️.rs` (`document_panel`) and `✏️editor/📌️panels/🛍️catalogue/🦀️.rs`
(`catalogue_panel`) for the `PanelTreeBuilder` / `tree_item_with_action` / `tree_item_desc` tree-panel
shape, and `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`'s `FormPanelBuilder`/`entity_detail`/
`PanelTreeBuilder` sources plus `📓️recipe-plugin.md` (ticket `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`)
for the `ui::*` builder-DSL → `BuiltNode` mechanics used where no SDK convenience wrapper exists.

## Key finding: the SDK-re-exported helpers are not enough for two of the six

`semio_framework_plugin`'s crate root re-exports `BuiltNode`/`UiAssemblyResult`/`scene_surface`/
`built_to_component_tree`/`tree_item*`/`PanelTreeBuilder`/`Component`/`HasBase`/`HasChildren`/
`Buildable`/`ActionId`/`ActionBinding`/`Trigger`/`RowAction`/`RowActionPlacement`/`UiText`/`UiValue`/
`UiFixedList` (all confirmed by grepping the actual `pub use app::{…}` list at the bottom of that
crate's `🦀️.rs`), but it does **not** re-export the raw `ui::*` builder constructors (`ui::field`,
`ui::input`, `ui::select`, `ui::slider`, `ui::tree_item`, `ui::column`, …), `InputKind`, or the
*contract*'s `SurfaceKind` (only the legacy `ui_wgpu::wgpu::SurfaceKind` reaches the crate root, via
`pub use ui_wgpu::wgpu::*;` — `WindowKindDefinition.surface_kind` genuinely wants that legacy type per
`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:20`, so `definition()` in all six files is untouched and still
imports plain `SurfaceKind` from `semio_framework_plugin`). `generations`/`form` need row actions and
per-question-kind controls the SDK has no wrapper for, so **I added
`semio_framework_ui_contract` as a direct dependency of `semio-s-plugin-flow`'s `Cargo.toml`** — this
mirrors that SDK crate's own `Cargo.toml` comment verbatim ("`ui_wgpu` leaves the guest dependency
graph entirely … add `semio-framework-ui-contract` … in its place"), so it's the sanctioned migration
step, not a workaround. For `scene_surface`'s `kind` parameter specifically I import
`semio_framework_ui_contract::SurfaceKind as ContractSurfaceKind` to avoid colliding with the legacy
`SurfaceKind` already in scope for `definition()`.

`TextEditorScene`/`NodeGraphScene`/`NodeGraphViewport` needed **no** new dependency: a scene-relocation
comment in `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🎬️scenes.rs` states these 15 product
scene structs were "relocated verbatim from `ui_wgpu`'s … `ui_wgpu` now re-exports these types instead
of defining them" — so `ui_wgpu::wgpu::TextEditorScene`/`NodeGraphScene` (already a dependency) are the
same types `scene_surface`'s `T: SceneDoc` bound wants, not a legacy duplicate.

## The six renderers

1. **`main`** (`✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️.rs`) — `build_node_graph_scene(surface, app_id,
   scene) -> UiNode` → `scene_surface(surface, ContractSurfaceKind::NodeGraph, &scene) ->
   UiAssemblyResult<BuiltNode>`. Scene construction unchanged (same `NodeGraphScene`/`fixture_to_workflow`
   logic); `FLOW_PLAY_APP_ID` import dropped (no longer needed, `scene_surface` has no controller-id arg).

2. **`compiled`** — same pattern with `TextEditorScene`/`SurfaceKind::TextEditor`.

3. **`preview`** — same pattern; stopped calling the retired `crate::playbook::render_generation_preview_text`
   (still `UiNode`-returning, untouched — see below) and builds the `TextEditorScene` inline instead.

4. **`generations`** — rebuilt locally instead of calling `crate::playbook::render_generations_tree`
   (still `UiNode`-returning). Outer tree/sections via `PanelTreeBuilder::section_or_placeholder`/
   `.section` (same shape `document_panel`/`catalogue_panel` already use); each generation row is a
   hand-built `ui::tree_item(...)​.try_row_action(rename).try_row_action(remove).try_on(Activate, select)`
   (recipe §2's `TreeItem` row — `RowAction`/`RowActionPlacement::Menu` replace the retired
   `UiTreeItemAction{placement:Menu}` pair); the "add" row reuses the already-exported
   `tree_item_with_action`. The old `presence.selected` stamp on the selected row has no build-time
   equivalent (recipe §6) — dropped, with a docstring pointing at the identical documented gap already
   in `main`/`inspection`.

5. **`form`** — rebuilt locally instead of calling `crate::playbook::{render_generation_form_body,
   render_question_field}` (still `UiNode`-returning). Checked `forms_bridge::widget_to_playbook_block`
   (`🧰️framework/…/🌊️flow/🌿️vcs/🦀️.rs:2582`) first: `flow_fixture_to_form_spec` only ever emits question
   kinds `"slider"`/`"note"`/`"image"`/`"text"`/`"single"` (its match is exhaustive, `_ => None`), so
   `question_control` only implements those (`ui::slider`, `ui::select` with `.try_item` per option,
   `ui::input(InputKind::Text)` as the default/`"text"` arm); `"note"`/`"image"` render as bare
   `ui::text(...)`, unwrapped, matching the retired function's identical early-return shape. Each
   field is `ui::field(label).try_id(...).try_child(control)`; the body is `ui::column().try_children(...)`.

6. **`inspection_panel`** — retired `ui_declarative_sections_to_tree`/`UiSectionNode`/`UiNode` (none of
   which exist in `semio_framework_plugin` any more) replaced by
   `PanelTreeBuilder::new(...).section_or_placeholder(..., UiFixedList::default(), labels.no_selection)`
   — the SDK's own "empty section" convenience path, which already produces an item id ending in
   `.empty` (matches the test's `contains("flow-play-inspector.empty")`).

`editor/🦀️.rs`'s two dispatch tables (`render`/`render_with_instance_operation_owner`) now add
`.map(semio_framework_plugin::built_to_component_tree)` to all six call sites — previously only
`document_panel`/`catalogue_panel` had it; the six were relying on a bare `UiNode` fitting the match's
`UiAssemblyResult<ComponentTree>` arms, which was the actual type mismatch.

## Real, un-migrated dependency left in place (out of scope, still compiles)

`crate::playbook::{render_generations_tree, render_generation_form_body, render_generation_preview_text,
render_question_field}` (`🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️.rs`'s `generation_forms`
submodule) still import and return the OLD `ui_wgpu::wgpu::UiNode`/`build_text_editor_scene`/etc. — but
those still exist there (`ui_wgpu::wgpu`'s own `🧩️component.rs` still defines `UiNode` and every retired
builder fn; only `semio_framework_plugin`'s re-export surface dropped them), so that module still
compiles as-is. It is used **only** by these three flow windows (grepped: no other plugin calls
`render_generations_tree`/`render_generation_form_body`/`render_generation_preview_text`), so I left it
untouched and reimplemented the equivalent logic locally in `generations.rs`/`form.rs` rather than
migrating shared framework code outside this ticket's stated scope.

## Two extra fixes

- **`✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/📏️proximity/🦀️.rs`** — `on_change: flow_action("setProximityDistance", None)`
  no longer fits `WindowMeasure::Slider.on_change: ActionDescriptor` (`flow_action` now returns the new
  `UiAssemblyResult<(ActionId, Option<UiValue>)>`). Fixed exactly like the sibling `🌐️grid` option: added
  a local `fn proximity_action(action: &str) -> ActionDescriptor` and pointed `on_change` at it.
  `WindowMeasure`/`ActionDescriptor` are a separate, un-migrated manifest-level type (same as
  `Keybinding.action`) — not part of this UiNode migration.

- **`✏️editor/🎚️config/🦀️.rs:253`** (`impl Mutation<FlowConfig> for FlowConfigMutation`) — missing
  `const DESCRIPTORS`/`fn descriptor` (E0046). Modeled on `✏️editor/👥️presence/🦀️.rs`'s
  `FlowPresenceMutation` (single-variant) and `📐️cad/…/✏️editor/🎚️config/🦀️.rs`'s `CadConfigMutation`
  (multi-variant, the exact same shape). Added one `MutationLeafDescriptor` per `FlowConfigMutation`
  variant (15 total, in declaration order: `SetContributions`, `Snapshot`, `SetPreviewOff`, `SetCamera`,
  `SetLodMode`, `SetProximityDistance`, `SetGridVisible`, `SetGridSnapEnabled`, `SetGridFactor`,
  `SetCatalogueSections`, `SetAutomationEnabled`, `SetGeneration`, `SetDuplicateWidgetProgress`,
  `CancelDuplicateWidget`, `SetLocale`) with real, distinct `owner`/`semantic_kind`/`display_name`/
  `emoji` per variant (kebab `#[dsl(key = …)]` string as `semantic_kind`), `invertibility:
  ExplicitMutation`, `diff_participation: Detect`, `composition: Atomic`,
  `required_language_surfaces: &[Rust, JsonSchema]` — all matching the `CadConfigMutation` precedent.
  `SetContributions` additionally lists `MutationOutcomeClass::Error` in `outcome_classes` since its
  `diff()` arm can return an error outcome (`flow::sync_host_flow_extension_contributions` failure) —
  every other variant lists only `Applied`. Added a `descriptor()` matching each variant to its indexed
  `DESCRIPTORS` entry.

## Files touched

- `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml` (new `semio_framework_ui_contract` dependency)
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (dispatch table `.map(...)`)
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️.rs`
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/🗣️compiled/🦀️.rs`
- `.../✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/📏️proximity/🦀️.rs`
- `.../✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs`
- `.../✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs`
- `.../✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs`
- `.../✏️editor/📌️panels/🔍️inspection/🦀️.rs`
- `.../✏️editor/🎚️config/🦀️.rs`

## What remains unverified

No `cargo check`/`cargo build` was run (forbidden this session — a `wasm-release` build holds the
target-dir lock). Everything above is verified only by reading the actual type/trait/fn definitions and
cross-checking against sibling call sites that already use the same APIs (`document_panel`,
`catalogue_panel`, `CadConfigMutation`, `🌐️grid`/`🔭️lod` options, `FormPanelBuilder`, the `📓️recipe-plugin.md`
table). Specific residual risk points: (1) the exact `TryInto<Label>`/`AsRef<str>`/`Into<Label>` bound
satisfaction on a few call sites in `generations.rs`/`form.rs` I reasoned about but could not
type-check; (2) whether `semio_framework_ui_contract`'s `wasm32-wasip2` feature set is already
satisfied by the plain path dependency I added (no `features = […]` given, matching
`semio-framework-plugin`'s own dependency line); (3) the `➕` new Cargo dependency itself has not been
resolved/fetched by Cargo. The coordinator's central build verifies all of this.
