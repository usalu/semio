# Scoped Commands Verification

## Passing checks

- `bun nx run @semio-tech/framework-rs:generate --skip-nx-cache`
- `bun nx run @semio-tech/framework-rs:check`
- `bun nx run @semio-tech/framework-renderer-react:lint --skip-nx-cache`
- `bun nx run @semio-tech/ui-react:lint --skip-nx-cache`
- React resolver aggregation targeted test: 1 passed.
- React owner-qualified collision-key targeted test: 1 passed.
- Taxonomy required OS/plugin command-facet targeted test: 1 passed.
- Taxonomy all command-owning scopes targeted test: 1 passed.
- `bun nx run @semio-tech/animate-plugin:test-quick --skip-nx-cache`: 228 passed.
- `bun nx run @semio-tech/framework-renderer-wgpu:lint --skip-nx-cache`
- `bun nx run @semio-tech/framework-renderer-wgpu:wasm --skip-nx-cache`; the Rust/wasm build and Trunk artifact sync complete. The run also exposed and fixed the stale artifact prefix in the existing target script.
- Filesystem audit: every OS, plugin, app, and mode has a `🎮️commands` facet; every window has a `🎬️actions` facet.
- Source audit: no remaining `CommandScope`/`CommandRef` declarations, serialized command scope, or newly added `[DEBUG]` instrumentation.

## Broader-suite observations

- `@semio-tech/repo-lib:test-quick` currently has 19 failures in concurrent repository taxonomy/workspace work. Both command-facet tests added by this ticket pass in isolation.
- The first WGPU quick run compiled successfully and ran 233 tests before fail-fast: 230 passed; the ticket exposed and then fixed stale fullscreen and command-panel assertions. One unrelated window-silhouette geometry assertion also failed.
- The WGPU quick rerun is currently blocked before this renderer's tests by concurrent Puzzle constructor edits: two `E0061` errors supply 19 arguments to constructors that now require 21 in Puzzle 5D and Puzzle 3D window components.
- The retained Flow plugin log records 15 passing tests before fail-fast on the pre-existing `delete_selection_action_removes_selected_synapses` assertion; 80 tests were then skipped by fail-fast. The command ownership migration compiles in the green Animate representative suite and the framework checks above.
