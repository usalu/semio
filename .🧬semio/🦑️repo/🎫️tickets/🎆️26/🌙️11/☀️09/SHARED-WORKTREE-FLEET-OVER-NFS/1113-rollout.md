# Rollout 2026-11-13

Contract: `1113-git-state-contract.md`. Executor reports: `1113-result-git-state.md`, `1113-result-attestation.md`. OS audit and worker prompts: `1113-audit-os-readiness.md`, `1113-worker-prompts.md`.

## g1-server (this Mac) — done

- The old replica (pid 7176) was stopped once at 00:45; launchd started the new code (pid 27071).
- The first scan dropped the tracked `.git/index`, reflog, config and IDE entries without publishing a delete, and published exactly one base: `.cache/f/index-base` → `tree 19442f42b75748f59c5283b757f72158561faf7e`.
- `status`: `Git state: shared (stage and commits replicate)`. `attest.json` is written every few seconds (platform darwin, HEAD `5ececaa78b9265fd72b4f7e8f411afac3c9e1df1`, manifest digest, stage tree).
- `.git/config` was normalised locally: `autocrlf=false`, `symlinks=true`, `fsmonitor=false`, `gc.auto=0`, `maintenance.auto=false`; `commit.template` (a path in another checkout) was removed.
- Live check: `git add` of one ticket file produced one `link` entry in the journal within seconds; `git reset` of it produced one `delete`; the stage tree returned to the base. Raw `.git/index` and reflog puts from g1-m01 and g1-m02 are now refused as ignored.

## Suites after both executors (re-run by the coordinator)

journal 127, boundary 39, git 9, attest 35, replica 61, race 14, replica git (three replicas, real git) 6, bootstrap 12, topology 56 — all pass. The journal streaming test needs `--timeout 30000` under a load average of 25. Two topology tests depended on this host (`/var/log` files already present, `SEMIO_*` in the shell); they now use a temporary directory and a scrubbed environment. The PowerShell suite (19 tests) did not run: no `pwsh` on this Mac.

## Not done

- g1-m01, g1-m02, g1-m03 still run the old daemon. Nobody can reach them from here (SSH refused or timed out). Each needs the "Upgrade an already joined worker" prompt in `1113-worker-prompts.md`: stop the replica once as the worker account, then `status` and `fleet --wait 120`. Until then `fleet` exits 1 with those origins `missing (no attestation)`, and they keep publishing raw `.git/index` (69 MB per change) into the store.
- An unpublished local stage on a worker is replaced by the server's base when the worker first starts the new code.
- g1-m03 has published nothing since 2026-11-12 16:40 local time.
- No Linux or Windows machine has joined. The join prompts are in `1113-worker-prompts.md`. The Windows provision script edits (root `.git` accepted, one-minute repeating trigger) have never been executed.
- Concurrent commits are linearised by the losing machine once it runs the new code (`1113-result-concurrent-commits.md`); old daemons still drop the loser.
- `AGENTS.md` still forbids mutating git commands and names an archivist. It was not edited (rule). Stage and commit now replicate from any worker.
- Executor A created `ajv` symlinks in `/Users/Shared/node_modules` (outside the worktree) to run tests.

## Acceptance command

On any machine, from the replica directory: `repo ./📜️script.ts fleet --wait 120 --lang en`. Exit 0 means every origin with sync on is fresh, settled, and agrees on the file manifest, HEAD commit and stage tree.

## 01:25 — workers upgraded, base republished

- g1-m01 and g1-m02 (both macOS) run the new code (`g1-m01-upgrade.md`, `g1-m02-upgrade.md`). The upgrade prompt's stop on `verify` was wrong (the old `verify` counts private `.git` paths) and is removed; see `1113-coordinator-answer-upgrade.md`.
- Fault found live: the workers' old daemons had consumed and dropped the base register before the restart, so the new code on them had no base and queued every stage entry (`waiting for the base stage`, GitKraken on g1-m02 showed a different stage). Remedy and new operation: `repo ./📜️script.ts rebase` writes `.cache/f/index-rebase`; the running replica publishes its current stage as a new base on its next publish step and removes the request. Run on g1-server at 01:22; both workers drained their queue.
- Live proof after that: 37 module files staged on g1-server at 01:22:36 and one ticket file staged and unstaged by the coordinator; g1-server, g1-m01 and g1-m02 attested the same stage tree after each step (`5eb68a6e…`, then `6bc6e1d0…`), same manifest digest, same HEAD, same code digest.
- Linux: every suite passes in an arm64 Linux container, the PowerShell suite passes on Linux pwsh (19), and a macOS + Linux two-replica run converged on edit, stage and commit with `fleet` exit 0 (`1113-result-linux.md`). Two defects fixed there: bash ≥ 5.2 `patsub_replacement` in the provision script, and read-only git objects failing to stage on a bind mount.
- Open: g1-m03 silent since 11-12 16:40 (keeps `fleet` at exit 1); no physical Linux or Windows worker; no live commit made on the shared branch; real Windows APIs unverified.

## 02:30 — Linux worker, untracked junk, repair

- g1-m04 (Linux) joined and attests the same manifest, HEAD and stage tree as g1-server, g1-m01, g1-m02.
- 246 paths were tracked in git but can never be shared (159 match the root `.gitignore`: pid, lock, sqlite, nx workspace data, 32 generated `interfaces/*.ts`; 89 symlinks point outside the worktree). Workers showed them as unstaged deletions. On the owner's decision they were removed from the index on g1-server (`git rm --cached`, files kept); the stage tree `646bab0a…` reached g1-m01, g1-m02 and g1-m04 at once. Not committed.
- g1-m03 runs the new code but has no root `.git`: its old daemon read past the shared `.git` events without folding them. New operation `repair` (`.cache/f/repair` request, consumed by the running replica): folds the store, takes over missing or older registers, re-queues shared files missing on disk. See `1113-coordinator-answer-g1-m03.md`.
- New operation `retire <origin>` (`1113-result-retire.md`); nothing retired.

## Fleet agreement reached

`fleet --wait 180` on g1-server exited 0: g1-server, g1-m01, g1-m02, g1-m03 (macOS) and g1-m04 (Linux) are fresh, settled and identical — manifest digest `c4891684da0b…`, HEAD `5ececaa78b…`, stage tree `c9569e4f91d0…`, code `04c8c5e990bb`. g1-m03 recovered its root `.git` through the store-folding `repair`. g1-m01 published 18 stage entries of its own, applied everywhere.

Open: no commit made on the live branch (301 paths staged); no Windows worker; `AGENTS.md` still names an archivist and forbids mutating git commands.

## 10:00 — state after the morning incidents

- Landed and re-run by the coordinator on g1-server (29 test files, all pass): chaos fixes (`1113-result-chaos-fixes.md`), settle-scan cost fix (`1113-landings.md`, 08:40), compaction capability, automatic operation off (`1113-result-compaction.md`), boot stage with snapshot fallback, admission rule and blocked-loop watchdog (`1113-result-boot.md`).
- g1-server: synchronisation on since 08:25, daemon supervised, code `37dde6043d2e`, attesting.
- g1-m01, g1-m02, g1-m03, g1-m04, g1-m06: no attestation since 08:41 (restart onto a mixed module, crash loop). Each needs the prompt in `1113-worker-recovery-0841.md`. g1-m05: silent since 04:44.
- Reverted files restored from blobs and published (2 module files, 11 others); one file left untouched because the disk held a third version (`📚️library/🏗️build/🏡️resident/🧪️tests/🛑️cancel/🟦️.ts`).
- Repo comparison for the relocation: `1113-audit-existing-repo-vs-fleet.md` (two runs disagree on several counts; the adopt dry run decides).
- Not done: fleet agreement after recovery, compaction `--apply`, relocation into the existing repos, Windows worker, commit on the live branch.

## 2026-11-14 09:05 — live folder deleted by an agent's scratch mirror, restored

- 09:05:35 `g1-server` journaled 50 deletes (`1794643535803-000000…000049-g1-server`): the whole `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity`. Cause: an agent's scratch script (`mirror.sh`: symlink farm into the live tree, then `rm -rf "$DST/$p"; cp -R`) deleted through a symlinked parent. Every `repo` command failed (`Cannot find module '../🌡️host-capacity/🎛️control/🟦️.ts'`) on all machines for about four minutes.
- 09:09 restored all 50 files from `<store>/v1/blobs` by the `base` hash of each delete event (sha256 verified, 0 missing). Agent stopped, its 26 GB copy removed, rule added to the agent preamble (no symlinks into the live tree, `realpath` check before recursive deletes).
- Same hour: the relay's hourly bootstrap archive (33.9 GB, written as a second copy in `<store>/v1/tmp`) took free space to 1 GiB; freed 29 GB by removing the idle Rust cache image. Fix in progress (package W, `1114-result-git-bloat.md`).
