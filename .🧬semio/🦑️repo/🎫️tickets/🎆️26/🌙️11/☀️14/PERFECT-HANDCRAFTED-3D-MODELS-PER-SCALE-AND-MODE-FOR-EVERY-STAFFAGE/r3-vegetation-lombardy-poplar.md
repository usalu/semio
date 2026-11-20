# Round 3 — `vegetation/lombardy-poplar` (Lombardy poplar, *Populus nigra* 'Italica')


Scripts (ticket folder): `r3-lombardy-poplar-model.ts` (every stem row, buttress, bark ridge, burl, branch row, crown envelope, flame, core, outlet and structural part per scale; rig, pose and area root fixes; writes `volumes/vegetation/lombardy-poplar.json` and the candidate `🗑️generated/r3-lombardy-poplar/🔣️.json`), `r3-lombardy-poplar-kit.ts` (flame tufts, elliptic core lofts, outlines; on top of `r3-scots-pine-kit.ts`, imported unchanged), `r3-lombardy-poplar-draw.ts` (ruling E1 drawings and plan symbol), `r3-lombardy-poplar-roots.py` (writes named members of the candidate into the catalog, everything else must stay equal), `r3-lombardy-poplar-ship.sh` (model, validate candidate, roots, splice, validate catalog), `r3-lombardy-poplar-frames.ts` (rig frames).

Re-run: `./r3-lombardy-poplar-ship.sh`, then `$BUN volumes-check.ts vegetation/lombardy-poplar --product`.

## 1. Reference

| Quantity | Real subject | Source | Asset |
| --- | --- | --- | --- |
| Height, mature | 20 to 30 m (rarely more) | dendrology knowledge (Mitchell, Field Guide to the Trees of Britain and Northern Europe; Bean) | 25.96 m (`dimensions.heightM`), kept |
| Crown width | 3 to 5 m; height to width 5:1 to 7:1 | same | 4.203 m by 4.392 m (6.2:1 and 5.9:1), kept |
| Crown form | fastigiate, narrow spindle: widest in the lower third, drawn to a point; foliage almost to the ground on open-grown trees, here clear stem to about 1.8 m (street tree) | same | modelled so |
| Stem | straight, persistent to the top; diameter at breast height 0.5 to 0.7 m at 25 m; fluted and buttressed foot; old bark dark grey with deep longitudinal furrows, burrs common | same; `wood-handbook-2021` for wood data | 0.63 m over the flare, 0.56 m at 1.3 m, 0.46 m at 6.3 m, 0.34 m at 12.3 m, 0.23 m at 18.3 m, 3 cm leader tip |
| Branching | very many branches round the stem (no whorls), leaving at 35 to 55 degrees and turning up to 70 to 85 degrees, almost parallel to the stem; branch length 2 to 4 m, base diameter 4 to 12 cm; second-order shoots also erect | same | 53 branches at 1:50, leaving at 44 to 54 degrees, bending up to 72 to 76 degrees |
| Leaf | rhombic to deltoid, 4 to 8 cm, on flattened petioles (flutters); below print size at every scale | `jones-2014` for leaf physics | leaves as flame masses, 0.9 to 1.6 m long |
| Sway | crown bends with the stem; branches are stiffly erect and move little against the stem | `vogel-1989`, `en-1991-1-4` | see root fix 3 |
| Mass | green poplar wood about 800 kg/m3; 2.6 t for this stem and branch volume | `wood-handbook-2021` | 2570 kg, kept |

## 2. Audit against the reference (state before round 3)

- Rig: the stem chain (root plate 0.3 m, four stem parts of 6 / 6 / 6 / 6.3 m) is right. The eight branch bones were 3.0 to 3.4 m long at every height, so the upper tiers reached up to 0.6 m out of the narrowing spire (tier 4: 3.0 m long where the crown radius is under 1 m). Foliage bones of tiers 3 and 4 likewise (2.1 to 2.2 m).
- Poses and clips: foliage bones turned up to 10 degrees and branch bones 5 degrees about joints near the stem axis in `storm`; on a column this tears the tiers apart instead of bending the column.
- Dimensions: agree with the reference; not changed.
- 2D: flames drawn as round scallops in horizontal tiers (a cypress-hedge look), no pointed apex (`qa-visual-2.md`: "no pointed apex; twigs absent"); the plan symbol note said 4.16 m by 4.4 m by 26 m against the declared 4.203 / 4.392 / 25.96.
- 3D before: 131 / 49 / 38 / 11 body solids; tiers of ellipsoids, eight branches only, no buttresses, no spire. At 1:500 the whole crown was one solid on `trunk-2`, so in `windy` and `storm` the stem and the rig left the crown (seen in the storm sheet of 2026-11-20 04:40).
- Thermal: `branch-bark` 10 m2 against 15.3 m2 lateral surface of the 53 modelled branches; transpiration anchors hovered inside or beside the old flames.
- Bulk pass of another agent (03:40 to 04:10): it rebound the fourteen 1:500 crown swells from `trunk-2` to the foliage bones and added a fourth decision sentence. The rebinding is right and is now made by the model script itself; its sentence is merged into the 1:500 body decision. No new scale was declared for this asset.

## 3. Root fixes in the asset (old to new)

1. `rig.bones[].lengthM` (and `mass.comX` at mid length): `br-l1` 3.4 to 2.91, `br-r1` 3.4 to 3.0, `br-l2` 3.4 to 2.87, `br-r2` 3.4 to 2.53, `br-l3` 3.2 to 2.14, `br-r3` 3.2 to 1.65, `br-l4` 3.0 to 1.35, `br-r4` 3.0 to 0.85 (each ends where the modelled 1:50 branch ends); `fol-l3` 2.2 to 1.94, `fol-r3` 2.2 to 1.45, `fol-l4` 2.1 to 1.15, `fol-r4` 2.1 to 0.65 (`mass.comX` = length). Joints, angles and hierarchy unchanged.
2. `poses[].loads[].x` of `fol-l3`, `fol-r3`, `fol-l4`, `fol-r4` follow the new bone tips (2.2 to 1.94, 2.2 to 1.45, 2.1 to 1.15, 2.1 to 0.65).
3. `poses` and `clips`: branch and foliage offsets scaled by 0.3 (storm: branches -5 to -1.5 degrees, foliage -10 to -3; windy: -2.5 to -0.75 and -6 to -1.8; calm `fol-top` 0.3 to 0.09), stem offsets kept: the column bends with its stem.
4. `thermal.regions[branch-bark].areaM2` 10 to 15 (factor 1.5, the limit of C16; lateral surface of the 1:50 branches is 15.34 m2). `stem-bark` 30 and `leaf-crown` 240 (exchange) unchanged.
5. `anchors` `tp-crown-1` to `tp-crown-9`: moved onto the upper outer face of the highest outer flame of their foliage bone (D2), bones unchanged.
6. `representations` 50 / 100 / 200 / 500, all three views, and `decisions`: redrawn as true views of each scale's body (E1), same ids, bones, roles and pens; front view: the stems lie behind the foliage, sibling flame groups overlap by 0.6 m (no slivers), hole rings under 0.15 m2 dropped. 1:500: the single `crown` shape on `trunk-2` is replaced by `crown-1` to `crown-4` on `trunk-1` to `trunk-4`; the eight branch hint lines are no longer drawn in the front view (seen end-on they coincided with the stem axis as a dashed line).
7. `symbols[0]`: crown hull, stem and scaffold lines follow the new body; dimensional note 4.20 m by 4.39 m by 25.96 m.

## 4. What is modelled

| Scale | body | thermal | structure |
| --- | --- | --- | --- |
| 1:50 | 726: root flare, 7 buttresses, 4 stem parts (5 to 8 sections, slight wander), 15 bark ridges (texture), 3 burls, 53 ascending branches (8 on rig bones), 9 inner leaf masses, 634 pointed flames (branch tip, side and inner flames, crown skin rows, base ring, spire) | 720: all bark and leaf solids by reference, 9 outlet lenses | 110: wood of flare, stem parts, rig branches and their flame carriers, 45 other branches, leader; 20 joints; fixed root plate; 22 masses |
| 1:100 | 462: flare, 5 buttresses, stem parts, 30 branches, coarser flames, 3-flame spire | 467, 5 outlets | 87 |
| 1:200 | 236: stem with flare, 16 branches, broad flames, 2-flame spire | 239, 3 outlets | 49 |
| 1:500 | 30: stem in 4 lofts, 8 rig branches as slender rods, crown as 4 flame-shaped tiers bound to the 4 stem parts (each ends in a pointed flame inside the tier above), 14 flame swells on the foliage bones | 32, 2 outlets | 45 |

Changed in this session (resume): the 1:500 crown (one solid on `trunk-2`) became four stem-bound tiers with the swells on the foliage bones; fitted to the declared width and depth (4.202 by 4.391 m); the 1:500 drawings follow; the front drawings of 1:50, 1:100 and 1:200 were cleaned as in fix 6.

## 5. Checks

- `volumes-check.ts vegetation/lombardy-poplar --product` (2026-11-20, after the last splice): `PASS (398.1 s)`, 42 rules ok, none failing. Silhouette IoU elevation / front / plan: 1:50 0.999 / 1.000 / 0.992, 1:100 0.999 / 1.000 / 0.990, 1:200 0.997 / 0.997 / 0.988, 1:500 0.996 / 0.995 / 0.992.
- `r3-validate.ts vegetation/lombardy-poplar`: `VALID vegetation/lombardy-poplar (catalog asset + volumes source): schema 0, validateAsset 0`; `--catalog-only` likewise VALID after the splice.
- No rule weakened, no request filed.

## 6. What I saw in the final renders

- 1:50 body, six views and three-quarter alone: a narrow flame-shaped column, widest between 5 and 9 m, pointed apex, closed skin of overlapping pointed flames with vertical run, fluted buttressed foot under a short bare stem. Front, back and both sides agree; the plan is a slightly lobed oval of 4.2 by 4.4 m. Reads as a Lombardy poplar at first sight.
- 1:50 `storm`: the column leans and bends as one; small steps between the tiers on the lee side, no opening.
- 1:100 and 1:200 body: same outline with coarser flames; at 1:200 the apex has a slight waist under the tip flame (two spire flames only).
- 1:500 body, calm and storm: smooth four-tier spindle with swells; in storm the crown now follows the stem and the rig stays inside (before: the stem left the crown by more than a metre at the top). Grey seams at the three tier joints are the price of a crown that bends.
- Structure 1:50 (right side) and 1:100, 1:500 (six views): tapering stem, steeply ascending branch wood round the stem, joints at the stem joints and branch collars, green root plate, masses at the flame centres; no bark, no leaves.
- Thermal 1:50 cut: red stem with flare, green branches, violet flames and inner masses, nothing outside the body.
- 2D 1:500 calm and storm, 1:200, 1:100, 1:50: true views; 1:500 shows three flame arches at the tier joints; front views are clean after fix 6.

## 7. Still imperfect

- The crown skin rows at 1:50 are placed by a stated rule (rows every 0.78 m, half-step turn, a fixed pseudo-random jitter), not flame by flame; branches, stem, buttresses, ridges, burls, spire and the 1:500 crown are stated one by one.
- 1:500: seams at the three tier joints in 3D and three arch lines in the 2D elevations; a real Lombardy crown has no tiers. They exist only so that the crown can bend with the four stem bones.
- 1:200: slight waist below the apex flame.
- The eight rig branches lie in one plane (the rig is planar); the other 45 branches radiate round the stem.
- `leaf-crown` at 1:50 is not opened by a `--cut` thermal sheet into layers: the region has no layers (correct for the data).

State: DONE, with the imperfections of section 7 stated plainly (1:500 tier seams, rule-placed skin rows at 1:50, slight apex waist at 1:200).
