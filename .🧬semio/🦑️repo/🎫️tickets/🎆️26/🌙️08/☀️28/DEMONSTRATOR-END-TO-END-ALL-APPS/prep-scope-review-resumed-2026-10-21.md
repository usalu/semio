# Preparation Scope Review — Resumed 2026-10-21

Source-only audit; no checks, builds, policies, application sources, or processes changed.

## Focused Build Mode Target

Owner manifest: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`, target `test-build-mode`. It currently lacks execution.kind source and sourceInputObservation freshness, whereas `test-build-caller-birth` explicitly declares both. Applying that same semantic source execution contract to mode is justified by its actual command; this is not an ownership bypass.

Its existing sole runtimeSourceEntries is already correct and should remain: `{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🟦️.ts`. The source dispatch in `🧪️tests/🔬️typescript-package-commands/🏗️build-and-direction-routes/🟦️.ts` runs exactly that file with Bun and an owned120000ms test budget. Its imports and literal source reads preserve the invocation implementation, fixtures/schema, neutral mode module, bootstrap resolver, and issuance source observation. Additional compiled closure entries are not needed merely because the test reads/transpiles source.

Actual test requirements: Bun test/SQLite/build, Node execution, TypeScript parsing/transpilation, esbuild oracle, Ajv, yargs-parser, and existing repo binary help for both backend selections. Bootstrap capabilities are planned and inspected; no selected native application is built or published by these tests. No Cargo/Wasm build prerequisite is required. The native repo binary itself and installed test tooling remain genuine prerequisites and must not be replaced with mocks.

The target currently uses inputs default,^default,testEnv,reads; reads expands the entire library/**/* subtree. Keep their freshness/ownership verification while correcting execution role. In `🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs`, named input modules are promoted to compile entries only for compile observation; explicit runtimeSourceEntries still receive executable source closure verification. In `📦️project-nodes/🟨️.mjs`, source execution is excluded from native target preparation. Thus both metadata fields matter: freshness expresses why broad inputs are observed, source expresses the execution domain. A neutral source-roster regression should bind the exact route and sole entry, as existing source-roster tests do for other source targets.

## Bounded Walker Observation

`🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs`, readBuildSourceInputMatchesV1, preserves per-scan exclusions and compiled-target association while sharing traversal by exact root. It yields/cancels during directories, entries, patterns, and candidates; avoids symlinks and generated directories; sets matched files rather than returning duplicates. No concrete correctness defect found.

One demonstrable bounded performance limitation: roots are memoized by exact string only. Patterns root/**/*.ts and root/child/**/*.json traverse child physically twice when both are selected; later scans of the exact same root reuse the list. Also every candidate is revisited for each pattern, necessary for distinct match/exclusion semantics but potentially reducible by indexing. This does not establish the cause or duration of current repeated preparation phases. Do not skip source checks or weaken cancellation to optimize it; a separate neutral overlapping-root regression and measured scan counters would be required before changing traversal reuse.

No claim of native test success is made; live check owners retain verification.

## Why Source-Demand Can Admit a Much Larger Owner Closure

Current source-demand target has source execution + freshness + exact single runtime entry, but its input list remains default,^default,testEnv,reads. In this owner manifest, reads expands `{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/**/*`. Caller-birth instead uses explicit process budget, build subtree, compiler subtree and script inputs, with no ^default and no whole-library reads. These are materially different admitted freshness extents.

Exact mechanism: owner-discovery.readBuildSourceInputMatchesV1 returns matched files regardless of compile role; discoverBuildOwnerAdmissionFactsV1 adds every matched file to value.files. admission.closeBuildOwnerAdmissionV1 then sourceOwner(file)→admit for EVERY value.files entry and admits all value.dependencies. Its dependencyInputs branch admits named-input owners too. owner-discovery accumulates ^default in caretInputs and propagates it to discovered dependencies, so broad dependency default observations recur through the finite owner closure. Freshness prevents those named inputs becoming compiled module entries; it does not suppress source-owner admission or matching. Source metadata likewise excludes selected native target preparation, not canonical source ownership.

Thus1039project progress may represent a real expanded admitted owner closure rather than a cheap listing of canonical registry names. The exact contribution of broad reads, ^default and shared router static imports is not proven by the count alone. The code proves those first two expansion paths; no evidence justifies blaming static router imports as the sole cause. An admitted owners/inputs summary or existing stage counts is required to attribute proportions. No recommendation to bypass registry/source verification or refactor closure semantics is made.

Independent40file content snapshots remained unchanged through2026-10-21T01:24:05.483Z, ruling out observed defining-module turnover during this finite window as an explanation for the delay.

## Demonstrator Unit Source Execution: Exact Runtime Union

Independent source audit caught a finite-entry omission before save: commandSourceImports excludes import.meta.vitest bodies. Four includeSource roots alone therefore omit seven real guarded test helpers plus two guarded production modules. Include all literal guarded modules explicitly; do not relax parser guard behavior.

Exact union16 roots (all verified existing absolute paths69–109UTF16units):
- `🧪️tests/🎚️config/🟦️.ts`
- `📜️script.ts`
- `🪧️brand.ts`
- `🔨️modules/🧩️runtime/♻️activation/🟦️.ts`
- `🧪️tests/🧪️demonstratorpanebranding/🟦️.ts`
- `🧪️tests/🎭️acceptance/🧬️console/🧪️.ts`
- `🧪️tests/🏛️shell-brand/🟦️.ts`
- `🔨️modules/🧩️runtime/🟦️.ts`
- `🧪️tests/🧪️demonstratorruntimebuildvariants/🟦️.ts`
- `🧪️tests/🧪️demonstratorpanedescription/🟦️.ts`
- `🔨️modules/📦️site/🗺️tile-serve-mode/🟦️.ts`
- `🧪️tests/🧪️demonstratormaptiles/🟦️.ts`
- `🧪️tests/🧪️demonstratorcompileclosure/🟦️.ts`
- `🧪️tests/🧪️demonstratorpanebootvariants/🟦️.ts`
- `🧪️tests/🧪️demonstratorexamples/🟦️.ts`
- `🧪️tests/🧪️demonstratorunionreceipt/🟦️.ts`

Derivation:7roots are config,4includeSource modules,console suite and long shell-brand suite. Script guarded imports add runtime implementation,4registerTests helpers and tile-serve-mode; guarded brand import is already a root. Brand adds2helpers. Activation adds unionreceipt helper. Total7+6+2+1=16. Normal static/package imports continue to derive transitively from those roots; this finite list is not a claim of16files total closure.

Owned leaf runVitest is library/🧪️testing/🧪️vitest/🟦️.ts, exactly the function re-exported by old public testing barrel; switching import preserves repositoryVitestPolicyV1, devToolingEnv and runVitestV1 budget/process owner. Moving runtimeBuildVariants import into existing Vitestguard narrows ordinary root launcher closure, but that runtime implementation is included explicitly above for tests.

Source execution does not erase generatedproducer authority. targetScriptSources still binds/requires actual command source, visits explicit runtimeSourceEntries with requiredSources, builds compiledNames and generatedImports, and computes generatedDependencies. applyBuildGeneratedTaskDependenciesV1 still adds exact producer dependencies; self-output rejection remains. execution.kind source changes native target classification/preparation, while freshness prevents broad named matchedinputs becoming executable compile roots. Explicit generated imports can still require legitimate producer tasks and large closure. Keep build/test-e2e compilation/native contracts untouched.

## Combined Saved Sixteen-Entry Boundary Review

Reread actual fixture, closed schema, AST/Ajv regression, manifest and root router after combined save. Both test and canonical-architecture have source execution/freshness and identical16runtimeentries matching independently derived union above. Commands and forwardAllArgs true remain exact; build and test-e2e retain no source execution/freshness/runtimeentry override and their authored site/E2E commands with forwardAllArgs false. Root router now imports exact owning runVitest leaf and runtimeBuildVariants import is inside existing Vitestguard.

AST test derives both include conditional branches plus includeSource from actual config, starts with config itself, recursively follows relative literal dynamicimports under actual import.meta.vitest guards, deduplicates and compares sorted exact16fixture roster. Schema requires16unique entries and closed2cases/2controls. It uses Ajv and TypeScript without introducing Bun/SQLite into Node Vitest. Windows relative outputs normalize backslashes; all checked entry paths remain bounded. No concrete remaining source/test mismatch found. Source execution plus freshness remains accepted distinct metadata semantics; actual command and explicit entries still undergo Source closure/generateddependency processing as proven above. No test/native command was run by reviewer.
