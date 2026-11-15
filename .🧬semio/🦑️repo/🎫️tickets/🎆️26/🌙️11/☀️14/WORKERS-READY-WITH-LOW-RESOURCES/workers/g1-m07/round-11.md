# Round 11 on g1-m07

## Machine
- Mac mini, 16 GiB memory, macOS 27.2.1, 107 GiB free on the data volume.

## Replica agent
- `state = running`, `runs = 50`; kickstarted 0 times. Code digest now "code":"eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc". `bootVersion` 6.

## Launcher
- Unchanged before and after all four tries: link -> `versions/cf25f2b751f83336/repo`, target inode 3496898, mtime 2026-11-14T22:55:52 (CET), Identifier `tech.semio.repo`, CDHash `6de7547a8b8ff384bbfbfc45e962462063ff0357` (`round-11.launcher.txt`).
- No `native_rc=4`. The owner's click for the new launcher is still open (launcher receipt was `waiting` at 23:46Z).

## Tries (4 of 10; stopped: three tries in a row with the same error)
| try | log | start | end | native_rc | peak resident of the run | pressure samples (1 / 2 / 4) |
|---|---|---|---|---|---|---|
| 1 | `~/native-bootstrap-round-11.log` | 00:31:52Z | 00:50:56Z | 75 | 3743568 KiB | 38 / 0 / 0 |
| 2 | `…-try-2.log` | 00:53:01Z | 00:58:22Z | 1 | 4136288 KiB | 11 / 0 / 0 |
| 3 | `…-try-3.log` | 01:00:25Z | 01:03:20Z | 1 | 2753616 KiB | 6 / 0 / 0 |
| 4 | `…-try-4.log` | 01:19:35Z | 01:22:35Z | 1 | 2596224 KiB | 6 / 0 / 0 |

No hard stop, no stall of 15 minutes (the stall rule was applied to the "Reading the project graph slice" phase only). All four failed in `workspace:setup` before any task ran: succeeded 0, failed 0, not run: all. Earlier phases passed each time; wall time per phase not measured.

### Try 1 (`native_rc=75`), too heavy project named
```
The project graph could not be read on this machine yet: @semio-tech/framework-os-dev (about 6588 MB) need(s) more memory than this machine can give one project (1756 MB). The inference was stopped before the host ran short of memory and the project(s) are marked too heavy here. Nothing is wrong with your tree; run the command again, the fleet is asked for them first.
```
### Tries 2, 3, 4 (`native_rc=1`): same failure, the round 5 one
Client text: `The resident service met an unexpected failure (reference 29bbd28b)` (try 2, at `assembly 33 of 703`), `(reference dcb6a45b)` (try 3 after `owners 1 of 1`; try 4 at `assembly 0 of 703`). All `request-failed` lines of the service log `host-ac6bfe8f337c.log` since 00:31Z:
```
2026-11-15T00:58:22.439Z request-failed 29bbd28b Error
2026-11-15T01:01:12.786Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
2026-11-15T01:03:20.600Z request-failed dcb6a45b Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/d006dc73e2154c304f0162074879b1bf869eb6df96f7478c317d568f0c4627d3.json'
2026-11-15T01:22:35.270Z request-failed dcb6a45b Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/d006dc73e2154c304f0162074879b1bf869eb6df96f7478c317d568f0c4627d3.json'
```
Stack of dcb6a45b:
```
2026-11-15T01:22:35.270Z request-failed dcb6a45b Error: ENOENT: no such file or directory, lstat '/Users/ueli/Documents/semio/.cache/b/g/m/build-input-closures/d006dc73e2154c304f0162074879b1bf869eb6df96f7478c317d568f0c4627d3.json'
    at lstatSync (unknown)
    at readInventoryRecordLeanV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🪶️lean-inventory/🟨️.mjs:12:48)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📁️workspace/🟦️.ts:86:20)
    at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs:146:491)
    at processTicksAndRejections (native:7:39)
```
- Round 8 says a missing `build-input-closures` file now counts as a miss; on this machine it still ends the request (`readInventoryRecordLeanV1`, `lstatSync`).
- The file `.cache/b/g/m/build-input-closures/d006dc73e2154c304f0162074879b1bf869eb6df96f7478c317d568f0c4627d3.json` exists now: no. Files in that folder now: 15788.
- Between try 3 and try 4, as rounds 6/7 prescribe for this error: `repo cache gc --area=graph-inference` once (01:03:43Z to 01:19:25Z). It ended with `This process (repository:build) made no progress for 13 min and has no live child: it exits now so that the leases it holds are released.` and `repository:command failed with exit code 124`; the folder went from 16803 to 15788 files. Try 4 then failed with the same reference.
- Reference 3420bc16: 0 occurrences in the service log.

### Access denial (the fact asked for in round 10)
Try 3 printed: `The graph service under the launcher was denied access; it is started again under the access of the calling app`. Service log around it:
```
2026-11-15T01:01:12.786Z request-failed da146983 Error: Relative filesystem fstatat failed (EPERM)
    at failure (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148:321)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:153:768)
    at processTicksAndRejections (native:7:39)
2026-11-15T01:01:12.786Z shutdown access-denied
```
Lines `where: root=… entry=… code=…` in that log: 0. The line the round asks for was not written on this machine; `exit.log` shows `start … via launchd, label tech.semio.repo.service.c3e3ffc11699` at 00:37:43Z and a direct start (pid 9714) at 01:01:12Z.

### `graph:` lines of the service log, verbatim (cut at 450 characters; headroom, peer-wait and single stale-file lines left out)
```
graph: project @semio-tech/framework-os-dev (project) grew by 3294 MB, more than the 1756 MB this machine gives one project; its inference is stopped before the host runs short of memory and the project is marked too heavy here
graph: 1 of 48 projects inferred by 5 worker processes in 265 s (0 failed, 0 lost, 0 restarts)
graph: generation 68 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 14 s
graph: No verified change journal cursor is recorded for generation 68; verifying the tree once.
graph: refreshing 1 projects (generation 69)
graph: 56 stale facts (analysis 36, readdir 18, exists 1, readdirNames 1) leave 1 projects stale
graph: os-hub is stale: the workspace file 🌎️hub/📦️packages/🦀️rust/.semio was removed
graph: os-hub is stale: the workspace file 🌎️hub/📦️packages/🦀️rust/.semio/hub was removed
graph: os-hub is stale: the workspace file 🌎️hub/📦️packages/🦀️rust/.semio/hub/db was removed
graph: os-hub is stale: the workspace file 🌎️hub/📦️packages/🦀️rust/.semio/hub/db/catalog was removed
graph: 56 more stale inputs are not listed
graph: headroom of 1024 MB is below one project (1527 MB), inferring one project at a time
graph: generation 69 published, 1 projects refreshed in 226 s (0 kept their edges)
graph: fact index of generation 69: 738523 facts of 5101 projects, 84.5 MB expanded (16.5 % of the 512 MB limit), 13.5 MB packed
graph: 3 derived results asked again after 56 stale facts: 2 unchanged, 0 changed
[script] graph:slice-worker · Assembling the graph: 0 of 703 projects
[script] graph:slice-worker · Assembling the graph: 33 of 703 projects
[script] graph:slice-worker · Assembling the graph: 65 of 703 projects
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 12 s
graph: No verified change journal cursor is recorded for generation 74; verifying the tree once.
graph: refreshing 1 projects (generation 75)
graph: 47 stale facts (analysis 36, readdir 11) leave 1 projects stale
graph: 47 more stale inputs are not listed
graph: headroom of 1024 MB is below one project (1765 MB), inferring one project at a time
graph: generation 75 published, 1 projects refreshed in 86 s (0 kept their edges)
graph: fact index of generation 75: 758321 facts of 5101 projects, 95.2 MB expanded (18.6 % of the 512 MB limit), 13.8 MB packed
graph: 1 derived results asked again after 47 stale facts: 0 unchanged, 0 changed
graph: generation 74 (5100 projects) restored from the remote cache, 5104 documents (48 MB) downloaded in 11 s
graph: headroom of 1021 MB is below one project (1765 MB), inferring one project at a time
graph: generation 75 published, 1 projects refreshed in 95 s (0 kept their edges)
```

## End of the round
- Not ready, marker `.cache/f/native-bootstrap.done` absent. Stopped after try 4 because tries 2, 3 and 4 failed with the same error and the one allowed gc did not change it. Waiting for a higher round.
