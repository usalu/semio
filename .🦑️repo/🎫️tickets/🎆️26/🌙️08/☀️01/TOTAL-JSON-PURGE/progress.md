# Total JSON Purge — progress log

## Completed (2026-08-01 session)

### Wire & types
- `DslValue` + `pack_rt` wire/pack codecs; compose-only JSON shim in store
- `ActionDescriptor` / context menu args → `DslValue` across `ui_wgpu`, plugins, infinite-world, wgpu renderer
- Plugin command JSON fallback removed; WIT host binary `list<u8>` for document/actions
- `backbone_worker_wire`: binary worker protocol (Rust + TS)
- Scene sync: `sync_from_scene_pack` / `syncFromScenePack`; `graph_scene_pack` / `editor_scene_pack`
- OS React: `encodeOperationEnvelopesPack`, `applyEnvelopes`, pack scene fields, collaboration apply path
- Sync fixtures: load `🔣️fixture.dsl` only; removed legacy `🔣️fixture.json` (3 dirs)

### Verification
- `cargo check --workspace` — green
- `framework-os-core` vitest — 204/204

## Incremental / exempt (per plan)
- **Repo MCP / Jack LSP / compose / mit-bestand** — JSON-RPC or compose shim unchanged
- **Nested scene host strings** — flow/map/board still use `*_json` text slots with `pk:` pack prefix where migrated; inner DAG/canvas APIs may still parse JSON-shaped text via `effective_json_field`
- **Tutorial / debug** — some `serde_json` at tutorial record boundaries; `[DEBUG]` logs remain for tutorial pack load
- **Jack graph DSL fixtures** — deferred
- **React vitest** — 6 footer/logo path failures noted in prior agent run (wasm stubs added)
- **Ticket MCP** — `ticket_close` blocked (namespace auth skipped)

## Grep gate (manual)
Runtime JSON still appears in tests, compose, flow eval chains, and legacy menu JSON deserialization (`GraphContextMenuItem`). Not zero grep; hot paths use pack/binary.
