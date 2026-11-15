# g1-m03 Round 6

- Machine: Mac mini M4, 16 GB memory, macOS 27.2.1 (26B131), free disk 88 GiB of 228 GiB before the run. Worktree `/Users/ueli/Documents/semio`.
- Result: one try, stopped with one SIGINT after 16 min 14 s because `ROUND.md` changed to round 7 while the try was waiting for the fleet (143 of 657 projects arrived). `native_rc=1` (interrupt), marker absent, `ready` false. No denial, no exit 4, no consent text: **consent text seen: no**. No hard stop, memory pressure level 1 in all 33 samples. The setup tasks were not reached, so there is no task list for this round.

## 1. Before the run and the P6 checks

    Pre-run facts, 2026-11-14T19:52:13Z
    - replica code digest (.cache/f/health.json): 28b4f47dc4d69fae3a080d0c58d3520741c02c444760d2a3cec506db943ae8ff, store ok, daemon pid 4292
    - `repo service host` processes (pid, ppid, age, rss KiB, arguments after the script path):
      (0 process(es))
    - last 8 lines of .cache/f/sidecars/build-agent.log:
        2026-11-14T19:41:53.682Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T19:41:54.730Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T19:41:55.828Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T19:41:57.133Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T19:41:58.299Z worker registered: a9fcc7ff-57c6-4508-95a2-103a9d085f5c cache http://192.168.178.70:6094
        2026-11-14T19:42:00.061Z load report sent: reachable again
        2026-11-14T19:44:02.114Z graph-server: alive, 0 machines seen, 0 requests held, read none
        2026-11-14T19:49:09.493Z graph-server: alive, 0 machines seen, 0 requests held, read none
    - memory pressure level 1, free disk 88Gi

P6 checks (sections 1, 2 and 4 of `ROUND-NOTE-P6.md`, run at 19:53Z after `workspace:setup` had started the graph service; section 3 not run, no denial and no exit 4):

```
P6 checks on g1-m03, 2026-11-14T19:53:09Z, after the first repo run of round 6 (try 1, workspace:setup started the graph service at about 19:52:50Z)

## 1. How the service is started
$ launchctl list | grep tech.semio.repo.service
8911	0	tech.semio.repo.service.d8505b2379b9
$ launchctl print gui/501/tech.semio.repo.service.d8505b2379b9 | grep -E "program|arguments|working directory|state|last exit"
	state = running

	program = /Users/ueli/.local/share/semio/launcher/bin/repo
	arguments = {
		/Users/ueli/.local/share/semio/launcher/bin/repo
		/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts
		repo
		service
		host
		run
--
	working directory = /Users/ueli/Documents/semio

	stdout path = /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-293b476f6b9d.log
	stderr path = /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-293b476f6b9d.log
		SSH_AUTH_SOCK => /var/run/com.apple.launchd.QSyGYyZDeI/Listeners
	}
--
	last exit code = (never exited)

	resource coalition = {
		ID = 24825
		type = resource
		state = active
$ ps -o pid,ppid,command -p 8911
  PID  PPID COMMAND
 8911     1 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M^[M^MM-oM-8M^Oproducts/M-pM^_M-&M^QM-oM-8M^Orepo/M-pM^_M^TM-(M-oM-8M^Omodules/M-pM^_M^SM^ZM-oM-8M^Olibrary/M-pM^_M^OM^WM-oM-8M^Obuild/M-pM^_M^ZM-*M-oM-8M^Odoor/M-pM^_M^SM^\M-oM-8M^Oscript.ts repo service host run
$ ls -l /usr/local/bin/repo
lrwxr-xr-x@ 1 root  wheel  27 Nov 13 22:02 /usr/local/bin/repo -> /Users/ueli/.local/bin/repo
$ echo $SEMIO_SERVICE_LAUNCHD
(unset in the agent shell)

## 2. Identity of the replica daemon
$ grep bootVersion ".../fleet/entry/entry.json"
"bootVersion":5
$ ps -axo pid,ppid,command | grep -E "boot.sh|boot.mjs|supervisor" | grep -v grep
 4284     1 /bin/sh /Users/ueli/Library/Application Support/semio/fleet/entry/boot.sh run
 4291  4284 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Library/Application Support/semio/fleet/entry/boot.mjs run
 4292  4291 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M^[M^MM-oM-8M^Oproducts/M-pM^_M-&M^QM-oM-8M^Orepo/M-pM^_M^TM-(M-oM-8M^Omodules/M-pM^_M-$M^]M-oM-8M^Oshared-worktree/M-pM^_M^TM^AM-oM-8M^Oreplica/M-pM^_M^[M^_M-oM-8M^Osupervisor/M-pM^_M^_M-&M-oM-8M^O.ts --boot
$ ls -la entry/ ; tail entry/shim.txt
    
288 Nov 14 20:53 .
96 Nov 13 20:28 ..
30745 Nov 14 20:38 boot.mjs
30745 Nov 14 20:38 boot.prev.mjs
3095 Nov 14 20:38 boot.ps1
2731 Nov 14 20:38 boot.sh
628 Nov 14 20:38 entry.json
218 Nov 14 20:53 state.json
28 Nov 14 20:38 worktree
tail: /Users/ueli/Library/Application Support/semio/fleet/entry/shim.txt: No such file or directory

## 4. Receipts
$ ls ".../semio/access/"*/ && cat ".../semio/access/"*/service.json
total 16
drwx------@ 4 ueli  staff  128 Nov 14 20:53 .
drwx------@ 3 ueli  staff   96 Nov 14 20:52 ..
-rw-------@ 1 ueli  staff  192 Nov 14 20:52 client.json
-rw-------@ 1 ueli  staff  193 Nov 14 20:53 service.json
-- /Users/ueli/Library/Application Support/semio/access/3f08bbea4d32/client.json
{"version":1,"role":"client","pid":8895,"state":"ok","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794685973229}
-- /Users/ueli/Library/Application Support/semio/access/3f08bbea4d32/service.json
{"version":1,"role":"service","pid":8942,"state":"ok","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794685980074}

$ repo preflight --lang en   (run in the checkout root at 19:53Z)
No preflight output: the command waited 270 s in the capacity door behind workspace:setup and graph-plan (`[capacity] waiting … for command: not enough free memory`), was admitted, printed `build:source-admission` and `build:implementation-source` and ended with exit 0 without a check line. Full output: round-6.preflight-during-try-1.txt.

$ repo ./📜️script.ts preflight --group g1 --lang en   (the fleet preflight in 🤝️shared-worktree/🗺️topology, which has the worktree access check; full output: round-6.fleet-preflight-during-try-1.txt)
[13/23] ok   worktree access: the replica can read /Users/ueli/Documents/semio
WARN time synchronisation: the synchronisation state cannot be read here (timedatectl or systemsetup missing or needs root)
WARN native build: pending: the native bootstrap has not completed (/Users/ueli/Documents/semio/.cache/f/native-bootstrap.done is missing), so this machine syncs but cannot build yet; run the provisioning script again to retry it, the log is /Users/ueli/Documents/semio/.cache/f/native-bootstrap.log
ok   worktree access: the replica can read /Users/ueli/Documents/semio
23 checks, 2 with warnings, none failed: the machine works, look at the warnings.
exit=0

Observations: service program is the launcher (/Users/ueli/.local/share/semio/launcher/bin/repo), arguments script.ts repo service host run, working directory the checkout, state running, parent of pid 8911 is 1. /usr/local/bin/repo still links to /Users/ueli/.local/bin/repo (provision script not run again). Replica daemon: bootVersion 5; chain sh boot.sh (4284) -> bun boot.mjs (4291) -> bun replica script (4292); entry/shim.txt does not exist, so SEMIO_WORKSPACE_ROOT could not be checked there.
```

Report back per machine: consent text seen no (no exit 4); section 1 `program = /Users/ueli/.local/share/semio/launcher/bin/repo`, `state = running`, `last exit code = (never exited)`; section 2 chain `sh boot.sh` (pid 4284, parent 1) -> `bun …/entry/boot.mjs run` (4291) -> `bun …/🤝️shared-worktree/🔁️replica/… ` (4292), `bootVersion` 5, no `entry/shim.txt`; section 3 not run; section 4 receipts `client.json` and `service.json` with `state: ok` and identity the launcher, fleet preflight `ok worktree access: the replica can read /Users/ueli/Documents/semio`.

## 2. to 4. Try

| Try | Log | Start | End | `native_rc` | Stale | Taken from the fleet at the start | Asked | Arrived until the stop |
|---|---|---|---|---|---|---|---|---|
| 1 | `~/native-bootstrap-round-6.log` | 19:52:13Z | 20:08:27Z | 1 (SIGINT, round 7 posted) | 657 | 0 | 657 | 143 |

Note: at the start of this try the graph was at generation 59 with 657 stale projects and nothing taken from the fleet, although round 5 try 3 had read all 703 owners at generation 44 two hours earlier.

## 5. Graph lines (graph service log `$TMPDIR/semio-repo/501/host-293b476f6b9d.log`, verbatim) and its last 30 lines at the stop

```
== /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-293b476f6b9d.log
-- graph lines
graph: 657 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 657
graph: waiting for the fleet: 0 of 657 projects arrived
graph: waiting for the fleet: 96 of 657 projects arrived
graph: waiting for the fleet: 143 of 657 projects arrived
-- last 30 lines when the try ended
graph: waiting for the fleet: 96 of 657 projects arrived
[repo] still working after 830 s; last phase started: source.served
graph: waiting for the fleet: 143 of 657 projects arrived
[repository:command] still running after 830 s
graph: waiting for the fleet: 143 of 657 projects arrived
[repository:command] still running after 840 s
graph: waiting for the fleet: 143 of 657 projects arrived
[repository:command] still running after 850 s
[repo] still working after 860 s; last phase started: source.served
graph: waiting for the fleet: 143 of 657 projects arrived
[repository:command] still running after 860 s
graph: waiting for the fleet: 143 of 657 projects arrived
[repository:command] still running after 870 s
graph: waiting for the fleet: 143 of 657 projects arrived
[repository:command] still running after 880 s
[repo] still working after 890 s; last phase started: source.served
graph: waiting for the fleet: 143 of 657 projects arrived
[repository:command] still running after 890 s
graph: waiting for the fleet: 143 of 657 projects arrived
[repository:command] still running after 900 s
graph: waiting for the fleet: 143 of 657 projects arrived
[repository:command] still running after 910 s
[repo] still working after 920 s; last phase started: source.served
graph: waiting for the fleet: 143 of 657 projects arrived
[repository:command] still running after 920 s
graph: waiting for the fleet: 143 of 657 projects arrived
[repository:command] still running after 930 s
SIGINT received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.
Cancelled (SIGINT); leases released.
[repository:command] still running after 940 s
```

No line with `inferred here`, `restored` or `closureRepaired`.

Bootstrap log, lines that mention the fleet (without the `repo` help text): none; its last lines:

```
Reading the project graph slice (927 s): flush 1 of 1.
SIGINT received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.
Cancelled (SIGINT); leases released.
Internal error: Original Script process interrupted
end 2026-11-14T20:08:27Z
native_rc=1
```

## `[capacity]` and `graph:` lines of the bootstrap log (verbatim)

```
[capacity] run: sized at 6% of the default from 15 earlier run(s) of its class, at least 4545 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 5399 MB reserved (7848 MB available, 2048 MB floor)
[capacity] run: sized at 6% of the default from 15 earlier run(s) of its class, at least 4545 MB because of its assembly phase
[capacity] run: sized at 46% of the default from 9 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 9 earlier run(s) of its class
[capacity] dependencies: sized at 76% of the default from 13 earlier run(s) of its class, at least 5886 MB because of its assembly phase
[capacity] admitted dependencies with 4563 of 5886 MB: only 4563 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 76% of the default from 13 earlier run(s) of its class, at least 5886 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7557 MB available, 2048 MB floor)
```

The `repo preflight --lang en` of the P6 checks waited 270 s in the same capacity door (`behind workspace:setup (quiet, 1 job(s), 2048 MB), graph-plan (running, 1 job(s), 660 MB)`) from 19:53Z to 19:58Z.

## Samples and wall time

33 samples every 30 s (`round-6.try-1.samples.txt`), pressure level 1 in all of them; largest process 3599632 KiB (3.43 GiB), pid 9029, ppid 8942 (child of the graph service's worker). Whole run 16 min 14 s; launcher, toolchain, native repo command and dependency scopes until `[semio] workspace:setup…`: about 37 s (19:52:13Z to about 19:52:50Z); waiting for the fleet until the stop: not measured exactly, about 15 min.

## Files

`run-round-6.sh`, `round-6.monitor.txt`, `round-6.prerun.txt`, `round-6.p6-checks.txt`, `round-6.preflight-during-try-1.txt`, `round-6.fleet-preflight-during-try-1.txt`, `round-6.try-1.samples.txt`, `round-6.try-1.service.txt`, `round-6.tail.txt`.
