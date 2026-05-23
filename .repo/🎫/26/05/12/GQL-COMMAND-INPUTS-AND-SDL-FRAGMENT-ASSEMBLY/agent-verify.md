# Verify (2026-05-12)

- `cargo test -p semio --lib`: 36 passed, 1 ignored.
- `cargo check --target wasm32-unknown-unknown -p semio`: ok.

Changes in `semio/rs/lib.rs`: command operation structs renamed to `*OperationInput`; `Mutation::session` → `SessionCommandInput`; `gql::sdl()` skips empty `SDL_FRAGMENT` pushes so normalized SDL matches `target.schema.graphql`.
