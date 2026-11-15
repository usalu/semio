# Chaos defects D1 to D4 fixed, structural conflict rule, stale-republish guard (2026-11-13)

Landed under the lock in two steps (`1113-landings.md`, entries 06:27Z and 07:08Z, three-way merge onto the live files, every test file of the module green against the live tree before release). Repo MCP did not connect; the ticket is not closed through `ticket_close`. No daemon was stopped, nothing was written to `semio-store`, `.git` or `.cache/f`.

## The rules as implemented

New member `🔁️replica/⚖️clash` (pure rule: `clashKeyV1`, `clashV1`, `retiredV1`; schema, fixture, Python oracle) and the apply path in `🔁️replica/🟦️.ts` (`settleClashes`, `retire`, `ancestryOf`, parked queue rows, `onlyDirectories`, stale check in `ingest`/`materialise`).

1. **Clash.** Two live manifest entries (`put`, `link`) clash when `foldNameV1(path)` is equal (case collision, rule 3 / D1) or the folded path of one is a directory prefix of the other (a file or link with entries below it, rule 2 / D2). The fold is the boundary's (the cross-OS table was live at landing). Sibling directories that differ only in case do not clash.
2. **Winner.** Entries newest hlc first (ties by path, the hlc already ends in the origin); an entry is kept unless it clashes with an entry kept before, otherwise it is retired. This is the "entry set with the newest hlc wins" reading: a file newer than every live descendant retires the descendants; a file older than some descendant is retired itself and the older descendants stay. `retiredV1` is the specification (fixture of 13 vectors, replayed by TS and by `⚖️clash/🧪️tests/🐍️.py`, which shares no code); the replica evaluates the same recursion lazily (`winnerOf`: beaten only by a newer rival that is itself kept).
3. **Where it runs.** `applyQueue` calls `settleClashes` before the pool. Rivals of a queued entry come from: the queued and parked entries indexed by fold key, the local registers (`folds.find` for equal keys and for ancestors), the ancestors in the manifest, a fold-keyed directory hint (`foldedDirectories`, kept by `setFile`/`forget`) with an exact scan when positive. A queued loser is retired without being materialised; a queued winner first retires the older rivals that are materialised here (so a case-insensitive volume never sees both and the later write never meets them); a parked entry whose winner is gone is queued again.
4. **Retire.** The local copy goes to `.cache/f/lost/<winner hlc>/<path>` (`overwritten`), the register becomes a delete carrying the hlc of the retired entry (not queued, not published, not counted), a `superseded` event names loser, winner and reason (`case-collision`, `directory-replaces-file`, `file-replaces-directory`).
5. **Who publishes what: nobody.** The manifest keeps both live entries on every machine; the rule is a pure function of it, so every machine derives the same effective set from the same manifest and no machine waits. Reasons against an explicit delete from one machine (author of the loser, lowest origin): it needs an election and a handshake (the elected machine may be offline: the queue would not drain), a delete minted after a concurrent legitimate re-creation of the loser path (with a newer hlc than the re-creation's peer view) would wrongly kill it, and the retired entry could not come back when its winner is deleted (the case-rename scenario of seed 540: edit wins, later the edit is deleted, the renamed file must return). Snapshots and compaction store the folded manifest unchanged; they never see the rule.
6. **Persistence.** The replica persists manifest rows only through the queue (the manifest of a non-queued path is mirrored from the registers on load). A retired entry therefore stays in the persisted queue as a **parked** row (queue flag 2: not pending, not counted in `queue.size`, written by checkpoint and delta log through `heldSnapshot`/`heldId`); on load the set is rebuilt from the registers (delete with the hlc of a live manifest entry) and the rows are parked again. Without this a restart turned the retired entry into a delete in the manifest and it never came back (seed 540, found by the sweep). `verify` skips such entries.
7. **D3 symlink ancestors.** `ancestryOf` inspects every ancestor with `lstat` (verified directories cached per pass, cleared on prune). Apply: a symlink ancestor blocks a put/link with a visible `blocked` (`symlink-in-the-way`, `parent-is-a-file`) and never follows the link; a delete through a link does nothing. A link the fleet has is an ordinary entry of the clash rule (so `ln` versus `ln/x.txt` is decided by hlc like `dir` versus `dir/c.txt`). A cycle only produces the blocked event (no `ELOOP`). Publish: `observe` and `relist` treat a path reached through a symlinked directory as not a file of the tree (never published under the link path, a known path below a new link is published as deleted).
8. **D4.** `applyQueue` runs the deletes (with their prune of emptied directories) before the puts and links of the batch, and the link branch retries `mkdir`/`rm`/`symlink` on `ENOENT`/`EEXIST`/`EINVAL` like the file branch. A rename onto a directory (`EISDIR`/`ENOTEMPTY`) is `blocked: directory-in-the-way`; a tree of only empty directories (never replicated) is removed so it cannot block a file at its path; a remote delete of one spelling no longer touches the file of another spelling on a case-insensitive volume, and a lost-copy or unlink on a path that vanished meanwhile is not an error.

## Stale republish (incident 06:14)

- What `base` carries: `base` of a `put`/`link` is the sha256 of the **sender's own register** of that path (`null` when it had none), of a delete the same; it is not the manifest winner. g1-m04 republished its tree from empty state, so its stale events carried `base: null`.
- Is "base must equal the current winner's sha, else concurrent, resolved by hlc only if its content is new" sound as a general rule? The first half is sound as a *classification* (a base that is not the winner marks a concurrent edit), but base alone cannot separate a legitimate concurrent edit (base = the common ancestor, also not the winner) from a stale republish; the discriminator is the second half, "content seen earlier at this path". It is only sound as a **local, non-manifest** decision: dropping the event from the manifest on machines that remember and keeping it on machines that do not (restart, late join) would make manifests diverge for good.
- Implemented therefore as re-assertion, which converges: `ingest` compares each winning `put` with the entry it replaces; when `base` differs from the replaced content and the content equals one of the last 8 superseded hashes of that path (in memory, last 50 000 changed paths; own publishes are remembered too; `.git/` is exempt because refs have their own linearisation), the path is marked. `materialise` then does not replace an unmodified local file whose register holds the replaced content: it emits `stale-republish` and `keepNewer`, so the existing flow publishes the newer content again with a fresh hlc, and the whole fleet converges on it (every machine that recognises the stale event re-publishes identical content; the highest hlc of identical content wins). A machine without the history applies the stale content and then the re-publication.
- Limits: memory is lost on restart; a deliberate revert to older content by a machine that has not seen the newest version is treated as stale and re-asserted against (its bytes are in the winner's history and in the store); the guard cannot help a fleet where **all** machines forgot.
- Test with the incident sequence: `⚖️clash/🧪️tests/🟦️.ts`, "the 06:14 incident" (v1 synced, v2 synced, a machine with a cleared register republishes v1 with `base` null and a fresh hlc; v2 stays everywhere, `stale-republish` emitted; with the history switched off the test fails).

## Tests

- `⚖️clash/🧪️tests/🟦️.ts` (17): schema validity, 13 fixture vectors against TS and the Python oracle, order independence, D1 both orders, D2 both orders (also across directory case), D3 (known link, local-only link with `blocked` and an untouched outside directory, cycle, publish through a link), D4 (40 deletes + 40 links), resurrection after a restart, empty-directory tree, stale republish. All run three to five real replicas with real git through the chaos harness; the invariants of `checkFleetV1` (tree, head, stage, status, view, fsck, oracle `🌪️chaos.py`, verdict) are asserted after each.
- `🔁️replica/🧪️tests/🌪️chaos.ts`: the four `test.failing` reproductions are normal tests D1 to D4.
- Whole module against the live tree after each landing: all 25 test files green (replica 85, resilience 23+16, boundary 69, attest 73, chaos 9, clash 17, ...).

## Sweep statistics

| run | profile | seeds | before | after |
|---|---|---|---|---|
| 3 x 30 | hostile, `--loose` | 500..599 | 92/100 (7 D1/D2 wedges, 1 error event) | 100/100 |
| 5 x 120 | hostile | 2001..2020 (before) | 19/20 (D4) | |
| 5 x 120 | hostile | 5001..5100 (intermediate) | | 99/100 (5074: content of an ignored path) |
| 5 x 120 | hostile | 7001..7100 (landed v1) | | 100/100 |
| 5 x 120 | hostile, `--loose` | 4001..4100 (first version) | | 86/100 (copyfile `ENOENT` of two spellings of one file, `EISDIR`) |
| 5 x 120 | hostile, `--loose` | 4001..4100 (after those fixes) | | 97/100 |
| 5 x 120 | hostile, `--loose` | 6001..6100 (landed v1) | | 97/100 (6018, 6094 directory spelled in another case, fixed by the second landing; 6038 duplicate commit) |
| 5 x 120 | hostile, `--loose` | 8001..8100 (final code) | | 99/100 (8037: edit of a path that a machine ignores) |
| soak, 4/5 machines x 120 | mixed | fresh seeds | | 37 runs 1 failure (empty-directory tree blocked a file; fixed), 49 runs 0 failures (landed v1), 39 runs 0 failures (final code) |

Load average on the host was 10 to 40 during all runs; settle budget 20 to 30 s.

## Found by the sweeps in this area and fixed

Retired entry lost after a restart (parked rows), `ENOENT` copy/unlink when two spellings of one file are deleted in one batch, `EISDIR` rename onto a directory, a tree of empty directories blocking a file, directory spelled in another case below a file (fold-keyed hint).

## Outside this area, not fixed

- **Duplicate commit (`history: N commits, expected N-1`)** under concurrent commits with outages and restarts: seeds 4093 (older code), 6038. Reproduced on the **unmodified base** module as well (3 of 6 replays of the trace of seed 6038), so it is not caused by this change: it is in the linearisation of concurrent commits (`linearizeRefV1` / `linearise`), `.cache/f/lost` or the git agent's area.
- **Edit of a path a machine ignores** (seeds 4040, 4047, 5074, 8037: `.gitignore` of one machine names a path that is edited later): diverges by design (ignored paths are forgotten); the harness generator only partly avoids it.

## Not verified

- Different-case directory spellings are detected through the fold-keyed hint maintained by `setFile`/`forget`; code paths that write the registers directly (adopt, join, takeover) do not update it, so a conflict created right after such a bulk step is found only after the next `setFile`-based change of that directory (the hint is rebuilt on every process start).
- The rule runs on macOS (case-insensitive APFS) only; Linux and Windows machines were not in the sweeps. On a case-sensitive volume the retirement is by rule (both spellings could exist), the unit tests of the pure rule and the Python oracle cover the logic, the apply path on ext4/NTFS is untested.
- Retired entries are not copied to `lost/` on machines that never held their bytes (the blob stays in the store while the manifest references it).
- 272k-file catch-up cost of `settleClashes` (index over the queue, about one fold per queued path) was not measured; steady state is O(queue + parked).
- The `fleet` CLI tests of attest timed out once under a load average of 12 (unrelated; green alone and on rerun).
- `ticket_close` was not run (repo MCP not connected).
