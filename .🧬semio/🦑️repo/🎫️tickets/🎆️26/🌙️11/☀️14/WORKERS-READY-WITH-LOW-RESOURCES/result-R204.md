# R204: fetch-first adoption, Windows record spelling, stale fleet records, samples of the read from 11:47 UTC (living report)

Written from 2026-11-18 11:48 UTC, continues R202. Scripts: `FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R204/` (`land.sh`, base copies `*.base.*`, laws in `law/`). Scratch: `scratchpad/R204/` (`samples.txt` raw samples, `sample.py` = R202's sampler made tolerant of a missing read record, `stage/` staged files and `diag/` read-only record scanners). No slice-lab run, no download, no kill except the one authorised `kill -INT`.

## 1. What happened to the 11:47 read (sampled)

- Started 11:47:16 (runner 98355, read 98361). It never reached the owners: it sat in `source.served` as "a peer builds the graph; this run waits as its front door alone, without a loader" from 11:47:45 until 12:03 (more than 16 minutes, 0 CPU, 2 idle processes). The graph server restarted at 12:01:56 on a code edit ("graph code changed, restarting") and attached to the same read. There was no owner phase, so R200's speed-up, R202's queue order and the booking rule were NOT exercised by this read; at 12:03:55 UTC I stopped it once (authorised) with `kill -INT 98355` (parent pid 1, command line holding `graph-server/read-record.json`), after my landing, so the next read carries the new code. Who the "peer" is was not found (the sidecar log shows no second graph build; the machine's pool child 66433 and its walk-farm children were the only graph-adjacent processes).

## 2. Landing 12:02:31 to 12:03:25 UTC (see `landings.md`, "R204 landing 1")

Tests first (red on the live code, green staged, green again live under the lock; 27 test directories ran under the lock).
- `📥️take/🧪️tests/🎣️fetch-first` (4): a simulated fleet that publishes every owner within T = 400 ms while one local look takes 150 ms (2 at once, 30 owners, 2250 ms alone): the read ends in under 1100 ms with all 30 owners adopted ("fetched"), only owners not yet published are looked at locally; a fleet that publishes nothing leaves all 6 owners to the own looks (no hang); an owner the pass missed (miss) and the fleet delivered later is adopted by the adopt loop without waiting for the pass; a source-text law that the read calls `takeFetchFirstV1` and counts an owner once.
- `🔭️look-queue/🧪️tests/🎣️detach` (1): an owner that asks while its batch is queued leaves the batch and is answered in about 5 ms while the slow owner of that batch takes 600 ms (base: 607 ms).
- `🧳️portable/🧪️tests/🪟️windows-spelling` (4): see section 3.
- Not run: the `genuine-birth` harness (a slice-lab style run, forbidden for this task); the law above uses the real `takePollV1`/`takePassV1` with a simulated fleet instead.

What the code does now: `takeFetchFirstV1` runs, beside the lanes of the take pass, an adopt loop (`takePollV1` with the new `hotOnly`: every 2 s it reads the published-names list of all origins and looks at every undecided listed owner, 2 at once, an owner again after 10 s; both numbers are in the take policy schema: `adoptConcurrency`, `adoptRetryMilliseconds`). A look is the existing look (farm child computes the key and verdict, the parent takes the record by key); the look queue now gives the asked owner a job of its own. An owner counts once whoever decided it. The census and the announcement to the fleet were already first in `resolveOwners`.

NOT built (said plainly): (a) cancelling a local inference when a valid record arrives while it runs (the claim in the fleet already keeps the other machines off an owner this machine computes, and the loop skips an owner that is taken before it starts); (b) a parent-side existence probe by key before any look (the key needs the assembly authority of native and release owners; the pool does not publish keys, only names; a probe by key would need `name key` lines in `pool-published-*.txt` from a new pool child); (c) shortest-expected-first for the local inference (R202's queue order covers the pool queue).

## 3. Portability of records across platforms

Sample of 60 neutral records (read-only, `diag/scan.ts`, last 4 h): the POSIX producers are clean except two things that hold only because every Mac shares the root `/Users/ueli/Documents/semio`: the `fact.pointer` and `fact.paths` of `field` rows stay absolute. The Windows producer (10 of the 60 records) leaves 14 472 raw `C:\semio\...` strings: in the JSON list in the `path` of the `resolve` rows (every element), in `fact.pointer`/`fact.paths` of the `field` row, plus the row `directoryExists C:\node_modules` (an ancestor probe, outside the root).
Cause (reproduced): `mapPathJson` caught every exception, so `neutralTextV1` refusing a backslash-spelled root (the intended "unportable, do not publish" path) was swallowed and the original text stayed in the row; and the element 0 / `field` fact object were not converted with the path function. Fix: the neutral direction maps every string element of a `resolve` list and the fact object of a `field` row through the path function (a path under the root becomes root token plus forward slashes, other text through the text function), the exception now propagates (the record is refused, not published raw). Law `🪟️windows-spelling`: rows from `C:\semio` (backslashes), `C:/semio` and `/Users/ueli/Documents/semio` are byte-identical in the neutral form; the taker localises them to either root; a list element that spells the root inside free text with backslashes is refused. Not changed: the `directoryExists`/`exists` rows of ancestors of the root (they differ by host, hold on any host when both say "absent").
Semantics revision: NOT bumped. The revision laws (`♻️revision`, `📏️revision` of the selected exchange) stay green; the digest covers inferrers, conformance corpus and graph schema, not the portable layer; records already stored stay valid and are read as before; records of Windows machines made before this landing stay unportable (they are inferred locally on other machines, no legacy path added).

## 4. Why fleet records are "stale" here (not determined; instrumentation landed)

Three named in this read (`ui-wgpu-layout-rs`, `fixture-generator-stdio-semio-v1-cad`, `ui-contract-conformance-rs`, fleet records of Mac producers 01, 03, 05 published 12:19): every file, field, listing, exists and resolve row HOLDS on this tree (0 bad rows, `diag/v6.ts`). The `closure` and `analysis` rows could not be judged by me outside the taker's context (a fresh process without the imported closure entries answered "does not hold" for 9, 9 and 2 closure rows with the questions `["some", paths]`, `["workspaceKeys", ...]`, `["generated"]`, which I do NOT count as evidence). So: the stale difference is, by elimination, in a closure or analysis row, not in a file. Earlier in the day, other stale Mac records showed the cause of the same family in plain rows: a `readdirNames` of `🖥️server/🎛️coordinator/📦️packages/🟦️typescript` (the listing holds `.🧾️store` and `node_modules`, both gitignored) and an `exists` of `🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts` (gitignored `🤖️generated/`, absent on the producer, present on this server): rows over gitignored, machine-local state differ between machines.
In this read 35 of 35 fleet records that were looked at were stale and 0 were adopted (R202's read: 13 adopted, 9 stale); the stale owners are mostly native `-rs` owners plus `repo-build-reports` and `schema-source-closure-ts`. This, not the speed of the take pass, is what keeps `fetched` at 0.
Landings 2 (12:30:41) and 3 (13:00:00): the verdict "stale" now carries `staleRow` (first row that does not hold: kind, path, fact) and the read prints it (English and German), and the adopt loop no longer looks again every 10 s at an owner it found stale (law red on the base: 40 looks in 200 s; now one per 250 s). Both start with the NEXT read; I used my one interrupt before them.

## 5. Samples of the read from 12:14 UTC (R202's format; `samples.txt`)

Start 12:14:04 UTC (the 11:47 read never reached its owners, section 1), runner 38180, read 38186, shared closures prime 12:16 to 12:19, owners from 12:19:50. Census: 2960 of 4785 owner records present locally, 1825 to infer or fetch. This read carries R200 (Rust owner speed-up), R202's landings 1 to 3 (queue order, hot lane, isolated native looks, named stale owners, published-names poll) and my landing 1 (fetch-first adoption, look detach, Windows spelling); it does not carry my landings 2 (stale row) and 3 (stale back-off).

| time | phase | done/total · present/fetched/here/stale | per min (10 min) | procs below farm door | show reserved/observed MB | spare MB | fleet wave claimed/published/children | load |
|---|---|---|---|---|---|---|---|---|
| 12:14:44 | start |  |  | 1 | 14336/353 | 2934 | 324/297/21c | 22.1 |
| 12:17:24 | prime |  |  | 10 | 14336/1274 | 5080 | 396/359/23c | 32.5 |
| 12:18:35 | prime |  |  | 10 | 14336/1269 | 9015 | 452/406/27c | 37.1 |
| 12:19:50 | infer | 4/4785 · 4/0/0/0 · run 0 |  | 9 | 14336/1699 | 8710 | 498/436/30c | 28.0 |
| 12:21:00 | read | 20/4785 · 14/0/6/2 · run 18 | 13.62 | 14 | 14336/3253 | 242 | 546/474/34c | 28.3 |
| 12:23:44 | read | 36/4785 · 20/0/16/10 · run 20 | 8.20 | 14 | 14336/5669 | 3210 | 632/539/43c | 50.8 |
| 12:24:41 | read | 36/4785 · 20/0/16/11 · run 20 | 6.58 | 9 | 14336/7456 | 3599 | 673/577/42c | 42.6 |
| 12:26:44 | infer | 38/4785 · 25/0/13/16 · run 20 | 4.93 | 19 | 14336/9425 | 2626 | 730/633/42c | 67.8 |
| 12:27:54 | infer | 38/4785 · 25/0/13/16 · run 20 | 4.21 | 19 | 14336/9046 | 2185 | 744/651/40c | 63.2 |
| 12:31:05 | infer | 38/4785 · 25/0/13/16 · run 20 | 1.78 | 19 | 14336/8431 | 1168 | 854/746/41c | 53.5 |
| 12:33:53 | read | 46/4785 · 32/0/14/19 · run 20 | 0.99 | 10 | 14336/8871 | 3864 | 845/720/40c | 71.0 |
| 12:37:19 | infer | 50/4785 · 35/0/15/23 · run 20 | 1.13 | 11 | 14336/11300 | 5210 | 970/815/46c | 56.3 |
| 12:38:08 | infer | 50/4785 · 35/0/15/23 · run 20 | 1.17 | 9 | 14336/10675 | 5743 | 988/831/46c | 57.8 |
| 12:40:53 | read | 55/4785 · 37/0/18/23 · run 22 | 1.31 | 10 | 14336/9252 | 5606 | 1042/879/42c | 53.9 |
| 12:43:38 | infer | 57/4785 · 37/0/20/27 · run 22 | 1.51 | 17 | 14336/11004 | 3429 | 185/133/23c | 55.2 |
| 12:46:46 | infer | 58/4785 · 37/0/21/27 · run 22 | 0.93 | 16 | 14336/8899 | 7131 | 247/159/44c | 43.5 |
| 12:49:28 | read | 62/4785 · 38/0/24/27 · run 21 | 1.06 | 11 | 14336/8536 | 4695 | 444/331/55c | 45.0 |
| 12:52:15 | infer | 74/4785 · 42/0/32/29 · run 27 | 1.67 | 11 | 14336/10413 | 6374 | 590/459/55c | 43.2 |
| 12:57:40 | infer | 75/4785 · 42/0/33/34 · run 27 | 1.56 | 14 | 14336/11247 | 7253 | 831/687/61c | 36.0 |
| 13:02:55 | infer | 81/4785 · 42/0/39/35 · run 27 | 0.66 | 13 | 14336/10137 | 5951 | 989/820/58c | 41.5 |
| 13:05:36 | infer | 83/4785 · 43/0/40/35 · run 27 | 0.67 | 6 | 14336/5593 | 7654 | 1066/882/61c | 32.6 |
| 13:08:16 | infer | 83/4785 · 43/0/40/35 · run 27 | 0.75 | 6 | 14336/6633 | 5807 | 1088/910/55c | 38.4 |
| 13:11:09 | read | 84/4785 · 43/0/41/35 · run 26 | 0.67 | 8 | 14336/7621 | 4812 | 1150/962/57c | 53.2 |
| 13:13:50 | infer | 85/4785 · 43/0/42/35 · run 27 | 0.37 | 6 | 14336/7698 | 5902 | 1188/1002/59c | 35.2 |

(Columns as in R202; "procs below farm door" = farm children plus helpers; "per min" = the sampler's 10-minute window; the first rows after 12:19:50 count owners already present locally.)

State after 30 minutes (12:44): 57 of 4785 done (37 present, 0 fetched, 20 inferred here, 27 stale), 1.5 per minute; 22 owners running, 82 "parked" lines (all lanes of owners parked 100 to 1000 s with 0 rows: "waiting for a farm child to look at the owner"), 4 farm pools of 1, 1, 4, 3 children, every child busy, 15 jobs queued, no stealing between pools.
State after 60 minutes (13:14): 85 of 4785 (43 present, 0 fetched, 42 here, 35 stale), 0.37 per minute, 27 running, 366 parked lines; booking/spare: the `show` grant stays 14 336 MB reserved (R200/R201 booking rule not effective: observed 5.6 to 11.3 GB), spare 1.2 to 9 GB; load 33 to 71 (Claude.app git polling and the slice lab, not this read). Pool wave counters: claimed 1188, published 1002 over 8 origins and 59 children at 13:13.
Do R200/R202's changes show? NOT in the rate or in parked lanes: at the same age the previous read had 73 done at 31 minutes, 83 at 60 (0.47 per minute); this one 57 and 85 (0.37 per minute): the same shape (a burst of owners already present, then about 1 per minute). The farm children are not CPU bound: the nine children sit at 0.2 to 5 percent CPU (stack samples of a child: all threads in `__ulock_wait2`/`kevent`), the parent at 1 percent, `repo leases status` shows 0 waiting leases and 0 waiting machine admissions, yet 15 to 27 owners are "running" for 1000 s with 0 rows and four pools hold 15 queued jobs. What the children wait for is NOT identified (it is a wait, not work). Fetch-first cannot show an effect while every fleet record it takes is stale.

## 6. Projection and answer to "does a whole read finish"

No whole read finished and none can at this rate: 4700 owners left at 0.4 to 1.5 per minute is 52 hours or more; a read that adopted the fleet's records would finish in about the fleet's time (the pool publishes about 40 records per minute over 8 machines: roughly 1.5 hours for 1825 owners) if those records held, which they do not on this machine. Two things decide whether a read finishes: (1) why 35 of 35 fleet records are stale on this tree (next read prints the row), (2) why nine farm children sit idle while 15 to 27 owners are marked running and jobs are queued.

## 7. Not verified, not built, what others must know

- The effect of my landings 2 and 3 in a live read (the read in progress started before them; I have no second interrupt: a coordinator interrupt of the runner `kill -INT <pid in graph-server/read-record.json>` starts a read 10 minutes later that carries them and prints the stale row).
- Cancelling a running local inference when a record arrives (not built); a probe by key before any look (the pool publishes names only); `genuine-birth` harness run (not run); the `directoryExists`/`exists` rows of ancestors of the root stay host-specific.
- Windows records already in the store stay unportable until the Windows pool restarts with the new code (pool roles need not restart on a Mac; the Windows producer needs the landing 1 portable layer).
- Law files: `law/` in the scripts folder; staged runs used `scratchpad/R204/t.sh` with the module-map preload (the observation module cannot be staged away from its directory: its law ran live under the lock with restore).
