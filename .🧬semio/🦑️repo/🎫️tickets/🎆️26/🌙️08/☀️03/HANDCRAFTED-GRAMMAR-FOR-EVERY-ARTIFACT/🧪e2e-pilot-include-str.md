# Pilot include_str + LanguageSpec registration

Date: 2026-08-06 (Grok resume)

## Rule
- Text facets (`🗣️dsl`, `🔧️op`, `🔺️diff`): `COMPONENT_GRAMMAR_SEMIO` via `include_str!("📖️component.grammar.semio")`
- Binary facets (`🎒️pack`, `📡️spr`): `COMPONENT_PROTOCOL_SEMIO` via `include_str!("📡️component.protocol.semio")`
- Engine `register()` / `register_pilot_languages()` (writer: `register_writer_languages`) wires each into `dsl::LanguageSpec` with `grammar`/`grammar_path` or `protocol`/`protocol_path` and `LanguageRole::{Document,Ops,Diff,Pack,Spr}`
- Hooks: `dsl::passthrough_hooks(id)` (writer also registers Embedded `jack`)

## Pilots
| Artifact | Facets include_str | LanguageSpec ids | Register site |
|---|---|---|---|
| 🕸️dag | dsl/op/diff/pack/spr | `dag.document`, `dag.op`, `dag.diff`, `dag.pack`, `dag.spr` | `⚙️engine::register_pilot_languages` |
| 🏗️fem ◻2d | dsl/op/diff/pack/spr | `fem.fem2d`, `fem.fem2d.op`, `fem.fem2d.diff`, `2d.pack`, `2d.spr` | `⚙️engine::register_pilot_languages` |
| 🏗️fem 🧊️3d | dsl/op/diff/pack/spr | `fem.fem3d`, `fem.fem3d.op`, `fem.fem3d.diff`, `3d.pack`, `3d.spr` | `⚙️engine::register_pilot_languages` |
| 🗒️note | dsl/op/diff/pack/spr | `note.document`, `note.op`, `note.diff`, `note.pack`, `note.spr` | `⚙️engine::register_pilot_languages` |
| ✒️writer | dsl/op/diff/pack/spr (+ jack) | `writer.document`, `writer.op`, `writer.diff`, `writer.pack`, `writer.spr`, `jack` | `⚙️engine::register_writer_languages` |

## Conformance
- Text facets: `parse_grammar(...).dialect == SemioDialect::Grammar`
- Binary facets: `SemioDialect::Protocol` + `verify_protocol_bytes` on `encode` / `encode_op` bytes
