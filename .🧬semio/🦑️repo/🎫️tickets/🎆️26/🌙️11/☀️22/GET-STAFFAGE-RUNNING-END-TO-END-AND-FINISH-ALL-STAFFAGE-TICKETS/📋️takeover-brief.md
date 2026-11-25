# 📋️ Takeover Brief for Multiview Fixers (2026-11-25 09:00)

Every fixer of the earlier chats stopped between 04:30 and 08:00 on 2026-11-25 without a final report (no process is left, the capacity queue was empty at 08:50). You take over ONE batch and bring it to the end.

## Read first, in this order

1. `R/CLAUDE.md` (binding), with `R` = `/Users/ueli/Documents/semio`.
2. `U/📋️multiview-brief.md`: the job, the rules, the proof list and the memory rule. All of it holds for you unchanged. `U` = `R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-STAFFAGE-RUNNING-END-TO-END-AND-FINISH-ALL-STAFFAGE-TICKETS`.
3. `U/multiview-<BATCH>.md` if it exists: the interim report of your predecessors. It is a claim, not a proof.
4. Your folder `U/🧰️matrix/<BATCH>/` (scripts, ops, plans, backups) and `U/🗑️generated/matrix/<BATCH>/` (trial copies, logs of runs; a log of a run that was killed proves nothing).
5. `U/multiview-MR.md` section 4 (recipe for people) and `U/multiview-memory.md` (the matrix generator no longer loads the whole catalog; one asset process stays below 400 MB if that repair is in place, measure before you rely on it).

## Takeover steps

1. Measure the state of every asset of `U/🧰️matrix/batches/<BATCH>.tsv` now, on the catalog files as they are: authors' gate `T/🔎️scan.ts --language en <ids…>` and the gate your predecessors used. Compare each catalog file with the trial copy and the backup in your folders, so that you know per asset whether it is untouched, written and proven, written and unproven, or half done. A catalog file that a predecessor wrote but never proved is proven by you or put back to its best proven state.
2. Continue with the predecessors' tools and ops where they are sound; do not start a second parallel tool chain for the same asset.
3. Bring every asset to the proof list of the multiview brief (section "Job", point 3). The matrix confirmation of an asset (`T/📊️matrix.ts --out <dir> --jobs 1 --assets <id>`) is mandatory and runs to its end.
4. Rewrite `U/multiview-<BATCH>.md` as one coherent report (no stacked interim sections). Its first line is `FINAL` only when every asset of the batch is either clean with all proofs or listed under "open" with numbers. Update the file after every finished asset, so that a stop loses nothing: first line `IN WORK` until then.

## Rules on top of the multiview brief

- Every bun, node or python run that renders or measures goes through `repo capacity run --memory=<MB> -- timeout <s> …` from the repo root (7000 matrix until you measured less, 5000 check CLI, 2500 vitest, 1000 scans). One heavy run at a time. About twelve fixers share this 16 GB machine.
- Never wait idle on the queue for more than its turn: prepare the next asset while a run waits.
- No sub-agents. Never stop to ask. Never stop halfway: the batch ends with the FINAL report.
- Do not delete `U/🗑️generated/matrix/<BATCH>` before your FINAL report is written; delete it then.
- Final message to the coordinator: the report path and at most six lines (clean count, open assets with their cell counts, requests filed in `U/requests.md`).

## Queue rule (added 2026-11-25 09:20, binding, replaces the reservation figures above)

Measured at 09:15: 3 runs admitted, 34 waiting, some for 30 minutes. The queue counts what a run RESERVES, not what it holds; the three admitted runs reserved 4300 MB and held 1175 MB, and the machine has only about 4.5 GB above its floor. Oversized reservations are what blocks everybody.

- The whole-catalog parse is gone from the tools (MM, 09:02 to 09:05): `📊️matrix.ts --assets <id>`, the check CLI with `--asset` and `🖨️proofs.ts --assets` read single documents. Measured: check CLI 180 MB, matrix process of a person 374 MB.
- Reserve: 800 MB for `🔎️scan.ts`, gates and small scripts; 1000 MB for the check CLI with `--asset`; 1200 MB for `📊️matrix.ts --assets <one id>`; 2500 MB for a vitest suite with a name filter. More only after you measured the peak of that very run (`/usr/bin/time -l`), then the peak plus 30 %.
- At most TWO requests of yours in the queue at any time (running or waiting). Withdraw the others now and chain them in one script instead.
- A request of 5000 MB or more can never be admitted today: withdraw it.
- Added by MM at 09:30, measured: **a tree needs more than 1200 MB in `📊️matrix.ts`**. The asset process of `olive-tree` (3 MB document) peaked at 1,204 MB resident, the one of `small-leaved-lime` (23 MB document) held 1,579 MB at scale entry 10 of 12 when its run was cut. Reserve **2500 MB for `📊️matrix.ts --assets <one tree or shrub of vegetation>`** until MM appends the peak of the largest trees here. People, animals and rigid assets stay at 1200 MB (rigid 260 MB, people 300 to 470 MB, the generator itself 27 MB).
- Added by MM at 12:00, measured: **the vitest suite `🔭️multiview-coverage` peaks at 2,802 MB** (process tree, 41/41 passed in 423 s on the loaded host; the test worker alone 2,597 MB in its last 20 s): reserve **3600 MB** for this one suite, 2500 MB is too little for it. Check CLI `--asset european-beech`: 757 MB peak (was 3,679 MB), so 1000 MB holds for large plants too. `📊️matrix.ts --assets weeping-willow` held at most 1,231 MB in 90 minutes: 2500 MB for a tree stands. `saguaro-cactus` in `📊️matrix.ts`: 320 MB (it is slow, not large; about 2.3 processor hours, so give it `--timeout-s 43200` and a `timeout` to match).

## Second takeover (added 2026-11-25 16:10, binding)

All fixers stopped between 13:10 and 14:30 at the Fable usage limit. The dev named the model for the rest: Opus 5.5 at medium effort. You continue one lane; everything above holds, with these additions:

1. **Leftover processes.** Some chains of the stopped fixers still run or wait (`ps -axww -o pid,ppid,etime,command | grep 'matrix/<BATCH>'`). Look at yours first: adopt a run that still proves the file that is in the catalog now, stop (SIGTERM, only processes whose command line names YOUR batch folder) what is stale, waits for a dead parent, or repeats a proof. They count against your two queue requests. Never touch a process of another batch.
2. **The engine moved** at 11:10 (MX) and again up to 12:34 (MX-2, loft shapes, not yet proven). Do NOT repeat a finished matrix or proof only because the engine changed: the full matrix and the full suites run once at the end on the frozen catalog and decide. Prove an asset when YOU change its file, once, on the engine of that moment.
3. **Evidence.** A result counts only when you read it in a log that ran to its end. Say "not run" for what did not run; a claim of a predecessor without its log is "claimed".
4. **Waiting.** Do not end your turn while your own runs are pending and work is left that does not depend on them. When only runs are left, wait for them with one background command that ends when they end.
5. A second coordinator works on the Linux worker g1-m04 (site, React target, proposals in `🧰️linux/`). It writes no catalog asset, no engine, no `T`. Do not touch `🎯️targets/⚛️react` or `🌐️site`.

## Third takeover (added 2026-11-25 19:00, binding)

The Opus fixers of the third chat stopped with their chat between 17:00 and 18:55; several of their background chains (`ps -axww -o pid,ppid,etime,command | grep 'matrix/<BATCH>'`) still run. A fourth coordinator chat took over. Models named by the dev: Fable 5.2 (medium) for every lane that models catalog assets, Sonnet 5.5 for engine, suite and site execution, Haiku 5.5 for read-only audits.

1. **Adopt first.** Before you queue anything, list the live processes of YOUR batch folder and the newest files of `🧰️matrix/<BATCH>` and `🗑️generated/matrix/<BATCH>`. A chain that still proves the file now in the catalog is yours: wait for it in the background while you prepare the next asset. Stop only stale chains of your batch.
2. **The engine moved again at 17:48** (MM's `engine.patch`, landed by MX-2: byte-identical cells, matrix of a person about 15x faster). The matrix of `jogger` went from 3014 s to 172 s, so a matrix that is still missing is cheap now: run it rather than leaving it as "not run".
3. **Finish.** The batch ends with the FINAL report only. Every asset is either clean with all proofs or listed under "open" with its numbers and the reason it cannot be cured in the asset (then file a `## multiview …` request in `U/requests.md`).
4. Never edit `T/📊️matrix.ts`, `T/🗃️cells.ts` (MM) or the shared engine (MX-2) from a batch lane; file a request.
5. Final message to the coordinator: report path and at most six lines.

## Reservation figures of MM (added 2026-11-25 20:05, measured on the engine of 17:48; replaces the figures above where it is lower)

Source: `harvest.py` over the 111 matrix records finished since 17:48 in `U/🗑️generated/matrix/*/` (seconds are wall time on the loaded host, load 20 to 30 on 10 cores), the peaks of the generator's own runs (`🧰️matrix/MM`, `guarded.sh`, `/usr/bin/time -l`) and the observed memory of running grants in `repo capacity status` (about 350 MB average while their reservations were 800 to 2500 MB). The other fixers' records carry seconds, not peaks: the figures below are the highest peak measured for the kind plus 30 %, rounded up. From now on `📊️matrix.ts` prints `peak <n> MB` (the largest resident set of the asset process) in its progress line of every ended asset.

| Run | Reserve | Measured |
| --- | --- | --- |
| `📊️matrix.ts --assets <person, animal, rigid asset>` | **800 MB** (not 1200) | rigid 260 MB, `finned-heat-exchanger` 317 MB, `jogger` 320 to 329 MB (tree of processes), `sleeping-person` 470 MB; generator 27 MB |
| `📊️matrix.ts --assets <tree or shrub, document below 1.5 MB>` | **1000 MB** (not 2500) | `atlas-cedar` 533 MB (before the landing) to 752 MB (stopped at a limit of 700 MB, trial engine), `saguaro-cactus` **466 MB peak** (done, 252 cells ok, 5,292 s wall on the loaded host, 2,171 processor s) |
| `📊️matrix.ts --assets <tree, document 1.5 to 6 MB>` | **1600 MB** | `olive-tree` (3 MB) 1,204 MB before the landing, 815 MB at entry 7 of 12 on the landed engine when it was stopped at a limit |
| `📊️matrix.ts --assets <tree, document above 6 MB>` | 2500 MB until measured | `small-leaved-lime` (23 MB) 1,579 MB at entry 10 of 12 before the landing; `weeping-willow` 1,231 MB |

Seconds on the landed engine (median, longest): people 353 s, `sleeping-person` 2,648 s, `person-pushing-bicycle` 2,178 s; trees and shrubs 361 s, `banana-plant` 1,855 s; machinery `excavator` 902 s; furniture 27 s, 114 s; components 20 s, 84 s; appliances 9 s, 72 s. Give a `timeout` of at least 3600 s for a person or tree and 600 s for a rigid asset; one request of 2500 MB or more waits behind every smaller one, so ask for the figure of its class.

### Vitest with a name filter (MM, 2026-11-25 20:20; priority figures, replace the 2500 MB of the queue rule)

Source: the `/usr/bin/time -l` lines of the fixers' own suite logs in `U/🗑️generated/matrix/*/` (`MX-2/vt/t-*.log`, `VOL/proof/vt-*.log`). `time -l` reports the largest process, the vitest worker; the process tree adds the main process (+8 % measured on `🔭️multiview-coverage`: worker 2,597 MB, tree 2,802 MB). Reserve = worker peak x 1.1 (tree) x 1.3, rounded up.

| Suite with a name filter | Worker peak measured | Reserve |
| --- | --- | --- |
| `🧊️catalog-silhouette` one asset (9 to 27 tests, 75 to 158 s) | 884 to 1,108 MB | **1600 MB** |
| `🧊️three-views` one asset (18 to 54 tests) | 1,544 to 1,719 MB | **2500 MB** |
| `🫁️breathing-catalog` one asset | 1,345 MB | **1950 MB** |
| `⚡️asset-power` one asset (the suite file alone loads 1,337 MB) | 1,337 MB | **1900 MB** |
| `asset-validation` one asset | not measured (assumed like `⚡️asset-power`) | 1900 MB |
| `🌡️catalog-physics` one asset (54 tests, 5 s) | 442 MB | **700 MB** |
| `🔭️multiview-coverage` | 2,597 MB (tree 2,802 MB) | 3600 MB (unchanged) |

## Kill rule (added 2026-11-25 21:10, binding)

At 21:03 queued requests of at least MH-5, MH-6 and MV-2 were sent SIGTERM from outside (`capacity.wait-cancelled`). Never use `pkill`/`killall` or a pattern such as `matrix/`, `capacity run` or `bun`. Stop only PIDs you started yourself or whose full command line contains your exact batch folder `🧰️matrix/<BATCH>/` or `🗑️generated/matrix/<BATCH>/` (with the trailing slash, so `MH-1` never matches `MH-1x`). Check the list before you send the signal.
