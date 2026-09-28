# Draw End-User Acceptance

The goal remains active until the editing workflows below are implemented and demonstrated in the running app. A passing pure algorithm test does not establish that an editor workflow works.

| Workflow | Required behavior | Current evidence |
| --- | --- | --- |
| Document | Create/open/save; SVG and raster import; SVG/PDF/raster export; artboard/background dimensions | Existing document and export code; runtime and roundtrip validation pending |
| Navigation | Pan, zoom, fit artwork, accurate coordinates at every zoom | Shared framing request, measured viewport fitting and initial editor integration added; 15 React framing tests, 36 mounted input tests and native scene codec pass. Browser startup now fits the loaded Demo at camera (100,100,2.655), confirmed by console diagnostics and screenshot after gating both example completion and host publication; eight StrictMode startup tests pass. Artifact-level bounds now have Rust/TS twins and twelve neutral cases, including cubic/quadratic extrema and rotated image corners; the 39-test TypeScript suite passes. Viewer framing and retained local camera ownership are implemented, with independent-window and pack-restoration native tests authored; the full TypeScript suite now passes 39 tests. Six targeted native viewer tests now pass, including independent-window camera ownership/restoration and unchanged artifact bytes. The full Draw native suite passed 343/343 tests. React role switching now captures and restores complete archives before publication and reattaches saved document bindings; 23 preparation/gate/startup tests pass. Viewer retained restore hooks are implemented. Live archive replay now reaches the initializer but refuses mutation-container admission; the allocation repair is authored and awaiting native/build validation. Rollback preserves the editor and subsequent rectangle creation works. The native archive bridge roundtrip test passes with owned members and history. Successful document/history transfer and live navigation remain unverified. Native renderer/startup and explicit Fit Drawing/Selection remain pending |
| Creation | Repeatable rectangle, ellipse, line, polygon, pen, text, image, group | Layer-panel, rectangle and pen creation now have visible paint and automatic selection, verified in 334 native tests; repeated pen/polygon draft ID regression reproduced and fixed, verified in 335 native tests; browser rectangle creation, visible paint and automatic selection verified; other creation tools still need browser journeys |
| Selection | Click, additive/subtractive selection, marquee, actual lasso, select all/none, locked/hidden handling | Precise contour-based path and primitive picking, bounded traversal, nested visibility/locking, real lasso, and select-all/deletion pruning are implemented and covered by the 376-test native suite. Stable browser Demo click now selects Orange Wedge instead of Red Frame; Boolean/trace hit geometry, exact stroke envelopes and broader selection journeys remain pending. |
| Transform | Move, resize, rotate, aspect ratio, snapping, numeric entry; cancel gesture without history changes | Full affine transforms including shear/reflection, numeric controls, multi-selection/group movement, eight resize handles and rotation preview/release/cancel are implemented and covered by the 376-test native suite. Shift/Alt constraints are shared with TypeScript fixtures. Stable browser path move/resize/rotate and one-step undo are verified. Modifier gestures, cancellation, snapping, keyboard/focus and large-scene audits remain pending. |
| Paths | Anchor/handle selection and editing, insert/delete nodes, open/close/reverse/join paths, shape conversion | Numeric anchor/control edits, exact subdivision including arcs, delete/open/close/reverse/join contours and exact primitive conversion are implemented. Edit Nodes now shows anchors/control stems, previews affine node drags, cancels and commits one undoable edit; the native integration test passes. Stable browser anchor and Bézier control dragging change path geometry and one Command-Z restores each gesture. Convert to Curve is verified by pointer and keyboard in the browser; a modified undo chord from a focused action button restores the line in one step. Disclosure Enter/Space, native action buttons and retained focus after conversion/undo are verified in the browser. Atomic multi-point translation now has a shared schema, Rust/TypeScript twins, ten neutral cases and independent Three.js/Ajv validation (114 TypeScript tests pass); native command/history checks are running. Persistent single-point selection now has a framework domain, snapshot-bound references, selected-point overlays and local-edit rebinding; Rust/TypeScript topology/reference tests are authored, and 118 TypeScript tests pass. Native integration and rebuilt browser verification remain pending. Canvas keyboard nudging now has eight typed shortcuts and a shared atomic affine planner; 120 TypeScript tests pass, while new native execution/history and browser verification remain pending. Modified point picks and combined multi-path dragging are now authored, including preservation of layer selection on blank node clicks; the TypeScript suite passes 121 tests. New native gesture/history checks and rebuilt browser verification remain pending. Point marquee/lasso, node deletion shortcuts and joining separate layers remain pending. Readable handle-coordinate rows pass their native projection test; rebuilt browser validation remains pending. |
| Geometry | Accurate Bézier bounds; nested affine composition; Boolean operations; simplify; image trace | Exact curve/arc bounds and affine twins pass TypeScript/Three.js oracle; Boolean/trace user workflows remain unverified |
| Layers | Name, hide, lock, duplicate, delete, reorder, group/ungroup, nesting; invalid drop rejection | Inspector and atomic arrangement added; drop validation and unique creation repaired; rename, numeric position and lock/unlock verified in browser; ungroup and broader arrangement runtime remain |
| Appearance | Fill none/solid/gradient; stroke none/color/width/caps/joins/dashes; opacity/blending; text typography | Solid fill, enablement, stroke color/width, opacity and blend controls added; cap/join menus and dash patterns added with bounded parsing; invalid demo line caps corrected and typed Rust/TypeScript cap/join values introduced with 16 neutral cases; 112 TypeScript tests and 18,111 assertions pass, and the 378-test native suite passes, with browser Flat/Round and Miter/Bevel changes plus separate one-step undos now verified by UI state and console projection logs; typed gradient/stop controls and fill alpha added; semantic text content/size editing, bilingual multiline inspector controls and retained cancellation are implemented, with 46 TypeScript tests passing; native text/history and browser checks are pending. Typography layout/font controls, canvas gradient handles, large-fill scheduling and browser verification remain |
| History | Each logical edit is one undo/redo action; multi-selection edits all succeed or none do | The 376-test native suite passes, including geometry/text/transform mutations, multi-selection movement and new node gesture preview/cancel/undo/redo. Browser Undo/Redo lock transitions were verified earlier; broader workflow journeys remain pending. |
| Interaction | Large work yields with progress/cancellation; no unbounded selection traversal | Existing retained framework owners; new selection command still uses bounded dispatch and needs a full complexity/admission audit |
| Access | English/German controls, accessible names, keyboard actions, focus behavior, customizable panels | New controls localized and labeled; 20 mounted EN/DE accessibility cases pass; disabled locked inspector verified in browser; named disclosure and Enter/Space activation now work in the actual shell; 22 shortcut ownership tests and 53 Tree component tests pass. Native action labels expose Arrange and node actions to keyboard users; browser curve conversion and undo are verified. Focus returns to the owning node after action replacement and undo in the browser. Full tree navigation, native focus parity and broader keyboard workflows remain pending |
| Collaboration | Local-first event publication, coherent shared document edits, ephemeral local selection, short reconnect | Existing framework ownership used; concurrent creation identity improvements started; scenario validation pending |

## Validation Queue

1. Native run 78443 compiled and reached 96 passing tests before three new selection-workflow failures stopped the run (99/396 executed). The fixtures now finish initial framework selection publication before dependent nudge/pointer actions. Rerun 9208 (`tests-native-selection-settled.txt`) terminated before Draw compilation because shared `semio-framework-os-kernel` has twelve compile errors (`RetainedCloneClose`/trait/lifetime mismatches). Native verification of these fixes and SVG normalization remains pending; do not claim the 396-test suite passes. Earlier run 71498's stale test-only `blake3` failure no longer appears in 78443.
2. Poll fresh inspector component build 93519, explicitly activate the completed Draw component, and verify readable node/gradient controls in the browser. Stroke choices and cap/join undo are verified on the earlier completed component. Translation and point-selection sources changed after build launch; verify component/descriptor coverage and rebuild after terminal completion when needed.
3. Complete persistent node selection, node keyboard editing, native focus parity, separate-layer joining and the outstanding native vector renderer parity.
4. Complete appearance, layer, import/export, image, typography and geometry-algorithm workflows with cancellation and history validation.
5. Exercise collaboration/reconnect and the remaining end-user acceptance journeys. The full editor goal remains active.

## Path Editing Checkpoint

Path coordinate, control-handle, subdivision (line/quadratic/cubic), node deletion, contour open/close and reversal commands now have Rust/TypeScript twins and a windowed bilingual inspector surface. Shared TypeScript fixtures and independent curve/mutation oracles pass. Native and browser validation remain pending; exact arc subdivision is also implemented; direct canvas handles remain unimplemented. This does not satisfy the complete Paths or Canvas acceptance rows yet.

## Canvas Paint Checkpoint

React path construction now preserves SVG arcs, including rotated ellipse flags and multiple contours. The shared 2D raster painter uses the same geometry for fill, stroke and clips. Stroke width/dashes stay in document units across zoom, and disabled/zero-width strokes remain invisible. Eleven focused tests pass with independent Three.js SVG/path geometry oracles and neutral fixtures. Browser pixel verification and native renderer parity are still outstanding. These checks do not complete the broader Paths, Appearance or Document rows.

## Selection Topology Checkpoint

See [selection topology](🎯️selection-topology.md) for the browser reproduction, root cause, language-neutral fixtures, Rust/TypeScript twins and Three.js oracle. The TypeScript suite passes 49 tests / 1,152 assertions. Native Select All/deletion-pruning tests and the rebuilt browser behavior remain unverified while existing jobs 30810 and 43904 continue. The preview currently reports Loading plugins and its server log contains a refused trusted-catalog proxy connection. The full editing-experience goal remains open.

## Text Layout Checkpoint

[Text layout](📝️text-layout.md) records shared line handling, canvas/raster paint fixes, text-origin composition, framing and export changes. Renderer tests pass 15/15; shared 2D and Draw TypeScript targets pass. Native coverage and rebuilt text-control interaction remain pending. Font shaping and Unicode PDF embedding remain required. SVG style fidelity is addressed in the later checkpoint below; native and browser export validation remain pending.

## SVG Export Checkpoint

[SVG export](🎨️svg-export.md) records the direct typed SVG serializer and Rust/TypeScript twins. The latest Draw TypeScript run passed 54 tests / 1,197 assertions, including nonfinite geometry/paint refusal and third-party XML parsing of gradients, exact affine/path data, multiline text, image opacity and explicit absent paint. Native compilation remains pending on handles 30810 and 43904. Editable SVG roundtrip, group isolation and browser download still require validation.

The current preview recovered after a transient shared module export mismatch for `hubTransientApplyRefusalV1`; a reload restored the shell. Ongoing shared HMR then removed the action control before interaction could execute. This does not establish a successful editing journey. The source export exists; no framework change was made for this transient mismatch.


## Affine Transform Checkpoint

[Exact affine transforms](↗️affine-transforms.md) now retain shear, reflection and collapsed axes, including authored fixtures, semantic/retained mutations, digest/clone ownership and bilingual numeric controls. The TypeScript suite passes 66 tests / 1,307 assertions with Three.js and Ajv oracles. Native tests and rebuilt browser controls remain unverified; resize/rotate handle transaction integration is still outstanding. The full editing goal remains active.


## Transform Handle Checkpoint

[Transactional transform handles](🎛️transform-handles.md) implements the shared canvas selection frame and resize/rotation handles, bounded pointer preparation, absolute affine preview and one release edit. The registered TS suite passes 75 tests / 1,373 assertions. Native and browser workflow tests remain unverified. This does not yet complete the Transform, Access or Interaction acceptance rows; outstanding geometry, snapping, accessibility and large-work items are recorded in the checkpoint.

## Node Manipulation And Picking Checkpoint

[Direct path manipulation](🖱️node-manipulation.md) records two-axis semantic positioning, affine pointer math, neutral fixtures and the 81-test TypeScript pass. Native command/history verification remains pending. Browser testing reproduced incorrect selection in the Demo: clicking orange selected Red Frame because the current picker accepts path bounding boxes. Precise painted-geometry hit-testing and retained canvas node gestures remain required; neither the Paths nor Selection acceptance rows are complete.

## Painted Path Picking Checkpoint

[Painted path picking](🎯️painted-picking.md) records the contour-based Rust/TypeScript implementations, retained pointer integration, concave-frame regression and independent geometry oracles. The latest TypeScript suite passes 95 tests / 5,668 assertions. The native suite compiled and reproduced a zero-tolerance diagonal rounding defect; that defect now has a red/green shared regression, and the full native rerun is pending. Browser confirmation awaits the existing component build. Primitive/polygon/Boolean/trace picking, exact stroke envelopes and direct node gestures remain incomplete.

## Primitive Picking And Native Fixture Checkpoint

[Painted picking](🎯️painted-picking.md) now includes indexed rectangle/ellipse/circle/line/polygon contours and thirteen neutral primitive cases, with 108 TypeScript tests passing. The full native run exposed 48 failures, primarily current-schema fixture omissions; [native fixture repairs](🧪️native-fixture-repairs.md) records the corrections and the fresh native validation. Exact ellipse conversion, Boolean/trace picking, stroke fidelity and browser journeys remain open. The complete editor goal remains active.

## Exact Ellipse And Build Recovery Checkpoint

[Exact ellipse conversion](🥚️exact-ellipse-conversion.md) replaces the cubic approximation with quarter-ellipse arcs in both implementations. The TypeScript suite passes 108 tests / 18,031 assertions. The previous native run reached 371/373 passing; the two remaining issues have source/fixture corrections pending run 94642. [Component build recovery](🔒️component-build-contention.md) records sampled Cargo lock waits, cancellation of only this chat’s build, observed recovery of the competing build, and the fresh materialization run 67402. The goal remains active.

## Direct Node Gesture Checkpoint

[Node manipulation](🖱️node-manipulation.md) now includes canvas-tool registration, retained nearest-node picking, bounded two-segment geometry preview, visible handles and one-edit release with cancellation. TypeScript run 3337 passed 110 tests / 18,063 assertions plus 44 Ajv cases; native run 22911 passed all 376 tests. The small subsequent selection/hover refinements are being checked in run 21749. The browser component is rebuilding in run 79374 and must then be explicitly activated before verification. The complete editor goal remains active; all incomplete acceptance rows above remain required.


The final refinement run 21749 was interrupted by a newly introduced shared-framework compile error, not a Draw test failure. The invalid Clone derive on owned UiValue arguments has been corrected; run 59565 now validates the latest sources. Browser move, resize, rotation and single-step undo were verified on the stable preview; new node-tool browser verification still awaits component activation. Full editor acceptance remains incomplete.

## Atomic Multi-Point Edit Checkpoint

[Atomic multi-point editing](↔️multi-point-editing.md) records the schema-first translation operation, shared tangent deduplication, neutral fixtures, red/green TypeScript evidence and active native run. Persistent canvas point selection and its topology-change invalidation remain required. The full editor goal remains active.

## Persistent Point Selection Checkpoint

[Persistent point selection](🎯️point-selection.md) records the snapshot-bound reference contract, visibility/lock-aware topology, framework domain, filled node markers, local-edit rebinding, TypeScript evidence and native/browser verification queue. Multiple selection, keyboard editing and bounded-work ownership remain mandatory.

## Keyboard Nudge Checkpoint

See [Canvas Keyboard Nudging](⌨️keyboard-nudging.md). Arrow/Shift-arrow commands now share one atomic planner for layers and snapshot-bound point selection. The TypeScript suite passes 120 tests; new native integration tests and browser behavior remain unverified until the existing jobs finish and the new component is activated. The full goal and ticket remain active.

## Multiple-Point Gesture Checkpoint

See [Multiple Point Selection and Combined Dragging](🖱️multiple-point-selection.md). The point domain now supports modified membership and combined dragging; preview and release share the same affine point translation. The latest TypeScript run passes 121 tests / 18,318 assertions. Native run 78443 and component build 93519 remain active. Browser export produced the correct SVG effect and a blob download link, but no completed file delivery was observed; export remains unverified end-to-end. The misleading Export PDF label is corrected in source and awaits descriptor publication.


## Editable SVG Import Checkpoint

[Editable SVG Import](📥️svg-import.md) records the absent import registration, new Rust/TypeScript absolute path normalization, fifteen neutral fixtures and independent Three.js geometry checks. The registered TypeScript suite passes 136 tests / 18,688 assertions. A mixed quadratic/cubic reference-parser defect is explicitly documented in its fixture. Native verification is currently prevented by shared OS-kernel compilation errors. The document deserializer and end-user import workflow remain required; the complete goal remains active.

## Authored Fill-Rule Checkpoint

[Authored Fill Rules](🌀️fill-rules.md) adds persisted even-odd/nonzero winding, shared scene/picking semantics, owned clone/digest handling and updated authored assets. The registered TypeScript target passes 141 tests / 18,710 assertions. Native run 29440 terminated before Draw compilation on a shared UI `SceneMaterialKind3d::Authored` exhaustiveness error. Component build 93519 remains live. The semantic inspector mutation, rebuilt browser checks and complete SVG importer remain required; the full goal stays active.


## Fill-Rule Semantic Editing Checkpoint

See [Authored Fill Rules](🌀️fill-rules.md). Fill-rule persistence/projection/picking now has a semantic setter, sparse diff, inverse, retained ownership/digest integration and bilingual inspector control. TypeScript validation passed 142 tests / 18,718 assertions and 48 field-patch cases. Native history/codec/inspector checks are authored; run 79513 is still live. Component run 93519 is still live; app interaction remains unverified. Full SVG import and the full editing goal remain open.


## Editable SVG Transform Checkpoint

[SVG import](📥️svg-import.md) now includes complete affine transform-list normalization in Rust/TypeScript with 28 neutral cases and independent Three.js geometry checks. Registered TypeScript checks passed 170 tests / 18,906 assertions, 48 field-patch cases and 36-command publication audit. Native run 79513 and component run 93519 are still live and predate the transform source. Full document import and app interaction remain incomplete. Previous goal turn was progress (semantic fill-rule setter, inspector, and passing TS evidence); this turn is progress (transform implementation and passing third-party/grammar evidence).


## SVG Document Builder Checkpoint

[Editable SVG Import](📥️svg-import.md) now records Rust/TypeScript document jobs with hierarchy, inherited solid paint, viewBox mapping, basic text and cancellation, plus source-level SVG IO registration. TypeScript reached 178 passing tests / 18,919 assertions. Native and app checks remain unverified. The IO registry requires ready-on-first-poll deserializers, so responsive end-user import must drive `SvgImportJob` through the editor's scheduling layer. Gradients/images/CSS, nested viewports, text fidelity and fully budgeted parsing remain required. Previous goal turn and this turn are progress; no completion claim.

## Continued: Gradient Verification, Native Assertion Corrections, and World Arrangement

See [SVG import](📥️svg-import.md) and [world arrangement](📏️world-arrangement.md). Linear gradient import passed the full TypeScript run (186 tests), including two Sharp raster oracles. Arrangement then reached 188 passing TypeScript tests / 19,058 assertions before an additional grandparent fixture. Native 79513 is terminal: 404/407 passed, three representation assertions corrected. Fresh native 88159 is live. Build 93519 and preview 76268 remain live; browser claims have not advanced. Goal and ticket remain open.

Final arrangement validation: **188 TypeScript tests / 19,076 assertions** passed (`tests-arrangement-final.txt`, exit 0). Native 88159 and build 93519 confirmed still live. See the arrangement note for current source changes and remaining limits.

## Continued: Layer Stack Steps

See [layer stacking](🗂️layer-stacking.md). Added localized Bring Forward / Send Backward controls, stable multi-selection planning and no-op boundaries in Rust/TypeScript. 189 TypeScript tests passed before the final planner optimization; final run 43880 is live. Native 88159 and build 93519 remain live. Native inspector/history additions and browser behavior remain unverified. The prior goal turn made concrete arrangement progress; this turn adds layer stack editing without narrowing the full goal.

Final stack run 43880 passed 189 tests / 19,139 assertions and both audits (exit 0). Native 88159 and component 93519 were polled afterward and confirmed live. No browser actions occurred this turn.

## Continued: Ungroup Editing and Group Compositing Finding

See [ungroup](🧩️ungroup.md). Added schema-first Rust/TypeScript ungroup behavior, localized controls and native history/selection coverage. Initial eight cases passed in the 190-test TypeScript run; final eleven-case run 72547 is pending. Scene projection currently ignores group opacity/blending; exact isolated compositing is still required, and ungroup refuses those selected groups for now. Native 88159 and component 93519 remain live. The preceding turn made concrete layer-stack progress; this turn adds structural editing without narrowing completion criteria.

Final ungroup run 72547 passed 190 tests / 19,265 assertions / 26 files and both audits (exit 0). Native 88159 and component 93519 were polled afterward and confirmed live. No browser actions occurred this turn.

## Continued: Group Compositing

See [isolated group compositing](🧩️group-compositing.md). Added group scope projection, reusable canvas composites and nested Rust/TypeScript SVG export. Independent raster comparisons passed with 197 Draw TypeScript tests / 46,952 assertions, and 52 focused renderer tests passed. Native 88159 finished 414/418; three fixture document IDs and one projection-only disabled-state assertion were corrected. Fresh native 5483 is live. Build 93519 remains live. PDF/native/raster parity, large-document budgets, effect-bearing ungroup and browser verification remain incomplete. The prior turn made ungroup progress; this turn changes real paint/export behavior without narrowing the goal.

Final compositing validation 24022 passed 197 tests / 46,955 assertions and both audits (exit 0). Native 5483 and component 93519 were polled afterward and confirmed live.


## Continued: PDF Compositing

See [PDF compositing](📖️pdf-compositing.md). Native export now carries isolated group and leaf compositing through scoped Form XObjects, with resource indexing and shared fixture tests. The registered PDF.js/Sharp pixel oracle awaits native fixture bytes. Draw TypeScript run 85503 passed 197 tests / 46,955 assertions with the new PDF oracle explicitly skipped; the later canvas-factory refinement is unexecuted. Native 5483 and component 93519 remain live and were not restarted. Native/PDF/browser parity and the full acceptance scope remain incomplete. This turn made source and test progress, not a completion claim.


## Continued: Full Blend Corpus and SVG Round Trips

See [blend modes and SVG round trips](🎨️blend-roundtrip.md). Corrected PDF's authored blend vocabulary and SVG import's CSS normalization/isolation handling. Expanded the neutral corpus to 37 scenes and verified every canvas/SVG blend plus editable SVG export/import/re-export pixels. Red run 41570 reproduced 26 failures; final run 77011 passed 270 tests / 145,453 assertions with the native PDF pixel oracle still skipped. Native 5483 and component 93519 remain live. [Raster export research](🖌️raster-export-research.md) records the remaining PNG rendering losses and the relevant first-party compositor. Full rendering, export, isolation-schema and end-user acceptance remain open.


## Continued: Authored Group Isolation

See [explicit isolation](🧩️explicit-isolation.md). Added the group field, semantic mutation and inverse, sparse diffs, retained clone/digest/candidate integration, localized inspector toggle, mixed boolean controls and explicit SVG isolation import. The 38-scene corpus includes the opaque isolated-group case. Final TypeScript run 16071 passed 273 tests / 148,561 assertions / 28 files; the native PDF oracle remains explicitly skipped. Native 5483 ended with one SVG serializer return-type error before tests; the caller was corrected and fresh full native run 17529 (`tests-native-isolation-pdf.txt`) now includes PDF fixture emission. Component 93519 remains active. Native/editor/browser verification and the rest of the full acceptance scope remain open.


## Full Blend Mode Inspector — 2026-09-28

See [🎨️blend-inspector.md](🎨️blend-inspector.md). Sixteen modes are now exposed with EN/DE labels and canonical field admission. RED session 79326 reproduced colorDodge rejection; GREEN 89172 passed 273 tests plus all 69 Ajv field cases and the 36-command publication audit. One PDF raster test still skipped. Native 17529 and component build 93519 remain live; native/browser verification of these latest edits remains pending. Native descriptor and isolation fixture assumptions were corrected in source. Ticket and goal remain active.


## Blend Mode Boundary Validation — 2026-09-28

See [🛂️blend-boundaries.md](🛂️blend-boundaries.md). RED 56195 demonstrated nested invalid mode acceptance. GREEN 12112 passed 274 tests, one skip, 148689 assertions, 69 Ajv field cases and the 36-command publication audit. Rust semantic, raw-diff and retained-candidate vocabulary guards/tests added; native execution pending. Handles 17529 and 93519 remain confirmed live. Native model deserialization/full-layer replacement coverage remains incomplete; the overarching goal and ticket stay active.


## Keyboard Node and Layer Deletion — 2026-09-28

See [⌫️node-deletion.md](⌫️node-deletion.md). Schema-first batch anchor/handle deletion implemented in Rust/TS; Delete and Backspace registered to a single atomic selection command. RED 91244 reproduced absent operation; GREEN 59617 passed 274 tests, one skip, 148721 assertions and the 37-command publication audit. Native command/inverse/history tests added but pending execution. Handles 17529 and 93519 remain live. Large-document resumability and browser keyboard acceptance remain open.


## Node Marquee Selection — 2026-09-28

See [▧️node-marquee.md](▧️node-marquee.md). Empty-space drags in node mode now prepare rectangle selection with replace/add/toggle and cancellation-preserving behavior. Native traversal hashes and collects anchors incrementally. GREEN 54316: 275 passed, one skipped, 148733 assertions, 69 field cases, 37-command audit. Native geometry/query/app journey tests added, execution pending. Handles 17529 and 93519 remain confirmed live. Browser verification and full goal completion remain open.


## Point Selection Publication Review — 2026-09-28

See [📨️point-publication.md](📨️point-publication.md). Fixed retained query serialization to publish the actual merged point selection with point granularity and check its escaped JSON byte size. Added neutral escaped-ID targets plus native overflow and inherited visibility/lock regressions. Native execution remains pending; 17529 and 93519 confirmed live after edits. No new test-suite pass claimed. Full editor goal remains active.
