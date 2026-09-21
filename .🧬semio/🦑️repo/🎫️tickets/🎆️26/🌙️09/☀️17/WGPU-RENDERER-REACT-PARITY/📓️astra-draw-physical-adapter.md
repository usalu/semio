# Draw Physical Acceptance Adapter

The ticket canvas-interactions script previously returned discovery-only case labels. It now physically selects rectangle and pen utilities, drives trusted browser pointer/key events, waits for actual action receipts and live layer-count publication, and captures cropped Canvas2d pixels around commits, selection/modifiers, draft cancellation and double-click commit. It does not claim a physical pass before the freshly activated app executes it.

Selection acceptance no longer treats Canvas2d pixels or the engagement status suffix as a selection oracle. Draw does not project framework `InteractionView` selection into either renderer's Canvas2d scene, and its status text deliberately hard-codes `0 selected`. Those are shared app limitations.

The replacement is a real selection-consuming document command. The probe clicks the first created rectangle with Direct Select, physically types a unique value into `drawing-canvas-engagement`, submits Enter, and requires the matching visible `drawing-play-layers.shape.<id>` tree row to change. It then Shift-clicks the second rectangle and proves the same submit changes no row because `engagementSubmit` requires exactly one selected id. Finally it Control/Meta-clicks the second rectangle to subtract it and requires a second unique name to appear on the original first row only. No action or selection state is injected.

The neutral `drawSelection` contract is validated against `🔬️canvas-interactions/🧬️schema/🎯️draw-selection/🔣️.json`. It pins the replace/add/subtract order, physical engagement field, canonical search-pane toggle and stable row-id family. Screenshots remain evidence around each physical pointer phase without being interpreted as highlights.

The five original neutral fixture cases remain. The fixture/Chromium trusted-cancellation contract is being rerun; actual Draw app calibration, runtime console evidence and both-renderer acceptance remain pending.

Fixture and actual Chromium CDP contract passes: trusted pointerdown/pointermove/pointercancel share pointerId2, with no pointerup. Evidence: `🗑️generated/astra-runtime/canvas-interactions-contract-3.log`. This is probe transport validation only; no activated Draw result is inferred. The runner now preserves a failure screenshot, observation, console lines and error receipt before closing the browser.

Isolated TypeScript compilation passes using installed tsc via Bun Nx exec with Node/Bun types and bundler module resolution (`🗑️generated/astra-runtime/canvas-interactions-types-4.log`). The strict fixture/schema and Chromium trusted-cancellation contract also pass (`canvas-interactions-contract-4.log`). This validates the adapter and neutral contract; application runtime is still pending.

Terra's source trace for the semantic witness is `📓️terra-draw-and-surface-specimen-audit.md`. Fresh React and WGPU Draw listeners have not run this revised path, so no renderer acceptance claim is made here.

## Fresh Draw Reference Calibration

Canonical Draw React activation1 completed in 4m35s. An owned no-HMR listener runs at 6364; the existing listener6064 is preserved. The first physical run stops at the obsolete mount selector while its failure screenshot visibly paints the Draw scene.

Actual discovery4 proves the canvas exists, but its host identity follows Interpreter.surfaceHostIdentityV1: data-controller-id is the retained record id `1`, data-surface-id is `window:drawing-composite`, and the authored scene identity is the ancestor data-ui-node-key `drawing.play.composite`. The semantic action ledger independently names the canonical app controller `s.draw.drawing@1/*#editor`. The fixture now addresses the authored surface wrapper and owning window canvas, without assuming a stable numeric record id. It does not remove the canvas requirement or introduce a legacy alias.

Draw2 reaches that exact live surface and stops because Utilities is folded. The adapter now physically unfolds the known window utility rail, chooses the Drawing group, waits for the exact utility, and presses it. Draw3 reaches Rectangle but stops because the live layer-count status and Artifact rows are folded. Draw4 opens the actual Actions and Artifact controls before measuring creation. These remain adapter calibration results, not Draw renderer passes. Diagnostic observations now preserve canvas owners and visible control ids on failure.

## Two-Gesture Failure After Calibration

Draw5 through Draw10 consistently create the first rectangle and fail the second rectangle's exact layer-count increment. The current adapter physically re-arms Rectangle before each gesture, requires `aria-pressed=true`, and waits for the first one-shot reset to Direct Select before proceeding. The public React ledger records both complete gesture sequences as applied; it intentionally omits argument payloads.

Draw10 adds temporary diagnostics at the real Canvas2dHost dispatch boundary. Both Down/Up pairs use `shapeRect`, a 1587×907 logical viewport, all modifiers false and `cancelled:false`. The first drag is (396.75,317.45)→(603.06,453.5); the second is (825.24,317.45)→(1031.55,453.5). Both are inside the same exact mounted host and do not overlap. The failure screenshot contains only the first new Rectangle row and shape. This rules out a missing re-arm, wrong pointer coordinates or host cancellation for this receipt; it does not yet establish the downstream root cause.

Terra's bounded next law exercises the real retained DrawingInstanceOperationOwner across commit, new artifact revision/view, re-arm and second commit, requiring two create-layer mutations and two one-shot reset effects with exact retirement. Execution is queued with Sol. See `📓️terra-draw-layout-transfer-audit.md`. No Draw renderer acceptance is claimed. The temporary host diagnostic is to be removed after retaining this receipt.

## Retained Creation Repair and Expanded Physical Law

DrawNative2 reproduces the failure in the real application owner: two correct shape gestures complete, but their name-only identity collides and the second CreateLayer is rejected. The repair now derives an identity using admitted operation context and persisted geometry. The native law also requires a third creation at the second rectangle's exact geometry to be a separate layer. DrawNative3 passes one selected law, with 276 tests outside the filter; a fresh full Draw run must establish the final source state. See `📓️astra-sol-draw-retained-owner.md`.

The physical adapter now includes `repeated-geometry-creation` after the selection checks. It reuses the second rectangle's exact endpoints, requires one new visible layer row and a different identity, and records both screenshot hashes without requiring a pixel difference for coincident opaque shapes. Every gesture sequence now waits for settled React outcomes and requires all relevant actions to be applied. TypeScript validation13 passes. Contract13 was an invalid command invocation (`contract` instead of the supported `test`); contract14 reruns the actual fixture and trusted Chromium cancellation law. Fresh Draw app activation and physical acceptance remain required.
