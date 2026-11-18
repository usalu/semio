# Git State of g1-m02 Converged With the Fleet

Date 2026-11-15, about 15:45Z to 15:55Z, on the request of the owner.

## Cause

`repo ./📜️script.ts status` said: `Git state: degraded, because git write-tree fails on this stage; nothing is published or applied until it works.` and g1-m02 had 5171 incoming events queued.

`git write-tree` failed with `Unable to create '.git/index.lock': File exists`. The file was empty, dated 06:53:08 (about nine hours old) and held by no process (`lsof` empty, no git process of the repository running).

## What I did

1. Checked that the index has 235354 entries, none missing from the object database (`git cat-file --batch-check`), no unmerged entries.
2. Copied the old index to `~/git-index-backup-before-fleet-stage-apply` (outside the worktree) before anything could replace it.
3. Removed the stale empty `.git/index.lock`. No git command that changes history, refs or the working tree was run. `git write-tree` (run once to confirm) only writes tree objects.

## Result

- `Git state: shared (stage and commits replicate); stage tree d4f57b999b0e43f416cc7fdc7987626bc133b3e9`, the stage tree of the fleet.
- Incoming queue 5171 -> 0, files tracked 293495 -> 298346, `git status` view now 50 modified, 11 deleted (7 staged), 105 untracked (fleet: `50/11/141`).
- Nothing lost: all 267 refs (`old-repo/*` branches included) and all objects are still in `.git`; the 1889 files the fleet kept from this machine are in `.cache/f/lost`; the previous stage (4211 staged deletions, 959 modified) was the old repository's index, now replaced by the fleet's stage and kept in the backup above.
- g1-m02 is `lagging (still moving)` for now; the remaining differences in `status` belong to g1-m01, g1-m03 and g1-server.
