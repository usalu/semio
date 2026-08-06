# 📋️ Registrar Handoff — S Plugin Package Glue Purity

## Summary

All **50** `✏️s/**` plugin and s-module Rust packages under `📦️packages/🦀️rust/` now use **`📦️glue.rs`** as the `[lib]` entry (`Cargo.toml` `path = "📦️glue.rs"`). Zero `📦️lib.rs` remain under that layout.

**38** packages were rename-only (taxonomy `#[path]` glue preserved; no extra `../../` on `#[path = "."]` groupings).

**12** fat zero-path entries were **extracted** to owner `🦀️component.rs` with thin package glue (`#[path = "../../🦀️component.rs"]` + `pub use`).

## Extracted owners

| Owner | Notes |
| --- | --- |
| `🖍️draw/🔄️fsm` | Full kernel → `🦀️component.rs` |
| `🖍️draw/🔄️fsm/✨️macros` | Domain in `🦀️component.rs`; glue keeps `#[proc_macro]` surface + `pub use component::*` for `crate::parse` hoisting |
| `📖️playbook/🧩️extensions/🌀️procedural` | Module app body extracted |
| `🌊️flow/🧩️extensions/🏗️bim` | BIM operators extracted |
| `📜️imperative/🧩️extensions/{🫀️core,🧠️logic,📝️text,🧮️math,🎮️control}` | Neural operators extracted |
| `🪵️sourcing/🧩️extensions/{🪵️beams,🧱️slabs,🪟️windows}` | Module bundle glue extracted |

## Registrar

No new workspace members. Existing member paths unchanged (still `…/📦️packages/🦀️rust`). Only `[lib] path` filename changed inside those manifests.

Published crate / component ids unchanged.

## Verification

- `cargo check -p semio-s-plugin-block -p semio-s-plugin-draw-fsm -p semio-s-plugin-playbook-procedural` attempted from repo root.
- **Partial compile** reached `semio-s-plugin-draw-fsm-macros` before host issues (see blockers).
- After proc-macro glue fix, macros crate logic compiles in isolation when workspace loads.

## Blockers (environment / parallel work)

1. **Xcode SDK license** on this host (`xcodebuild -license`) — blocks linking proc-macro and DSL derive dylibs.
2. **Workspace load failure**: missing `🧰️framework/🔨️modules/📚️compiler/⚡️implementations/🦀️rust/Cargo.toml` (concurrent framework crate consolidation). Blocks full-workspace `cargo check` until compiler registrar handoff lands.

## Ticket artifacts

- `🔧️glue-migrate.mjs` — bulk rename/extract driver
- `🧪glue-migrate-result.json` — machine log

```json
{
  "status": "done",
  "renamed": 38,
  "extracted": [
    "🔌️plugins/🌊️flow/🧩️extensions/🏗️bim",
    "🔌️plugins/📜️imperative/🧩️extensions/🫀️core",
    "🔌️plugins/📜️imperative/🧩️extensions/🧠️logic",
    "🔌️plugins/📜️imperative/🧩️extensions/🧮️math",
    "🔌️plugins/📜️imperative/🧩️extensions/🎮️control",
    "🔌️plugins/📜️imperative/🧩️extensions/📝️text",
    "🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams",
    "🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs",
    "🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows",
    "🔌️plugins/🖍️draw/🔄️fsm/✨️macros",
    "🔌️plugins/🖍️draw/🔄️fsm",
    "🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural"
  ],
  "blockers": [
    "Host Xcode license blocks proc-macro dylib link on this machine",
    "Missing framework compiler package Cargo.toml breaks workspace cargo until parallel consolidation completes"
  ],
  "handoffPath": ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/S-PLUGIN-PACKAGE-GLUE-PURITY/📋️registrar-handoff.md"
}
```
