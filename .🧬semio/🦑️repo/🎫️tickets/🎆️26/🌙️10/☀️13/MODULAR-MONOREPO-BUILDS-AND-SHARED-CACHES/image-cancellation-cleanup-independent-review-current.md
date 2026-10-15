# Image Cancellation Cleanup Independent Review

Read-only source review, 2026-10-15. No tests, Originals, image operations, production edits, or platform runtime claims. Paths below are relative to `🧰️framework/🔨️modules/🏃️process/`.

## Issued persistent image close can release its real guard after cancellation

`📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📥️issuance/🟦️.ts` 40–59 now enrolls the whole open/body/finally in `withIssuedResourceOperationV1`; finally always awaits `image.close` (58). Platform selection calls macOS `openNativeImageStorageV1`, not the temporary provisioning provider directly.

The ordinary macOS persistent opener at `💾️storage/🍎️image/🟦️.ts` 63 implements close as idempotent `closed=true; shared.release()`. It does not recheck aborted control or admit a new command. Underlying lease release (`🔒️leases/🟦️.ts` 160–164) directly rolls back/closes its already-owned SQLite handle, independent of new authority admission. Queued wrapper release at 263–264 directly releases its real lease, removes its ticket and releases its already-owned claim in nested finally. Thus a canceled accepted body can reach real guard settlement through this close path without inventing authority. Ticket deletion can throw, but claim release still runs; that failure must remain observable. Persistent close intentionally leaves the shared persistent image mounted; it does not promise detach or backing retirement.

## P1: Temporary owned-image close cannot detach after Original cancellation

`💾️storage/🍎️image/🟦️.ts` 81–103 is a separate temporary provision owner. Its close at 102 first invokes `recheck`, which checks the same body control at 97, then calls capture bound to that same control at 86. An aborted/deadline-exhausted Original therefore refuses before detach. Catch cleanup at 103 also calls the same capture and swallows its error; if mounted remains true, it deliberately leaves the directory. This is a concrete cleanup lifetime gap for this provider, not evidence that the issued persistent-image close follows it.

The gap can occur during failed provision as well: once mounted=true at 92, any later Original check failure enters catch 103, where detach admission uses the already canceled control. Additionally the local flag is only set after successful awaited attach; a command failure/cancellation after a side effect is not reconciled with actual mount state by this catch. Source inspection cannot establish that such a side effect occurred on this platform.

## Retirement is a new operation, not cancellation cleanup

Persistent `retireNativeImageStorageV1` at 68–77 checks Original control, acquires new image EX, reads `hdiutil info` under the same control, and admits `hdiutil detach` under it. It will refuse an already canceled Original at entry or command admission; this is appropriate for a separately requested destructive operation but does not provide a cleanup continuation for an already accepted temporary mount. `withExclusiveMaintenance` at 63 likewise checks live control before switching to new EX; it is not an abort escape hatch.

`🎛️owned-execution/📥️capture/🟦️.ts` 32–34 rejects exhausted/aborted controls before spawn, 36 terminates an already-owned child on abort, and 43–45 awaits child close before throwing. `🪓️termination/🟦️.ts` 14–61 performs cleanup of that already-owned process tree directly instead of minting another body control. That is the existing distinction image cleanup needs to preserve: cleanup of accepted physical ownership must be specifically enrolled and bounded, not a new body command admitted through the canceled Original or an unrelated fresh authority.

## Qualification boundary

Current issued-image drainage does not by itself fix temporary provider detach cleanup. Any cleanup owner must remain narrowly bound to the exact accepted physical image/mount identity, retain the actual guard through cleanup settlement, refuse new body operations, preserve primary plus cleanup failure evidence, and have explicit finite cleanup/cancellation semantics. No such cleanup continuation was found in the reviewed macOS temporary provider. Do not call persistent shared close a detach, and do not report cross-platform cleanup success from source review.
