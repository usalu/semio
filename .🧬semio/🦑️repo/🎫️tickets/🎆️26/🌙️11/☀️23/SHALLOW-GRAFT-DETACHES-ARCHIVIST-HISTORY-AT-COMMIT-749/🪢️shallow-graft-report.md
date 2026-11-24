# Shallow Graft Detached Commit 749 From 748

## What happened

- Commit 749 (`3d62b2ef22c`) was never damaged. Its object names 748 (`3cde99d423d`) as parent, locally and on GitHub
  (`gh api repos/usalu/semio/commits/3d62b2ef…` → parent `3cde99d4…`), and all 767 commits are in the object store.
- On 2026-11-23 12:40 a `.git/shallow` holding exactly `3d62b2ef22c562d836e27f09600a7f24fe84f5e8` appeared in this checkout.
  Git treats every commit listed there as a root, so `git log` ended at 749 (6 commits instead of 767) and every client
  drew 749 without a parent.
- Cause: the replica daemon shared `.git/shallow` (`GIT_SHARED_FILES` in `🤝️shared-worktree/🚧️boundary/🟦️.ts`, also in
  the heal list, the adopt "fleet wins" list and the attest git paths). The worker onboarded from a sparse shallow clone
  had a boundary at 749; the daemon published the file and every machine applied it. A shallow file describes what one
  object store lacks, so it is machine state and must never travel.
- Risk had it stayed: a squash, rebase or gc on the archivist would have written the cut into real history.

## Repair of the history

- The daemon on this machine restarted on the new code at 14:03 and removed the graft itself
  (`.cache/f/lost/1795439011903-git-shallow/shallow` keeps the file). `git rev-parse --is-shallow-repository` is `false`,
  `git rev-list --count HEAD` is 767, `git log -1 --format=%P 3d62b2ef22c` is `3cde99d423d…`.
- No history was rewritten, nothing was pushed, no ref moved.
- Every other machine repairs itself the same way once its daemon runs the new code.

## Prevention

| Layer | Change |
| --- | --- |
| Boundary | `.git/shallow` is off the shared allow-list (`gitSharedV1`), off `healGitV1`, off adopt's `GIT_FLEET_WINS`, off attest's `gitPathV1`; Python oracles and fixtures follow. A shallow file is neither published nor applied. |
| Daemon | `repairGitShallowV1` (`🌿️git/🟦️.ts`): each git round looks for `.git/shallow`; if every commit, tree and blob behind the parents of the listed commits is present, the file moves to `lost/<ms>-git-shallow/` and is removed. If the history is really absent the origin notes `git-broken-history-shallow` (fleet warns `git-broken`) and looks again every 10 minutes. |
| Archivist | `repo micro-commit` and `repo commit` refuse on a shallow repository (`gitShallow`, texts `micro.history-is-shallow` / `run.history-is-shallow`, en + de). |
| Docs | `🤝️shared-worktree/README.md`: allow-list corrected, new paragraph "The shallow boundary (2026-11-23)". |

## Verification

- `🌿️git/🧪️tests/🔧️config.ts`: 5 pass (2 new: graft over full history removed and kept in lost; real depth-1 clone and unknown commit left alone; git's own `rev-list --count` / `--is-shallow-repository` as oracle).
- `🔁️replica/🧪️tests/🩹️heal.ts`: 9 pass (1 new: a shallow file written on one machine yields no event and never reaches the other).
- `🚧️boundary/🧪️tests/🟦️.ts` 73 pass, Python oracle 0 mismatches (gitignore vectors skipped: `pathspec` not installed).
- `🩺️attest/🧪️tests` 149 pass, `🐍️agreement.py` 281543 vectors, 0 mismatches.
- `🔁️replica/🧲️adopt/🧪️tests` 30 pass.
- `📝️commit/🌳️git/🧪️tests/🟦️.ts` 1 pass (new), `📝️commit/📖️texts/🧪️tests` 3 pass.
- Literal-deadline law: fails on 16+ files of other work; `🩹️heal.ts` is not among them (its new test derives the deadline from `stretchedBudgetMillisecondsV1`).

## Not done / open

- The suites were run with `bun test` directly, not through `repo test`.
- The commit refusal was not exercised end to end on a shallow checkout (the graft was already gone); only `gitShallow` is tested.
- The shallow worker keeps its own boundary until the shared objects behind it are complete; it shows as `git-broken` (`history-shallow`) in `fleet` until then. Other machines were not inspected from here.
- `CLAUDE.md` says ".git is never shared"; the replica does share `HEAD`, `packed-refs`, `refs/**` and `objects/**`. Left as is.

## Scripts

- `make-shallow-private.py`, `wire-shallow-repair.py`: the exact, fail-closed replacements used for the edits (already applied; re-running fails on the count check).
