FINAL (2026-11-27 13:42: `pregnant-person` cured in the asset, a ring loft that was capped at both ends; `🧊️model-catalog` green for it alone, post-write proof clean with 0 failing matrix cells; the oracle widened by one clause so the uncapped ring stays under the watertight check, green for `playground-swing` and three more assets)

# 🧵️ Closing Lane CL-P: The Uterine Wall of `pregnant-person` Is Not Watertight

Paths: `R` = repo root, `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `U` = this ticket folder, `W` = `U/🧰️matrix/CL-P` (tools, kept), `GEN` = `U/🗑️generated/matrix/CL-P` (deleted with this report; every number below was read there in a log of a run that ended by itself). Every measuring run went through `repo capacity run` (`W/q.sh`), at most two requests at a time.

## 1. Cause: the asset, not the chord rule and not the loft tessellation

`uterine-wall` (structure, 1:50, 1:100, 1:200; bone `belly`) is a loft of 13 sections that returns into itself: section 13 repeats section 1 in `at`, `rx`, `ry`, `exponent` and `rotation`. It was authored with `capStart` and `capEnd` on. The two end rings therefore coincide point for point, and the two fan caps lie back to back in the same plane inside the ring. After the weld of the oracle every ring edge and every cap spoke is used twice in each direction: 4 overlapped directed edges per segment and no boundary edge.

| Scale | Segments | Overlapped edges (4 x segments) | Finding of the final pass |
| --- | --- | --- | --- |
| 1:50 | 26 | 104 | 104 |
| 1:100 | 24 | 96 | 96 |
| 1:200 | 13 | 52 | 52 |

Ruled out, read-only:

- A recent asset change: `git show <commit>:<file>` reads the same ring with both caps in every commit since the solid appeared (`01324745f80`, 11-22; absent in `b4927d2c311`, 11-21). The working file was last written 11-25 19:09.
- The chord rule of CX-Q (`S/🔨️modules/🧊️volume/⏱️job/🟦️.ts`): the chord only sets the number of segments. The overlap is 4 x segments for any number of segments, never 0.
- The loft tessellation (`R/🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🧵️loft/🟦️.ts`): the only uncommitted change is `solid.segments ??` (the flow tubes of CX-FLOW); the asset declares no `segments`. The kernel does what the document says: a ring with two caps.
- The check: `closed` with the weld-and-count exists unchanged since before 11-22. It never ran to its end for this asset before today: the earlier shard 12 that held `pregnant-person` left no log or JSON, so the final pass of today is the first verdict.

The catalog already has the right form for a ring: the chain links of `playground-swing` (`links-a-left` …, 15 sections, first = last, both caps off) and the case "measures the section path and wire diameter of a closed loft" of `🧊️volumes-rules` (C30). Scan of all 375 catalog files for lofts whose last section returns to the first: 8 solids; only the three `uterine-wall` solids are capped with coinciding end rings. `cane-loop` of `person-with-white-cane` is capped too, but has no `rotation`, so its end rings follow the one-sided end tangents, do not coincide and close properly (that asset passed in the same shard).

## 2. Cure

`W/ops/pregnant-person.json`, applied by `W/fix.ts`: `capStart` and `capEnd` of `volumes.{50,100,200}.structure[uterine-wall]` from `true` to `false` (6 values, nothing else). Catalog file `0ae4236ad669de49` -> `b4e6e895cd727dce` (1,496,345 bytes), written 12:52:56.

| Scale | Triangles before -> after | Boundary / overlapped before -> after | Signed volume before -> after | Reference |
| --- | --- | --- | --- | --- |
| 1:50 | 676 -> 624 | 0 / 104 -> 0 / 0 | 212.048 cm3 -> 212.048 cm3 | 221.7 cm3 |
| 1:100 | 624 -> 576 | 0 / 96 -> 0 / 0 | 412.684 cm3 -> 412.684 cm3 | 432.4 cm3 |
| 1:200 | 338 -> 312 | 0 / 52 -> 0 / 0 | 788.707 cm3 -> 788.707 cm3 | 848.2 cm3 |

- Reference: section area of the superellipse `4 rx ry Γ(1 + 1/n)² / Γ(1 + 2/n)` (n = 2.6) times the path length 0.81123 m (Pappus); the kernel value lies 4.4 to 7.0 % below it because the path is a polygon of 12 chords and the section a polygon of 26 to 13 corners. The caps add no volume (back to back), so the volume is the one of the skin.
- The columns "before" are read from the kernel through the oracle (`W/geo.ts --report`). The columns "after" for triangles, edges and volume are read from a replica independent of the kernel, written in Python (own quaternion rotation, own weld at 0.1 micrometre, the segment counts of the kernel): 624 / 576 / 312 triangles, 0 boundary and 0 overlapped edges, and the volume of the kernel to the last printed digit (the back-to-back caps added none). The kernel confirms the edges through the oracle (0 geometry findings after the cure, with the solid expected closed). The same replica reads 0 boundary and 0 overlapped edges for the four links of `playground-swing` (3998.12 mm3 per link; the kernel reads 151.929 cm3 for the 38 repeated links, 38 x 3998.13 mm3 = 151.929 cm3).
- Structure scenes after the cure: 113,100 -> 113,048, 45,068 -> 45,020, 28,368 -> 28,342 triangles (the caps), 1:500 unchanged at 19,216 (no uterine wall there). Still falling towards the coarser scales.

## 3. The check is not weakened: one clause added to the oracle

An uncapped loft was "open by design" for the oracle (`closedKind`), so the cure alone would have taken the solid out of the watertight check. `S/🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts` now has `loopLoft`: an uncapped loft whose last section repeats the first one in `at`, `rx`, `ry`, `exponent` and an explicit `rotation` is expected closed (`closedKind` returns true for it), so the weld-and-count and the positive signed volume are asserted for it. This widens the check; no existing expectation was removed. It now also covers the 4 link solids of `playground-swing` (8 with the thermal twins).

## 4. Proofs

| Proof | Result |
| --- | --- |
| Geometry findings before (`W/geo.ts`, the code of `🧊️model-catalog`) | 3 findings, the three of the final pass |
| Light proof of the trial copy before the write (`W/light.sh`, 12:52) | PASS: gate 0 hard findings, gate over the poses 0, `silhouette: OK`, `validate: 0 issues`, three-views sweep 0 findings, check CLI `--file` `✓`, geometry findings 0 |
| Post-write proof (`W/prove.sh`, 12:52 to 12:55; the catalog file equals the proven copy byte for byte) | gate 0 hard findings; official scan 1 clean; prints: every authored scale prints; `silhouette: OK`, 0 issues, 0 sweep findings; check CLI `✓`; geometry findings 0; matrix 252 cells: 238 ok, 14 group-drift, 0 unavailable, 0 fail (127 s) |
| `🧊️model-catalog` with `pregnant-person` as the only selected asset (command of the brief, 12:55 to 12:57) | exit 0, 1 file passed, 3 tests passed (84.75 s) |
| Geometry findings of `playground-swing` with the new clause (`W/geo.ts`, 12:43) | 0 findings; the 8 link solids `closed expected true`, 0 boundary, 0 overlapped, 151.929 cm3 each |
| `🧊️model-catalog` with `playground-swing` as the only selected asset (13:28) | exit 0, 1 file passed, 3 tests passed (21.06 s) |
| The other users of the oracle, whole suites (13:34, 13:36) | `🌊️flow-model` 12 passed of 12, `🧊️model-scale-files` 2 passed of 2 |
| `🧊️model-catalog` with `person-with-white-cane`, `tour-guide`, `parent-with-stroller` as the only selected assets (lofts in body and structure; 13:41) | exit 0, 1 file passed, 5 tests passed |

The suites `🧊️loft-shape`, `🧊️model-roundness`, `🧊️model-bounds` and `🧊️volumes-rules` were not run: they are owed for a change of shared code under `🔨️modules` or the kernel, and none was made (they do not import the oracle).

## 5. Stated plainly

- No kernel or module code was touched; one catalog file and the test oracle were written.
- The ring is now a skin without caps whose seam welds in the oracle; in the kernel mesh the first and the last ring are separate vertices at the same place, and the smooth normals use one-sided differences there, as for the swing links. Nothing was rendered or looked at.
- The kernel still accepts a capped loft whose end rings coincide (`validateModel` has no finding for it); only the oracle of `🧊️model-catalog` catches it. A validation finding in `R/🧰️framework/🔨️modules/🧊️3d/🗿️model/✅️validation/🟦️.ts` would catch it at authoring time; not done here, no other asset has the defect.
- I stopped one of my own queued requests by pid (a geometry audit made redundant by the light proof).
- The last run (two suites and three assets in one grant of 2500 MB) peaked at 2718 MB, 218 MB over its reservation; the single-asset runs peaked at 2015 MB and 1017 MB.
- The oracle clause was not typechecked by a `typecheck` run; it was executed by five green vitest runs.

## 6. Tools (`W`)

`env.sh`, `q.sh`, `fix.ts` and `ops/pregnant-person.json` (the cure), `make.sh` (trial copy), `geo.ts`/`geo.sh` (geometry findings and weld report per solid), `light.sh`, `prove.sh`, `modelcat.sh`, `chain.sh`, `suites.sh`.
