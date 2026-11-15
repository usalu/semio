# pyoracle - unblock the 23 norm Python oracle tests

## Root cause (corrects the earlier diagnosis)
1. `repo dependencies sync --scope python` was the wrong command. `dependencies sync|lock --scope <owner>` is the Bun/JavaScript dependency store; `python` is not a declared JS Source owner, so it fails with "Actual declared requested Source scope required". It is not a stale `@semio-tech/course` receipt, so nothing needed to be deleted or healed there. Python is provisioned by the root target `workspace:deps-python` (native script `sync python`: pinned uv, CPython 3.14 via `provisionInterpretersV1`, `uv sync --locked --group dev --group test` into the digest-keyed environment `.cache/b/d/m/python/environments/workspace-<key>`). The key hashes the manifest and lock digests, so a changed lock creates a new environment (self-healing by construction).
2. The real defect was the consumer side. The command layer sets `SEMIO_PYTHON` from `🧰️framework/⚙️configuration/🏃️process/🔣️.json` `python`, the literal `"python"` (system python 3.9 or nothing). Nothing connected the provisioned environment to test runs, so the oracle tests (`interpreter()` in `⚖️compliance/🧪️tests/🔬️python-oracle`) never saw it.
3. Interpreter provisioning is already zero-touch inside `deps-python`; `.cache/b/t/m/python/interpreters/cpython-3.14*` now exist.

## Fix
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🧰️toolchains/🟦️.ts`: new `pythonEnvironmentInterpreterV1` (Scripts/python.exe on Windows, bin/python elsewhere) and `provisionedPythonV1(workspace)` (the digest-keyed environment interpreter, `null` until synchronized or after any lock change).
- `🧰️framework/🔨️modules/🏃️process/🧭️routing/🎛️command/⚙️configuration/🟦️.ts`: `SEMIO_PYTHON` = explicit env, else the provisioned environment interpreter, else the configured fallback.
- Test `🧰️framework/🔨️modules/🏃️process/🏗️build/🧰️toolchains/🧪️tests/🟦️.ts` (new describe "the provisioned Python interpreter"): absent before sync, exact path once present, null after a lock change, Windows layout. `bun test` of that file: 26 pass, 4 skip, 0 fail (run via `repo tools exec bun`).

## Environment provisioning on this host
`repo run workspace:deps-python` cannot run here: the graph read fails (exit 75) because the root project `workspace` needs about 3530 MB to infer and the per-project limit is 3512 MB. Equivalent step executed instead (same variables the native script computes, `UV_PYTHON_DOWNLOADS=never`, existing interpreters): `repo tools exec uv -- sync --locked --project . --group dev --group test` -> exit 0, environment `.cache/b/d/m/python/environments/workspace-3b97251b6599d245`.

## Oracle tests
See the section below (cargo run `-p 'semio-s-plugin-norm-*' -- oracle python`, script `pyoracle-test.sh`).

### Result: not run to completion (host capacity)
The workspace-wide `cargo test -p 'semio-s-plugin-norm-*' -- oracle python` had compiled 185 crates after 40 minutes (rustc wrappers queued behind the critical describe build; load 66) and was stopped to free its 3584 MB grant. Pass/fail counts for the 23 oracle tests: 0 run (not failing; not started). Re-run when capacity allows:
`T/pyoracle-test.sh <tag> -- oracle python` (log `T/🗑️generated/pyoracle-<tag>.log`). Expected: `SEMIO_PYTHON` now resolves to `.cache/b/d/m/python/environments/workspace-3b97251b6599d245/bin/python`. Caveat: this holds only for command layers that go through `resolveCommandConfigurationV1`; if `repo cargo test` does not, export `SEMIO_PYTHON` to that path for the run.
Unverified: whether `repo cargo` test targets use the command configuration layer (not traced).

## Gap closed (trace, then fix) - applied BEFORE the tooling freeze
Trace: only the `command` script (`🏃️process/📜️script.ts` -> `resolveCommandConfigurationV1`) used the command configuration layer. `repo cargo ... test` and cargo test targets spawn through `admitCargoExecutionV1` (`🏃️process/🧪️testing/🦀️cargo/🟦️.ts`), which copied `request.environment ?? process.env` without ever setting `SEMIO_PYTHON`. So the gap was real.
Applied (3 files, all under `🧰️framework/🔨️modules/🏃️process/`; nothing edited since the freeze message):
- `🏗️build/🧰️toolchains/🟦️.ts`: `withProvisionedPythonV1(env, workspace)` is the single resolver (explicit value, else provisioned env interpreter); `provisionedPythonV1` now walks up from a Cargo sub-workspace to the checkout holding `pyproject.toml` + `uv.lock`.
- `🧭️routing/🎛️command/⚙️configuration/🟦️.ts`: uses the resolver (fallback to configured `python`).
- `🧪️testing/🦀️cargo/🟦️.ts`: `admitCargoExecutionV1` passes the environment through the resolver with `admission.workspaceRoot`.
- Test `🏗️build/🧰️toolchains/🧪️tests/🟦️.ts`: nested-workspace resolution, resolver adds/keeps `SEMIO_PYTHON`. `bun test` file: 26 pass, 4 skip, 0 fail (not type-checked by a build).
Note: these edits landed before the freeze but may still reload the service host once; no further edits will be made.

## Oracle tests: still pending
Waiting for `describe15.log` to end with `EXIT` (last seen: graph slice verification 2123 of 3420). Then run `T/pyoracle-test.sh <tag> -- oracle python`.
