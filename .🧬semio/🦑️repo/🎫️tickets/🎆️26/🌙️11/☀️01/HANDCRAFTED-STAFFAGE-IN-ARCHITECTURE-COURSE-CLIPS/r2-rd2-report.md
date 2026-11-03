# r2-rd2 report: demand clips system-losses, regulations, demand-power, certificate, recap

D = `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip`. Rendered all 18 scenes (ql, 15 fps, de, one at a time) and looked at 8 evenly spaced frames per scene plus crops. Scripts in `T/🧰️rd2/` (`render.sh`, `chain*.sh`, `sheet.py`, `crop.py`, `crop.sh`).

## Findings and fixes

| # | Where | Finding | Fix |
|---|---|---|---|
| 1 | `D/♨️system-losses/🐍️.py:145,480-481,507,525` Beat2 | Hot-water cylinder and gas boiler hovered about 0.2 m above the cellar floor | New `APPLIANCE_Y = GROUND_Y + 0.5`; both sit on the floor, flue and ripple anchors follow |
| 2 | `D/🏷️certificate/🐍️.py:843-856` Beat3 | Buyer and tenant were 1.0 unit tall beside houses with a 0.55 scale: about 3 m tall relative to the houses | Houses at scale 0.85, people use the house storey unit (`w_height / 2 / 2.5`, 70 % of a storey), badges and arrow follow `ground` |
| 3 | `D/⚡️demand-power/🐍️.py:749` Beat2 | Pendant lamp hung from the eave height in mid air | Hangs from the underside of the roof ridge |
| 4 | `D/⚖️regulations/🐍️.py:391` Beats 2-4 | Same: lamp in mid air in the upper storey | Hangs from the underside of the first-floor slab (0.26 long, left of the occupant) |
| 5 | `D/🔁️recap/🐍️.py:217-219` Beats 1 and 3 | Desk poked through the right wall, sat on the radiator, and the laptop floated at the desk-with-monitor height (`staffage_height` includes the monitor) | Desk at the left wall, laptop on the 0.72 m tabletop |
| 6 | `V/🧪️tests/🔣️staffage_only.json` | Guard (strengthened concurrently) flags the gas/electricity meter dial `_meter` | Allow-list entry: diagram instrument (meters stay glyphs per contract) |

Checked and fine: true scale of occupants (70 % of storey) in regulations, power, recap, system-losses; radiators on floors in all houses; thermal view and animation (breathing, boiler, heat pump, tank) read physically; handcrafted look consistent. No catalog asset was changed (catalog check not re-run).

## Open
- `system-losses` Beat3: boiler and heat pump icons have no floor line (stage diagram, no room). Left as is.
- Transient readout glitches while a value counts up (`Wärme20kWh` at 19.5 s in Beat3, `22Monate` thin space). Not staffage, not changed.
- Not my area, guard still failing: `cooling/📐️system-sizing::_vent_unit`, `primary-energy::_meter_glyph`, `practice::_thermo`, `practice::_gas_meter` (rd1), `physics/fundamentals::_thermometer`; `scenes.py::test_physics_figures_are_sized_staffage` errors because `physics._radiator` no longer exists.

## Tests
- `🐍️staffage_only`: 18 run, 5 failures, all outside my clips (listed above); my `_meter` failure resolved.
- Combined run before the allow-list edit: `staffage_only` + `scenes` + `regulations/🧪️tests/☀️sun` = 31 tests, 6 failures + 1 error, all in the files named above; the sun test passes.
