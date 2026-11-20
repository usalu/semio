# Round 3 — `vegetation/weeping-willow` (golden weeping willow, *Salix × sepulcralis* 'Chrysocoma')

State 2026-11-20: catalog asset spliced and valid (`r3-validate.ts vegetation/weeping-willow --catalog-only`: VALID, schema 0, validateAsset 0); `volumes-check.ts vegetation/weeping-willow --product`: **PASS** (82.9 s) at every scale.

Scripts (ticket folder): `r3-weeping-willow-model.ts` (every section, tress row, hem height, inset and outlet stated by hand per scale; rig, pose and anchor root fixes; writes `volumes/vegetation/weeping-willow.json` and the candidate), `r3-weeping-willow-kit.ts` (loft/lobe conversion and exact outlines), `r3-weeping-willow-draw.ts` (ruling E1 drawings), `r3-weeping-willow-roots.py` (writes named members into the catalog file), `r3-weeping-willow-apply.sh` (model, validate candidate, write roots, splice), `r3-weeping-willow-cross.ts` (where two scales' outlines differ), `r3-weeping-willow-ground.ts` (lowest body point in every pose and every quarter second of every clip), `r3-weeping-willow-frames.ts`.

## 0. What was there

No earlier round-3 work on this asset (no `r3-weeping-willow-*` scripts, no report). Start state: `r3-validate` VALID; source `volumes/vegetation/weeping-willow.json` equal to the catalog `volumes` (round 2, 61 / 46 / 43 / 25 body solids).

## 1. Reference

| item | real subject | source | asset before |
|---|---|---|---|
| habit | broad, rounded-pendulous crown; short stout trunk that forks low into a few steeply ascending main limbs; limbs arch outwards and droop at their ends; long pendulous yellow branchlets (whips) hang vertically in curtains from all arching branches, mostly to near the ground; under the crown an open "room" round the trunk | RHS Plant Finder (*S. × sepulcralis* var. *chrysocoma*: "weeping, with vigorous arching branches terminating in golden-yellow branchlets"); Mitchell, *Field Guide to the Trees of Britain and Northern Europe*; Johnson & More, *Collins Tree Guide* | two lollipop "cap" masses on stalks over a barrel of spindles pointed at both ends |
| height / spread | 15 m x 15 m (RHS, after 10-20 years); nursery data 15-25 m high, 15-25 m wide; spread usually equal to or larger than the height | RHS; van den Berk; Ebben | 11.1 m high, 16.2 m (x) x 14.0 m (z): a mid-aged specimen, wider than high (kept) |
| trunk | short, 1.5-3.5 m clear stem, 0.6-1.2 m diameter at maturity, base flared with shallow surface roots; bark grey-brown, deeply fissured into a coarse network of interlacing ridges | Mitchell; Collins Tree Guide; Wood Handbook (green density) | stem 0.95 / 0.85 / 0.65 m, fork at 3.5 m (kept) |
| main limbs | 2-5 co-dominant limbs, 20-40 degrees from the vertical at the fork, arching to 40-60 degrees and drooping below the horizontal at their ends | field habit, Mitchell | two limbs in the elevation plane at 19 degrees, 49 degrees, then **horizontal** at 9.85 m (outer limb rising 1 degree) |
| secondary branches | radiate in every azimuth from the main limbs, arch out to the crown edge and carry the hanging branchlets | habit | none: the whole depth of the crown (14 m) was carried by foliage only |
| branchlets | slender, 1-5 m long each, hanging in tiers so that the curtain reaches from the crown shoulder to 0.2-1.5 m above the ground with a ragged, tapering hem; curtain depth 0.5-1.5 m | habit; `provenance.notes` (curtains as pendulums 7-10 m long) | spindles 0.1-0.3 m thick, pointed at the top as at the bottom |
| leaf | lanceolate, 8-12 cm x 1-1.5 cm, finely serrate | Collins Tree Guide | strokes in the 2D |
| crown form | dome over the arching limbs, shoulder at about 0.8 of the height, sides falling almost vertically, widest at mid height, hem slightly tucked | habit; photographs of mature specimens | flat-topped barrel, top made of two separate balls |

## 2. Audit (start state)

- **Rig.** Two limb chains in the elevation plane (west, east), symmetric about the trunk axis. The outer limbs `limb-w-c` / `limb-e-c` run horizontally (world 179 / 1 degrees) at 9.85-9.90 m out to 6.9 m: a T-bar, not an arching willow limb, which droops at its end. The two outer curtains hang from that bar (`curt-w3` 9.6 m, `curt-w4` 9.7 m long). Everything else (fork at 3.5 m, limb angles 19 / 49 degrees from the vertical, sections 95 / 85 / 65 / 35 / 28 / 20 cm, pendulum curtains with their masses at mid length) agrees with the reference.
- **Body 1:50 (61 solids).** Barrel of 40 curtain spindles, each pointed at both ends, so the crown has no shoulder and the top of every curtain shows as a spike; two big balls on the limb forks and five cap lobes read as two lollipops; three frustum stacks fill the plan; trunk hidden in the barrel; no secondary branches (the crown 14 m deep has no wood carrying it); no ragged hem; plan shows a ring of separate balls round two ellipses.
- **2D, all scales.** Elevation: two lollipop crowns on stalks between barrels of strands; front: a flat-topped drum; plan: a ring of separate circles round two ovals. None is the view of a real willow nor of one body.
- **Thermal / structure.** References of the body solids, structure as cores in those spindles: they inherit the wrong form.


## 3. Root fixes (catalog asset, outside `volumes`), written by `r3-weeping-willow-roots.py` from the candidate of `r3-weeping-willow-model.ts`

| member | old | new | why |
|---|---|---|---|
| `rig` `limb-w-c` / `limb-e-c` `angleDeg` | 40 / -40 (world 179 / 1 degrees: horizontal bar at 9.85 m) | 71 / -71 (world 210 / -30 degrees: the outer limb droops 30 degrees below the horizontal, tip at 8.35 m, radius 6.5 m) | weeping habit: arching limbs droop at their ends; the horizontal bar poked 0.6 m out of any honest dome crown |
| `rig` `curt-w3` / `curt-e3` | `angleDeg` 91 / -91, `lengthM` 9.6, `mass.comX` 4.8 | 60 / -60 (still plumb), 8.85, 4.425 | pivot moved down with the drooping limb, hem kept at 0.25 m |
| `rig` `curt-w4` / `curt-e4` | 91 / -91, 9.7, 4.85 | 60 / -60, 8.1, 4.05 | same |
| `poses[*].loads` `wind-curt-?3` / `wind-curt-?4` `x` | 4.8 / 4.85 | 4.425 / 4.05 | drag still at the curtain's mid length |
| `anchors` `tp-curtain-1..10` | points of the round-2 spindles | 4 cm outside the 1:50 outlet lens of each emitter patch (same bones) | D2 |
| `representations.*.geometry` | constructed views (lollipop caps, barrel, ring of balls) | true orthographic views of each scale's body, part by part in depth order, fully hidden parts left out; strands, stem fissures drawn on visible faces | ruling E1 |
| `representations.*.decisions` | describing the old drawing | rewritten (en, de) for the new drawing | |
| `poses` `windy`, `storm` and every key of the clips `wind-sway`, `gust` (bone angles) | stem and limbs rotated up to -4.4 / -16.5 degrees in total; curtain angles +10 / +30 relative to those limbs, so their absolute swing was only 0 to 9 degrees and the lee curtains sank up to 2 m into the ground (also in the default clip `wind-sway`) | stem angles x 0.5, limb and cap angles x 0.25 (a 35 cm willow limb hardly moves; the curtains swing); every curtain angle set to the absolute swing its pose describes (10 degrees fresh breeze, 30 degrees gale) minus the rotation of its ancestors | root defect: pendulums turned down by their limbs, curtains in the ground in every windy frame |
| `poses[storm].description` | "... and the limbs approach their joint limits." / "... die Äste nähern sich ihren Gelenkgrenzen." | "... and the stiff limbs give by 1 to 2 degrees at each joint." / "... die steifen Äste geben an jedem Gelenk um 1 bis 2 Grad nach." | follows the corrected angles |

`dimensions` (11.1 x 16.212 x 14.0 m, 2950 kg) were not changed: they lie inside the reference range (spread larger than the height, a mid-aged specimen). `thermal.regions[].areaM2` were not changed: the model meets 14.5 and 22 m2 (section 5).

`r3-weeping-willow-ground.ts` after the fixes: lowest body point 0.000 m (the flare base on the ground) in every pose and every quarter second of every clip at 1:50, 1:100, 1:200 and 1:500; nothing below the ground.

## 4. What is modelled

The crown is a fountain, the real construction of a weeping willow crown: an envelope rho(y) = sqrt(1 - t^P), t = (y - 4.5) / 6.6, crown radius 8.106 m (x) by 7.0 m (z), widest at and below 4.5 m, top 11.1 m, P = 2.6 + 0.45 cos 2az (fuller shoulders over the two limbs east and west). Each tress starts at its tier's radius (apex, half radius, shoulder), follows the dome outwards along the meridian with sections that grow in width as the tresses side by side double, and hangs from the widest point vertically to a blunt, ragged tip. Hem heights and the set-in of the lobes are stated tables over the azimuth (24 and 12 values), so every scale samples the same ragged hem; widths vary by a stated factor table.

| scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 115: flare, 5 surface roots, stem in 2 lofts, 2 fork collars, 6 limb lofts, 4 secondary branches; 48 fountain tresses (24 of them split into arch on `cap-*` and hanging part on `curt-*`), 16 inner tresses, 6 fringes from the branches, apex tuft | 125: every body solid by reference in its region, flare ground face `sectionFaces: [-u]`, 10 outlet lenses (8 in hanging tresses, 2 in first-tier arches) | 57: 9 wood members (wood under 2.5 cm bark, fork collars included in limb-a), 10 pendulum rods (curtains to 1.7 m, caps along their bones), 18 joints (stem joint flattened above ground), support disc o1.52 m, 19 masses |
| 1:100 | 77: 3 surface roots, fewer sections, 32 tresses (16 split), 10 inner tresses, tuft | 87 | 38 |
| 1:200 | 52: flare without roots, 24 tresses (12 split), tuft | 62 | 28 |
| 1:500 | 26: flare, stem, limbs as 2-section tubes, 16 tresses one solid each, tuft | 36 | 20 |

Final check (`--product`, PASS): silhouette IoU elevation / front / plan 0.9995 / 0.9996 / 0.9985 (1:50), 0.999 / 0.998 / 0.999 (1:100), 0.999 / 0.999 / 0.999 (1:200), 0.999 / 0.998 / 0.998 (1:500); cross-scale p95 0.101 / 0.079 / 0.090 m (1:100, limits about 0.105), 0.128 / 0.090 / 0.105 m (1:200), 0.244 / 0.169 / 0.169 m (1:500); bounds within 0.08 m; bark areas 1:50 trunk 13.97 against 14.5 m2 (3.7 %), limbs 24.2 against 22 m2 (10 %); 1:100 8.7 % / 9.8 %; totals 1:200 0.3 %, 1:500 15.8 %; structure outside the body 0 at every scale.

## 5. What I saw in the final renders

- **1:50 body** (`r3-sheet`, six views, plus from above at turn 30 / tilt 55 and from below at turn 20 / tilt -30): front (x) a tall rounded mound, side (z) a broad dome, both with the veil falling vertically from the widest point to a scalloped hem 0.4-0.9 m above the ground, the short stem and flare showing under it; top: tresses radiate from the apex like a fountain, closed by the apex tuft; three-quarter: reads as a weeping willow at first sight. From below: an open room, stem, fork and the two limbs, inner tresses hanging into it. A faint seam runs round the crown at about 8 m where arches meet their hanging parts.
- **1:50 storm** (`--pose storm`): the veil streams 30 degrees to the lee on both sides, the arching crown stays, hems clear of the ground; the hanging parts separate from their arches by the pendulum swing (stub ends visible).
- **1:50 thermal, cut**: bark of stem, collars, flare and roots in `trunk-bark`, limbs and branches in `limb-bark` inside the foliage shell, nothing pokes out; outlet lenses flush.
- **1:50 structure**: stem and Y of limbs as tapered wood on the true limb axes (the outer limbs droop), pendulum rods hanging plumb from their pivots, joints, masses, support disc.
- **1:200 body**: the same fountain with 24 broad tresses; **1:500 body**: 16 tresses from the apex, a helmet-like mound with a scalloped hem and the stem; recognisable at a glance.
- **2D** (`r3-2d`, 1:50, 1:200, 1:500 opened; 1:100 rendered and validated): true views; elevation and front show arches overlapping the hanging tresses with strand lines, plan a radiating fountain; 1:500 plan scalloped, depth 14.0 m.

## 6. Still imperfect

1. Foliage is smooth lofts (tresses), no single whips or leaves; seen close the crown reads as bundled strands with seams rather than a translucent veil. A real willow is partly see-through; this model is opaque, so the limbs show only from below and in the cut.
2. The rig is planar (two limb chains in the elevation plane): the depth of the crown is carried by four unboned secondary branches that move rigidly with `limb-*-b`. Curtains are rigid pendulums: in wind each hanging part swings about its bone pivot and parts from its arch (visible in storm). A continuous veil needs bending (or more) curtain bones, which the rig schema and poses do not offer now.
3. The ragged hem is gentle (0.4-0.9 m, blunt tips): sharper, deeper teeth made the cross-scale p95 fail because each scale's notches fall elsewhere (the limit is the 2D pair's p95 plus one feature, and that p95 is itself capped by the comparison window). Cross-scale 1:100 elevation p95 0.101 m sits close to its limit.
4. 1:500: the 16 broad tresses leave small scallops in plan (each flat section bulges up to about 0.15 m between neighbours).
5. Surface roots are thin tapering tubes; seen from below they read as spikes.
6. Bark area 1:500 trunk 28 % under (only the total is compared there).
