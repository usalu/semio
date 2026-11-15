# 🔎 Audit: g1-m06 Package Error, native_rc=7, b72e2264, g1-m10, g1-m09

Read-only audit. No git command, repo/graph/build command or network call. Only this file was written. Paths below are relative to the ticket folder `WORKERS-READY-WITH-LOW-RESOURCES/` unless absolute. "Not in the files" means the data is not there.

## 1. g1-m06: `Cannot find package '@semio-tech/pack-json-decode-ts'`

**Occurrences.** `workers/g1-m06/round-18.md` has 8 hits (lines 27, 46, 66, 86, 106, 126, 146, 166). `workers/g1-m06/round-18.tail.txt` has 7 (lines 22, 31, 40, 52, 66, 83 are "edited by another process" retries "(1 of 6)" to "(6 of 6)", and line 100 is the final error). No other folder or round contains the string. Earlier rounds 1 to 17 of g1-m06 have 0 hits.

**First occurrence, `workers/g1-m06/round-18.md` line 27, with the 20 lines before it (lines 7 to 26):**

Lines 7 to 26 of `round-18.md` (the 20 lines before the first hit), then line 27 is the first hit:

```
7  - Launcher after the restart: link inode 3631708 to `versions/50ff6101d0ecc30c/repo`, identifier `tech.semio.repo`, CDHash `a0f39e389ce8a697c89d7f9357e72d37eb06ab0d`: unchanged.
8  - Runner rules: one SIGINT on a process above 8 GB resident, pressure level 4 in two samples in a row, no new graph progress line for 15 minutes, dependency store wait above 5 minutes; level 2 is recorded only. Slice worker sampler of `ROUND-NOTE-P12.md` section 1 runs with every try.
9  (blank)
10 ## Step 0 (`repo tools setup all`, Once)
11 (blank)
12 Run from the worktree root with `--lang en`. Start 04:04:15Z, end 04:05:46Z, wall time 91 s, exit code 75. No "Another process holds the dependency store" line. I did not run it a second time, as the round says once. Last 15 lines (without `[script]` lines):
13 (blank)
14 ```
15 [repo] still working after 50 s; last phase started: source.capture
16 [repo] still working after 5 s; last phase started: source.capture
17 [repo] still working after 50 s; last phase started: source.capture
18 [repo] still working after 10 s; last phase started: source.capture
19 [repo] still working after 50 s; last phase started: source.capture
20 [repo] still working after 20 s; last phase started: source.capture
21 [repo] still working after 80 s; last phase started: source.capture
22 [repo] still working after 80 s; last phase started: source.capture
23 [capacity] keeping the command grant (1 job(s), 1404 MB reserved) for the command that replaces this one: it does not queue again
24 [capacity] command: sized at 69% of the default from 1 earlier run(s) of its class
25 [capacity] took over the command grant of the command this one replaces (1 job(s), 1404 MB reserved): no queue
26 [capacity] command: sized at 69% of the default from 1 earlier run(s) of its class
27 Cannot find package '@semio-tech/pack-json-decode-ts' imported from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure
28 The stamps moved again after the restart: run the command again.
29 tools_rc=75
30 ```
```

**Stack.** Not in the files. The error is one line. No `at ...` frames and no `node:internal` line exist in `round-18.md` or `round-18.tail.txt`.

**Last occurrence, `workers/g1-m06/round-18.md` line 166 (Try 7, end 04:34:36Z), with the 20 lines before it (lines 146 to 165):**

```
146 Cannot find package '@semio-tech/pack-json-decode-ts' imported from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure
147 The stamps moved again after the restart: run the command again.
148 end 2026-11-15T04:30:07Z
149 native_rc=75
150 ```
151 (blank)
152 ## Try 7
153 (blank)
154 - start 2026-11-15T04:33:15Z, end 2026-11-15T04:34:36Z, `native_rc=75`, hard stop: none.
155 - Pressure levels of the 30 s samples (count, level):  3 pressure_level=1;. Peak resident of any process: 1466752 KB.
156 - Slice worker sampler (5 s), row with the largest resident size: ``; closure entries before: 0; closure entries after: 0.
157 - Launcher after the try: inode 3631707 mtime 2026-11-14T22:55:40+0100 /Users/ueli/.local/share/semio/launcher/bin/repo Identifier=tech.semio.repo CDHash=a0f39e389ce8a697c89d7f9357e72d37eb06ab0d ; marker absent.
158 - Setup tasks: succeeded 0, failed 0 (none).
159 (blank)
160 ```
161 [capacity] run: sized at 5% of the default from 15 earlier run(s) of its class
[capacity] admitted run with 4 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 10% of the default from 15 earlier run(s) of its class
[capacity] keeping the run grant (4 job(s), 839 MB reserved) for the command that replaces this one: it does not queue again
[capacity] took over the run grant of the command this one replaces (4 job(s), 839 MB reserved): no queue
Cannot find package '@semio-tech/pack-json-decode-ts' imported from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure
The stamps moved again after the restart: run the command again.
end 2026-11-15T04:34:36Z
native_rc=75
```

**Which command and file imported it.** The importing file is `🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure/🟦️.ts`, line 1: `import { decodeJsonSyntax, JsonSyntaxError, ... } from "@semio-tech/pack-json-decode-ts";`. The run is the repo build command. `round-18.tail.txt` line 19 to 21 shows `[script] {"owner":"repository:build","command":"repo",...}` and `build:source-admission · Admitting the sources of the build`. Step 0 is `repo tools setup all` (`round-18.md` line 10). The tries are `native-bootstrap.sh`, which runs the repo bootstrap `🐚️.sh setup`.

**Live tree: where the package lives.**
- Package manifest: `🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️decode/📦️packages/🟦️typescript/package.json`, `"name": "@semio-tech/pack-json-decode-ts"`, `"exports": {".": "./🟦️.ts"}`.
- Root `package.json` line 31 lists that directory in `"workspaces"`.
- `bun.lock` line 6796 to 6797: `"@semio-tech/pack-json-decode-ts@workspace:🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️decode/📦️packages/🟦️typescript"`.
- Root link (verified): `/Users/Shared/semio/node_modules/@semio-tech/pack-json-decode-ts` is a symlink (Nov 13 22:03) to the package directory above. `ls -L .../🟦️.ts` resolves. The root `node_modules` is itself a symlink to `.cache/b/d/m/j/86415d695aa883fffc22/node_modules`, which contains `pack-json-decode-ts`.
- Dependents: `🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure/📦️packages/🟦️typescript/package.json` line 13 `"@semio-tech/pack-json-decode-ts": "workspace:*"`. Its own `node_modules` symlink points to `.cache/.../w/87/node_modules`. The closure directory itself has no `package.json`. The closure's `🟦️.ts` is imported by the schema source, so resolution is by bun workspace link via the root `node_modules`, with no tsconfig `paths` alias (`tsconfig.json` has none for `@semio-tech`).
- Importers under `🧰️framework/🛍️products/🦑️repo`: only `🔨️modules/📚️library/🔣️taxonomy.json` mentions the name. No `.ts` source under that tree imports it directly.

**Evidence on g1-m06 of a missing link or an incomplete `deps-js-all`.** Not in the files. The message on m06 says `A source file was edited by another process while this command loaded it` for six retries (`round-18.tail.txt` lines 22 to 83), then `The stamps moved again after the restart: run the command again.`. These lines point to replica sync moving files during load, not to a missing link. The m06 folder has no `deps-js-all` line (0 hits in all m06 files). The m06 replica root `/Users/ueli/Documents/semio/node_modules` link is not in the files, so I cannot check it. The `deps-js-all` mentions in `census-worker-blockers.md` row 13 (ENOTDIR on `CLAUDE.md/package.json`) and `result-N5.md` line 29 are from other machines or older rounds.

**Count note.** The census says "six runs". The files show six retries inside one try (`round-18.tail.txt` 1 of 6 to 6 of 6), and 8 error blocks in `round-18.md` (step 0 plus try 1 to 7).

## 2. g1-m02: `native_rc=7`, three times

**Where the value comes from.** It is written by the worker's own script, not by the repo.
- `native-bootstrap.sh` (in the ticket root) runs `bash "$worktree/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh" setup`, sets `status=${PIPESTATUS[0]}`, and writes `echo "native_rc=$status"` to the log. So 7 is the exit status of the bootstrap `setup` process.
- The runner (`workers/g1-m02/run-round-13.py` line 83 to 84, `run-round-14.py` line 89, `run-round-20.py` line 86) only parses `native_rc=(\d+)` from the log. It never produces 7.
- The repo bootstrap `🐚️.sh` contains no `exit 7` (grep of the bootstrap directory: no hits). Its explicit exits are 0, 1, 2 and `exit 75` is not in it. So 7 is not set by a literal `exit 7` in the bootstrap. It must come from a child process whose status the script propagates. Which child is not in the files.

**The three occurrences and how they ended.**
- `workers/g1-m02/round-13.tail.txt` line 200: `native_rc=7`, preceded by `end 2026-11-15T02:06:43Z`. The runner's stop for this try is `stop=no new progress line for 905 s` (`round-14.samples.txt` and `round-13.md` line 16: `my SIGINT`).
- `workers/g1-m02/round-14.tail.txt` line 200: `native_rc=7`, preceded by `end 2026-11-15T03:20:29Z`. `round-14.samples.txt` line 718: `TRY 4 END native_rc=7 ... stop=no new progress line for 908 s`.
- `round-14.samples.txt` line 128: `2026-11-15T02:21:47Z TRY 1 END native_rc=7 ... stop=round 15 was published and no task has started`. `round-14.md` line 32: `Try 1 was interrupted against round 15's instruction ... sent it at 02:21:12Z`.

**Signal and repo message.** All three were SIGINT from the runner (`os.killpg(child.pid, signal.SIGINT)`). The repo message `Internal error: Original Script process interrupted` is not in the g1-m02 round 13 or 14 tails. The same message appears on g1-m03 `round-13.tail.txt` line 198, followed by `end 2026-11-15T02:16:32Z` and `native_rc=1`. So the repo message exists, but for m02 the status 7 is not explained by any file.

**Answer.** The status 7 is produced by the bootstrap `setup` process after the runner's SIGINT. The runner and the worker script only pass it through. The explicit mapping from SIGINT to 7 is not in the files.

## 3. Reference `b72e2264` on g1-m01, round 17 try 1

- Only one hit in `workers/g1-m01`: `round-17.samples.txt` line 306:
  ```
  04:15:36Z TRY 1 end: native_rc=1; hard stop: none; failed tasks: none; reference: b72e2264
  ```
- Context, `round-17.samples.txt` lines 300 to 306: the try's `run peak rss=0 KiB` at 04:15:36Z is the last process sample; the previous lines are `04:15:31Z pressure=1` and bun/`git` process rows. Line 306 is the try-end line.
- The writer is `workers/g1-m01/run-round-17.sh` line 109: `reference=$(grep -aoE "reference [0-9a-f]{8}" "$log" | tail -1 | cut -d" " -f2)`, and line 110 writes the try-end line. So `b72e2264` was taken from the native bootstrap log `~/native-bootstrap-round-17-try-1.log`, which is not in the worker folder.
- `request-failed b72e2264`: 0 hits in the ticket folder. No service log line exists in the files.
- The bootstrap message that contains `reference b72e2264`: not in the files.
- Round 17 try 2 (`round-17.samples.txt` line 393) has `reference: none`.

## 4. g1-m10: what it wrote, how far it got, what it waits for

**Files written** (`workers/g1-m10/`, 15 entries): `HANDOFF.md`, `round-15.pre-status.json`, `round-15.pre-status.stderr.txt`, `round-16.pre.md`, `round-16.tools-observations.txt`, `round-16.tools-progress.md`, `round-16.tools-setup.attempt2-rc1.metrics.json`, `round-16.tools-setup.last15.txt`, `round-16.tools-stack.txt`, `round-18.agreement-wait.txt`, `run-observed.py`, `run-tools-setup.sh`, `wait-agreement.sh`. There is **no `state.json`** and **no native try** (no `round-1x.*` native try files, and no `round-16.tools-setup.log` or `round-16.tools-setup.metrics.json` for attempt 3).

**How far it got (step 0, `repo tools setup all`, round 16):**
- Attempt 1: ended by external SIGTERM after about one hour. `round-16.tools-progress.md`, update at 2026-11-15T04:06:35Z: `attempt 1 of tools setup ended by external SIGTERM at 05:05 CET`.
- Attempt 2: `round-16.tools-setup.attempt2-rc1.metrics.json` = `{"start":"2026-11-15T04:06:42Z","end":"2026-11-15T04:18:06Z","rc":1,"wall_seconds":684,"attempt":2}`. Final error in `round-16.tools-setup.last15.txt`: `Internal error: Original physical body, drain and cleanup settlement failed: Protected maintenance and accepted drainage failed: ... Current complete Source binding or whole receipt changed; Original process admission closed` then `repository:command failed with exit code 1`.
- Attempt 3: `round-16.tools-progress.md` line 35, 2026-11-15T04:33:23Z: `Starting attempt 3 (attempt 2 files renamed ...attempt2-rc1.*)`. `run-tools-setup.sh` (hard-coded `"attempt":3`) would write `round-16.tools-setup.log` and `round-16.tools-setup.metrics.json`. Neither file exists in the folder, and `ps` showed no running `tools setup` process when I looked. Whether attempt 3 ran is not in the files.

**The agreement wait (`round-18.agreement-wait.txt`, made by `wait-agreement.sh`).** The census (`census-rounds-13-19.md` line 124) says the agreement was not reached. The file now shows it was:
```
04:29:02 m10=97445096cc0a inc=0 srv=cfb539bbd473 inc=0
04:31:02 m10=97445096cc0a inc=0 srv=cfb539bbd473 inc=0
04:32:07 m10=ee7ff4cbf98e inc=0 srv= inc=
04:33:08 m10=ee7ff4cbf98e inc=0 srv=ee7ff4cbf98e inc=0
AGREE
```
The wait is over. `round-16.tools-progress.md` line 35 says `replica code agrees (g1-m10 and g1-server ee7ff4cbf98e..., incoming 0)`. Its last line is the 04:33:23Z update, so the file's mtime 05:33 is later than the last data row.

**Waiting for now:** not in the files. The last written item is the attempt 3 start at 04:33:23Z, with no log after it.

## 5. g1-m09: any trace

- No `workers/g1-m09` folder exists (`ls: workers/g1-m09: No such file or directory`).
- In the ticket folder, `g1-m09` appears only in `audit-cross-network.md` (2 lines). Line 23: `| g1-m09 | not known | no attest.json | never attested | not found | not found |`. Line 28: `g1-m09 has only STORE/origins/g1-m09/fleet-auth.json (mtime 2026-11-14 02:13; createdAtMs 1794618785499). It has never written an attestation (not found). The directory was not present in the last attestation sweep.`
- `/Volumes/semio-store/v1/origins` does not exist on this machine, so the store could not be read here. The audit file's statement comes from an earlier read.
- Not in the files: any round, try, log, or census row for g1-m09. `census-rounds-13-19.md` and `ROUND.md` do not mention it.
