# 🔁 Offline Merge of g1-m07 (2026-11-25)

The replica daemon of g1-m07 was killed at about 08:59 (event loop blocked for 121 s in `follow`) and every restart timed out after 900 s until 15:46. A sampled child hung in `open` below `~/Documents` while the store was reachable. It started again without a restart by hand; why the hang ended is not known (the launcher's consent probe succeeded at 15:45:44).

It first ran fallback code of 20 November, without the line merge and without the publish fence, for about two minutes, then restarted itself onto the worktree code `96d644fb`.

| Effect | Paths | Resolution |
| --- | --- | --- |
| Fleet content installed over a local edit (`overwritten`, local bytes in `lost/`) | 10 | Three-way merge (base: last common fleet content). 9 clean, the Canvas dock by hand (window surface level of g1-m07, panel-clear tab chips of g1-m05) |
| Local edit kept over a fleet edit | `📖️procedural-descriptors` test | g1-m02's version restored; 16 of 16 tests pass |
| Older content republished over g1-m13's | `🏗️record-machine` | The fleet's stale rule restored g1-m13's version within a second |

Verification: none of the paths published by g1-m07 replaced fleet content it had not seen, apart from the two above; all 4003 files changed on this machine today are equal to the newest fleet event (5 ignored generated files stay local, 1 is unchanged from HEAD); `verify` of 471534 files showed only 6 files of g1-m03 that arrived during the run; fleet verdict of g1-m07 `ok`; UI suite on the merged tree 1724 tests pass. The merged Cargo.lock files were not compiled.

A backup of the local state is in `~/semio-offline-backup-2026-11-25` on g1-m07 (outside the worktree).
