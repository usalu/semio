# vegetation/small-leaved-lime — rework report

## Pass 1 (second round), 2026-11-22

### State found

- Catalog file and `T/volumes/vegetation/small-leaved-lime.json` were identical (md5 of `volumes`) and both still the old model of 2026-11-21: 31 bones, a closed ovoid of merged ellipsoid bosses, limbs east and west only.
- No report existed. `rework/12.md` holds only the reference row of author 12. An unnamed earlier author left `rework-small-leaved-lime-model.py` (hand numbers on the kit of author 12: `rework-12-kit.py`, `rework-12-assemble.py`, `rework-12-draw2d.ts`), `rework-small-leaved-lime-round.sh`, `rework-small-leaved-lime-publish.py` and an unpublished candidate under `🗑️generated/rework-small-leaved-lime/` that was `VALID` without `--volumes` (04:19). Nothing of it was in the catalog.
- Judged by rendering: the candidate (straight stem to the tip, 25 branches all round the compass in five tiers, 92 bones, hanging foliage masses per twig) is far better than the catalog ovoid. This pass continues from the candidate script.

### Reference (Tilia cordata, open-grown park tree, about 60 years)

| Quantity | Real subject | Old catalog model | Candidate found |
|---|---|---|---|
| Height | 13 to 16 m at 60 years (final 20 to 30 m) | 14.2 m | 14.4 m |
| Crown spread | 8 to 10 m, 0.6 to 0.7 of the height; dense ovoid to broad conical with a rounded top, widest at 35 to 45 % of the height | 9.0 x 8.9 m, closed egg | 9.3 x 8.7 m, widest at 4 to 6 m |
| Stem at 1.3 m | 0.45 to 0.5 m | about 0.45 m | 0.47 m |
| Root collar | 0.75 to 0.9 m across, 4 to 6 short root spurs of 0.8 to 1.2 m, often a burr (lignotuber) with sprouts | cone | 0.8 m, four spurs, burr lobes |
| Clear bole | 2.2 to 3 m in a park (crown lifted) | 2.5 m | 2.6 m |
| Stem habit | excurrent: one straight stem to the tip, taper about 30 mm per metre in the crown | hidden | one axis, 0.47 m to 16 mm |
| Branches of first order | 20 to 30, in whorl-like tiers of 4 to 6, each tier turned against the one below; base 100 to 160 mm in the lower tiers, 60 to 90 mm above; leave at 30 to 40 degrees above the horizon low down, 50 to 60 degrees high up; arch over, lowest tips at or below the horizontal; length 4 to 5 m (longest in the second tier) | 8 limbs, east and west only | 25 in five tiers, base 148 to 64 mm, 32 to 54 degrees, 5.0 m longest in tier 2 |
| Second and third order | alternate, two-ranked (distichous), boughs 40 to 60 mm, twigs 15 to 25 mm, zigzag, drooping | none | 2 to 3 boughs per branch, 2 twigs per bough |
| Foliage | heart-shaped leaves of 40 to 70 mm, two-ranked on drooping twigs: flat, hanging sprays that shingle over each other; crown closed from outside, shade leaves inside; leaf area about 250 to 350 m2 | ball bosses | one hanging lobed mass per twig, bough end and branch end, shade masses inside |
| Epicormic shoots | a brush of 10 to 30 thin upright shoots (8 to 20 mm) of 0.5 to 1.5 m around the collar and burr, leafy over their whole length | grape of small balls beside the bole | six shoots of 1.4 to 1.9 m with sausage-like leaf masses standing clear of the bole |
| Fresh mass above ground | 1.5 to 2.2 t | 1553 kg | 1990 kg |

### Defects of the candidate found on its sheets and in its first measured validation

`r3-validate.ts --file <candidate> --volumes` on the candidate as found (25 min 43 s on the loaded host): `INVALID … schema 0, validateAsset 0, checkVolumes 12`.

| # | Where | Defect | Kind |
|---|---|---|---|
| 1 | 1:50 body, collar | epicormic shoots were six fat sausages of 0.34 m standing 1.9 m high beside the bole (read as a tulip), not a brush of thin leafy shoots | form |
| 2 | 1:50 body, top | pointed, thin top with sky between 10 and 12.5 m (three branches in tier 5, four short top boughs); a lime of this age has a rounded, closed top | form |
| 3 | 1:100 body | `volumes-connected`: leaf masses of the epicormic shoots 2 and 3 floated (their shoots of 24 mm are below the 37 mm that print at 1:100 and were dropped, the leaves were not) | real defect |
| 4 | 1:100 against 1:50, 1:200 against 1:100 | `volumes-cross-scale` in all three views, boundary p95 at the saturation (142.5 / 285 mm): 1:100 had one wide, tall mass per bough (half height 0.75 m) where 1:50 has twig sprays of 0.5 m with sky between; 1:200 had an inner and an outer slab per branch | real defect (the scales were different trees in detail) |
| 5 | 1:500 thermal | `volumes-thermal-coverage`: outer bark area 80.0 m2 against 59.3 m2 declared (35 %): at 1:500 every branch is a stick of 240 mm (print minimum), so the exposed inner 1.1 m of 25 branches added about 20 m2 of bark that does not exist | real defect |
| 6 | 1:50 and 1:100 structure | `volumes-structure-coverage`: the members of the four root spurs lay up to 21.6 mm below the ground (round member on the axis of a root whose body is cut at the ground) | real defect, in the kit |
| 7 | poses, clips | the kit solved every pose offset as the share of the turn about the joint axis of the PARENT; since the rig change of 2026-11-22 (`rig-lateral-rest.md`) the product turns an offset about the bone's own rest axis | real defect, in the kit |
| 8 | 1:200, 1:500 body | epicormic foliage a ball of 0.7 m sitting 1 m up the bole (reads as a burl) | form |
| 9 | 1:500 body | the crown reads as shingled scales (pine cone); root spurs absent | form, open |

### What this pass changed (hand numbers in `rework-small-leaved-lime-model.py`, kit fixes in `rework-12-kit.py`)

- **Rig**: 95 bones (catalog 31, candidate 92): new fourth branch of the top tier `ls5a-collar`, `ls5a`, `fol-s5` (south, 11.4 m); tier 5 re-set (w 175°, e −5°, n 64°, s −116°; 50 to 52° rise, 2.5 to 2.9 m, arching 16 to 18°). All other joints as found: every branch hinge lies on the bark of the stem through a fixed collar bone, the hinge axis horizontal across the branch, so a branch bends in its own vertical plane.
- **Kit, pose offsets** (`Tree.offsets`): the offset is now the share of the turn about the own rest joint axis of the bone (`frames[bone]["w"]`), as the product turns it. Affects every tree built with the kit from now on (see Fundamental).
- **Kit, root members**: the core (structure member) of an axis that is cut at the ground is cut at the ground too.
- **1:50 body**: top tier and the four top boughs longer and flatter (42 to 48°, 1.0 to 1.6 m) with larger end masses (half width 0.8 / 0.6 m) so the top closes into a rounded dome; epicormic shoots: ten instead of six, 21 to 24 mm thick, 0.7 to 1.3 m long, spread over 140° on each side of the collar at 40 to 66°, each with a short lobed leaf mass of 0.42 m.
- **1:100 body**: the same hanging foliage masses as 1:50 at the places where the twigs carry them (meshed for 15 mm), twigs of 24 mm left out, boughs from 37 mm kept; one leading epicormic shoot per side with one leaf mass that stands for the brush.
- **1:200 body**: one hanging mass per bough (half width 1.2 m per 2.2 m of bough, half height 0.6 m) plus the branch-end masses of the finer scales; the innermost bough of the three lower tiers is the shade foliage. Replaces the inner and outer slab per branch.
- **1:500 body**: crown masses per branch sag more towards the tip (0.5 / 0.35 m) for a rounder outline; the shaded core along the stem now reaches from 3.2 to 10.6 m with 1.9 m radius, so the inner ends of the branches lie in shade foliage as in the tree.
- **Thermal**: regions follow the body solids of each scale (trunk bark, limb bark, epicormic stem, sun leaves, shade leaves inside, epicormic leaves) plus emitter domes; no list was authored separately.
- **Structure**: the wood itself, connected from every branch end to the root plate; supports at the root plate, masses on the heaviest segments.
- **2D**: all three views of all four scales re-drawn from the new solids by `rework-12-draw2d.ts` (ruler output, see what remains).
- **Decisions** (en, de) of 1:50, 1:100, 1:200 rewritten for the new content.

## Pass 2 (second round), 2026-11-22 14:20–17:30, cut off; finished by the umbrella finisher 20:40–21:10

State found by the finisher: the catalog still held the publication of 13:58, which the author's own full run of 14:05 (`final.log`) had measured `INVALID … checkVolumes 10` (among them `volumes-thermal-coverage` 1:200, 33 % against 30 %). The author then worked on the candidate (`🗑️generated/rework-small-leaved-lime/small-leaved-lime/🔣️.json`, built 17:22 from `rework-small-leaved-lime-model.py` of 17:22:05) through five full validations (`v1.log` … `v5.log`): 1:500 against 1:200 in plan, thermal area at 1:200 and of `limb-bark` at 1:50, a floating leaf mass at 1:200, the 1:100 plan drawing, and print findings at 1:500 that moved from shape to shape (`ls3a`, `ln5a`, `ln3a`). The last run ended after the cut-off (17:30): `INVALID … validateAsset 2, checkVolumes 0`, both `PRINT`: feature of `ln3a` 0.185 mm at 1:500. No report was written for this pass; the model script holds its numbers.

Cause of the print finding (measured with `finisher-print-probe.ts`: removing `ln3a` from the elevation makes 1:500 suitable, removing it from front or plan does not): the branch `ln3a` grows away from the viewer of the side elevation and lies behind the stem; only a wedge of 95 mm shows beside the stem at 9 m. Which far-side branch shows such a wedge changes with every small change of the model, which is why the finding wandered.

Finished: `rework-small-leaved-lime-print500.py` (new, one-shot on the built candidate, refuses another candidate) draws the underside of `ln3a` in the 1:500 side elevation 30 mm lower at mid length (0.06 mm on paper; the solid is unchanged). Then `rework-small-leaved-lime-publish.py`.

Proof:
- Candidate, 20:50: `r3-validate.ts --file … --volumes` → `VALID file/small-leaved-lime (file as it is): schema 0, validateAsset 0, checkVolumes 0`.
- Published 20:52 (catalog asset and `volumes/vegetation/small-leaved-lime.json`, source = catalog). Check CLI: `✓ small-leaved-lime (1/1)`, 1:500 min feature 0.224 mm, all four scales suitable.
- 21:05: `r3-validate.ts vegetation/small-leaved-lime --catalog-only --volumes` → `VALID vegetation/small-leaved-lime (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` (ten minutes on the loaded host).

Sheets read at full size by the finisher (candidate, `--file`): 1:50 body six views, 1:500 body six views, 2D 1:500. Verdict: one straight stem with root spurs and burr, five tiers of rising branches, hanging foliage masses per bough with shade foliage inside, a closed rounded top; 1:500 is modelled for its scale (one mass per bough, 240 mm limbs), not a thinned list; the 2D drawing agrees with the model. Rendered but not read: 1:50 structure, 2D 1:50.

Remains imperfect: the foliage masses read as stacked cushions, more so at 1:500; the 2D drawings are projected outlines of the solids (`rework-12-draw2d.ts`), not economical hand-drawn shapes, and a rebuild of the candidate can move the print wedge to another far-side branch (then `…-print500.py` refuses and the wedge has to be measured again); the asset file is 22 MB (574 body solids at 1:50), which makes every tool slow; poses, clips, thermal and 1:100 / 1:200 were not looked at by the finisher.
