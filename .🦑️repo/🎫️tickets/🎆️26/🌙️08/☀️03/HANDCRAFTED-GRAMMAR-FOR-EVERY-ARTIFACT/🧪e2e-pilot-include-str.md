# Pilot include_str + LanguageSpec registration

Date: 2026-08-06

## Rule
- Text facets (`🗣️dsl`, `🔧️op`, `🔺️diff`): `COMPONENT_GRAMMAR_SEMIO` via `include_str!("📖️component.grammar.semio")`
- Binary facets (`🎒️pack`, `📡️spr`): `COMPONENT_PROTOCOL_SEMIO` via `include_str!("📡️component.protocol.semio")`
- Engine `register()` wires each into `dsl::LanguageSpec` (`grammar`/`grammar_path` or `protocol`/`protocol_path`) with `LanguageRole::{Document,Ops,Diff,Pack,Spr}`

## Pilots
| Artifact | Facets with include_str | LanguageSpec register site |
|---|---|---|
| 🕸️dag | dsl/op/diff/pack/spr | `⚙️engine::register_artifact_languages` |
| 🏗️fem ◻2d | dsl/op/diff/pack/spr | `⚙️engine::register_fem2d_languages` |
| 🏗️fem 🧊️3d | dsl/op/diff/pack/spr | `⚙️engine::register_fem3d_languages` |
| 🗒️note | dsl/op/diff/pack/spr | `⚙️engine::register_note_languages` |
| ✒️writer | dsl/op/diff/pack/spr (+ jack Embedded) | `⚙️engine::register_writer_languages` |

## Conformance tests
Each text facet asserts `parse_grammar(...).dialect == Grammar`.
Each binary facet asserts `Protocol` and `verify_protocol_bytes` against `encode` / `encode_op` bytes.

## Compile status

- `semio-framework-os-kernel-dsl-grammar`: cargo check OK
- `semio-framework-os-kernel-dsl`: cargo check OK (protocol fields + reexport)
- `semio-s-plugin-note --lib`: Finished OK
- `semio-s-plugin-fem --lib`: Finished OK
- `semio-s-plugin-writer --lib` / `semio-s-plugin-dag --lib`: blocked by unrelated `semio-framework-os-kernel` EmbedFrom exhaustiveness errors (pre-existing)
- `cargo test -p dsl-grammar`: blocked by Xcode license linker (host), typecheck OK
