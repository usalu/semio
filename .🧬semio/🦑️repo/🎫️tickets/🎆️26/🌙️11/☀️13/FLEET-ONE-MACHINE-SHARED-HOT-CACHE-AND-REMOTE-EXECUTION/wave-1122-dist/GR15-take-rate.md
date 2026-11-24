# GR15: why the per-owner record take of the `workspace` selection is slow (2026-11-23, 20:55 to 21:35 Mac clock)

`B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`. Lock `lock.graph-read` (released). Scripts and probes: scratchpad `.../scratchpad/gr15/` (`census.ts`, `take.ts`, `fetchall.ts`, `ping.ts`, `auth.ts`, `fsync.ts`); nothing touched the live graph store or a worker.

## Verdict
1. The take code itself is not hours. Records of the selection, taken from the fleet into a scratch store on the coordinator, run at 190 ms per owner serial, 55 ms per owner with 8 looks, and 551 owners in 33.6 s with 48 looks (984 owners per minute). The workers that are not under pressure got through it: m03 and m06 are already in "reading the owners (round 1)" (m03 after 10 minutes), only the memory-pressured m08 is at "taken 48 of 605" after 57 minutes.
2. What makes m08 1 to 3 owners per minute is the machine, shown by its own `slice-memory.json`: the same main-thread work costs 45 times more (assembly authority of an owner 3.3 s on m08, 73 ms here), every fleet request is slow (lookup 9 s, blob 6.4 s on average against 5 to 30 ms here), 37 of 81 looks ended `unavailable` because the 15 s breaker marker was set, and the presence sessions of the mounted image took 330 s of 812 s. The fleet status shows why: workers `pressure warn`, load 8 to 33 on 10 cores, 3 to 6 remote actions running beside the stage (`worker idle`/`busy`), 2 GB swap.
3. Landed: removal of the self-inflicted parts (breaker, burst timeouts, duplicate closure fetches, low look width) so the take is bounded by the machine and not by our own waiting. No claim that this alone brings m08 to minutes: that needs the machine to stop running remote actions while its stage is not Ready (open item 1).

## 1. What a take costs per owner (measured on the real fleet entries)
- Census of the fleet cache (`census.ts`, instance `main`): 551 project-inference entries, record.br median 4.9 KB (max 670 KB), median 8 files per entry (record plus leaf blobs of the inventories), 0 closure documents in any entry. So for this selection GR6 (closure pack/zlib, verification) and GR12 (closure re-ask) are not on the take path, and nothing is hashed twice per closure group.
- Requests per owner: 1 lookup (`ac`) and 6.3 blobs (record plus about 5 leaves), 7.3 in all (`take.ts`: 527 owners, 527 lookups, 3,333 blobs, 19.1 MB). The cache server answers them at 5 to 30 ms (`fetchall.ts`: 60 owners, 177 requests, 0.3 s at width 12; `ping.ts`: 600 lookups p50 0.8 ms, p99 38 ms, max 57 ms).
- Profile of a take of 120 owners (bun `--cpu-prof`, serial, 29 s without my probe's own `rmSync`): fleet fetch and leaf writes about 4 s, `importProjectRecordV1` 9.4 s (80 ms per owner: observation blocks 8.4 s, brotli 2.4 s), implementation-guard `lstat`s 4 s (unpinned probe only). No claim, lock, sleep or poll interval on the path of a look (claims exist only for `claim`, not for `gate` inside a take look).

## 2. Scratch take, before and after (coordinator, load 30, same code path `createProjectFleetPortV1.gate` inside `runTakeLookV1`, real client and real entries, records imported into a private staging store)
| owners | looks at once | wall | per owner |
|---|---|---|---|
| 200 | 1 | 37.7 s (before), 17.3 s (after, quieter host) | 190 ms / 87 ms |
| 200 | 8 | 11.5 s | 58 ms |
| 200 | 48 | 10.1 s | 50 ms (the probe's own thread is the limit) |
| 551 | 48 | 33.6 s | 61 ms |
Owners per minute: serial 316 to 690, 48 wide 984; against 1 to 3 on m08. The change set does not make this probe faster (it was not slow); it removes what happens on a loaded machine (section 3).

## 3. What the worker's own numbers show (read-only: `/Users/Shared/semio-store/v1/origins/g1-m08/slice-memory.json` at 812 s of a process, `ready.json`)
- looks 81, average 18.5 s, longest 145 s; faults `unavailable` 37, `transport` 4 (the client says unavailable when its machine-wide breaker marker `down` is younger than 15 s).
- fleet `gateTimings`: metadata 38 requests 341 s (9 s each, 11 still active), blobs 27 requests 173 s (6.4 s each), seed 23 records 8 s (0.35 s each). A lookup deadline is 1.5 s: the deadline started when the look called, not when the request left, so on a main thread that is starved the timer fires with the answer already in the socket.
- look phases (sums over 81 looks): authority 271 s (parent computes the assembly authority itself because a selected read has no look farm), observe 896 s, `reusable` (inventory presence under the mounted-image consumer) 767 s, gate 522 s. Presence: 5 batches, 221 names, sessions 4: pre-verify 100 s, body 153 s, post-verify 76 s.
- The coordinator's `down` marker is rewritten about every 100 s although its server answers lookups in under 60 ms: any timed-out claim or put marks the machine closed for 15 s, and nothing lifts it before the 15 s have passed.

## 4. What changed
| file | change |
|---|---|
| `B🪪️graph-state/🛰️exchange/📡️client/🟦️.ts` | (a) a GET waits for one of `requests.readsInFlightMaximum` (24) slots before its deadline starts, so a burst of 48 looks times out on the server and not on its own queue; uploads and claims are not bounded. (b) an answered request (status below 500) that was sent after the `down` marker was written removes the marker: one slow claim no longer shuts every lookup of the machine for 15 s while other requests succeed. |
| `B🪪️graph-state/🧬️schema/🔣️.json` | `requests.readsInFlightMaximum` 24. |
| `B🪪️graph-state/📦️project-records/🟦️.ts` | closure documents of an entry (the case of GR6): one read per document hash across concurrent looks (`heads`, was one per owner: 17 duplicate downloads with 24 looks at once), one import flight per hash whatever look asks (was only for the same look), and a held re-check inside the flight; so a group of owners that shares a closure fetches, unpacks, rewrites and verifies it once. |
| `B🕸️graph/📥️inference/📦️project-nodes/📥️take/🟨️.mjs`, `🧬️schema/🔣️.json`, `🟨️.d.mts` | `fetchConcurrency` 48, `takeFetchConcurrencyV1`, `takeFetchFirstV1({ fetchConcurrency, fetches })`: the lanes of owners that are not stored here (records from the fleet) run 48 looks at once instead of 8 to 12; they wait for the fleet and for one presence batch of the mounted image, which costs the same for 8 owners and for 48 (m08: 80 to 100 s per session), so 605 owners need a fifth of the sessions; the memory gate still bounds them. The lane of stored records keeps its width. |
| `B🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs` | passes the two new inputs. |

## 5. Tests (`repo test-file`, one at a time)
- New `📡️client/🧪️tests/🩹️heal` 4/0: a later answer lifts the marker, an earlier request does not, a burst of 120 reads never has more than 24 in flight and none times out while it waits (5 rounds of 375 ms against a 1.5 s deadline), a cancelled waiter leaves the queue.
- New `📥️take/🧪️tests/🛬️fetch-lanes` 3/0 (fetch lane 48 at once, stored lane 12, cut process, batch arithmetic).
- New integration `📦️project-records/🧪️tests/🛬️bulk-take` 1/0, real cache server: 200 owners in 3 closure groups, 24 looks at once: 200 lookups, 203 blobs (200 records and the 3 closure documents once, each blob once), 3 imports (closure verified once), all 200 seeded, every closure entry holds, bound `stretchedBudgetMillisecondsV1(20000)` (took about 2 s). Before the change 220 blobs (17 closure documents fetched again by looks that started together).
- Still green: client `🚦️confirm` 1, graph-state 14 and `⏳️lease-wait` 17, selected-exchange 21 and `🖥️real-server` 3, project-records main 7, closures 3, pool-reader 3, pool-serve 1, lookup-fault 2, fleet-machines 6, retry 8, take main 9, fetch-first 4, hot-lane 3, two-machines 3, project-nodes away 1, selected-fleet 2, lanes 1.
- Failing, not caused here: `📦️project-records/🧪️tests/🌊️seed-stream` (the stored closure entry lists its rows sorted as text, row 7 is `10002.ts`; the assertions on stored and downloaded entries before it pass).
- At about 21:27 `project-records/🟦️.ts` did not compile for about 70 s (a doubled `const` of mine, fixed at once).

## 6. After: workers (observed only)
At 21:32 m03 (629 s) and m06 (1,798 s) were in "reading the owners (round 1)", m08 at "taken 48 of 605" (3,411 s), m02 still in the file inventory (11,383 s). The new code reaches a machine when its graph child restarts; nothing was restarted. Check m08's `gateTimings.metadata` average and `faults.unavailable` after its next restart.

## 7. Open items
1. The workers run 3 to 6 remote actions and load 8 to 33 on 10 cores while their stage is not Ready: the take, the file inventory (m02, m07, m10, m12, m13 for 9,000 to 12,000 s) and the presence sessions compete with compiles. A worker whose stage is not Ready should accept no remote action (or one) until it is; this is the lever for m08. Not changed (execution scheduler lane).
2. Assembly authority per owner in the parent (3.3 s on m08 against 73 ms here, about 33 minutes for 605 owners): the farm look that derives it is switched off for selected reads (`lookQueue` is undefined with `ownerAdmission`); a bounded memo per project group would remove it.
3. Presence: after a take has adopted the leaves of an entry (stored and verified by digest moments before), the inventory presence ask of the same look repeats the proof under the image lease (80 to 100 s per session on a loaded worker); remembering the adopted leaves for that look would drop it. Not done.
4. The cache server (single bun thread, 90 to 107 % CPU) answers lookups well, but `claims.jsonl` grows and the `down` marker of the coordinator is rewritten every 100 s; the retention sweeps run in dry-run mode (5,190 entries).
