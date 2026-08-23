# P10 Owned `remark-mdx-frontmatter` Retirement

Date: 2026-08-23
Owner boundary: root Storybook/tooling plus `@semio-tech/ui-react`
Decision source: accepted Terra scout `📓️terra-next-accepted-dependency-scout-after-globals-2026-08-23.md`

## Outcome

The owned root/UI Storybook boundary no longer imports, configures, or directly declares `remark-mdx-frontmatter`. Bun removed the two in-scope workspace tuple references while retaining the package resolution for the three excluded Compose manifests. The final uncached UI Storybook build has the same frozen discovery inventory as the green pre-retirement baseline: 231 entries, comprising 170 stories and 61 docs from 61 TS/TSX inputs, with zero MDX.

The root Nx-executed Storybook path now fails before building if an owned non-Compose `.mdx` file exists and fails after building if the UI index differs from the frozen 231/170/61/61 inventory or contains a non-TS/TSX input.

## Pre-Edit Census And Prerequisite Repair

The hidden pre-edit census was:

```text
find . -type f -name '*.mdx' ! -path './node_modules/*' ! -path './compose/*' ! -path './.🧬semio/*' ! -path './.git/*' -print | wc -l
0
```

The first complete uncached `bun x nx run @semio-tech/ui-react:build --skip-nx-cache` did not provide an acceptable baseline: two active UI stories imported the removed `@semio-tech/coda-desktop/renderer` owner. Retirement edits were held until the untouched-dependency build was repaired. The prerequisite corrections stayed on existing owner boundaries:

- Deleted `.storybook/stories/ui/🌳OntologyTree.stories.tsx` and `.storybook/stories/ui/✅ValidationTree.stories.tsx`; their only product owner was gone. Dormant Coda-scope stories were not brought into the active UI scope.
- Registered the existing owned `playgroundFlowWasmDevStubPlugin(repoRootPath)` in `.storybook/main.ts`, which resolves the real surface-node-graph WASM output when present and otherwise uses its existing development fallback.
- Pointed `.storybook/globals.css` at the current renderer engine `🎨️globals.css` location.
- Kept Node/Playwright infrastructure out of the browser graph: `.storybook/preview.tsx` owns its small active-scope prefix predicate instead of importing `.storybook/scopes.ts`, and the Tree story uses the UI package's already-declared `@testing-library/react` browser fixture surface plus native throwing assertions instead of `storybook/test` or the UI `./test` helper. No `kerberos` externalization, polyfill, shim, or dependency was added.
- Removed only the vanished Transaction provider section from `🔌Providers.stories.tsx`.
- Preserved Footer and Layout stories through the current generic footer-item API, replacing the removed product-specific funded-by item with a local-workspace item.

After those corrections, the full uncached build completed and established the accepted pre-retirement baseline: 231 entries = 170 stories + 61 docs, 61 TS/TSX inputs, zero MDX. A checkpoint was sent before manifest and lock edits.

## Retirement Changes

- Removed the `remark-mdx-frontmatter` import and `remarkPlugins` registration from `.storybook/main.ts`; `remark-gfm` and `remark-frontmatter` remain.
- Removed the direct dependency rows from root `package.json` and the UI React `package.json` only.
- Reconciled `bun.lock` with Bun 1.3.14. The lock now has three Compose workspace tuple rows and one shared `remark-mdx-frontmatter@5.2.0` resolution.
- Added the `🔖️StorybookDiscoveryGuard` region to the existing root `📜️script.ts`; the existing Nx build route invokes its zero-MDX scan before Storybook and exact discovery assertion afterward.
- Removed the two now-stale raw-color inventory allowlist rows for the deleted Coda-owned active UI stories. This was the only Rust-path text edit and did not run Cargo.
- Corrected the concurrently merge-corrupted `PurgeScript`/`CleanScript` region boundary comments without changing cleaner behavior.

## Exact Production File Inventory

1. `.storybook/globals.css`
2. `.storybook/main.ts`
3. `.storybook/preview.tsx`
4. `.storybook/stories/ui/✅ValidationTree.stories.tsx` (deleted)
5. `.storybook/stories/ui/🌳OntologyTree.stories.tsx` (deleted)
6. `.storybook/stories/ui/🔌Providers.stories.tsx`
7. `bun.lock`
8. `package.json`
9. `📜️script.ts`
10. `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts`
11. `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`
12. `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📐️Layout/🧪️story.tsx`
13. `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔚️Footer/🧪️story.tsx`
14. `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪵️Tree/🧪️story.tsx`

This report is the only ticket evidence file added by this packet. No Compose, Dagre, Cargo, AGENTS, launch, ticket metadata, or checklist file was changed.

## Final Verification

| Gate                                                                                                                                      | Result                                                                                                                                                                                                  |
| ----------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `find . -type f -name '*.mdx' ! -path './node_modules/*' ! -path './compose/*' ! -path './.🧬semio/*' ! -path './.git/*' -print \| wc -l` | PASS: `0` before and after                                                                                                                                                                              |
| `bun x nx run @semio-tech/ui-react:build --skip-nx-cache`                                                                                 | PASS: Storybook 10.5.6, 1603 modules; guard reports 170 stories, 61 docs, 61 TS/TSX inputs, zero MDX; independent index read reports 231 total                                                          |
| `bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache`                                                                            | PASS: 21 files, 724 tests                                                                                                                                                                               |
| `bun x nx run @semio-tech/ui-react:lint --skip-nx-cache`                                                                                  | PASS                                                                                                                                                                                                    |
| `bun x nx run @semio-tech/ui-react:typecheck --skip-nx-cache`                                                                             | PASS                                                                                                                                                                                                    |
| `bun install`                                                                                                                             | PASS with Bun 1.3.14; lock reconciled                                                                                                                                                                   |
| `bun install --frozen-lockfile`                                                                                                           | PASS: 1,945 installs across 1,997 packages, no changes                                                                                                                                                  |
| `bun ./📜️script.ts verify dependencies`                                                                                                   | PASS: baseline 238, current 135, 103 removed, zero new                                                                                                                                                  |
| `bun ./📜️script.ts verify dependencies list js --format json` compact count                                                               | PASS: 72 JavaScript dependency rows, zero `remark-mdx-frontmatter`                                                                                                                                      |
| `bun ./📜️script.ts verify dependencies list rust --format json` compact count                                                             | PASS: 63 Rust dependency rows                                                                                                                                                                           |
| `bun ./📜️script.ts verify dependencies parity js`                                                                                         | PASS: 83 manifests, 256 external rows, 108 evidenced, 148 unowned, zero undeclared imports, 44 lock workspaces, zero mismatches, 5 fixtures                                                             |
| Active non-Compose import/config scan for `remark-mdx-frontmatter` / `remarkMdxFrontmatter`                                               | PASS: no active import, dynamic import, or plugin registration                                                                                                                                          |
| Direct root/UI manifest parse                                                                                                             | PASS: absent in both manifests                                                                                                                                                                          |
| Compose manifest/lock scan                                                                                                                | PASS: three excluded Compose manifests; three matching lock workspace rows; one retained 5.2.0 resolution                                                                                               |
| `bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'`                                                                 | PASS: root script parses and bundles; the preceding bundled-dependency attempt correctly stopped on Playwright's unavailable Chromium-Bidi internal, so the syntax gate was rerun with imports external |
| Scoped `git diff --check`, `git diff --cached --check`, `git diff HEAD --check` over all 14 production paths                              | PASS: all three exit 0                                                                                                                                                                                  |
| Whole-tree `git diff --check`, `git diff --cached --check`, `git diff HEAD --check`                                                       | PASS: all three exit 0                                                                                                                                                                                  |

### Prettier Evidence

`bunx prettier --check` parsed every non-deleted parseable changed file. The combined command reported existing style drift in four shared files: `.storybook/main.ts`, root `📜️script.ts`, the styling Rust `📜️script.ts`, and the Layout story. The seven remaining changed parseable files pass together. HEAD already has the same two formatting hunks in `.storybook/main.ts`, the same two in the styling script, and the same two in the Layout story; root HEAD has 125 formatting hunks and the shared current file has 130 because of unrelated concurrent edits. The new Storybook guard hunk was manually aligned to Prettier and no longer appears in the formatter diff. Those shared files were not bulk-reformatted because that would rewrite unrelated concurrent work.

### Supplemental Styling Inventory

`bun x nx run @semio-tech/ui-styling-tokens:check-no-raw-colors --skip-nx-cache` reports 58 existing findings across unrelated active framework/styling stories and dormant Coda stories. The two removed allowlist rows name the two deleted active UI files and cannot suppress any remaining finding. No unrelated color migration was attempted.

## Absence Semantics

A broad non-Compose literal scan still sees only two intentional owned records: the permanent guard's failure diagnostic and the historical `🔒️dependencies.json` baseline row. It also sees Bun's four lock occurrences. There is no active non-Compose source import/config registration and neither in-scope manifest declares the package. The retained lock resolution is therefore Compose-owned, confirmed by the three excluded direct Compose manifests and matching three lock workspace rows rather than inferred from string absence alone.

## Concurrency Notes

The worktree contained extensive staged and unstaged concurrent work. No staging, commit, checkout, stash, reset, worktree, or other git-modifying operation was used. Scoped and whole-tree diff checks were run against working, staged, and HEAD views. The coordinator-reported root-script region comment corruption was repaired only at its two boundary comments; all concurrent cleaner content was preserved.
