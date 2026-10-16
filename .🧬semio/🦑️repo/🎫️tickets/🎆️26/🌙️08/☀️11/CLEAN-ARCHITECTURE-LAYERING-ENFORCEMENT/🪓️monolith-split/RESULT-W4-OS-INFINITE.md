# Result W4: `semio-framework-os-infinite` — WORLD and BOARD halves carved into producer crates

Executor: Sonnet 5.5, slug OS-INFINITE. `INF/` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/`. Tools: `🪓️monolith-split/oinf-tools/` (pipeline: `oinf_rs.py` lexer, `wmap.py` field-to-component map, `wengine.py`/`wrewrite.py`/`wtests.py` decomposition engine, `lane.py`/`units.py`/`settle.py`/`wplan.py` layering, `wgen.py`/`wtgen.py`/`wapi.py`/`wman.py`/`wreg.py`/`wapply.py` generators and switch-over, `woracle.py`/`wsb.py` stitched differential, `warity.py`, `wdefscan.py`, `wcoherence.py`, `wpaths.py`, `wcensus.ts` checks). Logs in `🗑️generated/oinf-*.log`.

## Status

Decisions followed: (1) no extension traits; `World3dState` was decomposed into component structs, interaction methods take explicit component references. (2) World half carved in a staging tree, verified, then switched over in one scripted pass (live world tree was checked unchanged against the snapshot immediately before the copy). (3) Board half carved after the relocation session finished (see "Board carve (executed)" at the end). (4) Terrain: `semio-framework-surface-terrain` (SURFACE agent) is a normal downward dependency of the components crate; no mount.

**No real `cargo check`/`cargo test` has been run.** `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-infinite-world-model` ends with `Finite original maintenance authority required` (`🗑️generated/oinf-final-check.log`), and root `Cargo.lock` is not yet refreshed for the seven new members (`name = "semio-framework-os-infinite-world-*"` count in `Cargo.lock`: 0). Run `bun repo cargo ... update --workspace --offline` first once the launcher works. Typeck (borrowck, trait methods, field privacy) is therefore not machine-verified; see Verification for exactly what is.

## Architecture

### 1. The god object is gone: `World3dState` is a thin owner of 13 components

152 fields moved (verbatim, with docs and `cfg(test)` markers) into component structs (all fields `pub`, each with `new`):

| Component | Owns |
|---|---|
| `WorldSessionState` | surface/controller id, bounds, pick bounds, domain binding, locale, declared actions, active utility, presentation, interaction revision, last snapshot fault |
| `WorldCameraState` | orbit rig, projection spec and frame policy, camera sync, camera fit request/owner/cursor, camera digest, lod pick, gizmo hover |
| `WorldSelectionState` | hover, selection, hovered component, marquee, paint stroke, pick targets/hover key, selection targets, highlight/provisional/disabled/celebrating ids |
| `WorldGumballState` | gumball selection/config/target/preview, transform mode, brush and catalogue-drop preview, relocate drag |
| `WorldInteractionState` | interaction mesh and object registries, instance interaction ids |
| `WorldAuthorityState` | the interaction authority state machine (holds its active cursor or job) |
| `WorldRetirementState` | the in-flight dynamic retirement cursor |
| `WorldSceneState` | meshes, appearances, textures, draws and rebuild, scene records, placeholder and dynamic-retirement slots |
| `WorldTerrainState` | terrain style, tiles, `TerrainSessionCore`, build and pending |
| `WorldOverlayState` | face overlay build/colors/generations, sub-element paint, scalar field, prepared status |
| `WorldBridgeState` | scene bridge, snapshot lease/apply, digests |
| `WorldAssetsState` | asset io authority, url bookkeeping, brush mesh announce |
| `WorldTraceState` | tool run trace layer and visibility |

The `World3dState` ↔ interaction cycle (cursors took `&mut World3dState` while the state owns the authority that owns the cursors) is broken the requested way: authority, retirement and the other held machines are separate components, and every cursor/job method and helper function takes the exact components it uses. Engine facts (`oinf-tools/w7.py`..`w10.py`): 280 state-taking functions found; 258 narrowed to explicit component parameters (width distribution before narrowing: 1 component 118, 2: 46, 3: 52, 4: 31, 5: 18); 11 free orchestrators (`render_world_3d`, `sync_world3d_state`, `step_world3d_scene_bridge`, `step_world3d_interaction`, ... each needs 6 to 11 components) and the 8 `World3dState` methods plus `Drop` stay on the host and are rewritten to `state.<component>.<field>`; 2,524 field uses and 909 call sites were rewritten mechanically (the counts include the test files, which were rewritten the same way (host variables found by local type inference: `state`, `wired`, `typed`, `bound`, `journey_state()` results, ...)). Methods live with their data: each cursor type sits in the crate of the component it operates on.

### 2. Crate graph (downward only; cycle-free by construction and checked)

`semio-framework-os-infinite-world-<id>`, lib roots `INF/🌍️world/<folder>/🦀️.rs`, manifests `INF/🌍️world/<folder>/📦️packages/🦀️rust/`:

| id (folder) | content | depends on |
|---|---|---|
| `model` (`🗃️model`) | records, consts, slot/pool registries, interaction ids, tool run trace (`⏯️tool-run-trace` moved inside) | external only |
| `components` (`🧩️components`) | the component structs, the types they own, `WorldMeshSurface` | model, `surface-terrain` |
| `cursors` (`🧲️cursors`) | ray/object/component picking, marquee, gumball cursors and jobs | model, components |
| `scene` (`🧱️scene`) | pools, chunking, LOD, environment, terrain meshing, placeholders, asset io | model, components |
| `bridge` (`🌉️bridge`) | dynamic retirement, face overlays, snapshot apply, guest scene bridge | model, components, cursors, scene |
| `interaction` (`🧵️interaction`) | pointer/wheel/gesture entry points, interaction authority | + bridge |
| `host` (`🎛️host`) | `World3dState`, frame sync, `render_world_3d`, orchestrators, `testkit` | all |

`semio-framework-os-infinite` stays as the facade: `INF/🌍️world/🦀️.rs` is explicit `pub use` lines only (every item name that code outside infinite imports from `world`, each exported from the crate that defines it, no glob and no wrapper module; the 112 scanned names also include enum variants), the lib root still mounts it with the wasi-exclusion cfg; its manifest depends on the seven crates under the non-p2 target table. Every world crate carries `#![cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]` (cfg gating kept exact; browser wasm lane compiled in the oracle).

External callers address the components directly: there is no whole-state wrapper layer. The 52 narrowed functions that code outside infinite imports are exported by the facade from the crate that defines them (`components`, `scene`, `bridge`, `interaction`, `host`), and each outside call site borrows exactly the components the function declares (see "Wrapper removal" below). Field-path consumer edits: `app-runtime` (`state.session.controller_id`), `scene-render-entry` (`state.trace.tool_run_trace_window_id`, 2 sites), `wgpu-engine-surfaces` test (`state.camera.orbit`, 4 sites), `wgpu-wheel-and-escape-routing` and `window-lifecycle-template-drag` tests (`state.session.bounds`), and two path strings in `📇️surface-verbs` docs/fixture.

### 3. Tests split along the same lines

The 524 KB unit file plus five suites (9,015 lines) became 17 integration test targets in the owning crates (`[[test]]` with `path = ../../🧪️tests/<dir>/🦀️.rs`, e.g. `host`: `unit`, `unit-2`, `unit-3`, `journal-sequences`, `pointer-gestures`, `surface-verbs`, `bridge-empty-build`, `scene-input-residency`; `model`: `unit`; `scene`: `unit`; ...). Each test went to the highest crate it depends on; helpers shared between targets are duplicated (about 1.1k lines in total; `with_world_step_context` and the test `Drop` for `World3dState` also live in `host/🧰️testkit`). `cfg(test)` in library code became `cfg(any(test, feature = "testkit"))` (and `not(...)` accordingly) so integration targets see the test-only fields/fns/Drop; every crate declares `testkit` (forwarded to lower crates) and a self dev-dependency enabling it. `include_str!` of 70 fixtures and ~50 peer source files turned into run-time reads relative to the manifest (`read_static`/`read_joined`, 129 reads; zero embedded bytes in test closures). `WORLD_SOURCE` source-guard tests now join the seven crate roots. The external-style `♻️resources` test stays with the facade unchanged. One dead helper (`draw_fixture_instance`, unreachable from any test) was dropped.

## Census (canonical `readRustSourceClosureV1`, host cfg; gate 4000 lines / 32 files / 262144 bytes; target 3500 / 28 / 230000)

Before: 68 files / 2,230,763 B / 42,238 lines for the whole crate; the world lane alone was 15 files (`🌍️world/*`), `World3dState` 152 fields. After (`🗑️generated/oinf-census-after.log`; files / bytes / active lines):

| Crate | lib | wasm lib | lib+testkit |
|---|---|---|---|
| model | 2 / 149,793 / 3,257 | same | 2 / 149,793 / 3,331 |
| components | 1 / 111,485 / 1,990 | same | 1 / 111,485 / 2,009 |
| cursors | 1 / 154,076 / 2,932 | same | same |
| scene | 1 / 126,078 / 2,223 | same | 1 / 126,078 / 2,391 |
| bridge | 1 / 147,401 / 2,357 | same | 1 / 147,401 / 2,526 |
| interaction | 1 / 208,907 / 2,736 | same | 1 / 208,907 / 3,348 |
| host | 1 / 123,828 / 1,846 | same | 2 / 126,594 / 2,051 |

Test closures (each `[[test]]` is its own unit): all single-file, at most `host world_host_unit_2` 1 / 231,870 B / 3,026 lines and `world_host_unit` 1 / 193,749 / 2,951; the others are between 21 and 945 lines. Unit tests inside `model` (trace module tests) make its `lib+cfg(test)` closure 5 / 193,741 B / **3,604** lines (above the 3,500 target, below the 4,000 gate; two fixtures stay embedded there).

## Verification (every command run, with result)

1. Baseline: stitched sandbox of the pre-carve world (`woracle.py base`): `rustc +nightly-2026-07-20 -Zno-codegen --error-format=json` with the infinite glue patches (extern crates dropped, external globs neutralised), configs lib 2558, test 4257, wasm32-unknown 2431, wasip2 1491, wasip2 test 4257 errors (all unresolved externals). Note `-o /dev/null` is refused in this tool sandbox (EPERM creating rustc temp dir under `/dev`); `-Zno-codegen --out-dir <scratch>` was used and emptied after each run, no `.rmeta` was consumed.
2. Phase 1 (decomposition in the single crate): identical error multisets in all five configs (lib, test, wasm, p2, p2 test), including every rewritten test.
3. Final (crates stitched as modules of one analysis crate, run on the LIVE tree after the switch-over, external-crate and `couldn't read` noise filtered from both sides, `CARGO_MANIFEST_DIR` set): lib 930 / 930, wasm 824 / 824, wasip2 737 / 737, test+`testkit` 1,365 / 1,365 errors, no difference except the abort-line count. `wasip2 test` shows 2 unresolved facade imports that exist only because that exotic config now mounts the facade-level resources test (the baseline never compiled it there); base and new also share 7 errors with different labels.
4. Sensitivity (seeded): leftover `state` / undefined type in a lib file (lib config) and in the unit tests (test config) are reported; making a cross-crate `pub fn` private reports E0603; making a `pub` field private is reported by the definition scan.
5. Arity: `warity.py` checks every call of a world function/method against definitions; regression set versus the pre-carve tree is empty (it found 3 genuine mistakes during development: host variables named other than `state`, the `self` Drop in tests, and the external-API test, all fixed).
6. Definition scan (`wdefscan.py`): non-pub definitions named by another crate or any test: 5, all in the moved trace module and all name collisions with std-style names (`batch`, `index`, `cursor`, `clear`, `retire`); everything else is `pub`.
7. Coherence (`wcoherence.py`): every inherent/trait impl and `artifact_retire_*!` call sits in the crate of its type (0 findings).
8. Paths: `wpaths.py` 141 `#[path]`/`include_*` literals plus 129 run-time read paths resolve (0 missing); longest path 189 bytes.
9. rustfmt parse (`rustfmt +nightly-2026-07-20 --emit stdout`) of all 7 lib roots, `testkit`, facade and 18 test files: exit 0. TOML: 7 manifests parse with `Bun.TOML`; `launch.json`, the seed and `📋️build-owners.json` parse; members byte-sorted; 42 launch routes (6 per crate, in both launch files) and 7 owner rows present (`/tmp` script `oinf-verify.ts`, deleted after use).
10. Launch routes for the 14 old `semio-framework-os-infinite:test --lib world...` filters now run `@semio-tech/os-infinite-world-<crate>-rs:test ... --tests <filter>` (28 rewrites across both files); filters spanning several crates point at the crate holding most of them, `world::` at `host`.

## Files changed

New: the seven crate nodes under `INF/🌍️world/` (lib roots, `📦️packages/🦀️rust/{Cargo.toml,📋️project.json}`, `🧪️tests/*`, `host/🧰️testkit`, trace moved to `model/⏯️tool-run-trace`). Replaced: `INF/🌍️world/🦀️.rs` (facade). Removed: the 13 former world child modules (`🧹️lifecycle`, `🧵️interaction/*`, `🧱️scene`, `🖌️overlay`, `🌉️bridge`, `🎛️frame`, `🖱️pointer`, `📡️assets`) and the six old test sources. Registries: root `Cargo.toml` (7 members), `📋️build-owners.json` (7 rows), `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (42 routes each, route rewrites), `INF/📦️packages/🦀️rust/Cargo.toml` (7 deps). Source readers: `📜️script.ts` (`INTERACTIVITY_AUDIT_WORLD3D_FILES`), `✏️s/🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript/📜️script.ts`, `📖️source-access/🟦️.ts` (`POLICY_SPLIT_SOURCE_ROOTS`) now read the seven roots. Consumers as listed above.

## Residual risks and open items

- Real compile and tests never ran (launcher); Cargo.lock refresh pending. Highest-risk categories not covered by the oracle: borrowck after narrowing (calls now borrow only the needed components, so strictly fewer overlaps than before, but untested), trait-method scoping (the only local trait, `WorldMeshSurface`, is used inside `components` only), field/method privacy beyond the name scan, `required` dev-dependency features of the new test targets.
- `wasip2 + cfg(test)` of the facade resources test is not compilable (world crates are empty there) — it was not part of any lane before.
- Doc comments in peers' sources still mention the old world root. The `📽️nested-cargo-package-projection` corpus has no generator (see "Wrapper removal"); its `🌍️world/🦀️.rs` row was already stale at HEAD and is untouched.
- The `model` lib+cfg(test) closure is 3,604 lines (see census); a follow-up can move the trace unit tests to an integration target.
- Helper duplication in test targets (about 1.1k lines) could move into a shared `testkit` crate.
- Terrain crate name: untouched here. The puzzle2d knowledge was relocated by another session before the board carve; the board carve started from its result.

## Board half: BoardHost and DagHost decomposition plan (the plan; executed in "Board carve (executed)")

Measured on the live board tree (`oinf-tools/t3.py`, `w18`-style field/method usage; no edits made):

| God object | fields | methods | impl lines | method-level SCCs |
|---|---|---|---|---|
| `BoardHost` (`➕️normal`) | 85 | 371 (107 pub) | 8,059 | all singletons (acyclic) |
| `DagHost` (`🕸️dag`) | 47 | 231 (115 pub) | 3,632 | all singletons (acyclic) |

Field clusters by the child modules whose methods touch them (the natural component seams):

`BoardHost` components (proposal, crate in brackets):
- `BoardScene` (28 fields touched by every child: `camera`, `nodes`, `handles`, `edges`, `wires`, `regions`, `width`, `dpr`, `grid_*`, `content_scene_generation`, `world_content_cache`) [board-scene, lowest].
- `BoardSelection` (`selection`, `preselect`, `preselect_removed`, `selection_exit_highlight`, `selection_options`, `hovered_id`, `hovered_kind`, `highlighted_ids`, `interaction`, `active_utility`, `selection_preview_crossing`) [board-selection].
- `BoardStyle` (style catalogs: `wire_kinds`, `edge_kinds`, `edge_tips`, `handle_kinds`, `node_kinds`, `link_compat_rules`, `icon_paint_cache`, `world_raster_tiling`, `automatic_lod`, `forced_draw_lod`, `port_mode`, `preserve_original_element_style`, `naming`) [board-style].
- `BoardBrush` (`brush_candidates_emit_key`, `brush_preview_emit_key`, `brush_alt_pressed`, `brush_node_size`, `brush_*_weights`, `suggestion_offset`, `drop_preview`, the fill job registers) [board-brush, with `BoardFillJob`].
- `BoardEvents` (`events`, `event_overflow`, `event_batch_overflow`, `gesture_serial`, `last_select_emit_sig`, `last_preselect_emit_sig`) [board-events, with `BoardEventQueue`/`BoardOwnedEvent`].
- `BoardGestures` (`transform_drag`, `region_drag`, `transform_flags`, `area_brush_extent`, `link_screen_preview`, `grid_snap_enabled`) [board-gestures].
- `BoardCommit` (`pending_pointer_commit`, `queued_pointer_commit`, `cancelled_pointer_plan_retirement`, `pointer_publication`, `pending_delete_planning`, `pending_delete_operation`) [board-commit, top].
- `BoardRetirement` (`close_phase`, `close_entity_retirement`, `close_strings`, `close_string_len`, `close_node_handles`, `opaque_scene_retirement`, `opaque_scene_fault`) [with `BoardHostRetirement`].
- `BoardHost` thin owner in the top crate: composes the components, keeps `new`, `Default`, `CanvasContent`, and the delegates external callers use (76 distinct method names called from 113 external files; callee closure 5,492 lines).

`DagHost` components: `DagViewport` (`width`, `height`, `dpr`, `wheel_zoom_*`, `automatic_lod`, `forced_draw_lod`, `grid_*`), `DagGraph` (`engine`, `host_snapshot`, `node_id_map`, `handle_key_map`, `edge_id_map`, `edge_engine_ids`), `DagPointer` (`widget_drag`, `pending_*`, `pan_anchor`, `last_pointer_down_*`, `ghost_node`), `DagPaintState` (`canvas_theme`, `computing_*_anim_phase`, `node_eval_status`, `unresolved_input_ports`, `icon_paint_cache`, `handle_port_*`, `edge_route_style`), `DagMinimap` (`minimap_widget_*`), `DagNoteEdit` (`editing_note`, `caret_visible`), thin `DagHost` owner.

Method of execution (the same engine that carved the world, `oinf-tools/wengine.py`, `wrewrite.py`, `wgen.py`): (1) field-to-component map as `wmap.py`; (2) classify every `self`-method of the god object by the components it touches (`.field` and callee closure), turn those touching one component into methods of that component and the rest into functions or methods taking explicit component references, widen to `pub`; (3) hosted orchestrators keep the host signature; (4) carve with the unit/repair pipeline (`lane.py`), export the external API names from their defining crates and re-point outside callers to the component borrows (no wrapper layer, as with the world half); (5) validate with the stitched differential, arity checker, coherence checker and definition scan. Expected shape: about 9 board crates plus the DagHost crates; the puzzle2d items (`puzzle_2d_lod_scale_json`, `puzzle2d.brush.`, `puzzle2d-fill-*`) must already be relocated by the other session so that no component crate names a plugin.

Blocker removed by the coordinator decision: no extension traits are needed once the god objects are components; the cycle in the world (state to cursors) was resolved the same way by narrowing cursor methods to explicit component references.

## Wrapper removal (compatibility layer deleted)

The 52 `host::api` functions kept the old `&mut World3dState` signatures for outside callers; that was a compatibility layer and is gone: the module `INF/🌍️world/🎛️host/🔌️api/` and its `pub mod api;` mount no longer exist, the pipeline no longer generates them (`oinf-tools/wapi.py` lost `wrapper_text`/`build_api`/`imports_for`; `wrun3.py`, `wrun4.py`, `wdefscan.py`, `wcoherence.py` no longer mention `api`), and the host manifest lost the two dependencies only the wrappers used (`semio-framework-ui-runtime`, `semio-framework-ui-contract`).

Decision per function: none of the 52 is an owner-level orchestration. Each one is a narrowed component function (`comp_list` is 1 to 3 components and none needs the whole owner), so the clean end state for all 52 is the first option, the call site borrows the components and calls the component function. No new `World3dState` methods were needed (the real owner-level orchestrations, e.g. `step_world3d_interaction`, `step_world3d_snapshot`, `step_world3d_scene_bridge`, were already host functions and stay as they are). The facade now exports every one of the 52 from its defining crate: `components` 4, `scene` 20, `bridge` 12 (2 of them non-p2 only), `interaction` 15, `host` 1 (`world3d_modelling_accessibility`; the wrapper there only forwarded to a same-crate function of the same name, now exported directly).

Re-pointing (tool `oinf-tools/wunwrap2.py`; each first argument `state` / `&mut state` / `world` became the component borrows in canonical component order with the mutability the narrowed function declares, e.g. `enqueue_world3d_event(&mut state.selection, &mut state.authority, intent)` instead of `enqueue_world3d_event(state, intent)`). 130 sites in 20 files (all files re-read right before the edit; consumer-side only): `mailbox-impl` 32, `wgpu-renderer-async-boundary` test 20, infinite `♻️resources` test 9, `frame-build` 8, `scene` (icon export) 7, `wgpu-engine-surfaces` test 6, `wgpu-navbar-footer-parity` test 6, `app-runtime` 5, `wgpu-os-host-standalone` 5, `keyboard-and-palette` 5, `settings-and-dock-persistence` 5, `pointer-input` 5, `tutorial-runtime` 4, `scene-render-entry` 3, `chrome-paint-and-navbar` 3, `chrome-frame-and-main-window` 2, `wgpu-shell-input` test 2, `icon-render` 1, `wgpu-icon-render` test 1, `window-actions-and-commands` 1. 119 call sites rewrote mechanically; the other 11 by hand: 2 asset-take sites in `mailbox-impl` (the token lookup now binds `let world = ...get_token_mut(surface_token)?;` and passes `&mut world.assets`), the `tutorial-runtime` scene point (binds `let world = state.world3d_state_for_window(id)?;` and passes `&world.session, &world.camera`; the closure site is mechanical), and 8 function values (`.any(world3d_asset_decode_pending)`, `.and_then(step_world3d_brush_mesh_announce)`, `.map(world3d_camera_projection)` x 4, `.map(world3d_projection_orientation)`, `.is_some_and(world3d_pending_camera_settle)`) which became closures over the needed components.

Source-text tests that pinned the old call shapes were updated to the new shapes: `frame-action-ledger` (draw-rebuild arm text, 2 anchors), `wgpu-wheel-and-escape-routing` (hover-clear anchor and its negative anchor), `scene-input-residency/🟦️.ts` (`world3d_reference_url_is_current(&state.scene, &url)`). The same TS test also still read the old facade module tree for `grid_snap_enabled`, `snap_world_point_to_grid(` (4) and `cancel_reference_image_url`, which the world carve had moved out of the facade; it now reads the seven crate roots (`WORLD_CRATES`). Checked with a bun emulation of its assertions (`bun test` of this file crashes bun 1.3.14 with a segfault; `vitest run` produced no output within 250 s), all five hold: true / 4 / true / true / true. A repository-wide scan (31,274 `*.rs` files from the git index plus untracked; 11,953 `ts/tsx/py/sh/jsonc` and 40,759 `json` files outside tickets and generated folders) found the 52 names nowhere else, apart from prose in two fixture JSON files.

Verification after the change (all on the live tree):

1. Stitched differential (`wsb.py`, lib / wasm / p2 / ktest): base 930 / 824 / 737 / 1,365 errors, new 930 / 824 / 737 / 1,365; the multiset differs only in the `aborting due to N previous errors` summary line (lib, wasm, ktest), as in every earlier run (`🗑️generated/oinf-oracle-unwrap-a.log`, `-b.log`). Note the oracle stitches only infinite, so it covers the facade resources test but not the 19 outside files.
2. Arity: `warity.py` over infinite, aggregated per name versus the pre-carve snapshot, regressions none (253 pre-existing counts versus 259 in the snapshot). Outside call sites: new `wextarity.py` checks the argument count of all 130 external uses against the narrowed definitions: 4 flagged, all false positives of the argument counter on closure parameter lists (`|key, source|` counts as two arguments; the same call shapes existed before; `world3d_return_native_mesh_gpu_step` 1 site, `world3d_close_native_mesh_step` 3 sites).
3. Definition scan 5 findings (unchanged, the trace-module names); coherence 0; paths 11 literals plus the run-time reads resolve, longest path 189 bytes; rustfmt parse of the 7 lib roots, the facade and all 114 outside files that import the world: exit 0.
4. Census (`🗑️generated/oinf-census-unwrap.log`): every crate unchanged except `host`, which shrank: lib 1 / 123,828 / 1,846, lib+testkit 2 / 126,594 / 2,051; `model` lib+cfg(test) 5 / 193,741 / 3,604 stays the maximum (gate 4,000).
5. Not run: cargo (launcher still refuses), so borrowck of the 130 rewritten sites is untested. The new call shapes borrow disjoint fields of one place, which the old shape did not allow in the same expression, so overlaps can only decrease; the remaining risk is a mutable/shared mismatch at a site whose variable is a shared reference, which the arity/oracle checks cannot see.

`📽️nested-cargo-package-projection/🔣️.json` was not regenerated: there is no generator. The file is an authored authority catalog pinned by `authorityCatalogSha256` in `🔣️taxonomy.json` (`semanticPackageProjectionContracts["nested-cargo-packages-v1"]`); `🔍️discovery/🟦️.ts` (`semanticPackageProjectionCatalog`) and `🧹️normalization/🟦️.ts` only read and validate it, and `occurrenceCount` is not recomputed or checked by any code. The row for `🌍️world/🦀️.rs` (`wgpu-engine-relative`, 5) was already stale at `HEAD` (the old token `../../📺️renderer/🧑‍🎨engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/` occurs 0 times in `HEAD:...🌍️world/🦀️.rs` and in all current infinite sources), so it is independent of this carve and of the wrapper removal. Editing it by hand would require re-pinning the digest in `🔣️taxonomy.json`, which belongs to the catalog owner.

## Board carve (executed)

Started from the tree after `RESULT-RELOCATE-BOARD-PUZZLE2D.md`; the board files were re-read (live `🎲️board` tree byte-identical to the snapshot taken at 18:10 immediately before the switch, checked again by `bswitch.py`). Tools (all new, in `oinf-tools/`): `bdec.py` decomposition engine (collect, classify, rewrite), `bstage.py`/`bprep.py` staging, `bmap.py`/`dmap.py` field-to-component maps, `bplan2.py`/`blane.py`/`bscc*.py`/`bcrates.py` layering, `bgen.py`/`btgen.py`/`bbuild.py`/`bman.py`/`bfinal.py` generators, `bapi.py`/`bfacade.py` facade, `bxinfer.py`/`bxfile.py` outside-caller rewriting, `bsb.py` stitched differential, `barity.py`, `bdefscan.py`, `bcoherence.py`, `bpaths.py`, `bresid3.py`, `bcensus.ts`, `bpoint.py`, `breg.py`, `bswitch.py`.

### Decomposition: no extension traits, no compatibility wrappers

`BoardHost` (85 fields, 371 methods) became a thin owner of 8 component structs, `DagHost` (47 fields, 231 methods) of 6; every field of the god objects is a `pub` field of exactly one component.

| host | components (struct fields on the owner) |
|---|---|
| `BoardHost` | `scene`, `focus`, `style`, `brush`, `outbox`, `gestures`, `commit`, `closing` |
| `DagHost` | `viewport`, `graph`, `pointer`, `paint`, `minimap`, `notes` |

Every method was classified once by the components it touches (`bdec.classify`):

| host | comp (method of one component) | narrow (free fn over 2-5 components) | free (no component) | owner (stays on the host, orchestrates) | traits kept |
|---|---|---|---|---|---|
| `BoardHost` | 132 | 164 methods + 6 former free fns | 40 | 39 | 3 |
| `DagHost` | 122 | 64 methods + 7 former free fns | 29 | 16 | 14 |

Only genuine owner methods (those that touch most components: pointer/render/close orchestration, constructors, snapshot load/save, `CanvasContent`) remain on `BoardHost`/`DagHost`. All other call sites borrow the component (`host.scene.m(..)`) or pass component references to a free function (`board_set_camera(&mut host.scene, &mut host.outbox, ..)`); repeated `borrow_mut()` receivers are bound once through a `match &mut recv { __host => .. }` wrapper so `RefCell` is borrowed once. No `impl Trait for` extension traits were added, there is no `pub fn x(&self)` delegate that merely forwards to a component (0 wrappers; checked by the `bdec` hazard list and `bdefscan.py`).

### Crate graph (11 producer crates, downward only; `INF/🎲️board/...`)

| crate (package `semio-framework-os-infinite-board-*`) | folder | contents | lib lines / bytes (gate 4000 / 262144) | test targets |
|---|---|---|---|---|
| `schema` | `🧬️schema` | layout contracts, palette, dag input records, layout inference, text io codecs | 2,547 / 136,810 | tidy_tree, progress, board_host (source-contract), dag_unit |
| `engine` | `⚙️engine` | retained graph engine, camera, selection, directed port descriptors | 3,298 / 168,477 (3,304 with testkit) | 6 (engine_unit 669 lines, undirected, quadrant, ...) |
| `normal-model` | `➕️normal/🗃️model` | events, plans, kinds, constants | 2,013 / 86,242 | 1 |
| `normal-scene` | `➕️normal/🖼️scene` | `BoardSceneState`, `BoardStyleState`, painting and viewport fns | 2,381 / 114,027 | 0 |
| `normal-focus` | `➕️normal/🎯️focus` | `BoardFocusState`, `BoardOutboxState`, picking, descriptor sync | 2,114 / 98,461 | 0 |
| `normal-gestures` | `➕️normal/🕹️gestures` | brush, gestures, commit, closing components | 1,600 / 74,886 | 0 |
| `normal-fill` | `➕️normal/🪣️fill` | fill capture, ingress, job, commit encoder | 3,145 / 155,644 | 1 (226 lines) |
| `normal-host` | `➕️normal/🧭️host` | `BoardHost` owner, pointer and render orchestration, `BoardHostRetirement` | 2,622 / 157,109 | 1 (board_host, 1,517 lines) |
| `dag-model` | `🕸️dag/🗃️model` | geometry, LOD, preview records, edits, snapshot loaders | 1,940 / 83,336 | 2 |
| `dag-graph` | `🕸️dag/🕸️graph` | `DagGraphState`, `DagViewportState`, engine sync, selection, projection | 1,986 / 104,797 | 0 |
| `dag-host` | `🕸️dag/🧭️host` | `DagHost` owner, pointer, paint, minimap, notes, wasm `DagSession` | 1,602 (1,763 wasm) / 106,511 | 4 (dag_unit 1,466 lines) |

Dependencies are layered (`schema` < `engine` < `normal-model` < `normal-scene` < `normal-focus` < `normal-gestures` < `normal-fill` < `normal-host`; `dag-model` < `dag-graph` < `dag-host`, the dag crates also use `engine`, `schema`, `normal-model`). Every crate has the `testkit` feature and a self dev-dependency (`cfg(any(test, feature = "testkit"))`), run-time fixture reads (`read_static`/`read_joined`), and its tests as `[[test]]` targets `board_<crate>_<slug>`. `INF/📦️packages/🦀️rust/🦀️.rs` (the `semio-framework-os-infinite` facade) keeps the `component`/`world` mounts and kernel aliases and re-exports every previously public path of `board::` (and the new free functions that outside code now calls) from the 11 crates; its `Cargo.toml` gained the 11 path dependencies. All `.json` fixtures, the `.ts`/`.proto`/`.graphql` schema twins and the `🖼️presentation` crate stayed where they were.

### What changed in the tree

- Created 50 files (11 libs, 19 test targets, 11 `Cargo.toml`, 11 `📋️project.json`), updated 3 (`🧭️host/🦀️.rs`, `🧬️schema/🦀️.rs` replaced in place, facade lib `📦️packages/🦀️rust/🦀️.rs`), removed 61 old `.rs` board module sources (all old module and test sources; the `🖼️presentation` crate and every fixture/json/ts/proto file kept). The exact created/updated/removed/outside-file lists are in `🗑️generated/oinf-board-switch-log.json`.
- 27 outside files re-pointed (callers borrow components, call the free functions, or the genuine owner methods): puzzle 2d engine/board-host/snapshot/wasm/edit and tests (linking, brush, select-tool-transactions, unit), flow/dag/sequence/procedural flow-window editors, flow host (authoring, evaluation history, unit tests, retirement tests), renderer `EngineCanvas` (board2d, node-graph, engine-surface-attach and two test files) and `NodeGraph` (+ unit tests). Receiver inference covers: declared types, constructor/return functions, closure parameters of `with_board_host*`, binder propagation, tuple binders, and first-argument receivers of former free functions; 4 temporary-receiver call sites (`BoardHost::new().x()`, `puzzle_board_host().naming()`, `DagHost::from_host_snapshot(..).entity_screen_json(..)`) are fixed through `bxinfer.TEMP_FIXES`.
- TS readers of board Rust source re-pointed (`bpoint.py`): root `📜️script.ts` (p3mn dag source), flow host `🧪️source-contract`, renderer tests `interactivity-mounted-engine-surface-lifetime`, `node-graph-wire-edit`, `node-graph-edit-rows`, `node-graph-gestures` (the `screen_pointer_gesture_begins_at` needle now reads `screen_hit(viewport, graph, minimap, sx, sy)...`, `is_draggable_body` is read from the schema lib), repo `📖️source-access` split-root list. Fixture JSON paths updated: dag `🔗️wire-edit`, flow `🔌️port-types`, dag-input `receivers.json` (the latter has no reader).
- Registration: root `Cargo.toml` (11 members, byte-sorted), `📋️build-owners.json` (11 rows), `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (6 routes per crate: build, check, test x prototype, release; unique `order` values from `4_build`/`4_gate`/`4_test` bases). All three parse (JSON/TOML) after the edit; no routes filtered board tests of the monolith crate.

### Verification (every command run, with result)

1. Patched stitched differential (`rustc +nightly-2026-07-20 -Zno-codegen` on sandbox copies of the live tree after the switch, `🗑️generated/oinf-board-live-oracle-1.log`, `-2.log`; same result as on the staging tree, `oinf-board-oracle-10.log`, `-11.log`): lib 931 base errors vs 735 new (197 base-only, all `LayoutError` glob cases), wasm 825 vs 778, p2 738 vs 542, ktest 1,366 vs 1,134. The only new lines are the `aborting due to N previous errors` counters and, in ktest, three `E0433 cannot find type DagExpandedPaths`: the test globs the unmounted `directed_presentation` crate, where `DagExpandedPaths` is defined (`🖼️presentation/🗂️expanded`); the oracle neutralises that glob. So name resolution of every crate, test target and the wasm/p2 cfg lanes is equal to the baseline.
2. Arity scan (`barity.py`, new tree vs baseline with the original outside files): 90 raw mismatches in the new tree, 91 in the baseline, all name collisions between unrelated same-named functions; after the final run the multiset difference is 4 new keys, all `commit`/`publish` collisions with different-type methods (same class as the 5 baseline-only keys). Two earlier genuine misses (`dag_screen_to_world` at 3 sites, `load_board_snapshot_json` at 2) were fixed by the first-argument receiver inference and are gone.
3. Residual method scan (`bresid3.py`, per host): remaining hits are component-internal receivers (`self.engine.*`, `scene_state.*`, `style_state.*`) only; it found four real misses that were fixed (tuple binders `fits`/`oversized` in the dag edit-rows test, `DagHost::from_host_snapshot(..).entity_screen_json`, and the whole `DagSession` wasm wrapper whose receivers `self.state.borrow().host`, `inner.host`, `g.host` were not matched; all `DagSession` method calls and `dag_take_pending_open_instance_id`/`dag_screen_to_world` now go through components; generated in `bdec.py` via `ext` aliases).
4. Definition scan (`bdefscan.py`): 2 findings left, both false positives (`admit` `pub(crate)` and `object` private free functions in `schema`; the same names are used only as methods in other crates, there is no cross-crate free-function caller). Coherence scan (`bcoherence.py`): 0 findings (no inherent impl outside its type crate, no orphan impls).
5. Census (`bcensus.ts`, canonical `readRustSourceClosureV1`, `🗑️generated/oinf-board-census-live.log`): every crate is one source file per lib; maximum lib 3,298 lines (`engine`, 168,477 bytes), maximum lib+cfg(test) 3,304; host cfg, wasm, p2 and lib+testkit all below 3,500 lines / 230,000 bytes; every test target below 1,550 lines (largest: `dag-host` dag_unit 1,466, `normal-host` board_host 1,517). Facade: 3 files / 44,119 bytes / 315 lines.
6. Paths (`bpaths.py` against the live tree): all 14 run-time fixture literals of the new test targets resolve; the `BOARD_HOST_SOURCE` source-contract tests now read the `normal-fill` lib (the text they assert on lives there) instead of the removed module files, and every needle string they use was checked to occur in that file.
7. Parsing: `rustfmt` (nightly, `skip_children`) parses all 58 generated files (31 under `INF`, 27 outside) without error (negative control run). TS reader needles (`gestures`, `edit-rows`, `wire-edit`, source-contract) were evaluated against the new sources with `bun` (`readers.ts` in the scratchpad): all hold. `bun test` was not run (it scans 7.6 M files).
8. Launcher: `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-infinite-board-schema --lib --offline` (`🗑️generated/oinf-board-cargo-schema.log`) fails in cache maintenance with `Finite original maintenance authority required`, before reaching rustc. No real compile or `cargo test` was possible; root `Cargo.lock` is not refreshed for the 11 new members.

### Residual risks and open items

- No typeck, borrowck, trait-method resolution or privacy check ran (launcher); the oracle covers name resolution only. Mitigated by construction (every `V.m(` of a known receiver classified; component field `pub`; narrow functions borrow disjoint components) and the scans above. The wasm `DagSession` (`RefCell<DagSessionInner>`) is the most borrowck-sensitive spot (`paint_scene` stays an owner method so `inner.width` is not read while a component is mutably borrowed).
- Methods called on receivers the inference cannot type (macros, deeply chained temporaries, generic helpers) would still compile-fail rather than miscompile (a moved method name does not exist on the owner); the first real `cargo check` will list them.
- Test code duplication across crates (helper fns copied into each test target) as in the world carve.
- `interactivityMountedEngineSurfaceLifetimeFailures` (root `📜️script.ts`) slices the dag source from `pub struct DagHostRetirement` to `enum DagNodeEvalStatusKind`; measured with `readModuleTreeSource`, that slice has length 0 both on the old module tree and on the new model/graph/host concatenation (retirement is a `ControlledRetirement<DagHost>` wrapper now, and the enum precedes the struct), so the P3mn dag-retirement requirements see the same empty text before and after; the root script itself was not run.
- The facade replicates the old `board::` paths (explicit re-exports) so unlisted callers keep resolving; a later pass can retire `board::` paths per consumer.
