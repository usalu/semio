# 🗂️ PDF descriptor: plugin-level action sets (2026-11-25)

Follows `🧱️pdf-descriptor-and-retirement.md` section A. The `stdio-pdf` descriptor was 4,496,151 JSON bytes against `CATALOG_DESCRIPTOR_MAX_BYTES` 4,194,304.

## Result

| Step | JSON bytes | Pack bytes |
| --- | ---: | ---: |
| before | 4,496,151 | 968,129 |
| reserved window actions moved to the app roster | 3,743,083 | 825,449 |
| plus plugin action sets (`pdf`, `pdf-editor`, `pdf17-editor`) | **1,281,668** | **299,702** |

The committed pair was regenerated (`settle-pdf-descriptor.ts` from the repo root, exit 0).

## What changed

### Framework schema (Rust, owned schema, TS)
- `ActionSet { id, appIds, actions }` and `PluginManifest.actionSets` (skipped when empty) in `🧾️plugin-manifest/🦀️.rs`. Shape chosen over `AppDefinition.actionSets`: membership lives on the set, so an app needs no new field and no `AppDefinition` literal changes.
- `PluginManifest::share_actions(id, member)`: rows every member declares identically move into one set. Refuses a repeated id, fewer than two members and no shared row.
- `PluginManifest::resolved_app` / `resolved`: the app's roster is the rows of every set naming it (table order) followed by its own rows. `validate_action_sets`: unique ids, members exist, no row twice, no member also declaring a set row, no two sets giving one app one action.
- Builder: `PluginBuilder::action_set(id, member_fn)`; sets form in declaration order after all apps registered. The runtime app definitions stay flat (the action registry is built from them); only the manifest is factored.
- Owned schema rows (`ActionSet`, `PluginManifest` v2) and the generated manifest TS were edited identically by hand (not regenerated with `repo run @semio-tech/framework:generate`).
- TS twin in the new manifest module `🧺️action-sets`: `resolveManifestApp`, `resolveManifestActionSets`, `actionSetsFault`; exported from `@semio-tech/framework-manifest` and `@semio-tech/framework-manifest/action-sets`.

### Where sets resolve
Design deviation from the brief: `window_kind_actions` / `resolveWindowActions` keep their signatures and now run on a RESOLVED app. Resolution happens once at each descriptor ingestion, because dozens of consumers read `app.actions` directly:
- TS: `fetchDescriptorDocument` (kernel descriptor admission) validates then resolves.
- Rust: trusted-catalog loader (after `validate_descriptor`, which now calls `validate_action_sets`), wasmtime node host `read_committed_descriptor`, renderer ProgramBridge install, MCP remote binding.
- Raw-JSON readers: descriptor-verification interactive-job audit and MCP catalog description census include set rows. `CATALOG_MANIFEST_FIELDS` accepts `actionSets`. The emitter refuses an invalid table.

### Reserved window actions
`try_build_definition` no longer copies interaction, reserved window, `cancelTypedOperation`, `hostEvent` and document-transfer actions into every window; they go once onto the app roster. `mount_natural_file_actions` (`saveArtifactFile`/`openArtifactFile`) does the same. Behaviour: window resolution is unchanged because roster rows are offered to every window that does not claim the id. The MCP catalog now lists these as app-wide (`*`) verbs.

### PDF plugin
`stdio-pdf` `plugin()` declares `pdf` (all 20 apps: 8 framework rows), `pdf-editor` (10 editors: 27 rows) and `pdf17-editor` (7 editors: 68 rows). Viewer sets were tried and refused by the framework (viewers share nothing beyond `pdf`), so they are not declared. The three shard `plugin()` functions declare none; shards stay flat.

## Tests
- Language-agnostic fixture `🧪️tests/🧫️fixtures/🧫️action-sets/🔣️.json` (10 resolution cases, 7 sharing cases):
  - Rust `action_sets_tests` (2 tests): 2/2 pass (`ms1.log`).
  - TS `bun test ./🧪️tests/🔬️action-sets/🟦️.ts` (Ajv-validated cases): 10/10 pass.
- `committed_descriptor`: size check passes (1,281,668 and 299,702 bytes); the first post-change run reported only staleness against the old committed pair, which was then settled. Final rerun (`as91.log`, on the tree with the PDF set declarations): `the_committed_descriptor_pair_is_the_pair_the_assembly_describes` 1/1 passes, exit 0. The described pair at that run was 1,300,331 JSON and 304,518 pack bytes (other sessions changed the PDF tree between runs; the committed pair equals it). One intermediate run (`as81`) failed because another session overwrote the PDF `plugin()` and dropped the three `action_set` declarations; they were re-added.
- Stage (coordinator blocker): see below.

## Stage blocker (`stage-pdf-component.ts`)
Cause: the stage compared each shard's own `describe()` (flat rosters, because shards declare no sets) with the plugin descriptor (factored). The descriptor was not stale. Fix: the stage resolves both sides with `resolveManifestActionSets` (the host loader's resolver) and orders each roster by action id before comparing. Rerun with the already built wasm (no rebuild): `r6w-stage.log`, exit 0; 3 components staged, descriptor 1,280,400 JSON bytes (final form) and 299,702 pack bytes.

## Other plugins
Every plugin's emitted descriptor changes (reserved actions move from each window to the app roster), so their committed `🔣️.json`/`🛂️.descriptor.semio` pairs are stale until `repo run workspace:components -- describe --all`. That needs each plugin's wasm build, which was not run. Only `stdio-pdf` has a native pair-equality test; `bim` and `note` committed-descriptor tests only load the old pair and still pass against it. Plugins declare no action sets yet; the mechanism is available to any plugin with several apps sharing rows.

## Not done / follow-ups
- Other plugins' descriptors not regenerated (wasm needed); wider workspace tests that assert reserved actions inside `window.actions` were not run.
- `repo run @semio-tech/framework:generate` not run (generated TS edited by hand to match the rows).

## Files changed
Framework: `🛂️manifest/🧾️plugin-manifest/🦀️.rs` and its `Cargo.toml`, `🛂️manifest/🧱️app-surface/🧬️app-definition/🦀️.rs`, `…/🧭️layout-and-action-resolution/🦀️.rs`, `🛂️manifest/🧺️action-sets/🟦️.ts`, `🛂️manifest/🟦️.ts`, `🛂️manifest/package.json`, generated manifest TS, schema rows `🧾️rows-a-h` and `🎟️rows-h-t`, `📦️packages/🦀️rust/🦀️.rs` (re-export), 16 test/fixture files that construct `PluginManifest` (new field), builder `🦀️.rs`, `🔌️program/🦀️.rs`, `🏗️try-build-definition/🦀️.rs`, `📜️declaration/…/natural-file-and-media-artifacts/🦀️.rs`, descriptor emission, kernel descriptor admission, catalog verification, descriptor verification, MCP catalog TS, trusted-catalog validation and loader, node host, renderer bridge, MCP remote binding. Hub: `stdio-pdf` `🦀️.rs`, committed descriptor pair. Tests: `🔬️action-sets` (Rust and TS) and its fixture. Ticket: `run-manifest-action-sets.sh`, `stage-pdf-component.ts`, this file.
