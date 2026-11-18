# stopped at a wall: stage workspace, phase -

- slice of 1 owners requested, the read admitted ?
- stages replaced by `repo --help` (not exercised): native, repo-mcp, dependencies, tools
- peak footprint 940 MB against the limit of 6553 MB
- owners inferred more than once across machines: 0
- leftover processes 0, leftover attached disk images 0
- load average at start 38.75
- lab processes that ran below normal priority (nice other than 0): 0
- parent event-loop lag, maximum - ms (limit 200 ms)
- processes of the machines busy in 40 percent of the samples
- seconds per activity: {"-":4,"build:source-admission":6,"build:implementation-source":14}
- selected graph publications sealed on the machines: 0
- owners of the read per machine (maximum seen: present / fetched from the fleet / inferred here of total): {}
- seconds each machine took from its own start to ready: {}
- disk images the lab detached after the machines were stopped: 2
- files of the frozen view that changed during the run (must be 0): 0
- frozen view: 167696 files (1839 MB logical, cloned in 339 s without copying bytes where the file system clones); 0 files of the live tree changed since the freeze and stay invisible to the machines
- not exercised by a selected read yet: assembly, validation, seal and publish of a slice generation, and a target run on the slice
- files below the live state of the checkout that the machines held open: 0
- seconds per stage: {"native":1,"repo-mcp":0,"dependencies":0,"tools":0,"workspace":22}
- seconds per phase: {}

## slice-lab-34ec-m1: workspace / - (stage-failed)

process: workspace: repo (exit 1)

```
[script] {"owner":"repository:build","command":"repo","stage":"loading"}
[script] {"owner":"repository:build","command":"repo","stage":"running"}
Starting the repo graph service for this host.
Waiting for the repo graph service (21 ms).
Waiting for the repo graph service (2077 ms).
[repo] still working after 5 s; last phase started: door.script
Waiting for the repo graph service (4125 ms).
Waiting for the repo graph service (6174 ms).
Waiting for the repo graph service (8216 ms).
[repo] still working after 10 s; last phase started: door.script
Waiting for the repo graph service (10218 ms).
Waiting for the repo graph service (12235 ms).
Waiting for the repo graph service (14276 ms).
Waiting for the repo graph service (16278 ms).
Waiting for the repo graph service (18313 ms).
[repo] still working after 20 s; last phase started: door.script
Waiting for the repo graph service (20360 ms).
The graph service could not answer (lost the service was killed without an exit record (its cgroup is gone) [log: [script] build:implementation-source · Reading the sources of the build implementation | Internal error: Original offline Rustup bootstrap absent | repository:command failed with exit co
```

| t (s) | machine | stage | state | phase | done | total | owners | procs | busy | footprint MB | cpu % | loop lag ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 2.2 | slice-lab-34ec-m1 | tools | running | - | - | - | {} | 1 | 1 | 43 | 2.7 | - |
| 4.4 | slice-lab-34ec-m1 | workspace | running | - | - | - | {} | 4 | 2 | 218 | 40.1 | - |
| 6.4 | slice-lab-34ec-m1 | workspace | running | - | - | - | {} | 4 | 1 | 283 | 79 | - |
| 8.5 | slice-lab-34ec-m1 | workspace | running | - | - | - | {} | 4 | 2 | 475 | 78.5 | - |
| 10.9 | slice-lab-34ec-m1 | workspace | running | - | - | - | {} | 5 | 3 | 711 | 99.6 | - |
| 12.9 | slice-lab-34ec-m1 | workspace | running | - | - | - | {} | 5 | 2 | 934 | 15.2 | - |
| 15.2 | slice-lab-34ec-m1 | workspace | running | - | - | - | {} | 5 | 1 | 934 | 1.5 | - |
| 17.4 | slice-lab-34ec-m1 | workspace | running | - | - | - | {} | 5 | 2 | 939 | 13.7 | - |
| 19.5 | slice-lab-34ec-m1 | workspace | running | - | - | - | {} | 5 | 1 | 940 | 2.4 | - |
| 21.5 | slice-lab-34ec-m1 | workspace | running | - | - | - | {} | 5 | 0 | 940 | 1.8 | - |
| 23.7 | slice-lab-34ec-m1 | workspace | running | - | - | - | {} | 3 | 1 | 157 | 1.3 | - |
| 26.2 | slice-lab-34ec-m1 | workspace | backoff | - | - | - | {} | 0 | 0 | 0 | 0 | - |

## home

- tools prepared for this slice: 
- paths outside the lab seam that changed during the run: 10: /Users/ueli/Documents/semio/.cache/f, /Users/ueli/Documents/semio/.cache/f/boot, /Users/ueli/Documents/semio/.cache/f/cache-facts.json, /Users/ueli/Documents/semio/.cache/f/daemon.lock, /Users/ueli/Documents/semio/.cache/f/health.json, /Users/ueli/Documents/semio/.cache/f/mirror-state.json, /Users/ueli/Documents/semio/.cache/f/owner-pool-children, /Users/ueli/Documents/semio/.cache/f/sidecars, /Users/ueli/Documents/semio/.cache/f/state.wal, /Users/ueli/Documents/semio/.cache/f/tmp
- downloads the stages tried and the lab refused: 0

| seed | method | seconds | machine |
| --- | --- | --- | --- |
| launcher | clonefile | 0 | slice-lab-34ec-m1 |
| runtimes | clonefile | 0 | slice-lab-34ec-m1 |

slice-lab-34ec-m1

| stage | seconds | verdict |
| --- | --- | --- |
| native | 1 | ok |
| repo-mcp | 0 | ok |
| dependencies | 0 | ok |
| tools | 0 | ok |
| workspace | 22 | failed (stage-failed, exit 1) |

