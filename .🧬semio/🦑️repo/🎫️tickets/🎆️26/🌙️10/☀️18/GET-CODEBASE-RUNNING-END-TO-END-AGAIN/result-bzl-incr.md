# result-bzl-incr: incremental conformance harness for Nx and Bazel behind the neutral interface

`C` = this ticket. `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`. Wave 1020d, resumed from the dead 1020c agent `a2cc2b64a904e44b8`. Stage-only until the coordinator sends "GO": nothing outside `C/` was written; verification ran in COPIES under `/private/tmp/bzlincr/` (removed at the end), one process at a time.

READY: staged set complete, `python3 C/bzl-incr-tools/apply.py` (dry run by default; `--write` applies) after GO; live run plan below.

## What it is

`repo incremental list|plan|prepare|run|discard` judges that every build implementation rebuilds exactly what an edit affects, with the SAME language-agnostic scenarios under `--tool=nx` and `--tool=bazel` and the results compared side by side.

- Disposable copy: `prepare` clones the tracked and untracked non-ignored files of the checkout (APFS clone where available) into `--root` (default `/tmp/sinc`, `<drive>:\sinc` on Windows; refused when inside or containing the live checkout) and runs `preflight`, `tools setup all`, `dependencies sync --all` there with the copy's own CLI. No edit can reach the live tree (`incrementalInsideV1` refuses absolute and `..` paths).
- Scenario run per implementation: prime (cache filled) -> `warm` (no-op: only uncached tasks may execute) -> `edit` (one-line mutation appended to one source) -> `revert` (exact original text restored in a `finally`, also on failure and cancellation; run again: everything restored from cache). 17 scenarios in 7 areas (framework TS, Rust crate, wasm component, Vite app, test target, lockfile, product activation) and 6 edit kinds (comment-only, body-change, signature-change, config, lockfile, unrelated-sibling).
- Expected set is a pure function of the declared slice (`incrementalExpectedV1`): `direct` (declared inputs contain the file), `must` (follows dependents whose output effect is `changed`), `may` (follows everything not provably `identical`), `cutoff` (dependents spared). Uncached tasks always run. Bazel follows every dependency (early cutoff on output digests), Nx follows only tasks that consume outputs (`dependentTasksOutputFiles`); that is the one place the two implementations legitimately differ and the report names it. Verdicts: `pass`, `stale` (a must-task did not run), `excess` (a task ran outside `may`), `stale-excess`, `failed`, `skipped`.
- Observation: Bazel through the compact execution log (`--execution_log_compact_file`, zstd, spawn field decoding in `observers.ts`: executed = spawn without cache hit, restored = all spawns cache hits, cached = no spawn); Nx through its own run record (`run.json`: `cache-miss` executed, local or remote hit restored).
- Comparison section in the report: per scenario and phase `same tasks` or `different tasks` with `only nx:` and `only bazel:` lists. Output EN and DE (`--lang`), `--json`, `--file=<report>`; progress per stage and cancellation checks between every stage (`advanceScriptInvocation`/`checkScriptInvocation`); exit code failing when any run is not a pass.
- Slice builder uses the Bazel adapter's own planner, cache decision, coverage classifier, source index and input expansion (`planBazelTasksV1`, `bazelCacheDecisionV1`, `bazelTaskCoverageV1`, `bazelSourceIndexV1`, `bazelTaskInputsV1`) so the expectation is the declared contract, and cross-checks the closure against the CLI's own `run --dry-run` plan for each tool.

## Changes (all staged; destinations under `B`)

New module `B/📏️incremental/` (9 files, longest destination 126 chars): `🟦️.ts` (model), `🧬️schema/🔣️.json`, `📋️scenarios/🔣️.json`, `👁️observers/🟦️.ts`, `🏃️runner/🟦️.ts`, `⌨️command/🟦️.ts`, `🧪️tests/🟦️.ts`, `🧪️tests/🧫️fixtures/🔣️.json` (fixture), `🧪️tests/🐍️oracle/🐍️.py` (independent Python oracle, standard library plus the `zstd` binary).

Anchored edits of existing files (`C/bzl-incr-tools/wire.py`, each idempotent, refuses on a missing or ambiguous anchor, composes with the other staged sets because it inserts at stable anchors only):
- `B/⚙️operations/🔣️catalog.json`: operation `incremental` (neutral/neutral, 5 subcommands, EN+DE), five flags (`--scenario`, `--implementations`, `--root`, `--phases`, `--fresh`) inserted before `--scope`, `incremental` added after `show` in the `operations` list of `--file` and `--json`.
- `B/⚙️operations/🧬️schema/🔣️.json`: `incremental` in `OperationName`.
- `B/⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json`: help snapshots (EN+DE): root rows, `incremental` and its five subcommand pages, inserted semantically after `preflight`; rows derived by the repository's own renderer (`render-help.py` -> `help-snapshots.json`).
- `B/📜️script.ts`: one dispatch line next to `preflight` (runs before any store or lease is taken in the live checkout; the child CLIs lock their own copy).
- `B/📥️caller/📤️child/🟦️.ts`: `incremental` excluded from the Nx argument translation list and its arguments validated at bootstrap like `preflight`.
- `B/🔧️tools/🏗️bazel/🧭️commands/🟦️.ts`: `bazelTaskReportOptionsV1(environment)` (one absolute path in `SEMIO_BUILD_TASK_REPORT` -> `--execution_log_compact_file=<path>`, else nothing; relative or multi-line refused).
- `B/🔧️tools/🏗️bazel/📜️script.ts`: import plus one `base:[...,...bazelTaskReportOptionsV1(process.env)]` on the execution invocation (the Bazel flags `bazelCommandPlanV1` receives).

Tools kept in `C/bzl-incr-tools/`: `names.py` (emoji path tokens), `gen-fixture.py`, `gen-scenarios.py`, `build.py` (src -> staged + manifest), `wire.py`, `apply.py`, `render-help.py`, `mkoverlay.py`, `mkapply-overlay.py`, `check-imports.py`, `tscheck.cjs`, `prep-tscheck.py`, `verify-help.ts`, `patch1-3.py` (authoring history), `src/` (authored sources), `staged/` (rendered set + `manifest.json` with hashes).

## Verification (exact)

- `python3 bzl-incr-tools/build.py`: 9 files rendered, no docstring emoji used twice, no path over 255.
- `python3 bzl-incr-tools/check-imports.py`: no dangling relative import. `node bzl-incr-tools/tscheck.cjs` (TypeScript compiler API with a virtual host that overlays the staged files and the patched `commands`/catalogue onto the live tree, repo compiler options, strict): 0 errors in the staged files.
- Overlay `/private/tmp/bzlincr/ov` (copies of the staged files plus the schema validator, `ajv` resolved from the live `node_modules`): `SEMIO_TEST_REPO_ROOT=<live> SEMIO_TEST_BAZEL_BINARY=<pinned bazel> bun test --timeout 500000 ./<overlay>/B/📏️incremental/🧪️tests/🟦️.ts`: 29 pass, 0 fail, 511 expects, 139 s. Covers: Ajv and the repository validator on schema, fixture, scenarios and every row; 15 evaluation cases, 7 verdicts, 13 mutations, 5 labels, 2 environment scrubs, 6 refusals each against the TypeScript implementation AND the Python oracle; compact execution logs decoded identically by TypeScript, the Python wire decoder and Bazel's own summary line; Nx run records against the installed Nx's own `StoreRunInformationLifeCycle`; REAL Bazel executing every Bazel evaluation case (executed set equal to the model, process count equal to Bazel's `INFO: N processes` line) and `rdeps` returning the modelled dependents; runner phases and restore-on-failure and cancellation; disposable-root and clone safety (emoji names, symlinks, nested root, 255-char paths); command arguments, bilingual errors, list/prepare/plan/run/discard with a stub CLI; the real ports; the comparison; the Bazel report option; the catalogue entry against the parser's option names.
- New Bazel fact proven with the real binary and encoded as a test: an action marked only `no-cache` is an action-cache HIT on the same output base (zero spawns on the second build); only a changed action key executes it. The adapter already does this right (`taskFingerprint` mixes `randomUUID()` for uncached tasks), so no adapter defect there; the stand-in workspace in the test mirrors it with a per-run nonce, and the harness models "uncached tasks always run" for both tools.
- `python3 bzl-incr-tools/apply.py --root /private/tmp/bzlincr/ap` (copies of the 7 edited files): 16 changes applied, second run `16 already applied`. `bun verify-help.ts` in that overlay: patched catalogue valid under the patched schema (Ajv strict), 60 of 60 help snapshots (EN+DE, root, operations, subcommands) equal the repository renderer, operation names equal the schema enum, 0 German texts equal to English.
- `python3 bzl-incr-tools/apply.py` against the real tree (read only, dry run): 16 changes, no refusal.
- Not run: anything against the real repo (quiet-tree rule), the operations/adapter/invocation suites after apply, `repo incremental` end to end, a disposable root prepare (needs network and the dependency sync).

## After GO: live plan

1. `python3 C/bzl-incr-tools/apply.py` (re-checks drift), then explicit-path tests: `bun test ./B/📏️incremental/🧪️tests/🟦️.ts` (with `SEMIO_TEST_BAZEL_BINARY`), `bun test ./B/⚙️operations/🧪️tests/🟦️.ts`, `bun test ./B/🔧️tools/🏗️bazel/🧭️commands/🧪️tests/🟦️.ts`, `bun build --target=bun 📜️script.ts`, `repo incremental --help --lang=de`.
2. `repo incremental list`, `repo incremental prepare` (once; heavy: clone + tools + dependency sync in the copy), `repo incremental plan --scenario=fw-ts-comment`, then `repo incremental run --scenario=fw-ts-comment` with both tools, widening to the other areas; fix every adapter defect the run finds.
3. Live risks to check first: `SEMIO_BUILD_TASK_REPORT` surviving the CLI's environment hand-off to the adapter; the 64 KiB output tail of `captureOwnedCommandV1` against large `--dry-run` plans (use `--graph=<file>` if truncated); the Nx `run.json` location; the full-graph `graph.json` size for the slice builder.

## Open items

- README EN+DE section for the command belongs to `docs-build` (the catalogue and `--help` are the authoritative text).
- `oracle.py` needs the `zstd` binary (the compact log is zstd); the TypeScript decoder uses the runtime's zstd and needs none.
