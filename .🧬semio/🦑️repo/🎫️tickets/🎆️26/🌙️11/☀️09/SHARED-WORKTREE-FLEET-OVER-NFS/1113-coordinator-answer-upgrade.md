# Coordinator Answer to g1-m01 and g1-m02 (2026-11-13)

Both upgrade records stopped at step 1 because `verify` exited 1. That stop condition was wrong and is removed from `1113-worker-prompts.md`.

- Drift confined to `.git/` (index, logs, config, hooks, `compose-*`, `cursor/`, `description`, `info/refs`, `gkcommittemplate.txt`) is the signature of the old daemon: the old `verify` still counts those paths as shared. It clears only after the restart. Do not wait for it.
- `content differs` on the module sources (g1-m01) meant the sources were still being edited. Wait until the sha256 of `🔁️replica/🟦️.ts` and `🌿️git/🟦️.ts` is unchanged for 60 seconds.

Continue from step 2 of the upgrade prompt now: read the pid from `.cache/f/daemon.lock`, `kill -TERM <pid>` once as the worker account, wait for the new pid, then `status`, `verify`, `fleet --wait 120`, and append the results to your `<origin>-upgrade.md`.

A stage on the worker that was never published is replaced by the group server's base stage when the new code first starts.
