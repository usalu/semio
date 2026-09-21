# WGPU 20 Accessibility Mirror Activation Audit

## Evidence and scope

This is a source-only audit. The recorded physical journey at `🗑️generated/astra-runtime/checkpoint20-iab/journey-physical.json` exposes WGPU switches named `Anzeige` and `Einstellungen`. The parent recorded that a physical click on `Anzeige` changed the panel, while Playwright's semantic `getByRole("switch", { name: "Anzeige", exact: true }).click()` and the corresponding Settings switch click returned successfully without a visible change after ten seconds.

## The mirror is deliberately visually hidden, but it is an activation surface

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts:37-43` creates a one-pixel clipped region. That makes the mirror non-painting; it does not make it pointer-coordinate driven, inert, or keyboard-only.

For an actionable switch the same module creates a native `button type="button" role="switch"` (`:50-59`). Its `click` handler enqueues a node-addressed `accessibility-activate` event containing `windowId`, `windowGeneration`, `nodeId`, and `nodeKey` (`:109-119`). It carries no canvas coordinates. Focus, blur, and editable value events use the same lossless route (`:110-122`).

The browser transport accepts that event into its lossless queue and requests a frame at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts:532-565`; input wire maps it to an accessibility dispatch event. The native route reaches `Shell::handle_accessibility_event` through the renderer host. Thus a role click is meant to exercise the real AX action lane. The physical success only establishes the separate hit-testing lane.

## Concrete stale-snapshot seam

A frame ACK does three related things in `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13029-13053`: it promotes interpreter input, publishes `input.hits()`, and derives both the chrome accessibility projection and chrome-hit registry from that promoted hit slice.

The later AX handler does not retain that projection. At `:13478-13502` it accepts only the fixed chrome generation and recomputes `chrome_accessibility_nodes(input.hits())`, rejecting a node/key that is absent in the newly derived vector with `Ok(false)`. The successful `bool` is intentionally not an error, so the browser can observe a successful DOM click and no state change. Node ids are re-enumerated from the current filtered hit slice in `:29654-29740`; their chrome generation is a fixed interpreter constant rather than the acknowledged input epoch.

The final dispatch is otherwise direct: `ui.panelToggle.display` calls `toggle_anchor(BottomLeft)` at `:13784-13786`, and `ui.panelToggle.settings` calls `toggle_anchor(BottomRight)` at `:13797-13800`. This supports an upstream addressing/validation failure, not a missing action implementation.

The source does not prove that an input registry changed in the specific recorded ten-second interval. It does prove that a previously announced chrome address has no exact presented-snapshot witness and can be silently refused after registry reordering or replacement. That is the smallest source-level explanation consistent with both semantic failures and physical success.

## Bounded repair

Publish a bounded chrome accessibility registry with the same ACK as the hit registry. Each entry must bind:

- the acknowledged `presented_input_epoch`;
- the published `nodeId` and `key`;
- the immutable resolved `HitTarget` or its exact control/action identity.

Have the AX handler resolve only that registry and reject a mismatched epoch before executing an action. Retire/replace the registry together with the next presented input snapshot. This uses the already bounded resolved input authority and avoids recomputing an address from a later hit ordering.

A narrower interim variant is to use `presented_input_epoch` as the chrome accessibility generation and verify it in the handler. It still re-derives node ids from `input.hits()`, so it is weaker than storing the presented registry.

## Fail-first law

Add a browser-to-worker behavioral case, not a mirror-unit test:

1. ACK a frame exposing the exact `Anzeige` and `Einstellungen` switches.
2. Invoke `getByRole("switch", { name: "Anzeige", exact: true }).click()` with no canvas coordinate event.
3. Require exactly one transport activation and a subsequent accepted frame with the Display state changed; repeat for Settings.
4. Preserve an element address, ACK a replacement chrome registry that removes or reorders it, then require its delayed activation to be refused with no action.

The present `wgpu-accessibility-interaction` browser test covers fake `enqueueLossless` emission only. It does not execute the browser transport, worker dispatch, Shell action, and next presented-frame assertion required to distinguish this regression.

