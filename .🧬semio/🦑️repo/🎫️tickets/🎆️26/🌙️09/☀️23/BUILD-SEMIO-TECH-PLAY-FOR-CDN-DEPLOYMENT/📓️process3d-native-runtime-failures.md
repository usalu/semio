# Process3D Native Runtime Failures

Actual full gate 26761 returned Nx exit 1 after 29m 10s: 375 run, 366 passed, nine failed, three ignored asset writers skipped. No failure expectations weakened.

```
thread 'editor::process3d::commands::world::tests::a_world_gesture_emits_one_stamped_edit_and_moves_the_viewers_cursor' (1520869) panicked at 🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌍️world/🧪️tests/🔬️unit/🦀️.rs:59:5:
assertion `left == right` failed: the cursor moves past the new step on the config lane
  left: []
 right: [SetCursor { value: Some(1) }]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        FAIL [   0.028s] ( 11/375) semio-s-artifact-process-process3d editor::process3d::commands::world::tests::a_world_gesture_emits_one_stamped_edit_and_moves_the_viewers_cursor
       START [         ] ( 12/375) semio-s-artifact-process-process3d editor::process3d::commands::world::tests::face_drag_negative_distance_yields_cut
test editor::process3d::commands::world::tests::a_world_gesture_emits_one_stamped_edit_and_moves_the_viewers_cursor ... FAILED

failures:
```

```
thread 'editor::process3d::component::unit_tests::context::history_edits_end_to_end' (1521624) panicked at /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧪️history-edit-acceptance/🦀️.rs:841:46:
process: the history edit of create-step (/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/🌱create-step/🪚️accepts on the initial document) breaks the generic mechanism: after /index = Number(Int(1)) the edited document does not survive save and load although the seeded one does: the archive load ends Fault: {"code":"plugin.internal.document-archive-replacement.closure-rejected","message":"document archive replacement failed its closure leg: recursive ownership closure validation rejected the candidate (Incomplete)","origin":"plugin","retryable":false,"scope":{},"severity":"error"}
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::process3d::component::unit_tests::context::history_edits_end_to_end ... FAILED

failures:

failures:
    editor::process3d::component::unit_tests::context::history_edits_end_to_end

```

```
thread 'editor::process3d::component::unit_tests::export_brep_out_returns_step_text_structured_payload' (1523679) panicked at 🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1192:66:
export brep:out: Payload("brep:out", "kernel replay failed")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::process3d::component::unit_tests::export_brep_out_returns_step_text_structured_payload ... FAILED

failures:

failures:
    editor::process3d::component::unit_tests::export_brep_out_returns_step_text_structured_payload

```

```
thread 'editor::process3d::component::unit_tests::vcs_artifact_app_production_maintenance_swap_is_authoritative_and_fail_closed' (1525219) panicked at 🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/../../././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs:292:5:
assertion `left == right` failed: production ingress carries every Process3d mutation variant
  left: 15
 right: 16
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::process3d::component::unit_tests::vcs_artifact_app_production_maintenance_swap_is_authoritative_and_fail_closed ... FAILED

failures:

failures:
```

```
thread 'editor::process3d::modes::edit::windows::workpiece::component::tests::render_world_scene_replays_the_timber_beam_instead_of_the_fallback_box' (1526012) panicked at 🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/🧪️tests/🔬️unit/🦀️.rs:34:45:
timber replay tessellates
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::process3d::modes::edit::windows::workpiece::component::tests::render_world_scene_replays_the_timber_beam_instead_of_the_fallback_box ... FAILED

failures:

failures:
    editor::process3d::modes::edit::windows::workpiece::component::tests::render_world_scene_replays_the_timber_beam_instead_of_the_fallback_box

```

```
thread 'editor::process3d::panels::document::tests::steps_past_the_viewers_cursor_render_pending' (1526232) panicked at 🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs:89:66:
document tree renders: PluginAssemblyError { code: "ui.tree-window.duplicate-key", message: "[tree-window] duplicate key \"process3d-play-document.stock\" in this panel body — two windowed containers share one window and one open state; give every windowed container its own node key" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test editor::process3d::panels::document::tests::steps_past_the_viewers_cursor_render_pending ... FAILED

failures:

failures:
    editor::process3d::panels::document::tests::steps_past_the_viewers_cursor_render_pending

```

```
thread 'standards::v1::subsets::any::io::component::binary::snapshot::mounted_snapshot_codec::retained_mounted_laws::mounted_region_has_no_batch_decoder_edge' (1527009) panicked at 🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/./🧪️tests/🔬️retained-mounted-laws/🦀️.rs:170:77:
mounted region start
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test standards::v1::subsets::any::io::component::binary::snapshot::mounted_snapshot_codec::retained_mounted_laws::mounted_region_has_no_batch_decoder_edge ... FAILED

failures:

failures:
    standards::v1::subsets::any::io::component::binary::snapshot::mounted_snapshot_codec::retained_mounted_laws::mounted_region_has_no_batch_decoder_edge

```

```
thread 'standards::v1::subsets::any::io::component::sqlite::snapshot::tests::public_cohort::sqlite_snapshot_process3d_public_declared_import_binds_independent_semantic_rows_and_exact_native_words' (1527376) panicked at 🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/../.././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🚦️public/🦀️.rs:20:261:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test standards::v1::subsets::any::io::component::sqlite::snapshot::tests::public_cohort::sqlite_snapshot_process3d_public_declared_import_binds_independent_semantic_rows_and_exact_native_words ... FAILED

failures:

failures:
    standards::v1::subsets::any::io::component::sqlite::snapshot::tests::public_cohort::sqlite_snapshot_process3d_public_declared_import_binds_independent_semantic_rows_and_exact_native_words

```

```
thread 'standards::v1::subsets::any::schema::inferences::component::tests::timber_document_replays_every_step_with_monotone_subtractive_volume' (1533585) panicked at 🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs:31:65:
replayed handle
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test standards::v1::subsets::any::schema::inferences::component::tests::timber_document_replays_every_step_with_monotone_subtractive_volume ... FAILED

failures:

failures:
    standards::v1::subsets::any::schema::inferences::component::tests::timber_document_replays_every_step_with_monotone_subtractive_volume

```

## Exact Owner Repairs and Rerun

The insert-step command now compares its already bounded post-insertion cursor directly with the prior config cursor. Clamping through the pre-insertion document truncated the new end back to zero and suppressed the required config edit. Four neutral cases cover empty, middle, past-end and unlimited cursors with independent serde_json exact output.

The pending-row law now creates one TreeWindows ledger per rendered body, preserving all three pending-count assertions. The mounted retained law reads the actual binary owner region. The SQLite import oracle resolves the existing physical support factory from the package manifest. The exhaustive production ingress assertion compares with the canonical fifteen-kind catalog; the removed sixteenth cursor verb belongs to config.

Removed exactly three ignored writers and their writer-only helper: `regenerate_example_fixtures`, `regenerate_step_mutation_vectors`, `regenerate_machine_stock_cursor_mutation_vectors`. Every read-only semantic, inverse, codec, asset and kernel law remains. No writer was executed and no asset was generated.

Full long native retry 21860 is active at `🗑️generated/process3d-owner-runtime-full.log`, using the same task-created cold compiler store, both Nx caches bypassed, full lib, ignore-default-filter, no-fail-fast and console capture. New case raises the read-only inventory to 376. Exact closure missing/orphan branch traces are temporarily gated behind the existing kernel protocol-laws dev feature and will be removed after diagnosis. Parent owns shared replay investigation; no green whole-suite claim yet.

## Archive Boundary Trace

Read-only source trace: Process declares stockSolid/steps/toolSolids through ArtifactSchema, emits deterministic content-addressed handles from inline stock/step payloads, and already supplies exact child packs for each current coordinate in genesis_process3d_child_pack. The shared seed_genesis_children and follow_derivable_children open those packs through declared member schema/factory with stamped parent/slot/child identity. The shared completed archive path hydrates/folds the candidate parent before completing missing candidate members through the same hook, then validates the exact recursive roster.

The shared ownership validator can return Incomplete for either a missing reachable artifact identity or an indexed member left unreachable from the current root. Both branches now have temporary feature-gated diagnostic statements. archive_member_entries serializes every held live member, while follow_derivable_children may defer coordinate following under child retirement/admission pressure. This is a source hypothesis, not the observed cause; no roster filtering, validator weakening, ambient cache, replacement alias or materialization behavior was changed. Flow sibling Incomplete was independently diagnosed by its owner as a test fixture saving member-less document_pack rather than document_archive, and that owner is repairing the test boundary. Process already calls document_archive in its acceptance law.

## Retained Timeline Replay Regression

The actual full retry `21860` exited 1 before runtime: the new independently owned timber scene test borrowed a `DslValue` into the consuming `FromValue` trait. The parent repaired that narrow test call and launched a focused timber proof. No replay or archive diagnostics were emitted by this failed compiler run.

The retained initializer's `process3d_apply_retained_mutation` contains a combined arm returning `None` for all seven timeline mutations. Its `ApplyForward` phase nevertheless commits each edit to applied history. Ordinary mutation application rebuilds `steps`, `stepPayloads`, and `toolSolids`; the initializer therefore reconstructs a different head and child roster from the same recorded history. This is a confirmed source defect; its exact relationship to the archive failure still requires the actual runtime diagnostic.

A permanent `retained_initializer_replays_every_neutral_timeline_mutation` law now drives the actual initializer with seven existing mutation quintets. Each uses one fuel per `StepContext`, hands off the candidate, proves terminal-empty ownership, closes the candidate through its registered disposer, and then compares the complete JSON snapshot to the independent `serde_json` reading of the neutral after asset. Candidate disposal occurs before assertions so a semantic mismatch cannot strand retained owners. No fixture or expected after file has been changed. Registered focused Nx red run `47307` is pending native preparation in `process3d-timeline-retained-red.log`.

The proposed repair uses a retained timeline cursor. It scans one source row, copies one resulting step, rebuilds one tool handle, or hashes one canonical flow node/edge per grant. It must replace all three timeline fields only after completion and retire displaced or cancelled payloads through the existing fixed retirement stack. It will preserve ordinary mutation semantics, actor/history ownership, one-unit cancellation, the existing publication lease, and existing timing/grant assertions.

## Actual Timeline Baseline and Authored Repair

The first focused compiler run `47307` exited 1 on a test-only API ownership typo: `snapshot_root()` returns an `Arc`, while `to_json_string` takes a borrowed value. The test now borrows its typed snapshot through `.as_ref()`; this does not retain a read alias beyond JSON encoding or loosen an expectation.

Semantic baseline `87523` then completed compilation and failed the actual production initializer law: **1 run, 0 passed, 1 failed, 376 outside the filter**, runtime 0.073 seconds, whole Nx target 8m49s. All seven candidates were handed off and disposed before comparison. Console proof includes create-step producing zero steps instead of one and delete-step retaining one instead of zero; rename/enabled/origin/measure/order projections retained their prior state. The first complete after-record equality fails on create-step. Actual failure is retained in `process3d-timeline-retained-semantic-red.log`.

The host-owned initializer now dispatches all seven timeline operations into a domain-owned retained cursor. It scans and copies individual rows, preserves canonical missing/no-op outcomes, remints analytic tool handles, and hashes reference STEP text in 4096-byte slices. It writes each Flow row through the existing controlled JSON writer (256 structural/character transitions per initializer unit), retires its retained source/output, and hashes output in 4096-byte slices. It preserves declaration-ordered Flow bytes and Float variants proven by the independent Node crypto oracle. Only a complete timeline replaces the snapshot's steps, payload rows, and tool handles. Cancellation closes partial writer/output and timeline owners; displaced timeline vectors/child handles close through the existing fixed retirement stack. Empty vector backing byte ownership is accounted across close pages. There is no whole-document diff/clone fallback or ambient registry.

The existing all-variant production-envelope expectation now explicitly includes its recorded deep step, renamed/disabled/origin/attach measure plus exact tool and Flow handles. Its prior expected snapshot encoded the faulty no-op behavior. Every existing digest, actor, publication, and timing assertion remains.

A second native law cancels at eight partial progress positions, proves zero-byte close performs no work, and closes through exact 64-byte grants to terminal-empty. The existing initializer launcher now runs both laws; current full Process inventory is 378 read-only native laws (three removed asset writers, three added cursor/initializer/cancellation laws). Green retry `93844` passed both scoped laws; the exact receipt is recorded below. The full current library receipt remains pending.

## Retained Timeline Initializer Focused Green Receipt

Actual session 93844 completed with Nx exit 0: 2 tests passed, 376 tests outside the filter, runtime 0.088 seconds, overall 2 minutes 15 seconds. The runtime console records all seven neutral timeline mutation after-records and all eight cancellation stops reaching terminal-empty with exact 64-byte release grants. The independent Node crypto oracle separately proved the seven declared Flow byte/hash identities. Log: generated process3d-timeline-retained-green.log.

The full current 378-case library retry is now running against the same task-created cold compiler outputs with both Nx caches bypassed. Its receipt remains pending; focused success does not imply full package success.

The cancellation law now reads its eight stops, zero-byte no-work grant, 64-byte release authority, maximum close steps, and exact nine-field empty terminal record from a closed neutral JSON fixture beside retained-laws. Native serde_json compares the actual terminal record with that fixture. The independent ticket oracle validates the same fixture with Ajv and rejects zero-release and retained-writer counterexamples; its expanded session 60562 completed with actual Nx exit 0 and all eight grant/stop inputs plus seven independent hash witnesses logged. Production behavior is unchanged by this fixture extraction.

## Physical Release Authority Review

The shared Value owner confirmed that allocation release authority is indivisible: a complete backing extent must fit the actual physical-release turn, and released bytes must be reported in that turn. The timeline's initial bookkeeping decremented backing byte credits across turns before freeing the complete allocation on the final turn. Its earlier 64-byte cancellation receipt proves its then-current terminal behavior, but does not validate this physical release contract.

A new assertion within the retained cancellation law constructs an empty one-item-capacity timeline backing, attempts release under extent minus one, retains the allocation, then releases it under its exact extent and compares the released-item/byte events. Session 18809 is the actual baseline; no production correction has been made before that receipt. The existing full session 97530 remains a baseline using current source, not a final consistency proof.

The correct design must expose the next physical demand, preserve work/copy credit separately from explicit physical release credit, and obtain admission for allocations larger than 4096 bytes rather than laundering that maximum. The existing initializer trait does not currently expose physical demand; the coordinating root is reviewing that generic boundary.

## Actual Physical Baseline and Corrected Owned Source

Focused physical baseline session 18809 completed with Nx exit 1: 1 test run, 0 passed, 1 failed, 377 outside the filter, runtime 0.037 seconds, overall 10m26s. The standard Vec backing extent was 200 bytes; under grant 199, the faulty retirement reported Pending released_bytes 199 rather than zero. The exact owner was fully drained before comparison, preserving clean failure evidence.

The timeline backing now remains owned below the complete extent, exposes that exact next demand, and reports its full physical extent only in its deallocation turn. The initializer external close preserves the supplied grant; its nested progress is forwarded. Erased terminal cursor Boxes also receive a separate complete physical grant and report their measured extent on the drop turn. The private replay pump admits exact release demand only within the existing 262144-byte domain allocation bound. Shared initializer/job/session demand propagation belongs to the root agent, not this source patch.

The neutral cancellation fixture separates one work item from a declared 4096-byte physical release floor and an explicitly funded larger allocation extent, bounded by 262144. It makes no claim to instantiate a 64-byte RetainedCloneGrant; this domain cursor uses the existing bounded typed copy helpers. The refusal witness still tests extent minus one, unchanged retained capacity/demand, zero progress under insufficient authority, and one full-extent release event. Focused green session 98021 is pending. Updated independent oracle 30099 completed with actual Nx exit 0: closed physical backing/cancellation inputs and all seven unchanged Flow byte/hash witnesses. Its 4096-byte declaration is the release floor; larger exact allocation demand remains separately admitted and is covered by the native law.

Full baseline session 97530 passed history_edits_end_to_end in 113.094 seconds and conflict_history_edits_end_to_end in 4.849 seconds. No closure rejection or diagnostic branch was observed there. Its exported timber geometry case still refused dowel-attach at exact boolean validation: non-adjacent face-136-0 and face-122-0 have zero separation away from a shared edge. The coordinating geometry owner received the actual DEBUG proof. Full package receipt remains pending, and this baseline predates the corrected physical source.

The two temporary protocol-laws closure diagnostic branches were removed after actual full-baseline history/archive success. The closure validator behavior was never changed. Its file is restored at those diagnostic hunks; the source will be compiled clean by current queued gates. Kernel replay diagnostics remain with the separate geometry owner.

The parallel geometry native compiler session 52794 supplied two actual E0599 diagnostics in the authored pump helper: ArtifactDocumentStoreDisposer close_step/terminal_is_empty require ArtifactOwnedDisposer in scope. The canonical trait is now imported as an anonymous method-resolution import; no alias or compatibility facade was added. That run exited 1 before geometry runtime assertions. Builder owns its retry; the physical source gate remains independent.


## Current Full Suite Partial Budget Failure

The unfiltered 378-case Process3D target session97530 exited1 after30m29. Its runtime reached the unchanged900000ms runner assertion budget while `vcs_artifact_app_production_maintenance_swap_is_authoritative_and_fail_closed` remained active. There is no final Nextest Summary. ANSI-stripped unique statuses prove57PASS,1FAIL,59START:58completed,1stillactive,319notstarted. Export geometry is the completed failure; its exact zero-distance face warning is retained for the builder-owned geometry diagnosis. History/archive edits and conflict edits passed113.094s and4.849s respectively. No whole-suite success is claimed.

A bounded2s stack capture during maintenance proves active decode release polling through FreshVcs demand → envelope release_step → decode worker drive. It does not prove finite progress. The runner, rather than an agent, ended the budget-exceeding runtime. Budget and assertions are unchanged.
