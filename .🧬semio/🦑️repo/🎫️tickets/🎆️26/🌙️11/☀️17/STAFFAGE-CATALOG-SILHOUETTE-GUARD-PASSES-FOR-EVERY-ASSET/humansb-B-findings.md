# Silhouette guard, humans group B (jogger, person-cooking, person-with-umbrella, parent-carrying-toddler, person-with-white-cane)

Result: all five assets have 0 failing `catalog-silhouette` checks at 1:500, 1:200, 1:100 and 1:50 (final triage). The guard is unchanged. `validate-some` (the validator of `validate-all`) is 0 for jogger, person-cooking and person-with-umbrella; still open: parent-carrying-toddler (4) and person-with-white-cane (2), all `extent-*` findings at 1:500/1:200 (see the end).

## Method (tools in this folder, prefix `humansb-B-`)
- `humansb-B-diag.ts` (clusters plus `PNG=` mask picture, drawn red, volume blue, patch ops applied in memory), `humansb-B-at.ts`, `humansb-B-which.ts` (which solid, slender widened, gives a reference outline), `humansb-B-dets.ts`, `humansb-B-probe.ts`, `humansb-B-flags.ts` (validator `lateral-ambiguous` list).
- `humansb-B-fit.ts`: per view, every deep drawn-only region is cut out of the polygons that cover it, every deep volume-only region is added as a traced polygon on the bone of the neighbouring shape, open strokes outside the volume are pulled onto its boundary; world outlines are carried to the rest frame by damped Newton through the exact evaluator (the carry is not affine for bones with a twin depth profile). `humansb-B-drive.sh` chains elevation, front, plan; `humansb-B-hulladd.ts` adds a polygon or stroke from the hull of volume solids; `humansb-B-reattach.ts` moves validator-flagged flat shapes to the nearest never-spreading ancestor.
- `humansb-B-apply.py` writes ops into the asset file: touched geometry arrays are rebuilt from the original text of unchanged shapes (byte-stable, a no-op apply was verified), volume values are replaced in place (`sil-span.py`).

## Causes found (shared by the humans)
1. Front view: the drawn left forearm/hand revolve sits 0.037 m lateral of the volume (plan already has `axisZ` -0.037), jogger fixed in `solid.axisZ`.
2. Pitched feet in front: the flat foot polygon is foreshortened; use the plan slices (side-profile boxes with lateral span `z0..z1`) in the front too; slice spans come from the shoe loft `rx` (`humansb-B-feetw.py`); the right shoe loft is narrower than the left.
3. Finger capsules (r 5.6 mm, pitch 12.5 mm) leave 1 mm slits that fray the plan outline at 1:50: radius widened to half the pitch (`humansb-B-fingers.py`).
4. Hairline wedge between pelvis cap and torso cap (tilted pelvis loft) opens a 10 cm deep slit in the volume (umbrella 1:200, 1:100): torso first section moved 0.02 m down (`humansb-B-secat.py`).
5. Slender umbrella ribs follow the drawn canopy width (0.236 m wide blocks in front): stroke `rib-line-*-s` in front keeps them slender at 1:200..1:50; at 1:500 ribs are 77 mm thick, shortened by one radius so they stay inside the rim.
6. Revolved forearm with an elliptic loft needs `lateralScale` (umbrella 1:50: 1.5 left, 1.3 right); hook handle of the umbrella had no front drawing (hull shape from `handle-3..10`).
7. Toe lofts sink 13 mm below ground at 1:50 (toddler): sections raised by half the thickness, bbox height back to 1.73 m.
8. White cane: finger strokes lay beside the hand (replaced from the finger capsule hulls), roller 9 mm too long (bbox), cane revolve kept.
9. Flat shapes on spread bones are flagged `lateral-ambiguous`; a flattened solid or patch must go to a non-spread ancestor (`stable()` in the fit) or keep its extruded solid (elevation).

## Validator extents (closed, `humansb-B2-*`)
- parent-carrying-toddler: 1:500 toes and feet were lifted (volume sections +0.041/+0.038/+0.03 m, they sank 41 mm below ground) and the drawn `fit-foot-r-500-1` by 0.041 m (drawn height 1.727 m); 1:500 plan toddler boot capsule `x1` 0.42 to 0.41 (length 0.6125 m); 1:200 toddler boot volume tip 0.205 to 0.2175 and drawn plan/elevation boot tips extended (width 0.585 to 0.598 m).
- person-with-white-cane: already 0 validator findings.
- Both assets: 0 failing guard checks at all scales, `validate-some` 0.
