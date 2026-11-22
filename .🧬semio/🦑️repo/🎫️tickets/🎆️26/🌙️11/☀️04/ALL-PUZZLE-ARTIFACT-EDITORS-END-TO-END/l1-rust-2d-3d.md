# L1 Rust 2D and 3D artifact crates

Status at hand-off: lib and bin targets of both workspaces pass `cargo check`; the test targets do not compile yet. 2D: 88 error sites in 24 files across 9 test crates (down from about 1,000 errors in 19 crates). 3D: 123 error sites in 24 files across 14 test targets (not yet touched). No test binary was ever run, because of an environment limit described below.

## Execution

`repo run` / mcp was not attempted: `2d-runtime.md` already records that `repo run @semio-tech/puzzle-2d-rs:*` and `repo cargo` die before compilation at `source.capture` (authorizer capacity 1,831/2,048 files) and at the `icon_pack_digest` gate. Native cargo was used instead, via `l1-native.sh <2d|3d> <cargo args>` in this folder. It adapts the 5D precedent: `unshare` tmpfs build dir, nightly-2026-07-20 toolchain, `CARGO_HOME` under `.cache/b/r/m/cargo`, `RUSTC_WRAPPER=""`, `-Z threads=1`, `--offline`, `--jobs 3`. `--locked` is replaced by `L1_LOCK=-q` because both `Cargo.lock` files were stale (cargo refused with `cannot update the lock file ... --locked`); cargo regenerated them offline.

Environment limits found:
- `/home` is at 100% (2.3 GB free). The first build dir under `🗑️generated` ran out of space (`No space left on device` while compiling `semio-os-directory-check-in`), so the build dir was moved onto a 7 GB tmpfs inside the unshare.
- `cargo test` (full codegen of the dependency closure) exhausted that 7 GB tmpfs (`failed to build archive ... No space left on device`). Test binaries were therefore not run; the remaining numbers are compile-only. A rerun needs either freed disk or a larger tmpfs (RAM was too tight to raise it safely).

## Commands run and results

| Command (via `l1-native.sh`) | Result |
|---|---|
| `2d check --workspace --keep-going` (lib/bin, 31 members) | PASS (exit 0) |
| `2d check --workspace --all-targets --keep-going` | FAIL: 9 test targets of 2D (list below), 88 sites |
| `3d check --workspace --all-targets --keep-going` | FAIL: 14 test targets, 123 sites; lib/bin did not report errors |
| `2d test --no-fail-fast -p <13 crates>` | Not run: tmpfs full during dependency codegen |

Project targets `build`, `check`, `test`, `test-*`, `verify-*` of `@semio-tech/puzzle-2d-rs`, `@semio-tech/puzzle-3d-rs` and 3D `ownership/compute` were not run through repo. Pass counts: 0 tests executed, because nothing reached the test runner.

2D crates that still fail to compile their tests: `semio-s-artifact-puzzle-2d` (58 errors), `-unit-tests` (33), `-connect-handles-preparation`, `-text-inverse` (lib test and `owned-text`), `-mut-any-5`, `-mutations`, `-schema`, `-interaction`, `-inferences`.
3D test targets that fail: `puzzle-3d-editor` (editor-defects 50, editor-manifest 20, editor-budgets, editor-task-resume, editor-action-producer, editor-panels), `-editor-session` (precompute-brush, precompute-fill), `-retained-action`, `-inferences`, `-puzzle-3d` (snapshot-sqlite), `-editor-modes`, `-precompute` (22), `-precompute-geometry`, `-editor-window`.

## Root causes fixed (2D)

1. Framework API drift in the shared source-custody test fixture. The retained-clone API moved from synchronous grant-based calls (`RetainedCloneBorrowAuthority::admit`, `RetainedCloneSource::admit_borrowed`, `close_step(grant)`, `advance(a, b, grant)`) to original-effect calls (`close_step(body, effect)`, `advance(a, b, body, effect)`, `take(output, effect, reply)`).
   - Rewrote `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🧪️tests/🔗️source-custody/🦀️.rs` on `semio_framework_value_clone_source_testing` and `semio_framework_value_testing_effect_bank`. It now provides `admit`, `close`, `admit_source` (an `Arc<T>` projected through `ArcLease`), `close_source`, `close_cursor` (effect-shaped step), `run`, `driven`, `close_turn`, `take`, `begin_close`. `Custody` and `Custody::borrow` became `pub` (they were `pub(crate)`, which silently fell through to `Deref` once tests live in other crates).
   - `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🔗️source/🧪️testing/🦀️.rs`: added `admit_projected<T, A, B>` (projection where the lent value `T` differs from the owner `A`) and made `admit_owned` call it. This is a framework test-support change, not production code.
   - Added optional deps plus dev-deps on `semio-framework-value-clone-source-testing` and `semio-framework-value-testing-effect-bank` to `🧱️records/📦️packages/🦀️rust/Cargo.toml`, wired to its `testkit` feature.
2. `cfg_attr(test, derive(serde...))` on records, snapshot and every mutation leaf is invisible to dependent crates after the crate carve, so downstream tests got `Puzzle2dSnapshot: Serialize` not satisfied (about 270 errors). Changed all 182 occurrences to `cfg_attr(any(test, feature = "testkit"), ...)`, added the same gate to the previously missing `CreateNode`, `DeleteNode` and the `Puzzle2dMutation` enum (`🧬️mutations/🟢️node/🌱create-node/🦀️.rs`, `.../🗑️delete-node/🦀️.rs`, `🧬️mutations/🦀️.rs`), and added a chained `testkit` feature plus `features = ["testkit"]` dev-dependency edges to all 31 member `Cargo.toml` files (script `l1-testkit-features.py`).
3. `board-host` testkit: the framework moved board helpers to free functions in `semio-os-infinite-board-normal-{focus,gestures,scene}` and made `BoardOutboxState::drain_events_json` `testkit`-gated. Added those crates as deps and dev-deps with `testkit` to `🎲️board-host/📦️packages/🦀️rust/Cargo.toml` and re-exported the functions from `🎲️board-host/🧪️tests/🧰️testkit/🦀️.rs` (`board_set_camera`, `set_selection_ids*`, `resolve_hit_world`, `set_camera_silent`, `set_transform_flags`, `set_preselect_state_silent`, `defers_descriptor_sync_from_js`, `overlay_paint_state_json`, `test_resolve_node_style_kind`, `indirect_handle_*`, `set_drop_preview_json`). This cleared the 81 `board-host` lib-test errors.
4. About 35 prepare/inverse/candidate/lookup/retained-clone test files migrated mechanically to the effect API (`l1-migrate-effect-tests.py`, `l1-fix-args.py`, `l1-fix-take.py`): `advance(..., grant)` to `test_source_custody::run(grant, |effect| ...advance(..., effect))`, `close_step(grant)` to `close_turn`, `owner.step(grant)` to `close_turn(... step(4096, effect))`, `cursor.take()` to `test_source_custody::take`, `close_cursor` callers to the effect step, `paged` cursor import paths to `semio_framework_value::paged::retained_clone::cursors::...`, and the `test_source_custody` imports.
5. `Cargo.lock` of 2D and 3D regenerated offline (stale).

Note: another agent is concurrently editing `🧬️mutations/🦀️.rs` (conflict-path code) and the root `Cargo.toml` members/deps (3D, dwg and pdf crates were dropped); I only added serde `cfg_attr` lines there.

## Remaining 2D failures (88 sites)

Each is a call-site or import that no longer matches the current production API; none is a defect in my fixtures. Grouped:
- Effect-API tests not yet migrated: `brush/.../fill-fixtures-and-mounted-session-helpers` (`original.turn(grant, |cx| capture.step(host, cx))` now takes 5 args; `close_original_retained_step` closure shape), `fill-frontier-checkpoint-and-job-laws` (`pump_one` removed from `MountedWorkerJobSession`; `step(cx)` arity), `candidate-acceptance-preview-and-proximity` line 67 (moved `original`), `🧬️retained-clone` test (`__Puzzle2dSnapshotRetainedCloneCursor` has no `advance`; derive cursors need `advance_into` through `effect::advance`), text prepare tests (private fields `phase`, `lookup`, `comparison` read from tests; `advance` signature mismatch), canonical test (`RetainedCloneSource::admit_owned` gone; needs `admit_owned` from the clone-source-testing crate), lookup test line 58/65 (`advance` takes `(source, query, effect)`).
- Missing symbols after the carve: `Puzzle2dCommand` (manifest-convergence, operations-and-command-envelope, recorded-kinds), `puzzle2d_manifest_for_tests`, `BODY_KEY`, `Puzzle2dNodeAnchor`/`Puzzle2dKindCompatibility`/`Puzzle2dCatalogNodeKind`/`Puzzle2dRepresentation`/`Puzzle2dAuthor`/`Puzzle2dCatalogWireKind` in the schema unit test (`🧬️schema/🧪️tests/🔬️unit/🦀️.rs`, 10 sites), `set_camera_silent`/`set_transform_flags`/`set_selection_ids_silent` in `select-tool-transactions` and `select-tool-history` (import from `semio_os_infinite_board_normal_*` or the board-host testkit).
- Trait-as-type: `protocol::Mutation::<Puzzle2dSnapshot>::diff/inverse` (E0782, `Mutation` is now a trait with a different generic shape; recorder, recorded-kinds, history-kit tests), `Puzzle2dMutation::diff` not found (select-tool).
- Signature drift: `ArtifactEditor::command_from_action` takes 3 args (gumball test, outliner laws), `load_board_snapshot_json` takes 6 args (engine-parse test), `SqliteSnapshotControl`/export/decode_native extra arg (sqlite snapshot tests, 13 sites), `issuer.generation().0` (`u64` now), `DslValue::from(&json!(..))` removed (use `pack_json::to_dsl_value`), `PagedBytes` is no longer an iterator, `VcsArtifactApp` not dereferenceable, presence test closure now `Fn(&NoPresence)`.

Full site list (file tail, line, message, source line):

```
88 sites 24 files
## 1 any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🔎️candidate-acceptance-preview-and-proximity/🦀️.rs
   67 use of moved value: `original` | 6 |         let mut original = fixture(semio_framework_job::default_now_us);
## 10 ️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🧪️fill-fixtures-and-mounted-session-helpers/🦀️.rs
   24 this method takes 5 arguments but 2 arguments were supplied | 24 |             match original.turn(FILL_TEST_GRANT, |cx| capture.step(host, cx)).expect("original captured d
   120 mismatched types | 120 |                   semio_framework_job::close_original_retained_step(cx, demand, |grant| Ok(match placeme
   138 mismatched types | 138 |                   semio_framework_job::close_original_retained_step(cx, demand, |grant| Ok(match capture
   152 this method takes 3 arguments but 2 arguments were supplied | 152 | ...ginal.turn(FILL_TEST_GRANT, |cx| pending.admit_pending(config, cx)).expect("same original mounted adm
   165 mismatched types | 165 | ...me(step) = session.return_original_cancel_alias_step(cx.original_cancel_token(), cx.retained_grant())
   187 no method named `pump_one` found for mutable reference `&mut MountedWorkerJobSession<semio | 187 |             match session.pump_one(pool, semio_framework_async::Lane::Background, FILL_TEST_GRANT) {
   224 mismatched types | 224 |                 let step = session.acknowledge_checked_out_outcome(cx.retained_grant());
   225 no method named `progress` found for enum `Result<T, E>` in the current scope | 225 |                 cx.consume_retained(step.progress())?;
   226 no method named `progress` found for enum `Result<T, E>` in the current scope | 226 |                 cx.consume_fuel(step.progress().copied_items as u64);
   229 mismatched types | 229 |             if matches!(step, RetainedCloneStep::Complete(_)) { return; }
## 4 ts/✳️any/✏️editor/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🧱️fill-frontier-checkpoint-and-job-laws/🦀️.rs
   161 this method takes 2 arguments but 1 argument was supplied | 161 | ...TEST_GRANT, |cx| stale_job.step(cx).map(|_| ())).is_err(), "different actual source roots must not ex
   206 no method named `pump_one` found for struct `MountedWorkerJobSession<J>` in the current sc | 206 | ...   assert!(matches!(refused.pump_one(&unavailable, semio_framework_async::Lane::Background, FILL_TEST
   280 no method named `pump_one` found for struct `MountedWorkerJobSession<J>` in the current sc | 280 |             session.pump_one(&pool, semio_framework_async::Lane::Background, FILL_TEST_GRANT),
   372 this method takes 5 arguments but 2 arguments were supplied | 372 | ... original.turn(FILL_TEST_GRANT, |cx| capture.step(&host, cx)).expect("original bounded capture refusa
## 3 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📼️recorder/🧪️tests/🔬️unit/🦀️.rs
   54 expected a type, found a trait | 54 |         inverses.push(Mutation::<Puzzle2dSnapshot>::inverse(row, &state).expect("every recorded row inver
   55 expected a type, found a trait | 55 | ...   state = protocol::apply_diff_cold(Mutation::<Puzzle2dSnapshot>::diff(row, &state).diff(), &state).e
   60 expected a type, found a trait | 60 | ...   state = protocol::apply_diff_cold(Mutation::<Puzzle2dSnapshot>::diff(step, &state).diff(), &state).
## 1 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🎯️board-event-and-ui-scope-laws/🦀️.rs
   157 cannot find value `BODY_KEY` in module `crate::standards::v1::subsets::any::schema::mutati | 157 | ...tations::selection::BODY_KEY], "window_bodies must be body keys, not pane ids");
## 2 ards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/📜️manifest-convergence-and-registry-laws/🦀️.rs
   64 cannot find type `Puzzle2dCommand` in this scope | 64 |             if !reserved && Puzzle2dCommand::try_from_action(id, None, None).is_none() {
   144 `PagedBytes<9223372036854775807>` is not an iterator | 144 |             envelopes.extend(operations);
## 1 s/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🕹️gumball-region-set-verb-and-snapshot-laws/🦀️.rs
   165 this function takes 3 arguments but 2 arguments were supplied | 165 | ...t!(<Puzzle2dPlayApp as ArtifactEditor>::command_from_action(verb, Some(&semio_framework_value::ToValu
## 3 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs
   162 the trait bound `DslValue: From<&semio_framework_pack_json::Value>` is not satisfied | 162 | ...ome(&semio_framework_value::DslValue::from(&json!({"camera": {"x": x, "y": y, "zoom": zoom}})))).expe
   165 the trait bound `DslValue: From<&semio_framework_pack_json::Value>` is not satisfied | 165 | ...ome(&semio_framework_value::DslValue::from(&json!({"kind":"node"})))).expect("registered source entit
   168 expected an `Fn(&NoPresence)` closure, found `ViewModel` | 168 |     assert_presence_round_trip(&mut a,&mut b,command,view,expected,(PUZZLE2D_INTERACTION_DOMAIN,Some((PU
## 3 rds/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🧩️engine-parse-and-handle-suggestion-laws/🦀️.rs
   15 this function takes 6 arguments but 3 arguments were supplied | 15 | ...d);semio_os_infinite::board::io::text::snapshot_assembly::load_board_snapshot_json(&mut BoardHost::def
   28 this function takes 6 arguments but 3 arguments were supplied | 28 | ...d);semio_os_infinite::board::io::text::snapshot_assembly::load_board_snapshot_json(&mut BoardHost::def
   34 this function takes 6 arguments but 3 arguments were supplied | 34 | ...d);semio_os_infinite::board::io::text::snapshot_assembly::load_board_snapshot_json(&mut BoardHost::def
## 3 ndards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🧮️operations-and-command-envelope-laws/🦀️.rs
   39 cannot find type `Puzzle2dCommand` in this scope | 39 |     let command = Puzzle2dCommand::from_action("setActiveExample", Some(json!({ "exampleId": PUZZLE2D_PLA
   158 type `VcsArtifactApp<EditorApp<semio_s_artifact_puzzle_2d_v1::editor::puzzle2d::Puzzle2dPl | 158 | ...   block_on(semio_plugin::app::artifact_app_laws::load_document_text(&mut *reopened.0, &saved, &mut s
   161 type `VcsArtifactApp<EditorApp<semio_s_artifact_puzzle_2d_v1::editor::puzzle2d::Puzzle2dPl | 161 | ...   block_on(semio_plugin::app::artifact_app_laws::load_document(&mut *reopened.0, &saved, &mut semio_
## 1 ️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🧰️history-acceptance-source-laws-and-camera-helper/🦀️.rs
   3 cannot find value `puzzle2d_manifest_for_tests` in this scope | 3 | semio_plugin::history_edit_acceptance_law!("puzzle", Puzzle2dPlayApp, puzzle2d_manifest_for_tests, "../.."
## 3 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️recorded-kinds/🦀️.rs
   159 cannot find type `Puzzle2dCommand` in this scope | 159 |     let command = Puzzle2dCommand::from_action(action, Some(args), Some(crate::editor::puzzle2d::config:
   168 expected a type, found a trait | 168 |     protocol::apply_diff_cold(Mutation::<crate::Puzzle2dSnapshot>::diff(row, state).diff(), state).expec
   175 expected a type, found a trait | 175 |         inverses.push(Mutation::<crate::Puzzle2dSnapshot>::inverse(row, &state).expect("the row inverts"
## 1 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool-history/🦀️.rs
   331 no method named `set_selection_ids_silent` found for struct `semio_os_infinite::BoardHost` | 331 |             host.set_selection_ids_silent(&ids(&spec["selection"]));
## 7 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool-transactions/🦀️.rs
   166 cannot find function `set_camera_silent` in this scope | 166 |     set_camera_silent(&mut host.scene, &mut host.outbox, 0.0, 0.0, 1.0);
   167 cannot find function `set_transform_flags` in this scope | 167 |     set_transform_flags(&mut host.scene, &mut host.gestures, true, false);
   252 cannot find function `set_selection_ids_silent` in this scope | 252 |     set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["left".to_string(), "
   340 cannot find function `set_transform_flags` in this scope | 340 |     set_transform_flags(&mut host.scene, &mut host.gestures, true, true);
   341 cannot find function `set_selection_ids_silent` in this scope | 341 |     set_selection_ids_silent(&mut host.scene, &mut host.focus, &mut host.outbox, &["left".to_string(), "
   568 cannot find type `Puzzle2dCommand` in this scope | 568 |     let command = Puzzle2dCommand::from_action("translateSelection", Some(json!({ "dx": 5.0, "dy": 0.0 }
   592 cannot find type `Puzzle2dCommand` in this scope | 592 |     let command = Puzzle2dCommand::from_action("applyBoardEvents", Some(json!({ "eventsJson": events }))
## 3 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️select-tool/🦀️.rs
   114 no method named `diff` found for enum `semio_s_artifact_puzzle_2d_mut_any_10::Puzzle2dMuta | 114 |     assert!(yields[1].1.diff(&dropped).messages().is_empty(), "replayed where it was recorded, the conne
   162 no method named `diff` found for enum `semio_s_artifact_puzzle_2d_mut_any_10::Puzzle2dMuta | 162 |     let outcome = mutations[0].diff(&base);
   257 no method named `diff` found for enum `semio_s_artifact_puzzle_2d_mut_any_10::Puzzle2dMuta | 257 |     assert!(mutations[0].diff(&base).is_applicable(protocol::MergePolicy::default()), "the leaf passes i
## 2 🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/📦️owned/🧪️tests/🦀️.rs
   63 `u64` is a primitive type and therefore doesn't have fields | 63 | ...ect(&self.law["route"]["into"])),generation:issuer.generation().0,revision:issuer.content_revision_now
   161 `u64` is a primitive type and therefore doesn't have fields | 161 | ...t(&native));let mut runner=Runner::new(law,issuer.generation().0);
## 13 🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
   78 cannot find module or crate `semio_framework_value` in this scope | 78 | ...on::from_dsl_value(&semio_framework_value::ToValue::to_value(&expected)))).unwrap();assert_neutral_fie
   108 this function takes 6 arguments but 5 arguments were supplied | 108 | ...er|(owner.export)(&codec.schema,&dialect(),&payload(&expected,encoding),&mut SqliteSnapshotControl::n
   120 this function takes 4 arguments but 3 arguments were supplied | 120 | ...er|Puzzle2dSnapshot::decode_sqlite_snapshot_native(&payload(&expected,encoding),&mut SqliteSnapshotCo
   124 this function takes 4 arguments but 3 arguments were supplied | 124 | ...er|Puzzle2dSnapshot::decode_sqlite_snapshot_native(&payload(&expected,SnapshotEncoding::Text),&mut co
   135 this function takes 6 arguments but 5 arguments were supplied | 135 | ...er|(owner.export)(&codec.schema,&dialect(),&payload(&expected,encoding),&mut SqliteSnapshotControl::n
   141 this method takes 4 arguments but 3 arguments were supplied | 141 | ...ner|play.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits),
   178 this method takes 4 arguments but 3 arguments were supplied | 178 | ...r|source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,exact), 
   179 this method takes 4 arguments but 3 arguments were supplied | 179 | ...ner|play.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,exact), 
   180 this function takes 4 arguments but 3 arguments were supplied | 180 | ...er|Puzzle2dSnapshot::decode_sqlite_snapshot_native(&expected,&mut SqliteSnapshotControl::new(&mut |_|
   181 this function takes 4 arguments but 3 arguments were supplied | 181 | ...er|<crate::Puzzle2dPlaySnapshot as ArtifactSqliteSnapshotHost>::decode_sqlite_snapshot_native(&expect
   184 this method takes 4 arguments but 3 arguments were supplied | 184 | ...r|source.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short), 
   185 this function takes 4 arguments but 3 arguments were supplied | 185 | ...er|Puzzle2dSnapshot::decode_sqlite_snapshot_native(&expected,&mut SqliteSnapshotControl::new(&mut |_|
   149 cannot find module or crate `semio_framework_value` in this scope | 149 | / semio_framework_dispatch_macros::dyn_enum_close!{
## 5 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔎️lookup/🧪️tests/🦀️.rs
   58 this method takes 3 arguments but 4 arguments were supplied | 58 | ...ect| cursor.advance(source.borrow(&snapshot), query.borrow(&target), 4096, effect)));
   65 this method takes 3 arguments but 4 arguments were supplied | 65 | ...ect| cursor.advance(source.borrow(&snapshot), query.borrow(&target), 4096, effect)));
   82 this method takes 3 arguments but 4 arguments were supplied | 82 | ...| cancelled.advance(source.borrow(&snapshot), query.borrow(&target), 4096, effect)).unwrap();
   84 this method takes 3 arguments but 4 arguments were supplied | 84 | ...t| cancelled.advance(source.borrow(&swapped), query.borrow(&target), 4096, effect)).is_err());
   89 this method takes 3 arguments but 4 arguments were supplied | 89 | ...| cancelled.advance(source.borrow(&snapshot), query.borrow(&target), 4096, effect)).unwrap(), Puzzle2d
## 6 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🧬️retained-clone/🦀️.rs
   75 no method named `advance` found for struct `__Puzzle2dSnapshotRetainedCloneCursor` in the  | 75 | ..._custody::run(RetainedCloneGrant::default(), |effect| cursor.advance(source.borrow(), controls["maximu
   82 no method named `advance` found for struct `__Puzzle2dSnapshotRetainedCloneCursor` in the  | 82 | ...hread(||test_source_custody::run(turn_grant, |effect| cursor.advance(source.borrow(), controls["maximu
   136 no method named `advance` found for struct `__Puzzle2dMutationRetainedCloneCursor` in the  | 136 | ...custody::run(RetainedCloneGrant::default(), |effect| cursor.advance(authority.borrow(&original), cont
   141 no method named `advance` found for struct `__Puzzle2dMutationRetainedCloneCursor` in the  | 141 | ...read(||test_source_custody::run(turn_grant, |effect| cursor.advance(authority.borrow(&original), cont
   170 no method named `advance` found for struct `__Puzzle2dMutationRetainedCloneCursor` in the  | 170 | ...read(||test_source_custody::run(turn_grant, |effect| cursor.advance(authority.borrow(&original), 4, 4
   176 no method named `advance` found for struct `__Puzzle2dMutationRetainedCloneCursor` in the  | 176 | ...   if !cancelled { assert!(test_source_custody::run(grant, |effect| cursor.advance(authority.borrow(&
## 2 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/📨️messages/🧪️tests/🦀️.rs
   60 mismatched types | 60 | ...|effect| cursor.advance(source, effect)).unwrap());
   80 mismatched types | 80 | ...|effect| cursor.advance(&source, effect)).unwrap());
## 1 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/📸️candidate/🧪️tests/🦀️.rs
   34 field `phase` of struct `semio_s_artifact_puzzle_2d_mut_any_5::standards::v1::subsets::any | 34 | ...),mutation.borrow(&payload), 4096, effect)));if cursor.phase==phase{reached=true;break}assert!(!matche
## 1 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️text/🎮️prepare/🧪️tests/🦀️.rs
   20 field `lookup` of struct `semio_s_artifact_puzzle_2d_mut_any_5::standards::v1::subsets::an | 20 | ...getMissing=>"targetMissing"},expected);assert!(cursor.lookup.terminal_is_empty()&&cursor.comparison.te
## 2 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧵️canonical/🧪️tests/🦀️.rs
   62 no associated function or constant named `admit_owned` found for struct `RetainedCloneSour | 62 | ...|RetainedCloneSource::admit_owned(mutation,(),full).unwrap_or_else(|(error,_,_)|panic!("original Puzzl
   77 mismatched types | 77 | ...thread(||cursor.advance(&mut byte,full).unwrap());assert!(step.ownership.progress().fits(full));assert
## 10 🧪️tests/🔬️unit/🦀️.rs
   23 cannot find type `Puzzle2dNodeAnchor` in this scope | 23 |     assert_eq!(node.anchor, Puzzle2dNodeAnchor::Fixed);
   38 cannot find struct, variant or union type `Puzzle2dKindCompatibility` in this scope | 38 |     let row = Puzzle2dKindCompatibility { source: "a".into(), target: "b".into(), bidirectional: true, im
   48 cannot find struct, variant or union type `Puzzle2dCatalogNodeKind` in this scope | 48 |         nodes: vec![Puzzle2dCatalogNodeKind {
   58 cannot find struct, variant or union type `Puzzle2dRepresentation` in this scope | 58 | ...   representations: vec![Puzzle2dRepresentation { id: "r1".into(), name: "mesh".into(), url: "u".into(
   59 cannot find struct, variant or union type `Puzzle2dHandleTemplate` in this scope | 59 |             handles: vec![Puzzle2dHandleTemplate {
   71 cannot find struct, variant or union type `Puzzle2dAttribute` in this scope | 71 |             attributes: vec![Puzzle2dAttribute { id: "a1".into(), key: "k".into(), value: "v".into(), def
   72 cannot find struct, variant or union type `Puzzle2dAuthor` in this scope | 72 | ...   authors: vec![Puzzle2dAuthor { id: "u1".into(), name: "Ada".into(), email: "a@b.c".into(), role: So
   74 cannot find struct, variant or union type `Puzzle2dCatalogHandleKind` in this scope | 74 |         handles: vec![Puzzle2dCatalogHandleKind {
   85 cannot find struct, variant or union type `Puzzle2dCatalogEdgeKind` in this scope | 85 | ...   edges: vec![Puzzle2dCatalogEdgeKind { id: "link.e".into(), name: "Link".into(), label: "Link".into(
   86 cannot find struct, variant or union type `Puzzle2dCatalogWireKind` in this scope | 86 | ...   wires: vec![Puzzle2dCatalogWireKind { id: "link.w".into(), name: "W".into(), label: "W".into(), des
```

## 3D (baseline only, no fixes yet)

The same carve and API-drift families appear: `crate::editor::...` / `crate::retained_command::...` paths not resolving inside carved crates (about 40 sites), missing test helpers (`unit_cube_mesh_buffers`, `with_puzzle3d_app_for`, `single_object_scene_json`, `object_flag`, `object_cores`, `PUZZLE3D_SESSION_SLOTS`, `DEFAULT_OVERLAP_BUDGET`), `Puzzle3dInference::infer` removed, `PluginApp` bound on `Box<VcsArtifactApp<...>>`, `Mutation::<...>::diff` as type, and `parry3d` method drift (`compute_local_aabb`, `contains_local_point`). 3D ownership compute was not built.

```
123 sites 24 files
## 3 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🧪️tests/🔬️unit/🦀️.rs
   41 cannot find function `outward_wound_unit_cube_mesh_buffers` in this scope | 41 |     let (positions, indices) = outward_wound_unit_cube_mesh_buffers();
   210 cannot find function `unit_cube_mesh_buffers` in this scope | 210 |     let (positions, indices) = unit_cube_mesh_buffers();
   219 cannot find function `unit_cube_mesh_buffers` in this scope | 219 |     let (positions, indices) = unit_cube_mesh_buffers();
## 2 s/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🖌️brush/🧪️tests/🔬️unit/🥦️suggestions-run-laws/🦀️.rs
   390 no method named `compute_local_aabb` found for struct `parry3d::shape::ConvexPolyhedron` i | 390 |                 let center = candidate.compute_local_aabb().center();
   394 no method named `contains_local_point` found for reference `&parry3d::shape::ConvexPolyhed | 394 |                         if other.contains_local_point(&center) {
## 22 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs
   16 cannot find function `unit_cube_mesh_buffers` in this scope | 16 |     let (positions, indices) = unit_cube_mesh_buffers();
   53 cannot find value `DEFAULT_OVERLAP_BUDGET` in this scope | 53 |         contact_tolerance: DEFAULT_OVERLAP_BUDGET,
   107 cannot find value `DEFAULT_OVERLAP_BUDGET` in this scope | 107 |         contact_tolerance: DEFAULT_OVERLAP_BUDGET,
   130 cannot find function `single_object_scene_json` in this scope | 130 |     let json = single_object_scene_json();
   161 cannot find function `single_object_scene_json` in this scope | 161 |     engine.set_scene(&single_object_scene_json()).expect("set_scene should succeed");
   177 cannot find function `single_object_scene_json` in this scope | 177 |     engine.set_scene(&single_object_scene_json()).expect("seed");
   179 cannot find function `unit_cube_mesh_buffers` in this scope | 179 |     let (positions, indices) = unit_cube_mesh_buffers();
   195 cannot find function `single_object_scene_json` in this scope | 195 |     session.set_scene(&single_object_scene_json()).expect("set_scene");
   197 cannot find function `unit_cube_mesh_buffers` in this scope | 197 |     let (positions, indices) = unit_cube_mesh_buffers();
   216 cannot find function `single_object_scene_json` in this scope | 216 |     session.set_scene(&single_object_scene_json()).expect("seed");
   269 cannot find function `unit_cube_mesh_buffers` in this scope | 269 |     let (positions, indices) = unit_cube_mesh_buffers();
   296 cannot find value `DEFAULT_OVERLAP_BUDGET` in this scope | 296 |         contact_tolerance: DEFAULT_OVERLAP_BUDGET,
   401 cannot find function `unit_cube_mesh_buffers` in this scope | 401 |     let (positions, indices) = unit_cube_mesh_buffers();
   508 cannot find function `unit_cube_mesh_buffers` in this scope | 508 |     let (positions, indices) = unit_cube_mesh_buffers();
   538 cannot find function `unit_cube_mesh_buffers` in this scope | 538 |     let (positions, indices) = unit_cube_mesh_buffers();
   555 cannot find function `seeded_cube_mesh_buffers` in this scope | 555 |     let (unseen, unseen_indices) = seeded_cube_mesh_buffers(97.0);
   566 cannot find function `seeded_cube_mesh_buffers` in this scope | 566 |     let (positions, indices) = seeded_cube_mesh_buffers(41.0);
   586 cannot find function `seeded_cube_mesh_buffers` in this scope | 586 |     let (positions, indices) = seeded_cube_mesh_buffers(53.0);
   608 cannot find function `seeded_cube_mesh_buffers` in this scope | 608 |     let (positions, indices) = seeded_cube_mesh_buffers(11.0);
   647 cannot find function `seeded_cube_mesh_buffers` in this scope | 647 |     let (positions, indices) = seeded_cube_mesh_buffers(149.0);
   678 cannot find function `unit_cube_mesh_buffers` in this scope | 678 |     let (positions, indices) = unit_cube_mesh_buffers();
   739 cannot find function `unit_cube_mesh_buffers` in this scope | 739 |     let (positions, indices) = unit_cube_mesh_buffers();
## 4 standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🧪️tests/🔬️unit/⏯️fill-run-laws/🦀️.rs
   4 unresolved import `crate::Puzzle3dPlaySnapshot` | 4 | use crate::Puzzle3dPlaySnapshot;
   65 cannot find module or crate `semio_framework_job_payload` in this scope | 65 |             assert!(page.len() <= semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES && page.len() <= TO
   103 this function takes 6 arguments but 5 arguments were supplied | 103 | ...ut=semio_framework_job::drive_step(job,cx,"puzzle3d-fill-run",semio_framework_job::InteractiveStage::
   470 expected a type, found a trait | 470 |     let outcome = protocol::Mutation::<Puzzle3dPlaySnapshot>::diff(&op, overlay);
## 4 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🧪️tests/🔬️unit/🦀️.rs
   136 cannot find function `mesh_lane` in this scope | 136 |         let lane = mesh_lane(&scene_snapshot);
   137 cannot find value `PUZZLE3D_FALLBACK_MESH_KIND` in this scope | 137 |         assert_eq!(&lane[..2], [PUZZLE3D_FALLBACK_MESH_KIND, VORTEX_MARKER_MESH_KIND]);
   139 cannot find function `world_meshes_json` in this scope | 139 |         let meshes: Vec<Value> = serde_json::from_str(&world_meshes_json(&scene_snapshot)).expect("meshe
   142 cannot find function `mesh_lane` in this scope | 142 |         assert_eq!(mesh_lane(&scene_snapshot), lane, "the lane is deterministic across calls");
## 2 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️action-producer/🦀️.rs
   22 mismatched types | 22 |         window_meta.view_state = Some(window_view());
   87 the trait bound `std::boxed::Box<VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>>: PluginApp` i | 87 | ...ations_on_this_thread(|| semio_plugin::artifact_app_laws::take_fixture_result_turn(&mut app, action_me
## 1 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️budgets/🎟️session-and-step-budget-laws/🦀️.rs
   56 mismatched types | 56 |         Some(measured_view_state()),
## 2 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️budgets/📷️camera-and-settings-laws/🦀️.rs
   11 cannot find value `PUZZLE3D_MESH_PAGE_SCAN_CHARS` in this scope | 11 |     let steps = page.div_ceil(PUZZLE3D_MESH_PAGE_SCAN_CHARS).saturating_mul(2).saturating_add(1);
   65 cannot find function `puzzle3d_action_uses_precompute` in this scope | 65 |         !puzzle3d_action_uses_precompute("registerBrushMesh"),
## 1 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️budgets/🗂️outliner-and-relocate-laws/🦀️.rs
   183 arguments to this function are incorrect | 183 | ...   assert!(Puzzle3dPlayApp::command_from_action(verb, Some((semio_framework_pack_json::to_dsl_value(&
## 2 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️panels/🧾️history-kit-close-laws/🦀️.rs
   78 expected a type, found a trait | 78 | ...tocol::apply_diff_cold(protocol::Mutation::<semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapsh
   117 expected a type, found a trait | 117 | ...   current = protocol::apply_diff_cold(protocol::Mutation::<semio_s_artifact_puzzle_3d_model::snapsho
## 1 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️panels/🪟️window-option-laws/🦀️.rs
   53 no method named `load_window_config_pack` found for struct `context::Puzzle3dApp` in the c | 53 |         reopened.load_window_config_pack(pack).await.expect("reload exact window pack");
## 1 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️task-resume/🦀️.rs
   25 mismatched types | 25 |         window_meta.view_state = Some(window_view());
## 31 andards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🍞️geometry-cache-and-step-budget-laws/🦀️.rs
   134 cannot find `retained_command` in `crate` | 134 |     use crate::retained_command::testing::PuzzleCommandWorkStep;
   183 cannot find `retained_command` in `crate` | 183 | ...), command, snapshot, config, &context, crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, tool_id);
   207 cannot find `editor` in `crate` | 207 |     Puzzle3dPlaySnapshot::new(crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&NAKAGIN_EXA
   239 cannot find `editor` in `crate` | 239 |     Puzzle3dPlaySnapshot::new(crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&CONCRETE_FO
   300 cannot find `editor` in `crate` | 300 | ...   let seed = Puzzle3dPlaySnapshot::new(crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot
   325 cannot find `editor` in `crate` | 325 | ...ect.vortices.iter().map(|vortex| crate::editor::puzzle3d::puzzle3d_vortex_full_id(&object.id, &vortex
   334 cannot find `editor` in `crate` | 334 |                 let from = crate::editor::puzzle3d::resolve_vortex_world_position(&scene_snapshot, &attr
   335 cannot find `editor` in `crate` | 335 |                 let to = crate::editor::puzzle3d::resolve_vortex_world_position(&scene_snapshot, &attrac
   362 cannot find `standards` in `semio_s_artifact_puzzle_3d_model` | 362 | ...   let bridged: semio_s_artifact_puzzle_3d_model::standards::v1::subsets::any::schema::SceneConfig = 
   67 cannot find struct, variant or union type `Puzzle3dSessionState` in this scope | 67 |         registry.check_in(stale, Puzzle3dSessionState { meshes: Some((7, "stale-meshes".into())), ..Defau
   73 cannot find value `PUZZLE3D_SESSION_SLOTS` in this scope | 73 | ...try_from(instance).expect("slot base") % PUZZLE3D_SESSION_SLOTS], "the retired generation is never han
   82 cannot find struct, variant or union type `Puzzle3dSessionState` in this scope | 82 |     registry.check_in(lease, Puzzle3dSessionState { meshes: Some((1, "x".repeat(PUZZLE3D_SESSION_PROCESS_
   83 cannot find value `PUZZLE3D_SESSION_PROCESS_BYTES` in this scope | 83 |     assert_eq!(registry.aggregate_bytes, PUZZLE3D_SESSION_PROCESS_BYTES, "a census exactly at the ceiling
   86 cannot find struct, variant or union type `Puzzle3dSessionState` in this scope | 86 |     registry.check_in(lease, Puzzle3dSessionState { meshes: Some((2, "y".into())), ..Default::default() }
   87 cannot find value `PUZZLE3D_SESSION_PROCESS_BYTES` in this scope | 87 |     assert_eq!(registry.aggregate_bytes, PUZZLE3D_SESSION_PROCESS_BYTES, "one byte past the ceiling is re
   97 cannot find type `Puzzle3dSessionSlot` in this scope | 97 |     let slot = size_of::<Puzzle3dSessionSlot>();
   98 cannot find type `Puzzle3dSessionState` in this scope | 98 |     let state = size_of::<Puzzle3dSessionState>();
   101 cannot find value `PUZZLE3D_SESSION_SLOTS` in this scope | 101 |     println!("[wave-P sizes] slot={slot} state={state} collision={collision} app={app} row={}", slot * P
   103 cannot find value `PUZZLE3D_SESSION_SLOTS` in this scope | 103 |     assert!(slot * PUZZLE3D_SESSION_SLOTS <= 8 * 1024, "the whole session row grew to {} bytes and is bu
   141 cannot find `retained_command` in `crate` | 141 | ...   let outcome = crate::retained_command::testing::step(original, work, command, snapshot, config, &i
   158 cannot find `retained_command` in `crate` | 158 | ...   _ => original.construct_boxed(Default::default(), |_scope| Ok((crate::retained_command::BoundedFir
   15 cannot find function `with_puzzle3d_app_for` in this scope | 15 |     let first = with_puzzle3d_app_for(session.clone(), &config, |app| app.geometry_jsons(&scene_snapshot,
   18 cannot find function `with_puzzle3d_app_for` in this scope | 18 |     let second = with_puzzle3d_app_for(session, &config, |app| {
   39 cannot find function `with_puzzle3d_app_for` in this scope | 39 |     with_puzzle3d_app_for(session.clone(), &config, |app| {
   43 cannot find function `with_puzzle3d_app_for` in this scope | 43 |     with_puzzle3d_app_for(session, &config, |app| {
   46 cannot find function `with_puzzle3d_app_for` in this scope | 46 |     with_puzzle3d_app_for(Some((4_003_u32, Some("document-other".to_string()))), &config, |app| {
   49 cannot find function `with_puzzle3d_app_for` in this scope | 49 |     with_puzzle3d_app_for(None, &config, |app| {
   61 cannot find function `puzzle3d_session_registry` in this scope | 61 |         let mut registry = puzzle3d_session_registry().lock().expect("session registry");
   70 cannot find function `puzzle3d_session_registry` in this scope | 70 |     let mut registry = puzzle3d_session_registry().lock().expect("session registry");
   80 cannot find type `Puzzle3dSessionRegistry` in this scope | 80 |     let mut registry = Puzzle3dSessionRegistry::default();
   162 mismatched types | 162 |         Some(measured_view_state()),
## 7 rds/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🍳️context-menu-and-engagement-defect-laws/🦀️.rs
   135 cannot find `editor` in `crate` | 135 |     use crate::editor::puzzle3d::commands::engagement_submit::PUZZLE3D_ENGAGEMENT_VERBS;
   240 cannot find `editor` in `crate` | 240 |         fallback.iter().all(|window_id| !crate::editor::puzzle3d::puzzle3d_window_id_is_kind(window_id))
   457 cannot find function `object_flag` in this scope | 457 |     assert_eq!(object_flag(&app, &object_id, "hidden"), Some(false), "the show write must reach the docu
   475 cannot find function `object_flag` in this scope | 475 |     assert_eq!(object_flag(&app, &object_id, "locked"), Some(true), "an explicit lock write lands while 
   479 cannot find function `object_flag` in this scope | 479 |     assert_eq!(object_flag(&app, &object_id, "hidden"), Some(true), "the row's explicit id decides what 
   508 this function takes 3 arguments but 2 arguments were supplied | 508 |         assert!(Puzzle3dPlayApp::command_from_action(verb, Some(&semio_framework_pack_json::to_dsl_value
   513 cannot find function `object_flag` in this scope | 513 |         assert_eq!(object_flag(&app, &object_id, flag), Some(true), "{verb} sets {flag}");
## 1 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🕹️roster-and-read-laws/🦀️.rs
   205 the trait bound `&mut context::Puzzle3dApp: PluginApp` is not satisfied | 205 |     if let Some(page) = semio_plugin::artifact_app_laws::take_fixture_result_turn(&mut app, instance_id,
## 12 s/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🥓️segmented-export-and-leftover-defect-laws/🦀️.rs
   19 cannot find `editor` in `crate` | 19 |     use crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_inline_budget_bytes;
   20 cannot find `editor` in `crate` | 20 |     use crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_json;
   288 cannot find `editor` in `crate` | 288 |     let page = crate::editor::puzzle3d::precompute::PUZZLE3D_MESH_PAGE_BASE64_CHARS;
   290 cannot find `retained_command` in `crate` | 290 |     assert!(steps <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS, "a whole page validates in {ste
   291 cannot find `retained_command` in `crate` | 291 |     assert!(page + 8 < crate::retained_command::PUZZLE_COMMAND_RAW_BYTES, "a page's two payload streams 
   304 cannot find `editor` in `crate` | 304 |     let digest = crate::editor::puzzle3d::precompute::brush_mesh_digest(&positions, &indices);
   289 cannot find value `PUZZLE3D_MESH_PAGE_SCAN_CHARS` in this scope | 289 |     let steps = page.div_ceil(PUZZLE3D_MESH_PAGE_SCAN_CHARS).saturating_mul(2).saturating_add(1);
   303 cannot find `standards` in `semio_s_artifact_puzzle_3d_model` | 303 | ..._artifact_puzzle_3d_model::standards::v1::subsets::any::schema::precompute_model_tests::context::seed
   60 cannot find function `object_cores` in this scope | 60 |     assert_eq!(object_cores(&reassembled), object_cores(&live), "the streamed export reproduces every obj
   94 cannot find function `object_cores` in this scope | 94 |     assert_eq!(object_cores(&two).len(), 2, "distinct payload must carry two objects");
   101 cannot find function `object_cores` in this scope | 101 |     assert_eq!(object_cores(&imported_projection(&settled)).len(), 2, "distinct leftover import must car
   343 cannot find function `puzzle3d_action_uses_precompute` in this scope | 343 |         !puzzle3d_action_uses_precompute("registerBrushMesh"),
## 4 ds/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🥩️opening-camera-settings-and-catalog-laws/🦀️.rs
   303 cannot find `editor` in `crate` | 303 |     let document = peak_of(&|| drop(Puzzle3dPlaySnapshot::new(crate::editor::puzzle3d::puzzle3d_snapshot
   313 cannot find `editor` in `crate` | 313 |     let document=crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&default_scene_snapshot()
   315 cannot find `editor` in `crate` | 315 |     let admitted=crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&host).expect("the twin r
   318 cannot find `editor` in `crate` | 318 |     assert!(crate::editor::puzzle3d::puzzle3d_snapshot_from_host_snapshot(&host).is_err());
## 10 🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🧈️relocate-clipboard-and-snapshot-defect-laws/🦀️.rs
   466 cannot find `editor` in `crate` | 466 |     use crate::editor::puzzle3d::commands::import_snapshot::puzzle3d_import_value;
   539 cannot find `editor` in `crate` | 539 |     use crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_refusal;
   540 cannot find `editor` in `crate` | 540 |     use crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_segmented;
   541 cannot find `editor` in `crate` | 541 |     use crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_segmented_budget_bytes;
   440 unresolved import `crate::retained_command` | 440 |     use crate::retained_command::PUZZLE_IMPORT_TOTAL_BYTES;
   542 unresolved import `crate::retained_command` | 542 |     use crate::retained_command::PUZZLE_COMMAND_OUTPUT_BYTES;
   384 cannot find `editor` in `crate` | 384 |     let distinct = crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_json(&puzzle3d_sc
   413 cannot find `editor` in `crate` | 413 |     let distinct = crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_json(&puzzle3d_sc
   467 cannot find `editor` in `crate` | 467 |     let payload = crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_json(&NAKAGIN_EXAM
   471 cannot find `editor` in `crate` | 471 | ...   crate::editor::puzzle3d::commands::export_snapshot::puzzle3d_export_json(&Puzzle3dSceneSnapshot::f
## 1 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🧾️inspector-and-manifest-laws/🦀️.rs
   233 arguments to this function are incorrect | 233 |     assert!(Puzzle3dPlayApp::command_from_action("noSuchAction", None, None).is_err());
## 1 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧰️prelude/🦀️.rs
   135 the name `SET_ACTIVE_UTILITY_ACTION_ID` is defined multiple times | 135 | use semio_plugin::SET_ACTIVE_UTILITY_ACTION_ID;
## 1 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🎬️action/🧪️tests/🦀️.rs
   8 associated function `frames_source` is private | 8 | ...   assert_eq!(Puzzle3dFocusSelectionWork::frames_source(&selected, row.get("target").unwrap().as_str().
## 1 🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🧪️tests/🔬️unit/🦀️.rs
   17 this method takes 2 arguments but 1 argument was supplied | 17 |             retirement.step(grant).expect("bounded window retirement turn");
## 6 🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs
   204 this function takes 6 arguments but 5 arguments were supplied | 204 | ...er|(codec.export)(PUZZLE_3D_SCHEMA, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true
   206 this function takes 7 arguments but 6 arguments were supplied | 206 | ...er|(codec.import)(PUZZLE_3D_SCHEMA, &dialect, &mut Some(database), encoding, &mut SqliteSnapshotContr
   228 this function takes 4 arguments but 3 arguments were supplied | 228 | ...er|Puzzle3dSnapshot::decode_sqlite_snapshot_native(&payload, &mut SqliteSnapshotControl::new(&mut cb,
   229 this method takes 4 arguments but 3 arguments were supplied | 229 | ...ll.encode_sqlite_snapshot_native(SnapshotEncoding::Binary, &mut SqliteSnapshotControl::new(&mut cb, s
   248 this function takes 4 arguments but 3 arguments were supplied | 248 | ...ng(|native_owner|Puzzle3dSnapshot::decode_sqlite_snapshot_native(&payload, &mut control, native_owner
   249 this method takes 4 arguments but 3 arguments were supplied | 249 | ...tive_owner|large.encode_sqlite_snapshot_native(SnapshotEncoding::Binary, &mut control, native_owner))
## 3 🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs
   36 no associated function or constant named `infer` found for struct `Puzzle3dInference` in t | 36 |     assert_eq!(Puzzle3dInference::infer(&snapshot).expect("valid materialized inference scene_snapshot"),
   41 no associated function or constant named `infer` found for struct `Puzzle3dInference` in t | 41 |     assert_eq!(Puzzle3dInference::infer(&Puzzle3dSnapshot::default()).expect("valid materialized inferenc
   47 no associated function or constant named `infer` found for struct `Puzzle3dInference` in t | 47 |     let inferred = Puzzle3dInference::infer(&snapshot).expect("valid materialized inference scene_snapsho
```

## Files created by L1

`l1-native.sh`, `l1-testkit-features.py`, `l1-migrate-effect-tests.py`, `l1-fix-args.py`, `l1-fix-take.py`, `l1-sites.py` (all in this folder). Logs under `🗑️generated/l1-*` are tool output and were removed.

## Suggested next steps

1. Free disk (or raise tmpfs) so `cargo test -p` can execute the crates that already compile (records, snapshot, mut-any-2/3/4/6/7/8/9/10, board-host): these validate the new fixture and the serde gates at runtime.
2. Fan the remaining 2D sites out by group above; the effect-API group is the largest and mechanical once the owner of `RetainedCloneCursor` derive confirms the intended `advance` shape for tests.
3. Start 3D from the `crate::editor` / `crate::retained_command` path group (about 40 sites) and the missing-helper group.
