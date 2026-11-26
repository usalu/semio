# ✂️ Editing features, end to end (en and de)

Journey: `journey-editing.ts` (`JOURNEY=editing bun journey-editing.ts en de`, one step per feature, rows addressed by `data-row-id` so the German kind names do not matter). Baseline run before any fix: `🗑️generated/edit/baseline-en.txt`.

## Feature, step, result, fix

| Feature | Step | Baseline | Root cause | Fix |
| --- | --- | --- | --- | --- |
| Row click select, Escape clear | selection: row click | selected (Preview) but the row was never painted | `sourcing_table` set no `selection_json`; the Table host reads `selectedIds` from it | `sourcing_table(..., selected_ids, ...)` writes `{"selectedIds":[...]}`; Pool and Curated render against `interaction.selection("rows")` (`render_body` in `🧩️sourcing-app`). Rows paint `data-active`. |
| Set Selection Mode… Multiple | selection: Set Selection Mode… | refused: `domain rows does not declare mode multiple` | `SelectionSpec` declared only Single, Replace, Pick | `📋️manifest`: modes Single+Multiple, methods Pick+Rectangle+Lasso, merges Replace, Additive, Subtractive, Invertive, Range |
| Select All, range merge | selection: Select All, shift range | no effect | the `rows` domain was `HierarchyProvider::Flat`, so the framework never asked the app for a topology | `HierarchyProvider::Topology` + `interaction_topology` (`🕹️selection`, stock in document order) |
| Shift range / ctrl invert / alt subtract on tables | selection: shift range | every click replaced | Table host hard-coded `merge: "replace"` | Table host uses `interactionMergeFromModifiers(event)` (shift range, ctrl/cmd invertive, alt subtractive) |
| Grid pick, marquee, additive, subtractive | selection: Grid marquee | console fault `dropped action "worldPick" ... no window kind declares it` | the grid scene was not bound to the `rows` domain, so the world host used the plain `worldPick` action | grid scene sets `domain_id`/`domain_granularity_id`, every instance carries `interactionId` = kind id (copies, parts, count glyphs), selected kinds draw highlighted, hovered id passed to the scene |
| Hover | grid hover | no fault | table row hover is the browser's own highlight; hover wiring on table rows made a fast hover-then-click lose the click (re-render between mouse down and up), so it was tried and removed | grid hover is domain bound; table rows do not broadcast hover |
| Copy, Cut, Paste… (7 anchors) | clipboard | no-op (default `ArtifactEditor` clipboard) | sourcing declared no clipboard media or hooks | `📋️clipboard`: fragment = `(objectId,count)` pairs; Copy = selected kinds with curated counts (1 for an uncurated kind); Cut = delete the selected curated kinds; Paste adds counts bounded by availability, anchor and position are ignored (the curation is a list). Wired through `clipboard_media_type/copy_fragment/cut_operations/paste_operations`. |
| Typed count entry (Pool, Curated) | counts: typed entry | not possible: stepper input was `readOnly` | Table `TableStepperCell` only knew keys and buttons | editable spinbutton: Enter or blur commits `{ value }`, Escape cancels, Home/End stay on the caret while typing; `tableStepperTypedValue` refuses negative, fractional, off-step and non-numeric text and clamps to the maximum; zero removes the kind |
| Negative `value` of Curation Set Count | Curation Set Count… | a negative value was read as 0 and removed the kind | `value.max(0.0)` | negative or non-finite `value` is a no-op |
| Row More (⋯) menu | row More menu | opened nothing | row context menu = plugin `context_menu` items + row buttons placed in the menu; sourcing supplied neither | every Pool and Curated row carries a hidden `menu` buttons cell: Curate (`curationAdd`, while stock lasts), Curate One Less (`curationSetCount delta -1`), Remove (`curationRemove`); labels en+de (`curate_less`) |
| Decrease buttons, Curate row action | counts | pass | | none |
| Stock From Catalogue, Restock button | Stock step | no visible effect on the demo: all module kinds are already in the stock | intended effect is to merge missing module kinds into the stock, keeping curated counts (Rust law `the_restock_button_publishes_a_merged_catalogue_document`) | verified end to end through the new "No example" example (empty stock): Restock and the action bring the ten module kinds back |
| setContributions | contributions step | pass | host-pushed, not a user action; observable as the Beams, Windows, Slabs, Reuse module toggles | none |
| Set Grid Instance Display… | grid display step | n/a | | three options must produce three distinct canvases |
| Note Shell Command, Record Tutorial | palette step | not exposed in any window's Actions menu or the Command menu | shell-only reserved verbs | asserted absent |
| Cancel of a running operation | not exposed | no operation in the editor runs long enough to show a cancel control (Export shows only a toast); `cancelTypedOperation` is a window action with no UI | recorded as not reachable |
| Lasso | not reachable | the world host offers rectangle only in this layout; lasso is declared in the spec | recorded |

## Files

Fixes: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📊️Table/🟦️.tsx` and its package entry, `…/🎯️targets/⚛️react/🌍️scene-and-world-hosts/🟦️.ts`, `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/⌨️table-stepper-keyboard/🔣️.json` (typed section, language-agnostic), `…/🧪️tests/🧮️table-host/🟦️.ts`; sourcing `✏️editor/{📋️clipboard,🕹️selection,🧩️sourcing-app,📋️manifest,🧱️editor-vocabulary,🗣️terminology,🎮️commands/🔢️curation-set-count,🎭️modes/✏️edit/🪟️windows/{🏊️pool,🧺️curated,🔢️grid}}`, domain `🧬️schema/🧾️domain` (`interactionId` on instances), operations mirror (re-export of the peer's `catalog_*` helpers).
Tests: `✏️editor/📋️clipboard/🧪️tests/🔬️unit/🦀️.rs` (clipboard, topology, row menu, painted selection, negative count), mounted in `🧪️tests/🧪️unit-all`; table-host vitest cases.

## Status

Last run (`🗑️generated/edit/run6.txt`): en 15/20, de 15/20 (30/40). The `check --lib` of the curation crate compiled all Rust above; the hub wasm that is deployed on 6081 contains the clipboard, row menu, painted selection, grid domain binding and set-count work but NOT the later edits (`HierarchyProvider::Topology`, grid hovered id), because the final `semio-hub-sourcing-component` unit was refused by the memory gate ("needs more memory than this machine carries ... Content store answered 507") for 2 hours and the attempt was cancelled (lock released).

Still failing, and why:
- Select All, Multiple mode, shift/ctrl/alt range merges, Grid marquee (4 steps per language): need the Topology rebuild (`ONLY=semio-hub-sourcing-component ONLY_HUB=1 sh native-wasm-build.sh`, then stage and restart; `editing-redeploy.sh` does the whole serialized sequence). Unverified until then.
- Stock From Catalogue step: Restock/action behaviour is covered, but selecting "No example" after a document-replacing load (Restock) takes about 30 s and, in the full journey on a loaded machine, exceeded 150 s: a slow whole-document reload, open defect, not diagnosed.
- Rust unit laws (`editing_tests` in `curation_unit`) were written but could not run: the test profile build failed in an unrelated peer change (`LazySlots::take_next_occupied_slot` missing in `semio-plugin-reactor-turn`). The table-host vitest cases could not run either (the repo runner routes that file to bun without a DOM).
- Unreachable from the UI: lasso, cancel of a running operation, Note Shell Command, Record Tutorial.
