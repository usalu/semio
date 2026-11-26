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
