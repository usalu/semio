# Open Node in S Opens App

## Symptom
Double-clicking / opening a media-graph node in s studio does nothing — stay stuck in studio instead of opening the app.

## Root cause
`openInstance` correctly emits `HostEffect::OpenPluginInstance`. The shell's `ensureSpawnedPlugin` updates `session.viewState.panelJson` with `activeSpawnedId` via a separate `SET_SESSION` dispatch. Immediately after, `applyHostEffects` commits its in-flight `nextViewState` (cloned from the pre-spawn `baseSession.viewState`) and **overwrites** that panel update, clearing `activeSpawnedId`. The window host only switches to the spawned app when `panel.activeSpawnedId` is set, so the UI stays on studio.

## Fix
Fold the spawned panel into `nextViewState` inside the `openPluginInstance` / `spawnPluginInstance` effect branches so the final session write keeps `activeSpawnedId`.
