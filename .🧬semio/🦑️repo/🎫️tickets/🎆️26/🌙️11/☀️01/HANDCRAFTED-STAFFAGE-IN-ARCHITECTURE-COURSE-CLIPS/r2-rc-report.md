# R2 rc: cooling series staffage render check

Rendered one scene at a time (quality l, 15 fps, de) and viewed frames: heating-comparison Beat1-3, summer-strategy Beat1-3 (incl. DEC Beat2), internal-gains Beat1-5, solar-radiation Beat1 (sun icon badge only). Scripts in `🧰️rc/` (`rc_render.sh`, `rc_frames.py`, `rc_probe.py`, `rc_px.py`, `chain1.sh`).

## Verdicts
- Scale: occupant, table, desk, laptop, toaster, lamps in the house section are about 0.7 storey height, feet on the slab, no clipping. Office room furniture, seated worker and rack have true scale relative to the room. Blinds sit outside the insulation line, scaled to the openings.
- Thermal reading: people, laptop, toaster, rack, lamps warm/animated; split unit goes orange to blue in `start-cooling`; DEC wheel and exchanger warm, humidifiers cold, regeneration heater warms later, sorption wheel spins from its rate set later in the beat.
- Consistent handcrafted look across scenes.

## Fixed
1. `❄️cooling/🎬️clip/⚖️heating-comparison/🐍️.py` `_build_ahu` (~l.196-199): the cooler floated about 0.4 units above the floor because the placement used the union bounds of the clip frames. It now takes the cast origin (`unit.shift([-3.15, floor_y])`), so the body stands on the floor; `coil_c` is a fixed point on the unit (`floor_y + 0.55`), and the coil temperature readout moved up 0.35 so "18 °C" no longer overlaps "Außenluft" (~l.422).
2. `🌤️summer-strategy/🐍️.py` Beat2 (~l.311, 319): labels "Plattentauscher" and "Verdunstungskühlung" touched; moved to x = -0.6 and 2.6.

## Open / observations
- The laptop in the open pose (office-neutral) reads pale rather than hot, since the surface is only about 35 degrees C against a 21 degree room; physically right but low contrast (internal-gains Beat3 desk, Beat4 card). The cast is thermal; no change made.
- The cooler's condensate pipe dips about 0.1 units below the slab line (drain into the ground).
- Stale audit notes: the heating-comparison badge laptop is already the thermal cast.

## Tests
- `staffage_only`: 10 tests OK. Cooling clip tests (`❄️cooling/🎬️clip/🧪️tests`): 4 OK. Internal-gains tests: 3 OK. Catalog was not touched, so no catalog check run.
- Generated output removed.
