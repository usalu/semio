# R2 audit ha1: appliances (authoring rules)

Scope: 11 assets in `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances`: circulation-pump, decentralised-hrv-fan, desktop-monitor, duct-fan, electric-fan-heater, electric-kettle, evaporative-humidifier, gas-boiler, gas-hob, hot-water-cylinder, kitchen-extractor-hood.

Method: structural audit of every asset (`T/🧰️ha1/audit.py`, `table.py`, `styles_vs_pens.py`, `bilingual.py`, `shape_ext.py`, `outline.py`). Checked validRange, pens against the styles text, region coverage per scale, pose bindings (all regions and emitters), clip and pose counts, mass sums, en/de completeness, unavailable 1:500 structure and dimension conventions (notes). Rendered previews were NOT reviewed in this pass, so visual judgements (reading, overlaps, hatching) are open.

## Checked and clean

- Every asset: `validRange` 20-75 / 75-150 / 150-350 at 1:50 / 1:100 / 1:200; 1:500 `unavailable` with reason (en/de), limiting feature, size and alternatives.
- Every authored scale draws every thermal region; no drawn region is missing from `thermal.regions`; no duplicate shape ids.
- Every pose binds all regions, all non-breath emitters and all operating keys (`thermal`). Each asset has at least 3 poses and 2 clips.
- Bone masses sum to `dimensions.massKg` for all 11.
- Dimensions match the stated conventions. Side-elevation assets (desktop-monitor, kitchen-extractor-hood) state that `widthM`/`depthM` are the in-view width and the depth; the notes agree with the numbers (e.g. 0.9 m hood width = `depthM`).
- en/de complete everywhere (only numeric labels equal in both languages).
- Catalog check `✓` for each touched asset (see below).

## Fixed (edits made)

1. `electric-fan-heater/🔣️.json`: pens normalised to the appliance family. 1:50 body 0.35 to 0.18, detail 0.18 to 0.13, texture 0.13 to 0.1, ground 0.5 to 0.35; 1:100 body 0.25 to 0.18. The 0.35 body was the heaviest pen in the family and above the 0.13 to 0.18 body range of the authoring rule. The 1:50 and 1:100 styles text (en and de) follows. Also 1:200 body was 0.15 while its styles text said 0.18; the pen is now 0.18 to match the text.
2. `electric-fan-heater/🔣️.json` clip `warm-up`: event `kind` `ignition` to `operating`. An electric element has no ignition; `ignition` is for gas.
3. `electric-kettle/🔣️.json`: region id `window` (a water region titled "Water gauge and water") renamed to `water` (incl. `water-skin` layer, pose region keys, shape regions and the en/de decisions, thermal assumptions and region text that named it "window"/"Fenster"). Glazing wording "gauge window" kept because it is the physical sight window.
4. `electric-kettle`, `evaporative-humidifier`, `gas-hob`: styles text read "detail none mm" / "Detail keiner mm" at 1:100 and 1:200. Now "detail not drawn" / "Detail entfällt".

## Catalog check (touched assets, `repo tools exec bun -- .../🩺️check/🟦️.ts --asset <id>`)

- electric-kettle: `✓` (1:200, 1:100, 1:50 suitable under office-laser; 1:500 unavailable)
- electric-fan-heater: `✓` (same tiers)
- evaporative-humidifier: `✓` (same tiers)
- gas-hob: `✓` (same tiers)

Raw output: `🗑️generated/ha1/check/` (deleted at the end of this pass).

## Judgment items (not changed)

- **Kettle energy balance**: 2 kW element at level 1, but only 189 W is bound to surfaces (plate 90, shell 45, lid 14, water 40). The water is not a region, so the boil clip (150 s heating) cannot come from the thermal model. The notes state this openly. Decide whether water should become a region with a large area.
- **Kettle mass**: 2.92 kg includes 1.7 kg water. Consistent with the notes, but other appliances carry dry masses (desktop-monitor, fan heater). Confirm one convention.
- **Evaporative humidifier energy**: outlet-air 0.28 m3/s at -10 K is about 3.4 kW of cooling. The pad is -90 W "illustrative, not a rating" (notes). The energy balance is not closed. A consistent set would be pad about -3.4 kW, or a much smaller flow for a domestic unit.
- **Humidifier drip clip**: 1.2 s for a 0.36 m drop (free fall would take 0.27 s; the clip may be film flow along the pad, not free fall). It is non-looping. A loop is the natural setting, but the cast callers depend on the index `loop` flag, so I did not change it.
- **Hot-water cylinder**: title "200 L" and mass 275 kg (200 kg water), but 1.6 m high and 0.54 m jacket diameter. The geometry and volume do not match a 200 L product. Notes say "product class", so this is a product-choice decision (1.2 to 1.3 m height is more typical for 200 L).
- **Gas boiler**: the 20 kW input leaves in water and flue. The flue-gas emitter carries about 330 W (0.0072 m3/s at +38 K). Documented; confirm this is the intended share.
- **Gas hob**: no `plan` viewpoint. A domino hob is read from the top (burner layout, pan supports). This is the "plan where needed" case. Adding a plan at 1:50 and 1:100 would be the next step.
- **Looping clips**: loops exist for the circulation pump only. Candidates where a loop is meaningful: gas-hob `flicker` (keys return to the same pose), humidifier `drip`. Duct-fan, HRV, hood and boiler clips are one-shot sequences.
- **Pen family outliers at 1:200**: decentralised-hrv-fan, desktop-monitor and kitchen-extractor-hood use body 0.18 at 1:200 (family 0.13). circulation-pump, gas-boiler and hot-water-cylinder use ground 0.2 (family 0.25). Both are within range; I did not change them.
- **HRV emitter reference**: supply-in is -5 K and exhaust-out -21 K. The reference (room air or outdoor air) is not stated in the emitter, and the regime (winter) is only implied by the notes. Check against the notes.
- **Not visually reviewed**: no renders were inspected in this pass. The 1:200 pump (casing and motor merged), the 1:200 fan heater block and the 1:50 kettle spout need a look at print size.

## Still open

- Visual review (render, print size and zoom) of the four touched assets after the pen change, especially the fan heater 1:50 body line at 0.18 mm.
- The other seven assets were audited structurally only; their catalog checks were not re-run because I did not touch them.
- Gas hob plan view (judgment above).
