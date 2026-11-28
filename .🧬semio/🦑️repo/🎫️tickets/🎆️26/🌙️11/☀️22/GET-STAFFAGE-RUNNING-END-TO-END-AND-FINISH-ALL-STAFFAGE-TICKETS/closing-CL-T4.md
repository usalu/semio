FINAL (2026-11-27 21:25: `european-beech`, `parent-with-child-on-shoulders` and `person-with-guide-dog` rewritten and proven; each has a green single-asset `🧊️model-catalog` run, 0 failing matrix cells, clean gate, scan, guard, three-views sweep and check CLI; every pose GLB is at most 7.61, 7.27 and 6.89 MiB; the beech thermal scene is 187,504 triangles; no test, budget or rule touched)

# 👑️ Closing Lane CL-T4: Beech Crown and Two Figures

Paths: `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `U` = this ticket folder, `W` = `U/🧰️matrix/CL-T4` (tools and ops, kept), `GEN` = `U/🗑️generated/matrix/CL-T4` (deleted with this report; every number below was read there in a log of a run that ended by itself). Every measuring run went through `repo capacity run` (`W/loop.sh`, `W/q.sh`), at most two requests at a time.

## 1. Result

| Asset | Before | After | Digest before -> after |
| --- | --- | --- | --- |
| `european-beech` | body 1,839,850 / 648,412 / 239,726 / 72,630 triangles (1:50, 1:100, 1:200, 1:500); thermal 1,961,930 at 1:50 (budget 400,000); structure 259,130 / 153,500 / 100,928 / 67,686; GLB 36.7 MB | body 91,424 / 56,930 / 34,340 / 18,268; thermal 187,504 / 141,666 / 101,044 / 46,172; structure 88,360 / 69,774 / 59,202 / 48,830; GLB per pose 7.37, 7.34, 7.20, 7.54, 7.33, 7.61 MiB (largest `mast-year`) | `dd64681657b54a2b` -> `ad0bb391069b7471` |
| `parent-with-child-on-shoulders` | structure 215,848 / 118,622 / 53,890 / 33,012; GLB 9.3 to 9.4 MB in the brief, 8.72 MiB in the default pose with the writer of 14:37 | structure 116,292 / 74,560 / 45,936 / 29,364; GLB 7.19, 7.27, 7.25 MiB (`shoulder-carry`, `child-waving`, `heel-strike-l`) | `a9a54766faa978e8` -> `e88299e1b85b073f` |
| `person-with-guide-dog` | structure 210,566 / 101,562 / 54,996 / 36,634; GLB 8.1 to 8.2 MB in the brief, 7.51 MiB in the default pose with the writer of 14:36 | structure 152,630 / 81,542 / 52,140 / 36,110; GLB 6.89, 6.81, 6.89 MiB | `6cf68e773716913a` -> `817600f202824ad0` |

Body and thermal triangles of the two figures are unchanged (124,082 and 224,954 at 1:50 for the parent, 159,998 and 213,594 for the handler).

## 2. Proofs

Light proof on the trial copy before the write (`W/light2.sh`: budgets, GLB of the default pose, gate over every subject, gate over the poses, silhouette guard with `validateAsset` and the three-views sweep, check CLI `--file`) plus the GLB of every pose (`W/sizes.ts`), then the write and the proof on the catalog file (`W/prove2.sh`). Each catalog file equals its proven copy (digests checked at 21:22). The three lines are in `U/requests.md` (19:10, 20:56, 21:06).

| Asset | Light proof | Gate | Scan | Guard, `validateAsset`, sweep | Check CLI | Matrix (252 cells) | `🧊️model-catalog` single run |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `european-beech` | PASS | 0 hard | clean | OK, 0, 0 | `✓` | 252 ok, 0 fail (756 s) | 3 passed of 3 (678 s, second run, see 5) |
| `person-with-guide-dog` | PASS | 0 hard | clean | OK, 0, 0 | `✓` | 0 fail: 252 group-drift (230 s) | 3 passed of 3 (169 s) |
| `parent-with-child-on-shoulders` | PASS | 0 hard | clean | OK, 0, 0 | `✓` | 0 fail: 7 ok, 224 group-drift, 21 range-end (261 s) | 3 passed of 3 (184 s) |

Silhouette guard of the beech, largest boundary distance per view (elevation, front, plan), before -> after: 1:50 13.1 / 13.1 / 13.1 -> 13.7 / 13.1 / 13.1 mm (limit 15); 1:100 26.3 / 26.3 / 26.3 unchanged; 1:200 33.5 / 37.5 / 37.5 unchanged; 1:500 37.5 / 37.5 / 41.9 unchanged. IoU and bounding box unchanged to the digit. No drawing was touched.

## 3. Cure of the beech

The drawings stay, because the outline of the tree in the three views does not move. `W/crown.ts` works on the mesh the guard itself measures (default pose, reference facets):

- Per view, inner lobes first, a lobe whose outline lies inside the union of the lobes that still carry that view stops carrying it; the union of the carriers stays the union of all. Carriers at 1:50: elevation 215, front 204, plan 124 of 425 lobes; 105 lobes carry no view (1:100: 146 / 142 / 86 of 259; 1:200: 92 / 81 / 66 of 134; 1:500: 43 / 44 / 35 of 61).
- Of a carrier's outline only the part that shows counts: a run of outline corner points whose cap lies inside the other carriers is left out (6493, 5142, 3489, 1936 points at the four scales).
- Each lobe becomes the convex polyhedron (`W/hull.ts`) of chosen corner points of its own reference facets: its showing outline points in the views it carries, the corner points of the same ellipsoid at 8 facets for the other sides, and under a transpiration outlet every point within 1 m of it. It shows the reference outline exactly where it counts and lies inside its ellipsoid everywhere else. 52.9 corner points per lobe at 1:50 (about 102 triangles instead of 3968; the handcrafted oaks and limes of the catalog have 180 to 300).
- Wood: twigs are tubes of 6 facets (13 to 16 mm radius, 0.3 mm on paper at 1:50), side branches of 12, limbs of 24 (their reference facets). This is the 0.6 mm the elevation guard moved at 1:50.
- Structure: joint spheres as faceted spheres of 12, limb members as tubes of 12. Outlets at their reference facets.
- One decision (en, de) added to the volumes of every scale. Ops: `W/ops/european-beech.json` (crown part `european-beech.crown.json`, the rest `european-beech.base.json`, text `european-beech.text.json`).

Measured on the way: reference facets on every lobe and limb alone give 294,070 body and 401,158 thermal triangles at 1:50 (the air flow is 99,696 there, not 66,528) and a GLB of 11.79 MiB; without leaving out covered outline points 8.34 MiB.

## 4. Cure of the two figures

Measured first (`W/own2.ts`, bytes by mesh group; `W/parts.py`, triangles by part). Parent at 1:50: structure 1.75 MB of the binary, body 1.34 MB, thermal 0.98 MB; of 215,848 structure triangles the members are 135,798, moment arcs and other glyphs 37,912 (one shared mesh, few bytes), joints 23,882, masses 18,256.

- **Merged (the cure the brief names).** `W/within.ts` tests every joint and mass solid in every named pose: enclosed means every vertex and facet centre, pushed out 2 %, lies inside a member. Parent: 11, 9, 6, 10 solids at the four scales (for example `joint-thigh-l` in the acetabulum, `joint-head` and `mass-head` in the cranium). Handler and dog: 3, 2, 1, 0. They are removed. Enclosed solids show from no side, and the axonometric depiction removes hidden surfaces exactly, so no structure drawing changes.
- **This alone is far too little**: it took the parent from 8.72 to about 8.4 MiB. Most joint spheres stand out of their members (for example `joint-shank-l`: 56 of 794 probes outside, `joint-forearm-l`: 377) and are visible knobs; removing them would change what the structure views show, so they stay.
- **Faceted instead (my addition).** The other joint and mass knobs are faceted bodies of 12 facets around; ellipsoid bone masses of the members are 12 or 16 facets where that stays within 0.04 mm of paper of the ellipsoid at the scale (parent 65+2, 61, 12, 6; handler 33+1, 22, 7, 1); short round bones of at most 0.3 mm of paper across are prisms of 8 facets (parent 72, 13, 4, 0; handler 73, 13, 0, 1); the parent's 36, 24 and 10 rib arcs are swept in steps of 1/16 of a turn. Long shafts (`slender`), lofts, extrusions stay as they were. A solid of just the smallest feature of its scale stays smooth: the first trial of the handler failed `volumes-feature` on six such knobs (29.4 mm against 30 mm) and nothing was written from it.
- One decision (en, de) per scale states the counts. Ops: `W/ops/<id>.json`, made by `W/knobs.py` from `W/ops/<id>.within.json`.

## 5. Stated plainly

- **"By themselves" is met for the beech and the handler, not fully for the parent.** The shared writer changed while I worked (the parent's untouched file went from 9.27 MiB at 14:20 to 8.72 MiB at 14:37). My asset change alone is worth 1.53 MiB on the parent's default pose (8.72 -> 7.19) and 0.62 MiB on the handler's (7.51 -> 6.89). Taken off the brief's figures that is about 7.86 MiB for the parent (under 8, margin 0.14, not 0.3) and 7.56 MiB for the handler. With the writer as it is tonight the margins are 0.73 and 1.11 MiB.
- **The beech sits 0.39 MiB under the budget in its largest pose**, and 3.2 MB of its GLB is the JSON chunk (1895 meshes, a record per solid). If lane CX-JSON's cut were reverted it would be the first to go over.
- **The beech's first `🧊️model-catalog` run crashed**, it did not fail: the vitest worker exited after 1845 s inside a 2500 MB grant with no finding. Run again alone with 3600 MB it passed 3 of 3; the largest process was 3459 MB. A shard that holds the beech needs that much.
- **The 3D look from oblique sides is coarser**: lobes hidden in all three views are 8-facet bodies, and a carrier is exact only in the views it carries. The guard does not see this; it is a modelling decision forced by the two budgets, written into the asset's decisions.
- In a pose that moves the limbs apart, a lobe whose outline part was covered in the default pose can show an edge up to a few centimetres inside the drawn scallop. The guard and the matrix (all six poses, 252 ok) did not object.
- The catalog files grew, because polyhedra are written out: beech 5.2 -> 10.2 MB, handler 7.1 -> 11.3 MB, parent 4.5 -> 5.6 MB.
- Not run: the other staffage suites on these three assets (`🧊️model-skeleton`, `🧊️catalog-silhouette`, `🌡️catalog-physics`, `🧊️model-digests`). The faceted finger and toe bones and vertebrae are polyhedra now; a case of `🧊️model-skeleton` that reads capsule radii of those small bones would notice. For MZ: re-run these for the three assets.

## 6. Tools (`W`)

`crown.ts`/`crown.sh` (outline carriers and corner points of a tree), `hull.ts` (convex hull in space), `facet.ts` (CL-T1's tool with the ops `keep` and `inscribed`), `within.ts`/`within.sh` (enclosed structure solids), `knobs.py` (ops of a figure), `own2.ts`/`own2.sh` (GLB bytes by mesh group), `sizes.ts`/`sizes.sh` (GLB of every pose against 7.7 MiB), `parts.py`, `groups.ts`, `glb.ts`, `join.py`, `beech.sh`, `trial.sh`, `try.sh`, `chain.sh`, `make.sh`, `light2.sh`, `prove2.sh`, `recat.sh`, `bundle.sh`, `loop.sh`, `q.sh`, `run.sh`, `env.sh`, `ops/`, `bundles/`.
