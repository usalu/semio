# H45 result: crash-orphan eviction of dirty unit slots

## Landed (06:00, lock process, script `T/🧰️scripts/H45/land-uncommitted-retire.py`)
Classification per recorded phase inside the eviction (writer death is already the exclusive victim lease, pid+startMs checked by the lease registry; a live writer keeps the lease, the slot is left):
- committed (or retiring) stray: `recoverNativeStorageReservationV1` then the funded-child retirement, as before.
- `reserved` (uncommitted) stray: never recovered; new storage export `retireNativeStorageUncommittedReservationV1` appends retiring+retired to the journal after its physical child is verified absent and its empty recorded namespace removed; a present physical child refuses.
- then the slot and its empty producer directory are dropped under the catalogue CAS (directory removal now in `💾️cache/📍️layout`, which keeps the retirement law "volume owns no rmdirSync" green).
Effect: the 8x "Actual recovered committed child required" and the "Producer charge record missing" are gone from the units run.

## Retirement test file: 56 pass / 1 fail (was 52/5)
Test-only fixes: roster path (descriptor caller moved to `📥️read-context`), constructor names after the open refactor, journal-append anchor, borrower close of a dual failure is an AggregateError containing the body error (product `closeBorrower`), fixtures sequential (480 s), child timeouts 120 s (the 5 s default killed the mapping child while the parent spawned its peer under load 45+), `rmdirSync` law via the layout owner.
Remaining: "isolated open lifetime" child: `expect(()=>probe.exec("BEGIN EXCLUSIVE")).toThrow()` (test line 111, probe on the lease db path, busy_timeout 0) does not throw; passed in 3 earlier runs, fails in the last 5 with no product change by me: unexplained (lease lock not held by the exclusive guard at that moment, or the lease file layout changed). Aggregates into the "durable bounded journal" red.

## Units test: 21 pass / 5 fail, new causes (not the orphan)
1. `LeaseOrderErrorV1 Self-deadlock`: the compiler-state child image `cs/v/m/<sha>` is requested shared while the same process holds it exclusive (19x; waits 15 s). Product: a nested child open inside an exclusive scope (H35's nested allowance in `🧹️cleanup/🎛️command` lets the nested open through, it must take the strongest mode up front or reuse the enclosing receiver). First suspect for tests 1, 3, 5.
2. Ordinary Nx: `ConcurrentEditErrorV1 Cargo profile policy Source disappeared` for `🏗️build/🎚️mode/🦀️profile/📋️constraints/🔣️.json` in the sandbox checkout `/private/tmp/sg/...` (tracked, present live): the sandbox lacks a file the profile policy capture reads (copy list/closure of the sandbox), so the fallback-declared fixture is not reached yet.
3. Unit cache: "no sealed snapshot of the tooling could be made after 6 attempts: ENOENT .../rc/🏁️entry.ts" in the same sandbox: seal source missing there, unit cache off for that process.
Run with `SEMIO_TEST_FILE_OFFLOAD=never` (default routing offloaded to g1-m13 and ended with "Original Script deadline refused").

## Verified / unverified
Typecheck of the 4 touched files: only 4 errors in `🧹️cleanup/📥️receiver/🧪️tests/📎️existing` (not mine); dangling-imports 0. Goals 26/0 and 57/0 NOT reached. Claimed test files released (claims line H45).
