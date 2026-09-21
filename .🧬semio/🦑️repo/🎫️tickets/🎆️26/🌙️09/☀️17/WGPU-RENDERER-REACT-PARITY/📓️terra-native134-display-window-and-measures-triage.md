+# Native134 Display, Context, and Measures Triage

## Scope and evidence

This is a read-only review of five failures in the Native134 receipt:

- [failures.json](../🗑️generated/astra-runtime/renderer-native134-full/failures.json)
- [run.log](../🗑️generated/astra-runtime/renderer-native134-full/run.log)

The receipt was collected before the subsequent Display-fixture correction. This report distinguishes that historical fixture wiring from production behavior visible in the current source. No build was run.

## Display transfer: three tests did not reach the asserted admission boundary

The following failures have the same cause:

1. `actual_display_release_refuses_item_and_byte_credit_before_dock_mutation`
2. `display_window_kind_reaches_shell_as_a_transfer_handle_and_new_window_drag`
3. `refused_window_publication_does_not_block_a_ready_display_peer`

The original `display_transfer_host_fixture` first painted and ACKed the Tree transfer handle, then rewrote `dock`, `dock_canvas_bounds`, and both dock-drop collections. That left the destination geometry only in the candidate fields. The current fixture has been corrected to establish that geometry before the document paint; this is the right ordering.

This mattered because the actual input paths deliberately use accepted geometry:

- pointer move derives `drag.drop_zone` from `presented_input_geometry` at [Shell WGPU source](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12817);
- release recomputes an absent zone from that same presented geometry at [Shell WGPU source](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12870).

The Native134 debug lines show a real Tree handle press followed by a hitless release at `800,500`. With stale presented geometry, the release has no drop zone, so it returns `Ok(())`; it never invokes either credit refusal, creates a new dock window, or enters a topology-publication cohort. It is therefore not evidence against those three properties.

The production admission order already satisfies the intended law. `finish_dock_drag` checks publication capacity, reserves the bounded journal, and only then applies the dock mutation ([Shell WGPU source](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:12870)). The reservation itself publishes the action before returning its token ([Shell WGPU source](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6975)). The cohort releaser explicitly visits each entry once, so a refused body cannot head-of-line block a ready peer ([Shell WGPU source](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7000)).

**Result:** fixture/ACK wiring, not a confirmed Display, topology-credit, or cohort-production defect. The corrected fixture must preserve the physical down/move/hitless-up sequence and only assert these laws after its target geometry has been accepted.

## Context menu: candidate geometry must not answer an interactive query

`context_menu_point_resolves_the_exact_concrete_window_instance` mutated `shell.dock_drop_bodies` and immediately queried `context_window_instance_id`. Production correctly reads `presented_input_geometry.dock_drop_bodies`, not the candidate map ([Shell WGPU source](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6881)).

The correct test law has two phases:

1. after candidate-only mutation, all points return `None`;
2. after sealing and acknowledging the same candidate, each point resolves its exact concrete window id.

The current test correction follows that shape. No production change is indicated; using candidate geometry here would let a context action target pixels not yet presented.

## Measures overlay: stale direct-candidate test, not a supported production input path

`window_measures_overlay_paints_and_dispatches_every_gesture_like_react` failed at the focus assertion for `keyboard-number-entry`. Its `paint_overlay` helper manually invokes `render_ui_document_step`, inspects `staged_hits`, and calls `interpreter::dispatch_ui_event` before a Shell candidate seal/ACK ([Measures test](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/📏️wgpu-window-measures/🦀️.rs:67), [gesture body](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/📏️wgpu-window-measures/🦀️.rs:112)).

That contradicts the production contract:

- a visible window with `presented_ready && !candidate_ready` cannot be sealed ([UI engine](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1112));
- only `acknowledge_presented_input` swaps candidate tree/router into presented tree/router ([UI engine](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1183));
- non-test `dispatch_event` returns no commands until `presented_ready` ([UI engine](../../../../../../🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:2810)).
  
The test-only candidate fallback therefore makes its keyboard assertion depend on residual test-engine state, rather than the accepted pixels and router that a native user can interact with. The normal Shell path already paints the overlay backdrop, routes the document through the stepped paint, and registers body hits ([Shell WGPU source](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4684)).

**Required test repair:** replace `paint_overlay` with a Shell-aware accepted-frame fixture.

1. Put the measure document in `window_measures_documents`, unfold its rail, and invoke `paint_window_measures_step` to completion using the actual window body rect.
2. Seal and ACK through the existing test-only `publish_retained_hit_registry`; this promotes both Interpreter state and Shell hit/geometry together ([Shell WGPU source](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13064)).
3. Read `input.hits()`, rather than `staged_hits()`. Drive toggle pointer events through `ShellState::handle_pointer_button`. Keyboard events may still use the host event bridge, but only after ACK, and must pass resulting `FocusChanged` commands to `note_content_focus_commands`.
4. Keep the existing action payload oracle and add an explicit assertion that the accepted overlay surface is the focus owner after the first Tab.

This preserves a behavioral test of the real overlay, validates the accepted-frame barrier, and does not weaken production to make candidate content interactive.

## Conclusion

All five Native134 failures in this requested group are stale fixture admission/ACK seams. The Display and context fixes already landed as test corrections. Measures still needs the accepted-frame fixture rewrite above. No production display, pointer, context-menu, or measures dispatcher defect was established by the Native134 receipt.

