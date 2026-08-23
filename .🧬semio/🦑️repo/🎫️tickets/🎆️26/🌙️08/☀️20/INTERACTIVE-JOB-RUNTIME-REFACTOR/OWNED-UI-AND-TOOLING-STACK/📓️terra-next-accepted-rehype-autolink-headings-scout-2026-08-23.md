# Terra Next Direct Dependency Scout — `rehype-autolink-headings` — 2026-08-23

## Decision

**ACCEPTED candidate: `rehype-autolink-headings`.**

This is one direct, single-identity Phase 10 root/UI tooling wave. The active `.storybook/main.ts` binding imports `rehypeAutolinkHeadings` and registers it in the root `@mdx-js/rollup` `rehypePlugins` array. Its transform input domain is independently proven empty, including generated Autodocs. Removing it removes no reachable heading-anchor behavior.

Expected direct boundary: **132 = 69 JavaScript + 63 Rust** → **131 = 68 JavaScript + 63 Rust**. This is a narrow dependency-wave packet, not Phase 10 acceptance.

## Exact Owners And Lock Graph

| Boundary          | Evidence                                                                                                                                                                  |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Active binding    | `.storybook/main.ts:14` imports `rehypeAutolinkHeadings`; line 168 is the sole non-Compose/non-ticket registration: `rehypePlugins: [rehypeAutolinkHeadings]`.            |
| Root/UI owners    | `package.json:192` and `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json:62` each declare `^7.1.0`.                                      |
| Compose retainers | `compose/client/lib/sketchpad/js/package.json:34`, `compose/client/ui/vscode/package.json:61`, and `compose/dev/algorithm/package.json:32` are excluded direct owners.    |
| Lock graph        | Five `^7.1.0` workspace tuples exist at root line 33, Compose lines 138/246/275, and UI line 537; one shared `rehype-autolink-headings@7.1.0` resolution is at line 3695. |

The executor removes only the root/UI import+array element, two manifest rows, and two root/UI tuples. The three Compose rows/tuples and the one shared resolution must remain.

## Independent Autodocs And MDX Reachability Proof

`rehype-autolink-headings` only visits HAST headings already carrying IDs and injects/updates an anchor link. It has no effect outside a Unified/MDX HAST processor.

The local `@mdx-js/rollup` transform constructs a VFile and invokes its processor only if the module extension is in the Markdown/MDX extension set. Installed Storybook core emits an Autodocs entry by copying the CSF story entry's existing `importPath`; its separate `extractDocs` method reads and analyzes actual MDX. Its internal `storybook:mdx-plugin` filters only `.mdx`, and root `.storybook/main.ts` removes that plugin before adding the root Rollup plugin. Generated Autodocs therefore do not create a virtual MDX input that can reach the configured rehype plugin.

Fresh evidence independently confirms that path:

- owned non-Compose/non-ticket `*.mdx` census: **0**;
- static-import, dynamic-import, and CommonJS-`require` scans for owned `.md`/`.mdx` edges: **zero**;
- current built index: **231 = 170 stories + 61 docs**, **61 unique `.tsx` inputs**, **61 `autodocs` docs**, zero Markdown/MDX and zero unsupported input;
- current SHA-256: `72e76f1580736f6612ed36b57d8fee1b0461adf1bc9c3c25ab88fe9e83713ce4`, the reliable zero-MDX baseline.

The root permanent `StorybookDiscoveryGuard` already rejects owned MDX before build and asserts the exact UI index after build. It must stay unchanged; do not add an autolink fallback, compatibility shim, replacement anchor renderer, externalization, or plugin-specific exemption.

## Differential And Executor Packet

Pre- and post-edit must preserve `0` owned MDX and Markdown/MDX module edges, index `231 = 170 + 61`, 61 unique TS/TSX inputs, 61 TSX Autodocs entries, and the exact pre/post SHA-256.

1. Record that differential and run a full uncached UI Storybook build before edits.
2. Remove only `rehypeAutolinkHeadings` and its one array element in `.storybook/main.ts`.
3. Remove only root/UI direct manifest rows; run `bun install` and retain exactly three Compose tuples plus the shared 7.1.0 resolution.
4. Do not change `@mdx-js/rollup`, Storybook, any other rehype package, Compose, Dagre, the discovery guard, globs, or externalization.
5. Run the independent gates below; stop on any changed count/hash, nonzero MDX edge, changed retention, or functional failure.

## Independent Audit Gates

```text
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
find . -type f -name '*.mdx' ! -path './node_modules/*' ! -path './compose/*' ! -path './.🧬semio/*' ! -path './.git/*' -print | wc -l
rg --hidden -n -P "\b(?:from\s+|import\s*\()['\"][^'\"]+\.mdx?['\"]" --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!compose/**' --glob '!.🧬semio/**' --glob '!.git/**' .
rg --hidden -n -P "\brequire\s*\(['\"][^'\"]+\.mdx?['\"]\)" --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!compose/**' --glob '!.🧬semio/**' --glob '!.git/**' .
rg -n 'rehype-autolink-headings|rehypeAutolinkHeadings' .storybook package.json '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json'
rg -n 'rehype-autolink-headings' compose/client/lib/sketchpad/js/package.json compose/client/ui/vscode/package.json compose/dev/algorithm/package.json bun.lock
bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'
bunx prettier --check .storybook/main.ts package.json '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json' 📜️script.ts
git diff --check
git diff --cached --check
git diff HEAD --check
```

Acceptance requires `68` JavaScript and `63` Rust identities; no root/UI source/config or direct-manifest row; three Compose manifests/tuples and one retained 7.1.0 resolution; frozen install; the permanent guard/index/hash differential; and clean scoped plus whole working/staged/HEAD diff checks.

## Compared Leaves And Non-Goals

- `@mdx-js/rollup`, Storybook packages, and all other rehype dependencies are configuration/infrastructure, not part of this packet.
- Compose remains excluded; Dagre remains held for its Rust/Wasm/OffscreenCanvas lane.
- PostCSS/Tailwind, coverage-v8, JCO/WASM materialization, JSDOM, runtime UI primitives, `three-mesh-bvh`, and `reveal.js` have active configuration, test, tooling, or runtime consumers.
- `eslint-plugin-react-hooks` and similar manifest-only rows lack truthful direct-use proof.
- Phase 9 candidates need dedicated runtime-replacement differentials and risk P3/P8/Cargo contention; no Rust source or Cargo command is permitted here.

No production source, manifest, lockfile, Compose input, ticket lifecycle state, Git state, or Cargo input was changed by this read-only scout.
