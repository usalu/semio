STATUS: PARTIAL. Store ladder: 58 of 72 crates' test targets compile natively and ran; 42 test binaries green, 66 tests red in 8 crates (causes below). 14 store crates still do not compile their tests, one more is blocked by the unported draw schema. `semio-framework-plugin-job` compiles and its lib test target runs green with 0 tests. The plugin family (plugfam/plugext crates) and replay-operation tests were NOT run. Logs: `C/🗑️generated/1101/testport-*.txt` (`sc-5` = last full native `check --tests`, `run-3`/`run-4` = test runs).

Tools: `C/1101-tools/testport/{t.sh,store.sh,chkt.sh,autofix.py}`, lists `store*.txt`. Method: `chkt.sh <tag> <list>` (cargo check --tests --keep-going, one pass for all test targets) then `autofix.py <log> 🧰️framework --apply` (inserts `.result` where rustc names a `RetainedWorkOutcome`, skips files newer than the log), then `LIST=<green list> store.sh <tag>` for the tests.

## Results
- run-3 (58 crates, killed by the 1 h `cargo:test` timeout inside sync-tests-2's slow `bounded_history_read_cursors_obey_the_neutral_law`): 42 binaries ok / 128 tests passed in ok binaries; red: codec-registry (2), command (2), initialization-replay (2), ledger (1), snapshot-clone (1), sync-tests-2 (see run-4).
- run-4 (tests-05/07/08/10, text-io, sync-tests-2 with that slow test skipped): 146 passed, 66 failed (counts include run-3 overlap for sync-tests-2).
- Verified green after my fixes: store-child, engine-branch, sync (25 incl. persistence-data-class), codec-factory, composition, durable-group-wire, engine-stage, backbone, presence-native-snapshot, message-ledger, and the rest of the 42.

## Production / infrastructure faults fixed
- `store/🔄️sync/🖥️native/🎭️actor/🔗️actor-hub-link-and-bootstrap`: `SharedUtf8::admit(..)` outcome unconsumed (native-only code, never seen by the wasm lanes).
- `os/🚪️io/📸️snapshot` + `💾️binary/📸️snapshot`: the `ArtifactPackMetadata` trait returns `BorrowedRecordSpecProducer` but clients could not name it (AGENTS: reexport explicitly); now `pub use`.
- `sync-native` Cargo: dev-dependency `semio-framework-os-store-sync/testing` (tests used `fixtures` only through accidental feature unification, broke when built alone).
- `engine/🌿️branch`: test binary lacked the `HeapWitness` global allocator its heap-receipt laws need (cfg(test) static added).
- `sync/🧪️tests/🔬️persistence-data-class`: fixture path pointed at the pre-move location.
- New `semio_framework_value::testing::admit_owned_source` (value `🧪️testing/🧾️wire`, testkit feature): drives the staged `RetainedCloneSourceAdmission`; replaces the removed `RetainedCloneSource::admit_owned` in 9 test sites.

## Test ports done (mechanical outcome API)
~320 `.result` insertions via autofix across 70 files; outcome-returning trait impls in store doubles (retirement, retained-edit, preparation, store, callers), value presence-retirement testing crates, hydration `UnitFactory`, canonical-edit factories, ledger/read/message-ledger/rejected-page-close closures, operation-wire admission closures, `close_retirement_recipient` callers (36), PagedBytes payload literals, history ledger iteration, `RetirementStep` arity, child receive rewritten on the `receive_step` pattern from `job/🛬️receiving/🧪️tests` (staged admission, `NativeReceive::receiver()`, `receive_original_retained_work`).

## Still red, with cause (not fixed)
1. Cold (unscheduled) authoring is refused by new "native requires original" guards: `native prefix replay requires its retained original preparation issuer` (replay-owned-state-operations, introduced 1e27248a0ba) and then `native edit diagnostics require their decoding continuation`. Hits tests-05 (18 tests) and sync-tests-2 (31 tests, plus 8 secondary "history ledger dropped before owner retired" panics). A peer was concurrently adding a `cold_prefix` authority in that file while I looked; I did not touch it. Decision needed: restore cold authority for all guards or give the fixture stores an installed preparation issuer.
2. Receive-helper tests on the not-yet-ported `Result` APIs: codec-registry native-snapshot (2) after my helper rewrite to `receive_retained_work` is unverified; command receiving (2) builds a bare `NativeDecodeControl::new` and now needs an original caller loan (`native decode work requires its original caller loan`).
3. initialization-replay (2): catalog test expects an error from a denied grant that now returns `Ok(Progress(0))`; replacement test `assert!(ready)`.
4. ledger (1): supersession cancel-prefix law sees `copied_bytes: 4095` where 0 is expected. snapshot-clone (1): fixed metadata closure policy no longer admits the demand (copy 5616, capacity 697, depth 2): fixture policy vs production demand drift.
5. tests-07 (3 sqlite snapshot laws): `original retirement requires its paid caller turn` and an `unwrap_err` on `Ok(Progress)`. tests-10 (1): resident backing exceeds 1024 close turns.
6. Compile-blocked (sc-5): tests-01/03/04/06/09 and sync-tests-1 (durable-group fixtures: `binding_step`/`retirement_progress` private, `advance_binding` and `take_bound` signatures, `generation()` now a method, `forwards` typed `DemoMutation`, `StepContextOwner` and `WorkerJobAdmissionContext` gone, `job.step(cx)` now needs a return claim, `RetainedPublicationRoot`, `PagedBytes` pointer/capacity assertions in backbone detach), durable-group-tests, member-open-tests (41), replay-operation (`PagedProjection<BoxedSnapshot>` bounds, step tests), presence (job API), operation-wire (9, partly fixed after sc-5, unverified), tests-02 (`derived_checkpoint_preview_admission_demands` not resolvable on `ArtifactStore<DemoSnapshot, DemoMutation>`: two crate instances), one-item-batch `mutation_apply` (needs staged admissions held in the state machine), read (2, autofix pending).
7. `semio-framework-os-store-member-open-store` dev-depends on `semio-s-artifact-draw-drawing-model` (feature testkit); that schema crate is unported (56 errors: `NativeDefault` lost `native_default`, retirement outcome `?`, lookup/owned). Excluded from the list.

## debugrm check
No compile error in the 72 crates' test targets traces to a removed `[DEBUG]` print (all errors are API migration, none "unused variable" errors; warnings only).

## Not done
Plugin-family tests (plugin-child, plugin-peer, decl-surface, inference gateway/receiving, command/typed-job crates of plugext/plugfam) and the plugjob-adjacent crates' tests were not compiled natively; replay-operation tests not run. hostfix owns plugin-host, host-shard, reactor-turn, actor-retained-turn.
