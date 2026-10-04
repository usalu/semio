# Painted Path Raster Integration — 2026-10-02

## Contract and Plan

Integrate Draw authored paths and appearance through a Rust/TypeScript raster job under geometry/raster. Inputs retain canonical segments, fill/fill rule, stroke, full affine, bounded output extent/origin and device tolerance. Outputs are straight RGBA images. Flattening, point conversion, stroke outlining, separate fill/stroke coverage and pixel painting are work granted; painted fill and stroke combine before scene opacity/blending. Native source conversion must process one segment per work; contour copies must process one point per work. No production PNG cutover until scene, text/image and export scheduling are handled.

Twenty manually authored neutral RGBA fixtures and a JSON input schema precede implementation. Pixel-center local paint sampling follows the prepared fill contract; exact shape coverage controls alpha. Independent SVG/native-canvas tests will establish the practical output limits, including gradients at boundaries. Images remain clipped to the requested extent; the scene consumer must choose cropped extents rather than allocate a full artboard for each layer.

## Verification Findings

Initial TS runs failed on incorrectly relative test imports; those were fixed. Behavioral red run 22923 then executed the stubs and failed all 23 added groups (284 old checks passed, PDF reference skipped because the initial environment name was wrong). Initial native red attempt 90788 stopped before tests on a stale generated graph reader using the removed OS-kernel JSON API. The generator already emits the current first-party pack-JSON call, but the Draw owner output catalog lacked its newly required policy and regeneration failed. Added the same explicit policy as the neighboring owner and will regenerate with the registered target; no generated reader hand-edit or compatibility wrapper is used.

Sharp at native one-pixel resolution approximates a tiny round cap eight alpha levels below its true circle area; its 128x averaged render matches the exact neutral area. Sharp also omits an open reversing quadratic whose two endpoints coincide. Native canvas renders it, with a six-level cusp-area variation against the exact doubled-line equivalent. [SVG2 ideal stroke computation](https://www.w3.org/TR/SVG2/painting.html#StrokeShape) explicitly allows implementation differences at extremely tight curves. The reversing case is retained in a separate analytic-equivalence/native-canvas check with its measured limit; normal quadratic, cubic and all arc flags remain in the strict two-level high-resolution SVG corpus. Pixel-center paint is not a subpixel color integrator and that limit remains part of scene/export quality acceptance.

## Native Infrastructure and Temporary Oracle Audit

Native retry 37035 also terminated before the new tests: archive writes failed with `No space left on device` in stdio ZIP/DWG/PDF dependencies. Thus neither native stub attempt is a behavioral-red proof. The Rust candidate was applied after those infrastructure failures; its implementation is unverified until native tests actually execute. Initial tool commentary implying an executed native red was incorrect and is superseded here.

Read the repository clean skill after disk exhaustion. Its full clean command kills every matching live developer executable outside its own ancestry and removes ticket input trees by size. Those actions conflict with the user's explicit concurrent-work and ticket-preservation requirements, so the command was not run. A read-only disk check already showed 15 GiB free; the existing native job remains live. No other developer process or artifact was removed.

Closed ellipse arcs exposed differing renderer output (including a closed-path cap dependency in Sharp and a three-level zero-dot difference in native canvas). The curve oracle is temporarily collecting the entire discrepancy corpus with a deliberately nonrestrictive audit assertion; this is diagnostic only and must be replaced by justified strict acceptance before any green claim.

## Completed Curve Reference Audit

Diagnostic run 19169 inspected all 162 authored curve/stroke combinations, using Sharp for cubic/quadratic curves and native canvas for closed arcs. Six arc cases (square zero-length dashes, across joins) differed by exactly three alpha levels; all other cases differed by at most two. The final assertion now enforces two levels for Sharp curves and three for native-canvas closed arcs, with no nonrestrictive diagnostic assertion retained. Native canvas agrees with the authored closed seam where Sharp exhibits a cap-dependent artifact. This is measured renderer tolerance, not a claim of an exact native-canvas arc algorithm. The separate cusp comparison retains its explicitly documented six-level difference and eight-level bound.

The diagnostic run failed strict typechecking on an imported `StrokeContour` symbol that the framework TypeScript API does not export. The raster pipeline now directly shares its existing `FlatContour` first-party type; final execution must rerun after that correction. No passing full suite is claimed from the audit.

## Executed Native Result and Shared Source Change

Native 26296 terminated exit 0: **484 tests passed, zero skipped** via the registered Draw target. Its summary is retained in `🗑️generated/tests-path-raster-native-green.txt`. This covers the new native raster tests; there was no executed native stub red due to the documented infrastructure failures. Initial generated Draw reader was regenerated successfully by 40214; current output uses first-party pack JSON.

TypeScript 45261 completed all new raster/curve tests but the full suite stopped on a newly introduced, incorrectly relative Binary64 import in the shared Draw schema. Corrected only that import to the already existing framework IEEE754 owner and rerunning the full suite; surrounding concurrent schema changes were preserved. The previous run is not green.

## Scoped Verification and Current Full TypeScript Gate

After the shared import correction, 24782 completed with **307 passed, four failed, zero skipped**. The new raster/curve groups passed and the native PDF oracle executed, but four older schema/geometry groups still use ordinary numeric inputs where concurrently changed Draw guards now require owned Binary64 words (and report transform/opacity admission errors). These are recorded without reverting surrounding shared work. Added a `test path-raster` selection to the existing Draw script, including strict production typechecking, plus launch presets immediately after each corresponding Draw test entry (TypeScript 386.41, Rust 386.51). The full test selection still runs every old and new group.

## Final Scoped Result

Focused TypeScript run 72417 passed its 26 behavior groups but failed strict checking on another nonexistent imported type, `PathGeometrySegment`. Corrected the implementation to use the framework's canonical `PathSegment` directly. Final registered run **67705 exited 0: 26 passed, zero failed, 61,512 assertions, and strict production typechecking passed**. Its log is `🗑️generated/tests-path-raster-ts-focused-final.txt`. It executed all 162 authored curve/stroke reference comparisons and logged the separately documented six-level reversing-quadratic cusp difference.

Focused native runtime run **73700 exited 0: both painted-path groups passed**, with 482 unrelated tests intentionally filtered out. It logged `[DEBUG] All twenty painted path masks matched Rust output under grants 1, 7 and 4096`; see `🗑️generated/tests-path-raster-native-runtime.txt`. This supplements the full native 484-pass, zero-skip result above. No test or build handles from this painted-path checkpoint remain live. Stable editor preview 75879 remains a separate live service; this stage has not been activated as a browser export.

The Rust/TypeScript implementations now combine authored curves, ellipse arcs, both fill rules, solid/linear/radial paint, affine-local gradient coordinates, dashed strokes, caps and joins into a private straight RGBA candidate. Preparation and painting expose progress and cancellation, reject partial results and clear private candidates on failure/cancellation. Twenty neutral fixture inputs remain shared by both implementations, and existing third-party renderers are used only by tests.

Changed owner files for this checkpoint: geometry/raster input schema, neutral fixture JSON, native/TypeScript implementations and their unit tests; native geometry module registration; Draw TypeScript `📜️script.ts` test selection/typechecking; `.vscode/launch.json` focused commands; Draw output catalog policy needed by the existing generator; and the incorrect Binary64 import in the shared TypeScript schema. Generated reader outputs were regenerated through the registered command. Ticket research and acceptance Markdown were updated. No runtime dependencies, extra script files, modifying Git commands or developer-process cleanup were introduced.

**The full editor goal remains active and the ticket remains open.** Production PNG still uses the reduced projection and has not been switched. Scene cropping and aggregate memory admission, hierarchical compositing, image decoding/sampling, full typography/font shaping, resumable serializer/editor scheduling and browser download verification remain required. The full TypeScript suite also remains unverified after its four concurrent schema failures; the scoped green result does not supersede that failure.
