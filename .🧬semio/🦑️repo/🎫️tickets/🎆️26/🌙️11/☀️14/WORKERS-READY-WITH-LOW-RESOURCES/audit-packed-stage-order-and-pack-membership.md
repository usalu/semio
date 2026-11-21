# Packed Stage Ordering and Fleet Pack Membership

At 01:18 UTC seven fresh receiver observations gathered by the fleet audit reported `stage-objects-missing`. Six Linux peers held the same intermediate tree and about 346 incoming items at the same cursor. Windows showed the intermediate tree with an existing extra backlog. The mirrors do not expose the historical missing object IDs. The source reproductions establish two defects that could contribute to this shape.

## Stage publication ordering

The replica's priority lane checks every required incremental object before publication. When a required object is packed-only and its loose path has no live shared PUT, that lane waits. The ordinary fallback immediately calls `publishIndexV1` with no preparation callback, so it can emit the same stage registers. The publish tail and the apply premerge route also use this unprepared call. Pack rescue runs on a separate cadence. The receiver deliberately applies available stage entries and retains missing entries, which explains how this ordering can produce a stable intermediate tree while the authoritative target remains ahead.

The private two-repository fixture packed one required blob and removed only its private loose copy. The other required blob was published loose. The actual replica journal then emitted both stage registers before any publication of the missing blob or its pack. Its receiver exposed a distinct intermediate index with exactly one queued stage entry. Publishing the exact pack and index subsequently drained the queue and brought its tree to the author's target. The fixture lives at `🧰️scripts/packed-stage-order-repro.ts`, and the historical reproduction passed all 17 assertions. The separate incremental publication repair is coordinated with the journal owner.

## Live fleet pack membership

The common filter and rescue callback formerly treated any path present in the intern table as fleet-held. An interned name has no winning operation, and a retired pack's winning operation is a deletion. Either case bypassed local-only filtering and skipped rescue in the same way as a live shared pack. A retire-only grant could also appear to permit publication through this false membership.

The callback now checks the exact manifest operation for a live `put`. Explicit publish grants remain valid, and the existing retirement filter still requires its own grant. The opened-replica fixture and independent SQLite oracle reproduced four false authorizations before the fix and confirm live puts, explicit authority, retired companions, intern-only names, and retired names afterward. The full pack suite passed five tests and 67 assertions, including private repack, GC, healthy pack rescue, and authorized retirement with Git integrity checks.

The generated transcripts are `packed-stage-order-repro.log`, `pack-membership-red.log`, and `pack-membership-green.log` under `🗑️generated`. Git activity in this audit was confined to disposable private fixtures. No worker or shared Git operation was issued.
