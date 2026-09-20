# OS Shortcut Routing Repair

## Proven defect

Checkpoint 8 delivered macOS `Control+Meta+F` through the normalized host keyboard lane. WGPU recognized it only in a special full-screen branch and passed an OS-owned invocation to `dispatch_command`, whose contract correctly refuses OS owners with `os commands must be dispatched by the shell`. The browser therefore received no full-screen directive.

The stale `mod+shift+f` entry came from React's control-badge map. React's executable OS command producer, `buildOsCommands`, declares `control+meta+f` for macOS and `f11` for Windows/Linux.

## Repair

- OS command shortcuts now use the same platform-filtered `resolved_commands` scan as other declared command bindings.
- An argless OS-owned match calls `apply_os_command` directly.
- An argless plugin/app/mode match retains the existing `dispatch_command` path.
- Any matched command with arguments retains the staged command-dialog/search path.
- The stale executable `mod+shift+f` row and the duplicate hardcoded full-screen branch were removed.

The shared neutral fixture is `🧫️fixtures/⌨️os-command-shortcuts/🔣️.json`, validated by `🧬️schema/⌨️os-command-shortcuts/🔣️.json`. The React oracle reads `buildOsCommands`. The Rust host-ingress law passes a real normalized `DispatchEvent::KeyDown` into `dispatch_normalized_event` and requires the shell full-screen latch with no guest action.

The host and React matchers now preserve Ctrl and Meta as separate requirements. On macOS, `mod+f` accepts exactly Meta+F while `control+meta+f` requires both modifiers. A shared collision vector proves that the fullscreen chord cannot be consumed by Find. React's matcher also accepts the canonical `control` spelling used by its own executable fullscreen declaration.

## Validation

- The first focused React run exposed that `control` was not recognized even though `buildOsCommands` emitted it. After repairing the real matcher, the focused Bun+Nx suite passed 1 file and 4 tests.
- Native Rust compilation and runtime/browser entry/exit remain root-owned; no Cargo or browser runtime claim is made here.
