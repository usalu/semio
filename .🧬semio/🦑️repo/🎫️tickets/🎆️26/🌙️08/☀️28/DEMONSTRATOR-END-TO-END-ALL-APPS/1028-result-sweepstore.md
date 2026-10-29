# sweepstore result

STATUS: PARTIAL GREEN. Native `--lib --tests` and wasm32-wasip2 `--lib` pass for the unowned store crates; frontier = exclusive-owner crates + bound propagation (puzzleplugin).

Checks: C/🗑️generated/1028/sweepstore-chk1..13.txt (crate lists sweepstore-*.txt, runner sweepstore-run.sh).

## Fixed (crate: cause -> fix), all under 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/
- engine-core (store-cursor-operations): finish_derived_report_replay lacked `P: Clone, Mutation: ColdMutation<P>` -> added where clause (wasm).
- sync (backbone-worker-wire-and-history-envelopes): ArtifactDiff/InverseMutation payload is CausalPayload(PagedBytes) -> `try_from_slice` / `to_vec_owner` conversions.
- sync tests (document-socket-connect, document-echo-suppression), command receiving tests: Vec<u8> payload literals -> `CausalPayload::try_from_slice`.
- engine-branch: ids-tests needed dev-deps semio-framework-trace, serde_json -> added `[dev-dependencies]`.
- operation-wire tests, backbone original-outbox tests: duplicate `#[global_allocator]` (HeapWitness vs RequestedAllocator, which wraps HeapWitness) -> removed the redundant test-module allocator.
- paged-one-item mutation-apply: `Stage` lacked Debug -> derive.
- codec native-snapshot receiving test: InteractiveJobCloseStep has no fields -> `.progress()`.
- document hydration (types + retirement): type definitions/Drop/impl bounds now `P: StoreState`, `M: StoreMutationFor<P>` (hub required Send+Sync+OpBinary/OpText). Native lib+tests green.
- testing-doubles: added deps engine-command + space-member; `preflight` error type `&'static str`; one-item publication fns return `String` errors (match SpaceMember trait).
- testing-prelude test: `physical_test_close_grant` -> `semio_framework_value::testing::`.
- replay/🧪️tests/🔐️authority Cargo.toml (peer-created): `workspace` must be 10 `..` (root); 9 hits 🧰️framework/Cargo.toml and breaks every cargo run ("member of the wrong workspace"). Peer rewrote it to 9 once; I reset to 10.

## Remaining (not mine / diagnosis)
- os-store-engine (lib test): duplicate `ColdMutation`/`MutationDiff` import at lifted/🏪️store/🦀️.rs:113-114 (puzzleplugin). engine lib was 301 bound errors (StoreState propagation).
- testing-doubles: bound errors (P/Mutation StoreState/StoreMutationFor) -> puzzleplugin sweep.
- engine-command (puzzlehub), replay-operation tests, initialization-replay (fwwasm), codec-factory (flowext), presence `observed_step` private + codec-registry (playbook).
- plugin-window-config lib (fem) errors seen via test closure.
- Not defects: store-sync, store-sync-native, store-sync-wasm are not wasm32-wasip2 targets (tokio excluded for p2 by design; sync-native native-only); the sync-tests-* crates carry unconditional tokio "net" and are native-only. Exclude from the wasip2 lane (sweepstore-wasm-crates.txt omits `-tests`).

## SHARED CHANGE
none (document hydration bounds use puzzleplugin's StoreState/StoreMutationFor).

## CHECKPOINT 22:55 (stopped)
Native `test --no-fail-fast` task: queued ~50 min behind machine admission (load 40-70), never started; I killed my own queued wrapper (pids 40111/40120). No per-crate pass/fail counts exist yet. No further edits since the sections above (all compile-atomic).
Files added since the first result: testing-doubles Cargo.toml + 🌱️retained-edit + 🗄️store, testing-prelude 🦀️.rs, replay/🧪️tests/🔐️authority Cargo.toml (workspace path), see lists above.
NEXT STEP: `cd /Users/ueli/Documents/semio && nohup bash C/🗑️generated/1028/sweepstore-test.sh 1 sweepstore-test-crates.txt &` (58 store crates that pass `check --tests`; output sweepstore-t1.txt/.rc), then `grep -E '^test result|FAILED|failed' ` per crate and fix migration-caused failures.
