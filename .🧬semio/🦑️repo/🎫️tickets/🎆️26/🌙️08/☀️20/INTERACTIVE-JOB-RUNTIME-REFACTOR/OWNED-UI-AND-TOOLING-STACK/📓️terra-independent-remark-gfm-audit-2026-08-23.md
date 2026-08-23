# Independent `remark-gfm` Retirement Audit — 2026-08-23

## Verdict

**ACCEPT**

Blockers: none.

This accepts only the narrow `remark-gfm` root/UI dependency-retirement wave. It is not a Phase 10 completion or acceptance claim.

## Scope And Diff Attribution

I independently read the accepted Terra scout, the implementation report, and the live diff. The current working packet removes exactly six `remark-gfm` references:

1. the `.storybook/main.ts` import;
2. the one `remarkPlugins: [remarkGfm]` registration;
3. the root manifest row;
4. the UI React manifest row;
5. the root Bun workspace tuple; and
6. the UI React Bun workspace tuple.

The permanent `StorybookDiscoveryGuard` in root `📜️script.ts` was not changed by this wave. The shared working tree is concurrently dirty, including P3/P8 Rust and earlier Storybook/dependency waves. I did not stage, reset, amend, or otherwise modify any concurrent file. The scoped and whole working, staged, and HEAD-combined `git diff --check` commands all exited zero, so there is no whitespace blocker in any of those snapshots.

The removal adds no parser replacement, GFM facade, compatibility shim, copied behavior, or dependency externalization. `.storybook/main.ts` retains the existing Rollup external handling for only `/\\.node$/`; neither `remark-gfm` nor a substitute is externalized.

## Fresh Runtime And Index Evidence

I reran:

```text
bun x nx run @semio-tech/ui-react:build --skip-nx-cache
```

The fresh Nx run record is a cache miss with status `0`. It rebuilt Storybook and the permanent guard accepted exactly `170 stories, 61 docs, 61 TS/TSX inputs, 0 MDX`.

I independently parsed the resulting `storybook-static/index.json`:

| Invariant              |     Fresh result |
| ---------------------- | ---------------: |
| total entries          |              231 |
| stories                |              170 |
| docs                   |               61 |
| unique import inputs   |               61 |
| input extensions       |      `.tsx` only |
| docs inputs            | 61 unique `.tsx` |
| docs tagged `autodocs` |               61 |
| Markdown/MDX inputs    |                0 |
| unsupported inputs     |                0 |

The executor recorded reliable byte-for-byte pre/post index evidence. The fresh index SHA-256 is `72e76f1580736f6612ed36b57d8fee1b0461adf1bc9c3c25ab88fe9e83713ce4`, exactly the recorded pre- and post-edit hash; the hash comparison is therefore valid, not inferred.

The independent owned, non-Compose, non-ticket census returned `0` `*.mdx` files. Static-import, dynamic-import, and CommonJS-`require` scans for owned `.md`/`.mdx` module edges returned no matches. This confirms that removal still has no reachable owned MDX processor input, including generated Autodocs.

## Required Gates

| Gate                                                      | Independent result                                                                                                                    |
| --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| Full uncached UI Storybook build                          | PASS; cache-miss status 0 and exact permanent guard/index above                                                                       |
| Current index hash vs reliable executor pre/post evidence | PASS; identical SHA-256                                                                                                               |
| `@semio-tech/ui-react:test-quick --skip-nx-cache`         | PASS; 21 files, 724 tests                                                                                                             |
| UI lint                                                   | PASS                                                                                                                                  |
| UI typecheck                                              | PASS                                                                                                                                  |
| `bun install --frozen-lockfile`                           | PASS; Bun 1.3.14, 1,945 installs across 1,997 packages, no changes                                                                    |
| Dependency ratchet                                        | PASS; baseline 238, current 133, 105 removed, zero new                                                                                |
| JavaScript direct list                                    | PASS; 70 rows, `remark-gfm` absent                                                                                                    |
| Rust direct list                                          | PASS; 63 rows                                                                                                                         |
| JavaScript manifest/source/lock parity                    | PASS; 83 manifests, 252 external rows, 106 evidenced, 146 unowned, 0 undeclared imports, 44 lock workspaces, 0 mismatches, 5 fixtures |
| Root/UI manifest and active source absence                | PASS; all direct dependency sections and `.storybook` source have zero `remark-gfm` / `remarkGfm` references                          |
| Owned Markdown/MDX file and module-edge scans             | PASS; 0 / zero static-dynamic / zero CommonJS                                                                                         |
| Root-script syntax                                        | PASS; `bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'` bundled one module                                    |
| Scoped working/staged/HEAD `git diff --check`             | PASS; all zero                                                                                                                        |
| Whole working/staged/HEAD `git diff --check`              | PASS; all zero                                                                                                                        |

No Cargo command was run.

## Retention And Non-Regression Checks

The three excluded Compose manifests remain the sole direct owners:

- `compose/client/lib/sketchpad/js/package.json:37`
- `compose/client/ui/vscode/package.json:64`
- `compose/dev/algorithm/package.json:35`

Each retains `remark-gfm: ^4.0.1`; matching Bun workspace tuples remain at lines 142, 250, and 279. `bun.lock:3705` retains exactly one shared `remark-gfm@4.0.1` resolution. Root and UI package parsing found no `remark-gfm` in any direct dependency section.

The required non-goals are intact:

- `@mdx-js/rollup` remains declared in root and UI and remains the active Storybook MDX integration.
- `rehype-slug` and `rehype-autolink-headings` remain declared in root/UI and registered in `.storybook/main.ts`.
- UI `dagre` remains declared unchanged.
- The root discovery guard’s input scan and exact `231 / 170 / 61 / 61` behavior are still active and passed in the fresh uncached build.

## Formatter Baseline

The two changed manifests pass `bunx prettier --check`. The combined configuration check reports existing formatting drift in shared `.storybook/main.ts` and concurrent root `📜️script.ts`. I inspected the `.storybook/main.ts` formatter delta: it is confined to pre-existing long `optimizeExclude` and `.node` externalization lines, not the deleted `remark-gfm` import/plugin hunk. The root-script formatter delta is unrelated concurrent work. Neither file was bulk-formatted, so this wave introduces no formatter regression and preserves others’ work.

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
rg --hidden -n -P "\\b(?:from\\s+|import\\s*\\()['\"][^'\"]+\\.mdx?['\"]" --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!compose/**' --glob '!.🧬semio/**' --glob '!.git/**' .
rg --hidden -n -P "\\brequire\\s*\\(['\"][^'\"]+\\.mdx?['\"]\\)" --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!compose/**' --glob '!.🧬semio/**' --glob '!.git/**' .
bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'
bunx prettier --check .storybook/main.ts package.json '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json' 📜️script.ts
git diff --check
git diff --cached --check
git diff HEAD --check
```

No production fix, manifest/lock edit, ticket lifecycle operation, Git mutation, staging action, worktree operation, or Cargo invocation was performed by this audit.
