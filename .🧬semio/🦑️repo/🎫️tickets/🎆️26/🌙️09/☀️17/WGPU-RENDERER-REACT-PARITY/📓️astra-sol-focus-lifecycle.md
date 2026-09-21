# Retained Focus Lifecycle

## Finding

The first validity check was insufficient. Shell recorded the focused surface and `NodeId`, and `Interpreter::retained_node_has_visible_focus` required a visible surface, router focus, and a node at that id. Native70 supplied the actual counterexample: after document revision 1 was replaced by revision 2 with a different authored key, the fresh arena reused `NodeId { index: 0, generation: 0 }`. Existence plus router focus therefore routed the replacement as though it were the focused predecessor. The exact law failed at line 190 before a keyboard gesture.

The repair retains a typed `RetainedContentFocus { node, key }` from the real `UiCommand::FocusChanged`. Interpreter captures the focused node's `NodeKey`, and keyboard dispatch now requires the current node at that arena id to have the same reconciliation key. This preserves the intended lifecycle without a synthetic focus event:

- A same-surface successor with the same authored key keeps the node and key identity, so focus and keyboard routing survive publication.
- A successor that removes or replaces the key can reuse an arena id but fails the key equality check, so it cannot receive the predecessor's keyboard input.
- A hidden panel is refused by the Shell's panel visibility check.
- A surface omitted from the published accessibility visibility roster is refused even if its old engine tree and focus record still exist.

The fix is confined to the Shell focus record and Interpreter focus identity methods; scene routing and document publication are unchanged.

## Registered native laws

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs` now adds and strengthens three bounded laws:

1. `same_surface_keyed_successor_retains_keyboard_focus_without_a_synthetic_focus_event` publishes and paints an initial Select, focuses it through the accessibility event boundary, publishes and paints a changed successor with the same key, and proves the exact focused `(surface, NodeId)` and physical Space dispatch survive without another focus event.
2. `successor_that_removes_the_focused_key_cannot_receive_retained_keyboard_input` replaces that key in a real successor, proves the Shell received no invented blur, and proves the stale node generation cannot route Space into the replacement.
3. `retained_panel_focus_routes_keyboard_without_activating_an_application_window` now proves both hidden-panel and omitted-surface closure reject physical Space dispatch. Restoring the same visible owner before closure recovers focus without a synthetic event.

Every law uses `publish_surface_records`, retained document paint/reconciliation, accessibility focus dispatch, and bounded visibility publication. Every acquired `UiDocumentLease` is closed through its exact close ladder. The existing neutral window/surface owner fixture remains sufficient; no capacity or schema change is needed.

The modified Rust test source parses through Rustfmt and passes `git diff --check`. No Cargo command was run locally. The root-owned exact filters are:

```text
same_surface_keyed_successor_retains_keyboard_focus_without_a_synthetic_focus_event
successor_that_removes_the_focused_key_cannot_receive_retained_keyboard_input
retained_panel_focus_routes_keyboard_without_activating_an_application_window
```

Native70 actual RED receipt: `successor_that_removes_the_focused_key_cannot_receive_retained_keyboard_input` failed at line 190 because the replacement still routed under the reused `{ index: 0, generation: 0 }` id. The same-key law was not among the reported failures. The key-qualified production repair was applied after that receipt; post-fix execution is pending the root-owned native lane.

Native71 verified that repair through Nextest `aa0cc1da-31cb-4d8f-bcd9-4a110cd921b1`: five selected, three passed, two failed, 1,256 outside the filter, in 247 ms. The same-key Select successor, removed-key refusal, and closed/hidden panel laws passed. The held `same_key_changed_control_kind_cannot_inherit_retained_keyboard_focus` law supplied its intended actual RED at line 218 because a Select→Input replacement under the same key still inherited focus. The other failure belongs to the separate capture packet.

## Same-key control-kind audit

React's actual DOM reconciliation preserves a focused node when both the authored key and host element kind remain `select`, then replaces that node and moves focus to `document.body` when the same key changes from `select` to `input`. The ticket-scoped Vitest oracle is `🔬️focus-lifecycle/🟦️.ts`, with config at `🔬️focus-lifecycle/🎚️config/🟦️.ts`.

```sh
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx exec --projects=workspace -- bunx vitest run --config '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️focus-lifecycle/🎚️config/🟦️.ts'
```

Receipt: Vitest 4.1.10, 1 file and 1 test passed, 30 ms test time, 2.05 s total. The new native law `same_key_changed_control_kind_cannot_inherit_retained_keyboard_focus` publishes and focuses a Select, publishes an Input under the same explicit key, and requires that the replacement cannot inherit focus or dispatch its Enter commit.

After Native71's RED, the retained identity became the closed triple `(NodeId, NodeKey, RetainedNodeFocusKind)`. `RetainedNodeFocusKind` exhaustively maps every `UiNode` variant, so adding a component kind requires an explicit focus identity decision. Keyboard routing now requires the current node to match both key and component kind. The change addresses the proved Select→Input boundary; it does not claim parity for configuration changes inside one component variant, such as an Input that may change its host element kind.

The older focus-tracker fixtures no longer inject unresolvable `FocusChanged(Some(arena_id))` commands. Pure map-clear and window-scope laws inject an intentional typed `RetainedContentFocus`; the retained-body activation law now publishes and paints a real document, focuses its real keyed node through accessibility dispatch, and closes its exact lease.

Native78 executed `same_key_changed_control_kind_cannot_inherit_retained_keyboard_focus` successfully in Nextest run `8d00db48-8e7d-4ec0-8892-e8d14063c596`. The full renderer census reached 1,257 passing laws before ten unrelated scene-paint workers were terminated during the separately diagnosed clipped-hit-target helper deadlock. This focus law completed before that hang.
