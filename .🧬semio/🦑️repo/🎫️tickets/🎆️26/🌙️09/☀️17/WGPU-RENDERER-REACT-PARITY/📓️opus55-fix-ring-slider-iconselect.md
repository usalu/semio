# Ring, Slider, IconSelect: wgpu to React parity (opus55)

STATUS: code written, NOTHING COMPILED OR TESTED SUCCESSFULLY by me. One `semio-ui-wgpu-paint-laws` run was started (first attempt failed on someone else's `SCROLLBAR_SIZE_PX` `* 2` typo, which I fixed to `2.0`; a second hit a source-boundary line cap that has since been raised to 4300 by another stream). The last run had not finished when the no-new-cargo policy arrived. Integration agent must compile and run everything below.

## Audit claims verified against source
- Ring angle (TAU*t from 3 o'clock in paint, kit, ring_t_at): CONFIRMED. React: t=0 at 12 o'clock clockwise (`t*2π-π/2`; drag `atan2+π/2`). The TS twin `retained-control-commit/🟦️.ts` and its fixture encoded the old wrong convention too.
- Ring keyboard: React Ring has no tabIndex/key handler (unfocusable SVG). wgpu listed Ring as focusable. CONFIRMED.
- Slider hover/focus/drag/disabled visuals missing in retained painter: CONFIRMED (no flags passed). React dial needle is `stroke-active-base`; wgpu used accent: CONFIRMED.
- Slider readout: retained path already equals React (`ui_number_display_text` == `uiNumberDisplayText`/`formatNumber`). Kit path cannot express unit/factor/precision (its node has no such fields); for its domain it equals React. Kit/immediate path is dead per coordinator, so not touched.
- IconSelect: audit says "icon grid"; React has none. React = mode select + textarea + 56px preview cell + import/clear buttons, locked (`disabled || !uniform`) at opacity-60. wgpu painted the raw value text: CONFIRMED.

## Changes (live path only)
- Theme: `accent_foreground` (🎨️styling/🎭️theme/🧩️component/🦀️.rs).
- Layout (🧮️layout/🦀️.rs): `RING_SIZE`, `RING_RADIUS`, `ring_frame/center/angle/point`, `ring_t_at` now React's convention (frame is React's fixed 100px square at inline start); `ICON_SELECT_PREVIEW_HEIGHT` (size-peta); `SCROLLBAR_SIZE_PX` literal fix (other stream's typo). Mounted layout: Ring node is 100px tall, IconSelect is control row + preview height.
- Paint: `paint_slider_travel` takes `SliderTravelState {hovered, dragging, focus_visible, disabled}`: hover = emphasized foreground (theme.text), dragging = active-base (theme.selected), focus-visible thumb = active-base fill inside border-emphasized ring, disabled = 0.5 alpha without the extra scrim, dial needle active-base, readout text emphasized on hover. `paint_ring_track_segment` (muted-foreground/30, 2 hairlines) and `paint_ring_orb` (r 5/7, accent fill + accent-foreground 1-unit stroke when selected, muted/40 disabled) shared by retained step and reference painter. Retained IconSelect: editor row (inner editor text, not raw value), preview cell (icon at 56px via atlas, else glyph text, else muted dash), hover/focus borders, 0.6 alpha lock.
- Declaration: `IconSelectorMode`, `UiIconSelectNode::{decode_mode, locked, editor_text}` (Rust twin of React `decodeIcon`/`classifyIconSelectorMode`/`innerFromIconForSelectorMode`).
- Events: Ring removed from `is_focusable` (⚡️events/🧭️event-model-and-focus).
- TS twin + fixture `🎛️retained-control-commit` ring case: now 0.5 for a press straight below.

## Tests / fixtures added
- Fixture `🧪️tests/🧫️fixtures/⭕️ring-angle/🔣️.json` (knob offsets and drag to t vectors, 3 frames) used by Rust `ring_value_and_angle_match_the_shared_react_vectors` (🔬️targets-wgpu-layout-unit) and by TS `🧪️tests/⭕️ring-angle/🟦️.ts` (renders the real React `Ring` for knob positions; the drag half re-states React's formula, it does not call the component).
- Fixture `🧪️tests/🧫️fixtures/🎴️icon-selector-mode/🔣️.json` used by Rust `icon_selector_modes_match_the_shared_classifier_vectors`. No TS consumer written.
- Paint laws `🔬️targets-wgpu-paint-unit/🎴️ring-slider-and-icon-select-state-paint-laws/🦀️.rs` (included from 🦀️.rs): ring orb positions/colors/disabled, slider resting/hover/drag/focus-visible/disabled/readout, icon selector text/dash/lock/focus.
- Events law: `a_ring_never_takes_tab_focus...` in 🔬️targets-wgpu-events-unit (focus-laws file).

## Tests to run (all unverified)
From /Users/ueli/Documents/semio: `repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml -p <crate>` for: semio-ui-wgpu-paint-laws, the events laws crate (events-unit), the layout laws crate (layout-unit), the mounted-layout laws crate, semio-ui-wgpu-paint-reference, `retained-control-commit` Rust test. Bun: `🧪️tests/⭕️ring-angle/🟦️.ts`, `🧪️tests/🎚️ring-press/🟦️.ts`, `retained-control-commit/🟦️.ts`.

## Expected fallout / known risks
- Existing tests asserting old ring angle, Ring/IconSelect node heights (control_height), IconSelect glyph counts, or `paint_slider_travel` arity may need updating; not yet run.
- The `ring_t_at` call sites in events pass node bounds; geometry now uses the 100px frame at bounds origin, not bounds center.
- Paint crate active-line budget (cap now 4300 in its Cargo.toml) may be tight.
- `paint_ring_orb` doc/tests assume `Theme::default()` dark; compile errors possible in tests (e.g. unused `rgba`, `Vec<(([f32;4]), ...)>` parens warning).

## Round 2 (code only, still NOT compiled or run)
- Locale (`🌐️locale/🔠️common-labels/🦀️.rs`): `UiCommonLabels` gains `import_file`, `clear`; new `UiIconSelectorLabels` (url, shortcode, math, data, emoji, text, vector), en+de, both terminologies. Text is the existing React bundle (`🌐️i18n/🧩️content-hosts` `iconSelector.mode.*`, `⚙️configuration` `common.importFile/clear`): the keys already existed in React, only the Rust twin was missing. Shared fixture `🧫️fixtures/🎴️icon-selector-labels/🔣️.json`; TS test `🧪️tests/🎴️icon-selector-labels/🟦️.ts` compares it to the real React bundles, Rust test `icon_selector_labels_match_the_shared_react_bundle...` to the Rust labels.
- IconSelect now stacks React's cells: mode select trigger (localized mode label + chevron), editor, preview, import and clear buttons (icon + label). Geometry in layout: `ICON_SELECT_*` consts (tokens only), `icon_select_rows`; node height is `ICON_SELECT_HEIGHT`. No locale named = no label text (no default language).
- Editing as React: edit buffer is the editor (inner) text, commit re-encodes under the stored icon's mode (`UiIconSelectNode::{decode_mode, encode_for_mode, commit_text}`, twin of `emitIconKindFromSelectorMode`); a locked selector (disabled or mixed) neither edits nor clears; pressing Clear stores `""`. `decode_mode` now uses the generated `IconName` catalog (no closure).
- Ring press starts only on the orb (`ring_orb_hit`, radius 7 scaled, disabled never) in `pointer_commit_action`; drag continuation is unchanged.
- Fixture `🎛️retained-control-commit` (shared by Rust router test and TS twin): ring case is now a drag from the orb; new cases `ring-ignores-a-press-off-its-orb`, `icon-select-clear-button-stores-the-empty-icon`, `icon-select-locked-ignores-clear`; icon-select typing case is `text:circle` + `-dot` = `text:circle-dot`. TS twin and the `🎚️ring-press` case-name list updated. Mode fixture gained `encode` vectors; TS test `🧪️tests/🎴️icon-selector-mode/🟦️.ts` runs the real React `classifyIconSelectorMode` on the same vectors.
- Layout tests: `ring_press_starts_only_on_the_orb`, `icon_selector_cells_stack_...`; paint law `icon_selector_paints_the_localized_mode_and_button_labels...`.

## Run list (all unverified)
Rust (framework workspace, `--manifest-path 🧰️framework/Cargo.toml -p <crate>`): semio-ui-wgpu-paint-laws, the events laws crates (events-unit, retained-control-commit), the layout laws crate, mounted-layout laws, semio-ui-locale tests, semio-ui-wgpu-paint-reference (check). Bun: `🧪️tests/⭕️ring-angle/🟦️.ts`, `🧪️tests/🎴️icon-selector-mode/🟦️.ts`, `🧪️tests/🎴️icon-selector-labels/🟦️.ts`, `🧪️tests/🎚️ring-press/🟦️.ts`, `🎛️retained-control-commit/🟦️.ts` (renderer engine tests).

## Still not done (honest gaps)
- The mode select does not open: needs the retained select popup machinery wired to a sub-region of one node and `migrateIconKindToIconSelectorMode` (not ported). Import file does nothing: no host file picker command exists for the retained target.
- IconSelect stays ONE Tab stop with one edit buffer; React has four (select, textarea, import, clear). Pressing outside the editor still begins editing the node.
- Textarea multi-line editing/rows (4 or 5 by mode) and the mono font are not mirrored; editor wraps single-buffer text.
- Slider hover uses node HOVERED plus pointer-in-track-cell only when a hover pointer is passed.
- Kit/immediate `render_*` paths untouched (dead per coordinator).
- Existing tests asserting old node heights or glyph counts for Ring/IconSelect may need updates.
