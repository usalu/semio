# 🪟️ Sourcing Surface, Viewer, Navigation, I18N and A11Y Features

Journey: `journey-surfaces.ts` (one step per feature, run for `en` and `de` against the native host on 6081).
Run: `JOURNEY=surfaces $BUN journey-surfaces.ts en de` (verdicts and screenshots under `🗑️generated/journey/surfaces*`).
`STEPS="<regexp>"` limits a run to matching steps (debug aid).

Columns: feature, journey step (title), result of the first run on the unmodified host, fix.

| # | Feature (inventory rows) | Step | First result | Fix |
|---|---|---|---|---|
| 1 | Boot, windows present (1, 5, 10 to 13) | boot without program/descriptor faults | pass | none |
| 2 | Breadcrumb `semio · sourcing · curation`, role chip, role buttons with chords (4) | breadcrumb, role chip and role buttons | pass | none |
| 3 | Accessible names of every button, tab, combobox, spinbutton, textbox, header (148 to 151) | every control of the editor surface has an accessible name (CDP accessibility tree); repeated on the viewer table | pass. The explorer's "unnamed" ids were DOM-only: row action buttons are named through `title` | none |
| 4 | Window labels, column headers, surface names in en and de (136 to 138, 145) | window labels, column headers and surface names | pass | none |
| 5 | Pool sorts Name, Module, Availability, both directions, `aria-sort` (31, 32, 34, 149) | pool sorts | en pass. de FAIL: Module sorted by module id while German labels were shown, rows looked unsorted | Pool and Curated re-sort by the displayed (localized) module label and name |
| 6 | Curated sorts Name, Availability, Count, both directions, `aria-sort` (42 to 44, 47) | curated sorts | en pass, de FAIL (same cause, name) | same |
| 7 | Module toggles Beams, Windows, Slabs, Reuse and combinations, pressed state (40, 121, 124, 127) | module toggles ... | pass | none |
| 8 | Typology: every option and tree path of beams, slabs, windows, reuse (41, 120, 123, 126) | every typology option filters to its tree path | pass for filtering; labels English in de | see 9 |
| 9 | Module labels, typology labels, stock names, Module and Typology columns localized (146, 147) | module toggles carry the module names..., stock names, module column and typology column | FAIL in de: all English | Sourcing catalog i18n: `catalog_module_label`, `catalog_typology_label`, `catalog_kind_name`, `catalog_typology_path` (`🧬️schema/🧾️domain/🦀️.rs`) used by Pool, Curated and Viewer; unit test `every_shipped_module_typology_segment_and_kind_has_an_english_and_a_german_label` |
| 10 | Curate row action and Remove (19, 21, 45) | curate row action adds one more and remove clears the row | pass (journey helper waits for each removal: rapid repeated Remove clicks on a stale table are no-ops) | none |
| 11 | Empty example via the navbar selector (117) | empty example through the navbar selector | pass: selector offers "No example" and "Demo" | none |
| 12 | Empty example via Set Active Example (17, 117) | empty example through Set Active Example | FAIL: form offered only Demo | `setActiveExample` arg options now start with the empty example ("No example"/"Kein Beispiel"); test `set_active_example_offers_the_empty_example_and_every_registered_example` |
| 13 | Demo session example directory (118) | demo session example directory is not offered as an example | by design: `demo-session` is a `.cmd.semio` placeholder script (same in 20 plugins), it is listed in the playground `examples` registry row but is no catalogue document, so registering it in the picker would offer a broken example | none; step proves neither picker offers it |
| 14 | Preview empty state, surface name, follows selection, Escape clears (53 to 56, 145) | preview shows the empty state ... | pass | none |
| 15 | Grid renders reuse GLB meshes (68) | grid draws reuse GLB meshes | FAIL: `/mesh/...glb` answered with the index HTML, page error `Could not load ...glb: Unexpected token '<'`, Grid canvas died | The sourcing component declared no `mesh-collection` asset: added `[[package.metadata.semio.assets]] kind = "mesh-collection"` to `🌎️hub/🧩️compositions/🪵️sourcing/🧩️component/📦️packages/🦀️rust/Cargo.toml` and the generated playground row `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🎮️playgrounds/🟦️.ts` |
| 16 | Grid instance display options (62 to 65) | grid draws reuse GLB meshes (option cycle) | pass | none |
| 17 | Grid rectangle selection (49, 50, 66) | grid pick and shift-drag rectangle selection | plain left drag orbits the camera (by design); Shift plus drag selects the enclosed instances and Preview follows | none. No selection utility is declared, so Utilities stay disabled |
| 18 | Projection and Window Options panes (G) | projection pane, window options pane | pass | none |
| 19 | Window Focus, Unfocus, Close, reopen (5) | window Focus, Unfocus, Close and Display-drag reopen | pointer: Preview works. Windows in the top band have their tab chrome under the navbar chips; after Focus the Unfocus button sits under the Artifact button, so Unfocus is only reachable by keyboard. Palette "Windows" entries only activate a window, they do not reopen a closed one; reopen is the drag of the window template from Display | journey uses the working paths (keyboard for Unfocus and top-band Close, Display drag for reopen). Open: tab chrome collision with the navbar band |
| 20 | Utilities bars | utility bars are named | pass: toggles named, disabled because the windows declare no utilities | none |
| 21 | Window Actions panes (15, 16, 17, 18, 20, 69 to 71, 91 to 93, 98, 99) | window Actions panes list the sourcing commands | pass | none |
| 22 | Command palette finds and runs sourcing commands (C, H) | command palette finds and runs sourcing commands | FAIL: palette showed only panels, windows and shell commands; no sourcing action | `useSearchItems` now adds the window actions flagged `inPalette` of the focused window (arg-less run directly, arg-carrying open the Actions pane form) |
| 23 | Keyboard operation of palette, Escape, Tab, Select All | dialogs and window events work from the keyboard | pass | none |
| 24 | Layout at laptop, tablet, mobile widths, no horizontal overflow (5) | layout survives ... viewport widths | pass | none |
| 25 | Split handle resizing (5) | split handle resizing | pass | none |
| 26 | Editor to Viewer and back by click (2, 6, 8, 9) | switching Editor to Viewer and back | FAIL as soon as anything was curated: `document archive replacement failed ... initialization requires its original installed native semantic producer` | `SourcingViewer` now provides the cold-replay document store owners (`document_store_owners_source_demands`, `build_document_store_owners`) like the draw viewer, with the editor's ordinal receiving and whole-document target; test `viewer_quotes_the_cold_replay_owner_catalog_so_history_bearing_documents_load` |
| 27 | Viewer table read-only, localized, shows curation (100 to 103) | viewer table is read-only, localized and shows the curation | FAIL: columns English, no curation column, `Id` column | viewer table: Name, Module, Typology, Availability, Curated, localized in en and de, rows localized; tests `view_model_*` |
| 28 | Undo shortcut in the viewer | viewer table sorts are not offered as edits and Escape keeps the document | FAIL: Ctrl/Cmd+Z in the viewer dispatched `undo`, the viewer refused it (`viewer.read-only`) and every later call of the instance (role switch, locale refresh) failed | shell key handler no longer dispatches undo and redo for a viewer app (`📤️document-transfer-tabs/🔸️document-transfer-ref/🟦️.tsx`) |
| 29 | Role switch by chord (4) | keyboard shortcuts switch Viewer to Editor and back | pass in a fresh session; repeated switches stalled because each Editor retirement failed (row 30) | see 30 |
| 30 | Editor retirement on role switch | role switches log no page errors | FAIL: every Editor to Viewer switch logged `original lifecycle receipt differs from its caller authority` (warning `predecessor ... retirement failed`), leaking the instance; the fourth switch never completed | close ladder: the window-config store rung debited fuel by copied items only; it now charges `original_close_work` (at least one unit for physical progress), `🔒️close-ladder/🦀️.rs` |
| 31 | Settings language switch en and de at runtime (N) | settings language switch flips every sourcing label | de/en pass after rows 9 and 28 | see rows 9 and 28 |

## Open findings

See the final status block of the run (appended by the journey owner).
