# Asset A5: plan (top-view) representations

Scope: `standing-adult`, `walking-commuter`, `seated-office-worker`, `city-car` now draw from above (`viewpoint: plan`). S = `🎓️teaching/🏛️architecture/🧍️staffage`.

## Findings on the existing plan support

- No asset had plan geometry in its authored scale representations. The plan sat only in symbols (40 assets: trees, vehicles, furniture, components), which the cast exporter cannot select (it picks the scale representation). Five assets declare `viewpoints: [elevation, plan]` (agave, bamboo-clump, hosta-clump, ivy-on-trellis, spiral-stair), but a plan cast of them renders an empty figure.
- The schema already allowed `representations[k].geometry.plan`. The evaluator places plan shapes in plan coordinates (x ahead, y sideways, metres) and moves them with their bone by the bone's x travel from the rest pose only.
- So the architecture supports per-pose plan geometry, but only rigid per-bone translation. A limb seen from above must stretch, which that cannot do.

## Changes in the archive

- Schema (`🧬️schema/🔣️.json`, twin `🟦️.ts`): capsule gets optional `followTip` (plan only). The `(x0, y0)` end follows the bone origin, the `(x1, y1)` end the bone tip. Arms, legs, trunk and a chair backrest stretch with the pose.
- Evaluation (`🔨️modules/🧮️evaluation/🟦️.ts`, `placeShape`): implements `followTip` through the same rest-relative x travel, and the module header documents it.
- Catalog check (`🔨️modules/🗂️catalog/🟦️.ts`): rule `follow-tip-plan-only`.
- Depiction (`🔨️modules/🎨️depiction/🟦️.ts`): a plan drawing no longer reaches the elevation bounds, so plan casts are sized to the plan, not to the figure's height.
- Cast exporter (`🔨️modules/📤️export/🎭️cast/🟦️.ts`): `heightM` of a plan cast is its plan depth.
- Four assets: `plan` geometry at 1:50, 1:100, 1:200, 1:500 with a different level of detail per scale, `presentation.viewpoints` and the manifest rows now list `plan`, and a plan sentence was appended to each scale's `decisions.silhouette` (en and de).
- Tests: `🧪️tests/🧍️plan-projection/{🟦️.ts,🥒️.feature}` and `🧫️fixtures/🧍️plan-projection/🔣️.json` (13 tests).

## What is drawn

- People (x ahead, y to the left, true size, arm span 0.5 m): shoulders (bound to the neck bone, so a lean moves them), waist and a trunk capsule pelvis to neck, head with face, ears, nose and parting, scarf or hood, upper arm, forearm and hand per side as stretching capsules, thighs and shanks, shoes (heel to ball and toe, laces). Region ids are bound (coat, hoodie, shirt, trousers, jeans, hair, face-skin, hand-skin, shoes, scarf, bag-leather) so the thermal view works in plan.
  - walking-commuter adds the shoulder bag on the left hip with its strap.
  - seated-office-worker draws arms over the desk to the laptop, thighs on the seat, the five-star chair base with castors, seat, armrests, backrest, desk top (1.4 m wide, authored estimate) and laptop with keys and lid. Shanks and shoes are under the desk and omitted. The backrest and the lid are rigid in plan (no bone follows their tip).
- city-car: body outline, bonnet, roof, tailgate, windscreen, rear glass, mirrors, headlamps, tail lamps, door gaps and bonnet creases. Wheels are hidden under the body. Root travel (lane poses) moves everything.

## Verification

- Catalog check: `✓` for standing-adult, seated-office-worker, walking-commuter, city-car, and the whole `humans` category (28/28).
- Staffage vitest: 49 files pass, 907 tests passed, 10 skipped. The one failing file (`📏️print-size`, missing `d3-format`) fails the same way without this change. `bun x tsc --noEmit -p tsconfig.json` is clean (apart from the same `d3-format` import).
- The `repo run @teaching/staffage:typecheck` MCP call spent 1h26m waiting for a dependency sync and failed with "Current immutable input capture work refused", unrelated to this work.
- Plan casts exported with the cast exporter (`viewpoint: plan`, `theme: dark`, `view: dark-line`, plus a thermal one each) and looked at in the browser pane: all four assets at 1:50, the three scale reductions, the walk-cycle frames, hatch-open and lane poses of the car, leaning-back of the worker. They agree with the independent gl-matrix kinematics in the test.
- Scripts and cast manifests: `🧰️a5/` (`plan-lib.ts`, `plan-human.ts`, `plan-main.ts` regenerate the geometry by text edit; `casts.json`, `casts-humans.json`).

## How to use in a cast manifest

```json
"excel-program-occupant-plan": { "asset": "standing-adult", "pose": "relaxed", "view": "dark-line", "viewpoint": "plan", "theme": "dark", "scale": 50 }
```

x ahead is to the right. `mirror: true` faces left. Rotating to another heading is a transform on the frame. The cast index `heightM` is the plan depth.

## Open points

- Poses do not rotate the pelvis or the shoulders about the vertical axis (the rig is sagittal), so a plan figure never turns. Walking, leaning, reaching and sitting track; turning the body is done by rotating the cast.
- The 40 assets with a plan symbol only (trees, bicycle, furniture, components) still render empty in plan casts, because the exporter picks scale representations.
