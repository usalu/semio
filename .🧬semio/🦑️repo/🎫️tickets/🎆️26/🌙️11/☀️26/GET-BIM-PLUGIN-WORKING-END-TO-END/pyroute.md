# pyroute: python route for `repo test-file`

## Contract (derived from the oracles)
- 43 bim `🐍️.py`: 36 have `main(sys.argv[1:])` under `__main__` and document `python 🐍️.py check <path to 🧫️fixtures/...>` (exit 1 on disagreement); 7 (`export-bim-1-ifc/*`) are modules imported by the parent oracle, with no entry point.
- Route: a `.py` file is mode `oracle`, suite `python`, always managed Python. Arguments: the `check <path to 🧫️fixtures/...>` / `<S>/🧫️fixtures/...` usage in the file's own text; the declared path is looked up from the oracle's directory upwards to the checkout (nearest wins), passed as `check <abs path>`; a declared path found nowhere is refused (`oracle.fixtures-missing`). No check usage: run bare, the exit code is the verdict (e.g. shared-worktree boundary oracle).
- A module (no `__main__`) runs as the nearest entry-point oracle above it (deduplicated in the plan); none above: refused (`oracle.orphan`).
- Interpreter: `provisionedPythonV1(checkout)` (toolchain plan: the locked `workspace-*` environment `repo dependencies ensure` syncs; `Scripts/python.exe` on Windows); missing: failed row telling to run `repo dependencies ensure`. Env `PYTHONIOENCODING=utf-8`, `PYTHONUTF8=1` (policy).
- Runs as a child of the Script envelope through `runRepositoryCommand` (progress lines, deadline 900 s, SIGINT/cancellation ends the child like the other routes, `--bail`, `--json`, `--dry-run`, en/de).
- Python files that need other CLIs (stdin JSON, `CORPUS.json`, `--catalogue`) are not standalone oracles; they run bare and fail on usage, as before they were refused.

## Changes (schema first; single TS implementation, no other language implements this command)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔬️test-file/`: `🧬️schema/🔣️.json` (Policy.oracle, mode enum `oracle`), `🔣️.json` (oracle policy), `🟦️.ts` (routing, `testFileIsOracleV1`, `testFileOracleOwnerV1`, `testFileOracleArgumentsV1`), `🏃️run/🟦️.ts` (`runOracleV1`), `📖️texts/🔣️.json` (3 new keys en/de, refusal text widened).
- Tests: `🧪️tests/🟦️.ts` (2 policy mutations, corpus test with independent Python twin, run test passing/failing/bail, SIGINT cancel test), `🧪️tests/🐍️oracle/🐍️.py` (modes `oracle-arguments`, `oracle-owners`), `🧪️tests/🧫️fixtures/🔣️.json` (`oracleArguments`, `oracleOwners` vectors), fixtures `🧪️tests/🧫️fixtures/🐍️{passing,failing,sleeping}/🐍️.py`.
- Docs: `🧰️framework/🛍️products/🦑️repo/📓️build-guide.md` (en/de).
- bim: `✏️s/🔌️plugins/🏙️bim/.../🧪️tests/💬️export-bim-1-bcf/🐍️.py` docstring now declares its usage line (`check <path to 🧫️fixtures/🚪️bcf/🏢️frame/issues.bcf>`), previously undocumented.

## Verification
- No rebuild needed: `repo` runs the TS sources directly (a dry run showed the new route immediately); `repo typecheck` on the three touched TS files: 0 errors.
- `repo test-file` on all 43 bim `🐍️.py`: exit 0, 36 passed (7 modules folded into `export-bim-1-ifc`), 0 failed, ~40-50 s total.
- Non-bim: `repo test-file 🧰️framework/.../🤝️shared-worktree/🚧️boundary/🧪️tests/🐍️.py` passed (292816 vectors, bare run).
- `repo test-file` on the test-file tests file: 18 pass, 1 fail. The failure is pre-existing and not mine: "a suite row's declared nested launches..." expects launches suites `[component-reclaim, native-artifacts, native-queued-cancellation]` but another session added `native-component-*` suites to the table (stale expectation in that test, left alone).

## Open
- A missing-environment refusal test is not included (the envelope resolves the real checkout, so a temp checkout cannot simulate it).

## Nested-launches fix
- File changed: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔬️test-file/🧪️tests/🟦️.ts` (line 261 expectation only).
- Stale expectation `["component-reclaim","native-artifacts","native-queued-cancellation"]` now lists all 10 declared launch suites: added `native-component-closure`, `-failures`, `-jobs`, `-real`, `-retry`, `-seal`, `-streaming` (verified in `🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🔣️.json`, each with a `test-<id>` target in `📋️project.json` and existing `Cargo.toml` launch paths; not a bug).
- Result: before `repo test-file` on the file: 18 pass, 1 fail; after: 19 pass, 0 fail (exit 0). Logs: `🗑️generated/nested-launches-before.log`, `🗑️generated/nested-launches-after.log`.
