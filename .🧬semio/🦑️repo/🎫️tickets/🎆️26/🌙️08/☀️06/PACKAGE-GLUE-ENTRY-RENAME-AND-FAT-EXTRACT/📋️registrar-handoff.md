# Registrar Handoff — Package Glue Entry Rename And Fat Extract

Ticket: `26/08/06/PACKAGE-GLUE-ENTRY-RENAME-AND-FAT-EXTRACT`
Date: 2026-08-06

## Summary

- Renamed **72** package-scoped Rust entries `📦️lib.rs` → `📦️glue.rs` under `**/📦️packages/**`.
- Remaining package `📦️lib.rs`: **0**.
- Updated matching package `Cargo.toml` `[lib] path = "📦️glue.rs"` (70 confirmed; bin-only trinity jack shell has no `[lib]`).
- Extracted priority fat package domain into owner-level `🦀️component.rs`; glue left as `#[path]` + `pub use` (macros: private `mod component` + crate-root `#[proc_macro]` wrappers).

## Root-file changes needed (registrar-owned — NOT edited here)

1. **`Cargo.toml` (workspace members)** — currently broken: member path missing on disk:
   - `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/👪️family/🧑‍🍳️recipe/⚡️implementations/🦀️rust`
   - Family dir only has `🦀️component.rs` + grammar; no `⚡️implementations/` tree.
   - Blocks all `cargo check` via workspace load. Fix/remove member when OS-impl agent finishes recipe migration.

2. **OS `⚡️implementations/**/📦️lib.rs`** — intentionally NOT renamed (other agents). After those rename to `📦️glue.rs`, registrar may need to refresh any generated manifests / nx file-maps that still say `lib.rs` for impl crates.

3. **Root `package.json` / `bun.lock` / `go.work` / `nx.json`** — no changes required from this ticket for the Rust glue rename. Nx file-map caches under `.nx/` were touched by string replace and will regenerate.

## Extracted owners (this ticket scope)

- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🦀️component.rs` (767 LOC)
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🫀️core/🦀️component.rs` (150 LOC)
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧠️logic/🦀️component.rs` (153 LOC)
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🧮️math/🦀️component.rs` (137 LOC)
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/🎮️control/🦀️component.rs` (58 LOC)
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/📝️text/🦀️component.rs` (144 LOC)
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/🦀️component.rs` (44 LOC)
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/🦀️component.rs` (44 LOC)
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/🦀️component.rs` (44 LOC)
- `✏️s/🔌️plugins/🖍️draw/🔄️fsm/✨️macros/🦀️component.rs` (1522 LOC)
- `✏️s/🔌️plugins/🖍️draw/🔄️fsm/🦀️component.rs` (2419 LOC)
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs` (894 LOC)

### Macros special case

`✏️s/🔌️plugins/🖍️draw/🔄️fsm/✨️macros/📦️packages/🦀️rust/📦️glue.rs` keeps `#[proc_macro]` / `#[proc_macro_derive]` wrappers at crate root (rustc requirement) delegating to `component::expand_*`. No `pub use component::*` (proc-macro crates cannot export non-macro items). Sibling `crate::parse` / `crate::analyze` inside the owner component were rewritten to `super::`.

### FSM sibling paths

`✏️s/🔌️plugins/🖍️draw/🔄️fsm/🦀️component.rs` internal `crate::{host,inspect,kernel,persist,testing}` references rewritten to `super::` so `#[path] mod component; pub use component::*;` resolves.

## Remaining fat glue (outside this agent scope)

### Zero `#[path]` (domain still in package glue)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs` (1112 LOC)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs` (858 LOC)

### Large files still notable

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📦️glue.rs` (1206 LOC, 8 #[path])
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs` (1112 LOC, 0 #[path])
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/📦️glue.rs` (1078 LOC, 368 #[path])
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs` (858 LOC, 0 #[path])
- `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📦️glue.rs` (483 LOC, 175 #[path])

Notes:
- `norm` / `puzzle` large LOC with hundreds of `#[path]` are taxonomy wiring, not domain-in-glue.
- Renderer wgpu target (1206 LOC / 8 path) looks like remaining domain-in-package-target glue — OS/renderer agents.
- Do **not** touch OS `⚡️implementations` trees from this ticket.

## Hardcoded reference updates

- Package-scope / script references `📦️lib.rs` → `📦️glue.rs` updated (see `🧪hardcoded-lib-updates.json`).
- Paths pointing into `⚡️implementations/**` were **reverted** back to `📦️lib.rs` so OS impls keep working until those agents rename.

## Verification

- Spot `cargo check` logs: `🧪cargo-check-spot.log`, `🧪cargo-check-summary.json`.
- Before workspace breakage: `semio-s-plugin-draw-fsm-macros` compiled successfully; `sourcing/beams` had also passed once.
- After concurrent workspace member deletion (recipe impl), **all** cargo checks fail at workspace load — blocker for registrar / OS-impl agent.

## Counts

| Metric | Value |
|--------|-------|
| Package `📦️glue.rs` after rename | 72 |
| Package `📦️lib.rs` remaining | 0 |
| Priority owners extracted | 12 |
| Remaining 0-path fat (out of scope) | 2 |


## Registrar follow-up (2026-08-06)
- Removed duplicate workspace member `🔌️plugin/⚡️implementations`; kept `🔌️plugin/📦️packages`; retargeted workspace.dep.
- Ticket closed.

- Removed stale recipe implementations member (dir deleted by OS agent).
- Retargeted renderer wgpu plugin path → packages.
