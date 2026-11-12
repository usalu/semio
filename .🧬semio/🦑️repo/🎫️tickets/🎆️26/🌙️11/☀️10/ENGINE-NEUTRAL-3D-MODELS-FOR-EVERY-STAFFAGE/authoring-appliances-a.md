# Authoring: appliances (a)

12 assets of `🔌️appliances`, each with a hand-authored, spliced and validated `volume`. All default gates pass (bounding box within 8 % per axis, IoU at least 0.80 for elevation, front and plan). Validator issues: 0 for every asset.

Sources (kept): `volumes/appliances/<slug>.json` (authored volumes), `authoring-appliances-a/<slug with underscores>.py` (one explicit composing script per asset, helpers in `lib.py`), `authoring-appliances-a/{run.sh,validate.ts,posecheck.ts,frames.ts,posetest.ts,show.py}`. Tool output: `🗑️generated/authoring-appliances-a/`.

Bounding box error is relative, x (widthM) / y (heightM) / z (depthM). IoU is elevation / front / plan at 1:50. Poses checked: every asset pose plus synthetic joint poses (see remarks): the solids are carried with the evaluated bone frames (`restFrames` to the pose frames), AABB-connectivity must stay one component.

| slug | solids | bbox error % x / y / z | IoU e / f / p | validator issues | poses checked | remarks |
|---|---|---|---|---|---|---|
| ceiling-diffuser | 8 | +0.0 / +0.0 / +0.0 | 0.998 / 0.998 / 1.000 | 0 | off, half, design | one rigid bone. Hollow collar as ring extrude; square cones as lofts; asset-level ajv reports a pre-existing `thermal.regions[0].operating.heatW` minimum error outside the volume (not touched). |
| circulation-pump | 18 | +0.0 / -0.2 / +0.0 | 0.985 / 0.951 / 0.989 | 0 | off, low, high, rotor 90, rotor 200 | 2D inconsistency: the terminal box is drawn 5 mm further out in the front view (z -0.055..-0.005) than in plan (-0.05..0); plan followed. The 2D leaves a 4.5 mm gap between motor and terminal box; a neck block closes it. |
| decentralised-hrv-fan | 15 | +0.0 / +0.0 / +0.6 | 0.889 / 0.999 / 0.999 | 0 | off, extract, supply (rotor 720), rotor 37 | depth of core and rotor not in any 2D view: chosen from the elevation painting order (see decisions). Honeycomb has through holes, hence elevation IoU 0.889 against the drawn opaque opening. |
| desktop-monitor | 10 | -0.5 / -0.0 / +0.0 | 0.984 / 0.983 / 0.951 | 0 | on, standby, off, panel +20, panel -5 | plan draws the rear hump as wide as the bezel (0.54 m) and does not account for the 5 degree tilt; hump modelled narrower (0.53 to 0.44 m), plan IoU 0.951. |
| duct-fan | 14 | +0.0 / +0.0 / +1.1 | 0.827 / 0.997 / 0.997 | 0 | off, low, high | opening is a true through void with 4 motor struts at the rear; elevation IoU 0.827 because the 2D paints the opening as a filled dark disc (above the gate, close to it). |
| electric-fan-heater | 19 | +0.2 / +0.0 / +0.8 | 0.958 / 0.996 / 0.988 | 0 | off, heating-low, heating | knob/dial on the -z side face; mode switch exists only in elevation (relief of 4 mm assumed). Housing is drawn 2.5 mm off-centre in z (-0.115..0.12) in plan and front; kept. |
| electric-kettle | 9 | -1.6 / -1.3 / -1.0 | 0.992 / 0.994 / 0.991 | 0 | off, heating, boiling | rest height 0.237 m; the boiling pose lifts the lid 3 mm to 0.240 m (checked). Width 0.241 m against `widthM` 0.245: the 2D extremes (spout -0.116, handle +0.125) sum to 0.241. Hollow tapered shell with water inside. |
| electricity-meter | 15 | +0.0 / +0.0 / +0.4 | 0.997 / 0.999 / 0.997 | 0 | idle, half-load, full-load | none. |
| gas-boiler | 28 | +0.0 / +0.0 / +0.1 | 1.000 / 0.996 / 0.997 | 0 | off, heating, hot-water | flames slide by design into the 25 mm front plate in off/heating (AABB gap burner-tube/flame is the retraction, flames are hidden inside the solid plate). |
| gas-hob | 13 | +0.0 / +0.0 / +0.0 | 0.978 / 0.966 / 1.000 | 0 | off, simmer, high, flame-body 20 | 2D inconsistency: the knob is a front-panel end-on circle at y 0.019 in elevation (and the rig axis is w), a circle at z -0.2 in plan and a block at y 0.04..0.052 in front. Elevation reading kept; the front view IoU misses the drawn block (0.00027 m2). Off pose retracts flames into the housing by design. |
| gas-meter | 19 | +0.0 / +0.0 / +0.9 | 0.993 / 0.987 / 0.998 | 0 | off, low-flow, rated-flow | 2D inconsistency: elevation `plate-line-2` is drawn at y -0.115, 5 mm below the housing bottom (-0.11); placed at -0.106 so the height stays 0.27 m. |
| hot-water-cylinder | 22 | +0.0 / +0.0 / +0.4 | 0.996 / 0.996 / 0.955 | 0 | cold, half-charged, charged, needle 85 | 2D inconsistency: elevation and front draw four 0.07 m feet at (+-0.215, +-0.215), whose outer corners lie outside the round r 0.3 jacket, but the plan omits them (plan IoU 0.955). Hex nut is 0.038 high in front, 0.034 in elevation; 0.038 used. |

All 12 assets are done; no asset misses a gate.

## Things a reviewer should look at

- Depth stacks that are not in any 2D view were decided: HRV fan (impeller z -0.0975..-0.0865, core z -0.07..-0.02), duct fan (blades z -0.0285..-0.0185, hub), boiler burner recess (25 mm front plate with a hole), kettle (water inside a hollow shell), heater mode-switch relief. They are documented in each `volume.decisions`.
- Duct fan elevation IoU 0.827 and HRV fan 0.889 are the lowest; both come from keeping the physically open ring opening against a drawn filled disc.
- `authoring-requests.md`: three notes (closed-loop revolve profiles rejected by `volume-profile-self-intersecting` although the kernel needs them for hollow bodies; `lateralScale` axis differs between schema text and kernel for axes v and w; `volume-check.ts` tessellates loft sections raw while `modelOf` fills a path-following rotation). Workarounds: ring extrudes with holes instead of loop revolves, `lateralScale` only about u (not used at all in this set), explicit `rotation` on every loft section.
- Hollow cones (kettle shell) use an extrude taper, which the kernel scales about the outer ring centroid (same as the schema for centred rings).
