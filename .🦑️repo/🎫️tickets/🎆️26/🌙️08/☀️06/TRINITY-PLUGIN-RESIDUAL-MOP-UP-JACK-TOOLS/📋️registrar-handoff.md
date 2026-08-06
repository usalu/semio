# 📋️ Registrar Handoff — Trinity Plugin Residual Mop-Up (Jack Tools)

**None needed.** Root `Cargo.toml` already lists:

```toml
    "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust",
    "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust",
```

Root `package.json` workspaces already include `…/🧠️lsp/📦️packages/🟦️typescript` (`@semio-tech/trinity-jack-lsp-worker`).

If a branch still has old jack paths, swap `🐚️shell/⚡️implementations/🦀️rust` → `🐚️shell/📦️packages/🦀️rust` and
`🧠️lsp/⚡️implementations/🦀️rust` → `🧠️lsp/📦️packages/🦀️rust`. Crate names frozen:
`semio-s-plugin-trinity-jack-shell`, `semio-s-plugin-trinity-jack-lsp`.
