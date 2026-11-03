STATUS: GREEN for semio-s-artifact-gis-gismap-tests --lib native: 84 passed, 0 failed, rc 0 (gis-t16.txt:83883), after patch store-replay-retire-unborn-registry-birth APPLIED (W-gis-1 wasm check rc 0). hub-gis(-component) and gisterrain native tests in flight/UNVERIFIED.

## gis-t6 failures and fixes (all under 🧪️tests/, no production fault)
1. Three initializer tests (host/owned tests: history_edit_initializer, store_initializer_cancel_and_stale, initialization_candidate_projects_child_handles): `UnsupportedOwner "native construction/retirement requires the original Job decoder issuer"`. Cause: tests built `StepContext::new(..)` with no original decoder; framework `StepContext::with_native_receiving/with_native_retirement` (job/⏱️context/🦀️.rs:186-192) needs one. Fix: ported to `OriginalJobFixtureCaller::turn_with_decoder` + `OriginalJobDecodeOwner` via new test helper `GisOriginalTurns` (finish() closes decoder, retires caller). Stale-generation case now admits the caller at generation+1.
2. `reorder_regions … produces_committed_diff`: committed fixture `…/🧪️tests/🧫️fixtures/🧬️mutations/🔃reorder-regions/🌳️moves/🔺️diff/🔣️.json` was stale (old-town 1→2). Production `diff()` builds one moved row for the payload id (`GisMapFeaturesDelta::relocation(base, from, to)`, list-delta 🦀️.rs:332) = region-park 2→1, matching the docstring. Fixture corrected.
3. New viewer test `the_viewer_map_scene_selects_and_hovers_nothing_and_declares_no_interactions` (viewer `🗺️map/🧪️tests/🔬️unit/🦀️.rs`) asserted `{"positions":[],"routes":[]}`; framework default selection is `"{}"`. Now compares to `tiled_map_default_selection_json()`.

## Earlier
- gismap-tests first compile fix: `.result` on retire_owned/close_step calls. gis-t3 wrong manifest, gis-t4 queue deadline. Manifest: `1102-tools/gis/manifest.txt`.
- TiledMap select/hover: editor tests (select + hover round trips, editor 🗺️map/🧪️tests/🔬️unit) and the new viewer read-only law. Viewer test lives in the viewer crate, not run by `-p semio-s-artifact-gis-gismap-tests` mirror: UNVERIFIED until the viewer crate test run.

## Not done
- Native tests for gismap editor/viewer crates, semio-hub-gis, gisterrain.

## Blocker detail (gis-t10, 17:40)
- File: `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧩️record/🔎️schema/🦀️.rs` (mtime 17:33), errors at gis-t10.txt:5942-5976. Owner: phase lane batch B1P. Not edited by me (not my batch).
- Rerun when clean: `1102-tools/slot.sh gis-t11 1102-tools/gis/tst.sh gis-t11 "$(cat 1102-tools/gis/manifest.txt)" -p semio-s-artifact-gis-gismap-tests --lib`
- gis-t9 earlier blocked on os-store-presence rejection (policy batch), since fixed.

## gis-t11 (after B1P)
- 81 pass / 3 fail: the 3 ported initializer tests panic in `OriginalJobFixtureCaller::admit` (job-testing-caller 🦀️.rs:11) because the receipt (64 B) is compared to heap observation (0): the test binary had no `HeapWitness` global allocator (same as the flow retirement tests, `#[global_allocator] static …: semio_framework_trace::HeapWitness`).
- Fix (tests only): `#[global_allocator] static GIS_TEST_HEAP` in gismap `host/owned/🧪️tests/🔬️unit/🦀️.rs`; dev-dependency `semio-framework-trace` (path) added to the gismap inferences-tests crate Cargo.toml (tests-crate manifest; coordinator: Cargo.lock for the gismap workspace may regenerate).
- The 5 gis-t6 fixes (reorder fixture, viewer select/hover test) are PROVEN by gis-t11 (they no longer fail).

## gis-t12 (global_allocator fix proven: 3 harness failures gone; 82 pass / 2 fail, gis-t12.txt:83265)
- PRODUCTION FAULT (patch, freeze): `gis_map_store_initializer_cancel_and_stale_generation_return_every_owner_terminal_empty` panics `original Job retirement requires its existing decoder receipt` (job/🛬️receiving/🦀️.rs:18). Root cause: `ArtifactStoreReplayInitializer::retire_native` (🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🦀️.rs:493-496) calls `begin_close()` then `cx.with_native_retirement(..)` on `registry_birth` that was never born (cancel/stale before the first construction turn); `SnapshotReadRegistryBirth::terminal_is_empty` (store/🔗️read/🔒️registry/🌱️birth/🦀️.rs:33) is true right after `begin_close` for an unborn birth, but the flow does not re-check, and `with_native_retirement` demands a decoder receipt that only exists after a construction. Patch: `1102-tools/gis/patches/store-replay-retire-unborn-registry-birth.patch` (+1 line: return after begin_close when terminal-empty). Not applied (freeze); needs the replay crate's batch owner (store) and a window.
- TEST: `the_initialization_candidate_projects_its_canonical_child_handles` — `close_gis_map_candidate` never reached terminal; hypothesis: the candidate close quote used copy-byte arg 0 while the owned-field retirement needs `GIS_MAP_OWNED_FIELD_BYTES` (as the nested-value test does). Changed; verifying in gis-t13.

## gis-t13 verdict (gismap-tests --lib native)
- 83 passed, 1 failed. The candidate-close quote fix (GIS_MAP_OWNED_FIELD_BYTES) is PROVEN: `the_initialization_candidate_projects_its_canonical_child_handles` passes.
- Remaining failure = the production fault above (`…cancel_and_stale_generation…`, "original Job retirement requires its existing decoder receipt"). Passes once the patch is applied in a window (apply to `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🦀️.rs` ~line 494, then rerun gis-tNN with the same command). Narrow check for the patch: `check -p semio-framework-os-store-engine-replay` native (replay crate).
- TiledMap select/hover: editor round-trip tests (select, hover) and the new viewer read-only law pass in this run (viewer test `…windows::map::tests::the_viewer_map_scene_selects_and_hovers_nothing_and_declares_no_interactions` not in the failure list).
- Test edits this wave: gismap host/owned tests (result API, GisOriginalTurns + global_allocator + candidate close quote), viewer map test, reorder-regions diff fixture, tests-crate Cargo.toml dev-dep `semio-framework-trace` (coordinator: gismap Cargo.lock may regenerate).
- Not run: semio-hub-gis(-component), gisterrain, editor/viewer crate-own tests.
- gis-t14: aborted by peer B6 churn (gismap/Cargo.toml edited mid-run, unresolved semio_s_artifact_stdio_semio_drawing_png_export at 23:59); gis-t15 reruns.

## gis-t15 (patch applied; W-gis-1 wasm --lib check of semio-framework-os-store-initialization-replay rc 0)
- 83 pass / 1 fail (gis-t15.txt:83508). The retirement-receipt panic is GONE (patch proven). Remaining assertion: stale generation now ends `Cancelled`, test expected `Fault` (:205). By design: `ArtifactStoreReplayInitializer::turn` (replay 🦀️.rs:263-274) sets `cancel_requested` on a generation/operation mismatch, so a stale turn retires as Cancelled; `Fault` only appeared before because the receipt-less retire failed. Test expectation changed to `Cancelled` (tests only); gis-t16 verifies. NOTE for other lanes: the same template asserts `Terminal::Fault` for stale in raster `host/owned/🧪️tests/🦀️.rs:132` and likely others (flow/etc.).

## 04:55 capacity blocker (gis-t16 in flight, waiting)
- gis-t16 holds a slot but waits in `repo cargo` capacity 2250+ s: "not enough free memory (need 802 MB; -286 MB spare above the 4096 MB floor; 0 running, behind the host itself)". Host memory check: `fseventsd` (macOS file-event daemon) resident ~13.8 GB of 32 GB, nothing of ours running. Likely cause of fleet-wide starvation; needs the user/coordinator (restarting fseventsd requires root, not done by me).

## 08:00 final state
- GREEN: `semio-s-artifact-gis-gismap-tests --lib` native, 84 passed / 0 failed, rc 0 (gis-t16.txt:83883). Includes TiledMap editor select/hover round trips + the new viewer read-only select/hover law.
- Production patch APPLIED: store replay `retire_native` early return for an unborn registry birth (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🦀️.rs` ~494); no leak: the early `return Ok(false)` only skips the registry-birth block, the next `retire_native` call proceeds to supersession/native/close_original, `close_demands` already skips the empty birth. W-gis-1 wasm --lib check of semio-framework-os-store-initialization-replay rc 0.
- Test expectation changed: stale generation ends `Cancelled` (by design). Other lanes' copies of this template (raster host/owned tests:132 …) may assert `Fault`.
- NOT RUN (UNVERIFIED): `semio-hub-gis` + `semio-hub-gis-component` native (hub manifest) and gisterrain native. gis-t17 (hub-gis) waited 2h20m in the slot queue with K=1 (W- class-0 entries always ahead); waiter stopped 08:00, nothing in flight.
- Rerun: `1102-tools/slot.sh gis-t18 1102-tools/gis/tst.sh gis-t18 🌎️hub/Cargo.toml -p semio-hub-gis -p semio-hub-gis-component`, then `… gis-t19 "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/Cargo.toml" --workspace`.
- Host: `fseventsd` holds ~13.8 GB RSS (cargo capacity waits up to 55 min "behind the host itself"); restart needs root.
