# wgpu text inputs: React parity fix (Input / longText / NumberStepper)

Status: code written (round 1 plus the round 2 section below), **NOT compiled or tested by me** (host saturated; one `check -p semio-ui-wgpu-paint` was still building dependencies when the build freeze arrived). Only the TypeScript twin was run (51 checks, passed with bun). An integration agent must compile and run the tests listed at the end.

## Audit claims verified
- Selection highlight absent from the live painter (only the test-only reference painter drew one): confirmed.
- IME preedit absent from Input (stepper only): confirmed.
- longText painted one line via `text_line_view`: confirmed. Single-line text also used wrap flow with no inset or scroll, so a long value wrapped below the box.
- Refusal message not painted for Input: confirmed (red border only).
- Stepper "Mixed" hard-coded English: confirmed. Another agent meanwhile added `RetainedPaintPresentation.locale` and `UiCommonLabels.mixed_values` (en "Mixed", de "Gemischt"); I wired the stepper to it.
- Editing keys: copy, cut, paste, select-all, Home/End, Shift-selection already existed; missing were word jumps, word/line deletion, vertical movement, line-aware Home/End, Meta line motion, newline policy on paste.

## Changes (live path only: paint, events, layout, a11y)
Paint (`🖌️paint`):
- New `✍️retained-edit-text-steps/🦀️.rs`: `edit_view` (text with preedit spliced in at caret; selection and preedit as byte ranges), `EditGeometry`, `edit_scroll`, `input_text_rect` (React `p-single` inset), probe/span/text/caret steps.
- `🔤️retained-glyph-and-node-cursors`: incremental wrap probe (`retained_wrap_probe_step`) that applies the same break law as `FontAtlas::wrap_lines` and the glyph painter, so highlight/caret positions agree with painted text; cursor gains `probe` and `edit_caret`.
- `🎛️control-nodes::paint_input_node_step` rewritten, phases: border+scissor, caret probe, selection/preedit highlight, text, caret, pop scissor, refusal message, presence.
  - Selection: opaque `theme.accent` fill, selected glyphs in `theme.border_emphasized` (React `::selection { background: var(--accent); color: var(--border-emphasized-color) }`, `🎨️styling/🖌️ui/🎨️.css`).
  - Preedit: underline in `theme.text`, caret at the IME's own cursor inside it (new `WidgetState.composition_cursor`).
  - Caret now `theme.text` (native caret = currentColor); was accent. Stepper caret likewise.
  - longText: wrapped multi-line, vertical scroll that keeps the caret line in view; single-line: horizontal scroll keeping the caret in view, text clipped by scissor.
  - Refusal: `theme.error`, `font_size_small`, directly below the field (React: `<span role="alert" class="text-destructive text-xs">` under the input). Limitation: layout does not reserve that row, so a later-painted sibling can cover it.
- Stepper: selection highlight and preedit underline beneath the centred value, caret in text colour at the preedit cursor, Mixed caption localized via `presentation.locale` (en "Mixed", de "Gemischt"; no locale gives React's "—" fallback). Phases renumbered 0..8.
- Removed `text_line_view` (obsolete, no compat shim) and its asserts.
- Layout: `layout::TEXTAREA_MIN_HEIGHT` (4.5 rem = 72 px, React `min-h-[4.5rem]`) used for longText in mounted layout.

Events (`⚡️events`): `route_edit_key` adds Ctrl/Alt+Arrow word motion (Alt: word end, Ctrl: next word start, both back to previous word start), Meta+Arrow line start/end, Home/End by logical line, Ctrl+Home/End, ArrowUp/Down across logical lines keeping the column (Meta: text start/end), Ctrl/Alt+Backspace/Delete word, Meta+Backspace to line start. Paste into a single-line field turns line breaks into blanks. IME update stores the preedit cursor.

Accessibility: refused Input projects an `alert` node (`<key>::refusal`, assertive, not focusable) right after the invalid field.

## Tests added
- Shared vectors: `🧬️contract/🧪️tests/🧫️fixtures/🧫️text-field-editing/🔣️.json` (36 key rows, 4 paste rows).
- Rust: `🧪️tests/🔬️targets-wgpu-events-unit/✍️text-field-editing-vector-laws` (included from its `🦀️.rs`): key rows, paste rows, IME cursor.
- Rust paint laws appended to `🧪️tests/🔬️targets-wgpu-paint-unit/✍️input-caret-and-focus-ring-paint-laws/🦀️.rs` (`RetainedTextInputLaws`): selection highlight and colours, caret position/colour, preedit underline and caret, longText 72px box + wrap + scroll, post-newline caret, two-line selection, horizontal scroll, refusal message, localized Mixed (none/en/de), stepper selection. Existing retained caret test updated to the text colour.
- Rust a11y: `a_refused_number_draft_projects_an_alert_after_its_invalid_field` in `🧪️tests/🔬️targets-wgpu-accessibility-projection/🦀️.rs`.
- TS twin: `🧬️contract/🧪️tests/🧪️text-field-editing/🟦️.ts`, registered in `🧬️contract/📦️packages/🦀️rust/📜️script.ts`; independent model plus `Intl.Segmenter` word boundaries as third-party oracle. **Run: 51 checks, passed.**

## To run (integration agent; from /Users/ueli/Documents/semio, one cargo at a time)
`repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml -p <crate>` for the laws crates covering: events-unit (`semio-ui-wgpu-events-laws`), paint-unit (`semio-ui-wgpu-paint-laws`), accessibility-projection laws, mounted-layout/layout laws (TEXTAREA_MIN_HEIGHT), and `check -p semio-ui-wgpu-paint` first. Also the contract package TS script for the new twin.

## Known risks / unverified
- Never compiled: expect small compile fixes (imports such as `Cow`, `semio_ui_text`/`semio_ui_locale` in test crates, `ui_contract::AccessibilityProjectionNode` path in the a11y file).
- Paint crate source boundary: `semio-ui-wgpu-paint` hit 4209 active lines against 4000 earlier; its Cargo.toml now says 4300 (raised by another agent). Re-check after merge; the new module may need a crate split.
- `maxSourceFiles = 32` for the paint crate: I added one file.
- Another agent's `SCROLLBAR_SIZE_PX` (`🧮️layout/🦀️.rs`, `* 2` on an f64) failed to compile at one point; not mine.
- Remaining gaps: textarea does not grow with content (`field-sizing-content`); wheel scrolls a textarea only while it is focused (the layout snapshot exists for the focused or hovered field); Date/File/Color pickers unchanged; word motion is class-based and the caret steps per scalar, not per grapheme cluster; double/triple-click then drag does not extend by word/line; a first click that focuses a field places the caret at the end unless it was hovered first.
- The reference painter and `🪀️widgets` immediate path were not touched, per the coordinator's liveness note.

## Round 2 (code only, still uncompiled)
Design: the router has no font, and `Ui` never owns the atlas. So the paint walk (which has the atlas) parks a **layout snapshot** on the node and events read it: no new dependency, no measurer trait.
- `tree::EditTextLayout`/`EditTextStop` (in `🧩️node-model`): every caret position (byte, visual line, pen) of the live text plus `byte_at`, `vertical`, `max_scroll`, `reveal`; `WidgetState` gains `edit_layout`, `caret_goal_x`, `refusal_row_laid`; `UiTree::request_layout`.
- `paint::refresh_input_text_layout` (new, in `✍️retained-edit-text-steps`): called from the engine frame step right after each node's interactive sync (`⚙️engine/🖥️frame-into-step-and-frame`); relays the text with the same break law as the painter (probe), caps at 64 KiB, keeps the caret in view by writing `scroll_offset` only when the caret moved. The painter now reads `scroll_offset` instead of deriving scroll from the caret (`edit_scroll` removed).
- Events: pointer press places the caret at the nearest stop (Shift extends), drag extends, double click selects the word, triple click the logical line (0.4 s, 4 px); `text_click`/`text_drag` on the router; wheel over a focused textarea scrolls it and yields to ancestors at its limits; ArrowUp/Down walk **visual** lines with a remembered pen (`caret_goal_x`), falling back to logical lines when no current snapshot exists.
- Refusal row reserved: `layout::input_refusal_row_height` (`font_size_small * 1.25`, React `text-xs leading-tight`) is added to the Input's layout height in mounted layout; the painter keeps the box at the old height and paints the message in the extra row; `paint::sync_input_refusal_rows` (called after every dispatch in the engine) requests a layout pass when a row appears or vanishes.
- New tests: events-unit `vertical_motion_follows_the_visual_lines_and_keeps_the_pen`, `a_press_places_the_caret_and_a_drag_selects`, `double_click_selects_the_word_and_triple_click_the_line`, `the_wheel_scrolls_a_textarea_within_its_layout`, `an_edit_layout_answers_nearest_stop_neighbours_and_scroll` (hand-laid layouts, no font needed); paint-unit `a_refusal_reserves_its_row_under_the_box_and_the_layout_pass_is_asked_for_it`, and the long-text/single-line scroll laws now refresh the layout first and assert stops against `FontAtlas::wrap_lines`.

### Run list (one cargo at a time, status: NONE RUN)
1. `check -p semio-ui-wgpu-paint`, then `-p semio-ui-wgpu-events`, `-p semio-ui-wgpu-mounted-layout`, `-p semio-ui-wgpu-engine` (hook sites), `-p semio-ui-retained-tree-tree`.
2. `test --manifest-path 🧰️framework/Cargo.toml -p semio-ui-wgpu-events-laws` (text-field vectors incl. new pointer/wheel/vertical laws, mirrored keyboard laws).
3. `... -p semio-ui-wgpu-paint-laws` (`✍️input-caret-and-focus-ring-paint-laws`, stepper laws).
4. accessibility-projection laws (alert node) and layout/mounted-layout laws (TEXTAREA_MIN_HEIGHT, refusal row).
5. Contract package script (TS twin, already 51 checks green under bun).
Things most likely to need a fix: the `PointerDown { modifiers }` binding in the pointer dispatch arm; `layout_tree_now` honouring `request_layout` in the new paint law; the new `semio_ui_locale` and `semio_ui_text` paths in test crates; the paint crate's active-line budget (4300 in its Cargo.toml).
