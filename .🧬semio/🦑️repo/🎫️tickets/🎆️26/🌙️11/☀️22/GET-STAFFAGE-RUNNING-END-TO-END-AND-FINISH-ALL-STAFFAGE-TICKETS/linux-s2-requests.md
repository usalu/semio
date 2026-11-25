FINAL

# 🧮️ Lane s2-requests: triage of `requests.md` against the code as it is (Linux worker g1-m04, 2026-11-25, 09:05 to 09:55)

`requests.md` held 88 requests when the lane began and 89 when it ended (the MM entry of 09:25 is row 89). Every row below was decided from the code and the catalog files of this checkout at the time stated; only what ran in this session is called measured.

Nothing of `S` is changed by this lane. One slip, undone: between 09:25 and 09:40 the test runner of this lane wrote three fixture folders into `S/🧫️fixtures` (`🔺️prism-union`, `🧊️prism-depth`, `🕳️blank-outline`, one `🔣️.json` each) through a mirror whose fixtures were still a link into the worktree. No test and no code read them. They were removed at 09:52 (`git status` of `S/🧫️fixtures`: 0 untracked), the mirror now copies the fixtures and `run-test.sh` refuses a linked mirror.

Short names: `E` = `S/🔨️modules/🧮️evaluation/🟦️.ts`, `P` = `S/🔨️modules/🔺️polygon/🟦️.ts`, `Si` = `S/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`, `St` = `S/🔨️modules/🏗️structure/🟦️.ts`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `K` = ticket `☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE`, `W` = `U/🧰️linux/s2`.

## 1. Outcome

- Six patches and six test cases, each with an independent reference, lie in `W` as proposals. All six pass `git apply --check` against the worktree (exit 0 each, 09:50), apply together, and a mirror with all six runs 238 tests in 15 files with 0 failures (bun test runner, exit 0). `tsc --noEmit` reports the same single error on the patched and on the unpatched mirror (a module of `🎯️targets` the mirror does not copy), so the patches and the new cases add no type error.
- Of the seven families named in the task, four are cured by a patch and measured on the affected asset (union, tie, blank shapes, in-band), two were cured in the asset by the Mac fixers while this lane ran (language geometry at 09:29, half cone at 06:06) and measured clean here, and one (shape depth) is cured by a patch but blocks no cell of `K`.
- Counts over the 89 rows: 12 answered (in code, in an asset or by a decision), 5 obsolete, 9 notes without a request, 2 undecided without a new measurement (rows 4 and 77), 61 still valid. Of the 61, 12 carry a patch of this lane and 49 stay open, most of them worked around in the assets. A thirteenth row with a patch is 76, answered in the asset, where the patch adds the missing rule.

## 2. The seven families

### 2.1 `PolygonUnionError` in `evaluate` / `unionPolygons` (rows 72, 78, 82; rows 9, 22, 27, 38, 41 belong to the family)

Reproduced on the current module with two in-memory variants of catalog people (`W/union-assets.ts`; nothing written to the catalog), over the subjects of the gate at 1:50:

| Variant | Worktree module | With `01-union-exact.patch` |
|---|---|---|
| `pregnant-person`, front `shank-r` with the profile point put back to `[0.1217, 0.0509]` (level edge, row 78) | 2 of 150 subjects throw (`waddle-walk` at 0.133 s and 0.289 s, 11 polygons) | 0 of 151 |
| `tour-guide`, elevation `hand-r` copied into the plan (row 82) | 1 of 117 throws (`gesture-point` at 7.992 s, 16 polygons) | 0 of 117 |

Both runs exit 0. The three thrown inputs are recorded in `W/tests/🔺️prism-union/🔣️.json`.

Two causes, found by running the stages of the union one by one on the recorded inputs:

1. `dropCollinear` (`P:147`) measures the first vertex of a ring against the last one and the last one against the first, also when it has already removed the other. A corner that arrives as two vertices 48 nanometres apart at the seam of the ring loses both: on the first shank input the slab union had built the right boundary (area 0.0334170, equal to polygon-clipping to 1e-16) and `dropCollinear` cut a corner 0.5 mm deep out of it (0.0333917), so the area check refused it.
2. `slabUnion` (`P:238`) decides in floating point: `yAt` of a nearly vertical edge, the 1e-9 node cell of `chainPieces` and the 1e-9 filter of the vertical steps do not agree on which pieces meet, and the boundary stays open (second shank input, hand input). Its trapezoid area is right in all three cases.

Cure (`01-union-exact.patch`, `P` only): `dropCollinear` measures only against vertices that are still there and clears the seam from both sides; `slabUnion` puts the vertices on a grid of a nanometre and decides everything after that in integer and rational arithmetic (`bigint`), so pieces that meet share one node and the boundary always closes. The float slab union is replaced, not kept beside it. No tolerance of the union changes (`EPS`, `THIN_M`, `NEAR_M`, `SAME_M` and the area check are as they were); the grid is 300 times finer than `SAME_M`. Cost: 31 to 45 ms for the three recorded inputs; the exact path runs only where the two edge-splitting passes fail.

Test `🔺️prism-union` (reference shapely `unary_union` through the existing oracle, and polygon-clipping): the three recorded inputs as one ring within the tolerance of the polygon-union vectors (measured difference 2.4e-8), and four profiles turned through the view direction in 964 steps. Worktree module: 3 fail, 4 pass. Patched: 7 pass. The existing `🔺️polygon-union` suite passes on both (24 of 24; also through `repo test-file` on the worktree, 3.3 s).

Not shown: the sweeps of the test pass on the worktree module too, so they guard and do not discriminate; the inputs of rows 41 and 72 no longer exist (the assets carry simplified profiles), so those two rows are cured by inference from the same cause, not by measurement. Row 9 was retried on the current module with the plumb arm put back (164 instants, 4 scales): 0 throw, so the repairs of 2026-11-23 already answer it.

### 2.2 `shapeDepthIn` for sweeps along v and u (rows 8, 80)

Still as described: `E:637`, the elevation reads every profile at `local(p, 0)` (`E:651`) and front and plan read every extruded profile as (u, v) at the middle of the span (`E:652`). Measured with `W/depth-probe.ts used-classical-guitar 50 stored plan`: `lid-panel` 0.3528 m against 0.1480 m in the side elevation, `pocket` 0.2680 against 0.1780, largest error over the 29 prisms 204.8 mm (the numbers of row 80).

Cure (`02-shape-depth-sweep.patch`, `E` only): one function `prismPoint` places a profile point at one end of the sweep; `solid3` draws the outline with it and `shapeDepthIn` ranks by the nearest corner with it, for the three sweeps, the taper to `zTip` and skinned solids, in all three views. Measured with the patch: largest error 0.1 mm; the digest of all evaluated rings is the same (`74609469c115feaa`), so no outline moves.

Test `🧊️prism-depth` (reference numpy, placing the profile by the definition of the sweeps): worktree 5 fail, patched 5 pass. Seven neighbouring suites (depth-order, solid-sweep, solid-placements, lateral-spread, front-projection, plan-projection, evaluation-state) pass with the patch, 82 of 82.

Risk, measured with `W/order-compare.ts` on seven assets: 174 of 444 paint orders (scale x pose x view) change, the evaluated rings differ in 0 states. The geometry gates do not see it; every sheet does. It blocks no cell of `K`. Whether each changed order is the right one was not looked at on a sheet.

### 2.3 The paper-height tie at the tolerance (rows 79, 88)

Measured with `W/measure-tie.ts used-harmonium` on the worktree: 1:50 side elevation drawn 22.54 mm against 23 mm, difference 0.46000000000000085 against tolerance 0.45999999999999996 (excess 8.9e-16 mm), finding `paper-height`; 1:100 excess 4.4e-16 mm, finding; the dimension check in metres has no finding. With `03-paper-tie.patch`: no paper finding at either scale, the metre check unchanged.

Cure (`T/🧰️.ts`): one function `exceeds(deviation, tolerance, magnitude)` decides `geometryFindings` (`agree`, `size`) and `paperFindings`; a deviation exceeds when it is above the tolerance by more than 64 units in the last place of the largest operand.

**The epsilon is arithmetic, not a widening.** It is a bound on the rounding of the two sides, it scales with the magnitude of the operands and not with the tolerance, and at 1 m it is 1.4e-14 m. D18 itself is unchanged. The test proves the other direction: a drawing one nanometre beyond D18 still fails at every denominator from 1:20 to 1:1000 and at every sheet position tried. The suggestion in row 88 (`tolerance · (1 + 1e-9)`) would have been a widening by 23 picometres per metre; this is five orders tighter and has a stated reason.

Test `⚖️tolerance-tie` (reference exact rational arithmetic, CPython `fractions`): 9 cases x 8 denominators x 5 sheet positions through `paperFindings`, 11 pass. Only a verdict within the rounding bound can change, and only from fail to pass.

Left open on purpose: the first half of row 79 (a declared section is compared with the height of the whole body). The harmonium passes without it once the tie is decided; comparing a section with the cut body is a rule change for the owner of `T`.

### 2.4 `blank-shapes` on unfilled symbols (row 86)

Measured with `W/measure-blank.ts kneeling-gardener gardener-block 200,500`: the test of `checkScene` (`T/🧰️.ts:462` to `467`, fills `shape:<id>` only) finds 90 of 90 pose x view x mode drawings blank at each scale, 15 cells each, 30 in all, and `blankReason` (`:421`) prints "pen none mm". With `04-blank-shapes.patch`: 0 blank, reason text "pen 0.18 mm".

Cure: `drawnIds` takes the fills and the outline runs of the shapes that carry ink (a fill, or a stroke wider than zero and not fully transparent); `blankReason` reads the pens of a symbol. The rule is not weakened: a shape whose path items carry no ink is still blank, and the test shows it.

Test `🕳️blank-outline` (reference sharp with librsvg, the ink of the written SVG): five drawings of the symbol have ink and count as drawn; the same with stroke width zero have no ink and count as blank. 5 pass.

### 2.5 `language-geometry` of `storage-crate-stack` (row 76)

Cause, measured at 09:26 with `W/language-probe.ts`: 21 of 63 drawings differ between English and German at 1:50 and at 1:100 (the side elevation in all seven modes, all three poses), 0 at 1:200 and 1:500. Every shape is shifted by 4.18 mm. The front section was named "Longitudinal crate section" / "Kistenlängsschnitt" instead of by its letters; the cut-line marker prints that name beside its arrows (`S/🔨️modules/🎨️depiction/✂️section/🟦️.ts:92`), the two phrases have different widths, and the scene is 58.4 mm wide in English and 52.7 mm in German. With the label set to `A-A` in memory: 0 of 63.

The Mac fixer wrote exactly that into the catalog file at 09:29. Measured again on the file: 0 of 63 at 1:50 and 1:100. The cells are cured in the asset.

What stays valid is the cause behind it: the validator accepts any text as a section name. `05-section-label-rule.patch` adds the rule `section-label` to `validateAsset` (one or two capital letters, the same in both languages). A scan of the catalog found 79 section declarations in 20 assets and this asset as the only one that broke the rule, so the rule reports nothing on the catalog as it is now.

Test `✂️section-label` (reference svgelements, where the paths of the SVG land in each language): 9 pass; `✅️asset-validation` passes with the patch, 83 of 83, as without.

### 2.6 The in-band measure on slender bodies (row 74, second half)

The check normalises each view to a unit longest side and flags a Hausdorff distance below 0.03 (`T/🪞️derived.ts:13`, `:91`). On a 3 m column that is 90 mm, a third of its width; on an 8 m beam 240 mm, more than its width.

Cure (`06-in-band-relative.patch`, `T/🪞️derived.ts`): between two views of one band the distance is taken relative to the box of the two views (`relativeHausdorff`: each coordinate difference divided by the larger extent along that axis). It is never below the plain distance and zero exactly where that is zero, so an exact copy, mirror, rotation or scaled copy is flagged as before. The threshold 0.03 and the band-copy test are unchanged.

**This changes a measure, so it is a decision for the dev, not a repair to land silently.** It narrows what counts as the same drawing, and by pass or fail it raises fewer flags. Measured on the default pose of all 375 assets at every authored scale (`W/inband-scan.ts`, 4,248 pairs of views, exit 0):

| | Flags below 0.03 | Assets | Not on the allowlist |
|---|---|---|---|
| Measure as it is | 26 | 15 | 4: `steel-column` front at 1:100, 1:200, 1:500 and `glulam-beam` plan at 1:500 |
| Box-relative | 19 | 11 | 0 |

Seven flags go: the four above (`steel-column` 0.0252 becomes 0.067, and 0.0067 becomes 0.067; `glulam-beam` 0.0287 becomes 0.371) and three that are already allowlisted (`norway-spruce` front at 1:200 and 1:500, `windmill-palm` front at 1:200). No other asset changes its verdict.

Test `🪞️slender-in-band` (reference shapely, `affinity.scale` and `hausdorff_distance` under the eight transforms): the four cases of the existing vectors keep their verdicts, a column and a beam are kept, an exact copy and two near copies of the column are still flagged. 12 pass.

The alternative without a change of measure is an allowlist line for each of the two. For `steel-column` that line would be untrue, since its front and side do differ, which is why the coordinator refused it.

### 2.7 The tapering half cone (row 81)

Measured with `W/findings-probe.ts used-concert-harp` on the catalog file of 06:06: no hard geometry finding in any pose at any of the four scales (tolerance 39 mm); row 81 had 84 failing cells. ML-1 drew the back as eight prisms swept along the box, so the cells are cured in the asset. The matrix confirmation of that file belongs to ML-1 and was not run here.

The request stays valid as debt: eight stacked prisms are a staircase, and the front of 1:200 and 1:500 is still flat. No patch was written, because the cells no longer block and the right cure is a schema decision. Design, schema first:

- A third `solid` hint for 2D shapes, `{ "kind": "lofted", "sections": [{ "at", "rv", "rw", "arc"? }] }`: elliptic sections along the bone, optionally over half a turn. It is the 2D twin of the `loft` the volumes already have (`S/🧬️schema/🟦️.ts:349`), so one body can be stated once for drawing and volume.
- It also answers rows 39, 69, 83, 84 and 87, which each end in "no `solid` of the schema is this body" (elliptic limbs, hands, leaning trunks).
- The narrower option, carrying a flat twin along the depth profile of its bone (`depthProfile`, `E:554`), would move every twin shape of every asset in every pose and is not proposed.

## 3. Order of landing

| Order | Patch | Changes | Risk for the running proofs |
|---|---|---|---|
| 1 | `03-paper-tie.patch` | `T/🧰️.ts` | None for any drawing that is not an exact tie; `used-harmonium` loses its 21 cells. |
| 2 | `04-blank-shapes.patch` | `T/🧰️.ts` (other hunks than 03) | Verdicts change only from blank to drawn, for shapes with ink; `kneeling-gardener` loses 30 cells. |
| 3 | `05-section-label-rule.patch` | `S/🔨️modules/🗂️catalog/🟦️.ts` | None today: the catalog has no violation since 09:29. A fixer who names a section by a phrase gets a validator issue. |
| 4 | `01-union-exact.patch` | `P` | Low. Instants that threw now evaluate, so a cell that failed on the error gets measured instead. Outlines that came from the float slab union move by nanometres (area differences of 3e-6 measured on the agave vectors). |
| 5 | `06-in-band-relative.patch` | `T/🪞️derived.ts` | None measured beyond the four assets of 2.6, but only after the dev's decision. The allowlist lines of `norway-spruce` and `windmill-palm` become unnecessary. |
| 6 | `02-shape-depth-sweep.patch` | `E` | High for pictures, none for geometry: 39 % of the sampled paint orders change. Land it when no fixer is between a proof and its sheets, and look at sheets afterwards. |

Patches 03 and 04 touch the same file in different hunks and apply in either order. On landing, each case folder of `W/tests` goes to `S/🧪️tests/<case>` and its `🔣️.json` to `S/🧫️fixtures/<case>/`.

## 4. All requests

Verdicts: **answered** (in code, by an asset or by a decision), **obsolete**, **valid** (with function and line), **note** (the entry asks for nothing). "Worked around" means the assets pass and the engine defect stays.

| # | Request | Verdict | Patch | Risk for running proofs | Landing |
|---|---|---|---|---|---|
| 1 | Structure solver: parts apart in a pose are one rigid body | Valid: `solveStructure` `St:589`, `reactionSolution` `St:243` solve one body on all active supports. Worked around in the assets. | none | none | later, own ticket |
| 2 | Plan of a tilted pose distorts flat bands of curved rods | Valid: `depthProfile` `E:542`, a twin is carried at one depth (`E:554` to `559`). Same root as 40. | none | none | with the lofted hint (2.7) |
| 3 | `volumes-silhouette`: sub-cell gap counted at the top bin | Valid: `boundarySpread` `Si:874`, `measureBanded` `Si:945`; module unchanged since 2026-11-22 12:28. Worked around. | none | none | later |
| 4 | Thermal coverage with `envelope: true` closes real gaps | Undecided: `🧊️volumes/🟦️.ts:400` and `:412` leave `envelope` solids out of the flood walls, which may already answer it; the radiator with the flag was not measured again. | none | none | measure first |
| 5 | Thermal coverage: a face on a closed neighbour is not outer | Valid, not measured again (rule in `🧱️rules/🟦️.ts:493` ff.). Worked around by `areaM2`. | none | none | later |
| 6 | Revolve `lateralScale` axis against the README | Valid, measured here: a `revolve` of radius 0.1 with `lateralScale` 2 is 0.2 (v) by 0.4 (w) about the u axis and 0.4 (u) by 0.2 (w) about the v axis. The README states the axis for the capsule only (line 209, along w); its `revolve` row (line 210) names `lateralScale` without an axis. The kernel (`🧰️framework/…/🌀️revolved/🟦️.ts:74`, `:150`) stretches the node X axis when the axis of revolution is not u. | none | none | state the axis in the README, then re-check the assets that use it |
| 7 | Bone mass has no lateral centre (`comZ`) | Valid: `S/🧬️schema/🟦️.ts:144` has `comX`, `comY` only; `massCentres` `E:758`. | none | none | later |
| 8 | `shapeDepthIn`: paint order of sweep v and u solids | Valid: `E:637`, `:651`, `:652`. Measured (2.2). | `02-shape-depth-sweep.patch` | high for pictures | 6 |
| 9 | `unionPolygons` fails on a shape seen exactly edge-on | Answered by the repairs of 2026-11-23 (`snapped` `P:74`, vectors `slivers`): retried here with the plumb arm, 0 of 164 instants throw. | none | none | none |
| 10 | `volumes-silhouette` against the print rule, lattice at 1:500 | Answered by row 71 (no engine change; `tower-crane` green). | none | none | none |
| 11 | Structure solver: one rigid body (reclaimed components) | Valid, same as 1. | none | none | later |
| 12 | A truss is a closed lattice, the rig is a tree | Valid: `memberDiagram` `St:518` cuts along the rig tree; the schema has no closures. | none | none | later, own ticket |
| 13 | Thermal coverage: copies of one `repeat` never seam | Valid, not measured again. Worked around with single boxes. | none | none | later |
| 14 | Support solids for pose-dependent supports | Valid: `supportsOf` `St:221` to `222` activates every `structure.supports` anchor in every pose. | none | none | later |
| 15 | Lifted or set-down part shares weight with the footing | Valid, same as 1. | none | none | later |
| 16 | Banded raster reports the top bin when the frame height changes | Valid, same as 3 and the tiling rows (43, 45, 50, 66). | none | none | later |
| 17 | No per-pose presence of solids and shapes | Valid: no `poses` or `hidden` member on shapes or solids in the schema (`hidden` exists on thermal regions only, `:705`). | none | none | later, schema |
| 18 | Slender curved tubes (`followedWidth`) | Valid: `followedWidth` `Si:375`, `stripOf` `Si:359`. Worked around. | none | none | later |
| 19 | Open detail strokes on a part a pose turns over | Valid as a feature request; no `face` on strokes in the schema. | none | none | later |
| 20 | Manifest rows can be lost under the replica | Mitigated outside the repo (`🧰️silhouette/sync-manifest.py`, `manifest-check.ts` 0 issues per the handoff); the single manifest writer in `S` does not exist. Valid. | none | none | later |
| 21 | catalog-silhouette: Hausdorff gate fails for an established human | Obsolete: the gate is unchanged (`S/🧪️tests/🧊️catalog-silhouette/🟦️.ts:104`), the silhouette rounds repaired the assets (385 of 385 on 2026-11-24 per the handoff, not run here). | none | none | none |
| 22 | `unionPolygons` cannot place a shape seen along its profile | Valid as a family (2.1); the sunbather input itself no longer exists. | `01-union-exact.patch` | low | 4 |
| 23 | Force residual when the support distribution switches | Valid, not measured again: `reactionSolution` `St:243` caps friction (`St:320`) without redistributing. | none | none | later |
| 24 | Nine check samples miss the jerks at clip keys | Valid: `queriesOf` (`🩺️check/🟦️.ts:55` to `60`) samples uniformly only. | none | none | later |
| 25 | Plan head of `seated-guitarist` is a stretched slab | Partly answered in the asset: `head` is a prism swept along w over 173 mm and `neck` a body of revolution; `hair`, `beard`, `ear-l`, `ear-r` are still 0.8 mm slabs swept along v. Valid for those four. | none (asset) | none | with the people rounds |
| 26 | `standing-adult` changed under the wave | Note: no action asked of the engine. | none | none | none |
| 27 | `unionPolygons`: second edge-on measurement (`person-pushing-bicycle`) | Same family as 22; the input no longer exists. | `01-union-exact.patch` | low | 4 |
| 28 | `repo run` hangs for `@teaching/staffage` | Valid. On Linux: `repo test-file` of one staffage file returns in 3.3 s (24 pass); `repo run @teaching/staffage:typecheck` had reached "the command starts" after 170 s and was ended by this lane's limit, without a result. | none | none | owners of the build system |
| 29 | Failures of neighbouring suites that are not flow code | Mostly obsolete: schema-conformance 43 of 43 and multiview-coverage 37 of 37 on 2026-11-24 per the handoff; the moments point is row 36. `model-batch`, `cast-export`, `cast-lod`, `multiview-sheet` were not run here. | none | none | re-run those four |
| 30 | catalog-silhouette: the two rasters decide differently on holes and slits | Valid: `measureView` seals the body mask only (`Si:1136` to `1137`), `fillSmallHoles` `Si:833` runs on each mask alone (`Si:879`). Worked around. | none | none | later, one silhouette ticket |
| 31 | A `slender` solid thicker than the feature is compared too wide | Valid: `strokesOf` `Si:394`, width from the mesh at `Si:411`. Worked around. | none | none | later |
| 32 | README: `extruded` with `sweep: "u"`, straight span between origin and tip | Valid: README line 146 names "w or v" only; `E:451` takes u and clamps the taper to the bone. | none | none | any time (docs) |
| 33 | Second measurement of the hole and slit asymmetry | Valid, same as 30. | none | none | later |
| 34 | `volumes-feature`: smallest dimension of a ribbon and a prism | Valid: `sortedExtents` `🧱️rules/🟦️.ts:126`, `thinnestDrawn` `:234`, `sheetExtents` `:354`. Worked around. | none | none | later |
| 35 | catalog-silhouette limits measured on plants | Obsolete: the suite decided (green on 2026-11-24 per the handoff) and the plants were repaired in the silhouette rounds. | none | none | none |
| 36 | Moments view: a member reads a different magnitude per view | Answered (stated in the entry, `small-reds.md` point 4); `memberMagnitude` in `🏗️structural-view/🟦️.ts:345`. The 1e-9 lobe threshold (`:387`) is still there: valid as a small point. | none | none | none |
| 37 | `walking-commuter`: plan feet and hand do not follow the stride | Answered (stated in the entry, `small-reds.md` point 3). | none | none | none |
| 38 | `unionPolygons`: prism faces 0.2 to 0.6 micrometres apart | Answered: `sleeping-person` `turn-over` 6.37 to 6.51 s, 71 instants at four scales, 0 throw here; the three inputs are `slivers` of the polygon-union vectors. | none | none | none |
| 39 | `lateralScale` of a revolved 2D solid is accepted and ignored | Valid: the revolved branch of `solid3` (`E:484` to `524`) never reads it; the type carries it (`S/🧬️schema/🟦️.ts:263`). | none | none | with the lofted hint, or remove the member |
| 40 | A front shape with a same-named side shape is a plate at one depth | Valid: `depthProfile` `E:554` to `559`. Worked around by ids. | none | would move every twin | not proposed (2.7) |
| 41 | `unionPolygons`: tapered toe prism of twelve points | Same family as 22; the asset carries a five-point profile, so the input is gone. | `01-union-exact.patch` | low | 4 |
| 42 | Sharp reference grows faster than linearly | Valid: `raster` (`🧊️catalog-silhouette/🔍️oracles/🟦️.ts:19`) still writes every triangle into one path. | none | none | later |
| 43 | Tiles change the largest boundary distance | Valid: `measureBanded` `Si:945` fills small holes per tile; the doc of `TILE_CELLS` (`Si:924`) says tiles never change a verdict. | none | none | later, with 30 |
| 44 | Centre line of a thin inclined blade is crooked | Valid: `strokesOf` `Si:394` takes the centre line from the mesh (`centreLineOf`, `🧵️centreline/🟦️.ts:121`). Worked around. | none | none | later |
| 45 | A small hole counts or not depending on its tile | Valid, same as 43. | none | none | later |
| 46 | 256-cell hole limit and hairline seal decide differently | Valid, same as 30 (`SMALL_OBJECT_CELLS` `Si:1100`). | none | none | later |
| 47 | What a `slender` solid is compared as | Valid: `strokesOf` `Si:394`, `axisStroke` `Si:419`. | none | none | later |
| 48 | The librsvg reference stops at 10 MB | Answered: the oracle of 2026-11-23 21:29 reads without the limit (`sharp(…, { unlimited: true })`, oracles `:31`). The triangle budget as a volume rule was not added: valid as a small point. | none | none | none |
| 49 | Revolved front shape and kernel tessellate half a step apart | Valid: `UNION_CIRCLE_SEGMENTS = 32` (`E:323`) against 30 kernel segments. | none | none | later |
| 50 | Banded measure reads more than the whole raster | Valid, same as 43. | none | none | later |
| 51 | Reference stroke covers more than the drawn stroke | Valid, same as 44 (cause measured in row 62). | none | none | later |
| 52 | polygon-clipping oracle throws on `hedgehog` | Answered: the oracle unites on an integer grid with retries (`exactIou`, oracles `:54` to `:98`); not run here. | none | none | none |
| 53 | A drawing cannot show a see-through hole | Answered by row 56. | none | none | none |
| 54 | A slender solid along a bent filled band is not followed | Valid: `followedWidth` `Si:375` fits one straight strip. | none | none | later |
| 55 | Openings at the 256-cell limit and hairline slots | Valid, same as 30. | none | none | later |
| 56 | The see-through hole is answered without an engine change | Note. | none | none | none |
| 57 | `strokesOf`: width and centre line the mesh does not give | Valid, same as 44. | none | none | later |
| 58 | A 0.5 mm clearance is sealed in the body mask only | Valid, same as 30. | none | none | later |
| 59 | Nested sheets with gaps below the raster cell | Answered by decision: the sheet stack is one pack at 1:50 (handoff, "For the dev's review"). | none | none | none |
| 60 | A bowed slender solid becomes one straight stroke | Valid, same as 44. | none | none | later |
| 61 | Dependency image of the Mac went away | Obsolete: the image is back (row 68). | none | none | none |
| 62 | `strokesOf`: measured cause of the wide reference | Valid, same as 44. | none | none | later |
| 63 | Notes for the next fixers | Note. | none | none | none |
| 64 | The detached image also stops R3-V | Obsolete, as 61. | none | none | none |
| 65 | A leftover `semio-trim` volume keeps the image from mounting | Valid for the storage code of macOS; cannot be checked on Linux. | none | none | owners of the build system |
| 66 | The small-hole rule decides by one cell, in two ways | Valid: tiling (43) and sealing (30). | none | none | later |
| 67 | `strokesOf` and the closed-pocket limit on people | Valid: 44 and 30. | none | none | later |
| 68 | The image is back; `strokesOf` on a long thin tube | First part obsolete, second valid (44). | none | none | later |
| 69 | Bent slender lofts at a slant; half profiles on limbs | Valid: 44, and 39 for the half profile. | none | none | later |
| 70 | Stroke of an askew slender prism has square ends | Valid: `strokeQuad` `Si:740`. | none | none | later |
| 71 | Lattice bars at 1:500 answered without an engine change | Note; its remaining ask is 44. | none | none | none |
| 72 | Union of an extruded outline with many short edges throws | Valid as a family (2.1); the asset carries the simplified outline, so its own input is gone. | `01-union-exact.patch` | low | 4 |
| 73 | `evaluate` thins profiles to 0.3 mm; the guard seals hairlines | Note by its own words. The facts hold: `simplifyRing(ring, 0.0003)` at `E:452`, sealing as in 30. | none | none | none |
| 74 | Band copy and in-band test on views with nothing more to print | First ask partly answered: the plan of the three walls is on `derivedViewAllowlist`, band copies have `bandCopyAllowlist` (48 entries). `steel-column` front and `glulam-beam` plan are valid and measured (2.6). The band copy of `stair-flight` (0.00376) is the same effect in the copy test and is not cured here. | `06-in-band-relative.patch` | none measured beyond four assets | 5, after the dev's decision |
| 75 | Four catalog documents rewritten with older content | Note of an incident; the writer is unknown and nothing in `S` can be checked for it. | none | none | none |
| 76 | `storage-crate-stack`: `language-geometry` | Answered in the asset at 09:29 (2.5), measured 0 of 63. The missing rule is valid. | `05-section-label-rule.patch` | none today | 3 |
| 77 | `saguaro-cactus`: the matrix does not end in 25 minutes | Undecided here: MM's repair bounds the memory (row 89); the run belongs to MV-3 and was not repeated. | none | none | MV-3 |
| 78 | Union of a prism swept along v throws at clip instants | Valid, reproduced: 2 of 150 subjects (2.1). | `01-union-exact.patch` | low | 4 |
| 79 | `paper-height` on a declared section, and a tie | The tie is valid and measured (2.3). The section against the cut body is a rule question, left open. | `03-paper-tie.patch` | none | 1 |
| 80 | `shapeDepth` ranks a swept solid by a wrong depth | Valid, measured: 204.8 mm (2.2). | `02-shape-depth-sweep.patch` | high for pictures | 6 |
| 81 | A tapering half cone has no solid hint | Cells answered in the asset: 0 hard findings in the poses at four scales (2.7). The hint is valid as debt. | none (design in 2.7) | none | schema decision |
| 82 | Union of a prism swept along w throws at one instant | Valid, reproduced: 1 of 117 subjects (2.1). With the patch the plan hand of `tour-guide` can be a solid, which is what its 7 `sync-bone-forward` findings wait for. | `01-union-exact.patch` | low | 4 |
| 83 | Tracer plates: six people keep hard findings | Note. Its conclusion is the lofted hint of 2.7. | none | none | people rounds |
| 84 | catalog-silhouette and the multiview gate ask opposite things | Valid as a design request (a drawing solid with a free cross-section): 2.7. | none | none | schema decision |
| 85 | Memory incident | Answered by the binding queue rule and row 89. | none | none | none |
| 86 | `blank-shapes` on a symbol drawn as an unfilled outline | Valid, measured: 30 cells (2.4). | `04-blank-shapes.patch` | none | 2 |
| 87 | Three walkers keep hard findings | Note. Same conclusion as 83. | none | none | people rounds |
| 88 | `paper-height`: tie decided by rounding | Valid, measured (2.3). | `03-paper-tie.patch` | none | 1 |
| 89 | The queue stands behind reservations that are no longer needed | Note to the coordinator (added 09:25). Seen here: a 2,500 MB request of this lane waited more than 10 minutes behind 5,000 MB requests. | none | none | coordinator |

## 5. Found on the way

- `S/🔨️modules/🗂️catalog/🧊️volumes-next-a` (2026-11-15) is imported by nothing in `🔨️modules`, `🧪️tests` or `📦️packages`. It looks like a stale copy of the volumes module.
- `validateAsset` compares the extents with `<=` in metres (`S/🔨️modules/🗂️catalog/🟦️.ts:478`). It has the same tie in principle as 2.3, with no case on record.
- `boneSync` and `anchorLandmarks` (`T/⚓️landmarks.ts:44`, `:117`) compare with a bare `>`. They were left alone: their distances come from square roots, so an exact tie is not expected, and importing `exceeds` there would close an import circle.
- The reference environment of the Python oracles was built on this host on first use (`~/.cache/semio/staffage-oracles`, 183 MB on `/home`, by the test helper of `S`).

## 6. Files

In `W` (kept): the six patches `01-…` to `06-…`; `tests/` with six case folders (`🔺️prism-union`, `🧊️prism-depth`, `⚖️tolerance-tie`, `🕳️blank-outline`, `✂️section-label`, `🪞️slender-in-band`; each a `🥒️.feature`, a `🟦️.ts`, a `🔣️.json`, three with a `🐍️.py`); `mirror.sh` and `run-test.sh` (a patched copy outside the worktree and the tests on it); the probes `union-assets.ts`, `union-inputs.ts`, `union-probe.ts`, `depth-probe.ts`, `order-compare.ts`, `measure-tie.ts`, `measure-blank.ts`, `language-probe.ts`, `inband-scan.ts`, `findings-probe.ts` and `lib.ts`.

Deleted at the end: `U/🗑️generated/linux-s2` (logs and scan rows of this lane) and the mirrors in the scratch folder.

To repeat a measurement: `M=$(sh W/mirror.sh <scratch> <name> <patch…>)`, then `sh W/run-test.sh "$M" <case…>` or `ROOT="$M" bun W/<probe>.ts …`.
