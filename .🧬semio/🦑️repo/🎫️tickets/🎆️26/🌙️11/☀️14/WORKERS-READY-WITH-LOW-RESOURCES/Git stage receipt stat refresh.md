# Git stage receipt stat refresh

Incremental stage receipt updated the index with `git update-index -z --replace --index-info` and then started an unrestricted `git update-index -q --refresh --ignore-missing`. The latter re-reads the index and checks worktree paths outside the batch. The coordinator observation for this ticket was an approximately 81.7 MB index with about 350,000 entries and 1,831 staged path changes.

The receive path now leaves stat refresh to subsequent Git commands. The existing Git batch still applies exact NUL-terminated paths and the supplied object IDs and modes. Its removal ordering, conflict handling, locking, and settlement are unchanged. Whole-stage adoption still uses Git to reconstruct and verify its temporary index and refresh stat data before installing it.

Git 2.54 checks pathspecs in both index preload and refresh. Its pathspec matching loop may visit each supplied pathspec for an unrelated index entry. Although `git add --refresh --pathspec-from-file=- --pathspec-file-nul` is a native stat-only operation with literal pathspecs, it still reads the complete index and can multiply the matching work by the batch size. [Git refresh loop](https://github.com/git/git/blob/v2.54.0/read-cache.c#L1387-L1469) · [Git pathspec matching](https://github.com/git/git/blob/v2.54.0/dir.c#L485-L550)

## Verification

- The focused private fixture failed against the old code because receiving the stage refreshed the unrelated tracked `b.txt`. The same test passed after removing the eager refresh.
- The fixture checks the received tree ID against the existing language-neutral fixture and Python oracle driving native Git. It verifies an index deletion, a file-to-directory transition, executable mode through the tree, and a staged addition with a different unstaged working copy. The working copy remains byte-identical. Porcelain status agrees with a copy of the index refreshed through the old command.
- A separate tiny native Git 2.54.0 probe compared a clean content match at a literal path containing square brackets, before and after a manual refresh. `git status --porcelain=v1`, `git diff --raw`, and the `git diff --quiet` exit code agreed. Stat-sensitive plumbing differed. Before refresh, `git diff-files --raw` reported `M` and `git diff-files --quiet` exited 1. After refresh, there was no raw change and the quiet exit code was 0.
- A repository-wide fixed-string source search found no existing `diff-files` caller. The search respected repository ignore rules and excluded the ticket tree and lockfile.
- The complete Git test file passed 53 tests (411 assertions), including whole-index recovery, preservation of unchanged cached entries, split and sparse indexes, index contention, and unmerged entries. All tests ran through `repo tools exec --light` and used private repositories.
