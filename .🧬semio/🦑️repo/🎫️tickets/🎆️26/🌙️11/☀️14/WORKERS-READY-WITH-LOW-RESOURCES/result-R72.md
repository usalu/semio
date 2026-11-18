stage green: yes; fix 1 landed by R73 (05:05), step B (R72b) landed, all suites green (laws and settle suites green in the scratch copy; the cutoff top suite and the standing preflight tests run inside land.py under the lock, the scratch copy has no ajv and not the whole tree)

# R72 step A: vouch of unwatched paths (fix 1)

Stage: `T/🧰️scripts/R72/` (`land.py --dry-run | --land`, `stage/f1..f4`). T = FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION (☀️13).

## Cause (measured on the real manifest, generation of 11-15 17:32)
- 18 152 index paths lie below `.cache`, `node_modules`, `target`, which the change journal never watches. The `resolve typescript` row alone puts `.cache/b/d/m/j/<fp>/node_modules/typescript/...` into 11 064 parts.
- `affectedPartsV1` calls such a part touched and `createVouchV1` made it `hard` (never vouchable). 7313 of 7470 closures are hit by this alone, exactly the logged `slow=7313`. Only 155 stored entries carry a vouch. No key component changes between reads.
- The dirty set is 91 102 paths because no read has published a generation since 11-15 17:32.

## Fix
- manifest: `createVouchV1` stamps every unwatched path/dir a part reads, `stamp(name, cheap)`.
- row-paths: `stampOfV1(path, memo, cheap)`: size, mtime, ctime, inode; none inside a 2 s racy window.
- generations: call site passes `cheap`.
- Stamping all 18 153 keys of the real manifest: ~75 ms. First read after landing is slow once; later passes and reads are fast. Worker slice readers benefit too.

## Proof
- 4 new laws in `🧾️vouch/🟦️.ts`: unchanged start = slow 0 and rows 0 (before the fix slow 40 of 40); edit below an unwatched region moves exactly its readers; revert moves them again; racy stamp never vouched; cheap stamp equals independent `statSync`.
- Scratch copy: vouch 12/12, hard-parts 2, epoch 8, dependencies 11, throughput 1, manifest 1, walk-farm 9.

## Closure membership
The three edited `.mjs` files are not named in `provider-closures/🔣️.json`; the real walk (preflight, tool-cover) runs in `land.py`.

## Not verified
Real start-phase time after landing; cutoff top suite and standing preflight until `land.py --land`.


# R72 step B: prime, silent children, plan-pass reuse (landed once, stage `T/🧰️scripts/R72b/`)

## What the prime child waited on (observed live, read 93751, prime from 04:00:39Z)
- Not a lock, not a dead peer, not the cache server. The preflight child ended in 30 s. The prime child (pid 10940) then derived the shared recipes one after another: it held 6 walk-farm children but only ONE worked at a time (0:46 CPU against 0:00.8 for the other five), the parent waiting for each answer (`for (recipe of recipes) await ...`). Under load 25 each recipe takes tens of seconds, 60 recipes serial = 11+ min; the farm was also closed and restarted once during the wait.
- Nothing bounded a silent child except the 900 s wall limit.
- Prime shared one lane with the heavy questions and ran before the plan, so nothing else moved.

## Changes
- heavy-questions: two lanes (prime and preflight / heavy questions); a child writes a life line on stderr every 2 s and is stopped after 60 s without output, refusal `graph.heavy-question-silent` (EN+DE); children are killed at process exit; policy and schema extended.
- `primeSharedClosuresOfWorkspaceV1` returns after the preflight (a refused tool closure still stops the read in seconds), the prime child runs detached and its failure never fails the read. `primeSharedClosuresV1` derives 4 recipes at once.
- Plan reuse inside one read: `journalGraphSeamsV1.unchanged` accepts a journal delta that names no path (nor parent directory) of the plan's fact index and no directory change; `hint.holds` replaces the 1 s hint age of `buildOnce`.

## Proof
- Laws (fail before, pass after): silent child stopped in under 10 s with EN/DE text; a child with a life line outlives the silence; a long prime does not hold a heavy question back while two primes stay serial; detached prime (ports); plan stays valid across an unrelated journal delta and not across a named path, an entry of a listed directory, an element of a resolve key, a directory change or an incomplete answer.
- land.py green: generations (4 suites), cutoff, vouch, graph-hooks, shared-prime, pass-verdicts, governed-dispatch, parent-budget, preflight, tool-cover, heavy-questions (one timing law red once, green on repeat).

## Measurements
Before (read 93751, started 03:31Z, old code): plan settle A 879 s (slow=7313), plan settle B 459 s (slow=7313), prime 11+ min (child ends at the 900 s wall limit).
Full before, read 93751 (started 03:31:18Z, old code, one process): plan A settle 879 s, plan B settle 459 s, prime 12.7 min (04:00:39 to ~04:13:50, serial, ended by itself), plan C (third pass, facts 2 min + settle ~8 min, slow=7313 again) ended 04:25:14Z; owners wave started 04:25:55Z. Start phase = 54.6 min, three full plan passes plus a serial prime.

After: NOT MEASURED. No read with the new code has started: the running read loaded the old modules, and the machine runs one read at a time (I did not cancel it). Expected, not measured: the first read with the new code still pays one slow settle (stored entries carry no vouch for unwatched paths; it seals them), its plan passes B and C then reuse or vouch, the prime is off the critical path; the second read (after a generation is published, small dirty set) is the warm case. To read the numbers of that read: `grep -h 'settle closures\|phase=graph.build.prime\|phase=graph.build.compute' /var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/semio-footprint/<graph child pid>.log` (pid = child of `read` in `.cache/f/graph-server/read-record.json`); the settle line must show `slow=` near 0 from plan pass B on, and `heavy-question-silent` must never appear in `read.log` for a live prime.

## Not verified
Real start-phase after; whether the journal delta reuse triggers on the busy fleet (law proven, real hit rate unknown); memory pressure of the detached prime (2.5 GB cap) running beside the owners wave.
