# Round 8 on g1-m07

## Machine
- Mac mini, 16 GiB memory, macOS 27.2.1 (Darwin 27.2.0), 108 GiB free of 228 GiB on the data volume (21:55Z).

## Step 1: replica agent
- `tech.semio.fleet.replica`: `state = running`, `runs = 47`, pid 19165, `last exit code = 0` at 21:55Z. Kickstarted 0 times. `.cache/f/health.json` fresh (written 21:55:08Z).
- Replica code digest before try 1: `d72ff30ebb826bfabac6ee5f126041ee5f833672897443b9666c557ce85335c1`; now: "code":"eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc". `bootVersion` 6.

## Launcher (steps 2 and 5), full listings in `round-8.launcher.txt`
| when | launcher path | target inode | target mtime (local, CET) | Identifier | CDHash |
|---|---|---|---|---|---|
| before try 1 (21:55:50Z) | regular file, inode 3490754 | 3490754 | 2026-11-14T21:11:35 | a.out | 713fd0d142560c91e4fd5ed6c449b32909e667e7 |
| after try 1 | link (inode 3496899) -> `versions/cf25f2b751f83336/repo` | 3496898 | 2026-11-14T22:55:52 | tech.semio.repo | 6de7547a8b8ff384bbfbfc45e962462063ff0357 |
| before/after try 2 | same link | 3496898 | 2026-11-14T22:55:52 | tech.semio.repo | 6de7547a8b8ff384bbfbfc45e962462063ff0357 |
| before/after try 3 | same link | 3496898 | 2026-11-14T22:55:52 | tech.semio.repo | 6de7547a8b8ff384bbfbfc45e962462063ff0357 |

- Plainly: try 1 installed the new signed launcher once (log: `The launcher changed: macOS asks once more …`). Tries 2 and 3 did NOT change it: inode, mtime and CDHash are identical (log: `The repo launcher is current, nothing was replaced`).
- `~/.local/bin/repo` unchanged throughout (inode 3490935, 18657778 bytes, Nov 14 21:11).

## Consent
- No try ended with `native_rc=4`. Instead try 1 printed, verbatim: `The graph service runs under the access of the calling app because repo has no consent for Documents yet; click Allow on the dialog "repo would like to access files in your Documents folder" to make it independent`.
- The owner clicked "Allow" before round 8 (for the old launcher, about 21:54Z). Whether a dialog for the new launcher was shown or answered: not measured (nobody was asked at the screen after 21:55Z).
- `repo preflight --lang en` now (exit 0), lines naming access or consent: `none`
- Receipts now:
```
{"version":1,"role":"client","pid":51051,"state":"ok","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794697061795}{"version":1,"role":"launcher","pid":63553,"state":"waiting","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794699970089,"launcherSha256":"cf25f2b751f833367b632787eddfd0a7179ead0122c51f8f4f65fd355e66a482"}{"version":1,"role":"service","pid":27826,"state":"ok","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794693764799}
```

## Tries (3 of 10, no hard stop, pressure level 1 in all 211 samples)
| try | log | start | end | native_rc | peak resident of the run |
|---|---|---|---|---|---|
| 1 | `~/native-bootstrap-round-8.log` | 21:55:50Z | 22:22:27Z | 75 | 4944704 KiB |
| 2 | `~/native-bootstrap-round-8-try-2.log` | 22:24:34Z | 22:55:05Z | 75 | 3222880 KiB |
| 3 | `~/native-bootstrap-round-8-try-3.log` | 22:57:16Z | 23:45:29Z | 1 | 735088 KiB (largest process on the machine 2452672 KiB, not of the run) |

Phases launcher, rust toolchain, native repo command and dependency scopes passed in every try; wall time per phase not measured. All three failed in `workspace:setup` before any task ran: succeeded tasks 0, failed tasks 0, not run: all.

### `[capacity]` lines, verbatim (try 1; tries 2 and 3 differ only in the available MB: 7283/7423/4215 and 9271/8603/8621)
```
[capacity] run: sized at 7% of the default from 15 earlier run(s) of its class, at least 4093 MB because of its plan phase
[capacity] admitted run: 8 job(s), 5073 MB reserved (8286 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 23% of the default from 15 earlier run(s) of its class, at least 4093 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 4093 MB reserved (7384 MB available, 2048 MB floor)
[capacity] run: sized at 102% of the default from 3 earlier run(s) of its class
[capacity] waiting 0s for run: not enough free memory (need 2048 MB; 704 MB spare above the 2048 MB floor, load 9.85; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted run: 1 job(s), 2048 MB reserved (7932 MB available, 2048 MB floor)
```

### Try 1: fleet refusal
```
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 658 and delivered 159 within 1015 s, 499 are still missing, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), workspace (about 6200 MB) (this machine can give about 3512 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```
### Try 2: fleet refusal
```
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 499 and delivered 95 within 1487 s, 404 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```
### Try 3: service failure after 48 minutes without progress
Client: `The resident service met an unexpected failure (reference 5c4943e4); try again, …`. The progress line stayed at `Reading the project graph slice (N s).` with no detail for 2868 s. Service log `host-9888631afea4.log` (service pid 27815, started 21:56Z, not via launchd):
```
This process (repository:build) made no progress for 48 min and has no live child: it exits now so that the leases it holds are released.
2026-11-14T23:45:29.378Z request-failed 5c4943e4 Error: Slice worker exited with code 124
    at <anonymous> (semio-source:…/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs:167:38)
```
No fleet line was printed in try 3.

### `graph:` lines of the service log, verbatim (cut at 400 characters)
```
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
graph: 658 stale projects, 0 taken from the fleet, 6 too heavy for this machine (@semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 8452 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), @semio-tech/framework-plugin (about 4018 MB); it can give about 3512 MB per project), it can infer 192 of the others (
graph: waiting for the fleet: 0 of 658 projects arrived
graph: waiting for the fleet: 159 of 658 projects arrived
graph: nothing to wait for any longer: the fleet was asked for 658 stale projects and delivered 159 within 1015 s; 499 remain, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), workspace (about 6200 MB), which this machine must never infer itself (it can give about 3512 MB per project); try again when a machine with free memor
graph: 658 stale projects, 159 taken from the fleet, 2 too heavy for this machine (@semio-tech/repo-lib (about 27770 MB), workspace (about 6200 MB); it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 499
graph: waiting for the fleet: 0 of 499 projects arrived
graph: waiting for the fleet: 47 of 499 projects arrived
graph: waiting for the fleet: 95 of 499 projects arrived
graph: nothing to wait for any longer: asked the fleet for 499 stale projects and received 95 within 1487 s, 404 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
```

## End of the round
- Not ready: marker `.cache/f/native-bootstrap.done` absent. Tries 4 to 10 not run: `ROUND.md` showed `round: 10` when try 3 ended (23:45:55Z), so this report closes round 8 and the newer round starts. No `round-9.md` exists on this machine: round 9 was never seen here.
