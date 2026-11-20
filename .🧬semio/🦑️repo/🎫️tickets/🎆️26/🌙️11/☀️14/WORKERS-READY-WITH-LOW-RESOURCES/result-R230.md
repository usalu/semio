# R230: wall 91 re-lock (`repo dependencies lock --all`), landed 2026-11-19 01:00:14 to 01:10:04 UTC

## What ran
- User decision (chat, 00:31 UTC): re-lock to fix wall 91; network allowed for this command.
- Base: byte copies of all six `bun.lock` (root, `♻️mit-bestand`, `✏️s`, `🌎️hub`, `🎓️teaching`, `🏢️semio-tech`) in scratch `R230/base/` (+ `base.sha`).
- Rehearsal (no CLI dry run exists): manifest-only scratch copy (13 MB: every `package.json`, bunfig, the nx patch, the scope lock), `bun install --lockfile-only --ignore-scripts --linker=hoisted --backend=clonefile` with a scratch cache. Offline it fails (`failed to resolve`, as R197 predicted: lock-only asks the registry); with network 50 s, semantic change of `✏️s/bun.lock` = exactly the two workspaces gaining `ui-react`, no package added, removed or changed. (The raw rehearsal output differs textually in format only; the repo command writes the committed format.)
- Landing: window 01:00, `landing.lock` taken after 12 s, owner `claude-R230`, base compared with `cmp` under the lock (all six equal), then `nice -n 19 repo dependencies lock --all` (exit 0, about 9 minutes, almost all in `dependencies:lock-ownership` frontier walk of 4.47 million entries under the load of the 6-hour read; the allowed 10 minutes sufficed), lock released by exit trap at 01:10:04. The helper script crashed afterwards in my own verification code (zsh `character not in range` on an emoji pattern) after the command had ended; nothing was left to restore, the exit trap released the lock.

## Exact diff against base
- `♻️mit-bestand`, `✏️s`, `🌎️hub`, `🏢️semio-tech` `bun.lock`: 10 diff lines each = two additions of `"@semio-tech/ui-react": "workspace:*"` (plus the comma on the previous line) in the `dependencies` of the workspaces `🧑‍💻dev/🧩️catalog/📦️packages/🟦️typescript` and `🧑‍💻dev/🏗️fem/📦️packages/🟦️typescript` (in the three other scopes the key is prefixed `../✏️s/`). Semantic comparison (`R230/sem.py`): exactly 2 workspace changes each, only `deps+ ui-react`, no version changed, no package added or removed. `ui-react` count per lock now 39, 38, 39, 39.
- Root `bun.lock` and `🎓️teaching/bun.lock`: byte-equal to base (mtime refreshed only).
- New shasums (12 chars): mit-bestand 2d7beaf6ffc9, s 5a1a07212416, hub cd6946e3df3a, semio-tech 6f57461d878e.

## Proof
`bun install --frozen-lockfile --dry-run --ignore-scripts --linker=hoisted --backend=clonefile` with a refusing registry (127.0.0.1:9) and a scratch HOME/cache, in the four scopes, on the manifest-only scratch copy with the new locks: exit 0 in all four (45 to 129 ms, "done"). The repo CLI has no verify operation (`dependencies sync|lock|ensure` only), so bun was called directly with the recipe's arguments.

## Real machines (read-only)
`/Users/Shared/semio-store/v1/origins/*/ready-timings.json` (`R230/deps.py`): no `dependencies` run after 01:10 UTC yet, so no before/after pair exists. Newest runs: g1-m06 11-18 22:06 to 22:28 `stage-failed` 124 (no progress for 10 min in `dependencies:store-ownership` frontier walk, the same slow walk), then 22:48 to 22:54 `ok` (the stage passed on that machine with the OLD locks, so on g1-m06 the old locks did not fail the stage; the walls-91 reading of R197 applies to a fresh install of the scope locks, not shown by this log). A new attempt after 01:10 UTC is needed to judge.

## NOT verified
- The `dependencies` stage on any real machine after the relock (no attempt yet).
- The dry-run used scratch manifests copied at 00:35 UTC, not the live manifests of 01:10; a manifest edited in between would not be covered. Nothing outside the lock files was examined for changes (the post-run `find` did not execute); the command output `R230/lock-run.txt` shows only progress lines.
- The frozen install itself (not `--dry-run`) with the cache of a real machine.
- Side note for the coordinator: `repo dependencies lock --all` costs about 9 minutes of an ownership walk on this loaded Mac, more than the 3-minute landing hold.

Files: `scratchpad/R230/{land.sh,sem.py,deps.py,base/,rehearse/,lock-run.txt,land-out.txt}`; landing entry in `T/landings.md` ("R230 landing").
