# vegetation/weeping-willow — rework report

Asset: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/weeping-willow/🔣️.json`. Hand numbers: `rework-weeping-willow-model.py` (ticket folder; begun by author 12 as `rework-12-weeping-willow-model.py`, continued here). Chisel and carrier of author 12, imported and not copied: `rework-12-kit.py`, `rework-12-assemble.py`, `rework-12-draw2d.ts`. One round (build, draw 2D, render, validate): `rework-weeping-willow-round.sh`. Publishing: `rework-weeping-willow-publish.py`.

## Pass 1 (second round), 2026-11-22

### State found

- No report of this asset existed. `rework/12.md` has only its reference row; batch 12 wrote `rework-12-weeping-willow-model.py` (01:35) and a candidate under `🗑️generated/rework-12/weeping-willow` that was never drawn in 2D, validated or published.
- Source `volumes/vegetation/weeping-willow.json` and catalog `asset.volumes` were identical (round-3 state of 2026-11-21 15:26): nothing was cut off mid-edit.
- Catalog model as rendered (1:50 body, six views): a closed umbrella dome of 223 overlapping slabs that reaches 1 m above the ground, 16.2 x 11.1 x 14.0 m. From afar it reads as a willow, but: no limb is visible, the rig is one drawing plane (19 bones: two limb chains east and west, eight "curtain" bones, two caps), the curtain is a closed shell with no air between tresses, the coarse scales are the 1:50 list (208 to 221 slabs at every scale), the structure members of the curtains end in the air.
- Candidate of author 12 as rendered: right skeleton (trunk, seven limbs all round, boughs, 63 tress bones), but a fountain: tresses left their limbs 30 degrees below the horizon and flared to 19.7 m width, a flat ring of caps on top. Better starting point than the catalog; rebuilt from it.

### Reference (Salix x sepulcralis 'Chrysocoma', open-grown, about 40 years)

| Quantity | Real subject | Catalog model before | Model now |
|---|---|---|---|
| Height | 10 to 14 m | 11.1 m | 10.9 m |
| Crown spread | 12 to 18 m, wider than high | 16.2 x 14.0 m | 16.2 x 14.9 m |
| Trunk at 1.3 m | 0.6 to 0.9 m, deeply furrowed, often leaning | 0.62 m smooth cone | 0.77 m, 18 furrows, leans 0.2 m over 2.6 m |
| Root collar | 1.0 to 1.3 m, 4 to 6 surface roots running 1.5 to 2 m | disc | 1.12 m, five buttress roots of 1.5 to 1.7 m |
| Fork | at 1.8 to 2.5 m into 4 to 8 limbs | at 3.5 m into 2 limbs | at 2.2 to 2.5 m into 7 limbs |
| Limbs | rise at 60 to 75 degrees, 0.3 to 0.4 m at the base, arch over at 8 to 10 m, ends droop, 6 to 7 m reach | 2, in the drawing plane | 6 all round (azimuth 168, 118, 44, -14, -62, -136), 9.8 to 10.6 m long, 0.33 to 0.38 m at the base, plus one centre limb to 10.9 m |
| Boughs (2nd order) | 3 to 5 per limb, 0.08 to 0.12 m, arching | none | 4 per limb, 0.09 to 0.12 m, 2.8 to 4.1 m |
| Withies (3rd order) | 2 to 6 mm thick shoots in tresses, 3 to 8 m, hanging plumb, ending 0.5 to 1.5 m above ground | none (slabs) | 63 tresses, each one bone, 20 to 23 mm bundle axis, 4.3 to 8.4 m, leaving 58 degrees below the horizon and plumb after the first third, ending 0.9 m above ground |
| Leaves | lanceolate 80 to 130 x 10 to 20 mm, alternate along the withies | slabs | foliage strand around each tress, 0.62 m wide and 0.34 m deep, lobed in eight tufts, air between strands |
| Crown form | broad dome, densest in the shell, a room inside; trunk and limbs visible between the tresses | closed dome | caps along the arch of every limb and bough over a ring of separate tresses; trunk and limbs visible |
| Wind | tresses stream to lee up to 40 degrees, limbs sway a few degrees | 8 curtains swing in the paper | 63 tresses swing about the lateral axis, trunk leans |

### Rebuilt

- **Rig** (94 bones): `root-plate`, `trunk-1`, `trunk-2`; per limb a fixed collar and three bones `limb-<k>-a/b/c` cut out of its own axis (k = w, nw, ne, e, se, sw, c); 63 tress bones `curt-<k><n>` at the point of the limb or bough they hang from. Every joint lies in the wood it moves.
- **Tress hinge**: each tress bone rests with its joint axis in the lateral world axis (`twistDeg`, `T/rig-lateral-rest.md`) and its axis in the plane of the paper below its attachment, so the wind pose turns every tress about the same axis wherever it hangs on the crown, and the flat elevation shapes stay exact (78 `lateral-ambiguous` findings of the candidate gone at the root, not by binding the drawings to the stem). `solve()` in the model script states this.
- **Body**: 1:50 and 1:100 trunk, roots, 7 limbs, 28 boughs, 63 withy axes, 35 caps, 63 strands; 1:200 one cap per limb system and a fall per host; 1:500 one cap per limb and 18 falls on the tresses that make the silhouette.
- **Thermal, structure, 2D, decisions**: produced from the same hand model by the carrier (thermal solids by reference to the body solids plus transpiration emitters; a member along every wood axis down to the root plate; every 2D shape redrawn as the outline of the solids it stands for, the foliage of every tress on its own tress bone so that the drawing swings with the model).
- **Poses and clips**: tresses take the old curtain statements (`like`), the trunk its own; the limbs are not turned in the poses (see open list).

### Root fixes forced by findings (all in `rework-weeping-willow-model.py` unless stated)

- Tresses leave their limb or bough 58 degrees below the horizon and are plumb after the first third (the candidate of author 12 flared to 19.7 m); they end 0.9 m above the ground (the first rebuild reached the ground). Outer limbs arch further over (`up` -108 to -110), the west and east limbs rise at 73 degrees, the centre limb is 9.7 m and carries a higher cap.
- Stem stations lie in the drawing plane (`z = 0`).
- Tress bones: joint axis in the lateral world axis (`twistDeg`), bone axis in the paper below the attachment; limbs are not turned in poses. `validateAsset` went from 82 findings (78 `lateral-ambiguous`, 1 `bone-lateral`) to 0.
- Every cap and every strand is its own 2D shape on its own bone (`group_bone`): the drawing tool keeps one polygon per shape, and grouped caps or strands that do not touch in a view lost their detached parts (symbol side view 15.18 m against 16.15 m, 1:500 front 14.15 m against 14.88 m).
- Cap noses reach 0.5 to 0.6 m beyond the limb and bough tips and the strands begin at the attachment, so cap and strand foliage meet at the shoulder.
- Scales: strands (0.62 x 0.34 m) and caps (1.7 to 2.2 m) are far above the smallest feature down to 1:200 (30 mm) and stay individual with the full relief at 1:50, 1:100 and 1:200 (`build`, `blob` in the model script: same number of sides, relief gain 1); 1:500 keeps all 63 strands as smoothed spindles (relief 0.6, 12 sides) and one merged cap per limb system. Reason besides the feature size: with falls on a few tresses the other withies hung bare (20 mm rods over 4 to 8 m: `volumes-silhouette` p95 469 mm, `volumes-cross-scale` p95 712 mm, `volumes-thermal-coverage` +61 % bark).
- Boughs exist at every scale at their true section (`slim`), wood ring steps 0.22 / 0.3 / 0.4 / 0.5 m (`K.RING_STEP`): at 1:500 the tresses of the boughs floated (23 body groups, 43 structure groups), at 1:200 six bough members missed their limb by 38 mm because the coarse polyline of the limb left its true curve.
- `area_trim` of `trunk-bark` 0.735: measured outer surface 15.48 m2 (mesh 21.1 m2 less the buried collars and root overlaps).
- Shared kit `rework-12-kit.py`, one backward-compatible addition in `Tree.build`: an axis with `slim` is not dropped when its base is below the feature (it is then built slender at its true section). No other tree sets it.

### Validation

- Catalog before (unchanged, still published): the round-3 model. NOT replaced in this pass, because the candidate is not `VALID` yet.
- Candidate `r3-validate.ts --file … --volumes --no-print`, runs of this pass: 82 + (not measured) → 1 + 21 → 0 + 11:
  `INVALID file/weeping-willow (file as it is): schema 0, validateAsset 0, checkVolumes 11 — PRINT checks skipped (--no-print)`.
- Passing now at all four scales: schema, `validateAsset`, `volumes-connected` (body and structure), `volumes-thermal-coverage`, `volumes-structure-coverage`, `volumes-feature`, `volumes-bounds`, `volumes-monotonic`. The print checks were never run on the candidate.

### Open (precise, for the next pass)

Rebuild first: `LOOKS="b50|1800|--scale 50" VALIDATE="--volumes --no-print" ./rework-weeping-willow-round.sh` (candidate under `🗑️generated/rework-weeping-willow/weeping-willow/`, deleted at the end of this pass; the script reads the catalog asset as its base, so run it BEFORE publishing or keep a copy of the round-3 catalog file under `🗑️generated/rework-weeping-willow/base/weeping-willow/🔣️.json`).

1. `volumes-silhouette` 1:100 elevation p95 93.8 mm (limit 30), front 93.8 (60), largest 521 / 563 mm; 1:200 elevation 142.5 (60), front 142.5 (120), largest 1042 mm; 1:500 elevation 168.7 (150), front soft 356 (300). Plan passes at every scale, 1:50 passes in every view. Suspected cause, not verified: `rework-12-draw2d.ts` leaves `slender` wood undrawn (`if not s["slender"]` in `rework-12-assemble.py`), and from 1:100 on the upper thirds of the withies between bough and strand, bough tips and limb ends are slender and visible beside the foliage in the side views. Fix to try: draw slender wood as open centre-line strokes on its bone (the round-3 drawing did that), or let the strand begin above the attachment.
2. `volumes-cross-scale` 1:100 against 1:50 elevation and front p95 93.8 mm (limit 33.8), 1:200 against 1:100 90.0 / 97.5 mm (60): the foliage form is the same, so the difference is the sampling along the path (`mass_samples` step = 1.7 x ring step: 0.37 / 0.51 / 0.68 m against relief wavelengths down to 0.74 m on a tress). Fix to try: one along-step for 1:50 to 1:200 (`K.RING_STEP` equal, or relief `(5, 4, …)` with fewer lobes along and no `fine` term).
3. `volumes-cross-scale` 1:500 against 1:200 plan p95 712.5 mm (limit 543.8): the merged cap per limb system (half width 2.3 m) against 5 individual caps. Fix to try: keep the individual caps at 1:500 with relief 0, or narrow the merged cap to the hull of its five.
4. Not rendered and read after the last change: 1:200 and 1:500 body (rendered, the Read tool failed), every thermal and structure sheet, poses `windy` and `storm` and the clips `wind-sway`, `gust`, `diurnal-transpiration` at two instants each, the 2D sheets of all scales. `storm` at 1:50 was read once before the last three changes: all tresses stream to lee about the lateral axis, no crack at the attachments.
5. Body 1:50, crown: the caps form a flat hat at 9 to 10 m with one higher centre cap; a real crown is a rounded dome. Raise the middle boughs (`BOUGHS` elevations 62 / 44 / 30 / 12) or add a second tier of caps on the centre limb.
6. Body, strands: every strand is a rigid lobed spindle 0.62 x 0.34 m; a sculptor's tress is a loose bundle of 10 to 20 withies with single leaves at the fringe. In a pose a strand swings as a rigid rod about its attachment (one bone); a streaming tress is curved. Two bones per tress would cost 63 more bones.
7. Rig: limbs keep their joints and limits but no pose or clip turns them (a limb that leaves the paper cannot be turned with flat shapes; the right fix is `solid` 2D shapes on the limb bones). Tress bone tips lie in the paper below the attachment, up to 0.6 m beside the true tress tip.
8. 2D: every shape is the projection outline of its solids (ruler output), 98 foliage shapes per view; not an economically redrawn drawing, no leaf or bark texture strokes, withies undrawn from 1:100 on.
9. Structure and thermal lists are the carrier's (members = wood cores at 0.55 of the bark radius, thermal = body by reference plus emitter lenses); they pass coverage and connection but were not judged on a sheet in this pass.
10. Decisions texts of 1:200 and 1:500 (`habit`, `drawing` in the model script) still describe caps per limb system and hanging falls; they must be rewritten for the strands kept at every scale before publishing.
11. Print checks (`r3-validate.ts` without `--no-print`) on the candidate.

### Fundamental issues

- `rework-12-draw2d.ts` keeps one polygon per shape: a shape whose solids fall into detached parts in a view silently loses all but the largest (found through extent findings only). It should emit one shape per part or fail.
- `rework-12-kit.py` `Tree.build`: branch origins lie on the true curve of the parent while the parent of a coarse scale is a coarse polyline (ring step up to 2.9 m at 1:500), so thin members miss their parent (worked around with finer `RING_STEP`); axes below the feature were dropped although tresses with bones hung from them (worked around with `slim`); `fine` relief and the number of sides of a foliage loft are hard-wired per scale, which makes 1:50 and 1:100 differ by more than the cross-scale limit (worked around by wrapping `build` and `blob`).
- `rework-12-assemble.py`: `Tree.solve` knew no `twistDeg`, so every hinge of a branch lay across the branch and a wind pose about the lateral axis reached each tress only by its projection (worked around with `solve` in the model script and `settle`).
- Tooling: the Read tool of the agent host stopped answering near the end of this pass (`PreToolUse hook did not respond`), so the last renders could not be read.

## 2026-11-23 00:05 to 00:45 — umbrella lot machinery: rebuild re-measured, published round-3 model repaired

- Catalog file (round-3 model) measured with `--volumes`: `INVALID … checkVolumes 3` (`volumes-connected`: 1:100 structure `member-curt-e3` / `member-curt-w3` 31.2 / 30.8 mm from their limb member; 1:200 body three free fringe masses; 1:500 body five free groups). The cheap validation had hidden this.
- Rebuild: `rework-weeping-willow-round.sh` with `VALIDATE="--volumes --no-print"`, 00:08 to 00:13: `INVALID file/weeping-willow (file as it is): schema 0, validateAsset 0, checkVolumes 11`, the eleven findings of the section above, number for number. Nothing changed in the rebuild; its open list stands. Candidate folder deleted again.
- Repair of the published model (`rework-weeping-willow-catalog-joins.py`): withies for the six fringe masses at 1:200 and 1:500 and for `tress-c23` at 1:500, collar joints `joint-curt-e3` / `joint-curt-w3` at 1:100. Source and catalog file `VALID … schema 0, validateAsset 0, checkVolumes 0` (full proof), check `✓ weeping-willow (1/1)`. Sheet opened: 1:500 body.
- Details: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/rework-machinery.md`, section 9.1.
