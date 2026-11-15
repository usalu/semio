# g1-m01 Round 7

## Before The Run (2026-11-14T20:28:16Z)

Replica code digest (`.cache/f/health.json`, field `code`): `d72ff30ebb826bfabac6ee5f126041ee5f833672897443b9666c557ce85335c1`

`ps -axo pid,ppid,etime,rss,args` of the graph service processes (cut at 260 characters):
```
89646     1       33:35  82608 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M^[M^MM-oM-8M^Oproducts/M-pM^_M-&M^QM-oM-8M^Orepo
89655 89646       33:32 445232 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'),crypto=awa
```

Graph service jobs (`launchctl list`; `launchctl print` lines state, runs, pended nondemand spawn, last exit code):
```
tech.semio.repo.service.c6c0f63aa094
	state = not running
	runs = 1
	last exit code = 4
		state = active
		state = active
tech.semio.repo.service.7a7ed4bee027
	state = running
	runs = 1
	last exit code = (never exited)
		state = active
		state = active
```

Last 8 lines of `.cache/f/sidecars/build-agent.log`:
```
2026-11-14T20:10:27.433Z worker registered: 9eef2daa-5e7d-4804-b47f-4aec646f9a9c cache http://192.168.178.70:6094
2026-11-14T20:15:26.278Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T20:20:26.925Z agent code changed, restarting
2026-11-14T20:20:26.930Z worker stopped: ueli-workstation-mac-mini-m6-16gb-256gb-01-lan.fritz.box-98089
2026-11-14T20:20:27.808Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T20:20:27.850Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T20:20:27.853Z worker registered: a270fe09-f9cc-4a8a-8f13-cf5c25a6de5f cache http://192.168.178.70:6094
2026-11-14T20:25:25.851Z graph-server: alive, 0 machines seen, 0 requests held, read none
```

## Result (written 2026-11-14T21:03:04Z): consent

- Machine: Mac mini, 16 GiB memory, macOS 27.2.1, free disk 86Gi of 228Gi.
- Start 2026-11-14T20:28:19Z, end 2026-11-14T21:02:29Z. Two tries. The round stopped at try 2 with `native_rc=4`: **consent** missing; not retried.
- `.cache/f/native-bootstrap.done` absent; not ready. No try got past the graph step: no nx task ran; succeeded, failed, not-run task lists = none.
- Memory pressure level 1 in all 64 samples; no hard stop. Peak resident size of a process of the run: 3 511 984 KiB.
- No `The graph service could not answer (timeout)`: no `launchctl kickstart` was needed or done. The replica job was running at the end of both tries: 0 kickstarts.
- No ENOENT, no `closureRepaired`, no gc.

## Consent Text (try 2, verbatim)

```
[semio] The launcher changed: macOS asks once more for the file access consent of the launcher (a running service keeps its file until it stops).
[semio] Der Launcher hat sich geändert: macOS fragt einmal erneut nach der Dateizugriffs-Erlaubnis des Launchers (ein laufender Dienst behält seine Datei, bis er endet).
A macOS dialog waits for your answer: "repo would like to access files in your Documents folder" (/Users/ueli/.local/share/semio/launcher/bin/repo). Click "Allow" in the session of the logged-in user (or switch on Documents Folder for `repo` under System Settings, Privacy & Security, Files & Folders), then run the command again. /Users/ueli/Documents/semio cannot be read before that.
```
The log says the launcher changed and macOS asks once more; launcher CDHash now, compared with round 6 (`713fd0d142560c91e4fd5ed6c449b32909e667e7`):
```
CDHash=6de7547a8b8ff384bbfbfc45e962462063ff0357
lrwxr-xr-x@ 1 ueli  staff  30 Nov 14 12:59 /Users/ueli/.local/share/semio/launcher/bin/repo -> versions/cf25f2b751f83336/repo
```
This agent clicks no macOS consent. The owner must click "Allow" on the dialog at the Mac or switch on Documents Folder for `repo` (`/Users/ueli/.local/share/semio/launcher/bin/repo`) under System Settings, Privacy & Security, Files & Folders. Second round in a row that ends this way; the pattern is the same as in round 6: try 1 runs 25 to 29 minutes on an already running service without a denial, the tree's build code changes meanwhile, try 2 starts a new service and that one has no consent.

## Tries

| try | start (UTC) | end (UTC) | native_rc | wanted | arrived | missing | seconds | local |
|---|---|---|---|---|---|---|---|---|
| 1 | 20:28:19 | 20:57:01 | 75 | 608 | 136 | 472 | 1399 | 192 |
| 2 | 20:59:28 | 21:02:20 | 4 (consent) | not reached | – | – | – | – |

Graph lines of try 1 (service log `host-6f286e53e68a.log`), verbatim, consecutive repeats removed, prefix = line number:
```
148:2026-11-14T20:26:53.496Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
152:2026-11-14T20:26:53.496Z shutdown access-denied
970:graph: 656 stale projects, 48 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 608
971:graph: waiting for the fleet: 0 of 608 projects arrived
1088:graph: waiting for the fleet: 88 of 608 projects arrived
1144:graph: waiting for the fleet: 136 of 608 projects arrived
1299:2026-11-14T20:57:01.283Z graph-wanted-expired {"wanted":"608","arrived":"136","seconds":"1399","missing":"472","local":"192"}
```
Bootstrap log, try 1:
```
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 608 and delivered 136 within 1399 s, 472 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```
Try 2, newest service log `host-6f286e53e68a.log`, lines with request-failed, access-denied, consent, shutdown:
```
148:2026-11-14T20:26:53.496Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
152:2026-11-14T20:26:53.496Z shutdown access-denied
```

## `[capacity]` Lines (verbatim, distinct)

Try 1:
```
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 2945 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3638 MB reserved (6600 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 12 earlier run(s) of its class
[capacity] dependencies: sized at 33% of the default from 16 earlier run(s) of its class, at least 4713 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 4713 MB reserved (7338 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7121 MB available, 2048 MB floor)
```
Try 2:
```
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 2948 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3641 MB reserved (7464 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 12 earlier run(s) of its class
[capacity] dependencies: sized at 33% of the default from 16 earlier run(s) of its class, at least 4720 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 4720 MB reserved (7306 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7137 MB available, 2048 MB floor)
```

## Checks of ROUND-NOTE-P6.md

Sections 1, 2, 4 after try 1 and section 3 after try 2: full output in `round-7.checks.txt`.

State after try 2 (21:03:04Z):
```
tech.semio.repo.service.c6c0f63aa094
	state = running
	program = /Users/ueli/.local/share/semio/launcher/bin/repo
	runs = 1
	last exit code = (never exited)
		state = active
		state = active
tech.semio.repo.service.7a7ed4bee027
	state = not running
	program = /Users/ueli/.local/share/semio/launcher/bin/repo
	runs = 1
	last exit code = 0
		state = active
		state = active
receipts:
service.json: {"version":1,"role":"service","pid":8520,"state":"ok","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794688290412}
client.json: {"version":1,"role":"client","pid":24535,"state":"ok","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794690134204}
launcher.json: {"version":1,"role":"launcher","pid":24535,"state":"waiting","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794690140772}
```

## Wall Time Per Phase

| try | total | graph step |
|---|---|---|
| 1 | 28 min 42 s | about 1530 s (fleet wait 1399 s) |
| 2 | 2 min 52 s | not reached: the service start waited for the dialog |

## Files

`round-7.samples.txt`, `round-7.checks.txt`, `round-7.tail.txt` (last 200 lines of the try 2 log); logs `~/native-bootstrap-round-7.log`, `~/native-bootstrap-round-7-try-2.log`.
