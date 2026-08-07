# Wave 3.e — Process machine catalog extensions

## Goal

Move the four domain machine catalogs (`wood`, `concrete`, `metal`, `robotic`) out of the process plugin crate’s compile-time `#[path]` wiring into individually packaged, runtime-installable extension crates under `✏️s/🔌️plugins/🏭️process/🧩️extensions/`, each contributing `Contribution::ProcessMachines` for host app `process3d-play`.

## What changed

### New extension crates (×4)

| Folder | Crate | Component package | `module_id` | Catalog label | Icon |
|--------|-------|-------------------|-------------|---------------|------|
| `🧩️extensions/🪵️wood` | `semio-s-plugin-process-wood` | `semio:process-extension-wood` | `wood` | Wood | `beam` |
| `🧩️extensions/🧱️concrete` | `semio-s-plugin-process-concrete` | `semio:process-extension-concrete` | `concrete` | Concrete | `slab` |
| `🧩️extensions/🔩️metal` | `semio-s-plugin-process-metal` | `semio:process-extension-metal` | `metal` | Metal | `wrench` |
| `🧩️extensions/🤖️robotic` | `semio-s-plugin-process-robotic` | `semio:process-extension-robotic` | `robotic` | Robotic | `cpu` |

Each extension follows the sourcing-beams anatomy:

- Owner `🦀️component.rs` — `MachineCatalog` impl + `catalog()` for tests + `ExtensionBundle` + `extension_exports!(bundle)`
- `📦️packages/🦀️rust/📦️glue.rs` — `#[path]` to owner component only
- `📦️packages/🦀️rust/Cargo.toml` — `role = "extension"`, `extends = "process"`, `contributes = ["process.machines"]`, `crate-type = ["cdylib", "rlib"]`
- `📜️script.ts` + `📋️project.json` — nx `test` / `test-quick` / `test-long` / `test-exhaustive` via `runCargoTestBudgeted`

Bundle shape (wood example):

```rust
ExtensionBundle::new("process-extension-wood", "Process Wood Machines", "0.1.0")
    .extends("process")
    .contributes(Contribution::ProcessMachines {
        app_id: "process3d-play".into(),
        module_id: catalog.catalog_id().into(),
        label: catalog.label().into(),
        icon_id: catalog.icon_id().into(),
        machines_json: serde_json::to_string(&catalog.machines()).unwrap_or_default(),
    })
```

Extensions depend on `semio-s-plugin-process` (path) for `WorkshopMachine`, `MachineCatalog`, and related artifact types — no reverse dependency from the host plugin.

### Process host plugin

- **`📦️glue.rs`**: Removed `catalog_wood` / `catalog_concrete` / `catalog_metal` / `catalog_robotic` path modules under `engine`.
- **`⚙️engine/🦀️component.rs`**: `builtin_installed_catalogs()` now returns **only** `GenericCatalog`. Domain catalogs appear exclusively through `sync_process_machine_contributions` → `CONTRIBUTED_MACHINE_CATALOGS` merge (Wave 2).
- **Deleted** engine topic files: `⚙️engine/{🪵️wood,🧱️concrete,🔩️metal,🤖️robotic}/🦀️component.rs` (logic lives in extensions).
- **`Cargo.toml` description**: No longer claims built-in machine catalogs in the main crate.

### Workspace

Root `Cargo.toml` `members` extended with all four extension package paths (after `semio-s-plugin-process`).

## Runtime behavior

- **Without installed extensions**: Workshop configurator lists only the **Geometry** (`GenericCatalog`) section until the host merges `process.machines` contributions (install/enable extensions or push `contributions_json` via `SetContributions`).
- **With extensions**: OS merges each extension’s `ProcessMachines` contribution; `installed_catalogs()` = generic + contributed, stable order (builtin first, then contributed).

No compile-time fallback path-deps from process → extensions (clean split; avoids cycles and matches “no shim left behind”).

## Verification

| Check | Status |
|-------|--------|
| `cargo check -p semio-s-plugin-process -p semio-s-plugin-process-{wood,concrete,metal,robotic}` | **Blocked** on this machine: Xcode license not accepted (`blake3` C build). Re-run after `sudo xcodebuild -license`. |
| Host `builtin_installed_catalogs()` | **Confirmed** — only `GenericCatalog`; no path-mods for wood/concrete/metal/robotic in process glue/engine. |
| Extension unit tests (catalog integrity + JSON round-trip) | Moved with catalog sources; wood adds `bundle_contributes_wood_machines_for_process3d_play`. |
| Process engine test `sync_process_machine_contributions_merges_hot_installed_catalogs` | Unchanged; still validates merge path without builtin domain catalogs. |

Recommended follow-up when toolchain works:

```bash
cargo test -p semio-s-plugin-process-wood -p semio-s-plugin-process-concrete \
  -p semio-s-plugin-process-metal -p semio-s-plugin-process-robotic
cargo test -p semio-s-plugin-process -- sync_process_machine
cargo clippy -p semio-s-plugin-process -p semio-s-plugin-process-wood -- -D warnings
```

## Files touched (product code)

- `✏️s/🔌️plugins/🏭️process/🧩️extensions/**` (new)
- `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🏭️process/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/⚙️engine/{🪵️wood,🧱️concrete,🔩️metal,🤖️robotic}/🦀️component.rs` (removed)
- `Cargo.toml` (workspace members)

## Out of scope (later waves)

- Wave 4a: root `package.json` workspaces, `.sxt` packaging, vite `/extensions` static copy
- Wave 5: end-to-end install/enable/invoke with `[DEBUG]` logs and ledger restore
- Playground/demonstrator manifest: pre-install the four process extensions so local dev matches prior “always four domain catalogs” UX
