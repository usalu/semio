FINAL (2026-11-27 05:56: the 14 animals have 0 geometry findings; `dairy-cow`, `parent-carrying-toddler` and `tower-crane` written, every pose GLB below 8 MB (largest 7.74, 7.62, 7.78 MB), every proof clean, `🧊️model-catalog` green for each as the only selected asset; no other listed asset is over with the final writer; nothing weakened; nothing open in the lane)

# 🐄️ Closing Lane CL-T3: Animal Scan and Pose GLBs Above 8 MB

Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` = this ticket folder, `W` = `U/🧰️matrix/CL-T3`, `M` = `U/🧰️matrix/MR`, `T14` = the 3D ticket `…/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE`, `GEN` = `U/🗑️generated/matrix/CL-T3` (deleted with this report; every number below was read there in a log of a run that ended by itself). Every measuring run went through `repo capacity run` (`W/q.sh`), at most two requests at a time (once three for about two minutes, section 7). No test, budget, schema limit, rule, engine, oracle or manifest was edited. Only `asset.volumes` of three catalog files changed.

## 1. Result

| Task | Before | Now |
| --- | --- | --- |
| Geometry scan of the 14 animals (`auditGeometry`, the code of `🧊️model-catalog`, `W/geo.ts`, 03:17) | not scanned | 0 findings for each of the 14: no self-intersection, no open closed solid, no mode whose triangles grow towards a coarser scale, no scene above 400,000 triangles. Nothing to cure. |
| `dairy-cow`, largest pose GLB | 8.10 MB (`grazing-head-down`, 8,493,104 bytes) | 7.74 MB (8,117,380 bytes), 0.26 MB below the budget |
| `parent-carrying-toddler`, largest pose GLB | 9.38 MB (`hoisting`, 9,840,312 bytes) | 7.62 MB (`heel-strike-l`, 7,990,744 bytes), 0.38 MB below |
| `tower-crane`, largest pose GLB | 8.55 MB (`max-reach`, 8,969,080 bytes) | 7.78 MB (8,161,264 bytes), 0.22 MB below |
| Every other non-plant asset with a `GLB of N MB` line (section 2) | 8.1 to 12.0 MB with the old writer | below 8 MB with the final writer of CX-GLB, not touched |

The budget is 8 MB = 8,388,608 bytes (`BUDGET.glbBytes` of the oracle), unchanged.

| Asset | Catalog file | Scene triangles 1:50 / 1:100 / 1:200 / 1:500 (body, thermal, structure; `a→b` changed) | Pose GLBs in MB | Matrix of the written file |
| --- | --- | --- | --- | --- |
| `dairy-cow` | `efd4161b451a39e0` (3,568,766 bytes) → `5c74cd98cdbf7305` (3,352,275 bytes), 195 changes, written 04:21 | body 102240→94556 / 41108→32504 / 21136→13556 / 9446→5308; thermal 174686→166044 / 86562→77192 / 45010→35940 / 20638→13980; structure 156430→155832 / 70814→70330 / 42320→40960 / 24816→24374 | standing-chewing 7.65→7.25, grazing-head-down 8.10→7.74, lying-resting 7.15→6.75, kneeling-front 7.53→7.09 | 147 ok, 105 group drift, 0 fail (the cells of the full matrix before: 147 ok, 105 group drift) |
| `parent-carrying-toddler` | `a9d6445426694a01` (4,415,987 bytes) → `31fbd1704b506dde` (3,210,192 bytes), 431 changes, written 05:09 | body 120646→75570 / 57134→28022 / 31746→12654 / 15994→5758; thermal 220450→129412 / 112738→56088 / 47520→22332 / 24114→10250; structure 195568→184942 / 106724→102996 / 46980→43944 / 28580→25248 | standing-carry 9.18→7.37, heel-strike-l 9.38→7.62, hoisting 9.38→7.57 | 77 ok, 175 group drift, 0 fail (before: 77 ok, 175 group drift) |
| `tower-crane` | `8904c22c03ec0d2a` (2,581,547 bytes) → `f42b9e767fb3b7f9` (2,582,061 bytes), 22 changes, written 05:20 | body 118528→90724 / 59212→48172 / 36676 / 17988; thermal 120024→92220 / 60464→49424 / 37864 / 18640; structure 108610→102562 / 70850→67874 / 51300 / 28364 | parked-weathervane 7.83→7.06, lifting-load 8.50→7.73, max-reach 8.55→7.78, skip-at-ground 8.52→7.75 | 210 ok, 42 range end, 0 fail (before: 210 ok, 42 range end) |

## 2. Sizes with the final writer (`W/glb.ts`, `writeGlbDetailed` per named pose, 03:38 to 03:51, catalog files as they stood then)

The list is every non-plant asset with a `budget pose: GLB of N MB` line in `U/modelcat-issues.txt` or in a shard log `🗑️generated/closing/model-catalog-NN.log` (31 assets with such a line, 16 of them vegetation, lane CL-T1), plus the five human assets CL-T1 rewrites.

| Asset | Poses | MB per pose | Largest |
| --- | --- | --- | --- |
| `tower-crane` | 4 | parked-weathervane 7.83, lifting-load 8.50, max-reach 8.55, skip-at-ground 8.52 | 8.55 OVER |
| `parent-carrying-toddler` | 3 | standing-carry 9.18, heel-strike-l 9.38, hoisting 9.38 | 9.38 OVER |
| `dairy-cow` (03:21) | 4 | standing-chewing 7.65, grazing-head-down 8.10, lying-resting 7.15, kneeling-front 7.53 | 8.10 OVER |
| `city-car` | 5 | parked 7.41, driving 7.89, hatch-open 7.39, bonnet-open 7.44, braking-nose-down 7.94 | 7.94 |
| `basketball-player` | 4 | dribbling 7.62, jump-shot-set 7.59, jump-shot-release 7.52, standing-with-ball 6.84 | 7.62 |
| `worker-carrying-box` | 5 | squat-grip 6.54, lift-mid 6.69, carrying-box-low 6.39, carrying-box-chest 7.07, setting-down 6.72 | 7.07 |
| `person-pushing-bicycle` | 3 | pushing-bicycle 6.83, standing-with-bicycle 6.92, lifting-onto-stand 7.06 | 7.06 |
| `tennis-player` | 5 | ready-stance 6.97, forehand-backswing 7.03, forehand-contact 6.98, follow-through 7.00, standing-between-points 6.30 | 7.03 |
| `person-with-rollator` | 4 | stand-at-grips 6.85, walking-midstance 6.75, seated-rest 6.65, stoop-to-basket 6.62 | 6.85 |
| `standing-desk-worker` | 3 | typing 6.60, touchpad-scrolling 6.70, reading-arms-crossed 6.51 | 6.70 |
| `person-sweeping` | 5 | push-start 6.32, pushing 6.61, push-end 6.61, broom-lifted 6.35, resting-on-broom 5.97 | 6.61 |
| `tram` | 4 | running 5.70, at-stop-doors-open 6.54, braking-stop 6.39, depot-pantograph-down 4.52 | 6.54 |
| `waiter-with-tray` | 4 | walking-heel-strike 6.43, walking-midstance 6.38, standing-with-tray 6.02, serving-gesture 6.13 | 6.43 |
| `person-in-winter-coat` | 5 | hands-in-pockets 5.70, arms-at-sides 5.75, stamping-feet 6.17, contact-left 6.28, passing-right 6.20 | 6.28 |
| For CL-T1: `parent-with-stroller` | 4 | pushing-stroller 6.58, pausing-with-stroller 6.42, checking-child 6.60, kerb-tilt 6.56 | 6.60 |
| For CL-T1: `runner-with-blade-prosthesis` | 6 | 5.03, 4.99, 5.19, 5.14, 5.15, 5.17 | 5.19 |
| For CL-T1: `running-child` | 7 | 4.32, 4.67, 4.70, 4.68, 4.78, 4.69, 4.64 | 4.78 |
| For CL-T1: `walking-toddler` | 4 | 3.92, 3.94, 3.84, 3.80 | 3.94 |
| For CL-T1: `person-with-umbrella` | 3 | 5.14, 5.15, 5.08 | 5.15 |

The five lines for CL-T1 stand in `requests.md` under `## closing CL-T3 — for CL-T1: pose GLB sizes …` (03:52). `city-car` passes with 0.06 MB to spare (`braking-nose-down`, 7.94 MB); it is not over and was not touched.

## 3. Where the bytes sit (`W/glb.ts --scenes`, `W/geo.ts --solids`, `W/meshes.ts`)

- The writer shares every byte-identical accessor, so the body scene costs once for body, thermal and structure. Scene triangles are therefore no measure of bytes: the mass markers of the crane (five equal ellipsoids of 3968 triangles) cost one mesh, its rounded boxes shared too (59,328 scene triangles removed by the first crane trial saved 0.17 MB), while the 163 rungs and brackets of its ladder (15,648 triangles) each held vertices of their own.
- `dairy-cow` `grazing-head-down` before: JSON 1.54 MB, positions 3.73 MB (float32), triangle indices 2.27 MB, normals 0.55 MB; unique to 1:50 structure 1.34 MB, to 1:50 thermal 1.08 MB (0.65 MB in `lying-resting`: the breath and plume glyphs of the pose), shared 2.33 MB. 1:50 body: 49 lofts 65,616 triangles, 25 ellipsoids 34,128, 4 capsules 2,496.
- `parent-carrying-toddler` before: 1:50 thermal 108 lofts with 165,052 triangles (the nested shells of air, sweater and sock), body 54 lofts with 83,404; 882 loft stations in the 1:50 body list (31 on one thigh).
- `tower-crane` `max-reach` (first trial): the mesh of the mast at 1:50 held 1.30 MB, at 1:100 (no ladder) 0.42 MB; JSON 1.83 MB, of it 0.77 MB the `extras` of the primitives. From the parked pose to a lifting pose the thermal scenes gain 34,000 triangles of flow glyphs and 0.67 MB (shared code, not the asset).

## 4. What was changed and why (`W/fix.ts <id>`, the repairs in `W/repairs.ts`, idempotent)

### `dairy-cow`, `parent-carrying-toddler`: loft stations the drawing cannot show (`thin`)

- Rule: from every loft, one at a time, the inner station whose removal moves the skin least is dropped, as long as every ring point of the authored loft (64 per station, the kernel's own frames, `loftFrames`) stays within a quarter of the smallest feature of the scale from the thinned skin: 1.9, 3.8, 7.5 and 18.8 mm at 1:50, 1:100, 1:200 and 1:500, which is 0.04 mm of paper. The first, the last and the widest stations (largest `rx`, largest `ry`, largest smaller radius) always stay, so no loft changes its length or the thickness that `volumes-feature` reads.
- A loft is bound to one bone or to one constant blend of bones (the trunk of the cow: pelvis 0.25, lumbar 0.5, thorax 0.25), which moves it as a whole, so the measure of the rest pose holds in every pose.
- `dairy-cow`: 573 of 2,637 stations dropped (body 93 / 120 / 124 / 107, thermal 2 / 5 / 12 / 45, structure 8 / 10 / 31 / 16 at 1:50 / 1:100 / 1:200 / 1:500). They are mostly the three-step roundings of the ends of the limb masses, which were carried from 1:50 into every coarser scale at the same millimetres (stations 2 to 3 mm apart on the claws at 1:50, 8, 10 and 12 mm apart on the trunk at 1:100), and the stations of the coat patches beyond what the curve of the trunk needs (`patch-saddle` 32 → 15 at 1:200, `th-patch-saddle` 31 → 10 at 1:500). The large end roundings of 1:50 and 1:100 stay: without their middle station the skin would move by 15 mm.
- `parent-carrying-toddler`: 3,137 of 5,569 stations dropped (body 471 / 430 / 361 / 297, thermal 527 / 446 / 134 / 121, structure 179 / 82 / 87 / 2), for instance `jeans-thigh-l` 31 → 14 and `coat-skirt-l` 16 → 2 (a straight cone) at 1:50.
- Two findings of the light proof shaped the rule, each on a trial copy that was not written:
  - cow, 1:500: `tail-3` lost the one station of its bulge and was 60.0 mm thin against 70.2 mm (`volumes-feature`). Hence the widest stations stay.
  - parent, 1:500: with the head thinned, the toddler's mouth outlet (an emitter of 4.5 mm) no longer sat on the skin, and `volumes-feature` admits an emitter only there. Hence in this asset the skin under every breath and plume outlet (the thermal solids of the bone and region of an emitter and the body solids they refer to) keeps every station. The outlets of the cow stay on its thinned skin (`validateAsset` 0 issues), so the cow has no such exception.
- The first decision of every scale records the rule in English and German ("The lofts keep only the stations the drawing can show: …"); a scale that records it is not thinned again, because a thinned loft no longer holds the authored skin to measure against (second run of `fix.ts --write`: 0 changes).
- Silhouette guard before → now (IoU and largest distance of elevation, front, plan; the boxes are unchanged to the tenth of a millimetre):
  - cow 1:50 0.9978/10.1, 0.9956/7.5, 0.9982/8.0 → 0.9976/10.1, 0.9954/7.5, 0.9982/8.0; 1:100 0.9962/3.8, 0.9949/7.5, 0.9981/5.3 → 0.9955/5.3, 0.9948/7.5, 0.9977/5.3; 1:200 0.9917/7.5, 0.9948/8.1, 0.9957/7.5 → 0.9912/7.5, 0.9946/8.1, 0.9946/7.5; 1:500 0.9801/19.9, 0.9803/16.2, 0.9882/10.0 → 0.9779/19.9, 0.9698/12.8, 0.9862/14.1 mm.
  - parent 1:50 0.9944/10.1, 0.9934/8.0, 0.9812/13.1 → 0.9945/10.9, 0.9934/8.0, 0.9806/13.1; 1:100 0.9899/10.6, 0.9907/8.4, 0.9750/18.8 → 0.9905/11.3, 0.9892/8.4, 0.9739/18.8; 1:200 0.9811/20.6, 0.9822/27.5, 0.9720/18.2 → 0.9758/20.6, 0.9796/27.5, 0.9680/18.2; 1:500 0.9616/82.8, 0.9689/55.2, 0.9654/25.3 → 0.9496/82.8, 0.9595/55.2, 0.9604/23.6 mm. The box of 1:200 and 1:500 is 0.9 mm narrower (0.6574 → 0.6565 m).
- `T14/volumes/animals/dairy-cow.json` equalled `asset.volumes` of the catalog before the write and was written with the new volumes (`W/sync-source.py`), so the chain of that ticket reproduces the repair. The sources of the parent and of the crane already differed from the catalog before this lane and were left alone.

### `tower-crane`: roundness the drawing cannot show (`sharpen`, `square`)

- The lattice bars are two-point revolves at the feature size of their scale (16 mm at 1:100, 30 mm at 1:200, 75 mm at 1:500) and structure members whose section is pinned by `volumes-rules` (area, extreme fibre, second moment against the member): nothing to thin, and they stay round.
- `sharpen`: the corner rounding of a box goes where the arc leaves the sharp corner by less than one smallest feature, `(√2 − 1) r` below the feature: radius below 18 mm at 1:50 (the 10 mm of `cross-base-x`, `cross-base-z`, `mast-foot` and the four counterweight slabs), below 36 mm at 1:100 (the 20 mm of the four ballast blocks, 10 mm of `mast-foot`, 30 mm of `slew-platform`). The 20 and 30 mm roundings of 1:50 stay. Effect on the largest pose: 8.55 → 8.39 MB.
- `square`: the 125 rungs and 38 brackets of the ladder texture of 1:50 (`mast-ladder-rung`, `mast-ladder-bracket-r`, `-l`, `texture: true`, round bars of 25 mm along a coordinate axis) are square bars of 25 mm with the same length, place and copies; a circle of that radius stays within 5.2 mm of its square, the feature is 7.5 mm. The rails of 40 mm stay round (8.3 mm). Effect: 8.39 → 7.78 MB.
- The first decisions of 1:50 and 1:100 say both in English and German.
- Silhouette guard before → now: 1:50 0.9996/12.0, 0.9997/12.0, 0.9998/13.1 unchanged; 1:100 0.9995/13.5, 0.9996/15.9, 0.9998/22.5 → 0.9995/10.6, 0.9996/15.9, 0.9998/22.5 mm; 1:200 and 1:500 are untouched scales and read the same numbers; the boxes are unchanged.
- Tried and dropped: joining the lattice bars of a constant pitch into repeated solids (the ties and diagonals of the mast, the diagonals of jib and counter-jib; 511 → 292 body solids at 1:50). The geometry findings stayed 0 and the JSON chunk fell by 0.2 MB, but the pose GLBs grew from 8.39 to 10.09 MB, because separate equal bars share their accessors across scales and modes and the copies of a repeated solid are written out. Nothing of it was written.

## 5. Proofs of the written files

| Proof | `dairy-cow` | `parent-carrying-toddler` | `tower-crane` |
| --- | --- | --- | --- |
| Light proof on the trial copy BEFORE the write (`W/light.sh`: gate over every subject and over the poses, guard, `validateAsset`, sweep, check CLI `--file`, geometry findings 0, every pose GLB at least 0.2 MB below 8 MB) | PASS 04:13 (FAIL 03:41: `tail-3`, not written) | PASS 05:07 (FAIL 04:46: mouth outlet; FAIL 05:01: 0.12 MB margin with every head loft kept; neither written) | PASS 05:18 |
| `fix.ts --write` twice | second run 0 changes | 0 changes | 0 changes |
| Catalog file against the trial copy | byte for byte | byte for byte | byte for byte |
| Only `asset.volumes` differs from the file before | yes | yes | yes |
| Gate `M/gate.ts --soft`, authors' scan `T/🔎️scan.ts` | exit 0, 0 hard findings | exit 0, 0 hard findings | exit 0, 0 hard findings |
| Prints (`../MC/prints.ts`) | every authored scale prints | every authored scale prints | 2 refusals at 1:75 (gap `block-body`/`block-sheave` and `block-plate-l`/`block-sheave` 0.052 mm against 0.15 mm): 2D shapes, which this lane did not touch |
| Guard, `validateAsset`, three-views sweep (`M/measure.ts`) | OK, 0 issues, 0 findings | OK, 0 issues, 0 findings | OK, 0 issues, 0 findings |
| Check CLI `--asset` | `✓` | `✓` | `✓` |
| Geometry findings (`W/geo.ts`) | 0 | 0 | 0 |
| Pose GLBs (`W/glb.ts`) | 7.25, 7.74, 6.75, 7.09 MB | 7.37, 7.62, 7.57 MB | 7.06, 7.73, 7.78, 7.75 MB |
| Matrix `T/📊️matrix.ts --jobs 1 --fresh --assets <id>`, to its end | 252 cells, 0 fail (2166 s) | 252 cells, 0 fail (222 s) | 252 cells, 0 fail (152 s) |
| `🧊️catalog-silhouette`, `🧊️three-views`, `🌡️catalog-physics`, `✅️asset-validation` with the name filter | 1 + 2 + 3 + 10 passed | 1 + 2 + 3 + 10 passed | 1 + 2 + 2 + 10 passed |
| `🧊️model-frame`, the case that pins the cow | 1 passed | — | — |
| `🧊️model-catalog` with the asset as the only selected one (the command of the brief) | 3 passed of 3, 0 findings (05:32) | 3 passed of 3, 0 findings (05:35) | 3 passed of 3, 0 findings (05:54) |

The catalog files had the digests of section 1 when the last suite ended (compared at 05:54), and the volume source of the cow equalled the catalog.

## 6. CX-Q

`U/closing-CX-Q.md` did not exist when this report was written (05:56). The quality rule `S/🔨️modules/🧊️volume/⏱️job/🟦️.ts` carries CX-Q's change since 02:53 (the chord of 1/3200 of the largest dimension grows with the denominator over 50), before the first measurement of this lane (03:17); every number here was measured with it. No asset workaround for growing triangles was added: the scan found no growth in the 14 animals, and the three written assets have no growth either (geometry findings 0).

## 7. Stated plainly

- Thinning is a visible change only below the stated distance, but it is a change: shaded views have fewer, longer facets along the lofts (a thigh of the parent has 14 rings where it had 31). I rendered none of them and opened no sheet.
- The two thresholds differ: a quarter of the smallest feature for a loft skin, one whole feature for the cut of a rounded corner and for the roundness of a 25 mm bar. A quarter-feature rule removes no rounding of the crane at all (it would ask for radii below 4.5 mm at 1:50), and without the crane cures the asset stays above the budget. The one-feature rule changes how the ladder and the box corners look in a shaded 3D view at 1:50 and 1:100 (square rungs, sharp block edges); in the drawings the guard, the gate and the matrix read the same cells as before.
- The margins are 0.26 MB (cow), 0.38 MB (parent) and 0.22 MB (crane), 3 to 5 % of the budget. The sizes of a pose depend on its thermal and structure glyphs, which shared code makes; a change there moves these numbers.
- Decisions of the cow and of the parent that count stations ("the same neck with five stations", "four to five stations per segment", "the neck is a loft of three stations") counted the main stations without the end roundings and were not literally true before (the 1:100 neck had 11); they are not corrected, and the neck now has 9, 6 and 5 stations at 1:100, 1:200 and 1:500. The appended sentence states the rule that set the counts.
- The structure lists were thinned too; `spine-thoracic` went from 5 stations to 2 with a skin moved by 0.00 mm (collinear stations of one radius). If those stations marked vertebra positions for a reader of the document, that reading is gone; no validator and no test reads them.
- The guard of the cow at 1:500 fell in the front view from 0.9803 to 0.9698 and that of the parent at 1:500 in the elevation from 0.9616 to 0.9496; both pass their gates ("silhouette: OK").
- `city-car` (7.94 MB) and `basketball-player` (7.62 MB) pass and were not touched; the car has 0.06 MB to spare.
- The group drift of the cow (105 cells) and of the parent (175 cells) and the range end of the crane (42 cells) are the cells of the full matrix before this lane; they are not fail cells and were not worked on.
- The four test files that name the crane (`🌐️thermal-multiview`, `🏗️structure-projection`, `🧊️chain-state`, `🧊️axial-twist`) read its rig and drawings, not `volumes`; they were not run. The whole suites without a filter were not run.
- The before state of the scan is the count of `W/geo.ts`, not a red run of the suite.
- Queue: three requests of this lane were queued together for about two minutes at 04:41 (two waiting, one running); the third was withdrawn by its pid before it was admitted. Two runs held more than they reserved: the size measurement of 18 assets 1317 MB against 900 MB, the suites of cow and parent 3397 MB against 3000 MB. Waiting (the longest `waiting Ns` line of each request): the first scan 24 minutes, the first measurement of the cow 28, the chain that wrote the cow 26, the measurement of crane and parent 22, the suites of the crane 18, the others 1 to 12 minutes.

## 8. Requests and notes in `requests.md`

- `CL-T3 rewrote dairy-cow at 2026-11-27 04:21`, `… parent-carrying-toddler at 2026-11-27 05:09`, `… tower-crane at 2026-11-27 05:20`.
- `## closing CL-T3 — for CL-T1: pose GLB sizes of its human assets with the final writer of CX-GLB` (03:52).

## 9. Files

Updated in the catalog (repo-relative): `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals/dairy-cow/🔣️.json`, `…/🗂️catalog/👤️humans/parent-carrying-toddler/🔣️.json`, `…/🗂️catalog/🏗️machinery/tower-crane/🔣️.json`.

Updated: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/volumes/animals/dairy-cow.json` (the volume source, equal to the catalog again), `requests.md` (three lines and one section appended).

Created in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`: `closing-CL-T3.md`; in `🧰️matrix/CL-T3/`: `env.sh`, `q.sh`, `fix.ts` and `repairs.ts` (the repair), `thin.ts` (what the thinning would drop), `geo.ts`, `geo.sh` (geometry findings, triangles per solid), `glb.ts`, `glb.sh` (pose GLB sizes, bytes per scene), `meshes.ts`, `meshes.sh` (bytes per mesh), `where.sh`, `try.sh`, `trial.sh`, `light.sh`, `lights.sh`, `prove.sh`, `chain.sh`, `before.sh`, `suites.sh`, `final.sh`, `mine.py`, `tell.py`, `sync-source.py`, `animals.txt` (the 14 scanned assets), `glb-mine.txt`, `glb-cl-t1.txt` (the 18 measured assets).

Removed: `🗑️generated/matrix/CL-T3` (tool output, trial copies, before copies, logs) after this report was written.
