# Verify Log

## Automated

- `bun nx run @semio-tech/framework-renderer-react:test` — 21/21 passed

## Spawned window chrome

- `refreshSpawnedUi` fetches `render`, `tools`, `windowEngagements`, `windowMeasures` for studio spawned apps
- Spawned `modeWindows` descriptor wires `engagement` and `measures` via `spawnedWindowChromeForKind`
- Footer toolbar uses `spawnedToolNodes` when `panel.activeSpawnedId` is set
- `processPluginOps` routes spawned-plugin sessions to `refreshSpawnedUi` instead of overwriting s-shell `windowUiByKind`

## Toolbar ribbon

- `ToolTree` restored premigration ribbon: `sortToolNodes`, `buildToolbarRibbonSegments`, collection picker `ToggleGroup`, batched `ButtonGroup`/`ToggleGroup` runs
