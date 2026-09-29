# Explore: Reusable UI Infrastructure & Web-App Patterns for the Quiz Product

Scope: locate reusable static-site build tooling, UI components, i18n, charts, local-first/client-state
utilities, and testing conventions for a future `quizze.architektur-und-technologie.de` quiz app.
Read-only exploration; no repo files changed by this pass.

## 1. Static website/app builds

The repo's standard for a deployable static web app is **Vite 7 + `@vitejs/plugin-react` + Tailwind v4
(`@tailwindcss/vite`)**, driven by a per-bundle `📜️script.ts` task router (per `AGENTS.md`: `project.json`/
`package.json` only ever call `bun ./📜️script.ts <command>`), with Nx orchestrating `dev`/`build`/`test`.
`bun` is the package manager; `nx` the task runner, exactly as mandated.

### `🏢️semio-tech/🎡️play` (CDN-deployable static grid of every semio app)

- `C:\git\semio\🏢️semio-tech\🎡️play\package.json` — `"scripts"` delegate to `bun nx run ...`; devDeps
  `"@tailwindcss/vite": "^4.1.18"`, `"@vitejs/plugin-react": "^5.1.2"`, `"vite": "^7.3.1"`.
- `C:\git\semio\🏢️semio-tech\🎡️play\📋️project.json` — Nx targets `prepare-dev`/`prepare-release` →
  `activate-dev` → `dev`/`serve` (continuous, port `SEMIO_TECH_PLAY_PORT=6033`) → `build` (`bun
  ./🔨️modules/📦️site/📜️script.ts build`, outputs `dist/pages`) → `test`/`test-e2e` (Playwright-style,
  `serve-e2e` continuous + `test-e2e`).
- `C:\git\semio\🏢️semio-tech\🎡️play\📜️script.ts` — root script only registers `test`, delegating to
  `runVitest(this.root, rest, "./🧪️tests/🎚️config/🟦️.ts")` from the shared `BundleScript`/`ScriptRouter`
  base classes in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`.
- `C:\git\semio\🏢️semio-tech\🎡️play\🔨️modules\📦️site\📜️script.ts` — the actual CDN publish step:
  `buildViteArtifact({ root, workspace: this.repoRoot, config: ".../🏗️builder/🌐️vite/🟦️.ts", output:
  "dist/site", owner: "play:site", signal, environment: { SEMIO_BUILD_MODE: "ship", SEMIO_RENDERER:
  "react", GIS_MAP_TILE_SERVE_MODE: "bundle" } })` then `publishPlayPages(dist/site, dist/pages,
  PLAY_HOST)`. Cancellation is wired via `AbortController` + `SIGINT`/`SIGTERM` — matches the
  "progress+cancellation for expensive ops" rule.
- `C:\git\semio\🧰️framework\🛍️products\🦑️repo\🔨️modules\📚️library\⚡️caching\🌐️vite\🟦️.ts` —
  `buildViteArtifact(options: ViteArtifactBuild)` shells out to the workspace's own resolved `vite/bin/
  vite.js` (`vite build --config <cfg> --configLoader bundle --outDir <tmp> --emptyOutDir`), stages
  artifacts only after success (`stageArtifacts`), and `serveVite(options: ViteService)` owns one Vite
  dev listener with `AbortSignal`-based teardown and an optional readiness/session endpoint
  (`SERVICE_READY_ENDPOINT`). This is the canonical **owned wrapper around the external `vite` package**
  — per `AGENTS.md`'s "external libraries behind an interface" rule, nothing else in the repo should
  import `vite`'s `createServer`/`build` directly outside this module.
- `C:\git\semio\🏢️semio-tech\🎡️play\🏗️builder\🌐️vite\🟦️.ts` — the actual `defineConfig(({ command }) =>
  ...)`. Key shape: `root`, `base: "./"`, `cacheDir: repoCacheDirectory(...)`, `publicDir`,
  `resolve.alias` mapping `@semio-tech/ui-react`, `@semio-tech/ui-react/runtime`, `@semio-tech/ui-styling`,
  `@semio-tech/assets`, `@semio-tech/framework(-os)` to their workspace source files (not `node_modules`),
  `server.fs.allow`, and a `plugins` array: `semioServeCloseVitePlugin()`, `semioHostHtmlVitePlugin(...)`
  (generates the HTML shell), `semioEmojiIndexHtmlVitePlugin`, `semioBackboneVitePlugin()`,
  `semioBlobVitePlugin()`, `semioAssetsVitePlugin(...)`, `react()`, `tailwindcss()`. `build:
  semioViteProductionBuild()` is the shared prod-build tuning (also owned, in `🧰️framework/🔨️modules/
  🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`).
- `C:\git\semio\🏢️semio-tech\🎡️play\🌐️.html` — minimal shell:
  ```html
  <div class="h-full w-full" id="root"></div>
  <script type="module" src="./🟦️.tsx"></script>
  ```
  Height uses a CSS var: `height: var(--ui-available-height, 100dvh);` — for embedding inside chrome later.
- `C:\git\semio\🏢️semio-tech\🎡️play\🎨️globals.css` — one line: `@import
  "../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎨️.css";` plus small
  `[data-slot="..."]` overrides — confirms Tailwind v4's CSS-first config (no `tailwind.config.js`).

### `♻️mit-bestand/🎤️präsentation/📅️33.projektetage` (a real, standalone, CDN-deployed single-topic site)

This is the closest existing analog to "one small quiz site deployed to its own domain": it ships a
`CNAME` file in `public/`, has its own `dev`/`build`/`test` Nx targets, and depends only on two workspace
packages.

- `C:\git\semio\♻️mit-bestand\🎤️präsentation\📅️33.projektetage\📦️packages\🟦️typescript\package.json` —
  `"dependencies": { "@semio-tech/presentation": "workspace:*", "@semio-tech/presentation-react":
  "workspace:*" }`; devDeps `@tailwindcss/vite ^4.1.18`, `@vitejs/plugin-react ^5.1.2`, `vite ^7.3.1`,
  `vitest ^4.0.17`. `"semio": { "app": { "kind": "projektetage", "port": { "dev": 6050, "env":
  "PRAESENTATION_PROJEKTETAGE_PORT" } } }` — every app registers its own fixed dev port via this
  `semio.app` convention (used by `.vscode/launch.json`, per `AGENTS.md`'s "devs use launch.json" rule).
- `C:\git\semio\♻️mit-bestand\🎤️präsentation\📅️33.projektetage\📦️packages\🟦️typescript\📜️script.ts` —
  `DevScript` → `runViteBunxDev(root, segments, { config: "../../🏗️builder/🌐️vite/🟦️.ts", portEnv:
  playgroundPortEnv("projektetage"), defaultPort: playgroundDevPortString("projektetage"), fixedPort:
  true })`; `BuildScript` → `runViteBuild(root, segments, "../../🏗️builder/🌐️vite/🟦️.ts")` (and explicitly
  **rejects** `--outDir/--config/--root` args — "Build output and configuration are owned by this Nx
  target"); `TestScript` → `runVitest(root, rest, "../../🧪️tests/🎚️config/🟦️.ts")`.
- `C:\git\semio\♻️mit-bestand\🎤️präsentation\📅️33.projektetage\📦️packages\🟦️typescript\🔖️spec.ts` — the
  **declarative content model** for the deck: `PresentationMeta`, `IntroSpec` (title/description/goal/
  authors/affiliations), `Participant`/`Embodiment`/`Disposition`/`MorphToSlot` types imported from
  `@semio-tech/presentation`, plus pure grid/crop/layout math (`split`, `splitFigureGrid`,
  `unionSourceCrops`, `remapSplitDispositions`). This is schema-first content driving React rendering —
  the same shape a declarative quiz model (questions/options/answers) should take. **Caveat**: this deck
  hardcodes `language: "de"` — it is single-language, not an example of the bilingual "no default
  language" pattern the quiz needs (see §3 for the actual owned i18n mechanism).
- Vite builder: `C:\git\semio\♻️mit-bestand\🎤️präsentation\📅️33.projektetage\🏗️builder\🌐️vite\🟦️.ts`
  (same shape as Play's, product-scoped).

## 2. UI modules — `🧰️framework/🔨️modules/🖱️ui`

Root: `C:\git\semio\🧰️framework\🔨️modules\🖱️ui`. This is one very large, taxonomy-organized design
system (400+ subfolders) with these top-level areas relevant to the quiz:

- `🧱️elements/` — ~90 leaf UI components, each `<Emoji>ComponentName/🟦️.tsx` (+ `📖️stories/🧪️.story.tsx`,
  often `🧪️tests/🧩️component/🟦️.tsx`, sometimes a `🎯️targets/⌨️tui` Rust twin for the TUI target).
- `🔨️modules/` — small composable presentation/behavior hooks shared across elements (class-name
  composition, border/surface/interaction presentation, keybinding context, flow-direction context,
  fuzzy-ranking, etc).
- `🧬️contract/`, `🧬️schema/` — typed contracts (accessibility, layout, retained-mode rendering) shared
  between the React and wgpu/TUI targets — this UI library is explicitly **multi-target** (React, `⌨️tui`,
  `🧊️wgpu`), matching the "domain-driven taxonomy... multiple languages" and "multi-device" rules.
- `🎨️styling/` — theming (see §2c).
- `🎯️targets/⚛️react` — the React package/barrel (see below).

### 2a. Components directly reusable for the quiz

| Quiz need | Element(s) | Path |
|---|---|---|
| Drag items into categories (classification) | `Table` with `dragDrop` (dnd-kit reordering) or plain `DragHandle` + native HTML5 DnD | `🧱️elements/📊️Table/🟦️.tsx`, `🧱️elements/🧱️DragHandle/🟦️.tsx` |
| Numerical sorting / ordering | `Table.dragDrop` (row reorder via `@dnd-kit/core`), or `Tree` (also drag-sortable) | `🧱️elements/📊️Table/🟦️.tsx`, `🧱️elements/🌳️Tree/🟦️.tsx` |
| Numerical matching (assign items to values) | `Select`, `Slider`, `KeyValue`, `Table` columns | `🧱️elements/🔽️Select`, `🧱️elements/🎚️Slider`, `🧱️elements/🔑️KeyValue` |
| Onboarding steps / multi-step flow | `Stepper` (progress-style), `Tabs`, `Dialog`/`UIDialog`, `Wizard` (⚠️ TUI-only today, no React target yet) | `🧱️elements/🪜️Stepper`, `🧱️elements/📑️Tabs`, `🧱️elements/💬️Dialog`, `🧱️elements/🧙️Wizard` |
| Name entry (anonymous/pseudonym/real name) | `Input`, `Field`, `Form`, `RadioGroup`-like via `ToggleGroup` | `🧱️elements/✏️Input`, `🧱️elements/📝️Field`, `🧱️elements/🧾️Form`, `🧱️elements/🎛️ToggleGroup` |
| Results screen table / leaderboard | `Table` (sortable, `stickyHeader`, `TableSkeleton` loading state) | `🧱️elements/📊️Table/🟦️.tsx` |
| Badges | `Chip` | `🧱️elements/🪙️Chip` |
| Toasts / feedback | none found under `🧱️elements` by that name — closest is `ChromeControlHint`/`Dialog` | `🧱️elements/💡️ChromeControlHint` |
| Loading states | `Skeletons` (`TableSkeleton` is also exported directly from `Table`) | `🧱️elements/🦴️Skeletons` |
| Context menu / right-click | `ContextMenu` | `🧱️elements/🖱️ContextMenu` |
| Node/graph diagram (not needed for spider chart, but owned SVG+ARIA canvas exists) | `Diagram` | `🧱️elements/🕸️Diagram` |

**No `Chart`/`Radar`/`Spider` element exists under `🧱️elements` or anywhere in `🧰️framework/🔨️modules/
🖱️ui`.** See §4 — the only owned "radar" visualization code is print/LaTeX, not a web component.

### 2b. `Table` component — drag-and-drop contract (full signature)

`C:\git\semio\🧰️framework\🔨️modules\🖱️ui\🧱️elements\📊️Table\🟦️.tsx` — imports `@dnd-kit/core`
(`closestCenter, DndContext, PointerSensor, useDraggable, useDroppable, useSensor, useSensors`) directly
(an allowed "existing library to test our implementation" per `AGENTS.md`, since it's used to *validate*
the sortable behavior rather than being re-exported raw). Key exported types:

```ts
export interface TableColumn<T = unknown> {
  id: string; header: React.ReactNode; accessor: (row: T) => React.ReactNode;
  width?: string; className?: string; headerClassName?: string;
  sortable?: boolean; visible?: boolean | ((data: T[]) => boolean);
}
export interface DragDropConfig {
  enabled?: boolean;
  pointerActivationDelayMs?: number;      // delay (ms) before drag activates, so double-click still lands
  pointerActivationTolerancePx?: number;
  pointerActivationDistancePx?: number;
  onDragStart?: (rowId: string) => void;
  onDragEnd?: (event: { active: string; over: string | null }) => void;
  canDrag?: (rowId: string) => boolean;
  canDrop?: (draggedId: string, targetId: string) => boolean;
  renderDragOverlay?: (rowId: string) => React.ReactNode;
}
export interface TableProps<T = unknown> {
  columns: TableColumn<T>[]; data: T[];
  onRowClick?, onRowContextMenu?, onRowDoubleClick?, onRowMouseEnter?, onRowMouseLeave?;
  sortColumn?: string; sortDirection?: "asc" | "desc"; onSort?: (columnId, direction) => void;
  selectedRows?: Set<string> | string[]; getRowId?: (row: T) => string;
  stickyHeader?: boolean; rowHeight?: "compact" | "normal" | "comfortable";
  hierarchical?: boolean; onToggleRow?: (rowId: string) => void;
  dragDrop?: DragDropConfig;
  rowDragProps?: (row: T, index: number) => React.HTMLAttributes<HTMLTableRowElement>; // native HTML5 DnD, independent of dragDrop
  renderMobileRow?: (...) => React.ReactNode; isMobile?: boolean;   // mobile row layout switch
}
export function tableSortNextDirectionV1(columnId, sortColumn, sortDirection): "asc" | "desc";
export function tableSortAriaV1(columnId, sortColumn, sortDirection): "ascending" | "descending" | undefined;
export const Table: <T>(props: TableProps<T>) => JSX.Element;
export const TableSkeleton: React.FC<{ columns; rowCount?; className? }>;
```

Only `PointerSensor` is wired in `useSensors(...)` (`🟦️.tsx:329-341`) — **no `KeyboardSensor`** is
registered, so today's `Table.dragDrop` reordering is pointer-only; a fully keyboard-accessible
classification/sorting question (per the "accessible UIs" rule) would need to add dnd-kit's
`KeyboardSensor` or build an explicit non-drag reorder fallback (e.g. "move up/down" buttons), matching
the existing `tableSortNextDirectionV1`/button-driven sort-header pattern which *is* keyboard-native.

`DragHandle` (`C:\git\semio\🧰️framework\🔨️modules\🖱️ui\🧱️elements\🧱️DragHandle\🟦️.tsx`) is the shared
grip affordance: accepts either dnd-kit's `attributes`/`listeners` spread, or raw
`onPointerDown/Move/Up/Cancel` for a hand-rolled pointer-capture drag, and renders inside
`ChromeControlHint` for an accessible tooltip/label (`labelId` resolves through the i18n port, §3).

### 2c. Theming / customization tokens

- `C:\git\semio\🧰️framework\🔨️modules\🖱️ui\🎨️styling\🎨️palette\🎨️.css` — **generated** file (header:
  `/* Generated from framework/ui/styling/🔣️.json — run 'bun ./📜️script.ts generate'. */`). Declares
  `@font-face` blocks (Anta, Anta Extended/Math/Symbols, Kelly Slab variants, Share Tech Mono, Noto Emoji,
  …) then CSS custom properties, e.g.:
  ```css
  --color-primary: #ff344f;
  --color-secondary: #34d1bf;
  --color-tertiary: #fa9500;
  --color-danger: #a60009;
  --color-warning: #fccf05;
  --color-info: #dbbea1;
  --color-success: #7eb77f;
  --color-gray-100 … --color-gray-900   /* full dark→light ramp */
  ```
- `C:\git\semio\🧰️framework\🔨️modules\🖱️ui\🎨️styling\🌓️theme\🎨️.css` just `@import
  "../🎨️palette/🎨️.css";` — theme = palette + (light/dark) selection.
- `C:\git\semio\🧰️framework\🔨️modules\🖱️ui\🎨️styling\🌓️theme\🏛️model\🟦️.ts` and `🔤️tokens/🐍️.py` +
  `🔤️tokens/🦀️.rs` — the token *source of truth* is generated/consumed cross-language (Python + Rust +
  generated CSS/JSON), consistent with the "domain-driven taxonomy... multiple languages" rule; components
  consume tokens purely via Tailwind v4 arbitrary-value classes, e.g. `text-[color:var(--active-base)]`
  (seen in the drag-and-drop story) or plain Tailwind utility classes (`bg-muted`, `text-muted-foreground`,
  `rounded-sm`) — Tailwind v4 is CSS-first (`@tailwindcss/vite`, no `tailwind.config.js`), so no bespoke
  theme JS config to wire up for a new app beyond importing the generated palette CSS and adding
  `tailwindcss()` to the Vite plugin list.
- `🎨️styling/🏗️builder/🌐️vite/🟦️.ts` exports the shared Vite plugin set every app's builder imports:
  `semioHostHtmlVitePlugin`, `semioEmojiIndexHtmlVitePlugin`, `semioAssetsVitePlugin`,
  `semioViteProductionBuild`, `createWorkspaceViteResolveConfig`, `uiTailwindBuildPlugins`, etc. — this is
  the one place to add a new app's HTML/theme wiring rather than hand-rolling Vite config.

### 2d. Accessibility

- Keyboard activation is unit-tested directly, e.g. `Toggle`
  (`🧱️elements/🔀️Toggle/🧪️tests/🧩️component/🟦️.tsx`) asserts Enter fires once and Space fires once via
  `keydown`+`keyup`, and that a disabled toggle suppresses both click and key activation.
  `tableSortAriaV1` (above) drives `aria-sort` on sortable table headers.
  `Diagram`'s i18n keys (`ui.diagram.*` in §3) show the pattern for an `application`-role interactive
  canvas that must self-announce keyboard help via a live region (`focusedNode`/`selectedNode`/
  `selectionCleared` strings) — the same pattern a keyboard-accessible spider-chart/classification widget
  should follow.
- `ChromeControlHint` (`🧱️elements/💡️ChromeControlHint`) is the shared accessible-tooltip/hint wrapper
  used by `DragHandle` and others.

### 2e. React is the standard target

React **19.2.3** is pinned consistently:
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/package.json:40`,
`🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json:28`,
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/package.json:20`,
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/
package.json:31`, root `package.json:276` (`"react": "^19.2.3"`). The UI library also ships an
`⌨️tui` (terminal UI, Rust) target and a `🧊️wgpu` target for some elements (e.g. `Table`'s
`🎯️targets/⌨️tui/🦀️.rs`) — no Solid/Svelte/web-components found; React is the sole browser UI target.
Components are imported via the workspace package alias `@semio-tech/ui-react` (resolved by every
builder's Vite `resolve.alias` to `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/
🟦️.tsx`), plus `@semio-tech/ui-react/runtime` and `@semio-tech/ui-react/test` sub-paths for the reconciler
runtime and test harness respectively.

## 3. i18n — canonical mechanism

**`i18next` + `react-i18next`, wrapped behind an owned `UiI18nPort` interface** (satisfies the "external
libraries behind an interface" rule — "do not import i18next outside this bundle").

- Port + schema: `C:\git\semio\🧰️framework\🔨️modules\🖱️ui\🧱️elements\📚️I18n\🟦️.tsx`
  ```ts
  export type UiLocale = ShellLocale;   // = "en" | "de" (ShellLocale defined in 🎚️axes/📽️projection/🟦️.ts)
  export type UiLabelValue = { readonly label: { normal: string; beginner: string }; manual?: string; tutorial?: string };
  export type UiTranslationKey = DeepUiTranslationKeys<UiTranslationSchema>;  // dot-path key union, e.g. "ui.common.loading"
  export type UiTranslateFn = <K extends UiTranslationKey>(key: K, options?: Record<string, unknown>) => unknown;
  export interface UiI18nPort {
    readonly t: UiTranslateFn;
    exists(key: string): boolean;
    changeLanguage(locale: UiLocale): Promise<unknown>;
    readonly language: string | undefined;
    readonly resolvedLanguage: string | undefined;
    readonly isInitialized: boolean;
  }
  ```
  `UiTranslationSchema` is one huge nested object of `ui.<area>.<key>: UiLabelValue` — a compile-time
  dictionary every UI string must live in (English + German both required — `registerUiTranslationBundles`
  requires **both locales or a compile error**, "both-locales-or-nothing guarantee").
- Wiring/runtime glue: `C:\git\semio\🧰️framework\🔨️modules\🖱️ui\🎯️targets\⚛️react\🟦️.tsx` (region `🔌️I18n
  Port`, ~line 4280 onward):
  ```ts
  export function registerUiTranslationBundles<S>(bundles: { readonly [L in UiLocale]: { translation: S } })
    : <K extends DeepUiTranslationKeys<S>>(key: K) => UiRegisteredTranslationKey
  export function createShellI18nInstance(initialLocale: UiLocale): typeof i18next   // per-embedded-shell instance
  export function disposeShellI18nInstance(instance: typeof i18next): void
  export const detectShellLocale: (language?: string) => ShellLocale               // normalizeUiLocale
  ```
  Locale normalization (**this is the "no default language" handling**):
  ```ts
  function normalizeUiLocale(language?: string): UiTranslationLocaleCode {
    return language?.toLowerCase().startsWith("de") ? "de" : "en";
  }
  function resolveRequestedUiLocale(): UiTranslationLocaleCode {
    const storedLocale = readStoredUiChromeLocale(createBrowserStoragePort());       // localStorage first
    if (storedLocale) return storedLocale;
    return normalizeUiLocale(i18next.resolvedLanguage || i18next.language ||
      (typeof navigator !== "undefined" ? navigator.language : undefined));          // else navigator.language
  }
  ```
  `i18next.init({ resources: uiChromeTranslationBundles, fallbackLng: "en", supportedLngs: ["en","de"],
  nonExplicitSupportedLngs: true, lng: requestedLocale, returnObjects: true, initImmediate: false,
  interpolation: { escapeValue: false }, react: { useSuspense: false, bindI18n: "languageChanged",
  bindI18nStore: "added removed" } })`. `initImmediate: false` forces synchronous init specifically so the
  very first paint is never in an untranslated/English-flash state.
  **Practical reading of "no default language"**: there is no app-baked default; the app detects the
  visitor's browser language (`navigator.language`), normalizes it to `"en"` or `"de"` (English wins any
  tie/unknown case — matches `AGENTS.md`'s "English first, then German second"), remembers the user's
  explicit override in `localStorage`, and `fallbackLng: "en"` is only the last-resort i18next fallback if
  a key is missing in the resolved language.
- Consumption hook: `useLabel(id: UiTranslationKey | UiRegisteredTranslationKey, options?):
  UiLabel` — `C:\git\semio\🧰️framework\🔨️modules\🖱️ui\🧱️elements\🏷️Label\🟦️.tsx:36-38` (overloaded for
  optional `id`). `DragHandle` above calls `useLabel(targetLabelId, { target: subject ?? "" })` for
  interpolated strings.
- Product-owned extension bundles register via `registerUiTranslationBundles({ en: { translation: {...} },
  de: { translation: {...} } })`, which returns a typed key-caster the product then uses instead of
  hand-rolling its own translation-key union — this is the pattern the quiz product should follow for its
  own `ui.quiz.*` strings (question prompts/options are *data*, not translation keys, and should carry
  their own `{ en, de }` pair directly in the quiz content model — see `UiLabelPair`/`UiLabelValue` shape
  above as the established `{ normal, beginner }`-tier pattern to mirror for `{ en, de }` question text).
- No language picker component named explicitly was found under `🧱️elements`; the settings panel driving
  `ui.settings.language.{en,de}` (see `UiTranslationSchema.settings.language` in `📚️I18n/🟦️.tsx`) is
  presumably composed from `Select`/`ToggleGroup` inline in the settings surface rather than a dedicated
  `LanguagePicker` element — worth building generically if the quiz needs a standalone one.

## 4. Charts (radar/spider) — no owned web/SVG implementation exists

Searched the whole repo for `radar`, `spider`, `Chart` in `.ts`/`.tsx`. Findings:

- **Print product only**: `C:\git\semio\🧰️framework\🛍️products\📓️print\🖋️latex\semio-viz-chart-polar-radial.sty`
  and `semio-viz-charts-polar.sty` are owned **LaTeX/TikZ** stylesheets (part of a huge `semio-viz-*`
  catalog of ~150 `.sty` chart families under `🧰️framework/🛍️products/📓️print/🖋️latex/`) — these render
  radar/polar charts **into PDF via Tectonic**, not into the browser. There is no JS/SVG/React radar
  component anywhere in the repo.
- Test-driven validation of the polar/radar math (per `AGENTS.md`'s "at least one third-party library to
  validate our own implementation" rule):
  `C:\git\semio\🧰️framework\🛍️products\📓️print\🧪️tests\🎡️charts-polar-radar\🟦️.ts` — uses `d3-scale`'s
  `scaleLinear` purely as the **oracle** to check the owned LaTeX/TikZ polar-projection math (`polar
  projection: zero at twelve o'clock, growing clockwise`, `px/py` helpers), compiled via `compileVizProbe`
  (`🧰️framework/🛍️products/📓️print/🔨️modules/🧪️viz-probe/🟦️.ts`) against fixtures at
  `🧰️framework/🛍️products/📓️print/🧫️fixtures/🎡️charts-polar-radar/{polar-scatter,polar-bars,radar}.tex`.
  Confirms `d3-scale` is an approved *test-oracle* dependency (imported directly in a test adapter, not
  re-exported), consistent with "use existing libraries as possible to test our implementation."
- **Implication for the quiz**: the spider/radar profile-classification chart has **no existing owned web
  component to reuse**. It must be built fresh as an SVG React element under `🧱️elements/` (e.g.
  `🕷️RadarChart` or similar), ideally reusing the same validated polar-projection math (`px`/`py`/
  `scaleLinear`-equivalent) that the print `.sty`/viz-probe tests already establish as correct, and adding
  a `d3-scale`-validated unit test mirroring `🎡️charts-polar-radar/🟦️.ts`'s oracle pattern. `Diagram`
  (`🧱️elements/🕸️Diagram/🟦️.tsx`) is the closest existing owned SVG+ARIA canvas element and is a
  reasonable structural template for accessibility (keyboard nodes/edges, live-region announcements) even
  though its domain (node-graph diagrams) is unrelated.

## 5. Client state & local-first

- **Domain-neutral async primitives** (exported from `@semio-tech/framework`, safe to reuse directly in a
  small quiz app talking to an HTTP/WebSocket server):
  - `retryWithJitteredBackoff<T>(fn: () => Promise<T>, options: { minMs, maxMs, signal? }): Promise<T>` —
    `C:\git\semio\🧰️framework\🔨️modules\⏳️async\🔁️jittered-backoff\🟦️.ts`. Full-jitter exponential
    backoff (`[minMs, min(maxMs, minMs·2^attempt)]`), always settles on `AbortSignal` abort rather than
    hanging — explicitly documented as "for short connection shortages only — the app must not freeze
    while this retries" (matches `AGENTS.md`'s "support short connection-shortages and not freeze the
    app").
  - `latestWins<T>(run: () => Promise<T>): () => Promise<T>` —
    `C:\git\semio\🧰️framework\🔨️modules\⏳️async\🥇️latest-wins\🟦️.ts`. Single-flight + trailing coalescer
    (N concurrent callers collapse into at most one extra follow-up call) — good fit for e.g. re-fetching
    the leaderboard on focus/interval without piling up requests.
  - `fetchWithTimeout(url, init, options): Promise<FetchTimeoutResponse>` —
    `C:\git\semio\🧰️framework\🔨️modules\🚪️io\🌐️fetch-timeout\🟦️.ts` — `AbortController`-based fetch
    timeout wrapper.
- **Full local-first/event-sourced document store** (the "real" CQRS/event-sourcing machinery) lives under
  `C:\git\semio\🧰️framework\🛍️products\💻️os\🔨️modules\🏪️store\` (`👷️worker/🟦️.ts` is 7,628 lines). It
  implements: a bounded local outbound-mutation queue (comment: "Bounded local outbound-mutation queue
  (finding 5)"), hub-session capability/auth, `retryWithJitteredBackoff`+`fetchWithTimeout`+`latestWins`
  composition for the hub connection lane, document check-in/lease/execution-target verification,
  `🔄️sync`, `👥️presence`, `🧾️document/📜️history`, `🧵️canonical-edit`. **This is the OS product's plugin/
  hub sync engine** — tightly coupled to the `os` shell, plugin actors, and hub authentication; it is
  architecturally the right *pattern* (event-driven, CQRS, bounded queue, jittered-backoff reconnect) but
  is too heavy/coupled to import wholesale into a standalone quiz app. Treat it as the reference
  implementation to imitate at a much smaller scope (a quiz-specific command queue + WebSocket reconnect
  loop built from the three domain-neutral primitives above), not as a dependency to pull in directly.
- **No generic owned IndexedDB or localStorage wrapper module** was found (no `🔨️modules/*storage*` or
  `*indexed-db*` taxonomy leaf under `🧰️framework/🔨️modules`). `localStorage` is used ad hoc in a handful
  of files (e.g. `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`'s
  `readStoredUiChromeLocale`/`readStoredUiChromeTerminology`/`createBrowserStoragePort` helpers, and
  `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`) via a small `createBrowserStoragePort()`
  seam rather than a published reusable "persisted local-only storage" package. A quiz app would need to
  either write its own thin IndexedDB/localStorage port (following the same "wrap external/browser API
  behind our own interface" pattern as `UiI18nPort`) or locate/ask about one in the `os`/`hub` products
  that this pass did not fully expand (the `🏪️store` tree above almost certainly has an IndexedDB-backed
  piece inside `👷️worker` given the "IndexedDB" grep hit, but line-level extraction was out of budget for
  this pass).

## 6. UI testing conventions

- **Component tests**: `vitest` + `jsdom` + `@testing-library/react`, explicit `include` file lists (not
  globs) inside a per-bundle `defineConfig` under `vitest/config`. Representative config:
  `C:\git\semio\🧰️framework\🔨️modules\🖱️ui\🎯️targets\⚛️react\🧪️tests\🎚️config\🟦️.ts` —
  ```ts
  export default defineConfig({
    root: testRoot,
    cacheDir: repoCacheDirectory(repoRoot, "vite", "ui-react"),
    resolve: { alias: [{ find: "@semio-tech/ui-react", replacement: resolve(root, "🟦️.tsx") }] },
    test: {
      root: testRoot, name: "@semio-tech/ui-react", environment: "jsdom",
      include: [ "../../../../🧱️elements/☑️Checkbox/🧪️tests/🧩️component/🟦️.tsx", /* …one line per test file… */ ],
      includeSource: ["../../🟦️.tsx"], coverage: { include: ["../../🟦️.tsx"] },
      passWithNoTests: false,
      setupFiles: [resolve(root, "../../../../🧪️tests/🧹️react-environment/🟦️.ts")],
    },
  });
  ```
- Representative component test (keyboard-accessibility asserted directly):
  `C:\git\semio\🧰️framework\🔨️modules\🖱️ui\🧱️elements\🔀️Toggle\🧪️tests\🧩️component\🟦️.tsx`
  ```tsx
  import { fireEvent, render } from "@testing-library/react";
  import { describe, expect, it, vi } from "vitest";
  import { Toggle } from "../../🟦️.tsx";
  it("activates once for Enter and once for Space", () => {
    const changes = vi.fn();
    const { getByRole } = render(<Toggle id="test.keyboard" icon="x" onPressedChange={changes} />);
    const button = getByRole("button");
    fireEvent.keyDown(button, { key: "Enter" });
    expect(button.getAttribute("aria-pressed")).toBe("true");
    fireEvent.keyDown(button, { key: " " });
    fireEvent.keyUp(button, { key: " " });
    expect(changes).toHaveBeenCalledTimes(2);
  });
  ```
- **Language-agnostic / fixture-driven unit tests** (the `AGENTS.md` "language-agnostic test for every
  feature" rule in practice): pure-logic table helpers are tested against a JSON fixture shared with (in
  principle) other-language twins, e.g.
  `C:\git\semio\🧰️framework\🔨️modules\🖱️ui\🧪️tests\📊️table-sort-header\🟦️.ts` reads
  `../../🧫️fixtures/📊️table-sort-header/🔣️.json` and asserts `tableSortNextDirectionV1`/`tableSortAriaV1`
  against every fixture `transitions[]` scenario — the doc-comment explicitly cross-references the wgpu
  target test that answers "to the same transitions."
- **Storybook**: one root Storybook at `C:\git\semio\.storybook\main.ts` (13KB config) aggregates every
  package-local `📖️stories/*.story.tsx` tree via `@storybook/react-vite`, with a `STORYBOOK_SCOPE`
  env var (comma-separated scope ids) driving `resolveActiveScopes`/`buildScopeStoryGlobs`/
  `buildScopeAliases` from `.storybook/📖️stories/🧭️coordination/🟦️.ts` — new products register a scope
  rather than each owning a separate Storybook instance. Representative story:
  `C:\git\semio\🧰️framework\🔨️modules\🖱️ui\📖️stories\🎭️drag-and-drop\🧪️.story.tsx` (`title: "🖱️ui⚛️react/
  DragAndDrop"`, `component: DragHandle`, plus a hand-rolled `ReorderableList` demo using native HTML5
  `draggable`/`onDragStart`/`onDrop` — explicitly noting "no dnd-kit wiring required for the story, just
  local list state").
- `🖱️ui/🧪️tests/📚️storybook-new-stories`, `📚️storybook-types`, `📚️storybook-uncovered-components` (seen
  in the directory listing, not opened) suggest there are meta-tests enforcing every new element ships a
  story and every story type-checks — worth checking before adding quiz-specific components so they
  satisfy the same coverage gates.

## Key open gaps for the quiz build

1. No owned web radar/spider chart — must be built new (§4).
2. `Table.dragDrop` is pointer-only (no `KeyboardSensor`) — classification/sorting questions need a
   keyboard-accessible alternative or an added sensor (§2b).
3. `Wizard` element has no React target yet (TUI/Rust only) — onboarding step flow should be composed from
   `Stepper`/`Tabs`/`Dialog` instead (§2a).
4. No generic reusable IndexedDB/localStorage port module found — only ad hoc `localStorage` reads in a
   few files; a quiz-scoped persisted-local-only store would need its own small wrapper (§5).
5. The one full local-first/event-sourced reference implementation (`🏪️store`) is OS/hub-specific and too
   heavy to import directly; reuse only the three domain-neutral async primitives
   (`retryWithJitteredBackoff`, `latestWins`, `fetchWithTimeout`) and mirror its bounded-queue/backoff
   pattern at quiz scale.
