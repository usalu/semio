FINAL (2026-11-27 12:10: both bounds findings cured in shared code, no catalog file written; `🧊️model-catalog` green for `external-venetian-blind`, `leaning-person`, `laptop`, `ivy-on-trellis`; post-write proof of both assets clean, matrix 0 fail cells each)

# 🧊️ Closing 2, Lane CL-B: Two Bounds Findings of `🧊️model-catalog`

Paths: `R` = repo, `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `K` = `🧰️framework/🔨️modules/🧊️3d/🗿️model` (the kernel), `U` = this ticket folder, `W` = `U/🧰️matrix/CL-B` (tools, kept), `GEN` = `U/🗑️generated/matrix/CL-B` (deleted with this report; every number below was read there in a log of a run that ended by itself). Every measuring run went through `repo capacity run` (`W/q.sh`), at most two requests at a time.

## 1. Neither finding is new, and neither comes from a recent change

- `git log` and `git diff` (read-only): `volumes`, `rig` and `dimensions` of `external-venetian-blind` are identical in the commits of 11-22, 11-24, 11-25 and in the working tree (same hash of the 1:50 body and of the rig); the wall, the poses and `dimensions` of `leaning-person` likewise (only `depthM` 0.54 -> 0.536 on 11-24).
- The candidates named in the order are not the cause: CX-Q's chord rule (`S/🔨️modules/🧊️volume/⏱️job/🟦️.ts`, `qualityOf`) changes segment counts only; the working-tree diff of the `🧊️model-catalog` oracle before this lane touches assimp, glTF-Transform and ids, not the extent; `🧊️model-bounds` does not read these two assets' extents.
- The blind already failed with the very same line in the first pass (`🗑️generated/closing/model-catalog-04.log`, 01:03). It was missing from `modelcat-issues.txt` only because shard 04 was still running when the list was written. The kernel line at fault dates from 11-18 (`git log -L`).

## 2. `external-venetian-blind`: a kernel bug (repeat of a weighted solid)

Measured before (`W/ahead.ts`, the extent exactly as the oracle reads it): body scene 1:50 ahead -0.2700 … 0.9312 m. The meshes reaching farthest ahead are skinned ones at ahead 0.1287 … 0.9312, left 0.3987 … 0.4013: the ladder cords and lift tapes. Each is authored once at left +0.4 m with `repeat` `[0, 0, -0.8]` (the second cord at left -0.4 m). The copy stood 0.8 m in front of the facade instead of 0.8 m beside the first: 0.13 + 0.80 = 0.93, hence 1.2012 m against 0.42 m (186 %).

Cause: `tessellateSolid` (`K/🔺️tessellation/🧱️solid/🟦️.ts`) computed `copy * extra * transform`. `extra` is the rest world matrix of the node of a weighted solid, so the repeat step was applied in model space, not in the node frame as the module states ("raw geometry in the node frame, then `transform`, then the k-th repeat copy"). A rigid solid has `extra` = identity and was never affected. A staffage bone frame is turned against the neutral model frame, so the lateral step became a step ahead.

Cure: `extra * copy * transform` (two lines and the docstring). Reach: 424 solids in the whole catalog are weighted and repeated, all of them in this asset (counted over every scale and list of the 375 assets), so its GLB, FBX, model JSON and axonometric drawings change and nothing else does.

Test first, in the kernel: `K/🧫️fixtures/🦴️skinning/🔣️.json` got the vector `repeated` (a weighted box repeated three times on a node turned a quarter about +Y, expected boxes computed by hand), `K/🧪️tests/🦴️skinning/🟦️.ts` the case "repeats a weighted solid in the frame of its node", which also compares with three placing the same copies as children of that node.

Measured after: ahead -0.2700 … 0.1500 = 0.4200 m, up 2.7500, left 1.2059 in all 14 poses, miss 0.06 of the tolerance (was 23.25).

## 3. `leaning-person`: the oracle measured the wall with the figure

Measured (`W/ahead.ts`): the body scene with the wall is 0.896 x 1.900 x 1.200 m (the reported 72.3 % / 9.2 % / 123.9 %); the figure alone is 0.5160 x 1.7401 x 0.5321 m against the declared 0.52 x 1.74 x 0.536 m (-0.8 % / 0.0 % / -0.7 %). The wall is the body solid `wall` of role `ground`, 0.38 x 1.9 x 1.2 m.

So the asset is right and the oracle measured something `dimensions` do not describe. The independent references:

- the schema: `dimensions` are "the physical envelope of everything that belongs to the asset" and "exclude … ground shading";
- the 2D dimension rule of the product (`S/🔨️modules/🗂️catalog/🟦️.ts`, both extent measures filter `shape.role !== "ground"`; `🧮️evaluation` likewise), under which this asset is valid with the wall drawn in every view;
- the authoring gate the oracle names as its source (`volumes-bounds`, `bodyDimensions`), which reads 0.5160 / 1.7401 / 0.5320 for this asset at 1:50 (`W/prove.sh`, the `measure` line) — the same figure to a tenth of a millimetre.

No asset-side cure exists that keeps those three: `dimensions` that include the wall fail the 2D rule (0.52 drawn against 0.90 declared), and a figure without its wall in 3D leans on nothing.

Cure, in the test code only: `sceneExtent` (`S/🧪️tests/🧰️support/🧊️.ts`) takes a set of solid ids to leave out and measures a mesh holding one of them triangle by triangle through the solid ranges the GLB carries (`extras.solids`); `contactSurfaces` (`S/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts`) names the solids of role `ground`, and `checkGlb` passes them on. The tolerance (8 %, at least 15 mm), the pose that decides and every other check are untouched; a mesh without range data is measured whole. The feature file states the rule and has the scenario "A contact surface is no part of the dimensions".

This narrows what the check measures, so stated plainly: three assets have role `ground` solids in a body list. `leaning-person` (the wall), `laptop` (four rubber feet, 6 mm) and `ivy-on-trellis` (2 solids). All three were run; 65 more assets have such solids only in `thermal` or `structure`, which the bounds check does not read.

## 4. Proofs

| Run | Result |
| --- | --- |
| Kernel `🦴️skinning` and `🔁️repeat` (`repo test-file`) | 5 of 5 and 16 of 16 passed |
| Kernel `🧬️provenance`, `📏️feature-edges`, `🎞️pose-animation`, GLB `🧊️vectors`, `🧪️oracles`, `🎞️scenes`, FBX `📦️structure`, `🧍️three-roundtrip`, `🔍️second-reader`, `🎞️scenes`, viewer `🧱️scene` | all passed (125 cases) |
| `🧊️model-catalog`, `external-venetian-blind,leaning-person,laptop` (10:24, 215 s, peak 2683 MB) | 5 of 5 passed (116 s, 20 s, 78 s per asset) |
| `🧊️model-catalog`, `ivy-on-trellis` (11:45, 178 s) | 3 of 3 passed |

Post-write proof (`W/prove.sh`; the catalog files are unchanged, `b6ffd717c49ee55b` and `8199c4216505f75b`):

| Asset | Gate | Scan | Silhouette guard, `validateAsset`, sweep | Check CLI | Matrix (252 cells) |
| --- | --- | --- | --- | --- | --- |
| `external-venetian-blind` (11:40) | 0 hard | clean | OK, 0, 0; bbox 0.4200 / 2.7500 / 1.2060 at 1:50 | `✓` | 0 fail: 252 ok (23 s) |
| `leaning-person` (12:09) | 0 hard | clean | OK, 0, 0; bbox 0.5160 / 1.7401 / 0.5320 at 1:50 | `✓` | 0 fail: 182 ok, 70 group-drift (206 s) |

## 5. Stated plainly

- Not run by this lane: the full `🧊️model-catalog`, the full kernel suite and the other staffage suites. `requests.md` ("closing 2 CL-B") asks MZ for the re-run of the four assets.
- The new kernel case was written after the fix was in place; it was not seen red. What stands for the red state is the measurement of section 2 (0.9312 m before, 0.1500 m after, same tool).
- Seen, not mine, not touched: the kernel case `🎞️scenes` "no … source … calls an engine-dependent Math function" fails on `K/✅️validation/🪨️polyhedron/🟦️.ts` and `K/📐️bounds/🪨️polyhedron/🟦️.ts` (10:12; 29 of 30 passed).
- The queue: a request of 600 to 2500 MB waited 20 to 65 minutes behind 3600 MB requests while 1 to 4 GB were spare; the tools of this lane peak at 209 to 1183 MB (`ahead.ts`), 486 MB (proof with matrix) and 2.1 to 2.7 GB (`🧊️model-catalog`, three assets).
- No catalog file was written, so there is no `rewrote` line.

## 6. Files

- `K/🔺️tessellation/🧱️solid/🟦️.ts`, `K/🧫️fixtures/🦴️skinning/🔣️.json`, `K/🧪️tests/🦴️skinning/🟦️.ts`
- `S/🧪️tests/🧰️support/🧊️.ts`, `S/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts`, `S/🧪️tests/🧊️model-catalog/🥒️.feature`
- `U/requests.md` (section "closing 2 CL-B"), this report
- Tools (`W`): `ahead.ts` (extent per body scene as the oracle reads it, with and without contact surfaces, and the meshes reaching farthest), `modelcat.sh`, `prove.sh`, `wait.sh`, `q.sh`, `env.sh`; from the earlier round `probe.ts`, `red.ts`, `top.ts`, `chain-a.sh`, `chain-b.sh`, `backup/`.
