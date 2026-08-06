# 📋️ Registrar Handoff — Sourcing Plugin Extensions De-Sandwich

**None needed for member lines.** Root `Cargo.toml` `[workspace] members` already point at Shape V2 locations:

```toml
    "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/📦️packages/🦀️rust",
    "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/📦️packages/🦀️rust",
    "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/📦️packages/🦀️rust",
```

No `[workspace.dependencies]` entries name these extension crates (dynamic plugin leaves).
No root `package.json` workspace edits (Rust-only extensions).

If a branch still has old sandwich members, swap `…/<ext>/⚡️implementations/🦀️rust` → `…/<ext>/📦️packages/🦀️rust`.

Published crate names **frozen**: `semio-s-plugin-sourcing-beams`, `semio-s-plugin-sourcing-slabs`, `semio-s-plugin-sourcing-windows`.

Component package ids **frozen**: `semio:sourcing-module-beams`, `semio:sourcing-module-slabs`, `semio:sourcing-module-windows`.

## Verification

Ticket-local overlays `verify-{beams,slabs,windows}` + `verify-shims/` (ui-wgpu compat for concurrent UI-family merge):
all three crates `cargo check` + `cargo test` green (1 unit test each).

Root workspace `cargo check -p semio-s-plugin-sourcing-*` remains blocked until UI-family/core registrar
repoints `semio-framework-core` ui_wgpu path onto `semio-framework-ui`. See `🧩handoff.json`.
