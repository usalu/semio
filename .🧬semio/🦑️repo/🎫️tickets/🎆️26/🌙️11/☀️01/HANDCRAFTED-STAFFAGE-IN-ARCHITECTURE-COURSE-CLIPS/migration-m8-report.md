# M8 report: shared helpers and presentation layer

## Done
- `radiator(center, color, height, width)`: now the cast `shared-radiator` (panel-radiator, dark-line, recoloured by `color`), fitted uniformly into the nominal box, feet on the box bottom, centred on x. Only the authored elevation (end section, ~1:3 aspect) exists, so the drawing is narrower than the old 2:1 front panel; `width` is now an upper bound. Tests relaxed accordingly (width `<=`).
- `watt_anchor`: casts `shared-watt-{laptop,monitor,bulb,toaster,vacuum,heater}` (laptop open, desktop-monitor on, pendant-lamp on, toaster idle, vacuum-cleaner running, electric-fan-heater heating). Added `monitor` (30 W). New public helper `fitted_staffage(cast, width=, height=, color=)`.
- New `V/🔣️icons` (outlines + schema) with sun, moon, flame, microbe, factory; `icon_glyph(kind, color, width)`; `sun_glyph`/`moon_glyph` use it.
- Presentation `source_files()` and optimization visuals source list now include the icons and staffage manifest. Presentation render only loads UI control SVGs (kept); no ad-hoc staffage found there.
- `house_section` / `room_section` draw no people or furniture: nothing to migrate.
- Rendered a test scene (watt anchors, radiator, sun, moon) and looked at it: works. Laptop/monitor/bulb side elevations read small but legible.
- Tests: `🐍️`, `🐍️silhouettes` (incl. new icon and staffage tests) pass. `🐍️scenes`/`🐍️integration` could not finish (they import clip modules being migrated concurrently; hang/errors) - re-run after M1-M7.

## Remaining callers (for coordinator deletion)
- `person_glyph` / `seated_person_glyph`: defined in V/🐍️.py; call sites left: `📊️demand/🎬️clip/🧪️practice/🐍️.py` plus tests (`V/🧪️tests/🐍️.py`, `🐍️silhouettes.py`).
- `object_silhouette` (kinds): `❄️cooling/🌤️summer-strategy` (microbe -> `icon_glyph`), `📊️demand/♨️system-losses` (flame -> `icon_glyph`), `🏭️primary-energy` (factory, flame -> `icon_glyph`; tree -> staffage), `🎯️optimization/⚖️index` (bulb -> cast `pendant-lamp`, sun/factory -> `icon_glyph`), V tests `🐍️silhouettes.py` (device/natural-profile tests to delete with the function).
- `lamp_glyph` (ad-hoc pendant, dict of cable/shade/bulb used for glow): callers `📊️demand/🧪️practice`, `🎯️optimization` room/extrema/experiments. Not migrated: callers animate the bulb part; needs a cast plus a separate glow point. Left to the owners/coordinator.
- When deleting: remove `_SILHOUETTES`, `object_silhouette`, `🫟️silhouettes` and its entries in presentation `source_files()` and `🎯️optimization/🎬️clip/🎨️visuals/🐍️.py`; delete `🔣️silhouettes.json` cases for devices.

## Proposal (implemented)
Sun, moon, flame, microbe, factory are diagram icons: moved to `V/🔣️icons`, so `🫟️silhouettes` can go entirely once the callers above are done.
