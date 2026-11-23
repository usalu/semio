# 🧵️ Job stall root cause, sourcing curation tests (2026-11-23)

## Finding 1: typed-operation "never settles" loops were turn-less test drivers

Instrumented `hoisted_tests_3::curation_set_count_supports_both_delta_and_absolute_value`.

| Layer | Measured |
| --- | --- |
| `typed_operation_diagnostic` | op in `Worker` stage, `session_rejected` with `admission_pending` |
| `WorkerJobSessionAdmission::advance` | stuck at stage 1 with a full fresh grant, nothing charged |
| `OriginalWorkerPageReservation::admit_step` | stuck at `begin_staged_page_funded` (`staged=None`, `pages=0`) |
| gate | `StepContext::payload_page_granted` is set by the first page action of a turn and cleared only by a turn renewal |

The curation test `settle()` (and three more hand-rolled loops in the editor unit tests) ran `maintenance_step` and `advance_typed_operation_publication` on the fixture's one long-lived context and never began a new host turn: no `fixture_renew_turn` (fuel, grant, payload page grant, receiving scope) and no drain of the per-turn ownership receipts (`take_typed_operation_preparation_receipt`; `advance_typed_operation_publication` returns early while an operation holds `publication_ownership_progress`). A real host turn is a fresh `StepContext`, so production never sees this.

The framework already ships the supported drivers: `artifact_app_laws::settle_registered_typed_operation` and `artifact_app_laws::settle_document_archive_load`. The curation tests use them now. This also removes class (2) `native construction exceeds original remaining Job grant` and `archive N never reached a terminal state` for the hand-rolled archive loops (their grant was never renewed, so the cumulative grant ran out).

File: `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`.

Result: `closes_mid_reserved_commit` passes; the typed-operation settlement assertions no longer fire; remaining failures are the real product defects below.

## Finding 2: hand-authored one-item preparations never declared foreign-step presence (browser stall too)

After Finding 1, `setFilterQuery`, `setContributions`, `sortTable` and `setFilterMinAvailability` stall with `interactive-job.publication-stalled`, the same signature as the browser probe (`one-item publication blocked`, `pending_artifact=true`, 4096 units).

Measured chain: `advance_apply_batch` returns `Blocked` in phase `Preparing` with `stage.folding` set and a fully funded grant (1 item, 497 KB copy, 16 MB capacity). The `Blocked` comes from `fold_batch_item` phase 12: the mutation (`SourcingCurationConfigMutation`, `may_emit_foreign_steps` defaults to `true`) requires `cursor.foreign_step_presence`, which is `None`. A backtrace of the `ArtifactStoreOneItemPrepared` birth shows the hand-written `SourcingCurationConfigPreparation::advance_step` calling `ArtifactStoreOneItemLiveAuthority::prepare_one_item`. Only sealer-driven preparers (`RetainedClonePreparation`) ever call `record_foreign_step_presence`; every hand-authored preparer (about 20 apps call `prepare_one_item`) left it `None`, so the fold blocked forever instead of failing. It is a silent infinite wait, not a funding shortfall.

Fix at the source: `prepare_one_item` (`seal_prepared`) now declares absence of foreign steps, since a hand-authored preparation computes its own post root and derives none; sealer-driven preparations keep declaring theirs through `record_foreign_step_presence` (they build through `seal_prepared_owned`, unchanged).

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/🦀️.rs`.

Verified natively: no `stalled` or `one-item publication blocked` lines remain in the full curation lib run.

## Results (curation lib, `--test-threads=1`)

| Run | passed | failed |
| --- | --- | --- |
| before (rust-suites report) | 63 | 33 |
| after Finding 1 | 74 | 22 |
| after Finding 2 | 80 | 16 |

The `original_host_configuration` heap witness `(63137, 47337)` vs `(3200, 0)` and `viewer_never_mutates` "captured shared original changed its Work or destination" no longer occur in the curation lib after Finding 1/2 (the mismatch came from typed-operation pages observed while the operation was wedged). Not re-run: the standalone `original_host_configuration` integration test and the hub tests.

## Still failing (not stall related)

1. Typed operations now settle but `curationAdd`, `curationSetCount`, drop/sort tests publish nothing: the settle receipt shows lanes `[Ui, Terminal]`, no `Artifact` lane, no document mutations (`InvocationResult` carries only the operation id). The job emits no mutation; this is the tool/job semantic side, not accounting.
2. `JSON parse work quote was not admitted` (`pack-json parse` via `AuthoringWallet::run`, pure unit test in the schema/inferences target and the `every_rendered_action...` law): the one-shot `parse()` wallet quote is refused. Not root-caused.
3. History acceptance (`history_edits_end_to_end`, conflict variant) and the example/archive loads (`a_saved_curation_archive...`, `retained_example_load...`) still fail after settlement (load names the app-minted id; stock empty).
4. UI arena cascade and SQLite quoted-allocation refusals: owned by others.

A sourcing wasm rebuild is required for the framework fix to reach the browser.
