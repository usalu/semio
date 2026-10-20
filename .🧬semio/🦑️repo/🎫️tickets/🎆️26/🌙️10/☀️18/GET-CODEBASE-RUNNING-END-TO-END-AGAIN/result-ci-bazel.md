# result-ci-bazel: CI on Bazel behind `bun repo` (executor ci-bazel, wave 1020e)

**READY** (staged, verified in an overlay, NOT applied). `C` = this ticket folder, tools and staged set in `C/ci-bazel-tools/`. `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`, `Z` = `B/🔧️tools/🏗️bazel`.
Nothing outside `C/` was written. No push, no CI run, no git-modifying command, no `bun repo`, no sub-agent. The only Bazel run was a scratch workspace under `/private/tmp/ci-bazel` (three-line test rule, provisioned Bazel 8.4.2, `nice -n 10`, one process at a time), removed afterwards.

## What the set does

`.github/workflows/build-system.yml` is rewritten around one value, `BUILD_TOOL` (default `bazel`, `workflow_dispatch` input `tool` switches to `nx`). Every build step is `bun repo --tool="$BUILD_TOOL" ...`; no step names an implementation, an installer, a flag or a cache path. The only other program is the CI report (`$CI_REPORT`), which reads what `bun repo` printed.

| job | runs | what it proves |
| --- | --- | --- |
| `plan` (ubuntu, 5 min) | one bash step | the `affected` range: pull request `base.sha..HEAD` (merge commit), push `before..HEAD`, new branch or dispatch scope `all` = `--all`, dispatch default `origin/main..HEAD` |
| `smoke` (linux, macos, windows, 60 min) | pinned Bun, `bun repo --help`, `preflight`, `tools setup $BUILD_TOOL`, Bun store restore, `dependencies sync --all`, `cache remote status` (informational), cache proof | zero-touch start and a local cache that stores once and serves the second run (remote cache skipped with `--skip-remote-cache`, so a peer cannot satisfy it) |
| `affected` (3 OS, after `plan`+`smoke`, 120 min) | full-history blobless checkout, `affected --targets=$TARGETS <range> --dry-run` (selection printed to the step summary), `cache status` before and after, `affected` for real, test summary, artifact upload | builds and tests only the changed projects and their dependents; hit rate reported; JUnit XML of every executed test uploaded (`test-results-<os>`, 14 days) |
| `parity` (ubuntu, after `plan`+`smoke`, 60 min) | `tools setup all`, the same `affected --dry-run` under `--tool=nx` and `--tool=bazel`, cache proof of the sample target under nx | both implementations select the same tasks for the range (`@teaching/architecture-course:test` is the sample) |

- **Shared remote cache, configured only by env/secrets, no tool logic.** It uses the neutral contract already in the tree (`B/🏃️execution/🧬️schema/🌐️remote-cache.json`, rendered by `bun repo` for Bazel and Nx): repository variable `SEMIO_REMOTE_CACHE_ENDPOINT` (or a secret of that name), optional variable `SEMIO_REMOTE_CACHE_INSTANCE`, secrets `SEMIO_REMOTE_CACHE_READ_TOKEN` and `SEMIO_REMOTE_CACHE_WRITE_TOKEN`. Pull requests, forks and dispatches: `ACCESS=read-only`, `UPLOAD=false`, read token. Only a push to the default branch: `read-write`, `true`, write token. No endpoint: the remote cache is off (contract default).
- **No endpoint = GitHub cache transport.** The local task cache directory is whatever `cache status` prints for the selected implementation (`cache-directory` command of the report), restored always, saved only by default-branch pushes, key `disk-<tool>-<os>-<arch>-<sha>` with prefix fallback (content-addressed, so no invalidation key; this also removed the Bazel file names `MODULE.bazel`, `.bazelrc`, ... that the previous workflow hashed). The Bun store travels as before.
- **Concurrency**: group `build-system-<event>-<PR number or ref>`, cancel only for pull requests (main pushes finish, so the cache is saved). Timeouts on every job. Permissions `contents: read`.
- **Pins**: `actions/checkout` v7.0.1 `3d3c42e5...`, `actions/setup-node` v7.0.0 `82076278...`, `actions/cache` v6.1.0 `55cc8345...` (unchanged), new `actions/upload-artifact` v7.0.2 `cf430e030ddbb5b0abf93d22962f4752f3646cd9` (latest release, resolved with `gh api repos/actions/upload-artifact/git/ref/tags/v7.0.2`, commit object). The Bun asset hashes were re-read from the official `SHASUMS256.txt` of `bun-v1.4.2` and equal the previous pins.
- **Two local composite actions** replace the triplicated setup: `.github/actions/bun-repo` (Node pin, pinned Bun with SHA-256 table per runner OS/arch, pin check against `package.json`) and `.github/actions/cache-proof` (clear, run, status, run, status, verdict). `dependabot.yml` gets the two directories so the inner pins keep being watched.

## New code (schema-first, tool-neutral, EN/DE, tested)

1. `B/🧾️ci-report/` (`🟦️.ts` pure functions, `📜️script.ts` CLI, `🧬️schema/🔣️.json`, `🧪️tests/🟦️.ts`, `🧪️tests/🧫️fixtures/🔣️.json`). Commands: `cache-directory`, `cache-delta` (hit rate = hits / (hits + executed), from the Bazel activity ledger the neutral `CacheStatus` already carries), `proof` (store grows, then stays; hit recognized by the implementation's own evidence: Nx markers, Bazel `disk|remote cache hit` marker, activity delta, or spawn markers), `plan` (selection size), `parity` (set equality of the tasks whose target the list names), `tests` (JUnit counts, failed tasks, cached count; `--require` makes a missing or failing result fatal). Exit 1 only for a failed verdict, 2 for usage. Boots with no `node_modules` (the validator import closure has no bare imports). Texts EN and DE (`--lang`, `SEMIO_LANGUAGE`, locale), both languages checked for identical keys and placeholders.
2. `Z/🧾️test-results/` (`🟦️.ts`, tests, fixture): new neutral contract `SEMIO_BUILD_TEST_RESULTS_DIR` (absolute, at most 200 characters, like `SEMIO_BUILD_EVIDENCE_DIR`). After every Bazel `test` step the adapter copies the `test.xml` of the final attempt of each label, run and shard from the build event stream into that directory (ASCII file name `<label>-<sha256 first 8>[.runN.shardM].xml`) and maintains a closed `index.json` (label, neutral status, cached, duration, file; rows replaced by file so several builds of one run add up; atomic). Never throws (warns), like the metrics ledger. Nx has no JUnit; the `tests` step then reports "no test results were exported".
3. Two anchored edits in `Z/⚙️operations/🟦️.ts` (one import line, one call line inside `recordBazelBuildMetricsV1`, right after the evidence copy). Disjoint from conc-bazel's 58 edits and parity's evidence edit (that one is already live and is my anchor).

## Apply (stdlib Python, all platforms)
```
python3 C/ci-bazel-tools/build.py          # only if a peer touched the workflow or the operations anchors (re-records base hashes)
python3 C/ci-bazel-tools/apply.py          # dry run (default) of all three groups: new 10, edit 3, replace 1, problems 0
python3 C/ci-bazel-tools/apply.py --write  # all-or-nothing, backup in C/ci-bazel-tools/backup-<stamp>
python3 C/ci-bazel-tools/apply.py --undo C/ci-bazel-tools/backup-<stamp> [--write]
```
Groups `report`, `adapter`, `workflow` (`--group`); `workflow` refuses without `report`. Drift (workflow changed since the recorded base hash, an anchor not unique, a new file with other content) writes nothing and exits 1. Apply needs a quiet tree only for the Bazel operations edit (no graph or storage-lease code touched). Afterwards, one file at a time: `bun test ./B/🧾️ci-report/🧪️tests/🟦️.ts`, `./Z/🧾️test-results/🧪️tests/🟦️.ts`, `./Z/⚙️operations/🧪️tests/🟦️.ts`; optionally the real-Bazel test with `SEMIO_TEST_BAZEL_BINARY` and `SEMIO_TEST_BAZEL_CATALOG=.cache/b/d/m/bazel/modules/<id>`.
Set in the repository settings: variable `SEMIO_REMOTE_CACHE_ENDPOINT` (http or https origin of `bun repo cache serve` or any Bazel HTTP cache that also speaks the Nx cache API), secrets `SEMIO_REMOTE_CACHE_READ_TOKEN`, `SEMIO_REMOTE_CACHE_WRITE_TOKEN`. Unset = GitHub-cache fallback.

## Verification (overlay `/private/tmp/ci-bazel/ov` = symlink mirror with the staged set applied; `C/ci-bazel-tools/verify.sh`)
- `bun test` ci-report: **17 pass** (323 expects). Language-agnostic fixture (`C/ci-bazel-tools/gen/*.json`, generated by `mkfixture.py` with an independent Python oracle and cross-checked with **jq** and **xmllint**; real Bazel 8.4.2 logs, build event streams and `test.xml` files from the scratch runs: fresh and cached, passing and failing). Covers extraction of JSON from decorated console output (CRLF, braces in strings), deltas, all three proof rules plus negative rows (nothing stored, second run stored, no hit, wrong tool's markers), selections with wildcards and configurations, parity equal/different, JUnit counts (nested suites, CDATA, comments, look-alike elements, self-closed), test summaries, language resolution, EN/DE key and placeholder parity, CLI exit codes 0/1/2 with `::error::` annotation.
- Workflow policy tests in the same file, **dry-run render** of 7 scenarios (the table below is the test's own output; the real plan script and the fallback script are executed with bash against a temp git repository): pinned SHAs only, permissions, timeouts, matrix linux/macos/windows, no step names an implementation or installer, every `bun` call is `bun repo`/`bun "$CI_REPORT"`, Bun hashes equal the release hashes, `BUN_VERSION` equals `packageManager`.

| scenario | remote cache | range | GitHub-cache steps |
| --- | --- | --- | --- |
| pull request, same repo | endpoint set, `read-only`, upload false, read token | `--base=<base.sha> --head=HEAD` | none |
| pull request from a fork | endpoint empty (off), read-only, no token | same | restore only |
| push to main, endpoint set | `read-write`, upload true, write token | `--base=<before> --head=HEAD` | Bun store saved |
| push to main, no endpoint | off, `read-write`, write token | same | restore and save |
| push, zero `before` | | `--all` | |
| dispatch nx, scope all | read-only, read token | `--all` | none (endpoint set) |
| dispatch affected, empty base | off | `--base=origin/main --head=HEAD` | restore only |

- `bun test` test-results: **6 pass + the real-Bazel test**, run with the pinned binary and the provisioned module catalog: **7 pass**. The fixtures are real build event streams (including `cachedLocally`), plus synthetic rows for shards, attempts, remote-only (`bytestream://`) outputs, missing files, statuses without test.xml, Unicode labels and non-JSON lines. The exported directory is read back by the neutral report (end-to-end contract).
- `bun test` of the existing Bazel operations suite: 10 pass, 2 skip on the patched overlay and on the live tree (identical).
- `tsc` strict + `noUncheckedIndexedAccess` over all staged TypeScript plus the edited operations module: 0 errors (`tsc.sh`; the only errors printed by the project are in the budget `♻️reset` module of another owner).
- **actionlint v1.7.7** (built from source, GOPATH in `/private/tmp`): `build-system.yml` with both local actions resolved: exit 0, no finding; two negative controls (undefined matrix key, missing required composite input) exit 1, so the check is live. `bash -n` over all 28 bash bodies: 0 failures (`lint-scripts.ts`). `shellcheck` and `pwsh` are not installed: shellcheck rules and the two PowerShell bodies (long paths, Windows Bun install, unchanged semantics) were not linted.
- `apply.py`: dry run on the live tree (new 10, edit 3, replace 1, 0 problems), idempotent second run (all `applied`), undo restores byte-for-byte, drift refusal proven (workflow edited after the base hash: exit 1, nothing written).

## Not verified / open items
1. The workflow itself was not run (instruction). First run is the proof; likely first failures: `bun repo preflight`/`tools setup` on Linux and Windows runners (xplat-linux/xplat-win items), Windows emoji path in `$CI_REPORT` (bash argument conversion; the repo already relies on emoji paths everywhere).
2. The real stdout of `bun repo cache status` and `affected --dry-run` was not observed: the report finds the last top-level JSON object that starts a line, so log lines around it are tolerated, but a line prefix on the JSON itself would not be. The proof reads the `activity` fields of `CacheStatus` first and falls back to log markers.
3. Hit rate needs the activity ledger (Bazel only); under Nx the summary says so. Restoring files into the owned cache directory behind the store's accounting is still the open question of xplat-ci item 5.
4. `affected` after a core change selects almost every project (134k targets, hours); a cold first main run will be long, later ones use the remote cache or the GitHub cache. Tune `TARGETS` (dispatch input) and the 120 min timeout after the first runs.
5. The neutral CLI help and README do not yet mention `SEMIO_BUILD_TEST_RESULTS_DIR` (catalogue and help snapshots belong to regen-help); documented in the module docstring and the workflow header.
6. `parity` compares `affected --dry-run` of both tools. For Bazel that is the neutral rule (documented in the catalogue), not the real `rdeps` run; a mismatch between the neutral rule and Bazel's rdeps result would need a plan dump of the real run.
7. Test discovery: the new test files follow the `🧪️tests/🟦️.ts` convention; whether `bun repo test` picks up a new module directory automatically was not run.
8. The `ciDependencies` law of `build-surfaces` (xplat-ci item 7) still expects the retired quiz workflow; it does not read this file.

## READY2: no default language (cli-zt READY2)
- Workflow: `SEMIO_LANGUAGE: en` in the workflow `env` (every job, also composite-action steps); `smoke` gains one step "Print the usage and run the preflight in German" with `SEMIO_LANGUAGE: de` (`bun repo --help`, `bun repo preflight`) so the German paths run on all three runners.
- CI report follows the same rule: `resolveCiLanguageV1` no longer falls back to English and throws "Language required: pass --lang en|de or set SEMIO_LANGUAGE" (CLI exit 2); fixture rows for an empty environment and an unsupported locale now expect the refusal; new policy test asserts the `en` env and exactly one German smoke step.
- Re-verified in a fresh overlay: report tests 18 pass, adapter tests 6 pass, tsc strict clean, `bash -n` 29 scripts ok, actionlint v1.7.7 exit 0, dry-run render still 7 scenarios. Staged set rebuilt (`apply.py` dry run: new 10, edit 3, replace 1, 0 problems).
