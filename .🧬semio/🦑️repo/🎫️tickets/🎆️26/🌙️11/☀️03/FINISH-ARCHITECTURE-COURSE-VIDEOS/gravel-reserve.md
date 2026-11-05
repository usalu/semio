# Roof gravel reserve 705

## Cause

`RES-FIN-ROOF-GRAVEL-SL-R-705` is the triangular roof-finish window at `(8.128, 10.763)`, `(6.79788636, 11.638462621)`, `(8.128, 16.889)`. Its declared owners are `FIN-ROOF-GRAVEL` plus the covering's interfaces `GUARD-RM-2` and `CHIMNEY-SALON-ROOF`.

The shortfall was not an uncovered strip of gravel. In 2D the window meets no other gravel reserve. The guard cuts the gravel and leaves an exact sliver (about `1.5e-21` m³) in the weathering course. One face of that sliver, `[5,2,0,3]`, is wound inward and shares edge `2-0` with the neighbouring face. `_reference_material_parts` used to take each face's first triangle as an outward half-space. The inward face made the sliver's half-space the complement of the sliver, so subtracting it deleted about `0.00557` m³ from the neighbouring weathering cell (`0.00839` m³ down to `0.00281` m³). The finish check reports that hole. The full-building runs quoted it as about `0.00297` m³.

`planes()` in `_reference_material_parts` now flips a face when the farther vertex lies on the positive side of the triangle. For a convex cell that is the outward direction, with no tolerance. After that orientation, gravel plus `GUARD-RM-2` occupy the window exactly.

## What was run

Isolated window: missing volume `0`.

Window plus every roof reserve whose outline meets it, including the beam joint cutters: reservation returned.

Full `SL-R` `_slab_geometry_uncached` (the call `building_geometry` makes), all `1150` finish voids, stopped on the reserve after 705: `RES-FIN-ROOF-GRAVEL-SL-R-705` completed and the next reserve started. Command: `repo tools exec uv -- run --project 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python --python 3.12 --group test python -u full.py` from this ticket, with `PYTHONUTF8=1` and `SEMIO_TEACHING_OUTPUT_ROOT` under `🗑️generated/geom`. Elapsed about 81 minutes. Exit 0. Log line: `stopped after window passed`.

`building_geometry()` itself was not executed to a return value. The same roof-slab call is what used to raise, and that call now passes this reserve. The rest of `building_geometry` (walls, columns, the other products) is the multi-hour build verified on 2026-11-02 after the same orientation change.

## Test

`StructuralAuthority.test_roof_gravel_window_705_is_exactly_occupied` in `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🧪️tests/🤝️authority.py`. It compares the declared owners' exact volume with the finish inside the window. Any shortfall raises in `_finish_reservations` or fails the equality. SciPy `ConvexHull` checks the same shared volume; a guard sliver too thin for Qhull must be under `10^-12` m³.

Command, from the repo root, with `PYTHONUTF8=1` and `SEMIO_TEACHING_OUTPUT_ROOT` set to this ticket's `🗑️generated/geom`:

`repo tools exec uv -- run --project 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python --python 3.12 --group test python -m unittest 🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🤝️authority.StructuralAuthority.test_roof_gravel_window_705_is_exactly_occupied`

Result: `Ran 1 test in 19.149s` / `OK`.
