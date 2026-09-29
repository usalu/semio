# Anatomy of `🧰️framework/🛍️products/🎤️presentation` and its consumers

Exploration for cloning the presentation product's structure into a new `🧰️framework/🛍️products/<emoji>quiz`
product (plus a renderer target and `teaching/…/quiz` domain content + `teaching/proctor` server). All paths are
relative to `C:\git\semio` unless stated otherwise. All findings below were verified by reading the actual
tracked files (`git -c core.quotepath=false ls-files`, `git grep`), not inferred.

---

## 1. Full file tree of `🧰️framework/🛍️products/🎤️presentation` (git-tracked)

```
🧰️framework/🛍️products/🎤️presentation/
├── README.md
├── 🟦️.ts                                         # product barrel — re-exports the core package
├── 🔣️oracle.json                                 # legacy/simple oracle registry (schemaVersion 1)
├── 🔮️oracles/🔣️.json                             # canonical oracle registry (schemaVersion 2, repo test-platform schema)
├── 📦️packages/🟦️typescript/
│   ├── package.json                              # @semio-tech/presentation (bundleKind: library)
│   ├── 📋️project.json                            # nx targets: test / test-quick / test-long / test-exhaustive
│   ├── 📜️script.ts                                # bun router: `bun ./📜️script.ts test [level]`
│   ├── 🟦️.ts                                     # THE MODEL — 2012 lines, zero runtime imports
│   └── 🎯️targets/⚛️react/
│       ├── package.json                          # @semio-tech/presentation-react (bundleKind: ui)
│       ├── 📋️project.json                        # nx targets: test / test-quick / test-long / test-exhaustive
│       ├── 📜️script.ts                            # bun router (same pattern as core)
│       ├── 🟦️.tsx                                # THE RENDERER — 5550 lines, React + reveal.js
│       ├── 🎨️.css                                 # 1406-line stylesheet, exported as "./🎨️.css"
│       ├── 🧰️vitest.setup.ts                      # jsdom polyfills + pdfjs-dist mock for renderer tests
│       └── 🔨️modules/
│           ├── 📝️markdown-html-compiler/🟦️.ts     # 359-line owned CommonMark/GFM-subset → HTML compiler
│           └── 🔌️pdf-canvas-port/🟦️.ts             # 115-line port (interfaces) + PdfCanvasResourceOwner lifecycle class
├── 🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts        # vitest config for the RENDERER's product-level test dir (see note below)
├── 🧪️tests/🎚️config/🟦️.ts                          # vitest config for the CORE product-level test dir
├── 🧪️tests/🎞️presentation-react-deck/🟦️.tsx         # 3862-line renderer test suite (register*Tests pattern)
├── 🧪️tests/📝️markdown-html-compilation/
│   ├── 🟦️.ts                                     # test adapter (defineTestAdapter) — differential vs `remark`
│   └── 🥒️.feature                                # Gherkin scenarios (oracle-tagged)
├── 🧪️tests/📝️owned-markdown-compiler/🟦️.ts         # plain vitest unit tests for the compiler
├── 🧪️tests/📽️presentation-core/🟦️.ts               # core model unit tests (intro/analogy/split/morph/…)
├── 🧪️tests/🔌️pdf-canvas-port/🟦️.ts                 # PdfCanvasResourceOwner + status/bitmap-size unit tests
├── 🧪️tests/🧭️slide-glob-assembly/🟦️.ts              # loadPresentationFromSlideGlob recovery test
└── 🧫️fixtures/📝️markdown-html-compilation/*.md     # 30 markdown fixture files (one per Gherkin vector)
```

**Important structural quirk to replicate exactly**: test-config files for a target live in a *mirrored* path
one level up from the target, not inside the target's own folder. I.e.

- Core config: `🧰️framework/🛍️products/🎤️presentation/🧪️tests/🎚️config/🟦️.ts` (sibling of `📦️packages/`)
- React-target config: `🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` — this
  is **not** under `📦️packages/🟦️typescript/🎯️targets/⚛️react/`; it is a parallel `🎯️targets/⚛️react/🧪️tests/…`
  tree hung directly off the product root. The `🎚️vitest-configuration-ownership` fixture (§4) confirms this is
  intentional ("ownerPath") and is what a repo policy checks for.

### Per-file purpose (the files actually read)

| File | Purpose |
|---|---|
| `README.md` | Product overview: layout table, commands, and the full domain-model glossary (Presentation → Column/Row → Chapter → Sequence → Thought → Participant/Embodiment/Figure/Video/Text/Pdf/Demo → Disposition → Morph (OneToOne/OneToMany/ManyToOne) → Arrangement → Transition → Slide → Template (Intro/Tile/Split)). |
| `🟦️.ts` (product root) | One-line barrel: `export * from "./📦️packages/🟦️typescript/🟦️.ts";` |
| `🔣️oracle.json` | Minimal oracle list (`id`, `package`, `version`, `entry`, `covers`, `reason`) — appears to be an older/simpler sibling of `🔮️oracles/🔣️.json`. Both exist simultaneously for this product; the newer one is richer (see next). |
| `🔮️oracles/🔣️.json` | Canonical registry per `🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json` — declares the `remark` oracle stack (`remark-parse`+`remark-gfm`+`remark-rehype`+`rehype-stringify` composed by `unified`) that the markdown compiler tests diff against; carries license/homepage/`hostPath`/`productionReachable`/`comparisonProfiles`/`migrationStatus: "parity-green"`. |
| `📦️packages/🟦️typescript/package.json` | `@semio-tech/presentation`, `type: module`, `private: true`, `exports: {".": "./🟦️.ts"}`, `bundleKind: "library"`, `semio.role: "framework"`, `semio.id: "presentation"`, devDeps only `typescript` + `vitest`. |
| `📦️packages/🟦️typescript/📋️project.json` | nx targets `test`/`test-quick`/`test-long`/`test-exhaustive`, all `nx:run-commands` → `bun ./📜️script.ts test [level]` with `cwd` set to the package dir. `namedInputs.default` lists the package's own files plus the three test files it owns. |
| `📦️packages/🟦️typescript/📜️script.ts` | `class TestScript extends BundleScript` from the shared `📚️library` script helpers; `resolveTestLevel` + `runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts")`; registered on a `ScriptRouter` with `defaultCommand: "test"`. |
| `📦️packages/🟦️typescript/🟦️.ts` | The declarative model. Zero runtime imports (no `import` of any runtime package — only `node:*`-free pure TS). Exports the full domain vocabulary (types + pure functions), described in §2. |
| `🎯️targets/⚛️react/package.json` | `@semio-tech/presentation-react`, `bundleKind: "ui"`, `exports: {".": "./🟦️.tsx", "./🎨️.css": "./🎨️.css"}`; runtime deps: `@semio-tech/framework`, `@semio-tech/presentation`, `@semio-tech/ui-react` (all `workspace:*`), `pdfjs-dist`, `react`, `react-dom`, `reveal.js`; devDeps include the whole remark/unified oracle stack plus `jsdom`/`@vitejs/plugin-react`/`vitest`. |
| `🎯️targets/⚛️react/📋️project.json` | Same 4-target shape as core; `namedInputs.default` additionally lists the core model file and the react target's own test-config + the two test dirs it consumes (`🔌️pdf-canvas-port`, `📝️owned-markdown-compiler`). |
| `🎯️targets/⚛️react/📜️script.ts` | Same router pattern; points `runVitest` at `"../../../../🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts"` (the mirrored config path noted above). |
| `🎯️targets/⚛️react/🟦️.tsx` | The React + reveal.js renderer. 5550 lines, ~200 exported functions/consts/types organized under `// #region` markers: Markdown, MountOptions, RevealChrome, RevealMorph, ArrangementSettled, HiddenPreflight, MorphView, SlideEpoch, PdfCanvasPort, Interaction, ArrangementSection, PresentationInteractionProvider, PresentationDeck (`mountPresentation`/`unmountPresentation`), json/Renderer. Imports `@semio-tech/framework` (`ephemeralBox`), `@semio-tech/presentation` (types + pure functions), `@semio-tech/ui-react` (design-system primitives: `Icon`, `Scrollable`, `SelectionMarquee`, `applyElementsSurfaceChrome`, React re-exports), `reveal.js` + its CSS, the local `🎨️.css`, and the two `🔨️modules/` ports. |
| `🎯️targets/⚛️react/🎨️.css` | 1406-line stylesheet; all classes prefixed `presentation-*` (e.g. `presentation-arrangement--interactive`, `presentation-morph-slot--figure`, `presentation-figure-scroll-viewport--axis-x`). Imported only from the `.tsx` (`import "./🎨️.css"`) and re-exported as a package subpath export. |
| `🎯️targets/⚛️react/🧰️vitest.setup.ts` | jsdom polyfills (`matchMedia`, `DOMMatrix`, `Path2D`, `HTMLMediaElement.play/pause`, relative-URL `fetch` stub keyed by extension, `Promise.withResolvers`, canvas 2D context stub, `ResizeObserver` stub, `PointerEvent` polyfill, `Range.getClientRects` polyfill) plus `vi.mock("pdfjs-dist", …)`. |
| `🔨️modules/📝️markdown-html-compiler/🟦️.ts` | Hand-written CommonMark/GFM-subset parser (`Inline`/`Block`/`Document` schema) + safe HTML serializer. URL-scheme allow-list (`http`,`https`,`mailto`,`tel`). Single export: `compileOwnedMarkdownToHtml(markdown): Promise<string>`. No third-party markdown library — this *is* the "own it, oracle-test it" pattern the AGENTS.md rule about external libraries describes. |
| `🔨️modules/🔌️pdf-canvas-port/🟦️.ts` | A **port** (hexagonal-architecture boundary): `PdfCanvasViewport`/`PdfCanvasPage`/`PdfCanvasDocument`/`PdfCanvasLoadingTask`/`PdfCanvasRenderTask`/`PdfCanvasPort` interfaces owned by the workspace, satisfied at runtime by `pdfjs-dist` (injected via `setPdfCanvasPort` in the `.tsx`, defaulted lazily). `PdfCanvasResourceOwner` class enforces disposal order (cancel render → cleanup page → destroy document/loading-task). This is the concrete example of "use external libraries behind an interface" from AGENTS.md. |
| `🧪️tests/🎚️config/🟦️.ts` | Plain object (not `defineConfig`) vitest config for the core package: `environment: "node"`, `include` the two core test files, `resolve.alias` maps `@semio-tech/presentation` → the package's own `🟦️.ts`, `coverage.include: ["🟦️.ts"]`, `passWithNoTests: false`. |
| `🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` | `defineConfig` vitest config for the react target: `environment: "jsdom"`, `plugins: [react()]`, a full `resolve.alias` list wiring `@semio-tech/presentation-react`, `@semio-tech/presentation`, `@semio-tech/framework`, `@semio-tech/ui-react`(+`/test`), `@semio-tech/animate-presentation-core`, and even the **consumer's** spec module (`@semio-tech/mit-bestand-praesentation-projektetage-spec` → `../../♻️mit-bestand/…/🔖️spec.ts`) so renderer tests can exercise the real projektetage catalogue data; `setupFiles: [".../🧰️vitest.setup.ts"]`. |
| `🧪️tests/🎞️presentation-react-deck/🟦️.tsx` | The big renderer test file. Exports `registerPresentationReactDeckTests(vitest, dependencies, source)` — a **language-agnostic test registration pattern**: it takes the live `vitest` module object and a `Pick<...>` of ~90 named exports from the `.tsx` module, so the *same* test bodies can run regardless of how the module is loaded. Covers `resolveRevealArrangement`, `PresentationDeck` DOM assertions (jsdom), CSS-content assertions (reads `🎨️.css` at test time and regex-matches selectors), pdf paging, figure scroll/zoom math, split/tile morphs, etc. |
| `🧪️tests/📝️markdown-html-compilation/🟦️.ts` | `defineTestAdapter({ implementation: "typescript", scenarios: {...} })` from the repo test-platform. Each scenario (`prose`, `inline`, `lists`, `links`, `code`, `tables`, `slide`) compiles the same markdown fixture with `compileOwnedMarkdownToHtml` (subject) and a `unified().use(remarkParse).use(remarkGfm).use(remarkRehype).use(rehypeStringify)` pipeline (oracle), then diffs JSON projections keyed `markdown/<vector>`. |
| `🧪️tests/📝️markdown-html-compilation/🥒️.feature` | Gherkin feature tagged `@capability-presentation-markdown-html @oracle-remark @comparison-ordered-json-v1`. 7 scenarios, each tagged `@id-<name> @level-<fundamental\|quick\|long> @mode-differential`, each with a `Given ... vectors` data table of `vector`/`fixture` pairs using `shared://📝️markdown-html-compilation/<file>.md` URIs. Long prose docstring explains *why* an oracle is needed and exactly which CommonMark/GFM constructs are and are not claimed. |
| `🧪️tests/📝️owned-markdown-compiler/🟦️.ts` | Plain `describe/it` vitest unit tests (not oracle-differential) — fixed fixtures (prose/fenced-code/lists/table), malformed-input determinism, and URL-scheme rejection (`javascript:`, `data:` dropped; `mailto:`/relative kept). |
| `🧪️tests/📽️presentation-core/🟦️.ts` | Large plain-vitest suite for the core model: `loadPresentationFromSlideGlob` (chapters/sequences/thoughts assembly + thought-template expansion), `intro`/`analogy` templates, `resolveArrangement`/`morphId`, `split`/`splitFigureGrid`/`unionSourceCrops`, `expandThoughtSlides` (auto-animate run grouping incl. fade breaks and no-shared-participant breaks), `centerResolvedArrangement`, `figureFrameForSourceAspect`, hash bookmark round-trips (incl. German query-key localization), tile-play helpers. |
| `🧪️tests/🔌️pdf-canvas-port/🟦️.ts` | Unit tests for `PdfCanvasResourceOwner` disposal ordering + superseded-load rejection, and `pdfCanvasStatusAnnouncement`/`pdfCanvasBitmapSize`. |
| `🧪️tests/🧭️slide-glob-assembly/🟦️.ts` | One recovery-style test: builds a 2-slide glob, asserts chapter/sequence/thought naming and `resolveArrangement` output. |
| `🧫️fixtures/📝️markdown-html-compilation/*.md` | 30 tiny markdown files, one per Gherkin vector (`paragraph.md`, `headings.md`, `emphasis-asterisk.md`, `list-nested.md`, `table-aligned.md`, `slide.md`, …). |

**Test runner**: `vitest` (v4) everywhere in this product — not `bun test`. (`bun test` / `bun:test` typings are used
elsewhere in the repo, e.g. `📚️library`, but the presentation product's own suites run under vitest via
`runVitest(...)` from the shared script helpers.) Package manager is `bun`; task runner is `nx` via
`bun nx run <project>:<target>`, and every target simply shells out to that package's own `📜️script.ts`.

**Docstring style** confirmed throughout: emoji-first `/** @emoji 🎤️ ... */` block comments, `@link {@link Foo}`
cross-references, and `// #region 🧲️Header` / `//#region 🔖️Name` folding markers grouping related exports. No
comments inside function/type bodies (matches the AGENTS.md rule).

---

## 2. How the model is declared

- **No JSON Schema.** There is no `🧬️schema/🔣️.json` (or similar) for the presentation domain model. "Schema-first"
  here means: TypeScript `interface`/`type` declarations in `📦️packages/🟦️typescript/🟦️.ts` *are* the schema — hand
  written, not generated from nor generating a JSON Schema. (Contrast with other repo areas, e.g. the pdf artifact
  schema mutations, which are schema-first with actual `🔣️.json` files — this product does not follow that
  pattern.) If the quiz product should be schema-first with a real JSON Schema artifact, that is a deliberate
  deviation from the presentation model to plan for explicitly, not something to copy.
- **Types are hand-derived, not code-generated.** Every interface (`Participant`, `Embodiment` union,
  `Disposition`, `Arrangement`, `Slide`, `Thought`, `Sequence`, `Chapter`, `Presentation`, `SlideFile`, …) is written
  directly in the `🟦️.ts` barrel; pure functions operate on plain data (no classes in the core model — the only
  class in the whole product is `PdfCanvasResourceOwner` in the react target's port module).
- **Zero-runtime-import rule**: `📦️packages/🟦️typescript/🟦️.ts` imports nothing at runtime — no `react`, no
  `reveal.js`, no `node:*`. It is pure data + pure functions. The renderer (`🎯️targets/⚛️react/🟦️.tsx`) is the
  *only* place that imports React/reveal.js/pdfjs-dist, and even there `pdfjs-dist` is kept behind the
  `PdfCanvasPort` interface. The README states this explicitly: "The model knows nothing about the DOM; the
  renderer is one target among possible others."
- **i18n**: the model supports exactly two "main languages", `PresentationLanguageKind = "de" | "en"`, set once per
  deck via `Presentation.language` (optional, defaults to `"en"` via `presentationLanguage()`). This does **not**
  mean per-string translation tables — a deck's slide *content* (titles, bullets, etc.) is authored directly in
  whichever language the author wrote it in (see the projektetage deck, entirely German). What *is* localized by
  `language` is the **bookmark vocabulary**: `presentationSlideBookmarkParamKeys("de")` swaps the URL query keys
  `chapter/sequence/thought/slide` for `kapitel/sequenz/gedanke/folie`, and the `intro()` template's generated
  chapter/sequence/thought/arrangement bookmark *names* (e.g. "Main"/"Hauptteil", "Title"/"Titel",
  "Faculty"/"Fakultät") switch by a lookup table keyed on the same union. So: no default language (AGENTS.md rule
  honored — `language` is optional and every localized lookup takes an explicit fallback), English first / German
  second in every such table, exactly as AGENTS.md prescribes.

---

## 3. How consumers mount it — `♻️mit-bestand/🎤️präsentation/📅️33.projektetage`

Full tracked tree (`git ls-files`) groups into: `🌐️public/` (52 static assets — images/pdf/video/html/json, incl.
a vendored `_files/` bundle for an embedded third-party page), `🎞️slide/**/*.ts` (29 slide/thought module files
under a deep chapter/sequence/thought folder hierarchy with emoji-prefixed, human-titleized folder names — German),
`🎨️globals.css` (Tailwind entry, imported from the deck module), `🏗️builder/🌐️vite/🟦️.ts` (Vite config),
`📦️packages/🟦️typescript/` (the app bundle: `package.json`, `🌐️.html`/`🌐️index.html`, `📋️project.json`,
`📜️script.ts`, `📦️index.ts`, `🔖️spec.ts`, `🟦️.ts`), `🧪️tests/🎚️config/🟦️.ts` + `🧪️tests/🧪️projektetage-deck/🟦️.ts`.

### `📦️packages/🟦️typescript/package.json` (verbatim, relevant fields)
```json
{
  "name": "@semio-tech/mit-bestand-praesentation-projektetage",
  "dependencies": {
    "@semio-tech/presentation": "workspace:*",
    "@semio-tech/presentation-react": "workspace:*"
  },
  "devDependencies": {
    "@tailwindcss/vite": "^4.1.18",
    "@vitejs/plugin-react": "^5.1.2",
    "typescript": "^5.9.3",
    "vite": "^7.3.1",
    "vitest": "^4.0.17"
  },
  "bundleKind": "application",
  "exports": { ".": "./🟦️.ts" },
  "semio": {
    "app": {
      "kind": "projektetage",
      "aliases": ["projektetage"],
      "packageRoot": "mit-bestand/präsentation/33.projektetage",
      "corePackage": "@semio-tech/mit-bestand-praesentation-projektetage",
      "definitionExport": "projektetagePlayAppDefinition",
      "hostKind": "projektetage",
      "port": { "dev": 6050, "env": "PRAESENTATION_PROJEKTETAGE_PORT" }
    }
  }
}
```
Note the `semio.app` block — this is how a *deployable app* (as opposed to a library/product target) self-registers
its dev port, its "play app" definition export name, and its CLI alias. `presentationPlayAppDefinition` (exported
from the core model, aliased `animatePlayAppDefinition`) is re-exported by the deck module as
`projektetagePlayAppDefinition` — a minimal `{ id, label, controllerId, modes, defaultModeId, devHost }` stub.

### `📋️project.json` (verbatim)
```json
{
  "name": "@semio-tech/mit-bestand-praesentation-projektetage",
  "namedInputs": {
    "default": [
      "{projectRoot}/**/*",
      "{workspaceRoot}/🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts",
      "{workspaceRoot}/♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🏗️builder/🌐️vite/🟦️.ts",
      "{workspaceRoot}/♻️mit-bestand/🎤️präsentation/📅️33.projektetage/slide/**/*.ts",
      "{workspaceRoot}/♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/**/*",
      "{workspaceRoot}/♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🧪️tests/🎚️config/🟦️.ts"
    ]
  },
  "targets": {
    "dev": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript",
        "command": "bun ./📜️script.ts dev",
        "env": { "PRAESENTATION_PROJEKTETAGE_PORT": "6050", "NODE_OPTIONS": "", "NX_NATIVE_COMMAND_RUNNER": "false", "NX_TASKS_RUNNER_DYNAMIC_OUTPUT": "false", "NX_TUI": "false", "VSCODE_INSPECTOR_OPTIONS": "" },
        "forwardAllArgs": true
      },
      "cache": false,
      "continuous": true
    },
    "build": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript",
        "command": "bun ./📜️script.ts build",
        "forwardAllArgs": true
      },
      "cache": true,
      "outputs": ["{projectRoot}/dist"]
    }
  }
}
```
(There is no `test` target block here even though the app has tests — `dev`/`build` only; the app's tests run
through the same `📜️script.ts test` router but nx doesn't get a dedicated `test` target entry in this particular
`project.json`. Worth double-checking whether that's deliberate before copying blindly.)

### `📜️script.ts` (verbatim)
```ts
#!/usr/bin/env bun
/** 🧭️ `@semio-tech/mit-bestand-praesentation-projektetage` task router: `bun ./📜️script.ts <dev|build|test> [args…]`. */
import { BundleScript, ScriptRouter, playgroundDevPortString, playgroundPortEnv, resolveTestLevel, runBundleScriptMain, runViteBunxDev, runViteBuild, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runViteBunxDev(this.root, segments, {
      config: "../../🏗️builder/🌐️vite/🟦️.ts",
      portEnv: playgroundPortEnv("projektetage"),
      defaultPort: playgroundDevPortString("projektetage"),
      fixedPort: true,
    });
  }
}
class BuildScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.some((arg) => /^(?:--outDir|--config|--root)(?:=|$)/.test(arg))) throw new Error("Build output and configuration are owned by this Nx target");
    runViteBuild(this.root, segments, "../../🏗️builder/🌐️vite/🟦️.ts");
  }
}
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}
const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url);
```
`playgroundPortEnv("projektetage")`/`playgroundDevPortString("projektetage")` derive the env-var name and default
port string from the app's `semio.app.kind`/alias — this is the shared "playground" convention referenced in
`🧰️framework/🛍️products/AGENTS.md` ("Playground: … driven by each domain's `PlaygroundAppDefinition`").

### Vite config (`🏗️builder/🌐️vite/🟦️.ts`, verbatim)
```ts
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import { semioAssetsVitePlugin, semioEmojiIndexHtmlVitePlugin, semioHostHtmlVitePlugin, playgroundStaticSiteBuildOptions, semioServeCloseVitePlugin } from "../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";

const dir = dirname(fileURLToPath(import.meta.url));
const bundleRoot = resolve(dir, "../../📦️packages/🟦️typescript");
const repoRoot = resolve(bundleRoot, "../../../../..");

export default defineConfig({
  root: bundleRoot,
  base: "./",
  publicDir: resolve(bundleRoot, "../../🌐️public"),
  plugins: [
    semioServeCloseVitePlugin(),
    ...semioHostHtmlVitePlugin(repoRoot, { title: "33. Projektetage", entry: "./🟦️.ts", bodyClass: "h-screen w-screen overflow-hidden", cnameHost: "33.projektetage.zukunft-bau.mit-bestand.de" }),
    semioEmojiIndexHtmlVitePlugin(bundleRoot),
    ...semioAssetsVitePlugin(repoRoot),
    tailwindcss(),
    react(),
  ],
  build: playgroundStaticSiteBuildOptions(),
  define: { "import.meta.vitest": "undefined" },
  server: { fs: { allow: [repoRoot] } },
  resolve: {
    alias: [
      { find: "@semio-tech/animate-presentation-core", replacement: resolve(repoRoot, "✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/⚡️implementations/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/animate-js", replacement: resolve(repoRoot, "✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/presentation", replacement: resolve(repoRoot, "🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/presentation-react", replacement: resolve(repoRoot, "🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx") },
      { find: "@semio-tech/ui-react", replacement: resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx") },
      { find: "@semio-tech/framework", replacement: resolve(repoRoot, "🧰️framework/📦️packages/🟦️typescript/🟦️.ts") },
      { find: "@semio-tech/mit-bestand-praesentation-projektetage-spec", replacement: resolve(bundleRoot, "🔖️spec.ts") },
    ],
  },
});
```
Static-site output goes through `playgroundStaticSiteBuildOptions()` (shared helper) — `nx build` output is
`{projectRoot}/dist`. Cross-package resolution here is **not** TS `paths`, it's explicit Vite `resolve.alias`
entries per consumer, always pointing straight at the producing package's real entry file (bypassing bun's own
node-modules symlink resolution). This same alias list (minus app-specific ones) is repeated near-verbatim in the
react target's own `🧪️tests/🎚️config/🟦️.ts` (§1) and in this app's own `🧪️tests/🎚️config/🟦️.ts`.

### Two HTML entry points exist
- `📦️packages/🟦️typescript/🌐️index.html` — loads `<script type="module" src="./🟦️.ts">` directly (real entry,
  matches the Vite config's `entry: "./🟦️.ts"` passed into `semioHostHtmlVitePlugin`).
- `📦️packages/🟦️typescript/🌐️.html` — an alternate/legacy-looking template that references `./js/index.ts`
  (does not match current file layout — likely stale; verify before copying).

### Deck assembly (`📦️packages/🟦️typescript/🟦️.ts`, key mechanics)
```ts
import "../../🎨️globals.css";
import { BAUKOMPONENTEN_ITEMS, ... presentationMeta, ... } from "./🔖️spec.ts";
export * from "./🔖️spec.ts";

const slideModuleLoaders = import.meta.glob<{ default: SlideFile }>("../../🎞️slide/**/*.ts");
const slideModules = Object.fromEntries(await Promise.all(Object.entries(slideModuleLoaders).map(async ([path, loadModule]) => [path, await loadModule()] as const)));
const sourceDeck: Presentation = loadPresentationFromSlideGlob(presentationMeta, slideModules);

const CHAPTER_ORDER = ["Einführung", "Recherche", "Bauteilportal", "Entwurfswerkzeug"] as const;
function reorderChapters(presentation) { /* reorders chapters map to this fixed order, throws if one is missing */ }
export const deck: Presentation = addZukunftBauBookends(reorderChapters(sourceDeck));

function mount(): void {
  const el = document.getElementById("root");
  if (!el) return;
  void Promise.all([import("@semio-tech/animate-js"), import("@semio-tech/ui-react")]).then(([{ mountPresentation }, { DEFAULT_UI_DRIVER }]) => {
    mountPresentation(el, deck, { transition: "fade", slideNumber: false, surfaceChrome: { appearance: "dark", device: "desktop", driver: DEFAULT_UI_DRIVER } });
  });
}
if (typeof document !== "undefined" && !import.meta.vitest) mount();

export { presentationPlayAppDefinition as projektetagePlayAppDefinition } from "@semio-tech/presentation";

if (import.meta.vitest) {
  const { registerProjektetageDeckTests } = await import("../../🧪️tests/🧪️projektetage-deck/🟦️.ts");
  await registerProjektetageDeckTests(import.meta.vitest, { ...exports... }, { directory: import.meta.dir, url: import.meta.url });
}
```
- Slides are declared as a **file-system-driven glob**: `import.meta.glob("../../🎞️slide/**/*.ts")`, then handed
  to `loadPresentationFromSlideGlob(meta, modules)` from the core model, which parses each import path as
  `🎞️slide/<chapter>/<sequence>/<thought>/<slide>.ts` (or `<thought>.ts` for a template file, e.g. the intro
  thought at `🎞️slide/🌷️Einführung/🪻️Einleitung/👋️Einleitung.ts` which is a 4-line file:
  `export default introThoughtFile(introSpec);`) and assembles the chapter/sequence/thought/slide tree, sorting by
  `SlideFile.order` within a thought.
- The `import.meta.vitest` guard is the repo-wide in-source-testing convention: the same module both mounts in the
  browser and, under vitest with `includeSource`, dynamically imports and registers its own test suite — this is
  why the react-target vitest config needs `includeSource: ["🟦️.ts"]` and `define: { "import.meta.vitest": "undefined" }` in the Vite build config to strip it from the production bundle.
- `🔖️spec.ts` is kept as a **separate file from the entry `🟦️.ts`** specifically to avoid the entry's top-level
  slide `await` from cycling into a slide file's static import of the spec (see its header docstring) — slide
  files import catalogue/meta constants from `@semio-tech/mit-bestand-praesentation-projektetage-spec` (aliased to
  `🔖️spec.ts`), never from the entry.

### `🧪️tests/🎚️config/🟦️.ts` (verbatim)
```ts
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import { semioAssetsVitePlugin } from "../../../../../🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts";

const dir = resolve(dirname(fileURLToPath(import.meta.url)), "../../📦️packages/🟦️typescript");
const repoRoot = resolve(dir, "../../../../..");

export default defineConfig({
  root: dir,
  plugins: [...semioAssetsVitePlugin(repoRoot), tailwindcss(), react()],
  resolve: { alias: [{ find: "@semio-tech/mit-bestand-praesentation-projektetage-spec", replacement: resolve(dir, "🔖️spec.ts") }] },
  test: { root: dir, name: "@semio-tech/mit-bestand-praesentation-projektetage", mode: "test", environment: "node", include: [], coverage: { include: ["📦️index.ts", "🔖️spec.ts"] }, includeSource: ["📦️index.ts"], passWithNoTests: false },
});
```
`🧪️tests/🧪️projektetage-deck/🟦️.ts` exports `registerProjektetageDeckTests(vitest, dependencies, source)` — same
dependency-injection test-registration pattern as the product's own `🧪️tests/🎞️presentation-react-deck/🟦️.tsx`
(asserts chapter order, German bookmark names incl. the "Zukunft Bau Auftakt" bookend slide injected at h=0,v=0).

### Launch configuration (`.vscode/🧩️launch.seed.jsonc`)

There is **exactly one** launch entry specific to this product family — the projektetage dev server. Quoted
verbatim (lines 958–976):
```json
{
  "name": "🛠️dev📽️projektetage",
  "type": "node-terminal",
  "request": "launch",
  "command": "bun nx run @semio-tech/mit-bestand-praesentation-projektetage:dev",
  "cwd": "${workspaceFolder}",
  "env": { "PRAESENTATION_PROJEKTETAGE_PORT": "6050" },
  "presentation": { "group": "3_dev", "order": 210 },
  "serverReadyAction": { "action": "openExternally", "pattern": "(http://(?:127\\.0\\.0\\.1|localhost):6050)", "uriFormat": "%s" }
}
```
(NB: the `"presentation"` key here is VS Code's own launch-config grouping field, unrelated to the semio
presentation *product* — do not confuse the two when grepping this file.) There is **no** dedicated launch entry
for `@semio-tech/presentation` or `@semio-tech/presentation-react` tests — those run through the single generic
entry near the top of the file:
```json
{ "name": "🧪️test", "type": "node-terminal", "request": "launch", "command": "bun nx run workspace:test", "cwd": "${workspaceFolder}", "presentation": { "group": "3_dev", "order": -49.5 } }
```
i.e. `bun nx run workspace:test` (and sibling `workspace:build`/`workspace:lint`/etc.) fan out to every nx project
including the presentation product; only long-running dev servers and a handful of one-off scripts get their own
named entry. **Conclusion for the quiz product**: only the quiz *renderer/host app* (the equivalent of
projektetage, if one exists) needs its own `dev` launch entry with a fixed port; the quiz product package itself
does not need one.

---

## 4. Registration points of a product in the repo

Search performed: `git grep -l "🎤️presentation\|framework.product.presentation\|@semio-tech/presentation\b\|@semio-tech/presentation-react\|mit-bestand-praesentation-projektetage"` over tracked files, excluding `.🧬semio` tickets and `.cursor/plans` (30 historical planning docs also matched — not registration, so omitted from the table below).

| File | What it does |
|---|---|
| `🧰️framework/🛍️products/🔣️.json` | The `x-semio` products collection. **Must add a new member** for the quiz product: `{"directory": "<emoji>quiz", "id": "framework.product.quiz", "kind": "product", "responsibility": "..."}`, alongside the existing `framework.product.os` / `.print` / `.presentation` / `.repo` / `.server` entries. |
| `package.json` (root) `workspaces` | Lists `🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript` and its `🎯️targets/⚛️react` sibling explicitly (2 lines, alphabetically placed among ~140 workspace entries). **Must add** the quiz package(s) here — bun workspaces are NOT auto-discovered; every package.json that should be linked needs an explicit entry. |
| `nx.json` | **No per-product entry needed.** Confirmed by reading it: project discovery is handled by the `@nx/js` plugin (`analyzePackageJson/analyzeSourceFiles/analyzeLockfile: false` — it relies on `project.json` files being present, which nx auto-discovers workspace-wide). No `🎤️presentation` mention anywhere in `nx.json`. |
| `tsconfig.json` (root) | **No `paths` mapping exists at all** (`compilerOptions` has no `"paths"` key) and no `🎤️presentation` mention. Cross-package TS resolution is done purely via bun workspace symlinks + each consumer's own bundler-level `resolve.alias` (Vite/Vitest configs, as shown in §1/§3) — not centralized. Nothing to register here for the quiz product either. |
| `🧅️layering.json` | **Does not exist** as a tracked file anywhere in the repo (only ticket-folder mentions of the word "layering" in past plan documents matched). Not a real registration point. |
| `🔒️dependencies.json` | **Generated, not hand-edited** — has `"generatedAt"`/`"commit"` fields and is clearly produced by a script that scans every `package.json`. It lists the presentation-react `package.json` as a `"users"` entry under every npm package it declares (e.g. `@types/react`, `@types/react-dom`, `pdfjs-dist`, `reveal.js`, …). Adding the quiz product's own `package.json` with its deps will make it show up here automatically the next time the generator runs — nothing to hand-edit. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | Contains a key literally named `"presentation"` (line 1974) but it is an unrelated generic file-kind entry for `.pptx` files (`emoji: "📽️"`, `role: "documentation"`), not a product registration. Other `presentation`-substring hits are all incidental (`🎨️representation`, `presentation-egice23/25` picture decks, UI "hotkey/tooltip presentation" naming, etc.). **No taxonomy edit needed** for a new product as such. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.d.ts` | A long docstring at the top **still claims** (stale — see caveat below) that the root `package.json` workspaces array does *not* list the two presentation packages. It in fact does (verified directly, §above). This looks like a leftover comment from before the workspace registration landed; flag/fix if touched, but it is not itself a registration mechanism — it is Bun global-type declarations, unrelated to product wiring. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json` | A **policy fixture** (46 entries, `schemaVersion: 1`) recording, per vitest-config root, its `previousPath` (legacy `vitest.config.ts` location), the canonical `ownerPath` (the `🧪️tests/🎚️config/🟦️.ts` file that now owns it) and `expectedName` (the package name). The presentation product's two entries, verbatim: `{"configurationRoot": "🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript", "previousPath": ".../vitest.config.ts", "ownerPath": "🧰️framework/🛍️products/🎤️presentation/🧪️tests/🎚️config/🟦️.ts", "expectedName": "@semio-tech/presentation", "projectionSha256": "98bf7de6..."}` and the matching react-target entry with `expectedName: "@semio-tech/presentation-react"`. This is fixture data for a completed migration ticket (has `projectionSha256` hashes) — it is not clear it must grow for every new product; verify whether a live repo policy scans for this pattern generically or whether this fixture is frozen history before assuming quiz needs an entry here. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎯️goals/🧫️fixtures/🎯️goal-documents.json` | One fixture goal JSON blob mentions `"replacing framework/product/presentation"` in the description of an unrelated `animate` goal draft — not a registration, just incidental prose. |
| `README.md` (repo root) | Two `<img>` badge links reference `🎙️presentation-egice25`/`🎤️presentation-egice23` **asset** paths (research-paper decks, a different, older, unrelated pair of presentation instances under `🧰️framework/🔨️modules/🖼️assets/📛️badge/...`) — not the product being explored. No product-table row exists in the README for this product; nothing to add there for quiz unless the maintainers want a similar badge. |
| `.vscode/🧩️launch.seed.jsonc`, `.vscode/launch.json` | See §3 — only the projektetage **app** gets a named dev entry; the product itself relies on the generic `workspace:test`/`workspace:build` entries. |
| `bun.lock` | Lockfile; will pick up quiz's deps automatically once `package.json`/workspaces are wired — not hand-edited. |

### Summary of the **minimum actual registration set** for a new framework product (as demonstrated by presentation)
1. `🧰️framework/🛍️products/🔣️.json` — add one `x-semio` member.
2. Root `package.json` `workspaces` — add the package path(s) (core + any target subfolders that are themselves bun workspaces, e.g. `.../🎯️targets/⚛️react`).
3. The product's own `📋️project.json` files (core + each target) — nx auto-discovers these; no central nx.json edit.
4. Optionally, if a consumer app needs its own dev server, a single `.vscode/🧩️launch.seed.jsonc` entry with a fixed `PORT`/`env` — modeled on the `🛠️dev📽️projektetage` entry.
Everything else (`🔒️dependencies.json`, `bun.lock`, taxonomy `presentation` key) is either generated or unrelated.

---

## 5. Goal file

`.🧬semio/🦑️repo/🎯️goals/RUNNING-FRAMEWORK/RUNNING-PRODUCTS/RUNNING-PRESENTATION/🎯️goal.json` (verbatim, full file):
```json
{
  "title": "Running Presentation",
  "description": "The presentation product runs on its own.",
  "prompt": "The presentation product runs on its own.",
  "status": "open",
  "dates": { "due": "2026-12-31" },
  "client": "cursor-chat",
  "llm": "grok-4",
  "parent": "🎯runningframework🎯runningproducts",
  "github": { "issue": "https://github.com/usalu/semio/issues/2622" }
}
```
Sibling goals under the same `RUNNING-PRODUCTS` parent (for context/model when creating a quiz goal):
`RUNNING-OS`, `RUNNING-PRINT`, `RUNNING-PRESENTATION`, `RUNNING-REPO`, `RUNNING-SERVER` — one goal folder per
framework product, each presumably following this same minimal shape (`title`/`description`/`prompt`/`status`/
`dates.due`/`client`/`llm`/`parent`/`github.issue`). No goal folder exists yet for `projektetage` or any other
consumer app — goals in this tree are per **product**, not per consumer.

---

## Open questions / things to verify before blind-copying

1. `📦️packages/🟦️typescript/🌐️.html` (script src `./js/index.ts`) does not match the current entry file name
   (`🟦️.ts` at the bundle root, referenced from `🌐️index.html`) — looks stale; confirm which HTML file is
   actually the live Vite entry before using either as a template.
2. Projektetage's `📋️project.json` has no explicit `test` nx target block even though `📜️script.ts test` exists
   and a test suite exists — confirm whether that's intentional before assuming every consumer app needs one.
3. Two oracle registries coexist for this product (`🔣️oracle.json` schemaVersion 1 vs `🔮️oracles/🔣️.json`
   schemaVersion 2) — for a new product, only the `🔮️oracles/🔣️.json` shape (richer, matches the repo test
   platform's own schema) should likely be copied; the simpler `🔣️oracle.json` may be legacy.
4. The `🎚️vitest-configuration-ownership` fixture's exact update trigger (is it regenerated by a live scan, or
   was it hand-authored once as migration proof?) was not confirmed — check the policy/test that consumes it
   before assuming the quiz product must add entries there.
