# R2 br: BIM roof reserve blocker (RES-FIN-ROOF-GRAVEL-SL-R-705)

Scripts: `T/🧰️rb2/` (`reserve_probe.py` is the 28 s harness, `subtract_probe.py` the minimal reproduction, `bimtest.py` the suite runner).

## Root cause

Not a missing gravel strip, parapet cut or slope rounding. The geometry is covered exactly (2D and 3D checks: gravel plus GUARD-RM-2 fill the 705 window). The 0.00557 m3 (not 0.003) went missing inside the exact subtraction `_reference_material_parts` (`🧱️model/🐍️.py`, nested `planes()`):

- Cutter half-spaces were built from the first non-degenerate triangle of every face and **trusted the face winding** for the outward direction.
- The gravel cut by the guard leaves an exact sliver cell (volume 1.5e-21 m3, x 8.038..8.078, y 12.6226..12.673, z 6.57..6.58). Its slanted face `[5,2,0,3]` is wound inward (shares edge 2-0 in the same direction as face `[2,0,1]`).
- With one inward plane the sliver's half-space intersection becomes huge, so subtracting a sliver of 1.5e-21 m3 removed 0.00557 m3 from the neighbouring weathering cell (0.00839 -> 0.00281 m3). The finish-reserve check then reports exactly that missing volume.
- It only appeared now because commit 651c804 regenerated the finish windows (705 and its neighbours are new); a single window reproduces it in 28 s instead of 2 h (`reserve_probe.py RES-FIN-ROOF-GRAVEL-SL-R-705`).

## Fix

`🧱️model/🐍️.py` `_reference_material_parts.planes`: orient each face plane outward from the cell's own vertices (flip when more vertex mass is on the positive side). Exact for Fraction cells, no tolerance. After the fix the 705 probe passes (`OK 28.1`), and the captured sub-case keeps the full 0.0083867 m3.

Regression test (schema-free language-neutral data): `🧱️model/🧪️tests/🧭️cutter-winding.json` (exact rational stock and sliver cutter as serialized meshes) + `🧭️cutter-winding.py`; expected volume from SciPy ConvexHull. Verified: fails without the fix (volume 0.002814), passes with it.

## Cheaper path for the clip tests

There is no persistent geometry fixture: only in-process memos (`_geometry_models_cached`, `_SLAB_GEOMETRIES`). Running all ModelCase classes in ONE process builds `building_geometry` once (about 2 h). A disk cache of the exact geometry would need new infrastructure; not added.

## BimClipContract (OI-81)

`test_all_bilingual_clauses_fit_native_caption_layout_for_both_audiences` does not build geometry. Cost: one Manim `Text` per trial line in `wrap_body_lines` (greedy, about 20 Pango/SVG builds per caption, about 450 captions). Fixed in `🏗️authoring/🔤️fonts/🐍️.py`: bisection for the longest fitting word prefix, whole-remainder fast path, cached width probe (`_fits_width`). Output identical to the old greedy wrapper (24 EN/DE clauses compared, 0 mismatches), 1.9x faster standalone. In the full run the test now passes in about 8 minutes under heavy machine load (was above the 560 s runner limit).

BimClipContract result (run 15:01): 6 of 7 tests ok, 1 failure unrelated to my change: `test_scenes_call_only_published_model_functions_and_no_abstract_wall_remains` subTest `🎨️visuals/🏛️savoye/🧱️core/📐️geometry.py` (file unchanged vs HEAD; it uses private `_model._surface_finishes`, `_source`, `_geometry_producers`, `__file__`, which the test requires to be published). Open for the clip owner.

## BIM suites: NOT completed

I started one process `bimtest.py BimClipContract BimInformationContract BimConsistencyContract` at 15:01. BimClipContract finished at 15:09. The geometry build for the other two was still running when the log last changed (15:09, output is buffered). At 16:46 no process of mine exists any more and the log has no `Ran N tests` line, so the run died silently (most likely killed with my shell at the context compaction) without a result. Counts for BimInformationContract and BimConsistencyContract are therefore unknown. Whether any reserve after 705 fails with the same defect is also unverified; a full SL-R slab build (1150 reserves) was started once and killed after 150 reserves (all passing) to free the machine.

Re-run (about 2.2 h, needs a detached process that survives the shell):
`T/m9_tests.sh`-style env + `uv run ... python T/🧰️rb2/bimtest.py BimInformationContract BimConsistencyContract` (also `BimStepExchangeContract` etc. share the same build in the same process).

## Files changed
`🏗️bim/🎬️clip/🧱️model/🐍️.py`, `🏗️authoring/🔤️fonts/🐍️.py`, new `🏗️bim/🎬️clip/🧱️model/🧪️tests/🧭️cutter-winding.json` and `.py`. Generated output removed; no background process left.
