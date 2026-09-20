# Astra Sol — WGPU Driver Editor

Date: 2026-09-20

## Outcome

The WGPU Settings → General panel now exposes React's complete seven-axis UI driver editor over the shared owned `OsUiDriver` preference document:

- `labels`: `full | icons`
- `labelTier`: `beginner | normal`
- `drag`: `handle | surface`
- `chrome`: `always | hover`
- `gumball`: `always | hover`
- `tooltips`: `full | minimal | none`
- `hotkeys`: `inline | tooltip | none`

The retained UI uses React's control ids, option values, option localization keys, save-name input, Save button, and custom-only Delete button. The editor section is closed initially (`defaultOpen: false`) like React and appends the localized `Modified` / `Geändert` marker while a draft exists. Custom drivers use their owned labels in the selector.

## Lifecycle and ownership

`ShellState::driver_document` resolves the same authority order as React's `uiDriverBase = uiDriverDraft ?? uiDriver`: session draft, selected custom document, selected builtin, then the default builtin. `setDriverField` seeds a complete document and changes only the named axis. It does not mutate `ChromePrefsState.custom_drivers`.

Every draft edit also rebuilds `ShellChromeBuildState.driver`, so all seven resolved axes change in the live renderer snapshot before Save. The WGPU `UiDriverChrome` contract now owns all seven typed axes and both builtins match `DEFAULT_UI_DRIVER` and `COMPACT_UI_DRIVER`.

Selecting a driver clears the draft before resolving live chrome. Save trims and React-slugifies the name, writes `custom.<slug>` with its canonical id, label and complete config, selects it, clears the draft, and clears the save-name field. Delete only accepts `custom.*`, removes the owned document, selects `default` when the deleted driver was active, clears the draft, and resolves default live chrome.

The production preference lane remains event-driven. The existing `UiPrefsSnapshot` comparison emits canonical `set_driver` and `set_custom_driver` mutations from the changed `driver_id` and `custom_drivers` map, then persists them through the existing bounded chrome-maintenance phase. No parallel storage format, adapter, CRUD path, runtime dependency, or script was added.

## Contract and tests

Added:

- `🧑‍🎨engine/🧬️schema/🚗️driver-editor/🔣️.json`: strict language-neutral schema for the seven axes, builtins, controls and lifecycle vectors.
- `🧑‍🎨engine/🧫️fixtures/🚗️driver-editor/🔣️.json`: shared default/compact documents and edit/select/save/delete vectors.
- `🧑‍🎨engine/🧪️tests/🚗️driver-editor/🟦️.ts`: Ajv schema validation, independent reducer oracle, and current React `UiDriver` / `ChromePanels` / `ShellHost` source oracle.
- `🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🚗️driver-editor/🦀️.rs`: retained UI, localization, live-draft, selection, save and delete laws, mounted from the WGPU Shell library tests.
- The driver oracle is included in the existing WGPU Vitest configuration.

Observed red evidence before repair:

1. Ajv strict mode rejected the first schema because `builtins.required` named fields that were not declared in `properties`. The schema now declares exact `default` and `compact` members and exact transition records.
2. The React source oracle initially expected literal per-axis ids even though React constructs them through `driverAxisSelectRow`. The oracle now validates that constructor call per axis and keeps literal checks for the fixed save/delete ids.

Final focused command:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker --skip-nx-cache -- '🧪️tests/🚗️driver-editor/🟦️.ts'
```

Result: **9 test files passed, 96 tests passed**.

`jq empty` passed for the schema and fixture. `git diff --check` passed. Rust/native/WASM execution was intentionally not launched from this packet because the root agent owns the active Cargo and renderer build queues. The Rust laws are present for that integrated run; no local Rust pass is claimed here.

## Coordinated window-law repair

During the root-owned native renderer compile, the earlier window packet's new law exposed three test-only compile seams. The law now imports `DockNode`, holds the fixture value before borrowing its MIME string, and handles `AdmittedSurfaceMap::try_insert` without imposing `Debug` on `World3dState`. Production window lifecycle code was not changed during this packet.
