# Framework testing-caller migration

## Done
- `semio-plugin-testing-caller` (`🧪️testing/🎟️caller/🦀️.rs`, `👷️work/🦀️.rs`): `check --all-targets` clean (exit 0).
  - `OriginalFixtureCaller` now delegates birth, turns and retirement to the framework `OriginalJobFixtureCaller` (effect-threaded source admissions, resident parent). No private copy of the old birth sequence remains.
  - Added `OriginalJobFixtureCaller::lend_context` and `settle_lent_fuel` (framework job testing caller) for the one long-lived lent context.
  - `retire` / `retire_decoder` return `OriginalTurnFailure`; closes use `close_original_effect_step` with `owner.step(body, effect)`.
  - Work fixture: `settled` (effect-shaped admission/take) and `receive_scope` (`with_native_receiving` + `source_witness` + `advance`); receivers are closed through `retire_owned`.
  - Cargo: `semio-framework-job` gets feature `testkit`.
- Linking the testkit pulls in `semio_framework_value_testing_allocator` (global allocator, a superset of `HeapWitness`). Removed the now conflicting `install_fixture_heap_witness!` macro, its re-export and its 22 call sites under `✏️s/🔌️plugins/**`, and the two `#[global_allocator]` in runtime-ops (`🦀️.rs`, `🧪️tests/original-view-read.rs`).
- `semio-plugin-runtime-ops`: `check --all-targets` clean; `test` (default features) 6 passed.

## Blocked / others' crates
- `test -p semio-plugin-runtime-ops --features original-actor-context-testing` (needed for `removed_surface_view_retires_within_the_next_funded_turns`): `semio-plugin-host-configuration` fails, `👷️job/📨️emission/🏛️host/🦀️.rs:77` uses `semio_framework_trace` (optional dep, only under `testkit`) in a `[DEBUG]`-style `eprintln` line, not mine.
- `semio-plugin-instance-dispatch` tests blocked by `semio-plugin-original-job-ownership`: `🔌️plugin/📜️declaration/👷️job/🎟️birth/🧪️tests/🦀️.rs` lines 14, 29 (`OriginalCallerSourceParent::admit` old signature), 52, 55 (needs `ResidentConstruction`), 65 (`close_step` needs effect; `RetainedWorkPoll`).
- Other test binaries that declare their own `#[global_allocator]` (e.g. `✏️s/🔌️plugins/🏗️fem/⚙️engine/📍node/🦀️.rs:67`) and link `semio-plugin-testing-caller` will now conflict; remove them the same way.
- A source-admission race ("Cargo.toml has N bytes where M were admitted") appears when peers edit the root manifest mid-run; retry.
