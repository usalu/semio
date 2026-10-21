# result-graph-fix-rest (wave 1021a, executor graph-fix-rest)

`C` = this ticket. `B` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`. Live edits, no git modifying command, no package.json edit (so no `repo dependencies lock` needed). Harness scripts: `C/graph-fix-rest-tools/` (h1 = wgpu source closure, h4 = one project, h5 = all 1,472 project files / 3,838 targets).

## (1) @semio-tech/framework-renderer-wgpu: FIXED
Root cause: the import `@semio-tech/ui/elements/Ports/interactive-jobs` is correct (`ui/package.json` exports `./elements/Ports/interactive-jobs`, and `@semio-tech/ui` is a declared `workspace:*` dependency of the package). The schema-owned source contract `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⚙️browser-build/🔣️.json` simply did not list `@semio-tech/ui` in `externalDependencies`. Fix: added `@semio-tech/ui` and (found by the same walk right behind it, `🚪️host-io/🟦️.ts` imports `@semio-tech/ui-contract/presentation/host-temporal-format`) `@semio-tech/ui-contract`, sorted. Verification (h1, calls the real `relativeSourceInputsAsyncV1` on the contract): before `Undeclared external source import @semio-tech/ui/...` then `...ui-contract/...`; after `OK files 21`, no further missing package.

## (2) @semio-tech/framework-os-dev "Build command Source escapes workspace" and (3) repo (`⚡️caching`) ENOENT lstat `🚦️ci`: FIXED, one shared root cause
`B/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs` (projectWithDefaults, the per-target `targetScriptSources` call) passed the RAW authored `target` (no default `cwd`), while every other consumer (`policy`, owner discovery) uses the defaulted one (`cwd: <projectRoot>`). `targetScriptSources` falls back to `options.cwd ?? "."` = the workspace root, so a project-relative command resolved against the workspace root:
- `@semio-tech/framework-os-dev` target `build-inputs` (`bun ../../🚚️distribution/📜️script.ts ...`, no cwd) resolved `../..` above the workspace -> "escapes workspace".
- `repo` targets `ci-baseline` (`bun ./🚦️ci/📜️script.ts baseline`), `generator-inputs` (`./🔏️inputs/...`), `toolchain` (`./🚀️bootstrap/...`) resolved to `<workspace>/🚦️ci` etc. -> ENOENT lstat. (Not stale declarations: `⚡️caching/🚦️ci` exists; nothing was renamed.)
- Every other cwd-less `bun ./📜️script.ts ...` target silently resolved to the workspace-root script (wrong, but existing), so this was also an unnoticed wrong closure.
Fix (1 token): `targetScriptSources(target, ...)` -> `targetScriptSources(inherited, ...)` (the `targetWithDefaults` result with `{projectRoot}`/`{workspaceRoot}` expanded in cwd, the same value owner discovery already uses). No manifest/project.json change.

Verification (h5, same inference function `cacheInternals.targetScriptSources` on every target of all 1,472 project files = 3,838 targets, once with the raw target = old behaviour, once with the defaulted target = new behaviour): raw 4 failures exactly = the 4 targets above; defaulted 0 failures. h4 on the two projects: raw 1 / 3 failures, defaulted 0. Not run: `projectWithDefaults` itself (needs the exclusive graph-writer lease; skipped on purpose, no graph build).

## Notes for the coordinator
- Peers (input-precision) have uncommitted edits in the same two library files (`🧭️script-inputs/🟨️.mjs`, `🧱️project-assembly/🟨️.mjs`); my edit is an in-place single-token replacement and was re-verified present after their writes.
- Remaining collect2 causes not touched here: own-output wasm and finite closure (workspace, @semio-tech/print) are input-precision's.
- The repository root `bun.lock`/package.json are untouched.
