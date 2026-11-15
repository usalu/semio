# Round 8 — g1-m08 — Complete

**consent — not ready.** Two native tries returned native_rc=4 with a pending macOS Documents consent dialog. Try 1 installed the signed versioned launcher once. Try 2 left its inode, mtime, target and CDHash unchanged. The observer followed the consent polling rule: preflight every 60 seconds, all observed polls exit 0 but no access line. The first wait reached 30 minutes; the second was superseded by round 9 at 22:00:38 UTC, which explicitly stops the wait. No native bootstrap was still active then, so no SIGINT was necessary.

## Machine and replica

Memory 17179869184 bytes (16 GiB), macOS 27.2 build 26B123, arm64 Apple M4. Replica code digest before this round: `590fdef8b75853d124d6f615cd2612d607146e27aaf2b94715a5d7b7606ad2f1`; bootVersion 6; health age 2.953 seconds. Replica login job was running before and during the round. Observer replica kickstarts: 0; graph kickstarts: 0; gc: 0. No source, permission or launcher repair was made by the observer. Launcher installation was performed by the prescribed native bootstrap.

Free disk before:
```text
Filesystem   1024-blocks      Used Available Capacity iused      ifree %iused  Mounted on
/dev/disk3s5   239362496  84893644 116732324    43%  832298 1167323240    0%   /System/Volumes/Data
```

Free disk after:
```text
Filesystem   1024-blocks      Used Available Capacity iused      ifree %iused  Mounted on
/dev/disk3s5   239362496  84937012 116688956    43%  832470 1166889560    0%   /System/Volumes/Data
```

## Attempts and consent waits

| Try | Start UTC | Native controller end UTC | Native wall s | native_rc | Consent wait s | Polls | Wait ended because |
|---|---|---|---:|---:|---:|---:|---|
| 1 | 2026-11-14T21:00:19Z | 2026-11-14T21:03:52Z | 212.934 | 4 | 1800.227 | 30 | 30-minute bound |
| 2 | 2026-11-14T21:33:53Z | 2026-11-14T21:34:34Z | 40.612 | 4 | 1564.155 | 27 | coordinator superseded round |

Native wrapper exits were 0; authoritative native_rc was parsed from each anchored native log line. Completion marker absent; ready=false. Wanted, arrived, missing and fleet wait seconds: not measured for either try, because both stopped at the consent check before graph delivery. Succeeded setup tasks: none emitted; failed Nx tasks: none emitted; not-run task names: not measured (no task-result list was emitted). The consent result is not categorized as an unexpected task failure.

## Try 1

Launcher changed during this try: **TRUE**. Changed compared with preceding try final identity: **NONE** (None means no preceding try). Thus try 2 launcher was plainly UNCHANGED. Exact ls -li, readlink, inode, mtime and codesign outputs:

Before:
```json
{
  "at": "2026-11-14T21:00:19Z",
  "ls_li": "1256211 -rwxr-xr-x@ 1 ueli  staff  18657778 Nov 14 21:30 /Users/ueli/.local/bin/repo\n1256110 -rwxr-xr-x@ 1 ueli  staff   6561378 Nov 14 21:29 /Users/ueli/.local/share/semio/launcher/bin/repo",
  "paths": {
    "/Users/ueli/.local/share/semio/launcher/bin/repo": {
      "exists": true,
      "is_symlink": false,
      "inode": 1256110,
      "mtime_ns": 1794688199215580974,
      "mtime_utc": "2026-11-14T20:29:59.215581+00:00",
      "target_inode": 1256110,
      "target_mtime_ns": 1794688199215580974,
      "size": 6561378,
      "readlink": null
    },
    "/Users/ueli/.local/bin/repo": {
      "exists": true,
      "is_symlink": false,
      "inode": 1256211,
      "mtime_ns": 1794688211341948235,
      "mtime_utc": "2026-11-14T20:30:11.341948+00:00",
      "target_inode": 1256211,
      "target_mtime_ns": 1794688211341948235,
      "size": 18657778,
      "readlink": null
    }
  },
  "codesign_rc": 0,
  "codesign_lines": [
    "Identifier=a.out",
    "CandidateCDHash sha256=713fd0d142560c91e4fd5ed6c449b32909e667e7",
    "CandidateCDHashFull sha256=713fd0d142560c91e4fd5ed6c449b32909e667e754cba2ac1f50b10a6458608a",
    "CDHash=713fd0d142560c91e4fd5ed6c449b32909e667e7",
    "TeamIdentifier=not set"
  ]
}
```

After:
```json
{
  "at": "2026-11-14T21:03:52Z",
  "ls_li": "1256211 -rwxr-xr-x@ 1 ueli  staff  18657778 Nov 14 21:30 /Users/ueli/.local/bin/repo\n1262266 lrwxr-xr-x@ 1 ueli  staff        30 Nov 14 22:00 /Users/ueli/.local/share/semio/launcher/bin/repo -> versions/cf25f2b751f83336/repo",
  "paths": {
    "/Users/ueli/.local/share/semio/launcher/bin/repo": {
      "exists": true,
      "is_symlink": true,
      "inode": 1262266,
      "mtime_ns": 1794690020886970333,
      "mtime_utc": "2026-11-14T21:00:20.886970+00:00",
      "target_inode": 1262265,
      "target_mtime_ns": 1794690020871943656,
      "size": 6523360,
      "readlink": "versions/cf25f2b751f83336/repo"
    },
    "/Users/ueli/.local/bin/repo": {
      "exists": true,
      "is_symlink": false,
      "inode": 1256211,
      "mtime_ns": 1794688211341948235,
      "mtime_utc": "2026-11-14T20:30:11.341948+00:00",
      "target_inode": 1256211,
      "target_mtime_ns": 1794688211341948235,
      "size": 18657778,
      "readlink": null
    }
  },
  "codesign_rc": 0,
  "codesign_lines": [
    "Identifier=tech.semio.repo",
    "CandidateCDHash sha256=6de7547a8b8ff384bbfbfc45e962462063ff0357",
    "CandidateCDHashFull sha256=6de7547a8b8ff384bbfbfc45e962462063ff03579508eb85f9a018487b76872a",
    "CDHash=6de7547a8b8ff384bbfbfc45e962462063ff0357",
    "TeamIdentifier=not set"
  ]
}
```

Peak run-owned RSS 3286912 KiB; peak machine-wide sampled RSS 3285360 KiB; 8 samples at approximately 30 seconds from launch. PID, PPID, full commands and parent chains retained. All 43 pressure observations normal (1): True. No resource hard stop. Round-6 thresholds were used (RSS above 8 GiB, critical 4 twice 30 seconds apart, warning 2 six times).

Consent text with 15 preceding native lines, verbatim:
```text
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"dependencies:preservation-ownership:865152","mode":"exclusive","elapsedMs":0}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"dependencies:preservation-ownership:865216","mode":"exclusive","elapsedMs":0}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"dependencies:preservation-ownership:865280","mode":"exclusive","elapsedMs":0}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"dependencies:preservation-ownership:865344","mode":"exclusive","elapsedMs":0}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"dependencies:preservation-ownership:865408","mode":"exclusive","elapsedMs":0}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"dependencies:preservation-ownership:865472","mode":"exclusive","elapsedMs":0}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"dependencies:preservation-ownership:865536","mode":"exclusive","elapsedMs":0}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"dependencies:preservation-ownership:865600","mode":"exclusive","elapsedMs":0}}
[script] {"owner":"repository:build","command":"repo","stage":"complete"}
[semio] workspace:setup…
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7814 MB available, 2048 MB floor)
[script] {"owner":"repository:build","command":"repo","stage":"loading"}
[script] {"owner":"repository:build","command":"repo","stage":"running"}
Starting the repo graph service for this host.
A macOS dialog waits for your answer: "repo would like to access files in your Documents folder" (/Users/ueli/.local/share/semio/launcher/bin/repo). Click "Allow" in the session of the logged-in user (or switch on Documents Folder for `repo` under System Settings, Privacy & Security, Files & Folders), then run the command again. /Users/ueli/Documents/semio cannot be read before that.
```

Consent preflight polls: 30; all rc=0: True; explicit access-ok line observed: False. Full stdout/stderr and poll times in round-8-try-1-consent-preflight-polls.json. No inference was made from the silent zero exit that the new launcher had consent. The pre-existing ok service receipt belonged to old PID 14729 at 20:33 UTC, predating this launcher installation.

Observed phase wall (approximately five-second log polling):

| Phase | Start elapsed s | Wall until next phase/native observed end s |
|---|---:|---:|
| [semio] Building the repo launcher (it provisions the pinned JavaScript runtime itself)… | 0.002 | 5.097 |
| [semio] Installing the native repository command… | 5.099 | 15.35 |
| [semio] Installing the dependency scopes (the planner of every later run needs them)… | 20.449 | 182.282 |
| [semio] workspace:setup… | 202.731 | 10.203 |

Exact internal phase wall is not measured. Consent wait is measured separately above. Required native capacity/graph lines, verbatim:
```text
NATIVE
[semio] The launcher changed: macOS asks once more for the file access consent of the launcher (a running service keeps its file until it stops).
[capacity] run: sized at 5% of the default from 10 earlier run(s) of its class
[capacity] admitted run: 8 job(s), 1205 MB reserved (7880 MB available, 2048 MB floor)
[capacity] run: sized at 5% of the default from 10 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
  fleet status                                    Show one picture of the shared workspace: who runs what (phase, progress, age, priority, code freshness), the queues with waiting-time estimates, memory admitted against really used, the Bazel servers, storage against its budgets, the sealed implementation and the last graph generation.
  --json                                          Print lists as JSON instead of one entry per line. (used by: graph, show, incremental, concurrency, bench, capacity, fleet, scheduler, leases, cache, remote-cache, doctor, tools, conformance, units, service, session)
  --origin <id>                                   remote-execution workers: origin id of this machine in the fleet (default: the origin of the replica, else the host name); the scheduler never gives a machine the units it requested itself. (used by: remote-execution)
  --limit=<n>                                     fleet status: how many runs the text output lists (default: 20, the JSON output always lists all). (used by: fleet)
  repo fleet status
      Run the service with the shared store and one local worker: gRPC for build tools on 127.0.0.1:8980, HTTP for workers, status and the fleet load table on 127.0.0.1:8981; hermetic Bazel actions can then run remotely.
  fleet status                                    Show one picture of the shared workspace: who runs what (phase, progress, age, priority, code freshness), the queues with waiting-time estimates, memory admitted against really used, the Bazel servers, storage against its budgets, the sealed implementation and the last graph generation.
  --json                                          Print lists as JSON instead of one entry per line. (used by: graph, show, incremental, concurrency, bench, capacity, fleet, scheduler, leases, cache, remote-cache, doctor, tools, conformance, units, service, session)
  --origin <id>                                   remote-execution workers: origin id of this machine in the fleet (default: the origin of the replica, else the host name); the scheduler never gives a machine the units it requested itself. (used by: remote-execution)
  --limit=<n>                                     fleet status: how many runs the text output lists (default: 20, the JSON output always lists all). (used by: fleet)
  repo fleet status
      Run the service with the shared store and one local worker: gRPC for build tools on 127.0.0.1:8980, HTTP for workers, status and the fleet load table on 127.0.0.1:8981; hermetic Bazel actions can then run remotely.
[capacity] dependencies: sized at 25% of the default from 10 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 1510 MB reserved (7083 MB available, 2048 MB floor)
[capacity] dependencies: sized at 25% of the default from 10 earlier run(s) of its class
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:33","mode":"exclusive","elapsedMs":43.563624999998865}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":214.26341599999978}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.1573330000010174}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":70.28962499999943}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.0000419999996666}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":357.1922919999997}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.0262499999989814}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":64.06783400000131}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.2876669999968726}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":59.041333999994094}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.5822499999994761}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":45.46504100000311}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.7480409999989206}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":45.67016599999624}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":2.0235830000019632}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":56.45904200000223}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.9158339999994496}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":53.3056249999936}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":4.685375000000931}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":71.2084579999937}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.6169999999983702}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":46.47087499999907}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.6607080000103451}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":48.933958000008715}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.6436669999966398}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":40.37370799999917}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.6247089999960735}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":40.01975000000675}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":2.538249999997788}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":73.84220900000946}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.8470830000005662}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":48.111874999987776}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":0.6088329999765847}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":43.55383299998357}}
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7814 MB available, 2048 MB floor)

RESIDENT SERVICE
```

## Try 2

Launcher changed during this try: **FALSE**. Changed compared with preceding try final identity: **FALSE** (None means no preceding try). Thus try 2 launcher was plainly UNCHANGED. Exact ls -li, readlink, inode, mtime and codesign outputs:

Before:
```json
{
  "at": "2026-11-14T21:33:53Z",
  "ls_li": "1256211 -rwxr-xr-x@ 1 ueli  staff  18657778 Nov 14 21:30 /Users/ueli/.local/bin/repo\n1262266 lrwxr-xr-x@ 1 ueli  staff        30 Nov 14 22:00 /Users/ueli/.local/share/semio/launcher/bin/repo -> versions/cf25f2b751f83336/repo",
  "paths": {
    "/Users/ueli/.local/share/semio/launcher/bin/repo": {
      "exists": true,
      "is_symlink": true,
      "inode": 1262266,
      "mtime_ns": 1794690020886970333,
      "mtime_utc": "2026-11-14T21:00:20.886970+00:00",
      "target_inode": 1262265,
      "target_mtime_ns": 1794690020871943656,
      "size": 6523360,
      "readlink": "versions/cf25f2b751f83336/repo"
    },
    "/Users/ueli/.local/bin/repo": {
      "exists": true,
      "is_symlink": false,
      "inode": 1256211,
      "mtime_ns": 1794688211341948235,
      "mtime_utc": "2026-11-14T20:30:11.341948+00:00",
      "target_inode": 1256211,
      "target_mtime_ns": 1794688211341948235,
      "size": 18657778,
      "readlink": null
    }
  },
  "codesign_rc": 0,
  "codesign_lines": [
    "Identifier=tech.semio.repo",
    "CandidateCDHash sha256=6de7547a8b8ff384bbfbfc45e962462063ff0357",
    "CandidateCDHashFull sha256=6de7547a8b8ff384bbfbfc45e962462063ff03579508eb85f9a018487b76872a",
    "CDHash=6de7547a8b8ff384bbfbfc45e962462063ff0357",
    "TeamIdentifier=not set"
  ]
}
```

After:
```json
{
  "at": "2026-11-14T21:34:34Z",
  "ls_li": "1256211 -rwxr-xr-x@ 1 ueli  staff  18657778 Nov 14 21:30 /Users/ueli/.local/bin/repo\n1262266 lrwxr-xr-x@ 1 ueli  staff        30 Nov 14 22:00 /Users/ueli/.local/share/semio/launcher/bin/repo -> versions/cf25f2b751f83336/repo",
  "paths": {
    "/Users/ueli/.local/share/semio/launcher/bin/repo": {
      "exists": true,
      "is_symlink": true,
      "inode": 1262266,
      "mtime_ns": 1794690020886970333,
      "mtime_utc": "2026-11-14T21:00:20.886970+00:00",
      "target_inode": 1262265,
      "target_mtime_ns": 1794690020871943656,
      "size": 6523360,
      "readlink": "versions/cf25f2b751f83336/repo"
    },
    "/Users/ueli/.local/bin/repo": {
      "exists": true,
      "is_symlink": false,
      "inode": 1256211,
      "mtime_ns": 1794688211341948235,
      "mtime_utc": "2026-11-14T20:30:11.341948+00:00",
      "target_inode": 1256211,
      "target_mtime_ns": 1794688211341948235,
      "size": 18657778,
      "readlink": null
    }
  },
  "codesign_rc": 0,
  "codesign_lines": [
    "Identifier=tech.semio.repo",
    "CandidateCDHash sha256=6de7547a8b8ff384bbfbfc45e962462063ff0357",
    "CandidateCDHashFull sha256=6de7547a8b8ff384bbfbfc45e962462063ff03579508eb85f9a018487b76872a",
    "CDHash=6de7547a8b8ff384bbfbfc45e962462063ff0357",
    "TeamIdentifier=not set"
  ]
}
```

Peak run-owned RSS 846384 KiB; peak machine-wide sampled RSS 1497152 KiB; 2 samples at approximately 30 seconds from launch. PID, PPID, full commands and parent chains retained. All 9 pressure observations normal (1): True. No resource hard stop. Round-6 thresholds were used (RSS above 8 GiB, critical 4 twice 30 seconds apart, warning 2 six times).

Consent text with 15 preceding native lines, verbatim:
```text
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"dependencies:sync-gate:7200","mode":"shared","elapsedMs":13916.64325}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"dependencies:sync-gate:7232","mode":"shared","elapsedMs":13917.463292}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"dependencies:sync-gate:7264","mode":"shared","elapsedMs":13917.957957999999}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"dependencies:sync-gate:7296","mode":"shared","elapsedMs":13918.49675}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"dependencies:sync-gate:7328","mode":"shared","elapsedMs":13918.98925}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"dependencies:sync-gate:7360","mode":"shared","elapsedMs":13919.467999999999}}
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"dependencies:sync-gate:7392","mode":"shared","elapsedMs":13919.976417}}
[DATA] Store dependency sync current {"version":1,"purpose":"current-store-dependency-sync-current","scopes":6}
[script] {"owner":"repository:build","command":"repo","stage":"complete"}
[semio] workspace:setup…
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6572 MB available, 2048 MB floor)
[script] {"owner":"repository:build","command":"repo","stage":"loading"}
[script] {"owner":"repository:build","command":"repo","stage":"running"}
Starting the repo graph service for this host.
A macOS dialog waits for your answer: "repo would like to access files in your Documents folder" (/Users/ueli/.local/share/semio/launcher/bin/repo). Click "Allow" in the session of the logged-in user (or switch on Documents Folder for `repo` under System Settings, Privacy & Security, Files & Folders), then run the command again. /Users/ueli/Documents/semio cannot be read before that.
```

Consent preflight polls: 27; all rc=0: True; explicit access-ok line observed: False. Full stdout/stderr and poll times in round-8-try-2-consent-preflight-polls.json. No inference was made from the silent zero exit that the new launcher had consent. The pre-existing ok service receipt belonged to old PID 14729 at 20:33 UTC, predating this launcher installation.

Observed phase wall (approximately five-second log polling):

| Phase | Start elapsed s | Wall until next phase/native observed end s |
|---|---:|---:|
| [semio] Building the repo launcher (it provisions the pinned JavaScript runtime itself)… | 0.005 | 5.09 |
| [semio] Installing the native repository command… | 5.095 | 10.098 |
| [semio] Installing the dependency scopes (the planner of every later run needs them)… | 15.193 | 15.15 |
| [semio] workspace:setup… | 30.343 | 10.269 |

Exact internal phase wall is not measured. Consent wait is measured separately above. Required native capacity/graph lines, verbatim:
```text
NATIVE
[capacity] run: sized at 5% of the default from 11 earlier run(s) of its class
[capacity] admitted run: 8 job(s), 1205 MB reserved (7951 MB available, 2048 MB floor)
[capacity] run: sized at 5% of the default from 11 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
  fleet status                                    Show one picture of the shared workspace: who runs what (phase, progress, age, priority, code freshness), the queues with waiting-time estimates, memory admitted against really used, the Bazel servers, storage against its budgets, the sealed implementation and the last graph generation.
  --json                                          Print lists as JSON instead of one entry per line. (used by: graph, show, incremental, concurrency, bench, capacity, fleet, scheduler, leases, cache, remote-cache, doctor, tools, conformance, units, service, session)
  --origin <id>                                   remote-execution workers: origin id of this machine in the fleet (default: the origin of the replica, else the host name); the scheduler never gives a machine the units it requested itself. (used by: remote-execution)
  --limit=<n>                                     fleet status: how many runs the text output lists (default: 20, the JSON output always lists all). (used by: fleet)
  repo fleet status
      Run the service with the shared store and one local worker: gRPC for build tools on 127.0.0.1:8980, HTTP for workers, status and the fleet load table on 127.0.0.1:8981; hermetic Bazel actions can then run remotely.
  fleet status                                    Show one picture of the shared workspace: who runs what (phase, progress, age, priority, code freshness), the queues with waiting-time estimates, memory admitted against really used, the Bazel servers, storage against its budgets, the sealed implementation and the last graph generation.
  --json                                          Print lists as JSON instead of one entry per line. (used by: graph, show, incremental, concurrency, bench, capacity, fleet, scheduler, leases, cache, remote-cache, doctor, tools, conformance, units, service, session)
  --origin <id>                                   remote-execution workers: origin id of this machine in the fleet (default: the origin of the replica, else the host name); the scheduler never gives a machine the units it requested itself. (used by: remote-execution)
  --limit=<n>                                     fleet status: how many runs the text output lists (default: 20, the JSON output always lists all). (used by: fleet)
  repo fleet status
      Run the service with the shared store and one local worker: gRPC for build tools on 127.0.0.1:8980, HTTP for workers, status and the fleet load table on 127.0.0.1:8981; hermetic Bazel actions can then run remotely.
[capacity] dependencies: sized at 25% of the default from 11 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 1510 MB reserved (7945 MB available, 2048 MB floor)
[capacity] dependencies: sized at 25% of the default from 11 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 1 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6572 MB available, 2048 MB floor)

RESIDENT SERVICE
```

## Replica status after round

Supported status command exited 0:
```text
Replica g1-m08 of group g1, store /Volumes/semio-store: reachable
Synchronization: on
Daemon: running (pid 29075)
281681 files tracked; queued: 10 incoming, ? outgoing; 26 overwritten files kept in .cache/f/lost
Sidecar build-agent: running (process 29400, 0 restart(s), 4093 s in this state, last exit -).
Sidecar build-scheduler: off on this machine (process -, 0 restart(s), 4125 s in this state, last exit -).
Sidecar build-cache: off on this machine (process -, 0 restart(s), 4125 s in this state, last exit -).
Journal: 10.21 MiB in 13 segments of 9 origins.
Newest snapshot by g1-server: 36588s old, 89.18 MiB.
Reclaimable now: 2 journal segments (2.29 MiB).
Last compaction 46051s ago: 861 blobs (27.00 GiB) removed; 88 blobs (129.23 MiB) were still too young.
Git state: shared (stage and commits replicate); stage tree 5c75c44d73137807925a4f37dfbb0a95529a546e.
Background tiers, last pass (cost, duty cycle of one core): hot 2 ms 0.8%, directory sweep 1531 ms 10%, file sweep 1491 ms 1.3%, tree walk 23700 ms 1.2%, git view 24899 ms 5%.
Latency over the last 94 publishes and 100 applies: publish p50 252 ms, p95 1512 ms; apply p50 322 ms, p95 677 ms.
g1-m01: 0 events (0 B) behind, last event 24s ago
g1-m02: 0 events (0 B) behind, last event 37s ago
g1-m03: 0 events (0 B) behind, last event 32s ago
g1-m04: 0 events (0 B) behind, last event 22s ago
g1-m05: 0 events (0 B) behind, last event 12176s ago
g1-m06: 0 events (0 B) behind, last event 223s ago
g1-m07: 1 events (395 B) behind, last event 5s ago
g1-m08: 0 events (0 B) behind, last event 78s ago
g1-server: 0 events (0 B) behind, last event 84s ago
Fleet: origin platform age lag (origins behind/queued) basis (cp:s = compared at its checkpoint taken s seconds ago, now = compared as it is now) sync code view (modified/deleted/untracked) latency (publish p50/p95 ms | apply p50/p95 ms) verdict
  g1-m01 macos 0s 0/0 cp:0s on 590fdef8b758 137/12/644 179/265|223/637 ok
  g1-m02 macos 3s 0/0 cp:3s on 590fdef8b758 137/12/644 217/622|272/635 ok
  g1-m03 macos 1s 0/0 cp:1s on 590fdef8b758 137/12/644 179/456|268/580 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m04 linux 3s 0/0 cp:3s on 590fdef8b758 137/11/644 174/435|329/853 diverged
  g1-m05 windows 3s 0/4 cp:- on 6996465a9065 141/12/644 -/-|2668/18865 lagging (not settled: no common checkpoint within 60 s yet, still moving)
  g1-m06 macos 0s 0/0 cp:0s on 590fdef8b758 137/12/644 210/24929|207/537 ok
  g1-m07 macos 1s 0/0 cp:1s on 590fdef8b758 137/12/644 249/838|234/959 ok
  g1-m08 macos 0s 0/0 cp:0s on 590fdef8b758 137/12/644 252/1512|322/677 ok
  g1-server macos 0s 0/0 cp:0s on 590fdef8b758 137/12/644 222/104311|179/556 ok
  warning: g1-m04 runs git 2.55.0, other machines run another version
  warning: the volume of g1-m04 behaves differently from most (case=sensitive normalisation=sensitive); the view is pinned, so this is a capability difference, not a divergence
  warning: g1-m06 runs git 2.50.1, other machines run another version
  g1-m04 differs in view.digest at a common checkpoint: #16=1e8a,#52=6fde,#86=6490,#114=a597,#121=b976,#147=9704,#163=3887,#188=ebd7,#21 instead of #16=321b,#52=44e9,#86=cf91,#114=6004,#121=09af,#147=4540,#163=e9ed,#188=6798,#21 (#n = bucket n of 256 of the paths, value = its digest)
The fleet is not in agreement (9 origins; fresh and checkpoint age mean within 60 s; lagging is not wrong, only not yet comparable).
```

## Completion

Exact last 200 native lines for the final try: round-8.tail.txt; per-try tails and process/resource/launcher evidence are retained. Native completion marker absent. State records native_rc=4 and failed=[consent] as required. The observer supervisor ended normally on supersession; its ready-stop guard ended without sending any signal. Proceeding immediately to round 9, which removes the consent wait.

Report completed 2026-11-14T22:02:02Z.
