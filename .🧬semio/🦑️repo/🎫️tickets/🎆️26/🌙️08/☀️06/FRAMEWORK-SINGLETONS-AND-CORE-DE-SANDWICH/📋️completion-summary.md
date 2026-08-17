# 🧬️ Framework Singletons + Core De-Sandwich — Completion Summary

## Shape V2 (in scope)

| Crate | New location | `package.metadata.semio` | nx `project.json` | Owner-root data |
|---|---|---|---|---|
| `semio-framework-hash` | `🔨️modules/#⃣hash/📦️packages/🦀️rust` | ✅ `role=framework`, `id=hash` | n/a (Rust-only) | n/a |
| `semio-framework-schema` | `🔨️modules/🧬️schema/📦️packages/🦀️rust` | ✅ | ✅ `@semio-tech/framework-schema` | ✅ `🔣️entity-kinds.json`, `🤖️generated.rs`, `🤖️generated/` at schema owner root; `build.rs` resolves `../..` |
| `semio-framework-editor` | `🔨️modules/✍️editor/📦️packages/🦀️rust` | ✅ | ✅ `@semio-tech/framework-editor-rs` | n/a |
| `semio-framework-core` | `🧰️framework/📦️packages/🦀️rust` | ✅ | ✅ `@semio-tech/framework-core-rs` | ✅ ts-rs target `🧰️framework/🤖️generated/🟦️manifest.ts`; godfile split into `🎯️action-bus`, `🔺️mesh`, `🖥️platform`, `🧩️ui` (+ `🧩️ui/🧠️kernel`) `🦀️component.rs` |

Dual layout **intentionally retained**: all four old `⚡️implementations/🦀️rust` trees remain workspace members (root `Cargo.toml` unchanged per ticket constraint). **Do not delete** until registrar cutover + full workspace green.

## Standalone `cargo check` (`DEVELOPER_DIR=/Library/Developer/CommandLineTools`)

Evidence: `🧪️cargo-check-core.txt`, `🧪️cargo-check-editor.txt` in this ticket folder.

| Crate | Result | Notes |
|---|---|---|
| `semio-framework-hash` | ✅ pass | Overlay restored; `Finished dev profile`. |
| `semio-framework-schema` | ✅ pass | Overlay restored; cold ~2.5m. |
| `semio-framework-core` | ❌ fail | Transitive `semio-framework-ui-wgpu`: `E0432`/`E0433` `crate::wgpu` — UI restructure uses `#[cfg(feature = "wgpu-engine")]` but `Cargo.toml` defines `engine`; no `pub mod wgpu` shim. **Out of this ticket's tree.** |
| `semio-framework-editor` | ❌ fail | Transitive `semio-framework-compiler-math` missing at `🧮️math/📦️packages/🦀️rust` (math family consolidation). **Out of this ticket's tree.** |

## Overlay hygiene (§8)

All four new `Cargo.toml` files had **header-only** overlay stubs (no `[workspace]` block), so `cargo check --manifest-path` incorrectly walked up to root and failed on unrelated workspace breakage. **Full verification overlays were restored** this session. Keep them until registrar cutover; then delete fenced blocks + nested `target/` + `Cargo.lock` per §8.

## Registrar

See updated `📋️registrar-handoff.md` (§7 blocker revised, §10 table refreshed). Root `Cargo.toml` / `package.json` edits remain registrar-owned.
