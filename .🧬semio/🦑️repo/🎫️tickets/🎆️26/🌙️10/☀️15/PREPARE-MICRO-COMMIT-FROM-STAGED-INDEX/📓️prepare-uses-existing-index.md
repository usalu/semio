# Prepare Uses the Existing Index

`micro-commit prepare` in `runMicroCommit` called `stageMicroCommitChanges`, which runs `git add -A` and retries around nested gitlinks. The skill then ran `micro-commit stage` before `micro-commit diff`, so a trigger committed the whole worktree.

Prepare now reads `git diff --cached` only. An empty index exits with `nothing is staged` and does not add paths. The explicit `stage` subcommand remains for a person who chooses to stage; the skill does not call it.

The check is a temporary git repository: porcelain and cached names before `prepare` must equal the same git output after `prepare`.
