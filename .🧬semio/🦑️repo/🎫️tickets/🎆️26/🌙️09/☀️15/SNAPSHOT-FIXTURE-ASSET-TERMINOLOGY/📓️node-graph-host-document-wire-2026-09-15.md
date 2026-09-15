# Node Graph Host Document Wire — 2026-09-15

## Change

`NodeGraphScene` no longer exposes flow host JSON as `fixtureJson`.

| Layer | Before | After |
| --- | --- | --- |
| Rust field | `fixture_json: Option<String>` | `host_document_json: Option<String>` |
| Scene wire | `fixtureJson` | `hostDocumentJson` |
| Retained catalog | `fixtureJson` | `hostDocumentJson` |

`Board2dScene.fixtureJson` is unchanged (board document, not flow host document).

## Updated (evidence)

- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs` encode/decode
- Flow/procedural/space `NodeGraphScene { host_document_json: … }` builders
- Wgpu flow sync reads `graph.host_document_json`
- `🕸️NodeGraph/🟦️.tsx` scene prop + flow engine detection
- Procedural/sequence node-graph edit ops use JSON key `hostDocumentJson`
- Flow `🔬️source-contract` bans `pub fixture_json: Option<String>` on scene types in scanned editor sources

## Remaining

- Engine contract TS golden tests still mention `fixtureJson` for flow scenes
- WASM session methods `fixtureJson()` / `setFixture` operation name
- Trinity rewriting world still emits `fixtureJson` on its own envelope (jack graph, not `NodeGraphScene`)
- Full `cargo test` / renderer vitest sweep not green
