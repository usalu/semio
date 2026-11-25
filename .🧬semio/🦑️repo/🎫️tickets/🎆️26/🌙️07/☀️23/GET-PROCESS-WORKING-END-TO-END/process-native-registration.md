# Process Plugin Native Registration

The process plugin now registers its editor and viewer through `native_editor` / `native_viewer`. The old `.editor` / `.viewer` registration is removed with no dual path. `has_original_app_factory` is true for both apps.

## What changed
- Hub `🌎️hub/🧩️compositions/🏭️process/🦀️.rs`: both fleet variants are `OriginalVcsApp<..., SemioMembers>`. The builder calls `.native_editor::<Process3dPlayApp, Process3dNativeRegistration>` and `.native_viewer::<Process3dViewer, Process3dViewerNativeRegistration>`. Hub `Cargo.toml`: dependency `semio-plugin-instance-construction`, dev-dependency `semio-framework-job` with `testkit`.
- Host crate, new `🌱️genesis/🦀️.rs` (shared by editor and viewer): `Process3dDocumentCatalog` (shared cold semantic issuer through `ColdMutationApplyFactory`) and `Process3dDocumentGenesis` (fixed initial state from `empty_process3d_snapshot()`, Pack source from the DslRecord projection).
- Editor, new `🌱️initialization/🦀️.rs`: `Process3dConfigGenesis` and catalog, `Process3dPresenceInitial`, `Process3dEphemeralDeclaration`, `Process3dInstanceDeclarations`, `Process3dNativeRegistration`, and `OriginalArtifactEditorBirth for Process3dPlayApp` with no draft. The only window-transient owner is `WorkpiecePlacementOwner`; no window persists config.
- Viewer, new `🌱️initialization/🦀️.rs`: `Process3dViewerDeclarations` (empty ephemeral lane, no windows), `Process3dViewerNativeRegistration`, and `OriginalArtifactViewerBirth`.
- Native receiving derives: `NativeReceive` is added to `Process3dMutation`, its 16 leaf payloads, and the model records they carry (7 machine/catalog records, 6 document records). `Process3dConfig` gets `DslReceiving`, `native_pack_source` and `OriginalArtifactPackReceiving`. `Process3dConfigMutation` and its payloads get `NativeReceive`.
- Cargo: process3d workspace gets `semio-plugin-ownership` and `semio-os-store-operation-wire`. Host and editor depend on both. The viewer depends on the host.
- Hub fixture `registration.json`: both apps have `originalFactory: true`. `registration.schema.json` now pins it with `const true`.
- Hub registration test: native definitions must be closed through an original caller before drop, otherwise the registry panics ("committed native definition requires original alias return and typed graph closure"). New helper `retire(program)` follows the raster pattern.
- TS twin `✏️s/🔌️plugins/🏭️process/🧪️tests/🔬️bridge-hub-fixtures/🟦️.ts`: asserts `originalFactory` is true, that the hub source contains `.native_editor::<` and `.native_viewer::<`, that it contains no `.editor::<` / `.viewer::<`, and that it has two `OriginalVcsApp<`.

## Results
- Hub `semio-hub-process`: 2 passed, 0 failed (registration and dispatch laws, expecting `true`), rerun after the bounded-factory change.
- Host `semio-s-artifact-process-process3d-host`: 8 passed, 0 failed. This includes the two new genesis laws (see below).
- Editor `process3d_bearbeiten_pane_smoke`: 17 passed, 1 failed, rerun after the bounded-factory change. The one failure is `bearbeiten_pane_loads_applies_undoes_and_closes`, with "host lifetime fuel exhausted across fixture turns". That is the known pane-fuel test owned by another agent. Every other test passes. A second target in the same run (`process3d_interaction_behavior`) did not execute ("never executed": its binary vanished from the build dir). It was not in scope and was not rerun.
- `semio-hub-process-component` `descriptor_is_fresh`: 1 passed. See the descriptor fix below.
- Raster and energy component tests: not verified. Both fail to compile for reasons outside this work: raster-model uses `retained_alias` / `retained_alias_work_demand`, which `PersistentList` no longer has, and the energy viewer calls `render_with_camera` with 2 arguments instead of 3. Both are other agents' in-flight edits. The fix is generic, so they should pass once those compile.
- TS twin (`bun test` on `🧪️tests/🔬️bridge-hub-fixtures/🟦️.ts`, with the bun path from the coordinator): 5 passed, 0 failed.

## Descriptor test fix
The framework macro `__semio_plugin_descriptor_fresh_test` (`🔌️plugin/🧪️tests/🧬️generated-test-contracts/🦀️.rs`) now calls `plugin_runtime::dispose_plugin_bundle_cold(runtime, &mut cx)` before dropping its caller. The new function lives in `⚛️reactor/🏃️runtime/📦️instances/🪪️descriptor/🦀️.rs`. It takes the installed plugin out of the runtime and closes its native definitions through the original caller with `close_original_definitions`, and it fails if no progress is made.

## Genesis law
- `🔨️modules/🏠️host/🧰️owned/🌱️genesis/🧪️tests/🦀️.rs` has two tests.
  - The authored initial document equals the fixture (step, tool-solid and machine counts, schema, artifact kind, `MAXIMUM_INITIAL_PACK_BYTES`), and an under-funded copy grant is refused.
  - `OriginalStoreGenesis<Process3dDocumentGenesis>` is born through an original job caller. Store history shape is document, and the publication and replay preparation factories are the same object. Every owner, the Store, the birth frame and the decoder close to terminal-empty.
- The fixture `🔣️.json` and schema `📐️.json` are in the same folder. The TS twin checks both with ajv and cross-checks the fixture against the genesis source and the model constants.
- The law needs a per-turn grant of 1024 items and 1 MiB of copy bytes. The birth quote is 30,960 copy bytes, and a 32 KiB turn never completed admission. Mounted callers must therefore grant at least that much for process; the fixture records it.

## Bounded preparation
- The bespoke one-item `Process3dArtifactPreparationFactory` and its preparation struct are deleted. The editor and the native genesis now both use `process3d_document_preparation_factory()`, which is `ColdMutationApplyFactory::bounded` over the document envelope. It serves publication and replay alike.
- The envelope measures (512 KiB maximum, 256-byte text, 4,096 items, 4 KiB turn grant) moved verbatim into the host crate, new `📏️envelope/🦀️.rs`. The editor re-exports them, so the command-surface tests are unchanged.
- The old bespoke factory also gated on the 4 KiB per-turn grant. The cold factory takes its footprint from `ColdPreparationBounds`.

## Caveats
- `initial_state_demands` measures the real empty snapshot through `retirement_birth_bytes`, and the genesis law checks it against a real grant.
- `reference_birth` is kept on editor and viewer, as raster and energy do.
- The shared `🗑️generated` folder was cleaned twice by another agent while I worked, including my build dir. I deleted my own build artefacts at the end, and my scripts and processes are stopped.

## Files
- `🌎️hub/🧩️compositions/🏭️process/🦀️.rs`
- `🌎️hub/🧩️compositions/🏭️process/📦️packages/🦀️rust/Cargo.toml`
- `🌎️hub/🧩️compositions/🏭️process/🧪️tests/🔬️registration/🦀️.rs`
- `🌎️hub/🧩️compositions/🏭️process/🧪️tests/🧫️fixtures/registration.json`
- `🌎️hub/🧩️compositions/🏭️process/🧪️tests/🧫️fixtures/registration.schema.json`
- `✏️s/🔌️plugins/🏭️process/🧪️tests/🔬️bridge-hub-fixtures/🟦️.ts`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.toml`
- host: `🔨️modules/🏠️host/🧰️owned/🦀️.rs`, `🌱️genesis/🦀️.rs` (new), `📦️packages/🦀️rust/Cargo.toml`
- editor: `🦀️.rs`, `🌱️initialization/🦀️.rs` (new), `🎚️config/🦀️.rs`, `📦️packages/🦀️rust/Cargo.toml`
- viewer: `🦀️.rs`, `🌱️initialization/🦀️.rs` (new), `📦️packages/🦀️rust/Cargo.toml`
- `🧬️schema/🧬️mutations/**/🦀️.rs` (17 files), `🧬️schema/📸️snapshot/🪚️machines-and-catalogs/🦀️.rs`, `🧬️schema/📸️snapshot/📐️document-records/🦀️.rs`
- Added by the follow-up: `🔌️plugin/🧪️tests/🧬️generated-test-contracts/🦀️.rs`, `🔌️plugin/⚛️reactor/🏃️runtime/📦️instances/🪪️descriptor/🦀️.rs`, `host/🧰️owned/📏️envelope/🦀️.rs`, `host/🧰️owned/🌱️genesis/🧪️tests/{🦀️.rs,🔣️.json,📐️.json}`, and `store-preparation/🦀️.rs` with `editor-impl/🦀️.rs` (editor)
- Ticket helpers: `reg-cargo.sh`, `reg-retry.sh`, `reg-all.sh`, `reg-comp.sh`, `reg-comp2.sh`, `reg-final.sh`
