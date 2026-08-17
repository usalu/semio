# 📋️ Registrar Handoff — Trinity Plugin Residual Mop-Up (Jack Tools)

**None needed for root manifests.** Confirmed present:

```toml
    "✏️s/🔌️plugins/🔱️trinity/📦️packages/🦀️rust",
    "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🐚️shell/📦️packages/🦀️rust",
    "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust",
```

Root `package.json` workspaces already include:
- `…/🧠️lsp/📦️packages/🦀️rust` (`@semio-tech/trinity-jack-lsp`)
- `…/🧠️lsp/📦️packages/🟦️typescript` (`@semio-tech/trinity-jack-lsp-worker`)

## New crates (Rule B residuals — stay separate installable units)

| crate / package | path | role |
|---|---|---|
| `semio-s-plugin-trinity-jack-shell` | `…/🔌️jack/🐚️shell/📦️packages/🦀️rust` | bin tool |
| `semio-s-plugin-trinity-jack-lsp` | `…/🔌️jack/🧠️lsp/📦️packages/🦀️rust` | LSP shim (installable; stays separate) |
| `@semio-tech/trinity-jack-lsp-worker` | `…/🔌️jack/🧠️lsp/📦️packages/🟦️typescript` | TS worker |

## Old member lines (if any branch still has them)

```toml
    "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🐚️shell/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/⚡️implementations/🦀️rust",
```

→ swap to the `📦️packages/🦀️rust` paths above. Crate names frozen.

## Handoff JSON

```json
{
  "owner": "ueli",
  "ticketPath": ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/TRINITY-PLUGIN-RESIDUAL-MOP-UP-JACK-TOOLS",
  "newCrates": [
    "semio-s-plugin-trinity-jack-shell",
    "semio-s-plugin-trinity-jack-lsp",
    "@semio-tech/trinity-jack-lsp-worker"
  ],
  "oldMemberLines": [
    "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🐚️shell/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/⚡️implementations/🦀️rust"
  ],
  "tests": {
    "lsp": {"check": "pass", "clippy": "pass", "test": "pass (0 tests)", "via": "ticket verify-lsp overlay"},
    "shell": {"check": "blocked-external", "test": "blocked-external", "note": "infinite-canvas build.rs still points at assets/packages/typescript shortcodes; assets live at assets/icons after concurrent de-sandwich. Prior session verified shell_loads_fixture against green root."},
    "tsWorker": {"vitest": "pass (0 tests)", "JackLspSession": "deferred — wasm export missing in lsp shim"}
  },
  "status": "done",
  "residualsDeferred": [
    "JackLspSession #[wasm_bindgen] surface for component.ts (pre-existing regression; follow-up ticket)",
    "shell live cargo re-verify once assets shortcodes path / root cycle clear"
  ]
}
```
