# Fix Procedural 3D Default Example Dropdown

## Bug

Playground navbar showed **No example** while the viewport rendered the hexagonal mushroom column (the plugin's `default_fixture`).

## Cause

React shell booted `activeExampleId` as `""` when no lock/default env was set, then announced `setActiveExample("")` once. Procedural 3D's initial document is already the hex-column fixture (`default_fixture()`), so geometry and dropdown disagreed. WGPU already seeds the first registered example via `sync_session_chrome`.

## Fix

`resolveBootExampleId` + once-per-instance boot announce: keep a valid active/default id, otherwise seed the first registered example (hex column for procedural3d) and announce that id.
