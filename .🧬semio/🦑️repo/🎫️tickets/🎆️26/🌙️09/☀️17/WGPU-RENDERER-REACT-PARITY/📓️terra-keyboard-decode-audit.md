# Browser Keyboard and Decode Audit History

## First Audit — Browser Keyboard Focus Ownership

The sealed checkpoint-13 browser run exposed a focus-dependent fullscreen failure. A boot/dismiss/enter/exit sequence passed in React and WGPU, but close-all, reopening a World body, then orbiting, panning, and zooming caused both later WGPU fullscreen transitions to fail. In the failing path trusted shortcut keys targeted an accessibility-mirror `DIV`; in the passing boot path they targeted `CANVAS#semio-wgpu-canvas`.

The initial audit identified canvas-only keyboard listeners as the ownership defect. The renderer root is the correct owner: it accepts the canvas and accessibility mirror while leaving native button Enter/Space activation, editable values, ordinary input handling, and outside-root events untouched. Combobox keys remain retained-control events, so native defaults are prevented there to avoid a second browser action. Composition and already-consumed events remain excluded.

The language-neutral keyboard fixture covered canvas, tree focus, native buttons, editable inputs, comboboxes, outside-root scope, and consumed events. The actual DOM coverage exercised user-event typing, Tab, and button activation plus the accessibility-mirror refresh that reproduces the canvas-to-DIV focus transition. The implementation owner reported 20/20 for the new scope suite and 49/49 across the three related suites before the later focus-loss cases. Browser runtime acceptance remained checkpoint-14 work.

The companion reference-decode audit is retained in `📓️astra-sol-reference-decode.md`, including its detailed initial negative-path addendum for source allocation, staged-token eviction, and cancellation.

## Modifier Reset Follow-up

### Scope

Read-only review of `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎮️input-wire/🟦️.ts` after the modifier-reset change. This audit did not run tests.

### Verdict

The reset path correctly removes stale modifier state when focus leaves the renderer root, the browser window blurs, the document becomes hidden, or the wire is disposed. `heldModifiers: string[]` is source-type correct: the admitted browser keyboard-event `key` is a DOM `KeyboardEvent.key` string, and the tracked values are those same strings.

The focus transition is race-safe for the intended ownership boundary. `focusout` defers its decision to a microtask and tests the final `document.activeElement` against `root.contains(...)`. A canvas-to-accessibility-mirror focus move therefore preserves held modifiers. A focus move outside the root emits one neutral `keyup` for each held modifier.

`reset()` copies and clears `heldModifiers` before admission. Consequently a `blur` or hidden-document reset that arrives before the queued focusout leaves that microtask with nothing to emit. Disposal first marks the wire disposed, then resets and unregisters every listener, so a queued focusout cannot publish after cleanup. The registered document event is `visibilitychange` with `visibilityState === "hidden"`; it is not a separate `pagehide` listener.

The reported fixture and user-event run reached 55/55 after the three focus-loss cases were added. That result is reported by the implementation owner; this audit performed no test run.

### Remaining Boundary Gap — P2

Modifier state is still reconstructed only from keyboard edges that reach the renderer root. A user can press Control or Shift outside the root, move the pointer into the canvas while keeping it held, and click or drag before any keyboard edge reaches the root. The wire has no held modifier for that gesture.

`🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs` uses `app.modifiers` for normalized pointer move, down, and up dispatch (lines 343–390). Those pointer variants do not independently carry browser modifier flags. The modifier-reset repair removes stale positive state, but this entry path yields a false negative for modifier-plus-pointer gestures.

Minimal reproduction:

1. Focus an element outside the renderer root.
2. Hold Control or Shift without sending a key edge to the root.
3. Move into the WGPU canvas and perform a pointer down or drag while still holding the modifier.
4. Observe that the normalized pointer event receives the unmodified `app.modifiers` state.

If browser-native pointer modifier parity is required, add modifier fields to the browser pointer event schema, wire each DOM pointer event's modifier flags, and use them for normalized pointer dispatch. Add the corresponding outside-root modifier gesture as a fixture. This is independent of the repaired reset lifecycle and does not justify changing that lifecycle.
