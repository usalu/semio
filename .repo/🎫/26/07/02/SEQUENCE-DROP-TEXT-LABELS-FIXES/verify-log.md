# Verify log

## Rust

- `cargo test` in `sequence/core`: 13 passed (includes `repeated_drops_after_replace_fixture_use_distinct_ids`, `replace_fixture_preserves_next_serial_and_selection`, `text_steps_use_data_ports_without_visible_execution_pins`)
- `cargo test` in `imperative/module/text`: 4 passed
- `cargo test` in `imperative/engine`: 6 passed (updated `composed_registry_runs_text_operators`)

## WASM

- `bun nx run @semio-tech/sequence-core:wasm`: success

## Vitest

- `@semio-tech/sequence-core:test`: 2 passed
- `@semio-tech/sequence-react:test`: 4 passed
- `@semio-tech/sequence-play:test`: 8 passed
- `@semio-tech/imperative-core:test`: 4 passed

## Root causes fixed

1. `loadFixtureJson` no longer replaces the entire `SequenceHost` (uses `replace_fixture` + monotonic `next_serial`).
2. React fixture sync compares canonicalized JSON to avoid spurious reloads on float formatting.
3. `text.*` operators use `into` scope-write like math/logic; `is_function_kind` includes `text.` for data-port rendering.
