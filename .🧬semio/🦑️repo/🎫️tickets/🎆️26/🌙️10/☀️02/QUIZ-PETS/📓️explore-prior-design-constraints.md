# Prior quiz design: constraints a pets layer must respect

Read-only audit for ticket `2026/10/02/QUIZ-PETS`. Nothing was run, built or edited; no server, test or git command that
modifies anything was used. Every statement comes from reading documents and the current working-tree source.

Evidence tags: **[S]** stated in a document of the prior ticket or a README, **[C]** confirmed by reading the current
source, **[D]** derived by me (an inference, not written anywhere), **[?]** could not be verified from here.

Path legend (all relative to `C:\git\semio`):

| Short | Path |
|---|---|
| `TK` | `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR` (prior ticket folder) |
| `Q` | `🧰️framework/🛍️products/❓️quiz` (product: schema, cores, README) |
| `RX` | `Q/🎯️targets/⚛️react` (renderer; implementation at the target root) |
| `M` | `RX/🔨️modules` (one folder per module, `🟦️.ts` / `🟦️.tsx`) |
| `T`, `F` | `Q/🧪️tests`, `Q/🧫️fixtures` |
| `S` | `🎓️teaching/🏛️architecture/❓️quiz` (the site: catalog, vite, deploy, e2e) |
| `UI` | `🧰️framework/🔨️modules/🖱️ui` (design system) |
| `LO` | `UI/🧱️elements/🥞️LayeredOverview/🟦️.tsx` |

## 0. The twelve facts that decide most things

1. **Nine live copies of every page exist at home.** `LayeredOverview rest="grid"` boots all panes at mount (`start="all"`),
   the home passes `budget = pages.length` and infinite suspend times, so nine page subtrees run at once, `inert` and
   `aria-hidden`, scaled into their cells. They stay mounted even while one page is opened (eight inert ones remain). A pets
   layer inside a page would be instantiated nine times. There is **no "this is a backdrop" flag or context**: a page only
   receives `{ opened, revealed }` (`PaneView`); the container carries `inert`/`aria-hidden`. [C] `M/🏠️home/🟦️.tsx:147-168,213-233`,
   `LO:236,385-404,482`.
2. **Mount the layer once, at app level**, as a sibling of `PresenceOverlay` (`fixed inset-0 z-40 pointer-events-none aria-hidden`
   in `RX/🟦️.tsx:474`). That is the only singleton overlay precedent; `PanePeers` is the only in-pane one (7 instances, each with
   its own 250 ms timer when peers exist). [C]
3. **The layer must have `pointer-events: none` everywhere.** The card host reveals a page on `pointerenter`/`pointerleave`
   (a pointer moving onto a foreign element fires `pointerleave` and conceals the page), a click anywhere on a card opens its
   page, drop targets are found with `document.elementFromPoint`, and Playwright's actionability check needs the target to receive
   the event. [C] `LO:439-442`, `M/🪟️chrome/🟦️.tsx:71-81`, `M/🤏️drag/🟦️.ts:14`.
4. **CSP of the release site is strict and lives in a `<meta>` tag** (GitHub Pages cannot send headers; the Caddyfile only guards
   the proctor API host): `default-src 'self'; script-src 'self' + sha256 of each inline boot script; style-src 'self' + sha256 of the
   boot style; style-src-attr 'unsafe-inline'; img-src 'self' data:; font-src 'self'; connect-src <proctor origin> <its wss twin>
   (no 'self'); manifest-src 'self'; object-src 'none'; base-uri 'none'; form-action 'self'`. So: no same-origin `fetch`/XHR of rig
   or sprite data, no `eval`, no blob images or blob workers, no WebAssembly (no `'wasm-unsafe-eval'` [D]), no runtime `<style>`
   element. The dev server has **no CSP**; violations show only in a release build or the e2e `rehearsal` topology. [C]
   `S/🏗️builder/🌐️vite/🟦️.ts:50-65,87`, `S/🚀️deploy/🟦️.ts:287-312`.
5. **`publish`/`deploy-check` refuse an over-budget artifact**: entry JS 260,000 B gzip, CSS 60,000 B gzip, 4,000,000 B in total;
   also any `[DEBUG]` string, source map, loopback address, unhashed file under `assets/`. Last recorded: JS 666.2 kB (gzip
   182.1 kB), CSS 248.9 kB (gzip 36.9 kB), 59 files / 2.13 MB. About 78 kB gzip of JS headroom as recorded on 2026-09-29, **not
   re-measured since** the hardening and icon work. [C+S] `S/🚀️deploy/🟦️.ts:86,319-346`, `TK/📓️react-report.md:454`,
   `TK/📓️deploy-readiness-report.md:105-108`.
6. **Nothing runs per frame at rest today** (the layered element's rAF chain stops when settled; the only continuous motion is
   the CSS task-icon loop, 3.2 s with long rests, gated by `prefers-reduced-motion: no-preference` and the `animateIcons`
   preference). Pets "slightly active by default" would be the first continuous consumer. An idle-CPU probe exists
   (`TK/dev_e2e_idle_cpu.ts`) but **no number was ever recorded** in any report. [C+S]
7. **A precedent for a decorative, switchable animation already landed today (2026-10-02)**: task icons with six microanimations,
   preference `animateIcons` (default on, "Animate task icons"), attribute `data-icon-motion` on `.quiz-app`, CSS gates, a test
   `T/🖼️task-icons`. It is documented only in `TK/📓️task-icons.md` and `Q/README.md:76-80`, **not in `design.md`**. [C+S]
8. **Presence schemas are closed** (`additionalProperties: false`, state <= 2 KiB, <= 15 frames/s per room from the client,
   <= 16 watched rooms, a socket closes on an inbound message > 4096 B). Sharing anything new (pet state) means: schema first, both
   cores (TS + Rust), conformance vectors with a Python/JSON-Schema oracle, proctor admission, client twin, privacy text, e2e.
   Letting pets look at other learners' cursors needs none of that: `PresenceView.peers` / `.rooms` already carry them, relative
   to anchors. [C]
9. **The preferences object is pinned by an exact-equality test** (`T/📡️presence-client/🟦️.tsx:839` expects exactly
   `{ theme, textSize, showCursors, showAnswers, animateIcons, locale: undefined }`), `readPreferences` copies only known fields (an
   unknown stored field is dropped on the next write), and a new key must exist in both language bundles and be used
   (`T/🗣️translation-completeness`). [C]
10. **The e2e gate fails every spec on any console error, page error, failed request (not abort), response >= 400, WebSocket error or
    CSP violation.** Chromium runs without reduced-motion emulation, four workers per topology, "a driven page keeps a core
    busy". A throwing or logging pet breaks the whole gate. [S] `TK/📓️dev-e2e-report.md:56-58,62-65`.
11. **The tree is dirty and shared.** Most quiz and site files show as modified (` M`) against `48d881aa7ab` in the session's git
    status, and the proctor image was labelled `48d881aa…-dirty` (`TK/📓️deploy-readiness-report.md:144`), so there is no clean
    baseline to diff pets against. Ticket `QUIZ-SORTING-NUMERIC-GUESSES` (today) edits `M/↕️sorting`, `M/📏️quantity`,
    the schema and `T/⌨️task-keyboard` while `QUIZ-PETS` will touch `RX/🎨️.css`, `RX/🟦️.tsx`, `M/🎛️preferences`, `M/🌐️i18n`.
    [C] Merge by editing only, never revert (AGENTS).
12. **Environment blockers to know before planning browser checks**: the dev proctor refuses the stale data folder
    `.🧬semio/🎓️teaching/proctor-dev` (storage v1 vs v2), so `dev` fails until it is removed (not deleted by anyone yet);
    `:typecheck` exits 1 for reasons outside the quiz (133 generated-binding errors, 0 in the quiz target); the Browser pane is hidden,
    so `visibilityState` is `hidden`, polling pauses and rAF is throttled. [S] `TK/📓️ui-hardening-report.md:25,157-159`,
    `TK/📓️closing-summary.md:166-167`, `TK/📓️task-icons.md:40-41`.

## 1. Section digest of `TK/📓️design.md` (the normative document)

The document has 17 sections and 398 lines. Later sections explicitly supersede earlier rules (see 1.1).

| § (lines) | Digest |
|---|---|
| 1 Tree and names (7-45) | Product `Q` (schema + 7 domain modules with TS and Rust twins side by side: randomness, sheet, validation, scoring, badges, lifecycle, views), `@semio-tech/quiz` glue only; React target at `Q/🎯️targets/⚛️react` with glue under `📦️packages`; proctor `🎓️teaching/🛂️proctor`; site/catalog under `🎓️teaching/🏛️architecture/❓️quiz`; four quizzes under `⚡️energy/{physics,heating,cooling,demand}`. Tests `🧪️tests/<case>/{🥒️.feature,🟦️.ts}`, shared vectors `🧫️fixtures`, oracles `🔮️oracles/🔣️.json`. |
| 2 Identifiers (47-52) | Slugs `^[a-z0-9]+(?:-[a-z0-9]+)*$`; learner/run/command ids 32 lowercase hex from `crypto.getRandomValues`; quiz revision = SHA-256 of the file bytes. |
| 3 Randomness (54-60) | Bit-exact across languages: FNV-1a 32 of the run id seeds MT19937; `uniform(n)` by rejection sampling; Fisher-Yates from the end. This is the repo's blessed deterministic PRNG and has a numpy oracle. |
| 4 Sheet (62-73) | `sheet(quiz, seed)`: shuffled task order, per task shuffled items (then `draw` subset), shuffled categories and value cards; an already ascending sorting is rotated left by one; sheet items carry only `{ id, label }`. Every learner's sheet differs. |
| 5 Answers (75-83) | Shapes for classification/sorting/matching; validity rules; partial answers valid while the run is open; submission needs every task complete (`run-incomplete`). |
| 6 Scoring (85-106) | Per task in [0,1], run = mean. Sorting and matching: magnitude-weighted pair concordance (log10 for logarithmic scales; Kendall/Spearman as third-party oracles); classification: profile distance. Cross-language tolerance 1e-12. |
| 7 Badges (108-115) | Evaluated after each submission: `perfect-quiz`, `perfect-tasks`, `completed-quizzes`. |
| 8 Learner lifecycle (117-142) | Pure `decide`/`evolve` shared by both cores; handle policy; command table; idempotent by command id; `at` = decision time. Partly outdated (roster actor and `learner-recalled` are gone, see 1.1). |
| 9 / 9a Proctor (144-176) | Rust `ServerInstance` over one SQLite file (WAL), roster + learner actors, projection saga, four queries, config by environment, Docker; wire mapping to the framework server contract. Static hosting here is superseded by §14. |
| 10 Client (178-195) | Steps introduction -> identity -> home -> run -> results -> leaderboard. Every task works by keyboard only and by pointer drag; spider diagrams are owned SVG with text alternatives. **Local-first with the four state classes**; outbox coalesced per (run, task), jittered retry, idempotent; connection state visible; submission has progress and cancel. **i18n: en and de everywhere, offered in that order, no default**; explicit choice persisted; the browser list may only preselect an offered language; when it names none the document has no `lang`, title/loading/`<noscript>` are bilingual and the client asks "Language · Sprache". **Customization: theme (light/dark/system) and text size via CSS custom properties on the semio palette.** |
| 11 Core API (197-221) | One TS type / Rust type per schema `$defs` entry; function table identical in both cores; vectors in `🧫️fixtures/<case>/🔣️.json` read by both cores and the Protocol v2 cases. |
| 12 Seams (223-236) | Dev ports site 6061, proctor 8791 (vite proxies `/instance /commands /queries /actors /scopes`). Renderer seam `mountQuiz(root, { proctor, tenant }) => unmount` and `QuizApp` component. Registrations list (workspaces, launch rows, taxonomy, oracle registry). |
| 13 Revisions after audit (238-245) | Leaderboard rows carry `tag = fnv1a32(learner id)` (8 hex), never the id (an anonymous id is a credential); both cores degrade identically on invalid input (no NaN, no throw). |
| 14 Card grid, CDN, Docker (247-291) | Site on a CDN (GitHub Pages shape) at `quizzes.architektur-und-technologie.de`, proctor API-only on Docker at `proctor.quizzes...`; release build bakes `VITE_PROCTOR_URL`. UI: every card is the design system's `WindowChrome` (`level="dialog"`), `Navbar` + `ShellBrandLogo`, shared `OverviewCard`. Home = nine sections in DOM/reading order (learner, quiz 1, how it works, quiz 2, **leaderboard (centre)**, quiz 3, badges, quiz 4, preferences); desktop >= 1024 3x3 with larger centre, tablet 768-1023 two columns, phone <= 767 one column. **Bundle budget: import only slim ui-react subpaths; main chunk near 414 kB** (stale, see 2.3). |
| 15 Presence and cursors (293-332) | Ephemeral shared state over `GET /scopes/{scope}/presence/ws` (`semio.presence.v1`): latest state per session, one `batch` per 100 ms tick, state <= 2 KiB and <= 30/s; rooms: roster scope `<catalog>` with `PresenceState`, place scopes with `CursorState`; cursor = `{ anchor, x, y }` in 0..1 relative to an anchor every learner renders. Client: two sockets, jittered reconnect, pointer frames <= 15 Hz coalesced, keyboard focus shared, others' cursors interpolated unless reduced motion, **decorative (`aria-hidden`) with a text roster for assistive technology**, label = identity display ("Anonymous #tag"); preference "Show others' cursors" (persisted local-only). No room on the identity screen. |
| 16 Layered home (334-357) | Home is the shared `LayeredOverview` (strip of real pages behind ONE `ui-veil` glass with a `clip-path` hole; reveal on hover **or keyboard focus**; 500 ms eased glide; reduced motion snaps; hash open / Escape / Overview button with focus management; list mode for touch phones; per-pane error boundary). Backdrop pages are pure views over session state (never a run: starting a run is a command). Schema `Screen` gains `quiz learner badges preferences`. |
| 17 Sharing inside the quizzes, live grid (359-398) | Supersedes §15's "never share answers or drags". Thinking room `<catalog>/quiz/<quiz>/thinking` with `ThinkingState { tag, answers }` (<= 2 Hz, aggregated semantically by item id because sheets differ); persisted `CrowdView` projection; item/category anchors and `drag { item }` in cursor states. `LayeredOverview` rest mode `"grid"`: all nine pages visible and live behind their cards; the leaderboard polls while home is visible; framework socket gains `watch` (<= 16 scopes, read-only, `watched` frames); home watches page, quiz and thinking rooms at ~4 Hz. Bounds: 64 tasks / 64 entries (`too-many`). |

### 1.1 What later documents changed relative to `design.md` (so the contract is read correctly)

| design.md says | Now |
|---|---|
| §9 proctor serves the site (`PROCTOR_SITE`) | removed in §14 (API only). |
| §10 preferences = theme + text size | code has `locale?, theme, textSize, showCursors, showAnswers, animateIcons` (`M/🎛️preferences/🟦️.tsx:41-63`); §10 language decision recorded at L190-194. |
| §14 "main chunk stays near 414 kB" | 666.2 kB (gzip 182.1) at the last recording; the enforced budget is in `S/🚀️deploy/🟦️.ts:86`. |
| §15 "items, cards and drags never shared" | superseded by §17. |
| §16 "leaderboard polls only while opened or revealed" | superseded by §17: `HomeScreen` polls while home is visible (`M/🏠️home/🟦️.tsx:107`, once, not per pane). |
| §8/§9a roster actor, `learner-recalled`, recall as command | roster actor gone, handle actors, recall is a `handle` query, `roster-full`, `runs-exhausted`, `answers-exhausted` (`TK/🗒️domain-notes-for-ui.md:21-52`). |
| leaderboard = all learners | `Leaderboard { rows (top 100), learners, own? }` (`Q/README.md:240-243`). |
| (absent) task icons, `Motion`, `animateIcons` | `Q/🧬️schema/🔣️.json:53-67`, `Q/README.md:76-80`, `TK/📓️task-icons.md`. |
| (absent) legal footer, "what is stored" dialog, `site.legal` | `TK/📓️ui-hardening-report.md:117`, `TK/🗒️ui-notes-for-deploy.md:46-55`. |
| (in progress) numeric guesses in sorting | `TK/../QUIZ-SORTING-NUMERIC-GUESSES/📓️design.md` (ticket 2026/10/02, same files as pets). |

The ticket was closed several times and **reopened again on 2026-10-01/02** (dev launcher, e2e, deploy readiness, hardening, task
icons); `TK/🎫️ticket.json` status is `open`. `TK/📓️closing-summary.md` ends at the 2026-09-29 night revision (167 lines).

## 2. Constraints relevant to a decorative, animated overlay

### 2.1 Accessibility

| Constraint | Source |
|---|---|
| Yardstick is **WCAG 2.2 AA**; German "du"; no default language. | [S] `TK/📓️audit-final-i18n-a11y.md:7` |
| Others' cursors are decorative: `aria-hidden`, `pointer-events-none`, never hit by the pointer; the information is available as text (online count, who is where). A decorative layer needs a text equivalent only if it carries information. | [S] `design.md` §15 (326-329); [C] `M/👥️presence/🟦️.tsx:1003-1014`; `audit-final-i18n-a11y.md:294` |
| **Reduced motion**: handled in CSS (`@media (prefers-reduced-motion: reduce)` cuts all `transition-duration` to 0.01 ms under `.quiz-app`, `RX/🎨️.css:410-416`), in `LayeredOverview` (`useReducedMotion`, `reducedMotion: "auto"`: glide and zoom snap, pointer pan off; **this hook is internal to the element, not exported from `@semio-tech/ui-react/chrome`**), in the card lift (`motion-reduce:`), and in task icons (keyframes only inside `@media (prefers-reduced-motion: no-preference)`). **A JS/rAF/SMIL/canvas pet is not covered by any of it.** `useMediaQuery` is exported from chrome and is the available primitive. | [S] `audit-final-i18n-a11y.md:152`, `audit-final-requirements.md:62,147-149`; [C] `LO:155,169-180`, `RX/🎨️.css:39-72,410-416` |
| Motion preferences follow the OS; the only own switch is "Animate task icons" (default on). N8 notes there is no own reduced-motion or contrast switch. | [S] `audit-final-requirements.md:147-149`; [C] `M/🎛️preferences/🟦️.tsx:48,61,151-154` |
| WCAG 2.2.2 (pause, stop, hide for moving content > 5 s) and 2.3.3 are not discussed except that the icon test cites `animation-from-interactions` (2.3.3). A permanent, auto-starting pet animation needs a visible stop/hide control. | [D] (`T/🖼️task-icons/🟦️.tsx:7`) |
| **Contrast of decorative text is still held to 4.5:1**: D5 fixed the peer name labels (ink computed per palette slot per appearance). Any speech bubble or label on a pet is held to the same; fixture `F/🌗️contrast-states` (colord oracle). | [S] `ui-hardening-report.md:52`; [C] `T/🌗️contrast-states/🟦️.tsx` |
| **Forced colors (Windows high contrast)**: every state painted by background, shadow or custom colour needs a rule inside `@media (forced-colors: active)`; the test parses the stylesheet with lightningcss and requires an entry per selector in `F/🌗️contrast-states/🔣️.json`. Pets drawn with custom colours must be hidden or get a rule plus a fixture entry. | [S] `ui-hardening-report.md:53`; [C] `RX/🎨️.css:418-466`, fixture `forcedColors` list |
| **Focus Not Obscured (SC 2.4.11)**: a peer label that would lie within 24 px (`PEER_LABEL_CLEARANCE_PX`) of the focused control is hidden (`data-shy`). Pets "walking on top of UI elements" must do the same (step aside, hide, or never overlap interactive controls). | [S] `audit-final-i18n-a11y.md:263`, `ui-hardening-report.md:94`; [C] `M/👥️presence/🟦️.tsx:889-911` |
| **Live regions**: the connection indicator is no live region; a separate polite `p[role=status][data-connection-announcer]` speaks once per alert/recovery; task announcers are polite, atomic, debounced 400 ms; presence, crowd and leaderboard updates are never announced; **backdrop panes carry no `role=status`/live region**. Pets must add none. | [S] `ui-hardening-report.md:51`, `audit-final-i18n-a11y.md:294`, `ui-reference-layered-landing.md:239` |
| **Focus order**: DOM order = reading order = tab order; the Overview button is first in the DOM; skip link first; `h1` takes focus on each screen change; opened page is a labelled region that takes focus, Escape closes it and returns focus to the card; modal dialogs are portaled into `.quiz-app`, make **all other children of `.quiz-app` inert** (a pets layer that is a child would be inerted too), trap Tab, Escape at document level in the capture phase. N7: two tab stops per sorting item is already a noted concern, so pets must add **zero** tab stops. | [S] `ui-reference-layered-landing.md:247`, `ui-hardening-report.md:72-73`; [C] `M/🪟️chrome/🟦️.tsx:240-321`, `LO:749-756` |
| **Keyboard**: every task works by keyboard only; drag is an addition on a grip; Escape abandons a drag and closes an opened page (`window` keydown, `!defaultPrevented`). A pet must not capture keys. | [S] `design.md` §10; [C] `RX/🟦️.tsx` + `M/👥️presence` `usePresencePointer` |
| **Touch**: hover reveal is mouse-only (`pointerType === "mouse"`); keyboard focus reveals; a tap opens the page; "no function is hover-only". Pets that react to the cursor need a touch fallback. Targets >= 24 px (`.quiz-target`). | [S] `audit-final-i18n-a11y.md:149-151`; [C] `LO:439-442`, `RX/🎨️.css:155-158` |
| Colour or opacity never carries meaning alone (badges fixed in D5 of the requirements audit; N13). | [S] `audit-final-requirements.md:119-122`, `audit-final-i18n-a11y.md:279` |
| Text size: root font size scales every rem token (1 / 1.125 / 1.25 / 1.5); home breakpoints scale with it. Pets sized in rem follow for free; px-based pets do not. | [S] `audit-final-i18n-a11y.md:144`, `ui-hardening-report.md:79`; [C] `RX/🎨️.css:7-9` |
| Not measured in a real browser: I9 (chip overflow), I10 (short viewports), I25 (labels over focus), real high-contrast mode, screen readers. | [S] `ui-hardening-report.md:149-155` |

### 2.2 Internationalisation

| Constraint | Source |
|---|---|
| Two languages `en`, `de`, offered in that order, **no default**; explicit choice persisted; otherwise the first offered entry of `navigator.languages` preselects (shown in the language switch); when none matches nothing is assumed, `<html>` has no `lang`, a neutral "Language · Sprache" chooser appears first. | [S] `design.md` §10 (190-194), `ui-hardening-report.md:123-126` |
| German addresses the learner with **"du"** everywhere; one German word per concept; counts as "Label: n" (no plural machinery); numbers/dates through `Intl` in the active locale. | [S] `react-report.md:199-201`, `ui-hardening-report.md:85,87,82`, `audit-final-i18n-a11y.md:47` |
| Every client string lives in `M/🌐️i18n/🟦️.ts` (typed `typeof QUIZ_BUNDLE_EN`); `T/🗣️translation-completeness` enforces equal keys, equal placeholders, non-empty labels, **no unused key**, no formal German, no hard-coded JSX text or attribute literals. Keys: 224 at audit, 272 after hardening. | [S] `audit-final-i18n-a11y.md:29-38`, `ui-notes-for-e2e.md:164`; [C] |
| Learner content (titles, labels, explanations) is data: `Text` requires both `en` and `de`. Pet names or speech that are domain content belong into the catalog/quiz JSON (or the bundle) in both languages. | [S] `Q/🧬️schema/🔣️.json` `Text`; `audit-final-i18n-a11y.md:38` |
| `lang` is on `.quiz-app` and on language buttons; parts in another language carry their own `lang`. Pet text inside art (SVG text, canvas, sprites) is invisible to all of this: **no baked-in words**. | [S] `audit-final-i18n-a11y.md:161`; [D] |
| `quiz.preferences.summary` is a one-line description of all settings and was already found stale once (I15); it is stale again (does not mention icon animation). A pets switch must update it in both languages. | [S] `audit-final-i18n-a11y.md:243`; [C] `M/🌐️i18n/🟦️.ts:101,437` |

### 2.3 Performance

| Constraint | Source |
|---|---|
| **Site artifact budget** (enforced at `publish` and in `deploy-check`, not at `build`/`test`): entry script <= 260,000 B gzip, stylesheet <= 60,000 B gzip, artifact <= 4,000,000 B. Recorded sizes: 414 kB -> 592 -> 614 -> 642 -> 666.2 kB JS (gzip 182.1 kB), CSS 248.9 kB (gzip 36.9 kB), 59 files 2.13 MB. The big 2026-09-29 win was moving the i18n port out of the 11,800-line ui-react barrel (1,691 kB -> 414 kB). **Pets must import only slim subpaths (`/chrome`, `/i18n`), never the barrel.** | [C] `S/🚀️deploy/🟦️.ts:86,319-346`; [S] `closing-summary.md:42-45,92`, `react-report.md:372-374,423,454`, `deploy-readiness-report.md:105-108` |
| Release build strips `console`, no source maps; a `[DEBUG]` string anywhere in shipped code fails `publish`. | [C] `S/🚀️deploy/🟦️.ts:337,342`; [S] `audit-deploy-security.md:170` |
| **Idle contract of the layered element**: transforms and veil are written imperatively (no React render per frame), pointer pan is passive and rAF-driven only while unsettled, one rAF chain that stops when settled, `contain: layout paint` and `will-change: transform` on strip/panes, percent geometry (no resize listeners). | [S] `ui-reference-layered-landing.md:103,251`; [C] `LO:163,401,507-573` |
| The overview is already heavy: nine live pages, ~32 `backdrop-filter` elements plus one veil (measured on the prototype: 704 DOM nodes), `PanePeers` timers, 10 s leaderboard poll, crowd refresh. The N12 note says **"the live grid performance was judged from CSS and code only"**. [D] Anything animating *under* the veil or a glass card forces the blur to be recomputed each frame; keep pets in a layer above the card overlay (z >= 40). | [S] `ui-reference-layered-landing.md:243`, `audit-final-requirements.md:166-168` |
| Existing animation under the veil: the four backdrop quiz pages render `TaskGlyph` (nine infinite CSS loops in total, 3.2 s each) with **no backdrop gate**. | [C] `M/📖️quiz-page/🟦️.tsx:196`, `RX/🎨️.css:39-72`, `TK/📓️task-icons.md:19-29` |
| **Idle CPU probe**: `TK/dev_e2e_idle_cpu.ts` (`bun dev_e2e_idle_cpu.ts [origin] [seconds]`, Playwright, 1440x900, `en-GB`) reads CDP `Performance.getMetrics` over a window (default 10 s) and prints the share of wall time in `TaskDuration`, `ScriptDuration`, `LayoutDuration`, `RecalcStyleDuration`, plus layout and style-recalc counts and node count, at three places: introduction, **overview at rest** (after 3 s), **inside a run** (after 2 s). Logs are `[DEBUG]`-prefixed. **No threshold and no result is recorded anywhere** (no report, no log); run it before and after to get a baseline. | [C] `TK/dev_e2e_idle_cpu.ts:1-38`; searched all `TK/*.md` and `🗑️generated` |
| Test pressure: a driven page keeps a processor core busy; 4 workers per topology, 2 each when both run; more workers were slower on 16 threads; both topologies about 7 minutes. Continuous pet animation multiplies this. | [S] `dev-e2e-report.md:62-65`, `S/README.md:80-82` |
| Presence cadence is the only rate budget in the docs: client state frames <= 15 Hz (`PRESENCE_FRAME_INTERVAL_MS` 67 ms), thinking <= 2 Hz (500 ms), watched rooms 4 Hz (250 ms), server tick 100 ms. No frame-time budget (ms/frame, fps) is defined anywhere. | [C] `M/👥️presence/🟦️.tsx:56-69`; [S] `design.md` §15, §17 |
| Layout work precedent for following moving elements: `placePeers` reads `getBoundingClientRect` only for peers present, scheduled by rAF on resize, scroll (capture, passive), `focusin` and a 500 ms (250 ms in panes) interval; writes `style.transform`. | [C] `M/👥️presence/🟦️.tsx:913-998` |
| Layout is **not stable**: the others' answers appear and disappear under every item (`CrowdChoices`) and shift the page by the height of those lines; documented as open (a hand-driven drag had to follow the bin). Pets anchored to elements will see this. | [S] `dev-e2e-report.md:102-105,118-121` |

### 2.4 Security, CSP, deploy

| Constraint | Source |
|---|---|
| The **site CSP is a `<meta http-equiv="Content-Security-Policy">`** written by the Vite build (`apply: "build"`) as the first thing after the charset, hashing every inline `<script>`/`<style>` of the document. Directives: `default-src 'self'`; `script-src 'self' <sha256…>`; `style-src 'self' <sha256…>`; `style-src-attr 'unsafe-inline'`; `img-src 'self' data:`; `font-src 'self'`; `connect-src <proctor origin> <wss twin>`; `manifest-src 'self'`; `object-src 'none'`; `base-uri 'none'`; `form-action 'self'`. `publish` re-verifies independently and fails on any `default-src`/`script-src`/`style-src` source that is not `'self'`, `'none'` or a sha256, on `connect-src` that is not exactly the proctor origin and its `wss` twin, and on any inline block not hashed. | [C] `S/🏗️builder/🌐️vite/🟦️.ts:46-65,67-95`, `S/🚀️deploy/🟦️.ts:283-312`, `S/🧪️tests/🧪️deploy/🟦️.ts:63,190-199`; [S] `deploy-readiness-report.md:93-99` |
| Consequences for pets: **script-src** no `eval`/`new Function`, no inline scripts, no blob workers (a module worker emitted as a hashed file from the site origin would fall under `'self'` [D]); **WebAssembly is not admitted** (CSP needs `'wasm-unsafe-eval'`, which the verifier rejects) [D]; **style-src** no runtime `<style>` element, no inline SVG `<style>`; CSSOM writes (`element.style.setProperty`, React `style` props, `classList`) and `style="…"` attributes are fine (`style-src-attr 'unsafe-inline'`, kept for the shared host template's placeholder and `<noscript>`); **img-src** same-origin files and `data:` only (no `blob:` images, no remote); **font-src** `'self'`; **connect-src** only the proctor, so **`fetch()`/XHR of own-origin rig or animation data is blocked** and fails the e2e guard; load data by importing it into the bundle (JS/JSON modules, `?url`/`?inline` assets). | [D] from [C] directives; [S] `ui-notes-for-deploy.md:5-25` |
| The existing client was grepped for runtime `createElement("style"|"script")`, `insertRule`, `adoptedStyleSheets`, `setAttribute("style")`: none. SVG/icon markup is set with `dangerouslySetInnerHTML` and is free of `<style>`/`<script>`/`style=`. | [S] `ui-notes-for-deploy.md:13-15` |
| Built assets must be content-hashed under `assets/` (served immutable), referenced assets only are copied (`semioReferencedAssetsVitePlugin`), no loopback or `localhost` string outside the baked proctor origin, no `sourceMappingURL=`, `/@vite/client`, `react-refresh`, `[DEBUG]`. | [C] `S/🚀️deploy/🟦️.ts:336-342`, `S/🏗️builder/🌐️vite/🟦️.ts:118-119` |
| **The Caddyfile is the proctor API host only**: `Content-Security-Policy: default-src 'none'; frame-ancestors 'none'`, HSTS, nosniff, `X-Frame-Options: DENY`, `Referrer-Policy: no-referrer`, request body limit 16 KiB, timeouts. It does not touch the site. `_headers` (`frame-ancestors 'none'`, nosniff, referrer policy; immutable `/assets/*`, documents no-cache) only helps CDNs that honour it; GitHub Pages ignores it and caches everything 10 minutes. | [C] `S/🚀️deploy/Caddyfile:27-57`, `S/🚀️deploy/🟦️.ts:88-101`; [S] `S/README.md:328-332` |
| Proctor limits that a shared pet protocol would meet: presence socket closes on an inbound message > 4096 B (state <= 2048), 30 frames/s per socket, <= 16 watched scopes, 2048 sockets per address and 8192 in total, commands 300/s per address, only five routes are mounted (`/blobs/*`, `/apps*`, `POST /scopes/{scope}/ephemeral` answer 404). | [S] `TK/🗒️edge-notes-for-deploy.md:23-35,46-56` |
| No XSS sink in the client: no `dangerouslySetInnerHTML` of learner data, no `innerHTML`, no `eval`. Presence is unauthenticated (tag, identity, surface are client-asserted): "fine for fun, do not build trust on it". | [S] `audit-deploy-security.md:151,164` |
| The introduction text comes from the catalog served by the proctor, so a very first visit with the proctor down shows only the waiting card; returning visitors use the cached catalog. | [S] `audit-final-requirements.md:163-165` |

### 2.5 Privacy

| Constraint | Source |
|---|---|
| Public by design: pseudonym/name, points, badges, runs, last submission on the leaderboard; while online the others see presence, place, pointer, keyboard focus, drags and **current draft answers** (live, not saved). The identity step, the leaderboard page and the introduction say so ("for fun"). | [S] `ui-notes-for-e2e.md:125,145,153-156`, `ui-hardening-report.md:115` |
| The built-in notice "What is stored / Was gespeichert wird" lists: on the server (handle, runs with answers and scores, badges), in the browser (learner id, settings, copy of runs, unsent answers), live-only (presence, pointer, current answers). A local-only pet preference falls under "your settings"; **a shared pet state would need its own sentence in both languages** and the `T/🔏️privacy-notice` test. | [S] `ui-notes-for-e2e.md:151-156`; [C] `M/⚖️legal/🟦️.tsx` |
| The learner id is the only credential: it is never shared in presence, leaderboard, crowd or session ids; presence carries the 8-hex `tag` (`fnv1a32(id)`); the schema refuses a learner id as a tag (`tag-invalid`). Any new shared state must carry the tag only and be validated before relay by both cores. | [S] `design.md` §13 (240-243), `Q/README.md:245-248,269-273`, `audit-deploy-security.md:160` |
| No third-party request is possible from the release site (CSP), no cookies, no external fonts or images. Nothing personal outside `semio.quiz.<tenant>.*` in `localStorage`. | [C] CSP above; `M/💾️persistence/🟦️.ts:153-181` |
| No access log is kept by Caddy/proctor (no client addresses stored). | [S] `S/README.md:292-296` |

### 2.6 Offline and connection-shortage behaviour

| Constraint | Source |
|---|---|
| Local-first: answers apply locally at once, a per-record outbox coalesced per (run, task) is retried with jittered backoff, idempotent by command id; the app must not freeze on a short outage; long offline is not accepted. | [S] `design.md` §10 (185-189), AGENTS rules |
| The connection indicator has tones `calm`, `busy`, `alert`; a throttled proctor (`429`, `503` + `Retry-After`) is "busy", not an alert; `Retry-After` honoured (capped 120 s), plus jittered backoff. | [S] `ui-hardening-report.md:118`, `ui-notes-for-deploy.md:57-66`; [C] `RX/🟦️.tsx:190-225` |
| Presence rooms: on socket loss members and watched rooms are **cleared at once** (`lost()`), status `waiting`, rejoin with jittered backoff that grows while sessions keep ending within 10 s (`PRESENCE_STABLE_MS`), latest state and watch re-sent after each rejoin. Pets that look at others must tolerate peers vanishing from one frame to the next. | [C] `M/👥️presence/🟦️.tsx:71-81,221-232,315-320` |
| Presence/roster exist only for an **identified** learner; before identification, on the identity screen, and on `learner`/`preferences` pages there is no cursor room (no tag, or personal pages). | [S] `design.md` §15 (331-332), `audit-final-requirements.md:83-87`; [C] `M/👥️presence/🟦️.tsx:436-466,524-526` |
| Pets that are purely local need no network at all and should work on the introduction and identity screens, offline, and before the proctor answers. | [D] |

### 2.7 Visibility handling

| Behaviour | Source |
|---|---|
| `usePolling` does nothing while `document.visibilityState === "hidden"` and asks at once when the tab becomes visible again (unless backing off). | [C] `M/🏆️leaderboard/🟦️.tsx:93-129` |
| `useDocumentVisible()` feeds `PresenceState.active` (the schema text says "visible and interacted recently"; the code uses visibility only). | [C] `M/👥️presence/🟦️.tsx:771-779`, `RX/🟦️.tsx:401,409`; schema `PresenceState.active` |
| `LayeredOverview` never suspends panes for hidden tabs on home (`suspendHiddenMs: Infinity`), so hidden-tab savings come only from browsers throttling rAF/timers. | [C] `M/🏠️home/🟦️.tsx:225` |
| In the Browser pane the page is `hidden`, polling is paused by design and rAF is throttled (a 500 ms glide took ~1-1.5 s). The earlier walk overrode `document.visibilityState` in the page for debugging only. | [S] `closing-summary.md:166-167`, `ui-reference-layered-landing.md:14` |
| [D] Pet loops must listen to `visibilitychange`, stop timers and rAF while hidden, and resume without a catch-up burst. | derived from the above |

### 2.8 Devices, viewports, text size

| Constraint | Source |
|---|---|
| Priority **desktop, then mobile, then tablet**. Breakpoints are the design system's: phone <= 767, tablet 768-1023, desktop >= 1024 (`UI_MOBILE_MAX_WIDTH_PX = 767`, `UI_TABLET_MAX_WIDTH_PX = 1023`); the **home layout queries are multiplied by the text scale** (text 1.5x leaves a viewport two thirds as wide and high) and a viewport shorter than 480 px x scale gets the list layout. | [S] `design.md` §14 (283-285), `audit-final-requirements.md:65`; [C] `M/🏠️home/🟦️.tsx:72-85,117`, `UI/📱️device/🟦️.ts:26-39` |
| Home modes: desktop 3x3 with tracks 1 : 1.5 : 1 by 1 : 1.4 : 1 (leaderboard in the centre), tablet 2 columns with the leaderboard on its own row, list below 768/short: one snap section per pane, **only the current and next pane live**, each with its own veil, no hover reveal on touch. | [C] `M/🏠️home/🟦️.tsx:47-70`, `LO:825-846`; [S] `ui-reference-layered-landing.md:221` |
| `.quiz-app` is `overflow-hidden` with `height: var(--ui-available-height, 100dvh)`; screens other than home scroll **inside** a `Page` (`overflow-auto`) container, so the document never scrolls and element positions change with nested scrolling (capture-phase scroll listener needed). Cards on home are compact and centred in their cells; the card layer takes the grid's tracks without gaps. | [C] `RX/🟦️.tsx:283-294,451`, `RX/🎨️.css:160-188` |
| E2E viewports: desktop 1440x900, phone 375x812 (`isMobile`, `hasTouch`). | [C] `S/🎭️e2e/🎚️config/🟦️.ts` |
| Not measured: 200 %/400 % zoom, landscape phone, "Largest" text, long German labels (I9, I10). | [S] `ui-hardening-report.md:152-155` |

### 2.9 The layered home: exactly what is rendered how many times

**Mechanism** (all [C], `M/🏠️home/🟦️.tsx` and `LO`):

- `HomeScreen` builds `panes = homePages(quizIds).map(page => ({ id, label, icon, render }))`: ids in reading order
  `learner, <quiz1>, intro, <quiz2>, board, <quiz3>, badges, <quiz4>, prefs`, then any further quizzes. For the architecture
  catalog that is 9 panes (physics, heating, cooling, demand). `render(paneState)` returns the real page component
  (`LearnerPage`, `QuizPage`, `IntroductionPage`, `LeaderboardPage`, `BadgesPage`, `PreferencesPage`) with `view = { opened, revealed }`.
- `<LayeredOverview rest="grid" mode={layout === "list" ? "list" : "strip"} gridTracks={...} lifecycle={{ budget: pages.length, suspend*Ms: Infinity }} />`.
- `rest="grid"` starts every pane at mount (`usePaneLifecycle(..., start = "all")`, initial `booted = all ids`), so nine page
  subtrees exist simultaneously; none is ever released (budget >= count, no finite suspend time). In `strip` mode each pane
  is `div[data-layered-pane=<id>]`, `position:absolute; inset:0` at full container size, `transform-origin: 0 0`, `will-change: transform`,
  `contain: layout paint`, with an imperative `transform: translate3d(..) scale(k)` and `clip-path: inset(..)` (`coverPlacement`) so it
  covers its cell; `inert` and `aria-hidden` unless it is the opened pane. A revealed pane (card hovered/focused) is **still inert**; it
  just shows clear.
- Opening a page (hash `#board`, click, Escape/Overview to close): cards and veil unmount, the pane loses `inert` and becomes a labelled
  region; **the other eight panes stay mounted**, inert. Each pane is wrapped in a `LayeredPaneBoundary` error boundary.
- `list` mode (<= 767 px x scale, or short): panes mount on demand (current + the next one), each `section` has its own veil and card.
- Veil: ONE `div.ui-veil` (`backdrop-filter`) with a `clip-path` hole, `z-30`; card overlay `z-[31]`, `pointer-events-none` with
  `pointer-events-auto` only on cards; the cards are wrapped in `display: contents` hosts (`pointerenter/leave` for mouse, `focus/blur`).

**Flag/context for "I am a backdrop"**: none. [C]

| Signal | What it is | Usable? |
|---|---|---|
| `PaneView { opened, revealed }` prop of every `*Page` | passed by `render(paneState)`; backdrop = `!opened && !revealed`; revealed = card hovered/focused (still inert) | yes, but only pages that receive the prop; `QuizPage`, `LeaderboardPage` and others take it, `PanePeers` uses `opened`. |
| DOM: `closest("[data-layered-pane]")`, `closest("[inert]")`, `aria-hidden` | `placePeers` already uses `closest("[inert]") === null` to ignore inert copies of an anchor | yes (precedent). |
| React context | not provided by `LayeredOverview` or `HomeScreen`; `PresenceContext` carries `view`, `showCursors`, `setTask` only | a new context set from `render` would be needed [D]. |

**What exists how often at home** [C]:

| Thing | Instances | Note |
|---|---|---|
| Page subtrees (`PageFrame`, `data-page`) | 9 (more with > 4 quizzes), always | the same page also appears as a compact card in the overlay; e.g. the preferences controls exist in the inert page and as a summary card |
| Compact cards + their hosts | 9, only while nothing is opened | `data-card`, `data-layered-card` |
| Veil / `ui-veil` | 1 | `backdrop-filter`; ~32 glass elements in total (prototype figure) |
| `HomeScreen` leaderboard poll, crowd refresh | 1 | lifted out of the panes on purpose |
| `PanePeers` (in-pane presence layers) | 7 (introduction, leaderboard, badges, 4 quiz pages) | each runs its own 250 ms interval + scroll/resize/focusin listeners while it has peers; renders nothing when `opened` or no peers |
| Infinite CSS task-icon loops (`.quiz-glyph[data-motion]`) | 9 under the veil (four quiz pages x their tasks) plus run/results | no backdrop gate |
| `PresenceOverlay`, `usePresencePointer`, `useDocumentVisible` | 1 each, at app level (`RX/🟦️.tsx:401,410,474`) | the pattern to copy |
| React `StrictMode` | wraps the whole app (`mountQuiz`) | effects run twice in dev; pet setup must be idempotent and clean up fully |

**Stacking map inside `.quiz-app`** [C]: pane content (`contain: layout paint`, clipped); `PanePeers` `z-10` (inside the pane); veil `z-30`;
card overlay `z-[31]`; Overview button `z-40`; `PresenceOverlay` `fixed inset-0 z-40`; skip link and modal `Dialog` backdrop `z-50`
(`fixed inset-0`, blurred); drag ghost `z-index: 1000`, `pointer-events: none`. A pets layer between 40 and 49 behaves like presence cursors:
over the cards, under dialogs. `position: fixed` inside a pane would be contained by the pane's transform and clipped [D].

**Side effects of backdrop pages**: pure views by rule ("showing, revealing or keeping one live never runs a command", `M/🏠️home` doc;
design §16). `QuizPage` does issue a read (`session.loadRun(latest)`) from its effect. Hover/focus never start a run (home-grid test and e2e
`🥞️layered-home` pin this). A pet layer must not call commands either.

**Existing tests that pin home DOM** (names to avoid, counts to keep): `[data-layered-pane]` count = pages; exactly one `[data-page="…"]`
per pane; `[data-peer][data-tag]` count 0 when nobody else is there; `[data-presence-anchor]` outside `[inert]`; `data-card`, `data-quiz-item`,
`data-quiz-drop`, `data-quiz-drag`, `data-crowd-*`, `data-connection-announcer`, `data-layered-*`, `data-overview-*` hooks
(`S/🧪️tests/🥞️layered-home/🟦️.ts:56-66,83,108-115`, `S/🧪️tests/👥️shared-presence/🟦️.ts:24,98`, `TK/🗒️ui-notes-for-e2e.md:34-40`). Use a fresh
prefix such as `data-pet-*`.

### 2.10 Closest precedents, in one place

| Precedent | What to copy | Where |
|---|---|---|
| Others' cursors | app-level fixed layer, `aria-hidden`, `pointer-events-none`, imperative `style.transform`, rect polling, `[inert]` exclusion, `data-shy` near focus, per-slot ink, forced-colors rule, `showCursors` preference | `M/👥️presence/🟦️.tsx:913-1031`, `RX/🎨️.css:316-367` |
| Task icons | CSS-only motion behind `@media (prefers-reduced-motion: no-preference)` and `.quiz-app:not([data-icon-motion="off"])`; preference default on; schema-optional field (domain-neutral product, site test requires it); test that asserts keyframes, gates and the preference | `RX/🎨️.css:39-72`, `T/🖼️task-icons/🟦️.tsx`, `TK/📓️task-icons.md` |
| LayeredOverview | pure geometry module + language-agnostic fixture + third-party oracles (`polygon-clipping`, `d3-ease`) + DOM cases + mutation checks (19/19 killed) | `UI/🔨️modules/🥞️layered-overview-geometry`, `TK/📓️layered-overview-report.md` |
| Domain neutrality | the product renders whatever the catalog says; site-specific content lives with the site (`🎓️teaching/🏛️architecture`) | `design.md` §1, §12; `TK/📓️task-icons.md:8` |

## 3. Presence (§15) and thinking room (§17)

### 3.1 State classification

| Class | Held where | Examples today |
|---|---|---|
| Persisted shared | proctor event store + projections | learner, run, answer, submission, badge events; `CrowdView`; leaderboard |
| Persisted local-only | browser `localStorage` | learner id, introduced flag, locale/theme/size/switches (`preferences`), cached catalog and learner view, outbox, cached runs |
| Ephemeral shared | polled from the proctor / presence sockets | leaderboard poll, roster, cursors, keyboard focus, drags, drafts (thinking) |
| Ephemeral local-only | the renderer | drag state, focus, current step, task on screen |

Sources: `design.md` §10 (185-189), `Q/README.md:312-319`. [S]

**Where pets would sit**: the *preference* is persisted local-only; position, pose, mood, fidgets are ephemeral local-only; the schema
has no place for a pet in any shared class. [D]

### 3.2 Rooms and wire shapes ([S] design §15/§17, [C] `M/👥️presence`, `Q/🧬️schema/🔣️.json:975-1054`)

- **Roster room** `<catalog>`: `PresenceState { tag, identity, place, active }` per session (changes on navigation and visibility; low rate).
- **Place rooms**: `<catalog>/introduction`, `/home`, `/leaderboard`, `/badges`, `/quiz/<quiz>` (page, run, results of one quiz); none for `identity`,
  `learner`, `preferences`, nor for a quiz place without its quiz. `CursorState { tag, cursor?, focus?, drag? { item } }`.
- **Thinking room** `<catalog>/quiz/<quiz>/thinking`: `ThinkingState { tag, answers }` (draft per task; matching in value form; <= 64 tasks / 64 entries).
- **Frames** (`semio.presence.v1`, text JSON): server->client `welcome { session, colour, roster }`, `batch { entries, left }` per tick, `watched { scope, entries, left, snapshot? }`,
  `refused { reason }`; client->server `state { state }` (<= 2 KiB, <= 30/s, excess dropped), `watch { scopes (<= 16), intervalMs }`.
  Session ids are server-random; `colour` is a palette slot; join/leave through the `Presence` registry; ping/pong, idle timeout 60 s;
  production requires the socket `Origin` to pass the allowlist; admission per scope (`PolicyPoint::Subscription` actions `join`/`publish`/`watch`).
- **Schemas are closed**: `Place`, `Cursor`, `PresenceState`, `CursorState`, `ThinkingState` all have `additionalProperties: false`; `Anchor` matches
  `^[a-z0-9]+(?:[:-][a-z0-9]+)*$`, <= 64 chars; `tag` is `^[0-9a-f]{8}$`.

### 3.3 Throttling and coalescing

| Where | Rule |
|---|---|
| Client send | latest state wins, never repeats the last sent, at most one frame per 67 ms (15 Hz) per room (`PRESENCE_FRAME_INTERVAL_MS`); thinking room 500 ms; watch interval 250 ms (`WATCH_INTERVAL_MS`, must be >= server tick) |
| Server | keeps latest state per session; publishes one `batch` per 100 ms tick of changed sessions plus leavers (O(changed) per client per tick) |
| Receive | pointer and focus are placed on anchors; others' cursors glide with a 120 ms CSS transition (cut by the reduced-motion rule); in watched rooms the update rate is 4 Hz |
| Reconnect | jittered backoff, longer while sessions keep ending in < 10 s, state and watch re-sent |

### 3.4 What cursor data exists

- Own pointer: `usePresencePointer(presence)` listens at document level (`pointermove` passive, `pointerdown` capture, `pointerleave`, window `blur`, `keydown`
  capture, `focusin/out`); it publishes `{ anchor, x, y }` **only while the pointer is over an element with `data-presence-anchor`**, rounded to 1e-4, nothing while
  the pointer leaves the page. Keyboard focus is shared only when moved by keyboard. [C] `M/👥️presence/🟦️.tsx:781-850`
- Others: `PresenceView.peers` (same-place room: `{ session, tag, label, colour, cursor?, focus?, drag? }`), `PresenceView.rooms` (by scope: the joined room plus, on
  home, every watched page/quiz room), `PresenceView.thinking`. Own sessions (any tab) are excluded. Peers are only present when the viewer is identified, the room
  is `open`, and (for drawing) `showCursors` is on. A peer's cursor is undefined while that peer's pointer is not over an anchor. Anchor keys: `home:<card>`,
  `quiz:<id>`, `quiz:<id>:tasks`, `leaderboard`, `badges`, `introduction`, `run`, `results`, `result:<task>`, `task:<id>`, `item:<id>`, `category:<id>`. [C] `M/👥️presence/🟦️.tsx:611-689,725-738`
- Colour per learner is one palette slot everywhere; the label is the identity display ("Anonymous #tag" for anonymous learners).

### 3.5 Decision aid: local vs shared pets, and looking at remote cursors

| Option | What it needs | Fits the documents? |
|---|---|---|
| Pets ephemeral local-only, **look at the own pointer** | one document-level `pointermove` listener (passive, rAF-coalesced) in the pets layer; touch needs a fallback (last touch point or idle gaze) | yes; no wire, server or privacy change |
| Pets ephemeral local-only, **look at remote cursors** | read `usePresenceView()`; convert `{ anchor, x, y }` to viewport pixels as `placePeers` does (anchor lookup outside `[inert]`, `getBoundingClientRect`); honour `showCursors`; expect 10-15 Hz in the joined room and 4 Hz in watched rooms, so interpolate; peers vanish on socket loss | yes; reuses data already shared and already consented to ("pointer ... is shown live") |
| Pets **ephemeral shared** (everyone sees the same pet interactions) | new `$defs` (closed schema) + `PresenceState`/`CursorState` or a new room kind in `roomScope`, `*Problem` validators in TS **and** Rust, conformance vectors from the Python/JSON-Schema oracle (`T/👥️shared-presence`, regenerate with `TK/generate_quiz_vectors.py`), proctor `presence_admission` (derived from the core), TS client, <= 2 KiB / 15 Hz, one more room within the 16-scope watch limit (home already watches 3 pages + N quiz rooms + N thinking rooms = 11 for four quizzes; from seven quizzes the list is silently cut, N6), privacy text in both languages, e2e on two devices | possible but the largest change; contradicts "decorative" |
| Shared *look* without shared *state* [D] | derive pet behaviour from a deterministic seeded function (FNV-1a / MT19937 of a shared value such as the learner tag or the place) so two clients show similar scenes without any frame | cheapest way to "consistent" pets; bit-exact PRNG already exists in both cores (`🎲️randomness`) |

## 4. Preferences contract

| Item | Value | Source |
|---|---|---|
| Module | `M/🎛️preferences/🟦️.tsx` (`QuizPreferences`, `readPreferences`, `writePreferences`, `PreferencesPanel`, `PreferencesCard`, `PreferencesPage`, `LanguageSwitch`, `LanguageChoice`) | [C] |
| Shape | `{ locale?: "en"\|"de"; theme: "system"\|"light"\|"dark"; textSize: "normal"\|"large"\|"larger"\|"largest" (scale 1 / 1.125 / 1.25 / 1.5); showCursors: boolean; showAnswers: boolean; animateIcons: boolean }` | [C] L41-49 |
| Defaults | locale: none until chosen; theme `system`; text `normal`; every boolean **true unless stored `=== false`** (`record.x !== false`) | [C] L52-63 |
| Storage | `localStorage["semio.quiz.<tenant>.preferences"]` (tenant `architecture`), JSON, one **slice**, last-writer-wins; slices: `introduced, learner, catalog, learner-view, preferences`; collections `outbox/<id>`, `runs/<id>` one key per record; failures (unavailable, full, corrupt) degrade to "nothing stored" | [C] `M/💾️persistence/🟦️.ts:108-181` |
| Cross-tab | other tabs' writes arrive through the `storage` event; `QuizApp` re-reads on `slice === "preferences"` or `cleared` | [C] `RX/🟦️.tsx:513` |
| Schema | **not in the normative JSON schema**; TS-only (grep of `🧬️schema` finds no preference field) | [C] |
| Effects | `locale` -> `applyLocale`, document `lang`, title; `theme` -> `useElementsSurfaceChrome({ appearance })`; `textSize` -> `--quiz-text-scale` on `<html>`; `animateIcons` -> `data-icon-motion="on\|off"` on `.quiz-app`; `showCursors` -> `PresenceProvider`/`PresenceOverlay`; `showAnswers` -> run/results/quiz-page crowd | [C] `RX/🟦️.tsx:451,474,505-509` |
| UI | three places: first-visit aside beside the introduction/identity (`PreferencesPanelCard`), the preferences card and page on home, the language switch in the navbar. Controls: `Segments` (`role=group`, `aria-pressed`) for language/theme/size; native checkboxes `.quiz-check` in `.quiz-target` labels for the three switches (labels "Show others' cursors", "Show what others think", "Animate task icons" and their German forms) | [C] L119-157, `M/🌐️i18n/🟦️.ts:100-103,436-439` |
| What a new switch costs | field in `QuizPreferences` + default in `readPreferences` + checkbox in `PreferencesPanel` + en/de label (and the stale `summary` sentence) + a root data attribute for CSS/JS gating + tests; the exact-object assertion `T/📡️presence-client/🟦️.tsx:839` and the literal `PREFERENCES` in `T/🏠️home-grid/🟦️.tsx:61` must be updated; storage of the raw object is rewritten whole on every change, so unknown stored keys are lost | [C] |
| Customization principle | theme and text size are CSS custom properties/root font size so the whole design system follows; larger text changes the home layout; no own contrast or reduced-motion switch (N8) | [S] `design.md` §10 (195), `audit-final-requirements.md:64,147-149` |

## 5. Testing contract

### 5.1 Layers that exist

| # | Layer | Where | Runner / command | Notes |
|---|---|---|---|---|
| 1 | Core unit twins | TS: `Q/🧪️tests/<case>/🟦️.ts` (list in `Q/🧪️tests/🎚️config/🟦️.ts`, vitest, node); Rust: `Q/🔨️modules/<m>/🧪️tests/🔬️unit/🦀️.rs` in crate `semio-framework-quiz` | `bun nx run @semio-tech/quiz:test`, `bun nx run @semio-tech/quiz-rs:test` | quick/long/exhaustive levels exist |
| 2 | **Language-agnostic Protocol v2 cases with third-party oracle** | `Q/🧪️tests/<emoji-kebab>/{🥒️.feature,🟦️.ts,🦀️.rs,🐍️.py}`; vectors `Q/🧫️fixtures/<case>/🔣️.json` generated from the Python reference by `TK/generate_quiz_vectors.py`; oracle registry `Q/🔮️oracles/🔣️.json` (schema v2; numpy, scipy, jStat, mathjs, ajv, python-jsonschema, colord, aria-query, i18next, lightningcss, lodash, d3-scale, opentype.js …; the design system's own registry `UI/🔮️oracles/🔣️.json` holds `polygon-clipping` for the layered element) | harness `parity exhaustive --owner ❓️quiz`, `contract --owner "🧰️framework/🛍️products/❓️quiz"` | features carry `@capability-… @oracle-… @comparison-… @id-… @level-… @mode-…`; a case name = one leading emoji + kebab-case, **sibling emojis must be unique**, generic emojis (`📄️`) are refused (renames in `ui-hardening-report.md:139`, `conformance-report.md:536`); `Q/README` says test helpers live inside the case |
| 3 | React component/journey tests (jsdom, Testing Library) | `Q/🧪️tests/<case>/🟦️.tsx`, **explicit include list** in `RX/🧪️tests/🎚️config/🟦️.ts` (17 files today) | `bun nx run @semio-tech/quiz-react:test` | oracles are dev dependencies of the package only, registered in the oracle registry; `T/🖼️task-icons` is the template for a decorative feature |
| 4 | Site node tests | `S/🧪️tests/{🧪️catalog,🧪️deploy,📰️host-document,🧱️local-stack}` + `🎚️config` | `bun nx run @teaching/architecture-quiz:test` | catalog + quizzes against draft-07 (ajv); deploy drift, CSP verifier with seeded violations, Dockerfile/compose/workflow parsing; host document (no `lang`, bilingual title, no extra inline block) |
| 5 | Playwright end-to-end gate | `S/🎭️e2e` + nine specs in `S/🧪️tests/` (`🚀️site-boot 🪪️first-visit 🥞️layered-home 🎯️quiz-runs 🏆️live-leaderboard 🗣️both-languages 📱️phone 👥️shared-presence 🔌️connection-shortage`), 19 tests per topology; topologies `dev` and `rehearsal` (**release build, CSP sealed, cross-origin, production-mode proctor**) | `bun nx run @teaching/architecture-quiz:test-e2e [-- rehearsal]` | throw-away stacks on 6161/8891 and 6162/8892; device guard fails on console/page error, failed request, status >= 400, WebSocket error, CSP violation |
| 6 | Release readiness gate | `deploy-check` (drift, `proctor check`, site artifact incl. CSP + budget, image build/check, stack check, e2e) | `bun nx run @teaching/architecture-quiz:deploy-check` | needs Docker |
| 7 | Mutation checks | temporary, restored byte-identical, reported (e.g. 19/19 layered element, 43/43 react, 12/12 presence) | by hand | part of what "done" meant for UI modules |
| 8 | Repo statutes | `bun ./📜️script.ts verify taxonomy report --scope "<dir>"`; new directories must be registered in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | | "clean=true errors=0 warnings=0" was the bar for `❓️quiz` and `🎓️teaching` |

Rules stated in `explore-test-conventions.md` and AGENTS: at least one language-agnostic test per feature, with the same output from at least one third-party
library; every third-party test dependency registered in the oracle registry (never a bare devDependency); no hand-written `project.json` per test case; temporary logs
prefixed `[DEBUG] `. [S] `TK/📓️explore-test-conventions.md:861-889`.

### 5.2 What "done" meant at each close (commands + counts)

| Close | Commands and results (as written) | Source |
|---|---|---|
| 2026-09-28 | `bun nx run-many -t test` for quiz, quiz-react, quiz-rs, proctor, site: 5/5 green; parity exhaustive `--owner ❓️quiz` 75/75; taxonomy clean (`🎓️teaching`, `❓️quiz`); Protocol v2 contract 0 breaches; `docker-image-build` + `-check`; `cargo test -p semio-framework-server` 89+5+3; browser walk (two origins as two devices) with no console errors | `closing-summary.md:27-37` |
| 2026-09-29 | 7 projects green (adds framework-server TS + Rust); parity 84/84 (11 cases); taxonomy clean (+ `🖥️server`); hub compiles; browser at desktop DE/dark and two devices; Docker image/stack checks; GHCR push and GitHub workflow **not run** | `closing-summary.md:75-83` |
| evening | 7 projects green; ui-react 962 pass / 22 known failures; play 44 pass / 3 pre-existing; demonstrator 19; parity 84/84; browser 1440x900 walk | `closing-summary.md:114-121` |
| night | 7 projects green (quiz-react 230); parity 93/93 exhaustive; taxonomy clean; hub compiles; two-device walk | `closing-summary.md:150-167` |
| 2026-10-02 hardening | quiz-react 364 (16 files), site 33; ui-react `test-exhaustive` 17 failed / 1027 passed (baseline 20/1024); typecheck exit 1 (133 pre-existing, 0 in quiz); taxonomy clean for `❓️quiz`, one unrelated error in `🎓️teaching` | `ui-hardening-report.md:20-31` |
| 2026-10-02 task icons (latest recorded) | `@semio-tech/quiz` quick 273/273; quiz-react 423/423; parity of the quiz owner 105/105 (one 103/105 run while another session regenerated fixtures); Rust crate 98/98; site 40/40; **not checked in a browser** | `TK/📓️task-icons.md:36-41` |
| dev-e2e, deploy-readiness | docs still carry placeholders: `RESULTS-PENDING` (`dev-e2e-report.md:6,90`), `TEST_RESULT`, `TAXONOMY_RESULT`, `GATE_RESULT`, `PROCTOR_TEST_RESULT` (`deploy-readiness-report.md:140,149-151`). **No document records the e2e gate or `deploy-check` as green after the hardening.** | |

### 5.3 Known flaky, slow or environment-bound areas

- E2E: ~7 min for both topologies; the longest test plays all four quizzes to a perfect score (2-3 min); two gate runs at once collide on fixed ports;
  more than four workers is slower; dependency cache must live in a `node_modules/.vite` directory; a full reload under test is avoided by freezing the dev server's watcher. [S] `dev-e2e-report.md:62-65,106-127`
- E2E drag helper has to follow the bin because crowd lines shift the layout (`dragInto`). [S] `dev-e2e-report.md:102-105`
- Parity can show 103/105 while another session regenerates fixtures. [S] `task-icons.md:39`
- `bun nx` sometimes fails before any task ("Plugin Worker did not receive a load message"); `NX_PLUGIN_NO_TIMEOUTS=true` avoids it. [S] `deploy-readiness-report.md:185-186`
- Windows: scripted rewrites of a file held by a running bun leave a stale tail; change sources with the editor tools only. [S] `deploy-readiness-report.md:182-184`
- `cargo` builds of large graphs on Windows (hub, play wasm) can die with `STATUS_NO_MEMORY` (PATH overflow); not relevant to the TS renderer. [S] `closing-summary.md:85-91`
- Dev proctor refuses the stale data folder `.🧬semio/🎓️teaching/proctor-dev` (v1 vs v2); `dev` fails until it is removed. [S] `ui-hardening-report.md:157-159`, `task-icons.md:41`
- `ui-react` has 17-22 known failing tests (Tree, shell layout, tokens, missing Playwright browser); `workspace:verify -- interactivity apps` reports 66 old errors; `:typecheck` of quiz-react exits 1 for generated-binding errors outside the quiz. A pets change in `UI/` must compare against these baselines. [S] `closing-summary.md:55-57`, `ui-hardening-report.md:25-28`
- The built-in Browser pane: `visibilityState: hidden`, rAF throttled; screenshots were not available; override `document.visibilityState` for debugging only. [S] `closing-summary.md:166-167`, `ui-hardening-report.md:40`

### 5.4 Open follow-ups and debt that can interact with pets

| Item | Why it matters for pets | Source |
|---|---|---|
| Layout shift when the crowd line appears/disappears (design decision of the client, unchanged) | pets standing on elements will be moved under; reserve space or follow rects | `dev-e2e-report.md:118-121` |
| I9 (German chip overflow), I10 (short viewports), I25 (labels over the focused control) implemented but **never measured** | pets add one more thing that can cover controls at 200 %/400 % zoom, landscape phones, "Largest" text | `ui-hardening-report.md:149-155` |
| N6: 16 watched rooms silently cut from seven quizzes | a shared pet room competes for the same budget | `audit-final-requirements.md:141-143` |
| N8: no own reduced-motion/contrast switch | the pets switch would be the second motion switch | `audit-final-requirements.md:147-149` |
| N5/N7/N8 (a11y): font is the display face Anta for all text, 28 tab stops in a 14-item sorting, touch users cannot peek | pets must not add stops; text in pets uses the same face | `audit-final-i18n-a11y.md:271-274` |
| D4: leaderboard cost grows with learners; 10 s poll by every home tab | CPU budget on the server side is already spoken for; pets must be client-only | `audit-final-requirements.md:112-118` |
| `site.legal.imprint/privacy` unset (owner step before public launch); GHCR push and Pages workflow never run | no effect on pets, but "ready to deploy" warns until set | `ui-hardening-report.md:160`, `S/README.md:131` |
| Parts of the stale `design.md` (§8/§9a, §14 budget) | do not copy the 414 kB figure; the enforced budget is in code | section 1.1 |
| No frame-time or idle-CPU threshold has ever been set | pets ticket has to define one and record the probe numbers | section 2.3 |
| Working tree is dirty; `QUIZ-SORTING-NUMERIC-GUESSES` is editing sorting/quantity/schema now | edit-by-merge only; avoid the same hunks in `RX/🎨️.css`, `M/🌐️i18n`, `RX/🟦️.tsx`, `M/🎛️preferences` | `git status` snapshot; `QUIZ-SORTING-NUMERIC-GUESSES/📓️design.md` |

## 6. Pets layer: MUST / MUST NOT / SHOULD (derived from the documents)

`[S]` stated, `[C]` code-confirmed, `[D]` my inference from the sources given.

### MUST

| # | Rule | Source |
|---|---|---|
| M1 | Mount the pets layer **exactly once per client at app level**, next to `PresenceOverlay`; never inside a `LayeredPane` page, a `PageFrame`, or any component that home renders nine times. | [C] `M/🏠️home/🟦️.tsx:147-168`, `LO:236,482`; [S] `layered-overview-report.md:16-17` |
| M2 | Be purely decorative: `aria-hidden="true"`, no focusable or tabbable descendant, no `role`, no live region, no accessible name leaking into controls. | [S] `design.md` §15; `audit-final-i18n-a11y.md:294`; `ui-reference-layered-landing.md:239` |
| M3 | Use `pointer-events: none` on the layer and everything in it (hover reveal, click-to-open, `elementFromPoint` drop test, Playwright). Pets are not clickable unless a separate decision adds a keyboard-operable control. | [C] `LO:439-442`, `M/🪟️chrome/🟦️.tsx:71-81`, `M/🤏️drag/🟦️.ts:14` |
| M4 | Stop all motion for `prefers-reduced-motion: reduce` (static pose or hidden), including JS/rAF/canvas/SMIL motion that the CSS transition rule does not cover, and react to the setting changing at runtime. | [S] `audit-final-requirements.md:62`; `audit-final-i18n-a11y.md:152`; [C] `RX/🎨️.css:39-72,410-416` |
| M5 | Ship an own persisted local-only switch (default per product decision) following the `animateIcons` pattern: `QuizPreferences` field, `readPreferences` default, checkbox, root data attribute as the single gate, en/de labels, refreshed `summary`, updated exact-shape test, new test. | [C] `M/🎛️preferences/🟦️.tsx:41-63,151-154`, `T/🖼️task-icons`, `T/📡️presence-client/🟦️.tsx:839` |
| M6 | Stay inside the release CSP: no `eval`, no inline `<script>`/`<style>` creation, no blob workers/images, no WebAssembly, no same-origin `fetch`/XHR, no remote origin; rig/animation data and art bundled and content-hashed under `assets/`; verify in the `rehearsal` e2e topology (dev has no CSP). | [C] `S/🏗️builder/🌐️vite/🟦️.ts:50-65`, `S/🚀️deploy/🟦️.ts:287-312`; [S] `ui-notes-for-deploy.md:5-25` |
| M7 | Keep the site artifact within `QUIZ_SITE_BUDGET` (260,000 B gzip JS, 60,000 B gzip CSS, 4,000,000 B total) and free of `[DEBUG]`, source maps and loopback strings; **measure first** (last recorded 182.1 kB gzip JS). | [C] `S/🚀️deploy/🟦️.ts:86,319-346` |
| M8 | Import only the slim `@semio-tech/ui-react/chrome` and `/i18n` subpaths and no external runtime library (workspace packages + react only); any third-party library only as a test oracle behind the registry. | [S] `design.md` §14 (289-290), `closing-summary.md:42-45`, `audit-agents-rules.md:80-82`, `explore-test-conventions.md:876-878` |
| M9 | Put every visible or spoken word (pet names, speech, labels, switch text) in **both** `en` and `de` (German informal "du"), in the client bundle or as `Text` content, with identical placeholders and no unused key; no words baked into art. | [S] `design.md` §10; `ui-hardening-report.md:85,123-126`; [C] `T/🗣️translation-completeness` |
| M10 | Add no tab stop and change no DOM/reading/tab order of any screen; the layer is a leaf outside the page flow. | [S] `ui-reference-layered-landing.md:247`, `design.md` §14 (283); `audit-final-i18n-a11y.md:273` |
| M11 | Not alter layout (no insertion into flow, no resizing of existing elements); follow elements by reading their rects, tolerate layout shifts, scroll in nested containers, resize, text-size and layout-mode changes (grid/tablet/list). | [S] `dev-e2e-report.md:102-105`; [C] `M/👥️presence/🟦️.tsx:913-998` |
| M12 | Never cover the control with keyboard focus (>= 24 px clearance) and never sit over modal dialogs: stay below z-50, expect to be made `inert` as a child of `.quiz-app`. | [S] `audit-final-i18n-a11y.md:263`, `ui-hardening-report.md:94`; [C] `M/🪟️chrome/🟦️.tsx:261-269` |
| M13 | Hold forced colors and contrast: every painted state gets a `@media (forced-colors: active)` rule plus a `F/🌗️contrast-states` entry (or the pets hide in forced colors); decorative text >= 4.5:1 in light and dark. | [S] `ui-hardening-report.md:52-53`; [C] `T/🌗️contrast-states/🟦️.tsx` |
| M14 | Follow theme and text size: colours from the design-system tokens/custom properties, sizes in rem. | [S] `design.md` §10 (195); [C] `RX/🎨️.css:7-9` |
| M15 | Pause when `document.visibilityState` is `hidden` and resume without a catch-up burst; no timer or rAF running for nothing. | [D] from `M/🏆️leaderboard/🟦️.tsx:93-129`, the idle contract in `ui-reference-layered-landing.md:251` |
| M16 | Never throw, log an error, or request a missing file: the e2e guard fails every spec on a console error, page error, failed request, status >= 400 or CSP violation. Wrap the layer in its own error boundary that fails silent. | [S] `dev-e2e-report.md:56-58` |
| M17 | Develop test-driven with at least one language-agnostic case (`Q/🧪️tests/<emoji-kebab>/🥒️.feature` + adapters) whose pure logic is compared with a third-party library registered in `Q/🔮️oracles/🔣️.json`, plus jsdom component tests listed in the react vitest config; register new directories in the taxonomy; unique sibling emoji. | [S] `explore-test-conventions.md:861-889`, `ui-hardening-report.md:137-139`, `conformance-report.md:536` |
| M18 | Keep the product domain-neutral: the generic layer and its extension seam live in the quiz product/target; the topic art and names (sun, cloud, house, solar panel, radiator, heat pump, window, wall, battery) are site/catalog content (domain-specific extension). | [S] `design.md` §1, §12; `TK/📓️task-icons.md:8` |

### MUST NOT

| # | Rule | Source |
|---|---|---|
| N1 | Instantiate pets, timers, listeners or rAF chains per backdrop pane or per card; do not rely on a "backdrop" flag (none exists). | [C] section 2.9 |
| N2 | Send anything new over the presence sockets, or learner ids, without schema-first changes in both cores, conformance vectors, proctor admission and a privacy sentence in both languages; do not widen a closed state (`additionalProperties: false`, 2 KiB). | [S] `design.md` §13, §15; `Q/README.md:245-273`; [C] schema |
| N3 | Start commands or other side effects from a decorative layer; backdrop pages are pure views and hovering never starts a run. | [S] `design.md` §16 (352), `ui-reference-layered-landing.md:239` |
| N4 | Rely on hover alone, or on a mouse; touch and keyboard must have an equivalent or simply no behaviour. | [S] `audit-final-i18n-a11y.md:149-151`, `ui-reference-layered-landing.md:247` |
| N5 | Carry meaning by colour, opacity or motion alone (nothing important may be told only by a pet). | [S] `audit-final-i18n-a11y.md:279`, `audit-final-requirements.md:119-122` |
| N6 | Use `position: fixed` or absolute geometry inside a pane (scaled, clipped, `contain: layout paint`), or put `data-card`, `data-page`, `data-peer`, `data-presence-*`, `data-quiz-*`, `data-crowd-*`, `data-layered-*`, `data-overview-*` on pet elements (tests count them). | [C] `LO:163,401`; `S/🧪️tests/🥞️layered-home/🟦️.ts`, `S/🧪️tests/👥️shared-presence/🟦️.ts:98` |
| N7 | Import the ui-react barrel, add an external runtime dependency, or leave `[DEBUG]` strings in shipped code. | [S] `design.md` §14, `closing-summary.md:42-45`; [C] `S/🚀️deploy/🟦️.ts:342` |
| N8 | Make the app depend on the proctor or the network for pets (first visit, identity screen, offline, rate-limited proctor all must look fine). | [S] `design.md` §10; [D] |
| N9 | Write `localStorage` outside the `semio.quiz.<tenant>.*` slices/collections, or store anything personal. | [C] `M/💾️persistence/🟦️.ts:153-181` |
| N10 | Rewrite or revert other agents' edits in the shared files; edit by merge, no modifying git commands. | [S] AGENTS; `closing-summary.md:48` (stash conflicts); dirty tree |

### SHOULD

| # | Rule | Source |
|---|---|---|
| S1 | Write transforms imperatively (no React render per frame), use one rAF chain that stops when nothing moves, coalesce input, and pause off-screen or occluded pets. | [S] `ui-reference-layered-landing.md:251`; [C] `LO:507-573` |
| S2 | Record an idle-CPU baseline with `TK/dev_e2e_idle_cpu.ts` (introduction, overview at rest, inside a run) before and after, and set a threshold (none exists); also re-run `publish` for the size. | [C] `TK/dev_e2e_idle_cpu.ts`; section 2.3 |
| S3 | Prefer deterministic, seeded pet simulation (pure functions of seed and time; FNV-1a/MT19937 precedent) so the logic is testable language-agnostically and cross-client consistency needs no wire. | [D] `design.md` §3 |
| S4 | Keep pets ephemeral local-only; let them look at remote cursors only through `usePresenceView()` and only when `showCursors` is on; handle peers vanishing. | [D] sections 3.4-3.5 |
| S5 | Follow anchors the way `placePeers` does (`data-presence-anchor`, skip `[inert]`, scroll capture + resize + `focusin` + interval, `data-shy` rule) instead of inventing a second layout observer. | [C] `M/👥️presence/🟦️.tsx:913-998` |
| S6 | Reduce or switch off pets in `list` mode / narrow or short viewports and at "Largest" text, honouring desktop > mobile > tablet; check at 375x812 touch. | [S] `audit-final-requirements.md:65`; [C] `M/🏠️home/🟦️.tsx:72-85` |
| S7 | Give pets their own stop/hide control (separate from "Animate task icons"), because auto-starting motion longer than five seconds needs a way to stop it. | [D] WCAG 2.2.2, not in the documents |
| S8 | Add the optional-field pattern if the catalog should choose pets per quiz (`TaskIcon` precedent: optional in the neutral schema, required by the site's catalog test, validated in both cores, sheet/views copy it). | [S] `TK/📓️task-icons.md:5-9` |
| S9 | Put the new normative text into `design.md` as a new section (it is the contract the next agent reads) and update `Q/README.md` and `S/README.md`. | [S] pattern of §14-§17 |
| S10 | Close the loop like the previous ticket: nx tests of the touched projects, `parity exhaustive --owner ❓️quiz` if a core changes, `verify taxonomy report` for `❓️quiz` and `🎓️teaching`, the e2e gate in both topologies, and a real-browser look at 1440x900, 768 and 375 in light and dark, English and German. | [S] `closing-summary.md:27-37,114-121,150-167` |

## 7. What I could not verify

- No code was run: no bundle size, idle CPU, test count or e2e result is measured here; every number is a recorded one with its date.
- Whether the e2e gate and `deploy-check` are green after the 2026-10-02 hardening: the reports still carry result placeholders.
- Whether the JS bundle is still under the 260,000 B gzip budget with today's tree.
- Behaviour of pets under `backdrop-filter` and nine live panes on real hardware (the audit itself says the live grid performance was judged from CSS and code only).
- WCAG 2.2.2 and the exact CSP treatment of module workers and WebAssembly are my reading of the platform rules, not statements of the documents.
- `QuizCardView` and `LearnerCard` were not read line by line; the instance counts in section 2.9 come from the home, chrome, presence and quiz-page modules and from the documents.
