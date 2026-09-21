# Shared Chrome Publication Fixture Audit

## Scope

Read-only audit of the shared normal-chrome test seam after the presented-input authority change. No build or test was run. The source establishes that completing a chrome walk only fills staging; only a later candidate seal and acknowledgement publishes input, retained owners, geometry, and accessibility together.

## Production boundary

`render_chrome_step` clears staging and begins the accessibility staging pass in `FrameSetup`, but deliberately does not publish it. The runtime must wait for GPU acceptance. [Shell](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:22759) documents that the staging rows remain resolvable from the prior accepted frame until `publish_retained_hit_registry` swaps them.

The actual promotion path is [Shell](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13028): it validates the witness, ACKs the UI candidate, swaps chrome owner maps and geometry, publishes `InputState` hits, promotes accessibility-visible documents, and records chrome accessibility. Test-only `publish_retained_hit_registry` is the correct single-frame substitute because it performs that same seal and ACK at [Shell](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:13064).

Do not make `render_chrome_step` publish on completion. That would make unsubmitted geometry interactive in production.

## Confirmed full-chrome fixture omissions

### Dense Actions

`publish_dense_actions_chrome` drives a complete `render_chrome_step` walk then returns its draw list without sealing or ACKing at [wgpu-window-actions-search-panes/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-window-actions-search-panes/🦀️.rs:58). Its caller immediately reads `input.hits()` and routes wheel/pointer events at lines 116–150. Those calls now correctly see only the previously accepted registry, or no registry on the initial call.

**Repair:** immediately before returning `draw`, call `shell.publish_retained_hit_registry(input)`. Keep `input.retire_hit_step()` before each replacement walk; it retires the prior input snapshot before minting the next candidate. This is a fixture-only acceptance of the draw the helper just completed.

### Palette and Find

`publish_palette_chrome` has the identical omission and one extra setup bypass: it begins its cursor at `Overlay`, completing only a suffix of the chrome frame, then returns without publication at [wgpu-shell-shortcuts-palette/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs:433). `FrameSetup` is the production owner that clears staged retained maps and begins the accessibility staging pass ([Shell](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:22759)). An Overlay-only helper can therefore mix a new dialog with previous staged rows even if it later ACKs.

**Repair:** use a default `ShellChromeFrameCursor` for the complete normal chrome walk, then call `shell.publish_retained_hit_registry(input)` before return. The test must not call `input.publish_hits()` directly: that would omit the UI tree ACK, modal geometry, retained-owner swap, and accessibility promotion.

## ToolRun is a distinct retained-document fixture

ToolRun does not use the normal chrome walk. `paint_panel` renders one retained document, calls `register_retained_hit_targets`, and returns only staged hit rows at [wgpu-tool-run-panel/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⏯️wgpu-tool-run-panel/🦀️.rs:48). The registration helper only invokes `input.register_hit`; it never publishes or accepts a UI candidate at [Interpreter](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:820).

The current ToolRun button tests consequently inspect `staged_hits` and call the generic `dispatch_ui_event` directly ([wgpu-tool-run-panel/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/⏯️wgpu-tool-run-panel/🦀️.rs:77)). This is a test-mode candidate-tree bypass, not evidence of a normal Shell input regression.

**Narrow repair if the intended law is presented input:** extend `paint_panel` to receive a test `ShellState`; begin the visibility pass before document paint, register the retained targets, then call `shell.publish_retained_hit_registry(&mut input)`. Derive controls from `input.hits()` and use the presented pointer/event entry point. This makes the test assert that only accepted ToolRun controls dispatch. If the test is retained-widget unit coverage only, leave it as a direct UI test and do not use it to diagnose a normal chrome failure.

## Reusable test helper contract

The lowest-risk shared helper belongs in the Shell test module beside the existing `ShellChromeFrameCursor` fixtures. It should accept an already configured `ShellState`, `InputState`, theme, bounds, and a complete chrome cursor; advance until `render_chrome_step` completes; then call only `publish_retained_hit_registry`. It must return the draw lists after the ACK and keep no candidate/witness across calls. Actions, palette, modal, panel, and dock tests can use it without copying a partial-paint shortcut.

## Verification laws

1. A completed normal chrome draw exposes no input before test ACK, then publishes one matching hit/accessibility/geometry snapshot after ACK.
2. Two Actions walks separated by scroll show only the current clipped rows after each ACK; the terminal row dispatches once through a pressed/released published hit.
3. A palette query updates rows only after its next ACK; outside press observes the accepted modal geometry and does not reach a background world.
4. A presented ToolRun panel exposes enabled controls after ACK; disabled Step never captures focus or dispatches, and a replacement/closed candidate cannot accept a prior row's release.
