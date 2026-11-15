# Result R7: a graph service without access to the worktree (EPERM, exit 4) on macOS workers

Landed 2026-11-15 05:1x CET under `T/landing.lock` (T = `FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`), 21 files changed + 2 new, `N/LAND-OK` created and removed again, recorded in `T/landings.md`, originals in `T/🗑️generated/R7-landing-*` (`land.py --rollback`). Script and payload: `T/🧰️scripts/R7/{land.py,payload.tar,build_payload.py}`. No graph or `repo` build command was run (none on this machine).

## Cause, with file and line (code as it was before the landing, `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`)

What the evidence proves and what it does not:

1. **Why no `where:` line ever appeared (proven).** The failing call is made in the slice load, which runs in a forked process (`B/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧵️process/🟨️.mjs`, `fork`). That process sent only `{message, stack}` to the service (`…/👷️slice-pool/👷️worker.ts:23`) and the service rebuilt the error with `Object.assign(new Error(message.message), {stack: …})` (`…/👷️slice-pool/🟨️.mjs:50`). `root`, `entry`, `code`, `errno`, which `failure()` in `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148` puts on the error, were dropped at that line. `deniedPlaceOfV1` (`B/🏡️resident/🏛️service/🧯️failure/🟨️.mjs:70`) therefore saw an error without `root` and printed nothing. The stack columns in the worker logs (`🟦️.ts:148:321`, `153:768`) equal the landed descriptor file exactly, so the workers did run the code that sets `root`/`entry`.
2. **Why a service without access was accepted (proven).** A service proved its access with one `readdir` of the checkout root: `B/🏡️resident/🏗️composition/🌐️host/🟨️.mjs:34` (`gateConsentV1`) calling `probeConsentV1` (`B/🏡️resident/🔐️consent/🟨️.mjs:37-41`), run in the service process only. The entry that fails is a nested `fstatat` made by the forked slice worker during `build:source-admission` (log lines `[script] build:source-admission` directly before every EPERM). So a service passed its gate, wrote a receipt `service.json: ok`, bound its socket and was reused by every client (`connectHostV1` takes whatever answers on the endpoint, `B/🏡️resident/🚪️client/🛰️plan/🟨️.mjs:61-62`) until the first request reached the nested place.
3. **Which route started the failing service.** g1-m07, try 2 (`workers/g1-m07/round-15.md`): `exit.log` has `{"event":"start","at":1794713729203,"pid":24332,"placed":false}` with **no `via:"launchd"`**, i.e. the second service (the one that was denied again at 03:37:55) was route (b), a direct child of the caller (`SEMIO_SERVICE_LAUNCHD=off` set by the retry in `B/🧭️invocation/🛰️plan/🟦️.ts:98`). The first one (exit row of pid 15306 at 03:35:29) has no start row in the evidence; whether it was route (a) or (b) is **not provable from the archive** (the ppid of the failing process was not logged). g1-m02 round 17: both start rows are missing from the archive too. So on these two machines the claim "the second service was started under the caller's access" holds, and it was denied as well. That is the new fact: **the denial does not depend on the launchd identity**; the caller's own `readdir` of the root succeeds (m02: `ls ~/Documents/semio` right after) while a nested place is refused to the process that does the work.
4. **Why exit 4 although route (b) exists (proven).** The retry reported the second denial as `consent`/exit 4 unconditionally (`B/🧭️invocation/🛰️plan/🟦️.ts:98` `catch(second)=>refusalOfPlanV1`), without asking whether the caller itself can read; the denied first service was also only waited for (`waited<30` x 100 ms) and never retired.
5. **Not established: which entry** is refused and why (TCC attribution of a forked detached child, a place below `.cache`, something else). The error never named it. This is exactly what the next failure will print (below).

## Fix

- **The service proves its own access before it binds** (`B/🏡️resident/🔐️consent/🟨️.mjs`, `…/🏗️composition/🌐️host/🟨️.mjs`): `probeConsentV1` now reads the root and then samples below it with the identity of the service: `lstat` of every top-level entry, listings of the first directories down to two levels and of the cache images under `.cache` down to four levels, bounded to 400 entries and 24 listings (measured 6 to 30 ms on this 111-entry root). Below the root only `EPERM` counts (the answer of the consent); `EACCES` is an ordinary permission and never blocks a build. The same sample then runs in a **forked, detached child** of the service (`sampleAccessInChildV1`, new `🔐️consent/🔎️sample/🟨️.mjs`, 26 ms), the shape of the slice worker; a child that does not answer in 8 s is no refusal. A refusal ends the service **before it binds**: exit 4, exit row `access-denied` with `place`, `route`, `pid`, `ppid`, `by:"worker"` where the child found it, receipt `service.json` with `place`/`route`/`ppid` (receipt schema extended), nothing loaded, no endpoint.
- **Route, pid, ppid everywhere**: the start sets `SEMIO_SERVICE_ROUTE` (`launchd` in the job's property list, `direct` for the child; `foreground` otherwise).
- **The worker error keeps its data** (`🧵️process/🟨️.mjs` `errorMessageV1`/`errorOfMessageV1`, used by `worker.ts` and the pool): `code`, `errno`, `root`, `entry` and the refusal cross the process boundary.
- **Every EPERM carries the `where:` line** (`🧯️failure/🟨️.mjs`): `where: root=<root> entry=<entry> code=EPERM route=<launchd|direct|foreground> pid=<n> ppid=<n>`; an EPERM that names nothing prints `root=? entry=? code=EPERM …`. The refusal text (EN/DE) names the place, route, pid and parent (`access-denied`, `consent-denied` now have `{place}`, schema texts changed, equal placeholders).
- **The client retires and replaces once** (`B/🧭️invocation/🛰️plan/🟦️.ts`, new `retireHostServiceV1` in `🚪️client/🛰️plan/🟨️.mjs`): first consent refusal, caller can read (deep sample) -> marker dropped, the service is retired by `SIGTERM` to the pid its status names and awaited, one bilingual line, one new service with `SEMIO_SERVICE_LAUNCHD=off`. A second consent refusal is **exit 4 only if the caller's own sample is denied then**; otherwise the failure `build.graph.service-denied` (exit 1, EN/DE "although this process can read it") that carries the service's own text with the `where` facts. The caller cannot read in the first place: exit 4 at once, no second attempt (unchanged).

## Proof (bun 1.4.2, `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`, first on an overlay under the scratchpad, then live by `land.py` before/after the landing)

| Law | Test file / name | Result |
|---|---|---|
| a launchd service without access is never adopted/never serves, for each of the three routes: exit 4, exit row and receipt with place/route/pid/ppid, no endpoint, nothing loaded | `🏡️resident/🧪️tests/🛡️consent-service` "a service without access to the worktree never becomes the shared service…" | pass |
| the child of the service (the slice-worker shape) refused a place the service can read also blocks the start | same file, "…forked child…" | pass |
| an ordinary EACCES below the root is no consent problem, the service serves | same file, "…really unreadable nested directory (an ordinary permission, EACCES)…" | pass |
| the service learns its route (`direct` for a real child, `launchd` in the plist), client retires by pid, waits, treats no service as retired | same file | pass |
| nested EPERM named, EACCES ignored, sample bounded (5000 names -> <= 400 touched), real child process, python3 as independent reader sees EACCES (errno 13) on the same directory | `🏡️resident/🧪️tests/🔐️consent` | 13 pass |
| every EPERM has `where:` with route/pid/ppid, text has the placeholders in EN/DE | `🏡️resident/🧪️tests/🧯️failure` | 8 pass |
| a real forked child over the real IPC channel keeps root/entry/code; the old message loses them (the cause, demonstrated) | `🏡️resident/🧪️tests/🏷️worker-error` | 2 pass |
| client falls back once, retires before the 2nd start (order `probe, ask, probe, invalidate, retire, ask`), 2nd denial with readable caller = `build.graph.service-denied` (not exit 4, EN/DE), 2nd denial with unreadable caller = exit 4, unreadable caller = no 2nd attempt | `🧭️invocation/🧪️tests/🛰️plan` | 9 pass, 2 red (see below) |

Live, around the landing (every `🏡️resident/🧪️tests/*/🟦️.ts` plus texts, plan, error module): all resident files 0 red before and after (consent 13, launchd 9, consent-service 17, failure 8, client 8, cancel 37, protocol 23, host/service/watch/provision pass live), texts test pass with the whole-source scan, error module pass. **Red before and after, not mine:** plan "a real client gets its plan and verified graph slice from a real service, and a corrupted slice is refused" (`Plan slice does not match its digest`; the landing script printed NEW RED only because the title carries a timing). Modules parse: `bun build --no-bundle` of all 18 touched sources and `JSON.parse` of the 4 changed schema/catalog files, and the live `import` of every touched module after the landing: ok.

## What a worker's log must show next

After the worker's replica has taken the code (digest changes; the service of the old code retires by the implementation tag) and the retried bootstrap runs:

1. **Healthy**: no `request-failed … EPERM`, no `access-denied`, `native_rc` not 4.
2. **The refusal is now at the start**, in `host-<tag>.log` and in `exit.log` of the runtime directory, as one line before the service binds:
   `access-denied denied Documents /Users/…/Documents/semio place=<the refused path> [by=worker] route=<launchd|direct> pid=<n> ppid=<n>`
   and the client prints the consent text with `<the refused path> is refused` (exit 4 only if the caller itself is refused at that place).
3. **If it still happens mid-request**, the service log has the failure directly followed by
   `where: root=<root> entry=<entry> code=EPERM route=<route> pid=<n> ppid=<n>`.
4. **If the caller can read but both services are denied**: the bootstrap ends with `native_rc=1` and the text "The graph service was denied access to the checkout although this process can read it …", not with 4.

**Line to copy if it still fails**: the `access-denied …` line of `host-*.log` (or the `request-failed …` line **and** the `where:` line below it), plus `tail -n 4` of the `exit.log` next to that log, plus, for the `place=` or `entry=` path: `ls -leO@ "<path>"` and `python3 T/🧰️scripts/P6/tcc-attribution.py --last 10m`. The place and the route answer whether it is the launcher identity (`route=launchd`), the caller (`route=direct`, check `ppid`) or the forked worker (`by=worker`).

## Not verified / what others must know

- **Not run on a real worker**: no TCC dialog or real EPERM exists on this machine, so the nested EPERM is injected (fake `stat`/`list`, fake child script). Whether the deep sample really catches the m02/m07 entry depends on that entry being among the sampled places (root entries, 24 listings, `.cache` four levels); if it is a deeper place, the mid-request `where:` line now names it and the retry is the same as before (but exit 1, not 4).
- The schema catalog hashes of the changed schema files (resident texts, consent receipt) are stale until `repo ./📜️script.ts schema generate` can run.
- `SEMIO_SERVICE_ROUTE` is new environment surface: tests/clients that print the service environment will see it.
- The plan test has two reds in the overlay (`the commands that plan a task graph…` is an overlay artefact: it passes live; `a real client gets its plan…` is red live too).
- Not touched: the service host (`🏛️service/🌐️host/🟨️.mjs`), the launchd module, everything of R5/R6; shared-worktree files untouched, so no replica lock was taken.

---

# R7b: the refused entry is `.cache/b/.reset.json`; why the retry did not rescue the run

Evidence: `workers/g1-m02/round-22.{service-try-1,samples,p12-try-1}.txt`. `where: root=…/.cache/b entry=.reset.json code=EPERM route=launchd pid=86077 ppid=86049` is followed in the same service log by `repository:command failed with exit code 4`, then `build:implementation-source`, `Internal error: Relative filesystem fstatat failed (EPERM)`, `exit code 1`; the client printed `The graph service could not answer (lost the service exited: access-denied (code 4) …)`, the try ended `native_rc=1`.

## Causes (R7 code as landed first; `B` = `…/🏗️build/🏡️resident`)

1. **The proof never asked for the marker.** `.reset.json` is looked up by the cache image opening in every image directory (`⚡️caching/…/📥️opening/♻️reset`, schema `intent`); it normally does not exist and the build expects `ENOENT`. My sample only `lstat`ed names a listing returned, so it never made that call. A process that may list `.cache/b` but not look names up in it answers `EPERM` for the absent name.
2. **The retry never ran.** The service closed itself and the connection dropped; `exchange()` in `B/🚪️client/🛰️plan/🟨️.mjs` made that `HostUnavailableErrorV1("lost")`, and the retry in `requestInvocationPlanV1` fires only for a `PlanRefusedErrorV1` with code `consent`; `lost` went to `graph.host-unavailable` (exit 1). The attempts loop of `requestPlanV1` then started a **new launchd service** for attempts 2 and 3, which hit the same marker: that is the `Internal error … exit code 1` after the first `exit code 4`. The closing exit row of a mid-request refusal had no tag, route or root, so `connectHostV1` could not recognise it either. The direct route was never tried in this run.

## Fix (landed 06:41 CET, `T/🧰️scripts/R7b/`, 11 files)

- **Proof across the mount boundary** (`B/🔐️consent/🟨️.mjs`): in every directory of the `.cache` subtree the sample first asks for `.reset.json` (ENOENT fine, EPERM = denied, place = that path), in the service and in the forked child; the gate accepts `list`/`stat` for tests and the service host gets the real checkout as `access`. A service that fails never binds.
- **Retry that rescues**: a dropped connection waits up to 1 s for the exit row; with a row `access-denied` of this tag the client gets the typed consent refusal (`servicePid` = the row's pid), so the existing path runs: caller's own sample, retire exactly that pid, one direct service under the caller (`SEMIO_SERVICE_LAUNCHD=off`). The closing row of a mid-request refusal now has `tag`, `route`, `root`, `folder`. Exit 4 only if the caller's own sample is denied, else `graph.service-denied`.
- **Remembered for the boot** (`rememberLaunchdRefusalV1`/`launchdRefusalV1`, `launchd-refused.json` beside the receipts, schema `LaunchdRefusal`): written by the client when the denied service's row says `route: launchd`; valid only after the machine's boot, for the same checkout and the same launcher bytes, at most 24 h; `spawnHostServiceV1` then skips launchd and prints one line (EN/DE): "The graph service runs under the access of the calling app: the launchd route was refused for <place> earlier since this machine started".

## Proof

consent 15 (fake `stat`: EPERM at `.cache/b/.reset.json` and `.cache/f/…` denied, ENOENT and EACCES fine, marker asked before the entries; memory: Ajv schema, owner-only, same boot, other boot, a day, future, other checkout, other launcher bytes, forget), consent-service 25 (launchd-route service stops at the marker with row and receipt naming it, serves with ENOENT; a raw socket server dropping the connection after writing the row gives `PlanRefusedErrorV1 consent` with pid, no row stays `lost`; a real foreground service refused mid-request leaves a row with tag/route/root/folder and the client text names the entry; remembered refusal makes no `launchctl` call, starts a direct child with `SEMIO_SERVICE_ROUTE=direct`, one line in both languages), plan 12 (remembered only for `launchd`, before the retire). Live after the repair: consent-service 25, consent 15, client 8, failure 8, host 36, plan 12, all pass. Parse proof of all 11 files (`bun build --no-bundle`, JSON parse).

## Incident

At 06:42 someone wrote `🏗️composition/🌐️host/🟨️.mjs` and the resident index from a copy older than my landing, removing my edits (their change adds `reload`; last writer wins). `T/🧰️scripts/R7b/reapply.py` re-applied mine under `T/landing.lock`. If the old behaviour shows again: `grep -c "launchd route was refused"` in the composition host.

## Next in the log / not verified

Next failure shows at the start `access-denied denied Documents <root> place=<root>/.cache/b/.reset.json [by=worker] route=… pid=… ppid=…`, then "started again under the access of the calling app" and a service with `route=direct`; later commands print the "launchd route was refused … earlier since this machine started" line. If the **direct** service fails at the same entry, the end is `native_rc=1` with `graph.service-denied` and the `where:` line: copy that, plus `ls -leO@ <root>/.cache/b`, `mount | grep .cache/b`, `hdiutil info`. Not verified: that the direct route really reads `.cache/b/.reset.json` (no worker access; why an absent name in that directory answers EPERM is not proven, the volume privacy class is the working hypothesis); schema catalog hashes still stale.

---

# R7c: why R7b did not show on g1-m02 round 23, and what changed

Read: `workers/g1-m02/round-23.{service-try-3,samples,p12-try-3}.txt`, the code. The client side of the bootstrap log (`native-bootstrap-round-23-try-3.log`) is **not** in the evidence, so what the two clients printed cannot be read; two runners ran try 3 at the same time (`TRY 3 START` 06:07:13 and 06:07:15, two `END` lines, same log file).

## What the files settle

1. **`route=launchd` with `ppid` 1235 / 13956 is not a wrong label.** The label comes from `SEMIO_SERVICE_ROUTE`, which only `startViaLaunchdV1` sets to `launchd` in the job's property list (the direct child gets `direct`). A job of the per-user launchd is a child of that user's launchd or of the launcher's wrapper, not of pid 1; pids 1242 and 13960 are consecutive-after-wrap numbers of that machine (the sampler shows pids around 11000 at 06:07), so these two are different generations, not one old process.
2. **The two denials are two different services: tags `e957b3fd40e8` (06:09:17) and `d71e909e3d92` (06:11:29)**, the second with a fresh log from byte 0. The implementation tag is a hash of the files `HostPolicy.serviceRevisionFiles` names; it changed between the two, so something in that list changed at about 06:10Z (not mine). **No direct-route service of tag e957 ever logged a failure after 06:09:17**; so either the retry did not run in that client or it ran and ended otherwise. This cannot be decided without the client log.
3. **Defects found by reading the code** (all fixed):
   - The memory "launchd refused for this boot" was written **before** the denied service had finished closing: the client read the closing exit row right after the refusal frame, when the row (`record()` runs after the drain) usually does not exist yet, so `denial?.route==="launchd"` was false and nothing was remembered. Now: retire first, then read the row, then remember; a missing row counts as launchd (the caller can read, the service could not), only an explicit `direct`/`foreground` route is not remembered.
   - **The access files were not part of the service revision.** `serviceRevisionFiles` held the composition host, the slice pool, the service host and the schema, but not `🔐️consent/🟨️.mjs` (the proof), its sample child, the failure module (the `where:` line), the launchd module and the process worker module. A change to only those files did not replace a running service, so R7b's consent module (05:42Z) ran together with an older composition host in a service started between 05:42Z and 05:47Z. They are now in the list (resident schema, one line), so every change of the access code starts a new service.
   - **A start-time proof cannot see a mount made later** (the coordinator's point, confirmed by `.cache/b/.reset.json` being absent or unreadable only once the image is attached).

## Fix (landed 07:18 CET, `T/🧰️scripts/R7c/`, 9 files, no new red; tests live: consent 16, consent-service 28)

- **Proof repeated when a mount appears** (`B/🔐️consent/🟨️.mjs` `mountSignatureV1`, `createMountReproofV1`; `B/🏗️composition/🌐️host/🟨️.mjs`): the device ids of the root, `.cache`, `.cache/b` and the directories below it (or the error code of a place the system refuses) are recorded when the start-time proof ran; **before every graph or slice load** the signature is compared and, only when it changed, the whole proof (service probe, then the forked child) runs again. A refusal is thrown as the same `EPERM` error with `root`/`entry` of the refused place, so it takes the typed path: `where:` line, refusal `consent`, closing row with tag/route/root, service closes, client retries once directly. A mount that appears *during* a load (attached by that same load) is still seen only by the `EPERM` that load meets: the typed path is the same, one request later for the proof, not for the refusal.
- **One log line proves the code**: at every service start `access-proof version=3 root=… protected=Documents sample=ok reproof=on route=<launchd|direct|foreground> pid=<n> ppid=<n>`; when a mount change triggers the repeat: `access-proof version=3 repeated: a mount changed since the start and <place> is refused (EPERM[, by=worker]) route=… pid=… ppid=…` (or `… and everything is readable`).
- **Memory after the retire** and **revision files** as above.

Proof: consent "mount that appears after the start" (signature changes with an attach, same signature is not re-proved, a refusal by the service probe and by the forked child are named, version 3); consent-service "mount appears after the service started" (a real foreground service: start line with version/route/pid/ppid; first request served; after the fake attach the next request ends as `PlanRefusedErrorV1 consent` naming `.cache/b/.reset.json` in both languages, `where: root=…/.cache/b entry=.reset.json code=EPERM route=… pid=…`, closing row, exit 4); plan "remembered after the retire" (order `ask, retire, read-row, remember, ask`; direct and foreground not remembered; no row remembered). Revision test of the resident module: 2 pass.

## Lines the worker must copy next time

From the **service log** (`$TMPDIR/semio-repo/501/host-<tag>.log`), the lines starting at the last service start:
`grep -a -n "access-proof\|access-denied\|request-failed\|where:\|shutdown" host-<tag>.log | tail -20`
 - `access-proof version=3 …` proves R7c in that service (no such line = old service; then also copy `ls -l` of the log and the tag in the file name).
From the **bootstrap log** of the try: every line containing `graph service`, `started again`, `launchd route was refused`, `denied access`, `could not answer`, `exit code`.
From the **runtime directory** (same folder as the log): `tail -n 6 exit.log` (rows `start` with `via`, rows `access-denied` with `route`, `pid`, `place`) and `ls -la "$HOME/Library/Application Support/semio/access/"*/` with `cat launchd-refused.json` (present after the first denial; its `place` and `at`).
Plus, for the place in `place=`/`entry=`: `ls -leO@ "<root>/.cache/b"`, `mount | grep "/.cache/"`, `hdiutil info | grep -i "image-path\|mount-point"`. If two runners start the same try, say so; both write the same log.

## Not verified

No real worker; the bootstrap client log of try 3 was not available, so whether the direct retry ran and what it printed stays open. The cause of `EPERM` for `.reset.json` in `.cache/b` once the image is mounted (a volume the launcher job may not enter) is still a hypothesis.
