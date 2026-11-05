# Bootstrap and Preflight Oracle Run Plan

Use the installed native executable selected by `%LOCALAPPDATA%/Semio/repo/installed.json`; the current digest is `4cbfc6499526282de516b1bfe2b9e2711d3f8138d827014526e03966fbc4e69a`. All commands below start with `repo` and use existing registered dispatch. Run expensive gates only after the shared JavaScript dependency writer is terminal and ready.

## Registered Gates

The declared workspace Python prerequisite is `repo run workspace:deps-python --parallel=1 --lang=en`. Its root project target invokes the native dependency script with exactly `sync python`, preceded by the declared UI-styling and energy-oracle dependency targets. The native script receives its original Script invocation, storage reservation, native caller phase, cancellation and resource control; no synthetic invocation or envelope is needed. It provisions the pinned interpreter with pinned uv and synchronizes the workspace's locked dev/test groups into the declared dependency partition.

The previous preflight owner `build-doctor-preflight` also owned unrelated POSIX doctor, parity and conformance tests and required Bazel. It excluded Windows despite the preflight's three-platform production policy. A neutral gate fixture first observed the existing owner restricted to Darwin/Linux and the missing dedicated target (exit 1). The source declaration now assigns only the `🩺️preflight` tree to `build-preflight`, admits all three platforms and requires no Bazel. The registered gate is `repo run repo:test-build-preflight --parallel=1 --lang=en`; the aggregate includes it once. JSON contract verification passes. The route-table Ajv/coverage/Python oracle remains pending dependencies. The old owner retains its unrelated declarations and restrictions.

The dedicated gate discovers exactly four test files: the portable closed-schema corpus, exhaustive Python matrix, toolchain/remedy bootstrap audit, and remedy decision-table oracle. It declares the registered Python tool and depends on `workspace:deps-python`. The test files use the tool-independent Python test port; no host-specific invocation is authored here. The remedy and matrix Python processes run their checked-in scripts with `-I`.

## Focused Independent Corpus Checks

After Ajv is ready, run the registered Bun tool under the supported capacity wrapper:

```text
repo capacity run --memory=768 --cpu=1 -- repo tools exec bun -- test ./<test-file>
```

Run each file separately so an unrelated process birth is never fabricated or aggregated:

- `./🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/📦️workspace-links/🟦️.ts`
- `./🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/🏷️managed-workspaces/🟦️.ts`
- `./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🐹️go-layout/🧪️tests/🟦️.ts`
- `./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🚧️fence/🟦️.ts`
- `./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📏️work/🟦️.ts`

The dependency owner records the independent SQLite and actual extracted stage/capture/replay proofs in `dependency-fence.md` and `dependency-stage-work.md`. Their retained pure verification inputs are `fence-check.ts` and `stage-work-check.ts`; these already passed and need no repeat unless the integrated Ajv checks uncover a new failure.

The native Go source-projection, actual junction and cold-child tests have already passed and are recorded in `nov05-bootstrap.md`. Do not repeat unrelated suites unless a new edit or failure requires it.

## Current Python Boundary Evidence

The read-only registered command `repo tools exec --light python -- --version` presently refuses `tool-missing`. The registry names Python 3.14 and a Windows installation hint, whereas `workspace:deps-python` correctly provisions a managed uv environment. `repo tools verify python --lang=en` is not a supported selector in the older pinned setup owner: its terminal result is `Unknown pinned tool: python`; no process was left waiting and no installation was attempted.

The suite runner binds declared registered tools as `SEMIO_TEST_<TOOL>_BINARY`. The previous preflight Python port ignored that binding and probed PATH candidates (`python3`, `python`, Windows `py -3`). An observed red proved that a bound interpreter still consulted PATH. The repaired port derives the binding name from the existing registered runner helper, requires a host-absolute bounded path, probes the exact executable with unchanged arguments, and refuses invalid binding without PATH fallback. Eight neutral rows passed; explicit UTF-16 boundary tests cover supplementary glyphs. Independent Ajv execution remains pending.

The new suite-owned managed bridge reads the existing `PythonEnvironmentV1` plan and actual partition roots. It invokes registered uv with that plan's `--locked --no-sync --project` arguments and the declared isolated `sys.executable` query. The registered tool port retains caller annotation, inherited cancellation, bounded capture and nonzero refusal. The result parser admits one exact absolute interpreter path beneath the declared managed environment, with no multiline, empty, foreign or relative result. Nine neutral rows passed. The returned path is assigned through the Python registry's declared environment locator, then the existing runner resolves/binds it and the test port probes it. No interpreter layout, arbitrary executable override or invocation authority is invented.

The managed uv query and actual Ajv/Python gate have not run while the shared JavaScript dependency writer is live. Do not treat a system install, an injected guessed executable path, or a skipped Python oracle as a passing managed setup. This document records verified neutral behavior and the remaining runnable dependency order; it does not claim the full gate is green.

The retained short read-only command `repo tools exec --light bun -- <ticket>/oracle-neutral.mjs` passed against the current source. It executes the eight bound-tool cases, nine managed-result cases, three glyphs at exact/over UTF-16 boundaries for both ports, and dedicated gate ownership/tool/prerequisite declarations. It creates no endpoints or child interpreter. Its Node assertion check is separate from the pending independent Ajv execution.
