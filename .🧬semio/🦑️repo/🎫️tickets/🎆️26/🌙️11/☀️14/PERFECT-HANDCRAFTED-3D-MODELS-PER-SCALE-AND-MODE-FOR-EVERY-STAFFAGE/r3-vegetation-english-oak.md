# Round 3 — `vegetation/english-oak` (mature open-grown English oak, *Quercus robur*)

State: DONE (2026-11-20). Catalog asset VALID (`r3-validate.ts vegetation/english-oak --catalog-only`: schema 0, validateAsset 0), `volumes-check.ts vegetation/english-oak --product`: **PASS (314.6 s)** at 1:50, 1:100, 1:200 and 1:500.

## 0. Resume and scripts

An earlier author had done most of the work and was cut off before writing this report. What I found on 2026-11-19 23:24: catalog = candidate document (`🗑️generated/r3-english-oak/🔣️.json`), VALID, and the check FAILED in `volumes-cross-scale` at 1:100 and 1:200 (p95 0.1425 against 0.109 m, 0.285 against 0.2175 m). That author had put this down to a tool defect in `tool-requests.md` ("r3 vegetation/english-oak (2026-11-19)"). I measured it again with one common window. The 1:100 crown really was 0.21 to 0.41 m away from the 1:50 crown, and 1:200 was 0.32 m away from 1:100. So the tool only hid how large the gap was. The model was at fault, because the coarse crowns had been invented anew and were not generalised from the finer ones. I fixed this in the model (section 4).

Scripts in the ticket folder (all hand-stated, no form generators):

- `r3-english-oak-tree.ts`: the tree in world metres at every scale: every station, clump, boss, tuft and spur, plus the per-scale selection tables `HIDDEN_100`, `MERGED_200`, `MERGED_500` and `SHADE_500`.
- `r3-english-oak-model.ts`: writes the volumes source, redraws every 2D shape as the true view of its scale's body (ruling E1), sets the anchors, the decisions and the candidate document.
- `r3-english-oak-splice.py` writes the roots (`rig`, `anchors`, `representations`). `r3-english-oak-run.sh` runs the whole chain.
- New in this round:
  - `r3-english-oak-hidden.ts`: how far each lobe moves the crown outline in elevation, front and plan. With the greedy option it removes lobes one at a time, smallest effect first.
  - `r3-english-oak-diff.ts`: where two scales' outlines part, with the solids involved, plus enclosed holes.
  - `r3-english-oak-look.ts`: zoomed shaded views, for example wood only.
  - `r3-english-oak-cross.ts` was extended with a `candidate` option and a pair filter.
- Backups of the state I found: `🗑️generated/r3-english-oak/{tree,model}-before-r3b.ts` and `source-before-r3b.json`.

Resume: `sh r3-english-oak-run.sh`, then `$BUN volumes-check.ts vegetation/english-oak --product`.

## 1. Reference

| quantity | real English oak (open-grown, mature) | source | asset |
|---|---|---|---|
| height | 18-25 m (open-grown); 15-20 m in parks | Pretzsch et al. 2015 (`provenance.references` pretzsch-2015), forestry practice | 18.2 m (`dimensions.heightM` 18.226) |
| crown spread | about equal to height when open-grown; crown radius about 7-9 m at DBH 0.8-1.0 m | Pretzsch 2015 (crown radius against DBH, *Q. robur*) | 18.3 x 15.0 m (width x depth) |
| DBH | 0.8-1.2 m for this size | Pretzsch 2015; forestry tables | 0.80 m (bole section of the rig), flare 0.92 m collar, buttresses to 1.13 m |
| bole | short, 2.5-4 m, then divides into a few heavy limbs (decurrent, broad crown) | botany (Mitchell, Trees of Britain and Northern Europe) | fork at 3.0-3.4 m |
| scaffold limbs | 3-6, wide-angled (50-80 degrees from vertical), heavy, sinuous, lower ones near horizontal | botany | 4 scaffolds at 73/70 degrees (low) and 48/37 degrees (mid), plus 2 ascending branches, 2 leader branches, 4 secondary boughs |
| pipe model | cross-section is about conserved over a fork | Shinozaki 1964 | bole 0.50 m² against the low limbs 0.10 + 0.08 and the upper trunk 0.28 = 0.46 m² |
| crown form | broad, irregular dome of clustered leaf masses with sky windows; foliage held in clumps at the ends of limbs | botany | 17 clumps in 3 tiers plus a shaded inner crown; sky windows in the plan |
| leaf | 7-12 cm, lobed | botany | leaf marks of the 1:50 drawing |
| wood | green oak density about 1000 kg/m³ | Wood Handbook 2021 Table 5-3 (wood-handbook-2021) | mass 5971 kg (kept) |
| bark | deeply fissured plates on old stems | botany | 18 ridges of 28 mm at 1:50 |

`dimensions` agree with the reference and were not changed.

## 2. Audit (what I found and judged)

- **Rig**: the earlier author had already given the planar rig lateral offsets (`BONE_Z`), so the 12 woody arms fill the plan. Joint heights and lengths match the reference (fork 3.0 m, upper trunk to 5.6 m, leader to 10 m, leader top to 13.4 m). No joint is in the wrong place. I kept the rig.
- **1:50 body**, looked at as wood only (`look.s50.body.wood...png`) and cut:
  - root flare with seven buttress roots, tapered bole with bark ridges, smooth Catmull-Rom limb lofts that start inside their parents (no gaps), 20 branchlets, 17 clumps with 6-11 bosses each, tufts at the branch tips.
  - Remaining deviations are stated in section 6.
- **1:100 / 1:200 / 1:500 crowns (root defect of the volumes)**. The coarse crowns were not generalisations of the finer crown:
  - 1:100 used separately invented `coarse` bosses (seat 0.55).
  - 1:200 used bare main lobes grown by 0.32 m.
  - 1:500 used 14 invented broad lobes.
  - Measured outline distance with one common window of 1.5 m:

    | pair | elevation | front | plan |
    |---|---|---|---|
    | 1:100 vs 1:50 | 0.21 m | 0.30 m | 0.41 m |
    | 1:200 vs 1:100 | 0.32 m | 0.32 m | 0.32 m |

  - In the drawings this showed as visibly different crown outlines from scale to scale.
- **2D**: these were true views of the old bodies. Stale texts:
  - The 1:500 `silhouette` and `retained` decisions ("crown outline of eight lobes", "two lowest limbs") no longer described the drawing.
  - The volumes decisions of 1:100 / 1:200 / 1:500 described the invented crowns.
- **Thermal / structure**:
  - Regions and areas are within the rules at every scale (section 5).
  - Structure is the idealised wood (tapered members per bone along the true axes, joints, root-plate support, masses), never the body.
  - No defect found.

## 3. Root fixes (catalog, outside `volumes`)

| member | old | new |
|---|---|---|
| `representations.{50,100,200,500}.geometry` (all views) | true views of the old bodies | true views of the new bodies, redrawn by `redrawScale` (same ids, bones, roles, z order). 1:500 now shows the two ascending branches and the boughs below the crown, and the three plan sky windows. 1:100/1:200 crowns follow 1:50. |
| `representations.500.decisions.silhouette` | "...one crown outline of eight lobes...", "underside rises into a notch above the bole between two low crown lobes" | broad lobed dome in three tiers; low limbs, ascending branches and boughs under one outline of about a dozen scallops; plan scalloped (en, de) |
| `representations.500.decisions.retained` | "the bole with its root flare and the two lowest limbs below the crown" | bole with root flare, the two lowest limbs, and the ascending branches and boughs where they show below the crown (en, de) |
| `anchors` (transpiration) | on the old lens positions | recomputed by the model on the new lobes; unchanged at 1:50 |

Not changed: `rig`, `dimensions`, `thermal.regions[].areaM2` (trunk-bark 25, limb-bark 42, leaf-sun 250, leaf-shade 200), poses, clips, `structure`.

## 4. What is modelled (volumes)

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 223, unchanged: root flare and 7 buttress roots, bole, 18 bark ridges (texture), upper trunk, leader, leader top, 4 scaffold limbs and 4 branches, 2 ascending and 2 leader branches, 4 secondary boughs, 20 branchlets (slender), 17 clumps with 6-11 bosses each, 7 tufts | 217: every body solid by reference in its region (except texture), 12 outlet lenses | 56 |
| 1:100 | 133: one flared root loft plus the 7 buttresses as 3-station lofts. Bole, upper trunk and leader with fewer stations; each scaffold limb one loft with its branch. Small branches and boughs keep their stations with coarser sections (sub 2 instead of 3). No ridges, no branchlets. Crown: all 17 main lobes, plus every 1:50 boss that shapes an outline in elevation, front or plan; the 47 bosses that only fill the inside (`HIDDEN_100`) are left out. 7 tufts. | 143 | 48 |
| 1:200 | 85: four short trunk lofts, limbs with branches on their 1:200 stations, ascending branches and boughs as 2-station lofts. Crown: 17 main lobes plus the bosses left after a greedy reduction of the 1:100 crown, in which each left-out boss moved the remaining outline by less than 0.20 m (`MERGED_200`, 35 bosses). 5 tufts. | 93 | 31 |
| 1:500 | 48: root flare, bole, leader, the 4 limbs on their main stations, the 2 ascending branches and 4 boughs as 2-station lofts at least 80 mm thick. The crown is one mass on the leader: 17 main lobes plus the bosses that survive a further greedy reduction at 0.40 m (`MERGED_500`). The top boss and the outermost bosses of the two low clumps stay, so height and spread are exact. The upper shade mass is swollen (`SHADE_500`) to close specks of a few cm between the top clumps. The three plan sky windows (about 0.5 m) stay open as at all finer scales. | 56 | 27 |

Every scale is distinct (`volumes-copied` 0 everywhere). Per-scale decisions (en, de) were rewritten for body and thermal at 1:100, 1:200 and 1:500.

Cross-scale after the fix (product, p95 against limit):

| pair | elevation | front | plan |
|---|---|---|---|
| 1:100 vs 1:50 | 0.0225 / 0.0375 | 0.015 / 0.030 | 0 / 0.030 |
| 1:200 vs 1:100 | 0.12 / 0.15 | 0.105 / 0.135 | 0 / 0.06 |
| 1:500 vs 1:200 | 0.244 / 0.3375 | 0.225 / 0.30 | 0.319 / 0.4125 |

None of these sit in the saturated last bin. The 2D pair agrees with the 3D pair within one feature.

## 5. Final check

`PASS (314.6 s)`, 42 rule lines ok.

- **Silhouette IoU** (elevation / front / plan):

  | scale | IoU |
  |---|---|
  | 1:50 | 0.995 / 0.995 / 0.993 |
  | 1:100 | 0.995 / 0.996 / 0.994 |
  | 1:200 | 0.996 / 0.996 / 0.994 |
  | 1:500 | 0.994 / 0.997 / 0.996 |

- **Bark area deviation**:

  | scale | trunk-bark | limb-bark |
  |---|---|---|
  | 1:50 | 0.022 | 0.109 (limit 0.15) |
  | 1:100 | 0.042 | 0.033 |
  | 1:200 | 0.010 | 0.108 |
  | 1:500 | 0.012 | 0.105 (was 0.374) |

  The 1:500 limb-bark value improved because the ascending branches and boughs are now in that body.
- `r3-validate.ts`: VALID (candidate and catalog).

The tool request of the earlier author about the saturated 2D window (`tool-requests.md`, "r3 vegetation/english-oak") still describes a real measurement quirk. It is no longer the reason this asset fails, because the asset does not fail.

## 6. What I saw in the final renders

All files are under `🗑️generated/r3-sheet/vegetation/` and `🗑️generated/r3-2d/`.

- **1:50 sheet** (six directions; body unchanged):
  - Broad lobed dome, slightly wider than tall in the side views, short bole, low limbs nearly horizontal and reaching out under the crown, shaded core visible in the gaps.
  - Plan: closed scalloped crown.
  - Three-quarter: tiers read clearly.
- **1:50 wood only, from the side** (`look.s50...wood...png`): joints closed, flare and buttresses right, bark ridges visible.
- **1:100 sheet**: same outline as 1:50 from every direction. The crown is a little smoother inside (interior bosses gone). The buttress roots are back at the foot in front, back and three-quarter.
- **1:200 sheet**: clumps read as lobed masses, each with its main bosses; tiers and limbs are clear in all six panels; no blobs.
- **1:500 sheet**: one scalloped dome of about a dozen bumps in every side view, two low limbs with the ascending branches visible under the crown, round flare.
- **1:50 thermal, cut**: trunk bark red, limb bark green, sun leaves around the shaded core, nothing poking out.
- **1:50 structure**: members along the true axes inside the ghost, support disc at the foot.
- **Poses**:
  - Pose `storm` at 1:100: crown and limbs bend together, no joint opens.
  - Clip `gust` at t = 1 s at 1:500: crown on the leader moves with it, limbs stay attached.
- **2D**:
  - 1:200 and 1:500 candidates looked at (`🗑️generated/r3-2d/file/r3-english-oak.s{200,500}.png`); 1:100 and 1:100 `storm` rendered as catalog drawings.
  - 1:500 has clean outlines, no slivers left inside the crown after `SHADE_500`, and three small plan windows.

## 7. Still imperfect (stated plainly)

- **Central leader**: the leader and leader top run as a thin, nearly straight axis up to 14.6 m inside the crown. A mature open-grown oak usually loses its leader and divides into several equal limbs. The axis is hidden in every exterior view by the clumps (visible only in cut and wood-only views). Changing it means changing the rig chain (`leader`, `leader-top`, the foliage carriers `fol-top`, `fol-core`) with its poses and clips, which I did not do.
- **Limb thickness**: scaffold limbs are 0.29-0.36 m, a little slender for an oak with a 0.80 m bole. The committed `limb-bark` area of 42 m² (C16, `area-decisions.md`) and the measured 1:50 area (deviation 0.109 of 0.15) leave no room for heavier limbs without changing physical data, which the brief forbids.
- **Branchlets at 1:50**: the 20 slender branchlets are straight and stand off their branches like a comb. They lie inside the foliage and do not show in any exterior view. Bending or forking them would be better.
- **Branch tips in poses**: in pose `storm` at 1:100 the tip of the low west branch stands about 0.2 m proud of its clump in the elevation. In `calm` it is at the clump surface.
- **1:500 crown binding**: the 1:500 crown is bound to `leader` as one mass (as before). Under sway it moves as a whole, not clump by clump. This is accepted at that scale.
- **Stale 1:100/1:200 2D texts**: the 1:100 and 1:200 2D decision texts ("eight foliage masses", "two leaf-cluster arcs per mass") still match the drawings, which keep one filled mass per foliage bone. I left them unchanged.
