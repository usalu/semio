# Session progress (2026-08-06)

## Landed
- P0 contracts + collision map
- M1 taxonomy (`artifactSpecFilenames`, `component.ts`, registry `validateTaxonomyTree` spec+TS checks)
- P1: `WireEdgeLabel`, fused `EdgeArrow` parse/print via `dsl_notation`, pack wire presence `0b100`
- M2 `.grammar.semio` dialect headers on all family fragments
- M3 `verify_protocol_bytes`, dag pack/spr protocol specs
- M5 `LanguageSpec::grammar_path`, `LanguageSpec::derived`
- P2 families scene/embed/geo; `dsl_lsp` + `s_language_bundle`; Jack LSP shim
- M7 policy allowlists (grammar/protocol/ts) + repurposed breach text
- W4 seed: 520 facet spec+TS stubs (`seed-artifact-specs.mjs`); 29 plugin TS packages
- Writer `OpenDocument` command (extension → `language_for_extension`)

## Blocked / remaining
- P2 writer: main window still jack-forks; `lang_from` not in derive; regex tokenizers remain
- P3 pilots: fem2d/note/dag need handcrafted grammars + LSP + conformance (stubs only)
- W4: per-artifact handcrafted specs (not `TEXT*` placeholders)
- P5: `DocumentDsl`/`OpText` derive path still required by all plugins
- P6/P7: full verify blocked (Xcode license on agent host; workspace multi-root noise)
- Repo MCP `ticket_close` unavailable in session

## Next
1. Run `populate-policy-allowlists.mjs` and paste into `📜️script.ts` allowlists
2. `cargo test` in `dsl_schema` / `pack_value` after Xcode license
3. Wave agents per `wave-ownership-*.txt` replacing stub specs
