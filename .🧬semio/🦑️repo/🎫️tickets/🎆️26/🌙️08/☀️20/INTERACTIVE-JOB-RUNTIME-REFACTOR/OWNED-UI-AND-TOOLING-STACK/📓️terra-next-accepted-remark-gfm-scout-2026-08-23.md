# Terra Next Direct Dependency Scout — `remark-gfm` — 2026-08-23

## Decision

**ACCEPTED candidate: `remark-gfm`.**

This is one small, direct Phase 10 root/UI-tooling wave. The binding is real: `.storybook/main.ts` imports `remarkGfm` and registers it in the live `@mdx-js/rollup` configuration. Its entire possible owned input domain is nevertheless empty. The root permanent discovery guard and a fresh uncached build prove that the current Storybook graph has zero MDX inputs and consists solely of the 61 generated Autodocs entries sourced from unique TS/TSX modules. Removing the registration therefore removes an unreachable GFM extension, not supported behavior.

The expected direct-dependency boundary is **134 = 71 JavaScript + 63 Rust** before, and **133 = 70 JavaScript + 63 Rust** after. This is a narrow candidate packet, not Phase 10 acceptance.

## Governing Boundary And Selection

The current coordinator list, the governing Phase 9/10 plan, and the accepted `remark-mdx-frontmatter` and `remark-frontmatter` scouts/implementation reports were re-read. Phase 9 is not a substitute candidate: its live runtime replacements require a dedicated differential and would either expand scope, touch Rust/P3/P8 ownership, or need Cargo validation. This candidate is confined to the root Storybook configuration, root/UI direct rows, and their root/UI lock tuples; it has no Rust source, Cargo, P3, or P8 overlap.

`remark-gfm` is accepted only after testing the adjacent-candidate premise rather than assuming that zero handwritten MDX makes generated Autodocs irrelevant.

## Exact Owner, Binding, And Lock Census

| Boundary           | Fresh evidence                                                                                                                                                                       |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Root direct owner  | `package.json:194` declares `remark-gfm: ^4.0.1`.                                                                                                                                    |
| UI direct owner    | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json:64` declares the same row.                                                                        |
| Active binding     | `.storybook/main.ts:18` imports `remarkGfm`; the active `mdx.default` configuration registers it in `remarkPlugins: [remarkGfm]`.                                                    |
| Other owned source | Exact hidden non-Compose/non-ticket scans found no other static import, dynamic import, or CommonJS `require` binding for `remark-gfm`.                                              |
| Compose retainers  | `compose/client/lib/sketchpad/js/package.json:37`, `compose/client/ui/vscode/package.json:64`, and `compose/dev/algorithm/package.json:35` are excluded direct owners.               |
| Lock reachability  | `bun.lock` has five workspace tuples: root line 35, the three Compose owners at lines 143/251/280, and UI at line 541. It has one shared `remark-gfm@4.0.1` resolution at line 3707. |

The executor removes exactly the root and UI rows and exactly the two matching root/UI tuples. The three Compose rows and tuples, and the one shared `remark-gfm@4.0.1` resolution they require, remain. A retained resolution is not a reason to remove a Compose dependency or to expand this wave.

## Reachability Proof, Including Generated Autodocs

The extension receives input only through the active MDX Rollup processor. Its local `transform` hook first filters by a supported file extension; its local format-aware MDX processor recognizes Markdown extensions and `.mdx`. Local `remark-gfm` source only adds GFM Micromark/MDAST/serialization extensions to that processor. It has no independent runtime effect.

Fresh source and index evidence shows no owned processor input:

- The hidden non-Compose/non-ticket owned census contains **0 `*.mdx` files**. The Storybook glob permits `*.stories` MDX only; it has no `.md` input alternative.
- Exact static-import, dynamic-import, and CommonJS-require scans show **zero owned `.md` or `.mdx` module edges**. Repository Markdown documentation is not in the Storybook module graph.
- A fresh `bun x nx run @semio-tech/ui-react:build --skip-nx-cache` passed the permanent guard at **170 stories, 61 docs, 61 TS/TSX inputs, 0 MDX**. The generated index has exactly **231 = 170 stories + 61 docs** entries.
- All 61 docs entries are `autodocs`, each imports a unique `.tsx` input, and no docs input has an `.md` or `.mdx` extension. A scan of the installed Storybook sources found no virtual-Autodocs MDX compiler path. Thus generated Autodocs do not bypass the extension-filter proof.

The current permanent root `StorybookDiscoveryGuard` already rejects owned MDX before build and asserts the exact discovery index afterward. It needs no semantic change for this removal: it must remain a root-invoked permanent guard with the same zero-MDX scan and `231 / 170 / 61 / 61` post-build assertion.

## Differential And Existing Coverage

The pre-edit baseline is the fresh complete uncached build above. The post-edit build must reproduce precisely:

| Invariant                               | Required before and after                              |
| --------------------------------------- | ------------------------------------------------------ |
| Owned MDX files                         | `0`                                                    |
| Owned `.md`/`.mdx` import/require edges | `0`                                                    |
| Storybook index                         | `231 = 170 stories + 61 docs`                          |
| Inputs                                  | 61 unique TS/TSX; zero MDX and zero unsupported inputs |
| Generated docs                          | 61 `autodocs` entries, all from TS/TSX                 |

The accepted `@semio-tech/ui-react:test-quick` baseline is 724 tests. It remains the functional regression gate; lint and typecheck cover the changed configuration/owner graph. No GFM replacement, parser, facade, fallback, copied behavior, externalization, or compatibility shim is permitted because the transform input is unreachable.

## Executor Packet

1. Record the pre-edit zero-MDX/module-edge evidence and run the full uncached UI Storybook build. Do not proceed on a timeout, incomplete index, or changed count.
2. In `.storybook/main.ts`, remove only the `remarkGfm` import and the single `remarkPlugins` element.
3. Remove only the direct root and UI React manifest rows.
4. Run `bun install`. The expected lock delta is only the root/UI workspace tuples; retain the three Compose tuples and the one 4.0.1 resolution.
5. Do not change the permanent discovery guard unless a diagnostic text correction is strictly necessary; do not weaken its scan or exact index assertion.
6. Run the post-edit differential and all independent gates below. Stop on any count, reachability, behavior, lock, or formatting regression.

## Required Independent Audit Gates

```text
# Pre/post input and module-edge proof
find . -type f -name '*.mdx' ! -path './node_modules/*' ! -path './compose/*' ! -path './.🧬semio/*' ! -path './.git/*' -print | wc -l
rg --hidden -n -P "\b(?:from\s+|import\s*\()['\"][^'\"]+\.mdx?['\"]" --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!compose/**' --glob '!.🧬semio/**' --glob '!.git/**' .
rg --hidden -n -P "\brequire\s*\(['\"][^'\"]+\.mdx?['\"]\)" --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!compose/**' --glob '!.🧬semio/**' --glob '!.git/**' .

# Full UI behavior/configuration gates
bun x nx run @semio-tech/ui-react:build --skip-nx-cache
bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache
bun x nx run @semio-tech/ui-react:lint --skip-nx-cache
bun x nx run @semio-tech/ui-react:typecheck --skip-nx-cache

# Direct-dependency and lock gates
bun install
bun install --frozen-lockfile
bun ./📜️script.ts verify dependencies
bun ./📜️script.ts verify dependencies list js --format json
bun ./📜️script.ts verify dependencies list rust --format json
bun ./📜️script.ts verify dependencies parity js

# Exact absence/retention and hygiene
rg -n 'remark-gfm|remarkGfm' .storybook package.json '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json'
rg -n 'remark-gfm' compose/client/lib/sketchpad/js/package.json compose/client/ui/vscode/package.json compose/dev/algorithm/package.json bun.lock
bun build ./📜️script.ts --target=bun --outfile=/dev/null --external='*'
bunx prettier --check .storybook/main.ts package.json '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json' 📜️script.ts
git diff --check -- .storybook/main.ts package.json bun.lock 📜️script.ts '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json'
git diff --cached --check -- .storybook/main.ts package.json bun.lock 📜️script.ts '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json'
git diff HEAD --check -- .storybook/main.ts package.json bun.lock 📜️script.ts '🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json'
```

Acceptance requires exactly 70 JavaScript and 63 Rust direct rows, no root/UI manifest row or active binding, the unchanged permanent guard/index, frozen install, and exactly three Compose manifest/lock tuples plus one retained `remark-gfm@4.0.1` resolution. Review Prettier honestly: shared `.storybook/main.ts` or root-script baseline drift must be distinguished from a new retirement hunk, never bulk-formatted merely to pass a scoped check.

## Rejected Alternatives And Non-Goals

| Candidate or area                                                                                                        | Rejection / boundary                                                                                                                           |
| ------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `rehype-slug`, `rehype-autolink-headings`                                                                                | Adjacent MDX-only plugin bindings, but deliberately not bundled. Each requires a separate one-identity scout and implementation packet.        |
| `@mdx-js/rollup`, `@mdx-js/*`, Storybook packages                                                                        | Explicitly out of this wave; the Rollup integration remains the configuration path and Storybook is the current test/discovery infrastructure. |
| Compose                                                                                                                  | Three direct owners prove the shared resolution remains; no Compose source, manifest, lock tuple, or test is touched.                          |
| Dagre                                                                                                                    | Held for the actual Rust/Wasm/OffscreenCanvas lane; no early retirement.                                                                       |
| PostCSS/Tailwind, coverage-v8, JCO/WASM materialization, JSDOM, runtime UI primitives, `three-mesh-bvh`, and `reveal.js` | Active configuration, tooling, test, or runtime consumers rather than a smaller isolated leaf.                                                 |
| `eslint-plugin-react-hooks` and manifest-only rows                                                                       | No accepted truthful direct-use proof; not selected merely because they might lack a conventional source import.                               |
| Phase 9 candidates                                                                                                       | Not smaller/independent under the current concurrent P3/P8 Rust and Cargo constraints.                                                         |

No production source, manifest, lockfile, Compose input, Cargo input, ticket lifecycle state, or git state was changed by this read-only scout.
