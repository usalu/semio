# Explore: framework UI module (`🖱️ui`) — map for the pets work

Read-only exploration, 2026-10-02. Nothing was built, run or edited; every claim below is from reading the files cited
(`path:line`). Nothing here was executed, so "tests pass" is **not** claimed anywhere.

Path abbreviations used throughout:

| Short | Full path (repo-relative) |
|---|---|
| `UI` | `🧰️framework/🔨️modules/🖱️ui` |
| `REACT` | `UI/🎯️targets/⚛️react` |
| `PKG` | `REACT/📦️packages/🟦️typescript` (glue only) |
| `STYLING` | `UI/🎨️styling` |
| `QUIZ-R` | `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react` |

The module's own charter: `UI/AGENTS.md:4` — "General, pure, clean, consistent business logic free ui components."
`UI/README.md:8` — the bundle holds the React design system, the `styling` token pipeline (generated JS/Rust/Python/.NET/Tailwind
output), the wgpu widget primitives, and the vendored assets.

---

## 1. Directory tree (3 levels) and the element list

### 1.1 Tree

```
UI/
├─ AGENTS.md, README.md, components.json     charter / readme / shadcn-style registry file
├─ 🧱️elements/            65 element folders, one per UI primitive (see 1.2). Per element: 🟦️.tsx (React impl),
│                         📖️stories/🧪️.story.tsx, 🧪️tests/<kind>/🟦️.tsx, optional 🧬️schema/, 🧫️fixtures/, and
│                         🎯️targets/{🧊️wgpu,⌨️tui}/🦀️.rs for the non-React twins
├─ 🔨️modules/             21 pure (no JSX/no DOM-state) helper modules: class-name-composition (cn), style-variants,
│                         dom-event-binding, layered-overview-geometry, *-presentation class tables, keybinding*,
│                         fuzzy-ranking, element-identity, flow-direction-context, presence-presentation
├─ 🎯️targets/
│   ├─ ⚛️react/            THE React target: 🟦️.tsx barrel (8 734 lines), 🌓️appearance, 🌐️i18n, 🎠️runtime, 🪟️chrome,
│   │                     🖌️render (test adapter), 🛠️build-tooling, 🧪️tests/🎚️config, 🧹️lint, 📦️packages/🟦️typescript
│   ├─ 🧊️wgpu/             Rust GPU target (arena, flex layout, paint, widgets, shell, …)
│   └─ ⌨️tui/              Rust terminal target
├─ 🎨️styling/             token pipeline: 🔣️.json (source), 🖌️ui/🎨️.css (7 551-line Tailwind v4 stylesheet),
│                         🌓️theme (UiTheme model + apply), 🎨️palette, 🤖️generated (CSS/TS), 🔤️tokens (py/rs twins),
│                         📦️packages/{🟦️typescript,🦀️rust,🐍️python,🔷️dotnet}, 🧪️tests, 🧫️fixtures
├─ 🌐️globals/🎨️.css       global rules (docs-content, fullscreen navbar, ghost mode, chrome reveal, scrollbar hiding)
├─ 🧵️styles/🎨️.css        the consumer-facing stylesheet entry (`@import` ui + globals + palette; 5 lines)
├─ 🌐️i18n/🧬️schema        JSON schema for the translation-totality cases (3 keys × en/de × normal/beginner)
├─ 🎚️axes/                 TS axes (locale en/de, terminology native/reuse, bootstrap/plan/execution/…)
├─ 📱️device/              pure viewport-breakpoint policy shared by react and wgpu (`🟦️.ts`, 💬 byte-parity with Rust dock)
├─ 🧬️contract, 🧬️schema    language-agnostic contracts + JSON schemas (40 schema folders, e.g. 🥞️layered-overview)
├─ 🧫️fixtures, 🧪️fixtures  shared input/expected vectors (JSON; 61 fixture folders)
├─ 🔮️oracles/🔣️.json      registry of third-party "oracle" libraries tests compare against (clsx, cva, polygon-clipping, d3-ease …)
├─ 🧪️tests/               cross-element tests (e.g. 📦️react-package-export, 🧹️react-environment, 🧪️owned-locale-detector-retirement)
├─ 📖️stories/             cross-element Storybook stories (🎭️app, 🎭️navbar-example-select, 🎭️uiintroduction, …)
└─ 🎬️scene, 🖌️render, 🖥️host, 🧠️runtime, 🪟️viewport, 📦️packages   scene/render/host/runtime plumbing, Rust-heavy (folder names only; not read)
```

`UI/📦️packages/🟦️typescript` contains only an empty `🎯️targets/⚛️react/node_modules`; the real TS package is
`PKG` (rule from memory: targets own packages, package roots are glue).

### 1.2 All 65 `🧱️elements/*` (one line each; ✱ = no React file, other target only)

- ↔️Resizable — splitter / corner-join hit targets (`↔️Resizable/🟦️.tsx:66-95` window pointermove drag).
- ↕️Collapsible — owned collapsible state + host contract.
- ⌨️Command — command palette list with an owned match score.
- ☑️Checkbox — controlled native checkbox incl. mixed state.
- ⚙️VirtualFileSystem — render-agnostic VFS column/row descriptors + view.
- ⚡️ActionGroup — grouped action buttons.
- ✏️Input — text/number input (float-artefact stripping, collapsed value, ResizeObserver).
- ➖️Divider ✱ — TUI only.
- ⭕️Ring — SVG circular position markers with rAF-throttled pointer drag (`⭕️Ring/🟦️.tsx:130-175`).
- 🃏️OverviewCard — the landing-page card (a `WindowChrome` at dialog level); see section 4.
- 🆔️ElementId — element-id grammar + DOM id/alias stamping (MutationObserver `🆔️ElementId/🟦️.tsx:107`).
- 🌈️Surface — `Level` (base…menu), `SurfaceScope`, glass/veil/surface fills (`🌈️Surface/🟦️.tsx:15-90`).
- 🌳️Tree — the large virtualised tree (4 765 lines).
- 🎀️Ribbon — ribbon row layout.
- 🎗️UiLabel — branded label type (`uiDataLabel`).
- 🎚️Slider — slider with pointer capture (`🎚️Slider/🟦️.tsx:349`).
- 🎛️ToggleGroup — single/multiple selection group.
- 🎨️Canvas — interactive 2D canvas (2 135 lines, window pointermove `:1758`).
- 🎬️Scene — R3F/three 3D scene, gizmo, rAF camera snap (`🎬️Scene/🟦️.tsx:1895-1927`).
- 🎴️IconSelector — icon-kind editor.
- 🏷️Label — typed `useLabel`/translation label + property-layout ResizeObserver (`:230-233`).
- 🐚️ShellScope — per-shell scope: root, **portal layer**, storage, i18n instance, selection (`:51-71`).
- 👥️PresenceBar — avatar bar with "+N" overflow.
- 💡️ChromeControlHint — hover tooltip portal for chrome controls.
- 💬️Dialog — modal dialog, preventable events, portal (`💬️Dialog/🟦️.tsx:396`).
- 📃️List ✱ — TUI only.
- 📊️Table — table with sortable header, row grid.
- 📋️MenuItem — shared menu-row presentation.
- 📐️Layout — mobile/desktop panel configuration.
- 📑️Tabs — tabs state + roving focus.
- 📚️I18n — `UiI18nPort`, `UiLabelValue`, the typed chrome translation schema (`:108-861`).
- 📜️Scrollable — native overflow scroll host.
- 📝️Field — labelled form field wrapper.
- 📨️UIDialog — staged-form confirm dialog.
- 📻️TableAvatar — owned avatar.
- 🔀️Toggle — toggle button/switch.
- 🔌️Ports — `reactHostPort`, `sceneHostPort`, flow/interactive-job ports (`🔌️Ports/🟦️.tsx:28-100`).
- 🔑️KeyValue ✱ — wgpu only.
- 🔘️Button — button.
- 🔚️Footer — footer bar.
- 🔝️Navbar — navbar + brand logo (ResizeObserver+MutationObserver `:157,164`).
- 🔣️Icons — `Icon` + built-in and "metabolism" catalogs.
- 🔤️Textarea — textarea.
- 🔲️WindowSilhouette — pure geometry of the U-cutout outline (chips/gaps → polygon/SVG path) (`:8-401`).
- 🔳️ButtonGroup — button group.
- 🔽️Select — select/dropdown with portal (`🔽️Select/🟦️.tsx:653`).
- 🕰️HistoryTable — VCS checkpoint table.
- 🕸️Diagram — xyflow graph + an **owned force simulation** with per-frame budget (`🕸️Diagram/🟦️.tsx:240-242,324,797-856`).
- 🖱️ContextMenu — context menu with fused submenu wings.
- 🖼️Panel — panel + `usePointerDrag` (`🖼️Panel/🟦️.tsx:93-110`).
- 🗂️WindowChrome — **the U-cutout window chrome** all cards/dialogs/menus are built on; see section 4.
- 🗨️Popover — floating popover with portal + ResizeObserver (`:329,404`).
- 🚗️UiDriver — customisation "driver" axes (`:25-39`).
- 🚧️WindowContentDeadLine — CSS-var clearance below floating chrome.
- 🥞️LayeredOverview — strip of live pages + one glass + card grid; see section 4.
- 🦴️Skeletons — loading skeletons (`animate-pulse` with reduced-motion guard).
- 🧙️Wizard ✱ — TUI only.
- 🧪️NavbarExampleSelect — "No example" navbar picker.
- 🧭️PanelTabBar — panel tab strip.
- 🧱️DragHandle — grip that starts a drag.
- 🧾️Form — native form boundary.
- 🪙️Chip ✱ — TUI only.
- 🪜️Stepper — numeric stepper with press-and-hold.
- 🪟️Window — floating window (ResizeObserver `:108,255`).
- 🪵️Log ✱ — TUI only.

Note: `UI/🔨️modules/` and the barrel are not "elements"; there is **no** `.css` file in any element
(`find UI -name '*.css'` → only `🌐️globals`, `🎨️styling/*`, `🧵️styles`). See 3.

---

## 2. Package layout, exports, registration

### 2.1 Implementation vs glue

Implementation:
- the React barrel `REACT/🟦️.tsx` (8 734 lines; region map at the `// #region` lines, e.g. `🃏️OverviewCard` `:5278`, `🥞️LayeredOverview` `:5283`, `🏷️Field`/inputs `:6002-6150`, framework re-exports `:8643-8730`);
- elements in `UI/🧱️elements/*/🟦️.tsx`; pure logic in `UI/🔨️modules/*/🟦️.ts`;
- `REACT/🌓️appearance/🟦️.ts` (496 lines), `REACT/🌐️i18n/🟦️.ts` (2 281), `REACT/🪟️chrome/🟦️.ts` (107), `REACT/🎠️runtime/🟦️.ts` (111), `REACT/🖌️render/🟦️.ts` (172, test adapter).

Glue (all three-line files):
- `PKG/🟦️.tsx:1-3` — `export * from "../../🟦️.tsx"; export * from "../../🎠️runtime/🟦️.ts";`
- `PKG/🟦️.ts:1-3` — `export * from "../../🖌️render/🟦️.ts"; export * from "../../🟦️.tsx"; export { default } from "../../🛠️build-tooling/🎨️styling/🟦️.ts";`
- `PKG/🟦️.mts:1` — postcss default.
- `PKG/📜️script.ts` (401 lines) is the only script file (`ScriptRouter` at `:391-401`: `dev build lint test canonical-architecture typecheck check-ui-primitives check-chrome-i18n`).
- `PKG/📋️project.json` registers the nx targets (`:18-116`); `namedInputs.default` (`:5-17`) already includes `UI/🧱️elements/**`, `UI/🔨️modules/**`, `UI/🧬️schema/**`, `UI/🧫️fixtures/**`, so new files in those folders invalidate the cache with no edit.

### 2.2 package.json (`PKG/package.json`)

```
:4   "name": "@semio-tech/ui-react"
:7   "type": "module"
:13  "exports": {
:14    ".": "./🟦️.tsx",
:15    "./runtime": "./🟦️.tsx",
:16    "./i18n": "../../🌐️i18n/🟦️.ts",
:17    "./chrome": "../../🪟️chrome/🟦️.ts",
:18    "./test": "./🟦️.ts",
:19    "./components.json": "../../../../components.json",
:20    "./🧵️.css": "../../../../🧵️styles/🎨️.css",
:21    "./🌐️globals-ui.css": "../../../../🌐️globals/🎨️.css",
:22    "./🌓️theme.css": "../../../../🎨️styling/🌓️theme/🎨️.css",
:23    "./postcss.config": "./🟦️.mts"
:24  },
```

- Dependencies (`:31-50`): `@semio-tech/{assets,ui-styling,framework}` workspace, `@dnd-kit/*`, `@react-three/{drei,fiber}`, `@xyflow/react`, `i18next`, `react-i18next`, `pdfjs-dist`, `react`/`react-dom` 19.2, `react-resizable-panels`, `three`, `three-mesh-bvh`, `xstate`.
- Dev/test-only (`:51-70`): `@testing-library/react`, `vitest 4`, `@vitest/coverage-v8`, `d3-color`, `d3-ease`, `polygon-clipping`, tailwind, `@vitejs/plugin-react`.
- `:82 "bundleKind": "ui"`; `:8-12 "semio": {dependencyRole: "ui", role: "framework", id: "ui-react"}`.
- Observation: `"./runtime"` (`:15`) points at the **same file as `"."`**; the real `🎠️runtime/🟦️.ts` is reached via the glue's second `export *` (`PKG/🟦️.tsx:3`), and some consumers alias `/runtime` straight to `🎠️runtime/🟦️.ts` (e.g. `♻️mit-bestand/🧺️demonstrator/🏗️builder/🌐️vite/🟦️.ts:57`).
- ui-react's own `tsconfig.json:21-26` only lists paths for `.`, `/runtime`, `/test`; `/chrome` and `/i18n` resolve through package.json `exports` (`moduleResolution: bundler`, `:7`).
- Workspace registration is an explicit list: root `package.json:105-106` (`UI/🎨️styling/…` and `REACT/…`), `:111-114` for the quiz/presentation targets. A new package needs a new line there.
- Package export test: `UI/🧪️tests/📦️react-package-export/🟦️.ts:10-17` only asserts `"."` → `./🟦️.tsx`, the self-alias path and that the glue contains `from "../../🟦️.tsx"`.

### 2.3 How things get registered

New **element**:
1. `UI/🧱️elements/<emoji><Name>/🟦️.tsx` (+ `📖️stories/🧪️.story.tsx`, `🧪️tests/🧩️component/🟦️.tsx`).
2. Add an `import … from "../../🧱️elements/<…>/🟦️.tsx"` + `export { … }` region in the barrel (template: OverviewCard `REACT/🟦️.tsx:5278-5281`, LayeredOverview `:5283-…`).
3. For the slim import surface also add it to `REACT/🪟️chrome/🟦️.ts` (template `:14-37`).
4. **Tests are not globbed.** `REACT/🧪️tests/🎚️config/🟦️.ts:24-62` is an explicit `include` array (jsdom, `:23`); a new test file does nothing until its path is listed there (OverviewCard `:50`, LayeredOverview `:51`, geometry unit test `:57`). In-source tests inside the barrel run via `includeSource: ["../../🟦️.tsx"]` (`:63`).
5. Storybook: stories are discovered by the root `.storybook` (scope `ui`, title prefix `🖱️ui⚛️react`, e.g. story ids `🖱️ui⚛️react-skeletons--table` in `UI/🧪️tests/📚️storybook-new-stories/🟦️.ts:15-60`); `.storybook/main.ts:55,60` aliases `@semio-tech/ui-react{,/test}` to the glue files.
6. No CSS file: see section 3.

New **subpath** export (`@semio-tech/ui-react/<x>`):
1. Entry file under `REACT/<emoji><x>/🟦️.ts` (pattern: `🪟️chrome`, `🌐️i18n`).
2. `PKG/package.json` `exports` (`:13-24`).
3. **Every consumer aliases subpaths by hand** — the quiz does it three times: Vite builder `🎓️teaching/🏛️architecture/❓️quiz/🏗️builder/🌐️vite/🟦️.ts:135-137`, vitest `QUIZ-R/🧪️tests/🎚️config/🟦️.ts:23-24`, tsconfig paths `QUIZ-R/📦️packages/🟦️typescript/tsconfig.json:12-13`. A brand-new subpath therefore means edits in every consumer; adding the pets to the **existing `/chrome`** needs none.

`/chrome` (`REACT/🪟️chrome/🟦️.ts`) currently exports: `cn`, `WindowChrome` (+types), the `OverviewCard*` family, `LayeredOverview` + all geometry helpers + types, `Navbar`/`SemioLogo`/`ShellBrandLogo`, `Icon`, `DEFAULT_UI_DRIVER`, the device constants/queries, the document surface-chrome API (`useElementsSurfaceChrome`, `bootstrapElementsSurfaceChromeDocument`, `read/writeStoredUiChromeAppearance`, `useMediaQuery`) and the presence palette (`presenceColor/presenceCssVar/presencePaint`) (`:13-107`). Not on `/chrome`: `SurfaceScope/Level`, `useShellFloatingSurfaceHost`/ShellScope, `measureWindowSilhouetteMetrics`/`useWindowSilhouetteGeometry` (both exist but are only on the barrel, `REACT/🟦️.tsx:5246-5247`).

`/i18n` (`REACT/🌐️i18n/🟦️.ts`): `registerUiTranslationBundles`, `uiI18n`, `setUiLocale`, `initUiLocaleSync`, `createShellI18nInstance`, `detectShellLocale`, `readStored/writeStoredUiChromeLocale`, `resolveUiLabel`, key types (`:27-45,2093-2283`).

Consumer import surface seen: quiz target → `/chrome` (`QUIZ-R/🟦️.tsx:32`, `🔨️modules/🏠️home/🟦️.tsx:22`, `🪟️chrome/🟦️.tsx:13`, `👥️presence`, `🏆️leaderboard`, `🏅️badges`, `▶️run`) and `/i18n` (`🔨️modules/🌐️i18n/🟦️.ts:14`); tests → `/test` (os renderer elements, plugins); demonstrator/play → `/runtime`.

---

## 3. Styling system

### 3.1 Where styles live

- **Tailwind v4, one stylesheet.** `STYLING/🖌️ui/🎨️.css:2` `@import "tailwindcss"`, `:4` typography plugin, `:14` `@source "../../"` (the whole `UI` tree is scanned; the comment `:5-13` documents that without it utilities used only inside `🧱️elements` such as `.z-menu` were dropped from the served CSS). Consumers import `@semio-tech/ui-react/🧵️.css` → `UI/🧵️styles/🎨️.css:2-6` (ui css + globals + palette).
- **Elements ship no CSS of their own.** They carry Tailwind utility strings (`cn(...)`, from `UI/🔨️modules/🏷️class-name-composition/🟦️.ts`, extended Tailwind-merge groups) and `data-*` hooks; anything needing `@keyframes`/`@property` is appended to the big `🖌️ui/🎨️.css` (e.g. `introduction-demo-*` `:1500-1777`, border clocks `:1043-1470`, `IconAnim` from `:1779`) or to the consumer's stylesheet (`QUIZ-R/🎨️.css` has `quiz-icon-*` keyframes). `LayeredOverview` adds zero CSS: it writes transforms/clip-paths imperatively and uses `.ui-veil`.
- Generated tokens: `STYLING/🔣️.json` (top keys `colors spacing fontStacks fontFaces strokes radii opacities metrics levels presence appearances`, `:3-418`) → `STYLING/🤖️generated/*` (CSS + `🔤️tokens/🟦️.ts`) + Rust/Python/.NET twins; regenerate with `bun ./📜️script.ts generate` in `STYLING/📦️packages/🟦️typescript` (`📜️script.ts:2,33`). Package `@semio-tech/ui-styling` exports `.`, `./🖌️ui.css`, `./🎨️palette.css`, `./tokens.generated` (`STYLING/📦️packages/🟦️typescript/package.json:23-28`).

### 3.2 Token definitions relevant to an overlay layer

`STYLING/🖌️ui/🎨️.css`:

```css
854  /* #region 🪜️LevelZScale — formula z(k) = k * --z-step (zStep=10), k=0..5 for base..menu */
856  --z-base: 0;      857  --z-window: 10;   858  --z-pane: 20;
859  --z-panel: 30;    860  --z-dialog: 40;   861  --z-menu: 50;
863  --z-navbar: 100;  864  --z-modal: 1000;  865  --z-tutorial: 10000;
866  --glass-saturate: 1.45;
871  --level-shade-step: 5%;   872 --glass-alpha-step: 0.12;   873 --glass-blur-step: 0.5rem;
874  --element-shade-step: 6%; 875 --hover-shade-step: 12%;
876  --veil-alpha: 0.4;        877 --veil-blur: 0.5rem;
```
(source of truth `STYLING/🔣️.json:400-412` `levels`: `zStep 10`, `glassAlphaStep 0.12`, `glassBlurStepPx 8`, `veilAlpha 0.4`, `veilBlurPx 8`.)

Utilities `z-base/window/pane/panel/dialog/menu/navbar/modal/tutorial` at `:6996-7034`; per-level surface cascade `[data-level="base|window|pane|panel|dialog|menu"]` setting `--surface-bg / --surface-alpha / --surface-blur / --border-element-color / --surface-hover` at `:933-980`; fills `ui-surface`, `ui-glass`, `ui-veil` at `:7046-7060` (the veil's host must carry `data-level="dialog"`, `:7040-7042`).

Spacing / size / shape (`@theme inline`, `:754-804`):

```css
755  --ui-spacing: var(--spacing-compact);   /* 0.2rem; `.touch` sets --spacing-touch 0.275rem at :654-656 */
756  --spacing-single: calc(1 * var(--ui-spacing));   757 --spacing-double: calc(2 * …)
758  --size-tiny: calc(3 * …)  759 --size-small: calc(5 * …)  762 --size-medium: calc(7 * …)  764 --size-large: calc(9 * …)
792  --stroke-hairline: 1px;  793 --stroke-default: calc(2 * …);  794 --stroke-focus: calc(3 * …);
795-804  --radius… : 0rem   (EVERY radius is 0 — sharp corners; shadows are all transparent, :805-820)
```

Brand/colour tokens: generated `--color-primary #ff344f`, `secondary #34d1bf`, `tertiary #fa9500`, `danger #a60009`, `warning #fccf05`, `info #dbbea1`, `success #7eb77f`, `dark #001117`, `light #f7f3e3` (`STYLING/🎨️palette/🎨️.css:170-224`, the concatenated palette entry), 12 presence hues `--presence-0..11` per appearance (`STYLING/🎨️palette/🎨️.css:226-270`, source `STYLING/🔣️.json:413-417`). Semantic aliases `--base --foreground --muted --muted-foreground --accent --active-base --border-normal-color --border-emphasized-color --border-element-color` at `🖌️ui/🎨️.css:90-124`.

### 3.3 Motion tokens — **there is no global motion scale**

No `--duration-*`, `--ease-*` or `--motion-*` token exists anywhere in `STYLING` (grep of `🔣️.json` for duration/easing/motion/transition: no hits; `🖌️ui/🎨️.css` hits only effect-specific ones). What exists:

- Registered per-effect clocks, all `@property`: `--loading-border-duration 1.6s` (`:1067`), `--waiting-border-duration 3.2s` (`:1091`), `--introduced-border-duration 1.6s` (`:1116`), `--celebrate-border-duration 1.2s` (`:1134`) — they are `@property` registered specifically so Tailwind's production pruning does not drop them (`:1061-1066`).
- Ad-hoc literals: chrome-frame `transition: border-color 120ms ease` (`:7117`), ghost/reveal `150ms` (`🌐️globals/🎨️.css:139,180`), OverviewCard hover lift `duration-200` (`OverviewCard/🟦️.tsx:112`), icon micro-animations `--icon-animation-duration` default `500ms` with per-icon 400-800 ms (`:1792`, `:1835-1960`).
- JS constants (pure, in the geometry module): `LAYERED_GLIDE_MS = 500` (`🥞️layered-overview-geometry/🟦️.ts:130`), `LAYERED_FOLLOW_LERP = 0.12` per frame (`:133`), `LAYERED_FOLLOW_EPSILON = 1e-4` (`:136`), `easeInOutCubic` (`:139-141`). Cubic in-out is re-implemented 4× (geometry `:139`, `introductionDemoEaseInOutCubic` used at `REACT/🟦️.tsx:3227`, inline in Scene `🎬️Scene/🟦️.tsx:1901`, and d3-ease in the test oracle).
- No physics/spring helper anywhere (grep for spring/damper/verlet/IK in `🧰️framework/🔨️modules` found nothing relevant; `AnimationMixer`/`SkinnedMesh` also absent). The closest "physics" is the Diagram's owned force simulation (`🕸️Diagram/🟦️.tsx:240-242` budget `{maxTicksPerFrame:4, maxFrameMs:6, …}`, alpha decay `:242`, frame scheduler `:797-815`, deadline `:844-856`).

### 3.4 Theming

- Appearance = **`.dark` class on the appearance root** (a shell's `.semio-scope` root, not necessarily `documentElement`): `REACT/🌓️appearance/🟦️.ts:230-236` (`root.classList.toggle("dark")`, `dataset.uiAppearance`, `colorScheme`), `ElementsSurfaceAppearance = "system"|"light"|"dark"` (`:21`), system via `matchMedia("(prefers-color-scheme: dark)")` (`:69-75,299-307`). CSS: light on `:root,[data-ui-theme],.semio-scope` (`🖌️ui/🎨️.css:90-107`), dark on `.dark` (`:109-124`).
- Theme model: `UiTheme` (colours, spacing, font stacks, strokes, metrics, `appearances.{light,dark}`) in `STYLING/🌓️theme/🟦️.ts`; `semioTheme()` `:600`, `builtinUiThemes()` `:607`, `applyUiThemeToRoot(root, theme)` `:660-699` writes `--color-*`, `--theme-{light|dark}-chrome-*`, `--spacing-*`, fonts, `--stroke-hairline`, glass knobs as inline vars and stamps `data-ui-theme` (per root, so several shells coexist); `setActiveUiTheme` `:727`. Custom themes persist under `ui.themes.custom` (`REACT/🟦️.tsx:1748-1773`).
- Device = `.touch` class + `data-ui-device` (`appearance:270-276`); breakpoints `UI_MOBILE_MAX_WIDTH_PX 767` / `UI_TABLET_MAX_WIDTH_PX 1023` (`UI/📱️device/🟦️.ts`, quoted in `chrome:95`).
- **High contrast** has no named theme. It is handled by media queries in the stylesheet: `@media (forced-colors: active)` (`🖌️ui/🎨️.css:7085-7102`: glass → `Canvas/CanvasText`, silhouette strokes → `CanvasText`) and `@media (prefers-reduced-transparency: reduce)` (`:7072-7083`: no backdrop-filter, solid `--surface-bg`) plus `@supports not (backdrop-filter)` fallback (`:7066-7070`). WCAG helpers exist in code: `contrastRatio`, `wcagContrastGrade`, `readableForegroundHex` (`STYLING/🌓️theme/🟦️.ts:387-500`), with a quiz test `🧪️tests/🌗️contrast-states`.
- `prefers-reduced-motion` conventions (three, all in use):
  1. CSS `@media (prefers-reduced-motion: reduce)` blocks that set `animation: none` (`🖌️ui/🎨️.css:1204,1248,1428-1470,1758-1777,1813-1817`).
  2. Tailwind variants `motion-reduce:transition-none` / `motion-reduce:animate-none` (OverviewCard `:112,135,163`, LayeredOverview `:363,818`, quiz `🪟️chrome:64`).
  3. JS: `LayeredOverview` `useReducedMotion(setting)` with prop `reducedMotion: "auto"|"always"|"never"` and a live `matchMedia("(prefers-reduced-motion: reduce)")` listener (`🥞️LayeredOverview/🟦️.tsx:155,169-180`; snaps instead of gliding `:578,594`); Introduction demo reads it once and **renders nothing** (`REACT/🟦️.tsx:3097,3341`).

---

## 4. Existing animation / physics / pointer / overlay / observer / drawing primitives

### 4.1 `🥞️LayeredOverview` and `🃏️OverviewCard` — what pets must stand on

**DOM produced by `LayeredOverview`** (`UI/🧱️elements/🥞️LayeredOverview/🟦️.tsx`), strip mode (`:849-878`):

```
div[data-layered-overview][data-mode="strip"][data-rest="grid"|"panorama"]      relative h-full w-full overflow-clip bg-background   (:851)
├─ button[data-layered-overview-button][data-level="dialog"]  (only while a page is opened)  ui-glass absolute right/top z-40 (:810-823)
├─ div[data-layered-strip] (grid rest: relative, contain:layout paint | panorama: grid, will-change-transform) (:853-867)
│   └─ div[data-layered-pane="<id>"][data-opened]  × N   relative overflow-hidden bg-background,
│        style contain:"layout paint", `inert` unless opened, aria-hidden unless opened (:391-404);
│        grid rest: position:absolute inset:0 transform-origin:0 0 will-change:transform (GRID_PANE_STYLE :163), transform+clip-path written per frame (:519-528)
├─ div[data-layered-veil][data-level="dialog"][data-veil="whole"|"hole"|"clear"]  className "ui-veil pointer-events-none absolute inset-0 z-30" (:870)
│        clip-path polygon(evenodd …) punches the "hole" over the revealed page; visibility hidden when "clear" (applyVeil :213-217)
├─ div[data-layered-overlay][role="group"][aria-label]  "pointer-events-none absolute inset-0 z-[31]" + app overlayClassName/overlayStyle with --layered-columns/--layered-rows (:871)
│   └─ div[data-layered-card="<id>"][data-revealed]  className "contents"; onPointerEnter/Leave (mouse only) reveal/conceal, onFocus/onBlur (:434-443)
│        └─ whatever `renderCard` returns (the app's card)
└─ {renderChrome(state)}   last child, inside the overflow-clip root (:876, prop :108)
```

List mode (touch phones, `:825-846`): `div[data-layered-overview][data-mode="list"]` > `div[data-layered-list][role="group"]` (snap-y scroller) > `section[data-layered-section="<id>"]` each with its own pane, `div[data-layered-veil]` (`ui-veil pointer-events-none absolute inset-0 z-30`, `:836`) and `div.pointer-events-none … z-[31] … overflow-y-auto` carrying the card host (`:838`).

Behaviour that matters for walking pets:
- Glass = ONE `ui-veil` with `backdrop-filter: blur(var(--veil-blur)) saturate(…)` (`🖌️ui/🎨️.css:7056-7060`); pages behind it are inert and blurred; the hovered/focused card's page becomes "clear" by a hole in the veil (geometry `veilClip/veilForRect/veilPolygon`, `🥞️layered-overview-geometry/🟦️.ts:194-218`).
- Pointer events: veil and overlay are `pointer-events-none`; **only the cards re-enable them** (`OverviewCard` button: `pointer-events-auto` `OverviewCard/🟦️.tsx:111`; section cards get it from the app, e.g. quiz `🪟️chrome/🟦️.tsx:64`, test `🥞️LayeredOverview/🧪️tests/🧩️component/🟦️.tsx:21`).
- Motion: rAF loop `run/tick` (`:544-573`) drives pointer-follow (lerp 0.12/frame, mouse only, window `pointermove` passive `:758-769`), 500 ms cubic glides and zoom; transforms are written imperatively in percent (no render per frame). Epoch counter guards stale ticks (`:538-542`). Scheduling helpers `scheduleIdle`/`requestIdleCallback` for warm boots (`geometry:341-360`).
- Reduced motion: snaps (`:578,594`) and disables the pointer pan (`:759`).
- The root is `overflow-clip` (strip) / `overflow-hidden` (list), panes are `contain: layout paint` → **anything rendered inside root or a pane is clipped to the overview box**; a pet that must rise above the top row's card edge should live in a sibling layer above the overlay (z > 31) or in the shell portal layer.
- Cards lift on reveal: `OverviewCard` button `hover:-translate-y-0.5` + `transition-transform duration-200` (`OverviewCard/🟦️.tsx:112-114`), quiz section cards `-translate-y-0.5` while `data-revealed` (`QUIZ-R/🔨️modules/🪟️chrome/🟦️.tsx:64`). So an edge moves 2 px over 200 ms → read `getBoundingClientRect()` per frame, never cache.

**DOM produced by `OverviewCard`** (`UI/🧱️elements/🃏️OverviewCard/🟦️.tsx:62-121`):

```
section[data-overview-card][aria-labelledby]  (as="section", :97)        | button[data-overview-card][data-hover-scope] (as="button", :103-118, pointer-events-auto, lifts)
└─ WindowChrome level="dialog" active=false stackSlot="<slot>-stack" stackDataAttrs={data-overview-card-stack} (:65-71)
```
`WindowChrome` (`UI/🧱️elements/🗂️WindowChrome/🟦️.tsx:395-622`) renders the U-cutout:

```
div[data-slot="<slot>-stack"][data-window-silhouette][data-level="dialog"][data-overview-card-stack]   relative flex flex-col overflow-visible bg-transparent (:496-505)
├─ svg|div[data-window-silhouette-border][data-kind=normal|active|loading|waiting|introduced|celebrated]  pointer-events-none absolute inset-0 z-[40] (:308-390)
├─ div[data-slot="window-chrome-cap"][data-ui-reveal-region="window-cap"]  style zIndex 2 (:507-514, constant :242)
│   ├─ div[data-slot="window-chrome-chip-cap"][data-window-silhouette-chip][data-dock="top"]  glass  → title chip `<slot>-title-chip` (:515-519)
│   ├─ div[data-slot="window-chrome-gap"][data-window-silhouette-gap]  pointer-events-none transparent U-gap (:520)
│   └─ (controls cell, absent on cards)
├─ div[data-slot="window-chrome-body-surface"][data-level]  aria-hidden pointer-events-none absolute glass behind the body (:551-560)
├─ div[data-slot="window-chrome-body"][data-window-silhouette-content]  style zIndex 1, clipPath = silhouette content clip (:561-572)
└─ div[data-slot="window-chrome-footer"] with [data-window-silhouette-chip][data-dock="bottom"] chips (:573-617)
```
So a card's **top edge is not a straight line**: the title chip (a glass cap of `min-h-medium` = `7 × --ui-spacing`) stands up on the left, the "U-gap" to its right is transparent and the body's top border sits one cap-height lower. Exact edge geometry already exists as data:
- `measureWindowSilhouetteMetrics(stack)` (`WindowChrome:136-166`) → `{width,height, top:{depth, chips:[{left,right}]}, bottom:{…}}` in the stack's own pixels, compensating CSS scale (`windowSilhouetteScale :106-109`).
- `useWindowSilhouetteGeometry(stack)` (`:169-221`): rAF-coalesced `ResizeObserver` + `MutationObserver` re-measurement; returns `WindowSilhouetteGeometry` whose `outline: WindowSilhouettePoint[]` is the full polygon (`UI/🧱️elements/🔲️WindowSilhouette/🟦️.tsx:71-84`, `windowSilhouetteOutline :225`, `windowSilhouetteContains :366`, inset `WINDOW_SILHOUETTE_PATH_INSET = 1` `:85`).
- Both are barrel-only (`REACT/🟦️.tsx:5246-5247`), not on `/chrome`.
- Z inside the chrome is stated **inline** (`WINDOW_CHROME_CHIP_ROW_STYLE {zIndex:2}`, `WINDOW_CHROME_BODY_PLANE_STYLE {zIndex:1}`, `WindowChrome:242-244`) because the Tailwind arbitrary utilities `z-[2]` once computed to `z-index:auto` (comment `:232-241`); `@source "../../"` (`🖌️ui/🎨️.css:14`) was added later, but `LayeredOverview` still uses `z-[31]` (`:871`) — prefer inline `style={{ zIndex }}` / `var(--z-*)` for a new layer.
- App-level hooks seen on cards: quiz adds `data-card`, `data-presence-anchor`, `data-revealed` via `OverviewCard.data` (`QUIZ-R/🔨️modules/🪟️chrome/🟦️.tsx:44-50`); grid cell wrapper `.quiz-home-cell` (`QUIZ-R/🎨️.css:177-188`).

### 4.2 Inventory (path:line — description)

**rAF scheduling / animation loops**
- `🥞️LayeredOverview/🟦️.tsx:544-573` — rAF tick for follow/glide/zoom; `:507-517` imperative `paint`; `:773` rAF-throttled list scroll.
- `REACT/🟦️.tsx:3068-3341` — `IntroductionDemonstrationOverlay`: ghost-cursor phases (`appear→travel→press→dragMove→release→linger→fadeOut→pause`), imperative transforms, SVG quadratic-bezier trail (`:3151-3170`), ripples; hides under reduced motion. Closest existing "animated decorative actor".
- `REACT/🟦️.tsx:4013-4045` — `createTutorialClock`: rAF-driven external store (`t += dt*rate`), read via `useSyncExternalStore`.
- `REACT/🌓️appearance/🟦️.ts:171-179,129-218` — chrome-reveal controller: one window `pointermove` per root, rAF-coalesced hit test against `[data-ui-reveal-region]` rects (band 24 px / edge 8 px).
- `WindowChrome:169-221` — rAF-coalesced geometry commit.
- `⭕️Ring/🟦️.tsx:130-175` — rAF-throttled drag flush.
- `🕸️Diagram/🟦️.tsx:324-1000` — owned force simulation with frame budget (see 3.3).
- `🎬️Scene/🟦️.tsx:1834,1855,1895-1927,2000-2007` — camera restore + eased snap.
- `🌳️Tree/🟦️.tsx:3353,3817`, `🏷️Label/🟦️.tsx:230` — rAF flushes.
- Quiz precedent (consumer, not UI): `QUIZ-R/🔨️modules/👥️presence/🟦️.tsx:960-1030` — peers' cursors in a decorative overlay re-placed via rAF on resize/scroll/focusin + 500 ms interval, glide unless reduced motion.

**Pointer tracking / drag**
- `🥞️LayeredOverview/🟦️.tsx:758-769` window `pointermove` (mouse only, passive).
- `REACT/🟦️.tsx:2306-2343` — ghost controller: document-capture pointerdown/move/up with a 4 px threshold (`PANEL_GHOST_MOVE_THRESHOLD_PX :2185`).
- `REACT/🟦️.tsx:2900-2960` — `useIntroductionPointerIdle`: window-capture pointer/wheel/key idle timer.
- `🖼️Panel/🟦️.tsx:93-110` — `usePointerDrag` (pointer capture). `🎚️Slider/🟦️.tsx:349`, `↔️Resizable/🟦️.tsx:66-95`, `🎨️Canvas/🟦️.tsx:1575,1758`, `🎬️Scene/🟦️.tsx:1600-1647`, `🌳️Tree/🟦️.tsx:3934`.
- dnd-kit (`@dnd-kit/core|sortable|utilities`) imported at `REACT/🟦️.tsx:124-126`, re-exported `:8648-8653`; `🧱️DragHandle`.
- Listener hygiene helper: `createDOMEventBinding()` (`UI/🔨️modules/👂️dom-event-binding/🟦️.ts:11-23`: `listen(target,type,fn,opts)` + `dispose()` in reverse order) — used by appearance, ghost controller, Ring.
- Shared gesture maths (non-UI): `🧰️framework/🔨️modules/🕹️interaction/👆️gesture/🟦️.ts`.

**Portal / overlay / layer**
- `ShellScope.portalLayerRef` (`🐚️ShellScope/🟦️.tsx:55-57`): "fixed-position overlay layer, last child of the shell root"; `useShellFloatingSurfaceHost()` (`:135-143`) returns it or `document.body`. Rationale `:124-134`: appearance is a **scope**, so a portal to `body` would read the light palette inside a dark shell — pets must use this host.
- Portals in use: Popover `🗨️Popover/🟦️.tsx:404`, Select `:653`, Dialog `:396`, ContextMenu `:101`, ChromeControlHint `:89`, Tree drop preview `:1215-1220` (`fixed z-tutorial pointer-events-none`), Introduction `REACT/🟦️.tsx:3792`, CanvasPickMenu `:1074` (`fixed z-tutorial`).
- Overlay stacking idiom: `z-tutorial` + inline `zIndex: "calc(var(--z-tutorial) + 2)"` for the introduction box and demo (`REACT/🟦️.tsx:3344,3713`); positioned `absolute` inside the portal layer when it exists, else `fixed` (`:3089,3683`) with host-relative coordinates (`introductionPointRelativeToHost`).
- Level machinery: `LevelProvider/useLevel/getLevelZClass` (`🌈️Surface/🟦️.tsx:21-52`).
- Ghost mode (`[data-ghost]`, `[data-dim]`) hides open chrome while dragging: `🌐️globals/🎨️.css:134-168`.

**ResizeObserver / MutationObserver**
WindowChrome `:189,201,325`; Navbar `:157,164`; Window `:108,255`; Popover `:329`; Select `:571`; Panel `:478`; Input `:274`; Label `:233`; WindowContentDeadLine `:109`; ElementId `:107` (MO); Tree `🎯️focus/🟦️.ts:35` (MO); Scene `:948,1704,1760` (MO); appearance `:386` (MO on class/style/data-ui-*); barrel `REACT/🟦️.tsx:2515,2521,2600,3447,3572,3587,6564,6625`. jsdom setup mocks `ResizeObserver` as a no-op class (`UI/🧪️tests/🧹️react-environment/🟦️.ts:7-13`); `MutationObserver` is jsdom-native.

**Canvas / SVG drawing helpers**
- `capturePosterFromCanvases` (`🥞️LayeredOverview/🟦️.tsx:127-151`) — composites `<canvas>` children to a PNG data URL.
- `WindowChromeSilhouetteBorder` SVG path + `foreignObject` masked conic fill (`WindowChrome:308-390`).
- `IconRenderPort` (`REACT/🟦️.tsx:262-615`): three.js `SVGRenderer`/GLB → SVG/PNG icon rendering.
- Ring SVG circles, Intro SVG trail/mouse glyph (`REACT/🟦️.tsx:3345-3370`), xyflow SVG edges, R3F WebGL Scene, `🎨️Canvas` 2D element.
- GLB/three exports for any rigged 3D path: `useGLTF`, `Clone`, `GLTFLoader` (`REACT/🟦️.tsx:127,139,8655-8697`) — **no skeletal-animation helpers** (`AnimationMixer`, `SkinnedMesh`) exist.

---

## 5. i18n in the UI module

- **Port**: `UiI18nPort { t, tIn(locale,key), exists(key), changeLanguage, language, resolvedLanguage, isInitialized }` (`📚️I18n/🟦️.tsx:894-904`); i18next/react-i18next stay behind it (`REACT/🌐️i18n/🟦️.ts:1-9`).
- **Locales**: exactly `en` and `de` (`UiLocale = ShellLocale`, `📚️I18n:12`; `🎚️axes/🔣️.json` locales `en`/`de`). Order "English first, German second".
- **Leaf shape** `UiLabelValue = { label: { normal, beginner }, manual?, tutorial? }` (`:18-22`); `resolveUiLabel(value, tier)` (`:34-41`). The tier follows the active `UiDriver.labelTier`.
- **Registering strings**: the domain-neutral chrome tree is typed `UiTranslationSchema` (`📚️I18n:108-861`) and filled for both locales in `uiChromeTranslationBundles` (`REACT/🌐️i18n/🟦️.ts:104-2088`, `satisfies Record<UiLocale,{translation: UiTranslationSchema}>` so a missing de/en key is a compile error). Products add their own via `registerUiTranslationBundles(bundles)` (`:2117-2122`), which requires **both** locales with the same schema type, merges into the singleton and every live per-shell instance, and returns a caster to a branded `UiRegisteredTranslationKey` (`:2111-2122`). Example consumer: `QUIZ-R/🔨️modules/🌐️i18n/🟦️.ts:14,56-80` (`phrase(normal, beginner = normal)` helper, `QUIZ_BUNDLE_EN`/`DE`).
- **Per-shell instances**: `createShellI18nInstance(locale)` / `disposeShellI18nInstance` (`:2149-2173`), supplied through `ShellScopeProvider`→`I18nextProvider` (`🐚️ShellScope:96-101`).
- **Totality guard**: `UI/🌐️i18n/🧬️schema/🔣️.json` + `📚️I18n/🧪️tests/🔬️translation-totality/🟦️.ts` (first entry of the vitest include list, `REACT/🧪️tests/🎚️config/🟦️.ts:25`); `canonical-architecture` script runs only this test (`PKG/📜️script.ts:50-56`).
- **"No default language" handling** — two layers, deliberately inconsistent:
  - *Elements take their words from the app*: `LayeredLabels` ("always the app's (no default language)", `🥞️LayeredOverview/🟦️.tsx:91-97`); `OverviewCard` has no strings. A pets element should do the same (labels via props).
  - *The chrome singleton still falls back to English*: `fallbackLng: "en"` (`REACT/🌐️i18n/🟦️.ts:2154,2230`), `normalizeUiLocale` returns `"de"` for `de*` else `"en"` (`:2176-2178`), `initUiLocaleSync` persists and sets `documentElement.lang` (`:2273-2279`).
  - *The quiz product implements true "no default"*: explicit persisted choice → first supported browser language → none (ask in all languages), `applyLocale(undefined)` removes `lang` (`QUIZ-R/🔨️modules/🌐️i18n/🟦️.ts:1-12,48-54`). Product README line: "Every learner-visible text carries English and German; there is no default language" (`🧰️framework/🛍️products/❓️quiz/README.md`).
- **Lint**: `check-chrome-i18n` (`PKG/📜️script.ts:182-389`) flags `?? "Capitalised"` fallbacks, bare capitalised JSX text and `as UiLabel` casts. Scanned roots (`:202-208`) are `UI/🎯️targets/⚛️react` (not `🧱️elements`), the os renderer, the demonstrator and play — the quiz is not scanned.

---

## 6. Customisation facilities

- **`UiDriver`** (`🚗️UiDriver/🟦️.tsx:25-39`): axes `labels full|icons`, `labelTier normal|beginner`, `drag handle|surface`, `chrome always|hover`, `gumball always|hover`, `tooltips full|minimal|none`, `hotkeys inline|tooltip|none`; built-ins `DEFAULT_UI_DRIVER`, `COMPACT_UI_DRIVER` (`:41-48`); strict `parseUiDriver` (unknown values throw); persistence `ui.chrome.driver`, `ui.drivers.custom` (`:77-110`); DOM mirrored as `data-ui-driver/-labels/-drag/-chrome-reveal/-gumball-reveal/-tooltips` (`appearance:109-117`). A "pets" axis would have to be added to this closed set (parser, serializer, DOM) — or kept as a consumer preference.
- **Appearance** (`system|light|dark`, `ui.chrome.appearance`), **layout** (`desktop|tablet`, `ui.chrome.layout`), **locale** (`ui.chrome.locale`), **terminology** (`ui.chrome.terminology`, `useUiTerminology`), **theme id/snapshot/custom themes** (`ui.chrome.theme`, `.snapshot`, `ui.themes.custom`), keybinding overrides (`🔨️modules/💾️keybinding-persistence`), introduction-seen flags (`ui.introduction.seen.<app>`), compute workers — all `read/writeStored…(storage: StoragePort)` pairs at `REACT/🟦️.tsx:1696-1825` and `appearance:404-437`. `StoragePort` (`🧰️framework/🔨️modules/🖥️platform/🟦️.ts:234`, browser/memory impls `:536,563`) is injected per shell (`ShellScope.storage`) so two shells never share keys.
- **Surface-chrome lease API**: `applyElementsSurfaceChrome({appearance, device, driver, browserDefaults}, root?)` returns a revert fn (`appearance:325-349`); `useElementsSurfaceChrome` `:359`.
- **Switchers/settings UI are not in the UI module**: the settings/display panels (`ShellSettingsPanel`, `ShellDisplayPanel`) live in the os renderer (story ids in `UI/🧪️tests/📚️storybook-new-stories`); the quiz builds its own (`QUIZ-R/🔨️modules/🎛️preferences/🟦️.tsx:44-62`: `QuizPreferences {locale?, theme, textSize, showCursors, showAnswers, animateIcons}` stored via `LocalStore`; `animateIcons` is exposed as `data-icon-motion="on|off"` on the app root, `QUIZ-R/🟦️.tsx:451`; `showCursors` gates `PresenceOverlay`, `:474`). A "show pets" toggle is therefore a natural sibling of `showCursors`/`animateIcons` (persisted local-only preference), with the UI element taking `enabled`/`reducedMotion` props.
- Ephemeral stores: `ephemeralBox/Map/Set(key, …)` from `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:91-101` are the repo's page-lifetime-surviving module state primitives (used throughout appearance/ShellScope).

---

## 7. Tests in the UI module

- **Runner**: Vitest 4, `environment: "jsdom"`, config `REACT/🧪️tests/🎚️config/🟦️.ts` (root `PKG`, `name: "@semio-tech/ui-react"`, `:14-67`), alias `@semio-tech/ui-react` → `PKG/🟦️.tsx` (`:18`), `setupFiles` `UI/🧪️tests/🧹️react-environment/🟦️.ts` (`:66`): no-op `ResizeObserver`, `scrollIntoView`, pointer-capture stubs, a `PointerEvent` polyfill extending `MouseEvent` (jsdom has none), `cleanup()` after each test (`:7-51`). `@semio-tech/ui-react/test` (`REACT/🖌️render/🟦️.ts`) is the repo-owned adapter over `@testing-library/react` (`render`, `screen`, `within`, `fireEvent` incl. pointer*, `waitFor`, `act`, `cleanup`; also installs its own `PointerEvent` class `:39-57`).
- **Include list is explicit** (`config:24-62`): element component tests, `📱️device`, several `🔨️modules/*` tests, os-renderer `🔎️ShellSearch`, the package-export test, and `.storybook/🧪️tests/*`. There is also a 9.8 k-line in-source registry `UI/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` fed by the barrel's `if (import.meta.vitest)` block (`REACT/🟦️.tsx:8731-8733`).
- **Conventions** (seen in LayeredOverview):
  - Component tests: `<element>/🧪️tests/🧩️component/🟦️.tsx` import `@testing-library/react` directly, fake the clock with `vi.useFakeTimers({ toFake: [setTimeout…, requestAnimationFrame, cancelAnimationFrame, performance, Date] })` and step frames by `act(() => vi.advanceTimersByTime(10))` (`🥞️LayeredOverview/🧪️tests/🧩️component/🟦️.tsx:37-47`), dispatch real `PointerEvent`s on `window` (`:163`), assert `data-*` attributes and inline style strings.
  - Pure logic: `<module>/🧪️tests/🔬️unit/🟦️.ts` reads a **JSON fixture** (`UI/🧫️fixtures/🥞️layered-overview/🔣️.json`), validates it against a **JSON schema** (`UI/🧬️schema/🥞️layered-overview/🔣️.json`, via Ajv) and cross-checks our implementation against **third-party oracles** (`d3-ease` `easeCubicInOut` at 257 samples; `polygon-clipping` for veil areas) — `🥞️layered-overview-geometry/🧪️tests/🔬️unit/🟦️.ts:1-70`. Oracles must be declared in `UI/🔮️oracles/🔣️.json` (`testOnly: true`, `hostPath`, `rationale`; entries `clsx`, `cva`, `polygon-clipping` `:101`, `d3-ease` `:130`).
  - Rust twins (wgpu/tui) test with `🧪️tests/🔬️…/🦀️.rs` and share fixtures.
- **Run commands** (not executed here):
  - `cd "UI/🎯️targets/⚛️react/📦️packages/🟦️typescript" && bun ./📜️script.ts test quick [vitest file filters…]` — levels `fundamental < quick < long < exhaustive` (`🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:956`; `resolveTestLevel` `:1035`; exhaustive turns coverage on); the remaining args go straight to `vitest run` (`vitestRunArguments :2613`).
  - nx: `bun nx run @semio-tech/ui-react:test` | `:test-quick` | `:test-long` | `:test-exhaustive` | `:typecheck` | `:lint` | `:check-ui-primitives` | `:check-chrome-i18n` | `:canonical-architecture` (`PKG/📋️project.json:55-116`).
  - launch.json: per-target pick-lists carry `@semio-tech/ui-react` (e.g. `.vscode/launch.json:20575,20622,…`; `🛠️dev🖱️ui🪁️typecheck` `:4321`, `⚖️check-ui-primitives🖱️ui🎯️targets⚛️react🟦️` `:18629`); seed in `.vscode/🧩️launch.seed.jsonc:2481-2485`.
  - Styling: `cd STYLING/📦️packages/🟦️typescript && bun ./📜️script.ts generate|fonts|test|twin`.
- **Lint gates relevant to new code**: `check-ui-primitives` forbids raw `<button|input|select|form|textarea|dialog|progress|table>`, `<svg` and `component: X` outside `🧰️framework/` and `.storybook/` (`PKG/📜️script.ts:64-180`; allowlist `:68-73` = demonstrator/play files) — so a pets renderer inside `🧰️framework/` may use `<svg>`, but the quiz **app** under `🎓️teaching/` may not; eslint config is `tseslint + eslint-plugin-storybook` only (`.storybook/📖️stories/🧹️linting/🟦️.ts:12-31`).
- Storybook boot-health specs enumerate story ids (`UI/🧪️tests/📚️storybook-new-stories/🟦️.ts`, `📚️storybook-uncovered-components`): new stories are not auto-registered there.

---

## 8. Placement recommendation (evidence-based)

### 8.1 What lives where today

`🧰️framework/🔨️modules/*` (libraries; 40 folders; **none except `🖱️ui` contains any `.tsx` or a `🎯️targets/⚛️react`** — verified by `find … -name '*.tsx'`):
⏪️time-travel, ⏯️tool-run, ⏱️trace, ⏳️async, ◻️2d (2D engine/booleans/text), ⚠️diagnostic, ✍️editor, 🌉️abi, 🌱️value, 🎒️pack (io/format), 🎠️kernel (ephemeral stores), 🎭️actor, 🎯️action-bus, 🏗️mesh-engine, 📏️intrinsic-size, 📐️geometry (engine, random), 📚️compiler, 📡️replication, 🔀️dispatch, 🔄️machine, 🔏️hash, 🔢️number, 🔤️typeset, 🔲️pixels, 🕸️graph, 🕹️interaction (pure gestures), 🖌️raster, 🖥️platform (StoragePort), **🖱️ui**, 🖼️assets (icons/fonts/cursors), 🗜️deflate, 🗺️surface, 🚪️io, 🛂️manifest, 🛠️tool-machine, 🧊️3d (mesh/rigid/collision), 🧩️action-argument-resolution, 🧬️schema, 🧮️math, 🧵️job. Shape: `📦️packages/{🟦️typescript,🦀️rust}` + `🟦️.ts`/`🦀️.rs` + `🧬️schema 🧫️fixtures 🧪️tests`.

`🧰️framework/🛍️products/*` (`🛍️products/🔣️.json:4-41` lists responsibilities): 💻️os (composable OS shell, app runtime), 📓️print (LaTeX templates + visualization library), 🎤️presentation (render-independent deck model + React/reveal.js renderer, `🎯️targets/⚛️react`), ❓️quiz (render-independent quiz model with TS + Rust cores, schema, CQRS-ish deciders, plus `🎯️targets/⚛️react`), 🦑️repo (repo tooling), 🖥️server (authoritative server, CQRS buses, replication gateway). Shape: a **domain model** (schema + pure deciders + two cores) with a React renderer as "one target among possible others" (`❓️quiz/README.md`, `🎤️presentation/README.md`). `🧰️framework/🛒️products/💻️os` is a stray sibling folder containing only `🔨️modules`.

### 8.2 Verdict: **(a) — a new element under `UI/🧱️elements/`, with its pure parts split into `UI/🔨️modules/`, exported through the existing `@semio-tech/ui-react/chrome`**

Evidence:
1. **Every comparable thing is already there.** The two pieces pets must stand on (`🃏️OverviewCard`, `🥞️LayeredOverview`), the geometry/lifecycle policy for them (`UI/🔨️modules/🥞️layered-overview-geometry`, pure TS, JSON-schema + fixture + oracle tested), the decorative-overlay patterns (Introduction ghost cursor, chrome-reveal controller), and the layering/portal/reduced-motion/appearance machinery (`ShellScope.portalLayerRef`, `--z-*`, `[data-level]`, `.dark`) are all in `UI`. Charter fit: `UI/AGENTS.md:4` "business logic free ui components" — pets have no domain.
2. **No framework module other than `🖱️ui` hosts React** (0 `.tsx` files elsewhere in `🔨️modules`), so option (b) would create the first non-UI module with a `🎯️targets/⚛️react`, its own workspace line (`package.json:105-114` is an explicit list), vitest config, nx project, and — because it must read UI tokens, `ShellScope`, `cn`, the silhouette geometry — a dependency on `@semio-tech/ui-react` that `ui-react` itself could not re-export from `/chrome`.
3. **Option (c) is for domain models.** Products carry a render-independent *model* with its own lifecycle, schema, two language cores and (quiz/server) event-sourced deciders (`❓️quiz/README.md`, `🛍️products/🔣️.json`). Pets are ephemeral, local-only decoration with no persisted/shared domain state; a product would duplicate `package.json/project.json/📜️script.ts/vitest/Cargo` scaffolding for no domain.
4. **Import-surface economics.** `/chrome` is the slim entry documents like the quiz use "and ship none of the shell's scene, flow, pdf or panel machinery" (`🪟️chrome/🟦️.ts:1-6`). Putting pets there needs **zero** consumer changes; a new subpath or package needs alias edits in the Vite builder, vitest config and tsconfig of each consumer (3 places for the quiz alone, section 2.3).
5. **Multi-implementation is already modelled inside `UI`**: schema-first (`UI/🧬️schema/<name>/🔣️.json`) + shared fixtures (`UI/🧫️fixtures/<name>/🔣️.json`) + oracle registry (`UI/🔮️oracles/🔣️.json`) + sibling `🎯️targets/{🧊️wgpu,⌨️tui}` — a render-independent pet model (rig, pose, steering/behaviour state machine, walking-surface extraction from outline polygons) can live as `UI/🔨️modules/<emoji>pet-*/🟦️.ts` today and gain a Rust twin next to it later, with the React renderer as the only React-specific part (`UI/🧱️elements/<emoji>Pets/🟦️.tsx`).

Suggested concrete placement (names are placeholders):

| Piece | Location | Precedent |
|---|---|---|
| Pure model + maths (walk-surface extraction from `WindowSilhouetteGeometry.outline`, steering, blink/eye-look, pet-vs-pet rules, easing) | `UI/🔨️modules/<emoji>pet-*/🟦️.ts` + `🧪️tests/🔬️unit/🟦️.ts` | `🥞️layered-overview-geometry` |
| JSON schema + shared vectors | `UI/🧬️schema/<emoji>pets/🔣️.json`, `UI/🧫️fixtures/<emoji>pets/🔣️.json` | `🥞️layered-overview` |
| Third-party cross-check (test-only) | declare in `UI/🔮️oracles/🔣️.json` | `d3-ease`, `polygon-clipping` entries |
| React renderer (decorative `aria-hidden`, `pointer-events-none` layer; rAF loop writing transforms imperatively; reduced-motion prop) | `UI/🧱️elements/<emoji>Pets/🟦️.tsx` (+ `📖️stories`, `🧪️tests/🧩️component`) | `🥞️LayeredOverview`, Introduction demo overlay |
| Export | barrel region next to `REACT/🟦️.tsx:5278-5299` **and** `REACT/🪟️chrome/🟦️.ts` | OverviewCard/LayeredOverview |
| Test registration | append to `REACT/🧪️tests/🎚️config/🟦️.ts:24-62` | lines `:50,51,57` |
| Quiz wiring + the "show pets" preference + species/messages in en/de | `QUIZ-R/🔨️modules/…` (domain-specific extension), toggle beside `showCursors`/`animateIcons` (`🎛️preferences:44-62`) | `PresenceOverlay` mounted at `QUIZ-R/🟦️.tsx:474` |

When the verdict would flip: choose (b) only if the rig/behaviour core must ship Rust/WASM and be consumed by non-UI crates *without* depending on `ui` (then `🔨️modules/<pets>` with `📦️packages/{🟦️typescript,🦀️rust}`, renderer still in `🖱️ui`); choose (c) only if pets acquire shared/persisted domain state (events, CQRS) of their own.

### 8.3 Implementation cautions found while reading (all from source, none measured at runtime)

- Layer outside the clip: `LayeredOverview` root is `overflow-clip`/`overflow-hidden` and panes are `contain: layout paint` (`:851,400-401`); mount pets as a sibling above `z-[31]` (e.g. via `renderChrome`, or the shell portal layer / a `fixed` layer like the quiz presence layer `QUIZ-R/…/👥️presence/🟦️.tsx:1010` `pointer-events-none fixed inset-0 z-40`).
- Use `style={{ zIndex: "var(--z-menu)" }}`-style inline values rather than arbitrary Tailwind z utilities (`WindowChrome:232-241`).
- Cards move (2 px lift/200 ms) and can be scaled when they are the *pages* of the grid rest; compute walk surfaces from live rects each frame, in the stack's own pixels (reuse `windowSilhouetteScale` logic `WindowChrome:106-109`), and treat `data-veil`/`data-opened` states (opened → cards are not rendered at all, `LayeredOverview:868`) as "no surfaces".
- Pointer: the overlay is `pointer-events-none`, so eye-tracking must listen on `window` (mouse only, passive, like `LayeredOverview:758-769`); touch has no hover (`pointerType === "mouse"` gates reveal `:439-440`).
- Reduced motion: honour `prefers-reduced-motion` (CSS or the `"auto"|"always"|"never"` prop pattern); the Introduction demo shows the repo's strictest precedent (render nothing, `REACT/🟦️.tsx:3341`). Also respect `forced-colors` and `prefers-reduced-transparency` if pets use glass.
- Accessibility: decorative layers in this codebase are `aria-hidden` + `pointer-events-none` (presence layer, Intro demo); any interactive pet needs an app-supplied accessible name via `labels` props (no default language, section 5).
- No global motion tokens exist: if pets introduce durations/easings, either add a `motion` block to `STYLING/🔣️.json` (+ regenerate + schema + Rust/Py/.NET twins) or keep them as named constants in the pure module like `LAYERED_GLIDE_MS` (`geometry:130`).
- Tests: rAF needs the fake-timer recipe (`🥞️LayeredOverview/🧪️tests/🧩️component/🟦️.tsx:37-47`); `ResizeObserver` is a no-op mock in jsdom (so geometry from `useWindowSilhouetteGeometry` is `pending` in tests; the pure functions take plain metrics), jsdom `getBoundingClientRect` returns zeros.
- `check-ui-primitives` allows raw `<svg>` under `🧰️framework/` only; `check-chrome-i18n` does not scan `🧱️elements`.
- The `reactHostPort` indirection (`🔌️Ports/🟦️.tsx:28-65`) is used by older elements (`WindowChrome`, `Surface`); newer ones (`OverviewCard`, `LayeredOverview`) use `React.*` directly. Either is accepted; the newer files are the closer templates.

### 8.4 Not verified / open

- Whether Tailwind currently emits `z-[31]` (and other arbitrary utilities) on the quiz serve — the `@source "../../"` fix is documented (`🖌️ui/🎨️.css:5-13`) but I did not measure it; inline z-index avoids the question.
- I did not read the Rust `🎯️targets/🧊️wgpu` / `⌨️tui` sources beyond folder names, so any pet-adjacent twin work there is unmapped.
- I did not run any test, lint or typecheck.
