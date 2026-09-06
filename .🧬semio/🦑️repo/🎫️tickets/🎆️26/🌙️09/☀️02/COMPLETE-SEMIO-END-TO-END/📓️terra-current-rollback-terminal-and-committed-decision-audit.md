# Current Backend Rollback, Terminal Cursor, And Committed Decision Audit

Status: read-only current-source audit on 2026-09-06. No source edit or
build was performed. The reported `document-mount` GREEN-17 receipt was not
rerun here.

## Verdict

The new rollback reservation prevents the old *all lost-owner rings full*
drop in the raw backend-registration API, and the new committed-decision
object consumes an authenticated `WalCommittedTransaction` rather than a raw
event. Two P0 liveness holes remain outside those narrow proofs:

1. an external `ArtifactRunnerTerminalJob::resume` can consume the last
   caller-owned close cursor on a submission refusal; and
2. each production storage constructor can return an ordinary `DbError` after
   ownership transfer, while its rollback slot has no independent idle
   maintenance wake if the initial close submission is refused.

Neither is a data-owner drop at the immediate transition: both retain the
owner in an internal object. They are nevertheless terminal-owner leaks: the
only entity capable of resuming closure can disappear.

## P0: External Terminal Resume Can Strand Its Strong Owner

`ArtifactRunnerTerminalJob` is now a genuine external close cursor:
[`db/🗿️artifact/🦀️.rs:4003`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4003)
owns the exact job; `close(self) -> Result<(), Self>` retains that same cursor
while a close poll is parked at [lines 4703-4734](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4703).
The published close law correctly proves no second direct close poll occurs
before an explicit retained wake ([lines 4821-4888](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4821)).

`resume`, however, consumes `self` and returns `()`:

1. It moves the sole external Job out at [line 4676](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4676).
2. On `WorkerPool::try_submit` refusal, it writes that Job back to
   `handoff.terminal_job` at [lines 4692-4697](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4692), then returns with no cursor.
3. If the authority was already dropped, no caller can invoke
   `take_terminal_job` again. The stored Job strongly captures `ArtifactRunner`
   ([lines 4079-4083](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4079)), and the runner holds the handoff, forming
   `handoff -> terminal Job -> runner -> handoff`; its pool-use and any
   engine/WAL close owner are then stranded.

The same cycle is reachable by simply dropping a successful, post-ready
`ArtifactAuthority` while its internal terminal Job is still parked. The
authority has no `Drop` implementation; explicit `shutdown_step` is the only
normal closure driver ([lines 4658-4665](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4658)). The direct handoff-state laws construct a
handoff rather than an actual runner/engine, so they cannot prove this
production owner path.

### Required correction

Make resume lossless: `resume(self) -> Result<(), Self>` (or a named retained
resume rejection) must return `Err(self)` after a failed CAS or failed pool
submission, restoring `owner` to `self` rather than only to the internal
slot. A caller that intentionally abandons it then runs the existing `Drop`
handoff, and a caller that owns shutdown still has a cursor to call `close`.

Do not use a strong Job held indefinitely in the handoff as the last-resort
Drop policy. The document/database owner that is allowed to outlive a caller
must retain and drive an explicit terminal-close cursor; otherwise a parked
strong Job is a reference cycle rather than a lifecycle root.

Add two native laws through a real `ArtifactAuthority`, not a synthetic
`ArtifactRunnerHandoff` alone:

1. Force post-ready submission refusal, take the terminal Job, drop the
   authority, make `resume` refuse, and require the returned cursor to close
   the exact runner once and permit pool shutdown.
2. Force the same post-ready refusal and drop the authority **without** taking
   the Job. Require the designated database/authority terminal registry to
   retain a close cursor and drive it to empty. If no such registry is added,
   this public Drop path must remain an explicit unsupported/fail-stop
   contract rather than a lifecycle claim.

## Prepared Rollback: Source-Owned But Not Constructor-Observable

The raw path is materially improved. `register_db_io_backend` reserves a
fixed rejected-backend slot before consuming the executor or pool use
([`db/🗄️storage/🦀️.rs:2852-2886`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2852)); if that reservation is unavailable it returns
`DbIoBackendRegistrationRejected::Incoming`, retaining the exact executor and
pool ([lines 2627-2663](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2627)). Post-transfer failures commit the executor,
credit, operation, pool and pool use to that pre-reserved row
([lines 2570-2595](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2570)). This no longer depends on the old primary/overflow/quarantine
lost-owner tiers.

The production prepared path does not preserve that typed result. It returns
`Result<DbIoBackendControl, DbError>` and discards the caller-visible
retirement token after committing it:

* [`register_db_io_backend_prepared_with_use`, storage/🦀️.rs:2889-2905](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2889),
* [`MemoryStorage::new`, storage/🦀️.rs:6699-6709](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L6699),
* [`FsStorage::open`, storage/🦀️.rs:7972-7980](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L7972),
* [`SqliteStorage::open_owned`, storage/🪶️sqlite/🦀️.rs:719-728](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs#L719),
* [`PostgresStorage::connect`, storage/🐘️postgres/🦀️.rs:840-850](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️.rs#L840), and
* [`Neo4jStorage::connect_owned`, storage/🌐️neo4j/🦀️.rs:997-1007](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️.rs#L997).

That is source-owned retention, not an immediate drop: the rejected-backend
registry still has the owner. But `DbIoBackendRollbackReservation::commit`
ignores a failed initial `db_io_request_rejected_backend_close`
([storage/🦀️.rs:2593-2595](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2593)), and that request uses a one-shot `try_submit` which merely clears
`scheduled` on refusal ([lines 2811-2838](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2811)). There is no generic DB registration maintenance
hook; only writer release installs one. Generic rejected-backend maintenance
is serviced by opportunistic `db_io_maintenance_step`
([lines 4615-4634](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L4615)), commonly from a later task poll. A failed constructor can therefore
leave an owned rollback and `WorkerPoolUse` indefinitely if it was the last
DB activity and the initial close submission was refused.

### Required correction and proof

Use one typed retained constructor failure, not a compatibility overload:
`register_db_io_backend_prepared_with_use` must return the same
owner-bearing rejection (or a storage-open rejection that contains it), and
each of the five constructors must propagate that retained close/retry owner
after executor creation. `DbError` remains correct only before any executor
or pool use exists. The database mount/open owner then polls `close_step` or
owns a preinstalled fixed maintenance ticket until the exact rollback row is
terminal.

The current native saturation law only fills the new rejected-backend
reservation rows (`RejectedBackendPressureLawSlots`,
[`storage/🦀️.rs:10013-10036`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L10013)) before exercising raw registration
([lines 10316-10336](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L10316)). It does not simultaneously fill the legacy
lost-owner tiers, nor does it execute any production constructor's
post-transfer failure. The separate `LostOwnerPressureLawSlots` is not used
by that registration law ([lines 10040-10057](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L10040)).

Add one native law which fills all three legacy tiers and all rollback rows,
then calls a production constructor/raw registration and proves the typed
incoming executor survives unchanged. Add a second law which injects a
post-transfer registration failure *and* one `try_submit` refusal, performs
no later DB operation, and proves the returned retained rejection (or its
installed maintenance ticket) reaches exact executor/credit/pool-use
retirement only after capacity returns.

## Committed Durable Group Decision: Correct Narrow Boundary, No Recovery Claim

`ArtifactCommittedDurableGroupDecisionV1` is crate-private and its sole
constructor consumes `WalCommittedTransaction`:
[`db/🗿️artifact/🦀️.rs:350-418`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L350).
It takes the authenticated transaction id/segment before decoding, retires
every retained body, calls `finish`, requires exactly one record and exactly
one `Event`, re-admits the canonical Store record, and checks that its parent
artifact id equals the replay document. The `WalCommittedTransaction`
constructor and its raw cursor fields are private; a caller cannot fabricate
this witness from event bytes. Its backing gate only yields after matching
`TxCommit` and count ([`db/📝️wal/🦀️.rs:1360-1401`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs#L1360)).

Thus this object is a valid *read witness* for one physical/logical committed
transaction. It does **not** create Store seals, publish the three edits, or
reconstruct a parent/child outcome. Its only current use is the happy-path
journal test ([`db/🗿️artifact/🦀️.rs:5761-5828`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L5761)); no production replay consumer calls
`recover_store_owned`. It must not be presented as durable Map recovery.

Before wiring a consumer, add four direct fixture vectors at this constructor:

1. an aborted Event is invisible because `WalCommittedCursor` never yields it;
2. Event plus a Command/Outbox, and two Events, in one committed transaction
   both reject as non-sole bodies;
3. a canonical but foreign parent artifact id rejects after transaction body
   retirement; and
4. a decision Event in transaction `n` cannot be paired with parent/child
   commands or a receipt from `n+1`; only the returned `{document,
   transaction_id, segment_index, record}` travels together.

One follow-up API requirement: after this helper has retired and finished the
transaction, a semantic `Corrupt` result does not latch the committed cursor
itself. A future replay loop could catch that error and continue. Add a
fail-closed replay-consumer boundary (or a committed-cursor semantic-failure
method) before production use, so a malformed decision Event cannot be
silently skipped after its bodies were released.

