# Draw viewer: three failing unit tests

Crate `semio-s-artifact-draw-drawing-viewer` (`✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/`). Input log: `🗑️generated/r26-viewer.txt` (12 passed, 3 failed).

## Was the host-only action change the cause?

No. None of the three failures touches the seven new host-input verbs. `drawing_viewer_canvas_declares_every_action_its_surface_dispatches` and `drawing_viewer_host_input_commands_publish_nothing` passed. Failure 2 did expose one latent flaw in the `setCamera` command (see below), which the host-only change did not create.

## 1. `set_mutation_wire_is_the_flat_tagged_record` (key order)

Root cause: the test's expectation was stale, not the framework.
- `DslValue::Object` is `Vec<(String, DslValue)>`, deliberately insertion-ordered (`🧰️framework/🔨️modules/🌱️value/🌳️intrinsic/🦀️.rs`). It is not canonical. Canonical ordering lives in the pack/JSON codecs, not in the value algebra.
- The test built `expected` through `serde_json::json!(...).into()`. `serde_json::Map` is a `BTreeMap` unless the `preserve_order` feature is unified into the build, so the expected value came out sorted (`framed, kind, viewport`) while `to_value()` correctly emits declaration order (`kind, viewport, framed`). The comparison depended on cargo feature unification, not on the contract.
- No framework change broke ordering (`From<&serde_json::Value> for DslValue` is unchanged and follows its source map).

Fix: build the expected value with `semio_plugin::dsl_value!`, which preserves literal order, so the test pins the flat tagged record in its authored order.

## 2. `drawing_viewer_camera_ownership_and_restore` (host event routing)

Two defects in the test harness and the command, found in sequence.

a. Harness: `host event routing requires its actual mounted actor and instance` (`🧰️framework/.../📨️dispatch/📥️staging/📣️host-event-routing/🦀️.rs`). A view-bearing `ActionMeta` must name the live runtime instance (`bind_instance_id(91)` already did) and the store's own local actor. The test used `meta("drawing-viewer-camera")`, an actor the app was never born under. Fix: dispatch under `meta(semio_os_kernel::LOCAL_ACTOR_ID)`, the actor `new_app_with_registry` mounted the app with.

b. Source: after routing passed, `expected an object, found String("{\"x\":18,...}")`. `DrawingViewCommand::SetCamera` carried `camera: String` (JSON text), but the typed-command channel (`dispatch_original_fixture_command`) turns the command into action arguments with `ToValue` and feeds them to `command_from_action`, which (as the host does) expects `args.camera` to be an object. The command value and its action arguments disagreed, so the typed path could never decode its own command. Fix at the root: the command field is now the structured `store::Viewport2d` (`#[dsl(block)]`, the same shape as Trinity Jack's `SetViewport`). `command_from_action` stores the validated pose directly, `camera_emit` reads it without re-parsing JSON, and the Default is `Viewport2d::default()`. The test now decodes the fixture camera into a `Viewport2d` for the command.

c. Source: after the command decoded, restoring the saved window packs failed with `original JSON operation wire header differs`. The persisted history of a window config pack stores every operation as its `OpBinary::encode_op` bytes (a Pack Value record, see `print_document_spr`/`history_op_payloads`), and the mutation declares exactly that through `original_pack_value_operation_receiving!`. The owner's `native_operation_wire()` installed the JSON receiver (`canonical().with_original_receiving()`, header `op`), so the first committed edit in a pack could never be read back. It only surfaced once a window carried two edits (the fixture moves `left` twice, which commits the first); a single uncommitted edit per window never reaches the decoder. Fix: the owner installs `canonical().with_original_pack_value_receiving()`, the receiver matching the declared persisted encoding. The Draw editor canvas owner (`✏️editor/.../🖼️canvas/🎚️config/🦀️.rs:230`) carries the identical `with_original_receiving()` declaration and the same latent defect for windows with two or more camera edits; its own test moves each window once, so it does not trigger it. Not changed here (outside the viewer crate, not verified).

## 3. `drawing_viewer_restores_edited_archive_and_preserves_history` (undo)

Root cause: the test's actor model was wrong, not the archive or the history engine.
- Undo only reverts an edit whose recorded actor equals the store's local actor (`StoreLog::edit_is_local`); a foreign edit is a silent no-op (`ForeignEdit` collapses to an empty result). The per-dispatch `meta.actor` does not change who the store is.
- Typed publication stamps each edit with `meta.actor`, so the editor's edits carried `drawing-role-archive`. The test mounted every app (editor, viewer, returning editor) as `local`, so after the archive restore every authored edit was foreign to the returning editor. The fixture's `undo` by the author was therefore a no-op ("received Retained Drawing"), exactly like the observer's expected no-op.
- The archive and the history bytes round-trip intact (the equality checks before the undo rows passed). Nothing replays against a wrong base.

Fix: model the two actors the fixture names as two replicas. Editor, viewer and returning editor are born as the fixture author; a separate observer replica, born as the fixture observer, restores the same archive. Each fixture row runs on the replica of its actor, so the author's undo/redo walk the history and the observer's undo is the foreign no-op.

## Files changed
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs` (command field type, `command_from_action`, `camera_emit`, Default)
- `.../👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️canvas/🎚️config/🧪️tests/🔬️unit/🦀️.rs` (tests 1 and 2)
- `.../👁️viewer/🧪️tests/🔬️unit/🦀️.rs` (test 3, actor model)
- `viewer-tests-run-2026-11-27.sh` (ticket helper: waits for peer cargo builds and 45 percent free memory, retries after infrastructure failures)

## Suite result
RESULT_PLACEHOLDER
