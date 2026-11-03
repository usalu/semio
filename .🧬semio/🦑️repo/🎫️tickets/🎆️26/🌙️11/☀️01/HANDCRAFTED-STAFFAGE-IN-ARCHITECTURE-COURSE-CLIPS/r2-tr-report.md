# R2 tr: thermal span review (renders ql, 15 fps, de, three frames each)

Scripts: `🧰️tr/` (`tr_render.sh`, `tr_frames.sh`, `tr_icons.py`). Generated output deleted.

| Scene | Verdict | Change |
|---|---|---|
| heating internal-gains Beat1 WinterInterneGewinne | ok: people, laptop, lamp warm against dark room, figures small but legible | none |
| heating internal-gains Beat2 PersonenPhiP | ok: seated worker orange, plume dots warm | none |
| heating internal-gains Beat3 GeraetePhiE | ok: person orange, rack lighter/orange, laptop thin (asset size) | none |
| cooling internal-gains Beat2 HumanFactor | ok: sleep/work/walk figures readable | none |
| cooling internal-gains Beat3 DevicesLighting | ok: rack and lamps hotter than the room, desk neutral (first two attempts failed in another agent's exporter edit, `defaultDepictionOptions is not defined`; third run fine) | none |
| physics fundamentals Beat1 Leistung (vacuum, kettle, shower icons) | at 25 K the 100 degC kettle could not be told from 60 degC; at 78 K the kettle is clearly red-orange against the peach vacuum and shower | `thermalSpanK` 78 for vacuum, kettle, shower |
| physics fundamentals Beat2 EnergieImAlltag (clerk, vacuum, shower) | comparison scene: shared span; clerk now cooler peach, still readable | clerk `thermalSpanK` 78 |
| ventilation mechanical Beat2 Waermerueckgewinnung, need Beat1 Lueftungsbedarf | ok: occupants warm, breath plumes visible, no heater glow | none |

Not reached: ventilation need Beat2/3, mechanical Beat1/3/4, demand certificate/practice, optimization comfort clips, physics shower/pot beats (hob, pot stay 25).

## Guard change (coordinator ruling: span unit is the Scene)
- `V/🧪️tests/🐍️staffage_only.py`: `scene_spans` now returns one span list per Scene class (the class plus the module-level helper functions it reaches by name; leftover module code is one more group). The test is `test_thermal_span_every_scene_draws_its_thermal_casts_on_one_span`; `resolved_casts` takes an optional syntax-node scope.
- `🧪️tests/🔣️staffage_only.json` `thermal_span.sources`: class-based vectors, plus a two-class case ([25], [40]).
- Staffage vitest `🌡️thermal-span/🟦️.ts`: the manifest test now only requires positive spans (the one-span rule lives in the Python guard).
- Tests: `staffage_only` 20 OK (twice); thermal-span vitest 21 passed.

## Open
- Physics Beat6 uses the vacuum at 78 K (shared cast), no other casts there. 78 K centres on the 21-22 degC air, so the cold half of the ramp (-56 degC) is unused in these scenes.
