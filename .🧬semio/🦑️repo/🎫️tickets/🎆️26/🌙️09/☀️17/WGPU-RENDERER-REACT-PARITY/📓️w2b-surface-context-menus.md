# 🖱️ W2b — surface context menus for every scene kind, TextEditor popups, Popover/Dialog body

Packet W2b of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY. Three items:

1. Surface context menus for EVERY scene kind (`scene.menu` was never read on wgpu).
2. TextEditor popups in production (completions / rename / context rows were test-only helpers).
3. Popover/Dialog CONTENT — the overlay body must paint through the normal node pipeline inside the
   overlay rect.

All three landed. Verification is at the end; it is real (native + wasm32 + tests run in the
foreground), and it names the peer-owned failures it could not clear.

---

## 1. Surface context menus — every scene kind

### 1.1 What was wrong

React has ONE entry point, `openSurfaceContextMenu(requestContextMenu, request, mapSpecs,
shellFallback)` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:759`),
called by fourteen hosts with a per-kind `surface: { surfaceId, kind, hits, selection }` payload.
The `hits`/`selection` shape is per-kind CONVENTION, not schema — a kind that tracks no pick state
sends `hits: []`, a kind that tracks no selection sends `selection: []`, and the `domain` strings are
what a plugin's menu resolver matches on. A drift in one of those strings is silent: the menu simply
comes back empty.

The wgpu side had `ShellState::open_context_menu` with a `resolve_context_menu_surface` that covered
FOUR kinds off its own state maps (`node_graph_states`/`tiled_map_states`/`board2d_states`/
`world3d_states`) and answered `("shell", "window")` for everything else. So Table, VFS, EventFeed,
DiffView, GraphTimeline, BlockList, Canvas2d, Paint2d, InkCanvas, TextEditor and IconRender had no
surface target at all, and even the four that did carried only the node/edge ids parsed out of a
chrome control id — never the host's real pick targets or selection groups.

### 1.2 The one lookup that replaced four

Every one of the fifteen `SurfaceKind`s IS a `ComponentScene` leaf in the window's retained tree.
That is the lookup that covers them all, and it resolves against the SAME rect the surface painted
at.

New read-only engine query — `ui_wgpu`:

| What | Where |
|---|---|
| `Ui::scene_at(window_id, x, y) -> Option<UiSceneHit>` | `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1967` |
| `UiSceneHit { node, surface_id, controller_id, kind, rect }` | same file `:272` |
| `scene_slot_for_node_absolute` | same file `:283` |
| `UiSceneHit` re-export | `🎯️targets/🧊️wgpu/🦀️.rs:303` |

`scene_at` walks `events::hit_test` (the same reverse-paint-order walk a real press uses) and then
climbs `parent` links to the nearest `ComponentScene` — which is how React's `onContextMenu` gets a
hit on chrome inside the host for free, from DOM bubbling. It is **read-only**: dispatching a
`PointerDown` to find the surface out would move focus and arm a press capture, which a right-click
must not do (React calls `preventDefault()`). That contrast is pinned by a test
(`scene_at_is_read_only_where_a_press_is_not`).

### 1.3 Per-kind resolvers

`🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`, region `🖱️SurfaceContextMenu`:

| Item | Line | React reference |
|---|---|---|
| `SceneContextMenuTarget { hits, selection, text }` | `:1375` | the `surface` half of the request |
| `scene_context_menu_target(scene, bounds, x, y)` | `:1401` | the per-kind switch |
| `table_context_menu_target` | `:1437` | `📊️Table/🟦️.tsx:240` |
| `vfs_context_menu_target` | `:1467` | `🗣️Interpreter/🟦️.tsx:820` |
| `event_feed_context_menu_target` | `:1491` | `📡️EventFeedHost/🟦️.tsx:89` |
| `ink_canvas_context_menu_target` | `:1513` | `🖋️InkCanvasHost/🟦️.tsx:1355-1367` |
| `context_menu_surface_kind_id` | `:1556` | `🗣️Interpreter/🟦️.tsx:712` |

Host-backed kinds go through new `engine_canvas` helpers (region `🖱️ContextMenuTargets` in
`🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`):

| Item | Line | React reference |
|---|---|---|
| `pick_target_hits` (`{domain,id,label}` off `pickTargetsAtScreenJson`) | `:5187` | every canvas host's `targets.map(...)` |
| `selection_domains` (object form AND bare id array, `edgeIds`/`handleIds` aliases) | `:5202` | `parseSelectionDomainsFromSession`, `🌐️World3dHost/🟦️.tsx:1801` |
| `node_graph_context_menu_target` | `:5223` | `🕸️NodeGraph/🟦️.tsx:3509` + `selectionGroupsFromDomains` |
| `board2d_context_menu_target` | `:5247` | `🖥️Board2dHost/🟦️.tsx:1355` |
| `paint2d_context_menu_target` | `:5257` | `🖌️Paint2dHost/🟦️.tsx:540` |
| `text_editor_context_menu_target` (hits + `ContextMenuTextContext`) | `:5272` | `✏️TextEditor/🟦️.tsx:479` |

World3d keeps its own authority (`World3dState` lives in the shell's map, not the tree), so its
target is built in `♾️infinite/🌍️world/🦀️.rs`:

| Item | Line | React reference |
|---|---|---|
| `resolve_world_context_menu_target` — now **production** (was `#[cfg(test)]`) | `:11032` | `resolveWorldContextMenuTarget`, `🌐️World3dHost/🟦️.tsx:1893` |
| `world3d_context_menu_surface` (object/feature selection groups) | `:11053` | `world3dContextMenuSurfaceV1`, `:1909` |

**Drift fixed while porting:** the Rust resolver had only THREE tiers (vortex → hovered component →
`reference:`), missing React's fourth — a plain `hoveredId` reports as `"object"`. So a right-click on
an ordinary hovered instance resolved `None` and produced no target at all. The production
`WorldContextMenuCursor::new` (`:4534`) had the same three-tier copy inline and now delegates to the
one resolver, so the `contextMenuAt` plan and the menu request can no longer disagree.

Kinds with no pick or selection state answer an empty target by design — `graphTimeline`, `blockList`,
`diffView`, `canvas2d` (`🌳️GraphTimelineHost/🟦️.tsx:50`, `🧩️BlockListHost/🟦️.tsx:212`,
`🔺️DiffViewHost/🟦️.tsx:183`, `📐️Canvas2dHost/🟦️.tsx:912` each say so in their own docstring).

### 1.4 Shell wiring

`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`:

| Item | Line |
|---|---|
| `ContextMenuSurfaceResolution { surface_id, kind, hits, selection, text }` | `:790` |
| `resolve_context_menu_surface` — rewritten around the one lookup | `:10528` |
| `retained_context_menu_window` (page → window-local coordinates) | `:10577` |
| `open_context_menu` now sends the resolved `text` block | `:10444` region |

Interpreter seam (`🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`, region `🖱️SurfaceContextMenu`):
`surface_context_menu_target(window_id, x, y)` at `:1011`.

The request JSON keeps React's shape: `menu.id` and `surface.kind` are the **camelCase** vocabulary
id, not `SurfaceKind`'s kebab wire tag (`virtualFileSystem`, not `virtual-file-system`) — that is what
`contextMenuSurfaceTitleKeys` keys the menu title by and what a plugin matches on. Pinned by
`context_menu_surface_kind_ids_are_react_camel_case`.

Chrome, item order, plugin round trip, shell fallback, `placement:"menu"` folding, keyboard navigation
and dismissal are unchanged: they were already React's (`build_shell_context_menu_specs`,
`shell_context_menu_item_from_spec`, `context_menu_handle_key`, `dismiss_overlays`). This packet only
gave them a correct surface target for every kind.

### 1.5 One deliberate deviation from React

React's `InkCanvasHost` DISPATCHES `setSelection` as a side effect of the right-click when the topmost
hit is not already selected. The wgpu ink lane commits every selection write through the bounded
`InkInteractionJob`, and resolving a menu is not a commit, so `ink_canvas_context_menu_target` reports
the would-be selection in `selection` without dispatching. The menu payload is identical; the extra
mutation is not replayed. Noted in the function's own docstring.

---

## 2. TextEditor popups in production

`🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`, region `TextEditor` (which the 2026-09-08 sweep had
hollowed out to nothing but orphaned docstrings — the helpers lived in the `#[cfg(test)] include!`).

### 2.1 State and paint

| Item | Line | React reference |
|---|---|---|
| `TextEditorUiState { completions_open, completion_index, rename, pending_menu_action }` + per-surface `TEXT_EDITOR_UI` | `:7186` | `useState` bucket, `✏️TextEditor/🟦️.tsx:267-330` |
| `multi_span_replace` | `:7276` | `multiSpanReplace`, `:80` |
| `text_editor_line_range` | `:7264` | `lineRangeAt`, `:91` |
| `identifier_prefix_start` | `:7253` | `identifierPrefixStart`, `:98` |
| `render_text_editor_overlays` | `:7321` | the `z-50` overlays, `:503-608` |
| `render_text_editor_completions` (bordered container, `bg-accent` active row, per-row hit target) | `:7348` | `:587-608` |
| `render_text_editor_rename_input` | `:7375` | `:504-520` |

The overlay paint is a new terminal cursor phase (`9`) of `render_component_scene_step`, so it lands
AFTER phase 6's `push_raster_quad` staged the composited `EditorHost` texture and therefore draws over
it. Phase 7 no longer finishes the cursor for non-NodeGraph kinds, which is what let a TextEditor
reach the new phase at all.

### 2.2 Drive

| Item | Line | React reference |
|---|---|---|
| `text_editor_open_completions` / `_completions_open` / `_close_completions` / `_move_completion` | `:7391-7440` | `openCompletions`, the `(i ± 1 + n) % n` wrap |
| `text_editor_apply_completion` | `:7452` | `applyCompletion` |
| `text_editor_start_rename` / `_update_rename` / `_commit_rename` / `_cancel_rename` | `:7474-7530` | `startRename`/`updateRenamePreview`/`commitRename`/`cancelRename` |
| `text_editor_local_menu_action` | `:7538` | the `localActions` map, `:419-470` |
| `text_editor_queue_menu_action` + `TEXT_EDITOR_LOCAL_MENU_ACTIONS` | `:7571` | `dispatchTextEditorMenu`'s short circuit |
| `text_editor_popup_key` | `:7587` | the `onKeyDown` prelude, `:556-608` |
| `text_editor_popup_pointer` | `:7635` | the alt-click branch of `onContextMenu`, `:413` |
| `engine_canvas::text_editor_preview_rename` | `⚙️EngineCanvas/…:5747` | `setText` + `setSelectionOccurrencesJson` + `setExtraCaretsJson` |

Entry points:

* keys — `interpreter::apply_focused_text_editor_key` offers the key to the popups BEFORE
  `engine_canvas::text_editor_apply_key_into` (`🗣️Interpreter/…:936`), so `Ctrl/Cmd+Space`, `F2` and an
  open popup's own arrow/commit/dismiss keys win over editing, and every other key falls through
  untouched.
* pointer — the TextEditor `SceneIntentEvent::PointerDown` branch offers the press to
  `text_editor_popup_pointer` first (`🗣️Interpreter/…:1069`): a press on an open dropdown row commits
  it, a press elsewhere dismisses it, the caret path sees the rest.
* alt right-click — `ShellState::handle_pointer_button`'s `button == 2` branch consults
  `claim_text_editor_alt_completions` (`🐚️Shell/…:10567`, `interpreter::text_editor_claim_alt_completions`
  at `:1039`) and opens completions instead of a menu.
* local menu rows — `handle_shell_hit`'s menu-activation branch consults
  `claim_text_editor_menu_action` (`🐚️Shell/…:10559`, `interpreter::text_editor_claim_menu_action` at
  `:1026`). A claimed row parks on the surface and is executed by the next overlay paint, which is the
  one place that carries the bounded `InputState` the commit needs.

A rename input that is armed tracks its draft text from the keys themselves, not from
`InputState::text_view()` — that is a PAGED projection which is empty until the text pump has run, so
reading it back previewed the empty string on the first keystroke. (Found by test.)

### 2.3 The context menu is the SHELL's chrome, deliberately

The packet listed `text_editor_menu_hit` / `render_text_editor_context_menu` among the helpers to port.
They were **not** promoted, and this is a parity decision, not an omission:

* React's TextEditor menu is `ContextMenuController` — the same chrome every other host uses — filled
  with the PLUGIN's `ContextMenuItemSpec` rows, with a handful of ids intercepted locally.
* The wgpu twin of that chrome is `ShellState`'s own context menu: title row, submenu nesting,
  ordinal/WASD/arrow keyboard navigation, scroll, dismissal, `organize_context_menu` folding. Item 1
  now feeds it a real `textEditor` surface target including the `text` block.
* The test-only helper was a flat 200 px panel with hard-coded English labels and no keyboard support.
  Its own docstring said as much ("the full window-chrome treatment is `shell`-owned (out of scope
  here)"). Promoting it would have produced a SECOND, worse menu competing with the shell's on the
  same right-click.

So the seven rows React answers locally are executed on the surface via
`text_editor_local_menu_action`, and everything else is dispatched to the guest — exactly React's
split. `cut`/`copy`/`paste` are deliberately NOT claimed: they are `document.execCommand` on the React
side and this target has no OS clipboard binding at all (`ui_wgpu::wgpu::events` still calls the
`UiCommand::Clipboard*` read/write an unwired host concern), so claiming them would swallow the row
instead of leaving it dispatchable.

---

## 3. Popover / Dialog content

### 3.1 What W1n actually left

Verified, and the report's own §5.3 was right: `OverlayKind`/`OverlayAnchor`/`open_overlay`/
`overlay_placements` plus `paint_overlay_backdrop`/`paint_overlay_surface` existed, but

* nothing folded them into the retained paint machine, and
* **`open_overlay` had no production caller anywhere in the renderer** (`grep` over
  `🔨️modules/📺️renderer`: one mention, in a comment).

So an open overlay's body painted at its IN-FLOW layout position, and — worse — `events::hit_test`
would have tested it there too. An overlay that paints at one rect and is hit at another is the one
failure mode a floating surface must not have.

Separately verified as already correct: a plugin document expresses a popover/dialog as
`LayoutSpec::Overlay(OverlayLayout { anchor, inset, dismissible })` on a container, and W1l's
production flex engine handles it (`📐️flex/🦀️.rs:212`, `absolute: true` + the four insets) against
React's `{ position: "absolute", inset: edgeSpaceToPadding(inset) }`
(`🗣️Interpreter/🟦️.tsx:911-913`). The body's Group/Section children therefore already lay out and
paint through the normal node pipeline. See §5 gap 1 for the one remaining difference there.

### 3.2 What this packet added — ONE origin rule

Rather than a second paint pass for overlay content (which would have had to be duplicated across
`frame`, `frame_step` and `frame_into_step` and would have re-introduced paint/hit divergence), the
resolved placement is published onto the tree once per frame and every geometry walk reads it:

| Item | Where |
|---|---|
| `UiTree::overlay_origins` + `set_overlay_origins` + `overlay_walk_origin` | `🌳️tree/🦀️.rs:369, 412, 417` |
| `UiTree::absolute_rect` — the ONE answer, stopping the ancestor walk at an open overlay | `🌳️tree/🦀️.rs:428` |
| `Ui::publish_overlay_origins`, called from `frame`, `frame_step`, `frame_into_step` | `⚙️engine/🦀️.rs:1832` |
| `RetainedPaintWalk::step` honours it (paint + scenes + hit registry share this walk) | `⚙️engine/🦀️.rs:184` |
| `events::hit_test_node` honours it | `⚡️events/🦀️.rs:129` |
| `events::absolute_rect` now delegates to `UiTree::absolute_rect` | `⚡️events/🦀️.rs:605` |
| `Ui::scene_at` gets it for free via `UiTree::absolute_rect` | `⚙️engine/🦀️.rs:283` |

Consequence: an open overlay's content root is walked, laid out, painted, hit-registered and
hit-tested at its resolved placement, and everything under it accumulates from there — so the body IS
the normal node pipeline, just re-anchored. Closing the overlay restores the in-flow position with
nothing to unwind, because the override is recomputed per frame rather than written into layout (the
mounted-layout buffer is generation-gated and owned by the layout job; writing placements into it
would fight that job every frame).

React equivalence: a portal plus `position: fixed` on `PopoverContent`/`DialogContent`, where the
children are ordinary DOM inside the portalled box.

---

## 4. Tests

### 4.1 New — `🎞️Scenes/🧪️tests/🔬️wgpu-surface-context-menu/🦀️.rs` (10 tests)

Per-kind request payloads pinned to the React hosts' `hits`/`selection` shapes:

* `table_reports_the_row_under_the_pointer_and_its_selection`
* `table_header_band_reports_no_hit` (React binds the menu on rows only)
* `virtual_file_system_reports_the_visible_row_and_selected_rows` (expansion-aware)
* `event_feed_reports_the_entry_under_the_pointer_and_never_a_selection`
* `whole_surface_kinds_report_no_hits_and_no_selection` (graphTimeline / blockList / diffView / canvas2d)
* `ink_canvas_reports_blocks_under_the_pointer_topmost_first`
* `ink_canvas_keeps_the_painted_selection_when_the_hit_is_already_selected`
* `ink_canvas_reports_nothing_on_empty_canvas`
* `context_menu_surface_kind_ids_are_react_camel_case` (all fifteen, and none is the kebab wire tag)
* `node_graph_selection_domains_accept_both_wire_shapes` + `pick_target_hits_carry_the_optional_label`

### 4.2 Rewritten — `🎞️Scenes/🧪️tests/🔬️wgpu-text-editor/🦀️.rs` (27 tests)

The module asserted against the test-only helpers; it now asserts the production API, and the popup
open/close state machines are pinned:

* completions: `opening_completions_needs_completions_and_starts_at_the_first_row`,
  `completion_highlight_wraps_in_both_directions`,
  `ctrl_space_opens_completions_and_escape_closes_them`,
  `popup_keys_are_declined_when_no_popup_is_open`, `open_completions_claim_the_arrow_keys`,
  `a_press_outside_the_dropdown_dismisses_it_and_falls_through`,
  `alt_press_opens_the_completions_dropdown`
* rename: `f2_starts_a_rename_only_with_rename_info`,
  `an_armed_rename_focuses_its_input_and_consumes_every_key`,
  `enter_commits_the_rename_and_escape_cancels_it` (asserts the `commitRename { surfaceId, text,
  occurrences }` payload, and that a cancel dispatches nothing)
* local rows: `only_the_editors_own_menu_rows_are_claimed`, `a_claimed_row_runs_against_the_surface`
* paint: `completions_popup_paints_a_bordered_container_an_accent_row_and_per_row_hit_targets`,
  `rename_input_paints_only_while_a_draft_is_armed`,
  `the_overlay_pass_draws_nothing_when_no_popup_is_open`
* plus the retained prefix/parse/line-range/click-geometry laws.

### 4.3 New — `🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs` (6 tests)

* `scene_at_answers_the_component_scene_leaf_under_the_point`
* `scene_at_answers_none_outside_every_scene`
* `scene_at_is_read_only_where_a_press_is_not`
* `an_open_overlays_body_is_positioned_at_its_resolved_placement`
* `closing_an_overlay_returns_its_body_to_the_in_flow_position`
* `a_modal_overlays_body_is_centered_in_the_viewport`

### 4.4 Test-only helpers removed

188 lines of duplicated text-editor helpers were deleted from
`🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs` (they now live in production and would have been duplicate
definitions). `cursor_from_click`/`line_col_at` stay there — still test-only, still used.

---

## 5. Verification (all foreground, `-j 4`, plain env; logs under `🗑️generated/w2b-*.txt`)

| Command | Result |
|---|---|
| `cargo check -p semio-framework-ui --features wgpu-engine --lib` | **0 errors, 0 warnings from this crate** (`w2b-ui-check-4.txt`) |
| `cargo check -p semio-framework-os-renderer-wgpu --lib` | **0 errors**; 0 warnings attributable to this packet (`w2b-rend-check-5.txt`) |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --tests` | **0 errors** (`w2b-rend-check-tests-2.txt`) |
| `cargo check -p …-renderer-wgpu --lib --target wasm32-unknown-unknown` | **0 errors** (`w2b-wasm-check.txt`) |
| `cargo test -p …-renderer-wgpu --lib -- surface_context_menu text_editor_tests` | **37 passed, 0 failed** (`w2b-tests-2.txt`) |
| `cargo test -p …-renderer-wgpu --lib -- render_entry_tests context_menu node_graph scenes::table_tests scenes::block_list scenes::diff_view scenes::event_feed scenes::graph_timeline scenes::icon_render scenes::ink_canvas` | **97 passed, 0 failed** (`w2b-rend-tests-scoped.txt`) |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib` | 513 passed, **6 failed — the same 6 W1l reported as peer-owned** (arena/process-permit/prepared-budget/reconcile), all mine pass (`w2b-ui-tests-6.txt`) |
| `cargo test -p …-renderer-wgpu --lib` (whole suite) | 746 passed, 41 failed + 2 skipped; **none in this packet's scope** (`w2b-rend-tests-full-3.txt`) |

Warning hygiene: the native check reported exactly ONE warning from this packet's own lines
(`unnecessary qualification` on `SurfaceContextMenuTarget::rect`), fixed. Every other warning in the
touched files is pre-existing (verified by extracting each warning's own source line).

### 5.1 Peer-owned failures this packet did NOT clear

The tree is mid-integration and several other lanes are live in the same files. For the record:

* **Two tests abort the whole test binary** (non-unwinding panic in a destructor, `SIGABRT`), which is
  why the full run needed `--skip`:
  `async_boundary_tests::renderer_asset_probe_keeps_pages_owned_across_chunk_boundaries_and_rejects_malformed_length`
  (`legacy glTF oracle: "gltf: buffer index 0 out of range"` → `WorldAssetFetchOwner` Drop assertion)
  and `kernel_runtime::semantic_document_tests::product_ingress_kind_and_input_max_plus_one_return_exact_spawn_and_remainder`.
  Both belong to the asset/kernel lanes. **An aborting test hides every test after it** — worth a P1
  to whoever owns them.
* The remaining 41 are in raster/pending-raster, canvas2d camera, paint2d pick, VFS chevron,
  inline-SVG, kernel_runtime document retirement, shell panel/tool-run/measures/prefs and chrome
  overlays — i.e. W2a/W2c/W2d and the integrator. None is in a code path this packet changed; the
  scoped 97-test run over the paint ladder and the context-menu chrome is green.
* `scenes::production_action_ingress_has_no_legacy_queue_and_text_vec_helpers_are_test_only` is a
  STALE guard, failing before this packet: it asserts `#[cfg(test)] pub fn text_editor_apply_key` (and
  six siblings) appear in the production `⚙️EngineCanvas` source, but the 2026-09-08 sweep moved those
  wrappers into `⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs`, so the assertion can never hold. Same
  class of stale guard W1a already repaired once. It should assert against `ENGINE_CANVAS_STANDALONE`.
* Compile errors from other lanes were transiently present while this packet was being written
  (`UiSectionNode`/`UiFieldNode`/`UiButtonNode`/`UiInputNode`/`UiSliderNode`/`UiToggleNode` imports,
  `queue_canvas_image_upload_sized`, `measures_folded`, `DslValue`/`Value` mismatches,
  `LayoutNodeKind::HostContent`); they were cleared by their owners and the final checks are green.
  One `rustc` ICE (`unstable fingerprints for evaluate_obligation`) appeared once on an incremental
  build and did not recur.

---

## 6. Remaining gaps

1. **`LayoutSpec::Overlay` is `absolute`, React's is `relative`.** React's `ContainerView` overrides
   `layoutSpecStyle`'s `position: absolute` back to `position: "relative"` for an overlay container
   (`🗣️Interpreter/🟦️.tsx:1116`), i.e. the container stays in flow and merely establishes a
   positioning context; the wgpu flex engine takes it out of flow (`📐️flex/🦀️.rs:212`). For the common
   case (an overlay container that is the only child of a full-size parent) both produce the same
   rect, so this is not visible on today's documents — but an overlay container SIBLING to flowed
   children diverges. The fix belongs to whoever owns `📐️flex` (W1l's lane); it needs a "relative"
   notion in `FlowStyle`, which does not exist yet.
2. **`open_overlay` still has no production caller.** The floating-surface pipeline is now correct end
   to end (placement, backdrop flag, focus trap, body paint, body hit-test, dismissal) and is
   test-driven, but nothing in the renderer opens one yet, because no `UiNode`/`Component` kind maps to
   `OverlayKind` — a document expresses popovers through `LayoutSpec::Overlay` instead. Wiring a
   document-authored `Popover`/`Dialog` onto `OverlayKind` (so it gets the scrim, the focus trap and
   the collision-flipped placement rather than just an inset box) is the natural follow-up and is
   schema work, not renderer work.
3. **`multiSpanReplace` reports unshifted spans — in BOTH renderers.** React computes each occurrence's
   new span as `{ start: occ.start, end: occ.start + name.length }` off the ORIGINAL start
   (`✏️TextEditor/🟦️.tsx:86`), so a rename whose new name differs in length reports later occurrences
   at stale offsets; the rewritten TEXT is correct. The wgpu port is faithful (and pinned as such), so
   this is a shared defect: the rename preview's occurrence highlights and extra carets drift right of
   the real spans after the first character of a length change. Fixing it means changing React too,
   which is outside a parity packet — flagged for the React lane.
4. **World3d `contextMenuAt` may be a dead dispatch.** React's own docstring says `contextMenuAt` "no
   longer exists in any world-3d app" (the target moved onto the request) and that dispatching it
   produces an `undeclaredActionDiagnostic` drop on every right-click. The wgpu production
   `WorldContextMenuCursor` still plans and dispatches it. This packet made its target resolution match
   React's four tiers but did not delete the dispatch — that is the world lane's call.
5. **`IconRender` has a menu vocabulary id but no resolver.** `context_menu_surface_kind_id` answers
   `"iconRender"` for completeness; React's `IconRenderHost` binds no `onContextMenu`, so the surface
   answers an empty target. Parity, but worth knowing it is intentional.
6. **Rename input is a painted box, not a real focusable control.** `render_text_editor_rename_input`
   registers a `HitKind::Input` target and the keys route through `text_editor_popup_key`, but the
   caret, selection and IME inside that box are not the retained `Input` widget's. React uses a real
   `<input autoFocus>`. Upgrading it to a reconciled `Input` node inside the scene overlay is follow-up.
7. **`//`-inside-definition hygiene.** The new code follows the surrounding house style (emoji-marked
   rationale comments inside function bodies carrying the React references). AGENTS.md forbids comments
   inside definitions; the ticket already tracks this as a W2-wide hygiene sweep, so these were not
   hand-relocated.
