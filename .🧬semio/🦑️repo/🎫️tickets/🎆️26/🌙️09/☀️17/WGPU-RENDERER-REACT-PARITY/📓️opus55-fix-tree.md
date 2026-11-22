# WGPU Tree, TreeSection and TreeItem parity fix

Scope: audit `📓️opus55-audit-structure-overlays.md` section 1, items 3, 7, 8, 14 and 20 (keyboard, header, guides, direction, roving) plus the tree part of 10 (modifier selection, double click) and the tree accessibility projection. Production path only: `semio-ui-wgpu-paint` retained tree painting, `⚡️events`, `📌️mounted_layout`, `🧱️layout-stack`. The immediate widget path was not touched.

## Audit verification (Tree sections)

| Audit claim | Verified |
|---|---|
| No tree key handler in wgpu | Confirmed: only Enter/Space on disclosure heads and the a11y mirror. Item rows were not focusable at all. |
| No roving tabindex | Confirmed. Rows of a document tree were not tab stops; only branch rows (via `disclosure_is_interactive`) were. |
| Section chevron missing | Confirmed: the header painted a folder icon and nothing else. |
| Section label not uppercase / semibold | Confirmed (also painted `text_muted` when closed; React is always `text-element`). |
| Indent guides absent on the document path | Confirmed. `tree_draw_guides` no longer exists anywhere; `paint_tree_guides` survives only in the reference painter. |
| Modifier selection, double click missing | Confirmed: a press toggled the disclosure and fired the row action, with no modifiers and no double-click rule. |
| Tree mirrors in RTL while React forces LTR | Confirmed (`retained_tree_row_metrics` and the painter both used the window flow). |
| "Typeahead: not in React tree either" | Confirmed. It is implemented anyway as asked, as a WAI-ARIA extra. |
| Item indent "tokens match" | Tokens match, but React indents a section's rows by one level (`TreeContext.level + 1` under the section); wgpu rows start at the section's own x. NOT changed (hit rects, layout fixtures and chevron geometry depend on it). Residual, see below. |

Follow-up (code-only, uncompiled): rows now sit one indent level under their section header (painter indent x3, `tree_item_chevron_rect`, guide columns now k*INDENT+INSET with a section stem from the header and elbows for top-level rows; no numeric layout or hit fixture asserted the old x, the chevron hit law only checks width), the header label is tracked by the new `Theme::tracking_wide` (0.025 em, `paint_retained_glyph_step_tracked`), and React's `TreeItem` now renders `aria-level={level}` on its three `treeitem` shells (it was missing; no React test was added or run).

A finding the audit did not make: React's `TreeItem` never renders `aria-level` (only the tests set it), while `treeNavigateV1`/`rowOf` read `aria-level ?? 1`. In the real React DOM every row is level 1, so Left never climbs to the parent. The wgpu twin uses real levels (the WAI-ARIA behaviour). React was not changed.

## What changed

Pure rule, shared vectors:
- `🧱️elements/🌳️Tree/⌨️roving/🦀️.rs` (new, Rust twin of `⌨️roving/🟦️.ts` and `🎚️selection/🟦️.ts`): `tree_key`, `tree_pointer`, `tree_typeahead`, `tree_roving_stop`, `tree_selection`. Included by the light tier (`🪶️light/🦀️.rs` as `tree_roving`).
- Fixture `🌳️Tree/🧪️tests/🧫️fixtures/⌨️roving/🔣️.json`: tree shape, key and click sequences with the expected focus, selection, folded state, activations. Replayed by the Rust test `🌳️Tree/🧪️tests/⌨️roving/🦀️.rs`, by the engine test (single-select key cases through the real router) and by a TypeScript test appended to `🧪️tests/⌨️roving-tabindex.tsx` (the plain-key subset React answers).

Keyboard and focus (`⚡️events`):
- New module `⚡️events/🌳️tree-roving-and-keys/🦀️.rs`: visible rows, ordered rows, roving stop, `route_tree_key`, `route_tree_press`, `interactionSelect` pick.
- `🧭️event-model-and-focus`: item rows are focusable and hit targets; the tab order holds ONE row per tree (remembered focus, else first selected, else first); `FocusState` remembers the roving row, the range anchor and the last press.
- Keys: Up, Down, Home, End; Right unfolds or enters; Left folds or climbs; Enter and Space click the row (fold plus activation or pick); with an interaction domain Shift+move extends the range, Shift+Space ranges, Ctrl/Cmd+Space toggles; printable keys are typeahead (reuses the select typeahead buffer and its 700 ms reset). Left and Right are not mirrored.
- Pointer: a press clicks with the merge its modifiers name (Shift range, Ctrl/Cmd invert, bare replace). A second press on the same row within 0.4 s clicks nothing (React `event.detail > 1`).
- Emitted actions: a row with an activation fires it (React `activateBinding ? dispatchTrigger : pickClick`); a row of a domain-bound tree sends `interactionSelect` with `{merge, method: "pick", targets}` in React's `treePickIntentInputV1` shape (range = replace with the range ids, invert = just the row). The scope, version and authored args are borrowed from the row's `interactionHover` binding (the only domain binding a wgpu row carries).
- Shell: `retained_node_is_tree_row` so a focused tree row receives printable characters and Space as `KeyDown` (like a focused Select) instead of `TextInput`.

Painting (`🖌️paint/🌲️retained-tree-node-painting`):
- Section header: fold chevron in the gutter at React's `size-small` (only when the section has rows, chevron-down open, chevron-right closed, mirrored for Up flow), label uppercase and semibold, always `text-element`.
- Indent guides in a free paint phase: per row the ancestor stems, the parent stem and elbow, and the branch stem of an open row, in `text_muted` at 40 percent (`border_emphasized` when selected or hovered).
- Focus ring on the keyboard-focused row.
- LTR is forced for every non-table Tree in `retained_tree_row_metrics` (layout, hit and events) and in the painter.

Accessibility projection:
- `AccessibilityProjectionNode.multiselectable` (Rust contract, TS contract, DOM mirror `aria-multiselectable`).
- The wgpu walk stamps `aria-level` as nesting among tree rows (a section is no level) and leaves one tabbable row per tree.

Also: `Cargo.lock` gained the edges `semio-ui-wgpu-events` to `semio-ui-wgpu-light` and `semio-ui-wgpu-engine-scene-laws` to `semio-ui-action` (hand edited, no cargo command touched the lock).

## Tests

Run from `/Users/ueli/Documents/semio` as `repo cargo --store=shared --configuration=prototype -- test --manifest-path 🧰️framework/Cargo.toml -p <crate> [filter]`.

Verified: `semio-ui-wgpu-light` filter `tree_roving`: 3 passed (shared JSON vectors, roving stop, React-parity keys).

NOT compiled or run (build policy stopped my runs; first attempt of paint hit the source boundary 4215/4000 while the cap was being raised to 4300, now 4228 lines):
1. `semio-ui-wgpu-events` check (new module `⚡️events/🌳️tree-roving-and-keys`, new dep on light, Cargo.lock edited by hand).
2. `semio-ui-wgpu-engine-scene-laws` (new tests in `🧪️tests/🌳️tree-keyboard-and-header`: key vectors through the router, one tab stop with memory, no RTL mirroring, double-click, `interactionSelect` picks with Shift/Ctrl, header chevron and uppercase semibold label draw-list assertions, guide elbows and stems). Also run the existing `retained_section_collapse_tests`, esp. `a_nested_tree_item_opens_from_its_row_and_keeps_its_action`, which exercises the changed press path. New dev-dep `semio-ui-action` was added.
3. `semio-ui-wgpu-hosting` filter `a_tree_projects_multiselectable` (a11y projection: multiselectable, levels, single tab stop), plus the whole a11y projection module (document fixture rows must keep `tabbable`).
4. `semio-ui-wgpu-events-laws` and the paint/mounted-layout laws, because item rows are now focusable and hit targets, the tab order has one stop per tree, and `retained_tree_row_metrics` forces LTR for non-table trees (fixtures with RTL trees may need updating).
5. Shell keyboard crate containing `⌨️keyboard/🦀️.rs` (new `retained_node_is_tree_row` re-export chain in the Interpreter wgpu crate).
6. `semio-ui-contract` accessibility tests (new `multiselectable` field) and the TS `⌨️roving-tabindex.tsx` vitest (new fixture replay) plus the TS contract projection test.
Likely compile-fix spots: `tree_selection` signature use, `FocusState` new fields, `UiFlow` literal in the RTL test, `IconAtlas::from_packed` entries, `KIND_SOLID` re-export.

Extra run items for the follow-up: `semio-ui-wgpu-layout`/mounted-layout laws (chevron rect), `semio-ui-theme` laws (new field `tracking_wide`, the theme-engine unit test table), the React `Tree` vitest suites (`aria-level` on rows; check snapshots), and re-check the guide test `a_row_paints_the_stem_and_elbow_guides_that_hang_it_from_its_parent_or_section` (expects 2 leaf elbows, 1 branch elbow, 1 for a top-level leaf).

Scrollbars: the overlays work (`paint_retained_scrollbar`, `press_scrollbar`, `scrollbar_for`) acts on any `SCROLLABLE` node and runs in the frame's Scrollbars phase. A Tree does not scroll itself (it sits in a Scroll layout and windows its rows with spacers), so its scroll host already gets the thumb; no tree-specific code was needed and none was added.


## Residual and honest limits

- Not implemented: Ctrl+A and `*` expand siblings, a roving stop remembered by the a11y projection (the projection uses focused, else first selected, else first; the router remembers the last focused row).
- `multiselectable` and multi-select merges are inferred from `interaction_domain`; the document has no `selectionMode` prop.
- The double-click emits nothing for document trees, matching React's interpreter, which wires no `onDoubleClick` for tree rows.
- Item label size (`text-xs` against `font_size_body`) and the selected/hover fills were not re-derived.
