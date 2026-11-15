# g1-m04 Round 12 (pause, read-only; taken 2026-11-15T01:29:52Z)

No bootstrap was run. Last `native_rc`: 75 (round 10, try 10, ended 01:29:06Z).

**This machine is not in the state the round describes.** It never reaches its own graph read with a plan step of 7 to 18 GB: it finds 517 stale projects, calls all of them too heavy, asks the fleet and receives nothing (46 tries, 0 arrived). The reason I read in the code is in `BLOCKER.md` of this folder: requests are stored and read per platform, and this is the Linux worker.

## 1. `graph:` Lines of the Last Try (`/run/user/1000/semio-repo/1000/host-190b5b27062b.log`, repeated lines once)

```
graph: No verified change journal cursor is recorded for generation 389; verifying the tree once.
graph: 517 stale projects, 0 taken from the fleet, 517 too heavy for this machine (@semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 511 more; it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 517
graph: nothing to wait for any longer: the fleet was asked for 517 stale projects and delivered 0 within 300 s; 517 remain, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 511 more, which this machine must never infer itself (it can give about 0 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: nothing to wait for any longer: the fleet was asked for 517 stale projects and delivered 0 within 301 s; 517 remain, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 511 more, which this machine must never infer itself (it can give about 0 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: nothing to wait for any longer: the fleet was asked for 517 stale projects and delivered 0 within 302 s; 517 remain, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 511 more, which this machine must never infer itself (it can give about 0 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
```

All distinct `graph:` lines of that log, numbers folded (431 lines in total):

```
graph: No verified change journal cursor is recorded for generation 389; verifying the tree once.
graph: 517 stale projects, 0 taken from the fleet, 517 too heavy for this machine (@semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 511 more; it can give about 0 MB per project), it can infer 0 o
graph: waiting for the fleet: 0 of 517 projects arrived
graph: nothing to wait for any longer: the fleet was asked for 517 stale projects and delivered 0 within 300 s; 517 remain, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 
graph: the replica manifest cannot vouch for the tree (the replica daemon is not healthy); verifying the tree once.
```

No line names a generation number or `restored`.

## 2. Newest Three `host-*.log`, Replica Digest

```
-rw------- 1 deck deck 1064726 2026-11-15 02:29:45.866945056 +0100 /run/user/1000/semio-repo/1000/host-190b5b27062b.log
-rw------- 1 deck deck  472253 2026-11-15 00:11:43.217366582 +0100 /run/user/1000/semio-repo/1000/host-191b99617bb4.log
-rw------- 1 deck deck  266175 2026-11-14 23:05:05.910165056 +0100 /run/user/1000/semio-repo/1000/host-81f15d8fbbe2.log
replica code digest: "code":"eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc"
"code":"52e46eee5c577cf8425b43f243681f24326d6645807f938982368d8c64cbfc7f"
```

## 3. Machine State

```
PSI memory: some avg10=0.06 avg60=0.15 avg300=0.08 total=25786586446 full avg10=0.06 avg60=0.15 avg300=0.08 total=21096064292 
               total        used        free      shared  buff/cache   available
Mem:           15848        5861        3269          26        7090        9987
Swap:          25331       10340       14991
five largest processes (rss KiB, pid, ppid, age, command):
783652 2935472 2935455   02:20:36 /home/deck/.cache/semio/runtimes/bun/c678040f14fe0440eb839d37cbd0ce4c051a32da72806ac97de6a6aab6bf728f/bun /home/deck
589304 119149  119113  1-01:59:34 /home/deck/.local/bin/claude --permission-mode auto
247004   2156    1977 16-01:57:11 /usr/lib/kf6/baloo_file
223928 2937733 2937655   02:19:26 /home/deck/.cache/semio/runtimes/bun/c678040f14fe0440eb839d37cbd0ce4c051a32da72806ac97de6a6aab6bf728f/bun --eval  co
142712 2937655   1977    02:19:29 /home/deck/.cache/semio/runtimes/bun/c678040f14fe0440eb839d37cbd0ce4c051a32da72806ac97de6a6aab6bf728f/bun /home/deck
repo service host / repo show projects processes (pid, ppid, age, rss KiB):
2937655    1977    02:19:29 142712 repo service host run
(end of list; nothing was stopped)
```

Waiting for round 13.
