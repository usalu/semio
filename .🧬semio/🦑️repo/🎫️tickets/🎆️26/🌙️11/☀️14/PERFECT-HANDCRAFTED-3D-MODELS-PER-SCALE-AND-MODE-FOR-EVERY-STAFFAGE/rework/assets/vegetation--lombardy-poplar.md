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
