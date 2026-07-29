---
technology: protocol
emoji: 🧩
---

# Protocol

A Blockly-like visual editor for generating code/data: a strict, ordered list of steps containing typed blocks, module-extensible via contributed block kinds. List-based (not canvas/map-based).

## Entities

- **ProtocolSpec** — schema id, title, ordered `steps`
- **ProtocolStep** — id, title, description, ordered `blocks`
- **ProtocolBlock** — id, label, kind, and kind-specific fields (default, options, schema, ...)
- **ProtocolOp**/**ProtocolDiff** — `vcs::Operation`/`OperationDiff` implementations for add/remove/move step and block, and title updates

## Bundles

| Bundle                          | Role                                                              |
| ------------------------------- | ----------------------------------------------------------------- |
| `protocol/rs`                   | Domain crate (`protocol`): spec/ops/diff, VCS wiring, wasm bridge |
| `protocol/program/rs`            | Standalone "Protocol" app/plugin (`semio:protocol` component)     |
| `protocol/module/procedural/rs` | Contributes a `ProtocolBlockKind` (e.g. `buildingComponent`)      |

## Mechanisms

- `framework/program/rs::protocol_mode` — shared strict-list builder engine (add/remove/move-step, add/remove/move-block op-builders, `build_protocol_list_scene`, `render_protocol_builder`), reused by `protocol/program/rs` and `forms/program/rs` (forms' "Blueprint" mode)
- `Contribution::ProtocolBlockKind` — module extensibility point for host apps embedding the builder
- `framework/renderer/react/components/protocol-list-host.tsx` — dedicated React host (drag-and-drop sortable step/block list + block-kind palette) for `SurfaceKind::ProtocolList`
