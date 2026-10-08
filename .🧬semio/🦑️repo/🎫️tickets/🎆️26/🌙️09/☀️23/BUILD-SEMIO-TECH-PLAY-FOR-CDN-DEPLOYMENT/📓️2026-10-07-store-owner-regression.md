# Store Owner Regression Gate

The complete selected Store module ran through the registered kernel owner in the same initially cold task compiler generation. Actual exit 1: 371 tests ran, 343 passed, 28 failed, zero ignored, 845 other kernel tests filtered by the explicit Store module selection. Test runtime was 17.52 seconds. The original output is `🗑️generated/store-owner-suite-green.log`; its filename described the intended result, not the actual result. Both previously corrected snapshot-factory cases and all five retained-genesis/external-reader corpus cases passed. No overall Store acceptance is claimed.

## Complete Actual Failure Inventory

| Family | Failing Test |
| --- | --- |
| batch | `artifact_store_batch_publication_stages_two_hundred_mutations_into_one_ledger_slot_and_one_undo_step` |
| deferred | `a_deferred_local_step_yields_on_its_wall_deadline_before_its_operation_cap` |
| deferred | `a_deferred_remote_supersession_adopts_what_an_undeferred_one_adopts` |
| deferred | `a_local_edit_or_a_further_remote_change_restarts_the_deferred_replay` |
| deferred | `cancelling_a_deferred_replay_at_any_step_leaves_the_store_untouched` |
| outbound | `every_locally_authored_operation_is_announced_exactly_once` |
| clone | `retained_clone_preparation_store_lifecycle_matches_neutral_oracle` |
| admission | `retained_hydrators_refuse_actorless_history_from_the_neutral_law` |
| member | `retained_member_publication_preserves_order_group_identity_and_exact_maximum_grant_progress` |
| physical Pack | `sqlite_snapshot_native_physical_pack_record_preserves_literal_table_and_numeric_tags` |
| snapshot | `store_close_releases_a_returned_read_before_its_displaced_root` |
| supersede law | `a_refused_inverse_is_one_fatal_mutation_and_the_session_stays_repairable` |
| supersede law | `a_withdrawn_planner_folds_as_a_no_op_at_every_site_and_its_recorded_input_restores_it` |
| supersede law | `every_input_row_is_admitted_and_folded_as_the_table_says` |
| supersede replay | `a_member_document_with_supersessions_hydrates_to_its_superseded_state` |
| supersede replay | `an_interior_revert_keeps_the_early_exit_equal_to_the_full_replay` |
| supersede replay | `bounded_history_read_cursors_obey_the_neutral_law` |
| supersede replay | `convergence_early_exit_equals_the_full_replay` |
| supersede replay | `downstream_warning_error_and_fatal_outcomes_are_reported_per_mutation` |
| supersede replay | `finished_replays_commit_atomically_and_refuse_stale_or_blocking_ones` |
| supersede replay | `retained_hydration_preserves_the_original_opened_actor_across_stored_authors` |
| supersede replay | `supersede_replay_corpus_matches_the_store` |
| tool transaction | `a_transaction_streaming_while_a_local_step_waits_never_starves_it` |
| tool transaction | `batched_gestures_stream_into_one_open_edit_and_a_closing_gesture_commits_it` |
| viewer head | `a_finalize_as_a_new_alternative_moves_only_its_author` |
| viewer head | `a_port_rebinding_moves_no_content_and_keeps_a_finished_replay` |
| viewer head | `the_persisted_pair_restores_edits_history_edits_alternatives_the_head_and_warnings` |
| viewer head | `two_peers_on_one_folder_converge_through_an_open_history_edit` |

## Proven Shared Contracts and Repair Scope

Production Store construction creates a Report replay for foreign-step unit flags, calls `step` once, then `finish`. The actual stepper deliberately yields Pending for bounded physical retirement and convergence work even with a false deadline. Construction therefore refuses valid stored histories with `edit replay has not finished`. The narrow repair drives that exact replay to Finished, retaining the stepper's one-item cleanup and admitted byte demand. Several direct report fixtures also assume one-call completion. Their shared bounded fixture driver now requires the finished witness plus exact completed semantic total before calling finish; outcome, actor and final-state assertions remain intact.

Three locally authored batch fixture paths supplied a hardcoded publication actor while opening the Store under its local actor. The observed failures were NothingToUndo. The helper requests now use the exact Store opened actor, preserving explicit peer author isolation in other tests. This applies to retained-clone lifecycle, ordinary batch helper, outbound announcement and tool-transaction batch helper. The returned-read snapshot case independently counts one edited allocation and one exact retained genesis allocation; total retirement remains two, matching the observed original red count rather than suppressing it.

Deferred semantic progress may stay unchanged while a physical owner retires, because the current stepper returns Pending after cleanup. Those cases still need a complete progress/phase witness rather than merely relaxing the semantic assertion. The 200-operation batch and member-wire publication remain under their original grants and turn limits; pending phase/checkpoint diagnostics were added to the batch fixture failure message to identify the concrete blocking authority. The physical Pack disagreement concerns literal object-member order and requires an exact encoder/fixture contract investigation. Hydration rejection and review retirement are still unaccepted until exact causes and fresh runtime receipts are established.

All 371 selected cases remain a required regression gate. No test is removed, skipped or marked passing on source inspection alone.

## Canonical Terminal Retry

The complete original371-case selection is queued as session51062, generated log `store-owner-suite-terminal-retry.log`, using the same task-private ship native store and both Nx cache bypasses. No passing result is claimed. Cad full native session29950 is independently compiling current geometry source.

Confirmed production roots repaired before this retry:

- Construction drives its report replay to its exact terminal result rather than finishing the first Pending cleanup turn.
- EditReplay respects the supplied deadline after one exact retirement, diagnostic-settlement, or convergence unit. A false deadline continues; a genuine blocked external retirement still yields. Every child retirement retains its1-item/current exact-byte grant and complete/empty witness.
- The controlled Pack dynamic Object writer now preserves the authored intrinsic member sequence, matching the unchanged uncontrolled encoder. Only schema-owned maps sort. The independent SQLite literal-order oracle remains; reversed intrinsic objects produce distinct bytes while both encoders agree for that exact sequence.

Fixture roots repaired without changing operation/state/history assertions:

- Terminal replay fixtures explicitly drive their current retained cursor. Deferred fixtures distinguish semantic fold progress from actual diagnostic/convergence/cleanup-owner work, retaining monotone bounds, original per-turn budgets, original10,000turn terminal limit, cancellation and adoption equality.
- Locally authored publication helpers retain the actual Store opened actor, including outbound announcement and streaming transaction helpers. Explicit peer/foreign-actor cases remain.
- The third retained snapshot close fixture independently counts the exact genesis factory and edited snapshot factory. External result-head readers explicitly witness Blocked, return their exact owner, then resume to terminal emptiness.

Remaining exact diagnostics requested by the retry:200-item admission/fold phase and framework backing demand, retained member-wire phase, document hydration rejection cause, and atomic streaming inverse-owner append. Grants and timeouts were not increased.

## Ordered Intrinsic Value Authority Correction

Root's current GIS oracle established that native DslValue::Object carries an explicit member sequence. The controlled Pack dynamic writer sorted that sequence while the uncontrolled writer retained it; the mismatch is the actual codec defect. The uncontrolled path is preserved, and the controlled dynamic path now visits authored Object members directly. Schema-owned FieldValue::Map still sorts canonically. The Store physical native fixture canonicalizes only its declared map owner before encoding; reversed intrinsic member order must produce distinct bytes, both encoders must produce equal bytes for that same reversed object, and exact decoding must retain the reversed field. The independent SQLite text-order oracle remains. This supersedes the earlier provisional uncontrolled-sort interpretation.

## Owner Payload Page Ceiling and Mutable Transaction Funding

The preserved 512-byte batch grant cannot fund a default 4096-byte payload allocation. Before production changes, the co-located neutral owner-page-ceiling corpus and native boundary test were authored: counts 0/1/63/64/65/127/128/129/199/200, exact one-below allocation rejection, ordered concatenation, exact admitted/returned-byte equality, and default 4096-byte preservation. Ajv and fast-json-patch independently check the same ordered array. Source oracle session40261 finished exit0 with one passed case; native Value boundary and whole Store results remain pending.

PagedList now selects its payload page ceiling before acquiring backing, retains fixed fanout and exact metadata charging, and preserves that authority when cloned. Store staging uses the actual admitted byte grant; open transaction inverses fund full owner pages, closed inverses retain exact final extents. PreparingCursor funds at most one genuine destination inverse allocation per turn before any atomic transaction append. Neither current content nor edit rows change during that funding. An allocation exceeding the admitted grant produces typed VcsError refusal rather than repeated assumed progress. Existing inverse order/content, publication atomicity and bounded disposal remain asserted by the full371 gate.

Actual page-ceiling independent source oracle: scoped framework-value Nx exec EXIT0; 1 passed / 0 failed, `[DEBUG] Paged list owner page ceiling=512 ordered third-party rows=10`. Native Value session17707 and full Store371 session51062 remain pending.

The two existing physical-owner neutral corpora explicitly change ownerWords from six to seven: the one additional native word carries the owner-selected payload ceiling. Their actual-height, no extra backing allocation, preserved pointer and exact-final-extent assertions are retained; the metadata Page shape, fanout and genuine allocation extents do not change.

The first terminal retry51062 stopped before test execution because the concurrent kernel package wrapper drifted to direct cargo test and passed --lib twice. Root restored canonical generic policy/level routing and proved5 wrapper cases independently. Whole371 no-fail-fast retry69383 uses current owner-page/funding/ordered-value/deferred source and original grants. No native receipt is claimed until this command completes.
# Value Capacity-bearing Fixture Census

Actual expanded native88452 EXIT1:21 run /20 pass /1 fail /142 outside filter. The new owner-selected512-byte payload ceiling case passed with DEBUG10 boundaries/10 ordered rows, exact one-below admission, typed invalid ceilings and exact retirement. Remaining failure retained_paged_list_adopts_completed_owner_larger_than_copy_budget reports zero progress under copy64/capacity4096. RetainedFieldCursor lazily allocates a boxed StringCursor and its close_granted now requires complete size_of<StringCursor>() physical release from maximum_copy_bytes; that allocation exceeds64. This is an admission/physical-close contract question, not evidence that owner handoff should copy72 payload bytes. No grant, timeout or nonzero-progress assertion has been weakened.

Latest required native census: Value21/no-fail-fast88452, Store371/no-fail-fast69873, Cad45976707 and one-unit plugin53486 are active against this ticket's private cold-native store. These are pending, not green. Independent final Cad source67/318 and strict types are actual green; independent runner+fuel2 source cases are actual green. No repeated broad compile is launched while these owners progress.

Whole Store69383 actually exited1 before runtime with4E0425: the new before-step owner capture was accidentally placed into drive_local rather than the wall-deadline case that used it. The capture now sits immediately before that case's step_reprojection, preserving all existing progress/deadline/grant/terminal assertions. Full371 retry69873 is queued in store-owner-suite-page-ceiling-full-retry.log; no new Store runtime success is claimed.

Initial native retained_paged_list17707 actually failed its first of10 cases;9 were not run. DropProbeCursor supplied only cold close_step, so the current explicit close_granted trait correctly refused UnsupportedOwner. Expanded list::48007 actually failed its first of21 cases;20 were not run. DerivedScaffoldOwner expected WorkLimit but its custom child lacked the same granted authority.

The co-located fixture cursors now implement explicit grants: DropProbe retains exact controlled close/binding retirement, InsufficientScaffoldChild refuses grants below its128-byte release requirement with typed WorkLimit, and NonconformingRetirementChild deliberately reports all three dimensions one above the grant. The over-grant test preserves exact rejection, child-value ownership, source index and terminal-close assertions while naming the current retained-clone grant. No production close default was weakened. Full21-case retry88452 uses --no-fail-fast in value-page-owner-capacity-close-full.log; registered11.35 has the same flag.


## Current Whole Store Receipt and Release Authority

The registered uncached whole-owner retry ran all 371 selected cases: **364 passed, 7 failed, 845 outside the selection**. No fail-fast cutoff hid other selected cases. The log is generated/store-owner-suite-page-ceiling-full-retry.log.

| Actual failing case | First current witness | Owner |
|---|---|---|
| deferred_reprojection_tests::cancelling_a_deferred_replay_at_any_step_leaves_the_store_untouched | total.is_some at cancellation fixture line173 | Editors/plugins: bounded replay completion |
| retained_hydrators_refuse_actorless_history_from_the_neutral_law | admitted bob/carol row unexpectedly Initialization at unit9964 | Root: hydration retirement authority |
| retained_member_publication_preserves_order_group_identity_and_exact_maximum_grant_progress | Preparing stalled, checkpoint completedItems8195/completedBytes8194 | Editors/plugins: preparation grants |
| supersede_replay_tests::a_member_document_with_supersessions_hydrates_to_its_superseded_state | Initialization at fixture603 | Root: hydration retirement authority |
| supersede_replay_tests::bounded_history_read_cursors_obey_the_neutral_law | second Blocked after captured head reader released at fixture1319 | Editors/plugins: retained readers |
| supersede_replay_tests::retained_hydration_preserves_the_original_opened_actor_across_stored_authors | Initialization at fixture660 | Root: hydration retirement authority |
| tool_transaction_tests::a_transaction_streaming_while_a_local_step_waits_never_starves_it | waiting local step exceeds original32 turns at fixture507 | Editors/plugins: fairness under streaming |

The complete Value list selection produced 20 passes and one failure among21: the oversized completed child cannot release its physical cursor under64 bytes of copy credit. This is a real conflation of copy and release authority. The canonical repair adds maximum_release_bytes to RetainedCloneGrant and released_bytes to RetainedCloneProgress. Remaining parent credit subtracts every dimension independently. Physical cursor and retirement-frontier frees spend release credit; allocation still spends capacity credit; copying still spends copy credit. No capacity credit becomes release or copy credit.

A strict language-neutral release-authority schema and corpus preserve copy64/release4096, a128-byte cursor, and the exact127-byte refusal boundary. The independent Ajv/Uint8Array/Buffer/fast-json-patch oracle validates retained versus empty ownership. The native fixture witnesses zero premature destruction, exact128-byte release, no copy charge, and terminal closure. Both existing64-byte insufficient-scaffold laws keep64 release credit and their typed refusal.

Partitioned callers now cycle capacity, copy, release, each with its original per-turn byte maximum. Store checks refusal after one full three-axis cycle; its total byte checkpoint includes allocated, copied, and released bytes. This prevents admitting simultaneous budgets through one outer byte grant. Physical allocation observation assertions compare actual deallocations to released_bytes. Current proofs queued: independent source75335 and full unfiltered Value library48123; no passing receipt claimed.


The independent release source receipt is now actual **2 tests passed, 0 failed**, covering the512-byte page corpus and strict new release-authority corpus. Both printed their DEBUG witnesses; source75335 exited0. Full native Value48123 remains pending.

The generic one-unit native test compiled and executed one selected case, failing because the fixture dropped its returned retained PreviewReady payload. The fixture now closes each StepOutcome under1 item/4096 bytes and asserts terminal-empty before dropping it; job/work/remaining-fuel assertions are unchanged. Exact retry59506 is pending. No job protocol source changed for this fixture repair.

## Final Authority Retry Inventory

Full unfiltered Value48123 actually ran164 cases:161 passed and3 failed, none skipped. The new exact127/128 release-boundary law and original oversized-owner adoption both passed. The allocator law exposed BoxCursor.advance still charging its terminal120-byte child scaffold free to copying; it now charges release. Two refusal-oracle failures named the old three-axis message; their neutral message/display now state item, copy, capacity, or release. The original exact allocator, typed-kind/path, Ajv and SQLite assertions remain. Full164 retry80643 is pending.

The one-unit fixture59506 progressed past its retained PreviewReady closure and then correctly received Blocked because its sole completion cell had no mounted consumer. Production close_step explicitly requires that consumer. The fixture now retains the exact completion clone through bounded job/work disposal, then asserts unpublished output remains None. No production close behavior, timeout, fuel or grant changed. Exact retry41626 is pending.

Actual member-publication source79492 passed its independent Ajv/JSON/TextEncoder law. The8194-byte wire is consumed once for decoding and once for retained disposal, so the neutral fixture explicitly declares two passes plus the existing32 publication turns. Per-turn grant4096, exact pointer/group identity and atomic publication assertions remain.

The new schema-first reprojection authority counts completed mutations instead of structural checkpoint callbacks. Its neutral12-mutation/cap1 law preserves deadline/cancel checks, zero-cap pending and exact one-fold-per-turn. Final independent combined source49563 exited0:13 passed,0 failed,2429 assertions. It includes strict operation schema, fast-json-patch replay, and the history physical512-byte backing against explicitly declared4096 release bytes.

History read retirement now exposes its next physical byte demand without changing alias disposal. The neutral law retains one work item, explicit4096 release, exact512 allocation witness, captured head pointer/content, and original reader-blocking assertion pending actual native65124. Full broader Store store::5009 and full Cad98147 are pending; no final native success is claimed.

Final review corrected three shadowed history progress bindings: each now compares the reported released byte count against the declared4096-byte authority rather than itself. Native65124/5009 may have captured the earlier fixture, so accepted final receipts must include this tightened assertion.

Read-only current process census confirms live compilation rather than a stale owner: current rustc children compile Semio, DWG, PDF, GLTF, PNG and PPTX selected media artifacts with active CPU. Full Value, Cad, Store and focused history/command gates are preserved; no preparation lock was removed and no compiler owner interrupted.

## Complete Broader Store Receipt

Actual5009 EXIT1: **537 run,520 passed,17 failed,681 outside selection**. Full no-fail-fast log: generated/store-owner-suite-operation-authority-full.log. Original hydration failures, wire publication, streaming fairness and the new operation boundary test passed. The original371 subset is contained in this broader gate; no failures were filtered out.

| Failed Case | Actual First Witness | Repair Owner |
|---|---|---|
| durable_store_group_journal_commit_flips_one_shared_root_then_adopts_exactly_once | actual actor:owned-group-fixture versus expected local; destructor abort after primary assertion | Root |
| edit_message_clamp_settlement_cancels_at_every_owned_stage | settlement137 Option unwrap None | Editors/plugins |
| member_history_dictionary_is_atomic_and_bounded_by_neutral_records | actual1431 expected1403 wire bytes | Root |
| member_history_dictionary_retains_every_denied_owner_until_exact_close | actual407 expected379 wire bytes | Root |
| member_factory_selection_retains_input_through_denial_and_handoff | actual357 expected329 wire bytes | Root |
| member_factory_selection_uses_only_complete_closed_declarations | actual357 expected329 wire bytes | Root |
| member_history_verification_rechecks_every_owner_transition_and_retires_exact_bytes | actual315 expected287 wire bytes | Root |
| member_history_verification_retains_input_and_bounds_verified_handoff | actual315 expected287 wire bytes | Root |
| derived_clone_matches_clone_and_serde_oracles_with_distinct_capacity_credits | UnsupportedOwner: clone cursor has no capacity-bearing close authority | Editors/plugins |
| recursive_depth_envelope_accepts_boundary_and_rejects_the_next_box | typed field DepthLimit at intended boundary | Editors/plugins |
| recursive_partial_copy_cancels_within_the_declared_depth_envelope | original100000-turn close witness unreached | Editors/plugins |
| unit_generic_enum_and_recursive_records_remain_grant_bounded | original100000-turn copy witness unreached | Editors/plugins |
| lifecycle_fixture_is_schema_first | release constructor copy credit0 versus fixture4096 | Editors/plugins |
| a_deferred_remote_supersession_adopts_what_an_undeferred_one_adopts | progress1→3 exceeds mutation cap1 | Editors/plugins |
| a_local_edit_or_a_further_remote_change_restarts_the_deferred_replay | restarted reported progress exceeds cap2 | Editors/plugins |
| cancelling_a_deferred_replay_at_any_step_leaves_the_store_untouched | progress1→3 exceeds mutation cap1 | Editors/plugins |
| bounded_history_read_cursors_obey_the_neutral_law | inferred512 demand absent; exact actual demand census required | Editors/plugins |

The operation driver still reported edit-close units through the legacy ReplayProgress. Explicit operation authority now selects operation-counted progress before work, excludes edit-close units from total/done, and leaves generic deadline callback replay unchanged. The native neutral12/cap1 test now also requires total12 and each progress delta<=1. Existing progress/restart/fairness/cancellation bounds remain unchanged; actual retry required.

Full Value80643 actually exited1: **164 run,163 passed,1 failed,0 skipped**. Its remaining allocator120-byte deallocation occurs when generated enum construction moves each retained Box field into its final enum, freeing those scaffold boxes. The generator now admits their exact summed extent through release credit and reports it once, with zero payload-copy charge for the owned moves. Existing allocator/serde/source-boundary assertions remain; full164 retry14515 is pending.

Final full Value14515 actually exited0: **164 run,164 passed,0 failed,0 skipped**,5.139s native runtime. Exact enum scaffold release, real allocator birth/release, both typed refusal oracles and every list/ordered value law passed. The one-unit mounted-completion command41626 also exited0: **1 passed,1059 outside selection**, both domain-spends/scalar-fallback DEBUG observed1/remaining0. These are actual green receipts before the planned shared Job physical-page refinement.

Further broad Store roots authored after actual red: ordered-map clone now provides explicit granted close for key/value cursors and every native owned backing, using controlled retirement and independent capacity/release axes. Snapshot lifecycle schema/corpus/source now names the three-axis single4096-byte allowance; existing physical maximum is unchanged. Clamp settlement's native include pointed three ancestors away to an unrelated fixture; it now mounts its actual adjacent clamp law, preserving every cancellation stage and original1/7-byte grants.

## Actual History Extent Census and Fixture Correction

The focused diagnostic reached production retirement: demands={1,392,4096}, releases={0,5,7,8,16,21,25,392,4096},165 turns, headBlocked=false. The separately retained head preserved the exact Arc pointer and typed contents on every turn. The earlier fixture's512-byte inverse assumption was false; the actual page is4096. ReturnedSnapshotReadRetirement releases its own alias via Arc::into_inner(None), leaving the independent readable head live; it does not own authority to force that external alias closed. Neutral/native law now checks4096 observed demand within its unchanged4096 physical grant, requires exact readable head survival after cursor terminal, and explicitly retires that external owner afterward. Genuine exclusive-factory/shared-reader blocking laws remain intact. Focused native and independent replay source retries are pending.

Three-axis lifecycle combined source oracle actual exit0 (store-history-release-three-axis-source.log). Complete wider Store537 still requires retry after recursive owner contracts and root exact actor census repairs.

Final current real4096-page plus three-axis lifecycle independent combined source actual EXIT0:13pass0fail2438assertions, store-history-real-page-source.log. Native history and retained-clone diagnostics still pending; this source receipt is not a native readiness claim.

## Canonical Recursive Structural Ceiling

`RetainedFieldCursor` and `BoxCursor` each consume one structural level before driving the next owner. A chain with d boxes therefore requires2*d+1 levels including the terminal field. The previous laws mistook this declared structural ceiling for a box count:64boxes need129levels;256boxes need513levels. The new neutral strict corpus keeps original64/512 ceilings and proves31/255 accepted versus32/256 refused, plus zero and one boundaries. Existing nested fixture explicitly uses255boxes. No production depth guard was loosened. Native ordered equality and independent serde projection remain exact. The partial cancellation law preserves work1 and copy2 but closes under its explicitly declared65536 release axis; a2-byte release cannot deallocate an actual boxed cursor.

`recursive-structural-depth-source.log` actual uncached Nx exit0: one source case passed,613 assertions, six unrelated source cases filtered; strict Ajv and independent fast-json-patch preserve every ordered Unicode chain. Correctly partitioned native `store-retained-clone-args-current-retry.log` remains pending; no native receipt claimed.

## Actual Retained Clone Family Green

Corrected Nx argv partition in `store-retained-clone-args-current-retry.log` actually passed11/11 native cases,1211 outside selection,13.517 seconds native,4m44 Nx. This includes all seven structural chain boundaries, generic enum/recursive clone equality, partial cancellation, captured reader authority, ordered native maps and independent serde shape. Original work/copy/depth/capacity ceilings remain; explicit physical release is separate. Temporary stalled-turn diagnostic was removed after this receipt. Full Store family consistency remains required.

## Actual Captured History Read Law Green

`history-read-exact-extent-args-retry.log` uncached Nx exit0: one native law passed,1222 outside selection,0.041 seconds native,6m23 Nx. The original captured-head pointer/content and one-work/4096-release assertions passed, including the exact external-head readable witness after cursor terminal. No shared-reader production guard was relaxed.
