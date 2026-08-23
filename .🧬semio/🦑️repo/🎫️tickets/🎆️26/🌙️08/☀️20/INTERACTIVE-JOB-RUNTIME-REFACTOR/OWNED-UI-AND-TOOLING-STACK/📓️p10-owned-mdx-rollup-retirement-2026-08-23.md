# Owned `@mdx-js/rollup` Retirement — 2026-08-23

## Outcome

The narrow Terra-scoped root/UI `@mdx-js/rollup` retirement is implemented. The empty root
Storybook adapter import and append are gone, the two direct root/UI manifest rows are gone, and Bun
reconciliation removed the matching root/UI workspace tuples. The Storybook-internal MDX removal
loop remains unchanged, so the internal `storybook:mdx-plugin` is still removed without reordering
any surviving Vite plugin.

This report covers only this dependency wave. It does not claim Phase 10 completion or alter the
accepted Compose, Dagre, Storybook/addon-docs, Phase 3, or Phase 8 boundaries.

## Pre-Edit Baseline And Reachability

The pre-edit uncached UI Storybook build completed successfully. The permanent discovery guard and
an independent parse of `storybook-static/index.json` agreed on the exact accepted baseline:

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

The owned non-Compose/non-ticket census found zero `*.mdx` files. Static/dynamic import and CommonJS
`require` scans found zero owned `.md`/`.mdx` module edges.

Installed-source inspection and an executable ordering probe reproduced the accepted boundary:

- Installed `@mdx-js/rollup` creates a VFile but processes it only when the file extension belongs
  to its configured Markdown/MDX extension set.
- Installed Storybook core creates Autodocs by retaining the CSF entry's `importPath`; the separate
  `extractDocs` route reads actual MDX.
- Installed addon-docs prepends `storybook:mdx-plugin` before
  `storybook:package-deduplication`, and the plugin filters only `/\.mdx$/`.
- With before/after sentinels and a pre-existing Rollup adapter, the live root configuration removed
  both MDX transforms while preserving the sentinels and every root/scope plugin in order, then
  appended the empty root `@mdx-js/rollup` adapter last.

The pre-edit final probe order was:

```text
storybook:package-deduplication
sentinel:before
sentinel:after
@tailwindcss/vite:scan
@tailwindcss/vite:generate:serve
@tailwindcss/vite:generate:build
ui-assets-serve
ui-assets-build
playground-flow-wasm-dev-stub
static-dir-serve/plugin-modules
static-dir-build/plugin-modules
static-dir-serve/renderer-modules
static-dir-build/renderer-modules
playground-iframe-embed-headers
@mdx-js/rollup
```

## Exact Change

The implementation changed only the intended wave:

1. `.storybook/main.ts`: removed `await import("@mdx-js/rollup")` and the empty
   `config.plugins.push(mdx.default({}))` append.
2. `package.json`: removed the root direct dependency row.
3. `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json`: removed the UI
   React direct dependency row.
4. `bun.lock`: removed the two matching workspace tuples through `bun install`; Bun canonically
   re-nested the existing `estree-walker` resolutions while retaining the MDX/Rollup chain.

The internal removal loop, permanent discovery guard, Storybook globs, Storybook/addon-docs,
Compose, Dagre, Cargo, P3, and P8 inputs were not changed. No replacement, fallback, shim, facade,
externalization, or `@mdx-js` subdependency retirement was added.

## Post-Edit Differential And Ordering

The post-edit uncached UI Storybook build completed successfully. Its independently parsed index is
byte-identical to the pre-edit index:

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

The post-edit executable ordering probe is exactly the pre-edit list above without only its final
`@mdx-js/rollup` item. The internal `storybook:mdx-plugin` and injected pre-existing Rollup adapter
are still removed; all surviving plugins and sentinels retain their positions.

The owned MDX census and both module-edge scans remain zero.

## Lock And Dependency Boundary

The target has zero active adapter imports/appends and zero direct root/UI manifest declarations.
The excluded Compose owners and matching Bun workspace tuples remain exactly three, all `^3.1.1`:

- `compose/client/lib/sketchpad/js/package.json`
- `compose/client/ui/vscode/package.json`
- `compose/dev/algorithm/package.json`

The lock retains exactly one shared `@mdx-js/rollup@3.1.1` resolution and its exact dependency chain
to `@mdx-js/mdx`, `@rollup/pluginutils`, `source-map`, and `vfile`. Storybook/addon-docs remains
`^10.4.0`; Dagre remains `^0.8.5`.

## Verification

| Gate                                   | Result                                                                                                                                |
| -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| Pre-edit uncached Storybook build      | PASS; status 0 and exact accepted index/hash                                                                                          |
| Post-edit uncached Storybook build     | PASS; status 0 and byte/semantic-identical index                                                                                      |
| Installed plugin-order differential    | PASS; only the trailing root adapter disappeared                                                                                      |
| UI quick suite                         | PASS; 21 files, 724 tests                                                                                                             |
| UI lint                                | PASS                                                                                                                                  |
| UI typecheck                           | PASS                                                                                                                                  |
| `bun install`                          | PASS; reconciled lock                                                                                                                 |
| Frozen install                         | PASS; Bun 1.3.14, 1,949 installs across 1,997 packages, no changes                                                                    |
| Dependency ratchet                     | PASS; baseline 238, current 130, 108 removed, zero new                                                                                |
| JavaScript dependency list             | PASS; 67 identities and target absent                                                                                                 |
| Rust dependency list                   | PASS; 63 identities                                                                                                                   |
| JavaScript dependency parity           | PASS; 83 manifests, 246 external rows, 103 evidenced, 143 unowned, 0 undeclared imports, 44 lock workspaces, 0 mismatches, 5 fixtures |
| Source/manifest/lock exactness         | PASS; zero active/root/UI target, 3 Compose tuples, 1 shared resolution and exact chain                                               |
| Owned MDX and Markdown/MDX edges       | PASS; all zero                                                                                                                        |
| Root script syntax                     | PASS; one-module Bun build with all imports externalized                                                                              |
| Changed manifest formatting            | PASS                                                                                                                                  |
| Formatter baseline                     | PRESERVED; only pre-existing `.storybook/main.ts` long-line drift and concurrent root `📜️script.ts` drift remain                      |
| Scoped working/staged/HEAD diff checks | PASS                                                                                                                                  |
| Whole working diff check               | PASS                                                                                                                                  |
| Whole staged and HEAD diff checks      | UNRELATED BLOCKER; staged prior audit capture has a blank line at EOF                                                                 |

The whole staged/HEAD blocker is
`OWNED-UI-AND-TOOLING-STACK/🧪️terra-independent-rehype-autolink-headings-quick-2026-08-23.txt:217`.
It predates and is outside this packet; staging and unrelated-owner edits are prohibited. The
target wave introduces no whitespace error.

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
