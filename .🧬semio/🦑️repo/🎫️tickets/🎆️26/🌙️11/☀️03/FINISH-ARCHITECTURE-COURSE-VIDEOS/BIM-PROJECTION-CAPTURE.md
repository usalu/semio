# Actual BIM Projection Failure Capture

At 2026-11-07T15:47:37.102Z, robust capture session 62666 retained the actual production input for BEAM-PRIMARY-F-7-24: 177 facets, projection matrix and primitive metadata. The original Open exposed material boundary was reproduced, so process exit 1 is expected for this diagnostic capture. This is a successful input capture, not a repaired kernel or completed lesson.

Reduction session 81345 uses normal 2 GiB admission. The 8 GiB capture has ended. Production surfaces and the 19 frozen model/cache files are unchanged at this checkpoint. Current native productions remain Trees 19642 and Tower 13162.

Earlier capture 38292 reproduced the error but lost its facet export when metadata encoding encountered a frozenset. Replacement 61729 was stopped early. The current writer self-test checks exact Fraction/float/tuple/matrix round-trip and deliberately failing metadata; each actual projected input is checkpointed atomically before the production call. Authored capture/reduction inputs remain in the ticket root; logs and captured data remain in the generated folder until all consumers finish.

## Reduced Case And Provenance

The 177-facet input reduced to a 27-point ring plus one quadrilateral. The received float ring self-intersects within roughly 1e-14 of a vertex and leads to four unmatched directed endpoints. A candidate winding classifier avoided the exception but left invalid output; it was rejected and reverted.

An isolated replay of the existing public solid_geometry for BEAM-PRIMARY-F-7-24 took approximately 20 seconds. Its matching exact pre-lift Fraction rings had zero exact nonadjacent contacts or proper crossings. The corresponding float-lifted/Prim rings self-intersected. Replacing the first captured ring with its exact pre-lift coordinates made the existing projected kernel succeed with one 38-vertex region. This establishes precision loss in the visual pipeline, not a demonstrated frozen-model defect.

Root authorized schema/actual-case regressions and exact coordinate preservation through material_skin, primitive polygon storage and affected view consumers, with conversion at the screen/Cairo boundary. The 19 model/cache files remain frozen. This is a proven cause and authorized repair scope; no completed repair or production acceptance is claimed here.

## Exact Visual Pipeline Freeze

The final exact-carrying surfaces, primitive storage and axon/view path pass 15 surface/native-finish/car checks and 14 bounded region/fragment checks. The complete isolated 177-facet beam passed production painter and Cairo in 40.766 seconds after exact centering was added. The earlier broad region run entered an expensive unscoped ramp test and was canceled incomplete; it is not a pass. The later unfiltered canonical suite still owns that gate.

All 19 model/cache hashes match freeze-v2. Frozen visual SHA-256: surfaces 9112659ed56c6da150440c30db32d22eb6c8c4b6af9cb68398e0794494160a62; primitives c84d0d324dbc0d8ac7599b2623fa6dc1c456eb3454f77cc4ffdc539702a9eddb; axon 4e19251dde63c6cc6be00d3c8b90c129b054595f60057ce7fd2078f108fb019f.

Production restarts the first three BIM lessons (114–116) serially at 8 GiB with the validated cache. Other BIM lessons remain deferred for headroom; no full-building or lesson acceptance is claimed by the isolated replay.
