# Process Suites

All suites green. No product source file was changed; every failure was environmental.

| Suite | Result |
|---|---|
| metal (rust package + component) | 5 passed, 0 failed |
| wood | 6 passed, 0 failed |
| robotic | 5 passed, 0 failed |
| concrete | 5 passed, 0 failed |
| bridge (`semio-process-mutation-bridge`) | 3 passed, 0 failed (was 0 tests) |
| hub `semio-hub-process` (🌎️hub workspace) | 2 passed, 0 failed (was 0 tests) |
| hub `semio-hub-process-component` | 1 passed (`descriptor_is_fresh`), 0 failed |
| host (`semio-s-artifact-process-process3d-host`) | 6 passed, 0 failed |
| `@semio-tech/process-js` | 8 passed, 0 failed (bun test of the 4 runtime source entries, 3 new ajv tests) plus the retained-route audit (`routes=34 migrated=34`) validated |

## Environmental root causes (no assertion or budget touched)
1. First bridge run: rustc "Interrupted system call" while the disk was full (4.5 GB free). Test binaries over 20 MB deleted, rerun passed.
2. `@semio-tech/process-js` could not resolve `smol-toml`: the dependencies image `.cache/b/d/fixed.dmg` had a zeroed header (corrupt, unmountable). Removed the corrupt image entries and ran `repo dependencies ensure`, which recreated and repopulated it.
3. Host and hub failed in `semio-ui-theme`: the gitignored generated `🔤️tokens/🦀️.rs` lacked `CHIP_TINT_PERCENT`. Regenerated it from the projection (`🗑️generated/gen-styling.ts`).
4. Hub also failed once while another agent was mid-edit on `semio-s-artifact-stdio-semio`; passed on rerun.
5. `repo test @semio-tech/process-js` via MCP ran 45 min stuck in graph slice reading and was abandoned; bun was used directly.

## New tests (test-first: fixture and schema, Rust test, TS twin)
- Bridge: `answer()` extracted from `main` (the bin now only prints). Laws: the document inventory equals the shared fixture (16 mutation ids, schema/version/producer echoed, no `surface` key), every row is a production `Process3dMutation::DESCRIPTORS` entry with matching variant and outcomes and ids are unique, and unknown or malformed requests are refused with exit code 2. Oracle: `serde_json` parse plus the production descriptors read independently.
- Hub: registration law (the manifest serialised with `serde_json` matches pluginId, label, version, editor and viewer app ids and roles, capabilities, example count) and dispatch law (`has_original_app_factory` per app, and an unknown app is refused).
- TS twin: ajv 8.20 compiles both JSON schemas strictly and validates both fixtures; the hub registration fixture is checked against the app roles and ids the hub source registers. Wired into `typescript-package-commands` and `📋️project.json` `runtimeSourceEntries`.
- Finding, not fixed: the process editor and viewer register through `.editor` and `.viewer`, so `has_original_app_factory` is false for both (raster uses `.native_editor` with a native registration). The fixture records the current state (`originalFactory: false`). Migrating process to native registration lives in the editor crate (excluded, owned by another agent) and was not attempted.

## Browser preview and hub wasm build
Not rerun in this pass: the disk filled twice (fully, blocking all tools once) and other agents are mid-edit in the framework. Existing evidence is `browser-e2e-v5.md`: Process 3D renders one canvas (native route, hub v17, Firefox) with the Workpiece, Projection, Window Options and Actions panels. The extension components build for `wasm32-wasip2` (`🗑️generated/build-ext.log`), and `semio-hub-process-component` is the cdylib that carries the same `plugin()` that the new registration tests exercise natively.

## Environmental causes (no assertion or budget touched)
1. Bridge: rustc "Interrupted system call" on a nearly full disk; rerun passed.
2. `process-js`: the dependencies image `.cache/b/d/fixed.dmg` had a zeroed header; removed it and ran `repo dependencies ensure`.
3. Host and hub: stale gitignored `🔤️tokens/🦀️.rs` lacked `CHIP_TINT_PERCENT`; regenerated.
4. Hub rebuilds failed several times on other agents' in-flight edits (`semio-s-artifact-stdio-semio`, `semio-framework-value-native-owned`, `OriginalActorSuccession`) and on ENOSPC; passed on retry. `repo test` over MCP hung 45 min and was abandoned.

## Files changed
- Bridge: `🏭️bridge/🦀️.rs` (extract `answer`, mount tests), `🏭️bridge/Cargo.toml` (`serde_json` dev-dependency), `🏭️bridge/Cargo.lock`, new `🏭️bridge/🧪️tests/🔬️inventory/🦀️.rs`, `🧫️fixtures/inventory.json`, `inventory.schema.json`.
- Hub: `🌎️hub/🧩️compositions/🏭️process/🦀️.rs` (mount tests), new `🧪️tests/🔬️registration/🦀️.rs`, `🧪️tests/🧫️fixtures/registration.json`, `registration.schema.json`.
- JS: new `🧪️tests/🔬️bridge-hub-fixtures/🟦️.ts`, `🧪️tests/🔬️typescript-package-commands/🟦️.ts`, `📦️packages/🟦️typescript/📋️project.json`, `📦️packages/🟦️typescript/package.json` (`ajv` 8.20.0 devDependency; `bun.lock` not regenerated, run `repo dependencies lock --scope` for it).
- Ticket: `suites-cargo.sh`, `suites-root.sh`, `suites-hub-retry.sh`, `process-suites.md`.
- Regenerated ignored files: `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️tokens/🦀️.rs`; recreated `.cache/b/d` dependency image.
