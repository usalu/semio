# Original Component Cursor Read-Only Audit

Scope: original World component edge cursor, overlapping-wire law, shared Scene closest-pair math. Read applicable root, products, os, infinite, and ui AGENTS. No production edits, git mutations, builds, or runtime tests performed.

## Result

No additional production blocker found in this bounded review. Near/far segment clipping is intentionally left to the root agent's separate work.

The current perspective fixture genuinely uses the camera position as ray origin, not the near plane: `projection_spec_ray_from_screen` returns `camera.position` and a normalized direction (Scene math lines 399–411). The fixture camera position `[0,0,0]`, target `[0,0,-1]`, and central pointer produce origin zero and direction negative Z. Thus the authored component assertion `best.primary == 1` matches the near segment endpoint at Z=-1. Parallel projection instead originates on the near plane; its depth is relative to that plane, so a future parallel law must not reuse the perspective numeric depth assertion blindly.

`ray_segment_closest` evaluates both segment endpoints, the ray-origin boundary, and the feasible interior solution. Its returned `segment_point` remains on the authored edge. Component ranking projects that point onto the normalized cursor direction; unconstrained forward solutions agree with the ray parameter, while the boundary case is explicitly rejected when projected depth is negative. No fabricated midpoint or vertex substitutes the edge geometry.

Target construction matches the strengthened fixture: the object IDs contain `@` and `#`, and the unbound-source target becomes `near@wire#0.edge.0`. Source-bound targets append `~handle~label~revision`; this fixture intentionally does not exercise that branch. Cursor completion waits for registry terminality and revision/generation checks before resolving the winning token. Publication is driven through the real plan publisher, and the law drains InputState closure and component cursor closure.

## Actionable Authored-Test Gap

Low priority: `world_native_overlapping_wires_pick_original_closest_depth` asserts the serialized target for both select and hover, but does not assert the action verb or merge semantics. A regression publishing a select action for hover can satisfy all its current target assertions. Add the expected action identity for each purpose and select merge value, using the existing action schema conventions. This is a coverage gap, not an observed runtime failure.

The law also does not assert cursor origin/direction explicitly; the current origin/depth conclusion follows source inspection and its camera fixture. No test-pass or runtime-working claim is made here.
