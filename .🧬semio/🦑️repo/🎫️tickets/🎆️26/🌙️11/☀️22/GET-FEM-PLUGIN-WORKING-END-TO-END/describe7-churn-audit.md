# Describe7 churn audit: the FEM graph inventory (read-only)

Scope: `repo run workspace:components -- describe fem --profile prototype`, log `🗑️generated/pub-describe7.log.run` (4 786 lines, last write 09:40:03, reading clock at 17 546 s). Nothing was started, stopped, killed or edited. Code read under `🔨️modules/📚️library/🏗️build/🕸️graph/` (`📥️load/🟦️.ts`, `📥️load/🧲️membership-follow`, `📥️load/♻️pass-reuse`, `📥️load/🔕️settle-quiet`, `📥️load/🧭️registry-follow`, `🚪️admission/🟦️.ts`, `🚪️admission/🔮️ahead/🟦️.ts`, `🔐️authority` limits in `🧬️schema/🔣️.json`).

## 1. What the log shows

Counter transitions of `completed owner checks` (reading-clock seconds, counter value), taken from the first line that carries each value:

| Reading s | Completed owner checks | Admitted owners | Δ checks | Δ / 436 |
|---|---|---|---|---|
| 1 473 | 456 | 436 | | |
| 4 177 | 2 200 | 436 | +1 744 | 4.0 |
| 8 044 | 3 944 | 436 | +1 744 | 4.0 |
| 14 197 | 6 560 | 436 | +2 616 | 6.0 |
| 17 465 | 7 903 | 436 | +1 343 | 3.1 |

- Checks per admitted owner: 1.05, 5.0, 9.0, 15.0, 18.1. Each step is a near-exact multiple of 436, so the counter advances by whole sweeps over all admitted owners, not by a trickle on a few owners.
- Rate: about 1 480 checks/h over the last 3 300 s (14 197 to 17 465 s), and about 1 600 to 1 500/h over the earlier windows. This is higher than the ~600/h in the brief. The steps are lumpy: one sweep of 436 takes roughly 1 000 to 2 000 s.
- `observations` (196 947), `verified memo rows` (2 944) and `admitted owners` (436) are constant for the whole run. Only `completed owner checks` moves.
- The stall counter (`N of 600`) is 6 or 7 in the tail. Per the emitter (see `describe5-progress-audit.md` section 1) that is seconds since the last progress, so progress is recorded every few seconds. The process is slow, not deadlocked.
- No counter ever drops. A restart of the inventory creates a fresh report with counts at zero (`settledInventory` calls `createGraphInventoryAwaitReportV1()` per round), so the single round-1 inventory has run uninterrupted since 1 473 s. No round restart happened.
- `Planning 1 targets.` appears 5 times (lines 5, 440, 2 036, 2 134, 2 619), each followed by a `graph service holds this request` line with a fresh elapsed value. These are client re-submissions of the same request; the reading clock is continuous across them. They are not inventory restarts.
- The log names no owner and no path. The `[graph-settle]`, `[graph-retry]`, `[graph-owner-retry]`, `[graph-inventory]` and `[graph-owner]` lines go to the worker's stderr and are not in this capture. The only tags present are `[capacity]`, `[script]`, and the progress and service lines.

## 2. Code: what re-checks an owner, and what ends the inventory

**What counts as an owner check.** `ownerCompleted()` (`📥️load/🟦️.ts` line 150) increments once per successful `run()` of an owner in `closeBuildOwnerAdmissionV1`. `run()` goes through `observeOwnerFactsMemoizedV1`, so the counter includes memo answers as well as fresh discoveries. The outcome split (memo / checkpoint / discovery) is recorded but not printed in the progress line. Aggregate checks therefore cannot distinguish cheap memo hits from real re-observation.

**Same-key repeats are impossible within one closure.** `createDiscoveryAheadV1` (`🔮️ahead/🟦️.ts`) caches every flight in a `Map` keyed by `owner + targets + inputs` and never evicts. A repeat of the same key inside one closure returns the same promise. So the growth comes from new keys: an owner whose `targets` or `inputs` set grew (the `dirty` and `requestOf` logic in `🚪️admission/🟦️.ts` lines 160 to 215), or from new passes (`pass` goes up to `owners+65536`). The data fits about 18 distinct (targets, inputs) variants per owner by the end. This is a closure that keeps widening, not one that re-checks identical requests. Whether it is caused by tree churn is not shown by the log.

**Owner-level memo invalidation (where churn would enter).** `observeOwnerFactsMemoizedV1` answers from memory only while `hold(rows)` (`buildGraphSourceFactsHoldV1`) still holds. `evictOwnerFactsReferencingV1` forgets every owner whose rows name a moved path or the listing of its parent. A churning directory that many owners list (for example a root or a shared build directory) therefore forces re-observation of all those owners. That is the only path by which tree changes add owner observations without restarting the round.

**What ends the inventory phase (`settledInventory`, line 246).**
- A round ends normally when `inventory()` returns. It is retried only when it throws `BUILD_MEMBERSHIP_CHANGED` or a path error that `followedPathsV1` recognises (ENOENT/ENOTDIR under the root, or a typed concurrent edit).
- After a throw: if the same path signature repeats twice, it throws. If there are no paths and `round >= repairRounds` (4, from `🧬️schema/🔣️.json` line 15), it throws. Otherwise it pauses `min(round,8) * 250 ms` (no pause for a membership error).
- **There is no round cap when paths are non-empty and differ between rounds.** A live tree that keeps moving paths keeps the loop going. This is a latent unbounded retry, not something the current log shows (no restart has occurred).
- There is **no quiescence wait** in the inventory phase.

**Where quiet is used.** `awaitPlannerQuietV1` (`🔕️settle-quiet/🟦️.ts`) is called only from the owner-round `onRound` (line 229), only for `paths` that lie inside the planner's own tree (`🏗️build/`, computed from `../../../` of the settle-quiet module). For any other path it returns 0 immediately. It waits for the `sliceRestartQuiet` values in `🏡️resident/🧬️schema/🔣️.json`; I did not read those numbers, so the wait bound is unverified. FEM build outputs under `C:\semio` are outside the planner tree and do not trigger this wait.

**Owner-round loop (after inventory).** The loop breaks when facts hold and the membership digest and admission are unchanged (line 317). It stops at `repairRounds` (4) regardless (line 322). The seal then records `sealedWithLaterChanges` (line 325). So the design already tolerates a live tree at this stage. Quiescence is not required to finish.

**Conclusion for question 2.** The inventory phase does not wait for quiet. It ends when one full `inventory()` pass completes without a membership or path error. A live replicated tree can therefore never block the phase on quiet. What it can do is keep the pass slow (memo invalidation) or, if paths keep changing, loop without a cap. The log shows neither a restart nor a round cap being hit; it shows one slow pass.

## 3. Which paths are changing

The log does not name any path, and no `[graph-settle]` or `[graph-retry]` line (which would) is present in this capture. I could not determine whether the churn is FEM agents' build outputs or logs under `C:\semio`, or other sessions' edits. To settle it, the next run should capture stderr (`2>&1`) into `🗑️generated/`, so the `[graph-settle]` and `[graph-owner-retry]` lines with their `first` paths are kept.

## 4. Root-fix proposal

Make the inventory read a stable snapshot of the membership instead of the live tree. Compute the inventory once against a snapshot of the workspace listing, and exclude by policy the volatile trees that the build graph never reads (`.cache`, `🗑️generated`, ticket folders, dependency and build output directories). Those paths should never enter the membership set or the owner memo rows, so their churn cannot evict owners or force a retry. Add a hard cap to `settledInventory` that matches `repairRounds`, so a path-retry loop cannot be unbounded; when the cap is reached, seal with `sealedWithLaterChanges` (the owner round already does this) instead of retrying. Finally, print the memo / checkpoint / discovery split in the progress line and key the owner memo by a stable target set, so that the 436 owners are observed once per real change, not once per new (targets, inputs) variant. Together these make the first pass converge on a live tree.
