# BIM exposed material boundary

The BIM 115 TAP-HALL proxy exposed an open material boundary in the Savoye axon. The captured input contains 1,128 facets; reduction isolated 18 coplanar facets and then five exact floating-point facets. Four adjacent positive strips intersect an oppositely oriented rectangle. Tile seams had been cancelled before intersection noding, so an active edge crossing a removed seam could be classified as one uninterrupted span.

The surface kernel now nodes active edges at intersections with cancelled seams while emitting only active edges. The schema-first five-facet fixture reproduced the exception before the change; it and the six existing material-surface tests pass afterwards. The fixture independently compares area and boundary with Shapely. Full captured-input verification passed under a 2 GiB capacity grant: all 1,128 captured facets produced 432 exposed regions in 1.72 seconds.

Changed production and test files:

- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/🧩️surfaces/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/🧩️surfaces/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/🧩️surfaces/🧪️tests/🔣️tap.schema.json`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/🧩️surfaces/🧪️tests/🔣️tap.json`

Ticket inputs: `bim-boundary-capture.py`, `bim-boundary-reduce.py`, `bim-boundary-minimize.py`, `bim-boundary-verify.py`. Captured data and run output are retained under `🗑️generated` until closure. BIM model and cache sources were not changed.

The captured 18-facet coplanar plane completed in 0.027 seconds. All 19 frozen BIM cache source SHA-256 hashes still match. The surface module is registered through the BIM clip test aggregator.

## BIM 114 projected-material capture

A separate actual BIM 114 failure occurs in projected_material_regions, not the earlier material_skin TAP-HALL case. Ticket-root bim-projection-capture.py runs the actual villa axon with validated full-slab-v2 and captures the failing primitive and exact facet/matrix arguments under generated; bim-projection-reduce.py retains inputs for reduction. Session 38292 is active under an 8 GiB capacity grant. No diagnosis or fix is claimed until its output is captured.

First projection capture reproduced the original Open exposed material boundary, but its exception handler failed before writing facets because primitive metadata contained frozenset. The failed log is retained; it is not a successful input capture. The ticket capture script now retains only primitive tag/class and has been relaunched as session 61729 under the same validated cache and 8 GiB grant. No production or model-cache source changed.

Capture v2 was stopped incomplete before source changes to strengthen the writer. Ticket input bim_projection_io.py now atomically checkpoints exact facets/matrix before every projected call, independent of best-effort metadata. Ticket input bim-projection-writer-test.py passed exact Fraction/float/tuple/matrix round trips with the real codec and confirmed a frozenset metadata failure leaves the essential checkpoint intact. Reducer consumes that checkpoint. Corrected capture session 62666 is active with the same cache and normal 8 GiB reservation.

Capture v3 retained BEAM-PRIMARY-F-7-24, 177 visible facets and the projection matrix. Exact reduction found a two-facet failing case, now retained as a schema-first permanent regression. Shapely reports that one captured 27-point ring is self-intersecting in its native XY chart. A provisional exact winding-side classifier eliminated the exception but its converted float output remained GEOS-invalid; that candidate was reverted and is not accepted. Isolated provenance harness bim-projection-provenance.py is running on the existing beam producer to identify the first precision/topology change.

The isolated provenance replay matched both 27-point native pre-lift rings to the captured beam. Exact rational predicates found no nonadjacent contacts or crossings in either; the float-lifted native XY ring self-intersects. Replacing the captured first ring with its original exact pre-lift coordinates made the existing projected union succeed. The accepted repair keeps Fraction coordinates in material_skin, Prim polygon storage and projected region boundaries, carries exact height transforms, and centers before converting at the screen boundary. The Prim producer token now includes the face normalizer and Prim constructor. No tolerance or topology repair is used.

Both schema-first new regression cases were red before the fix. The final Python 3.12 render environment passed 15 tests across surface, native-finish and car-scale modules, including exact beam topology, independent Shapely area comparison, exact material intersection coordinates, height scaling and exact centering. Four bounded region tests plus the ten fragment tests passed (14 total). The isolated real BEAM-PRIMARY-F-7-24 replay reconstructed all 177 visible facets and passed the production painter and Cairo in 40.766 seconds. One earlier combined region run was stopped incomplete after it unexpectedly entered the expensive uncached ramp/slab calculation; it is not a pass. The canonical full suite remains required.

Final production SHA-256 at freeze: surfaces 9112659ed56c6da150440c30db32d22eb6c8c4b6af9cb68398e0794494160a62; axon primitives c84d0d324dbc0d8ac7599b2623fa6dc1c456eb3454f77cc4ffdc539702a9eddb; axon view 4e19251dde63c6cc6be00d3c8b90c129b054595f60057ce7fd2078f108fb019f. All 19 frozen BIM model/cache input hashes match freeze-v2.

Additional updated production files: `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/🧊️axon/🧩️primitives.py`, `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/🏛️savoye/🧊️axon/🐍️.py`; surfaces kernel and test module were already listed. Created permanent schema/fixture: `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/🧩️surfaces/🧪️tests/🔣️projection.schema.json`, `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/🧩️surfaces/🧪️tests/🔣️projection.json`. Created retained ticket inputs: `bim-projection-capture.py`, `bim_projection_io.py`, `bim-projection-writer-test.py`, `bim-projection-reduce.py`, `bim-projection-balance.py`, `bim-projection-provenance.py`, `bim-projection-beam-verify.py`. Generated logs, checkpoints and captured data remain under the ticket generated directory pending cleanup at closure.

Exact successful scoped test selections (all under `🎓️teaching.🏛️architecture.🏗️bim.🎬️clip`):

- `🎨️visuals.📐️drafting.🧩️surfaces.🧪️tests.🐍️.MaterialSkin`: `test_language_neutral_skin_matches_independent_geos`, `test_tap_hall_cancelled_seams_node_crossing_edges`, `test_actual_beam_projection_crossing_winding_matches_geos`, `test_material_skin_preserves_exact_intersection_coordinates`, `test_axon_centres_exact_coordinates_before_screen_conversion`, `test_progress_cancellation_and_owned_output`, `test_actual_source_gravel_production_boundary`, `test_language_neutral_visible_material_edges_match_geos`, `test_actual_painter_inks_only_visible_material_regions_and_reuses_native_boundaries`, `test_noncovering_annular_tube_uses_the_actual_production_skin_dispatch`.
- `🧪️tests.🧊️native-finish.🐍️.NativeFinish`: all four tests, and `🧪️tests.🚘️car-scale.🐍️.CarScale`: its one test; combined with the surface class, 15 passed.
- `🎨️visuals.📐️drafting.🧩️regions.🧪️tests.🐍️.OwnedUnion`: `test_neutral_exact_union_and_complete_boundary`, `test_progress_cancellation_cache_isolation_and_producer_identity`, `test_source_affine_tjunction_matches_integer_geos_before_publication`, `test_exact_closed_material_projection_matches_all_face_geos`; plus every test in `🎨️visuals.📐️drafting.✂️fragments.🧪️tests.🐍️.FragmentKernel`: 14 passed total.

Coordinator accepted the visual source freeze and restarted the serial BIM information/objects/representations production under 8 GiB using full-slab-v2. Source 114 attempt PID 30668 is active; the former projected-boundary failure point and resulting source receipt remain to be verified. This production start is not a pass.
