# Stroke Appearance Controls

Added English/German inspector fields for line caps (flat/round/square), joins (miter/round/bevel), and editable dash patterns. Dash patterns use nonnegative lengths separated by spaces; an empty or all-zero pattern produces a solid stroke. The input is bounded to128 ASCII characters, preventing an unbounded parse or mutation payload. Existing multi-selection, ancestor locking, mixed-value projection, and commit-on-blur behavior applies.

Field changes dispatch through ReplaceLayerStroke, preserving the other stroke attributes and using existing event/history ownership. The action field catalog exposes the same fields. Added Rust/TypeScript dash parser twins with shared language-neutral cases and an independent Ajv grammar oracle. Schema-first field-patch fixtures cover valid and invalid cap/join/pattern values.

During integration, the shared property decoder was found to misread numeric-looking text as JSON numbers. PatchLayer and PatchLayers now use a shared field-aware decoder, so a name `123` or a single dash length `8` remains text. Numeric and Boolean fields retain their typed parsing. Native tests cover appearance preservation, inverse restoration, scene projection, and bilingual inspector controls.

Validation completed: preimplementation test failed because the parser module did not exist; `ts-stroke.txt` then passed16 tests with309 assertions,42 field-patch oracle cases and27 published command routes. `git diff --check` passed for the Draw plugin. The native library check `check-stroke.txt` passed in3m32s. Full native tests are still running in `tests-lasso-all.txt`, session46142; its source capture timing must be checked before attributing subsequent edits to that run. No browser stroke workflow has been verified.

Also corrected the descriptor test's assumption that every mutation targets a layer: UpdatePathGeometry correctly declares the path entity.

The latest lasso/stroke component activation is running in `activate-stroke-lasso.txt`. Browser tab4 briefly exposed the Draw shell and canvas headings, then displayed Loading plugins. This is progress beyond the empty DOM, but does not establish a stable or usable editor.

## Typed Gradient Editing

Added schema-first FillEdit operations and Rust/TypeScript twins: none/solid/linear/radial conversion, gradient coordinates, solid/stop opacity, stop colors and offsets, insertion with color interpolation, and removal while retaining at least two stops. Switching gradient kinds preserves the stop list. Rejected edits preserve the source.

The new editFill command commits one ReplaceLayerFill mutation. The argument bridge supplies typed numeric, color and selection values from the inspector. The published command set now contains28 routes (22 bounded commands and6 gesture routes). Added native command-codec, bridge and bilingual/locked inspector tests.

The inspector exposes a fill-type select, linear endpoints or radial center/radius, solid alpha, and windowed gradient stop rows with color/alpha/position plus add/remove actions. New stops occupy the midpoint of the largest existing interval.

Completed validation: preimplementation TypeScript run failed for the missing fill module. `ts-fill-oracle.txt` passed17 tests/356 assertions,42 field-patch cases and28 publication routes. Three.js Vector4 independently checks stop interpolation; Immer independently applies alpha, offset and removal edits. Native library check is running in `check-fill.txt` (session80134). The full native test run is still live (session46142), and the prior component activation completed successfully in11m21s.

Known remaining limitations before the full goal can be complete: fill editing currently limits the synchronous operation to64 stops; large-gradient editing must move to a resumable owner. New gradients start with local coordinates0..100; creation should use actual local artwork bounds or an explicit relative-coordinate model. Direct canvas gradient handles and stable browser interaction are unverified/unimplemented. The earlier activation began before these fill changes and cannot be assumed to include them.

`activate-stroke-lasso.txt` completed successfully and activated one changed Draw component. Its start predates the fill work, so browser inspection must establish which controls it includes.

## Visible Creation Defaults

Browser inspection confirmed that Add Rectangle previously produced no fill and no stroke. New authored closed shapes now receive an opaque teal fill and a two-unit dark round outline; lines and pen paths receive the outline without a fill; Boolean/trace results receive a fill. Explicit existing appearance is preserved. This is an editor creation policy, applied consistently by the Add Layer command and gesture commit path; schema constructors and imported document styles are not rewritten.

New layer creation requests framework selection of the exact created ID(s). The 334-test native run `tests-creation-paint.txt` passed in full, including actual registered editor creation, scene paint visibility, selected IDs, rectangle gesture and pen draft publication. Shared appearance fixtures and the TypeScript/Immer oracle passed: 24 tests, 948 assertions, plus 42 field cases and 28 publication routes (`ts-creation-paint.txt`). The initial TypeScript run failed on the absent policy module before implementation.

Repeated pen/polygon drafts were found to retain fixed constructor IDs. A new targeted regression is running before that correction (`tests-draft-identity-red.txt`, session 75983). The previously started matching component build completed successfully in 6m37s, but it predates these new creation defaults; browser verification requires another matching build after creation fixes are finalized.

### Creation Verification Complete in Native Runtime

The post-fix full suite passed: **335 tests run, 335 passed, zero skipped**, assertion duration 2.028s, Nx task 17.3s (`tests-creation-final.txt`). This includes visible/default-preserving creation paint, immediate selection and distinct repeated pen/polygon drafts. It supersedes the pending native identity-fix status. TypeScript remains passing at 24 tests/948 assertions. `git diff --check` passes.

A fresh matching Draw-only component/descriptor build is now running (`draw-describe-creation.txt`) for browser verification. The previous matching build is terminal-successful and must not be polled or restarted as if it were still active. Browser creation controls should be retested only after the new build completes, because hot reload reset the demo during the prior browser edit attempt. No browser paint/automatic-selection pass is claimed yet.

### Text Editing Gap Audit — 2026-09-27

Current DrawingTextBody stores x, y, content and size only. The inspector emits no text-specific fields, and drawing_op_for_layer_field refuses text content and text-size edits. The mutation vocabulary has no text-content/style mutation. Thus creating a text layer does not currently provide an end-user text editing workflow. This is an implementation gap, separate from the ongoing role-transfer repair.

The correction requires schema-first text mutation payloads and their diff/inverse/binary/text twins, owned replay/retirement integration, shared neutral fixtures and a JSON Patch oracle, then bilingual inspector controls using one commit on blur and atomic multi-selection semantics. Content must preserve numeric-looking and multilingual strings exactly; size must be finite and positive. Font family/style/weight, alignment and multiline metrics also need an explicit artifact/rendering contract rather than UI-only fields. Browser attempts during the current rebuild encountered live preview reloads before an added layer could be inspected, so no successful text-creation runtime claim is made from those attempts.
