# Queued Enrollment Independent Review

Read-only inspection on 2026-10-15 of `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts`. No runtime, builds, Originals, reserve recovery, filesystem sealing, or production changes. This describes caveats of the proposed transition; it does not recommend or qualify an actual transition. Current reserve contract remains 4 MiB.

## Already returned queued holder

After acquisition returns at 263–265, its release closure rolls back/closes the resource database, removes its arrival ticket, and releases its already-open claim. It does not call metadata headroom retention/recheck, acquire catalog custody, or deliberately create another claim. Underlying lease release at 160–164 rolls back and closes SQLite. Ticket removal remains a directory metadata mutation and can fail; nested finally still closes the claim. Thus a blanket read-only/immutable seal of the old queue directories can break release cleanup even though no new enrollment occurs.

## Already published waiter

The loop at 252–275 does not invoke `retireResourceQueueClaimsV1` again and has no headroom recheck. It retains its own original claim from 249 throughout polling. However polling is not read-only: for each foreign ticket it acquires catalog SH and calls `acquireResourceLease` EX on the derived claim path (255), then removes that foreign ticket when custody succeeds. `databasePath` explicitly tolerates absent paths and mkdirs directories (101–113); `openDatabase` creates databases (91–97); `initialize` may create schema/insert identity (117–130). Therefore an absent claim behind a visible foreign ticket can cause an already-enrolled poller to create a new claim database. It does not have an existing-only claim-open path.

Once eligible, the waiter opens/acquires its target resource with generic `acquireResourceLease` at 261. The target database need not have existed when its ticket was published: the earlier `databasePath` call at 245 calculates/validates a path, not a database handle. Consequently an already-enrolled waiter can create its resource database after transition. Existing target databases still require writable SQLite transaction behavior according to the generic opener; source alone does not establish behavior under any proposed filesystem seal.

Failure/cancellation removes the waiter's own ticket and closes its held claim at 277–279, again requiring queue-directory mutation. Foreign ticket removal during polling and own ticket removal during release/error are ongoing old-namespace writes, despite absence of new top-level admissions.

## Catalog EX is a short exclusion boundary, not an enrollment census

Catalog SH encloses claim publication only at 248–249 and individual foreign-claim sweeps at 255. Waiters do not retain catalog SH over their entire wait. Holders do not retain it over protected work or release. Catalog EX can therefore coexist with already-published sleeping waiters/holders; it does not prove all old users have drained. A poller that was outside a sweep while EX was held can acquire SH and resume the create-capable path after EX releases.

There is also a prepublication gap: new admission performs retirement at 241 (EX released by that function's finally), then path/queue setup at 243–247, before publication SH at 248. Holding EX during transition excludes publication/sweeps at that instant but does not by itself reject callers already between retirement and publication. An options-without-control admission skips retirement entirely at 241. No durable sealed-generation/admission refusal check appears in 234–249. A transition invariant forbidding new old admissions therefore needs a mechanism beyond this existing catalog lock; this inspection does not claim such a protocol currently exists.

## Reserve dependency

Existing published waiters and returned holders have no direct dependency on `headroom.recheck`; their current failure/cleanup paths can still require filesystem metadata capacity. New controlled admission does require reserve retention through 241 and retirement 224/230. Permanently zeroing/sealing the exact reserve inode would conflict with that current retention contract if any such admission or retirement is allowed to run. Proving no new old admissions is separate from preserving enrolled waiters' completion operations.

These are source paths, not evidence that missing foreign claims, absent target databases, sealing failures, or low-space cleanup failures currently occur on the canonical mount.
