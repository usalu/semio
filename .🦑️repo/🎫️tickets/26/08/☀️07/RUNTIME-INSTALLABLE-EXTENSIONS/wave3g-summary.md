# Wave 3.g — Imperative module extensions

Ticket: `26/08/07/RUNTIME-INSTALLABLE-EXTENSIONS`

## Status: **Complete** (code); **cargo verify blocked** (workspace flow path + Xcode license)

## Goal

Five imperative modules (`core`, `text`, `math`, `logic`, `control`) are packaged `ExtensionBundle` crates contributing `Contribution::ImperativeModule`, with host consumption via `sync_imperative_module_contributions` — no `linked-modules` compile-time aggregation.

## Delivered

### Extension crates (×5)

Each under `✏️s/🔌️plugins/📜️imperative/🧩️extensions/{🫀️core,📝️text,🧮️math,🧠️logic,🎮️control}/`:

- `ExtensionBundle` + `extension_exports!`, `extends = "imperative"`, `contributes = ["imperative.module"]`
- `imperative_module_contribution()` + `handler(imperative.module/evaluate, …)` using `semio-s-imperative-extension-sdk`
- `Cargo.toml`: `cdylib` + `rlib`, component metadata, framework plugin guest deps

### `semio-s-imperative` registry

- Removed `linked-modules` feature and `bootstrap_linked_modules` / path-deps to extension crates
- `imperative_module_registry()` composes only from synced `contributions_json` (+ optional `register_native_imperative_module` for in-process tests)
- Contributed operators without a native registrar register invoke-backed stubs (`PendingExtension` until shell `invokeExtension` runs)

### Imperative play host

Already wired (this wave confirmed):

- `ImperativeConfig.contributions_json` + `SetContributions` config op → `sync_imperative_module_contributions`
- `setContributions` action + `🎮️commands/🧩️contribution`
- `render()` calls sync from config

### Shell

- `buildContributionsJson` omits disabled extensions (ledger `enabled: false`)
- Extensions panel lists catalog + ledger; install URL/file, uninstall, enable toggle

## Verification (when toolchain works)

```bash
cargo test -p semio-s-imperative
cargo test -p semio-s-plugin-imperative-core -p semio-s-plugin-imperative-text -p semio-s-plugin-imperative-math
bun nx run @semio-tech/imperative-plugin:test-quick
```

Local agent: full-workspace `cargo check` fails on missing `semio-s-plugin-flow-extension-core` path (unrelated in-flight flow wave); native `cc` also blocked by Xcode license.

## Files touched

- `✏️s/🔨️modules/📜️imperative/📇️registry/🦀️component.rs`
- `✏️s/🔨️modules/📜️imperative/📦️packages/🦀️rust/Cargo.toml`, `📦️glue.rs`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/*/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/.../ShellHost/🟦️component.tsx` (extension contribution filter + file install)
