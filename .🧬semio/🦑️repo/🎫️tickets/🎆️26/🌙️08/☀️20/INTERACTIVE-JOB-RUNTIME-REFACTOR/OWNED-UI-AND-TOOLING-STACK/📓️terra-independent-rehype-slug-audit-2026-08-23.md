# Independent `rehype-slug` Retirement Audit — 2026-08-23

## Verdict

**ACCEPT**

Blockers: none.

This accepts only the narrow root/UI `rehype-slug` dependency-retirement wave. It is not a Phase 10 completion or acceptance claim.

## Independent Scope Review

I read the accepted scout, the implementation report, and the live scoped diff. The live working hunk is exactly the prescribed six deletions:

1. `.storybook/main.ts` `rehypeSlug` import;
2. its one `rehypePlugins` array element;
3. the root direct-manifest row;
4. the UI React direct-manifest row;
5. the root Bun workspace tuple; and
6. the UI React Bun workspace tuple.

No permanent guard, Storybook glob, non-target source, Compose manifest, Cargo input, or P3/P8 file was changed by the packet. It adds no heading parser, slugger, anchor fallback, shim, facade, externalization, or replacement dependency. The only active Rollup external remains the pre-existing `/\\.node$/` behavior.

## Fresh Storybook And MDX Reachability Evidence

I reran `bun x nx run @semio-tech/ui-react:build --skip-nx-cache`. The fresh Nx task was a cache miss with status `0`; the permanent root guard reported `170 stories, 61 docs, 61 TS/TSX inputs, 0 MDX`.

Independent parsing of the rebuilt `storybook-static/index.json` produced:

| Invariant              |                                                             Result |
| ---------------------- | -----------------------------------------------------------------: |
| entries                |                                                                231 |
| stories                |                                                                170 |
| docs                   |                                                                 61 |
| unique inputs          |                                                          61 `.tsx` |
| docs inputs            |                                                   61 unique `.tsx` |
| docs tagged `autodocs` |                                                                 61 |
| Markdown/MDX inputs    |                                                                  0 |
| unsupported inputs     |                                                                  0 |
| SHA-256                | `72e76f1580736f6612ed36b57d8fee1b0461adf1bc9c3c25ab88fe9e83713ce4` |

That byte hash equals the reliable executor pre/post hash. The owned non-Compose/non-ticket census returned `0` `*.mdx` files; static-import, dynamic-import, and CommonJS `require` scans found no owned `.md`/`.mdx` module edge.

The installed-source review independently closes the generated-Autodocs question. `@mdx-js/rollup` calls its processor only when the VFile extension is an allowed Markdown/MDX extension. Storybook core creates an Autodocs entry by carrying forward the CSF story entry's existing `importPath`; its separate `extractDocs` route is the actual-MDX reader. The internal Storybook MDX plugin filters only `.mdx`, and root configuration removes it before adding the root Rollup plugin. Thus neither handwritten nor generated docs reach the removed HAST-heading transform.

## Verification Results

| Gate                                       | Independent result                                                                                                                    |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------- |
| Full uncached UI Storybook build           | PASS; cache miss/status 0 and exact 231/170/61/61/0 index                                                                             |
| Reliable pre/post hash comparison          | PASS; fresh SHA-256 exactly matches                                                                                                   |
| UI quick suite                             | PASS; 21 files, 724 tests                                                                                                             |
| UI lint and typecheck                      | PASS                                                                                                                                  |
| Frozen install                             | PASS; Bun 1.3.14, 1,945 installs across 1,997 packages, no changes                                                                    |
| Dependency ratchet                         | PASS; baseline 238, current 132, 106 removed, zero new                                                                                |
| JavaScript direct list                     | PASS; 69 rows, `rehype-slug` absent                                                                                                   |
| Rust direct list                           | PASS; 63 rows                                                                                                                         |
| JS dependency parity                       | PASS; 83 manifests, 250 external rows, 105 evidenced, 145 unowned, 0 undeclared imports, 44 lock workspaces, 0 mismatches, 5 fixtures |
| Root/UI source and direct-manifest absence | PASS; zero `rehype-slug` / `rehypeSlug` references or rows                                                                            |
| Owned MDX/module-edge proof                | PASS; 0 files, zero static/dynamic edges, zero CommonJS edges                                                                         |
| Root script syntax                         | PASS; one-module `bun build` with externals                                                                                           |
| Scoped working/staged/HEAD diff checks     | PASS; all zero                                                                                                                        |
| Whole working/staged/HEAD diff checks      | PASS; all zero                                                                                                                        |

No Cargo command was run.

## Required Retentions

Root/UI removal correctly preserves exactly three out-of-scope Compose direct owners:

- `compose/client/lib/sketchpad/js/package.json:35`
- `compose/client/ui/vscode/package.json:62`
- `compose/dev/algorithm/package.json:33`

Their matching Bun tuples remain at lines 139, 247, and 276, and the one shared `rehype-slug@6.0.0` resolution remains at line 3699. Root/UI package parsing found no direct row in any dependency section.

The non-goals are intact: `rehype-autolink-headings` remains both root/UI-declared and actively registered; `@mdx-js/rollup` remains root/UI-declared and active; UI `dagre` remains declared; and the permanent zero-MDX/exact-index guard remains unchanged and passed.

## Formatter And Concurrent Tree Baseline

The two changed manifests pass Prettier. The combined scoped formatter command reports pre-existing shared-file drift in `.storybook/main.ts` and concurrent root `📜️script.ts`. I inspected the `.storybook/main.ts` formatter delta: it is only the pre-existing long `optimizeExclude` and `.node` externalization lines, not this import/array deletion. No bulk formatting was done.

The shared working tree contains concurrent P3/P8 and prior dependency changes. Scoped and whole working/staged/HEAD `git diff --check` each passed. I did not stage, unstage, reset, commit, or otherwise mutate Git state.

## Commands Run

```text
bun x nx run @semio-tech/ui-react:build --skip-nx-cache
bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache
bun x nx run @semio-tech/ui-react:lint --skip-nx-cache
bun x nx run @semio-tech/ui-react:typecheck --skip-nx-cache
bun install --frozen-lockfile
bun ./📜️script.ts verify dependencies
bun ./📜️script.ts verify dependencies list js --format json
bun ./📜️script.ts verify dependencies list rust --format json
bun ./📜️script.ts verify dependencies parity js
find . -type f -name '*.mdx' ! -path './node_modules/*' ! -path './compose/*' ! -path './.🧬semio/*' ! -path './.git/*' -print | wc -l
rg --hidden -n -P "\b(?:from\s+|import\s*\()['\"][^'\"]+\.mdx?['\"]" --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!compose/**' --glob '!.🧬semio/**' --glob '!.git/**' .
rg --hidden -n -P "\brequire\s*\(['\"][^'\"]+\.mdx?['\"]\)" --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!compose/**' --glob '!.🧬semio/**' --glob '!.git/**' .
bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'
bunx prettier --check .storybook/main.ts package.json '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json' 📜️script.ts
git diff --check
git diff --cached --check
git diff HEAD --check
```

No production fix, manifest/lock change, ticket lifecycle operation, Git mutation, or Cargo invocation was performed by this audit.
