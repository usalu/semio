# PY1: Python oracle bound zero-touch from the files

## Cause
`suite.tools` was hand-declared per suite and read in 4 places (runner `🗂️suites/🏃️run`, caching + build suite scripts for the managed uv interpreter, test-file routing `python:`). Files that call the oracle helper (`testPythonV1`, `requireTestToolV1("python")`) in a suite without `tools:["python"]` got no managed interpreter and no `SEMIO_TEST_PYTHON_BINARY`: "Independent Python oracle required" / "Test tool python not found".

## Fix (root, no per-suite lists)
- Schema-first hints in `PROC/🧪️testing/🧰️toolchain/🔣️.json` (`usage.hints` per tool, `usage.closure` = follow relative imports into `🧪️tests` dirs, max 512 files).
- `testToolsUsedByV1(text)` / `testToolsUsedByFilesV1(files)` in the toolchain module (direct hints plus test helpers imported, e.g. harness/store helpers).
- `processTestSuiteToolsV1(suite, root, files)` = declared tools U derived tools that the tool registry knows (python yes, ripgrep not registered = host lookup); `processTestSuiteBindsToolV1` for the scripts. Used by: suite runner (pins the binding for exactly the run files, so test-file's one-file row binds per file), caching + build suite scripts (uv environment), test-file routing (`route.python`). Barrel re-exports added.
- Existing explicit `tools` stay valid (union); none needed hand-adding. All offenders in the three tables (build, caching, process) are fixed by derivation, including `law-coverage-tests` (store/contract) and the test-file member.
- Probe robustness: tool probe timeout 10 s hard-coded -> `binding.probeTimeoutMilliseconds: 60000` (the first stretch run failed "oracle required" because the bound python's `--version` probe timed out at load 50).

## Law
`LIB/🏗️build/🔬️test-file/🧪️tests/🔭️tool-binding/🟦️.ts` (+ independent Python scanner `🐍️.py` re-implementing hints+closure): (1) synthetic suite without tools: direct, helper-indirect and plain files bind python/none; (2) for every discovered file of the three real tables, the independent scan decides, and test-file routing (`route.python`) and `processTestSuiteToolsV1` must both bind python when it says so; (3) the three runner sources never read `.tools?.includes(` directly.

## Runs (repo test-file, one at a time, taskpolicy -b, load 36-58)
| file | result |
|---|---|
| tool-binding law | 3 pass / 0 fail (1080 s) |
| UC/💾️store/🧪️tests/📦️contract | 20 pass / 4 fail (1044 s); before: 0/1 python unbound. The 4 failures are 60 s test timeouts under load (spawnSync status null), not binding |
| 📚️library/🧪️testing/🎚️levels/🧪️tests/⏲️stretch (plain route) | 2 pass / 1 fail; python found; remaining fail is a real divergence: Python model expects quick x7.9 = 2370000 ms, code returns 1800000 (clamp to schema ceiling from the budget clock/F2 work); the model or test needs the clamp, owner F2/levels |
| 🏗️build/🔬️test-file/🧪️tests | 13 pass / 1 fail (1218 s); before 11/3. Failure = deadline-fixture starved by load |

## Open
- Stretch oracle vs clamp (above). Law not re-run after the probe-timeout edit (one constant read from protocol).
- Plain-mode python preparation failure is still only a notice (`run.python-unavailable`); with derivation the runner now refuses when python is registered but not provisioned.
