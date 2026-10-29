# Grasshopper and Excel Nakagin Clips

## Request

Two new full video series, built and rendered with the same pipeline and quality bar as the BIM series (`🎓️teaching/🏛️architecture/🏗️bim/🎬️clip`) and the energy series. Both series use **one concrete example**, the **Nakagin Capsule Tower** (Kisho Kurokawa, Ginza, Tokyo, 1970–1972, demolished 2022), as a simplified, rounded *teaching model* (said once per clip).

- **Grasshopper**: one capsule is modelled parametrically, the capsule types are placed on planes around the cores, the core and the capital (roof head) are modelled parametrically, and the lower building is only a simple massing; anatomy (script, component, param, input, slider, panel, plugin, tree, branch …); casting; data trees on the aggregated tower `{BUILDING;TOWER;STORY;FACADE}` with two capsule items per facade branch.
- **Excel**: the Nakagin room program and the energy calculation, with **one worksheet per capsule type**; anatomy (workbook, worksheet, cell, reference, formula …).

"Fullblown, visual, didactically rich": every concept is shown at the same moment on the Nakagin drawing or on the canvas/workbook that computes it; no abstract boxes; no number without its object and unit; values change live and the drawing follows.

T = this ticket folder. Facts: `T/research-nakagin.md`, `T/research-grasshopper.md`, `T/research-excel.md`. Toolkit contract: `T/audit-toolkit.md`, `T/api.md`. Registration touchpoints: `T/audit-registration.md`. Agent rules: `T/agent-rules.md`.

## Folder Layout

```
🎓️teaching/🏛️architecture/
  🏢️nakagin/                     shared teaching model of the building (domain-specific example, used by both series)
    🔣️.json  🧬️schema/🔣️.schema.json  🐍️.py  🧪️cases/🔣️.json  🧪️tests/🐍️.py  🧪️tests/🟨️.cjs  README.md
    🎨️visuals/🐍️.py              Manim drawings of capsule, core, capital, podium, tower (axonometric + plan + elevation)
  🦗️grasshopper/
    🎬️clip/  README.md  🐍️.py (playlist)  🗣️transcript.{en,de}.md
      🧱️model/  🐍️.py 🔣️.json 🧬️schema/   stdlib Grasshopper semantics: data trees, params, casting, components, the Nakagin definitions
      🎨️visuals/🐍️.py               ClipScene base
      🎨️visuals/🦗️canvas/🐍️.py      canvas widgets: component, param, slider, panel, toggle, value list, wire, group, ribbon tabs, param viewer, Rhino viewport frame
      🧪️cases/🔣️.json  🧪️tests/🐍️.py  🧪️tests/🟨️.cjs  🧪️tests/🎭️presentation/
      🦴️anatomy/🐍️.py  💊️capsule/🐍️.py  🗼️tower/🐍️.py  🫗️casting/🐍️.py  🌳️trees/🐍️.py
    🦴️anatomy/🎬️clip/🐍️.py … (standalone entries, one per video, same as 🏗️bim/<topic>/🎬️clip/🐍️.py)
  📊️excel/
    🎬️clip/  README.md  🐍️.py  🗣️transcript.{en,de}.md
      🧱️model/  🐍️.py 🔣️.json 🧬️schema/   stdlib spreadsheet semantics: A1 references, copy translation, formula parser/evaluator, dependencies, en/de formula text; the Nakagin workbook
      🎨️visuals/🐍️.py               ClipScene base
      🎨️visuals/📗️workbook/🐍️.py    workbook widgets: window, title, ribbon strip, name box, formula bar, grid, sheet tabs, reference colour frames, fill handle, trace arrows, error tags, chart
      🧪️cases/🔣️.json  🧪️tests/🐍️.py  🧪️tests/🟨️.cjs  🧪️tests/🎭️presentation/
      🦴️anatomy/🐍️.py  📍️references/🐍️.py  📋️program/🐍️.py  ⚡️energy/🐍️.py
    🦴️anatomy/🎬️clip/🐍️.py …
```

## Videos

| Video id | Scene class | Folder | Story (protagonist on the Nakagin) |
| --- | --- | --- | --- |
| grasshopper_anatomy | `Beat1_Anatomy` | 🦴️anatomy | Building the capsule shell on an empty canvas: the script (.gh definition) as a graph; a Number Slider (domain, rounding, value) wired into a Box component; component anatomy (name/nickname, input params left, output params right, wires, preview, states grey/orange/red); a floating param vs an input param; a Panel reading the output with its path `{0}`; a plugin (.gha) adding a tab (e.g. Ladybug sun hours on the capsule window); item → list (a Series of storey heights gives a list of capsules) → tree (branches, paths `{0;0}`, items with indices); German/English term legend |
| grasshopper_capsule | `Beat2_Capsule` | 💊️capsule | One capsule, parametric: width/length/height sliders → box → rounded edges → round window cut on the front face → the four bolt points on the back face; every slider moves the drawing live (dimension lines on the capsule); the capsule types (from `🏢️nakagin`) as variants of the same definition; each type placed on a plane (origin, X, Y, Z axes drawn) with Orient from the capsule's base plane to the target plane; a List Item picks the type per plane |
| grasshopper_tower | `Beat3_Tower` | 🗼️tower | Core parametric (square plan, storey height, storey count, shaft), planes generated per storey and facade on the core, capsule rotations, the capital (roof head) parametric on top of each core, the lower building as a simple extruded massing; both towers assembled; counts (capsules, storeys) from the definition |
| grasshopper_casting | `Beat4_Casting` | 🫗️casting | Each param has a type; mismatched data is cast: Panel text "2.5" → Number width; number → integer storey count (rounding rule); point → plane (world-aligned plane loses the facade orientation: the capsule faces the wrong way, which motivates explicit planes); number → domain; circle → curve; box → brep; integer → boolean (window on/off); failed cast "Type A" → Number (red component, message); a casting matrix built up from what was shown |
| grasshopper_trees | `Beat5_DataTrees` | 🌳️trees | The aggregated tower as a tree `{BUILDING;TOWER;STORY;FACADE}` with two capsule items per branch; Panel and Param Viewer; path anatomy (branch, path, index); Flatten, Graft, Simplify, Path Mapper `{B;T;S;F}(i) → {B;T;S}` (capsules per storey), Flip Matrix, branch matching between a capsule tree and a per-tower list; counts per tower/storey drawn as highlighted capsules on the tower |
| excel_anatomy | `Beat1_Anatomy` | 🦴️anatomy | The Nakagin workbook (.xlsx) with sheets Program, one sheet per capsule type, Climate, Energy; worksheet, column, row, cell address, range, name box, formula bar; constant vs formula vs displayed format (2.3 → "2.30 m"); operators and functions; the capsule floor area as the first formula (and the capsule drawn with the area hatched); recalculation when the width changes; precedents/dependents arrows; typical errors (#DIV/0!, #REF!, #NAME?, #VALUE!); German UI and function names with `;` separators in the German version |
| excel_references | `Beat2_References` | 📍️references | Relative references filled down the capsule-type rows; absolute `$B$1` (interior temperature) ; mixed references in a two-way table; cross-sheet `'Type A'!B3`; 3D reference `SUM('Type A:Type C'!B12)`; defined names; what breaks: wrong relative copy, deleted sheet → #REF!; each reference drawn as a coloured frame and the copied formula shown before/after with the shift arrows |
| excel_program | `Beat3_Program` | 📋️program | The room program: each type sheet holds its fittings and net areas; the Program sheet holds the capsule list (every capsule with tower, storey, facade, type) as a table; COUNTIF/COUNTIFS per type, SUMPRODUCT of counts × areas, XLOOKUP of type values, totals per tower; bar chart of areas; each count lights the matching capsules on the tower drawing |
| excel_energy | `Beat4_Energy` | ⚡️energy | Per type sheet: envelope surfaces from dimensions (walls, roof, floor, window), exposed vs attached to the core, U-values, H_T = Σ U·A, H_V from air change rate and volume, heating load and annual heating demand with Tokyo climate from the Climate sheet; the Energy sheet sums all capsules by type count; a what-if (better window or insulation) updates every dependent cell and the hatched surfaces; kWh/(m²·a) per type compared |

Scene class names are unique per series module. Each video is registered like the BIM ones (registry `⚡️energy/🎬️clip/🔣️.json`, presentation cases, READMEs — see `T/audit-registration.md`).

## Single Source Of Truth

- `🏢️nakagin/🔣️.json` holds every building value (capsule types, dimensions, window, bolts, cores, storeys, facades, planes/rotations, capital, podium, program items, energy inputs, climate), each with a basis (`sourced` with a source key, `assumed`, `derived`). `🏢️nakagin/🐍️.py` derives geometry (capsule solids, planes, tower placement, tree paths), program areas and envelope areas. Both series read numbers only from it (Grasshopper through its definition engine, Excel through its workbook engine).
- The Grasshopper engine evaluates the definitions shown on the canvas; every panel value, count and preview drawn comes from evaluating that definition.
- The Excel engine evaluates the Nakagin workbook; every cell value shown comes from evaluating that workbook; the workbook values are cross-checked against `🏢️nakagin` derived values in tests.

## Validation (test-driven, language-agnostic cases, third-party oracles)

- Each model has portable cases `🧪️cases/🔣️.json` (inputs → expected outputs) and a schema validated with AJV in `🧪️tests/🟨️.cjs`.
- Oracles (test group only, never imported at runtime): `rhino3dm` (planes, transforms, boxes for capsule placement and casting point → plane), `openpyxl` (A1 parsing, `Translator` for references on copy, writing the Nakagin workbook to .xlsx), `formulas` (evaluates that .xlsx and must reproduce every cell value of our engine), NumPy/SciPy for geometry and energy sums. Data-tree operations get an independent oracle implementation in the test file.
- Presentation tests cover every scene like the BIM `🧪️tests/🎭️presentation`.

## Visual Principle

Every screen pairs **the tool** (Grasshopper canvas, Excel workbook window) with **the Nakagin drawing** that shows what the data mean: a slider value is a dimension line on the capsule; a panel item lights the capsule it refers to; a tree branch is a facade of a storey on the tower; a cell is the hatched surface or the counted capsules it computes. Wires and reference frames share the colour of the drawn element. Styling, fonts, captions and chrome come from the shared energy authoring toolkit; the BIM drafting helpers are reused, never modified.

## Fleet And Phases

1. Research and audits (Haiku): research-nakagin, research-grasshopper, research-excel, audit-toolkit, audit-registration.
2. Contract (coordinator): `T/api.md`, `T/agent-rules.md`.
3. Foundations (Sonnet, parallel): NAKAGIN model + drawings; GH engine + definitions; GH canvas toolkit + ClipScene; XL engine + workbook; XL workbook toolkit + ClipScene; registration + entries + READMEs.
4. Storyboards (Sonnet, parallel, one per video) → storyboard audits (Haiku).
5. Scenes (Sonnet, one per video) with keyframes in both languages.
6. Frame and fact audits (Haiku) → fixes → full tests → renders (en/de × speaker/spectator, sequential per language) → ticket close.
