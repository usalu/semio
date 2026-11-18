# g1-m06 Round 18

Running. One block per try is appended as each try ends; a summary follows at the end of the round. Rounds 12 to 17 were not run here: round 11 try 3 ran from 00:33Z until the Mac restarted at 03:57Z (see `round-11.md`), and I read `round: 18` at 03:51Z.

- Machine: Mac mini, 16 GB memory, macOS 27.2.1 (build 26B131) since the restart at 03:57Z; before it Darwin 25.6.0.
- Replica agent after the restart: running, `runs = 1`, not kickstarted; `bootVersion` 6; code digest at 04:03Z `97445096cc0a4394ffbc45b5ae588ae86aea987d1329f6f4aa4f7779616437ea`, `store` ok, nothing pending.
- Launcher after the restart: link inode 3631708 to `versions/50ff6101d0ecc30c/repo`, identifier `tech.semio.repo`, CDHash `a0f39e389ce8a697c89d7f9357e72d37eb06ab0d`: unchanged.
- Runner rules: one SIGINT on a process above 8 GB resident, pressure level 4 in two samples in a row, no new graph progress line for 15 minutes, dependency store wait above 5 minutes; level 2 is recorded only. Slice worker sampler of `ROUND-NOTE-P12.md` section 1 runs with every try.

## Step 0 (`repo tools setup all`, Once)

Run from the worktree root with `--lang en`. Start 04:04:15Z, end 04:05:46Z, wall time 91 s, exit code 75. No "Another process holds the dependency store" line. I did not run it a second time, as the round says once. Last 15 lines (without `[script]` lines):

```
[repo] still working after 50 s; last phase started: source.capture
[repo] still working after 5 s; last phase started: source.capture
[repo] still working after 50 s; last phase started: source.capture
[repo] still working after 10 s; last phase started: source.capture
[repo] still working after 50 s; last phase started: source.capture
[repo] still working after 20 s; last phase started: source.capture
[repo] still working after 80 s; last phase started: source.capture
[repo] still working after 80 s; last phase started: source.capture
[capacity] keeping the command grant (1 job(s), 1404 MB reserved) for the command that replaces this one: it does not queue again
[capacity] command: sized at 69% of the default from 1 earlier run(s) of its class
[capacity] took over the command grant of the command this one replaces (1 job(s), 1404 MB reserved): no queue
[capacity] command: sized at 69% of the default from 1 earlier run(s) of its class
Cannot find package '@semio-tech/pack-json-decode-ts' imported from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure
The stamps moved again after the restart: run the command again.
tools_rc=75
```

## Try 1

- start 2026-11-15T04:06:07Z, end 2026-11-15T04:07:31Z, `native_rc=75`, hard stop: none.
- Pressure levels of the 30 s samples (count, level):  3 pressure_level=1;. Peak resident of any process: 1138976 KB.
- Slice worker sampler (5 s), row with the largest resident size: ``; closure entries before: 0; closure entries after: 0.
- Launcher after the try: inode 3631707 mtime 2026-11-14T22:55:40+0100 /Users/ueli/.local/share/semio/launcher/bin/repo Identifier=tech.semio.repo CDHash=a0f39e389ce8a697c89d7f9357e72d37eb06ab0d ; marker absent.
- Setup tasks: succeeded 0, failed 0 (none).

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class
[capacity] admitted run with 5 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 8% of the default from 15 earlier run(s) of its class
[capacity] keeping the run grant (5 job(s), 948 MB reserved) for the command that replaces this one: it does not queue again
[capacity] took over the run grant of the command this one replaces (5 job(s), 948 MB reserved): no queue
Cannot find package '@semio-tech/pack-json-decode-ts' imported from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure
The stamps moved again after the restart: run the command again.
end 2026-11-15T04:07:31Z
native_rc=75
```

## Try 2

- start 2026-11-15T04:10:38Z, end 2026-11-15T04:12:03Z, `native_rc=75`, hard stop: none.
- Pressure levels of the 30 s samples (count, level):  3 pressure_level=1;. Peak resident of any process: 1316528 KB.
- Slice worker sampler (5 s), row with the largest resident size: ``; closure entries before: 0; closure entries after: 0.
- Launcher after the try: inode 3631707 mtime 2026-11-14T22:55:40+0100 /Users/ueli/.local/share/semio/launcher/bin/repo Identifier=tech.semio.repo CDHash=a0f39e389ce8a697c89d7f9357e72d37eb06ab0d ; marker absent.
- Setup tasks: succeeded 0, failed 0 (none).

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class
[capacity] admitted run with 6 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 7% of the default from 15 earlier run(s) of its class
[capacity] keeping the run grant (6 job(s), 1057 MB reserved) for the command that replaces this one: it does not queue again
[capacity] took over the run grant of the command this one replaces (6 job(s), 1057 MB reserved): no queue
Cannot find package '@semio-tech/pack-json-decode-ts' imported from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure
The stamps moved again after the restart: run the command again.
end 2026-11-15T04:12:03Z
native_rc=75
```

## Try 3

- start 2026-11-15T04:15:10Z, end 2026-11-15T04:16:38Z, `native_rc=75`, hard stop: none.
- Pressure levels of the 30 s samples (count, level):  3 pressure_level=1;. Peak resident of any process: 1672912 KB.
- Slice worker sampler (5 s), row with the largest resident size: ``; closure entries before: 0; closure entries after: 0.
- Launcher after the try: inode 3631707 mtime 2026-11-14T22:55:40+0100 /Users/ueli/.local/share/semio/launcher/bin/repo Identifier=tech.semio.repo CDHash=a0f39e389ce8a697c89d7f9357e72d37eb06ab0d ; marker absent.
- Setup tasks: succeeded 0, failed 0 (none).

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class
[capacity] admitted run with 1 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 27% of the default from 15 earlier run(s) of its class
[capacity] keeping the run grant (1 job(s), 512 MB reserved) for the command that replaces this one: it does not queue again
[capacity] took over the run grant of the command this one replaces (1 job(s), 512 MB reserved): no queue
Cannot find package '@semio-tech/pack-json-decode-ts' imported from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure
The stamps moved again after the restart: run the command again.
end 2026-11-15T04:16:38Z
native_rc=75
```

## Try 4

- start 2026-11-15T04:19:41Z, end 2026-11-15T04:21:04Z, `native_rc=75`, hard stop: none.
- Pressure levels of the 30 s samples (count, level):  3 pressure_level=1;. Peak resident of any process: 1435008 KB.
- Slice worker sampler (5 s), row with the largest resident size: ``; closure entries before: 0; closure entries after: 0.
- Launcher after the try: inode 3631707 mtime 2026-11-14T22:55:40+0100 /Users/ueli/.local/share/semio/launcher/bin/repo Identifier=tech.semio.repo CDHash=a0f39e389ce8a697c89d7f9357e72d37eb06ab0d ; marker absent.
- Setup tasks: succeeded 0, failed 0 (none).

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class
[capacity] admitted run with 4 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 10% of the default from 15 earlier run(s) of its class
[capacity] keeping the run grant (4 job(s), 839 MB reserved) for the command that replaces this one: it does not queue again
[capacity] took over the run grant of the command this one replaces (4 job(s), 839 MB reserved): no queue
Cannot find package '@semio-tech/pack-json-decode-ts' imported from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure
The stamps moved again after the restart: run the command again.
end 2026-11-15T04:21:04Z
native_rc=75
```

## Try 5

- start 2026-11-15T04:24:12Z, end 2026-11-15T04:25:37Z, `native_rc=75`, hard stop: none.
- Pressure levels of the 30 s samples (count, level):  3 pressure_level=1;. Peak resident of any process: 1330176 KB.
- Slice worker sampler (5 s), row with the largest resident size: ``; closure entries before: 0; closure entries after: 0.
- Launcher after the try: inode 3631707 mtime 2026-11-14T22:55:40+0100 /Users/ueli/.local/share/semio/launcher/bin/repo Identifier=tech.semio.repo CDHash=a0f39e389ce8a697c89d7f9357e72d37eb06ab0d ; marker absent.
- Setup tasks: succeeded 0, failed 0 (none).

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class
[capacity] admitted run with 1 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 27% of the default from 15 earlier run(s) of its class
[capacity] keeping the run grant (1 job(s), 512 MB reserved) for the command that replaces this one: it does not queue again
[capacity] took over the run grant of the command this one replaces (1 job(s), 512 MB reserved): no queue
Cannot find package '@semio-tech/pack-json-decode-ts' imported from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure
The stamps moved again after the restart: run the command again.
end 2026-11-15T04:25:37Z
native_rc=75
```

## Try 6

- start 2026-11-15T04:28:44Z, end 2026-11-15T04:30:07Z, `native_rc=75`, hard stop: none.
- Pressure levels of the 30 s samples (count, level):  3 pressure_level=1;. Peak resident of any process: 1513936 KB.
- Slice worker sampler (5 s), row with the largest resident size: ``; closure entries before: 0; closure entries after: 0.
- Launcher after the try: inode 3631707 mtime 2026-11-14T22:55:40+0100 /Users/ueli/.local/share/semio/launcher/bin/repo Identifier=tech.semio.repo CDHash=a0f39e389ce8a697c89d7f9357e72d37eb06ab0d ; marker absent.
- Setup tasks: succeeded 0, failed 0 (none).

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class
[capacity] admitted run with 1 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 27% of the default from 15 earlier run(s) of its class
[capacity] keeping the run grant (1 job(s), 512 MB reserved) for the command that replaces this one: it does not queue again
[capacity] took over the run grant of the command this one replaces (1 job(s), 512 MB reserved): no queue
Cannot find package '@semio-tech/pack-json-decode-ts' imported from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure
The stamps moved again after the restart: run the command again.
end 2026-11-15T04:30:07Z
native_rc=75
```

## Try 7

- start 2026-11-15T04:33:15Z, end 2026-11-15T04:34:36Z, `native_rc=75`, hard stop: none.
- Pressure levels of the 30 s samples (count, level):  3 pressure_level=1;. Peak resident of any process: 1466752 KB.
- Slice worker sampler (5 s), row with the largest resident size: ``; closure entries before: 0; closure entries after: 0.
- Launcher after the try: inode 3631707 mtime 2026-11-14T22:55:40+0100 /Users/ueli/.local/share/semio/launcher/bin/repo Identifier=tech.semio.repo CDHash=a0f39e389ce8a697c89d7f9357e72d37eb06ab0d ; marker absent.
- Setup tasks: succeeded 0, failed 0 (none).

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class
[capacity] admitted run with 4 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 10% of the default from 15 earlier run(s) of its class
[capacity] keeping the run grant (4 job(s), 839 MB reserved) for the command that replaces this one: it does not queue again
[capacity] took over the run grant of the command this one replaces (4 job(s), 839 MB reserved): no queue
Cannot find package '@semio-tech/pack-json-decode-ts' imported from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure
The stamps moved again after the restart: run the command again.
end 2026-11-15T04:34:36Z
native_rc=75
```

## Try 8

- start 2026-11-15T04:37:46Z, end 2026-11-15T04:39:12Z, `native_rc=75`, hard stop: none.
- Pressure levels of the 30 s samples (count, level):  3 pressure_level=1;. Peak resident of any process: 1378304 KB.
- Slice worker sampler (5 s), row with the largest resident size: ``; closure entries before: 0; closure entries after: 0.
- Launcher after the try: inode 3631707 mtime 2026-11-14T22:55:40+0100 /Users/ueli/.local/share/semio/launcher/bin/repo Identifier=tech.semio.repo CDHash=a0f39e389ce8a697c89d7f9357e72d37eb06ab0d ; marker absent.
- Setup tasks: succeeded 0, failed 0 (none).

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class
[capacity] admitted run with 3 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 12% of the default from 15 earlier run(s) of its class
[capacity] keeping the run grant (3 job(s), 730 MB reserved) for the command that replaces this one: it does not queue again
[capacity] took over the run grant of the command this one replaces (3 job(s), 730 MB reserved): no queue
Cannot find package '@semio-tech/pack-json-decode-ts' imported from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure
The stamps moved again after the restart: run the command again.
end 2026-11-15T04:39:12Z
native_rc=75
```

## Try 9

- start 2026-11-15T04:42:18Z, end 2026-11-15T04:43:40Z, `native_rc=75`, hard stop: none.
- Pressure levels of the 30 s samples (count, level):  3 pressure_level=1;. Peak resident of any process: 1062608 KB.
- Slice worker sampler (5 s), row with the largest resident size: ``; closure entries before: 0; closure entries after: 0.
- Launcher after the try: inode 3631707 mtime 2026-11-14T22:55:40+0100 /Users/ueli/.local/share/semio/launcher/bin/repo Identifier=tech.semio.repo CDHash=a0f39e389ce8a697c89d7f9357e72d37eb06ab0d ; marker absent.
- Setup tasks: succeeded 0, failed 0 (none).

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class
[capacity] admitted run with 1 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 27% of the default from 15 earlier run(s) of its class
[capacity] keeping the run grant (1 job(s), 512 MB reserved) for the command that replaces this one: it does not queue again
[capacity] took over the run grant of the command this one replaces (1 job(s), 512 MB reserved): no queue
Cannot find package '@semio-tech/pack-json-decode-ts' imported from /Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure
The stamps moved again after the restart: run the command again.
end 2026-11-15T04:43:40Z
native_rc=75
```

## Try 10

- start 2026-11-15T04:46:49Z, end 2026-11-15T11:52:54Z, `native_rc=1`, hard stop: memory pressure level 4 in two samples in a row.
- Pressure levels of the 30 s samples (count, level):  6 pressure_level=1; 841 pressure_level=2; 2 pressure_level=4;. Peak resident of any process: 1350528 KB.
- Slice worker sampler (5 s), row with the largest resident size: ``; closure entries before: 0; closure entries after: 0.
- Launcher after the try: inode 3631707 mtime 2026-11-14T22:55:40+0100 /Users/ueli/.local/share/semio/launcher/bin/repo Identifier=tech.semio.repo CDHash=a0f39e389ce8a697c89d7f9357e72d37eb06ab0d ; marker absent.
- Setup tasks: succeeded 0, failed 0 (none).

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class
[capacity] waiting 0s for run: memory pressure is raised (need 512 MB; 4003 MB spare above the 2048 MB floor, load 21.01; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 5s for run: memory pressure is raised (need 512 MB; 3521 MB spare above the 2048 MB floor, load 22.05; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 11s for run: memory pressure is raised (need 512 MB; 3670 MB spare above the 2048 MB floor, load 20.75; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 17s for run: memory pressure is raised (need 512 MB; 3060 MB spare above the 2048 MB floor, load 22.05; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 23s for run: memory pressure is raised (need 512 MB; 3132 MB spare above the 2048 MB floor, load 20.52; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 28s for run: memory pressure is raised (need 512 MB; 3644 MB spare above the 2048 MB floor, load 19.92; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 34s for run: memory pressure is raised (need 512 MB; 3906 MB spare above the 2048 MB floor, load 18.65; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 39s for run: memory pressure is raised (need 512 MB; 3402 MB spare above the 2048 MB floor, load 17.79; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 45s for run: memory pressure is raised (need 512 MB; 2990 MB spare above the 2048 MB floor, load 19.41; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 50s for run: memory pressure is raised (need 512 MB; 3043 MB spare above the 2048 MB floor, load 18.34; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 55s for run: memory pressure is raised (need 512 MB; 3197 MB spare above the 2048 MB floor, load 17.43; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 60s for run: memory pressure is raised (need 512 MB; 2594 MB spare above the 2048 MB floor, load 16.91; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 66s for run: memory pressure is raised (need 512 MB; 3623 MB spare above the 2048 MB floor, load 16.2; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 71s for run: memory pressure is raised (need 512 MB; 2970 MB spare above the 2048 MB floor, load 16.33; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted run with 3 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 12% of the default from 15 earlier run(s) of its class
[capacity] run: sized at 98% of the default from 3 earlier run(s) of its class
[capacity] dependencies: sized at 22% of the default from 15 earlier run(s) of its class
[capacity] waiting 0s for dependencies: memory pressure is raised (need 1341 MB; 3624 MB spare above the 2048 MB floor, load 16.63; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 5s for dependencies: memory pressure is raised (need 1341 MB; 3611 MB spare above the 2048 MB floor, load 15.7; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 11s for dependencies: memory pressure is raised (need 1341 MB; 3084 MB spare above the 2048 MB floor, load 16.86; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 17s for dependencies: memory pressure is raised (need 1341 MB; 3512 MB spare above the 2048 MB floor, load 18.15; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 22s for dependencies: memory pressure is raised (need 1341 MB; 2796 MB spare above the 2048 MB floor, load 19.82; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 28s for dependencies: memory pressure is raised (need 1341 MB; 3418 MB spare above the 2048 MB floor, load 18.55; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 34s for dependencies: memory pressure is raised (need 1341 MB; 3099 MB spare above the 2048 MB floor, load 19.07; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 40s for dependencies: memory pressure is raised (need 1341 MB; 2927 MB spare above the 2048 MB floor, load 17.86; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 46s for dependencies: memory pressure is raised (need 1341 MB; 3059 MB spare above the 2048 MB floor, load 18.99; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 51s for dependencies: memory pressure is raised (need 1341 MB; 3403 MB spare above the 2048 MB floor, load 18.11; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 56s for dependencies: memory pressure is raised (need 1341 MB; 3008 MB spare above the 2048 MB floor, load 17.06; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 62s for dependencies: memory pressure is raised (need 1341 MB; 3079 MB spare above the 2048 MB floor, load 18.26; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 67s for dependencies: memory pressure is raised (need 1341 MB; 2955 MB spare above the 2048 MB floor, load 16.38; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 73s for dependencies: memory pressure is raised (need 1341 MB; 3226 MB spare above the 2048 MB floor, load 15.31; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 79s for dependencies: memory pressure is raised (need 1341 MB; 2722 MB spare above the 2048 MB floor, load 14.89; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 84s for dependencies: memory pressure is raised (need 1341 MB; 3177 MB spare above the 2048 MB floor, load 14.42; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 90s for dependencies: memory pressure is raised (need 1341 MB; 2717 MB spare above the 2048 MB floor, load 13.5; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 95s for dependencies: memory pressure is raised (need 1341 MB; 3293 MB spare above the 2048 MB floor, load 12.82; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 100s for dependencies: memory pressure is raised (need 1341 MB; 2544 MB spare above the 2048 MB floor, load 14.44; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 105s for dependencies: memory pressure is raised (need 1341 MB; 3421 MB spare above the 2048 MB floor, load 15.6; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 111s for dependencies: memory pressure is raised (need 1341 MB; 2675 MB spare above the 2048 MB floor, load 16.29; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 117s for dependencies: memory pressure is raised (need 1341 MB; 3293 MB spare above the 2048 MB floor, load 17.06; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted dependencies after 120s although memory pressure is raised: nothing of ours is running and memory holds
[capacity] admitted run: 1 job(s), 512 MB reserved (3664 MB available, 2048 MB floor)
Reading the project graph slice (25113 s): reservation 0 of 2048; running (name, kind, seconds, rows): ; waiting for memory: 2048 MB are asked for (2048 MB are enough to start), 1090 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and n
Reading the project graph slice (25115 s): reservation 0 of 2048; running (name, kind, seconds, rows): ; waiting for memory: 2048 MB are asked for (2048 MB are enough to start), 1162 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and n
Cancelled (SIGINT); leases released.
Internal error: Original Script process interrupted
end 2026-11-15T11:52:54Z
native_rc=1
```

First error line (log line 29515) and the 15 lines before it:

```
Reading the project graph slice (25091 s): reservation 0 of 2048; running (name, kind, seconds, rows): ; waiting for memory: 2048 MB are asked for (2048 MB are enough to start), 1182 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and n
Reading the project graph slice (25093 s): reservation 0 of 2048; running (name, kind, seconds, rows): ; waiting for memory: 2048 MB are asked for (2048 MB are enough to start), 1182 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and n
Reading the project graph slice (25095 s): reservation 0 of 2048; running (name, kind, seconds, rows): ; waiting for memory: 2048 MB are asked for (2048 MB are enough to start), 1182 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and n
Reading the project graph slice (25097 s): reservation 0 of 2048; running (name, kind, seconds, rows): ; waiting for memory: 2048 MB are asked for (2048 MB are enough to start), 1152 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and n
Reading the project graph slice (25099 s): reservation 0 of 2048; running (name, kind, seconds, rows): ; waiting for memory: 2048 MB are asked for (2048 MB are enough to start), 1152 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and n
Reading the project graph slice (25101 s): reservation 0 of 2048; running (name, kind, seconds, rows): ; waiting for memory: 2048 MB are asked for (2048 MB are enough to start), 1152 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and n
Reading the project graph slice (25103 s): reservation 0 of 2048; running (name, kind, seconds, rows): ; waiting for memory: 2048 MB are asked for (2048 MB are enough to start), 1182 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and n
Reading the project graph slice (25105 s): reservation 0 of 2048; running (name, kind, seconds, rows): ; waiting for memory: 2048 MB are asked for (2048 MB are enough to start), 1182 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and n
Reading the project graph slice (25107 s): reservation 0 of 2048; running (name, kind, seconds, rows): ; waiting for memory: 2048 MB are asked for (2048 MB are enough to start), 1182 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and n
Reading the project graph slice (25109 s): reservation 0 of 2048; running (name, kind, seconds, rows): ; waiting for memory: 2048 MB are asked for (2048 MB are enough to start), 1090 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and n
Reading the project graph slice (25111 s): reservation 0 of 2048; running (name, kind, seconds, rows): ; waiting for memory: 2048 MB are asked for (2048 MB are enough to start), 1090 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and n
Reading the project graph slice (25113 s): reservation 0 of 2048; running (name, kind, seconds, rows): ; waiting for memory: 2048 MB are asked for (2048 MB are enough to start), 1090 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and n
Reading the project graph slice (25115 s): reservation 0 of 2048; running (name, kind, seconds, rows): ; waiting for memory: 2048 MB are asked for (2048 MB are enough to start), 1162 MB are free above the 2048 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and n
SIGINT received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.
Cancelled (SIGINT); leases released.
Internal error: Original Script process interrupted
```

## Try 11

- start 2026-11-15T11:56:26Z, end 2026-11-15T11:57:56Z, `native_rc=130`, hard stop: memory pressure level 4 in two samples in a row.
- Pressure levels of the 30 s samples (count, level):  1 pressure_level=2; 3 pressure_level=4;. Peak resident of any process: 894416 KB.
- Slice worker sampler (5 s), row with the largest resident size: ``; closure entries before: 0; closure entries after: 0.
- Launcher after the try: inode 3631707 mtime 2026-11-14T22:55:40+0100 /Users/ueli/.local/share/semio/launcher/bin/repo Identifier=tech.semio.repo CDHash=a0f39e389ce8a697c89d7f9357e72d37eb06ab0d ; marker absent.
- Setup tasks: succeeded 0, failed 0 (none).

```
[capacity] run: sized at 5% of the default from 14 earlier run(s) of its class
[capacity] waiting 0s for run: memory pressure is raised (need 512 MB; 1145 MB spare above the 2048 MB floor, load 11.65; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 5s for run: memory pressure is critical (need 512 MB; 1187 MB spare above the 2048 MB floor, load 10.87; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 11s for run: memory pressure is critical (need 512 MB; 1223 MB spare above the 2048 MB floor, load 11.04; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 16s for run: memory pressure is raised (need 512 MB; 1225 MB spare above the 2048 MB floor, load 10.24; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 22s for run: memory pressure is critical (need 512 MB; 1084 MB spare above the 2048 MB floor, load 10.06; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 27s for run: memory pressure is raised (need 512 MB; 1183 MB spare above the 2048 MB floor, load 9.33; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 33s for run: memory pressure is raised (need 512 MB; 1247 MB spare above the 2048 MB floor, load 8.29; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 38s for run: memory pressure is raised (need 512 MB; 1267 MB spare above the 2048 MB floor, load 8.29; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 43s for run: memory pressure is raised (need 512 MB; 1218 MB spare above the 2048 MB floor, load 7.96; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 49s for run: memory pressure is raised (need 512 MB; 1325 MB spare above the 2048 MB floor, load 7.48; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 54s for run: memory pressure is raised (need 512 MB; 1303 MB spare above the 2048 MB floor, load 7.12; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 59s for run: memory pressure is raised (need 512 MB; 1296 MB spare above the 2048 MB floor, load 6.71; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 64s for run: memory pressure is critical (need 512 MB; 1212 MB spare above the 2048 MB floor, load 6.42; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 70s for run: memory pressure is raised (need 512 MB; 1199 MB spare above the 2048 MB floor, load 6.14; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 75s for run: memory pressure is raised (need 512 MB; 1361 MB spare above the 2048 MB floor, load 5.73; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 81s for run: memory pressure is raised (need 512 MB; 1190 MB spare above the 2048 MB floor, load 5.59; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 86s for run: memory pressure is raised (need 512 MB; 1234 MB spare above the 2048 MB floor, load 5.46; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 86s for run: memory pressure is raised (need 512 MB; 1234 MB spare above the 2048 MB floor, load 5.46; 0 running, 0 ahead); behind the host itself (nothing of ours)
Waiting for host capacity was cancelled by SIGINT
end 2026-11-15T11:57:56Z
native_rc=130
```

## Try 12

- start 2026-11-15T12:01:27Z, end 2026-11-16T02:02:52Z, `native_rc=75`, hard stop: memory pressure level 4 in two samples in a row.
- Pressure levels of the 30 s samples (count, level):  23 pressure_level=1; 797 pressure_level=2; 731 pressure_level=4;. Peak resident of any process: 1355232 KB.
- Slice worker sampler (5 s), row with the largest resident size: `2026-11-16T01:58:34Z pid=29560 rss=3897MB footprint=[bun [29560]: 64-bit    Footprint: 3955 MB (16384 bytes per page)] last=[graph: 260 of 703 projects arrived from the fleet, 51 inferred here, 392 left; the server last progressed 15 s ago, estimated time left: about 9 min]`; closure entries before: 0; closure entries after: 42567.
- Launcher after the try: inode 5101653 mtime 2026-11-16T02:47:23+0100 /Users/ueli/.local/share/semio/launcher/bin/repo Identifier=tech.semio.repo CDHash=3f4bbfeb8ae668c405b7e88a2df8a1909043512c ; marker absent.
- Setup tasks: succeeded 0, failed 0 (none).

```
[capacity] run: sized at 5% of the default from 14 earlier run(s) of its class
[capacity] waiting 0s for run: memory pressure is critical (need 512 MB; 1134 MB spare above the 2048 MB floor, load 3.42; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 6s for run: memory pressure is critical (need 512 MB; 1055 MB spare above the 2048 MB floor, load 3.3; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 11s for run: memory pressure is critical (need 512 MB; 1208 MB spare above the 2048 MB floor, load 3.36; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 17s for run: memory pressure is raised (need 512 MB; 1174 MB spare above the 2048 MB floor, load 3.25; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 22s for run: memory pressure is raised (need 512 MB; 1215 MB spare above the 2048 MB floor, load 4.46; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 28s for run: memory pressure is raised (need 512 MB; 1292 MB spare above the 2048 MB floor, load 5.23; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 33s for run: memory pressure is raised (need 512 MB; 1137 MB spare above the 2048 MB floor, load 4.97; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 39s for run: memory pressure is raised (need 512 MB; 1353 MB spare above the 2048 MB floor, load 4.81; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 44s for run: memory pressure is raised (need 512 MB; 1223 MB spare above the 2048 MB floor, load 4.5; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 50s for run: memory pressure is raised (need 512 MB; 1238 MB spare above the 2048 MB floor, load 4.3; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 56s for run: memory pressure is raised (need 512 MB; 1182 MB spare above the 2048 MB floor, load 5.64; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 62s for run: memory pressure is critical (need 512 MB; 1310 MB spare above the 2048 MB floor, load 5.23; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 67s for run: memory pressure is raised (need 512 MB; 1229 MB spare above the 2048 MB floor, load 5.05; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 72s for run: memory pressure is raised (need 512 MB; 1375 MB spare above the 2048 MB floor, load 4.89; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 78s for run: memory pressure is raised (need 512 MB; 1226 MB spare above the 2048 MB floor, load 4.58; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 84s for run: memory pressure is critical (need 512 MB; 1129 MB spare above the 2048 MB floor, load 4.37; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 89s for run: memory pressure is raised (need 512 MB; 1161 MB spare above the 2048 MB floor, load 4.26; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 95s for run: memory pressure is raised (need 512 MB; 1093 MB spare above the 2048 MB floor, load 4.72; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 100s for run: memory pressure is critical (need 512 MB; 1603 MB spare above the 2048 MB floor, load 4.58; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 105s for run: memory pressure is raised (need 512 MB; 1443 MB spare above the 2048 MB floor, load 4.46; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 110s for run: memory pressure is raised (need 512 MB; 1523 MB spare above the 2048 MB floor, load 4.1; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 116s for run: memory pressure is raised (need 512 MB; 1260 MB spare above the 2048 MB floor, load 4.09; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted run after 120s although memory pressure is raised: nothing of ours is running and memory holds
[capacity] run: sized at 27% of the default from 14 earlier run(s) of its class
[capacity] run: sized at 98% of the default from 3 earlier run(s) of its class
[capacity] dependencies: sized at 24% of the default from 14 earlier run(s) of its class
[capacity] waiting 0s for dependencies: memory pressure is critical (need 1486 MB; 1564 MB spare above the 2048 MB floor, load 4.3; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 5s for dependencies: memory pressure is raised (need 1486 MB; 1737 MB spare above the 2048 MB floor, load 4.12; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 11s for dependencies: not enough free memory (need 1486 MB; 1444 MB spare above the 2048 MB floor, load 3.88; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 17s for dependencies: memory pressure is raised (need 1486 MB; 1650 MB spare above the 2048 MB floor, load 4.85; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 22s for dependencies: memory pressure is critical (need 1486 MB; 1342 MB spare above the 2048 MB floor, load 4.86; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 27s for dependencies: not enough free memory (need 1486 MB; 1397 MB spare above the 2048 MB floor, load 4.47; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 33s for dependencies: memory pressure is critical (need 1486 MB; 1340 MB spare above the 2048 MB floor, load 4.27; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 38s for dependencies: not enough free memory (need 1486 MB; 1469 MB spare above the 2048 MB floor, load 5.21; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 44s for dependencies: memory pressure is critical (need 1486 MB; 1329 MB spare above the 2048 MB floor, load 4.96; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 49s for dependencies: memory pressure is critical (need 1486 MB; 1287 MB spare above the 2048 MB floor, load 4.64; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 54s for dependencies: not enough free memory (need 1486 MB; 1128 MB spare above the 2048 MB floor, load 4.91; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 60s for dependencies: memory pressure is critical (need 1486 MB; 1186 MB spare above the 2048 MB floor, load 4.59; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 65s for dependencies: not enough free memory (need 1486 MB; 1342 MB spare above the 2048 MB floor, load 5.51; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 71s for dependencies: not enough free memory (need 1486 MB; 1433 MB spare above the 2048 MB floor, load 20.6; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 76s for dependencies: memory pressure is critical (need 1486 MB; 1402 MB spare above the 2048 MB floor, load 29.31; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 82s for dependencies: not enough free memory (need 1486 MB; 1225 MB spare above the 2048 MB floor, load 48.34; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 87s for dependencies: not enough free memory (need 1486 MB; 1101 MB spare above the 2048 MB floor, load 57.28; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 93s for dependencies: not enough free memory (need 1486 MB; 1091 MB spare above the 2048 MB floor, load 67.43; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 99s for dependencies: memory pressure is critical (need 1486 MB; 1029 MB spare above the 2048 MB floor, load 63.71; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 104s for dependencies: not enough free memory (need 1486 MB; 1121 MB spare above the 2048 MB floor, load 59.01; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 110s for dependencies: memory pressure is critical (need 1486 MB; 1294 MB spare above the 2048 MB floor, load 54.36; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 115s for dependencies: not enough free memory (need 1486 MB; 1181 MB spare above the 2048 MB floor, load 51.85; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 121s for dependencies: memory pressure is critical (need 1486 MB; 1124 MB spare above the 2048 MB floor, load 48.02; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 126s for dependencies: memory pressure is critical (need 1486 MB; 1153 MB spare above the 2048 MB floor, load 41.18; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 132s for dependencies: not enough free memory (need 1486 MB; 1076 MB spare above the 2048 MB floor, load 38.04; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 137s for dependencies: not enough free memory (need 1486 MB; 1178 MB spare above the 2048 MB floor, load 37.08; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 143s for dependencies: memory pressure is critical (need 1486 MB; 1147 MB spare above the 2048 MB floor, load 34.19; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 149s for dependencies: memory pressure is critical (need 1486 MB; 1169 MB spare above the 2048 MB floor, load 31.53; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 154s for dependencies: not enough free memory (need 1486 MB; 1156 MB spare above the 2048 MB floor, load 30.37; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 160s for dependencies: not enough free memory (need 1486 MB; 1172 MB spare above the 2048 MB floor, load 28.09; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 166s for dependencies: not enough free memory (need 1486 MB; 1235 MB spare above the 2048 MB floor, load 26.24; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 171s for dependencies: memory pressure is critical (need 1486 MB; 1329 MB spare above the 2048 MB floor, load 22.6; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 177s for dependencies: memory pressure is critical (need 1486 MB; 1242 MB spare above the 2048 MB floor, load 21.19; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 182s for dependencies: memory pressure is critical (need 1486 MB; 1199 MB spare above the 2048 MB floor, load 19.73; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 188s for dependencies: memory pressure is critical (need 1486 MB; 1182 MB spare above the 2048 MB floor, load 18.39; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 193s for dependencies: not enough free memory (need 1486 MB; 1358 MB spare above the 2048 MB floor, load 17.16; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 198s for dependencies: memory pressure is critical (need 1486 MB; 1319 MB spare above the 2048 MB floor, load 16.02; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 204s for dependencies: not enough free memory (need 1486 MB; 1182 MB spare above the 2048 MB floor, load 14.82; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 209s for dependencies: not enough free memory (need 1486 MB; 1255 MB spare above the 2048 MB floor, load 13.71; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 215s for dependencies: memory pressure is critical (need 1486 MB; 1431 MB spare above the 2048 MB floor, load 12.7; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 220s for dependencies: memory pressure is critical (need 1486 MB; 1198 MB spare above the 2048 MB floor, load 12.88; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 225s for dependencies: memory pressure is critical (need 1486 MB; 1330 MB spare above the 2048 MB floor, load 12.09; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 231s for dependencies: memory pressure is critical (need 1486 MB; 1214 MB spare above the 2048 MB floor, load 11.44; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 236s for dependencies: not enough free memory (need 1486 MB; 1158 MB spare above the 2048 MB floor, load 11.08; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 242s for dependencies: not enough free memory (need 1486 MB; 1203 MB spare above the 2048 MB floor, load 9.68; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 247s for dependencies: not enough free memory (need 1486 MB; 1218 MB spare above the 2048 MB floor, load 9.07; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 252s for dependencies: not enough free memory (need 1486 MB; 1332 MB spare above the 2048 MB floor, load 10.34; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 257s for dependencies: memory pressure is critical (need 1486 MB; 1388 MB spare above the 2048 MB floor, load 9.67; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 263s for dependencies: not enough free memory (need 1486 MB; 1248 MB spare above the 2048 MB floor, load 9.14; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 268s for dependencies: memory pressure is critical (need 1486 MB; 1258 MB spare above the 2048 MB floor, load 8.49; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 274s for dependencies: memory pressure is critical (need 1486 MB; 1338 MB spare above the 2048 MB floor, load 7.97; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 279s for dependencies: not enough free memory (need 1486 MB; 1332 MB spare above the 2048 MB floor, load 7.49; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 285s for dependencies: memory pressure is critical (need 1486 MB; 1375 MB spare above the 2048 MB floor, load 6.89; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 291s for dependencies: memory pressure is critical (need 1486 MB; 1316 MB spare above the 2048 MB floor, load 6.5; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 296s for dependencies: not enough free memory (need 1486 MB; 1375 MB spare above the 2048 MB floor, load 6.06; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 302s for dependencies: memory pressure is critical (need 1486 MB; 1094 MB spare above the 2048 MB floor, load 37.54; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 307s for dependencies: not enough free memory (need 1486 MB; 1215 MB spare above the 2048 MB floor, load 59.03; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 313s for dependencies: not enough free memory (need 1486 MB; 1314 MB spare above the 2048 MB floor, load 60.87; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 318s for dependencies: memory pressure is critical (need 1486 MB; 1186 MB spare above the 2048 MB floor, load 73.78; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 324s for dependencies: memory pressure is critical (need 1486 MB; 1292 MB spare above the 2048 MB floor, load 78.36; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 329s for dependencies: not enough free memory (need 1486 MB; 1199 MB spare above the 2048 MB floor, load 86.74; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 335s for dependencies: memory pressure is critical (need 1486 MB; 1256 MB spare above the 2048 MB floor, load 96.37; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 341s for dependencies: memory pressure is critical (need 1486 MB; 1314 MB spare above the 2048 MB floor, load 92.01; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 346s for dependencies: swap use is above its ceiling (need 1486 MB; 1359 MB spare above the 2048 MB floor, load 104.02; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 351s for dependencies: not enough free memory (need 1486 MB; 1341 MB spare above the 2048 MB floor, load 119.66; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 357s for dependencies: memory pressure is critical (need 1486 MB; 1070 MB spare above the 2048 MB floor, load 111.44; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 363s for dependencies: not enough free memory (need 1486 MB; 1156 MB spare above the 2048 MB floor, load 102.68; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 368s for dependencies: memory pressure is critical (need 1486 MB; 1148 MB spare above the 2048 MB floor, load 94.54; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 373s for dependencies: not enough free memory (need 1486 MB; 1480 MB spare above the 2048 MB floor, load 87.13; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 379s for dependencies: not enough free memory (need 1486 MB; 1323 MB spare above the 2048 MB floor, load 80.31; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted dependencies after 383s although memory pressure is raised: nothing of ours is running and memory holds
[capacity] waiting 0s for run: memory pressure is critical (need 512 MB; 972 MB spare above the 2048 MB floor, load 47.71; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted run: 1 job(s), 512 MB reserved (2993 MB available, 2048 MB floor)
[capacity] the memory footprint of running process trees cannot be read on darwin (Command failed: footprint -f bytes --noCategories -p 72824 -p 35901 -p 35891); the resident set is used instead, which can overstate what a process holds
[capacity] Der Speicher-Fußabdruck laufender Prozessbäume kann unter darwin nicht gelesen werden (Command failed: footprint -f bytes --noCategories -p 72824 -p 35901 -p 35891); stattdessen gilt der residente Speicher, der überschätzen kann, was ein Prozess hält
Reading the project graph slice (943 s): flush 1 of 1.
Reading the project graph slice (945 s): flush 1 of 1.
[capacity] Der Speicher-Fußabdruck laufender Prozessbäume kann unter darwin nicht gelesen werden (Command failed: footprint -f bytes --noCategories -p 72824 -p 35901 -p 35891); stattdessen gilt der residente Speicher, der überschätzen kann, was ein Prozess hält
The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 7556 MB, more than the 6553 MB this machine gives a plan, and only 1599 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
end 2026-11-16T02:02:52Z
native_rc=75
```

## Summary of Round 18 (written 2026-11-16T03:11:16Z)

Not ready, 12 tries, no setup task ran in tries 1, 10, 11 and 12 (the ones I read in full); marker absent. I read no round between 18 and 31: try 10 ran 7 h and try 12 ran 14 h, and from about 02:00Z to 03:10Z on 2026-11-16 reads under `~/Documents` blocked for my shell (cause not measured; they answer again). I start nothing further: round 31 says to stop.

| Tries | Start to end (UTC) | `native_rc` | How it ended |
| --- | --- | --- | --- |
| 1 to 9 | 11-15 04:06 to 04:43, about 85 s each | 75 | try 1: "The stamps moved again after the restart: run the command again." Tries 2 to 9 are in the blocks above. |
| 10 | 11-15 04:46 to 11:52 | 1 | my runner sent SIGINT on pressure level 4 in two samples in a row ("Internal error: Original Script process interrupted") |
| 11 | 11-15 11:56 to 11:57 | 130 | same stop rule, while "Waiting for host capacity" |
| 12 | 11-15 12:01 to 11-16 02:02 | 75 | "the plan stage inference (during inference) grew to 7556 MB, more than the 6553 MB this machine gives a plan, and only 1599 MB of memory were available"; my runner also recorded a level 4 stop |

Rounds 23 and later say memory pressure is no stop rule; my runner still applied the round 18 rule (level 4 twice in a row) in tries 10 to 12 because I had not read those rounds.
