# g1-m03 Round 3

- Machine: Mac mini M4, 16 GB memory, macOS 27.2.1 (26B131), free disk 100 GiB of 228 GiB before the run. Worktree `/Users/ueli/Documents/semio`.
- Result: four tries, marker absent, `ready` false. Tries 1 to 3 ended with the fleet refusal and `native_rc=75`; in try 3 the fleet delivered 502 of 703 projects. Try 4 was stopped with one SIGINT after 7 min 5 s because `ROUND.md` changed to round 5 (`native_rc=1`). No hard stop in any try, memory pressure level 1 in all 97 samples. `workspace:setup` never got past the graph step, so there are no nx task failures.

## 1. Before the run

    Pre-run facts, 2026-11-14T16:57:38Z
    - replica code digest (.cache/f/health.json): 39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759, store ok, daemon pid 96303
    - `ps -axo pid,ppid,etime,rss,args` lines of `repo show projects` or `repo service host` processes:
         8384     1       02:14 448656 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M^[M^MM-oM-8M^Oproducts/M-pM^_M-&M^QM-oM-8M^Orepo/M-pM^_M^TM-(M-oM-8M^Omodules/M-pM^_M^SM^ZM-oM-8M^Olibrary/M-pM^_M^OM^WM-oM-8M^Obuild/M-pM^_M^ZM-*M-oM-8M^Odoor/M-pM^_M^SM^\M-oM-8M^Oscript.
         8386  8384       02:12 400240 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'),crypto=await import('node:crypto'),awaitModule=await import('node:module');\012const hash=value=>crypto.createHash('sha256').update(value).digest('hex
      (2 process(es))
    - last 8 lines of .cache/f/sidecars/build-agent.log (last written 2026-11-14 17:47:19 local):
        2026-11-14T16:46:18.468Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
        2026-11-14T16:46:18.516Z worker sandbox: darwin-sandbox confined network denied
        2026-11-14T16:46:18.521Z worker registered: cb70c2bd-c10f-472f-928a-404779c70970 cache http://192.168.178.70:6094
        2026-11-14T16:47:18.096Z agent code changed, restarting
        2026-11-14T16:47:18.100Z worker stopped: ueli-workstation-mac-mini-m4-16gb-256gb-03-lan.fritz.box-4967
        2026-11-14T16:47:19.172Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
        2026-11-14T16:47:19.260Z worker sandbox: darwin-sandbox confined network denied
        2026-11-14T16:47:19.285Z worker registered: ebe16378-2d6a-4b2e-9cb3-ebdf91d94182 cache http://192.168.178.70:6094
    - memory pressure level 1, free disk 100Gi
    - the two processes above, arguments after the script path: 8384 = repo service host run  (parent 1, the resident graph service started by the stopped try 3 of round 2; 2 min 14 s old, so not older than one hour and not a sidecar child); 8386 is its child.

No `repo show projects` process ran, so there was nothing to wait for.

## 2. and 3. Tries

| Try | Log | Start | End | `native_rc` | Wanted | Arrived | Missing | Seconds | Local |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `~/native-bootstrap-round-3.log` | 16:58:01Z | 17:06:01Z | 75 | 703 | 0 | 703 | 302 | 192 |
| 2 | `~/native-bootstrap-round-3-try-2.log` | 17:09:04Z | 17:15:26Z | 75 | 703 | 0 | 703 | 300 | 192 |
| 3 | `~/native-bootstrap-round-3-try-3.log` | 17:18:36Z | 17:44:42Z | 75 | 703 | 502 | 201 | 1487 | 192 |
| 4 | `~/native-bootstrap-round-3-try-4.log` | 17:48:11Z | 17:55:16Z | 1 (SIGINT, round 5 posted) | 201 (703 stale, 502 taken from the fleet) | 0 until the stop | not measured | not measured (stopped after about 360 s of `workspace:setup`) | 192 |

Wait between the tries: 180 s each (runner timestamps in `round-3.monitor.txt`).

First and last "arrived" line per try (graph service log `$TMPDIR/semio-repo/501/host-f74179f31e9f.log`, verbatim):

- Try 1: first and last `graph: waiting for the fleet: 0 of 703 projects arrived`
- Try 2: first and last `graph: waiting for the fleet: 0 of 703 projects arrived`
- Try 3: first `graph: waiting for the fleet: 0 of 703 projects arrived`, last `graph: waiting for the fleet: 502 of 703 projects arrived`. Steps seen: 
0 of 262 of 310 of 358 of 406 of 454 of 502 of 
- Try 4: first and last `graph: waiting for the fleet: 0 of 201 projects arrived`

Missing after try 3: 201, this machine can infer 192, so 9 projects more than it can infer.

## 6. Lines that mention the fleet, `wanted`, `arrived`, `serving`, `restored`, `Restarting`, `adopted`

Bootstrap logs (without the `repo` help text of the launcher build). Try 1:

```
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 703 and delivered 0 within 302 s, 703 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

Try 2:

```
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 703 and delivered 0 within 300 s, 703 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

Try 3:

```
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 703 and delivered 502 within 1487 s, 201 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

Try 4:

```
```

Graph service log, all four tries (line number, first occurrence of each distinct line; the file is cumulative since 16:51Z):

```
141:graph: No verified change journal cursor is recorded for generation 44; verifying the tree once.
1427:graph: 703 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 703
1428:graph: waiting for the fleet: 0 of 703 projects arrived
1498:graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 302 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
1499:2026-11-14T17:06:01.259Z graph-wanted-expired {"wanted":"703","arrived":"0","seconds":"302","missing":"703","local":"192"}
2226:graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 300 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2227:2026-11-14T17:15:26.759Z graph-wanted-expired {"wanted":"703","arrived":"0","seconds":"300","missing":"703","local":"192"}
2957:graph: waiting for the fleet: 262 of 703 projects arrived
2976:graph: waiting for the fleet: 310 of 703 projects arrived
3002:graph: waiting for the fleet: 358 of 703 projects arrived
3020:graph: waiting for the fleet: 406 of 703 projects arrived
3057:graph: waiting for the fleet: 454 of 703 projects arrived
3106:graph: waiting for the fleet: 502 of 703 projects arrived
3201:graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 502 within 1487 s, 201 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
3202:2026-11-14T17:44:42.256Z graph-wanted-expired {"wanted":"703","arrived":"502","seconds":"1487","missing":"201","local":"192"}
3834:graph: 703 stale projects, 502 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 201
3835:graph: waiting for the fleet: 0 of 201 projects arrived
```

No line with `serving`, `restored`, `Restarting` or `adopted` in the bootstrap logs or in the service log.

## `[capacity]` and `graph:` lines of the bootstrap logs (verbatim)

Try 1:

```
[capacity] run: sized at 6% of the default from 13 earlier run(s) of its class, at least 4573 MB because of its assembly phase
[capacity] admitted run with 1 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 30% of the default from 13 earlier run(s) of its class, at least 4573 MB because of its assembly phase
[capacity] run: sized at 100% of the default from 9 earlier run(s) of its class, at least 4573 MB because of its assembly phase
[capacity] run: sized at 100% of the default from 9 earlier run(s) of its class, at least 4573 MB because of its assembly phase
[capacity] dependencies: sized at 85% of the default from 7 earlier run(s) of its class, at least 5879 MB because of its assembly phase
[capacity] admitted dependencies with 3750 of 5879 MB: only 3750 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 85% of the default from 7 earlier run(s) of its class, at least 5879 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (5032 MB available, 2048 MB floor)
```

Try 2:

```
[capacity] run: sized at 6% of the default from 14 earlier run(s) of its class, at least 4571 MB because of its assembly phase
[capacity] admitted run with 3 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 14% of the default from 14 earlier run(s) of its class, at least 4571 MB because of its assembly phase
[capacity] run: sized at 100% of the default from 8 earlier run(s) of its class
[capacity] run: sized at 100% of the default from 8 earlier run(s) of its class
[capacity] dependencies: sized at 85% of the default from 8 earlier run(s) of its class, at least 5880 MB because of its assembly phase
[capacity] admitted dependencies with 3362 of 5880 MB: only 3362 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 85% of the default from 8 earlier run(s) of its class, at least 5880 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6818 MB available, 2048 MB floor)
```

Try 3:

```
[capacity] run: sized at 6% of the default from 14 earlier run(s) of its class, at least 4569 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 5423 MB reserved (7856 MB available, 2048 MB floor)
[capacity] run: sized at 6% of the default from 14 earlier run(s) of its class, at least 4569 MB because of its assembly phase
[capacity] run: sized at 46% of the default from 8 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 8 earlier run(s) of its class
[capacity] dependencies: sized at 85% of the default from 9 earlier run(s) of its class, at least 5880 MB because of its assembly phase
[capacity] admitted dependencies with 4641 of 5880 MB: only 4641 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 85% of the default from 9 earlier run(s) of its class, at least 5880 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6177 MB available, 2048 MB floor)
```

Try 4:

```
[capacity] run: sized at 6% of the default from 15 earlier run(s) of its class, at least 4565 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 5419 MB reserved (7682 MB available, 2048 MB floor)
[capacity] run: sized at 6% of the default from 15 earlier run(s) of its class, at least 4565 MB because of its assembly phase
[capacity] run: sized at 46% of the default from 8 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 8 earlier run(s) of its class
[capacity] dependencies: sized at 76% of the default from 10 earlier run(s) of its class, at least 5881 MB because of its assembly phase
[capacity] admitted dependencies with 4364 of 5881 MB: only 4364 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 76% of the default from 10 earlier run(s) of its class, at least 5881 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6343 MB available, 2048 MB floor)
```

## 5. Samples with parent pid (every 30 s, `ps -axo rss,pid,ppid,args | sort -nr | head -3`)

| Try | Samples | Pressure level other than 1 | Largest process (KiB, pid, ppid) |
|---|---|---|---|
| 1 | 16 | 0 | 3649296, pid 10265, ppid 8386 (`bun -- …`, child of the resident graph service) |
| 2 | 13 | 0 | 2838128, pid 16170, ppid 8386 |
| 3 | 53 | 0 | 2961552, pid 21014, ppid 8386 |
| 4 | 15 | 0 | 3027968, pid 36055, ppid 8386 |

Peak of the round: 3649296 KiB (3.48 GiB). Files: `round-3.try-<k>.samples.txt`.

The `git` process above 1 GB: seen in every try (40 parent chains recorded in `round-3.try-<k>.git-parents.txt`). Each time the same chain, for example at 16:58:02Z:

```
2026-11-14T16:58:02Z git pid 9553
 9553 47709 2414432 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.hooksPath=/dev/null -c filter.lfs.clean= -c filter.lfs.smudge= -c filter.lfs.process= -c filter.lfs.r
47709 47677 383904 /Applications/Claude.app/Contents/Frameworks/Claude Helper.app/Contents/MacOS/Claude Helper --type=utility --utility-sub-type=node.mojom.NodeService --lang=en-US --service-sandbox-type=none --user-data-dir=/Users/ueli/Library/Application Support/Claude --standard-schemes=cowork-ar
47677     1 250992 /Applications/Claude.app/Contents/MacOS/Claude
    1     0 /sbin/launchd
```

So the parent is the Claude desktop app (`Claude Helper … NodeService`, child of `/Applications/Claude.app/Contents/MacOS/Claude`, child of launchd): the app in which this worker agent runs polls the git state of the worktree. Each such `git` lives a few seconds and takes 2.3 to 2.5 GB. It is not started by the bootstrap, the replica or a sidecar.

## Wall time per phase

| Phase | Try 1 | Try 2 | Try 3 | Try 4 |
|---|---|---|---|---|
| whole run | 8 min 0 s | 6 min 22 s | 26 min 6 s | 7 min 5 s |
| waiting for the fleet | 302 s | 300 s | 1487 s | not measured (stopped) |
| launcher, toolchain, native repo command, dependency scopes | not measured separately | not measured separately | not measured separately | not measured separately |

## Files

`run-round-3.sh` (runner), `round-3.monitor.txt`, `round-3.prerun.txt`, `round-3.try-<k>.samples.txt`, `round-3.try-<k>.git-parents.txt`, `round-3.try-<k>.service-lines.txt`, `round-3.tail.txt` (last 200 lines of try 4), `round-3.try-3.tail.txt`.
