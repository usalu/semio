# Implementation S1: the host warden (`🛡️warden`)

Date 2026-11-24. Scope: design §2. Everything lives in `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/` (called `W/` below). Nothing was committed, no process of the release build was touched, the warden loop never ran in enforce mode against this machine.

## Files created

| path (below `W/`) | content |
|---|---|
| `🧬️schema/🔣️.json` | `Policy`, `Kinds`, `Roots`, `Never`, `Platforms`, `Table`, `Mode`, `Level`, `Pressure`, `Host`, `ProcessRow`, `Subject`, `Previous`, `Action`, `Decision`, `Alarm`, `Cancel`, `Lock`, `Protect`, `Suspended`, `KindTotal`, `State`, `Messages` (en/de) |
| `🟦️.ts` | pure part: parser, classification, decision, signal plan, alarm wording |
| `🔁️loop/🟦️.ts` | effectful loop with ports, `ensure`, readers, lock, state/alarm/cancel files |
| `⌨️main/🟦️.ts` | entry (`--observe`, `--directory <host capacity dir>`; mode file otherwise; default enforce) |
| `🧪️tests/🟦️.ts` | 16 tests (fixtures vs oracle, parser, plans, alarms, third-party schema check, 4 mounted tests with real children, ensure, default ports) |
| `🧪️tests/🐍️.py` | independent Python oracle (`--decide`, `--classify`, `--write`, `--check`), stdlib only, reads policy and tables from the schema as data |
| `🧪️tests/🧫️fixtures/🔣️.json` | 38 decision cases, 15 classification cases (inputs hand-authored; every `expect` written by the oracle), real macOS and hand-written Linux `ps` text |
| `🧪️tests/⏱️overhead/🟦️.ts` | 60-tick observe measurement on the real machine (private dir, signal port throws) |

## Files edited (compressed megabytes; ripple was 4 files, so it was done)

- `📡️sampling/🟦️.ts`: `reduceHostSampleV1` adds `compressedMegabytes` (from `counters.compressedBytes`) only when the counter is known, so Linux and Windows samples keep their old shape.
- `📏️decision/🟦️.ts`: `HostSampleV1.compressedMegabytes?: number|null`.
- `🧬️schema/🔣️.json` (host-capacity): `Host.properties.compressedMegabytes` (integer|null).
- `🧪️tests/📡️sampling/🧫️fixtures/🔣️.json`: the three `reduce` cases with a compressor counter got `compressedMegabytes` 4096 / 6144 / 10240 in their `expect`.
- Verified: `repo test-file` on the sampling test (12 pass) and on the host-capacity test (28 pass) after the edit.

## Exported API (signatures)

Pure (`W/🟦️.ts`): `HOST_WARDEN_POLICY_V1`, `HOST_WARDEN_TABLES_V1`, `HOST_WARDEN_PLATFORMS_V1`, `HOST_WARDEN_TABLE_V1`;
`requireHostWardenV1<T>(definition, value): T`, `isHostWardenV1(definition, value): boolean`;
`parseWardenElapsedV1(text): number`; `parseWardenProcessTableV1(text): readonly HostWardenRowV1[]` (row: `pid,parent,group,rssKilobytes,stopped,elapsedSeconds,start,command`);
`wardenSubjectIdV1(pid, start): string` (`<pid>.<sha256(start) 8 hex>`); `wardenOwnerLabelV1(command, prefixes?): string`;
`classifyWardenSubjectsV1(rows, tables?, options?: {protect?, within?, prefixes?, nowMs?, suspendedSince?: Map<id,sinceMs>}): readonly HostWardenSubjectV1[]`;
`planWardenSignalsV1(rows, {pid, scope?}, tables?, exclude?: Set<number>): readonly number[]` (parent first);
`decideHostWardenV1(policy, host, subjects, previous): HostWardenDecisionV1`;
`formatHostWardenAlarmV1(language, alarm): string`, `buildHostWardenAlarmV1({atMs, observed, level, action, subject, host, decision}): HostWardenAlarmV1`, `formatHostWardenNoticeV1(language, name, values?)`, `formatHostWardenStopReasonV1(language, reason, values?)`.
Types: `HostWardenPolicyV1, HostWardenKindV1, HostWardenTablesV1, HostWardenHostV1, HostWardenRowV1, HostWardenSubjectV1, HostWardenPreviousV1, HostWardenActionV1, HostWardenDecisionV1, HostWardenAlarmV1, HostWardenCancelV1, HostWardenStateV1`.

Loop (`W/🔁️loop/🟦️.ts`):
`runHostWardenLoopV1(options: {ports, policy?, tables?, within?, prefixes?, maxTicks?, signal?, code?, platform?, sources?}): Promise<{reason: "idle"|"code"|"signal"|"locked"|"limit"; ticks; counters}>`;
`HostWardenPortsV1 = {sample(): Promise<Host>, table(): Promise<rows>, signal(pid, name), now(), write(line), directory: string, sleep(ms, signal?), mode()}`;
`defaultHostWardenPortsV1({hostPolicy, directory?, observe?, platform?})`;
`ensureHostWardenV1({directory?, code?, nowMs?, spawn?, observe?, main?}): {started, reason: "running"|"replacing"|"spawned"|"failed"}` (never throws);
`readHostWardenStateV1(directory?, nowMs?): HostWardenStateV1|null` (validated; null when absent or older than `staleMilliseconds` 30000);
`readHostWardenCancelV1(pid, directory?, nowMs?): HostWardenCancelV1|null` (younger than 3 600 000 ms);
`hostWardenCodeDigestV1(): string` (16 hex over the four warden sources), `hostWardenDirectoryV1(hostCapacityDirectory?)`, `wardenProcessStartV1(pid, platform?)`.
`directory` arguments of the readers and of `ensure` are the host-capacity directory (the warden adds `warden/`); the `directory` port of the loop is the warden directory itself.

## Pattern tables (all in the schema; a test proves every `source` file exists and every emoji fragment is a substring of it)

Kinds, first match wins (rank, actions, cancel signal, scope `tree` everywhere):

| kind | rank | pattern (over the command line) | source (repo-relative below `🧰️framework/`) |
|---|---|---|---|
| remote-unit | 10 | first token basename `rustc`, then `/remote-execution/work/` in the arguments | `🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/👷️worker/🟦️.ts`: the worker builds `work = <directory>/work/<task>`, `root = work/r`, expands the root into the arguments and runs the compiler (macOS: `sandbox-exec -p <profile> <rustc> args`, which replaces itself by rustc) |
| local-unit | 20 | first token basename `rustc` | `🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts` (the Bun wrapper has `rustc` as an argument, not as first token, so only the compiler matches) |
| pool-child | 25 | `🌐️owner-pool/🧒️child/` | `…/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🧒️child/🟨️.mjs`, spawned by `🏃️runner/🟨️.mjs` line 82 as `[process.execPath, childEntry, file]`, detached; actions suspend+cancel, SIGKILL |
| farm-worker | 30 | `🏊️project-farm/👷️worker/` | `…/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/👷️worker/🟨️.mjs` (spawned by `🏊️pool/🟨️.mjs` line 309) |
| warm-deriver | 35 | `🧮️derive/♨️warm/⌨️main/` | `…/🧭️invocation/🕸️graph/🧠️memo/🧮️derive/♨️warm/⌨️main/🟦️.ts`; spawned detached with `process.execPath`, i.e. the pinned Bun, which is itself a root, so the root rule finds it although its parent is pid 1 |
| heavy-question | 40 | `🪆️heavy-questions/📜️script.ts` | `…/🕸️graph/📥️inference/🧭️script-inputs/🪆️heavy-questions/📜️script.ts` |
| test | 50 | first token basename `bun`, second token `test` | `🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts` |
| slice-worker | 60 | `👷️slice-pool/👷️worker.ts` | `…/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts` |
| stage | 90 | `🚦️ready/🏃️wrapper/🟦️.ts`; actions: cancel only, SIGTERM | `…/🌐️remote-execution/🛰️sidecar/🚦️ready/🏃️wrapper/🟦️.ts` |

Roots: `^\S*/\.cache/b/t/m/bun/`, `^\S*/semio/runtimes/bun/`, `^(?:\S*/)?semio-build-scheduler(?:\.exe)?(?:\s|$)`.
Never: `🛡️warden/` (own entry and tests), `🛰️sidecar/(?!🚦️ready/🏃️wrapper/)` (launcher and every role: cache server, agent, scheduler, graph, pool puller, Ready supervisor; verified on the live table: `…/🛰️sidecar/🟦️.ts cache|agent|scheduler|graph|pool` and `…/🛰️sidecar/🕸️graph/🏃️runner/🟦️.ts`), `🌐️remote-execution/(?:🏃️entry|🖥️server|🗓️scheduler)/`, `🔁️replica/` (supervisor `🔁️replica/🛟️supervisor/🟦️.ts`), `/semio/fleet/entry/` (fleet boot).

### How the stage root is identified

By pattern, no `stages.json` needed. `readyStage...` in `🚦️ready/🟦️.ts` line 1110 spawns `process.execPath <…/🚦️ready/🏃️wrapper/🟦️.ts> [--launch-record=…] --deadline=… <exitFile> <command> args…` detached. The wrapper's command line therefore always contains `🚦️ready/🏃️wrapper/🟦️.ts`, it runs on the pinned Bun (root), and it is the one entry under `🛰️sidecar/` that the Never lookahead lets through. The wrapper forwards SIGTERM to the real command and writes the exit file, so SIGTERM to the tree is the clean cancel; its children (slice worker, farm workers, rustc) are subjects of their own kinds, and the stage's megabytes are the memory of everything below it that is no subject. More reliable than a pid file: it needs no supervisor cooperation and cannot go stale.

### Pool child: what happens to its claimed owners (read from `🌐️owner-pool/🏃️runner/🟨️.mjs` and `🟦️.ts`, not run)

A warden SIGKILL is an external kill: `child.killed` stays false, `proc.exited` calls `onExit`, which records the exit, sets `lastError` and, when the child lived under 10 s, a 30 s back-off for the wave; the supervisor loop then starts a replacement child. `abandon()` of a claim runs only in the in-process failure path, so the claims of the killed child are not released: they run out after their declared lifetime (the expected run time of the owner) and the owner is then claimed again by a replacement child or by another machine's puller. A suspended child keeps its claim (heartbeat stops, the lifetime runs on); `suspendMaximumSeconds` 120 bounds that. The pool puller parent (`🛰️sidecar/🏊️pool/`) is never a subject.

## Behaviour notes

- Decision: pure, 4 arguments exactly as in the prompt; oracle and TypeScript agree on all 38 cases; the stored `expect` is the oracle's.
- Subject megabytes = resident of the process plus every process whose nearest subject it is (grandchildren below a non-subject count for the nearest subject above). Resident memory only (no `footprint` call): the macOS compressor hides idle memory (a 40 MB test hog showed 17 MB), so idle subjects are under-priced.
- Tick: one cheap `ps` (pid, ppid, pgid, rss, state, etime, lstart) in parallel with `vm_stat` and `sysctl` (no `hdiutil`, no file-table call). The expensive `args=` column costs about three times the rest of the table, so command lines are read through `ps -ww -o pid=,args= -p <new pids>` only for processes not yet cached (key pid + start text); a process younger than 3 s or still printed `(name)` is not cached (it may not have executed yet; seen in the test as `(sleep)` then `sleep 5`). The full eight-column form stays in the schema as `Table.arguments` and is what the parser documents and the tests use.
- Order of effects: suspend writes `state.json` before SIGSTOP; cancel writes `cancelled/<pid>.<start digest>.json` before SIGCONT + the kind signal; resume signals first and writes after (a crash between must not lose a stopped process); alarm append, then both languages on stderr. Observe mode (and Windows) signals nothing, writes no cancel record and announces each decision once, not every tick.
- A process stopped by somebody else is treated as suspended: first sight starts its clock, after 120 s it is cancelled (not calm) or, when it cannot be cancelled, resumed; at calm it can be resumed by the hysteresis.
- Exit resumes everything this run suspended (idle, code change, signal, limit); start resumes whatever the previous `state.json` left suspended (pid + start text verified against the table first).
- Lock: `lock.json` created with `wx`; stale when the pid is dead or its start text differs; a second loop in the same process refuses it (tested).
- `ensureHostWardenV1`: one read of `state.json` plus `kill(pid, 0)`; no spawn while a holder with a state younger than 30 s is alive; code differing → `replacing` (the holder ends itself within 60 s); otherwise detached spawn with the output in `warden.log` (rotated to `.1` at 4 MiB).

## Tests run (`repo test-file <file>` only)

| file | result |
|---|---|
| `W/🧪️tests/🟦️.ts` | 16 pass, 0 fail, 1560 expect() calls, 22.6 s |
| `W/🧪️tests/⏱️overhead/🟦️.ts` | 1 pass, 0 fail |
| host-capacity `🧪️tests/📡️sampling/🟦️.ts` (after the Host edit) | 12 pass, 0 fail |
| host-capacity `🧪️tests/🟦️.ts` | 28 pass, 0 fail, 4608 expect() calls |
| `python3 W/🧪️tests/🐍️.py --check` | fresh |
| type check (`tsc --noEmit` through a scratch tsconfig that extends the root one and lists the 5 TypeScript files, 14 s) | no errors; the five warden files were in the program (`--listFiles`) |

The third-party check ran (Python `jsonschema` 4.25.1 found on the system Python, not the pinned test Python): 100+ records of the fixtures validated by both validators with the same verdicts, plus four broken records refused by both. The mounted tests ran real `bun -e` children (marker in the command line, `within: process.pid`, injected tables, a signal port that refuses any pid but the spawned children): guard stops the cheapest (`ps` state `T`), the third calm tick resumes it, critical pressure kills it (SIGKILL; cancel record, alarm lines EN+DE, `state.json` validates), the next is suspended and resumed on exit, a second loop gets `locked`, a dead predecessor's suspended child is resumed and its stale lock taken over, observe mode signals nothing, a protected path excludes everything, idle/code/signal exits work.

## Overhead measured (this machine, 32 GB macOS, loaded by other agents; 60 observe ticks, 2 runs)

Last run: tick p50 105 ms, p95 349 ms; whole process tree 59.8 CPU ms per tick (loop plus the `ps`, `vm_stat`, `sysctl` it starts; from the difference of a 65-tick and a 5-tick run under `time -p`) = 3.0 % of one core at the calm interval (2000 ms), 12.0 % at the tight interval (500 ms); resident 77 MB (51 MB in the plain entry run). The run before the last: p50 87 ms, p95 184 ms, 52 ms per tick (2.6 % / 10.5 %). The first version with the `args=` column every tick measured p50 155 ms, p95 395 ms, 97 ms per tick. Remaining cost: about 20 ms CPU of `ps`, the rest spawns of `vm_stat` and two `sysctl` and the JavaScript. Not measured: Linux.

## Not done, not verified, decisions to take

- Callers are not wired: `ensureHostWardenV1` is not called from `withHostCapacityV1`, the native-build broker, the remote-execution worker or the Ready supervisor; the Ready supervisor mirror and `repo fleet status` columns of design §2.4 are not built.
- The release build under `/Users/ueli/.sinc/play` runs its own pinned Bun under `.cache/b/t/m/bun/`, which is a Root. Before the warden runs in enforce mode on the coordinator, write `<host-capacity dir>/warden/protect.json` = `["/Users/ueli/.sinc/play"]` (and `mode` = `observe` while the release builds). The entry reads both every tick. I did not write either file anywhere real.
- The real-machine runs were: one 9 s observe run of the entry with a private `mktemp -d` directory (5 ticks, state valid, two live pool children classified, stopped by my SIGTERM, lock released), and the overhead runs. No enforce run.
- Linux: the parser is tested on a hand-written sample only; `ps` needs a UTF-8 locale to print the emoji paths (`C.UTF-8` is set in the schema; a host without it prints `?` and no emoji pattern matches). Not run on Linux, in a devcontainer or in a cgroup. `state=` is a procps alias that I did not run.
- Windows: no SIGSTOP and no `ps`: the schema declares it observe-only (`Platforms.observeOnly`); the table port maps `readHostProcessRowsV1` rows (name only, no start text, no state), so only name patterns (`semio-build-scheduler`, `rustc`) can match and nothing is ever signalled. Not run on Windows.
- Patterns use `/` only (unix command lines). The `heavy-question`, `farm-worker`, `slice-worker`, `warm-deriver` and `stage` patterns were matched against source paths and the command lines of the spawn sites, not against a live process of that kind (none ran during the session; live kinds seen: pool-child, local-unit wrapper rows).
- `main`, the overhead test and the test import the authored policy from the product (`🛍️products/…/🏗️build/🔣️policy.json`), the same way the product's `🎛️control` module does; the framework loop itself takes the host policy as an argument.
- Counters in `state.json` are per process run, not persisted across warden restarts.
