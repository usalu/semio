# Wave 1b (M3) — intermediate flip

**Ticket:** `26/08/06/OS-EXCLUSIVE-STATE-AUTHORITY`  
**Date:** 2026-08-06  
**Gate:** `cargo check -p semio-framework-os-kernel --lib` ✅ · `cargo check -p semio-framework-plugin --lib` ✅ · `cargo check -p semio-framework-plugin-host --lib` ✅

## Landed

### A. Emit + Draft types
- `Emit<Operation, ConfigOperation = NoConfigOperation, DraftOperation = NoDraftOperation>` gains `draft_operations: Vec<DraftOperation>` and `Emit::draft(...)`.
- `DocumentApp` gains associated types `Draft` / `DraftOperation` (in-crate apps use `NoDraft` / `NoDraftOperation` aliases of `NoConfig` / `NoConfigOperation`).
- `DraftView<'a, D>` added alongside `DocumentView` / `ConfigView`.
- `VcsDocumentApp` owns a guest `draft_store: DraftStore<…>` and applies `draft_operations` in `dispatch_emit` (no command-log rows; ephemeral).

### B. EngineHandles on handle
- `DocumentApp::handle` now takes `draft: &DraftView<'_, Self::Draft>` and `engines: &EngineHandles`.
- `VcsDocumentApp::dispatch_typed_command_inner` materializes the draft projection and passes `EngineHandles::empty()` (host engines not yet threaded through exchange).

### C. WIT engine-derive / engine-read
- Already present on `host` interface in `world.wit` as `engine-derive` / `engine-read` funcs (not separate WIT interfaces).
- Host implements them on `HostState` via `DocumentSession.engines` (`EngineCache`), gated by `ArtifactKind::Engine` + Invoke/Read.
- Bindgen path already points at `../../../📦️packages/🦀️rust/📜️wit`.

### D. DocumentSession
- Exists in plugin host: `{ generation, command_log_len, engines }`.
- Docstring updated to name the upcoming `store` / `config_store` / `draft_store` / `command_log` move.

### E. register_document_app_zst + consts
- `DocumentApp::APP_ID` / `DOCUMENT_SCHEMA` associated consts; `app_id` / `document_schema` default to them.
- `PluginBundle::register_document_app_zst::<A>(app)` added; `semio_plugin!` migrated to it.
- In-crate `DummyApp` + `TestApp` compile on the new contract (ZST path exercised via macro + testkit).

### CHANNEL_VERSION
- **Not bumped** (still 4) — `AppCommand` / `AppFrame` shapes unchanged this pass.

## Remains (next Wave 1b / CHANNEL 5)

1. **Full receiverless** — drop `&self` from all `DocumentApp` methods; make apps true ZSTs with only associated fns.
2. **Drop `Fn() -> A` factory** — make `register_document_app` turbofish-only once Wave 2 migrates all `✏️s` apps; remove old factory path.
3. **Host-authoritative stores** — move `DocumentStore` / `ConfigStore` / `DraftStore` / `command_log` / cache out of `VcsDocumentApp` into `DocumentSession`; delete guest `INSTANCES` TLS + `ViewState` map.
4. **CHANNEL_VERSION → 5** — rewrite `exchange` / `AppCommand`/`AppFrame` so host sends packs and applies `Emit` itself; thread real `EngineHandles` from host cache.
5. **Separate WIT interfaces** (optional plan fidelity) — split `engine-derive` / `engine-read` out of `host` into their own imports.
6. **Wave 2** — migrate ~32 plugin apps to consts + `Draft`/`DraftOperation` + new `handle` signature (out of this wave’s edit globs).

## Files touched
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` (docstring only this pass; session/WIT stubs pre-existed)
- Ticket logs: `🧪check-*-wave1b.*`, `🔧️patch-wave1b*.mjs`

## Integration requests
- None new this pass (no Cargo.toml / script.ts / launch.json changes).
