# g1-m06 Round 1

Result: `native_rc=1`, not ready. No task failed; `workspace:setup` never started a task. It waited 1040 s for memory headroom and I ended it with one SIGINT.

## Machine

| Item | Value |
| --- | --- |
| Model | Mac mini, Apple M4, 10 cores |
| Memory | 16 GB (17179869184 bytes) |
| Free disk | 106 GiB of 228 GiB on the data volume |
| OS | macOS 26.6.2 (25G83) |
| Replica code digest (`status`) | `3d9ead73396c`, row `ok`, 0 events behind |
| Git | 2.50.1 (`status` warns that other machines run another version) |

## Run

| Item | Value |
| --- | --- |
| Start | 2026-11-14T10:49:07Z |
| End | 2026-11-14T11:09:15Z |
| `native_rc` | 1 (after my SIGINT at 11:09:15Z) |
| Marker | absent |
| Failed tasks | none reported; no task ran |
| Hard stop conditions of the round | none met: no `graph: refreshing` or `Inferring projects` line in the log, largest process 3.0 GB, pressure level 1 in all 40 samples |

The stop was my decision and is outside the round's hard stop list. The same wait ran 26,897 s (7.5 hours) in last night's attempt on this machine without ever starting, so I gave this run 17 minutes and then sent one SIGINT to the `repo run workspace:setup` process.

## The Blocking Wait

First and last line of the wait, verbatim (514 such lines, one every 2 s):

```
Reading the project graph slice (2 s): reservation 0 of 7776; running (name, kind, seconds, rows): ; waiting for memory headroom (7776 of 7885 MB reserved).
Reading the project graph slice (1040 s): reservation 0 of 7776; running (name, kind, seconds, rows): ; waiting for memory headroom (7776 of 6179 MB reserved).
```

The slice asks for 7776 MB. The second number fell from 7885 MB to about 6200 MB within the first two minutes and stayed between 6159 and 6477 MB afterwards, so the request can never be met on this machine as it stands.

- The 7776 comes from `$TMPDIR/semio-slice-peak-501.json`, which holds `{"megabytes":7776}` and was written on 2026-11-14 at 03:20 local time by last night's fifth attempt. That attempt ran the graph slice and failed 8 of 30 setup tasks.
- A `repo service host run` process (pid 67415) with a `bun --eval` child (pid 67417, 1.75 to 1.86 GB resident) has been alive since 00:34 local time. I did not start it on purpose and did not stop it. It was the largest process on the machine for most of the run. Whether it holds part of the reservation was not measured.
- The replica daemon held 1.46 to 1.48 GB resident during the run.

## The 15 Lines Before the Error

The only error line is `Internal error: Original Script process interrupted`, caused by my SIGINT. The lines before it:

```
Reading the project graph slice (1018 s): reservation 0 of 7776; running (name, kind, seconds, rows): ; waiting for memory headroom (7776 of 6391 MB reserved).
Reading the project graph slice (1020 s): reservation 0 of 7776; running (name, kind, seconds, rows): ; waiting for memory headroom (7776 of 6387 MB reserved).
Reading the project graph slice (1022 s): reservation 0 of 7776; running (name, kind, seconds, rows): ; waiting for memory headroom (7776 of 6390 MB reserved).
Reading the project graph slice (1024 s): reservation 0 of 7776; running (name, kind, seconds, rows): ; waiting for memory headroom (7776 of 6264 MB reserved).
Reading the project graph slice (1026 s): reservation 0 of 7776; running (name, kind, seconds, rows): ; waiting for memory headroom (7776 of 6254 MB reserved).
Reading the project graph slice (1028 s): reservation 0 of 7776; running (name, kind, seconds, rows): ; waiting for memory headroom (7776 of 6258 MB reserved).
Reading the project graph slice (1030 s): reservation 0 of 7776; running (name, kind, seconds, rows): ; waiting for memory headroom (7776 of 6195 MB reserved).
Reading the project graph slice (1032 s): reservation 0 of 7776; running (name, kind, seconds, rows): ; waiting for memory headroom (7776 of 6189 MB reserved).
Reading the project graph slice (1034 s): reservation 0 of 7776; running (name, kind, seconds, rows): ; waiting for memory headroom (7776 of 6183 MB reserved).
Reading the project graph slice (1036 s): reservation 0 of 7776; running (name, kind, seconds, rows): ; waiting for memory headroom (7776 of 6159 MB reserved).
Reading the project graph slice (1038 s): reservation 0 of 7776; running (name, kind, seconds, rows): ; waiting for memory headroom (7776 of 6178 MB reserved).
[repo] still working after 1040 s; last phase started: door.script
Reading the project graph slice (1040 s): reservation 0 of 7776; running (name, kind, seconds, rows): ; waiting for memory headroom (7776 of 6179 MB reserved).
SIGINT received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.
Cancelled (SIGINT); leases released.
Internal error: Original Script process interrupted
```

## `[capacity]` and `graph:` Lines

There is no `graph:` line in the log. The `[capacity]` lines, verbatim, in order of first appearance (repeat counts in brackets):

```
[capacity] run: sized at 8% of the default from 14 earlier run(s) of its class   [2]
[capacity] admitted run: 8 job(s), 1695 MB reserved (6289 MB available, 2048 MB floor)   [1]
[capacity] run: sized at 46% of the default from 2 earlier run(s) of its class   [3]
[capacity] dependencies: sized at 48% of the default from 11 earlier run(s) of its class   [2]
[capacity] admitted dependencies: 1 job(s), 2955 MB reserved (6320 MB available, 2048 MB floor)   [1]
[capacity] waiting 0s for run: not enough free memory (need 2048 MB; 876 MB spare above the 2048 MB floor, load 3.28; 0 running, 0 ahead); behind the host itself (nothing of ours)   [1]
[capacity] admitted run: 1 job(s), 2048 MB reserved (6309 MB available, 2048 MB floor)   [1]
```

The graph lines that do appear before the wait:

```
Loading the project graph of 440cffa5563d.
Planning 1 targets.
```

## Peak Resident Size

Sampled every 30 s, 40 samples (`~/native-bootstrap-round-1.samples.txt` on this machine).

| Process | Peak resident |
| --- | --- |
| `bun --eval` pid 18573, a child of this run, during the phases before `workspace:setup` | 3,146,608 KB (3.0 GB) |
| `bun --eval` pid 67417, child of the old service host | 1,860,864 KB |
| Replica daemon | 1,479,600 KB |

Memory pressure level (`kern.memorystatus_vm_pressure_level`) was 1 (normal) in all 40 samples.

## Wall Time per Phase

The log has no time stamps per phase; only the total and the wait are measured.

| Phase | Wall time |
| --- | --- |
| Launcher build, Rust toolchain, native repository command, `repo-mcp:build`, dependencies | about 168 s together (total minus the wait); not measured separately |
| `workspace:setup`: waiting for memory headroom | 1040 s |
| Total | 1208 s |

## Other Observations

- The log starts with `[semio] Homebrew not available; install pkg-config, Cairo, Pango and DejaVu fonts before running native C++ builds`.
- launchd left the replica agent down once during the run (`state = not running` for more than 20 s); my runner restarted it with `launchctl kickstart` at 10:52:08Z. The same happened 5 times between 10:09Z and 10:47Z before the round.
- Before the round I stopped my own retry loop from last night and its sixth attempt, which had waited 7.5 hours on the same line.
- `round-run.sh` in this folder is the runner I used (bootstrap, sampler, hard stop check, replica restart).
