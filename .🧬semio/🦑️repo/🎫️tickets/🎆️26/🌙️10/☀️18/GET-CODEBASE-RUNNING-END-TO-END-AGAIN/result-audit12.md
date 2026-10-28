# result-audit12 (lane audit12, prefix `ac`, wave 1027b): G46 live composition audit

STATUS 20:25 (final). G46 lost no change: every multi-batch file has a hash-linked chain, and every batch's own base→new hunks are still present on live. The parse scan found no duplicate binding. G46 did add **4 regressions**, and one more pre-existing bug surfaced on live. Fixes for all five are staged in 5 batches, each mirror-verified (`ac1`): red on base, green with the fix.

Nothing on live was edited, no product run was started, and no sub-agent was used. The mirror `ac1` was dropped.

## FOR coordinator (each batch is independent; base = live 20:05; `python3 -I C/audit12-tools/mkbatch.py <batch>` rebuilds any batch from live through its anchored edit in `C/audit12-tools/edits.py`)

**Common form for every batch.** `S=C/audit12-tools/staged/<batch>`:
- apply: `python3 -I $S/stage.py apply`
- revert: `python3 -I $S/stage.py revert`
- verify: `sh $S/verify.sh /Users/ueli/Documents/semio`

The verify scripts are read-only: pinned bun, no envelope, artifacts in `.cache/test-sandbox/ac-v`.

1. FOR coordinator: **`sigexit`** fixes a HIGH regression from progress `sigint`. Since `sigint`, `withScriptProcessEnvelope` calls `process.exit(130)` inside its error path. A caller that catches the cancelled run therefore never prints its DATA and never sets its own status.
   - Red on live:
     - `📥️inputs/🧮️digest/🧪️tests` is 3/3: the cancel child exits 130, not 1, with no DATA, and the Original then closes.
     - `📥️inputs/🧪️tests/📏️windows` is 2/2: expected 0, got 130.
     - The rusttests lane expected 5/0 and 3/0.
   - Fix (1 file, `🏃️process/🟦️.ts`): keep the notice and rethrow the error. Exit 130/143 only on `uncaughtException`/`unhandledRejection` of that very error. Bun reports a top-level-await rejection as `uncaughtException`; I probed this.
   - Green: digest 5/0, windows 3/0. The watchdog sigint law stays 12/0: exit 130/143, the EN/DE notice, no dump, with the Python oracle. progress stays 24/0.
   - On the mirror, verify must run with `--no-seal`, because the seal breaks 2 ps-based watchdog laws on base too.
   - This touches a hot file. Rebuild with `mkbatch.py sigexit` if it drifts.
2. FOR coordinator: **`farmexports`** fixes a regression from farmid `authority`. That batch exported `adoptInferenceAuthorityV1` and `inferenceAuthorityRowsV1` from `👁️observation/🟨️.mjs` and `.d.mts`, but did not add them to the declaration-roster fixture.
   - Red on live: observation 51/1 and `🦀️rust/📥️source` 3/1.
   - Fix: add the 2 names to `👁️observation/🧪️tests/🧫️fixtures/🔣️.json`.
   - Green: 52/0 and 4/0.
3. FOR coordinator: **`typedecl`** fixes the 5 tsc errors that G46 added. They are attributed by a pre-G46 overlay type-check (`C/audit12-tools/tscbase.mjs`):

   | Batch | Error | Fix |
   |---|---|---|
   | windows `winlaunch` | `🔑️session/🟨️.d.mts` still declares `canonicalRootV1(root)`, while the test passes `platform`: 3× TS2554 | add `platform?:string` to the declaration |
   | progress `phases` | door `languages` is typed `string[]`: TS7053 | type it `readonly ("en"\|"de")[]` |
   | rusttests `envelope` | suites test `toContain(unknown)`: TS2769 | `toContain(String(…))` |

   - Green: tsc shows only the 3 pre-G46 errors (operands, followup, optional-cache). door 9/0, service 119/0, suites 9/0.
4. FOR coordinator: **`saytest`** fixes a test left stale by progress `say`. The lane only parse-checked it.
   - The bug: the diagnostics budget law reads `finish(true)` outside its forced `SEMIO_LANGUAGE=en`. A German, or two-language, hidden-lines row then lands before the newest lines.
   - Red on live: 0/1.
   - Fix: both `finish()` reads are inside the en block.
   - Green: 1/0 under de, en and C.
   - The other 11 fails in that file are envelope-only and pre-existing.
5. FOR coordinator: **`privparent`** fixes a latent bug in optcache `store` that shows on live.
   - The bug: the private graph parent budget models the staging suffix as `.9999999.99.tmp`. The observation module writes `path+"."+pid+"."+Date.now()+".tmp"`, and `Date.now()` has 13 digits. So live `.cache/test-sandbox` is admitted at 248 units, but the real path is 259.
   - Red on live (bare, default parent): `denial` fails with `Observation block staging path exceeds256 characters`, so the suite is 9/4/1. The node/Python oracle is red.
   - Fix: count 7 pid digits and 13 clock digits, and add a short system scratch parent: `/tmp/sg`, or `<tmpdir>/sg` on Windows.
   - Green: the oracle passes, and the bare suite is at the lane's documented 10/3/1 (the same 3 maintenance-worker laws). No private repository is left behind.
   - The same 10/3/1 holds on live with an explicit `/private/tmp` parent.
6. FOR coordinator (not a code fix): pinpub's **`pinned-publication` Nx cache law** ("each pinned publication receives a distinct Nx cache") is red on live 3 times out of 3, including at load 27: `Actual recovered committed child required` from `retireNativePlatformFundedChildStorageV1`, via `📦️budget/📥️opening:89`.
   - This law never ran green anywhere. The lane skipped it on its mirror (socket path), and its live 12/0 was a forecast, not a run.
   - Likely cause: state in the native pool. `.cache/b/n/m/*/.native-birth-*.jsonl` holds 9 birth journals that look half-born, the same class as nx3-d 10-25.
   - Owner: pinpub / nx.
7. FOR coordinator (already handled by the gate at 19:35): builderlock `multiwriter` laws were green on live at 19:20: 21, 179, 1 and 2 pass, 0 fail. course:test still went red. The lane laws do not exercise the lease kernel's signal identity.

## Composition: no lost change, no duplicate binding
- Tool: `C/audit12-tools/compose.py`, re-run at 20:00.
- It covered 29 landed manifests in the gate's order (`🗑️generated/coord/g46-apply.sh`, then `g46-apply2.sh`), which is 220 files. Docs5 was checked separately with its own `apply.sh check`: all 9 sub-batches are applied.
- **22 files were touched by ≥ 2 batches.** Every link has `base(k) == new(k-1)`, so each later batch was rebuilt on top of the earlier one. For every batch, the reverse of its own `-U0` diff applies to live (`git apply --check -R`), so each intent is present.
- Overlaps:
  - agents16 allrun → behind → progress umlaut (host-capacity ×5)
  - farmfree → farmid (project-farm mjs; farmfree's mjs row is base == new)
  - demand / optcache → pinned (fixture, support)
  - rusttests → cmdid (command test)
  - winlaunch → winps pin (runtime-provision go)
  - progress umlaut / lang / rows / sigint / usage → status / say / phases (11 files)
- Removed lines with no paired addition (`stats.py`): I reviewed them by hand and all are intended rewrites. Examples: pinpub's stale tests, and winlaunch's `/dev/urandom` replaced by `crypto/rand`.
- Parse: `scan.ts` over 128 TS/MJS files found 0 bad. 58 JSON files parse, with no duplicate keys. 9 Python files compile. Go vet, build and test binaries pass for windows/amd64, windows/arm64, linux/amd64 and darwin/arm64, and the darwin Go tests pass 51/51. The jloss watcher builds with rustc: 11/11 unit tests, and the type-checks for x86_64-windows, aarch64-linux and x86_64-linux are clean.

## Laws on live (pinned bun 1.4.2, from the root, one suite at a time; `C/audit12-tools/laws.sh` + `laws.tsv`, summary `🗑️generated/audit12/laws.txt`)

These match the lanes' reports:

| Batch | Result |
|---|---|
| agents16 behind / farmfree | 7/0, 25/0 |
| farmid | 10/0 |
| demand | 30/0, 4/0, 4/0 |
| nxsparse | 1/0, 11/0, 2/0, 7/0 |
| pinned admission | 20/0 with a short artifact dir |
| jloss | 2/0, 13/0, 9/0, 8/0 with the freshly built watcher |
| freshmtime | law 2/0 |
| wasmlock | 4/0, oracle 0 violations |
| cbudget clock | 31/0, index ok |
| split | 10/0, 3/0, 6/0, 10/0 (dashhot env-run) |
| winlaunch service | 119/0 |
| fleetpy | 10/0, 1/0 |
| wininstall | 8/0 |
| cisrc | 6/0 |
| cmdid | 5/0 wire, 3/0, 1/0 |
| progress lang / status / umlaut dirs | all green except the lane's base set (receiver 1, envelope-only) |
| winps native | 6/0, 14/0 |
| vitestext (Play / demonstrator) | 75/75, 57/57 |
| host-capacity Python oracles | ×4 "vectors equal the oracle" |

These differ from the lanes' reports and are explained:

| Law | Result | Explanation |
|---|---|---|
| allrun `🌱️grow` (g31 replay) | 0/1 | Red before G46 too; I bisected on the mirror. agents16 already documents it as load-flaky, red on base. |
| freshmtime suites | 157/1 | The 1 is an "unhandled error between tests": `unit-cache/🧪️tests/⏳️slow/🟦️.ts` is a compiler shim listed in the lane's `suites.txt`. This is a harness defect, not a product defect. |
| usage door test | 8/1 | Only when `SEMIO_TEST_ARTIFACT_DIR` lies inside the checkout (the marker walk finds the real root). Bare it is 9/0. |
| rusttests command test | 5/0 | Better than the expected 2/4, because cmdid declared the library birth. |
| ledger | 4/4 | The lane's known-open set. |

## Files
- Tools: `C/audit12-tools/`: `compose.py`, `stats.py`, `blame.py`, `tscbase.mjs`, `tsconfig.g46.json`, `laws.sh`, `laws.tsv`, `bisect.sh`, `edits.py`, `mkbatch.py`, `verify.common.sh`.
- Staged batches: `C/audit12-tools/staged/{sigexit,farmexports,typedecl,saytest,privparent}/`. Each holds `manifest.json`, `base/`, `files/`, `stage.py` (template, mode law OK) and `verify.sh`. `privparent/` also holds `law_privparent.py`.
- Evidence: `C/🗑️generated/audit12/`: `laws.txt`, `laws/*.log`, `compose*.txt`, `stats.txt`, `tsc*.txt`, `verify/*.log`.
