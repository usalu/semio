# P10 Owned `remark-frontmatter` Retirement

Date: 2026-08-23

Owner boundary: root Storybook/tooling plus `@semio-tech/ui-react`

Decision source: accepted Terra scout `📓️terra-next-accepted-remark-frontmatter-scout-2026-08-23.md`

## Outcome

The root/UI boundary no longer imports, registers, or directly declares `remark-frontmatter`. The existing Storybook MDX Rollup path retains `remark-gfm`, `rehype-slug`, and `rehype-autolink-headings`; no replacement parser, facade, fallback, copied behavior, externalization, or dependency was added. Compose was not changed.

The direct dependency boundary is now **134 = 71 JavaScript + 63 Rust**. Bun retains exactly three `remark-frontmatter` workspace tuples for the excluded Compose manifests and one shared `remark-frontmatter@5.0.0` resolution.

This is a narrow implementation result only. It is not a Phase 10 acceptance claim.

## Pre-Edit Differential

Before dependency edits, the complete uncached command `bun x nx run @semio-tech/ui-react:build --skip-nx-cache` completed successfully. The permanent guard reported exactly **170 stories, 61 docs, 61 TS/TSX inputs, 0 MDX**; the resulting index therefore contained **231** entries. The non-Compose/non-ticket `*.mdx` census was `0`, and both prescribed static/dynamic import and CommonJS `require` scans for owned `.md`/`.mdx` module edges returned no matches.

The pre-edit baseline matched the accepted scout exactly, so implementation proceeded.

## Changes

1. `.storybook/main.ts`
   - Removed the sole `remarkFrontmatter` import.
   - Removed the sole `remarkFrontmatter` entry from `remarkPlugins` while retaining `remarkGfm`.
2. `package.json`
   - Removed only the direct root `remark-frontmatter` row.
3. `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`
   - Removed only the direct UI React `remark-frontmatter` row.
4. `bun.lock`
   - Reconciled with `bun install`; only the root and UI workspace tuples lost `remark-frontmatter` for this packet.
5. `📜️script.ts`
   - Generalized the existing Storybook discovery guard's text-only diagnostics from the prior single-plugin wording to all retired owned MDX transforms, as allowed by the scout. Scan and index behavior remain unchanged.
6. This report.

No Compose manifest, Cargo input, Dagre binding, `remark-gfm`, `@mdx-js/rollup`, rehype plugin, Storybook glob, launch configuration, coordinator report/list, ticket lifecycle state, or git state was changed by this packet.

## Final Verification

| Command / proof | Result |
| --- | --- |
| Owned `*.mdx` census before and after | PASS: `0` |
| Prescribed `.md/.mdx` import/dynamic-import scan before and after | PASS: no matches |
| Prescribed `.md/.mdx` CommonJS `require` scan before and after | PASS: no matches |
| `bun x nx run @semio-tech/ui-react:build --skip-nx-cache` before and after | PASS: Storybook 10.5.6; exact guard result `170 stories, 61 docs, 61 TS/TSX inputs, 0 MDX` |
| Independent final `storybook-static/index.json` read | PASS: `231 = 170 stories + 61 docs`, 61 unique inputs, zero unsupported inputs |
| `bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | PASS: 21 files, 724 tests |
| `bun x nx run @semio-tech/ui-react:lint --skip-nx-cache` | PASS |
| `bun x nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | PASS |
| `bun install` | PASS: Bun 1.3.14; saved reconciled lock; 1,945 installs across 1,997 packages, no install changes |
| `bun install --frozen-lockfile` | PASS: 1,945 installs across 1,997 packages, no changes |
| `bun ./📜️script.ts verify dependencies` | PASS: baseline 238, current 134, 104 removed, zero new |
| `bun ./📜️script.ts verify dependencies list js --format json` | PASS: 71 rows; `remark-frontmatter` absent |
| `bun ./📜️script.ts verify dependencies list rust --format json` | PASS: 63 rows |
| `bun ./📜️script.ts verify dependencies parity js` | PASS: 83 manifests, 254 external rows, 107 evidenced, 147 unowned, 0 undeclared imports, 44 lock workspaces, 0 mismatches, 5 fixtures |
| Root/UI source and manifest absence | PASS: no active binding; parsed manifests absent in all dependency sections |
| Compose and lock semantics | PASS: 3 Compose manifest rows, 3 matching lock tuples, 1 retained 5.0.0 resolution |
| `bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'` | PASS: bundled one module in 18 ms |
| Scoped working/staged/HEAD `git diff --check` | PASS: all zero exit status |
| Whole-tree working/staged/HEAD `git diff --check` | PASS: all zero exit status |

### Formatter And Existing Diagnostics

The exact prescribed combined `bunx prettier --check` parsed all four scoped files and retained the accepted shared-file baseline: `.storybook/main.ts` and root `📜️script.ts` report existing style drift. Both changed manifests pass Prettier together. Formatter diffs contain no retirement import/plugin hunk and no generalized guard-diagnostic hunk, so this packet does not add formatting drift. The shared files were not bulk-formatted because that would rewrite concurrent work.

The Storybook build continued to expose its existing CSS selector, unresolved runtime asset/font, docgen, browser-externalization, and chunk-size warnings. They were not hidden or treated as new failures. The previously recorded raw-color baseline is also unchanged by scope; no raw-color file was edited, and no additional Nx target was run because this packet was restricted to the exact UI Nx gates.

## Commands Run

```text
find . -type f -name '*.mdx' ! -path './node_modules/*' ! -path './compose/*' ! -path './.🧬semio/*' ! -path './.git/*' -print | wc -l
rg -n -P "\b(?:from\s+|import\s*\()['\"][^'\"]+\.mdx?['\"]" --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!compose/**' --glob '!.🧬semio/**' --glob '!.git/**' .
rg -n -P "\brequire\s*\(['\"][^'\"]+\.mdx?['\"]\)" --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!compose/**' --glob '!.🧬semio/**' --glob '!.git/**' .
bun x nx run @semio-tech/ui-react:build --skip-nx-cache
bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache
bun x nx run @semio-tech/ui-react:lint --skip-nx-cache
bun x nx run @semio-tech/ui-react:typecheck --skip-nx-cache
bun install
bun install --frozen-lockfile
bun ./📜️script.ts verify dependencies
bun ./📜️script.ts verify dependencies list js --format json
bun ./📜️script.ts verify dependencies list rust --format json
bun ./📜️script.ts verify dependencies parity js
rg -n 'remark-frontmatter|remarkFrontmatter' .storybook package.json '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json'
rg -n 'remark-frontmatter' compose/client/lib/sketchpad/js/package.json compose/client/ui/vscode/package.json compose/dev/algorithm/package.json bun.lock
bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'
bunx prettier --check .storybook/main.ts package.json '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json' 📜️script.ts
git diff --check
git diff --cached --check
git diff HEAD --check
```

No Cargo command, extra Nx target, ticket lifecycle operation, git-modifying command, staging action, or worktree operation was used.
