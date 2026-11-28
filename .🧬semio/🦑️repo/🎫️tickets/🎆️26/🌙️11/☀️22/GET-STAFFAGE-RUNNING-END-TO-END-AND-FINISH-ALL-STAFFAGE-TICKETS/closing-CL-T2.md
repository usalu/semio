FINAL (2026-11-27 02:49: 30 assets written and clean with every proof, `🧊️model-catalog` green for each as the only selected asset; the 15 named assets and 15 more of the same kinds found in the shards still running; one test pin moved, nothing weakened; nothing open in the lane)

# 🧊️ Closing Lane CL-T2: Self-Intersections, Watertight Solids and Triangle Budgets

Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `U` = this ticket folder, `W` = `U/🧰️matrix/CL-T2`, `M` = `U/🧰️matrix/MR`, `GEN` = `U/🗑️generated/matrix/CL-T2` (deleted with this report; every number below was read there in a log of a run that ended by itself). Every measuring run went through `repo capacity run` (`W/q.sh`), at most two requests at a time. No budget, schema limit, rule, engine, oracle or manifest was edited; one pin of `🧪️tests/🧊️model-bounds` was moved to an equally strict form (section 4).

## 1. Result

Before: the geometry findings of `auditGeometry`, the code of `🧊️model-catalog` (`W/geo.ts`, read at 00:39 for the 15 named assets and at 01:13 for the 100 assets of the lane's categories in the shards 04, 06, 08, 10, 12, 13, 14). After: 0 geometry findings for each of the 30, and the real suite green with the asset as the only selected one. Structure triangles are those of the four structure scenes at the quality of their scale; `a→b` marks a scene that changed.

| Asset | Finding before | Catalog file | Structure triangles 1:50 / 1:100 / 1:200 / 1:500 | Matrix of the written file |
| --- | --- | --- | --- | --- |
| `ceiling-diffuser` | structure grows 1:100 2924 → 1:200 3212 | `3f0ca95d17a1bae7` → `251b9a3865787fd0` (172954 bytes), 6 changes, written 01:17 | 3982 / 2924 / 3212→1704 / 2500→1552 | 231 ok, 21 unavailable, 0 fail |
| `etics-brick-wall-12` | 1 × not watertight (1:200) | `0b56beac8707d967` → `bf182b78f7a7495e` (163781 bytes), 6 changes, written 01:41 | 5852 / 3736 / 2896 / 2762 | 252 ok, 0 unavailable, 0 fail |
| `etics-brick-wall-30` | 1 × not watertight (1:200) | `da0a91545e98dc44` → `bf931a538c2603d8` (163573 bytes), 6 changes, written 01:41 | 5852 / 3736 / 2896 / 2762 | 252 ok, 0 unavailable, 0 fail |
| `gas-boiler` | ModelInvalidError: self-intersection at /solids/223/profile; behind it: structure grows 1:200 2348 → 1:500 2400 | `a5d8cff96c970b22` → `73998dcacecb6ee1` (336357 bytes), 4 changes, written 01:33 | 7532 / 3642 / 2348 / 2400→1684 | 231 ok, 21 unavailable, 0 fail |
| `height-adjustable-desk` | structure grows 1:200 7294 → 1:500 7832 | `e82cce3d6c64cb7c` → `a37461eb4539b855` (359145 bytes), 11 changes, written 01:33 | 15974 / 11048 / 7294 / 7832→4316 | 196 ok, 0 unavailable, 0 fail |
| `kitchen-extractor-hood` | structure grows 1:100 5202 → 1:200 5620 | `cd4dbb462b5b8e8b` → `fb52eecff1326abb` (323259 bytes), 3 changes, written 01:18 | 9150 / 5202 / 5620→4112 / 3692 | 119 ok, 21 unavailable, 0 fail |
| `picnic-table` | structure grows 1:200 5194 → 1:500 5686 | `15cc6fa31031170b` → `40436c50048ce848` (393646 bytes), 7 changes, written 01:48 | 12534 / 9550 / 5194 / 5686→4036 | 84 ok, 0 unavailable, 0 fail |
| `room-thermostat` | structure grows 1:50 2728 → 1:100 2760 | `c28197ed41478167` → `7a2631fb449ad161` (157903 bytes), 3 changes, written 01:56 | 2728 / 2760→1440 / 1276 / 1260 | 210 ok, 42 unavailable, 0 fail |
| `shower-head` | ModelInvalidError: self-intersection at /solids/28/profile; self-intersection at /solids/63/profile | `052b452fdcb09441` → `0ab95a7ca041b951` (356928 bytes), 1 changes, written 01:17 | 13900 / 9018 / 6994 / 5542 | 147 ok, 21 unavailable, 0 fail |
| `solid-brick-wall-1960` | 2 × not watertight (1:200) | `e98dec33025718df` → `abf614929cf0c115` (145653 bytes), 9 changes, written 01:17 | 8848 / 3100 / 2956 / 2762 | 252 ok, 0 unavailable, 0 fail |
| `tilt-window` | structure grows 1:200 1948 → 1:500 2016 | `a1671783a4860101` → `f79a22ce7588eca7` (257312 bytes), 3 changes, written 01:49 | 4508 / 3096 / 1948 / 2016→1588 | 252 ok, 21 unavailable, 0 fail |
| `used-baby-grand-piano` | structure grows 1:200 9176 → 1:500 9248 | `7f66a2fdaf00b98c` → `85c0f0165c0f59a3` (963850 bytes), 9 changes, written 01:34 | 17350 / 13812 / 9176 / 9248→6938 | 252 ok, 0 unavailable, 0 fail |
| `used-backrest-park-bench` | structure grows 1:200 8354 → 1:500 11242 | `644457b64037ad5f` → `6c16341eeb1a7ad6` (239129 bytes), 13 changes, written 01:48 | 16030 / 12176 / 8354 / 11242→5944 | 252 ok, 0 unavailable, 0 fail |
| `used-bathtub` | structure grows 1:200 4402 → 1:500 5038 | `ba0702279bd3d97e` → `b1ffcacc64e7df73` (326780 bytes), 6 changes, written 01:49 | 10606 / 6276 / 4402 / 5038→3278 | 252 ok, 0 unavailable, 0 fail |
| `used-bicycle-shelter` | 18 × not watertight (1:100, 1:200, 1:500) | `f03d873e01d3ad68` → `d0b7ac4274d0097d` (768606 bytes), 18 changes, written 01:43 | 113240 / 69744→68592 / 45628→45052 / 29100→28524 | 231 ok, 0 unavailable, 0 fail |
| `used-chess-table` | structure grows 1:200 3086 → 1:500 3676 | `b64d8a57d9521613` → `e9fac61afa47d646` (114282 bytes), 7 changes, written 01:18 | 7424 / 5170 / 3086 / 3676→2296 | 252 ok, 0 unavailable, 0 fail |
| `used-electric-guitar-and-combo` | structure grows 1:200 9034 → 1:500 9286 | `40fea58df6c0be38` → `dae4e00b20596b20` (438989 bytes), 5 changes, written 01:51 | 19676 / 10480 / 9034 / 9286→6442 | 252 ok, 0 unavailable, 0 fail |
| `used-extendable-dining-table` | structure grows 1:200 7280 → 1:500 7844 | `15df5603af05634f` → `bb35135e5a5bfffe` (205013 bytes), 9 changes, written 01:56 | 14066 / 10888 / 7280 / 7844→4988 | 252 ok, 0 unavailable, 0 fail |
| `used-forced-draught-oil-burner` | 3 × not watertight (1:50) | `7c1bfeafa66198c1` → `a2c1093d2f96074f` (308572 bytes), 6 changes, written 01:32 | 15526 / 8462 / 7378 / 6466 | 231 ok, 21 unavailable, 0 fail |
| `used-park-armchair` | structure grows 1:200 11344 → 1:500 16352 | `e1665198b9bee962` → `dbb1a03534c3506d` (346352 bytes), 18 changes, written 01:36 | 21096 / 15026 / 11344 / 16352→4896 | 0 ok, 0 unavailable, 0 fail |
| `used-pump-drinking-fountain` | structure grows 1:200 5970 → 1:500 6014 | `01d7430f506a5238` → `2d7a552ef2c4dabf` (299648 bytes), 9 changes, written 01:37 | 13880 / 10062 / 5970 / 6014→3704 | 252 ok, 0 unavailable, 0 fail |
| `used-recliner-chair` | structure grows 1:200 8934 → 1:500 11492 | `50acde1068b869f8` → `abb398deae81d56f` (293883 bytes), 9 changes, written 01:38 | 16688 / 9410 / 8934 / 11492→7880 | 252 ok, 0 unavailable, 0 fail |
| `used-recycling-bin-cluster` | structure grows 1:200 9386 → 1:500 12368 | `db071c464408c221` → `fbcc7e7ca0f787a8` (446538 bytes), 14 changes, written 01:53 | 20236 / 13364 / 9386 / 12368→6440 | 252 ok, 0 unavailable, 0 fail |
| `used-tree-surround-bench` | structure grows 1:200 16412 → 1:500 19194 | `101d39f984fb50df` → `42e1e7c598b5ff16` (928344 bytes), 38 changes, written 01:39 | 36396 / 30852 / 16412 / 19194→7314 | 182 ok, 0 unavailable, 0 fail |
| `used-tv-sideboard` | structure grows 1:200 5086 → 1:500 5316 | `0a64b3ccfd094700` → `3a16476613a1d8be` (196219 bytes), 6 changes, written 01:55 | 9840 / 7226 / 5086 / 5316→3996 | 42 ok, 0 unavailable, 0 fail |
| `used-upright-piano` | structure grows 1:200 8638 → 1:500 9756 | `5d26f32a24f3fb77` → `ca2e6663e07193ac` (409782 bytes), 11 changes, written 01:40 | 14132 / 10298 / 8638 / 9756→5904 | 84 ok, 0 unavailable, 0 fail |
| `used-wall-unit` | 12 × not watertight (1:50, 1:100, 1:200, 1:500) | `d9d98d18172043f1` → `4682e2a8765c24e8` (384977 bytes), 24 changes, written 01:42 | 22154 / 16658 / 13756 / 11082 | 70 ok, 0 unavailable, 0 fail |
| `used-washing-machine` | structure grows 1:200 7610 → 1:500 7766 | `1d5cdbe5088aa4c8` → `76a6480f25842bba` (278594 bytes), 5 changes, written 01:50 | 13868 / 9764 / 7610 / 7766→4922 | 210 ok, 0 unavailable, 0 fail |
| `used-writing-desk` | structure grows 1:200 7698 → 1:500 8596 | `771f9254086ce505` → `c053ffbd3aceae5f` (159670 bytes), 9 changes, written 01:18 | 16576 / 9272 / 7698 / 8596→5600 | 84 ok, 0 unavailable, 0 fail |
| `used-xylophone` | structure grows 1:200 11638 → 1:500 14404 | `2bf02d678780929d` → `e154e9d2894c4963` (425220 bytes), 22 changes, written 01:54 | 22548 / 14204 / 11638 / 14404→7444 | 252 ok, 0 unavailable, 0 fail |

The first 15 lines of the task are `shower-head`, `gas-boiler`, `solid-brick-wall-1960`, `used-forced-draught-oil-burner` and the eleven budget assets. The other 15 were found by the scan of the shards still running: `picnic-table`, `used-backrest-park-bench`, `tilt-window`, `used-bathtub`, `used-washing-machine`, `used-electric-guitar-and-combo`, `used-recycling-bin-cluster`, `used-xylophone`, `used-tv-sideboard`, `room-thermostat`, `used-extendable-dining-table` (budget), `used-wall-unit`, `used-bicycle-shelter`, `etics-brick-wall-12`, `etics-brick-wall-30` (watertight). The other 85 assets of that scan have 0 geometry findings.

## 2. What was changed and why (`W/fix.ts <id>`, the repairs in `W/repairs.ts`, idempotent)

### Self-intersection: two revolve profiles that were drawn as loops and not closed

- `shower-head` 1:50 `face-zone-ring` (and its thermal reference): the profile `[[0.011, 0.027], [0.0128, 0.0275], [0.0128, 0.0305], [0.011, 0.031]]` is open, so the kernel closes it through the axis (rule A4) and both ends drop along the same line. The first point is repeated: the ring of 27 to 31 mm radius that the name and the neighbours (`face`, `face-centre`) show.
- `gas-boiler` 1:200 `th-hx`: `[[-0.14, 0.0365], [-0.14, 0.115], [0.043, 0.115], [0.043, 0], [0.01, 0], [0.01, 0.0365]]`, the drum with the 73 mm burner bore; open, its last edge folds back onto the drop to the axis. The first point is repeated, as `th-hx-shell` of 1:100 is written.
- Behind the error of `gas-boiler` stood a budget finding that the exception had hidden (structure 2348 at 1:200, 2400 at 1:500); cured with it (below).

### Not watertight: copies of one solid that share a face

- `solid-brick-wall-1960`, `etics-brick-wall-12`, `etics-brick-wall-30`, 1:200: six bond bands of 167 mm as one box repeated at a pitch equal to its height. They alternate as `bond-band-a` and `bond-band-b` (three copies each at a pitch of 333 mm, the same six bands in the same places), as the courses of 1:100 alternate as `course-a` and `course-b`; the structure member of the brick wall likewise (`member-bond-band-a`, `-b`). The decision of each says so.
- `used-forced-draught-oil-burner` 1:50: four floor boards of 96 mm at a pitch of 96 mm. They alternate as `floor-boards` and `floor-boards-b` (two copies each at 192 mm) with their thermal and structure references. The decision said "four 93 mm floor boards with 4 mm gaps", which the solids never were; I first modelled exactly that (93 mm, pitch 97 mm): geometry findings 0, but the silhouette guard failed (plan 1:50: boundary distance 114.4 mm against the gate of 15 mm, because the plan drawing is one closed floor), so nothing was written, and the decision now names what is modelled ("four 96 mm floor boards laid tight as two alternating pairs").
- `used-wall-unit`, all four scales: three blankets of 0.9 m laid end to end as one extrusion repeated at 0.9 m. They are the outer pair (`blankets…`, two copies at 1.8 m) and the middle one (`blankets-mid…`), with a thermal and a structure reference each.
- `used-bicycle-shelter` 1:100, 1:200, 1:500: eight, four and four spoke capsules per wheel that all began in the axle point, where the end caps of opposite spokes share their rim. They are four, two and two rods through the axle (the same spokes in the same places, half as many solids, no cap in the axle). 1:50 was clean (its spokes begin on the hub flange).

### Triangles that grow: markers of one feature

In all 23 budget assets the growth sits in the `mass` markers (in `used-park-armchair`, `used-backrest-park-bench` and `used-recliner-chair` also in the joint markers); glyphs and members fall in sum at the failing step. Example `used-park-armchair`, 1:200 → 1:500: glyphs 4680 → 4040, members 904 → 664, 12 joint ellipsoids 3456 → 8 with 5824, 8 mass ellipsoids 2304 → 5824. The cause is measured in section 5.

- The cure, the one the catalog already uses elsewhere (`used-sewing-machine` 1:500, 122 box masses at 1:500): at the scale that grows, every ellipsoid `mass` marker is a block with the same centre and the same extents (12 triangles). At 1:500: `height-adjustable-desk`, `used-baby-grand-piano`, `used-chess-table`, `used-pump-drinking-fountain`, `used-recliner-chair`, `used-tree-surround-bench`, `used-upright-piano`, `used-writing-desk`, `picnic-table`, `tilt-window`, `used-bathtub`, `used-washing-machine`, `used-electric-guitar-and-combo`, `used-recycling-bin-cluster`, `used-xylophone`, `used-tv-sideboard`, `used-extendable-dining-table`, `gas-boiler`.
- `used-park-armchair` 1:500: the eight joint ellipsoids too (blocks, as the joints of `used-baby-grand-piano` and `used-chess-table` are).
- `used-backrest-park-bench` 1:500: the masses, and the six bolt and dowel capsules as cylinders of the same axis and extents (a revolve from cap end to cap end): 4560 → 912 triangles for the twelve copies.
- `ceiling-diffuser`: the one mass marker at 1:200 (where it grew) and at 1:500 (which would otherwise grow against the new 1:200).
- `kitchen-extractor-hood` 1:200: only `mass-fan-unit`, which is a block at 1:500 already; the wheel mass stays an ellipsoid at both scales.
- `room-thermostat` 1:100: the knob mass, a 15 mm ball of 1332 triangles (the whole structure scene of 1:50 has 2728), is a 15 mm block; the board mass beside it was a block already.
- Every decision that names the structure says it in both languages; where a text said "ball" (`gas-boiler` 1:500, `room-thermostat` 1:100) the word is replaced.

No body and no 2D shape was changed in any budget asset; the silhouette guard reads the same numbers.

## 3. Proofs of the written files (each of the 30)

| Proof | Result |
| --- | --- |
| Light proof on the trial copy BEFORE the write (`W/light.sh`: gate over every subject and over the poses, guard, `validateAsset`, sweep, check CLI `--file`, geometry findings 0) | 30 PASS; one FAIL before (the first oil-burner copy, section 2), not written |
| `fix.ts --write` twice | second run 0 changes, 30 of 30 |
| Catalog file against the trial copy | byte for byte, 30 of 30 |
| Gate `M/gate.ts --soft`, authors' scan `T/🔎️scan.ts` | exit 0 and 0 hard findings, 30 of 30 |
| Prints (`../MC/prints.ts`) | every authored scale prints, 29 of 30; `used-bicycle-shelter`: 1 refusal at the range end 1:1000 (`gutter` 0.189 mm, `purlin-1` 0.152 mm against 0.2 mm), 2D shapes that this lane did not touch (not measured before the write) |
| Guard, `validateAsset`, three-views sweep (`M/measure.ts`) | OK, 0 issues, 0 findings, 30 of 30 |
| Check CLI `--asset` | `✓`, 30 of 30 |
| Matrix `T/📊️matrix.ts --jobs 1 --fresh --assets <id>`, to its end | 0 fail cells, 30 of 30 (cells in section 1; what is not `ok` or `unavailable` is reported group drift or the range end; `used-park-armchair` is group drift in all 252 cells, 0 fail) |
| `🧊️catalog-silhouette`, `🧊️three-views`, `🌡️catalog-physics`, `✅️asset-validation` with the name filter | 1 + 2 + 2 + 10 passed, 30 of 30 (`gas-boiler` 2 + 4 + 4 + 10: its name also selects `used-wall-hung-gas-boiler`) |
| `🧊️model-catalog` with the asset as the only selected one (the command of the brief) | 3 passed of 3, 30 of 30 (01:49 to 02:46) |
| `🧊️model-bounds`, the cases that pin `gas-boiler` ("boiler flue collar") and the brick wall ("bonded masonry") | 4 passed each |

The catalog files still had the digests of section 1 when the last suite ended (compared at 02:47). The first `🧊️model-catalog` run of `shower-head` (01:49) was red with three findings that are not the asset's: `glTF-Transform: Missing required extension, "KHR_mesh_quantization"` for its three poses, at the minute another lane rewrote the GLB exporter (`🧰️framework/…/📤️export/🧊️glb/🟦️.ts`, 01:51); the same file ran green at 02:37.

## 4. The one test that was edited

`🧪️tests/🧊️model-bounds/🟦️.ts` pinned the wall of 1:200 as `parts: /^bond-band$/, count: 1, copies: 6, spans: 1` ("six bond bands that touch"), which is exactly the shape that `🧊️model-catalog` refuses: two suites asked for opposite things. The pin is now `parts: /^bond-band-[ab]$/, count: 2, copies: 6, spans: 1` (the form of its own 1:100 line: two solids, the same six copies, the same single span), the 1:500 line forbids `/^bond-band/` instead of `/^bond-band$/`, and the docstring says "six bond bands in two repeated bands that touch" (`W/wall-pin.py`, written in the same chain as the wall, 01:17). The four cases that read the wall pass.

## 5. Asset or tessellator: the reduced cases (`W/reduce.ts`, in `requests.md`)

- Watertight: one box 12 triangles, 0 boundary, 0 overlapped; the same box twice at a pitch equal to its size 0 boundary, 10 overlapped; six times 50 (the number of the wall); at a pitch 4 mm larger 0. The kernel closes every copy; the oracle welds the copies of one solid. The finding is the asset's.
- Self-intersection: both profiles are faulty open and simple with the first point repeated. The validator applies the closing rule; the profile was wrong.
- Budget: an ellipsoid of one feature in diameter at 1:50 / 1:100 / 1:200 / 1:500 has, for an asset of 0.6 m, 120 / 210 / 360 / 960 triangles, of 0.9 m 120 / 120 / 272 / 624, of 1.7 m 120 / 120 / 120 / 342, of 2.5 m 120 / 120 / 120 / 224, of 10 m 120 at every scale; a block has 12. `qualityOf` takes the chord as min(largest dimension / 3200, 2 % of the feature); below 0.96 m (1:100), 1.92 m (1:200) and 4.8 m (1:500) the first term wins, the chord stays and the marker grows. The findings are cured in the assets; the cause is shared and stays: 1270 ellipsoid masses and 1004 ellipsoid joints at 1:500 in the catalog pass only where members and glyphs shrink faster. Reported for a decision; no shared code was changed.

## 6. Stated plainly

- Blocks in place of ellipsoids are a visible change of the structure views at the coarse scale (a cube of one feature where a ball of one feature was). I rendered none of them and opened no sheet.
- A block has the extents of its ellipsoid, so its corners reach 41 % further than the ball did; `validateAsset`, gate and `volumes-connected` raise nothing for any of the 30, but whether a corner shows outside a slender member in a view was not looked at.
- `used-recliner-chair` keeps its four capsule joints at 1:500 (2316 triangles, 1036 at 1:200) and `kitchen-extractor-hood` its ellipsoid wheel mass (1088 at 1:200, 960 at 1:500): the sums fall, so the suite passes; they are not simpler solids than at the finer scale.
- Inside the lists, markers still grow between finer scales in several assets (for instance the masses of `used-bathtub`, 624 at 1:100 and 740 at 1:200); the suite judges the sum of a scene, and I changed only the scale whose sum grew (and `ceiling-diffuser` 1:500 with it).
- The a/b split of touching copies changes no triangle: the same boxes stand in the same places and still touch; what changed is that no single solid has copies with a common face. A cut through two touching solids still shows the joint line it showed before.
- The decision of the oil burner was corrected to the geometry, not the geometry to the decision.
- The scan covered the furniture, appliances, objects, components, machinery and vehicles of the seven shards that were still running (100 assets). Humans, animals and plants were not scanned (`walking-toddler`, `running-child`, `runner-with-blade-prosthesis`, `parent-with-stroller`, `snake-plant`, `saguaro-cactus` and the trees stand in the issue list or the logs with budget findings of other lanes). Assets of the finished shards without a finding were not measured again.
- The logs of the shards that tested an asset before it was written still list its finding (shard 04, landed 01:03: `picnic-table`, `used-backrest-park-bench`, `used-wall-unit`).
- The before state of the 15 scanned assets is the count of `W/geo.ts`, not a red run of the suite; the whole suites without a filter were not run.
- Waiting (the longest `waiting Ns` line of each request): the first probe 16 minutes, the trial copies 26, the scan 23, the last two suite chains 33 and 43, each behind grants that held far more than they reserved (`repo capacity status` showed 9039 MB and later 8463 MB observed against 3600 MB reserved) or reserved far more than they held (two grants of 2500 MB at 146 and 78 MB). No process of another lane was touched, none of mine was withdrawn. The last suite chain peaked at 3004 MB against its 3000 MB reservation.

## 7. Requests and notes in `requests.md`

- 30 lines `CL-T2 rewrote <asset> at <date time>` (01:17 to 01:56).
- `## closing CL-T2 — the three geometry kinds of 🧊️model-catalog reduced …` (01:50): the three reduced cases and the shared cause of the growing markers, for the owner of `S/🔨️modules/🧊️volume/⏱️job/🟦️.ts`.
- Queue figures measured here (`/usr/bin/time -l`, largest process): geometry audit of 15 assets 265 MB, of 100 assets 643 MB; light proof, write, proofs and matrix of 13 assets in one chain 773 MB; the five suites of 10 or 11 assets in one chain 2808, 2837 and 3004 MB.

## 8. Files

Updated in the catalog (repo-relative, each `…/<id>/🔣️.json` under `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/`): `🔌️appliances/` `shower-head`, `gas-boiler`, `ceiling-diffuser`, `kitchen-extractor-hood`, `room-thermostat`, `used-washing-machine`, `used-electric-guitar-and-combo`; `🧱️components/` `solid-brick-wall-1960`, `etics-brick-wall-12`, `etics-brick-wall-30`, `used-forced-draught-oil-burner`, `tilt-window`; `🛋️furniture/` `height-adjustable-desk`, `used-chess-table`, `used-park-armchair`, `used-recliner-chair`, `used-tree-surround-bench`, `used-writing-desk`, `picnic-table`, `used-backrest-park-bench`, `used-wall-unit`, `used-tv-sideboard`, `used-extendable-dining-table`; `🪑️objects/` `used-baby-grand-piano`, `used-pump-drinking-fountain`, `used-upright-piano`, `used-bathtub`, `used-recycling-bin-cluster`, `used-xylophone`, `used-bicycle-shelter`.

Updated: `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️model-bounds/🟦️.ts` (the wall pin, three lines), `requests.md` (30 lines and one section appended).

Created in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS/`: `closing-CL-T2.md`; in `🧰️matrix/CL-T2/`: `env.sh`, `q.sh`, `fix.ts` and `repairs.ts` (the repair), `geo.ts`, `geo.sh` (the geometry findings and where the triangles sit), `invalid.ts`, `invalid.sh` (the solid behind a validation path), `reduce.ts`, `reduce.sh` (the reduced cases), `try.sh`, `light.sh`, `lights.sh`, `prove.sh`, `chain.sh`, `first.sh`, `second.sh`, `suites.sh`, `mine.py`, `tell.py`, `wall-pin.py`, `wall-pin.sh`, `rows.py`, `all.txt` (the 15 named assets), `running.txt` (the 100 scanned assets).

Removed: `🗑️generated/matrix/CL-T2` (tool output, trial copies, logs) after this report was written.
