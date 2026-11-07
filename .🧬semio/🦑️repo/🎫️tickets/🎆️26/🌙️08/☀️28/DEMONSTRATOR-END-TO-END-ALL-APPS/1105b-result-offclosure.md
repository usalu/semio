STATUS: PARTIAL. GREEN (finished wasm runs): raster, animate, sequence, cad editors; draw scene-support, scene, scene-paint, scene-mounted, mutations, io (offclosure-45). Patched but UNVERIFIED: draw editor-store-preparation (wire staged admission, replay factory, presence retire), host-digest (duplicate-rewrite, SetArtboard arm), layout-mutations/engine small fixes, viewer, write/export. offclosure-49 was blocked by a peer's in-flight framework edit (semio-plugin-retained-command: unresolved import `retained_command_job`, needs its manifest dep/lock); earlier retries by dsl-record-model NativeSnapshotFailure (hostcheck). Rerun: chk.sh with -p draw-editor-store-preparation -p draw-host-digest -p draw-editor-interaction -p draw-write -p draw-viewer, then layout editor.

| plugin | verdict | log |
|---|---|---|
| raster editor | GREEN (rc 0, 0 errors, Finished) | offclosure-18.txt |
| animate editor (semio-s-artifact-animate-presentation-animate) | GREEN (rc 0, 0 errors, Finished in 20m48s) | offclosure-33.txt |
| sequence editor, v1, root | GREEN (root: cargo Finished, 0 errors; post-step "Cargo profile Source changed" from a concurrent edit) | offclosure-19/20/22.txt |
| layout editor | BLOCKED by draw scene-support (offclosure-41: 7 errors, all scene-support); layout-model/diff-plan/diff retained/mutations/decisions now `.result`-ported; engine `background_scene` and diff `.work` posting still open | offclosure-41.txt |
| draw | scene-support flatten RED (6, below); metadata set-artboard owner dirs added (no longer in errors) | offclosure-41.txt |
| cad editor | GREEN after the window-config/tool-run refactor settled | claims |
| writer, lowpoly | static tool_contract -> plugin_app_close_prelude port only; UNVERIFIED | - |

## Animate (done)
- Tile row delta moved into tile-grid (🧮️tile-grid/🔺️delta): `list_delta!` generates `Keyed for $row`, so it must live with the crate that owns FigureTileDraft; the diff re-exports. No dependency cycle (tile-grid depends on nothing animate).
- host/owned `.result`; editor: `dsl` extern, config-store-preparation on the new ArtifactStoreOneItemPreparation API (begin_copy_byte_demand, last_turn_work, 4-arg advance), reserved-import turn type, play-app `Fault.into()`, PresentationConfigMutation + SetEngagementInput `DslReceiving` and `original_ordinal_operation_receiving!`.

## Layout: what is left (offclosure-35)
- layout-model, layout-diff-plan: green. layout-diff retained (collection, final-list, data, page-cache) now reads `.result` (work value is dropped there; posting `.work` needs the enclosing close_step fns to return RetainedWorkOutcome, not done).
- layout-engine (13): `ui_render::text` no longer exists (display-list-and-hit-testing:11, layout-engine-and-views:138); text-fonts:167 missing lifetime; `semio_s_artifact_layout_layout_model::background_drawing_scene` removed (drawing-labels-and-rasters:13); drawing composition: `VectorFillHead` no longer a root export of semio_framework_io_ports (now `admission::vector`), `EmbeddedChildAdmissionCursorV1` is not Sync (composition:46), enum variants now take 2 args (composition:31), mismatched types (74). I already redirected the Vector* imports in scene/🧵️drawing and renamed FontWorkLedger -> RetainedWorkLedger (11 files); composition/🦀️.rs still uses the old `VectorFillHead` path.
- layout-mutations-content-2 (4): `crate::set_story_runs` missing for ✍️edit-story (28/39/78/84): the module that held it was not mounted in the carve.

## Draw: what is left
- scene-support/📽flatten.rs:154 `semio_framework_2d::booleans::boolean_paths_many` and :176 `semio_framework_2d::trace::trace_bitmap_paths` are gone BY DESIGN: they were synchronous one-shot kernels. Replacements are the resumable, work-ledgered `PathBooleanJob` (booleans/🛤️paths) and `BitmapTraceJob` (🔍️trace), both `advance(budget, &mut RetainedWorkLedger)` with funded retirement (`into_retirement`), and the draw scene already has its own `DocumentBooleanJob` (🎬️scene/🔀️booleans). Not re-added, not ported: a synchronous wrapper would bypass cancellation/funding. Proposal: make `flatten_drawing_document_to_scene_nodes` (used by the SVG and PDF export serializers and 3 tests) a resumable `DocumentSceneFlattenJob` driven by those jobs under the export job's grant, and have the serializers consume it through the export job's step loop. This is a design change for the draw/export owner.
- metadata `set-artboard`: both MutationLeaf errors trace to "MutationLeaf source authority failed: No such file or directory" (derive reads the taxonomy owner dirs; every sibling has ↩️inverse, 🎮️prepare, 🔺️diff, 🦠️mutation; set-artboard has only 🦠️mutation, 🧬️schema and 🧪️.json). I split its MutationKind/ColdMutationKind and added CanonicalJsonTree to DrawingArtboard and NativeReceive to DrawingIdentityAssignment; the authority gap needs its owner dirs or a registered single-file exception. UNVERIFIED which file the derive wants.
- scene/📷️raster: removed a stale `use crate::preparation::DocumentSceneError` (duplicate of crate::error).

## Earlier work (still valid)
Raster, sequence, stdio jpg/svg/gif, manifests (all locked), tool_contract -> plugin_app_close_prelude ports in animate/layout/draw/cad tests/writer/lowpoly (see claims lines).


## Draw flatten migration (done, wasm-checked up to the store-preparation block)
- Deleted the sync flatten (scene-support/📽flatten.rs) and its re-exports; legacy sync svg/pdf writers, `drawing_document_to_svg/pdf/json_to_svg`, `drawing_document_to_semio_drawing` and the legacy svg/pdf test dirs are gone (write jobs `SvgWriteJob`/`PdfWriteJob` remain the one writer). The TS twins (svg/pdf serializer .ts) were NOT touched and still mirror the deleted Rust: owner of the TS lane must drop them.
- New crate `semio-s-artifact-draw-drawing-scene-mounted` (🧬️schema/🎬️scene/🧵️mounted): the retained geometry owner extracted from editor-interaction; `reconcile(Option<AppRenderOperationContext>, take)` is generic over editor and viewer. Consumers: editor (play-app, gesture-owners, clipboard job), canvas-pointer-down, viewer. Locked (lock-29/30/31).
- Viewer: mounted hooks (prepare, maintenance, close, pending_effects) + canvas renders `scene::view::nodes` from the mounted plan.
- Exports: png/svg/pdf/dxf/dwg borrowed `serialize` now refuses (`owned_only_refusal`); DXF_FACTORY/DWG_FACTORY added; `ExportProducer` gained `semio-project` and `semio-encode` stages (plan -> semio drawing -> the one stdio encoder), fuel per node, cancellable like the other stages.
- `vector:out`: `drawing_vector_media(plan, doc)` uses the mounted plan and the SvgWriteJob (inline bounded loop because `export_media` is a sync trait method), pages drained under exact grants.
- Scene testkit: `preparation::testkit::{scene_plan,scene_nodes_of_plan,scene_nodes}`; ~17 tests migrated, svg/fill-rule/import round-trip tests drive SvgWriteJob. scene dev-dep `testkit` feature added to the crates whose tests use it. Test targets NOT compiled yet (only --lib checks).
- Fixed on the way: drawing-mutations retained (.result, missing `;` in drag, rename-layer inverse `take(grant)`), host-arena owned-field-decode `.result`, `OriginalWorkFailure` messages via `domain_message()`.

## Layout engine and diff
- Engine: embedded drawings are no longer materialized into a sync scene. `background_scene`, the labels/rasters module and the stroke placement are deleted; every placed drawing frame now emits a deferred `drawing_bounds` image that the streaming `background_drawing_*` consumers paint. The 6 engine unit tests that asserted the materialized strokes/labels/rasters were removed with the capability (unverified compile: offclosure-46).
- Layout-diff `.work` posting: NOT possible at this layer. Every retained close_step in the diff/validation ladders implements a framework trait (`RetainedDiffApply::close_step`, `RetainedMutationClose*`, `ColdMutation*`) whose return is `Result<RetainedCloneStep,ValueError>`; the framework wrappers drop work as well. Posting needs those traits to return `RetainedWorkOutcome` (framework change). The ladder reads `.result` meanwhile.
- Open: `include_background` parameter of `assemble_display_list_items` is now unused (warning).

## offclosure-46 (layout editor) after this session
Errors left in the log: draw editor-store-preparation (24 + 2), draw editor-interaction presence retire signature (2), draw host-digest (duplicate-rewrite: take/advance arity + .result, mutation-digest: DrawingMutation::SetArtboard not matched), layout-mutations (retained spread/batch step_granted `.result`: patched), layout-engine (frame_strokes inference: removed the dead variable). The draw items are in the drawing/reacthost lane's files (its reacthost-rs-2 log shows the same errors); I left them alone. The new SetArtboard variant needs a digest arm from that owner.

## TS parity status (coordinator question)
Only SVG had a real legacy TS serializer body (drawingSceneToSvg); it is gone and the twin file is now `export {};` like the PDF/PNG/DXF/DWG/JSON twins, which were already empty stubs (never had a TS serializer). The job-driven TS writers `SvgWriteJob` and `PdfWriteJob` (🧵️write/🟦️.ts) already exist and share fixtures/oracles with the Rust jobs in their tests. The svg-import round-trip TS test now drives TS SvgWriteJob. TS tests were NOT run. DXF/DWG have no TS producer (the owned `ExportProducer` has no TS twin at all, also for png/svg/pdf), so their new `semio-project`/`semio-encode` stages are Rust-only like the existing stages.

## Index slip
I ran `git rm --cached` once on the svg TS twin and reverted it with `git reset` of that path. `git diff --cached -- <path>` is empty (index matches HEAD for it); the only difference is the unstaged working-tree change (file now `export {};`). Reported to the coordinator.

## Post-e2e debt: framework close_step -> RetainedWorkOutcome (deferred by coordinator)
Goal: `RetainedDiffApply::close_step`, `RetainedMutationClose*`, cold-mutation retained traits and the dsl/derive/retained derive return `RetainedWorkOutcome<RetainedCloneStep>` so layout-diff/validation/mutation ladders post `.work` instead of reading `.result`. Census of files with `close_step ... Result<RetainedCloneStep,ValueError>` signatures (overcount): framework products/os 114, replication 30, layout 48, energy 46, puzzle 19, stdio 14, draw 13, trinity 4, actor/io/pack 3 each, others ~20 (about 314 files). One atomic change after activation and e2e.

## TS runs (bun, no cargo)
- svg import document tests incl. the SVG round-trip on the TS SvgWriteJob: 64 pass, 0 fail.
- layout drawing planner corpus incl. the 7 new oracle cases (testDrawingPlanner vs oracle.mjs via 1105b-tools/offclosure/run_planner.ts): ok.
- TS SvgWriteJob and PdfWriteJob write tests: 10 pass each.
- Pending: offclosure-50 once runtime's W-runtime-chk4/chk5 report rc 0 (chk3 was still running).
