# result-lfix — port the editor and plugin fixture layers to the current typed-operation API

Executor `lfix`. No cargo run, no git write. Tools and logs: `lfix-tools/` (inputs) and `🗑️generated/lfix-*` (logs).

## 1. What changed

### 1.1 The current API in one paragraph

Every app-level fixture call now needs an original caller context: `dispatch_typed`, `handle_action` (now `args: &mut Option<DslValue>`, answers `Option`), `render` (answers `Option<()>`, tree read through `original_render_tree_source`), `maintenance_step(cx, grant)`, `close_step(cx, grant)`, `advance_typed_operation_publication(identity, cx)`, `ingest_operations`, `tick_backbone`, `poll_document_archive_load`, `import_media`, `export_media`/`produce_media`/`consume_media`, `seal_artifact_envelope_ingress`, `advance_artifact_envelope_load` and the `artifact_app_laws` helpers (`settle_registered_typed_operation`, `settle_history_verb`, `load_document`, `close_registered_fixture_app`, `assert_undo_redo_round_trip`, ...). Fixture constructors take `(mounted_policy, identity)` and `with_registry` needs `settle_store_constructor`.

### 1.2 Framework additions (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing`)

| Where | Addition |
| --- | --- |
| `🎟️caller/🦀️.rs` | `OriginalFixtureScope` (`admit`, `cx`, `retire`, `Drop`): the fixture-lifetime owner of one `OriginalFixtureCaller` and its single `StepContext<'static>`. Exported from `semio_framework_plugin_testing_caller`, the testing root and `artifact_app_laws`. |
| `🧪️artifact-app-laws/🔎️declared-verb-probing` | `observe_registered_fixture_app` (bounded render, closure over the built root), `render_registered_fixture_app_over` (snapshot override); `render_registered_fixture_app` delegates. |
| `🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading` | `dispatch_fixture_command` (addressed `handle_command` loop), `export_fixture_media` (batch `export_media` loop), `submit_/poll_/retain_/take_fixture_media_export*` (the receiver based media export protocol), `produce_fixture_media`. |
| `🧪️artifact-app-laws` re-exports | all of the above. |
| `🧪️testing/📦️packages/🦀️rust/Cargo.toml` | new path dependency `semio-framework-plugin-document-media` (for `MediaArtifact` in `produce_fixture_media`). Cargo.lock needs a refresh. |
| `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/📤️jobs` | `testing::maintain` and `maintain_items` gained a trailing `cx`. |

### 1.3 Port pattern

* Fixture wrapper (`struct XApp(pub VcsArtifactApp<..>, pub OriginalFixtureScope)`, `Deref`/`DerefMut` to field 0, `Drop` pays the app close then the scope retires). Calls read `app.0.method(.., app.1.cx())` so the two borrows stay disjoint.
* Raw (unwrapped) tests: one local `scope` per test, helper fns gain a trailing `cx` parameter.
* `render` now goes through `render_registered_fixture_app` (JSON projection) or `observe_registered_fixture_app` (walk the built root in place); owned `ComponentTree` helpers became projection helpers where needed (draw `rendered_drawing_canvas`, remodel/flow/cad render helpers).
* Error types follow the new answers: `OriginalTurnFailure`/`PluginLifecycleFailure` are projected to `Fault`/`String` with `primary_fault()` or the `Debug` form wherever a helper keeps its old signature.

### 1.4 Plugins ported (212 fixture files carry the new API)

demonstrator, wfc, sourcing, lowpoly, vcs, mathematical, process, imperative, note, shooting, architect, bim, sequence, energy, forms, block, space, dag, playbook, raster, gis (terrain, map), layout, puzzle (2d, 3d, 5d, jobs), norm (15 crates), flow, reasoning, writer, procedural (2d, 3d), fem (2d, 3d), cad, animate, remodel, trinity (rewriting), draw, stdio (24 artifact crates incl. media export, natural file import/save).

Per plugin the changed fixture files: stdio 24, norm 16, draw 13, procedural 12, puzzle 12, fem 11, raster 10, flow 8, cad 8, layout 8, writer 7, gis 7, forms 7, remodel 6, reasoning 5, animate 5, space 5, sourcing 5, note 5, process 4, block 4, dag 4, trinity 3, wfc 3, mathematical 3, playbook 3, lowpoly 3, sequence 2, architect 2, shooting 2, energy 2, demonstrator 1, vcs 1, imperative 1, bim 3 (see 3.1).

Besides the typed-operation port, these same files also got the small generation drifts that block compilation: `create_document_envelope` fifth argument (genesis producer, 12 call sites in process/cad/stdio docx), `new_app_with_registry*` / `with_registry*` / `create_app` policy and identity arguments, `dispatch_typed` two-argument calls (reasoning).

## 2. Verification (static only)

* `lfix-tools/scan/stale.py`, `stale2.py`, `stale3.py` (call arity scanners against the framework signatures): `stale.py --list ✏️s/🔌️plugins` and the two companions. Remaining hits are all verified false positives (own methods named `close_step`/`maintenance_step`/`render`, local `new_app()` helpers, UFCS calls that already carry `cx`, the `layout/🎞️intrinsic` orphan, bim see 3.1) except the puzzle wasm bridges (3.2).
* `lfix-tools/ports/*.py` are the idempotent, assert-guarded codemods (`wrapper.py` is the library). A re-run after a peer reverted a file re-applies the port (bim twice).
* Oracle (`posrun.py`, rustc nightly-2026-07-20 type/privacy differential, stubbed externals): see 2.1.

### 2.1 Oracle

Family `lfix3` (109 crates holding the changed fixtures, `--cfgs test`, two independent runs `a2` and `b1`, identical results; `sourcing-curation` excluded because it crashes the stub pass).

* 5285 errors after the second iteration (3015 stub-involved): E0432 1559, E0433 2128, E0185/E0220/E0050/E0252 and similar stub-layer rows. **No type-check class** (E0061/E0308/E0599/E0277 on non-stub code): rustc stops at name resolution, so the oracle never reaches argument-count or type checks. It is blind to the very signature changes this port is about; the arity scanners (section 2) carry that verification.
* Privacy: 1 non-stub row, `E0603 Puzzle3dInstanceOperationOwner` private import in `puzzle5d` (a mirror re-export of the puzzle3d session owner; not a file of this port). The earlier `a2` run also showed `E0624 FlowChildGroupJobFactory::new` is `pub(super)` called from `flow-unit-tests` line 133: the call is unchanged from HEAD (`git show HEAD:` has it), so it is pre-existing and not caused by this port.
* Resolve (2021 non-stub rows): classes are `crate` in nested use braces (harness artifact of the oracle's `__ext` rewrite), names generated by the stripped derives (`LayoutCommand`, `RasterCommand`, `Presentation*`, `Block2dLabels`, `Block5dLabels`, `ArchitectConfig`, ...), and `support`/`snapshot` module imports. A text search of the report for every name this port introduced (`OriginalFixtureScope`, `observe_registered_fixture_app`, `render_registered_fixture_app_over`, `dispatch_fixture_command`, `export_fixture_media`, `submit_fixture_media_export`, `produce_fixture_media`, `settle_registered_typed_operation`, `load_document`, `close_registered_fixture_app`) finds zero rows.
* There was no matching baseline for this family (`--no-base` / earlier families differ), so "new versus baseline" could not be computed; the claim above is the name search plus the class analysis.


## 3. Open items and risks

### 3.1 bim is contested

A peer (the bim unmirror/regeneration) rewrote 1794 bim files at 14:16 and reverted the fixture port twice. `ports/bim_editor.py` and `ports/bim_viewer.py` re-apply it idempotently (they assert the old text). Re-run them after the peer is done, then `stale.py --list ✏️s/🔌️plugins/🏙️bim` must print nothing.

### 3.2 Not done

1. Puzzle wasm bridges `🧩️puzzle/🗿️artifacts/🧊️3d/.../✏️editor/🌉️wasm/🦀️.rs` and `🖐️5d/.../🌉️wasm/🌉️bridge/🦀️.rs`: production wasm exports still call `maintenance_step`, `close_step`, `seal_artifact_envelope_ingress`, `advance_artifact_envelope_load` without a `cx`. They need a production source for the original context (host design decision), not a fixture scope.
2. Store disposer fixtures (`ArtifactOwnedDisposer::close_step(&mut owner, items, bytes)` and `PluginCloseStep`) in cad (2 files), gis (3), process3d retained laws, draw gesture-operation-owner: the disposer API moved to `close_step(owner, RetainedCloneGrant)` and `PluginLifecycleStep`; those tests assert grant semantics that must be re-authored, not mechanically ported.
3. `✏️s/🧑‍💻dev/**` (composition generation3d ~135 sites, sequence-runtime, trinity graph) sits two generations behind (`handle_action` with three arguments, `close_step(1, bytes)`); out of the `✏️s/🔌️plugins/**` scope, not touched.
4. `layout/🎞️intrinsic` orphan test, `bounded_document_store_owners` versus `funded_bounded_artifact_store_owners`, and the raster/stdio media owner sites other than the export protocol: unchanged.
5. Cargo.lock refresh for the new `document-media` dependency of the testing laws crate.

### 3.3 Risks the static pass cannot see

* One scope owns 32768 fuel and one retained stock for the whole test body (`OriginalFixtureCaller` policy). Very long loops (puzzle fill runs of up to 262144 host turns, stdio close loops of 10000 turns) may exhaust it and surface as "original fixture ... exhausted" faults; split such tests into several scopes if so.
* Media export (`submit_/poll_/retain_/take_fixture_media_export*`): the completed export keeps its download pending until the segmented download is published and drained; the mp3/avi/mp4/raster cancel and stream tests were ported without a way to run them.
* `OriginalFixtureScope` holds a raw pointer to the leaked caller (so it is `!Send`): scopes must be admitted on the thread that uses them (all ported fixtures do).
* The oracle stubs are blind to framework signature changes, so arity and type conversions were checked by the scanners and by reading the diffs of every codemod family, not by rustc.

## 4. Crates to compile (for cc)

Framework: `semio-framework-plugin-testing-caller`, `semio-framework-plugin-testing-laws`, `semio-s-plugin-puzzle-jobs`.

Plugin test targets (`bun repo cargo --store=shared ... -- test --no-run -p <crate>`), per plugin the crates holding the changed fixtures (nearest owning manifest; the unit-test companion crates `*-unit-tests`/`*-tests` of the same plugin are the ones that carry most of the changed files):

139 crates: the exact set (92, lexical `#[path]`/`include!` closure from each changed fixture file to its owning crate) unioned with the heuristic set (110, every crate whose tree holds a changed test or fixture file). 44 changed files could not be mapped lexically (modules pulled in by other mechanisms), hence the union. `semio-s-artifact-sourcing-curation` is listed but excluded from the oracle family (it crashes the stub pass). Compile with `--no-run` first; a failing crate that holds no changed file is a pre-existing break.

* animate (1): `semio-s-artifact-animate-presentation-animate`
* architect (1): `semio-s-artifact-architect-program-architect`
* bim (3): `semio-s-artifact-bim-model-bim`, `semio-s-artifact-bim-model-v1`, `semio-s-artifact-bim-model-viewer`
* block (4): `semio-s-artifact-block-2d-editor`, `semio-s-artifact-block-3d-editor`, `semio-s-artifact-block-3d-viewer`, `semio-s-artifact-block-5d-editor`
* cad (3): `semio-s-artifact-cad-cad-editor`, `semio-s-artifact-cad-cad-modes`, `semio-s-artifact-cad-cad-transform`
* dag (1): `semio-s-artifact-dag-dag-dag`
* demonstrator (1): `semio-s-artifact-demonstrator-playground-editor`
* draw (4): `semio-s-artifact-draw-drawing-editor`, `semio-s-artifact-draw-drawing-editor-tests`, `semio-s-artifact-draw-drawing-editor-tool`, `semio-s-artifact-draw-drawing-viewer`
* energy (1): `semio-s-artifact-energy-model-editor`
* fem (4): `semio-s-artifact-fem-2d-editor`, `semio-s-artifact-fem-2d-editor-commands`, `semio-s-artifact-fem-3d-editor`, `semio-s-artifact-fem-3d-editor-commands`
* flow (6): `semio-s-artifact-flow-flow-config`, `semio-s-artifact-flow-flow-editor`, `semio-s-artifact-flow-flow-original-command-retirement-tests`, `semio-s-artifact-flow-flow-unit-tests`, `semio-s-artifact-flow-flow-unit-tests-2`, `semio-s-artifact-flow-flow-viewer`
* forms (3): `semio-s-artifact-forms-forms-commands`, `semio-s-artifact-forms-forms-config-tests`, `semio-s-artifact-forms-forms-forms`
* gis (6): `semio-s-artifact-gis-gismap`, `semio-s-artifact-gis-gismap-config`, `semio-s-artifact-gis-gismap-gis2d`, `semio-s-artifact-gis-gismap-gis2d-unit-tests`, `semio-s-artifact-gis-gismap-gismap`, `semio-s-artifact-gis-gisterrain-editor`
* imperative (1): `semio-s-artifact-imperative-procedure-editor`
* layout (4): `semio-s-artifact-layout-layout-editor`, `semio-s-artifact-layout-layout-editor-commands`, `semio-s-artifact-layout-layout-editor-commands-tests`, `semio-s-artifact-layout-layout-editor-panels`
* lowpoly (2): `semio-s-artifact-lowpoly-lowpoly-commands`, `semio-s-artifact-lowpoly-lowpoly-editor`
* mathematical (1): `semio-s-artifact-mathematical-equation-editor`
* norm (15): `semio-s-artifact-norm-din16798-editor`, `semio-s-artifact-norm-din18599-editor`, `semio-s-artifact-norm-din4108-editor`, `semio-s-artifact-norm-en1990-editor`, `semio-s-artifact-norm-en1991-editor`, `semio-s-artifact-norm-en1992-editor`, `semio-s-artifact-norm-en1993-editor`, `semio-s-artifact-norm-en1994-editor`, `semio-s-artifact-norm-en1995-editor`, `semio-s-artifact-norm-en1996-editor`, `semio-s-artifact-norm-en1997-editor`, `semio-s-artifact-norm-en1998-editor`, `semio-s-artifact-norm-en1999-editor`, `semio-s-artifact-norm-iso16757-editor`, `semio-s-artifact-norm-vdi3805-editor`
* note (2): `semio-s-artifact-note-note-mut-family`, `semio-s-artifact-note-note-unit-tests`
* playbook (1): `semio-s-artifact-playbook-playbook-v1`
* plugin (1): `semio-s-plugin-playbook-procedural`
* procedural (6): `semio-s-artifact-procedural-2d-editor`, `semio-s-artifact-procedural-2d-node-graph-edit`, `semio-s-artifact-procedural-2d-viewer`, `semio-s-artifact-procedural-3d-editor`, `semio-s-artifact-procedural-3d-node-graph-edit`, `semio-s-artifact-procedural-generation3d`
* process (1): `semio-s-artifact-process-process3d-editor`
* puzzle (7): `semio-s-artifact-puzzle-2d-history-edit-runtime-tests`, `semio-s-artifact-puzzle-2d-unit-tests`, `semio-s-artifact-puzzle-2d-v1`, `semio-s-artifact-puzzle-3d-editor`, `semio-s-artifact-puzzle-5d-host-admission-tests`, `semio-s-artifact-puzzle-5d-puzzle5d`, `semio-s-artifact-puzzle-5d-v1-unit-tests`
* raster (3): `semio-s-artifact-raster-raster-editor`, `semio-s-artifact-raster-raster-editor-commands`, `semio-s-artifact-raster-raster-editor-tests`
* reasoning (2): `semio-s-artifact-reasoning-wires-window-transient`, `semio-s-artifact-reasoning-wires-wires`
* remodel (3): `semio-s-artifact-remodel-remodeling-editor`, `semio-s-artifact-remodel-remodeling-examples`, `semio-s-artifact-remodel-remodeling-modes`
* sequence (2): `semio-s-artifact-sequence-sequence`, `semio-s-artifact-sequence-sequence-v1`
* shooting (1): `semio-s-artifact-shooting-shooting-shooting`
* sourcing (1): `semio-s-artifact-sourcing-curation`
* space (4): `semio-s-artifact-space-home`, `semio-s-artifact-space-home-kinds-conformance`, `semio-s-artifact-space-home-viewer`, `semio-s-artifact-space-space-editor`
* stdio (33): `semio-s-artifact-stdio-avi`, `semio-s-artifact-stdio-avi-tests`, `semio-s-artifact-stdio-bcf-io-registry`, `semio-s-artifact-stdio-csv-io-registry`, `semio-s-artifact-stdio-docx-base`, `semio-s-artifact-stdio-epw-editor`, `semio-s-artifact-stdio-html`, `semio-s-artifact-stdio-html-mutations`, `semio-s-artifact-stdio-jpg`, `semio-s-artifact-stdio-jpg-io-registry`, `semio-s-artifact-stdio-json`, `semio-s-artifact-stdio-json-mutations-2`, `semio-s-artifact-stdio-md`, `semio-s-artifact-stdio-md-tests`, `semio-s-artifact-stdio-mp3-editor`, `semio-s-artifact-stdio-mp4`, `semio-s-artifact-stdio-mp4-viewer`, `semio-s-artifact-stdio-pdf-mut-base-4`, `semio-s-artifact-stdio-pdf-pdf-snapshot-value-tests`, `semio-s-artifact-stdio-png`, `semio-s-artifact-stdio-png-tests`, `semio-s-artifact-stdio-pptx-base`, `semio-s-artifact-stdio-semio-brep`, `semio-s-artifact-stdio-semio-brep-io`, `semio-s-artifact-stdio-semio-mesh-editor`, `semio-s-artifact-stdio-tsv`, `semio-s-artifact-stdio-tsv-mutations`, `semio-s-artifact-stdio-txt`, `semio-s-artifact-stdio-txt-tests`, `semio-s-artifact-stdio-wav-io-registry`, `semio-s-artifact-stdio-xml`, `semio-s-artifact-stdio-xml-tests-2`, `semio-s-artifact-stdio-zip-editor`
* trinity (2): `semio-s-artifact-trinity-rewriting-editor`, `semio-s-artifact-trinity-rewriting-v1`
* vcs (2): `semio-s-artifact-vcs-vcs-editor`, `semio-s-artifact-vcs-vcs-editor-unit-tests`
* wfc (4): `semio-s-artifact-wfc-2d-v1`, `semio-s-artifact-wfc-3d-wfc3d`, `semio-s-artifact-wfc-bitmap-bitmap`, `semio-s-artifact-wfc-bitmap-tests`
* writer (3): `semio-s-artifact-writer-writer`, `semio-s-artifact-writer-writer-editor`, `semio-s-artifact-writer-writer-v1`

