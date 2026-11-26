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
| 17 | Grid rectangle selection (49, 50, 66) | grid pick and shift-drag rectangle selection; grid Select utility turns a plain drag into rectangle selection | plain drag orbits; Shift plus drag selects; no utility declared (Utilities disabled) | the editor manifest declares a `select` utility for the Grid window (`SOURCING_UTILITY_SELECT`, like block3d); wasm change, journey step passes once the next hub build is staged |
| 18 | Projection and Window Options panes (G) | projection pane, window options pane | pass | none |
| 19 | Window Focus, Unfocus, Close, reopen (5) | window Focus, Unfocus and Close work with the mouse on every window, keyboard Close works, palette and Display drag reopen | FAIL for a mouse user: the tab chips of Pool, Curated and Grid sat under the navbar controls (Focus, Close, and after Focus the Unfocus button, unreachable); palette Windows entries only activated, never reopened | `useNavbarBandShift` in the dock steps chips of top-edge stacks below the navbar band (`🎨️Canvas/⚓️dock/🟦️.tsx`); palette Windows entries now reopen a closed window (`search-items`, `handleTemplateDrop` to the right of the layout); Display drag still works |
| 20 | Utilities bars | utility bars are named | pass: toggles named, disabled because the windows declare no utilities | none |
| 21 | Window Actions panes (15, 16, 17, 18, 20, 69 to 71, 91 to 93, 98, 99) | window Actions panes list the sourcing commands | pass | none |
| 22 | Command palette finds and runs sourcing commands (C, H) | command palette finds and runs sourcing commands | FAIL: palette showed only panels, windows and shell commands; no sourcing action | `useSearchItems` now adds the window actions flagged `inPalette` of the focused window (arg-less run directly, arg-carrying open the Actions pane form) |
| 23 | Keyboard operation of palette, Escape, Tab, Select All | dialogs and window events work from the keyboard | pass | none |
| 24 | Layout at laptop, tablet, mobile widths, no horizontal overflow (5) | layout survives ... viewport widths | pass | none |
| 25 | Split handle resizing (5) | split handle resizing | pass | none |
| 26 | Editor to Viewer and back by click (2, 6, 8, 9) | switching Editor to Viewer and back | FAIL as soon as anything was curated: `document archive replacement failed ... initialization requires its original installed native semantic producer` | `SourcingViewer` now provides the cold-replay document store owners (`document_store_owners_source_demands`, `build_document_store_owners`) like the draw viewer, with the editor's ordinal receiving and whole-document target; test `viewer_quotes_the_cold_replay_owner_catalog_so_history_bearing_documents_load` |
| 27 | Viewer table read-only, localized, shows curation (100 to 103) | viewer table is read-only, localized and shows the curation | FAIL: columns English, no curation column, `Id` column | viewer table: Name, Module, Typology, Availability, Curated, localized in en and de, rows localized; tests `view_model_*` |
| 28 | Undo shortcut in the viewer | viewer table sorts are not offered as edits and Escape keeps the document | FAIL: Ctrl/Cmd+Z in the viewer dispatched `undo`, the viewer refused it (`viewer.read-only`) and every later call of the instance (role switch, locale refresh) failed | shell key handler no longer dispatches undo and redo for a viewer app (`📤️document-transfer-tabs/🔸️document-transfer-ref/🟦️.tsx`) |
| 29 | Role switch by chord (4) | keyboard shortcuts switch Editor to Viewer and back | pass in a fresh session; after several round trips the next switch stalled because each failed Editor retirement leaked its instance (row 30) | see 30 |
| 30 | Editor retirement on role switch | four consecutive role round trips complete | FAIL: every Editor to Viewer switch logged `original lifecycle receipt differs from its caller authority` (warning `predecessor ... retirement failed`), leaking the instance; the fourth switch never completed | close ladder: the window-config store rung debited fuel by copied items only; it now charges `original_close_work` (at least one unit for physical progress), `🔒️close-ladder/🦀️.rs` |
| 31 | Settings language switch en and de at runtime (N) | settings language switch flips every sourcing label | de/en pass after rows 9 and 28 | see rows 9 and 28 |

## Final status

Latest full run (host with the 22:59 component, TS fixes live): `journey-surfaces.ts` en and de, 34 steps each, 64 of 68 pass. The four failures are the two features that need the next hub rebuild, in both languages: `grid Select utility turns a plain drag into rectangle selection` (manifest declares the `select` utility, wasm) and `four consecutive role round trips complete` (close-ladder fix, wasm). `sourcing-journey.ts` passes 17 of 17 in en and in de (shared kit now exports localized stock names through the session: `s.kinds`, `s.reuse`, `s.n(english)`).

Pending: both wasm changes are written and batched; the hub build is refused/queued behind the carve (`hub-carve-build.sh` holds the lock). `scratchpad rebuild4.sh` retries build, stage and host restart under the lock; once it reports `OK attempt`, rerun `JOURNEY=surfaces bun journey-surfaces.ts en de` and `bun sourcing-journey.ts en de`.

## Open findings

1. Editor retirement fault (row 30) and the Grid select utility (row 17): fixes written, awaiting the hub rebuild and browser proof.
2. No Rust unit test has been executed (native test target hung in `source.served`); non-test code compiled in the 22:16 wasm build.
3. The generated playground registry row was edited by hand to match the new `mesh-collection` asset; regenerate when the graph is available.
4. The hub descriptor pair is stale independent of this ticket.
5. Resolved since the first report: tab chrome under the navbar (dock steps top-edge chips below the band), palette reopen of closed windows, kit localized stock names.
