# Registrar handoff — Framework singleton package purity

## Norm applied

Packages are glue-only (`📦️glue.rs`). Domain lives at module owners (`🦀️component.rs`).

## Completed

| Module | Owner domain | Package entry |
|--------|--------------|---------------|
| ✍️editor | `🔨️modules/✍️editor/🦀️component.rs` (moved from ~2480 LOC package lib) | `📦️glue.rs` → `#[path]` + `pub use` |
| #⃣hash | `🔨️modules/#⃣hash/🦀️component.rs` | `📦️glue.rs` |
| 🧬️schema | `🔨️modules/🧬️schema/🦀️component.rs` (`include!("🤖️generated.rs")` path fixed) | `📦️glue.rs` |
| 🖱️ui (top) | unchanged (feature gates only) | `📦️lib.rs` → `📦️glue.rs`; targets point to `📦️glue.rs` |
| 🖱️ui ⌨️tui target | `🔨️modules/🖱️ui/⌨️tui/🦀️component.rs` (extracted from target lib) | `🎯️targets/⌨️tui/📦️glue.rs` |
| 🖱️ui 🧊️wgpu target | already path-wired to sibling `🦀️*.rs` + `🦀️component.rs` | `📦️lib.rs` → `📦️glue.rs` (rename only) |
| 🧮️math | already path-wired | `📦️lib.rs` → `📦️glue.rs` |
| 🗺️surface | already path-wired | `📦️lib.rs` → `📦️glue.rs` |
| framework `semio-framework-core` | `optional_json_to_dsl` moved to `🧩core/🎯️action-bus/🦀️component.rs` | `📦️lib.rs` → `📦️glue.rs` (core module `#[path]` wiring unchanged) |

`semio-framework-core` `📜️script.ts` typegen header strings updated to reference `📦️glue.rs`.

## `cargo check` (DEVELOPER_DIR=/Library/Developer/CommandLineTools)

- `semio-framework-hash` — OK
- `semio-framework-editor` — OK
- `semio-framework-core` — OK
- `semio-framework-schema` — OK

## Out of scope (not touched)

- OS implementations, compiler, `🖱️ui/🎨️styling` package (still `📦️lib.rs`).
- Root `Cargo.toml` / `package.json` / lockfiles / `go.work` / `nx.json`.

## Follow-ups for registrar / taxonomy

- Broadcast `📦️lib.rs` → `📦️glue.rs` for framework singleton Rust packages in taxonomy / plugin entry-file detection (`has_entry_file` still checks `📦️lib.rs`).
- Optional: further split `🖱️ui/🧊️wgpu/📦️glue.rs` re-export surface if glue-only means zero `pub use` blocks (currently path-wiring + curated re-exports).

## Ticket id

`26/08/06/FRAMEWORK-SINGLETON-PACKAGE-PURITY`

## Registrar follow-up applied
- Updated `🔣️taxonomy.json` entryFilenames / packagingFileNames: `📦️lib.rs` → `📦️glue.rs`.
- Updated root `📜️script.ts` default package entry fallback to `📦️glue.rs`.
- Ticket closed.
