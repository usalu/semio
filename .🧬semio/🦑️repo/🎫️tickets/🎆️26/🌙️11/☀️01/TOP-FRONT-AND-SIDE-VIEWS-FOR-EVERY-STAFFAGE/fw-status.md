# FW status (framework worker)

State: step 1 READY (preview and check CLIs). Schema, validator and consumers are done too; the full test run is still pending.

`S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = this ticket folder.

## Preview CLI

```
repo tools exec bun -- 🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset <id> --scale 50 [--pose id | --clip id --time s] [--viewpoint elevation|front|plan] [--views] [--view dark-line] --out <dir>
```

- `--viewpoint front|plan` renders the single view; file names get the suffix `-front` / `-plan` (`<asset>--<pose>--dark-line-front-1-50.svg`).
- `--views` writes one sheet `<asset>--<pose|clip@t>--views-dark-line-1-<scale>.svg/png` with side, front and top view of the same instant side by side at the same scale, bottom aligned. Add `--view <v>` for another of the seven views.
- PNG: `rsvg-convert` is used when installed; on native Windows the CLI falls back to the system `python` with PyMuPDF (`fitz`). Add `--no-png` to write SVG only, `--dpi 150` for smaller PNGs.

## Check CLI

`repo tools exec bun -- S/🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset <id>` now depicts every named pose and every sampled clip instant at every authored scale in all 7 views and all 3 viewpoints (messages read `pose walk 1:50 front dark-line: ...`), reports `no shapes` for an empty viewpoint, and measures print suitability on all three viewpoints unconditionally.

## Schema and validator

- `S/🧬️schema/🔣️.json` and `🟦️.ts`: `AuthoredRepresentation.geometry` requires `elevation`, `front`, `plan` (each minItems 1); Symbol geometry unchanged; `presentation.viewpoints` and the manifest entry `viewpoints` are deleted (the 148 manifest rows are rewritten).
- Validator: `front-empty`, `plan-empty` (en+de), `viewpoint-unlisted` removed; every front/plan shape region must name an existing thermal region (same `need` as the elevation); `representation-copied` compares all three viewpoints, tagged by viewpoint.
- Content workers: just delete `presentation.viewpoints` from your asset files; the schema now rejects it.

## Consumers

React detail viewer: new always-offered viewpoint control (Side, Front, Top / Seitenansicht, Vorderansicht, Draufsicht), address parameter `vp`, viewpoint tag, export panel request carries it. The dimensions fact `Viewpoints` is gone. Cast exporter no longer needs a "has no front geometry" guard.

## Solid placements (revolved and extruded front and plan shapes) — READY

Every front or plan shape may carry `solid`; the profile is authored in the bone's local frame like an elevation shape and placed with the full bone transform (so pose and clip rotation, slat tilt, flap and boom raise, canopy furling all read right):

```json
"solid": { "kind": "revolved", "axisBone": "pole", "axisX": 0.0, "axisZ": 0.0, "mirror": true }
"solid": { "kind": "extruded", "z0": -0.2, "z1": 0.3 }
```

- `revolved` (umbrella, parasol canopy and ribs, lid, drum): front paper x = lateral distance from the vertical axis through the origin of `axisBone` (rest elevation x `axisX`; the axis travels with that bone), y = world height. `mirror: true` also draws the reflection about the axis, so author a half profile (x ≥ 0 side) and get the whole silhouette; the front silhouette equals the elevation silhouette of the revolved part at every instant. Plan = circle of the widest lateral reach about the axis, centred at the axis (x = elevation x, y = `axisZ`). Rest-pose consistency: author profile x relative to the axis so that at rest the front half-width equals the elevation half-width.
- `extruded` (slats, flaps, door panels, booms, boards): front = `z0..z1` (lateral, left is +) by the height range of the profile, plan = forward range of the profile by `z0..z1`. This works for bones horizontal at rest where `followTip` has zero span.
- `solid` is allowed in front and plan only, is exclusive with `followTip`, `axisBone` must exist, `z0 < z1` (validator codes `solid-viewpoint`, `solid-follow-tip`, `solid-extent`). The shape's `kind` (rect, poly, capsule, ellipse, path) is only the profile; the output is a closed ring, so use `filled`/`role` as usual.
- Schema: `$defs/Solid`, `Shape.solid`; twin `Solid` in `S/🧬️schema/🟦️.ts`; evaluation `solidRings` in `S/🔨️modules/🧮️evaluation/🟦️.ts`.
- Test: `S/🧪️tests/🧊️solid-placements` (Gherkin, hand-computed vectors in `S/🧫️fixtures/🧊️solid-placements`, shapely oracle); run with the ticket-local config `T/🧰️FW/vitest.solid.config.ts` (same recipe as the three-views fallback). Passes.

### Revolved front shapes

See the `revolved` rows above; the first design draft `revolved: {axisBone, axisX}` is now `solid: {kind: "revolved", ...}`.

## Lateral spread (`abductionDeg`) — READY (section by TS)

Pose and clip-key bone offsets accept `abductionDeg` (revolute joints, -180..180): the bone turns out of the sagittal plane about its own normal after its sagittal rotation, cumulative down the chain, interpolated by clips and transitions. Positive is towards the asset's left (+z); right-side parts use negative values (a right-facing side view shows the right side: near wing negative, far wing positive).

```json
"bones": { "arm-n": { "angleDeg": -46, "abductionDeg": -75 }, "arm-f": { "angleDeg": -32, "abductionDeg": 75 } }
```

- **Decision (coordinator): all three views agree in every pose, the elevation foreshortens by cos φ.** `solveFrames` in `S/🔨️modules/🦴️rig/🟦️.ts` solves the 3D chain (a point at distance s along a spread bone is s·sin φ lateral and keeps s·cos φ of its reach; children ride on the spread frame) and `solveRig` returns its orthographic side projection (non-rigid affine with the projected axes) whenever a bone is spread, else the unchanged 2D solve. So elevation placement, mass, thermal, structure, kinematics and grounding read one chain. `solveLateral` gives the lateral position of origin and tip of the spread bones.
- Front and plan: projected bone travel (already in the matrix) plus lateral position; rigid shapes follow the origin (children ride on a spread parent), `followTip` rect/poly stretch origin to tip (plan also forward), plan `followTip` capsules move both ends; the fraction along the bone is read from height (front) / forward position (plan) when the bone has at least half its length of that extent at rest, otherwise from the lateral position across the shape (inner end to outer end), so a folded wing authored along the view axis grows outwards. `solid` shapes ignore it. Evaluation: `spreadShape`/`placeShape` in `S/🔨️modules/🧮️evaluation/🟦️.ts`; animation `FullOffset.abductionDeg`; schema `BoneOffset.abductionDeg` (json + twin); validator `joint-limit` (fixed or prismatic joint, above 180 degrees).
- Guard: the three-views sweep checks `spread-elevation-x-plan-x`, `spread-elevation-y-front-y`, `spread-front-x-plan-y` for every pose and clip instant with a spread bone (tolerance as for the default pose), so spread poses must keep the three views one object.
- Content authoring: author the folded rest outline in front/plan and bind each wing/arm/leg part to its bone with `followTip: true`; spreading needs no extra shapes. At `abductionDeg = 0` (every default pose) nothing changes.
- Demonstration: `herring-gull` poses gliding (+-75), gliding-up (70), gliding-down (80), approach-a (75), flare-key and landing (55), touchdown (30); near wing negative, far wing positive.
- Tests: `S/🧪️tests/🧊️lateral-spread` (Gherkin, hand-computed `S/🧫️fixtures/🧊️lateral-spread`, numpy + shapely oracle, validator cases, gull); run with `T/🧰️TS/vitest.lateral.config.ts`.

## Axial twist (`twistDeg`) — READY (section by TS)

Pose and clip-key bone offsets accept `twistDeg` (revolute joints, -180..180): the bone and its whole subtree turn about the bone's own longitudinal axis, after its sagittal rotation and spread, accumulating down the chain (children are placed in the twisted frame of the parent), interpolated by clips and transitions. Positive twist turns the bone's local +y axis towards the right of the asset: a bone pointing up (mast, turntable, torso) turns counter-clockwise seen from above (forward goes to the left), a bone pointing down clockwise.

- Same `solveFrames` 3D chain in `S/🔨️modules/🦴️rig/🟦️.ts` as the spread (`twist` is a rotation of the frame about `u`), so elevation, front and plan (rigid shapes, `followTip`, spread and `solid`) are projections of one chain. A jib slewed by tau is `cos tau` as long in the side view and reaches `sin tau` of its length sideways. `solid` extruded shapes widen their lateral range by the lateral sweep of the bone; revolved solids are twist invariant (plan axis shifts with the lateral position of the axis bone).
- Schema `BoneOffset.twistDeg` (json + twin, -180..180); `FullOffset`/`EvaluatedBone.offset` carry it; validator `joint-limit` for a twist on a fixed or prismatic joint and above 180 degrees.
- Authoring rule for front and plan: a bone that lies along the viewing axis at rest (jib, boom, wing) needs its front and plan shapes bound with `followTip: true` (lateral rule: inner end follows the origin, outer end the tip), or a plan `capsule` with `followTip`, so they slew; the guard (`spread-*` checks of the three-views sweep now run for every pose or clip instant with a spread or twisted bone) tells you.
- Tests: `S/🧪️tests/🧊️axial-twist` (Gherkin, hand-computed `S/🧫️fixtures/🧊️axial-twist`, numpy + shapely oracle shared with `🧊️lateral-spread/🐍️.py`, validator cases, crane); run with `T/🧰️TS/vitest.lateral.config.ts`.
- Demonstration: `tower-crane` pose `parked-weathervane` slews its jib 60 degrees. The rig gained a zero-length-motion revolute bone `turntable` (child of `mast` at 39 m, `slewing` now child of it with the same placement; joint limits 0..0 for the angle, twist free), because the former `slewing` joint is fixed and horizontal. QA6 owns the views: the guard now reports `spread-elevation-x-plan-x` for the pose (elevation x [-8.25, 27.65] vs plan x [-16.5, 55.3] m at 1:50/1:100, same at 1:200/1:500), i.e. the jib/counter-jib/trolley plan shapes are not yet slew-aware (bind them with `followTip` as above), the elevation is already right.

### Update (spread and twist): front and plan shapes are carried by the full 3D transform — TS

Authors no longer need `followTip` workarounds for spread and twist. For a bone that is spread or twisted, or has such an ancestor (`SolvedFrame.affected`), `S/🔨️modules/🧮️evaluation/🟦️.ts` places front and plan geometry with the 3D transform from the rest frame to the current frame (`carry` in `S/🔨️modules/🦴️rig/🟦️.ts`: `now.o + now.M · rest.M⁻¹ · (p − rest.o)`) and projects it to the view:

- Rigid shapes, and `followTip` shapes on a bone that has at least half its length as extent in that view at rest (`inView`), are rotated exactly: in plan about the vertical (jib, counter-jib, pendants, trolley slew as authored), in front about the forward axis. A plan point `(x, y)` is the rest point (x, height of the bone origin, lateral y); a front point `(x, y)` is (forward position of the bone origin, height y, lateral x). A bone along the viewing axis has no in-view extent: its `followTip` shapes still grow outwards (lateral rule), rigid shapes keep their end-on silhouette.
- `solid` extruded: the profile swept along the lateral axis of the frame from `z0` to `z1` (measured from the rest origin of the bone), projected, convex hull per ring. `solid` revolved: revolved about the vertical axis of its axis bone carried to the current frame (a tilted axis is legal), each profile edge as the hull of the projected circles of its two ends.
- New `Bone.z` (lateral rest offset of the origin in the parent frame, default 0, schema JSON + twin): the pivot of a spread or twist of a bone beside the symmetry plane. Author front and plan shapes of such a bone centred on the pivot; `herring-gull` `arm-n`/`arm-f` now have `z` = -0.068 / 0.068.
- Without spread or twist (and for bones whose chain has none) the old path runs unchanged (bit-identical). The three-views guard passes on `tower-crane` (`parked-weathervane`, no crane shape edited) and `herring-gull`.
- Tests: rigid-shape vectors in `🧫️fixtures/🧊️axial-twist` (torso block turning about the vertical) and `🧫️fixtures/🧊️lateral-spread` (rigid arm and shank shapes), oracle `🧪️tests/🧊️lateral-spread/🐍️.py` carries points with numpy rotation matrices.

## Coordinator Broadcast: Full 3D Placement Landed
Rigid shapes, `followTip` shapes and revolved/extruded solids on spread or twisted bones now follow the bone's full 3D frame. A twist rotates about the vertical in plan; revolved solids revolve about their axis bone's carried frame, so a tilted axis is legal. `Bone.z` (lateral rest offset of a bone origin, default 0) is available for bones beside the symmetry plane. Author those shapes centred on that pivot. Do not use `followTip` workarounds.

### Update: solids under spread/twist, `Bone.z` validation, consumer audit — TS

- **Solid vectors** (`🧫️fixtures/🧊️axial-twist`, oracle `🧪️tests/🧊️lateral-spread/🐍️.py`, Gherkin in `🧪️tests/🧊️axial-twist/🥒️.feature`): extruded slab on a torso twisted 45/90 degrees, revolved drum on a torso tilted back 14 degrees (tilted axis), a lid opened 105 degrees on a twisted body (revolved and extruded parts); hand-computed with the frame algebra written in the fixture description; numpy + shapely recompute the same extents.
- **Validator:** `bone-lateral` — the rest lateral position of the origin and tip of every bone (accumulated `Bone.z`) must stay within half of `dimensions.depthM` plus max(0.02 m, 3 % of the largest dimension). `loop-closing` now also compares `abductionDeg` and `twistDeg` of the first and the closing key of a looping clip (they were ignored before).
- **`Bone.z` in an unaffected chain** keeps the authored front/plan placement and does not change the elevation (vector `boneZ` in `🧫️fixtures/🧊️lateral-spread`); spread about a bone with `z` pivots about it.
- **Consumer audit** (all read bones through `solveRig`/`EvaluatedBone`, so they follow the 3D chain's side projection): evaluation (placement, mass centres via `massCentres(solveRig)`, kinematics and grounding `groundedY`), thermal (angles from `EvaluatedBone`), structure (`worldOf`/`angleRad`: statics of the projected sagittal system; a bone turned end-on has a zero projected direction and falls back to angle 0, no NaN), respiration (merges `abductionDeg`/`twistDeg` of the base offsets unchanged), animation (`FullOffset`, blend, hermite, cycle turns), catalog validator (joint limits, now loop closing and `Bone.z`), structure harness and thermal fixtures (types), depiction/React/cast export/python `_index` (use evaluated shapes or the exported index only, no bone offsets). Fixed: the loop-closing gap above; nothing else ignored the new fields.

## Depth ordering

Implemented in the depiction, not in evaluation: `depthOrder` in `S/🔨️modules/🎨️depiction/🧊️depth/🟦️.ts`, called by `prepare` (`🧩️shapes`) after the authored sort (`z`, then position), so every view, the hidden-line removal and the exports use the same order.

Rule: the authored paint order is the truth at rest. At a pose or clip instant a pair of shapes of **different bones** is painted the other way round only when its physical depth **reverses against the rest pose** by more than 2 mm (`DEPTH_TOLERANCE_M`). Depth, nearer = larger:
- plan: the top of the shape: the larger y of the bone origin and tip, or for a `solid` shape the highest point of its placed profile;
- front: the foremost point: the larger x of origin and tip, or the foremost point of the placed solid profile;
- elevation: the smaller lateral z of the bone (the 3D chain `solveFrames` of the evaluated offsets, `restFrames` at rest); the viewer of the side view is on the right of the asset, which is why the authored arm of that side is in front.

Why relative to rest and not absolute: a bone height is not the height of the drawn part (a car body on the root bone sits below its tyre bones, yet the body hides the tyres from above), so an absolute depth would repaint static assets. Shapes of one bone, ground shapes and pairs that keep their rest relation keep the authored order, so static drawings and all poses where nothing passes behind anything stay bit-identical; a flame that sinks below the hob, a thigh that rises past a part, a thigh spread to the right of the asset are painted correctly.

No authored `depth` bias was needed (no schema change): coplanar cases stay on `z`. To make a part sink behind another, author the two parts on different bones with the part that sinks on the bone that moves; no other authoring is needed.

Test: `S/🧪️tests/🧊️depth-order` (Gherkin, hand-computed vectors in `S/🧫️fixtures/🧊️depth-order`, shapely oracle with its own topological sort), passes.

## Same-region merge

Fixed in `outlineRuns` (`S/🔨️modules/🎨️depiction/🧩️shapes/🟦️.ts`): the rule "filled shapes of one region on the same or adjacent bone hide each other's inner outlines" now applies only when the shape whose outline is cut is itself a **filled closed** shape. Unfilled strokes (lines, paths, unfilled rings, spokes, ribs, unfilled rects) are no longer removed by a filled shape of their own region; they are cut only where a filled shape with a **higher z** (genuine occlusion, as before) covers them. Workers can drop their workarounds (separate regions or filled hairlines for fan rings, mixer flights, steering wheels, bucket ribs). Rule of thumb: put a detail above its body (higher `z`); a detail with a lower `z` than a filled shape that covers it is hidden, correctly.

Test: `S/🧪️tests/🔗️region-merge` (Gherkin, hand-computed lengths in `S/🧫️fixtures/🔗️region-merge`, shapely oracle); it fails on the old rule (detail ring 0 mm instead of 5.2 mm) and passes now.

## Feature width measure (0.194-0.197 mm)

Cause: a raster artefact, not a cache. `featureWidthMm` (`S/🔨️modules/🖨️representation/📐️measure/🟦️.ts`) rastered every shape at pitch `minFeature / 6` = 0.033 mm and took the radius of a cell as the distance to the nearest outside cell centre, in whole pixels. A hand 0.28 by 0.18 mm is about 8 by 5 pixels, so its width came out as a multiple of the pitch (2·3 px = 0.200 mm exactly, other tiny shapes 0.194-0.197), unrelated to its real width (0.18 mm), and the width of a shape a few pixels across barely moved when it grew. Fix: (1) small shapes are rastered at a refined pitch (about 1.2e5 cells, down to 2 µm); (2) the radius of a cell is the distance less half a pixel, the distance to the boundary.

Second defect, found with QA2's note: `measureSuitability` cached by asset, denominator, scale and profile only, so a representation changed in place (or replaced) kept the old verdict. The key now carries a fingerprint of the representation, the rig and the poses.

Test: `S/🧪️tests/🔬️feature-width` (Gherkin, hand-computed widths in `S/🧫️fixtures/🔬️feature-width`, shapely oracle by erosion and by opening; stale-cache case). 12 of 12 pass.

## BROADCAST: one 3D path, elevation solids, one outline per solid — TS

For every content worker (read this before touching front, plan or elevation geometry of a part that is spread, twisted, tilted or swivels):

1. **One placement path for every bone.** There is no old path any more: every front and plan shape is carried by the 3D transform from the rest frame to the current frame of its bone and projected (`placeProjected` in `S/🔨️modules/🧮️evaluation/🟦️.ts`). Rigid shapes rotate with their bone in every pose (a thigh swinging 22 degrees narrows its plan footprint to cos 22 deg; a leaning torso foreshortens its rigid plan shapes). Outputs are identical where a bone is untilted and unspread. `followTip`: plan capsules keep the end-follows-origin/end-follows-tip rule; rect and poly `followTip` only matter for a bone that has no extent in the view (an end-on bone), where the shape additionally grows from the displacement of the origin to that of the tip.
2. **`solid` works on ELEVATION shapes too** (schema doc, validator no longer refuses it). One solid is one 3D definition: the profile is authored in the bone's local frame (like an elevation shape), `extruded {z0, z1}` sweeps it along the lateral axis of the bone frame (z measured from the rest lateral position of the bone origin, i.e. `Bone.z` included), `revolved {axisBone, axisX, axisZ, mirror}` revolves it about the carried vertical axis of its axis bone. It projects consistently to the side (x, y), front (z, y) and plan (x, z) view. A revolved axis follows the full frame of the axis bone, so a lid opened 105 degrees by a plain sagittal rotation or a mixer drum tilted 14 degrees revolves about its tilted axis (no spread or twist needed). Without `mirror` the half profile is drawn (front: signed forward distance becomes lateral; elevation: as authored; plan: the full circle).
3. **One outline per solid.** The projection is the union of the swept edges (extruded) or of the per-edge hulls of the projected circles (revolved): `S/🔨️modules/🔺️polygon/🟦️.ts` `unionPolygons`, shapely `unary_union` equivalent, no external library. A concave profile (L-shaped bracket) stays one non-convex ring; an unaffected extruded rectangle is exactly its 4-vertex rectangle. The profile is simplified to 0.3 mm first and revolved circles use 32 segments, which keeps a revolved solid at about 10 to 250 ms per evaluation in the whole catalog.
4. **Authoring rule for end-on bones (wings, jibs, booms, door leaves that lie along the viewing axis at rest):** author the part as a solid, not as a front or plan sliver. The profile is the part's outline in the bone frame (reuse the elevation geometry), `extruded` z0/z1 is its rest lateral thickness (absolute rest coordinates; for a part beside the symmetry plane give the bone `z` and put z0/z1 around it), and the same solid goes into elevation, front and plan. Spread, twist and sagittal swing then carry it in all three views. `herring-gull` wings (arm-n/f, hand-n/f at 1:50, 1:100 and 1:200, all three views) are the reference: the earlier QA1 attempt that was 0.04 to 0.07 m too high was a profile in the wrong frame (front-plane coordinates instead of bone-local ones) and a missing `Bone.z` pivot, not a placement bug.
5. **Validator:** `elevation-lateral` (en/de): a non-solid elevation shape on a bone that is spread or twisted in any pose or clip, whose front twin (same id, same bone) is wider than max(0.05 m, 10 % of the largest dimension), should be a solid; thin parts are fine. `follow-tip-viewpoint` now allows `followTip` rects and polys in front and plan and capsules in plan only. `bone-lateral` and the loop-closing check for spread and twist are unchanged.
6. Vectors, oracle and Gherkin: `🧫️fixtures/🧊️axial-twist` (elevation solids at 0, 45, 90 degrees, revolved drum, plain sagittal 105 degree lid, L-shaped bracket as one ring with area 0.03 and 0.148492 m2, upright slab as exactly its rectangle), `🧪️tests/🧊️axial-twist`, oracle `🧪️tests/🧊️lateral-spread/🐍️.py` (numpy frames, shapely `unary_union` areas). Updated vectors for the single path: `🧫️fixtures/🧍️front-projection`, `🧍️plan-projection`, `🧊️three-views`, `🧊️lateral-spread`.

### Update (QA11): depth of flat front and plan shapes — TS

A flat plan shape is carried at the HEIGHT of the middle of its elevation twin (same shape id, same bone, not a solid) at rest, a flat front shape at the FORWARD POSITION of that middle, instead of at the rest position of the bone origin. A rigid part far from the origin of a tilted or yawed bone (scooter apron, lamps on a raked steer axis, a pedestal at the top of a leaning torso) therefore pivots about its true place. Without an elevation twin the depth is the bone origin: author such a part as a `solid` (its 3D definition has the depth built in). Vector: `pedestal` in `🧫️fixtures/🧊️axial-twist` (torso tilted 14 degrees: plan shift 0.95 sin 14 = 0.229826 m), numpy and shapely oracle updated.

STATUS for QA1/QA5/QA7/QA8/QA9/QA12: landed and documented above in "BROADCAST: one 3D path, elevation solids, one outline per solid": the single 3D path (no old path), elevation solids, solid outline union (one ring per solid), and end-on carry (extruded solids on end-on bones, authoring rule 4). Ticket-local config 209 of 209 tests, `tsc` clean except `d3-format`.

## BROADCAST: revolved axis = the axis bone's own rest direction — TS (QA12 mixer drum)

A `revolved` solid now revolves about the LONGITUDINAL axis of its `axisBone` (the bone's local x axis: its own rest direction, whatever its rest tilt, carried by the pose), not about the world vertical. A drum bone with a rest angle of 166 degrees revolves about its own tilted axis (no more 5.36 m plan circle). `axisX` still shifts the axis sideways in the rest pose (the axis passes through the rest point `(axisX, rest height of the bone origin, axisZ)`), `axisZ` is a lateral shift, `mirror: false` draws the half profile as before (its sign reference is the carried forward direction, or the bone normal for an axis that lies along the forward direction).

Authoring rule: **`axisBone` must be a bone that lies ALONG the symmetry axis of the part** (pole, mast, trunk, drum, hub, lid hinge-normal bone), not a bone that merely sits on it. Where an asset named a horizontal or otherwise sideways bone as `axisBone` and relied on the old vertical axis, name the pole/stem/axle bone instead (or add a zero-length bone along the axis).

Vector (language-agnostic, hand-computed): `hub` in `🧫️fixtures/🧊️axial-twist` (foot whose rest axis is horizontal: disc of radius 0.1 m in front, 0.22 m by 0.2 m band in plan and elevation); the tilted-pose revolve is covered by the drum on the torso tilted back 14 degrees. Oracle `🧪️tests/🧊️lateral-spread/🐍️.py`: axis = third-party rotation matrix column of the axis bone. Config 210 of 210.

Whole-catalog three-views guard after this change (8 of 322 failing): NEWLY failing because their solids name a non-axial `axisBone`: **market-parasol** (all poses at 1:50: front y 2.96 vs elevation 2.46, plan y +-1.18 vs front +-1.00; the canopy bone is not along the pole), **bar-stool** (default pose consistency), **date-palm** (sweep, pose storm 1:50: front x [-2.45, 3.05] vs plan y [-2.83, 3.07]). Not changed by this fix, still open from other workers: person-with-umbrella, pregnant-person, running-child, seated-reader (default-pose consistency). Previously failing and now passing: herring-gull, mallard-duck, parent-with-stroller, person-with-rollator.

## Twist limits of a joint (`twistMinDeg`, `twistMaxDeg`) — QA14

A revolute joint may declare `twistMinDeg` (-720..0) and `twistMaxDeg` (0..720); without them the twist stays bounded to -180..180 as before (the spread keeps its fixed ±180). `effectiveOffset` (`S/🔨️modules/🦴️rig/🟦️.ts`) clamps `twistDeg` to them, the validator `joint-limit` compares the authored twist with the effective one and reports the joint's own limits, schema `Joint` (json + twin) gained the two fields, `BoneOffset.twistDeg` is -720..720 (the joint decides). Used by `spiral-stair` bone `walker-turn` (0..360: a person circling the column azimuth 11.25 .. 326.25 degrees, replacing the two-bone workaround). Test: vectors `fullTurnJoint`, poses `turn-270-full`/`turn-360-full`, frames and `limits` in `S/🧫️fixtures/🧊️axial-twist`, scenario "A joint may declare whole turns" in `S/🧪️tests/🧊️axial-twist/🥒️.feature`; vitest `🧊️axial-twist` passes (run with `T/🧰️TS/vitest.lateral.config.ts`, 216 tests in the 10 files of that config pass).

## Section views — READY (FW2)

A front or plan view that is drawn as a section is now declared per representation and per viewpoint, and named as such everywhere.

- **Schema:** `AuthoredRepresentation.sections` (optional, sibling of `geometry`) = `{ front?: ViewSection, plan?: ViewSection }`, `ViewSection` = `{ plane: number, label: { en, de } }` (JSON `$defs/ViewSection`, twin `ViewSection` in `S/🧬️schema/🟦️.ts`; the name `Section` was taken by the member cross-section). `plane` is metres along the view axis: **front** = the forward position x (the elevation x coordinate) of a vertical cut plane across the forward axis, seen from the front (the part behind it, x < plane, remains); **plan** = the height y of a horizontal cut plane, seen from above (the part below remains). `label` is the section name, e.g. `A-A` / `B-B` (the same in both languages unless an asset needs otherwise). The side view (`elevation`) is the rig plane and has no section of its own.
- **Validator:** `section-plane-outside` (en/de): the plane must lie within the default-pose extent of the asset (x extent for front, y extent for plan, plus 15 mm).
- **Captions** (`S/🔨️modules/🎨️depiction/✂️section/🟦️.ts`, exported from the package as `viewpointCaption`, `sectionOf`, `sectionMarker`): "Section A-A" / "Schnitt A-A" instead of "Front view" / "Vorderansicht" (plain views read "Side view", "Front view", "Top view" / "Seitenansicht", "Vorderansicht", "Draufsicht"). Used by the preview `--views` sheet captions (now wrapped to their cell), by the title line of front and plan depictions, and by the React viewer (viewpoint control and tag).
- **Cut-line marker:** the elevation of a sectioned asset shows, at 1:100 and 1:50 only (annotation labels on), a thin chain line across the object at the plane with 4 mm thick ends, 6 mm beyond the object, a 5 mm viewing arrow at each end (front: pointing backwards to -x; plan: pointing down) and the section letters beside the arrows (`A-A` gives A and A). Items are `section:front:*` / `section:plan:*` in the annotations layer.
- **Cast index:** `section` (`{ plane, label }`) is written to the cast index when the cast's viewpoint is a sectioned front or plan view at its scale.
- **Content:** `sections` set on all authored representations of `bookshelf` (front A-A at x = 0.4 m, through the middle of the 0.8 m width), `three-seat-sofa` (front A-A at x = 1.0 m, the middle seat), `curtain-wall-bay` (front A-A at x = 0.7 m, plan B-B at y = 2.075 m, the middle of the vision glass), `masonry-wall-with-lintel` (front A-A at x = 1.25 m, the middle of the opening; plan B-B at y = 1.5 m, half height of the opening) and `sliding-door` (plan A-A at y = 1.0 m). Script: `T/🧰️FW2/sections.py` (atomic writes; only the `sections` field was added).
- **Tests:** `S/🧪️tests/✂️section-views` (Gherkin, hand-computed vectors in `S/🧫️fixtures/✂️section-views`, shapely oracle for the marker geometry), run with `T/🧰️FW2/vitest.section.config.ts` together with the two layout tests below.

## Layout fixes (FW2)

- **Dimension clearance:** the overall dimension now starts its extension lines 1.5 mm (`DIMENSION_GAP_MM`) right of the drawn geometry of the viewpoint (shape boxes, not the elevation bounds, which were wrong for the front view) and stands 6 mm (`DIMENSION_OFFSET_MM`) beyond the drawing, so leaders and ticks never cross the glyph at 1:500 or in the 1:200 plan. Test `S/🧪️tests/📐️dimension-clearance` (shapely distance of every dimension line to the geometry box; retractable-bollard 1:500, street-lamp 1:200 and every authored scale of both, all viewpoints).
- **Caption and scale-bar fit:** text is measured with the Helvetica and Helvetica-Bold advance widths (`textWidthMm` in `S/🔨️modules/📐️units/🟦️.ts`, with 4 % slack) instead of 0.56 em per character, so title, subtitle and scale-bar labels are reserved their real width and cannot run past the scene. Sheet captions wrap to the cell (`captionLines`). Test `S/🧪️tests/🔤️caption-fit` (PyMuPDF `helv`/`hebo` as reference; gas-boiler and gas-hob 1:200, gas-hob 1:100, ceiling-diffuser 1:50, all viewpoints, en and de).
