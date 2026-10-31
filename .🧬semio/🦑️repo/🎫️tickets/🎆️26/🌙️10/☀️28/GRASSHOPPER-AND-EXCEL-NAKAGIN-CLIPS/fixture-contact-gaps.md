# Reference Fixture Contact Gaps

Pre-support-fix audit of unmirrored default Backslash reference interior. All families retain the same geometric gaps under hand/usable mapping. Bounds are actual generated mesh AABBs; a disjoint AABB proves no mesh contact, while overlapping AABBs alone do not prove support. Root owns fixes.

|Actual part key|Minimum XYZ m|Maximum XYZ m|
|---|---|---|
|tub-bowl|[-1.08, 0.27, 0.29]|[-0.14, 0.88, 0.8]|
|wetroom-floor|[-1.12, 0.2, 0.2]|[0.08, 1.85, 0.245]|
|washbasin|[-0.42, 0.81, 0.79]|[-0.06, 1.17, 0.98]|
|basin-trap|[-0.249, 0.23, 0.586]|[-0.231, 1.109, 0.79]|
|tub-waste|[-0.619, 0.23, 0.246]|[-0.601, 0.584, 0.29]|
|wetroom-back|[-1.12, 0.2, 0.2]|[0.08, 0.225, 2.12]|
|toilet-flush|[-0.83, 1.717, 0.852]|[-0.812, 1.753, 0.888]|
|toilet-cistern|[-1.06, 1.68, 0.38]|[-0.84, 1.79, 0.94]|
|shower-riser|[-0.959, 0.236, 0.87]|[-0.941, 0.326566, 1.956155]|
|shower-hose|[-0.852, 0.24318, 0.896217]|[-0.708, 0.287, 1.553783]|
|tv-screen|[0.715, 1.825, 1.375]|[0.733, 2.295, 1.785]|
|tv-bezel|[0.736, 1.78, 1.3]|[0.78, 2.44, 1.86]|
|audio-console|[0.733, 1.2, 1.08]|[0.778, 1.68, 1.32]|
|media-wall|[0.78, 0.25, 0.2]|[1.11, 2.75, 2.23]|
|speaker-cone-0|[0.739, 0.915, 1.525]|[0.764, 1.125, 1.735]|
|speaker-grille-0-0|[0.728, 1.018, 1.525]|[0.734, 1.022, 1.735]|
|media-shelf-0|[0.53, 0.25, 1.94]|[0.78, 2.75, 1.965]|
|shelf-bottle-rail-0|[0.554, 0.3, 1.984]|[0.566, 2.7, 1.996]|

## Legitimate Contact Anchors

- tub-bowl: an explicitly inferred integral wetroom base/feet between its bottom and wetroom-floor; retain the tub cavity. Its existing drain/waste should also connect to the rear/floor outlet, not terminate a few millimetres short.
- washbasin/trap: actual mount/bracket or molded ledge to wetroom-back, and a continuous outlet crossing the wall face. A pipe is a geometric connection, not a structural-capacity certification.
- tub/basin taps and shower: wall roses/stems/clips into the existing wetroom enclosure or supported fixture deck; do not fill the bowl voids. Mount handset/hose in a real holder or connect the hose coupling.
- toilet-flush: sleeve/stem entering the cistern face; preserve the visible button projection.
- tv-screen: a perimeter seat/gasket bridging the3mm outer-ring-to-bezel gap; keep the curved CRT front.
- audio-console: rear standoffs or perimeter mounting frame into media-wall, preserving control profiles.
- speakers: annular rim/housing or brackets join the cone outer perimeter to media-wall and hold the separated grille wires at their endpoints. Preserve the intentional acoustic air space and grille gaps; no solid slab filling the void.
- shelf-bottle rails: short endpoint posts into actual media-shelf tops. Preserve rail height and shelf useful area.

## Layout And Scaling Constraints

For J/L/S, media cabinet/shelf/audio front edges move toY1.2 and speaker0 cones/grilles move+.4 inY. New mounts must follow the same transformation. Construct them after layout adjustment from actual part bounds, or include every new key in the exact layout change. Short posts must not be deformed by a min-vertex-only shelf crop; new speaker housings must not remain at the old cone location. Apply existing hand reflection and usable-bound scaling to mounts exactly as to their fixtures.

No source-derived exact historical fastener profile is claimed. These are explicit inferred physical attachments needed to make the rendered reconstruction continuous, while retaining material/fitting/program identities and areas.

The broader independent check found no out-of-usable vertices across18 reference control/family cases, but disconnected media/sanitary clusters in each. The all140 physical doorway scalar audit is separately saved: maximum port delta1.11e−16m, wall-plane offset4.44e−16m, normal dot1.0.
