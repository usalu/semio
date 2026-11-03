# Migration M4 report: cooling clips (excluding internal-gains)

## Scope result

Only two of the five subclips drew staffage; `☀️solar-radiation`, `💧️transmission-humidity`, `📐️system-sizing` and the series file `❄️cooling/🎬️clip/🐍️.py` contain none (checked by grep for people, furniture, devices, trees, radiators, blinds), so they are unchanged. The regeneration heater in `🌤️summer-strategy` Beat2 is a block-diagram component, and the microbe and sun are diagram glyphs, so they stay.

## Changes

- `⚖️heating-comparison/🐍️.py`: `_internal_gains(house, *, overheated=False)` now uses staffage only.
  - Occupant: `staffage_motion`, thermal `idle-breathing` in Beat1 and thermal `fanning` in Beat2 (`overheated=True`).
  - Kitchen: `dining-table` plus thermal `toaster`, replacing the hand-drawn counter and pot.
  - Desk: `height-adjustable-desk` with a thermal `laptop` on top (`DESK_SURFACE_M = 0.72`).
  - Lamps: two thermal `pendant-lamp`s, replacing `lamp_glyph`.
  - Removed `person_glyph`, `lamp_glyph` and `object_silhouette("laptop")`. The unit is derived from the old occupant height, so the layout is unchanged.
- `🌤️summer-strategy/🐍️.py`: `_person` is now a thermal fanning occupant (`staffage_motion`). The hand-drawn louvre lines are replaced by `_blinds`, two thermal `external-venetian-blind` `sun-and-shade` clips sized to each window opening.
- Casts added to `⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json` (all with `"theme": "dark"`):
  - `cooling-heating-comparison-occupant`, `-occupant-fanning`, `-table`, `-toaster`, `-desk`, `-laptop`, `-lamp`
  - `cooling-summer-strategy-occupant`, `cooling-summer-strategy-blind`

## Verification

- Python tests `❄️cooling.🎬️clip.🧪️tests.🐍️` pass (4 tests).
- Low-quality (l, 15 fps) renders looked at via frame crops:
  - heating-comparison Beat1 (breathing occupant, lamps, toaster on table, laptop on desk) and Beat2 (fanning occupant) look right.
  - summer-strategy Beat1 showed the occupant and blinds in place, but that render predates the `theme` contract change.
  - The summer-strategy re-render was killed while queued behind machine memory pressure (1100+ s wait), so it is not re-verified after the theme change.
  - `repo tools exec ffmpeg` failed in this environment (repo CLI bun error), so frames were extracted with PyAV (`av`) from the project venv.

## Findings

- Animated thermal clips are very heavy: every frame is a cairo group of thousands of paths, and an initial render with animated toaster, laptop and occupant did not finish in over 20 minutes. Only the occupant is therefore animated; the toaster, laptop and lamps use `staffage` (first frame). Manim hashing also warns about the large mobject count, so use `--disable_caching`.
- The `height-adjustable-desk` asset includes a monitor (cast height 1.18 m), so the laptop sits beside the monitor, and the desk surface height is a constant rather than `staffage_height`.
- When the cast cache key changes (schema/theme change), all casts re-render (about 9000 frames), which is slow on a loaded machine.
