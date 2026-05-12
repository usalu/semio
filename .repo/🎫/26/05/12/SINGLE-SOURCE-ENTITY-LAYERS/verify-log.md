# Verification (2026-05-12, follow-up)

## Commands (exit 0 unless noted)

- `cargo check -p semio --target-dir target-ssel` — **ok** (after fixing `BackboneStatus` Copy, `BackboneAttach` `&connection_uri`, wasm `bootstrap_runtime_from_open_uri` `Ok(...)`).
- `cargo check -p semio --target wasm32-unknown-unknown --target-dir target-ssel-wasm` — **ok** (same fixes).
- `cargo test -p semio --target-dir target-ssel schema_matches_target_graphql_file -- --nocapture` — **ok** (1 test).
- `bunx tsc --noEmit` in `semio/react` — **exit 2**: many pre-existing errors (duplicate `usePiece` / `Kit` type shadow vs `@semio/js` `Kit`, missing `@semio/js` exports for embedded-test-only symbols, missing `PositionInput`, etc.). Not fully cleared in this pass.

## Notes

- Isolated `CARGO_TARGET_DIR` avoids the concurrent default `target/` lock called out in the prior log.
