# Result R32: `Relative filesystem fstatat failed (EPERM)` for `.cache/b` `.reset.json` is not a TCC refusal

PREPARED ONLY, not landed. Package: `T/🧰️scripts/R32/` (T = `FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION`): `build_payload.py`, `payload/{base,new,files.txt}`, `land.py`, `overlay.py`, probes `probe-sandbox.py`, `probe-mount.sh`/`.ts`, `repro-errno.ts`, `repro-errno-fixed.ts`. No graph or `repo` command was run. Scratch copies and the scratch disk image (detached, `mount` shows none) are deleted.

## Landing

```
cd "/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/R32" && python3 land.py --land        # inside the coordinator window: add --lock-held
```
`--dry-run` is clean (4 edits, 2 new files, all `edit`/`new`, parse proof of every file), `--rehearse` landed, was idempotent and rolled back byte-identical. `land.py` takes `T/landing.lock`, runs 14 test files before and after (prints `NEW RED` and the rollback command), appends to `T/landings.md`; rollback: `python3 land.py --rollback T/🗑️generated/R32-landing-<HHMMSS>`. Touches 6 files, none of the R26/R28/R29/R30 files; no shared-worktree file, so no replica lock.

## Cause (what is proven, with file:line as of today, `D` = `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts`)

The failing call is `readEntry` of the relative directory helper (`D:153`, darwin branch: `fstatat(dirfd, ".reset.json", AT_SYMLINK_NOFOLLOW)`, the stack columns `148:321` and `153:768` of every log). It is made by `readInactivePublicationDataV1` (`⚡️caching/…/📥️opening/♻️reset/🟦️.ts:123`: `current.entry(protocol.intent)` in the capsule `.cache/b/<p>/`, anchor `.cache/b`); the marker is normally absent, so this is the one `fstatat` of the normal path that is **expected to fail**, and so the only one whose errno is ever read.

1. **The errno is read by a second native call after the failed one** (`D:148` `settle` calls `errno()` = a second FFI call `__error()` plus `toArrayBuffer`, after `Number(run())`). Thread-local errno is not protected against anything that runs on the JS thread between the two calls. **Reproduced here** with the pinned bun 1.4.2 (`repro-errno.ts`, scratch capsule, 150 fresh processes read ENOENT correctly, long runs do not): per 300 000 to 1 500 000 failed `fstatat` calls of an absent name 1 to 48 read a wrong number, EPERM(1) among them (runs: 4 EPERM + 19 x 316 of 1.5 M; 3 EPERM of 300 k with `BUN_JSC_forceRAMSize=128 MB`, 2 of 1 M, 3 of 1 M; the services run with `BUN_JSC_forceRAMSize`). The old code takes the first reading as the truth: EPERM -> `failure()` (`D:148`) -> `accessDeniedOfV1` (`🏛️service/🧯️failure/🟨️.mjs:12`) names every EPERM below Documents "macOS consent" -> `failureRefusalV1` returns `access-denied` (`:97`) -> the host drains and exits 4 (`🌐️host/🟨️.mjs:177`) -> the client retires and retries direct. A direct service then meets the same read pattern again; that is the "direct is refused too" of g1-m03 try 11 (07:49:33Z launchd pid 56873, 07:50:46Z direct pid 60693).
2. **The `where:` place was wrong.** `root=` is the root of the helper (`.cache/b`), the entry was looked up in the child directory (`.cache/b/<p>`), so the line named `.cache/b/.reset.json`, a path nobody asked for. Every R7/R7b/R7c probe checked that wrong or at best the right path by name from the service process, and passed (`access-proof … sample=ok`); this is why R7c saw "proof ok, request refused".

## Ruled out, with evidence (this Mac, macOS 27.2.1, arm64)

- **(a) sandboxed child**: the only Seatbelt profiles are `📥️inputs/🔭️read-oracle/🍎️seatbelt/🟦️.ts:16-23` (a CLI that checks declared inputs) and `🌐️remote-execution/🔒️sandbox/🟦️.ts:37-39` (remote actions); neither wraps the graph service or the slice worker, and both deny only `file-read-data`. `probe-sandbox.py` under that profile: `fstatat` of an absent name = **ENOENT(2)**, `fstatat` of the existing file = ok, only `open` for data = EPERM(1). A sandbox cannot turn an absent name into EPERM.
- **(b) marker owner/mode/flags/ACL/symlink**: `fstatat` needs search permission on the directory only (EACCES, not EPERM, without it: existing test `🧭️denied-entry`); the entry is absent, so no flag of the file exists. Not applicable.
- **(c) stat during attach/detach**, **(d) stale dirfd / device confinement**: `probe-mount.sh` (10 MB APFS image attached at `cap/g/m`, the opener's exact access `anchor → withDirectory("g") → entry(".reset.json")`, three times, also with the bun from the mounted tools image and against the live `.cache/b` with eight attached images): **null (ENOENT) every time**. The helper does enforce "same device as root" (`D:152` `Original physical relative directory volume required`, `D:153` `Original relative child volume required`: reproduced with `entry("m")`), but as a plain `Error` without code and never as EPERM; `.reset.json` lies on the host volume, not on the mount.
- TCC for the path itself: the worker logs show `access-proof … sample=ok` (path-based, service and forked child) in the same service that then fails; m02 round 24 ran with the same proof and no EPERM.

**Not proven**: what overwrites errno with 1 on the workers so that it happens at the very first plan request of nearly every service (here 1e-6 to 3e-5 per failed call, not 1.0). The fix does not depend on it.

## Fix (at the cause)

- `D` `settleNativeCallV1` (exported, new): a failed read-only call counts a number only when **two failed calls in a row report it** (`reads: 2`); a success on a repeat is the answer; EINTR and nameless numbers are retried without counting; bound `maximumErrnoAttempts` 4. `settle` uses it (stat, open, readlink of existing entries; no write path). The error now carries `directory` (the real path of the frame, tracked per frame) and `reads`.
- `🏛️service/🧯️failure/🟨️.mjs`: `relativeCallDivergenceV1`: an error that carries `reads` and whose place (`directory/entry`) the service **itself reads by path (node `lstat`) without EPERM** is **not** the consent: new typed refusal `unserved / host-relative-call-refused` (EN+DE in the resident schema, says "service itself reads the same place by path … ({found}) … not the proven missing macOS file access consent … nothing was changed"), the service keeps running (no `access-denied` exit, no retire-and-retry detour). A path-confirmed EPERM stays `consent / access-denied` and now names the real place. Errors without `reads` (node fs, reliable errno) are judged as before. The `where:` line gets `directory=`, `reads=` and `pathcheck=`.

## Proof (bun 1.4.2, `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`, overlay under scratchpad, red before / green after)

| Law | Test | Before | After |
|---|---|---|---|
| two agreeing readings decide, one odd reading never, success on repeat, EINTR/nameless retried, attempts bound | `observation/🧪️tests/🎯️errno-consensus` (new, 4 tests) | file does not load (export missing) | 4 pass |
| the loop equals an independent Python rule on 400 seeded random sequences | same, `🐍️oracle.py` | red | pass |
| real helper: 20 000 absent entries -> null; refused entry in a child frame carries `directory` and `reads: 2` | same | red | pass |
| unconfirmed EPERM = `host-relative-call-refused` with place/reads/found/reference, log line `where: … reads=2 … pathcheck=ENOENT`; present/EACCES answers; path-confirmed EPERM stays consent with the real place; errors without `reads` unchanged; default check on a real scratch tree; EN/DE placeholders equal and wording | `resident/🧪️tests/🧯️failure` (+4 tests) | 9 pass 4 fail | 13 pass |
| real loop, old single reading vs new, 3 x 1.5 M failed `fstatat` with `BUN_JSC_forceRAMSize=128 MB` | `repro-errno-fixed.ts` | wrong readings 23, 46, 52 (EPERM 4, 0, 4) | **0 wrong in 4.5 M** (all `2/reads2`) |

After: observation `errno-consensus` 4, `denied-entry` 1, `📁️descriptor` 7 (before: 6 pass 1 fail, a flaky concurrency-sensitive test), resident `failure` 13, `consent-service` 31, `worker-error` 2, `consent` 16, all other resident suites 0 red live. In the overlay `host`, `service`, `watch`, `provision` are red for overlay reasons (source-revision closure of a partial tree) and pass live (36, 37, 14, 16); `land.py` therefore runs them live before and after.

## Lines a worker must show after landing

Healthy: no `request-failed da146983 … fstatat failed (EPERM)`, `native_rc` not 1 or 4 for this reason. If it still happens, the service log now has, instead of `shutdown access-denied`:
`where: root=<…/.cache/b> directory=<…/.cache/b/<p>> entry=.reset.json code=EPERM reads=2 route=<launchd|direct> pid=<n> ppid=<n> pathcheck=<ENOENT|present|EACCES>`
and the client prints "A filesystem call of the resident service was refused with EPERM for <place> (2 readings agreed), but the service itself reads the same place by path without a refusal (<found>) …". That combination means a **real, repeatable** refusal of the `fstatat` that the service's path read does not share: copy that `where:` line, `ls -leO@ "<directory>" "<directory>/.."`, `mount | grep "/.cache/b"`, `sw_vers`, `uname -m`. If `pathcheck` is absent and the text is the consent text with `place=<…/<p>/.reset.json>`, the refusal is confirmed by path too and the consent dialog/Full Disk Access is the cause after all. A request that previously failed on a single bad reading now simply continues (no log line).

## Not verified / others must know

- No worker run; the deterministic occurrence on the workers is not reproduced here (see above). The classification and the consensus are proven on the injected sequences, on 4.5 M real calls and with a scratch tree; the real service end to end is not run.
- Same single-read pattern also exists in `🏃️process/🔒️leases/📁️physical/🟦️.ts:32-35` (`flock` contention check, 3 attempts already retry), not touched.
- The schema catalog hash of the resident schema stays stale until `schema generate` runs (as for R7). `reads` is an in-process error field, not a wire message.
- R7's start-time proof (`🔐️consent/🟨️.mjs`) is unchanged; its `RESET_MARKER_V1` probe path is correct by name, only the error side was wrong.
