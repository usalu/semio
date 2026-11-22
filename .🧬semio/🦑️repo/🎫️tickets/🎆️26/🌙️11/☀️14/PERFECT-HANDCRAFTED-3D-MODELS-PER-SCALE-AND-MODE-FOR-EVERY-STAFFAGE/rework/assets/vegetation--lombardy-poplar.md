# vegetation/lombardy-poplar — rework report

Asset: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/lombardy-poplar/🔣️.json`. Hand numbers: `rework-lombardy-poplar-model.py` (ticket folder). Chisel and carrier: `rework-12-kit.py`, `rework-12-assemble.py`, `rework-12-draw2d.ts` of author 12 (imported, not copied; author 12 had not started this asset, `rework/12.md` has no section on it). Looking: `rework-lombardy-poplar-look.sh`.

## Pass 1 (2026-11-22)

### Reference (Populus nigra 'Italica', open-grown, about 45 years)

| Quantity | Real subject | Old model (measured) | Deviation |
|---|---|---|---|
| Height | 25 to 30 m | 25.96 m | fine |
| Crown width | 3.5 to 5 m, a sixth to a seventh of the height, widest at a third of the height, acute top | 4.2 x 4.4 m | fine |
| Stem at 1.3 m | 0.7 to 0.9 m, deeply furrowed grey bark | 0.64 m at the foot of `trunk-1` | thin |
| Stem base | fluted, 1.1 to 1.4 m across, shallow surface roots running 1 to 2 m | 1.04 m flare, short buttress fins | small |
| Clear bole | 1.5 to 2.5 m, crown reaches nearly to the ground | about 2 m | fine |
| Stem | excurrent: one stem to the tip | 4 stem bones to 24.6 m | fine |
| Primary branches | 40 to 80, all round the compass, leaving at 15 to 30 degrees from the stem and turning up until parallel to it; the scaffold branches 5 to 8 m long, 0.12 to 0.18 m at the base | rig: 8 branch bones of 0.85 to 3.0 m, all in the drawing plane, straight at 24 to 29 degrees; about 100 short loose branch solids with no bone of their own | rig is flat (no branch towards or away from the viewer), scaffold branches less than half as long as real, no upturn |
| Second order | upright side shoots 1 to 2.5 m, appressed to the branch | none | missing |
| Foliage | rhombic leaves 4 to 8 cm on the current shoots: slender upright plumes around every shoot, vertical grooves between them, serrated outline | about 600 small tufts on 9 foliage bones that hang beside the branch tips | tufts do not follow a branch; foliage bones are not wood |
| Structure | stem as a tapering cantilever fixed in the root plate, every branch a cantilever inside its parent | members as rods, branch members not on the rig | not the load path of the body |
| Mass | stem volume about 5.5 m3 green wood at 780 kg/m3 plus branches: 4.5 to 6 t | 2570 kg | half |

### Rebuilt

- **Rig** (38 bones, 3D): `root-plate`, `trunk-1` to `trunk-4`, `fol-top` cut out of the one stem axis; sixteen scaffold branches in four tiers (east, north, west, south, each tier turned against the one below), each a `br-<tier><compass>` bone and a `fol-<tier><compass>` bone cut out of the branch axis itself, so every joint lies in the wood it moves. Poses and clips carry the old sagittal lean over to the 3D bones (lee about the lateral axis); wind loads are redistributed by foliage area.
- **Body 1:50**: stem through 14 hand stations (0.78 m at breast height, fluted base 1.2 m, bark furrows), five surface roots, 16 scaffold branches (5 to 7.2 m, leaving at 18 to 27 degrees from the stem, turning up to parallel), 25 shorter branches from 1.7 m to the leader, 6 upright side shoots per scaffold branch and 2 per short branch; a foliage plume lofted along every shoot.
- **1:100, 1:200, 1:500**: the same axes, cut at 32 / 66 / 160 mm wood; plumes merged per branch (two per scaffold branch at 1:100, one at 1:200 and 1:500, four inner crown masses and the top at 1:500).
- **Thermal**: every bark and foliage solid by reference to its body solid; nine transpiration outlet domes on plume tops. **Structure**: a round member at 0.55 of the bark radius along every wood axis, joint spheres at rig joints, root plate disc as the fixed support, mass spheres on the heaviest segments.
- **2D**: every shape redrawn as the projection of the solids of its scale in the product's shape frames.

### Measured after the rebuild

| Quantity | New model |
|---|---|
| Height / width / depth | 25.80 m / 4.52 m / 4.27 m (1:100 4.51 x 4.22, 1:200 4.56 x 4.31, 1:500 within 2 % as validated) |
| Stem | 0.78 m at 1.3 m, base 1.2 m, five surface roots of 1.2 to 1.5 m, 0.24 m at 18 m, tip at 25.3 m |
| Scaffold branches | 16, 5.0 to 7.2 m, 0.14 to 0.18 m at the base, leaving at 18 to 27 degrees from the stem and turning up by 19 to 24 degrees |
| Short branches | 28 from 1.5 m to 22.6 m, 1.4 to 4.2 m |
| Side shoots | 96 on the scaffold branches, 56 on the short branches |
| Solids body / thermal / structure | 1:50 436 / 445 / 264, 1:100 275 / -, 1:200 110 / -, 1:500 60 / - / 43 |
| Mass | 5456 kg (wood volume x 780 kg/m3 plus foliage) |
| Bark areas | stem 45.8 m2, branches 63.9 m2 (written to the thermal regions with the kit's 0.95 trim) |

### Root fixes besides the volumes

- `rig`: 38 bones (was 22); `dimensions`, bone masses, centres and sections from the modelled wood.
- `poses` and `clips`: offsets re-solved per bone as a lee rotation; abduction offsets under 0.1 degree dropped so that every drawn shape stays in its plane (`lateral-ambiguous`); point wind loads per foliage-carrying bone by foliage area, the stem line load keeps the id `wind-trunk-1`, the crown load on a stem bone is `wind-crown-<bone>`; the gust clip scales every point load.
- `anchors`: support footprint from the root collar; nine transpiration anchors on plume tops (emitter diameter 0.5 m).
- `representations` (all four scales, three views) and the true-extent symbol redrawn from the solids; decisions texts en/de rewritten per scale.

### Validation

- Candidate document: `r3-validate.ts --file` -> `VALID file/lombardy-poplar (file as it is): schema 0, validateAsset 0` (01:24).
- Published 01:26 (`rework-lombardy-poplar-publish.py`, then `volumes-insert.py`: "volumes spliced: 1:50 authored, 1:100 authored, 1:200 authored, 1:500 authored").
- `r3-validate.ts vegetation/lombardy-poplar --catalog-only --volumes`: see "Final line" below.

### Rendered and read after the last change

1:50 body (six views), 1:50 thermal cut, 1:100 body, 1:500 body, 1:500 structure, pose `storm`, clip `gust` at 2.5 s, 2D sheet 1:50. Read before the last radius trim of the coarse scales (form unchanged, plumes 8 to 15 % slimmer afterwards): 1:50 structure, 1:200 body, 2D sheet 1:200. Rendered but not read: clip `wind-sway` at 0.8 s.

### What remains imperfect (for pass 2)

- Not rendered at all: pose `windy`, clip `diurnal-transpiration`, a second instant of `gust` and `wind-sway`; thermal at 1:100, 1:200, 1:500; structure at 1:100, 1:200; 2D sheets 1:100 and 1:500; no close-ups (`--only`) of the stem base, a branch collar or a plume.
- 1:50 body: the plumes are egg-shaped lofts with a blunt nose; a real poplar plume is more ragged and more pointed. Between the tiers a few through-views to the stem remain (left side at 11 m, right side at 15 m), and the crown starts abruptly at 1.9 m with one clump; there are no epicormic shoots or burrs at the stem base.
- 1:50 body: branch collars, the junction of side shoots and the bark furrows were not inspected in close-up.
- 1:500 body: the top is a separate bulb above the uppermost plumes (a neck at about 22 m) instead of one continuous spire.
- Thermal: all regions are references to body solids (no own solids besides the emitters); the emitter domes sit as small caps on plume tips and were only judged at sheet size. `areaM2` of `leaf-crown` (exchange basis) was left as authored (240 m2).
- Structure: members are the wood axes at 0.55 radius; mass spheres only on the heaviest three segments; no separate member for the root plate below ground.
- 2D: every shape is a projection outline of the solids (ruler output), not an economically redrawn line drawing: 246 shapes per view at 1:50; the plan in line mode reads as overlapping arcs; no texture strokes for bark or leaves.
- Rig: only the 16 scaffold branches move on their own; the 28 short branches are rigid to their stem bone. Sway is a lee rotation in each bone's own plane, no torsion and no lateral flutter.
- File size: the catalog file grew from 6.4 MB to 14.4 MB (62 000 vertices at 1:50); the plume meshes could be coarser.
- Decisions text of 1:50 says "4.4 m across" (measured 4.52 x 4.27 m).


- 2026-11-22 12:2x, rig verification (not the volume author): `poses` and `clips` re-solved with `rig-reoffset.ts vegetation/lombardy-poplar --write` because the rig now turns a pose offset about the bone's own rest axes instead of the parent's joint axis. 37 branch and foliage bones with a rest abduction had moved by up to 8.3 mm (storm, wind-sway, gust). Every bone keeps the posed frame it had before the rig change (largest difference of a bone tip after the re-solve 0.04 mm). Only `asset.poses` and `asset.clips` were written; rig, shapes, volumes untouched. Do not run the tool on this asset again (`rig-reoffset.done.json`); new poses are authored about the bone's own axes (`rig-lateral-rest.md` 1.1).

- 2026-11-22 12:5x, rig verification: the re-solve above was taken back (`rig-reoffset.ts vegetation/lombardy-poplar --restore --write`, then the redundant zero offsets removed). Reason: the re-solved offsets carried abduction and twist parts of a few thousandths of a degree on all 37 bones, which made every bone count as spread and raised `validateAsset` from 36 to 48 findings (11 `elevation-lateral`, 1 more `lateral-ambiguous`), while the difference they removed is at most 8.3 mm at a foliage tip of a 25 m tree (storm; 0.17 mm at 1:50). `poses` and `clips` hold the author's offsets again (angles to 1e-4 degree, no abduction or twist offset); under the present rig they pose the tree up to 8.3 mm from where it stood before the rig change.

## Pass 1 (second round) (2026-11-22, afternoon)

Start state: `volumes/vegetation/lombardy-poplar.json` and the catalog `asset.volumes` were identical (all four scales); the rebuild of the first pass was complete and published, so there was nothing to reconcile. Judged by rendering (1:50 six views, single right view without rig, 1:200 and 1:500 right view, 1:50 structure, 2D sheet 1:50).

### Reference used for this round (Populus nigra 'Italica')

| Quantity | Real subject | Model at the start of this round | Deviation |
|---|---|---|---|
| Outline | a closed spindle: widest (4 to 5 m) at about a third of the height, tapering evenly to an acute tip, no steps; the crown is dense all round, the stem is not seen from outside above 3 m | mean crown radius 1.9 m from 8 to 12 m, but sectors with a radius of 0.6 to 0.9 m at 14 to 15 m and an empty sector at 18 m (measured per metre of height and 30 degree sector); visible shoulders at 10, 16 and 20 m | notched, stepped |
| Shoot foliage | each upright shoot a lanceolate flame, 4 to 6 times as long as wide, widest in the lower third, acute tip | egg-shaped lofts with an elliptic blunt nose ("pine cone" reading) | blunt |
| Short branches between the scaffold branches | one every 0.4 to 0.8 m of stem, in a 2/5 to 3/8 spiral | 28 over 21 m, none between 12.5 and 14.7 m | too few in the upper half |
| 1:500 crown | one spindle with plume relief | four cylindrical inner masses: a diamond lump with a flat cut at 2 to 4 m, a neck at 19 to 20 m (radius 0.9 m against 1.3 m at 1:50), a ledge at 21.5 m under a separate bulb | wrong outline at bottom and top |
| Stem, base, roots, scaffold branches, rig | as in the table of the first pass | 0.78 m at 1.3 m, base 1.2 m, 16 scaffold branches of 5 to 7.2 m turning up to parallel, 38 bones on the wood axes | fine, kept |

### Rebuilt in this round

- `rework-lombardy-poplar-model.py` made re-runnable on the published asset (it mapped the poses from the bone ids of the old flat rig, which no longer exist; poses and clips are now carried over angle by angle).
- **Foliage form, all scales**: the flame profile `flame` (rounded foot, widest at the stated `peak`, drawn out to an acute tip) replaces the kit's elliptic nose for this asset only (the kit of author 12 is untouched).
- **1:50 / 1:100 / 1:200 body**: ten more short branches (at 5.7, 6.85, 9.8, 13.2, 13.9, 15.15, 16.65, 17.35, 18.05, 18.75 m; 38 in all) with their side shoots and flames, placed where the measured sector profile had holes. Least sector radius now 1.1 to 1.4 m from 4 to 21 m (was 0.0 to 0.9 m at 14, 15 and 18 m).
- **1:500 body**: the four inner crown masses re-stated through hand stations that follow the spindle (0.75 m at 1.8 m, 1.5 m from 9 to 11 m, 1.2 m at 20 m, 0.6 m at 23.2 m inside the top shoot); top shoot from 21.3 m as one spire; scaffold plumes 0.85 m.
- A crown profile measurement (least / mean / largest radius per metre and 30 degree sector) is printed by the model script for every scale.

### Pass 1 (second round), continued by the next author (2026-11-22, 15:36 onwards)

Start state found: the round above had been cut off before publishing. Catalog and `volumes/vegetation/lombardy-poplar.json` were identical (state of 01:26 / 12:37), while the model script (14:58) and the candidate `🗑️generated/rework-lombardy-poplar/lombardy-poplar/🔣️.json` (14:58) already held the flame foliage, the ten further short branches and the 1:500 spindle. Judged by rendering the candidate afresh (1:50 six views and single right view, 1:100, 1:200, 1:500 body) against the catalog state described above: the candidate is better (acute flames instead of eggs, no empty sectors, 1:500 one spindle without neck and ledge). Candidate proven (`r3-validate.ts --file … --volumes`: `VALID … schema 0, validateAsset 0, checkVolumes 0`) and published at 15:46 (`rework-lombardy-poplar-publish.py`, `volumes-insert.py`: all four scales spliced). Source and catalog agree again.

Read in this continuation (candidate of 14:58): body 1:50 (six views, right view without rig), 1:100, 1:200, 1:500; structure 1:50 and 1:500; thermal 1:50 (cut) and 1:500; 2D sheets 1:50, 1:200, 1:500; pose `storm`; clip `gust` at 2.5 s; stem base close-up.

#### Structure reference and defects (engineer's view)

| Quantity | Real subject | Model at the start | Deviation |
|---|---|---|---|
| Load-bearing section of the stem | the wood inside the bark: 0.78 m over bark at 1.3 m, bark of an old Lombardy poplar 25 to 40 mm, so about 0.72 m (0.92 of the bark radius); fluted at the base like the outer form | member solids at 0.55 of the bark radius, round at the fluted base | 45 % too thin: second moment 9 % of the real one |
| Branch section | bark 3 to 8 mm on 60 to 180 mm, about 0.94 of the bark radius | 0.55 | too thin |
| Bone `section` in the rig (area, second moment, extreme fibre) | the wood section | the over-bark circle | 9 % too much area, 40 % too much stiffness |
| Masses | every segment carries its own wood and foliage mass (stem segments 300 to 2300 kg, scaffold branches 40 to 110 kg) | mass spheres on the three heaviest bones only (two, one at coarse scales), hidden inside the member | masses of the crown missing |
| Joints | a branch is fixed inside the stem; the joint lies on the stem axis | spheres of 0.68 branch radius on the stem axis, invisible | not readable |

#### Rebuilt (`rework-lombardy-poplar-model.py`, the kit of author 12 untouched)

- **Members, all four scales**: every wood solid's member restated at the wood section (`WOOD`: 0.92 stem and roots, 0.94 branches), with the oval of the roots and the flutes of the stem base of the body it lies in; slender twigs keep their true section and `slender`.
- **Rig sections**: `areaM2`, `secondMomentM4`, `extremeFibreM` and the title of every bone recomputed for the wood radius.
- **Masses**: one sphere per bone at the bone's centre of mass (wood and foliage), radius half the radius of a wood sphere of that mass; left out where it is below the smallest feature of the scale.
- **Joints**: sphere at every rig joint, 1.06 of the wood radius it lies in, never below ground (the root joint at 0.4 m is limited to its height).
- Structure decisions texts (en, de) say the above; solid counts in the text follow the list.
- Solids in the structure list: 1:50 341 (was 306), 1:500 79 (was 43).

Finding met on the way (a rule that is right): `volumes-structure-coverage` reported `joint-trunk-1` 181.9 mm, then 116.6 mm outside the body silhouette while the joint sphere took the root flare radius and reached below ground; fixed in the model, not in the rule.

#### Validation and publication of the structure stage

- Candidate: `r3-validate.ts --file … --volumes` -> `VALID file/lombardy-poplar (file as it is): schema 0, validateAsset 0, checkVolumes 0` (16:17).
- Published 16:17 (`rework-lombardy-poplar-publish.py --again`, then `volumes-insert.py`: all four scales spliced).
- Final line: `r3-validate.ts vegetation/lombardy-poplar --catalog-only --volumes` -> `VALID vegetation/lombardy-poplar (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0`.

#### Rendered and read after the last change

Read: structure 1:50 and 1:500 (six views, calm). The body, thermal and 2D lists did not change after the reading listed above (the last change touched only the structure lists, the rig sections and the structure decisions).

Rendered after the last change but NOT read (the image reader of the session stopped answering at 16:18; the files were deleted with the other tool output): pose `windy` (body), pose `storm` (structure), clip `wind-sway` at 0.8 s and 2.4 s, clip `gust` at 1.2 s, clip `diurnal-transpiration` at 6 s.

#### What remains imperfect (for the next pass)

- Not read after the last change: structure at 1:100 and 1:200; structure in the poses `windy` and `storm` and in the clips (members, joint spheres and mass spheres ride on their bones, but nobody has looked).
- Never read in this round: thermal at 1:100 and 1:200; 2D sheet 1:100; pose `windy`; clips `wind-sway` and `diurnal-transpiration` (the previous author rendered them at 15:04 and was cut off; whether he read them is not recorded).
- Structure, all scales: the joint spheres (1.06 of the wood radius) do not show in the sheets, no orange is visible at 1:50 or 1:500; the coarse spheres (6 to 10 segments) probably lie inside the member. Either a finer sphere or a short collar ring around the member is needed. The mass spheres of the stem segments show, those of the branches are small dots.
- Structure: no member for the root plate below ground; the five surface roots are members only down to 1:200.
- Body 1:50: the outline has shoulders where the foliage of a scaffold tier ends (elevation, left at 8.5 to 10 m and 12 to 13.5 m, right at 9 m); the lowest scaffold foliage leans outward by about 10 degrees (also at 1:200 and 1:500, where it reads as a slight skirt at 8 m); the crown begins abruptly at 1.9 m; no epicormic shoots or burrs on the clear bole; branch collars and side-shoot junctions were never looked at in close-up (`r3-sheet` has no zoom on a region).
- Body 1:500: the four inner crown masses meet with visible horizontal seams at about 7, 13 and 19.5 m (left view); the silhouette is right.
- Thermal: all bark and foliage solids are references to body solids (`of`), the only own solids are the nine emitter domes; `leaf-crown` `areaM2` 240 (exchange basis) is as authored, not measured.
- 2D: shapes are projection outlines of the solids, not an economically redrawn line drawing; the 1:50 plan in line mode is a ring of overlapping lobes with short branch strokes; the 1:200 elevation carries two tiny stray outlines at the crown base (3.5 m); no bark or leaf texture strokes.
- Rig: only the 16 scaffold branches (two bones each) and the stem chain move; the 42 short branches are rigid to their stem bone; sway is a lee rotation without torsion.
- File size: the catalog file is 21.2 MB (86 900 vertices in the 1:50 body, 58 500 at 1:100); 1:100 has almost as many solids as 1:50 (511 against 520) and could merge the side-shoot flames into their branch flame.
- Decisions text 1:50 says "4.5 m across" (measured 4.53 x 4.23 m).
