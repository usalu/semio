# Graph slice read performance (why every `repo run` waits hours)

Scope: `repo run workspace:components -- describe fem --profile prototype` (PID 17996, log `🗑️generated/pub-describe3.log`) spent 3 h 49 min (13 760 s) in "Reading the project graph slice" before it sealed; ~16 000 s wall until the describe's later phases. Measured here: where the time goes, one correct fix in the hold path, and what remains structural.

Method: stage timeline from the 17996 log (the wording of the status line is the same for every inventory phase, only the counters change), the persisted admission memo of the same scope (`.cache/b/g/m/selected/171969…admission.json`, 62 MB, 162 021 distinct observation rows, 425 owners, 769 436 owner-row references) replayed through the real `observationsHold*` functions in isolated bun scripts (`🧪️graph-read-perf/`, inputs only), `process.cpuUsage()` and `--cpu-prof`.

## Findings (measured)

1. **The host is saturated, so everything is 5-20x slower than the same code on a quiet machine.** `% Processor Time` 100 % on 16 logical CPUs (rustc builds, `MsMpEng` 190 %, several `python3 -` stdin hangs at 30 % each, a GitKraken `git.exe` PID 48368 with 25 000 CPU-s from Nov 20 now idle), D:/E:/G: spinning disks at 650-1 270 % disk time. The slice worker used only 22-29 % of one core (44540: 372 CPU-s in 28 min; 31072: 3 118 CPU-s in 3 h 20 min): it is waiting, not computing. `lstatSync` alone: 54 us warm, 170-2 100 us on the loaded host (first touch), single calls up to 1.9 s.
2. **A read is several full passes over the tree, not one.** Per round the loader does: inventory (memo hold of all owner rows) -> owner discovery (`owners-round`, 3 600 steps) -> `settledInventory(true)` (a second full verification) -> `facts-hold` over the union of observed rows (a third), and repeats the whole cycle while the membership or an observed row moved (`repairRounds: 4`; the log shows `membershipMoved:true, admissionMoved:true` and the `[graph-settle]` lines naming `✏️s\🔌️plugins\🏗️fem\…\Cargo.toml` and `…\📸️snapshot`: peers edit the very tree that is being read). Timeline of 17996 (seconds into the run): 0-225 service start; 225-7 600 cold inventory of the scope (no usable memo: its format key `fcdd8b…`/`9bf27c…` changes whenever one of the 8 files in `graphAdmissionFormatUrlsV1` is edited, four times today; the memo is then a miss for every scope); 7 600-10 418 owners round 1 (434 projects, 3 600 steps); 10 418-10 814 second inventory + facts hold; 10 814-11 244 owners round 2; 11 244 inventory again; worker/host restart at about 11 438 (progress back to 0 of 600, the in-memory verdict/owner state is lost); 11 438-14 000 restarted read with the memo (6 min memo check of 221 312 rows, 12 min discovery of the owners the memo no longer held, 15 min inventory with 854 208 verified memo rows, 3 min facts hold, 4 min seal/publish).
3. **One verification pass is 4-5 `exists/lstat/…` probes per directory entry that a listing already proves.** The memo has 86 618 `lstat`/`exists`/`fileExists`/`directoryExists` rows; 67 487 of them (78 %) have a parent directory whose own `readdir` row is verified in the same pass (31 698 `readdir` rows), yet every child was probed with its own syscall. Measured costs of the isolated hold per kind (warm cache, rows = distinct memo rows): `lstat` 14 s, `resolve` 23 s, `fileExists` 17 s, `exists` 9 s, `readdir` 10 s, `directoryExists` 3.5 s; one full pass over the 101 282 holding rows is 14-15 s warm and 120-240 s under the host load.
4. Not causes (measured): memo read/parse 0.7-1.3 s (62-154 MB), recording inside a capture (no extra rows), the shared round verdict cache (per-owner holds in one round cost 18 s vs 15 s for one hold over the distinct rows; without a shared round it would be 59 s), `holdMetadataPaths` thrash (small), `resolve` row de-duplication by directory (13 339 distinct of 13 457).
5. Service reload/recycle: the host reload identity only covers the 20 `serviceRevisionFiles` (host, pool, memory, reload modules), not the graph modules, so peer edits of graph code do not restart the host; the restart at 11 438 s is a worker recycle/replacement (new `bun` slice worker 44540 at 15:23, old one 31072 still alive with 2 GB). Each replacement starts the read over (`recycles`/`carried` keeps only the owner facts the pass reached).

## Root cause (what is wrong in the code, ranked)

* **R1, per-child probes next to a verified listing** (fixed here, see below): the verification of an observation set pays one syscall per child fact although the directory listing it verifies anyway proves the same fact. 78 % of the probe rows were avoidable.
* **R2, a live tree is read with a full-tree verification per pass and no change journal**: ~220 k rows are re-proven by syscalls three times per round and again per repair round, while the resident `🔭️watch` supervisor (native watcher, fences, trust state) already knows what changed. Cold/format-miss reads and every worker replacement pay the whole price again. A real fix (not done here, needs a design decision) is to let the host watcher answer "unchanged since fence N" for rows of verified directories, as `🕰️generations/🧭️frontier` already does for the generation graph (`stableDirectories`), and to key the admission memo on the facts a format edit actually changes, not on the whole format hash.
* **R3, environment**: CPU/disk saturation of the host (finding 1) multiplies every pass by 5-20x. Recommend the coordinator stops the `python3 -` stdin hangs (PIDs 40068, 15420, 25980, 35836, ~30 % CPU each, started 04:17-13:10) and checks `MsMpEng` exclusions for `C:\semio` and `C:\Users\Ueli\AppData\Local\semio` (I did not kill anything).

## Fix (R1)

File: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs`

* `listedChildHoldsV1(kind,path,fact,listed)` (new, next to `typedListingV1`): answers `lstat`, `exists`, `fileExists`, `directoryExists` rows from the round's typed listing of the parent when the parent is a `readdir` row of the same call (`listed`); the listing is read once through the existing round cache and is the same one the `readdir` row verifies. Only confirmations are derived: a missing name that equals a listed one case-insensitively, links, other node types and any disagreement fall through to the real probe, which stays the only authority (so a stale fact is still found by the probe). Directories above 64 entries use a lazily built exact/case-folded index.
* `observationsHoldInRoundV1`: builds `listed` from the `readdir` rows of the call and consults the helper in `check` before the generic probes. Applies to the memo check, the per-owner holds and the final fact hold alike.

Tests (new, `👁️observation/🧪️tests/📇️listed-children/`): `🔣️.json` (language-agnostic scenarios: still-holding facts, changed type / appeared / vanished, a 66-entry directory over the index threshold), `🐍️.py` (independent Python oracle of the child facts), `🟦️.ts` (bun): every verdict with and without the listing equals the oracle; with the listing a confirmed child costs 0 `lstatSync/statSync/existsSync` calls on its path and without it at least one; case-fold, link, and a 300-entry directory read once with 0 child probes. 4 pass / 0 fail natively (`bun test`, 19 s).

## Measurements before / after (isolated `observationsHoldWithinRoundV1`, 118 018 rows of kinds lstat/exists/fileExists/directoryExists/readdir, same memo, host under load, 3 runs each)

| | CPU user+sys | wall |
| --- | --- | --- |
| baseline (probe every child) | 28.1 / 28.4 / 28.5 s | 136 / 85 / 105 s (an earlier pair: 241 / 122 s) |
| listing proves children | 18.7 / 18.8 / 19.5 s (-33 %) | 82 / 69 / 76 s (-30 %; earlier pair 160 / 183 s) |

Kernel time falls from 20.2-20.8 s to 12.6-13.8 s. Wall time is noisy because the host is saturated; CPU time is the stable figure.

## Exact phase times of the 17996 read (the loader's own `[graph-phase]` summary, host log `%LOCALAPPDATA%\semio\repo\host\host-5ea94aed61d0.log`)

| phase | ms | meaning |
| --- | --- | --- |
| inventory 1: memo miss, scoped owner discovery + settle rounds + memo write | 7 298 213 | 2 h 01 min, cold: 435 owners x ~17 s |
| membership hash | 29 069 | |
| owners round 1 (`graph-original-inference`) | 2 867 711 | 48 min, 434 projects |
| inventory 2 (repair round 1, memo skipped for follow-ups) | 260 863 | |
| inventory 3 after the worker replacement: memo check 102 195 + discovery/write | 2 246 854 | 37 min: the memo hold failed (sources moved), the owners were re-observed |
| graph expansion (`portableInputs`) | 220 079 | 3.7 min |
| publication (write 70 MB graph + receipt) | 222 843 | 3.7 min |

So 7 298 + 2 247 s (74 %) is owner discovery, 2 868 s (21 %) is project inference; the hold fix addresses the verification share inside both (per-owner memo holds counted 854 208 row references).

## End to end

My own run `repo run workspace:components -- describe fem --profile prototype` (repo.exe PID 54964, started 17:11:53, log `🗑️generated/perf-describe.log`) was still in the cold graph read when I had to stop waiting (17:42, 1 700 s in): the sealed receipt of the scope no longer held (peers kept editing `✏️s/🔌️plugins/🏗️fem`), the whole-memo hold failed after 44 800 verified rows, and the scoped owner discovery re-observed owners (53 000 distinct observations so far, same shape as the first 17996 inventory). It uses the patched hold (the slice worker loads the graph modules fresh from disk, the host does not reload for them). At the coordinator's request I killed it (`taskkill /T /F /PID 54964`, 17:51, 2 288 s into the read, still in cold discovery) because it competed with PID 17996. The describe did NOT complete within my time; the earlier 17996 run (unpatched) finished its graph read at about 14 000 s and was in the build phase at 16:59.

## Still open (needs a design decision, not done)

1. Change-journal based verification (R2) so a read is O(changes) instead of O(rows x passes x rounds); the persisted per-scope admission memo is the right substrate (62-154 MB per scope, 15 scopes, 2.3 GB on disk).
2. A graph admission format change invalidates every scope at once (cold start of hours); narrower keys (per file kind actually consumed) would keep most owner facts.
3. Worker recycle/replacement should carry the verified-row verdicts, not only owner facts.
4. `resolve` rows cost 1.7 ms each warm (13 457 rows, 23 s per pass), the largest single remaining hold cost.
5. Every `repo` invocation starts with `build:source-admission` (`source.capture`), also `repo service host status`; unrelated to the graph read but adds a fixed minute per command.
