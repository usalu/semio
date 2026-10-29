# 1029 Result: rta (native runtime tests: generator, koordinator, aggregator, aussuchen)

STATUS: RED, NO NATIVE TEST HAS RUN YET (0 pass / 0 fail). Every native workspace `test` build dies at compile, so there is no runtime verdict for any of the four panes. Eight compile/gate blockers were found and fixed (list below); the remaining frontier is owned by peers' in-flight migrations (`reference_birth` -> `ReferenceArtifactAppBirth`, `OriginalStoreGenesisOwner` bounds in `semio-framework-plugin-ownership`, stdio-semio DSL-native). Last verification checks (`rta-cadchk2`, `rta-curchk5`, `--workspace --keep-going`, lib targets only) were still queued when this file was written. Tools: `C/1029-tools/rta/{run,seq,seq2,chk}.sh`, offline test-target gate probe `C/1029-tools/rta/testgate.ts <workspace dir>`; logs `C/🗑️generated/1029/rta-*.{txt,rc}`.

## Per pane
| pane | workspaces | tests run | blocker at last attempt | fixes by me |
|---|---|---|---|---|
| generator | procedural generation3d (93 members, 686 tests) | none | `semio-framework-plugin-testing-laws` (5 errors: peer migrating `reference_birth()` to `ReferenceArtifactAppBirth`, files `viewer-app/editor-app` touched 09:47) | testing-laws `dispatch_typed` (item 1), acceptance harness bounds (item 4) |
| koordinator | cad (41 members, 122 tests) + 4 extension workspaces | TS: 4 ext configs pass; native none | lib compile errors in cad-play/transient/picking/presence/host (items 6-7), then test-only errors (see CAD below) | items 6, 7 |
| aggregator | puzzle 3d (39 members, 825 tests) | TS: puzzle config 13 files/48 tests pass; native none | reached `semio-s-artifact-puzzle-3d` lib test (fixed, item 2c) then queue/peer churn | items 2, 2c, 3 |
| aussuchen | sourcing curation (3 members, 33 tests) + beams/slabs/windows | none | `semio-framework-plugin-ownership` 370 errors (peer, upstream of everything) after my sourcing fixes | item 8 |

## Fixes (all root-cause, no shims; each re-read before edit)
1. testing-laws: `assert_two_registered_instances_converge_with_members` called the deleted `VcsArtifactApp::dispatch_typed`; now `dispatch_original_fixture_command` (like its registry-less twin). File `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/👯️paired-apps-history-and-convergence-laws/🦀️.rs`. SHARED.
2. puzzle 3d editor test target `editor-surface` (497 KB, 7.8k lines) was refused by the pre-compiler source gate for the WHOLE `🧊️3d` workspace; 5 of its 16 law files were stale duplicates (old `crate::editor::puzzle3d`/`semio_framework_job::WORKER_JOB_SESSION_SLOTS` paths) of the `editor-panels` laws. Deleted the 5 duplicate dirs, moved the imports into `🧪️tests/🧰️prelude/🦀️.rs`, split the remaining 11 laws into `editor-surface` (`🔬️unit`), `editor-defects`, `editor-manifest` (+2 `[[test]]` in the editor `Cargo.toml`). Offline gate probe over 53 test targets: 0 over.
2c. puzzle 3d root crate lib test `🧪️tests/🔬️design-parity-schema`: `use crate::*` no longer exported the model types; explicit `semio_s_artifact_puzzle_3d_model::{Puzzle3dAttribute,Puzzle3dAuthor,Puzzle3dCatalogVortexKind,Puzzle3dCatalogVortexTemplate,Puzzle3dObjectAnchor,Puzzle3dRepresentation}` imports (stale after the model carve).
3. puzzle 3d precompute `collision-engine`: `brush_collision_free` was `#[cfg(test)]` but called by `compute_brush_cache_entry` (`cfg(any(test, feature="testkit"))`): real lib compile error whenever `testkit` is on; now `cfg(any(test, feature = "testkit"))`.
4. `semio-framework-plugin-testing-acceptance` (dev-dep of every artifact test crate): `Snapshot: ArtifactDsl` left `ArtifactApp` in 706; 20 generic harness fns got `where <A as ArtifactApp>::Snapshot: pack::snapshot::ArtifactDsl` (puzzlefin's convention for `load_document_text`), `acceptance_verb` passes `&mut Some(meta)`. Files `🧰️framework/.../🔌️plugin/🧪️tests/🧪️history-edit-acceptance/*/🦀️.rs`. SHARED.
5. cad `cad-model-diff`: `diff_text!` expands to `semio_framework_diagnostic`; dep added to its Cargo.toml. (The same defect exists in `📕️norm/*` diff crates and `🧱️block/{2d,3d,5d}` per a scan; not mine.)
6. sourcing curation carve (this is the aussuchen component: `semio-hub-sourcing` -> `semio-s-artifact-sourcing-curation`; these errors are in NON-test lib targets, so the wasm component cannot build either):
   - `curation-examples`: dropped the unresolvable `pub use semio_s_artifact_sourcing_curation_tests::editor::sourcing::*` (a dependency cycle; the mounted files only need `crate::{Filters,TableSort,SortDirection}`).
   - `curation-domain`: `🚪️io/📝️text/📸️snapshot` imports `SourcingModule` + `SourcingModules`; `sourcing_json_envelope_is_bounded` `pub(crate)` -> `pub` (used by `curation-inferences`).
   - `🧬️schema/🔺️diff`: `list_delta!` with `key: object_id` emitted an orphan `impl Keyed for CuratedItem` (row lives in `examples`); now `list: Vec<CuratedItem>, key: String = by CurationCuratedKeys, values_only` with a local `KeyOf` marker (pattern of lowpoly/process3d diff).
   - presence: `[f64;3]` has no `NativeDefault`, so `#[value(required)]` on `world_camera_position/target` (same decision as rtb's process3d presence).
   - editor store preparation (config + artifact lane): ported to the 706 one-item API: `advance(&mut self, _body, grant, _receiving: &mut semio_framework_dsl::NativeDecodeControl<'_>)` and factory `begin_copy_byte_demand` (puzzle 3d template). Config mutation: `DslReceiving` on the enum, its 6 edit records and `TableSort`, plus `original_ordinal_operation_receiving!(SourcingCurationConfigMutation)`. Tests-crate root: `extern crate semio_framework_os_kernel as dsl;` (needed by `app_commands!`). THESE LAST EDITS ARE NOT YET COMPILED (upstream blocked); expect follow-up errors in `semio-s-artifact-sourcing-curation-tests`.
7. cad koordinator libs (NON-test, wasm path too):
   - `cad-play`: `geometry-import` used `semio_mesh_snapshot_from_mesh_data`/`mesh_extent` which were `pub(crate)` in the model crate -> `pub`.
   - `cad-transient`: `transient_root!` needs `pack` and `semio_framework_os_kernel` at crate root -> `extern crate` + `semio-framework-os-kernel`/`semio-framework-pack` as normal deps (pack moved from dev-deps).
   - `cad-picking`: `GeometryOrder` private but reachable via `GeometryBuckets.order` -> `pub(crate)`.
   - `cad-presence`: duplicate `RetireOwned` (derive + `artifact_retire_struct!`) -> macro line removed, schema leaf gets the derive; `[f64;3]` NativeDefault -> `#[value(required)]` on camera position/target.
   - `cad-host`: `CadStore::new` needs the publication type: `::<ArtifactSnapshotPublication>` (layout/puzzle2d spelling).
   Remaining known CAD test-only errors (not fixed): `cad-config` lib test (`CadConfigMutation::diff/inverse` need `ColdMutation as _`), `cad-inferences` lib test (`scene-compute` imports `crate::CadWorkingScene`), `cad-picking`/`cad-presence`/`cad-transient`/`cad-host` lib tests share the lib errors above.
8. Disk: the shared cargo volume (`.🧬semio/🦑️repo/⚡️cache/rust/fixed/m`, 48 GiB) hit 0 bytes free four times; each time every rustc died with `No space left on device` / `Invalid argument` (disguised as random compile errors). Mitigation used: one workspace at a time, `CARGO_INCREMENTAL=0`, retry on ENOSPC. COORDINATOR: the fleet needs the new `compiler image headroom` queue (now present) or a larger image; a run that waits 10-30 min for headroom is normal.

## Open (not mine, blocks native tests and, for 1-3, also the wasm check)
- `semio-framework-plugin-testing-laws`: 5 errors, `ArtifactViewer/Editor::reference_birth` now returns `Option<ReferenceArtifactAppBirth<Snapshot>>` (peer edit 09:47) but `artifact-app-laws/bounded-viewer-fixture-and-viewer-laws` + `fixture-dispatch-steps-and-document-loading` + `authoring-law` still use `fn()->Snapshot`. Owner: whoever migrates `reference_birth`.
- `semio-framework-plugin-ownership`: 370 errors at 10:50 (`OriginalStoreGenesisOwner::{State,Mutation}` bounds); peer in-flight.
- `semio-s-artifact-stdio-semio-object-snapshot` (semiodsl in-flight, `ArtifactChild<..>: NativeReceive<dyn NativeValueSource>`).
- stdio-binary `reference_birth` test impls (peer sweep).

## TypeScript (vitest) results
| config | files | tests | result |
|---|---|---|---|
| puzzle (`🧩️puzzle/🧪️tests/🎚️config`) | 13 | 48 | pass |
| cad ext aec-building / spatial-shape / aec-building-structure / aec-building-energy | 1 each | 1 / 2 / 2 / 1 | pass |
| cad dev (`🧑‍💻dev/📐️cad/🧪️tests/🎚️config`) | 13 | 344 | 332 pass, 12 FAIL (below) |
No vitest config exists for procedural (generation3d) or sourcing (their TS test dirs are `rust-package-commands`/`typescript-package-commands` wrappers only).

cad dev failures (none is on the demonstrator path):
- 9 x `geometry.current-browser-binding-required` / `SemioGeometryCloseReceipt ... phase`: ENVIRONMENT. `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/📦️packages/🦀️rust/🕸️bindings/semio_session*` is a gitignored generated wasm-bindgen output from 10-06 (before `begin_close/close_step/...` landed). Regenerate with Nx target `@semio-tech/s-spatial-kernel-semio-session-rs:wasm`; it is not in the demonstrator activation closure (the cad plugin evaluates geometry inside its own wasm component).
- 3 x `rules of hooks` (`PreviewItem`, `SpatialPickTargetNode`, `TessellatedCommitMesh` in the cad-js renderer `🧪️tests/🪝️hookorder`): hook-call count of the mount render differs from the rerenders (2 vs 1 distinct counts). Not triaged further: the demonstrator package does not reference `@semio-tech/cad-js` (grep over its sources: no hit), so the pane does not mount this renderer directly.

## Awaiting dispatch (generator host events)
Nothing observed natively yet. generation3d's `ArtifactEditor::host_event` (`✏️editor/🪟️artifact-editor-impl/🦀️.rs:485`) is the only host-event producer in the workspace and no test of it exists; the dispatch agent's note (host events refuse without a wire producer) applies unchanged. No native test failure could be attributed to it because none ran.

## Next step (exact)
After the peers' `reference_birth` + ownership migrations compile: `cd /Users/ueli/Documents/semio && C1029-tools/rta/seq2.sh v 'pz3d|…/🧩️puzzle/🗿️artifacts/🧊️3d' 'gen3d|…/🌀️procedural/🗿️artifacts/🧊️generation3d' 'cad|…/📐️cad/🗿️artifacts/📐️cad' 'cur|…/🪵️sourcing/🗿️artifacts/🗂️curation'` (detached, one workspace at a time; finals in `rta-<n>-final.{txt,rc}`), then triage failures per pane.
