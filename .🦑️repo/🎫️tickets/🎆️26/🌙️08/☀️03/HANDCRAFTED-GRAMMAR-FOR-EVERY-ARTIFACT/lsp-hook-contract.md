# LSP hook contract

## Registry

`LanguageSpec` in `dsl` facade: `id`, `extension`, `role` (`Document` | `Config` | `Ops` | `Embedded`), `grammar` (`include_str!`), `grammar_path`, `LanguageHooks`.

## Hooks (`LanguageHooks`)

`canonicalize`, `classify`, `diagnostics`, `complete`, `hover`, `format` (default canonicalize), `symbols`, `occurrences`, `rename`, `definitions`.

`LanguageSpec::derived::<P>()` fills hooks from `dsl_schema::LanguageService` until handcrafted override.

## Hosts

- `LanguageSession` — in-process; writer `render_main_scene` calls synchronously.
- `dsl_lsp` — JSON-RPC 3.17; `semanticTokens/full` returns `{ data: number[] }` (UTF-16 deltas at boundary).
- `s_language_bundle` — `✏️s/🔨️modules/🗣️lang/` registers all plugin languages.

## Writer boundary

Keep `TextEditorScene` JSON (`tokens_json`, `diagnostics_json`, …); map LSP results 1:1. No `writer/semanticTokens` vendor shape.

## Vendor

`semio/documentContext`, `semio/editorExtras`, `semio/grammar` optional.
