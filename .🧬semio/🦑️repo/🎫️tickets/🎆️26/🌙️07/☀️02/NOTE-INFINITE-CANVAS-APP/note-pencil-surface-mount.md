# Pencil surface mount ownership

Attempt 52 loaded the Note guest and reached the editor. Selecting Pencil panicked before a gesture could publish. The failing Wasm was 99,273,224 bytes.

The browser console identifies ordered-map destruction in function `30342`. Function `85729`, the `ViewModel` destructor, calls it twice. The dispatch function `85914` identifies `Event::SurfaceVisible` and its inlined `plugin_mount_surface` through the source location and the surrounding surface-context diagnostics. Its destructor call at `0x21ab79f` drops the decoded full `ViewModel`. `.for_window_instance` returns a clone, and the source view is abandoned before it reaches the surface registry. Pencil puts a value into the per-window utility map, which triggers the ordered-map retirement invariant.

`plugin_mount_surface` now acquires the exact instance before decoding, then transfers the original owned view directly into `SurfaceContexts::insert`. The registry already funds retirement of refused and displaced views. It creates the binding projection once and copies its two scoped string values into the retained original view for instance-level background work. An app-level panel or reserved section still uses its full host context.

The reactor scan found one other production contextual projection used to retain a mounted window binding and one independent context-menu wire projection. The browser's original Manifest refresh path already stages and retires its host-view owners. The only direct `plugin_refresh_ui` caller found was a native test, and its pending-effects clone was fixed in the preceding change.

## Files

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🖥️surface-render/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🪟️surfaces/🧪️tests/🪟️surface-context-lifecycle/🦀️.rs
- `note-owned-mount.test.ts` in this ticket.

## Validation

The native regression consumes the existing language-neutral surface-context fixture. It transfers a deserialized view with a nonempty ordered map and deliberately stale scalar fields, verifies exact mounted and retained window routing, and drains the registry after a refused unknown window. SQLite independently joins the fixture's window IDs to its utility map. The Bun test also checks the producer-to-registry transfer in the actual source.

`repo tools exec --light bun -- test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️02/NOTE-INFINITE-CANVAS-APP/note-owned-mount.test.ts'` passed both tests with 11 assertions. A generated Bun preload supplied the unchanged Git HEAD mount source without changing the shared worktree. The SQLite oracle passed and the custody assertion failed on the original order, as expected. The full Rust run and browser acceptance are pending the coordinated guest rebuild.
