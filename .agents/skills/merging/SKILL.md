---
name: merging
description: Importing an external git history (vendor or upstream tree) on the archivist machine. There is nothing else to merge in the fleet.
---

> **ARCHIVIST MACHINE ONLY.** This repo is a shared worktree edited live by a fleet of machines (see the root `README.md`, "Shared worktree"). There are no branches, merges, pull requests or merge queue for daily work, and `.git` is never shared. Fleet workers must **never** run git for this; if you are not on the archivist machine, stop and do nothing.

# Merging

## What remains true

- **Fleet work needs no merge.** Concurrent edits are replicated live; same-file races resolve last-writer-wins. Nothing is integrated afterwards.
- **History stays linear on `main`.** The archivist commits its replica on a timer (see `.agents/skills/commit/SKILL.md`). It never creates merge commits, feature branches or release branches (releases are tags on `main`).

## The one case: importing an external git history

Use this only when a tree with its own git history (a vendor or upstream project, a repo that is being absorbed) must enter the repo.

1. On the archivist machine, fetch the external history into a throw-away location **outside** the shared worktree (for example a bare clone under `.cache/`, which is ignored and never replicated).
2. Export the wanted tree state (`git archive` or a checkout into a temporary directory) and copy the files into their target folder in the archivist's replica. Respect the root `.gitignore`: ignored paths are not shared.
3. From here on the files are ordinary edits: the replica daemon copies them to the fleet like any other change, and the archivist's next timer commit archives them as a normal linear commit on `main`.
4. Record the provenance (upstream URL, commit hash, license) in the commit bullets or the folder's README, because the external history itself is not kept.
5. Delete the throw-away clone.

Do not use `git merge`, `git subtree`, `git pull` or a rebase of the foreign history onto `main`. Do not run any of this on a fleet worker.

Related: `.agents/skills/commit/SKILL.md`, `.agents/skills/micro-commit/SKILL.md`.
