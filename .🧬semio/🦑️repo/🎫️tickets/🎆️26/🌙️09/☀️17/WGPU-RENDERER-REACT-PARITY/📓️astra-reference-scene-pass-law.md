# Actual Reference Scene Pass

The earlier shared reference-visual law computed an equivalent matrix independently of `render_world_3d`. It could therefore miss a defect in the final textured-instance path. The read-only reference audit correctly left the observed placement difference unassigned.

The new native law `reference_visual_geometry_reaches_the_actual_textured_scene_pass` publishes the shared fixture's actual reference JSON through the world bridge, provides the fixture's decoded natural raster descriptor, invokes the real `render_world_3d`, and inspects its emitted textured draw. It checks texture identity, one emitted plane, and all four Three-derived world corners directly from that instance's model.

This law is registered in the existing World unit module and has not yet been compiled or run. No reference transform production change was made. The matching Three fixture and oracle pre-exist this extension. Runtime telemetry and fresh paired screenshots remain required to attribute the checkpoint placement delta.
# Executed Native Receipt

The actual reference scene-pass law passed in the focused World run on 2026-09-21. Nextest 3f0fe47d-d96d-4bda-b18d-a50b97bd3c7a executed two selected tests in 0.439 seconds: the reference geometry law passed, while the intentionally red grid producer law failed because it emitted 504 finite line vertices. There were 479 tests outside the selected filter.

This verifies texture identity and all four fixture corners in the actual emitted textured scene instance. It does not establish browser placement/pixel parity; fresh telemetry and paired screenshots remain necessary.
