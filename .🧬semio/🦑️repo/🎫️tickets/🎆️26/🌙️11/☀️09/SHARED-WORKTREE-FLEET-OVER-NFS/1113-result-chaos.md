# Chaos / convergence harness for the fleet replica (2026-11-13)

Only test files were added; no module source (`🟦️.ts` outside `🧪️tests`) was edited, no daemon touched, nothing written to `semio-store`, `.git` or `.cache/f`. Repo MCP did not connect, ticket bookkeeping is manual.

## Files (all in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/🧪️tests/`)

- `🌪️harness.ts` — seeded PRNG (mulberry32), serialisable operations, fleet builder, operation generator, `performV1`, `settleV1`, `checkFleetV1`, `replayV1`, `runChaosV1`, ddmin shrinker `shrinkV1`.
- `🌪️chaos.ts` — the tests (quick, soak, negative controls, fixed trace, open-defect tests).
- `🌪️chaos.py` — independent oracle: git CLI + Python stdlib only, shares no code with the module.
- `🌪️sweep.ts` — seed-sweep runner (one child process per seed, parallel, optional shrinking). Wrapper: `../🧰️scripts/1113-chaos-sweep.sh` in this ticket.

## How it works

- N replicas (3 to 5), each with a real git repository in a temp dir outside the worktree, all on one temp store. Every machine reaches the store through its own symlink, so a store outage for one machine is real: the link is removed, `storeReachableV1` fails and the driven round is skipped exactly as `runV1` does (there is no injection seam in `ReplicaOptionsV1` for reachability, the symlink is the seam).
- Rounds are driven (`publishOnceV1` then `followOnceV1`), thrown errors become `error` events as in `runV1`.
- Operations (concrete, JSON-serialisable; a trace replays without the generator): create, modify in place, atomic replace (tmp + rename), delete, rename, case-only rename, nested tree, chmod, relative symlink, `git add`, `git rm --cached`, `git reset -q --`, `git commit` (unique message, sometimes `--allow-empty`), `git status`, `.gitignore` add/remove, clean restart, crash restart (drop the instance, `release()`, `forgetGitV1`, reopen from persisted state), `repairV1`, `requestRebaseV1`, store outage/reconnect, partial rounds on a random subset (others lag), plus rmdir, file/directory swap, file/symlink swap, torn write (half file, one publish round, rest of the file).
- Generator rules that keep results free of by-design divergence (each was a false positive found by the first sweeps): never mutate a path any machine ignores (private paths diverge by design), never touch `.gitignore` through generic ops, never write through or rename a symlink (a rename can make a relative target escape the tree and be refused), `.gitignore` edits, case renames and file/directory swaps run between two full settles (sync points), a retyped path is frozen afterwards.
- After a burst: `settleV1` reconnects everyone and runs rounds until store heads, cursors and manifests are unchanged for 3 rounds with empty queues (45 s default budget, else violation `settle`).

## Invariants checked (`checkFleetV1`, each violation is `category: detail`)

- `tree`: identical shared trees per `scanSharedV1` (path, kind, size, executable bit, sha256, symlink target; this includes the allow-listed `.git` files and objects).
- `head`, `write-tree`, `stage` (`ls-files -s -z`), `status` (`status --porcelain=v2 -z --untracked-files=all`, includes modes and oids), `view` (`gitViewV1` digest) identical on every machine.
- `history`: no merge, exactly `1 + successful commits` commits, every commit message exactly once.
- `fsck` (`--no-dangling --no-reflogs`) clean, no `index.lock` left, `queue` empty, no `error` event (sink events plus thrown errors), `verify` (`verifyV1`) ok.
- `oracle`: `🌪️chaos.py` re-checks head, tree, stage, status, linear history, messages, fsck and working files (path, exec bit, link target, sha256) across all worktrees.
- `verdict`: every machine's attester is ticked into a real `attest.json` (code, cursors, manifest, git state, view) and `fleetReportV1` must be `ok` with no differing field.
- `restart`: after the checks every replica is closed and reopened, the fleet settles again and the store heads and manifests must be unchanged, then the invariants run again (`after-restart …`).
- Negative controls in `🌪️chaos.ts` prove the check fires (unpublished edit, unpublished commit, stray stage entry, untracked file are reported by tree/head/write-tree/stage/status/history/verify/oracle/verdict).

## Usage

Run a test file (existing convention):

    cd <tests dir> && env -u SEMIO_FLEET_GROUP -u SEMIO_FLEET_NFS_HOST -u SEMIO_FLEET_NETWORK -u SEMIO_REMOTE_CACHE_ENDPOINT -u SEMIO_REMOTE_CACHE_ACCESS -u SEMIO_REMOTE_CACHE_INSTANCE <bun> test ./🌪️chaos.ts --timeout 120000

- Quick (default, in the normal suite, about 75 s with the defect tests): 12 fixed seeds `[1..6,8..13]`, 3 machines, 25 steps. Whole file: 9 pass, 1 skip (soak).
- Replay one seed: `SEMIO_CHAOS_SEED=<n> SEMIO_CHAOS_MACHINES=<m> SEMIO_CHAOS_STEPS=<s> [SEMIO_CHAOS_PROFILE=hostile] [SEMIO_CHAOS_LOOSE=1]`. The seed, replay line, violations and the numbered trace are printed on failure. Seeds belong to a harness version (the generator draws changed while the generator was refined); the serialised trace is the authoritative reproduction.
- Soak (opt-in): `SEMIO_CHAOS_SOAK=<seconds>` runs fresh seeds on 4 or 5 machines, 120 steps (`SEMIO_CHAOS_STEPS`), until the budget is spent or 3 failures; prints the base seed (`SEMIO_CHAOS_BASE` to repeat) and writes failing reports to `$TMPDIR/chaos-soak-<seed>.json`.
- Seed sweep: `🧰️scripts/1113-chaos-sweep.sh --from 1 --count 200 --parallel 4 --settle 20000` (env `BUN=<bun>`). Flags: `--machines N --steps N --profile mixed|hostile --loose --no-restart-check --settle ms --timeout s --shrink [--shrink-max N --budget N]`. `--one <seed>` prints one result line. Failures go to `🗑️generated/chaos/seed-<seed>.json` (ops, notes, violations, and after `--shrink` the ddmin-minimised `shrunk` trace) plus `summary-*.json`; delete that folder when done. The profile `hostile` weights crash, restart, outage, commit and type swaps up and rounds down; `--loose` drops the sync points around case renames and directory swaps (this triggers D1 and D2 on purpose).

## Sweep statistics (final harness)

| run | machines x steps | profile | seeds | pass | fail | wall (parallel) |
|---|---|---|---|---|---|---|
| A | 3 x 30 | mixed | 1..200 | 200 | 0 | 262 s (4) |
| B | 3 x 30 | hostile | 1001..1150 | 150 | 0 | 199 s (4) |
| C | 5 x 120 | hostile | 2001..2020 | 19 | 1 (D4, seed 2015) | 125 s (3) |
| D | 4 x 80 | mixed | 3001..3020 | 20 | 0 | 70 s (3) |
| soak test | 4/5 x 120 | mixed | 16 fresh seeds (base 542882647) | 16 | 0 | 160 s |
| loose | 3 x 30 | hostile, `--loose` | 500..599 | 92 | 8 (7 D1/D2 wedges, 1 error event in a D1 scenario) | 183 s (4) |

Before the final harness several hundred more seeds ran while the harness was being corrected (mixed 3x30 seeds 1..300, hostile 3x30 80 + 150, 5x120 mixed seeds 1000..1029 all passing after the false positives listed above were removed from the generator). Every run included the clean-restart check. Load average on the host was 20 to 40 during the sweeps, no run needed more than its settle budget except the wedged ones.

## Defects, ranked

All four are reproduced by explicit operation traces in `🌪️chaos.ts` as `test.failing` (the suite stays green; when a defect is fixed the test starts failing, remove `.failing` then). Line numbers are of `🔁️replica/🟦️.ts` at 05:10 on 2026-11-13 and move while other agents edit.

1. **D3 apply writes through a symlink parent.** Sequence: m0 `symlink ln -> dir` (a directory), m1 (not yet synced) creates `ln/x.txt`; settle. Observed: on m0 and m2 the put of `ln/x.txt` is applied through the link and lands at `dir/x.txt`, which is then published as a new untracked path; `verify` reports `missing-local ln/x.txt` on them; m1 keeps `ln/x.txt` and its queue holds the `ln` link forever (`lagging`). Expected: a put below a path that is a symlink locally must not follow it (refuse or queue, and resolve the file/link conflict). Seeds: 312 (3x30 hostile, older harness: variant with a symlink cycle, `ELOOP` lstat error event). Cause: `materialise` never checks that no ancestor of `target` is a symlink: `mkdir(dirname(target),{recursive:true})` and `rename(temporary,target)` at lines 1348 and 1360, and `mkdir` + `symlink` in the link branch at lines 1318-1320 all follow link ancestors; the `lstat(target)` at 1283 throws `ELOOP` (not `gone`) for a looping link ancestor. Test: `DEFECT D3`.
2. **D2 file versus directory on one path wedges the fleet.** Sequence: m0 replaces directory `dir` by a file `dir`, m1 edits `dir/c.txt` before either has seen the other (also: `touch build` vs `mkdir build/` created concurrently). Observed: both registers stay live (last writer per path), every machine queues the other entry forever (`blocked: parent-is-a-file` or `directory-in-the-way`), `queue` is not empty, `verify` shows `missing-local`, the verdict stays `lagging`, trees and `git status` differ. In the link branch the unguarded `mkdir` raises `EEXIST` as an `error` event (seed 43 older harness). Expected: a deterministic winner (for example the newer HLC across the path and its prefixes) and a lost copy for the loser, convergence. Seeds: 43 (older harness), 65, 501, 515, 593, 596 (`--loose` runs 500..599 and 3x30 hostile 1..80 earlier). Cause: `materialise` returns `"queued"` for `directory-in-the-way` (line 1301) and `parent-is-a-file` (line 1352) and nothing ever resolves two live registers whose paths are prefix-related; the register model is per path, so last-writer-wins never compares `dir` with `dir/c.txt`. Link branch lines 1316-1325 have no `ENOTDIR`/`EEXIST` handling. Test: `DEFECT D2`.
3. **D1 case-collision wedges the fleet.** Sequence: m0 renames `b.txt` to `B.txt` (publishes delete `b.txt`, put `B.txt`), m1 edits `b.txt` before the rename arrives. Observed: `b.txt` (m1 put wins over the delete) and `B.txt` are live together; on a case-insensitive volume each machine queues the other's entry forever (m0 holds `B.txt`, m1 and m2 hold `b.txt`), so trees differ, `queue` is not empty, verdict `lagging`. (Two machines creating `q.txt` and `Q.txt` concurrently converged in a probe; a rename racing an edit wedges.) In the scenario seed 540 (`--loose`) the same race also produced a transient `ENOENT unlink dir/C.txt` or `copyfile` error event on one machine (about 4 of 6 replays). Expected: the same deterministic winner and lost copy as for any other conflict, convergence. Seeds: 17, 7 (first harness version), 511, 522, 591 (`--loose`). Cause: `refusalOf` returns `"case-collision"` when `replica.folds.find(path)` finds a live other-case path (line 1248) and `materialise` maps it to `"queued"` (line 1280); the collision is only resolved if the other path is deleted later, which does not happen when its register is newer than the delete. Test: `DEFECT D1`.
4. **D4 symlink apply races the prune of its directory (error events).** Sequence: one publish deletes 40 files in `p/` and creates 40 symlinks in `p/`; applied by another machine. Observed in 5 of 6 replays (and seed 2015, 5x120 hostile): `ENOENT … symlink '../b.txt' -> …/p/lN` or `EEXIST … mkdir …/p` error events, 1 to 9 per machine; the fleet still converges afterwards (retry), so only the `error` invariant fails. Expected: no `error` event; the file branch of `materialise` retries `mkdir`/`rename` (lines 1347-1361) but the link branch does not. Cause: `applyQueue` applies entries in a `pool` (line 1368), a delete applied in parallel runs `prune` (line 1233, called at line 1296) and removes the directory between the link branch's `mkdir` (line 1318) and `symlink` (line 1320), which have no retry on `ENOENT`/`EINVAL`/`EEXIST`. Test: `DEFECT D4` (5 attempts, each about 83 percent likely to show an error).

## Observations that are not defects (by design, left out of the oracle)

- A path that becomes ignored is forgotten and diverges when edited or deleted afterwards (the generator never mutates ignored paths). An empty `.gitignore` or an ignore rule for `.gitignore` itself makes `.cache` show up as untracked.
- A symlink whose relative target leaves the worktree is refused (`unsafe-link`) and stays local; a rename can create one.
- Empty directories are not replicated; a machine that deleted the last file of a directory locally keeps the empty directory.
- `repair` while the store is down and `restart`/`crash` while down are skipped (a replica cannot be reopened during an outage in this harness; `openReplicaV1` with an unreachable store was not exercised).
- Not generated: unicode normalisation forms (NFD), Windows-only names, files above 100 KB, more than 5 machines, clock skew, concurrent edits during a publish (the `torn` operation covers a half-written file seen by one publish round).

## Notes on the code under test

- Everything else held across all runs: stage and commit replication, concurrent commits (linearisation kept every message exactly once, history linear, no merge), crash restarts (replayed tail), clean restarts (no store change), repair, rebase requests, outages with lag, in-place and atomic edits, executable bit, symlinks, the allow-listed `.git` objects, attestations and the fleet verdict.
- No file named `🌪️*` is picked up by the daemon restart rule (all under `🧪️tests`).
