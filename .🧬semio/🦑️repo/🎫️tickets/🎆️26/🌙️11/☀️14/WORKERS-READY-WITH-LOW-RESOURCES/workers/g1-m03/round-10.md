# g1-m03 Round 10

- Machine: Mac mini M4, 16 GB memory, macOS 27.2.1 (26B131), free disk 87 GiB of 228 GiB before the run. Worktree `/Users/ueli/Documents/semio`.
- Result: two tries, both `native_rc=1` with the **same unexpected failure of the graph service** (reference `3420bc16`, an `Error` without text thrown in `requireRepositoryBuildGraphWriterV1`) during the local inference of the graph step. Two tries in a row with the same failure: the round stopped. Marker absent, `ready` false.
- **No access problem in this round**: no `native_rc=4`, no `fstatat failed (EPERM)`, no `where: root=… entry=… code=…` line (0 in the graph service log), no fallback line, no "runs under the app's access" line. The graph service read the checkout for 37 minutes. The launcher receipt before the run was still `state: waiting`.
- No hard stop; memory pressure level 1 in every sample. The setup tasks were not reached.
- Try 2 was started although the round has no rule for this failure: its text says `try again`, and the stop rule needs two tries with the same failure. It was the comparison try; no third try was started.

## The failure (verbatim)

Bootstrap log, try 1 and try 2:

```
The resident service met an unexpected failure (reference 3420bc16); try again, the details under that reference are in the service log /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-ac6bfe8f337c.log.
end 2026-11-14T23:36:15Z
native_rc=1
The resident service met an unexpected failure (reference 3420bc16); try again, the details under that reference are in the service log /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-ac6bfe8f337c.log.
end 2026-11-14T23:48:53Z
native_rc=1
```

Graph service log `$TMPDIR/semio-repo/501/host-ac6bfe8f337c.log`, the two entries (both carry the same reference):

```
6282:2026-11-14T23:36:15.205Z request-failed 3420bc16 Error
6283-    at requireRepositoryBuildGraphWriterV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:204:162)
6284-    at repositoryBuildGraphWriterDirectoryV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:210:2)
6285-    at ownerCachePathV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:86:62)
6286-    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:290:242)
6287-    at run (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.mjs:183:21)
6288-    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:290:101)
6289-    at processTicksAndRejections (unknown:7:39)
6290-[repository:command] still running after 1440 s
--
12091:2026-11-14T23:48:53.644Z request-failed 3420bc16 Error
12092-    at requireRepositoryBuildGraphWriterV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:204:162)
12093-    at repositoryBuildGraphWriterDirectoryV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:210:2)
12094-    at ownerCachePathV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:86:62)
12095-    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:290:242)
12096-    at run (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.mjs:183:21)
12097-    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:290:101)
12098-    at processTicksAndRejections (unknown:7:39)
12099-[repository:command] still running after 2191 s
```

The four lines before the first entry and before the second entry:

```
[script] build:projects · Inferring projects: 90 of 99 projects · 1 running, longest 0 s: @semio-tech/os-extension-rs (language:rust, 0 s, 0 rows)
[script] build:projects · Inferring projects: 93 of 99 projects
graph: 0 of 99 projects inferred by 8 worker processes in 41 s (0 failed, 0 lost, 0 restarts)
[fleet] projects from the fleet: 0 taken, 0 stored, 0 offered, 0 waited for another machine, 0 computed here | Projekte aus der Flotte: 0 übernommen, 0 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 0 hier berechnet
--
[script] build:projects · Inferring projects: 168 of 176 projects · 2 running, longest 0 s: semio-framework-job-outcome (language:cargo, 0 s, 0 rows), semio-framework-job-outcome-borrow (language:cargo, 0 s, 0 rows)
[script] build:projects · Inferring projects: 171 of 176 projects
graph: 1 of 176 projects inferred by 4 worker processes in 147 s (0 failed, 0 lost, 0 restarts)
[fleet] projects from the fleet: 1 taken, 0 stored, 0 offered, 0 waited for another machine, 0 computed here | Projekte aus der Flotte: 1 übernommen, 0 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 0 hier berechnet
--
```

Where the tries were when it happened: try 1 at `owners 281 of 659` after 1426 s of `workspace:setup`; try 2 at `owners 400 of 459` after about 600 s. Try 2 started with 459 stale projects, so the 200 projects try 1 had finished were kept.

## 1. Before the run

    Pre-run facts, 2026-11-14T23:11:42Z
    - replica agent: launchctl print gui/501/tech.semio.fleet.replica:
        	state = running
        	runs = 47
        	pid = 95575
        	last exit code = 0
        		state = active
        		state = active
    - replica code digest (.cache/f/health.json): eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc, store ok, daemon pid 95592 , health.json age 4 s
    - "bootVersion":6
    - replica agent kickstarted: 0 times
    - graph service jobs:
        -	4	tech.semio.repo.service.3f8d4f1eae84
    - repo service host processes: 0
    - receipts:
        client.json: {"version":1,"role":"client","pid":91817,"state":"ok","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794697050112}
        launcher.json: {"version":1,"role":"launcher","pid":95592,"state":"waiting","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794697757406,"launcherSha256":"cf
        service.json: {"version":1,"role":"service","pid":66015,"state":"denied","root":"/Users/ueli/Documents/semio","folder":"Documents","identity":"/Users/ueli/.local/share/semio/launcher/bin/repo","at":1794697053191}
    - last 8 lines of .cache/f/sidecars/build-agent.log:
        2026-11-14T23:08:33.017Z worker reconnecting: Unable to connect. Is the computer able to access the url?
        2026-11-14T23:08:34.101Z worker registered: e60eb409-e272-4f0a-b31a-216a36db045b cache http://192.168.178.70:6094
        2026-11-14T23:08:34.478Z load report sent: reachable again
        2026-11-14T23:09:09.651Z graph-server: alive, 0 machines seen, 0 requests held, read none
        2026-11-14T23:09:15.747Z worker stopped: ueli-workstation-mac-mini-m4-16gb-256gb-03-lan.fritz.box-82638
        2026-11-14T23:09:48.036Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
        2026-11-14T23:09:48.082Z worker sandbox: darwin-sandbox confined network denied
        2026-11-14T23:09:48.085Z worker registered: a4aa01b4-7fad-4c13-9662-7c1178bcad5a cache http://192.168.178.70:6094
    - memory pressure level 1, free disk 87Gi

## 2. and 5. Launcher before and after each try (raw: `round-10.launcher.txt`)

| When | Link inode | Target | Identifier | CDHash | Changed |
|---|---|---|---|---|---|
| before try 1 (23:11:43Z) | 2817021 | `versions/cf25f2b751f83336/repo` | `tech.semio.repo` | `6de7547a8b8ff384bbfbfc45e962462063ff0357` | (reference; the same as in rounds 8 and 9) |
| after try 1 (23:36:16Z) | 2817021 | same | `tech.semio.repo` | `6de7547a…` | no |
| after try 2 (23:49:17Z) | 2817021 | same | `tech.semio.repo` | `6de7547a…` | no |

`~/.local/bin/repo`: inode 2811848, mtime 20:35:40Z, unchanged.

## Tries

| Try | Log | Start | End | `native_rc` | Stale at the start | Taken from the fleet | Reached |
|---|---|---|---|---|---|---|---|
| 1 | `~/native-bootstrap-round-10.log` | 23:11:43Z | 23:36:15Z | 1 | 659 | 0 | local inference, `owners 281 of 659`, then the failure |
| 2 | `~/native-bootstrap-round-10-try-2.log` | 23:38:16Z | 23:48:53Z | 1 | 459 | see the graph lines | local inference, `owners 400 of 459`, then the same failure |

Wait between the tries: 120 s. No fleet refusal in this round: after `waiting for the fleet: 0 of 659 projects arrived` the service went on with local inference.

Too heavy projects named at the start of try 1 (verbatim):

```
graph: 659 stale projects, 0 taken from the fleet, 7 too heavy for this machine (@semio-tech/repo-lib (about 27770 MB), workspace (about 13780 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-dev (about 8452 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-os-kernel (about 6984 MB), and 1 more; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 659
```

## Graph lines per try (graph service log, verbatim) and its last 30 lines when the try ended

### Try 1

```
== /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-ac6bfe8f337c.log (lines after 0)
-- graph lines
graph: 659 stale projects, 0 taken from the fleet, 7 too heavy for this machine (@semio-tech/repo-lib (about 27770 MB), workspace (about 13780 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-dev (about 8452 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-os-kernel (about 6984 MB), and 1 more; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 659
graph: waiting for the fleet: 0 of 659 projects arrived
graph: waiting for the fleet: 552 of 659 projects arrived
graph: refreshing 659 projects (generation 60)
graph: 130 stale facts (analysis 77, readdir 28, file 25) leave 659 projects stale
graph: breach-_framework_products_repo_script_ts is stale: the workspace file 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🔐️consent/🔎️probe/🟨️.mjs was added
graph: breach-_framework_products_repo_script_ts is stale: the workspace file 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🔐️consent/🟨️.d.mts was added
graph: breach-_framework_products_repo_script_ts is stale: the workspace file 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🔐️consent/🟨️.mjs was added
graph: breach-_framework_products_repo_script_ts is stale: the workspace file 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🔐️consent/🟦️.ts was added
graph: breach-_framework_products_repo_script_ts is stale: the workspace file 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🛡️consent-service/🟦️.ts was added
graph: asking the host for up to 8 worker processes for 48 projects; the projects wait for them
graph: asking the host for up to 9 worker processes for 151 projects; the projects wait for them
-- last 30 lines when the try ended
[script] build:projects · Inferring projects: 86 of 99 projects · 2 running, longest 0 s: @semio-tech/os-plugin-registry-schema-rs (language:rust, 0 s, 0 rows), @semio-tech/os-dsl-rs (language:rust, 0 s, 0 rows)
[script] build:projects · Inferring projects: 90 of 99 projects · 2 running, longest 0 s: @semio-tech/os-services-rs (language:cargo, 0 s, 0 rows), @semio-tech/os-extension-rs (language:rust, 0 s, 0 rows)
[script] build:projects · Inferring projects: 90 of 99 projects · 1 running, longest 0 s: @semio-tech/os-extension-rs (language:rust, 0 s, 0 rows)
[script] build:projects · Inferring projects: 93 of 99 projects
graph: 0 of 99 projects inferred by 8 worker processes in 41 s (0 failed, 0 lost, 0 restarts)
[fleet] projects from the fleet: 0 taken, 0 stored, 0 offered, 0 waited for another machine, 0 computed here | Projekte aus der Flotte: 0 übernommen, 0 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 0 hier berechnet
2026-11-14T23:36:15.205Z request-failed 3420bc16 Error
    at requireRepositoryBuildGraphWriterV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:204:162)
    at repositoryBuildGraphWriterDirectoryV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:210:2)
    at ownerCachePathV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:86:62)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:290:242)
    at run (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.mjs:183:21)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:290:101)
    at processTicksAndRejections (unknown:7:39)
```

### Try 2

```
== /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-ac6bfe8f337c.log (lines after 6305)
-- graph lines
graph: refreshing 459 projects (generation 62)
graph: 91 stale facts (analysis 66, file 25) leave 459 projects stale
graph: asking the host for up to 8 worker processes for 48 projects; the projects wait for them
-- last 30 lines when the try ended
[script] build:projects · Inferring projects: 160 of 176 projects · 8 running, longest 0 s: semio-framework-io-failure-owner (language:cargo, 0 s, 0 rows), semio-framework-io-binary-source (language:cargo, 0 s, 0 rows), semio-framework-manifest-input-compiler (language:cargo, 0 s, 0 rows)
[script] build:projects · Inferring projects: 168 of 176 projects · 2 running, longest 0 s: semio-framework-job-outcome (language:cargo, 0 s, 0 rows), semio-framework-job-outcome-borrow (language:cargo, 0 s, 0 rows)
[script] build:projects · Inferring projects: 171 of 176 projects
graph: 1 of 176 projects inferred by 4 worker processes in 147 s (0 failed, 0 lost, 0 restarts)
[fleet] projects from the fleet: 1 taken, 0 stored, 0 offered, 0 waited for another machine, 0 computed here | Projekte aus der Flotte: 1 übernommen, 0 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 0 hier berechnet
2026-11-14T23:48:53.644Z request-failed 3420bc16 Error
    at requireRepositoryBuildGraphWriterV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:204:162)
    at repositoryBuildGraphWriterDirectoryV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:210:2)
    at ownerCachePathV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:86:62)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:290:242)
    at run (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.mjs:183:21)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:290:101)
    at processTicksAndRejections (unknown:7:39)
[repository:command] still running after 2191 s
[repository:command] still running after 2201 s
[repo] still working after 2210 s; last phase started: source.served
[repository:command] still running after 2211 s
```

Other `graph:` lines of the round (first occurrence of each distinct beginning, without the `waiting for memory headroom` lines):

```
graph: No verified change journal cursor is recorded for generation 59; verifying the tree once.
graph: 659 stale projects, 0 taken from the fleet, 7 too heavy for this machine (@semio-tech/repo-lib (about 27770 MB), workspace (about 13780 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-dev (about 8452 MB), @semio-tech/framework-os (about 8368 MB), @semio-tech/framework-os-kernel (about 6984 MB), and 1 more; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 659
graph: waiting for the fleet: 0 of 659 projects arrived
graph: waiting for the fleet: 552 of 659 projects arrived
graph: refreshing 659 projects (generation 60)
graph: 130 stale facts (analysis 77, readdir 28, file 25) leave 659 projects stale
graph: @semio-tech/filesystem-observation is stale: the workspace file 🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🧪️tests/🧭️denied-entry/🟦️.ts was added
graph: @semio-tech/plugin-registry is stale: the workspace file 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/🔗️import-resolution/🟦️.ts was added
graph: repo is stale: the workspace file 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/⚙️setup/🧪️tests/⏳️busy-funding/🟦️.ts was added
graph: breach-_framework_products_repo_script_ts is stale: the workspace file 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/💰️wave-costs/🟦️.ts was added
graph: 130 more stale inputs are not listed
graph: asking the host for up to 8 worker processes for 48 projects; the projects wait for them
graph: headroom of 1041 MB is below one project (3857 MB), inferring one project at a time
graph: inferring projects with 1 worker processes (1536 MB each)
graph: headroom of 365 MB again (111 MB taken), resuming
graph: headroom of 696 MB is below one project (1893 MB), inferring one project at a time
graph: headroom of 660 MB again (95 MB taken), resuming
graph: headroom of -272 MB is below one project (305 MB), inferring one project at a time
graph: headroom of 669 MB again (464 MB taken), resuming
graph: 0 of 48 projects inferred by 1 worker processes in 52 s (0 failed, 0 lost, 0 restarts)
[fleet] projects from the fleet: 0 taken, 0 stored, 0 offered, 0 waited for another machine, 0 computed here | Projekte aus der Flotte: 0 übernommen, 0 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 0 hier berechnet
graph: asking the host for up to 9 worker processes for 151 projects; the projects wait for them
graph: headroom of 337 MB is below one project (1406 MB), inferring one project at a time
graph: headroom of 284 MB again (237 MB taken), resuming
graph: headroom of 270 MB is below one project (817 MB), inferring one project at a time
graph: headroom of 433 MB again (3 MB taken), resuming
graph: headroom of 162 MB is below one project (1893 MB), inferring one project at a time
graph: headroom of 99 MB again (3 MB taken), resuming
graph: headroom of 94 MB is below one project (1384 MB), inferring one project at a time
graph: headroom of 260 MB again (3 MB taken), resuming
graph: headroom of 129 MB is below one project (1384 MB), inferring one project at a time
graph: headroom of 80 MB again (2 MB taken), resuming
graph: headroom of 76 MB is below one project (629 MB), inferring one project at a time
graph: headroom of 897 MB again (2 MB taken), resuming
graph: headroom of -137 MB is below one project (866 MB), inferring one project at a time
graph: headroom of 957 MB again (228 MB taken), resuming
graph: headroom of 504 MB is below one project (866 MB), inferring one project at a time
graph: headroom of 764 MB again (228 MB taken), resuming
graph: headroom of 593 MB is below one project (866 MB), inferring one project at a time
graph: headroom of 509 MB again (186 MB taken), resuming
```

## `[capacity]` and `graph:` lines of the bootstrap logs (verbatim)

Try 1:

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class, at least 4515 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 5215 MB reserved (7536 MB available, 2048 MB floor)
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class, at least 4515 MB because of its assembly phase
[capacity] run: sized at 46% of the default from 9 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 9 earlier run(s) of its class
[capacity] dependencies: sized at 25% of the default from 15 earlier run(s) of its class, at least 5895 MB because of its assembly phase
[capacity] admitted dependencies with 4729 of 5895 MB: only 4729 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 25% of the default from 15 earlier run(s) of its class, at least 5895 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (8070 MB available, 2048 MB floor)
```

Try 2:

```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class, at least 4511 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 5211 MB reserved (7817 MB available, 2048 MB floor)
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class, at least 4511 MB because of its assembly phase
[capacity] run: sized at 46% of the default from 9 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 9 earlier run(s) of its class
[capacity] dependencies: sized at 25% of the default from 15 earlier run(s) of its class, at least 5896 MB because of its assembly phase
[capacity] admitted dependencies with 5554 of 5896 MB: only 5554 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 25% of the default from 15 earlier run(s) of its class, at least 5896 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7554 MB available, 2048 MB floor)
```

## Samples

| Try | Samples | Pressure level other than 1 | Largest process | Whole run |
|---|---|---|---|---|
| 1 | 49 | 0 | 5774640 KiB, pid 96917, ppid 96823 | 24 min 32 s |
| 2 | 22 | 0 | 5711728 KiB, pid 12493, ppid 96823 | 10 min 37 s |

Replica agent `state = running`, kickstarted 0 times; replica digest `eabbc9226732…`, `bootVersion` 6. No try fell into a window in which the fleet answered nothing because of the server move, as far as the logs show.

## Files

`run-round-10.sh`, `run-round-10b.sh` (comparison try), `round-10.monitor.txt`, `round-10.prerun.txt`, `round-10.launcher.txt`, `round-10.try-<k>.samples.txt`, `round-10.try-<k>.service.txt`, `round-10.tail.txt` (last 200 lines of try 2), `round-10.try-1.tail.txt`.
