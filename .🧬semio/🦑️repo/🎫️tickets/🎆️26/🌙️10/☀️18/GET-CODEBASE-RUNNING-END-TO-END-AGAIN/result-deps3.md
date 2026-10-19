# result-deps3 (CHECKPOINT 14:23, lock attempt pid 29799 still running)

## Root cause 1 (FIXED): "Queued cache maintenance must begin outside its reader scope"
`runOwnedBuildInvocationV1` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts`, bd9e4e02bd0) wrapped EVERY repo command in `withRepositoryCacheStoreV1(root,"dependencies",...)` (reader). `repo dependencies lock|sync` then enters `withJavascriptDependencyStorageV1` -> `withRepositoryCacheStoreQueuedMaintenanceV1` (queued EX, since 8f91299152c) while that reader was live -> guard refuses.
Fix: for `selected.args[0]==="dependencies"` only the diagnostics reader is opened; the dependencies reader is skipped (queued EX opens after no reader of that partition exists). Guard and nonwaiting/queued design untouched.

## Root cause 2 (FIXED): "Exact displaced Source lock retained"
After publishing bun.lock the displaced old lock (`.xxxxxxx` stage file in repo root) is re-checked by full stamp incl. ctime. macOS adds `com.apple.macl` ~170 ms after the rename, bumping ctime (confirmed by [DEBUG] log: only ctimeNs differed). Fix in `.../📦️dependencies/💾️storage/🟦️.ts`: new exported `sameDisplacedJavascriptDependencyFileIdentityV1` (dev, ino, mode, nlink, size, mtime; no ctime) plus a chunked sha256 content check against the digest the publish returned. Test appended to `.../💾️storage/🧪️tests/🟦️.ts` (passes: `bun test ./<path> -t "displaced lock"`, 1 pass). [DEBUG] log removed.

## Lock attempts (cwd repo root)
1. 12:38 coordinator chain: Queued ... reader scope (rc 1, root cause 1)
2. 12:48 after fix 1: `Exact displaced Source lock retained` (rc 1; root cause 2; bun.lock got republished)
3. 12:52 same refusal; 4. ~12:55 with [DEBUG]: ctime-only difference
5. after fix 2a (single read of 660 KB): `Bounded relative file read required` (read > 65536 per call); fixed with chunked read
6. `Relative filesystem openat failed ENFILE` (system file table spike from peers; transient)
7. 590 s cap hit twice (rc 142, machine load avg 60-70, lock was progressing, not refused); then background run with 2400 s cap
8. 13:27-14:01 `Bun current workspace membership changed during verification` (rules-fix deleted the trinity jack lsp rust shim package while locking). Manifest batch applied after it.
9. 14:02 run killed by me at 14:12 to apply coordinator correction (drop config/schema/mutations key); 10. 14:12 run pid 29799 running, no refusal so far.
Refusals are not caused by concurrent `bun repo cargo`: ~12 `bun repo cargo ... metadata` processes ran during attempt 9, no admission refusal; they only add load. Concurrent manifest/workspace edits DO invalidate an in-flight lock (attempt 8).

## Manifest batch applied (coordinator request)
- `🧰️framework/📦️packages/🟦️typescript/package.json`: removed 6 stale `./repo/modules/library/*` exports. Added key `./config/schema/mutations` was NOT kept (coordinator correction).
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/package.json`: removed `./builder/python/inventory`.
- `✏️s`, `🌎️hub`, `🏢️semio-tech`, `♻️mit-bestand` `package.json`: removed workspace entry `.../🔌️jack/🧠️lsp/📦️packages/🦀️rust` (deleted shim). Leftover dir there holds only build-output `pkg/` and a `node_modules` symlink.

## FINAL 14:48
Attempt 10 (pid 29799, started 14:12, no refusals): `[DATA] Store dependency lock complete completedLocks=6 stagedRoots=6`, no `error:` line, process exited, bun.lock current. Treated as exit 0 (run via nohup, exit code not captured by shell; absence of any error line and the complete DATA line). Manifest state at that run: 6 framework stale exports removed, ui-styling inventory export removed, config/schema/mutations NOT added; the four workspace lists were later changed by a peer to `.../rust/pkg` (coordinator handles as follow-up).
Files changed by me: build `📜️script.ts` (dependencies skips dependencies reader), dependencies storage `🟦️.ts` (+ helper, chunked sha256 check), storage `🧪️tests/🟦️.ts` (+1 test), 6 package.json (see above). [DEBUG] logs removed.
