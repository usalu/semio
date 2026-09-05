# Terra Map Durable Group Current Frontier

## Current Delta From the Prior Durable Audit

No durable group-commit implementation has landed. The current OS source still contains only the private in-memory `ArtifactGroupVisibility*` family and its history/cursor staging helpers; there is no `ArtifactGroupCommit*`, `ArtifactGroupJournal*`, recovery witness, or kernel journal port. The prior conclusion remains RED.

The material correction for GIS Map is **participant arity**. `GisMapCreateRegionGroupWorkV1` carries three independent mutations: map parent, drawing child, and value child; the optional image is rejected and has no work lane. Therefore the smallest primitive that can execute this exact Map work is a fixed **three-member** atomic commit, not the earlier deliberately-smaller one-parent/one-child proof packet. A two-member primitive remains a useful kernel pilot, but cannot be presented as executing this Map work.  
Source: `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:45-55,92-164`.

No compilation, launch, store mutation, WAL write, or recovery run was performed in this audit.

## Smallest Reusable Current Slice

| Existing slice | Exact current path | What it supplies | Missing piece that prevents Map execution |
| --- | --- | --- | --- |
| Retained member preparation | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:17510-17525,17687-17807` | An exact-member, exact-generation/revision wire candidate; bounded preparation; history and displaced-owner reservation; exact-member abort. | No group commit/adoption operation. Once reserved, ordinary advance explicitly fails with “requires an atomic group visibility authority” (`:17717-17725`). The erased publication API cannot yield a recoverable prepared root. |
| Existing two-member preparation law | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:22671-22749` | Evidence that two candidates reserve real history and retirement capacity while all ordinary roots remain unchanged; stale/cancel abort is bounded. | It deliberately aborts every candidate and proves no commit. It covers two demo members, not Map's parent+drawing+value triple. |
| In-memory decision/staging | `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs:188-245,452-535`; `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:2144-2196,2399-2422` | One-process staged history/cursor read selection and exact adoption/abort under a shared pointer. | It has no durable identity/frame/replay. It stages only VCS and cursor; `ArtifactStore::snapshot`, `snapshot_ref`, `snapshot_read`, and `snapshot_root` read `current` directly (`🏪️store/🦀️.rs:14312-14344`). |
| Parent-anchor WAL mechanics | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs:423-445,1520-1522,1633-1759` | One document's transaction framing, hash-chain recovery, `Fsync`, and an opaque `WalRecord::Event` carrier. A parent-anchor event can physically contain a typed triple frame in one WAL transaction. | `ArtifactWal` is one-document only; Event has no group codec/semantic replay. `ArtifactEngine` owns one document/WAL (`🗿️artifact/🦀️.rs:1153-1176`) and ignores Event on replay (`:1251-1284`). |

The smallest viable new seam is consequently narrow: a kernel-owned journal port consumed by a private fixed Map triple coordinator, with the parent Map as durable anchor and exactly two already-existing members selected from its stable `drawing` and `value` handles. It must journal one canonical frame containing all three prepared recovery payloads before making any root visible. Reusing `TransactionCoordinator` is excluded: it dispatches child members and then parent sequentially, and uses reverse `Undo` compensation on failure.  
Source: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:19331-19354,19422-19440,19509-19565`.

The kernel must receive only a port/receipt boundary; it cannot import db implementation types because db depends on the OS kernel, not conversely.  
Source: `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml:42-61`; `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/Cargo.toml:32-51`.

## RED Executable Laws To Add

These are proposed executable laws, not tests run in this audit. Each is currently unimplementable or would fail with the present API surface.

1. `map_group_three_members_share_one_durable_anchor_commit`

   Prepare exactly Map, `gismap-drawing`, and `gismap-value`; require one durable receipt whose participant set and expected/result generations/revisions name all three. Reopen after the receipt and require all three post-states. Current failure: no triple request/receipt, no prepared-root recovery encoder, and no group journal port.

2. `map_group_current_roots_never_tear_across_anchor_visibility`

   While the triple is prepared, every ordinary root read sees all old states; after the durable anchor decision, every captured group read sees all new states. Current failure: the only group gate covers VCS/cursor, while materialized root APIs bypass it at `🏪️store/🦀️.rs:14312-14344`.

3. `map_group_crash_at_every_anchor_write_is_all_old_or_all_new`

   Use `FaultStorage` / `CrashHarness` to fail or tear each anchor append and sync boundary; reopen must select exactly the old triple or the full new triple, never one/two members. The existing harness is reusable (`🛢️db/🧪️testkit/🦀️.rs:259-360,489-560`), but current Event replay is ignored by `ArtifactEngine`, so there is no group recovery decision.

4. `map_group_member_failure_never_invokes_undo_as_atomicity`

   Inject a failure after drawing preparation or after the anchor attempt and assert no member's ordinary `Undo` is called; before a trusted anchor commit the exact candidates are aborted, after it recovery resolves the outcome. Current `TransactionCoordinator` necessarily fails this law because phase 2 calls member dispatch sequentially and then `compensate` uses reverse-order `Undo` (`🏪️store/🦀️.rs:19331-19354,19515-19555`).

5. `map_group_stale_value_prevents_anchor_append_and_leaves_all_members_old`

   Change only the value member after triple preparation but before journal commit; require a stale rejection, no durable group frame, no visibility decision, and exact bounded candidate retirement. The existing member preparation does revalidate an individual member (`🏪️store/🦀️.rs:17728-17760`), but no coordinator revalidates all three at one journal boundary or exposes an append witness.

## Scope Boundary

Do not broaden the first executable Map packet to child genesis, arbitrary N-member transactions, catalog activation, rendering, or fanout. It must first prove one existing Map parent plus its two existing stable children through one parent-anchor durable decision. A separate two-member primitive may be retained as an internal precursor, but it is insufficient evidence for the three-lane GIS Map work.

## Mounted Owner Audit After Durable-11 (2026-09-05)

This addendum is source-only and performed after the reported `durable11` receipt. It did not run a build or a native law. The new `DurableOwnedMapCommitOperationV1` is a useful retained **kernel-local** owner, but it is not yet a production Map/host integration.

### Actual reachability

`rg` of all Rust source finds exactly one construction of `mount_map`: the in-module fixture law at `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2832-2869`. The method itself is `pub(super)` at `:838-857`, as is `begin_retained_commit` at `:729-755`; a DB, GIS, plugin host, or browser adapter cannot construct it. The only `DurableOwnedGroupJournalSinkV1` implementation is `FakeJournalSink` at `:2394-2417`. There is no DB/WAL implementation of either journal trait anywhere in current Rust source.

The actual GIS result remains a typed, non-applying work value — `GisMapCreateRegionGroupWorkV1` at `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:45-56,92-164` — and has no Store/group call site. At the durable DB boundary, `ArtifactEngine` still owns exactly one document and one `ArtifactWal` (`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1153-1176`), not the fixed Map triple. Therefore the mounted law establishes no real Map publication, Map recovery, provider approval, or host rendering claim.

### What the retained owner does establish

The wrapper keeps all three Stores, the coordinator, and the sink in `ManuallyDrop<Option<_>>` (`…/durable-group/🦀️.rs:697-711`). Its `advance(&mut self)` lends the same retained owners to the coordinator (`:1092-1099`); an ordinary `Err` from journal work remains in `Journal`, rather than recreating the journal. `take_terminal_owners` requires the coordinator's exact terminal witness and takes each owner once (`:1101-1116`). The reported mounted law exercises that `ErrorThenCommit` retry and one-use handoff, but only with three `DemoSnapshot` Stores and the fake sink (`:2832-2869`).

The read/control surface is also internally coherent under normal safe Rust borrowing: `capture_snapshot(&self)` is read-only, while `advance`, `cancel`, and `acknowledge` require `&mut self` (`:1072-1099`). The shared visibility capture rejects mixed/foreign roots (`:649-674`). There is no demonstrated concurrent Map actor, scheduler, or data-serving path, however; a future host must serialize all of those turns around one retained operation rather than distribute the three Stores to independent document actors.

### P0 — bare nonterminal drop is a deliberate fail-stop **and leaks the retained owners**

`Drop` only asserts that all five `ManuallyDrop` slots are `None` (`…/durable-group/🦀️.rs:1119-1130`). A nonterminal request cancellation, a panic after `mount_map`, or a caller that treats `advance`'s `Err` as terminal therefore panics. The five `ManuallyDrop` fields do not recursively destroy their contained Stores/coordinator/sink, so a caught unwind cannot release the staged roots, reservations, journal owner, or writer capability. Replacing these fields with ordinary fields would not solve it: `ArtifactStore::Drop` independently asserts a terminal-empty durable-group root (`🏪️store/🦀️.rs:16790-16830`).

This is acceptable only as an invariant of a longer-lived owner. It is not a cancellation mechanism. Before the first `advance`, the production boundary must park the complete mounted operation in one actor/retained-slot state. A request/future may observe `Blocked`, `Pending`, or `Err`, but may never own the only operation value. On pre-receipt cancellation it calls `cancel` then continues grants through `Absent`/abort/close; on a journal error it preserves the same operation and retries/resolves it; after a valid receipt it continues publish/adopt/ack/close even if the originating request disconnects. Only `take_terminal_owners` may return the Stores and sink to the enclosing host.

Do not expose `mount_map` by simply widening its visibility. The missing public integration seam must remain Store-owned: it should consume three Store-verified preparations plus the three Store owners and an already-exclusive journal sink, then return this one retained operation to a single host authority. In particular, do not hand the stores to three independent `ArtifactEngine`/`ArtifactAuthority` instances; their current document/WAL ownership cannot provide the required shared lifecycle.

### P0 — uncertain durable I/O is only simulated

The trait correctly makes `begin_commit` an infallible, non-I/O transfer (`:253-266`) and leaves any `journal.advance` error in the same retained `Journal` phase (`:917-938`). That avoids the prior unsafe “error means absent” shape. It remains a contract, not an I/O proof: the fake sink decodes into a `Mutex`, increments counters, and returns synthetic receipt values (`:2407-2443`). There is no writer permit, WAL transaction, fsync, crash/reopen resolution, or durable `Absent` oracle behind it.

The first DB-side sink must own the exclusive anchor writer for the entire mounted operation. Its first fallible action belongs in `DurableOwnedGroupJournalCommitV1::advance`, never `begin_commit`; after an append/sync response failure it must retain the same writer/transaction identity and return only `Pending`, a validated `Committed(receipt)`, or a validated `Absent`. It must not map timeout, cancellation, writer contention, or a backend error to `Absent`. A local `cancel` is correctly ignored after the receipt-side phases begin (`:770-790`), but no actual sink currently proves cancellation-after-uncertain-I/O resolves this way.

### Required P0 executable host laws

1. `map_durable_mount_parks_all_owners_when_request_turn_errors` — use the real DB journal sink; make its first append/sync result uncertain, drop the request reply/future, retain the mounted owner in the host slot, then retry to one receipt. Assert no second begin, no Store drop, and one eventual terminal handoff.
2. `map_durable_mount_cancel_after_uncertain_io_waits_for_trusted_absence` — cancel while the real sink reports an uncertain error. Assert the three old snapshots remain readable, no owner is released, and cleanup starts only after the same journal owner proves `Absent`.
3. `map_durable_mount_receipt_survives_disconnect_and_ack_is_host_owned` — commit, disconnect before acknowledgement, then continue the retained owner to closure without a caller-supplied receipt or a Store leak. A later duplicate/reconnect observes the original durable receipt, never a second commit.
4. `map_durable_mount_serializes_capture_control_and_close` — schedule a read, `advance`, cancellation, and close on the actual single host owner. Every capture is exactly all-old or all-new, and no independently scheduled parent/drawing/value authority can mutate during the mounted phase.

Until these are connected to a real Sink and a single Map host owner, Durable-11/Durable-12 are Store-contract qualifications only, not evidence of a durable Map execution path.
