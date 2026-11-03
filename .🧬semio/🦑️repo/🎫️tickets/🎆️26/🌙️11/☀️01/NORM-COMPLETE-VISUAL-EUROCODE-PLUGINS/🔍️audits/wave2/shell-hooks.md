# Shell hooks (wave 2) - what changed, what is wired, what is open

Nothing was compiled or run. All Rust is checked by reading only; "ready for rebuild: shell hooks ..." lines are appended to `🗑️generated/integrate/latest.md`.

## Framework (domain-neutral, schema-first)
- New module `🧰️framework/🔨️modules/🛂️manifest/🧱️app-surface/📱️shell-hooks/` (crate `semio-framework-manifest-app-surface`, mounted in its `🦀️.rs`, re-exported from `semio_framework` and `semio_framework::manifest`):
  - Schema: `🧬️schema/🛰️.proto`, `🧬️schema/🟦️.ts` (ShellDeviceSignal, ShellHookCause, ShellLayoutRequest incl. app_id, ShellBottomSheet, ShellKeyEvent, ShellKeyOutcome, ShellAccessibleNode).
  - `PluginShellHooks` trait with neutral defaults: `choose_layout(&ShellLayoutRequest) -> Option<ShellLayoutChoice>` (1), `handle_key(&ShellKeyEvent) -> ShellKeyOutcome` (2), `accessible_nodes(scope, lang)` (4).
  - `ShellHookRegistry` (by plugin id), `ShellHookTracker` (names start / resize / inputModality / selection once, first wins by that order), `ShellHookDriver` (registry + tracker + touch + selection + current sheet; another app restarts with `start`).
  - `bottom_sheet_node(sheet, id, children)` builds the bottom sheet as UI tree node (3); `ShellAccessibleNode::projection` / `accessible_projection` turn plugin nodes into `AccessibilityProjectionNode` for the platform tree (4).
- UI contract: new `ContainerRole::Sheet` (wire "sheet") + builder `ui_contract::sheet(label)`; ARIA role "complementary" (Rust `container_role_name`, TS `containerRoleName`, label rule, retained reconcile arm, typed wire choice list, TS/Rust ContainerRole schema strings, generated `ui-contract/🟦️.ts` hand-fixed). `AccessibilityProjectionNode` now derives `Default`.
- wgpu native accessibility: platform roles `complementary`, `listbox`, `table` (the norm roles `listbox` and `table` had no mapping); fixture `🧪️tests/🧫️fixtures/♿️native-accessibility-tree/🔣️.json` roles table extended (AccessKit consumer tree stays the third-party oracle); TS test iterates the `sheet` role.
- Shell: `ShellDockState.shell_hooks: ShellHookDriver`; `ShellDockState::observe_device_hooks(docs, width, lang)` called in `plan_dock_windows` (every frame, asks only on change) sets `layout_override` and applies `apply_layout_diff`; `handle_pointer_button` notes touch from `PointerKind`, `handle_key` notes non-touch; `handle_keyboard_async` offers the key to the plugin (`shell_key_event`, scope = active window id) BEFORE the shell shortcut ladder when idle: `Consumed` ends it, `Action{id}` goes through `dispatch_app_keybinding` (an action no window owns falls through to the shell defaults).
- Tests: `📱️shell-hooks/🧪️tests/🦀️.rs` (tracker causes, driver asks only on change and keeps sheet, key before shell, projection, sheet node is complementary container, camel-case wire) and `🧪️tests/🥒️.feature`.

## Norm side
- `app-surface/🔌️shell-hooks/🦀️.rs` (mounted in the panels crate, re-exported): `NormShellHooks` implements the trait:
  - `choose_layout`: variant from the app id (`enNNNN`), viewer when `#viewer`; `device_view(signal, prefs, has_selection)` -> `visual_layout_with` / `view_layout_with` under the stored prefs (request text decoded by `decode_layout_prefs` -> else `stored_layout_prefs()` -> default); layout id `norm-visual-*`; bottom sheet (editor only, EN/DE label "Properties"/"Eigenschaften") also kept in `current_sheet()`.
  - `handle_key`: window scope from the active window id (`handle` -> Handle, `viewport` -> Viewport, else Global), chord found in the documented map, `dispatch_shortcut` -> action ids `norm.shortcut.<kebab action>` and `norm.handle.<key>.<fine|normal|large>`.
  - `accessible_nodes`: scene list + one `entity_node` option per entity, published by `render_viewport_panel` via `publish_scene_access`.
- Bottom sheet rendered: `render_inspector_value` (single choke point of all ten editors) wraps the inspector in `in_bottom_sheet` (container role sheet) when the last layout choice carried a sheet.
- Entity names: `viewport3d::description::{spoken_utilisation, entity_spoken_name}` is the one source; screen-reader `entity_node` and the lowering instance `"label"` both use it (cycle-free: viewport3d is below visual-chrome). No golden fixture contains the lowering label (grep over fixtures/json/feature found none), so no regeneration request was needed; the test `the_viewport_instance_label_is_the_screen_reader_name_of_the_entity` pins it.
- `check_accessible_name` unified: `check_row_facts(check, locale)` -> `check_row_node` (new `CheckRowFacts.quantities`), name = node name + ", " + clause. NotApplicable checks now speak only the status word (no "utilisation"), the en1993 e2e assertion was adjusted accordingly.
- Tests: `🔌️shell-hooks/🧪️tests/🦀️.rs` + `🥒️.feature`; appended to `🧪️tests/🔬️viewport-panels/🦀️.rs`; screen-reader tests.

## Follow-up wiring (coordinator request)
1. Registration: `semio_framework::register_shell_hooks(plugin_id, hooks)` (process-wide, consulted by every `ShellHookRegistry` after its own entries) and `PluginBuilder::shell_hooks(Arc<dyn PluginShellHooks>)` (registers under the builder's plugin id). The hub norm composition (`🌎️hub/🧩️compositions/📕️norm/🦀️.rs`) calls `.shell_hooks(Arc::new(NormShellHooks))` after `.package_id`; the shell knows nothing about norm.
2. Selection feed: `plan_dock_windows` sets `shell_hooks.note_selection(any interaction_selection domain has ids)` before `observe_device_hooks`, so the `selection` cause fires and the phone sheet opens/closes.
3. Actions: `shortcut_action_id` / `kebab` (focus-order) and `shortcut_action_definitions()` (manifest-and-media); `viewport_window_definition` now declares one window action per distinct shortcut action, labels = the shortcut's EN/DE label. Tab walk ids are `norm.handle.tab.forward|backward` (fixed: Shift+Tab was named "large").
4. Integration test `a_resize_from_desktop_to_mobile_switches_the_layout_and_a_selection_opens_the_sheet_through_the_registered_hooks` (registers via the global path); plus `every_answered_action_is_declared_by_the_viewport_window_with_both_languages`.

## Open
1. Stored prefs: the shell passes `prefs: None`; hook falls back to `store_layout_prefs`; host should load `norm.visual.layout` and call it.
2. Hooks are in-process only (no WIT/ABI export); a wasm/JS-hosted norm plugin would need an ABI addition. The hub composition path covers natively linked hosts.
3. Unverified until first build: all Rust above (e.g. `ActionDefinition.id` field name, builder chain for the sheet node, `DomainSelection.ids`).
