# Verification V1: demand and physics clips

Scope: `📊️demand/🎬️clip/**` (except `🧪️practice`, owned by C1) and `🧲️physics/🎬️clip/**`. All renders: one scene at a time, `--jobs 1` equivalent (plain `manim -ql --fps 15`), host memory never below 39 GB available.

## Renders (l, 15 fps) and what the frames showed

| Scene | Result |
|---|---|
| primary-energy `Beat4_Referenzgebaeude` | Renders fully (60 s); the FadeIn replacement works. Two houses with thermal person, lamp and radiator on the floor line, scale right. Seen: heat ripples of the reference house start about 1 s before the house itself is visible (ghost ripple at 12 s). Cosmetic and pre-existing in the beat structure; left as is. |
| fundamentals `Beat2_EnergieImAlltag` | Clerk, vacuum (thermal, light blue), tank and cable fine. Fixed: the cable ended in mid-air next to the handle; the plug point is now on the canister body (`plugged_cable`). Re-rendered and checked. |
| fundamentals `Beat3_Energieerhaltung` | Thermal pendant lamp hangs from the ceiling with bulb ripples on the bulb; thermal person stands on the floor, 1.72 m scale correct, readable on dark. |
| fundamentals `Beat4_Waermepumpe` | Radiator at the end of the heating-water loop; ok. |
| fundamentals `Beat5_Strahlung` | Radiator appears at the glass in the winter segment, with its plume; ok. |
| fundamentals `Beat6_ThermischeMasse` | Vacuum icon in the comparison row ok, no other staffage. |
| fundamentals `Beat7_SensibelLatent` | Plume and mouth offsets checked on 2x crops (111 to 116 s): the red plume starts above the head, the blue exhale starts at the mouth and flows forward, ripples are centred on the chest. No nudge needed. |
| fundamentals `Beat1_Leistung` | No staffage call site (vacuum is only the "Staubsauger" label); render stopped on purpose. `Beat8` has none either. |
| recap `Beat1`, `Beat2`, `Beat3`, `Beat4` | Thermal person, pendant lamp, desk with monitor and chair read well inside the house on the floor; Beat2 and Beat4 contain no staffage (checked, no regressions). |

Frame extraction: PyAV from the teaching Python cache (`v1_pick.py` sheets, `v1_crop.py` magnified crops), viewed with Read.

## Observations (not changed)

- Panel radiator asset is a side elevation (0.285 m wide, 0.85 m high): it reads as a thin pale-blue sliver with a beige side, little heat colour. Heat ripples and plumes carry the meaning. A front or warmer-surface asset pose would improve this (asset owner decision).
- Thermal people are shown in elevation with the unit chosen from the storey (about 1.69 m body, 1.79 m with plume and sole margin); two clips fit the body to exactly 1.75 m. Both are within 6 percent.
- Figures in house sections (recap, primary energy) are small (about 30 px at 480p) because the houses are small; legible at 1080p.

## Tests

Failure causes were the pre-staffage structure assumptions. Tests were updated to the staffage structure, not loosened in what they protect:

- `🧪️tests/🔣️scenes.json`: removed `_radiator`, `_person`, `_lamp` from `physics_shared` (they are now cast helpers, not the shared glyphs); added `physics_staffage` cases.
- `🧪️tests/🐍️scenes.py`: new `test_physics_figures_are_sized_staffage` (height of radiator, person and lamp, feet on the given foot, lamp hangs from its anchor, bulb inside the lamp, cable starts at the anchor); `test_overview_emitters_scale_with_house` now compares the demand-power radiator with `staffage_height * metre` and floor contact (was the glyph's 0.65 m and 0.12 m clearance).
- `🧪️tests/🐍️integration.py`: `assert_radiator` uses the cast height and floor contact (2 percent); new `assert_person` (body height 95 to 110 percent of 1.75 m scaled to the storey, soles on the floor within 4 percent; the group includes the breathing plume and sole margin); physics constructor bounds locate the `0.855` conceptual heater and the physical radiator by cast; `_radiator` calls must pass an explicit height (3rd positional or keyword), `radiator` calls keep height and width keywords; the cooling seated worker test uses the new `group` / `chest` / `laptop` / `head` keys.
- Cooling person and seated tests were failing for the same reason; fixed because they sit in the same file. Ventilation occupant tests belong to A8.
- The old `🐍️silhouettes` test module no longer exists (removed with the silhouettes); nothing to fix.

Results: see the end of this file.

## Files changed by V1

- `🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` (`plugged_cable`)
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🐍️scenes.py`, `🔣️scenes.json`, `🐍️integration.py`
- Ticket inputs: `v1_render.sh`, `v1_chain.sh`, `v1_chain2.sh`, `v1_chain3.sh`, `v1_pick.py`, `v1_crop.py`, `v1_probe.py`. `🗑️generated/v1` deleted.

## Results

Final run (scenes, integration, section_consumers, sections, transforms, base visuals tests, regulations sun test): 49 tests, OK (530 s). Practice-related subtests also pass now (C1 migrated it). Earlier run: 15 failures and 2 errors before the test updates.
