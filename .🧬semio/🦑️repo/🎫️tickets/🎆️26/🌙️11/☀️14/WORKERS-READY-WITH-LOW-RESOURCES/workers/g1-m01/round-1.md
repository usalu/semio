# g1-m01 Round 1

- Machine: Mac mini, 16 GiB memory, macOS 27.2.1; 81 GiB free on the data volume after the run (90 GiB before).
- Replica daemon code digest (attestation in the store): `c9e91a264c1e` (`c9e91a264c1ec85103656255c3cc93db79012f64b476c8d4af26fa10bbec88a4`). The daemon restarted itself twice during the run (pid 80502 at about 10:45Z, pid 9435 now).
- Start `2026-11-14T10:48:02Z`, end `2026-11-14T11:50:47Z` (62 min 45 s).
- `native_rc=1`. Marker `.cache/f/native-bootstrap.done` absent. **Hard stop**: one SIGINT at 11:50:47Z because `kern.memorystatus_vm_pressure_level` read 2 (warning) in one of 126 samples; the 125 others read 1 (normal). Last log lines: `SIGINT received: the running work is cancelled and its leases are released`, `Cancelled (SIGINT); leases released.`, `Internal error: Original Script process interrupted`.
- Failed tasks: none reached. `tasks: 15 stale, 0 up to date` was planned; no task had finished or failed when the run was stopped.

## State Before The Round (not part of the protocol, done before it arrived)

- At about 10:40Z the graph storage image `.cache/b/g/m` (12 GiB) was 100 % full (36 MiB free); a bootstrap had just failed on it (`The disk is full: partition / has 88.08 GiB free, so the resident service could not finish writing`). `repo cache cas gc`, `repo cache gc --area=graph`, `repo doctor --fix --only=caches` freed nothing of it and `repo reset` failed (`Internal error: Unknown closure reference publication: task-keys/ee/ee43b8….json`). I stopped the graph service, detached the image and renamed the folder to `.cache/b/g-full-2026-11-14` (still there, 12 GiB). **So this round started with an empty graph image: every owner was inferred again.** Details: `☀️09/SHARED-WORKTREE-FLEET-OVER-NFS/g1-m01-relocate.md`, last section.
- A bootstrap of mine, 30 s old, was running when the protocol arrived; stopped with one SIGINT to its process group before the round.
- The graph service (`repo service host start`) was running from 10:44Z.

## Phases (from the log; it has no timestamps, seconds are those the tool prints)

| Phase | Wall time |
|---|---|
| launcher, toolchain, `repo` install, dependency scopes | not measured separately (about 62.75 min total minus the two below: about 4 min) |
| `workspace:setup`, first plan: `Reading the project graph slice` up to `assembly 703 of 703` | 2344 s |
| build tools, then `Dependency publication requires fresh process implementation admission` / `Restarting with the current implementation (1 of 6)` | under 20 s |
| second plan after the restart: `Reading the project graph slice`, reached `owners 252 of 703` | 1127 s, stopped by the SIGINT |

The restart planned the whole graph a second time: the first 2344 s were not reused.

## `[capacity]` Lines, Verbatim

```
[capacity] run: sized at 8% of the default from 8 earlier run(s) of its class, at least 2882 MB because of its plan phase
[capacity] admitted run: 8 job(s), 4072 MB reserved (6585 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 14 earlier run(s) of its class
[capacity] dependencies: sized at 89% of the default from 9 earlier run(s) of its class
[capacity] admitted dependencies with 3280 of 5466 MB: only 3280 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (8020 MB available, 2048 MB floor)
[capacity] run: reservation grown from 2048 MB to 2574 MB of the 8190 MB it asked for, after 0s: nothing else of ours is running (other runs of ours only wait to grow) and only that much memory is free (4720 MB available, 2048 MB floor); the run goes on with 2 of 8 job(s)
[capacity] run: sized at 145% of the default from 3 earlier run(s) of its class
[capacity] keeping the run grant (2 job(s), 2574 MB reserved) for the command that replaces this one: it does not queue again
[capacity] took over the run grant of the command this one replaces (2 job(s), 2574 MB reserved): no queue
```

## `graph:` Lines

None: the log has no line starting with `graph:` and no `Inferring projects` line (0 matches). The graph progress was printed as `Reading the project graph slice (<s> s): owners <n> of 703`; the owner count passed 300 of 703 in the first plan. The runner only matched the two wordings of `ROUND.md`, so it did not stop there. After the restart the slice waited first: `reservation 0 of 6553 … waiting for memory: 6553 MB are asked for (2048 MB are enough to start), 2660 MB are free above the 2048 MB floor of this machine`.

## Memory

- Peak resident size seen (30 s samples, `round-1.samples.txt`): 6 207 360 KiB (5.92 GiB), a `bun` process of the build; next 5 741 280 and 5 714 736 KiB. No process above 8 GB.
- At the pressure sample (11:50:47Z): `bun` of the build 3 410 320 KiB, a `git` process 1 154 816 KiB (1 874 304 KiB 30 s later; not started by the bootstrap script as far as measured, owner not measured), `bun` replica daemon 530 352 KiB.

## Files

- `round-1.tail.txt`: last 200 lines of `~/native-bootstrap-round-1.log`.
- `round-1.samples.txt`: the 30 s samples with the pressure level.
- `run-round.sh`: the runner used (starts `W/native-bootstrap.sh`, samples, sends the one SIGINT).
