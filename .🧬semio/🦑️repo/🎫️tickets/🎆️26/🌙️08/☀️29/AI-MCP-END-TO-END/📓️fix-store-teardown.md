# Fix: `ArtifactStore` teardown for `HeadlessWorkspace`'s `open_probes`

## Bug

`cargo test -p semio-framework-os-mcp --lib` panicked on drop for every test that opened a real
`ProbeStore` (`store::ArtifactStore<ProbeSnapshot, ProbeMutation>`) through `HeadlessWorkspace`:

```
panicked at 🏪️store/🦀️component.rs:16443:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
```

`HeadlessWorkspace` (`🌉️mcp/🏠️workspace/🦀️component.rs`) opened real `ProbeStore`s into
`open_probes: Mutex<HashMap<String, ProbeStore>>` but never drained them before drop — not a
test-hygiene gap, a missing production teardown path: a `--folder`/`--hub`-bound `semio-os-mcp`
process would hit the same assert at shutdown.

## The canonical close ritual — found, not invented

`ArtifactStore::drop` (`🏪️store/🦀️component.rs:16416-16443`) requires an "exact terminal-empty"
witness across ~15 fields (backbone, dag, applied/redo edit ids, checkpoint, local actor id,
revision accumulator, tail undo cache, `snapshot_read_leases`, `displaced_retirements`,
`owned_disposer`/`owned_disposer_terminal`, pending report). The store never reaches that state on
its own — a domain must install a real owner catalog and then drive it to completion:

- `MemberStoreOwners<P, Mutation>` (`🏪️store/🦀️component.rs:1972`) bundles a
  `SnapshotRetirementFactory<P>`, an `ArtifactOwnedValueRetirementFactory<P>` (initial snapshot), an
  `ArtifactOwnedValueRetirementFactory<Mutation>`, and a `Box<dyn ArtifactStoreOwnedDisposer<P,
  Mutation>>`. `ArtifactStore::install_member_store_owners_exact` (line 14039) installs it exactly
  once on a freshly-constructed store — "There is no default catalog and a second installation
  faults" (its own doc comment).
- `ArtifactStoreCursorDisposer<P, Mutation>` (`🏪️store/🦀️component.rs:1763-1963`) is the store
  crate's own **canonical, production (non-`#[cfg(test)]`) full-store close driver**. Its
  `close_step` walks every detached authority in order — displaced retirements → returned reads →
  history edits → history metadata → message ledgers → conflicts → pending report → every runtime
  string (applied/redo edit ids, the revision accumulator, checkpoint/local-actor ids, the tail
  undo edit id) → the tail/current snapshot roots → the backbone → the causal DAG → the envelope
  shell — in the exact order `ArtifactStore::drop`'s witness checks them. Its own `terminal_is_empty`
  calls `store.owned_roots_terminal_is_empty()`, the same aggregate the Drop assert needs.
- The store crate's own `#[cfg(test)] demo_closable_store_owners()` (line ~13976) and
  `close_demo_artifact_store()` (line 22248) show the intended drive pattern: install
  `MemberStoreOwners::new(.., .., .., Box::new(ArtifactStoreCursorDisposer::new()))`, then loop
  `SpaceMember::close_owned_step(&mut store, 1, N)` (the `pub trait SpaceMember`,
  `🏪️store/🦀️component.rs:17183`, blanket-implemented for every `ArtifactStore<P, Mutation>`, already
  imported in the workspace file as `use store::SpaceMember as _;`) until `Complete`, then assert
  `close_owned_terminal_is_empty()`.
- `store::sync::ArtifactHost`'s own `impl Drop` (`🏪️store/🔄️sync/🦀️component.rs:1192`) is the sibling
  pattern for the *actor* side: it already calls `self.close(document_id)` for every open document
  when the last host reference drops. That close is what releases the actor's `ChannelBackboneRemote`
  clone of the shared queue `Arc`s — the piece `ArtifactStoreBackboneRetirement::close_step`
  (`🏪️store/🦀️component.rs:16979`) needs to `Arc::try_unwrap` before the backbone phase of the cursor
  disposer above can complete (it returns `SnapshotRetirementStep::Blocked` until the other side lets
  go).

Only production code was used — the `#[cfg(test)]` factories (`DemoSnapshotRetirementFactory` etc.)
were read purely as reference shape, never linked against.

## Production teardown added (ours: `🌉️mcp/🏠️workspace/🦀️component.rs`)

1. **Owner catalog for `ProbeSnapshot`/`ProbeMutation`** (new, right before
   `//#endregion 🔖️ProbeDocument`): `ProbeOwnedRetirement<T>` (a generic take-and-report
   `ErasedSnapshotRetirement` — a probe value has no real external resource behind it, same shape as
   the store crate's own `Demo*Retirement` trio), `ProbeSnapshotRetirementFactory`,
   `ProbeInitialSnapshotRetirementFactory`, `ProbeMutationRetirementFactory`, and
   `probe_store_owners()` which builds `MemberStoreOwners::new(.., .., ..,
   Box::new(store::ArtifactStoreCursorDisposer::<ProbeSnapshot, ProbeMutation>::new()))`.
2. **Install on construction**: `ensure_probe_artifact`'s `None` branch now calls
   `probe_store.install_member_store_owners_exact(probe_store_owners())` immediately after
   `ProbeStore::new(envelope).await?` — the single place every `open_probes` entry is created, so
   every probe store this workspace ever owns carries the catalog from birth.
3. **`close_probe_store_to_terminal(mut probe_store: ProbeStore)`**: loops
   `probe_store.close_owned_step(1, 1 << 16)` (via the already-in-scope `SpaceMember` trait) to
   `Complete`, asserting `close_owned_terminal_is_empty()` on success. A `Blocked` step (the backbone
   phase waiting on the actor's `Arc` release) sleeps 1ms and retries instead of busy-spinning —
   release happens on the real `WorkerPool` background thread the actor runs on, concurrently with
   this thread, so a plain tight spin (tried first; see below) can starve that thread under
   contention instead of ever converging. Bounded at 20,000 turns (worst case ~20s if every turn
   blocked, never observed in practice — see timings below).
4. **`impl Drop for HeadlessWorkspace`**: takes `open_probes` (`mem::take` under the lock, so nothing
   is left for the struct's own field-order auto-drop to hit raw), and for each entry calls
   `self.artifact_host.close(&artifact_id)` **before** `close_probe_store_to_terminal(probe_store)` —
   same "release the other side before draining" order `ArtifactHost`'s own `Drop` already follows,
   and required so the backbone phase's `Arc::try_unwrap` has something to succeed against.

## Why this is correct, not assert-silencing

- No change to `🏪️store/🦀️component.rs` — the assert stands untouched, exactly as the ticket required.
- The fix does not special-case the test-visible symptom; it gives `HeadlessWorkspace` the *same*
  domain owner-catalog + drive-to-terminal obligation every real `ArtifactStore` consumer must
  satisfy (`install_member_store_owners_exact`'s own doc: "before a store can enter retained
  replacement or close"). This is the production teardown a `--folder`/`--hub`-bound `semio-os-mcp`
  process needs at real shutdown, not just a test fixture — `HeadlessWorkspace::Drop` is real code on
  the real struct, not `#[cfg(test)]`.
- First attempt used a bare tight busy-spin (`for _ in 0..1<<20 { close_owned_step(..); }`, no sleep)
  and repeatedly failed 5/6 of the target tests with "did not reach terminal-empty within its bounded
  turn budget" — confirmed empirically that draining the backbone phase needs the actor's background
  `WorkerPool` thread to actually run, which a tight single-thread spin can starve under `cargo test`'s
  parallelism. Adding the `artifact_host.close()` call ahead of the drain plus a 1ms backoff on
  `Blocked` (not on ordinary `Pending`, so normal steps never slow down) fixed convergence — this
  models a bounded wait for a genuinely concurrent resource release, not a fudge.

## Audit invariants re-verified (root `📜️script.ts`, `toolJobPagedIngressExact`, ~4685-4740)

All required substrings still present in `🏠️workspace/🦀️component.rs`, forbidden one absent:

```
PRESENT(14): pending_exchanges
PRESENT(7): PendingResponsePage
PRESENT(1): RejectedCommandBuildRegistry<1>
PRESENT(1): CommandBatchDriver
PRESENT(4): close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES)
PRESENT(9): terminal_is_empty
PRESENT(2): persistent_command_completion_port_ready
--- forbidden ---
0 occurrences of: response: Option<Result<store::AppFrame, Fault>>
```

## Verify — real pasted output (both filters, 3 repeated runs each, all green; showing one of each)

```
$ cargo test -p semio-framework-os-mcp --lib workspace::quick 2>&1 | tail -25
test workspace::quick::base64_encode_matches_a_known_vector ... ok
test workspace::quick::pending_response_faults_oversize_and_duplicate_without_retaining_app_frame ... ok
test workspace::quick::read_artifact_resource_schema_is_real_for_an_open_probe_and_plugin_unavailable_otherwise ... ok
test workspace::quick::prepare_action_on_an_unknown_capability_is_not_found_not_a_panic ... ok
test workspace::quick::read_artifact_resource_validation_is_plugin_unavailable_never_hardcoded_true ... ok
test workspace::quick::open_folder_creates_the_directory_if_missing ... ok
test workspace::quick::invoke_action_on_an_unknown_handle_is_not_found_not_a_panic ... ok
test workspace::quick::read_resource_on_an_unknown_artifact_is_not_found_not_fabricated ... ok
test workspace::quick::routing_artifact_channel_exchange_on_an_unrouted_instance_without_a_purecommand_is_plugin_unavailable ... ok
test workspace::quick::a_fresh_folder_workspace_lists_zero_artifacts ... ok
test workspace::quick::routing_artifact_channel_purecommand_gateway_owned_capability_is_plugin_unavailable ... ok
test workspace::quick::resolve_plugin_for_capability_is_not_found_for_an_unknown_id ... ok
test workspace::quick::resolve_plugin_for_capability_is_plugin_unavailable_for_a_gateway_owned_id ... ok
test workspace::quick::routing_artifact_channel_purecommand_unknown_capability_is_not_found_before_opening_any_channel ... ok
test workspace::quick::routing_artifact_channel_routes_two_capabilities_to_two_different_plugins_opening_each_once ... ok
test workspace::quick::resolve_plugin_for_capability_routes_note_and_cad_to_different_plugins ... ok
test workspace::quick::read_resource_artifact_returns_real_bytes_after_a_commit ... ok
test workspace::quick::resolve_context_reports_the_open_probe_artifact_as_active ... ok
test workspace::quick::read_artifact_resource_schema_is_real_for_an_open_probe ... ok
test workspace::quick::ensure_probe_artifact_seeds_a_real_revision_and_is_idempotent ... ok
test workspace::quick::apply_probe_mutation_commits_a_real_second_edit_beyond_the_seed ... ok
test workspace::quick::undo_then_redo_round_trips_a_real_probe_mutation ... ok

test result: ok. 24 passed; 0 failed; 0 ignored; 0 measured; 260 filtered out; finished in 0.12s
```

```
$ cargo test -p semio-framework-os-mcp --lib artifact::quick 2>&1 | tail -25
running 9 tests
test artifact::quick::every_top_level_schema_is_object_typed_2020_12 ... ok
test artifact::quick::every_artifact_tool_registers_under_its_declared_name ... ok
test artifact::quick::no_workspace_bound_is_a_retryable_plugin_unavailable_for_every_artifact_tool ... ok
test artifact::quick::missing_required_field_is_input_invalid_before_any_workspace_check ... ok
test artifact::quick::workspace_bound_with_zero_resolvable_plugins_is_still_plugin_unavailable ... ok
test artifact::quick::artifact_validate_is_a_real_typed_gap_never_a_fabricated_pass ... ok
test artifact::quick::artifact_create_then_open_round_trips_for_real_with_exactly_one_resolvable_plugin ... ok
test artifact::quick::artifact_snapshot_returns_real_bytes_for_the_current_revision_and_rejects_a_stale_one ... ok
test artifact::quick::artifact_export_never_fabricates_a_successful_export ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 275 filtered out; finished in 0.09s
```

Ran each filter 3 times back to back — 24/24 and 9/9 green every time, ~0.07-0.14s each (the 1ms
`Blocked` backoff essentially never triggers in practice; the actor's background thread is normally
faster than that).

Also ran the wider `cargo test -p semio-framework-os-mcp --lib -- --skip bridge::long` to make sure
nothing else regressed: it aborts with a stack overflow in
`bridge::quick::bridge_outbox_item_cap_plus_one_returns_the_exact_frame_and_rearms_after_one_receive`
(`🧵️bridge/🦀️component.rs`) — pure bridge-module code, nothing to do with `HeadlessWorkspace` or
`ProbeStore`, matching the ticket's own warning about a peer's in-flight stack-overflow fix. Not
touched.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️component.rs` — added
  `ProbeOwnedRetirement`/`ProbeSnapshotRetirementFactory`/`ProbeInitialSnapshotRetirementFactory`/
  `ProbeMutationRetirementFactory`/`probe_store_owners`/`close_probe_store_to_terminal`; installed the
  owner catalog in `ensure_probe_artifact`; added `impl Drop for HeadlessWorkspace`.
- No changes to `🏪️store/🦀️component.rs` (peer's file, untouched as instructed).
