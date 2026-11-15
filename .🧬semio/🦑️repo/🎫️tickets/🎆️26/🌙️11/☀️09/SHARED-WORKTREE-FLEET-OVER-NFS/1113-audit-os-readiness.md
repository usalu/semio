# 🔍️ OS Readiness Audit for the Shared Worktree Fleet (2026-11-13)

Read-only audit. Nothing was edited, started, stopped or published. Paths are relative to the module `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/` (short: `MOD/`). Ticket paths are relative to this ticket folder (short: `TICKET/`). Times: the store is read at 2026-11-13 00:18 CET (23:18 UTC).

## 1. Origins in the store

Store: `/Users/Shared/semio-store/v1/origins`. Each row is read from `<origin>/head.json` and its mtime. The HLC prefix is milliseconds since the Unix epoch, decoded with `date -u -r`.

| Origin | head.json HLC (ms prefix) | Decoded (UTC) | head.json mtime (CET) | Platform stated by a file | Verdict |
| --- | --- | --- | --- | --- | --- |
| `g1-server` | `1794525504445-000000-g1-server` | 2026-11-12 23:18:24 | Nov 13 00:18:24 | macOS: this Mac, topology `g1.server.platform` is `macos` (`MOD/🗺️topology/🔣️.json`, group g1) | Publishing, active |
| `g1-m01` | `1794525498112-000000-g1-m01` | 2026-11-12 23:18:18 | Nov 13 00:18:18 | macOS: `TICKET/g1-m01-join.md:3` ("This Mac ... Mac mini"), `g1-m01-join.md:27` (`/Volumes/semio-store`) | Publishing, active |
| `g1-m02` | `1794525526640-000000-g1-m02` | 2026-11-12 23:18:46 | Nov 13 00:18:46 | not stated in any file | Publishing, active |
| `g1-m03` | `1794498032940-000000-g1-m03` | 2026-11-12 15:40:32 | Nov 12 16:40:32 | not stated in any file | Looks stopped (see below) |

Facts behind the `g1-m03` verdict:

- Its `head.json` has not moved since 2026-11-12 15:40:32 UTC. The other three origins moved between 23:18:18 and 23:18:46 UTC, so the lag is 7 h 38 min.
- Its only journal segment is `g1-m03/segments/000000000000.jsonl` (582 bytes, 2 lines: a `link` of `CLAUDE.md` at HLC `1794492222893`, and one `put` at `1794498032940`). Its `home.json` is dated Nov 12 15:03 CET.
- The cursors in this machine's `.cache/f/cursor.json` (`g1-m03` at segment 0, byte 582) agree that g1-m03 has published nothing since. This Mac cannot see g1-m03's daemon, so "stopped" is an inference from the store, not a process check.
- No `attest.json` exists in any origin directory (`find /Users/Shared/semio-store/v1/origins -name attest.json` is empty). The attestation design in `TICKET/1113-git-state-contract.md:49-59` is not in the store yet.

Other store facts:

- `g1-m02` and `g1-m03` are in `/Users/Shared/semio-store/v1/origins`. No file states their platform. `grep` for `windows|linux|darwin|C:\\|/srv/semio|/Users/` in their segments returns nothing.
- `g1-server` local identity: `/Users/Shared/semio/.cache/f/machine.json` = `{"originId":"g1-server","groupId":"g1","storeRoot":"/Users/Shared/semio-store"}`.
- Replica of this Mac: `.cache/f/daemon.lock` = `{"pid":7176,"startedMs":1794523467675,"originId":"g1-server"}`, process 7176 is alive (`ps -o pid,user,lstart,command -p 7176`, started Nov 12 23:44:27, parent 1, user `ueli`).
- Origin ids are never reused (`TICKET/fleet-setup.md:13`). The next unused id after the existing `g1-server`, `g1-m01`, `g1-m02`, `g1-m03` is therefore not `g1-m02` or `g1-m03`.

## 2. Service install, account, mount, git, symlinks and exec bit per OS

### macOS

- Replica service: LaunchDaemon `/Library/LaunchDaemons/tech.semio.fleet.replica.plist` (label `MOD/🗺️topology/🟦️.ts:36-37`). Built by `daemonOf` (`MOD/🗺️topology/🟦️.ts:221-229`) with `KEEP_ALIVE` (`MOD/🗺️topology/🟦️.ts:231`: `RunAtLoad`, `KeepAlive` true, `ThrottleInterval` 2) and used at `MOD/🗺️topology/🟦️.ts:262`.
- Restart after exit 0: yes. `KeepAlive` true restarts the job after any exit.
- Account: `UserName` is the placeholder `__WORKER_USER__` (`MOD/🗺️topology/🟦️.ts:48` and `:224`), replaced with the worker account. The daemon runs as the worker account, not root.
- PATH: `EnvironmentVariables` `PATH=/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin` and `HOME` (`MOD/🗺️topology/🟦️.ts:220`). Homebrew's `/opt/homebrew/bin` is not on it.
- git on PATH: not guaranteed. `/usr/bin/git` exists only with the Xcode Command Line Tools. The provisioner reports the missing tools with `xcode-select --install` (`MOD/🗺️topology/🥾️provision/🐚️.sh:253-257, 263-265`), but only inside the native bootstrap, which runs after join (`🐚️.sh:319-334`, last step `620`). Nothing checks git before the replica starts.
- Store mount: NFS. A LaunchDaemon `tech.semio.fleet.store-mount` (`MOD/🗺️topology/🟦️.ts:34-35, 193-195`) runs `mount` every 60 s (`StartInterval` 60, `RunAtLoad`) and calls `mount_nfs -o <options>` (`🐚️.sh:501-508`). The archive mount uses `mount_nfs -o ro,vers=3,nolocks` (`🐚️.sh:373-374`) when the server is not Linux.
- Symlinks: replicated as `link` events and not in the bootstrap tar (`MOD/🔁️replica/🥾️bootstrap/🟦️.ts:117`: "links stay to `join`"). Created by `join` on macOS.
- Executable bit: honoured. `MOD/README.md:205` ("on macOS and Linux a file whose executable bit differs from the store counts as different"). Bootstrap and settle check it (`MOD/🔁️replica/🥾️bootstrap/🟦️.ts:171, 213`, `if(!windows)`).

### Linux

- Replica service: systemd template unit `/etc/systemd/system/semio-fleet-replica@.service` (`MOD/🗺️topology/🟦️.ts:40`), instance `semio-fleet-replica@<account>` (`MOD/🗺️topology/🥾️provision/🐚️.sh:563-567`, `systemctl enable "$INSTANCE"`), started at `🐚️.sh:611-613` (`systemctl enable --now`, or `try-restart` for changed units at `:612`).
- Restart after exit 0: yes. `Restart=always` and `RestartSec=2` (`MOD/🗺️topology/🟦️.ts:252`).
- Account: `User=%i`, so the instance name is the worker account (`MOD/🗺️topology/🟦️.ts:251`).
- PATH: the unit has no `Environment=` line, so systemd's default PATH applies. `ExecStart` uses the absolute `REPO=/usr/local/bin/repo` (`MOD/🗺️topology/🟦️.ts:23`), which is a symlink to the per-account launcher (`🐚️.sh:417`).
- git on PATH: not guaranteed. The provisioner names `git` in the build tools list (`🐚️.sh:253-257`, `:263`, `:269`), which the native bootstrap checks after join. The replica does not check for git.
- Store mount: NFS. The persistent mount is an `/etc/fstab` line generated by `mount-plan` (`🐚️.sh:455-484`). A Linux worker of a macOS group server mounts NFS v3 with `nolock` (`🐚️.sh:372`; `TICKET/provisioning-review.md:23-25`). The archive mount uses the same options (`🐚️.sh:371-372`).
- Symlinks: replicated as `link`; on Linux created by `join`. The bootstrap tar does not carry them (`MOD/🔁️replica/🥾️bootstrap/🟦️.ts:117`).
- Executable bit: honoured (`MOD/README.md:205`; `MOD/🔁️replica/🥾️bootstrap/🟦️.ts:171, 213`).

### Windows

- Replica service: scheduled task `semio-fleet-replica` (`MOD/🗺️topology/🟦️.ts:43`). Trigger: `LogonTrigger` (`MOD/🗺️topology/🟦️.ts:242`). Principal: `<LogonType>InteractiveToken</LogonType>` (`MOD/🗺️topology/🟦️.ts:237`, `taskOf` at `:232-241`). The task runs only while the worker account is logged on. A headless or rebooted worker does not sync until someone logs on.
- Restart after exit 0: not guaranteed. The task uses `<RestartOnFailure>` with `PT1M` and 255 retries (`MOD/🗺️topology/🟦️.ts:243`, used at `:264`). Task Scheduler restart applies to failed runs, and the contract needs a restart after exit 0 (`TICKET/1113-git-state-contract.md:61-63`). Nothing in `🔵️.ps1` adds an exit-0 policy.
- Account: the `-User` argument, default `$env:USERDOMAIN\$env:USERNAME` (`MOD/🗺️topology/🥾️provision/🔵️.ps1:29`); the script refuses any other account (`🔵️.ps1:47`).
- Command and PATH: `Command` is `repo` with a relative name (`MOD/🗺️topology/🟦️.ts:264`). It resolves only through the user PATH. `🔵️.ps1:232-240` adds the launcher directory to the user PATH, and the task is not started until logon. Git is installed only by the native bootstrap through winget (`🔵️.ps1:361` names Git as one of the winget packages). The replica does not check for git.
- Store mount: Client for NFS (`mount.exe`, drive `S:\`) or SMB (`net use`). Mount tasks run on logon (`MOD/🗺️topology/🟦️.ts:205, 211`, `/SC ONLOGON`). `🔵️.ps1:167-179` mounts with `Test-Path`.
- Symlinks: need Developer Mode or the privilege. Without it, the entry is skipped with `symlink-unsupported`, and nothing is published for it (`MOD/README.md:436-437`).
- Executable bit: not honoured on Windows (`MOD/🔁️replica/🥾️bootstrap/🟦️.ts:171`, `windows||`; `:213`, `if(!windows)`).
- Long paths: `LongPathsEnabled` is set and read back (`🔵️.ps1:147-152`); the worktree root is at most 15 characters (`🔵️.ps1:62`).

## 3. Places that still say `.git` is not shared, must not exist, or refer to an archivist

These statements contradict the contract (`TICKET/1113-git-state-contract.md:3, 14-24, 36-43`), which makes the root `.git` shared through an allow-list.

Module code and docs:

- `MOD/README.md:187`: "Git for Windows is not involved (`.git/` is not replicated)".
- `MOD/README.md:334`: "Copy a complete checkout (`.git/` not needed)". Manual step 3 of the g1-server setup.
- `MOD/README.md:387-390`: "7. **Archivist (optional).** ... git and a `.git/` of its own, which is not replicated. It commits its replica on a timer and pushes `main`. The skills `commit`, `micro-commit` and `merging` ... Keep it in one group and never run git mutations anywhere else."
- `MOD/README.md:585`: "Git mutations on a worker. `.git/` is not shared, so a worker's git state is meaningless to the fleet."
- `MOD/🚧️boundary/🟦️.ts:173-174`: the root-`.git` deny-list `GIT_PRIVATE_FILES` and `GIT_PRIVATE_DIRECTORIES`. Per the contract, everything under `.git` except the allow-list (`HEAD`, `packed-refs`, `shallow`, `refs/**`, `objects/**` minus temp and alternates) must be private. The current list does not include `index`, `config`, `logs`, `hooks`, `info` (except `info/exclude`), `description` or `cursor`. `gitPrivateV1` is at `:177-185`. `sharedV1` treats the root `.git` as shared unless `gitPrivateV1` matches (`:188-190`).

Ticket docs:

- `TICKET/fleet-setup.md:78`: "The archivist is one machine with its own `.git`, which is not replicated. Do not create `.git` in a fleet worktree." This contradicts the contract and the live `/Users/Shared/semio/.git`.
- `TICKET/mac-worker-prompt.md:33`: "Do not use git commands that change the shared worktree." This has no stage/commit exception (the contract allows staging and commits on request).
- `TICKET/owner-file-changes.md:23, 55-60, 236, 258-263, 279`: "Only the archivist machine touches git". This is the old model.
- `TICKET/git-state.md` is superseded by the contract (`TICKET/1113-git-state-contract.md:3`), but its content still describes the file-level sharing.

Code outside the module that is still wrong: none found in the three provisioning scripts. They contain no git-specific handling at all (grep for `core.`, `autocrlf`, `gitconfig`, `.git/config` is empty).

Missing contract parts (grep results): no `gitStateV1`, `write-tree`, `update-index`, or `🌿` anywhere in `MOD/`; no `attest` anywhere in `MOD/`; no `restarting` or `daemon.lock` self-restart in `MOD/🔁️replica/🟦️.ts`; no `fleet` operation in the replica command list (`MOD/🔁️replica/⌨️command/🟦️.ts:18`, which lists `seed|join|run|status|snapshot|relay|compact|verify|bootstrap-archive|bootstrap-settle|sync`). The `fleet --wait` command in the contract and in the prompts does not exist yet.

## 4. Effects of a root `.git` on the provisioning scripts

- `MOD/🗺️topology/🥾️provision/🐚️.sh:358-361`: `if [ -f "$WORKTREE/$NATIVE_BOOTSTRAP" ]` (line 358) skips the archive; otherwise `elif [ -n "$(ls -A "$WORKTREE")" ]` refuses. A root `.git` makes the worktree non-empty, so a worktree with `.git` but without the native checkout file is refused ("neither empty nor a checkout"). The message at `:361` does not mention `.git`.
- `MOD/🗺️topology/🥾️provision/🔵️.ps1:194-199`: the same check with `Get-ChildItem -LiteralPath $Worktree -Force` and `Count -gt 0`. Same effect.
- `🐚️.sh:383-384` and `🔵️.ps1` tar-entry count: `find "$WORKTREE" -type f | wc -l` must equal `EXPECTED_FILES` from the manifest. The manifest counts every `put` entry, including the `.git` entries. The count stays consistent when `.git` is in the archive, and it counts `.git` files after extraction. No break, but the count will move if the allow-list changes the manifest.
- `🐚️.sh:388-405` (`release_foreign_namespace`, called at line 405) handles only `node_modules`. Nothing normalises `.git/config`, which holds per-OS keys. `TICKET/1113-git-state-contract.md:45-47` requires `core.filemode`, `core.symlinks`, `core.ignorecase`, `core.precomposeunicode` and removal of an external `commit.template`. No such code exists.
- After extraction, `bootstrap-settle` (`MOD/🔁️replica/🥾️bootstrap/🟦️.ts:190-222`) moves every shared file that differs from the store to `.cache/f/lost/bootstrap-<ms>/`, then copies the store version back. Because `.git/index`, `.git/config` and `.git/logs/**` are currently shared, a worker receives the g1-server index and config and would get `lost/` copies of its own (see section 5).
- `join` refuses a non-empty worktree whose shared files do not match (`MOD/🔁️replica/🟦️.ts:1408-1409`; text at `MOD/🔁️replica/📖️texts/🔣️.json:78`). With `.git` shared, a worktree that already has a `.git` from another source takes this refusal.
- The contract's "a worktree without a root `.git` skips the realm" (`TICKET/1113-git-state-contract.md:43`) is not implemented. Nothing in the replica reads git state.

## 5. What the bootstrap archive contains

Command: `bootstrap-archive` (`MOD/🔁️replica/⌨️command/🟦️.ts:32`), handled by `runBootstrapCommandV1` (`MOD/🔁️replica/🥾️bootstrap/🟦️.ts:233-244`). It writes `writeBootstrapArchiveV1` (`:118-154`).

Scope:

- The entries are the `put` entries of the folded store manifest, sorted (`:121`), minus `unsafe` paths (`:38`). Links are not included (`:117`). Each entry is read from the local worktree and verified against its manifest hash; a missing, busy, changed or different file is skipped (`:96-115, 137-139`).
- The list of entries comes from the store manifest. The bytes come from the local worktree and are verified against the manifest hash. A `.git` path therefore appears only if the store holds it as a `put`.
- It does not exclude `.git` explicitly. `.git` is included by whatever the store manifest contains and by the boundary's shared rule (see section 3).

Current archive (`/Users/Shared/semio-store/v1/bootstrap`):

- `manifest.json`: `{"version":1,"hlc":"1794522008459-000003-g1-server","files":272338,"bytes":11592555877,"sha256":"5423f2d4..."}`. The HLC decodes to 2026-11-12 22:26:48 UTC.
- `worktree.tar`: 12,085,576,704 bytes, mtime Nov 12 23:25 CET.
- Entries under `.git/` in the tar: 54 (listed with `tar -tf`).
  - Shared by the contract: `.git/HEAD` and `refs/**` (`refs/heads` 2, `refs/remotes` 2, `refs/stash` 1), `packed-refs` (1), `objects/pack` (7), `objects/info` (5), `info/refs` (1).
  - Private by the contract, yet present: `.git/index` (1 entry, the g1-server index, a ~69 MB file on disk), `.git/config` (1), `.git/logs/HEAD` (1) and `.git/logs/refs` (5), `.git/cursor/crepe` (6, an IDE directory), 14 `.git/hooks/*.sample` files.
- No `.lock` or `tmp_` entries under `.git/`, and no `.cache/` entries.
- Windows: the executable bit is ignored (`bootstrap TS:171, 213`), and links are never in the tar, so a Windows worker creates them only if Developer Mode is on.

## 6. Live state of the replica on this Mac (read-only)

- Process 7176 (`ueli`) is `bun ./...script.ts run`, pid from `.cache/f/daemon.lock`. The relay and the cache are also running (`ps -axo pid,user,command`: relay with `--peers g2=/Volumes/sem...`).
- The local `.git/index` is 69,283,078 bytes (`ls -la /Users/Shared/semio/.git/index`, mtime Nov 13 00:19).
- `.cache/f/state.json` is 105 MB, `state.log` 83 KB, `lost/` has at least five `<hlc>` directories (`1794522387076-…-g1-m01`, `1794523956832-…-g1-m02`, `1794525093574-…-g1-m02`, `1794525096655-…-g1-m02`, `1794525396722-…-g1-m02`), consistent with the `overwritten` churn described in `TICKET/1113-git-state-contract.md:8`.
- This Mac's LaunchDaemons in `/Library/LaunchDaemons`: `tech.semio.fleet.cache`, `.compact`, `.peer-mounts`, `.relay`, `.replica` (no `store-mount`, which only a worker has).

## 7. Open items the coordinator must decide (no speculation here)

- Upgrade of already joined workers: the replica code is not self-restarting yet (see section 3), so the stop-once procedure of `TICKET/1113-git-state-contract.md:65-67` is the only path. It needs the `fleet` command, which does not exist yet.
- Windows restart after exit 0 is not provided by the task definition (section 2).
- `g1-m02` and `g1-m03` platforms are not in any file. Their join prompts depend on it.
- `g1-m03` looks stopped. Whether it is a machine that must be restarted or retired is a coordinator decision; origin ids are not reused.
