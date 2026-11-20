# WGPU Ink Pointer Leave

WGPU Ink now publishes the shared `interactionHover` clear when the pointer leaves the retained pane. The existing scene queue orders the clear after any pending hover work for the same retained window generation. The action uses the mounted surface ID and the editor’s interaction domain. The Ink scene keeps the last published hover until the clear is admitted, so a repeated leave is quiet even if the guest still projects the old hover. A read-only Navigator cannot publish pointer hover.

The WGPU browser host recognizes DOM `pointerleave` for mouse and pen. The browser transport coalesces motion and leave samples by pointer identity, so rapid re-entry supersedes an older exit. The native WGPU and shared native hosts normalize winit `CursorLeft` into the same dispatch event. A physical leave retires retained hover without releasing the gesture capture. Inside the renderer, a move outside the Ink component or onto a specialized scene uses the same retained scene command.

## Verification

- Browser input wire and browser frame transport suites passed all 92 tests with the renderer WGPU Vitest configuration. The suites cover the browser wire fixture and coalescing in both move/leave orders.
- All 19 changed Rust files passed the pinned nightly parser with `rustc -Z parse-crate-root-only --edition=2024`. The Rust regression reads the same neutral Ink tool fixture as React and covers exact action arguments, repeated leave, and Navigator silence.
- `repo tools exec git -- diff --check` passed for the changed source and fixture paths.
- The Rust shell regression build was cancelled after its cold dependency build began. It admitted 843 packages and reserved 3.5 GiB on the shared host. The Rust regression and live native/WGPU paths remain unverified at runtime.

## Source Paths

```text
🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🏗️router-construction-and-intents/🦀️.rs
🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🖱️accessibility-and-pointer-dispatch/🦀️.rs
🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🧭️event-model-and-focus/🦀️.rs
🧰️framework/🔨️modules/🖱️ui/🖌️render/🖱️dispatch/🎬️fire-cursor-and-dispatch/🦀️.rs
🧰️framework/🔨️modules/🖱️ui/🖌️render/🖱️dispatch/🫧️events-bubble-and-capture/🦀️.rs
🧰️framework/🔨️modules/🖱️ui/🖥️host/🪟️window/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-host/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎮️input-wire/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/📋️contract/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🖱️dom-events/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🎮️interaction-state/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🚀️native-winit-app/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎮️wgpu-browser-input-wire/🟦️.ts
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎮️wgpu-browser-input-wire/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧫️fixtures/🎮️wgpu-browser-input-wire/🔣️.json
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🏁️ink-interaction-job-publish-and-close/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/📤️ink-event-pages-and-edit-routing/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/✒️ink-canvas-model-state/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🗃️scene-state/🧯️surface-state-and-retirement/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/🖱️pointer-input/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🖱️scene-interaction/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/🎬️scene-intents-and-clipboard/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/📋️receipts-drop-and-clipboard/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/✒️ink-gestures-and-closing-owners/🦀️.rs
```
## Capture Ownership Follow-Up

A pointer exit now checks the capture owner of the exact retained window before changing its hover route. A leave from a different pointer preserves this route, so the owning pointer can still clear hover. The existing captured-slider regression now verifies per-window ownership during a competing pointer exit and after releasing capture. All five Rust source files parsed with the pinned nightly compiler and passed the whitespace check. The native regression remains pending.

```text
🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🔬️introspection-queries/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/📕️close-scene-pointer-and-dispatch/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/🖱️pointer-input/🦀️.rs
🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🧱️engine-fixtures-and-scene-lifetime-laws/🦀️.rs
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/🦀️.rs
```
