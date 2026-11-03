# 📖️ Terminology: en and de Vocabulary of the Staffage Views

Status: PROPOSAL, docs only, from slot 20 (haiku-docs). Once the coordinator approves it, it is binding for every worker, every language file, every caption, every validator message, every test expectation and every README sentence of the staffage library.
Snapshot: 2026-11-02, read while the viewer, depiction and catalog files were being edited. Line numbers drift; re-grep before you edit.
Library (L): `🎓️teaching/🏛️architecture/🧍️staffage`.

## 1. The rule set

R1. Three viewpoints, one name each:
- `elevation`: **Side elevation** / **Seitenansicht**
- `front`: **Front elevation** / **Vorderansicht**
- `plan`: **Plan (top view)** / **Draufsicht**

R2. The three together: **Three views** / **Drei Ansichten**. The viewpoint selector choice "all" uses this name.

R3. The seven render modes are **representations** / **Darstellungen**. Never "Ansicht" or "view" for a mode. Mode names: Dark line / Dunkle Strichzeichnung, Dark solid / Dunkler Vollton, Light line / Helle Strichzeichnung, Light solid / Heller Vollton, Thermal / Thermisch, Forces / Kräfte, Moments / Momente.

R4. Grundriss is used only for a cut plan section: **Plan section B-B** / **Grundriss-Schnitt B-B**. The plan geometry itself, the plan viewpoint and the plan symbol are **Draufsicht**, never Grundriss.

R5. Front sections read **Section A-A** / **Schnitt A-A** (see open question OQ1).

R6. Never as labels or validator messages: "Side", "Front", "Top" alone; "Seite", "Vorne", "Oben" (the new viewpoints module); "All three" / "Alle drei"; "Stirnansicht" and "Frontansicht" (use Vorderansicht); "Aufriss" (use Seitenansicht or Vorderansicht); "Top view" as a label (only as the parenthesis of Plan (top view)); "elevation" alone for the side view in prose (write side elevation).

R7. "view" in English prose may still mean one of the three viewpoints, as in "top view" in a parenthesis, or "this view" when the context is a viewpoint. It must never mean a mode.

R8. Identifiers do not change with this document: `elevation`, `front`, `plan`, the type `View`, the mode ids `dark-line` and so on, the field `view` of a cast row, the URL keys `view` and `vp`, the flags `--view`, `--viewpoint`, `--views`, and the validator ids such as `elevation-lateral`. Only prose, labels and descriptions change. A rename of an identifier is a schema decision for the coordinator (OQ3).

R9. The viewpoint of a view is always named with its representation in a caption: "Dark line · Front elevation", never a caption that names only one of them. A side elevation caption must name its viewpoint too (see C8).

R10. Raw ids never reach user text. A label is looked up in the table of section 2, in the language asked for, never printed as `front`, `dark-line` or `elevation`.

## 2. Vocabulary table

| Concept | Identifier | en label | de label | Notes |
|---|---|---|---|---|
| side viewpoint | `elevation` | Side elevation | Seitenansicht | rig plane; the side view of the figure |
| front viewpoint | `front` | Front elevation | Vorderansicht | the forward view |
| plan viewpoint | `plan` | Plan (top view) | Draufsicht | seen from above |
| the three viewpoints | (set) | Three views | Drei Ansichten | selector choice "all" |
| the choice of viewpoint | `Viewpoint` | Viewpoint | Blickrichtung | keep the existing i18n label |
| render mode | `dark-line`, `dark-solid`, `light-line`, `light-solid`, `thermal`, `forces`, `moments` | representation (see R3) | Darstellung | the seven of R3 |
| front cut | `sections.front` | Section A-A | Schnitt A-A | OQ1 |
| plan cut | `sections.plan` | Plan section B-B | Grundriss-Schnitt B-B | R4 |
| cut-line marker | (drawing) | Section A-A | Schnitt A-A | the letters of the cut |
| member cross-section | (not a view) | Section | Querschnitt | `detail.col.section`; stays, it is not a view |
| sheet of three views | (sheet) | Three views of {title} | Drei Ansichten von {title} | the sheet title |

Caption grammar: `{representation} · {viewpoint}`, for example "Dark line · Front elevation" and "Dunkle Strichzeichnung · Draufsicht". A plan cut reads "Dark line · Plan section B-B" and "Dunkle Strichzeichnung · Grundriss-Schnitt B-B".

## 3. Open questions for the coordinator

- OQ1. Front cut label. Proposed: "Section A-A" / "Schnitt A-A" (plain), so only the plan cut carries its name. Alternative: "Front section A-A" / "Vorderansichts-Schnitt A-A", which is clumsier. Decide once.
- OQ2. The frame sentence is settled by the coordinator as D6 (x forward, y up, z left; the left-handed wording is corrected). Owners use D6's text, not the brief's "right-handed" sentence.
- OQ3. Identifier renames: the type `View`, the field `view` in cast rows and metadata, the i18n keys `viewer.view*`, `playground.view*`, `compare.view`. Proposed: keep for now, rename in a later schema change.
- OQ4. "Focus view" / "Fokusansicht" (viewpoints module, WORDS `focus`) names a pane, not a viewpoint or a mode. Proposed: "Focused pane" / "Fokussiertes Feld".
- OQ5. Compare page wording (`compare.*`): proposed text is in section 4 (C-rows), pending approval.

## 4. Deviations: file:line, current text, replacement, owner

Owners: viewer = sonnet-viewer; contracts = sonnet-contracts; exports = sonnet-exports; structural = sonnet-structural; validation = sonnet-validation; course = sonnet-course; category workers = the owner named on the board for `🗂️catalog/<category>/`.

### 4.1 i18n, English (`L/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🔠️english/`), owner viewer

| ID | File:line | Current | Replacement |
|---|---|---|---|
| E1 | `🎛️viewer.ts:5` `viewer.view` | "View" | "Representation" |
| E2 | `🎛️viewer.ts:7` `viewer.viewpoint.elevation` | "Side" | "Side elevation" |
| E3 | `🎛️viewer.ts:8` `viewer.viewpoint.front` | "Front" | "Front elevation" |
| E4 | `🎛️viewer.ts:9` `viewer.viewpoint.plan` | "Top" | "Plan (top view)" |
| E5 | `🎛️viewer.ts:20` `viewer.background.auto` | "By view" | "By representation" |
| E6 | `🎛️viewer.ts:29` `viewer.contrast` | "This ground and this view have little contrast; choose \"By view\" or another view." | "This ground and this representation have little contrast; choose \"By representation\" or another representation." |
| E7 | `🎛️viewer.ts:44` `viewer.analyticsOnly` | "...apply to the thermal, force and moment views." | "...apply to the thermal, force and moment representations." |
| E8 | `🎛️viewer.ts:73` `viewer.environmentHelp` | "Changes the thermal view live; ..." | "Changes the thermal representation live; ..." |
| E9 | `🎛️viewer.ts:74` `viewer.environmentOnlyThermal` | "Only the thermal view reads the environment; ..." | "Only the thermal representation reads the environment; ..." |
| E10 | `🎛️viewer.ts:83` `viewer.inspectorNone` | "This view carries no analytic values; ..." | "This representation carries no analytic values; ..." |
| E11 | `🎛️viewer.ts:87` `viewer.legendsNone` | "This view needs no legend: ..." | "This representation needs no legend: ..." |
| E12 | `🎛️viewer.ts:89` `detail.compare` | "Compare views" | "Compare representations and viewpoints" (OQ5) |
| E13 | `🎛️viewer.ts:93` `detail.shareDone` | "The link to this exact view is on the clipboard." | "The link to this exact representation and viewpoint is on the clipboard." |
| E14 | `🧪️playground.ts:14` `playground.view` | "View" | "Representation" |
| E15 | `🧪️playground.ts:16` `playground.background.paper` | "Paper (follows view)" | "Paper (follows representation)" |
| E16 | `🧪️playground.ts:20-26` `playground.view.*` | "Dark line", "Dark solid", "Light line", "Light solid", "Thermal", "Forces", "Moments" | No change. These are the reference mode names of R3. |
| E17 | `🧪️playground.ts:32` `playground.contrast.body` | "This view draws light ink on dark or dark ink on light, ... Choose Paper to follow the view." | "This representation draws light ink on dark or dark ink on light, ... Choose Paper to follow the representation." |
| E18 | `⚖️compare.ts:4` `compare.lede` | "...or one asset in other views, styles and analytic states." | "...or one asset in other viewpoints, representations and analytic states." |
| E19 | `⚖️compare.ts:9` `compare.sameAsset` | "Same asset, other views" | "Same asset, other viewpoints or representations" |
| E20 | `⚖️compare.ts:10` `compare.sameView` | "Other assets, same view" | "Other assets, same representation and viewpoint" |
| E21 | `⚖️compare.ts:17` `compare.view` | "View" | "Representation" |
| E22 | `⚖️compare.ts:23` `compare.inspectHint` | "...of an analytic view to read its value." | "...of an analytic representation to read its value." |
| E23 | `📖️detail.ts:84` `detail.thermal.unsolved` | "Open the thermal view to see solved values." | "Open the thermal representation to see solved values." |
| E24 | `📖️detail.ts:173` `detail.structure.unsolved` | "Open the force or moment view to see solved reactions." | "Open the force or moment representation to see solved reactions." |
| E25 | `📖️detail.ts:191` `detail.export.lede` | "...the {view} view and scale 1:{scale}..." | "...the {representation} representation, the {viewpoint} viewpoint and scale 1:{scale}..." The code must pass the localised names, not the raw id (see O6). |
| E26 | `📖️detail.ts:200` `detail.export.sheetSingle` | "This view, one page" | "This representation, one page" |
| E27 | `📖️detail.ts:201` `detail.export.sheetAll` | "All seven views, one page each" | "All seven representations, one page each" |
| E28 | `📖️detail.ts:218` `detail.format.pdf.body` | "...or all seven views." | "...or all seven representations." |
| E29 | `🧱️core.ts:27` `hero.lede` | "...draws in seven synchronized views." | "...draws in seven synchronized representations." |

### 4.2 i18n, German (`L/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🔡️german/`), owner viewer

Row labels T1 to T26 in this section are local row labels, not decisions. The test rows of section 4.8 are labelled TS1 to TS11 so that they do not collide with them.

| ID | File:line | Current | Replacement |
|---|---|---|---|
| T1 | `🎛️viewer.ts:7` `viewer.view` | "Ansicht" | "Darstellung" |
| T2 | `🎛️viewer.ts:22` `viewer.background.auto` | "Je Ansicht" | "Je Darstellung" |
| T3 | `🎛️viewer.ts:27` `viewer.theme` | "Thema der Analyseansichten" | "Thema der Analysedarstellungen" |
| T4 | `🎛️viewer.ts:31` `viewer.contrast` | "Dieser Untergrund und diese Ansicht haben wenig Kontrast; wählen Sie „Je Ansicht“ oder eine andere Ansicht." | "Dieser Untergrund und diese Darstellung haben wenig Kontrast; wählen Sie „Je Darstellung“ oder eine andere Darstellung." |
| T5 | `🎛️viewer.ts:46` `viewer.analyticsOnly` | "Diese Optionen gelten für die Thermik-, Kraft- und Momentenansicht." | "Diese Optionen gelten für die Darstellungen Thermik, Kräfte und Momente." |
| T6 | `🎛️viewer.ts:75` `viewer.environmentHelp` | "Ändert die Thermikansicht live; ..." | "Ändert die thermische Darstellung live; ..." |
| T7 | `🎛️viewer.ts:76` `viewer.environmentOnlyThermal` | "Nur die Thermikansicht liest die Umgebung; ..." | "Nur die thermische Darstellung liest die Umgebung; ..." |
| T8 | `🎛️viewer.ts:85` `viewer.inspectorNone` | "Diese Ansicht trägt keine Analysewerte; ..." | "Diese Darstellung trägt keine Analysewerte; ..." |
| T9 | `🎛️viewer.ts:89` `viewer.legendsNone` | "Diese Ansicht braucht keine Legende: ..." | "Diese Darstellung braucht keine Legende: ..." |
| T10 | `🎛️viewer.ts:91` `detail.compare` | "Ansichten vergleichen" | "Darstellungen und Blickrichtungen vergleichen" (OQ5) |
| T11 | `🎛️viewer.ts:95` `detail.shareDone` | "Der Link auf genau diese Ansicht liegt in der Zwischenablage." | "Der Link auf genau diese Darstellung und Blickrichtung liegt in der Zwischenablage." |
| T12 | `🧪️playground.ts:16` `playground.view` | "Ansicht" | "Darstellung" |
| T13 | `🧪️playground.ts:18` `playground.background.paper` | "Papier (folgt der Ansicht)" | "Papier (folgt der Darstellung)" |
| T14 | `🧪️playground.ts:34` `playground.contrast.body` | "Diese Ansicht zeichnet helle Linien ..." | "Diese Darstellung zeichnet helle Linien ..." |
| T15 | `⚖️compare.ts:6` `compare.lede` | "...oder ein Asset in anderen Ansichten, Stilen und Analysezuständen." | "...oder ein Asset in anderen Blickrichtungen, Darstellungen und Analysezuständen." |
| T16 | `⚖️compare.ts:11` `compare.sameAsset` | "Gleiches Asset, andere Ansichten" | "Gleiches Asset, andere Blickrichtungen oder Darstellungen" |
| T17 | `⚖️compare.ts:12` `compare.sameView` | "Andere Assets, gleiche Ansicht" | "Andere Assets, gleiche Darstellung und Blickrichtung" |
| T18 | `⚖️compare.ts:19` `compare.view` | "Ansicht" | "Darstellung" |
| T19 | `⚖️compare.ts:25` `compare.inspectHint` | "...einer Analyseansicht..." | "...einer Analysedarstellung..." |
| T20 | `📖️detail.ts:86` `detail.thermal.unsolved` | "Öffnen Sie die thermische Ansicht, ..." | "Öffnen Sie die thermische Darstellung, ..." |
| T21 | `📖️detail.ts:175` `detail.structure.unsolved` | "Öffnen Sie die Kraft- oder Momentenansicht, ..." | "Öffnen Sie die Kraft- oder Momentendarstellung, ..." |
| T22 | `📖️detail.ts:193` `detail.export.lede` | "...die Ansicht {view} und den Maßstab 1:{scale}..." | "...die Darstellung {representation}, die Blickrichtung {viewpoint} und den Maßstab 1:{scale}..." (see O6) |
| T23 | `📖️detail.ts:202` `detail.export.sheetSingle` | "Diese Ansicht, eine Seite" | "Diese Darstellung, eine Seite" |
| T24 | `📖️detail.ts:203` `detail.export.sheetAll` | "Alle sieben Ansichten, je eine Seite" | "Alle sieben Darstellungen, je eine Seite" |
| T25 | `📖️detail.ts:220` `detail.format.pdf.body` | "...oder alle sieben Ansichten." | "...oder alle sieben Darstellungen." |
| T26 | `🧱️core.ts:29` `hero.lede` | "...in sieben synchronen Ansichten." | "...in sieben synchronen Darstellungen." |

### 4.3 Viewpoint names in code (owner viewer, `L/🎯️targets/⚛️react/🔨️modules/🔭️viewpoints/🟦️.ts`)

The file was changing during the read. The WORDS block (`label`, `help`, `sheet`, `sheetHelp`, `sheetScale`, `unavailable`, `nothingInstead`) already uses the R1 and R2 names; the list below is what still deviates at the snapshot.

| ID | File:line | Current | Replacement |
|---|---|---|---|
| V1 | `🔭️viewpoints/🟦️.ts:22` `VIEWPOINT_NAMES.elevation` | `{ en: "Side", de: "Seite" }` | `{ en: "Side elevation", de: "Seitenansicht" }` |
| V2 | `🔭️viewpoints/🟦️.ts:23` `VIEWPOINT_NAMES.front` | `{ en: "Front", de: "Vorne" }` | `{ en: "Front elevation", de: "Vorderansicht" }` |
| V3 | `🔭️viewpoints/🟦️.ts:24` `VIEWPOINT_NAMES.plan` | `{ en: "Top", de: "Oben" }` | `{ en: "Plan (top view)", de: "Draufsicht" }` |
| V4 | `🔭️viewpoints/🟦️.ts:25` `VIEWPOINT_NAMES.all` | `{ en: "All three", de: "Alle drei" }` | `{ en: "Three views", de: "Drei Ansichten" }` |
| V5 | `🔭️viewpoints/🟦️.ts:30` `VIEWPOINT_PHRASES.elevation` | `{ en: "side view", de: "Seitenansicht" }` | `{ en: "side elevation", de: "Seitenansicht" }` |
| V6 | `🔭️viewpoints/🟦️.ts:31` `VIEWPOINT_PHRASES.front` | `{ en: "front view", de: "Vorderansicht" }` | `{ en: "front elevation", de: "Vorderansicht" }` |
| V7 | `🔭️viewpoints/🟦️.ts:32` `VIEWPOINT_PHRASES.plan` | `{ en: "top view", de: "Draufsicht" }` | `{ en: "plan (top view)", de: "Draufsicht" }` |
| V8 | `🔭️viewpoints/🟦️.ts:33` `VIEWPOINT_PHRASES.all` | `{ en: "side, front and top view together", de: "Seiten-, Vorder- und Draufsicht zusammen" }` | `{ en: "three views together", de: "Drei Ansichten zusammen" }` |
| V9 | `🔭️viewpoints/🟦️.ts:65` `MISSING.elevation` | `{ en: "This drawing has no side view.", ... }` | `{ en: "This drawing has no side elevation.", de: "Diese Zeichnung hat keine Seitenansicht." }` |
| V10 | `🔭️viewpoints/🟦️.ts:66` `MISSING.front` | `{ en: "This drawing has no front view.", ... }` | `{ en: "This drawing has no front elevation.", de: "Diese Zeichnung hat keine Vorderansicht." }` |
| V11 | `🔭️viewpoints/🟦️.ts:67` `MISSING.plan` | `{ en: "This drawing has no top view.", de: "Diese Zeichnung hat keine Draufsicht." }` | `{ en: "This drawing has no plan (top view).", de: "Diese Zeichnung hat keine Draufsicht." }` |
| V12 | `🔭️viewpoints/🟦️.ts:45` `WORDS.focus` | `{ en: "Focus view", de: "Fokusansicht" }` | OQ4: "Focused pane" / "Fokussiertes Feld" |
| V13 | `🔭️viewpoints/🟦️.ts:1,3,4,5,14,17` docstrings | "side, front, top or all three", "The side view is the rig plane", "the top view above the side view", "replaced by another view", "all three on one sheet" | "side elevation, front elevation, plan (top view) or three views"; "The side elevation is the rig plane"; "the plan above the side elevation"; "replaced by another viewpoint"; "three views on one sheet" |
| V14 | `🔭️viewpoints/🟦️.ts:122` `FAILED` | "This view could not be drawn" / "Diese Ansicht konnte nicht gezeichnet werden" | "This representation could not be drawn" / "Diese Darstellung konnte nicht gezeichnet werden" (or name the viewpoint: "This side elevation could not be drawn") |

Line 124 (docstring of `sheetOf`) and line 139 (`draw-failed` message) carry "view" in prose; replace the same way.

### 4.4 Captions (owner contracts, `L/🔨️modules/🎨️depiction/`)

| ID | File:line | Current | Replacement |
|---|---|---|---|
| C1 | `✂️section/🟦️.ts:28` `CAPTIONS` | `elevation: tx("Side view", "Seitenansicht")`, `front: tx("Front view", "Vorderansicht")`, `plan: tx("Top view", "Draufsicht")` | `elevation: tx("Side elevation", "Seitenansicht")`, `front: tx("Front elevation", "Vorderansicht")`, `plan: tx("Plan (top view)", "Draufsicht")` |
| C2 | `✂️section/🟦️.ts:33` `viewpointCaption` section branch | `` `${language === "de" ? "Schnitt" : "Section"} ${section.label[language]}` `` for every viewpoint | Plan cut only: `Plan section` / `Grundriss-Schnitt`; front cut: `Section` / `Schnitt`. For example a table `SECTION_NAMES` with `front: { en: "Section", de: "Schnitt" }`, `plan: { en: "Plan section", de: "Grundriss-Schnitt" }`, then `` `${SECTION_NAMES[viewpoint][language]} ${label}` `` (OQ1) |
| C3 | `✂️section/🟦️.ts:1,30` docstrings | "Section views ... instead of "Front view" / "Vorderansicht"", "else "Side view", "Front view" or "Top view" (Seitenansicht, Vorderansicht, Draufsicht)" | "instead of "Front elevation" / "Vorderansicht""; "else "Side elevation", "Front elevation" or "Plan (top view)" (Seitenansicht, Vorderansicht, Draufsicht)" |
| C4 | `🟦️.ts:54` `VIEW_TITLES["dark-line"]` (depiction) | `tx("Line drawing, dark", "Strichzeichnung, dunkel")` | `tx("Dark line", "Dunkle Strichzeichnung")` (the i18n reference: E16 in English, section 4.2 in German) |
| C5 | `🟦️.ts:55` `VIEW_TITLES["dark-solid"]` | `tx("Solid, dark", "Vollton, dunkel")` | `tx("Dark solid", "Dunkler Vollton")` |
| C6 | `🟦️.ts:56` `VIEW_TITLES["light-line"]` | `tx("Line drawing, light", "Strichzeichnung, hell")` | `tx("Light line", "Helle Strichzeichnung")` |
| C7 | `🟦️.ts:57` `VIEW_TITLES["light-solid"]` | `tx("Solid, light", "Vollton, hell")` | `tx("Light solid", "Heller Vollton")` |
| C8 | `🟦️.ts:192-193` caption | `options.viewpoint === "elevation" ? viewTitle : \`${viewTitle} · ${viewpointCaption(...)}\`` | Always `` `${viewTitle} · ${viewpointCaption(...)}` `` so the side view is named too (see C8) |
| C9 | `🟦️.ts:1,5,6,7,11,141,211` docstrings | "seven synchronized paper-space views", "analytic views", "Structural views", "one of the seven views", "all seven views" | "seven synchronized representations", "analytic representations", "Structural representations", "one of the seven representations", "all seven representations" |
| C10 | `🖨️preview/🟦️.ts:1,6,97` docstrings | "in any of the seven views", "`--viewpoint` draws the side (`elevation`), `front` or top (`plan`) view", "one sheet per view with the side, front and top view", captions "Side view", "Front view", "Section A-A" | "in any of the seven representations", "draws the side elevation (`elevation`), the front elevation (`front`) or the plan (top view, `plan`)", "one sheet per representation with the three views", captions "Side elevation", "Front elevation", "Section A-A" |
| C11 | `🖨️preview/🟦️.ts:~126` `sheet()` column headers | `VIEWS.map((v, c) => text(..., v))`: raw ids such as `dark-line` | The localised representation names of C4 to C7 and the thermal names, language set by the caller (see C11) |

### 4.5 Schema descriptions (owner contracts, `L/🧬️schema/`)

| ID | File:line | Current | Replacement |
|---|---|---|---|
| S1 | `🔣️.json:71-72` `Viewpoint.description` | "...the rig plane (side or section), the orthogonal front elevation, or the plan." | "...the side elevation (the rig plane), the front elevation, or the plan (top view)." The word "section" goes out (see S1). |
| S2 | `🔣️.json:713` `Shape.solid` description | "How a shape of any view (elevation, front or plan) ... the elevation, front and plan of one solid ..." | "How a shape of any viewpoint (side elevation, front elevation or plan) ... the side elevation, the front elevation and the plan of one solid ..." |
| S3 | `🔣️.json:1400` `AuthoredRepresentation.sections.description` | "Which of the two orthogonal views of this representation are drawn as a section instead of an elevation or a plan." | "Which of the two projections, the front elevation or the plan (top view), is drawn as a section." |
| S4 | `🔣️.json:1419` `ViewSection.description` | "A view drawn as a section: ... plan: the height y of a horizontal plane, seen from above) and is named by `label`, such as A-A. The side view shows the cut-line marker at that plane." | "A viewpoint drawn as a section: ... plan: the height y of a horizontal plane, seen from above; the plan section is named Grundriss-Schnitt in German) ... The side elevation shows the cut-line marker at that plane." |
| S5 | `🔣️.json:3243` `widthM` description | "Horizontal extent of the default elevation drawing ..." | "Horizontal extent of the default side elevation drawing ..." |
| S6 | `🔣️.json:3248` `depthM` description | "Extent perpendicular to the elevation plane (the plan's y axis) ..." | "Extent perpendicular to the side elevation plane (the y axis of the plan (top view)) ..." |
| S7 | `🟦️.ts:9` header of the twin | "Model space is a right-handed elevation plane: x to the right, y up ..." | "Model space is the side elevation plane: x forward (drawn to the right), y up ..." The handedness sentence follows D6 (OQ2). |
| S8 | `🟦️.ts:49` `VIEWPOINTS` doc | "`elevation` is the rig plane ... `front` is the orthogonal elevation seen from the front ... `plan` is seen from above" | "`elevation` is the side elevation, the rig plane ... `front` is the front elevation ... `plan` is the plan (top view), seen from above" |
| S9 | `🟦️.ts:232-233` shape docs | "projected to the view (elevation, front or plan)"; "at elevation x `axisX`" | "projected to the viewpoint (side elevation, front elevation or plan)"; "at side-elevation x `axisX`" |
| S10 | `🟦️.ts:297` `ViewSection` doc | "sheets and the viewer caption the view "Section A-A" / "Schnitt A-A"" | "sheets and the viewer caption the cut "Section A-A" / "Schnitt A-A" (front) and "Plan section B-B" / "Grundriss-Schnitt B-B" (plan)" |
| S11 | `🟦️.ts:358` | "orthographic projections are the elevation, the front and the plan" | "orthographic projections are the side elevation, the front elevation and the plan (top view)" |
| S12 | `🟦️.ts:660-661` `widthM`, `depthM` | "elevation drawing", "elevation plane", "the plan's y" | "side elevation drawing", "side elevation plane", "the plan (top view) y" |

### 4.6 Validator messages (owner contracts, `L/🔨️modules/🗂️catalog/🟦️.ts`)

| ID | File:line | Current | Replacement |
|---|---|---|---|
| M1 | `🟦️.ts:245` `elevation-lateral` (en) | "...author it as a solid ... so the side view follows..." | "...so the side elevation follows..." (de "Seitenansicht" is already right) |
| M2 | `🟦️.ts:254` `follow-tip-viewpoint` (de) | "...nur für Vorderansicht- und Grundrissformen vorkommt." | "...nur für Vorderansichts- und Draufsichtformen vorkommt." |
| M3 | `🟦️.ts:293` `extent` messages | en "...of the ${label} ${viewpoint} view..." with the raw id `front` or `plan`; de "Vorderansicht" / "Draufsicht" | en "...of the ${label} front elevation..." / "...plan (top view)..." via the names of V2 and V3; de unchanged |
| M4 | `🟦️.ts:307` `section-plane-outside` | en "The ${viewpoint} section ..." (raw id); de "Der Schnitt ... der ${viewpoint === "front" ? "Vorderansicht" : "Draufsicht"}" | en "The front section" / "The plan section"; de "Der Schnitt" for front, "Der Grundriss-Schnitt" for plan (R4), and "der Vorderansicht" / "der Draufsicht" only as the viewpoint name |
| M5 | `🟦️.ts:338-339` `alternative-symbol-view-missing` | en "...for the view(s) ${lacking.join(", ")}; an alternative must be drawable in elevation, front and plan." (raw ids) | en "...for the viewpoint(s) ${names}; an alternative must be drawable in side elevation, front elevation and plan (top view)." with the names of V1 to V3 |
| M6 | `🟦️.ts:356` `elevation-empty` | en "The elevation geometry must not be empty."; de "Die Ansichtsgeometrie darf nicht leer sein." | en "The side elevation geometry must not be empty."; de "Die Seitenansichtsgeometrie darf nicht leer sein." |
| M7 | `🟦️.ts:358` `front-empty` (en) | "The front geometry must not be empty." | "The front elevation geometry must not be empty." |
| M8 | `🟦️.ts:359` `plan-empty` | en "The plan geometry must not be empty."; de "Die Grundrissgeometrie darf nicht leer sein." | en "The plan (top view) geometry must not be empty."; de "Die Draufsichtgeometrie darf nicht leer sein." (R4) |
| M9 | `🟦️.ts:382` `symbol-empty` | en "A symbol needs elevation or plan geometry."; de "Ein Symbol braucht Ansichts- oder Grundrissgeometrie." | en "A symbol needs side elevation or plan (top view) geometry."; de "Ein Symbol braucht Seitenansichts- oder Draufsichtgeometrie." (The schema conflict is in the inventory, item 4.8.) |

### 4.7 Other modules (owners as named)

| ID | File:line | Current | Replacement | Owner |
|---|---|---|---|---|
| O1 | `🏗️structure/🟦️.ts:530` (de) | "...in der Aufrissebene" | "...in der Ebene der Seitenansicht" | structural |
| O2 | `🏗️structure/🟦️.ts:530` (en) | "...in the elevation plane" | "...in the side elevation plane" | structural |
| O3 | `📤️downloads/🟦️.ts:113` file description | "asset ..., view ${metadata.view}, viewpoint ${metadata.viewpoint}, ..." | "asset ..., representation ${metadata.view}, viewpoint ${viewpointName(metadata.viewpoint)}, ..." (the machine key stays, OQ3) | viewer |
| O4 | `🖼️svg/🟦️.ts:136` desc (en) | "...view ${m.view} (${m.viewpoint})..." | "...representation ${m.view}, viewpoint ${m.viewpoint} (side elevation, front elevation or plan)..." | exports |
| O5 | `📕️pdf/🟦️.ts:249` subject | "...view ${meta.view}, ${meta.viewpoint}" | "...representation ${meta.view}, viewpoint ${meta.viewpoint}" | exports |
| O6 | `🔬️detail/📤️export-panel/🟦️.tsx:91` | `t("detail.export.lede", { view: request.view, ... })` passes the raw mode id | pass `{ representation: <localised name of request.view>, viewpoint: <localised name of request.viewpoint>, scale }` (D-C2) | viewer |
| O7 | `🎛️viewer/🟦️.tsx:202` (Segmented label) | `t("viewer.viewpoint")` | no change; the label is correct | viewer |
| O8 | `🎛️viewer/🟦️.tsx:151` `<Tag>` | `viewpointName(viewer.viewpoint)` | no change once E2 to E4 land | viewer |

### 4.8 Tests and fixtures (owner validation, with the contracts owner for shared fixtures)

| ID | File:line | Current | Replacement |
|---|---|---|---|
| TS1 | `🧪️tests/✂️section-views/🟦️.ts:125` | `toContain("Section B-B")` | `toContain("Plan section B-B")` (en) and the German `Grundriss-Schnitt B-B` checked too |
| TS2 | `🧪️tests/✂️section-views/🟦️.ts:126` | `toContain("Front view")` | `toContain("Front elevation")` |
| TS3 | `🧪️tests/✂️section-views/🥒️.feature:4` | `instead of "Front view" / "Vorderansicht"` | `instead of "Front elevation" / "Vorderansicht"`; the words "the side view at 1:100" become "the side elevation at 1:100" |
| TS4 | `🧪️tests/✂️section-views/🥒️.feature:23` | `Then they read "Section A-A", "Schnitt A-A", "Front view" and "Vorderansicht"` | `Then they read "Section A-A", "Schnitt A-A", "Front elevation", "Vorderansicht", "Plan section B-B" and "Grundriss-Schnitt B-B"` |
| TS5 | `🧫️fixtures/✂️section-views/🔣️.json:13` | `"en": "Side view", "de": "Seitenansicht"` | `"en": "Side elevation"` |
| TS6 | `🧫️fixtures/✂️section-views/🔣️.json:14` | `"en": "Front view", "de": "Vorderansicht"` | `"en": "Front elevation"` |
| TS7 | `🧫️fixtures/✂️section-views/🔣️.json:15` | `"en": "Top view", "de": "Draufsicht"` | `"en": "Plan (top view)"` |
| TS8 | `🧫️fixtures/✂️section-views/🔣️.json:17` | `"en": "Section B-B", "de": "Schnitt B-B"` (plan) | `"en": "Plan section B-B", "de": "Grundriss-Schnitt B-B"` |
| TS9 | `🧪️tests/🧍️front-projection/🟦️.ts:158` | `toContain(view === "front" ? "Vorderansicht" : "Grundriss")` | `toContain(view === "front" ? "Vorderansicht" : "Draufsicht")` (R4) |
| TS10 | `🧪️tests/🔤️caption-fit/🟦️.ts:43` | `"Größe in °C · Grundriss µ"`, `"Schnitt A-A · Strichzeichnung, dunkel"` | `"Größe in °C · Draufsicht µ"`, `"Schnitt A-A · Dunkle Strichzeichnung"` (R4, C4) |
| TS11 | `🧪️tests/🔤️caption-fit/🟦️.ts:89` | en `"light-and-burn@150 · Section A-A through ..."` | no change (front cut) |

### 4.9 The new viewpoints module tests and the viewer tests

Any test under `L/🎯️targets/⚛️react/🧪️tests/` that expects "Side", "Front", "Top", "All three", "Seite", "Vorne", "Oben", "Alle drei", "Ansicht" for a mode or "view" for a mode must follow V1 to V4 and E1 to E21. The file list was not complete at the snapshot (the viewer tests are being edited); the owner runs `grep -rn` with the patterns of section 5 and fixes every hit.

### 4.10 Prose sweep of the catalog (owner: the category workers, per `🗂️catalog/<category>/`)

Counts are case-insensitive substring hits in `🗂️catalog/**/🔣️.json` at the snapshot. They are not all deviations, but each needs the rule applied.

| Pattern | Hits | Rule |
|---|---|---|
| `Grundriss` (German) | 503 | Only a plan cut (R4). Replace the rest by `Draufsicht`, for example `Grundrisssymbol` becomes `Draufsichtsymbol`. |
| `Stirnansicht`, `Frontansicht` | 15, 13 | `Vorderansicht`. |
| `Aufriss` | 16 | `Seitenansicht` or `Vorderansicht`, by the view meant. |
| `front view` (en) | 541 | `front elevation`. |
| `side view` (en) | 123 | `side elevation`. |
| `top view`, `plan view` (en) | 103, 184 | `plan (top view)` on first mention, then `plan`; `plan view` becomes `plan`. |
| `elevation` alone (en) | 836 | `side elevation` when the side is meant; `front elevation` when the front is meant. Do not write a bare `elevation` for either. |
| `Ansicht` for a mode | (not counted by pattern) | `Darstellung`. The pattern check of section 5 finds the prose. |
| `Vorderansicht`, `Draufsicht`, `Seitenansicht` (de) | 782, 832, 257 | Already correct. Keep. |
| `from above` (en) | 113 | Keep when it describes the plan; it is not a label. |

### 4.11 README (`L/README.md`, owner: coordinator)

| ID | Line (snapshot) | Current | Replacement |
|---|---|---|---|
| R-a | 17 | "The same state drives all seven views — dark line, ..." | "The same state drives all seven representations — dark line, ..." |
| R-b | 48 | "`--environment` with another view is refused." | "`--environment` with another representation is refused." |
| R-c | 53 | "`thermalSpanK` on a cast (thermal view only)" | "(thermal representation only)" |
| R-d | 60 | "`geometry.elevation` (side), `geometry.front` and `geometry.plan` (top)"; "the viewer offers all three always"; "`--views` for a side, front and top sheet" | "`geometry.elevation` (side elevation), `geometry.front` (front elevation) and `geometry.plan` (plan, top view)"; "the viewer always offers the three views"; "`--views` for a sheet of the three views" |
| R-e | 65 | "authored in the **bone's local frame like an elevation shape**"; "the view is right at every pose" | "like a side-elevation shape"; "the three views are right at every pose" |
| R-f | 83 | "the side view (elevation) is its orthographic projection" | "the side elevation (`elevation`) is its orthographic projection" |
| R-g | 97 | "...so elevation, front and plan agree"; "in the side view" | "...so the side elevation, the front elevation and the plan agree"; "in the side elevation" |
| R-h | 119 | "colours the thermal, forces and moments views and has no default" | "colours the thermal, forces and moments representations and has no default" |
| R-i | 120 | "`elevation` (default), `front` or `plan`" | "`elevation` (side elevation, default), `front` (front elevation) or `plan` (plan, top view)" |

### 4.12 Export and CLI prose (owner exports)

| ID | File:line | Current | Replacement |
|---|---|---|---|
| X1 | `🎬️cli/🟦️.ts:74` error text | "--environment applies to --view thermal only" | Keep the flag name. The prose "thermal representation" can follow in the same message if the owner wants. |
| X2 | `🎬️cli/🟦️.ts:2` header | "--view dark-line" | Keep. It is an identifier. |

## 5. Check patterns for the owners (not run as a gate)

These patterns find the deviations above. The owners run them in their own areas and fix every hit; this document does not claim they are clean.

- `grep -rn -E '"(Side|Front|Top|All three|Seite|Vorne|Oben|Alle drei)"' L/🎯️targets L/🔨️modules` (labels)
- `grep -rn -E 'Side view|Front view|Top view|Section B-B|Grundriss[^-]' L` (captions, validator, tests; the last one also finds the R4 cases, check each by hand)
- `grep -rn -E 'Stirnansicht|Frontansicht|Aufriss|Seite"|Vorne|Oben' L` (banned words)
- `grep -rn -E 'Ansicht' L/🎯️targets/⚛️react/🔨️modules/🌐️i18n` (every German `Ansicht` must now be a viewpoint, not a mode)

## 6. What this proposal does not change

- No identifier, schema key, flag, URL key, cast field or file name changes (R8).
- No code, test or fixture is edited by this slot; the owners apply the changes in section 4.
- The German `Blickrichtung` for the viewpoint choice stays.
- `detail.col.section` ("Querschnitt") is a member cross-section, not a view, and stays.
