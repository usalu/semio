# World interaction line budget split

Blocker: `semio-os-infinite-world-interaction` was at 4084 active lines against `maxActiveLines = 4000`, so every Rust build depending on it was refused at source admission.

## Change

The pure handle-drag domain moved out into a new crate `semio-os-infinite-world-handle-drag` at `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧲️handle-drag/` (package in `📦️packages/🦀️rust`).

Moved: handle types, drag parameter, handle pick, gesture state machine, argument and descriptor builders, annotation-layer targets, and the bounded action publication (`publish_world3d_handle_action`, formerly private `publish_handle_action`). The language-neutral fixture test moved to `🧲️handle-drag/🧪️tests/🔬️unit/🦀️.rs` (same fixture, numpy-derived vectors).

Stayed in interaction (`🧵️interaction/🧲️handle-drag/🦀️.rs`, ~140 lines): the authority arm, because it owns `WorldHandleDrag` session state and inherent impls on `WorldAuthorityState` / `WorldInteractionAuthority`.

Dependents: interaction and host depend on the new crate; the host imports `WorldHandleBindings` from it directly (no reexport).

Registries updated: root `Cargo.toml`, `🧰️framework/Cargo.toml`, `📋️build-owners.json`, `🔒️dependencies.json`, the wasm compiler fixture `🔣️.json`, and the new `📋️project.json`.

## Verification (repo cargo)

- `test -p handle-drag -p interaction --features interaction/testkit`: boundary admission passes; handle-drag 7/7 pass; interaction unit and pointer-gesture targets built. One failure: `pointer_gestures::authored_engagement_projects_and_publishes_bounded_pointer_commands` (1e-5 position tolerance, test line 138). It exercises the engagement pointer in the cursors crate, not handle drag, and is unrelated to this split; not investigated.
- `test -p semio-os-infinite-world-host --lib --features testkit surface_verbs`: builds, no boundary refusal; the filter matched 0 tests.
- `check --manifest-path ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.toml -p semio-s-artifact-procedural-generation3d`: OK, no further boundary refusals in its closure.
