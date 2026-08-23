# Owned `rehype-slug` Retirement — 2026-08-23

## Outcome

The accepted narrow Terra packet is implemented. The sole active non-Compose Storybook `rehypeSlug` import and plugin registration are removed, as are only the root and UI React direct manifest rows and their two Bun workspace tuples. The excluded Compose boundary retains exactly three direct rows, three lock workspace tuples, and the one shared `rehype-slug@6.0.0` resolution.

This is one direct-dependency retirement under the open Owned UI and Tooling Stack ticket. It is not a Phase 10 completion or acceptance claim.

## Governing Inputs Read Completely

- root `AGENTS.md`
- `🧰️framework/🔨️modules/🖱️ui/AGENTS.md`
- master `📋️master.md`
- `📓️coordinator-dependency-boundary-2026-08-22.md`
- `📝️coordinator-current-js-dependencies.txt`
- `📓️terra-next-accepted-rehype-slug-scout-2026-08-23.md`

The repo MCP goal and ticket tool surface was not exposed in this session. Execution stayed inside the existing master ticket as directed. No ticket lifecycle, goal lifecycle, Cargo, Compose, coordinator report, Git index, Git branch, or Git commit operation was performed.

## Exact Changed Inventory

| File                                                                                | Packet-owned change                                                                              |
| ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| `.storybook/main.ts`                                                                | Removed `rehypeSlug` import and its one `rehypePlugins` array element only.                       |
| `package.json`                                                                      | Removed the root direct `"rehype-slug": "^6.0.0"` row only.                                    |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json` | Removed the UI React direct row only.                                                            |
| `bun.lock`                                                                          | `bun install` removed only the matching root and UI React workspace tuples.                      |
| this report                                                                         | Recorded the implementation, reachability proof, exact differential, and verification evidence. |

The permanent `StorybookDiscoveryGuard` in `📜️script.ts` was not changed by this packet. `rehype-autolink-headings`, `@mdx-js/rollup`, the Compose boundary, Dagre, Storybook packages/globs, externalization, and guard behavior remain intact. No heading parser, slugger, fallback, facade, shim, or replacement dependency was added. Concurrent P3/P8 and shared tooling work was preserved untouched.

## Pre-Edit Stop-Condition Baseline

Before any edit, the exact input scans produced:

| Invariant                                                                        | Pre-edit result |
| -------------------------------------------------------------------------------- | --------------: |
| owned `*.mdx` files, excluding `node_modules`, Compose, ticket metadata, and Git |               0 |
| static `.md`/`.mdx` import edges                                                 |               0 |
| CommonJS `.md`/`.mdx` require edges                                              |               0 |

`bun x nx run @semio-tech/ui-react:build --skip-nx-cache` completed successfully before any edit. The permanent guard reported `170 stories, 61 docs, 61 TS/TSX inputs, 0 MDX`.

An independent parse of `storybook-static/index.json` proved:

- 231 entries exactly: 170 stories plus 61 docs;
- 61 unique inputs, all `.tsx`;
- 61 unique docs inputs, all `.tsx`;
- all 61 docs entries carry the `autodocs` tag;
- zero Markdown/MDX and zero unsupported inputs.

The reliable pre-edit index SHA-256 was `72e76f1580736f6612ed36b57d8fee1b0461adf1bc9c3c25ab88fe9e83713ce4`. Every stop-condition value matched the accepted Terra scout, so execution continued.

## Installed-Source Autodocs Versus MDX Reachability

The installed packages independently confirm that the configured plugin has no reachable owned input:

- `node_modules/@mdx-js/rollup/lib/index.js` constructs a `VFile` and calls `formatAwareProcessors.process` only when the file extension belongs to `formatAwareProcessors.extnames`.
- `node_modules/@mdx-js/mdx/lib/util/create-format-aware-processors.js` defines those extensions from the Markdown and MDX extension sets.
- `node_modules/@storybook/addon-docs/dist/_node-chunks/mdx-plugin-C4QBPO5J.js` filters Storybook's separate MDX transform with `/\.mdx$/`; the active root configuration removes that plugin before registering the root Rollup plugin.
- `node_modules/storybook/dist/core-server/index.js` creates Autodocs by copying the existing CSF story entry's `importPath` into a docs entry. Its separate MDX path analyzes real MDX and emits attached/unattached MDX tags.

The fresh index then proves the live outcome rather than relying only on source inspection: all 61 generated docs entries reuse 61 `.tsx` inputs and no Markdown/MDX module reaches the rehype processor.

## Lock And Boundary Semantics

`bun install` completed successfully. The complete packet-owned unstaged `bun.lock` delta relative to the existing index is exactly two deleted workspace rows: root and UI React. Fresh exact scans prove:

| Invariant                                                   | Post-edit result |
| ----------------------------------------------------------- | ---------------: |
| active root/UI `rehype-slug` or `rehypeSlug` references     |                0 |
| direct package-manifest owners outside `node_modules`       |                3 |
| Compose manifest rows at `^6.0.0`                           |                3 |
| Bun lock workspace tuples at `^6.0.0`                       |                3 |
| Bun lock `rehype-slug@6.0.0` resolutions                    |                1 |
| retained active `rehype-autolink-headings` Storybook import |                1 |
| retained root/UI `@mdx-js/rollup` manifest rows              |                2 |
| retained UI Dagre direct rows                               |                1 |

The retained Compose owners are:

- `compose/client/lib/sketchpad/js/package.json`
- `compose/client/ui/vscode/package.json`
- `compose/dev/algorithm/package.json`

`🔒️dependencies.json` intentionally retains the historical dependency-freeze baseline recorded at commit `95b8688ee2`; it is not a live direct manifest and was not changed. The live verifier and fresh direct list correctly classify `rehype-slug` as removed.

## Post-Edit Differential

The second full uncached UI Storybook build completed successfully. The permanent guard again reported `170 stories, 61 docs, 61 TS/TSX inputs, 0 MDX`. The independent parser reproduced every pre-edit invariant, including all 61 docs as unique TSX Autodocs inputs. The post-edit index SHA-256 is byte-for-byte identical: `72e76f1580736f6612ed36b57d8fee1b0461adf1bc9c3c25ab88fe9e83713ce4`.

The post-edit input scans again produced 0 owned MDX files, 0 static Markdown/MDX module edges, and 0 CommonJS Markdown/MDX edges.

## Commands And Results

| Command                                                                                                                    | Result                                                                                                                                                                                                      |
| -------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bun x nx run @semio-tech/ui-react:build --skip-nx-cache` before edit                                                      | PASS; exact guard baseline 170/61/61/0.                                                                                                                                                                     |
| independent `storybook-static/index.json` parser before edit                                                               | PASS; 231 = 170 + 61, 61 unique TSX inputs, 61 Autodocs, 0 MD/MDX/unsupported; reliable SHA-256 reproduced.                                                                                                 |
| installed-source processor and Autodocs/MDX reachability audit                                                             | PASS; Rollup accepts Markdown/MDX extensions, Autodocs reuses CSF TSX import paths, and real MDX follows a separate path.                                                                                   |
| `bun install`                                                                                                              | PASS; exact two-tuple lock delta.                                                                                                                                                                           |
| `bun x nx run @semio-tech/ui-react:build --skip-nx-cache` after edit                                                       | PASS; exact guard baseline reproduced.                                                                                                                                                                      |
| independent index parser and SHA-256 after edit                                                                            | PASS; all semantic counts and raw index bytes equal.                                                                                                                                                        |
| `bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache`                                                             | PASS; 21 files, 724 tests.                                                                                                                                                                                  |
| `bun x nx run @semio-tech/ui-react:lint --skip-nx-cache`                                                                   | PASS.                                                                                                                                                                                                       |
| `bun x nx run @semio-tech/ui-react:typecheck --skip-nx-cache`                                                              | PASS.                                                                                                                                                                                                       |
| `bun install --frozen-lockfile`                                                                                            | PASS; 1,945 installs across 1,997 packages, no changes.                                                                                                                                                     |
| `bun ./📜️script.ts verify dependencies`                                                                                    | PASS; current ratchet 132, no new dependency.                                                                                                                                                               |
| `bun ./📜️script.ts verify dependencies list js --format json`                                                              | PASS; exactly 69 JavaScript identities and no `rehype-slug`.                                                                                                                                                |
| `bun ./📜️script.ts verify dependencies list rust --format json`                                                            | PASS; exactly 63 Rust identities.                                                                                                                                                                           |
| `bun ./📜️script.ts verify dependencies parity js`                                                                          | PASS; manifests 83, external rows 250, evidenced 105, unowned 145, undeclared imports 0, lock workspaces 44, lock mismatches 0, lock fixtures 5.                                                            |
| exact source/manifest/lock/non-goal scans                                                                                  | PASS with the counts above; `rehype-autolink-headings`, MDX Rollup, Compose, and Dagre retained.                                                                                                            |
| `bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'`                                                  | PASS.                                                                                                                                                                                                       |
| `bunx prettier --check .storybook/main.ts package.json '<UI package.json>' 📜️script.ts`                                    | Existing shared formatting baseline remains: FAIL only for `.storybook/main.ts` and `📜️script.ts`; both package manifests pass. The retirement hunks are deletion-only, and no bulk formatting was applied. |
| whole working `git diff --check`                                                                                           | PASS.                                                                                                                                                                                                       |
| whole staged `git diff --cached --check`                                                                                   | PASS.                                                                                                                                                                                                       |
| whole HEAD `git diff HEAD --check`                                                                                         | PASS.                                                                                                                                                                                                       |
| scoped working/staged/HEAD `git diff ... --check` over `.storybook/main.ts`, both manifests, `bun.lock`, and `📜️script.ts` | PASS for all three.                                                                                                                                                                                         |

Storybook and Vitest emitted their established warnings, including asset resolution, docgen, browser externalization, CSS selector, bundle-size, and `NO_COLOR` notices; both pre- and post-edit builds completed successfully and the test count stayed exact.

The tree remains intentionally dirty with concurrent P3/P8 and earlier staged work. Nothing was staged, unstaged, reverted, committed, or otherwise mutated through Git by this packet. P3 and P8 remain preserved.
