# FEM 3D interaction + panels tests

Run: `cargo test -p semio-s-artifact-fem-3d-editor-interaction -p semio-s-artifact-fem-3d-editor-panels --lib` (native cargo, `C:\t\3dy`, script `a3d-ip-test.sh`, log `🗑️generated/ip-test.log`; repo binary not used).
Result: interaction 19/19 passed (was 2/19), panels 6/6 passed (was 0/6). rc=0.

## Root causes and fixes
- Snapshots dropped without retiring (14 interaction tests): added a test-only guard `Retiring<T>` (retires through `OriginalWorkFixture`) in `🕹️interaction/🧪️tests/♻️retiring/🦀️.rs`, wired in `🕹️interaction/🦀️.rs`; used by `🧪️tests/🔬️unit`, `🧪️tests/👥️identity`, `🧭️gumball/🧪️tests/🔬️unit` for every sealed snapshot (demo, moved, folded, preview, decoded, applied).
- Lowercase DOF fixture: already fixed in `🎮️commands/🗂️remove-selection/🧪️tests/🧫️fixtures/🔣️.json` (by the editor agent); the identity tests include it, so no change needed (nodal load `dof: "tz"` is accepted).
- Panels: 6 tests lacked the UI arena and retired nothing; `📌️panels/📊️results/🧪️tests/🔬️unit/🦀️.rs` now builds the boot snapshot through a `Boot` guard that holds `FixtureUiArena` and retires the snapshot.
- `the_transform_utility_is_read_from_the_addressed_window`: its last assertion predated the framework's exact window ownership (`ViewModel::window_utility`: a per-window entry beats the global `active_utility_id`). The test now states both facts (global brush does not disarm the window; a per-window brush does) and retires its ViewModel.
- Gumball history replay: the store needs 10562 retire steps for the 3D demo; the loop bound of 4096 was copied from 2D. Raised to `1 << 16`.

## Framework root cause outside scope
`🧰️framework/.../🔌️plugin/🧵️instance/🧱️construction/📦️packages/🦀️rust/Cargo.toml`: sources file (edited 13:22 by someone else) uses `semio_plugin_command` without the dependency, which broke every build; added `semio-plugin-command` path dependency (one line).

## Open
Nothing in scope. No `[DEBUG]` prints remain.
