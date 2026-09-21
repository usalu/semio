# Close Interaction Checkout Wake Audit

## Evidence

The physical failure was recorded in [window-close-interaction-checkout-fault.json](🗑️generated/astra-runtime/checkpoint19-iab/window-close-interaction-checkout-fault.json): the close click at 17:42:40.805 was followed by a 17:42:41.417 watchdog quarantine in Render, before any GPU cursor existed:

```
phase=Render engine=0 upload=1 gpu-cursor=None
upload-progress=(0,0,0) input-wait=InteractionCheckout
```

That identifies the pre-submit input authority gate. It does not establish a GPU submission failure, a component-close owner leak, or an unacknowledged input publication.

## Confirmed owner route

A discrete browser event checks out the only `AppInteractionState` under `dispatch-event`, reserves one completion, then awaits `dispatch_normalized_event`:

- [renderer.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:10481) performs the checkout, reservation, and spawn.
- [renderer.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12839) has the wasm `spawn_local` completion, which always returns the interaction as `ResumeDispatch`.
- [renderer.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:12879) lets the presenter restore that returned interaction without consuming the remaining cursor.

Render must retain its candidate while this happens. At [renderer.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:15417), `InteractionCheckout` returns `AwaitingRuntime`; only a returned interaction permits candidate progress. Aborting it on this state would violate the presented-input boundary. The candidate has not reached GPU submit or input acknowledgement on this branch.

The old close path was capable of holding that checkout across the guest journal: the close mutation is synchronous ([Shell.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15887)), but the former close arm awaited `note_control_command`, whose normal tail awaits `dispatch_action` ([Shell.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:20265)). A guest action can therefore delay the return the current present candidate needs.

## Current repair review

The current close arm at [Shell.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13839) now does exactly the required synchronous close followed by `arm_control_command`. That helper queues `noteShellCommand` in `deferred_actions` without awaiting it ([Shell.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:20276)). The existing close law explicitly requires this separation and its reason ([wgpu-dock-close-reopen-journals.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🛰️wgpu-dock-close-reopen-journals/🦀️.rs:86)).

This is the narrow coherent repair. The initial dispatch can complete, the wasm completion wakes the host, and Render restores the interaction before candidate progress. The journal still runs through the existing bounded deferred lane ([Shell.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:14118)), preserving one `shell.windowClose` record with the closed window.

No additional repeated-frame invalidation is the correct remedy. A normal returned completion calls the mailbox finish/waker, and browser ticks retain `continue_frame` while presentation is pending. The prior artifact predates confirmation of this source repair; I did not rerun the physical journey.

## Component-close fairness

The external component-close bridge deliberately refuses first admission while the presenter or frame build owns a live frame ([os-host.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs:694)). This prevents it from detaching resources needed by the pending candidate. It does not own `AppInteractionState`, so it is not a source-level cycle with the close checkout. After the repaired dispatch returns and the candidate reaches a terminal presenter state, the normal redraw progression may admit the close request.

## Required regression

Add one actual renderer law rather than another source-string test:

1. Build a real presented input candidate containing a dock-close control and arrange a guest journal action to remain pending only after the close event returns.
2. Dispatch the real close control through `RuntimeApply::DispatchEvents`.
3. Assert the close changes dock state and leaves exactly one deferred `noteShellCommand(shell.windowClose, closedWindow)`.
4. Before releasing the deferred journal, drive the real presenter: it must restore `ResumeDispatch`, advance/submit the candidate, and never report `InteractionCheckout`, acknowledge input early, or fault.
5. Release the deferred action and assert one journal record. Then allow the component-close bridge to retire only after the presenter has no pending candidate.

This covers the observed ordering and preserves the last completed frame while the new candidate is waiting. It does not claim the recorded watchdog was exclusively the journal await; the artifact lacks checkout-site telemetry. Confidence that the queued-close repair removes the demonstrated source hazard is high; attribution of the historical stall to that hazard is medium.



## Separate remaining close path

The `mod+shift+w` path reaches `ShellShortcut::CloseWindow` and `close_active_window` ([Shell.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:16130), [Shell.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15944)). It closes synchronously and therefore does **not** recreate this checkout stall, but it currently queues no `shell.windowClose` journal entry.

Unify cap and keyboard under a small user-close wrapper: invoke `close_dock_window(window_id)`, then queue the same `noteShellCommand(shell.windowClose, { windowId })` exactly once. Keep `close_dock_window` itself pure, since non-user callers should not acquire a history record. Add a keyboard twin of the existing close-journal law. This is a reachable parity gap, not evidence for the recorded Render stall.



The timing boundary is additionally enforced by the host: it does not pump pending runtime applies while Render/CloseGpu/Acknowledge holds the presented input publication ([winit-app.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs:240)). Thus the newly queued journal cannot begin before the current candidate is terminal; it is only created as a later `pending_frame_deferred` owner at frame finish ([renderer.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:16333)).



React confirms the keyboard path is the same user-close authority: `useControlKeybinding("ui.window.close")` calls `closeWindow(activeWindowId)` ([Canvas.tsx](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1814)), and `closeWindow` calls `onWindowClose` before changing layout ([Canvas.tsx](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1475)). The keyboard journal gap is therefore confirmed parity work, not merely an inferred convention.

