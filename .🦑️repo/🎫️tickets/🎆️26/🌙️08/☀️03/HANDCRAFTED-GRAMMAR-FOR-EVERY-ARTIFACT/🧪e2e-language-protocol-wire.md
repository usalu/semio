# LanguageSpec protocol wire

## Facade (`🗣️dsl`)
- `LanguageSpec` fields: `grammar`, `grammar_path`, `protocol`, `protocol_path`
- `LanguageRole::Pack` / `LanguageRole::Spr` (alongside Document/Ops/Diff/Embedded/Config)
- `LanguageSpec::derived` copies protocol fields from parent
- `passthrough_hooks(lang)` for facets without a custom `DslIdiom`

## `verify_protocol_bytes`
- Requires `SemioDialect::Protocol`
- `start frame` / id containing `pack` → SPK magic + 32-byte header
- `start record` / id containing `spr` → non-empty op/record bytes (**does not** require SPK/SPR file magic)

## Unit tests (`dsl_grammar`)
- `parse_grammar_sets_dialect_grammar_vs_protocol`
- `verify_protocol_bytes_branches_pack_spk_vs_spr_record`
- Plugin facet sweep + dag pack/spr handcrafted parse checks

## Cargo / linker
Host may be blocked by Xcode license; record failures in this ticket rather than opening Xcode UI.
