# Phase 9 Live Dependency Census — 209

## Checkpoint

The dependency-freeze boundary contains 209 unique third-party names: 75 Rust and 134 JavaScript. This is 29 below the 238-name baseline at commit `95b8688ee2f62f4056b6403c282bf0c76172c37c`.

## Verification

- `bun ./📜️script.ts verify dependencies` — exit 0; no new dependencies.
- `bun ./📜️script.ts verify dependencies list rust | jq 'length'` — 75.
- `bun ./📜️script.ts verify dependencies list js | jq 'length'` — 134.

The newly absent Rust rows at this checkpoint include `jsonschema`, `notify`, `pollster`, `reqwest`, `spade`, `thiserror`, `ts-rs`, `uuid`, and `wit-parser`. The count is a progress ratchet, not the Phase 9 or Phase 10 exit gate. Owned stdio compression, the plugin WASM interpreter, storage/server boundaries, native rendering, browser bindings, and the JavaScript toolchain remain open.
