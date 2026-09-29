# 🏗️ Site, Deployment and Shared Registrations — Report (site-infra)

Scope: the `@teaching/architecture-quiz` website, its deployment for `quizze.architektur-und-technologie.de`, root scripts,
launch rows, taxonomy registration of every directory of this ticket, and the end-to-end browser walk.

## 1. What exists now

### Site `@teaching/architecture-quiz` — `🎓️teaching/🏛️architecture/❓️quiz/`

| Path | Content |
|---|---|
| `🟦️.ts`, `🌐️.html` | entry at the **owner root**: imports `./🎨️.css`, then `mountQuiz(root, { proctor: "", tenant: "architecture" })` |
| `🎨️.css` | brand layer: `@import "…/🎨️palette/🎨️.css" theme(static); @tailwind utilities source(none);` → every palette variable as `:root` custom properties, no utility classes |
| `🏗️builder/🌐️vite/🟦️.ts` | shared plugins (`semioHostHtmlVitePlugin` title "Quizze · Architektur und Technologie", `cnameHost`, `semioEmojiIndexHtmlVitePlugin`, `semioReferencedAssetsVitePlugin`, tailwind, react), `base: "/"`, output `📦️packages/🟦️typescript/dist`, dev proxy of `/instance /commands /queries /actors /scopes` (ws included) to `http://127.0.0.1:${PROCTOR_PORT:-8791}`, regex aliases for `@semio-tech/quiz`, `quiz-react` (→ `🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx`), `ui-react`, `framework-server`, `framework` |
| `📦️packages/🟦️typescript/{package.json, 📋️project.json, 📜️script.ts}` | glue only; `semio.app` (hostKind `architecture-quiz`, port 6061, env `TEACHING_ARCHITECTURE_QUIZ_PORT`); targets `dev` (continuous), `build` (→ dist, cached), `test`, `check`, `docker-image-build`, `docker-image-check` |
| `🧪️tests/🎚️config/🟦️.ts`, `🧪️tests/🧪️catalog/🟦️.ts` | catalog + every quiz: TS core `catalogIssues`/`quizIssues` report nothing, and **ajv 8.20** (third-party oracle) accepts them against the draft-07 contract |
| `🚀️deploy/Dockerfile` | `rust:1-bookworm` builder: bun install, site `bun ./📜️script.ts build`, `CARGO_TARGET_DIR=/out cargo build --release --locked -p teaching-proctor --bin proctor`, content staged by `find 🏛️architecture -path "*/❓️quiz/🔣️.json"` keeping relative paths; runtime `debian:bookworm-slim` + `tini`, user `quiz`, volume `/srv/quiz/data`, `PROCTOR_SITE=/srv/quiz/site`, `PROCTOR_CATALOG=/srv/quiz/content/🏛️architecture/❓️quiz/🔣️.json`, `PROCTOR_BIND=0.0.0.0`, `PROCTOR_MODE=production`, `HEALTHCHECK` on `/instance` with `X-Forwarded-Proto: https`, `ENTRYPOINT tini -- proctor`, `CMD serve` |
| `🚀️deploy/compose.yaml` | service `proctor` on `127.0.0.1:8791`, `PROCTOR_ALLOWED_ORIGINS=https://quizze.architektur-und-technologie.de`, `PROCTOR_TRUSTED_FORWARDING=proxy`, volume `proctor-data`, `stop_grace_period: 30s` |
| `🚀️deploy/Caddyfile` | `quizze.architektur-und-technologie.de { encode; HSTS/nosniff/referrer headers; reverse_proxy 127.0.0.1:8791 }` (automatic TLS) |
| `🚀️deploy/🟦️.ts` | operator verbs: `checkQuizCatalog` (`cargo run --release -p teaching-proctor --bin proctor -- check <catalog>` in a private target dir), `buildQuizImage`, `checkQuizImage` (fresh volume, production posture, ready behind the proxy, `/` + SPA fallback, cleartext refused, `docker stop` exit 0, `proctor.sqlite` on the volume, cleanup on Ctrl-C) |
| `README.md` | layout, launch rows, **Deploy**: DNS, first start, backup (stop + copy the SQLite file + WAL/SHM), restore, update |

### Shared registrations

- Root `package.json`: workspace `🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript`; scripts
  `dev|build|test|check:teaching:architecture-quiz`, `build|check:teaching:architecture-quiz:docker-image`,
  `dev|build|test|check|rebuild:teaching:proctor`, `test:quiz`, `test:quiz:react`, `test:quiz:rs`, `typecheck:quiz:react`.
- `.vscode/🧩️launch.seed.jsonc` (curated) → `.vscode/launch.json` regenerated with the registry's own `generateLaunchJson`
  (`site_infra_launch.ts`; output byte-identical to what `@semio-tech/plugin-registry:generate` writes, catalog untouched):
  3_dev `🛠️dev🎓️teaching🏛️architecture❓️quiz` (213.6, opens :6061), `🛠️dev🎓️teaching🛂️proctor` (213.61),
  `🔁️rebuild🎓️teaching🛂️proctor` (213.611), compound `🧭️compound🎓️teaching🏛️architecture❓️quiz🛂️proctor` (213.62),
  `🧪️test❓️quiz🟦️`, `🧪️test❓️quiz⚛️react`, `🧪️test❓️quiz🦀️`, `🧪️test🎓️teaching🛂️proctor🦀️`,
  `🧪️test🎓️teaching🏛️architecture❓️quiz` (213.63–.67), `🛠️dev❓️quiz⚛️react🪁️typecheck` (213.68);
  4_build `📦️build🎓️teaching🏛️architecture❓️quiz` (11.1), `📦️build🎓️teaching🛂️proctor` (11.2),
  `📦️build🎓️teaching🏛️architecture❓️quiz🐳️docker-image` (11.3);
  4_gate `✅️check🎓️teaching🏛️architecture❓️quiz📚️catalog` (11.1), `✅️check🎓️teaching🛂️proctor📚️catalog` (11.15),
  `⚖️gate🎓️teaching🏛️architecture❓️quiz🐳️docker-image` (11.2). Test levels and `quiz-rs:build` stay on the family rows
  (`⚖️test-quick📋️` … pickers now list every quiz/teaching project), as the audit accepted.
- `.claude/launch.json`: `teaching-proctor` (8791), `architecture-quiz` (6061).
- `🔣️taxonomy.json` via `site_infra_register_taxonomy.ts` (idempotent; re-run after new directories): kinds `teaching`,
  `teaching-{proctor,architecture,energy,physics,heating,cooling,demand,quiz}` (scoped by `parentKindIds`), global
  `deploy` 🚀️; member lists `members-of-teaching*`; `❓️quiz` in `members-of-products`; every quiz/proctor/react module,
  test case and fixture name in `members-of-{modules,tests,fixtures}` (derived from the inventory's unresolved
  directories); fixed contract `docker-compose` (`**/compose.yaml`, also clears `🌎️hub/compose.yaml`'s move finding);
  `areas["🎓️teaching"]=clean`, `areaLayers["🎓️teaching"]=implementation`.
- `.dockerignore`: header names both root-context images.

### Shared library changes (small, anchored)

1. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts:2642` `viteCliEntry` — `runViteBunxDev` resolved
   `node_modules/.bin/vite`, which on this Windows machine is an npm-written POSIX shell shim; bun parsed it as JS
   (`Expected ")" but found ""$(echo ""`) and **every** `runViteBunxDev` dev server died (projektetage too). Vite now
   resolves through `vite/bin/vite.js` (as `runViteBuild` already did; both share the helper).
2. `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:921` `semioReferencedAssetsVitePlugin` — serves
   `/🖼️assets/*` in dev; a build copies only the asset files the written bundle references (here the palette's 21
   woff2 fonts, dist 4.0 MB) instead of `semioAssetsVitePlugin`'s 174 MB asset root.

## 2. Verification (all run)

| Command | Result |
|---|---|
| `bun nx run @teaching/architecture-quiz:test --skip-nx-cache` | 11/11 passed (catalog + 4 quizzes, TS core and ajv) |
| `bun nx run @teaching/architecture-quiz:check` | `proctor check`: catalog architecture, 4 quizzes, 7 badges, exit 0 |
| `bun nx run @teaching/architecture-quiz:build --skip-nx-cache` | ok; `🌐️-*.js` 1,685 kB (≈367 kB gzip), css 36.5 kB, `pdf.worker` 1,232 kB (emitted, never loaded), 21 fonts |
| `bun install` | ok; `node_modules/@teaching/architecture-quiz`, `@semio-tech/quiz(-react)` linked |
| `docker compose config` (in `🚀️deploy`) | resolves |
| `docker-image-build`, `docker-image-check` | **not run: the Docker daemon is not running** (client 29.4.0, `dockerDesktopLinuxEngine` pipe absent); both verbs stop with `the Docker daemon does not answer: …` |
| `caddy validate` | not run: Caddy is not installed |
| production posture, natively (debug proctor, `PROCTOR_MODE=production`, origin allowlist, `TRUSTED_FORWARDING=proxy`, `PROCTOR_SITE=dist`) | `/instance` 200 with `X-Forwarded-Proto: https`; cleartext 403 `x-semio-refusal: insecure-transport`; `/` 200 `text/html`, `no-cache`, site title; `/leaderboard` 200 (SPA fallback); `/assets/missing.js` 404; emoji-named assets 200 with correct types; CORS preflight grants only `https://quizze.architektur-und-technologie.de` |
| `bun ./📜️script.ts verify taxonomy report --scope 🎓️teaching` | `clean=true errors=0 warnings=0` |
| `… --scope 🧰️framework/🛍️products/❓️quiz` | fast inventory 0 violations; the full report's single error in each of 3 runs was `reference-preimage-unreadable` on a file being edited during the scan (🪪️identity, 🏠️home, 📬️outbox-delivery) — a race with live edits, re-run once edits settle |
| `bun nx run workspace:verify -- interactivity apps` | exit 1 with the same 66 pre-existing failures as before my rows (1× `launch.json … exceed fixed capacity 512`, 64× plugin apps without React+WGPU launch variants, 1× extension descriptor); none concerns this ticket; the three new dev rows are discovered as launch-only products |
| router scripts (`site_infra_check_router.ts`, `fixedSourceDispositionDecision("root-script")`) | site, proctor, quiz core/rs/react `📜️script.ts`: all `tool-metadata`, no finding |
| scoped `tsc` over the site files | 0 errors in site/quiz files (52 pre-existing in `✏️s/🔌️plugins/📕️norm/…`) |

### Browser walk (`site_infra_e2e.ts`, Playwright on system Chrome; plus a manual pass in the built-in browser)

Intro → pseudonym → full physics run (classification selects + keyboard move buttons) → submit → **100 %**, badges
Physics Expert + Numerical Brain → leaderboard → second device (empty storage, `de-DE`, handle typed as
`  WALK   …  `) recalls **the same learner id**. Runs:

| Target | Viewport | Problems |
|---|---|---|
| dev site :6061 + dev proctor :8791 | 1280×900 | 2× `POST /queries net::ERR_ABORTED` (dev StrictMode double effect; absent in production) |
| built site served by proctor :8792 | 1280×900, 768×1024, 375×812 | **0**, no horizontal overflow |

Screenshots and `problems.txt`: `🗑️generated/site-infra-screens-{dev-1280,built-1280,768,375}/`. Brand fonts verified:
`--font-sans` = Anta…, 17 font faces loaded, headings render in Anta.

## 3. Defects for other owners

1. **react — bundle weight.** `🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts:12` imports `registerUiTranslationBundles`,
   `uiI18n`, `setUiLocale` from the `@semio-tech/ui-react` barrel; no narrower module exists (they live in
   `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` region `🔌️I18n Port`, ~4281–4490). Rendered composition of
   the site (`site_infra_bundle.ts`): react-dom 1,056 KiB (react-dom/server comes along), `🖼️assets/🌱️metabolism` icons
   680 KiB, `🖱️ui/🧱️elements` 331 KiB, three 249 KiB, icons 88 KiB, i18next 81 KiB; the quiz renderer itself 128 KiB.
   Treating all UI modules as side-effect-free (`--pure-ui`) only removes three + some elements (−405 KiB), so no
   site-level hack was adopted. Fix: move the i18n port into its own ui-react module/subpath export and import that.
2. **react — German register.** Identity strings use *Sie* (`🌐️i18n/🟦️.ts:301-310`, "Wie möchten Sie erscheinen?",
   "Ihr Pseudonym"); the catalog introduction and the rest use *du*.
3. **react — mobile leaderboard.** At 375 px the leaderboard description ("All learners with a submitted quiz, ranked
   by to…") is clipped instead of wrapping (`site-infra-screens-375/08-leaderboard.png`).
4. **react — first paint without a proctor.** With the proctor down, a first visit keeps the static host placeholder
   (no connection state, no error); with it up, "Loading… Waiting for the connection" shows next to "✓ Connected" for
   ~1–2 s.
5. **react/proctor — ties.** Equal totals get ranks 1, 2, 3 (no shared rank); design.md does not specify it.
6. **proctor — cache headers.** `🎓️teaching/🛂️proctor/🔨️modules/🌐️site/🦀️.rs:160-163` `is_hashed` splits the stem on
   `-`, so Vite hashes containing `-` (`🌐️-Djvsi-pa.js`) get `max-age=3600` instead of `immutable` (the css got
   immutable). Treat everything under `assets/` as hashed, or accept `[A-Za-z0-9_-]{8}` tokens.
7. **proctor — Windows exe lock.** A running dev proctor holds `target/debug/proctor.exe` (uplifts are hard links into
   the shared build dir); any rebuild, including `@teaching/proctor:check`, then fails with `Access is denied`. The site
   check uses the release profile in a private target dir; the proctor's own `check`/`dev` do not.
8. **shared builder (minor).** `SEMIO_FAVICON_HEAD_HTML` (`…/🏗️builder/🌐️vite/🟦️.ts:480`) uses relative `./` favicon
   hrefs, which break under path routes; the quiz keeps its URL at `/`, so there is no effect today.
9. **tooling (minor).** Under Bun, Vite's native config loader "restarts" on a config edit but keeps the old config
   (the module cache ignores the cache-busting query); the dev server must be restarted by hand.
10. **precedent.** `🎤️presentation` fails the taxonomy exactly as the first quiz react layout did
    (`target-inside-package-boundary`, package implementation files, unresolved test dirs) — not touched.

## 4. Ticket-folder files (site-infra)

Scripts: `site_infra_register_taxonomy.ts`, `site_infra_inventory.ts` (fast taxonomy inventory), `site_infra_launch.ts`,
`site_infra_check_router.ts`, `site_infra_e2e.ts`, `site_infra_bundle.ts`. Outputs (delete with `🗑️generated`):
`🗑️generated/site-infra-*.log|json`, `tsconfig.site-infra.json`, `site-infra-screens-*/`.
