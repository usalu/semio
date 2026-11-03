# Asset agent A4: toaster, vacuum cleaner, electric fan heater

Assets (category `appliances`, folder `S/🗂️catalog/🔌️appliances/`): `toaster`, `vacuum-cleaner`, `electric-fan-heater`. Manifest rows were appended at the end of `entries` in `S/🗂️catalog/🔣️.json`. Scratch generator scripts (hand-designed shapes, computed hose chain) are in `T/🧰️a4/` (`build.mjs <id>` rewrites an asset JSON; `render.sh`, `sheet.mjs` render figure-only cast sheets).

## Heat mechanism (no second mechanism)
The existing thermal model carries all of it: `operating` regions with `heatW` (electric power released into a surface), `vent`/`fan` emitters with `volumeFlowM3S` and `temperatureRiseK` for warm air, and the transport layer (parcels/pathlines) for jets. Nothing was extended. Rated power is split so that the sum equals the electrical input:

| Asset | Input | Split |
|---|---|---|
| toaster | 900 W | housing 180, top plate 100, bread crust 150, slot air 470 (0.0044 m3/s, +88 K, vapour 0.03 kg/kg) |
| vacuum-cleaner | 700 W | shell 110, lid 40, exhaust grille 30, exhaust air 520 (0.030 m3/s, +14.4 K, directed backwards) |
| electric-fan-heater | 2000 W (2 x 1000 W stages) | housing 33, grille 60, top cover 93, warm jet 1814 (2 x 0.016 m3/s, +47 K, direction 0 degrees) |

Trap found in `transport/🔥️sources`: the emitter level L scales flow AND temperature rise, so heat goes with L squared. Therefore the fan heater has two stage keys with one emitter each (not one emitter at 0.5).
Sources: IEC 60335-2-9, IEC 60335-2-2/EN 60312-1, IEC 60335-2-30, Regulation (EU) 666/2013, Regulation (EU) 2015/1188, Incropera 2018, ISO 10456; the splits and product dimensions are tagged `authored-estimate` (typical product class, no single product).

## Assets
- toaster: front view 0.30 x 0.20 m, 1.3 kg. Poses idle (empty), toasting (lever down), popped. Clips toast-cycle, cool-down. Moving parts: lever, carriage with slices, dial.
- vacuum-cleaner: side view 1.5 x 0.76 m, 5.1 kg, canister with ten-segment hose, telescopic wand, swivel nozzle. The hose reaches the grip in every pose through an IK solve done in the script (error under 2 mm). Poses parked, running, running-pulled; clips floor-stroke (loop), run-and-park.
- electric-fan-heater: side view 0.196 x 0.31 m, 1.5 kg, louvred outlet grille at the front, dial, handle. Poses off, heating-low, heating; clips warm-up, step-down-and-off.

Scales: 1:50, 1:100, 1:200 authored with individual drawings; 1:500 is explicitly `unavailable` for all three (sub-0.2 mm features; alternative 1:200). Vacuum hose/wand at 1:200 are thickened from 0.035 to 0.048 m (documented in decisions).

## Verification
- `check --category appliances`: all six appliances (A3's three included) print the check mark; the manifest validates (`--asset` runs show 1/1 each).
- Library vitest run (49 files): 876 passed, 11 failed. Failures are missing external tools (qpdf, ezdxf, ffmpeg), a timing test, print-size, a cast-export CLI error test, and `asset-validation > consistent manifest` with `category-missing`: the test's manifest fixture predates the new category `appliances` (A3's plumbing, not these assets).
- Rendered and looked at (figure-only cast, dark-line, dark-solid, light-solid, thermal) at 1:50, 1:100 and 1:200 for all three; transient thermal clip frames of the toaster (t = 4, 30, 100, 160, 250 s: casing and plate warm up, bread browns, popped toast stays warm) and the fan heater jet (parcels view, clip warm-up, 2000 W warm plume leaving the front grille and rising) were checked.

## Notes and doubts
- Steady poses render the thermal view at equilibrium (toaster casing saturates the shared scale); use the clips for gradients.
- The transport view (parcels/pathlines) needs clip time: a single pose at t = 0 shows no jet. A fan-heater transport cast over the full 900 s clip is slow (about 2 min); cast shorter clips for course use.
- The toaster slices sit slightly above the plate when lowered (12 mm), so the default pose shows little bread; popped shows it clearly.
- Generated output was deleted; scripts remain in `T/🧰️a4/`.
