# R183: the `tools` stage of g1-m05 waits behind a background graph read (living report)

Scripts: `T/🧰️scripts/R183/` (T = `☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`): `patch.py` + `patch2.py` (anchored edits), `stage.sh` (mirror + map), `preload.ts`, `tests.sh`, `battery.sh`, `land.sh` (+ `land.tsv` in scratch `R183`).

## 1. Who starts `show projects`, and why it holds the store

* Only one code path in the build tooling starts `repo show projects` by itself: the graph server of the machine (`🛰️sidecar/🕸️graph/🟦️.ts`, role `graph` of the sidecar; runner `🏃️runner`). Its loop (`🪪️graph-state/🙋️wanted/🛠️server`) starts a **refresh** read when headroom is 9400 MB, pressure normal, the worktree quiet and a fleet claim is granted (every 600 s at most), and a **request** read for requests of other machines. m05 has 64 GB (serve minimum 24000 MB), so it serves and refreshes. Nothing in the readiness loop, the pool or a warm-up starts one.
* On g1-m05 the holder pid 30880 is `not-ours` in the containment log (no ready owner token), cwd `C:\semio\.cache\b\c`, started 03:27:06, holding since 03:28:01. I cannot read the host log of m05, so that the graph server started it is established by elimination (no person window: `person none`; no other starter in the code), not by its log.
* Same pattern on this Mac (read-only evidence): `.cache/f/graph-server/read-record.json` reason `refresh`, running 5 h 13 min; lease registry entry of the read (pid 42303): `standing.priority: background`, phase inference 1926 of 4775, holds `native-fixed-image:` of the images `b/l`, `b/o`, `b/d`, `b/g` all **shared since the start of the read** (18,800 s) plus exclusive `graph-custody`. So a graph read holds the dependency store (and the others) SHARED for its whole life, hours.
* Why that blocks the stage: `repo tools setup all` runs `setupRepositoryDependencyPublicationV1` -> `observeRepositoryDependencyBootstrapFundingV1` -> `withFundingParentV1`, which needs the parent exclusively (`wait:false`, retried by `retryWhileDependencyStoreBusyV1` with the heartbeat line you see). One shared reader of 5 h keeps every exclusive taker out. The holder line does not print the mode; shared is inferred from the Mac registry, not read on m05.
* Not changed: the read still holds the store shared for its life (R173's split of duties is staged, not landed; it narrows what a reader walks, not how long it holds).

## 2. Stall rule versus a legitimate wait (built)

Why every try ended: the stage prints the same holder line with only seconds moving; the stall tracker rightly calls it no progress, 600 s later `readyStalledV1` stops the tree (`stalled`), or a landing changed the code (`code-changed`). Both restart the stage and lose its queue position.

New, in `🚦️ready/🌲️process-tree/🟦️.ts` (pure) and `🚦️ready/🟦️.ts` (runner):
* `readyHolderWaitOfV1` parses the newest wait line (English and German, silent variant). `readyHolderCustodyV1` judges it from the process table, the registry entry and the clock: `progressing` (alive with the registered start, entry renewed within `holderSilentMilliseconds` 300000, bound not spent), `silent`, `gone` (no process, or pid reused), `bound` (stated limit spent), `unknown`.
* The runner checks every 10 s (`holderCheckMilliseconds`): `progressing` and `gone` count as progress (no stall stop); `silent` is not progress, the stall rule ends the try and the result carries `holderSilent`, shown as the typed bilingual refusal `stage-holder-silent` and reason `holder-silent` (names pid, command, silent seconds); `bound` and `unknown` keep the old rule. The hard cap of the stage (4 x soft cap) still bounds every wait. The code-change restart is kept deliberately (a landing may be the fix).
* Pre-emption: after `holderYieldMilliseconds` (120 s, scaled by machine speed) behind a holder, the runner reads `.cache/f/graph-server/read-record.json`; if the record is a running read (no status) whose runner has the recorded start and the holder is the runner or below it (`readyBackgroundReaderV1`, kill plan from the table), it stops that tree (SIGTERM, then SIGKILL after the grace; `taskkill /T /F` per pid on Windows) with kill reason `yield`, said in English and German in the log and in `kills.log`. A holder that is not below the recorded runner (person, another agent, another checkout) is never touched. The graph server finds its read ended, the read is repeated later from the waves it published.
* Graph server gate: while `ready.json` of the machine says not ready in state running, waiting or backoff (fresh within 10 min), the graph server neither serves requests nor refreshes (`mayServe`, `mayRefresh` in `🕸️graph/🟦️.ts`). Group server (`server`) and a person's observation are not gated.

## 3. Windows specifics

* Liveness of a holder is `process.kill(pid,0)` (libuv `OpenProcess` + exit code, so an exited process with an open handle is gone) plus the registered start against the CIM creation time (`Get-Process ... StartTime`, 15 s timeout); a pid whose start differs by more than 5 s is a reused pid. When the start cannot be read (loaded host), the entry counts as gone after 300 s without renewal (R99 heartbeat rule); a holder with a known matching start stays whatever its heartbeat. A dead holder's registry entry is removed and named by the waiter itself (`observeDependencyStoreHoldersV1`), the OS frees its file locks: no human needed. Proven with Windows-shaped fakes (law below).
* The runner's own pre-emption uses the process table of `readyReadTableV1` (CIM rows with creation times on Windows).

## 4. Laws (red first)

* `🌲️process-tree/🧪️tests/🤝️holder-custody` (6): authored cases for parse, custody, reader plan, foreground; independent Python oracle (own regular expressions, fixpoint plan) gives the same answers; sliding judgement; tracker unchanged; German lines; Windows-shaped dead/reused/unknown-start holders through the real `observeDependencyStoreHoldersV1`.
* `🧪️tests/🤝️holder-custody` (6, real processes, scratch state root): live renewing holder is no stall (stage runs 9 s past a 2.5 s stall window); silent holder is stopped by the rule and named; gone pid is no stall; a background read below a recorded runner gives way (both languages, kill record) and the same holder without record is left alone; ready.json gate; source shape of the graph server.
* Red on the old files: process-tree law does not load (exports missing); runner laws 5 of 6 fail on old behaviour (old rule stalls the live-holder stage, holder never named, read never stops).

## 5. Landed and what g1-m05 did next

LANDED 2026-11-18 05:00:00 to 05:01:58 UTC (entry in landings.md), green under the lock (9 files), no restore. Staged battery before: 15 files green (the only red was the pinned scaling law, extended by the two new keys; one load flake of `🔭️read`, green twice). No bun cache entry names the scratch path (cache disabled; I also removed `~/Library/Caches/bun/@t@` once by mistake at the start, a regenerable cache).

g1-m05 at 05:07 UTC (read-only): the holder 30880 ended before the new code ran: `ready-log-tools.txt` says verbatim "Process 30880 (741d3068... show projects) held the dependency store since 2026-11-18T03:28:01.557Z but no longer exists; its claim was removed". The stage `tools` (pid 33620, started 04:54, reason `adopted`, attempts 13) now waits, verbatim: "Process 36528 (00945ece1f0c86f1ae8c1be7943b22149d6b578ee93573a4c3062833694b4eea run @semio-tech/fem-3d-rs:check --output=stream --parallel=8) holds the dependency store since 2026-11-18T03:55:14.747Z; waiting for it (736 s of at most 21600 s)". That is a real build task (not a graph read; `not-ours`, started 03:54:07), so the yield never touches it; with the new custody it is no stall while it renews its entry. Whether it finishes, and the stage after it, is the next observation.

## 6. NOT verified
* Never run on Windows: real pre-emption (taskkill path), CIM start times, `show projects` registry `standing` on m05; the starter on m05 is inferred, not read from its host log.
* The old sidecars run old code until they restart (the ready role restarts at once; the graph role waits up to an hour while busy); the stage in flight (pid 33620) was adopted by the old runner logic until the ready role restarted.
* Holder progress is judged by heartbeat only (no CPU/work counter on Windows): a live, renewing but useless holder waits until the stated bound (21600 s) or the stage hard cap.
* Option not built: identify background holders by `standing.priority: background` in the registry entry (present on this Mac) instead of the graph server's read record.
* Leaked test processes on this Mac (`ready-tools-*/grandchild.js ... show projects`, 13 h old) belong to peers' tests; not touched.

