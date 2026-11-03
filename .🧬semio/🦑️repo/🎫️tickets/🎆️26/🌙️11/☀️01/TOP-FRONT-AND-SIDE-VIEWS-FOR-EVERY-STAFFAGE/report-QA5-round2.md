# QA5 round 2 (humans: sleeping-person, standing-adult, standing-phone-user, tour-guide, walking-commuter, wheelchair-user, worker-carrying-box)

Scripts: `T/🧰️QA5/` (`all.ts` driver, `setz.py`, `abduction.py`, `elev_solids.py`, `limb_solids.py`, `shape-probe.ts`, `montage.py`, `raster.py`); generator fixes in `T/🧰️H4/`. All JSON writes go through `T/🧰️FW/atomic_json.py`.

## Done
- `Bone.z` on arm and thigh bones (real lateral offsets) for all seven assets, so spread and twist pivot physically.
- Motions restored: adult `pointing` arm-r -85; tour-guide waving, wave-swing-a/b arm-r -50 (forearm +-8), gesturing-to-group -35, turning-to-group torso +20 / head +25 twist; sleeper side pelvis 35 / torso 25 and curled pelvis 50 / torso 35 twist (legs follow the roll).
- Flag and finial, worker box and tape, phone-user ponytail, adult 1:500 arms and forearms, worker 1:500 legs: `solid` extruded in the bone frame (profile = the scale's own elevation shape).
- Elevation shapes wide on spread or twisted bones carry `solid` (new `elevation-lateral` validator rule).
- Sleeper: front thigh and shin domes `followTip`; arm ellipses centred on their bones.
- Tour-guide pointer checked: plan capsule follows the hand and spreads (waving: 0.6 m lateral, physically right for a 0.55 m pointer).

## Open
- sleeping-person: guard `spread-*` findings at curled / turn-over (front x vs plan y 0.07 m; front y vs elevation y at 1:500 side) and `eye` 0.197 mm at 1:50; vitest case fails for it.
- standing-adult 1:500 `sil-hand` and standing-phone-user 1:500 `phone`: 0.197 mm (the value repeats for unrelated tiny shapes, so it looks like the measure's floor, not a width I can widen).

## Round 3 (after the single 3D path landed)
- Sleeper rebuilt: torso, duvet (chest, hips, thighs, shins), neck, head, hair and both arms are extruded solids in elevation, front and plan (`T/🧰️QA5/sleeper_3d.py`), rolled by the pelvis/torso twists. Guard clean.
- Print fixes: ear and eye enlarged, adult 1:500 hand, tour-guide 1:500 pole/pointer/forearms (extruded limb solids, wider pole), phone solid in front/plan and merged into the hand at 1:500.
- Check: ok on all seven assets. Three-views vitest: 14 of 14. 50 montages read (sleeper 1:50/1:100 rendered with `--views`).
- Open: preview CLI writes an "explanation" sheet instead of the views sheet for sleeping-person at 1:50/1:100 when called with `--views` in the first runs (later runs fine); `coverage.py --consistency` not re-run.
