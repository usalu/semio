# Wave 1.B — Extension Store (summary)

## Delivered

### 1. TypeScript extension store (`🔌️plugin/📦️packages/🟦️typescript/🏪️store/`)

- **`📜️store.ts`**: `createExtensionStore`, `installFromBytes`, `installFromUrl`, `uninstall`, `listInstalled`.
- **Package unpack** (`unpackExtensionPackage`): semio binary envelope strip (optional) + deflate zip via `fflate`; entries `manifest.semio` / `🛂️manifest.semio` + `component.wasm` + optional `assets/*`; manifest decoded with `decodePackValue` from `@semio-tech/framework-os-core`.
- **Materializers**:
  - `nativeMaterialize` — writes raw `component.wasm` only.
  - `webMaterialize` — jco transpile + `🟨️host-shim.js` / bridge / worker via shared **`🌐plugin-web-materialize.ts`** (extracted from os-dev `📜️script.ts`).
- **Dev Vite plugin**: `semioExtensionStoreVitePlugin` — `POST /extensions/install` (raw bytes or `{ "url" }`), `GET /extensions/watch` SSE (`.extension-watch` marker, debounced like plugin hot-swap).
- **Install root (dev)**: `🧑️‍💻️dev/🔌️extension-modules` (`defaultExtensionInstallRoot`).

### 2. Vite (`⚙️vite.config.ts`)

- `staticDirVitePlugin` route **`/extensions`** → `installedExtensionsDir`.
- Alias `/extensions` → install dir; `fs.allow` includes install dir.
- `semioExtensionStoreVitePlugin({ installRoot, repoRoot })` beside `semioPluginHotSwapVitePlugin`.

### 3. Hub mirror (`🌎️hub/📦️packages/🦀️rust/📦️bin.rs`)

- `GET /extensions` — lists `install.json` per subdirectory under `extensions_root`.
- `GET /extensions/{extension_id}/{*rest}` — serves static assets (js/wasm/json MIME).
- `extensions_root`: `OS_HUB_EXTENSIONS_DIR` or `{OS_HUB_DATA}/extension-modules`.

### 4. Native host

- **`WasmPluginRuntime::load_bytes`** in `🔌️plugin/🖥️host/🦀️component.rs` — shared `load_from_wasm_bytes` with `load()`.
- Fixed pre-existing broken `eprintln!(` in `AppFrame::Emit` handler (blocked `cargo check`).

### 5. Shared web glue

- **`🌐plugin-web-materialize.ts`**: `pluginComponentBridgeSource`, `pluginWorkerSource`, `hostShimSource`, `rewritePreview2ShimImports`, `transpilePluginComponent`, `ensurePreview2ShimVendorAt`.
- Os-dev `📜️script.ts` imports these instead of duplicating ~400 lines.

## Workspace

- `@semio-tech/plugin-extension-store` added to root `package.json` workspaces.

## Verification (this ticket)

```bash
bun -e "…"  # unpack smoke — see agent log [DEBUG] unpack ok demo.ext 8
cargo check -p semio-framework-plugin-host  # OK after Emit fix
```

Hub `cargo check` may fail locally on Xcode license / blake3 C build (environment), not on these edits.

## Follow-ups (not Wave 1.B)

- **Wave 1.A** Rust `pack`/`unpack`/`verify` + BLAKE3 `content_hash` — store currently uses SHA-256 for `packageHash` in `install.json` until shared blake3 helper lands.
- **Wave 1.C** `createExtensionSource` + registry `role: extension`.
- **Wave 1.D** space ledger ops + ShellHost lifecycle.
- Production vite **copy** of `/extensions` for ship builds (Wave 4a).

## Files touched

| Area | Path |
|------|------|
| Store | `…/🏪️store/📜️store.ts`, `package.json` |
| Shared glue | `…/🌐plugin-web-materialize.ts` |
| Dev | `…/🧑️‍💻️dev/…/📜️script.ts`, `⚙️vite.config.ts` |
| Host | `…/🔌️plugin/🖥️host/🦀️component.rs` |
| Hub | `🌎️hub/…/📦️bin.rs` |
| Root | `package.json` workspaces |
