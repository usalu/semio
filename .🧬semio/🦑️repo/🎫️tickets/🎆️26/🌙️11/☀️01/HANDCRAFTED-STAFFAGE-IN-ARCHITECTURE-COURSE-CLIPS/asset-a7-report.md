# Asset agent A7: server-rack and clip swaps

## Asset `server-rack` (S/🗂️catalog/🔌️appliances/server-rack, manifest row appended)
- Front elevation of a 42U 19-inch cabinet: 2.0 m x 0.6 m (1.0 m deep, 234 kg), casters, posts, roof, a closed perforated door, a 4U storage array and six 2U servers at the bottom, switch, patch panel and cable brush at the top, open units between (seen through the door perforation). Dimensions from IEC 60297-3-100 (44.45 mm unit, 482.6 mm width), tagged literature; masses and splits `authored-estimate`.
- Poses `idle`, `full-load`, `off` (three named poses are required). Clips `load-ramp` (loop, 60 s, idle -> full -> idle) and `power-up` (the check requires two clips).
- Heat: same mechanism as the other appliances. `operating` regions with `heatW` on key `load` (idle 0.3, full 1): servers 120 W, switch 20 W, cabinet 160 W; the rest leaves in a `fan` emitter `exhaust-air` (0.26 m3/s, +15 K = 4703 W at level 1) on key `airflow` (idle 0.55). Total 5.0 kW at full load, 1.51 kW at idle. The emitter model scales flow and rise together (heat with L squared), hence the separate airflow key (same trap A4 found).
- Front-to-back airflow runs perpendicular to the front view, so the emitter sits on the roof edge and points up (hot-aisle air rising); documented in the assumptions. No plan viewpoint was authored.
- Scales: 1:50, 1:100, 1:200 authored individually, 1:500 `unavailable` (reason, limiting feature 30 mm post, alternatives 200/100). Print suitable under office-laser at all three.
- Trap found: shapes of the same region on the same bone hide each other's outlines (`joined` rule), so detail shapes (bays, vents, ports) carry no `region`; and the cast sets `patterns: false`, so hatches never show in casts. The perforation is therefore a detail path of dashes (door behind the equipment fronts), not a hatch.
- Generator scripts (hand-designed shapes, JSON emitted): `T/🧰️a7/` (`rack.mjs`, `build.mjs`, `fix.mjs`, `casts.mjs`, `render.sh`, `beat.sh`, `frames.py`).

## Verification
- `check --category appliances`: 8/8 pass, the rack included (stroke 0.10-0.13 mm, feature at least 0.34 mm).
- Casts dark-line, dark-solid and thermal (`theme: dark`) at 1:50/100/200, idle and full-load, rasterised with PyMuPDF (no rsvg-convert, no extra Chromium needed) and looked at; the first version (dash grid fighting the unit edges, vent squiggles, hidden details, halo pills in thermal) was redrawn until it read as a rack. Thermal: servers red-orange at full load, switch red, cabinet orange, door and open units room-temperature grey.
- Python: staffage tests and cooling internal-gains tests pass (22 OK). Whole-module run of cooling clip tests, section_consumers and scenes: failures in `scenes.SceneScaleTest` (glyph identity, heating plant, other migrations, same as M1 noted) and 4 cooling-animation errors from my harness missing `SEMIO_TEACHING_OUTPUT_ROOT`; none touches the rack.
- Vitest `repo run @teaching/staffage:test` did not run: it waited 15 minutes on another agent's dependency build and was abandoned; the catalog check (which evaluates, solves and prints every authored scale) is the gate that ran. Please run the vitest suite once the build queue is free.

## Task 2: cooling internal-gains Beat 3
- Cast `cooling-internal-gains-server-rack` (full-load, thermal, office-neutral, dark) in the manifest; `_HEAT_M["server-rack"]` added; the stack of two monitors replaced by one rack at room scale (`unit = room h / 2.5`, 2.0 m tall in a 2.6 unit room). Cable end and heat point follow the rack. Test case list got `server-rack` as a thermal cast. Monitor casts stay for the usage icons.
- Rendered Beat3_DevicesLighting at l / 15 fps (ffmpeg from imageio, frames with pyav): the rack stands on the floor next to the desk, warm.

## Task 3 (coordinator extra): heating internal-gains Beat 3
- Cast `heating-internal-gains-monitor` replaced by `heating-internal-gains-server-rack` (full-load, thermal, office-neutral, dark); the rack stands on the floor at the right, cables and heat point adapted, unused `desk_y` removed. The pose is held, not `load-ramp`: the clip is 60 s and the beat shows the device for about 4 s.
- Rendered Beat3_GeraetePhiE at l / 15 fps and viewed frames at 10 s and 16 s: rack, laptop and worker read well; the "rack equipment" narration now matches.

## Notes
- Generated output under `T/🗑️generated/a7` deleted at close; scripts stay in `T/🧰️a7/`.
