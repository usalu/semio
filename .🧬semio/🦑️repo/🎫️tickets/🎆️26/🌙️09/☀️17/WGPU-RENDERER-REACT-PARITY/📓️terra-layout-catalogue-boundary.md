# Layout Catalogue Boundary Review

Read-only source review on 2026-09-20. No production edit, build, test, or activation was run.

## Verified boundary

The retained renderer already exposes the browser-safe catalogue contract without Layout knowledge.

- 🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:496-529 retains only the keys of drag data during drag-over and passes the sorted MIME list to Scenes. It never reads a value.
- 🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1609-1658 admits the base catalogue MIME and emits canvasDragOver with surfaceId, x, y, width, height, and types.
- Scenes/🦀️.rs:1703-1733 emits canvasDrop with the raw base-MIME value as dragData, only after bounds and byte-capacity checks.

Layout's catalogue now supplies the missing declarative witness. Its roster is exactly page, rect, text, and image (✏️editor/📌️panels/🛍️catalogue/🦀️.rs:23-26). For every row it writes both application/x-semio-catalogue-item with JSON {kind:…} and an empty application/x-semio-catalogue-kind.{kind} MIME entry (46-55). The witness survives the browser's protected drag-over data store because it is a type, while the JSON remains available only at drop.

The mismatch is confined to Layout's app bridge. LayoutPlayApp::command_from_action still aliases an absent drop_kind to required kind for both actions (✏️editor/🦀️.rs:495,502). The renderer emits neither key. The retained work correctly owns the state after a typed command is admitted: it projects the pointer through the Blueprint camera and stores the transient preview at 606-614, then clears that transient on leave/drop and changes active page for a page drop at 616-626.

## Small owned repair packet

Keep CanvasDragOver and CanvasDrop as semantic typed commands with their existing kind; do not make their command codec carry browser raw data. Replace the two obsolete drop_kind aliases in the Layout action bridge with two Layout-owned adapters:

1. **Drag-over adapter.** Read only types; require the base catalogue MIME and exactly one allowed application/x-semio-catalogue-kind.{kind} witness from the four-item catalogue roster. It synthesizes the existing typed kind for CanvasDragOver. It must reject a missing, unknown, or ambiguous kind witness. This makes the existing typed transient preview exact without getData during drag-over or a renderer domain parser.
2. **Drop adapter.** Read only dragData; strictly parse the Layout-owned JSON into the same four allowed kinds before it synthesizes the typed kind for CanvasDrop. Malformed JSON, a non-object, a missing/non-string kind, and every other kind must be rejected before addPage or addFrame.
3. **Direct typed-command defense.** canvas_drop::handle currently treats every non-page string as a frame kind (🎮️commands/📥️canvas-drop/🦀️.rs:63-70). Its match must also reject any direct typed command outside the same allow-list. The bridge is the real input boundary; this closes the second route to an arbitrary frame kind.

The app should publish the roster and two MIME constants from one app-owned source rather than duplicating the strings in args_bridge; they are currently private constants in the catalogue panel. The renderer stays unchanged and simply transports generic MIME names and raw drop data.

## Terminal lifecycle detail

A valid drop reaches retained work and clears LayoutWindowTransient.drop_preview at ✏️editor/🦀️.rs:616-626. A malformed raw drop rejected in command_from_action cannot start that work, so it cannot clear the previously admitted transient by that route. The repair therefore needs an explicit terminal policy for a refused drop: either dispatch a retained clear-only input action before returning the decode fault, or have the generic drop boundary emit its existing leave action before the potentially rejected drop. This is a real source consequence, not evidence that the current happy path fails.

## Required proof changes

- Extend the Layout bridge test at ✏️editor/🧪️tests/🔬️unit/🦀️.rs:441-476 with renderer-shaped types and dragData, including absent, ambiguous, and unknown MIME witnesses plus malformed and unknown JSON.
- Update only the direct typed fixtures that name the semantic kind: ✏️editor/🧪️tests/🔬️unit/🦀️.rs:145,152 and 🎮️commands/👇️canvas-pointer-down/🧪️tests/🔬️unit/🦀️.rs:183-212. The latter must continue to prove the retained preview appears and leave clears it.
- Add an end-to-end neutral fixture/law that carries the renderer's actual action shapes: a drag-over types list containing both catalogue MIME entries and a drop dragData JSON value. It must prove the selected kind reaches the typed command and mutates only for the four-roster payloads.
- Keep the actual React dispatch oracle requested for the producer/runtime boundary; this audit did not run it.

Confidence: high for the boundary and small bridge packet; high for the malformed-drop transient distinction.

## Terminal Lifecycle And Retained-Path Follow-up

Read-only source review on 2026-09-20. No production edit, build, test, or activation was run.

### Producer fact and decision

The current React drop handler reads the raw MIME value and returns before clearing local state or sending an action when it is empty; for a nonempty value it sends only `canvasDrop` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🟦️.tsx:902-913`). The current WGPU route sends `canvasDragLeave` for an empty/out-of-bounds payload (`🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1703-1716`) but sends only `canvasDrop` for a nonempty payload (`1717-1733`). Thus malformed nonempty JSON currently reaches the strict app bridge without a terminal retained cleanup.

The correct generic producer contract is **leave before drop**. Every drop emits `canvasDragLeave`; a nonempty raw value then emits the unchanged raw `canvasDrop`. Empty payloads emit leave only. This retires the preview before strict decoding, while the malformed raw value is still sent to—and refused by—the app decoder. It must not be converted to a successful leave-only semantic command. A valid Layout drop clearing the preview again is idempotent: Layout's retained work clears for both commands, and only the second command applies page/frame consequences (`✏️editor/🦀️.rs:616-626`).

### WGPU ordering proof and bounded caveat

`write_scene_action_batch` reserves all action and byte credits before staging its supplied slice (`🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1594-1607`). The bounded action queue publishes and pops the staged actions in index/FIFO order (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎬️action/🦀️.rs:928-946,980-999`). The frame transaction inserts each into `FrameActionOwners` in that same order (`🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12868-12892`); the deferred cursor pops FIFO (`9675-9677`) and awaits `dispatch_gesture_action` before it returns the cursor for the following action (`10132-10150`). A leave-first batch therefore cannot supersede a valid drop: the guest completes leave before it starts drop.

There is one bounded-transfer limitation worth testing before describing the pair as atomic beyond the UI queue. `FrameActionOwners` has 256 independent slots (`9591-9617`), and the transaction calls `try_push` separately for every input action (`12887`). At 255 occupied slots, leave can enter and the following drop can fault with `frame input action credits exceeded`. The pair is atomic at action-queue admission, but not at this later transfer. This is an existing generic action-transfer limitation; the terminal packet should add a frame-owner-at-255 law for leave/drop and either establish a paired transfer guarantee or scope the contract to admitted frame actions.

### React ordering requirement

`Canvas2dHost`'s dispatch passes through the `onAction` promise (`📐️Canvas2dHost/🟦️.tsx:770-780`). `ShellHost` allocates each input its own causal ledger entry (`🏛️ShellHost/🟦️.tsx:6617-6635`) and resolves that promise only after the guest operation settles (`7092-7113`). Two unawaited calls are separate inputs and do not supply a component-level happens-before guarantee. The React producer must await the leave dispatch before invoking `canvasDrop`; the raw drop must be invoked in a `finally` path so a leave refusal does not suppress a valid mutation. The actor bridge will retain those sequential calls (`🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts:310-344`), but that does not remove the producer's responsibility to serialize the two operations.

Required focused laws: valid React/WGPU action order is drag-over, leave, drop; empty drop ends with leave only; malformed nonempty JSON is leave then the unchanged raw drop and the app decoder refuses it. The existing WGPU valid-drop shell law must update from drag-over/drop to this three-action sequence. The React contract needs a controlled promise test, not just a synchronous action-array assertion, to prove drop has not begun before leave settles.

### New Retained Layout Test Review

The new neutral-envelope test correctly refuses every fixture row whose expected kind is null and checks all valid renderer-shaped coordinates (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:628-646`). The new retained test drives the real registry and settle path for page, rect, text, and image; it proves left-window preview locality, right-window isolation, terminal preview retirement, one document consequence, and the default-camera conversion from `(24, 36)` in a `100 × 100` viewport to `(-26, -14)` (`649-696`, with the production conversion at `✏️editor/🦀️.rs:606-613`). Those assertions cover the intended camera/world/page frame geometry.

One concrete page-semantic assertion is absent: after the page case, the test checks only the page count (`684-690`), although retained work sets the addressed Blueprint window's `active_page_id` to the new `page-{count}` (`✏️editor/🦀️.rs:620-625`). Add an assertion that the left configuration selects the created page and that the right configuration remains unchanged. This would catch a page creation whose document consequence succeeds but whose per-window navigation consequence is lost.

Confidence: high for leave-first ordering, raw-fault preservation, and the retained-test gap; high for the identified downstream frame-owner capacity limitation.
