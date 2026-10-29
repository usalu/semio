# 1029 nxdep result

STATUS: GREEN. `repo run workspace:deps-browsers` (Nx, default tool, no NX_FORCE_REUSE_CACHED_GRAPH) exits 0: "Successfully ran target deps-browsers for project workspace" (`🗑️generated/1029/nxdep-run.{txt,rc}`; the failing pre-fix run is kept as `nxdep-run1.txt`).

## Root cause
`nxBuildEngineInputsV1` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`) adds the private Nx engine named input `{externalDependencies:["nx","typescript",...]}` to EVERY project at Nx projection time (`projectBuildGraphForNxV1`), by package name; Nx resolves a package name to the `npm:<name>` external node.
The graph Nx receives is a published SLICE of the generation (`assembleStream().finish()` in `🏗️build/🕸️graph/🕰️generations/🟨️.mjs`). A slice keeps only the external nodes that its projects' edges reach or that their fragment inputs name in `externalDependencies`. Two defects:
1. the engine input is not in any fragment (added after the slice), so its packages are never requested; for `workspace:deps-browsers` the pinned slice held exactly one external node (`npm:@iarna/toml`), no `npm:nx`, no `npm:typescript` (verified in `.cache/b/g/m/snapshots`);
2. `referenced` names were matched only by node key (`npm:x`), never by package name.
Bazel does not read the Nx projection, hence green. It only surfaced now because the smaller closure of `workspace` no longer reaches `npm:nx` through some other project's edge.

## Fix (no input removed)
- `generations/🟨️.mjs`: `assembleStream` takes `external` (names the consuming backend adds to every project) and resolves every `externalDependencies` name by node key, else `npm:<name>`; the transitive closure over global edges is unchanged.
- `generations/🟨️.d.mts`: option types for `assemble`/`assembleStream`.
- `generations/🔌️load/🟨️.mjs` + `.d.mts`: `loadGenerationGraphV1` passes `await host.externalDependencies?.()` to the assembly; host contract documented.
- `🏗️build/🕸️graph/🟦️.ts` `generationHostV1`: `externalDependencies()` returns the packages of the generic and native engine inputs (`nxBuildEngineInputsV1`, lazy import, same pattern as `inputs/🎯️selected/🦀️rust`).

## Tests (language-agnostic oracle + third party)
- `generations/🧪️tests/🌊️stream/🐍️.py` (independent Python model): package-name resolution, new owner `h`/node `npm:plain`/case `an-external-dependency-named-by-its-package-name-without-an-edge`; fixture `🔣️.json` regenerated with `🐍️.py write`.
- `generations/🧪️tests/🌊️stream/🟦️.ts`: new test for the `external` option. Results: stream 6 pass, `🔌️load` 77 pass, `🧪️tests/📤️portable` 3 pass.

## SHARED CHANGE
Generation assembly + generation host (shared repo-infra). Additive: slices may carry the engine packages' external nodes (+ their lock closure); full graphs unchanged. This changes graph implementation identity, so the next run of every peer rebuilds the graph once.

## Files
All under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/`: `🟦️.ts`, `🕰️generations/🟨️.mjs`, `🕰️generations/🟨️.d.mts`, `🕰️generations/🔌️load/🟨️.mjs`, `🕰️generations/🔌️load/🟨️.d.mts`, `🕰️generations/🧪️tests/🌊️stream/{🐍️.py,🔣️.json,🟦️.ts}`.
