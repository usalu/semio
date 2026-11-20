# Round 3 — `vegetation/apple-tree` (full-standard orchard apple, *Malus domestica*)

State 2026-11-19: catalog asset spliced and valid, `volumes-check.ts vegetation/apple-tree --product` **PASS** at every scale. The work continues an earlier author's cut-off session (scripts `r3-apple-tree-*`, source volumes, an unwritten candidate document); what was found and what was redone is said in section 0.

Scripts (all in the ticket folder): `r3-apple-tree-model.ts` (every limb section, branch, lobe, apple and outlet stated by hand per scale; writes `volumes/vegetation/apple-tree.json` and the candidate document), `r3-apple-tree-kit.ts` (tube and lobe conversion, outlines), `r3-apple-tree-draw.ts` (ruling E1: the 2D shapes as true views), `r3-apple-tree-roots.py` (writes named members of the candidate into the catalog file, everything else must stay equal), `r3-apple-tree-cross.ts` (where two scales' outlines differ), `r3-apple-tree-frames.ts`, `r3-apple-tree-probe.ts`.

## 0. What was there and what was redone

- Found: `r3-validate` VALID, check FAIL on 10 rule lines (silhouette at all scales because the redrawn 2D had never been written to the catalog, cross-scale, bark areas at 1:50 and 1:100, structure outside the body at 1:500). The catalog still carried the round-2 volumes; only the lateral bone offsets `z` had been written.
- Seen in the renders of that state and rejected: the crown was two stacked tiers of lobes with a waist at 4 m ("double burger"); at 1:100 the lobes were flat discs; at 1:500 it was two flying saucers and a butterfly in plan; the candidate elevation drew a bulb under the ground line (a disc of the full flare radius at every loft section, drawing height 6.59 m); foliage 1.5 m beyond any wood in the quarters between the scaffolds; apples only on the three faces towards the drawing viewpoints; outlets in lobe tops that are buried by the ring above; square 1.4 m support plate.
- Kept: the woody model (root flare with buttress roots, stem, leader, scaffold lofts with collars, fruit branches, shoots with spurs), the lateral offsets, the tool scripts. Redone: the whole crown at all four scales, fruit, outlets and their anchors, the tube outline at joints, the fruit marks of the drawings, the missing root-flare shapes, the layer of the scaffold bases, the support, the trunk top, the decisions, the bark areas.

## 1. Reference

| item | real subject | source | asset |
|---|---|---|---|
| form | Hochstamm (full standard) on seedling rootstock, about 30 years: clear stem, leader with 3-4 scaffold limbs (Oeschberg crown), fruit branches of second order, fruit wood; crown broad and rounded, wider than high, flattish top, drooping skirt | pomological practice (Oeschberg training), `provenance.notes` | as modelled |
| clear stem | at least 1.6 m, 1.8 m in current nursery standards | German Hochstamm definition | 1.6-1.7 m (`trunk` ends 1.6 m, fork to 1.8 m) |
| height / crown width | 6-8 m / 6-9 m for a mature standard | Pretzsch et al. 2015 (crown size of urban trees), orchard practice | 6.35 m / 6.79 m x 6.2 m (`dimensions`, unchanged) |
| stem diameter | 0.30-0.45 m at 30 years | orchard practice | 0.38 m (`trunk.section`), 0.39-0.42 m modelled with the swelling under the fork, flare 0.6 m at the ground |
| branch taper | cross-section area is about conserved over a fork (pipe model): stem 0.113 m² against two lower scaffolds 2 x 0.031 + leader 0.071 = 0.133 m² | pipe model (Shinozaki 1964), rig sections | consistent, rig sections kept |
| scaffold angle | 45-60 degrees from the vertical at the stem, arching outwards to near horizontal under the fruit load | Oeschberg training | lower tier 50/52 degrees then 78 degrees; upper tier 27 degrees then 44 degrees (rig, kept) |
| fruit branches | several per scaffold, 2-7 cm thick, 1-2 m long; tertiaries | `model-findings.md` row 36 | 14 of o0.04-0.07 m, 0.8-1.8 m, 8 forks at 1:50 |
| leaf | ovate, 6-10 cm | botany | 8.5 cm leaf marks in the 1:50 elevation |
| apple | 7-8 cm at harvest | `provenance.notes` (0.2 kg) | spheres o0.08 m |
| crown base | 1.8-2.4 m with a drooping skirt | orchard practice (clearance for mowing and grazing) | 2.3 m |

`dimensions` were not changed: they agree with the reference.

## 2. Audit against the reference

- **Rig**: joints and lengths are plausible (fork 1.55-1.6 m, upper tier 0.5 and 0.7 m above it, leader to 5.3 m, scaffold lengths 3.4-3.9 m in two parts). The rig is planar by construction (bones turn in the elevation plane; `z` is an offset). Defect found by the earlier author and fixed by him at the root: every limb lay in the plane z = 0; the offsets now carry the limbs to 189, 9, 143 and 323 degrees in plan. Remaining deviation, stated plainly: the lower tier is two opposite limbs 9 degrees off the plane and the upper tier two limbs 37 degrees off it, so the four scaffolds stand at 9 / 143 / 189 / 323 degrees instead of roughly every 90 degrees. Turning them further is not possible without lengthening the limbs beyond their declared bone lengths, masses and sections (at 115 degrees the upper west limb would be 3.1 m instead of 2.2 m and leave the crown). The two empty quarters are carried by the four stoutest second-order limbs (o0.07 m, 1.7-1.8 m, on `sc-w1a`, `sc-e1a` and twice on `ld`).
- **2D, all scales** (round 2 state): constructed views that contradicted each other and the rig (front and plan drew `fol-w1`/`fol-e1` as left and right although both lie fore and aft; patch shapes `crown-fill-*`, `sc-*-mass`; fruit drawn o0.18 m in front and plan; no root flare at 1:200 and in the 1:500 front; no shape for the lower leader in the elevations).
- **Solids** (round 2): 45 solids at 1:50, trunk loft buried 0.35 m in the ground and running as a cone through the crown, 4+4 limbs, five fruit (`model-findings.md` rows 36, 37).
- **Thermal data**: `trunk-bark` 3.58 m² and `limb-bark` 6.08 m² are below a hand calculation of the tree the rig describes (section 3).

## 3. Root fixes (catalog asset, outside `volumes`)

| member | old | new |
|---|---|---|
| `rig.bones[].z` (earlier author, already in the catalog) | none | `sc-w1b` -0.2184, `sc-e1b` 0.2371, `sc-w2b` 0.7783, `sc-e2b` -0.7598, shoots and foliage bones accordingly |
| `anchors` `tp-sun-1..7`, `tp-shade-1..5` | points of the round-2 crown | 4 cm outside the outlet patch of the lobe that carries them (same bones) |
| `representations.*.geometry` | constructed views | true orthographic views of each scale's body, shape by shape, same ids, roles, bones, tones; removed `crown-fill-*`, `sc-*-mass` and the arcs of masses that are hidden in a view; added `stem-fork` (bone `ld`) in the elevations and in the 1:500 front, `root-plate-mass` at 1:200 (elevation, front) and in the 1:500 front; fruit marks are the apples of the body that are visible from the viewpoint (true size); scaffold bases are drawn under stem and leader (z 0.93) so that no butt end crosses the stem |
| `representations.*.decisions` | counts of the old drawings | `silhouette` at every scale, `retained` at 1:100 and 1:200, `omitted` at 1:100 rewritten (en, de) |
| `thermal.regions[trunk-bark].areaM2` | 3.58 | 4.8 (factor 1.34; by hand: flare 0.66 + buttress roots 0.7 + stem 1.75 + lower leader 1.72) |
| `thermal.regions[limb-bark].areaM2` | 6.08 | 9.1 (factor 1.497, the limit of C16; by hand 9.4 net: scaffolds 6.45, upper leader 0.52, fruit branches 2.5, forks 0.3). The remaining 0.3 m² is listed for the reviewer in `area-review.md` |

Poses and clips were not touched (no joint moved in the plane). Checked: sheet of pose `gale-windfall` at 1:50 (all joints closed, crown follows its limbs) and the 2D of pose `harvest-laden`.

## 4. What is modelled

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 192: root flare and 5 buttress roots, stem with swelling under the fork and a knot, trunk top rounded into the leader, leader in 2 parts, 8 scaffold parts as swept lofts with collars, 14 fruit branches, 8 forks, 4 shoots with 12 spurs, 83 lobes (skirt 12, widest ring 12 at 3.25 m, shoulder 10, upper rings 9 and 8, top 7, 14 small lobes between the rings, 11 shade lobes), 53 apples all round | 204: every body solid by reference in its region, 12 outlet lenses in free lobe faces | 42: wood under the bark of the 16 load-bearing bones, 15 joints, round root plate o1.6 m, 3 stem masses and 7 foliage masses |
| 1:100 | 110: one flared root loft, limbs with 4-5 sections, 8 fruit branches, 4 single shoots, 59 lobes, 27 apples | 118, 8 outlets | 38 |
| 1:200 | 43: stem parts and 8 scaffold parts with 3 sections, 21 masses (ring of 8, 6 shoulder, 4 upper, top, 2 cores), 10 apples | 52: shoots as slender twigs only here, 5 outlets | 38 |
| 1:500 | 13: flare, stem, lower leader, 10 crown masses on `ld` | 31: upper leader, scaffolds, shoots and 3 apples as hidden solids inside the crown, 2 outlets | 23 |

The crown is one broad rounded dome: outer faces of the lobes lie on the envelope r(az) x (1 - ((y - 3.25)/3.1)^2.5)^0.4 with r = 3.39 m fore and aft and 3.10 m sideways, widest at 3.25 m, top 6.35 m, skirt down to 2.3 m; the shoulders are as broad as the ends of the upper scaffolds need (tip of `sc-e2b` at radius 2.53 m, 5.48 m high).

Final check: `PASS (109.0 s)`; silhouette IoU 0.997 / 0.993 / 0.995 (1:50), 0.997 / 0.994 / 0.995 (1:100), 0.998 / 0.991 / 0.997 (1:200), 0.996 / 0.996 / 0.997 (1:500); cross-scale p95 0.094 / 0.109 / 0.098 m against 0.109 (1:100), 0.19 / 0.15 / 0.17 (1:200), 0.17 / 0.11 / 0.13 (1:500); bark areas at 1:50 trunk 4.77 (0.7 %), limb 9.46 (4.0 %), twig 0.166 (10 %). `r3-validate.ts vegetation/apple-tree --catalog-only`: VALID, schema 0, validateAsset 0.

## 5. What I saw in the final renders

- **1:50 body** (`r3-sheet`, six views, and one view from below at turn 215, tilt -18): from every side a broad, flat-rounded orchard crown on a short stout stem; no waist; buttress roots read as roots; the scaffolds leave the fork at about 40 degrees above the horizontal and disappear into the skirt; from below the fruit branches and the open underside show; apples sit on all sides. Top: closed dome of lobes, a rounded cap. Three-quarter: reads as an apple tree at first sight.
- **1:50 structure**: stem, leader and four scaffolds as tapered wood on the true limb axes, joints at the rig joints, round plate at the ground; in plan the uneven limb azimuths (section 2) are plain to see.
- **1:50 thermal, cut**: shade lobes inside the sun lobes, wood inside both, nothing pokes out; lenses lie in the lobe faces.
- **1:100 body**: same tree with broader lobes, flared root as one loft. **1:100 structure** (opened): as at 1:50 with simpler taper, round plate, masses at the limb ends. **1:500 structure** (opened): the sixteen members as prisms of their declared sections, so the stem steps at each joint like a telescope; plate and flare cone at the foot.
- **1:200 body**: three bands of masses under a cap, scaffolds visible under the crown.
- **1:500 body and thermal cut**: stem with flare, four low side masses, four shoulder masses, rounded top; hidden limbs inside the crown.
- **2D** (`r3-2d`, candidate and catalog, 1:50 and 1:200 opened; 1:100 and 1:500 rendered and validated, not opened): clean scalloped outlines per foliage mass, stem with flare, limbs emerging behind the stem, apples as small circles; dimension 6.35 m.
- Pose `gale-windfall` 1:50 sheet and pose `harvest-laden` 1:50 drawing: closed, no gaps.

## 6. Still imperfect

1. Foliage is smooth ellipsoid lobes (C8): no leaves, no twigs finer than the forks; the crown surface reads as a cluster of bubbles when seen close. The peers (oak) use the same abstraction with smaller sub-lobes and open sky between the masses; this crown is closed, which is right for a dense apple in summer but hides the limb structure from the side.
2. Scaffold azimuths 9 / 143 / 189 / 323 degrees are not the even whorl of a trained crown; the cause is the planar rig (section 2). A true fix needs out-of-plane bones in the rig schema.
3. Wood that carries no bone (fruit branches, forks, buttress roots) moves rigidly with its parent bone in poses and clips and has no member in `structure`.
4. 1:200: the masses are wide (tangential semi-axis 1.4-1.9 m against 0.9-1.1 m radial), so the elevation shows three horizontal bands; recognisable, not elegant. 1:500: a groove remains between the two tiers of masses.
5. `limb-bark` 9.1 m² is the C16 limit, the hand value is 9.4 m².
6. Cross-scale front 1:100 sits exactly on its limit (0.10875 m); any change of the 1:50 or 1:100 crown needs `r3-apple-tree-cross.ts 50 100` again.
7. In pose `harvest-laden` one apple mark of the plan lies 5 cm outside the crown outline (the foliage bones turn by different angles).
8. Not looked at: the 1:100 and 1:200 thermal sheets, the 1:200 structure sheet and the 1:100 and 1:500 drawings were rendered but not opened; the check and the validator cover their containment, coverage and extents.
9. 1:500 structure: members are prisms of the declared sections without taper (stepped at the joints).
