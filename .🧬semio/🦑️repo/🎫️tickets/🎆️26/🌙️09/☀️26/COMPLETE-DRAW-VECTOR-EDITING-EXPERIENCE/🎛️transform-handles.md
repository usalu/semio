# Transactional Canvas Transform Handles

## Current-State Finding

The earlier audit only considered the older render hook. The current `render_with_request_context` hook already receives InteractionView and uses it for the inspector. Draw can pass that same authoritative selection to its canvas render. No host-specific incremental-mutation gumball is needed for Draw. The same scene paths can paint the selection box and handles in React/native rendering; their pointer input remains in the existing retained Draw gesture owner.

The implementation adds eight resize handles and a rotation handle, world-space absolute matrices, fixed screen-size hit regions, original-transform capture, transient scene preview, release-only atomic mutations and existing Escape/cancel rollback. Pointer-down preparation must retain bounded traversal and preserve selected ancestor deduplication. Locked/hidden layers must never be transformed. Shift constrains corner proportions and rotation. Existing numeric controls provide a keyboard-accessible alternative; individual focusable handles and changing modifiers mid-drag need their own completion checks.

Seven neutral drag fixtures cover resize, edge resize, reflection, quarter-turn rotation, aspect preservation and returning to the start. The registered red test (57428, exit 1) failed on the missing handle module before implementation. This is not yet a verified user workflow.

## Implemented Flow

- Added Rust/TypeScript handle geometry twins: eight resize anchors, rotation above the selection, an eight-pixel hit radius, opposite-edge anchoring, reflected scales when crossing an anchor, aspect constraints and 15° rotation snapping.
- The request-context render hook passes the current framework selection into canvas rendering. It paints a selection rectangle, rotation stem and nine handle squares. Handle squares retain screen size during preview. The same scene data reaches both renderers; native pixels remain unverified.
- The existing retained pointer query now accumulates selected bounds during its bounded traversal, including selected ancestors, and checks handles before the ordinary object drag. Preparation captures original transforms and deduplicates selected descendants of selected groups. It retains existing visibility/lock/singular-parent refusals.
- Preview projection now carries one absolute world matrix. The old translation-only preview API was replaced throughout Draw. Selected groups apply that matrix once to their descendants.
- Release conjugates the world matrix through each selected root's parent, decomposes it without losing shear, and publishes one logical resize/rotate edit. Ordinary move gestures keep their original transform decomposition. Returning to the start, Escape and pointer cancellation retain the existing no-edit paths.
- Shift pressed at pointer-down constrains a handle. A shifted press away from a handle returns to the existing additive selection release path.
- Text hit-test bounds now use shared multiline Unicode line extents. Constant-size shape bounds now use their actual path geometry instead of transforming a coarse local rectangle.

## Verification

- First corrected registered Draw TS suite: **73 tests / 1,361 assertions**, handle 79456 exited 0.
- Added negative-angle snapping and invalid handle-input checks. Final registered suite: **75 tests / 1,373 assertions**, plus **44 independent Ajv field-patch cases**, handle 36279 exited 0; `tests-handles-contract.txt`.
- Neutral expected matrices and Three.js point mapping verify resize, reflection, rotation, fixed-screen hit targets, constraints and identity return.
- Native unit tests exercise retained handle preparation, unchanged document previews, cancellation and one release transform. A registered-editor test checks nine rendered handle records, resizing/rotation of a child under a sheared nonuniform parent, no artifact publication during preview, one release publication, exact world geometry and undo/redo.
- Those native tests are authored but **not verified**. Native suite 30810 and component build 43904 were polled after the changes and remain live. Their source-capture timing must be checked at completion; no duplicate build was launched.
- Scoped `git diff --check` passed. Preview tab 4 is retained for browser work after the component is rebuilt. No browser handle interaction or native pixel pass is claimed.

## Remaining Work

The render-side selection bounds traversal still completes synchronously inside the existing synchronous scene projection; a large-document render/progress audit is required. Boolean/trace picking still contains pre-existing fallback bounds, so their handle boxes need exact retained result geometry. Focusable/announced individual handles, cursor feedback, snapping to guides/grid/other objects, changing modifiers during a drag, and center-anchored Alt resizing remain. Native compile/runtime results and real browser pointer/cancel/undo checks must pass before marking this workflow complete. The overall Draw goal remains active.

## Live Modifiers And Center Anchoring

Added four neutral centered-resize fixtures (corner, edge, aspect constraint, reflected scale). The initial registered run failed exactly those four cases: `tests-handles-centered-red.txt`, handle 48297, exit 1. Rust and TypeScript now accept an explicit centered flag, anchor at the original selection center and use half-extents for the drag ratio. The corrected registered Draw suite passed **79 tests / 1,405 assertions**, plus **44 Ajv field-patch cases** (`tests-handles-centered.txt`, handle 31918, exit 0).

The React canvas session now copies modifiers into each queued pointer sample. The emitted batch takes modifiers from its final sample, never from mutable state at dispatch time. A delayed-dispatch test mutates the caller's key object after sampling, releases with different keys, and queues later hover; it failed before implementation (`tests-pointer-modifiers-red.txt`, handle 2803, exit 1). After the host change, the gesture-lane and mounted input suites passed **43 tests in 2 suites** (`tests-pointer-modifiers.txt`, handle 59302, exit 0). The infinite-canvas DOM event driver now forwards modifiers on movement as well as down/up.

Draw's pointer command records carry Alt on down/up and Shift/Alt on move. The retained transform owner updates its constraint and center flags before every preview and before release. Registered-editor coverage now toggles both flags during one gesture and exercises cancel/history under the new mode. These Rust tests remain unverified. Key-only changes with a stationary pointer do not yet trigger a new preview; release still uses current modifiers.

The native scene interpreter also discarded movement modifiers. It now forwards the event's SceneModifiers through CanvasPointerWire; the existing owner/cancel test asserts live Shift/Alt and their cleared release values. Native renderer tests are authored but not yet executed for this change.

## Native Build Follow-Up

Native test process 30810 finished with exit 1 after 121 minutes: missing `semio_s_artifact_stdio_xml` and unresolved `export`. The XML dependency was already added after that process initialized its graph; the SVG wrapper's missing local module import was fixed here. A fresh registered native suite is running as **29811**, log `tests-native-handles-svg-fixed.txt`. It started before the modifier edits, so source capture and coverage need checking at completion. Component materialization **43904** remains live; no duplicate was started. The broader ticket and goal remain open.

The registered native renderer subset was dispatched after all native modifier edits: `bun nx run @semio-tech/framework-renderer-wgpu:test-wgpu-unit --excludeTaskDependencies -- scenes::canvas2d_tests`, process **23844**, log `tests-native-pointer-modifiers.txt`. Keep polling this handle rather than starting another copy. A focused diff whitespace check passed after the edits.


## Browser Resize Evidence

Stable preview 6065 selected Orange Wedge and exposed its eight resize handles plus rotation stem. Dragging the bottom-right handle from (470,632) to (500,652) changed scale X to 1.3228410008071025 and scale Y to 1.0491761924332275, with translations (-0.4035512510088782,-2.1514584189537014) preserving the opposite geometry corner. The canvas inspector retained the selected layer. Error/warning console query remained empty. Undo and rotation are being checked next.


Resize undo restored X=Y=0 and scale X=Y=1. Dragging the rotation handle from (423,197) to (500,197) then produced Rotation=18.4440702967987 degrees with scale unchanged. Screenshot `🗑️generated/rotation-handle-verified.png` captures this state. A single Command-Z was sent to restore the demo; completion is checked separately.


Rotation undo completed: X=0, Y=0, Rotation=0; scale stayed 1. Final browser warning/error query was empty. Browser move/resize/rotation plus single-step undo are verified for this selected path. Modifier gestures, other transform topologies and cancellation still require browser checks.
