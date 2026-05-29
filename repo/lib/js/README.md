# @repo/lib (JS)

Repo policy lint facade and GraphQL CLI subprocess helpers.

## Policy scripts (`script.ts` only)

- **File**: `export const policyFile = "index.ts"` plus `export const policy = defineLint(...)` in the bundle `script.ts`.
- **Folder / bundle / technology**: `export const policy = defineLint(...)` in `script.ts` at that directory — runner resolves entity kind from `folder(path)` GraphQL.

Run:

```bash
bun path/to/script.ts policy
bun repo/lib/js/bin/lint.ts path/to/script.ts
```

Nx registers `./repo/lib/js/nx-plugin.mjs`, which matches `**/script.ts` that export `policy` and adds cacheable `breach-*` targets (`bun "<script.ts>" policy`).
