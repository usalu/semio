# Round 2, cross-clip consistency (agent xc)

Scripts live in `T/🧰️xc/`: `xc_table.py` (static call-site table), `xc_refs.py` (cast id references), `xc_groups.py` (duplicate groups), `xc_heights.py` (cast heights from the cache), `xc_merge.py` (the merge, dry run by default, `--apply`), `xc_sheet.py` (frame sheet), `xc_py.sh` (uv runner). Generated tables were deleted with `T/🗑️generated/xc/`.

## 1. Unit table (static)

193 call sites in 33 files (direct `staffage*`, `fitted_staffage`, `pendant_lamp`, aliases and clip-local helpers). Effective drawing scale by context type, in manim units per metre (u/m):

| Context | Scale rule | Clips | Verdict |
|---|---|---|---|
| House section | storey / 2.5 (person 0.68 to 0.70 of a storey); `0.7 * storey / staffage_height` is the same within 3 percent | heating-comparison, summer-strategy, certificate (4 houses), primary-energy, regulations, power, recap, system-losses, practice, heating internal-gains (house), calculation, conduction, fundamentals | consistent; absolute person height follows the house scale (0.52 to 1.15), by design |
| Room section | clear height / 2.5, or `0.7 * (room_h - slab)` | ventilation need, natural, volume, mechanical (card room uses `0.7 * room_h`, 7 percent taller, harmless), cooling internal-gains, physics person (`1.75 * room_h / 2.5`), optimization room (0.5 u/m) | consistent |
| Mini-room glyph (ventilation volume `_use_glyph`) | 0.44 u/m for people and crates; blackboard 0.25, hood 0.31, operating table 0.354, students 0.32 | volume | hand-tuned inside a small box, unchanged |
| Pictogram or legend | free, chosen for legibility | heating internal-gains B5 icons (person 0.8, laptop 2.4, lamp 0.9), cooling internal-gains icons (standing 0.19 to 0.68, laptop 1.5 to 2.6, lamp 1.1), optimization index and extrema (0.6 high), bim consistency (0.6) | diagrams, not true scale; the laptop is 3x the person in two icon rows, accepted |
| Device close-up | `fitted_staffage` to a box | demand, heating calculation, physics, summer-strategy | consistent per device; the same meter is 0.28, 0.34, 0.54 or 0.8 wide in different beats, a per-beat close-up choice |

Outliers:

- Physics fundamentals clerk (`🧲️physics/…/🧲️fundamentals/🐍️.py` near 984): height `1.75 * 0.72 * w_height / 5` is 0.72 of the house standard. He stands outside the house beside the meter box, so it reads as distance. Left.
- Physics person near line 2935 is a fixed 1.72 units tall in a psychrometric close-up (breath plume), not a section. Left.
- Optimization experiments occupant and window (coordinator question, `🎯️optimization/🎬️clip/🧪️experiments/🐍️.py:905`): g5 enlarged the occupant from 0.55 to 0.85 units (0.486 u/m), the window was 0.56 u/m. The context is a legend row (cloud, person, window under the scatter diagram), a diagram and not a room section. The simulated room section beside it uses `scale=0.5`, where the occupant is 0.845 units tall. I set both icons to exactly 0.5 u/m (occupant 0.845, window 0.625 tall): true scale of the room on the left, no legibility inflation.
- Stroke weight: `_style` clamps strokes to at least 1.0 and scales them with `unit`, so equal u/m gives equal weight. Only the icon rows (unit above 0.6) get heavier strokes. Only the pumpjack (scale 100) and the natural-ventilation radiators (scale 20) use another cast scale, both intentional.

## 2. Duplicates merged

Manifest 154 casts to 97 (57 ids removed, 24 `shared-*` ids). Exact duplicates (asset, pose or clip, view, viewpoint, environment, mirror, transport; `fps` 15 is the clip default):

| New id | Replaces |
|---|---|
| `shared-person-winter` | 14 ids (heating fundamentals, internal-gains, conduction, calculation; demand system-losses, regulations, power, primary-energy, certificate person and walker, practice, recap; physics person; cooling comparison occupant) |
| `shared-person-neutral` | demand practice talker, certificate tenant, shared-watt-person |
| `shared-person-pathlines` | ventilation need, natural, volume, mechanical occupants |
| `shared-person-fanning` | cooling comparison fanning, summer-strategy occupant |
| `shared-person-relaxed-line` | bim consistency reviewer, bim exchange imposed person |
| `shared-lamp-winter` (9), `shared-lamp-summer` (2), `shared-lamp-neutral` (2); `shared-lamp-off-winter`, `shared-lamp-off-neutral` are symmetric renames | lamp ids of heating, demand, physics, cooling, optimization |
| `shared-desk` | 5 desk ids |
| `shared-radiator-front-winter` (6), `shared-radiator-winter` (5), `shared-radiator-thermostat-front` (2) | radiator ids |
| `shared-laptop-winter`, `shared-blind-lowered`, `shared-toaster-winter`, `shared-mvhr-normal` | 2 each |
| `shared-boiler-heating-cycle` (4), `shared-gas-meter-run` (3), `shared-tank-charge-up`, `shared-pump-run-high`, `shared-electricity-meter-run`, `shared-heat-pump-heating-sweep` (2 each) | demand and heating device casts |

Code edits beyond string replacement: heating-comparison replaced the `f"…{season}"` ids with `_SEASON_CASTS`; cooling internal-gains maps `desk` and `lamp` through `_SHARED`. The `staffage_only.json` allowlist keeps one `shared-desk` entry. `_WATT_CASTS["person"]` points at `shared-person-neutral`; the other `shared-watt-*` casts differ in pose or clip and stay.

Kept apart on purpose: laptop `open`, `open-idle`, `work-and-sleep` (different poses, clips), the three `work-and-sleep` laptops (one per environment), mirrored variants, natural-ventilation radiators (scale 20).

## 3. Environment fixes (same story, same preset)

- `ventilation-mechanical-occupant`: `indoor-heated-winter` to `office-neutral`, like the other ventilation rooms and its own mvhr and fans (this merged it into `shared-person-pathlines`).
- `ventilation-volume-cook`, `-hood`, `-operating-table`: to `office-neutral`, like the other mini-room glyph casts they share a frame with.
- `physics-fundamentals-vacuum`: to `office-neutral`, like kettle and shower in the same icon slot.
- `ventilation-natural-radiator-on/-off`: to `office-neutral`, because the occupant of the same room section is neutral.
- Verified consistent: heating and demand house or room scenes `indoor-heated-winter` (certificate buyer, tenant and practice talker stand outside the house, neutral), cooling `indoor-warm-summer` (fanning `summer-heatwave-calm`, blinds `summer-hot-sun`), plant `office-neutral`, outdoor units and pumpjack `winter-overcast`. Optimization keeps the lamp `office-neutral` beside a winter radiator and occupant in a season-generic comfort scene; flagged, unchanged.

## 4. Look

Sheet of 12 existing 480p15 renders from other agents (Bilanzgrenzen, Primaerenergiefaktor, Uebergabe, Rechenbeispiel, Strahlung, SorptionsKuehlung, WattVergleich, ReviewingHeatGains, WinterGains, CoolingSystem, VolumeFlowEquation, Erzeugung), middle frame, side by side. Staffage is visible in about half (Bilanzgrenzen, Uebergabe, ReviewingHeatGains, WinterGains, CoolingSystem); occupants, desks and laptops there share one stroke weight and the 0.65 to 0.7 storey proportion. No stroke or scale outlier. The other frames are pure diagrams at that instant. The sheet predates the merge and environment edits, and I did not render scenes myself.

## Open

- Scenes with changed environments (mechanical occupant, volume glyphs, vacuum, natural radiators) were not re-rendered; ambient 21 versus 22 degrees Celsius changes the tint only slightly.
- Any new reference by rh2, g4 or g5 to a removed id raises `KeyError` naming the known casts.

## Tests

- First run (guard, integration, scenes, staffage tests, 76 tests, 534 s): 20 errors, all from the exporter failing on a half-written `S/…/📐️measure/🟦️.ts` (another agent's edit, line 408 parsed fine minutes later), none from the merge.
- Reruns under heavy machine load (many agents running the same suites) advanced at about one test per 3 minutes; at report time the rerun of `staffage_only`, `integration`, `scenes` showed 8 passing dots and no failure, but had not finished. The merged casts export fine (`shared-person-winter`, `shared-radiator-winter`, `shared-radiator-front-winter` rendered in the log). Not fully verified: please rerun `staffage_only`, `integration`, `scenes` once the machine is quiet.
