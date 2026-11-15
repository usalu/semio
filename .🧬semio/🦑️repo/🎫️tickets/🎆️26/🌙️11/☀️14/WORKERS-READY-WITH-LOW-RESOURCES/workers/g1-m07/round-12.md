# Round 12 on g1-m07 (pause, read-only, 2026-11-15T01:30:27Z)

No bootstrap was run and none was running. Last try: round 11 try 4, `native_rc=1` (see `round-11.md`).

## 1. `graph:` lines of the service log of the last try (`host-ac6bfe8f337c.log`), verbatim
23 distinct lines match (duplicates removed). Left out: 22 lines of the forms `graph: waiting for the graph build of a peer (… generation N was the newest when this command queued)` and `graph: a peer builds the graph; …`, which only repeat with a growing number of seconds.
```
graph: No verified change journal cursor is recorded for generation 61; verifying the tree once.
graph: 659 stale projects, 0 taken from the fleet, 12 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 27770 MB), repo (about 14264 MB), workspace (about 13780 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os (about 8368 MB), and 6 more; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 659
graph: refreshing 659 projects (generation 62)
graph: breach-_framework_products_repo_script_ts is stale: the workspace file 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/📈️adaptive-waves/🟦️.ts was added
graph: project @semio-tech/framework-rs (language:cargo) grew by 1883 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: No verified change journal cursor is recorded for generation 62; verifying the tree once.
graph: refreshing 611 projects (generation 63)
graph: project @semio-tech/framework-os-kernel (language:cargo) grew by 2376 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: 611 stale projects, 0 taken from the fleet, 12 too heavy for this machine (@semio-tech/framework-os-dev (about 76010 MB), @semio-tech/repo-lib (about 19514 MB), repo (about 14264 MB), workspace (about 13896 MB), @semio-tech/framework-plugin (about 8626 MB), @semio-tech/framework-os-kernel (about 6984 MB), and 6 more; it can give about 3512 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 611
graph: project @semio-tech/framework-os-dev (project) grew by 2811 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: project @semio-tech/framework-os-dev (project) grew by 3294 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: generation 68 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 14 s
graph: No verified change journal cursor is recorded for generation 68; verifying the tree once.
graph: refreshing 1 projects (generation 69)
graph: generation 69 published, 1 projects refreshed in 226 s (0 kept their edges)
graph: fact index of generation 69: 738523 facts of 5101 projects, 84.5 MB expanded (16.5 % of the 512 MB limit), 13.5 MB packed
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 12 s
graph: No verified change journal cursor is recorded for generation 74; verifying the tree once.
graph: refreshing 1 projects (generation 75)
graph: generation 75 published, 1 projects refreshed in 86 s (0 kept their edges)
graph: fact index of generation 75: 758321 facts of 5101 projects, 95.2 MB expanded (18.6 % of the 512 MB limit), 13.8 MB packed
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 11 s
graph: generation 75 published, 1 projects refreshed in 95 s (0 kept their edges)
```

## 2. Newest three service logs and the replica code digest
```
-rw-r--r--@ 1 ueli  staff   359363 Nov 14 21:22 /var/folders/xq/z6t8hmcd0qj871bx0sj7pbn80000gn/T//semio-repo/501/host-293b476f6b9d.log
-rw-------@ 1 ueli  staff   259813 Nov 15 00:48 /var/folders/xq/z6t8hmcd0qj871bx0sj7pbn80000gn/T//semio-repo/501/host-9888631afea4.log
-rw-r--r--@ 1 ueli  staff  2594466 Nov 15 02:30 /var/folders/xq/z6t8hmcd0qj871bx0sj7pbn80000gn/T//semio-repo/501/host-ac6bfe8f337c.log
```
Replica code digest (`.cache/f/health.json`): "code":"eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc"
"code":"52e46eee5c577cf8425b43f243681f24326d6645807f938982368d8c64cbfc7f"

## 3. Machine state now
- Memory pressure level (`kern.memorystatus_vm_pressure_level`): 1
- Swap: total = 2048.00M  used = 942.44M  free = 1105.56M  (encrypted)
- Five largest processes (rss KiB, pid, parent pid, age, command):
```
1070368 65849 63552    01:40:10 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M^[M^MM-oM-8M^Oproducts/M-pM^_M-&M^QM-oM-8M^Orep
522000 45001 44969    18:32:21 /Applications/Claude.app/Contents/Frameworks/Claude Helper (Renderer).app/Contents/MacOS/Claude Helper (Renderer) --type=renderer --user-data-dir=/Users/ueli/Library/Application Support/Claude --standard-schemes=cowork-artifact,c
464896 20784 45003       00:04 /Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.hooksPath=/dev/null -c filter.lfs.clean= -c filter
317440 18188 18185       10:12 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'),crypto=awa
313696 91741 91740    14:43:08 /Users/ueli/Library/Application Support/Claude/claude-code/2.1.325/485fdac4332e/claude.app/Contents/MacOS/claude --output-format stream-json --verbose --input-format stream-json --effort=medium --model=claude-fable-5-2 --permissi
```
- `repo service host` / `repo show projects` processes (rss KiB, pid, parent pid, age), not stopped:
```
271392 18185     1       10:13 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M^[M^MM-oM-8M^Oproducts/M-pM^_M-&M^QM-oM-8M^Orepo/M-pM^_M^TM-(M-oM-8M^Omodules/M-pM^_M^SM
317440 18188 18185       10:12 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'),crypto=await import('node:crypto'),awaitModule=awa
```
(empty block = none running)
