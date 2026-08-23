# P10 `remark-mdx-frontmatter` Independent Audit — 2026-08-23

## Verdict

**ACCEPT** — no blocker found. This is a narrow acceptance of the owned root/UI dependency wave only; it is **not Phase 10 acceptance**.

## Scope And Concurrent State

The governing plan, accepted Terra scout, P10 implementation report, and repository instructions were read before validation. The audit inspected the declared 14-file P10 inventory:

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

The shared tree also contains concurrent dependency-wave changes in the same manifests and lockfile (`globals`, `i18next-browser-languagedetector`, `react-router`, `pixelmatch`, `pngjs`) and a large unrelated `📜️script.ts` delta. They were not attributed to this acceptance. Whole-tree checks below nevertheless used the live shared state.

## Retirement Evidence

- `.storybook/main.ts` no longer imports or registers `remark-mdx-frontmatter`; the active `@mdx-js/rollup` pipeline retains only `remark-gfm` and `remark-frontmatter`.
- Parsed root and UI React manifests both report `remark-mdx-frontmatter: absent` in every dependency section.
- A non-Compose/non-ticket literal scan has only the intentional dependency-baseline record, permanent guard diagnostic, and Bun lock rows. There is no active source import, dynamic import, or plugin registration.
- The owned MDX census is `0`:

  ```text
  find . -type f -name '*.mdx' ! -path './node_modules/*' ! -path './compose/*' ! -path './.🧬semio/*' ! -path './.git/*' -print | wc -l
  0
  ```

- `bun.lock` retains exactly three Compose workspace tuple rows and one `remark-mdx-frontmatter@5.2.0` resolution. The three direct retained owners are `compose/client/lib/sketchpad/js/package.json`, `compose/client/ui/vscode/package.json`, and `compose/dev/algorithm/package.json`.
- The permanent root `StorybookDiscoveryGuard` is present, runs before the build MDX path and after the build index read, and its extracted region is byte-identical to Prettier output. The uncached build emitted its successful guard message.

## Prerequisite Repairs

The deleted tree stories each imported only `@semio-tech/coda-desktop/renderer`, which is no longer an active UI product owner. Removing them is a legitimate current-source repair, not an MDX behavior workaround. The removed styling allowlist entries name exactly those two deleted files.

The other source adjustments also match current owners: the CSS import points to the present renderer-engine globals location; the removed Transaction provider story referred to removed UI API; Footer/Layout use the current generic footer-item model; and the Tree interaction fixture uses the UI package's already-declared `@testing-library/react` surface with native throwing assertions.

`playgroundFlowWasmDevStubPlugin(repoRootPath)` is an existing owned build fallback (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️vite-elements-assets.ts:191`), already used elsewhere in that source. Its registration repairs Storybook's current missing-WASM-artifact path and lets real output resolve when available. It is not a new MDX/frontmatter compatibility layer, externalization, shim, or dependency. No MDX parser/facade/copy was added; manifests only remove the selected direct identity, and `.storybook/main.ts` adds no externalization rule (its pre-existing `.node` rule is unchanged).

## Re-run Gates

| Command | Observed result |
| --- | --- |
| `bun x nx run @semio-tech/ui-react:build --skip-nx-cache` | PASS. Storybook 10.5.6 completed; permanent guard: `170 stories, 61 docs, 61 TS/TSX inputs, 0 MDX`. Independent `storybook-static/index.json` read: `231 = 170 stories + 61 docs`, `61` unique inputs, no unsupported paths. Build emitted existing CSS/font/chunk-size warnings only. |
| `bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache` | PASS: 21 files, 724 tests. |
| `bun x nx run @semio-tech/ui-react:lint --skip-nx-cache` | PASS. |
| `bun x nx run @semio-tech/ui-react:typecheck --skip-nx-cache` | PASS. |
| `bun install --frozen-lockfile` | PASS: Bun 1.3.14, 1,945 installs across 1,997 packages, no changes. |
| `bun ./📜️script.ts verify dependencies` | PASS: baseline 238, current 135, 103 removed, zero new. |
| JS list JSON compact count | PASS: 72 rows, `remark-mdx-frontmatter` absent. |
| Rust list JSON compact count | PASS: 63 rows. |
| `bun ./📜️script.ts verify dependencies parity js` | PASS: 83 manifests, 256 external rows, 108 evidenced, 148 unowned, 0 undeclared imports, 44 lock workspaces, 0 mismatches, 5 fixtures. |
| `bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'` | PASS: bundled one module in 19 ms. |
| Scoped working/staged/HEAD `git diff --check` | PASS: all zero exit status. |
| Whole-tree working/staged/HEAD `git diff --check` | PASS: all zero exit status. |

## Formatter And Raw-Color Baselines

Relevant `bunx prettier --check` reports four pre-existing shared-file failures: `.storybook/main.ts`, root `📜️script.ts`, the styling `📜️script.ts`, and the Layout story. The same four HEAD versions also fail the exact stdin Prettier checks. The seven other parseable P10 files pass together. The added discovery-guard region itself is Prettier-identical, so this does not regress formatting and is not a P10 blocker; no broad concurrent-file rewrite was performed.

`bun x nx run @semio-tech/ui-styling-tokens:check-no-raw-colors --skip-nx-cache` exits nonzero with exactly **58** known raw-color findings. This is an honest pre-existing baseline, not a passing gate. Neither deleted P10 story is in its output. The only styling-script change deletes the two allowlist paths for exactly those deleted files, so it cannot suppress a surviving finding or introduce a new one. No raw-color finding was added or regressed by this retirement.

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
bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'
bunx prettier --check <relevant parseable P10 files>
bun x nx run @semio-tech/ui-styling-tokens:check-no-raw-colors --skip-nx-cache
git diff --check; git diff --cached --check; git diff HEAD --check
```

No Cargo command, ticket lifecycle operation, git-modifying command, staging action, or implementation change was performed by this audit.
