# Remove Orphaned `export_wasm_machine`

## Completed

- Removed the public `export_wasm_machine` proc-macro entry point, its parser and expansion implementation, and its unit test from draw FSM's macro crate.
- Removed the stale runtime documentation reference to `export_wasm_machine!`.
- Confirmed that the FSM crate does not re-export the proc macro; its only macro-related public surface remains `pub use component::*` for its own runtime implementation.

## Verification

- `grep -rn --exclude-dir=target --exclude-dir=node_modules --exclude-dir=.git -- "export_wasm_machine" ✏️s` exits 1 with no matches.
- `rustfmt --edition 2021 --check` passes for all three touched Rust sources.
- `git diff --check` passes for the touched paths.

## Package Tests

- `bun ./…/🔄️fsm/✨️macros/📦️packages/🦀️rust/📜️script.ts test` — passed: 9/9 tests.
- `bun ./…/🔄️fsm/📦️packages/🦀️rust/📜️script.ts test` — could not compile because `semio-framework-os-kernel` has 12 unresolved `zip` references. The failing references are in its independently mounted `🧩️extension` and `🪐️space` modules, while the package's active `Cargo.toml` is concurrently modified. This dependency failure does not name the removed macro or either draw FSM macro source.
