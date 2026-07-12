# WS-B — framework/sync actor layer (notes)

## Delivered
- `framework/sync/rs/Cargo.toml` — added native deps (`tokio` rt/net/time/sync/macros, `tokio-tungstenite 0.26`, `notify 8`), shared `tokio` sync+macros + `futures-util`, wasm deps (`wasm-bindgen`, `wasm-bindgen-futures`, `js-sys`, `web-sys` WebSocket/MessageEvent/…), dev-deps `tempfile` + `tokio` full/test-util.
- `framework/sync/rs/lib.rs` — grew into the actor layer (regions: 🔖Protocol, 🔖Endpoints, 🔖SyncSession, 🔖Host, 🔖NativeActor, 🔖WasmActor, 🔖Fixtures, 🧪Tests).
- `framework/sync/fixtures/` — README + 3 JSON fixtures replayed by cargo test (and later WS-E vitest).

## Verification (all green)
Because the root workspace `Cargo.toml` is being actively rewritten by a concurrent agent
(mathematical/… → infinite/board/… reorg; the members list is mid-edit and TRANSIENTLY dropped
`framework/sync/rs`, `trinity_rewrite` pointed at a missing path, etc.), the crate could not be
built through the real workspace. Verified instead in an isolated workspace (`ws-b-iso/`) containing
only the 4-crate path-dep closure (sync, vcs, framework-core, framework-hash):

- `cargo test -p semio-framework-sync` → 9 passed / 0 failed.
- `cargo check -p semio-framework-sync --target wasm32-unknown-unknown` → clean (no sync-crate warnings).
- `cargo build -p semio-framework-sync` → clean.

## Coordination note (NOT my file — left untouched)
Root `Cargo.toml` currently omits `framework/sync/rs` from `[workspace].members` due to the in-flight
concurrent rewrite. Once that settles, `framework/sync/rs` must be present in the members list again
for CI. I did not edit the shared root manifest to avoid clobbering the concurrent agent's live edit.

## Scope decision
Used a self-contained in-process WS hub (real `HubClientFrame`/`HubServerFrame` protocol via
`tokio-tungstenite` server side) in tests instead of adding a `[lib]` target to WS-C's `os-hub` bin
crate — lower risk, respects crate ownership, still exercises the real wire protocol end-to-end
(convergence, reconnect + `since` backlog catch-up).
