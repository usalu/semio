# Uncaptured implementation import refused for a package JSON subpath

## Symptom
A native build restart crashed the repo tool: `Uncaptured implementation import refused: @semio-tech/process/artifacts/native-build/storage/schema.json (imported by ...⚙️setup/💰️funding/📋️intent)` with `declaringImage:false, declared:false, targetImage:false` (log `🗑️generated/native-component-flow-hub-14.txt`).

## Root cause
- Not an import-site problem: the specifier is a literal static import, `closureLiteralSitesV1` reports it as scanned, and the captured projection maps it to `📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️json/🟦️.ts`.
- The shown importer is the directory `.../📋️intent`, not the module. The refusal is raised inside the capture traversal (`captureNxAuthorizerSourcesV1`, line `runtimeResolveV1(specifier, dirname(...))`, hence the appended importer chain), which runs inside the tool process under the Bun immutable-image plugin.
- `runtimeResolveV1` is `Bun.resolveSync(specifier, directory)`. Bun runs registered `onResolve` plugins for `Bun.resolveSync` when the specifier ends in `.json` (reproduced standalone: hook fires for `schema.json` and `../x.json`, not for other package specifiers). The plugin in `🍞️bun/🟦️.ts` saw a directory importer that is no captured image, declared nothing and refused.
- It only surfaces when a `.json` package specifier is resolved for the first time in a closure-hosted process (restart, cold memo); cached resolutions hide it.

## Fix
`🏗️build/📥️source/🔏️closure/🍞️bun/🟦️.ts`: the catch-all `onResolve` returns (default resolution) when the importer is not a captured image and is a directory. A directory can never be an importing module, so this only covers `resolveSync` queries; real imports from images or unknown files are still refused.

## Test
`🏗️build/📥️source/🔏️closure/🧪️tests/🕸️resolution/🟦️.ts`: new "a captured Bun child resolves a package JSON subpath from a directory without an importer refusal" runs the real stream loader and `Bun.resolveSync` of that exact specifier from a directory. Failed before (importer refusal), passes after; the resolution, sites, mismatch, diagnostics and test-tree closure suites pass via `repo test-file`; `repo cache status --lang=en` runs without refusal.
