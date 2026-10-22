---
name: Total JSON Purge
overview: "Purge every runtime use of JSON from the semio framework, OS product, and plugins: binary protocols for all commands, binary pack formats for all documents, and the handcrafted DSL/op text syntaxes as the only human/LLM-facing representation."
todos:
  - id: ticket-open
    content: Read repo://goals and open ticket via repo MCP
    status: completed
  - id: native-value
    content: Replace serde_json::Value with native WireValue in store pack_rt, dsl_schema shapes, framework kernel; drop serde derives from protocol envelopes
    status: completed
  - id: plugin-binary
    content: Delete JSON fallback command envelope; binary WIT host params; typed manifest/context-menu decode without serde_json
    status: completed
  - id: js-bridges
    content: Convert WGPU/React scene *Json fields, handleAction, backbone-worker, and UI action args to binary pack bytes
    status: in_progress
  - id: backbone-persistence
    content: Binary backbone HTTP envelope, pack+spr document persistence replacing document/v1 JSON, binary path-map diffs
    status: in_progress
  - id: exports
    content: Switch all plugin import/export to .spk + DSL text; remove application/json mimes and .json pickers
    status: completed
  - id: debug-llm
    content: Route all debug/LLM output through print_op/DocumentDsl; DSL tutorial snapshots; pack-encoded presence/session state
    status: pending
  - id: fixtures
    content: Convert sync fixture.json, Jack graph JSON fixtures, storybook fixtures to DSL/pack; rewrite JSON-string test assertions
    status: in_progress
  - id: verify-close
    content: Run cargo test + vitest, runtime [DEBUG] confirmation, final grep gate, close ticket
    status: in_progress
isProject: false
---

# Total JSON Purge: Binary Protocols + Handcrafted DSL Everywhere

## Context

The binary/DSL foundation already exists and stays authoritative:

- Commands: `protocol_wire` frames (lane u8 + tag u8 + LEB128 fields), `protocol_channel` `AppCommand`/`AppFrame`, `OpBinary` (`format u8 | ordinal varint | pack record body`)
- Documents: `.spk` pack snapshots (`pack_format`) + `.spr` append-only op history (`protocol_format`)
- Debug/LLM text: `DocumentDsl` records, `OpText` one-line ops, `.ops` logs (`protocol_history`)

What remains is JSON leaking through as the dynamic value model, JS/WASM string bridges, dev persistence envelopes, plugin fallbacks, user exports, and fixtures. This plan removes all of it.

Scope decisions (confirmed): repo MCP/CLI and Jack LSP keep JSON-RPC (external spec-mandated interop). `compose` and `mit-bestand` are separate technologies, untouched. Build-time config files (`package.json`, `Cargo.toml`, `tsconfig`, …) are unaffected. User-facing formats become `.spk` binary + DSL text twin; `.json` import/export is dropped entirely.

## Workstream 1: Native dynamic value model (root cause)

`serde_json::Value` is the universal escape hatch. Replace it with a native domain value type so nothing depends on serde_json at runtime.

- In [store lib.rs](🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚡️implementations/🦀️rust/📦️lib.rs) `pack_rt` region: make `encode_wire_value`/`decode_wire_value` operate on a native `WireValue` (aligned with `dsl_schema::FieldValue`), delete `encode_json_value`/`decode_json_value` and `impl DocumentPack for serde_json::Value`. Schema-less apps get real `RecordSpec` grammars instead.
- In [dsl_schema lib.rs](🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/⚡️implementations/🦀️rust/📦️lib.rs): replace `Shape::Value`/`FieldValue::Value` JSON-literal shapes with the native value literal so DSL text never embeds JSON.
- In [framework kernel lib.rs](🧰️framework/⚡️implementations/🦀️rust/📦️lib.rs): retype `InvocationResult`, `ViewState`, `KernelOperation`, action args, and tutorial events from `serde_json::Value` to `WireValue`/typed structs with binary codec + DSL printing.
- In [protocol_causal](🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/🔗️causal/⚡️implementations/🦀️rust/📦️lib.rs) and [protocol_command](🧰️framework/🛍️products/💻️os/🔨️modules/📡️protocol/🎮️command/⚡️implementations/🦀️rust/📦️lib.rs): drop `serde::Serialize`/`Deserialize` derives from `OperationEnvelope`, `Edit`, `OperationMeta` — binary codec plus `OpText` are the only representations.
- Remove `serde_json` from all in-scope `Cargo.toml` dependencies as each crate is cleaned.

## Workstream 2: Plugin surface fully binary

- In [plugin lib.rs](🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🦀️rust/📦️lib.rs): delete the fallback `dispatch_command_frame` JSON envelope (`{kind,name,args}`); all commands go through typed binary `AppCommand`. Manifest/effects/events encode `WireValue` directly (no `serde_json::to_value` hop).
- In [world.wit](🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️implementations/🦀️rust/📜️wit/📜️world.wit) and [plugin host lib.rs](🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️implementations/🦀️rust/📦️lib.rs): replace `payload-json`/`*-json` string params with `list<u8>` binary wire values; decode `PluginManifest` from wire bytes into typed structs without a `serde_json::Value` intermediate; same for `context_menu` request/response.

## Workstream 3: JS/WASM bridges binary

- In [WGPU renderer lib.rs](🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧊️wgpu/⚡️implementations/🦀️rust/📦️lib.rs) (~250 serde_json uses) and [OS React shell index.tsx](🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/⚛️react/⚡️implementations/🟦️typescript/📦️index.tsx) (~100 JSON.parse/stringify): replace all scene `*Json` string fields (camera, selection, fixture, tables, paint2d, board, dag, world3d, …) with binary pack bytes (`Uint8Array`) decoded via `decodePackValue`; `handleAction`/`handle_action_js` exchange binary command envelopes instead of JSON strings; drag/drop payloads become pack bytes.
- In [backbone-worker.ts](🧰️framework/🛍️products/💻️os/⚡️implementations/🟦️typescript/🟦️backbone-worker.ts) and [os index.ts](🧰️framework/🛍️products/💻️os/⚡️implementations/🟦️typescript/📦️index.ts): worker messages to `store_worker` become binary (drop `handleRequestJson`); `encodePackValue`/`decodePackValue` are the only TS codec, updated for the native value model.
- In [framework ui react index.tsx](🧰️framework/🔨️modules/🖱️ui/⚛️react/⚡️implementations/🟦️typescript/📦️index.tsx) and [framework ts index.ts](🧰️framework/⚡️implementations/🟦️typescript/📦️index.ts): UI action/staging args carried as pack values, not JSON.

## Workstream 4: Backbone and persistence envelopes

- Dev `/semio-backbone` HTTP in [dev script.ts](🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/⚡️implementations/🟦️typescript/📜️script.ts) and `readBackboneEnvelope`/`writeBackboneEnvelope` in [os index.ts](🧰️framework/🛍️products/💻️os/⚡️implementations/🟦️typescript/📦️index.ts): binary `encode_backbone_message` bytes with `content-type: application/octet-stream`, replacing the `{kind: snapshot|operations}` JSON envelope.
- Persisted `document/v1` JSON envelopes (`wrapDocumentEnvelope`/`documentFromEnvelopeJson`): replaced by the existing binary bundle `encode_document_pack_bytes` (pack + spr), with `.dsl`/`.ops` text mirrors for humans. Per-app `envelope_json`/`projection_json` bridges (CAD `CadDocumentVcs`, `FlowDocumentVcs`, puzzle3d, procedural, norm, shooting, …) move to pack bytes + DSL text.
- Path-map diffs in [db document lib.rs](🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️document/⚡️implementations/🦀️rust/📦️lib.rs) (`encode_pathmap` = `serde_json::to_vec`): binary pack record encoding.

## Workstream 5: User-facing import/export → .spk + DSL

Across all plugins in `✏️s/🔌️plugins/**` (CAD spatial, flow, `.note.json`, `.theme.json`, shooting/remodel QC, architect, animate present deck, tutorial downloads):

- `HostEffect::DownloadMediaExport` emits `.spk` (binary, `application/octet-stream`) and `.dsl`/`.ops` (text) — no `application/json` mime anywhere.
- File pickers accept `.spk,.dsl,.ops` (drop `application/json,.json`), including the OS shell import in the React renderer.
- Animate present embedded deck: pack bytes (base64 in `<script>` if needed), not `application/json` script tags.

## Workstream 6: Debug and LLM surfaces → handcrafted syntax only

- Every debug log, tutorial export, and LLM-facing string that currently uses `JSON.stringify(...)`/`serde_json::to_string` switches to `print_op`/`DocumentDsl` printing (tutorial recording `document_json`/`last_envelope_json` become DSL text snapshots).
- `sessionStorage` identity and collaboration presence payloads in the React shell become pack bytes (base64) or DSL text.

## Workstream 7: Fixtures and tests

- Sync actor `fixture.json` files under [store sync fixtures](🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/⚡️implementations/🦀️rust/🧫️fixtures) → `.ops` DSL fixtures; golden `.bin` wire fixtures stay.
- Jack graph JSON fixtures → the Jack graph DSL under [math graph dsl](🧰️framework/🔨️modules/🧮️math/🕸️graph/🗣️dsl/⚡️implementations/🦀️rust/📦️lib.rs).
- Storybook host fixtures → pack/DSL. All existing test files are extended in place (no new test files); assertions on JSON strings (e.g. flow/cad UI tests) rewritten against typed values or DSL text.

## Order of execution

Workstream 1 first (everything downstream depends on the native value type), then 2 and 3 in parallel (plugin surface and JS bridges), then 4–5 (persistence and exports), then 6–7 (debug surfaces and fixtures). The in-flight uncommitted context-menu work in flow/cad/sequence UI files is preserved — edits merge around it, never revert it.

## Verification

- `cargo test` across the workspace and `vitest` via nx after each workstream.
- Runtime confirmation with `[DEBUG]`-prefixed logs on the renderer/plugin/backbone paths (dev server + browser), removed afterwards.
- Final gate: repo-wide search proving `serde_json`, `json!`, `JSON.parse`, `JSON.stringify`, and `application/json` are absent from all in-scope paths (allowed remnants: repo MCP/CLI, Jack LSP, compose/mit-bestand, build configs).

## Ticketing

Open a new ticket via repo MCP (server was not ready during planning; read `repo://goals` first and associate — likely the same goal family as the recent protocol/pack tickets, e.g. `🎯r2602/🎯runningsketchpad`). All scratch logs go in the ticket folder; close with summary and file list when done.