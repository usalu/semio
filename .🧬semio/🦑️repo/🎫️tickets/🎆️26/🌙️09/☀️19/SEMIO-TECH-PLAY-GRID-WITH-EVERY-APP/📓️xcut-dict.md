# 🧊️ xcut-dict — neural `Dictionary` ownership (cross-cutting bucket XCUT-DICT)

## 2026-09-22 (session 6)

Bucket owner report for `final Dictionary ownership must be explicitly retired or owned by a cold
boundary` (`🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️.rs:101`). Predecessor
STATUS.md was swept; this pass started from the framework code and the fleet logs.

## 1. The protocol, as the code defines it

`Dictionary::drop` calls `OrderedMap::release_shared()`
(`🧰️framework/🔨️modules/🌱️value/🗂️ordered/🦀️.rs:136`). That returns `Ok` in exactly two cases —
the map root is `None` (an EMPTY dictionary) or another `Arc` still shares the root — and `Err`
otherwise, which the drop turns into the panic. So the rule is precisely:

> **The LAST owner of a NON-EMPTY `Dictionary` must never be plain-dropped.**

Legal exits, all already in the framework:

| owner | exit |
|---|---|
| a `Dictionary` you hold | `into_retirement()` → `retirement::retire_value_cold` |
| any domain type | `neural_engine::ColdRetire::retire_cold(value)` |
| a scope/field you rebind | `std::mem::replace(&mut slot, next).retire_cold()` (`imperative_engine::replace_scope_cold`) |
| a value built then discarded | `ColdValueOwner` / `ColdDictionaryBuilder` / `ColdOwner<T>` |
| a mutation / a diff a generic seam minted | `protocol::Mutation::retire_cold` / `MutationDiff::retire_cold` (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:128,186` — the DEFAULT is a plain drop, each technology overrides it) |
| a type that is moved through too many seams to name one owner | declare `Drop` on the TYPE, as `imperative_engine::Step` does (`✏️s/🔨️modules/📜️imperative/⚙️engine/🦀️.rs:53`, with the rationale in its docstring) |

The discriminator behind every single red in the fleet logs: the law held a NON-EMPTY dictionary.
The sibling law that passed next to it used `Dictionary::new()` (empty root → `Ok`). Nothing about
the tests was "stale"; they simply never owned what they minted.

Nobody "owns the retirement" at a single call site by default. The protocol is: **whoever is the
final owner retires, and any type that cannot name one final owner declares a `Drop` boundary.**
The test harness is NOT special — `testkit`/`ColdOwner` only give tests the same explicit owner
production has.

## 2. (a)/(b) classification per law

### (a) PRODUCTION defects — fixed
| law(s) | defect |
|---|---|
| `os_spr::protocol_laws::assert_mutation_inverse_law` (used by every plugin) | the law MINTS `mutation.inverse(base)` — a whole `Vec<Op>` — and raises an outcome per step, then PLAIN-DROPS both. A generic seam that builds operations and throws them away, i.e. exactly what `Mutation::retire_cold`'s own contract forbids. Every technology whose operation owns a fail-closed root aborted inside the law. |
| imperative `run` command, `export_media("result:out")` | both call `ImperativeHost::run()` and drop the `RunResult`. In the wasm guest this ABORTS the app on every Run press / every `result:out` export, not just in tests. |
| `imperative_engine::RunResult` / `EffectLogEntry` | own `Dictionary`s with no cold boundary at all, so no caller COULD retire them correctly. |
| `🎬️sequence` `SequenceHost::replace_snapshot`, `set_step_params_json`, the working-scene `EditStepParams` apply/inverse, `SequenceRunState::advance`'s three scope rebinds | plain assignment over a live root (7 sites). PEER-OWNED guest code → proposed diff, §4. |

### (b) Laws that never owned what they minted — fixed test-side with the in-repo recipe
The recipe that already passes is `✏️s/🔌️plugins/🖨️raster/…/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`:
every constructed mutation ends in `protocol::Mutation::retire_cold(mutation)`, every displaced
projection in the crate's own `retire_*_snapshot`, laws go through the `_cold` twins.
Applied to: `edit_step_params_inverse_law`, `op_text_round_trips_edit_step_params`, the five
`edit_step_params/warns-that-step-1-already-carries-the-requested-params` fixture laws,
`host_runs_default_snapshot`, and the three `imperative_engine` executor laws.

## 3. Fixes landed (absolute paths)

Production, framework-general:
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs` — `assert_mutation_inverse_law` and `assert_mutation_inverse_law_cold` route every minted inverse through `Mutation::retire_cold` and every raised diff through `MutationDiff::retire_cold`.
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️tests/⚖️protocol-laws-unit/🦀️.rs` — NEW `ColdCounterMutation` fixture (fail-closed `Drop` + retirement counter) and the two laws `mutation_inverse_law_retires_every_operation_it_mints` / `…_cold_retires_…`.
- `/Users/ueli/Documents/semio/✏️s/🔨️modules/📜️imperative/⚙️engine/🦀️.rs` — `impl ColdRetire for EffectLogEntry` and `for RunResult` (deliberately NOT `Drop`: `🎬️sequence::retire_run_result_cold` destructures both field-by-field, which a `Drop` type forbids).

Production, imperative plugin:
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏃️run/🦀️.rs` — the `run` command retires its `RunResult`.
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` — `export_media("result:out")` retires its `RunResult`.

Tests:
- `…/📜️procedure/…/✏️editor/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` — `host_runs_default_snapshot` retires the result.
- `…/📜️procedure/…/🧬️schema/⚙️operations/🧪️tests/🔬️unit/🦀️.rs` — `edit_step_params_inverse_law` binds and retires the operation.
- `…/📜️procedure/…/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs` — `op_text_round_trips_edit_step_params` retires both ends.
- `…/📜️procedure/…/🧬️schema/🧬️mutations/🔧edit-step-params/🧪️tests/🧪️warns-that-step-1-already-carries-the-requested-params/🦀️.rs` — `built_outcome`, the inverse law and the canonical-JSON law retire every value they mint.
- `/Users/ueli/Documents/semio/✏️s/🔨️modules/📜️imperative/⚙️engine/🧪️tests/🔬️unit/🦀️.rs` — the three `executor_runs_*` laws retire their `RunResult`.

## 4. PROPOSED DIFF — `🎬️sequence` guest (peer slice S10)

**Full diff, backtraces and rationale: the TRACKED companion `📓️xcut-dict-sequence-diff.md`**
beside this file. (It was originally written to `🗑️generated/xcut-dict/proposed-sequence.diff.md`,
which the repo workspace-cleanup swept at ~16:05–16:25 on 2026-09-22 together with this bucket's
STATUS.md and both run logs — hence the tracked copy.)

Root cause, stated once: `StepParams`/`SequenceStep` declare `ColdRetire` but **no `Drop`
boundary**, unlike the sibling `imperative_engine::Step` whose docstring states exactly why the
boundary must live on the type. All five `Dictionary ownership` backtraces in `sequence-sequence`
have the identical frame 4 — `core::ptr::drop_glue::<StepParams>` — under a dying
`SequenceHostSnapshot` (`SequenceHost::replace_snapshot`, `set_step_params_json`) or a dying
`SequenceWorkingScene` held behind an `Arc<dyn Any>` inside
`ArtifactChild<SemioFlowSnapshot>`'s local owner, released by the STORE when it drops a displaced
`SequenceSnapshot`. That last one is unreachable from any call site in `🎬️sequence`, so **only the
`Drop` boundary fixes the bucket**; the call-site-only variant (7 × `mem::replace(…).retire_cold()`
at `✏️editor/🦀️.rs` lines 223, 545, 661, 1256, 2383, 2426, 2455) closes 3 of the 5.

## 5. Numbers (real runs, private `CARGO_TARGET_DIR`, play native mutex)

Both run logs lived under `🗑️generated/xcut-dict/` and were swept by the workspace-cleanup at
~16:05–16:25; the counts below are transcribed from them verbatim. A re-run should use
`CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/play-fleet/xcut-dict/target`.

**run1** (2026-09-22 12:06→12:36, play native mutex, private target dir, `CARGO_INCREMENTAL=0`,
`RUST_MIN_STACK=33554432`, `RUST_BACKTRACE=1`, `cargo test --no-fail-fast … -- --test-threads=4`,
nine `-p` crates, `--features semio-framework-os-kernel/protocol-laws`). Zero compile errors — the
framework law change is source-compatible with every technology in the workspace:

| crate | result | `Dictionary ownership` panics |
|---|---|---|
| `semio-framework-os-kernel` (`--features protocol-laws`) | **1121 ok / 0 failed** | 0 |
| `semio-framework-os-kernel-neural-engine` | **56 ok / 0 failed** | 0 |
| `semio-s-artifact-imperative-procedure` | 136 ok / 8 failed | **0** (was 8) |
| `semio-s-artifact-dag-dag` | 204 ok / 3 failed | 0 |
| `semio-s-artifact-note-note` | 394 ok / 2 failed | 0 |
| `semio-s-artifact-mathematical-equation` | 365 ok / 19 failed | 0 |
| `semio-s-artifact-reasoning-wires` | 176 ok / 13 failed | 0 |
| `semio-s-artifact-sequence-sequence` | 200 ok / 7 failed | 5 (peer-owned, §4) |
| `semio-s-imperative` | 4 ok / 3 failed | 3 (found by this run, fixed after it) |

The eight remaining `imperative-procedure` reds are OTHER buckets, all already red in the knowledge
agent's `🗑️generated/knowledge/pass1.txt` before this pass: the `pathRef: {"owner":null,"slot":null}`
vs `{}` fixture/`skip_serializing_if` bucket (3 `committed_json_is_canonical`), bucket 2
(`edit history insertion requires its exact mutation retirement factory`), the content-addressed
`flow`-handle churn in `delete_step_inverse_law`, and two backbone/undo laws. One of them,
`run_command_expands_scope_into_readable_rows_without_truncation`, is no longer a Dictionary abort
at all — it now fails its own content assertion (`json.contains("log.print")`), which belongs to the
`knowledge` agent.

**run2** (verification, same mutex/target dir/env,
`-p semio-s-imperative -p semio-s-artifact-imperative-procedure -p semio-s-artifact-sequence-sequence`):

| crate | result | `Dictionary ownership` panics |
|---|---|---|
| `semio-s-imperative` | **7 ok / 0 failed** (was 4/3) | **0** |
| `semio-s-artifact-imperative-procedure` | 136 ok / 8 failed | **0** |
| `semio-s-artifact-sequence-sequence` | 200 ok / 7 failed | 5 (peer-owned, §4) |

Fleet-wide, across the nine crates actually run: **5 `Dictionary ownership` panics left, all five in
the peer-owned `🎬️sequence` guest**; every other crate is at zero, down from 8 (imperative-procedure)
+ 3 (semio-s-imperative, found by run1) + the flow set the peer already fixed on 09-21.

## 6. What remains, and why

- `🎬️sequence`: 5 Dictionary reds (`replace_snapshot_preserves_next_serial_and_selection`,
  `repeated_drops_after_replace_snapshot_use_distinct_ids`, `set_step_params_json_updates_step_params`,
  `store_applies_and_undoes_step_create`,
  `…modes::edit::windows::main::config::tests::sequence_window_ownership_runtime_isolates_restores_and_resets_exact_windows`).
  All of them already use the CORRECT harness (`ColdOwner::new(SequenceHost::default())`,
  `new_sequence_store`); the panic is inside guest production code, and run2's backtraces put it
  at `drop_glue::<StepParams>` under a dying `SequenceHostSnapshot` / `SequenceWorkingScene` — the
  latter reached through an `Arc<dyn Any>` inside the framework's own `ArtifactChild` local owner,
  which NO call site can reach. Only the `Drop` boundary of §4 fixes it. Peer no-touch list →
  `📓️xcut-dict-sequence-diff.md`.
  (`sequence-sequence`'s other 2 reds, `import_media_steps_in_*`, are a different bucket:
  `interactive-job.missing-reserved-builder`.)
- `🎭️playbook` / `🧩️procedural`: also peer-owned; not run in this pass. If they still show the panic,
  the §4 pattern (`std::mem::replace(…).retire_cold()` at every rebind, or a `Drop` on the params
  newtype) is the same fix.

## 7. Appendix — verbatim failure lists (the logs were swept; these are transcribed)

`semio-s-artifact-imperative-procedure`, run2 — `136 passed; 8 failed`, **0 `Dictionary ownership`**:

```
editor::procedure::component::unit_tests::two_instances_converge_disjoint_edits_via_backbone
editor::procedure::component::unit_tests::undo_after_add_step_restores_original_document_exactly
editor::procedure::modes::edit::windows::main::tests::run_command_expands_scope_into_readable_rows_without_truncation
standards::v1::subsets::any::schema::mutations::binary::tests::document_text_round_trip_with_applied_operation
standards::v1::subsets::any::schema::mutations::create_step::tests_rejects_a_duplicate_step_id_at_the_root_path::committed_json_is_canonical
standards::v1::subsets::any::schema::mutations::edit_step_params::tests_warns_that_step_1_already_carries_the_requested_params::committed_json_is_canonical
standards::v1::subsets::any::schema::mutations::reorder_steps::tests_warns_that_an_over_clamped_index_leaves_the_tail_step_in_place::committed_json_is_canonical
standards::v1::subsets::any::schema::operations::tests::delete_step_inverse_law
```

All eight were already red in `🗑️generated/knowledge/pass1.txt` before this pass. Their current
(post-fix) failure reasons, for whoever picks them up:

- the three `committed_json_is_canonical` — `pathRef` canonical form, e.g.
  `left: … "pathRef": Object {"owner": Null, "slot": Null}` vs `right: … "pathRef": Object {}`
  (bucket 1 of brief v2: a hand-written `ToValue` must skip `None`).
- `document_text_round_trip_with_applied_operation` —
  `apply: ValidationFailed("edit history insertion requires its exact mutation retirement factory")`
  (bucket 2: bare `ArtifactStore::new` in the test, needs an owners-installing guard).
- `delete_step_inverse_law` — content-addressed `flow` handle churn: the restored snapshot carries
  `imperative-flow-e377ceae1ea2a2bc` where the base carries `imperative-flow-3faf9c3be6d96916`.
- `run_command_expands_scope_into_readable_rows_without_truncation` — **no longer an abort**; it now
  fails its own assertion `json.contains("log.print")` ("main table lists default path steps after
  run"). A content defect for the `knowledge` agent.
- the two `editor::procedure::component::unit_tests::*` backbone/undo laws — untouched here.

`semio-s-artifact-sequence-sequence`, run2 — `200 passed; 7 failed`:

```
editor::sequence::component::unit_tests::import_media_steps_in_inserts_a_new_step_from_an_object_payload   ← other bucket
editor::sequence::component::unit_tests::import_media_steps_in_wraps_a_bare_scalar_payload                 ← other bucket
editor::sequence::component::unit_tests::repeated_drops_after_replace_snapshot_use_distinct_ids            ← XCUT-DICT
editor::sequence::component::unit_tests::replace_snapshot_preserves_next_serial_and_selection              ← XCUT-DICT
editor::sequence::component::unit_tests::set_step_params_json_updates_step_params                          ← XCUT-DICT
editor::sequence::modes::edit::windows::main::config::tests::sequence_window_ownership_runtime_isolates_restores_and_resets_exact_windows  ← XCUT-DICT
standards::v1::subsets::any::schema::operations::tests::store_applies_and_undoes_step_create               ← XCUT-DICT
```

The `import_media_steps_in_*` pair fails with
`Fault { code: "interactive-job.missing-reserved-builder", message: "media port 'steps:in' is
registered but has no concrete resumable importer" }` — not this bucket.

`semio-s-imperative`, run1 `4 passed; 3 failed` → run2 **`7 passed; 0 failed`**. The three were
`engine::tests::{executor_runs_steps_in_order, executor_runs_control_if_then_branch,
executor_runs_control_repeat}`, each aborting at
`drop_glue::<semio_s_imperative::engine::RunResult>` — found by run1, fixed test-side, green in run2.
