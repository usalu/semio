# 📝️ haiku-spatial-audit: Projection Conventions and Spot Checks (relayed by the coordinator)

This is a static, read-only audit of the snapshot of 2026-11-02. Line numbers may have drifted since.

## Conventions as implemented (confirms D6)

| View | Paper mapping | Viewer side |
|---|---|---|
| Elevation | paper (x, −y) | at −z, the asset's right (`🧊️depth:34`) |
| Front | paper x = +z, y = −y | at +x |
| Plan | paper x = x, paper y = −z (so +z is up the page) | at +y |

How each shape type is projected:
- **Extruded** shapes sweep z0..z1 in the bone frame.
- **Revolved** shapes take `axisX` as an absolute rest-world coordinate, not relative to the bone (`🧮️evaluation:430`). In front and elevation the mirror is optional; the plan always draws the full body.
- **Hidden lines** are not drawn dashed; occluded outline stretches are removed. Section markers are drawn in the elevation only.

## Discrepancies

| # | Discrepancy | Owner |
|---|---|---|
| D2 | README says "World x right"; it should say forward. | docs |
| D3 | `🧮️evaluation:7-8` and README:61 say front and plan shapes follow only y or x travel. In fact they are carried in full 3D. | rigs, docs |
| D4 | The schema and README say a front `followTip` rect or poly stretches vertically. The code only stretches bones with no front extent, laterally (`🧮️evaluation:497`). | rigs, contracts |
| D5 | A plan capsule with `followTip` also moves along lateral z, but the docs mention x only (`🧮️evaluation:495`). | rigs, contracts |
| D6 | The front overall dimension measures the elevation bounds (`🎨️depiction:44`), and fit-to-frame hides the error. Contracts reports this is now fixed. | contracts (verify) |
| D7 | The validator checks only elevation height and width, never `depthM` or the front and plan extents. The backlog now adds extent codes. | contracts (verify) |
| D8 | Ground-role shapes are excluded from the elevation bounds but included in the plan dimension and in the front `groundSpan`. | contracts |
| D9 | A `solid` in the elevation is accepted silently, while the README says front and plan only. | contracts |
| D10 | Respiration is added only to bones that have a keyed pose offset (`🧮️evaluation:659-661`), so breathing may vanish on the others. | rigs, thermal |
| D11 | Lateral z is dropped from bones, anchors, loads, masses and kinematics. Partly fixed by D2 of the work board. | rigs, structural |
| D12 | Front and plan shapes without an elevation twin are pinned at the bone-origin depth and not validated. | contracts, rigs |
| D13 | Depth uses only the bone origin and tip, not the shape's extent; a constraint cycle falls back to an arbitrary order; `rig:124` drops `bone.z` in the path without spread. | rigs |

## Asset spot checks

| Asset | Finding | Owner |
|---|---|---|
| cyclist | **Defect.** The arms have no lateral z or abduction, so the hands sit at z = 0, while the grips are at ±0.285 (front) and ±0.3075 (plan). There is only `grip-r` and no `grip-l`. The pedal spindles are at z = 0 while the thighs are at ±0.095. | humans |
| person-with-umbrella | Consistent: the shaft is at the midline. The canopy `axisX` 0 works only because the bone origin rests at x = 0, which is fragile. | humans |
| worker-carrying-box | Handedness is correct. Risk: the box is two independent revolute halves, so they can split when the hand angles diverge. | humans |
| horse | The near legs are at −z, which is correct. No pose spreads the legs. | animals |
| tower-crane | The extruded jib and the 3D-carried plan chords make slewing correct. | machines |
| market-parasol | `rib-right` and `rib-left` are sagittal ribs, so "right" does not mean −z; consider renaming them. | objects |
| sliding-door | Verified: W 3.1, D 0.1, H 2.15. | buildings |
| elderly-with-cane | **Defect.** The arms and hands have no lateral z, so the cane in the right hand sits at the midline. | humans |
| person-on-crutches | **Defect.** The arms have no lateral z, so both crutches sit at lateral 0 and overlap in plan. | humans |
| runner-with-blade-prosthesis | **Defect.** The legs have no lateral z, so the prosthesis is "right" only in name. | humans |
| seated-guitarist | **Defect.** `fret-hand` and `strum-hand` are anchors on the guitar at the midline, while the hands are at ±0.218. | humans |
| standing-phone-user | **Defect.** The phone is in hand-r, but the `grip-l` thumb anchor is on hand-l. | humans |
| parent-carrying-toddler | **Defect.** There is no z anywhere, so the toddler sits on the midline instead of a hip. | humans |
| person-with-white-cane | Right-hand cane, consistent. | — |
| labrador-dog, herring-gull | The near side is −z, consistent. The gull's wing abductions are symmetric and signed correctly. | — |
| weeping-willow | Sway comes only from abduction, and branch symmetry is unchecked. | vegetation |
| forklift | **Defect.** The tine rect profiles are transposed: they run 0.98 m down instead of 1.07 m forward. | machines |
| excavator | There is no slew or twist; it is planar only. | machines |
| spiral-stair | The tread azimuth exists only in the shapes, not in the rig. | buildings |
| tied-canopy | One bone; the supports were not read. | buildings |
| city-car | Left is +z, which is correct. Not checked: whether the elevation's single mirror is the right-hand one. | machines |
