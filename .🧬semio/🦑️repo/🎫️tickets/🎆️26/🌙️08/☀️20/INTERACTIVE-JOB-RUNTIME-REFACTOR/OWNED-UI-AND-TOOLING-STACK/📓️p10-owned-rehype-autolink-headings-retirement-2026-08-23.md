# Owned `rehype-autolink-headings` Retirement — 2026-08-23

## Outcome

The narrow Terra-scoped root/UI `rehype-autolink-headings` retirement is implemented. The active
Storybook import and sole `rehypePlugins` binding are gone, the two direct root/UI manifest rows are
gone, and Bun reconciliation removed only the matching two root/UI workspace tuples. The three
excluded Compose tuples and one shared `rehype-autolink-headings@7.1.0` resolution remain.

This report covers only this single dependency wave. It does not claim Phase 10 completion or alter
the accepted Compose, Dagre, Phase 3, or Phase 8 boundaries.

## Pre-Edit Baseline And Reachability

The full uncached UI Storybook build completed successfully before any edit. The permanent discovery
guard and an independent parse of `storybook-static/index.json` agreed on the exact baseline:

| Invariant              |                                                    Pre-edit result |
| ---------------------- | -----------------------------------------------------------------: |
| Entries                |                                                                231 |
| Stories                |                                                                170 |
| Docs                   |                                                                 61 |
| Unique inputs          |                                                          61 `.tsx` |
| Docs inputs            |                                                   61 unique `.tsx` |
| Docs tagged `autodocs` |                                                                 61 |
| Markdown/MDX inputs    |                                                                  0 |
| Unsupported inputs     |                                                                  0 |
| SHA-256                | `72e76f1580736f6612ed36b57d8fee1b0461adf1bc9c3c25ab88fe9e83713ce4` |

The owned non-Compose/non-ticket census found zero `*.mdx` files. Static imports, dynamic imports,
and CommonJS `require` scans found zero owned `.md`/`.mdx` module edges.

Installed-source inspection reproduced the accepted reachability proof:

- `@mdx-js/rollup/lib/index.js` creates a VFile but invokes its processor only when the module has an
  extension present in the configured Markdown/MDX `extnames` set.
- Storybook core creates generated Autodocs by copying the CSF story entry's existing `importPath`.
  The actual-MDX path is the separate `extractDocs` method.
- The installed Storybook docs MDX plugin filters only `/\.mdx$/` and is removed by the root
  `.storybook/main.ts` configuration before the root Rollup plugin is added.

Therefore all 61 generated Autodocs keep their TypeScript/TSX import path and cannot reach the
removed HAST-heading transform.

## Exact Change

The wave changed only:

1. `.storybook/main.ts`: removed the `rehypeAutolinkHeadings` import and the sole
   `rehypePlugins` option; the now-empty options object is formatted as `mdx.default({})`.
2. `package.json`: removed the root direct `rehype-autolink-headings` row.
3. `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`: removed the UI
   React direct row.
4. `bun.lock`: removed the root and UI workspace tuples through `bun install`.

No discovery guard, Storybook glob, externalization rule, production source, Compose input, Dagre
input, Cargo input, launch configuration, or P3/P8 file was changed by this wave. No shim,
replacement anchor renderer, fallback, facade, or plugin-specific guard exemption was added.

## Post-Edit Differential

The post-edit full uncached UI Storybook build also completed successfully. Its independently parsed
index is byte-identical to the pre-edit index:

| Invariant              |                                                   Post-edit result |
| ---------------------- | -----------------------------------------------------------------: |
| Entries                |                                                                231 |
| Stories                |                                                                170 |
| Docs                   |                                                                 61 |
| Unique inputs          |                                                          61 `.tsx` |
| Docs inputs            |                                                   61 unique `.tsx` |
| Docs tagged `autodocs` |                                                                 61 |
| Markdown/MDX inputs    |                                                                  0 |
| Unsupported inputs     |                                                                  0 |
| SHA-256                | `72e76f1580736f6612ed36b57d8fee1b0461adf1bc9c3c25ab88fe9e83713ce4` |

The owned MDX census and both module-edge scans remain zero.

## Verification

| Gate                                   | Result                                                                                                                                |
| -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| Pre-edit uncached UI Storybook build   | PASS; cache miss/status 0 and exact baseline                                                                                          |
| Post-edit uncached UI Storybook build  | PASS; cache miss/status 0 and byte-identical index                                                                                    |
| UI quick suite                         | PASS; 21 files, 724 tests                                                                                                             |
| UI lint                                | PASS                                                                                                                                  |
| UI typecheck                           | PASS                                                                                                                                  |
| `bun install`                          | PASS; reconciled lock                                                                                                                 |
| Frozen install                         | PASS; Bun 1.3.14, 1,945 installs across 1,997 packages, no changes                                                                    |
| Dependency ratchet                     | PASS; baseline 238, current 131, 107 removed, zero new                                                                                |
| JavaScript dependency list             | PASS; 68 identities and target absent                                                                                                 |
| Rust dependency list                   | PASS; 63 identities                                                                                                                   |
| JavaScript dependency parity           | PASS; 83 manifests, 248 external rows, 104 evidenced, 144 unowned, 0 undeclared imports, 44 lock workspaces, 0 mismatches, 5 fixtures |
| Root/UI source and manifest absence    | PASS; zero target package/symbol references                                                                                           |
| Compose lock retention                 | PASS; exactly 3 workspace tuples and 1 shared `7.1.0` resolution                                                                      |
| Root script syntax                     | PASS; one-module Bun build with all imports externalized                                                                              |
| Manifest formatting                    | PASS; both changed manifests match Prettier                                                                                           |
| Formatter baseline                     | PRESERVED; only pre-existing `.storybook/main.ts` long-line drift and concurrent root `📜️script.ts` drift remain                      |
| Scoped working/staged/HEAD diff checks | PASS                                                                                                                                  |
| Whole working diff check               | PASS                                                                                                                                  |
| Whole HEAD diff check                  | PASS after concurrent owner repaired its working-tree line                                                                            |
| Whole staged diff check                | STALE INDEX BLOCKER; the index still contains unrelated trailing whitespace in `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md:25`               |

The concurrent owner repaired that line in the working tree, making the whole working and HEAD
checks clean. The staged snapshot remains stale; staging is prohibited for this executor, and the
file is outside this packet. The target wave introduces no whitespace error.

## Required Retentions

The excluded Compose owners and their matching Bun workspace tuples remain:

- `compose/client/lib/sketchpad/js/package.json`
- `compose/client/ui/vscode/package.json`
- `compose/dev/algorithm/package.json`

The single shared `rehype-autolink-headings@7.1.0` lock resolution remains for those owners.
`@mdx-js/rollup` remains declared at root/UI and active in `.storybook/main.ts`; Storybook packages,
Dagre, the zero-MDX discovery guard, and all unrelated rehype/tooling dependencies remain unchanged.

## Commands Run

```text
bun x nx run @semio-tech/ui-react:build --skip-nx-cache
bun install
bun x nx run @semio-tech/ui-react:build --skip-nx-cache
bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache
bun x nx run @semio-tech/ui-react:lint --skip-nx-cache
bun x nx run @semio-tech/ui-react:typecheck --skip-nx-cache
bun install --frozen-lockfile
bun ./📜️script.ts verify dependencies
bun ./📜️script.ts verify dependencies list js --format json
bun ./📜️script.ts verify dependencies list rust --format json
bun ./📜️script.ts verify dependencies parity js
find . -type f -name '*.mdx' ! -path './node_modules/*' ! -path './compose/*' ! -path './.🧬semio/*' ! -path './.git/*'
rg --hidden -n -P "\b(?:from\s+|import\s*\()['\"][^'\"]+\.mdx?['\"]" ...
rg --hidden -n -P "\brequire\s*\(['\"][^'\"]+\.mdx?['\"]\)" ...
bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'
bunx prettier --check .storybook/main.ts package.json '<ui-react>/package.json' 📜️script.ts
git diff --check
git diff --cached --check
git diff HEAD --check
```

No Cargo command, Git-modifying command, ticket lifecycle operation, or coordinator edit was made.
