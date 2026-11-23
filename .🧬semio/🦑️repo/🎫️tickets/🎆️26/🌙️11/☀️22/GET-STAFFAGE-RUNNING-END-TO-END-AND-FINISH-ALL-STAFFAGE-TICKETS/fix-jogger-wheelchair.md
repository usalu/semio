# 🏃️ Fix of `jogger` and `wheelchair-user`

Session 2026-11-23, about 05:15 to 13:40 (cut once by a usage limit). Every line quoted below was run to the end in this session with the pinned bun, one process at a time. No rule, threshold, test or engine file was touched; only the two asset files changed.

## Result

| Asset | Before | After |
| --- | --- | --- |
| `jogger` | `✗` `clip run-cycle@0.180` / `@0.540: force residual 7.84e+2 N` | `✓ jogger (1/1)` |
| `wheelchair-user` | `✗` `push-cycle@0.000: -1.39e+3 N`, `ramp-roll@0.000: -1.19e+3 N` | `✓ wheelchair-user (1/1)` |
| validator `r3-validate.ts --file … --volumes --no-print`, both | | `VALID … schema 0, validateAsset 0, checkVolumes 0` (also `VALID` without `--volumes`, print lines included) |
| `🧊️catalog-silhouette`, `jogger` | fails (5 views or axes) | **still fails**: `jogger 1:100 front Hausdorff: expected 0.0456… to be less than or equal to 0.03` (4 views, see Open) |

## `jogger`

**Cause.** The run cycle was twelve poses 60 ms apart that were never solved against each other: on the 1/240 s grid the old clip slid or lifted off at 16 of 37 instants of one step, the ground force ran from -16 to +22 body weights, and the body landed with a velocity kink (22 body weights at t = 0). The `ballistic` flags were right; the stance was not a stance.

**Fix.** `run-cycle` is re-authored by a generator (`🧰️fix-two/jogger-run-cycle.ts`, its docstring states the model) and written into the asset as text:

- 24 key poses 1/32 s apart (12 per side, the right step mirrors the left). Stance 0.25 s, flight 0.125 s, cycle 0.75 s (was 0.72 s). The new times are whole multiples of the 1/240 s step of planted root motion; with 0.72 s the two flights inherited different pelvis speeds (2.498 and 2.482 m/s), which put 0.45 g of horizontal inertia on the loop seam.
- Centre of mass: vertical ground force `W·(0.6 + 1.41·sin(πs) + 0.31·sin(2πs))` in stance (peak 2.13 body weights, its impulse carries the weight), a parabola in flight (take-off 0.53 m/s, landing -0.69 m/s, toe-off 10 mm above landing); horizontal braking and propulsion `-0.2·W·sin(2πs)`; average speed 2.34 m/s as before.
- Stance keys: thigh, shank and foot solved so that the centre of mass is on the target over the planted ball; the sole is exactly flat while the heel span lasts (0 to 0.0625 s), then the heel rises to 40° at toe-off.
- Trunk lean solved per key (-2.9° to -10.3°, design value -5.4°), because planted root motion carries the pelvis through the flight at one speed; this keeps the centre of mass at its speed in the air.
- Four joint values around the flight are solved with the product's own clip sampling so that the root velocity is continuous at toe-off and landing (the foot leaves and meets the ground without speed over it).
- Contacts, phases and events follow the new times; `advanceM` 1.68312797 → 1.75484512. Named poses `initial-contact-l`, `midstance-l`, `flight-l` and all clip ids are kept; 12 key poses were added (`weight-acceptance`, `absorption`, `drive`, `push-off`, `flight-rise`, `flight-fall`, `-l`/`-r`), texts `{en, de}`.
- By hand: `dimensions.widthM` 0.984 → **0.962** (rule `extent-width`: the new default pose is 0.960 to 0.964 m wide; the manifest row says 0.984, inside its 10 %, not edited). Breathing pattern `run`: `rateBpm` 41.67 → 40, `tidalVolumeL` 1.4 → 1.46 and the assumption text (40 per minute, 2.7 Hz), so that one breath still spans two strides and the ventilation stays 58 L/min.

**Measured** (`🧰️fix-two/scan.ts`, 181 instants of the 1/240 s grid): all nine check samples have residual 0 (landing: support 0.32 W, midstance 2.10 W). 8 instants still slide: the two samples 4 and 8 ms after each landing (26 and 47 N) and the two 4 and 8 ms before each toe-off (176 and 106 N). There the modelled force steps to and from 0.6 W and a spline that is only continuous in velocity cannot follow a step.

**Sheets opened** (`r3-sheet.ts --file … --scale 100 --clip run-cycle --only right`, t = 0, 0.125, 0.25, 0.3125): it reads as a run. Flat landing ahead of the body with the other shank folded back; a deep midstance (knee about 64°) with the swing knee passing; toe-off on the ball with the other knee high and the arms opposed; flight with both shoes clear of the ground. The first render of the flight showed the tip of the trailing shoe at the ground line; the toes now stay bent for four keys after toe-off and the second render is clear.

## `wheelchair-user`

**Cause.** Two things. (1) As the previous fixer found, `push-cycle` and `ramp-roll` are steady rolling authored as one-shots, so every joint started from rest and the trunk needed 27 m/s² in the first key interval. (2) New: every push stroke switched from the gripping to the swinging arm between two keys (hand 66° in 1/32 s), 0.59 body weights of inertia at 0.25 s after each grip; the old check never sampled it (the scan shows it in the old file at 1.25 s: residual 9.85 N).

**Fix** (`🧰️fix-two/wheelchair-loops.ts`): both clips are loops with a closing key. Rear wheel (0.305 m) and castor (0.0678 m) turn 1 : 4.4985, so the shortest loop that closes both is two wheel turns and nine castor turns:

- `push-cycle`: 3 strokes of 1 s, 240° per stroke as before, 3 s (was 4 s), 97 keys, closing key wheel -720°, castor -3240°, `advanceM` 5.1103 → 3.8327 (4π·0.305). The repeated second stroke is used (the first began from another grip); the release of the rims is eased over six keys.
- `ramp-roll`: 4 strokes at the authored wheel speed of 192°/s with 180° per stroke, so a stroke lasts 0.9375 s and the loop 3.75 s (was 2 s), 121 keys, `advanceM` 2.0441 → 3.8327. Push keys and timing unchanged (hands stay on the rims), the recovery is sampled from the authored spline 8 % faster.
- Castor keys step 33.75° and 27° (were 33.7389° and 26.9912°): the castor rolls 0.03 % further than the wheel, 1.2 mm per loop. Tracks, phases and events repeat per stroke; descriptions `{en, de}` rewritten. Dimensions and mass unchanged.

**Measured**: residual 0 at every one of the 721 and 901 grid instants of the two loops; first and closing key identical for every body bone.

**Sheets opened** (`push-cycle` t = 0 and 0.3125): at 0 the hand grips the rim behind the top with the trunk upright; at 0.3125 the trunk leans forward and the hand has followed the rim to the front and is letting go. It reads as a push. The other three rendered sheets (0.1875, 0.625, ramp 0.1875) were not opened.

## Files

Updated: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/jogger/🔣️.json`, `…/wheelchair-user/🔣️.json`.

Created (ticket): `fix-jogger-wheelchair.md`, `🧰️fix-two/jogger-run-cycle.ts`, `wheelchair-loops.ts` (runs on the first authoring only, refuses a file that already loops), `scan.ts`, `limits.ts`. Removed: `🗑️generated/fix-two/`. The previous `🧰️fix-originals/jogger-run-cycle.ts` is superseded and was left in place.

## Open, stated plainly

- **`catalog-silhouette` is red for `jogger`** and I did not fix it: 1:100 front 46 mm (limit 30; the body has a gap at the crotch that the drawn shorts cover), 1:50 elevation 25 mm, front 28 mm, plan 28 mm (limit 15: fingers not drawn, shirt hem, shoe against sock, shoulder caps, hair against shirt). It is the same disagreement between drawing and volume as before the re-authoring (the old file: 5 problems). The existing repair tool `STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/REP-A/fit.ts` ran 9 minutes and found 19 patches, but 1:100 front stayed at 45.6 mm and 1:50 plan at 18.5 mm; nothing was applied. It needs about ten drawn shapes re-traced by hand.
- **`wheelchair-user` fails the same guard** (`1:200 elevation Hausdorff 0.1125 > 0.06`; 1:50 all three views). Its drawings, volumes and default pose were not touched here; I did not check the old file.
- Jogger: 8 of 181 instants slide (above); the heel dips 2.7 mm under the ground 15 ms before landing and the ball 3.0 mm for a moment after toe-off; in flight the vertical acceleration wanders between -0.45 and -1.25 g; midstance is deeper and the trunk swing (7°) larger than a real jog.
- Manifest not edited: `jogger` `widthM` 0.962 (row 0.984). `powerW` was not recomputed after the breathing change. Clip durations are not in the rows.
- Catalog-wide suites and the site build were not run. The repo ticket tools were not used (the `semio` MCP server did not connect).
