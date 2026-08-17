# Shape V2 package entry — GLUE (normative correction 2026-08-06)

Packages are glue ONLY. Package-level entry files MUST be named `glue`, never `lib` or `component`.

## Rust
- File: `📦️packages/🦀️rust/📦️glue.rs` (rename from `📦️lib.rs`)
- Cargo.toml: `[lib] path = "📦️glue.rs"`
- Content: only `#[path]` wiring, `pub use`, feature/cfg gates, `extern crate` aliases — NO domain logic
- Domain lives at owner tree: `<owner>/…/🦀️component.rs`

## TypeScript
- Packaging entry at package level: `🟦️glue.ts` (not domain `component.ts` inside packages)
- Re-export only; domain in owner `🟦️component.ts`

## Forbidden under packages/
- `⚡️implementations`
- Domain bodies in entry files
- Files named `lib.rs` / `component.rs` at package root (targets may keep thin glue similarly named `📦️glue.rs`)
