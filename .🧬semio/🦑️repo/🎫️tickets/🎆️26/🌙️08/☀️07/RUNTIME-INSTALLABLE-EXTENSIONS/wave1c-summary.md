# Wave 1.C — Catalog and PluginSource

## Registry (`📇️registry/📜️script.ts`)

- Extended `PluginRegistryEntry` with `role` (`"plugin"` | `"extension"`, default `"plugin"`), optional `extends`, and `capabilities` (from `[package.metadata.semio].contributes`).
- `parsePluginCargo` reads `role`, `extends`, and `contributes` from the semio metadata block.
- Generated `🟦️plugins.ts` now emits:
  - `PLUGIN_BUILD_TARGETS` — crates with `role: "plugin"` only (32 rows).
  - `EXTENSION_TARGETS` — crates with `role: "extension"` (5 rows: flow-extension-bim, playbook-module-procedural, sourcing-module-beams/slabs/windows).
  - `extensionModuleUrl()` — `/extensions/<id>/<js>` URLs parallel to `pluginModuleUrl`.
- `🔣️plugins.json` includes the new fields for all 37 catalog rows.

## Framework core (`🧩core/🟦️component.ts`, `#region 🔌️PluginSource`)

- `EXTENSION_SOURCE_WATCH_PATH` = `/extensions/watch`.
- `createExtensionSource()` — `PluginSource` with `id: "extensions"`, catalog from `EXTENSION_TARGETS`, module URLs under `/extensions/`.
- `multiplexPluginSources(...)` — merges `list()`, tries `moduleUrl()` on each child, fans out `subscribe()`.
- `resolvePlaygroundBoot` unions `PLUGIN_BUILD_TARGETS` + `EXTENSION_TARGETS` with role-appropriate module URLs.

## ShellHost

- `pluginSource` useMemo now `multiplexPluginSources(createDevPluginSource(registry), createExtensionSource())`.

## Exports

- `@semio-tech/framework-core` / renderer-react re-export `createExtensionSource`, `multiplexPluginSources`.
- `@semio-tech/framework-os-dev` re-exports `EXTENSION_TARGETS`, `extensionModuleUrl`.

## Verification

- `bun …/📇️registry/📜️script.ts generate` — 37 plugin crates, catalog written.
- `bun …/📇️registry/📜️script.ts check` — exit 0 (catalog fresh, playground validation green).

## Follow-ups (other waves)

- W1.B: mount `/extensions` static route + SSE watch so `createExtensionSource` receives real install events.
- W1.D: extension ledger + Settings UI; contributions pushed to all consumers.
- Sourcing extension crates still lack `contributes = ["sourcing.module"]` in Cargo.toml (capabilities empty until W2/W3).
