# Round 3 — `vegetation/agave` (century plant, *Agave americana*)

State: DONE (2026-11-20). Catalog spliced, `volumes-check.ts --product` PASS at all four scales, `r3-validate.ts --volumes` VALID (schema 0, validateAsset 0, checkVolumes 0). What is still imperfect is listed plainly in section 7.

Scripts (all in `T`): `r3-agave-plant.ts` (the rosette stated by hand: 28 leaves, stem, spear, section laws), `r3-agave-kit.ts` (vectors, bone-local lofts), `r3-agave-rig.ts` (rig, poses, clips, loads, anchors), `r3-agave-model.ts` (volumes of every scale and the 2D of every scale and view), `r3-agave-texts.ts` (bilingual texts), `r3-agave-roots.py` (writes the root members into the catalog file), `r3-agave-ship.sh` (build, validate the candidate, write roots, splice, validate). Probes: `r3-agave-frames.ts`, `r3-agave-extent.ts`, `r3-agave-tune.ts`, `r3-agave-rigtest.ts`, `r3-agave-probe2d.ts`, `r3-agave-probe1.ts`, `r3-agave-crop.ts`. Baseline of the round-2 asset: `r3-agave-baseline.json`.

## 1. Reference

| Feature | Real plant | Source | Model |
|---|---|---|---|
| Rosette height / spread | 1-2 m / 2-3.7 m | Gentry 1982 | 1.445 m / 2.5 x 2.37 m (`dimensions`, unchanged) |
| Leaf count of a mature rosette | 20-40 open leaves | Gentry 1982; Irish & Irish 2000 | 28 plus the spear |
| Phyllotaxis | spiral, divergence about 137.5 degrees | Nobel 1988 (rosette geometry) | 137.5 degrees from azimuth 10 |
| Leaf length x width | 1-2 m x 15-25 cm, lanceolate | Gentry 1982 | 0.92-1.20 m x 14-21.5 cm |
| Leaf planform | broad clasping base, narrowed above it (neck), widest about mid-leaf, acuminate | Gentry 1982 | width law 0.56 / 0.36 (neck at 11 %) / 0.50 (55 %) / point |
| Leaf thickness | 5-10 cm at the base, 2-3 cm at mid-blade, thin towards the tip | Nobel 1988; own knowledge | 54-85 mm base, 20-26 mm mid, 35 % of that at the tip |
| Leaf section | guttered (channelled) above, rounded keel below | Gentry 1982 | V section, channel half angle 30 degrees (base) / 17 (mid) / 34 (tip), keel core |
| Margin | undulate to crenate, teeth 5-10 mm on prominences, 2-6 cm apart | Gentry 1982 | prominences of 5.5 mm every 4 cm at 1:50 (teeth merged into them) |
| Terminal spine | 3-5 cm, conical, dark brown | Gentry 1982 | 4 cm, base radius 4 mm, dark |
| Habit | lower leaves spread and recurve to the ground, inner leaves ascending to erect, central cone ("spear") of furled leaves | Irish & Irish 2000 | leaves 1-8 arch to 2-12 cm above the bed, 9-16 rise and level out, 17-23 stand at 50-65 degrees, 24-28 nearly upright round the spear |
| Stem | short thick stem (caudex) hidden by the leaf bases, to 0.3 m, about 0.22-0.27 m thick | Gentry 1982; `area-decisions.md` (cylinder o0.22 x 0.42 = 0.29 m2) | revolved stub 0.3 m high, radius 0.13 at the bed to 0.07 at the top, outer area 0.300 m2 |
| Mass | not sourced | - | `massKg` 62 unchanged; the modelled leaf volume suggests the real plant is heavier (about 3 l per leaf), no cited value, so not changed |

## 2. Audit of the round-2 asset against the reference

- Rig: 21 bones. Ten leaf "groups" (`leaf-r1..r5`, `leaf-l1..l5`), each standing for two to three leaves, all hinged at ONE point (x 0.3 on the stem axis, the stem top) and one `leaf-s`. A rosette has every leaf on its own insertion along the stem, spiralled; 28 leaves cannot move as ten. Root defect.
- `root-plate` mass centre at `comX` 0.5 (half a metre beside the plant, because the bone is 1 m long along +x). Root defect.
- Anchors: transpiration outlets at `y` 0 on the bone axes of group bones, i.e. in the air between leaves.
- 2D: 1:50 drew about 19 leaves as flat capsules/polys in the paper plane, 1:500 two filled blobs in the elevation against a star in plan (`author-requests.md` 01 and 54: the views contradicted each other, IoU 0.47 between 1:200 and 1:500).
- Solids (round 2): stem a rectangular extrusion 0.42 x 0.22 x 0.26 (`model-findings.md`), leaves flat plates, no channel, no neck, no spine; structure a copy of the leaf plates.
- `dimensions` (1.445 / 2.5 / 2.373 m): within the reference, kept. `areaM2`: leaf regions are exchange areas (kept); caudex 0.29 m2 kept, the new stem measures 0.300 m2.

## 3. Root fixes (written into the catalog asset)

| Member | Old | New |
|---|---|---|
| `rig.bones` | 21: root-plate, caudex, 10 leaf groups (8 with two bones) hinged at the stem top, `leaf-s` | 59: root-plate, caudex, `leaf-01..28-a` (insertion to knee, 0.46-0.60 m) and `-b` (knee to tip), each at its own insertion on the stem (height 0.05-0.279 m, radius 0.10-0.05 m, own `z`), `spear` (1.145 m) |
| leaf masses | 35.4 kg over 10 groups | 35.4 kg over 28 leaves by leaf volume (1.75 kg for the largest leaf), about two thirds in the proximal bone |
| `root-plate.mass` | `comX` 0.5, `comY` 0 | `comX` 0, `comY` 0.03 |
| `poses` (5) | 10-19 bone offsets | 28-57: every leaf keeps the change of azimuth and elevation of the old group nearest to it (inverse-square blend); new: a leaf tip never goes below 25 mm above the bed (fresh-breeze pressed several tips through the ground) |
| pose loads | 11 wind loads (e.g. 19.688 N each in fresh-breeze) | 29 wind loads with the same total (216.568 N in fresh-breeze, 5.412 N summer, 2.409 N night) |
| `clips` (wind-sway, gust) | keys on group bones | the same keys re-solved per leaf |
| `anchors` | 13 emitter anchors on group bones at `y` 0 | on the upper face of real leaves (1, 3, 5, 7, 10, 13 spreading; 18, 20, 22, 24, 26, 27 erect; spear) |
| `thermal.emitters` | anchors renamed bones | same ids, same physics |
| `structure.assumptions`, `summary`, `provenance.notes`, `provenance.references` | "eleven leaf groups" | 28 leaves; references Gentry 1982 and Irish & Irish 2000 added |
| `representations[*].geometry` and `.decisions` | flat shapes, views contradicting | redrawn (section 5) |

`dimensions`, `thermal.regions[].areaM2`, `materials`, `exports`, `presentation` untouched.

## 4. Volumes per scale

| Scale | body | thermal | structure |
|---|---|---|---|
| 1:50 | 199: per leaf two parts (overlap at the knee, the proximal part drawn in so no skins coincide), each part keel core + two wings rising by the channel angle with the crenate margin; 28 spines; stem revolve; spear revolve + spine | 212: every body solid by reference in its region, 13 outlet patches in the channel of real leaves and on the spear | 120: fibrous core member per leaf part along the curved centre line (a third of width and thickness), 58 joints (insertions, knees, stem, spear), stem, spear and root-plate members, support pad, two masses |
| 1:100 | 170: as 1:50 with smooth margins, spine merged into the tip, fewer sections | 183 | 120 |
| 1:200 | 58: one lens blade per leaf part (keel to margin tops), at least 30 mm, run out to the point over the last eighth | 71, outlets set into the blade top | 62: members, support, masses; joints merged |
| 1:500 | 58: one blade of 75 mm per part with four to five sections, pointed tip; plain stem, two-segment spear | 71 | 62: members of 75 mm tapering at the tips |

Kept from the other agent's pass of 04:00 (`living-models.md`: "distal blades taper to a terminal point at the two coarsest scales"): the idea, now stated in `lensSpan` of `r3-agave-model.ts` (taper over the last eighth instead of one end section), so the source and the catalog agree. Nothing else differed between catalog and source.

Final check line: `PASS` (`🗑️generated/r3-agave/check.txt` is deleted with the generated folder; numbers of the last run): silhouette IoU elevation / front / plan 0.943 / 0.943 / 0.993 (1:50), 0.929 / 0.929 / 0.993 (1:100), 0.929 / 0.932 / 0.985 (1:200), 0.925 / 0.914 / 0.980 (1:500), limits 0.90 / 0.88 / 0.88; bounds within 15 mm; caudex area deviation 3.5 % / 2.5 % / 1.5 % / 6.8 %; cross-scale IoU 1.00; monotonic strict at every step.

## 5. 2D (ruling E1)

Leaves leave the paper plane, so every leaf shape must be a `solid` (flat shapes give `lateral-ambiguous`). An `extruded` solid is a prism whose span is linear along the bone, so one prism per bone cannot follow a curved, channelled blade: the first redraw (one box per half bone, bounding spans) gave IoU 0.72-0.82 in elevation and front and kinked plank leaves. Now each leaf part is drawn by plates that step along the curve (3 / 3 / 2 / 2 per bone at 1:50 / 100 / 200 / 500) and across the hull of the section (wings, shoulders, keel at 1:50; wings and keel at 1:100; blade and rib of the lens at 1:200; one at 1:500), spans fitted by least squares; at 1:50 and 1:100 one face plate per bone lies inside the blade and covers the plate joints wherever the face of the leaf is seen. Shapes per view: 730 / 394 / 226 / 114. The 1:50 groove stroke stays; at 1:100 it is merged.

## 6. What I saw in the final renders

- 1:50 body sheet (`r3-sheet`, six views): reads at once as an agave: star of 28 channelled leaves, lowest arching to the bed, inner ones upright round the spear; crenate margins and dark spines visible; plan shows neck, widest middle, acuminate tips. Front close-up in fresh-breeze: closed at every knee, no tip below ground. Grade A.
- 1:50 thermal: spreading leaves red, erect leaves and spear green, stem violet under the bases; outlets are small flush patches. Structure: blue cores along every leaf into the stem, pad and plate at the bed. A.
- 1:100 body: same plant, smooth margins. 1:200: lens blades with necks and points, single clean seam at each knee (the zigzag of coincident skins seen before is gone). 1:500: 28 pointed blades, still a star and not a mass; members taper inside the blades; outlets flush. A at each scale for what the scale can show.
- 2D (`r3-2d`) 1:50 and 1:100: line views are clean curved leaves with keel lines; the solid plan shows one knee line per leaf and a fine rim line 3 mm inside the margin (the face plate). 1:500 at true size: a small star in plan and a tuft in elevation, ink-heavy in the line elevation because 28 blades meet in 3 mm of paper. 1:200 2D: judged from the overlay sheet only (thin red rims), the `r3-2d` panel was rendered but not opened.

## 7. Still imperfect (plain)

- Solid views (`light-solid`, `dark-solid`) at 1:50 and 1:100: leaves seen edge-on show short ticks at the plate joints, because the product strokes every fill. Not solvable in the asset; filed in `tool-requests.md` (2026-11-20, two entries: fill strokes; `unionPolygons` throwing on a sweep-w shape seen edge-on, which ruled out the keel fin that would have hidden the ticks).
- The stem foot in the 1:50 and 1:100 line elevations has a small stepped outline where the lowest plate ends meet the stem profile.
- 1:500 leaves are 75 mm thick at mid-blade (real about 50 mm) because of the feature floor; members there are as thick as the blade.
- `massKg` 62 looks low against the modelled leaf volume; no cited value, so unchanged.
