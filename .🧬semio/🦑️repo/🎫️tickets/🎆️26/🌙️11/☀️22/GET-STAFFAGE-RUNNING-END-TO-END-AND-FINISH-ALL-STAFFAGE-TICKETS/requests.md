
## USED-HOUSEHOLD-OBJECTS-STAFFAGE-COLLECTION — structure solver: parts that are apart in a pose stay one rigid body

File: `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts` (`solveStructure`). Carried over from the ticket's `requests.md` (authors' measurements of 2026-11-22, not re-measured by the finisher).

`solveStructure` treats the whole asset as one instantaneously rigid body on equal springs. Storage aids that stay behind (blanket, pallet, depot table, floor tile) therefore share their reactions with the carried or set-up object.

- `used-coffee-table`, poses `carried` and `in-use`: the four glides of the 28.6 kg depot table and the hands / leg ends of the 22.6 kg coffee table 1.10 m ahead take a reaction distribution linear in x; hand calculation for `in-use` with the coffee-table weight only: about 8 % leaks to the near depot glides, front and rear legs 0.17 W against 0.29 W per leg although the centre of mass is central.
- `used-coat-stand`, pose `carried`: the grips report 61.0 N and 3.9 N, so they carry the 11.8 N of the 1.2 kg blanket lying on the floor (hand calculation for the 4.62 kg stand alone: 29.7 N and 15.7 N). With the blanket aside in a home pose the leak falsely stabilised the tipping check (9.4 N m against 9.3 N m), so the author keeps the blanket under the feet in the home poses.
- Same leak stated in `structure.assumptions` of `used-kitchen-base-cabinet` (3.2 kg blanket on the four grips), `used-high-chair` (floor tile on the hands), `used-double-bed` (carried mattress shared with the stack), `used-washing-machine` (pallet), `used-bedside-table` and `used-bunk-bed` (authors work around it with an upward load at the centre of the part left on the floor), `used-sewing-machine` (table and blanket against the carried case).

Proposal: solve each group of bones that is not connected by contact in the pose as its own rigid body, or let a pose declare `bodies: [[bones…], …]`. No work-around inside an asset is exact.

## USED-HOUSEHOLD-OBJECTS-STAFFAGE-COLLECTION — 2D plan of a tilted pose distorts flat plan bands of curved rods

File: `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts` (`carry` of front and plan shapes). Carried over (author's observation on `used-coat-stand`, pose `carried`, stand tilted 40 degrees; the finisher saw the same kind of offset noted for `used-bedside-table`, marks in the tilted carried plan).

Flat radial plan bands of curved rods (feet, hooks) take the depth of their elevation twin and are drawn distorted in the plan of the tilted pose. Proposal: a per-bone 3D shape (or `solid` placement) for such parts; within the schema only a `solid` per rod would draw them right.

## USED-HOUSEHOLD-OBJECTS-STAFFAGE-COLLECTION — volumes-silhouette: sub-cell gap counted at the top bin of the boundary histogram

File: `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts` (`measureBanded`, `boundarySpread`). Carried over (author's measurement on `used-kitchen-base-cabinet`, 1:200 front).

A real 2 mm gap between the plinth panel (y 0.014) and the blanket (y 0.012), drawn identically in 2D and 3D, gave `p95M:front` = `hausdorffM:front` = 0.1641 m (limit 0.12 m) with IoU 0.9994; 0.1641 m is exactly the top bin of the histogram (21 cells of 7.8 mm), which `boundarySpread` assigns when one side has no boundary inside a tile. Proposal: measure such cells against the coarse boundary instead of counting them at the top bin. Worked around in the asset by closing the gap at 1:200.

## USED-HOUSEHOLD-OBJECTS-STAFFAGE-COLLECTION — volumes-thermal-coverage with `envelope: true` closes real air gaps

File: `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌊️flood/🟦️.ts` and the thermal-coverage rule in `…/🧊️volumes/🧱️rules/🟦️.ts`. Carried over (author's measurement on `used-oil-filled-radiator`, 1:200).

Seven typified 30 mm fins over the true 0.358 m stack with 24.7 mm open gaps: with `envelope: true` the outer surface of all regions measures 0.990 m² against 2.688 m² (63 %, limit 30 %); without the flag thermal coverage and cross-scale pass. Proposal: apply the envelope hull to `volumes-cross-scale` and the volume band only, never to the flood of `volumes-thermal-coverage`. Worked around in the asset by not setting the flag.

## USED-HOUSEHOLD-OBJECTS-STAFFAGE-COLLECTION — volumes-thermal-coverage: a face resting on a closed neighbour of another region is not counted as outer

File: the thermal-coverage rule in `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts` (C29, C31 c). Carried over (author's measurement on `used-moving-boxes`, 1:50).

Region `carried-box` (closed 0.614 x 0.344 x 0.354 m box, outer surface 1.1007 m²) measures 0.898 m²: its 0.211 m² bottom resting on the lid of the box below (region `stacked-boxes`) is not counted because the covering lid faces only the closed cavity of its own box. Proposal: count a face covered by a solid of another region as outer whenever that region has air-reachable surface anywhere. Worked around in the asset: `areaM2` set to the measured definition (0.8895 and 1.5448 m²), stated in the thermal assumptions.

## USED-HOUSEHOLD-OBJECTS-STAFFAGE-COLLECTION — revolve `lateralScale` axis against the README

Files: `🎓️teaching/🏛️architecture/🧍️staffage/README.md` section "Volume" (table row `revolve`) and `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🧊️solids/🟦️.ts`. Carried over (coordinator's note on `used-wc-pan-cistern`, no number recorded).

The README says `lateralScale` of a revolved solid stretches along w (lateral); in the WC pan authoring it stretched the forward direction, and the author compensated in `assets/used-wc-pan-cistern/author-*`. Proposal: confirm the axis against the README, fix whichever is wrong, then re-check every asset that uses `lateralScale`.

## USED-HOUSEHOLD-OBJECTS-STAFFAGE-COLLECTION — bone mass has no lateral centre (`comZ`)

File: `🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts` (bone `mass`: `comX`, `comY` only) and `🧬️schema/🔣️.json`.

Measured on `used-bunk-bed`: the bone `ladder` sits at z -0.219 m so that its twist lays the ladder flat in the stored bundle, while the ladder itself stands at z -0.5265 m when set up (its anchors carry z -0.3075 m relative to the bone); the 6.06 kg of the ladder therefore act 0.31 m too far inwards in the lateral reaction split of the poses `set-up` and `in-use`. Proposal: an optional `comZ` on the bone mass, carried with the bone frame like the anchors' `z`.

## USED-MUSICAL-INSTRUMENTS-STAFFAGE-COLLECTION — `shapeDepthIn`: paint order of `sweep: "v"` / `"u"` solid shapes in a rolled pose

File: `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts` (`shapeDepthIn`, line 637 ff.). Carried over from the ticket's `requests.md` (author's measurement on `used-trumpet`, 2026-11-22 about 17:10; the finisher re-read the function at 21:20 and found it unchanged: the elevation still ranks by `local(p, 0)`, front and plan by the extruded span mid only; not re-measured).

`shapeDepthIn` ranks a shape in the elevation by its 2D profile points placed in the bone's (u, v) plane at w = 0, and in front/plan by the extruded span mid only. For an extruded solid with `sweep: "v"` the profile is a (u, w) outline and for `sweep: "u"` a (v, w) outline, so their points are read in the wrong plane. Measured on the `used-trumpet` candidate in the pose `carried` (case bone twisted −91°): the 6 mm case bottom drawn as a `sweep: "v"` box ranked about 0.2 m too far, so `depthOrder` flipped the revolved trumpet parts (bell, caps, buttons, mouthpiece) over it and the trumpet showed through the closed case in the side elevation.

Proposal: in `shapeDepthIn` map the profile point to the bone frame by the solid's sweep (u: (z, p.x, p.y); v: (p.x, z, p.y); w: (p.x, p.y, z)) with z sampled at z0 and z1, as `solid3` already does for the placement. Worked around in the asset: boxes and capsules of the case bone are drawn with `sweep: "w"`, whose profile lies in (u, v); the finisher opened the `carried` sheet of the published asset and the trumpet does not show through.

## PERFECT-HANDCRAFTED-3D-MODELS — `unionPolygons` fails on an extruded 2D shape seen exactly edge-on

File: `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts` (`unionPolygons`, line 284), called from `placeShape` in `🔨️modules/🧮️evaluation/🟦️.ts` (line 705).

Measurement (2026-11-22 21:07, `humans/parent-carrying-toddler`, check CLI): `✗ … clip walk-carrying@0.750: the union of 12 polygon(s) with area cannot be built: neither the edge-splitting nor the slab union closed a boundary of the expected area`. The twelve inputs are the prism faces of the extruded front shape `glove-thenar-l` (bone `hand-l`) at 1:50 and 1:100: quads of x 0.15765…0.18565 m stacked in y whose neighbours overlap by 0.04 mm (e.g. y 0.821159…0.83308 and 0.821117…0.833037), because pose `walk-6` hung the upper arm exactly plumb and the profile plane is seen edge-on. `validateAsset` and `checkVolumes` do not see it (default pose only); only the check CLI and the viewer at that instant do. Reproduce: set `upper-arm-l` of `walk-6` back to `{"angleDeg": 0}`, `forearm-l` to `{"angleDeg": 14}` and run `bun <ticket>/finisher-union-probe.ts <asset json> walk-carrying 0.75`.

Proposal: snap near-coincident parallel edges (below about 1e-4 m or 1e-6 of the extent) before the edge splitting, or return the hull of the faces for a prism whose projected faces are all collinear strips; add the twelve quads as a fixture with a test. Worked around in the asset: `walk-6` swings the arm 1° (`rework-05-parent-carrying-toddler-walk6.py`, `ARM_SWING` in `…-roots.py`); check CLI `✓` since.

## PERFECT-HANDCRAFTED-3D-MODELS — `volumes-silhouette` against the print rule for lattice bars at 1:500 (`machinery/tower-crane`)

File: `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts` (`strokesOf` / `followedWidth`, `memberOf`). Carried over from `rework/assets/machinery--tower-crane.md` ("Fundamental issues" 1; not re-measured by the finisher).

`r3-validate.ts machinery/tower-crane --catalog-only --volumes` is not `VALID`: two findings, both 1:500 `volumes-silhouette` (elevation IoU 0.794 against 0.9, plan 0.833 against 0.88; author's measurement of 2026-11-22 afternoon). A lattice chord of 120 to 160 mm must be a pen stroke at 1:500 (a filled strip is under 0.2 mm at 1:1000: `PRINT range-end-feature`), but an unfilled `member` shape with a pen as wide as the bar is compared as about 144 mm of ink against about 250 mm of body for the `slender` solid that follows it. As filled strips the two agree (elevation passes, plan 0.984) and the print rule refuses the band. Reproduce: `sh <ticket>/rework-tower-crane-diff.sh 500 elevation zoom "6,42.5,12,46.5"`.

Proposal: use one width on both sides for a `slender` solid and the member stroke it follows; fixture: one chord with its stroke at 1:500. No work-around inside the asset satisfies both rules; the asset stays `VALID` without `--volumes` and with two measured findings.

## RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION — structure solver: one rigid body for bodies that rest separately in a pose

Same root as the first section of this file (used household objects); carried over from `RECLAIMED-…/requests.md`, where the authors' full measurements stay. File: `🔨️modules/🏗️structure` (`solveStructure`, `reactionSolution`: 6x6 on all active supports, equal stiffness). Measurements, all still valid for the published assets (all 22 proven `VALID … checkVolumes 0` on 2026-11-22 evening):

- `reclaimed-steel-column`: bearers with mass add their weight to the column supports (off by 236 N of 2 495 N, 9 %) or halve the column reaction; worked around with massless bearers and sling.
- `reclaimed-rafter-bundle`, reinstalled: solver 1.292 / 1.136 / 1.286 / 1.134 kN against hand values 1.347 / 0.862 / 1.320 / 1.320 kN, false end moment 844 Nm, peak 898 instead of 498 Nm; worked around by applying the yard reactions as known `carried` loads.
- `reclaimed-trapezoidal-sheet-stack`, `reinstalled-snow`: every support 2.19 kN; by hand about 1.27 kN at an inner purlin and 0.5 kN at an end purlin.
- `reclaimed-roof-tile-pallet`, `reinstalled`: rafter pins carry 1.32 kN, the patch with battens and snow weighs 0.39 kN; no work-around possible (the 138 tiles left on the pallet are part of the lot).
- `reclaimed-parquet-crate` (lifted: bearers add 3.6 % to the fork load; reinstalled: crate weight shared with the field bedding), `reclaimed-paving-slab-pallet` (2.5 kN wheel load shared by the nine pallet feet), `reclaimed-oak-beam` (25 N per reaction): stated in `structure.assumptions`.

Proposal: split the evaluated state into the groups of bones whose subtree has its own active contact or support set and no movable-joint load path to the others (or accept a pose-level `bodies` declaration) and solve each group on its own supports.

## RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION — structure solver: a truss is a closed lattice, the rig is a tree

File: `🔨️modules/🏗️structure` (member diagrams are cut through the rig tree). Measurement on `reclaimed-timber-roof-truss`, reinstalled W-truss under covering and full snow (9.00 m span, 2.20 m rise, 17.25 kN): reactions 8.63 / 8.63 kN exact, but the moment view shows 17.5 kNm in each top chord and 70 Nm in the long webs, where a pin-jointed Fink truss carries about -19.6 kN in the top chords at the heels and +17.6 kN in the bottom chord with a panel moment of about 1 kNm. `reclaimed-lattice-girder` is affected the same way. Proposal: optional `rig.closures: [{ bone, at, to, toAt }]` and a frame or truss stiffness solve over the closed graph.

## RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION — `volumes-thermal-coverage`: copies of one `repeat` never seam

Measurement on `reclaimed-paving-slab-pallet`, 1:100: one 500 x 50 x 500 mm slab box with `repeat` count 8 (step 50 mm, faces touching) read `info:outerAreaM2:slab-stock` 9.600 m² (`info:cavityAreaM2` 7.0 m²) against 2.600 m² for the same slabs as 16 separate boxes, and the neighbouring region lost its face too (`slab-field` 1.100 against 1.600 m²). Separate touching solids of one region seam (C29 b); copies of one `repeat` do not. Work-around in the asset: individual boxes. Proposal: let the seam rule of C29 b treat the copies of a `repeat` like separate solids.

## RECLAIMED-BUILDING-COMPONENTS-STAFFAGE-COLLECTION — support solids for pose-dependent supports

File: `🔨️modules/🏗️structure` (`supportsOf` activates every `structure.supports` anchor in every pose) and the rule `volume-anchor` (a `part: "support"` solid needs an anchor of `structure.supports`). Measurement on `reclaimed-floorboard-stack`: stored 6 bearer contacts, lifted 2 crane-hook pins, reinstalled 7 sleeper contacts; listing the bearer contacts as global supports would send about 6/8 of the pack weight to the bearers in the lifted pose, so the asset (like most of the collection) keeps `structure.supports: []` and has no support solids. Proposal: let `part: "support"` solids name contact anchors that a pose or clip lists in `contacts`, or add per-pose `supports`.

## USED-PARK-OBJECTS-STAFFAGE-COLLECTION — structure solver: a lifted, carried or set-down part shares the weight with the footing supports

File: `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts` (`solveStructure`, `reactionSolution`). Same defect as the household section above; carried over from `used-chess-table` and `used-bubbler-stone-fountain` in the ticket's `requests.md` and re-measured by the finisher on 2026-11-22 with `USED-PARK-OBJECTS-STAFFAGE-COLLECTION/reactions.ts` (pose `maintenance`, before the work-around):

- `used-chess-table`: 13078.6 N on two soil and two sling contacts, 3269.7 N each; the lifted column and top weigh 2444.9 N (1222.4 N per sling).
- `used-pump-drinking-fountain`: hook 5263.5 N against 399.6 N for head, lever and piston. `used-sandpit-digger`: hook 4132.6 N against 276.1 N. `used-outdoor-fitness-station`: hands 2926.2 N and 3092.5 N against 14.0 N each. `used-wayfinding-signpost`: hands 840.5 N and 807.6 N against 13.2 N in all. `used-spring-rocker`: four sling legs 510 to 546 N each against 265.2 N in all. `used-seesaw`: 233.0 N on each of five supports against 380.6 N per sling. `used-tree-surround-bench`: 179 N per hand against 63.6 N. `used-bicycle-shelter`: 147 N per hand against 17.7 N. `used-double-swing`: set-down seat 293.2 N against 38.7 N. `used-slide-tower`: set-down roof 1155 to 1344 N per rest against 230.4 N. `used-park-lantern`: lawn mat 224.7 N against 197.4 N by moments about the hinge pin.

Worked around in all twelve assets (and already by their authors in `used-playground-roundabout` and `used-bubbler-stone-fountain`): the lifting point is a `load` anchor and the hook, sling, hand or resting reaction is an applied upward load of the statics of the lifted part alone (`lift_statics.py`); after it the footing supports carry exactly the rest (chess table 2 x 5316.9 N = 13078.6 - 2444.9 N). The work-around cannot show the lifting reaction as a support in the forces view and still treats bodies that only lie beside each other (the bicycle under the shelter, sand and edging of the sandpit, tyres of the seesaw) as one rigid body. Proposal as above: one rigid body per group of bones joined by fixed joints or active contacts, or a pose-level `bodies`.

## USED-PARK-OBJECTS-STAFFAGE-COLLECTION — volumes-silhouette: banded raster reports the top bin for an unchanged outline when only the frame height changes

File: `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts` (`measureBanded`, `boundarySpread`); probably the defect of the household section "sub-cell gap counted at the top bin".

Measured by the finisher on `used-concrete-table-tennis`, 1:50 front, 2026-11-22: two strip footings added below the ground, as the same boxes in 2D and 3D. With a footing depth of 0.40 m: `p95M` 48.8 mm (limit 30.0 mm), largest boundary distance 153.8 mm (21 cells of 7.32 mm). With 0.30, 0.50, 0.60 or 0.80 m and nothing else changed: no finding. The numbers do not move when the footing length changes from 1.30 to 1.10 m or the spall of the foot is removed, so the reported distance does not come from the footing but from a tile elsewhere (the perforated net with 60 holes of 25 mm is the only fine boundary). Proposal: as in the household section; a finding should also name the tile (view coordinates) it comes from. Worked around: the footings are founded frost-free at 0.80 m, which is also the more usual depth.

## USED-PARK-OBJECTS-STAFFAGE-COLLECTION — no per-pose presence of solids and shapes (lifting gear, people, covers)

Files: `🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts` and `🧬️schema/🔣️.json` (poses, solids, shapes).

Most of the 29 used park objects (the ledger `wave.md` names them) describe slings, hooks, ladders, workers or users in their `maintenance` and `in-use` poses that the drawing cannot show: a part cannot be absent in one pose and present in another, so lifted assets hang in the air without sling or hook and users exist only as loads. Authors stowed small parts inside others (`used-playground-roundabout`: eyebolt pushed 85 mm into the column cap) where that was possible. Proposal: an optional `poses` list on solids and shapes (present only in the named poses, blended by pose weight in clips), or a pose-level `hidden` list of solid and shape ids.

## USED-PARK-OBJECTS-STAFFAGE-COLLECTION — volumes-silhouette with slender curved tubes (`followedWidth`)

File: the silhouette rule as above (C24 `followedWidth`, `stripOf`). Carried over from the ticket's `requests.md` (author's measurement on `used-park-armchair`, not re-measured by the finisher).

A bent Ø25 armrest tube (loft over two 60 mm bends) declared `slender: true` at 1:200 and 1:500 and drawn as a true-width filled outline made the elevation IoU drop to 0.698 (1:200) and 0.789 (1:500): `followedWidth` measures the strip across the principal axis of the whole outline, so the U of tube and bends reads as about 70 mm wide. Proposal: measure the width along the solid's own centre line. Worked around: the tubes are not declared slender at the two coarse scales.

## USED-PARK-OBJECTS-STAFFAGE-COLLECTION — open detail strokes on a part that a pose turns over

File: `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts` and the evaluation of plan shapes. Carried over (author's observation on `used-beer-garden-set`, pose `maintenance`, not re-measured by the finisher).

The table is turned 180 degrees about its long axis; its open plan strokes (glue joints, beer rings, carved initials) are carried by the bone and drawn mirrored on top of the plan although they lie on the hidden underside. Proposal: give a stroke the depth of its side of the bone's silhouette or an optional `face`, and drop it when that face looks away.

## USED-PARK-OBJECTS-STAFFAGE-COLLECTION — manifest rows can be lost under the replica

File: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json` and every wave's `publish.ts`. See `USED-PARK-OBJECTS-STAFFAGE-COLLECTION/manifest-drift.md`: 14 of 20 rows of this wave were lost once at about 16:55 although all publishers share one lock folder (the lock is a local folder under a replica with about one second of lag). At 20:27 the finisher found all 28 earlier rows present and inserted the missing row of `used-bicycle-shelter`; `verify.ts` of the ticket compares candidate, catalog copy, manifest row and collection membership without writing. Proposal: one manifest writer that derives the rows from the asset files.

## STAFFAGE-HUMAN-COLLECTION-WAVE — catalog-silhouette: the Hausdorff gate fails for an established human, so the people derived from it cannot pass it

File: `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🟦️.ts` line 104 (`expect(measured.hausdorffM).toBeLessThanOrEqual(2 * feature)`) with `measureAsset` of `🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`.

Measured by the finisher of HW-01 … HW-09 on 2026-11-22 21:36 with the test itself, one asset (`bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ../../🧪️tests/🧊️catalog-silhouette/🟦️.ts -t worker-carrying-box`): `AssertionError: worker-carrying-box 1:50 elevation Hausdorff: expected 0.10744911586420801 to be less than or equal to 0.015` (IoU 0.991; front 20 mm, plan 27 mm against the same 15 mm). `worker-carrying-box` is a manifest asset from before today and the base figure of the three HW-04 people. The closed guard ticket (`☀️17/STAFFAGE-CATALOG-SILHOUETTE-GUARD-PASSES-FOR-EVERY-ASSET`) proved bounding box and IoU; the largest boundary distance of two smallest features (15 mm at 1:50) is not met by the base figures, and the new people inherit their outlines: of the 23 people of HW-01 … HW-09 only `standing-desk-worker`, `office-worker-carrying-laptop`, `doctor-with-clipboard`, `patient-with-iv-pole` and `hiker-with-trekking-poles` meet it at every scale and view (table in `humans-a.md`).

Proposal: decide whether the gate is the largest distance (then the base humans need a repair pass first, e.g. `humansb-D-auto.ts` of the guard ticket, and the derived people after them) or a quantile (`p95M`, as `volumes-silhouette` uses); until then the full `catalog-silhouette` suite is red independently of the human wave. Not worked around: no outline was replaced by a trace only to meet a gate the base figure misses.

## STAFFAGE-HUMAN-COLLECTION-WAVE — unionPolygons cannot place an extruded shape seen along its profile (humans b)

File: `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts:284` (`unionPolygons`), called from `🔨️modules/🧮️evaluation/🟦️.ts:705` (`placeShape`).

Measured by the finisher of HW-10 … HW-18 on 2026-11-22 with `K/🧰️scripts/FIN-B/depict.ts` (one shape at a time): in `sunbather-on-towel` as the worker shipped it, the front-view shape `hand-r` (`solid: { kind: "extruded", sweep: "w", z0: -0.1695, z1: -0.1315 }`, a hand profile swept across the hand) throws `PolygonUnionError: the union of 11 polygon(s) with area cannot be built: neither the edge-splitting nor the slab union closed a boundary of the expected area` at `fanning-face@0.25` and `@0.75` at 1:50 (the check CLI reports it as a failed clip instant). The lying figure's hand points at the viewer there, so the long edges of the profile are parallel to the viewing direction and the side faces of the prism project to slivers. Turning the wrist keys by 1.37°, 2° or 3.71° moves the failure to other instants and scales (4 to 6 of 81 instants of the loop fail: 7 polygons at 1:100, 2 polygons at 1:500), so it is a band of orientations, not one angle.

Proposal: drop projected faces below an area tolerance before the union and, for a convex profile, take the convex hull of the projected prism; add a fixture with a thin prism rotated through the view direction.

Worked around in the asset (`K/🧰️scripts/HW-16/sunbather.ts`): the hands are drawn in the front view as prisms swept along the hand bone (`sweep: "u"`), which place at all 81 instants and all four scales.

## STAFFAGE-HUMAN-COLLECTION-WAVE — the structure solver leaves a force residual when its support distribution switches (humans b)

File: `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts` (reactions and residual, about lines 559 to 677).

Measured with `K/🧰️scripts/FIN-B/struct.ts` on `picnicker-sitting-on-ground` as the worker shipped it, clip `idle-breathing` on the pose `leaning-back` (contacts `blanket-ground`, `seat`, `palm-l` active; inertial forces 0.0 N at every instant): at 0 s the reactions are blanket 40 N, seat 635 N, palm 5 N and the residual is 0; from 1.125 s to 2.375 s (the inhale) they are blanket −107.6/262.1 N, seat 104.2/262.0 N, palm −87.1/155.1 N (fx/fy) with a residual of −90.45 N in x and −68.20 N in z, although the pose and every other instant are in equilibrium. The check CLI reports `clip idle-breathing@1.250: force residual -8.97e+1`. The same pattern (opposed horizontal reactions at the friction limit and a remainder) appears in `dancer`, `pirouette@1.925`, standing in fourth position after the landing: contact-l fz −158.3 N, heel-l −164.9 N, heel-r +160.0 N at 200 N normal force, residual −136.53 N in z.

Proposal: when a friction cone saturates, redistribute the redundant horizontal reactions (minimum-norm solution inside the cones) instead of reporting the remainder as unbalanced; keep the distribution continuous in the position of the centre of mass.

Worked around in the assets: the picnicker's idle clip breathes in the cross-legged pose; the dancer's landing is slower and loads the second foot when it is down.

## STAFFAGE-HUMAN-COLLECTION-WAVE — nine check samples miss the jerks at clip keys (humans b)

File: `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts` (`queriesOf`), `🔨️modules/🎞️animation/🟦️.ts:377` (`smooth` keys are Hermite through their neighbours).

Measured on `dancer` as shipped: a `smooth` key followed by a held or `linear` key arrives with a velocity (centre of mass rising at 0.075 m/s at the key at 0.75 s, then at rest), which the central difference of the kinematics turns into −2839 N of ground reaction at exactly 0.75 s and +4140 N at the landing key 1.85 s. `--samples 9` samples 0, 0.275, 0.55 … and saw only the after-effect at 1.925 s. Proposal: let the check also sample every key time, or document that a `smooth` key must be followed by a `smooth` key; an `in` ease into a halfway key and an `out` ease out of it give a jerk-free start and stop (used in the dancer now, 225 instants in equilibrium).

## STAFFAGE-HUMAN-COLLECTION-WAVE — the plan head of seated-guitarist is a slab stretched for its own playing pose (humans b)

File: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/seated-guitarist/🔣️.json`, `representations.*.geometry.plan` shapes `head`, `hair`, `neck`, `beard`, `ear-l`, `ear-r` (`solid: { kind: "extruded", sweep: "v", z0, z1 }` 0.8 mm thick, profile 0.60 m long along the bone).

Measured with `K/🧰️scripts/FIN-B/order.ts`: the shapes are a frontal slab pre-stretched so that it projects to a head only at the 20° forward tilt of `strumming`; in the pose `resting` (4° tilt) the head is 0.22 mm wide in the plan at 1:200 instead of about 0.95 mm, and on an upright head (the derived `picnicker-sitting-on-ground`) it was 0.00 mm wide at every scale, so the picnicker had no head in the plan. Fixed in the picnicker only (`plan_head` in `K/🧰️scripts/HW-10/picnicker-sitting-on-ground.py`: the true horizontal outline swept along the bone). Proposal: redraw the six plan shapes of the base the same way.

## STAFFAGE-HUMAN-COLLECTION-WAVE — standing-adult changed under the wave (humans b)

`🗂️catalog/👤️humans/standing-adult/🔣️.json` was rewritten at 16:58 during the wave (its front sleeves `sleeve-l`/`sleeve-r` are revolved solids now; the people derived before that time carry extruded ones). `K/🧰️scripts/HW-17/gen_person_queuing.py` stops in `fit_arms` on today's base (`ValueError: fit_arms: unexpected solid of sleeve-l in front`). No action asked of the engine; the owner of the change should know that generators of the wave that derive from `standing-adult` (HW-10 coffee, HW-11 waving, HW-12 violinist, HW-13 sweeping, HW-15 signing, HW-16 sunbather, HW-17 queuing) may not reproduce their shipped files any more. The shipped `person-queuing` is the worker's file with a named patch (`K/🧰️scripts/FIN-B/patch.py`).

## STAFFAGE-HUMAN-COLLECTION-WAVE — `unionPolygons`: second measurement of the edge-on prism failure (`humans/person-pushing-bicycle`)

Same root as the section "PERFECT-HANDCRAFTED-3D-MODELS — `unionPolygons` fails on an extruded 2D shape seen exactly edge-on". File: `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts` (`unionPolygons`), reached from `evaluate(asset, query, { kind: "scale" … })`.

Measured 2026-11-22 at about 21:55 with the check CLI: `✗ person-pushing-bicycle … clip push-walk@7.087: the union of 2 polygon(s) with area cannot be built …` and `clip push-walk@16.537: the union of 4 polygon(s) …`. Isolated with `STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/FIN-A/union.ts person-pushing-bicycle push-walk 7.0875`: only 1:200, only the front shape `shoe-toe-l` (bone `toe-l`, `poly` with `solid` extruded, sweep `w`) alone; the instant lies between the gait keys `walk-heel-off-r` (toe 16.7°) and `walk-late-swing-l` (toe 0°).

Worked around in the asset: the swinging toe of the two late-swing keys is lifted by 1° (`🧰️scripts/HW-09/poses_person_pushing_bicycle.ts`); check CLI `✓` since. Any other instant of a clip can still meet the same coincidence.

## flow view — repo run hangs for @teaching/staffage

Observed by the finisher of `STAFFAGE-THERMODYNAMIC-FLOW-VIEW-INSTEAD-OF-AIRFLOW-CONES` on 2026-11-22.

- 20:25 to about 22:10: the first finisher called the MCP tool `mcp__repo__run` (type check of `@teaching/staffage`); the call never returned and no file was changed. The scheduler history lists `run @teaching/staffage:typecheck`, `run @teaching/staffage:test` and `exec --projects=@teaching/staffage` as `failed`. No `mcp__repo__*` build tool and no `repo run` was called by the second finisher.
- 23:05:57, the one bounded trial of the catalogue command: `repo test-file "🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🌊️flow-field/🟦️.ts"` under `perl -e 'alarm 280; exec @ARGV'` returned in 1 s with exit 0: `passed … (plain, 0.4 s, 10 pass, 0 fail)`, `test-file: 1 passed, 0 failed`. So `test-file` (bun test of one file) works; what hangs is the graph path of `run` / `exec` for this project (the targets `typecheck` and `test` of `S/📦️packages/🟦️typescript/📋️project.json` are `repo ./📜️script.ts typecheck|test` behind the scheduler).
- Everything else was proven with the underlying commands, each bounded: `bun node_modules/.bin/tsc --noEmit -p tsconfig.json --incremental false` in `S/📦️packages/🟦️typescript` (5 s, 0 errors) and in `S/🎯️targets/⚛️react/📦️packages/🟦️typescript` (5 s), `bun node_modules/vitest/vitest.mjs run --configLoader native --config <config> <filter…>`.
- Proposal: `repo run <project>:<target>` needs a deadline of its own that ends with a named error (which lease, which graph step it waits for) instead of waiting for ever, and the MCP tool must report progress or fail when the scheduler entry is `failed`.

## flow view — failures of neighbouring suites that are not flow code

Measured 2026-11-22 22:13 to 22:17 with vitest (library config), for the umbrella coordinator's suite run:

- `🧬️schema-conformance`: `👤️humans/mobility-scooter-user` and `👤️humans/elderly-person-knitting`: `/asset/provenance/notes` "must be object" (a plain string where a `{ en, de }` text is required).
- `🌤️reference-environment` "validates a preset id …": `validateAsset(solar-thermal-collector)` returns 34 issues (`volumes-feature`: `header-upper`, `header-lower` 22 mm thin at 1:500, and others). The three byte comparisons of that suite are green again (see `flow-view.md`).
- `🎭️cast-export`, `🧊️cast-lod` (2): the course manifest names clip `heating-sweep` of `heat-pump-outdoor-unit`; the asset has `heating-modulation`, `defrost-sequence`.
- `🔭️multiview-coverage` (2): planted contact 0.080 m > 0.072 m; `standing-adult` moments `pelvis|bending-moment` differs between elevation (−1.215) and front (−0.0012).
- `📑️multiview-sheet` (2): forklift in the moments mode, PDF paper x 73.55 mm against 73.24 mm (first angle) and 24.46 against 24.00 (third angle); the SVG of the same sheet agrees. No flow is drawn in that mode; the flow marks of a sheet are told apart by the id prefix `flow-`, asset shapes carry `shape:` / `outline:` / `hatch:`, so the catalog shape ids `flow-stub`, `flow-meter-…` do not collide.
- `🧊️model-batch`: the whole suite does not finish in 10 minutes on this host (killed after 11.5 minutes; every pose GLB and thermal FBX now develops a flow); 4 of its 11 cases were run by name and pass. It needs one unbounded run by the coordinator.
