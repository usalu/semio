# W15a — ui-element residual gaps closed (source + tests only)

Input: `📓️audit-w14-elements-residual.md` §9 items 2, 3, 4, 5, 6, 7, 10. Items 1, 8, 9 (CAD pick lane,
curved edges, instance opacity) belong to W14g and were not touched. NO wasm build and NO activation
were run here — W14d/W14f own live verification.

Every claim below was grepped before it was acted on; the "was" line of each item states what the
grep actually found, which in three cases was narrower or **wider** than the audit said.

---

## 2 — `WidgetNode<E>` gains `Progress`/`Image`/`Group`/`ComponentScene`/`ExternalSlot`

**React source**: `🗣️Interpreter/🟦️.tsx` `renderComponent`'s `case "progress"` → `ProgressView`
(`:2098-2118`, a `bg-muted h-tiny w-full` track with a `bg-accent` fill at
`uiProgressFractionV1(completed, total)`), `case "image"` → `ImageView`, `case "surface"` →
`SurfaceView`, `case "extension"` → `ExtensionView`, and `group` as a `container` with `role="group"`
→ `ContainerView`.

**Was**: `WidgetNode<E>` carried 15 arms against `UiNode`'s 20 — confirmed by reading the enum, not by
trusting the audit. The gap was visible in the test harness itself: `to_widget_node`'s last arm
(`🔬️targets-wgpu-engine-unit/🦀️.rs`) collapsed all four of Progress/Image/ComponentScene/ExternalSlot
to an *empty placeholder `Text`*, with a "KNOWN GAP" comment saying no two-pipeline comparison was
possible for them, and `UiNode::Group` was mapped onto `WidgetNode::Section`.

**Now**:

- `🪀️widgets/🦀️.rs:236-268` — five new variants in region `🧩️KitCompositeParity`
  (`Progress { id, completed, total }`, `Image { id, src, alt }`,
  `Group { id, label, default_open, children }`,
  `ComponentScene { surface_id, hit_kind, hit_control_id }`, `ExternalSlot { body_key }`).
- `🪀️widgets/🦀️.rs:270-281` — `WidgetNode::component_scene(&UiComponentSceneNode)` takes the scene's
  `HitKind`/control id from `input::retained_scene_hit`, so the kit and the retained hit registry
  cannot answer differently for the same node. `input::retained_scene_hit` was widened
  private → `pub(crate)` for this (`📥️input/🦀️.rs:689`).
- `🪀️widgets/🦀️.rs:340-368` — measure arms (progress = a `SIZE_TINY` band, image = its decoded
  natural size clamped to `UI_IMAGE_MAX_BOX_HEIGHT` else one `alt` line, group = header + children,
  scene/slot = container-filling).
- `🪀️widgets/🦀️.rs:470-479` — render dispatch; `🪀️widgets/🦀️.rs:481-553` — the five painters
  (`render_progress`, `render_image`, `render_group`, `render_scene_placeholder`,
  `render_external_slot`), each drawing the same geometry and tokens as its retained twin.
- `🖌️paint/🦀️.rs:2003-2010` — `progress_bar_rects_of(completed, total, bounds, theme)` extracted so
  both kits price the bar from ONE function; `progress_bar_rects` is now a one-line delegate.
- `🔬️targets-wgpu-engine-unit/🦀️.rs:693,707-712` — `to_widget_node` maps all five for real, and the
  "KNOWN GAP" comment is replaced by a pointer to the new law.

**Hit parity**: `ComponentScene` registers its hit target (the kit's own `render_widget` arm);
Progress/Image/Group-body/ExternalSlot register none, which is exactly what
`input::retained_hit_registration` does for those kinds (its `_ => None` arm). `Group` registers a
`section.chevron.<id>` header target, matching `Section`.

**Law**: `every_ui_node_kind_has_a_widget_kit_arm_that_paints`
(`🔬️targets-wgpu-engine-unit/🦀️.rs`) — all five paint something; a collapsed `Group` paints strictly
less than an open one; a `ComponentScene`'s kit hit contract equals `retained_scene_hit`'s.

---

## 3 — bold/semibold text actually wired

**React source**: `TextView`, `🗣️Interpreter/🟦️.tsx:1168` —
`cn("text-foreground", component.emphasize ? "font-semibold" : "text-sm")`. Emphasized text is BOTH a
size step (it loses `text-sm`) and a real weight change.

**Was**: `draw_text_weighted`/`TextWeight::Semibold`/`faux_bold_offset` existed and were unit-tested
(W2k) with **zero callers outside their own definition** — confirmed by grep over the whole target.
`retained_text_node_step` had no weight parameter at all and only swapped the font size.

**Now**:

- `🖌️paint/🦀️.rs:132-141` — `paint_retained_glyph_step_weighted(…, flow, weight, …)`, the one
  weighted retained entry; `paint_retained_glyph_step`/`…_flowed` are one-line delegates at
  `TextWeight::Regular`, so every other retained run is byte-identical to before.
- `🖌️paint/🦀️.rs:150-155` — the shared `paint_retained_glyph_step_inner`.
- `🖌️paint/🦀️.rs:~195` — the retained grant is priced at `weight.strikes()` items and
  `strikes * size_of::<UiInstance>()` bytes (the number `TextWeight::strikes`'s own doc comment and
  the W2k text law already specified), and `🖌️paint/🦀️.rs:~222` emits the second strike at
  `x + faux_bold_offset(size)`, same baseline, same advance.
- `🖌️paint/🦀️.rs:324-336` — `retained_text_node_step_weighted`; `retained_text_node_step` delegates
  at `Regular`.
- `🖌️paint/🦀️.rs:~700` — the `UiNode::Text` arm passes `TextWeight::of(emphasize)`.
- `🎯️targets/🧊️wgpu/🦀️.rs:328` — `paint_retained_glyph_step_weighted` re-exported;
  `draw_text_weighted` added to the `widgets` re-export list.

**Law**: `an_emphasized_retained_text_node_strikes_every_glyph_twice_without_moving_the_pen`
(`🔬️targets-wgpu-paint-unit/🦀️.rs`) — an emphasized glyph's BOTH strikes land inside ONE grant (never
split across two, which is what the doubled reservation buys), the second sits at exactly
`faux_bold_offset`, the first lands on the same pen x as the regular run, and a regular run still
costs at most one strike per grant.

---

## 4 — Select scroll chevrons scroll

**React source**: `🧱️elements/🔽️Select/🟦️.tsx` — `scrollSelectViewport` (`:769-775`,
`max(24, floor(clientHeight * 0.8))`), `SelectScrollUpButton`/`SelectScrollDownButton`
(`:790-818`), and the `overflow-y-auto` viewport the DOM clamps for it.

**Was**: `select_scroll_step`/`select_clamped_scroll` existed, were unit-tested, and had **zero
callers** — they were two of the four `never used` warnings on the crate's own baseline check. The
chevrons painted and did nothing; rows were always drawn from index 0.

**Now** (`🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs`, region `🔼️ScrollSlots`, `:155-222`):

- Two `WidgetContext::scroll_offsets` slots per open popup — `select.<id>.scroll` (current offset)
  and `select.<id>.scrollStep` (an UNAPPLIED chevron direction). The press handler cannot clamp (it
  has neither the item list nor the resolved popup height), so it only arms a direction; the painter,
  which has both, prices and clamps it.
- `select_scroll_control_id`/`select_scroll_control_parts`, `select_scroll_viewport_height`,
  `select_scrolled_offset` (the one consumer of BOTH `select_scroll_step` and
  `select_clamped_scroll`), `select_scrolled_row_window` (first row + bounded count).
- `arm_select_scroll` / `clear_select_scroll` (`:482`, `:490`), re-exported from the target root
  (`🎯️targets/🧊️wgpu/🦀️.rs:373`).
- `render_select_menu` (`:395-460`) resolves the offset before painting, scissors the popup band,
  paints only the `select_scrolled_row_window` rows at `-offset`, and registers the two chevron hit
  bands LAST so they win the hit resolve (`HitRegistry::resolve` iterates in reverse).
- Production press path: `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:~10970` — `arm_select_scroll` answered
  before the `match id`, for the same reason the pane chips are (the id is decoded, not prefixed).
- Production close path: `ShellState::close_open_selects` (`🐚️Shell/…/🦀️.rs:~11720`) is now the ONE
  close route (outside-press, Escape, toggle) and drops both slots; the item-commit arm drops them
  for its own select.

**Law**: `a_chevron_press_scrolls_the_popup_one_react_step_and_leaves_no_slot_behind`
(`🔽️Select/🧪️tests/🔬️wgpu-select-keyboard/🦀️.rs`).

---

## 5 — Popover/Dialog content paint folded into `Ui::frame_step`'s retained phase ladder

**React source**: `🗨️Popover`/`💬️Dialog` portal their content and the browser paints the surface and
the modal scrim for them.

**Was**: re-verified and the audit was right — `paint_overlay_backdrop`/`paint_overlay_surface` were
called from nothing but the `🪟️OverlayApi` façade (`Ui::overlay_placements`), and a repo-wide grep
found **zero production consumers of `overlay_placements` outside the crate**. So no host was
hand-pumping it either: an open `Dialog`/`Popover` painted its content with no surface under it, on
every frame.

**Now**:

- `⚙️engine/🦀️.rs:~200` — new `RetainedPaintPhase::Overlays`, between `Synchronize` and `Paint`.
- `⚙️engine/🦀️.rs:~1455` (`frame_step`) and `~1690` (`frame_into_step`) — the phase paints ONE
  overlay's chrome per step from `frame.overlay_chrome`, a fixed
  `[Option<UiOverlayPlacement>; UI_FRAME_OVERLAY_CHROME = 8]` array captured once when the frame
  opens (no per-frame `Vec`).
- `⚙️engine/🦀️.rs:~1856` — `Ui::retained_overlay_chrome` selects the three kinds whose content is an
  ordinary document subtree with no self-painted surface: `Dialog`, `CommandPalette`, `Popover`.
  `SelectPopup`, `ContextMenu` and `Tooltip` are deliberately excluded — each already paints its own
  chrome, and a `SelectPopup`'s overlay ROOT is the trigger's rect, so surface chrome at that
  placement would draw a menu panel over a closed trigger.
- `🖌️paint/🦀️.rs:~3020` — `retained_overlay_chrome_step`, one bounded retained grant
  (`RETAINED_OVERLAY_CHROME_ITEMS = 16`) covering backdrop + surface.
- **Ordering**: the chrome goes into the layer's OVERLAY bucket (`ui_overlay_pass` draws after the
  main and raster passes), so it composites above every panel. That would have *hidden the content
  it exists to carry*, because the content paints in the normal bucket. So `DrawList` gained a
  narrowly-scoped routing depth — `begin_overlay_route`/`end_overlay_route`/`overlay_routed`
  (`🖍️draw/🏷️types/🦀️.rs:~593-640`), with the eight ordinary `push_*` sites routed through
  `active_ui_instances`/`active_vector_vertices` — and the `Paint` phase wraps `paint_node_step` in
  it for any node under a chrome-owning overlay root. `RetainedPaintVisit`/`RetainedPaintWalkStep`
  now carry that root, inherited down the subtree (`⚙️engine/🦀️.rs:133-196`). A caller that never
  opens a pair paints exactly where it always did.

**Law**: `the_frame_ladder_paints_an_open_overlays_own_surface_chrome_under_its_content`
(`🔬️targets-wgpu-engine-unit/🦀️.rs`) — drives `frame_into_step` to `Ready`; no open overlay ⇒ empty
overlay bucket; an open `Popover` ⇒ chrome at the resolved placement, its content AFTER it in the
same bucket, every routing pair closed by publish; an open `SelectPopup` ⇒ still nothing.

---

## 6 — chrome shell Input commit goes through `InputMeta::commit_value`

**React source**: `🗣️Interpreter/🟦️.tsx:1263-1266` — a `number` input's `min`/`max`/`step`, which the
DOM enforces.

**Was**: confirmed still open — zero `commit_value`/`InputMeta::` hits in the Shell wgpu file.
`commit_focused_input` dispatched `"value": input.text_view()` raw.

**Now**: `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:~11045` — `meta.commit_value(input.text_view())`, with its
`Option<DslValue>` result becoming the action's `args` object. `None` (a `number` buffer that does
not parse) dispatches NOTHING rather than inventing a number, which is the primitive's own contract.
The caret blurs either way, because the gesture is over whichever answer the constraint gave.

**Coordination with W14c** (same file): the change is a single surgical replacement inside
`commit_focused_input`'s `input_metas` arm. Nothing of anyone else's was reverted; the file was
re-read between edits and three "modified on disk" notices during the packet were left alone.

---

## 7 — production Select materialises arena children

**React source**: Radix/`SelectContent` mounts its children when the popup opens.

**Was**: **worse than the audit stated.** The audit said "Paint itself (`paint_select`) reads inline
`UiSelectNode.items`, so the VISIBLE menu is fine". Grep says `paint_select` is `#[cfg(test)]`
(`🖌️paint/🦀️.rs:2275`), reachable only from the `cfg(test)` `paint_node`; the PRODUCTION retained
`UiNode::Select` arm paints the trigger only, and `retained_hit_registration`'s `Select` arm mints one
target for the trigger. With `children_of`/`apply_tree` `cfg(test, testkit)`-gated, an open popup on
the live renderer **painted nothing, hit nothing and projected nothing**.

**Now** — the rows are minted by the production stepper, with a bounded ledger rather than by
ungating the recursive allocating `apply_tree`:

- `🌳️tree/🦀️.rs:~383` — `UiTree::composite_rows: Vec<(NodeId owner, NodeId row)>`, capped by
  `UI_COMPOSITE_ROWS = 1_024`; `mint_composite_row`, `retire_composite_row_step`,
  `retire_composite_row_of_step`, `composite_row_count`, `composite_rows_of`, and a `detach_child`
  that is the exact inverse of `attach_child` (`🌳️tree/🦀️.rs:~558-640`).
- `🔀️reconcile/🦀️.rs:1146` — `select_item_row` ungated (now `pub(crate)`), and `with_item_value_arg`
  with it.
- `🖌️paint/🦀️.rs` — `sync_interactive_state_node_step` gains `SelectMint` (a row the scan did not
  find is synthesized, keyed by the item's `value`, then positioned by the existing `SelectWrite`)
  and `SelectRetire` (a CLOSED popup that still owns rows unmounts them, one per step). A popup whose
  item list shrank retires the excess one per step and re-mints anything still declared, so it
  converges without leaking.
- `🔀️reconcile/🦀️.rs:95` + `:939` — a new first reconcile phase `Compose` drains the ledger one node
  per step BEFORE `Adopt`/`Unlink`/`Mount` relink the arena from the record set, which is what would
  otherwise orphan every synthesized child and leak a slot per republish;
  `UiDocumentReconcileCursor::rearm` starts there. `UiTree::close_document_binding_step` drains it
  first too, so a row never outlives the owner it hangs off.

**Law**: `the_production_sync_pass_materialises_an_open_selects_option_rows_and_unmounts_them_on_close`
(`🔬️targets-wgpu-paint-unit/🦀️.rs`) — drives `sync_interactive_state_node_step` directly, with no
`apply_tree` anywhere: closed ⇒ zero rows; open ⇒ one keyed `Button` per item with real layout in
popup order; a second pass ⇒ still two (re-found by key, not re-minted); closed again ⇒ zero rows and
the owner's sibling chain exactly as the rows found it.

**Residual (hand-off)**: the popup's own glass/menu surface is still `cfg(test)`
(`paint_select`'s `push_glass`), so a production open popup now paints real rows with no panel behind
them. That is a paint-only gap on top of a lane that previously painted nothing; it is NOT covered by
item 5's overlay-chrome phase, which deliberately excludes `SelectPopup` (see item 5).

---

## 10 — `UiDriverTooltips` axis + `Ui::set_window_flow` in production

### 10a `UiDriverTooltips`

**React source**: `🧱️elements/🚗️UiDriver/🟦️.tsx:23` (the axis), `:41`/`:43` (`DEFAULT_UI_DRIVER`
`tooltips: "full"`, `COMPACT_UI_DRIVER` `tooltips: "none"`), `:80-84` (`resolveUiDriver`), and
`🏷️Label/🟦️.tsx:186-190` (`useControlTooltipText`: `if (driver.tooltips === "none") return
undefined`, then `if (!always && inlineText) return undefined`).

**Was**: zero `UiDriver` hits anywhere in `🎯️targets/🧊️wgpu`, confirmed. The Shell carried
`driver_id`/`custom_drivers` and never read either for presentation.

**Now**:

- `🖥️chrome/🦀️.rs`, region `🎙️DriverTooltips` — `UiDriverLabels`, `UiDriverTooltips`
  (`from_axis`, React's own three allowed values), and `UiDriverChrome` with `DEFAULT`/`COMPACT`,
  `builtin(id)`, `with_axes(labels, tooltips)` (a custom driver's `config` overriding the builtin,
  keeping the builtin for an axis it omits or spells wrong — React throws there, a renderer cannot),
  and `tooltip_shows(label_visible)` = React's two guards. Re-exported from the target root.
- `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — `ShellChromeBuildState::driver`, resolved by
  `resolve_ui_driver_chrome(driver_id, custom_drivers)` (React's `resolveUiDriver` ladder) at all
  three places preferences land: the boot load phase, the preference re-sync, and the
  `os.setDriver` mutation.
- The gate lands on the ONE production tooltip read, `render_chrome_tooltip_step` phase 0 (and its
  `cfg(test)` twin `render_chrome_tooltip`): a driver with `tooltips: "none"` clears the armed hover
  and paints nothing.

**Not ported** (stated, not hidden): React's `full` vs `minimal` tiers differ only by the
manual/tutorial links a full tooltip adds, and this target's tooltip surface paints a label (plus its
declared shortcut) and nothing else — so the two tiers are behaviourally identical here and only
`none` changes what the user sees. `tooltip_shows`'s `label_visible` half is implemented and tested
but every current chrome call site passes `false`, because the wgpu chrome registers tooltip titles
for icon-only controls; a caller that paints an inline caption can pass `true` without further work.

### 10b `Ui::set_window_flow`

**React source**: `🖼️Panel/🟦️.tsx:516` — every anchored `Panel` is wrapped in a `FlowProvider` whose
value is `flowFromAnchor(anchor)`. A window body/dialog gets no provider and stays at the default.

**Was**: `Ui::set_window_flow` had zero production callers — every window sat at `UiFlow::DEFAULT`.

**Now**:

- `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:~1393` — `set_ui_document_flow(window_id, flow)` over the
  thread-local `UI_ENGINE`.
- `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:~3253` — `PanelAnchor::contract_anchor()` (this target's physical
  `Left`/`Right` anchors onto the contract's logical `Start`/`End`, the same mapping
  `anchorHorizontal` makes) and `PanelAnchor::flow()` = `UiFlow::for_anchor`.
- `🐚️Shell/…/🦀️.rs:~20900` — the anchored-panel document painter calls
  `set_ui_document_flow(window, anchor.flow())` before `render_ui_document_step`. The mobile
  tab-bar panel path is deliberately untouched: it has no anchor, and React gives it no
  `FlowProvider` either.

Latent rather than visible today — both shipped locales are LTR and only an `End`-anchored panel
turns it on — but the seam is real instead of dead.

---

## Gates

| gate | result |
|---|---|
| `cargo check -p semio-framework-ui --features wgpu-engine --lib` | **0 errors** (`🗑️generated/w15a-check-5.txt`). Baseline had 4 `never used` warnings; two of them (`select_scroll_step`, `select_clamped_scroll`) are gone because item 4 consumes them. The remaining `focus_visible` warning is pre-existing and not this packet's. |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib --tests` | **0 errors** (`🗑️generated/w15a-check-tests-2.txt`) |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib` | see `🗑️generated/w15a-ui-test-1.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib` | see `🗑️generated/w15a-renderer-check-2.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` | see `🗑️generated/w15a-renderer-wasm32.txt` |
| `interactivityMountedLayoutTextSelfTests` | see below |
| `bun nx run @semio-tech/ui-rs:test-wgpu-engine` | see below |

## NOT run (out of packet scope, stated per the brief)

- No wasm build, no `activate-*`, no serve, no browser probe — W14d/W14f own live verification of
  every change here.
- No CAD work (audit items 1/8/9 — W14g).
- The `world`/`infinite` and Shell-side test suites were not run; this packet's own gates are the ui
  crate's lib tests plus the two renderer checks.

## Hand-offs

1. **Select popup surface** (item 7 residual): production now paints real option rows with no glass
   panel behind them, because `paint_select`'s `push_glass` is `cfg(test)`. Whoever owns the next
   Select lane should either ungate the popup surface or give the `SelectPopup` overlay kind a
   placement that is the POPUP's rect rather than the trigger's, at which point item 5's chrome phase
   can cover it.
2. **`push_raster_quad` inside an overlay**: the raster bucket has no overlay twin, so an `Image`
   inside an open `Dialog`/`Popover` still composites in the raster pass, under the overlay chrome.
   Bounded, visible only for a `data:` image inside a modal.
3. **`full` vs `minimal` tooltip tiers** are indistinguishable on this target until the tooltip
   surface grows manual/tutorial links (item 10a).
4. **`label_visible`** is wired through `UiDriverChrome::tooltip_shows` but every chrome call site
   passes `false`; a caller that paints an inline caption should pass `true`.
