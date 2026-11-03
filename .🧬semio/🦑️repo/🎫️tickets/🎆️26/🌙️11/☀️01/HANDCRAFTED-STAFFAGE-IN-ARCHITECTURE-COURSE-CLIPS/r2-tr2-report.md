# R2 tr2: thermal span review of the scenes tr did not reach

Scripts: `🧰️tr2/` (`tr2_render.sh`, `sheet.py`, `crop.py`, `spans.py`). Renders: ql, 15 fps, de, one scene at a time (`--jobs` is not a manim flag, so one sequential process). Frames: PyAV contact sheets and crops. Generated output deleted.

| Scene | Verdict | Change |
|---|---|---|
| ventilation need Beat2 Lueftungsweg | no staffage (diagram air parcels), readable | none |
| ventilation need Beat3 Fahrplan | only the MVHR icon (orange, warm), readable | none |
| ventilation mechanical Beat1 Grundtypen | duct fans pale/neutral, plumes readable | none |
| ventilation mechanical Beat3 Anlagenvergleich | standing occupants warm peach, not glowing; MVHR/pendulum fan readable | none |
| ventilation mechanical Beat4 Kanalquerschnitt | no staffage | none |
| demand certificate Beat1 BedarfVsVerbrauch | person warm, radiator red-orange against dark room: hot looks hot; small but readable | none |
| demand certificate Beat3 PflichtangabenUndFolgen | buyer/tenant warm peach beside the houses, labels readable | none |
| demand practice Beat1 BerechnungVsAbrechnung | person peach, radiator red, readable | none |
| demand practice Beat2 HydraulischerAbgleich | radiators, valves, pump, boiler/meter icons readable | none |
| demand practice Beat3 SerienAbschluss | icons, radiator, two persons warm, readable | none |
| optimization extrema Beat1 (room visuals: occupant, lamps, cooler, radiator) | occupant and lamps warm, room readable on dark | none |

Not rendered: optimization comfort clips other than extrema Beat1 (room visuals are shared by every ClipScene through `RoomVisuals`; the same casts, 25 K, were visible in extrema).

## Guard failure from the coordinator
`staffage_only::test_thermal_span_every_scene_draws_its_thermal_casts_on_one_span` failed with `spans=[25, 70]` for `❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py`: Beat2_SorptionsKuehlung draws the solar-thermal collector (70 K, set by another agent) next to wheel, exchanger, humidifier and heater (25 K).
Ruling applied (one span covering the hottest cast shown together): `V/🧍️staffage/🔣️.json` `cooling-summer-strategy-{wheel,exchanger,humidifier,heater}` 25 -> 70. Those four ids are used only in Beat2 (grep over the architecture tree), so no split was needed; the blind (Beat1) stays 25. Beat2 itself was not re-rendered after the change.

## Tests
- `staffage_only`: Ran 22, OK (after the fix).
- Staffage library vitest (`🧰️cr/round.sh` (c) command): 56 files passed, 1131 tests passed, 11 skipped, rc 0.
- The thermal-span vitest expectation (`🌡️thermal-span/🟦️.ts`, "thermalSpanK in the cast manifest") still asserts that a positive number validates and zero/negative/non-number values are rejected by the schema and `castManifestIssues`. That is the per-cast contract; the one-span-per-Scene rule lives in the Python guard (class-based vectors including a two-class [25],[40] case). Not loosened: validity checks unchanged, only the cross-cast uniformity moved to the per-Scene Python guard.

## Open
- Summer-strategy Beat2 at 70 K should be glanced at once by whoever owns that clip (collector hot, heater now 70 K shows less saturation than before).
