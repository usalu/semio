# Focus and Clipboard Retirement

## Boundary

The retained WGPU Interpreter has four input owners that can outlive a scene unless the scene or window close lane retires them explicitly:

- `FocusedTextEditor`, addressed by window, retained node, and exact runtime `host_id`;
- `FocusedInkEditor`, addressed by window generation, retained node, and exact runtime `host_id`;
- `FocusedInkSurface` plus each `InkClipboardStream`, addressed by window generation, retained node, exact runtime `host_id`, and the presented scene rectangle;
- each native Ink clipboard completion, which additionally needs an ABA-safe fixed-slot generation.

The wire `surfaceId` is document data and can be shared by sibling scene hosts. It is not a local input-owner key. The runtime `host_id` includes the exact window lifetime and a stable component-mount generation, so replacement cannot inherit focus or clipboard work and an unchanged mount does not change identity merely because candidate and presented trees alternate arenas.

## Observed gaps

`close_ui_document_one` currently retires scene intents, pointer capture, Canvas state, and the retained UI surface before it advances the UI close owner. It does not retire Text/Ink focus, semantic clipboard streams, or native semantic clipboard completions. The existing `closing_window_retires_an_unfinished_ink_clipboard_stream_before_reopening` law also focused the scene with its wire surface ID, so its live-address admission failed before it reached this gap. The fixture now reads the exact retained `scene.host_id`.

Native semantic clipboard completions currently capture only their fixed slot index. Window or component retirement must be able to free a slot, so a late callback can address a successor mounted at the same index. `complete_pending_native_ink_clipboard` now names this unchanged pre-repair behavior so the fail-first law can invoke the real callback seam.

## Fail-first laws

The registered native laws are:

- `ui_command_wiring_tests::closing_window_retires_an_unfinished_ink_clipboard_stream_before_reopening`;
- `ui_command_wiring_tests::closing_one_window_retires_only_its_ink_clipboard_owner`;
- `ui_command_wiring_tests::a_late_native_clipboard_callback_cannot_complete_a_reused_slot`.

The first law requires a full window close to retire the focused address and unfinished stream before reopening the same window. The second mounts streams for two distinct retained hosts and requires closing one window to preserve the sibling. The third removes and reuses a native completion slot, then delivers the old callback and requires the successor outcome to remain empty.

Renderer Native107 established all three laws as behavioral RED before the production repair. The root-owned run `renderer-native107-host-and-retirement` selected 32 renderer laws: 25 passed and 7 failed, with 1,286 outside the filter. The three failures above were independent and exact:

- the unfinished browser stream remained mounted after its window closed;
- closing the first of two windows left the first stream mounted instead of preserving only the sibling;
- the old native callback wrote through a reused fixed slot.

The run completed in 2.190 seconds of test time and 1 minute 35 seconds through Nx. Its log is `🗑️generated/astra-runtime/renderer-native107-host-and-retirement/run.log`.

## Repair design

The close lane advances one matching owner per opportunity in this order: Text focus, Ink editor focus, Ink surface focus, one browser clipboard stream, and one native clipboard completion. Matching is exact. A window close requires window ID plus window generation; a component retirement requires window ID, retained node, and `host_id`. No step clears unrelated ownership.

Native completion slots now retain a monotonic nonzero generation beside the optional mounted operation. Admission advances the generation and the callback captures `(slot, generation)`. Completion writes only when both still match. Retirement may therefore release a mounted slot immediately; a late callback becomes a benign stale token and cannot mutate a successor.

Full window retirement now invokes the matching-owner step before `Ui::close_surface_one`. Component removal is already represented by an exact `ScenePointerTarget`; the Interpreter drains at most one corresponding focus/clipboard owner before the next document render opportunity. The existing Scenes retirement remains the source of scene-state cleanup. This Interpreter owner only drains focus, streams, and native completion slots.

`with_live_ink_surface`, focused Text/Ink key handling, and focused Ink blur/copy/paste now reject `scenes::scene_host_retiring(host_id)`. This makes component removal immediately non-interactive while the bounded cleanup cursor finishes. The retirement step calls that predicate only outside a Scenes retirement borrow.

## Neutral contract

The Ink clipboard fixture and its strict JSON Schema now state exact-live-address completion, close-and-retire, old-completion rejection, bounded independent admission, matching-only retirement, one-owner-per-opportunity progress, and generation-token slot reuse. Its ownership address uses `hostId` rather than wire `surfaceId`.

The mounted React clipboard suite remains the independent DOM oracle for clipboard ownership; it does not establish native worker-slot ABA behavior. The scoped command was:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-react:test-long '../../../../🧪️tests/🖋️ink-canvas-clipboard/🟦️.tsx' --silent=false --reporter=verbose --skip-nx-cache
```

It passed 1 file and 7 tests in 13.81 seconds of Vitest time; Nx completed in 15.2 seconds. Receipt: `🗑️generated/astra-runtime/react-ink-clipboard-retirement.log`.

## Validation boundary

Root owns Cargo and renderer gates. Renderer Native108 is the first post-repair compile/runtime gate and includes the three exact law names above. A full renderer census and an activated browser run remain necessary before any physical WGPU clipboard-lifetime acceptance claim.

Renderer Native108 (`renderer-native108-component-lifetime`) ran 38 selected renderer laws: 30 passed and 8 failed. All three clipboard retirement laws passed, as did the corrected presented-keyboard law. The remaining failures were Canvas lifetime/camera laws owned outside this packet.

The GREEN receipt exposed one narrower address omission on review: `FocusedTextEditor` still stores the stable host but not its window generation, and its close matcher compares only the window ID. A delayed close for an old window lifetime could therefore clear the reopened lifetime's Text focus. `stale_window_close_cannot_clear_a_successor_text_editor_focus` is now registered as a fail-first law; its production branch remains unchanged until the root-owned native gate records RED.

Review also found that every editor address retains the presented arena `NodeId`. An unchanged keyed editor keeps its stable `host_id` across acknowledgement, but the accepted candidate can represent it with a different arena address. Without an acknowledgement rebase, the next Text key clears focus, the next Ink key cancels the active edit, and clipboard completion refuses its otherwise-live target.

The three `presented_editor_` laws exercise Text focus, Ink edit focus, focused Ink clipboard, one active clipboard stream, and one generation-stamped pending native owner. They accept A, reconcile an unaccepted A,C topology, supersede and accept A,B, then stage A,C,B. Candidate construction must preserve every old address. Successful acknowledgement must rebase only exact same-host owners to B's new `NodeId`. Native execution is pending; no production rebase is claimed.

Native110 ran no tests. Its renderer compile stopped because the new fixture passed renderer `SurfaceKind` to the neutral scene encoder, whose schema boundary requires `ui_contract::SurfaceKind`. The two fixture encodes now pass the explicit contract variants. This is compile correction only and provides no behavioral receipt.

Renderer Native113 supplied the first valid cross-acknowledgement receipt. It ran 46 selected laws: 39 passed and 7 failed, with 1,285 outside the filter. All three `presented_editor_` laws failed at their post-acknowledgement assertions after proving the exact host survived and the arena node changed from index 3 to index 2. The Text owner, Ink editor owner, focused Ink clipboard address, active stream, and generation-stamped native pending owner all retained the old presented node. Test execution took 1.297 seconds and Nx completed in 3 minutes 52 seconds. Receipt: `🗑️generated/astra-runtime/renderer-native113-owner-integration/run.log`.

The production repair now prepares one fixed-size editor-address plan while the matching UI candidate is sealed. Candidate construction does not mutate any input owner. Preparation resolves the candidate node and its actual layout rectangle through `Ui::candidate_scene_geometry_for_presented_node`, validates the exact window generation, stable host, and scene kind, and records removal as an explicit empty candidate. The plan contains at most one Text focus, one Ink edit focus, one focused Ink clipboard address, 32 browser streams, and 32 native pending slots. Native pending entries retain their exact slot generation.

Only a successful `acknowledge_presented_input` commits the plan. Exact unchanged owners receive the accepted arena node; clipboard addresses also receive the accepted scene rectangle. Exact removed owners clear. A concurrently changed owner, slot generation, stream ID, address, hidden window, or nonparticipating window remains untouched. A refused acknowledgement discards the prepared plan without mutation.


## Native117 accepted editor receipt

Renderer Native117 ran 52 selected laws: 46 passed and 6 failed in 1.322 seconds; Nx completed in 3 minutes 15 seconds. All three `presented_editor_` laws passed. The bounded prepare/commit rebase therefore preserves Text focus, Ink focus, the focused Ink rectangle, fixed-slot clipboard streams, and generation-stamped pending native clipboard owners until matching pixel acknowledgement, then changes only entries whose exact old owner still matches. No editor-address failure remained in this gate.

