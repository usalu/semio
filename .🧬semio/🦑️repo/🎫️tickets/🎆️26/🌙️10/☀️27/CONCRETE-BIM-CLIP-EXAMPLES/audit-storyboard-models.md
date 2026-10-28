# Audit: Storyboard "Models Share a Schema" (Beat4_PartialModels)

Read-only audit of `storyboard-models.md` against `research-ifc.md` (sections 10, 11, 14, 15), `research-villa-savoye.md`, `audit-current.md` ("Prior Decisions To Keep"), `agent-rules.md` (quality bar) and `api.md`. Only this file was written. Severity: **high** = factual or quality-bar violation that must change before capture; **medium** = clear risk that needs a fix or an explicit label; **low** = polish.

Corrected clause lengths are counted in characters (Python `len`, Unicode). Each corrected clause is at most 113 characters.

## Facts settled by search this session

| Item | Result | Source |
|---|---|---|
| `IfcStructuralSurfaceMember` in IFC 4.3 with `Thickness` | Exists. Direct instances are centred on their reference surface; depth is in `Thickness`. Storyboard statement "Thickness is a parameter of the surface member" is correct. | [IFC 4.3 IfcStructuralSurfaceMember](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcStructuralSurfaceMember.htm) |
| `IfcStructuralCurveMember` in IFC 4.3 | Exists; `PredefinedType` is `IfcStructuralCurveMemberTypeEnum`. `RIGID_JOINED_MEMBER` (beam-like, rigid or pinned joints) and `PIN_JOINED_MEMBER` (axial only, truss-like) are different choices. The storyboard names the class correctly but does not state which member type the pilotis use. | [IFC 4.3 IfcStructuralCurveMember](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcStructuralCurveMember.htm), [IfcStructuralCurveMemberTypeEnum (dev build)](https://standards.buildingsmart.org/IFC/DEV/IFC4_3/HTML/lexical/IfcStructuralCurveMemberTypeEnum.html) |
| Schema identifier `IFC4X3_ADD2` | Research section 1 marks it as [P] (IFC 4.3.2.0). Search results show only "under development" builds, so keep the chip but check it against the released 4.3.2.0 page (research 16.5). | `research-ifc.md` section 1 |
| `IfcRoot.Name`, `IfcProduct.ObjectPlacement` | Both OPTIONAL in IFC 4.3 (research section 2 and the Description `$` example, line 220). Only `GlobalId` is mandatory on `IfcRoot`. | `research-ifc.md` sections 2 and 9 |

## 1. Factual errors and overclaims

| Beat key | Severity | Problem | Concrete fix (en / de) |
|---|---|---|---|
| idea paragraph; beat `schema` | high | "A count matrix shows the three models use the same IFC classes with different content." The matrix itself shows zero cells, so the models do not use the same classes. The shared thing is the schema vocabulary. | Idea text: "A count matrix shows one shared vocabulary, with different classes and counts in each model." Narration `schema` is already correct and stays. |
| beat `schema` footer | high | "every object has GlobalId, Name and placement (IfcRoot, IfcProduct)". Name and ObjectPlacement are OPTIONAL. Overclaim. | en "Shared rules: every object has a GlobalId (IfcRoot); the rules are the same in all three models." (96 chars) / de "Gemeinsame Regeln: jedes Objekt hat eine GlobalId (IfcRoot); die Regeln gelten in allen drei Modellen gleich." (109 chars) |
| beat `federation` (narration) | high | "GlobalIds link them" states the shared-GlobalId case as if it were how federation works. In the lesson, the same GlobalId for the slab is set by `uuid5(tag)` (api.md section 1), i.e. a construct. Prior decision: a shared schema does not imply shared instances or identity. Coordination does not bring GlobalIds into agreement by itself. | en "Coordination puts the models on one origin. One GlobalId marks the shared slab; each model keeps its author." (108) / de "Koordination legt Modelle auf einen Ursprung. Eine GlobalId markiert die gemeinsame Decke; Urheber bleiben." (107) |
| beat `federation` (on screen, y -1.95) | high | "same GlobalId in {k} models" and "same GlobalId = the same object" present the teaching construct as fact. No statement that the lesson sets the GlobalId. | Statement line: en "A shared schema does not make shared objects: this teaching model sets the same GlobalId." (89) / de "Ein gemeinsames Schema macht keine gemeinsamen Objekte: Dieses Lehrmodell setzt dieselbe GlobalId." (98) |
| beat `federation` (on screen, y -1.35) | high | "Federated: architecture rev a · structure rev b · services rev c". Letters contradict the storyboard's own rule (revisions are integers, "rev N") and read as the UK letter-revision convention (research 11). | "Federated: architecture rev {N_arch} · structure rev {N_struct} · services rev {N_serv}" with values from `partial_model(id)["revision"]`. de "Zusammengeführt: Architektur Revision {N} · Tragwerk Revision {N} · Haustechnik Revision {N}". |
| beat `federation` (badge) | low | `badge("coordinated", GOOD)` reads like a status code (UK S1 "coordination"). Not a global ISO code. | Use a label such as "checked" (en) / "geprüft" (de), or no badge. Never S0–S4. |
| beat `federation` (Idea) | medium | "Beat 1 and beat 6 end on the same federated villa picture": beat 1 is `villa_axon(AX=(-3.2, 0.15), S1=0.16)`; beat 6 is `(-3.0, 0.0)`, `S2=0.11`, explode 0. Not the same picture. | Use the same point and scale in both beats, or drop "on purpose" and say "similar picture". |
| beat `federation` (layout) | medium | Villa at `S2` = 0.11 is about 5.1 high, centred at y 0.0, so spans y -2.55 to +2.55. Stage bottom is -2.5 and the caption band starts below -2.62. | Centre at y 0.2 (span -2.35 to 2.75). |
| beat `check` (narration) | high | "A coordination check finds what no single model shows: a downpipe too close to a piloti." Presents an invented clash as a finding. The clash is a teaching scenario (research 13: the services layout is not documented; api.md has no clash source). | en "In this teaching scenario, a clash check finds what no single model shows: a downpipe too close to a piloti." (108) / de "Kollisionsprüfung im Lehrbeispiel: ein Fallrohr liegt zu nah an einer Pilotistütze, was kein Einzelmodell zeigt." (112) |
| beat `check` (on screen) | high | No on-screen label that the clash, downpipe route and rule are invented. | Add above the plan at (-3.4, 2.65), size 19, MUTED: en "Teaching scenario: the downpipe route is chosen for this lesson." (64) / de "Lehrbeispiel: Die Fallrohrführung ist für diesen Clip frei gewählt." (67) |
| beat `check` (card line 2) | medium | "required" is an assumed teaching rule (`clash_check` `required` basis = assumed, api.md). The line "teaching rule" implies a rule set in the scene, and the German "Lehrregel" is unnatural. | en "clearance {c} m < required {r} m · assumed rule" / de "Abstand {c} m < gefordert {r} m · angenommene Regel". Numbers from `clash_check()`. |
| beat `translation` (narration) | high | "translated into another schema: structural analysis". IFC 4.3 has its own structural analysis domain (research 10; Prior decision: "the chosen analysis schema is an example"). The narration implies a schema outside IFC. | en "For one use case, the structure model is translated into a structural analysis model: IFC's analysis domain." (108) / de "Für einen Anwendungsfall wird das Tragwerksmodell in ein Tragwerksanalysemodell übersetzt: die IFC-Analysedomäne." (113) |
| beat `translation` (frame title) | medium | Target chip is the model's schema name; the Prior decision says to name it as an example, not as the standard. | Label the frame "example analysis schema" / "Beispiel-Analyseschema" under the chip. |
| beat `members` (count row 2) | high | "IfcStructuralSurfaceMember (first-floor slab) m shells". The IFC object count is one surface member. Shells are the teaching mesh from `translate_structure()["shells"]`, not IFC objects. Overclaim. | Row 2: "IfcStructuralSurfaceMember 1" (count from `members` of kind surface). New row 3: "analysis shells m (teaching mesh)" / "Analyse-Schalen m (Lehr-Netz)", shells lit one by one. Shift rows to y 1.4, 0.9, 0.4, -0.1; reference chip to -0.6; mapping note to -1.1. |
| beat `members` (reference chip) | medium | "each analysis member keeps the GlobalId of the physical object it came from". Reads as an IFC feature. The IFC link is unverified (research 10, 15.14). | "source GlobalId" as stored in the translation record: en "translation record: source GlobalId C3" (current caption is fine apart from "keeps") / de "Übersetzungsprotokoll: Quell-GlobalId C3". Keep "no IFC relationship name is claimed". |
| beat `members` (slab thickness) | medium | "t = 0.xx m" shows a value that research 3.2 does not source (thickness not found). `api.md` marks slab thickness `assumed`. | Dimension text "t = {value} m (assumed)" / "t = {value} m (angenommen)", value from `quantity()`. |
| beat `members` (shells) | medium | "subdivides into shells on the 4.75 m grid" is a teaching mesh choice, presented as what the analysis does. | Add "teaching mesh" to the shell label (see row above). |
| beat `services` (note y -1.35) | high | "The Savoyes' roof leaked every autumn: drainage is a services concern." Juxtaposing leak and drainage implies a cause. Research 8.1 has the leak; 7.4 names waterproofing and detailing (medium). No drainage cause is sourced. The note says "no cause is claimed", which the text contradicts. | en "The Savoyes' roof leaked every autumn. The drain routing shown here is a teaching scenario." (91) / de "Das Dach der Savoyes war jeden Herbst undicht. Die Entwässerung hier ist ein Lehrbeispiel, nicht historisch." (108) |
| beat `services` (on screen) | high | Boiler on ground floor, radiator under salon ribbon window, roof drains and downpipe routes are invented; no source in research 8 or 2. Not labelled. | Add label (MUTED, size 19): en "Teaching scenario: boiler, radiator and drain positions are chosen for this lesson." (83) / de "Lehrbeispiel: Lage von Kessel, Heizkörpern und Abläufen ist für diesen Clip frei gewählt." (89). |
| beat `services` (note) | low | "Model note (once, small): radiator positions are teaching-model positions; do not add another 'teaching model' mark." This repeats the teaching-model label that the storyboard says appears once in beat 1. | Delete this sentence; the scenario label above replaces it. Keep "teaching model" once (beat 1). |
| beat `services` (downpipes, boiler, radiator types) | low | `IfcPipeSegment` for downpipes, `IfcBoiler`, `IfcSpaceHeater` placement: no research source. Research 15.15 requires stating the modelling decision. | Keep the classes (real IFC 4.3 names). Add the decision to the scenario label, and check `PredefinedType` values against 4.3.2.0 before naming any value. |
| beat `services` (roof garden) | medium | "Roof garden ... the way the roof garden is drained" links the hatched garden to drains. No source for drainage routing. | Covered by the scenario label; keep the hatch and area only. |
| beat `loss` (narration) | medium | "render colour" is an unsourced claim. Research 7.1 gives "white box, dark green ground floor", 7.2 says the render specification was not found. The model's render item is a material layer. | en "Ribbon windows, infill walls, render layer and curved glass wall are not in the analysis. Record the losses." (108) / de "Fensterbänder, Ausfachungen, Außenputz und gebogene Glaswand fehlen in der Analyse. Verluste werden festgehalten." (113) |
| beat `loss` (footer) | medium | "{lost} properties lost": `lost = len(analysis["losses"])`, while the table shows "at most five rows". If there are more than five entries, the table and the counter disagree. | State the rule in the build: show all loss entries (aggregate by `source_tag` when more than five), so the table count equals `lost`. |
| beat `loss` (reversibility line) | low | Placement of the line "Translation is not automatically reversible." is not specified; it sits under footer badges at y -1.95. | Place at y -2.3, size 19. Keep text: en "Translation is not automatically reversible." / de "Eine Übersetzung ist nicht automatisch umkehrbar." (Prior decision: lossless inverse not guaranteed. Matches.) |
| beat `assumptions` (narration) | medium | "material": the analysis adds material properties (stiffness, density). The concrete material is already in the structure model. Ambiguous. | en "Assumptions add what was never drawn: nodes, material properties, supports and loads." (85) / de "Annahmen ergänzen, was nie gezeichnet wurde: Knoten, Materialkennwerte, Lagerung und Lasten." (92) |
| beat `assumptions` (row 2) | medium | "reinforced concrete, 25 kN/m³": density is an assumed load parameter (api.md: loads `assumed`); the row has no `assumed` badge. | Row text en "reinforced concrete, unit weight 25 kN/m³ (assumed)" / de "Stahlbeton, Wichte 25 kN/m³ (angenommen)"; value from `loads`. |
| beat `assumptions` (loads) | low | "tributary()['area_load']" is a tributary-area estimate, not an FEM result (api.md). | Label the load row "tributary estimate" / "Einzugsflächen-Schätzung". |
| beat `translation` (rule rows) | low | Mapping rule `IfcColumn → IfcStructuralCurveMember` is a modelling decision (research 15.15). | Add a "mapping decision" label or small MUTED text: en "modelling decision" / de "Modellierungsentscheidung". |
| beat `structure` (pilotis member type) | low | Pilotis are shown as `IfcStructuralCurveMember` without a member type (RIGID_JOINED_MEMBER vs PIN_JOINED_MEMBER). | State the chosen type in the build docs; no need on screen. |
| beat `whole` (counter) | medium | "N objects in three models": N is the union of GlobalIds, so the shared slab counts once. Label implies a plain sum. | en "N unique objects in three models" / de "N eindeutige Objekte in drei Modellen". Lit elements light while the counter runs. |
| beat `whole` (narration) | OK | Clause is accurate. | none |
| beat `architecture` (narration) | OK | "ribbon windows" as IfcWindow in an IfcOpeningElement matches research 14.8 and 15.2. The clause lists "the spaces", which no beat shows. | Optional: add a spaces row, or drop "and the spaces" (en/de). |
| beat `structure` (narration) | OK | Pilotis, slabs and ramp. "It leaves out render and windows" is a model choice; the storyboard's truthfulness note says so. | none |
| beat `loss` (ISO) | OK | No ISO 19650, CDE or UK status code on screen. "Federated" (beat 6) is UK guidance usage and consistent with research 11. | none |
| all beats (ISO 19650 status) | OK | No S0–S4 codes; revisions are integers. Only the letters in beat 6 break this (see above). | none |
| beat `check` (BCF) | low | "Issue (BCF) points to both GlobalIds hidden behind a collapse". BCF refers to elements by GlobalId (research 11, OK), but the BCF topic GUID is a different identifier (Prior decision). A collapsed chip cannot be photographed. | Show both `guid_chip`s visible, caption "Meldung (BCF) verweist auf beide GlobalIds" (keep). Do not show a topic GUID as a GlobalId. |
| beat `services` (dates) | OK | "Villa Savoye, Poissy · 1928–1931" matches the research teaching value "built 1928–1931". Start year is disputed (1928 vs 1929, medium). Keep the range, do not name a start date. | none |

## 2. Villa Savoye claims not supported or not labelled

| Beat key | Severity | Problem | Concrete fix (en / de) |
|---|---|---|---|
| `structure` (count row, Not-in-model) | high | "25 pilotis" and the "5 × 5 grid" are shown as the Villa's count. Research 2.11: not settled; lex.dk says 25 columns, a student slideshow 15. Research 13/Recommended values: "Do not state a count; mark as open." | Row label en "piloti (teaching model count)" / de "Pilotis (Lehrmodellzahl)". Caption or chip: en "count open in sources" / de "Anzahl in Quellen offen". The 5 × 5 grid is also a teaching choice; say so on the grid label, or omit the "5 × 5" wording. |
| `architecture` (row 2) | medium | Ribbon windows on all four façades (api.md: four `RW-*` windows). Research 4.4 / Open question 8: only the south façade per FR Wikipedia; EN/Rost say the band wraps. Unresolved. | Row label en "ribbon windows, all four sides (teaching choice)" / de "Fensterbänder, alle vier Seiten (Lehrentscheidung)". |
| `architecture` (material strip) | medium | "render / brick infill / plaster" and per-layer thicknesses: research 3.1 "brick or plaster" (medium); thicknesses unsourced; api.md marks them assumed. | Title en "Infill wall: layers (assumed)" / de "Ausfachung: Schichten (angenommen)". |
| `services` (radiator under salon ribbon window) | high | Radiator position "under the ribbon window" has no source. Salon orientation is a teaching simplification (api.md; research 10.5 says salon faces south-east). | Covered by the scenario label (section 1). Do not name the salon compass side on screen unless the orientation note is shown. |
| `services` (110 m²) | medium | Roof garden area 110 m² is medium confidence (research 2.7, Cité snippet). The storyboard types the value on screen. | Value from `building_geometry()["spaces"]` (area), label "about {value} m²" / "rund {value} m²". |
| `loss` (render colour) | medium | "render colour" lost property: only white box and dark green ground floor are sourced (7.1). | Use "render layer" (see section 1). |
| `members` (slab) | medium | Slab thickness not sourced (3.2). | Label "assumed" (see section 1). |
| `structure` (ramp) | low | Ramp listed as structural (teaching choice). Research 3.5: layout known, slope and length not found. Ramp type HALF_TURN_RAMP is in api.md but not named on screen. | Keep; do not name the ramp type on screen (research 13: verify the flight arrangement before naming). |
| `structure` (chip LoadBearing) | low | "Pset_ColumnCommon.LoadBearing = TRUE" for C3. Research 15.5 warns against TRUE as a universal rule. | en "Pset_ColumnCommon.LoadBearing = TRUE (this piloti)" / de "Pset_ColumnCommon.LoadBearing = TRUE (dieser Pilotis, tragend)". |
| `whole` (years) | OK | 1928–1931 is the research teaching value. | none |
| `assumptions` (fixed base) | OK | "fixed base / eingespannt" carries an assumed badge. | none; keep the badge. |

## 3. Numbers typed by hand (not from model functions)

| Beat key | Severity | Problem | Concrete fix |
|---|---|---|---|
| `services` | high | "value '110 m²'" typed. | Compute from `building_geometry()["spaces"]` area; format with `number()`. |
| `check` | high | "0.15 m" (piloti radius) typed in the description of the inset. | Use `building_geometry()["pilotis"]` radius (api.md: piloti Ø 0.30 m assumed; research 2.12 not found). Label assumed in the inset. |
| `federation` | high | "rev a · rev b · rev c". | Numbers from `partial_model(id)["revision"]` (see section 1). |
| `assumptions` | medium | "25 kN/m³" written as row text. | Value from `loads` (the example text is only an example; make sure the build reads it). |
| `translation`, `members` | medium | "4.75 m" written in beat 9 and "IfcStructuralCurveMember n" counts. | "4.75" from `clearance()["spacing"]`; `n` and `m` from `translate_structure()`. |
| `whole`, `structure`, `services` | low | Typed numbers in descriptions: 25 pilotis, lag 1/25, C3, `4.75 m`, `t = 0.xx m`. | Labels and captions must come from the model; the descriptions are fine as design notes. |
| all beats | medium | German number format: "4.75 m", "0.15 m", "t = 0.xx m", "q = {q} kN/m²" will show the English decimal point in German. | Route every number through `number(value, digits)`; German uses a comma (4,75 m; 0,15 m; 1,5 kN/m²). |

## 4. Abstract visuals (violate "nothing abstract; every value shown at the same time by its meaning")

| Beat key | Severity | Problem | Concrete fix |
|---|---|---|---|
| `schema` (matrix) | high | Matrix of bare numbers in tinted cells is an abstract grid. The storyboard lights elements only in the thumbnails, not in the same animation as each cell count. | In the same 0.9 s as each column's counts, light the counted elements in that column's thumbnail (`lit(tags, color)`). Show the class name at the lit element. Keep the tint as a secondary cue. |
| `translation` (frame) | high | "dashed frame ... inside, empty for now" is an empty abstract box. | Fill the frame in the same beat with the target class chips and their counts from `translate_structure()` (IfcStructuralCurveMember n, IfcStructuralSurfaceMember m), or a small `analysis_view` at `S4`. Do not leave it empty. |
| `whole`, `architecture` (card) | medium | Counters ("N objects", "object total") run without lighting the counted parts in the same step. | Light the counted parts during the counter (same rule as the class rows). |
| `structure` (Not in this model) | medium | "IfcCurtainWall ×n" is a count with no lit object. Only the windows flash. | Flash the ghost curtain wall as well, in the same step. |
| `structure` (Not in this model) | medium | Omitted classes shown in `WARN`. `WARN` means a problem or loss (agent-rules, colour meaning). An omission by design is neither. | Use `MUTED` for the omission line and the ghost flash; keep `WARN` for loss in beat 11. |
| `architecture` (material strip) | low | Bands in distinct colours are abstract when the layer names are not visible on the band. | Label each band with its name (already listed at x 4.5; move it next to the band). |
| `loss` (counters) | low | "properties lost · members kept" counters: bare numbers. Acceptable because the loss table and fading elements show the items. | Keep; confirm the table rows and the ghosted elements appear in the same step. |

## 5. German quality

| Beat key | Severity | Problem | Concrete fix (de) |
|---|---|---|---|
| `check` (narration) | high | "Koordinationsprüfung" for a clash check. The term is "Kollisionsprüfung". Card uses "Konflikt-01". | "Kollisionsprüfung" in narration; card "Kollision-01 · Fallrohr {a_tag} ↔ Pilotistütze {b_tag}". |
| `translation` (narration) | high | "übersetzt ... in ein anderes Schema: die Tragwerksanalyse": a schema is not "die Tragwerksanalyse". | "... in ein Tragwerksanalysemodell übersetzt: die IFC-Analysedomäne." (see section 1) |
| `assumptions` (narration) | medium | "Material" is too general for the analysis input. | "Materialkennwerte" (see section 1). |
| `loss` (narration) | high | Imperative "Halte die Verluste fest" uses the informal "du" form. Every other clause is neutral (no imperative or address). "Putzfarbe" = colour; the model item is the render layer. "gebogenes Glas" conflicts with row 2 "gebogene Glaswand". | "Außenputz", "gebogene Glaswand", "Verluste werden festgehalten" (see section 1). |
| `loss` (line) | low | "Halte" is not used anywhere else. | "Die Verluste werden festgehalten." |
| `check` (card line 2) | medium | "Lehrregel" is unnatural. The required clearance is an assumed rule. | "angenommene Regel" (see section 1). |
| `members` (mapping note) | medium | "Linie ≠ Stützenkörper, Fläche ≠ Deckenkörper". "Fläche" and "Linie" are not the IFC element names; "Stab" and "Flächenelement" are. | de "Stabelement ≠ Stützenkörper · Flächenelement ≠ Deckenkörper" (59); en "line member ≠ column solid · surface member ≠ slab solid". |
| `assumptions` (row 2) | medium | "Stahlbeton, 25 kN/m³": kN/m³ is "Wichte" (specific weight), not "Dichte". | "Stahlbeton, Wichte 25 kN/m³ (angenommen)". |
| `schema`, `federation`, `translation`, `services` (badges) | low | "rev N" in German badges. | "Revision N" in German (or "Rev. N" consistently). |
| `architecture` (row label) | low | "Dachwände" is unclear for solarium screens. | "Fassaden- und Sichtschutzwände auf dem Dach". |
| `whole`, `schema` | low | Optional: the term "Fachmodell" is the discipline-model term. The clips use "Modell" and "Architekturmodell", which is natural German. | Optional: "Drei Fachmodelle, ein Schema" in narration `schema`. Not required. |
| `services` (narration) | OK | "Haustechnik", "Kessel", "Heizkörper", "Dachabläufe" are correct. | Add "Fallrohre" to the clause (see section 6). |
| `federation` (narration) | OK | "Die Koordination legt ... auf einen Ursprung" reads as federation. | Use the corrected clause (section 6). |

## 6. Caption length (narration clauses, characters)

Current narration (all under 115):

| Key | EN | DE | Status |
|---|---|---|---|
| whole | 66 | 77 | OK |
| architecture | 99 | 99 | OK |
| structure | 88 | 85 | OK |
| services | 90 | 96 | OK |
| schema | 80 | 75 | OK |
| federation | 91 | 108 | OK (closest) |
| check | 88 | 105 | OK |
| translation | 93 | 104 | OK |
| members | 97 | 101 | OK |
| assumptions | 74 | 83 | OK |
| loss | 104 | 105 | OK |

Corrected clauses (proposed in section 1 and 5): services en 101 / de 107; federation en 108 / de 107; check en 108 / de 112; translation en 108 / de 113; assumptions en 85 / de 92; loss en 108 / de 113. None over 115. The two closest to the limit are translation de (113) and loss de (113); keep them, or shorten the German if the audio runs over.

On-screen strings proposed in this audit: footer `schema` de 109; statement `federation` de 98; note `services` en 83 / de 89; leak note `services` de 108; scenario labels 64–89. All within two lines at the caption size; no string over 115.

## 7. Layout risks (overlaps, caption band, text size)

No explicit text size is below 17 (the smallest is 17, layer names in beat 2). The 16 pt floor is met where sizes are given. Several items have no size: dimension text, leader labels, `guid_chip`, `badge`, `readout`, matrix cells, rev badges, `q` readout, class chips. Set every one explicitly (17 or larger) and check it in the layout report.

Character widths below are rough estimates at about 0.13–0.14 Manim units per character for size 18–20 text, to be confirmed in `…-layout.md`.

| Beat key | Severity | Problem | Concrete fix |
|---|---|---|---|
| `architecture` (material strip) | high | Title "Ausfachungswand: Materialschichten" (34 chars at size 18, about 4.5 units) starts at x 1.0 and reaches the layer-name column at x 4.5 (y -1.2). The strip at y -1.55 spans x 1.0 to 1.0 + 10 × total thickness: any total above 0.35 m runs into x 4.5. | Title "Ausfachung: Schichten (angenommen)" (33 chars, still long): move the title above the strip. Put layer names in a column at x 5.0 (size 17). Cap the strip at 3.4 units wide (scale to total thickness, e.g. 3.4 / total). |
| `architecture` (class rows) | medium | Class column from x 1.55 to label x 3.4 (1.85 units). "IfcCurtainWall" (14 chars) at size 21 is about 1.8–2.2 units: touches the label. | Label at x 3.7, or class size 19 in both rows. |
| `schema` (footer) | medium | DE footer 109 chars at size 20 is about 13–15 units, at or over the stage width (13.6). | Two lines, or size 18. |
| `federation` (drawing) | medium | Exploded villa spans y -2.55 to 2.55 (0.05 below the stage floor -2.5). | Centre at y 0.2 (see section 1). |
| `federation` (leaders) | medium | Two leaders from the chip at x 3.55 to the slab at x about -3 cross the whole drawing: clutter and ambiguity. | Replace with two markers on the two slab copies in the drawing, plus one label beside each; no long leaders. |
| `check` (inset vs card) | medium | Inset circle (centre (3.6, 0.55), radius 1.75) reaches y -1.2; card line 1 at y -1.35 (text top about -1.17). Collision. | Centre (3.6, 0.8), radius 1.5 (bottom at -0.7). The clash geometry still fits; keep the dashed ring at the required clearance. |
| `check` (issue chip) | low | Issue chip at y -2.15 (size 19) with bottom near -2.35, close to the caption band. | Keep; move to y -2.05 if the card moves up. |
| `translation` (rule rows) | high | Rule row width 3.8 for two class chips and an arrow. "IfcStructuralCurveMember" (24 chars) and "IfcStructuralSurfaceMember" (26 chars) at size 18 are about 3.3–3.6 units each: overflow. | Two-line chips (class name on the second line), size 17, or widen the middle column to 4.2 (x -2.1 to 2.1). |
| `translation` vs `members` | medium | Beat 8 structure axon at (-4.3, 0.1), `S3` 0.12; beat 9 at (-3.0, 0.1), 0.15, described as "same position and scale". The morph jumps. | Use (-4.3, 0.1) and 0.12 in both beats. Analysis view must use the same projection (api.md item 5). |
| `members` (count rows) | medium | Row labels up to 28 characters ("IfcStructuralPointConnection") plus counts, right zone 6.3 wide. | Size 18–19 for class labels, or the German class in a second line. |
| `loss` (footer) | low | Reversibility line placement unspecified; footer badges at y -1.95 (size 21). | Line at y -2.3 (size 19), stays above the caption band (-2.5 to -2.62). |
| `whole`, `services`, `members` (sizes) | low | Dimension text, chips and badges have no size. | Set explicitly; no size below 17. |
| caption band | OK | No element positioned below -2.5 except the exploded villa (fixed by moving the centre). | none |
| title chip | OK | Top items (y 2.4–2.7) stay under 3.0. | none |

## Summary of findings

- Narration corrections: 6 clauses (services, federation, check, translation, assumptions, loss) with corrected en/de; all corrected clauses are at most 113 characters. The longest current source clauses are 104 (en, loss) and 108 (de, federation), so no clause exceeds the 115 limit as written.
- Biggest factual risks: the shared GlobalId presented as fact (beat 6), the invented clash and services layout not labelled as teaching scenarios (beats 4 and 7), "25 pilotis" stated as the Villa's count (beat 3), the surface member count shown as shells (beat 9), and the letter revisions "rev a/b/c" (beat 6).
- Typed numbers and abstract visuals: "110 m²" and "0.15 m" typed; the matrix (beat 5) and the empty translation frame (beat 8) are abstract. German fixes: Kollisionsprüfung, Wichte, Materialkennwerte, Außenputz, the Lehrregel to angenommene Regel, and the imperative in beat 11.
- Layout: the beat 2 strip title collides with the layer list at x 4.5; the beat 8 rule chips overflow the 3.8-unit width; beat 6 needs a lower centre; beat 7 inset collides with the check card; beat 8 and beat 9 use different positions and scales.
- Verified this session: `IfcStructuralSurfaceMember` (IFC 4.3, `Thickness`) and `IfcStructuralCurveMember` (IFC 4.3, `PredefinedType` enum) exist as the storyboard uses them; `IFC4X3_ADD2` is kept from research [P] and still needs a check against the released 4.3.2.0 page. Sources: [IfcStructuralSurfaceMember](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcStructuralSurfaceMember.htm), [IfcStructuralCurveMember](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcStructuralCurveMember.htm), [IfcStructuralCurveMemberTypeEnum](https://standards.buildingsmart.org/IFC/DEV/IFC4_3/HTML/lexical/IfcStructuralCurveMemberTypeEnum.html).
