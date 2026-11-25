# Python clip chrome stills (low quality, after the chip change)

## Method
- Real scene route: `python-scene-stills.py <CHAPTER> en` runs the poster route's own command (`manim -ql -s --save_last_frame`, 854x480 at 15 fps, `--from_animation_number 0,<n>` where the poster declares one) on the chapter's poster segment, under `lockf -k /private/tmp/claude-501/semio-manim.lock timeout 400`. It writes only under `🗑️generated/stills/`, never into `🖼️posters`.
- Owner-helper route: `python-chrome-gallery.py` (`Gallery` scene: `card_grid`, `watt_anchor`, `window_card`, `chip`, `window_frame` with `top_chips`/`bottom_chips`, `frame_chips`, light `window_frame`, BIM `panels.frame_box`) rendered with `manim -ql -s` plus `SEMIO_TEACHING_LANGUAGE=en`, `SEMIO_TEACHING_AUDIENCE=speaker`, `SEMIO_TEACHING_PROFILE=final`.
- Queue runners: `python-stills-run.sh`, `python-chrome-gallery-run.sh`. Logs: `🗑️generated/still-<CHAPTER>.txt`.
- Python: course env `.🧬semio/🎓️teaching/⚡️cache/python` (cpython 3.12, manim 0.22.0). No source file edited, no git write, no cargo, no pkill.

## Results per requested scene

| Requested scene | Chapter (script, scene) | Result | Still |
| --- | --- | --- | --- |
| BIM objects | `BIM_02_Objects` (`🏗️bim/🎬️clip/🧬️objects`, Beat2_ObjectOrientation) | FAIL: exit 124, 400 s timeout, no frame | none |
| BIM representations | `BIM_03_Representations` (Beat3_SameObject) | FAIL: exit 124 | none |
| BIM exchange | `BIM_05_Exchange` (Beat5_Interfaces) | FAIL: exit 124 | none |
| Capsule cards/table | `Grasshopper_02_Capsule` (Beat2_Capsule) | FAIL: exit 124 | none |
| Nakagin | no own clip; the Nakagin chrome (`_chip`, `🏢️nakagin/🎨️visuals`) is drawn by the Grasshopper clips; proxy `Grasshopper_03_Tower` (Beat3_Tower) | FAIL: exit 124 | none |
| Trees | `Grasshopper_05_Trees` (Beat5_DataTrees) | FAIL: exit 124 | none |
| Casting | `Grasshopper_04_Casting` (Beat4_Casting) | FAIL: killed at 400 s, log ends at the first FadeIn, no exit line written | none |
| Optimization | `Optimization_Extrema` (`📉️extrema`, Beat1_LocalGlobalExtrema) | PASS, real scene | `evidence/python/optimization-extrema.png` |
| Cooling internal gains | `Cooling_02_InternalGains` (`👥️internal-gains`, Beat5_Mitigation) | PASS, real scene | `evidence/python/cooling-internal-gains.png` |
| Primary energy | `EnergyBalance_05_Primaerenergie` (`🏭️primary-energy`, Beat3_Rechenbeispiel, animation 28) | FAIL: exit 1 (see errors) | none |
| Excel references | `Excel_02_References` (`📍️references`, Beat2_References) | FAIL: exit 124 | none |
| Shared owners (driver) | `python-chrome-gallery.py` | PASS, renders | `evidence/python/chrome-owner-samples.png` |

## Verdicts

### Optimization_Extrema (real scene): PASS with one defect
- Title chip "Direction, domain and scope" is a square red chip with text readable.
- Defect (layout, not glass): the orange line "≈ 220.5 CHF/a" sits directly under "θ* = 360° · 882 kWh/a" with no leading, the lines nearly touch. Source is the result annotation next to the thumbnail chip, not a chip edge.
- Thumbnail (light picture card) has square corners and no label overlap.

### Cooling_02_InternalGains (real scene): PASS, inconclusive on glass
- Title chip "Mitigation & smart design" is square, readable, and drawn as a full box. It is a free label chip, not an attached frame chip, so the inner-edge rule does not apply.
- "Smart load control" panel: header text is inside the rectangle, no separate header chip is visible at 854x480. Cannot confirm glass-vs-body difference or inner-edge-only drawing at this resolution. Needs a higher-resolution still to decide.
- No label overlap; VDI reference top-right is low contrast red on dark but legible.

### chrome-owner-samples (driver, shared owners): PASS
- Card headers ("Power · Leistung", "Area", "Heat", "Solar") are square chips inside their rectangle; body text readable.
- Picture frame: two top chips and one bottom chip over the teal picture, one divider between abutting chips, visibly translucent.
- Light card (white body, grey header chip): header distinct, no text lost.
- BIM `frame_box` (dark, grey header chip): square, header chip distinct.
- No label escapes its rectangle.

## Errors (exact)
- `EnergyBalance_05_Primaerenergie`: `RuntimeError: Staffage export failed: asset heat-pump-outdoor-unit has no clip heating-sweep; clips: heating-modulation, defrost-sequence` (the staffage exporter, not chrome code; the chapter cannot render in this checkout).
- Timeouts (exit 124 at 400 s, real scene): `BIM_02_Objects`, `BIM_03_Representations`, `BIM_05_Exchange`, `Grasshopper_02_Capsule`, `Grasshopper_03_Tower`, `Grasshopper_05_Trees`, `Excel_02_References`; `Grasshopper_04_Casting` killed at 400 s.
- Gallery first attempt: `ValueError: Set SEMIO_TEACHING_LANGUAGE to en or de` (environment only; rerun with the variables set passed).

## Not verified (open)
- Capsule cards/table, BIM objects/representations/exchange, trees, casting, Excel references, and primary energy have no still of their own call sites. The driver covers only the shared owners (frame, chips, card, frame_box). Per-scene chrome drivers for those call sites (`_light_card`, representations info table, `_chip`/`_storey_chip`, casting plates, trees plate, excel references card, primary-energy inset) are still needed.
- Nakagin: no own clip; checked only through the Grasshopper tower proxy, which timed out.
- Glass vs body at a level that proves "visibly different translucent glass" and "inner edges only" needs a larger still than 854x480.
