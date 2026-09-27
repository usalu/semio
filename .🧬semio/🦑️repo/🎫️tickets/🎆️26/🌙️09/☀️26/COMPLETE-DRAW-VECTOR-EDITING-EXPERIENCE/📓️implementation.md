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

## Regression Verification Checkpoint

Resolved the full native suite's twelve failures. Production corrections remove close-only phantom path bounds and decode Boolean child layer IDs without confusing dots or Unicode in IDs. Fixture corrections use typed numeric comparisons, real retained envelope admission, explicit inspector pages and drawable stroke targets. Registered editor runtime verifies drag preview/cancel/commit and exact undo/redo for contour joins, segment conversion and whole-shape conversion. The final full native suite passes 327/327 (no skips). See `🖥️runtime-verification.md`, `↔️direct-manipulation.md`, and `📐️path-editing.md` for evidence and limitations. Goal remains active.

## Verified Group Movement and First Browser Workflow

The new full native suite passes 331/331. Browser inspection now reaches actual local Draw editing: layer creation and selection expose the real property controls. This revealed invisible creation defaults, missing automatic selection and an unfitted initial camera, all recorded in `🖥️runtime-verification.md`. These are next concrete usability fixes. The current targeted component/descriptor build remains live (21529); the hub build failed on the remaining eleven lifetime errors after its observability response type was corrected. Goal/ticket remain active.

## Creation Workflow Repair

The browser-discovered unpainted/unselected creation problem is addressed by a shared editor creation policy. Add Layer and gesture commits now initialize visible paint and request framework selection of the created IDs. Existing explicitly supplied paint is preserved. Rust and TypeScript share ten appearance fixtures; the TypeScript/Immer oracle passes 24 tests/948 assertions. The full native suite passed 334/334 after the paint and selection change, including registered editor layer-panel, rectangle tool and pen creation.

A targeted regression then reproduced repeated pen-draft identity reuse (`tests-draft-identity-red.txt`: failed on the second pen creation). Retained pen/polygon draft completion now uses the same document/operation-aware identity assignment as Add Layer. The helper was extracted without changing Add Layer's ID algorithm. Each retained draft captures its exact operation context; the fallback fixture path also checks existing document IDs. The full post-fix native run is active as 38978 (`tests-creation-final.txt`). No passing post-fix result is yet claimed.

The prior matching component/descriptor build 21529 succeeded but predates this creation workflow repair. A new matching build and browser verification remain necessary. Initial camera framing and individually labeled inspector controls remain open findings.

### Creation Verification Complete in Native Runtime

The post-fix full suite passed: **335 tests run, 335 passed, zero skipped**, assertion duration 2.028s, Nx task 17.3s (`tests-creation-final.txt`). This includes visible/default-preserving creation paint, immediate selection and distinct repeated pen/polygon drafts. It supersedes the pending native identity-fix status. TypeScript remains passing at 24 tests/948 assertions. `git diff --check` passes.

A fresh matching Draw-only component/descriptor build is now running (`draw-describe-creation.txt`) for browser verification. The previous matching build is terminal-successful and must not be polled or restarted as if it were still active. Browser creation controls should be retested only after the new build completes, because hot reload reset the demo during the prior browser edit attempt. No browser paint/automatic-selection pass is claimed yet.


## Inspector Usability Dispatch and Accessibility

Corrected Draw blur input bindings to Commit for property, fill/gradient and path-coordinate edits. Restored labels and disabled states in the shared React input/select interpreter. Added neutral event/locale fixtures, a native projection regression, and 20 mounted React cases with a third-party accessible-name oracle. Browser verification now covers rename, position persistence across selection, locked controls, and undo/redo of locking; see `🖥️runtime-verification.md` for exact evidence and test limits. Full native validation remains unresolved after two runner budget terminations. Canvas framing, direct manipulation and the remaining acceptance matrix are still open; the overall goal and ticket remain active.
