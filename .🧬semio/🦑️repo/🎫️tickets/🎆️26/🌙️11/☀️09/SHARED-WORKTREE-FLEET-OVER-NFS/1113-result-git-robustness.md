# Result: git-state robustness (2026-11-13)

Module `MOD` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/`. Landed 04:49 as small edits and a three-way merge onto the live files after `1113-result-latency.md` existed (the latency batch of 04:35 was merged, not overwritten); the repo MCP did not connect, so the ticket is not closed. Contract rewritten: `1113-git-state-contract.md` (publish, apply, stage health, concurrent commits). Operator view: `MOD/README.md` section "Git state".

## Audit findings, verified against the code (the auditor ran nothing; every confirmation below was reproduced by a test that fails on the old code)

| # | Verdict | Rule implemented | Test |
|---|---|---|---|
| 3 stage publish silently disabled when the record is lost or unparsable, apply resets the local index | confirmed (old `publishIndexV1` returned `waiting` with no event; `readKnown` swallowed the parse error) | The record no longer carries the known tree. The last synchronised tree is the fold of the registers this machine has published or applied (`host.held`, the replica's `files` table), folded as `verifyIndexV1` folds, kept in memory afterwards. A lost, torn or garbage `git-index.json` therefore changes nothing; a stage made while stopped is published after the restart. When a stage has no proof of its origin (no base applied yet, lost state, interrupted base replace) the shared fold wins, but the old `.git/index` is first copied to `.cache/f/lost/index-*`, an `overwritten` event (`local-stage-replaced`, `count`, `lost`) names it, and the copy is listable with `GIT_INDEX_FILE=<copy> git ls-files -s`; if the copy fails the stage is not replaced (`backup-failed`). A deleted `.git/index` is restored from the fold (`repaired`, `index-missing`) and publishes no delete. Replaced `.git` with gone objects: the fold fails, state `waiting`/`objects-missing`, no reset | unit: "a fold-derived known tree publishes a local stage even when the local record is garbage…", "a stage with no proof of its origin is replaced…", "a missing .git/index is restored…"; replica: "a lost local record does not matter…" (old code: fails) |
| 3 (no silent states) | confirmed (`status` read only the base register) | `.cache/f/git-index.json` = `{version:1,state,reason,since,applying}` (tmp + fsync + rename, schema `StageRecord`). Each change emits one `blocked` event (`repaired` on return to `shared`); `status` prints `Git state: <reason>` (en/de texts, schema enum); attestation shows the disagreement through `indexTree` and `queue.incoming` | unit: record valid against the schema in each state; replica: "unmerged stage entries make status name the reason…" |
| 4 base publish voids other machines' staged entries; unmerged stage reset by `read-tree` | confirmed, both variants | Option (a) chosen (simplest correct): no base for a large change; entries in batches of 5000 however many paths differ. Bases only for genesis (group server, none yet) and explicit `rebase`. `rebase` reports voided registers: `published` event (`reason` `rebase`, `count`, `lost`) plus `.cache/f/lost/rebase-*.tsv` (`path`, origin of the register, `<mode> <oid>` or `absent`): every register of any machine whose value differs from the stage it publishes. Option (b) is not needed: nothing but a rebase publishes a base. A stage whose `write-tree` fails (unmerged) is never reset; the apply defers (`unmerged`) | unit: "a very large change is published as entries…" (6000), "a rebase request publishes a base and lists…", "unmerged entries are reported once…"; replica: "a base is published only for the first stage and for an explicit rebase…" (5001 bulk + 3 earlier entries survive on all three machines, no `overwritten`), "a rebase lists the registers…" |
| 5 crash windows between publish/apply and the record | confirmed (A: re-emit with fresh hlc; B: applied remote stage re-published) | A: own emitted events are in `held` after replay, so nothing is re-emitted (restart simulated with `forgetGitV1`). B: the paths an apply is writing are recorded (`applying`) before the write and cleared after settle; a publish after a crash skips them; a base replace in flight makes the publish wait | unit: "the paths an interrupted apply was writing are not published…", "a fold-derived known tree…" (restart) |
| 6 linearisation falls back to the loser's whole tree | confirmed (revert of the winner's F1 reproduced) | `mergeTreesV1`: per path, base = parent tree of the replayed commit, ours = tip tree, theirs = loser tree; only paths theirs changed against base are taken; identical changes stay; a path both changed differently (also file against directory) takes the side with the later committer time, a tie keeps the tip; no unmerged state, no whole-tree fallback; git failure retries. `linearised` event carries `count` = conflicting paths (en/de text `event.linearised.conflict`) | unit: Python oracle `🐍️merge.py` (git CLI + the rule in Python) over 8 cases, file/directory case, "linearising concurrent commits that touch one path differently…" (both time orders); replica: "concurrent commits that change one file differently…" (old code: never settles) |
| 7 (git part) `git-index.json` written without fsync, parse error swallowed | confirmed | atomic fsync write; unparsable record reads as idle, harmless now | schema test above |
| 8 git child without timeout | confirmed in `git()` | not changed: timeouts belong to the resilience agent's protocol; nothing in my paths adds a blocking call | not tested |
| 20 `.git/index` deleted | confirmed (`before===""` returned silently) | restored, see 3 | see 3 |
| 20 branch switch replicates HEAD only | refuted as a defect: the stage change of a checkout replicates as registers, HEAD and refs as shared files; the contract already forbids branch checkout as a workflow | none | none |
| 20 `packed-refs` rewrite / object pruning on one machine | unconfirmed, unchanged | none | none |
| 21 contract promised `core.untrackedCache` and `index.version=4` that the code did not set | confirmed at audit time, fixed meanwhile by the latency batch (verified in the live `normaliseGitConfigV1`) | none from me | the live config test |
| git interplay: ref event whose objects never arrive | confirmed (one `blocked` note, then silent) | `linearizeRefV1` returns `retry` with `waitedMs` (per local/incoming pair, cleared on success); the replica hook adds one more `blocked` event `objects-missing-timeout` with the waited seconds after 5 minutes; back-off stays 250 ms doubling to 30 s; the queue count is attested | unit: "a ref whose objects have not arrived reports a growing wait…"; the 5-minute event itself is not tested end to end |
| git interplay: `index.lock` held | confirmed (flat 1 s retry, `blocked` event every retry) | back-off 1 s to 30 s, `waiting`/`index-busy` once, `degraded`/`index-lock-stale` after 10 minutes with the age in `seconds`, completes when the lock is gone | unit: "a held index.lock backs off…" |
| git interplay: detached HEAD, new branches and tags | no defect found: new branches, tags and a detached `HEAD` replicate as plain last writer wins, concurrent detached commits linearise through `update-ref --no-deref HEAD`, `git fsck` stays clean | none | replica: "new branches, tags and a detached HEAD…" (passes on old and new code: a regression guard, not a fix) |
| git interplay: unmerged entries, index extensions | unmerged confirmed (above). Extensions: no defect; temporary indexes now run with `core.splitIndex=false` and `index.sparse=false` so no `sharedindex.*` is left behind | as above | unit: `index.version=4`, `core.untrackedCache`, `core.splitIndex`, all three, and a sparse (cone, `--sparse-index`) repository through publish, apply, verify |

Also changed: `missingObjectsV1` uses `cat-file --batch-check=%(objectname)` (does not open the objects): 30 000 loose ids 3.8 s to 0.22 s in isolation; it was 5 of the 8 to 10 s of a 30 000-entry apply.

## Measurements (30 000 registers, macOS, load average 25 to 35 from other agents; `🌿️git/🧪️tests/measure.ts` was a scratch script, deleted)

- Journal: 5 527 980 bytes in the store for 30 000 entry registers (about 184 B each, 7 segments); appended in 6 batches of 5000 in 0.2 to 0.3 s.
- Publish (diff-tree against the held fold, 6 batches in memory): 0.7 s on the quieter run, 1.4 to 2.0 s under load.
- Apply of 30 000 pending registers, one `update-index --index-info` batch: 1.8 to 2.1 s with unique blobs (7.6 to 10.3 s before the `cat-file` change); `update-index` itself 76 ms, `write-tree` 0.43 to 0.49 s, existence check 0.7 to 1.0 s.
- Fold of 30 000 registers (`verify`, and the one-time derivation of the known tree at daemon start): 0.2 to 0.4 s; first publish after a cold start including it: 0.4 to 2.5 s.
- Memory: the registers stay in the manifest and path book like any 30 000 files; a path's register is overwritten by later events, so the realm is bounded by the number of tracked paths.

## Tests (macOS, live tree after landing, 0 fail)

| Suite | Pass |
|---|---|
| `📒️journal` | 127 |
| `🚧️boundary` | 39 |
| `🌿️git` (`🟦️.ts` + both Python oracles) | 36 (was 10) |
| `🩺️attest` | 59 |
| `🔁️replica/🟦️.ts` | 73 |
| `🔁️replica/🏁️race.ts` | 14 |
| `🔁️replica/🌿️git.ts` | 19 (14 live before: 1 replaced, 5 new) |
| `🔁️replica/🌪️chaos.ts` | 8 |
| `🔁️replica/🥾️bootstrap` | 12 |
| `🗺️topology/🧪️tests/🟦️.ts` | 56 |

Failing first: the new git unit tests against the old module (only the three new exports shimmed so that imports resolve): 20 of 35 failed. The new replica tests against the old module: 5 of 18 failed (large stage, lost record, rebase report, unmerged status, conflicting concurrent commits, which never settled within 107 s); the detached and new-branch test passed. Before landing, the same suites ran on a copy of the live tree plus my changes: identical counts (boundary needs the repository `.gitignore`, so it ran live only).

## Not verified

- Linux and Windows; no run against the live fleet; old daemons that have not restarted still publish a base for more than 5000 changed paths and ignore the new record.
- A real `.git` replacement (the fold-fails path is covered by the missing-object test, not by deleting `.git` under a running daemon), a real second process holding `index.lock`, a real `kill -9` between emit and bookkeeping (simulated by clearing the in-memory state and writing the marker).
- The `objects-missing-timeout` event after 5 minutes at replica level, and the `rebase` command output: the command only writes the request, the voided list is in the daemon's events and the `lost` file.
- Attestation has no new field; stage health reaches the fleet only through events, `status`, `indexTree` and the queue count.
- `git()` has no timeout (finding 8), `state.json` and `clock.json` fsync (finding 7, replica part) and the other non-git findings are untouched.
- A conflicting path takes the later committer time; clock skew between machines can pick the older content, and the dropped version stays only in the unreferenced original commit.
- Cost of the one-time fold at start on the 270 000-path live tree was measured only with 30 000 registers.
