# M1 report: heating internal-gains and fundamentals

Scope: `🔥️heating/🎬️clip/👥️internal-gains` and `.../🧲️fundamentals` (composition and speech files had no glyph uses).

## Call sites replaced
Internal gains (`👥️internal-gains/🐍️.py`)
- Beat1 house: `person_glyph` -> thermal `standing-adult` (idle-breathing clip); hand-drawn desk -> `height-adjustable-desk` (dark-line, white); `_laptop` (object_silhouette) -> thermal `laptop`; `lamp_glyph` -> thermal `pendant-lamp`.
- Beat2: `seated_person_glyph` -> thermal `seated-office-worker` (typing clip). The asset carries its own desk, chair and laptop.
- Beat3: seated glyph, `_desk`, `_laptop`, `_rack`, drawn socket -> the worker (own desk and laptop; laptop heat point at the laptop) plus thermal `desktop-monitor` on the desk end. Socket dropped; cables start at a flash point. The colour-set "heat up" of laptop/rack became a Flash/Indicate.
- Beat4: `lamp_glyph` -> `pendant-lamp` (unit 3.2); the bulb fill animation is gone (the thermal lamp already glows).
- Beat5: three bar icons -> thermal standing-adult (motion), laptop, pendant-lamp.
- Helpers `_spot`, `_worker`, `_on_surface` added; `_laptop/_desk/_rack` removed.

Fundamentals (`🧲️fundamentals/🐍️.py`)
- `radiator(...)` -> thermal `panel-radiator` (pose rated); `person_glyph` -> thermal `standing-adult` idle-breathing (`staffage_motion`). Both are placed with `unit = room metre * stage scale` and no longer scaled by the stage group (so the motion frames stay valid). `RADIATOR` bounds, the glow rectangle and `_warmth` now derive from constants matching the asset (0.32 m wide, 0.81 m high). `OCCUPANT_SCALE` removed.
- Test `visuals/🧪️tests/🐍️scenes.py::test_heating_closeup_retains_real_dimensions` now reads `stage["occupant"/"radiator"]` and compares with `staffage_height(...)`.

## Casts added (V/🧍️staffage/🔣️.json, all `theme: dark`)
`heating-fundamentals-panel-radiator`, `heating-fundamentals-occupant`, `heating-internal-gains-{standing-adult, office-worker, desk, laptop, monitor, pendant-lamp}`.

## Verification
- `unittest` scenes.test_heating_closeup_retains_real_dimensions + section_consumers: 10 tests OK.
- Low-quality (l, 15 fps) renders of chapters Heating_04_InternalGains and Heating_01_Introduction; frames extracted with `repo tools exec ffmpeg` (ffmpeg via SEMIO_TOOL_FFMPEG pointing to the imageio binary) and viewed: people, laptop, monitor, lamp and radiator read well in thermal colours on the dark background; breathing and typing play.
- Fixes after viewing: Beat3 doubled desk removed; lamp and icon sizes increased; radiator bounds corrected.

## Open issues
- Chapter assembly reports "Authored sources changed while rendering" because other agents edit concurrently; the beat mp4s were produced and inspected.
- Whole-file scenes test run has 12 failures from other migrations (physics glyph identity checks), not from these clips.
- Warm-up transients (laptop/monitor `power-on-warm-up`, lamp `switch-on-warm-up`) are not used; devices appear already warm.
- The beat 3 narration still says "rack equipment" while the visual is a desktop monitor.
- Deleted from the ticket folder at close: `🗑️generated/`, temporary patch scripts (m1_patch*.py, m1_b3.txt).
