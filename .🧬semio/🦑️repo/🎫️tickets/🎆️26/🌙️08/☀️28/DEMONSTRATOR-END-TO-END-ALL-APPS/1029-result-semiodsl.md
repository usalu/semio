STATUS: GREEN for the compile frontier. `s.stdio.semio` (TOP + all 19 macro crates, object/presentation/brep-viewer/kit-examples/mesh-editor/brep-io) checks clean native (`chk11`) and wasm32-wasip2 (`chk12`); the energy hub wasm check (`chk13`, with `-p semio-hub-energy-component`) shows NO stdio-semio error, its only failure is `energy-model-snapshot` (E0119 duplicate `ArtifactPackMetadata`, typedops). Tests (`chk20`): all 17 `fixture_honesty_law` + new `every_record_subset_mints_and_reopens_a_real_child_envelope` (18 subsets incl. flow) PASS; remaining failures are not subset codecs (see Open).

Check logs: `C/🗑️generated/1029/semiodsl-chkN.{txt,rc}` (N=1..8); scripts `C/1029-tools/semiodsl/{derive,fields,pack,deps,owner,regen,chk.sh}`.

## Recipe applied to value, table, text, audio, video, animation, model, cad, document, presentation, mesh, graph, image, brep, drawing, object, kit (flow keeps its own opener, see below)
1. Snapshot types (`🧬️schema/📸️snapshot/🦀️.rs`): `NativeReceive` + `DslRecord`/`DslScalar`/`DslEnum` (+ `RetireOwned` only where neither derive nor `artifact_retire_*!` existed); snapshot also `#[dsl(layout = "lines")]`; self-recursive types `#[value(receive_recursive)]`; enum-typed fields `#[dsl(statements, block)]`; `Default`-valued struct fields of foreign-default types get `semio_native_default!`.
2. Binary pack (`🚪️io/💾️binary/📸️snapshot/🦀️.rs`): hand `impl ArtifactPack` replaced by `semio_s_artifact_stdio_semio_base::semio_snapshot_record_pack!(Snapshot)` (derived record document in the semio pack envelope, the exact thing `OriginalRecordMemberSnapshotOpen` reads). Hand text `ArtifactDsl` kept (envelope id/extension unchanged).
3. Crate deps added to the snapshot-owner crates: `semio-framework-dsl-record`, `semio-framework-dsl-record-derive`, `semio-framework-value-native-owned`.
4. `semio_member_owner!` default `open` = `OriginalRecordMemberSnapshotOpen<$snapshot>` and emits `member_store_owners_copy_byte_demand` = `bounded_artifact_store_owners_source_demands::<S,M>()?.copy_bytes` (same catalog-priced pattern as draw/dag/flow).

## SHARED CHANGE
- `value-native-owned`: `NativeReceive`+`NativeDefault` for `f32`; `NativeDefault for (A,B)` (`📥️default`).
- `dsl-record` binding: new `👯️pair` module: `DslField`/`BorrowedDslField`/`DslSequenceView` for a homogeneous pair `(T,T)` (brep `BrepCoedge.prange: (f64,f64)`).
- `os-store` child receive (`🪆️child/🛬️receive`): `ArtifactChild` `NativeReceive`/receiver now `I: ?Sized` (object/kit children under the derive's `dyn NativeValueSource`).
- `s.stdio.semio` base: macros `semio_member_owner!` (default open + copy demand), `semio_snapshot_record_pack!`, `semio_native_default!`; geometry types derive `NativeReceive` + native defaults.

## Other fixes found on the way
- mesh/brep editors (`📬️preparation`): `PagedOneItemEdit` API drift (`post_mode`, `PagedOneItemPost[Read]`).
- semio flow subset opener (`SemioFlowSnapshotDecode`): `MemberSnapshotOpenOperation::step/take_ready` now return `Result<_, OriginalTurnFailure>` (flow keeps its hand streaming decoder + hand pack; not part of the 11).
- object/kit diffs: `#[child(kind = "s.stdio.semio")]` on `ArtifactChild` diff fields (derive now demands it).
- presentation snapshot + mutations: `#![recursion_limit = "256"]` (auto-trait `Send` over the DocBlock receiver cycle).

## Fixtures
All 39 committed `*.pack.semio` of the 17 converted subsets (examples, assets, mutate-test fixtures) regenerated from their DSL twin (`C/1029-tools/semiodsl/{fixtures,regen_pairs}.py`, temp test removed). Canonical-JSON snapshot fixtures untouched. Hand text DSL (`.dsl.semio`) unchanged and byte-identical.

## Tests run (`chk18`-`chk21`)
- PASS: `fixture_honesty_law` x17 (decode fixture == demo, encode(demo) == fixture bytes), `every_record_subset_mints_and_reopens_a_real_child_envelope` (new, replaces the mesh-only mint/reopen test; opens each subset's committed example through `OriginalRecordMemberSnapshotOpen`, flow through its own opener).
- FAIL, not caused by the pack/open change (verified by message):
  - `diff_grammar_conformance_law` in graph/kit/object/table: text diff grammar recognizer (`diff grammar did not recognize ...`); text diff codecs untouched.
  - editor `history_edits_end_to_end` / `conflict_history_edits_end_to_end` (audio, table, text, value, video): stack overflow in the default 2 MiB test thread; with `RUST_MIN_STACK=256M` the real panic is `live original Store slot remains allocated` in `plugin-ownership 🌱️initialization/🌱️genesis/📦️store` (framework).
  - top crate `shared_snapshot_read...`, `returned_read_leases...`, `supersession_...` (`InverseRefused ... native edit diagnostics require their decoding continuation`) and the 8 `sqlite_snapshot_semio_base_*` (`native decode has no available explicit retirement slot`): framework store/sqlite.
  Baseline comparison impossible: the tree did not compile before this change.
- Free-fixes in foreign crates (compile blockers for the test build): `stdio-contract-testing/registered-job` `wire()` now uses `RetainedToolWireInput::admit_original` (private `try_new`); `plugin-testing-laws bounded-viewer-fixture` uses `semio_framework_plugin_job::{Fault,reference_birth_fault}`.

## Open / decisions
1. OPEN DECISION (cross-language): ~20 `mutate-semio-*` Python/TS second implementations (`🐍️.py`, `📦️pack-carrier`, `🟦️.ts` in value, table, text, audio?, model, cad, document, object, image, graph, drawing, brep, kit, video, presentation, mesh, animation) re-implement the RETIRED hand binary pack and compare pack digests/bytes against the Rust subject; they now disagree. Options: (a) one shared Python/TS record-document codec driven by the exported derived RecordSpec (schema-first), (b) drop the pack carrier from those suites. Not done.
2. Dead hand binary codecs remain in each `🚪️io/💾️binary/📸️snapshot/🦀️.rs` (`encode_*_snapshot_binary`, `read_/write_*_lp` ...): compiler `never used` warnings list them; delete in a follow-up sweep (cosmetic).
3. Flow subset keeps its hand streaming opener + hand pack (only API-signature fix); migrate it to the same macro recipe later if desired.
4. Presentation (snapshot + mutations) needs `#![recursion_limit = "256"]`; further presentation consumers (hub components) may need it too.
5. Compiler image ENOSPC repeatedly killed runs (fleet pressure); `C/1029-tools/semiodsl/{retry,hub,test}.sh` retry/wait for space.
