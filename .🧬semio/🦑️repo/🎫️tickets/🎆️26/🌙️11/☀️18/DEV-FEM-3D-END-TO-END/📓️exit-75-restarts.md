# Exit 75 restarts of the describe-rs native build

## Evidence

- `chain.log`: step 10 ended 75 at 15:19 (1226 s), 16:48 (about 40 min) and 18:00 (72 min). The logs of attempts 1 and 2 were overwritten by `> $G/$n.txt` per attempt, and `native-run.ts` only started keeping attempts at 18:02, so no stderr of a failed attempt survives. The reason text ("Yielding ..." / "It yields ...") is therefore unrecoverable.
- Exit 75 sources: the yield handler registered in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧷️format-stamps/🟨️.mjs` (`registerLeaseYieldHandlerV1`: any yield, stale or stuck, aborts the root with a stale-implementation error), the `self-check` guards (`requireCurrent`, skipped while pinned), and a child that exits 75.
- The script process is pinned (`withReceivedImplementationPinV1`, process-wide), so every `requireCurrent` self-check is a no-op there: peer edits of TS modules do not stale the build.
- Format stamp schemas bound to `native-fixed-image:` / graph resources last moved at 12:58 (native.publication) and 16:01 (dependency.walk), before attempt 1 started (16:08). No stamp moved during either failed window, so a stamp-based stale yield is unlikely for attempts 1 and 2.
- Stuck: threshold is 60 min (20 min for graph facts). On Windows the activity sampler treats any live child as work (CPU of the tree is unreadable), and the plan phase prints progress lines, so a stuck verdict by the holder is unlikely while compiling; the 15:19 exit at 1226 s matches the 20 min graph-facts bound though.
- Live attempt 3 (pid 12032) refreshes `progressAtMs` every 20 to 30 s in the lease registry; the only exclusive holds are the cargo custody lease, the target recipe and the compiler cpu slots, none bound by a stamp.

## Defect fixed

A pinned process promises to finish with the implementation it loaded, yet the yield watcher still honoured a "stale" yield request (verified only by the stamp moved on disk), killing an hour of work for a result that stays valid. `leaseYieldAcceptV1` now refuses a stale request with `pinned` while the process runs under its process-wide pin (`leaseImplementationPinnedV1`); a stuck request still needs the holder's own silent clock.

Files: `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts`, `🤝️yield/🟦️.ts`, `🧬️schema/🔣️.json` (corpus `pinned`), `🧪️tests/🐍️model.py`, `🧪️tests/🧫️fixtures/🔣️.json` (case `stale-refused-while-pinned`), checked by `🧪️tests/🟦️.ts`.

## Test

`bun test 🧪️tests/🟦️.ts` in the standing folder: 25 pass, 1 fail. The failure ("idle tree ages into stuck") is pre-existing and Windows specific: an unreadable tree CPU counts as work.

## Open

The actual trigger of the 16:48 and 18:00 exits is not proven. Next occurrence: keep the attempt log (`native-run.ts` now does) and grep for "Yielding", "outdated implementation" or a child exit.
