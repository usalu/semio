# silhouette ORACLE — the exact polygon oracle of `🧊️catalog-silhouette` computes `hedgehog`

2026-11-23. One file changed: `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🔍️oracles/🟦️.ts` (`exactIou` and its helpers; `raster` and `sharpIou` untouched). No threshold, tolerance, assertion, fixture or asset was touched; `hedgehog` stays in `exact`; the reference is still `polygon-clipping` 0.15.7 alone (the only clipping library in `node_modules`), the owned union code is not used.

## Cause

`stages.ts` on the dumped views of `hedgehog` (the polygons `exactIou` receives): every failure is inside the union of the reference faces (38 360 projected triangles at 1:50, 26 216 at 1:100), never in the drawing or the final overlay. The old oracle united sixteen regions per sweep and gave up on the whole snap unit when one sweep threw.

- Level 0 (sixteen triangles on the grid) already throws, e.g. 1:50 elevation at 10 µm chunk 1215 of 2258, at 0.1 mm chunk 782 of 2128. Two pairs were refused by the library in every order and identity tried (`strategies.ts` on the float grid at 0.1 mm), e.g. the sliver `[-0.054, 0.0549] [-0.0577, 0.0546] [-0.0610, 0.0544]` with `[-0.0596, 0.0545] [-0.0610, 0.0544] [-0.0697, 0.0567]`: a vertex of one triangle lies a few micrometres off the long edge of the other, and the grid points are float multiples of the unit (`0.061000000000000006`), so the sweep line orders the segments inconsistently.
- Level 1 and deeper throw because the operands are results of earlier unions: their intersection vertices are off the grid (`[0.0701607…, 0.0859821…]`, the coordinates of the request) and nearly coincide with other edges.

Three coarser snaps cannot help: each one meets other near-coincident edges among the quills (four of six views threw at all three). The same weakness was hidden in `cooking-pot`: 1:50 elevation threw at 10 µm and 1:50 front at 10 µm and 0.1 mm; they only passed on a coarser snap.

## Change

1. Integer grid: the rings are rounded to integer multiples of the unit (`gridded`) and all clipping runs on those integers; the IoU is a ratio of areas, so no scaling back is needed. With integers no pair of `hedgehog` is refused at any of the three units.
2. `merged`: sixteen regions per sweep as before; a sweep that throws is replaced by the union of the unions of its two halves, down to single pairs.
3. `overlay`: a pair (union or intersection, also the final overlays of `exactIou`) that throws is computed once more with both operands rounded back to the grid (`regridded`, zero-area rings dropped). If that throws too, the error propagates, the next snap unit is tried and after the last one `exactIou` throws as before: no path returns without a computed comparison.
4. The drop limit for degenerate rings is unchanged (more than 4 grid cells), now decided on integers.

Recoveries needed for `hedgehog` at 10 µm (`strategies.ts` on the integer grid, which tries more identities than the final code): 1 to 18 refused sixteen-way sweeps per view (of about 2 260 at 1:50), 0 to 5 regridded pairs per view; 1:100 elevation needs none. A regridded pair moves vertices by at most half a unit (5 µm).

## Numbers

Method: `dump.ts` wrote the measured views of the five assets once (48 views); `compare.ts` ran the old and the new `exactIou` on that identical input; `diff.py` made the table. Columns: exact IoU before, after, difference, owned IoU.

| asset | scale | view | before | after | diff | owned |
| --- | --- | --- | --- | --- | --- | --- |
| `cooking-pot` | 1:50 | elevation | 0.997258131 | 0.997291388 | 3.3e-05 | 0.997601 |
| `cooking-pot` | 1:50 | front | 0.998332381 | 0.998064223 | 2.7e-04 | 0.998099 |
| `cooking-pot` | 1:50 | plan | 0.993906132 | 0.993906132 | 0.0e+00 | 0.993175 |
| `cooking-pot` | 1:100 | elevation | 0.999234635 | 0.999234635 | 0.0e+00 | 0.998742 |
| `cooking-pot` | 1:100 | front | 1.000000000 | 1.000000000 | 0.0e+00 | 1.000000 |
| `cooking-pot` | 1:100 | plan | 0.991566411 | 0.991566411 | 0.0e+00 | 0.990156 |
| `cooking-pot` | 1:200 | elevation | 1.000000000 | 1.000000000 | 0.0e+00 | 1.000000 |
| `cooking-pot` | 1:200 | front | 1.000000000 | 1.000000000 | 0.0e+00 | 1.000000 |
| `cooking-pot` | 1:200 | plan | 0.996432126 | 0.996432126 | 0.0e+00 | 0.995881 |
| `retractable-bollard` | 1:50 | elevation | 0.997364673 | 0.997364673 | 0.0e+00 | 0.997306 |
| `retractable-bollard` | 1:50 | front | 0.999652771 | 0.999652771 | 0.0e+00 | 0.999713 |
| `retractable-bollard` | 1:50 | plan | 0.999974486 | 0.999974486 | 0.0e+00 | 0.999981 |
| `retractable-bollard` | 1:100 | elevation | 0.997810005 | 0.997810005 | 0.0e+00 | 0.998342 |
| `retractable-bollard` | 1:100 | front | 0.999978083 | 0.999978083 | 0.0e+00 | 1.000000 |
| `retractable-bollard` | 1:100 | plan | 0.999885440 | 0.999885440 | 0.0e+00 | 0.999899 |
| `retractable-bollard` | 1:200 | elevation | 0.999896447 | 0.999896447 | 0.0e+00 | 0.999906 |
| `retractable-bollard` | 1:200 | front | 0.999884790 | 0.999884790 | 0.0e+00 | 0.999895 |
| `retractable-bollard` | 1:200 | plan | 1.000000000 | 1.000000000 | 0.0e+00 | 1.000000 |
| `retractable-bollard` | 1:500 | elevation | 1.000000000 | 1.000000000 | 0.0e+00 | 1.000000 |
| `retractable-bollard` | 1:500 | front | 0.999999025 | 0.999999025 | 0.0e+00 | 1.000000 |
| `retractable-bollard` | 1:500 | plan | 1.000000000 | 1.000000000 | 0.0e+00 | 1.000000 |
| `panel-radiator` | 1:50 | elevation | 0.999295548 | 0.999295548 | 0.0e+00 | 0.999506 |
| `panel-radiator` | 1:50 | front | 0.999391400 | 0.999391400 | 0.0e+00 | 0.999374 |
| `panel-radiator` | 1:50 | plan | 0.989412422 | 0.989412422 | 0.0e+00 | 0.989410 |
| `panel-radiator` | 1:100 | elevation | 0.976427956 | 0.976427956 | 0.0e+00 | 0.977224 |
| `panel-radiator` | 1:100 | front | 0.995628097 | 0.995628097 | 0.0e+00 | 0.995831 |
| `panel-radiator` | 1:100 | plan | 0.957781800 | 0.957781800 | 0.0e+00 | 0.989639 |
| `panel-radiator` | 1:200 | elevation | 0.976185446 | 0.976185446 | 0.0e+00 | 0.980021 |
| `panel-radiator` | 1:200 | front | 0.998894518 | 0.998894518 | 0.0e+00 | 0.999167 |
| `panel-radiator` | 1:200 | plan | 0.972007032 | 0.972007032 | 0.0e+00 | 0.976702 |
| `sliding-door` | 1:50 | elevation | 0.999958878 | 0.999958878 | 0.0e+00 | 0.999959 |
| `sliding-door` | 1:50 | front | 0.999344206 | 0.999344206 | 0.0e+00 | 0.999548 |
| `sliding-door` | 1:50 | plan | 0.982565968 | 0.982565968 | 0.0e+00 | 0.983843 |
| `sliding-door` | 1:100 | elevation | 0.999360080 | 0.999360080 | 0.0e+00 | 0.999350 |
| `sliding-door` | 1:100 | front | 0.999720703 | 0.999720703 | 0.0e+00 | 0.999742 |
| `sliding-door` | 1:100 | plan | 0.978247237 | 0.978247237 | 0.0e+00 | 0.978725 |
| `sliding-door` | 1:200 | elevation | 0.997909272 | 0.997909272 | 0.0e+00 | 0.998075 |
| `sliding-door` | 1:200 | front | 0.974583787 | 0.974583787 | 0.0e+00 | 0.976146 |
| `sliding-door` | 1:200 | plan | 0.954809387 | 0.954809387 | 0.0e+00 | 0.959407 |
| `sliding-door` | 1:500 | elevation | 0.992055903 | 0.992055903 | 0.0e+00 | 0.994600 |
| `sliding-door` | 1:500 | front | 0.976423777 | 0.976423777 | 0.0e+00 | 0.982422 |
| `sliding-door` | 1:500 | plan | 0.963046484 | 0.963046484 | 0.0e+00 | 0.978070 |
| `hedgehog` | 1:50 | elevation | throws | 0.944668456 | - | 0.941710 |
| `hedgehog` | 1:50 | front | throws | 0.943251809 | - | 0.943008 |
| `hedgehog` | 1:50 | plan | throws | 0.934598017 | - | 0.935426 |
| `hedgehog` | 1:100 | elevation | 0.937708578 | 0.937708578 | 0.0e+00 | 0.936798 |
| `hedgehog` | 1:100 | front | 0.945451423 | 0.945732750 | 2.8e-04 | 0.946025 |
| `hedgehog` | 1:100 | plan | throws | 0.941230682 | - | 0.940786 |

- 39 of the 42 views of the other four assets are equal to nine decimals (difference 0).
- `cooking-pot` 1:50 elevation (3.3e-5) and 1:50 front (2.7e-4) and `hedgehog` 1:100 front (2.8e-4) differ because the old oracle threw at 10 µm and returned the value of a coarser snap; the new one computes at 10 µm. Per unit, old against new: `cooking-pot` 1:50 elevation 10 µm throws / 0.997291388, 0.1 mm 0.997258131 / 0.997258131; 1:50 front 10 µm throws / 0.998064223, 0.1 mm throws / 0.998044002, 0.5 mm 0.998332381 / 0.998332381. At the unit the old oracle used, the values are identical.
- One same-unit difference exists off the path the test takes: `cooking-pot` 1:50 elevation at 0.5 mm 0.990535529 / 0.990296960. `ties.ts`: 24 rings of `visible` and 46 of `full` have exactly 4 cells of area there; the old float test `area > 4 * unit * unit` kept 20 and 40 of them by rounding noise, the integer test drops all (the inside area changes by 4.00 mm², 16 cells). At 10 µm such a ring is 0.0004 mm² and no view changes in nine decimals.
- `hedgehog` at all three units with the new oracle (1:50 elevation, front, plan; 1:100 elevation, front, plan): 10 µm 0.944668 0.943252 0.934598 0.937709 0.945733 0.941231; 0.1 mm 0.944550 0.943063 0.934678 0.937291 0.945451 0.941382; 0.5 mm 0.940865 0.941736 0.934746 0.936118 0.944548 0.939817. Largest distance to the owned IoU at 10 µm: 0.0030 (limit 0.06).
- Time of `exactIou` for `hedgehog`: 0.4 to 11 s per view, about 20 s for the six.

## Proof

Run in `📦️packages/🟦️typescript` with the pinned bun, each to the end, after the oracle was installed:

- `bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ../../🧪️tests/🧊️catalog-silhouette/🟦️.ts -t "(hedgehog)( |$)"`: 1 passed, 384 skipped, 46 s, exit 0.
- the same with `-t "(cooking-pot|retractable-bollard|panel-radiator|sliding-door)( |$)"`: 4 passed, 12 s, exit 0.

## Files

- changed: `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🔍️oracles/🟦️.ts`
- scripts in `🧰️silhouette/ORACLE/`: `env.sh`, `dump.ts` (views to JSON), `compare.ts` (an oracle module on dumped views), `diff.py` (table), `stages.ts` (which stage of the old oracle throws; holds the old union), `strategies.ts` (recovery strategies and their counts), `ties.ts` (rings at the drop limit), `exact-part.ts` and `patch.py` (the new part and how it was installed)
- tool output in `🗑️generated/silhouette/ORACLE/` (190 MB of dumps, copies of the oracle, tables, test logs): deleted

Open: the entry "silhouette SAV — the polygon-clipping oracle throws on `animals/hedgehog`" of `requests.md` is answered by this change; `requests.md` and `silhouette-SAV.md` were not edited (not owned by this task). The MCP server `semio` did not connect in this session; no ticket was opened or closed by this task.
