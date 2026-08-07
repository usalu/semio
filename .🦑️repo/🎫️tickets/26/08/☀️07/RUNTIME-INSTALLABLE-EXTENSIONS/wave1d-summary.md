# Wave 1.D Summary — Durable Extension Ledger + ShellHost Lifecycle

Ticket: `26/08/07/RUNTIME-INSTALLABLE-EXTENSIONS`
Date: 2026-08-07

## Done

### 1. SpaceProjection ledger

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs`

- Added `InstalledExtension` (`extensionId`/`version`/`sourceUri`/`packageHash`/`enabled`, camelCase serde + `dsl::DslRecord`).
- Added `SpaceProjection.extensions: Vec<InstalledExtension>` with `#[serde(default)]`.
- Ops: `InstallExtension`, `UninstallExtension`, `SetExtensionEnabled`.
- Diff apply/absorb/diff/backwards mirror InstallProgram/UpsertUser patterns (upsert-by-id).
- Extended existing op/backwards/diff tests in the same file.
- Wired `pub mod space` into os host glue (`🖥️host/📦️packages/🦀️rust/📦️glue.rs`) so `crate::space::*` resolves under `os-host-full`.

### 2. ShellHost lifecycle

File: ShellHost `🟦️component.tsx`, `#region 🔌️PluginRuntime`

- `extensionLedger` state + `installExtension` / `installExtensionFromFile` / `uninstallExtension` / `setExtensionEnabled`.
- Store via POST `/extensions/install` when available; space ledger ops dispatched best-effort on the active session.
- **Preserved** existing `invokeExtension` host-effect branch (not reverted).
- `setContributions` pushes to every loaded plugin with apps + a live instance (try/catch); disabled extensions filtered from `contributionsJson`.
- Plugins panel excludes `EXTENSION_TARGETS`; Extensions panel is separate.

### 3. ChromePanels Settings split

- Kept Plugins panel.
- Added Extensions panel id `framework.settings.extensions`, grouped by `extends`/host.
- Install from URL (`window.prompt`) and file; Uninstall; Enable/Disable toggle.
- `ExtensionsHostApi` wired like `PluginsHostApi`.

### 4. Tests

- In-file space tests extended for the three new ops.
- `cargo test -p semio-framework-os` could not run here: Xcode license blocks `cc` (exit 69). See `space-test.log` / `space-test2.log` in this ticket folder.

## Key files

- `…/🪐️space/🦀️component.rs`
- `…/🖥️host/📦️packages/🦀️rust/📦️glue.rs`
- `…/ShellHost/🟦️component.tsx`
- `…/ChromePanels/🟦️component.tsx`
