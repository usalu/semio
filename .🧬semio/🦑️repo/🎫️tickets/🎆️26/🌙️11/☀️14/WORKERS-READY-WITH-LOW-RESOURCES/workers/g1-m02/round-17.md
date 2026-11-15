# g1-m02 Round 17

round: 17

Result: `consent`, not ready. One try, ended after 53 s with `native_rc=4`; as ROUND.md says for `native_rc=4` I stopped the series and report. No nx task ran (succeeded: none, failed: none, not run: all). `.cache/f/native-bootstrap.done` does not exist. No plan line was printed, so there is no `MB per project` line to report for this try.

## Machine

Mac mini `Mac16,10`, Apple M4, 10 cores, 16 GB (17179869184 bytes), macOS 27.2.1 (26B131). Free disk: not measured in this round.

## What Happened

| Try | Log | Start (UTC) | End (UTC) | `native_rc` | Largest process on the machine | How it ended |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | `~/native-bootstrap-round-17.log` | 03:47:35 | 03:48:28 | 4 | 2374656 KiB | macOS denied the graph service twice: under the launcher, then under the caller's access |

The fallback of round 10 ran and printed its line, then the second service was denied too. Bootstrap log, verbatim:

```
Starting the repo graph service for this host.
Waiting for the repo graph service (8311 ms).
Loading the project graph of 440cffa5563d.
Planning 1 targets.
The graph service under the launcher was denied access; it is started again under the access of the calling app / Der Graphdienst unter dem Launcher wurde abgewiesen; er wird erneut mit dem Zugriff der aufrufenden App gestartet
Starting the repo graph service for this host.
Waiting for the repo graph service (4052 ms).
Loading the project graph of 440cffa5563d.
Planning 1 targets.
macOS denied the resident service access to your Documents folder while it answered a request (reference da146983). The service has stopped itself; it runs with the file access consent of the process that started it and not of `repo`. Switch on Documents Folder for `repo` (/Users/ueli/.local/share/semio/launcher/bin/repo) under System Settings, Privacy & Security, Files & Folders (or Full Disk Access), then run the command again, or keep the checkout outside Documents, Desktop and Downloads. Details: /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-c5f530f58f3a.log.
end 2026-11-15T03:48:28Z
native_rc=4
```

Graph service log `host-c5f530f58f3a.log`, both denials, verbatim:

```
[script] build:source-admission · Admitting the sources of the build
2026-11-15T03:48:22.978Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
    at failure (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148:321)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:153:768)
    at processTicksAndRejections (native:7:39)
2026-11-15T03:48:22.979Z shutdown access-denied
graph: plan answered, managed 121 MB, resident 453 MB (released 0 MB)
2026-11-15T03:48:28.804Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
    at failure (same stack)
2026-11-15T03:48:28.805Z shutdown access-denied
graph: plan answered, managed 121 MB, resident 434 MB (released 0 MB)
```

- The line `where: root=… entry=… code=…` that round 10 asks for is NOT in the log: `grep -c "where:"` on `host-c5f530f58f3a.log` gives 0. The error text has no path.
- My own process can read the worktree: `ls ~/Documents/semio` from the session that started the try succeeded right after it. So `native_rc=4` here does not mean "your own process cannot read the worktree either".
- What differs from the tries of rounds 9 to 16, which were never denied on this machine: this try came right after `repo tools setup all` (step 0 of round 16, exit code 1). In those rounds the service was reused or started without a denial; here a new service (`host-c5f530f58f3a.log`) was started and denied within 15 s, twice.
- A `repo service host` process from before is still running: pid 86375, parent 1, age 01:06:33 at 03:49Z, resident 67152 KiB, with a child pid 86380 (300240 KiB). The log `host-79a74804a430.log` was still being written at 03:49Z. I did not stop it.
- Launcher unchanged during the try: target `versions/cf25f2b751f83336/repo`, inode 3679486, `Identifier=tech.semio.repo`, `CDHash=6de7547a8b8ff384bbfbfc45e962462063ff0357`.
- Whether a consent dialog was on screen was not measured (I cannot see or answer it). The owner's action named by the text: switch on Documents Folder for `repo` under System Settings, Privacy & Security, Files & Folders.
- Pressure level 1 in all samples of the try.

## Before the Run (2026-11-15T03:47Z)

- Replica agent: `state = running`, `runs = 26`; started by me 0 times in this round.
- Replica code digest (`.cache/f/health.json`): `97445096cc0a4394ffbc45b5ae588ae86aea987d1329f6f4aa4f7779616437ea`
- `bootVersion`: 6
- Step 0 of round 16 (`repo tools setup all`) ran once before this series: exit code 1 after 1554 s, `Internal error: Complete acquired dependency retirement plan required` (details in `round-14.md`). Try 1 of this series started right after it, at 03:47Z.

## Try 1

- Log `/Users/ueli/native-bootstrap-round-17.log`: `start 2026-11-15T03:47:35Z`, `end 2026-11-15T03:48:28Z`, `native_rc=4`
- Runner: `native_rc=4 peak_rss_kib=2374656 run_peak_rss_kib=462864 stop=None fleet_refusal=False closure_missing=False nx_task_lines=0 error_lines=0 moving=False`
- P12 sampler peak: `2026-11-15T03:48:22Z pid=23539 rss=278MB footprint=[bun [23539]: 64-bit    Footprint: 90 MB (16384 bytes per page)] last=[graph: plan answered, managed 105 MB, resident 378 MB (released 0 MB)]`

Key lines of the graph service log, verbatim, in order:

```
2026-11-15T03:48:22.978Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
2026-11-15T03:48:28.804Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
```

Service failure (3 lines before, the line, the stack):

```
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
2026-11-15T03:48:22.978Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
    at failure (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148:321)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:153:768)
    at processTicksAndRejections (native:7:39)
2026-11-15T03:48:22.979Z shutdown access-denied
graph: plan answered, managed 121 MB, resident 453 MB (released 0 MB)
[script] {"owner":"repository:build","command":"repo","stage":"loading"}
[script] {"owner":"repository:build","command":"repo","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] build:source-admission · Admitting the sources of the build
[repo] still working after 10 s; last phase started: source.served
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
```

Service failure (3 lines before, the line, the stack):

```
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[repo] still working after 5 s; last phase started: source.served
2026-11-15T03:48:28.804Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
    at failure (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148:321)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:153:768)
    at processTicksAndRejections (native:7:39)
2026-11-15T03:48:28.805Z shutdown access-denied
graph: plan answered, managed 121 MB, resident 434 MB (released 0 MB)
repository:command failed with exit code 4
=== /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-79a74804a430.log from byte 215427
[repository:command] still running after 3911 s
[repo] still working after 3920 s; last phase started: source.served
[repository:command] still running after 3921 s
[repository:command] still running after 3931 s
```

P12 sampler `round-17.p12-try-1.txt` (2 samples with a slice worker):

```
service log: /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-79a74804a430.log
closure entries before: 42537
2026-11-15T03:47:42Z no slice worker
… (no footprint above 6000 MB)
2026-11-15T03:48:32Z no slice worker
closure entries after: 42537
```

nx task lines: 0

Last 40 lines of the bootstrap log (without `[script] {` rows):

```
[script] bun:dependency-storage remains active on dependencies:sync-gate:6912 (shared, 17 s)
[script] bun:dependency-storage remains active on dependencies:sync-gate:6944 (shared, 17 s)
[script] bun:dependency-storage remains active on dependencies:sync-gate:6976 (shared, 17 s)
[script] bun:dependency-storage remains active on dependencies:sync-gate:7008 (shared, 17 s)
[script] bun:dependency-storage remains active on dependencies:sync-gate:7040 (shared, 17 s)
[script] bun:dependency-storage remains active on dependencies:sync-gate:7072 (shared, 17 s)
[script] bun:dependency-storage remains active on dependencies:sync-gate:7104 (shared, 17 s)
[script] bun:dependency-storage remains active on dependencies:sync-gate:7136 (shared, 17 s)
[script] bun:dependency-storage remains active on dependencies:sync-gate:7168 (shared, 17 s)
[script] bun:dependency-storage remains active on dependencies:sync-gate:7200 (shared, 17 s)
[script] bun:dependency-storage remains active on dependencies:sync-gate:7232 (shared, 17 s)
[script] bun:dependency-storage remains active on dependencies:sync-gate:7264 (shared, 17 s)
[script] bun:dependency-storage remains active on dependencies:sync-gate:7296 (shared, 17 s)
[script] bun:dependency-storage remains active on dependencies:sync-gate:7328 (shared, 17 s)
[script] bun:dependency-storage remains active on dependencies:sync-gate:7360 (shared, 17 s)
[script] bun:dependency-storage remains active on dependencies:sync-gate:7392 (shared, 17 s)
[DATA] Store dependency sync current {"version":1,"purpose":"current-store-dependency-sync-current","scopes":6}
[semio] workspace:setup…
[capacity] run: sized at 95% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (8382 MB available, 2048 MB floor)
Starting the repo graph service for this host.
Waiting for the repo graph service (258 ms).
Waiting for the repo graph service (2305 ms).
Waiting for the repo graph service (4306 ms).
[repo] still working after 5 s; last phase started: door.script
Waiting for the repo graph service (6307 ms).
Waiting for the repo graph service (8311 ms).
Loading the project graph of 440cffa5563d.
Planning 1 targets.
[repo] still working after 10 s; last phase started: door.script
The graph service under the launcher was denied access; it is started again under the access of the calling app / Der Graphdienst unter dem Launcher wurde abgewiesen; er wird erneut mit dem Zugriff der aufrufenden App gestartet
Starting the repo graph service for this host.
Waiting for the repo graph service (1 ms).
Waiting for the repo graph service (2006 ms).
Waiting for the repo graph service (4052 ms).
Loading the project graph of 440cffa5563d.
Planning 1 targets.
macOS denied the resident service access to your Documents folder while it answered a request (reference da146983). The service has stopped itself; it runs with the file access consent of the process that started it and not of `repo`. Switch on Documents Folder for `repo` (/Users/ueli/.local/share/semio/launcher/bin/repo) under System Settings, Privacy & Security, Files & Folders (or Full Disk Access), then run the command again, or keep the checkout outside Documents, Desktop and Downloads. Details: /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-c5f530f58f3a.log.
end 2026-11-15T03:48:28Z
native_rc=4
```
