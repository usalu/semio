STATUS: PLAN CURRENT as of 14:36 Nov 2 (189 patch files scanned, regenerate: `python3 1102-tools/winplan/gen.py`); 136 of 136 pending diffs apply in sequence on a scratch copy (0 fail); compile UNVERIFIED (freeze).

# 1102 Patch Window Plan (lane winplan)

Tools: `1102-tools/winplan/{scan.py,seq.sh,gen.py,order.txt,notes.json}`. Raw: `🗑️generated/1102/winplan-scan.json`, `winplan-seq.txt`. Paths below are relative to C; patch paths are `a/ b/` from the repo root unless noted (apply with `cd /Users/ueli/Documents/semio && git apply <patch>`; the 1101 `admission` diff needs `-p0` from `🧰️framework`).

## 0. Verdict

- Pending diffs on the current tree: policy 57, cadi18n 22, uii18n 6, testkit 1, gen-brep 1 (1101). The set of 136 diffs applies cleanly **in the order of section 3** on a scratch copy of the touched files (`winplan-seq.txt`, 136 OK / 0 FAIL).
- TWINS: lane phase (37 patches, own order in `phase/patches/order.txt`) supersedes 4 policy patches (`supersedes.txt`); those policy twins are SKIPPED. Phase batches B1P/B5P follow policy B2/B5.
- ONE ordering trap: `cadi18n/cad-mode-edit.patch` FAILS alone on the current tree (`git apply --check`, hunk @539); it only applies after `uii18n/cad-step-labels-1` and `-2`. Apply uii18n before cadi18n, never the reverse (`cadi18n/verify.sh` already assumes this).
- ONE duplicate: `rtpanes/laws-publish-framework-schema-documents.md` is superseded by `testkit/publish-framework-schemas.patch` (same file, same fix). Apply only the testkit patch; the rtpanes md is DO NOT APPLY.
- Obsolete/already applied: see the inventory state column. `rtpanes/by-example-law-redo.md` is already in the tree (`redo must reapply` present in `🪧️pane-smoke-law/🦀️.rs`).
- Script patches are fail-closed (assert exactly one anchor) and run on a scratch copy of their target OK: `testport/cursor-dev-vcs-testkit.py`, `flowext/directed-dag-facade-exports.py`; `stdio/decouple-mesh-export-from-pdf.py` dry-run plans 17 edits (default is dry-run, `--apply` writes).

## 1. Inventory

State: PENDING = not in tree, applicable now; APPLIED/OBSOLETE/SUPERSEDED = do not apply. Batch refers to section 3.

| gen | lane | patch (under `<gen>-tools/<lane>/patches/`) | kind | targets | state | batch | note |
|---|---|---|---|---|---|---|---|
| 1102 | actinfra | `emitter-request-bytes.diff` | diff | 1 | PENDING (check clean) | B0 | seq OK |
| 1102 | actinfra | `emitter-request-bytes.test.ts` | other | 1 | TEST FILE (add any time) | B0 | test for `emitter-request-bytes.diff`; copy to the describe emitter contract `🧪️tests/` dir named in its header (relative imports ../../📋️contract); validator plus Ajv oracle; fails on the unpatched contract by design |
| 1102 | actinfra | `retention-intermediates.diff` | diff | 1 | PENDING (check clean) | B0 | seq OK |
| 1102 | actinfra | `retention-size-law.test.ts` | other | 1 | TEST FILE (add any time) | B0 | new test, goes to `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧾️receipt/📦️retention/🧪️tests/🪶️size-law/🟦️.ts`; under a tests dir so it may be copied even during the freeze; fails on the unpatched module by design |
| 1102 | actinfra | `worker-spare-instances.diff` | diff | 2 | PENDING (check clean) | B0 | seq OK |
| 1102 | cadi18n | `cad-editor-callers.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `cad-engine-0.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `cad-engine-1.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `cad-engine-2.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `cad-mode-edit.patch` | diff | 1 | PENDING (alone: FAIL, needs earlier batch) | B8 | seq OK; see section 4 |
| 1102 | cadi18n | `cad-mode-window-building.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `cad-mode-window-energy.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `cad-mode-window-shape.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `cad-mode-window-structure.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `cad-spec-json-1.patch` | diff | 60 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `cad-spec-rust-0.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `cad-spec-rust-1.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `cad-test-languages.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `cad-tests-1.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `cad-tests-2.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `cad-tests-3.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `cad-tests-4.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `name-derive-test.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `name-derive.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `name-dwg-test.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `name-geometry-1.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `name-geometry-2.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `name-history-test.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `name-import.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `name-kit.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `name-panels-artifact-test.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `name-panels-artifact.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `name-panels-inspection-test.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `name-panels-inspection.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `name-picking-test.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `name-sample-scene.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `name-terminology.patch` | diff | 1 | PENDING (check clean) | B8 | seq OK |
| 1102 | cadi18n | `web-acceptance-1.patch` | diff | 1 | PENDING (check clean) | B10 | seq OK |
| 1102 | cadi18n | `web-demonstrator-1.patch` | diff | 1 | PENDING (check clean) | B10 | seq OK |
| 1102 | cadi18n | `web-shellhost-0.patch` | diff | 1 | PENDING (check clean) | B10 | seq OK |
| 1102 | cadi18n | `web-shellhost-1.patch` | diff | 1 | PENDING (check clean) | B10 | seq OK |
| 1102 | cadi18n | `web-shellhost-2.patch` | diff | 1 | PENDING (check clean) | B10 | seq OK |
| 1102 | describe | `step-geometry-dev-dep.md` | plan | 1 | PENDING (hand edit) | B6 | add `semio-framework-async-macros = { workspace = true }` to step geometry `[dev-dependencies]` (and step workspace `[workspace.dependencies]` if absent); currently 0 hits |
| 1102 | flowext | `directed-dag-facade-exports.py` | script | 1 | PENDING (script, fail-closed) | B4 | adds 2 re-exports to `♾️infinite/📦️packages/🦀️rust/🦀️.rs`; ran OK on a scratch copy; unblocks flow-host unit tests |
| 1102 | phase | `order.txt` | other | ? | ORDER FILE (not a patch) | - | phase's own apply order / supersedes list; this plan follows them |
| 1102 | phase | `semio-framework-actor-retained-turn.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-job-failure.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-job-payload.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-job.patch` | diff | 6 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-os-flow-extensions.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-os-flow-wasm.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-os-history.patch` | diff | 2 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-os-renderer-wgpu.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-os-store-document.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-os-store-engine-replay.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-os-store-initialization-replay.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-os-store-replay-operation.patch` | diff | 4 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-pack-record-retained.patch` | diff | 2 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-plugin-decl-app.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-plugin-decl-emission.patch` | diff | 2 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-plugin-job-receiving.patch` | diff | 3 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-plugin-job.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-plugin-tool-run-turns.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-plugin-window-config.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-plugin.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-replication-history.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-text-splice-native.patch` | diff | 2 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-value-allocation-return.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-value-clone-source.patch` | diff | 2 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-value-control-contract.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-value-derive-expansion.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-value-derive.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-value-edit.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-value-native-owned.patch` | diff | 3 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-value-ordered.patch` | diff | 3 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-value-paged-list.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-value-persistent.patch` | diff | 3 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-framework-value-retained-map.patch` | diff | 1 | PENDING (check clean) | B1P | seq OK |
| 1102 | phase | `semio-s-artifact-draw-drawing-scene.patch` | diff | 1 | PENDING (check clean) | B5P | seq OK |
| 1102 | phase | `semio-s-artifact-fem-3d-editor-results-window.patch` | diff | 1 | PENDING (check clean) | B5P | seq OK |
| 1102 | phase | `supersedes.txt` | other | ? | ORDER FILE (not a patch) | - | phase's own apply order / supersedes list; this plan follows them |
| 1102 | policy | `semio-framework-2d-path-flatten.patch` | diff | 1 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-actor-retained-turn.patch` | diff | 2 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-actor.patch` | diff | 2 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-artifact-flow-flow-copy.patch` | diff | 1 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-dispatch-macros.patch` | diff | 8 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-job-context.patch` | diff | 1 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-job.patch` | diff | 2 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-os-flow-host.patch` | diff | 2 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-os-kernel.patch` | diff | 2 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-os-services-native-io.patch` | diff | 1 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-os-services-native-paths.patch` | diff | 1 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-os-store-codec-registry.patch` | diff | 1 | PENDING (check clean) | B2 | seq OK |
| 1102 | policy | `semio-framework-os-store-composition.patch` | diff | 1 | PENDING (check clean) | B2 | seq OK |
| 1102 | policy | `semio-framework-os-store-engine-branch.patch` | diff | 1 | PENDING (check clean) | B2 | seq OK |
| 1102 | policy | `semio-framework-os-store-engine-command.patch` | diff | 1 | PENDING (check clean) | B2 | seq OK |
| 1102 | policy | `semio-framework-os-store-engine-replay.patch` | diff | 4 | PENDING (check clean) | B2 | seq OK |
| 1102 | policy | `semio-framework-os-store-ephemeral.patch` | diff | 2 | PENDING (check clean) | B2 | seq OK |
| 1102 | policy | `semio-framework-os-store-one-item-batch.patch` | diff | 1 | PENDING (check clean) | B2 | seq OK |
| 1102 | policy | `semio-framework-os-store-owned-schema-decode.patch` | diff | 1 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-os-store-owned-schema-fields.patch` | diff | 3 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-os-store-presence.patch` | diff | 2 | PENDING (check clean) | B2 | seq OK |
| 1102 | policy | `semio-framework-os-store-read.patch` | diff | 2 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-os-store-replay-operation.patch` | diff | 4 | SUPERSEDED by phase twin | - | skip; phase `semio-framework-os-store-replay-operation.patch` replaces it (phase supersedes.txt) |
| 1102 | policy | `semio-framework-os-store-replay.patch` | diff | 2 | PENDING (check clean) | B2 | seq OK |
| 1102 | policy | `semio-framework-pack-record-retained.patch` | diff | 1 | SUPERSEDED by phase twin | - | skip; phase `semio-framework-pack-record-retained.patch` replaces it (phase supersedes.txt) |
| 1102 | policy | `semio-framework-pixels.patch` | diff | 1 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-plugin-decl-emission.patch` | diff | 1 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-plugin-decl-registry.patch` | diff | 1 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-plugin-instance-jobs.patch` | diff | 1 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-plugin-instance-typed-ops.patch` | diff | 1 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-plugin-job-receiving.patch` | diff | 3 | SUPERSEDED by phase twin | - | skip; phase `semio-framework-plugin-job-receiving.patch` replaces it (phase supersedes.txt) |
| 1102 | policy | `semio-framework-plugin-job.patch` | diff | 2 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-plugin-reactor-core.patch` | diff | 2 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-plugin-reactor-turn.patch` | diff | 1 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-plugin-retained-command.patch` | diff | 1 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-plugin-tool-run.patch` | diff | 1 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-plugin-window-config.patch` | diff | 1 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-plugin.patch` | diff | 3 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-replication-history.patch` | diff | 1 | SUPERSEDED by phase twin | - | skip; phase `semio-framework-replication-history.patch` replaces it (phase supersedes.txt) |
| 1102 | policy | `semio-framework-value-derive.patch` | diff | 1 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-value-native-storage.patch` | diff | 1 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-framework-value-retained-map.patch` | diff | 1 | PENDING (check clean) | B1 | seq OK |
| 1102 | policy | `semio-s-artifact-draw-drawing-io.patch` | diff | 1 | PENDING (check clean) | B5 | seq OK |
| 1102 | policy | `semio-s-artifact-draw-drawing-scene-paint.patch` | diff | 2 | PENDING (check clean) | B5 | seq OK |
| 1102 | policy | `semio-s-artifact-draw-drawing-scene.patch` | diff | 1 | PENDING (check clean) | B5 | seq OK |
| 1102 | policy | `semio-s-artifact-energy-model-session.patch` | diff | 2 | PENDING (check clean) | B5 | seq OK |
| 1102 | policy | `semio-s-artifact-fem-2d-editor-session.patch` | diff | 1 | PENDING (check clean) | B5 | seq OK |
| 1102 | policy | `semio-s-artifact-gis-gismap-gis2d.patch` | diff | 1 | PENDING (check clean) | B5 | seq OK |
| 1102 | policy | `semio-s-artifact-puzzle-2d-mut-any.patch` | diff | 1 | PENDING (check clean) | B5 | seq OK |
| 1102 | policy | `semio-s-artifact-puzzle-2d-snapshot.patch` | diff | 1 | PENDING (check clean) | B5 | seq OK |
| 1102 | policy | `semio-s-artifact-puzzle-2d-text-inverse.patch` | diff | 1 | PENDING (check clean) | B5 | seq OK |
| 1102 | policy | `semio-s-artifact-stdio-ifc-2x3-base-details.patch` | diff | 2 | PENDING (check clean) | B6 | seq OK |
| 1102 | policy | `semio-s-artifact-stdio-ifc-4x3.patch` | diff | 1 | PENDING (check clean) | B6 | seq OK |
| 1102 | policy | `semio-s-artifact-stdio-ifc-details-guidance.patch` | diff | 1 | PENDING (check clean) | B6 | seq OK |
| 1102 | policy | `semio-s-artifact-stdio-ifc-details-provider.patch` | diff | 1 | PENDING (check clean) | B6 | seq OK |
| 1102 | policy | `semio-s-plugin-flow-brep.patch` | diff | 1 | PENDING (check clean) | B5 | seq OK |
| 1102 | policy | `ts-framework-process-wasm-build.patch` | diff | 1 | PENDING (check clean) | B9 | seq OK |
| 1102 | rtpanes | `by-example-law-redo.md` | plan | 1 | APPLIED | - | redo step already in `🪧️pane-smoke-law/🦀️.rs` (`redo must reapply` x1) |
| 1102 | rtpanes | `energy-manifest-uses-stdio-semio-edit-mode.md` | plan | 1 | PENDING (hand edit, 1 line) | B5 | energy editor `📚️examples-and-manifest/🦀️.rs:4`: `use semio_s_artifact_stdio_semio::editor::modes::edit;` -> `use crate::editor_edit as edit;`; production fault (energy pane panics at build_definition: undeclared window kind framework.window.mesh); needed for the energie pane boot |
| 1102 | rtpanes | `laws-publish-framework-schema-documents.md` | plan | 1 | SUPERSEDED (do not apply) | - | same file and same fix as `testkit/publish-framework-schemas.patch`; applying both would publish twice and double the helper |
| 1102 | runtime | `apply-debug-settle-cause.py` | script | 1 | OPTIONAL [DEBUG] diagnostic | - | inserts an `eprintln!("[DEBUG] settle stop ...")` into the SAME laws file as the testkit patch (`🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs`, different line); apply only if runtime asks, after B3, and remove again before ticket close. Not part of the plan order |
| 1102 | stdio | `decouple-mesh-export-from-pdf.py` | script | 8 | PENDING (script, dry-run default) | B6 | new crate + 8 manifests (17 edits planned); `--apply` writes. Last in B6; adds crate `semio-s-artifact-stdio-semio-drawing-png-export` |
| 1102 | testkit | `publish-framework-schemas.patch` | diff | 2 | PENDING (check clean) | B3 | seq OK |
| 1102 | testport | `cursor-dev-vcs-testkit.py` | script | 1 | PENDING (script, fail-closed) | B4 | adds vcs `testkit` dev-dep to store-cursor Cargo.toml; ran OK on a scratch copy |
| 1102 | uii18n | `cad-step-labels-1.patch` | diff | 1 | PENDING (check clean) | B7 | seq OK |
| 1102 | uii18n | `cad-step-labels-2.patch` | diff | 1 | PENDING (check clean) | B7 | seq OK |
| 1102 | uii18n | `flow-form-labels-1.patch` | diff | 1 | PENDING (check clean) | B7 | seq OK |
| 1102 | uii18n | `flow-form-labels-2.patch` | diff | 1 | PENDING (check clean) | B7 | seq OK |
| 1102 | uii18n | `procedural-docstring-1.patch` | diff | 1 | PENDING (check clean) | B7 | seq OK |
| 1102 | uii18n | `process-status-steps-1.patch` | diff | 1 | PENDING (check clean) | B7 | seq OK |
| 1101 | actinfra | `component-scheduler-flag-views.patch` | diff | 1 | OBSOLETE | - | made from /tmp/sa-dep.ts; replaced by `requireCompatibleCargoFlagViewsV1` already in the tree; never apply |
| 1101 | closure | `closure-manifest-deps.patch` | diff | 32 | APPLIED | - | ~40 Cargo.toml hunks all present (`git apply -R --check` passes) |
| 1101 | compute | `trapped-guest-test.md` | plan | ? | TEST-ONLY PLAN | - | two stale-test layers, 18 tests still failing; needs shard-owner kernel-driving test rewrite, no production edit; allowed any time (tests dirs). Not part of the window |
| 1101 | describe | `APPLY-AFTER-FREEZE.md` | plan | 1 | PARTIAL | B6 | items 1,2,5 (test dirs) applied; item 4 pending = same edit as `1102 describe/step-geometry-dev-dep.md` |
| 1101 | energy | `bridge-construction-recipient.py` | script | 1 | APPLIED | - | `StepRecipient` x3 in energy snapshot bridge |
| 1101 | fem | `PLAN.md` | plan | ? | APPLIED | - | fem result: PLAN items applied (fem-31/33 wasm --lib GREEN) |
| 1101 | flowext | `geometry-close-ticket.py` | script | 1 | APPLIED | - | prefix `semio_framework_value::geometry_close_ticket(` gone |
| 1101 | flowext | `retirement-tests.md` | plan | 3 | PARTIAL | - | `cfg(any(test,feature="testkit"))` + retirement-tests Cargo.toml applied; item 2 (window-config test loop) is a test-dir design task, item 3 optional; nothing to batch |
| 1101 | gen | `brep-boolean-funded-phases.patch` | diff | 5 | PENDING (check clean) | B5 | seq OK |
| 1101 | mutpolicy | `gen-host-displaced.py` | script | 1 | PENDING (script) | B5 | generation3d host `close_step(grant).expect(` -> `.result.expect(`; anchor count 1 now |
| 1101 | mutpolicy | `raster-model.py` | script | 2 | PENDING (script, asserts) | B5 | raster-model 3 sites; all anchors still old (1 hit each) |
| 1101 | rtpanes | `process3d-pane-smoke-target.md` | plan | 1 | PENDING (hand edit) | B5 | append `[[test]] process3d_pane_smoke` to process3d editor Cargo.toml (0 hits now); not on the activation path; low priority |
| 1101 | runtime | `apply-ack-branch-receipt.py` | script | 2 | APPLIED | - | per 1101 audit: `worker_outcome_pending` branch present |
| 1101 | runtime | `apply-lifecycle-reserved-return.py` | script | 1 | APPLIED | - | per 1101 audit: `reserved_before`/`original_lifecycle_net_after` present |
| 1101 | stdio3 | `decode-denied-close-grant.patch` | diff | 1 | APPLIED | - | path in patch is relative to `🌱️value/🛬️decode`; one-line guard present at line 60; `git apply --check` fails only on the path form |
| 1101 | testport | `README-dbg.txt` | other | ? | NOT A PATCH | - | debug note |
| 1101 | testport | `cold-reserve-output.py` | script | 1 | APPLIED | - | `None if grant==Self::cold_authority_grant()` present (testport result confirms); policy `os-store-replay` patch still checks clean over it |
| 1101 | testport2 | `instance-render-tool-clock-feature.md` | plan | 2 | PENDING (hand edit) | B4 | render Cargo.toml line 25 `artifact-app-testing = []` -> add `semio-framework-tool-machine/artifact-app-testing`; plus cfg change in `📥️staging/🛠️tool-machine/🦀️.rs:60`; author offers an alternative, needs a decision |
| 1101 | testport2 | `reactor-jobs-cargo-protocol-laws.md` | plan | 1 | PENDING (hand edit) | B4 | reactor jobs Cargo.toml line 54: features ["mutation-testing"] -> ["mutation-testing","protocol-laws"]; tree still old |
| 1101 | testport2 | `time-travel-instance-jobs-lib-test-cycle.md` | plan | 3 | HOLD (design) | - | gating of test helpers/modules across `♻️retirement/🧪️tests` and instance staging; no exact edit given; UNVERIFIED if applied; do not apply in the window |
| 1101 | ts2d | `admission-within-workspace.diff` | diff | 1 | APPLIED | - | `mkdir` import + `.cache/test-admission` present; diff paths are -p0 relative to `🧰️framework`, so the repo-root check fails on path only |
| 1101 | ts2d | `apply.py` | script | 7 | PENDING (script) | B11 | copies the 3 files then strips `booleanRegions, `/`BooleanOptions, ` from two index files (anchors still present, 1 + 2 hits); NOT fail-closed, overwrites files |
| 1101 | ts2d | `booleans.test.ts` | other | 1 | PENDING (file copy) | B11 | whole-file replacement driven by `apply.py`; tree copy still the old UnitRetirement version |
| 1101 | ts2d | `booleans.ts` | other | 1 | PENDING (file copy) | B11 | whole-file replacement driven by `apply.py`; tree copy still the old UnitRetirement version |
| 1101 | ts2d | `paths.ts` | other | 1 | PENDING (file copy) | B11 | whole-file replacement driven by `apply.py`; tree copy still the old UnitRetirement version |

`policy/patches/_superseded/` (13 files incl. `.check` markers excluded): old-name copies of the 17 renamed policy patches and `unknown.patch` (163 KB); never apply.

## 2. File-overlap matrix

Diff x diff (same target file in more than one pending patch):

| target (tail) | patches | hunks (old line ranges) | overlap |
|---|---|---|---|
| `…ts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs` | cadi18n/cad-mode-edit.patch, uii18n/cad-step-labels-2.patch | cadi18n/cad-mode-edit.patch [[36, 43], [50, 57], [157, 164], [485, 492], [496, 503], [539, 546], [549, 556], [559, 572]]; uii18n/cad-step-labels-2.patch [[521, 527], [547, 554]] | HUNKS OVERLAP |
| `…️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/⏪️history-and-convergence/🦀️.rs` | cadi18n/cad-tests-2.patch, cadi18n/name-history-test.patch | cadi18n/cad-tests-2.patch [[133, 140]]; cadi18n/name-history-test.patch [[259, 266]] | files shared, hunks disjoint |
| `…cts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs` | cadi18n/name-terminology.patch, uii18n/cad-step-labels-1.patch | cadi18n/name-terminology.patch [[3, 11], [26, 32], [75, 82]]; uii18n/cad-step-labels-1.patch [[53, 59]] | files shared, hunks disjoint |

Other collisions (script/plan vs diff, same crate or same file, from reading the targets):

- `cadi18n/cad-mode-edit.patch` x `uii18n/cad-step-labels-2.patch`: same file `…/✏️editor/🎭️modes/✏️edit/🦀️.rs`; hunk ranges OVERLAP (cadi18n 549-556 vs uii18n 547-554; cadi18n's @539 context carries uii18n's edited lines), so ORDER IS MANDATORY: uii18n first. Alone, cadi18n fails.
- `uii18n/cad-step-labels-1.patch` and `flow-form-labels-1.patch` both edit a `✏️editor/🗣️terminology/🦀️.rs`, in different artifacts (cad vs flow): different files, no collision.
- `testkit/publish-framework-schemas.patch` x superseded `rtpanes/laws-publish-...md`: same file `🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs`, same fix; testkit wins.
- `policy/semio-framework-os-store-replay.patch` x applied `testport/cold-reserve-output.py`: same file `⏩️replay-owned-state-operations/🦀️.rs`; the script is already in the tree and the policy diff checks clean over it (hunks 58-78, 380-402, 575-582 vs the script's cold-reserve region).
- `policy/semio-s-artifact-stdio-ifc-*` x `stdio/decouple-mesh-export-from-pdf.py`: different crates (ifc vs semio drawing/mesh); no shared file. Both are stdio-lane Cargo/manifests-adjacent but disjoint.
- `ts2d/apply.py` x `policy/ts-framework-process-wasm-build.patch`: both TS under `🧰️framework`, different modules (`◻️2d` vs `🏃️process/📦️artifacts/🕸️wasm-build`): no collision.
- Closure-applied Cargo.toml files (sourcing, gis, sequence, process, stdio, procedural/generation3d) are already in the tree; no pending patch edits the same manifest lines except `stdio/decouple` (adds deps to semio drawing/mesh manifests) and `cadi18n/cad-engine-0` + `cad-spec-rust-0` (cad manifests, `semio-framework-ui-locale` dep): re-run `git apply --check` right before those if a Cargo.lock regen happened.

## 3. Apply order (batches, single owner each)

Rule: apply patches of a batch one by one with `git apply`, then run the narrow check once for the batch. Never run two batches at once (one cargo per owner, slot.sh). If a check fails, bisect inside the batch in the listed order.

### B0 repo tooling: act18 blockers (emitter request bytes, Cargo retention, worker instances) (owner: `actinfra`)
Patches, in order:
1. `1102-tools/actinfra/patches/emitter-request-bytes.diff` (OK)
1. `1102-tools/actinfra/patches/retention-intermediates.diff` (OK)
1. `1102-tools/actinfra/patches/worker-spare-instances.diff` (OK)
1. copy `1102-tools/actinfra/patches/emitter-request-bytes.test.ts` into the emitter contract tests dir (new file)
1. copy `1102-tools/actinfra/patches/retention-size-law.test.ts` to `…/📦️retention/🧪️tests/🪶️size-law/🟦️.ts` (new file)
1. update the pinned worker-flag fixtures (`🏗️bazel/👷️workers/🧪️tests/🧫️fixtures/🔣️.json`, `🏗️bazel/🏃️execution/🧪️tests/👷️worker/🟦️.ts`) in the same window
- Check: `bun test ./<emitter contract tests>` (emitter-request-bytes test), `bun test ./<retention>/🧪️tests` (size-law), the worker tests/fixtures named in the actinfra result (`🏗️bazel/👷️workers/🧪️tests`, `🏗️bazel/🏃️execution/🧪️tests/👷️worker`); then the coordinator restarts activation. The 11 other `maximumOutputBytes+maximumManifestBytes` request sites are NOT covered (author: not tested).
- Why here: ACTIVATION BLOCKER FIRST: act18 ended `exit 130` at 14:35:50 after the flow-math component failed deterministically for every component (`Actual closed artifact reader custody required`: `descriptorEmitterRequestV1` asks 2149580800 bytes, the reader schema allows 2147483648). `emitter-request-bytes.diff` fixes that; `retention-intermediates.diff` shrinks the emitter tree 1.1 GB -> ~15 MB (1 GiB nx cache-put cap); `worker-spare-instances.diff` rebalances Bazel worker instances (RepoBuild=3) and needs fixture updates in the same window plus a Bazel server restart to take effect. Independent of every Rust patch. Diffs 1 and 2 are -p0 (no a/ b/): `git apply -p0 <diff>` from the repo root.

### B1 framework core: value, pack, pixels, job, plugin (owner: `policy`)
Patches, in order:
1. `1102-tools/policy/patches/semio-framework-value-derive.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-value-native-storage.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-pixels.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-2d-path-flatten.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-job.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-dispatch-macros.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-actor.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-actor-retained-turn.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-artifact-flow-flow-copy.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-plugin-job.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-plugin-reactor-core.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-plugin-reactor-turn.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-plugin-decl-registry.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-plugin.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-plugin-window-config.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-job-context.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-os-flow-host.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-os-kernel.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-os-store-read.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-plugin-decl-emission.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-plugin-instance-jobs.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-plugin-instance-typed-ops.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-plugin-retained-command.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-plugin-tool-run.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-value-retained-map.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-os-services-native-io.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-os-services-native-paths.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-os-store-owned-schema-decode.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-os-store-owned-schema-fields.patch` (OK)
- Check: `repo cargo ... check --manifest-path 🌎️hub/Cargo.toml --keep-going -p <the 13 crates of this batch>` native, then the same with `--target wasm32-wasip2` for `semio-framework-plugin`, `-plugin-job`, `-plugin-job-receiving`, `-value-native-storage`, `-job`
- Why here: Leaf-to-root by crate layer: value storage, pack, pixels, 2d flatten, job, flow copy, plugin job, reactor, plugin. Every edit replaces unwrap/expect with `ValueError::invariant` (exists from 1101); no signature changes, so order within the batch is by layer only.

### B2 framework os-store family (owner: `policy`)
Patches, in order:
1. `1102-tools/policy/patches/semio-framework-os-store-codec-registry.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-os-store-composition.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-os-store-ephemeral.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-os-store-one-item-batch.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-os-store-presence.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-os-store-replay.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-os-store-engine-branch.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-os-store-engine-command.patch` (OK)
1. `1102-tools/policy/patches/semio-framework-os-store-engine-replay.patch` (OK)
- Check: `check -p semio-framework-os-store-replay -p semio-framework-os-store-replay-operation -p semio-framework-os-store-engine-replay -p ...-engine-command -p ...-engine-branch -p ...-presence -p ...-ephemeral -p ...-one-item-batch -p ...-composition -p ...-codec-registry` native, then `describe-1` equivalent (describe Rust closure) as the regression gate
- Why here: Depends on B1 only for crates it imports. `replay` and `replay-operation` first: they introduce the `VcsError::{InverseRefused,ValidationFailed,HistoryReplaying}` handling (Option::filter) that the engine crates call.

### B1P framework phase-machine fixes (lane phase), AFTER policy B1/B2 (owner: `phase`)
Patches, in order:
1. `1102-tools/phase/patches/semio-framework-value-control-contract.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-value-allocation-return.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-value-native-owned.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-value-persistent.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-value-derive-expansion.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-value-derive.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-value-edit.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-value-paged-list.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-value-ordered.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-value-clone-source.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-value-retained-map.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-pack-record-retained.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-actor-retained-turn.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-replication-history.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-text-splice-native.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-job.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-job-failure.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-job-payload.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-os-flow-wasm.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-os-flow-extensions.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-os-store-initialization-replay.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-os-store-replay-operation.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-os-store-engine-replay.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-os-store-document.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-os-history.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-os-renderer-wgpu.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-plugin.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-plugin-job.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-plugin-job-receiving.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-plugin-tool-run-turns.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-plugin-decl-emission.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-plugin-decl-app.patch` (OK)
1. `1102-tools/phase/patches/semio-framework-plugin-window-config.patch` (OK)
- Check: same as B1/B2 for the crates a patch touches (native check, wasm for plugin/job/value crates); plus `semio-framework-value-derive*` proc-macro expansion users (every crate deriving)
- Why here: phase patches edit many of the same files and sites as policy B1/B2 (same names: actor-retained-turn, job, os-kernel, os-store-replay-operation, pack-record-retained, replication-history, value-derive, value-retained-map). A phase patch that FAILS in the sequence table conflicts with an applied policy patch: skip it if policy already carries the edit, otherwise rebase it. Phase-only patches (job-failure, job-payload, flow-extensions, flow-wasm, store-initialization-replay, text-splice-native, value-*) apply independently. Proc-macro (`value-derive`, `value-derive-expansion`) first inside the batch: it changes what every derive user compiles.

### B3 shared test-law constructors publish framework schemas (owner: `testkit`)
Patches, in order:
1. `1102-tools/testkit/patches/publish-framework-schemas.patch` (OK)
- Check: `check --tests -p semio-framework-plugin-testing-artifact-app-laws` and the new law `registered_construction_publishes_framework_schemas_before_the_manifest` (dummy fixture crate), native
- Why here: Schema publication before any pane smoke test constructs an app (energie `bind-weather-file` RefUnresolved). Must precede the rtpanes re-run; after B1/B2 because the laws crate links plugin and store.

### B4 framework test-support scripts and Cargo edits (owner: `testport`)
Patches, in order:
1. `1102-tools/flowext/patches/directed-dag-facade-exports.py` (script, run from repo root)
1. `1102-tools/testport/patches/cursor-dev-vcs-testkit.py` (script, run from repo root)
1. `1101-tools/testport2/patches/reactor-jobs-cargo-protocol-laws.md` (hand edit, 1 line)
1. `1101-tools/testport2/patches/instance-render-tool-clock-feature.md` (hand edit, 2 sites)
- Check: `check --tests -p semio-framework-os-store-cursor -p semio-framework-os-flow-host -p <reactor jobs crate> -p <instance render crate>` native
- Why here: Pure manifest/facade edits that unblock test compiles; they only add features or re-exports (definitions before callers). Apply scripts first, hand edits (reactor-jobs, render) after reading the current line; render edit needs the author decision recorded in the plan.

### B5 plugins: policy fixes, gen brep, raster, gen3d host (owner: `policy (mutpolicy scripts and gen brep by the same owner in this batch)`)
Patches, in order:
1. `1102-tools/policy/patches/semio-s-artifact-draw-drawing-io.patch` (OK)
1. `1102-tools/policy/patches/semio-s-artifact-energy-model-session.patch` (OK)
1. `1102-tools/policy/patches/semio-s-artifact-fem-2d-editor-session.patch` (OK)
1. `1102-tools/policy/patches/semio-s-artifact-gis-gismap-gis2d.patch` (OK)
1. `1102-tools/policy/patches/semio-s-artifact-puzzle-2d-text-inverse.patch` (OK)
1. `1102-tools/policy/patches/semio-s-plugin-flow-brep.patch` (OK)
1. `1101-tools/gen/patches/brep-boolean-funded-phases.patch` (OK)
1. `1102-tools/policy/patches/semio-s-artifact-puzzle-2d-mut-any.patch` (OK)
1. `1102-tools/policy/patches/semio-s-artifact-puzzle-2d-snapshot.patch` (OK)
1. `1102-tools/policy/patches/semio-s-artifact-draw-drawing-scene-paint.patch` (OK)
1. `1102-tools/policy/patches/semio-s-artifact-draw-drawing-scene.patch` (OK)
1. `1101-tools/mutpolicy/patches/gen-host-displaced.py` (script, fail-closed on count 1)
1. `1101-tools/mutpolicy/patches/raster-model.py` (script, asserts exactly one anchor per site)
1. `1101-tools/rtpanes/patches/process3d-pane-smoke-target.md` (hand edit, optional, not on the activation path; skip if time is short)
1. `1102-tools/rtpanes/patches/energy-manifest-uses-stdio-semio-edit-mode.md` (hand edit, 1 line; REQUIRED for the energie pane, not optional)
- Check: `check --target wasm32-wasip2 --manifest-path <plugin workspace> -p <crate>` for each crate in `policy/patches/index.json` of this batch (draw-drawing-io, energy-model-session, fem-2d-editor-session, gismap-gis2d, puzzle-2d-text-inverse, flow-brep), then `-p semio-s-artifact-raster-model`, the generation3d host crate and the brep crates (`brep-boolean`, `analysis-check`, `resumable-boolean-job`, `brep-session-state`)
- Why here: Gen brep adds a `BrepSession::Failed` variant and `step_budgeted`: definition and every match arm are in the same patch (5 files), so it is compile-atomic only if the whole patch applies. Apply it after the flow-brep policy patch (same plugin family, disjoint file).

### B5P plugin phase-machine fixes (lane phase) (owner: `phase`)
Patches, in order:
1. `1102-tools/phase/patches/semio-s-artifact-draw-drawing-scene.patch` (OK)
1. `1102-tools/phase/patches/semio-s-artifact-fem-3d-editor-results-window.patch` (OK)
- Check: wasm `--lib` check of the draw drawing-scene crate
- Why here: single plugin patch after policy B5.

### B6 stdio plugins: ifc policy fixes, step dev-dep, mesh/pdf decouple (owner: `stdio`)
Patches, in order:
1. `1102-tools/policy/patches/semio-s-artifact-stdio-ifc-2x3-base-details.patch` (OK)
1. `1102-tools/policy/patches/semio-s-artifact-stdio-ifc-4x3.patch` (OK)
1. `1102-tools/policy/patches/semio-s-artifact-stdio-ifc-details-guidance.patch` (OK)
1. `1102-tools/policy/patches/semio-s-artifact-stdio-ifc-details-provider.patch` (OK)
1. `1102-tools/describe/patches/step-geometry-dev-dep.md` (hand edit, 1 line + workspace dependency)
1. `python3 -I 1102-tools/stdio/patches/decouple-mesh-export-from-pdf.py` (dry-run first, then `--apply`)
- Check: wasm `--lib` check of the 4 ifc crates, then `check --tests -p semio-s-artifact-stdio-step-geometry` (describe item), then `check -p semio-s-artifact-stdio-semio-drawing-png-export -p <mesh export> -p <drawing export> -p <pdf>` after the decouple script
- Why here: Policy ifc diffs are independent single-crate edits; the decouple script goes LAST because it creates a crate and rewrites 8 manifests (a failure leaves a half-applied set only if `--apply` is interrupted: it plans all 17 edits in memory first).

### B7 uii18n: terminology, form and status labels (cad step labels first) (owner: `uii18n`)
Patches, in order:
1. `1102-tools/uii18n/patches/cad-step-labels-1.patch` (OK)
1. `1102-tools/uii18n/patches/cad-step-labels-2.patch` (OK)
1. `1102-tools/uii18n/patches/flow-form-labels-1.patch` (OK)
1. `1102-tools/uii18n/patches/flow-form-labels-2.patch` (OK)
1. `1102-tools/uii18n/patches/procedural-docstring-1.patch` (OK)
1. `1102-tools/uii18n/patches/process-status-steps-1.patch` (OK)
- Check: wasm `--lib` check of the cad editor crate, flow editor crate, procedural foundation semantic-ui crate and process workpiece window crate
- Why here: cadi18n is built on top of cad-step-labels-1/2, so they land before B8.

### B8 cadi18n: schema-first cad interaction texts, then readers, then tests (owner: `cadi18n`)
Patches, in order:
1. `1102-tools/cadi18n/patches/cad-editor-callers.patch` (OK)
1. `1102-tools/cadi18n/patches/cad-engine-0.patch` (OK)
1. `1102-tools/cadi18n/patches/cad-engine-1.patch` (OK)
1. `1102-tools/cadi18n/patches/cad-engine-2.patch` (OK)
1. `1102-tools/cadi18n/patches/cad-mode-edit.patch` (OK)
1. `1102-tools/cadi18n/patches/cad-mode-window-building.patch` (OK)
1. `1102-tools/cadi18n/patches/cad-mode-window-energy.patch` (OK)
1. `1102-tools/cadi18n/patches/cad-mode-window-shape.patch` (OK)
1. `1102-tools/cadi18n/patches/cad-mode-window-structure.patch` (OK)
1. `1102-tools/cadi18n/patches/cad-spec-json-1.patch` (OK)
1. `1102-tools/cadi18n/patches/cad-spec-rust-0.patch` (OK)
1. `1102-tools/cadi18n/patches/cad-spec-rust-1.patch` (OK)
1. `1102-tools/cadi18n/patches/cad-test-languages.patch` (OK)
1. `1102-tools/cadi18n/patches/cad-tests-1.patch` (OK)
1. `1102-tools/cadi18n/patches/cad-tests-2.patch` (OK)
1. `1102-tools/cadi18n/patches/cad-tests-3.patch` (OK)
1. `1102-tools/cadi18n/patches/cad-tests-4.patch` (OK)
1. `1102-tools/cadi18n/patches/name-derive-test.patch` (OK)
1. `1102-tools/cadi18n/patches/name-derive.patch` (OK)
1. `1102-tools/cadi18n/patches/name-dwg-test.patch` (OK)
1. `1102-tools/cadi18n/patches/name-geometry-1.patch` (OK)
1. `1102-tools/cadi18n/patches/name-geometry-2.patch` (OK)
1. `1102-tools/cadi18n/patches/name-history-test.patch` (OK)
1. `1102-tools/cadi18n/patches/name-import.patch` (OK)
1. `1102-tools/cadi18n/patches/name-kit.patch` (OK)
1. `1102-tools/cadi18n/patches/name-panels-artifact-test.patch` (OK)
1. `1102-tools/cadi18n/patches/name-panels-artifact.patch` (OK)
1. `1102-tools/cadi18n/patches/name-panels-inspection-test.patch` (OK)
1. `1102-tools/cadi18n/patches/name-panels-inspection.patch` (OK)
1. `1102-tools/cadi18n/patches/name-picking-test.patch` (OK)
1. `1102-tools/cadi18n/patches/name-sample-scene.patch` (OK)
1. `1102-tools/cadi18n/patches/name-terminology.patch` (OK)
- Check: wasm `--lib` check of cad spec crate, cad engine (interaction), cad editor (modes, windows, callers); then `check --tests` on the cad unit tests; then `python3 <cad>/🧪️tests/🌐️languages/🐍️.py` (language-agnostic, passes on the scratch copy)
- Why here: Order = Cargo deps -> spec Rust types (schema owner) -> spec JSON (60 files) -> engine readers -> kit and commands -> modes/windows -> editor callers -> tests. Author: production + schema + data must land in ONE window, compile-atomically; run the batch check only after the whole batch is applied (the embedded JSON is parsed by the new types, so a half batch cannot compile). A reader before its type would not compile.

### B9 TS: process wasm-build optional report callback (owner: `policy`)
Patches, in order:
1. `1102-tools/policy/patches/ts-framework-process-wasm-build.patch` (OK)
- Check: `bun test ./<wasm-build tests>` with explicit ./ path and `tsc --noEmit` of the framework process package
- Why here: Framework TS before demonstrator TS; callers opt in, no break.

### B10 TS: ShellHost locale request, demonstrator, acceptance (owner: `cadi18n`)
Patches, in order:
1. `1102-tools/cadi18n/patches/web-acceptance-1.patch` (OK)
1. `1102-tools/cadi18n/patches/web-demonstrator-1.patch` (OK)
1. `1102-tools/cadi18n/patches/web-shellhost-0.patch` (OK)
1. `1102-tools/cadi18n/patches/web-shellhost-1.patch` (OK)
1. `1102-tools/cadi18n/patches/web-shellhost-2.patch` (OK)
- Check: `bun test ./<locale-request tests>` (explicit ./ path), tsc of the demonstrator, then coordinator-run `test-e2e`
- Why here: web-shellhost-0 (new locale-request module) -> -1 (its test) -> -2 (ShellHost uses it) -> demonstrator -> acceptance test. Requires B8 only for the wire shape of cad texts.

### B11 TS: framework 2d booleans onto the retained work runtime (owner: `e2eready`)
Patches, in order:
1. `python3 1101-tools/ts2d/patches/apply.py` (copies booleans.ts, paths.ts, booleans.test.ts; strips two export names)
- Check: `bun test ./🧰️framework/🔨️modules/◻️2d/🔀️booleans/🧪️tests/🟦️.ts` (explicit path), tsc of the 2d package
- Why here: Independent of everything else; whole-file replacement, so run it last and only if the 2d package is in the demonstrator closure (check `graph`); skip otherwise.

## 4. Conflicting / obsolete against the current tree

- TWINS (policy vs phase): phase's `supersedes.txt` names 4 pairs (pack-record-retained, replication-history, os-store-replay-operation, plugin-job-receiving): the policy twin is SKIPPED, the phase patch replaces it (inventory state SUPERSEDED). `phase/semio-framework-os-kernel.patch` is no longer in phase's `order.txt`: treat as dropped; the policy `os-kernel` patch stays. Remaining phase patches are disjoint from policy except where the sequence table shows FAIL.
- CONFLICT (order): `cadi18n/cad-mode-edit.patch` fails alone (`patch failed … 🎭️modes/✏️edit/🦀️.rs:539`); applies after `uii18n/cad-step-labels-1` and `-2`. Not obsolete.
- OBSOLETE: `1101 actinfra/component-scheduler-flag-views.patch` (target is /tmp/sa-dep.ts; fix already in the tree as `requireCompatibleCargoFlagViewsV1`).
- ALREADY APPLIED (do not re-run): closure manifest deps, stdio3 decode guard, ts2d admission diff, energy bridge, runtime x2, flowext geometry-close-ticket, testport cold-reserve, rtpanes by-example redo, fem PLAN.
- SUPERSEDED: rtpanes `laws-publish-framework-schema-documents.md` (by `testkit`).
- FALSE FAILURES of the repo-root `git apply --check`: stdio3 decode (path relative to `🛬️decode`), ts2d admission (`-p0` from `🧰️framework`). Both content-verified applied.
- STALE-RISK: the 1101 `ts2d` copies were made at 21:02 Nov 1 from a base whose `paths.ts` differs from the tree only in lines 73/91/97 (the tree still has the old `advance(1)` + `commitSemanticWork` code), i.e. the patch is a successor, not a revert. Safe, but whole-file: any edit made to these 3 files after 21:02 is lost; re-diff immediately before B11.
- HOLD (needs design, not applicable as written): testport2 `time-travel-instance-jobs-lib-test-cycle.md`; flowext `retirement-tests.md` item 2 and 3; compute `trapped-guest-test.md`.
- POLICY SIDE: all 34 policy diffs were generated against the tree of ~02:45-13:10; any production edit under the touched files between now and the window invalidates a hunk. Re-run `seq.sh` right before the window (it takes under a minute).

`git apply --check` result per diff (current tree, each alone):

- CLEAN: 139
- FAIL: error: patch failed: ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs:539: 1
- FAIL: error: /tmp/sa-dep.ts: No such file or directory: 1
- ALREADY-APPLIED: 2
- FAIL: error: 🛬️decode/🦀️.rs: No such file or directory: 1

## 5. Risky patches (compile risk named by authors or by reading)

- policy (author): energy `run-job` (annotated closure return type), fem2d session (`return` inside a closure arm), ifc2x3 `field_edit` (borrow moved out of the `with_native_receiving` closure), os-store-replay (`Option::filter` shadowing). Check these four crates first inside B2/B5/B6.
- gen `brep-boolean-funded-phases.patch`: 5 files, adds a `BrepSession::Failed` variant, makes `step_unbudgeted` non-pub, adds `is_funded_validation`; every consumer match on the enum must have the new arm (patch adds them in `analysis-check` and `brep-boolean`); a third consumer elsewhere would fail to compile. grep `BrepSession::` workspace-wide before applying.
- cadi18n (author: cargo UNVERIFIED): changes `InteractionSpec.label`, `TransitionSpec.label`, `SelectionSpec.prompt`, `DisplayItemSpec::Label.text` from `String` to `LocalizedLabel` and threads `(terminology, locale)` through ~10 fns and 4 window `render`/`engagement` fns; 60 spec JSON files rewritten (320 KB patch). Largest cross-crate signature change in the window; keep B8 whole or revert whole.
- stdio `decouple-mesh-export-from-pdf.py`: creates a crate, rewrites 8 manifests and a lib path; generated, not reviewed by anyone else; also touches Cargo.lock implicitly on next build.
- testkit patch: adds `.expect` in test-support (`publish_framework_schemas`); policy gate counts it unless the file is under `🧪️testing`/tests. Compile risk low (author: UNVERIFIED).
- ts2d `apply.py`: not fail-closed, overwrites 3 files with copies from Nov 1 21:02; stale-copy risk named in section 4.
- render feature edit (testport2): cfg change in tool-machine staging may need `feature = "artifact-app-testing"` only; two alternatives were offered; wrong choice reproduces E0609 in the render lib tests.
- Cargo.lock: the closure Cargo.toml patch already regenerated 7 plugin lockfiles + hub lock; cad-engine-0 and cad-spec-rust-0 add `semio-framework-ui-locale` to cad manifests, which can require a lock refresh the owner may not run (only the coordinator runs `dependencies sync|lock`).
- cadi18n `name-*` (15 patches, appeared 13:50): object-name schema change across cad geometry, terminology, kit, derive, import, panels (3d artifact + inspection), picking/history/dwg/sample-scene tests; edits the same engine file `cad-engine-1` touches. The cadi18n set was regenerated twice (13:20, 13:50; `cad-kit`/`cad-commands`/`cad-tests-5,6` came and went), so B8 order is taken from the alphabetical glob that the author's `verify.sh` proves, and must be re-verified by `seq.sh` immediately before the window.
- actinfra `retention-intermediates.diff` (-p0): changes custody of dependency rlibs from `retained` to `current` in `completedCargoClosureRosterV1`; author-named risk: component-pass wasm builds no longer retain dependency rlibs; run the component-pass tests after applying. It does not touch Rust, so it cannot break the Bazel activation compile.
- actinfra `worker-spare-instances.diff`: changes `--worker_max_instances` planning (RepoBuild=3, RepoScript=2); 3 concurrent wasm builds under a 4017 MB worker budget (author: cargo children are outside the budget, the capacity door covers them); pinned fixtures must change in the same window; a running Bazel server keeps the old flags until restarted.

## 6. New since last refresh / unplaced

- none

## 7. Refresh log

- 14:36: regenerated; 189 files, 0 unplaced.
- 14:40: FINAL refresh; act18.status has `exit 130 2026-11-02 14:35:50`; B0 (actinfra) is the activation blocker fix and goes first.
