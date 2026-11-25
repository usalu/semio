# L47: Backbone Close Custody, Settlement Ledger Window Birth, Stale Digest Test

This is source analysis only. Nothing was compiled or run, because the brief forbids cargo and builds while the native batch runs. Every fix below is UNVERIFIED.

Inputs:
- `🗑️generated/l32-3d.log` (full panic texts; the L41 log has none for panels)
- `l35` §3, `l38` B, `l41`, `l43` §4

Abbreviations:
- FW = `🧰️framework/🛍️products/💻️os/🔨️modules`
- ED = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`

## 1. `two_instances_converge_disjoint_object_edits_via_backbone`

### The replication itself converges
The panic is not raised in the test body. It comes from `close_registered_fixture_app` (FW/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs:544). That function is called by `impl Drop for Puzzle3dApp` (ED/🧪️tests/🧰️context/🏗️raw-app-fixture/🦀️.rs:328) after both `object_count` assertions have already passed.

So replication was never the problem: both instances receive and apply each other's edits. What hangs forever is the close of the replicated instance.

### Where the close sticks
The l32 diagnostic reads `rung OwnedStores document advance=false`. The close quote is `RetirementDemand { release_bytes: 56, depth: 3 }`. The grant admits it, yet nothing moves for 4096 idle turns.

The close path, step by step:
1. The document lane runs `original_disposal_step` → `ArtifactDocumentStoreDisposer` → `ArtifactStore::close_owned_store_step`. That adds one level of depth and passes the close to the installed `ArtifactStoreCursorDisposer`.
2. The disposer reaches phase `Backbone`. `close_take_backbone_retirement` → `ArtifactStoreBackboneRetirement::admit_attached` moves the `Backbones::Memory` owner into a retirement frame. The disposer keeps that frame as `active`.
3. Inside the frame, the uri is retired first. Then `close_step` calls `detach_queue(&mut value.inbox)`. That function required `Arc::strong_count(queue) == 1`. If the count was higher, it answered `Progress(0)` and kept the queue.
4. `MemoryBackbone::pair` gives the two ends crossed queues: A's inbox is B's outbox, and the reverse. Whichever instance closes first therefore always sees `strong_count == 2` on both of its queues, and the close returns `Progress(0)` forever.
5. The quote matches this exactly:
   - 56 bytes = `shared_retirement_allocation_bytes::<Mutex<VecDeque<BackboneMessage>>>()`, which is the 16-byte Arc header plus an 8-byte Mutex plus a 32-byte VecDeque.
   - Depth 3 = 1 for the frame, +1 for the store disposer, +1 for the document disposer.

The same retirement had the same shared-alias refusal on `Backbones::Channel`, for both queues and the outbound wake, while the `ChannelBackboneRemote` still lives. The existing laws hid this by dropping the peer first: `drop(remote)` in the wake tests, and `drop(peer)` in the detach law.

### Fix (domain-neutral, `FW/🏪️store/🔗️backbone/♻️retirement/🦀️.rs`)
- `detach_queue` is replaced by `release_queue`. It takes this end's `Arc` and calls `Arc::into_inner`.
  - The last end gets the queue back and retires its messages one by one, as before. The ArcInner is freed, so the receipt's `released_bytes` is the quoted 56.
  - Any other end only drops its alias. Nothing is freed, so the receipt reports `released_bytes: 0` (the heap law `heap.released == receipt.released` still holds).
  - The quote stays the full ArcInner bytes as an upper bound.
- The wake branch works the same way: `Arc::into_inner(wake)`. The last owner releases `wake_release_bytes` (the ArcInner plus a uniquely held callback). Any other owner releases 0.
- `Arc::into_inner` is atomic, so a peer that closes at the same moment cannot double-free the queue or abandon it.

### New law
`memory_pair_ends_retire_while_their_peer_still_shares_the_queues` in `FW/🏪️store/🔗️backbone/♻️retirement/🧪️tests/🎟️admit/🦀️.rs`:
- It takes a live `MemoryBackbone::pair` and puts one queued `Genesis` message on A's outbox.
- End A must retire completely while B still holds the queue (a weak probe still upgrades).
- End B must then retire the queue and its message (the probe no longer upgrades).

## 2. `settlement_work_ledger_is_finite_and_progresses`

### What the 200 turns are spent on
The test performs exactly one `advance_typed_operation_publication` unit per test turn. It never calls `fixture_renew_turn`, so each unit runs under the lent context's whole remaining grant.

The l32 rungs show this sequence:
- **Turn 1:** the latest-wins command is already `accepted=true`, `context_phase=0` (`lw64:ctx/lookup/accepted/closing=(0,false,true,false,None)`).
- **Turn 50:** still `context_phase 0`.
- **Turn 100:** `context_phase 2`.
- **Turns 24 to 39 and beyond:** `delta items 512 … fuel 512` every turn.

The 512 is `GENESIS_CURSOR_UNITS_PER_TURN = 512` in FW/🔌️plugin/🌱️initialization/🌱️genesis/🦀️.rs:110. That is the per-turn batch of `OriginalStoreGenesis` phases 4 and 6, so these turns are running a store genesis.

Why a store genesis runs inside the edit's settlement:
- `advance_latest_wins_command_one` (FW/🔌️plugin/🧵️instance/🧾️typed-operations/📥️staging/📬️publication/🖱️inline-interaction-and-latest-wins/🦀️.rs) calls `window_config_store.ensure_for_view(view_state, cx)` before the command may start.
- The ledger test mounts its app with `mounted()`, which never births the `puzzle3d-main` window config partition.
- So the first window-addressed command births that Window's whole native config store (`WindowConfigPartitionBirth = OriginalStoreGenesis<WindowGenesisOwner<_>>`). The birth runs through the cursor and initializer phases at 512 units per turn, between turns 24 and about 95.

The 500 KB turn (turn 27: copy 500048, capacity 484164) falls inside this genesis initializer, not inside the edit:
- The genesis already builds its envelope with `empty_document_envelope_from_genesis_owners` and admits the fixed owner catalogs page by page.
- The largest of those catalogs is the 8192-slot edit-message ledger: about 393 KiB of slot pages plus about 136 KiB of index arrays.
- Its size is fixed per store and does not depend on the document. So this is not an O(document) clone of the puzzle snapshot. It is a fixed, one-time cost per Window store, and the edit's ledger was paying it.

### Why it does not belong to the edit
- A mounted host births every Window in its roster on its first frame, before any action: `pending_effects` → `window_config_store.ensure_for_roster` (FW/🔌️plugin/🧵️instance/🖼️render/📥️staging/🪟️projections/🦀️.rs:142).
- The panels and gestures fixture does the same through `Puzzle3dApp::ensure_window` → `birth_window_config`.
- Only the action-producer fixture skips it. That makes the 200-turn edit ledger measure the one-time Window store genesis, which is about 70 to 80 of its 200 one-unit turns.

### Fix (`ED/🧪️tests/🔬️action-producer/🦀️.rs`)
- New `birth_mounted_window(app, cx)`. It drives `app.stores.window_config_store.ensure_for_roster(Some(&window_view()), cx)` under renewed fixture turns until a turn is ready and moves nothing, exactly as `birth_window_config` does.
- The ledger test calls it after `mounted()` and before `dispatch_fixture_action`.
- The 200-turn bound and every assertion are unchanged.

### Residual risk
After the genesis, the remaining work in l32 was:
- latest-wins phases 2 and 3 and the command start (about turns 95 to 120),
- `Worker` (seen at turn 150),
- `Retiring`, where the result-page ladder tail moves 1 to 47 items per turn (turn 200).

Removing about 75 genesis turns should put the settlement near 120 to 130 turns. If the next run still exceeds 200, the cause is the Retiring result-page tail (`l35` §3). The next step there is a per-turn `[DEBUG]` print of `page_publication.phase_witness()`.

### Follow-up (not changed)
Every native store genesis admits a fixed edit-message ledger of about 0.5 MiB (`ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY = 8192`, FW/🏪️store/📒️ledger/📨️messages/🦀️.rs). That includes every Window config partition, which only ever holds a handful of edits.

The ledger could admit its slot pages lazily, page by page as `free` hands out slots, while keeping only the 16 KiB bucket array eager. That would make a Window birth O(1) small. It changes the open-addressing ledger's admission contract, the replay admission at `📒️ledger/📨️messages/🔁️replay` and the hydration catalogs. It needs a compiled law run, so it was not done blind.

## 3. Stale `🔏️digest/🧪️tests/🦀️.rs`

`original_group_digest_wrapper_retains_native_edit_and_separate_physical_frame` still called `owner.advance_digest(grant)`. The trait has had `advance_digest(window, effect)` since `l43` §4.

Fix:
- New helper `advanced(owner, grant)`. It quotes `digest_window(grant)` and `next_digest_work_demand(window)`, then runs one `advance_digest(window, effect)` inside `AuthoringWallet::self_funded().run(grant, work, …)`, observing the heap only around the step.
- An unadmitted action answers zero progress and zero heap.
- The denied-grant loop and the funded step both use the helper.
- `ArtifactStoreBatchDigest` is imported for the helper's signature.
- The fixture digests, the heap equalities and the `released == held + born` ledger are unchanged.

## Files changed
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/♻️retirement/🦀️.rs`
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/♻️retirement/🧪️tests/🎟️admit/🦀️.rs`
- `/home/deck/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️action-producer/🦀️.rs`
- `/home/deck/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📬️publication/🤝️group/🔏️digest/🧪️tests/🦀️.rs`

## Next run
- Crate `semio-os-store-backbone`: the retirement laws, including the new memory-pair law.
- 3D `editor-panels`: `two_instances_converge_disjoint_object_edits_via_backbone`.
- 3D `action-producer`: both tests.
- The store command retirement public test that includes `group_digest`.
