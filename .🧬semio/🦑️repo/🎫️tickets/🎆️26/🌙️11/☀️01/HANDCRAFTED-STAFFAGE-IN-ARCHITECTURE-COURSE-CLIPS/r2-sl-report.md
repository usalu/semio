# r2-sl report: system-losses Beat3 floor, label glitches

D = `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip`. Scripts in `T/🧰️sl/`.

## Findings and fixes
1. `D/♨️system-losses/🐍️.py` Beat3 (`Beat3_Erzeugung`): boiler and heat pump floated. Added a `floor` (line plus hatch, same style as the cellar ground in `_plant`) at the appliances' base `Y - 0.6`, faded in with the stage tag. It stays for the whole beat and sits under the boiler, the heat pump and the later centred appliance.
2. `Wärme20kWh` was not a missing space. `_readout` (`D/♨️system-losses/🐍️.py` ~l.269) rebuilt its children with `remove`/`add` while a `play` was running, so the renderer kept drawing the children of the first frame of that play. Stale `Wärme` copies piled up under the live label, which read as glued text (repro and fix proven with `T/🧰️sl/stale.py`). It now calls `become` on the inner label box in place, which keeps `label[0]` (used by the JAZ fly-in copies) valid.
3. `22Monate` (`D/🏷️certificate/🐍️.py:400`): the number-to-word gap used the thin space `\,` (0.17 em, about 3 px at 480p). Now `\;`. It is the only `\,\text{word}` pattern in the energy series. Numbers followed by `\,\mathrm{unit}` keep the thin space by design, and no other concatenation pattern (f-string number followed by a unit) exists in en or de.

## Checked visually (de, ql, 15 fps)
`Beat3_Erzeugung` 8 frames plus crops at 18-23 s: floor under the boiler and under the pump, no ghosts in `Wärme 27 kWh`. `certificate/Beat1` at 37 s shows `22 Monate` spaced.

## Tests
- `visuals.🧪️tests.🐍️` 7 OK, `🐍️scenes` 11 OK (first run had 7 errors from concurrent staffage rendering, passes alone and on rerun), regulations `☀️sun` 2 OK.
- `🐍️staffage_only`: 18 run, 1 failure, `test_rule_a_allowlist_has_a_reason_and_no_stale_entries` for entry `primary-energy/🐍️.py::_plant_panel` (stale allow-list entry, another agent's area).

## Open
- Floor y assumes the sprites fill their 1.2 box; looks seated in the frames.
