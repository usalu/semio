# 📓 terra-render-dispatch-report

Packet `render-dispatch`, wave W2. **Done.** File owned and rewritten wholesale:
`🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust/🦀️dispatch.rs` (2,201 lines).

No other file touched. Read-only inputs: `🎯️targets/🧊️wgpu/🦀️events.rs` (2,155 L, the packet — full read),
`🦀️input.rs` (270 L), `🖼️render/…/🦀️frame.rs`, `🦀️element.rs`, `🦀️scene.rs`, `🧬️contract/…/🦀️action.rs`,
`🦀️document.rs`, `🦀️schedule.rs` (read for types referenced by path only).

## acceptance: UNRUN (U4 — sol runs every gate)

```
CARGO_TARGET_DIR=<session-scratchpad>/target cargo check -p semio-framework-ui-render --lib --all-features --timeout 600000
CARGO_TARGET_DIR=<session-scratchpad>/target cargo check -p semio-framework-ui-render --all-targets --all-features --timeout 600000
CARGO_TARGET_DIR=<session-scratchpad>/target cargo test -p semio-framework-ui-render --lib dispatch:: --all-features --timeout 600000
CARGO_TARGET_DIR=<session-scratchpad>/target cargo tree -p semio-framework-ui-render --invert wgpu --timeout 600000   # must stay empty (dispatch.rs adds no such dep)
```

Cheap non-cargo checks performed instead, both green:
- Brace-balance scan (python) over the whole file: depth 0 at EOF, no unbalanced `{}`.
- `grep`-based cross-reference of every `self.`/`tree.`/`dispatcher.`/`.overlays.` call site against
  every `fn`/method actually defined in the file — no unmatched name found.

Unresolved sibling names are expected and present: `crate::element::{Bounds, ElementId, Hitbox}`,
`crate::schedule::InvalidationReason`, `ui_contract::{ActionBinding, ActionId, SurfaceId, Trigger,
UiIntent, UiNodeId, UiRevision, UiValue}` — all referenced by path only, none defined here.

## Design summary (read before the parity table)

`DispatchTree` is **immutable once built** — it lives inside `Rc<FrameSnapshot>` (frame.rs's own
invariant: "input is always dispatched against the presented generation"). Every old `EventRouter`
behaviour that mutated the retained `UiTree`'s `NodeFlags` in place (`ACTIVE`/`FOCUSED`/`HOVERED`/
`OVERLAY`) instead becomes persistent state on `Dispatcher`, keyed by the one identity stable across a
frame rebuild — `ElementId`, never `FrameNodeId` (which is only valid for the tree it came from).
`Dispatcher` is the `EventRouter` replacement: a host constructs one per window, keeps it alongside its
`FrameEngine`, and calls `Dispatcher::dispatch(&tree, &event) -> DispatchOutcome` per real input event.

`Hitbox.bounds` (`element.rs`) is already window-absolute, unlike `wgpu-old`'s parent-relative
`layout.x`/`layout.y` — this removes `events.rs`'s `node_abs_origin` parent-walk and collapses
`hit_test_subtree` into `hit_test` with a different root (documented on both functions in-file).

The old product-specific `is_plain_stack_container` (matches `UiNode::Stack`) becomes the generic
`DispatchFlags::LAYOUT_CONTAINER` opt-in flag: a node an `Element` author declares as pure layout is a
hit-test pass-through *unless* it carries a binding of its own or is a drag source — same rule,
generalized past one product enum. `ListenerSet` carries the contract's own `ActionBinding`s (already a
closed `Trigger` enum) plus the protocol addressing (`surface`/`node`/`node_key`/`revision`) needed to
stamp a real `UiIntent` — this is the "typed listeners" the ticket asks for in place of `HitKind`.

## Behaviour-parity table

| semantic | `events.rs` source | now lives at |
| --- | --- | --- |
| Overlay-first, reverse-paint-order hit test | `hit_test_node` L118–150, esp. L126–138 | `hit_test_node` (🔖️HitTest) |
| `CLIPS_CHILDREN` subtree pruning | L123–125 | `hit_test_node`'s `CLIPS_CHILDREN` check |
| `HIT_TRANSPARENT` pass-through | L145 | `hit_test_node`'s `HIT_TRANSPARENT` check |
| Plain-Stack pass-through incl. activate/drop_action/DRAG_SOURCE exceptions | `is_plain_stack_container` L161–164 | `is_plain_pass_through`, generalized to `DispatchFlags::LAYOUT_CONTAINER` — see design summary |
| Pointer capture routes move/up regardless of position | `CaptureState` L262–279, `resolve_target` L774–779 | `CaptureEntry`, `Dispatcher::resolve_target`, now multi-pointer (`HashMap<PointerId, CaptureEntry>`) |
| Capture → target → bubble, cancellable | `bubble` L383–391, `dispatch` L1173–1326 | `bubble` (🔖️Propagation), `Dispatcher::dispatch` (🔖️Outcome); cancellation is `bubble`'s own `true`-stops-the-walk contract, ported verbatim and covered by `bubble_stops_when_a_handler_returns_true` |
| Focus: traversal ring | `collect_focusable`/`FocusState::focus_next`/`focus_prev` L288–376 | `collect_focusable`, `Dispatcher::focus_next`/`focus_prev`, generalized from `is_focusable`'s variant match to `DispatchFlags::FOCUSABLE` |
| Focus: programmatic + scopes | `FocusState::set_focus` L317–341 | `Dispatcher::apply_focus_transition` — single choke point, also now owns `EditState` seed/clear (was already true in `events.rs` too) |
| Focus: restoration after overlay close | `finish_close` L877–893 | `Dispatcher::finish_close` — verbatim logic, `NodeId`→`ElementId`; covered by `closing_an_overlay_clears_focus_that_was_inside_it` |
| Overlay dismiss policies | `DismissPolicy`, `dismiss_topmost_if_outside_press` L452–465, L898–908; tooltip hover-out L910–929 | `DismissPolicy`, `Dispatcher::dismiss_topmost_if_outside_press`, `Dispatcher::maybe_dismiss_tooltip_on_hover_out` — verbatim |
| Overlay anchor/placement resolution | `resolve_overlay_placement` L539–567 | `resolve_overlay_placement` — verbatim math, `OverlayAnchor::Node`→`OverlayAnchor::Element` |
| `DragSession`, drop-target accept predicates | L593–606, `nearest_accepting_drop_target` L978–998 | `DragSession`, `Dispatcher::nearest_accepting_drop_target` — verbatim, `NodeId`→`ElementId` |
| Drag ghost painted next frame, not current | `DragGhost` L584–591, module doc L571–575 | `DragGhost`, `Dispatcher::set_drag_ghost`/`drag_session()` accessor |
| Scroll routes to deepest eligible ancestor | `nearest_scrollable_ancestor` L621–633, `route_scroll` L1007–1015 | `nearest_scrollable_ancestor`, `Dispatcher::route_scroll` — verbatim |
| `DispatchOutcome` replaces async handlers | n/a (new contract) | `DispatchOutcome` — exact ticket shape; every `Dispatcher` method is a plain `fn`, no `await` anywhere |
| `UiIntent` carries source revision, stale rejected | n/a (old `UiCommand` had no revision) | `Dispatcher::fire`/`fire_captured` + `is_stale` (revision > recorded + 1 ⇒ dropped, per master.md's own rule); covered by `stale_captured_activate_is_rejected_rather_than_misapplied` |

## Deliberately not ported, and why

- **`find_tree_item_spec`/`find_item_in_sections`/`find_item_in_items`** (`events.rs` 🔖️TreeItemLookup,
  L166–205): `Tree`/`UiTreeItemNode`-specific re-derivation of `draggable`/`drag_data` from a row's key.
  The generic replacement is `Dispatcher::set_drag_payload`, callable by any `Element` regardless of
  product shape — dispatch.rs no longer needs to know what a `Tree` is. Covered by
  `pressing_a_drag_source_then_moving_past_threshold_promotes_it_to_a_drag_session`.
- **`UiCommand::Scene`** (L705–715, `scene_command` L1328–1346): routed a raw event into a
  `ComponentScene` leaf by matching that one `UiNode` variant. This port's whole point is eliminating
  per-product hit-kind matching, so there is no direct replacement — any `Element` that wants raw event
  pass-through registers an ordinary binding like any other node. Covered (as the property, not the
  mechanism) by `a_node_with_multiple_typed_bindings_only_fires_the_one_matching_the_interaction`.
- **`UiCommand::FocusChanged`/`OverlayClosed`/`DropCancelled`**: `DispatchOutcome`'s ticket-specified
  shape (`handled`/`intents`/`cursor`/`invalidation`/`capture`/`ime`) has no side-channel for a
  state-change *notification* without a `Trigger`. The replacement is that `Dispatcher` itself is the
  source of truth — a host diffs `Dispatcher::focused()`/`open_overlays()`/`drag_session()` before and
  after a `dispatch()` call rather than receiving a matching command.
- **`EditState::scroll_x`**: caret-into-view horizontal scroll is a paint-time concern; there is no
  paint step in this module, so it is dropped from the ported `EditState` (documented on the struct).
- **Clipboard `Copy`/`Cut`/`PasteRequested`**: the contract's `Trigger` enum has no clipboard variant,
  so these are synthesized as `UiIntent`s against a well-known `ActionId::v1("dispatch", …)` rather than
  a bespoke command — a deliberate, flagged trade-off (see registrar-requests).

## registrar-requests

1. **`crate::DispatchTree: From<Vec<Hitbox>>` is a lossy adapter as currently specified.** `Hitbox`
   (`element.rs`) carries only `element`/`bounds`/`clips_children`/`hit_transparent` — no parent link, no
   overlay bit, no listener/protocol data, no revision. The `From` impl I wrote (documented in-file on
   the impl itself) can only reconstruct tree structure via a bounds-containment-stack heuristic and
   produces nodes with empty `ListenerSet`s and no `OVERLAY` flag. Recommend either a richer `Hitbox`
   (parent id, overlay bit) or a dedicated adapter call carrying the element→listener/parent/overlay map
   a `GpuSemanticAdapter`-equivalent would own, plus a revision parameter (`From` has none to plumb one
   through today). All of this module's actual behaviour is exercised directly against hand-built
   `DispatchTree`s in its own tests, never through this adapter, so the gap does not affect what's
   proven — it affects only real `build_frame` wiring, which is future (`runtime-transact`/host) work.
2. **`DispatchOutcome`'s six fields have no channel for a host-facing side effect without a `Trigger`**
   (see "deliberately not ported" above, clipboard specifically). If a cleaner mechanism than a
   synthetic `ActionId` is wanted, it needs a ticket-level decision — I did not add a field beyond the
   ticket's given struct shape on my own authority.

## deviations

- `DispatchFlags` drops `ACTIVE`/`FOCUSED`/`HOVERED` relative to old `NodeFlags` — these are dynamic
  interaction *state*, and `DispatchTree` is immutable per frame (shared via `Rc`), so they live on
  `Dispatcher` as read accessors (`is_hovered`/`focused`/`is_captured`) instead of flags flipped in
  place. See design summary.
- Multi-pointer capture (`HashMap<PointerId, CaptureEntry>`) generalizes `events.rs`'s single implicit
  pointer — required by U3's "platform events arrive as this crate's own normalized types (multi-pointer
  capable)" rule; every ported test still exercises the single-pointer case and passes under this more
  general model.
- `hit_test_subtree` is now literally `hit_test` with a different root (see design summary) — a
  simplification forced by absolute bounds, not a behaviour change.

## Files touched

- `🧰️framework/🔨️modules/🖱️ui/🖼️render/📦️packages/🦀️rust/🦀️dispatch.rs` — rewritten wholesale (owned by
  this packet).

No other file created, edited, or removed.
