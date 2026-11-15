# g1-server: One Canonical Repository At `~/Documents/semio` (2026-11-14)

Status (16:58 CET): **Phase 1 and Phase 2 done and verified. Image re-binding landed in both members (storage 12:52 to 13:03, replica `relocate` 13:03, lock released 16:58 after all 47 test files of the shared-worktree module passed on the live tree). Proof before the switch: storage tests and `repo cache status` exit 0, `repo tools list` exit 0; the `repo show projects` proof was interrupted by the coordinator (graph commands are forbidden on this Mac until further notice), so the first graph read after the switch is the live proof. Phase 3 prepared, not started: no go for the switch yet. Nothing deleted.** Waiting for: the coordinator's `go`, the owner's answers to decisions 2 and 3 below (the owner said "Yes, with the go" at 16:48, read as agreement to switch at the go, not as an answer to 2 and 3), and the owner at the keyboard for the one sudo script.

Goal: exactly one semio repository on this Mac, at `/Users/ueli/Documents/semio`, being the live fleet tree of origin `g1-server`, holding every piece of local work of the old checkout; the old copy deleted; fleet healthy.

Files of this task in the ticket folder: this report, `g1-server-canonical-decisions.tsv` (every path decision), `g1-server-canonical-refs.tsv` (refs created), `g1-server-canonical-staged.patch` (the old stage), `🧰️scripts/1114-g1-server-canonical/` (plan script, switch script).

## Decisions The Owner Has To Take

1. **73 conflicts, limit 50: answered by the coordinator before the apply (accepted, live wins for all 73; the 13 source conflicts are 12 November work that the fleet has redone since).** 60 are generated `Cargo.lock` files, 13 are source files (list below). Rule applied so far: live version stays, the old version is saved under `.cache/f/lost/old-repo-g1-server/conflict/<path>` (done, 73 files, hashes checked). Recommendation: accept, and take the 47 clean edits and 7 new files into the live tree (they are published to the fleet).
2. **Three linked worktrees of the old repository** (`~/Documents/semio-kinan-wip` 28.5 GiB, `~/Documents/semio-niloufar-wip` 9.1 GiB, `~/Documents/semio.worktrees/2` 29.4 GiB). All three are clean (no modified, added or untracked file), their branches are pinned in the live repository. They stop working when the old `.git` goes (their `.git` file points into it), and the repository rules forbid worktrees. Recommendation: delete them together with the old checkout (67 GiB). `/Users/Shared/node_modules` (five symlinks into `semio-niloufar-wip/node_modules`) goes with them.
3. **Boot behaviour changes.** At `/Users/Shared/semio` the four services are system LaunchDaemons and start at boot. `~/Documents` is a protected folder, so the provision script renders them as LaunchAgents of `ueli`: they start at login, not at boot, and the first start asks once for the consent "repo would like to access files in your Documents folder". This Mac has no automatic login (`autoLoginUser` unset, FileVault off). Without automatic login a reboot leaves the fleet without replica, relay and cache server until `ueli` logs in at the screen. Recommendation: switch on automatic login for `ueli` (System Settings > Users & Groups).
4. **Native bootstrap marker: answered by the coordinator (the provision run must not block on the native bootstrap, it is being fixed separately).** The switch script therefore writes the marker only for the duration of the provision run and removes it afterwards, so `preflight` keeps showing the native build as pending. `.cache/f/native-bootstrap.done` does not exist in the live tree, so the provision run would end with the full native bootstrap (`workspace:setup`, 30 minutes or more, exit 3 when it fails). The fleet is ready before that step starts.

## Phase 1: Inventory (read-only)

| | live fleet tree | old checkout |
|---|---|---|
| path | `/Users/Shared/semio` | `/Users/ueli/Documents/semio` |
| joined | yes, origin `g1-server`, group `g1`, store `/Users/Shared/semio-store` | no |
| HEAD | `5ececaa78b9265fd72b4f7e8f411afac3c9e1df1` | `5ececaa78b9265fd72b4f7e8f411afac3c9e1df1` (equal) |
| branch | `🐙ueli/⛳wip` | `🐙ueli/⛳wip` |
| status paths | (fleet stage) | 127: 120 `M ` and 7 `A ` staged, nothing unstaged, nothing untracked |
| stashes | 3 | 18 |
| refs | 223 | 180 |
| packs | 5 packs 11.32 GiB (+1 added here) | 6 packs 4.95 GiB, 20 garbage temp files 4 MiB |
| linked worktrees | none | 3 (see above), all clean |
| size | `.cache` 41.2, `🧰️framework` 24.9, `.🧬semio` 21.9, `.git` 13.8, `🌎️hub` 9.3, `👴️leutwiler` 7.9, `✏️s` 3.0 GiB | `.cache` 145.6, `🧰️framework` 24.9, `.🧬semio` 17.8, `.git` 15.7, `🌎️hub` 9.3, `👴️leutwiler` 7.9, `✏️s` 3.0 GiB |

Both on `/dev/disk3s5` (`/System/Volumes/Data`, device 16777234): a rename, never a copy. Free: 89 GiB.

Further semio repositories (search of `/Users/ueli` and `/Users/Shared` to depth 4 for `.git` + `AGENTS.md` + `.🧬semio`): none besides the two. The three linked worktrees above are checkouts of the old repository. `/Users/Shared/semio-store` is the group store and stays.

Old `.cache` (145.6 GiB) is only build-tool disk images under `.cache/b`: `r` 96.0, `p` 24.0, `t` 8.0, `d` 8.0, `g` 6.0, `o` 2.0, `n` 1.5, `l` 0.1 GiB (dated 20 October to 10 November). The live tree has its own image for every one of these partitions (`g` 23.2, `d` 13.0, `t` 3.6 GiB, toolchains included), so nothing is "reusable and absent". No image of the old checkout is mounted.

What names `/Users/Shared/semio` on this machine:

| where | what | follows by |
|---|---|---|
| `/Library/LaunchDaemons/tech.semio.fleet.{replica,relay,cache,compact}.plist` | `WorkingDirectory`, `SEMIO_WORKSPACE_ROOT` | provision script (retired, LaunchAgents rendered) |
| `/Library/LaunchDaemons/tech.semio.fleet.peer-mounts.plist` | no worktree path | stays |
| `~/Library/Application Support/semio/fleet/entry/{worktree,entry.json,state.json}` | boot program's worktree | provision step 6a (`install --take-over`) |
| `/etc/exports`, `/etc/nfs.conf` | store only (`/Users/Shared/semio-store`) | unchanged |
| `/etc/zshenv`, `/etc/profile` | no path (group, NFS host, network, old cache variables) | provision step 5 rewrites the block |
| `/usr/local/bin/repo` → `~/.local/bin/repo` → `~/.local/share/semio/launcher/bin/repo` | no worktree path | unchanged |
| `/etc/semio/remote-cache.env`, `/var/lib/semio-remote-cache` | no worktree path | unchanged |
| 9 disk images mounted at `/Users/Shared/semio/.cache/b/{l,o,d,t,g,p,n,r}/m` and one nested under `n/m` | mount points inside the tree | detached before the rename |
| absolute symlinks in the tree: `node_modules`, `.nx/installation`, five `<app>/node_modules` → `/Users/Shared/semio/.cache/b/…` | link targets | see Phase 3 |
| `~/.gitkraken/{config,profiles/…/localRepoCache,profiles/…/profile}` | recent repository | after the switch |
| `~/.claude/projects/-Users-ueli-Documents-semio` | keyed by the canonical path | keeps working (the path is the same after the switch) |
| running processes | replica daemon and its three sidecars (build-agent, build-scheduler 8980/8981, build-cache 6094), relay, cache server 6090, `semio-repo-watch`, test processes of other agents | stopped at the switch |

Not naming it: `~/.gitconfig`, `~/.claude.json`, `~/.claude/settings.json`, shell profiles, `~/.cargo/config.toml`, `~/.codex/config.toml`.

## Phase 2: Merge Of The Old Checkout

### Git objects and refs (done 11:53, verified)

- Missing in the live object store: 14568 objects reachable from old refs and stashes (86 commits, 1 tag, 10872 trees, 3609 blobs) plus 33 staged blobs only in the old index. Packed read-only from the old repository into one pack of 197 MiB (`pack-06dd6af4858b57e42cd3285561900489bae8dda6`, `git verify-pack` ok) and added to `/Users/Shared/semio/.git/objects/pack` as new files (`.pack`, `.rev`, `.idx` in that order, through `tmp_` names and renames; nothing existing touched).
- 41 refs created with `git update-ref` under `refs/old-repo/g1-server/` (`g1-server-canonical-refs.tsv`): `HEAD`, `stash-0` … `stash-17`, `stash`, 9 branch heads (`heads/2`, `b1`, `b3`, `b4`, `backup`, `kinan/bericht-diff`, `worktree-agent-afa905cc164f52ed8`, `🐳kinan/⛳wip`, `🦢niloufar/⛳wip`), 3 tags, 3 `agents/*`, 4 `codex/*`, 2 gitkraken refs. Every other old ref exists in the live repository with the same value.
- Proof: every ref resolves to its recorded id, `git cat-file -e` passes for all 41, and `git rev-list --objects` over the 41 refs walks 1975972 objects without a missing one.
- Kept local only (not replicated, 733 MiB) in `.cache/f/lost/old-repo-g1-server/git/`: `reflog-only/` (7 commits that only the old reflogs reach, 179 objects, 7.0 GiB uncompressed, as a verified pack plus `commits.txt`; not put into `.git` because it would be journaled to every machine), `meta/` (old reflogs, `HEAD`, `config`, `packed-refs`, `index`, `FETCH_HEAD`, `info/exclude`, five hooks, worktree list).

### Stage

`git diff --cached --binary` of the old checkout: `g1-server-canonical-staged.patch` (346 KiB, 127 paths).

### Working tree (applied 12:05 after the coordinator's answer)

The local work of the old checkout is exactly its 127 staged paths. Decided by content: the old file, its version in old HEAD and the live file compared by SHA-256, cross-checked with the replica's own read-only adoption plan (`planAdoptV1`, journal and git evidence; `🧰️scripts/1114-g1-server-canonical/plan.ts`, 122 s). Both agree on every path.

| decision | paths | state |
|---|---|---|
| take (live equals old HEAD, the old change is new work) | 47 | applied: each live file compared again at the moment of writing and still equal to old HEAD, including the 6 paths under `📚️library/` that the coordinator asked to guard |
| copy in (absent in live, never deleted by the fleet) | 7 | applied (still absent at the moment of writing) |
| conflict (changed on both sides): live stays | 73 | old versions saved in `.cache/f/lost/old-repo-g1-server/conflict/` |
| same after line-end normalisation (old file has CRLF, git status clean) | 5 | nothing to do |
| stale | 0 of the 127 | |

Record of the apply: `g1-server-canonical-applied.tsv` (54 rows, all `applied`, hash of the old file and of the live file before the write, time). Each file was written beside its target and renamed over it, and its hash checked afterwards.

The 7 new files: `1112-audit-directcmds.md`, `1112-audit-unauthorized.md`, `1112-rename-fieldprep.files`, `1112-result-floweditor.md`, `1112-result-graphslice.md`, `1112-result-puzzlefinal.md` (ticket `26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS`) and `resolution-2026-11-12-memory.md` (ticket `26/10/04/SOLVE-ALL-MERGE-CONFLICTS`).

The 13 conflicts that are not lockfiles (a trial three-way merge with old HEAD as base says whether the two changes overlap):

| path | trial merge |
|---|---|
| `📋️build-owners.json` | overlaps |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts` | clean |
| `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🪡️field/📦️packages/🦀️rust/Cargo.toml` | overlaps |
| `…/🪡️field/🦀️.rs` | overlaps |
| `…/🪡️field/🧳️preparation/📦️packages/🦀️rust/Cargo.toml` | overlaps |
| `…/🪡️field/🧳️preparation/📦️packages/🦀️rust/tests/close.rs` | overlaps |
| `…/🪡️field/🧳️preparation/📦️packages/🦀️rust/tests/producer.rs` | overlaps |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts` | clean |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts` | clean |
| `…/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🌡️pressure/🟨️.mjs` | overlaps |
| `…/👷️slice-pool/🧱️limit/🟨️.mjs` | overlaps |
| `…/📚️library/🏗️build/🏡️resident/🧪️tests/🛑️cancel/🟦️.ts` | overlaps |
| `…/📚️library/🏗️build/🧠️memory/🟨️.mjs` | clean |

The rest of the old tree: 2243 tracked files differ from the live tree and are unchanged since old HEAD (or equal an earlier fleet version): their bytes are in git, the fleet's newer version stays. 1490 files exist only in the live tree.

### Ignored data: dropped (nothing moved)

| what | size | why |
|---|---|---|
| `.cache/b` images of the old checkout | 145.6 GiB | every partition exists in the live tree; old build output |
| `.cache/{boot-recovery,command,f,scratch,semio,test-sandbox}` | under 10 MiB | process scratch |
| 76 ignored scratch folders `🧪️purity-*` in ticket `26/08/17/END-TO-END-TAXONOMY-NORMALIZATION` (nested test repositories, 16 to 26 October) | 70 MiB | test leftovers |
| five hidden temp folders `<app>/.<7 hex>` (7 November) | 3 MiB | tool leftovers |
| 2037 other ignored entries (`.venv`, `.nx`, `node_modules`, `dist`, `bazel-out`, ticket `🗑️generated`, …) | | the live tree has an entry at the same path; build output |
| 20 `tmp_obj_*`/`tmp_pack_*` files in the old `.git/objects` | 4 MiB | interrupted git writes |

## Phase 3: Relocation (prepared, not started)

How the workers did it: `relocate` into the account's existing Documents repository (identity carried, store adopted there, old tree retired). Here the task asks for the other form, which the same-volume layout allows: the joined live tree itself is renamed to the canonical path, so machine identity, cursors, clock, `state.ckp`, the cache sidecar's data (`.cache/f/cache-server`, `identity/`) and the build images keep their place inside the tree and no adoption runs. The provision script then finds a joined machine at the canonical path (`machine.json` with origin `g1-server`) and refreshes mounts and services for it.

### The owner's one script

`🧰️scripts/1114-g1-server-canonical/switch.sh` (in the ticket folder; before the switch it is copied together with the topology directory to a folder outside both trees, and run from there as `ueli`; it asks for the sudo password once). Two lines: it stops the four services, detaches the cache images, renames the old checkout to `~/Documents/semio.old-20261114` and the live tree to `~/Documents/semio`, then runs the fleet's own provision script for the server role, which renders the services for the new path. It deletes nothing, and any failure before the renames restarts the old services with both trees untouched.

Steps and why in this order:

1. Checks: run as `ueli` from outside both trees, both trees on one volume, live tree joined as `g1-server`, target name free, no landing lock held, no image of the old checkout attached.
2. `sudo launchctl bootout system/tech.semio.fleet.{replica,relay,cache,compact}`: unloaded jobs are not restarted by `KeepAlive`, so nothing comes back at the old `WorkingDirectory`. The plists stay in `/Library/LaunchDaemons` until step 6 retires them, which is what makes the rollback possible.
3. Every process that still holds a file in the tree is stopped by pid (`kill`, after 12 s `kill -KILL`): the sidecars, `semio-repo-watch`, test processes.
4. `hdiutil detach` of the ten image mounts under `.cache/b` (nested ones first). A mounted volume inside a renamed folder keeps its old mount path in the kernel, so the images must be detached before the rename.
5. `mv ~/Documents/semio ~/Documents/semio.old-20261114`, `mv /Users/Shared/semio ~/Documents/semio` (same volume: two renames, no copy).
5b. The hint `image-relocation.json` is written into `.cache/b` of the moved tree, every partition is admitted and reported (`bound`/`unbound`), refused ones are set aside, `repo cache status` is the end-to-end probe (see the finding and the re-binding below).
6. `provision.sh g1 g1-server --user ueli --server` with `SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.71 SEMIO_FLEET_NETWORK=192.168.178.0/24` (the values this server was provisioned with), from the copy of the topology directory. What it does on a joined server: removes a root `node_modules` link that points outside the tree, rewrites the `semio fleet` blocks of `/etc/exports`, `/etc/nfs.conf`, `/etc/zshenv`, `/etc/profile`, installs the boot program for the new worktree (`install --take-over`), removes the four system daemons and writes and loads the four LaunchAgents of `ueli` (`~/Library/LaunchAgents/tech.semio.fleet.{replica,compact,cache,relay}.plist`, logs in `~/Library/Logs`), waits up to 90 s for the replica (the consent prompt appears here), then the native bootstrap (decision 4).

Differences the re-render brings besides the path (taken from the rendered definitions, compared with the installed ones): LaunchAgents with `LimitLoadToSessionType Aqua` instead of system daemons; the replica starts through the boot program (`boot.sh run`); the cache server binds `--host fleet` instead of `0.0.0.0`; `ThrottleInterval` 5 instead of 2; the `SEMIO_REMOTE_CACHE_ENDPOINT/ACCESS/INSTANCE` variables leave `/etc/zshenv`, `/etc/profile` and the service environment (the endpoint comes from the fleet facts); `/etc/exports` gains the read-only line for the peer groups when their host names resolve.

### Finding: the build-cache images do not survive a move of the worktree (verified in the code)

Each image is described by `.cache/b/<partition>/image.json`, which records `ancestry`: the inode of every parent folder up to `/`. The opener compares it on every open and refuses with `Original Native image ancestry changed` (`🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🟦️.ts`, `requireNativeImageAncestryBindingV1`; the chain is `b`, `.cache`, worktree, `Shared`, `Users`, `/` today and gets one entry longer under `~/Documents`). The receipt is never rewritten and a valid receipt with another ancestry triggers no reset, and the leases image `l` is opened by every command, so after a plain rename every `repo` command would refuse. Dependency generations inside `d` additionally hold absolute links into `/Users/Shared/semio/…` ("receipts survive no relocation"). There is no command to detach, re-attach or rebind images.

The coordinator did not accept cold caches as the designed outcome and asked for a proper re-bind. Built as follows.

### Image re-binding after a verified move (storage member `🌐️platform/🥾️boot`, opener, replica `relocate`)

What the ancestry check protects against: an image reached through another path (a link or alias), an image directory that was replaced by another one at the same path, and a receipt that was copied to another place together with a copy of the image. A rename on the same volume is none of these: the backing file, the underlying mount directory and every directory below the moved one keep their inodes.

Design (schema first: `AncestryRelocation`, `AncestryRelocationHint`, `AncestryRebinding`, `AncestryRebindProtocol`, `AncestryRelocationCase` in the storage schema):

- `rebindNativeImageAncestryV1({directory, movedFrom, movedTo})` rewrites only the `ancestry` of one receipt, and only when all of this holds: the publication is settled (no `.publication`), the receipt is schema-valid, `fixed.dmg` is one regular file with one name whose inode and size are the published ones and which lies on the directory's volume (a copy has another inode: refused), the underlying mount directory `m` is the published inode and mode, lies on the same volume and is empty (so the image is detached), the published chain equals exactly "the current inodes from the image directory's parent up to `movedTo`, followed by the parents of `movedFrom`" (the recorded old place: a foreign image or another origin is refused), and the old path no longer reaches the same directory (an alias is refused). The new receipt is written beside the old one, synced, the facts are checked again, and it is renamed over the old one; all other fields stay byte for byte.
- `recordNativeImageRelocationV1` writes the hint `image-relocation.json` (`{version, movedFrom, movedTo}`) into a directory above the moved images; the hint grants nothing by itself.
- `admitNativeImageRelocationV1(directory)` is called by the image opener (`inspectPersistentReceiver`) before it reads the receipt: when the published chain differs from the current one and the nearest ancestor holds a hint, the re-binding above runs; any refusal leaves everything as it was and the inspection refuses as before. On a machine without a hint the only added work is one read of the receipt. Nested images (the Nx namespaces inside `n`) are healed the same way when their parent is attached, because the search for the hint walks up through the mount.
- Replica: `carryImagesV1(from, to)` in `🔁️replica/📍️relocate`, called by `relocate --phase move` after the adoption: when the new tree has no `.cache/b`, the old tree's `.cache/b` is renamed into it as a whole (same volume, no image attached) and the hint is written. It never throws and a new tree with caches of its own keeps them. The replica does not import the build code (its code closure stays as it is); the hint's shape is checked against the storage schema in the relocate test.
- This server (rename of the whole tree): the switch script writes the hint (`movedFrom` `/Users/Shared/semio`, `movedTo` `/Users/ueli/Documents/semio`), runs a check that admits every partition and prints `bound`/`unbound` per partition, sets aside only the refused ones, then probes with `repo cache status`; only if that is refused by the image layer the whole folder is set aside (cold caches as the fallback).

Tests (written first, in a scratch copy; then the landing was rehearsed on an APFS clone of the modules):

- `🥾️boot/🧪️tests/🚚️relocation/` (`🟦️.ts`, corpus `🔣️.json` of 13 cases validated against `AncestryRelocationCase`): worktree renamed, partition directory moved between worktrees, already current, copied image, hard-linked image, resized image, foreign receipt ancestry, other declared origin, origin still reaching the image, filled root, replaced root, pending publication, malformed receipt; every refusal leaves the receipt bytes and no temporary file; the accepted ones are idempotent, validated with Ajv against `ImageReceipt`/`AncestryRebinding`, compared with an SQLite JSON oracle, and a copy made after the re-bind is refused again. Second test: the opener's admission with and without hint, with a copied image, a foreign hint, a malformed hint, for a top-level and a nested layout. 2 tests, 165 assertions, pass.
- Real data: the detached leases image of the old checkout (`~/Documents/semio/.cache/b/l`, receipt written by the tool on 20 October) was moved to another directory and back: refused before, `rebound`, bound after, a copy of it refused (`Original Native image backing required`), and after the way back the receipt is byte-identical to the original.
- Relocate member: new test "the build cache images go along to a tree without any and bind again through the hint" (kept / failed / none / carried, hint validated with Ajv against the storage schema, the storage admission binds the moved image); 12 tests pass in the clone.
- Regression: all 80 storage test files run directly with bun on the untouched live tree and on the modified clone. Same result in 76; the 4 that differ fail in the clone on "workspace root … not found" or skip (the clone is not a workspace) and one was a timing flake that passes alone. 16 files already fail at baseline when run this way ("Original parent Script process invocation required": they need the repo test runner).

Limits, stated plainly:

- Not proven by a run of the real opener on a moved real partition before the switch; the switch itself is that run, with the per-partition check and the fallback.
- State inside the images that names the old absolute path is not re-bound by this: dependency generations in `d` hold links into `/Users/Shared/semio/…` (their own layer decides whether to re-link or reinstall; packages N1 and N3 work there), lease rows in `.cache/b/c` are keyed by path (new rows are created), cargo fingerprints carry absolute paths.
- The other three places that compare the chain (`🧹️whole`, `🍎️image`, `🧱️fixed`: retirement paths) do not admit a relocation by themselves; they see the re-bound receipt after the first inspection.
- Two moves without any open in between are not chained (the second hint replaces the first): such an image is refused and created again.

Not affected by any of this: replica state, `.cache/f/cache-server`, `.cache/f/identity`, `/var/lib/semio-remote-cache`, `~/Library/Caches/semio/runtimes`. Stale leftovers that name the old path: `~/Library/Caches/semio/lease-registry/<hash>/directory.json`, `~/Library/Caches/semio/scheduler/state.json` (checked in Phase 4).

### After the script (no sudo)

Path-keyed leftovers are corrected (absolute links inside the tree, GitKraken's recent list), then Phase 4.

## Baseline Before The Switch (about 11:55–12:05, live tree at `/Users/Shared/semio`)

- `status`: daemon running, synchronisation on, 278595 files tracked, sidecars build-agent, build-scheduler, build-cache running, `Git state: shared`, every origin 0 events behind.
- `verify`: exit 1, drift on 4 of 273929 paths (`…/☀️14/WORKERS-READY-WITH-LOW-RESOURCES/workers/g1-m0{1,2,3,7}/round-1.samples.txt`, files that other sessions write at this moment), 1 path refused as too long.
- `fleet`: exit 1, "not in agreement" on a moving tree: g1-m01, g1-m03, g1-m07 `ok`; g1-server, g1-m02, g1-m06, g1-m05 `lagging (not settled)`; g1-m04 `diverged` (view digest); g1-m08 `stale`.
- Listening: 6090 (cache server, pid of `tech.semio.fleet.cache`), 6094 (build cache sidecar), 8980 and 8981 (scheduler), the last three on 192.168.178.70.
- Landing locks held at 11:52: `1113-landing.lock` (`claude-package-AD`, 11:43) and `landing.lock` (`N4`, 11:50).

So the checks of Phase 4 are compared with this baseline: a `verify` drift on files that are being written and a `lagging` verdict are the state before the switch, not a result of it.

## Not Verified Yet

- Everything of Phase 3 and Phase 4.
- That the first `repo` command at the new path creates fresh images without help (the code path for an absent `.cache/b` is the one of a new worker; not run here).

## Owner's Answers And What Was Done (2026-11-14, 17:00 to 18:53)

The owner answered "Yes to both" to decisions 2 and 3.

- **Linked worktrees deleted.** Before the delete: each `realpath` checked, `git status --porcelain` empty in all three, ignored content only generated output (build output, caches, virtual environments, rendered files), no image mounted, and the three branch tips resolve in the live repo: `refs/old-repo/g1-server/heads/🐳kinan/⛳wip` b2ba75e8db08, `…/heads/🦢niloufar/⛳wip` cfe96f15b66e, `…/heads/b4` 2e09d030f5bb. Deleted: `/Users/ueli/Documents/semio-kinan-wip`, `/Users/ueli/Documents/semio-niloufar-wip`, `/Users/ueli/Documents/semio.worktrees` (held only `2` and a `.DS_Store`). Free space on the volume went from 61.6 GiB to 121.9 GiB: **60.3 GiB freed**. The three `git fsmonitor--daemon` processes of those folders ended by themselves.
- **Leftover removed:** the five `ajv` symlinks in `/Users/Shared/node_modules` (made by an executor on 11-13 for tests, they pointed into `semio-niloufar-wip/node_modules`) and the then empty folder. The live tree resolves `ajv` from its own `node_modules`.
- **Automatic login: not done, the owner's step.** It is a security setting and needs the owner's password: System Settings, Users & Groups, "Automatically log in as" `ueli`. Best right after the switch.

## Switch Script Revised For The Landings Of 2026-11-14 (state 2026-11-15 00:40)

Read: `WORKERS-READY-WITH-LOW-RESOURCES/result-P6.md`, `ROUND-NOTE-P6.md`, `result-P3.md`, `1114-result-provision-skew.md`, the current provision script (7eb0cc06e2fb). Nothing live was changed. `switch.sh` is syntax-checked only, never run.

What I found on this Mac and what the script does about it (the head of `switch.sh` lists every removal for the owner):

| finding | script |
|---|---|
| Launcher is still the old single file (`launcher/bin/repo`, identifier `a.out`, 12:38); no `versions/`, no stamp, no consent marker | step 0 runs the current installer before anything stops; a failure ends the script with nothing changed |
| `/etc/zshenv` and `/etc/profile` carry `SEMIO_REMOTE_CACHE_ENDPOINT` (6090), `_ACCESS`, `_INSTANCE` (`semio`) | the current provision script rewrites the block with the three `SEMIO_FLEET_*` lines only; the script prints the lines that leave and checks afterwards |
| `tech.semio.fleet.cache` on 6090 is the pre-fleet-facts server; the facts name 6094 / `main`; no client was connected | step 6 unloads it and removes the agent file that provision renders, only when the facts name 6094 |
| The topology still renders the 6090 job for every server | not changed by me; open point for the coordinator (in `SERVER-RELOCATION.md`) |
| `~/.local/bin/go`, `gofmt` point into the live tree's tools image | re-pointed after the rename |
| `~/.local/bin/python3.12`, `python3.14` point into the OLD checkout's tools image (dangling today) | not touched; after the switch the path names the new tree's tools image, the native bootstrap re-links them |
| Graph service and probe jobs of `ueli` may be loaded for the old path | unloaded in step 1 |
| The provision script re-runs itself through sudo and needs the topology copy to equal the tree's | the script refuses a stale copy of the topology directory, of `rebind-check.ts` and of itself |
| Consent click | step 7 waits up to 10 minutes for the replica daemon and the launcher marker, then prints the `ROUND-NOTE-P6.md` checks |

Not verified before the switch: the installer run on this Mac (it needs the pinned Go, which is on `PATH` through the link into the mounted tools image), the consent dialog itself, the provision run, the re-bind against the real moved partitions.
