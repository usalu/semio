# g1-m01 Round 6

## Before The Run (2026-11-14T19:51:31Z)

Replica code digest (`.cache/f/health.json`, field `code`): `28b4f47dc4d69fae3a080d0c58d3520741c02c444760d2a3cec506db943ae8ff`

`ps -axo pid,ppid,etime,rss,args` of the graph service processes (cut at 260 characters):
```
(no such process at this time)
```

Last 8 lines of `.cache/f/sidecars/build-agent.log`:
```
2026-11-14T19:41:54.639Z worker reconnecting: Unable to connect. Is the computer able to access the url?
2026-11-14T19:41:55.695Z worker reconnecting: Unable to connect. Is the computer able to access the url?
2026-11-14T19:41:56.763Z worker reconnecting: Unable to connect. Is the computer able to access the url?
2026-11-14T19:41:57.851Z worker reconnecting: Unable to connect. Is the computer able to access the url?
2026-11-14T19:41:58.881Z worker registered: a812dfb0-7617-4523-a677-e9333e448fd2 cache http://192.168.178.70:6094
2026-11-14T19:42:01.704Z load report sent: reachable again
2026-11-14T19:44:08.218Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T19:49:11.662Z graph-server: alive, 0 machines seen, 0 requests held, read none
```

## Result (written 2026-11-14T20:27:58Z): consent

- Machine: Mac mini, 16 GiB memory, macOS 27.2.1, free disk 86Gi of 228Gi.
- Start 2026-11-14T19:52:07Z, end 2026-11-14T20:27:18Z. Two tries. The round stopped at try 2 with `native_rc=4`: **consent** missing. Not retried, as the round says.
- `.cache/f/native-bootstrap.done` absent; not ready. `workspace:setup` did not get past the graph step in either try: no nx task ran; succeeded, failed, not-run task lists = none.
- Memory pressure level 1 in all 66 samples; no hard stop. Peak resident size of a process of the run: 3 022 160 KiB.
- The ENOENT of round 5 (`build-input-closures/…json`) did not come back; no `closureRepaired` line in any log; `repo cache gc --area=graph-inference` was not needed and not run.

## Consent Text (try 2, verbatim)

```
macOS denied the resident service access to your Documents folder while it answered a request (reference e07192d7). The service has stopped itself; it runs with the file access consent of the process that started it and not of `repo`. Switch on Documents Folder for `repo` (/Users/ueli/.local/share/semio/launcher/bin/repo) under System Settings, Privacy & Security, Files & Folders (or Full Disk Access), then run the command again, or keep the checkout outside Documents, Desktop and Downloads. Details: /var/folders/gd/byw98s3s0p95zhdpzyv769740000gn/T/semio-repo/501/host-6f286e53e68a.log.
```
Service log `host-6f286e53e68a.log`:
```
148:2026-11-14T20:26:53.496Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
152:2026-11-14T20:26:53.496Z shutdown access-denied
```
No macOS dialog was answered by this agent (it never clicks consents); whether a dialog was shown on the screen was not measured. The owner must switch on Documents Folder for `repo` (`/Users/ueli/.local/share/semio/launcher/bin/repo`) under System Settings, Privacy & Security, Files & Folders.

Observation: the service of try 1 (launchd job `tech.semio.repo.service.7a7ed4bee027`, program = the launcher, receipt `state: ok`) read the Documents folder for 29 minutes without a denial. The denial came on the first request to the next service (`host-6f286e53e68a.log`, another implementation tag, started for try 2). Same pattern as rounds 2, 3 and 5: the first request to a freshly started service is denied.

## Tries

| try | start (UTC) | end (UTC) | native_rc | wanted | arrived | missing | seconds | local |
|---|---|---|---|---|---|---|---|---|
| 1 | 19:52:07 | 20:21:32 | 75 | 655 | 150 | 505 | 1498 | 192 |
| 2 | 20:23:47 | 20:26:53 | 4 (consent) | not reached | – | – | 2 | – |

Wait between the tries: 120 s (plus the checks).

Graph lines, service log `host-293b476f6b9d.log` (try 1), verbatim, consecutive repeats removed, prefix = line number:
```
141:graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
791:graph: 655 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 655
792:graph: waiting for the fleet: 0 of 655 projects arrived
869:graph: waiting for the fleet: 94 of 655 projects arrived
935:graph: waiting for the fleet: 142 of 655 projects arrived
1048:graph: waiting for the fleet: 150 of 655 projects arrived
1141:graph: nothing to wait for any longer: asked the fleet for 655 stale projects and received 150 within 1498 s, 505 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
1142:2026-11-14T20:21:32.139Z graph-wanted-expired {"wanted":"655","arrived":"150","seconds":"1498","missing":"505","local":"192"}
```
Bootstrap log, try 1:
```
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 655 and delivered 150 within 1498 s, 505 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```
Try 1 started from 655 stale projects with `0 taken from the fleet` (round 5 try 2 had 509 stale and 456 arrived): nothing of the earlier deliveries was reused. No `restored`, `inferred here` or `closureRepaired` line in try 1; try 2 has no graph line at all.

## `[capacity]` Lines (verbatim, distinct)

Try 1:
```
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 2941 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3634 MB reserved (7232 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 12 earlier run(s) of its class
[capacity] dependencies: sized at 37% of the default from 16 earlier run(s) of its class, at least 4704 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 4704 MB reserved (7130 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6969 MB available, 2048 MB floor)
```
Try 2:
```
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 2944 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3637 MB reserved (7553 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 12 earlier run(s) of its class
[capacity] dependencies: sized at 37% of the default from 16 earlier run(s) of its class, at least 4712 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 4712 MB reserved (6939 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6922 MB available, 2048 MB floor)
```

## Checks of ROUND-NOTE-P6.md (sections 1, 2, 4 after try 1; section 3 after try 2)

Full output: `round-6.checks.txt`. Summary:
- Section 1: one job `tech.semio.repo.service.7a7ed4bee027`, pid 89646, `program = /Users/ueli/.local/share/semio/launcher/bin/repo`, arguments `<launcher> …/door/📜️script.ts repo service host run`, working directory `/Users/ueli/Documents/semio`, state running. Wrapper pid 89646 has parent 1 (shown as bun in `ps`). `SEMIO_SERVICE_LAUNCHD` unset. `/usr/local/bin/repo -> /Users/ueli/.local/bin/repo` (still the old target; the provision script has not run again, and this agent uses no sudo).
- Section 2: `bootVersion` 6. Chain: `/bin/sh …/entry/boot.sh run` (97867, parent 1) → `bun …/entry/boot.mjs run` (97875) → `bun …/shared-worktree/…` (97876). The child of `boot.sh` shows as bun; `entry/shim.txt` does not exist, so the launcher step could not be confirmed from a shim log.
- Section 3 (after the exit 4): `0 rows (of 1 consulted requests in the window)`.
- Section 4 after try 1: receipts `client.json`, `launcher.json`, `service.json`; `service.json`: `state: ok`, `identity: /Users/ueli/.local/share/semio/launcher/bin/repo`, `folder: Documents`, pid 89655. `repo preflight --lang en` ended with exit 0 and printed no `access` line and no `worktree-access-denied` (three progress lines only, in the checks file).
- codesign: launcher CDHash `713fd0d142560c91e4fd5ed6c449b32909e667e7`, `~/.local/bin/repo` CDHash `71734f86275220c08e184dd6a8137df653e92f50`, both Identifier `a.out`, no team. No value from an earlier round to compare with (not measured before).

State after try 2 (20:27:58Z):
```
launchctl list | grep tech.semio.repo.service:
-	4	tech.semio.repo.service.c6c0f63aa094
89646	0	tech.semio.repo.service.7a7ed4bee027
receipts:
service.json: {"version":1,"role":"service","pid":6224,"state":"denied","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794688013519}
client.json: {"version":1,"role":"client","pid":6188,"state":"ok","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794688001784}
launcher.json: {"version":1,"role":"launcher","pid":6188,"state":"ok","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794688002049}
```

## Wall Time Per Phase

| try | total | graph step |
|---|---|---|
| 1 | 29 min 25 s | 1604 s (fleet wait 1498 s) |
| 2 | 3 min 6 s | 2 s, then the denial |

## Files

`round-6.samples.txt`, `round-6.checks.txt`, `round-6.tail.txt` (last 200 lines of the try 2 log); logs `~/native-bootstrap-round-6.log`, `~/native-bootstrap-round-6-try-2.log`.
