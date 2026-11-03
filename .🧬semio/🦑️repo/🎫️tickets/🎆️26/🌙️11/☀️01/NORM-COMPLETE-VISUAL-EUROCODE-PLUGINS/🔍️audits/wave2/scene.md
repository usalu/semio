# Wave 2 scene agent report (nothing compiled; all code written by reading)

## 1. Placements and revisions (all ten editors and viewers)
- Kit `NORM/🎬️structural-scene/🧰️projection-kit/📍️placement`: `placements_of(&doc)` reads `placements[id].{x,y,z,yaw}`; `content_stamp`/`revision_of` replace the zero stamps; `PlacementCommit::of(current, moves)` = one insert/set batch (one undo step, `inverse(before)` for undo, release on the present pose = empty). Tests in `🧪️tests/🔬️unit` (commit, undo, stamps, flat-shape law).
- 20 viewport files rewired (`scene_of`, `handles_of`, viewer `render_with`, en1992 `handles_of`).
- Ruling applied: ONE flat shape `placements[id=E].{x,y,z,yaw}`; en1997 handles fixed (`.origin.x` to `.x`). en1992/en1996 keep per-entity `placement_*`/`plan_*` fields.
- OPEN: only en1999 has a `placements` collection in its snapshot; the other nine need it (schema/mutations/io, owners; requests.md). No `live_scene!` edits (coordinated in requests.md).

## 2. Deformation
- EN 1991: new `🌀️deflection` (mechanics kernel `member_modes`): wind column (H/500 at characteristic net wind load) and snow roof beam (L/250), `Motion::Ramp` = animated load application; closed-form tests. Stiffness is a stated assumption (snapshot holds actions only).
- EN 1994: 3D deformed beam already existed; added slip distribution along studs (2D diagram, push-out law). 3D slip arrows still OPEN.
- EN 1995: 3D deformed member and floor vibration mode already existed in `🪵️members` (audit was stale). No change.
- EN 1996: new `🌀️lateral`: 1 m wall strip deflection (kernel), mid-height eccentricity, cracked zones (`3e - t/2`), flanges (from `ShearFlange`), bearing stress blocks. Second-order effect of N not included.

## 3. Diagrams (each with table twin and bound_check)
EN 1990 SLS limit lines (`sls-limits:<member>`; gamma*psi bars still OPEN); EN 1991 psi chart (`psi-factors`; table duplicated, request to move into compliance); EN 1992 `shear-truss`, `shear-truss-sketch`, `strut-and-tie` (indicative, deep members) plus 3D shear planes at d (layer `shear`); crack pattern OPEN; EN 1994 `slip-distribution`; EN 1995 `n-ef`, splitting plane and row shear block (3D); EN 1996 see 2; EN 1998 `pushover`, `q-factor`, `force-profile` (new `🌋️capacity`); EN 1993 haunches, gussets, effective width: NOT DONE (not started; needs snapshot fields for haunches, owner request advisable).
Unverified values: psi_1/psi_2 constants (EN 1990 A1.1, EN values), strut width z/4 in STM, bilinear pushover plateau from alpha_u/alpha_1.

## 4. Golden fixtures
All ten now have `🎬️scene/🧪️tests/🧫️fixtures/🏛️scene/🔣️.json` (empty, to be blessed); en1992 test moved from `🏛️office-frame` to `🏛️scene` and `SEMIO_BLESS` to `SEMIO_UPDATE_GOLDEN`; en1997 path fixed; en1998 golden test added; en1994 also bridge fixture. Each `build-enXXXX/latest.md` has "ready for test: bless scene golden" and "ready for rebuild: scene ...". Goldens will also change because of new entities.

## 5. Read-only marker
`En19xxViewer::READ_ONLY = true` in all ten viewers with a test.

## Cargo / cross-owner
mechanics dependency added to en1991 and en1996 workspaces/scene crates; en1992 scene gets `checks` as a normal dependency; needs `synchronize` (requests.md). Existing en1991/en1995/en1992 tests were adjusted for new diagram ids.
