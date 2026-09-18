# W1m — wgpu UI intent dispatch (ActionDescriptor → UiIntent)

Packet W1m of `26/09/17/WGPU-RENDERER-REACT-PARITY`. Implemented 2026-09-17/18.
Inputs: `📓️audit-interpreter-elements.md` §7.2/§7.3 + findings #2/#10/#11 (work packet 2),
`📓️audit-runtime-boot-input.md` action-dispatch rows.

## 1. Dispatch model — before

```
guest patch ──▶ UiDocumentTree (surface, revision, records w/ bindings)
                      │  reconcile::ui_node_from_record
                      ▼
               UiNode spec   ── record_action(trigger, controller) ─▶ ActionDescriptor
                                                                     { controller_id, action: NAME ONLY, args }
                      │        (binding.action.scope + .version DROPPED here)
                      ▼
           tree::Node { key, spec, state, layout, flags }        ← no address, no revision
                      │
   pointer/key event ─┤ events::EventRouter
                      │   commit_action(): merges the gesture scalar INTO args under "value"/"delta"
                      ▼
      UiCommand::App { window_id, action: ActionDescriptor }
                      │
                      ▼
 os-renderer interpreter::apply_ui_commands → publish_retained_action
                      │   (reserve controller_id + action, copy args, stamp windowId)
                      ▼
               BoundedActionQueue (256 items / 1 MiB) ─▶ RuntimeMailbox
```

No surface, no revision, no node id, no node key, no seq. Nothing could be dropped as stale and
nothing could be de-duplicated: the queue is FIFO over opaque `(controller, action, args)` triples.
Zero `UiIntent` hits anywhere under `🎯️targets/🧊️wgpu` (audit §7.2). React, by contrast, reserves
`UiInterpreterContext.onAction` for its 14 unowned scene-host elements and routes every semantic
control through `onIntent` / `UiDocumentStore::buildIntent`.

## 2. Dispatch model — after

```
guest patch ──▶ UiDocumentTree (surface, revision, records w/ bindings)
                      │  reconcile Mount step
        ┌─────────────┴──────────────────────────────┐
        ▼                                            ▼
   UiNode spec (unchanged carrier:            UiIntentBindings {
   controller_id / name / authored args         address: { surface, revision, node, node_key },
   for paint + the chrome widget path)          bindings: [(Trigger, ActionId)]   ← scope+name+version
                      │                       }  stamped on tree::Node.intent, RE-stamped every revision
                      ▼
   pointer/key event ─┤ events::EventRouter::build_intent
                      │   • intent_is_stale(node.intent.revision, document.revision()) ⇒ DROP
                      │   • ActionId resolved from the node's own binding for this trigger
                      │   • seq minted per surface (UiIntentSequencer)
                      │   • args (authored) and input (trigger payload) kept APART
                      ▼
      UiCommand::App { window_id, intent: UiIntentCommand }
                      │
                      ▼
 os-renderer interpreter::apply_ui_commands
                      │   • InputState::admit_intent → BoundedActionQueue::admit_intent_seq
                      │       seq ≤ last admitted for this surface ⇒ Duplicate, dropped
                      │   • intent.descriptor(): args ⊕ input merged here, `name@version` beyond v1
                      ▼
               BoundedActionQueue ─▶ RuntimeMailbox
```

Scene hosts and the chrome's immediate-mode widgets keep the bare `ActionDescriptor` — a node no
document published carries `intent: None`, has no revision to be stale against, and fires with a
default address. That is exactly React's split.

## 3. Field mapping

| `ui_contract::UiIntent` | React source | wgpu source (after) | Notes |
|---|---|---|---|
| `surface` | `store.state.surface` | `UiDocumentTree::surface()`, stamped at mount | `UiIntentAddress.surface` |
| `revision` | `store.state.revision` | `UiDocumentTree::revision().0` at mount | re-stamped on every revision, never diffed |
| `node` | `record.id` | `UiNodeRecord::id.0` | |
| `nodeKey` | `record.key` | `UiNodeRecord::key` | survives id churn |
| `trigger` | `binding.trigger` | `FiredAction.trigger` chosen by the gesture | `Commit` for `commit:"blur"` inputs, `Change` otherwise; `Delta` only when bound |
| `action` | `binding.action` (full `ActionId`) | `UiIntentBindings::action_for(trigger)` | **scope + version now survive**; previously only `name` |
| `args` | `binding.args` | `ActionDescriptor.args` (authored, unmerged) | |
| `input` | `dispatchTrigger`'s `input` | `FiredAction.input` | separate field, merged only in `descriptor()` |
| `seq` | `store.seq += 1n` | `UiIntentSequencer::next(surface)` | per-surface monotonic |
| — (`controllerId` resolved from surface) | — | `UiIntentCommand.controller_id` | wgpu-only: the host's bridge still needs it (see gap G3) |

Payload merge is React's `uiIntentPayload`/`uiInputField` verbatim: a scalar is named by its trigger
(`delta` for `Trigger::Delta`, `value` otherwise) and merged **over** the authored args; a map payload
merges key-wise; `input: None` dispatches the authored args untouched.

Staleness is `🖌️render/🖱️dispatch/🦀️.rs::is_stale` verbatim: `current > recorded + 1`.

## 4. Files changed

| File | Change |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎬️action/🦀️.rs` | new `🎬️Intent` region: `UiIntentAddress`, `UiIntentBindings`, `UiIntentCommand` (+`descriptor`/`payload`/`action_name`/`input_field`), `intent_is_stale`, `UiIntentSequencer`; `BoundedActionQueue` gains a per-surface admitted-seq ledger with `admit_intent`/`admit_intent_seq`/`admitted_seq` and `UiIntentAdmission{Accepted,Stale,Duplicate}` |
| `…/🧊️wgpu/🌳️tree/🦀️.rs` | `Node.intent: Option<UiIntentBindings>` |
| `…/🧊️wgpu/🔀️reconcile/🦀️.rs` | `record_intent_bindings`; Mount stamps/re-stamps it; `surface_plugin_id` replaces the hardcoded `""` on `UiExternalSlotNode.plugin_id`; `children_of` gains an `open` gate and `reconcile_children` feeds it `WidgetState::open` |
| `…/🧊️wgpu/⚡️events/🦀️.rs` | `FiredAction`/`fired_action`/`bare_action`/`descriptor_action_id` replace `commit_action`; `EventRouter::{build_intent, app_command, push_app_command}` + `intents: UiIntentSequencer`; `UiCommand::App` now carries `UiIntentCommand`; focus fns return `(NodeId, FiredAction)`; `pointer_commit_action` takes `&Node` and gates the stepper's `Delta` path on declared bindings; `constrain_number_input` (pub) enforces `min`/`max`/`step` at commit |
| `…/🧊️wgpu/📥️input/🦀️.rs` | `InputState::admit_intent` |
| `…/🧊️wgpu/🪀️widgets/🦀️.rs` | `WidgetNode::Input`/`ControlNode::Input` gain `min`/`max`/`step`/`accept`; `InputMeta` carries them plus `input_kind` and gains `commit_value`; `register_input_meta` threads them |
| `…/🧊️wgpu/🐚️shell/🦀️.rs` | stale doc comment refreshed |
| `🧱️elements/🪜️Stepper/🎯️targets/🧊️wgpu/🦀️.rs` | `register_input_meta` call updated (`"number"`, `step`) |
| os-renderer `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` | `UiCommand::App` arm admits by seq, then `publish_retained_action(&intent.descriptor())` |

## 5. Tests

New / rewritten laws (all passing):

| Test | File | Pins |
|---|---|---|
| `a_stale_revision_intent_never_reaches_the_queue` | `🧪️tests/🔬️targets-wgpu-action-unit` | `current > recorded + 1` refusal; a refused intent does not advance the seq cursor |
| `per_surface_seq_orders_and_deduplicates_intents` | same | replay and reorder refused; seq is per surface, not global |
| `a_surfaces_sequencer_is_monotonic_and_independent_of_its_neighbours` | same | minting |
| `an_intents_payload_names_its_scalar_by_trigger_and_merges_over_the_authored_args` | same | `value`/`delta` naming, authored args survive, payload-free triggers |
| `a_versioned_action_id_survives_as_its_own_address` | same | `name@2` |
| `mounting_a_record_stamps_its_whole_dispatch_contract_onto_the_arena_node` | `🧪️tests/🌳️document-tree-reconcile` | address + `(trigger, ActionId)` round-trip incl. scope/version; `binds()` answers absence |
| `a_new_revision_restamps_every_surviving_node…` | same | identity preserved, revision refreshed |
| `an_extension_slot_carries_its_publishers_plugin_id_instead_of_an_empty_string` | same | `plugin_id` = surface's first segment |
| `a_closed_select_materializes_no_option_rows_at_all` | `🧪️tests/🔬️targets-wgpu-reconcile-unit` | open/close gate; `HAS_POPUP` still announced |
| `an_intent_fired_against_a_revision_the_user_never_saw_is_dropped` | `🧪️tests/🎛️retained-control-commit` | end-to-end stale drop through the real router; one revision behind still fires |
| `each_gesture_on_a_surface_advances_that_surfaces_own_seq` | same | seq 1,2 + surface/node_key on the wire |
| `a_number_inputs_min_max_and_step_constrain_what_it_commits` | same | clamp high/low, snap to step, unconstrained passthrough |
| `a_stepper_takes_the_relative_path_only_when_it_declares_a_delta_binding` | same | both descriptors non-empty — only the stamped binding list decides |
| `number_constraints_clamp_and_snap_exactly_once_each` | `🧪️tests/🔬️targets-wgpu-input-unit` | step ladder starts at `min`, zero step is no step, NaN stays a refusal |
| `an_input_metas_commit_value_carries_its_own_constraints` | same | text verbatim, number constrained, empty number commits nothing, `accept` reaches the host |

Adapted to the new `UiCommand::App` shape: `🎛️retained-control-commit` (whole fixture law of 15+
cases still green through `intent.descriptor()`), `🔬️targets-wgpu-engine-unit`,
`🔬️wgpu-ui-command-wiring` (now builds a `UiIntentCommand` with a distinct seq per case).
`🔬️targets-wgpu-events-unit` was adapted concurrently by the peer working that file.

Two existing select-row laws now open the popup first (`open_select` helper) — a closed `Select`
legitimately has no rows any more.

## 6. Verification (all foreground, logs in `🗑️generated/w1m-*.txt`)

| Command | Result |
|---|---|
| `cargo check -p semio-framework-ui --features wgpu-engine --lib --keep-going` | exit 0 (warnings emitted ⇒ expansion really ran) |
| `cargo check -p semio-framework-ui --features wgpu-engine,testkit --lib --keep-going` | exit 0 |
| `cargo check -p semio-framework-ui --target wasm32-unknown-unknown --lib --keep-going` | exit 0 |
| `cargo check -p semio-framework-ui --target wasm32-wasip2 --lib --keep-going` | exit 0 |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --keep-going` | exit 0 |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib` (my filters) | 82 passed, 0 failed |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- wgpu_ui_command_wiring interpreter` | 38 passed, 4 failed — all pre-existing / peer-lane, see below |

Note: `cargo` was SIGKILLed (exit 137) repeatedly under a ~16-cargo peer fleet; every command above
was retried until it survived. `CARGO_INCREMENTAL=0` made this materially more reliable.

### Failures that are NOT this packet's

Verified by re-running with my three new document tests skipped, and by `git diff --stat` showing
large uncommitted peer edits in those exact files:

- `wgpu::flex::tests::*` (3), `wgpu::mounted_layout::tests::*`, `wgpu::layout::tree_row_rect_tests::*`,
  `wgpu::input::retained_hit_target_tests::*` — W1l's live flex/layout rewrite (785 + 327 + 67 lines
  uncommitted). The hit-target failure is a `315 vs 315.392` rect width, i.e. layout math.
- `wgpu::paint::tests::painting_a_focused_*` (5) — W1n's live paint edits (553 lines uncommitted).
- `wgpu::engine::tests::*`, `wgpu::prepared::tests::*` — timing/budget assertions flapping under the
  peer fleet (`largest observed layout slice was 46ms` against an 8 ms budget) plus engine edits (117
  lines uncommitted); the set changes run to run.
- `wgpu::component::ui::ui_node_wire_format_tests::scene_records_serialize_to_golden_json` — code has
  `gridVisible`/`selectableNodes`/`selectableEdges`/`selectableHandles`, the golden JSON does not.
- `wgpu::reconcile::document_tree_reconcile_tests::a_surfaces_second_document_…` — `ArenaFull` from
  the process-wide record arena when the whole 509-test suite runs in one process. **Reproduced
  identically with my three tests skipped**, and green when the module runs alone. Pre-existing.
  My new tests nonetheless retire their documents explicitly (`retire()` helper) so they add none of
  that pressure.
- os-renderer: 3 SVG raster-decode tests + `scene_command_right_click_on_text_editor_…` — the Scene
  path and image decoding, neither touched here; that target has 100 + 359 lines of peer edits.

## 7. Remaining gaps

- **G1 — the kernel seam is still a fixture.** `🎯️targets/🧊️wgpu/🪢️kernel-seam/🦀️.rs` names
  `ui_contract::UiIntent` but its own module docstring says *"Test fixture … Production rendering
  dispatches through RuntimeMailbox; this fixture has no production router."* The production path
  finished here is `events → UiCommand::App → admit_intent → publish_retained_action →
  BoundedActionQueue → RuntimeMailbox`. Converting `UiIntentCommand` into a real
  `ui_contract::UiIntent` at that seam needs `UiText`/`UiValue` arena credits on the renderer thread
  and a surface→instance router; both are out of this packet and gated on the protocol-flip packet
  the seam's own docstring cites.
- **G2 — staleness is enforced renderer-side only.** `EventRouter::build_intent` has the live
  document and refuses there (as React's `Dispatcher` does). The host-side `admit_intent_seq` does
  ordering/dedup only, because `apply_ui_commands` has no document handle. `BoundedActionQueue::
  admit_intent(intent, current_revision)` exists for a caller that does.
- **G3 — controller identity still diverges from React.** React's `uiIntentToActionDescriptor` uses
  `intent.action.scope` as the controller; wgpu keeps the session/app controller
  (`reconcile::record_action`'s deliberate choice). The full `ActionId` now travels, so flipping this
  is a one-line change once someone confirms which side is right — but it would change every live
  dispatch, so it is deliberately not done here.
- **G4 — `Select` open-gating covers the testkit path only.** `children_of`/`apply_tree` are
  `cfg(any(test, feature = "testkit"))`; the production document path gives a `Select` record no
  children at all and `paint_select` draws rows from the inline spec. A `Select` opened by the user
  therefore materializes rows on the next `apply_tree`, not on the toggle. Making the toggle itself
  re-expand needs an ungated `UiTree::sync_select_rows`, which would also change what
  `sync_select_popup_rows` sees in production — deliberately left to the paint/events owner.
- **G5 — `accept` is carried, not applied.** `InputMeta.accept` and `UiInputNode.accept` now reach
  the host, but the wgpu target has no file picker to hand it to (an accepted, bounded gap, audit
  finding #13).
- **G6 — the chrome's own commit path does not yet call `InputMeta::commit_value`.** The helper and
  its constraints are in place and unit-tested; wiring `🐚️Shell/🎯️targets/🧊️wgpu`'s
  `commit_focused_input` to use it instead of `"value": input.text_view()` is a one-line change in a
  file three peers are editing concurrently, so it is left for that lane.
- **G7 — an inert `Button` no longer emits an empty `App` command.** Previously a `Button` whose
  descriptor had an empty action name still pushed `UiCommand::App`; it is now silent, matching
  React's `emitIntent` returning `undefined`. Behavioural change, believed strictly correct.
