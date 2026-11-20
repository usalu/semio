# Coordinator log 2026-11-19 (g1-server, continues handover-2026-11-19.md)

## 00:26 UTC state
- Whole read pid 18668 alive, 4217 of 4785, 10 running, memory-headroom waits (6032 of 6932 MB). Rate fell to ~2/min (audit-fleet-status-0030.md). Seal expected ~02:00 UTC or later.
- R219 bases: all 5 modified files still equal the staged base, 5 new files absent (checked 00:28 UTC).
- g1-m05 attempt 25 stopped as `stalled` at 00:19 (10 min silent at `dependencies:funded-ownership:state` exclusive; a dev serve held the dependency store shared). → R223.
- Load 40 on 10 cores; a desktop-app `git` process at 585 % CPU.

## User decisions (chat, 00:31 UTC)
- Wall 91: run `repo dependencies lock --all` → R230.
- Enable `replica.compaction.automatic` → R231 (land after the seal, IO must not slow the read).
- Git polling of the desktop app: the user reduces it themselves.
- g1-m04, g1-m06, g1-m09: the user checks the machines; R232 handles the shared-tree side of m04/m06.

## R219 landing variant
`☀️13/…/🧰️scripts/R219/land2.sh` = `land.sh` with the replica ordering fix: under the lock, base check, write the 5 new files, wait 120 s (`R219_SETTLE`), re-check the 5 bases, then write the modified files; if a base moved during the settle, only the new files are removed (a peer's edit is never overwritten) and exit 2. Rehearsed on a scratch root 00:29 UTC: forced red restores exactly and releases the lock; a peer edit during the settle keeps the peer's bytes, removes the new files, exit 2.

## Agents started (00:27–00:35 UTC)
| Package | Model | Scope |
|---|---|---|
| R223 | Sonnet | g1-m05 tools-stage stall, R222 open points |
| R224 | Sonnet | replica landing ordering (daemon order / loader retry / shared `land-lib.sh`) |
| R225 | Sonnet | cross-machine task-result sharing (R203/R205) |
| R226 | Sonnet | lease timeouts per class (R218 table), stall detector agreement |
| R227 | Sonnet | `affected` reverse index (R210), lands only after the seal |
| R228 | Sonnet | hermetic task views, cargo state-root/source-root split (R196/R198) |
| R229 | Sonnet | Rust probe phase ledger on workers, dominant phase (handover step 4) |
| R230 | Sonnet | wall 91 relock (user-approved) |
| R231 | Sonnet | automatic replica compaction (user-approved, after the seal) |
| R232 | Sonnet | g1-m04 / g1-m06 software walls, digest mismatch |
| audit | Haiku | audit-fleet-status-0030.md (done; its R206 "blocked" claim is stale: the `rl` spill role exists since 2026-11-18 16:30) |

## 00:42 UTC
- R231 STAGED, NOT LANDED: `☀️13/…/🧰️scripts/R231/land.sh` with `R231_SEALED_BY_COORDINATOR=1` after the seal (9 files, topology + defaults; every replica daemon self-restarts after it). Land after R219 in the post-seal window.
- R227 STAGED, NOT LANDED (`☀️13/…/🧰️scripts/R227/land.sh`); shares `🕰️host/🟦️.ts` with R219: after R219 lands, R227 must rebuild its base (resume the agent). Live tree checked 00:55: no `affected-index` left behind.

## 00:55 UTC: the whole read FAILED at its end (no seal)
- pid 18668 exited 1 after 24784 s. Totals from read.log: 3723 of 4785 projects inferred by 9 worker processes in 24697 s (36 failed, 0 lost, 21 restarts); fleet: 2410 taken with 1170 closure entries (25 re-inferred), 1336 stored, 81 offered, 0 waited, 2727 computed here; owners 4190 of 4785 done (65 present, 122 fetched, 4003 inferred here, 595 wanted from the fleet); 4581 owners published to the fleet owner pool f9e3ed6c1a45.
- Cause: `Internal error: Native dependency has no build project owner: …/🎚️config/📐️owner-contract-and-mutation/📦️packages/🦀️rust (required by semio-hub-stdio-mutation-bridge:build; candidates=4785) (reference c38752f2)`. A peer created that crate at 00:41 UTC, mid-read; the enumeration predates it. A mid-read new owner kills a 7-hour read instead of a repair pass / `sealedWithLaterChanges`. → R233 (urgent).
- No generation sealed, so `repo show projects` from a sealed graph is not possible yet; seal totals are open.
- The graph server starts a new whole read ~10 min after a read ends (R204/R211). That read is the step-3 measurement (first read with R216 adoption-first + form-4 owner records; 4581 owners in the fleet pool).
- R219 is landed at 01:00 with `land2.sh` (no read running; the next read carries it). Bases re-checked 00:57: equal, lock free.

## 01:10 UTC
- R230 LANDED (wall 91): `repo dependencies lock --all` 01:00:14–01:10:04 under the lock; four locks gained exactly the two `ui-react` lines each, no version change; frozen dry-run passes offline in all four scopes. The command spent ~9 min in an ownership walk (performance wall: a lockfile-only relock must not walk the whole ownership) — open.
- R230's 10-min hold consumed the 01:00 window; window extended to 01:15 for R219 then R223 (note in landings.md). R219 took the lock 01:10:04.
- R226 STAGED (lease class waits); conflicts with R223 on the ready process-tree schema → rebases after R223, lands 01:30. R229 lands 01:30.
- New automatic read (runner 53552, 01:02) sits in the front-door wait "a peer builds the graph" (>7 min, generation -1): watch for the custody stall/handover rule (600 s + 300 s).
