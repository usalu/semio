# R203: why six machines each compiled the probe, and what is shared today (2026-11-18, read 11:35 to 12:05 UTC)

Answer in one line: **task results are not shared (0 hits ever, every run a new hash); cargo units ARE shared (about 690 to 730 foreign units consumed per probe run by m01, m08, m10); owner records are shared (88 % of 1.89 M reads cross-machine).** The Rust task is rebuilt per tree state because its hash moves with every tooling landing and every peer edit in its ~460 crate closure; the crate-level unit cache carries the sharing, and a probe run on a warm fleet is NOT 530 crates of compiling.

Sources (all read-only): ready logs and `ready-timings.json` in `/Users/Shared/semio-store/v1/origins/<m>/`; cache server `.cache/f/cache-server/{v1/main,v1/.meta,fleet/provenance.jsonl,fleet/consumption.json,v1/claims.jsonl}`; mtimes of the Cargo closure. Scratch/scripts: `scratchpad/R203/` (`closure.mjs`, `changed.sh`, `units.mjs`, `prov*.mjs`, `claims.mjs`), kept: `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R203/`.

## 0. New evidence sources that exist (nobody had used them)
* `fleet/provenance.jsonl` (134 MB): one row per first put `{n,k,y,o,t}` = namespace, kind (`nx`, `ac`, `cas`), key, ORIGIN machine, ms. This answers "stored by whom when" for every Nx task result and every unit, also for entries that were superseded later.
* `fleet/consumption.json`: `producer>consumer: {hits, units}` of action results READ (units = compiled units among them), since 2026-11-15 18:01 UTC. Cumulative, not per run.
* Unit entries are REAPI ActionResults containing `semio-unit-cache` and `out/lib<crate>-<metadata>.rlib` (6 refs). A fleet HIT calls `claimSlot` (`unit-cache/🛰️fleet/🟦️.ts:68-74`), which rewrites the entry's `storedAt` in `.meta`; a plain read never does (`💾️store/🟦️.ts` `claim`: "a read never does this"). So `storedAt` inside a run window on an entry whose FIRST put (provenance) is earlier = that machine restored it.

## 1. Task 1: per machine and run (UTC, 2026-11-18)

Cacheable tasks of the probe: 3 of 7 (`dsl-derive-rs:generate-schema-index` T1, `norm-din4108:build` T2, `norm-din4108-rs:build` R). The Nx hash is not logged anywhere; the hashes below are KEYS OF PUTS from the provenance journal, attributed to a task only where the entry's slot or size proves it. "Present before start" is read from the journal (a hit never writes a row).

| machine | Nx run | Cache | puts of the run (key, time, by) | result present before start? | outcome |
|---|---|---|---|---|---|
| g1-m08 | 03:39:52 to 04:04:35 | 0/7 | `8466745029782334868` 03:40:28 (36 s after start: T1) | none today | miss; R compiled 23m36 |
| g1-m07 | 03:43:38 to 04:06:22 | 0/7 | `15132650495004308041` 03:43:33 (T1) | T1 of m08 existed under ANOTHER key (8466...); R none | miss; R 22m13; R key = m03's (slot `storedAt` refresh 04:06:02) |
| g1-m03 | 03:49:08 to 03:58:30 | 0/7 | `15698478355623763392` 03:49:55 (T1, slot `7692e8...`), `7427260498132517366` 03:58:49 (R, 1.22 MB) | T1 of m08 and m07 under two OTHER keys; R none | miss; R 8m42 (units restored from m07/m08, see 3) |
| g1-m10 | about 04:03:00 to 04:04:33 | 0/4 then FAILED | claim `15170646253472068348` (T1, claims journal of R179) | R and one small entry existed; T1 key differs from all three stored | miss on T1, then hit on a small entry -> R180 custody refusal (the exclusive-in-shared bug; fixed 04:33) |
| g1-m10 | stage try 04:24:53 to 05:07:30 | failed (stage) | `5435418261525268206` 04:58:32 (R finished before the stage failed) | `7427...` (m03) existed; m10's key differs | miss, class (b) 04:13 engine edit, see 2 |
| g1-m10 | 05:55:04 to 06:08:54 | 0/7 | `15654663038477575698` 06:01:08 (4654 B), `1135565722543611963` 06:08:50 (R) | `7427...`, `5435...` existed, both under the pre-R181 engine digest | miss, class (b) R181 05:02 |
| g1-m02 | not an Nx run of the stage | stage: "3 up to date" | `14770878279894858595` 08:50:51, `17659045328307640942` 09:15:00, `6674947145143656807` 09:16:58 (another run on m02, not the ready stage) | n/a | the ready stage at 09:20 found its 3 tasks "have the recorded key" in m02's OWN task-key pointers (`createTaskRecorderV1`, per-host `.cache/b/g/m/task-keys`): local, not fleet. Peak 2825 MB because nothing was built |
| g1-m01 | 10:07:51 to 10:19:04 | 0/7 | `14857313210561908297` 10:08:19 (T1, same slot `7692e8...` as m03's), `6995988290010011912` 10:19:00 (R, 1.2 MB) | m02's `6674...` (09:16:58) and m10's `1135...`, `15654...` existed: different keys | miss; R 10m38; ledger of units in 3 |

Graph state digests logged: m03 `30ed064446df`, m10 `c219d6bd9055`, m02 `440cffa5563d` (m01/m07/m08 logs are cut at the head). The ready digest `70ffaf34...` is the same on all six (stage inputs, not tree state). The tree itself is ONE live tree replicated within a second, so two machines differ only in WHEN they hashed, never in WHAT tree they saw: any difference of hash between machines is a time difference (peer edit, landing) or a machine-specific value.

Numbers: 12 task-result puts on 2026-11-18, 12 distinct keys. The only repeated keys are the Rust key of m03/m07/m08 (3 machines, one key) and the T1 slot of m03/m01. No machine has ever been served a task result (`Cache: 0/7` on all six; the cache server keeps one entry per task slot, so m03's, m07's and m10's entries are gone and only m01's R entry `6995...` and m02's/m10's small ones remain).

## 2. Tasks 2 and 3: why every miss was a miss

I did NOT run the Nx hasher (that is a whole-workspace graph read, forbidden here), so the per-input diff of two hashes is not reconstructed. What is established instead is the moved-input class per pair, from landings and from the mtimes of the closure.

Closure approximation: the path-dependency closure of `semio-s-artifact-norm-din4108` (`Cargo.toml` + workspace dependency table) is 457 crate directories (`closure.json`); files edited in them, tests/fixtures/generated excluded (the Nx `production` input), by mtime (= when the live tree received the edit):

| from start of | to start of | closure files edited | landings in between that move the hash |
|---|---|---|---|
| m08 03:39:52 | m07 03:43:38 | 0 | none |
| m07 03:43:38 | m03 03:49:08 | 1 (03:47:33 renderer `container.tsx`) | none |
| m03 03:49:08 | m10 04:03:00 | 2 (03:55:25 renderer staging) | none |
| m10 04:03:00 | m10 04:25:00 | 3 (04:07:46 vite builder, 04:07:56 native-transpile, 04:23:09 `owned-runtime` .rs) | (b) 04:13:08 to 04:13:31 live edits of keyed engine files (R179 7.2): engine digest moved |
| m10 04:25:00 | m10 05:55:04 | 7 (04:46 ui accessibility, 05:04:30 limits Cargo.toml, 05:20:10 to 05:20:28 five .rs) | (b) R181 05:02 narrowed the engine digest: every task hash moved once |
| m10 05:55:04 | m02 ~09:05 | 14 | (b) R188 06:02/06:30 (cargo target-dir name), R189/R192 |
| m02 ~09:05 | m01 10:07:51 | 3 (09:26:04 `ui/styling .css`, 10:06:45 and 10:07:05 native-transpile project.json/script.ts) | (b) R198 09:30 (local unit key; its report says Nx hashes do not move) |
| m10 05:55:04 | m01 10:07:51 | 17 distinct files (13 `.rs`/`Cargo.toml`/`project.json` crates) | |

Over 2026-11-17 22:00 to 2026-11-18 12:00 UTC: 61 edited closure files in 29 bursts; the gap between bursts is 6 to 83 minutes, median 21 minutes (gaps in minutes: 10 41 29 31 83 18 14 38 6 10 10 10 46 8 12 16 23 18 16 51 47 21 17 41 35 34 40 66). A probe run takes 9 to 24 minutes. So **class (a) dominates over any interval longer than about 20 minutes** (m10 to m01: 17 files, 4 h 12 min), and the three concurrent starts (m08, m07, m03: 0 to 2 files apart) did not miss because of edits but because they ran at the same time (3, below). Classification per miss:

* m07, m03 (T1 only): T1 hash differs from the machine before it although the closure edits are 0 and 1 and no landing occurred. NOT explained by production sources. T1 has no declared `inputs` (so Nx's `default`/`^default`, which include tests, fixtures and stories of its dependencies) AND it rewrites a tracked 2.2 MB file `✨️derive/🗂️schema-document-index.json` (outputs `[]`, `cache:true`, content = projection of the whole schema corpus, R179 7.3). Observed: that file was rewritten at 10:08:12 by m01's T1 (its mtime here, 7 s after m01's put) and shows `M` against HEAD (2682 insertions, 2677 deletions). The replica copies it to all machines, so the NEXT machine hashes different bytes. Classification (d) (generated file rewritten by a task) is the best-supported cause; the hash itself was not recomputed, so it is "likely", not proven.
* m10 04:24 run: (b) engine files edited live 04:13 plus (a) `owned-runtime`.
* m10 05:55 run: (b) R181 (05:02) moved every hash once; the entries of m03/m07/m10 (04:58) were all under the old digest.
* m01 10:07 run: (a) 17 closure files since m10's key, (b) R188/R198 tooling, (d) the index file. The current entries (m02 09:16:58, m10 06:08) differ from m01's key.
* (c) machine-specific values: none found. R179 measured the engine digest independent of HOME/cwd/state dir; `runtime` inputs print `darwinarm64` and `1.4.2`; `SEMIO_BUILD_MODE` is the only env input of `sharedGlobals` and was not compared (open).

Misses with the SAME hash existing (task 3): **none observed**. The one hit path ever reached was m10 at 04:03 (a remote hit that failed on the custody bug, fixed by R180 at 04:33 and not exercised since: no machine has run the three tasks after 04:33 while an equal key existed). The three concurrent starts of 03:39 to 03:49 could not hit (entries were not stored yet) and could not wait (claim lapsed after 120 s: R179, fixed 04:34). `Cache: 0/7` is Nx's own count and is correct for every row above.

## 3. Task 4: cargo units (measured, then the ledger added)

Facts, all from the cache server:
* Consumption since 11-15 18:01 UTC: 1,892,187 action-result reads, 1,667,531 (88.1 %) between different machines; unit reads 4,077 of which 3,223 (79 %) cross-machine. Foreign units consumed by consumer: m01 727 (m07 277, m02 353, m03 60, m08 37), m10 701 (m07 443, m03 207, m08 51), m08 686, m03 413, m02 397, m07 299. These sums match one probe run each for m01, m08, m10.
* m01's run window 10:07:51 to 10:19:10: 688 unit entries had their `storedAt` refreshed, every one with an EARLIER first put in the journal (362 first stored by m07 272, m03 53, m08 37 at 03:xx, 324 by m02 at 08:xx/09:xx, 2 by m03 on 11-15) and **0 new unit puts by m01**. That is, of the 756 units of the Rust task about **688 (91 %) were restored from the fleet and at most 68 were built locally**, none of the 688 compiled again.
* m10's window 05:55 to 06:09: 3 new unit puts, the rest restored (consumption 701). m03/m07/m08 at 03:39 to 04:06 divided the units among themselves through unit claims: m07 uploaded 276, m03 76, m08 37 (389 distinct units) and each consumed the others' (m08 consumed 437 from m07 and 249 from m03 over its life).
* So the 10m38 of m01's Rust task is not "compile 530 crates": at most 68 units plus restoring 688 and the cone of crates above the peer-edited ones (a crate that depends on an edited crate gets a new unit key and is rebuilt; din4108 and its layers sit at the top of that cone).
* Caveat: the restore count of m01 is inferred from the claim-on-hit rule and the journal; the logs have no unit lines (confirmed: no `[units]` text existed anywhere before today).

Added (landed 12:00:01 to 12:00:12 UTC, entry in `landings.md` "R203 landing", script `🧰️scripts/R203/land.sh`, restore rehearsed on a scratch root with a forced red and the base-differs path):
* `[units] N restored local, M restored fleet, K compiled (S stored, X not shared)` plus the German line, printed at the end of every Nx run from the growth of the five counters in `.cache/f/unit-counters` (`📊️counters` `unitLedgerV1`, `unitLedgerLinesV1`; printed from the `finally` of `⚡️caching/🚀️bootstrap/📜️script.ts`).
* `[nx-task] <project>:<target>[:<configuration>] <nx task hash> stored|hit` for every task whose cache output the run created or read (`🪢️slot-settle` `nxTaskLedgerV1`, `nxTaskLedgerLineV1`; `settleNxSlotsV1` returns `tasks`). Two ready logs can now be compared with `diff`: this settles section 1's open "hash per machine".
* Proof: new laws `📊️counters/🧪️tests` (3: real counter files of a workspace, never negative, English and German lines equal an independent python rendering over 200 seeded pairs) and `🪢️slot-settle/🧪️tests` (2: stored/hit/old/uncached rows, settlement carries the ledger and still fails soft). Red on the base files (exports missing), green staged (5/0) and green live under the lock (3/0 and 2/0, import smoke ok, `script.ts` transpiles). Existing `unit-cache/🛰️fleet/🧪️tests` staged with the new counters: 17 pass, 0 fail (246 s under load 26 to 36). Neither file is in the engine-digest surface: no Nx task hash moves.
* Not verified: the real end of an Nx run with the new lines (needs a worker's next run; running processes keep the old code); `SEMIO_UNIT_CACHE_WORKSPACE_ROOT` must equal the worker's root for the counters to count (the launcher sets it, `🚪️launch/🟦️.ts:120`).

## 4. Task 5: is anything shared between machines today?

| kind | shared? | numbers |
|---|---|---|
| owner records / graph documents (`ac`, non-unit) | **yes** | 1,667,531 of 1,892,187 reads cross-machine (88 %), 224,656 own |
| cargo units | **yes** | 3,223 cross-machine unit reads; m01 restored about 688 of 756 units on 10:07 to 10:19 with 0 new uploads; m10 701, m08 686 |
| Nx task results | **no** | 12 puts, 12 distinct keys on 11-18; `Cache: 0/7` on m08, m07, m03, m10, m01; m02 never ran Nx (local "up to date" record); the only mechanism that could have served one (R180 hit path) was broken until 04:33 and no equal key has met a later run since |

Why task results do not share yet, in order of weight: (1) every landing in the engine surface and every closure edit changes the key (17 closure files, 4 h, 3 landings between m10 and m01); (2) T1 rewrites a tracked file that is an input of dependants and the replica spreads it; (3) the earlier runs overlapped (fixed by R179). Items 1 and 3 are now narrow (R181, R179), so the next machine that starts the probe while ANOTHER machine's R key is current and the closure has not changed will either hit it or wait for the holder.

What the next landed change makes shareable: with the 12:00 landing the next Nx run of every worker prints the two ledgers; from that moment "same hash on two machines" and "units restored vs compiled" are one `grep` over the ready logs. The change that would make T1 shareable is NOT done: give `dsl-derive-rs:generate-schema-index` declared inputs (the schema corpus) and declare the index as its output or stop tracking it (a pure generated file in the closure); needs its own law (the generator output is a pure function of the schema tree: compare with `mutationSchemaDocumentIndexProjection`) and a human decision on the tracked file.

## 5. Not verified / for others
* Per-input hash diffs were not computed (no hasher run). The closure is the Cargo path closure (457 directories), not Nx's project closure; dev-dependencies and `sharedGlobals` are not in it; mtimes are the times this machine received the files.
* Which key belongs to T2 (`norm-din4108:build`) is unknown: no put can be attributed to it; m10 hit a small entry at 04:03, so one of the small keys is T2's or an entry I attributed to T1 is not.
* m02's "3 up to date" came from m02's own pointer store; how m02 produced the three puts (08:50, 09:15, 09:16) is not in any log I could read.
* R202 (adoption of fleet records) and R197 (lab) were not touched. The claims journal starts 10:32 UTC (rotated), so claims of earlier runs cannot be read.
* The T1 self-modification cause is the best-supported hypothesis, not a proof.
