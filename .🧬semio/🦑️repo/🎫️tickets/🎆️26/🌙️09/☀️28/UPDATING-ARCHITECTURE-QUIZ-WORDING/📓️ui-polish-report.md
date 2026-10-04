# 📓️ UI polish report

Work package "UI polish" of 2026-10-02 in ticket `2026/09/28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR`: no layout shift from
the crowd, type checks that are real gates, old development data that strands nobody, the browser measurements the
unit tests cannot make, and a refused sign-up said clearly. Every number below was measured on this host (Windows 11,
bun 1.4.2 and 1.3.13, Chromium of the repository's Playwright); every command is quoted with its result.

Paths are relative to the repository root; `<ticket>` is this folder, `<react>` is
`🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`, `<quiz>` is `🧰️framework/🛍️products/❓️quiz`, `<site>` is
`🎓️teaching/🏛️architecture/❓️quiz`.

## 1. No layout shift from the crowd

**Decision.** Where the others' answers appear, their place exists before they do, with a size that depends on the text
size alone (`lh` units), and only while the learner shows what the others think. When the preference is off, nothing
is reserved. A place that must not move anything is exactly one size whatever is in it; what does not fit is cut on
screen and said in full to a screen reader.

**Change.**

- `<react>/🔨️modules/🗳️crowd/🟦️.tsx`: the crowd context carries `shown`; `CrowdProvider` takes `shown`; `CrowdChoices`
  and `CrowdPosition` take the item `id` and `still`, render nothing when not shown and an empty, unspoken line
  (`data-crowd-empty`, `aria-hidden`) when nobody answered. `CrowdSource` stacks its two lines ("what others think now",
  "what everyone answered") in one grid cell, the one that does not apply invisible and unspoken.
- `<react>/🎨️.css`: `.quiz-crowd { min-block-size: calc(1lh + 2px) }`, `.quiz-crowd-still { block-size: calc(1lh + 2px);
  overflow: hidden }`, `.quiz-crowd-source` (grid stack), the track of a sorting shrinks instead of overflowing.
- Classification, sorting and matching (`🗂️classification`, `↕️sorting`, `🃏️matching`): every item has its `still`
  line. The matching table is `table-fixed` with a `<colgroup>`, so a count never widens a column.
- `▶️run`, `📖️quiz-page`, `🏁️results`: pass `shown`; the quiz page keeps the place of "Learning now: n" (invisible and
  unspoken at 0).
- Results, column "Everyone" — found by the browser measurement, not by the first unit test: the column existed before
  the crowd was known, but its cells did not have a size of their own, so rows grew and, on the phone, tables widened
  when the answers of everyone arrived (39, 31, 30 and 17 boxes moved in the four measured cases; a row from 46 px to
  65 px, a table from 427 px to 509 px). Now every cell holds one box `.quiz-crowd-everyone` (16 em wide; one line for a
  place in a sorting, two lines for counts), with a dash before anyone answered. The counts that fit show, most given
  first; the sentence for screen readers says all of them.
- Peers' pointers and name labels are drawn in a layer over the page (`position: absolute` inside a fixed layer); they
  were never in the flow — now a test says so.

**Tests.** `<quiz>/🧪️tests/💭️crowd-client/🟦️.tsx`, describe "🧘️ a page that stays still while the others come and go"
(layout-independent: the same lines with the same sizing classes in every crowd state — nobody, one thinking, three
thinking, the submitted runs — for all three kinds; none when hidden; the CSS rules; the stacked source; the column
and the fixed boxes of everyone; the quiz page; "learning now"; the peers' layer). End to end:
`<site>/🧪️tests/👥️shared-presence/🟦️.ts` measures `itemBoxes(anna)` (helper in `<site>/🎭️e2e/🚶️learner/🟦️.ts`) before
Ben plays and asserts equality after Ben answered and after Ben submitted and left.

**Evidence** (`bun ui_polish_measure.ts still|results` against the stack on 6071/8801; boxes of every item, the card,
its footer and the scroll height, compared to 0.5 px):

| Screen | Cases | Boxes that moved |
|---|---|---|
| Run: classification, sorting, matching × desktop 1440 × 900 and phone 375 × 812 × text normal, large, larger, largest | 24 | 0 on join, 0 on leave in all 24 (line heights 17.97 / 19.98 / 22 / 25.98 px) |
| Overview while another learner starts and leaves a quiz, desktop and phone | 2 | 0 / 0 |
| Quiz page while another learner thinks along and leaves, desktop and phone | 2 | 0 / 0 |
| Results while the answers of everyone arrive (`none → submitted` on fresh data, then `submitted → submitted`), desktop and phone, normal and largest | 4 + 4 + 4 | 0, columns unchanged (before the fix: 39 / 31 / 30 / 17) |

The red state of the end-to-end assertion is the one `📓️dev-e2e-report.md` §6 describes; the assertion itself was
written after the fix and has only been seen green.

## 2. Typecheck is a real gate

### 2a. `@semio-tech/quiz-react:typecheck`

**Truth.** The 133 errors were not in the quiz: 83 in `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts`, 46 in the UI contract,
4 in the actor binding — all against **stale git-ignored bindings** under `🤖️generated/`, which Rust type-generation
tests write:

| Binding | Generator target |
|---|---|
| `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` | `@semio-tech/framework-rs:generate` |
| `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts` | `@semio-tech/ui-contract-rs:generate` |
| `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🎚️ui-axes/🟦️.ts` | `@semio-tech/ui-rs:generate` |
| `🧰️framework/🔨️modules/🎭️actor/🤖️generated/🎭️actor/🟦️.ts` | `@semio-tech/framework-actor-rs:typegen` |

`workspace:schema-generate` only rewrites a catalog; the earlier report that named it was wrong.

**Decision.** Depend on the generators, do not narrow the scope: the other type-check targets of the repository
check their whole import closure, and a quiz that compiles against stale framework types is not checked. Warm, the
four generators cost 2.4 s, 2.7 s, 2.2 s and 0.9 s.

**Change.** `<react>/📦️packages/🟦️typescript/📋️project.json`: `typecheck.dependsOn` the four generators plus
`@semio-tech/ui-styling-tokens:generate` and `@semio-tech/assets:build`. Three generators could not write a binding
that was missing (they `include_str!` their own output): `seedGeneratedFile` in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🗂️files/🟦️.ts`, called by
`🧰️framework/📦️packages/🦀️rust/📜️script.ts`, `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts` and
`🧰️framework/🔨️modules/🎭️actor/🧬️typegen/🏃️execution/🟦️.ts`, so a fresh clone generates them (proven by removing each:
3 s, 10 s, 28 s, output identical).

**Evidence.**

- Before: `bun nx run @semio-tech/quiz-react:typecheck` → exit 1, 133 `error TS`.
- With the UI-contract binding emptied: the target ran its six dependencies, the binding came back (0 → 45018 bytes).
- Now: `bun nx run @semio-tech/quiz-react:typecheck` → `Successfully ran target typecheck for project
  @semio-tech/quiz-react and 6 tasks it depends on`, exit 0.
- Deliberate error (`export const POLISH_DELIBERATE_TYPE_ERROR: number = "not a number";` appended to
  `🔨️modules/🗳️crowd/🟦️.tsx`): `../../🔨️modules/🗳️crowd/🟦️.tsx(237,14): error TS2322: Type 'string' is not assignable to
  type 'number'.`, exit 1. Removed; the target is exit 0 again.

### 2b. `@teaching/architecture-quiz:typecheck`

**Change.** `<site>/📦️packages/🟦️typescript/tsconfig.json` (new; the site entry, the Vite configuration, the stack,
the deploy verbs, the end-to-end harness and every test and spec); verb `typecheck` in the site's `📜️script.ts`; target
in `📋️project.json` (same six dependencies); scripts in the site's and the root `package.json`
(`typecheck:teaching:architecture-quiz`); launch row `🛠️dev🎓️teaching🏛️architecture❓️quiz🪁️typecheck` in
`.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json`; `deploy-check` step 2 "sources type-checked" (before anything
is built); README of the site (launch rows, the eight steps of the readiness gate).

**Errors found, fixed at the root** (first run: 3 `error TS`):

| Where | Cause | Fix |
|---|---|---|
| `<site>/🏗️builder/🌐️vite/🟦️.ts(110,29)` | The configuration mixed Vite's own plugin types with the repository's owned build contract | Rewritten on the owned contract (`defineOwnedBuildConfigFactory`, `uiTailwindBuildPlugins`, `uiReactBuildPlugin`, `OwnedBuildPlugin`); `robots.txt` and the web manifest are written in `closeBundle`. Build exit 0, `siteArtifactProblems` → `[]`. |
| `<site>/🚀️deploy/🟦️.ts(444,231)` | An identity literal widened to `{ kind: string }` | Typed as `IdentityClaim` of the quiz schema |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts(2071,20)` | `nativeEnv` inferred without the variables added later | `const nativeEnv: NodeJS.ProcessEnv` |

The order of the readiness gate is a function now (`deployCheckSteps` in `<site>/🚀️deploy/🟦️.ts`), and
`<site>/🧪️tests/🧪️deploy/🟦️.ts` holds it against the package's own targets, the numbered list in the README and the
`include` of the `tsconfig.json`: no drift, **sources type-checked**, catalog, site, image, image check, stack,
end-to-end gate.

**Evidence.** `bun nx run @teaching/architecture-quiz:typecheck` → `Successfully ran target typecheck for project
@teaching/architecture-quiz and 6 tasks it depends on`, exit 0. The whole readiness gate with the new step:
`bun nx run @teaching/architecture-quiz:deploy-check` → exit 0, `[deploy] ready to deploy: no drift from
🚀️deploy/🔣️.json (0 s); sources type-checked (59 s); catalog valid in the Rust core (3 s); site built and verified as
the CDN artifact (8 s); proctor image built (156 s); proctor image checked (4 s); stack checked (22 s); end-to-end gate
(540 s)` (two warnings, both known: no imprint and no privacy notice configured).

## 3. Old development data strands nobody

**Decision.** The launcher's own folder `.🧬semio/🎓️teaching/proctor-dev/` is disposable by definition, so the dev
launcher — and only it — settles it before anything is built: data of another storage format is **moved aside** to the
sibling `proctor-dev.v<format>-<YYYYMMDDTHHMMSS>` (never overwritten: `-1`, `-2`, …), the proctor starts with empty
data and one line says so. Moving aside instead of deleting keeps the developer's data for whoever still wants to look
at it; starting instead of stopping is the zero-touch behaviour. A folder the developer named with `PROCTOR_DATA` is
not the launcher's to move: the command ends at once, status 1, with one line that names it and says how to go on.
`proctor serve` itself is unchanged and refuses such a file in every mode — the convenience is in the TypeScript
launcher, not in the binary.

**Change.** `🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts`: `PROCTOR_STORAGE_FORMAT`, `storedProctorFormat` (reads the format
row read-only with the SQLite the runtime ships — `bun:sqlite` under Bun, because `node:sqlite` only exists from Bun
1.4 and the repository pins 1.3.14; statement finalized and connection closed, or Windows cannot move the folder),
`settleDevelopmentData`, `DevelopmentDataRefused`, `developmentDataSettled`, `runDevelopmentProctor`. Used by
`🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📜️script.ts` (`dev`) and `<site>/🧱️stack/🟦️.ts` (`dev`). READMEs of the site
and of the proctor.

**Tests.** `<site>/🧪️tests/🧱️local-stack/🟦️.ts`, describe "development data of another storage format" (10 tests): the
twin constants equal the Rust source; reads the row and changes nothing; a row that only the write-ahead log of a
killed proctor holds; no format for no database, no row, no SQLite file; leaves current data alone; moves aside and
says the one line; never overwrites what it set aside; "newer" and "another" wording; a named folder is refused and
untouched; a dev command ends over a refusal with that one line and status 1, not a stack trace.

**Evidence** (on a copy of the real v1 folder under `🗑️generated/polish/`; the original has sha256 `e8e61dac…` before
and after and was never opened for writing):

- `bun ui_polish_dev_data.ts <v1 copy> <work> 8801` → `14/14 checks passed` on bun 1.4.2 and on 1.3.13: moved aside in
  6 ms to `proctor-dev.v1-20261002T062650`, byte for byte; the real proctor then answers `GET /instance` over fresh v2
  data; `proctor serve` in development **and** production exits 1 with `… holds format semio.teaching.proctor.sqlite
  v1; this proctor reads semio.teaching.proctor.sqlite v2` and leaves the file as it was.
- The line a developer reads: `<data> held disposable development data in an older storage format
  (semio.teaching.proctor.sqlite v1; this proctor reads v2, and there is no migration). It was moved aside to <aside>
  and the proctor starts with empty data. Delete <aside> when you do not need it; to reset development data yourself
  at any time, stop the proctor and delete <data>.`
- Both dev verbs with `PROCTOR_DATA=<v1 copy>`: `bun ./📜️script.ts dev` in the site package → 2 lines, exit 1, 484 ms;
  in the proctor package → 2 lines, exit 1, 478 ms: `[stack] <data> holds proctor data in an older storage format (…).
  PROCTOR_DATA names that folder, so it is left as it is: delete or move it, or unset PROCTOR_DATA to use the
  launcher's own disposable folder <own>.`

The developer's real folder still holds v1: the next `dev` on this machine will move it aside and say so.

## 4. Browser measurements

Scripts: `ui_polish_measure.ts` (`still`, `pages`, `results`, `reflow`, `contrast`), `ui_polish_home_cards.ts`,
`ui_polish_shots.ts`, against a stack of their own started and stopped by `ui_polish_stack.ps1` (site 6071, proctor
8801, own data directory and Vite cache, the dev server not watching, so nobody's edit reloads a page under
measurement).

### Reflow: German, 1280 × 720 and its 200 % and 400 % equivalents

Eleven screens per row: home cards, a run of each kind before and after answering (classification, classification
with spider diagrams, sorting, matching), results, leaderboard. "Cut" is text clipped by a box that hides its overflow
(visually hidden text and the deliberately one-line crowd places excluded); "overlap" is two visible texts over each
other.

| Zoom (viewport) | Text | Screens | Page scrolls sideways | Text cut | Texts overlapping | Tables scrolling inside their own frame | Navigation-bar title shortened |
|---|---|---|---|---|---|---|---|
| 100 % (1280 × 720) | normal | 11 | 0 | 0 | 0 | 0 | 0 |
| 100 % (1280 × 720) | largest | 11 | 0 | 0 | 0 | 1 | 0 |
| 200 % (640 × 360) | normal | 11 | 0 | 0 | 0 | 12 | 0 |
| 200 % (640 × 360) | largest | 11 | 0 | 0 | 0 | 4 | 11 |
| 400 % (320 × 180) | normal | 11 | 0 | 0 | 0 | 12 | 11 |
| 400 % (320 × 180) | largest | 11 | 0 | 0 | 0 | 13 | 11 |

Home cards (`ui_polish_home_cards.ts`, 14 viewport sizes × normal and largest): no card taller than its cell, no two
cards over each other.

**What failed and was fixed.**

| Finding | Fix |
|---|---|
| Home grid: cards overlapped the next row where the window was wide but low (the grid was chosen from the width and one height threshold of 480 px) | `<react>/🔨️modules/🏠️home/🟦️.tsx`: the least height a grid needs follows from its rows (`homeGridMinHeight`: 160 px a row and 56 px of chrome, scaled by the text size — 600 px for three columns, 856 px for two); below it the overview is the list. Cells scroll inside themselves as the last resort (`.quiz-home-cell { overflow-y: auto }`). 24 height vectors in `<quiz>/🧫️fixtures/🏠️home-grid/🔣️.json`. |
| List mode: cards taller than the window were centred and lost their top | `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx`: `items-[safe_center]` generated no CSS in Tailwind 4.3; now `items-center-safe` |
| Footer chips of cards and the "Updated" chip of the leaderboard overlapped their neighbour or lost their end | `🪟️chrome`: chips wrap (`CARD_CHIP_WRAP`); title chips wrap |
| Matching: the table was wider than its card at 320 px | `table-fixed` with a `<colgroup>`; the selects fill their cell |
| The page scrolled sideways although every table had a scroll frame | Visually hidden text (`.sr-only`, absolutely positioned) escaped frames that were not positioned: every `overflow(-x)-auto` frame is `relative` now (results, leaderboard, presence details, profile, spider diagram, page frame, language chooser); a test over every scroller keeps it so |
| Sorting: the crowd's track pushed the average out of its line | The track shrinks (`flex: 0 1 4em; min-width: 1.5em`) |

**Left as it is.** At 640 px with the largest text and at 320 px the title in the navigation bar ends in an ellipsis
(33 of 66 rows). The bar is the design system's fixed-height bar; the full title is the name of that button and the
document title. Tables wider than a narrow window scroll inside their own frame, by design.

### Forced colours (`forcedColors: "active"`), light and dark

Each state must differ in computed style from the state beside it — 16 of 16 do.

| State | Light | Dark |
|---|---|---|
| Pressed language, theme and text-size button vs unpressed | background `rgb(55, 0, 110)` on white text vs transparent on black | background `rgb(26, 235, 255)` on black text vs transparent on white |
| Current task marker vs another task | the same pair of colours | the same pair of colours |
| Own leaderboard row vs another row | `outline: solid 2px rgb(55, 0, 110)` vs none | `outline: solid 2px rgb(26, 235, 255)` vs none |
| Online mark vs its row | `rgb(0, 0, 0)` vs transparent | `rgb(255, 255, 255)` vs transparent |
| Crowd marker vs its track | `rgb(0, 0, 0)` vs transparent | `rgb(255, 255, 255)` vs transparent |
| Area of a spider diagram vs its rings | stroke 3 px `rgb(55, 0, 110)` vs 0.75 px grey | stroke 3 px `rgb(26, 235, 255)` vs 0.75 px grey |

Nothing had to be changed for forced colours.

## 5. A refused sign-up

`🗒️signup-notes-for-ui.md` exists, so this was built.

**Decision.** A `429` that names the allowance `sign-up` is an answer, not a shortage: the client stops, says what is
true and does not send the sign-up again by itself. Until the wait the proctor named is over, it sends no further
sign-up at all and answers with the time that is left; recalling an existing pseudonym — a read — keeps working. Every
other `429` stays a shortage that is waited out silently, as before. `roster-full` says that only the operator can
help.

**Change.**

- `<react>/🔨️modules/🛂️proctor/🟦️.ts`: `ProctorThrottled.allowance` (read from the error body next to `retryAfterMs`),
  `SIGN_UP_ALLOWANCE`, `signUpsSpent`; such an error is neither transient nor "throttled", so `retryTransient` ends at
  once and the header does not say "server busy".
- `<react>/🔨️modules/🧭️session/🟦️.ts`: `SessionFailure` kind `waiting` (`allowance`, `retryAfterMs`); `identify`
  remembers until when no sign-up may be sent.
- `<react>/🔨️modules/🪪️identity/🟦️.tsx`: `failureProblem` says it; the handle field is marked invalid, and described
  by the alert, only when the problem is about the handle.
- `<react>/🔨️modules/🌐️i18n/🟦️.ts`: `quiz.identity.signUpsSpent`, `quiz.identity.signUpsSpentSoon`, new
  `quiz.rejection.rosterFull`, English and German (du).

| Case | English | German |
|---|---|---|
| Allowance spent, wait known | Too many new learners have signed up from your network just now, so the quiz server takes no further sign-up from it for the moment. This passes by itself: try again in about 36 s – or continue with a pseudonym you already have, which works right away. | Aus deinem Netzwerk haben sich gerade sehr viele neu angemeldet, deshalb nimmt der Quiz-Server von dort im Moment keine weitere Anmeldung an. Das geht von selbst vorbei: Versuch es in etwa 36 s noch einmal – oder mach mit einem Pseudonym weiter, das du schon hast; das geht sofort. |
| Allowance spent, no wait named | … try again in a moment – … | … Versuch es gleich noch einmal – … |
| Roster full | The quiz server has reached its maximum number of learners and takes no new ones until its operator raises the limit; trying again does not help until then. You can continue with a pseudonym that already exists. | Der Quiz-Server hat seine Höchstzahl an Lernenden erreicht und nimmt keine neuen auf, bis die Betreiberin oder der Betreiber die Grenze anhebt; ein neuer Versuch hilft bis dahin nicht. Du kannst mit einem Pseudonym weitermachen, das es schon gibt. |

**Tests.** Shared vectors `<quiz>/🧫️fixtures/🎭️identity-step/🔣️.json` (`refusals`, 4) and
`<quiz>/🧫️fixtures/🚦️rate-limits/🔣️.json` (`allowances`, 5 answers); `<quiz>/🧪️tests/🚦️rate-limits/🟦️.tsx` (what the
transport reads from each answer; only `sign-up` ends the call) and `<quiz>/🧪️tests/🎭️identity-step/🟦️.tsx`, describe
"🎭️ a sign-up the proctor refuses for now" (the sentences in both languages; no second sign-up before the wait is
over, the time left counts down, a pseudonym is recalled during the wait; the form shows an alert, is not busy, the
button is ready, a second click sends nothing, the handle field is not marked invalid).

**Evidence** (`bun ui_polish_signup.ts` against a real proctor with `PROCTOR_LIMIT_SIGNUPS_BURST=1` and
`PROCTOR_LIMIT_SIGNUPS_PER_HOUR=100`): 10 of 10 checks. The second sign-up got `429`, `Retry-After: 35`,
`{"kind":"throttled","message":"the sign-up allowance of this address is spent","retryAfterMs":34774,"allowance":"sign-up"}`;
the form said the German sentence with "in etwa 35 s", was not busy, showed no "Server ausgelastet"; 2.5 s later still
one sign-up sent; a second click sent none and said "in etwa 33 s"; the first learner's pseudonym was recalled during
the wait without a sign-up; a third learner read the English sentence with "30 s", waited them and got in (`429`, then
`200`). The alert at 1280, 640 and 320 px wide with the largest text, and in forced colours: no sideways scroll,
nothing cut.

`roster-full` was verified by the unit vectors only, not against a full proctor.

## Final gates

Run on the final sources, one after the other, nothing else running on the machine:

| Command | Result |
|---|---|
| `bun nx run-many -t test -p @semio-tech/quiz-react @teaching/architecture-quiz --skip-nx-cache` | exit 0 — `@teaching/architecture-quiz`: `Test Files 5 passed (5)`, `Tests 131 passed (131)`; `@semio-tech/quiz-react`: `Test Files 18 passed (18)`, `Tests 492 passed (492)` |
| `bun nx run @semio-tech/quiz-react:typecheck` | exit 0 — `Successfully ran target typecheck for project @semio-tech/quiz-react and 6 tasks it depends on` |
| `bun nx run @teaching/architecture-quiz:typecheck` | exit 0 — `Successfully ran target typecheck for project @teaching/architecture-quiz and 6 tasks it depends on` |
| `bun nx run @teaching/architecture-quiz:test-e2e` | exit 0 — `[rehearsal] 26 passed (6.5m)`, `[dev] 26 passed (8.3m)`, `[e2e] passed in 505 s: dev, rehearsal` (first attempt, no rerun needed) |
| `bun ./📜️script.ts verify taxonomy report --scope "🎓️teaching"` | `[verify taxonomy report] clean=true errors=0 warnings=0 scope=🎓️teaching` |
| `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"` | `[verify taxonomy report] clean=true errors=0 warnings=0 scope=🧰️framework/🛍️products/❓️quiz` (the directory `🧪️tests/🖼️task-icons` that was unregistered at the start is registered now, by its owner) |
| `bun nx run @teaching/architecture-quiz:deploy-check` (not asked for; run because its steps were changed) | exit 0 — all eight steps, its own end-to-end gate again `[rehearsal] 26 passed (6.6m)`, `[dev] 26 passed (8.1m)`, `[e2e] passed in 493 s: dev, rehearsal` |

The first end-to-end run was made before the order of the readiness gate became a function; the second one, inside
`deploy-check`, ran on the final sources. The only edits after it changed the emoji that opens six docstrings (no
code); after them, at script level (`bun ./📜️script.ts test` and `typecheck` in both packages): 18 files / 492 tests
and 5 files / 131 tests passed, both type checks exit 0.

The stack of the measurements is stopped (`ui_polish_stack.ps1 stop` → `0 listener(s) left on 6071, 8801`); the ports
6061, 8791, 6161, 6162, 8891 and 8892 were used by this work package only through the end-to-end gate itself; nothing
listens on any of the eight ports and no container runs now. No `[DEBUG]` log is left in a source or a ticket script.

## Open issues

1. **Navigation-bar title** ends in an ellipsis at narrow widths (above). A wrapping title needs a design-system bar
   that may grow.
2. **`🎚️ui-axes` binding is a bootstrap cycle.** Every `📜️script.ts` imports the repository library, whose barrel imports
   `🛂️manifest/🟦️.ts`, which imports the generated `🎚️ui-axes/🟦️.ts` as a value. When that file is missing, no script
   starts — including the generator that writes it (`Cannot find module './🤖️generated/🎚️ui-axes/🟦️.ts'`). The other
   three bindings regenerate from nothing now; this one needs the library's barrel untangled from the manifest.
3. `<site>/📦️packages/🟦️typescript/package.json` still lists `@tailwindcss/vite` and `@vitejs/plugin-react`, which the
   Vite configuration no longer imports directly. Left in place to keep `bun.lock` still while others work.
4. Under bun 1.3.13 the Vite dev server of the stack exited with status 1 once ("the site exited with status 1"); under
   1.4.2 it never did. Not investigated; `package.json` pins 1.3.14.
5. The results show at most the counts that fit into the box of everyone (two lines, most given first). The full
   distribution is in the sentence for screen readers and on the quiz's page; a pointer user has no way to open the
   rest from the results.
6. Creating the nx project graph takes two to three minutes per invocation on this host, so each `bun nx …` gate
   costs that before it starts.
7. The repo MCP servers (`repo`, `semio`) did not connect; ticket bookkeeping stayed manual and the ticket is left
   open for the coordinator.

Issue 2 is flagged as a follow-up task of its own ("Break the ui-axes binding bootstrap cycle").

One slip, cleaned up: a run of `ui_polish_dev_data.ts` with relative arguments made the proctor create two fresh data
folders under `🗑️generated/polish/` at the repository root (the proctor's working directory). The folder was removed
at once (it was git-ignored and held nothing else), and the script resolves its arguments now.

## Files

**Created**

- `<site>/📦️packages/🟦️typescript/tsconfig.json`
- `<ticket>/ui_polish_dev_data.ts`, `ui_polish_measure.ts`, `ui_polish_home_cards.ts`, `ui_polish_shots.ts`,
  `ui_polish_signup.ts`, `ui_polish_stack.ps1`
- `<ticket>/📓️ui-polish-report.md`

**Updated**

- `<react>/🟦️.tsx`, `<react>/🎨️.css`, `<react>/📦️packages/🟦️typescript/📋️project.json`
- `<react>/🔨️modules/`: `🗳️crowd/🟦️.tsx`, `🗂️classification/🟦️.tsx`, `↕️sorting/🟦️.tsx`, `🃏️matching/🟦️.tsx`,
  `▶️run/🟦️.tsx`, `📖️quiz-page/🟦️.tsx`, `🏁️results/🟦️.tsx`, `🏠️home/🟦️.tsx`, `🪟️chrome/🟦️.tsx`,
  `🏆️leaderboard/🟦️.tsx`, `👥️presence/🟦️.tsx`, `📇️profile/🟦️.tsx`, `🕸️radar/🟦️.tsx`, `🛂️proctor/🟦️.ts`,
  `🧭️session/🟦️.ts`, `🪪️identity/🟦️.tsx`, `🌐️i18n/🟦️.ts`
- `<quiz>/🧪️tests/`: `💭️crowd-client/🟦️.tsx`, `📡️presence-client/🟦️.tsx`, `🏠️home-grid/🟦️.tsx`,
  `🗣️translation-completeness/🟦️.tsx`, `🚦️rate-limits/🟦️.tsx`, `🎭️identity-step/🟦️.tsx`
- `<quiz>/🧫️fixtures/`: `🏠️home-grid/🔣️.json`, `🎭️identity-step/🔣️.json`, `🚦️rate-limits/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts`,
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🗂️files/🟦️.ts`
- `🧰️framework/📦️packages/🦀️rust/📜️script.ts`, `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts`,
  `🧰️framework/🔨️modules/🎭️actor/🧬️typegen/🏃️execution/🟦️.ts`
- `<site>/README.md`, `<site>/📦️packages/🟦️typescript/📜️script.ts`, `📋️project.json`, `package.json`
- `<site>/🏗️builder/🌐️vite/🟦️.ts`, `<site>/🚀️deploy/🟦️.ts`, `<site>/🧱️stack/🟦️.ts`, `<site>/🎭️e2e/🚶️learner/🟦️.ts`
- `<site>/🧪️tests/`: `👥️shared-presence/🟦️.ts`, `📱️phone/🟦️.ts`, `🧱️local-stack/🟦️.ts`, `🧪️deploy/🟦️.ts`
- `🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts`, `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📜️script.ts`,
  `🎓️teaching/🛂️proctor/README.md` (one paragraph)
- `package.json` (one script), `.vscode/🧩️launch.seed.jsonc`, `.vscode/launch.json`

**Removed**

- `<ticket>/🗑️generated/polish/` (the tool output of this work package), after its numbers were copied here.
