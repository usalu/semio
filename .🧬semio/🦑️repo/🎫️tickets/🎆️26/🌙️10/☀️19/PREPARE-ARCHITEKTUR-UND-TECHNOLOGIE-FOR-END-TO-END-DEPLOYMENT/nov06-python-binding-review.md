# Python Binding Independent Review — November 6

No blocking defect found in the requested three suite routes from this bounded source review. This is static evidence, not an execution or a readiness receipt.

The caching and build suite manifests select the seven repaired oracle consumers through storage-compiler, storage-compiler-processes, and build-host-capacity; all three declare python and have no file-launcher or function override. The caching and build wrappers conditionally call the same workspace query with the invocation cancellation signal and pass its result to the shared suite runner. The runner resolves declared tools against that environment and overlays the resulting binding into the Bun file process environment. The consumers require the binding and probe that exact absolute executable; missing or invalid bindings fail before PATH lookup. The seven consumers contain no previous Python command fallback.

The workspace query protocol names workspace, uv, and python and queries sys.executable using isolated Python. Its implementation passes the planned arguments and variables to the registered tool, propagates the abort signal, bounds captured output, and rejects an interpreter outside the planned environment before projecting the registered locator. The reviewed plan records locked/no-sync and only-managed/manual controls; this review did not independently trace their plan implementation. Existing workspace environment readiness and actual canonical execution remain unverified here.

Scope observation: the shared suite runner's function and custom file-launcher branches do not pass the computed tools/environment to runOwnedCommand. None of the three reviewed declarations uses those branches, so this is not a defect in the requested paths; future Python suites using those alternatives require separate review. The query occurs before the runner's platform check, but all three selected suites explicitly support Windows, Linux, and macOS.

No uv command, environment setup, oracle, graph, managed binding injection, or job polling was performed. No lifecycle or source change was made.

## Follow-up: Actual Query Prerequisite

The plan implementation in process build toolchains sets workspace project root to `.`, main Python series 3.14, and groups dev/test. It derives the environment directory from project, uv version, interpreter series, host, groups, and manifest/lock digests. Its actual run arguments are `run --locked --no-sync --project .`; its variables override the incoming environment with `UV_NO_CONFIG=1`, `UV_PYTHON_PREFERENCE=only-managed`, `UV_PYTHON_DOWNLOADS=manual`, and the planned project environment, interpreter root, Python series, and cache paths. The helper appends `python -I -c import sys;print(sys.executable)`.

**Prepared-environment guard is absent.** The planner reads and hashes the manifest and lock but does not check that the target environment exists or was synchronized. The helper invokes uv immediately after planning and only afterward validates that the returned absolute interpreter path is lexically inside the planned environment. Therefore a successful query does not prove that workspace dependencies were prepared. The [uv CLI reference](https://docs.astral.sh/uv/reference/cli/) describes project environment creation by `uv run` and says `--no-sync` avoids synchronization; those controls do not establish an existence guard. The exact behavior of this repository's pinned uv version with a missing environment was not executed or independently inspected, so creation is a documented possibility rather than an observed result. Keep the canonical query behind genuine workspace preparation and do not infer sync readiness from the returned interpreter.

No uv invocation or environment mutation occurred in this follow-up.

## Replacement Standalone Query Review

The replacement selects receipt-verified uv, runs `python find` with the planned main series and explicit managed-only, no-project, no-download and system-selection flags, propagates cancellation and bounds output, and lexically checks the resulting path against the managed interpreter root. The focused tests cover the principal command, receipt wiring, path and failure cases. This review did not execute the command or independently establish canonical gate success.

**Windows environment case gap:** `workspaceTestPythonVariablesV1` removes only the exact uppercase `UV_PYTHON_PREFERENCE` property. An inherited differently cased spelling such as `uv_python_preference` remains in the object. The uv registry inherits variables, and `toolEnvironmentV1` retains their spelling; Windows environment variable names are case-insensitive, so the child can still receive the conflicting preference despite the explicit managed flag. The current test uses only uppercase names and a Linux environment. Remove all case-insensitive matches for Windows and cover mixed-case inherited values through the actual child-environment builder. This is a static finding; no Windows reproduction was run.

Containment is lexical, not a realpath or interpreter receipt verification; the standalone interpreter itself is selected by the registered uv and then probed by the suite runner. The evidence provided covers one genuine host query, not all platform matrices or full workspace environment readiness.
