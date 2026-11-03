# R2 rv: ventilation series staffage render check

E = `🎓️teaching/🏛️architecture/⚡️energy/🌬️ventilation/🎬️clip`. Every scene rendered alone (quality l, 15 fps, de, speaker) with `🧰️rv/rv_chain.sh`, frames cut with PyAV and looked at (sheets via `rv_sheet.py`, zoom crops via `rv_zoom.py`, `rv_mosaic.py`). Generated folder deleted.

## Scenes looked at
need Beat1/2/3, heat-flow Beat1-4, volume Beat1/2/3, natural Beat1-6, mechanical Beat1-4, handoff Beat1/2. No staffage in: heat-flow Beat2-4, volume Beat1, natural Beat3/5, mechanical Beat4, handoff.

## Findings and fixes
| Where | Finding | Fix |
|---|---|---|
| `❓️need/🐍️.py:346` Beat1 | label "Abluft" touched the right wall | label x `hi + 0.62` to `hi + 0.95` |
| `❓️need/🐍️.py:631` Beat3 | MVHR card icon spigots touched the "mechanisch" title | fitted size 1.0 x 1.0 to 0.9 x 0.78 |
| `🌡️heat-flow/🐍️.py:322` Beat1 | fan heater (0.6 high) tiny under the 1 m3 cube | 0.8 high, anchor y -1.34 (top unchanged) |
| `🪟️natural/🐍️.py:361` Beat1 | "innen 26 °C" label overlapped head plume | occupant x `+0.85` to `+0.55`, gentle-flow end points `+0.6` to `+0.3` (`:435-438`) |
| `🪟️natural/🐍️.py:1477` Beat6 | plume reached the right wall | occupant x -2.0 to -2.25 |
| `⚙️mechanical/🐍️.py` Beat3 (`lbl_cyc2`, `person`) | label "70 s Zuluft" ran into the pendulum unit and wall; occupant plume touched the right wall in all three panels | label to `[p3 - 1.52, -1.05]`; occupant x `px + 1.17` to `px + 0.95` |
| `📦️volume/🐍️.py:423-425` Beat2 kitchen | cook, stove and hood poked through the right wall | cook x 0.52 to 0.3, hood 0.8 to 0.58, air paths shifted |

## Checked and fine
- Scale: occupants are 0.7 of the clear room height everywhere, equal across the four occupants in need and volume Beat3; mini rooms (crates, passive MVHR, office worker with desk and chair, three pupils with desks, op table with light stand, cook) in proportion and on the floor.
- Feet on floors, no floating: all rooms. Radiators (natural Beat2) stand on the floor under the window, small and pale when off, salmon when warm.
- Thermal view and air: breathing plumes and pathlines move; MVHR impeller turns in mechanical Beat3 while the central streams run (blade phase differs between frames); duct fans in Beat1/2 sit in the stream path, spin only while the streams run; the reversing pendulum fan changes state with each 70 s phase (`_pendulum_half`), stream direction alternates in the same `Succession` with equal run time.
- Look consistent: all staffage in the thermal view on dark, same line weight.

## Tests
- `🌬️ventilation/🎬️clip/🧪️architecture` and `📦️volume/🧪️tests`: 7 OK.
- Visuals `staffage_only`: 15 of 18 OK in the last run (run 1: 3 failures, run 2: 2). The failures are in the demand series, from concurrent work by others: `demand-primary-energy-gas-meter` not in the manifest, stale allowlist entry `🏭️primary-energy::_plant_panel`, (first run) `🧪️practice::_gas_meter` not allowlisted. None touches ventilation.
- Catalog check not run: no catalog asset was changed.

## Open
- Crates stay dark-line (cargo, no heat meaning).
- One transient `window is not defined` in a cast export (volume student) from concurrent exporter edits; passed on retry.
- Non-staffage overlap seen: volume Beat1 "10m" dimension under the formula box (not touched).

Edited: `E/❓️need/🐍️.py`, `E/🌡️heat-flow/🐍️.py`, `E/🪟️natural/🐍️.py`, `E/⚙️mechanical/🐍️.py`, `E/📦️volume/🐍️.py`. Scripts in `🧰️rv/`.
