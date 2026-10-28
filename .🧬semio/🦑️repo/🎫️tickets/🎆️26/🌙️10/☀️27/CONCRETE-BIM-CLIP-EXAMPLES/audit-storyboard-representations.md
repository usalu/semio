# Audit: Storyboard Representations ("One Object, Many Views")

Read-only audit of `storyboard-representations.md` against `research-ifc.md` (section 7 and the sections it relies on), `research-villa-savoye.md`, `audit-current.md` ("Prior Decisions To Keep"), `agent-rules.md` (quality bar) and `api.md`. No repository file was changed. The only file written is this report.

## Method and limits

- All six documents read in full. Every narration clause (10 en, 10 de) was counted by script (scratch script in the session scratchpad, outside the ticket).
- Two WebSearch lookups settled disputed IFC points (section 3 below). Both are search snippets; the buildingSMART pages were not fetched.
- Not done: no `capture.py` run and no render. Findings about 720p legibility, overlaps and text size are arithmetic estimates from the stage geometry (1 Manim unit = 720 px / 8 units = 90 px at 720p). A grep for the `caption_bar` font size timed out, so the two-line wrap at 11.5 units is unverified.

## 1. Caption length (rule 6)

Policy: about 115 characters. Result: no clause exceeds 115 in either language. The longest are beat 4 en (108) and beat 4 de (103).

| Beat key | en chars | de chars | Over 115 |
| --- | ---: | ---: | --- |
| source | 105 | 94 | no |
| plan | 98 | 99 | no |
| section | 96 | 91 | no |
| elevation | 108 | 103 | no |
| three | 85 | 93 | no (contains "3D", a name, not a value) |
| stored | 104 | 102 | no |
| derived | 86 | 90 | no |
| update | 91 | 98 | no |
| copies | 85 | 103 | no |
| identity | 93 | 96 | no |

Corrected clauses in section 2 were counted by hand and are below 115 (en 95 to 110, de 93 to 113). Re-count before publishing.

Digits: only beat 5 contains a digit ("3D"), a view name. Acceptable.

## 2. Issues

Severity: high = breaks an owner rule or a factual claim; medium = overclaim, layout risk or missing German; low = wording or polish. Corrected narration is given in full where the clause changes.

| # | Beat key | Severity | Problem | Concrete fix |
| --- | --- | --- | --- | --- |
| I-01 | all (de) | low | The storyboard uses "Fensterband" throughout. "Bandfenster" is the standard German term for a ribbon window and is the term the audit brief names. Use one term everywhere. | Replace "Fensterband" with "Bandfenster" in on-screen labels and in the de clauses of source, plan, elevation and update (corrected below). |
| I-02 | header; open item 4 | medium | Overclaim: "the historically best supported one" and "the only one with strong historical support". research-villa 4.4 rates the presence of ribbon windows as high and their extent as low. The south-only claim rests on one source (FR Wikipedia) and is disputed by Rost ("band wraps the box"); open question 8 is unresolved. | Replace with: "The south ribbon window is the only extent a source names (FR Wikipedia, research-villa 4.4); the other sides are disputed (open question 8) and are teaching simplifications." Apply the same wording to open item 4. |
| I-03 | source (beat 1) | medium | The axonometric view draws a ribbon band on all four façades (api.md section 1: one ribbon IfcWindow per wall). This visually asserts the disputed extent. | Draw only W-S and RW-S in ARCH/GLASS. Draw the other three bands as MUTED outline with a label "teaching simplification / Lehrvereinfachung". |
| I-04 | source, plan, elevation | medium | Teaching orientation. The storyboard puts the salon and the ribbon window on the south and the curved glass wall on the north. research-villa 10.5 (EN Wikipedia) says the salon faces south-east and the terrace east. api.md states the orientation as a simplification, but the clip only says "Teaching model" in beat 1. | Add a visible badge to the beat 1 hero: en "teaching orientation", de "Lehrorientierung". Keep the captions unchanged. |
| I-05 | section (beat 3) | high | Breaks bar 3 (numbers from model functions). Level marks "±0.00 / +3.50 / +6.70 / +9.40" are typed, although the text says "from storeys". `building_geometry()["storeys"]` holds only ground, first floor (assumed +3.50) and roof (assumed +6.70). +9.40 is the box height (sourced, single source), not a storey. research-villa 2.6: storey heights are not sourced at all. | Take ±0.00, +3.50, +6.70 from `storeys` and +9.40 from `box["height"]`. Each mark carries its basis from the model value ("assumed" or "sourced"). Rewrite the beat 3 text accordingly. |
| I-06 | plan (beat 2, wall detail) | medium | "0.30 m" is typed as the example for `quantity(wall,"thickness")`. Thickness is assumed in api.md. The wall build-up "render / brick / plaster" is not sourced (research-villa 7.2: render specification not found; 3.1: infill brick or plaster). The IFC Qto name for thickness is `Width` (see section 3), so an IFC-name label must not say "thickness". | Show only the model value. Tag the detail "assumed build-up / angenommener Aufbau". If an IFC name is shown: "Width (Qto)". |
| I-07 | stored (beat 6) | high | Breaks bar 3. The FootPrint row says "thin strip because the wall is 0.30 m thick" as on-screen text. The value is assumed and typed. | Replace with "thin strip: thickness from the model" / de "dünner Streifen, Stärke aus dem Modell". The value is read from `quantity(wall,"thickness")` if shown at all. |
| I-08 | stored (beat 6) | medium | "five south pilotis circles" is a typed count. research-villa 2.11: the count is not settled (lex.dk 25, student slideshow 15). The "Recommended teaching values" table says: do not state a count. | Drop the count. Draw the pilotis from `geo["pilotis"]` at the south edge with no number on screen. |
| I-09 | stored (beat 6) | medium | Narration overclaim: "IFC stores the wall as Axis, FootPrint and Body". IFC does not require these three representations; this is the model's choice. The claim that the elevation is derived is supported by the same logic as plan and section, but research-ifc 7 names only plan and section. | en: "In this model the wall is stored as Axis, FootPrint and Body; plan, section and elevation are derived from it." (110 characters). de: "Im Modell ist die Wand als Axis, FootPrint und Body gespeichert; Grundriss, Schnitt und Ansicht sind abgeleitet." (112 characters). |
| I-10 | stored (beat 6); three (beat 5) | medium | (a) The Body chip reads "IfcExtrudedAreaSolid over IfcRectangleProfileDef" without the "modelled as" qualifier that open item 2 promises. (b) Technical gap: an extrusion of a rectangle profile is a solid without a hole. Beat 5 shows the void and the window subtracted from the body, so the storyboard must say how the void enters the Body (for example an IfcBooleanResult difference) or that the void is carried only by the opening. research-ifc has no statement on this; [M]. | Chip: "modelled as IfcExtrudedAreaSolid over IfcRectangleProfileDef". Beat 5 on-screen text: state the void mechanism in the modelled form. Verify on IFC 4.3.2.0 before publication. |
| I-11 | stored (beat 6) | low | Wording: "all three representation chips are attributes of one IfcProductDefinitionShape". The representations are the items of `IfcProductDefinitionShape.Representations`; the product points to that shape through its Representation attribute (research-ifc 9: `#121 IFCPRODUCTDEFINITIONSHAPE`). | "All three are representations of one IfcProductDefinitionShape, referenced by one IfcProduct." |
| I-12 | stored (beat 6); api M3 | medium | FootPrint type. M3 hard-codes `Curve2D`. The WebSearch shows that the IFC4 FootPrint template is mainly 2D curves (`IfcGeometricCurveSet`), and that `Surface2D` is also a RepresentationType. The 4.3 table was not confirmed in the snippets. The storyboard draws the footprint as a hatched filled polygon, which is a region. | Do not hard-code the type. Take it from `represent("W-S","footprint")`. Add to the open items: "verify FootPrint RepresentationType (Curve2D or Surface2D) on the 4.3.2.0 page". Narration does not name a type. |
| I-13 | stored (beat 6) | low | German sublabels. "Achse" for "centre line" is ambiguous (grid axis). "Grundfläche" is the floor area, not the footprint outline. | Centre line: de "Mittellinie". FootPrint: de "Umriss im Grundriss". |
| I-14 | stored (beat 6) | medium | Missing German. "one GlobalId, three shape representations" has no German. The agent rule requires every visible string in both languages. | de: "eine GlobalId, drei Formdarstellungen". |
| I-15 | stored (beat 6) | medium | Overlap risk. Three leader lines run from the R1 chip (x ≈ 4.45, y 2.82) to the three left chips (x ≈ -5.75). They cross the H drawings (rows span x -6.9 to 1.5, rows 1 to 3 up to y 2.95). | Remove the lines from R1. Use a vertical bracket on the left margin (x ≈ -6.9) joining only the three left chips, labelled "one GlobalId / eine GlobalId", and repeat the GlobalId chip text at the top of the column. |
| I-16 | three, stored (beat 5, 6) | low | Relation chain: three chips and two relation labels must fit an 8.4-unit row. Labels such as "IfcOpeningElement · <tag>" at 18 pt may not fit; the fit is unmeasured. | Use two-line chips (class line, tag line). Measure with `capture.py`. |
| I-17 | three (beat 5); stored (beat 6) | high | Breaks the owner's bar "never show a bare box standing for a wall". Beat 5 shows an isolated wall solid (`element_axon`), and the beat 6 Body row reuses it. The storyboard's own checklist (section 4) claims the wall sits in a recognisable villa "in every view", which beat 5 contradicts. | Beat 5: keep the villa context as ghosted outline (first-floor slab outline and the pilotis row), with the wall solid in ARCH on top. Beat 6 Body row: same ghosted pilotis row behind the solid. Fallback in beat 5 becomes the default. |
| I-18 | three (beat 5) | low | German comma: "eine massive Wand, mit einer Öffnung" has a stray comma. | de: "Die 3D-Ansicht zeigt den Körper: eine massive Wand mit einer Öffnung, die das Fenster füllt." |
| I-19 | elevation (beat 4) | low | "Solarium screens" / "Solariumswände" are not in research-villa-savoye. api.md models them as curved roof screens; research is silent on shape. Plausible, but unsourced. | Add a source to research-villa-savoye, or label the screens "assumed shape" in the model, and keep the caption. |
| I-20 | section (beat 3) | low | Meaning shift in de: en "cuts through slab, pilotis and window"; de "zeigt Decke, Pilotis und Fenster" (shows, not cuts through). "Decke" is less precise than "Geschossdecke" for a slab. | de: "Der Schnitt geht durch Geschossdecke, Pilotis und Fenster: Brüstungs- und Sturzhöhe werden zu Maßlinien." |
| I-21 | derived (beat 7) | medium | Abstract element. The share bar (horizontal bar with a GLASS segment) is a bar chart standing for a value. It breaks "nothing abstract; the value is shown by its meaning". | Remove the bar. Show the glazing share as the GLASS cross-hatch fraction on the elevation, with the percentage at the ribbon. If the bar is kept, its label must say "share of gross wall area / Anteil der Bruttowandfläche". |
| I-22 | derived (beat 7) | medium | Missing German. All readout labels are English only: gross wall area, opening area, net wall area, glazing share. The German narration "Bruttowand" is ambiguous (it reads as a wall, not an area). "Fenster" for the opening area is also inexact. | Labels: "gross wall area / Bruttowandfläche", "opening area / Öffnungsfläche", "net wall area / Nettowandfläche", "glazing share / Glasanteil". Narration en: "The same geometry gives the areas: gross wall, opening, net wall and the glazing share." de: "Dieselbe Geometrie liefert die Flächen: Bruttowandfläche, Öffnung, Nettowandfläche und Glasanteil." |
| I-23 | derived (beat 7) | low | Percent format. The storyboard uses " %" (space) in both languages. English is "40%"; German is "40 %". | en: "40%"; de: "40 %" (use the language-aware `number()` formatter). |
| I-24 | derived (beat 7) | low | Sublabels `GrossSideArea` and `NetSideArea` are IFC Qto names of tier [D]. The WebSearch snippet of the official 4.3.2.0 Qto_WallBaseQuantities page gives definitions that agree with the storyboard: GrossSideArea is taken from the middle-plane elevation without openings; NetSideArea accounts for openings. The snippet spells the footprint property `GrossFootPrintArea`. | Keep the sublabels. Update open item 1: name and definition confirmed on the search snippet; fetch the 4.3.2.0 page before publication. |
| I-25 | update (beat 8) | medium | Overclaim. "every view and number follows" implies the plan changes. The storyboard itself says the plan is regenerated and looks identical. "Planungsereignis" is an awkward calque; "Planänderung" is the natural term. | en: "A design change lowers the sill. The ribbon window grows; all views and numbers are regenerated." (95 characters). de: "Eine Planänderung senkt die Brüstung. Das Bandfenster wird höher; alle Ansichten und Zahlen werden neu erzeugt." (111 characters). |
| I-26 | update (beat 8) | medium | Plan legibility at 720p. The board plan is at 0.12 units/m, so the 0.30 m wall is 0.036 units, about 3 px, and the ribbon glass lines are 1 to 2 px. The plan thumbnail is not readable at 720p. The same holds for the R2 slots (about 0.10 units/m, wall about 2.7 px). | Set a minimum stroke of 2.5 px at 720p (about 0.028 units) for wall poché and glass lines in thumbnails, or raise the board plan scale to about 0.16 and shrink the 3D. State in the storyboard that thumbnails identify, and detail is read only in the hero. |
| I-27 | update (beat 8) | low | Readout panel: seven rows (event card, chip with revision badge, five values with a small MUTED reference) in 2.9 units. That is about 0.41 units per row at 18 to 20 pt. Tight. | Measure with `capture.py`. Move the event card to a header line, or set the MUTED references in a second column. |
| I-28 | copies (beat 9) | low | de tense: "Kopierte Zeichnungen blieben zurück" (past) against the en conditional "would stay behind". On screen, "neu erzeugt · keine Handlung" is stilted. | de: "Kopien würden zurückbleiben: Jede Kopie müsste von Hand abgeglichen werden, Ansicht für Ansicht." On-screen chip de: "neu erzeugt · ohne Eingriff". |
| I-29 | identity (beat 10) | medium | Narration conflicts with beat 8: the sill (an opening's geometry, not only its representation) changes. The on-screen text "four filled slots linked to the chip" and "linked view / verknüpfte Ansicht" implies that the GlobalId links the views. research and "Prior Decisions To Keep" say GlobalId gives stable references and does not propagate revisions. The views follow because they come from one model. | en: "A representation shows the object; a design change updates it, and its GlobalId stays the same." (95 characters). de: "Eine Darstellung zeigt das Objekt; eine Planänderung aktualisiert es, und seine GlobalId bleibt dieselbe." (105 characters). On screen, the leaders say "from one model / aus einem Modell", not "linked". |
| I-30 | identity (beat 10); copies (beat 9) | low | German: "bleibt gleich" is weaker than "bleibt dieselbe" for identity. The badges "rev 1", "rev 2" and "rev 1 · GUID" are English only. | de: "bleibt dieselbe". Badges: "Rev. 1 / Rev. 1" with a German-capable label; the 'rev' label gets de "Rev." |
| I-31 | Object card (beat 2) | medium | Object card rows show "Type · Infill wall" and "Pset_WallCommon.LoadBearing · FALSE" as facts. research-villa 13 and 16.8 say which walls are infill is unverified, and api.md marks them assumed. The rows have no German. | Show each row's basis from `information()` as a chip ("assumed"/"angenommen"). de rows: "Typ · Ausfachungswand"; "Pset_WallCommon.IsExternal · TRUE" with de label "Außenwand"; "LoadBearing · FALSE" with de label "tragend · FALSE". |
| I-32 | all assumed values | medium | Truthfulness anchor 4 says the single "teaching model" mention in beat 1 covers the assumed numbers. The quality bar allows one mention per clip, but on screen the assumed values (sill, head, storey elevations, wall thickness, ribbon length) look identical to sourced ones (box 21.5 m). | Give every assumed value a basis chip from the model (same mechanism as I-31). Keep the single "teaching model" caption. |
| I-33 | all text (beats 2 to 10) | medium | Text size. Dimension values, GlobalId monospace, chip sublabels, relation labels and "small" MUTED references have no size. The rule is at least 16 pt. 18 pt is about 0.2 units, about 19 to 22 px tall at 720p (estimate, from the default font 48 being about 1/12 of frame height; verify). | Set one floor: 16 pt for every string, 18 pt for labels, 18 to 20 pt for property rows. Verify with `capture.py`. |
| I-34 | R2 slots (beats 2, 6, 10) | medium | Slot size 2.35 x 1.15 units is about 211 x 104 px at 720p. A 18 pt label plus a thumbnail plus the GlobalId inside it is tight. The claim "four views readable at 720p" holds for identity and for the hero, not for the thumbnail detail (see I-26). | State in the storyboard: thumbnails identify the view; detail is read in the hero. Keep the labels outside the thumbnail frame if they collide. |
| I-35 | plan (beat 2) | low | Dimension values are real and model-driven (good). The 21.5 m box length is sourced but single-source (research-villa 2.1, medium). api.md marks it "sourced" without the single-source note. | Keep the value. Add "single source" to the api basis, or leave it as "sourced" with the research note. |
| I-36 | caption band | low | Caption band top at y = -2.66 (below -2.62 and below SAFE_BOTTOM -2.60). Drawing stage bottom -2.55 leaves 0.11 units (about 10 px at 720p). Compliant on paper. | No change. Confirm with the first rendered frame. |

## 3. Checked and compliant

- IFC names: `IfcWall`, `IfcWindow`, `IfcOpeningElement`, `IfcRelVoidsElement`, `IfcRelFillsElement`, `Axis`, `FootPrint`, `Body`, `SweptSolid`, `GlobalId` (22 characters) are consistent with research-ifc sections 6, 7, 8 and 9.
- Void and fill chain (wall to opening to window) matches research-ifc 6 and 15.2.
- Plans and sections are derived views (research-ifc 14.9 and 15.1). Beat 8 "regenerated, identical" matches the derived-view rule.
- Product versus representations (beat 6) and "a representation does not change identity" (beat 10, after I-29) match the "Prior Decisions To Keep" list.
- Dark green ground floor, set-back, pilotis, white box, ramp (research-villa 3.5, 5.3, 7.1) are supported.

## 4. WebSearch results used

- FootPrint: IFC4 FootPrint template is mainly 2D curves (`IfcGeometricCurveSet`); Surface2D is a RepresentationType in IFC4; 4.3 table not confirmed in snippets. Sources: [IFC 4.3.2.0 IfcShapeRepresentation](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/IfcShapeRepresentation.htm), [IFC4 FootPrint GeomSet template](https://standards.buildingsmart.org/IFC/RELEASE/IFC4/ADD1/HTML/schema/templates/footprint-geomset-geometry.htm).
- Qto_WallBaseQuantities (4.3.2.0 snippet): thickness is the property `Width`; GrossSideArea is the middle-plane elevation without openings; NetSideArea subtracts openings; spelling `GrossFootPrintArea`. Sources: [IFC 4.3.2.0 Qto_WallBaseQuantities](https://standards.buildingsmart.org/IFC/RELEASE/IFC4_3/HTML/lexical/Qto_WallBaseQuantities.htm). Vendor context only (2016, not authoritative): [Allplan help](https://help.allplan.com/PLANBAR/2016-1/1033/Allplan/79138.htm).

## 5. Corrected narration (en and de), changed clauses only

| Key | en | de |
| --- | --- | --- |
| source | unchanged | Lehrmodell der Villa Savoye: ein Objekt, die Südwand des Obergeschosses mit ihrem Bandfenster. |
| plan | unchanged | Der Grundriss schneidet die Wand auf Fensterhöhe: Wand als Poché, Bandfenster als dünne Glaslinien. |
| section | unchanged | Der Schnitt geht durch Geschossdecke, Pilotis und Fenster: Brüstungs- und Sturzhöhe werden zu Maßlinien. |
| elevation | unchanged | Die Ansicht zeigt die ganze Südfassade: Bandfenster, dunkelgrünes Erdgeschoss, Pilotis, Solariumswände. |
| three | unchanged | Die 3D-Ansicht zeigt den Körper: eine massive Wand mit einer Öffnung, die das Fenster füllt. |
| stored | In this model the wall is stored as Axis, FootPrint and Body; plan, section and elevation are derived from it. | Im Modell ist die Wand als Axis, FootPrint und Body gespeichert; Grundriss, Schnitt und Ansicht sind abgeleitet. |
| derived | The same geometry gives the areas: gross wall, opening, net wall and the glazing share. | Dieselbe Geometrie liefert die Flächen: Bruttowandfläche, Öffnung, Nettowandfläche und Glasanteil. |
| update | A design change lowers the sill. The ribbon window grows; all views and numbers are regenerated. | Eine Planänderung senkt die Brüstung. Das Bandfenster wird höher; alle Ansichten und Zahlen werden neu erzeugt. |
| copies | unchanged | Kopien würden zurückbleiben: Jede Kopie müsste von Hand abgeglichen werden, Ansicht für Ansicht. |
| identity | A representation shows the object; a design change updates it, and its GlobalId stays the same. | Eine Darstellung zeigt das Objekt; eine Planänderung aktualisiert es, und seine GlobalId bleibt dieselbe. |

Lengths of the corrected clauses were estimated by hand (en 95 to 110, de 93 to 113). Re-count with the script before publishing.

## 6. Open items for the author

1. Decide the teaching-orientation badge text (I-04) and whether the non-south ribbon bands stay drawn (I-03).
2. Replace the typed level marks and wall thickness with model values (I-05, I-06, I-07).
3. Choose one void mechanism for the Body (I-10) and verify it on IFC 4.3.2.0.
4. Verify the FootPrint RepresentationType on IFC 4.3.2.0 (I-12) and the Qto names (I-24).
5. Measure every label size, relation chain and slot with `capture.py` at 720p (I-16, I-26, I-27, I-33, I-34).
6. Source or label the solarium screens (I-19).
