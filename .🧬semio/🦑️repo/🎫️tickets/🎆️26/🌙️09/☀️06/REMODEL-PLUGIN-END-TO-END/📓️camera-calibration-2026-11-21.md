# Per-view camera calibration

The reconstruction run resolves each accepted frame's intrinsic calibration from its stream's camera binding. The solver receives the calibrated focal lengths, principal point, pixel skew and selected lens model on the stored frame's pixel grid. Frames without usable bound calibration use a centered pinhole estimate from their decoded dimensions. Video source dimensions describe the original container and can differ from its stored, already-downscaled frames.

Geometric pair verification, initial-pair estimation, registration, retriangulation, bundle adjustment, and reprojection all select the calibration of the relevant image. Dense stereo projects between independently calibrated reference and source images. The corresponding view calibration follows each depth map through fusion, TSDF integration, texturing, quality reporting, and reprojection covariance.

The engine orders its accepted frames by timestamp after applying stream sync offsets. Ingest verdicts and published trajectory poses use the source stream ordinal plus that stream's frame index, so two streams can both supply frame zero. The checkpoint digest includes the stream camera binding and sync offset, with length-delimited variable fields.

## Verification

The asymmetric camera CSV contains three views with separate dimensions, uneven focal lengths, off-center principal points, skew, Brown–Conrady distortion, fisheye distortion, and an undistorted pinhole model. The companion [LAPACK oracle script](📷️calibration-oracle.py) constructs the pixel observations and uses DGELS to independently recover 28 three-dimensional points. The bounded Rust SfM test uses those observations through seed, PnP, bundle adjustment, pruning and snapshot preparation.

The session ingress test uses two streams whose calibration rows are in reverse order. Both streams supply frame zero. One video stores a 32×16 frame from a 640×320 source, and the other stores a 24×24 still. With a 16-pixel long-edge limit, they produce 16×8 and 16×16 reconstruction frames. The test checks frame identity, timestamp ordering, resized dimensions, calibration values and checkpoint invalidation. The engine test covers mixed-size dense sources, fusion and texture views, capture identifiers and terminal QC.

The native SfM package suite passed with 29 tests and one ignored diagnostic. That run included the new asymmetric fixture. The existing suite's imports were repaired after the geometry package split so tests can call public geometry functions and independently evaluate their results. Regenerating the checked-in CSV with LAPACK returned byte-identical results. The native reconstruction package passed both tests, including mixed-calibration depth sources, fusion, texture preparation and sparse/QC readback.

The initial focused native session attempt stopped while compiling its transitive stdio dependencies so the main WASM build could reclaim disk. The first WASM build compiled the affected geometry, SfM, geo, reconstruction, engine, session and editor packages. The wider build later received SIGKILL while compiling the unrelated stdio Semio artifact. The native session reruns below used the later warmed dependency graph.

## Remaining capture constraints

Calibration coordinates are expressed on the stored asset pixel grid. Imports that resize video should calibrate on those stored dimensions. Reconstruction resizes a decoded frame to the configured long-edge limit and transforms its intrinsics onto the resulting pixel grid. The bounded still decoder enforces a 262,144-pixel source admission envelope before this resize.

## Native Session Test Wiring

The warmed native session build reached test compilation and reported six errors. The unit tests now import the mutation constructors from their owning leaf modules and specify `ValueError` for the successful close-work result. The Hamming oracle reads the engine's existing descriptor and pair schedule fields at settlement, before the test driver closes the engine. The test repair leaves the production engine and resize fixture untouched. The native rerun is pending the active PNG build.

## Native Session Lifecycle

The rebuilt session tests passed resize-oracle and mixed-input calibration checks. The existing test binary reproduced two checker lifecycle failures: the one-fuel driver delivered no outcome, and cancellation observed no work before the cancellation request. The fixture caller was lending the one-unit job budget to return-origin setup and trying to pay host return-table birth and refresh from the four-item business grant. The test driver now admits the table explicitly, uses the framework's quoted host refresh turns, and prepares the return origin before applying the job's fuel and deadline. Receipt, acknowledgement, and retirement still use the same original caller. The revised lifecycle driver needs a native rebuild and rerun. Orbit fixture parsing is tracked separately.

The funded rerun exposed a production double charge. A reconstruction decision used the final unit of work and the same turn sometimes charged preview or checkpoint preparation. An exhausted turn now leaves buffered work for the next funded turn, which flushes the preview before a new decision. The run retains terminal completion until its outcome is admitted. Both session jobs report completion after their retirement state is physically empty. The native fixture marks locally acknowledged escrow rows for reclamation after receiving each outcome. The focused one-fuel test passed in 0.42 seconds, and cancellation with original-caller retirement passed in 0.17 seconds.

The first bounded native session run passed the resize oracle, mixed-calibration ingress, checkpoint encoding and replay, revalidation, and run schema. Its 160px short-orbit fixture reconstructed six registered cameras and 369 dense points. It produced no replacement mesh, and the provisional mesh/undo law and exact trace expectation failed. The short-orbit tests now retain the authored 320px dimensions. A separate resize fixture continues to exercise reduced output. The Hamming law now checks the complete 36-frame pair schedule at the first delivered preview after matching.

A standalone native test binary linked the revised session test source against the 28 dependency artifacts recorded by the warmed Cargo fingerprint. Both the 36-frame bitvec Hamming oracle and the authored 320px provisional publication and undo law passed. The latter applied each provisional mutation, resolved the replacement mesh from appended content, and restored the exact base snapshot through the recorded inverses.

The one-fuel diagnostic for the authored short orbit recorded 8,486 verdicts, 1,489 accepted matches, 6,718 rejected matches, and 434 sparse points. The run registered all eight cameras and published 512 dense points, six content operations and one commit. The checker and short-orbit rows in the shared JSON fixture agree with every recorded field. The standalone binary also passed one-fuel decision delivery, cancellation, resize, mixed calibration, checkpoint encoding, checkpoint replay, revalidation, and the run schema. The first standalone interactive timing run reached its 240-second wall limit before reporting its assertions.

The full-orbit one-fuel diagnostic reached matching after 8,953 decisions with a preview pending and only 4,186 available copy bytes from the test caller's cumulative 1 GB stock. The fixture runs the 36-view numerical law with the framework's normal interactive lane fuel and keeps the one-fuel contract for the checker and eight-view cases.

The funded 36-view run registered all 36 cameras, recorded 119,521 verdicts, and published 512 dense points, six content operations, and one commit. It recorded 15,057 accepted matches, 112,983 rejected matches, and 1,633 sparse points. Its preview ticks reported features, matching, poses, dense, and products. The native recorder completed all three cases in 458.19 seconds. The revised language-neutral fixture then passed its native assertion in 456.68 seconds, including its published quality reports and trace records.

The first two interactive timing runs expired before reporting their assertions. Instrumenting the eight-view run showed its virtual turns depleted the test caller's cumulative 1 GB copy budget after reaching dense stereo. The run had 3,403 copy bytes left and remained at dense view 1 and decision 3,412 from turn 200,000 through turn 2,500,000. The test caller now allows cumulative copies up to the platform's counter limit while using the existing per-turn grant and watchdog deadlines. Six focused tests passed with this accounting change. The funded timing law passed in 374.13 seconds.

All 12 active native reconstruction-session tests passed across the targeted runs. Two fixture-recording diagnostics remain explicitly ignored in ordinary runs. The Remodel and ticket diff passed the whitespace check.
