# PostCSS Loader Ownership — Independent Pre-Audit

> **Current status — preparation only.** This is a read-only map of the remaining React package PostCSS implementation and its loader boundary. The coordinator’s private Bun, Node, PostCSS, Tailwind, and Vite probes are evidence cited below; this audit ran no PostCSS, Vite, browser, or product build.

## Current implementation and explicit consumer

The current implementation is `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/postcss.config.ts`. It contains both the `OwnedPostcssConfig` type and the Tailwind plugin object `{ plugins: { "@tailwindcss/postcss": {} } }`; therefore it is semantic styling behavior, not filename-only glue.

The React package manifest is the demonstrated direct consumer: it exports `./postcss.config` as `./postcss.config.ts`. Its package dependency contract includes `@tailwindcss/postcss`, PostCSS, and Tailwind. The React dev/build package router delegates Storybook and contains no direct PostCSS configuration selection. The current scoped token scan also found taxonomy/disposition and frozen package-purity records, but no further active direct PostCSS consumer. That is not a proof against computed or external consumers.

The target semantic owner should sit in the UI styling/PostCSS concern, as an anonymous leaf. Any remaining package-local entry must be only an exact delegation to that owner. The valid anonymous package entry demonstrated by the coordinator is a package-contained `🟦️.mts` explicit-ESM default reexport; it preserves package export resolution without putting implementation in the package or using an export target outside it.

## Loader law

`postcss-load-config` can load an anonymous filename only when its caller supplies that filename through `searchPlaces`. Vite does not forward custom search places: a string `css.postcss` is interpreted as a search directory, not an arbitrary config filename. The coordinator’s private actual Vite result proves that passing an anonymous filename string succeeds while leaving `@apply underline` untransformed. It is a failed selector, not an acceptable migration route.

Vite accepts explicit inline PostCSS plugin options. When those plugins are loaded from the semantic anonymous owner, the same private input transforms correctly. This is a separate interface from the present named-plugin object and must be represented in the final consumer map. The extraction must not silently use the failed filename-string route or drop Tailwind during a successful-looking Vite build.

## Retained controls and frozen input semantics

The authored controls at `📋️postcss-loader-controls/🔣️.json` and `📋️postcss-loader-controls/🧬️schema/🔣️.json` remain the authority for two vectors. Installed Ajv validates the control data. The coordinator reports byte-identical configuration and complete CSS output under Bun 1.3.14 and native Node 24.15.0 for both the fixed current config and an anonymous fixture loaded through `searchPlaces`:

- `utility-application` produces `.probe { text-decoration-line: underline; }`.
- `inline-theme-application` produces `.probe { color: #123456; }`.

Both outputs retain the installed Tailwind 4.3.3 license header and contain no residual `@apply`. Bun’s compatibility `process.version` is not a separate Node runtime. The private Vite control uses `configFile:false`, no public directory, `write:false`, and the corrected ordinary CSS-splitting fixture. It does not prove a product Vite, browser, Storybook, or CSS application build.

The frozen package-purity authority has an existing PostCSS row at `🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json:2091-2113`, including its historical source identity, disposition, and external-tool-entry contract. Preserve its semantics and update only the exact old/new identities and disposition relationships required by the final move. Do not broaden the `postcss-config` exemption: current taxonomy describes conventional discovery at `🔣️taxonomy.json:23272-23282`, while actual Vite behavior forbids treating that filename rule as a universal runtime selector.

## Required final closure

The stable schema-first map should cover:

1. the anonymous styling owner and any literal-minimal package ESM reexport;
2. the package export, all current Vite/config/package/default consumers, and any discovered editor or dynamic source-data reader;
3. Vite’s explicit-inline plugin path separately from standalone `postcss-load-config` search-place loading;
4. package/project/Nx inputs and a registered ownership route, including the frozen fixture/control files it reads;
5. exact absence of the old implementation file and proof that a retained conventional leaf has no configuration body; and
6. current taxonomy/disposition updates without a general PostCSS exemption.

## Limits

No new runtime control ran in this audit. The coordinator’s private results establish the exact loader and transform boundaries stated here; they do not establish broad PostCSS discovery, a React/Storybook build, Vite product output, editor integration, browser rendering, or external consumer reachability. Acceptance waits for the executor’s stable owner map and source/consumer/cache/registration evidence.
