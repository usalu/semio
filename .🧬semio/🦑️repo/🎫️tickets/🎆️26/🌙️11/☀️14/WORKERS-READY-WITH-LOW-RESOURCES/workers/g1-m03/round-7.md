# g1-m03 Round 7

- Machine: Mac mini M4, 16 GB memory, macOS 27.2.1 (26B131), free disk 88 GiB of 228 GiB before the run. Worktree `/Users/ueli/Documents/semio`.
- Result: **consent**. Two tries. Try 1 ended with the fleet refusal (`native_rc=75`, 56 of 515 arrived). Try 2 ended with **`native_rc=4`**: macOS denied the graph service the Documents folder. No further try, as the round says. Marker absent, `ready` false. No hard stop, memory pressure level 1 in every sample of both tries. The setup tasks were not reached in this round.
- No graph service timeout occurred (`The graph service could not answer (timeout)` is in no log), so no `launchctl kickstart` was run. The replica agent was `state = running` at every look; it was kickstarted 0 times.

## Consent text (try 2, verbatim from `~/native-bootstrap-round-7-try-2.log`)

```
macOS denied the resident service access to your Documents folder while it answered a request (reference e07192d7). The service has stopped itself; it runs with the file access consent of the process that started it and not of `repo`. Switch on Documents Folder for `repo` (/Users/ueli/.local/share/semio/launcher/bin/repo) under System Settings, Privacy & Security, Files & Folders (or Full Disk Access), then run the command again, or keep the checkout outside Documents, Desktop and Downloads. Details: /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-6f286e53e68a.log.
end 2026-11-14T20:39:33Z
native_rc=4
```

The owner has to switch on Documents Folder for `repo` (`/Users/ueli/.local/share/semio/launcher/bin/repo`) under System Settings, Privacy & Security, Files & Folders (or answer the dialog, if one is shown in the session). This worker agent changed no privacy setting.

What happened around it, as observed:

- The graph service that was denied (pid 18649, job `tech.semio.repo.service.a909995b451a`) was started in try 1 at about 20:10Z and had the receipt `service.json` `state: ok` then. It served try 1 for 23 minutes. At 20:39:33Z, in try 2, the same process got `request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)`, wrote `shutdown access-denied` and ended with exit 4 (`launchctl print`: `state = not running`, `runs = 1`, `last exit code = 4`).
- The launcher binary `/Users/ueli/.local/share/semio/launcher/bin/repo` has the modification time 2026-11-14 20:35:26Z, and `~/.local/bin/repo` 20:35:40Z: both were rebuilt by the bootstrap of try 2 (started 20:35:24Z; every run prints `[semio] Building the repo launcher`). Its CDHash now is `713fd0d142560c91e4fd5ed6c449b32909e667e7` (Identifier `a.out`, no team identifier). The CDHash of the earlier rounds was not recorded here (section 3 was not due before), so "the CDHash changed" is not measured; the rebuild time four minutes before the denial is.
- `tcc-attribution.py --last 30m`, `--last 60m` and `--last 60m --all` each print `0 rows (of 3 consulted requests in the window)`: no row of the service to paste.

Full output of section 3: `round-7.p6-section3.txt`:

```
P6 section 3 on g1-m03, 2026-11-14T20:40:35Z, after try 2 ended with native_rc=4 at 20:39:33Z (21:39:33 local)

$ python3 …/P6/tcc-attribution.py --last 30m
0 rows (of 3 consulted requests in the window)

$ codesign -dvvv ~/.local/share/semio/launcher/bin/repo ~/.local/bin/repo 2>&1 | grep -E "Identifier|CDHash"
-- /Users/ueli/.local/share/semio/launcher/bin/repo (Nov 14 21:35:26 2026)
Identifier=a.out
CandidateCDHash sha256=713fd0d142560c91e4fd5ed6c449b32909e667e7
CandidateCDHashFull sha256=713fd0d142560c91e4fd5ed6c449b32909e667e754cba2ac1f50b10a6458608a
CDHash=713fd0d142560c91e4fd5ed6c449b32909e667e7
TeamIdentifier=not set
-- /Users/ueli/.local/bin/repo (Nov 14 21:35:40 2026)
Identifier=a.out
CandidateCDHash sha256=71734f86275220c08e184dd6a8137df653e92f50
CandidateCDHashFull sha256=71734f86275220c08e184dd6a8137df653e92f50c050d3df9782a84e0ed30361
CDHash=71734f86275220c08e184dd6a8137df653e92f50
TeamIdentifier=not set

## graph service jobs after the exit 4
-	4	tech.semio.repo.service.a909995b451a
-	0	tech.semio.repo.service.d8505b2379b9
-- tech.semio.repo.service.a909995b451a
	state = not running
	program = /Users/ueli/.local/share/semio/launcher/bin/repo
	working directory = /Users/ueli/Documents/semio
	runs = 1
	last exit code = 4
		state = active
		state = active
-- tech.semio.repo.service.d8505b2379b9
	state = not running
	program = /Users/ueli/.local/share/semio/launcher/bin/repo
	working directory = /Users/ueli/Documents/semio
	runs = 1
	last exit code = 0
		state = active
		state = active

## receipts
-- client.json: {"version":1,"role":"client","pid":33170,"state":"ok","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794688770173}
-- launcher.json: {"version":1,"role":"launcher","pid":18618,"state":"ok","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794687004102}
-- service.json: {"version":1,"role":"service","pid":18649,"state":"denied","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794688773517}

## graph service log host-6f286e53e68a.log: lines with access-denied, consent, request-failed, e07192d7 (and 6 lines after the first)
1245:2026-11-14T20:39:33.489Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
1249:2026-11-14T20:39:33.490Z shutdown access-denied
    at failure (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148:307)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:153:768)
    at processTicksAndRejections (native:7:39)
2026-11-14T20:39:33.490Z shutdown access-denied
repository:command failed with exit code 4
-- its last 12 lines
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (7143 MB available, 2048 MB floor)
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] build:source-admission · Admitting the sources of the build
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
2026-11-14T20:39:33.489Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
    at failure (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148:307)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:153:768)
    at processTicksAndRejections (native:7:39)
2026-11-14T20:39:33.490Z shutdown access-denied
repository:command failed with exit code 4

$ python3 …/P6/tcc-attribution.py --last 60m --all
0 rows (of 3 consulted requests in the window)

$ python3 …/P6/tcc-attribution.py --last 60m
0 rows (of 3 consulted requests in the window)
```

## 1. Before the run and the P6 checks after the first `repo` run (sections 1, 2, 4)

    Pre-run facts, 2026-11-14T20:09:21Z
    - replica code digest (.cache/f/health.json): 28b4f47dc4d69fae3a080d0c58d3520741c02c444760d2a3cec506db943ae8ff, store ok, daemon pid 4292
    - replica agent: launchctl print gui/501/tech.semio.fleet.replica:
        	state = running
        	runs = 43
        	pid = 4284
        	last exit code = 0
        		state = active
        		state = active
    - `repo service host` processes (pid, ppid, age, rss KiB):
        pid 8911 ppid 1 age 16:28 rss 298720 (… repo service host run)
        pid 8942 ppid 8911 age 16:25 rss 520288 (… repo service host run)
        pid 18021 ppid 18006 age 00:00 rss 912 (… repo service host run)
      (2 process(es))
    - graph service job(s):
        tech.semio.repo.service.d8505b2379b9
        	state = running
        	runs = 1
        	last exit code = (never exited)
        		state = active
        		state = active
    - last 8 lines of .cache/f/sidecars/build-agent.log:
        2026-11-14T20:09:03.911Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T20:09:05.348Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T20:09:06.476Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T20:09:07.712Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T20:09:08.774Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T20:09:09.906Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T20:09:10.939Z worker registered: 41c8df99-c2c4-47d4-8471-8e17b6f931a1 cache http://192.168.178.70:6094
        2026-11-14T20:09:13.256Z load report sent: reachable again
    - memory pressure level 1, free disk 88Gi

P6 checks at 20:10Z (during try 1, before the denial):

```
P6 checks on g1-m03, 2026-11-14T20:10:19Z, after the first repo run of round 7 (try 1)

## 1. How the service is started
$ launchctl list | grep tech.semio.repo.service
18635	0	tech.semio.repo.service.a909995b451a
8911	0	tech.semio.repo.service.d8505b2379b9
$ launchctl print gui/501/tech.semio.repo.service.a909995b451a | grep -E "program|arguments|working directory|state|runs|pended nondemand spawn|last exit"
	state = running
	program = /Users/ueli/.local/share/semio/launcher/bin/repo
	working directory = /Users/ueli/Documents/semio
	runs = 1
	last exit code = (never exited)
		state = active
		state = active
	arguments = {
		/Users/ueli/.local/share/semio/launcher/bin/repo
		/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts
		repo
		service
		host
		run
	}
$ launchctl print gui/501/tech.semio.repo.service.d8505b2379b9 | grep -E "program|arguments|working directory|state|runs|pended nondemand spawn|last exit"
	state = running
	program = /Users/ueli/.local/share/semio/launcher/bin/repo
	working directory = /Users/ueli/Documents/semio
	runs = 1
	last exit code = (never exited)
		state = active
		state = active
	arguments = {
		/Users/ueli/.local/share/semio/launcher/bin/repo
		/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts
		repo
		service
		host
		run
	}
$ ps -o pid,ppid,command -p 18635
  PID  PPID COMMAND
18635     1 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M^[M^MM-oM-8M^Oproducts/M-pM^_M-&M^QM-oM-8M^Orepo/M-pM^_M^TM-(M-oM-8M^Omodules/M-pM^_M^SM^ZM-oM-8M^Olibrary/
$ ps -o pid,ppid,command -p 8911
  PID  PPID COMMAND
 8911     1 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M^[M^MM-oM-8M^Oproducts/M-pM^_M-&M^QM-oM-8M^Orepo/M-pM^_M^TM-(M-oM-8M^Omodules/M-pM^_M^SM^ZM-oM-8M^Olibrary/
$ ls -l /usr/local/bin/repo
lrwxr-xr-x@ 1 root  wheel  27 Nov 13 22:02 /usr/local/bin/repo -> /Users/ueli/.local/bin/repo

## 2. Identity of the replica daemon
"bootVersion":6
18571     1 /bin/sh /Users/ueli/Library/Application Support/semio/fleet/entry/boot.sh run
18579 18571 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Library/Application Support/semio/fleet/entry/boot.mjs run
18580 18579 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M^[M^MM-oM-8M^Oproducts/M-pM^_M-&M^QM-oM-8M^Orepo/M-pM^_M^TM-(M-oM-8M^Omodules/M-pM^_M-$M^]M-oM-8M^Oshared-w
shim.txt: ls: /Users/ueli/Library/Application Support/semio/fleet/entry/shim.txt: No such file or directory

## 4. Receipts
-- /Users/ueli/Library/Application Support/semio/access/3f08bbea4d32/client.json
{"version":1,"role":"client","pid":18618,"state":"ok","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794687003860}
-- /Users/ueli/Library/Application Support/semio/access/3f08bbea4d32/launcher.json
{"version":1,"role":"launcher","pid":18618,"state":"ok","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794687004102}
-- /Users/ueli/Library/Application Support/semio/access/3f08bbea4d32/service.json
{"version":1,"role":"service","pid":18649,"state":"ok","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794687015340}
```

`repo preflight` in the checkout root was not run again in this round: in round 6 it waited 270 s in the capacity door and printed no check line. The fleet preflight of round 6 (19:58Z) had `ok worktree access: the replica can read /Users/ueli/Documents/semio`.

Report back per machine: consent text seen **yes, exit code 4** (try 2); section 1 `program = /Users/ueli/.local/share/semio/launcher/bin/repo` (two jobs: `…d8505b2379b9` of round 6 and `…a909995b451a` of this round, both `runs = 1`; the new one was started by launchd without a kickstart); section 2 `bootVersion` 6, chain `sh boot.sh` -> `bun …/entry/boot.mjs run` -> `bun …/🔁️replica/…`, no `entry/shim.txt`; section 3 no rows; section 4 receipts at 20:10Z `client`, `launcher`, `service` all `state: ok` with identity the launcher, after the denial `service.json` `state: denied`.

## 2. to 4. Tries

| Try | Log | Start | End | `native_rc` | Stale | Taken from the fleet at the start | Asked | Arrived | Missing | Seconds |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `~/native-bootstrap-round-7.log` | 20:09:21Z | 20:33:10Z | 75 | 658 | 143 | 515 | 56 | 459 | 1218 |
| 2 | `~/native-bootstrap-round-7-try-2.log` | 20:35:24Z | 20:39:33Z | 4 (consent) | not printed | not printed | not printed | not reached | not reached | not reached |

Wait between the tries: 120 s.

## 5. Graph lines per try (graph service log, verbatim) and its last 30 lines when the try ended

### Try 1

```
== /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-293b476f6b9d.log (lines after 1082)
-- graph lines
-- last 30 lines when the try ended
graph: waiting for the fleet: 143 of 657 projects arrived
[repository:command] still running after 920 s
graph: waiting for the fleet: 143 of 657 projects arrived
[repository:command] still running after 930 s
SIGINT received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.
Cancelled (SIGINT); leases released.
[repository:command] still running after 940 s
[repo] still working after 950 s; last phase started: source.served
[repository:command] still running after 950 s
[repository:command] still running after 960 s
[repository:command] still running after 970 s
[repo] still working after 980 s; last phase started: source.served
[repository:command] still running after 980 s
[repository:command] still running after 990 s
[repository:command] still running after 1000 s
[repo] still working after 1010 s; last phase started: source.served
[repository:command] still running after 1010 s
[repository:command] still running after 1020 s
[repository:command] still running after 1030 s
[repo] still working after 1040 s; last phase started: source.served
[repository:command] still running after 1040 s
[repository:command] still running after 1050 s
[repository:command] still running after 1060 s
[repo] still working after 1070 s; last phase started: source.served
[repository:command] still running after 1070 s
[repository:command] still running after 1080 s
[repository:command] still running after 1090 s
[repo] still working after 1100 s; last phase started: source.served
2026-11-14T20:11:15.556Z shutdown superseded
[script] {"owner":"repository:build","command":"repo","stage":"complete"}
== /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-6f286e53e68a.log (lines after 0)
-- graph lines
graph: 658 stale projects, 143 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 515
graph: waiting for the fleet: 0 of 515 projects arrived
graph: waiting for the fleet: 8 of 515 projects arrived
graph: waiting for the fleet: 56 of 515 projects arrived
-- last 30 lines when the try ended
graph: waiting for the fleet: 56 of 515 projects arrived
[repository:command] still running after 1290 s
graph: waiting for the fleet: 56 of 515 projects arrived
[repository:command] still running after 1300 s
[repo] still working after 1310 s; last phase started: source.served
graph: waiting for the fleet: 56 of 515 projects arrived
[repository:command] still running after 1310 s
graph: waiting for the fleet: 56 of 515 projects arrived
[repository:command] still running after 1320 s
graph: waiting for the fleet: 56 of 515 projects arrived
[repository:command] still running after 1330 s
[repo] still working after 1340 s; last phase started: source.served
graph: waiting for the fleet: 56 of 515 projects arrived
[repository:command] still running after 1340 s
graph: waiting for the fleet: 56 of 515 projects arrived
[repository:command] still running after 1350 s
graph: waiting for the fleet: 56 of 515 projects arrived
[repository:command] still running after 1360 s
[repo] still working after 1370 s; last phase started: source.served
graph: waiting for the fleet: 56 of 515 projects arrived
[repository:command] still running after 1370 s
graph: waiting for the fleet: 56 of 515 projects arrived
[repository:command] still running after 1380 s
graph: waiting for the fleet: 56 of 515 projects arrived
graph: nothing to wait for any longer: asked the fleet for 515 stale projects and received 56 within 1218 s, 459 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T20:33:17.694Z graph-wanted-expired {"wanted":"515","arrived":"56","seconds":"1218","missing":"459","local":"192"}
[capacity] the memory footprint of running process trees cannot be read on darwin (footprint printed no process); the resident set is used instead, which can overstate what a process holds
[capacity] Der Speicher-Fußabdruck laufender Prozessbäume kann unter darwin nicht gelesen werden (footprint printed no process); stattdessen gilt der residente Speicher, der überschätzen kann, was ein Prozess hält
[repository:command] still running after 1390 s
[repo] still working after 1400 s; last phase started: source.served
```

### Try 2

```
== /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-6f286e53e68a.log (lines after 1205)
-- graph lines
2026-11-14T20:39:33.490Z shutdown access-denied
-- last 30 lines when the try ended
[repo] still working after 1640 s; last phase started: source.served
[repository:command] still running after 1640 s
[repository:command] still running after 1650 s
[repository:command] still running after 1660 s
[repo] still working after 1670 s; last phase started: source.served
[repository:command] still running after 1670 s
[repository:command] still running after 1680 s
[repository:command] still running after 1690 s
[repo] still working after 1700 s; last phase started: source.served
[repository:command] still running after 1700 s
[repository:command] still running after 1710 s
[repository:command] still running after 1720 s
[repo] still working after 1730 s; last phase started: source.served
[repository:command] still running after 1730 s
[repository:command] still running after 1740 s
[repository:command] still running after 1750 s
[repo] still working after 1760 s; last phase started: source.served
[repository:command] still running after 1760 s
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (7143 MB available, 2048 MB floor)
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] build:source-admission · Admitting the sources of the build
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
2026-11-14T20:39:33.489Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
    at failure (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148:307)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:153:768)
    at processTicksAndRejections (native:7:39)
2026-11-14T20:39:33.490Z shutdown access-denied
repository:command failed with exit code 4
```

No line with `inferred here`, `restored` or `closureRepaired` in this round.

Refusal of try 1 in the bootstrap log (verbatim):

```
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 515 and delivered 56 within 1218 s, 459 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

## `[capacity]` and `graph:` lines of the bootstrap logs (verbatim)

Try 1:

```
[capacity] run: sized at 6% of the default from 15 earlier run(s) of its class, at least 4543 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 5397 MB reserved (7618 MB available, 2048 MB floor)
[capacity] run: sized at 6% of the default from 15 earlier run(s) of its class, at least 4543 MB because of its assembly phase
[capacity] run: sized at 46% of the default from 9 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 9 earlier run(s) of its class
[capacity] dependencies: sized at 76% of the default from 14 earlier run(s) of its class, at least 5887 MB because of its assembly phase
[capacity] admitted dependencies with 5000 of 5887 MB: only 5000 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 76% of the default from 14 earlier run(s) of its class, at least 5887 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7098 MB available, 2048 MB floor)
```

Try 2:

```
[capacity] run: sized at 6% of the default from 15 earlier run(s) of its class, at least 4539 MB because of its assembly phase
[capacity] admitted run with 3 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 14% of the default from 15 earlier run(s) of its class, at least 4539 MB because of its assembly phase
[capacity] run: sized at 100% of the default from 9 earlier run(s) of its class
[capacity] run: sized at 100% of the default from 9 earlier run(s) of its class
[capacity] dependencies: sized at 76% of the default from 15 earlier run(s) of its class, at least 5888 MB because of its assembly phase
[capacity] admitted dependencies with 5396 of 5888 MB: only 5396 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 76% of the default from 15 earlier run(s) of its class, at least 5888 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 4 earlier run(s) of its class
[capacity] waiting 0s for run: not enough free memory (need 2048 MB; 1750 MB spare above the 2048 MB floor, load 5.44; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted run: 1 job(s), 2048 MB reserved (7143 MB available, 2048 MB floor)
```

## Samples and wall time

| Try | Samples | Pressure level other than 1 | Largest process | Whole run |
|---|---|---|---|---|
| 1 | 48 | 0 | 2908720 KiB, pid 18742, ppid 18649 | 23 min 49 s, of which 1218 s waiting for the fleet |
| 2 | 9 | 0 | 3535280 KiB, pid 31744, ppid 31714 | 4 min 9 s (about 220 s dependency scopes, then `workspace:setup` until the denial) |

## Files

`run-round-7.sh`, `round-7.monitor.txt`, `round-7.prerun.txt`, `round-7.p6-checks.txt`, `round-7.p6-section3.txt`, `round-7.try-<k>.samples.txt`, `round-7.try-<k>.service.txt`, `round-7.tail.txt` (last 200 lines of try 2), `round-7.try-1.tail.txt`.
