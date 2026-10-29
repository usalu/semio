# 1028 result: demots

Status: GREEN. `test`, `canonical-architecture`, `test-runtime-catalog` pass (Bazel, rc 0); demonstrator TypeScript has 0 tsc errors.

## Runs
- `…:test`: first two attempts died in graph planning (inference-cache ENOENT race under concurrent graphs; then a peer's in-flight energy crate `semio-s-artifact-energy-model-mut-model-environment` had no native owner). Third attempt reached vitest and exposed the real failure (below); after the fix it PASSED.
- `…:canonical-architecture`: PASSED (4 tests). `…:test-runtime-catalog`: PASSED.
- Typecheck: no project typecheck target covers the demonstrator, so `node_modules/.bin/tsc -p C/1028-tools/demots/tsconfig.json` (extends root tsconfig, includes demonstrator ts/tsx, `types:["bun"]`). 0 errors in `♻️mit-bestand/🧺️demonstrator`. Non-demonstrator errors seen (not mine): puzzle `🌉️wasm/🟦️.ts` cannot find `@semio-tech/puzzle-wasm` (wasm package not built yet), and `PluginRuntime/🧺️handle-ui-intake/🟦️.ts` `issueTarget` on `Set<Promise<unknown>>` (framework renderer, 4 errors).

## Errors fixed
- Vitest suite `🔨️modules/🧩️runtime/♻️activation/🟦️.ts` failed: `Module …/📦️parsers/🧬️schema/🔣️.json needs an import attribute of "type: json"`. Cause: repo build bootstrap `🏗️build/🚀️bootstrap/🟦️.ts` line 15 imported the parser schema JSON without `with {type:"json"}` (the other two JSON imports had it). Fix: added the attribute.
- Audit finding 1: `serve`, `prepare-e2e`, `serve-e2e` now set `SEMIO_BUILD_MODE=prototype` (runtime `readBuildMode`).
- Audit finding 5/13: added `@semio-tech/framework-renderer-react: workspace:*` to demonstrator dependencies.

## Files changed
- `♻️mit-bestand/🧺️demonstrator/📋️project.json`
- `♻️mit-bestand/🧺️demonstrator/package.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🟦️.ts` (line 15)
- `C/1028-tools/demots/tsconfig.json` (scratch typecheck config)

## COORDINATOR
- `bun.lock` must be relocked for the new demonstrator dependency `@semio-tech/framework-renderer-react` (workspace). I did not run `repo dependencies lock`.
- Graph planning is fragile under concurrent peers (cache ENOENT race; peers' half-migrated crates break planning). Retrying resolved it.

## SHARED CHANGE
- `🏗️build/🚀️bootstrap/🟦️.ts` (repo build tool): json import attribute for the parser schema; required for any vitest suite that transitively loads the bootstrap module under Node.
