# Astra Sol Command Palette

## Outcome

The WGPU shell now implements the Search and Find palette as a bounded, resumable chrome plan shared by paint, hit publication, keyboard/pointer routing, accessibility projection, and dialog census. The old Search/Find path only opened a blank `400 × 55%` glass rectangle. The replacement paints the authored React `CommandDialog` structure: centered dialog-level glass and veil, `sm:max-w-lg` width, large input row, command-list height cap, grouped headings, result rows with optional descriptions, selection/hover state, and localized empty state.

The command producer now matches `ShellHost` declaration semantics and order:

1. panel leaves;
2. window kinds;
3. `inPalette` resolved commands;
4. only in host mode with a live panel: spawn rows, `studio.undo`, `studio.redo`, `studio.home`.

A session is required, app keybindings are not copied into a second palette catalogue, `commitCheckpoint` and `studio.goHome` are absent, and every argument-bearing command is one `…` row opening its staged command form. Command row IDs use React's dotted `commandAddressKey(...).replaceAll(":", ".")` projection. Staged activation opens the command's actual category leaf before setting the expanded command key.

## Runtime contract

- Exact input IDs: `ui.search.input`, `ui.find.input`.
- Exact dialog census IDs: `ui.search.dialog`, `ui.find.dialog` at `dialog` level.
- Search/Find query state is exposed as the accessibility textbox value and keeps the real accessibility mirror input focused.
- Search and Find share one typed plan with bounded capacity and the existing 20-result fuzzy ranker.
- Keyboard query, navigation, Enter, Escape, pointer hover, physical row click, and accessibility Value/Activate events converge on the existing `activate_search_item` / `activate_find_item` funnels.
- Closing or activating clears browser/renderer focus and the next complete chrome publication retires the dialog, input, and result hits.
- Search activation of an argument-bearing command opens `bottom-middle → command.category.<category>` and publishes its staged form.
- The render uses `Level::Dialog` glass plus the dialog veil. Geometry derives from theme tokens: 512 px desktop maximum, seven-spacing viewport inset, nine-spacing input height, three-spacing input padding, one-spacing input gap, five-spacing icon, 93.75-spacing list cap, group/heading single spacing, item `py-tiny`, and empty `py-medium`.

## Schema and independent React oracle

Added:

- `🧱️elements/🔎️ShellSearch/🧬️schema/🔣️.json`
- `🧱️elements/🔎️ShellSearch/🧫️fixtures/🔣️.json`
- `🧱️elements/🔎️ShellSearch/🧪️tests/🧩️component/🟦️.tsx`

The neutral fixture pins control identities, English/German copy, authored geometry tokens, producer family order, forbidden obsolete rows, the dotted canonical Set Theme ID, its staged action key, and deterministic query outcomes. The independent oracle renders the actual React `UISearch`, `UIFind`, and repository-owned `CommandDialog`, validates the JSON schema with Ajv, pins authored class/token authority, checks focus/filter/group/order/empty behavior, and activates the exact staged row.

The WGPU native laws consume the same neutral fixture and cover:

- real normal-chrome publication from `Overlay` through `PersistPreferences`;
- dialog census, 512 px input hit, textbox value/focus, filtered deterministic Set Theme row, Enter activation, Appearance category form publication, and retirement;
- Find textbox/value/filter publication and physical row-click activation;
- exact producer family order, host suffix, forbidden rows, dotted command ID and staged action;
- empty producer without a session.

The parity probe's former open-then-Escape palette step is now a full journey assertion. It opens by chord, waits for an actual focused DOM/accessibility-mirror input, derives the localized Set Theme label from the visible input language, types the query, verifies the exact React canonical row (and WGPU's corresponding accessible/physical row), clicks the row, waits for dialog/input/row retirement, and requires the `command.category.appearance` panel, its staged form section, and the actual Execute/Reset actions. This supports the light/English focused run and the dark/German full journey without using the earlier unequal screenshot as a color or text oracle.

## Executed validation

Passed:

```text
bun nx run @semio-tech/ui-react:test-quick -- --run '../../../../../../🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔎️ShellSearch/🧪️tests/🧩️component/🟦️.tsx'
Test Files 1 passed; Tests 3 passed
```

Passed parser/fixture checks:

```text
rustfmt --edition 2021 --emit stdout <Shell WGPU source> >/dev/null
rustfmt --edition 2021 --emit stdout <palette native test> >/dev/null
rustfmt --edition 2021 --emit stdout <command registry native test> >/dev/null
bun -e 'JSON.parse(...)'  # schema and fixture
node --check 🐍️parity-interact-probe.mjs
```

No Cargo test, WGPU browser activation, or parity journey was run in this packet because the root task owns those serialized heavy gates.

## Root runtime gate

Source-coherent marker: **PALETTE-SOURCE-COHERENT**.

Exact native test filter:

```text
shell_shortcuts_palette_tests
```

The focused acceptance names inside that filter are:

```text
shell_shortcuts_palette_tests::the_command_palette_publishes_filters_and_activates_through_normal_chrome
shell_shortcuts_palette_tests::find_publishes_filters_and_activates_with_a_physical_row_click
shell_shortcuts_palette_tests::the_palette_rows_follow_the_react_declaration_order
shell_shortcuts_palette_tests::a_palette_without_a_session_is_empty_like_react
```

Remaining acceptance is a fresh root-owned native test run and controlled WGPU/React browser journey after the shared Shell source reaches the next compiler-coherent checkpoint. Browser acceptance must be based on that fresh activation.

## Root Browser Oracle Correction

The first React-only runtime replay established that the Form disclosure starts closed, and that Set Theme uses the authored argument id themeId. Root corrected the probe to require the selected Appearance panel plus command.category.appearance.form and command-os.os.setThemeId-execute/-reset. Focused query, canonical result, physical activation and dialog/input retirement remain required. The old arg.value assertion was a probe defect.
