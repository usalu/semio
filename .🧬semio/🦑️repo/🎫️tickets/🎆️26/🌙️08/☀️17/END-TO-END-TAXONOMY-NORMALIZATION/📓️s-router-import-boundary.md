# Scoped Router Import Boundary

## Change

The root router previously imported `.storybook/scopes.ts` eagerly. That module builds its package-derived Storybook scope catalog at import time, so unrelated taxonomy and package-preview commands could trigger complete repository discovery before their scoped work started.

The two actual consumers now import the scope module asynchronously when needed: explicit Storybook scope parsing, and the existing Storybook freshness check in the full validation gate. Their callers await them. No Storybook validation branch or scope definition was removed, and no runtime dependency was added.

## Test-Driven Evidence

The permanent language-neutral vector declares the deferred module, exact consumer methods, caller methods, and catalog-root guards. The TypeScript parser independently confirms there is no eager import, both consumers await the same module, and both callers await the converted methods.

The initial AST assertion failed on the eager import. Its first diagnostic printed a complete parent-linked AST; that test-owned process was terminated and the assertion was corrected to print only module strings. No unrelated process was stopped. A first runtime guard threw inside discovery's error-swallowing directory helper, so its apparent success was not accepted as proof. The corrected guard records attempted enumeration, returns no directory entries, and fails outside the imported module. It reproduced a repository-root enumeration attempt with zero passes, one failure, one assertion, in 4.33 seconds overall; no actual catalog traversal was allowed by that guard.

After deferred imports were installed, the complete packet passed two tests, zero failures, fourteen assertions, in 7.96 seconds. The actual router import completed in 575.78 milliseconds and emitted the expected `[DEBUG] router import completed without catalog enumeration` marker. That is an import-boundary result, not a full Storybook runtime or CLI performance claim.

The command used the absolute ticket test path:

```text
bun test /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️router-import-boundary.test.ts
```

Scoped `git diff --check` passed for the root router, normalizer, and the changed permanent fixture paths. No production leaf was moved by this packet. Compose remained untouched.
