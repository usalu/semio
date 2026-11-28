# Graph-plan starvation: idle retained reservations were charged in full

Scope: live gate of the 16 GB Mac at about 10:25 to 10:35 (`repo capacity status --json --lang=en`, `~/Library/Caches/semio/host-capacity/{grants,queue}`, service logs `/var/folders/*/T/semio-repo/501/host-*.log`), decision code `🌡️host-capacity/📏️decision/🟦️.ts`.

## 1. The decision, reproduced with the gate's own function

Test `🌡️host-capacity/🧪️tests/🚏️idle-reservation` builds the snapshot of the report (4973 MB available, normal pressure, 7 grants, 4 waiters) and calls `evaluateHostCapacityV1` on the sample after `withUnmanagedHostV1`, exactly as the gate does, with the shipped policy (`🏗️build/🔣️policy.json`). Without the new rule (case 2) it answers:

| quantity | value | where it comes from |
| --- | --- | --- |
| floor | 2048 MB | 12.5 % of 16384, bounds 1536 to 4096 |
| unmanaged | 6420 MB | 16384 - 4973 available - 4991 observed by grants (the interactive reserve of 4096 MB is lower) |
| share | 7916 MB | 16384 - 2048 - 6420 |
| unrealized | **2080 MB** | gismap-gis2d 1536-550 = 986, stdio 1536-1447 = 89, tile 0 (holds 2244), gisterrain 1536-531 = 1005; the three quiet runs are past their 300 s ramp and charge only what they hold |
| room | **845 MB** | observed side 4973 - 2048 - 2080; budget side 7916 - 7071 committed (the same 845) |

Verdicts: planner `wait` reason `memory`, the second planner, both cargo requests `wait` reason `queue-order`. Nobody is admitted.

Why the planner is not admitted:

1. `reservationOf` returns the full `reservedMegabytes` for every grant that retains its reservation, for the whole life of the grant. The cargo grants are `retainReservation` and `batch`, each books 1536 MB and has held 531 to 550 MB for an hour, so 2080 MB of idle headroom counts as "reserved but not yet resident" against the available memory that already shows the real use. The room is 845 MB, the planner needs at least 2048 MB.
2. Reduced start does not help: `relaxedOf` lowers the minimum only when `floorMegabytes < minimumMegabytes`. The slice pool books the planner with `minimumMegabytes` 2048 and `floorMegabytes = min(elasticity.minimumMegabytes, ...)` = 2048, so floor equals minimum: it already starts at the least a slice worker can start with. The `shrunk` path needs room of at least the minimum (845 < 2048) and `lendable` only counts ramping, non-retaining grants (none).
3. Lending does not help: `lendIdleReservationsV1` lends from a `batch` retained grant only when it holds under `rampIdleObservedBasisPoints` (25 %) of its reservation. The three cargo grants hold 36 %, 94 % and 146 %, and `graph-plan` grants are never lenders.
4. Head of line did work as designed and made the stall total: after 60 s the planner is the head, it also passed the bypass time and `drains` is true (its ask of 6553 MB fits an empty machine's 7916 MB share), so `planBarrier` and `blocked` hold the whole queue (`queue-order`) and a newer request passes only if it fits beside the head's 2048 MB (845 MB: none does). That is correct reservation, but it cannot create memory: the cargo grants run for hours and release nothing, so the room never rose above 845 MB and the planner starved for 3138 s. (At 10:25:24 it was admitted when the cargo use shifted; the ledger shows `admittedAtMs` 10:25:24 for arrival 09:32:56, so the report's snapshot is the second before that.)

So the missing piece was not the head-of-line rule but the room it reserves: a reservation that stays charged at its estimate long after it is measured.

## 2. Fix

`📏️decision/🟦️.ts` new `settledRetainedHostMegabytesV1`, used by `reservationOf` in `decideHostCapacityV1`: a grant that retains its reservation and has run `memory.settledAfterMilliseconds` (900 s, since admission or resume) with a known observation is charged `min(reserved, max(observed x retainedKeepBasisPoints, observed + retainedKeepMegabytes))` (12500 bp and 256 MB in the shipped policy). Unchanged (full reservation): young grants, unobserved grants, `graph-plan` grants (the worker reservation is their no-kill guarantee), farm children (`holder`). A grant that over-uses (tile: 2244 observed, 1536 reserved) is charged what it holds through the existing `committedOf = max(reservation, observed)`.

Effect on the snapshot: unrealized 2080 to 601 MB, room 845 to 2324 MB; the planner is admitted `shrunk` at 2324 MB (at least the minimum 2048, less than its 6553 MB ask); everything behind it stays `queue-order`, and the second planner is not admitted in the same round (no double booking of the same room). On the live ledger at 10:33 the new code reads unrealized 576 MB (cargo margin 256 + 47, planner 273) instead of 1293 MB.

Schema-first: `🧬️schema/🔣️.json` memory policy gets `retainedKeepMegabytes` and `retainedKeepBasisPoints` (optional, with `settledAfterMilliseconds` as the settle time), the product policy `🏗️build/🔣️policy.json` ships 256 and 12500, the `HostCapacityPolicyV1` type lists them. No shim: policies without the fields keep the old charge, the shipped one carries them.

Risk: a cargo grant that grows back into its 1536 MB after being charged at 806 MB is seen at its next measurement; meanwhile the host warden (cancel-only for cargo) protects the machine, and the margin is 25 % or 256 MB.

Not changed, by decision:
- Head of line / barrier: its behaviour is correct once the room is real (new rows prove a newer request that fits alone does not overtake the planner, and the planner takes the room the moment its minimum fits). The existing `🚦️starvation` and `🪂️backfill` suites already cover streams of newer requests.
- Reduced start: nothing below 2048 MB can start a slice worker.
- Two planners from different clients: they come from two services of one checkout and one agent session (`a3fba9d52db0.d`): pid 62657 (the generation that read for an hour, superseded) and pid 30457 (the new generation, 18 min old, `supersede-churn.md`); the slice pool coalesces only inside one service, and a superseded generation keeps its read by design. They queue cleanly at the gate: the second planner waits `memory` (and `queue-order` for the cargo requests behind it) while the first holds 4131 MB and is not admitted over it. Merging across generations would need the new generation to wait for the old one's read; left as a follow-up, not forced here.

## 3. Tests

- New `🌡️host-capacity/🧪️tests/🚏️idle-reservation/` (`🔣️.json` rows, `🟦️.ts`, independent Python oracle `🐍️.py`): 8 cases whose verdicts and reservations equal the oracle (live snapshot admitted at 2324 MB; same snapshot without the rule starves; cargo younger than the settle time is charged in full; a graph-plan grant is never reclaimed; a newer 512 MB request waits behind the planner that cannot fit yet; the planner takes 2451 MB when room appears and the newer one waits; two planners: one admitted, one queued; a planner takes at most the room the host share leaves), plus explicit reproduction of the old numbers (2080 unrealized, 845 room, `memory`) and the unit rows of `settledRetainedHostMegabytesV1`. 7 tests pass (`repo test-file`).
- Regression: see the live section for the suites that read the shipped policy.
- Regression with the shipped policy (`repo test-file`, 14 files, 0 failures): retained, ledger-update, overcommit, graph-read-precedence, reserve, shipped-policy, settled-read, progress, scaling, starvation, backfill, gate (34), decision table (30), idle-reservation (7). `repo typecheck --no-importers` on the decision file and the new test: 0 errors. `🌤️followup` (`followup-2`) fails on its stored world with and without this change (a fixture policy without the new fields, other agents' head-of-line edit, see `warden-guard-level.md`); not touched.

## 4. Live

How the fix reaches the gate: the decision runs inside every process that waits (each `repo` client and each resident service evaluates `evaluateHostCapacityV1` on the shared ledger and queue itself, from the immutable implementation it started with). New processes (`repo capacity status`, every new `repo run`, the next service generation) load the new code at once; the two resident generations that hold the planners (pid 62657, the superseded generation that is reading, and pid 30457, 18 min old) keep the code they started with until they retire. Restarting them would cut a planner that holds 4131 MB after an hour of reading (owner checks are not persisted, `slice-resume-and-cap.md`) and could not admit anything now, so I did not stop them: the old generation retires when its read answers, a new generation starts after the settle quiet time.

Evidence on the live ledger, `repo capacity status --json --lang=en` (new code), 10:33 to 10:40:
- unrealized memory 576 to 585 MB (old rule on the same ledger: 1293 MB: 273 planner + 47 stdio + 973 gisterrain); room 770 to 1263 MB (varies with the host).
- the growth request of the running planner (4404 to 4960 MB, waiting for the old code's gate for over an hour: `waiting ... to grow the reservation from 4404 MB to 4960 MB` in `host-11ba87a1f84c.log`) is decided `admit` / `fits` 4960 MB by the new code at 10:39 and 10:40 (4199 and 4959 MB available).
- the second planner (min 2048 MB) is decided `wait` / `memory` while only 1098 MB are free above the floor (its minimum is really not free, the first planner holds 4131 MB): it queues cleanly behind the first instead of starving behind idle cargo headroom. It is admitted by the new code as soon as 2048 MB are free (that is the 4973 MB snapshot of the report: 2324 MB).
- the first planner was admitted by itself at 10:25:24 (arrival 09:32:56, 3148 s): the report's numbers are the seconds before.

Files: `🌡️host-capacity/📏️decision/🟦️.ts`, `🧬️schema/🔣️.json`, `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json`, new `🌡️host-capacity/🧪️tests/🚏️idle-reservation/{🟦️.ts,🔣️.json,🐍️.py}`, this note.
