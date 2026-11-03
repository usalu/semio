# Scene Check 67

The scene library `semio-s-artifact-draw-drawing-scene` still fails `repo cargo check`. Check 67 is `/tmp/draw-scene-check67.txt`, EXIT 1, 45 errors.

Crates that compiled past rustc in this stretch: retained-command, scrub-host, command, publication-mounted, reactor-lifecycle, typing-edit.

Still blocking:

- `semio-framework-plugin-runtime-instances` — 15 errors in `retirement`
- `semio-framework-plugin-time-travel` — 30 errors across `store-state/finalize`, `opening`, `cleanup-turns`, `decision/discard`

The scene crate itself has not been rustc'd. Native and wasm checks of `semio-hub-draw`, lib tests, draw-js tests, describe/registry, and the React host are still ahead.
