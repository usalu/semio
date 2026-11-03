# Staffage re-audit (static): architecture course clips, round 2

Read-only Haiku audit, saved by the coordinator. Paths relative to `A = 🎓️teaching/🏛️architecture/`, `V = ⚡️energy/🏗️authoring/🎨️visuals/`.

**Verdict:** mostly clean, not complete. The earlier findings A1–A14, B1–B4, C1–C6 and D1–D8 are fixed. Removed helpers are gone; the manifest has 126 casts with 0 structural failures and 0 unresolved call-site ids.

## Open findings

### Orphans
- `V/🐍️.py:322-345` `_WATT_DEVICES`/`_WATT_CASTS` keep `bulb`, `monitor` and `vacuum`. No `watt_anchor` uses them. Manifest rows `shared-watt-bulb|monitor|vacuum` are reached only through the dead entries and `V/🧪️tests/🐍️icons.py:63`.

### Still hand-built from primitives
| # | Location | Construct | Route |
|---|---|---|---|
| H1 | `🔥️heating/🎬️clip/🧮️calculation/🐍️.py:1261-1268` | gas boiler rect, thermostat circle, `energy_tank` | gas-boiler, hot-water-cylinder |
| H2 | `📊️demand/…/🏭️primary-energy/🐍️.py:191-199` `_derrick` | oil rig line lattice | new machinery asset |
| H3 | `🧪️practice/🐍️.py:764-767` | circulation pump, circle plus impeller | circulation-pump |
| H4 | `🧪️practice/🐍️.py:646-655` `_valve` | radiator valve | new asset (thermostatic valve) |
| H5 | `🌬️ventilation/🎬️clip/⚙️mechanical/🐍️.py:778-782` | MVHR housing as rounded rect | mvhr-unit |
| H6 | `🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:525-550, 2894-2902` | gas hob, cooking pot | new assets |
| H7 | `🧲️physics/…/🐍️.py:426-443` `_blind` | venetian blind slats | external-venetian-blind |
| H8 | `🧲️physics/…/🐍️.py:1838-1869` | heat-pump cycle exchanger, compressor, expansion valve | new assets or schematic |
| H9 | `❄️cooling/…/🌤️summer-strategy/🐍️.py:304-333` | DEC: sorption wheel, plate exchanger, humidifiers, heater coil | devices or schematic |
| H10 | `V/🐍️.py:1227-1246` `energy_tank` (used in physics, heating calculation, practice, demand-power `_wide_tank`) | storage tank rect plus level | hot-water-cylinder where meant |
| H11 | `🔥️heating/🎬️clip/🧱️conduction/🐍️.py:232-242` `_slab` | oblique wall tile | conduction diagram (keep) |
| H12 | `🌬️ventilation/…/🪟️natural/🐍️.py:248-249` | valve circle on radiator | with H4 |

- `🧲️physics/…/🐍️.py:553-563` `_clock` duplicates `V/🐍️.py:1193-1205` `clock_glyph`.
- The kettle beat (`🧲️physics/…:746-777`) relabels a device frame as "Wasserkocher" and draws no kettle.

### Non-thermal heat figure
- `❄️cooling/…/⚖️heating-comparison/🐍️.py:278` `watt_anchor(60, compare="laptop")` uses dark-line `shared-watt-laptop`.

### Stale text
- `V/🐍️.py:1153-1154` `_filled_outline` docstring still says "human contour".

### Outside energy
- Optimization `🏠️room/🐍️.py:587-588` and `📉️extrema/🐍️.py:439-440` draw the occupant, radiator and cooler as dark-line.
- BIM IFC geometry, Grasshopper, Excel and morph: data or IFC geometry, no action.
- The factory icon in primary-energy and optimization index is a diagram icon, so it is kept.

## Coordinator ruling
- The coordinator dispatched every H item, the orphans, the laptop fix and the optimization thermal fix. Rulings:
  - **H11:** keep, because it is a conduction material diagram.
  - **H8 and H9:** become handcrafted staffage. Each component of the refrigeration and DEC equipment is real hardware.
  - **H10:** a building's hot-water or buffer store uses the hot-water-cylinder asset. Abstract "energy store" bars in charts stay as diagrams.
  - **Optimization room and extrema:** follow the thermal rule, because these clips are about thermal comfort.
