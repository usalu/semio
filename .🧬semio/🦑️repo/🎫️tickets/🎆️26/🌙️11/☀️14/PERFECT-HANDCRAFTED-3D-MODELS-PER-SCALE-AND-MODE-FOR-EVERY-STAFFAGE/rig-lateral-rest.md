# Rig: full 3D rest placement per bone (2026-11-22)

`S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = this ticket folder. Product documentation: `S/README.md`, section "Rest placement of a bone". This note is the authoring guide for the human and animal authors, followed by what was changed and the test results.

## 1. Authoring guide

### 1.1 The fields

A bone frame has three axes: `u` along the bone (local x), `v` its normal (local y), `w` its joint axis. World axes are `[ahead, up, left]`. Under the usual upright sagittal parent `w` is the left of the asset.

| `Bone` field | default | turns the bone about | positive sense |
| --- | --- | --- | --- |
| `x`, `y`, `z` | `z` 0 | — | origin in the parent frame, along the parent's `u`, `v`, `w` |
| `angleDeg` | required | the parent's joint axis `w` | `u` towards `v` (counter-clockwise in the side elevation under a sagittal parent) |
| `abductionDeg` | 0 | its own normal `v`, after `angleDeg` | `u` towards `w` (to the left under a sagittal parent); range -180..180 |
| `twistDeg` | 0 | its own axis `u`, after the other two | `v` to `-w` and `w` to `v`; range -180..180 |

- Order: `R(rest) = Rw(angleDeg) · Rv(abductionDeg) · Ru(twistDeg)`, each about the axis as already turned by the one before.
- A pose or clip offset (`BoneOffset.angleDeg`, `abductionDeg`, `twistDeg`) is applied after the rest placement in the same order, about the bone's **own rest axes**: `R(bone) = R(parent) · R(rest) · R(offset)`. So `angleDeg` in a pose always turns the bone about its own `w`, wherever the rest placement put that axis. Joint limits (`minDeg`, `maxDeg`, `twistMinDeg`, `twistMaxDeg`) limit these offsets.
- Children ride on the parent: their `x`, `y`, `z` are along the parent's `u`, `v`, `w`. The tip of the parent is `x = parent.lengthM, y = 0, z = 0`.
- The twist does not move the bone; it chooses which way its joint axis points, i.e. what an `angleDeg` offset does and how the children's fields read.
- Mirror rule (left to right): negate `z`, `abductionDeg`, `twistDeg` of the rest placement and of the offsets; keep `x`, `y`, `angleDeg`. With mirrored frames the same anatomical motion has the same `angleDeg` on both sides and opposite `abductionDeg` / `twistDeg`.
- A rig without any `abductionDeg` / `twistDeg` on a bone is solved by the plain planar code path, bit for bit as before.

### 1.2 Do not solve angles by hand: `rig-rest.ts`

```
cd T ; BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
$BUN rig-rest.ts <category>/<slug> [--bone ID]...                 # rest frame of every bone: origin, tip, u, v, w as [ahead, up, left]
$BUN rig-rest.ts --file <candidate 🔣️.json> [--bone ID]...
$BUN rig-rest.ts <asset> --solve BONE [--parent ID] [--at "x,y,z"] [--u "x,y,z"] [--w "x,y,z"]
```

`--solve` prints the paste-ready `parent, x, y, z, angleDeg, abductionDeg, twistDeg` that put BONE under `--parent` with its origin at the world point `--at`, its axis along `--u` and its joint axis along `--w` (every omitted value = as it is now; `--w` is made perpendicular to `--u`). The two uses:

- new lateral bone (it need not be in the rig yet; then `--parent`, `--at` and `--u` are required and `--w` defaults to the joint axis of the parent, made perpendicular to `--u`): `--solve clavicle-l --parent torso --at "<joint>" --u "<tip minus joint>" --w "-1,0,0"`. Corrected 2026-11-22: the tool used to refuse a bone that did not exist yet (`no bone "clavicle-l"`), so the recipe could only be followed after pasting a dummy bone; `rig-rest.ts` now solves it directly. The points are world points of the REST pose in the coordinates the frame list of the same tool prints (the root at its authored rest placement; checked on `standing-adult`: a new bone solved at the printed joint and direction of `clavicle-l` gets its three angles back to 0.07° from a direction rounded to three decimals);
- re-parent without moving: `--solve arm-l --parent clavicle-l` (run it on a candidate file that already holds the new parent). The limb keeps its world rest frame, so its own solids, 2D shapes, anchors, masses, regions, members, poses and clips stay valid as they are.

### 1.3 Worked example: clavicle and arm (exact right angles; fixture `S/🧫️fixtures/🦴️lateral-rest`)

Torso upright (`u` up, `v` back, `w` left), shoulder height 0.42 m up the torso, sternoclavicular joint 0.02 m beside the plane, clavicle 0.17 m, arm 0.6 m:

```json
{ "id": "clavicle-l", "parent": "torso", "x": 0.42, "y": 0, "z": 0.02,  "angleDeg": 0, "abductionDeg": 90,  "twistDeg": 90,  "lengthM": 0.17, "joint": { "type": "revolute", "minDeg": -15, "maxDeg": 40 } },
{ "id": "clavicle-r", "parent": "torso", "x": 0.42, "y": 0, "z": -0.02, "angleDeg": 0, "abductionDeg": -90, "twistDeg": -90, "lengthM": 0.17, "joint": { "type": "revolute", "minDeg": -15, "maxDeg": 40 } },
{ "id": "arm-l", "parent": "clavicle-l", "x": 0.17, "y": 0, "angleDeg": -90, "twistDeg": 90,  "lengthM": 0.6, "joint": { "type": "revolute", "minDeg": -60, "maxDeg": 170 } },
{ "id": "arm-r", "parent": "clavicle-r", "x": 0.17, "y": 0, "angleDeg": -90, "twistDeg": -90, "lengthM": 0.6, "joint": { "type": "revolute", "minDeg": -60, "maxDeg": 170 } }
```

Rest frames with the torso origin at height 1.06 m:

| bone | origin | tip | u | v | w |
| --- | --- | --- | --- | --- | --- |
| clavicle-l | [0, 1.48, 0.02] | [0, 1.48, 0.19] | left | up | back |
| clavicle-r | [0, 1.48, -0.02] | [0, 1.48, -0.19] | right | up | ahead |
| arm-l / arm-r | [0, 1.48, ±0.19] | [0, 0.88, ±0.19] | down | ahead | left |

- Clavicle: `abductionDeg ±90` lays it sideways, `twistDeg ±90` turns its normal up, so in a pose `clavicle.angleDeg +θ` lifts the shoulder on both sides (shrug), `abductionDeg +θ` retracts the left and protracts the right shoulder (mirror: `-θ` on the other side), `twistDeg` rolls the clavicle about itself.
- Arm: `angleDeg -90` points it down the clavicle's normal, `twistDeg ±90` brings its joint axis back to the left: exactly the frame of a sagittal hanging arm. `arm.angleDeg` flexes forward, `abductionDeg` spreads (left +, right −), as before.
- A lifted clavicle carries the arm outwards by the same angle. To let the arm hang: `arm-l.abductionDeg -θ`, `arm-r.abductionDeg +θ` (fixture case `shrug-hanging`: clavicles +30, arms ∓30, arm tips at [0, 0.965, ±0.167]).
- A sloping clavicle (acromial end higher by `s` degrees): `abductionDeg ±(90 − s)`, and the arm `angleDeg -(90 + s)`. A clavicle that also runs backwards has no round numbers: use `rig-rest.ts --solve`.

The real asset `humans/standing-adult` (clavicle from the sternoclavicular joint [0.055, 1.390, ±0.020] to the acromioclavicular joint [0.004, 1.420, ±0.165], 0.157 m, running 19° back and 11° up; glenohumeral centre [0, 1.3975, ±0.153] unchanged):

```json
{ "id": "clavicle-l", "parent": "torso", "x": 0.2247, "y": -0.0773, "z": 0.02, "angleDeg": 61.4374, "abductionDeg": 67.8405, "twistDeg": 147.6436, "lengthM": 0.15656, "joint": { "type": "revolute", "minDeg": -8, "maxDeg": 35, "twistMinDeg": -40, "twistMaxDeg": 5 } },
{ "id": "arm-l", "parent": "clavicle-l", "x": 0.14246, "y": -0.0196, "z": 0.00908, "angleDeg": -101.0285, "abductionDeg": 3.7653, "twistDeg": 108.6213, "lengthM": 0.3135, "joint": { "type": "revolute", "minDeg": -60, "maxDeg": 170 } }
```

(right side: `z`, `abductionDeg`, `twistDeg` negated; the right clavicle's twist limits are -5..40.) Shrug in `arms-crossed`: `clavicle-l/-r.angleDeg 7`.

### 1.4 Worked example: pelvis half and leg

Pelvis upright at hip-joint height 0.90 m (`u` up, `v` back, `w` left). The hip bone runs from the sacroiliac joint [0, 0.97, 0.03] down and out to the hip joint [0, 0.90, 0.085] (0.089 m, 51.8° below the horizontal); the thigh keeps the sagittal frame (`u` down, `v` ahead, `w` left):

```json
{ "id": "ilium-l", "parent": "pelvis",  "x": 0.07,    "y": 0, "z": 0.03, "angleDeg": -180, "abductionDeg": 38.1572, "twistDeg": -90, "lengthM": 0.08902, "joint": { "type": "revolute", "minDeg": -5, "maxDeg": 5 } },
{ "id": "thigh-l", "parent": "ilium-l", "x": 0.08902, "y": 0,            "angleDeg": -38.1572, "twistDeg": 90, "lengthM": 0.42, "joint": { "type": "revolute", "minDeg": -30, "maxDeg": 110 } }
```

Frames: `ilium-l` u [0, -0.786, 0.618], v [0, 0.618, 0.786], w back; `thigh-l` origin [0, 0.90, 0.085], tip [0, 0.48, 0.085]. `ilium.angleDeg +θ` hikes the hip (tip up and out). Right side: `z -0.03`, `abductionDeg -38.1572`, `twistDeg +90` on the ilium and `-90` on the thigh. These numbers come from the same solver as the asset's but were not run through the product.

### 1.5 Worked example: quadruped scapula

Thorax bone along the spine pointing ahead (`u` ahead, `v` up, `w` left), origin [0.30, 1.25, 0]. The scapula lies on the rib cage 0.13 m beside the plane, from its dorsal border down and forward to the shoulder joint (60° below the horizontal, 0.36 m), its lower end 12° further out; no twist, so its joint axis stays nearly lateral and `scapula.angleDeg` swings it in its own plane (protraction / retraction of the limb). The humerus hangs from its tip back and down in the sagittal plane:

```json
{ "id": "scapula-l", "parent": "thorax",    "x": 0.05, "y": 0.05, "z": 0.13, "angleDeg": -60, "abductionDeg": 12, "lengthM": 0.36, "joint": { "type": "revolute", "minDeg": -15, "maxDeg": 15 } },
{ "id": "humerus-l", "parent": "scapula-l", "x": 0.36, "y": 0,                "angleDeg": -60.5451, "abductionDeg": -5.967, "twistDeg": 10.4302, "lengthM": 0.30, "joint": { "type": "revolute", "minDeg": -40, "maxDeg": 40 } }
```

**Validation (corrected 2026-11-22: authors found that this example did not validate).** Between 01:59 and the afternoon of 22 Nov `validateAsset` reported `lateral-ambiguous` for every flat elevation shape on a bone resting out of the paper (error taken as `extent · sin θ`: 1.5 mm at 1:50 for this 0.36 m scapula at 12°). The validator maintainer closed this (`T/validator.md`, "lateral, typification, outline strokes, views", item (b)): the rest placement defines the plane of the drawing, so the rest drawing is exact and only what a pose adds is measured (`plateLean`). The example validates as written. What still holds: a pose or clip offset that turns such a bone out of its rest plane makes its flat shapes (and those of its descendants in front and plan) findings — draw those parts as `solid` shapes or bind the drawing to the parent (1.6, item 6); and give a child that shall swing sagittally its upright frame in the REST placement (the humerus line below), not by an offset that is always on.

Frames: scapula tip (shoulder joint) [0.526, 0.995, 0.205], u [0.489, -0.847, 0.208]; humerus u [-0.5, -0.866, 0], w left (what `rig-rest.ts --solve humerus-l --parent scapula-l --u "-0.5,-0.866,0" --w "0,0,1"` gives on such a rig; like 1.4, not run through the product). Right side: `z`, `abductionDeg`, `twistDeg` negated. Joint order of the fore limb: scapula – humerus – radius – carpus – cannon – pastern – hoof.

The same recipe gives an axle (`abductionDeg 90`, wheels revolve about `axis: "u"` of the axle bone) or a handlebar (child of the steerer, `abductionDeg ±90`); no vehicle was converted or tested.

### 1.6 What to re-bind after changing a rig

1. **The bone whose rest frame changed** (e.g. a former sagittal stub that becomes a lateral clavicle): every `volumes` solid bound to it, at every scale and in body, thermal and structure, is written in its (`u`, `v`, `w`) frame and must be re-expressed in the new frame, in the catalog file **and** in `T/volumes/<category>/<slug>.json`: `p' = Bnewᵀ · Bold · p` for points (`p0`, `p1`, loft `sections[].at`, `transform.translation`) and `q' = q(Bnewᵀ · Bold) ⊗ q` for `sections[].rotation` / `transform.rotation`. `rig-lateral-standing-adult.py` (`rebind`) does this for capsule, loft and ellipsoid; copy it. If your asset is generated by scripts (`rework-NN-<slug>-*.py`), change the scripts' frames instead, or the next build puts the old coordinates back.
2. **A limb that was re-parented without moving** (`--solve BONE --parent NEW`): nothing. Its poses and clips keep their meaning because its rest axes are the same.
3. **A limb whose rest frame moved** (e.g. shoulders made wider): its solids, elevation shapes, anchors, masses and members follow automatically (bone-local). Its **front and plan shapes do not**: they are drawn in world rest coordinates and stay where they were — shift them by the same world vector (front `x` = lateral, plan `y` = lateral) and check with `r3-2d.ts` / `r3-project2d.ts --check`. `solid` shapes with `sweep: "w"` state `z0`, `z1` in world z at rest: shift them too. Torso solids that must meet the limb (deltoid, hip) are yours to re-model.
4. **Poses and clip keys that moved the old bone** (e.g. a shrug written as the stub's twist): rewrite them on the new axes and re-solve the offsets of the children so they keep their posed direction; clip keys that inherit such a pose and override the child must be re-solved as well (`reshrug` in the same script).
5. **New bones** need their own solids where the rules ask for them: a bone of at least 5 % of the asset height (or with a `section`) is load-bearing and needs a `part: "member"` structure solid at every authored scale (`volumes-structure-coverage`); a bone with 2D shapes needs a body solid (`volumes-bones`). Give a new bone `mass` only if you take that mass away from its parent.
6. **2D shapes on a lateral bone itself**: elevation shapes live in the bone's (`u`, `v`) plane; on a bone that points out of the paper a flat elevation shape is edge-on and `validateAsset` reports `lateral-ambiguous` (also for a flat elevation shape of a child that an un-compensated clavicle lift carries out of the paper). Draw such parts as `solid` shapes — `{ "kind": "extruded", "sweep": "u", "z0": 0, "z1": <length> }` with the profile in (`v`, `w`) for a bar along the bone — or bind the drawing to the parent, as `standing-adult` does (its clavicles have no 2D shapes; the hoodie is drawn on the torso). Front and plan shapes on any bone are flat world-rest drawings and work unchanged. For `sweep: "w"` on a bone that is not sagittal, `z0`/`z1` minus the world z of the bone's rest origin are the local `w` coordinates. Revolved solids on a non-sagittal bone were not tested. Since 2026-11-22 afternoon `lateral-ambiguous` accepts a flat elevation shape on a bone that merely rests inclined (the 12° scapula of 1.5) and reports only a plane resting edge-on or the error a pose adds beyond the rest drawing: `validator.md`, item 12.
7. `dimensions.depthM`: every bone origin and tip must stay within half of it plus the tolerance (`bone-lateral`).

### 1.7 Checking

```
python3 <your edit script>                       # write the candidate to T/🗑️generated/<yours>/<slug>/🔣️.json first
$BUN rig-rest.ts --file <candidate> --bone clavicle-l --bone arm-l      # frames as intended? limb frame unchanged?
$BUN r3-validate.ts --file <candidate>           # no new finding against the catalog asset
$BUN r3-sheet.ts --file <candidate> --scale 50 --list structure          # front, back and top: the bone runs joint to joint, the skeleton did not move
$BUN r3-sheet.ts --file <candidate> --scale 50 --pose <arm-raising pose> # limb leaves from the tip of the girdle bone
$BUN r3-sheet.ts --file <candidate> --scale 50 --list structure --pose <shrug pose> --only front
$BUN r3-2d.ts --file <candidate> --scale 50 --pose <pose>
$BUN rig-lateral-compare.ts <catalog 🔣️.json> <candidate>               # largest move of any 2D shape / anchor per pose and clip instant (0 where nothing was meant to move); shapes are paired by id and bone (a changed depth order is no move), scales not authored in both documents are skipped and named
```

In the sheet the magenta line is the bone from its joint (yellow dot) to its tip: a lateral bone shows in the front, back and top panels and is a dot in the side panels.

## 2. What was found and what was changed

State found (previous agent, killed 01:45): `S/🔨️modules/🦴️rig/🟦️.ts` complete (rest `twistDeg`, offsets about the bone's own rest axes, exact right angles); nothing else. Every consumer already read the one 3D chain (`solveChain` / `restFrames` / `carry`), so no second code path existed to remove.

| Area | Change |
| --- | --- |
| Schema `S/🧬️schema/🔣️.json`, twin `🟦️.ts` | `Bone.twistDeg` added (-180..180); `Bone`, `Bone.abductionDeg`, `BoneOffset` described with axes, order and senses. |
| `🔨️modules/🦴️rig` | as found (reviewed, covered by the new test). |
| `🔨️modules/🗂️catalog` (validator) | `lateral-ambiguous`: the tilt of an elevation shape is measured against the paper (the bone's `w` against world z) and the rule also covers bones that leave the sagittal plane at rest and their descendants; unchanged numbers for sagittal rigs. |
| `🔨️modules/🏗️structure/🧰️harness` | case bones state `abductionDeg` / `twistDeg` as rest placement instead of emulating them with offsets on a fake revolute joint. |
| `🔨️modules/🔗️contracts` | `BoneFrame` description. |
| Checked, no change needed | evaluation (shapes, solids, anchors, loads, masses, kinematics, depth), animation (grounding), volume skeleton and node placements (a revolute joint turns about the node's own `w`), frames, depiction, multiview, structure projection, thermal, exporters, react viewer (none solve a rig themselves), `r3-lib.ts`, `r3-sheet.ts`, `r3-2d.ts`, `r3-project2d.ts`, `r3-validate.ts` (all read `evaluate` / `restFrames`), the python staffage reader (builds a structure case, solves no rig). Checked by reading the code and by the tests below; the exporters (GLB, FBX, PDF, DXF) and the react viewer were not run on an asset with a lateral bone. |
| README | new section "Rest placement of a bone"; "Lateral spread" wording. |
| Test | `S/🧫️fixtures/🦴️lateral-rest/🔣️.json`, `S/🧪️tests/🦴️lateral-rest/{🥒️.feature, 🟦️.ts, 🐍️.py}`: 18 cases (hand-computed frames, extents and anchors; hanging arm equals the sagittal arm beside the plane; scipy / numpy / shapely oracle for frames, node placements at rest and posed, anchors, solid extents; schema via ajv; validator; sagittal rig bit-identical to the planar composition). It does not cover thermal regions, structural members or clips on a lateral bone. |
| Ticket tools | `rig-rest.ts`, `rig-lateral-compare.ts`, `rig-lateral-standing-adult.py`, `rig-lateral-test.sh`. |
| `humans/standing-adult` | `clavicle-l/-r` were sagittal 55 mm stubs along the antero-posterior axis with the arm 0.133 m beside them; they are now the anatomical clavicles (section 1.3). Arms re-parented without moving (rest frames equal to 2e-5). Shrug of `arms-crossed` and `hands-warming` and the seven `shiver` keys that inherit it and override the arms rewritten (clavicle `angleDeg`, arm offsets re-solved). All clavicle-bound structure solids (clavicle, scapula, scapular spine and neck, acromion, coracoid, glenoid, sternoclavicular joint; 42 in the catalog, 32 in the source) re-expressed in the new frame. Only `rig`, `poses`, `clips` and those solids were written; every other byte of both files is unchanged. |

Not done on the asset:

- Pelvis halves: the hip-bone solids sit on `pelvis` and would have to be re-bound in a volumes list another author works on; recipe in 1.4.
- Shoulder breadth was kept, so the brief's 0.39–0.40 m is **not** met: acromion tips at ±0.180 m (0.36 m biacromial), joint centres ±0.153 m. The figure follows the de Leva female table (1.72 m, 59 kg), for which I judged 0.36 m right; a 1.75 m male figure needs 0.39–0.40 m (acromioclavicular joint about ±0.18, joint centre about ±0.17) and then step 3 of 1.6. The asset's author should confirm or overrule this.
- `rework-01-standing-adult-{rig,skeleton}.py` (author 01's generator) still describe the old stub; running that build again would undo the conversion. The next author of this asset must move the clavicle frame into those scripts first.

Behaviour that changed for existing assets: bones that already had a rest `abductionDeg` (`lombardy-poplar`, `mallard-duck`, `pigeon`) now take their `angleDeg` offsets about their own tilted joint axis instead of the parent's; at rest nothing moves, in sway and wing poses the difference should be second order in the 5°–20° tilt. I did not render or re-validate these three after the change: `lombardy-poplar` was VALID before the validator edit, `pigeon` and `mallard-duck` were INVALID before (172 and 14 findings from their own rework). So "an asset with rest abduction is unchanged" is not claimed; only all-sagittal assets are bit-identical.

## 3. Test results (native vitest through `rig-lateral-test.sh`, bun)

| Test | Result |
| --- | --- |
| `🦴️lateral-rest` (new) | 18 / 18 (also after the asset change) |
| `🦴️forward-kinematics` | 7 / 7 |
| `🧊️axial-twist` | 33 / 33 (also after the asset change) |
| `🧊️solid-placements`, `🧊️solid-sweep`, `🧊️member-solid` | 3 / 3, 16 / 16, 11 / 11 |
| `🧬️schema-twin` | 3 / 3 |
| `🏗️rigid-trees-3d`, `🏗️structure-projection`, `🏗️cantilever`, `🌉️simply-supported`, `⚖️equilibrium` | 33, 53, 6, 7, 91: all pass (harness change; `rigid-trees-3d` also after the asset change) |
| `🦶️foot-plant`, `🏃️inertial-consistency` | 3 / 3, 8 / 8 |
| `🧊️lateral-spread` | 25 / 27 — the two herring-gull cases look for shapes `near-arm` / `near-hand` that the catalog gull does not have (failed the same way in the previous agent's log of 01:34, before my edits) |
| `🧊️model-skeleton` | 22 / 27 — node tree, pose replay, clip sampling, FBX replay pass; the five failures assert solids of other assets (elderly forearm radii, wheel centres, idler pins, dog lumbar vertebrae, goat torso junction) |
| `🧊️chain-state` | 19 / 20 — one case looks up a catalog shape that is not there (`shape.bone` of undefined) |
| `🧬️schema-conformance` | 42 / 43 — "extruded solid with an unknown sweep axis" is accepted by the schema; my schema edit touched only `Bone` and `BoneOffset`; cause not investigated |
| `🧍️front-projection`, `🧍️plan-projection` | 8 / 9, 10 / 13 — radiator width; shape ids of standing-adult, commuter and seated worker that the tests do not find (run before the asset change, not re-run after it) |
| `🧊️three-views` | 316 / 329 (run after the asset change) — `standing-adult`, `lombardy-poplar` and `mallard-duck` pass; twelve assets fail a `spread-*` agreement of their views and `elderly-with-cane` its elevation width. Eleven of the twelve have no rest abduction or twist, so their solve is the unchanged planar path; `pigeon` has rest abduction and was already INVALID, and I cannot say whether its failure predates the offset change |
| `🏗️structure-oracle` | no test file in that folder; nothing ran |

For the suites with failures I have no run from before the rig edit (except the 01:34 `lateral-spread` log), so "not caused by the rig" rests on what each failing assertion checks, not on a before/after comparison. No test was changed or weakened.

`humans/standing-adult`: `r3-validate.ts` gives the same findings before and after, line for line — 53 with the volumes source (37 `volumes-feature`, 16 `volume-anchor`), 466 for the catalog file alone. The asset was INVALID before and still is; the rig change adds and removes nothing. `--volumes` (the measured volume rules) was not run. `rig-lateral-compare.ts` before / after: under 0.01 mm in `relaxed`, `pointing`, `idle-breathing`, `fanning`; at most 3.1 mm in the shrug poses and `shiver` (the shoulder now rises on the arc of the real clavicle). That the re-expressed skeleton solids did not move is judged from the before / after structure sheets at 1:50, not measured. Sheets read: structure 1:50 default pose before and after (six views) and top alone, body 1:50 `pointing`, structure 1:50 `arms-crossed` front, structure 1:200 `pointing` front, 2D 1:50 `arms-crossed`. Scales 100 and 500 and the thermal list were not rendered. In the 2D of `arms-crossed` the sleeves draw as hatched slabs in side and front view; the arm frames are unchanged, so I take this as the asset's own 2D state, but I did not render that pose before the change.

Not run: `repo run @teaching/staffage:typecheck` and the whole-catalog suites (`✅️asset-validation`, exports) — the graph commands are off limits and the catalog is mid-rework. The new code ran under bun's transpiler only, so type errors would not have been caught.

## 4. Verification (2026-11-22, second agent)

Everything below was run natively (bun, `tsc`, vitest through `rig-lateral-test.sh`, the react config directly), one process at a time. Scripts kept in `T`: `rig-verify-standing-adult.ts`, `rig-reoffset.ts` (+ `rig-reoffset.done.json`), `rig-verify-render.sh`. Corrections to sections 1.2 and 1.5 above are marked "corrected 2026-11-22" in place: `rig-rest.ts --solve` now takes a bone that is not in the rig yet (`--parent`, `--at`, `--u`), and the scapula example of 1.5 validates since the validator maintainer changed `lateral-ambiguous` to measure only what a pose adds to the rest plane (my request and the answer are in `T/validator.md`).

### 4.1 Typecheck

`tsc -p <tsconfig> --noEmit --incremental false` (TypeScript 5.9.3):

| Project | Result |
| --- | --- |
| `S/📦️packages/🟦️typescript` (modules, schema, tests) | one error from the change: `🧪️tests/🦴️lateral-rest/🟦️.ts(172)` passed `readonly Bone[]` to a mutable parameter; fixed (`dressed` takes `readonly Bone[]`). Left: 4 errors in `🧪️tests/🧊️volumes-connected/🟦️.ts` (157, 180, 242, 282: cast of `ScaleVolume` to an authored list). That file is new, untracked work of the volume rules and does not touch the rig; not mine to change, reported here. |
| `S/🎯️targets/⚛️react/📦️packages/🟦️typescript` | 0 errors (also with the new react case). |
| `S/🌐️site/📦️packages/🟦️typescript` | 0 errors in staffage; 1 in `🧰️framework/🛍️products/🦑️repo/…/⚡️caching/🌐️vite/🟦️.ts(121)` (`ResourceLeaseControl` not found): the repo product, unrelated. |
| Ticket tools `r3-lib`, `r3-sheet`, `r3-2d`, `r3-project2d`, `r3-validate`, `rig-rest`, `rig-lateral-compare` | none in these files; 4 older errors in what they import (`volumes-base.ts(100)`, `volumes-lib.ts(21, 703)`, `volumes-silhouette.ts(130)`). `volumes-base.ts(100)` casts `SolvedFrame` to a mutable frame; the `readonly` vectors are older than the change (context lines of the diff), so not caused by it. |

### 4.2 Consumers of bone frames

Reading the code again confirmed section 2: no consumer solves a rig itself. Two things were proven by running them.

**(a) New test cases** (fixture block `consumers` in `S/🧫️fixtures/🦴️lateral-rest/🔣️.json`: hand-computed boxes of the clavicle lifted by 30° and 15°, the capsule box, the joint load of the hanging arm). `S/🧪️tests/🦴️lateral-rest`: 18 → 29 cases, all pass; `S/🎯️targets/⚛️react/🧪️tests/🧊️model-lateral-rest` (new, 3 cases, pass).

| Consumer family | What is asserted | Read back by |
| --- | --- | --- |
| depiction, multiview (`resolveViews`) | the lifted clavicle in the hand box of elevation, front and plan, light-solid and dark-line, 1e-6 m | — |
| SVG | same box in the written file, 0.5 mm | jsdom, svgelements |
| PDF | paint inside the box, paper beside and above it | PDFium at 1200 dpi |
| DXF, DWG | a polyline with exactly that box per view, 1e-6 m | ezdxf, ezdwg |
| sheet | three clavicles at the sheet origins (SVG); three polylines of the hand sizes in the sheet DXF | jsdom, ezdxf |
| frames | the 20 fps schedule holds 0.25 s and 0.5 s; box and bone tip there equal the hand values for 15° and 30° | — |
| cast, levels of detail | `logical.partsM` of a pose cast; the frame at 0.5 s of the full (20 frames) and preview (10 frames) level of a clip cast, from a scratch catalog | jsdom |
| 3D model (`modelOf`, the viewer's `modelOfAsset`) | kernel replay of pose and clip without a clamp; clavicle tip and arm origin at the hand tip (1e-9 m pose, 1e-4 m between clip keys) | kernel `poseModel` |
| GLB, FBX | the clavicle capsule under its node in the hand box (2 mm, tessellation); FBX clip replayed, arm origin on the hand tip at both instants (1 mm) | three `GLTFLoader`, `FBXLoader`, `AnimationMixer` |
| structure | 36.7749 N shear and 6.2517 N·m bending at the sternoclavicular joint, linear to zero at the tip; joint loads of clavicle and arm in all 7 cases | numpy (`r × F` over the subtree) |
| structure projection | bending diagram drawn over the hand baseline in front, no ordinate in the side elevation (looks along the bone), out-of-plane mark in plan | — |
| thermal, forces, moments views | the clavicle shape carries its region and sits in the hand box in the three views | — |
| react 3D stage | `prepareModel`: tessellated clavicle solid in the hand box, arm on the tip, clip instants; `ModelStage` becomes ready for the pose and the clip without an alert | kernel, jsdom |

Not covered by a case: PNG sequence / MP4 / WebM rasterisation (they rasterise the SVG frames asserted above; `rsvg-convert` and `ffmpeg` were not run), transport emitters on a lateral bone, the python reader (it reads casts; it solves no rig, checked in `🐍️.py`).

**(b) `humans/standing-adult`** (`bun rig-verify-standing-adult.ts`; 4 poses, 2 instants of each of the 3 clips, 1:50). Largest difference to `evaluate`:

| Consumer | Largest difference |
| --- | --- |
| drawing scene (what SVG and PDF serialise): arm, forearm and hand shapes, three views | 0.50 mm |
| CAD scene (DXF / DWG): extent of all outlines against the drawn shapes | 0.41 mm |
| GLB loaded by three: every bone node, 4 poses, 3 scenes | 0.05 mm |
| FBX replay by three: every bone, 3 instants per clip | 0.34 mm |
| model replay (kernel): poses `relaxed`, `arms-crossed`, `pointing` and both instants of `idle-breathing`, `fanning` | 1e-15 m; `shiver` between keys 0.34 mm |
| structure: joint moment of each clavicle against `r × F` of its subtree summed independently | equal to 4 decimals ([4.2828, 0, 1.1606] N·m left, [-4.2816, 0, 0.9488] right; 2.647 kg hang from each clavicle) |

Sheets read as images: `arms-crossed` light-solid and moments, `pointing` light-solid (first-angle sheets, 1:50). `pointing` is right in all three views (arm from the shoulder, plan and front agree). Thermal solved; its sheet was written and not read.

Findings on the asset, not caused by the rig (for author 01; asset not touched):

- `hands-warming`: the kernel replay clamps `thumb-metacarpal-l/-r`, `index-proximal-l/-r`, `middle-proximal-l/-r` (`joint-limit`), so the 3D model shows those fingers up to 5.5 mm from the 2D evaluation. Their offsets have `twistDeg 90` at `twistMaxDeg 90` together with an angle and an abduction; the kernel measures the twist of the composed rotation (swing-twist), which then exceeds 90°. These bones have no rest abduction or twist, so their solve is the unchanged path. Either keep the pose inside the limit or give the mechanism joint the staffage order of rotations; see "still open".
- `arms-crossed`: the sleeves draw as large hatched slabs in the front and side elevation (seen again in the sheet). The arm frames differ from before the rig change by at most 3.1 mm (section 3), so this is the 2D state of the sleeves.
- Sheets of the `forces` / `moments` views: the width dimension of the front and side view is drawn below the legend, its extension lines cross the legend text. Sheet layout, unrelated to the rig.

### 4.3 What the rig change did to existing assets (new finding)

Section 2 named three assets with a rest abduction. Measured over the catalog with `rig-reoffset.ts` (old reading `Rw(rest + θ) · Rv(a + φ) · Ru(τ)` against the present chain), five assets authored before the change were moved by it, one of them badly:

| Asset (file time before my write) | Bones with rest abduction | Largest move of a bone tip | Done |
| --- | --- | --- | --- |
| `humans/elderly-with-cane` (01:27) | 34 (`hand-r` -50° carries the cane) | 260 mm: the cane tip left the ground by up to 62 mm in every pose and both clips | re-solved; cane tip at y 0.000 again; `validateAsset` 10 → 2 |
| `animals/pigeon` (00:58) | 40 feather bones | 11.5 mm (opened wings, take-off) | re-solved; 172 → 172 findings |
| `vegetation/lombardy-poplar` (01:26) | 37 | 8.3 mm at a foliage tip of a 25 m tree (storm) | left as authored (see below) |
| `humans/parent-with-stroller` (22:05, 21 Nov) | 12 (the child's limbs) | 5.0 mm | re-solved; 193 → 193 |
| `animals/mallard-duck` (20:43, 21 Nov) | 4 | 1.2 mm (`wing-stretch` only) | re-solved; 14 → 14 |

Re-solve = `R(offset') = Rv(a)ᵀ · Rw(θ) · Rv(a + φ) · Ru(τ)` per bone, pose and clip key, so every bone has exactly the frame it had before the change (largest difference afterwards under 0.01 mm). Only the text of `asset.poses` and `asset.clips` was replaced (file re-read immediately before, every other key compared equal), a line is in each asset's report under `T/rework/assets/`. The tool refuses a second run on the same asset (`rig-reoffset.done.json`).

`lombardy-poplar`: the re-solve was written and taken back. It puts abduction and twist parts of thousandths of a degree on all 37 bones, which makes them "spread" for the validator: 48 findings with the re-solved offsets (37 `lateral-ambiguous`, 11 `elevation-lateral`) against 1 with the author's offsets under the rule as it stands since 12:16 (36 at 12:12 under the rule before), for 8 mm on a 25 m tree. `poses` and `clips` hold the author's offsets again (round-trip to 1e-4 degree, no abduction or twist offset; file 14 395 412 → 14 395 426 characters).

Not touched, for their authors: `vegetation/english-oak` (32 bones with rest abduction and angle offsets; old against new reading differs by up to 20 mm, storm; the file was rewritten at 03:38, after the change, so I cannot tell which reading it was authored for — author 12: if the poses were solved before 01:45, run `bun rig-reoffset.ts vegetation/english-oak` and decide). `objects/market-parasol` has rest abduction but no angle or twist offset on those bones: unaffected. All other assets with rest abduction or twist were written after the change.

Rendered and read (`rig-verify-render.sh`: every pose and two instants of every clip with `r3-sheet.ts` and `r3-2d.ts` at 1:50; 128 images for duck, pigeon and poplar). Read: duck `wing-stretch` (sheet and 2D), duck `dabble` 2.8 s; pigeon `wings-half-open`, `airborne-wings-down` (sheets), `takeoff-wings-up` (2D); poplar `storm` (sheet and 2D), `wind-sway` 0.96 s. All read naturally: wings open symmetrically with regular feather fans, the upended duck is coherent, the poplar leans to lee in one curve. The other 118 images were rendered and not read; for them the claim rests on the numeric identity with the pre-change frames (duck, pigeon) and on the 8.3 mm bound (poplar). Seen and not changed: the pigeon's 2D wing in the opened poses is a stick with stepped blocks (its author's report `rework/08.md` already says the 2D wing was not redrawn).

### 4.4 The failing suites, case by case

How "caused by the rig change" was decided: the diff of `S/🔨️modules/🦴️rig/🟦️.ts` changes the solve only for a bone with a rest `abductionDeg` or `twistDeg` (for every other bone `oriented` is the old expression `Rw(rest + θ) · Rv(φ) · Ru(τ)`), so an asset without such a bone is solved exactly as before; and `git show HEAD:<asset>` (commit of 21 Nov 18:13, before the change) shows whether a shape id a test looks for was already gone.

| Suite | Before → now | Verdict per failing case |
| --- | --- | --- |
| `🧬️schema-conformance` | 42/43 → 43/43 | Outdated fixture. The "unknown sweep axis" case used `sweep: "u"`, which has been a valid axis since the sweep enum became `u, v, w` (the test next to it accepts all three). Fixture now uses `"x"`. Not the rig. |
| `🧊️chain-state` | 19/20 → 20/20 | Outdated id. The case took the shapes `upper-arm-r/-l` of `person-with-umbrella`; the reworked asset (already in HEAD) draws the upper arms as `sleeve-r/-l` on the same bones. The case now takes the shape of the bone `upper-arm-r/-l`, which is what its title says; assertion unchanged. Not the rig. |
| `🧊️lateral-spread` | 25/27 → 27/27 | Outdated ids. The two gull cases took `near-/far-arm` and `near-/far-hand`; the gull (in HEAD) draws the wing as coverts, secondaries and ten primaries per side. The cases now take every shape on the bones of each wing (`humerus-n/-f` and their descendants); the three assertions per pose are unchanged and hold. Not the rig. |
| `🧍️front-projection` | 8/9 → 9/9 | Outdated number. Front cast of `panel-radiator` 1.2025 m against a band of 1.0–1.2: the asset (in HEAD, sagittal rig) is 1.1025 m across (`dimensions.depthM`: 1.0 m panel, valve head 0.095 m beyond one end, plug 0.0075 m beyond the other) plus 0.05 m canvas margin per side. Band moved to 1.1–1.3 (same width); reason written into the fixture description. Not the rig. |
| `🧍️plan-projection` | 10/13 → 12/13 | Two outdated ids fixed: the arm span took shapes named `upper-arm*`, the lean took `shoulders`; `standing-adult`, `walking-commuter`, `seated-office-worker` (in HEAD) draw `sleeve-l/-r` on `arm-l/-r` and `shirt` on `torso`. The cases now go by bone (`armSpanM.bones`, `lean.bone` in the fixture); spans 0.47–0.51 m inside the unchanged 0.36–0.58 band, lean travel 0.147 m against the unchanged 0.04. **Still failing, asset mid-edit:** "moves the commuter's foot in plan exactly as the independent kinematics predicts" needs the capsule `shoe-heel-l`; `walking-commuter` (author 01, file 03:19, report 03:32) draws the shoe as the rigid poly `shoe-rear-l`. The exact prediction of the case is for a capsule that follows the bone ends; for a rigid poly it would have to replicate the depth of the elevation twin. Left red rather than loosened; to be re-pinned when the commuter's plan shoe is final. Not the rig (legs are sagittal). |
| `🧊️model-skeleton` | 22/27 → 18/27 | All nine assert the solids of catalog assets that their authors are rebuilding (four more than at 02:00 because the rebuild went on). Eight fail on the presence, the section or the place of authored solids in the volume lists (a lookup by id that finds nothing, or a measure between two solids); the ninth (goat torso junction) poses `pelvis`, `lumbar`, `thorax`, which are sagittal bones. I did not prove for `parent-carrying-toddler` (8 bones with rest abduction, written 03:50, after the change) that its 2.2 mm are independent of the pose solve; its author wrote the asset under the present rig. Assets and findings: `goat` (no solid `mammary-suspension`; barrel junction sections 46 mm apart), `elderly-with-cane` (`radius-l` shaft differs by 1.8 mm between scales, limit 0.1 mm), `city-car` / `delivery-van` / `vintage-car-1929` ("Missing wheel centre"), `red-squirrel` (0 of 4 guard locks), `excavator` (idler clearance 37 mm for 2 mm), `labrador-dog` ("Missing lumbar vertebra L1"), `parent-carrying-toddler` (`th-toddler-nose-breath` 2.2 mm from its landmark, limit 1 mm), `parent-with-stroller` (no body solid on `child-hand-l` at one scale; the case passed at 12:50 and failed at 13:40, the volume lists were rewritten at 13:16 by their author; my write of 12:23 changed `poses` and `clips` only and is still in the file). Assets not touched, tests not changed. |
| `🧊️three-views` | 316/329 → 316/329 | `elderly-with-cane` (elevation width) was caused by the rig change and passes after the re-solve of 4.3. The 13 that fail now are all `spread-*` disagreements between the authored drawings of two views. Six have no bone with rest abduction or twist (`child-on-kick-scooter`, `horse`, `office-swivel-chair`, `person-with-umbrella`, `person-with-white-cane`, `sleeping-person`): unchanged solve, the drawings disagree. Five were rewritten by their authors after the change (`herring-gull` 12:36, `red-fox` 12:56, `worker-carrying-box` 12:29, `parent-carrying-toddler` 03:50, `seated-office-worker` 02:54): mid-edit. `parent-with-stroller`: the front drawing reaches 0.10 m below the ground at 1:500 in every instant, also where no bone is turned (tolerance 0.054 m; the rig moved it by at most 5 mm, undone in 4.3). `pigeon`: at 1:100 in `take-off` 0.43 s the front drawing starts 21 mm above the elevation (tolerance 20 mm); its 2D wing was not redrawn (4.3). |

No test was weakened: three fixtures were corrected (one enum value, one band shifted, two selections by bone), three cases select shapes by bone instead of by a vanished id, every assertion and tolerance is as it was.

### 4.5 Still open

- **Mechanism joint against pose twist** (4.2, `standing-adult` `hands-warming`): the 3D model clamps a bone whose pose combines a twist at its limit with an angle or abduction, because the kernel limits the swing-twist twist and the staffage rig the third Euler angle. Shows as up to 5.5 mm between the 3D model and the drawings of that pose. Not from the rest placement; needs a decision in `S/🔨️modules/🧊️volume/🦴️skeleton` (limit in the staffage order) or in the pose.
- `🧍️plan-projection` commuter stride (4.4) and the asset findings of `model-skeleton` and `three-views`: with the authors.
- `vegetation/english-oak`: which reading its poses were solved for (4.3).
- Typecheck: `🧪️tests/🧊️volumes-connected` (4 errors) and, since about 13:00, `🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts(190)` (4) and `🧪️tests/🧊️volumes-rules(583)` (2) from the volume-rule work in progress; the react project then also reported 10 errors in `🧰️framework/🔨️modules/🖱️ui/🧱️elements` (two copies of `@types/react` in the package cache), where it had 0 at 02:55. None is in a file of the rig change.
- Not run: `✅️asset-validation` and the other whole-catalog suites, PNG/MP4/WebM rasterisation, `--volumes` on the re-solved assets.
