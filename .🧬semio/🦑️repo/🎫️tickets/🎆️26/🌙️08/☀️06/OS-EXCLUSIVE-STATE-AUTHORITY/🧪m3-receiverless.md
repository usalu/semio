# M3 Receiverless DocumentApp

**Ticket:** 26/08/06/OS-EXCLUSIVE-STATE-AUTHORITY
**Date:** 2026-08-06

## Landed
- `DocumentApp: Default + Send + 'static` with associated consts `APP_ID` / `DOCUMENT_SCHEMA`
- All DocumentApp methods are associated functions (no `&self`)
- `handle` takes `DraftView` + `EngineHandles`; `Emit` has `draft_operations`
- `register_document_app::<A>(app)` turbofish-only (factory closure removed)
- `CHANNEL_VERSION = 5` in spr channel
- Guest still holds stores in `VcsDocumentApp` (draft lane applied); host `DocumentSession` owns EngineCache

## Verify
- cargo check -p semio-framework-plugin --lib ✅
- cargo check -p semio-framework-os-kernel --lib (see log)
- cargo check -p semio-framework-plugin-host --lib (see log)

## Remaining
- Move DocumentStore/ConfigStore/DraftStore/command_log fully into host DocumentSession
- Delete guest INSTANCES TLS / ViewState
- Rewrite exchange packs for host-applied Emit
- Wave 2: migrate all plugin DocumentApp impls to associated fns + Draft types + ZST (no Mutex fields)
