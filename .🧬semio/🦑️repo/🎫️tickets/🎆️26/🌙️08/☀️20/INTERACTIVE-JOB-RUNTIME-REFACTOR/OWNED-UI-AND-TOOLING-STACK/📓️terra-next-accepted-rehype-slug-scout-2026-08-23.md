# Terra Next Direct Dependency Scout — `rehype-slug` — 2026-08-23

## Decision

**ACCEPTED candidate: `rehype-slug`.**

This is one direct, single-identity Phase 10 root/UI tooling wave. The binding is active configuration: `.storybook/main.ts` imports `rehypeSlug` and passes it to the live `@mdx-js/rollup` `rehypePlugins` array. It is nevertheless unreachable because neither handwritten Markdown/MDX nor generated Autodocs reaches that processor. Removal deletes the unavailable heading-ID transform; it does not remove, recreate, or change any displayed documentation behavior.

The accepted boundary moves from **133 = 70 JavaScript + 63 Rust** to **132 = 69 JavaScript + 63 Rust**. This is not Phase 10 acceptance.

## Governing Boundary And Candidate Selection

I re-read the coordinator boundary, direct-dependency list, prior `remark-mdx-frontmatter`, `remark-frontmatter`, and `remark-gfm` scout/implementation/audit reports, and the current source/lock graph. The coordinator lists 70 JavaScript and 63 Rust identities; `rehype-slug` is still a direct JavaScript identity.

`rehype-autolink-headings` has the same direct-owner and zero-input shape. It is deliberately **not** bundled. `rehype-slug` is selected first because it is the producer of the heading IDs that the remaining autolink binding consumes, so this packet removes one extension while leaving the other identity untouched for a separate evidence packet. The permanent zero-MDX guard prevents a later MDX addition from silently restoring a path with different heading behavior.

No smaller truthful Phase 9 candidate is selected: those candidates require a runtime replacement/differential and would expand beyond this root/UI tooling lane or overlap active Rust/P3/P8 work. The selected packet touches no Rust/Cargo source or P3/P8-owned file.

## Exact Binding, Owners, And Lock Reachability

| Boundary              | Fresh evidence                                                                                                                                                                                             |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Active source binding | `.storybook/main.ts:17` imports `rehypeSlug`; line 169 registers `rehypePlugins: [rehypeSlug, rehypeAutolinkHeadings]`. Exact non-Compose/non-ticket source scans find no other `rehypeSlug` binding.      |
| Root direct owner     | `package.json:193` declares `rehype-slug: ^6.0.0`.                                                                                                                                                         |
| UI direct owner       | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json:63` declares the same row.                                                                                              |
| Compose retainers     | `compose/client/lib/sketchpad/js/package.json:35`, `compose/client/ui/vscode/package.json:62`, and `compose/dev/algorithm/package.json:33` are out-of-scope direct owners.                                 |
| Lock graph            | `bun.lock` has five `rehype-slug: ^6.0.0` workspace tuples: root line 34, the three Compose owners at 140/248/277, and UI line 539. It has exactly one shared `rehype-slug@6.0.0` resolution at line 3701. |

The executor removes exactly the root/UI source import+array element, two manifest rows, and two corresponding Bun workspace tuples. It must retain the three Compose manifest rows/tuples and the one shared 6.0.0 resolution.

## Processor And Generated-Autodocs Reachability Proof

`rehype-slug` only visits HAST heading elements and assigns their `id` properties. It has no standalone side effect. It runs solely because the active root `@mdx-js/rollup` configuration receives it as a `rehypePlugin`.

The local `@mdx-js/rollup` transform creates a `VFile`, then calls `process` only if its extension belongs to the local format-aware list. That list is Markdown extensions plus `.mdx`. The installed Storybook `storybook:mdx-plugin` likewise filters only `/\\.mdx$/`; `.storybook/main.ts` explicitly removes it before adding the root Rollup plugin. Therefore there is no second Vite MDX transform left to reach this binding.

The premise was tested against generated Autodocs rather than inferred from the zero handwritten-MDX census:

- The owned non-Compose/non-ticket census is **0 `*.mdx` files**. Static import, dynamic import, and CommonJS `require` scans find **zero** owned `.md`/`.mdx` module edges.
- Installed Storybook core creates an Autodocs entry directly from the CSF story entry’s existing `importPath`. Its separate `extractDocs` path reads and analyzes actual MDX files. Autodocs therefore uses the TS/TSX entry rather than producing a virtual MDX module for Vite.
- The fresh uncached build index is **231 = 170 stories + 61 docs**, with **61 unique `.tsx` inputs**, **61 docs entries tagged `autodocs`**, no Markdown/MDX input, and no unsupported input. The raw index SHA-256 is `72e76f1580736f6612ed36b57d8fee1b0461adf1bc9c3c25ab88fe9e83713ce4`, matching the immediately preceding reliable zero-MDX waves.

Thus no owned document is transformed into HAST and neither `rehype-slug` nor its remaining autolink counterpart is invoked for generated Autodocs. The candidate is a genuine configuration binding with a provably empty current input domain.

## Differential And Permanent Guard

Before and after the one-identity change, require all of the following:

| Invariant                                                     | Required value                                 |
| ------------------------------------------------------------- | ---------------------------------------------- |
| owned `*.mdx` files                                           | `0`                                            |
| owned `.md`/`.mdx` import, dynamic-import, or `require` edges | `0`                                            |
| Storybook index                                               | `231 = 170 stories + 61 docs`                  |
| source inputs                                                 | 61 unique TS/TSX, 0 Markdown/MDX/unsupported   |
| generated docs                                                | 61 `autodocs` entries from those TS/TSX inputs |

The current root `StorybookDiscoveryGuard` already enforces the owned zero-MDX input census before build and this exact UI index after build. Do not change it, weaken it, add a plugin-specific exemption, or replace it with a compatibility behavior. The executor should record a pre/post index SHA-256 as an additional differential when both builds complete.

## Executor Packet

1. Re-record the input/module-edge proof and run the full uncached UI Storybook build. Capture its index count, unique extensions/tags, and SHA-256; stop if the permanent guard differs.
2. Remove only `rehypeSlug` and its one `rehypePlugins` element in `.storybook/main.ts`.
3. Remove only the root and UI React direct manifest rows.
4. Run `bun install`; expect only the root/UI workspace tuples to lose `rehype-slug`. Retain the three Compose tuples and the shared `rehype-slug@6.0.0` resolution.
5. Do not alter `rehype-autolink-headings`, `@mdx-js/rollup`, Storybook packages/globs, Dagre, externalization, or the permanent guard. Do not add an owned heading parser, slugger, anchor fallback, facade, shim, or replacement dependency.
6. Re-run every gate below and require the exact post-edit differential before requesting audit.

## Required Independent Audit Gates

```text
# Pre/post input and module-edge proof
find . -type f -name '*.mdx' ! -path './node_modules/*' ! -path './compose/*' ! -path './.🧬semio/*' ! -path './.git/*' -print | wc -l
rg --hidden -n -P "\b(?:from\s+|import\s*\()['\"][^'\"]+\.mdx?['\"]" --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!compose/**' --glob '!.🧬semio/**' --glob '!.git/**' .
rg --hidden -n -P "\brequire\s*\(['\"][^'\"]+\.mdx?['\"]\)" --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!compose/**' --glob '!.🧬semio/**' --glob '!.git/**' .

# Full UI gates
bun x nx run @semio-tech/ui-react:build --skip-nx-cache
bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache
bun x nx run @semio-tech/ui-react:lint --skip-nx-cache
bun x nx run @semio-tech/ui-react:typecheck --skip-nx-cache

# Dependency and lock gates
bun install
bun install --frozen-lockfile
bun ./📜️script.ts verify dependencies
bun ./📜️script.ts verify dependencies list js --format json
bun ./📜️script.ts verify dependencies list rust --format json
bun ./📜️script.ts verify dependencies parity js

# Exact absence, retention, syntax, and hygiene
rg -n 'rehype-slug|rehypeSlug' .storybook package.json '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json'
rg -n 'rehype-slug' compose/client/lib/sketchpad/js/package.json compose/client/ui/vscode/package.json compose/dev/algorithm/package.json bun.lock
bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'
bunx prettier --check .storybook/main.ts package.json '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json' 📜️script.ts
git diff --check -- .storybook/main.ts package.json bun.lock 📜️script.ts '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json'
git diff --cached --check -- .storybook/main.ts package.json bun.lock 📜️script.ts '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json'
git diff HEAD --check -- .storybook/main.ts package.json bun.lock 📜️script.ts '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json'
```

Acceptance requires exactly 69 JavaScript and 63 Rust direct identities, absence of root/UI binding/manifests/tuples, exactly three retained Compose manifest/lock tuples plus one `rehype-slug@6.0.0` resolution, zero new dependency, frozen install, permanent guard/index equality, and honest formatter/diff results.

## Compared Candidates And Non-Goals

| Candidate or area                                                                                                        | Disposition                                                                                                                                                               |
| ------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `rehype-autolink-headings`                                                                                               | Same zero-input transform shape but intentionally deferred; do not bundle it with `rehype-slug`. It remains a direct config/manifests/Compose identity for its own scout. |
| `@mdx-js/rollup`, `@mdx-js/*`, Storybook packages                                                                        | Configuration/infrastructure, explicitly out of scope. The local Rollup extension gate is the proof here, not a deletion target.                                          |
| Compose                                                                                                                  | Three direct retainers prove the lock resolution must remain; no Compose source, manifests, lock tuples, or tests are touched.                                            |
| Dagre                                                                                                                    | Held until the genuine Rust/Wasm/OffscreenCanvas directed-layout lane is ready.                                                                                           |
| PostCSS/Tailwind, coverage-v8, JCO/WASM materialization, JSDOM, runtime UI primitives, `three-mesh-bvh`, and `reveal.js` | Active configuration, test, tooling, or runtime consumers, not smaller isolated zero-input leaves.                                                                        |
| `eslint-plugin-react-hooks` and other manifest-only rows                                                                 | Rejected without a truthful direct-use proof; absence of an ordinary import is insufficient.                                                                              |
| Phase 9/runtime candidates                                                                                               | Require dedicated replacement/differential packets, risk P3/P8 Rust overlap, or need Cargo; not smaller/independent under this boundary.                                  |

No production source, manifest, lockfile, Cargo input, Compose input, ticket lifecycle state, or Git state was changed by this read-only scout.
