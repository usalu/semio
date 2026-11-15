# g1-m03 Round 9

- Machine: Mac mini M4, 16 GB memory, macOS 27.2.1 (26B131), free disk 87 GiB of 228 GiB before the run. Worktree `/Users/ueli/Documents/semio`.
- Result: **consent** (`native_rc=4` in try 3), round stopped as instructed. Three tries: try 1 and try 2 ended with the fleet refusal (`native_rc=75`), try 3 with `native_rc=4` after 38 s of `workspace:setup`. Marker absent, `ready` false. No hard stop; memory pressure level 1 in every sample. The setup tasks were not reached.
- The launcher was **unchanged** in all three runs (same inode of the link 2817021, same target `versions/cf25f2b751f83336/repo`, same mtime 20:56:38Z, same CDHash `6de7547a8b8ff384bbfbfc45e962462063ff0357`, identifier `tech.semio.repo`); every run printed `[semio] The repo launcher is current, nothing was replaced`. So this denial is **not** caused by a replaced launcher.
- No line "runs under the app's access" (or any line about a direct child or the caller's access) is in the three bootstrap logs or in the graph service log. The graph service of this round ran as the launchd job `tech.semio.repo.service.3f8d4f1eae84` (pid 65997, parent 1; worker pid 66015).

## What happened to the graph service (as observed)

- Started in try 1 at about 22:06Z by launchd (`runs = 1`). It answered try 1 (20 min) and try 2 (26 min): 51 minutes of work in the checkout without a denial.
- Try 3 started at 22:56:55Z. At 22:57:33Z, on the first request of that try, the same service process (pid 66015) got the denial, stopped itself, job `last exit code = 4`, `state = not running`.
- The same pattern as in round 7 (service started 20:10Z, worked 23 minutes, denied at 20:39:33Z on the first request of the next try). In round 7 the launcher had been rebuilt in between; in this round it was not.
- Receipts at 22:58Z: `client.json` `state: ok` (pid 91817, 22:57:30Z), `launcher.json` `state: waiting` (pid 79408, written 22:32:15Z during try 2), `service.json` `state: denied` (pid 66015, 22:57:33Z). The consent dialog for `repo` has, as far as the receipts show, not been answered on this Mac.

## Consent text (try 3, verbatim)

```
macOS denied the resident service access to your Documents folder while it answered a request (reference e07192d7). The service has stopped itself; it runs with the file access consent of the process that started it and not of `repo`. Switch on Documents Folder for `repo` (/Users/ueli/.local/share/semio/launcher/bin/repo) under System Settings, Privacy & Security, Files & Folders (or Full Disk Access), then run the command again, or keep the checkout outside Documents, Desktop and Downloads. Details: /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-9888631afea4.log.
end 2026-11-14T22:57:33Z
native_rc=4
```

Graph service log around the denial (`host-9888631afea4.log`):

```
2159:2026-11-14T22:57:33.166Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
2163:2026-11-14T22:57:33.166Z shutdown access-denied
-- last 12 lines
[script] build:source-admission · Admitting the sources of the build
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
2026-11-14T22:57:33.166Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
    at failure (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148:307)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:153:768)
    at processTicksAndRejections (native:7:39)
2026-11-14T22:57:33.166Z shutdown access-denied
[capacity] the memory footprint of running process trees cannot be read on darwin (footprint printed no process); the resident set is used instead, which can overstate what a process holds
[capacity] Der Speicher-Fußabdruck laufender Prozessbäume kann unter darwin nicht gelesen werden (footprint printed no process); stattdessen gilt der residente Speicher, der überschätzen kann, was ein Prozess hält
repository:command failed with exit code 4
```

Section 3 of `ROUND-NOTE-P6.md` after the exit 4:

```
$ python3 …/P6/tcc-attribution.py --last 30m
0 rows (of 2 consulted requests in the window)
$ launchctl print gui/501/tech.semio.repo.service.3f8d4f1eae84 | grep -E "state|runs|last exit code|program"
	state = not running
	program = /Users/ueli/.local/share/semio/launcher/bin/repo
	runs = 1
	last exit code = 4
		state = active
		state = active
```

## 1. Before the run

    Pre-run facts, 2026-11-14T22:05:48Z
    - replica agent: launchctl print gui/501/tech.semio.fleet.replica:
        	state = running
        	runs = 45
        	pid = 37539
        	last exit code = 0
        		state = active
        		state = active
    - replica code digest (.cache/f/health.json): 590fdef8b75853d124d6f615cd2612d607146e27aaf2b94715a5d7b7606ad2f1, store ok, daemon pid 37556 , health.json age 1 s
    - "bootVersion":6
    - replica agent kickstarted: 0 times (state = running)
    - graph service jobs:
        -	4	tech.semio.repo.service.a909995b451a
        -	0	tech.semio.repo.service.d8505b2379b9
    - `repo service host` processes:
      (0 process(es))
    - last 8 lines of .cache/f/sidecars/build-agent.log:
        2026-11-14T21:58:54.732Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T21:58:56.231Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T21:58:57.529Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T21:58:58.579Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T21:58:59.656Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T21:59:00.862Z worker registered: 4c7b5b5e-1eb8-41f6-92a3-e0616dd21798 cache http://192.168.178.70:6094
        2026-11-14T21:59:02.976Z load report sent: reachable again
        2026-11-14T22:02:46.121Z graph-server: alive, 0 machines seen, 0 requests held, read none
    - memory pressure level 1, free disk 87Gi

## 2. and 5. Launcher before and after each try (raw: `round-9.launcher.txt`)

| When | Link inode | Target | mtime of the target (UTC) | Identifier | CDHash | Changed |
|---|---|---|---|---|---|---|
| before try 1 (22:05:49Z) | 2817021 | `versions/cf25f2b751f83336/repo` | 20:56:38Z | `tech.semio.repo` | `6de7547a8b8ff384bbfbfc45e962462063ff0357` | (reference) |
| after try 1 (22:26:21Z) | 2817021 | same | 20:56:38Z | `tech.semio.repo` | `6de7547a…` | no |
| after try 2 (22:54:55Z) | 2817021 | same | 20:56:38Z | `tech.semio.repo` | `6de7547a…` | no |
| after try 3 (22:57:55Z) | 2817021 | same | 20:56:38Z | `tech.semio.repo` | `6de7547a…` | no |

`~/.local/bin/repo`: inode 2811848, 18657778 bytes, mtime 20:35:40Z at all four times, unchanged.

## Tries

| Try | Log | Start | End | `native_rc` | Stale | Taken from the fleet at the start | Asked | Arrived | Missing | Seconds |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `~/native-bootstrap-round-9.log` | 22:05:49Z | 22:26Z | 75 | 658 | 0 | 658 | 159 | 499 | 905 |
| 2 | `~/native-bootstrap-round-9-try-2.log` | 22:28:22Z | 22:54Z | 75 | 658 | 207 | 451 | 48 | 403 | 1486 |
| 3 | `~/native-bootstrap-round-9-try-3.log` | 22:56:55Z | 22:57:33Z | 4 (consent) | not printed | not printed | not printed | not reached | not reached | not reached |

Waits: 120 s after try 1 and after try 2.

Too heavy projects named (try 1): at the start `6 too heavy for this machine (@semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 8452 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), @semio-tech/framework-plugin (about 4018 MB); it can give about 3512 MB per project)`; in the refusal still missing: `@semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), workspace (about 6200 MB)`. The refusal of try 2 names no too heavy project.

Refusal texts (bootstrap logs, verbatim):

```
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 658 and delivered 159 within 905 s, 499 are still missing, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), workspace (about 6200 MB) (this machine can give about 3512 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 451 and delivered 48 within 1486 s, 403 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

## Graph lines per try (graph service log, verbatim) and its last 30 lines when the try ended

### Try 1

```
== /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-9888631afea4.log (lines after 0)
-- graph lines
graph: 658 stale projects, 0 taken from the fleet, 6 too heavy for this machine (@semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 8452 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), @semio-tech/framework-plugin (about 4018 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 658
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 159 of 658 projects arrived
-- last 30 lines when the try ended
[repository:command] still running after 1060 s
[repo] still working after 1070 s; last phase started: source.served
graph: waiting for the fleet: 159 of 658 projects arrived
[repository:command] still running after 1070 s
graph: waiting for the fleet: 159 of 658 projects arrived
[repository:command] still running after 1080 s
[repository:command] still running after 1090 s
graph: waiting for the fleet: 159 of 658 projects arrived
[repo] still working after 1100 s; last phase started: source.served
[repository:command] still running after 1100 s
graph: waiting for the fleet: 159 of 658 projects arrived
[repository:command] still running after 1110 s
graph: waiting for the fleet: 159 of 658 projects arrived
[repository:command] still running after 1120 s
graph: waiting for the fleet: 159 of 658 projects arrived
[repo] still working after 1130 s; last phase started: source.served
[repository:command] still running after 1130 s
graph: waiting for the fleet: 159 of 658 projects arrived
[repository:command] still running after 1140 s
graph: waiting for the fleet: 159 of 658 projects arrived
[repository:command] still running after 1150 s
graph: waiting for the fleet: 159 of 658 projects arrived
[repo] still working after 1160 s; last phase started: source.served
[repository:command] still running after 1160 s
graph: waiting for the fleet: 159 of 658 projects arrived
[repository:command] still running after 1170 s
graph: waiting for the fleet: 159 of 658 projects arrived
graph: nothing to wait for any longer: the fleet was asked for 658 stale projects and delivered 159 within 905 s; 499 remain, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), workspace (about 6200 MB), which this machine must never infer itself (it can give about 3512 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T22:26:08.500Z graph-wanted-expired {"wanted":"658","arrived":"159","seconds":"905","missing":"499","local":"192","heavy":", among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), workspace (about 6200 MB) (this machine can give about 3512 MB per project)"}
[repository:command] still running after 1180 s
```

### Try 2

```
== /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-9888631afea4.log (lines after 1179)
-- graph lines
graph: 658 stale projects, 207 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 451
graph: waiting for the fleet: 0 of 451 projects arrived
graph: waiting for the fleet: 48 of 451 projects arrived
-- last 30 lines when the try ended
[repo] still working after 2780 s; last phase started: source.served
graph: waiting for the fleet: 48 of 451 projects arrived
[repository:command] still running after 2781 s
graph: waiting for the fleet: 48 of 451 projects arrived
[repository:command] still running after 2791 s
graph: waiting for the fleet: 48 of 451 projects arrived
[repository:command] still running after 2801 s
[repo] still working after 2810 s; last phase started: source.served
graph: waiting for the fleet: 48 of 451 projects arrived
[repository:command] still running after 2811 s
graph: waiting for the fleet: 48 of 451 projects arrived
[repository:command] still running after 2821 s
graph: waiting for the fleet: 48 of 451 projects arrived
[repository:command] still running after 2831 s
[repo] still working after 2840 s; last phase started: source.served
graph: waiting for the fleet: 48 of 451 projects arrived
[repository:command] still running after 2841 s
graph: waiting for the fleet: 48 of 451 projects arrived
[repository:command] still running after 2851 s
graph: waiting for the fleet: 48 of 451 projects arrived
[repository:command] still running after 2861 s
[repo] still working after 2870 s; last phase started: source.served
graph: waiting for the fleet: 48 of 451 projects arrived
[repository:command] still running after 2871 s
graph: waiting for the fleet: 48 of 451 projects arrived
[repository:command] still running after 2881 s
graph: nothing to wait for any longer: asked the fleet for 451 stale projects and received 48 within 1486 s, 403 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T22:54:43.306Z graph-wanted-expired {"wanted":"451","arrived":"48","seconds":"1486","missing":"403","local":"192","heavy":""}
[repository:command] still running after 2891 s
[repo] still working after 2900 s; last phase started: source.served
```

### Try 3

```
== /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-9888631afea4.log (lines after 2147)
-- graph lines
2026-11-14T22:57:33.166Z shutdown access-denied
-- last 30 lines when the try ended
[repository:command] still running after 2941 s
[repository:command] still running after 2951 s
[repo] still working after 2960 s; last phase started: source.served
[repository:command] still running after 2961 s
[repository:command] still running after 2971 s
[repository:command] still running after 2981 s
[repo] still working after 2990 s; last phase started: source.served
[repository:command] still running after 2991 s
[repository:command] still running after 3001 s
[repository:command] still running after 3011 s
[repo] still working after 3020 s; last phase started: source.served
[repository:command] still running after 3021 s
[repository:command] still running after 3031 s
[repository:command] still running after 3041 s
[repo] still working after 3050 s; last phase started: source.served
[repository:command] still running after 3051 s
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (6838 MB available, 2048 MB floor)
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] build:source-admission · Admitting the sources of the build
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
2026-11-14T22:57:33.166Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
    at failure (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148:307)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:153:768)
    at processTicksAndRejections (native:7:39)
2026-11-14T22:57:33.166Z shutdown access-denied
[capacity] the memory footprint of running process trees cannot be read on darwin (footprint printed no process); the resident set is used instead, which can overstate what a process holds
[capacity] Der Speicher-Fußabdruck laufender Prozessbäume kann unter darwin nicht gelesen werden (footprint printed no process); stattdessen gilt der residente Speicher, der überschätzen kann, was ein Prozess hält
repository:command failed with exit code 4
```

No line with `inferred here`, `restored` or `closureRepaired`.

## `[capacity]` and `graph:` lines of the bootstrap logs (verbatim)

Try 1:

```
[capacity] run: sized at 6% of the default from 15 earlier run(s) of its class, at least 4525 MB because of its assembly phase
[capacity] waiting 0s for run: not enough free memory (need 4525 MB; 4238 MB spare above the 2048 MB floor, load 7.65; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 6s for run: not enough free memory (need 4525 MB; 4445 MB spare above the 2048 MB floor, load 7.35; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted run with 1 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 30% of the default from 15 earlier run(s) of its class, at least 4525 MB because of its assembly phase
[capacity] run: sized at 100% of the default from 9 earlier run(s) of its class, at least 4525 MB because of its assembly phase
[capacity] run: sized at 100% of the default from 9 earlier run(s) of its class, at least 4525 MB because of its assembly phase
[capacity] dependencies: sized at 42% of the default from 15 earlier run(s) of its class, at least 5892 MB because of its assembly phase
[capacity] admitted dependencies with 3084 of 5892 MB: only 3084 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 42% of the default from 15 earlier run(s) of its class, at least 5892 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (5283 MB available, 2048 MB floor)
```

Try 2:

```
[capacity] run: sized at 6% of the default from 15 earlier run(s) of its class, at least 4522 MB because of its assembly phase
[capacity] admitted run with 3 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 14% of the default from 15 earlier run(s) of its class, at least 4522 MB because of its assembly phase
[capacity] run: sized at 100% of the default from 9 earlier run(s) of its class
[capacity] run: sized at 100% of the default from 9 earlier run(s) of its class
[capacity] dependencies: sized at 39% of the default from 15 earlier run(s) of its class, at least 5893 MB because of its assembly phase
[capacity] admitted dependencies with 3838 of 5893 MB: only 3838 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 39% of the default from 15 earlier run(s) of its class, at least 5893 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (5448 MB available, 2048 MB floor)
```

Try 3:

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class, at least 4517 MB because of its assembly phase
[capacity] admitted run with 3 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 11% of the default from 15 earlier run(s) of its class, at least 4517 MB because of its assembly phase
[capacity] run: sized at 100% of the default from 9 earlier run(s) of its class
[capacity] run: sized at 100% of the default from 9 earlier run(s) of its class
[capacity] dependencies: sized at 30% of the default from 15 earlier run(s) of its class, at least 5894 MB because of its assembly phase
[capacity] admitted dependencies with 4391 of 5894 MB: only 4391 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 30% of the default from 15 earlier run(s) of its class, at least 5894 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6838 MB available, 2048 MB floor)
```

## Samples

| Try | Samples | Pressure level other than 1 | Largest process |
|---|---|---|---|
| 1 | 41 | 0 | 3695888 KiB, pid 66114, ppid 66015 |
| 2 | 53 | 0 | 3167408 KiB, pid 77609, ppid 66015 |
| 3 | 2 | 0 | 2341616 KiB, pid 91217, ppid 47709 |

Replica agent `state = running` at the start, kickstarted 0 times; replica digest `590fdef8b758…`, `bootVersion` 6.

## Files

`run-round-9.sh`, `round-9.monitor.txt`, `round-9.prerun.txt`, `round-9.launcher.txt`, `round-9.try-<k>.samples.txt`, `round-9.try-<k>.service.txt`, `round-9.tail.txt` (last 200 lines of try 3), `round-9.try-1.tail.txt`, `round-9.try-2.tail.txt`.
