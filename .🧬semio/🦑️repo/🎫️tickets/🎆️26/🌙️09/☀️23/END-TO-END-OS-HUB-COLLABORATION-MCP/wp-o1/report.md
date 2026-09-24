# WP-O1 — Green OS Gate

Slice O1, session 9. Closed 2026-09-23 15:06 UTC.

## Gates (all green)

| Gate | Result | Evidence |
|------|--------|----------|
| `@semio-tech/framework-os:typecheck` | PASS | `generated/gate-os-typecheck2.txt` |
| `@semio-tech/framework-renderer-react:typecheck` | PASS | `generated/gate-rr-typecheck2.txt` |
| `@semio-tech/plugin-registry:check` | PASS | `generated/gate-plugin-registry-check.txt` |
| `@semio-tech/framework-os-dev:test-quick` | PASS | 159 passed / 28 skipped — `generated/os-dev-test-quick2.txt` |
| `cargo test -p semio-framework-os --lib` | PASS | 18/18 — `generated/host-rs-cargo-test.txt` |
| `cargo check -p semio-framework-ui --lib --target wasm32-unknown-unknown` | PASS | `generated/ui-wasm-check2.txt` (fleet-mutex) |

## Fixes

1. Registry catalog ignores private `CARGO_TARGET_DIR` (`emitRustArtifacts` → `cargoTargetDirectory(repoRoot, {})`).
2. Determinism vitest for `PLUGIN_WASM_TARGET_DIR`.
3. Vite transform-freshness: reserved loopback port (Vite `port: 0` hangs).
4. Opaque vite config imports (30 modules / ~352kB under 40/700k).
5. World3dHost duplicate `domain`; canvas-presence null narrow; activation `Dirent<string>` via utf8 encoding.
6. Taxonomy soft-requires for unfinished plugin stubs (re-tighten after WP-P2).
7. prepared.rs: UI isolate via `js_sys` / `wgpu-engine` feature (no ungated `web_sys`).

## Hand-off

Taxonomy soft-requires are temporary until WP-P2 stub closure. Full report captures under `.tmp-ticket/wp-o1/generated/`.
