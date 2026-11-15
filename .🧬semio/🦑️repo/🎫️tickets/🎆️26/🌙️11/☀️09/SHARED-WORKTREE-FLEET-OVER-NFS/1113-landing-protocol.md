# Landing Protocol (binding for every agent that changes the module)

## Incident 2026-11-13 05:12

A landing copied whole files over the live module (`🔁️replica/🟦️.ts`, `🩺️attest/🟦️.ts`, `🎣️tiers/🟦️.ts`, `🔁️replica/⌨️command/🟦️.ts`) from a copy that was based on the 04:56 version. That erased the warm-restart landing of 05:05 (binary checkpoint `state.ckp`, `checkpointReplicaV1`, `🔁️replica/💾️state`). Every daemon restarted into code that could not read `state.ckp`, found no `state.json` (removed by the takeover), started with empty state and republished its whole tree: g1-server published 272 546 files and a new stage base (journal segment 66 → 133); g1-m01, g1-m02 and g1-m06 did the same.

Stored versions (content-addressed blobs in `/Users/Shared/semio-store/v1/blobs/<2 hex>/<sha256>`):

| File | Base (before warm restart) | Warm-restart landing 05:05:35 | Overwriting landing 05:12:21 |
|---|---|---|---|
| `🔁️replica/🟦️.ts` | `c450259512747636…` (04:56:01) | `b1b4520c580a7ce9…` | `31cfe1675a0f4923…` |
| `🩺️attest/🟦️.ts` | `6f65076258fdd1f1…` (04:36:25) | `5d8dca4a05619a2a…` | `7d42790078a283fb…` |
| `🎣️tiers/🟦️.ts` | `0e15ad337b129068…` (04:36:25) | `81e77f8f772d789e…` | `363ae7ca95322e8a…` |

## Rules

1. One landing at a time. Take the lock with `mkdir "<ticket folder>/1113-landing.lock"` (atomic; fails when held) and write your agent label and the time into `1113-landing.lock/owner`. If it is held, wait and retry; never remove a lock you do not own unless its `owner` file is older than 30 minutes. Remove the lock directory when you are done.
2. Never copy a whole file from your working copy over a live file. With the lock held: read the live file, compute your change as a three-way merge (`git merge-file` with base = the live version your copy started from, ours = live now, theirs = your version) or as anchored edits, resolve every conflict by hand, and save.
3. Before releasing the lock, run EVERY test file under every `🧪️tests` directory of the module against the live tree (not only the suites you know: `ls` them), with `--timeout 180000` and `SEMIO_*` unset. All must pass. A failing suite of another agent means you broke its landing: fix it before releasing.
4. Before the last save, load-check every touched source file (import smoke test with the cached bun).
5. Record the landing in `1113-landings.md` in the ticket folder: time, agent label, files, the sha256 of each file before and after.
6. A daemon that finds no usable local state while `.cache/f/machine.json` and cursors exist must rebuild from the store fold. It must never publish its tree as new.
