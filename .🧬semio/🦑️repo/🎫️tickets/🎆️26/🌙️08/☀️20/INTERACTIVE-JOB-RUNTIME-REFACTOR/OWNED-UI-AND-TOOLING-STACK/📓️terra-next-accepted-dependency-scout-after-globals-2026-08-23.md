# Terra Next Accepted Dependency Scout After Globals — 2026-08-23

## Decision

**ACCEPT exactly one candidate: `remark-mdx-frontmatter`.**

The live direct boundary is **136 = 73 JavaScript + 63 Rust** identities. Retiring this one tooling identity after its prescribed gate makes the in-scope boundary **135 = 72 JavaScript + 63 Rust**. This is not a manifest-only removal: one live root Storybook configuration import registers the plugin in the active `@mdx-js/rollup` pipeline. Its configured input set is empty, so removing that one transform is behavior-preserving without copying or reimplementing frontmatter semantics.

## Complete In-Scope Census

The full non-Compose, non-ticket search has exactly three in-scope rows:

| Boundary | Evidence |
| --- | --- |
| Root manifest | `package.json` declares `remark-mdx-frontmatter: ^5.2.0`. |
| UI React manifest | `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json` declares the same direct tooling row. |
| Active source | `.storybook/main.ts:20` imports the plugin; line 171 registers it in the `remarkPlugins` array passed to `@mdx-js/rollup`. |

No other non-Compose/non-ticket source consumer, indirect adapter, or configuration use exists. Storybook's configured globs permit `.mdx`, but an exhaustive hidden-file census outside `node_modules`, `compose`, and `.🧬semio` found **zero `*.mdx` files**. Thus the transform has no current document to receive, and TS/TSX stories remain served by the unchanged Storybook pipeline.

Out-of-scope Compose manifests also directly declare the package (`compose/client/lib/sketchpad/js`, `compose/dev/algorithm`, and `compose/client/ui/vscode`). They are intentionally neither edited nor counted by the dependency verifier. They explain why the resolved lock package must remain after the in-scope direct rows are removed.

## Exact Replacement Boundary

Remove only:

1. the `remarkMdxFrontmatter` import in `.storybook/main.ts`;
2. its one element in `mdx.default({ remarkPlugins: [...] })`;
3. the two non-Compose direct manifest rows above.

The replacement is the existing `@mdx-js/rollup` Storybook path with the empty frontmatter-transform stage omitted. Do not add an owned frontmatter parser, compatibility facade, or copied behavior: no in-scope MDX input invokes this feature.

## Required Pre-Edit Differential And Permanent Proof

Before modifying source, record these two proofs against the shared tree:

1. Run an exhaustive hidden-file MDX census with the stated exclusions and assert zero files. This is the differential's input invariant: the old pipeline and the pipeline without this transform process the same empty MDX set.
2. Run the bounded UI Storybook build under its permanent Nx target and record the discovered story list plus build success. After the removal, repeat the same target; the set of discovered TS/TSX stories and success result must be identical. Do not count a timed-out or incomplete build as a pass.

Add a permanent Storybook discovery assertion in the existing root Storybook/verify path that fails when an in-scope `.mdx` file appears while the transform is retired. Keep the existing `*.mdx` glob only if that guard fails loudly; otherwise narrow the glob to the supported TS/TSX story extensions. The permanent gate must run through Nx and prove both the empty MDX census and unchanged TS/TSX discovery.

Then run `bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache`, the relevant Storybook Nx build uncached, UI lint/typecheck, frozen install, `verify dependencies`, JS list/parity, literal absence scans, Prettier for parseable changed files, and scoped working/staged/HEAD diff checks.

## Expected Manifest And Lock Result

Remove the root and UI React direct-manifest rows, then reconcile only with Bun. The two matching workspace tuples in `bun.lock` must lose `remark-mdx-frontmatter`; its package resolution must remain because the three excluded Compose workspace manifests still require it. No transitive `@mdx-js/mdx` removal is expected: `@mdx-js/rollup` continues to require it. Frozen install and JS parity must confirm this rather than relying on a predicted lock diff.

## Explicit Deferrals

- **Sharp — DEFER.** `@semio-tech/print:test-quick` still fails before Sharp behavior with `ENOENT`. The data exists at `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️tokens.json`; `🎨print-design-token-paints/🟦️component.ts:69` incorrectly inserts `📦️packages/🦀️rust/`. Repair that path first, then run an actual panel-glass old-versus-owned image differential before considering Sharp. Do not accept it from static inspection.
- **Dagre — HOLD.** It remains installed until the required Rust/Wasm/OffscreenCanvas lane and its acceptance gate complete.
- **`eslint-plugin-react-hooks`, `its-fine`, `jose`, `jsonc-parser`, and `@types/reveal.js` — DEFER.** Current in-scope scans show no direct executable consumer; selecting any would be manifest-only.
- **React resizing, i18n, PDF/canvas, PostCSS/Tailwind, root lint/build/Nx, and MDX/GFM transforms other than the selected empty frontmatter stage — DEFER.** Each retains a live non-empty runtime or tooling role that lacks this candidate's empty-input proof.

No production source, manifest, lockfile, ticket metadata, checklist, Cargo, Compose, Dagre, or cache state was edited by this scout.
