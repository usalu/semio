# R2 sc2: scale and mirror consistency fixes

Source: `r2-cc-report.md` items 1 to 3. V = `⚡️energy/🏗️authoring/🎨️visuals`.

## 1. Laptop scale (people next to laptops)
Laptop is 0.275 m wide, person 1.69 m. Laptop now uses the same metres-to-scene factor as the person beside it.
- Heating internal-gains `_b5_icon`: `shared-laptop-winter` unit 2.4 to 0.8 (person unit 0.8).
- Cooling internal-gains Beat3: removed `unit = unit * 1.5`; laptop (was `unit * 1.5`) and desk use the room metre `room["h"] / 2.5`, like the rack. Laptop heat spot follows.
- Cooling internal-gains Beat4 `mini_laptop`: unit 2.6 to 0.6 (`mini_person` is 0.6).
- Left alone: laptop grid icons (unit 1.5, `f_N` panels, no person in the panel), lamp icon sizes. The laptop is now small beside a person, which is the physical ratio.
- Also fixed a crash found while rendering: cooling Beat3 shadowed the translator `tr` with a tracker (`tr.animate`), now `trk.animate` (another agent had renamed the other uses).

## 2. One metre per series (ventilation volume)
`_use_cast` now takes no height; every mini-room cast uses `_USE_UNIT` (0.44, also used by the crates). Standing adults were 0.77 and 0.62, cook 0.77, office worker 0.62, student 0.42; all are now one scale. Seated worker and student are the same cast, three students fit the classroom box (checked in the de render).

## 3. Mirror at call time
Decision: yes. The exporter's `mirror` is a pure flip about the world origin (`matrix(-1 0 0 1 ...)`, `figureBoundsM` x swapped and negated). Frames are parsed to world metres relative to the origin, so negating x of the parsed points and flipping the bounds gives the same drawing. The transport plume asymmetry lives in the frames themselves, so it flips with the figure.
- `mirror: bool = False` on `staffage`, `staffage_motion` (V/🧍️staffage/🐍️.py: `_flipped`, `_bounds`, `_place`, `_Figure`, `_Motion`, `_Source`) and `fitted_staffage` (V/🐍️.py). The motion content key gets `~mirror`.
- Merged casts (manifest V/🧍️staffage/🔣️.json):
  - `ventilation-need-occupant-mirrored` -> `shared-person-pathlines` + `mirror=True` (need clip `_stand(..., mirror=)`, `_room_occupants`).
  - `ventilation-volume-office-worker` + `ventilation-volume-student` -> `shared-office-worker-pathlines` (transportAgeS 3), student `mirror=True`.
  - `heating-calculation-radiator` + `-mirrored` -> `shared-radiator-thermostat` (calculation clip).
  - `optimization-room-radiator` -> `shared-radiator-winter` + `mirror=True` (optimization room clip).
- Guard `staffage_only.json` `forbidden_casts` gets all six removed ids; `scenes.json` transport casts updated; `scenes.py` mirror test added and the occupants test uses `mirror=`.
- The six ids appear nowhere else in `🎓️teaching` (grep, outside ticket folders and `.venv`).
- Remaining manifest mirror casts (`shared-blind-lowered`, `ventilation-natural-radiator-*`, `ventilation-volume-hood`) are untouched; they have no unmirrored twin.

## Renders (ql, 15 fps, de and en, one at a time), frames viewed
Volume Beat2_Luftwechselrate (classroom/office/kitchen glyphs), heating internal-gains Beat5_SummeUndDichte, cooling internal-gains Beat3_DevicesLighting and Beat4_CumulativeLoad: laptop is now the size of its physical ratio, students and office worker match. Only de frames were extracted; en rendered with exit 0.

## Tests
- `staffage_only`: 25 tests OK.
- `section_consumers` + `integration` + staffage Python suite + `scenes`: 81 tests OK (language en).
- Volume clip tests: 2 OK. The calculation clip has no test module; its import is exercised by `scenes`.

## Open
- The calculation clip radiator pair (shared-radiator-thermostat, mirror) was not rendered.
- Other cc findings (radiator metre helper, thermal spans 70/78, standing literals in cooling Beat2) are out of this task.
