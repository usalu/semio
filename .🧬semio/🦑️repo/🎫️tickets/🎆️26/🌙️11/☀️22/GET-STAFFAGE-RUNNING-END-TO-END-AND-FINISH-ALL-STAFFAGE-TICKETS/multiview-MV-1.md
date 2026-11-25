FINAL (2026-11-25 11:30: 14 of 14 plants clean with all proofs, 0 failing matrix cells; open points without a failing cell in section 5)

# 🌳️ Multiview Fixer Report: Batch MV-1 (14 Plants)

Fixer report of 2026-11-24 and 2026-11-25 (first fixer until 06:25, takeover from 09:00; one coherent state, every asset measured again by the takeover, section 6) for `🧰️matrix/batches/MV-1.tsv` (brief `📋️multiview-brief.md`, causes B, D and G of `multiview-diagnosis.md` 5.5 and 5.6). Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` = this ticket folder, `W` = `U/🧰️matrix/MV-1`, `M` = `U/🧰️matrix/MR`. Every number was measured in this session. No rule, threshold, test, tool, fixture, engine, schema, README or manifest file was edited; the scripts of `M`, `🧰️silhouette/R4-P` and the other fixers were read and imported, not changed.

## 1. Result

| Plant | Scan before (hard) | Gate before (hard, every subject) | Gate after | Scan after | Matrix after | Other proofs |
|---|---|---|---|---|---|---|
| `apple-tree` | 11 | 359 | 0 | 0 | 252 cells, 84 ok, **0 fail** (794 s) | all green |
| `bamboo-clump` | 105 | 1864 | 0 | 0 | 252 cells, 0 ok, **0 fail** (1927 s) | all green |
| `black-alder` | 26 | 312 | 0 | 0 | 252 cells, 147 ok, **0 fail** (1063 s) | all green |
| `black-locust` | 4 | 56 | 0 | 0 | 252 cells, 0 ok, **0 fail** (1065 s) | all green |
| `common-ash` | 182 | 3796 | 0 | 0 | 252 cells, 224 ok, **0 fail** (865 s) | all green |
| `common-lilac` | 8 | 236 | 0 | 0 | 252 cells, 252 ok, **0 fail** (273 s) | all green |
| `common-reed` | 335 | 7425 | 0 | 0 | 252 cells, 35 ok, **0 fail** (259 s) | all green |
| `crab-apple` | 10 | 279 | 0 | 0 | 252 cells, 105 ok, **0 fail** (259 s) | all green |
| `dawn-redwood` | 16 | 396 | 0 | 0 | 252 cells, 252 ok, **0 fail** (1943 s) | all green |
| `english-oak` | 90 | 2138 | 0 | 0 | 252 cells, 182 ok, **0 fail** (1655 s) | all green |
| `european-beech` | 191 | 3939 | 0 | 0 | 252 cells, 252 ok, **0 fail** (2127 s) | all green |
| `field-maple` | 18 | 299 | 0 | 0 | 252 cells, 252 ok, **0 fail** (530 s) | all green |
| `fig-tree` | 8 | 247 | 0 | 0 | 252 cells, 252 ok, **0 fail** (626 s) | all green |
| `hawthorn` | 46 | 830 | 0 | 0 | 252 cells, 84 ok, **0 fail** (385 s) | all green |

Sum before: 1050 hard findings in the scan, 22176 in the gate over every subject.

"Gate" is `M/gate.ts`: the matrix's own `geometryFindings` over every pose, clip instant and transition at every authored scale (392 to 530 subjects per plant). "Scan" is the official `T/🔎️scan.ts` (poses). The suite already holds the two changes of the second worker (descendant bones in the anchor test, hard tier only for twins of one id); the numbers "before" are measured with them, so nothing of those two kinds was repaired in an asset.

## 2. What was wrong and how it is repaired

Three causes, one tool each, chained by `W/make.sh <id>` (idempotent: a second run was compared byte by byte with the first for the plants it was run twice on, see section 3).

### 2.1 Flat front and plan outlines on bones the wind turns (cause D): `carry.ts`

A foliage mass is one elevation outline, one front outline and one plan outline on its bone. The elevation outline turns with the bone as a whole; the front and the plan outline are one sheet at one depth and foreshorten (hawthorn `fol-7` 1:200 in `gale`: elevation y 3.868 … 6.127 m, front 4.541 … 5.893 m; whole tree 443 mm lower in the front than in the side elevation).

Tried and not used: one prism per mass (`front-sweep` of `M/fit.ts`) cannot follow a round mass (a box against an ellipse misses by about 0.2 radius at 15 degrees) and needs an upright bone; the plates of `🧰️silhouette/R4-P/redraw.ts` were rejected on rowan for the contour lines they draw in the default pose (`multiview-MS.md`).

What `carry.ts` does instead keeps the drawing of the default pose as it is:

- It measures, per scale, view and bone, the largest distance between the extent of the bone in the view and in the elevation over every subject, and works only on bones beyond 0.75 tolerance whose views agree in the default pose to half the tolerance (a bone whose views draw different parts is left and named).
- The elevation outline of the bone is cut at stations along the bone axis nearest to the viewing direction (3, 4, 6, 8, 11 or 14: the first number that brings the bone below 0.75 tolerance). At each station a plate stands across that axis: an `extruded` solid 0.004 mm thick whose profile is the drawn outline of the bone in that view, scaled down about its middle to the band the elevation chord covers and cut by the outline itself. Every plate lies inside the drawn outline and below it in the paint order (`z` below the lowest shape of the bone, same tone and region), so the default pose draws what it drew before.
- The drawn outline stays the sheet on top. A sheet that shares its id with an elevation shape is carried by the engine at one depth and overshoots on the lee side in a gale (hawthorn `fol-5` plan: 250 mm beyond the elevation); it takes the suffix `-mvs`, so the engine carries each vertex at the depth of the elevation silhouette there.
- In a turned pose the bone carries each plate at its own depth; the plates show at the windward and lee edge what the sheet alone loses.
- Plates that do not bring a part nearer (slender limbs) are not kept.

A bone with plates draws several shapes, so the matrix compares it in the reported tier; the per-plant sections therefore state the measured drift of the bones with plates before and after, and the whole-figure extents (`sync-forward`, `sync-height`, `sync-lateral`), which stay in the hard tier, are clean.

Limits, stated plainly:

- The plates are the solid that side and front (or plan) outline have in common, not the foliage body, so in a turned pose a plate can show beyond the body by about its depth times the sine of the angle. Measured where a default pose is itself turned: `apple-tree` (default pose 0.56 degrees at the foliage), front boundary distance of the silhouette guard 13.1 mm before, 22.5 mm at 1:50 and 64.1 mm at 1:100 with plates (limits 15 and 30 mm): the guard failed. The six plants that had no synchrony finding at all (`apple-tree`, `crab-apple`, `fig-tree`, of which the default pose is turned, and `common-ash`, `dawn-redwood`, `european-beech`) therefore carry no plates (`-` in `plants.tsv`); their drawings are untouched, only anchors moved.
- In a gale the edge of a mass shows the lobes of its plates beside the outline (sheets, section 4).
- Foliage as fine as the bamboo's cannot be cut by the outline of all its leaves at once (the polygon clipper overflows its stack); there only the four foliage bones that set the extents carry plates, at 1:100, 1:200 and 1:500.

### 2.2 Shapes on another bone than the part (causes B and G): `rebind.ts`, `fix-common-lilac.ts`

- `black-locust` (`rebind.ts`, then plates on one bone only): the side elevation drew every lateral limb and foliage mass on the stem bones (`trunk`, `ld1` … `ld3`), front and plan on the bones of the limbs; in `storm` the limbs turn up to 9 degrees, the stem 1 to 2, so elevation and plan were 623 mm apart. A flat elevation shape on a limb that stands askew to the paper would be `lateral-ambiguous` for the validator; `rebind.ts` therefore binds each of the 140 shapes to the bone of its front twin as a plate: an `extruded` solid 0.004 mm thick, swept along the lateral axis of the bone at the lateral place the front twin draws the part, with a profile solved so that the plate is seen exactly as the outline in the rest pose. 14 shapes on the limb `l9`, whose plane stands nearly edge-on to the paper, stay on the stem and are named by the tool. Once the twins share a bone the matrix compares them in the hard tier: the foliage mass `fol-l1` is 420 mm (1:200) and 422 mm (1:500) higher in the elevation than in the front in `storm` (tolerance 396 mm); at 1:50 and 1:100 the bone draws several shapes and has no hard finding. The first fixer gave `carry.ts` its default flags: 36 plates on three masses at all four scales, and the plates of `fol-l1` moved the front boundary of the silhouette guard at 1:50 from 13.1 to 53.7 mm and at 1:100 from 26.3 to 48.9 mm (limit 30 mm): the real `🧊️catalog-silhouette` test failed. The takeover keeps plates only where a hard finding needs them: `--limit 1.0 --scales 200,500` (the three plates of `fol-l1` at 1:200 and at 1:500, where the guard allows 60 and 150 mm).
- `english-oak` draws its crown on the stem bones in all three views; 194 single flat shapes (patches of the silhouette round and limb pieces) sat on askew limb bones, each in one view only, and moved alone (elevation top 421 mm above the front in `storm`, front 419 mm wider than the plan in `wet-snow`). `rebind.ts --stem` binds each to the nearest ancestor bone in the drawing plane, at the same place in the rest pose. Two of those patches (`sv1-200-p-202` and `sv1-200-p-203`, plan 1:200) lie 37.5 mm apart: 0.0075 mm of white between two 0.18 mm pens. On their askew limb bone the default pose foreshortened the pair until the pens met; on the stem bone the gap stayed and 1:200 stopped printing (`unavailable-scale-unsuitable` in three views, 21 failing matrix cells, check CLI `✗`: the state the first fixer left). `fix-english-oak.ts` (takeover) moves the nearest vertex of `sv1-200-p-202` 9 mm into `sv1-200-p-203`: the two patches of one leaf mass are joined in every pose, min gap of 1:200 is 0.993 mm.
- `common-lilac` 1:500: the stem cluster is a cone (radius 0.24 m at the ground, 0.60 m at 1.45 m); the plan drew the circle of its foot, elevation and front the cone: 360 mm apart in every pose. `fix-common-lilac.ts` draws the plan circle with the radius of the top of the loft (read from the body).

### 2.3 Anchors off the drawing of their bone (causes B and C): `anchors.ts`

Every landmark anchor (here the transpiration outlets) is measured as the matrix measures it, over every scale, view and subject. An anchor that fails anywhere is searched a new place on a grid of half a tolerance round its rest position, on its own bone or on any bone that is drawn in the three views and draws the place itself (a bone with child bones is measured against its own shapes only: the descendant rule of the suite would let a stem pass everywhere); the nearest place whose worst distance is at most 0.6 tolerance (bamboo 0.8) is taken, and a change of the bone counts as half a tolerance of movement, so an outlet stays on its bone when its bone draws a place nearby. The result is saved as absolute ops in `W/ops/<id>.json`, which `make.sh` applies. `english-oak`: its twelve outlets were bound to foliage and limb bones that no view draws at any scale (the matrix skips such an anchor); with `--undrawn` they are bound to the stem bone that draws their place. The leaf patches `th-tp-*` of the thermal volumes were not moved (`validateAsset` reports nothing new; physics and breathing tests pass as before).

## 3. Per plant

"Proof" lines are the output of `W/prove.sh` as printed: gate, scan, then silhouette guard, `validateAsset` and three-views sweep outside the test runner twice (first triple: the state before, from the backup; second triple: now), the check CLI with the number of authored scales it calls suitable, the real tests with a name filter, and the matrix of the plant (cells by status).

### `apple-tree`

- Before: scan 11 hard (anchor-landmark 11); gate over every pose, clip instant and transition 359 hard (anchor-landmark 359 (worst 200 mm)).
- After: gate exit 0 hard findings: 0; scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings.
- Changed (`make.sh apple-tree`): `anchors.ts --ops ops/apple-tree.json`: 3 anchors (tp-sun-2 on fol-e1, tp-shade-2 on fol-core-up, tp-shade-5 on fol-top).

```
apple-tree | gate exit 0 hard findings: 0
apple-tree | scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings
apple-tree | apple-tree silhouette: OK
apple-tree | apple-tree validate: 0 issues 
apple-tree | apple-tree three-views sweep: 0 findings 
apple-tree | apple-tree silhouette: OK
apple-tree | apple-tree validate: 0 issues 
apple-tree | apple-tree three-views sweep: 0 findings 
apple-tree | check: exit 0 ✓ apple-tree (1/1); 4 suitability lines
apple-tree | 🧊️catalog-silhouette exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
apple-tree | 🧊️three-views exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
apple-tree | 🌡️catalog-physics exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
apple-tree | 🫁️breathing-catalog exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);
apple-tree | ✅️asset-validation exit 0  Test Files 1 passed (1); Tests 10 passed | 73 skipped (83);
apple-tree | ⚡️asset-power exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
apple-tree | done 02:34
apple-tree | matrix exit 0 [matrix] 252 cells: 84 ok, 0 unavailable, 0 fail; assets: 1 done, 0 failed by name, 0 missing (794 s with the wait in the queue, try 1) status {'ok': 84, 'group-drift': 147, 'range-end': 21}
```

### `bamboo-clump`

- Before: scan 105 hard (anchor-landmark 103, sync-forward 2); gate over every pose, clip instant and transition 1864 hard (anchor-landmark 1856 (worst 273 mm); sync-forward 8 (worst 100 mm)).
- After: gate exit 0 hard findings: 0; scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings.
- Changed (`make.sh bamboo-clump`): `carry.ts --only fol-1,fol-2,fol-4,fol-5 --scales 100,200,500`: 235 depth plates (4804 points) below the front and plan outlines of 4 bones at 1:100, 1:200, 1:500 (fol-1, fol-2, fol-4, fol-5); worst bone drift 173 mm before, 144 mm after; `anchors.ts --ops ops/bamboo-clump.json`: 10 anchors (tp-sun-1 on fol-1, tp-shade-1 on fol-1, tp-sun-2 on fol-2, tp-shade-2 on culm-2-c, tp-sun-3 on fol-3, tp-shade-3 on fol-3, …).

```
bamboo-clump | gate exit 0 hard findings: 0
bamboo-clump | scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings
bamboo-clump | bamboo-clump silhouette: OK
bamboo-clump | bamboo-clump validate: 0 issues 
bamboo-clump | bamboo-clump three-views sweep: 0 findings 
bamboo-clump | bamboo-clump silhouette: OK
bamboo-clump | bamboo-clump validate: 0 issues 
bamboo-clump | bamboo-clump three-views sweep: 0 findings 
bamboo-clump | check: exit 0 ✓ bamboo-clump (1/1); 4 suitability lines
bamboo-clump | 🧊️catalog-silhouette exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
bamboo-clump | 🧊️three-views exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
bamboo-clump | 🌡️catalog-physics exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
bamboo-clump | 🫁️breathing-catalog exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);
bamboo-clump | ✅️asset-validation exit 0  Test Files 1 passed (1); Tests 10 passed | 73 skipped (83);
bamboo-clump | ⚡️asset-power exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
bamboo-clump | done 01:02
bamboo-clump | matrix exit 0 [matrix] 252 cells: 0 ok, 0 unavailable, 0 fail; assets: 1 done, 0 failed by name, 0 missing (1927 s with the wait in the queue, try 2) status {'group-drift': 252}
```

### `black-alder`

- Before: scan 26 hard (sync-bone-height 15, sync-bone-forward 8, anchor-landmark 3); gate over every pose, clip instant and transition 312 hard (anchor-landmark 21 (worst 693 mm); sync-bone-forward 92 (worst 863 mm); sync-bone-height 199 (worst 1041 mm)).
- After: gate exit 0 hard findings: 0; scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings.
- Changed (`make.sh black-alder`): `carry.ts`: 294 depth plates (9407 points) below the front and plan outlines of 13 bones at 1:50, 1:100, 1:200 (fol-1, fol-10, fol-11, fol-12, fol-2, fol-3, fol-4, fol-5, …); worst bone drift 1041 mm before, 294 mm after.

```
black-alder | gate exit 0 hard findings: 0
black-alder | scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings
black-alder | black-alder silhouette: OK
black-alder | black-alder validate: 0 issues 
black-alder | black-alder three-views sweep: 0 findings 
black-alder | black-alder silhouette: OK
black-alder | black-alder validate: 0 issues 
black-alder | black-alder three-views sweep: 0 findings 
black-alder | check: exit 0 ✓ black-alder (1/1); 4 suitability lines
black-alder | 🧊️catalog-silhouette exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
black-alder | 🧊️three-views exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
black-alder | 🌡️catalog-physics exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
black-alder | 🫁️breathing-catalog exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);
black-alder | ✅️asset-validation exit 0  Test Files 1 passed (1); Tests 10 passed | 73 skipped (83);
black-alder | ⚡️asset-power exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
black-alder | matrix exit 0 [matrix] 252 cells: 147 ok, 0 unavailable, 0 fail; assets: 1 done, 0 failed by name, 0 missing (1063 s) status {'ok': 147, 'group-drift': 105} 
black-alder | done 22:38
```

### `black-locust`

- Before: scan 4 hard (sync-forward 4); gate over every pose, clip instant and transition 56 hard (sync-forward 56 (worst 623 mm)).
- After: gate exit 0 hard findings: 0; scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings.
- Changed (`make.sh black-locust`): `rebind.ts`; `carry.ts --limit 1.0 --scales 200,500`: 6 depth plates (80 points) below the front and plan outlines of 1 bones at 1:200, 1:500 (fol-l1); worst bone drift 422 mm before, 54 mm after.

```
black-locust | gate exit 0 hard findings: 0
black-locust | scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings
black-locust | black-locust silhouette: OK
black-locust | black-locust validate: 0 issues 
black-locust | black-locust three-views sweep: 0 findings 
black-locust | black-locust silhouette: OK
black-locust | black-locust validate: 0 issues 
black-locust | black-locust three-views sweep: 0 findings 
black-locust | check: exit 0 ✓ black-locust (1/1); 4 suitability lines
black-locust | 🧊️catalog-silhouette exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
black-locust | 🧊️three-views exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
black-locust | 🌡️catalog-physics exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
black-locust | 🫁️breathing-catalog exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);
black-locust | ✅️asset-validation exit 0  Test Files 1 passed (1); Tests 10 passed | 73 skipped (83);
black-locust | ⚡️asset-power exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
black-locust | done 10:33
black-locust | matrix exit 0 [matrix] 252 cells: 0 ok, 0 unavailable, 0 fail; assets: 1 done, 0 failed by name, 0 missing (1065 s with the wait in the queue, try 1) status {'group-drift': 252}
```

### `common-ash`

- Before: scan 182 hard (anchor-landmark 182); gate over every pose, clip instant and transition 3796 hard (anchor-landmark 3796 (worst 2700 mm)).
- After: gate exit 0 hard findings: 0; scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings.
- Changed (`make.sh common-ash`): `anchors.ts --ops ops/common-ash.json`: 10 anchors (tp-sun-1 on lead-b-1, tp-sun-2 on lead-c-2, tp-sun-4 on lead-a-b, tp-sun-5 on t2-3-b, tp-sun-6 on t1-3, tp-sun-9 on lead-b-2, …).

```
common-ash | gate exit 0 hard findings: 0
common-ash | scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings
common-ash | common-ash silhouette: OK
common-ash | common-ash validate: 0 issues 
common-ash | common-ash three-views sweep: 0 findings 
common-ash | common-ash silhouette: OK
common-ash | common-ash validate: 0 issues 
common-ash | common-ash three-views sweep: 0 findings 
common-ash | check: exit 0 ✓ common-ash (1/1); 4 suitability lines
common-ash | 🧊️catalog-silhouette exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
common-ash | 🧊️three-views exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
common-ash | 🌡️catalog-physics exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
common-ash | 🫁️breathing-catalog exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);
common-ash | ✅️asset-validation exit 0  Test Files 1 passed (1); Tests 10 passed | 73 skipped (83);
common-ash | ⚡️asset-power exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
common-ash | done 00:37
common-ash | matrix exit 0 [matrix] 252 cells: 224 ok, 0 unavailable, 0 fail; assets: 1 done, 0 failed by name, 0 missing (865 s with the wait in the queue, try 1) status {'ok': 224, 'group-drift': 28}
```

### `common-lilac`

- Before: scan 8 hard (sync-bone-forward 4, sync-bone-lateral 4); gate over every pose, clip instant and transition 236 hard (sync-bone-forward 118 (worst 360 mm); sync-bone-lateral 118 (worst 360 mm)).
- After: gate exit 0 hard findings: 0; scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings.
- Changed (`make.sh common-lilac`): `fix-common-lilac.ts`.

```
common-lilac | gate exit 0 hard findings: 0
common-lilac | scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings
common-lilac | common-lilac silhouette: OK
common-lilac | common-lilac validate: 0 issues 
common-lilac | common-lilac three-views sweep: 0 findings 
common-lilac | common-lilac silhouette: OK
common-lilac | common-lilac validate: 0 issues 
common-lilac | common-lilac three-views sweep: 0 findings 
common-lilac | check: exit 0 ✓ common-lilac (1/1); 4 suitability lines
common-lilac | 🧊️catalog-silhouette exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
common-lilac | 🧊️three-views exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
common-lilac | 🌡️catalog-physics exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
common-lilac | 🫁️breathing-catalog exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);
common-lilac | ✅️asset-validation exit 0  Test Files 1 passed (1); Tests 10 passed | 73 skipped (83);
common-lilac | ⚡️asset-power exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
common-lilac | matrix exit 0 [matrix] 252 cells: 252 ok, 0 unavailable, 0 fail; assets: 1 done, 0 failed by name, 0 missing (273 s) status {'ok': 252} 
common-lilac | done 20:59
```

### `common-reed`

- Before: scan 335 hard (anchor-landmark 289, sync-forward 5, sync-bone-forward 41); gate over every pose, clip instant and transition 7425 hard (anchor-landmark 6921 (worst 196 mm); sync-bone-forward 441 (worst 128 mm); sync-forward 63 (worst 128 mm)).
- After: gate exit 0 hard findings: 0; scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings.
- Changed (`make.sh common-reed`): `carry.ts`: 146 depth plates (971 points) below the front and plan outlines of 20 bones at 1:50, 1:100, 1:200, 1:500 (culm-1a, culm-1b, culm-2a, culm-2b, culm-3b, culm-6b, culm-7a, culm-8b, …); worst bone drift 186 mm before, 136 mm after; `anchors.ts --ops ops/common-reed.json`: 12 anchors (tp-leaves-1 on culm-1b, tp-leaves-2 on culm-2b, tp-leaves-3 on culm-3b, tp-leaves-4 on culm-4b, tp-leaves-5 on shoot-1, tp-leaves-6 on culm-6b, …).

```
common-reed | gate exit 0 hard findings: 0
common-reed | scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings
common-reed | common-reed silhouette: OK
common-reed | common-reed validate: 0 issues 
common-reed | common-reed three-views sweep: 0 findings 
common-reed | common-reed silhouette: OK
common-reed | common-reed validate: 0 issues 
common-reed | common-reed three-views sweep: 0 findings 
common-reed | check: exit 0 ✓ common-reed (1/1); 4 suitability lines
common-reed | 🧊️catalog-silhouette exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
common-reed | 🧊️three-views exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
common-reed | 🌡️catalog-physics exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
common-reed | 🫁️breathing-catalog exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);
common-reed | ✅️asset-validation exit 0  Test Files 1 passed (1); Tests 10 passed | 73 skipped (83);
common-reed | ⚡️asset-power exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
common-reed | matrix exit 0 [matrix] 252 cells: 35 ok, 0 unavailable, 0 fail; assets: 1 done, 0 failed by name, 0 missing (259 s) status {'group-drift': 217, 'ok': 35} 
common-reed | done 22:09
```

### `crab-apple`

- Before: scan 10 hard (anchor-landmark 10); gate over every pose, clip instant and transition 279 hard (anchor-landmark 279 (worst 299 mm)).
- After: gate exit 0 hard findings: 0; scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings.
- Changed (`make.sh crab-apple`): `anchors.ts --ops ops/crab-apple.json`: 3 anchors (tp-sun-2 on fol-2, tp-sun-3 on fol-3, tp-sun-4 on fol-4).

```
crab-apple | gate exit 0 hard findings: 0
crab-apple | scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings
crab-apple | crab-apple silhouette: OK
crab-apple | crab-apple validate: 0 issues 
crab-apple | crab-apple three-views sweep: 0 findings 
crab-apple | crab-apple silhouette: OK
crab-apple | crab-apple validate: 0 issues 
crab-apple | crab-apple three-views sweep: 0 findings 
crab-apple | check: exit 0 ✓ crab-apple (1/1); 4 suitability lines
crab-apple | 🧊️catalog-silhouette exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
crab-apple | 🧊️three-views exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
crab-apple | 🌡️catalog-physics exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
crab-apple | 🫁️breathing-catalog exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);
crab-apple | ✅️asset-validation exit 0  Test Files 1 passed (1); Tests 10 passed | 73 skipped (83);
crab-apple | ⚡️asset-power exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
crab-apple | matrix exit 0 [matrix] 252 cells: 105 ok, 0 unavailable, 0 fail; assets: 1 done, 0 failed by name, 0 missing (259 s) status {'ok': 105, 'group-drift': 147} 
crab-apple | done 20:30
```

### `dawn-redwood`

- Before: scan 16 hard (anchor-landmark 16); gate over every pose, clip instant and transition 396 hard (anchor-landmark 396 (worst 945 mm)).
- After: gate exit 0 hard findings: 0; scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings.
- Changed (`make.sh dawn-redwood`): `anchors.ts --ops ops/dawn-redwood.json`: 2 anchors (tp-shade-1 on branch-1-1, tp-shade-2 on branch-1-3).

```
dawn-redwood | gate exit 0 hard findings: 0
dawn-redwood | scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings
dawn-redwood | dawn-redwood silhouette: OK
dawn-redwood | dawn-redwood validate: 0 issues 
dawn-redwood | dawn-redwood three-views sweep: 0 findings 
dawn-redwood | dawn-redwood silhouette: OK
dawn-redwood | dawn-redwood validate: 0 issues 
dawn-redwood | dawn-redwood three-views sweep: 0 findings 
dawn-redwood | check: exit 0 ✓ dawn-redwood (1/1); 4 suitability lines
dawn-redwood | 🧊️catalog-silhouette exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
dawn-redwood | 🧊️three-views exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
dawn-redwood | 🌡️catalog-physics exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
dawn-redwood | 🫁️breathing-catalog exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);
dawn-redwood | ✅️asset-validation exit 0  Test Files 1 passed (1); Tests 10 passed | 73 skipped (83);
dawn-redwood | ⚡️asset-power exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
dawn-redwood | matrix exit 0 [matrix] 252 cells: 252 ok, 0 unavailable, 0 fail; assets: 1 done, 0 failed by name, 0 missing (1943 s) status {'ok': 252} 
dawn-redwood | done 22:01
```

### `english-oak`

- Before: scan 90 hard (anchor-landmark 84, sync-height 3, sync-lateral 1, sync-bone-forward 2); gate over every pose, clip instant and transition 2138 hard (anchor-landmark 2063 (worst 4354 mm); sync-height 16 (worst 421 mm); sync-lateral 2 (worst 419 mm); sync-bone-forward 57 (worst 553 mm)).
- After: gate exit 0 hard findings: 0; scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings.
- Changed (`make.sh english-oak`): `rebind.ts --stem`; `fix-english-oak.ts`; `carry.ts --area 0.002`: 50 depth plates (2259 points) below the front and plan outlines of 2 bones at 1:50, 1:100, 1:200, 1:500 (fol-top, leader-top); worst bone drift 553 mm before, 59 mm after; `anchors.ts --ops ops/english-oak.json`: 12 anchors (tp-sun-1 on trunk, tp-sun-2 on trunk-upper, tp-sun-3 on trunk-upper, tp-sun-4 on trunk-upper, tp-sun-5 on trunk-upper, tp-sun-6 on trunk-upper, …).

```
english-oak | gate exit 0 hard findings: 0
english-oak | scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings
english-oak | english-oak silhouette: OK
english-oak | english-oak validate: 0 issues 
english-oak | english-oak three-views sweep: 0 findings 
english-oak | english-oak silhouette: OK
english-oak | english-oak validate: 0 issues 
english-oak | english-oak three-views sweep: 0 findings 
english-oak | check: exit 0 ✓ english-oak (1/1); 4 suitability lines
english-oak | 🧊️catalog-silhouette exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
english-oak | 🧊️three-views exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
english-oak | 🌡️catalog-physics exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
english-oak | 🫁️breathing-catalog exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);
english-oak | ✅️asset-validation exit 0  Test Files 1 passed (1); Tests 10 passed | 73 skipped (83);
english-oak | ⚡️asset-power exit 0  Test Files 1 passed (1); Tests 2 passed | 22 skipped (24);
english-oak | done 10:17
english-oak | matrix exit 0 [matrix] 252 cells: 182 ok, 0 unavailable, 0 fail; assets: 1 done, 0 failed by name, 0 missing (1655 s with the wait in the queue, try 1) status {'ok': 182, 'group-drift': 70}
```

### `european-beech`

- Before: scan 191 hard (anchor-landmark 191); gate over every pose, clip instant and transition 3939 hard (anchor-landmark 3939 (worst 3559 mm)).
- After: gate exit 0 hard findings: 0; scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings.
- Changed (`make.sh european-beech`): `anchors.ts --ops ops/european-beech.json`: 11 anchors (tp-sun-2 on t1-4-b, tp-sun-3 on t3-4-b, tp-sun-4 on t3-4-b, tp-sun-5 on t3-1-b, tp-sun-7 on t2-1-b, tp-sun-8 on t5-1, …).

```
european-beech | gate exit 0 hard findings: 0
european-beech | scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings
european-beech | european-beech silhouette: OK
european-beech | european-beech validate: 0 issues 
european-beech | european-beech three-views sweep: 0 findings 
european-beech | european-beech silhouette: OK
european-beech | european-beech validate: 0 issues 
european-beech | european-beech three-views sweep: 0 findings 
european-beech | check: exit 0 ✓ european-beech (1/1); 4 suitability lines
european-beech | 🧊️catalog-silhouette exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
european-beech | 🧊️three-views exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
european-beech | 🌡️catalog-physics exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
european-beech | 🫁️breathing-catalog exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);
european-beech | ✅️asset-validation exit 0  Test Files 1 passed (1); Tests 10 passed | 73 skipped (83);
european-beech | ⚡️asset-power exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
european-beech | matrix exit 0 [matrix] 252 cells: 252 ok, 0 unavailable, 0 fail; assets: 1 done, 0 failed by name, 0 missing (2127 s with the wait in the queue, try 1) status {'ok': 252}
```

### `field-maple`

- Before: scan 18 hard (sync-forward 3, anchor-landmark 14, sync-bone-height 1); gate over every pose, clip instant and transition 299 hard (sync-forward 13 (worst 245 mm); anchor-landmark 282 (worst 351 mm); sync-bone-height 4 (worst 224 mm)).
- After: gate exit 0 hard findings: 0; scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings.
- Changed (`make.sh field-maple`): `carry.ts`: 170 depth plates (5541 points) below the front and plan outlines of 11 bones at 1:50, 1:100, 1:200 (fol-core-low, fol-limb-a1, fol-limb-a2, fol-limb-b1, fol-limb-b2, fol-limb-c1, fol-limb-c2, fol-limb-c3, …); worst bone drift 392 mm before, 150 mm after; `anchors.ts --ops ops/field-maple.json`: 2 anchors (tp-sun-2 on fol-limb-b1, tp-sun-6 on fol-limb-a2).

```
field-maple | gate exit 0 hard findings: 0
field-maple | scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings
field-maple | field-maple silhouette: OK
field-maple | field-maple validate: 0 issues 
field-maple | field-maple three-views sweep: 0 findings 
field-maple | field-maple silhouette: OK
field-maple | field-maple validate: 0 issues 
field-maple | field-maple three-views sweep: 0 findings 
field-maple | check: exit 0 ✓ field-maple (1/1); 4 suitability lines
field-maple | 🧊️catalog-silhouette exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
field-maple | 🧊️three-views exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
field-maple | 🌡️catalog-physics exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
field-maple | 🫁️breathing-catalog exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);
field-maple | ✅️asset-validation exit 0  Test Files 1 passed (1); Tests 10 passed | 73 skipped (83);
field-maple | ⚡️asset-power exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
field-maple | matrix exit 0 [matrix] 252 cells: 252 ok, 0 unavailable, 0 fail; assets: 1 done, 0 failed by name, 0 missing (530 s) status {'ok': 252} 
field-maple | done 21:16
```

### `fig-tree`

- Before: scan 8 hard (anchor-landmark 8); gate over every pose, clip instant and transition 247 hard (anchor-landmark 247 (worst 188 mm)).
- After: gate exit 0 hard findings: 0; scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings.
- Changed (`make.sh fig-tree`): `anchors.ts --ops ops/fig-tree.json`: 2 anchors (tp-sun-7 on fol-top, tp-shade-2 on fol-core-low).

```
fig-tree | gate exit 0 hard findings: 0
fig-tree | scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings
fig-tree | fig-tree silhouette: OK
fig-tree | fig-tree validate: 0 issues 
fig-tree | fig-tree three-views sweep: 0 findings 
fig-tree | fig-tree silhouette: OK
fig-tree | fig-tree validate: 0 issues 
fig-tree | fig-tree three-views sweep: 0 findings 
fig-tree | check: exit 0 ✓ fig-tree (1/1); 4 suitability lines
fig-tree | 🧊️catalog-silhouette exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
fig-tree | 🧊️three-views exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
fig-tree | 🌡️catalog-physics exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
fig-tree | 🫁️breathing-catalog exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);
fig-tree | ✅️asset-validation exit 0  Test Files 1 passed (1); Tests 10 passed | 73 skipped (83);
fig-tree | ⚡️asset-power exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
fig-tree | matrix exit 0 [matrix] 252 cells: 252 ok, 0 unavailable, 0 fail; assets: 1 done, 0 failed by name, 0 missing (626 s) status {'ok': 252} 
fig-tree | done 20:52
```

### `hawthorn`

- Before: scan 46 hard (sync-bone-forward 15, sync-forward 3, sync-height 3, sync-bone-height 12, anchor-landmark 13); gate over every pose, clip instant and transition 830 hard (anchor-landmark 271 (worst 248 mm); sync-bone-forward 269 (worst 496 mm); sync-bone-height 190 (worst 673 mm); sync-forward 25 (worst 245 mm); sync-height 75 (worst 443 mm)).
- After: gate exit 0 hard findings: 0; scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings.
- Changed (`make.sh hawthorn`): `carry.ts`: 198 depth plates (7526 points) below the front and plan outlines of 10 bones at 1:50, 1:100, 1:200 (fol-1, fol-2, fol-3, fol-4, fol-5, fol-6, fol-7, limb-1, …); worst bone drift 673 mm before, 95 mm after.

```
hawthorn | gate exit 0 hard findings: 0
hawthorn | scan exit 0 [scan] 1 assets: 1 clean, 0 with a hard finding, 0 hard findings
hawthorn | hawthorn silhouette: OK
hawthorn | hawthorn validate: 0 issues 
hawthorn | hawthorn three-views sweep: 0 findings 
hawthorn | hawthorn silhouette: OK
hawthorn | hawthorn validate: 0 issues 
hawthorn | hawthorn three-views sweep: 0 findings 
hawthorn | check: exit 0 ✓ hawthorn (1/1); 4 suitability lines
hawthorn | 🧊️catalog-silhouette exit 0  Test Files 1 passed (1); Tests 1 passed | 384 skipped (385);
hawthorn | 🧊️three-views exit 0  Test Files 1 passed (1); Tests 2 passed | 773 skipped (775);
hawthorn | 🌡️catalog-physics exit 0  Test Files 1 passed (1); Tests 2 passed | 838 skipped (840);
hawthorn | 🫁️breathing-catalog exit 0  Test Files 1 passed (1); Tests 1 passed | 289 skipped (290);
hawthorn | ✅️asset-validation exit 0  Test Files 1 passed (1); Tests 10 passed | 73 skipped (83);
hawthorn | ⚡️asset-power exit 0  Test Files 1 skipped (1); Tests 24 skipped (24);
hawthorn | matrix exit 0 [matrix] 252 cells: 84 ok, 0 unavailable, 0 fail; assets: 1 done, 0 failed by name, 0 missing (385 s) status {'ok': 84, 'group-drift': 168} 
hawthorn | done 20:10
```

## 4. Sheets

Product sheets (side, front, plan; line and solid) rendered with `W/look.sh` (list of the takeover: `W/sheets.tsv`, 24 sheets through `W/sheets.sh`; the first fixer's sheets of `hawthorn` 1:50 and 1:200 in `summer-calm` and `gale` were still on disk) and opened by the takeover: `black-locust` 1:100 default pose before and after, 1:500 `storm` after; `english-oak` 1:200 default pose after; `black-alder` 1:100 `gale` before and after; `field-maple` 1:100 `storm` after; `common-reed` 1:100 `storm` after; `hawthorn` 1:200 `gale` before and after. The sheets are tool output and are deleted with the generated folder.

- Default poses: unchanged where plates were added (they lie inside and below the drawn outline). `black-locust` 1:100: the side elevation now draws the whole outline of two foliage masses that the stem bone hid in part before (each mass is carried at the depth of its own limb); nothing else differs. `english-oak` 1:200: no visible difference (one vertex moved 9 mm).
- Wind poses, what the plates buy: the views agree (`hawthorn` 1:200 `gale`: front 5.90 m before against 6.15 m in the side elevation, 6.13 m after).
- Wind poses, what the plates cost, stated plainly: in `gale` and `storm` the front and the plan show the lobes of the plates beside the outline and, where a slender part was cut straight, short straight edges. Most visible on `black-alder` 1:100 `gale` (front: a straight horizontal edge and two notches in the middle of the crown; plan: a stepped edge on the lee side), minor on `field-maple` 1:100 `storm` (two ragged edge pieces in the front) and on `hawthorn` 1:200 `gale` (two lobes below the top mass). `common-reed` and `black-locust` 1:500 `storm` show no such edge. The drawings of those poses were wrong before (extents up to 1.0 m apart between the views) and are right in their extents now, but less calm in line; no gate measures this, so it is named here and under "open".

## 5. Open

- No asset of the batch has a hard scan finding, a failing matrix cell or a red proof line (section 1).
- Drawing quality in wind poses (no failing cell, judged on the sheets, section 4): the depth plates of `carry.ts` show straight cut edges and lobes in `gale` and `storm`, most on `black-alder` (294 plates), less on `field-maple` (170) and `hawthorn` (198). A cleaner repair is a real solid per foliage mass (one loft whose side and front profiles are the two outlines) instead of plates; that is a new tool, not done in this batch.
- `black-locust`: the bone `ld2` draws different parts in the elevation (6 shapes) and in front and plan (2 shapes); the matrix reports it as group drift (2350 mm in the default pose, reported tier, no failing cell). The 14 elevation shapes of the limb `l9` stay on the stem bone (edge-on plane).
- Reported tier (group drift) remains on eight plants, see the matrix column of section 1 (cells with status `group-drift` are not `fail`).
- Declared dimensions were not touched; none of the repairs changes a default-pose extent.
- The coordinator named `rowan` and `saucer-magnolia` as leftovers: they are assets of batch MV-3 (`🧰️matrix/batches/MV-3.tsv`), not of MV-1; they were not touched here.

## 6. Takeover of 2026-11-25: state found and what was done

- Found at 09:00: no report file; 14 catalog files written by the first fixer between 18:45 and 20:20 on 2026-11-24, each byte-identical to its trial copy or to a later `make.sh` run; complete proof logs for all 14 (runs that ended, 20:10 to 06:25). 12 plants green in every line. `black-locust`: the real `🧊️catalog-silhouette` test failed (front boundary 53.7 mm at 1:50 and 50.3 mm at 1:100, limit 30 mm), a regression against the backup. `english-oak`: 3 hard scan findings `unavailable-scale-unsuitable` at 1:200, check CLI `✗`, 21 failing matrix cells.
- Measured again by the takeover on the catalog files (09:00 to 09:07, `T/🔎️scan.ts --language en` over the 14 ids in one run): 13 clean, `english-oak` 3 hard findings. The catalog files of the 12 green plants were not written again; their proof lines in section 3 are the first fixer's complete runs on exactly these bytes.
- `english-oak`: `fix-english-oak.ts` (new step of the chain, section 2.2), tried on a copy first (every scale prints, gate 0), then `make.sh english-oak`; the catalog file equals the trial copy byte for byte; all proofs and the matrix run again (section 3).
- `black-locust`: trials on copies: without plates the guard passes but the gate has 10 hard findings (`fol-l1`, 1:200 and 1:500); with the plates of `fol-l1` at all four scales the guard fails at 1:50 and 1:100. Written: plates of `fol-l1` at 1:200 and 1:500 only (`plants.tsv`: `--limit 1.0 --scales 200,500`; the 36 plates of the earlier flags removed first with `carry.ts --limit 99`, as `finish.sh` with `STRIP=1` does), then all proofs and the matrix again.
- Idempotence (`W/again.sh`): a second `make.sh` changed no byte of `black-locust` (sha-256 `f8c93712678f56ac…`) and of `english-oak` (`cc5d67aca6783df6…`).
- Memory, measured with `/usr/bin/time -l` (queue rule of 09:20): `english-oak` (20 MB document): `📊️matrix.ts --assets` 1456 MB peak (1139 s), anchor search 1829 MB, `measure.ts` 1822 MB, `carry.ts` 1323 MB, check CLI 984 MB, scan and gate below 450 MB. `black-locust` (1.7 MB): matrix 821 MB, `measure.ts` 979 MB, `carry.ts` 243 MB. The 1200 MB of the queue rule are too little for the matrix of a tree as large as `english-oak`: peak plus 30 % is 1900 MB.

## 7. Files

Updated (catalog, `S/🗂️catalog/🌳️vegetation/<id>/🔣️.json`): `apple-tree`, `bamboo-clump`, `black-alder`, `black-locust`, `common-ash`, `common-lilac`, `common-reed`, `crab-apple`, `dawn-redwood`, `english-oak`, `european-beech`, `field-maple`, `fig-tree`, `hawthorn`.

Created or updated in `U/🧰️matrix/MV-1/` (kept): first fixer: `anchors.ts`, `carry.ts`, `rebind.ts`, `fix-common-lilac.ts`, `bone.ts`, `probe.ts`, `extreme.ts`, `make.sh`, `trial.sh`, `prove.sh`, `matrix.sh`, `vt.sh`, `look.sh`, `status.sh`, `q.sh`, `plants.tsv`, `before.json`, `ops/<id>.json` (14), `report-head.txt`, `report-rows.py`, `write-report.py`; takeover: created `fix-english-oak.ts`, `suit.ts`, `exp.sh`, `finish.sh`, `takeover.sh`, `again.sh`, `sheets.sh`, `sheets.tsv`, `sheets.txt`, `open.txt`, `report-tail.txt`; updated `plants.tsv` (flags of `black-locust`), `q.sh`, `make.sh`, `trial.sh`, `prove.sh`, `matrix.sh` (reservations of the queue rule, runs inside one reservation), `report-head.txt`, `write-report.py`.

Created: `U/multiview-MV-1.md` (this report). Removed: `U/🗑️generated/matrix/MV-1` (backups, trial copies, logs, matrix output, sheets). No request was filed in `U/requests.md`.
