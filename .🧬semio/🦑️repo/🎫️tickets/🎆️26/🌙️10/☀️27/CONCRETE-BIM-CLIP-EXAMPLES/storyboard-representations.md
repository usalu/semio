# Storyboard: 👁️representations (`Beat3_SameObject`, `bim_representations`)

Title en "One Object, Many Views" / de "Ein Objekt, Viele Darstellungen". Ten beats, one NARRATION row each. Protagonist: the **south first-floor façade wall `W-S` (`IfcWall`) of the Villa Savoye with its ribbon window `RW-S` (`IfcWindow`)**. One GlobalId, four drawings (plan, section, elevation, 3D), three stored IFC representations (Axis, FootPrint, Body), derived quantities, one design event, one stale copy, one identity. Phase 1 only: no code edited. Facts follow `research-ifc.md` (sections 4, 6, 7, 8, 14, 15) and `research-villa-savoye.md` (2.1, 2.2, 4.4, 5.3).

Truthfulness anchors used below:

- IFC stores per-element `Axis`, `FootPrint` and `Body` shape representations (`IfcShapeRepresentation`: RepresentationIdentifier + RepresentationType); plans, sections and elevations are **not** stored geometry, view tools derive them from the model (research-ifc 7, safe statement 9, mistake 1).
- A window sits in a wall through `IfcOpeningElement`: wall -[`IfcRelVoidsElement`]-> opening -[`IfcRelFillsElement`]-> window (research-ifc 6, safe statement 8, mistake 2).
- Gross/net side areas are quantities a tool derives (`Qto_WallBaseQuantities.GrossSideArea / NetSideArea`, source tier [D]; see open item 1).
- The ribbon window on the south façade is the historically best supported one (research-villa 4.4, FR Wikipedia); all other numbers (sill, head, wall thickness, storey elevations) are `assumed` in the model, covered by the single "teaching model" mention in beat 1.

## 1. Stage, zones, persistent elements

Manim frame 14.22 x 8; the BIM baseline frames show the title at the top left (y > 3.3) and a two-line caption reaching up to y = -2.66 (energy `SAFE_BOTTOM` = -2.60). Drawing stage: **x in [-6.9, 6.9], y in [-2.55, 3.10]** (13.8 x 5.65). Nothing touches the caption band.

Two layouts carry the clip.

**Sequence layout (beats 1-7 and 10):** one large drawing at a time, a persistent identity column on the right.

| Zone | Rectangle | Content |
| --- | --- | --- |
| H (hero) | x [-6.9, 1.5], y [-2.55, 3.10], centre (-2.7, 0.275) | the drawing of the beat, scale chosen so it fits 8.4 x 5.65 incl. its dimension lines |
| R1 (identity) | x [2.0, 6.9], y [2.55, 3.10] | `guid_chip("W-S", (4.45, 2.82))`, width 4.8 |
| R2 (views) | x [2.0, 6.9], y [0.10, 2.45] | 2 x 2 **slots**, each 2.35 x 1.15: plan (3.2, 1.875), section (5.7, 1.875), elevation (3.2, 0.675), 3D (5.7, 0.675). Slot = dashed MUTED frame with a 18 pt label (Plan / Grundriss, Section / Schnitt, Elevation / Ansicht, 3D). A filled slot holds the real drawing at thumbnail scale (`labels=False, grid=False`) with `W-S` in ARCH and `RW-S` in GLASS, frame turns ARCH while that view is the hero |
| R3 (object card) | x [2.0, 6.9], y [-2.55, -0.10] | `property_table` "Object data · unchanged / Objektdaten · unverändert" (rows below) |

**Board layout (beat 8, a drawing sheet):** top row y [0.20, 3.10]: elevation x [-6.9, -0.45] (scale 0.30, 6.45 x 2.82, centre (-3.65, 1.65)) and the live readout panel x [0.0, 6.9]. Bottom row y [-2.55, 0.0]: plan x [-6.9, -3.7] (scale 0.12), section x [-3.5, 0.7] (scale 0.22), 3D x [1.0, 6.9].

**Pair layout (beat 9):** live elevation (rev 2) left and a static copy of the rev 1 elevation right, both scale 0.20 (4.3 x 1.9), centres (-3.4, 0.55) and (3.4, 0.55); three stale mini copies of plan, section, 3D below the right one.

Colour code, constant for the whole clip (toolkit palette): wall `W-S` = `ARCH`, window `RW-S` = `GLASS`, dimension lines, values and the highlighted GlobalId text = `DATA`, copy/stale/mismatch = `WARN`, unchanged/consistent = `GOOD`, everything else in a drawing = `MUTED`/`INK` outline and `CONCRETE`, `GREEN_GROUND` for the dark-green ground floor in elevation. Hatch spacing is the same in all views.

Object card rows (all from `information("W-S", property_id, lang)` and `element("W-S")`): `IfcWall · <predefined_type>`; `Type · <type name>` (Infill wall); `Pset_WallCommon.IsExternal · TRUE`; `Pset_WallCommon.LoadBearing · FALSE`. It appears in beat 2 and is **never edited** until beat 8, which makes "the data stay the same in every view" visible.

Data used throughout (never typed): `base = load_model()`; `wall = element("W-S", base)`; `win = element("RW-S", base)`; `geo = building_geometry(base)`; `wg = [w for w in geo["walls"] if w["tag"] == "W-S"][0]`; `op = wg["openings"][0]` (start, end, sill, head); `q0 = quantities("W-S", base)`; `event = <model-supplied demo event, see request M5>`; `new = apply_event(event, base)`; `q1 = quantities("W-S", new)`; `gid = global_id("W-S")`. Numbers below like `21.5 m`, `0.90 m` are format illustrations only.

Counters: every shown quantity counts up from 0 to its model value while its dimension line or hatch grows (`ValueTracker` + `always_redraw` readout with `self.number(v, d)` and unit; German decimals handled by `number`).

Dock: at the end of a view beat the hero drawing moves into its slot (`ReplacementTransform(hero, thumb)` of the same widget at thumbnail scale; if the morph looks bad, crossfade). The frame of the slot flashes ARCH once.

## 2. Beat table

| # | key | caption en | caption de | hero (zone H) |
| --- | --- | --- | --- | --- |
| 1 | `source` | Teaching model of the Villa Savoye: one object, the south wall of the first floor with its ribbon window. | Lehrmodell der Villa Savoye: ein Objekt, die Südwand des Obergeschosses mit ihrem Fensterband. | whole villa axonometric, `W-S`/`RW-S` lit, class labels, GlobalId chip |
| 2 | `plan` | The plan cuts the wall at window height: the wall in poché, the ribbon window as thin glass lines. | Der Grundriss schneidet die Wand auf Fensterhöhe: Wand als Poché, Fensterband als dünne Glaslinien. | first-floor plan, length dimensions, cut-level gauge, wall detail |
| 3 | `section` | The section cuts through slab, pilotis and window: sill and head heights become dimension lines. | Der Schnitt zeigt Decke, Pilotis und Fenster: Brüstungs- und Sturzhöhe werden zu Maßlinien. | villa section through the salon, dimension chain on the south wall |
| 4 | `elevation` | The elevation shows the whole south façade: ribbon band, dark green ground floor, pilotis, solarium screens. | Die Ansicht zeigt die ganze Südfassade: Fensterband, dunkelgrünes Erdgeschoss, Pilotis, Solariumswände. | south elevation, band length and height |
| 5 | `three` | The 3D view shows the body: a solid wall, voided by an opening that the window fills. | Die 3D-Ansicht zeigt den Körper: eine massive Wand, mit einer Öffnung, die das Fenster füllt. | isolated wall solid, extrusion, void and fill chain |
| 6 | `stored` | IFC stores the wall as Axis, FootPrint and Body; plan, section and elevation are derived from the model. | IFC speichert die Wand als Axis, FootPrint und Body; Grundriss, Schnitt und Ansicht werden abgeleitet. | three stored representations in rows, derived arrows to the slots |
| 7 | `derived` | The same geometry gives the areas: gross wall, window, net wall and the glazing share. | Dieselbe Geometrie liefert die Flächen: Bruttowand, Fenster, Nettowand und den Glasanteil. | elevation with area hatches + readout rows + share bar |
| 8 | `update` | A design event lowers the sill. The ribbon window grows, and every view and number follows. | Ein Planungsereignis senkt die Brüstung. Das Fensterband wächst; jede Ansicht und jede Zahl folgt. | board layout, all four views live |
| 9 | `copies` | Copied drawings would stay behind: each copy needs manual coordination, view by view. | Kopierte Zeichnungen blieben zurück: Jede Kopie müsste von Hand abgestimmt werden, Ansicht für Ansicht. | pair layout: live rev 2 vs stale copy rev 1 |
| 10 | `identity` | A representation changes how the object appears, not what it is: the GlobalId stays the same. | Eine Darstellung ändert, wie das Objekt erscheint, nicht was es ist: Die GlobalId bleibt gleich. | villa axon rev 2 + chip + four filled slots linked to the chip |

Clauses are 85-108 characters (one caption, at most two lines) and contain **no numbers**; every value is on the drawing, read from the model. "Teaching model" is said once (beat 1 caption); no other on-screen duplicate.

NARRATION literal for the scene file (keys in beat order):

```python
NARRATION = [
    ["source", "Teaching model of the Villa Savoye: one object, the south wall of the first floor with its ribbon window.", "Lehrmodell der Villa Savoye: ein Objekt, die Südwand des Obergeschosses mit ihrem Fensterband."],
    ["plan", "The plan cuts the wall at window height: the wall in poché, the ribbon window as thin glass lines.", "Der Grundriss schneidet die Wand auf Fensterhöhe: Wand als Poché, Fensterband als dünne Glaslinien."],
    ["section", "The section cuts through slab, pilotis and window: sill and head heights become dimension lines.", "Der Schnitt zeigt Decke, Pilotis und Fenster: Brüstungs- und Sturzhöhe werden zu Maßlinien."],
    ["elevation", "The elevation shows the whole south façade: ribbon band, dark green ground floor, pilotis, solarium screens.", "Die Ansicht zeigt die ganze Südfassade: Fensterband, dunkelgrünes Erdgeschoss, Pilotis, Solariumswände."],
    ["three", "The 3D view shows the body: a solid wall, voided by an opening that the window fills.", "Die 3D-Ansicht zeigt den Körper: eine massive Wand, mit einer Öffnung, die das Fenster füllt."],
    ["stored", "IFC stores the wall as Axis, FootPrint and Body; plan, section and elevation are derived from the model.", "IFC speichert die Wand als Axis, FootPrint und Body; Grundriss, Schnitt und Ansicht werden abgeleitet."],
    ["derived", "The same geometry gives the areas: gross wall, window, net wall and the glazing share.", "Dieselbe Geometrie liefert die Flächen: Bruttowand, Fenster, Nettowand und den Glasanteil."],
    ["update", "A design event lowers the sill. The ribbon window grows, and every view and number follows.", "Ein Planungsereignis senkt die Brüstung. Das Fensterband wächst; jede Ansicht und jede Zahl folgt."],
    ["copies", "Copied drawings would stay behind: each copy needs manual coordination, view by view.", "Kopierte Zeichnungen blieben zurück: Jede Kopie müsste von Hand abgestimmt werden, Ansicht für Ansicht."],
    ["identity", "A representation changes how the object appears, not what it is: the GlobalId stays the same.", "Eine Darstellung ändert, wie das Objekt erscheint, nicht was es ist: Die GlobalId bleibt gleich."],
]
```

## 3. Beat details

Common: every beat is `self.step(key, *animations, seconds=...)`; labels via `self.tr(en, de)`, numbers via `self.number(v, d)`. After the main animation of a beat with a long clause, an extra `Indicate`/pulse keeps the eye on the highlighted object instead of a frozen hold. Values written `f(...)` come from `api.md` section 2.

### Beat 1 `source` (anim ≈ 3.6 s)

On screen:

1. H: `villa_axon((-2.7, 0.15), scale s1, model=base, highlight=("W-S", "RW-S"), disciplines=("architecture", "structure"))`, `s1` chosen so the drawing fits 7.6 x 4.8. Everything is ghosted outline; `W-S` fills ARCH, `RW-S` GLASS; pilotis, dark-green ground floor and the ramp are visible so the villa is recognisable.
2. Leader labels (via `side_labels` / `leader`): "`IfcWall` · W-S" (from `wall["class"]`, tag) in ARCH to the wall; "`IfcWindow` · RW-S" in GLASS to the ribbon band.
3. R1: `guid_chip("W-S", ...)` fades in; a `leader` from the lit wall to the chip. The 22 characters of `gid` are shown (monospace) — identity written beside the object it names.
4. Slots, R3 and the other views do not exist yet.

Animation: ghost villa `FadeIn` with lag 0.9 s, then `W-S` and `RW-S` light up (`Indicate`), labels grow, chip and leader draw. At the beat end the axon fades (the slot for 3D stays empty until beat 5).

Meaning shown at the same time: the GlobalId is attached to the wall by the leader line; class names are attached to the lit parts.

### Beat 2 `plan` (anim ≈ 4.2 s)

On screen:

1. Slot frames (4 dashed frames with labels) fade in at R2; object card appears at R3.
2. H: `villa_plan("first", (-3.3, 0.9), 0.23, model=base, highlight=("W-S", "RW-S"), grid=True, labels=True, layers=("structure", "architecture"))`, size 4.95 x 4.37, south wall at the bottom. Wall poché in ARCH, ribbon window as two thin GLASS lines, pilotis below the cut shown dashed (hidden), box outline, ramp, curved glass wall at the north.
3. Two stacked dimension lines under the south wall, labelled at their right ends: window length (inner, y ≈ -1.62) and wall length (outer, y ≈ -1.98), values `f(op["end"] - op["start"])` and `quantity(wall, "length")` shown as `self.number(v, 1) + " m"`, positions taken from the drawing's coordinate mapping (request T2), not measured by hand. Both numbers count up while the lines grow.
4. Right part of H (x [-0.4, 1.5]): **cut-level gauge** — a small vertical ruler with level marks first floor / roof from `building_geometry()["storeys"]` and a DATA marker at `represent("W-S", "plan")["derived"]["cut_z"]` labelled "cut / Schnitt +x.xx m" (request M2), tied by a dashed leader to the plan's cut symbol.
5. Below the gauge: **wall detail** callout (`wall_detail("W-S", point, scale)`, request T6, optional): the wall cut at the window, magnified: poché with the layer set of the wall type and a dimension "0.30 m" = `quantity(wall, "thickness")`. A thin leader line runs from the wall in the plan to the detail. This is where the old abstract "thickness = 0.30 m" lives now, on the object.

Animation: grid and ghost pilotis (dashed) draw first, then poché `FadeIn` with lag, then `W-S` fills ARCH, ribbon GLASS lines draw, dimension lines `GrowFromCenter` with counters, gauge and detail fade in. Dock at the end: plan -> slot 1.

### Beat 3 `section` (anim ≈ 4.2 s)

On screen:

1. H: `villa_section((-3.0, 0.55), 0.37, model=base, cut="salon", highlight=("W-S", "RW-S"))`, 7.0 x 3.5. Pilotis, hatched slabs, ground line, level marks `±0.00 / +3.50 / +6.70 / +9.40` (from `storeys`, right of the drawing x [0.55, 1.5]). South is the left end: the cut `W-S` is ARCH, the window gap GLASS; the north wall ghosted.
2. **Dimension chain inside the salon next to the south wall**, three stacked lines with `side_labels`-style dodged labels: sill height (slab top to sill), window height (sill to head), storey wall height (slab top to roof underside). Values: `op["sill"]`, `op["head"]` and `wg["base"]`, `wg["top"]` read from `building_geometry`, converted to heights by the toolkit dimension (request T2), each counting up from 0 as the line grows from the slab.
3. A dashed DATA line at `cut_z` across the whole section labelled "plan cut / Grundrissschnitt": ties the plan of beat 2 to this section (the plan is the horizontal slice at this height).
4. R2/R3 unchanged; slot 1 shows the plan thumbnail.

Animation: ground line, pilotis wipe up, slabs, walls hatch; then `W-S` lights, window gap lights; dimension lines grow from the slab with counters; cut line draws in from left. Dock: section -> slot 2.

### Beat 4 `elevation` (anim ≈ 4.0 s)

On screen:

1. H: `villa_elevation("south", (-2.7, 0.7), 0.33, model=base, highlight=("W-S", "RW-S"))`, 7.1 x 3.1: ground line, dark-green set-back ground floor (`GREEN_GROUND`), pilotis, white first-floor box face tinted ARCH, ribbon band GLASS, solarium screens above. Level marks right of the drawing.
2. Dimensions: wall length (`quantity(wall, "length")`) below the ground line, ribbon band length (`op["end"] - op["start"]` via the mapping) along the top of the band, band height (`sill` to `head`) at the east end. Counters as before.
3. Dotted horizontal projection lines from the section's sill and head levels continue across the elevation (DATA 40 % opacity) for 1 s, then fade: the same two heights appear in both views.

Animation: ground line, pilotis, dark-green ground floor wipe left to right, box face, ribbon, screens, then dimension lines. Dock: elevation -> slot 3.

### Beat 5 `three` (anim ≈ 4.4 s)

On screen:

1. H upper (y [-0.4, 3.1]): isolated wall solid `element_axon("W-S", (-2.7, 1.3), 0.30, model=base, extrude, opening, window)` (request T3) in ARCH; dimensions length / height at the solid (counters). The 3D slot is still empty.
2. H lower (y [-2.5, -0.7]): relation chain from `relations_of("W-S")`, three chips connected by arrows: `IfcWall · W-S` (ARCH) -- `IfcRelVoidsElement` --> `IfcOpeningElement · <tag>` (MUTED dashed) -- `IfcRelFillsElement` --> `IfcWindow · RW-S` (GLASS). Class names from the relation entries; tags of the other side from the same call.

Animation (all driven by three `ValueTracker`s): footprint rectangle on the ground; `extrude` 0 to 1 raises the solid to the wall height (the height dimension grows with it); `opening` 0 to 1 lifts the translucent opening block out of the wall, leaving the hole, while the Voids arrow draws; `window` 0 to 1 slides the glass into the hole while the Fills arrow draws. Beat end: the solid is also placed into slot 4 (3D thumbnail, `ReplacementTransform`).

Fallback if `element_axon` does not exist: `villa_axon(..., highlight=("W-S","RW-S"))` large in H upper with ghost villa, lit wall, no extrusion animation; the chain stays.

### Beat 6 `stored` (anim ≈ 4.4 s)

On screen (H, three rows 8.4 x 1.8, y centres 2.05, 0.25, -1.55; left column x [-6.9, -4.6] chip, drawing x [-4.4, 1.3]):

| Row | chip (from `represent("W-S", kind)`) | drawing |
| --- | --- | --- |
| Axis | identifier `Axis` + type (`Curve2D`), sublabel "centre line / Achse" | the wall axis as a dash-dot ARCH polyline from `represent("W-S","axis")["primitives"]`, end dots, laid over the faint south edge of the villa with the five south pilotis circles (`geo["pilotis"]` with y at the south edge), so the axis visibly runs through the pilotis row |
| FootPrint | identifier `FootPrint` + type, sublabel "plan outline / Grundfläche" | the wall footprint polygon from `represent("W-S","footprint")["primitives"]`, hatched ARCH, thin strip because the wall is 0.30 m thick, with the window span marked by two ticks |
| Body | identifier `Body` + type (`SweptSolid`), sublabel `IfcExtrudedAreaSolid` over `IfcRectangleProfileDef` | the beat-5 solid, moved (`TransformFromCopy`) and scaled into this row |

R1 chip stays; three thin lines run from the chip down the left margin to the three chips ("one GlobalId, three shape representations"). The right zone's slots 1-3 get a dashed GOOD frame with the heading "derived drawings / abgeleitete Zeichnungen" (R2 header, 18 pt); three dashed GOOD arrows run from the Body row to plan, section and elevation slots. The 3D slot keeps its solid frame (it is the Body itself, rendered).

Animation: rows build one after another (axis draws, footprint wipes, body arrives); chips appear with their row; the dashed arrows grow last while the three slots pulse once. All three representation chips use the identical GlobalId chip reference above (they are attributes of one `IfcProductDefinitionShape` of one entity).

### Beat 7 `derived` (anim ≈ 5.0 s)

On screen:

1. H upper (y [0.45, 3.1]): `villa_elevation("south", (-2.7, 1.8), 0.31)` re-enlarged from slot 3 (`TransformFromCopy(slot3 thumb, hero)`), 6.7 x 2.9; the first-floor wall face is the stage for the hatches (rest ghosted).
2. H lower (y [-2.5, 0.2]): four readout rows (swatch | label | counting value), plus the share bar:
   - gross wall area: ARCH diagonal hatch over the whole wall face, value `q0["gross_side_area"]` m², small sublabel `GrossSideArea` (open item 1);
   - opening area: GLASS cross hatch over the ribbon, value `q0["opening_area"]` m²;
   - net wall area: hatch over the wall face **minus** the ribbon, value `q0["net_side_area"]` m², sublabel `NetSideArea`;
   - glazing share: horizontal bar of the gross area length with the GLASS segment proportional to `q0["glazing_share"]`; the percentage is the same number as `self.number(q0["glazing_share"] * 100, 0) + " %"`.
3. Chip and slots remain. Slot 3 flashes ARCH.

Animation: each hatch wipes left to right over the elevation (`always_redraw` hatch of the polygon clipped at x(t)) while its counter rises with the covered area; so the number is literally the drawn area. Then the ribbon hatch slides out of the gross hatch (net = gross - opening, the ribbon area "falls" into the net row as a minus), then the bar grows. Moves on without rows when the next beat starts.

### Beat 8 `update` (anim ≈ 5.2 s)

Transition (1.2 s): hero rows and R-zone fade; slot thumbnails, the elevation hero and the chip `ReplacementTransform`/`animate` into the **board layout**: elevation top left, readout panel top right, plan, section and 3D along the bottom. The chip moves to the panel header.

Readout panel (x [0.0, 6.9], y [0.2, 3.1]): row 0 event card (monospace, `step_panel`-like) with the event's real fields: `OpeningResized`, object id (the window tag), `sill` target, `head`; row 1 the same `guid_chip("W-S")` plus a revision badge `rev 1` (`base["revision"]`) that ticks to `rev 2` (`new["revision"]`); rows 2-6: **sill**, **head**, **opening area**, **net wall area**, **glazing share** — large live value in DATA, small MUTED "rev 1: …" value beside it (from `base`), a swatch matching the drawing element.

Views (all `always_redraw`, driven by one `ValueTracker t` that moves the sill from `op["sill"]` to the event's sill; each frame builds `m_t = apply_event({**event, "sill": s(t)}, base)` and calls the widgets and `quantities("W-S", m_t)`, so every intermediate number is a model value):

- elevation (top left): ribbon band grows downward, band height dimension and the GLASS hatch track `s(t)`, the dotted sill line slides;
- section (bottom middle): sill dimension shrinks, window height grows, counters live;
- 3D (bottom right): the ribbon window in the villa axon gets taller;
- plan (bottom left): regenerated with a visible sweep like the others but **looks identical** (length and thickness are unchanged). A small GOOD tag "regenerated · same result / neu erzeugt · gleiches Ergebnis" appears only if `represent("W-S","plan",new)["primitives"] == represent("W-S","plan",base)["primitives"]`; otherwise the changed element is redrawn. Truthful lesson: the model decides which views change.

A thin DATA sweep line travels once across all four cells at the same time (the "regenerate" cue) when the tracker starts.

Meaning at the same time: every readout number sits next to the drawing element it measures (dimension in section, band height in elevation, hatch in elevation, window in 3D); old values stay as MUTED reference, so change is visible as difference.

### Beat 9 `copies` (anim ≈ 3.6 s)

Transition (0.9 s): board collapses into the pair layout. The live elevation is rebuilt at scale 0.20 (rev 2, GOOD frame, badge `rev 2`, label "linked view / verknüpfte Ansicht"). Right: a static elevation from `base` (rev 1) in a WARN dashed frame with badge `rev 1` and label "copy / Kopie".

- On the copy a DATA dashed outline shows where the ribbon **should** be according to the model (`op` of `new`), with the copy's own band in WARN; below both: sill labels, left `new` sill, right `base` sill (two model values, no arithmetic).
- A WARN "≠" sits between them.
- Below the copy: three stale mini thumbnails (plan, section, 3D from `base`, WARN frames, badge `rev 1`, WARN question marks), each with a curved arrow "by hand / von Hand" pointing back at the live view — the manual coordination the narration names. The plan thumbnail gets a GOOD tick instead (it was not affected), underlining that the model decides, humans cannot see that from a copy.
- Left bottom: GOOD chip "regenerated · no action / neu erzeugt · keine Handlung".

Animation: copy appears first and freezes, the mismatch outline draws, "≠" pops, the three thumbnails stagger in with their arrows.

### Beat 10 `identity` (anim ≈ 3.2 s)

Transition (1.0 s): pair layout collapses back to the **sequence layout**: H = `villa_axon` of `new` (taller ribbon visible), R2 = all four slots filled with `new` thumbnails (rev 2), R3 = the unchanged object card, R1 = the chip with badges `rev 1`, `rev 2`.

On screen: below the chip (R1 area extended to y [2.0, 3.1]) a two-row identity comparison: "rev 1 · `element("W-S", base)["guid"]`" and "rev 2 · `element("W-S", new)["guid"]`", the strings identical and aligned character by character; a GOOD "=" between and a GOOD tick. Leader lines from the chip to each of the four slots flash in turn (plan, section, elevation, 3D) and then to the axon.

Animation: slots fill in sequence, leaders flash, the two GlobalId strings align and the tick pops, `Indicate(chip, GOOD)` at the end.

## 4. Quality-bar checklist

| Bar | How this storyboard meets it |
| --- | --- |
| No bare box | the wall is the Villa Savoye south wall in a recognisable villa (pilotis, dark-green ground floor, ramp, solarium screens) in every view |
| Value = meaning | each number is drawn at the same time: dimension lines (plan, section, elevation), hatches (areas), bar (share), highlighted window (sill, head), chain of relations (void/fill), GlobalId chip tied by leaders |
| Moving values | `ValueTracker` + `always_redraw` for counters, hatches, extrusion, and the sill event |
| Numbers from the model | all through `quantity`, `quantities`, `building_geometry`, `represent`, `information`, `global_id`, `apply_event` |
| IFC names | `IfcWall`, `IfcWindow`, `IfcOpeningElement`, `IfcRelVoidsElement`, `IfcRelFillsElement`, `Axis`/`FootPrint`/`Body`, `SweptSolid`, `IfcExtrudedAreaSolid`, `Pset_WallCommon.IsExternal/LoadBearing`, `GlobalId`; plans/sections are derived (research-ifc 7, 14.9, 15.1) |
| Teaching model | said once, caption of beat 1 |
| Captions | 85-108 characters, no numbers |

## 5. Requests to the model and toolkit agents

Model (`api.md` section 2) — all additive; please confirm or adjust in `api.md`:

| id | request |
| --- | --- |
| M1 | `building_geometry()["walls"][i]["openings"][j]` `sill` and `head` and wall `base`/`top` as **absolute elevations above ±0.00** (or document the reference) |
| M2 | `represent("W-S", "plan")["derived"]` holds `cut_z` (absolute m, inside the window zone); `represent(.., "section"/"elevation"/"3d")["derived"]` may repeat it as `cut_z`/`view` |
| M3 | `represent("W-S", "axis")` -> `identifier "Axis"`, `type "Curve2D"`; `"footprint"` -> `"FootPrint"`, `"Curve2D"`; `"body"` -> `"Body"`, `"SweptSolid"` with `derived: {profile: "IfcRectangleProfileDef", solid: "IfcExtrudedAreaSolid", depth}`; primitives in metres (plan polylines / polygon; body edges in 3D) |
| M4 | `quantities("W-S")` returns `gross_side_area`, `opening_area`, `net_side_area`, `glazing_share` (fraction 0..1), `volume`; `quantity(element, "length"/"height"/"thickness")` for wall values |
| M5 | one authored demo event for this clip, e.g. `design_events["opening-lowered"]` in the JSON or `design_event("opening-lowered", model) -> {"type":"OpeningResized","object_guid":<RW-S guid>,"sill":…,"head":…}`; sill lower than now, head unchanged, `basis: assumed`. Scenes must not invent the number |
| M6 | `apply_event` keeps all GlobalIds and raises `revision` by 1; `element("W-S", new)["guid"] == element("W-S", base)["guid"]` |
| M7 | `relations_of("W-S")` exposes the Voids and Fills entries with the other side's tag and class (opening and window) |
| M8 | `information("W-S", "IsExternal"/"LoadBearing", lang)` works for walls (`Pset_WallCommon`) |

Toolkit (`api.md` section 3) — MUST unless marked:

| id | request |
| --- | --- |
| T1 | `villa_plan/section/elevation/axon` accept `highlight=("W-S","RW-S")` with fixed colours (wall ARCH, window GLASS); `labels=False`, `grid=False` give thumbnails; `parts[tag]` addressable; same widget at two scales has identical submobject structure (dock morph) |
| T2 | coordinate mapping on every drawing: `drawing.at(x, y=None, z=None)` -> screen point (plan (x,y), section (y,z), elevation (x,z), axon (x,y,z)) so dimension lines and the cut-level line are placed from model coordinates |
| T3 | `element_axon(tag, point, scale, *, model, extrude=1.0, opening=1.0, window=1.0)` — isolated wall solid with void block and window, three 0..1 parameters (see beat 5; fallback described there) |
| T4 | `hatch(polygon, color, spacing)` plus a polygon clip helper (clip at x or by rectangle) so hatches can be redrawn per frame; same hatch spacing in all views |
| T5 | `draw_primitives(primitives, point, scale, *, mode="plan"|"axon", color, dashed=False)` for the Axis / FootPrint rows |
| T6 | SHOULD: `wall_detail(tag, point, scale, *, model)` — plan detail of the wall cut at the window with layer set and thickness dimension |
| T7 | `dimension()` and the widgets are cheap enough for `always_redraw` over ~120 frames x 4 views in beat 8; otherwise the scene quantises the tracker to 24 steps |
| T8 | `guid_chip`, `badge`, `property_table` as in api.md; `property_table` supports a header line and 18-20 pt rows in 4.9 x 2.4 |

## 6. Open items

1. `Qto_WallBaseQuantities.GrossSideArea / NetSideArea` are source tier [D] in research-ifc section 4; verify against IFC 4.3.2.0 before using the sublabels (the rows work without them).
2. `IfcExtrudedAreaSolid` + `IfcRectangleProfileDef` for the wall body is a modelling choice [M]; the chip says "modelled as", not "always".
3. The `FootPrint` representation type string is whatever `represent()` returns; the model agent should keep it consistent with research-ifc section 7.
4. The south ribbon window is the only one with strong historical support (research-villa 4.4, open question 8); the other three ribbon windows are teaching-model simplifications and are never shown highlighted in this clip.
