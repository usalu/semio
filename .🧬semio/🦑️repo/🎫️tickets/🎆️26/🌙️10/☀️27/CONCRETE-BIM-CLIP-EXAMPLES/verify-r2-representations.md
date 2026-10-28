# Verification: Beat3_SameObject (One Object, Many Views), Round 2 and Round 3

Verdict: **FAIL**

Defects: 1 blocker, 3 major, 11 minor.

Scope: clip `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/👁️representations/🐍️.py`, class `Beat3_SameObject`, all 18 narration keys in English and German (36 PNGs read). Frames are current: scene file written 20:59, keyframes 21:00 to 21:02, so no re-capture was needed. Crops used for zoom checks are in `T/🗑️generated/verify-representations/`. Values were checked against `🧱️model/🔣️.json` (with its basis flags), `T/research-villa-savoye.md` and `T/research-ifc.md`. No source file was edited.

---

## Defects

### Blocker

**B1. Printed arithmetic does not hold (volume beat)**
- Key: `volume`. Frames `en-volume`, `de-volume`.
- Location: `_beat_volume`, scene lines 1005 to 1006. The formula line prints the volume with 3 digits, while the void and wall values are printed with 2 digits.
- On screen: "19.35 - 7.22 = 12.135 m³" (DE: "19,35 - 7,22 = 12,135 m³"). 19.35 − 7.22 = 12.13, not 12.135. The wall label reads "12.13 m³" while the line above it reads 12.135. The void is 24.05 × 0.30 = 7.215, displayed as 7.22.
- Fix: print every operand and result with the same digits and rounding rule, for example "19.355 − 7.215 = 12.135" or "19.35 − 7.22 = 12.13" with the wall label also 12.13. Compute the printed result from the printed operands, and use one rounding rule everywhere in the clip.

### Major

**M1. Assumed dimensions shown as facts, without the assumption mark**
- Keys: `source`, `derived`, `takeoff`.
- Locations:
  - `_beat_source`, line 342: "18.50 m × 1.10 m" (window) and line 345: "21.50 m × 3.00 m" (wall), no `_basis(...)`.
  - `_beat_derived`, lines 682 to 683: "21.50 m × 3.00 m", "18.50 m × 1.10 m", and the areas 64.50 / 20.35 / 44.15 m², no star.
  - `_beat_takeoff`, lines 958 to 959: "18.50 × 1.30 = 24.05" formula, no star.
  - The `source` frame has no "* assumed in the teaching model" footnote at all.
- Basis: `/elements[31]/attributes` start, end, sill and head are all `assumed` in the model. Research fact 4.5 states that ribbon window length, height and sill were not found. The wall height depends on assumed storey levels (`/storeys` basis `assumed`).
- Fix: append `self._basis(...)` to the window and wall dimensions in `_beat_source` and to the derived and takeoff formulas. Add the footnote to the `source` frame in both languages.

**M2. Object card shows translated labels, not IFC property names**
- Keys: every frame with the object card (all 18).
- Location: `_card`, lines 240 to 254. Rows take `item["label"]` from `_model.information(...)`, so the screen reads "External", "Load-bearing", "Fire rating". The model itself carries `Pset_WallCommon.IsExternal`, `Pset_WallCommon.LoadBearing` and `Pset_WallCommon.FireRating`.
- Rule: "IFC names are real IFC 4.3 names (`Pset_ColumnCommon.LoadBearing` …)".
- Fix: show the property names (`Pset_WallCommon.IsExternal`, `Pset_WallCommon.LoadBearing`, `Pset_WallCommon.FireRating`) in the row label, with the translated label as a secondary text if wanted. Same in DE.

**M3. FootPrint drawn as a filled area but labelled as a curve**
- Keys: `stored`, `lod`, `scale200`.
- Location: `_row_footprint`, lines 583 to 598. The footprint is a filled polygon (`_drafting.polygon(..., ARCH, 0.45 ...)`) plus a hatch. The label in `_beat_stored` says `Curve2D` "outline in plan"; `lod` says "Curve2D outline".
- Research: `T/research-ifc.md` section 7 lists FootPrint as a 2D projection, and open item 16.2 and 16.3 are still unverified for the RepresentationType. A filled 2D area and a 2D curve are different representation types.
- Fix: verify FootPrint RepresentationType against IFC 4.3.2.0. Then either draw only the outline with `Curve2D`, or relabel the filled region as `Surface2D`. Apply the same wording in `stored`, `lod` and `scale200`.

### Minor

**m1. Star marks inconsistent for assumed levels and layers**
- `_cut_key`, line 415: "head +5.50", "sill +4.40", "cut +4.95" have no star. The model levels are assumed (`/storeys` basis `assumed`), and the section beat marks "+3.50*" and "+6.70*".
- `_board_section`, line 725: the update and copies sections show "+3.50" and "+6.70" unmarked.
- `_beat_sweep`, line 916: `_basis('sill')` is appended after the head value, so the star lands on "head 2.00 m*". The assumed sill "0.90 m" has no star.
- `_layer_rows`, line 811 (used by `lod` and `scale20`): layer thicknesses 0.03 / 0.24 / 0.03 m are assumed, but the layer legend has no star (the plan detail does).
- "+9.40" is sourced as box height, but the ±0.00 reference is assumed and research fact 2.2 says the reference level is not stated.
- Fix: one rule. Every value whose basis is `assumed` gets the star at the value it marks, in every beat.

**m2. No title on any keyframe**
- `TITLE` is defined at scene line 45 ("One Object, Many Views") but no frame draws it. The top band is empty in all 36 PNGs. The energy clips show a section heading. The notes in `T/scene-representations.md` say the shared `present` stopped drawing the title.
- Fix: restore the title in the shared `present` or draw it in the scene, in both languages.

**m3. Sweep magnifier: wall poché overruns its frame**
- `_cut_plan`, lines 881 to 899, used by `_beat_sweep`. Frames `en-sweep`, `de-sweep`: the yellow hatched wall continues about 14 px (at 1×) past the right edge of the magnifier frame.
- Fix: clip the strip to the frame, or enlarge the frame so the poché ends inside it.

**m4. Dimension tick runs into the first glyph of its label**
- Copies: `_beat_copies`, lines 1083 to 1084 (`offset=-0.12`). The "0.70 m*" (rev 2) and "0.90 m*" (rev 1) labels are crossed by the tick.
- Stored: `_row_footprint` / `_beat_stored`, line 595 (`offset=-0.2`). The "3.00 m*" label is crossed by the tick.
- Elevation: line 506. The "1.10 m*" and "0.90 m*" ticks touch the "*".
- Fix: move the text a fixed distance past the tick, or place the label on the far side of the tick.

**m5. English legend shows square brackets around "white"**
- Model name `🧱️model/🔣️.json` line 463 is "Render (white)". In the EN keyframes (`en-lod`, `en-scale20`, `en-plan`) it renders as "Render [white]". The DE frames render "Außenputz (weiß)" with round brackets.
- Fix: find the EN text path (font fallback or string replacement) and render round brackets, as in DE.

**m6. "The lowered sill shrinks it" is not shown with a before value**
- `_beat_volume`, lines 986 to 1011. The narration (`volume`, EN and DE) says the lowered sill shrinks the wall volume. The frame shows only the rev 2 values (wall 12.13 m³, void 7.22 m³). The rev 1 wall volume is not printed, though `takeoff` shows rev 1 values for the same kind of quantity.
- Fix: show the rev 1 wall volume next to the rev 2 value, as `takeoff` does.

**m7. Gross and net wall hatches coincide in the final keyframe**
- `_area_hatch`, line 639, and `_beat_derived`, lines 666 to 699. Both the gross wall and the net wall hatch cover the same wall, so in the final frame one olive hatch shows and the net area cannot be told apart from the gross area. The legend shows two different hatches.
- Fix: draw net wall as its own region (the wall minus the opening), or remove the gross hatch once the net hatch is shown.

**m8. German basis word "belegt" is ambiguous**
- Object card, DE, "Tragend FALSE belegt" (`_card` / `_model.information`). "belegt" reads as "occupied" to most German readers. It is meant as "sourced".
- Fix: use "Quelle" or "gesichert" for `sourced` in the DE basis mapping.

**m9. LOD values in mm do not state their scale**
- `_beat_lod`, lines 822 to 841. The legend prints "0.03 m → 1.5 mm" and "0.24 m → 12.0 mm", which is a 1:20 conversion, but the beat never says so. `scale20` does state 1:20.
- Fix: add a "1:20" label to the `lod` layer legend, or state the scale in the caption.

**m10. Ribbon bands on other façades are drawn without a note**
- `source`, `three` and `identity` show teal ribbon bands on the west and east façades too (model `OP-E`, `OP-W`, `OP-N`; `🔣️.json` elements 32 to 34). The caption and narration speak of the south wall only. Research facts 4.4 and 10.5 say only the south façade is documented; the extent of ribbon windows is disputed.
- Fix: add a short teaching-model note in `source`, or draw the other façades without the ribbon, as the documented facts say.

**m11. German terms**
- Section label "Grundrissschnitt +4,95" (`_cut_plan` / section, scene line 488). The standard term is "Schnittebene".
- The event name "OpeningResized" is shown untranslated in DE in `update` and `takeoff` (`_beat_update`, `_beat_takeoff`). Give the DE label, for example "Öffnung geändert", and keep the identifier in brackets if needed.

---

## Checked and clear

- Units and scale: at 1:200, 0.30 m → 1.5 mm, 21.50 m → 107.5 mm, 0.03 m → 0.15 mm; at 1:20, 0.30 m → 15 mm, layers 1.5 / 12.0 / 1.5 mm. All consistent in EN and DE.
- Areas: 21.50 × 3.00 = 64.50 m²; 18.50 × 1.10 = 20.35 m²; 64.50 − 20.35 = 44.15 m²; 20.35 ÷ 64.50 = 31.6 %. Revision 2: 18.50 × 1.30 = 24.05 m²; 40.45 m²; 24.05 ÷ 64.50 = 37.3 %. All print correctly.
- Levels: +3.50 + 0.90 = +4.40 (sill); +4.40 + 1.10 = +5.50 (head); cut at mid-window +4.95 and +4.85 after the sill change to 0.70 m (+4.20). The section chain 1.00 + 1.10 + 0.90 = 3.00 m. Elevation window 18.50 m with 1.50 m margins each side of 21.50 m.
- Plan detail: 0.30 m thickness, 1.50 m pier, layer order and values consistent with the model.
- IFC: `Axis` / `Curve2D`, `Body` / `SweptSolid`, `IfcExtrudedAreaSolid` over `IfcRectangleProfileDef`; chain wall → `IfcRelVoidsElement` → `IfcOpeningElement` → `IfcRelFillsElement` → `IfcWindow` matches research sections 6 and 7. GlobalId `3fNY9OuTHQ5QVew2$dtdIV` has 22 characters from the valid alphabet, and is the same in rev 1 and rev 2.
- Captions: all within two lines; nothing overlaps the caption band.
- Layout tool: "stroke through chip of 31.6%" is a false positive, because the chip is an opaque plate over the window band. The "glass line" touch with the detail title is 0.01 units and not visible. The "18.50 m*" line hit is the dimension line, which does not touch the label.
- German: decimal commas and thin spaces are correct in all DE frames; "%" is spaced in DE; no English text left in DE except the items in m11.
- Narration against picture: each clause matches the frame it runs over, with the exception of m6.
- Depth against the energy clips: each beat carries a worked numeric chain, comparable to the energy beats (for example "1.2 times 1005 = 1206 joules"). The villa example is concrete in every beat.

## Not verifiable from keyframes

- `sweep`: capture jumps to the final cut height (1.45 m). The 0.50 m solid state narrated in the clip is not in any keyframe. The same holds for the intermediate states of `takeoff`, `volume` and `update`.
- Card values "External", "Load-bearing" and "Fire rating" have no drawn counterpart on the villa in any frame; this is a persistent data card and is not counted as a defect.

---

## Counts

| Severity | Count | Keys |
| --- | --- | --- |
| Blocker | 1 | volume |
| Major | 3 | source, derived, takeoff, stored, lod, scale200, all (card) |
| Minor | 11 | plan, section, update, copies, stored, lod, scale20, sweep, volume, derived, source, three, identity, all |
