# Explore: the teaching site as the consumer of the quiz challenge levels

Ticket `2026/10/02/QUIZ-CHALLENGE-LEVELS`. Read-only exploration of `🎓️teaching/🏛️architecture/❓️quiz/`, the four
energy quizzes and the launch files, as they are on disk on 2026-10-02 (other sessions are editing the same tree).
Marks: **[read]** verified by reading the file or running a read-only script over it; **[ran]** computed with a throw-away
script over the JSON; **[infer]** my inference, not verified.

Paths below are relative to `C:\git\semio`. `SITE` = `🎓️teaching/🏛️architecture/❓️quiz`, `ENERGY` =
`🎓️teaching/🏛️architecture/⚡️energy`, `PRODUCT` = `🧰️framework/🛍️products/❓️quiz`.

---

## 1. Content census

### 1.1 Overview [ran]

Catalog `architecture` (`SITE/🔣️.json:4`) names four quizzes in this order (`SITE/🔣️.json:42-47`): `physics`, `heating`,
`cooling`, `demand`. Nine tasks, 109 items, 7 matching dimensions, 3 task kinds. Every text is `{en, de}` (schema `Text`
requires both, "there is no default language", `PRODUCT/🧬️schema/🔣️.json:31-39`).

| quiz (file lines) | task id | kind | prompt (EN, shortened) | authored items | `draw` (items per run) | dims / categories | unit(s) |
|---|---|---|---|---|---|---|---|
| physics (488) | `power-or-energy` | classification | Decide for each quantity whether it is a power ... or an energy (demand) | 16 (8 power, 8 energy) | 12 | 2 categories | none |
| physics | `powers` | sorting | Sort the powers in ascending order, smallest first. Quantity: power in watts (W); the values span about 25 orders of magnitude. | 14 | 10 | - | W, logarithmic, `prefixed:true` |
| physics | `energies` | sorting | Sort the amounts of energy in ascending order, smallest first. Quantity: energy in watt-hours (Wh); ... about 16 orders of magnitude. | 12 | 9 | - | Wh, logarithmic, `prefixed:true` |
| heating (338) | `u-values` | matching | Assign each building component its thermal transmittance (U-value) in W/(m²·K) ... Smaller means better insulated. | 17 | 10 | 1 dim | W/(m²·K), log, not prefixed |
| heating | `heating-load-and-demand` | matching | Assign each building its specific design heating load in W/m² ... and its annual heating demand in kWh/(m²·a) ... | 11 | 8 | 2 dims | W/m², kWh/(m²·a) |
| cooling (298) | `air-change-rates` | matching | Assign each room use its typical design air change rate in 1/h ... | 15 | 10 | 1 dim | 1/h |
| cooling | `cooling-load-and-demand` | matching | Assign each building its specific cooling load in W/m² ... and its annual cooling demand in kWh/(m²·a) ... | 9 | 7 | 2 dims | W/m², kWh/(m²·a) |
| demand (263) | `standard-profiles` | classification with spider diagrams | Each spider diagram shows the typical profile of one residential energy standard per m² ... Assign each standard to its profile. | 6 | all (no `draw`) | 4 axes, 6 categories (one profile each), 1 item per category | axes: kWh/(m²·a) x3, EUR/(m²·a) |
| demand | `final-energy` | matching | Assign each building with its supply system its total specific final energy demand for heating, hot water and auxiliary energy in kWh/(m²·a) ... | 9 | 7 | 1 dim | kWh/(m²·a) |

The first task of a quiz stays first in a run, the others are shuffled (schema `Quiz.description`, `PRODUCT/🧬️schema/🔣️.json:~263`).
Every run draws `draw` items out of the authored ones (`Sheet` is "a pure function of (quiz, seed)").

What this means per kind [read, schema]:

- **Sorting.** Learner sees only labels and icons of the drawn items (`SheetItem` = `id,label,icon`, no `value`,
  `PRODUCT/🧬️schema/🔣️.json:~396-405`); the sheet is explicitly "solution-free". Items have one number `value` in the
  quantity's base unit (`SortingItem`, schema l.~160). So "show the keys" for sorting = putting the item's `value` into the
  sheet, which today does not exist (contract change, see section 5).
- **Matching.** The sheet already shows value cards: `SheetDimension.cards` = "Card values in presentation order; a card
  is addressed by its index" (schema l.~342-352), the multiset of the drawn items' true values. So in matching the learner
  today sees all candidate numbers and assigns them to items (one `<select>` per item and dimension). "Keys visible" has no
  natural meaning; "keys hidden / guess" would mean replacing the cards by a typed number per item per dimension (like
  sorting's existing optional guess field) and scoring by log distance; or hiding the card list but offering the
  free input. **[infer]**
- **Classification.** No numbers on items. The only numbers are the spider-diagram axes/profiles of `demand/standard-profiles`
  (`Axis.min/max`, `Category.profile`). There "keys" would be the profile values written as numbers on the spokes; "hidden" =
  diagram without the numeric tick labels. Plain `power-or-energy` has no numeric key at all (the "key" is the category
  semantics) - a level can only change the number of categories shown/hints/time. **[infer]**

### 1.2 Numeric ranges [ran: node script over the JSON files]

"decades" = log10(max/min). "pairs >1000" = unordered pairs of the authored items of the task whose values differ by a
factor greater than 1000 (hint candidate density); all values are strictly distinct (no ties anywhere); the adjacent
minimum ratio is the closest two neighbours in sorted order.

| task / dimension | unit | min | max | decades | min adjacent ratio | pairs > 1000 | pairs > 100 | pairs > 10 |
|---|---|---|---|---|---|---|---|---|
| physics/powers | W | 35 (tea light) | 3.828e26 (Sun) | 25.04 | 1.60 | 60/91 | 72/91 | 82/91 |
| physics/energies | Wh | 15 (phone charge) | 1.644e17 (world primary energy per year) | 16.04 | 3.70 | 39/66 | 49/66 | 59/66 |
| heating/u-values | W/(m²·K) | 0.1 | 5.8 | 1.76 | 1.15 | 0/136 | 0/136 | 26/136 |
| heating/heating-load | W/m² | 10 | 160 | 1.20 | 1.20 | 0/55 | 0/55 | 2/55 |
| heating/heating-demand | kWh/(m²·a) | 15 | 303 | 1.31 | 1.107 | 0/55 | 0/55 | 6/55 |
| cooling/air-change-rate | 1/h | 0.13 | 540 | 3.62 | 1.25 | 4/105 | 17/105 | 54/105 |
| cooling/cooling-load | W/m² | 6 | 1000 (data centre) | 2.22 | 1.25 | 0/36 | 1/36 | 9/36 |
| cooling/cooling-demand | kWh/(m²·a) | 2 | 8000 (data centre) | 3.60 | 1.25 | 2/36 | 7/36 | 14/36 |
| demand/final-energy | kWh/(m²·a) | 14 | 409 | 1.47 | 1.30 | 0/36 | 0/36 | 6/36 |

Consequences for the "off by a factor of more than 1000" hint [ran + infer]:

- It only has a chance to fire in the two physics sortings (65 % and 59 % of all pairs), in `air-change-rate` (4 % of pairs;
  only warehouse/passive-house/apartment/sports hall vs cleanroom-iso-5 and iso-6) and in `cooling-demand` (2 of 36 pairs, all
  involving the data centre at 8000). In the other five numeric dimensions (u-value, both heating dims, cooling-load,
  final-energy) the maximum ratio is 58 or less, so a fixed "1000" factor would never fire: those quantities span 1.2-1.8
  decades. A threshold per task (authored or derived, e.g. a fraction of the task's own span in decades) is needed, or the
  hint stays a sorting/physics feature.
- Physics adjacent ratios [ran]: powers 2.86 10 2 5.5 1.64 5.56 50 1.6 176 42.6 312 9305 2.2e9; energies 6.2 6.75 4.3 3.7 6 7.33
  5.68 13.2 303 292500 56.2. The last gaps are huge (sun vs sunlight-on-earth 2.2e9; germany primary vs world 56; wind turbine year
  to Germany primary 292500), i.e. a learner who mistakes a tea light for a power plant is the case the hint targets. The
  intro already says "swapping two neighbours costs little, mistaking a tea light for a power plant costs a lot" (the
  scoring is magnitude weighted on the log scale; `SITE/🔣️.json`, introduction paragraph 3).
- Matching with a log scale and drawn subset (7-10 of 9-17 items) means the cards are often 1.1-1.3x apart (min adjacent ratio
  1.107-1.30 in the five energy-building dims). A learner guessing a number freely (hard level) could not tell 14 from 23
  from 30 kWh/(m²·a) without the cards; scoring must be tolerance/log-distance based. **[infer]**

### 1.3 Per-question time (what is being asked in a run) [ran + infer]

Counts per drawn task: sorting 9 or 10 items; matching 7-10 items x 1 dimension (7-10 assignments) or 7-8 items x 2 dimensions
(14-16 assignments); classification 12 items (power/energy) or 6 items (profiles). Average label length 31-66 characters,
explanation 104-255 characters (explanations are only shown in the results). A defensible expert limit is therefore not a
flat number but proportional to the assignment count, e.g. a base of about 20 s plus 6-10 s per item/assignment for a first
reading: about 80-120 s for a 10-item sorting, 90-180 s for a two-dimension matching, 60-100 s for 12 binary classifications,
and about 90 s for the spider-profile task (6 diagrams to read). Those are my estimates, nothing in the repo fixes them
**[infer]**. The driver's current real-time cost of a perfect sorting is the move buttons one click and one poll each
(`sortInto`), see 4.

---

## 2. Authoring fields related to difficulty, time, points, weights, hints, explanations, feedback, sources

Verified by reading the schema (`PRODUCT/🧬️schema/🔣️.json`) and walking every key path of the four quiz files [ran].

Key paths in use (count = occurrences over the four files): quiz: `$schema, schema, id, emoji, title{en,de}, description{en,de}, tasks`;
task: `kind, id, title, icon{emoji,motion}, prompt, dimensions, items, draw, axes, categories, quantity`; item: `id, label, icon,
explanation{en,de}` plus kind-specific `category` (22 items, classification) / `value` (26 items, sorting) / `values{<dimension id>}`
(61 items, matching); category: `id, label, icon, description` (physics only), `profile{axis id: number}` (demand only); dimension:
`id, icon, quantity{label,unit,scale,prefixed}`; axis: `id, label, unit, min, max`.

| concern | exists today? | where |
|---|---|---|
| difficulty / level | **No.** No field in schema, content, product (grep for `difficulty`, `timeLimit`, `deadline`, `countdown`, `hint`, `challenge` over `PRODUCT` finds only a `🧫️fixtures/🧬️schema-conformance` mention that is not an authoring field; README has no hits for hint/difficulty/level) | - |
| time limit | **No** (`QuizOptions.timing` is `RetryTiming`, i.e. retry back-off of the outbox, `PRODUCT/🎯️targets/⚛️react/🟦️.tsx:201`, not a question timer; `QuizSession` has a `now` seam, `🔨️modules/🧭️session/🟦️.ts:622`) | - |
| points / weight | **No authoring field.** Score is computed: each task earns 0-1 (`Score`, schema l.~43-47), the run is the mean as a percentage; leaderboard "total" is "Sum of the best scores over the catalog quizzes, in points (score x 100)" (schema l.885). Task weights are all equal: "a run scores the mean of its task scores in sheet order" (`PRODUCT/🔨️modules/📏️scoring/🟦️.ts:3`, `mean(...)` l.171) | scoring is in the cores, not in content |
| hints | **No** | - |
| explanations | **Yes**, one `explanation: Text` per item (classification, sorting, matching items; 109 of 109 items have one, optional in schema). Shown in the results rows; the e2e driver checks that a row contains it (`expectFeedback`) and that nothing of it shows before submission (`expectNothingRevealed`) | `SortingItem`/`MatchingItem`/`ClassificationItem.explanation` |
| feedback texts | Only those explanations; the rest is product-owned i18n (`PRODUCT/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts`) | - |
| sources | **Only inline prose** inside `explanation` ("... Source: BAnz AT 04.12.2020 B1, Table 2."). 23 of 109 items carry "Source"/"Quelle" in their explanation: heating/u-values 16 of 17, heating-load 4 of 11, cooling 1+1, physics/power-or-energy 1, demand 0. No structured `source` field, no URLs field | inline text |
| category description | `Category.description` (2 categories of physics `power-or-energy`) | schema l.~120-130 |
| icons / motion | Yes, on tasks, items, categories, dimensions; enforced by the site test (see 3.3) | schema `Icon` |
| `draw` | Items per run for a task, integer >= 2, absent = all; the one numeric knob a level could reuse (fewer/more items) | `ClassificationTask/SortingTask/MatchingTask.draw` |

Existing related user-facing mechanics (not authoring) [read]: sorting already has an optional typed numeric guess per
item (`SortingAnswer.guesses`, reorders by guess, never changes the score; design in
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-SORTING-NUMERIC-GUESSES/📓️design.md`, ticket closed). `aria-label` is
`quiz.sorting.guess` (item label), placeholder `quiz.sorting.guessPlaceholder`, preview `quiz.sorting.guessPreview` = "= {{value}}",
in `PRODUCT/🎯️targets/⚛️react/🔨️modules/↕️sorting/🟦️.tsx:119-142`. A "hard" sorting level that hides keys and demands guessed numbers
overlaps with this optional feature, which today never scores.

---

## 3. How the site mounts the product

### 3.1 `mountQuiz` call [read: `SITE/🟦️.ts:30-33`]

```ts
const root = document.getElementById("root");
if (root) mountQuiz(root, { proctor: bakedProctorOrigin(), tenant: ARCHITECTURE_QUIZ_TENANT, material: ARCHITECTURE_QUIZ_MATERIAL, logo, legal: site.legal, pets: () => import("../🐾️pets/🟦️.ts").then((module) => module.ARCHITECTURE_MENAGERIE) });
```

- `ARCHITECTURE_QUIZ_TENANT = "architecture"` (`SITE/🟦️.ts:21`); `bakedProctorOrigin()` = `import.meta.env.VITE_PROCTOR_URL ?? ""` (`""` =
  same origin; dev proxy). No `languages`, `transport`, `presence`, `storage` or `timing` option is passed: the browser's
  language list decides (`navigator.languages`), `QuizOptions` seams exist (`PRODUCT/🎯️targets/⚛️react/🟦️.tsx:191-202`):
  `proctor, tenant, material?, logo?, legal?, pets?, transport?, presence?, storage?, languages?, timing?`.
- `logo` is the emblem SVG raw (`?raw`); `legal` = `site.legal` of `SITE/🚀️deploy/🔣️.json` (currently `{}` so no legal links); `pets` is
  a lazy import of `../🐾️pets` (architecture menagerie, 20 species).
- `material` makes the client work while the proctor is away: `new Deputy(options.material)` in `useSetup`
  (`🟦️.tsx:487-496`), i.e. the **device** also holds all solutions and decides/scores locally ("deputy offline"). So any level
  rule must exist in the TS core used by the device and in the Rust proctor.
- Languages: schema forces `{en, de}` everywhere. The language is stored or chosen by the browser; if the browser names neither,
  the client shows only a language chooser. The page `🌐️.html` has no `lang` (the builder writes the title in both languages,
  `SITE/🏗️builder/🌐️vite/🟦️.ts:26-40`).
- The README row for the entry is stale: `mountQuiz(root, { proctor: <baked proctor origin or "">, tenant: "architecture", material })`
  (`SITE/README.md:13`) omits `logo`, `legal`, `pets`.

### 3.2 How the catalog and material are built [read]

- `SITE/📚️catalog/🟦️.ts:11-18`: static JSON imports of `SITE/🔣️.json` and the four `ENERGY/*/❓️quiz/🔣️.json`; exports exactly
  `ARCHITECTURE_QUIZ_MATERIAL: { catalog: Catalog; quizzes: readonly Quiz[] }`, quizzes `[physics, heating, cooling, demand]`
  cast through `unknown`. Nothing is fetched at run time; the quizzes (with all solutions) are part of the site script.
- `SITE/🏗️builder/🌐️vite/🟦️.ts`: `defineOwnedBuildConfigFactory`; plugins: host HTML (`quizHostDocument`), emoji index,
  referenced assets, tailwind, React, and `quizReleaseDocumentVitePlugin` (CSP sealed with SHA-256 of the inline boot scripts,
  `connect-src` = baked proctor origin only; `robots.txt`, `manifest.webmanifest`, `CNAME`); `server.proxy` for
  `/instance /commands /queries /actors /scopes` to `http://127.0.0.1:${PROCTOR_PORT ?? 8791}` with `ws:true` (l.24-25, 135-139);
  aliases for `@semio-tech/quiz`, `quiz-react`, `pets`, `pets-react`, `ui-react*`, `framework*`; env `TEACHING_ARCHITECTURE_QUIZ_CACHE`
  and `TEACHING_ARCHITECTURE_QUIZ_WATCH=off` (l.113, 138). No content-specific plugin.
- Catalog `SITE/🔣️.json`: `introduction` with 5 paragraphs (they describe scoring: "Every task earns between 0 and 100%", "partially correct
  answers earn partial points", magnitude-weighted pair costs, badges, leaderboard "ranks everyone by the sum of their best quiz
  scores") and 7 badges: `physics-expert`, `heating-expert`, `cooling-expert`, `demand-expert` (rule `perfect-quiz`),
  `numerical-brain` (`perfect-tasks` kind `sorting`), `pattern-seer` (`perfect-tasks` kind `classification`), `completionist`
  (`completed-quizzes`). **A level-aware product must say what these rules mean per level** ("perfect on easy" earning the
  same badge as "perfect on expert"?). The introduction text must change too (it states the scoring).
- Pets are grounded in `quiz`, `quiz/task`, `quiz/task/item` ids: `SITE/🧪️tests/🐾️pet-cast/🟦️.ts:34` builds `groundable` and
  every species names grounds in the quiz files. **Renaming or removing a quiz/task/item id breaks the pet-cast test**; adding
  authoring fields does not.

### 3.3 What the site tests enforce on content (precedents for level authoring rules) [read: `SITE/🧪️tests/🧪️catalog/🟦️.ts`, 71 lines]

Vitest config `SITE/🧪️tests/🎚️config/🟦️.ts` includes only `🧪️catalog`, `🧪️deploy`, `🧱️local-stack`, `📰️host-document`, `🐾️pet-cast`
(node); the other test folders are Playwright specs run by the e2e gate. The catalog test:

1. `catalog.id === "architecture"`, at least one quiz (l.23-26).
2. `catalogIssues(catalog, quizzes)` from the TS core is `[]` (l.28-30), the draft-07 schema accepts catalog (ajv, l.32-34).
3. The material module equals the documents on disk in catalog order and exports nothing else (l.36-39).
4. Per quiz file: `quizIssues(quiz)` is `[]` (TS core); every task has an icon with a distinct emoji and a motion in `MOTIONS`
   (l.46-52); every item, category and dimension has an icon, and "no two items of a task look and move alike" (same emoji+motion
   pair, l.54-60); ajv accepts the quiz (l.62-64).

So the precedent for authoring rules is two-layered: structural rules live in the product's `quizIssues`/`catalogIssues`
(TS core, with a Rust twin used by `proctor check`) and the contract in the schema (ajv third-party oracle); the site adds
content-policy rules (icons everywhere, distinct looks) in its own test. A level authoring rule (e.g. "every sorting task has
a `hintFactor`" or "hint threshold < span") would follow the same split. **[infer]**

Other site tests touching content: `🧪️tests/🗣️both-languages` plays physics, heating, demand perfectly in both languages and
checks no raw i18n key (`quiz.x.y`) or `{{placeholder}}` remains (driver `unresolvedLabels`); note it plays three quizzes
(`for (const id of ["physics", "heating", "demand"])`, l.~112), not cooling.

---

## 4. End-to-end

### 4.1 Gate and projects [read: `SITE/🎭️e2e/🟦️.ts`, `SITE/🎭️e2e/🎚️config/🟦️.ts`]

- `bun nx run @teaching/architecture-quiz:test-e2e [dev] [rehearsal] [--serial] [--keep] [--proctor <exe>] [playwright args]` (script
  `SITE/📦️packages/🟦️typescript/📜️script.ts`, target `test-e2e` in `📋️project.json`). It builds the proctor once, boots throw-away
  stacks, runs Playwright (v1.62.1, Chromium) per topology with `PLAYWRIGHT_BASE_URL`, and deletes the stack on success.
- Topologies and ports (`QUIZ_E2E_PORTS`, `🎭️e2e/🟦️.ts:45`): `dev` site 6161 / proctor 8891 (development mode, Vite dev
  server with proxy, `TEACHING_ARCHITECTURE_QUIZ_WATCH=off`); `rehearsal` site 6162 / proctor 8892 (release build with
  `PROCTOR_URL=http://127.0.0.1:8892` baked in, served static by `serveStaticSite`, proctor in production mode granting
  `PROCTOR_ALLOWED_ORIGINS=http://127.0.0.1:6162`). Run dir `.🧬semio/🎓️teaching/architecture-quiz-e2e/<run>/`. Control endpoint
  (`POST /proctor/stop|start`) on a free loopback port for the away/shortage specs.
- Env a spec reads: `TEACHING_ARCHITECTURE_QUIZ_E2E_TOPOLOGY|_PROCTOR|_CONTROL`, `PLAYWRIGHT_BASE_URL`.
- Playwright config (`🎚️config/🟦️.ts`): `workers: 4` (2 each when both topologies run), `fullyParallel:false`, `timeout: 600_000`,
  `expect.timeout: 20_000`, `actionTimeout: 20_000`, `navigationTimeout: 60_000`, `retries: 0`, viewport 1440x900 (`DESKTOP_VIEWPORT`),
  phone 375x812 (`PHONE_VIEWPORT`, `isMobile`, `hasTouch`). Projects in order:

| project | depends on | specs (folder under `SITE/🧪️tests/`) |
|---|---|---|
| `boot` | - | `🚀️site-boot` |
| `desktop` (fullyParallel) | boot | `🪪️first-visit`, `🥞️layered-home`, `🎯️quiz-runs`, `🏆️live-leaderboard`, `🗣️both-languages` |
| `phone` | boot | `📱️phone` |
| `presence` | desktop, phone | `👥️shared-presence` |
| `shortage` | presence | `🔌️connection-shortage` |
| `away` | shortage | `📴️proctor-away` |
| `pets` (fullyParallel) | away | `🐕️pet-walk` (841 lines) |

(The user-facing summary asked for "desktop, phone, presence, shortage, away, pets": that is exactly this list, plus `boot`.)

### 4.2 Driver `SITE/🎭️e2e/🚶️learner/🟦️.ts` (602 lines) [read]

Catalog access from disk: `CATALOG`, `QUIZZES` (authored JSON incl. solutions), `quizOf(id)`, `badgeOf(id)`, `badgesFor(perfect[])`,
`ascending(task, ids)`, types `SourceTask {kind,id,title,items,draw?,categories?,quantity?,dimensions?}`, `SourceItem {id,label,explanation,category?,value?,values?}`,
`SourceQuiz`, `SourceBadge`. Everything is answered by item id because sheets are shuffled per run (no position answers).

Fixtures and devices: `test` (Playwright `base.extend`) with `device(locale = "en", language = BROWSER_LOCALES[locale]) : Promise<Device>`;
`Device {context, page, locale, problems, expectingFailures(body)}`; any console error, page error, failed request, HTTP >= 400,
socket error or CSP violation fails the test unless announced via `expectingFailures` (the "proctor away" specs). `proctor("stop"|"start")`
takes the stack's proctor away.

Journey signatures (verbatim):

```ts
export async function arrive(device: Device, timeout?: number): Promise<void>
export async function readIntroduction(device: Device): Promise<void>
export async function identify(device: Device, identity: Identity, atRest = false): Promise<void>
export async function enter(device: Device, identity: Identity, atRest = false): Promise<void>
export type Identity = { readonly kind: "anonymous" } | { readonly kind: "pseudonym" | "name"; readonly handle: string };
export function handle(prefix: string): string
export async function playQuiz(device: Device, quiz: string): Promise<void>
export async function openTask(device: Device, index: number): Promise<void>
export async function openTaskById(device: Device, task: string): Promise<void>
export async function shownTask(device: Device): Promise<string>
export async function shownItems(device: Device): Promise<readonly string[]>
export async function itemBoxes(device: Device): Promise<Readonly<Record<string, readonly number[]>>>
export async function classify(device: Device, item: string, category: string): Promise<void>
export async function dragInto(device: Device, item: string, category: string): Promise<void>
export async function sortInto(device: Device, order: readonly string[]): Promise<void>
export async function moveDown(device: Device, item: string): Promise<void>
export async function shownQuantity(device: Device, value: number, quantity: SourceQuantity): Promise<string>
export async function match(device: Device, dimension: string, item: string, label: string): Promise<void>
export async function unmatch(device: Device, dimension: string, item: string): Promise<void>
export async function answerTask(device: Device, quiz: SourceQuiz, how: "perfect" | "flawed"): Promise<void>
export async function answerRun(device: Device, quiz: SourceQuiz, how: "perfect" | "flawed"): Promise<void>
export async function expectNothingRevealed(device: Device, quiz: SourceQuiz): Promise<void>
export async function submitRun(device: Device): Promise<void>
export function percent(text: string): number
export async function shownResults(device: Device): Promise<{ readonly score: number; readonly tasks: readonly { readonly task: string; readonly score: number; readonly rows: readonly { readonly label: string; readonly text: string }[] }[] }>
export async function expectFeedback(device: Device, quiz: SourceQuiz, how: "perfect" | "flawed"): Promise<Awaited<ReturnType<typeof shownResults>>>
export async function goHome(device: Device): Promise<void>
export function connection(device: Device): Locator
export async function expectSaved(device: Device): Promise<void>
export async function learnerId(device: Device): Promise<string>
export async function unresolvedLabels(device: Device): Promise<readonly string[]>
```

Places: `card(page, id)`, `pane(page, id)`, `screen(page, name)`, `primary(scope)`, `secondary(scope)`, `way(page, "overview"|"back"|"forward"|"up")`.

How a spec answers today (`answerTask`, l.455-480) [read]:

- **Classification:** for each shown item (`shownItems` reads `[data-quiz-item]` ids), `classify` selects the category in the
  item's `<select>` (`[data-quiz-item="<id>"] select`). The flawed variant `dragInto` one item with the real mouse into a
  different category bin (`[data-quiz-drop="category:<id>"]`).
- **Sorting:** `ascending(task, items)` computes the true order from the authored `value`s; `sortInto` clicks the "move up" button
  (`[data-quiz-item="<id>"] button` first) once per place and polls after each click until the item moved
  (`ol > [data-quiz-item]`). Flawed = `moveDown` of the first pair with different values. **The driver never types a guess**
  although the guess field exists (`aria-label` `quiz.sorting.guess`).
- **Matching:** per dimension and item, `match` finds in `[data-quiz-drop="slot:<dimension>:<item>"] select` the option whose
  text equals `shownQuantity(...)` (`Intl.NumberFormat(locale, {maximumFractionDigits: 6}) + " " + unit`) and selects it (prefers
  the card the item holds, else a free one). It throws `matching by a prefixed quantity (...) is not driven yet` for
  `prefixed:true`; all current matching dimensions are `prefixed:false`. Flawed = swap the smallest and largest cards of the
  first dimension (`unmatch` then `match`).
- It asserts `items.length === min(task.draw ?? task.items.length, task.items.length)` before answering.
- Run steps: `playQuiz` clicks `primary(card(page, quiz))` and waits for `screen(page, "run")`; the run's task steps are
  `screen(page, "run").locator("nav button")` (with `aria-current="step"`, `[data-complete]` when answered); `submitRun`
  clicks `primary(screen(page,"run"))`, then `primary(page.getByRole("alertdialog"))`, waits for `screen(page, "results")`.
  Results: `shownResults` reads the first `<p>` percentage of `screen "results"` and for each `screen "task-result"` the rows of
  `table:not(.quiz-plot) tbody tr` (crowd figures are also tables, hence the filter); `expectFeedback` requires one row per drawn
  item (times dimensions for matching), each containing the item's `explanation[locale]`, and 100 % for `perfect`, `> 0` for `flawed`.
- Hooks the product renders for tests: `data-card`, `data-layered-overview`, `data-layered-pane`/`data-opened`, `data-quiz-item`,
  `data-quiz-drop` (`category:<id>`, `slot:<dimension>:<item>`), `data-quiz-grip`, `data-complete`, `data-presence-anchor`
  (`task:<id>`, `result:<id>`), `data-earned`, `data-tone` on the connection indicator, `data-quiz-nav`, `.quiz-plot` tables.

Who uses what [ran: grep]: `answerRun` is used by `🎯️quiz-runs`, `🏆️live-leaderboard` (via a local helper), `🐕️pet-walk` (l.793),
`👥️shared-presence` (l.90), `📱️phone` (l.54), `📴️proctor-away` (l.45,85,99), `🔌️connection-shortage` (l.73), `🗣️both-languages` (l.116);
`answerTask` by `🪪️first-visit` (l.39) and `🔌️connection-shortage` (l.23,43,52). All of them call `playQuiz(device, quiz)` and
assume no level choice step exists and that a perfect answer yields 100 % and the badge set of `badgesFor`.

### 4.3 What an e2e spec "play the same quiz on easy vs hard vs expert" needs from the driver **[infer]**

1. A level parameter wherever a run starts: `playQuiz(device, quiz, level?)` (or a separate `chooseLevel(device, quiz, level)`) plus an
   assertion helper `shownLevel(device)`; whichever hook the UI uses for the level (suggest a `data-quiz-level` attribute).
2. Level-aware `answerTask`: the solution source is the same authored JSON, but the path differs. Sorting: easy/medium could
   keep `sortInto`; hard needs a `guess(device, item, value)` helper that types into the `Guess for <item>` field (aria-label from
   `quiz.sorting.guess`, Enter commits; the item then orders itself) and for matching a numeric entry helper in place of
   `match` (selects by card text). Today's `shownQuantity` formatting and the prefixed-quantity limitation (`prefixed:true` is not
   driven) become relevant: the physics sortings are `prefixed:true`, so typing guesses needs `formatQuantity`/SI-prefix
   handling or a base-unit typed value (e.g. `1e3`, the parser reads `1e3 kW`).
3. Reading shown keys on easy/medium for assertion (new hook per item, e.g. `[data-quiz-item] [data-quiz-key]`) and checking
   keys are **absent** on hard (an analogue of `expectNothingRevealed`, which scans the page text for explanations).
4. Hints: a helper to read the hint of an item/task and force it (a wrong sort that places the sun first), with the 1000x rule.
5. Time: Playwright 1.62.1 has `page.clock` (not used by any spec today; I grepped `tests/` for `.clock`). For expert,
   specs need either `page.clock.install()` / `fastForward()` or a time seam in the client (`QuizSession` has `now`; `QuizOptions`
   exposes only `timing: RetryTiming`, which is about retries). Real-time expert tests would be flaky: today's perfect sorting of 10
   items performs up to ~45 clicks, each awaited with a poll, and matching 16 select operations; the global 20 s step timeout and
   600 s test timeout are generous, but a question timer of 60-120 s would race with a loaded runner (the README says both
   topologies take 7 min on an idle machine, 11 beside other builds).
6. Points: `shownResults` returns only `score` (percent) and task scores. A level multiplier needs a way to read points (run, quiz,
   leaderboard total in points = score x 100 today) and `expectFeedback`'s "perfect gives 100 %" needs a level-aware
   rule; the leaderboard specs (`🏆️live-leaderboard`, which orders learners by total of best quiz scores) and `badgesFor` too.
7. Time-expired behavior (expert): submitting nothing/partial after time out; `answerRun` asserts the `[data-complete]` marker on
   every step and `primary(run)` enabled before submit - both change if a timed-out question auto-completes.
8. Deputy/offline: `📴️proctor-away` and `🔌️connection-shortage` run the level path on the device with the proctor stopped, so level
   rules must hold in the deputy path as well; add cases to those two files rather than a new project when possible
   (project `shortage`/`away` run alone, in sequence, which is expensive).
9. Anything new that counts online/idle devices has to join the `presence` project; the `desktop` project runs 5 files in parallel
   on a shared proctor and "holds whoever else is learning" (config docblock): a level spec belongs there, asserting only on its own
   learner/pseudonym (`handle(prefix)` makes unique pseudonyms).
10. `🧪️tests/👥️shared-presence` and the crowd figures use item anchors (`item:<id>`, "what the other thinks"): hidden keys
    change what a peer's draft shows (guesses), see 4.2 `thinking` drafts (`ThinkingAnswer.guesses`).

---

## 5. Deploy and readiness gates affected by a contract change

- **Where content lives in deployables [read].** The proctor image carries the catalog and every quiz in the layout the catalog's
  relative paths expect, and validates them at image build with `proctor check`: `SITE/🚀️deploy/Dockerfile:47-57` (`find "🏛️architecture" -path "*/❓️quiz/🔣️.json"` into `/stage/content`,
  `RUN /out/release/proctor check "/stage/content/🏛️architecture/❓️quiz/🔣️.json"`), `PROCTOR_CATALOG="/srv/quiz/content/🏛️architecture/❓️quiz/🔣️.json"` (l.78). The build context admits `!🎓️teaching/**/❓️quiz/🔣️.json` and `**/*.rs`
  (`Dockerfile.dockerignore`). **The site script and the proctor image must be released together whenever the quiz JSON gains a field**:
  the Rust core (schemas `additionalProperties: false`, every `$defs` object has `additionalProperties:false`) refuses unknown
  authoring fields. **[infer for the exact behavior; the schema property is read]**
- **`deploy-check` (8 steps)** [read: `SITE/🚀️deploy/🟦️.ts:936-985`]: (1) no drift from `🚀️deploy/🔣️.json`; (2) sources type-checked
  (`@teaching/architecture-quiz:typecheck`, covers the site entry, vite config, stack, deploy verbs, e2e gate and every test/spec,
  so a driver or `SourceTask` change must compile); (3) catalog valid in the Rust core (`checkQuizCatalog` = `proctor check`
  with the release binary over `SITE/🔣️.json`); (4) site built and verified as the CDN artifact (`publishQuizSite`: entry
  points, sealed CSP document, hashed assets, no source maps, size budget `QUIZ_SITE_BUDGET = { scriptGzipBytes: 260_000,
  styleGzipBytes: 60_000, totalBytes: 4_000_000 }` (`🟦️.ts:100`); material with solutions is inside the script chunk, so more
  authored text raises it); (5) proctor image built; (6) image checked (labels, user, no shell, stack files, API as the CDN site meets it:
  preflight, POST, presence, drain); (7) stack checked (compose, Caddy, backup/restore); (8) end-to-end gate. Needs Docker.
  `deploymentWarnings` warns while `site.legal` names no imprint/privacy page.
- **Staged bundles** [ran: `ls`]: `.🧬semio/🎓️teaching/architecture-quiz-poc/` contains `STAGED.txt` (staged 2026-10-02T18:55:52Z from
  `202c4b7b...` "with the working tree", site 63 files / 2680424 bytes, proctor 5 files, image
  `sha256:ac696220... 202c4b7b...-dirty 2026-10-02T17:04:36+02:00`), `proctor/` (`.env`, `Caddyfile`, `compose.yaml`, `README.txt`,
  `certificates/`, `proctor-image.tar` 33 233 408 bytes) and `site/` which is **empty right now** (I listed it: no files, last modified 21:47,
  although `STAGED.txt` still says 63 files). A contract change makes the proctor image tar stale (built before the sorting-guess
  and navbar work of the same day? image created 17:04; I did not inspect which Rust sources it contains) and the staged site stale;
  both must be rebuilt: `bun nx run @teaching/architecture-quiz:publish` (site, empties `dist`, then `docker-stack-bundle` for the
  proctor, "a site build empties `dist`, so stage the bundle after `publish`", README + memory).
- **Storage format** [read]: proctor SQLite `PROCTOR_STORAGE_FORMAT = { schema: "semio.teaching.proctor.sqlite", version: 2 }`
  (`🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts:70`; Rust `FORMAT_SCHEMA`/`FORMAT_VERSION` in `🔨️modules/🗄️storage/🦀️.rs:62`, table `proctor_format`).
  A database of another version is refused at open, "by design without migration"; `dev` and `@teaching/proctor:dev` move an
  old dev database aside (`.🧬semio/🎓️teaching/proctor-dev.v<format>-<time>/`). The project is greenfield (no migration allowed), so
  adding the level to `StartRun`/`RunStarted` events, the read models (`PROJECTOR_REVISION`, currently 5 per memory) and
  the leaderboard points would bump the format to 3; a dev proctor that is still running with an older build refuses new queries and must be restarted.
  The browser-side local store has **no version**: keys are `semio.quiz.<tenant>.<slice>` and `semio.quiz.<tenant>.<collection>/<id>`
  (`PRODUCT/🎯️targets/⚛️react/🔨️modules/💾️persistence/🟦️.ts:153-156`); stored runs/preferences from before the change just have no
  level - a new reader must tolerate absence or the dev must clear site data (no legacy code is allowed by `AGENTS.md`, so
  prefer clearing and a version in the key). **[infer]**
- **Other gates that read content:** `@teaching/proctor:test`/`check` (`SITE` `check` target = `proctor check` over the catalog), parity
  (`🧪️test` module, quiz parity 108/108 per memory) with shared vectors in `PRODUCT/🧫️fixtures`, `bun nx run @teaching/proctor:capacity`.
  CI: `.github/workflows/architecture-quiz.yml` runs `bun nx run @teaching/architecture-quiz:test`, `…:publish` (site job),
  `docker-image-build`, `docker-image-publish` (release job).
- **`🧪️tests/🧪️deploy`** (277 lines) pins the Dockerfile admission list, the workflow and the deploy-check step list (`l.258`
  asserts the 8 labels in order); it does not look at quiz fields, but adding/removing a step or a deployable file breaks it.

---

## 6. Dev workflow rows

### 6.1 Rows that exist on disk [read]

`.vscode/launch.json` (VS Code, group `3_dev`):

| name | line | command | env | order |
|---|---|---|---|---|
| `🛠️dev🎓️teaching🏛️architecture❓️quiz` | 4172 | `bun nx run @teaching/architecture-quiz:dev` (proctor + site) | `TEACHING_ARCHITECTURE_QUIZ_PORT=6061`, `PROCTOR_PORT=8791` | 213.6; `serverReadyAction` opens `http://127.0.0.1:6061` |
| `🛠️dev🎓️teaching🏛️architecture❓️quiz🌐️site` | 4191 | `bun nx run @teaching/architecture-quiz:dev-site` | same two | 213.605 |
| `🛠️dev🎓️teaching🛂️proctor` | 4193 | `bun nx run @teaching/proctor:dev` | `PROCTOR_PORT=8791` | 213.61 |
| `🧭️compound🎓️teaching🏛️architecture❓️quiz🛂️proctor` | 25181 | compound of the proctor row and the quiz dev row | - | 213.62 |
| `🧪️test🎓️teaching🏛️architecture❓️quiz` | 4269 | `bun nx run @teaching/architecture-quiz:test` | | |
| `🛠️dev🎓️teaching🏛️architecture❓️quiz🪁️typecheck` | 4290 | `...:typecheck` | | |
| `📦️build…❓️quiz` / `🚚️publish…❓️quiz` | 6707 / 6718 | `...:build`, `...:publish` | | |
| `📦️build…🐳️docker-image`, `🚚️publish…🐳️docker-image`, `📦️bundle…🐳️docker-stack` | 6740 / 6751 / 6761 | | | |
| `✅️check…📚️catalog` | 6763 | `...:check` | | |
| `⚖️gate…🎭️e2e` | 6785 | `...:test-e2e` | | |
| `⚖️gate…🐳️docker-image`, `⚖️gate…🐳️docker-stack`, `⚖️gate…🚀️deploy` | 6787 / 6798 / 6808 | `docker-image-check`, `docker-stack-check`, `deploy-check` | | |

`.claude/launch.json` (preview launcher), lines 541-583 [read]:

| name | command | port | env |
|---|---|---|---|
| `teaching-proctor` | `bun nx run @teaching/proctor:dev` | 8791 | `PROCTOR_PORT=8791` |
| `architecture-quiz-site` | `bun nx run @teaching/architecture-quiz:dev-site` | 6061 | `TEACHING_ARCHITECTURE_QUIZ_PORT=6061`, `PROCTOR_PORT=8791` |
| `architektur-und-technologie-quizze` | `bun nx run @teaching/architecture-quiz:dev` | 6061 | same two |

Ports: site 6061 (`semio.app.port.dev` in `📦️packages/🟦️typescript/package.json`, Nx `dev` env pins `TEACHING_ARCHITECTURE_QUIZ_PORT=6061`),
proctor 8791 (`deployment.proctor.port`). E2E uses 6161/8891 and 6162/8892.

### 6.2 `…-steady` and `…-beside` variants: they are NOT on disk **[read; surprising]**

You asked for `architektur-und-technologie-quizze-steady` and `…-beside`. The memory notes of earlier sessions say they exist
(`-steady`: `TEACHING_ARCHITECTURE_QUIZ_WATCH=off`; `-beside`: site 6063, proctor 8793, own `PROCTOR_DATA`, runs the site script
directly). I grepped `-i "steady\|beside"` in both launch files: no hits. `git show HEAD:.claude/launch.json` also has only the three
rows above (`git status` shows `.claude/launch.json` unchanged vs the index; last modified Oct 2 17:26). A ticket report
(`QUIZ-PERIOD-LEADERBOARDS/📓️report.md:83`) says the `-beside` row was added, so it was presumably lost with a later overwrite/pull.
The two variants are therefore **absent** and must be re-created if the coordinator needs them. The mechanisms they relied on do
exist: `TEACHING_ARCHITECTURE_QUIZ_WATCH=off` is honoured by `SITE/🏗️builder/🌐️vite/🟦️.ts:138` (`hmr:false, watch:null`) and
`TEACHING_ARCHITECTURE_QUIZ_CACHE` moves Vite's cache (l.113); `PROCTOR_PORT`/`PROCTOR_DATA` are read by the proctor launcher
(`🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts`). The Nx `dev` target pins `TEACHING_ARCHITECTURE_QUIZ_PORT` (`📋️project.json` dev
`env`), so a second stack has to run `bun ./📜️script.ts dev-site` directly (as the lost `-beside` row did). **[infer for the
re-creation; the pinned env is read]**. `AGENTS.md` demands that executable commands are registered in `launch.json` following
the existing grouping, so any new row belongs in both files.

---

## 7. Other facts worth knowing

- The site README table of specs lists 11 spec folders; `🧪️tests` actually has 17 test folders (the extra: `🐾️pet-cast` and
  `🧪️deploy`, `🧱️local-stack`, `📰️host-document`, `🧪️catalog` as node tests; `🎚️config` is the vitest config). **[read]**
- `bun` in the preview launcher is 1.3.13, the shell's 1.4.2 (memory) - not re-verified.
- Sorting results show a "Your guess" column when the learner guessed (ticket `QUIZ-SORTING-NUMERIC-GUESSES`, `📓️design.md`
  contract: `SortingAnswer.guesses` in the base unit; two cores and a Python reference carry it). That ticket closed the same day.
  Any hard level that scores guessed numbers builds on it but changes its "never enters scoring" rule.
- Staged-site discrepancy: `STAGED.txt` says the site was staged with 63 files at 18:55 but `…/architecture-quiz-poc/site` is empty
  at the time of this exploration (last write 21:47). `📦️packages/🟦️typescript/dist/` and `dist/pages/quizzes/` hold a built site
  (index.html, hashed assets in both) - a `publish` output of a session; not compared with HEAD.

---

## Appendix: one complete task per kind, verbatim from disk

Extracted with `sed` line ranges from the current files (the lines are exactly as on disk; the trailing comma after the demand
task belongs to the array it sits in).

### A. classification with spider diagrams: `ENERGY/📊️demand/❓️quiz/🔣️.json` lines 12-151, task `standard-profiles`

```json
    {
      "kind": "classification",
      "id": "standard-profiles",
      "title": { "en": "Energy Standards and Their Profiles", "de": "Energiestandards und ihre Profile" },
      "icon": { "emoji": "🕸️", "motion": "sway" },
      "prompt": {
        "en": "Each spider diagram shows the typical profile of one residential energy standard per m² of floor area and year: heating demand, cooling demand, ventilation heat loss and net energy costs. Assign each standard to its profile.",
        "de": "Jedes Netzdiagramm zeigt das typische Profil eines Wohngebäude-Energiestandards je m² Fläche und Jahr: Heizwärmebedarf, Kühlbedarf, Lüftungswärmeverlust und Netto-Energiekosten. Ordne jedem Standard sein Profil zu."
      },
      "axes": [
        {
          "id": "heating",
          "label": { "en": "Heating demand", "de": "Heizwärmebedarf" },
          "unit": "kWh/(m²·a)",
          "min": 0,
          "max": 350
        },
        {
          "id": "cooling",
          "label": { "en": "Cooling demand to keep 26\u00a0°C", "de": "Kühlbedarf für 26\u00a0°C" },
          "unit": "kWh/(m²·a)",
          "min": 0,
          "max": 10
        },
        {
          "id": "ventilation",
          "label": { "en": "Ventilation heat loss (air leakage plus ventilation after heat recovery)", "de": "Lüftungswärmeverlust (Undichtheit plus Lüftung nach Wärmerückgewinnung)" },
          "unit": "kWh/(m²·a)",
          "min": 0,
          "max": 70
        },
        {
          "id": "costs",
          "label": { "en": "Net energy costs for heating, hot water and auxiliary energy (after PV credit)", "de": "Netto-Energiekosten für Heizung, Warmwasser und Hilfsenergie (nach PV-Gutschrift)" },
          "unit": "€/(m²·a)",
          "min": -5,
          "max": 55
        }
      ],
      "categories": [
        {
          "id": "profile-a",
          "label": { "en": "Profile A", "de": "Profil A" },
          "icon": { "emoji": "🔺", "motion": "pulse" },
          "profile": { "heating": 26, "cooling": 4, "ventilation": 9, "costs": 4.2 }
        },
        {
          "id": "profile-b",
          "label": { "en": "Profile B", "de": "Profil B" },
          "icon": { "emoji": "⬛", "motion": "sway" },
          "profile": { "heating": 303, "cooling": 8, "ventilation": 61, "costs": 50 }
        },
        {
          "id": "profile-c",
          "label": { "en": "Profile C", "de": "Profil C" },
          "icon": { "emoji": "⚫", "motion": "bounce" },
          "profile": { "heating": 20, "cooling": 7, "ventilation": 8, "costs": -2 }
        },
        {
          "id": "profile-d",
          "label": { "en": "Profile D", "de": "Profil D" },
          "icon": { "emoji": "🔷", "motion": "float" },
          "profile": { "heating": 135, "cooling": 6, "ventilation": 43, "costs": 23 }
        },
        {
          "id": "profile-e",
          "label": { "en": "Profile E", "de": "Profil E" },
          "icon": { "emoji": "⭐", "motion": "spin" },
          "profile": { "heating": 15, "cooling": 2, "ventilation": 6.5, "costs": 3.7 }
        },
        {
          "id": "profile-f",
          "label": { "en": "Profile F", "de": "Profil F" },
          "icon": { "emoji": "🔻", "motion": "flip" },
          "profile": { "heating": 55, "cooling": 5, "ventilation": 32, "costs": 10 }
        }
      ],
      "items": [
        {
          "id": "unrenovated-old-building",
          "label": { "en": "Unrenovated old building (1960s single-family house)", "de": "Unsanierter Altbau (Einfamilienhaus der 1960er-Jahre)" },
          "icon": { "emoji": "🏚️", "motion": "sway" },
          "category": "profile-b",
          "explanation": {
            "en": "Profile B: about 300\u00a0kWh/(m²·a) heating (type EFH_E of the IWU, the Institute for Housing and Environment), a leaky envelope (n50\u00a0≈\u00a08\u00a01/h) losing about 60\u00a0kWh/(m²·a) through air exchange, an uninsulated roof that heats up in summer, and about 50\u00a0€/(m²·a) for gas at 12\u00a0ct/kWh.",
            "de": "Profil B: rund 300\u00a0kWh/(m²·a) Heizwärme (IWU-Typ EFH_E), eine undichte Hülle (n50\u00a0≈\u00a08\u00a01/h) mit rund 60\u00a0kWh/(m²·a) Lüftungswärmeverlust, ein ungedämmtes Dach, das sich im Sommer aufheizt, und rund 50\u00a0€/(m²·a) für Gas zu 12\u00a0ct/kWh."
          }
        },
        {
          "id": "wschvo-1995",
          "label": { "en": "House to the Thermal Insulation Ordinance 1995 (WSchVO 1995)", "de": "Haus nach Wärmeschutzverordnung 1995 (WSchVO 1995)" },
          "icon": { "emoji": "📜", "motion": "sway" },
          "category": "profile-d",
          "explanation": {
            "en": "Profile D: about 135\u00a0kWh/(m²·a) heating (IWU type EFH_I), n50\u00a0≈\u00a04\u00a01/h with about 43\u00a0kWh/(m²·a) ventilation loss, and about 23\u00a0€/(m²·a) with a low-temperature gas boiler.",
            "de": "Profil D: rund 135\u00a0kWh/(m²·a) Heizwärme (IWU-Typ EFH_I), n50\u00a0≈\u00a04\u00a01/h mit rund 43\u00a0kWh/(m²·a) Lüftungswärmeverlust und rund 23\u00a0€/(m²·a) mit einem Niedertemperatur-Gaskessel."
          }
        },
        {
          "id": "enev-2014",
          "label": { "en": "New build to EnEV 2014", "de": "Neubau nach EnEV 2014" },
          "icon": { "emoji": "🏠", "motion": "pulse" },
          "category": "profile-f",
          "explanation": {
            "en": "Profile F: about 55\u00a0kWh/(m²·a) heating (slightly above today's minimum envelope, which the Energy Saving Ordinance (EnEV) 2016 tightened by about 20%), airtight to n50\u00a0≈\u00a01.5\u00a01/h but ventilated by windows or exhaust fans without heat recovery (about 32\u00a0kWh/(m²·a)), and about 10\u00a0€/(m²·a) with a gas condensing boiler.",
            "de": "Profil F: rund 55\u00a0kWh/(m²·a) Heizwärme (etwas über der heutigen Mindest-Hülle, die die EnEV 2016 um rund 20\u00a0% verschärfte), luftdicht bis n50\u00a0≈\u00a01,5\u00a01/h, aber über Fenster oder Abluftventilatoren ohne Wärmerückgewinnung gelüftet (rund 32\u00a0kWh/(m²·a)), und rund 10\u00a0€/(m²·a) mit einem Gas-Brennwertkessel."
          }
        },
        {
          "id": "kfw-40",
          "label": { "en": "KfW Efficiency House 40", "de": "KfW-Effizienzhaus 40" },
          "icon": { "emoji": "🏡", "motion": "float" },
          "category": "profile-a",
          "explanation": {
            "en": "Profile A, a funding standard of KfW (German state development bank): about 26\u00a0kWh/(m²·a) heating (IWU 2015, Table 26), heat-recovery ventilation cuts the ventilation loss to about 9\u00a0kWh/(m²·a), and a heat pump brings the costs down to about 4\u00a0€/(m²·a).",
            "de": "Profil A: rund 26\u00a0kWh/(m²·a) Heizwärme (IWU 2015, Tabelle 26), die Lüftung mit Wärmerückgewinnung senkt den Lüftungswärmeverlust auf rund 9\u00a0kWh/(m²·a), und eine Wärmepumpe drückt die Kosten auf rund 4\u00a0€/(m²·a)."
          }
        },
        {
          "id": "passive-house",
          "label": { "en": "Passive house", "de": "Passivhaus" },
          "icon": { "emoji": "🌱", "motion": "sway" },
          "category": "profile-e",
          "explanation": {
            "en": "Profile E: at most 15\u00a0kWh/(m²·a) heating and n50\u00a0≤\u00a00.6\u00a01/h with at least 75% heat recovery (Passive House Institute), about 6.5\u00a0kWh/(m²·a) ventilation loss, the lowest cooling demand thanks to planned shading, and about 4\u00a0€/(m²·a) with a compact heat pump.",
            "de": "Profil E: höchstens 15\u00a0kWh/(m²·a) Heizwärme und n50\u00a0≤\u00a00,6\u00a01/h mit mindestens 75\u00a0% Wärmerückgewinnung (Passivhaus Institut), rund 6,5\u00a0kWh/(m²·a) Lüftungswärmeverlust, der kleinste Kühlbedarf dank geplanter Verschattung und rund 4\u00a0€/(m²·a) mit einem Kompaktgerät mit Wärmepumpe."
          }
        },
        {
          "id": "plus-energy-house",
          "label": { "en": "Plus-energy house", "de": "Plusenergiehaus" },
          "icon": { "emoji": "➕", "motion": "pulse" },
          "category": "profile-c",
          "explanation": {
            "en": "Profile C: a passive-house-like envelope with heat pump and a large PV roof that exports more electricity than the building buys, so the net costs are negative (about −2\u00a0€/(m²·a) at 30\u00a0ct purchase and 8\u00a0ct feed-in); larger glazing raises the cooling demand.",
            "de": "Profil C: eine passivhausähnliche Hülle mit Wärmepumpe und einem großen PV-Dach, das mehr Strom einspeist, als das Gebäude bezieht, daher sind die Nettokosten negativ (rund −2\u00a0€/(m²·a) bei 30\u00a0ct Bezug und 8\u00a0ct Einspeisung); größere Verglasung erhöht den Kühlbedarf."
          }
        }
      ]
    },
```

### B. sorting: `ENERGY/🧲️physics/❓️quiz/🔣️.json` lines 353-486, task `energies`

```json
    {
      "kind": "sorting",
      "id": "energies",
      "title": { "en": "From Phone Charge to World Energy Use", "de": "Von der Handyladung zum Weltenergieverbrauch" },
      "icon": { "emoji": "🔋", "motion": "bounce" },
      "prompt": {
        "en": "Sort the amounts of energy in ascending order, smallest first. Quantity: energy in watt-hours (Wh); the values span about 16 orders of magnitude.",
        "de": "Sortiere die Energiemengen aufsteigend, die kleinste zuerst. Größe: Energie in Wattstunden (Wh); die Werte umfassen rund 16 Größenordnungen."
      },
      "quantity": { "label": { "en": "Energy", "de": "Energie" }, "unit": "Wh", "scale": "logarithmic", "prefixed": true },
      "items": [
        {
          "id": "phone-charge",
          "label": { "en": "One full smartphone charge", "de": "Eine volle Smartphone-Ladung" },
          "icon": { "emoji": "📱", "motion": "bounce" },
          "value": 15,
          "explanation": {
            "en": "About 15\u00a0Wh: a 4,000\u00a0mAh battery at 3.85\u00a0V stores 15.4\u00a0Wh.",
            "de": "Rund 15\u00a0Wh: ein Akku mit 4.000\u00a0mAh bei 3,85\u00a0V speichert 15,4\u00a0Wh."
          }
        },
        {
          "id": "boil-water",
          "label": { "en": "Heating 1\u00a0litre of water from 20\u00a0°C to boiling", "de": "1\u00a0Liter Wasser von 20\u00a0°C zum Kochen bringen" },
          "icon": { "emoji": "♨️", "motion": "pulse" },
          "value": 93,
          "explanation": {
            "en": "About 93\u00a0Wh: 4.19\u00a0kJ/(kg·K)\u00a0×\u00a01\u00a0kg\u00a0×\u00a080\u00a0K\u00a0=\u00a0335\u00a0kJ; a 2\u00a0kW kettle needs about 3\u00a0minutes plus losses.",
            "de": "Rund 93\u00a0Wh: 4,19\u00a0kJ/(kg·K)\u00a0×\u00a01\u00a0kg\u00a0×\u00a080\u00a0K\u00a0=\u00a0335\u00a0kJ; ein 2-kW-Wasserkocher braucht dafür knapp 3\u00a0Minuten plus Verluste."
          }
        },
        {
          "id": "chocolate-bar",
          "label": { "en": "Food energy of a 100\u00a0g chocolate bar", "de": "Brennwert einer 100-g-Tafel Schokolade" },
          "icon": { "emoji": "🍫", "motion": "flip" },
          "value": 628,
          "explanation": {
            "en": "About 540\u00a0kcal\u00a0≈\u00a0628\u00a0Wh (1\u00a0kcal\u00a0=\u00a01.163\u00a0Wh) according to the nutrition label.",
            "de": "Rund 540\u00a0kcal\u00a0≈\u00a0628\u00a0Wh (1\u00a0kcal\u00a0=\u00a01,163\u00a0Wh) laut Nährwerttabelle."
          }
        },
        {
          "id": "daily-food",
          "label": { "en": "Daily food energy of an adult", "de": "Täglicher Nahrungsenergiebedarf eines Erwachsenen" },
          "icon": { "emoji": "🍽️", "motion": "sway" },
          "value": 2700,
          "explanation": {
            "en": "About 2,300\u00a0kcal\u00a0≈\u00a02.7\u00a0kWh (reference values of the German Nutrition Society, DGE, for moderate activity, about 1,900–2,500\u00a0kcal); it keeps 100\u00a0W of body heat going for 24\u00a0hours.",
            "de": "Rund 2.300\u00a0kcal\u00a0≈\u00a02,7\u00a0kWh (DGE-Referenzwerte bei mäßiger Aktivität, etwa 1.900–2.500\u00a0kcal); damit laufen 100\u00a0W Körperwärme 24\u00a0Stunden lang."
          }
        },
        {
          "id": "heating-oil-litre",
          "label": { "en": "One litre of heating oil", "de": "Ein Liter Heizöl" },
          "icon": { "emoji": "🛢️", "motion": "sway" },
          "value": 10000,
          "explanation": {
            "en": "About 10\u00a0kWh: net calorific value 42.6\u00a0MJ/kg\u00a0×\u00a00.845\u00a0kg/l (DIN 51603-1).",
            "de": "Rund 10\u00a0kWh: Heizwert 42,6\u00a0MJ/kg\u00a0×\u00a00,845\u00a0kg/l (DIN 51603-1)."
          }
        },
        {
          "id": "ev-battery",
          "label": { "en": "Usable capacity of an electric car battery", "de": "Nutzbare Kapazität eines E-Auto-Akkus" },
          "icon": { "emoji": "🚙", "motion": "bounce" },
          "value": 60000,
          "explanation": {
            "en": "About 60\u00a0kWh (e.g. VW ID.3: 58\u00a0kWh net), enough for roughly 350\u00a0km.",
            "de": "Rund 60\u00a0kWh (z. B. VW ID.3: 58\u00a0kWh netto), genug für rund 350\u00a0km."
          }
        },
        {
          "id": "petrol-tank",
          "label": { "en": "A full 50-litre tank of petrol", "de": "Eine volle 50-Liter-Tankfüllung Benzin" },
          "icon": { "emoji": "⛽", "motion": "pulse" },
          "value": 440000,
          "explanation": {
            "en": "About 440\u00a0kWh: 50\u00a0l\u00a0×\u00a08.8\u00a0kWh/l (42.7\u00a0MJ/kg\u00a0×\u00a00.745\u00a0kg/l), seven times the car battery, yet an engine turns only about a quarter of it into motion.",
            "de": "Rund 440\u00a0kWh: 50\u00a0l\u00a0×\u00a08,8\u00a0kWh/l (42,7\u00a0MJ/kg\u00a0×\u00a00,745\u00a0kg/l), siebenmal der Autoakku, doch ein Motor setzt nur etwa ein Viertel davon in Bewegung um."
          }
        },
        {
          "id": "household-electricity",
          "label": { "en": "Annual electricity use of a two-person household", "de": "Jahresstromverbrauch eines Zwei-Personen-Haushalts" },
          "icon": { "emoji": "💡", "motion": "pulse" },
          "value": 2500000,
          "explanation": {
            "en": "About 2.5\u00a0MWh (2,500\u00a0kWh) per year (Stromspiegel für Deutschland).",
            "de": "Rund 2,5\u00a0MWh (2.500\u00a0kWh) pro Jahr (Stromspiegel für Deutschland)."
          }
        },
        {
          "id": "heating-demand-house",
          "label": { "en": "Annual heating demand of an unrenovated 1960s single-family house", "de": "Jahres-Heizwärmebedarf eines unsanierten Einfamilienhauses der 1960er-Jahre" },
          "icon": { "emoji": "🏚️", "motion": "sway" },
          "value": 33000000,
          "explanation": {
            "en": "About 33\u00a0MWh: 110\u00a0m²\u00a0×\u00a0303\u00a0kWh/(m²·a) (IWU German residential building typology 2015, type EFH_E), thirteen times the household electricity.",
            "de": "Rund 33\u00a0MWh: 110\u00a0m²\u00a0×\u00a0303\u00a0kWh/(m²·a) (IWU Deutsche Wohngebäudetypologie 2015, Typ EFH_E), dreizehnmal der Haushaltsstrom."
          }
        },
        {
          "id": "wind-turbine-year",
          "label": { "en": "Annual yield of a modern onshore wind turbine", "de": "Jahresertrag einer modernen Windenergieanlage an Land" },
          "icon": { "emoji": "💨", "motion": "float" },
          "value": 10000000000,
          "explanation": {
            "en": "About 10\u00a0GWh: 5\u00a0MW\u00a0×\u00a0about 2,000\u00a0full-load hours, the electricity of about 4,000 two-person households.",
            "de": "Rund 10\u00a0GWh: 5\u00a0MW\u00a0×\u00a0rund 2.000\u00a0Volllaststunden, der Strom von rund 4.000 Zwei-Personen-Haushalten."
          }
        },
        {
          "id": "germany-primary-energy",
          "label": { "en": "Germany's annual primary energy consumption", "de": "Jährlicher Primärenergieverbrauch Deutschlands" },
          "icon": { "emoji": "🏙️", "motion": "pulse" },
          "value": 2925000000000000,
          "explanation": {
            "en": "About 2.925\u00a0PWh (2,925\u00a0TWh): 10,529\u00a0PJ in 2024 (AG Energiebilanzen 2025).",
            "de": "Rund 2,925\u00a0PWh (2.925\u00a0TWh): 10.529\u00a0PJ im Jahr 2024 (AG Energiebilanzen 2025)."
          }
        },
        {
          "id": "world-primary-energy",
          "label": { "en": "The world's annual primary energy consumption", "de": "Jährlicher Primärenergieverbrauch der Welt" },
          "icon": { "emoji": "🌍", "motion": "spin" },
          "value": 164400000000000000,
          "explanation": {
            "en": "About 164.4\u00a0PWh (164,400\u00a0TWh): 592\u00a0EJ in 2024 (Energy Institute, Statistical Review of World Energy 2025). The Sun delivers this much to the Earth in about one hour.",
            "de": "Rund 164,4\u00a0PWh (164.400\u00a0TWh): 592\u00a0EJ im Jahr 2024 (Energy Institute, Statistical Review of World Energy 2025). So viel liefert die Sonne der Erde in etwa einer Stunde."
          }
        }
      ],
      "draw": 9
    }
```

### C. matching with two dimensions: `ENERGY/❄️cooling/❓️quiz/🔣️.json` lines 182-296, task `cooling-load-and-demand`

```json
    {
      "kind": "matching",
      "id": "cooling-load-and-demand",
      "title": { "en": "Cooling Load and Cooling Demand", "de": "Kühllast und Kühlbedarf" },
      "icon": { "emoji": "❄️", "motion": "spin" },
      "prompt": {
        "en": "Assign each building its specific cooling load in W/m² (the heat to remove on a hot design day, VDI 2078) and its annual cooling demand in kWh/(m²·a) (the useful cooling per year to keep about 26\u00a0°C), both per m² of floor area.",
        "de": "Ordne jedem Gebäude seine spezifische Kühllast in W/m² (die an einem heißen Auslegungstag abzuführende Wärme, VDI 2078) und seinen Jahres-Kühlbedarf in kWh/(m²·a) (die Nutzkälte pro Jahr, um etwa 26\u00a0°C zu halten) zu, beide je m² Fläche."
      },
      "dimensions": [
        {
          "id": "cooling-load",
          "icon": { "emoji": "🧊", "motion": "pulse" },
          "quantity": { "label": { "en": "Specific cooling load", "de": "Spezifische Kühllast" }, "unit": "W/m²", "scale": "logarithmic", "prefixed": false }
        },
        {
          "id": "cooling-demand",
          "icon": { "emoji": "📅", "motion": "flip" },
          "quantity": { "label": { "en": "Annual cooling demand", "de": "Jahres-Kühlbedarf" }, "unit": "kWh/(m²·a)", "scale": "logarithmic", "prefixed": false }
        }
      ],
      "items": [
        {
          "id": "passive-house-home",
          "label": { "en": "Passive-house home with external shading", "de": "Passivhaus-Wohngebäude mit außenliegendem Sonnenschutz" },
          "icon": { "emoji": "🌱", "motion": "sway" },
          "values": { "cooling-load": 6, "cooling-demand": 2 },
          "explanation": {
            "en": "About 6\u00a0W/m² and 2\u00a0kWh/(m²·a): shading and low internal gains keep it comfortable without a chiller; the Passive House Institute limits overheating above 25\u00a0°C to 10% of the hours.",
            "de": "Rund 6\u00a0W/m² und 2\u00a0kWh/(m²·a): Verschattung und geringe innere Lasten halten es ohne Kältemaschine behaglich; das Passivhaus Institut begrenzt Überhitzung über 25\u00a0°C auf 10\u00a0% der Stunden."
          }
        },
        {
          "id": "new-home-geg",
          "label": { "en": "New home to GEG 2024 with external blinds (DIN 4108-2)", "de": "Neues Wohngebäude nach GEG 2024 mit Außenjalousien (DIN 4108-2)" },
          "icon": { "emoji": "🏠", "motion": "pulse" },
          "values": { "cooling-load": 15, "cooling-demand": 5 },
          "explanation": {
            "en": "About 15\u00a0W/m² and 5\u00a0kWh/(m²·a): the summer heat protection of DIN 4108-2, which the GEG (German Buildings Energy Act) requires, limits solar gains, so cooling is needed only on a few hot days (≈\u00a0350\u00a0full-load hours).",
            "de": "Rund 15\u00a0W/m² und 5\u00a0kWh/(m²·a): der sommerliche Wärmeschutz nach DIN 4108-2 begrenzt die Solargewinne, gekühlt wird nur an wenigen heißen Tagen (≈\u00a0350\u00a0Volllaststunden)."
          }
        },
        {
          "id": "school-new-build",
          "label": { "en": "New school building (GEG), cooled", "de": "Neues Schulgebäude (GEG), gekühlt" },
          "icon": { "emoji": "🏫", "motion": "pulse" },
          "values": { "cooling-load": 30, "cooling-demand": 8 },
          "explanation": {
            "en": "About 30\u00a0W/m² and 8\u00a0kWh/(m²·a): many people per m² make the load high, but the summer holidays fall into the hottest weeks (≈\u00a0250\u00a0full-load hours).",
            "de": "Rund 30\u00a0W/m² und 8\u00a0kWh/(m²·a): viele Menschen je m² machen die Last hoch, doch die Sommerferien fallen in die heißesten Wochen (≈\u00a0250\u00a0Volllaststunden)."
          }
        },
        {
          "id": "office-passive-house",
          "label": { "en": "Passive-house office building", "de": "Bürogebäude im Passivhausstandard" },
          "icon": { "emoji": "🏢", "motion": "sway" },
          "values": { "cooling-load": 20, "cooling-demand": 10 },
          "explanation": {
            "en": "About 20\u00a0W/m² and 10\u00a0kWh/(m²·a): efficient equipment, external shading and night ventilation keep the load low (≈\u00a0500\u00a0full-load hours). Source: Passive House Institute criteria for non-residential buildings.",
            "de": "Rund 20\u00a0W/m² und 10\u00a0kWh/(m²·a): effiziente Geräte, außenliegender Sonnenschutz und Nachtlüftung halten die Last klein (≈\u00a0500\u00a0Volllaststunden). Quelle: Kriterien des Passivhaus Instituts für Nichtwohngebäude."
          }
        },
        {
          "id": "office-geg-shading",
          "label": { "en": "New office building (GEG) with external sun shading", "de": "Neues Bürogebäude (GEG) mit außenliegendem Sonnenschutz" },
          "icon": { "emoji": "⛱️", "motion": "sway" },
          "values": { "cooling-load": 40, "cooling-demand": 20 },
          "explanation": {
            "en": "About 40\u00a0W/m² and 20\u00a0kWh/(m²·a): people, computers and lighting add about 25\u00a0W/m² to the shaded solar gains; offices see about 500\u00a0full-load hours of cooling (VDI 2078 practice).",
            "de": "Rund 40\u00a0W/m² und 20\u00a0kWh/(m²·a): Menschen, Computer und Licht bringen rund 25\u00a0W/m² zu den verschatteten Solargewinnen; Büros kommen auf etwa 500\u00a0Kühl-Volllaststunden (Praxis nach VDI 2078)."
          }
        },
        {
          "id": "attic-flat",
          "label": { "en": "Attic flat under an uninsulated roof with unshaded roof windows", "de": "Dachgeschosswohnung unter ungedämmtem Dach mit unverschatteten Dachfenstern" },
          "icon": { "emoji": "🔆", "motion": "pulse" },
          "values": { "cooling-load": 50, "cooling-demand": 25 },
          "explanation": {
            "en": "About 50\u00a0W/m² and 25\u00a0kWh/(m²·a): the sun heats the roof surface to 60–70\u00a0°C (sol-air temperature, VDI 2078), and the heat passes the uninsulated roof almost unhindered.",
            "de": "Rund 50\u00a0W/m² und 25\u00a0kWh/(m²·a): die Sonne heizt die Dachhaut auf 60–70\u00a0°C auf (Sonnenlufttemperatur, VDI 2078), und die Wärme gelangt fast ungehindert durch das ungedämmte Dach."
          }
        },
        {
          "id": "hospital",
          "label": { "en": "Hospital with operating rooms and intensive care", "de": "Krankenhaus mit OP-Sälen und Intensivstation" },
          "icon": { "emoji": "🏥", "motion": "pulse" },
          "values": { "cooling-load": 70, "cooling-demand": 90 },
          "explanation": {
            "en": "About 70\u00a0W/m² and 90\u00a0kWh/(m²·a): medical equipment, strict room conditions (DIN 1946-4) and 24-hour operation add up to about 1,300\u00a0full-load hours.",
            "de": "Rund 70\u00a0W/m² und 90\u00a0kWh/(m²·a): Medizintechnik, strenge Raumbedingungen (DIN 1946-4) und Betrieb rund um die Uhr ergeben etwa 1.300\u00a0Volllaststunden."
          }
        },
        {
          "id": "office-1970s",
          "label": { "en": "1970s office tower, fully glazed, without external shading", "de": "Bürohochhaus der 1970er-Jahre, vollverglast, ohne außenliegenden Sonnenschutz" },
          "icon": { "emoji": "🏙️", "motion": "float" },
          "values": { "cooling-load": 100, "cooling-demand": 60 },
          "explanation": {
            "en": "About 100\u00a0W/m² and 60\u00a0kWh/(m²·a): unshaded glass lets in several hundred watts per m² of façade; with about 600\u00a0full-load hours that is three times the demand of a shaded new office.",
            "de": "Rund 100\u00a0W/m² und 60\u00a0kWh/(m²·a): unverschattetes Glas lässt mehrere hundert Watt je m² Fassade herein; mit etwa 600\u00a0Volllaststunden ist das der dreifache Bedarf eines verschatteten Neubaubüros."
          }
        },
        {
          "id": "data-centre",
          "label": { "en": "Data centre (server room floor)", "de": "Rechenzentrum (Serverraumfläche)" },
          "icon": { "emoji": "🖥️", "motion": "pulse" },
          "values": { "cooling-load": 1000, "cooling-demand": 8000 },
          "explanation": {
            "en": "About 1,000\u00a0W/m² and 8,000\u00a0kWh/(m²·a): every watt of IT power turns into heat, around the clock (≈\u00a08,000\u00a0full-load hours).",
            "de": "Rund 1.000\u00a0W/m² und 8.000\u00a0kWh/(m²·a): jedes Watt IT-Leistung wird zu Wärme, rund um die Uhr (≈\u00a08.000\u00a0Volllaststunden)."
          }
        }
      ],
      "draw": 7
    }
```
