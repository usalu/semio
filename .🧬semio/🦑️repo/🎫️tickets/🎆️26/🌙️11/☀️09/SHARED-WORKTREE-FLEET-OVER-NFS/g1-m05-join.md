# g1-m05 (Windows) Join And Merge Report

State as of 2026-11-15 12:30 CET. Machine: the owner's Windows workstation, origin `g1-m05`, group `g1`, worktree `C:\semio`, store `S:\` (NFS host 192.168.178.70). Log folder (kept): `C:\semio-merge-20261113-2107`.

## Result In Short

| item | state |
|---|---|
| Joined, daemon running from the worktree, synchronisation on | yes |
| Provision exit code | 3 on every real run (fleet ready, native build pending) |
| `.cache\f\native-bootstrap.done` | **not written** |
| `verify` | **exit 1** (see Step 5) |
| Fleet line of this machine | **`lagging (not settled: no common checkpoint within 60 s yet, still moving)`**, never `ok` so far |
| `C:\semio` the only semio repository | **no**: `C:\git\semio` and `C:\semio-stale-g1-m02` still exist, nothing was deleted |

## Step 1: Inventory

`inventory.md` in the log folder (2026-11-13 21:12).

| path | HEAD | branch | status lines | stashes | git dir |
|---|---|---|---|---|---|
| `C:\git\semio` (OLD-2) | 5ececaa78b9 | 🐙ueli/⛳wip | 512 | 25 | directory |
| `C:\semio` | 5ececaa78b9 | 🐙ueli/⛳wip | 1812 | 3 | directory |
| `C:\semio-stale-g1-m02` (OLD-1) | 5ececaa78b9 | 🐙ueli/⛳wip | 2978 | 3 | directory |
| `E:\git\semio.worktrees\🐙ueli\🌿1` | ad12ff6b565 | 🐙ueli/🌿1 | 0 | 25 | linked worktree of `C:\git\semio` |

## Step 2: Merge (case 2c for both)

| | OLD-1 `C:\semio-stale-g1-m02` | OLD-2 `C:\git\semio` |
|---|---|---|
| identical | 2704 | - |
| copied | - | 410 |
| OLD newer (copied, C:\semio version kept under lost old) | - | 101 |
| OLD older (kept under lost old) | 13 | 1 |
| deleted in OLD (listed only) | 261 | 0 |
| refs created (`refs/old-repo/*`, `refs/heads/old-repo/*`) | 6 | 39 |
| verify (`OLD-N-verify.md`) | PASS | PASS |

- 2c-4 (`robocopy /MOV`) was not run for OLD-2.
- SHA-256 check of the "copied" and "OLD newer" paths (step 5): 510 equal, 1 different: `🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/❓️quiz/🔣️.json` (OLD-2, "OLD newer"); the fleet changed it after the merge (`step5-hash-mismatch.tsv`).

## Step 3: Plan Run

Exit 0. Summary: features and mount unchanged, `S:` already mounted, launcher present, "the worktree holds no repository to adopt". No adopt report exists (`C:\semio\.cache\f\lost\adopt-*.md` is absent).

## Step 4: Real Runs

No `-Server`, `-FirstMachine`, `-Force`, no `seed`. The token is set in user scope and was never printed.

| run | exit | stopped at | fix |
|---|---|---|---|
| before the merge task | 1, 1, 3 | mount options, NFS host, `.Count` in native bootstrap | topology `windowsMountFlags`, provision `🔵️.ps1` (NfsHost override, one `-o` per option), native bootstrap `.Count` |
| 1 (11-14 18:39) | 3 | native bootstrap | see log |
| 2 (11-15 00:47) | 3 | dependency sync: "Current exact Windows relative path descriptor changed" | usage record moved into its own folder (`.cache\b\d\usage\usage.json`) so that rewriting it no longer changes a fenced ancestor directory |
| 3 (11-15 02:31) | 3 | `repo run workspace:setup`: graph service timeout; `LNK1181: cannot open input file 'kernel32.lib'` | the MSVC tree was extracted inside a case-sensitive folder (`kernel32.Lib` on disk); code: the compiler scratch folder is created case-insensitive; this machine: 122 folders switched back (elevated, `case-fold.txt`) |
| 4 (11-15 04:06) | 3 | `repo run workspace:setup`: "the slice worker ended 4 times in a row with exit code 75 because the code on this machine kept changing under it" | none possible here: other machines were editing the build library |

Unelevated tries of the failing command after run 4 (`repo run workspace:setup`, logs `workspace-setup-try*.txt`, `workspace-setup-retries.md`):

| try | result |
|---|---|
| 1 | cut by my own 25 min limit while building |
| 2 | `closure.provider-entry-stale` (reference ec0ee1a8), a closure layer was lost, probably by the cut of try 1 |
| 3 | cut by my 2 h limit at 4352 of 4774 projects; 0 projects came from the fleet; the graph service stops inferring when the client leaves |
| 4 | `graph: @semio-tech/demonstrator-playground needs 2778 MB but one project is granted 2048 MB here and 5207 MB are free after 111 s: refused with exit 75` (reference 8d74dac7, `BUILD_GRAPH_MEMORY_REFUSED`) |
| loop | retries every 5 minutes, see `workspace-setup-retries.md` |

The graph memory refusal is the open item of ticket `☀️14/WORKERS-READY-WITH-LOW-RESOURCES` (no worker has the marker yet according to its census). This machine has no `workers/g1-m05` folder there.

## Step 5: Checks

- `status` (`status.txt`): daemon running, source worktree, nothing queued.
- `verify` (`verify.txt`): exit 1. 11-15 05:15: 143 paths; 11-15 11:47: 10 paths, of which 8 were files other machines were writing at that moment and 2 were `.git/objects/info/commit-graphs/*`. The two git paths are fixed in code (below). 11-15 12:45, after the fix: 26 paths, no git path among them, all in the two ticket folders other machines were writing during the run (`PERFECT-HANDCRAFTED-3D-MODELS…`, `WORKERS-READY-WITH-LOW-RESOURCES/workers/*`) plus one `native-green.md`; the run took 9 min under build load. A run with exit 0 needs a quiet fleet and is still missing.
- `fleet --wait 600` (`fleet.txt`, 11-15 12:10): exit 1, "The fleet is not in agreement (10 origins…)". This machine: `g1-m05 windows 0s 3/0 cp:0s on b93aabf29dd1 952/12/3983 -/-|130451/344846 lagging (not settled: no common checkpoint within 60 s yet, still moving)`. Other machines at that moment: m02, m03, m04, server `diverged`; m07, m10 `stale`; m08 `ok`.
- Apply latency of this machine is high (p50 130 s, p95 345 s in that run; 9 s when no build was running).
- `preflight` (`preflight.txt`, 11-14): warning `native-build`.

## Step 6: Old Repositories

Nothing was removed. Both old repositories still exist.

- `C:\git\semio`: in use. A `repo tools exec … 🎓teaching/🏛architecture/⚡energy/…authoring` process started there on 11-15 08:31, the desktop session works in it, and it owns the linked worktree on `E:`. It has changed since the merge of 11-13, so the comparison of step 2 must be repeated before any removal.
- `C:\semio-stale-g1-m02`: merge verified (PASS); the `robocopy /L` check and the removal wait for the owner's confirmation.

## Code Changes Made On This Machine (replicated)

| area | file | change | test |
|---|---|---|---|
| native directory usage | `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/📏️usage/🟦️.ts`, schema | record in `usage/usage.json` beside the volume root | meter test, 8 pass |
| Windows host port | `…/📁️directory/🪟️host/🟦️.ts` | `setCaseSensitive(path,sensitive=true)` can clear the flag | 10 pass, 1 skip |
| Windows compiler | `…/🛠️native-compiler/🪟️windows/🟦️.ts` | scratch folder made case-insensitive before extraction | import check only, no fresh provisioning of the compiler |
| journal, attest | `🤝️shared-worktree/📒️journal/🟦️.ts`, `🩺️attest/🟦️.ts` | `readThroughReplacementV1`: a store file replaced under the reader (`EBADF`/`ESTALE` on the Windows NFS client) is read again | attest test passes |
| replica | `🔁️replica/🟦️.ts` `keepLost` | a read-only copy under `lost` is made writable before it is written again; the kept copy stays writable | 2 pass |
| replica | `🔁️replica/🟦️.ts` `verifyV1` | paths the pack gate keeps on a machine (`.git/objects/info/*`, local packs) are not drift, as for the running replica | 2 pass |
| earlier | topology `windowsMountFlags`, provision `🔵️.ps1`, native bootstrap | see step 4 | topology tests need a Mac, not run |

## Findings Still Open

1. Native build: graph inference on this machine is slow (over 2 h cold, nothing taken from the fleet) and ends with the per-project memory refusal. Owner ticket: `WORKERS-READY-WITH-LOW-RESOURCES`.
2. The daemon was ended nine times by its own watchdog on 11-15 ("the event loop was blocked for 120 s in drift").
3. The daemon log holds many `EBADF` round errors from store reads that the new retry does not cover.
4. On Windows a read-only local file blocks an incoming put or delete (`EPERM`); the replica does not clear the attribute. Not fixed.
5. The `S:` mount is the one made before the mount option fix and is invisible to `mount.exe` in the unelevated session; a clean remount needs an elevated shell.
6. NFS anonymous uid/gid is not set by provisioning; files written by this machine may be read-only for others (seen on 11-14).
7. `g1-m05` held fleet graph claims for g1-m10 and g1-m02 (census of the other ticket) without delivering, because its inference is that slow.
8. The fleet as a whole was not in agreement at any check; that is outside this machine.
9. Two hung `repo status` processes from 11-14 01:29 (pids 22284, 15488) were left alone (no taskkill by name).
10. The repo MCP was not available in the session; no ticket was opened or closed through it.
11. Not run: the wider storage test suite, topology tests (Mac only), a fresh provisioning of the Windows compiler.
