# R202: the fresh whole-workspace read from 10:32 UTC: samples, adoption of fleet records, priming, queue order (living report)

Written from 2026-11-18 10:33 UTC. Scripts: `FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R202/` (`sample.py` = the sampler, `table.py` = the table below, `patch.py`/`patch2.py`, `land1.sh`/`land2.sh`, `law/`, `t.sh` staged test runner with the module-map preload). Scratch (outside the worktree): `scratchpad/R202/` (`samples.txt` = every raw sample, `diag/verify2.ts` = the record check of section 3, `stage/`). Nothing was started or stopped by me except `sample` (read-only `sample(1)` of two processes) and my own foreground/background samplers; no slice-lab run, no download, no kill.

## 1. Start of the fresh read

- Coordinator interrupt 10:32:15 UTC. `read-record.json`: refresh read started **10:32:44.975 UTC** (runner 15218, read 15224, door/farm 15334). That is 29 s after the interrupt, not 10 minutes. Cause (sidecar log `build-agent.log`, 10:32:38): `graph-server: refreshing the fleet generation in the background (26 s quiet, never since the last)`. The graph server was restarted by the sidecar's code-edit rule (R201 landing 10:31 touched `🏗️build/🔣️policy.json`: "restarting after 90 s without a further change"), a restarted server has `refreshedAt = never`, `age = Infinity >= refreshIntervalMilliseconds`, and the quiet check (`quiet >= refreshQuietMilliseconds`, 26 s here, not 180 s) passed. So the 600 s rule and the quiet rule were NOT exercised and are NOT the reason for any delay today; the earlier 10-minute delay (R194) happens only when the server lives on after an interrupt (`age` small). The tree was quiet 26 s; "the tree is never quiet" is false for the 180 s of R194's reading only if the quiet threshold is 180 s; the live threshold in the schema is the one read above. No rule change is needed, none made.
- Phases: source capture/served 10:32:44 to 10:36:20 (dependency receipt stale: `env:BUN_INSTALL`, `dependencies:generation-verify` exclusive 16 s, `build:cargo-targets`); `shared closures in a child` 10:36:20 to about 10:41:40 (**5.3 minutes**); owners from 10:41:40.
- Code in this read: R192 loader fix (set-asides stayed **0** the whole time), published-names poll (R194), R201 booking rule (not yet effective: the `show` grant is 14 336 MB while it is younger than 900 s; spare 4.6 to 9.9 GB afterwards), late-refusal fix, directory-fact fix (R184). My landing of 11:00 does not reach this read (see section 6).

## 2. Samples (UTC; raw lines in `scratchpad/R202/samples.txt`)

| time | phase | done/total · present/fetched/here/stale | per min (10 min) | procs below farm door | show reserved/observed MB | spare MB | fleet wave claimed/published/children | load |
|---|---|---|---|---|---|---|---|---|
| 10:37:40 | prime |  |  | 11 | 14336/2096 | 9452 | 101/82/10c | 40.5 |
| 10:41:43 | infer | 0/4785 · 0/0/0/0 · run 0 |  | 11 | 14336/2656 | 9135 | 151/117/12c | 38.9 |
| 10:46:13 | infer | 18/4785 · 6/0/12/0 · run 21 | 4.00 | 10 | 14336/7102 | 3725 | 335/295/15c | 39.6 |
| 10:50:31 | infer | 25/4785 · 6/0/19/1 · run 20 | 2.84 | 11 | 14336/8148 | 5335 | 482/439/15c | 47.5 |
| 10:53:23 | infer | 32/4785 · 16/0/16/6 · run 25 | 2.52 | 13 | 14336/7395 | 5146 | 517/474/15c | 39.6 |
| 10:56:37 | infer | 42/4785 · 37/0/5/7 · run 27 | 2.31 | 21 | 14336/9225 | 5310 | 575/528/15c | 43.9 |
| 11:03:13 | infer | 73/4785 · 42/0/31/9 · run 26 | 3.90 | 10 | 14336/7983 | 5244 | 745/689/15c | 40.0 |
| 11:07:39 | infer | 78/4785 · 42/0/36/9 · run 27 | 3.26 | 10 | 14336/8983 | 7114 | 808/743/13c | 27.5 |
| 11:10:37 | infer | 82/4785 · 42/0/40/9 · run 27 | 2.72 | 12 | 14336/9676 | 7646 | 863/793/14c | 46.4 |
| 11:14:04 | infer | 83/4785 · 42/0/41/9 · run 27 | 0.92 | 10 | 14336/10048 | 6414 | 893/817/15c | 35.3 |
| 11:18:21 | infer | 83/4785 · 42/0/41/9 · run 27 | 0.47 | 9 | 14336/8401 | 6801 | 61/27/15c | 51.8 |
| 11:26:31 | infer | 85/4785 · 42/0/43/9 · run 27 | 0.18 | 7 | 14336/6694 | 7597 | 214/171/12c | 49.5 |
| 11:30:50 | infer | 85/4785 · 42/0/43/9 · run 27 | 0.16 | 8 | 14336/6988 | 6459 | 244/197/13c | 36.3 |


Columns: done/total = the ledger line of the read (`owners N of 4785 done`); present/fetched/here/stale = records found in the local store / taken from the fleet / inferred here / taken from the fleet but not holding here; procs below the farm door = farm children plus helpers (9 farm children from 10:42); fleet wave = sum over the origins whose `pool.json` names the read's wave `b42af294e180` (claimed, published, children).

## 3. The central question: does the read adopt what the fleet publishes?

**Answer: NO, and the cause is not what the 11:36 line suggests.**
- Correction to the 11:36 reading: "42 present" is the number of owners whose look has FINISHED, not the census. The census line of this read says `owner records present: 2951 of 4785` (08:12 read: 2948 of 4785; the previous read's start 10:32: 2948). So nothing invalidated the stored records between 08:12 and 10:43: R198's local key, R192, R195/R196 did not drop them (2951 vs 2948 files). All 42 finished local looks held (0 stale among local records). No stamp or key change to hunt.
- Why only 42 looks finished in 63 minutes: the look path (one farm child per batch of up to 6 owners, sequential inside the child: assembly authority + verification) took minutes per batch under load 30 to 50 on 10 cores; lanes of owners were parked 15 to 38 minutes with 0 rows. A look answers when its slowest owner is done, and the slowest are native (Cargo) owners (5288 Cargo.toml in the tree): plain owners waited with them. Farm busy children averaged 7.5 of 9 at 10:50 to 11:00, only 4 of 9 with 4 not started from 11:30 (the farm holds while the tree footprint passes 85% of the read's grant). Rate fell from 4.5/min (first minutes) to 0.10/min.
- The poll that adopts published records never ran: it starts only after the take pass over all owners ends; 45 minutes after the pass began there was still no `wanted` line. So 0 fetched means "not reached", not "rejected".
- Fleet hits that did happen: 13 fleet records fetched and seeded locally (markers `*.json.fleet`, 8 at 10:42:44 to 10:42:50, then one per 5 to 8 minutes); 9 reported "stale in the fleet" (rows do not hold here). Why those 9 are stale is NOT determined (the read does not log the owner or the failing row); landing 2 now names them.
- Quality of what the fleet publishes, measured on this machine (`diag/verify2.ts`: unpack, localise, check every file/field/listing/resolve row against the tree now; closure and analysis rows NOT checked): 400 records published in the last 90 minutes by 8 producers: 382 hold (95.5 percent); the 6 Mac producers 338 of 346 (97.7 percent); the Windows producer DESKTOP-6NSLEUO 44 of 54, all 10 failures are `resolve` rows spelled `C:\\semio\\...` that the portable layer (`localRecordV1`, kind `resolve` in `jsonKinds`) does not move to this root (a defect, not fixed). No key mismatch was found: fleet hits are found by key.
- The fleet is not the bottleneck: pool children end their wave in 3 to 63 s with 0 to 6 owners ("finished its wave after 23 s and 2 owners"); the store already holds most of the wave; pool counters tick (25 to 50 per minute) mostly on re-publication of about 600 distinct owner names across four ask waves of another machine (names files: 88bb 591, abd2 562, 97c7 559, 828d 277, this read's wave b42af 9 names).
- Not obtained: the concrete key pair (pool store key vs read ask key) for one owner; the markers hold digests that are not action-cache keys and the read logs no key.


## 4. Priming (shared closures in a child)

Measured: shared closures in a child 10:36:20 to about 10:41:40 = **5.3 minutes** (08:12 read: 19 to 24; R194 had one walk worker busy of five). At 10:38 the prime child had 4 walk workers busy at about 55 percent each (load 28 to 40); at 08:35 load was 54 to 56 with 54 percent system time and one worker busy. So there is no serial dependency or lock: `PRIME_CONCURRENCY_V1` = 4 recipes at once works when cores are free; the earlier cost was contention. Reuse by digest: entries are durable memo nodes checked against the tree, so a landing under the tool directories makes them stale again; whether this prime derived or loaded is unknown (counts were never logged). Landing 1 logs `shared closures primed in N s: recipes, derived, loaded, stale` for the next read. Nothing else changed.

## 5. Queue order

Landing 1: `farmQueueOrderV1` (pool queue served shortest expected first, aging so no owner waits more than about its own cost, sent-back jobs in front; costs from the cost table megabytes). Evidence from the live farm (134 parked lines): 4 pools of 1, 1, 5, 2 children, no stealing between pools; an idle child stood while another pool queued in 13 percent of the lines; queue depth averaged 2.8, so the ordering helps little in this read; the larger effect is landing 3 (native or release owners alone in their look batch) and landing 1's hot lane (fleet-published owners looked at first). All three act only in the next read. Law first: `⏱️queue-order` (python cross-check), `🔥️hot-lane`, `🧱️isolated-looks` (python cross-check), `🔬️stale-line`; red on the live code before.

## 6. Landings and effect

1 (11:00:00 to 11:00:26), 2 (11:32:02 to 11:32:07), 3 (11:35:12 to 11:35:20, 12 s after the window closed because the lock was held by R200 for 2 minutes): see `landings.md`. **The read was interrupted once with `kill -INT 15218` at 11:36:49 UTC** (authorised). It ended; the graph server shows `read none` at 11:37 and starts the next refresh by its 600 s rule, and the new read (R200 code + landings 1 to 3) started at **11:47:16 UTC** (`605 s since the last`, 19 s quiet: the 600 s rule holds after an interrupt; runner 98355, read 98361). Its measurements are NOT in this report: my 90 minutes of sampling ended with the old read; the sampler script `sample.py` works unchanged on the new one. Note the old read had run 64 minutes for 86 owners.

## 7. Projection and what is not verified

Old read: 4699 owners left at 0.10 to 3 per minute: it could not finish (26 hours at best, 800 at the end). Lower bound of the look design on this machine: 4785 owners / 6 per batch / 9 children at about 3 minutes per batch = about 4.4 hours, only if no child is held and the machine is not saturated. Not verified: any effect of landings 1 to 3 or of R200 in a live read; why the 9 fleet records are stale; the closure and analysis rows of fleet records; whether the Windows `resolve` rows are the only portable defect. Levers not built: stealing between pools; a parent-side import of fleet records without a child look for non-native owners (key computable in the parent); a persistent cross-child cache of native dependency roots (R200 area); raising the farm hold share; the Claude.app git polling and the slice lab that keep the machine at 0 percent idle (not touched).
