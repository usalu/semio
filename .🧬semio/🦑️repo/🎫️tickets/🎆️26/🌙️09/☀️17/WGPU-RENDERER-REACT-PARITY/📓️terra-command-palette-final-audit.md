# Terra Command Palette Final Audit

## Scope

Read-only source audit of Search and Find against the repository-owned React implementation. No test, build, browser activation, or Git operation was run. The prior owner notes accurately describe producer order, dotted command IDs, the 20-result ranker, and the staged-command flow; the probe's exact staged-field assertion is invalid. The gaps below are in the interactive and accessibility boundary, plus the journey that currently accepts them.

## Confirmed parity

- Search production follows React's panel leaves, window kinds, resolved `inPalette` commands, then the host-only spawn/undo/redo/home suffix in `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14614-14687`; the React oracle is `🧱️elements/🏛️ShellHost/🟦️.tsx:10474-10561`.
- The WGPU command IDs use `command_address_stable_key(...).replace(':', ".")`, and argument-bearing commands use one ellipsis row that opens `command-form:<stable-key>` in `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:19513-19558`. The activation path resolves the command category and sets the expanded key at `:14888-14906`, matching React's panel-path and `SET_COMMAND_EXPANDED` consequence.
- Search and Find use the React field weights, threshold, deterministic declaration-order tie-break, and 20-result limit. The ranker calls are at `:14690-14720`; the React oracle is `🧱️elements/🔎️ShellSearch/🟦️.tsx:41-62, 168-190`.

## Actionable findings

### P0 — The browser accessibility input suppresses all palette keyboard actions

WGPU publishes the palette input from a generic `HitKind::Input`, so its projected role is `textbox` (`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:28912-28955`). The mirror creates a native text `<input>` for that role (`🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts:47-59`). Its browser key bridge explicitly returns before admission for every non-modifier key on a non-combobox input (`🎯️targets/🧊️wgpu/🎮️input-wire/🟦️.ts:58-69`). Consequently, while the focused WGPU palette input can emit a value change, `Escape`, arrow navigation, and `Enter` never reach the native handlers that implement them (`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15142-15184` and `:15208-15220`).

React makes this input a `combobox` with list/active-descendant semantics (`🖱️ui/🧱️elements/⌨️Command/🟦️.tsx:278-312`) and its command root owns the corresponding navigation and Enter activation. This is a browser-visible keyboard parity failure, not a native-law concern. Publish the palette input as a combobox and provide its list/active-descendant relationship, or adjust the mirror bridge so the palette's focused textbox forwards these keys without allowing ordinary inputs to do so.

Confidence: high. The event path is complete in source; it does not rely on a timing assumption.

### P1 — Pressing a group heading, empty state, or list padding closes WGPU's modal

The WGPU plan emits `Group` and `Empty` entries (`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14766-14818`), but the painter registers hit targets only for the input and result rows (`:24443-24486`, `:24510-24575`). `dismiss_overlays` closes any open palette when the pointer is not on one of its limited hit kinds (`:14325-14340`). Therefore a press on a painted heading, empty message, or the deliberate spacing after a row has no overlay hit and dismisses the palette.

React's `DialogContent` treats every descendant of its dialog boundary as inside (`🖱️ui/🧱️elements/💬️Dialog/🟦️.tsx:458-482`); `CommandGroup` and `CommandEmpty` are descendants, so the same press is inert and keeps the dialog open. Register a dialog-content/background hit for the full palette rect that absorbs presses, while preserving row and input hits above it.

Confidence: high. The WGPU plan paints these regions and the dismissal predicate contains no dialog-rect fallback.

### P1 — React's close affordance and modal/list structure are absent from WGPU accessibility and paint

`UISearch` and `UIFind` call `CommandDialog` without disabling its default close button (`🧱️elements/🔎️ShellSearch/🟦️.tsx:73-75, 203-205`). `DialogContent` defaults `showCloseButton` to true and renders an accessible Close control (`🖱️ui/🧱️elements/💬️Dialog/🟦️.tsx:421-429, 530-579`). WGPU's command-palette plan has only input, group, row, and empty entries (`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:1125-1137, 14795-14818`); no close target is painted or projected.

More broadly, WGPU accessibility is a flat list generated only from physical hits at depth zero (`:28912-28956`). It therefore lacks React's dialog, listbox, group heading, and live empty-status hierarchy (`🖱️ui/🧱️elements/💬️Dialog/🟦️.tsx:530-545`; `🖱️ui/🧱️elements/⌨️Command/🟦️.tsx:317-350`). The mirror cannot invent the relationships because its projection record contains no list ownership or active-descendant fields (`🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts:6-29`). Add explicit non-hit palette projection nodes and an accessible close control, with the input/option tree nested beneath the dialog.

Confidence: high. The React close button is enabled by default and the WGPU plan/projection enumerations are exhaustive at these call sites.

### P2 — WGPU activates a result on pointer-down; React activates only on click

The WGPU pointer-down handler calls `handle_shell_hit` immediately (`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12365-12458`), and a palette row directly calls `activate_search_item` or `activate_find_item` (`:13276-13287`). React's item handler records pointer-down only to determine suppression; activation happens in `onClick` (`🖱️ui/🧱️elements/⌨️Command/🟦️.tsx:409-424`). A press on a WGPU row followed by a drag or release outside activates and closes it, while React does not emit the click.

Defer palette-row activation until an uncancelled primary-button release over the same row, retaining pointer move solely for selection/hover.

Confidence: high. Both implementations expose their complete pointer event decision points.

### P2 — Reopening after Escape retains React's query but clears WGPU's

`UISearch`/`UIFind` own their query state outside `CommandDialog` and clear it only in `handleSelect` (`🧱️elements/🔎️ShellSearch/🟦️.tsx:41, 64-70, 192-198`). `ShellHost` remains mounted with both components regardless of `open` (`🧱️elements/🏛️ShellHost/🟦️.tsx:11402-11403`), so Dialog dismissal preserves an unactivated query. In contrast, the WGPU search and find shortcut arms clear the query before either opening or closing (`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15746-15768`).

Keep the query on unactivated dismissal and clear it only after a selected item, matching React. This also preserves the filtered result/selection context when the user briefly closes and reopens the palette.

Confidence: high. This follows the owned React state lifetime and the explicit WGPU clears.

### P2 — Locale-sensitive fuzzy ranking remains intentionally non-equivalent

React normalizes every query and candidate with Unicode NFKD before removing marks (`🖱️ui/🔨️modules/🔎️fuzzy-ranking/🟦️.ts:25-29, 96-123`). WGPU lowercases and strips marks already present but deliberately cannot decompose precomposed characters (`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:20115-20130`). German labels are in the producer surface (for example `Rückgängig` in `🧱️elements/🛠️ShellHelpers/🟦️.tsx:2757`), so accented queries can receive different exact/prefix/substring scores and ordering.

The current probe's English and German Set Theme text contains no combining or precomposed accent and cannot expose this. Add a localized rank fixture that includes at least one precomposed mark and its unaccented/decomposed query, then implement the same normalization contract without introducing a runtime dependency.

Confidence: high for the algorithm divergence; medium for the exact end-user ordering impact because it depends on the live candidate set.

## `exerciseCommandPalette` false-positive coverage

### P1 — The staged-form assertion names no React form item and produces a false failure

After selecting Set Theme, the journey requires `command.os.os.setThemeId.arg.value` (`🐍️parity-interact-probe.mjs:850-865`). React instead derives one `TreeDataItem` per definition argument with the identifier `command.${expandedElementKey}.arg.${def.id}` (`🧱️elements/🛠️ShellHelpers/🟦️.tsx:4821-4827`). For Set Theme, `def.id` is `themeId`, so the model identity is `command.os.os.setThemeId.arg.themeId`; the engine contract directly asserts that identity and the corresponding execute/reset action IDs (`🧪️tests/🔬️engine-contract/🟦️.ts:9800-9805`). There is no source basis for the `.arg.value` identifier.

The supplied React run already demonstrates the valid observable consequence: `command.category.appearance.form`, `command-os.os.setThemeId-execute`, and `command-os.os.setThemeId-reset` are present in its recorded controls (`🗑️generated/astra-runtime/palette-react-15/steps.json:10372,10453,10463`). Thus this failure does not establish a React/WGPU staged-form difference. Change the journey to require the Appearance panel, form section, and execute/reset action controls. Only add a browser-level assertion for the staged field after establishing its actual rendered control ID; its `TreeDataItem` identity is not automatically the DOM control ID.

Confidence: high. The React builder, contract, probe predicate, and real React control inventory all agree.

The current journey validates opening, focused input, the localized Set Theme label, pointer activation, and dialog retirement. Once its invalid staged-field predicate is corrected, it will validate the staged-form consequence through the observed section and action controls. It does not validate the most important keyboard paths:

- It types via the focused DOM input and activates through `driver.click`, never Arrow/Enter/Escape (`🐍️parity-interact-probe.mjs:800-855`). Thus it can pass while the P0 keyboard path is unreachable.
- It permits any WGPU row whose accessible label equals the target. The canonical command-ID assertion is React-only (`:835-846`), and WGPU mirror rows expose the ordinal hit key (`ui.search.item.<n>`) rather than `data-command-item-id` (`🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts:68-72`). It also does not require the filtered row set to contain exactly that one result. A duplicate label, incorrect WGPU command identity, or additional false-positive match can therefore pass the journey.

Extend the journey to press ArrowDown/ArrowUp, Enter, and Escape against the focused WGPU accessibility control; assert the dialog remains open for heading/empty clicks; assert close-button availability; and expose a stable command ID in the WGPU projection so both renderers can require the same selected item and exact filtered result set.

## Validation status

Static inspection only, as requested. The root-owned native and browser gates remain necessary after these findings are addressed.
