# 🔭️ Multiview Coverage Matrix: Diagnosis and Repair Plan

Measured on 2026-11-24, 09:03 to 12:20, on the shared worktree (10 cores, 16 GB, load average 10 to 12 from two other verification jobs). Nothing under `S` and no asset was changed.

Paths: `R` = repo root, `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`, `T` = `S/🧪️tests/🔭️multiview-coverage`, `K` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️02/STAFFAGE-MULTIVIEW-TOP-FRONT-SIDE-COVERAGE`, `U` = this ticket folder. Scripts: `U/🧰️matrix/` (section 9). Line numbers are those of the files on 2026-11-24.

## 1. Result in short

- The matrix is far from clean, and the reason is the catalog, not the generator. In a stratified sample of 36 assets (4 per category) 34 finished: **3,669 of 8,463 cells fail (43 %), in 25 of 34 assets**. Extrapolated to the 94,038 cells of today's 375 assets: **about 32,000 to 38,000 failing cells (34 to 40 %)**.
- A cheap scan of the whole catalog (every pose of every authored scale, the same check functions as the matrix, 4,542 s) finds a hard finding in **249 of 375 assets**; 126 have none.
- Ten codes occur. Two families carry 94 % of the failing cells: cross-view synchrony (`sync-*`, 2,380 cells, 13 sample assets, 168 catalog assets) and `anchor-landmark` (1,855 cells, 14 sample assets, 133 catalog assets).
- Four causes lie in shared code and need no asset work (sections 5.1, 5.2, 5.4, 5.5). Together they clear 498 of the 3,669 failing sample cells (14 %) and make 22 more catalog assets clean (126 to 148).
- The rest is asset data: **227 assets need a change, 191 of them a drawing change** (158 synchrony, 37 band copies, overlap 4), 102 an anchor change, 20 a decision on a symmetric view. People are the bulk: 73 of 75.
- A full matrix run costs at least 36 hours of single-shard time today (humans and vegetation are 86 % of it), and a shard that is cut loses all its cells. The generator needs per-asset output before a full run is worth starting (section 8).
- The regression suite `🔭️multiview-coverage` (23 of 23) does not see any of this: its catalog cases pin a handful of named assets, and two of them (`standing-adult`, `walking-commuter`) have hard synchrony findings in the scan.

## 2. What a cell is

- **Key.** One cell is asset × scale entry × viewpoint × mode (`T/🧰️.ts:43-57`, key at `:131`).
- **Scale entries** (`T/🧰️.ts:137-151`): the four presets 1:500, 1:200, 1:100, 1:50 and every further declared key; for each authored representation the two ends of its `validRange` when they differ from the key (`<key>:band-min`, `<key>:band-max`); for each scale declared unavailable one entry per alternative (`<key>>alt<i>:scale|symbol|asset`).
- **Viewpoints and modes**: 3 × 7 = 21 cells per scale entry. An asset with four authored bands has 12 entries and 252 cells. The universe of today's catalog, enumerated with `universe` (`T/🧰️.ts:157-159`): **94,038 cells** (humans 18,900, vegetation 16,884, components 14,637, objects 14,511, furniture 9,807, appliances 9,660, machinery 4,032, animals 3,339, vehicles 2,268).
- **A cell is an aggregate.** Its subjects are every pose, every sampled instant of every clip (start, keys, mid-intervals, extremes of 48 probes, 8 uniform, end) and the transitions (every pose to the rest pose plus three seeded pairs at five fractions), all in seven modes, poses and key instants in English and German (`T/🧰️.ts:248-263`). Band ends take only the poses and the start and end of each clip (`:492-494`). `checked` counts the renders; a sample asset has 7,700 to 42,400.
- **Status** (`T/🧰️.ts:431-447`): `ok`; `unavailable` (declared with reason); `range-end` (the print profile refuses the band only at a range end, D30, `K/🗂️work-board.md:110`); `group-drift` (tier two of D41, reported only, `K/🗂️work-board.md:130-132`); `fail` (at least one named check failed in at least one subject). A cell nobody produced is `fail` with `cell-missing` (`:620`). Clean means no `fail`.
- **Fan-out.** A geometric finding belongs to the evaluated state, so it fails all seven modes of both views it compares (`T/🧰️.ts:311-318` pass `MODE_LIST`): 14 cells per scale entry. The same representation is checked again at its two range ends, and a `scale` alternative copies the cells of its target (`:574-576`). Sample: 1,186 failing preset cells, 1,139 at band-min, 1,167 at band-max, 177 in alternatives. One defect in one pose of an asset with four bands is therefore up to 168 failing cells; the number of failing cells is about three times the number of failing (asset, band, view, mode).

## 3. What the ticket demands

- `K/🎫️ticket.json:5`: every asset has a correct plan, front and side view across all poses, clips, transitions, supported scales, seven modes and consumers; "explicit missing-combination failures; coverage matrix, proof sheets, regression tests".
- `K/📋️brief.md:17`: a missing combination is a named failure, never a substitution. `:61`: "Handcraft, never derive. Rotating, mirroring or flattening the elevation is not a valid plan or front."
- Decisions that the matrix enforces (`K/🗂️work-board.md`): D18 one tolerance max(2 %, 15 mm) (`:93`); D30 range ends apart (`:110`); D33 and D34 a view reused unchanged in two bands is FAIL (`:113-114`); D41 per-bone extents of single-shape bones and the anchor point-in-shape test are hard FAIL, group extents are reported (`:130-132`); D45 one anchor per physical contact at its real x, y, z (`:153`).
- Last audit (`U/older-tickets.md:80-81`, `:91-101`): matrix and proof sheets open; no full run on record after v2 of 2026-11-02 (8,068 of 37,233).
- `S/README.md` states the drawing rules the findings rest on: three authored views per representation (`:130`), parts that turn need a `solid` in front and plan (`:141`), all three views agree in every pose (`:183`).

## 4. The sample

Four assets per category, at the 12.5, 37.5, 62.5 and 87.5 % quantiles of the asset document size (`U/🧰️matrix/sample.ts`), run two at a time, one per shard, with `timeout 1800` (`U/🧰️matrix/run-sample.sh`). Two assets did not finish within 1,800 s and left no cells: `person-with-arm-sling`, `weeping-willow`.

### 4.1 Cells by status

| Category | Assets | Cells | ok | unavailable | range-end | group-drift | fail | Failing assets |
|---|---|---|---|---|---|---|---|---|
| animals | 4 | 924 | 0 | 84 | 0 | 0 | 840 | 4 |
| appliances | 4 | 987 | 419 | 63 | 0 | 156 | 349 | 3 |
| components | 4 | 1,008 | 861 | 21 | 0 | 70 | 56 | 1 |
| furniture | 4 | 1,008 | 234 | 21 | 0 | 266 | 487 | 3 |
| humans | 3 | 756 | 119 | 0 | 42 | 49 | 546 | 3 |
| machinery | 4 | 1,008 | 667 | 42 | 21 | 35 | 243 | 3 |
| objects | 4 | 1,008 | 973 | 21 | 0 | 0 | 14 | 1 |
| vegetation | 3 | 756 | 140 | 0 | 0 | 231 | 385 | 3 |
| vehicles | 4 | 1,008 | 147 | 0 | 0 | 112 | 749 | 4 |
| **Total** | **34** | **8,463** | **3,560** | **252** | **63** | **919** | **3,669** | **25** |

Clean in the sample: electricity-meter, reclaimed-brick-pallet, used-cast-iron-sectional-boiler, used-district-heating-substation, used-coat-stand, oil-pumpjack, used-concrete-table-tennis, used-violin, used-wayfinding-signpost.

### 4.2 Failing cells by code

A cell can carry several codes. "Bands" sums the preset and its two range ends; "alt" are alternatives of an unavailable scale.

| Code | Cells | Assets | Viewpoints | Modes | preset / band-min / band-max / alt | Bands 50 / 100 / 200 / 500 (+alt) | Categories |
|---|---|---|---|---|---|---|---|
| `anchor-landmark` | 1,855 | 14 | elevation 315, front 987, plan 553 | all seven, 265 each | 609 / 581 / 581 / 84 | 455 / 483 / 476 / 357 (+84) | vehicles 553, animals 392, vegetation 259, appliances 224, machinery 175, furniture 168, humans 84 |
| `sync-bone-height` | 1,260 | 10 | elevation 630, front 630 | all seven, 180 each | 378 / 378 / 420 / 84 | 308 / 336 / 322 / 210 (+84) | animals 560, humans 294, furniture 280, appliances 70, vegetation 56 |
| `sync-bone-forward` | 1,190 | 9 | elevation 595, plan 595 | all seven, 170 each | 364 / 364 / 392 / 70 | 294 / 308 / 308 / 210 (+70) | animals 560, humans 336, furniture 238, vegetation 56 |
| `sync-height` | 1,022 | 8 | elevation 511, front 511 | all seven, 146 each | 322 / 294 / 322 / 84 | 266 / 266 / 238 / 168 (+84) | animals 476, furniture 168, vehicles 168, appliances 154, humans 56 |
| `sync-forward` | 868 | 7 | elevation 434, plan 434 | all seven, 124 each | 280 / 266 / 280 / 42 | 224 / 224 / 252 / 126 (+42) | animals 434, humans 196, furniture 140, vegetation 98 |
| `sync-bone-lateral` | 252 | 2 | front 126, plan 126 | all seven, 36 each | 84 / 84 / 70 / 14 | 84 / 84 / 70 / 0 (+14) | animals 140, vehicles 112 |
| `sync-lateral` | 182 | 2 | front 91, plan 91 | all seven, 26 each | 70 / 42 / 42 / 28 | 56 / 56 / 42 / 0 (+28) | animals 140, appliances 42 |
| `analysis-cue` | 108 | 4 | 36 each | moments only | 33 / 33 / 33 / 9 | 27 / 27 / 27 / 18 (+9) | furniture 39, appliances 36, machinery 33 |
| `derived-view-band-copy` | 77 | 4 | front 14, plan 63 | all seven, 11 each | 63 / 0 / 0 / 14 | 14 / 21 / 21 / 7 (+14) | machinery 35, appliances 14, furniture 14, objects 14 |
| `paper-width` | 56 | 1 | elevation 28, plan 28 | all seven, 8 each | 14 / 14 / 28 / 0 | 0 / 0 / 14 / 42 | components 56 |

Readings:

- No code depends on the mode except `analysis-cue` (moments). Every geometric code fails all seven modes alike: these are defects of the evaluated geometry, not of a depiction.
- The four bands are hit about equally. The defects sit in the rig, the anchors or in every band's drawing, not in one scale.
- By family (a cell counted once per family): synchrony 2,380 cells in 13 assets (1,589 with nothing else), anchors 1,855 in 14 (1,064 with nothing else), both together 791; cue 108; band copy 77; paper 56.
- Codes of the engine that did not occur: `cell-missing`, `cell-unchecked`, `representation-missing`, every `unavailable-*` and `alternative-*` declaration code, `symbol-*`, `<view>-empty`, `<view>-bounds`, `dimension-*`, the `scene-*` codes, `blank`, `blank-shapes`, `paper-height`, `resolve-error`, `state-identity`, `language-geometry`, `loop-closure`, `analysis-cue-legend`, and `derived-view-in-band` (which the catalog scan finds in 20 other assets, section 5.4).

### 4.3 Per asset

| Category | Asset | Cells | Fail | Codes (cells) | Seconds |
|---|---|---|---|---|---|
| animals | hedgehog | 210 | 168 | sync-bone-forward 112, sync-bone-height 112, sync-height 112, anchor-landmark 91 | 28 |
| animals | mallard-duck | 231 | 210 | sync-bone-forward 140, sync-bone-height 140, sync-forward 140, sync-height 140, anchor-landmark 70 | 541 |
| animals | red-fox | 231 | 210 | sync-bone-forward 140, sync-bone-height 140, sync-bone-lateral 140, sync-lateral 140, sync-height 140, sync-forward 126, anchor-landmark 98 | 79 |
| animals | roe-deer | 252 | 252 | sync-bone-forward 168, sync-bone-height 168, sync-forward 168, anchor-landmark 133, sync-height 84 | 258 |
| appliances | electric-fan-heater | 231 | 70 | anchor-landmark 70 | 14 |
| appliances | electricity-meter | 252 | 0 | | 25 |
| appliances | hot-water-cylinder | 252 | 48 | analysis-cue 36, derived-view-band-copy 14 | 23 |
| appliances | used-tumble-dryer | 252 | 231 | sync-height 154, anchor-landmark 154, sync-bone-height 70, sync-lateral 42 | 34 |
| components | reclaimed-brick-pallet | 252 | 0 | | 47 |
| components | used-cast-iron-sectional-boiler | 252 | 0 | | 20 |
| components | used-district-heating-substation | 252 | 0 | | 58 |
| components | used-steel-oil-storage-tanks | 252 | 56 | paper-width 56 | 58 |
| furniture | used-coat-stand | 252 | 0 | | 32 |
| furniture | used-coffee-table | 252 | 252 | sync-bone-height 168, sync-height 168, anchor-landmark 168, sync-bone-forward 98, derived-view-band-copy 14 | 24 |
| furniture | used-high-chair | 252 | 199 | sync-bone-forward 140, sync-forward 140, sync-bone-height 112, analysis-cue 3 | 24 |
| furniture | used-sun-lounger-bench | 252 | 36 | analysis-cue 36 | 22 |
| humans | office-worker-carrying-laptop | 252 | 252 | sync-bone-forward 168, sync-bone-height 168, sync-forward 168, anchor-landmark 84, sync-height 56 | 121 |
| humans | person-with-arm-sling | | | not finished in 1,800 s | >1,800 |
| humans | standing-desk-worker | 252 | 126 | sync-bone-height 126 | 402 |
| humans | teenager-with-headphones | 252 | 168 | sync-bone-forward 168, sync-forward 28 | 1,199 |
| machinery | finned-heat-exchanger | 252 | 33 | analysis-cue 33 | 16 |
| machinery | oil-pumpjack | 252 | 0 | | 244 |
| machinery | sorption-wheel | 252 | 35 | derived-view-band-copy 35 | 24 |
| machinery | wheel-loader | 252 | 175 | anchor-landmark 175 | 216 |
| objects | used-climbing-net-pyramid | 252 | 14 | derived-view-band-copy 14 | 280 |
| objects | used-concrete-table-tennis | 252 | 0 | | 23 |
| objects | used-violin | 252 | 0 | | 106 |
| objects | used-wayfinding-signpost | 252 | 0 | | 34 |
| vegetation | common-ash | 252 | 210 | anchor-landmark 210 | 514 |
| vegetation | red-oak | 252 | 49 | anchor-landmark 49 | 224 |
| vegetation | rosemary-bush | 252 | 126 | sync-forward 98, sync-bone-forward 56, sync-bone-height 56 | 261 |
| vegetation | weeping-willow | | | not finished in 1,800 s | >1,800 |
| vehicles | cargo-bike | 252 | 252 | anchor-landmark 252 | 304 |
| vehicles | motor-scooter | 252 | 203 | anchor-landmark 203 | 263 |
| vehicles | tram | 252 | 112 | sync-bone-lateral 112 | 118 |
| vehicles | vintage-car-1929 | 252 | 182 | sync-height 168, anchor-landmark 98 | 83 |

## 5. Root cause per code

Each cause names where it lies (matrix, shared module or asset data), the code path, one measured cell, and the count in the catalog. Catalog counts come from `U/🧰️matrix/catalog-scan.ts`: the matrix's own functions (`evaluate`, `extentOf`, `boneSync`, `anchorLandmarks`, `derivedFlags`, `resolveViews`) over every pose of every authored scale of all 375 assets, plus the forces and moments scene of the default pose. It leaves out clip instants, transitions, range ends and symbols, so its asset counts are lower bounds. In the sample it found every geometric family the matrix found in all 34 assets; the only two differences are codes it does not measure (`paper-width`, the cue of a symbol).

### 5.1 `paper-width`: defect of the matrix

- **Where.** `T/🧰️.ts:364-366` builds the set of drawn ids as `shape:<id>` and `sceneBox` (`:113-128`) measures only those paths. `S/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts:182` gives that id to the fill of a filled closed shape; an unfilled shape is drawn as `outline:<id>:<n>` (`:205`). A drawing whose outermost part is unfilled is therefore measured too small.
- **Measured** (`paper-probe.ts used-steel-oil-storage-tanks elevation`): at 1:500 the dimensions give 3.50 mm, the evaluated extent is 3.50 mm, the `shape:` paths span 3.28 mm, the `shape:` and `outline:` paths together 3.50 mm. The missing 0.11 m is `line-kit`, an unfilled body rect. Same at 1:350 (4.69 against 5.00 mm) and 1:1000, and in the plan. At 1:50 to 1:200 the pipework is filled and the check passes.
- **Count.** 56 cells, 1 of 34 sample assets. The scan does not measure it, so the catalog count is open; by the sample rate a few hundred to about 2,000 cells.
- **Repair.** Shared code, no asset work: the paper box of `checkScene` reads the fill and the outline runs of the shapes of the view.

### 5.2 `analysis-cue`: shared structural view, and a rule the matrix has for thermal only

- **Where.** `T/🧰️.ts:417-421`: a forces or moments scene without a hatch pattern and without a dashed stroke fails; only the thermal mode may be unhatched when its legend says so (`:420`). In the moments view a pattern exists only as the fill of a member shape whose value is above 15 % of its side of the range (`S/🔨️modules/🎨️depiction/🏗️structural-view/🟦️.ts:26`, `:122-124`), and only for bones that are eligible (a section or at least 5 % of the height, `:81-85`). Joint moments are drawn solid (`MOMENT_DASH.joint` is undefined, `:50`); only reaction and applied moments are dashed. The bending-moment diagram (`drawDiagram`, `:419`) carries neither.
- **Two measured cases** (`cue-probe.ts`, 1:100, default pose, moments):
  - `hot-water-cylinder`: the only loaded bone is `base` (66.03 N·m), 0.06 m long without a section, so not eligible; every eligible bone reads 0.000 N·m and the legend runs from -1.00 to 1.00. Patterns 0, dashes 0 in all three views. Nothing strong exists to hatch.
  - `used-sun-lounger-bench`: the stringers carry -614.5 and -191.2 N·m and the diagram is drawn (peak 614.5), but the filled member shapes of the view are the two footings at -14.7 N·m, 2.4 % of the range. Patterns 0, dashes 0. Here a reader without colour gets no cue although a strong moment is on the sheet.
- **Count.** Sample: 108 cells in 4 assets, moments only, all bands and range ends, one case on a symbol alternative (`used-high-chair`). Catalog, default pose: 12 assets (finned-heat-exchanger, hot-water-cylinder, market-parasol, pendant-lamp, reclaimed-steel-stair, shopping-trolley, storage-crate-stack, tealight, tilt-window, used-bollard-light, used-brine-water-heat-pump, used-sun-lounger-bench); the forces mode did not lack its cue in any asset at the default pose. Extrapolated about 500 to 900 cells.
- **Repair.** Shared code, no asset work, two parts:
  1. `structural-view`: the bending-moment diagram and the shear and axial diagrams get a cue of their own (hatch by sign, as the member fills have), so a strong value is never colour-only.
  2. `T/🧰️.ts`: the unhatched-legend rule becomes the same for the three analysis modes; the member legend already states the 15 % threshold in both languages (`structural-view/🟦️.ts:108-110`). A regression case beside `T/🟦️.ts:324-354` pins one asset of each kind.

### 5.3 `derived-view-band-copy`: asset data

- **Where.** `T/🧰️.ts:509-521` with `T/🪞️derived.ts:84-99`: the front or plan of the default pose equals the same view of another authored band (Hausdorff distance of the unit-normalised point sets below 0.005, identity only). Decision D34 makes this a failure.
- **Measured.** `hot-water-cylinder` 1:500 plan against 1:200 plan: distance 0; the two shape lists are byte-identical in the asset document. `sorption-wheel`: the plan is one shape at 1:50, 1:100 and 1:200. `used-climbing-net-pyramid`: all 321 plan shapes of 1:100 have the geometry of a 1:50 shape.
- **Count.** Catalog, exact for the preset entries: **37 assets, 109 band-view entries, 763 preset cells** (plus the copies in alternatives); components 11, furniture 10, objects 7, appliances 4, machinery 4, vegetation 1. Fourteen of them are byte-identical lists (`static-copies.py`). Field: `representations.<key>.geometry.front|plan`.
- **Repair.** Per asset, by hand: redraw the copied view of one of the two bands so that the finer one shows what its scale can print. No shared-code change; the validator's `representation-copied` (`S/🔨️modules/🗂️catalog/🟦️.ts:567-572`) compares whole representations and stays silent for a single copied view, which is why these assets pass the catalog check.

### 5.4 `derived-view-in-band`: fixture data, with a decision per asset

- **Where.** Same function; the front or plan equals the elevation, or the plan the front, under identity, a mirror or a quarter turn (distance below 0.03). Allowed when `S/🧫️fixtures/🔭️multiview-coverage/🔣️.json` names the asset and view with a physical reason. `derivedViewAllowlist` is an empty list (line 50) and has been since the file was last committed (2026-11-03).
- **Count.** None in the sample. Catalog: **20 assets, 37 band-view entries, 259 preset cells**: boxwood-ball, ceiling-diffuser, decentralised-hrv-fan, duct-fan, etics-brick-wall-12, etics-brick-wall-30, european-larch, glulam-beam, norway-spruce, pendant-lamp, rowan, saucer-magnolia, solid-brick-wall-1960, steel-column, tealight, used-bollard-light, used-park-lantern, used-playground-roundabout, waste-bin, windmill-palm.
- **Repair.** A body of revolution has the same front and side by nature: one allowlist line with its reason (lamps, bollard, ball, diffuser, tealight, bin, column, roundabout: about 9). The others are drawings to tell apart: five trees at 1:500 and 1:200 (european-larch, norway-spruce, rowan, saucer-magnolia, windmill-palm; distance 0.018 to 0.028, a crown drawn the same from two sides), the three walls and the beam (plan equal to elevation), the two fans (plan a quarter turn of the front). About 11 assets of hand work.

### 5.5 `anchor-landmark`: one cause in the matrix, two in the asset data

- **Where.** `T/⚓️landmarks.ts:87-102`: every contact, attachment and emitter anchor must lie on the drawn shapes of **its own bone** (`views[view].get(anchor.bone)`, `:94`) within D18, in every view in which that bone is drawn.
- **Catalog.** 133 assets, 6,866 findings over the poses. By where the anchor lies instead (`catalog-scan.ts`, same tolerance):

| Where the anchor lies | Findings | Share | Assets with such a finding |
|---|---|---|---|
| on a shape of a descendant bone | 3,930 | 57 % | 31 assets have only this kind |
| on a shape of another bone | 2,158 | 31 % | |
| off every drawn shape | 778 | 11 % | 68 |

- **Cause A, matrix: the anchor is bound to the parent of the part that touches.** `wheel-loader`: contact `contact-r-l` is bound to `rear-frame`, the wheel is drawn on the wheel bone below it; 1,020 mm off the shapes of `rear-frame` in the elevation, on the wheel. `cargo-bike` `contact-r` on `frame` (328 mm), `motor-scooter` `contact-f` on `steer` (232 mm), `vintage-car-1929` `contact-f-l` on `axle-f` (339 mm). The binding is right: a contact bound to the spinning wheel bone would orbit with it. The test is too narrow. With the shapes of the bone and its descendants, 31 assets lose the code entirely (humans 11, objects 5, machinery 4, vegetation 3, vehicles 3, components 2, animals 1, appliances 1, furniture 1) and 57 % of the findings go. In the sample this clears wheel-loader (175 cells) and red-oak (49).
- **Cause B, asset data: the anchor is bound to a bone other than the one that draws its place.** `common-ash`: the transpiration emitters `tp-sun-*` and `tp-shade-*` sit in the crown at up to 3,564 mm from the shapes of their branch bone, on the foliage of another bone. `roe-deer` `contact-fn` is 50 mm off `hoof-fn` in the plan. Field: `anchors[].bone`, or the shape's `bone`.
- **Cause C, asset data: the anchor lies on no drawn shape.** Three kinds:
  - no `z` (D45): `electric-fan-heater` `foot-back` and `foot-front` have no `z` and sit at the centre line, 16 mm off the body in the front while the feet are drawn at ±0.06 to 0.09 m. 58 assets have a failing landmark anchor without `z`; 215 assets have at least one such anchor at all (`static-flat.py`).
  - a consequence of 5.6: the front or plan shape does not follow its bone in a pose, so the anchor that does follow leaves it (`used-coffee-table` `table-foot-fl` 248 mm in `turning-key`; `red-fox` `nose` 67 to 155 mm off the head in the front). 92 of the 133 assets also have a synchrony finding.
  - an anchor in the air (emitters beside the shape).
- **Count.** Sample: 1,855 cells, 14 assets, front 987, plan 553, elevation 315. Left for data work after cause A: **102 assets** (humans 26, vegetation 25, animals 10, appliances 10, furniture 9, objects 8, machinery 5, vehicles 5, components 4); 32 of them have no synchrony finding and are anchor data alone.
- **Repair.** Cause A in `T/⚓️landmarks.ts`, with a vector. Causes B and C per asset: rebind, add `z` and split lumped anchors (D45, the structural fixtures follow), or follow 5.6.

### 5.6 `sync-*` and `sync-bone-*`: asset data, one part in the matrix

- **Where.** `T/🧰️.ts:311-313` compares the extents of the three views of one state (forward: elevation x with plan x; height: elevation y with front y; lateral: front x with plan y); `T/⚓️landmarks.ts:34-53` does the same per bone when the bone has exactly one shape in both views (hard), else as `group-drift` (soft). Tolerance D18 of the largest dimension.
- **How the engine places the shapes.** An elevation shape is in the rig plane. A front or plan shape without a `solid` is a flat sheet carried at one depth by the transform of its bone (`S/🔨️modules/🧮️evaluation/🟦️.ts:336-347`, depth rule at `:536`, `placeProjected` at `:594-595`). When the bone turns, the sheet foreshortens or shows its edge. `S/README.md:141` therefore requires a `solid` for every part that turns. The engine is right; the drawings do not follow the rule.
- **Catalog.** 168 assets, 10,151 hard per-bone findings and 651 extent findings over the poses. Per-bone findings by pose and by the construction of the front or plan shape:

| Pose | Front or plan shape | Same shape id in both views | Other shape id |
|---|---|---|---|
| other than rest | flat | 3,743 (102 assets) | 766 (30) |
| other than rest | `followTip` | 2,118 (11) | 611 (8) |
| other than rest | solid | 1,967 (62) | 476 (12) |
| rest | flat | 102 (29) | 107 (21) |
| rest | `followTip` | 36 (8) | 33 (6) |
| rest | solid | 172 (40) | 20 (6) |

- **Cause D, asset data: a flat front or plan shape on a bone that a pose turns** (4,509 findings, 44 %). `used-coffee-table`, pose `turning-key`: bone `table`, elevation y [0.614, 1.214], front y [0.914, 0.914]: the front shape has become a line; whole figure 1.214 against 0.914 m, 300 mm. `roe-deer` `grazing` to `alert`: neck 539 mm, front y [-0.215, 1.124] against elevation [0.000, 0.778]. `vintage-car-1929`: the front tyres are flat rects on the spinning wheel bones (`angleDeg` 1858), front y starts at 0.183 instead of 0. `used-tumble-dryer` and `used-high-chair`: the carry pose tilts the unit 8 degrees. In the documents, 210 assets have a flat front or plan shape on a bone turned 5 degrees or more by a pose and 147 assets have no solid in front or plan at all (`static-flat.py`).
- **Cause E, asset data: a `followTip` shape that does not reach where the elevation limb goes** (2,729 findings, 27 %, in 14 assets, mostly the quadrupeds and birds). `roe-deer`: 590 forward findings on `followTip` shapes, `forearm-ff` elevation x [0.492, 0.597] against plan [0.504, 0.623].
- **Cause F, asset data: a solid whose profile is not the elevation shape** (2,443 findings in other poses, 24 %; 40 assets already at rest). `standing-adult` `relaxed`: neck, elevation y [1.432, 1.558], front solid [1.376, 1.597], 56 mm. `barista`, `walking-commuter` (thigh, front x [-0.060, 0.212] against plan [-0.003, 0.197]). These are the two reference people of the regression suite.
- **Cause G, asset data: the views draw different extents for one bone at rest** (470 findings at rest, 5 %, 74 assets). `office-worker-carrying-laptop` 1:500 `standing`: `arm-r` elevation x [-0.101, 0.070], plan [-0.152, 0.129]. `teenager-with-headphones`: `head` plan reaches 45 mm behind the elevation head.
- **Cause H, matrix: the per-bone test compares two different parts** (2,013 of the findings above, 20 %, across causes D to G). `tram` 1:200: bone `gangway` has one shape in the front (`articulation-arm`, x [-0.160, 0.160]) and one in the plan (`gangway`, y [-1.000, 0.999]); 840 mm, 112 cells, although each part is drawn right and the rest of the gangway is hidden in the front (D40). The hard tier should hold for a shape and its twin of the same id; a bone whose single shapes carry different ids belongs to the reported tier. With that, 10 assets lose their synchrony findings entirely (apple-tree, city-bus, dining-table, evaporative-humidifier, olive-tree, shopping-trolley, spiral-stair, tealight, tram, used-bubbler-stone-fountain). D41 words the rule per bone, so this is a decision of the ticket owner.
- **Count.** Sample: 2,380 cells in 13 assets. Catalog after cause H: **158 assets** (humans 73, vegetation 27, animals 13, objects 13, furniture 12, appliances 8, components 8, machinery 2, vehicles 2). 74 of them are wrong already in the rest pose (humans 53, animals 11, vegetation 9, furniture 1); 84 only in other poses.
- **Repair.** Per asset, by hand: front and plan shapes of turning bones become solids whose profile is the elevation shape; `followTip` limbs are corrected or become solids; rest-pose extents are made equal. No shared-code change besides cause H.

## 6. Estimate for the whole catalog

Two estimates of the failing cells agree: the sample rate per category times the category universe gives 37,110; the scan's 249 failing assets times the mean of 147 failing cells per failing sample asset gives about 36,600; with the category means instead, 32,000. The sample is small for components and objects (one failing asset each, while the scan finds 25 and 32), so the per-category split is rough.

| Cause | Lies in | Sample cells | Catalog assets (scan) | Catalog cells (estimate) |
|---|---|---|---|---|
| D to G synchrony of the drawings | asset data | 2,268 | 158 | 24,000 to 25,000 |
| H per-bone test on different parts | matrix | 112 | 10 cleared | about 1,200 |
| B, C anchors bound or placed wrong | asset data | 1,631 | 102 | 12,000 to 13,000 (about 8,000 without a synchrony failure in the same cell) |
| A anchor on a descendant bone | matrix | 224 cleared fully | 31 cleared | about 1,800, more inside assets that keep other anchor findings |
| band copy | asset data | 77 | 37 | 763 preset cells, exact; with alternatives about 900 |
| in-band derived | fixture and asset data | 0 | 20 | 259 preset cells, exact |
| moments cue | structural view and matrix | 108 | 12 at the default pose | 500 to 900 |
| paper box without outlines | matrix | 56 | not measured | a few hundred to 2,000 |

After the four shared-code repairs (5.1, 5.2, cause A, cause H) the sample has 3,171 failing cells in 19 assets instead of 3,669 in 25, and the catalog scan 227 assets with a finding instead of 249.

Hand work, counted from the scan (lower bounds, poses only):

| Work | Assets | By category |
|---|---|---|
| Drawing change (synchrony or band copy) | 191 | humans 73, vegetation 28, objects 20, components 19, furniture 19, animals 13, appliances 11, machinery 6, vehicles 2 |
| of it synchrony | 158 | see 5.6 |
| of it band copy | 37 | see 5.3 |
| Anchor change | 102 | see 5.5; 32 need nothing else |
| Symmetric view: allowlist line or redraw | 20 | about 9 lines, about 11 redraws |
| Any asset or fixture change | 227 | humans 73, vegetation 43, objects 26, components 21, furniture 19, appliances 18, animals 13, machinery 8, vehicles 6 |
| None | 148 | components 37, objects 32, vegetation 24, appliances 21, furniture 20, machinery 8, vehicles 3, humans 2, animals 1 |

The per-asset list is appendix A.

## 7. What a run costs

- **Sample, measured per asset** (one shard each, two shards at a time, host at load 10 to 12): 14 to 1,199 s for the 34 that finished, sum 5,719 s; two cut at 1,800 s; median of the 36: 81 s. Means per category: appliances 24 s, furniture 26 s, components 46 s, objects 111 s, machinery 125 s, vehicles 192 s, animals 227 s, vegetation at least 700 s, humans at least 881 s.
- **Throughput** varies by a factor of 60: `electric-fan-heater` 754 checked renders per second, `teenager-with-headphones` 12. The number of renders does not predict the time (`office-worker-carrying-laptop`: 24,318 renders in 121 s; `teenager-with-headphones`: 14,280 in 1,199 s). One shard held 1.7 GB.
- **Full catalog**, category means times today's counts: **at least 130,800 s, 36 hours of single-shard time**, humans 66,000 s and vegetation 46,900 s of it. With `--jobs 2` on this host under today's load at least 18 hours; with 8 shards on a free 10-core host about 4.5 hours at these rates, and 8 shards of 1.7 GB do not fit beside anything else in 16 GB.
- **Fragility.** A shard writes its cells once, when all its assets are done (`T/📊️matrix.ts:38`); a cut or killed shard leaves nothing and its assets become `cell-missing`. A crash inside one asset drops that asset silently into `cell-missing` (`:32-36`). The audit of 2026-11-23 lost a seven-minute run this way, this diagnosis two assets.
- **The cheap gate.** `catalog-scan.ts` over all 375 assets: 4,542 s in one process (12.1 s per asset; vegetation 34.9, humans 17.7, the rest 1.7 to 8.8). It reproduces the geometric families of the matrix per asset and is the right loop for repair work; the matrix of one repaired asset (`--assets <id>`) confirms.

## 8. Ordered plan to a clean matrix

1. **Decide and land the four shared-code repairs** (one worker, the owner of `T` and of the structural view; no asset is touched):
   1. paper box reads fills and outline runs (`T/🧰️.ts:364-366`, 5.1);
   2. diagram cue in `structural-view/🟦️.ts` and one unhatched-legend rule for the three analysis modes in `T/🧰️.ts:417-421` (5.2);
   3. anchor test against the shapes of the bone and its descendants (`T/⚓️landmarks.ts:94`, cause A);
   4. per-bone hard tier for twins of one id only (`T/⚓️landmarks.ts:37-47`, cause H; needs the owner's word because D41 is worded per bone).
   Each with a vector in `S/🧫️fixtures/🔭️multiview-coverage/🔣️.json`, a scenario in `T/🥒️.feature` and a case in `T/🟦️.ts`. Effect: 14 % of the failing sample cells, 22 catalog assets clean.
2. **Make the generator survive a long run** (`T/📊️matrix.ts`): one cell file per asset written when the asset ends, a rerun that skips finished assets, a merge of shards produced elsewhere (`--shard i/n` exists; the merge only runs in the process that spawned the shards), a crash of one asset recorded as a named failure with its message. Without this, step 6 is a 36-hour run that one timeout voids.
3. **Promote the scan to the authors' gate**: the function set of `catalog-scan.ts` as a target of the suite, so that a worker checks one asset in seconds. Until then the script in `U/🧰️matrix/` serves.
4. **Anchor data, 102 assets** (mechanical per asset, no redraw): rebind anchors to the bone that draws their place, add `z` and split lumped contacts (D45). Do the 32 anchor-only assets first; the 70 others together with their drawings in step 5. Structural fixtures that pin an anchor follow.
5. **Drawings, 191 assets**, in this order, each batch closed with the scan and with `matrix.ts --assets` on the batch:
   1. band copies, 37 assets, one view of one band each (small, independent);
   2. symmetric views, 20 assets: about 9 allowlist lines with a physical reason, about 11 redraws;
   3. the two reference people `standing-adult` and `walking-commuter` (cause F), because the regression suite and the other people copy their construction;
   4. people, 71 more: solids from the elevation profile on every turning bone, rest-pose extents equal;
   5. animals 13 (`followTip` limbs, cause E) and vegetation 27 (sway poses);
   6. the `used-*` furniture, appliances, components and objects of 2026-11-22 (carry and turn poses on flat shapes), and the 2 vehicles and 2 machines.
   These are the same assets and the same shapes the silhouette repair of this week works on: 74 asset documents changed today while this was measured. The two repairs must run as one pass per asset, or the second undoes the first.
6. **Full run** on the survivable generator, sharded over the fleet, humans and vegetation on their own shards; then `T/🖨️proofs.ts` for the proof sheets the ticket also demands (not run here, cost not measured); record the summary in `K/📦️handoff.md`.
7. **Close** `K` when the summary shows no `fail` and the sheets exist.

## 9. Scripts and reproduction

All in `U/🧰️matrix/`, run with the pinned bun from `T` as working directory, one process at a time:

| Script | Purpose |
|---|---|
| `sample.ts <S/🗂️catalog>` | picks the stratified sample |
| `run-sample.sh <R> <out> [limit]` | runs `📊️matrix.ts` over the sample, two assets per call, and records the seconds per asset |
| `classify.ts <out> <T/🧰️.ts>` | tables of section 4 and the category extrapolation |
| `sample-facts.py <out>` | cells by scale kind and family, renders against seconds |
| `catalog-scan.ts <S/🧪️tests> <scan.jsonl> [ids]` | the whole-catalog scan of section 5 (resumes) |
| `scan-summary.py`, `cross.py`, `appendix.py` | tables of sections 5 and 6 and appendix A |
| `cue-probe.ts`, `paper-probe.ts` | the measured examples of 5.1 and 5.2 |
| `static-copies.py`, `static-flat.py` | counts from the asset documents alone |

The raw output (`U/🗑️generated/matrix-diagnosis`) is deleted; the sample takes 1 h 52 min to reproduce, the scan 76 min.

## 10. Limits

- 34 assets are 9 % of the catalog; the cell estimates carry that uncertainty, the per-category ones more. The asset counts of the scan are exact for what it measures and lower bounds for the matrix.
- The two unfinished assets belong to the two most expensive categories; their cost and their cells are missing from every sample number.
- The catalog moved during the measurement (74 asset documents carry today's date). Sample and scan are snapshots of 09:03 to 10:55 and 11:02 to 12:18.
- `paper-width` was found in one asset and is not counted in the catalog. `analysis-cue` is counted at the default pose only.

## Appendix A. Assets with a hard finding in the catalog scan

Findings over the poses of all authored scales. Sync: per-bone findings at rest / in other poses, the construction of the front or plan shapes concerned, how many compare different shape ids, and the extent findings. Anchors: findings and where the anchor lies. Band copy and in-band: scale and view. Cue: scales without a moments cue at the default pose.

| Category | Asset | Sync (rest / posed findings; shapes) | Anchors (findings; where) | Band copy | In-band | Cue |
|---|---|---|---|---|---|---|
| animals | dairy-cow | 2 / 315; flat+followTip; 42 other id; 11 extent | 71; other bone 48, descendant 15, off 8 |  |  |  |
| animals | domestic-cat | 7 / 406; flat+followTip; 90 other id | 2; descendant 2 |  |  |  |
| animals | goat | 8 / 384; flat+followTip+solid; 299 other id; 5 extent | 19; off 9, other bone 8, descendant 2 |  |  |  |
| animals | hedgehog | 0 / 39; flat; 2 extent | 6; other bone 4, off 2 |  |  |  |
| animals | herring-gull | 26 / 174; flat+solid; 4 extent |  |  |  |  |
| animals | horse | 8 / 213; flat+followTip+solid; 86 other id | 4; other bone 2, off 2 |  |  |  |
| animals | labrador-dog | 2 / 541; flat+followTip; 18 other id; 35 extent | 30; descendant 21, other bone 6, off 3 |  |  |  |
| animals | mallard-duck | 0 / 57; flat; 15 extent | 6; off 6 |  |  |  |
| animals | pig | 14 / 300; flat+followTip; 10 extent | 18; other bone 12, off 6 |  |  |  |
| animals | pigeon | 3 / 152; flat+solid; 9 extent |  |  |  |  |
| animals | red-fox | 30 / 662; solid; 12 extent | 99; other bone 88, off 6, descendant 5 |  |  |  |
| animals | roe-deer | 8 / 928; flat+followTip+solid; 42 extent | 38; other bone 34, descendant 4 |  |  |  |
| animals | sheep | 10 / 319; flat+followTip; 7 extent | 8; off 8 |  |  |  |
| appliances | ceiling-diffuser |  |  |  | 100 front, 200 front, 50 front |  |
| appliances | decentralised-hrv-fan |  |  |  | 50 plan |  |
| appliances | duct-fan |  |  | 100 plan, 200 plan | 50 plan |  |
| appliances | electric-fan-heater |  | 18; off 18 |  |  |  |
| appliances | gas-hob |  | 3; other bone 3 |  |  |  |
| appliances | hot-water-cylinder |  |  | 200 plan, 500 plan |  | 100, 200, 50, 500 |
| appliances | kitchen-extractor-hood |  |  | 100 plan, 50 plan |  |  |
| appliances | pendant-lamp | 0 / 4; flat |  |  | 100 front, 200 front, 50 front | 100, 200, 50 |
| appliances | used-crt-television | 0 / 0; extent only; 16 extent |  | 200 plan, 500 plan |  |  |
| appliances | used-dishwasher | 0 / 6; flat; 1 other id; 3 extent |  |  |  |  |
| appliances | used-electric-bass-and-cabinet |  | 48; descendant 48 |  |  |  |
| appliances | used-electric-cooker | 0 / 0; extent only; 3 extent | 2; off 2 |  |  |  |
| appliances | used-electric-guitar-and-combo |  | 16; other bone 12, off 4 |  |  |  |
| appliances | used-electronic-drum-kit |  | 20; other bone 20 |  |  |  |
| appliances | used-electronic-organ |  | 24; off 21, other bone 3 |  |  |  |
| appliances | used-fridge-freezer | 0 / 9; flat; 6 other id; 3 extent | 118; descendant 48, other bone 47, off 23 |  |  |  |
| appliances | used-microwave-oven | 0 / 2; flat | 31; descendant 30, off 1 |  |  |  |
| appliances | used-tumble-dryer | 0 / 1; flat; 3 extent | 15; other bone 6, off 6, descendant 3 |  |  |  |
| appliances | used-washing-machine | 0 / 2; flat; 12 extent | 28; off 22, other bone 6 |  |  |  |
| components | curtain-wall-bay |  |  | 100 front, 100 plan, 200 front, 200 plan |  |  |
| components | etics-brick-wall-12 |  |  | 100 front, 200 front, 50 front, 500 front | 100 plan, 200 plan, 500 plan |  |
| components | etics-brick-wall-30 |  |  | 100 front, 200 front, 50 front, 500 front | 100 plan, 200 plan, 500 plan |  |
| components | external-venetian-blind |  |  | 100 plan, 200 plan, 500 plan |  |  |
| components | glulam-beam |  |  | 100 plan, 500 plan | 500 plan |  |
| components | reclaimed-cast-iron-column | 0 / 0; extent only; 4 extent |  |  |  |  |
| components | reclaimed-cobblestone-bag |  | 6; descendant 6 |  |  |  |
| components | reclaimed-precast-stair | 0 / 4; flat; 8 extent | 32; off 20, other bone 12 |  |  |  |
| components | reclaimed-steel-column |  |  | 100 plan, 50 plan |  |  |
| components | reclaimed-steel-stair |  |  |  |  | 500 |
| components | reclaimed-trapezoidal-sheet-stack | 0 / 8; flat; 8 other id; 8 extent |  |  |  |  |
| components | sectional-garage-door |  | 11; off 6, other bone 5 |  |  |  |
| components | sliding-door |  | 15; other bone 15 |  |  |  |
| components | solid-brick-wall-1960 |  |  | 100 front, 200 front, 200 plan, 50 front, 500 front, 500 plan | 100 plan, 200 plan, 500 plan |  |
| components | spiral-stair | 2 / 6; flat; 8 other id |  | 100 front, 100 plan, 200 front, 200 plan |  |  |
| components | stair-flight |  |  | 100 front, 100 plan, 200 front, 50 plan |  |  |
| components | steel-column |  |  | 200 plan, 500 plan | 100 front, 200 front, 500 front |  |
| components | tilt-window |  |  |  |  | 200 |
| components | used-air-water-heat-pump-monobloc | 0 / 0; extent only; 3 extent | 10; off 8, other bone 2 |  |  |  |
| components | used-brine-water-heat-pump |  |  |  |  | 100, 200, 50 |
| components | used-forced-draught-oil-burner | 0 / 1; flat |  |  |  |  |
| components | used-pellet-boiler | 0 / 96; flat; 8 extent |  |  |  |  |
| components | used-towel-radiator | 0 / 0; extent only; 9 extent | 36; descendant 36 |  |  |  |
| components | used-trench-convector |  |  | 100 front, 200 front, 500 front |  |  |
| components | used-wall-hung-gas-boiler | 0 / 0; extent only; 15 extent |  |  |  |  |
| furniture | bookshelf |  |  | 100 plan, 200 plan, 50 plan, 500 plan |  |  |
| furniture | cafe-chair | 0 / 1; flat; 1 extent |  |  |  |  |
| furniture | dining-table | 2 / 8; flat; 10 other id |  | 100 plan, 200 plan, 50 plan, 500 plan |  |  |
| furniture | office-swivel-chair | 5 / 30; flat; 35 other id; 4 extent |  |  |  |  |
| furniture | park-bench |  | 12; off 12 | 200 plan, 500 plan |  |  |
| furniture | single-bed |  | 25; descendant 15, off 10 | 100 plan, 50 plan |  |  |
| furniture | three-seat-sofa |  | 112; descendant 112 |  |  |  |
| furniture | used-bedside-table | 0 / 1; flat; 5 extent | 12; off 12 |  |  |  |
| furniture | used-coffee-table | 0 / 11; flat; 4 extent | 32; off 24, other bone 8 | 100 front, 200 front |  |  |
| furniture | used-extendable-dining-table |  |  | 100 front, 200 plan, 50 front, 500 plan |  |  |
| furniture | used-folding-bookcase-ladder |  |  | 200 front, 500 front |  |  |
| furniture | used-high-chair | 0 / 14; flat; 3 extent |  |  |  |  |
| furniture | used-hinged-wardrobe | 0 / 7; flat | 58; descendant 52, off 3, other bone 3 |  |  |  |
| furniture | used-kitchen-base-cabinet | 0 / 0; extent only; 1 extent |  |  |  |  |
| furniture | used-park-armchair | 0 / 8; flat; 4 extent | 8; other bone 4, off 4 |  |  |  |
| furniture | used-recliner-chair | 0 / 2; flat+solid; 4 extent | 12; off 8, descendant 4 |  |  |  |
| furniture | used-shoe-cabinet |  | 12; descendant 8, other bone 4 | 200 plan, 500 plan |  |  |
| furniture | used-sun-lounger-bench |  |  |  |  | 100, 200, 50, 500 |
| furniture | used-tv-sideboard | 0 / 0; extent only; 3 extent |  | 200 plan, 500 plan |  |  |
| furniture | used-wall-unit | 0 / 0; extent only; 4 extent | 4; off 3, descendant 1 | 100 plan, 200 plan, 500 plan |  |  |
| furniture | used-wingback-armchair | 0 / 0; extent only; 4 extent |  |  |  |  |
| humans | barista | 4 / 7; flat+solid; 2 extent |  |  |  |  |
| humans | basketball-player | 0 / 8; solid; 2 extent | 2; descendant 2 |  |  |  |
| humans | child-on-kick-scooter | 17 / 739; flat+followTip+solid; 756 other id; 12 extent | 47; descendant 47 |  |  |  |
| humans | construction-worker-with-shovel | 15 / 36; flat+solid |  |  |  |  |
| humans | courier-with-hand-truck | 0 / 13; flat+followTip; 3 other id | 24; off 16, descendant 8 |  |  |  |
| humans | cyclist | 3 / 22; flat+solid; 1 extent | 112; descendant 64, other bone 48 |  |  |  |
| humans | dancer | 5 / 47; solid; 38 other id; 3 extent | 9; descendant 5, other bone 4 |  |  |  |
| humans | doctor-with-clipboard | 2 / 20; flat |  |  |  |  |
| humans | e-scooter-rider | 5 / 60; solid | 128; descendant 128 |  |  |  |
| humans | elderly-person-knitting | 1 / 34; flat+solid |  |  |  |  |
| humans | elderly-with-cane | 3 / 135; flat+solid | 14; descendant 12, off 2 |  |  |  |
| humans | hiker-with-trekking-poles | 0 / 11; flat+followTip; 4 other id | 1; descendant 1 |  |  |  |
| humans | jogger | 0 / 16; flat+solid; 8 other id |  |  |  |  |
| humans | kneeling-gardener | 2 / 7; solid; 1 other id | 24; descendant 24 |  |  |  |
| humans | leaning-person | 1 / 29; flat+solid; 6 other id; 6 extent | 24; other bone 20, off 4 |  |  |  |
| humans | mobility-scooter-user | 1 / 1; flat | 64; descendant 32, off 32 |  |  |  |
| humans | nurse-walking | 2 / 241; flat; 37 extent | 11; descendant 9, other bone 1, off 1 |  |  |  |
| humans | office-worker-carrying-laptop | 2 / 182; flat; 14 extent | 24; descendant 18, off 4, other bone 2 |  |  |  |
| humans | painter-at-easel | 2 / 4; solid |  |  |  |  |
| humans | parent-carrying-toddler | 0 / 57; solid | 20; off 20 |  |  |  |
| humans | parent-with-child-on-shoulders | 2 / 74; flat; 6 extent | 20; descendant 12, off 8 |  |  |  |
| humans | parent-with-stroller | 6 / 83; flat+solid; 2 extent | 224; descendant 220, off 4 |  |  |  |
| humans | patient-with-iv-pole | 2 / 162; flat; 6 extent | 24; descendant 18, off 4, other bone 2 |  |  |  |
| humans | person-carrying-ladder | 8 / 108; flat+solid; 33 other id; 1 extent | 28; descendant 28 |  |  |  |
| humans | person-carrying-shopping-bags | 1 / 134; flat+solid; 7 other id; 4 extent | 18; descendant 18 |  |  |  |
| humans | person-checking-watch | 0 / 6; solid |  |  |  |  |
| humans | person-cooking | 0 / 2; solid; 1 extent |  |  |  |  |
| humans | person-drinking-coffee | 8 / 45; flat+solid; 10 other id; 3 extent |  |  |  |  |
| humans | person-eating-standing | 1 / 10; flat+solid; 3 other id; 3 extent |  |  |  |  |
| humans | person-hanging-laundry | 2 / 37; flat+solid |  |  |  |  |
| humans | person-in-rain-poncho | 0 / 68; flat; 18 extent | 24; descendant 18, off 4, other bone 2 |  |  |  |
| humans | person-in-winter-coat | 2 / 32; flat+solid; 4 extent | 18; descendant 18 |  |  |  |
| humans | person-on-crutches | 0 / 12; flat; 3 extent | 8; off 8 |  |  |  |
| humans | person-pushing-bicycle | 4 / 55; solid | 284; descendant 208, other bone 76 |  |  |  |
| humans | person-queuing | 2 / 8; solid | 1; descendant 1 |  |  |  |
| humans | person-signing | 5 / 72; flat+solid; 14 other id |  |  |  |  |
| humans | person-stretching | 6 / 38; flat+solid; 11 other id; 7 extent | 4; off 4 |  |  |  |
| humans | person-sweeping | 9 / 38; flat+solid; 1 extent |  |  |  |  |
| humans | person-taking-selfie | 3 / 6; solid; 1 extent |  |  |  |  |
| humans | person-with-arm-sling | 0 / 159; flat; 7 extent | 18; descendant 18 |  |  |  |
| humans | person-with-guide-dog | 3 / 61; flat+followTip+solid | 8; descendant 8 |  |  |  |
| humans | person-with-rollator | 0 / 31; flat+solid |  |  |  |  |
| humans | person-with-umbrella | 0 / 2; flat; 1 extent |  |  |  |  |
| humans | person-with-white-cane | 3 / 17; flat+solid; 2 extent |  |  |  |  |
| humans | photographer-with-camera | 0 / 29; flat+solid; 1 extent |  |  |  |  |
| humans | picnicker-sitting-on-ground | 12 / 22; flat+solid | 22; other bone 18, descendant 4 |  |  |  |
| humans | postal-carrier | 5 / 196; flat+solid; 26 extent | 24; descendant 18, off 4, other bone 2 |  |  |  |
| humans | pregnant-person | 0 / 14; flat+solid |  |  |  |  |
| humans | presenter-at-flipchart | 2 / 12; solid |  |  |  |  |
| humans | runner-with-blade-prosthesis | 2 / 161; solid; 2 extent |  |  |  |  |
| humans | schoolchild-with-backpack | 0 / 3; solid |  |  |  |  |
| humans | seated-guitarist | 0 / 5; flat+solid | 46; off 40, other bone 6 |  |  |  |
| humans | seated-office-worker | 2 / 23; flat+solid |  |  |  |  |
| humans | seated-reader | 4 / 46; flat+solid |  |  |  |  |
| humans | skateboarder | 0 / 96; solid; 1 extent | 140; other bone 128, descendant 10, off 2 |  |  |  |
| humans | sleeping-person | 1 / 30; flat+solid | 27; other bone 27 |  |  |  |
| humans | standing-adult | 4 / 14; flat+solid |  |  |  |  |
| humans | standing-desk-worker | 3 / 6; solid |  |  |  |  |
| humans | standing-phone-user | 1 / 8; solid |  |  |  |  |
| humans | standing-violinist | 4 / 12; solid |  |  |  |  |
| humans | surveyor-with-total-station | 6 / 14; flat+solid; 6 other id; 1 extent | 3; other bone 3 |  |  |  |
| humans | teenager-with-headphones | 4 / 0; flat+solid; 1 extent |  |  |  |  |
| humans | tennis-player | 1 / 8; solid; 1 extent |  |  |  |  |
| humans | tour-guide | 0 / 10; solid |  |  |  |  |
| humans | traveller-with-rolling-suitcase | 5 / 208; flat+solid; 24 extent | 32; descendant 21, other bone 7, off 4 |  |  |  |
| humans | waiter-with-tray | 4 / 11; flat+solid; 2 extent | 6; descendant 5, off 1 |  |  |  |
| humans | walking-commuter | 2 / 14; solid |  |  |  |  |
| humans | walking-toddler | 1 / 5; solid; 3 extent |  |  |  |  |
| humans | waving-person | 4 / 16; flat+solid |  |  |  |  |
| humans | wheelchair-user | 0 / 2; flat | 80; descendant 46, off 34 |  |  |  |
| humans | worker-carrying-box | 0 / 214; flat+solid; 3 extent |  |  |  |  |
| humans | worker-with-jackhammer | 9 / 23; flat |  |  |  |  |
| humans | worker-with-pallet-jack | 1 / 4; flat+followTip+solid; 4 other id | 33; descendant 25, off 6, other bone 2 |  |  |  |
| machinery | concrete-mixer-truck |  | 325; descendant 325 |  |  |  |
| machinery | diesel-generator | 0 / 0; extent only; 2 extent | 80; descendant 80 |  |  |  |
| machinery | evaporative-humidifier | 1 / 2; flat; 3 other id | 6; off 6 |  |  |  |
| machinery | finned-heat-exchanger |  |  |  |  | 100, 200, 50 |
| machinery | forklift | 0 / 0; extent only; 6 extent | 196; descendant 168, off 20, other bone 8 |  |  |  |
| machinery | heat-pump-outdoor-unit |  |  | 100 plan, 200 plan, 50 plan, 500 plan |  |  |
| machinery | plate-heat-exchanger |  | 9; off 9 | 100 plan, 200 plan |  |  |
| machinery | scissor-lift |  | 128; descendant 128 |  |  |  |
| machinery | scroll-compressor |  | 6; off 6 |  |  |  |
| machinery | solar-thermal-collector |  | 12; off 12 | 100 front, 100 plan, 50 front, 50 plan |  |  |
| machinery | sorption-wheel |  |  | 100 plan, 200 plan, 50 plan |  |  |
| machinery | wheel-loader |  | 170; descendant 170 |  |  |  |
| objects | bicycle-stand |  |  | 100 front, 100 plan, 200 front, 200 plan, 50 front, 50 plan |  |  |
| objects | charcoal-grill | 0 / 0; extent only; 3 extent | 3; off 3 |  |  |  |
| objects | market-parasol |  |  |  |  | 100, 200, 500 |
| objects | patio-heater |  |  | 100 plan, 200 plan |  |  |
| objects | shopping-trolley | 2 / 4; flat; 6 other id | 60; descendant 60 |  |  | 200 |
| objects | stepladder | 0 / 24; flat |  |  |  |  |
| objects | storage-crate-stack |  |  | 100 front, 200 plan, 50 front, 500 plan |  | 100, 200, 50, 500 |
| objects | tealight | 2 / 4; flat; 6 other id |  | 100 plan, 200 plan, 50 plan | 100 front, 200 front, 50 front | 100, 200, 50 |
| objects | used-basketball-hoop | 0 / 9; flat; 2 extent |  |  |  |  |
| objects | used-bicycle-shelter |  | 12; other bone 12 |  |  |  |
| objects | used-bollard-light |  |  | 200 plan, 500 plan | 200 front, 500 front | 100, 200, 50, 500 |
| objects | used-bubbler-stone-fountain | 2 / 8; flat; 10 other id |  |  |  |  |
| objects | used-cello | 0 / 0; extent only; 4 extent | 12; descendant 12 |  |  |  |
| objects | used-classical-guitar | 0 / 14; flat; 24 extent | 48; other bone 45, descendant 3 |  |  |  |
| objects | used-climbing-net-pyramid |  |  | 100 plan, 50 plan |  |  |
| objects | used-concert-harp | 0 / 6; flat | 96; descendant 80, other bone 16 |  |  |  |
| objects | used-djembe-and-cajon | 0 / 0; extent only; 3 extent |  |  |  |  |
| objects | used-double-bass |  | 22; off 16, descendant 6 |  |  |  |
| objects | used-double-swing | 0 / 4; flat |  |  |  |  |
| objects | used-drum-kit | 0 / 5; flat | 85; descendant 81, other bone 4 |  |  |  |
| objects | used-football-goal | 0 / 3; flat; 1 other id | 10; other bone 10 |  |  |  |
| objects | used-park-lantern |  |  |  | 500 front |  |
| objects | used-playground-roundabout |  |  |  | 200 front, 500 front |  |
| objects | used-sandpit-digger |  |  | 100 front, 50 front |  |  |
| objects | used-spring-rocker | 0 / 0; extent only; 6 extent |  |  |  |  |
| objects | used-timpani-pair |  | 36; descendant 36 |  |  |  |
| objects | used-upright-piano |  | 18; descendant 18 |  |  |  |
| objects | used-viola |  | 2; other bone 2 |  |  |  |
| objects | used-water-play-table | 0 / 0; extent only; 4 extent |  |  |  |  |
| objects | used-wc-pan-cistern | 0 / 3; flat; 9 extent |  |  |  |  |
| objects | used-xylophone |  | 100; descendant 100 |  |  |  |
| objects | waste-bin |  |  |  | 500 front |  |
| vegetation | apple-tree | 23 / 69; flat+followTip; 92 other id | 11; other bone 7, off 4 |  |  |  |
| vegetation | bamboo-clump | 0 / 0; extent only; 2 extent | 103; other bone 81, off 22 |  |  |  |
| vegetation | black-alder | 7 / 51; flat; 35 other id | 3; other bone 3 |  |  |  |
| vegetation | black-locust | 0 / 0; extent only; 4 extent | 20; descendant 20 |  |  |  |
| vegetation | boxwood-ball |  |  |  | 500 front |  |
| vegetation | common-ash |  | 236; other bone 170, descendant 54, off 12 |  |  |  |
| vegetation | common-lilac | 2 / 6; flat |  |  |  |  |
| vegetation | common-reed | 0 / 41; flat; 5 extent | 297; other bone 264, off 25, descendant 8 |  |  |  |
| vegetation | crab-apple |  | 10; off 9, other bone 1 |  |  |  |
| vegetation | dawn-redwood |  | 16; other bone 16 |  |  |  |
| vegetation | english-oak | 5 / 17; flat; 20 other id; 4 extent | 84; other bone 84 |  |  |  |
| vegetation | european-beech |  | 209; other bone 191, descendant 18 |  |  |  |
| vegetation | european-larch |  |  |  | 500 front |  |
| vegetation | field-maple | 0 / 1; flat; 3 extent | 14; other bone 11, off 3 |  |  |  |
| vegetation | fig-tree |  | 8; other bone 8 |  |  |  |
| vegetation | hawthorn | 28 / 166; flat; 167 other id; 6 extent | 13; other bone 13 |  |  |  |
| vegetation | hornbeam-hedge | 3 / 19; flat+solid; 12 other id; 6 extent |  |  |  |  |
| vegetation | horse-chestnut |  | 188; other bone 104, descendant 84 |  |  |  |
| vegetation | hosta-clump | 0 / 0; extent only; 5 extent |  |  |  |  |
| vegetation | hydrangea | 2 / 6; flat |  |  |  |  |
| vegetation | ivy-on-trellis | 8 / 31; flat; 1 extent | 19; other bone 19 |  |  |  |
| vegetation | japanese-maple |  | 10; other bone 10 |  |  |  |
| vegetation | maize-row |  | 211; other bone 108, off 103 |  |  |  |
| vegetation | miscanthus-grass | 0 / 11; flat; 4 extent | 100; descendant 78, other bone 12, off 10 |  |  |  |
| vegetation | multi-stem-birch | 0 / 1; flat |  |  |  |  |
| vegetation | norway-maple | 0 / 2; flat; 1 extent | 15; other bone 15 |  |  |  |
| vegetation | norway-spruce |  |  |  | 200 front, 500 front |  |
| vegetation | olive-tree | 22 / 66; flat+followTip; 88 other id | 11; other bone 7, off 4 |  |  |  |
| vegetation | pampas-grass | 0 / 54; flat; 9 extent | 43; descendant 37, other bone 5, off 1 |  |  |  |
| vegetation | potted-monstera | 0 / 4; flat |  |  |  |  |
| vegetation | red-oak |  | 12; descendant 12 |  |  |  |
| vegetation | rhododendron | 2 / 6; flat |  |  |  |  |
| vegetation | rosemary-bush | 0 / 2; flat; 2 extent |  |  |  |  |
| vegetation | rowan | 0 / 12; flat; 4 extent | 3; off 2, other bone 1 |  | 500 front |  |
| vegetation | saguaro-cactus |  |  | 200 plan, 500 plan |  |  |
| vegetation | saucer-magnolia |  | 2; other bone 2 |  | 500 front |  |
| vegetation | scots-pine | 0 / 13; flat |  |  |  |  |
| vegetation | small-leaved-lime | 19 / 95; flat; 18 other id | 221; descendant 132, other bone 89 |  |  |  |
| vegetation | stone-pine | 0 / 22; flat |  |  |  |  |
| vegetation | sunflower | 0 / 0; extent only; 4 extent |  |  |  |  |
| vegetation | sycamore-maple | 0 / 6; flat | 21; other bone 21 |  |  |  |
| vegetation | tulip-tree |  | 24; descendant 24 |  |  |  |
| vegetation | weeping-willow | 0 / 0; extent only; 8 extent | 10; other bone 10 |  |  |  |
| vegetation | windmill-palm |  |  |  | 200 front |  |
| vegetation | wisteria-pergola | 0 / 6; flat |  |  |  |  |
| vehicles | cargo-bike |  | 120; descendant 96, other bone 24 |  |  |  |
| vehicles | city-bicycle |  | 158; descendant 120, other bone 38 |  |  |  |
| vehicles | city-bus | 3 / 18; flat; 21 other id | 196; descendant 180, off 16 |  |  |  |
| vehicles | city-car | 0 / 0; extent only; 12 extent | 36; descendant 36 |  |  |  |
| vehicles | delivery-van |  | 20; descendant 20 |  |  |  |
| vehicles | electric-kick-scooter |  | 84; descendant 84 |  |  |  |
| vehicles | motor-scooter |  | 187; descendant 154, other bone 33 |  |  |  |
| vehicles | tram | 3 / 24; flat; 27 other id |  |  |  |  |
| vehicles | vintage-car-1929 | 0 / 0; extent only; 12 extent | 98; descendant 48, off 36, other bone 14 |  |  |  |
