# WGPU Runtime Routing Audit, September 29

Read-only source audit. No tests or live runtime were run by this audit; no runtime parity claim is made.

## Correct Implementation Boundary

The actual OS shell is `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, not the UI module's small retained shell. The OS shell's `ShellState` starts at line 3746; `render_chrome_step` starts at 25535. Dock implementation is adjacent under `🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs`: `DockState` line 107 and `compute_dock_drop_zone` line 1127.

OS drag admission exists at Shell line 7937. Exact normalized tab `.drag` hit routing exists at 15009 and stack drag routing at 15022. Move/release update zones at 15183 and 15239. Thus the unused UI retained-shell drag stubs are not evidence that product docking is missing. No `dispatch_shell_event` / `ShellEvent` use was found under the OS renderer.

The standalone UI retained shell (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🐚️shell/🦀️.rs`) does store-only navbar labels at line 130 and emits only tab activation in dispatch at 162. Treat those as separate library completeness gaps until an effective product route is demonstrated.

## Existing Reports Must Be Rechecked Against Current Source

`📓️astra-remaining-chrome-projections.md` reports missing conflict inline buttons/detail and marketplace installation controls. Current Shell source has conflict toolbar lifting and semantic DiffView detail at 9338-9395, marketplace verbs beginning at 9441, file/URL installation rows at 9527/9534, and action routing at 11939-11972. Those older reported gaps are not still established by source.

`📓️astra-sol-dock-interactions.md` documents eight physical docking fixture cases. Its earlier WGPU receipts stalled at generation 1 even though pointer input reached the correct hit path. It explicitly requires fresh browser acceptance after worker wake fixes. Prioritize a fresh physical run before editing already-present drag/drop logic.

## Runtime and Validation Routes

Native entrypoint is `🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs` (native main line 12). Its script consumes already-completed runtime artifacts and runs the native binary with boot axes; build/activation is owned by the dev Nx graph.

Native event bridge is `🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs`: `handle_event` line 150 enqueues bounded input; `handle_metrics` 163 queues metrics and resize; `redraw` 181 calls redraw_core. Winit normalization begins at 840. The mounted runtime is worker/frame driven; do not infer frame progress solely from event dispatch.

Registered commands:

```sh
bun nx run @semio-tech/framework-renderer-wgpu:test-wgpu-unit
bun nx run @semio-tech/framework-renderer-wgpu:test-native
bun nx run @semio-tech/ui-rs:test-wgpu-engine
bun nx run @semio-tech/ui-rs:check-wgpu-engine-wasm
bun nx run @semio-tech/framework-renderer-wgpu:serve
```

Renderer `test-wgpu-unit` runs the crate's library tests with a long budget; source documents that short budgets previously killed the suite without a red test. Renderer `test-native` is wider. UI engine cases are also mounted library tests. These commands are verified against project registrations and scripts, not executed.

Physical docking adapter lives at ticket `🔬️dock-interactions/📜️script.ts`; its report provides run commands and fresh-context requirements. Its existing direct bun commands should be routed through a ticket Nx target if execution is performed under the user's Nx-only runner requirement.
