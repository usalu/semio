# Astra Sol Scene Cancellation Contract

Phase 1 defines the shared cancellation law and records the current React reference boundary. It does not implement native scene cancellation and does not claim the root-owned pointer-cancel route is complete.

## Contract

Every active scene gesture is owned by the exact `(surfaceId, generation, pointerId)` tuple. A cancellation applies only when all three fields match. A stale generation or sibling surface is ignored. Replacing a generation cancels the old owner before admitting the new one, and an exact cancellation leaves the next Down admissible.

Cancellation retires pointer capture plus queued and in-progress intent. It preserves selection and camera values that were already published. Except for TiledMap navigation, it discards uncommitted geometry, brush, wire, marquee, and selection-drag state without routing through normal Up and without publishing a commit mutation.

The language-neutral source is:

- `🧬️schema/🛑️scene-pointer-cancellation/🔣️.json`
- `🧫️fixtures/🛑️scene-pointer-cancellation/🔣️.json`

| Family | Representative gesture | Terminal | Current React evidence |
| --- | --- | --- | --- |
| NodeGraph | wire drag | discard preview and queued mutation; preserve published selection/camera | unsupported: the graph pointer surface has Down/Move/Up and no cancel handler; the unrelated graph parameter Slider cancel is not a canvas oracle |
| Board2d | node or link drag | discard preview, Board intent, claim, and buffered mutation | partial: mounted React retires the contact and releases capture, but calls neither `pointerUpScreen` nor a Board session cancel primitive |
| Ink | pencil stroke | discard draft stroke/marquee and queued mutation | unsupported: no React cancel handler; `onPointerLeave` currently calls the committing Up handler and is not a cancel oracle |
| TiledMap | middle-button pan | commit navigation through the existing Up terminal, end continuous pan, clear marquee/capture, mirror camera | reference exact: the existing callback invokes `session.pointerUpScreen(lastPoint)` and performs every listed cleanup |
| Paint2d | brush, marquee, or navigator pan | discard brush/marquee/pan and queued mutation | unsupported: no React cancel handler; normal Up commits marquee or brush work |
| TextEditor | selection drag | discard drag preview and queued mutation | unsupported: no React cancel handler; normal Up ends selection through the editor session |

The four unsupported rows are contract gaps, not passing React parity claims. Phase 1 deliberately avoids fabricating `pointercancel` behavior with `pointerup` or asserting that missing handlers discard edits.

## Oracles

`🧪️tests/🛑️scene-pointer-cancellation/🟦️.ts` validates the six-case fixture with Ajv, pins the single TiledMap exception, proves exact owner/generation/pointer matching, and scans each actual interaction surface so a Slider-level cancel cannot be mistaken for NodeGraph canvas support.

The mounted Board law dispatches genuine DOM `pointerdown`, window `pointercancel`, and a second `pointerdown`. It proves the present React handler emits no Up, calls no partial `cancelAreaSelect`, and admits the second contact. This is intentionally classified `contact-only` because the Board session's drag remains uncancelled.

The TiledMap law extracts its actual production callback through the TypeScript compiler and invokes it with a real event target. It proves exactly one `pointerUpScreen(lastX,lastY)`, both pressed flags cleared, pan ended, marquee reset, capture released, camera mirrored, and the demand renderer returned to idle.

NodeGraph, Ink, Paint2d, and TextEditor have no callable React cancellation producer today. Their phase-2 mounted laws must start red against real host/session stubs and turn green only when those hosts expose explicit noncommitting terminals.

## Production Implementation Plan

### Shared ingress and intent ownership

1. Preserve pointer identity instead of dropping it at `winit-app`: change renderer `handle_pointer_cancel()` to accept the `PointerInfo`/pointer id from `DispatchEvent::PointerCancel`, pass it to `ShellState::handle_pointer_cancel`, and replace the unaddressed `UiEvent::PointerCancel` with an addressed cancel event.
2. Add `pointer_id` to pointer variants in `SceneIntentEvent` and to the active gesture witness. `SceneInteractionIntent` already owns `window_id`, `surface_id`, its monotonic `generation`, and `tree_revision`; cancellation must match surface, generation, and pointer before marking matching queued/current records retiring.
3. Add a bounded `SceneIntentQueue::cancel_pointer_step(owner)` that retires at most one matching queued or in-progress record per grant and calls each retained job's cancel/close path before release. It must not translate Cancel to `PointerUp`.
4. Store the active scene owner when Down is accepted and clear it only after exact Up/Cancel or generation replacement. A late cancel for the previous generation cannot touch a reopened same-id surface.

### Family-owned terminals

- **NodeGraph:** add noncommitting `pointer_cancel_screen` APIs to both Flow and Dag hosts; expose them through `NodeGraphEngine` and `engine_canvas::node_graph_pointer_cancel_into`. Clear node/wire/slider drag previews and pending edit signature without serializing or committing the fixture.
- **Board2d:** add one BoardHost `pointer_cancel_screen` terminal covering node drag, link drag, brush, region, transform, and selection. It must cancel the retained pointer authority, restore/discard transient domain state, then let EngineCanvas release `board_pointer_controller_id` and its action claim. `cancel_area_select` remains an internal branch and cannot be the public all-gesture terminal.
- **Ink:** add `InkInteractionEvent::PointerCancel` plus `scenes::ink_pointer_cancel_into`. Cancel matching `InkInteractionJob`s before Publish, clear `drag`, `pointer_was_down`, marquee, pending live events, and optimistic `ink_overrides`, and emit neither selection nor `inkApplyEvents` commit. React needs a dedicated handler; `onPointerLeave={handlePointerUp}` must not serve as cancellation.
- **TiledMap:** route exact cancellation to existing `tiled_map_pointer_up_into` using the retained last point, then clear `SceneDragMode` and publish/mirror the camera exactly once. Primary marquee state is discarded by the existing React cancel callback; only navigation uses the Up terminal.
- **Paint2d:** add `engine_canvas::paint2d_pointer_cancel_into` to reset `Paint2dMarquee` tracking/points and navigator `pan_last`. Add a RasterHost brush cancel primitive that drops brush preview without normal Up's selection/brush commit.
- **TextEditor:** add EditorHost `pointer_cancel_screen` to clear `drag_selecting` while preserving the last published selection, expose it through `engine_canvas::text_editor_pointer_cancel_into`, and cancel matching queued Interpreter intent before it can publish after the gesture owner has retired.

### Phase-2 laws

For every family: Down → Move → Cancel must produce no forbidden commit, leave no queued/in-progress authority or capture, reject a late completion from the cancelled generation, ignore sibling/stale cancellation, and accept a new Down. NodeGraph covers wire and node drag; Board covers node and link drag; Ink covers stroke and marquee; TiledMap covers middle pan and primary marquee; Paint2d covers brush, marquee, and navigator pan; TextEditor covers drag selection.

## Validation

- Focused React/neutral scope: **3 files, 33/33 tests passed**.
- Included the neutral contract, mounted Board contact oracle, and production TiledMap callback oracle.
- The first focused run exposed an over-broad NodeGraph source probe because a graph parameter Slider has its own `onPointerCancel`; the corrected probe scopes itself to the graph canvas pointer surface. The second run passed.
- Board mounting emits existing jsdom `HTMLCanvasElement.getContext` not-implemented diagnostics from a sibling trace component and a Three.js duplicate-instance warning; the suite still passed and the cancellation assertions executed.
- No Cargo, native build, WGPU activation, generated app artifact, or production Rust edit was run.
