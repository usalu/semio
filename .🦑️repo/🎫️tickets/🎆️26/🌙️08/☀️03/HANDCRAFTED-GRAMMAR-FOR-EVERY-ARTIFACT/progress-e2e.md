# End-to-end follow-through (2026-08-06)

## User mandate
- Grammars = text (`🗣️dsl`, `🔧️op`, `🔺️diff`); protocols = binary (`🎒️pack`, `📡️spr`).
- LanguageSpec protocol fields + Pack/Spr roles; pilot include_str + register; verify_protocol_bytes; evidence md.

## Done
- Dialect sweep: 156 grammars + 104 protocols (see `🧪e2e-dialect-sweep.json`).
- LanguageSpec: `protocol`/`protocol_path`, `LanguageRole::{Pack,Spr}`, `derived`, `passthrough_hooks`.
- Pilots dag/fem2d/fem3d/note/writer: include_str on all 5 facets; LanguageSpec registration; dialect + verify_protocol_bytes tests.
- Evidence: `🧪e2e-pilot-include-str.md`, `🧪e2e-language-protocol-wire.md`, `🧪e2e-pilot-wire-checklist.txt`.

## Blocked
- Full `cargo test` on host: workspace members still point at deleted `⚡️implementations` crates after OS kernel consolidation.

## LanguageSpec protocol wire confirmed (post-consolidation)
- Sole `LanguageSpec` source: `🗣️dsl/🦀️component.rs` with `protocol`/`protocol_path` + Pack/Spr + `passthrough_hooks`.
- Agent-noted stale dsl `⚡️implementations` paths are gone (consolidated); current `cargo metadata` blocker is duplicate `semio-framework-plugin` (packages vs implementations) — separate from this wire.

## LanguageSpec protocol wire (Grok)

- Fields + helpers + LanguageSession diagnostics/verify_protocol_bytes landed.
- Pilots dag/fem2d/note/writer register grammar+protocol.
- Evidence: `🧪e2e-language-protocol-wire.md`.
