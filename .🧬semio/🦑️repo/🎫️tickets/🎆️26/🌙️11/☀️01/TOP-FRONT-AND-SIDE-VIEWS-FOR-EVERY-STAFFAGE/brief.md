# Brief: Top, Front and Side Views for Every Staffage

Repo `C:\git\semio`. Follow `C:\git\semio\AGENTS.md`. In particular: no modifying git commands, no worktrees, run tools through `repo`, prefix temporary logs with `[DEBUG] `, and write natural English and German text.

- **S** = `🎓️teaching/🏛️architecture/🧍️staffage` (the archive)
- **T** = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE` (this ticket)
- **Prior rules:** read the binding rules in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/authoring-v2.md`, sections "Files and ownership" and "Representations".

## Goal

Every catalog asset gets three handcrafted views in every **authored** representation (scales 1:50, 1:100, 1:200 and 1:500):

- `geometry.elevation`: the side view
- `geometry.front`: the front view
- `geometry.plan`: the top view

Each view must work for every pose and every clip. Today 497 of 531 authored representations are elevation only. Six assets are already complete: `solar-thermal-collector` and `split-ac-indoor-unit`; use them as references. Representations with status `unavailable` stay unavailable.

## Coordinate contract

All values are model metres, authored in the **rig rest pose**, bound to the same bones as the elevation.

| View | Paper x | Paper y | How it follows poses and clips |
|---|---|---|---|
| elevation | forward (world x) | up | full bone transform (unchanged) |
| front | lateral, the asset's **left is +x**, centred on its sagittal/symmetry plane (x = 0) | up, ground y = 0 | by the bone's y travel from rest; on `rect`/`poly`, `followTip: true` stretches linearly between the bone's origin and tip travel |
| plan | forward, the same x as the elevation | lateral, the asset's **left is +y**, the same lateral coordinate as front x | by the bone's x travel; a `capsule` with `followTip: true` stretches from bone origin to bone tip |

This is the mechanism in `S/🔨️modules/🧮️evaluation/🟦️.ts` `placeShape`/`frontStretch`. Study `S/🧫️fixtures/🧍️plan-projection` and `S/🧫️fixtures/🧍️front-projection`.

**Limbs and bones that swing in poses or clips:** legs, arms, a crane jib, an excavator boom, door leaves, blind slats, wheels, a swing.
- In plan, use `capsule` with `followTip: true` along the bone axis, so strides, seated thighs and reaching arms stretch correctly.
- In front, use `followTip` rects or polys where height changes along the bone, so bending knees and a raised boom shorten or rise.
- Choose the bone binding of every front and plan shape so that **every pose and clip reads correctly**. Render several poses and clip instants and look at them.

**Orthographic consistency, enforced by the new test.** At the asset's `presentation.defaultPose`, for every authored scale, with tolerance max(0.02 m, 3 % of the largest dimension):
- the elevation x-range equals the plan x-range
- the elevation y-range equals the front y-range
- the front x-range equals the plan y-range
- these match `dimensions`: `widthM` is the elevation x extent, `depthM` the lateral extent, `heightM` the height

At every pose and every clip sample, all three views must be non-empty with finite bounds. Ground shapes (`role: "ground"`) are excluded from extents.

## Authoring rules (handcrafted, per scale)

- **Draw each scale.** Every view at every scale is designed for its paper size: 1:500 composed silhouette, 1:200 main masses, 1:100 articulated, 1:50 full detail.
  - Never copy one scale's view into another scale; the `representation-copied` check now covers all three views.
  - Never derive front or plan geometry with a generic algorithm, such as extruding, bounding boxes or convex hulls of the elevation.
- **Scripts.** Scratch scripts may compute coordinates (mirroring left and right, offsets, unit conversions). You decide and review every shape. Put scripts in `T/🧰️<worker-id>/` and previews in `T/🗑️generated/<worker-id>/`.
- **Thermal binding.** Every filled surface shape carries `region`/`regions` and `material` like its elevation counterpart, so the thermal, structural and transport views work in every viewpoint.
- **Occlusion.** Use `z` order for overlap.
  - Front view: the near (left or right) limb overlaps correctly; the far side is hidden behind the torso where it should be.
  - Plan view: draw what is seen from above. A head or hat covers the shoulders, a crown covers the trunk, a roof covers the cabin, a seat covers the legs below.
- **Ids.** Shape ids are unique within each view; reusing the elevation's ids for the same part is fine and encouraged.
- **Presentation.** Remove `presentation.viewpoints` from your asset files. The list is deleted from the schema, because all three views are now mandatory.
- **Pens and hatches.** Pens, hatches and `decisions` are shared per representation. Extend the `decisions` texts (en and de) to say how the front and plan views were drawn at that scale.
- **Print profile.** Each view must pass the default print profile `office-laser` (min stroke, gap, feature).
- **Editing files.** Edit JSON with the Write or Edit tools, or with Python `json.load`/`json.dump(..., ensure_ascii=False, indent=2)`, matching the file's current formatting. Never write JSON through Bash heredocs. Asset files can be large (up to 0.5 MB): read and write them through scripts, not by hand-copying.
- **Ownership.** Edit ONLY the asset folders assigned to you. Others work concurrently: framework worker FW in `S/🔨️modules`, `S/🧬️schema`, `S/🎯️targets` and `S/🌐️site`; test worker TS in `S/🧪️tests` and `S/🧫️fixtures`.

## Tools

The check CLI validates, evaluates and checks suitability for all views:
```
repo tools exec bun -- 🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts --asset <id>
```

The preview CLI renders previews:
```
repo tools exec bun -- 🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🖨️preview/🟦️.ts --asset <id> --scale 50 [--pose id | --clip id --time s] [--viewpoint elevation|front|plan] [--views] --out <dir>
```

FW is adding `--viewpoint` and `--views` (a sheet with side, front and top for one pose or clip instant). Until `T/fw-status.md` says they are ready, render with a ticket-local bun script that calls `evaluate` and `depict` with the `viewpoint` option. Look at the PNGs with the Read tool, at print size and zoomed in, for the default pose, every named pose and 2–3 clip instants per clip, and iterate until each view reads well.

Run the test suite once TS reports it in `T/ts-status.md`:
```
repo run @teaching/staffage:test --lang=en
```
If `repo` is broken, follow the fallback described in that status file.

## Report

Write `T/report-<worker-id>.md`. For each asset, give:
- per scale, how front and plan were drawn and how the bones are bound
- the poses and clips checked, and which previews were inspected
- the check output
- any remaining doubts

Your final message: a short summary of the same.

## Coordinator Rule: Guard Failures on Spread or Twist
If the three-views guard fails for a pose with `abductionDeg` or `twistDeg`, fix the front and plan shapes; never remove the motion. Author shapes centred on their bone in the rest frame. Set `Bone.z` for bones beside the symmetry plane. Full 3D placement then makes the three views agree. Motion values must be physically right for what the pose name and description say.

## Coordinator Rule: Killing Processes
Never use `taskkill /IM`, `pkill bun` or any other kill by name or pattern. Kill only a PID that you started yourself. QA10 killed every bun process on 2026-11-02, so any run that died must be rerun.

## Coordinator Rule: Atomic Asset Writes
Never write an asset JSON in place. Use `write_json_atomic(path, value)` from `T/🧰️FW/atomic_json.py`, or write to a temporary file in the same folder followed by `os.replace`. Read only after your script has finished.

## Same-Region Merge (Landed)
Unfilled strokes (rings, spokes, ribs) are now cut only by a filled shape with a higher `z`. Give a detail a higher `z` than its body, and drop any workaround that strips regions from unfilled shapes. See "Same-region merge" in fw-status.md.
