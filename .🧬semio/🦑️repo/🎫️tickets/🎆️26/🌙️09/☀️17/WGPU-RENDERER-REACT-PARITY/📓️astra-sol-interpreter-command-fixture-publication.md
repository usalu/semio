# Interpreter Command Fixture Publication

## Evidence

Native 134 showed four Interpreter laws failing behind candidate-only fixture setup. Clipboard failed before readback because direct `apply_tree` had not created accepted presented input. Ink cancellation compared against an authored scene surface instead of the mounted document surface. Ink editing expected private focus controls to use the authored surface even though retained controls use generated host identity. Canvas replaced only a candidate tree and expected the terminal lane to observe a replacement that was never sealed or acknowledged.

Receipt: `🗑️generated/astra-runtime/renderer-native134-full/failures.json`. Source audit: `📓️terra-native134-map-and-command-publication-audit.md`.

## Coherent fixture repair

The command test now has `publish_presented_document`, which performs document publication, bounded reconciliation, visibility publication, input-candidate seal, and ACK. It does not manufacture a presented router.

The clipboard law publishes a two-record document containing a real Input binding, dispatches Tab only after ACK, applies the mocked paste through the production command path, verifies the presented edit state, and drains the retained document close owner.

The two Ink laws use the retained Ink document projection through the generalized publication helper. Public action and cancellation identity comes from the mounted scene's `surface_id`, which equals the owning document window. Private edit controls are derived from its generated `host_id`; neutral expected control suffixes remain unchanged. The stale editor case closes the retained document, republishes a successor, and verifies that the old focus address cannot mutate it.

The Canvas law begins on accepted Canvas pixels, then publishes and ACKs a same-key TextEditor replacement. The new component receives a distinct generated host. The old Canvas gesture reaches its existing cancelled terminal action only after the replacement is presented, and the successor document is retired through the bounded close queue.

Changed file: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs`.

`rustfmt --edition 2021 --check` passes. Native execution is root-owned and pending Native 135.

Native 135 proved clipboard and Ink cancellation GREEN. It exposed two fixture follow-ups. The generalized editor document omitted Canvas2d from its closed surface-kind match; it now encodes a real base Canvas scene. The Ink lifecycle's retained close correctly cleared the global focus address before the stale key arrived, leaving the test-local `InputState` focus until an input event observed the close. The law now sends one key while the retained close is pending, requiring immediate refusal, focus clearing, and zero actions; it then drains the close, republishes the successor, and repeats the stale refusal. No production editor or Canvas cancellation behavior changed. Receipt: `🗑️generated/astra-runtime/renderer-native135-full/failures.json`.

Native139 reached the Canvas law's second `publish_document`, where the prior ACK-owned candidate baseline still had to advance. Other staged-document fixture helpers already close that bounded baseline before publishing the next generation. `publish_focus_rebase_document` now does the same for generation greater than one, then publishes and reconciles the real successor as before. The cancelled `canvasPointerUp`, fresh generated host, and exact successor-kind assertions remain strict. Receipt: `🗑️generated/astra-runtime/renderer-native139-nine-and-measures/run.log`. Focused Native140 verification is pending.

## Native144 Canvas terminal ownership

Native144 accepted and presented the Canvas-to-TextEditor replacement, but the next scene-interaction opportunity returned false instead of publishing the old Canvas gesture's cancelled terminal. The same empty-candidate reconciliation gap described in `📓️astra-sol-map-retirement-progress.md` had omitted the old Canvas retirement record. The shared reconcile repair now emits that exact owner.

At presenter ACK, the Interpreter inspects the exact visible retirement heads and marks retired Canvas hosts for terminal cancellation before returning control to input. This prevents an input opportunity immediately after ACK from missing the terminal simply because the successor no longer paints Canvas. The mark is host-scoped and bounded by the already bounded visible-window and retirement queues. The later scene-retirement handoff repeats the host-scoped mark idempotently.

`cancel_canvas_pointer_gesture_into` now treats only a pending whole-document close as silent. A component replacement inside a still-live document emits the existing `canvasPointerUp { cancelled: true }` from its retained gesture address even while the old scene host is retiring. Whole-window close remains silent and continues to discard its Canvas gesture through the close queue.

Native144 receipt: `🗑️generated/astra-runtime/renderer-native144-compact-ax-locale-red/run.log`; verification is pending Native145 under `test(live_canvas_gesture_cancels_when_its_published_generation_is_replaced)`.
## Native145 closed-document reopen fixture

Native145 reached the exact close fence in `ink_canvas_text_and_table_editing_matches_the_react_host_lifecycle`, drained the presented document, and then called the live-replacement helper for generation 2. That helper first tried to finish generation 1 reconcile, but the closed document no longer owns such a reconcile job, so the bounded 4,096-step fixture ceiling correctly remained pending.

The fixture now separates `publish_new_focus_rebase_document`, which publishes and settles a new or reopened document, from `publish_focus_rebase_document`, which additionally completes a still-live prior generation before replacement. The Ink close/reopen branch uses the new-document path. The live Canvas-to-Text replacement law still uses the replacement path and therefore preserves the stale-gesture retirement sequence. No production reconcile or close behavior changed.

Native rerun remains root-owned.
