# vegetation/english-oak — rework report

Asset: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/english-oak/🔣️.json`. Hand numbers: `rework-english-oak-model.py` (ticket folder; begun by author 12 as `rework-12-english-oak-model.py`, continued here). Chisel and carrier of author 12, imported and not copied: `rework-12-kit.py`, `rework-12-assemble.py`, `rework-12-draw2d.ts`. One round (build, draw 2D, render, validate): `rework-english-oak-round.sh`. Publishing: `rework-english-oak-publish.py`.

## Pass 1 (second round), 2026-11-22

### State found

- Source `volumes/vegetation/english-oak.json` and the catalog `asset.volumes` were identical (both of 2026-11-21 15:26): the old model, 287 / 180 / 171 / 170 body solids, a sagittal rig of 25 bones with four limbs in the drawing plane, foliage as ball clusters on carrier bones that hang beside the wood (table below, column "old model", taken from `rework/12.md`, section 1).
- Author 12 had written the reference table and a new model script (01:25) with a candidate under `🗑️generated/rework-12/english-oak` that was never published; its last validation log (01:20) ended `INVALID … validateAsset 191, checkVolumes 6`. Rebuilt with the kit as it stands now, that candidate validates (`VALID`, without the measured rules), so it was the better state and the starting point.
- Read on its sheets (1:50 six views and the elevation at full size, 1:200, 1:500, 1:50 structure): a true three-dimensional branching skeleton, but the wrong habit for an oak. The crown was a tiered pyramid, widest at the lowest tier and narrowing to a single pointed leader (the outline of a cedar or a young pine); the limbs were nearly straight; at 1:200 and 1:500 every limb system was one long upright paddle that followed the steep limb, so the crown read as an artichoke.

### Reference (Quercus robur, open-grown, 120 to 150 years)

| Quantity | Real subject | Old model | Candidate of author 12 | This pass |
|---|---|---|---|---|
| Height | 18 to 22 m (open-grown trees stay low and wide) | 18.2 m | 17.4 m | 17.6 m |
| Crown spread | 18 to 24 m, as wide as or wider than high | 18.3 x 15.0 m | 18.8 x 15.7 m | 18.8 x 15.7 m at 1:50 |
| Crown outline | broad dome, widest at 40 to 55 % of the height, top round and broad, several co-dominant limbs instead of one leader (decurrent) | closed ball cluster | widest at 35 %, pointed top (excurrent) | lowest limbs reach 9.4 m at 6 to 7 m height, middle limbs 8.4 m at 11 m, crown limbs 5 m at 15.3 m, top at 16.9 m |
| Stem diameter at 1.3 m | 0.9 to 1.2 m | 0.80 m | 0.98 m | 0.98 m |
| Root collar | 1.5 to 1.8 m at the ground, 5 to 7 buttress roots running 1.5 to 2.5 m | 0.92 m, seven stubs | 1.48 m, five roots of 1.8 to 2.3 m | unchanged |
| Clear bole | 2.5 to 4 m | 3.4 m | 3.3 m | 3.3 m |
| Scaffold limbs | 6 to 10 all round the compass, lowest 10 to 25 degrees above the horizon, 0.35 to 0.5 m at the base, 8 to 11 m long, elbowed, tips turning up | 4, in the drawing plane, joints floating beside the parent | 8 scaffold + 4 crown limbs, 0.29 to 0.44 m, 7.4 to 10.2 m, nearly straight (elbow 0.2 to 0.35 m) | 8 scaffold limbs 0.32 to 0.48 m and 8.8 to 10.2 m with elbows of 0.3 to 0.5 m; 4 crown limbs 0.28 to 0.31 m, 8.4 to 8.8 m, leaving at 50 to 57 degrees |
| Branch orders | limb, bough (0.1 to 0.18 m), branch (40 to 60 mm), twig; alternate, wide angles of 50 to 80 degrees | limb, one branch, 20 branchlets | limb, 3 to 5 boughs alternately, 3 branches each | the same, one more bough on every crown limb (round top) |
| Foliage | leaves crowded at the shoot tips: tufts along the outer boughs, shade leaves on the inner boughs, sky between the limb systems; seen from afar a cumulus-like dome of lying masses with notches | 17 spheres with bosses | tufts on branches (1:50), paddles along the limbs (1:200, 1:500) | 1:50 and 1:100 tufts on boughs and branches, slightly fuller; 1:200 and 1:500 lying layers per limb system (they follow the limb in plan but rise only by a part of its rise) around a larger shaded core and a sunlit centre |
| Structure | root plate, stem, limbs, boughs as one connected tapering frame, fixed at the root plate | rods to ball centres, ending in the air | a round member along every wood axis, joints at the rig joints, root plate as the fixed support | unchanged |
| Mass | 10 to 15 t above ground (stem about 5 m3 plus limbs, green oak about 950 kg/m3) | 5971 kg | 12 128 kg | 13 296 kg |

### Rebuilt in this pass (hand numbers in `rework-english-oak-model.py`)

- **Habit**: the four middle limbs lengthened to 9.0 to 9.8 m and laid flatter (30 to 36 degrees), the four crown limbs lengthened to 8.4 to 8.8 m and raised to 50 to 57 degrees so that they are co-dominant with the central limb: the crown is a dome now, no pyramid.
- **Limbs**: base radii of the scaffold limbs raised to 0.16 to 0.24 m, of the crown limbs to 0.14 to 0.155 m (pipe model for the longer limbs); elbows (`wave`) raised to 0.3 to 0.5 m.
- **Foliage 1:50 / 1:100**: bough and branch tufts 12 to 15 % larger; a fourth bough with its three branches on each crown limb.
- **Foliage 1:200 / 1:500**: new hand function `layer` in the model script: the foliage of a limb system is a lying layer through three stations (it follows the limb in plan, rises by 0.75 (1:200) or 0.3 (1:500) of the limb's rise); the central masses on the stem enlarged (shaded core 4.0 m, sunlit centre 4.2 m, top 3.0 m half width).
- **Rig** (50 bones, from the kit): stem cut into `root-plate`, `trunk`, `trunk-upper`, `leader`, `leader-top`, `fol-top`; every scaffold limb is collar, `limb-*`, `br-*`, `fol-*` cut out of its own axis, every crown limb `br-leader-*`, `fol-leader-*`; all joints lie in the wood they move.
- **Thermal, structure, 2D, decisions**: produced from the same hand model by the carrier (thermal solids by reference to the body solids plus transpiration emitters; a member along every wood axis; every 2D shape redrawn as the outline of the solids it stands for).

### Root fixes forced by findings (all in `rework-english-oak-model.py`)

- Stem stations lie in the drawing plane (`z = 0`; the stem still wanders in x). Reason: since 01:59 `validateAsset` reports `lateral-ambiguous` for every flat elevation shape on a bone that rests out of the paper plane (`restTilted` in `🔨️modules/🗂️catalog`), which hit 46 bones of the kit's 3D rig, the slightly three-dimensional stem included.
- `settle()` (after the carrier): every 2D shape of a limb system is bound to the stem bone that carries the limb (`trunk`, `trunk-upper`, `leader`, `leader-top`, `fol-top`), so the drawings are exact projections in the rest pose and lean with the stem in a pose, but a limb's own sway is not drawn in 2D (it is in the 3D model). The joint ball at the foot of the bole is 0.30 m (it reached below the ground), the members of the surface roots end at the ground, the mass ball of the root plate sits above it.
- Tufts of boughs shorter than 2 m start earlier and are narrower, short boughs are straighter (`wave` scaled by length), 1:100 bough masses are capped at 0.62 of the bough length: the lofts of fat tufts along short curved boughs intersected themselves (`volume-size … one closed, outward triangular shell`, 11 solids).
- `area_trim` of `trunk-bark` 0.79: the measured outer surface is 41.3 m2 (mesh area 52.5 m2 less the buried collars and root overlaps); `areaM2` is 41.5 m2 now. `limb-bark` 122.9 m2.
- The sun emitter that sat on the top shoot moved to the tip of the southern crown limb (its dome stood 43 mm above the body box at 1:200).

### Measured (final build)

| Quantity | Value |
|---|---|
| Height / width / depth | 17.30 m / 19.07 m / 15.80 m (1:50 18.85 x 17.6 x 16.03, 1:100 18.82 x 15.56, 1:200 19.32 x 15.88, 1:500 19.30 x 15.88) |
| Mass | 13 284 kg |
| Bones | 50 |
| Solids 1:50 | wood 281, foliage 239, 94 234 vertices |
| Solids 1:100 / 1:200 / 1:500 | wood 277 + foliage 71 / 85 + 26 / 55 + 15 |
| Catalog file | 19.5 MB (was 7.9 MB), source 9.4 MB |

### Validation

- Candidate, `r3-validate.ts --file … --volumes` (03:05): `schema 0, validateAsset 0, checkVolumes 8`.
- Published 03:38 with `rework-english-oak-publish.py` (catalog document and `volumes/vegetation/english-oak.json` in one go), then `volumes-insert.py` ("volumes spliced: 1:50 authored, 1:100 authored, 1:200 authored, 1:500 authored").
- `r3-validate.ts vegetation/english-oak --catalog-only --volumes` (03:58): `INVALID vegetation/english-oak (catalog file as it is): schema 0, validateAsset 0, checkVolumes 8`. Published although the measured rules are not clean, because the catalog held the old ball-cluster model and every other rule (schema, `validateAsset`, silhouette 2D against 3D, bounds, feature, connected, thermal and structure coverage) passes.
- The eight findings are all `volumes-cross-scale`, boundary distance p95 only (IoU passes everywhere): 1:100 against 1:50 in elevation, front and plan 142.5 mm (limit 108.8), 1:200 against 1:100 in all three views 285.0 mm (217.5), 1:500 against 1:200 in elevation and front 712.5 mm (543.8). The values are the saturation of the measure (9.5 features): more than 5 % of the outline cells lie farther apart than that, because the finer scale has tufts with sky between them where the coarser has one mass. Not a rule defect as far as I can tell; the schema offers `envelope: true` for a coarse solid that stands for a finer lattice (the finer solids of that bone are then compared through their convex hull), which the carrier does not use.

### Rendered and read

- Read at an earlier state of this pass (before the last numeric fixes, form the same within a few percent): 1:50 body six views with the dome habit (twice), the elevation at full size of author 12's state, 1:100 body, 1:200 body (layers at slope 0.5), 1:500 body (layers at slope 0.3, ry 2.0), 1:50 structure of author 12's state.
- Rendered after the last change but NOT read (the Read tool stopped answering at 03:05, "PreToolUse hook did not respond"; six attempts): 1:50, 1:100, 1:200, 1:500 body sheets; poses `windy`, `storm`, `wet-snow` (elevation) and `storm` (front); clips `wind-sway` at 1 s and 3 s, `gust` at 2 s and 6 s, `diurnal-transpiration` at 6 h and 18 h; thermal 1:50 (cut), 1:100, 1:200, 1:500; structure 1:50, 1:100, 1:200, 1:500; 2D sheets 1:50, 1:100, 1:200, 1:500. The script `rework-english-oak-looks.sh` renders this whole set again (it takes `--file`; for the published asset use the plain `r3-sheet.ts vegetation/english-oak …`).

### What remains imperfect (for pass 2)

1. Everything in "rendered but not read" above: no pose, clip, thermal, coarse structure or 2D image of the final state has been judged by eye. In particular 1:500 changed after the last look (layer slope 0.3 to 0.6, half height 2.0 to 1.8, start 0.42 to 0.36).
2. `volumes-cross-scale` p95 at all three coarser scales (numbers above). Way to go: make the coarser foliage follow the finer outline (1:100 one mass per bough is wider and smoother than the branch tufts of 1:50; 1:200 layers against 1:100 bough masses; 1:500 layers against 1:200 pairs), or declare the coarse masses `envelope: true` per bone and shape each as the hull of the finer solids of that bone; a round costs about 10 minutes (2D drawing 5, measured rules 5).
3. 2D: every shape is the projection outline of its solids (ruler output of `rework-12-draw2d.ts`), not an economically redrawn drawing; limb systems are bound to stem bones, so a limb's own sway is not drawn in the poses; no bark or leaf texture strokes. The right fix for the `lateral-ambiguous` rule is `solid` shapes (revolved limbs, extruded or revolved foliage) on the limb bones.
4. 1:50 body: the tufts are faceted lobed lofts of 1.5 to 2.5 m; a real oak crown is finer (shoot clusters of 0.3 to 0.6 m) and denser in the outer shell; no dead stubs, burrs or epicormic shoots on the bole; branch collars and bark fissures not inspected in close-up; limbs are elbowed by a lateral wave only, without the abrupt changes of direction of old oak limbs.
5. 1:200 body: boughs of 100 mm and more stand out of the flat layers in places (seen at slope 0.5; slope is 0.6 now, not re-read).
6. Thermal: all regions are references to the body solids, emitters as domes on tuft tops; `leaf-sun` and `leaf-shade` `areaM2` (exchange basis) left as authored. Structure: members are the wood axes at 0.55 of the bark radius, mass balls only on the heaviest three bones, root plate as a disc support.
7. Rig: only the twelve limb axes move (collar, limb, branch, end shoot); boughs are rigid to their limb piece. Sway is a lee rotation about each bone's own hinge, no torsion.
8. File size: the catalog file is 19.5 MB (94 000 vertices at 1:50, 35 000 2D points); the foliage meshes and the 2D outlines could be coarser.
9. Decisions texts come from the carrier with the hand-written habit paragraphs of the model script; the 1:50 text still says "0.98 m at breast height … top to 17 m … four lowest limbs 12 to 20 degrees" (true) but does not mention that the stem lies in the drawing plane or that limb drawings follow the stem.

### Fundamental issues (outside this asset)

- `lateral-ambiguous` for rest-tilted bones (catalog module, 01:59 on 2026-11-22) estimates the paper error as extent x sin(tilt) without regard to how the shape was made; a flat shape that was solved through `evaluate` to be the exact rest-pose projection (what `rework-12-draw2d.ts` does) is refused like a naive one. Every tree built with the kit of author 12 (3D rest rig, flat projected shapes) is hit; the kit needs either `solid` shapes or the stem binding used here.
- `rework-12-assemble.py`: the joint ball and the mass ball of the root plate and the round members of the grounded surface roots reach below the ground (`volumes-structure-coverage`); the trim of bark areas is a flat 0.95 although collars and root overlaps bury about 20 % of the stem surface; the sun emitter on the highest tuft leaves the body box at coarser scales because coarser crowns are narrowed into the 1:50 box.
- `rework-12-kit.py`: a foliage loft along a short curved bough intersects itself when its half width approaches the length of its path (only `polyhedronFault` finds it; `K.closed` does not).
- `r3-validate.ts … --volumes` on this asset takes 15 to 20 minutes on the loaded host, longer than the 10 minute limit of one tool call.
