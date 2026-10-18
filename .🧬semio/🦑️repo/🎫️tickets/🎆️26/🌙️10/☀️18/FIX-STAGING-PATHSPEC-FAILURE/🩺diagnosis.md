# 🩺 Staging Pathspec Failure

## 🩺 What failed

Source Control reported:

```
error: daemon terminated
fatal: pathspec ':(literal)…/📊️metrics/📦️packages/🐹️go/⏱️benchmark.go' did not match any files
```

`git add` stops the whole batch on the first missing pathspec, so nothing in that batch was staged.

## 🩺 Cause

`core.fsmonitor` is true. `.git/fsmonitor--daemon.ipc` was a stale socket from 2026-10-06. Nothing was listening on it. `git fsmonitor--daemon start` failed with `could not start IPC thread pool … File exists`, and every git command printed `error: daemon terminated`.

The missing path is not a file. The Go benchmark source is the directory file `⏱️benchmark/🐹️.go`. A dead monitor reply collapsed that into `⏱️benchmark.go`, and literal pathspec matching then failed.

Stripping U+FE0F from a real emoji path produces the same `:(literal)… did not match any files` error. The names on disk keep the variation selector. The phantom path does not.

## 🩹 Fix

Removed the stale socket and lock, then started the monitor. `git fsmonitor--daemon status` now reports that it is watching this worktree, and `git status` no longer prints `daemon terminated`.

Staged every unstaged path that exists on disk, plus tracked deletions whose blob is still in `HEAD`. Worktree bytes of sampled modified files were unchanged. Index blob ids of the held paths were unchanged.

## 🛟 Held on purpose

These paths are added or modified in the index and deleted in the worktree. Staging them would drop the only indexed copy. They stay unstaged.

- 20 `AD` paths under `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/…/c5h-tools`, `…/pai-tools`, and `zz-gocomp-stage`
- 2 `MD` paths: `📚️library/🏗️authoring/📦️crate-tree/🗺️plan/🟦️.ts` and `📚️library/🔍️discovery/🟦️.ts`
