# Native Editing Parity Audit

The preceding goal turn made progress: full transform controls and retained history obtained native evidence; two fixture defects were repaired and export diagnosis was started. Current native rerun 75362 and activation 18283 were revalidated live in this turn.

## Authoritative Source Findings

Native surface paint/gesture/PixelStrokeCommand carries layer_id, expected_image_key and operation only. Its target traversal selects visible pixels through visible groups, but currently discards the document locked field entirely. This permits a native stroke preview on a locked layer until authoritative publication refuses it. A new native regression uses the existing language-neutral protection fixture (already shared with Rust/TS protection and independent oracle checks). Test source is authored before implementation; the focused red run has not yet been launched.

Native EngineCanvas's paint publication routes only editPixels (wgpu target around 6084/6096); no editMask path is present. Native RasterHost marquee_hits_json selects layer bounds, not intrinsic pixel coverage. PixelStrokeCommand carries no pixel-selection coverage. React PixelEditingOverlay separately implements mask target, rectangle/ellipse/lasso/wand, selection combination, fill and algorithms. Native interaction parity is therefore incomplete even though shared algorithms and native rendering tests pass.

The native document sync also replaces the parsed document without cancelling an existing gesture. A transform/lock/image revision change during a stroke needs exact gesture invalidation, while identical scene updates must preserve the gesture. This requires a test and source revision witness, not cancelling every sync.

## Required Follow-Through

First verify and fix native locked/ancestor-locked stroke admission with the existing schema. Then carry target choice, selection intent/coverage and algorithm actions through shared semantic tool configuration/controls and native publication. Reuse shared affine/selection/pixel implementations; avoid duplicating algorithms or creating a second document authority. Verify gesture cancellation on target revision changes and native mask painting/history. Complete live keyboard, saved-document, two-client and large-image acceptance remains required.

The focused native protection red run is launched through the existing framework-surface-rs:test target with filter paint_stroke and a ticket-scoped output directory (raster-native-protection-red-1.log). Production native lock handling is not changed until this regression executes.

## Current Run State

Focused protection red run 79604 is confirmed live. Its TypeScript prerequisite passed one test; native test execution has not yet completed. Full Raster rerun 75362 is also confirmed live (latest progress over 530 seconds). Activation 18283 was revalidated live. Do not restart any of these from elapsed time alone. No native protection production fix has yet been applied. The export timing statements remain intentionally temporary and require removal after diagnosis.

Exact native publication evidence: EngineCanvas's wgpu paint route writes selection:null and only editPixels. RasterHost's PixelGesture::target checks visible pixels/groups but no locks; parse_layer currently lacks a retained locked field. The regression exercises locked-pixel, editable-pixel, inherited-pixel and outside from the existing canonical protection fixture. Native document synchronization does not currently compare the captured gesture frame/revision.

## Gesture Admission and Revision Implementation

Focused native red run 79604 executed three tests: one passed, two failed, 261 filtered. It confirmed both defects: locked-pixel admits a gesture; a lock update leaves an active gesture alive. Added a neutral stroke-revision fixture covering unchanged/rename versus lock/hide/move/resize/image replacement/deletion. Native tests use the same fixture as TypeScript; TypeScript updates are independently applied by fast-json-patch.

The native projection now retains pixel/group locked fields. Target traversal refuses locked pixels and locked ancestor branches. A captured gesture compares selected id, intrinsic-to-world matrix and image key against the current visible/unlocked target. Document/asset/selection updates cancel only invalidated gestures; identical refreshes preserve them. Utility changes cancel the gesture. Revision checks borrow image keys without cloning per scene refresh. Native paint green run 93882 is running; results remain pending.

React uses pixelGestureRevision to invalidate the captured gesture and asynchronous selection work, including visibility changes previously omitted from the dependency list. Tool dimensions retain their separate id/extent dependencies. Pure model red run 82212 failed on the missing new export (109 pass/1 fail/1 error); green run 72046 passed strict source checking and **146 tests**, zero failures. Initial red attempt 72791 failed before tests because pixels:test is not an existing target; the correct existing target is pixels:test-typescript.

Mounted UI run 75604 is active with eight shared fixture cases. It must prove unchanged and renamed targets actually publish a stroke while changed targets do not. This is not browser/live runtime evidence. Full Raster run 75362 remains live, unrelated export diagnostics still pending.

Shared revision cases now also cover cleared selection, tool switches, the same tool and adding a secondary selected layer without changing the target. Pure-model green run 2 (82434) passed strict source checks and **150 tests**, zero failures. Mounted run 75604 and native paint run 93882 remain live; if mounted execution captured the earlier eight-case fixture, rerun for the current twelve cases (expected suite total 47). Native full Raster run 75362 is still compiling, so the export logs remain pending. No temporary export logging was removed before collecting its evidence.

## Native Verification

Native paint green run 93882 passed **68 tests, 196 filtered**, exit 0. This verifies lock/ancestor-lock refusal and revision invalidation without changing composited pixels. Mounted run 75604 passed **43 tests**, including the initial eight revision cases; the four later session cases require the live follow-up 44843 (expected 47). Pure TS model/source checking passed 150 tests. Temporary export instrumentation was removed after the full 321-test Raster run passed; an uninstrumented focused export run is active. Native mask painting, pixel selections and filter controls remain open.

## Shared Mask Stroke Contract

Mounted follow-up 44843 completed with 47/47 tests passing, exit 0. The twelve shared revision cases now have mounted evidence. Uninstrumented export run 69035 remains live.

Mask painting uses the existing canonical editMask command schema: layerId, exact expectedMask descriptor, alphaStroke operation and optional selection. No second authoritative editing API is introduced. New neutral mask-stroke fixtures cover linked and unlinked pixel/group owners, asset intrinsic extents, blank masks, disabled/inverted masks, erasure, and descriptor/owner/ancestor revision invalidation. The Rust intent tests were added before production changes; focused red run 39760 is pending. TypeScript uses the same fixture with independent Three.js matrix inversion and fast-json-patch revision application.

The native projection currently loses mask transform, linking and asset identity. It must retain the complete descriptor. Finished intent must route to editMask with exact revision bytes while preserving bounded queue admission and retry retention. Target choice still needs shared config/scene/controls wiring; native pixel selection and algorithm controls remain open.

## Native Mask Intent Implementation

Red run 39760 completed with the expected missing set_paint_target, PaintTarget, set_mask_value and take_paint_edit compiler diagnostics (14 errors). Its prerequisite passed one test. Pure model mask fixture run 10973 passed strict source checks and 170 tests. Admission assertions were then added to the same 14 revision cases and a follow-up TS run was launched.

Native mask projection now retains the full descriptor plus authored fallback extents. This preserves the exact expectedMask including enabled/invert/link flags and null dimensions, independently of resolved pixel display extents. PaintGesture supports pixel and group masks in linked or parent coordinates, and publishes alphaStroke with mask value or zero for erasing. Target/descriptor changes invalidate gestures; unchanged targets preserve them. PaintStrokeCommand replaces the pixel-only command without a compatibility alias. The native queue route uses its action and revision field/value, preserving reserve-before-publication and consume-after-success. Full paint green run is pending.

Shared config and scene fields are not yet connected to the new target/value setters. Thus this implementation is foundational native intent support, not end-user native mask-completion evidence. Browser acceptance remains blocked by the previously recorded navigation policy restriction.

## Shared Controls and Scene Wiring

Configuration schema now requires paintTarget (pixels/mask) and integer maskValue (0–255). Neutral config vectors failed first in Raster TS red run 91621 (137 pass, 8 fail), then green run 34207 passed 145 tests. Added Rust config mutations, typed semantic commands, action decoding/registration, retained route metadata, command enumeration and EN/DE controls for both brush and eraser. Shared Paint2dScene carries the fields; native sync caches and applies both. React overlay uses controlled scene properties and publishes setPaintTarget/setMaskValue instead of private local settings. Mounted tests use a semantic config harness to republish these properties. Required scene fixtures and authored stories were updated without compatibility defaults.

Native PaintGesture preview consumers required parent-module visibility on command, world and points; these are now pub(super). An unsolicited peer report independently identified the same compile issue; no cross-chat reply was sent. Native mask preview uses grayscale mask coverage.

Uninstrumented export 69035 terminated before tests on a changed SVG serializer return type: Raster wrapped Result<String,String> inside Ok. Raster now directly propagates write_svg_xml's result. This is not an export test pass. Full Raster run 63331 verifies this integration plus config/command/scene changes; mounted run 88851 and native queue-publication run 20371 are active. Native paint 88342 remains active. TS mask/admission follow-up 57824 passed 170 tests; a seventh placement case (blank mask fallback versus owner image extent) was added afterward and needs the next run.

## Verification Follow-Up

Mask placement/model run 76880 passed 171 tests including the seventh authored-fallback case. Mounted config run 88851 completed 36 pass/11 fail: mask tests still counted setPaintTarget as the first document edit, protected-layer tests disallowed this legitimate local config action, and a broad assertion insertion accidentally added a mask-target assertion to two keyboard tests. The test harness now explicitly verifies each target config action before clearing that call for subsequent document-edit assertions; the unrelated keyboard assertions were removed. No production behavior was weakened to satisfy these tests. Mounted follow-up is running.

Native command census expectations were extended from 23 to 25 commands (24 config/document retained routes plus export), including action-decoding, wire keywords, config publication contracts and factory proof metadata. The new fields are also represented in the shared UI wire golden. Native runs remain pending.

Active verification was revalidated: paint 88342 (`raster-native-mask-stroke-green-1.log`), full Raster 63331 (`raster-mask-config-native-1.log`), WGPU bridge 20371 (`raster-mask-stroke-host-1.log`), mounted 64055 (`raster-mask-config-mounted-2.log`). None has a terminal result yet. Bridge uses the existing Nx exec route for Cargo nextest and is waiting on the shared artifact directory; no locks or peer processes were modified. The current turn made concrete implementation and verified TS progress; it is not a no-progress/blocked turn.

Mounted follow-up 64055 completed 45 pass/2 fail. Both remaining failures were stack overflows in jsdom/cssstyle's computed-style recursion while querying the disabled placeholder option by role (source line 92), rather than edit behavior. The test now resolves the labeled Layer combobox and inspects its native options collection for the placeholder text and disabled state. Mounted rerun 48839 is active. PaintGesture preview fields were re-read and confirmed pub(super); another unsolicited peer message refers to the preceding build checkpoint. No cross-chat reply was sent.

## Mounted Mask Controls Verified

Mounted run 48839 passed all 47 tests, exit 0. This verifies semantic setPaintTarget and setMaskValue republishing, mask erasure/fill, pixel/group affine coordinates, protected-layer refusal, selection cancellation and pending edit completion. It is mounted DOM evidence, not a live browser acceptance pass.

Full native Raster run 63331 terminated before tests because the schema-leaf RasterConfig fields paint_target and mask_value lacked mandatory #[state(config)] metadata. Both annotations are now present; full native rerun was launched as raster-mask-config-native-2.log. The shared-config Rust implementation was already annotated at the enclosing value/DSL level, but that does not replace schema-leaf field metadata. Paint and WGPU bridge verification remain pending.
