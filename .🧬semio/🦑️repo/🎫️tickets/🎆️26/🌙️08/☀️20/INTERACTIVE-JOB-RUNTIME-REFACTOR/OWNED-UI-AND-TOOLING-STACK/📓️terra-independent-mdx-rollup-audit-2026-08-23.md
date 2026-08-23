# Independent `@mdx-js/rollup` Retirement Audit — 2026-08-23

## Verdict

**ACCEPT** — no blockers.

This accepts only the narrow root/UI `@mdx-js/rollup` dependency wave at **131 = 68 JavaScript + 63 Rust** → **130 = 67 JavaScript + 63 Rust**. It is not Phase 10 acceptance and does not accept changes to Compose, Dagre, Storybook/addon-docs, P3, or P8.

## Scope And Source Review

The live HEAD-to-index diff contains exactly four wave paths:

1. `.storybook/main.ts`: deletes only `await import("@mdx-js/rollup")`, its empty `mdx.default({})` append, and the now-empty separator.
2. `package.json`: deletes the root direct `^3.1.1` row.
3. `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`: deletes the UI direct `^3.1.1` row.
4. `bun.lock`: deletes those two workspace tuples and performs Bun's canonical `estree-walker` re-keying.

The Storybook removal loop remains verbatim: it still removes both existing `@mdx-js/rollup` plugins and the promised `storybook:mdx-plugin`. `config.build.rollupOptions.external` is unchanged. No shim, fallback, facade, replacement adapter, plugin-specific exemption, or externalization was added.

## Installed-Plugin Ordering And Reachability

The installed addon-docs plugin is named `storybook:mdx-plugin` and filters `/\.mdx$/`. The installed Rollup adapter only processes a VFile when its extension is in the processor's Markdown/MDX extension set. Storybook's generated Autodocs retain the CSF `importPath`; actual MDX is handled by the separate `extractDocs` path.

I ran the live `viteFinal` with two sentinels, a promised internal MDX plugin, and both string/object forms of a pre-existing Rollup adapter. The result was:

```text
hasInternalMdx=false
hasRollup=false
plugins=[
  sentinel:before,
  sentinel:after,
  @tailwindcss/vite:scan,
  @tailwindcss/vite:generate:serve,
  @tailwindcss/vite:generate:build,
  ui-assets-serve,
  ui-assets-build,
  playground-flow-wasm-dev-stub,
]
```

The sentinels remained first and adjacent. The reviewed pre-image differs only by its trailing root `@mdx-js/rollup` append, so post-retirement ordering is exactly the pre-image order minus that final empty adapter. The retained loop, rather than its removal, is therefore necessary for the installed Storybook internal plugin to stay absent.

The permanent `BuildScript` runs `assertNoOwnedStorybookMdx` before Storybook and `assertUiStorybookDiscovery` after it. Its current successful uncached invocation is consequently a guard execution, not just an index inspection. Independent guard-equivalent census found zero owned `*.mdx`; static/dynamic import and CommonJS-require scans found zero owned `.md`/`.mdx` edges. Compose still has seven excluded live adapter imports, outside this root/UI wave.

## Independent Build Differential

`bun x nx run @semio-tech/ui-react:build --skip-nx-cache` completed with Nx `cache-miss` and status `0`. The independently parsed `storybook-static/index.json` is byte-identical to the accepted pre-image hash:

| Invariant           |                                                             Result |
| ------------------- | -----------------------------------------------------------------: |
| Entries             |                                                                231 |
| Stories             |                                                                170 |
| Docs                |                                                                 61 |
| Unique inputs       |                                                          61 `.tsx` |
| Docs inputs         |                                                          61 `.tsx` |
| `autodocs` docs     |                                                                 61 |
| Markdown/MDX inputs |                                                                  0 |
| Unsupported inputs  |                                                                  0 |
| SHA-256             | `72e76f1580736f6612ed36b57d8fee1b0461adf1bc9c3c25ab88fe9e83713ce4` |

## Dependency And Lock Boundary

Root/UI manifests and root/UI active adapter import/require edges are absent. The only remaining direct owners are the three required Compose packages:

- `compose/client/lib/sketchpad/js/package.json:23`
- `compose/client/ui/vscode/package.json:54`
- `compose/dev/algorithm/package.json:28`

Their three lock workspace tuples remain at lines 125, 237, and 269. Exactly one shared `@mdx-js/rollup@3.1.1` resolution remains (line 1359), retaining its `@mdx-js/mdx`, `@rollup/pluginutils`, `source-map`, and `vfile` dependency chain. No `@mdx-js` subdependency was removed.

`bun install --frozen-lockfile` passed under Bun 1.3.14: 1,949 installs across 1,997 packages, no changes. The dependency ratchet passed at 130, with 108 baseline identities removed and none added. Fresh parsed lists contain 67 JavaScript identities and 63 Rust identities; the target is absent. JavaScript parity passed: 83 manifests, 246 external rows, 103 evidenced, 143 unowned, 0 undeclared imports, 44 lock workspaces, 0 mismatches, and 5 fixtures.

## Quality And Hygiene Gates

| Gate                                          | Independent result                                                                        |
| --------------------------------------------- | ----------------------------------------------------------------------------------------- |
| UI quick                                      | PASS — 21 files, 724 tests                                                                |
| UI lint                                       | PASS                                                                                      |
| UI typecheck                                  | PASS                                                                                      |
| Root script syntax                            | PASS — `bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'`          |
| Changed manifests Prettier                    | PASS                                                                                      |
| `.storybook/main.ts` Prettier                 | Existing baseline failure; HEAD itself exits 1, and the wave only removed three lines     |
| `📜️script.ts` Prettier                        | Existing concurrent baseline failure; HEAD itself exits 1 and this wave does not touch it |
| `bun.lock` Prettier                           | Not applicable: Prettier has no parser for Bun lockfiles                                  |
| Scoped working/staged/HEAD `git diff --check` | PASS                                                                                      |
| Whole working/staged/HEAD `git diff --check`  | PASS                                                                                      |

The prior quick-capture EOF condition is synchronized: it is currently staged as a clean `A` file, not `AM`. The related prompt is an unrelated staged modification. All three whole-tree checks above are clean, so neither is a wave blocker.

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
bunx prettier --check package.json '<ui-react>/package.json'
bunx prettier --check .storybook/main.ts
bunx prettier --check 📜️script.ts
git show HEAD:.storybook/main.ts | bunx prettier --check --stdin-filepath .storybook/main.ts
git show 'HEAD:📜️script.ts' | bunx prettier --check --stdin-filepath 📜️script.ts
git diff --check
git diff --cached --check
git diff HEAD --check
```

No production source, manifest, lockfile, Git state, ticket lifecycle, or Cargo input was changed by this audit. No temporary log was written outside the ticket.
