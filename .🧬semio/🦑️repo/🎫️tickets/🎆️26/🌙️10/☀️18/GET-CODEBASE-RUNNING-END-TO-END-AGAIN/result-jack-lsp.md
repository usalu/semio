# result-jack-lsp (executor jack-lsp, wave 1022a)

## Decision: remove the dead Jack LSP worker package and every reference (no bridge crate)

Evidence (all checked in the tree, 10-23):
- `dsl_lsp` exists (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧠️lsp/🦀️.rs`, `pub mod lsp` of `semio_framework_dsl`) but it is a generic subset: `handle_json_rpc` answers only `initialize`, `semanticTokens/full`, `shutdown` (everything else is `{}`), one response string per request. It has no hover/completion JSON-RPC, no snapshot loading and no `jack/loadSnapshot` or graph domain, which is exactly what `serveJackLspV1` drives (`loadSnapshotForDomain`, `handleMessageJson` returning a message array).
- Jack's `LanguageSpec`s (`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🔨️modules/🧩️registration/🦀️.rs`) all use `passthrough_hooks`, so `LanguageSession` over them yields no tokens or completions. A bridge over `dsl_lsp` would be a hollow engine.
- The real Jack language service (`semio-s-artifact-trinity-jack-language-service`: parse, complete, lint, hover on a `JackSnapshot`) has no JSON-RPC surface at all; the worker's protocol would have to be invented (new schema, new crate pulling os-kernel/plugin/graph-dsl into a cdylib) for a host that does not exist.
- The old wasm "engine" never worked: git `fd14da55f18^` shows `semio-s-plugin-trinity-jack-lsp` was a 4-line `pub use dsl_lsp::lsp::{handle_json_rpc, LanguageSession}` cdylib with no `wasm-bindgen` exports; the ignored `rust/pkg/trinity_jack_lsp_bg.wasm` on disk was 519 bytes and exported nothing (no `JackLspSession`).
- Zero consumers: no TypeScript, React or Rust code in the repo imports `serveJackLspV1`, `@semio-tech/trinity-jack-lsp(-worker)`, spawns the worker, or speaks LSP (no monaco/codemirror/`textDocument/` anywhere outside the worker's own tests). Writer uses `LanguageSession` in-process from Rust and is untouched.
- AGENTS.md: greenfield, no compatibility layers/shims, no dead code, do not leave speculative infrastructure. Deleting is the clean long-term solution; a real Jack editor LSP can be designed schema-first when a host exists.

## Changes (live edits)
- Deleted the whole module `✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/` (AGENTS.md, `🟦️.ts`, `🔖️protocol`, `🧪️tests`, `📦️packages/🟦️typescript`, ignored `📦️packages/🦀️rust/{pkg,node_modules}`). `🔨️modules/🔌️jack/🐚️shell` stays.
- Removed the two workspace lines (`…/🧠️lsp/📦️packages/🟦️typescript` and `…/🦀️rust/pkg`) from `✏️s/package.json`, `🏢️semio-tech/package.json`, `♻️mit-bestand/package.json`, `🌎️hub/package.json`.
- `📋️build-owners.json`: removed owner `@semio-tech/trinity-jack-lsp-worker`.
- `🔒️dependencies.json`: removed the two `…/🧠️lsp/…/package.json` rows.
- `…/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/📇️catalog.json`: removed the `@semio-tech/trinity-jack-lsp` wasm engine row.
- Fixtures (repo-lib): `⚡️caching/🧪️tests/🧫️fixtures/nx-contract/🔣️.json` (wasm output row removed), `🧪️tests/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json` (owner row removed) and its test `🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts` (owner count 53 -> 52).
- Intentionally untouched: `🧪️tests/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json` is a dated (2026-08-27) non-authoritative census of an already-gone `📦️glue.rs`; its test only asserts it stays non-authoritative. The generic framework taxonomy name `🧠️lsp` stays (the dsl module uses it).

## Lock entries
See "Status" below.

## Status
(in progress: lock step + verification)
