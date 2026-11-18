# g1-m08 Round 30: Before the Run

Written 2026-11-15T17:55Z by a new agent session on this machine (ticket `26/11/15/SET-UP-MAC-MINI-05-AS-FLEET-WORKER-G1-M08-AND-COLLABORATE-WITH-THE-COORDINATOR`).

## Why Nothing Arrived Since Round 18

- The machine restarted at 2026-11-15T04:26:10Z (`kern.boottime`), 13 minutes after the last report (round 18, try 1, `native_rc=75`, 04:13:44Z). The round 18 observer and its session ended with it. No run of rounds 19 to 29 exists for this machine.
- `next.md` still says `round: 27`; `ROUND.md` says `round: 30`. I follow round 30 and label the files `round-30`.

## Machine (measured 17:50Z to 17:55Z)

- Mac mini, 16 GB (`hw.memsize` 17179869184), macOS 27.2.1 (Darwin 27.2.0, arm64), 120 GiB free of 228 GiB, memory pressure level 1, load average 13.98.
- Replica `status` (exit 0, 33 s): synchronization on, daemon running (pid 36092, started 17:48Z), store reachable, 298956 files tracked, 10 incoming queued, outgoing not measured (`?`), 27 overwritten files kept in `.cache/f/lost`. Code `b4c6513b87ef` (health file `b4c6513b87ef56c0cbaf790613fc13a7ac00f8881f15c91f2660655312a85d3a`), the same as g1-m02, g1-m04, g1-m07. Verdict of g1-m08: `lagging (not settled: no common checkpoint within 60 s yet, still moving)`, 0 events behind itself, at most 11 events behind another origin. `The manifest is not verified against the tree yet`.
- Sidecars: `build-agent` running (process 36506, 0 restarts); `build-scheduler` and `build-cache` off on this machine.
- Git state: `shared`, stage tree `bb58893207bab4930c7772b9e165f8a862e67a27`.
- Marker `.cache/f/native-bootstrap.done`: absent.
- Step 0 (`repo tools setup all`): exit 0 in round 16 (1924 s); not run again by me.

## What I Found Running, and What I Do

The automatic ready runner (sidecar role `ready`, child 36510 of `build-agent`) holds `.cache/f/ready/lease.json` and is working through the five stages by itself:

- `native` done 17:42:04Z, `repo-mcp` done 17:49:33Z, `dependencies` running since 17:49Z (stage 3 of 5), `attempts` 0, `failure` null, peak 1773 MB (`.cache/f/ready/state.json`, `<store>/v1/origins/g1-m08/ready.json`).
- An earlier `tools` log of the runner (17:48Z, before the daemon restarted for new code) ends with: `Cannot find package '@semio-tech/process' imported from …/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup` and `The stamps moved again after the restart: run the command again.`

Round 30 says to run `W/native-bootstrap.sh`. That script runs the same stages the runner is in the middle of, and the protocol allows one graph command at a time. I therefore do NOT start it while the runner makes progress: I observe the runner (read-only: `round-30-observe.sh`, 30 s samples of its status row, pressure and the three largest processes, 5 s samples of the slice worker) and report what it does. I start `W/native-bootstrap.sh` by hand only if the runner reports a failed stage (`backoff`) or stands still.

Coordinator: if you want the manual run in any case, write it into `workers/g1-m08/next.md`; I read it every round.
