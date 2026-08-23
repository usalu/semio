# Owned `remark-gfm` Retirement — 2026-08-23

## Outcome

The accepted narrow Terra packet is implemented. The sole active non-Compose Storybook `remark-gfm` import and plugin registration are removed, as are only the root and UI React direct manifest rows and their two Bun workspace tuples. The excluded Compose boundary retains exactly three direct rows, three lock workspace tuples, and the one shared `remark-gfm@4.0.1` resolution.

This is one direct-dependency retirement under the open Owned UI and Tooling Stack ticket. It is not a Phase 10 completion or acceptance claim.

## Governing Inputs Read Completely

- root `AGENTS.md`
- `🧰️framework/🔨️modules/🖱️ui/AGENTS.md`
- master `📋️master.md`
- `📓️coordinator-dependency-boundary-2026-08-22.md`
- `📝️coordinator-current-js-dependencies.txt`
- `📓️terra-next-accepted-remark-gfm-scout-2026-08-23.md`
- the master and child ticket records
- all goal records through the filesystem fallback because the repo MCP resource/tool surface was not exposed in this session

No ticket lifecycle, goal lifecycle, Cargo, Compose, coordinator report, Git index, Git branch, or Git commit operation was performed.

## Exact Changed Inventory

| File                                                                                | Packet-owned change                                                                 |
| ----------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `.storybook/main.ts`                                                                | Removed `import remarkGfm from "remark-gfm"` and `remarkPlugins: [remarkGfm]` only. |
| `package.json`                                                                      | Removed the root direct `"remark-gfm": "^4.0.1"` row only.                          |
| `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json` | Removed the UI React direct row only.                                               |
| `bun.lock`                                                                          | `bun install` removed only the matching root and UI workspace tuples.               |
| this report                                                                         | Recorded the implementation and verification evidence.                              |

The permanent `StorybookDiscoveryGuard` in `📜️script.ts` was not changed by this packet. `@mdx-js/rollup`, both configured rehype plugins, the Compose boundary, Dagre, and guard behavior remain intact. Concurrent P8 work added separate unstaged root-script guard changes during execution; those changes were preserved untouched and are not attributed to this packet.

## Pre-Edit Stop-Condition Baseline

The exact scans from the accepted packet produced:

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
- zero `.md`/`.mdx` and zero unsupported inputs.

The pre-edit index SHA-256 was `72e76f1580736f6612ed36b57d8fee1b0461adf1bc9c3c25ab88fe9e83713ce4`.

## Lock And Boundary Semantics

`bun install` completed successfully and reported one installed-package removal. The complete unstaged `bun.lock` delta relative to the existing index is exactly two deleted workspace rows: root and UI React. Fresh exact scans prove:

| Invariant                                             | Post-edit result |
| ----------------------------------------------------- | ---------------: |
| active root/UI `remark-gfm` or `remarkGfm` references |                0 |
| Compose manifest rows at `^4.0.1`                     |                3 |
| Bun lock workspace tuples at `^4.0.1`                 |                3 |
| Bun lock `remark-gfm@4.0.1` resolutions               |                1 |
| root/UI lock tuples                                   |                0 |
| retained `@mdx-js/rollup` root/UI bindings            |                4 |
| retained active rehype bindings in Storybook          |                3 |
| retained UI Dagre direct rows                         |                1 |

The retained Compose owners are:

- `compose/client/lib/sketchpad/js/package.json`
- `compose/client/ui/vscode/package.json`
- `compose/dev/algorithm/package.json`

## Post-Edit Differential

The second full uncached UI Storybook build completed successfully. The permanent guard again reported `170 stories, 61 docs, 61 TS/TSX inputs, 0 MDX`. The independent parser reproduced every pre-edit invariant, including all 61 docs as unique TSX Autodocs inputs. The post-edit index SHA-256 is the same byte-for-byte value, `72e76f1580736f6612ed36b57d8fee1b0461adf1bc9c3c25ab88fe9e83713ce4`.

The post-edit input scans again produced 0 owned MDX files, 0 static Markdown/MDX module edges, and 0 CommonJS Markdown/MDX edges.

## Commands And Results

| Command                                                                                                                    | Result                                                                                                                                                                                                      |
| -------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bun x nx run @semio-tech/ui-react:build --skip-nx-cache` before edit                                                      | PASS; exact guard baseline 170/61/61/0.                                                                                                                                                                     |
| independent `storybook-static/index.json` parser before edit                                                               | PASS; 231 = 170 + 61, 61 unique TSX inputs, 61 Autodocs, 0 MD/MDX.                                                                                                                                          |
| `bun install`                                                                                                              | PASS; exact two-tuple lock delta.                                                                                                                                                                           |
| `bun x nx run @semio-tech/ui-react:build --skip-nx-cache` after edit                                                       | PASS; exact guard baseline reproduced.                                                                                                                                                                      |
| independent index parser and SHA-256 after edit                                                                            | PASS; all semantic counts and raw index bytes equal.                                                                                                                                                        |
| `bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache`                                                             | PASS; 21 files, 724 tests.                                                                                                                                                                                  |
| `bun x nx run @semio-tech/ui-react:lint --skip-nx-cache`                                                                   | PASS.                                                                                                                                                                                                       |
| `bun x nx run @semio-tech/ui-react:typecheck --skip-nx-cache`                                                              | PASS.                                                                                                                                                                                                       |
| `bun install --frozen-lockfile`                                                                                            | PASS; 1,945 installs across 1,997 packages, no changes.                                                                                                                                                     |
| `bun ./📜️script.ts verify dependencies`                                                                                    | PASS; current ratchet 133, no new dependency.                                                                                                                                                               |
| `bun ./📜️script.ts verify dependencies list js --format json`                                                              | PASS; exactly 70 JavaScript identities.                                                                                                                                                                     |
| `bun ./📜️script.ts verify dependencies list rust --format json`                                                            | PASS; exactly 63 Rust identities.                                                                                                                                                                           |
| `bun ./📜️script.ts verify dependencies parity js`                                                                          | PASS; manifests 83, external rows 252, evidenced 106, unowned 146, undeclared imports 0, lock workspaces 44, lock mismatches 0, lock fixtures 5.                                                            |
| `bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'`                                                  | PASS.                                                                                                                                                                                                       |
| exact source/manifest/lock scans                                                                                           | PASS with the counts above.                                                                                                                                                                                 |
| `bunx prettier --check .storybook/main.ts package.json '<UI package.json>' 📜️script.ts`                                    | Existing shared formatting baseline remains: FAIL only for `.storybook/main.ts` and `📜️script.ts`; both package manifests pass. The retirement hunks are deletion-only, and no bulk formatting was applied. |
| whole working `git diff --check`                                                                                           | PASS.                                                                                                                                                                                                       |
| whole staged `git diff --cached --check`                                                                                   | PASS.                                                                                                                                                                                                       |
| whole HEAD `git diff HEAD --check`                                                                                         | PASS.                                                                                                                                                                                                       |
| scoped working/staged/HEAD `git diff ... --check` over `.storybook/main.ts`, both manifests, `bun.lock`, and `📜️script.ts` | PASS for all three.                                                                                                                                                                                         |

Storybook and Vitest emitted their established build/test warnings, including asset resolution, docgen, browser externalization, CSS selector, bundle-size, and `NO_COLOR` notices; both pre- and post-edit builds completed successfully and the test count stayed exact.

The previously reported stale staged-report whitespace snapshot was not present at the final checkpoint: whole `git diff --cached --check` passed. The tree remains intentionally dirty with concurrent P3/P8 and earlier staged work. Nothing was staged, unstaged, reverted, committed, or otherwise mutated through Git by this packet.
