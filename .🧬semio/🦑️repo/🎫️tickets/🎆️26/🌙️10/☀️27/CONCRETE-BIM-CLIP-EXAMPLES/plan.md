# Concrete BIM Clip Examples

## Request

The six BIM clips (`🎓️teaching/🏛️architecture/🏗️bim/🎬️clip`) teach on one abstract 6 × 3 × 0.30 m box `WALL-01` drawn as an outlined cuboid, with floating labels ("thickness = 0.30 m", "material · fire resistance") and a 2 × 2 grid standing for an FEM model. The owner wants every clip to carry a proper architectural example — the **Villa Savoye** — so that nothing is abstract and every value on screen is shown at the same time by its meaning on that building. Rendering and concept depth must match the energy clips (`🎓️teaching/🏛️architecture/⚡️energy/**/🎬️clip`).

Baseline still: `T/🗑️generated/frames/Beat1_DataToInformation/` (bare cuboid + text).

## One Building For All Six Clips

**Villa Savoye, Poissy (Le Corbusier and Pierre Jeanneret, 1928–1931)**, as a simplified, rounded *teaching model* (said once per clip). Values come from `T/research-villa-savoye.md` (section "Recommended teaching values"); IFC vocabulary from `T/research-ifc.md`. The model is one schema-first JSON (`C/🧱️model/🔣️.json` + `🧬️schema`) read by a stdlib model (`C/🧱️model/🐍️.py`); every drawing and every number is derived from it.

The model holds a *selection* of the real building that is enough to draw recognisable plans, sections, elevations and an axonometric and to carry each lesson:

- spatial structure: `IfcProject` → `IfcSite` (Poissy) → `IfcBuilding` (Villa Savoye) → `IfcBuildingStorey` ground floor / first floor / roof garden, with elevations;
- structural grid (axes and spacing), every piloti on it (`IfcColumn` occurrences of one `IfcColumnType` "Piloti"), first-floor slab and roof slab (`IfcSlab` FLOOR / ROOF), the ramp (`IfcRamp`);
- the first-floor envelope walls (`IfcWall`) with their ribbon windows (`IfcWindow` in `IfcOpeningElement`, `IfcRelVoidsElement` + `IfcRelFillsElement`), the curved ground-floor glass wall whose radius follows the car's turning path, the curved roof-garden screen walls;
- services: a heating boiler and radiators, rainwater drains from the roof garden (`IfcFlowTerminal`, `IfcPipeSegment` … names from `T/research-ifc.md`);
- three discipline models (architecture, structure, services) on the same schema, one analytical translation (structural analysis model) with explicit assumptions and losses, revisions and links.

## Protagonists Per Clip

| Clip | Scene class | Protagonist and story | Values shown by their meaning |
| --- | --- | --- | --- |
| 📊️information | `Beat1_DataToInformation` | the grid spacing between two pilotis on the ground floor; question: can the Savoyes' car drive in between the pilotis and turn under the house along the curved glass wall? | raw number → dimension line between two pilotis in the ground-floor plan; clear width (spacing − piloti diameter) hatched with the car's swept path passing through; turning radius drawn as the curved glass wall; properties (material, LoadBearing, FireRating) attached to the highlighted piloti; relationships drawn as lines to storey, slab and grid |
| 🧬️objects | `Beat2_ObjectOrientation` | IFC class tree beside an axonometric of the villa; one piloti instance among all occurrences of the piloti type; the ribbon-window wall for has-a vs is-a | instance count lights all pilotis; diameter and height as dimension lines on a piloti; volume as the concrete filling the cylinder and the total concrete of all pilotis; inherited attributes (GlobalId, Name, ObjectPlacement) slide down the tree; window fills an opening in the wall (not a kind of wall) |
| 👁️representations | `Beat3_SameObject` | one first-floor façade wall with its ribbon window in plan, section, elevation and axonometric, plus IFC representation identifiers (Axis, Body, FootPrint …) | wall length / height / sill / window height as dimension lines in the matching views; wall area and window area as hatches with the glazing share; a design event (window sill lowered / window taller) regenerates all four views together while the GlobalId chip stays |
| 🧩️models | `Beat4_PartialModels` | architecture, structure and services models of the villa on the same IFC schema; federation; translation to a structural analysis model | each model as its own coloured axonometric layer of the same villa with its object counts; federation overlays them by GlobalId on shared coordinates; the slab becomes surface members on piloti point supports; lost information listed with the element it came from |
| 🔌️exchange | `Beat5_Interfaces` | exchanging one piloti and the first-floor slab: IFC STEP lines vs a triangle mesh vs a structural analysis model | STEP excerpt with the GlobalId / class / property tokens linked to the drawn piloti; mesh export as the actual triangulated cylinder with its triangle count and no class/property left; slab idealized at mid-surface; tributary area per piloti hatched in plan; area load and piloti reaction as arrows sized by value |
| 🔗️consistency | `Beat6_Consistency` | a roof-garden build-up change (new waterproofing and soil) raises the roof load; identity stays, derived values follow, the linked structural analysis still uses the old revision | GlobalId chip constant through the change; build-up thickness animated in section; load per m² and piloti reaction as growing arrows; revision badges on each model; the stale link turns orange on the analysis model with old vs new values; resolution re-translates and the arrows match again |

## Visual Principle

Every screen pairs **the concept** (class tree, property table, STEP text, model cards, revision badges) with **the villa drawing** that shows what the concept refers to at the same time. A highlighted IFC token is linked by a leader line or matching colour to the drawn element. No value appears without its object and unit; every value is drawn (dimension line, hatch, filled volume, arrow length, count of lit elements). Moving values use shared `ValueTracker`s and `always_redraw`. Drawings are architectural: plan cuts with poché, section cuts with hatch, elevation lines with ground line, axonometric with shaded faces, scale bar or dimensions. Styling, fonts, colours, captions and chrome come from the shared energy authoring toolkit.

## File Ownership

| Owner | Files |
| --- | --- |
| model agent | `C/🧱️model/🐍️.py`, `C/🧱️model/🔣️.json`, `C/🧱️model/🧬️schema/🔣️.schema.json`, `C/🧪️cases/🔣️.json`, `C/🧪️tests/🐍️.py`, `C/🧪️tests/🟨️.cjs`, `C/🧪️tests/🟨️metadata.mjs` (only if it must change) |
| toolkit agent | `C/🎨️visuals/🐍️.py` (ClipScene base, shared helpers; topic `_information` … `_consistency` bodies removed), `C/🎨️visuals/🏛️savoye/🐍️.py` (villa drawing widgets), `C/🎨️visuals/🧾️panels/🐍️.py` (class boxes, property tables, STEP panel, model cards, badges) |
| scene agents (one each) | `C/📊️information/🐍️.py`, `C/🧬️objects/🐍️.py`, `C/👁️representations/🐍️.py`, `C/🧩️models/🐍️.py`, `C/🔌️exchange/🐍️.py`, `C/🔗️consistency/🐍️.py` — each owns its full `present_<topic>` implementation inside its scene class and its bilingual `NARRATION` |
| coordinator | `C/README.md`, `T/*`, rendering, delivery, ticket |

Scene class names, video ids, standalone wrappers (`🏗️bim/<topic>/🎬️clip/🐍️.py`) and curriculum registrations stay unchanged.

## Phases

1. Research and audits (Haiku, read-only): `T/research-villa-savoye.md`, `T/research-ifc.md`, `T/audit-current.md`, `T/audit-toolkit.md`.
2. Contract: `T/api.md` (model functions, JSON shape, widget signatures).
3. Parallel build (Sonnet): model + tests, toolkit, six storyboards `T/storyboard-<topic>.md`.
4. Parallel scenes (Sonnet): six scene implementations with keyframe captures in both languages.
5. Audit (Haiku, read-only): frame-by-frame review against the quality bar, IFC/historical fact check of every narration clause.
6. Fixes, full tests, final renders (en + de, speaker), delivery, ticket close.
