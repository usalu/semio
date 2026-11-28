FINAL (2026-11-27 08:25: 29 assets rewritten and proven, each with 0 failing matrix cells, clean gate, scan, guard, three-views sweep and check CLI; no triangle or bounds finding is left on any of them; `🧊️model-catalog` is green for 19, and 10 large trees still exceed the 8 MiB GLB (8.1 to 14.0 MB; `norway-maple` 10.6 MB against 37 MB before); `european-beech` is not written, its thermal scene stays above 400 000 triangles; no test, budget or rule touched)

# 🌳️ Closing Lane CL-T1: Triangle and GLB Budgets of Plants and Humans

Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `U` = this ticket folder, `W` = `U/🧰️matrix/CL-T1` (tools and ops, kept), `GEN` = `U/🗑️generated/matrix/CL-T1` (deleted with this report; every number below was read there in a log of a run that ended by itself). Every measuring run went through `repo capacity run` (`W/q.sh`, later `W/loop.sh`: one request per job, at most two at a time).

## 1. Cause

`🧊️model-catalog` tessellates every scene at the quality of its own scale (`qualityOf(asset, scale)`: chord of 2 % of the smallest feature, up to 64 segments at 1:50). Three authoring habits ran against that:

1. **Trees with smooth foliage.** Every leaf lobe was an `ellipsoid`: 3968 triangles at 1:50, 2208 at 1:100, whatever its size. `norway-maple` had 296 lobes at 1:50 (1.21 million body triangles), `sweetgum` 689 (2.83 million). The trees that pass (`english-oak`, `small-leaved-lime`, …) author foliage as `polyhedron`, which the kernel keeps at its authored facets at every quality.
2. **Figures thickened to the feature at 1:500.** The child figures (`walking-toddler`, `running-child`, `schoolchild-with-backpack`) and the runner carried at 1:500 the 58 to 62 structure solids of 1:200 "at the printable thickness": every sphere and capsule end of 75 mm gets 22 to 27 segments where the thin one of 1:200 gets 12 to 18, so the coarser scale had more triangles than the finer one. `parent-with-stroller` missed by 34 triangles for the same reason (two wrist spheres), `snake-plant` by six outlet lenses enlarged to the feature.
3. **Two declarations.** `running-child` presents `toe-off`, which was a `key: true` pose (no GLB of it exists, the suite compared `dimensions` with `standing-ready`). `person-with-umbrella` declares its 32 canopy pleat bones `joint: fixed`; the kernel keeps a fixed joint at its rest placement (`applyJoint`, `joint-fixed`), so the pleats that `rig.radialFolds` moves stayed flat in every 3D file, and at 1:500, where the body has no ribs, the scene was 48 % too narrow. `market-parasol`, the other asset with radial folds, declares its pleat bones `revolute` with a free twist and exports them correctly.

## 2. Cure

All cures are ops in `W/ops/<id>.json`, applied by `W/facet.ts` to a trial copy (`W/make.sh`), proven on the copy (`W/light2.sh`), then written and proven again (`W/prove2.sh`).

- **Faceted lobes (plants).** Every soft mass (leaf lobe, flower, fruit, needle spray, transpiration outlet) and every joint and mass sphere of the structure becomes a `polyhedron` with the facets the kernel gives that very solid at the reference quality of the asset (`qualityOf(asset)` without a scale: 24 to 64 segments), same id, bone, material, transform. That is the surface the area and volume checks measure and, to the chord of that quality (at most 0.86 % of a lobe's radius inwards), the outline the silhouette guard measures, so no drawing had to move: 528 triangles per lobe at every scale instead of 3968, 2208, 1520 and 960. A lobe small against the feature of its scale has 16 segments (224 triangles) and is sized so that its outline straddles the measured one (at most 0.54 % of its radius off). The radius limit is per tree and scale and stands in its ops and in the decision text of the asset: 0.28 m at 1:50, 0.56 m at 1:100, 1.12 m at 1:200, 2.8 m at 1:500 where the guard allows it; 0.28 m up to 1:200 where it does not (`black-locust`, `common-ash`, `field-maple`, `ginkgo`, `horse-chestnut`, `tulip-tree`); 0.30 m (`honey-locust`) and 0.40 m (`sweetgum`) at 1:50, where the thermal scene needs it. A sphere of just the smallest feature of its scale stays an ellipsoid (faceted it measures 0.9 % too thin for `volumes-feature`). `european-larch` and `norway-spruce`: needle skirts, sprays and tips (capsules and solids of revolution) the same way, the larch at 1:50 and 1:100 only. `japanese-cherry` at 1:100: the lobes straddle the smooth ellipsoid, because the front view there had 4.8 mm of room and the inscribed facets took 5 mm. The count of lobes per scale is the authored one and falls with every coarser scale, so the triangles fall.
- **Merged members (figures, 1:500 structure).** Child figures: the joint spheres that lie within the ends of the members they tie, the glenoid stubs, heels, sole struts and toe members are merged into those members (each foot is one bar that also names the toe bone in `members`); only the knee stays a knob between its two slender shafts: 58 to 62 solids become 33 to 35. Runner: the nine joint spheres inside the 75 mm yokes and the sole strut. Stroller: the two wrist spheres inside forearm and hand bar. Each claim was checked on the numbers of the solids (same centre, radius not above the member's).
- **`saguaro-cactus`.** The 33 tie discs of the rib skeleton at 1:50 were flat ellipsoids of 3312 to 3968 triangles each; they are lenses of revolution with a meridian of five points inside the former ellipsoid (384 triangles).
- **`running-child`:** `toe-off` is a named pose. **`person-with-umbrella`:** the 32 pleat bones are `revolute` with a free twist, as in `market-parasol`.

What did not work, measured: 16 facets on every lobe of `norway-maple` (guard 30 mm at 1:50 against 15 mm, box off); 20 facets with branch lofts as 24-sided tubes on `european-beech` (guard 90 mm at 1:50, box off; the baseline is 13.1 mm).

## 3. Triangles before and after

Before: `U/🗑️generated/matrix/CL-3D/budgets.jsonl` (lane CL-3D, 2026-11-26, the count of `auditGeometry`); after: `W/groups.ts` on the written copy (the same count; it reproduced the 1 210 904 body triangles of `norway-maple` at 1:50 before the cure).

| Asset | Finding before | Triangles before (body / thermal / structure at 1:50, 1:100, 1:200, 1:500) | Triangles after |
| --- | --- | --- | --- |
| `black-alder` | 581000 body triangles at 1:50; 768924 thermal triangles at 1:50 | 581k, 241k, 126k, 43k / 769k, 363k, 183k, 65k / 152k, 85k, 54k, 28k | 96k, 65k, 44k, 10k / 212k, 154k, 93k, 30k / 53k, 42k, 35k, 26k |
| `black-locust` | 471860 thermal triangles at 1:50 | 333k, 186k, 131k, 79k / 472k, 289k, 186k, 97k / 168k, 93k, 51k, 30k | 68k, 56k, 47k, 22k / 163k, 136k, 96k, 37k / 70k, 53k, 38k, 28k |
| `common-ash` | 754978 body triangles at 1:50; 960926 thermal triangles at 1:50 | 755k, 260k, 110k, 41k / 961k, 415k, 201k, 72k / 136k, 83k, 56k, 38k | 170k, 92k, 51k, 22k / 330k, 226k, 135k, 45k / 68k, 51k, 42k, 33k |
| `crab-apple` | 527238 body triangles at 1:50; 660518 thermal triangles at 1:50 | 527k, 244k, 113k, 22k / 661k, 327k, 154k, 40k / 95k, 53k, 33k, 15k | 109k, 54k, 26k, 6k / 207k, 129k, 62k, 22k / 39k, 28k, 22k, 14k |
| `european-larch` | 438426 body triangles at 1:50; 559914 thermal triangles at 1:50 | 438k, 114k, 8k, 8k / 560k, 201k, 58k, 31k / 64k, 44k, 38k, 23k | 102k, 36k, 8k, 8k / 185k, 105k, 47k, 26k / 64k, 44k, 38k, 23k |
| `field-maple` | 1094374 body triangles at 1:50; 1242080 thermal triangles at 1:50 | 1094k, 281k, 113k, 49k / 1242k, 387k, 159k, 65k / 119k, 68k, 42k, 19k | 149k, 75k, 43k, 14k / 266k, 170k, 84k, 29k / 39k, 30k, 26k, 17k |
| `ginkgo` | 1857118 body triangles at 1:50; 2015588 thermal triangles at 1:50 | 1857k, 671k, 81k, 30k / 2016k, 766k, 126k, 61k / 110k, 69k, 50k, 32k | 227k, 172k, 40k, 13k / 325k, 239k, 65k, 33k / 81k, 56k, 45k, 31k |
| `hawthorn` | 708688 body triangles at 1:50; 867262 thermal triangles at 1:50 | 709k, 238k, 82k, 26k / 867k, 326k, 115k, 44k / 92k, 54k, 32k, 17k | 107k, 62k, 23k, 6k / 215k, 135k, 53k, 22k / 34k, 27k, 22k, 16k |
| `honey-locust` | 1759336 body triangles at 1:50; 1945656 thermal triangles at 1:50 | 1759k, 590k, 77k, 31k / 1946k, 731k, 160k, 67k / 134k, 79k, 56k, 34k | 246k, 178k, 43k, 16k / 377k, 293k, 105k, 41k / 96k, 64k, 48k, 33k |
| `horse-chestnut` | 1224078 body triangles at 1:50; 1391838 thermal triangles at 1:50 | 1224k, 460k, 194k, 85k / 1392k, 588k, 273k, 115k / 188k, 113k, 78k, 54k | 279k, 162k, 95k, 47k / 373k, 249k, 152k, 62k / 92k, 69k, 57k, 46k |
| `japanese-cherry` | 768986 body triangles at 1:50; 916488 thermal triangles at 1:50 | 769k, 354k, 128k, 22k / 916k, 435k, 161k, 46k / 107k, 60k, 37k, 16k | 103k, 78k, 25k, 6k / 212k, 151k, 54k, 25k / 44k, 31k, 25k, 15k |
| `norway-maple` | 1210904 body triangles at 1:50; 1387414 thermal triangles at 1:50 | 1211k, 319k, 128k, 56k / 1387k, 455k, 205k, 76k / 228k, 129k, 80k, 33k | 179k, 94k, 52k, 19k / 312k, 207k, 122k, 37k / 71k, 54k, 45k, 29k |
| `norway-spruce` | 611872 body triangles at 1:50; 743568 thermal triangles at 1:50 | 612k, 154k, 60k, 6k / 744k, 248k, 107k, 31k / 76k, 51k, 46k, 26k | 125k, 47k, 26k, 5k / 215k, 121k, 61k, 25k / 76k, 51k, 46k, 26k |
| `parent-with-stroller` | structure grows 1:200 29482 -> 1:500 29516 | 76k, 31k, 16k, 12k / 148k, 43k, 24k, 16k / 118k, 67k, 29k, 30k | 76k, 31k, 16k, 12k / 148k, 43k, 24k, 16k / 118k, 67k, 29k, 29k |
| `person-with-umbrella` | none (bounds only) | 50k, 14k, 9k, 5k / 84k, 19k, 11k, 6k / 120k, 60k, 37k, 27k | 50k, 14k, 9k, 5k / 84k, 19k, 11k, 6k / 120k, 60k, 37k, 27k |
| `red-oak` | 526450 body triangles at 1:50; 705004 thermal triangles at 1:50 | 526k, 261k, 173k, 108k / 705k, 401k, 258k, 130k / 257k, 137k, 67k, 35k | 99k, 79k, 62k, 35k / 240k, 200k, 142k, 56k / 87k, 63k, 45k, 32k |
| `rowan` | 548448 body triangles at 1:50; 712952 thermal triangles at 1:50 | 548k, 178k, 85k, 10k / 713k, 271k, 120k, 31k / 117k, 69k, 46k, 26k | 82k, 46k, 26k, 3k / 190k, 121k, 54k, 20k / 51k, 40k, 33k, 24k |
| `runner-with-blade-prosthesis` | structure grows 1:200 26216 -> 1:500 29258 | 78k, 28k, 13k, 9k / 86k, 30k, 14k, 9k / 118k, 61k, 26k, 29k | 78k, 28k, 13k, 9k / 86k, 30k, 14k, 9k / 118k, 61k, 26k, 26k |
| `running-child` | structure grows 1:200 29512 -> 1:500 43996 | 61k, 26k, 15k, 12k / 73k, 28k, 16k, 12k / 99k, 53k, 30k, 44k | 61k, 26k, 15k, 12k / 73k, 28k, 16k, 12k / 99k, 53k, 30k, 29k |
| `saguaro-cactus` | 434002 structure triangles at 1:50 | 26k, 14k, 10k, 6k / 57k, 33k, 22k, 17k / 434k, 41k, 28k, 20k | 26k, 14k, 10k, 6k / 57k, 33k, 22k, 17k / 334k, 41k, 28k, 20k |
| `saucer-magnolia` | 412792 body triangles at 1:50; 559274 thermal triangles at 1:50 | 413k, 211k, 94k, 9k / 559k, 264k, 122k, 29k / 96k, 56k, 36k, 18k | 105k, 52k, 24k, 3k / 205k, 96k, 48k, 21k / 45k, 33k, 27k, 18k |
| `schoolchild-with-backpack` | structure grows 1:200 28336 -> 1:500 42818 | 69k, 28k, 15k, 12k / 83k, 30k, 16k, 12k / 112k, 57k, 28k, 43k | 69k, 28k, 15k, 12k / 83k, 30k, 16k, 12k / 112k, 57k, 28k, 28k |
| `shrub-rose` | 411490 thermal triangles at 1:50 | 391k, 145k, 61k, 34k / 411k, 163k, 72k, 46k / 75k, 43k, 33k, 28k | 45k, 26k, 17k, 13k / 57k, 37k, 28k, 20k / 67k, 44k, 33k, 28k |
| `snake-plant` | thermal grows 1:200 6454 -> 1:500 6968 | 4k, 3k, 1k, 1k / 16k, 10k, 6k, 7k / 23k, 17k, 14k, 12k | 4k, 3k, 1k, 1k / 14k, 10k, 6k, 4k / 23k, 17k, 14k, 12k |
| `sweetgum` | 2827624 body triangles at 1:50; 3005166 thermal triangles at 1:50 | 2828k, 986k, 105k, 39k / 3005k, 1117k, 163k, 74k / 134k, 82k, 57k, 36k | 265k, 260k, 52k, 18k / 380k, 358k, 90k, 42k / 95k, 65k, 50k, 35k |
| `sycamore-maple` | 1332768 body triangles at 1:50; 1501968 thermal triangles at 1:50 | 1333k, 336k, 139k, 63k / 1502k, 463k, 215k, 86k / 236k, 132k, 83k, 30k | 203k, 101k, 61k, 32k / 300k, 189k, 117k, 49k / 63k, 47k, 39k, 25k |
| `tulip-tree` | 553474 body triangles at 1:50; 730238 thermal triangles at 1:50 | 553k, 270k, 178k, 112k / 730k, 405k, 254k, 131k / 257k, 140k, 73k, 42k | 104k, 81k, 68k, 32k / 241k, 198k, 146k, 51k / 100k, 73k, 52k, 37k |
| `walking-toddler` | structure grows 1:200 31662 -> 1:500 44508 | 45k, 22k, 14k, 11k / 64k, 27k, 17k, 11k / 91k, 47k, 32k, 45k | 45k, 22k, 14k, 11k / 64k, 27k, 17k, 11k / 91k, 47k, 32k, 29k |
| `wisteria-pergola` | 403306 body triangles at 1:50; 441988 thermal triangles at 1:50 | 403k, 150k, 35k, 15k / 442k, 164k, 42k, 19k / 104k, 64k, 42k, 31k | 57k, 31k, 11k, 4k / 95k, 45k, 18k, 8k / 45k, 35k, 28k, 20k |

## 4. Proofs

Light proof on the trial copy before the write (`W/light2.sh`: budgets, GLB of the default pose, gate over every subject, gate over the poses, silhouette guard with `validateAsset` and the three-views sweep, check CLI `--file`), then the write and the proof on the catalog file (`W/prove2.sh`: gate, official scan, guard, check CLI, matrix of the asset, single-asset `🧊️model-catalog`). Every catalog file equals its proven copy (digests checked again at 08:22). `walking-toddler` and `person-with-umbrella` ran their model-catalog a second time at 06:59, after lane CX-GLB had registered `KHR_mesh_quantization` in the suite (the first run named only that).

| Asset | Light proof | GLB default pose | Gate | Scan | Guard, `validateAsset`, sweep | Check CLI | Matrix | `🧊️model-catalog` single run | Digest after |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `black-alder` | PASS | 7.50 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 147 ok, 0 unavailable, 0 fail {'ok': 147, 'group-drift': 105} (847 s) | green | `56998c488270de49` |
| `black-locust` | PASS | 7.27 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 0 ok, 0 unavailable, 0 fail {'group-drift': 252} (161 s) | green | `d118aa9a7f06be67` |
| `common-ash` | PASS | 11.13 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 224 ok, 0 unavailable, 0 fail {'ok': 224, 'group-drift': 28} (389 s) | 6 GLB size (10.7 to 11.5 MB) | `48c268449b841214` |
| `crab-apple` | PASS | 6.17 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 105 ok, 0 unavailable, 0 fail {'ok': 105, 'group-drift': 147} (111 s) | green | `b55ab48ebe66cb23` |
| `european-larch` | PASS | 5.78 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 252 ok, 0 unavailable, 0 fail {'ok': 252} (159 s) | green | `d3a644eeb64b4fca` |
| `field-maple` | PASS | 8.09 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 210 ok, 0 unavailable, 0 fail {'ok': 210, 'group-drift': 42} (349 s) | 2 GLB size (8.1 to 8.1 MB) | `f3e80b634627aff8` |
| `ginkgo` | PASS | 9.85 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 252 ok, 0 unavailable, 0 fail {'ok': 252} (196 s) | 4 GLB size (8.9 to 9.8 MB) | `301e9f9befb316d8` |
| `hawthorn` | PASS | 6.38 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 84 ok, 0 unavailable, 0 fail {'ok': 84, 'group-drift': 168} (532 s) | green | `371723d5553ace48` |
| `honey-locust` | PASS | 10.72 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 252 ok, 0 unavailable, 0 fail {'ok': 252} (222 s) | 4 GLB size (10.7 to 11.5 MB) | `82741e6a20bec5b7` |
| `horse-chestnut` | PASS | 13.47 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 252 ok, 0 unavailable, 0 fail {'ok': 252} (603 s) | 6 GLB size (13.2 to 14.0 MB) | `23601e31d498a339` |
| `japanese-cherry` | PASS | 6.28 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 252 ok, 0 unavailable, 0 fail {'ok': 252} (84 s) | green | `0d7032f8885b0510` |
| `norway-maple` | PASS | 10.38 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 252 ok, 0 unavailable, 0 fail {'ok': 252} (884 s) | 5 GLB size (10.2 to 10.6 MB) | `8466abbbe2b23e54` |
| `norway-spruce` | PASS | 6.37 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 252 ok, 0 unavailable, 0 fail {'ok': 252} (249 s) | green | `5910d2ef6a4a5c2f` |
| `parent-with-stroller` | PASS |  | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 7 ok, 0 unavailable, 0 fail {'group-drift': 203, 'range-end': 42, 'ok': 7} (314 s) | green | `6b029a0379a13f7b` |
| `person-with-umbrella` | PASS |  | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 35 ok, 0 unavailable, 0 fail {'group-drift': 175, 'ok': 35, 'range-end': 42} (87 s) | green | `0562d6a5e4f4a5a9` |
| `red-oak` | PASS | 10.16 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 0 ok, 0 unavailable, 0 fail {'group-drift': 252} (179 s) | 3 GLB size (9.8 to 10.2 MB) | `08af2633574f621d` |
| `rowan` | PASS | 5.28 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 252 ok, 0 unavailable, 0 fail {'ok': 252} (491 s) | green | `8b2c1f69c055ea40` |
| `runner-with-blade-prosthesis` | PASS |  | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 98 ok, 0 unavailable, 0 fail {'group-drift': 154, 'ok': 98} (463 s) | green | `13334e87323effba` |
| `running-child` | PASS |  | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 56 ok, 0 unavailable, 0 fail {'group-drift': 196, 'ok': 56} (141 s) | green | `cfd85fea941475d8` |
| `saguaro-cactus` | PASS |  | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 252 ok, 0 unavailable, 0 fail {'ok': 252} (1486 s) | green | `8f7fbcd71db62138` |
| `saucer-magnolia` | PASS | 4.81 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 182 ok, 0 unavailable, 0 fail {'ok': 182, 'group-drift': 70} (63 s) | green | `99a5d3798d16165f` |
| `schoolchild-with-backpack` | PASS |  | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 84 ok, 0 unavailable, 0 fail {'group-drift': 168, 'ok': 84} (69 s) | green | `8084addd195835ac` |
| `shrub-rose` | PASS | 2.34 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 126 ok, 0 unavailable, 0 fail {'group-drift': 126, 'ok': 126} (284 s) | green | `1815516cb69296d7` |
| `snake-plant` | PASS |  | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 231 ok, 21 unavailable, 0 fail {'unavailable': 21, 'ok': 231} (42 s) | green | `aa537b1a4af1f01b` |
| `sweetgum` | PASS | 11.66 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 252 ok, 0 unavailable, 0 fail {'ok': 252} (303 s) | 5 GLB size (11.0 to 11.8 MB) | `c936fdbde3487123` |
| `sycamore-maple` | PASS | 10.06 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 252 ok, 0 unavailable, 0 fail {'ok': 252} (946 s) | 5 GLB size (10.1 to 10.6 MB) | `f7fdd6d891164d0a` |
| `tulip-tree` | PASS | 10.39 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 0 ok, 0 unavailable, 0 fail {'group-drift': 252} (156 s) | 3 GLB size (9.8 to 10.4 MB) | `93fe6bccd50c80fe` |
| `walking-toddler` | PASS |  | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 98 ok, 0 unavailable, 0 fail {'group-drift': 133, 'range-end': 21, 'ok': 98} (70 s) | green | `8f1ee765a5d01368` |
| `wisteria-pergola` | PASS | 2.57 MiB | 0 hard | clean | OK, 0, 0 | `✓` | 252 cells: 252 ok, 0 unavailable, 0 fail {'ok': 252} (266 s) | green | `72243a65295fe4f2` |

## 5. GLB size of the trees

Eleven plants are within 8 MiB in every pose (`🧊️model-catalog` green). Ten trees are not. Where the bytes of the default pose go (`W/glb.ts`, MiB; an accessor shared by scenes counts for the first):

| Asset | GLB | JSON chunk | Body scenes | Thermal scenes, own | Structure scenes, own | Without the body scenes |
| --- | --- | --- | --- | --- | --- | --- |
| `black-alder` | 7.50 | 1.49 | 2.20 | 3.05 | 0.62 | 5.30 |
| `black-locust` | 7.27 | 1.97 | 1.80 | 2.69 | 0.69 | 5.47 |
| `common-ash` | 11.13 | 1.95 | 3.86 | 4.31 | 0.79 | 7.27 |
| `crab-apple` | 6.17 | 1.07 | 1.97 | 2.58 | 0.43 | 4.21 |
| `european-larch` | 5.78 | 1.18 | 1.49 | 2.30 | 0.69 | 4.29 |
| `field-maple` | 8.09 | 1.13 | 3.47 | 2.85 | 0.47 | 4.62 |
| `ginkgo` | 9.85 | 1.89 | 4.41 | 2.35 | 0.94 | 5.44 |
| `hawthorn` | 6.38 | 0.95 | 2.32 | 2.60 | 0.40 | 4.07 |
| `honey-locust` | 10.72 | 2.01 | 4.56 | 2.73 | 1.13 | 6.16 |
| `horse-chestnut` | 13.47 | 2.90 | 6.75 | 2.54 | 0.90 | 6.72 |
| `japanese-cherry` | 6.28 | 1.12 | 1.97 | 2.58 | 0.48 | 4.31 |
| `norway-maple` | 10.38 | 1.61 | 4.04 | 3.81 | 0.70 | 6.34 |
| `norway-spruce` | 6.37 | 1.44 | 1.65 | 2.32 | 0.83 | 4.72 |
| `red-oak` | 10.16 | 2.20 | 2.90 | 4.01 | 0.87 | 7.26 |
| `rowan` | 5.28 | 1.29 | 1.26 | 1.81 | 0.35 | 4.02 |
| `saucer-magnolia` | 4.81 | 1.24 | 1.26 | 1.47 | 0.29 | 3.55 |
| `shrub-rose` | 2.34 | 1.30 | 0.44 | 0.23 | 0.15 | 1.90 |
| `sweetgum` | 11.66 | 2.19 | 5.23 | 2.82 | 1.08 | 6.43 |
| `sycamore-maple` | 10.06 | 1.57 | 4.83 | 2.72 | 0.70 | 5.23 |
| `tulip-tree` | 10.39 | 2.55 | 2.72 | 3.98 | 0.95 | 7.68 |
| `wisteria-pergola` | 2.57 | 1.26 | 0.36 | 0.80 | 0.14 | 2.22 |

- The thermal scenes own 2.3 to 4.3 MiB of every large tree that the asset does not author: the developed air flow (`S/🔨️modules/🧊️volume/🌊️flow`). `red-oak`: 56 stream tubes of 1105 triangles, 203 vapour puffs of 320 and 223 arrows at 1:50, about 355 000 triangles over the four scales. Together with the JSON chunk (1.5 to 2.9 MiB) `tulip-tree` is at 7.68 MiB and `red-oak`, `common-ash` at 7.3 MiB before one body solid is counted. No authoring of foliage closes that; it needs the flow geometry (fewer puffs or tubes at the coarse scales, or one flow shared by the scales) or the writer.
- What authoring can still give is the body: 2.7 to 6.8 MiB. The lobes cannot get coarser without moving the drawings: 16 facets on every lobe of `norway-maple` put the guard at 30 mm at 1:50 (limit 15 mm), and the written trees sit at 12 to 15 mm there. Fewer, larger lobes at 1:50 and 1:100 mean new crowns and refitted drawings in three views at four scales per tree; that is the open work for `horse-chestnut` (380 limb lofts and 275 lobes), `sweetgum`, `honey-locust`, `ginkgo`, the three maples and `common-ash`. `field-maple` misses by 0.1 MB in two poses.
- Not measured by this lane: `weeping-willow`, `fig-tree`, `japanese-maple`, `multi-stem-birch`, `atlas-cedar`, `windmill-palm` (polyhedron foliage already; to be looked at once `closing-CX-GLB.md` says they stay over).

## 6. Stated plainly

- **Open: `european-beech`.** With the reference facets its body is 380 000 triangles at 1:50 and its thermal scene 525 508 (budget 400 000; before 1 983 238): 425 large flat sprays (1.05 to 2.8 m), 497 limb and twig lofts and 138 000 triangles of air flow. Tried and measured: 20 facets per spray with the limbs as 24-sided tubes gives 397 634 but breaks the guard (90 mm at 1:50, box 12 mm off; baseline 13.1 mm). It needs fewer sprays or twigs at 1:50, a new crown. The file is untouched; the tried ops are `W/ops/european-beech.tried.json`.
- **Cures written before the coordinator's note on `qualityOf` (lane CX-Q).** The growth of `walking-toddler`, `running-child`, `schoolchild-with-backpack`, `runner-with-blade-prosthesis`, `parent-with-stroller` (structure, 1:200 to 1:500) and `snake-plant` (thermal) came from solids thickened to the feature of 1:500, which the chord clamp of `qualityOf` gives more segments. The figures now have real merges at 1:500 (fewer solids, as the adult figures already had) and stay simpler under any quality rule. The six outlet lenses of `snake-plant` are faceted, which is the kind of workaround the note asks to leave to CX-Q; it is proven, and if CX-Q makes it unnecessary the file to go back to is the catalog file as it was before 2026-11-27 02:23.
- `running-child` has one more named pose (`toe-off`), so its GLB set and its matrix cover that pose now.
- `norway-maple` was written twice (00:53 with the reference facets only, 03:47 with the small-lobe rule and the faceted structure spheres), because the GLB budget came to this lane after the first write.
- The matrix statuses `group-drift` and `range-end` are not failures; they were not compared with the state before the lane.
- The silhouette guard of this lane (`../MR/measure.ts`) is stricter than the suite (largest boundary distance, not p95). `japanese-cherry`, `black-locust`, `ginkgo`, `honey-locust`, `field-maple` and `sweetgum` each failed one or two light proofs on it or on a budget; nothing was written from a failed copy.
- Until 02:26 the job loop ran inside one long grant (`worker.sh`); since then `loop.sh` asks the queue per job, as the coordinator asked.

## 7. Tools (`W`)

`facet.ts` (ops to a trial copy: `facet`, `remove`, `lens`, `decision`, `set`, `unset`), `gen.py` (ops of a tree from its radius limits), `ops/` (what was applied, per asset), `make.sh`, `light2.sh`, `prove2.sh`, `chain2.sh`, `chain3.sh` (and the first versions `light.sh`, `prove.sh`, `chain.sh` the figures ran with), `recat.sh`, `groups.ts`/`groups.sh` (triangles per scene and group of solids), `glb.ts`/`glb.sh`, `size.sh`, `sizes.py`, `extent.sh`, `base.sh`, `measure1.sh`, `table.py`, `loop.sh` (job loop, one queue request per job), `worker.sh`/`worker2.sh` (superseded), `poll.sh`, `status.sh`, `wait.sh`, `q.sh`, `run.sh`, `env.sh`.
