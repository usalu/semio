# Independent `remark-frontmatter` Retirement Audit — 2026-08-23

Verdict: ACCEPT

Blockers: none.

This is acceptance of one narrow dependency-retirement wave only, not Phase 10 acceptance.

## Scope Audited

I read the accepted scout `📓️terra-next-accepted-remark-frontmatter-scout-2026-08-23.md`, the implementation report `📓️p10-owned-remark-frontmatter-retirement-2026-08-23.md`, and inspected the live worktree/index/HEAD diffs without trusting their conclusions.

The operational wave removes the one root Storybook `remarkFrontmatter` import and its one `remarkPlugins` binding, plus the two direct owner rows (root and `@semio-tech/ui-react`). `bun.lock` removes exactly the corresponding two non-Compose workspace tuples. The root discovery guard's wording was generalized from a particular retired package to retired owned MDX transforms; its constants, walk, build invocation, and enforcement are retained.

The combined HEAD diff includes separately staged predecessor work: removal of `remark-mdx-frontmatter`, the Flow-WASM build repair, other already-scoped dependency waves, and concurrent Phase-8/root-script work. I assessed the current frontmatter delta separately from those overlays.

## Independent Functional Gates

| Command                                                                   | Result                                                                                                |
| ------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `bun x nx run @semio-tech/ui-react:build --skip-nx-cache`                 | PASS. Permanent guard: `170 stories, 61 docs, 61 TS/TSX inputs, 0 MDX`.                               |
| Independent `storybook-static/index.json` parse                           | PASS: `231 = 170 stories + 61 docs`; 61 unique inputs, all `.tsx`, zero MDX.                          |
| `bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache`            | PASS: 21 files, 724 tests. Nx emitted its flaky-task notice after the successful run; no test failed. |
| `bun x nx run @semio-tech/ui-react:lint --skip-nx-cache`                  | PASS.                                                                                                 |
| `bun x nx run @semio-tech/ui-react:typecheck --skip-nx-cache`             | PASS.                                                                                                 |
| `bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'` | PASS: one module bundled.                                                                             |

The full build retains pre-existing non-fatal Vite/docgen/static-asset warnings. It completed successfully and the guard executed after the Storybook build.

## Ownership, Source, Manifest, and Lock Proof

The prescribed static/dynamic `.md`/`.mdx` import scans returned zero owned edges (no output from either `from`/dynamic-`import` or CommonJS `require` pattern, with `node_modules`, `compose`, tickets, and Git excluded).

`rg -n 'remark-frontmatter|remarkFrontmatter' .storybook package.json <ui-package-manifest>` returned no match. Parsed root and UI manifests have no `remark-frontmatter` property in any object-valued dependency section. The JS inventory also reported `js=71 remark-frontmatter=false`; Rust is `63`.

The retained Compose contract is exact:

| Location                                       | Section/value             |
| ---------------------------------------------- | ------------------------- |
| `compose/client/lib/sketchpad/js/package.json` | `devDependencies: ^5.0.0` |
| `compose/client/ui/vscode/package.json`        | `devDependencies: ^5.0.0` |
| `compose/dev/algorithm/package.json`           | `devDependencies: ^5.0.0` |

The lock has exactly three matching workspace tuples and exactly one `remark-frontmatter@5.0.0` resolution. No Compose row was changed.

## Dependency Gates

| Command                                                         | Result                                                                                                                                 |
| --------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `bun install --frozen-lockfile`                                 | PASS: 1,945 installs across 1,997 packages; no changes.                                                                                |
| `bun ./📜️script.ts verify dependencies`                         | PASS: baseline 238; current 134; 104 removed; no new dependency.                                                                       |
| `bun ./📜️script.ts verify dependencies list js --format json`   | PASS: 71 JS rows; target absent.                                                                                                       |
| `bun ./📜️script.ts verify dependencies list rust --format json` | PASS: 63 Rust rows.                                                                                                                    |
| `bun ./📜️script.ts verify dependencies parity js`               | PASS: 83 manifests, 254 external rows, 107 evidenced, 147 unowned, 0 undeclared imports, 44 lock workspaces, 0 mismatches, 5 fixtures. |

Thus the accepted boundary is exactly `134 = 71 JS + 63 Rust`.

## No Substitute or Concealed Runtime Behavior

The active Storybook MDX configuration has `remarkPlugins: [remarkGfm]`: it has no frontmatter plugin, facade, replacement implementation, or package-specific externalization. The existing Rollup external is only `/\\.node$/` and was not changed by this wave. The Vite built-in-module externalization warnings are unrelated existing UI-source behavior and contain no target-package reference.

The Flow-WASM dev resolver visible in the staged predecessor delta is not a frontmatter substitute: it resolves an existing WASM package file first, falling back only for absent development artifacts, and has targeted tests beside its implementation. It neither imports nor names `remark-frontmatter`; the fresh Storybook build emitted real Flow WASM assets. Its prior repair is therefore not a compatibility layer for this retirement.

## Formatting and Diff Hygiene

`prettier --check` passes both touched manifests. It reports `.storybook/main.ts` and `📜️script.ts`; the same two failures occur when their exact HEAD contents are piped to Prettier, so this is an honest shared baseline rather than a newly introduced failure.

All of the following were run:

```text
git diff --check                         PASS
git diff HEAD --check                    PASS
git diff --check -- <five scoped paths>  PASS
git diff --cached --check -- <scope>     PASS
git diff HEAD --check -- <scope>         PASS
```

The sole exception is whole-tree `git diff --cached --check`, which reports exactly two trailing spaces in the stale staged predecessor report `📓️p10-owned-remark-mdx-frontmatter-retirement-2026-08-23.md` (its date and owner-boundary lines). The live working file has already removed them, the combined HEAD check passes, and the checked frontmatter production scope is clean in working, staged, and HEAD comparisons. It is not a production change or a blocker for this wave; I neither staged nor altered it.

No production files, manifests, lockfiles, Git state, or ticket lifecycle were changed by this audit. For transparency, I initially invoked the broad repository `bun ./📜️script.ts test quick` wrapper before resolving the documented UI target; it delegated to unrelated Compose/Rust work and failed. It is excluded from the verdict. The required narrow UI quick target was then rerun and passed 724/724; no direct Cargo command was issued.
