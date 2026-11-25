# 🗺️ Sourcing Plugin Feature Inventory

Read-only inventory of every user-facing feature of the sourcing plugin, for end-to-end verification in the browser.

## Path legend

All paths are relative to `/Users/ueli/Documents/semio/` unless absolute.

| Abbrev | Expands to |
|---|---|
| `P` | `✏️s/🔌️plugins/🪵️sourcing/` |
| `E` | `P🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/` (curation subset root, Rust source) |
| `EE` | `E✏️editor/` (editor app, Rust source) |
| `EW` | `EE🎭️modes/✏️edit/🪟️windows/` (editor windows) |
| `M` | `.cache/b/p/m/browser/prototype/modules/🪵️sourcing/🔣️.json` (staged main manifest: apps, windowKinds, actions, io, keybindings, examples) |
| `MB` / `MS` / `MW` | `.cache/b/p/m/browser/prototype/modules/🪜️sourcing-module-beams/🔣️.json`, `…/🧇️sourcing-module-slabs/🔣️.json`, `…/🪟️sourcing-module-windows/🔣️.json` |
| `FW` | `🧰️framework/🔨️modules/🛂️manifest/🕹️actions/` (framework-declared actions reused by sourcing) |
| `J` | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END/sourcing-journey.ts` |

Journey steps (from `J`, 17 steps, run for `en` and `de`):

1. boot without program/descriptor faults or page errors
2. Pool, Curated, Preview and Grid windows render
3. Pool lists the ten demo stock kinds and the reuse rows
4. Grid renders a 3D canvas
5. curation add: Curated gains the row (uses the Pool stepper, i.e. `curationSetCount` with delta +1, not `curationAdd`)
6. curation set count (Curated stepper, `curationSetCount`)
7. undo and redo of the count edit (`mod+z`, `mod+shift+z`)
8. Preview renders the selected kind (row select, `interactionSelect` pick)
9. curation remove (`curationRemove`)
10. drag pool to curated and back (`dropOnCurated`, `dropOnPool`)
11. filter by query (`setFilterQuery`)
12. filter by module (`setFilterModule`, slabs only)
13. filter by typology (`setFilterTypology`, option index 1 only)
14. filter by minimum availability (`setFilterMinAvailability`)
15. sort by availability (`sortTable`, pool availability column only)
16. example selector Demo to Empty to Demo (`setActiveExample`)
17. folder save, reload and reconnect keep the curated state (host folder sync)

Coverage column values: `step N (name)` for journey coverage, `NO` for not covered. `NO (internal)` marks codecs and plumbing that are not user-facing.

## Feature table

### A. Apps, modes, layout, panels

| # | Feature | Where declared | How a user triggers it (en / de) | Journey |
|---|---|---|---|---|
| 1 | Editor app `s.sourcing.curation@1/*#editor` (role editor, icon library, label Editor/Editor, default mode curation) | `M` apps[0]; `EE📋️manifest/🦀️.rs` | Opened for a `catalogue.sourcing` document (activation `onArtifactKind`) | step 1 (boot) |
| 2 | Viewer app `s.sourcing.curation@1/*#viewer` (role viewer, label Viewer/Betrachter, mode view Ansicht/View) | `M` apps[1]; `E👁️viewer/🎭️modes/👁️view/🦀️.rs` | Opened as viewer of the same document; entry point not found in source | NO |
| 3 | Curation mode (Curation/Kuratierung), single mode in editor | `M` apps[0].modes; `EE🗣️terminology/🦀️.rs` | Implicit default mode | step 1 (implicit) |
| 4 | Terminology breadcrumb "Designing with Existing Stock/Entwerfen mit Bestand > Select/Aussuchen" and app breadcrumb semio > sourcing > curation | `M` apps[0].terminologyBreadcrumbs, breadcrumb; `EE🗣️terminology/🦀️.rs` | Header breadcrumb | NO |
| 5 | Default layout: Pool left; Curated (top, 0.55) over Preview (bottom, 0.45) in the middle; Grid right | `M` apps[0].defaultLayout | Opens with app; split handles resize | step 2 (windows present) |
| 6 | Viewer layout: single Table window (framework.window.table, Table/Tabelle) | `M` apps[1].defaultLayout, windowKinds | Only pane of the viewer | NO |
| 7 | History panel (Verlauf/History, settings group) in editor | `M` apps[0].panelTabs | Settings group panel tab | NO |
| 8 | History panel in viewer | `M` apps[1].panelTabs | Settings group panel tab | NO |
| 9 | Curation mode of the viewer, View/Ansicht mode (eye icon) | `E👁️viewer/🎭️modes/👁️view/🦀️.rs` | Implicit | NO |

### B. Editor windows

| # | Feature | Where declared | How a user triggers it (en / de) | Journey |
|---|---|---|---|---|
| 10 | Pool window (surface `sourcing.pool.table`, icon library, Pool/Pool) | `EW🏊️pool/🦀️.rs`; `M` windowKinds sourcing-pool | Left column of the layout | step 2 (window present) |
| 11 | Curated window (icon tags, Curated/Kuratiert) | `EW🧺️curated/🦀️.rs`; `M` sourcing-curated | Top middle | step 2 |
| 12 | Preview window (World3d object preview, icon preview, Preview/Vorschau) | `EW👁️preview/🦀️.rs`; `M` sourcing-preview | Bottom middle; shows selected kind | step 2 |
| 13 | Grid window (3D mesh canvas, icon grid-3x3, Grid/Raster) | `EW🔢️grid/🦀️.rs`; `M` sourcing-grid | Right column | step 2 |
| 14 | Grid 3D canvas renders | `EW🔢️grid/🦀️.rs` (MeshWindowKit) | Canvas in Grid window | step 4 |
| 15 | Window actions shared by all four editor windows: interactionSelect, interactionHover, clearSelection, selectAll, setSelectionMode, setInteractionGranularity, cancelTypedOperation, hostEvent, exportArtifactDocument, importArtifactDocument | `M` apps[0].windowKinds[*].actions | Per-window palette/keys (see rows below) | partial (see rows) |

### C. Editor tool rows (`bounded_first_step_tool_proofs!`, 15 rows = `SourcingCurationCommand` variants)

| # | Feature | Where declared | How a user triggers it (en / de) | Journey |
|---|---|---|---|---|
| 16 | `loadDocumentJson` Load Document JSON / Dokument-JSON laden, arg `json`, not in palette | `EE🦀️.rs` (tool proof row); `EE🎮️commands/📥️load-document-json/🦀️.rs`; `M` actions | No visible control found in sourcing UI; command channel only | NO |
| 17 | `setActiveExample` Set Active Example / Aktives Beispiel festlegen, arg `exampleId` (options: demo only), palette | `EE🎮️commands/🎬️set-active-example/🦀️.rs`; `M` actions | Navbar example selector (`playground.navbar.fixture`) | step 16 (Demo to Empty to Demo) |
| 18 | `stockFromCatalogue` Stock From Catalogue / Bestand aus Katalog, palette | `EE🎮️commands/📇️stock-from-catalogue/`; `EW🏊️pool/🦀️.rs` (filter bar) | Pool filter bar button "Restock From Catalogue"/"Bestand aus Katalog" (id `sourcing-pool-restock`, icon refresh-cw) | NO |
| 19 | `curationAdd` Curation Add / Kuratierung hinzufügen, arg objectId, not in palette | `EE🎮️commands/➕️curation-add/`; `EW🧺️curated/🦀️.rs` | Curated row "Curate"/"Kuratieren" (plus icon) action button | NO (step 5 uses curationSetCount) |
| 20 | `curationSetCount` Curation Set Count / Kuratierte Anzahl festlegen, args objectId, delta or value, palette | `EE🎮️commands/🔢️curation-set-count/`; `EW🏊️pool/🦀️.rs`, `EW🧺️curated/🦀️.rs` | Stepper +/- or typed number in the "Curated" column (Pool) and "Count" column (Curated) | step 5, step 6 (stepper +) |
| 21 | `curationRemove` Curation Remove / Kuratierung entfernen, arg objectId, destructive | `EE🎮️commands/➖️curation-remove/`; `EW🧺️curated/🦀️.rs` | Curated row "Remove"/"Entfernen" (trash-2 icon) | step 9 |
| 22 | `dropOnPool` Drop On Pool / Auf Pool ablegen | `EE🎮️commands/🏊️drop-on-pool/`; `EE🧱️editor-vocabulary/🦀️.rs` | Drag a Curated row onto the Pool table | step 10 |
| 23 | `dropOnCurated` Drop On Curated / Auf Kuratiert ablegen | `EE🎮️commands/🧺️drop-on-curated/`; `EE🧱️editor-vocabulary/🦀️.rs` | Drag a Pool row onto the Curated table | step 10 |
| 24 | `setFilterQuery` Set Filter Query / Filterabfrage festlegen, arg value | `EE🎮️commands/🔎️set-filter-query/`; `EW🏊️pool/🦀️.rs` | Search text box (Search/Suchen, placeholder Search…/Suchen…, id `sourcing-filter-query`), Change event | step 11 |
| 25 | `setFilterModule` Set Filter Module / Filtermodul festlegen, args moduleId, enabled | `EE🎮️commands/🧱️set-filter-module/`; `EW🏊️pool/🦀️.rs` | Module toggle per installed module (label from module, id `sourcing-filter-module-<moduleId>`, layers icon) | step 12 (slabs only) |
| 26 | `setFilterTypology` Set Filter Typology / Filtertypologie festlegen, arg path or value | `EE🎮️commands/🏛️set-filter-typology/`; `EW🏊️pool/🦀️.rs` | Typology select (Typology/Typologie; All Typologies/Alle Typologien, id `sourcing-filter-typology`) | step 13 (option index 1 only) |
| 27 | `setFilterMinAvailability` Set Filter Min Availability / Mindestverfügbarkeit festlegen, args delta or value | `EE🎮️commands/📉️set-filter-min-availability/`; `EW🏊️pool/🦀️.rs` | Number input Minimum availability/Mindestverfügbarkeit (id `sourcing-filter-min-availability`), Change event | step 14 |
| 28 | `sortTable` Sort Table / Tabelle sortieren, args columnId, direction asc or desc | `EE🎮️commands/↕️sort-table/`; `EE📋️manifest/🦀️.rs` | Click a sortable column header (toggles direction) | step 15 (pool availability only) |
| 29 | `setContributions` Set Contributions / Beiträge festlegen, arg json; host command, not in palette | `EE🎮️commands/🧩️set-contributions/`; `M` commands[0] | Not user-facing; host pushes module packs (topic `sourcing.module`) | NO |
| 30 | `setGridInstanceDisplay` Set Grid Instance Display / Raster-Instanzdarstellung festlegen, arg value | `EE🎮️commands/🔢️set-grid-instance-display/`; `EW🔢️grid/☑️options/🔢️instance-display/🦀️.rs` | Grid window "Instance display"/"Instanzdarstellung" select (3 options, see below) | NO |

### D. Pool window columns, filters and sort

| # | Feature | Where declared | How a user triggers it (en / de) | Journey |
|---|---|---|---|---|
| 31 | Pool column Name/Name (sortable: name) | `EW🏊️pool/🦀️.rs` (`render`, `SOURCING_CURATION_POOL_SORT_COLUMNS`) | Click header | NO |
| 32 | Pool column Module/Modul (sortable: module) | same | Click header | NO |
| 33 | Pool column Typology/Typologie (not sortable) | same | Visible only | step 13 (filter only, column not checked) |
| 34 | Pool column Availability/Verfügbarkeit (sortable: availability) | same | Click header, aria-sort | step 15 |
| 35 | Pool column Curated/Kuratiert (stepper, not sortable) | same | Stepper +/- | steps 5, 6 |
| 36 | Pool table drag source (mime `application/x-semio-sourcing-object`, drag handle `data-drag-role="transfer"`) | `EE🧱️editor-vocabulary/🦀️.rs` (`SOURCING_DRAG_MIME`) | Drag row handle | step 10 |
| 37 | Pool table drop target (`dropOnPool`) | `EW🏊️pool/🦀️.rs` | Drop Curated row | step 10 |
| 38 | Filter combination: query, modules, typology, minimum availability and sort together | `EW🏊️pool/🦀️.rs` (`pool_kinds`, `filtered_stock`) | Set several filters | NO |
| 39 | Restock From Catalogue button (id `sourcing-pool-restock`) | `EW🏊️pool/🦀️.rs` (`filter_bar`) | Button in pool filter bar | NO |
| 40 | Module filter toggles for Beams, Slabs and Windows (one per installed module) | `EW🏊️pool/🦀️.rs` (`available_modules`) | Toggle, pressed state | step 12 (Slabs only) |
| 41 | Typology filter options from module trees (Beams/Solid Timber/Glulam etc.) | `EW🏊️pool/🦀️.rs` (`typology_flatten`) | Select option | step 13 (index 1 only) |

### E. Curated window columns and row actions

| # | Feature | Where declared | How a user triggers it (en / de) | Journey |
|---|---|---|---|---|
| 42 | Curated column Name/Name (sortable: name) | `EW🧺️curated/🦀️.rs` (`render`, `SOURCING_CURATION_CURATED_SORT_COLUMNS`) | Click header | NO |
| 43 | Curated column Availability/Verfügbarkeit (sortable) | same | Click header | NO |
| 44 | Curated column Count/Anzahl (sortable, stepper) | same | Stepper +/- | step 6 |
| 45 | Curated column Actions/Aktionen (Curate/Kuratieren plus, Remove/Entfernen trash) | `EW🧺️curated/🦀️.rs` (`curated_row_actions`) | Row buttons | step 9 (Remove); Curate NO |
| 46 | Curated table drag source and drop target (`dropOnCurated`) | same | Drag row, drop on Curated | step 10 |
| 47 | Sort order of curated table honours name, availability and count only; other sorts fall back to document order | `EW🧺️curated/🦀️.rs` | Header click | NO |

### F. Preview and selection

| # | Feature | Where declared | How a user triggers it (en / de) | Journey |
|---|---|---|---|---|
| 48 | Select rows (interactionSelect, domain rows, granularity object, mode single, merge replace, method pick) | `M` apps[0].interactions; windowKinds interactionSelect | Click a table row cell | step 8 |
| 49 | Select with rectangle method (declared as window arg) | `M` windowKinds interactionSelect args method | Not exposed by interaction definition (pick only) | NO |
| 50 | Select with lasso method (declared as window arg) | `M` windowKinds interactionSelect args method | Not exposed by interaction definition (pick only) | NO |
| 51 | Select merge modes additive, subtractive, invertive, range (window arg merge) | `M` windowKinds interactionSelect args merge | Not exposed by interaction definition (replace only) | NO |
| 52 | Hover broadcast on pointer channel (interactionHover) | `M` interactions hover enabled | Pointer hover over rows | NO |
| 53 | Preview renders selected kind (World3d, fit camera per geometry) | `EW👁️preview/🦀️.rs` | Select a row, Preview shows object | step 8 |
| 54 | Preview empty state "No selection"/"Keine Auswahl" | `EE🗣️terminology/🦀️.rs` (`no_selection`) | Nothing selected | NO |
| 55 | Preview surface name Object preview/Objektvorschau | `EW👁️preview/🦀️.rs` | Accessible name of canvas surface | NO |
| 56 | Clear Selection/Auswahl aufheben (clearSelection) | `M` windowKinds, `FW⏪️…`/`FW🕹️interaction-verbs/` | Escape key (keybinding) | NO |
| 57 | Select All/Alles auswählen (selectAll) | `M` windowKinds | Ctrl/Cmd+A (keybinding) | NO |
| 58 | Set Selection Mode single or multiple (setSelectionMode) | `M` windowKinds | Not in palette; command channel | NO |
| 59 | Set Granularity/Granularität, object (setInteractionGranularity) | `M` windowKinds | Not in palette | NO |
| 60 | Cancel Operation/Vorgang abbrechen (cancelTypedOperation) | `M` windowKinds (all four editor windows and viewer Table) | Cancel typed operation | NO |
| 61 | Window Event (hostEvent) kinds blur, captureLost, retiring | `M` windowKinds hostEvent; `FW⏪️history-and-host-event-actions/` | Host-driven when window loses focus or closes | NO |

### G. Grid window

| # | Feature | Where declared | How a user triggers it (en / de) | Journey |
|---|---|---|---|---|
| 62 | Grid instance display "Instance display"/"Instanzdarstellung" select (measure id `sourcing-grid-window.instance-display`) | `EW🔢️grid/☑️options/🔢️instance-display/🦀️.rs`; labels `EE🗣️terminology/🦀️.rs` | Select in Grid window | NO |
| 63 | Instance display option Line behind grid/Linie hinter Raster (`line-behind`, copies in a line behind) | `EW🔢️grid/🎚️config/🦀️.rs` | Choose option | NO |
| 64 | Instance display option Representative only/Nur Vertreter (`representative`, one copy) | same | Choose option | NO |
| 65 | Instance display option Representative and count/Vertreter und Anzahl (`representative-with-count`, count glyphs) | same; `EW🔢️grid/🦀️.rs` (`append_grid_count_glyph_instances`) | Choose option | NO |
| 66 | Grid rectangle selection on canvas | `EW🔢️grid/🦀️.rs` (`world3d_selection_json("rectangle")`) | Drag rectangle in canvas | NO |
| 67 | Grid geometry meshes: box, slab and frame kinds, plus unit box | `EW🔢️grid/🦀️.rs` (`box_parts`, `append_kind_scene_meshes`) | Canvas display | step 4 (canvas only) |
| 68 | Reuse GLB meshes in Grid (`/mesh/🧊️hexagonal-cut-concrete-forest-left.glb`, and right) | `E📚️examples/🎬️demo/`, `M` examples[0] | Canvas display | NO |

### H. Editor actions (palette, keyboard, history, clipboard)

| # | Feature | Where declared | How a user triggers it (en / de) | Journey |
|---|---|---|---|---|
| 69 | Undo/Rückgängig (undo, palette, icon undo-2) | `FW⏪️history-and-host-event-actions/🦀️.rs`; `M` apps[0].actions | Ctrl/Cmd+Z (keybinding mod+z) or palette | step 7 |
| 70 | Redo/Wiederholen (redo, palette, icon redo-2) | same | Ctrl/Cmd+Shift+Z (mod+shift+z) or palette | step 7 |
| 71 | Commit Checkpoint/Checkpoint festschreiben (commitCheckpoint, palette, icon git-commit) | same | Palette | NO |
| 72 | Create Alternative/Alternative erstellen (createAlternative, palette, icon git-branch) | same | Palette | NO |
| 73 | Switch Alternative/Alternative wechseln (switchAlternative, arg alternativeId, palette) | same | Palette | NO |
| 74 | Checkout Checkpoint/Checkpoint auschecken (checkoutCheckpoint, palette) | same | Palette | NO |
| 75 | Revert to Command/Auf Befehl zurücksetzen (revertToCommand, arg entrySeq, icon clock) | same | History panel entry | NO |
| 76 | Edit Mutation/Mutation bearbeiten (historyEditBegin, args mutationId, store) | same | History panel | NO |
| 77 | Set Mutation Input/Mutationseingabe setzen (historyEditInput, args path, value, edit insert or remove, generation) | same | History edit form | NO |
| 78 | Use Selection/Auswahl verwenden (historyEditUseSelection, args path, generation) | same | History edit form | NO |
| 79 | Withdraw Mutation/Mutation zurückziehen (historyEditWithdraw) | same | History edit form | NO |
| 80 | Restore Mutation/Mutation wiederherstellen (historyEditRestore) | same | History edit form | NO |
| 81 | Accept Draft/Entwurf übernehmen (historyEditAccept) | same | History edit form | NO |
| 82 | Discard Draft/Entwurf verwerfen (historyEditDiscard) | same | History edit form | NO |
| 83 | Finalize History Edit/Verlaufsbearbeitung abschließen (historyEditFinalize) | same | Opens finish dialog | NO |
| 84 | Commit History Edit/Verlaufsbearbeitung festschreiben (historyEditCommit, choice overwrite, destructive) | same | Dialog submit | NO |
| 85 | Back to Review/Zurück zur Prüfung (historyEditBack) | same | Dialog cancel | NO |
| 86 | Exit History Editing/Verlaufsbearbeitung beenden (historyEditExit) | same | History panel | NO |
| 87 | Cancel Replay/Neuanwendung abbrechen (historyEditCancelReplay) | same | History panel | NO |
| 88 | Replay Again/Erneut anwenden (historyEditRerun) | same | History panel | NO |
| 89 | Finish-history dialog "Finish editing history/Verlaufsbearbeitung abschließen" with Overwrite/Überschreiben (danger) and submit New alternative/Neue Alternative | `M` apps[0].dialogs; `EE` dialog declaration | After History Edit Finalize | NO |
| 90 | History command filter Set History Filter/Verlaufsfilter festlegen (values all, withoutMutations, onlyMutations) | `FW` shared; `M` apps[0] and apps[1] | History panel select | NO |
| 91 | Copy/Kopieren (copy, palette) | `FW` shared clipboard; `M` apps[0] | Ctrl/Cmd+C; editor and viewer | NO |
| 92 | Cut/Ausschneiden (cut, palette) | `FW` shared clipboard; `M` apps[0] | Ctrl/Cmd+X (mod+x) | NO |
| 93 | Paste/Einfügen (paste, args anchor: original, middle, centroid, bottomLeft, bottomRight, topLeft, topRight; position vector) | `FW` shared clipboard; `M` apps[0] | Ctrl/Cmd+V (mod+v) | NO |
| 94 | Select All keybinding Ctrl/Cmd+A | `M` apps[0].keybindings | Keyboard | NO |
| 95 | Escape to clear selection | `M` apps[0].keybindings | Keyboard | NO |
| 96 | Note Shell Command/Shell-Befehl vermerken (noteShellCommand, args commandId, label, detail) | `FW` shared; `M` apps[0] and apps[1] | Palette or command channel | NO |
| 97 | Record Tutorial/Tutorial aufzeichnen (recordTutorial) | `FW` shared; `M` apps[0] and apps[1] | Palette | NO |
| 98 | Export Document/Dokument exportieren (exportArtifactDocument, shell, editor windows and viewer Table) | `M` windowKinds; `FW🗃️utility-tool-and-document-actions/` | Window action | NO |
| 99 | Import Document…/Dokument importieren… (importArtifactDocument, shell) | same | Window action | NO |

### I. Viewer app (`#viewer`)

| # | Feature | Where declared | How a user triggers it (en / de) | Journey |
|---|---|---|---|---|
| 100 | Viewer Copy keybinding mod+c | `M` apps[1].keybindings | Keyboard | NO |
| 101 | Viewer Table window with cancelTypedOperation, hostEvent, export and import document | `M` apps[1].windowKinds | Only pane | NO |
| 102 | Viewer history filter, note shell command, record tutorial, copy | `M` apps[1].actions | Palette | NO |
| 103 | Viewer io: none declared (exportFormats and importFormats empty) | `M` apps[1].io | Not applicable | NO |

### J. Import and export formats (both directions)

| # | Feature | Where declared | How a user triggers it (en / de) | Journey |
|---|---|---|---|---|
| 104 | Text utf-8 DSL export (`semio curation.curation.dsl`) | `E🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs`; `EW`/`P` io entry in `MBS`-style `M` contributions (`ioEntries` 1) | Export Document (window action) | NO |
| 105 | Text utf-8 DSL import | `E🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs`; `M` ioEntries[0] | Import Document | NO |
| 106 | JSON rfc8259 export | `E🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs`; ioEntries 2 | Export Document | NO |
| 107 | JSON rfc8259 import | `E🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs`; ioEntries 4 | Import Document | NO |
| 108 | ZIP 2.0 export (DSL authoritative member plus rfc8259, exact fidelity) | `E🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs`; ioEntries 3 | Export Document | NO |
| 109 | ZIP 2.0 import | `E🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/🔖️2.0/✳️any/🦀️.rs`; ioEntries 5 | Import Document | NO |
| 110 | Load Document JSON (whole-document replace outside history from JSON) | `EE🎮️commands/📥️load-document-json/🦀️.rs` | Command channel only, see row 16 | NO |
| 111 | Catalog output port `catalog:out` (kit.catalog for downstream puzzle kit import) | `E🚪️io/🦀️.rs` and `EE🧱️editor-vocabulary/🦀️.rs` (`sourcing_curation_io`) | Connect port in graph | NO |
| 112 | Binary snapshot codec | `E🚪️io/💾️binary/📸️snapshot/` | Not in manifest exportFormats; internal | NO (internal) |
| 113 | SQLite snapshot codec | `E🚪️io/🪶️sqlite/📸️snapshot/` | Not in manifest exportFormats; internal | NO (internal) |
| 114 | Diff codecs (text and binary) | `E🚪️io/📝️text/🔺️diff/`, `E🚪️io/💾️binary/🔺️diff/` | Undo/history internals | NO (internal) |
| 115 | Mutation codecs (text and binary) for CreateCuratedItem, ChangeCuratedItemCount, DeleteCuratedItem | `E🚪️io/📝️text/🧬️mutations/`, `E🚪️io/💾️binary/🧬️mutations/` | History persistence | NO (internal) |

### K. Examples

| # | Feature | Where declared | How a user triggers it (en / de) | Journey |
|---|---|---|---|---|
| 116 | Demo example (id demo, label Demo/Demo): 12 stock kinds (4 beams, 3 windows, 3 slabs, 2 reuse), empty curation | `E📚️examples/🎬️demo/`; `M` examples[0]; `EE🧱️editor-vocabulary/🦀️.rs` (`examples()`) | Example selector "Example"/"Beispiel" = Demo | step 3, step 16 |
| 117 | Empty example (id `""`, `EMPTY_CURATION_TEXT`), handler exists but not offered in the picker | `EE🎮️commands/🎬️set-active-example/🦀️.rs`; `EE🧱️editor-vocabulary/🦀️.rs` (`EMPTY_EXAMPLE_ID`); picker options from `examples()` only, see `EE📋️manifest/🦀️.rs` line 117 | Example selector (no Empty option offered) | NO (step 16 expected to fail) |
| 118 | Demo session example directory (label Demo Session/Demo-Sitzung) | `EE📚️examples/🎬️demo-session/` | Not registered in `examples()` | NO (not user-facing) |

### L. Module plugins (contributions)

| # | Feature | Where declared | How a user triggers it (en / de) | Journey |
|---|---|---|---|---|
| 119 | Beams module (label Beams, icon beam): stock Glulam GL24h 200×400 (24), KVH C24 100×200 (60), Steel IPE 200 (12), Steel HEA 160 (8) | `MB` topicContributions; `P🧩️extensions/🪵️beams/` | Rows in Pool; restock | step 3 (list), step 14 (min availability) |
| 120 | Beams typology tree: Beams > Solid Timber (Glulam, KVH), Steel (IPE, HEA) | `MB` topicContributions typologyJson | Typology select | NO |
| 121 | Beams module filter toggle (English label "Beams", not localized) | `EW🏊️pool/🦀️.rs` | Toggle | NO |
| 122 | Slabs module (label Slabs, icon slab): Concrete Slab 240mm (30), CLT Slab 160mm (20), Hollow Core Slab 265mm (16) | `MS` topicContributions; `P🧩️extensions/🧱️slabs/` | Rows in Pool | step 3 |
| 123 | Slabs typology tree: Slabs > Concrete, CLT, Hollow Core | `MS` typologyJson | Typology select | NO |
| 124 | Slabs module filter toggle | `EW🏊️pool/🦀️.rs` | Toggle, pressed | step 12 |
| 125 | Windows module (label Windows, icon window): Casement Window 100×120 (18), Fixed Window 150×150 (10), Tilt and Turn Window 120×140 (14) | `MW` topicContributions; `P🧩️extensions/🪟️windows/` | Rows in Pool | step 3 |
| 126 | Windows typology tree: Windows > Casement, Fixed, Tilt and Turn | `MW` typologyJson | Typology select | NO |
| 127 | Windows module filter toggle | `EW🏊️pool/🦀️.rs` | Toggle | NO |
| 128 | Reuse stock rows (module-id reuse, Hexagonal Cut Concrete Forest Left/Right, availability 12) | `E📚️examples/🎬️demo/` | Rows in Pool | step 3 |
| 129 | Contributions lane: host pushes sourcing.module packs, capped at 2048 bytes; installable_contributions keeps actionable modules | `EE🧱️editor-vocabulary/🦀️.rs` (`SOURCING_CURATION_CONFIG_CONTRIBUTIONS_BYTES`) | Host, not UI | NO |
| 130 | Restock from catalogue adds all installed module kinds, keeping existing counts | `M` apps[0].actions stockFromCatalogue description | Restock button | NO |

### M. Settings and customization

| # | Feature | Where declared | How a user triggers it (en / de) | Journey |
|---|---|---|---|---|
| 131 | Filter config (query, module ids, typology path, min availability, sort) in SourcingCurationConfig | `EE🎚️config/🦀️.rs` (`filters_*`, dsl keys `filter-query`, `filter-modules`, `filter-typology`, `filter-min-availability`, `sort`) | Filter controls | steps 11 to 15 |
| 132 | Grid instance display config (`line-behind`, `representative`, `representative-with-count`) | `EW🔢️grid/🎚️config/🦀️.rs` | Grid select | NO |
| 133 | Contributions config (`contributions_json`) | `EE🎚️config/🦀️.rs` | Host | NO |
| 134 | Folder sync: attach folder, persist events, reload, reconnect curated state | host sync (not sourcing source); journey uses `#s-sync-status`, `framework.sync.folder`, `#s-folder-reconnect` | Sync menu: Attach/Verbinden | step 17 |
| 135 | Presence (multi-user) | `EE👥️presence/` and `E✏️editor/…/👥️presence/` contain only empty placeholders | Not implemented | NO (not implemented) |

### N. Internationalization (en and de)

| # | Feature | Where declared | How a user triggers it (en / de) | Journey |
|---|---|---|---|---|
| 136 | Window labels Pool, Curated/Kuratiert, Preview/Vorschau, Grid/Raster | `EE🗣️terminology/🦀️.rs` (`SourcingLabels`) | Window headers | step 2 (en and de) |
| 137 | Filter labels Search/Suchen, Search…/Suchen…, Minimum availability/Mindestverfügbarkeit, All Typologies/Alle Typologien | same | Filter bar | steps 11, 13, 14 |
| 138 | Column labels Name/Name, Module/Modul, Typology/Typologie, Availability/Verfügbarkeit, Curated/Kuratiert, Count/Anzahl, Actions/Aktionen | same | Table headers | steps 3, 6, 15 |
| 139 | Row action labels Curate/Kuratieren, Remove/Entfernen | same | Row buttons | step 9 (Remove) |
| 140 | Restock From Catalogue/Bestand aus Katalog label | same | Pool filter bar | NO |
| 141 | No selection/Keine Auswahl | same | Preview empty state | NO |
| 142 | Instance display labels Instance display/Instanzdarstellung, Line behind grid/Linie hinter Raster, Representative only/Nur Vertreter, Representative and count/Vertreter und Anzahl | same | Grid select | NO |
| 143 | Drag hint Drag to window/Verschieben (matched by journey) | host drag UI | Drag handle | step 10 |
| 144 | Command labels and descriptions (en and de) for all manifest actions | `M` apps[*].actions label, semantics.description | Palette, tooltips | partly (see rows 16 to 68) |
| 145 | Table surface names Pool table/Pool-Tabelle, Curated table/Kuratierte Tabelle, Object preview/Objektvorschau | `EW🏊️pool/🦀️.rs`, `EW🧺️curated/🦀️.rs`, `EW👁️preview/🦀️.rs` | Accessible names | NO |
| 146 | Module labels and typology labels (Beams, Slabs, Windows, Solid Timber etc.) are English-only strings (no German) | `MB`, `MS`, `MW` topicContributions label | Toggles and typology select in German locale | NO (gap) |
| 147 | Example and stock names are English-only (Glulam GL24h 200×400 etc.) | `E📚️examples/🎬️demo/`, module descriptors | Rows in German locale | NO (gap) |

### O. Accessibility and keyboard

| # | Feature | Where declared | How a user triggers it (en / de) | Journey |
|---|---|---|---|---|
| 148 | Window panels exposed as tabpanel named by window label | host layout; journey `getByRole("tabpanel")` | Screen reader | step 2 |
| 149 | Sortable column headers with button and aria-sort | `EW🏊️pool/🦀️.rs`, `EW🧺️curated/🦀️.rs` | Header button | step 15 |
| 150 | Named textbox, spinbutton, combobox and toggle for filters | `EW🏊️pool/🦀️.rs` | Keyboard entry | steps 11 to 14 |
| 151 | Drag handle `data-drag-role="transfer"` with localized drag hint | `EE🧱️editor-vocabulary/🦀️.rs` | Keyboard or pointer drag | step 10 |
| 152 | Keyboard shortcuts editor: mod+z, mod+shift+z, mod+c, mod+x, mod+v, escape, mod+a | `M` apps[0].keybindings | Keyboard | steps 7 (undo, redo) only |
| 153 | Keyboard shortcut viewer: mod+c | `M` apps[1].keybindings | Keyboard | NO |
| 154 | Window events blur, captureLost and retiring end open gestures cleanly | `M` windowKinds hostEvent | Focus or close | NO |
| 155 | Dialog focus and keyboard flow for finish-history dialog | `M` apps[0].dialogs | Keyboard | NO |

## Summary counts

- Feature rows in table: 155 (rows 1 to 155).
- Rows covered by journey (`step N`): see column; uncovered rows are listed below.
- Journey steps: 17. Known expected failure: step 16 (Empty example not offered in picker, row 117).

## NOT covered features

Editor and viewer apps and modes:
- Viewer app entry point and whole viewer surface (rows 2, 6, 8, 9, 100 to 103)
- Breadcrumb and terminology labels (row 4)
- Viewer Table window (row 101)

Tool rows:
- `loadDocumentJson` (row 16, row 110)
- `stockFromCatalogue` / Restock From Catalogue (rows 18, 39, 130, 140)
- `curationAdd` / Curate row action (row 19)
- `setFilterModule` for Beams and Windows (rows 121, 127)
- `setFilterTypology` for other options and tree paths (rows 41, 120, 123, 126)
- `sortTable` on Pool name and module, Curated name, availability, count (rows 31, 32, 42, 43, 47)
- `setContributions` (row 29, row 129, row 133)
- `setGridInstanceDisplay` and all three display options (rows 30, 62 to 65, 132)
- `setActiveExample` Empty option (row 117)
- Set Filter Query/Min Availability with typed-value edge cases beyond the journey (rows 24, 27)

Interactions and windows:
- Rectangle, lasso, additive, subtractive, invertive, range selection (rows 49 to 51)
- Hover (row 52)
- Clear selection, select all, selection mode, granularity (rows 56 to 59)
- Cancel operation and host window events (rows 60, 61, 154)
- Preview empty state and surface names (rows 54, 55, 145)
- Grid rectangle selection and grid reuse meshes (rows 66, 68)
- Layout resizing and Preview/Grid interactions beyond rendering (row 5)
- Typed Count input in the Curated and Pool steppers (row 20)

History (framework-declared):
- Commit checkpoint, create alternative, switch alternative, checkout checkpoint, revert to command (rows 71 to 75)
- Full History edit chain: begin, input, use selection, withdraw, restore, accept, discard, finalize, commit, back, exit, cancel replay, rerun (rows 76 to 88)
- Finish-history dialog with Overwrite and New alternative (row 89)
- History panel and command filter (rows 7, 8, 90)

Clipboard and editing:
- Copy, cut, paste with anchors (rows 91 to 93)
- Escape, select all keyboard shortcuts (rows 94, 95, 152, 153)
- Note Shell Command and Record Tutorial (rows 96, 97)

Import and export:
- Export Document and Import Document window actions (rows 98, 99)
- Text utf-8 export and import (rows 104, 105)
- JSON rfc8259 export and import (rows 106, 107)
- ZIP 2.0 export and import (rows 108, 109)
- Catalog output port `catalog:out` (row 111)
- Binary snapshot, SQLite snapshot, diff and mutation codecs are internal and not user-facing (rows 112 to 115)

Examples and modules:
- Empty example (row 117)
- Beams, Slabs and Windows typology trees and module toggles beyond the slabs toggle (rows 120, 123, 126, 127)
- Module contribution lane (row 129)
- Demo session example directory is not registered (row 118)
- Reuse GLB meshes in Grid (row 68)

Settings, sync and i18n gaps:
- Grid instance display config (row 132), contributions config (row 133)
- Folder sync is covered only as save, reload and reconnect (row 134 partly)
- German localization of module labels and stock names (rows 146, 147)
- Command descriptions and tooltips not visually verified (row 144)

Accessibility:
- Table surface accessible names (row 145)
- Keyboard flow for dialogs and window events (rows 154, 155)
