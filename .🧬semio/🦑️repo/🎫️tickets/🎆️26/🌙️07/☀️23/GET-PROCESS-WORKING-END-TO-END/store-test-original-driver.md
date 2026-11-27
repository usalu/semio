# Store test driver on an original turn

## Approach
`store/test-support::dispatch_test_command` no longer calls a bare `ArtifactStore::dispatch`. Each command now runs inside a funded original turn of `semio_framework_job::OriginalJobFixtureCaller`:

1. admit a caller with a finite stock, a constant clock and a far deadline;
2. `caller.turn(stock, |cx| cx.with_original_work_loan(fuel, deadline, ...))`;
3. inside the loan, `native_encoder_work()` is lent to the `EntityIdentityAuthority` through a local `IdentityWorkLender` (`CallerIdentityWork`), exactly like the production reserved-commit route and the history-edit acceptance driver;
4. the dispatch future is polled to completion inside the loan, `consume_fuel(work.performed())` charges the Work back, then the caller is retired (`caller.retire(stock)`) so every command settles its own terminal closure.

Signature and all assertions of the callers are unchanged. `OriginalJobReturnOrigin` and `step(cx, claim)` were not needed: a work loan needs no return origin.

## Results per crate
| Crate | Result |
| --- | --- |
| `semio-os-store-tests-08` | 41 passed, 0 failed (viewer-head, supersede-law, field laws). Previously every dispatching law failed with `original work requires its original caller loan`. |
| `semio-os-store-tests-01` | Skipped, does not compile: 132 errors from a stale retained-clone cursor API (`advance`/`take` signatures) in `store/🧪️tests/🧬️retained-clone`. Unrelated to this change; it does not call `dispatch_test_command`. |
| `semio-os-store-tests-02` | Does not compile (113 errors): stale APIs (`close_step(grant)` now `(body, effect)`, `FactoryProducerReplyComplete` has no `.result`, `SharedOriginalStorage` vs `Arc`, `snapshot_owner` bound). Unrelated to the driver. |
| `semio-os-store-tests-03` | Does not compile (70 errors): stale store API (`install_document_store_owners_exact`, `begin_apply_batch`, `generation_now` moved behind the wrapper `.0`, `RetainedTextMutation: OpBinaryReceiving` missing). |
| `semio-os-store-tests-04` | Does not compile (29 errors): `close_step`/`box_close_step` signature migration. |
| `semio-framework-artifact-infinite-dag-vcs` | Passes (1 test). Builds and links the new test-support. |
| `semio-framework-artifact-space-collection` | Compiles; `retirement_law` and `persisted_receiving_law` (the `dispatch_test_command` users) pass. 6 of 29 lib tests fail, all in sqlite snapshot receiving, none via the store driver: `sqlite_snapshot_collection_original_receiving_{boxed_custody,cumulative,cancellation,complete,refusals}` (e.g. `canceled boxed body cannot publish`) and `sqlite_snapshot_framework_collection_public_borrowed_native_preflight_admits_actual_owner` (`snapshot retirement exceeds original physical grant`). Reported, not fixed: separate sqlite receiving/retirement product work, no relation to dispatch. |
| `semio-framework-artifact-space-space` | Does not compile (36 errors): `decode_sqlite_snapshot_native` gained a 4th argument and returns `RetainedWorkOutcome`. |
| `semio-os-store-testing-doubles` | Passes (0 tests, builds). |

Crates that do not compile are stale-API ports (older than this change); each needs its own migration. `semio-os-store-tests-01` was skipped on the coordinator's instruction (another agent ports it).

Earlier multi-crate run deadlocked on a broker over-estimate (fixed by the coordinator), so it produced no results.

## Product fixes
None. The driver exposed no product bug in the crates that compile; the six sqlite receiving failures in space-collection are listed above with evidence and are independent of it.

## Changed files
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧰️test-support/🦀️.rs` (driver, `CallerIdentityWork`, local `block_on_original_turn`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧰️test-support/📦️packages/🦀️rust/Cargo.toml` (`semio-framework-job` gains `features = ["testkit"]`)

## tests-01 port
`semio-os-store-tests-01` (`🏪️store/🧪️tests/🏙️bin-01`) now compiles: 132 errors to 0 (`cargo check --tests`). The stale retained-clone cursor API was the larger part (79 errors in `🧬️retained-clone`); 53 more sat in the five `🔬️unit` companions (`close_step(body, effect)`, `try_admit_one_returned(effect, ..)`, `retire_owned(&mut Option, effect, reply)`, `close_cursor_admit -> Result<_, RetirementPollExit>`, `close_granted(effect)`). Every assertion is kept; no `[DEBUG]` left.

### Port
- `retained-clone/🦀️.rs`: local extension trait `CursorTurns` gives cursors the old call shape (`advance(source, body, grant)`, `taken(grant)`, `began()`, `closed(grant)`) over the real original driver `semio_framework_value_clone_source_testing::drive::{warm, drive, drive_reply}` (reply-shaped `advance_into`/`take`/`begin_close_into`/`close_into` with `recover_*`). The ordered-map insert cursor is driven through `RetainedOrderedMapInsertCursor::new` plus `advance_into`/`take_into`/`close_into`. Sources are admitted through `admit_owned` and `admit_projected` on the thread bank (the old `admit_owned_source` measured the bank's own 680 byte birth); closes go through `drive::drive`.
- `bin-01/📦️packages/🦀️rust/Cargo.toml`: dev-dependency `semio-framework-value-clone-source-testing`.
- `unit/...completed-record...`: `ErasedSnapshotRetirement::close_step(.., effect.grant().maximum_copy_bytes, effect)`; the two `StepContext::new` decode tests now run on `OriginalEnvelopeCaller::admit_cleanup("failed-field")` with `renew_each_turn(4|1, 8192)` and `cancel().cancel_now()`, because a bare `StepContext` has no parent source supplier any more.
- `unit/...envelope-close...`, `...catalog-staging...`, `...one-item-publication...`, `...edit-retirement...`: `effect_tests::with_effect` / `semio_os_store_testing_doubles::store::closed_with` / `drive::drive` helpers (`settled_turn`, `observed_turn`, `driven_batch_close`), `UnitOwnedRetirementFactory` moved to the framed factory form, `UnitOwnedRetirement` gained `close_into`, `recover_close_into`, `native_header_release`.

### Product fix
`ArtifactStoreOneItemPrepared::edit` and `::edit_digest` were `pub`, which the sealed-prepared law forbids (accessors `edit()` and `edit_digest()` already exist and no other module reads the fields). They are private now, in `🏪️store/🎮️one-item/📬️publication/🦀️.rs`.

### Test run
`cargo test` of the crate: 40 passed, 15 failed, 2 not finishing (the two skipped ones spin at 200 percent CPU for 40+ minutes). Evidence of the rest:
| Failure | Evidence | Reading |
| --- | --- | --- |
| 9 retained-clone tests (`fixed_arrays...`, `utf8_copy...`, `derived_clone...`, `recursive_partial_copy...`, `recursive_depth_envelope...`, `production_snapshot_read...` and the two hangs `retained_vector_and_completed_optional...`, `unit_generic_enum_and_recursive_records...`) | `retained copy stalled under grant copy 512/7 ... next demand copy_bytes 1028/49`; `original fixture grant ... maximum_depth 8192 exceeds its finite Source stock (1024)`; `progress.fits(grant)` in the snapshot read cancel turn | The old tests hand a deliberately small per-turn copy credit; the cursors now quote body plus alias credit, so a grant below the quote makes no progress (the unbounded `loop`s spin). Needs per-turn grants derived from `advance_demands`, and depth envelopes within the 1024 bank stock. Test porting design, not a product bug. |
| `artifact_child_retained_clone_matches...` | `drive_reply` heap `(36, 0)` against receipt `(0, 0)` | The `ArtifactChildCloneCursor` advance allocates 36 bytes that its receipt does not declare. Likely a product receipt bug in `🪆️child/🧬️retained-clone`. |
| 4 `raw_replay_retirement_tests` | `replay retirement stalled under its own quote copy 48, release 4048, depth 2; step Progress(default)` | `doubles::close_granted` (owner-less pump poll) makes no progress at the exact quote of `EditReplay` custody. Probably the poll form of the replay owner needs a reply-shaped original invocation (the same owners need `drive` for the edit retirement port). Not fixed: `close_granted` has 89 callers. |
| `member_owned_batch_admission...` | `wrong.mutations.terminal_is_empty()` false after 1024 funded `close_granted` turns | `MemberStoreOwnedBatch::close_granted` returns no progress at the test grant. Needs its quote. |
| `one_item_publication_source_denies_generic_or_unbounded_shortcuts` | the law scans source text for `pub fn advance_apply_batch(`, `cancel_apply_batch(` and `pub fn close_step(&mut self, grant: ArtifactStoreOneItemGrant)`; the code moved to `⚙️engine/🧳️bundle` and the one-item close has no such function any more | Stale source scan, to be rewritten against the new owners. The sealed prepared part of this law passes after the product fix. |
| 2 `completed_envelope_registry_*` | `terminal ticket has no explicit native header lifecycle` | Fix added (`native_header_release` on `UnitOwnedRetirement`) but not verified: the re-run stopped on a concurrent edit, `semio-os-store-test-support` `🦀️.rs:217` (`Box::pin(store.dispatch(command, identity))` needs `'static`), not part of this port. |

## tests-02/03/04 and space-space port

Scratch logs live in the session scratchpad (`port/`), none in the repo. No `[DEBUG]` marker of this port remains.

### New shared helpers
- `store/test-support`: `with_original_identity(|identity| ...)` runs one closure inside a funded original turn (the former body of `dispatch_test_command`, now its only user); `block_on_original_turn` is public. Tests that held one cumulative `EntityIdentityAuthority` across several steps are wrapped once, so the cumulative-ceiling assertion and the `identity.encode` observation are kept.
- Ported call shapes: `close_step(grant)` becomes `envelope_close_turn(grant, |e| x.close_step(grant.maximum_copy_bytes, e))`; ladder-shaped owners go through `settle`; `admit_clone`, `take_returned_snapshot_read_retirement`, operation-wire `advance`/`close` go through `pump_turn`; `decode_sqlite_snapshot_native` through `semio_os_space_testing_sqlite_native_caller::slot`; `ArtifactDerivedHistoryPreview` literals gain `replacement`/`supersessions`; `Mutation` bounds become `StoreMutationFor<P>`.

### semio-os-store-tests-04: compiles, 92 of 96 pass (was 29 compile errors)
Product fixes made while porting:
- `discard_parsed_envelope` (`store/io/text/replay-ops-and-document-parsing`) popped every history ledger except `transitions`, so a refused parse that had a transition aborted in `ArtifactHistoryLedger::drop`. It now pops `transitions` as `retire_unadopted` does (`document_text_rejects_missing_metadata_and_dangling_transitions_without_synthesis`).
- `test-support` `RoundTripValueRetirement` dropped a bare `SharedOriginalStorage` ("shared original alias must retire or transfer"); a `RoundTripAlias` now retires it through its own counted-alias contract (`test_support_round_trip_helpers_pass_for_demo_operation`).
- `store/testing/doubles`: `DemoOneItemPreparationFactory` and `DemoPublishedRootWitness` lacked `close_state_work_demand` ("factory state lacks a native work quote") and `close_state_terminal_native_release_safe` ("terminal ticket has no explicit native header lifecycle"); both added (`interaction_store_default_seeds_from_hover_default_on_original_publication`).
Law changes (not weakened silently):
- `create_alternative_appends_commits_to_its_own_checkpoint_chain`: the trunk is implicit and never a row of `vcs.alternatives` (documented on `trunk_alternative_id`), so "trunk stays at one checkpoint" is now asserted as "checkpoints not owned by a branch (after the fork id) count one".
- `backbone_retirement_*`: the product test `memory_pair_ends_retire_while_their_peer_still_shares_the_queues` fixes the current law (a pair end releases its alias while the peer lives, the last end drains the messages). The test now asserts that law, then drains the peer end with the same bounded 17-byte grant and the same progress assertions.
- Batch tests now use a 64 KiB copy grant: the batch publication frame copy demand exceeds the 4096-byte `physical_test_close_grant`.
Still failing (4), with evidence:
- `original_envelope_parse_preserves_genesis_without_a_discarded_projection_clone`: expected 0 projection clones, actual 1. Backtrace: `replay_loaded_history` ends in `Arc::try_unwrap(state).unwrap_or_else(|shared| (*shared).clone())`; for an empty history the replay state is shared with the genesis, so the clone is unavoidable while `ParsedDocumentText.snapshot` is an owned `P`. Product decision needed (e.g. `snapshot: Arc<P>` or lazy); not changed.
- `apply_batch_publication_keeps_denied_inputs_and_binds_one_exact_issuer`, `interaction_store_apply_updates_hover...`, `interaction_store_reset_discards_hover...`: `advance_apply_batch` returns `PreparationRefused` then `Blocked`, fault "original work requires its original caller loan" under `receive`, and "original work requires its original issuer and resolved quote" under `receive_with_work`. The fixture recipient (`FixtureStoreWork`) does not satisfy the current original-work protocol; the same old-style driver is in `tool-transaction` and `outbound-announcement`. Needs a funded caller turn harness around `advance_apply_batch`, not done.

### semio-os-store-tests-03 and semio-os-store-tests-02: not compiled in this port
Each cargo cycle waited 10 to 30 minutes on the shared capacity gate, so only source edits were made and none verified:
- 03: `receiving_fixture.receive(|receiving|` became `receive_with_work` (26 sites). Nothing else ported; the 70 errors of the first run (store API drift, `RetainedTextMutation: OpBinaryReceiving`) remain.
- 02: `ExactDemoInitialSnapshotRetirementFactory` and the two `Observed` factories moved to `retire_owned_demands`/`retire_owned(&mut Option<_>, effect, reply)`/`recover_retire_owned` via `factory_retire_framed`; `Observed::close_step` takes `body`; identity tests wrapped in `with_original_identity`; `ladder`/`exit_error` local helpers; `close_step`, `step`, `close_factory_ticket`, `abort_one_item_publication`, `take_returned_snapshot_read_retirement`, source admission and operation-wire cursor calls ported; the preborn witness ticket is born through `prepare_factory_retirement` plus `transfer_factory_original`; `original_snapshot_ports_without_representation_bounds` takes `StoreMutationFor`. Open: `returned_snapshot_read_releases_an_alias_before_the_displaced_root...` (`ReturnedSnapshotReadRetirement::admit`, `Arc` canonical root versus `SharedOriginalStorage`), the `persisted_document_hydration_release_tests` module (`replacement` field, `lease`, `as_ptr`), and everything a compile will still show.

### semio-framework-artifact-space-space
Compiles; lib 19 passed, 2 failed; `retirement_law` 2 passed. Ported in `space/io/sqlite/snapshot/tests`: four `decode_sqlite_snapshot_native` calls through `slot(|out| ...)`; the raw `(codec.export)`/`(codec.import)` calls through new local `exported`/`imported`/`published` helpers (same shape as the space-collection tests); `encode_sqlite_snapshot_native` and `decode_sqlite_snapshot_record_native` through `slot` with the new `&mut |record, snapshot_output, native|` closure.
Failures, both in the sqlite snapshot receiving family another agent owns in space-collection (not touched):
- `sqlite_snapshot_framework_space_public_borrowed_native_preflight_admits_actual_owner`: "native encoder requires its original construction recipient", raised in the shared `space/tests/sqlite/preflight` crate; the collection twin fails identically.
- `sqlite_snapshot_framework_space_native_input_retained_materialization_uses_same_caller_ledger`: `charged` is 0, so `decode_sqlite_snapshot_native` no longer charges `reconstruction_remaining_bytes` of the SQL control (assertion "native parser and retained typed fields must publish their actual cumulative ownership"); the ownership is now accounted on the native owner, the test still reads the old ledger.

