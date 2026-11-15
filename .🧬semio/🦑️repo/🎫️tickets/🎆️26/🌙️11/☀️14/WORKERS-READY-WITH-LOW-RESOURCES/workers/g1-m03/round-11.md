# g1-m03 Round 11

- Machine: Mac mini M4, 16 GB memory, macOS 27.2.1 (26B131), free disk 87 GiB of 228 GiB before the run. Worktree `/Users/ueli/Documents/semio`.
- Result: four tries, marker absent, `ready` false. Tries 1 and 2 ended with `native_rc=75`. **Tries 3 and 4 got past the graph step into the setup tasks** and both stood still in `workspace:deps-build-tools` on `Another process holds the dependency store; waiting for it`; try 3 was interrupted by the 15 minute stall rule (`native_rc=1`), try 4 by the SIGINT for round 12 (PAUSE) after the same wait had reached 723 s (`native_rc=1`).
- Reference `3420bc16` did **not** appear again (0 times after 23:53Z). No `native_rc=4`, no `fstatat failed (EPERM)`, no `where:` line in this round.
- No hard stop; memory pressure level 1 in every sample of all four tries.
- Tasks that failed in round 5 and now **pass**: `@semio-tech/repo-lib:cpp-setup`, `@semio-tech/plugin-registry:generate`. No task ended as failed in this round; the run never reached `framework-os-mcp-rs:build`, `deps-cargo`, `deps-trunk`, `deps-tools`.

## New standstill: `workspace:deps-build-tools` waits for the dependency store

Try 3: the line `Another process holds the dependency store; waiting for it (N s of at most 21600 s)` ran from 0 s to 920 s (first at about 00:56Z); no nx task finished in that time. Try 4: from 0 s (about 01:21Z) to 723 s at the SIGINT.

Before the first wait line (try 3, verbatim, without JSON progress lines):

```
[script] cache:entry · Verifying build tools
[script] cache:entry · Verifying build tools
[script] cache:entry · Verifying build tools
[script] cache:entry · Verifying build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] build:implementation-source · Reading the sources of the build implementation
[repo] still working after 1820 s; last phase started: source.served
> repo "$SEMIO_NX_COMMAND_RUNNER" c33bdbae0baf0c33b0a4ecaa123008729204ea2cd8cb8031b6cf7ea2bbcd823f
[script] build:source-admission · Admitting the sources of the build
[repository:command] still running after 440 s
[script] build:implementation-source · Reading the sources of the build implementation
Another process holds the dependency store; waiting for it (0 s of at most 21600 s)
```

Observation during try 4 at 01:23Z (file `round-11.try-4.holder.txt`): the only process on the machine that works on the dependency tools is the nested command of that task, `bun ./📜️script.ts repo tools setup all` (pid 73661), and it is the one that waits. Nothing else touches the dependency store except its own ancestors: `repo run workspace:setup` (67978) -> resident worker (68661) -> `nx.js run workspace:setup --parallel=1` (69101) -> `run-executor.js` (73502) -> nx plugin (73659) -> `repo tools setup all` (73660/73661). `workspace:deps-js-all` had been started earlier in the same run (`> nx run workspace:deps-js-all`) and has no `✔` line. So the holder is most likely inside the same run (an ancestor, or the unfinished `deps-js-all`); not proven, the lease registry was not read.

Stall record of try 3 (ps lines of the graph service and its children, last 60 lines of the graph service log): `round-11.stall.txt`.

A mistake of this worker, corrected in `round-11.monitor.txt`: at 01:12:14Z the runner was stopped by hand in the belief that its watchdog would interrupt a healthy run; the watchdog had already sent the SIGINT at 01:11:58Z. Try 4 was then started with `run-round-11b.sh` 102 s after the end of try 3.

## Tries

| Try | Log | Start | End | `native_rc` | Stale at the start | Fleet | Reached |
|---|---|---|---|---|---|---|---|
| 1 | `~/native-bootstrap-round-11.log` | 23:53:52Z | 00:11Z | 75 | 259 | asked 259, arrived 153 within 901 s, 106 missing, among them the too heavy `@semio-tech/s-dev-composition (about 4250 MB)` | fleet refusal |
| 2 | `~/native-bootstrap-round-11-try-2.log` | 00:13:59Z | 00:23Z | 75 | see graph lines | - | local inference, `owners 44 of 259`; refusal because `@semio-tech/framework-os (about 5424 MB)` and `@semio-tech/framework-process (about 3612 MB)` need more than the 1756 MB this machine gives one project |
| 3 | `~/native-bootstrap-round-11-try-3.log` | 00:25:34Z | 01:11:58Z | 1 (SIGINT, stall rule) | see graph lines | - | `owners 259 of 259`, generation 67 uploaded, 13 tasks passed, then the dependency store wait |
| 4 | `~/native-bootstrap-round-11-try-4.log` | 01:13:40Z | 01:33:54Z | 1 (SIGINT, round 12) | not printed (graph current) | - | setup tasks, the dependency store wait again |

Waits: 120 s after tries 1 and 2; 102 s after try 3.

Refusal texts (bootstrap logs, verbatim):

```
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 259 and delivered 153 within 901 s, 106 are still missing, among them the too heavy project(s) @semio-tech/s-dev-composition (about 4250 MB) (this machine can give about 3512 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
The project graph could not be read on this machine yet: @semio-tech/framework-os (about 5424 MB), @semio-tech/framework-process (about 3612 MB) need(s) more memory than this machine can give one project (1756 MB). The inference was stopped before the host ran short of memory and the project(s) are marked too heavy here. Nothing is wrong with your tree; run the command again, the fleet is asked for them first.
```

## Setup tasks

Try 3, in log order (`✔` passed, `>` started without a result line):


- ✔ @semio-tech/ui-rs:generate
- ✔ @semio-tech/assets:build
- ✔ @semio-tech/ui-styling-tokens:generate
- ✔ @semio-tech/framework-schema:generate
- > workspace:deps-js-all
- ✔ @semio-tech/dsl-derive-rs:generate-schema-index
- ✔ @semio-tech/repo-lib:cpp-setup
- ✔ repo:generator-inputs
- ✔ repo-mcp:build
- ✔ @semio-tech/plugin-registry:generate
- ✔ repo-mcp:install
- ✔ @semio-tech/ui-styling-py:deps
- ✔ @semio-tech/energy-oracle-py:deps
- > workspace:deps-go
- ✔ @semio-tech/repo-test-dotnet:deps
- > workspace:deps-wasm-opt
- ✔ @semio-tech/framework-graph:generate
- > workspace:deps-build-tools

Try 4, in log order:

- ✔ @semio-tech/ui-rs:generate
- ✔ @semio-tech/assets:build
- ✔ @semio-tech/ui-styling-tokens:generate
- ✔ @semio-tech/framework-schema:generate
- > workspace:deps-js-all
- ✔ @semio-tech/dsl-derive-rs:generate-schema-index
- ✔ @semio-tech/repo-lib:cpp-setup
- ✔ repo:generator-inputs
- ✔ repo-mcp:build
- ✔ @semio-tech/plugin-registry:generate
- ✔ repo-mcp:install
- ✔ @semio-tech/ui-styling-py:deps
- ✔ @semio-tech/energy-oracle-py:deps
- > workspace:deps-go
- ✔ @semio-tech/repo-test-dotnet:deps
- > workspace:deps-wasm-opt
- ✔ @semio-tech/framework-graph:generate
- > workspace:deps-build-tools

No `✖` line, no `Failed tasks:` block and no nx summary in either try (both were interrupted).

## Graph lines of the round (graph service log `host-ac6bfe8f337c.log`, from the first line of this round on; verbatim, each distinct line once, without `waiting for memory headroom`)

```
graph: 259 stale projects, 0 taken from the fleet, 11 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), workspace (about 13896 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-renderer-wgpu (about 6858 MB), and 5 more; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 259
graph: waiting for the fleet: 153 of 259 projects arrived
graph: nothing to wait for any longer: the fleet was asked for 259 stale projects and delivered 153 within 901 s; 106 remain, among them the too heavy project(s) @semio-tech/s-dev-composition (about 4250 MB), which this machine must never infer itself (it can give about 3512 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: No verified change journal cursor is recorded for generation 63; verifying the tree once.
graph: refreshing 259 projects (generation 64)
graph: 61 stale facts (analysis 58, file 2, readdir 1) leave 259 projects stale
graph: breach-_framework_products_repo_script_ts is stale: the workspace file 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🧪️tests/🧵️ready-context/🟦️.ts was added
graph: semio-os-infinite is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/💾️store/🟦️.ts changed
graph: semio-os-infinite is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/⌨️command/🌐️host/🟨️.mjs changed
graph: semio-os-infinite is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/🟨️.mjs changed
graph: semio-os-infinite is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🚪️client/🛰️plan/🟨️.mjs changed
graph: semio-os-infinite is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🟨️.mjs changed
graph: semio-os-infinite is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts changed
graph: repo is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📖️texts/🟦️.ts changed
graph: repo is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/💰️funding/🟦️.ts changed
graph: semio-os-infinite is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📥️adopt/🟦️.ts changed
graph: semio-os-infinite is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts changed
graph: semio-os-infinite is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏛️service/🧯️failure/🟨️.mjs changed
graph: semio-os-infinite is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🟨️.mjs changed
graph: semio-os-infinite is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs changed
graph: semio-os-infinite is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs changed
graph: semio-os-infinite is stale: the source analysis of 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️host/🟦️.ts changed
graph: 46 more stale inputs are not listed
graph: asking the host for up to 8 worker processes for 48 projects; the projects wait for them
graph: headroom of 1024 MB is below one project (1028 MB), inferring one project at a time
graph: headroom of 516 MB again (44 MB taken), resuming
graph: the worker processes are starting; this process infers no project meanwhile and holds them for the workers for at most 60 s
graph: project @semio-tech/framework-process (project) grew by 1806 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: headroom of -706 MB is below one project (1341 MB), inferring one project at a time
graph: project @semio-tech/framework-os (project) grew by 2712 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: inferring projects with 1 worker processes (1536 MB each)
graph: headroom of 839 MB again (694 MB taken), resuming
graph: headroom of -1040 MB is below one project (1823 MB), inferring one project at a time
graph: 0 of 48 projects inferred by 3 worker processes in 393 s (0 failed, 0 lost, 0 restarts)
[fleet] projects from the fleet: 2 taken, 0 stored, 0 offered, 0 waited for another machine, 1 computed here | Projekte aus der Flotte: 2 übernommen, 0 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 1 hier berechnet
graph: headroom of 1024 MB is below one project (1357 MB), inferring one project at a time
graph: project @semio-tech/framework-renderer-wgpu (language:cargo) grew by 2092 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/repo-lib (project) grew by 2051 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: headroom of 1024 MB is below one project (1094 MB), inferring one project at a time
graph: 1 of 48 projects inferred by 6 worker processes in 519 s (0 failed, 0 lost, 0 restarts)
[fleet] projects from the fleet: 0 taken, 0 stored, 0 offered, 0 waited for another machine, 1 computed here | Projekte aus der Flotte: 0 übernommen, 0 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 1 hier berechnet
graph: the worker processes of the earlier waves are ready; the 48 projects of this wave go to them
graph: headroom of 759 MB is below one project (1219 MB), inferring one project at a time
graph: headroom of 1845 MB again (9 MB taken), resuming
graph: headroom of 785 MB is below one project (1262 MB), inferring one project at a time
graph: headroom of 750 MB again (19 MB taken), resuming
graph: headroom of -731 MB is below one project (2699 MB), inferring one project at a time
graph: 0 of 48 projects inferred by 7 worker processes in 56 s (0 failed, 0 lost, 0 restarts)
[fleet] projects from the fleet: 0 taken, 0 stored, 0 offered, 0 waited for another machine, 0 computed here | Projekte aus der Flotte: 0 übernommen, 0 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 0 hier berechnet
graph: the worker processes of the earlier waves are ready; the 158 projects of this wave go to them
graph: headroom of 1024 MB is below one project (2699 MB), inferring one project at a time
graph: headroom of 915 MB again (107 MB taken), resuming
graph: headroom of 909 MB is below one project (2699 MB), inferring one project at a time
graph: headroom of 906 MB again (56 MB taken), resuming
graph: headroom of 906 MB is below one project (2699 MB), inferring one project at a time
graph: headroom of 648 MB again (205 MB taken), resuming
graph: headroom of 891 MB is below one project (2699 MB), inferring one project at a time
graph: headroom of 846 MB again (420 MB taken), resuming
graph: headroom of 840 MB is below one project (2699 MB), inferring one project at a time
graph: headroom of 1287 MB again (46 MB taken), resuming
graph: headroom of 805 MB is below one project (858 MB), inferring one project at a time
graph: headroom of 773 MB again (100 MB taken), resuming
graph: headroom of 627 MB is below one project (858 MB), inferring one project at a time
graph: headroom of 792 MB again (660 MB taken), resuming
graph: headroom of 793 MB is below one project (1162 MB), inferring one project at a time
graph: headroom of 1649 MB again (799 MB taken), resuming
graph: headroom of 435 MB is below one project (1214 MB), inferring one project at a time
graph: headroom of 401 MB again (2 MB taken), resuming
graph: headroom of 586 MB is below one project (690 MB), inferring one project at a time
graph: headroom of 1333 MB again (363 MB taken), resuming
graph: 0 of 158 projects inferred by 8 worker processes in 132 s (0 failed, 0 lost, 0 restarts)
graph: the worker processes of the earlier waves are ready; the 1 projects of this wave go to them
graph: headroom of 372 MB is below one project (808 MB), inferring one project at a time
graph: 0 of 1 projects inferred by 8 worker processes in 39 s (0 failed, 0 lost, 0 restarts)
graph: generation 67 published, 260 projects refreshed in 1006 s (0 kept their edges)
graph: fact index of generation 67: 723153 facts of 5101 projects, 92.3 MB expanded (18.0 % of the 512 MB limit), 12.8 MB packed
graph: 1054 derived results asked again after 0 stale facts: 806 unchanged, 248 changed
graph: generation 67 (5100 projects) uploaded to the remote cache, 5104 documents (24 MB) in 8 s
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 9 s
graph: No verified change journal cursor is recorded for generation 74; verifying the tree once.
```

The last 30 lines of the graph service log at the end of each try are in `round-11.try-<k>.service.txt`.

## 1. Before the run

    Pre-run facts, 2026-11-14T23:53:52Z
    - replica agent: launchctl print gui/501/tech.semio.fleet.replica:
        	state = running
        	runs = 47
        	pid = 95575
        	last exit code = 0
        		state = active
        		state = active
    - replica code digest (.cache/f/health.json): eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc, store ok, daemon pid 95592 , health.json age 3 s
    - "bootVersion":6
    - replica agent kickstarted: 0 times
    - graph service jobs:
        96813	0	tech.semio.repo.service.19c193692817
    - repo service host processes:
        pid 96813 ppid 1 age 41:32 rss 65664
        pid 96823 ppid 96813 age 41:29 rss 173616
    - receipts:
        client.json: {"version":1,"role":"client","pid":12483,"state":"ok","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794699530882,"launcherSha256":"cf25f2b75
        launcher.json: {"version":1,"role":"launcher","pid":96785,"state":"ok","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794697940078,"launcherSha256":"cf25f2b
        service.json: {"version":1,"role":"service","pid":96823,"state":"ok","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794697947170,"launcherSha256":"cf25f2b7
    - last 8 lines of .cache/f/sidecars/build-agent.log:
        2026-11-14T23:50:39.782Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T23:50:40.866Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T23:50:42.008Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T23:50:43.341Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T23:50:44.428Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T23:50:45.531Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T23:50:46.603Z worker registered: a73f7d68-662b-4ab6-9642-627ec55d9da5 cache http://192.168.178.70:6094
        2026-11-14T23:50:49.404Z load report sent: reachable again
    - memory pressure level 1, free disk 87Gi

## Launcher before and after each try (raw: `round-11.launcher.txt`)

Unchanged in all five observations (before try 1, after tries 1, 2, 3, 4): link inode 2817021 -> `versions/cf25f2b751f83336/repo`, identifier `tech.semio.repo`, CDHash `6de7547a8b8ff384bbfbfc45e962462063ff0357`. `~/.local/bin/repo`:
    2811848 -rwxr-xr-x@ 1 ueli  staff  18657778 Nov 14 20:35 /Users/ueli/.local/bin/repo

## `[capacity]` and `graph:` lines of the bootstrap logs (verbatim)

Try 1:

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class, at least 4509 MB because of its assembly phase
[capacity] admitted run with 7 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 6% of the default from 15 earlier run(s) of its class, at least 4509 MB because of its assembly phase
[capacity] run: sized at 52% of the default from 9 earlier run(s) of its class
[capacity] run: sized at 52% of the default from 9 earlier run(s) of its class
[capacity] dependencies: sized at 25% of the default from 15 earlier run(s) of its class, at least 5896 MB because of its assembly phase
[capacity] admitted dependencies with 3880 of 5896 MB: only 3880 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 25% of the default from 15 earlier run(s) of its class, at least 5896 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7200 MB available, 2048 MB floor)
```

Try 2:

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class, at least 4506 MB because of its assembly phase
[capacity] admitted run with 6 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 6% of the default from 15 earlier run(s) of its class, at least 4506 MB because of its assembly phase
[capacity] run: sized at 59% of the default from 9 earlier run(s) of its class
[capacity] run: sized at 59% of the default from 9 earlier run(s) of its class
[capacity] dependencies: sized at 25% of the default from 15 earlier run(s) of its class, at least 5897 MB because of its assembly phase
[capacity] admitted dependencies with 4372 of 5897 MB: only 4372 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 25% of the default from 15 earlier run(s) of its class, at least 5897 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7388 MB available, 2048 MB floor)
```

Try 3:

```
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 4504 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 5190 MB reserved (7818 MB available, 2048 MB floor)
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 4504 MB because of its assembly phase
[capacity] run: sized at 46% of the default from 9 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 9 earlier run(s) of its class
[capacity] dependencies: sized at 25% of the default from 16 earlier run(s) of its class, at least 5897 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 5897 MB reserved (8629 MB available, 2048 MB floor)
[capacity] dependencies: sized at 25% of the default from 16 earlier run(s) of its class, at least 5897 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6455 MB available, 2048 MB floor)
[capacity] run: reservation grown from 2048 MB to 2513 MB of the 12565 MB it asked for, after 0s: nothing else of ours is running (other runs of ours only wait to grow) and only that much memory is free (5211 MB available, 2048 MB floor); the run goes on with 1 of 8 job(s)
[capacity] run: sized at 198% of the default from 4 earlier run(s) of its class
[capacity] run: grown from 1 to 2 of 8 job(s) after 1590s, reservation 2513 MB -> 3949 MB: memory freed up and nothing of ours waits (3800 MB available, 2048 MB floor); work that starts from now on uses the new jobs
```

Try 4:

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class, at least 4497 MB because of its assembly phase
[capacity] admitted run with 6 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 6% of the default from 15 earlier run(s) of its class, at least 4497 MB because of its assembly phase
[capacity] run: sized at 59% of the default from 10 earlier run(s) of its class
[capacity] run: sized at 59% of the default from 10 earlier run(s) of its class
[capacity] dependencies: sized at 25% of the default from 15 earlier run(s) of its class, at least 5899 MB because of its assembly phase
[capacity] waiting 0s for dependencies: not enough free memory (need 5899 MB; 3050 MB spare above the 2048 MB floor, load 7.24; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted dependencies with 4636 of 5899 MB: only 4636 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 25% of the default from 15 earlier run(s) of its class, at least 5899 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 5 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7444 MB available, 2048 MB floor)
[capacity] run: reservation grown from 2048 MB to 2191 MB of the 12565 MB it asked for, after 0s: nothing else of ours is running (other runs of ours only wait to grow) and only that much memory is free (4203 MB available, 2048 MB floor); the run goes on with 1 of 8 job(s)
[capacity] run: sized at 198% of the default from 5 earlier run(s) of its class
[capacity] run: grown from 1 to 2 of 8 job(s) after 195s, reservation 2191 MB -> 3949 MB: memory freed up and nothing of ours waits (4121 MB available, 2048 MB floor); work that starts from now on uses the new jobs
```

## Samples

| Try | Samples | Pressure level other than 1 | Largest process |
|---|---|---|---|
| 1 | 36 | 0 | 3588304 KiB, pid 21542, ppid 96823 |
| 2 | 19 | 0 | 6317504 KiB, pid 32372, ppid 96823 |
| 3 | 93 | 0 | 6543408 KiB, pid 39215, ppid 96823 |
| 4 | 41 | 2 | 3245728 KiB, pid 69101, ppid 68661 |

## Files

`run-round-11.sh`, `run-round-11b.sh`, `round-11.monitor.txt`, `round-11.prerun.txt`, `round-11.launcher.txt`, `round-11.stall.txt`, `round-11.try-4.holder.txt`, `round-11.try-<k>.samples.txt`, `round-11.try-<k>.service.txt`, `round-11.tail.txt` (last 200 lines of try 4), `round-11.try-3.tail.txt`.
