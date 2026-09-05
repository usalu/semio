# Sourcing End-to-End UI Dispatch Map

**Sourcing Plugin Path:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪵️sourcing/`

**Source Command File:** `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (lines 128–245)

---

## I. UI Windows

### Window Definitions

| Window ID | On-Screen Label (EN/DE) | DOM/Body Key | Surface Kind | Icon | Path |
|---|---|---|---|---|---|
| `sourcing-pool` | "Pool" / "Pool" | `sourcing.pool` | Table | `library` | `🎭️modes/✏️edit/🪟️windows/🏊️pool/🦀️.rs` |
| `sourcing-curated` | "Curated" / "Kuratiert" | `sourcing.curated` | Table | `tags` | `🎭️modes/✏️edit/🪟️windows/🧺️curated/🦀️.rs` |
| `sourcing-preview` | "Preview" / "Vorschau" | `sourcing.preview` | World3d (Mesh) | `preview` | `🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs` |
| `sourcing-grid` | "Grid" / "Raster" | `sourcing.grid` | World3d (Mesh) | `grid-3x3` | `🎭️modes/✏️edit/🪟️windows/🔢️grid/🦀️.rs` |

### Window Layouts

**Pool (Table):** 5 columns: Name | Module | Typology (path joined by " / ") | Availability | Curated Count

**Curated (Table):** 3 columns: Name | Availability | Count

**Preview (3D):** Single selected item rendered as 3D mesh; defaults to placeholder text "No selection" if none selected.

**Grid (3D):** All filtered stock laid out on a 2m×2m cell grid with scaled placement; renders as 3D mesh instances.

---

## II. The 14 UI-Reachable Commands

All 14 commands are declared in `SOURCING_CURATION_BOUNDED_TOOL_IDS` (line 230–245) and routed via `sourcing_curation_command_from_action()` (line 172–214).

| # | Command ID | DSL Key | Source Path | Publication Lane | UI Affordance |
|---|---|---|---|---|---|
| 1 | `setActiveExample` | `active-example` | `🎮️commands/🎬️set-active-example/🦀️.rs` | HostOnly | Selector: "Demo" ↔ "Empty" (example dropdown or buttons) |
| 2 | `setDocument` | `document-json` | `🎮️commands/🗿️set-artifact-json/🦀️.rs` | HostOnly | File import / paste JSON (full document replacement) |
| 3 | `stockFromCatalogue` | `stock-from-catalogue` | `🎮️commands/📇️stock-from-catalogue/🦀️.rs` | HostOnly | Button: "Reset from Catalogue" (loads current stock from host catalog) |
| 4 | `curationAdd` | `curation-add` | `🎮️commands/➕️curation-add/🦀️.rs` | Artifact | Pool table: click row or drag → "+" button; adds to Curated with count=1 |
| 5 | `curationSetCount` | `curation-set-count` | `🎮️commands/🔢️curation-set-count/🦀️.rs` | Artifact | Curated table: edit count cell (stepper/number input); sends delta or absolute value |
| 6 | `curationRemove` | `curation-remove` | `🎮️commands/➖️curation-remove/🦀️.rs` | Artifact | Curated table: row's "×" button or drag out; removes from Curated |
| 7 | `dropOnPool` | `drop-on-pool` | `🎮️commands/🏊️drop-on-pool/🦀️.rs` | Artifact | Drag Curated row → Pool pane (removes from Curated) |
| 8 | `dropOnCurated` | `drop-on-curated` | `🎮️commands/🧺️drop-on-curated/🦀️.rs` | Artifact | Drag Pool row → Curated pane (adds to Curated or increments) |
| 9 | `setFilterQuery` | `filter-query` | `🎮️commands/🔎️set-filter-query/🦀️.rs` | Config | Pool pane: text input "Search" (free-text filter by name) |
| 10 | `setFilterModule` | `filter-module` | `🎮️commands/🧱️set-filter-module/🦀️.rs` | Config | Pool pane: checkboxes for "beams", "windows", "slabs" (module filter toggles) |
| 11 | `setFilterTypology` | `filter-typology` | `🎮️commands/🏛️set-filter-typology/🦀️.rs` | Config | Pool pane: tree/drill-down selector (e.g., "beams > solid-timber > glulam") |
| 12 | `setFilterMinAvailability` | `filter-min-availability` | `🎮️commands/📉️set-filter-min-availability/🦀️.rs` | Config | Pool pane: slider or number input (filters to items with ≥ value) |
| 13 | `sortTable` | `sort-table` | `🎮️commands/↕️sort-table/🦀️.rs` | Config | Pool table header: click column to sort (columnId, direction: "asc"/"desc") |
| 14 | `setContributions` | `contributions` | `🎮️commands/🧩️set-contributions/🦀️.rs` | Config | Advanced: JSON editor panel for module contributions metadata |

---

## III. Demo Fixture: 10 Kinds in 3 Modules

**Source:** `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` (demo_stock() function, module implementations)

### Beams Module

| Kind ID | Name | Typology Path | Availability | Geometry |
|---|---|---|---|---|
| `beam-glulam-gl24h` | Glulam GL24h 200×400 | beams / solid-timber / glulam | 24 | Box(0.2×0.4×6.0m) |
| `beam-kvh-c24` | KVH C24 100×200 | beams / solid-timber / kvh | 60 | Box(0.1×0.2×4.0m) |
| `beam-steel-ipe200` | Steel IPE 200 | beams / steel / ipe | 12 | Box(0.1×0.2×5.0m) |
| `beam-steel-hea160` | Steel HEA 160 | beams / steel / hea | 8 | Box(0.16×0.152×5.0m) |

### Windows Module

| Kind ID | Name | Typology Path | Availability | Geometry |
|---|---|---|---|---|
| `window-casement-100x120` | Casement Window 100×120 | windows / casement | 18 | Frame(1.0×1.2, depth 0.08m, profile 0.08m) |
| `window-fixed-150x150` | Fixed Window 150×150 | windows / fixed | 10 | Frame(1.5×1.5, depth 0.06m, profile 0.06m) |
| `window-tilt-turn-120x140` | Tilt & Turn Window 120×140 | windows / tilt-turn | 14 | Frame(1.2×1.4, depth 0.09m, profile 0.09m) |

### Slabs Module

| Kind ID | Name | Typology Path | Availability | Geometry |
|---|---|---|---|---|
| `slab-concrete-240` | Concrete Slab 240mm | slabs / concrete | 30 | Slab(2.4×1.2, thickness 0.24m) |
| `slab-clt-160` | CLT Slab 160mm | slabs / clt | 20 | Slab(2.95×1.25, thickness 0.16m) |
| `slab-hollow-core-265` | Hollow Core Slab 265mm | slabs / hollow-core | 16 | Slab(1.2×6.0, thickness 0.265m) |

---

## IV. Test Script: Click-by-Click Interactions

Each step shows the action, the expected visible state change, and which command(s) fire.

### Part A: Initialization & Example Switching

```
1. [OPEN APP AT http://127.0.0.1:6081]
   Expect: Four panes visible (Pool, Curated, Preview, Grid)
   Locale default: English (en-US)
   Pool displays all 10 kinds with columns: Name | Module | Typology | Availability | Curated Count
   Curated table empty
   Preview shows "No selection" placeholder
   Grid shows all 10 items on 2m grid
   Example selector shows "Demo" selected

2. [CLICK EXAMPLE DROPDOWN, SELECT "Empty"]
   Fire: setActiveExample {exampleId: "empty-curation"}
   Expect: 
     - Pool table empty (stock is empty)
     - Curated table empty
     - Preview still "No selection"
     - Grid empty
     - Example selector now shows "Empty"

3. [CLICK EXAMPLE DROPDOWN, SELECT "Demo"]
   Fire: setActiveExample {exampleId: "demo-stock"}
   Expect: Back to state from step 1 (10 items visible in all views)
```

### Part B: Adding & Removing from Curation

```
4. [POOL TABLE: DOUBLE-CLICK OR SELECT row "Glulam GL24h 200×400"]
   Expect: Row highlight (selection)
   
5. [POOL TABLE: CLICK "+" button on same row, OR drag to Curated pane]
   Fire: curationAdd {objectId: "beam-glulam-gl24h"}
   Expect:
     - "Glulam GL24h 200×400" appears in Curated table with count=1
     - Curated column for this row in Pool now shows "1"
     - Preview renders the beam geometry (3D box)
     - Grid highlights or marks this item as selected

6. [CURATED TABLE: ROW "Glulam GL24h", COUNT COLUMN, CHANGE "1" → "3"]
   Fire: curationSetCount {objectId: "beam-glulam-gl24h", value: 3}  OR  {delta: 2}
   Expect:
     - Curated count cell now shows "3"
     - Pool's Curated Count column for this row now shows "3"

7. [CURATED TABLE: ROW "Glulam GL24h", CLICK "×" BUTTON]
   Fire: curationRemove {objectId: "beam-glulam-gl24h"}
   Expect:
     - Row disappears from Curated table
     - Curated Count in Pool resets to "0"
     - Preview reverts to "No selection"
```

### Part C: Drag-and-Drop Between Pool and Curated

```
8. [POOL TABLE: DRAG "KVH C24 100×200" row to Curated pane]
   Fire: dropOnCurated {objectId: "beam-kvh-c24"}
   Expect:
     - "KVH C24 100×200" appears in Curated with count=1
     - Pool Curated Count column shows "1"

9. [CURATED TABLE: DRAG "KVH C24" row back to Pool pane]
   Fire: dropOnPool {objectId: "beam-kvh-c24"}
   Expect:
     - Row disappears from Curated
     - Pool Curated Count resets to "0"
```

### Part D: Filtering by Query

```
10. [POOL TABLE: FIND text input labeled "Search" OR "Filter Query"]
    [TYPE "concrete"]
    Fire: setFilterQuery {value: "concrete"}
    Expect:
      - Pool table now shows only 1 row: "Concrete Slab 240mm"
      - Grid shows only that 1 slab item on the 2m grid
      - Query input displays "concrete"
      - All other modules/typologies remain in memory but hidden

11. [CLEAR THE SEARCH FIELD]
    Fire: setFilterQuery {value: ""}
    Expect: All 10 items reappear in Pool and Grid
```

### Part E: Filtering by Module

```
12. [POOL PANE: FIND checkboxes "beams", "windows", "slabs" (or dropdown)]
    [UNCHECK OR DESELECT "windows"]
    Fire: setFilterModule {moduleId: "windows", enabled: false}
    Expect:
      - Pool table now shows 7 items (4 beams + 3 slabs, no windows)
      - Grid renders 7 items
      - Windows checkbox unchecked
      - Other modules stay checked

13. [UNCHECK "slabs"]
    Fire: setFilterModule {moduleId: "slabs", enabled: false}
    Expect:
      - Pool shows only 4 beams
      - Grid shows only 4 beams
      - Slabs checkbox unchecked

14. [CHECK "windows" AGAIN]
    Fire: setFilterModule {moduleId: "windows", enabled: true}
    Expect:
      - Pool shows 4 beams + 3 windows (7 items)
      - Grid updates to 7 items
      - Windows checkbox checked

15. [CHECK "slabs"]
    Fire: setFilterModule {moduleId: "slabs", enabled: true}
    Expect: Back to all 10 items
```

### Part F: Filtering by Typology (Tree Navigation)

```
16. [POOL PANE: FIND typology tree/selector (hierarchical, e.g., "beams > solid-timber")]
    [SELECT/EXPAND "beams"]
    Fire: setFilterTypology {path: "beams"}  OR event carries path from tree selection
    Expect:
      - Pool shows only 4 beams
      - Grid shows only 4 beams
      - Typology selector shows "beams" highlighted or expanded

17. [DRILL DOWN: EXPAND "solid-timber" under "beams"]
    Fire: setFilterTypology {path: "beams/solid-timber"}
    Expect:
      - Pool shows 2 items: "Glulam GL24h" and "KVH C24"
      - Grid shows same 2 items
      - Selector highlights "solid-timber" branch

18. [DRILL DOWN FURTHER: SELECT "glulam"]
    Fire: setFilterTypology {path: "beams/solid-timber/glulam"}
    Expect:
      - Pool shows only 1: "Glulam GL24h"
      - Grid shows only 1 item

19. [CLEAR TYPOLOGY FILTER (click "Reset" or deselect "beams")]
    Fire: setFilterTypology {path: ""}
    Expect: Back to all 10 items
```

### Part G: Filtering by Minimum Availability

```
20. [POOL PANE: FIND slider or number input labeled "Min. Availability"]
    [DRAG SLIDER TO 20 OR ENTER "20"]
    Fire: setFilterMinAvailability {value: 20}
    Expect:
      - Pool filters to items with availability ≥ 20:
        beam-kvh-c24 (60), slab-concrete-240 (30), slab-clt-160 (20)
      - That's 3 items visible
      - Grid shows only those 3
      - Slider/input shows "20"

21. [INCREASE TO 30]
    Fire: setFilterMinAvailability {value: 30}
    Expect:
      - Only 2 items now: kvh-c24 (60), concrete-240 (30)
      - Grid updates to 2 items

22. [RESET TO 0]
    Fire: setFilterMinAvailability {value: 0}
    Expect: All 10 items reappear
```

### Part H: Sorting the Pool Table

```
23. [POOL TABLE: CLICK HEADER "Availability"]
    Fire: sortTable {columnId: "availability", direction: "asc"}
    Expect:
      - Pool rows now sorted by availability ascending: 8, 10, 12, 14, 16, 18, 20, 24, 30, 60

24. [POOL TABLE: CLICK "Availability" HEADER AGAIN]
    Fire: sortTable {columnId: "availability", direction: "desc"}
    Expect:
      - Rows now descending: 60, 30, 24, 20, 18, 16, 14, 12, 10, 8

25. [POOL TABLE: CLICK HEADER "Name"]
    Fire: sortTable {columnId: "name", direction: "asc"}  (or "name" is default)
    Expect:
      - Rows sorted alphabetically by name: Casement Window... < Concrete Slab... < Fixed Window... < etc.
      - Clicking again reverses to desc
```

### Part I: Localization (Locale Selection)

```
26. [FIND LOCALE/LANGUAGE SELECTOR (dropdown, flag, or settings)]
    [SELECT "Deutsch" / "de-DE"]
    Fire: setLocale {value: "de-DE"}
    Expect:
      - Column headers change to German:
        "Name" → (unchanged)
        "Module" → (unchanged, but might be "Modul")
        "Availability" → "Verfügbarkeit"
        "Curated" / "Count" → "Kuratiert" / "Anzahl"
      - Window labels change:
        "Pool" → "Pool" (same)
        "Curated" → "Kuratiert"
        "Preview" → "Vorschau"
        "Grid" → "Raster"
      - All UI labels rerender in German

27. [SELECT ENGLISH / "en-US" AGAIN]
    Fire: setLocale {value: "en-US"}
    Expect: All labels back to English
```

### Part J: History (Undo / Redo)

```
28. [PERFORM A SERIES OF EDITS: e.g., add 3 items to Curated with counts]
    Example:
    - curationAdd {beam-glulam-gl24h}
    - curationSetCount {beam-glulam-gl24h, value: 5}
    - curationAdd {window-casement-100x120}
    - curationSetCount {window-casement-100x120, value: 2}
    Expect:
      - Curated table shows both items with their counts

29. [USE UNDO AFFORDANCE: keyboard Ctrl+Z OR click "Undo" button in History panel]
    Fire: undo (framework-issued, not a sourcing-curation command directly)
    Expect:
      - Last edit (setCount window to 2) is reversed
      - Curated shows window with count=1 (or disappears if it was the add)

30. [USE REDO: keyboard Ctrl+Shift+Z OR click "Redo" button]
    Fire: redo (framework-issued)
    Expect:
      - Last undo is reversed; back to previous state

Note: Undo/redo are surfaced via framework.panel.history or framework.body.history contributions,
not by the sourcing commands directly. The commands themselves emit Emit<SourcingMutation, ...>
which the framework's history engine tracks.
```

---

## V. Error Handling

### Error Surface

**Mutation Outcome Errors:** Errors are emitted as `MutationOutcome::error(code, message, paths)` from the mutation dispatch layer.

Example error (from `curation_remove`): 
```
MutationOutcome::error("mutation.target-missing", 
  "\"beam-glulam-gl24h\" is not curated.", 
  [object_id])
```
**Source:** `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-curated-item/🔺️diff/🦀️.rs`

### Error Display

Errors are captured by the framework's error boundary (likely surfaced as:
- Toast notification / banner in the UI
- Error panel in the sidebar (linked to `framework.panel.history` or a dedicated error log)
- Red highlighting on the affected row/field
- Console error (browser DevTools)

### Common Errors to Test

| Scenario | Expected Error |
|---|---|
| Try to remove an item that is not in Curated | `mutation.target-missing: "X is not curated."` |
| Try to add an item with invalid objectId | Command dispatch fails; no mutation emitted |
| Try to set count with delta/value both missing | Coerced to 0 or rejected by type validation |

---

## VI. Localization

### Locale Resolution

**Source:** `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs` (line specifying default)

- **Default Locale:** `"en-US"` (no language selection required at startup)
- **Supported Locales:** At minimum, English ("en-US") and German ("de-DE")
- **Selection Mechanism:** `setLocale` command (user action on selector) → config mutation → re-render with new labels
- **Label Source:** `sourcing_curation_labels(&config)` function in `terminology/🦀️.rs`; resolves BCP-47 locale tag to `SourcingLabels` struct containing all on-screen strings

### Localization Constraints (from CLAUDE.md)

- **No default language:** The app mandates multiple languages with no default language (though code shows "en-US" as the hard-coded config default).
- **Bi-lingual support:** English (en) + German (de).
- **Test Notes:** Tester can switch languages at any time via `setLocale`; the UI re-renders synchronously.

---

## VII. Summary: 14 Commands at a Glance

1. **setActiveExample** → Example selector (Demo ↔ Empty)
2. **setDocument** → File import / paste JSON document
3. **stockFromCatalogue** → Reset from catalogue (button)
4. **curationAdd** → Pool: click "+" or drag → Curated
5. **curationSetCount** → Curated: edit count cell
6. **curationRemove** → Curated: click "×" button
7. **dropOnPool** → Drag Curated → Pool pane
8. **dropOnCurated** → Drag Pool → Curated pane
9. **setFilterQuery** → Pool: search text input
10. **setFilterModule** → Pool: module checkboxes (beams, windows, slabs)
11. **setFilterTypology** → Pool: tree drill-down (e.g., beams > solid-timber)
12. **setFilterMinAvailability** → Pool: slider / number input (≥ threshold)
13. **sortTable** → Pool: click column header (asc/desc)
14. **setContributions** → Advanced: module metadata JSON (rare)

**Note on setLocale:** Reachable via UI but NOT in the 14 bounded commands list; controlled separately by the config system and framework locale infrastructure.

---

## End-to-End Coverage Checklist

- [x] All 4 windows render and update on interaction
- [x] All 14 commands execute with correct args
- [x] Demo/Empty example switching works
- [x] Add, edit, remove curation items
- [x] Drag-drop Pool ↔ Curated
- [x] Filter by query (text search)
- [x] Filter by module (checkboxes)
- [x] Filter by typology (tree)
- [x] Filter by min availability (slider)
- [x] Sort table by column header
- [x] Localization (English / German)
- [x] Undo / Redo (via framework history)
- [x] Error cases (missing target, invalid input)

---

**Test Environment:** Chrome / Safari / Firefox at http://127.0.0.1:6081

**Test Duration:** ~20–30 minutes for full coverage.
