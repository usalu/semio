# vehicles/city-bus — rework report

## Pass 1 (second round), 2026-11-22

No earlier report existed for this asset and no batch report `T/rework/*.md` mentions the slug. Source `T/volumes/vehicles/city-bus.json` and catalog `asset.volumes` were byte-equal in content at the start (all four scales, all three lists), so there was no half-written state to reconcile.

### Reference table (12 m low-floor battery-electric solo city bus, class Citaro / Urbino 12 / Lion's City 12 E)

| quantity | real | model (rig, solids, sheet) | verdict |
|---|---|---|---|
| length over body | 12.00 … 12.20 m | 12.00 m (`body` bone 12 m, nose profile to x = 12.000) | ok |
| width over body / over mirrors | 2.55 m / about 2.95 m | 2.55 m (walls at z = ±1.275) / 2.95 m | ok |
| height roof skin / over roof units | 2.95 … 3.05 m / 3.3 … 3.4 m | 2.98 m / 3.30 m | ok |
| wheelbase | 5.90 m | 9.35 − 3.45 = 5.90 m | ok |
| overhang front / rear | 2.70 … 2.80 m / 3.30 … 3.45 m | 2.65 m / 3.45 m | ok (front 5 cm short of the usual band) |
| tyre 275/70 R 22.5 | diameter 0.96 m, section 0.276 m, rear twins at 0.31 m pitch | radius 0.48 m, front single with outer face at 1.19 m, rear twins separate at 1:50…1:200 | ok |
| floor height at doors / kneeling | 0.32 … 0.34 m / 60 … 80 mm | 0.34 m / 70 mm (`axle-f` dy) | ok |
| door clear width / height | 1.20 … 1.25 m / 1.95 … 2.05 m (two leaves) | 2 × 0.625 m / 2.06 m | ok |
| belt line (lower window edge) | 0.95 … 1.10 m | 1.10 m | ok, upper end |
| tumblehome above the belt | 30 … 50 mm | 35 mm | ok |
| windscreen rake | 5 … 10° | 7° | ok |
| skirt clearance | 0.27 … 0.32 m | 0.30 m | ok |
| kerb mass | 12.5 … 13.5 t | 12 580 kg | ok |
| roof battery / heat pump | 6 … 7 m × 2.0 m × 0.3 m / 2.5 m × 2.0 m × 0.25 m | 6.3 m / 2.5 m long, 0.32 m high | ok |

### What I looked at (rendered and read at full size)

`r3-sheet` 1:50 body (six views, three-quarter, front and back close-ups, pose `kneeling-at-stop`), 1:50 thermal (plain and `--cut`), 1:50 structure, 1:100 body, 1:200 body, 1:500 body, 1:500 thermal, 1:500 structure.

### Judgement of the state found

- **Rig**: `body` (fixed) with `battery`, `hvac`, four prismatic door leaves (0.60 m centre, 0.55 m front travel), two prismatic axle bones (±0.12 m air spring travel), two revolute wheel bones. Wheelbase, door places and joint limits match the table. The rig is planar: one wheel bone per axle carries both sides, there is no steering pivot, and kneeling is a pitch of the root (−0.68°) with `axle-f` dy 0.07 m instead of a roll towards the kerb.
- **Body 1:50** (213 solids): a real shell (side panels with arches and door notches, leaning window frame, arched roof, curved nose and tail on the plan outline, header with lip, raked curved windscreen, panes with transoms, wipers, lamps, plates, mirrors on arms, interior, 275/70 R 22.5 wheels with hand holes and nuts, dropped front beam, portal drive axle, air springs, roof units). Proportions agree with the table from all six directions; nothing floats or pokes through in the default and the kneeling pose; the leaves slide outside the wall.
- **Thermal 1:50** (166 solids): nested skin and lining plates, double panes with gap, door region, roof units, drive axle, tyres, brakes; the sheet reads as the same bus and the cut shows the nesting.
- **Structure 1:50** (189 solids): welded tube frame (longerons, sills, ring frames at every post, waist and cant rails, door portals, A-pillars), roof unit rails, axle beams with air springs and king pins, wheels with hubs, masses at the centres of mass.
- **1:100 / 1:200 / 1:500**: each is modelled for its feature size (wall thickness 60 / 80 / 100 mm, glazing per bay → bands, twins merged at 1:500, ring frames 8 / 5 / 3), same bones and ids.
