# Quiz React target — map for the pets integration

Read-only exploration, 2026-10-02. Nothing outside this file was touched. Line numbers are from the working tree as it was when read; the quiz tree has many uncommitted edits by others (every file of the target is `M`, `🔨️modules/⚖️legal/` is untracked/new), so lines may drift.

Path aliases used below:

- `R` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`
- `Q` = `🧰️framework/🛍️products/❓️quiz`
- `UI` = `🧰️framework/🔨️modules/🖱️ui` (the design system, `@semio-tech/ui-react`)
- `SITE` = `🎓️teaching/🏛️architecture/❓️quiz` (the only site that mounts the client today)

---

## 0. Decision-relevant facts (read this first)

1. **Mount point.** `Client` in `R/🟦️.tsx:397-478` renders `PresenceProvider > div.quiz-app > [skip link, header, main, LegalFooter, PresenceOverlay]`. `<PetLayer>` belongs as a sibling right next to `<PresenceOverlay>` at `R/🟦️.tsx:474`, inside `PresenceProvider` (so it can call `usePresenceView()`), still inside `.quiz-app` (so it inherits `lang`, tokens, `data-*` hooks).
2. **z-order ladder (numeric, all in the root stacking context).** page content and navbar (auto / level-base 0) < `LayeredOverview` glass veil `z-30` < card layer `z-[31]` < overview "Overview" button `z-40` = `PresenceOverlay` `z-40` (DOM-later wins) < `Dialog` backdrop `z-50` and skip link `z-50` < drag ghost `z-index:1000`. A pet layer at `z-[35]`..`z-[39]` (or `z-40` rendered before `PresenceOverlay`) sits above cards and below dialogs. WindowChrome internals use `z-30` (title chip) and `z-[40]` (silhouette border, `UI/🧱️elements/🗂️WindowChrome/🟦️.tsx:259,346`); those are only compared in the root context if no ancestor makes a stacking context (a revealed card has `-translate-y-0.5`, which does). Verify visually.
3. **Dialogs make everything else `inert`.** `Dialog` (`R/🔨️modules/🪟️chrome/🟦️.tsx:259-269`) sets `inert` on every child of `.quiz-app` except its own backdrop. A `PetLayer` that is a direct child of `.quiz-app` becomes `inert` while a dialog is open (it keeps rendering, animations keep running; it just takes no input). `placePeers` already skips anchors inside `[inert]` (`R/🔨️modules/👥️presence/🟦️.tsx:924`); the home overview keeps all nine pages mounted but `inert` behind the glass (`UI/🧱️elements/🥞️LayeredOverview/🟦️.tsx:399`), so a surface query must also skip `[inert]` or it will find cards of hidden pages.
4. **No local pointer position is exposed.** `usePresencePointer` (`R/🔨️modules/👥️presence/🟦️.tsx:803-850`) only converts the pointer to an anchor-relative `Cursor` and hands it to `QuizPresence.point`. A pet layer needs its own passive `pointermove` listener (same pattern, ref + rAF, not React state). Remote cursors are available as `usePresenceView().peers` / `.rooms` (see §6).
5. **Tests that a new pet feature will touch (found by reading them):**
   - `Q/🧪️tests/🖼️task-icons/🟦️.tsx:49` asserts `css.match(/animation-name:/g)` has length `MOTIONS.length` (6). Any `animation-name:` added to `R/🎨️.css` breaks it (use the `animation:` shorthand, put pet CSS in another sheet, or amend the test).
   - `Q/🧪️tests/🗣️translation-completeness/🟦️.tsx:65-70` requires every key to be a literal `"quiz.<group>.<name>"` (exactly three segments, regex `/"(quiz\.[a-zA-Z]+\.[a-zA-Z]+)"/`) that appears in a client source file, and every used key to exist. A fourth segment or a computed key fails the "used / registered" check.
   - `Q/🧪️tests/📡️presence-client/🟦️.tsx:839` asserts the exact object `{ theme, textSize, showCursors, showAnswers, animateIcons, locale }` from `PreferencesPanel.onChange`; adding a `pets` field requires updating it. `Q/🧪️tests/🏠️home-grid/🟦️.tsx:61` has a typed `PREFERENCES` literal (typecheck breaks if `pets` is required).
   - New test files must be added to **both** `include` lists: `R/📦️packages/🟦️typescript/tsconfig.json:18-41` and `R/🧪️tests/🎚️config/🟦️.ts:32-50` (explicit file lists, no glob).
   - The translation test also pins `@semio-tech/ui-react` imports to exactly `/i18n` and `/chrome` (line 78-80), requires `role="list"` on every `ul/ol` with `list-none` (line 108-111), and scans `R/🔨️modules/**/🟦️.ts(x)` plus `R/🟦️.tsx`, so a new `🔨️modules/🐾️pets/` folder is scanned automatically.
6. **Release CSP of the site** (`SITE/🏗️builder/🌐️vite/🟦️.ts`, `quizContentSecurityPolicy`): `default-src 'self'; script-src 'self'+hashes; style-src 'self'+hashes; style-src-attr 'unsafe-inline'; img-src 'self' data:; font-src 'self'; connect-src <proctor>; object-src 'none'`. Inline `style=""` attributes are allowed; runtime-injected `<style>` tags, remote images/fonts and `eval` are not. Inline SVG, canvas, `data:` images and bundled assets are fine.
7. **No JS reduced-motion handling exists in the quiz target** (only CSS). `useMediaQuery` is exported by `@semio-tech/ui-react/chrome` and works with `"(prefers-reduced-motion: reduce)"`; `LayeredOverview` has its own private `useReducedMotion` (`UI/🧱️elements/🥞️LayeredOverview/🟦️.tsx:155,475`) that is not exported through `/chrome`.
8. **Design language is flat and sharp:** every `--radius-*` is `0rem`, every `--shadow-*` is transparent (`UI/🎨️styling/🖌️ui/🎨️.css:795-811`). Pets are the first rounded/organic thing on screen; that is a design choice, not something the system forbids.

---

## 1. Module inventory (`R/🔨️modules/*`, 26 folders, 7 749 lines in modules + 546 root + 466 css)

Each module is a single file `🟦️.ts` or `🟦️.tsx` (no module-local CSS, no module-local tests; tests live in `Q/🧪️tests`). All exports are re-exported (explicitly named) from `R/🟦️.tsx:49-167`.

| Module | Lines | Purpose | Main exports |
|---|---:|---|---|
| `↕️sorting` | 235 | Sorting task view: ordered `<ol>` with up/down buttons, grip drag, numeric guess field | `SortingTaskView, ordered, reordered` |
| `▶️run` | 244 | Run player: run card + current task card, progress, submit `alertdialog` | `RunScreen, TASK_KIND_ICONS, TaskGlyph, TaskView` |
| `⚖️legal` (new, untracked) | 131 | Footer + privacy dialog; site passes imprint/privacy URLs | `LegalFooter, PrivacyNotice, legalLink`, type `QuizLegal` |
| `🃏️matching` | 145 | Matching task: per-dimension selects + card pool, table rows are anchors | `MatchingTaskView, assignCard` |
| `🌐️i18n` | 764 | EN + DE chrome bundles registered on the shared i18n port; `quizText(locale)` | `QUIZ_BUNDLE_EN/DE, QUIZ_LOCALES, REJECTION_LABELS, TASK_KIND_LABELS, applyLocale, isQuizLocale, localized, preferredLocale, quizText`, types `QuizLabelKey/QuizLocale/QuizText` |
| `🎛️preferences` | 202 | Persisted local-only display preferences + the Settings card/page, language chooser | `QuizPreferences, readPreferences, writePreferences, textScale, THEME_CHOICES, TEXT_SIZES, PreferencesPanel, PreferencesPanelCard, PreferencesCard, PreferencesPage, LanguageSwitch, LanguageChoice, EveryLanguage, everyLanguage, LOCALE_NAMES` |
| `🏁️results` | 278 | Results screen: score card, new badges, one card per task with answer-vs-solution tables | `ResultsScreen, TaskResultView` |
| `🏅️badges` | 122 | Badges overview card, page, per-quiz earned badges | `BadgesCard, BadgesPage, awardOf, badgesEarnedIn` |
| `🏆️leaderboard` | 366 | Leaderboard card (excerpt) + page (sortable table), polling hook | `LeaderboardCard, LeaderboardPage, BOARD_EXCERPT_SIZE, LEADERBOARD_POLL_MS, POLL_BACKOFF_MAX_MS, boardExcerpt, ownRow, pollDelay, sortLeaderboard, usePolling` |
| `🏠️home` | 236 | The layered overview: nine pages behind glass, cards above | `HomeScreen, HOME_GRID_MIN_HEIGHT_PX, HOME_GRID_TRACKS, homeCells, homeLayoutQueries, homePages`, type `HomeLayout` |
| `👋️introduction` | 85 | First-visit screen, "How it works" card + page | `IntroductionScreen, IntroductionCard, IntroductionPage` |
| `👥️presence` | 1 088 | Shared presence: sockets, rooms, cursors, roster, overlay (see §6) | see §6 |
| `💾️persistence` | 187 | `StorageArea` seam over `localStorage`, tenant-prefixed typed store | `browserStorageArea, localStore, memoryStorageOrigin, localChange, isRecord`, types `LocalStore/StorageArea/LocalSlice/...` |
| `📇️profile` | 213 | Learner card, profile page (runs table, badges), switch-identity dialog | `LearnerCard, LearnerPage, SwitchIdentity` |
| `📏️quantity` | 191 | Locale-aware number/unit/score/date formatting | `formatQuantity, formatScore, formatPoints, formatNumber, formatDate, formatInstant, formatClock, engineering, parseQuantity, withUnit, SI_PREFIXES, SIGNIFICANT_DIGITS, oneDecimal` |
| `📖️quiz-page` | 208 | Quiz card on the overview + quiz page (tasks list, crowd) | `QuizCardView, QuizPage`, type `Act` (also `sheetOfQuiz`, not re-exported) |
| `📮️outbox` | 325 | Persisted answer outbox, retry/backoff, cross-tab merge | `Outbox, coalesce, coalescingKey` |
| `🕸️radar` | 525 | Owned-SVG spider diagrams + geometry | `RadarChart, radarLayout, radarPolygon, ...` |
| `🗂️classification` | 106 | Classification task: pool + category bins, chips are anchors | `ClassificationTaskView` |
| `🗳️crowd` | 198 | What the others think, per item (live or submitted) | `CrowdChoices, CrowdPosition, CrowdProvider, CrowdSource, chooseCrowd, crowdPlace, liveItems, meanPosition, submittedItems, useCrowd` |
| `🛂️proctor` | 416 | Typed proctor client over `@semio-tech/framework-server`, retry/rate-limit | `ProctorClient, proctorTransport, retryTransient, newId, commandEnvelope, ...` |
| `🤏️drag` | 86 | Pointer drag-and-drop (ghost clone, drop zones `[data-quiz-drop]`) | `startPointerDrag, dropZoneAt, DRAG_THRESHOLD_PX` |
| `🧩️task` | 103 | Shared pieces of task views: live region, focus-after-render, `DragGrip` | `TaskViewProps, DROP_ZONE_CLASS, SELECT_CLASS, ICON_BUTTON_CLASS, LiveRegion, useAnnouncement, useFocusAfterRender, DragGrip, elementId` |
| `🧭️session` | 753 | Event-folded client state + `QuizSession` controller (external store) | `QuizSession, QuizState, QuizStep, HOME_PAGES, evolveQuizState, initialQuizState, restoreQuizState, runAwards, openRunOf, lastSubmittedRunOf, mergeRunViews`, types |
| `🪟️chrome` | 321 | The quiz's card language on `@semio-tech/ui-react/chrome` | `QuizCard, PageFrame, CardIcon, CardAction, BodyButton, Glyph, Mark, Missing, Facts, Segments, Dialog, ProblemNote, textPresentation, cn` |
| `🪪️identity` | 221 | Identity step (anonymous / pseudonym / name) + shared name/notice helpers | `IdentityScreen, learnerName, noticeProblem, failureProblem, thrownProblem, handleFault, handleFaultMessage` |

The session is an external store: `QuizSession.getSnapshot` (`🧭️session/🟦️.ts:376`) / `subscribe` (379) return `{ state, connection }`; intents are `open(step)` (445), `startRun` (504), `answer` (534), `submit` (545). There is **no event stream** for the UI; `dispatch` is private. Pets that want to react to "answer given", "run submitted", "badge earned" must diff `state` (`state.step`, `state.awards`, `state.runs[run].answers/result`).

---

## 2. Root component tree and the overlay mount point

### 2.1 What `R/🟦️.tsx` exports

- Public seam: `mountQuiz(root, options): () => void` (`R/🟦️.tsx:534-545`) and `QuizApp(options)` (`:492-530`), `QuizOptions` (`:173-183`). Everything else is the re-export block (`:49-167`), plus `ConnectionStatus`, `connectionState`, `connectionMessage`, `waitingMessage`, `documentTitle`, `ConnectionTone`.
- Entry for the CSS: `import "./🎨️.css";` (`R/🟦️.tsx:47`). Package glue `R/📦️packages/🟦️typescript/🟦️.tsx` is `export * from "../../🟦️.tsx";` (2 lines).
- Imports from the design system, all via the slim `/chrome` subpath (`R/🟦️.tsx:19-32`): `DEFAULT_UI_DRIVER, Navbar, ShellBrandLogo, UI_AVAILABLE_HEIGHT, UI_MOBILE_MEDIA_QUERY, UI_TABLET_MEDIA_QUERY, bootstrapElementsSurfaceChromeDocument, elementsSurfaceDeviceForMatches, navbarFillItem, useElementsSurfaceChrome, useMediaQuery, type NavbarItem`.

### 2.2 Top-level tree

```
mountQuiz()                                  R/🟦️.tsx:534   readPreferences + bootstrapElementsSurfaceChromeDocument + applyLocale, then
  createRoot(root).render(<StrictMode><QuizApp {...options}/></StrictMode>)          :538-543
QuizApp                                      :492
  useSetup -> { store: localStore, presence: QuizPresence, session: QuizSession }    :480-488 (once, in useState)
  useSyncExternalStore(session.subscribe, getSnapshot)  -> { state, connection }     :499
  preferences state + store.watch re-read on "preferences" slice (other tabs)        :500, :513
  locale = preferences.locale ?? preferredLocale(options.languages ?? browser)       :501
  useElementsSurfaceChrome({appearance: preferences.theme, device, driver, ...})     :504   (sets .dark on <html>)
  useRootTextScale(textScale(preferences.textSize))  -> --quiz-text-scale on <html>  :505, :350-358
  if locale defined:  <Client .../>                                                  :519
  else:               <div.quiz-app><main><LanguageChoice/></main><LegalFooter/></div>   :520-529  (no providers, no presence)
Client                                       :397-478
  <PresenceProvider view showCursors setTask>                                         :450
    <div className="quiz-app flex min-h-0 flex-col overflow-hidden bg-background text-foreground"
         style={{ height: UI_AVAILABLE_HEIGHT }} lang={locale}
         data-icon-motion={preferences.animateIcons ? "on" : "off"}>                  :451
      <a href="#quiz-main" class="sr-only focus:not-sr-only ... focus:z-50">          :452-461  skip link
      <header className="shrink-0"><Navbar items=[brand, fill, connection, presence status, language switch]/></header>   :462-464
      <main id="quiz-main" tabIndex={-1} className="flex min-h-0 flex-1 flex-col outline-none">   :465
        {notice ? <ProblemNote/> : null}
        <Screen .../>                                                                 :471
      </main>
      <LegalFooter legal locale text/>                                                :473
      <PresenceOverlay view show={preferences.showCursors} itemLabel/>                :474
    </div>
  </PresenceProvider>
```

Verbatim, the end of `Client` (`R/🟦️.tsx:471-477`):

```tsx
          <Screen session={session} state={state} connection={connection} text={text} locale={locale} preferences={preferences} onPreferences={onPreferences} />
        </main>
        <LegalFooter legal={options.legal} locale={locale} text={text} />
        <PresenceOverlay view={view} show={preferences.showCursors} itemLabel={itemLabel} />
      </div>
    </PresenceProvider>
```

`Client` already computes everything a pet layer needs: `state` (step, catalog, learner, runs), `locale`, `text`, `preferences`, `visible = useDocumentVisible()` (line 401, a hidden tab), `at = presencePlace(...)`, `view = presenceView(shared, text)` (412).

### 2.3 Routing: pages are a state machine plus hash, not a router

- There is no router library. The screen is `state.step: QuizStep` (`R/🔨️modules/🧭️session/🟦️.ts:52`):
  `{screen:"introduction"} | {screen:"identity"} | {screen:"home"; page?: string} | {screen:"run"; run} | {screen:"results"; run}`.
- `Screen` (`R/🟦️.tsx:296-348`) is a `switch` on `step.screen`; every non-home screen is wrapped in `Page` (`:285-294`): `div.min-h-0.flex-1.overflow-auto.p-double > div.quiz-page.mx-auto.w-full.max-w-6xl` (`quiz-pair` when an aside exists on first visit).
- Home pages are addressed by hash and rendered by `LayeredOverview` with `routing="hash"`, `openedId={opened}`, `onOpenedIdChange={(page) => session.open(...)}` (`R/🔨️modules/🏠️home/🟦️.tsx:214-225`). `HOME_PAGES = { learner:"learner", introduction:"intro", leaderboard:"board", badges:"badges", preferences:"prefs" }` (`🧭️session/🟦️.ts:48`); a quiz page is addressed by the quiz id (`#heating`).
- Focus management: on a step change `main.querySelector("h1")?.focus()` (`R/🟦️.tsx:416-419`), keyed by `stepKey(step)` (`:242`, `run:<id>` / `results:<id>` / screen name). A pet layer must not steal focus and must not appear in the tab order.

### 2.4 Where `<PetLayer>` goes, and what it must know

Mount: between `LegalFooter` and `PresenceOverlay` (or right after `PresenceOverlay`) inside `.quiz-app`, e.g. `<PetLayer step={state.step} catalog={state.catalog} preferences={preferences} locale={locale} />`, `aria-hidden="true"`, `pointer-events-none fixed inset-0 overflow-hidden z-[35]`. This copies the proven recipe of `PresenceOverlay` (`R/🔨️modules/👥️presence/🟦️.tsx:1010`):

```tsx
<div ref={layer} aria-hidden="true" data-presence-layer="" className="pointer-events-none fixed inset-0 z-40 overflow-hidden">
```

Why `fixed inset-0` works: `.quiz-app` is `overflow-hidden` but has no transform/contain, so fixed descendants escape it; the document itself never scrolls (the `.quiz-app` height is `var(--ui-available-height, 100dvh)` and scrolling happens inside `Page` / `PageFrame` / `LayeredOverview` panes). So viewport pixels = layout of the app, and all geometry is `getBoundingClientRect()` in viewport pixels, re-measured on scroll (capture) / resize / focusin / timer, exactly as `usePlacing` does (`:971-998`).

Two `.quiz-app` roots exist (language-choice branch, `:520-529`, and `Client`); pets only make sense in `Client`.

`inert`/drag interplay: the drag ghost (`.quiz-drag-ghost`, `position:fixed; z-index:1000; pointer-events:none`, `R/🎨️.css:256-265`) is appended to `.quiz-app` and `dropZoneAt` uses `document.elementFromPoint` (`R/🔨️modules/🤏️drag/🟦️.ts:13-16`); a `pointer-events:none` pet layer is skipped by `elementFromPoint`, so it cannot break drops.

---

## 3. Props / config contract between site and app

### 3.1 `QuizOptions` (the whole seam, `R/🟦️.tsx:170-183`)

```ts
export interface QuizOptions {
  readonly proctor: string;        // proctor base URL; "" = the site's own origin
  readonly tenant: string;         // catalog id (also the localStorage namespace)
  readonly logo?: string;          // inline SVG markup for the navbar
  readonly legal?: QuizLegal;      // { imprint?: string; privacy?: string } absolute http(s) URLs
  readonly transport?: ProctorConnect;   // test seam: replaces the network
  readonly presence?: PresenceConnect;   // test seam: replaces the presence WebSockets
  readonly storage?: StorageArea;        // test seam: replaces localStorage
  readonly languages?: readonly string[];// test seam: replaces navigator.languages
  readonly timing?: RetryTiming;         // test seam: retry bounds
}
```

The site calls (`SITE/🟦️.ts:27`): `mountQuiz(root, { proctor: bakedProctorOrigin(), tenant: ARCHITECTURE_QUIZ_TENANT, logo, legal: site.legal });` where `ARCHITECTURE_QUIZ_TENANT = "architecture"` and `logo` is imported `?raw` from the emblem SVG. Branding = `logo` + the catalog title (shown in the navbar, `R/🟦️.tsx:421-431`); colours/fonts come entirely from the design system.

### 3.2 Languages

`LANGUAGES = ["en","de"]` (`Q/🧬️schema/🟦️.ts:31`), every learner-visible string is `Text = {en, de}` with **no default language**; `localized(text, locale)` = `text[locale]` (`🌐️i18n/🟦️.ts:39`). The locale is the stored explicit choice, else the first browser language the client speaks (`preferredLocale`, `:30`), else none (then only `LanguageChoice` + footer render, `R/🟦️.tsx:520-529`).

### 3.3 Catalog and quiz definitions (what the client receives)

The client never sees quiz sources; it gets the solution-free `CatalogView` from the proctor (`Q/🧬️schema/🟦️.ts:318-329`):

```ts
CatalogTaskView  = { id, kind: TaskKind, title: Text, icon?: TaskIcon }
CatalogQuizView  = { id: Slug, emoji: string, title: Text, description: Text, tasks: readonly CatalogTaskView[] }
CatalogBadgeView = { id, emoji, label: Text, description: Text }
CatalogView      = { id, title: Text, introduction: { title, paragraphs: Text[] }, quizzes: CatalogQuizView[], badges: CatalogBadgeView[] }
TaskIcon         = { emoji: string, motion: "bounce"|"pulse"|"spin"|"sway"|"float"|"flip" }   // :66, MOTIONS :46
```

Authoring side: `Quiz` (`:121-129`: `id, emoji, title, description, tasks`), `Catalog` (`:143-151`: `id, title, introduction, quizzes: string[] (paths), badges`). The architecture catalog is `SITE/🔣️.json`: four quizzes `physics 🧲`, `heating 🔥`, `cooling ❄️`, `demand 📊`, seven badges (`🧲 🔥 ❄️ 📊 🧮 🔍 🏁`); task icons `⚡ ☀️ 🔋 🧱 🔥 🌬️ ❄️ 🔌 🕸️` (+ motions).

`state.catalog` is `CatalogView | undefined` (loaded async, cached in localStorage slice `catalog`). Topic signals available to the client without any schema change: `quiz.id` (slug), `quiz.emoji`, per-task `icon.emoji`, per-item `id`/`label` in the sheet of a run (`RunView.sheet.tasks[].items[]`), and `state.step` / `presencePlace`.

Proctor config: not a prop. `proctor` is only the base URL; the client speaks the framework server contract through `ProctorClient` (`🛂️proctor/🟦️.ts`), `RETRY_TIMING` bounds, rate limits; tenant = catalog id.

### 3.4 Where a `pets` roster could come from (options, not a decision)

- **A. Site-supplied (matches `logo` / `legal`).** Add `readonly pets?: PetRoster` to `QuizOptions` (`R/🟦️.tsx:173`), pass it `Client` → `PetLayer`. Domain-neutral framework, domain-specific extension: the architecture site would pass sunny/cloudy/housy/solary/radiatory/pumpy/windowy/waly/battery, keyed by quiz id (`physics|heating|cooling|demand`) and/or task icon emoji. No wire/schema change; offline-safe. Roster must be `{en,de}`-localised if pets have names/tooltips.
- **B. Schema-first (data in the catalog).** Add an optional pet/companion field to `Quiz` and `CatalogQuizView` in `Q/🧬️schema/🔣️.json` (+ `🟦️.ts`, `🦀️.rs` twins, `🔮️oracles`, views in `Q/🔨️modules/👁️views`), so `state.catalog.quizzes[i].pet` flows through the proctor and the localStorage `catalog` cache. More work (Rust twin + conformance test) but makes pets topic-owned by the quiz authors.
- **C. Hybrid:** the roster (rig/skeleton definitions, behaviours) in the framework or site; the catalog only names which pet fits which quiz/task.
- Shared pets across learners (pets visible to others) would need new shared state; `CursorState` is a closed object: `object(state, "", report, ["tag"], ["cursor","focus","drag"])` (`Q/🔨️modules/✅️validation/🟦️.ts:568`), i.e. required `tag`, optional `cursor|focus|drag` (unknown keys are, by this allow-list shape, presumably refused — not tested). Local-only pets avoid this entirely.

---

## 4. Preferences module (`R/🔨️modules/🎛️preferences/🟦️.tsx`)

### 4.1 Existing preferences and their type

```tsx
export type ThemeChoice = "system" | "light" | "dark";                      // :18
export type TextSize = "normal" | "large" | "larger" | "largest";           // :21
export interface QuizPreferences {                                          // :42-49
  readonly locale?: QuizLocale;      // stored only once the learner chooses
  readonly theme: ThemeChoice;
  readonly textSize: TextSize;
  readonly showCursors: boolean;
  readonly showAnswers: boolean;
  readonly animateIcons: boolean;
}
```

Choice tables (the pattern a 3-way "pets" choice would follow), `:24-36`:

```tsx
export const THEME_CHOICES: readonly { readonly value: ThemeChoice; readonly label: QuizLabelKey }[] = [
  { value: "system", label: "quiz.preferences.themeSystem" }, ...
export const TEXT_SIZES: readonly { value; label; scale }[] = [ { value: "normal", label: "quiz.preferences.textNormal", scale: 1 }, ... ]
```

### 4.2 Persistence

`readPreferences` / `writePreferences` (`:52-68`):

```tsx
export function readPreferences(store: LocalStore): QuizPreferences {
  const stored = store.read("preferences");
  const record = isRecord(stored) ? stored : {};
  return {
    locale: isQuizLocale(record.locale) ? record.locale : undefined,
    theme: THEME_CHOICES.some((choice) => choice.value === record.theme) ? (record.theme as ThemeChoice) : "system",
    textSize: TEXT_SIZES.some((size) => size.value === record.textSize) ? (record.textSize as TextSize) : "normal",
    showCursors: record.showCursors !== false,
    showAnswers: record.showAnswers !== false,
    animateIcons: record.animateIcons !== false,
  };
}
export function writePreferences(store: LocalStore, preferences: QuizPreferences): void { store.write("preferences", preferences); }
```

- Storage: `localStorage`, single JSON value per tenant at key **`semio.quiz.<tenant>.preferences`** (e.g. `semio.quiz.architecture.preferences`); slices `introduced | learner | catalog | learner-view | preferences` (`R/🔨️modules/💾️persistence/🟦️.ts:110,155-161`, `prefix = \`semio.quiz.${tenant}.\``). It is "persisted local-only", last-writer-wins, shared by all tabs; other tabs' writes arrive via the `storage` event and `QuizApp` re-reads on `slice === "preferences"` (`R/🟦️.tsx:513`). Storage failures degrade to "nothing stored" (`persistence:24-73`).
- Defaults are chosen so unknown/missing keys behave as "on" for booleans (`!== false`) and a fixed fallback for enums. A new `pets` enum should default explicitly in `readPreferences` (e.g. `"calm"`), "slightly active by default".
- Application: `QuizApp` holds `useState(() => readPreferences(store))` and `change(next)` = `setPreferences(next); writePreferences(store, next)` (`R/🟦️.tsx:500,515-518`). Preferences reach the DOM through (a) `useRootTextScale` -> `--quiz-text-scale` on `<html>`, (b) `useElementsSurfaceChrome` -> `.dark` on `<html>`, (c) `data-icon-motion` on `.quiz-app` (`:451`), (d) `PresenceProvider showCursors` (`:450`). A pets preference would naturally become `data-pets="off|calm|lively"` on `.quiz-app` plus a prop to `PetLayer`.

### 4.3 UI exposure

`PreferencesPanel` (`:119-157`) renders three `Segments` rows (language, theme, text size) and three checkboxes (cursors, answers, motion). It is shown in three places: first-visit aside (`PreferencesPanelCard` in `Screen`, `R/🟦️.tsx:312`), the Settings card/page on home (`PreferencesCard`/`PreferencesPage`, anchor `home:preferences`, page `#prefs`), and tested directly in tests. Row markup to copy for a segmented row (`:131-136`):

```tsx
<div className={row}>
  <span className={name} aria-hidden="true">{text("quiz.preferences.theme")}</span>
  <Segments label={text("quiz.preferences.theme")} options={THEME_CHOICES.map((choice) => ({ value: choice.value, label: text(choice.label) }))} value={preferences.theme} onChange={(theme) => onChange({ ...preferences, theme })} />
</div>
```

`Segments` (`R/🔨️modules/🪟️chrome/🟦️.tsx:162-180`) = `role="group"` of `aria-pressed` buttons; the forced-colors rule `.quiz-app [aria-pressed=true]` already covers it.

### 4.4 Recipe: add `pets: "off" | "calm" | "lively"`

1. `🎛️preferences`: `export type PetMode = "off" | "calm" | "lively"`; `PET_MODES` table like `THEME_CHOICES` (labels `"quiz.preferences.petsOff" | "petsCalm" | "petsLively"`, plus `"quiz.preferences.pets"` for the row label); add `readonly pets: PetMode` to `QuizPreferences`; add `pets: PET_MODES.some(...) ? record.pets as PetMode : "calm"` in `readPreferences`; add a `Segments` row in `PreferencesPanel`; extend `quiz.preferences.summary` text.
2. `🌐️i18n`: keys in **both** `QUIZ_BUNDLE_EN` (`:85-104`) and `QUIZ_BUNDLE_DE` (`:421-440`) (see §5).
3. `R/🟦️.tsx`: `data-pets={preferences.pets}` on `.quiz-app` (`:451`) and `<PetLayer mode={preferences.pets} .../>`.
4. Tests to touch: `Q/🧪️tests/📡️presence-client/🟦️.tsx:839` (exact `onChange` object), `Q/🧪️tests/🏠️home-grid/🟦️.tsx:61` (`PREFERENCES`), and a new test for the preference (pattern: `Q/🧪️tests/🖼️task-icons/🟦️.tsx:53-64`: `localStore(memoryStorageOrigin().tab(), "arch")`, `store.write("preferences", {...})`, `readPreferences`, `render(<PreferencesPanel .../>)`, `getByRole("checkbox", {name})`, `fireEvent.click`).

---

## 5. i18n module (`R/🔨️modules/🌐️i18n/🟦️.ts`)

- Strings are declared as nested objects of `phrase(normal, beginner = normal)` leaves (`:53`, shape `{label:{normal, beginner}}`), under a single root key `quiz` with groups: `app, nav, connection, preferences, introduction, identity, home, learner, quizPage, task, run, classification, sorting, matching, radar, results, leaderboard, crowd, presence, legal, rejection` (EN `:56-389`; DE `:392-725`).
- `QUIZ_BUNDLE_DE: typeof QUIZ_BUNDLE_EN` (`:392`) — the type forces identical keys; `QuizLabelKey = DeepUiTranslationKeys<typeof QUIZ_BUNDLE_EN>` (`:728`, dot paths like `quiz.run.submit`).
- Registered on the shared i18next port: `registerUiTranslationBundles({ en: { translation: QUIZ_BUNDLE_EN }, de: { translation: QUIZ_BUNDLE_DE } })` (`:730`; `UI/🎯️targets/⚛️react/🌐️i18n/🟦️.ts:2117`). Only `en`/`de` are accepted by the registration type (`UiLocale`), and there is no default language (`quizText(locale)` passes `lng: locale` explicitly, `:761-763`; `QUIZ_LOCALES` order = EN, DE).
- Usage: `const text = quizText(locale)` (memoised in `Client`, `R/🟦️.tsx:399`), `text("quiz.run.progress", { done, total })`; placeholders are `{{name}}`. Counts are written "Label: n" so no plural forms exist (enforced by test).
- Learner content (titles, prompts, labels) is data `Text {en,de}`, not chrome keys; pet **names/tooltips/speech** that are not data from the catalog would be chrome keys; pet content from a roster must be `Text`.
- **Completeness test** `Q/🧪️tests/🗣️translation-completeness/🟦️.tsx`: same keys EN/DE (`:33-36`); non-empty labels and identical placeholders (`:38-45`); each key resolves via the port in both locales (`:47-57`); keys used in sources == registered keys, where "used" is the regex `/"(quiz\.[a-zA-Z]+\.[a-zA-Z]+)"/` over `R/🟦️.tsx` and `R/🔨️modules/**/🟦️.ts(x)` loaded with `import.meta.glob(..., {query:"?raw"})` (`:28,65-70`); German is informal "du" (no `Sie/Ihr...`, `:72-76`); single German words per concept (`:115-121`); `ui-react` imports only `/i18n` and `/chrome` (`:78-80`).
- **To add strings:** add the key to EN and DE under an existing group (three segments only, letters only in each segment, `quiz.<group>.<name>`), reference it as a literal in a client source file (not built dynamically — the test cannot see computed keys, and an unused key fails), keep German informal and placeholders identical. For a new group (e.g. `pets`) add it to both bundles; `quiz.pets.<name>` is fine, `quiz.pets.mood.calm` is not.
- Pet-related a11y text candidates: pets layer is decorative (`aria-hidden`), so likely only the preference labels (`petsOff/petsCalm/petsLively`, row label, summary) need keys.

---

## 6. Presence module (`R/🔨️modules/👥️presence/🟦️.tsx`, 1 088 lines)

### 6.1 Layers

1. **Sockets/rooms** (`:55-371`): `PresenceRoom<S, W>` over one WebSocket; JSON frames `semio.presence.v1`; state frames throttled to `PRESENCE_FRAME_HZ = 15` (`PRESENCE_FRAME_INTERVAL_MS = 67`), latest state wins (`publish` → `schedule`/`flush`, `:202,346-363`); drafts at ≤ 2 Hz (`THINKING_FRAME_INTERVAL_MS = 500`); watched rooms report every `WATCH_INTERVAL_MS = 250` (about 4 Hz), at most `MAX_WATCHED_ROOMS = 16`; rejoin with jittered backoff (`rejoinDelay`, `:78`).
2. **`QuizPresence`** (`:500-605`): joins the roster room (catalog id), the room of the current place (`roomScope(catalog, place)`; none for identity/learner/preferences), the thinking room of an open run. `update(input)` is called from `Client` in an effect (`R/🟦️.tsx:409`), `point(cursor|undefined)`, `focusOn(anchor)`, `drag(item)`, `stop()`. External store: `subscribe`/`getSnapshot` (`:513-518`).
3. **View** (`:608-689`): `presenceView(snapshot, text): PresenceView`.
4. **React** (`:741-1087`): `PresenceProvider` / `usePresenceView` / `usePresenceTask` / `useDocumentVisible` / `usePresencePointer` / `PresenceOverlay` / `PanePeers` / `PresenceStatus` / `PresenceList` / `OnlineMark`.

### 6.2 Coordinate system and anchors

A cursor is **relative to an anchor element's box**, not to the viewport — `Cursor = { anchor: string; x: 0..1; y: 0..1 }` (`Q/🧬️schema/🟦️.ts:412`), built by (`:781-789`):

```tsx
export function cursorAt(anchor: HTMLElement, x: number, y: number): Cursor | undefined {
  const key = anchor.dataset.presenceAnchor;
  const box = anchor.getBoundingClientRect();
  if (key === undefined || box.width <= 0 || box.height <= 0) return undefined;
  const unit = (value: number): number => Math.round(Math.min(1, Math.max(0, value)) * 10_000) / 10_000;
  return { anchor: key, x: unit((x - box.left) / box.width), y: unit((y - box.top) / box.height) };
}
```

The anchor is the nearest `[data-presence-anchor]` ancestor of the event target (`anchorOf`, `:791`), so a pointer over unanchored chrome sends `cursor: undefined`. Anchor keys (`PRESENCE_ANCHORS`, `:725-738`): `introduction`, `leaderboard`, `badges`, `run`, `results`, `home:<card>` (built as `PRESENCE_ANCHORS.home(card)` with card = `learner`, `quiz:<id>`, `introduction`, `board`, `badges`, `preferences`, giving e.g. `home:learner`, `home:quiz:heating`, `home:board`), `quiz:<id>`, `quiz:<id>:tasks`, `task:<id>`, `result:<id>`, `item:<id>`, `category:<id>`. Items and categories are stamped on the task row elements (`data-presence-anchor={PRESENCE_ANCHORS.item(id)}` on `<li data-quiz-drag data-quiz-item>` in sorting `:208`, classification chips `:51`, bins `:91`, matching rows `:105`). Because the key is semantic (`item:tea-light`) it survives other viewports and shuffled orders; two learners at the same place resolve to different pixels.

`QuizPresence.point` de-duplicates identical cursors (`:544-548`); the room throttles to 15 Hz.

### 6.3 Local pointer stream: not reusable, easy to duplicate

`usePresencePointer(presence)` (`:803-850`, called at `R/🟦️.tsx:410`) registers on `document`/`window`: passive `pointermove`, capture `pointerdown`, `pointerup`/`pointercancel`, `pointerleave` on `<html>`, `blur`, capture `keydown`, `focusin`/`focusout`. It only forwards to `presence.point/drag/focusOn`; nothing holds the raw `clientX/clientY`. Signature: `export function usePresencePointer(presence: QuizPresence): void`. A pet layer therefore needs its own listener (cheap, same events, store `{x,y,pointerType,at}` in a ref; use `pointerType === "mouse"` vs touch because touch has no hover pointer; clear on `pointerleave`/`blur` like `leave`).

### 6.4 Remote cursors: reusable

```tsx
export interface PeerCursor { session; tag; label; colour: number; cursor: Cursor | undefined; focus: Anchor | undefined; drag: string | undefined }   // :611
export interface PresenceView { me; roster: PresenceRoster | undefined; colours: ReadonlyMap<string, number>;
  peers: readonly PeerCursor[];                                  // this place's room, own sessions removed
  rooms: ReadonlyMap<string, readonly PeerCursor[]>;             // every joined/watched room by scope (home watches the pages behind the cards)
  thinking: ReadonlyMap<string, readonly ThinkingState[]> }      // :624
export function usePresenceView(): PresenceView                  // :757 (reads PresenceContext)
export function PresenceProvider(props: { view; showCursors?: boolean; setTask; children }): ReactElement   // :752
```

- `peers[i].cursor` is `{anchor, x, y}` = anchor-relative; to get pixels use the same lookup as `placePeers`: first `[data-presence-anchor="<css-escaped key>"]` element outside `[inert]`, then `box.left + x*box.width` (`:918-941`, `anchorSelector` `:884`).
- `view.colours` maps learner tag -> roster palette slot; `presenceColor(slot, appearance)`, `presencePaint(slot, appearance)` (from `/chrome`) and `paintStyle(slot)` / `peerInk(slot, appearance)` (`:866,880`) give the learner colour and a readable ink; pets could be tinted/accessorised per learner.
- Remote cursor data is empty when nobody else is online, when `showCursors` is off (context only; `PresenceContext` itself is not exported, so a hook cannot read `showCursors` — `Client` has `preferences.showCursors` directly), when the place has no room (identity, learner page, preferences), or before the roster room is `open`.
- Pets reacting to other learners ("a friend arrived") can use `view.roster?.online`, `view.roster.learners[].place`, `view.peers.length`.

### 6.5 Painting, z-index, pointer-events

- `PresenceOverlay` (`:1003-1014`): returns `null` unless `show && view.peers.length > 0`; otherwise `div[data-presence-layer][aria-hidden]` `pointer-events-none fixed inset-0 z-40 overflow-hidden` with `PeerMarks` (`:943-969`): per peer `div.quiz-peer[data-peer="cursor"][data-anchor][data-x][data-y][hidden]` (16×16 SVG arrow + `span.quiz-peer-label`) and optional `div.quiz-peer-focus[data-peer="focus"]`.
- Positioning is **imperative**: `placePeers(layer, within?)` writes `mark.style.transform = translate(left, top)` (and width/height for focus frames) after each React layout (`usePlacing`, `:971-998`: `useLayoutEffect` every render + `resize`, capture `scroll`, `focusin`, an interval of 500 ms (250 ms for panes), coalesced into one `requestAnimationFrame`, with a `typeof requestAnimationFrame !== "function"` fallback for jsdom). Marks glide through CSS `transition: transform 120ms linear` (`R/🎨️.css:318-325`), i.e. the 15 Hz frames look smooth without JS tweening, and `prefers-reduced-motion: reduce` collapses it to 0.01 ms (`:410-416`).
- Label avoidance: `data-shy` hides a label that would cover the focused control (`PEER_LABEL_CLEARANCE_PX = 24`, `:889-911`).
- `PanePeers` (`:1019-1031`): per-page layer `pointer-events-none absolute inset-0 z-10` inside `PageFrame` for the pages behind the overview; positions in the pane's own unscaled pixels (`within`), so marks follow the scaled page.
- `placePeers`/jsdom: tests fake layout with a `box()` helper (`Q/🧪️tests/📡️presence-client/🟦️.tsx:740-755`) because jsdom rects are zero; `cursorAt` returns `undefined` for zero-size boxes. Pet geometry code must tolerate all-zero rects.

---

## 7. CSS (`R/🎨️.css`, 466 lines; plus the design system)

### 7.1 Structure and loading

One stylesheet for the whole target, imported by the root component (`R/🟦️.tsx:47`) and exposed as `@semio-tech/quiz-react/🎨️.css` (`package.json` exports). Header comment says its job: "only holds what utilities cannot say" — almost all styling is Tailwind utilities with design-system tokens inline in the TSX (`p-double`, `gap-single`, `text-muted-foreground`, `border-normal`, `bg-active-base`, `size-workbench`, `h-large`, ...). Modules have **no** CSS of their own. The site CSS (`SITE/🎨️.css`) is `@import "…/🖱️ui/🧵️styles/🎨️.css"` (Tailwind v4 + palette + tokens + globals) and `@source "…/❓️quiz/🎯️targets/⚛️react"`: **every `.tsx` under the target is scanned for Tailwind classes**, so a new module's utilities work without extra config; a pure `.css` addition needs to be in `R/🎨️.css` (or imported by it).

Sections in order: root font size from `--quiz-text-scale` (`:7-9`) · `.quiz-app` wrapping/hyphens (`:11-18`) · `.quiz-nowrap/.quiz-name` (`:22-31`) · `.quiz-glyph` (`:34-37`) · task-icon motion block (`:41-72`, six `@keyframes quiz-icon-*` `:74-153`) · `.quiz-target` 24 px minimum (`:156-158`) · home grid `.quiz-home-grid/.quiz-home-cell` (`:165-188`) · `.quiz-page/.quiz-pair` (`:191-203`) · `.quiz-me/.quiz-alert/.quiz-note/.quiz-progress/.quiz-radio/.quiz-check/.quiz-input/.quiz-backdrop` · drag (`.quiz-drag-ghost/.quiz-dragging/.quiz-drop-active`, `:256-275`) · radar (`:278-314`) · peers (`:318-367`) · crowd (`:370-387`) · `.quiz-online` · `.quiz-card-link` · `.quiz-footer` · reduced motion (`:410-416`) · forced colours (`:420-466`).

### 7.2 Naming convention

Class prefix `quiz-`, kebab-case, block-element by hyphen (`quiz-peer`, `quiz-peer-label`, `quiz-peer-focus`, `quiz-radar-ring`, `quiz-crowd-marker`), state/variant by **data attributes** not modifier classes (`data-motion`, `data-icon-motion`, `data-peer`, `data-shy`, `data-revealed`, `data-state="earned|locked"`, `data-earned`, `data-used`, `data-tone`, `data-presence-anchor`, `data-quiz-drag/-item/-drop/-grip`, `data-card`, `data-page`), custom properties `--quiz-*` (`--quiz-text-scale`, `--quiz-peer`, `--quiz-peer-ink-light|dark`, `--quiz-crowd-at`). Keyframes `quiz-icon-<motion>`. Every rule has a leading `/* <emoji> … */` comment.

### 7.3 Tokens (from the design system, not from this sheet)

Defined in `UI/🎨️styling/🖌️ui/🎨️.css`:
- Colours (semantic aliases, `:90-124`): `--base, --panel, --foreground, --muted, --muted-foreground, --accent, --active-base, --active-foreground, --active-hover, --hover-interactive-fill, --border-normal-color, --border-emphasized-color, --border-element-color`; light on `:root`, dark on **`.dark`** (a class on `<html>`; `useElementsSurfaceChrome` sets it; `appearance()` in presence reads `document.documentElement.classList.contains("dark")`, `:852`). Also `--color-warning`, `--color-secondary` (used in `.quiz-alert/.quiz-note`).
- Spacing: `--ui-spacing` (compact or touch variant), `--spacing-single = 1×`, `--spacing-double = 2×` (`:755-757`); Tailwind `p-double`, `gap-single`.
- Strokes `--stroke-hairline 1px`, `--stroke-default 2px`, `--stroke-focus 3px` (`:792-794`).
- Radii all `0rem`, shadows all fully transparent (`:795-811`).
- z-scale: `--z-base 0, --z-window 10, --z-pane 20, --z-panel 30, --z-dialog 40, --z-menu 50, --z-navbar 100, --z-modal 1000, --z-tutorial 10000` (`:856-865`) with utilities `z-base … z-tutorial` (`:6996-7034`). The quiz target does **not** use these tokens: it uses raw numbers that happen to coincide (`z-40` PresenceOverlay = `--z-dialog`; `z-50` Dialog backdrop = `--z-menu`; drag ghost `1000` = `--z-modal`; `PanePeers z-10`).
- Glass/veil knobs: `--glass-saturate`, `--glass-alpha-step`, `--glass-blur-step`, `--veil-alpha`, `--veil-blur` (`:866-878`); `.quiz-backdrop` uses `color-mix(in srgb, var(--base) 60%, transparent)` + `blur(0.5rem) saturate(1.45)`.
- Motion: no dedicated duration/easing tokens found in the main stylesheet (grep for `--duration`, `--ease`, `--motion` returned nothing); the target uses literals (`120ms linear`, `3.2s ease-in-out infinite`, Tailwind `duration-200 motion-reduce:transition-none` in `QuizCard`, `R/🔨️modules/🪟️chrome/🟦️.tsx:64`).
- Device breakpoints: `UI_MOBILE_MAX_WIDTH_PX = 767`, `UI_TABLET_MAX_WIDTH_PX = 1023` (`UI/📱️device/🟦️.ts:26,31`), queries `UI_MOBILE_MEDIA_QUERY`, `UI_TABLET_MEDIA_QUERY` (exported via `/chrome`). Home measures them in the learner's text size (`homeLayoutQueries`, `🏠️home:78-85`).

### 7.4 Dark / light / contrast / reduced motion

- Dark/light: one class switch on `<html>` (`.dark`), tokens re-resolve; presence palettes are computed per appearance (`presencePaint(colour, appearance())`) and name-label ink for both appearances is precomputed as `--quiz-peer-ink-light|dark` (`:880-882`), picked by `.dark .quiz-peer-label` (`R/🎨️.css:355-357`). Bootstrapped before first paint in `mountQuiz` (`R/🟦️.tsx:536`).
- Contrast: WCAG ≥ 4.5 : 1 for peer labels is **tested** (`Q/🧪️tests/🌗️contrast-states`, oracle `colord`); no `prefers-contrast` rules exist in the target (grep: none).
- Forced colours: `@media (forced-colors: active)` block (`:420-466`) + fixture `Q/🧫️fixtures/🌗️contrast-states/🔣️.json` listing each state/selector/properties the test must find (`forcedColors[]`). **Any pet visual that carries meaning through a background/custom colour would need an entry there** (or be pure decoration, `aria-hidden`, not state).
- Reduced motion (CSS only): `@media (prefers-reduced-motion: no-preference)` gates the icon animations and also requires `.quiz-app:not([data-icon-motion="off"])` (`:41-72`); `@media (prefers-reduced-motion: reduce)` sets `transition-duration: 0.01ms !important` on `.quiz-app *, ::before, ::after` (`:410-416`) — it does **not** touch `animation-*`. Tailwind `motion-reduce:` variants are used on cards. The user-level switch for motion is `animateIcons` (label "Animate task icons"), applied through `data-icon-motion`.

---

## 8. DOM landmarks (walkable surfaces)

### 8.1 Universal building blocks

- **Card** = `QuizCard` (`R/🔨️modules/🪟️chrome/🟦️.tsx:24-82`) → `OverviewCard as="section"` (`UI/🧱️elements/🃏️OverviewCard/🟦️.tsx:62-121`) → root element
  `<section data-overview-card="" data-card="<kind>" data-presence-anchor="<anchor>" data-revealed? aria-labelledby=<heading id> class="group min-h-0 min-w-0 …">`
  containing the `WindowChrome` stack (`UI/🧱️elements/🗂️WindowChrome/🟦️.tsx:395+`): `div[data-slot="quiz-card-stack"][data-overview-card-stack][data-level="dialog"]` > title cap `div[data-slot="quiz-card-title-chip"]` (the "chip" on the card's top edge: icon + heading `h1|h2|h3`) > body `div[data-slot="quiz-card-content"]` (padding `p-double`, `flex flex-col gap-double`) > footer `div[data-slot="window-chrome-footer"]` with `window-chrome-footer-left|right` chips holding the `OverviewCardAction` buttons (`button[data-overview-card-action="primary|secondary"]`, min-height 24 px).
  The top of a card is irregular: the title chip occupies only the top-left (`windowChromeTitleChipClass`, `relative z-30`), the rest of the top edge is a transparent gap (`data-slot="window-chrome-gap"`), and the silhouette border is drawn by a `pointer-events-none z-[40]` SVG-ish overlay. For "walk on top of a card", use `section[data-overview-card]` `getBoundingClientRect().top` as the ground and the title chip rect (`[data-slot="quiz-card-title-chip"]`) as a raised platform.
- Clickable cards on the overview additionally get `pointer-events-auto cursor-pointer transition-transform duration-200` and, when revealed, `-translate-y-0.5` (a 2 px lift on hover/focus, `QuizCard` line 64) — surfaces move by 2 px; measure per frame or on hover.
- **Buttons**: `OverviewCardAction`, `BodyButton` (`button.quiz-target … border border-normal`), `Segments` buttons (`aria-pressed`), nav-step buttons in the run card (`nav > ol > li > button[aria-current="step"].quiz-target`), icon buttons `.quiz-target … border` (up/down), `LegalFooter` link-buttons (`footer.quiz-footer`), Navbar items.
- **Standard landmarks outside cards:** `header.shrink-0 > nav#ui.navbar[data-slot="navbar"][aria-label]` (`relative h-large` plus the base level z class, `UI/🧱️elements/🔝️Navbar/🟦️.tsx:226`), `main#quiz-main`, `footer.quiz-footer` (`border-t`, last row; `R/🔨️modules/⚖️legal/🟦️.tsx:102`), skip link `a[href="#quiz-main"]`.
- **Scroll containers:** non-home screens: `div.min-h-0.flex-1.overflow-auto.p-double > div.quiz-page` (`R/🟦️.tsx:285-294`); pages behind overview: `div.quiz-page-frame[data-page="<page>"].overflow-auto` (`PageFrame`, `chrome:96-105`); leaderboard card body `div.min-h-0.flex-1.overflow-auto`; tables `div.max-w-full.overflow-x-auto`. Pets standing on elements inside a scroller must re-measure on `scroll` (capture) and be clipped to the scroller's rect.

### 8.2 Home (overview)

Path: `main#quiz-main > div.quiz-home.flex.min-h-0.flex-1.flex-col > [h1.sr-only] [ProblemNote?] [div[role=status] progress while starting a run] > div.relative.min-h-0.flex-1 > div[data-layered-overview][data-mode="strip"|"list"][data-rest="grid"]` (`UI/🧱️elements/🥞️LayeredOverview/🟦️.tsx:851`; `R/🔨️modules/🏠️home/🟦️.tsx:200-235`):

- `div[data-layered-strip]` (`contain: layout paint`) with one `div[data-layered-pane="<page>"]` per page (`aria-hidden`, **`inert`** unless opened, scaled live copy of the page; line 391-404).
- `div[data-layered-veil][data-level="dialog"].ui-veil.pointer-events-none.absolute.inset-0.z-30` (glass).
- `div[data-layered-overlay][role="group"].quiz-home-grid.pointer-events-none.absolute.inset-0.z-[31]` (871): CSS grid with `--layered-columns/--layered-rows` (desktop `3 columns 1:1.5:1` × `3 rows 1:1.4:1`, tablet 2 columns, list below 768 px or short viewports `HOME_GRID_MIN_HEIGHT_PX = 480`). Children: `div[data-layered-card="<page>"][data-revealed]` (`display: contents`) > `div.quiz-home-cell` (`flex`, centred, `padding: var(--spacing-double)`) > `div.contents` (click wrapper) > the card `section`.
- In `list` mode (phones/short): `div[data-layered-list][role=group]` of `section[data-layered-section=<page>]`, each with its own veil `z-30` and card host `z-[31]`.
- When a page is opened: overlay/veil unmount; the page fills the area with `div[data-layered-pane][data-opened][role=region]`; the **"Overview" button** `button[data-layered-overview-button].ui-glass.absolute.right-double.top-double.z-40` appears at the top right (`:812-822`).

Cards on the overview (reading order = DOM order = `homePages`, `🏠️home:40-43`): `data-card` / anchor:

| Card | `data-card` | `data-presence-anchor` | Source |
|---|---|---|---|
| Learner | `learner` | `home:learner` | `📇️profile:105-106` |
| Quiz i (1..n) | `quiz:<quizId>` | `home:quiz:<quizId>` | `📖️quiz-page:79-80` |
| How it works | `introduction` | `home:introduction` | `👋️introduction:56-57` |
| Leaderboard (centre, larger cell) | `board` | `home:board` | `🏆️leaderboard:191-192` |
| Badges | `badges` | `home:badges` | `🏅️badges:49-50` |
| Preferences | `preferences` | `home:preferences` | `🎛️preferences:176-177` |

Inside cards: quiz card body `p.text-xs` description, `ul.facts` (`Facts`, `role=list`), earned-badges line; leaderboard card `table` rows `tr` (own row `.quiz-me`, `aria-current="true"`); badges card `ul[role=list]` of tiles `[data-state="earned|locked"][data-earned]`, `[data-badge-mark]`.

### 8.3 Quiz page (home page opened over the overview, hash `#<quizId>`)

`PageFrame[data-page="quiz:<id>"]` (`📖️quiz-page:184`) > `QuizCard card="quiz" anchor=quiz:<id>`, `QuizCard card="quiz-tasks" anchor=quiz:<id>:tasks` containing `ol[role=list] > li.flex.border.border-normal.px-double.py-single` (one row per task: index, `TaskGlyph`, title, kind), then `QuizCard card="quiz-crowd"` (live/past crowd) with per-task `section > h3` and crowd bars. Other pages similarly: `PageFrame[data-page="learner|intro|board|badges|prefs"]`. Others' cursors inside a pane render in `div[data-pane-peers=<scope>]`.

### 8.4 Run page

`main > div.min-h-0.flex-1.overflow-auto.p-double > div.quiz-page > div.quiz-run.flex.flex-col.gap-double` (`▶️run:112`):
1. `QuizCard card="run" anchor="run"` (h1, `focusableHeading`): progress `progress.quiz-progress`, `nav[aria-label] > ol[role=list] > li > button.quiz-target[aria-current="step"]` (task steps, with ✓/○ marks), hint text, footer `Home` + `Submit` (`aria-disabled` until ready).
2. `QuizCard card="task" anchor="task:<taskId>"` containing the task view (keyed by task id):
   - sorting: `ol[role=list][aria-label]` > `li[data-quiz-drag][data-quiz-item=<id>][data-presence-anchor="item:<id>"][data-quiz-drop="item:<id>"].grid.border.border-normal.bg-background` (grip `span[data-quiz-grip]`, index, label, up/down `button`s, guess field, crowd position track `.quiz-crowd-track`) — `▲ smallest` / `▼ largest` `p`s above/below.
   - classification: pool `section[data-quiz-drop="pool"]` + bins `section[data-quiz-drop="category:<id>"][data-presence-anchor="category:<id>"].border-dashed` containing chips `ul[role=list] > li[data-quiz-drag][data-quiz-item][data-presence-anchor="item:<id>"]`; category spider charts `div.quiz-radar > svg.quiz-radar-chart[role=img]`.
   - matching: per dimension `section` > card pool `ul[role=list]` of `[data-quiz-drag][data-used]` cards, and a `table` whose `tr[data-quiz-drop="slot:…"][data-presence-anchor="item:<id>"]` are the rows.
3. Dialogs: `Dialog` (`role="alertdialog"` for confirm/progress) portals to the end of `.quiz-app`: `div[data-quiz-dialog].quiz-backdrop.fixed.inset-0.z-50.grid.place-items-center` > `div[role=dialog|alertdialog][aria-modal=true]` > `QuizCard card="dialog"`.

### 8.5 Results page

`div.quiz-results.flex.flex-col.gap-double` (`🏁️results:223`): `QuizCard card="results" anchor="results"` (h1, score `p.text-lg`, optional `section > ul[role=list] > li[data-earned].border` new badges, footer `Leaderboard` + `Home`), then one `QuizCard card="task-result" anchor="result:<taskId>"` per task containing `table[aria-labelledby]` inside `div.max-w-full.overflow-x-auto` (rows `tr`, `th scope=row`, verdict "✓ Correct"/"◐"/"✗" text). A "missing" variant `card="results"` with `p[role=status]` while the result has not arrived.

### 8.6 Other screens

Introduction (first visit): `QuizCard card="introduction" anchor="introduction"` + preferences aside (`card="preferences"`) in `.quiz-pair` (two columns from 1024 px). Identity: `form > QuizCard card="identity"` with `fieldset.border`. Language chooser (no locale yet): `QuizCard card="language"`. Waiting: `div[aria-busy] > QuizCard card="waiting"#quiz-waiting`. Leaderboard page: `QuizCard card="leaderboard" anchor="leaderboard"` with `div[role=region][tabindex=0].overflow-x-auto > table`.

### 8.7 Suggested stable hooks for pets (existing, no markup change needed)

`section[data-overview-card]` (ground = top edge; `data-card`, `data-presence-anchor` identify it), `[data-slot="quiz-card-title-chip"]`, `header nav[data-slot="navbar"]`, `button[data-overview-card-action]`, `.quiz-target`, `li[data-quiz-item]`, `[data-quiz-drop]` (drop zones, avoid standing in them while `.quiz-drop-active`), `tr` rows in `.quiz-me`/leaderboard tables, `footer.quiz-footer`. Exclude: anything inside `[inert]`, `[data-layered-pane]`, `[data-quiz-dialog]` subtree, elements with zero rect, `[hidden]`, `.sr-only`, the drag ghost, and elements clipped by an `overflow` ancestor (intersect with the scroller rect).

---

## 9. Tests

- **Runner:** Vitest 4 + `@vitejs/plugin-react`, **jsdom** (`environment: "jsdom"`), `@testing-library/react` 16 + `@testing-library/user-event` 14, setup file `UI/🧪️tests/🧹️react-environment/🟦️.ts` (polyfills `ResizeObserver`, `scrollIntoView`, pointer capture, a `PointerEvent` class extending `MouseEvent` so `clientX/Y` exist; `afterEach(cleanup)`). Config: `R/🧪️tests/🎚️config/🟦️.ts` (root = `R/📦️packages/🟦️typescript`; aliases `@semio-tech/quiz-react`, `@semio-tech/quiz`, `framework`, `framework-server`, `ui-react`, `ui-react/i18n`, `ui-react/chrome` to source files; `css: { include: [/🎨️\.css(?:\?|$)/u] }`; `passWithNoTests: false`).
- **Where product tests live:** `Q/🧪️tests/<emoji-name>/🟦️.tsx` (React), `🟦️.ts` (TS core), `🦀️.rs`, `🐍️.py`, `🥒️.feature` — language-agnostic by design: shared inputs/expected outputs in `Q/🧫️fixtures/<emoji-name>/🔣️.json` imported with `import grid from "../../🧫️fixtures/🏠️home-grid/🔣️.json";` and checked against a third-party oracle (e.g. `colord` for contrast, `lightningcss` for the stylesheet, `Intl.NumberFormat` for quantities, `ajv`). The React target runs exactly these 17 files, listed explicitly in the config `include` (`R/🧪️tests/🎚️config/🟦️.ts:32-50`): `📐️quantity-formatting, 🕷️radar-geometry, 📬️outbox-delivery, ⌨️task-keyboard, 🚶️learner-journey, 🗣️translation-completeness, 🏠️home-grid, 🖼️task-icons, 📡️presence-client, 💭️crowd-client, 🌍️language-choice, 📢️live-regions, 🌗️contrast-states, 📇️learner-pages, 🚦️rate-limits, 🎭️identity-step, 🔏️privacy-notice`.
- **How they render the app:**
  - Whole app: `render(<QuizApp proctor="" tenant="…" presence={QUIET_PRESENCE} transport={() => fakeTransport} storage={memoryStorageOrigin().tab()} languages={["en"]} timing={TIMING} />)` (`🚶️learner-journey:337,676-810`), or `mountQuiz(root, {...})` (`🌍️language-choice:126`). The proctor is a `FakeProctor` double that speaks the framework wire with the core's own deciders (`🚶️learner-journey:141+`); presence is a stub `PresenceConnect` with a socket double; storage is `memoryStorageOrigin().tab()`; network timing is shrunk.
  - Single components: `render(<PreferencesPanel preferences={…} locale="en" text={quizText("en")} onChange={fn} />)`, `<HomeScreen session={stubSession()} state={STATE} .../>` (`🏠️home-grid:287-297`), `<PresenceOverlay view={{...EMPTY_PRESENCE_VIEW, peers}} show />` (`🌗️contrast-states:101`), `<PresenceProvider view=… setTask=…>` around cards.
  - Layout is faked: jsdom has no layout, so tests stub `getBoundingClientRect` (`box(el, {left,top,width,height})`, `presence-client:740+`) and `matchMedia` (`vi.stubGlobal("matchMedia", …)`, `home-grid:267-282`; without a stub `useMediaQuery` returns its default `false`). `requestAnimationFrame` is guarded in presence code.
  - Stylesheet is asserted as text (`import stylesheet from "…/🎨️.css?raw"`), via regex or `lightningcss` visitor (`🌗️contrast-states`).
- **Exact commands:** from the repo root `bun nx run @semio-tech/quiz-react:test` (root script `bun run test:quiz:react`, `package.json:223`), `…:test-quick`, `…:test-long`, `…:test-exhaustive`, `bun nx run @semio-tech/quiz-react:typecheck` (`typecheck:quiz:react`, `package.json:221`). They run `bun ./📜️script.ts test [level]` in the package dir → `runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts")` (default level `quick`). Developers use VS Code launch entries `🧪️test❓️quiz⚛️react` and `🛠️dev❓️quiz⚛️react🪁️typecheck` (`.vscode/🧩️launch.seed.jsonc:1208-1217,1251-1261`, mirrored in `.vscode/launch.json:2102,2146`); any new executable command must be registered there per the repo rules.
- **Site level:** `SITE/🧪️tests/🎚️config/🟦️.ts` (node-only vitest: catalog, deploy, local stack, host document) and Playwright specs `SITE/🧪️tests/<name>/🟦️.ts` + `SITE/🎭️e2e` (`🥞️layered-home`, `👥️shared-presence`, `📱️phone`, `🗣️both-languages`, `🔌️connection-shortage`, …) driving a booted stack; they assert geometry of cards and presence on a 1440×900 desktop and a 375×812 phone. A pet layer must stay `pointer-events: none` / `aria-hidden` so those clicks and role queries are unaffected.

---

## 10. `R/📦️packages/🟦️typescript/*`

- `package.json`: name **`@semio-tech/quiz-react`**, `version 0.1.0`, `type: module`, `private: true`, `license LGPL-3.0-or-later`, `bundleKind: "ui"`, `semio: { role: "framework", id: "quiz-react" }`.
  - `exports`: `".": "./🟦️.tsx"` (package glue, which re-exports `../../🟦️.tsx`) and `"./🎨️.css": "../../🎨️.css"`. No `main`/`types`; consumers resolve TS source directly.
  - `dependencies`: `@semio-tech/framework`, `@semio-tech/framework-server`, `@semio-tech/quiz`, `@semio-tech/ui-react` (all `workspace:*`), `react ^19.2.3`, `react-dom ^19.2.3`. (AGENTS rule: no new runtime libraries; external ones only behind an owned interface.)
  - `devDependencies` (test-only): `@testing-library/react`, `@testing-library/user-event`, `vitest ^4`, `jsdom ^24`, `@vitejs/plugin-react`, `colord 2.10.0`, `lightningcss 1.32.0`, `i18next 25.10.10`, `d3-scale`, `lodash`, `opentype.js`, `aria-query`, `typescript ^5.9.3`, type packages. `nx.includedScripts: []`.
- `tsconfig.json`: extends the repo root `tsconfig.json`; `types: ["bun","react","react-dom","vite/client"]`; **paths**: `@semio-tech/quiz-react → ./🟦️.tsx`, `@semio-tech/quiz → Q/📦️packages/🟦️typescript/🟦️.ts`, `@semio-tech/framework → 🧰️framework/📦️packages/🟦️typescript/🟦️.ts`, `@semio-tech/framework-server → 🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🟦️.ts`, `@semio-tech/ui-react/i18n → UI/🎯️targets/⚛️react/🌐️i18n/🟦️.ts`, `…/chrome → UI/…/🪟️chrome/🟦️.ts`, `@semio-tech/ui-react → …/📦️packages/🟦️typescript/🟦️.tsx`, `dom-accessibility-api`. `include`: the package glue, the root `🟦️.tsx`, `🔨️modules/**/*.ts(x)`, `🧪️tests/**/*.ts`, and the 17 product test files (explicit). New test files must be added here **and** to the vitest config.
- `📋️project.json`: Nx project `@semio-tech/quiz-react`; `namedInputs.default` covers the target files, the product's schema/modules/tests/fixtures; targets `test`, `test-quick`, `test-long`, `test-exhaustive`, `typecheck`, each `nx:run-commands` with `cwd` = the package dir and `command: "bun ./📜️script.ts <test [quick|long|exhaustive] | typecheck>"` (`forwardAllArgs`).
- `📜️script.ts`: `#!/usr/bin/env bun`; `ScriptRouter` from the repo library registering `test` (`TestScript` → `resolveTestLevel(segments, "quick")` + `runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts")`) and `typecheck` (`TypecheckScript` → `node_modules/typescript/bin/tsc --noEmit -p tsconfig.json`), default command `test`. No build script: the target ships as source, built by the site's Vite (`SITE/🏗️builder/🌐️vite/🟦️.ts`: `@tailwindcss/vite`, `@vitejs/plugin-react`, release CSP).

---

## 11. `@semio-tech/ui-react` usage

Only two subpaths, enforced by a test: **`/chrome`** (`UI/🎯️targets/⚛️react/🪟️chrome/🟦️.ts`, 107 lines) and **`/i18n`** (`UI/🎯️targets/⚛️react/🌐️i18n/🟦️.ts`). Never the barrel.

From `/chrome` (the whole slim surface): `cn` (class composition), `WindowChrome`, `windowChromeTitleChipClass`, `OverviewCard`, `OverviewCardAction`, `OverviewCardOpenChip`, `overviewCardChipClass`, `LayeredOverview` + its pure geometry (`LAYERED_*`, `cellOffset`, `stripGrid`, `veil*`, `trackTemplate`, …) and types (`LayeredPane`, `LayeredCardState`, `LayeredCell`, `LayeredTracks`, …), `Navbar`, `SemioLogo`, `ShellBrandLogo`, `navbarFillItem`, `Icon` / `IconName`, `DEFAULT_UI_DRIVER`, `UI_AVAILABLE_HEIGHT`, `UI_MOBILE_MAX_WIDTH_PX`, `UI_TABLET_MAX_WIDTH_PX`, `UI_MOBILE_MEDIA_QUERY`, `UI_TABLET_MEDIA_QUERY`, `elementsSurfaceDeviceForMatches`, `bootstrapElementsSurfaceChromeDocument`, `useElementsSurfaceChrome`, `useMediaQuery`, `resolveElementsSurfaceChromeDark`, `presenceColor`, `presenceCssVar`, `presencePaint`.

Who imports what in the quiz target:

| Module | Imports from `/chrome` |
|---|---|
| root `🟦️.tsx:19-32` | `DEFAULT_UI_DRIVER, Navbar, ShellBrandLogo, UI_AVAILABLE_HEIGHT, UI_MOBILE_MEDIA_QUERY, UI_TABLET_MEDIA_QUERY, bootstrapElementsSurfaceChromeDocument, elementsSurfaceDeviceForMatches, navbarFillItem, useElementsSurfaceChrome, useMediaQuery, NavbarItem` |
| `🪟️chrome:13` | `Icon, OverviewCard, OverviewCardAction, cn, overviewCardChipClass, IconName` |
| `🏠️home:22` | `LayeredOverview, UI_MOBILE_MAX_WIDTH_PX, UI_TABLET_MAX_WIDTH_PX, useMediaQuery, IconName, LayeredCardState, LayeredCell, LayeredPane` |
| `👥️presence:48` | `Icon, presenceColor, presencePaint` |
| `🏆️leaderboard` | `Icon, overviewCardChipClass` |
| `🏅️badges` | `Icon` |
| `▶️run:10` | `IconName` (type) |

From `/i18n` (only `🌐️i18n:14`): `registerUiTranslationBundles, resolveUiLabel, setUiLocale, uiI18n, DeepUiTranslationKeys, UiLabelValue`.

`OverviewCard` (`UI/🧱️elements/🃏️OverviewCard/🟦️.tsx`): props `slot, icon, title, children, footerLeft, footerRight, className, bodyClassName, contentClassName, data`; `as="section"` (used by the quiz: `headingId, headingLevel, headingRef, focusableHeading`) or `as="button"`. Stamps `data-overview-card`, `data-slot="<slot>-title-chip|-content|-stack"`, `data-overview-card-stack`. Quiz slot = `"quiz-card"`.

`LayeredOverview` (`UI/🧱️elements/🥞️LayeredOverview/🟦️.tsx`, 880 lines): described in §8.2. Relevant props the quiz uses: `panes, cells, rest="grid", gridTracks, renderCard, overlayClassName="quiz-home-grid", mode, routing="hash", openedId, onOpenedIdChange, lifecycle, labels`; it also has `reducedMotion?: "auto"|"always"|"never"` (default `"auto"`) and `pan` (not used with `rest="grid"`). Hover reveal uses `pointerType === "mouse"` pointer enter/leave (`:439-440`) — touch never "reveals".

---

## 12. Not verified / open points

- Whether WindowChrome's `z-30` chip / `z-[40]` silhouette actually compete with a `z-[35]` fixed layer in the root stacking context in the running app (depends on ancestors creating stacking contexts): needs a browser check.
- Whether the proctor rejects extra keys in `CursorState` (only the allow-list shape in `cursorIssues` was read, not the wire behaviour).
- Exact current behaviour of the dev server was not started (read-only brief). No test was run.
- `Q/🧪️tests/🔁️…`/Rust twins are irrelevant to a React-only pet layer, but a catalog-level pet field (option B in §3.4) would have to touch the schema JSON, `🟦️.ts`/`🦀️.rs` twins, `🔮️oracles/🔣️.json` and the language-agnostic tests.
