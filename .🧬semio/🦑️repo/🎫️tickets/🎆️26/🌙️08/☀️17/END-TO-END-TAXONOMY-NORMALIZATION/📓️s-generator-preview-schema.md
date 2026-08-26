# S-GENERATOR-PREVIEW-SCHEMA

## Outcome

Taxonomy v7 now requires every `owned` generator contract to declare the exact owner-project `<project>:preview-generated` Nx target. External contracts forbid `previewTarget` entirely. Discovery exposes only the derived `bun nx run <previewTarget>` command and validates the owner project route as `nx:run-commands`, the exact owner cwd, and `bun ./📜️script.ts preview-generated`.

All 14 owned contracts declare the target; all four external contracts omit it. The workspace continues to resolve 183 Nx projects.

## TDD Evidence

The focused tests were introduced before the implementation and initially failed because `generatorNxPreviewCommand` did not exist. After adding the schema and validator:

```text
bun test …/🧪️index.test.ts --test-name-pattern='generator preview targets|read-only preview target'
2 pass; 215 filtered; 0 fail; 34 expect() calls; 4.02s
```

The negative contract proves that a missing owned preview, an external preview, and a non-canonical preview name all fail schema validation. The live-workspace assertion proves every declared target exists and routes to the existing owner script without an alternate command surface.

## Touched Paths

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`

The normalization loader still needs to consume the frozen manifest and construct exact regeneration records; that is the engine handoff, not a schema fallback.
