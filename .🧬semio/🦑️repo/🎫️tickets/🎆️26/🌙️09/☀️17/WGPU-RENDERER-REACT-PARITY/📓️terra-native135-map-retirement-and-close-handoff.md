# Native135 Map Retirement and Deferred-Close Handoff Audit

## Scope and evidence

Read-only audit of Native135's only Map failure:

- [Native135 receipt](./🗑️generated/astra-runtime/renderer-native135-full/failures.json) records `retained_map_key_replacement_and_removal_retire_the_old_gesture_without_input` failing at the post-next-frame old-gesture assertion.
- [Native135 log](./🗑️generated/astra-runtime/renderer-native135-full/run.log) shows the replacement document is repeatedly and correctly refused with `InterruptedClose` while its baseline is incrementally released. That refusal is retryable by design; it is not the cause by itself.
- The fixture uses the real retained document, actual paint, hit registration, seal, and acknowledgement path in [the EngineCanvas test](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:180) and [its replacement law](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:879). It is not a direct-paint helper that bypasses presentation.

No test or build was run for this audit.

## Confirmed causal cycle

This is a production lifecycle defect, with a bounded production repair. It is not a request to synchronously dispose Map state at acknowledgement.

1. A changed retained Map key correctly creates a new component host and a retirement record for the old host. The reconciliation path compares the presented key, kind, and wire surface, assigns a new host on mismatch, and calls the retirement callback in [UI reconciliation](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:1057) through [the retirement construction](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:1096). The same-key refresh takes the identity-preserving branch, so it should continue to preserve the drag.

2. Because the old host is still in the presented tree before acknowledgement, the retirement is intentionally staged as a candidate retirement. Once the candidate is accepted, it is moved into the window's `scene_retirements` queue by [`acknowledge_presented_input`](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1182).

3. The next `render_ui_document_step` first advances ingress and reconciliation. In reconciliation, the loop deliberately breaks as soon as `scene_retirements` is nonempty at [UI engine line 1743](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1729). Yet the only Interpreter consumer of that queue is after the complete frame state-machine at [Interpreter lines 2805–2817](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2805). Therefore the queue prevents the render from reaching its own consumer. The old `SCENE_STATE` entry, including `MapMarquee` drag state, stays active.

4. This explains the exact observed split: acknowledgement immediately makes the old public pointer target non-live, because [`scene_pointer_target_is_live`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:706) compares the presented key, host, mount generation, and node. The fixture's line 900 passes. But the internal Map state still exists, so [`tiled_map_drag_active`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:8536) remains true at line 903.

The state does not need an eager mutation. [`retire_scene_identity`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:1366) atomically moves the exact component state into its one-owner retirement slot. It also retires the Map host owner. The incremental retirement cursor subsequently clears its saved Map drag without emitting an up or selection at [Scenes lines 641–689](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:641). Thus successful admission alone makes old `SCENE_STATE` lookup inert; the cursor can continue releasing CPU/GPU resources fairly.

## Minimal repair

Add one Interpreter helper that runs **before ingress/reconciliation** in `render_ui_document_step`:

1. Borrow `UI_ENGINE` only long enough to clone `retired_component_scene(window_id)`.
2. If there is no head, continue normally.
3. If the one scene retirement owner is occupied, return `false`; the existing leading [`close_retired_scene_surface_one`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2631) turn continues it.
4. Convert that exact head to `ScenePointerTarget`, call `retire_scene_identity` outside the UI borrow, then reacquire UI and acknowledge the exact same queue head. Preserve the existing convention: a false `retire_scene_identity` result means there was no matching mounted Scene state, and still acknowledges this UI retirement. A full retirement slot is the sole reason to leave the head queued.
5. Return `false` after one head. It charges one retained owner per opportunity and allows the next call to make document progress.

Share this helper with the existing post-frame block or remove the latter. It must not scan windows, drain a whole queue, or make `acknowledge_presented_input` synchronously close a Map. The old receiver remains uncallable at ACK; the next bounded document opportunity consumes one exact old state.

## Fail-first and acceptance laws

Keep the present Native135 fixture as the primary regression. It already proves all important boundaries:

- same host/key refresh retains its active Map gesture;
- key replacement ACK immediately revokes the old target;
- no replacement or retirement step creates actions;
- one subsequent bounded document presentation removes old drag and exact Engine token;
- the new host can begin a fresh gesture;
- node removal obeys the same no-input and bounded retirement law.

Add one focused Interpreter-level law for the causal boundary, using the existing real retained Map document:

1. accept Map A and start a Map drag;
2. accept Map B with the same document node and a different key;
3. assert the A target is non-live immediately after ACK;
4. call exactly one document-render opportunity for B;
5. assert that it admits/acknowledges only A's retirement head, does not generate an action, and leaves B's normal document work pending;
6. later drive retirement terminally and assert no A Scene state or Engine token remains.

That law will fail while the queue consumer remains after reconciliation and avoids a test-only direct `take_retired_component_scene`.

## Deferred-close handoff: residual required law

The recent dock-tab repair is directionally correct: it mutates dock layout then queues `shell.windowClose` through [`arm_control_command`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13839), while the older `note_control_command` path awaits dispatch. This avoids holding interaction across a guest call. The shell-owned `deferred_actions` outlive frame candidates; the frame job explicitly records that candidate-owned work is dropped while actions live in `AppRuntime::frame_actions` at [frame-job lines 372–379](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs:372).

One concrete law is still needed before calling this lifecycle complete:

- dispatch a real dock-close control while a presented-input candidate is in flight;
- supersede or cancel that candidate before ACK;
- drive candidate discard/interaction restoration and the deferred lane;
- assert no input candidate is acknowledged, the dock mutation happened once, and exactly one `shell.windowClose` note for the closed ID is eventually dispatched.

This is an owner-transfer/cancellation law, not a request for a second cancellation mechanism. It ensures the close journal belongs to the durable deferred-actions owner rather than the discarded candidate. The source has explicit candidate-return paths in [frame-job](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs:230), but the present dock test covers only the non-superseded close.

Also route `mod+shift+w` through the same close-and-arm operation. It currently calls `close_active_window` directly at [Shell lines 15942–15949](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:15942), whereas React's common close callback invokes `onWindowClose` before its layout update at [Canvas lines 1475–1485](../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1475). Reusing the same shell helper keeps the close journal exact for both user entry points.


