# Cleanup Delegation Review

Real Script constructor(root,repoRoot,invocation) in process routing13 checks the incoming invocation and stores all three values. BundleScript24 forwards these values unchanged. Current CleanScript.run is async27, awaits runCachePrune109, and that private async method awaits new CachePruneScript(cachingRoot,this.repoRoot,this.invocation).run124. Thus current delegation uses the genuine parent Original, correct module/workspace roots, and propagates failures/cancellation instead of losing the async result.

The neutral route test extracts the actual class through TypeScript AST, compiles through TypeScript and esbuild, records constructor/arguments and compares call decisions against SQLite. Its fake Script intentionally isolates delegation; this is not itself a genuine issued-resource runtime test. Existing runtime invocation refusal/success coverage should remain separate.

## Registered Owner Route

@semio-tech/repo-lib:test is declared in library/📦️packages/🟦️typescript/📋️project.json, invokes that package 📜️script.ts test, which registers TestScript from library/🧪️tests/🔬️typescript-package-commands/🟦️.ts. That handler21 runs the explicit workspace-contract/🟦️.ts entry via runRepositoryTestCommand. The entry currently has30 explicit side-effect imports; generic test-directory placement does not register the new cleanup command test. Add its explicit ../../🧼️workspace-cleanup/🎮️command/🧪️tests/🟦️.ts import there, and include the new path/fixture/schema in owning target runtime Source closure metadata as required. Alternatively register a separate explicit owned cleanup test target.

## Actual Graph Withholding Cause

Recovered compressed graph fragment .cache/b/g/m/generations/o/ae9eed38a1770cc38ce77f624ffe9df787741525ee1d332cb7e11d6f1d5dc4a5.json.br using bounded native Brotli decode. Workspace unavailable.reason is Cannot find module at OS plugin/window/config/tests/fixtures/retained-pack-load/🔣️.json; require stack points to retained-pack-load/🟦️.ts sha94607c11e62f5eeed0aa2a7edf880011271c401483e0f5c94449fa4f9e31cc7e. It is a missing fixture import, not TypeScript parse diagnostics.

script-inputs unresolvedSourceImportV1 wraps missing resolution as BUILD_SOURCE_UNPARSEABLE, then generic withheld display says does not parse. Fix fixture path/ownership or restore intended fixture; do not edit valid syntax to silence the misleading message. Root and parser owner received exact evidence. No Source changes by auditor.
