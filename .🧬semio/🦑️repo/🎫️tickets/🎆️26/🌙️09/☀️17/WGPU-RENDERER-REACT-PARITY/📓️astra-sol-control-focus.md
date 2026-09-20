# Astra Sol Control Focus

Implemented the first four control gaps from `📓️terra-controls-final-audit.md`. The packet is limited to accessibility focus ownership, retained Select keyboard routing, and blur-committing Input transport. Stepper cancellation remains in the root packet.

## Result

### Accessibility focus now owns the Shell keyboard route

The retained accessibility dispatcher now returns the exact `Vec<UiCommand>` produced by `EventRouter`. Shell consumes `FocusChanged`, updates `ShellChromeBuildState.content_focus`, and makes the owning non-panel window active. A focus command therefore establishes the same retained keyboard owner as pointer and keyboard dispatch.

The Shell host law publishes a real retained Select document, focuses it through `AccessibilityEvent::Focus`, and verifies the resulting content-focus and active Select ownership before any key is sent.

### Select Space and printable typeahead use `KeyDown`

The Shell key mapper now consults the focused retained node kind. A focused Select receives printable characters and the Space press as `UiEvent::KeyDown`; an editable Input continues to receive printable characters as `UiEvent::TextInput`. Space release stays outside the retained Select state machine.

The renderer's Space interception now yields the press to Shell when a retained Select owns keyboard focus. Existing context-menu hold behavior remains intact. The Shell host law proves focus → Space open → Space commit and focus → printable `d` typeahead → Enter commit against a published `framework.settings.appearance` Select, including the final `dark` value.

The neutral browser keyboard fixture adds explicit read-only combobox Space and printable cases. Both keydown and keyup cross the real `wireBrowserKeyboard` owner, while the existing editable Input case remains native text input. React's production Select component law remains the independent implementation oracle for Space and locale-invariant typeahead.

### Accessibility blur commits a staged blur Input exactly once

Added `AccessibilityEvent::Blur` through the browser mirror, bounded lossless transport, Rust wire decoder, worker preflight, Shell, and retained `EventRouter`. The mirror emits Blur from a genuine DOM blur listener and suppresses refresh-induced synthetic focus and blur while it rebuilds the mirrored tree.

`AccessibilityUiEvent::Value` now updates the retained edit draft without emitting an action when the Input declares `commit: "blur"`. Blur clears focus through the existing `FocusState` boundary, which emits the one commit action and `FocusChanged { node: None }`. Repeated Blur on the already-blurred node emits no action.

The production mirror browser law focuses the actual generated `<input>`, dispatches a real bubbling `input` event, and moves DOM focus to a button. It expects `accessibility-focus`, `accessibility-value`, `accessibility-blur`, `accessibility-focus` in order. The retained-engine law separately proves no `setDriverSaveLabel` action before blur, one completed-value action on blur, and zero actions on repeated blur.

## Laws

- `retained_key_mapper_separates_select_typeahead_and_space_from_input_text`
- `accessibility_focus_arms_shell_select_space_and_typeahead_routing`
- `accessibility_blur_input_stages_values_and_commits_exactly_once_on_blur`
- `decodes accessibility blur with bounded node identity` in the Rust browser input-wire suite
- `carries an addressed accessibility blur on the bounded lossless lane`
- `transports a real editable blur after the final staged accessibility value`
- Neutral browser keyboard cases `combobox-retained-space` and `combobox-retained-typeahead`
- Existing React Select law `supports Home, End, Page, Space, and locale-invariant typeahead`

## Validation

The root-owned native lane received a sealed Rust source marker before Native33. This agent did not run Cargo, native builds, WGPU activation, or renderer artifact targets.

- WGPU production accessibility mirror and bounded browser wire: **2 files, 36/36 tests passed**.
- React browser keyboard scope, including the neutral Select Space/typeahead cases and editable Input split: **1 file, 36/36 tests passed**.
- Production React Select independent Space/typeahead oracle: **1/1 selected test passed**; 12 unrelated tests skipped by the exact name filter.

The first ordinary Nx attempt waited on a concurrently invalidating project graph and was cancelled before target execution. A daemon-free target retry automatically ran its registered schema/UI generation dependencies, then rejected unsupported forwarded Vitest flags before executing tests. The final receipts used root's bounded `nx exec` path with cached graph reuse and exact test/config paths.

## Source Boundary

- UI render accessibility event and host admission: `🧰️framework/🔨️modules/🖱️ui/🖌️render/🖱️dispatch/🦀️.rs`, `🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️enqueue/🦀️.rs`
- Retained focus/value/blur state machine and law: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs`, `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs`
- Shell command ownership, key mapping, renderer Space routing, and law: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs`
- Browser transport and mirror: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎮️input-wire/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs`
- Browser oracles: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/♿️wgpu-accessibility-interaction/🟦️.tsx`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎮️wgpu-browser-input-wire/🟦️.ts`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎮️wgpu-browser-input-wire/🦀️.rs`, `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/⌨️browser-keyboard-scope/🔣️.json`

Shared concurrent palette accessibility fields and branches (`editable`, `controls`, `activeDescendant`, editable-combobox key ownership) and the root pointer-cancel packet were preserved.

## Native34 Follow-up

The exact Native34 Select capture law exposed an ownership error in refreshed layout: a synthesized option retained its popup-relative physical rect during document reconciliation, then the next mounted-layout solve classified it as an ordinary in-flow `Button` and overwrote that rect. `LayoutNodeKind::OverlayRow` now carries the already-resolved parent-relative popup rect through the bounded solver as a fixed absolute row. It remains in the retained text pipeline, so glyph-run and line-credit accounting stay balanced.

The dense Actions law constructs its replacement app action catalog after `ShellState` construction. It now runs the same `refresh_window_action_panes` publication boundary as `refresh_ui`, asserts that publication is fault-free, and then keeps every existing exact row, clipping, scroll, paint, and dispatch assertion. This makes the test exercise the mutated live catalog rather than the stale pane document produced by the original fixture constructor.

Source-only audit passed `git diff --check`. Per the root-owned native lane rule, this agent did not run Cargo; the Native34 laws remain pending validation by the root gate.
