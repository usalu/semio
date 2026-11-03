# A11y-devices (wave 2): DoD 4 and 5

Status: code, unit tests and `.feature` scenarios written, NOT compiled or run (no cargo by rule). Rebuild requested in `🗑️generated/integrate/latest.md`. Crates: `semio-s-plugin-norm-app-surface-visual-chrome` (device-profiles, accessibility, themes, manifest) and `semio-s-plugin-norm-structural-scene-meshing` (colour-maps). Source gate after changes: visual-chrome 19 files, 2613 active lines; meshing 13 files, 2126 lines.

## Found, not new work
Wave 1 already had device profiles (targets 24/44 px), live region, focus order, i18n census, themes, colour maps (CIE76 + Machado), table twins, `check_accessible_name`. Gaps closed here:

## 1. Responsive, customisable layouts (DoD 5)
- Schema first: `📱️device-profiles/🧬️schema/{🦀️.rs,🛰️.proto,🟦️.ts,🔗️.graphql}`: `LayoutPrefs{dock, tablet, mobile}`, `DockPrefs` (column shares, splits, hidden windows), `TabletPrefs` (viewport share, side tab order and active), `MobilePrefs` (tab order, active tab, `SheetState`, sheet height). Persisted local-only: `STORAGE_KEY = "norm.visual.layout"`, `PERSISTENCE = "persistedLocalOnly"`; value-codec round trip tested.
- `📐️layout/🦀️.rs`: defaults (22/52/26, 50/50 splits, tablet 60/40, mobile tabs viewport/properties/checks/diagrams), `sanitised` (any stored value becomes valid: clamps, NaN, duplicates, unknown roles, viewport never hidden), `validate_layout` (path: rule lines), `LayoutCommand` + `apply_layout` (resize columns, split, hide/show, viewport share, move/activate tab, sheet state/height, reset), `bottom_sheet(device, prefs, has_selection)` (mobile only, over the model tab, collapsed peek 0.12, half = preference, full 0.92, handle >= 44 px).
- `manifest-and-media`: `visual_layout_with(variant, device, &prefs)` builds the desktop dock (hidden windows dropped, shares renormalised), the mobile single tabbed stack, the tablet two panes with tabbed side stack; `visual_layout` = defaults.
- Tests: `📱️device-profiles/🧪️tests/{🦀️.rs,🥒️.feature}` (9 new tests).

## 2. Keyboard
- `⌨️focus-order`: shortcut map now scoped (`Global`, `Viewport`, `Handle`), so the same arrows orbit in the viewport and nudge a focused handle (Shift = large x10, Alt = fine x0.1, matching `Modifiers::scale` of the handle drag tool). New: viewport selection cycling (PageDown/PageUp, `cycle_selection`), Enter = edit handles of the selection, Tab/Shift+Tab handle walk, Undo (Ctrl+Z), Redo (Ctrl+Shift+Z, Ctrl+Y), catalogue search (Ctrl+K, /), existing focus (Alt+1..5, F6), checks (`[` `]`), help; `lookup(scope, chord)`, `chord_text(chord, mac)`. All labels EN + DE.
- Tests in `♿️accessibility/🧪️tests` (uniqueness per scope, nudge steps, undo/redo/search reachable in every scope, cycling, chord printing).

## 3. Screen reader
- `♿️accessibility/🗣️screen-reader/🦀️.rs`: `AriaNode` with `attributes()` and `missing_required()`; `viewport_node` (application + summary), `scene_list_node` (listbox), `entity_node` (option: name + spoken utilisation and status, detail, posinset/setsize, selected), `handle_node` (slider: value text with unit and locale decimal, min/max from limits, key help EN/DE), `check_row_node` (row: title, subject, status and utilisation spoken, clause), `diagram_nodes` (img described by the table twin + table node with row/column counts).
- Contract oracle: WAI-ARIA 1.2 required attributes (`slider` needs `aria-valuenow`, `option` needs `aria-selected`) and role list checked in tests (hand-copied from the spec; no a11y library available offline).
- NOT wired yet into the panels renderers (see requests).

## 4. CVD-safe palettes and high contrast
- `colour_maps::metrics::{delta_e2000_lab, delta_e2000, min_pairwise_delta_e2000}` (CIEDE2000). Third-party validation: the published Sharma/Wu/Dalal test pairs (six pairs, 4 decimals) and the independent Python implementation in `🔮️oracles/🐍️.py` (also fixed: `main()` was unguarded, so importing it hung; now `if __name__ == "__main__"`). Python values were checked against the same Sharma data.
- Measured with the Python twin (not Rust): status colours pass(0.3)/warning(0.95)/fail/not-applicable, minimum pairwise CIEDE2000 under protan/deutan/tritan: light 19.8, dark 6.0 (pass vs n/a under protanopia), high contrast 6.9; pass vs fail >= 21.1, warning vs fail >= 17.2; ramp ends >= 34.2, safe end vs failure start >= 18.8. Thresholds in tests: all pairs >= 5, pass/fail >= 20, warning/fail >= 16, ramp ends >= 15, break at 1 >= 10. Dark/high-contrast pass vs n/a is the weakest pair (colour alone insufficient there; glyph and hatch carry the difference, already tested).
- `themes::effective_theme(prefs, chosen)` (forced colours / more contrast select the built-in high-contrast theme unless the chosen bundle is itself high contrast) and `motion_allowed`. Built-in themes tested for ramp end distinguishability and zero contrast findings.
- Not done: extra user-selectable ramps (viridis/cividis/greyscale ids in `RampId`); the viewport config still lists `ColourRamp::{Viridis,Inferno,Coolwarm,Grayscale}` separately from the norm ramps (inconsistency to unify by the colour owner).

## 5. Touch
- `👆️gestures/🦀️.rs`: `Gesture` -> `Intent` per `DeviceClass`: tap = Pick/Replace, long-press = Pick/Additive, long-press drag = Rectangle/Additive (all checked against the methods and merges of the generated `viewport_interaction_definition`), double tap = frame, one finger orbit, two pan, pinch zoom (clamped 0.25..4), handle drag only on desktop and tablet (`FreeDrag`), mobile focuses the handle and uses +/- nudge buttons (a drag starting on a handle orbits, never edits), pencil hover = hover on tablet only. Hit area >= 44 px on mobile and tablet is `DeviceProfileSpec::handle_hit_px` (tested), sheet handle 44 px.

## Fixes of wave-1 test paths (crates moved one level deeper in the split)
`CARGO_MANIFEST_DIR` joins in: colour-maps oracle (`../../../🎨️colour-maps`), themes schema dir, diagrams2d tests dir (`../../../🪟️visual-windows`), i18n census root (6 levels up). They would have failed on first run.

## Open
1. Artifact `layout()` in all ten editors and viewers still hard-code the desktop tree (22/50/28) and know no device; see `requests.md` (a11y -> artifacts). Needs a host hook to re-request the layout on resize or preference change.
2. Wire `screen_reader` nodes, scoped shortcuts, `bottom_sheet` into panels renderers (panels crate, next shared pass); a UI to edit `LayoutPrefs` (dock resize, tab reorder) is not built, only the command reducer.
3. Unverified until the first build: all Rust here (derive on the layout schema, closure borrow shapes in `visual_layout_with` and `nudges`), the Python oracle JSON shape in Rust tests, i18n census over the new files.
4. Item "undo/redo" and "catalogue search" are key chords only; the command handlers belong to the live editor.

## Follow-up: wiring (coordinator request)
- Layouts: all ten editors and viewers delegate `layout()` to `visual_layout_with` / `view_layout_with` (new `layout_for(device, &LayoutPrefs)`); the builders declare named layouts `norm-visual-{desktop,mobile,tablet}` (`edit_named_layout`, `view_named_layout`). `view_layout_with` keeps model, checks, diagrams only.
- Host hook: schema-first `DeviceSignal{widthPx, touch}` (Rust, proto, ts, graphql) and `device_view(signal, prefs, has_selection)` -> class, layout id, profile, sheet. The plugin API has no resize or device hook (layout switching lives in the wgpu shell: `layout_override`/`resolve_layout_for_mode`), so the shell call is an open request; the seam it needs (named layout ids) is declared.
- Keyboard: `⌨️shortcut-dispatch` in the panels crate maps scoped chords to handle keys (nudge, handle walk) or host actions; test cross-checks all 12 nudge chords against `key_intent`/`Modifiers::scale`. Existing `gesture_step` already runs keys through the drag statechart. Host key handler still to call `dispatch_shortcut`.
- Found by the e2e test design: tablet has no outliner window, so `focus_order(Tablet)` was wrong (listed five stops); now the four stops of the tablet panes.
- E2E: `en1993 ✏️editor/🧪️tests/🔬️unit/🦀️.rs` `the_editor_adapts_pane_tree_focus_order_and_accessible_names_to_every_device_class`: per device class the manifest's named layout tree, focus order = windows of the tree, EN/DE check names from the evaluated default document.
- NOT wired: mobile bottom-sheet UI (no sheet node used yet), viewport entity names via `entity_node` (lowering labels/golden fixtures unchanged), `check_accessible_name` still has its own format (richer: quantities and clause) next to `check_row_node`; properties/outliner/diagrams already carry labels and the table twin. Nothing compiled.
