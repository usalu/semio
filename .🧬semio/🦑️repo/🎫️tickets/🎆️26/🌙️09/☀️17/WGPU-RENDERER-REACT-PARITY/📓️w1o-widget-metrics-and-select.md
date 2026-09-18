# W1o — Widget metric + behaviour parity, and Select's keyboard/placement

Packet W1o of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`: audit work packets **9** (icon size and
line height off the shared token source) and **10** (Select keyboard navigation + collision-aware
placement), plus the audit's remaining P1/P2 widget rows not owned by W1l/W1m/W1n
(`📓️audit-interpreter-elements.md` §2, §3, §5.2).

Everything below is a port of a number or rule that exists in React source, cited file:line. Where
React and wgpu genuinely cannot agree (no bold font face on disk, no `focus-visible` in a canvas),
that is stated as a bounded gap rather than papered over.

## 1. Metric table — React value → wgpu before → wgpu after

| Metric / behaviour | React (file:line) | wgpu before | wgpu after (file:line) |
|---|---|---|---|
| **Icon, inline control** (Button, Toggle, Select chevron, IconSelect) | `renderControlIcon`'s default `size="small"` → CSS class `size-small` = `calc(5 × --ui-spacing)` = **16px** (`🧱️elements/🔣️Icons/🟦️.tsx:419, 452-470`; `🗣️Interpreter/🟦️.tsx:975`) | `ICON_TINY = 14.0` hardcoded | **16px**, `chrome::ICON_TINY` off `chrome.iconInlineUiSpacing` — landed by the peer token pass before this packet; verified and locked by a test here |
| **Icon, tree row + row actions** | `resolveControlIconNode(icon, 12)` = **12px** (`🗣️Interpreter/🟦️.tsx:251, 1861, 1874`) | `TREE_ICON_SIZE = 14.0`, duplicated in two files | **12px** — `chrome::ICON_TREE_ROW` (`🖥️chrome/🦀️.rs:22-25`), consumed by `🖌️paint/🦀️.rs:47` and `🪀️widgets/🦀️.rs:207` |
| **Icon, tree ROW fold chevron** | `size-tiny` = `calc(3 × --ui-spacing)` = **9.6px** (`🧱️elements/🌳️Tree/🟦️.tsx:4576`) | `ICON_TINY` (14, then 16) | **9.6px** — `chrome::SIZE_TINY` (`🖥️chrome/🦀️.rs:15-20`), used at `🖌️paint/🦀️.rs:407, 2350` and `🧱️elements/🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs:207` |
| **Icon, SECTION/group fold chevron** | `size-small` = **16px** (`🌳️Tree/🟦️.tsx:251, 2182, 2609, 4401`) | same const as the row chevron — the two could not differ | **16px**, now a per-call-site argument: `tree_draw_chevron(…, size)` (`🪀️widgets/🦀️.rs:468-474`) |
| **Panel/section header height** | `--size-medium` = `calc(7 × --ui-spacing)` = **22.4px** (`chrome.panelHeaderHeightUiSpacing`) | `PANEL_HEADER = 24.0` literal, in two files | **22.4px** token-derived (`🖌️paint/🦀️.rs:47`, `🪀️widgets/🦀️.rs:200`) |
| **Line height, wrapped text** | per-token ramp `--text-*--line-height`: **14.4 / 16 / 19.2 / 22.4 / 25.6 px** for `2xs/xs/sm/base/lg` (`🎨️styling/🖌️ui/🎨️.css:803-812`) | `size * 1.35` (12.8px text → 17.28px, 11% short) | exact ramp — `text::line_height` (`📝️text/🦀️.rs:88-116`), fed by five NEW generated tokens (§4) |
| **Line height, single line** | same ramp — a `<p>` line box is the same height wrapped or not | `max_height.max(size)` — raw glyph bbox, **disagreed with wgpu's own wrapped formula** | same `line_height` (`📝️text/🦀️.rs:520-530`), so the two agree by construction |
| **Line height, paint baselines** | — | `size * 1.35` again, a third copy | `text::line_height` (`🖌️paint/🦀️.rs:124`, `:1958`), and the layout worker's glyph preview (`📌️mounted_layout/🦀️.rs:160`) |
| **Text color, plain** | `text-foreground` — `TextView` is `text-foreground` in BOTH states and only swaps `text-sm` for `font-semibold` (`🗣️Interpreter/🟦️.tsx:1168`) | `theme.text_muted` for non-emphasized — every plain paragraph was dimmed | `theme.text` in both states (`🖌️paint/🦀️.rs:620-627, 1958`) |
| **Text weight, emphasized** | `font-semibold`, same size | larger size, no weight | unchanged (size swap) — **no bold `.ttf` ships**, documented gap (§5) |
| **Progress track height** | `h-tiny` = **9.6px**, `w-full` (`🗣️Interpreter/🟦️.tsx:2114`) | `theme.padding_standard` = 3.2px sliver | **9.6px** `chrome::SIZE_TINY` (`🖌️paint/🦀️.rs:1874`) |
| **Progress corner radius** | `rounded-full` → `--radius-full`, which this design system pins to **0** (`🖌️ui/🎨️.css:772-781`) | `height * 0.5` — a pill nothing else in the UI has | `theme.border_radius` (`🖌️paint/🦀️.rs:1887-1893`) |
| **Progress fill color** | `bg-accent` (`🗣️Interpreter/🟦️.tsx:2116`) | `theme.progress` (the outcome palette) | `theme.accent` |
| **Separator stroke** | `<hr>` border = `--stroke-hairline` | literal `1.0` | `theme.stroke_hairline` (`🖌️paint/🦀️.rs:1968`) — same value today, no longer a literal that can drift |
| **Focus ring color** | `formControlFocusBorderClass` = `focus-visible:border-accent` (`🔨️modules/📝️form-control-presentation/🟦️.ts:14`) | `theme.border_emphasized` — an unrelated border token | `theme.accent` at all 11 control focus sites (`🖌️paint/🦀️.rs`, Button/Input/Select/Toggle/IconSelect/NumberStepper, retained + `cfg(test)` painters) |
| **Select popup row height** | `SelectItem` is `py-single … text-sm` → 19.2 + 2×3.2 = **25.6px** (`🧱️elements/🔽️Select/🟦️.tsx:729`) | `theme.control_height` = 22.4px | `select_row_height` (`🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs:64-66`) |
| **Select popup offset / inset** | `sideOffset = 4`, viewport `p-single` = 3.2 (`🔽️Select/🟦️.tsx:476`, `select-viewport`) | `2.0` and `2.0` literals | `SELECT_SIDE_OFFSET = 4.0`, inset `theme.padding_standard` (`🔽️Select/…/🦀️.rs:54-99`) |
| **Select row text origin** | `ps-single` inset, vertically centred | `row.x + 8.0`, baseline `row.y + 18.0` | `padding_standard` inset, centred baseline (`🖌️paint/🦀️.rs` retained rows + `paint_select`; `render_select_menu`) |
| **Select popup placement** | `resolveSelectPlacement`: flips above when the content does not fit below and there is more room above; `collisionPadding = 8` (`🔽️Select/🟦️.tsx:245-265, 477`) | always `bounds.y + bounds.h + 2.0` | `select_menu_top` (`🔽️Select/…/🦀️.rs:80-92`), resolved per node from its absolute rect and the tree root's height (`select_menu_top_for`, `🖌️paint/🦀️.rs`), used by the retained painter, the row layout sync and `paint_select` alike |

## 2. Keyboard state machines

All three tables below are ported rule-for-rule; the Select one lives with the element as pure
functions (`🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs` region `🔖️Keyboard`) and is routed by
`events::EventRouter::route_select_key`.

| Control | React | wgpu before | wgpu after |
|---|---|---|---|
| **Select, closed** | `ArrowDown`/`Enter`/`Space` open on the selected row, `ArrowUp` on the last, a printable key opens on the first match (`🔽️Select/🟦️.tsx:399-408`) | nothing — keyboard could not open a Select at all | identical (`select_key` + `select_open_index`) |
| **Select, open** | `ArrowDown`/`ArrowUp` wrap, `Home`/`End`, `PageDown`/`PageUp` ±10 clamped, `Enter`/`Space` commit the highlighted row, `Tab` closes without restoring focus, `Escape` closes, printable keys extend a 700 ms typeahead query scanning cyclically from the active row (`🔽️Select/🟦️.tsx:435-466, 641-672`) | only `Escape` (generic overlay dismissal) and mouse clicks | identical, including the `-1`-for-nothing-highlighted arithmetic; the query expires against `EventRouter`'s existing monotonic clock |
| **Select highlight** | `aria-activedescendant` / `data-highlighted` row | no concept | `tree::WidgetState::highlighted`, painted like a hovered row by the retained painter and `paint_select`, cleared on every dismissal path (`finish_close`) |
| **Slider** | `ArrowRight`/`ArrowUp`/`PageUp` +1 step, `ArrowLeft`/`ArrowDown`/`PageDown` −1, `Home`/`End` to the ends, ×10 for page keys or any `Shift` chord (`🧱️elements/🎚️Slider/🟦️.tsx:383-396`) | none — a focused slider ignored every key | `slider_key_value` (`⚡️events/🦀️.rs`), clamped to the track |
| **Toggle** | renders as a `<button>`, so `Enter`/`Space` click it | none (`focused_button_activation` matched `UiNode::Button` only) | `focused_value_key_activation` fires its `Change` with the flipped value |
| **NumberStepper** | native number input arrows | none | `ArrowUp`/`ArrowDown` = its ± segments, through the SAME binding-gated `Delta`-vs-`Change` rule a click takes (`number_stepper_fired`, now shared by both call sites) |
| **Tab order / Escape / focus trap** | native DOM order; dialogs trap | already implemented (`focus_next`/`focus_prev`, `topmost_focus_trap_root`) | unchanged, except `Tab` on an open Select now closes the popup AND still moves focus (React's `setOpen(false, false)`) |

## 3. Localisation / terminology

`🏷️label/🦀️.rs`'s `Locale × Terminology` matrix already resolves every wgpu label, and both enums are
generated from the same `🎚️axes/🔣️.json` React's `SHELL_LOCALES`/`SHELL_TERMINOLOGIES` come from. That
was asserted rather than assumed: a new test reads the JSON itself and compares ids, order and count
against `Locale::ALL`/`Terminology::ALL`, and a second proves the tiering (locale-sensitive
`native`, invariant `data`, one distinct cell per pair) — `🧪️tests/🔬️targets-wgpu-label-localized-label-value-round-trip/🦀️.rs`.

Typeahead matching needed the same care: React normalises with NFKD + mark stripping, so "Ärger"
matches "a". Rust std has no Unicode normalisation and this crate takes no dependency for one, so
`normalize_select_text` strips already-decomposed marks AND folds the Latin-1 Supplement through a
64-entry table (`LATIN1_BASE`) — exact for German/French labels, identity elsewhere, same as NFKD
leaves them.

## 4. Token source changes (W1k please note)

Added to `🎨️styling/🔣️.json` and regenerated (`bun ./📜️script.ts generate` in
`🎨️styling/📦️packages/🦀️rust`):

- `metrics.typography.text{2xs,Xs,Sm,Base,Lg}LineHeightPx` = `14.4 / 16.0 / 19.2 / 22.4 / 25.6` —
  the px values `--text-*--line-height` computes to at the 16px compact root. The CSS still carries
  its own `calc()` ratios; folding those into the generated source is W1k's call.
- `metrics.chrome.sizeTinyUiSpacing` = `3.0` — `--size-tiny`, the one step of the CSS `--size-*` ramp
  with no generated token (`--size-small`/`--size-medium` are `controlHeightSmall`/`controlHeight`).

## 5. Remaining gaps (not fixed here)

1. **No bold font face on disk** — `emphasize` still swaps font SIZE, not weight. Shared asset gap
   (`📝️text/🦀️.rs` header); fixing it needs a bold `.ttf` for Anta/Kelly Slab.
2. **No `focus-visible` distinction** — a pointer-focused control shows the accent ring React would
   only show for keyboard focus. Needs a "last input was a key" bit on `EventRouter`.
3. **`bg-muted` has no `Theme` field** — the progress TRACK uses `theme.separator`; React uses
   `--muted`. One new theme field (W1k's lane) would close it.
4. **Select popup, remaining React features**: no viewport max-height clamp / scroll buttons (React's
   `availableHeight`), no `side`/`align`/`position` props, no per-item icon or check mark, no
   disabled rows. The immediate-mode kit (`render_select_menu`) also never flips, because
   `WidgetContext` carries no measured viewport — it passes `0.0`, which `select_menu_top` treats as
   "place below". Threading a viewport into that struct touches the Interpreter target's public
   `framework_widget_context`, so it is left for whoever owns that seam.
5. **`UiCommand::FocusChanged` is still an intentional no-op** in the Interpreter wgpu target
   (`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:504`), deferred to `w3-shell-input-cutover`'s
   `note_content_focus_commands`. Unchanged by this packet, re-confirmed still deferred.
6. **Checkbox, radio group, tabs, badge/chip** — none is a `ui_contract::Component` kind, and React's
   own generic `Tabs`/`Chip`/`Checkbox` elements have zero production call sites through the
   Interpreter (audit §3). Nothing to port; a real tri-state checkbox would be a contract change on
   BOTH renderers, not a wgpu gap.
7. **Input `date`/`color`/`file` kinds** have no native picker on a canvas (audit §2, W1m's lane).
8. React's `UiLabelPair {normal, beginner}` experience axis (audit §6) still has no wgpu counterpart.

## 6. Tests

New:
- `🧱️elements/🔽️Select/🧪️tests/🔬️wgpu-select-keyboard/🦀️.rs` — 9 tests: open/navigate/commit/close
  key tables, wrap-and-clamp arithmetic, cyclic typeahead (incl. accent folding), normalisation, open
  intents, row/menu metrics, below-vs-flipped placement, row rect stacking.
- `🧪️tests/🔬️targets-wgpu-widget-metrics/🦀️.rs` (mounted from `🪀️widgets`) — icon boxes are React's
  three distinct sizes, tree metrics equal the theme tokens, progress track geometry both determinate
  and indeterminate.

Extended:
- `🧪️tests/🔬️targets-wgpu-text-unit/🦀️.rs` — per-token line box, off-ramp proportionality,
  wrapped-vs-single-line agreement.
- `🧪️tests/🔬️targets-wgpu-label-localized-label-value-round-trip/🦀️.rs` — axes-source parity, tiering.
- `🧪️tests/🔬️targets-wgpu-events-unit/🦀️.rs` — 9 routing tests (Select open/wrap/commit/typeahead/
  Tab/disabled, Slider keys and values, Toggle Enter/Space).
- `🧪️tests/🔬️targets-wgpu-paint-unit/🦀️.rs` — progress and focus-ring expectations updated to the new
  React-derived values; the two open-Select tests now reconcile with `state.open` set (a closed
  Select materialises no rows since the `children_of` gate landed, so they were asserting on a tree
  that had none).

**Verification** (foreground, logs in `🗑️generated/w1o-*.txt`):

| Command | Result |
|---|---|
| `cargo check -p semio-framework-ui --features wgpu-engine --lib --keep-going` | ✅ 0 errors (`w1o-check-2.txt`) |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib --target wasm32-unknown-unknown` | ✅ 0 errors (`w1o-check-wasm.txt`) |
| `cargo test … --lib -- <select/keyboard/line_height/icon/progress/axes/label/slider/toggle/focus filters>` | ✅ **66 passed, 0 failed** (`w1o-tests.txt`) |
| `cargo test … --lib` (whole suite, final run) | 494 passed / **16 failed**, none in the W1o surface: 7 `prepared` permit/witness tests, 3 `mounted_layout` (space-between, overlay out-of-flow, cross-axis align — W1l's live spacing/flex work), 1 `layout::tree_row_rect` fixture off by 0.008px of the new spacing ramp, the scene golden-JSON, the engine slot-table budget and hostile-fixture arena, one `reconcile` document-tree test, and one perf slice (63ms) measured under full fleet load. An earlier run of the same suite did not compile at all while `flex::FlexTree::solve`/`MountedLayoutJob::measure_one` were mid-rename in W1l's lane — that has since cleared. |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --keep-going` | 13 errors, ALL in `🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` (`canvas_sat`/`canvas_lum`/`canvas_set_*` missing — another lane's half-landed edit). The `semio-framework-ui` dependency compiled clean on the way there (`w1o-check-renderer.txt`). |

**Peer-stale code I had to fix to get a compiling test binary** (additive only, nothing reverted):
`🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs` (7 `Input` literals missing W1m's new `min`/`max`/`step`/
`accept`/`input_kind`), `🧪️tests/🔬️targets-wgpu-events-unit/🦀️.rs` + `🧪️tests/🎛️retained-control-commit`
(`UiCommand::App { action }` → `{ intent }`, asserted via `intent.descriptor()`), and one borrow
error in `🧪️tests/🔬️targets-wgpu-action-unit/🦀️.rs:321`.

## 7. Files changed

```
🎨️styling/🔣️.json                                        (+6 tokens, regenerated)
🎯️targets/🧊️wgpu/📝️text/🦀️.rs                            line_height ramp + both measure paths
🎯️targets/🧊️wgpu/🖥️chrome/🦀️.rs                          SIZE_TINY, ICON_TREE_ROW
🎯️targets/🧊️wgpu/🦀️.rs                                   re-exports
🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs                           icon sizes, line heights, text color,
                                                          progress, separator, focus ring, select
                                                          geometry + keyboard highlight
🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs                         TREE_ICON_SIZE, PANEL_HEADER, chevron size
🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs                  glyph preview line height
🎯️targets/🧊️wgpu/🌳️tree/🦀️.rs                            WidgetState::highlighted
🎯️targets/🧊️wgpu/⚡️events/🦀️.rs                          route_select_key, typeahead buffer,
                                                          focused_value_key_activation,
                                                          slider_key_value, number_stepper_fired
🧱️elements/🔽️Select/🎯️targets/🧊️wgpu/🦀️.rs               🔖️Geometry + 🔖️Keyboard, kit updated
🧱️elements/🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs                 chevron sizes
🧪️tests/…                                                see §6
```
