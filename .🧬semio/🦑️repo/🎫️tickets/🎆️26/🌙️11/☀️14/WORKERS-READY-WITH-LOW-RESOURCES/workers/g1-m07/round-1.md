# g1-m07 Round 1

round: 1

## Machine
- Mac mini (`ueli-workstation-mac-mini-m4-16gb-256gb-02`), macOS 27.2.1 (26B131), 16 GB memory (17179869184 bytes), 125 GiB free of 228 GiB on the data volume before the run.
- Replica daemon code digest (`.cache/f/health.json` after the run): `3d9ead73396cf6b15cf5162c44f2d7e91bc5f3441076fd31219273c78cc1aa77`, store `ok`.

## Run
- Command: `W/native-bootstrap.sh "$HOME/native-bootstrap-round-1.log"`, once.
- Start 2026-11-14T10:49:56Z, end 2026-11-14T11:07:08Z (17 min 12 s).
- `native_rc=1`. `.cache/f/native-bootstrap.done` absent.
- Hard stop: none met. No `graph: refreshing N projects` and no `Inferring projects` line in the log; largest process 3205312 KiB resident; `kern.memorystatus_vm_pressure_level` was 1 in all 35 samples.

## Failure
No nx task ran: the run ended while `repo run workspace:setup` was reading the project graph slice. First error line verbatim, with the 15 lines before it:

```
Reading the project graph slice (250 s): flush 1 of 1.
[repo] still working after 260 s; last phase started: door.script
Reading the project graph slice (252 s): flush 1 of 1.
Reading the project graph slice (254 s): flush 1 of 1.
Reading the project graph slice (256 s): flush 1 of 1.
Reading the project graph slice (258 s): flush 1 of 1.
Reading the project graph slice (260 s): flush 1 of 1.
Reading the project graph slice (262 s): flush 1 of 1.
Reading the project graph slice (264 s): flush 1 of 1.
Reading the project graph slice (266 s): flush 1 of 1.
Reading the project graph slice (268 s): flush 1 of 1.
Reading the project graph slice (270 s): flush 1 of 1.
Reading the project graph slice (272 s): flush 1 of 1.
Reading the project graph slice (274 s): flush 1 of 1.
Reading the project graph slice (276 s): flush 1 of 1.
The resident service met an unexpected failure (reference a5924299); try again, the details under that reference are in the service log /var/folders/xq/z6t8hmcd0qj871bx0sj7pbn80000gn/T/semio-repo/501/host-916ec84dcebe.log.
```

Entry of that reference in the service log (`$TMPDIR/semio-repo/501/host-916ec84dcebe.log`):

```
2026-11-14T11:07:08.198Z request-failed a5924299 Error: Slice worker exited with code 75
    at <anonymous> (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs:167:24)
    at processTicksAndRejections (native:7:39)
```

Failed tasks: none listed by nx (the plan did not finish). Failed step: `workspace:setup` (graph slice load).

## `[capacity]` lines (verbatim, in order, repeats removed)

```
[capacity] run: sized at 6% of the default from 9 earlier run(s) of its class, at least 4093 MB because of its plan phase
[capacity] waiting 0s for run: not enough free memory (need 4093 MB; 3856 MB spare above the 2048 MB floor, load 13.07; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted run with 1 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 32% of the default from 9 earlier run(s) of its class, at least 4093 MB because of its plan phase
[capacity] run: sized at 100% of the default from 6 earlier run(s) of its class, at least 4093 MB because of its plan phase
[capacity] dependencies: sized at 76% of the default from 7 earlier run(s) of its class
[capacity] admitted dependencies with 4595 of 4677 MB: only 4595 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] keeping the dependencies grant (1 job(s), 4595 MB reserved) for the command that replaces this one: it does not queue again
[capacity] took over the dependencies grant of the command this one replaces (1 job(s), 4595 MB reserved): no queue
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] waiting 0s for run: not enough free memory (need 2048 MB; 1763 MB spare above the 2048 MB floor, load 5.92; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted run: 1 job(s), 2048 MB reserved (6433 MB available, 2048 MB floor)
```

## `graph:` lines

None in the log.

## Peak resident size
Sampled every 30 s (`round-1.samples.txt`, 35 samples). The three largest values seen:

```
3205312 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \012const fs=await i
3114784 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \012const fs=await i
2651712 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \012const fs=await i
```

## Wall time per phase
The log carries no time stamps per line; the phases below are bounded by the log's own counters.

| Phase | Log lines | Wall time |
|---|---|---|
| toolchain (`Installing the rust-toolchain.toml toolchain…`) | 10 – 13 | not measured |
| `repo run repo-mcp:install` (`Installing the native repository command…`) | 14 – 811 | not measured |
| `repo dependencies sync --all` (`Installing the dependency scopes…`) | 812 – 30197 | not measured |
| `repo run workspace:setup` | 30198 – 30361 | about 290 s: 8 s waiting for the graph service, then `Reading the project graph slice` up to `(276 s): flush 1 of 1`, then the failure |
| whole run | | 17 min 12 s |

## Files
- `round-1.tail.txt`: last 200 lines of the log.
- `round-1.samples.txt`, `round-1.monitor.txt`: samples and the monitor's start, end and hard-stop record.
- `run-round.sh`: the runner used (bootstrap in its own process group, sample every 30 s, one SIGINT on a hard stop).
