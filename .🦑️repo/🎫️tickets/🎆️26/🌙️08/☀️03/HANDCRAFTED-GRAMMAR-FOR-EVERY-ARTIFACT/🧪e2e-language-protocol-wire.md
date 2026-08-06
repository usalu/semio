# E2E: LanguageSpec protocol wiring

**Ticket:** `2026/08/03/HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT`  
**Agent:** Cursor Grok 4.5  
**Date:** 2026-08-06

## Product rule

| Surface | Facets | Spec file | LanguageSpec fields | LanguageRole |
|---|---|---|---|---|
| Text | 🗣️dsl, 🔧️op, 🔺️diff | .grammar.semio (dialect grammar) | grammar / grammar_path | Document, Ops, Diff, … |
| Binary | 🎒️pack, 📡️spr | .protocol.semio (dialect protocol) | protocol / protocol_path | Pack, Spr |

Never put grammar files on pack/spr. Never put protocol files on dsl/op/diff.

## API changes (dsl facade)

### Fields

- `protocol: Option<&'static str>` and `protocol_path: Option<&'static str>`
- Kept `grammar` / `grammar_path` for text roles
- `LanguageRole::{Pack, Spr}`

### Helpers

- `is_text_role()` / `is_binary_role()`
- `parsed_grammar()` — `parse_grammar` + assert `SemioDialect::Grammar`
- `parsed_protocol()` — `parse_grammar` + assert `SemioDialect::Protocol`
- `verify_protocol(bytes)` — `dsl_grammar::verify_protocol_bytes`
- `passthrough_hooks(lang)` for facets without a custom `DslIdiom`
- `LanguageSpec::derived` copies grammar **and** protocol fields
- `language_for_semio_content` resolves dsl / op / pack / spr envelopes

### Re-exports

`pub use dsl_grammar::{parse_grammar, print_grammar, verify_protocol_bytes, GrammarFile, SemioDialect, Recognizer}`

## LanguageSession / dsl_lsp

- Text: `diagnostics()` uses hooks + `parsed_grammar()` when text role
- Binary: `verify_protocol_bytes(bytes)` when protocol text is present
- Also: `grammar_file()` / `protocol_file()`

## Pilot registration

| Artifact | Document | Ops | Pack | Spr |
|---|---|---|---|---|
| dag | dag.document + pack protocol | dag.op + spr protocol | dag.pack | dag.spr |
| fem2d | fem.fem2d + pack protocol | fem.fem2d.op + spr protocol | 2d.pack | 2d.spr |
| note | note.document + pack protocol | note.op + spr protocol | note.pack | note.spr |
| writer | writer.document + pack protocol | writer.op + spr protocol | writer.pack | writer.spr |

Facet `include_str!` constants: `COMPONENT_GRAMMAR_SEMIO` / `COMPONENT_PROTOCOL_SEMIO`.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/⚡️implementations/🦀️rust/📦️lib.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧠️lsp/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧠️lsp/⚡️implementations/🦀️rust/📦️lib.rs`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/⚙️engine/🦀️component.rs`
- Ticket: `lsp-hook-contract.md`, this file, `🔧️wire-language-protocol-*.mjs`

## How to verify

1. Pilot facet tests: `::dsl::parse_grammar(COMPONENT_*)` + dialect assert.
2. After plugin register: `dsl::language("dag.document")` has both grammar and protocol; `language("dag.pack").role == Pack`.
3. `LanguageSession::open(spec, text).diagnostics()` for text; `verify_protocol_bytes(&bytes)` for pack/spr.
4. Full `cargo check`/`test` blocked on this host by Xcode `cc` license (exit 69) — use Linux/CI.

## Compile blockers

- Host `cc` requires Xcode license (see `🧪e2e-xcode-cc-block.txt`).
- Workspace duplicate package name `semio-framework-plugin` (packages vs implementations) observed during check attempt.

## Remaining pack/spr registration coverage

- Pilots only (~4 artifacts / 5 Pack-role files) register Pack/Spr languages today.
- 104 `.protocol.semio` files under plugins; most lack `register_language`.
- Fan out Document/Ops (+ paired protocol) and Pack/Spr registrations across remaining artifacts.

## Host incident note

During this session, `⚡️implementations/**` trees under `🗣️dsl` (59 files) were deleted from the worktree by a concurrent process and restored from `HEAD` via `git show` + rewrite (not `git checkout`). LanguageSpec patches were re-applied. Backups of the patched `lib.rs` / `component.rs` / lsp lib are under `🧪backups/`.
