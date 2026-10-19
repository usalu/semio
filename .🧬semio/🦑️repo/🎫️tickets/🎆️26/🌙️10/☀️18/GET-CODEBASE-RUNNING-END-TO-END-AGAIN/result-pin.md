# Result pin: build graph inputs account for script provider designations

Executor `pin` (Sonnet 5.5), foreground, no sub-agents, no cargo, no modifying git command, no `package.json` / `bun.lock` edit. `L` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`, `M` = `L/🏗️build/🕸️graph/📥️inference`.

## Result

DONE. Every command whose script closure reaches the framework provider loader (`🧰️framework/🔨️modules/🏃️process/🔌️providers/🟦️.ts`) now also hashes the designation, the designation schema and the full import closure (source and external packages) of every designated provider module. The 15 `cache:true` framework targets that run provider-using scripts therefore invalidate on product admission changes. Probe on the actor wasm script: closure 418 files, including all five provider modules.

## Design (schema-first, one declared source)

* The designation is now data: `L/🏃️process/🔌️providers/🔣️.json` (`{version:1, providers:{<port>: <path relative to the file>}}`), closed by `L/🏃️process/🔌️providers/🧬️schema/🔣️.json`. `repositoryScriptProvidersV1()` (the wire map handed to script processes) is derived from it (validated with `validateJsonSchemaSubset`), so the runtime designation and the build inputs share one source; the former hard-coded list in `🟦️.ts` is gone.
* Inference (`M/🧭️script-inputs/🟨️.mjs`): while following a command's literal imports, when the visited module is the provider loader (coordinate `SCRIPT_PROVIDER_LOADER_MODULE`), `scriptProviderImportsV1(cache)` adds the designation JSON and its schema as resources and each designated module as a source import, then the normal closure walk recurses into them (so their imports and external packages are inputs too). The extra edges live inside the memoized per-module computation (key includes the designation path), so observation/reuse records stay valid. The declaration is validated against its schema at inference time (fail closed).
* `createScriptInputCache(context)` accepts `context.scriptProviderDesignation` (default: the repository declaration) so the law can run against a hermetic designation; `ScriptInputCacheV1.providerDesignation?` added to `🟨️.d.mts`.
* Coordinates `SCRIPT_PROVIDER_LOADER_MODULE`, `SCRIPT_PROVIDER_DESIGNATION`, `SCRIPT_PROVIDER_DESIGNATION_SCHEMA` in `M/📍️coordinates/🟨️.mjs`; the designation and schema are in the inference implementation-revision authority list (`M/♻️revision/🟨️.mjs`), so editing either also invalidates inference records.

## Files

Created: `L/🏃️process/🔌️providers/🔣️.json`, `L/🏃️process/🔌️providers/🧬️schema/🔣️.json`, `L/🏃️process/🔌️providers/🧪️tests/🧫️fixtures/🔣️.json`, `M/🧪️tests/🔌️providers/{🟦️.ts,🔣️.json,🧬️schema/🔣️.json}`.
Changed: `L/🏃️process/🔌️providers/🟦️.ts`, `L/🏃️process/🔌️providers/🧪️tests/🟦️.ts` (declaration corpus test, Ajv vs framework validator), `M/🧭️script-inputs/🟨️.mjs`, `M/📍️coordinates/🟨️.mjs`, `M/♻️revision/🟨️.mjs`, `M/🟨️.d.mts`, `L/🏗️build/🧪️tests/🟦️.ts` (registers the new law).

## Tests (language-agnostic corpus + third-party oracle)

* `M/🧪️tests/🔌️providers/🟦️.ts` (3 tests): corpus `🔣️.json` (closed by schema, validated with Ajv) describes a workspace, a designation, a loading and a non-loading command and five edits with the observable relation: editing a designated module, a module in its import closure, or designating a new module changes the input hash of exactly the loading command; editing an undesignated module changes nothing; editing an unrelated module only changes the non-loading command. Real-repository test: three real framework scripts (actor wasm, ui react typecheck, framework typescript) include designation, schema, loader and all five designated modules, and every file of each module's esbuild-computed import closure (independent oracle).
* Mutation check: forcing `loadsProviders=false` fails the corpus and the real-repo test (2 fail); restored.

## Verification run

* `bun test ./…/🔌️providers/🟦️.ts` (inference law): 3 pass. `bun test ./…/📥️inference/🧪️tests/🟦️.ts` (reload/roster): 5 pass. `bun test ./…/🏃️process/🔌️providers/🧪️tests/🟦️.ts`: 3 pass (incl. the declaration corpus).
* `bun …/🎯️precise-command-inputs/🟦️.ts` (exact-closure law, esbuild oracle): exit 0. `currentImplementationRevisionV1()` loads and equals the loaded implementation revision.
* `inv-tools/tc.ts` strict tsc over the 3 changed/new TS files: 0 diagnostics.
* `bun test …/🏗️build/🧪️tests/🟦️.ts` (suite importing the new law): the 31 failures are all `Original parent Script process invocation required` (no script envelope outside `bun repo`), same class as before this change; no failure mentions providers.

## Open

* Not run: any `bun repo run` target or a real Nx graph (no cargo, no envelope outside `bun repo`), so the hash change inside an actual Nx run is verified through `genericTargetCommandInputs` (the function that builds target inputs), not end to end.
* Plain Node cannot load the inference root (pre-existing `ERR_IMPORT_ATTRIBUTE_MISSING` in `native-build/💾️storage` JSON imports); the new code only adds a `.ts` validator import of the kind the module already uses.
* Bazel actions: the provider closure is added by the same inference, so Bazel inputs derived from it follow; Bazel's own environment issuance is unchanged (see result-inv).
