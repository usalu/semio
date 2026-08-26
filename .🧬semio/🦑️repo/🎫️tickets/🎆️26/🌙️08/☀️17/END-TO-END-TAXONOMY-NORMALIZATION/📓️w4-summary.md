# Language Neutral Taxonomy — W4 Summary

## Mechanism landed

- `schemaFacetKinds` (`🧬️data` / `📜️interface`) and `schemaFormats.📜️wit` in `🔣️taxonomy.json`
- `packagingDirNames` global + per-ecosystem (`🦀️rust`: benches + `🟦️typescript`; `🐍️python`: `🎨️styling`)
- Discovery: `schemaFacetFormatEntries`, packaging dir/file scanners, census promotion
- Policy: `policyPackageLanguagePurityBreaches` at **high** priority; `VerifyScript.runGate` block
- `nx` target `verify-package-purity`

## Migrations completed

| Wave | Scope |
|------|--------|
| M1 | WIT merged → `🔌️plugin/🧬️schema/📜️component.wit`; guest bindgen repointed; old `📜️wit/` removed |
| M2 | `📇️registry`, `🪟️window-kits`, `🏪️store` → `🔌️plugin/` root; stale `📦️packages/🟦️typescript/📇️registry` removed |
| M3 | Styling neutral assets → `🎨️styling/` owner root; generators updated |
| M4 | Print `🖼️assets` + `📄️template` at product root; tectonic + font catalog paths |
| M5 | OS `🧫️fixtures`, vscode assets, sourcing windows stubs |
| M6 | `partial_movie_files` deleted + `.gitignore` |
| M7 | `🔣️components.json`, `🔣️ui-axes.json`, `🔣️semio_logo.svg`, CSS hoists, `🛂️adapters.manifest.json`, `dsl_value_serde` → `🔀️dsl-value-serde/🦀️component.rs` |

## Known follow-ups

1. **Plugin host compile** (`semio-framework-plugin-host`): `WasmPluginRuntime` still expects removed `plugin-world` / `PluginWorld` API. `WasmtimeRuntime` + `actor_bindings` path is correct; legacy runtime migration is tracked under MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME.
2. **Residual packaging violations**: `README.md` / duplicate `AGENTS.md` under some `📦️packages/<lang>/` leaves; hub admin `🎨️globals.css` (product-specific hoists).
3. **`verify taxonomy enforce`**: may still report pre-existing `collection-manifest-shape` errors unrelated to this ticket — scope with `--scope` when gating.

## Baseline vs acceptance

Re-run `bun ./📜️script.ts verify taxonomy report` and `policy` after host runtime migration for full green gate.
