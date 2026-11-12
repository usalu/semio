# facade lane (11/12) result

## Verified
- W-facade-4 hub wasm `-p semio-plugin` rc 0; W-facade-5 native `Cargo.toml -p semio-plugin` rc 0 (current code incl. MediaConstruction protocol).
- W-facade-8 (hub wasm, 8 crates): stdio-contract, -contract-editing, -contract-media-export (my edits) compiled with no errors.

## Done
- wp2-declaration: dropped removed No*RetirementFactory re-exports; `MediaConstruction` exported at `semio_plugin` root.
- `MediaConstruction` (decl_app media-export): frame + failure + failure retirement; one birth protocol (`built`, `settle`, `close_demand`, `close_step`).
- `build_media_export_job(.., &mut MediaConstruction, cx)` on trait, wrappers, 11 implementors; `MediaExportPreparation.construction` closes/retires failed births (open item 3).
- Snapshot Pack job ported (admission, effect-based cursor advance/close).
- IncrementalMediaExportJob (mp4/avi/mp3): body/page/cursor born via MediaConstruction; wrappers pass it through.
- raster/layout `admit_original`: body birth via MediaConstruction (rest of those jobs NOT ported: stale close_turn(grant)/step API remain).
- epw native-export test passes a MediaConstruction (not compiled).
- stdio editing contract: `semio_framework_plugin` -> `semio_plugin` (41 sites; dep is semio-plugin).

## Not verified / open
- W-facade-8 rc 101: unrelated upstream reds block the wasm check of mp4/avi/mp3/raster/layout/epw crates: sqlite snapshot ports (stdio binary/semio-base/avi/mp4/mp3/step/epw snapshot, draw drawing-model, layout-model, layout-diff-plan: receive_pack / *_sqlite_snapshot_* output-slot signatures) and semio-os host (22 errs, workflow-store). Not facade-owned.
- raster/layout/remodel mesh-export jobs still need a full port (grant-based close, old step signature); epw/mp4-viewer crates are not in root or hub workspace.
- native root Cargo.toml has none of these crates; native check applies only to semio-plugin.
- Note: I ran one read-only `cargo metadata` to learn workspace membership.
