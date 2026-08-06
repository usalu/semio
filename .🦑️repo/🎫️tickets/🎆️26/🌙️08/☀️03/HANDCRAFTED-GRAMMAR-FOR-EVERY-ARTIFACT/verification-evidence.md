# P6/P7 verification evidence (2026-08-06)

## TypeScript
- `bun test` in `🦑️repo/📚️lib` (taxonomy/discovery): **129 pass**, 9 fail (unrelated pre-existing: dependency-boundary, ui.css, micro-commit, playground ports, command budgets, resolveCargoPackageName).

## Rust
- Host `cargo test` / `cargo check` for proc-macro and test binaries requires a working platform linker on this macOS host (`cc` exit 69 / Xcode SDK path). **No Xcode in dev workflow** — use Linux devcontainer/CI for full `cargo test` linkage.
- `cargo check --target wasm32-unknown-unknown` for writer: compiles through dependency graph until host proc-macro `dsl_derive` link step (same linker constraint).

## Writer
- `[DEBUG] writer.main tokens_json=…` emitted from main window render when language registry provides tokens (`language_tokens_json` / idiom classify).
- `OpenDocument` command resolves extension via `dsl::language_for_extension`.
- Regex tokenizer removed; jack/wire via `DslIdiom` + trinity semantic tokens.

## Engine
- `WireEdgeLabel` + pack wire `0b100` label bit.
- `verify_protocol_bytes` + dag pack/spr specs.
- 520 facet spec/ts seeds + 29 plugin TS packages.

## Ticket close
- Repo MCP `ticket_close` unavailable in agent session; close manually with this file + `progress-session.md` + contracts.
