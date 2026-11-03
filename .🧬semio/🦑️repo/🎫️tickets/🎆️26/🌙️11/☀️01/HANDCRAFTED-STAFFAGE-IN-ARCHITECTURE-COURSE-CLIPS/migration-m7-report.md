# Migration M7: non-energy course clips

All casts carry `"theme": "dark"`, view `dark-line` (recolouring only works on `-line` views, `dark-solid` ignores `color`).

## Replaced call sites

| Series / clip | Was | Now (cast) |
|---|---|---|
| optimization / `🎨️visuals/🏠️room` (room section, used by index, sampling, pareto, extrema) | `person_glyph`, `_art.radiator` | `optimization-room-occupant` (orange), `optimization-room-radiator` (panel-radiator rated, mirrored, on the rear wall, red) |
| optimization / `🧪️experiments` (room section) | `person_glyph` | `optimization-room-occupant` |
| optimization / `🧪️experiments` (scatter icons) | `person_glyph`, hand-drawn window polygon | `optimization-experiments-occupant`, `optimization-experiments-window` (tilt-window, tilted-open) |
| optimization / `📉️extrema` (`_icons`) | `_art.radiator` | `optimization-extrema-radiator` |
| bim / `🔗️consistency` (`person()`, 4 uses) | head-and-shoulders circle glyph | `bim-consistency-reviewer` (relaxed), `bim-consistency-decider` (arms-crossed), recoloured to actor colour |
| excel / `📋️program` (`plan(... people=True)`, new `occupant()`) | `person_glyph` | `excel-program-occupant` |

Code: `🎯️optimization/🎬️clip/{🎨️visuals/🏠️room,🧪️experiments,📉️extrema}/🐍️.py`, `🏗️bim/🎬️clip/🔗️consistency/🐍️.py`, `📊️excel/🎬️clip/📋️program/🐍️.py`. Casts are in `V/🧍️staffage/🔣️.json` (8 entries, top of the list).

## Evidence (frames viewed with Read)

- `🗑️generated/m7/images/Sheet_*.png`: all 8 casts rendered (the contact sheet that showed `dark-solid` ignores the colour).
- `🗑️generated/m7opt/i/Winter_*.png`: the real `room_section` winter and summer with orange occupant and red radiator at the rear wall.
- `🗑️generated/m7icons/i/Sheet_*.png`: excel plan occupants at 2 scales, consistency people (3 sizes), extrema icons, experiments icons.
- Scripts: `m7_casts.py`, `m7_sheet.py`, `m7_opt.py`, `m7_icons.py`.
- Tests: optimization test file ran 70 tests; the failures are unrelated to this change (a missing `SEMIO_TEACHING_OUTPUT_ROOT` environment variable, and a JSON schema `$schema` draft assertion). A rerun with the variable set (lifecycle + optimization, then BIM `BimConsistencyContract`) was still running when this report was written.

## Left as is, deliberately

- `⚖️index` `glyph()` uses `object_silhouette` for bulb, sun and factory: criterion pictograms, not staffage. They stay until M8 removes the helper. Decision needed: keep these as diagram icons.
- bim `📊️information` `_car()`: a drawn 1929 period car in plan, stretched to the model's car width and rotated along a swept path. The archive has no `city-car` plan view. Needed asset: a plan symbol of a period car (or `city-car` with a plan representation).
- excel `📋️program` plan occupants: the old glyph was also an elevation figure in a plan (a marker). The archive has no plan representation of any human, so it is still an elevation figure. Needed: plan views of `standing-adult`.
- excel `📋️program` fitting rectangles (bed, desk) and bim `🔌️exchange` imposed-load dots: data-driven schematic rectangles and structural load symbols, not entourage.
- `optimization/_room._cooler` and `lamp_glyph` (ceiling lamps, cooling unit): fixtures with no catalog equivalent; owned by the shared helpers.
- No staffage found in grasshopper, morph, nakagin, typology or parametric. Their capsules, trees (data trees, IFC trees) and the like are diagrams or building geometry.

## Notes

- The panel-radiator cast includes a 0.2 m wall stub, which overlaps the rear wall in the room section; this is invisible there.
- The tilted-window icon is a thin section drawing (small but accurate).

## New assets available

- `vintage-car-1929` (vehicles, A6): replaces the bim `📊️information` `_car()`. Cast with `viewpoint: plan`, pose `parked` or clip `drive-by`; plan x is the length (nose +x), y the width, 4.38 m by 1.70 m, authored at 1:50, 1:100, 1:200 and 1:500. Rotate along the swept path; no stretching needed.
- `split-ac-indoor-unit` (appliances, A6): replaces the optimization room `_cooler`. Side elevation, origin at the bottom of the casing on the wall plane (place it 0.29 m below the mounting height), poses `off` and `cooling`, `transport` view shows the cold jet; 1:50 to 1:200 (1:500 unavailable).

## Follow-up (new assets)

Code changes, all compile (`py_compile`), NOT yet viewed as rendered frames:

- bim `📊️information` `_car()` -> cast `bim-information-car` (`vintage-car-1929`, parked, `viewpoint: plan`), stretched to the model's car length and width, then rotated by the heading; caller logic unchanged.
- optimization room `_cooler()` -> cast `optimization-room-cooler` (`split-ac-indoor-unit`, cooling, mirrored); used by room section, experiments and the extrema icon.
- Ceiling lamps (room section, experiments) -> `pendant()` and `pendant_lit()` in the room module: casts `optimization-room-lamp-off` and `-on`, with the lit cast crossfaded by the old brightness. The extrema lamp icon uses `optimization-index-lamp`.
- optimization `⚖️index` `glyph()`: electricity uses cast `optimization-index-lamp`; sun and factory use `icon_glyph`. No `object_silhouette` call remains in my scope.
- excel `📋️program` occupants -> `viewpoint: plan` cast `excel-program-occupant` at true plan metres (`standing-adult` now has plan geometry in the catalog; `asset-a5-report.md` did not exist yet). The old marker (0.41 m tall) is now full size, so the 0.4 m spacing in `plan()` may overlap and needs a look.

Not verified: the render pipeline was blocked for the last hour by host memory pressure (the capacity gate reported "critical", 9 jobs ahead), so the targeted frames of the new casts were never produced. Frames from the first round (before this follow-up) are in the main section above. To finish: run `m7_icons.py` (it renders car, cooler, lamps at three brightnesses, index pictograms, plan occupants, extrema and experiments icons) and `m7_opt.py`, then check the plan occupant spacing and the lamp bulb offset (0.06 m above the lamp's bottom in `pendant()`).

### Follow-up verification (frame viewed)

`🗑️generated/m7icons/i/Sheet_*.png` rendered once the host freed up and shows: the vintage car in plan at headings 0, 270 and 315 degrees; the cooler (extrema icon and room size); pendant lamps off, half and fully lit (crossfade works); the index pictograms (lamp, coin, sun, factory icon); the plan occupant at both plan scales (true shoulder width, one occupant per type, no overlap). The room section with the new lamps and cooler was not re-rendered (`m7_opt.py`); the first-round room frame predates them.

### Room section re-rendered (`m7_opt.py`, frame viewed)

Winter and summer sections show the pendant lamps with their bulb, halo and light cone aligned, the occupant, the radiator on the rear wall, and the cooler. Fix made after viewing: the cooler floated 0.9 m from the rear wall; it now hangs on the rear wall (room section) and on the cut line (experiments room, `CUT - 0.105`). The experiments change compiles but was not re-rendered.

## Plan views available

- (A5) `standing-adult` (poses relaxed, arms-crossed, pointing, hands-warming), `walking-commuter` (all poses and the walk-cycle clip), `seated-office-worker` (with chair, desk and laptop) and `city-car` (parked and the lane poses) now render with `"viewpoint": "plan"`, `"view": "dark-line"`, `"theme": "dark"`, scale 50, 100, 200 or 500 (x ahead to the right, `mirror` for the other way, the cast `heightM` is the plan depth). Use `standing-adult` for the excel `📋️program` plan occupants and `city-car` for the bim `📊️information` `_car()`. Details in `asset-a5-report.md`.
