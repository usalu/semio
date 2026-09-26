# Draw Vector Editing Experience

## Scope and Acceptance

Deliver coherent editing of vector documents through the existing Draw artifact and framework command/event system: selection-aware inspection; layer hierarchy, visibility, locking, ordering and grouping; shape and path editing; appearance and transforms; geometry algorithms; undoable commands; localized accessible controls; import/export; executable behavioral coverage and runtime validation.

## Baseline Research

The drawing artifact already has paths, shapes, text, images, groups, booleans, tracing, typed mutations, gesture owners, native serialization and PDF/SVG export. Its inspector currently renders only a schema/utility/layer-count summary, because an older implementation predates the framework's `render_with_request_context` selection API. The framework now exposes that API, and CAD demonstrates its use. Draw still uses the older render override. Prior end-to-end tickets document historical runtime and harness failures; those are evidence for investigation, not current test results.

Repository MCP is reachable through the configured stdio launcher. Read `repo://goals` and associated this ticket with Running Drawing. Reopening the June completeness ticket partially renamed its directory before returning errors; the current implementation is tracked in this September ticket. No Git modifications or worktrees are used.

## Validation

Baseline Nx TypeScript tests and Rust checks started; results pending. New behavior will be specified in language-neutral fixtures before implementation and checked through owned implementations and existing independent libraries where applicable.

## Implementation Checkpoint

Replaced the placeholder inspector with selection-aware, localized controls and mixed-value handling. Edits use semantic mutations and atomic history commits. Added validation for finite numeric input, ranges and colors, degree-based rotation, fill/stroke enablement and stroke color. Added selection arrangement commands for grouping, duplication, deletion, stacking, alignment and equal-gap distribution. Deep duplication assigns descendant identifiers and remaps internal Boolean references; layer row decoding preserves dotted IDs.

Added independent Rust/TypeScript affine and cubic-extrema geometry with shared fixtures. Group transforms now reach scene rendering and geometric bounds. Added language-neutral selection fixtures, command schema, Three.js/Immer oracle and Ajv patch validation. Registered Draw validation in the launch seed and launch configurations.

Validation actually completed: Nx TypeScript suite passed 6 tests / 47 assertions, plus 18 Ajv field-patch cases and 26-route publication authority with 5 hostile checks. This run predates the additional fill/stroke enablement cases (23 total), which still require rerunning. Rust checking and selected tests remain in progress. The ordinary check was blocked in framework graph generation by unrelated WGPU taxonomy ordering; the targeted check uses Nx's excludeTaskDependencies with existing generated dependencies. No browser runtime confirmation yet.

Remaining acceptance work includes direct manipulation and node editing, complete path algorithms and appearance controls, robust layer reparenting, import/export roundtrips, cancellation behavior for large selections, and browser interaction validation. The overall goal and ticket remain open.

The failed June ticket reopen renamed its directory without changing its metadata; verified the metadata against the repository version and restored the original directory name, preserving all contents. The expanded TypeScript suite passed again, including all 23 appearance patch cases. Rust compilation found eight integration errors (UI contract Label versus locale Label, action builder method, and module path); these were corrected and the check is rerunning. Preview startup failed because the root dev router required an Nx daemon that did not start with the isolated workspace settings; investigating the direct existing Nx serve target.

## Latest Verified Checkpoint

The corrected Rust artifact check completed successfully through Nx (exit 0; approximately six minutes, dependencies excluded because graph generation was separately blocked earlier). The latest TypeScript run passed 7 tests / 49 assertions, all 23 field-patch cases, and publication authority for 26 routes with 5 hostile checks. Rust behavioral tests are still building/waiting in the shared native toolchain and have not reported assertions yet. Repeated add/drop creation now generates operation-scoped layer IDs, and layer drops resolve exact IDs, reject locked/invalid/cyclic destinations, and adjust final sibling indices. New fixtures cover repeated creation and layer drop semantics.

The direct existing Nx Draw serve target reached the dev router. It reported that the staged Draw bundle predates these sources and is waiting on the existing hub process. Started the indicated existing Nx activate-draw-react-dev target to build/activate current sources. Browser verification remains pending.

Added shared Rust/TypeScript exact cubic splitting and affine inverse helpers for upcoming node editing. The new Three.js oracle passed after changing the inverse fixture comparison to numerical tolerance (IEEE negative zero is numerically equal to zero): 8 TypeScript tests and 89 assertions passed, plus 23 patch cases. Subsequently aligned numeric scale patch validation with the existing mutation authority's positive-scale requirement and added three boundary cases (26 total); that final expansion still needs its next run. The initial native test build remains pending, and a separate Nx check of test compilation has started. No `UpdatePathGeometry` mutation has been added yet; its required persistence/digest/retirement integration is documented in the path-editing design note.

The attempted `check --tests` was rejected by the repository's native input contract before Cargo ran; test compilation therefore remains with the already-running dedicated Nx test target. This is a tooling argument rejection, not a compiler result. Remaining workflows are tracked in [End-User Acceptance](📋️acceptance.md), with the next persistence change detailed in [Semantic Path Editing Design](📐️path-editing.md).

## Path Persistence Progress

The previous goal turn made concrete progress (source changes, independently passing fixtures and a successful library check). Current sessions were re-polled and remain live; no build was restarted merely because its observer timed out. Added the schema-first `UpdatePathGeometry` semantic mutation with an explicit typed diff facet, inverse, Rust/TypeScript implementations and a shared before/after curve fixture. Integrated segment hashing, ownership census, one-segment clone steps, cancellation retirement and commit into the retained mutation authority. Added native tests for roundtrip, exact retirement, cancellation without document changes and control-point digest distinction. The first TypeScript mutation/Immer/Ajv run passed; an arc case was then added to cover `largeArc` serialization. Rust compile and behavior verification are still pending.

Current verification handles: native behavioral test session 5335 remains live (original selection/geometry filter; new path mutation/cancellation tests need inclusion in the next dedicated run); path mutation library check session 63005 is live; Draw activation session 15883 and serve session 94715 remain pending. Latest expanded field-boundary TypeScript run completed successfully with 26 patch cases. The first path mutation TypeScript run passed, followed by additions for arc serialization and native cancellation/digest tests. No end-to-end node editing is claimed.

## Node Editing Progress

See [Path Editing](📐️path-editing.md) for the new typed mutation, schema surfaces and command/inspector integration. Latest TypeScript command publication test passes (10 tests / 150 assertions; 27 registered routes). Rust library check and Draw preview activation are running with the corrected segment DSL annotation. Native tests continue waiting/building through the shared toolchain. No browser interaction has been verified. The overall goal remains incomplete.

Native check exposed DSL enum/required-field codec declaration errors in the new editPath command. Fixed by using DslScalar for unit fields and Box<PathEdit> for the required tagged edit; subsequent native check is in progress. TypeScript layer-parser regression test failed before the fix and passed after preserving schema-authorized fields. Latest tests are in generated/ts-layer-parser-fixed.txt.

## Arc and Selection Checkpoint

Exact arc subdivision, affine arc bounds, nested canvas picking, ancestor visibility/locking and frontmost picking are implemented with shared fixtures. See [Path Editing](📐️path-editing.md). Native library checks for the path command and selection geometry passed; native tests remain live. TypeScript arc/extrema tests passed (13 tests, 264 assertions, 26 field-patch cases and 27 publication routes).

[Runtime Verification](🖥️runtime-verification.md) records the actual blank browser result and pending activation. No browser workflow is claimed working. The goal and ticket remain open.
