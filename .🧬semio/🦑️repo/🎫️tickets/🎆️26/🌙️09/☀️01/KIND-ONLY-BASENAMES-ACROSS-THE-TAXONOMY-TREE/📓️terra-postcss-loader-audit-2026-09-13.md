# PostCSS Loader Extraction — Independent Acceptance Audit

> **Current status — accepted for the bounded PostCSS/configuration extraction.** Static closure is independently checked; the executor's current full ordinary and isolated registered routes are green. This audit did not duplicate them.

## Current semantic boundary

The active implementation is `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🎨️styling/🟦️.ts`. It owns the PostCSS configuration object, the Tailwind plugin dependency, and the explicit Vite plugin-array factory. It exports only repository-owned data/function types (`OwnedPostcssConfiguration`, `uiPostcssConfiguration`, `uiPostcssInlinePlugins`, and the default object); no external PostCSS type escapes its API.

The package leaf `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.mts` is anonymous glue containing exactly:

```ts
export { default } from "../../🛠️build-tooling/🎨️styling/🟦️.ts";
```

`package.json` maps `./postcss.config` to `./🟦️.mts`. The old fixed `postcss.config.ts` is absent. This distinguishes an executable semantic owner from the package-resolution leaf and leaves no domain-named implementation basename.

## Loader and consumer closure

The tool-configuration schema/fixture/test identify twelve owners, twenty-two consumer/source-data records, and eight project-input bindings. PostCSS has a direct source-data record for the package glue and a React project default-input record for the semantic owner. The ownership target independently includes the owner, anonymous package entry, package manifest, frozen historical authority, schema, fixture, test, and React project manifest.

The two actual loader protocols are intentionally separate:

- `postcss-load-config` receives the explicit `searchPlaces: ["🟦️.mts"]` selector and loads the anonymous package entry;
- Vite receives `uiPostcssInlinePlugins()` as an inline plugin array. It never receives an anonymous filename string, which would silently leave Tailwind directives untransformed.

The installed-tool test checks both Tailwind vectors: utility `@apply underline` and inline-theme `@apply text-taxonomy`, requiring concrete transformed declarations and rejecting residual `@apply`. It also checks Bun and Node package export resolution. Its Vite build is private, `configFile: false`, and `write: false`.

The preserved `remaining-package-purity-authority` row remains historical provenance (`non-authoritative-concurrent-source-byte-drift`) rather than a new all-PostCSS exemption. The current semantic owner differs from the retired fixed filename, while the frozen row remains available for identity review.

## Static observations

I independently read the owner, `.mts` entry, package export, fixture, schema, ownership test, target inputs, and launch registrations. The current source satisfies the required separation and contains no stale live `postcss.config.ts` consumer. The launch seed and derived launch both register `🧹clean🧩️taxonomy🎚️tool-configuration-ownership`, and the package script delegates through Nx to the same registered target.

The schema’s PostCSS section fixes the owner id, anonymous `.mts` entry, package export, explicit standalone selector, inline Vite factory, and two behavior cases. It thereby rejects a fixed configuration filename or a Vite filename-string substitution rather than merely detecting a moved source file.

## Current execution evidence

The executor's full ordinary package route passed **9 tests and 171 assertions in 6.72 seconds**. Its isolated registered route,
`bun nx run @semio-tech/repo-lib:test-tool-configuration-ownership --skip-nx-cache`, passed **9/171** with `NX_DAEMON=false`, `NX_ISOLATE_PLUGINS=false`, and ticket-private workspace/cache/tmp roots; test time was **18.46 seconds**, Nx target time **19.3 seconds**, cache skipped, exit 0.

Installed Node 24.15.0 and Bun 1.3.14 each resolved the anonymous package entry through the explicit standalone selector and produced the two exact Tailwind 4.3.3 outputs, with shared digest `e02af85dbf7d53cb716f6fd43212728051ab01972383af868b6458ed8aa250fd`. The focused Vite `write:false` inline-plugin control also passed within that suite. Ticket-control schema v2 validated both retained cases with Ajv.

The final source map corrected two stale consumers from the earlier 23-row checkpoint: the OS Vite config record now uses `🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts`, which supplies the semantic Vite config to `runViteBunxDev`, and a concurrently removed VS Code README record is no longer represented. The final map is therefore 12 owners, 22 consumers, and 8 project bindings.

I accept the anonymous PostCSS owner and package glue, explicit standalone and Vite protocols, source-data/cache/launch closure, and bounded installed-tool evidence. No dev server, production Vite output, package publication, browser journey, or external consumer reachability is claimed.
