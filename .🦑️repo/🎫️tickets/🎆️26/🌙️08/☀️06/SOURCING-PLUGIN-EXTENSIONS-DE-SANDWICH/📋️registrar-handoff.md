# 📋️ Registrar Handoff — Sourcing Plugin Extensions De-Sandwich

**None needed.** Root `Cargo.toml` `[workspace] members` already point at the Shape V2 locations (confirmed
at lines 172–174):

```toml
    "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/📦️packages/🦀️rust",
    "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/📦️packages/🦀️rust",
    "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/📦️packages/🦀️rust",
```

No `[workspace.dependencies]` entries name these extension crates (dynamic plugin leaves). No root
`package.json` workspace edits apply (Rust-only extensions).

If a future registrar pass still has the old `⚡️implementations` paths on a branch, swap each member line:

| Old | New |
| --- | --- |
| `…/🪵️beams/⚡️implementations/🦀️rust` | `…/🪵️beams/📦️packages/🦀️rust` |
| `…/🧱️slabs/⚡️implementations/🦀️rust` | `…/🧱️slabs/📦️packages/🦀️rust` |
| `…/🪟️windows/⚡️implementations/🦀️rust` | `…/🪟️windows/📦️packages/🦀️rust` |

Published crate names frozen: `semio-s-plugin-sourcing-beams`, `semio-s-plugin-sourcing-slabs`,
`semio-s-plugin-sourcing-windows`. Component ids frozen: `semio:sourcing-module-beams/slabs/windows`.
