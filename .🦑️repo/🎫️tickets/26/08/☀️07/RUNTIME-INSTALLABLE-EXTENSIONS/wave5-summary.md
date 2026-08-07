# Wave 5 — Verification gate

## Static (confirmed)
- `standalone-wasm`: **0** product hits
- `RequestPluginExchange` / `requestPluginExchange`: **0** product hits
- `@semio-tech/flow-module-bim`: removed from vite allowlist; comments renamed to `flow-extension-bim`

## Catalog
- 26 `EXTENSION_TARGETS` after phantom-path cleanup + registry generate
- Launch.json regenerated

## Runtime / cargo
**Blocked** on this machine: Xcode license not accepted (`cc` exit 69). Evidence in `cargo-check-w3.log`.

After `sudo xcodebuild -license`:
```bash
cargo test -p semio-framework-os-flow --lib
cargo test -p semio-s-plugin-flow-extension-math -p semio-s-plugin-process-wood
bun ./📜️script.ts package  # from any extension rust package
# then OS E2E: install URL → invoke → disable → uninstall → reload ledger
```
