# M3 Host Authority Progress

**Ticket:** `26/08/06/OS-EXCLUSIVE-STATE-AUTHORITY`
**Date:** 2026-08-06
**Slice:** m3-host-authority

## Design (host-authoritative)

```
Host process
  HostState.engines: EngineCache          # plugin-wide, WIT engine-derive/read
  HostState.sessions: HashMap<u32, DocumentSession>
    DocumentSession
      generation / command_log_len
      document: SessionLanePack           # opaque pack+spr+ops
      config: SessionLanePack
      draft: SessionLanePack

Guest WASM (still transitional)
  INSTANCES TLS → AppInstance → VcsDocumentApp
    typed DocumentStore / ConfigStore / DraftStore / command_log / cache
  DocumentApp is receiverless (view, command) → Emit
```

Channel (CHANNEL_VERSION = 5):
- `AppCommand::PureCommand` — host sends command + document/config/draft packs
- `AppFrame::Emit` — guest returns op packs for host apply
- `AppFrame::Draft` — draft-lane pack snapshot

## Moved this pass

| Item | Where |
|---|---|
| Opaque `SessionLanePack` (document/config/draft) | Host `DocumentSession` |
| Per-instance session map | `HostState.sessions` |
| EngineCache (host-owned, plugin-wide) | `HostState.engines` (was on single DocumentSession) |
| Pack mirroring on LoadDocument/LoadConfig/Hello/PureCommand | Host `exchange` pre-adopt |
| Pack mirroring on Document/Config/Draft/Emit frames | Host `exchange` post-adopt |
| Session allocate/remove | `create_app` / `destroy_app` |
| `INSTANCE_VIEW_STATES` TLS deleted | Guest — decode view_state from wire only |
| Channel PureCommand / Emit / Draft variants | spr channel codec |

## Still guest-owned (gaps)

1. **Typed stores** — `VcsDocumentApp` still owns `DocumentStore` / `ConfigStore` / `DraftStore` / `command_log` / projection cache.
2. **INSTANCES TLS** — still required to route object-safe `PluginApp` until PureCommand path materializes views without guest stores.
3. **PureCommand apply** — guest arm returns unsupported; host does not yet `dispatch` Emit ops onto typed/erased stores.
4. **Host typed apply** — needs schema-keyed type-erased `dispatch` (or guest returns new packs); `DocumentCodec` today covers print/parse only.
5. **Command log** — length counter on host; full log rows still guest.
6. **EngineHandles threading** — guest still passes `EngineHandles::empty()` in typed dispatch; WIT derive/read exist but not wired through exchange.
7. **ProgramBridge / renderer** — still use legacy Command + guest-owned LoadDocument path (no PureCommand callers yet).
8. **Wave 2 apps** — ZST migration continues separately.

## Next steps

1. Guest PureCommand: hydrate ephemeral views from packs → `DocumentApp::handle` → `AppFrame::Emit` (no guest store mutation).
2. Host: apply Emit ops (type-erased or re-encode packs) and bump session generation/command_log_len.
3. Delete guest INSTANCES once create/exchange no longer need guest `PluginApp` boxes for state.
4. Thread real `EngineHandles` from host cache into handle.
5. Migrate ProgramBridge callers to PureCommand.

## Verify

See `🧪m3-host-authority-*-check.err` in this ticket folder.

| Gate | Result |
|---|---|
| `cargo check -p semio-framework-plugin --lib` | **GREEN** |
| `cargo check -p semio-framework-plugin-host --lib` | **GREEN** |



## PureCommand guest hydrate (follow-up)

- Guest `AppCommand::PureCommand` now hydrates document/config/draft lanes from host packs, runs `handle_command_frame`, and returns `AppFrame::Emit` with encoded op packs + output/diagnostics.
- `VcsDocumentApp` captures `last_emit_wire` in `dispatch_emit`.
- `PluginApp` gained `take_last_emit_wire` / `hydrate_*_lane`.
- Still transitional: guest also applies Emit locally (stores not deleted); host must apply Emit onto session authority next.
- `cargo check -p semio-framework-plugin --lib` GREEN after wiring.

## Host Emit apply (this pass)

- `DocumentCodec::apply_ops_binary` — type-erased fold: `parse_document_pack` → `DocumentStore::dispatch(Apply)` → `print_document_pack`.
- `store::lane_schema_from_spr` — schema fallback from `.spr` when session schemas unset.
- `SessionLanePack::apply_emit_ops` — host `AppFrame::Emit` path calls codec fold (falls back to `pending_binary_ops` when schema/codec/baseline missing).
- `DocumentSession` carries `document_schema` / `config_schema` / `draft_schema`; `create_app` seeds document schema from manifest `AppIo.document_schema`; `bind_session_schemas` for explicit bind.
- `post_adopt_frame_packs` Emit arm applies document/config/draft ops and bumps `generation` / `command_log_len`.

### Remaining gaps

1. **Guest duplicate apply** — `VcsDocumentApp::dispatch_emit` still mutates guest stores; host is now authoritative for mirrored packs.
2. **INSTANCES TLS** — guest `PluginApp` box still required for PureCommand hydrate path.
3. **Config/draft schema** — manifest `AppIo` often empty; callers should `bind_session_schemas` or rely on spr-derived schema + registered codecs.
4. **Empty lane baseline** — Emit apply requires pre-adopted pack+spr (PureCommand pre-adopt satisfies this).
5. **EngineHandles** — not threaded through PureCommand guest handle yet.
6. **Command log** — host only tracks `command_log_len`; row payloads still guest-owned.
7. **ProgramBridge** — no PureCommand migration yet.

### Verify (2026-08-06)

| Gate | Result |
|---|---|
| `cargo check -p semio-framework-plugin-host --lib` | **GREEN** (`🧪m3-host-authority-host-check2.err`) |
| `cargo check -p semio-framework-plugin --lib` | **GREEN** (`🧪m3-host-authority-plugin-check3.err`) |

## Emit apply (follow-up)

- `DocumentCodec::apply_ops_binary` folds `encode_ops_vec` ops onto pack+spr via `DocumentStore::dispatch(Apply)`.
- `SessionLanePack::apply_emit_ops(schema, ops)` uses the codec when schema is bound; otherwise keeps `pending_binary_ops`.
- `DocumentSession` carries optional `document_schema` / `config_schema` / `draft_schema`.
- `WasmPluginRuntime::bind_session_schemas` exposes binding for callers.
- Host Emit arm calls `apply_emit_ops` (not stash-only).
- `cargo check -p semio-framework-plugin-host --lib` **GREEN** (`🧪m3-emit-apply-check.err`).

### Still open
- Callers must `bind_session_schemas` after `create_app` or Emit cannot fold.
- Guest typed stores / INSTANCES TLS / EngineHandles threading.
- ProgramBridge → PureCommand migration.
