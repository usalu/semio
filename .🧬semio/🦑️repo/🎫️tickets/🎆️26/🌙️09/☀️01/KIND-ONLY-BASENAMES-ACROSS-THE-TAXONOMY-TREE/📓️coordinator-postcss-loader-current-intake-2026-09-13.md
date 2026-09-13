# PostCSS Configuration Loader Intake

This read-only intake covers the remaining named React package configuration after the non-Vitest extraction. No product source changed and no PostCSS/CSS/Vite build ran.

The current `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/postcss.config.ts` contains an authored `OwnedPostcssConfig` interface and the configuration object `{ plugins: { "@tailwindcss/postcss": {} } }`. The package exports it as `./postcss.config`. The current taxonomy retains `postcss-config` as a fixed filename/disposition. A limited scan of the root router, shared repository library, React package router and package manifest found that package export as the direct current token consumer; this is not a repository-wide reachability proof.

Installed `postcss-load-config/src/index.js` supports additional `options.searchPlaces` before its conventional candidate names. Its second argument is a directory search root. Installed Vite's `resolvePostcssConfig` treats a string `css.postcss` value as that directory search root, calls the loader with only a `stopDir`, and does not forward custom search places. Therefore setting Vite's `css.postcss` string to an arbitrary anonymous filename is not a demonstrated working selector. Vite also accepts inline PostCSS options with an actual plugin array, which is a different interface from the current named-plugin object.

A coherent execution slice must first find all current package/default/Vite/editor consumers, then move actual configuration ownership under the relevant styling concern and prove the chosen native loader route. Any conventional discovery leaf that proves necessary must be exact minimal delegation with its native consumer documented; the current implemented config body is not mandatory filename glue. Do not add a broad fixed-name exemption, silently drop the plugin, or claim custom search-place support in Vite merely because the underlying standalone loader supports it. Preserve the existing package's dependency interface and test the installed PostCSS/Tailwind output on a bounded private fixture before acceptance.


## Completed Current Native Controls

The coordinator added two schema-backed private CSS vectors at `📋️postcss-loader-controls/🔣️.json` and `📋️postcss-loader-controls/🧬️schema/🔣️.json`. Installed Ajv validates the controls. Actual `postcss-load-config` loaded both the current conventional source and a byte-identical anonymous `🟦️.ts` fixture through `searchPlaces`. Both configurations equal the exact named-plugin object. Installed PostCSS/Tailwind transformed both vectors identically under Bun 1.3.14 and native Node 24.15.0; Bun's reported compatibility `process.version` v24.3.0 is not a second Node runtime. Cross-runtime complete output bytes were independently compared and match.

| Vector | Exact resulting declaration |
| --- | --- |
| utility-application | `.probe { text-decoration-line: underline; }` |
| inline-theme-application | `.probe { color: #123456; }` |

Both full outputs contain the installed Tailwind v4.3.3 license header and no remaining @apply. Native package resolution also succeeds in both runtimes for a private package whose export points to an anonymous `🟦️.mts` leaf containing only a default reexport of the semantic source. That demonstrates an anonymous, explicit-ESM package entry option without an invalid package export target outside the package root. It does not require relocating the current test barrel or copying the implementation into packaging.

The actual native Vite build control, with configFile:false, no public directory and write:false, produced these distinct observed results from the same private CSS input:

| css.postcss input | Actual Vite result |
| --- | --- |
| anonymous filename string | Build succeeds but emits `.probe { @apply underline; }`; Tailwind was not applied. |
| explicit inline plugins loaded from the anonymous config | Build succeeds and emits the expected underline declaration. |

The initial private Vite attempt set cssCodeSplit:false while using a CSS-only Rollup entry; Vite rejected that fixture option before transformation. The corrected attempt used ordinary CSS splitting and reached both results above. This was a probe configuration error, not a repository build failure. No product stylesheet, browser, Storybook or full app build was run.

A completed broader current-source/configuration token search found the package export, the configuration itself, dependency declarations, taxonomy/disposition parsers and the frozen package-purity fixture. The React dev/build router delegates to the root Storybook and does not name this PostCSS file; no additional active direct PostCSS config consumer was found in that scanned population. This does not prove absence of computed external consumers. Preserve the existing public config contract through valid anonymous package glue and actual native resolution, with all contexts/inputs and fixed-source-disposition retirement verified in the final extraction. Do not silently use the failed Vite filename-string route.

Raw outputs are private `🗑️generated/coordinator/postcss-loader/{bun,node,vite}/result.json`; complete transformed vector strings and Vite outcomes are retained in this report. The two authored JSON controls must remain after scratch disposal. No production file was changed by these controls.
