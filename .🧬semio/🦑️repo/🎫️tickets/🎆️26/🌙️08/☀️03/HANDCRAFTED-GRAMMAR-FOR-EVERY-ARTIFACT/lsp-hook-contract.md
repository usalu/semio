# LSP hook contract

## Registry

`LanguageSpec` in `dsl` facade:

- `id`, `extension`
- `role`: `Document` | `Config` | `Ops` | `Embedded` | `Diff` | `Pack` | `Spr`
- **Text:** `grammar` / `grammar_path` (`.grammar.semio`, `dialect grammar`) for `🗣️dsl` / `🔧️op` / `🔺️diff`
- **Binary:** `protocol` / `protocol_path` (`.protocol.semio`, `dialect protocol`) for `🎒️pack` / `📡️spr`
- `hooks` (`IdiomHooks`): `canonicalize`, `classify`, `complete`

Helpers: `LanguageSpec::parsed_grammar`, `parsed_protocol`, `verify_protocol`, `passthrough_hooks`, `is_text_role` / `is_binary_role`.

`LanguageSpec::derived` copies grammar+protocol fields from the parent.

## Hosts

- `LanguageSession` — in-process; writer calls synchronously.
  - Text: `semantic_tokens_lsp`, `completions_at`, `canonicalize`, `diagnostics` (hooks + grammar dialect check)
  - Binary: `verify_protocol_bytes(bytes)` when `protocol` is set
- `dsl_lsp` — JSON-RPC 3.17; `semanticTokens/full` returns `{ data: number[] }`
- `s_language_bundle` — `✏️s/🔨️modules/🗣️lang/`

## Writer boundary

Keep `TextEditorScene` JSON; map LSP results 1:1.

## Vendor

`semio/documentContext`, `semio/editorExtras`, `semio/grammar` optional.
