# lateral-ambiguous: solids for pregnant-person, sleeping-person, tour-guide

`S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `T` = this folder. Assets: `S/🗂️catalog/👤️humans/<slug>/🔣️.json`. Outputs and before-copies: `T/🗑️generated/lateral/` (delete when done).

## What the rule demands
`validateAsset` (`S/🔨️modules/🗂️catalog/🟦️.ts`, `lateral-ambiguous`): every shape that has no `solid`, is not `role: ground`, and sits on a bone that is spread or twisted (own or ancestor `abductionDeg`/`twistDeg` in any pose or clip key) must have `solid` if `extent * sin(max tilt out of the view plane) * 1000 / denominator` exceeds 0.2 mm. The rule applies to all three views (elevation too), at all four scales. Passing human assets give every such shape an extruded (`sweep w` elevation, or `v` plates) or revolved (`axis u`, `mirror`) solid. Findings before: pregnant-person 6, sleeping-person 2, tour-guide 11 (19). After: 0, 0, 0 (and no other rule: the extent-vs-dimensions checks needed care, see below).

Observations that shaped the work:
- Geometry of a solid shape is in the bone-local (u, v) frame; flat front/plan shapes are in world coordinates, so every front/plan flat shape had to be redrawn (profile + hint), not only annotated.
- A revolved solid takes its profile radius from the v offset, and `axisX` shifts the axis without shifting the profile: the profile is compensated by `axisX * v.x` (vertical bones only). `lateralScale` is validated but not used by the 2D evaluator, so no elliptic cross-sections are possible; plan ellipses (shoulders, head) become rectangles or circles.
- An open polyline (detail or texture line) with a `solid` evaluates to no rings at all (existing weakness of the passing assets). Open elevation lines were therefore redrawn as closed 2 mm ribbons (still `filled: false`); open front/plan lines became thin plates (`extruded v`) or thin bars (`extruded w`).
- Extruded plates with the true volume thickness overshoot at oblique abduction (square vs round section) and broke the default-pose extent checks. Elevation solids were therefore chosen per shape by IoU against the posed volume over all poses and clip instants: thin plate (10 mm, reproduces the old flat projection), full-thickness plate, or revolved (only when the rest silhouette stays at least 0.97 IoU of the old).

## Changes (84 + 16 + 184 = 284 shape objects, exact-string replacements, `volume` untouched)
Method: `T/lateral-convert.ts` (candidates from the old outline and the volume projection, selected by IoU with the posed volume, then rest IoU, then default-pose extent fit), applied by `T/lateral-apply.py` (re-reads the file, checks the old text, one replacement per shape, JSON re-parsed). Per shape plan: `T/🗑️generated/lateral/plan-<slug>.json` (candidate scores, evidence, pick).

| asset | view | shapes (every scale 50/100/200/500 where they occur) | solid added |
|---|---|---|---|
| sleeping-person (16) | front | foot-l, foot-r (`sil-foot-*` at 500) | `extruded v` plate, profile (u,w) from the old outline, thickness 6 mm |
| | plan | same 4 | `revolved u`, mirror, disc radius from the lateral half width, `axisX` compensated |
| pregnant-person (84) | elevation | sleeve/forearm/hand/thumb l,r, cuff-rib, fingers (`sil-*` at 500) | mostly `extruded w` thin (10 mm centred on the volume lateral centre); 2 full plates, 1 revolved; open lines as ribbons |
| | front | shoulder/elbow/fist ellipses, cuffs, `upper-arm` | `revolved u` (outline envelope), a few `extruded v` plates |
| | plan | arm, forearm, hand capsules | `revolved u` (profile of the elevation outline, scale 1, 0.85 or 0.72), `extruded w` 0.8 |
| tour-guide (184) | elevation | head, hair, face details, hands, arms, torso details, pointer, pole, flag | `extruded w` thin (81), full plate (4), 1 revolved |
| | front | torso, neck, head parts, arms, hands, fingers, badge, lanyard, pole grip, pointer | `revolved u` (arms, hands, head, torso), `extruded v` plates (hair, cap, lines, zip, hem), few `extruded w` |
| | plan | arms, hands, trunk, shoulders, head, face, cap, seams, parting | `revolved` discs / scaled twin profile, `extruded w` rectangles for ellipse-like parts, bars for open lines |

Exact solid per shape/scale: see `plan-<slug>.json` (`new.solid`, `pick`).

## Verification
- `validateAsset`: 0 findings in all three (`findings-before.txt`, `findings-after.txt`). Schema conformance and all other catalog checks pass in the library suite below.
- Elevation vs before: default pose of sleeping-person 1.0000 (identical); tour-guide 0.9988 to 0.9993 at 1:50 to 1:500 (only the 2 mm ribbons differ, closed body shapes identical to the mm); pregnant-person 0.986 to 0.995 (default pose has abducted arms: a solid projects differently there, this is the correction the rule asks for). Minimum over all poses and four clip instants: sleeping 1.000, tour-guide 0.962, pregnant 0.986 (see `snap-<slug>.txt`).
- IoU against the volume at rest, before -> after (`snap-<slug>.txt`, same function as `volume-check.ts`):
  - sleeping-person 1:50: elevation 0.976 = 0.976, front 0.9376 -> 0.9430, plan 0.9671 -> 0.9665 (1:500: 0.9396 -> 0.9454, 0.9671 -> 0.9664).
  - pregnant-person 1:50: elevation 0.9638 =, front 0.9663 -> 0.9647, plan 0.9260 -> 0.9315; 1:200: 0.9547 -> 0.9531, 0.9334 -> 0.9314; 1:500: front 0.8260 =, plan 0.7512 -> 0.7856.
  - tour-guide 1:50: elevation 0.9436 -> 0.9420, front 0.9542 -> 0.9508, plan 0.9003 -> 0.8918; 1:100 plan 0.8921, 1:200 plan 0.8915; 1:500 front 0.7768 -> 0.7801, plan 0.8229 -> 0.8217.
- IoU against the posed volume (every pose + 4 clip instants, mean; `posed-<slug>.txt`): improved or equal in nearly all cells, e.g. tour-guide front 1:50 0.9074 -> 0.9152, plan 0.8354 -> 0.8558, elevation 0.9280 -> 0.9300; pregnant plan 0.9203 -> 0.9210 (1:500 0.7460 -> 0.7635); sleeping front 0.9403 -> 0.9445. Not improved: sleeping plan 1:50..200 about -0.0001..-0.0006 (rectangle vs capsule), pregnant elevation 1:500 -0.0007, tour-guide plan 1:500 -0.0009.
- Honest regressions at rest: tour-guide plan -0.0085 (50/100/200), front -0.0034, elevation -0.0016 (50); pregnant front -0.0016; sleeping plan -0.0006. Cause: the 2D evaluator cannot draw elliptic plan cross-sections, so ellipses (shoulders, head) become rectangle or circle.
- Renders (read and checked): `T/🗑️generated/lateral/img/*.svg.png` (before/after, default pose and extremes: sleeping curled, pregnant cradling-belly, tour-guide wave-swing-b, turning-to-group, holding-flag, head close-up). Nothing detaches or mirrors; the cap and head plates tilt with a turned head (expected for plates); plan shoulders are now rectangles; plan shoulder seams and hair parting are thin bars.
- `volume-check.ts` still reports bbox FAIL for pregnant-person and tour-guide (x -51 %, -54 %); the volume is untouched and this predates the task.

## Tests (direct vitest, no `repo` commands)
- Targeted library (schema-conformance, asset-validation, three-views, plan/front-projection, multiview-sheet/coverage, model-catalog, course-coverage, catalog-physics, volume-schema, model-exports, representation*, print-suitability, thermal-multiview): 1227 passed, 0 failed (16 files).
- React `catalog-loading`: 8/8 passed (0 invalid assets).
- Full staffage library: 2500 passed, 1 failed, 0 skipped, 101 files; the failure is `dwg-reopen` (`staffage_dwg.wasm` not built, pre-existing).
- Full React: 98 passed, 0 failed, 0 skipped, 24 files (the count differs from the earlier doc, which counted 211+; the same 24 files ran).
- Python consumer: not run (no Python 3.12/manim, as before); the manifest rows do not mirror `dimensions` changes (none made).

## Unresolved
- Plan ellipses (shoulders, head, face) cannot be elliptic; honouring `lateralScale` in `solid3` would fix the small rest-IoU regressions (engine change, not done).
- Open detail lines became ribbons or thin bars; shoulder seams and hair parting in plan are lateral bars, not diagonals.
- `repo` MCP not connected: ticket not closed with `ticket_close`; `T/🗑️generated/lateral/` still to delete; helper scripts kept: `lateral-*.ts`, `lateral-apply.py`, `lateral-run.sh`, `lateral-test.sh`, `lateral-png.sh`.
