# 🐧 W19: Linux and Windows workers (g1-m04, g1-m05), 2026-11-25 08:54 to 09:12

I read everything from the coordinator: `/Users/Shared/semio-store/v1/origins/g1-m0{4,5}/` (attest.json, view.txt, ready.json, ready-timings.json, ready-log-*, segments), the scheduler table (`GET :8981/v1/fleet/load` through `readRemoteExecutionFleetFactsV1`) and the replica's own view diff (`viewDiffV1`, `fleetReportV1`). I did nothing on a worker and restarted nothing. Probe scripts: `W19/view-diff.ts`, `W19/fleet-load.ts`, `W19/fleet-load-raw.ts`, `W19/closure-spelling.ts`, `W19/closure-newest.ts`. Landings: `T/landings.md` "W19".

`repo fleet status` itself fails right now with `The fleet report broke its own schema: /cas/leases/expired must be at least 0` (`🚢️fleet-status/📥️readers/🟦️.ts:216` passes a negative CAS census through). That belongs to the CAS lanes and I did not fix it. Everything below comes from the readers the command uses.

## 1. g1-m04 (linux-amd64)

### "replica says diverged", 2,742 files only at g1-server: this was a view defect, not divergence
- m04's tree is identical to the fleet's. Its attest `manifest` is `d41e1bd7…`, the same value as m01 and the other macOS workers, and it has the same `files` (470,467), `indexTree` and HEAD.
- What differs is the git-status view. On the coordinator and m05, every path that was staged and then deleted from disk appears twice, as `AD p` and as ` D p`. On m04 it appears once, as `AD p`. Counted: `🔁️replica/sa/` has 2,737 `AD` plus 2,737 ` D` lines on g1-server and m05, but only 2,737 `AD` lines on m04. The other 4 of the 2,741 lines are 3 ticket files and `-inf`. That accounts for the whole gap: `deleted` is 5,495 on macOS and 2,754 on m04.
- Cause: `🌿️git/🟦️.ts` `gitViewV1` runs `spellingV1` on case- or normalisation-insensitive volumes (APFS, NTFS) and passes every tracked path missing from disk as `extra.deleted`. `parseGitViewV1` then added ` D p` whenever that exact line was missing, even when git had already printed `AD p` for the same path. Linux takes the plain branch, so it never adds the extra line. Any staged-then-deleted path therefore made every Linux machine look "diverged". The 2,737 index entries left behind by the SA1 stray `sa/` folder triggered it here.
- Fix (landed): a spelling-absent path now becomes the single line `<index column>D p`, and git's own line for that path is dropped. This is exactly what Linux git prints: `AD`/`MD` stay one line, and a path absent through a case variant is still ` D`. The `gitViewV1` docstring was updated to match.
- Tests: a pure parser test, plus a real-git test (stage two files, delete the folder, expect `AD …` twice on every volume). On APFS this goes through the spelling branch, which is the failing case. git 69/0, attest 146/0 (Python oracle included).
- Effect: once each replica runs the new code, the coordinator and the macOS and Windows views drop their 2,737 duplicate lines and m04's verdict turns `ok`.

### "CODE 9a5947d500a861f1 differs from the tree's 36d62b285841a7a0"
- This code is the agent sidecar's closure digest, taken once at start (`👷️agent/🟦️.ts:50`, `baseline`). The tree's code is the closure the coordinator computes now. The replica code on m04 (`attest.code 8493edad…`) matches every other machine.
- I checked whether the closure would differ on a case-sensitive disk. Every one of the 988 files resolves to an exactly spelled name (0 inexact). All files are mode 644. The bun version is pinned to 1.4.2. The newest change in the closure is from 06:38Z (`🎚️mode/🦀️profile`). So the same content should give the same digest, and the likely reading is that m04's agent has not restarted since then.
- The coordinator cannot see why. Possible reasons are the restart policy (30 min minimum interval, 30 min maximum deferral, 5 min grace, up to 2 h drain), or a successor that dies on Linux before it listens. The sidecar logs are local to m04. Not fixed: see open item 3.
- Side findings from m04's attest: `service-definition` failed 10 times (last 08:07; m05 too, macOS never), probably `needs-privilege` for the systemd unit (`🗺️topology/🛠️service/🟦️.ts:183`), which needs a one-time `sudo sh` by a person. m04 also published `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/Cargo.lock` at 08:05, a worker rewriting a lock file in the shared tree. m05 did the same with root `Cargo.lock` at 07:22 and `🌎️hub/Cargo.lock` at 08:19.

### Disk: "needs 6144 MB, 3609 MB free"
- The requirement is right, so I did not change it. `diskRequirements.dependencies` = 6,144 MB (`🚦️ready/🧬️schema/🔣️.json:85`) is a declared value, because m04 has never had an `ok` run. Measured cold-install growth on macOS workers with clonefile sharing was 8,556 MB (m02), 6,517 MB (m03), 6,208 MB (m11) and 3,482 to 3,526 MB (m08, m13). Linux copies instead of cloning, so 6 GiB is a floor, not an overstatement. W8's install model for Linux is 29.5 GiB.
- Collection really ran: all 16 collectors ran and each freed 0 B (ready.json tail). The only semio consumer listed is `.cache/b/r/m/cargo` (2,178 MB, the cargo store). Collecting it would leave about 5.8 GB, still short, and the stage would fill it again. The other roughly 460 GiB of `/home` (475,758 MB total) belong to the host's own `deck` user.
- Verdict: `disk-unfit` is correct and said once (W8 logic). m04 becomes Ready only after a person frees about 2.5 GB, or realistically about 25 GB for the whole install, on that host.

## 2. g1-m05 (windows-amd64)

### Code 9511dcd9af5b3764 can never equal the tree's code: separator defect
- `🛰️sidecar/🧮️closure/🟦️.ts:45` named each file in the digest by `path.slice(root.length)`, which on Windows is `\🧰️framework\…`. So the Windows agent's digest differs from macOS and Linux even with identical bytes, and `repo fleet status` will always flag m05.
- Fix (landed): digest lines use `/` separators. The digest value on macOS and Linux is unchanged. New test: a Python `os.walk` oracle computes the same digest from the bytes. closure 6/0.

### Replica lag ("lagging", 22 files short, 79 view lines)
- m05's apply latency is p50 39.5 s and p95 90 s (macOS: 0.3 s and 1.4 s); its hot tier makes 1,852 passes against about 66,000 on macOS. Its `manifest 4bf6a5…` differs and it has 22 fewer files, which is lag rather than a defect: no delete or put conflict is visible, and the 79 differing lines are edits in flight (tickets, os modules). Windows file I/O is the limiter; I could not measure deeper from here.

### Dependencies stage: why it fails or stalls for hours
Timeline (`ready-timings.json`): attempts 38 to 44, with runs of 702 to 15,604 s; outcomes include `stage-timeout` twice (9,585 s and 15,251 s, the second at the hard cap), `stalled` 15,604 s at `dependencies:workspace-edges`, and `stage-failed`. The current run has been going about 4.5 h (`still running after 16082 s`, last phase `source.served`, latest activity a bun child for 12 or more minutes).
1. **Hold-and-wait refusal against the idle collector (fixed).** In `ready-log-dependencies-failed.txt` (02:56Z), the producer held `javascript-dependency-build:C:\semio` exclusively and asked for `javascript-dependency-pin:C:\semio:2a5cd89d850242461f8a` shared. `cache gc --idle` (pid 56840) held that pin for 62 s while it deleted that generation. The lease order put both leases in the `namespace` class, so the producer was refused after the 15 s grace, after 2,545 s of work. A pin is held exclusively only by `tryConcurrentPinExclusiveV1` (`🚦️concurrent/💾️storage/🟦️.ts:85`, `timeoutMs:0`), which never waits. That is exactly the contract of the existing `pin` class. Landed in `🔒️leases/🧭️order/🔣️policy.json`: `javascript-dependency-pin:` moved into class `pin`, and a new edge `namespace -> pin` bound to held prefix `javascript-dependency-` with EN/DE reasons. The producer pins first and checks existence afterwards (`💾️storage/🟦️.ts` `pinJavascriptDependencyGenerationsV1`, then the generation entry check), so after waiting it simply rebuilds the deleted generation. Test: build held plus pin request is allowed; another exclusive hold still refuses, and so does the bindings lease. order 79/0, wait-for 6/0, component-driver 24/0.
2. **Slow ownership walk on Windows (open).** In the same log, `store-ownership:readText` advanced 64 steps in 168 to 266 s at times (`recentPerSecond 0`), while a whole walk took 1,533 s against about 30 s for the same counts at the start. Every 64th step publishes a lease activity and checks the frame (`💾️storage/🟦️.ts:958`). The stall is in reading package manifests or in that publication on NTFS. I need a trace on m05 to tell which; I did not guess.
3. **Edge checking hit by peers' edits (open, from W8).** `dependencies:workspace-edges` reached counter 162,941 in a 15,604 s stall, and an earlier run ended `Declared provider descriptor changed`. A Windows run is long enough to be invalidated by the fleet's edits.
4. **Units 0 hit / 154 miss, `upload-expired 521`.** The Windows key (x86_64-pc-windows-msvc toolchain `20b3468…`) can only hit units another Windows machine uploaded, and m05 is the only one, so 0 hits is expected. The 521 expired uploads are spool jobs older than `maximumJobAgeMilliseconds` (6 h) at the uploader's next pass: m05's last key sample dates from 2026-11-23 23:42, so the uploader did not run for days on a non-Ready machine. The jobs expired while `client.usable()` was true (`📤️uploader/🟦️.ts:199`), so this is not a denial.

## Changed files
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🌿️git/🟦️.ts`, `…/🌿️git/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🧮️closure/🟦️.ts`, `…/🛰️sidecar/🧪️tests/🧮️closure/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🧭️order/🔣️policy.json`, `…/🧭️order/🧪️tests/🟦️.ts`

## Tests (one `repo test-file` at a time, load 15 to 18)
git 69/0, attest 146/0, sidecar closure 6/0, lease order 79/0, wait-for 6/0, component-driver 24/0, ground 4/1. The ground failure is not related to this change: the image-mapping `waitBound` message lacks its German half.

## Open items
1. `repo fleet status` is broken by a negative CAS census (`expired` < 0): CAS lane.
2. m04 needs host disk freed by a person (at least about 2.5 GB for the stage, about 25 GB for the install), plus the systemd `needs-privilege` step.
3. m04 agent code: to tell a stale agent from a different closure, the agent load report should carry its current closure digest (and the restart verdict) next to `code`. That is a scheduler-schema change, which I did not land while play `site-build` (08:41) is running.
4. m05 ownership-walk `readText` stalls and long edge checks need a trace on the machine.
5. Workers rewrite `Cargo.lock` files in the shared tree (m04 `◻️2d/Cargo.lock`, m05 root and `🌎️hub/Cargo.lock`).
6. The coordinator's index still carries 2,737 staged `🔁️replica/sa/` entries (SA1). They are harmless for the view now, but a person should unstage them.
