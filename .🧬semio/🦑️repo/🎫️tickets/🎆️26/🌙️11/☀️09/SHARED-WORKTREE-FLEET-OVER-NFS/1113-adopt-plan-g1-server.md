# Adoption plan: `/Users/ueli/Documents/semio` into origin g1-server (2026-11-13, dry run)

Produced read-only by the module's own dry-run path (`planAdoptV1`, the code behind `join --adopt --dry-run` and `relocate --dry-run`) with the new content-based rule, run from a scratch copy of the module against the live store `/Users/Shared/semio-store` and the unchanged repository. Nothing was written to `/Users/ueli/Documents/semio`, the live worktree, the live state or the store (runtime about 50 s; script `🧰️scripts/1113-adopt2/dryrun.ts`). Raw plan: not kept (reproducible). Git facts were read with read-only git commands on both repositories.

## Counts per class

| class | paths | meaning |
|---|---|---|
| same | 270377 | identical, adopted as they are |
| materialise | 607 | missing in Documents, come from the fleet |
| stale | 1795 | older versions of fleet files (or fleet-deleted, 79 of them): fleet wins, nothing published, local bytes in `.cache/f/lost/adopt-<stamp>/stale/` |
| edit | 89 | local edit on a fleet version that did not change since the base: published as new edits |
| publish | 7 | only here (the staged new ticket files `1112-*.md`, `resolution-2026-11-12-memory.md`, `1112-rename-fieldprep.files`, ...): published as new work |
| conflict | 30 | changed here and in the fleet since the base: fleet's version stays, local one in `.cache/f/lost/adopt-<stamp>/conflict/` and the report |
| nested | 461 | inside 77 nested repositories (ticket scratch repositories), left alone |

Evidence read: 1944232 journal events of all eight origins, 103 commits of the fleet's history searched. Every `stale` decision here is `journal` evidence. Git relation: `equal` (HEAD `5ececaa78b92` on both sides, branch `🐙ueli/⛳wip`); 127 staged paths.

Cross-checks:
- The local work is exactly the staged set: `git diff --cached` has 127 paths (7 added, 120 modified); 119 of them are `edit` or `conflict`, the 7 added are `publish`, and the last one is `stale` (the fleet already holds that content). No `edit`/`conflict` path is unstaged.
- No `stale` file has a local modification time later than the fleet's last event of the path: with the old "newer side wins" rule 0 of the 1795 would have overwritten the fleet in this case, but the old rule would have decided the 89 edits by time (all 89 local files are older than the fleet's last event of the path, e.g. `1028-coordination.md` 2026-11-12 22:22 against 2026-11-13 06:41) and kept the fleet's identical-to-HEAD content, dropping the owner's staged edits. The new rule publishes them because the fleet's content equals the base. Worth a glance by the owner (list: `git diff --cached --name-only` minus the conflicts below).
- Nothing in `stale` is an edit: the staged edits are all in `edit`/`conflict`.

## Conflicts (30; the stop threshold is 50, so the dry run does not stop; 26 are generated `Cargo.lock` files)

Reasons are all `both-changed`. The fleet's version stays; the Documents version goes to `.cache/f/lost/adopt-<stamp>/conflict/<path>`.

- `Cargo.lock` (local 4e7889bd57, fleet e2dd3f7203)
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/Cargo.lock` (local 590ce00b1c, fleet 86d2b5bba4)
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/Cargo.lock` (local bb36c3905e, fleet a716cb4e71)
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/Cargo.lock` (local a35f0d815b, fleet 96d718d470)
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/Cargo.lock` (local bce9e764db, fleet d6f65e9485)
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/Cargo.lock` (local 6c5c376cc9, fleet 82ff5e75e5)
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/Cargo.lock` (local e6ab732ae7, fleet 36672801a4)
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/Cargo.lock` (local 7ddf3f3c84, fleet 5b9d3b5e8b)
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/Cargo.lock` (local 1162d77d1f, fleet d5e315bbc1)
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/Cargo.lock` (local 1dbb79c8f5, fleet fea0a0bbce)
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/Cargo.lock` (local a823256c41, fleet ccd8d5a2d2)
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/Cargo.lock` (local e8945e1bda, fleet acfd64e96e)
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/Cargo.lock` (local 0a245d352e, fleet 529e359d42)
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/Cargo.lock` (local 6d312da7e7, fleet 69e64b3a76)
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/Cargo.lock` (local 9c63ed5124, fleet 5482182eb5)
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/Cargo.lock` (local 2a7ea3bb78, fleet 42f8f0c4cc)
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/Cargo.lock` (local e2ef5db21c, fleet 2a09267c34)
- `✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/Cargo.lock` (local cc17d0d00c, fleet ba8a1a3afa)
- `✏️s/🔌️plugins/📕️norm/Cargo.lock` (local ab7e988d65, fleet 4050fc4680)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/Cargo.lock` (local 552fa9a66b, fleet 57b8403d93)
- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/Cargo.lock` (local 180315a6e5, fleet 4d3d95d60c)
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/Cargo.lock` (local 0eec967829, fleet 6abf3baab2)
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/Cargo.lock` (local 02c128ef3b, fleet b35f118fe6)
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/Cargo.lock` (local d74ee48f19, fleet b356f48f1d)
- `🌎️hub/Cargo.lock` (local 0997852bdf, fleet 00aa476120)
- `📋️build-owners.json` (local 923f27bf9d, fleet 891cb537df)
- `🧰️framework/Cargo.lock` (local 11320163b4, fleet d144325cf4)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🌡️pressure/🟨️.mjs` (local 8ec8091d35, fleet 98ae1db0b4)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🛑️cancel/🟦️.ts` (local 876c1ab1b6, fleet e07d0b7967)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.mjs` (local 1cacb7a104, fleet eaf32e90fc)

Suggested handling after the real run: regenerate the 26 lockfiles with cargo (the fleet's versions are the current ones), and compare the four non-lock files (`📋️build-owners.json`, the two `🟨️.mjs`, one `🟦️.ts`) with their kept copies by hand.

## Commits, branches, stashes unique to Documents

- Refs: Documents 180, fleet worktree 160. Only in Documents: 9 branch heads (`2`, `b1`, `b3`, `b4`, `backup`, `kinan/bericht-diff`, `worktree-agent-afa905cc164f52ed8`, `🐳kinan/⛳wip`, `🦢niloufar/⛳wip`), 2 tags (`demo`, `premigration`), 3 `refs/agents/*`, 4 `refs/codex/turn-diffs/*`, 2 gitkraken refs. Only in the fleet: 1 codex ref. `refs/stash` differs.
- Stashes: 18 in Documents (reflog), 3 in the fleet. A stash list lives in the reflog, which is private; a differing `refs/stash` is decided for the fleet. The new adopt lifts every local stash commit (up to 1000) into loose `refs/adopt/<stamp>/stash-N` refs, lifts local-only refs (not remote-tracking) into loose refs, and keeps a differing non-branch ref under `refs/adopt/<stamp>/...`; the differing `packed-refs`/`HEAD`/non-branch ref files go to `.cache/f/lost/adopt-<stamp>/git/`. Still do the manual lift below first if the owner wants the names kept exactly.
- Commits: 4369 commits reachable from Documents' refs, 52 of them are not in the fleet worktree's object store (stash commits and the branch/agent/codex refs above). The object union adds them.
- Linked worktrees (`semio-kinan-wip`, `semio-niloufar-wip`, `semio.worktrees/2`) keep their metadata in `.git/worktrees`, which is private and stays; the three worktree directories are untouched.
- Pre-step for the owner (writes only refs, loses nothing): `git -C ~/Documents/semio update-ref refs/old-repo/1/HEAD HEAD` and, per stash, `refs/old-repo/1/stash-<i>` from `git stash list --format=%H`. Not run here (Documents is read-only for this task).

## Local-only ignored data

Not read by adopt, never published, stays where it is: `.cache` 250 GB (exact figure of the 2026-11-13 09:00 audit, `1113-audit-existing-repo-vs-fleet.md`; the fleet worktree has 429 MB), `.venv` 567 MB, `.nx` 38 MB, `.tmp-diskproof` 144 KB, `.pytest_cache` 36 KB. Git pack data: 6 packs, 4.95 GiB (the object union only copies loose files that are missing).

## What the real run would do

1. `relocate` (this Mac is joined at `/Users/Shared/semio`): identity and cursors move, then adopt in Documents. The 1795 stale files are replaced by the fleet's versions (bytes kept), the 607 missing files materialise, 89 edits and 7 new files are published, the 30 conflicts keep the fleet's version with the Documents copies in `lost/`.
2. Stop conditions: conflicts > 50 (now 30), new work > 1000 (now 96), relation `unrelated` (now `equal`), `fleet` not ok. None applies; the stage is replaced by the fleet's (old index kept in `lost/`).
3. Run the plan again right before the real run: the fleet moves and the counts drift.
