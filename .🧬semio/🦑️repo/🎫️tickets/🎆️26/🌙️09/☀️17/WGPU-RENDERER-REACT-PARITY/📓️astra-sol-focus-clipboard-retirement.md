# Focus and Clipboard Retirement

## Boundary

The retained WGPU Interpreter has four input owners that can outlive a scene unless the scene or window close lane retires them explicitly:

- `FocusedTextEditor`, addressed by window, retained node, and exact runtime `host_id`;
- `FocusedInkEditor`, addressed by window generation, retained node, and exact runtime `host_id`;
- `FocusedInkSurface` plus each `InkClipboardStream`, addressed by window generation, retained node, exact runtime `host_id`, and the presented scene rectangle;
- each native Ink clipboard completion, which additionally needs an ABA-safe fixed-slot generation.

The wire `surfaceId` is document data and can be shared by sibling scene hosts. It is not a local input-owner key. The runtime `host_id` includes window generation, arena identity, and component generation, so replacement cannot inherit focus or clipboard work.

## Observed gaps

`close_ui_document_one` currently retires scene intents, pointer capture, Canvas state, and the retained UI surface before it advances the UI close owner. It does not retire Text/Ink focus, semantic clipboard streams, or native semantic clipboard completions. The existing `closing_window_retires_an_unfinished_ink_clipboard_stream_before_reopening` law also focused the scene with its wire surface ID, so its live-address admission failed before it reached this gap. The fixture now reads the exact retained `scene.host_id`.

Native semantic clipboard completions currently capture only their fixed slot index. Window or component retirement must be able to free a slot, so a late callback can address a successor mounted at the same index. `complete_pending_native_ink_clipboard` now names this unchanged pre-repair behavior so the fail-first law can invoke the real callback seam.

## Fail-first laws

The registered native laws are:

- `ui_command_wiring_tests::closing_window_retires_an_unfinished_ink_clipboard_stream_before_reopening`;
- `ui_command_wiring_tests::closing_one_window_retires_only_its_ink_clipboard_owner`;
- `ui_command_wiring_tests::a_late_native_clipboard_callback_cannot_complete_a_reused_slot`.

The first law requires a full window close to retire the focused address and unfinished stream before reopening the same window. The second mounts streams for two distinct retained hosts and requires closing one window to preserve the sibling. The third removes and reuses a native completion slot, then delivers the old callback and requires the successor outcome to remain empty.

No behavioral native receipt is claimed yet. Renderer Native106 is the root-owned fail-first run.

## Repair design

The close lane will advance one matching owner per opportunity in this order: Text focus, Ink editor focus, Ink surface focus, one browser clipboard stream, and one native clipboard completion. Matching is exact. A window close requires window ID plus window generation; a component retirement requires window ID, retained node, and `host_id`. No step scans or clears unrelated ownership.

Native completion slots retain a monotonic nonzero generation beside the optional mounted operation. Admission advances the generation and the callback captures `(slot, generation)`. Completion writes only when both still match. Retirement may therefore release a mounted slot immediately; a late callback becomes a benign stale token and cannot mutate a successor.

Full window retirement invokes the matching-owner step before `Ui::close_surface_one`. Component removal is already represented by an exact `ScenePointerTarget`; the Interpreter retains at most one corresponding focus/clipboard retirement owner and drains it incrementally before the next document render opportunity. The existing Scenes retirement remains the source of scene-state cleanup. This Interpreter owner only drains focus, streams, and native completion slots.

`with_live_ink_surface`, focused Text/Ink key handling, and focused Ink blur/copy/paste also reject `scenes::scene_host_retiring(host_id)`. This makes component removal immediately non-interactive while the bounded cleanup cursor finishes. The retirement step itself must not call that predicate while the Scenes retirement owner is borrowed.

## Neutral contract

The existing Ink clipboard fixture already states exact-live-address completion, close-and-retire, old-completion rejection, and bounded independent admission. Its ownership address must use `hostId` rather than wire `surfaceId`, and the schema will name matching-only retirement, one-owner-per-opportunity progress, and generation-token slot reuse. The existing React mounted clipboard suite remains the third-party DOM oracle for clipboard ownership; it does not establish native worker-slot ABA behavior.

## Validation boundary

Root owns Cargo and renderer gates. After the actual RED receipt, the exact focused native filter should include the three law names above. A full renderer census and an activated browser run remain necessary before any physical WGPU clipboard-lifetime acceptance claim.
