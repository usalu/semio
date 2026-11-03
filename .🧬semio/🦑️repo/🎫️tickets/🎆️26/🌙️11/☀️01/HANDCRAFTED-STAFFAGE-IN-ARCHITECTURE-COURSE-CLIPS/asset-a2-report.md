# Asset agent A2 report: standing-adult and person-cooking

S = `🎓️teaching/🏛️architecture/🧍️staffage`, T = this ticket folder. Both assets live in `S/🗂️catalog/👤️humans/<id>/🔣️.json` and have a row in `S/🗂️catalog/🔣️.json` (inserted before `standing-phone-user`, two minimal Edits).

## standing-adult (generic scale figure)

- 1.69 m, 59.2 kg, fleece sweatshirt, jeans, canvas sneakers (about 0.7 clo, normal indoor wear), no phone, headphones or ponytail. Default environment is 21 degrees C still air.
- Poses: `relaxed`, `arms-crossed`, `pointing` (near arm horizontal, 3 deg below), `hands-warming` (palms toward a heat source).
- Clips: `idle-breathing` (5 s loop), `shiver` (1.2 s loop, 8 alternating shakes of trunk, head and knees with the arms folded), `fanning` (2 s loop, near hand swings in front of the face).
- Rig and the four drawings per scale come from `standing-phone-user` (a transform by `T/a2/build-standing-adult.mjs`): shapes and regions of phone, headphones and ponytail were removed, and the 1:500 silhouette was re-listed without them. Poses and clips were designed by hand.
- Thermal binding: `baseMet` 1.2 and `activityMet` 1.2 (relaxed, arms-crossed, hands-warming), 1.3 (pointing, talking pattern), 1.4 (fanning clip track). The model has no hand-set shiver or sweat. The two-node model produces them from the environment. Probe `T/a2/probe.ts` solved the clips against catalog presets:

| clip | office-neutral | winter-overcast | summer-heatwave-calm |
|---|---|---|---|
| idle-breathing | 1.20 met, core 36.85, wet 0.06 | 2.61 met (shivering heat), skin 15.5 | 1.20 met, wet 0.54, 190 W latent |
| shiver | 1.20 met | 2.64 met, skin 15.9, core 36.56 | wet 0.55 |
| fanning | 1.40 met | 2.71 met | 1.40 met, core 37.53, wet 0.56, bf 90 |

  Use `shiver` with `winter-overcast` (or `winter-clear-night`) and `fanning` with `summer-heatwave-calm` or `summer-hot-sun`. Both clips pick their own breathing pattern (`shivering` 20 bpm, `warm` 18 bpm / 0.55 L).
- Respiration patterns: quiet-standing, talking, shivering, warm. All are `authored-estimate` (guided by ICRP 66 and Gupta 2010).
- Dimensions: height 1.69, width 0.328 (the four drawings measure 0.314 to 0.342 m in the relaxed pose, within the 15 mm tolerance), depth 0.4.

## person-cooking

- Standing-adult figure at a 0.90 m kitchen counter (0.60 m deep, plinth recess, worktop overhang), hob plate, a 0.22 m x 0.14 m pot with 2.5 L of water and a wooden spoon. Total mass 133.5 kg (person 59.2 kg, counter 70 kg, pot 4.2 kg, spoon 0.06 kg).
- Objects follow the courier-with-hand-truck pattern. A prismatic `counter-lift` bone under the pelvis floats the counter on the floor via the `counter-contact` anchor. `counter` and `pot` are fixed bones, and `spoon` is a revolute child of `hand-r`. So the counter belongs to the figure but does not load the person.
- Poses: `stirring` (1.8 met), `tasting` (1.6 met), `standing-by` (1.4 met, third pose for the rule of at least 3). Clips: `stirring` (2 s loop, spoon on an ellipse) and `stir-and-taste` (4 s loop, second clip for the rule of at least 2). Arm angles come from an analytic two-link solution (`reach` in `T/a2/build-person-cooking.mjs`) so the hand grips the spoon and the tip sits in the pot (stirring) or at the mouth (tasting).
- Thermal: `thermal.operating` key `hob` (off 0, simmer 0.4, boil 1). Region `pot` is an `operating` region with 220 W at level 1. Emitter `steam` (exhaust, 0.003 m3/s, +55 K, 0.12 kg vapour per kg, anchored at the pot mouth, driven by `hob`). Counter, hob and spoon are passive. All new values are `authored-estimate`. The cooking activity 1.6 to 2.0 met follows ISO 7730 Annex A. Default environment is a 22 degrees C kitchen.
- Dimensions: height 1.69, width 1.088, depth 0.62.

## Drawings (all four scales authored, all pass `office-laser`)

- 1:50 and 1:100: counter in separate parts (plinth, body, door panel, handle at 1:50, worktop, hob plate, burner ring at 1:50), pot with rim, handle and band, spoon with bowl.
- 1:200: counter body, worktop merged with the hob (both regions), pot as one block, spoon thickened to 32 mm so it stays printable. 1:500: one stepped column listing counter, hob, pot and spoon, with the spoon motion not drawn. Every decision text (en/de) was extended per scale.

## Verification

- `repo tools exec bun -- 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset standing-adult|person-cooking --samples 9|5` prints a check mark for both. The `--category humans` run also exits 0.
- Early failures I fixed: the shiver clip's linear keys gave a force residual (inertial spikes), so it now uses smooth keys with 8 shakes. Widths and heights now match within tolerance, and `person-cooking` has three poses and two clips.
- The vitest run in `S/📦️packages/🟦️typescript` (884 passed, 6 failed). Failures: ffmpeg MP4/WebM, qpdf/PDF reopen and PNG-zip tests (missing tools or timeouts on this machine) and the print-size file. `asset-validation` fails only on `category-missing` from another agent's new manifest category (`appliances`), not from these assets.
- Rendered with `S/🔨️modules/📤️export/🎬️cli/🟦️.ts` to svg (scale 25, plus 50/100/200/500 for the cooking figure) and converted with PyMuPDF (`rsvg-convert` is not installed on this host): all 4 + 3 poses in dark-line, dark-solid and thermal, and clip frames of idle-breathing, shiver and fanning, all read in the Read tool. The generated folder was deleted afterwards.

## Remaining doubts

- `arms-crossed` in a side profile reads as hands folded at the belly rather than strictly crossed (planar rig, forearm flexion limited to 150 degrees). It still shows hands tucked in a protective pose. `pointing` has an extended hand blob but no separate index finger at 1:50.
- The figure is the archive's female de Leva table (1.69 m); clothing and silhouette are neutral. The steam is a transport emitter only and is not drawn in the line or solid views.
- The cast contract's `scale` field and the thermal presets live outside the asset, so `shiver` and `fanning` only become warm or sweaty when the cast chooses the cold or hot environment preset named above.

## Files

- `S/🗂️catalog/👤️humans/standing-adult/🔣️.json`, `S/🗂️catalog/👤️humans/person-cooking/🔣️.json`, `S/🗂️catalog/🔣️.json` (two rows).
- `T/a2/build-standing-adult.mjs`, `T/a2/build-person-cooking.mjs`, `T/a2/patch-cooking.mjs`, `T/a2/probe.ts`, `T/a2/render.sh`, `T/a2/sheet.py` (scratch generators and viewers).
