# Total JSON Purge — progress log

## Foundation (done)
- `dsl_schema`: `DslValue` accessors, `to_dsl_value` / `from_dsl_value` via `dsl_value_serde.rs` (no JSON text on wire)
- `store::pack_rt`: `encode_wire_value` / `decode_wire_value` on `DslValue`; `encode_pack_value`; compose-only `encode_json_value` shim
- `DocumentPack for DslValue`; compose-only `DocumentPack for serde_json::Value`
- Rust workspace: `cargo check --workspace` green (2026-08-01)

## TS / backbone (partial)
- Binary backbone HTTP + pack/spr bundle helpers in framework-os-core
- backbone-worker binary wire envelope to wasm worker
- OS React shell + shooting export pickers updated toward `.spk`/`.dsl`/`.ops`

## Remaining (follow-up)
- Vitest root config path issues; 10 failing tests in prior run (fixture paths, AppChannel)
- Sync `fixture.json` manifests → `.ops` loaders
- Scene `*Json` fields in WGPU/React/framework UI
- Grep gate: serde_json/JSON.parse still present in many plugin UI paths (action args)
- WIT `*-json` host params
- framework-core `serde_json::Value` fields → `DslValue`
- protocol_causal serde derives removal
