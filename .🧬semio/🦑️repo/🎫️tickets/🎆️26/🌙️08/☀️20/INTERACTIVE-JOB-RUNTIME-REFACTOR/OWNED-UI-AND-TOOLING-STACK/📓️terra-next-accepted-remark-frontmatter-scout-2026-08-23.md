# Terra Next Direct Dependency Scout — `remark-frontmatter` — 2026-08-23

## Decision

**ACCEPTED candidate: `remark-frontmatter`.**

This is the smallest truthful next direct-retirement wave in the accepted boundary. It is a live root Storybook configuration binding, not a manifest-only deletion, but all modules the configured MDX Rollup plugin can transform are absent from the owned Storybook graph. Removal therefore deletes an unreachable parser extension rather than recreating frontmatter behavior.

The expected direct boundary moves from **135 = 72 JavaScript + 63 Rust** to **134 = 71 JavaScript + 63 Rust**.

## Governing Boundary

The governing Phase 9/10 plan, current coordinator boundary, accepted `remark-mdx-frontmatter` implementation/audit, and prior Terra scout were re-read. Phase 9 remains an ordered runtime-replacement wave requiring dual-run differentials; no Phase 9 candidate is selected because none is clearly smaller and independent of active P3/P8 Rust work or Cargo contention. This packet is Phase 10 root/UI tooling only and has no Rust source or Cargo touch.

## Exact Census

| Boundary | Evidence |
| --- | --- |
| Root manifest | `package.json` directly declares `remark-frontmatter: ^5.0.0`. |
| UI React manifest | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json` directly declares the same row. |
| Active source | `.storybook/main.ts:18` imports `remarkFrontmatter`; line 171 includes it in `mdx.default({ remarkPlugins: [remarkGfm, remarkFrontmatter] })`. |
| Other owned source | Exact non-Compose/non-ticket import, dynamic-import, and CommonJS-require scans find no further `remark-frontmatter` binding. |
| Compose owners | `compose/client/lib/sketchpad/js/package.json`, `compose/client/ui/vscode/package.json`, and `compose/dev/algorithm/package.json` directly declare it; all are out of scope. |
| Lock reachability | `bun.lock` currently has five workspace dependency tuples (root, UI React, and the three Compose owners) and one `remark-frontmatter@5.0.0` resolution. |

The source binding is active configuration, so the candidate meets the direct-use requirement. Its configured input set is nevertheless provably empty:

- The hidden non-Compose/non-ticket census contains **0 `*.mdx`** files.
- The only direct UI Storybook glob with an MDX extension is `./stories/ui/**/*.stories.@(js|jsx|mjs|ts|tsx|mdx)`; it has no `.md` alternative.
- Exact owned code scans found **zero** static import, dynamic import, or CommonJS `require` edge to `*.md` or `*.mdx`.
- The accepted uncached build index is **231 = 170 stories + 61 docs**, sourced by exactly **61 TS/TSX** import paths with zero unsupported/MDX inputs.

There are many repository Markdown files, but they are not Storybook inputs and none is imported into the owned module graph. They are not evidence against the candidate.

## Runtime/Tooling Behavior And Existing Alternative

The local installed `@mdx-js/rollup` source gates its `transform` hook on a supported file extension. Its local MDX processor declares only Markdown extensions plus `.mdx`; neither reaches the owned Storybook module graph above. `remark-frontmatter` itself only registers Micromark/MDAST frontmatter extensions on an MDX processor. With no transformed source file, removing that registration cannot alter any compiled module.

The replacement is the existing Storybook `@mdx-js/rollup` path with this unreachable extension removed. Do not add an owned parser, compatibility facade, fallback, copied frontmatter behavior, externalization, or dependency. The established `StorybookDiscoveryGuard` already rejects owned MDX inputs before build and asserts the exact TS/TSX index afterward; generalize its wording only if needed so it accurately describes all retired owned MDX-only transforms.

## Fixture Coverage And Differential

The exact input invariant is stronger than a fixture approximation: the outgoing parser receives no owned Storybook module. Preserve both sides of this differential:

1. Before edits, record the zero-MDX census, zero `.md`/`.mdx` module-import scan, and complete uncached UI Storybook index.
2. After edits, the root Nx build must again pass the permanent guard and independently yield the same `231 / 170 / 61 / 61 / 0-MDX` index.

The existing `@semio-tech/ui-react:test-quick` suite has 724 passing tests at the accepted boundary and is the functional fixture regression gate. Lint and typecheck cover the changed configuration and manifest owner.

## Executor Packet

1. Re-record the pre-edit input invariant and uncached `@semio-tech/ui-react:build --skip-nx-cache` discovery inventory; do not proceed on a timeout or incomplete build.
2. Remove only the `remarkFrontmatter` import and its one `remarkPlugins` element in `.storybook/main.ts`.
3. Remove only the root and UI React direct-manifest rows.
4. Keep `@mdx-js/rollup`, `remark-gfm`, `rehype-slug`, `rehype-autolink-headings`, Storybook globs, and Compose manifests unchanged. If the guard text changes, retain its exact scan/index behavior and its root Nx invocation.
5. Reconcile solely with `bun install`; expect only the root/UI workspace lock tuples to lose `remark-frontmatter`. The three Compose tuples and the shared package resolution must remain.

## Required Independent Gates

```text
# Input and source proof, before and after
find . -type f -name '*.mdx' ! -path './node_modules/*' ! -path './compose/*' ! -path './.🧬semio/*' ! -path './.git/*' -print | wc -l
rg -n -P "\b(?:from\s+|import\s*\()['\"][^'\"]+\.mdx?['\"]" --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!compose/**' --glob '!.🧬semio/**' --glob '!.git/**' .
rg -n -P "\brequire\s*\(['\"][^'\"]+\.mdx?['\"]\)" --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!compose/**' --glob '!.🧬semio/**' --glob '!.git/**' .

# Functional/configuration proof
bun x nx run @semio-tech/ui-react:build --skip-nx-cache
bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache
bun x nx run @semio-tech/ui-react:lint --skip-nx-cache
bun x nx run @semio-tech/ui-react:typecheck --skip-nx-cache

# Dependency/lock proof
bun install
bun install --frozen-lockfile
bun ./📜️script.ts verify dependencies
bun ./📜️script.ts verify dependencies list js --format json
bun ./📜️script.ts verify dependencies list rust --format json
bun ./📜️script.ts verify dependencies parity js

# Exact absence and hygiene
rg -n 'remark-frontmatter|remarkFrontmatter' .storybook package.json '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json'
rg -n 'remark-frontmatter' compose/client/lib/sketchpad/js/package.json compose/client/ui/vscode/package.json compose/dev/algorithm/package.json bun.lock
bunx prettier --check .storybook/main.ts package.json '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json' 📜️script.ts
git diff --check -- .storybook/main.ts package.json bun.lock 📜️script.ts '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json'
git diff --cached --check -- .storybook/main.ts package.json bun.lock 📜️script.ts '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json'
git diff HEAD --check -- .storybook/main.ts package.json bun.lock 📜️script.ts '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json'
```

Acceptance requires exact list counts `71` JavaScript and `63` Rust, no direct root/UI manifest or active source/config binding, three Compose manifest/tuple retainers, one retained 5.0.0 resolution, frozen install, and unchanged Storybook discovery. A retained transitive MDX dependency is not grounds to remove unrelated packages.

## Explicit Non-Goals And Rejections

- **Do not remove `remark-gfm`, rehype plugins, or `@mdx-js/rollup` in this packet.** GFM and generated-doc behavior require their own explicit source/input proof; no bundling.
- **Do not touch Compose.** Its three direct rows intentionally retain the lock resolution.
- **Dagre remains held** until the real Rust/Wasm/OffscreenCanvas lane is accepted.
- **Sharp remains blocked** by its print-path precondition and required image differential.
- **`eslint-plugin-react-hooks` and analogous no-consumer rows remain rejected as manifest-only.** They do not meet this scout's truthful-direct-use standard.
- **PostCSS/Tailwind, coverage-v8, JCO/WASM materialization, JSX/UI primitives, test harnesses, `three-mesh-bvh`, `reveal.js`, and JS DOM are live configuration/runtime/test consumers.** They are not safe leaf deletions.
- **No Phase 9 dependency was selected.** Runtime candidates require a replacement/differential packet and would overlap active Rust/P3/P8 work or require Cargo validation.

No production source, manifest, lockfile, Cargo input, Compose input, ticket lifecycle state, or git state was changed by this scout.
