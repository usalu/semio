# Report QA9: Visual QA of O1 Objects

## Round 1–2
- **Checks:** the check, sweep and consistency are ok for all eight assets at every scale. The three-views vitest passes 16 of 16.
- **Grill:** helper bones `lid-axis` and `lid-handle` are deleted. The lid shapes are bound to `lid`. The revolved axis uses the existing `lid-vent` bone, the handle is an `extruded` solid, and the rim is a revolved ring. `T/🧰️O1/lib.py` is now idempotent.
- **Swing `pendulum-forward`:** the chain was drawn but lay exactly on the A-frame leg (30° swing against 29.5° leg lean). It is now a bold chain pen drawn above the legs at every scale.
- **Rigs:** no asset has bones beyond git HEAD.
- **Inspected:** the 1:50 montages of grill, swing and parasol, and one scale of each of the others.

## Open (round 3)
- **Grill open-lid plan:** the revolved disc overstates the depth by about 0.2 m, because the revolved solid did not follow the tilted lid axis. TS is making revolved solids follow the full 3D frame of their axis bone. Then bind the axis to `lid` itself.
- **Parasol:** ribs that are not sagittal are missing in plan. They become physical rib bones with a static `abductionDeg` azimuth, which is exact in 3D, and the elevation foreshortens them. The hem stays a revolved circle; an octagon would need a polygon that follows several bones.
- Sheets at 1:100–1:500 are not all inspected.

## Round 3
- **Parasol:** 8 physical rib bones, horizontal at rest, each with a static `abductionDeg` azimuth per pose (`tan φ = cos β · tan θ`). The furled pose folds exactly. The revolve axis is `runner`, and a tilted canopy shows an elliptical hem. All poses and clips pass.
- **Grill lid:** blocked, because TS's tilted revolved axis applied only to spread or twisted chains. The coordinator ordered one 3D path for every bone (TS).
- **Consistency:** `coverage.py --consistency` is ok for 30 of 30; the three-views vitest passes 16 of 16.
- **Not inspected:** some sheets at 1:100–1:500 (listed in QA9's final message).

## Round 4
- **Writes:** atomic.
- **Same-region details raised:** patio-heater (12 shapes) and bicycle-stand (6). 14 shapes stay deliberately below their body (genuine occlusion).
- **Grill lid:** still waiting on TS.
