# Fleet Setup When Each Machine's Existing Repository Is The Worktree

Replaces the worktree rules of `fleet-setup.md` (that file is not edited). Everything else there (addresses through the three `SEMIO_FLEET_*` variables, origin ids, cache token, exit codes, acceptance) still holds. Module README: sections "Existing repository, adopt and relocate" and "macOS privacy".

## Rules

- One repository per machine: the account's existing repository, turned collaborative. Canonical paths: Windows `C:\semio`, Linux `$HOME/semio`, macOS `$HOME/Documents/semio` (the topology holds templates `~/semio`, `~/Documents/semio`, `C:\semio`, resolved against the real home of the editing account, never the home of root under `sudo`).
- The store stays where it is (`/Users/Shared/semio-store` exported by the group server).
- The worktree may be empty (bootstrap archive), already joined, or the account's own repository: the last is adopted, never refused. A repository without the fleet module is refused with the instruction to update it to a revision that has the module.
- Adopt is loss-free and decided by content, never by a modification time (an old version with a fresh mtime, or a file the fleet deleted since, must not overwrite or resurrect). Classes: `same`; `stale` (an older version of a fleet file, a fleet-deleted file, or unchanged since the repository's last commit: the fleet wins, nothing is published, bytes in `.cache/f/lost/adopt-<stamp>/stale/`); `edit`/`publish` (local work, published); `conflict` (changed here and in the fleet since the base: the fleet's version stays, the local one is in `.cache/f/lost/adopt-<stamp>/conflict/` and listed in `lost/adopt-<stamp>.md`). Local stashes, tags and packed-only branches: lift them into loose `refs/old-repo/*` before adopting; a differing `packed-refs`/`HEAD`/non-branch ref goes to the fleet and the local copy to `lost/adopt-<stamp>/git`. Ignored files and nested repositories are untouched; git objects are united, histories must share a commit (`--force` otherwise), branches are linearised with every commit kept, the old index is kept in `lost/` and the fleet's stage replaces it.
- A machine that already joined elsewhere relocates (`relocate`, or automatically by the provision script when the installed replica service points to another path): same origin id, never reused, the old tree is retired and never deleted.
- Dry run first, always: `provision.sh ... --plan` (mounts the store, prints `join --adopt --dry-run` or `relocate --dry-run`, changes nothing else).

## Commands

Copy the topology directory to a temporary folder outside the worktree, export `SEMIO_LANGUAGE=en`, `SEMIO_FLEET_GROUP`, `SEMIO_FLEET_NFS_HOST`, `SEMIO_FLEET_NETWORK` together, `SEMIO_FLEET_CACHE_ENDPOINT` unset.

```
bash <temporary copy>/provision.sh g1 <origin> --user <account> --plan      # dry run, record the plan
bash <temporary copy>/provision.sh g1 <origin> --user <account>             # adopt or relocate for real
```
Windows (elevated): `provision.ps1 g1 <origin> -User "$env:USERDOMAIN\$env:USERNAME" -Plan`, then without `-Plan`. Add `--force` / `-Force` only for a repository without a common commit after the owner agreed; `--relocate-from <old>` / `-RelocateFrom <old>` names the old tree when it cannot be detected.

Exit codes: 0 done; 3 native build pending (as before); 4 (macOS) fleet ready but the agents wait for the consent to read the folder.

## macOS

Services that run `repo` (replica, compact, relay, cache) are LaunchAgents of the account in its graphical session; the store mounts stay LaunchDaemons. Log in at the screen; the first start asks once "repo would like to access files in your Documents folder": Allow. If it was declined: System Settings > Privacy & Security > Files & Folders (or Full Disk Access) > repo. The consent belongs to the launcher binary and survives runtime updates; it is asked again only after the launcher is rebuilt. The group server needs automatic login. `preflight` fails the `access` check with the exact instruction; `status` and `fleet` print `worktree-access-denied`.

## Stop conditions

A dry run that shows more than 50 `conflict` paths or more than 1000 paths of new local work (`publish` plus `edit`; the dry run prints both limits; `stale` is expected and never a reason to stop), `no-common-history`, `running` (old daemon alive), or `unverified` after `finish`: stop, record, report. Nothing is lost at any of these points; the old tree is untouched.
