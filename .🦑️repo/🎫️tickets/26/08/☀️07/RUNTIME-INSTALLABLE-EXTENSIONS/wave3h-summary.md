# Wave 3.h — Sourcing & playbook extensions under ExtensionBundle

Ticket: `26/08/07/RUNTIME-INSTALLABLE-EXTENSIONS`

## Status: **Complete** (code); **runtime E2E not re-run** (dev server / Xcode)

## Sourcing (beams / windows / slabs)

| Change | Detail |
|--------|--------|
| Bundle type | `PluginBundle` + `plugin_exports!` → `ExtensionBundle` + `extension_exports!` |
| Metadata | `extends = "sourcing"` added on all three `Cargo.toml` (`role = "extension"`, `contributes = ["sourcing.module"]` already present) |
| Host engine | `sourcing_modules()` no longer merges compile-time `BeamsModule` / `WindowsModule` / `SlabsModule`; modules come **only** from `sync_sourcing_module_contributions` |
| Tests | `available_modules_tracks_contributed_modules` seeds a beams contribution JSON instead of assuming built-in registry |

Extensions remain workspace members (`Cargo.toml` lines 43–45). Install/enable path: ShellHost loads extension wasm from `/extensions` or dev catalog → `buildContributionsJson` → `sourcing-curate` `setContributions` → `sync_sourcing_module_contributions`.

## Playbook (procedural)

| Change | Detail |
|--------|--------|
| Bundle type | `module_bundle()` returns `ExtensionBundle` with `.extends("playbook")`, keeps `register_document_app::<ModuleApp>` for params/preview UI |
| Export | `extension_exports!(module_bundle)` replaces `plugin_exports!` |
| Metadata | `extends = "playbook"` on `Cargo.toml` |
| Test | `bundle.manifest` field (not `manifest()`) |

`buildingComponent` `PlaybookBlockKind` contribution unchanged; invoke path for block preview still flows through the module document app + host external slots.

## Chrome / Shell (Wave 1.D completion)

Documented here because it gates install/enable UX for these extensions:

- **ChromePanels**: `ExtensionsHostApi.installFromFile` + “Install from file” control (`.sxt` / octet-stream POST to `/extensions/install`)
- **ShellHost**: `installExtensionFromFile`, extensions tab in bottom-right (`frameworkExtensionsTabs`), plugins panel excludes `EXTENSION_TARGETS` ids
- Contribution push respects extension ledger `enabled` flag (primary + spawned refresh paths)

## Recommended verification

1. Dev server: install `sourcing-module-beams` from built `.sxt` URL → enable → open sourcing-curate → pool shows beams typology.
2. Install all three sourcing extensions → `available_modules()` lists three sections.
3. Install `playbook-module-procedural` → playbook builder palette includes `buildingComponent`.
4. Disable extension in Settings → Extensions → confirm palette/module disappears without uninstall.
5. `[DEBUG]` logs: `extension store install ok`, `setExtensionEnabled`, `setContributions push`.

Blocked locally: workspace `cargo check` (missing flow-extension-core path in tree) and Xcode license for `blake3` C build.

## Files touched

- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/{🪵️beams,🪟️windows,🧱️slabs}/🦀️component.rs` + `Cargo.toml`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs` + `Cargo.toml`
- `🧰️framework/.../ChromePanels/🟦️component.tsx`, `ShellHost/🟦️component.tsx`
