# Silhouette guard: animals group B (roe-deer, pig, sheep, dairy-cow, goat, horse)

Result: all six assets have 0 failing checks of `catalog-silhouette` at 1:500, 1:200, 1:100 and 1:50 (triage `silhouette-triage.ts current`), and `validateAsset` reports 0 issues for them. The guard was not touched.

## Method (scripts in this folder, prefix `animals-appliances-components-b-`)

The recurring cause (see `silhouette-animals-appliances-components.md` of the readiness ticket) is that drawn polygons, ellipses and capsules were authored separately from the volume lofts. The drawing is the wrong side in nearly every case, so the drawn body shapes were re-derived from the volume:
- `trace.ts`: projects the volume solids of a bone (`bone:<id>`, or id substrings, `&` prefix = clipped to the existing shape) into the view, rasterises them, traces the outline (pixel contour, Douglas-Peucker, merged outer loops, closing radius `CLOSE`) and maps it back to the local coordinates of the drawn shape by evaluating the asset in memory with test polygons (affine fit, 4 correction passes, max error printed). This handles bones that are rotated or foreshortened in the default pose (far legs: y scaled 0.63 to 0.99).
- `auto.py` / `rt.sh` / `retrace.py` / `restore.py` / `lib.py`: apply the traced polygon as a span edit (the shape becomes a closed `poly`, key order kept, the rest of the file byte-identical), set values, restore a shape from HEAD.
- `prof.ts`/`prof.sh` (column/row profile drawn against volume), `diag.sh`, `loc.sh`, `bones.ts`, `val.ts` (all issues of `validateAsset` for chosen assets), `sum.py`, `run.sh` (triage of one asset in 4 s).

## Per asset

- pig: front hindbody/chest/neck/ears/legs/hooves retraced (drawn belly 30 mm lower than the loft, ellipses against superellipse sections); plan capsules and ears, head, neck, jaw retraced; elevation hindbody, jaw, tail-root, ham retraced. bbox x 1.221 against 1.23: the 1:50 snout ellipsoid moved 4.5 mm forward (`translation` 0.313 to 0.3175) and the drawn elevation snout shifted with it.
- sheep: 1:50 to 1:200 all views retraced (extras: thigh and humerus bones belong to the trunk silhouette in front and elevation). bbox x 1.2841 against 1.27: the 1:50 tail lofts (`tail-root`, `tail-tip`) were too wide, not too long (world minimum x is set by the section radii `rx`): `rx` scaled by 0.8 and 0.78.
- roe-deer: 1:50 to 1:200 retraced; 1:50 plan eyes (cy 0.04 to 0.018: the eye sat outside the narrow volume head); 1:500 plan hindbody and chest retraced with thigh, shank, humerus and forearm bones (the legs are strokes at 1:500, the reference widens the body). 1:500 volume: hind legs `z` 0.06 to 0.0575 (depth 0.225 against 0.2), far hind hoof shortened (it sank 32 mm below ground); drawn leg strokes aligned to the same centres, stroke ends shifted 23 mm; `ears` retraced.
- dairy-cow: 1:50 to 1:200 retraced including `path` shapes (path becomes poly, `solid` hints dropped where the validator accepts a flat shape); patches (`patch-*`, `rumen-flank`) and `muzzle`/`blaze` are clipped to the volume outline (`&` mode) so they stay inside the body; 1:200 head lengthened by 25 mm (bbox 2.5124 against 2.55) and chest `ry` 0.372 to 0.362 (width); 1:500 gaskin lofts shortened (they sank 42 mm below ground), 1:500 front retraced.
- goat: 1:50 to 1:200 retraced with the pelvis (`rump`) belonging to the hindbody; head, horns, jaw, beard from their own solids; 1:50 head loft last section moved 9 mm (bbox 1.3 against 1.31); 1:500 horn loft shortened to 70 % (it stood 54 mm above the 1.09 m height), 1:500 front and plan retraced, 1:500 `ear-span` stroke widened to the box ears (z 0.14 to 0.17).
- horse: 1:50 to 1:200 retraced (extras: pelvis in the hindbody; pastern and hoof bones into the cannon shape where no own shape exists); hairline crack between pelvis and hindbody lofts (5.6 mm gap at x 0.197 to 0.2025, crack H 0.136 m at 1:200/1:100/1:50): pelvis last section moved to x 0.2075 at 1:200, 1:100, 1:50; 1:50 thigh `ry` capped at 0.1115 (depth 0.63 against 0.62) and muzzle 6 mm longer (bbox x); 1:50 ears: extruded `solid` span narrowed to the 26 mm loft (z0/z1 +-0.0885/0.0615); 1:50 plan `tail-4`: `zTip` taper +-0.032; 1:500 plan retraced.

## Traps

- Shapes with a `solid` hint (extruded or revolved) carry bones that leave the view plane; dropping the hint makes the validator report `lateral-ambiguous` (horse tails and ears). Retracing is therefore only valid for shapes whose bone stays in plane (the tool drops `solid` for the others; horse tails/ears were restored from HEAD afterwards with `restore.py` and fixed through `z0/z1/zTip`).
- 1:500 legs (and other slender members) are unfilled strokes: the reference follows the stroke widths, retracing them as closed polygons erases them (IoU 0.75). The tool skips shapes with `filled` not true.
- The guard bbox allows one feature, but `validateAsset` (`extent-*`) allows only max(2 %, 15 mm) of the declared dimension: tracing a volume that is 4 cm too long or too high at 1:500 copies the error into the drawing. Check `val.ts` after every batch; fix the volume (legs below ground, horn too tall, width) first.
- Cell size: tracing at feature/3 widens the drawing by up to half a cell (34 mm at 1:500), use feature/6 (default now) and a tolerance of 0.6 cell.
- Toes (two lofts per hoof): closing radius 1 to 2 cells keeps the notch but merges the toes into one loop; separate loops are merged by a zero-width bridge.
- 1:500 raster limits (cell 0.019 m): single-cell column mismatches decide IoU 0.91 to 0.92; aligning drawn stroke centres with the loft centres (not offset by 5 mm) removes them.
- `replace_obj` pretty-prints a node with the indentation of its line; use the lib `put` for key/value nodes (an inline array in solids, multi-line in shapes), otherwise the JSON breaks.

## Changed files (all `🗂️catalog/🐕️animals/<asset>/🔣️.json`)

roe-deer, pig, sheep, dairy-cow, goat, horse.
