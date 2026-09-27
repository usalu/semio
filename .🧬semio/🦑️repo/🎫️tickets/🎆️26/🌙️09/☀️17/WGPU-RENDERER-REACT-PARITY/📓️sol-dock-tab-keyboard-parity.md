# Dock Tab Keyboard Parity

## Result

The accepted accessibility projection now carries `tabbable` separately from `focusable`. Every Dock tab remains focusable, while only the selected tab in each stack is in the browser tab order. Named Dock body scroll regions publish nonfocusable `tabpanel` nodes, so every tab's `controls` relation resolves to an accepted node.

The browser mirror keeps roving focus inside the exact tab group identified by `aria-controls`: Left/Right wrap, Home/End select the edge, and focus movement does not select. Enter/Space enqueue the existing exact addressed `accessibility-activate` event and stop the untargeted canvas key path. Shell activation now resolves a Dock tab label to its live window ID; focus alone only records accessibility focus. Existing generation plus node-id/key validation rejects stale addresses before either path runs.

## Contract and tests

The language-neutral schema and fixture are:

- `engine/🧬️schema/⌨️dock-tab-keyboard/🔣️.json`
- `engine/🧫️fixtures/⌨️dock-tab-keyboard/🔣️.json`

They cover a three-tab stack beside a two-tab stack, one tab stop per stack, Left/Right wrap, Home/End, Enter/Space, exact group boundaries, and a stale generation.

The actual React oracle mounts production `Mode` and measures its real tab buttons. Its observer stubs are scoped to the test and removed after unmount. The browser mirror oracle consumes the same fixture. The native law `accepted_dock_tabs_publish_one_tab_stop_per_stack_and_activate_only_an_exact_address` consumes it in `wgpu-shell-input` and checks the accepted projection plus exact/stale dispatch.

## Validation

- RED: all five projected Dock tabs had `tabIndex=0`; production React initially exposed the test environment's missing `ResizeObserver`.
- PASS: focused production React plus mirror laws, 2 passed and 18 skipped. Log: `🗑️generated/sol-dock-keyboard-green-2.log`.
- PASS: full WGPU accessibility interaction TypeScript suite, 20 passed. Log: `🗑️generated/sol-dock-keyboard-full-2.log`.
- PASS: TypeScript accessibility projection twin, 263 checks. Log: `🗑️generated/sol-dock-tabbable-contract-ts.log`.
- PASS: `rustfmt --emit stdout` parsed the modified Shell source and `wgpu-shell-input` test through scoped Nx execution. Log: `🗑️generated/sol-dock-keyboard-rust-parse.log`.
- A complete Rust accessibility projection literal census found 11 constructors and no missing `tabbable` field.
- Native execution is queued with the parent build coordinator; no native pass is claimed here.
