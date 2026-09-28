# Playbook Files VFS Producer

## Scope

Playbook Builder now declares a sixth app-owned window, `playbook-files`, with body and surface id `playbook.play.files`. The mode's default tab stack, editor render router and app manifest all include it as a `virtual-file-system` surface.

## Neutral Contract

The existing `scene-showcase` fixture and schema now cover all six windows and carry an exact Files projection. It names three domain-neutral file kinds, an invisible root row, two step rows and one nested block row. The row schema is closed, so `navigateUri` and any undeclared action field are structurally forbidden. Expected visible ids and levels cover both the initially expanded tree and the collapsed first step.

The producer maps the current snapshot deterministically:

- root id `playbook`;
- step id `step/<step-id>`;
- block id `block/<step-id>/<block-id>`.

Rows keep source order, contain no navigation URI, and publish no selection, hover or action. Drag/drop is explicitly disabled. The file-kind names resolve through the app's native/reuse English and German terminology pack.

## Actual React Oracle

The registered showcase test validates the fixture with Ajv 2020, flattens the raw rows with the production `buildVirtualFileSystemSceneRows`, and mounts the actual `VirtualFileSystemHost`. It checks the initial ids and levels, focuses the real step disclosure button, collapses with Enter, reopens with Space, and requires zero dispatched domain actions. The disclosure state stays renderer-local exactly as the host contract intends.

Focused command:

`NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long bun nx run @semio-tech/framework-renderer-react:test --excludeTaskDependencies -- '../../../../🧪️tests/🎬️playbook-scene-showcase/🟦️.ts' --silent=false --reporter=verbose`

Result: **1 file, 2/2 passed**, Vitest 32.74 s, Nx 39.5 s.

## App-backed Rust Law and Limits

The Files window unit constructs an actual composed `PlaybookSnapshot`, calls the production scene producer, and compares its decoded schema and rows to the same neutral fixture. It also requires absent selection, disabled drag/drop and no `navigateUri`. The Rust law is authored but has not yet run; no app-build or native passing result is claimed. Physical WGPU Files rendering, disclosure and accessibility remain part of the root-owned next runtime acceptance.

The physical route is the artifact `s.playbook.playbook@1/*`, app/editor `playbook-play`, bundled example `demo`, mode `builder`, window `playbook-files`. From a fresh host, activate that artifact and example, open the Files tab, and verify the raw visible hierarchy matches the current Demo snapshot: step rows in document order and each block directly under its owning step. Focus a step's disclosure through accessibility, collapse it, confirm only that step's blocks disappear, reopen it, and confirm the same stable ids and levels return. No selection receipt, navigation action or document mutation should be emitted by those disclosure interactions.
