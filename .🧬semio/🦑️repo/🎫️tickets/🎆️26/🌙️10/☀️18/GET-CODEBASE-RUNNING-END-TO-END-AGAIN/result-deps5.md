# result-deps5 (executor deps5, 2026-10-20 03:15 - 04:30)

HARD FREEZE respected: nothing under `📚️library/`, no manifest, no `📜️script.ts`, no taxonomy file was edited. Everything is STAGED in `C/deps5-tools/` and applied by `C/deps5-tools/apply.sh` (anchored, base-sha checked, refuses while a sync or `deps4-*.sh` runs). Sync attempt 60 (`deps4-loop2.sh`) was left running; my benchmarks only read it (they add one busy core).

## 1. Verdict: where the > 2 h go (sync attempt 50, 7000 s, measured from `deps4-sync50.log`, `C/deps5-tools/bench/timeline.py`)

| stage | seconds | share | what it is |
|---|---|---|---|
| installed-paths scans | 3,455 | 49 % | 12 scans (2 rounds x 6 generations), 135-554 s each, 70-84k entries |
| binding publication | ~2,420 (288 of 309 bindings done at the alarm) | 35 % | 32-binding cycles of 146-371 s: 4.6-11.6 s per binding |
| source-provider walks | 547 | 8 % | 3 whole walks (134, 196, 217 s); 6 more scoped walks + 6 input-plan reads still follow publication |
| workspace tree passes | 474 | 7 % | 15 passes of 617k operations (11-122 s), 6 before and 9 inside publication (one per full fence) |

Hypotheses from the task, answered with evidence:
- Per-file durable journal transactions: NO. The scan has no journal call; `store.recheck()` (durable journal + cross-process lease operation, 0.4-1.3 s per call under load per `result-cargo-pressure.md`) is called 3x per binding in `fence(false)` (once in the fence, twice inside `readJavascriptDependencyPreservationSourceV1`) = 927 per sync, i.e. a few minutes to 20 min.
- Per-path lease acquire/release: NO. `native-fixed-image:.cache/b/d` is held exclusively once for the whole run; the `resource-directory-admission:v1` events are one per binding.
- Repeated full-tree stamps per binding: NO. The 617k-operation tree pass runs only in the full fences (every 32 bindings, `fullFenceInterval`), 15 per sync, 474 s in total.
- O(n^2) re-verification: YES, three of them.
  1. `publishJavascriptDependencyPreservingSourceDescriptorV1` re-lstat'ed ALL 864 source ancestors at EVERY descriptor checkpoint: 248-355 checkpoints per binding = 107-153k lstat per binding (30-47 million per sync). Measured on the real tree, no-op publish of an already correct binding: 1.08-6.13 s each (load 40-50).
  2. `fence(false)` after every binding re-read all 308 endpoints (5,216 lstat, 0.18-1.3 s) plus 3 durable rechecks: O(n^2) = 95k endpoint observations per sync.
  3. The installed-paths scan re-lstat'ed every ancestor directory of the current entry at about 10 `check()` calls per entry: 79 lstat per entry, 6.63 million lstat and 797,670 checks per scan of a 84k-entry generation (302 s wall, 37.6 s CPU measured; the rest is waiting for a CPU at load 40-180).
  4. (provider walk) `retain()` re-lstat'ed the whole ancestor chain for every provider lookup: 842k lstat and 18,965 manifest read+parse+sha256 for 20.9k distinct observations.

## 2. The staged fix (5 changes, 1 schema constant, 3 new test suites, suite registration)

All in `…/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts` (15 anchored ops), `🧬️schema/📥️install.json` (+ `scanProgressWorkUnits: 4096`), plus registration in `⚡️caching/📜️script.ts` and `⚡️caching/📋️project.json`:

1. Scan (`scanJavascriptDependencyInstalledPathsV1`): `light()` (control + custody, no syscall) on the hot path; the ancestor stamps are checked at every progress frame and each directory is compared with itself when it closes; frames every 4096 work units instead of 64; dead `[...path].length` disjunct removed.
2. Publication ancestry: new exported `javascriptDependencyStrictAncestorsV1`; `ancestry()` of the publisher checks the strict ancestors of the endpoint's parent (the only directories through which the publisher resolves paths) instead of all 864.
3. Per-binding fence: new exported `requireJavascriptDependencyEndpointFenceV1` (endpoint stamp + link target + parent identity + ancestor anchors of the endpoint, 12 lstat) replaces `fence(false)`; `fence()` is now always the complete fence and runs at every `fullFenceInterval` (32) and after the last binding (unchanged final `verifyReady`/`fence`). The comparison is extracted unchanged as `requireJavascriptDependencyPreservationSourceUnchangedV1`.
4. Provider walk (`readJavascriptDependencySourceOwnerCandidatesV1`): `retain()` memoizes the first observation per path (upgrade to a full stamp still re-lstats) and `manifest()` memoizes the parsed profile per path+stamp, replaying the same `charge()` count. The two `replay()` passes (before/after `context.recheck`) are untouched.
5. Not changed: `readJavascriptDependencyPreservationSourceV1` (its test counts rechecks), the number of installed-paths scans (12), the full fences, `store.recheck()` cadence at full fences, all schemas other than the one constant.

## 3. Fence ledger (what each removed check was for and where it lives now)

| old check | new | why not weaker |
|---|---|---|
| scan: every ancestor lstat at ~10 points per entry | directory stamp (dev, ino, mode, nlink, size, mtime, ctime) compared when the directory closes; all ancestors at each frame and around publish/yield | The scan result only attests names+kinds. A name/kind change needs a create/unlink/rename in that directory, which advances its ctime irreversibly; ctime is part of the stamp, so unchanged stamp at close == unchanged listing since open. Intermediate checks only gave earlier failure. Mutation table (13 cases) passes on old and new code with identical accept/refuse decisions; deleting the close check fails 8 cases, replacing it by an inode-only check fails the same 8, removing the frame checks fails none (proves they are early-abort only), removing the file self-check breaks the event hook (cases "never fired"). |
| scan: `custody` + cancellation at every check | still at every entry/dirent (`light()`), at each await | No await exists between the removed duplicates. Cancel test: abort after 500 lstats stops within 30 more. |
| scan: progress every 64 work units | every 4096 | Frame count asserted == floor(work/4096) +-1 against a Python walk; old code fails this test. |
| publisher: 864 ancestors per checkpoint | strict ancestors of the parent per checkpoint | The publisher opens `targetDirectory` and `parentPath` and then works descriptor-relative; no other ancestor can redirect it. Every other ancestor is compared at the next full fence and the final fence. Table: lineage swapped early/late and retargeted endpoint refused in old and new; an UNRELATED ancestor swapped during a publish is now accepted by the publish but refused by the full comparison that follows (old code refused it one step earlier). |
| `fence(false)`: all 308 rows/parents/ancestors, 3 `store.recheck()` | endpoint fence for the published binding; full comparison + `store.recheck()` + tree custody + input replay every 32 bindings and at the end | Each unpublished endpoint is already compared against its expected stamp by `captureBinding` inside its own publish; the rest is compared by the full fence (ctime makes any change, even reverted, visible). `store.recheck()` is a custody liveness witness: acceptance still requires the final fence and the frame-settle recheck. Law test pins `fullFenceInterval === 0 -> fence()`, the final `verifyReady` fence and the absence of `fence(false)`. |
| provider walk: re-lstat on every lookup | one observation per path in the walk, validated by the two unchanged `replay()` passes and the manifest descriptor `same()` checks | Every memoized path is compared with its first-seen anchor/full stamp at the end (`full` rows by 7-field stamp); the only lost behaviour is aborting earlier. Provider-walk table (9 cases, mutation at 55/60/80/85/97 % of the checkpoints: manifest rewrite, provider removal, relink, unrelated sibling) identical on old and new; removing the second replay or both replays fails the table. Real tree: identical `execSha256 eea35dd09f95f2aa` (1,421 relations). The existing 19-mode `declared execution checks actual provider instances` fixture (run with `semver` linked; 3 stale modes fail identically) gives identical per-mode outcomes old vs new. |

## 4. Benchmarks (real tree, read-only; load average 31-52 on 10 cores; `C/deps5-tools/bench/`)

| step | current | staged |
|---|---|---|
| installed-paths scan of one generation (84,345 entries; `bench/scan.bench.test.ts`, 3 old / 4 new runs at load 31-52) | 302 / 202 / 168 s wall, 37.6-45.3 s CPU, 6,634,626 lstat, 797,714 checks | 48 / 54 / 64 / 51 s wall, 9.1-9.7 s CPU, 219,987 lstat, 462,967 light checks; identical entries and sha256 (`b482f05e60b6`, Python-verified in the test) |
| provider walk `readJavascriptDependencySourceOwnerExecutionV1` (6 scopes, 1,421 relations) | 137.5 s wall, 34.0 s CPU, 841,962 lstat, 18,965 manifest reads | retain memo only 87.1 s / 28.0 s / 47k lstat; with manifest memo 32.1 s wall, 14.8 s CPU, 47,056 lstat, 1,334 reads (same sha256) |
| `readJavascriptDependencySourceOwnerScopeReadV1` of the root scope (post-publication tail; the 5 other scopes refuse early because their bindings are not published yet, so only `.` is representative) | 55.8 s wall, 10.8 s CPU, 282,387 lstat, 5,288 manifest reads | 9.0 s, 5.3 s CPU, 58,743 lstat, 688 reads |
| publisher, no-op publish of an existing correct binding | 1,077 ms (60 bindings) / 3,989 ms (20) / 6,131 ms (5): 96-153k lstat, 248-355 checkpoints | 84.7 ms, 1,065 lstat |
| per-binding fence | 3 durable rechecks (0.4-1.3 s each, cargo-pressure numbers; not re-measured) + pass 0.18-1.3 s, 5,216 lstat | 0.11-0.15 ms, 12 lstat |
| binding stage, 309 bindings | measured 2,420 s for 288 (8.4 s each) | publish 0.085 s x 309 + 10 unchanged full fences (about 40-60 s each) = about 450-650 s (estimate from the components) |

Estimated sync at comparable load: scans 3,455 -> about 550 s, binding stage 2,420 -> about 600 s, provider walks 547 -> about 130 s (plus a shorter tail), tree passes unchanged: from about 2.8 h (attempt 60 pace extrapolated: installs skipped, 55 min to the start of publication) to about 30-40 min. Not measured end to end (needs the lease and an idle machine).

## 5. Apply procedure (between sync attempts)

1. Let attempt 60 finish (it entered binding publication at about 04:07) or stop it: `kill` the `deps4-loop2.sh` shell first, then `kill -TERM` the `bun repo dependencies sync --all` pid, wait until `ps` shows no `dependencies sync`/`deps4-*` process; the sync is resumable (generations and published bindings persist).
2. `sh C/deps5-tools/apply.sh check` (anchors + base sha); `sh C/deps5-tools/apply.sh apply --dry-run`; `sh C/deps5-tools/apply.sh apply` (add `--rebase` if a peer moved a target; anchors are verified either way). Combined with `C/walkperf-tools` and `C/tsc-stage-tools` storage ops it commutes in all six orders (checked: identical resulting text); install.json and the new test files are only touched here.
3. `sh C/deps5-tools/run-tests.sh real` (explicit paths, minted envelope, `SEMIO_TEST_ARTIFACT_DIR` realpath): expect 2 + 5 + 1 + 1 pass.
4. Restart `deps4-loop2.sh` (set `n` above 60). One apply invalidates any walkperf generation receipt once (storage module is in its closure).
5. Registered suites: `bun repo run @semio-tech/…:test-javascript-installed-paths | test-javascript-endpoint-fence | test-javascript-provider-walk` (project name of `⚡️caching`, not executed here).

## 6. Verification actually run

- `sh C/deps5-tools/run-tests.sh mirror /private/tmp/claude-501/deps5-mirror-test` (symlink mirror with the staged files): scan 2 pass (72 expects), fence 5 pass (94), providers 1 pass (15), diagnostics 1 pass.
- Existing suites old tree vs mirror, identical results: storage `🧪️tests/🟦️.ts` 26 pass / 20 fail, `👓️reading` 42 / 38, `📤️diagnostics` 1 / 0 (the failures are stale tests, e.g. the scan test expects maximumPathUnits 256 while the schema says 1024; `semver` missing).
- Same new scan table against the unpatched module: the 13-case mutation table passes (same decisions), the frame/cancel test fails (negative control). Publication table against the old publisher: only `unrelated-ancestor-swapped` differs, as designed.
- Mutations (`C/deps5-tools/mutants.py`): close-check removed / inode-only -> 8 cases fail; frame checks removed -> pass; second replay removed -> 2 cases fail; replays removed -> all refusal cases fail; retain-memo ignoring full upgrade -> passes (no fixture reaches an upgrade; code keeps the exact old behaviour for it).
- `tsc -p` (strict, repo tsconfig) over the patched storage module and the three new tests: 0 errors. `apply.py apply --dry-run --ignore-sync`: 8 ops files + 6 new files.
- Not run: any `bun repo` command, an end-to-end sync, Windows/Linux/devcontainer, taxonomy laws for the new test directories `📏️paths`, `🚧️fence`, `🔭️providers`, launch registration (`.vscode` untouched).

## 7. Open items

1. Installed-paths scans are still 12 per sync. A generation-local stamp ledger (record directory stamps in scan 1, re-lstat only directories in scan 2, rescan racy directories younger than 2 s) would cut about 290 s more; not staged because it needs its own fence proof.
2. The `fence()` calls before installs (6 complete fences, tree pass each) and the 6 scoped `ScopeRead`s after publication are unchanged; walkperf's frontier verification is the natural fix for the tree pass.
3. Stale existing tests worth fixing after apply: `bounded exact production scan` (needs path length 1024 fixture), `declared execution checks actual provider instances` (equal-copy, peer-copy, optional-absent-unowned-parent, unsupported-imports now accepted).
4. Launch/seed registration of the three suites; taxonomy admission of the new test directory names.

## 8. Reproduce the benchmarks
`python3 C/deps5-tools/apply.py mirror /private/tmp/claude-501/deps5-mirror-new --instrument` (and `... deps5-mirror-old --old --instrument`), `python3 C/deps5-tools/apply.py render storage <out.ts>` for the scan bench (`DEPS5_NEW_SCAN`), then `bun test ./C/deps5-tools/bench/<name>.bench.test.ts` with `SEMIO_SCRIPT_PROCESS_INVOCATION="$(bun C/deps5-tools/bench/mint.ts)"`; `python3 C/deps5-tools/bench/timeline.py <sync log>` prints the stage timeline. The mirrors were deleted after the run.
