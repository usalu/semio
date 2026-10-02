# 📓️ Explore: the architecture quiz site (for the QUIZ-PETS ticket)

Read-only map of the quiz **site** and its topic tree, as of the working tree on 2026-10-02. Nothing was run (no build, no
test, no server); every statement is read from the files cited. Line numbers are of the current working tree (the tree has
many uncommitted edits by other agents; the site is mostly modified or untracked relative to `HEAD`).

Path abbreviations used below:

| Short | Full path |
|---|---|
| `SITE` | `🎓️teaching/🏛️architecture/❓️quiz` |
| `TOPICS` | `🎓️teaching/🏛️architecture/⚡️energy` |
| `QUIZ` | `🧰️framework/🛍️products/❓️quiz` (the domain-neutral quiz product) |
| `REACT` | `QUIZ/🎯️targets/⚛️react` (renderer `@semio-tech/quiz-react`) |
| `UI` | `🧰️framework/🔨️modules/🖱️ui` (design system) |
| `VITE-LIB` | `UI/🎨️styling/🏗️builder/🌐️vite/🟦️.ts` (shared Vite helpers) |

---

## 0. Ten facts that decide the pet design

1. **The site is a thin shell.** `SITE/🟦️.ts` is 28 lines: it loads CSS and calls `mountQuiz(root, { proctor, tenant, logo, legal })`. All
   UI lives in `REACT` (`@semio-tech/quiz-react`), which is **domain-neutral** (docstring `REACT/🟦️.tsx:3`). The natural seam for
   pets is a new optional field on `QuizOptions` (`REACT/🟦️.tsx:173-183`), exactly like `logo` and `legal`.
2. **The client never reads the quiz JSON files.** It receives a solution-free `CatalogView` from the proctor
   (`QUIZ/🧬️schema/🔣️.json:786-812`). The static JSON under `TOPICS` is only read by the proctor (Rust), the tests and the e2e
   learner. Pet definitions that need to reach the browser must be **imported into the bundle** or travel through the proctor.
3. **`connect-src` in the release CSP is exactly the proctor origin + its `wss://` twin** (`SITE/🏗️builder/🌐️vite/🟦️.ts:59`) and a test
   enforces equality (`SITE/🚀️deploy/🟦️.ts:304-305`). A runtime `fetch("/pets/x.json")` is **blocked in release**. Use static
   `import` (JSON natively, SVG via `?raw`/`?url`, both already used) instead.
4. **Quiz/catalog schemas are closed** (`additionalProperties: false` in JSON Schema, `#[serde(deny_unknown_fields)]` in Rust at
   `QUIZ/🧬️schema/🦀️.rs:39,68,78,86,109,…`). Adding a `pets` key to a quiz, task or catalog means changing three twins (JSON Schema,
   `🦀️.rs`, `🟦️.ts`) and the proctor projection; editing any quiz file also changes its revision and voids open runs
   (`🎓️teaching/README.md`, "Adding a quiz", step 7). Keeping pet data **out** of the quiz documents avoids all of that.
5. **Tasks already carry a themed emoji and a motion**: `"icon": { "emoji": "☀️", "motion": "spin" }` (e.g.
   `TOPICS/🧲️physics/❓️quiz/🔣️.json:185`). Motions are the closed enum `bounce|pulse|spin|sway|float|flip`
   (`QUIZ/🧬️schema/🔣️.json:53-56`, `QUIZ/🧬️schema/🟦️.ts:46`) played by CSS only when the learner neither prefers reduced motion nor
   switched "Animate task icons" off (`REACT/🎨️.css:39-72`, `data-icon-motion` at `REACT/🟦️.tsx:451`, preference
   `animateIcons` at `REACT/🔨️modules/🎛️preferences/🟦️.tsx:48,61`). Pets should honour the same two switches.
6. **There is a ready-made, pointer-transparent overlay precedent**: `PresenceOverlay` is
   `pointer-events-none fixed inset-0 z-40 overflow-hidden`, `aria-hidden`, re-places marks on rAF/scroll/resize/500 ms timer from
   `[data-presence-anchor]` boxes while skipping `[inert]` duplicates (`REACT/🔨️modules/👥️presence/🟦️.tsx:918-941,971-1010`). A pet layer
   can copy that pattern and reuse the same anchors (`PRESENCE_ANCHORS`, `:725-738`).
7. **Home is a pointer-panned panorama of 9 cards plus live, inert copies of every page behind the glass**
   (`UI/🧱️elements/🥞️LayeredOverview/🟦️.tsx:393-399,827-871`; geometry `UI/🔨️modules/🥞️layered-overview-geometry/🟦️.ts:122-155`
   `pointerOffset`/`followStep`). Card boxes **move** while the pointer moves, and every anchor exists twice (card + inert page). Pets that
   walk on cards must re-sample `getBoundingClientRect()` each frame and ignore `closest("[inert]")`.
8. **The e2e gate fails a spec on** any console error, page error, failed request, HTTP >= 400 response, socket error, or CSP violation
   (`SITE/🎭️e2e/🚶️learner/🟦️.ts:157-185,212-224`), and the phone spec fails on any horizontal overflow
   (`SITE/🧪️tests/📱️phone/🟦️.ts:9,24-37`). So: no 404 for a pet asset, no CSP-blocked blob/eval/style injection, overlay must be
   `fixed`/`overflow-hidden`, and it must never intercept pointer events (Playwright actionability).
9. **Release size budget**: `scriptGzipBytes 260_000`, `styleGzipBytes 60_000`, `totalBytes 4_000_000`
   (`SITE/🚀️deploy/🟦️.ts:86`). The current `SITE/📦️packages/🟦️typescript/dist` (built 02:53 today; may be stale) measures
   **JS 747,609 B raw / ~210,600 B gzip**, **CSS 253,408 B raw / ~37,600 B gzip**, **2,142,996 B total excl. `dist/pages`** (gzip measured with
   `gzip -9`; the gate uses Node's `gzipSync`). Headroom is roughly 49 KB gzip of JS, 22 KB gzip of CSS, 1.85 MB total.
10. **Every new directory/test folder needs registration in four places**: the repo taxonomy (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`),
    the nx `namedInputs.default` of `SITE/📦️packages/🟦️typescript/📋️project.json:4-26` (else cached `build`/`test` do not invalidate), the vitest
    `include` list (`SITE/🧪️tests/🎚️config/🟦️.ts:14`) or the Playwright `projects[].testMatch` (`SITE/🎭️e2e/🎚️config/🟦️.ts:39-43`). Tailwind utility
    generation for a new folder is a further risk (see §7.2): prefer plain CSS shipped with the pet module.

---

## 1. Directory tree of the site (`SITE`)

```
❓️quiz/
├─ README.md                      site runbook: files table (L9-21), launch rows (L25-35), e2e gate (L52-99), deploy (L101-338)
├─ 🌐️.html                        host document source; one <div id=root> + <script type=module src=./🟦️.ts>; title in both languages
├─ 🟦️.ts                          site entry: imports CSS, mounts the quiz app (see §2)
├─ 🎨️.css                         2 lines of CSS: imports the semio design-system chain + @source for the React target
├─ 🔣️.json                        catalog "architecture": introduction, 4 quiz paths, 7 badges (see §3)
├─ node_modules/                  empty workspace shim dir
├─ 🏗️builder/
│  └─ 🌐️vite/🟦️.ts                Vite config: host HTML, release CSP meta, aliases, dev proxy (see §4)
├─ 📦️packages/🟦️typescript/       nx project @teaching/architecture-quiz
│  ├─ package.json                scripts -> `bun nx run …`; deps @semio-tech/quiz(+react); devDeps vite, vitest, ajv, yaml
│  ├─ 📋️project.json              nx targets + namedInputs
│  ├─ 📜️script.ts                 the only script file: command router (dev, dev-site, build, test, test-e2e, check, publish, docker-*, deploy-check)
│  └─ dist/                       build output (git-ignored): assets/🌐️-<hash>.js|css, index.html, 404.html, fonts, cursors, pages/quizzes/ (staged CDN artifact)
├─ 🚀️deploy/
│  ├─ 🔣️.json                     single source of hosts, image, port, site.legal (imprint/privacy URLs, currently {})
│  ├─ 🧬️schema/🔣️.json            JSON Schema for the file above
│  ├─ 🟦️.ts                       operator verbs + artifact/CSP/budget verification (publish, docker-*, deploy-check)
│  ├─ Dockerfile, Dockerfile.dockerignore, compose.yaml, Caddyfile   proctor image + Caddy (API host only, NOT the site)
├─ 🧱️stack/🟦️.ts                  local stack: proctor + site dev server started/awaited/stopped as one
├─ 🎭️e2e/
│  ├─ 🟦️.ts                       the Playwright gate: boots throw-away stacks (dev and rehearsal), runs Chromium specs
│  ├─ 🎚️config/🟦️.ts              Playwright config: projects boot -> desktop|phone -> presence -> shortage
│  └─ 🚶️learner/🟦️.ts             the shared "learner" fixture + catalog sources + locators (fails a test on console/page/request/CSP problems)
└─ 🧪️tests/                       one folder per spec, each `🟦️.ts`
   ├─ 🎚️config/🟦️.ts              vitest config (node) — only 4 of the folders below
   ├─ 🧪️catalog  🧪️deploy  🧱️local-stack  📰️host-document      vitest (node) specs
   └─ 🚀️site-boot  🪪️first-visit  🥞️layered-home  🎯️quiz-runs  🏆️live-leaderboard  🗣️both-languages
      📱️phone  👥️shared-presence  🔌️connection-shortage        Playwright (browser) specs
```

There is **no** `public/` directory (`publicDir: false`, `SITE/🏗️builder/🌐️vite/🟦️.ts:113`), no site `tsconfig`, no site `typecheck`
target. Only `@semio-tech/quiz-react:typecheck` (`REACT/📦️packages/🟦️typescript/📋️project.json:51-57`) type-checks client code.

The topic tree (`TOPICS`), the documented layout (`🎓️teaching/README.md`, "Layout" and "Topic tree"):

```
🎓️teaching/
├─ README.md
├─ 🛂️proctor/                    Rust API server (separate package @teaching/proctor)
└─ 🏛️architecture/
   ├─ README.md                  site overview, quiz table, badge table, sources
   ├─ ❓️quiz/                     the site (above)
   └─ ⚡️energy/                   domain
      ├─ 🧲️physics/❓️quiz/🔣️.json    28,019 B, 444 lines, quiz "physics"
      ├─ 🔥️heating/❓️quiz/🔣️.json    22,860 B, 307 lines, quiz "heating"
      ├─ ❄️cooling/❓️quiz/🔣️.json    18,627 B, 271 lines, quiz "cooling"
      └─ 📊️demand/❓️quiz/🔣️.json     16,295 B, 241 lines, quiz "demand"
```

A topic leaf holds only `❓️quiz/🔣️.json` today. `🎓️teaching/README.md` mentions a `🎬️clip` leaf "not built yet"; it is **not** registered in
the taxonomy (see §8).

---

## 2. Site entry and boot path

`SITE/🌐️.html:1-12` — no inline code, no language:

```html
<title>Architecture and Technology Quizzes · Quizze zu Architektur und Technologie</title>
...
<div id="root"></div>
<script type="module" src="./🟦️.ts"></script>
```

`SITE/🟦️.ts` (full code, 28 lines):

```ts
9   import "./🎨️.css";
10  import { mountQuiz } from "@semio-tech/quiz-react";
11  import logo from "../../../🧰️framework/🔨️modules/🖼️assets/🪧️logos/🛡️emblem/🌘️dark-round/🖋️vector.svg?raw";
12  import { site } from "./🚀️deploy/🔣️.json";
15  export const ARCHITECTURE_QUIZ_TENANT = "architecture";
18  export function bakedProctorOrigin(): string { try { return (import.meta.env.VITE_PROCTOR_URL as string | undefined) ?? ""; } catch { return ""; } }
26  const root = document.getElementById("root");
27  if (root) mountQuiz(root, { proctor: bakedProctorOrigin(), tenant: ARCHITECTURE_QUIZ_TENANT, logo, legal: site.legal });
```

What this gives a pet author:

- **Passes today**: `proctor` (origin, `""` = same origin in dev/tests), `tenant` (`"architecture"` = catalog id), `logo` (inline SVG markup via
  `?raw`, rendered by `ShellBrandLogo` with `dangerouslySetInnerHTML`, `UI/🧱️elements/🔝️Navbar/🟦️.tsx:252-256`),
  `legal` (`site.legal` from `🚀️deploy/🔣️.json`, currently `{}` so no footer links).
- **Not passed (all optional seams exist)**: `languages`, `transport`, `presence`, `storage`, `timing` (`REACT/🟦️.tsx:173-183`). Language is
  never defaulted: stored locale, else the browser list, else the bilingual chooser (`REACT/🟦️.tsx:492-530`).
- **Branding** = the emblem SVG (navbar), the catalog title (navbar text, document title), the palette in the CSS chain (§7).
- `mountQuiz` (`REACT/🟦️.tsx:534-545`) applies the stored theme + locale before first paint, then renders `<StrictMode><QuizApp {...options}/></StrictMode>`.
- `QuizApp` (`:492-530`) sets up store/proctor/presence/session (`useSetup`, `:480-488`), computes `locale`, applies `useElementsSurfaceChrome` (theme),
  `useRootTextScale` (`--quiz-text-scale`), document title/lang; with **no locale it returns only the language chooser** (`:519-529`) — no `Client`,
  so a pet overlay mounted in `Client` would not exist on that screen.
- `Client` (`:397-478`) renders: skip link, `Navbar` (brand, connection, presence, language switch), `<main id="quiz-main">` with the `Screen`,
  `LegalFooter`, and finally `<PresenceOverlay …/>` (`:474`) inside `<div class="quiz-app …" data-icon-motion=…>` (`:451`). A pet layer belongs right
  beside `PresenceOverlay` (still inside `PresenceProvider`, `:450`).
- Steps/screens: `introduction | identity | home | run | results` (`HOME_PAGES` for learner/introduction/leaderboard/badges/preferences/quiz pages);
  `Screen` is `REACT/🟦️.tsx:296-348`; non-home screens scroll in `.min-h-0 flex-1 overflow-auto p-double` (`Page`, `:285-294`).
- Imports from the framework: `@semio-tech/quiz-react` (aliased to `REACT/📦️packages/🟦️typescript/🟦️.tsx` which is glue-only `export * from "../../🟦️.tsx"`),
  and transitively `@semio-tech/quiz` (TS core), `@semio-tech/ui-react[/chrome|/i18n]`, `@semio-tech/framework[-server]` — all via aliases in the
  Vite config (§4), not via npm packaging.
- Quiz-react exports a large reexport surface (`REACT/🟦️.tsx:49-167`), including `PresenceOverlay`, `PRESENCE_ANCHORS`, `placePeers`,
  `useDocumentVisible`, `TaskGlyph`, `TASK_KIND_ICONS`, `QuizCard`, `Glyph`, i18n (`quizText`, `QUIZ_BUNDLE_EN/DE`, `applyLocale`).

---

## 3. Catalog and per-topic quiz JSON

### 3.1 Schema that validates them

`QUIZ/🧬️schema/🔣️.json` (draft-07, `$id https://json.schemas.assets.semio-tech.com/framework/product/quiz/schema.json`, 1130 lines). Twins:
`QUIZ/🧬️schema/🦀️.rs` and `QUIZ/🧬️schema/🟦️.ts` (types + `catalogIssues`, `quizIssues`, `MOTIONS`). Definitions carry
`"x-semio-formats": ["🔣️jsonschema","🦀️rust","🟦️typescript"]` (multi-implementation). Both JSON files point at it with a relative `$schema`:

- catalog: `"$schema": "../../../🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json#/$defs/Catalog"` (`SITE/🔣️.json:2`)
- quiz: `"$schema": "../../../../../🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json#/$defs/Quiz"` (`TOPICS/*/❓️quiz/🔣️.json:2`)

Validation in the repo: Vitest `SITE/🧪️tests/🧪️catalog/🟦️.ts` (TS core `catalogIssues`/`quizIssues` + ajv draft-07 against the schema, plus "each
task has an icon with a distinct emoji and a valid motion", `:44-49`); `bun nx run @teaching/architecture-quiz:check` runs the Rust core
(`proctor check`, `SITE/📦️packages/🟦️typescript/📜️script.ts:43-47`).

### 3.2 Catalog `SITE/🔣️.json` (89 lines)

Top-level keys (schema `Catalog`, `QUIZ/🧬️schema/🔣️.json:292-307`, all required except `$schema`): `$schema`, `schema: "semio.quiz.catalog/v1"` (`:3`),
`id: "architecture"` (`:4`), `title {en,de}` (`:5`), `introduction {title, paragraphs[]{en,de}}` (`:6-30`), `quizzes[]` (`:31-36`), `badges[]` (`:37-87`).

How it references topic quizzes — relative paths **from the catalog file**:

```json
31  "quizzes": [
32    "../⚡️energy/🧲️physics/❓️quiz/🔣️.json",
33    "../⚡️energy/🔥️heating/❓️quiz/🔣️.json",
34    "../⚡️energy/❄️cooling/❓️quiz/🔣️.json",
35    "../⚡️energy/📊️demand/❓️quiz/🔣️.json"
36  ],
```

Resolution: the proctor (Docker image stages the tree to `/srv/quiz/content/🏛️architecture/…`, `SITE/🚀️deploy/Dockerfile:76`), the catalog test
(`resolve(dirname(catalogPath), path)`, `SITE/🧪️tests/🧪️catalog/🟦️.ts:18`) and the e2e learner (`resolve(site, path)`,
`SITE/🎭️e2e/🚶️learner/🟦️.ts:100`). Badges: `{id, emoji, label{en,de}, description{en,de}, rule}` with `rule.kind` one of `perfect-quiz{quiz}`,
`perfect-tasks{taskKind?,quiz?}`, `completed-quizzes` (`QUIZ/🧬️schema/🔣️.json:242-281`). Badge ids/emojis today: `physics-expert 🧲`, `heating-expert 🔥`,
`cooling-expert ❄️`, `demand-expert 📊`, `numerical-brain 🧮`, `pattern-seer 🔍`, `completionist 🏁`.

Every learner-visible text is `Text = {en, de}` (both required, no default language; `QUIZ/🧬️schema/🔣️.json:35-45`).

### 3.3 Topic quiz `TOPICS/<topic>/❓️quiz/🔣️.json`

Top-level (schema `Quiz`, `QUIZ/🧬️schema/🔣️.json:226-241`; required `schema,id,emoji,title,description,tasks`):

```json
{ "$schema": "…#/$defs/Quiz",
  "schema": "semio.quiz/v1",
  "id": "physics",                 // Slug, also the folder slug; matches the perfect-quiz badge
  "emoji": "🧲",                    // 1-16 code points; README says: the topic folder's emoji
  "title": {"en":…,"de":…}, "description": {"en":…,"de":…},
  "tasks": [ { "kind": "classification|sorting|matching", "id": …, "title": …, "icon": {"emoji":"⚡","motion":"pulse"},
               "prompt": …, "items": [ … ], "draw": 12, …kind-specific… } ] }
```

Task shapes (`QUIZ/🧬️schema/🔣️.json:125-218`): `classification` = `categories[]` (id,label,description?,profile?) + optional `axes[]` (id,label,unit,min,max) + `items[]{id,label,category,explanation?}`;
`sorting` = `quantity{label,unit,scale,prefixed}` + `items[]{id,label,value,explanation?}`; `matching` = `dimensions[]{id,quantity}` + `items[]{id,label,values{dim:number},explanation?}`.
`draw` = items drawn per run. Task `icon` is optional in the schema but the site's catalog test requires it with distinct emojis per quiz.

The 9 tasks today, with the themed icon each already has (useful anchor points for pets):

| Quiz (folder emoji) | Task id | kind | icon.emoji / motion | draw |
|---|---|---|---|---|
| `physics` 🧲 | `power-or-energy` | classification | ⚡ pulse | 12 |
| | `powers` | sorting (W) | ☀️ spin | 10 |
| | `energies` | sorting (Wh) | 🔋 bounce | 9 |
| `heating` 🔥 | `u-values` | matching | 🧱 flip | 10 |
| | `heating-load-and-demand` | matching (2 dims) | 🔥 float | 8 |
| `cooling` ❄️ | `air-change-rates` | matching | 🌬️ sway | 10 |
| | `cooling-load-and-demand` | matching (2 dims) | ❄️ spin | 7 |
| `demand` 📊 | `standard-profiles` | classification + 4 `axes` (spider) | 🕸️ sway | (6 items) |
| | `final-energy` | matching | 🔌 bounce | 7 |

(`TOPICS/🧲️physics/❓️quiz/🔣️.json:13-16,182-185,322-325`, `❄️cooling:13-16,167-170`, `📊️demand:13-16,141-144`, `🔥️heating:13-16,185-188`.)

Item ids already name pet-relevant objects: physics `sun, sunlight-square-metre, pv-module-peak, pv-annual-yield, ev-battery, phone-charge, wind-turbine,
kettle, tea-light, wallbox …`; heating `single-glazing, triple-glazing, window-geg, window-passive-house, wall-geg, wall-passive-house, roof-*, roller-shutter-box,
box-type-window …`; cooling `data-centre, office-*, passive-house-dwelling, attic-flat …`; demand `old-house-heat-pump, kfw-55-gas-solar, plus-energy-house,
passive-house-direct-electric …`.

Where the data reaches the browser: the proctor projects the catalog into `CatalogView`/`CatalogQuizView {id, emoji, title, description, tasks[{id,kind,title,icon?}]}`
(`QUIZ/🧬️schema/🔣️.json:760-812`); only `emoji` and `tasks[].icon` of the themed data are visible to the client. `QUIZ/🧬️schema/🔣️.json` and the Rust
twin are closed — a new field means schema + `🦀️.rs` + `🟦️.ts` + projection changes (see fact 4).

---

## 4. Build: Vite config, package, nx targets, script

### 4.1 `SITE/🏗️builder/🌐️vite/🟦️.ts` (143 lines)

- Imports the shared helpers from `VITE-LIB` (`:8-17`): `playgroundStaticSiteBuildOptions`, `semioEmojiIndexHtmlVitePlugin`, `semioHostHtmlVitePlugin`,
  `semioHostTitleText`, `semioReferencedAssetsVitePlugin`, `semioServeCloseVitePlugin`, `semioViteProductionBuild`. Imports the catalog and the deploy
  JSON **with `with { type: "json" }`** (`:18-19`).
- `quizHostDocument` (`:32-41`): bilingual title + loading text + noscript, `cnameHost` = site host.
- `quizContentSecurityPolicy` (`:50-65`) and `quizReleaseDocumentVitePlugin` (`:71-95`, `apply: "build"`, `enforce: "post"`): injects description metas, theme-color
  metas (`#f7f3e3` light / `#001117` dark), `<link rel=manifest>`, and the CSP `<meta http-equiv>` right after the charset; emits `robots.txt`,
  `manifest.webmanifest`.
- `defineConfig` (`:110-143`):
  - `root: siteRoot`, `base: "/"`, **`publicDir: false`** (`:111-113`), `cacheDir` from `TEACHING_ARCHITECTURE_QUIZ_CACHE`.
  - plugins (`:115-123`): `semioServeCloseVitePlugin`, host HTML, `semioEmojiIndexHtmlVitePlugin(siteRoot)` (serves/emits `🌐️.html` as `index.html` + `404.html`),
    `semioReferencedAssetsVitePlugin(repoRoot)`, `@tailwindcss/vite`, `@vitejs/plugin-react`, the release-document plugin.
  - `build` (`:124`): `playgroundStaticSiteBuildOptions({ ...semioViteProductionBuild(), outDir: <package>/dist })` -> target es2022, **`esbuild.drop: ["console","debugger"]`**,
    no sourcemaps, minify, `emptyOutDir` (`VITE-LIB:56-80`). Default Vite `assetsInlineLimit` (4 KB) applies — nothing overrides it.
  - `define` (`:125`): `import.meta.vitest` undefined; at build `import.meta.env.VITE_PROCTOR_URL` = JSON-stringified proctor origin.
  - `server` (`:126-130`): `fs.allow: [repoRoot]`; proxy `/instance, /commands, /queries, /actors, /scopes` -> `http://127.0.0.1:${PROCTOR_PORT ?? 8791}` with `ws: true`;
    `TEACHING_ARCHITECTURE_QUIZ_WATCH=off` disables HMR/watch (used by e2e).
  - `resolve.alias` (`:131-142`) — **the only way site code sees framework code**:
    `@semio-tech/quiz-react` -> `REACT/📦️packages/🟦️typescript/🟦️.tsx`; `@semio-tech/quiz` -> `QUIZ/📦️packages/🟦️typescript/🟦️.ts`; `@semio-tech/ui-react`, `/i18n`, `/chrome`
    -> `UI/🎯️targets/⚛️react/…`; `@semio-tech/framework-server`, `@semio-tech/framework`. `dedupe: ["react","react-dom"]`.
    A new framework package imported by the site (or by `REACT`) that is not via a relative path needs a new alias line here, in the vitest config
    (`SITE/🧪️tests/🎚️config/🟦️.ts:13`) and in `REACT/🧪️tests/🎚️config/🟦️.ts`.

**Importable assets (answer to "would JSON pet definitions or SVG be importable?")**: yes.
- JSON: native Vite JSON import incl. named exports (already `import { site } from "./🚀️deploy/🔣️.json"`, `SITE/🟦️.ts:12`).
- SVG as string: `import logo from "…/🖋️vector.svg?raw"` (`SITE/🟦️.ts:11`); also `?url`/`?inline` work (bundled as hashed `assets/…` or a `data:` URI under 4 KB).
- Static files referenced by absolute URL `/🖼️assets/<path>` are served in dev and **copied to `dist/🖼️assets/` only when the built CSS/JS/HTML text contains the
  URL** (`semioReferencedAssetsVitePlugin`, `VITE-LIB:958-996`, regex `^/🖼️assets/…` over `.css|.html|.js|.mjs`). The assets module is
  `🧰️framework/🔨️modules/🖼️assets` (fonts, cursors, `🔣️icons`, `🖼️images`, `🪧️logos`, `📃️list/🦊️animals.json`, …).
- Nothing for `.json`/`.svg` is excluded; no svgr plugin (no `?react`).
- Not available: runtime `fetch()`/XHR of site files in release (CSP `connect-src`, §7).

### 4.2 `SITE/📦️packages/🟦️typescript/package.json`

`name @teaching/architecture-quiz`, `private`, `type: module`, `bundleKind: application`. Scripts (`:8-14`) all `bun nx run @teaching/architecture-quiz:<target>`: `dev, dev:site, build, test, test:e2e`.
Deps (`:15-18`): `@semio-tech/quiz`, `@semio-tech/quiz-react` (`workspace:*`). DevDeps (`:19-27`): `@tailwindcss/vite ^4.1.18`, `@vitejs/plugin-react ^5.1.2`, `ajv 8.20.0`, `typescript ^5.9.3`, `vite ^7.3.1`,
`vitest ^4.0.17`, `yaml 2.9.0`. `semio.app` (`:35-49`): `kind architecture-quiz`, `packageRoot SITE`, dev port **6061** (env `TEACHING_ARCHITECTURE_QUIZ_PORT`).
Root `package.json:94` lists the package as a workspace; root aliases at `:204-215` (`dev:teaching:architecture-quiz`, `…:site`, `build`, `test`, `test:e2e`, `check`, `docker-*`, `publish`, `deploy`).

### 4.3 `SITE/📦️packages/🟦️typescript/📋️project.json` (nx)

- `namedInputs.default` (`:4-26`): `{projectRoot}/**/*` plus **explicit** workspace paths — site `🟦️.ts|🌐️.html|🎨️.css`, the palette CSS, builder, `🚀️deploy/**`, `🧱️stack/**`, `🎭️e2e/**`, `🛂️proctor/🏗️bootstrap/**`,
  `🧪️tests/**`, `🎓️teaching/🏛️architecture/**/❓️quiz/🔣️.json`, quiz schema, quiz modules `*.ts`, `QUIZ/📦️packages/🟦️typescript/**`, `REACT/**`, server package, `UI/**`, framework package.
  **A new site folder (e.g. `🐾️pets/`) or non-JSON files in a topic folder are NOT inputs** until added here.
- Targets: `dev` (`:28-46`, continuous, deps `@semio-tech/assets:build`, `@semio-tech/ui-styling-tokens:generate`), `dev-site` (`:47-65`), `build` (`:66-77`, cached, input `PROCTOR_URL`, output `{projectRoot}/dist`),
  `publish` (`:78-87`), `test` (`:88-97`, cached), `test-e2e` (`:98-108`), `check` (`:109-116`), `docker-image-build|check|publish`, `docker-stack-check`, `deploy-check` (`:117-163`).
  All are `nx:run-commands` with `cwd` = the package dir and `command: "bun ./📜️script.ts <cmd>"` (AGENTS.md pattern).

### 4.4 `SITE/📦️packages/🟦️typescript/📜️script.ts` (81 lines)

Commands registered (`:79`): `dev` (`runDevStack`, proctor + site), `dev-site` (`runViteBunxDev`, fixed port 6061), `build` (`runViteBuild`), `test [quick|long|exhaustive]` (`runVitest` with
`../../🧪️tests/🎚️config/🟦️.ts`, `:29-34`), `test-e2e [dev] [rehearsal] [--serial] [--keep] [--proctor <exe>] [Playwright args]` (`runQuizEndToEnd`), `check` (`checkQuizCatalog`),
`publish` (build + verify + stage `dist/pages/quizzes`), `docker-image-build [--tag] [--jobs]`, `docker-image-check [--tag] [--port] [--keep]`, `docker-image-publish [--tag]`,
`docker-stack-check [--tag] [--http-port] [--https-port] [--keep]`, `deploy-check [--tag] [--jobs]`. Subcommands exist only for `test` and `test-e2e` (levels/topologies/flags).

---

## 5. Tests, e2e, stack

### 5.1 `SITE/🧪️tests/*` (vitest, node) — config `SITE/🧪️tests/🎚️config/🟦️.ts` (15 lines), `environment: "node"`, include list at `:14`:

| Folder | Covers |
|---|---|
| `🧪️catalog` | catalog + each quiz: TS core issues, ajv draft-07 contract, distinct task icon emojis + valid motions (`:25-55`) |
| `🧪️deploy` | `🚀️deploy/🔣️.json` vs schema (ajv), drift detection, Dockerfile/compose (parsed with `yaml`)/Caddyfile/workflow, **sealed release document (CSP) and CDN artifact checks incl. size budget** (`:187-226`) |
| `🧱️local-stack` | waiting for servers, gate command line, owned command tree-kill, static origin of the release rehearsal vs Vite preview (third-party oracle) |
| `📰️host-document` | pre-app document assumes no language, bilingual title, noscript text, **no additional inline script/style beyond the shared host template**, legal links only if set |

Vitest alias only maps `@semio-tech/quiz` (`:13`). Run: `bun nx run @teaching/architecture-quiz:test` (CI job `site` runs it before `publish`,
`SITE/🧪️tests/🧪️deploy/🟦️.ts:181`).

### 5.2 `SITE/🎭️e2e/*` + browser specs under `SITE/🧪️tests/*` (Playwright, Chromium)

- Gate `SITE/🎭️e2e/🟦️.ts` (299 lines): boots throw-away stacks under `.🧬semio/🎓️teaching/architecture-quiz-e2e/<run>/`: topology `dev` (Vite dev server **6161**, dev proctor **8891**, one origin via the
  proxy) and `rehearsal` (release build with the proctor origin baked in, static server **6162**, production-mode proctor **8892**, cross-origin; CSP active). Runs both by default, side by side
  (2 workers each; 4 when alone). Invoked by `bun nx run @teaching/architecture-quiz:test-e2e [-- dev|rehearsal --grep … --headed --serial --keep --proctor <exe>]`. Takes ~7 min for both.
- Config `SITE/🎭️e2e/🎚️config/🟦️.ts`: `testDir = SITE/🧪️tests`, `workers 4`, `timeout 300 s`, desktop 1440x900, phone 375x812 (`:23-26`); projects (`:39-43`):
  `boot` (`🚀️site-boot`) -> `desktop` (`🪪️first-visit, 🥞️layered-home, 🎯️quiz-runs, 🏆️live-leaderboard, 🗣️both-languages`, fully parallel) | `phone` (`📱️phone`) -> `presence` (`👥️shared-presence`, runs alone)
  -> `shortage` (`🔌️connection-shortage`, stops/starts the proctor). Requires `PLAYWRIGHT_BASE_URL` set by the gate.
- Fixture `SITE/🎭️e2e/🚶️learner/🟦️.ts`: reads the catalog + quizzes from disk (`:97-100`) and computes answers by item id; `Device` collects problems (`:157-205`); the `device` fixture fails the test on any problem
  (`:212-224`); locators by hooks `[data-layered-card]`, `[data-layered-pane]`, `[data-overview-card-action]`, `#quiz-main [data-card="…"]:not([inert] *)` (`:231,236,251`), `[data-layered-overview]`.
- Browser specs: `🚀️site-boot` (cold-start, clean console), `🪪️first-visit`, `🥞️layered-home` (nine cards, live inert pages behind glass, hover reveal, hash route, Escape), `🎯️quiz-runs`
  (every quiz to 100% + badges; one mistake per task), `🏆️live-leaderboard`, `🗣️both-languages` (every screen in EN and DE, no unresolved labels), `📱️phone`
  (no horizontal overflow), `👥️shared-presence` (cursors, thinking, crowd), `🔌️connection-shortage`.

### 5.3 `SITE/🧱️stack/🟦️.ts` (242 lines)

`dev`: reuses a proctor already answering `GET /instance` on `PROCTOR_PORT`, else builds + launches one, waits ready, starts `dev-site`; one Ctrl+C stops both. Exports `awaitReady`, `httpAnswers`, `launchOwned`,
`serveStaticSite` (CDN-like static origin used by the rehearsal), `SITE_FIRST_ANSWER_MS`. Cross-platform argv-only spawning.

### 5.4 Framework-side tests that a pet renderer in `REACT` would join

`REACT/🧪️tests/🎚️config/🟦️.ts` (jsdom vitest, `include` list of `QUIZ/🧪️tests/<name>/🟦️.tsx` files, e.g. `🖼️task-icons`, `🌗️contrast-states`, `📢️live-regions`, `🗣️translation-completeness`) run by
`bun nx run @semio-tech/quiz-react:test` (`REACT/📦️packages/🟦️typescript/📜️script.ts:7-12`). Language-agnostic twin tests in the product follow the quartet pattern `QUIZ/🧪️tests/<name>/{🟦️.ts|🥒️.feature|🐍️.py|🦀️.rs}`
(e.g. `🕸️profile-similarity`) — the AGENTS.md "same output with a third-party library" requirement.

---

## 6. Launch files

### 6.1 `.claude/launch.json` (583 lines; flat, `runtimeExecutable: bun`, `port`) — quiz-related tail, `:541-581`:

| name | runtimeArgs | port / env |
|---|---|---|
| `teaching-proctor` (`:542`) | `nx run @teaching/proctor:dev` | 8791, `PROCTOR_PORT` |
| `architecture-quiz-site` (`:555`) | `nx run @teaching/architecture-quiz:dev-site` | 6061, `TEACHING_ARCHITECTURE_QUIZ_PORT`, `PROCTOR_PORT` |
| `architektur-und-technologie-quizze` (`:569`) | `nx run @teaching/architecture-quiz:dev` (proctor + site) | 6061 |

### 6.2 `.vscode/launch.json` (22,140 lines) and seed `.vscode/🧩️launch.seed.jsonc` (5,369 lines, same rows)

All rows are `type node-terminal`, `cwd ${workspaceFolder}`, `command bun nx run <project>:<target>`, `presentation {group, order}`. Quiz block, group **`3_dev`** (`launch.json:1964-2152`):

| order | name | command |
|---|---|---|
| 213.6 | `🛠️dev🏛️architektur-und-technologie❓️quizze` | `…architecture-quiz:dev`; env ports; `serverReadyAction` opens `:6061` |
| 213.605 | `🛠️dev🎓️teaching🏛️architecture❓️quiz🌐️site` | `…:dev-site` |
| 213.61–213.616 | `🛠️dev🎓️teaching🛂️proctor`, `🔁️rebuild…`, `🩺️health…`, `💾️backup…`, `♻️restore…`, `🧨️erase…🔍️dry-run`, `🧨️erase…` | `@teaching/proctor:*` (inputs `proctorBackupFile`, `proctorEraseHandle` at `:20178-20185`) |
| 213.63 / .64 / .65 / .66 | `🧪️test❓️quiz🟦️`, `🧪️test❓️quiz⚛️react`, `🧪️test❓️quiz🦀️`, `🧪️test🎓️teaching🛂️proctor🦀️` | `@semio-tech/quiz[-react|-rs]:test`, `@teaching/proctor:test` |
| 213.67 | `🧪️test🎓️teaching🏛️architecture❓️quiz` | `…architecture-quiz:test` |
| 213.68 | `🛠️dev❓️quiz⚛️react🪁️typecheck` | `@semio-tech/quiz-react:typecheck` |

Groups **`4_build`** (`:4563-4617`): `11.1 📦️build…❓️quiz`, `11.15 🚚️publish…❓️quiz`, `11.2 📦️build🎓️teaching🛂️proctor`, `11.3 📦️build…🐳️docker-image`, `11.35 🚚️publish…🐳️docker-image`.
**`4_gate`** (`:4618-4690`): `11.1 ✅️check…❓️quiz📚️catalog`, `11.15 ✅️check🎓️teaching🛂️proctor📚️catalog`, `11.16 ⚖️gate…🛂️proctor🏋️capacity`, `11.17 ⚖️gate…❓️quiz🎭️e2e`, `11.2 ⚖️gate…🐳️docker-image`, `11.25 ⚖️gate…🐳️docker-stack`, `11.3 ⚖️gate…🚀️deploy`.
Naming grammar: `<verb-emoji><verb><scope-emoji><scope>…<variant-emoji><variant>` with verbs `🛠️dev | 🧪️test | ✅️check | ⚖️gate | 📦️build | 🚚️publish`. Project names also appear in pick-list `inputs` (e.g. `"@teaching/architecture-quiz"` at `:20585,20801,20864,20994,21437,21458`).
The seed file mirrors these rows (e.g. `:1074-1260`, `:2640-2770`); both files must be edited together when a new command is registered. No quiz-site command is new if pets add no script.

---

## 7. Theming, branding, CSS, CSP — constraints for an animated SVG/canvas overlay

### 7.1 CSP actually shipped (built `SITE/📦️packages/🟦️typescript/dist/index.html`, produced from `SITE/🏗️builder/🌐️vite/🟦️.ts:50-65`)

```
default-src 'self';
script-src 'self' 'sha256-…'(x4 boot scripts);
style-src 'self' 'sha256-…'(x1 boot style);
style-src-attr 'unsafe-inline';
img-src 'self' data:;
font-src 'self';
connect-src https://proctor.quizzes.architektur-und-technologie.de wss://proctor.quizzes.architektur-und-technologie.de;
manifest-src 'self'; object-src 'none'; base-uri 'none'; form-action 'self'
```

Delivered as a `<meta http-equiv>` (GitHub Pages cannot send headers); `frame-ancestors` only via `_headers` (`SITE/🚀️deploy/🟦️.ts:91-101`). The Caddyfile governs only the **proctor API host** (`default-src 'none'`, `SITE/🚀️deploy/Caddyfile:36-45`), not the site. Deploy tests assert: `default-src|script-src|style-src` sources are only `'self'`, `'none'` or sha256 hashes (`:295`); `connect-src` exactly the proctor + ws twin (`:304-305`); `object-src`/`base-uri` `'none'`; `form-action` set (`:306-307`).

Consequences for pets:

| Technique | Verdict under this CSP | Why |
|---|---|---|
| Inline `<svg>` elements rendered by React/DOM (paths, groups, transforms, `<animate>` SMIL, presentation attributes, `style=""` attributes, `element.style.setProperty`, CSS vars) | allowed | `style-src-attr 'unsafe-inline'`; DOM/CSSOM manipulation is not blocked |
| CSS `@keyframes`/classes from the bundled stylesheet (`assets/*.css`), Web Animations API (`el.animate`) | allowed | same-origin stylesheet is `'self'` |
| `<style>` element inserted at runtime (CSS-in-JS runtimes, `<style>` inside SVG markup injected via `innerHTML`/`dangerouslySetInnerHTML`) | **blocked** | `style-src` has no `'unsafe-inline'`; only the boot style hash. Use constructed stylesheets (`adoptedStyleSheets`) or put rules in the bundled CSS |
| `<img src="x.svg">` / `url()` backgrounds to bundled files or `data:` URIs | allowed | `img-src 'self' data:`; but an SVG-as-image has **no DOM access** (cannot follow the pointer or take page CSS vars) |
| `blob:` URLs for images (`URL.createObjectURL` -> `<img>`), `blob:` Workers | **blocked** | `img-src` lacks `blob:`; `worker-src` falls back to `script-src` (`'self'` + hashes) |
| Module Web Worker from a bundled same-origin URL (`new Worker(new URL("./w.ts", import.meta.url), {type:"module"})`) | allowed (untested here) | falls under `script-src 'self'` |
| `<canvas>` 2D / OffscreenCanvas drawing | allowed | not governed by CSP itself; `drawImage` sources must be `'self'`/`data:` |
| `eval`, `new Function`, expression engines | **blocked** | no `'unsafe-eval'` |
| `fetch()`/XHR of site JSON/SVG at runtime | **blocked in release** | `connect-src` is only the proctor; works in dev (same origin) so it would pass `dev` e2e and fail `rehearsal` |
| `<object>`/`<embed>`/`<iframe srcdoc>` | blocked | `object-src 'none'`; `default-src 'self'` for frames |
| `<use href="sprite.svg#id">` to an external file | uncertain (unverified) | avoid; inline `<symbol>` in the document instead |
| Web fonts as `data:` | blocked | `font-src 'self'` — fonts must be files under `/🖼️assets/…` or `assets/` |

Release-artifact checks that also constrain code (`SITE/🚀️deploy/🟦️.ts:319-346`): content-hashed file names under `assets/`; no `sourceMappingURL=`, `/@vite/client`, `react-refresh`, **`[DEBUG]`** in any built html/js/mjs/css (`:342`); no `localhost`, `127.0.0.1`, `[::1]` outside the baked proctor origin (`:341`); budget (`:86,331-344`). The host-document test additionally forbids new inline scripts/styles in the document (`SITE/🧪️tests/📰️host-document/🟦️.ts:46`). `esbuild.drop: ["console","debugger"]` strips console calls in release (`VITE-LIB:74-78`).

### 7.2 Theme, branding, CSS

- `SITE/🎨️.css` = `@import "…/UI/🧵️styles/🎨️.css"` (Tailwind v4 + palette + tokens + glass + globals) and `@source "…/REACT"` (`:4-5`). **Tailwind source detection is fragile here**: `@import "tailwindcss"` sits in `UI/🎨️styling/🖌️ui/🎨️.css:2` with `@source "../../"` (the UI module), and the comment at `:5-13` states that only that file's `@source` is honoured, relative to itself, and that the outer `@source` lines in `🧵️styles/🎨️.css` (and by the same logic `SITE/🎨️.css:5`) "reach nothing". I did not run a build to see which utilities of `REACT` or of a new folder survive; treat Tailwind utilities used **only** in a new directory as at risk and verify in the built CSS (`dist/assets/🌐️-*.css`) — or ship plain CSS with the pet module (imported from its `.tsx` like `REACT/🟦️.tsx:47` imports `./🎨️.css`).
- `REACT/🎨️.css` (466 lines): root font-size is `calc(100% * var(--quiz-text-scale,1))` (text size 1/1.125/1.25/1.5, so everything rem-based scales); `.quiz-app { overflow-wrap; hyphens }`; `.quiz-glyph { font-variant-emoji: text }` (emoji shown in the **monochrome Noto Emoji face**, not colour); icon motions `quiz-icon-bounce|pulse|spin|sway|float|flip` at `3.2s ease-in-out infinite`, gated by
  `@media (prefers-reduced-motion: no-preference)` **and** `.quiz-app:not([data-icon-motion="off"])` (`:39-72`); reduced motion also clamps transitions to 0.01 ms (`:410-416`); `@media (forced-colors: active)` section (`:418+`) gives states system colours; peers' look: `.quiz-peer { position:absolute; transition: transform 120ms linear; will-change: transform }` (`:316-358`).
- Appearance: `html.dark` class + `data-ui-appearance`; theme choices `system|light|dark` (`REACT/🔨️modules/🎛️preferences/🟦️.tsx:18,24-28`). Boot colours: light `#f7f3e3` text `#001117`; dark `#001117` / `#f7f3e3`. Palette tokens in `UI/🎨️styling/🎨️palette/🎨️.css:173-216` (`--color-primary #ff344f`, `secondary #34d1bf`, `tertiary #fa9500`, `danger #a60009`, `warning #fccf05`, `info #dbbea1`, `success #7eb77f`, `dark #001117`, `light #f7f3e3`, grays); fonts Anta (sans), Kelly Slab (serif), Share Tech Mono (mono) + Noto Emoji faces. Spacing tokens `--spacing-single|double|compact|touch`; strokes `--stroke-default`; surfaces `--base`, `--foreground`, `--active-base`.
- Preferences (local-only, persisted): `QuizPreferences {locale?, theme, textSize, showCursors, showAnswers, animateIcons}` (`REACT/🔨️modules/🎛️preferences/🟦️.tsx:42-49`, read defaults `:52-63`). A pet toggle belongs here; its labels must exist in EN and DE (`QUIZ_BUNDLE_EN/DE` in `REACT/🔨️modules/🌐️i18n/🟦️.ts`; the `🗣️translation-completeness` test fails otherwise) and `both-languages` e2e checks for unresolved labels.
- Hooks pets can walk on / measure (all `data-*` already present): `data-card="<name>"` on every `QuizCard` (`REACT/🔨️modules/🪟️chrome/🟦️.tsx:51`), `data-presence-anchor` (cards, leaderboard, `quiz:<id>`, `task:<id>`, `item:<id>`, `category:<id>`; `REACT/🔨️modules/👥️presence/🟦️.tsx:725-738`), `data-quiz-item`/`data-quiz-drag` on sorting/classification items (`↕️sorting:207`, `🗂️classification:51`), `data-layered-card|pane|overview|overlay|veil` on home, `#quiz-main`, `.quiz-app`. Duplicates behind glass carry `inert`.
- Layers on home (z-index): veil `z-30`, layered overlay `z-[31]`, presence overlay `z-40` (`UI/🧱️elements/🥞️LayeredOverview/🟦️.tsx:836,871`; `REACT/…/👥️presence/🟦️.tsx:1010`). Modal `Dialog` renders via `createPortal` (`REACT/🔨️modules/🪟️chrome/🟦️.tsx:13`). The pointer feed already exists (`usePresencePointer`, `:803-849`: passive `pointermove`, `pointerdown`, `blur`, `focusin`) but is only sent to the proctor; pets need their own passive listener. `useDocumentVisible()` (`:771`) is the existing "page hidden" signal.

---

## 8. Where an "architecture pets" definition set would live

Existing placement logic in this tree (taxonomy `🔣️taxonomy.json:6663-6750` kinds, `~12009-12048` members):

- `teaching` members: `🛂️proctor`, `🏛️architecture`.
- `teaching-architecture` members: `❓️quiz` (site), `⚡️energy` (domain). `teaching-quiz` may sit under `architecture` and the four topics; its children: `🧱️stack`, `🎭️e2e` (+ `🚶️learner`).
- Topics (`🧲️ 🔥️ ❄️ 📊️`) have exactly one registered member, `❓️quiz` (`members-of-teaching-topics`). `🎬️clip` is in the README but not registered.
- The Docker context admits only `🎓️teaching/**/❓️quiz/🔣️.json` (`SITE/🚀️deploy/Dockerfile.dockerignore`), so pet files next to quiz files **never enter the proctor image** and never change a quiz revision.
- The site already imports sibling data by relative path (`./🚀️deploy/🔣️.json`, `../../../🧰️framework/…/vector.svg?raw`), and the framework is domain-neutral with domain extensions supplied by the site (`logo`, `legal`, `tenant`).

### Option A (recommended): subject-level cast `🎓️teaching/🏛️architecture/🐾️pets/` beside `❓️quiz` and `⚡️energy`

- Content: one cast document (e.g. `🔣️.json`, schema-first, `topic` field per pet = `physics|heating|cooling|demand`, interactions between pets) + SVG parts under per-pet folders (`☀️sun`, `☁️cloud`, `🏠️house`, `🔆️solar-panel`, `🔥️radiator`, `♨️heat-pump`, `🪟️window`, `🧱️wall`, `🔋️battery`).
- Reasoning: the cast is cross-topic by nature (sun <-> solar panel <-> battery <-> house <-> heat pump span physics, heating, demand), `🏛️architecture` is the level that already owns shared site material (`README.md`, catalog via `❓️quiz`), and the site entry needs one import (`import pets from "../🐾️pets/🔣️.json"` -> `mountQuiz(root, { …, pets })`), symmetric to `site` from `🚀️deploy/🔣️.json`. One registration (new kind `teaching-pets`, parent `teaching-architecture`, add to `members-of-teaching-architecture`).
- Touch list: taxonomy kind + members; `📋️project.json` `namedInputs.default` (`{workspaceRoot}/🎓️teaching/🏛️architecture/🐾️pets/**/*`); plain CSS instead of Tailwind classes (§7.2); a vitest folder under `SITE/🧪️tests/` (e.g. `🧪️pets`) added to `🧪️tests/🎚️config/🟦️.ts:14` (validate cast against its schema with ajv) and, if browser-visible, a Playwright spec added to `testMatch` in `🎭️e2e/🎚️config/🟦️.ts:39-43`; README rows; and on the framework side a **domain-neutral** schema + renderer (`QUIZ/🧬️schema` neighbour or a new `🔨️modules` entry in `QUIZ`, React part under `REACT/🔨️modules/🐾️pets/🟦️.tsx` per the "targets own packages; package roots glue only" rule), exposed as `QuizOptions.pets?`.

### Option B: per-topic leaves `🎓️teaching/🏛️architecture/⚡️energy/<topic>/🐾️pets/` (sibling of `❓️quiz`, like the README's `🎬️clip`)

- Content: each topic folder holds the pets themed to that topic (physics: sun, cloud, battery; heating: radiator, window, wall, heat pump; cooling: window/shade, cloud; demand: house, solar panel, heat pump). Shared choreography still needs a home in Option A's folder, so this is effectively a hybrid.
- Reasoning for: topics already own content and the topic emoji (`quiz.emoji` must equal the folder emoji); pet <-> topic mapping is implicit by path, like `❓️quiz/🔣️.json`.
- Costs: four new directories to register (`members-of-teaching-topics` gets a second member and `teaching-pets` needs four `parentKindIds`), four more `namedInputs` globs (or `**/🐾️pets/**`), cross-topic interactions need yet another location, the site would need four imports (or a glob `import.meta.glob`, available in Vite), and pets that appear across screens (introduction, identity, leaderboard have no topic) do not map to a topic at all.

### Pitfalls common to both

- Do not put pet data into `🔣️.json` quiz/catalog files unless the proctor must know it (§0 fact 4).
- Static import only (CSP, §7); keep the cast small (budget §0 fact 9) or split into a lazily `import()`-ed chunk (a dynamic `import()` of a bundled chunk is a same-origin script load and allowed; it is **not** a `fetch`).
- Mount in `Client` only (`REACT/🟦️.tsx:397-478`); no pets on the bare language chooser (`:519-529`) unless the overlay is also added there.
- Tests required by AGENTS.md: a language-agnostic test per feature with a third-party oracle (site precedent: ajv, `yaml`, Vite preview; product precedent: `.feature` + `.py` + `.rs` + `.ts` quartets).
- Register any new command in both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (and `.claude/launch.json` if it is a dev server) following §6; scripts only through `📜️script.ts` (`project.json` -> `bun ./📜️script.ts <cmd>`; `package.json` -> `bun nx run …`).
