# 2026-10-10 Core tests (core-tests executor)

Tooling (outside ticket, `.🧬semio/🦑️repo/⚡️cache/play-fleet/core-tests/`): `build.sh <crate-dir> <name> [--lib|--tests|--test x] [--target t]` (slot-gated test build, json in `<name>.build.json`), `run.sh <name> [filter]` (each test in its own process, perl alarm `TMO`, `TGT=<lib target>`; `<name>.summary.txt`, per-test output in `<name>.res/`), `check.sh <crate-dir> <name> [args]` (slot-gated `cargo check --lib`). Private dirs `target` and `build` there.

## Final counts (native `--lib`, per-test processes, per-test timeout)

| crate | result |
|---|---|
| value | **298 ok / 0 fail / 0 timeout** (was 260 / 29 / 4 of 293) |
| pack-json | **88 ok / 0 fail** (was 85 / 1) |
| async | **93 ok / 0 fail** |
| tool-run (framework) | **44 ok / 0 fail** (was 42 / 2) |
| io-sqlite-snapshot | **53 ok / 0 fail** |
| graph-layout-run | **13 ok / 0 fail** |
| value-derive integration tests | all 7 integration targets ok incl. `neutral_owner` 4/4 (proc-macro lib has no tests) |

wasm32-wasip2 `cargo check --lib`: value, async, job, pack-json, tool-run, graph-layout-run, io-sqlite-snapshot all 0 errors. Plugin SDK crate `cargo check -p semio-framework-plugin --lib` native: **0 errors** (confirmed after the `ContextToolRunSources` payload change; last run after all my edits).

## Urgent items done (in order received)

1. **CancelToken RetireOwned** unified in async (`🛑️cancel/♻️retirement`), duplicate `♻️cancel-return` unmounted files deleted, `📜️script.ts` law names fixed; `CancelToken::is_same_node` added (see 8).
2. **OptionCursor** None bug fixed; fd-window-edit's `#[ignore]` removed (test not run: plugin crate tests not built here).
3. **RetainedClone for HashMap/BTreeMap/HashSet/BTreeSet** (`🌱️value/🧬️retained-clone/🧺️std-collections`): entry scaffolds and the entry insert move priced; std table/node backing unpriced (same convention as the existing `RetireOwned` for these types). 4 tests vs Clone + serde_json oracles.
4. **paged_list hangs**: stale test fixtures (child quotes ignored its alias binding; grants below the alias copy). All fixed.
5. **ArtifactCanonicalJsonTree** for `OrderedMap<V>` / `OrderedSet` in pack-json (proved vs serde_json and the types' `ToValue` wire).
6. **ToolRunView / ToolRunJobPort RetireOwned** in the OS plugin tool-run; `payload` is `Option<Arc<Vec<u8>>>` (also `ContextToolRunSources.payload`).
7. **U64Text** (finished before the hand-over to derive-ext): `ArtifactCanonicalJsonNode::U64Text(u64)` + `ArtifactCanonicalDecimalU64(pub u64)` newtype (tree, RetireOwned, RetainedClone, ToValue/FromValue as decimal string), test vs serde_json string output. Note `derive-ext` now owns the tree module; the names are in `pack-json/🛫️encode/🧭️tree` and `🛫️encode/🔣️scalar`.
8. **`WorkerJobSession::try_admit_owned` cancel check**: `WorkerJobAdmissionControl::admission_cancel()` (default `None`, `Some(original_cancel_token())` for `StepContext`); `try_admit_owned` (session, authority owner) and the preparation admission refuse with InvariantViolated when `params.cancel` is not the same node as the context token. NOT run: the job lib tests do not compile (stale `with_retained_work`, `try_new`, `Arc::as_ptr(&session.inner)` etc. predate the migration) and the ui host test crate was not built; lib check native + wasm is 0 errors.
9. **graph-layout-run** ported to the new job protocol (`step` -> `Result<Option<JobOutcomeBorrow>>`, `borrow_outcome`, grant-based `close_step`, close demands). Compute turns stage `Preview/Checkpoint/Fault` bytes that a `RetainedJobPublication` pays and lends; delivered publications retire from the next step's wallet. `testing::layout_run_drive`/`layout_run_close` and all unit tests ported (`Harness`-less: one `StepContextOwner` per run, delivered publication retired before the owner closes). Public `testing` feature helpers keep their signatures, so dag/jack/wires plugin tests compile against them again.
10. **`SqliteSnapshotControl::with_retirement_child`**: ledger is now a lent `SqliteSnapshotLedgerSlot` (same address in the child), callback and allocation port reborrowed, forwarded bytes and limits written back; refuses with OwnershipLimit without an active parent slot or with an occupied recipient. Added `NativeDecodeRetirementRecipient::is_reserved`. The `🫙️prefix` test passes (53/53).

## Design decisions in the value triage (evidence in code and fixtures)

- **Copy accounting**: native metadata, scalar leaf and `String` retirement are zero payload copy. Evidence: commit 48e4756 (Bytes cursor test "native metadata retirement invented payload copy", leaf test "native zero copy close", `factory/📬️closing.json` with `copyWords 0`, header copy 0 and a terminal grant with copy 0/capacity 0/depth 1). Only trivial-element `Vec<T>` payloads (`Vec<u8>`, `Vec<u32>`) still count `ProcessedBytes`. Stale tests/fixtures updated accordingly: physical-work, intrinsic, copy-demand, close-demands, decode/encode allocation headers (`ownerSlotHeaderBytes` 0), factory drain/metadata asserts, result transfer, queue, type, unit tests.
- **Controlled retirement depth** (`♻️retirement/🎮️controlled`): `next_depth_demand` is now `1 + nested` (per-turn depth) instead of `cursors.len() + nested`; the paged cursor frontier is heap-iterative, so tree depth no longer inflates the depth quote (fixes the pack-json deep-tree test without raising the 64 policy). Whole value suite stays green.
- **Overrun** (`🛬️decode`): a refused `advance` beyond the declared stage total does not commit (`completed ≤ total` is an invariant the continuation checks). Fixture case `parent-overrun-remains-visible` expected `completed 1` and the TS oracle (`🪆️stage/🟦️.ts`) now matches.
- **Mutex test**: the macOS pthread mutex lazily boxes 64 bytes on first `lock()`; the test now measures that platform release instead of assuming futex.
- **tool-run**: the ToolRun original test's copy denial now keeps capacity at the quoted value (a narrow child birth under a smaller copy grant is the deliberate narrow-work design); `owners` added to the retirement fixture and schema.
- **neutral_owner** integration test ported: `RetainedCloneSource::admit_owned` / `RetainedCloneBorrowAuthority::admit` (owning the source), explicit drains, grant copy = fixture work + alias copy, fixture capacity 4096.

## Not done / open

- Optional ledger item (exact release receipt for `FromValue::from_value_controlled` temporaries): not started.
- Test targets of **pack** (8 errors: stale `🎟️storage` test calling missing `admit_record_storage`/`retire_record_storage`, a non-Debug `Symbol`), **replication** (12 errors: text canonical seal tests `take_owner`/`Phase::Unwrap`, missing args in canonical identity/authority tests, dictionary source type annotation) and **tool-machine** (5 errors: stale `ActorId: From<&str>` and mismatched types) do not compile. Their libs are green; these were never part of the 29-failure baseline.
- **job** lib tests do not compile (stale `with_retained_work`, `try_new`, `next_close_demands`, `Arc::as_ptr(&session.inner)`); ui host `window-unit` try_admit_owned cancel test and the plugin crate's un-ignored `window_config_apply_conserves_custody_for_absent_optional_fields` were not run.
- Std map/set clone: backing unpriced (documented), `iter().nth(i)` entry access is O(i) per entry.

## Update (red test builds, parked for the release path)

State is consistent; everything below compiles.

- **pack** lib tests now build: **178 ok / 1 fail of 179**. Added `RetainedRecordBodyCursor::{record_storage_admission_demands, admit_record_storage, record_storage_retirement_demands, retire_record_storage}` in `🎒️pack/🌱️value/🎟️storage/🦀️.rs` (copy 0, exact capacity/release, one close transition per funded turn; the storage tests pass), `#[derive(Debug)]` on the borrowed-cursor `Symbol`, preflight test accepts the literal `native encoding canceled` refusal, operation-pages test funds the source alias copy, inline-symbols test funds the alias copy for the bind turn.
- Remaining pack failure: `format::retained_inline_symbols::tests::retained_inline_symbols_native_canonical_utf8_paged_indices_and_exact_granted_close` panics "source custody did not reach terminal" at `📐️format/🏷️symbols/🎮️decode/🧪️tests/🦀️.rs:29` (its `close` helper loop for the source custody; with the corrected alias-copy grant the later steps use `one_payload_turn(1, ..)`-style grants that cannot fund the 32-byte alias release/copy; next step: give the close helper and later grants the alias copy bytes).
- Not started yet: replication (12 test-compile errors), tool-machine (5), job lib tests, ui `original_clipboard_session_admission` and host `window-unit` runs, optional `from_value_controlled` release ledger.

## Update: `pack::record_spec_producer` / `reconstruct_record_controlled` / `record_controlled_semantic`

These three are not framework-pack functions and cannot be one generic implementation: each plugin's `pack` module (wires `📦️pack`, forms `📦️pack`, gis/obj/gltf/ply/stl `📦️pack`) is the artifact-specific adapter, i.e. `record_spec_producer() = <Record>::__dsl_spec_producer()` (no value argument, so a generic version would need a turbofish at every call site), `reconstruct_record_controlled` = row census + `<Record>::__dsl_from_record_controlled` + the artifact's own reconstruction/validation, `record_controlled_semantic` = the artifact's own projection into `EncodedRecord`s. Only mathematical lacked any such module (its `store::ArtifactNativeSnapshot` impl called a non-existent `pack::`), while it already owns `controlled::reconstruct`, `controlled_output::project` and `EquationPackRecord`. Fix: the equation binary snapshot impl (`➗️mathematical/…/➗️equation/…/💾️binary/📸️snapshot/🦀️.rs`) calls them directly (`EquationPackRecord::__dsl_spec_producer()`, `controlled::reconstruct(record, native, maximum)`, `controlled_output::project(self, maximum, native)`); no `pack::` reference remains there. `cargo check` of `semio-s-artifact-mathematical-equation` shows no error in that file; its 3 remaining errors are the owner's (`RetireOwned` conflict for `EquationCommand` in the plugin SDK `🦀️.rs:15083`, `EquationEdgeDsl: RetireOwned` unsatisfied in `🚪️io/📝️text/📸️snapshot/🦀️.rs:64`). Forms keeps its own module (nothing to delete in favour of a framework function; a shared generic would need a trait plus turbofish call sites in every plugin).

## Update: `RetainedClone for [T; N] where T: RetainedClone`

`🌱️value/🧬️retained-clone/🦀️.rs`: one array impl for every `T: RetainedClone` (Rust coherence forbids a separate Copy fast path next to it, so the fast path is a trait-level switch): `RetainedClone` gained `const BITWISE: bool = false` and `fn bitwise_array::<N>(&[Self; N]) -> Option<[Self; N]>` (defaults refuse; the scalar macro sets `true`/`Some(*source)`; `[T; N]` is bitwise iff `T` is). `ArrayCursor<T, N>` (now a real struct, no longer `ScalarCursor<[T; N]>`): bitwise element types clone in one funded copy turn (`size_of::<[T; N]>()` copy, same accounting as before); every other element clones through its own child cursor (bind, child turns, scaffold close, then moved into an inline `[Option<T>; N]` slot array, no heap) and the finished array is assembled in one funded move; cancellation retires child, pending element, slots, assembled output and binding through the granted close ladder. Tests (value `🔗️source/🧪️tests`): Clone oracle for `[u32;3]`, `[[f64;2];2]`, `[String;2]`, `[Vec<u8>;2]`, `[Option<String>;2]`, `[[String;2];2]`, `[String;0]`, and cancel-at-every-turn for `[String;3]`, `[Vec<u8>;3]`, `[u32;3]`. Results: value lib **300 ok / 0 fail**, value wasm32-wasip2 check 0 errors, plugin SDK native check 0 errors, `semio-s-artifact-stdio-pdf` `cargo check --lib --all-features` native **0 errors** and wasm32-wasip2 **0 errors**. Caveat: the former impl accepted `T: Copy + RetireOwned` without `RetainedClone`; arrays of Copy element types that only derive `RetireOwned` now need `RetainedClone` on the element (none surfaced in the SDK crate or pdf).
