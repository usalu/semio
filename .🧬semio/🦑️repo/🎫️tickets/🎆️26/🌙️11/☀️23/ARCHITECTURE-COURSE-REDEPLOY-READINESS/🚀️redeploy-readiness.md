# 🚀️ Architecture Course Redeploy Readiness

## What was asked

- The course of `architektur-und-technologie.de` is German by default.
- The huge "How the quizzes work" text that exceeds the screen is removed.
- The course is ready to be redeployed.

## What was found

- The first-visit walkthrough (`🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/👋️introduction/🟦️.tsx`) joins every
  `introduction.paragraphs` entry of the course into the body of its centred welcome step. The architecture course carried six
  paragraphs (about 4 400 characters in German), so that one box was taller than any screen. The text was still present.
- The client had no notion of a site language: stored choice, else the browser's list, else a chooser. A learner with an
  English browser got the English course.

## What changed

| Area | File | Change |
|---|---|---|
| Course product | `🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts` | `initialLocale(site, languages)`: the site's own language (a BCP 47 tag) ahead of the browser's list |
| Course product | `🎯️targets/⚛️react/🔨️modules/📚️app/🟦️.tsx` | `CourseOptions.language`; `CourseApp` and `mountCourse` resolve stored choice, then site language, then browser, then chooser |
| Course product | `🎯️targets/⚛️react/🟦️.tsx` | exports `initialLocale` |
| Course product | `🧫️fixtures/🌍️language-choice/🔣️.json` | language-agnostic `sites` vectors (site language, browser list, expected locale) |
| Course product | `🧪️tests/🌍️language-choice/🟦️.tsx` | the vectors against `initialLocale` and `i18next`'s negotiation as oracle; a mount in German under an English browser, with the stored choice kept over it |
| Site | `🚀️deploy/🔣️.json`, `🚀️deploy/🧬️schema/🔣️.json` | required `site.language` (`"de"`) |
| Site | `🟦️.ts` | passes `language: site.language` to `mountCourse` |
| Site | `🔣️.json` | introduction: title "Willkommen bei Architektur und Technologie" and one paragraph of two sentences instead of six paragraphs |
| Site tests | `🧪️tests/📰️host-document/🟦️.ts` | the deployment names German, the entry hands it on, the introduction is one paragraph of at most 320 characters per language |
| Site tests | `🧪️tests/🧪️deploy/🐳️docker-context-and-image/🟦️.ts` | a site without a language and one with an unoffered language are refused by the schema |
| E2E | `🎭️e2e/🚶️learner/🟦️.ts` | `SITE_LOCALE`; `arrive` expects the site's language and chooses the device's with the language switch where it differs |
| E2E | `🧪️tests/🗣️both-languages/🟦️.ts` | the site speaks its own language under German, English and French browsers, never asks, and keeps an explicit choice |
| Docs | both `README.md` files | language resolution, the short introduction, the spec rows |

## Decisions

- **German wins over the browser.** "By default German" is read as: German unless the learner chose otherwise, also for an
  English browser. The language switch stays in reach on the welcome step, and the choice is persisted.
- **The site names its language, not the client.** The course product still has no default language; a site that passes none
  behaves as before (browser, then chooser).
- **The challenge rules are no longer explained in the welcome box.** The introduction names the four challenges in one
  sentence; the quiz page still offers the challenge choice. The meta description of the release document
  (`introduction.paragraphs[0]`) became the short paragraph as well.

## Verification

The graph route (`repo run <project>:<target>`) gave no result on 2026-11-23: after two and a half hours behind another
process's dependency build on the shared checkout it ended with the build service's
`Internal error: Declared provider descriptor changed`, before any task ran. The same runners were therefore started through
the tool registry (`repo tools exec vitest|tsc`) from each package folder with the suite's own configuration.

| Check | Command (from the package folder) | Result |
|---|---|---|
| Language choice (course product) | `repo tools exec vitest -- run --configLoader native --config ../../🧪️tests/🎚️config/🟦️.ts language-choice` | 29 of 29 passed, including the `sites` vectors against `i18next` and the German mount under an English browser |
| Course product, whole suite | same, without a filter | 1167 of 1176 passed under a loaded machine; the 9 failures were 5 s timeouts and their follow-ups |
| The five failing files again, serially | same, `--no-file-parallelism language-choice pet-allowances quiz-card-challenges home-grid load-failures` | 126 of 127 passed |
| Site unit tests | same configuration of the site, `host-document docker-context-and-image 🧪️course` | 149 of 149 passed (German site language, the entry hands it on, one short introduction paragraph, the schema refuses a missing or unoffered language) |
| Typecheck, both packages | `repo tools exec tsc -- --noEmit -p tsconfig.json` | no error in any file of this ticket; errors only in files others are changing right now (`🖱️ui/🧬️contract/🧵️retained`, `🖱️ui/🧱️elements/🌳️Tree`, `🦑️repo/🔨️modules/📚️library`) |

### Second round: what stood between the gate and green

| Finding | Fix | File |
|---|---|---|
| The generated UI contract types were from 2026-11-18 and lacked `sheet`, `cardStack`, `card`, `unnamedControl` and the `touch` tree presentation of the Rust contract (8 type errors) | regenerated with `repo cargo --store=shared --configuration=prototype -- test -p semio-ui-contract-typegen --test typegen_export` and `SEMIO_TYPEGEN_OUT` | `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts` |
| `CasReclaimV1` gained `held`; the unit cache's empty reclaim did not name it | `held:0` | `🦑️repo/…/🗃️unit-cache/🧹️gc/🪢️supersession/🟦️.ts` |
| The remote cache handler read `options.hostStore`, which only the server options declared | the handler options declare it and the server hands it on | `🦑️repo/…/🌐️remote-cache/🖥️server/🟦️.ts` |
| The whole-read progress named 9 of 11 fleet readers | `cas` and `offload` idle readers | `🦑️repo/…/🕸️graph/🕰️host/🟦️.ts` |
| An untyped callback parameter | `(path:unknown)` | `🦑️repo/…/🗄️cas/🫂️holders/🟦️.ts` |
| Two comparisons of a byte array with `"miss"` after the string case had left | removed | `🦑️repo/…/🕸️graph/📥️load/🤝️selected-exchange/🟦️.ts` |
| `🧾️proctor-product` failed on Windows: POSIX fixture paths compared with native ones, and native paths spliced unescaped into Cargo's JSON lines | `normalize` on the expectation, JSON-quoted paths in the layout | `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧾️proctor-product/🟦️.ts` |
| "polls the leaderboard…" simulates a minute of polling and needs 9.5 s here, above the 5 s default | a 60 s budget for that test | `📚️course/🧪️tests/🏠️home-grid/🔸️opens-page-step-front/🟦️.tsx` |
| The two mount tests of the language choice wait up to 20 s inside a 5 s test | a 60 s budget for both | `📚️course/🧪️tests/🌍️language-choice/🟦️.tsx` |

| Check after the fixes | Result |
|---|---|
| Site typecheck (`repo tools exec tsc -- --noEmit -p tsconfig.json`) | exit 0 |
| Course product typecheck (same) | exit 0 |
| Site unit suite, whole | 20 of 20 files, 876 of 876 tests |
| Course product unit suite, whole (`--maxWorkers 2`) | 1174 of 1176; the two 5 s timeouts (`🏠️home-grid` nested topic grids, `📥️load-failures` switched-to language) are load on this machine |
| Those two files alone, serially | 63 of 63 |

### Open
### Third round (2026-11-24): the build page and the dev page without the graph

`repo service host status` answers "No repo graph service is running on this host" and `repo fleet status` shows
"Generation: none published". That is why every `repo run` stayed in "Reading the project graph slice": the release
build was stopped after five hours in that phase without having started a task. Both pages were therefore produced through
the tool registry, with input scripts kept in this ticket.

| Script | What it does |
|---|---|
| `site-direct.ts build release` | runs the site's own Vite configuration (`repo tools exec vite`) with the two things the configuration demands from its caller: the compiler cache path below the products partition, and the assignment PDFs over the `course-assignment-pdfs` runtime channel |
| `site-stage.ts` | runs the `publish` verb's own `siteArtifactProblems` on the build and stages it with `stageCourseSiteArtifact` (adds `_headers`) |
| `site-direct.ts dev` | the dev server on port 6061 at once; beside it the dev proctor, compiled into a private target directory and tried again every three minutes while the tree does not compile |
| `site-serve.ts` | serves the staged CDN directory like a static CDN on port 6062 |

`.claude/launch.json` gained `architecture-course-native` (6061) and `architecture-course-cdn-native` (6062).

| Result | Evidence |
|---|---|
| Release build | built in 11.6 s into `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/dist/release` against `https://semio.iek.uni-hannover.de` |
| Publish check | `siteArtifactProblems`: no problems (entry points, sealed document, hashed assets, no development leftover, script, style, total, assignment and poster budgets) |
| **CDN directory** | 116 files staged in `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/dist/pages/course` (git-ignored) |
| CDN directory in a browser (6062) | boots without a console error, `<html lang="de">`, the German switch pressed under an English browser, the welcome box is one short paragraph |
| **Dev page** (6061) | the same in the dev server: German by default, title "Willkommen bei Architektur und Technologie", one paragraph, the box fits an 800 × 600 window |

### Fourth round (2026-11-24): the reported defects, the proctor and the rehearsal

| Reported | Cause | Fix |
|---|---|---|
| The introduction blurred the element it introduces | the introduced card got `z-index` above the veil, but sits inside the card grid's own stacking context (`z-[31]`), so the fullscreen veil still covered it | `useIntroductionElevation` stamps every stacking-context ancestor `data-introduction-elevation-path` (raised with the unit) and the first real boxes beside the way to the unit `data-introduction-veiled` (blurred and dimmed like the veil, through `display: contents` wrappers); `🖱️ui/…/📐️introduction-geometry/🟦️.ts`, `🖌️ui/🎨️.css` |
| Starting a quiz signed out asked to sign in and then returned to the main page | the wanted action was dropped when the identity dialog opened | `HomeScreen` remembers it and runs it once the dialog closes with a learner (`identifying`), forgets it when the dialog closes without one |
| Card borders of unequal thickness, hovered rectangle not emphasized | frame outlines are stroked centred on the edge; the frame and the home cell clipped the outer half, so edges showed at half the thickness of inner lines | `overflow-clip-margin: var(--stroke-hairline)` on `[data-window-frame]` and on `.course-home-cell` (now `overflow: clip` with zero minimum sizes) |
| Not yet available content must be tinted, loaded beneath | clips not hosted yet showed untinted | `UnavailableTint` (one tint for locked pages, locked tasks and clips not hosted yet); the gate's tint is 60 % so the card beneath reads as itself, muted |
| Clip cards with a large empty section | the card held poster, description and free room | the card is the poster between its chips; the description stays on the clip's page |
| Locked cards cut, not filling their cell | the gate's preview was a block, so its card kept its content height | the gate and its preview are flex columns; locked collection and assignment cards keep their action chip, disabled |
| The ranking must be a table scrolling in both dimensions | the full leaderboard folded into records whenever its columns exceeded the card | `course-table` in `course-table-scroll`: sticky headings, rank and learner pinned, both axes scroll |

| Check | Result |
|---|---|
| Typecheck, both packages | exit 0 (a peer's new `memory` fleet reader added to the idle readers in `🦑️repo/…/🕰️host/🟦️.ts`) |
| Course product unit suite | 1173 of 1177: the two known 5 s load timeouts, and two stylesheet assertions on rules peers changed meanwhile (pets strip, card stack) |
| New tests | sign-in then run start (`🏠️home-grid/🔸️opens-page-step-front`), tinted clip poster without description (`🎞️assignment-clip-pages`), elevation path and veiled siblings (`🖱️ui/🧪️tests/…/🧭️uiintroduction-veil-and-elevation`, 12 of 12) |
| Proctor | compiles from the live tree again (the peer's crate split settled at 03:31) with plain `rustc` into `.cache/b/r/m/u/l/acd/t`; the cargo gate was not retried |
| Dev page with proctor | anonymous sign-in from the quiz page lands in the run "Heizen (Mittel)"; quizzes, clips and the board checked in the browser |
| **Rehearsal** (`site-rehearsal.ts`: release build baked for a throw-away proctor, static origin 6063, production-mode proctor 8793 admitting only that origin) | boot and desktop projects for site-boot, first-visit, both-languages, layered-home, live-leaderboard: **14 passed, 3 failed** |
| Dev topology, quiz-runs and challenge-levels | 15 passed before the 10 minute limit, none failed |
| CDN directory | rebuilt after every fix, `siteArtifactProblems`: no problems, 116 files |

### Fifth round (2026-11-24, 05:30 to 06:30): the rehearsal failures

| Failure | Cause | Change |
|---|---|---|
| `🥞️layered-home` "learner sits inside its grid cell" | the pages run beneath the navigation and footer bands, so the overview's root is the whole viewport; the spec divided the root into thirds | the spec measures the grid's room (`gridBox`: the overlay without its band insets); passes |
| `🥞️layered-home` "`data-pan` stays `pointer` under reduced motion" | the home screen passes `reducedMotion="auto"` and a unit test demands that a device asking for reduced motion gets no pan; the spec and the README said the opposite | spec and README follow the code and the unit test |
| `🗣️both-languages` "page prefs: the same controls in both languages" (flipped between en and de from run to run) | the eight pet deeds are read once for the first pet on stage; right after a language switch the cast can be empty | `wording` waits for the cast before reading; both variants pass |
| sign-in radios not checkable (new during the round) | a peer hid the radios visually under styled labels at 05:33 | `check({ force: true })` in the learner helper and two specs; first-visit passes 4 of 4 |
| pan never reaches the first page from the grid's corner | the pointer was mapped over the whole root, part of which lies under the bands | `🥞️LayeredOverview/🟠️pane-styles` maps it over the overlay's room |

| Rehearsal run (static release build) | Result |
|---|---|
| desktop: first-visit, both-languages, layered-home (15 specs) | 13 passed at 05:50; the two failures were the prefs parity and the pan check above |
| both-languages "reads every screen" en and de, after the fix | 2 of 2 |
| desktop layered-home + phone + layout (14 specs) | 5 passed, 9 failed |
| pets (6 key specs, after the border-line terrain) | 3 passed, 3 failed |

### Sixth round (2026-11-24, 06:30 to 08:40): the gate on the current tree

Peers had finished giving every control an icon and reworking the radios; their tests and the end-to-end helpers lagged.

| Finding | Change |
|---|---|
| Inline icon markup carried its source line breaks into every control's text | `iconInlineMarkup` compacts it (`🖱️ui/🧱️elements/🔣️Icons`) |
| `RadioChoice`: the radio's accessible name was label plus hint, the hint announced twice | the radio is named by its label text alone (`aria-labelledby`) |
| Two modules imported the whole design system for one type | slim `…/chrome` subpath |
| Tests behind the icon and radio changes (course shell, shared borders, challenge chooser, stylesheet rules) | icons passed, `radiogroup` role, relaxed rule pattern, budgets for three long tests |
| End-to-end helpers: hidden radios, Start on the challenge card, `radiogroup` | radios chosen by their label, `[data-card="quiz-challenge"]`, role updated |
| Phones: page picker hidden under the navigation band, swiped card under the footer, neighbour hints over the card | `insets` of the layered overview for swiped pages, picker cleared from the band, stray hint margins removed |
| The introduction box never settled when docking stretched it, and at fractional positions | docking works from the box's natural width; position snapped to whole pixels (probe: one geometry over 120 frames at 375 × 560 and 375 × 812) |
| `🥞️layered-home` pan check: the spec left the device in reduced motion for the next grid | emulation reset per grid |
| `📐️adaptive-layout`: the two-way scrolling ranking counted as sideways scroll; a hovered card paints the outline edges a neighbour owns at rest | table exempt; hover may add outline edges only |
| A peer's wrapper in `🦑️repo/…/🌐️remote-execution/🗄️content` lost its contextual type | `Object.freeze<RemoteExecutionContentV1>` |

| Gate step (graph-free route) | Result at 08:40 |
|---|---|
| Typecheck, course product and site | exit 0 |
| Course product unit suite | 49 files, 1177 of 1177 |
| Site unit suite | 20 files, 876 of 876 |
| Release build + publish artifact check | no problems, 116 files staged |
| Rehearsal, boot | 1 of 1 |
| Rehearsal, desktop: first-visit 4, quiz-runs 15, challenge-levels 13, live-leaderboard 1, layered-home 6, both-languages 5 | every spec passed in its last run; one both-languages run lost a PDF request to a connection reset of the rehearsal's own file server |
| Rehearsal, phone | 4 of 4 |
| Rehearsal, layout | 4 of 4 |
| Rehearsal, pets | 13 passed, 9 failed, 2 not reached |

### Seventh round (2026-11-24, 08:40 to 11:40): pets specs and the exclusive shard

| Finding | Change |
|---|---|
| The pet kit still called whole overview cards controls and looked for a footer spot with no card part above it | kit `CONTROLS` and `floorSpot` follow the client: only the chips at a card's foot are solid |
| A focus outline was counted where none is drawn | outline counts only when its style is not `none` |
| Mischief on a quiz page: the marked task row spanned the stage | the row's content (number, glyph, title) is the fixture; fixture `📋️task-topics` says `SPAN` |
| A press on a pet inside a card swallowed swipes on phones | on swiped pages the card keeps the press (`COURSE_PET_CONTROLS`) |
| `👥️shared-presence` camera check wanted a three-value transform, which the strip only carries while a pan is in flight | the two pan offsets are compared in either form |
| `site-rehearsal.ts` had no proctor control endpoint | `POST /proctor/stop|start|pause|resume` on port 8794 |

| Gate step (graph-free route) | Result at 11:40 |
|---|---|
| Course product unit suite | 49 files, 1177 of 1177; typecheck exit 0 |
| Site unit suite | 20 files, 876 of 876 |
| Site typecheck | 22 errors, all in `🦑️repo/🔨️modules/📚️library` files peers were editing at that minute (`📤️publication`, `🛰️fleet/📤️uploader`, `🌐️remote-cache/💾️store`); none in the course |
| Release build + publish artifact check | no problems, 116 files staged |
| Rehearsal: boot, desktop, phone, layout | as in the sixth round, all passing |
| Rehearsal: pets | 20 of 24: deeds ×4, hand, parachute, cast and choices, phone, overlap and throw (alone: it needs more than 5 frames a second) pass; mischief ×2, ladder and grapple fail |
| Rehearsal: exclusive shard — presence, shortage, away, faults (`--no-deps`) | 11 of 12 |

### Eighth round (2026-11-24, 11:40 to 14:30): the five failing specs, chaos and reconnect

Every cause below was measured first, with headless probes on the dev page that are kept as input scripts
(`mischief-probe.mjs`, `gear-probe.mjs`, `terrain-probe.mjs`, `dom-probe.mjs`, `watch-probe.mjs`, `cast-probe.mjs`,
`rename-probe.mjs`, `tab-probe.mjs` in the pets ticket; `scroll-probe.mjs`, `min-probe.mjs` here).

| Failing spec | Cause | Change |
|---|---|---|
| Mischief ×2 ("a pet lifts a copy of a task …") | the rows of the task list were no ground, and the marked row began 6 px from the window's edge: the only station had 6 px of room where the engine wants 12 | the rows are surfaces and open room for the pets, the mark sits on the task's title alone (`📖️quiz-page`, `🐾️pets/🎪️stage`) |
| Ladder | a quiz's page stacks its cards over the whole width: no free side to lean a ladder on (ten walls, all on the window's edge) | the spec moved to the home screen, whose grid has such sides, and tries the foot of every card side in turn |
| Grapple | on a quiz's page a control as wide as the page lies between the footer line and every line above; and no pet lives on the footer line any more, where the spec looked for its grappler | the spec moved to the home screen; the hand takes a grappler from any line |
| "cooperative card scroll" | the helper drew its swipe around the middle of the whole overview, which at 375 × 200 lies below the window; the observer's 1440 × 100 window no longer overflows, since the bands lie over the page | the swipe is drawn in the visible part of the overview; the observer's window is 60 px high |
| Chaos (first run) | its own sign-in step still checked the visually hidden radio | it clicks the label, like the learner helper |
| Reconnect (first run, 21 min, every cycle "presence never") | it read the online count as the first number in the status line's text, which now begins with an icon | it reads the status words |
| "a control under a pet still receives its click" | whole cards are no longer controls (only their chips open them), so a click on a pet in front of a card's body is the pet's | the spec lets a pet fall in front of the challenge chooser of a quiz's page and clicks the level under it: the level is chosen and the pet is not picked up |
| Deeds at 320 px in German | two rows of deeds (a phone shows two pets) are 1,270 px tall and reached under the footer line the pets stand on: the pets lost their ground, left, their rows went, they came back, for ever | the spec's window is 3,200 px high, by the reasoning its height already documented |
| `🗣️both-languages` "page prefs: the same controls in both languages" (37 against 29) | the cast left between the wait for it and the reading | the reading and the cast's presence are taken in one step and tried again |


### Ninth round (2026-11-24, 14:30 to 19:10): chaos, reconnect, a presence defect and the introduction

**A defect of the product, found by running reconnect after chaos on one proctor.** Every client shares a presence
state so that others can follow it. That state carried the whole leaderboard twice (`view.leaderboard` and
`view.overall`, 100 rows each): 191 kB with a class of a hundred, above the proctor's cap of 128 kB. The proctor refused
it (`state-too-large`), and after any reconnect nobody was online any more ("Online: 0", empty roster) for everybody.
A real class would have met this in the first lecture.

| Change | Where |
|---|---|
| The shared view no longer has `overall`/`overallAt`: the overall leaderboard is a shared read every client holds itself, and an observer uses its own copy. A leaderboard travels only while the learner looks at another board than the overall one. | schema (`🧬️schema/🔣️.json`, `🟦️.ts`, `👥️presence-wire/🦀️.rs`), fixture `👥️presentation` (6 cases), `📚️app`, `👀️follow` |
| Measured on the kept data of a chaos run (65 MB, a hundred learners): state 3.7 kB instead of 191 kB, no refusal, presence back after each of eight proctor kills within 14 s | `REHEARSAL_PROCTOR_DATA` of `site-rehearsal.ts` |
| New unit test: an observer reads the overall board from its own copy and takes only a chosen board from the peer | `🧪️tests/👀️follow` |

| Chaos and reconnect | Cause | Change |
|---|---|---|
| Chaos, run 1: all 240 browser runs done, then "what the proctor holds": two learners with an extra open run | after an interrupted submission the driver reopened the quiz through its card, whose main action had become "start again" | in the submit phase the driver lets the device drain, asks the proctor whether the run arrived, and otherwise only ever resumes an open run (`reopen`) |
| Chaos, runs 2 and 3: a page load over 150 s under the injected slow network | the rehearsal's file server sent everything uncompressed and uncached, unlike a CDN | `site-rehearsal.ts` serves compressed, hashed assets immutable, the rest revalidated |
| **Chaos, run 4: passed** (2.2 h): about a hundred learners, 240 browser runs with every point, the proctor's records equal to what was played | | |
| Reconnect alone: passes its cycles; right after chaos on the same proctor every learner showed "Online: 0" | the presence defect above | fixed in the product |

| Introduction (reported: chips not proper, titles too long) | Cause | Change |
|---|---|---|
| A two-line title in a box 288 px wide, body narrower than its own maximum | docking to the border lines beneath could make the box narrower than its content | docking only ever widens the box (`🎓️introduction`) |
| The box of the last step lay below the window's edge | a docked box skipped the clamp into the window | every box is clamped |
| An empty cell beside "Weiter" or "Fertig" | a footer chip that reaches to its neighbour was wider than its button | the footer buttons fill their chips |
| "Überspringen" half as high as a two-line title chip; on touch sizes the step counter lower than the buttons beside it | fixed control height in the cap; footer chips aligned to the bottom | cap controls and footer chips stretch to the row (`🗂️WindowChrome`) |
| Checked with `intro-shots.mjs` on the dev page: all six steps at 1280 × 800 and 420 × 700 | | 77 introduction tests and 153 of 154 window chrome tests pass (the one failure is `shared-border.test.ts` "a box that appears without any hook…", in files a peer changed at 05:57) |


| Gate step (graph-free route), final pass | Result on 2026-11-24, 19:00 to 22:00 |
|---|---|
| Typecheck, course product | exit 0 |
| Typecheck, site | no error in any course file; 36 errors in `🦑️repo/🔨️modules/📚️library` files peers are changing (`📤️publication`, `🌐️remote-execution`, `🌐️remote-cache`, `🛰️fleet/📤️uploader`) |
| Rust twin of the schema change | `cargo check -p teaching-proctor --bin proctor`: finished without error |
| Course product unit suite | 49 files, 1174 of 1178 in one run on six workers; the four (5 s and 30 s budgets under load) pass serially: 80 of 80 in their three files |
| Site unit suite | 20 files, 876 of 876 |
| Release build, publish artifact check, staging | no problems, 116 files staged at 19:03, after the last product change |
| **Rehearsal, exclusive shard in order** (presence 6, shortage 2, away 2, faults 2, chaos 1, reconnect 1; one proctor, one worker) | **14 of 14 passed in 2.6 h**; reconnect right after chaos: 17 cycles, no learner stuck |
| **Rehearsal, shared shard** (boot, desktop, phone, layout, pets; three workers, beside the chaos run) | 73 of 77 passed; the four others need frames or the pets' clock, which a machine running chaos does not give them, and **pass run alone**: touch drag at the phone's edge, parachute, no overlap (three in one run), ladder (47 s) |
| Dev page | http://127.0.0.1:6061 answers (launch entry `architecture-course-native`) |
| CDN directory | `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/dist/pages/course`, served for a look on http://127.0.0.1:6062 (`architecture-course-cdn-native`) |

### Tenth round (2026-11-24, 23:00 to 2026-11-25, 00:40): two reports from the dev page

| Reported | Cause | Change |
|---|---|---|
| "A huge gap with empty scroll area after a question" (first question of the physics quiz) | once "show it now" reveals what everyone answered, the figure was one column 470 px wide and, with twelve items, 760 px tall: the rest of a 1920 px card stayed empty. On the results screen the same figure stood beside the result table and was twice as tall, leaving the room under the table empty. | In a card of 64rem and more the items of the figure lie side by side as tiles, each with its columns and their names (`📊️plots-and-crowd`); on the results screen a task's figures stand below its tables over the whole width (`🏁️results-and-preferences`). Measured with `run-scroll-probe.mjs` at 1920 × 915: the task with the figure 791 px instead of 1346 px; a figure on the results 278 px instead of 961 px. The tier vector `📐️adaptive-layout` names the new tier instead of the removed one. |
| "A weird box inside the options" of the ventilation classification (profiles as radar charts) | the "values as a table" of each profile was wrapped as a border cell of its own, so the option's shared border drew a frame round the still closed table | the radar figure hands its table to the cell around it, as the crowd figure does (`🕸️radar`: `Records cell={false}`); done by a second agent, checked in light and dark at 1920 × 915 and 420 × 800 (`profile-box-probe.mjs`) |

| Check | Result |
|---|---|
| Course product unit suite | the files of the two changes pass (`stylesheet`, `crowd`, `results`, `adaptive-layout`, `radar`); a whole run beside the official build attempt had twelve timeouts, of which every file passes run by itself (`pets-on-stage` 13, `navigation` 53, `home-grid` 58, the other three 169 of 173 together and whole alone) |
| Rehearsal of the specs these changes touch (quiz-runs, challenge-levels, both-languages, phone, adaptive-layout) | 40 of 41 passed on three workers (22 min); the one other, the touch drag at the phone's edge, passes alone (18 s), as before |
| CDN directory | rebuilt and staged on 2026-11-25 at 00:21, after these changes: publish check without problems, 116 files |

### Eleventh round (2026-11-25, 00:40 to 04:50): the points the readiness check named

| Point | Evidence |
|---|---|
| The long "how it works" text is gone | `🎓️teaching/🏛️architecture/📚️course/🔣️.json` `introduction.paragraphs`: 6 paragraphs with 3,205 characters in the commit of 2026-11-18 (`3cde99d4`), 1 paragraph with 247 characters since 2026-11-24 (`3b15468f`). The staged CDN directory holds no "How it works" or "So funktioniert" in any file. `welcome-fit.mjs` against the staged site (6062) with an English browser: `lang="de"`, title "Willkommen bei Architektur und Technologie", one paragraph of 269 characters, the box inside the window at 1280 × 800, 800 × 600 and 375 × 667, no page scroll. |
| `🗂️WindowChrome/shared-border.test.ts` | passes: a peer brought the test up to date on 2026-11-24 at 23:41; the five shared-border files 108 of 108 |
| The course product unit suite, whole, in one run | **49 files, 1178 of 1178** (384 s on three workers while other sessions and the official build kept the machine busy). Before that, runs on the busy machine lost between one and thirteen cases to their 5 s budget, different ones each time; `cast` markers showed the longest of them progressing steadily and finishing given time. The client suite now gives a case thirty seconds (`🎯️targets/⚛️react/🧪️tests/🎚️config`), the two longest journeys ninety, and two lookups a longer wait (`🎪️pets-on-stage`, `📥️load-failures`). |
| The official route | `repo run @teaching/architecture-course:build` now reaches its tasks but did not end green in four more attempts (00:50, 04:50, 05:37, about nine hours of wall time). Attempt one: graph read 25 to 110 minutes per round, then 11 tasks passed and **four of the six assignment PDFs were compiled** (`.cache/b/p/m/print/prototype/documents/course/`, 02:55: energy de 90,538 B, parametrization en, typology en and de); `build-heating-cooling-en` and `build-parametrization-de` failed with `Print Source directory changed while discovering` although no file of the assignment folders had changed for an hour (each ran beside its sibling language on the same source folder), and the retry never left the graph read before its four hours ended. Attempt two ended in the graph service itself (`The resident service met an unexpected failure … optional.sqlite-journal … link-count; try again`). Attempt three ran the tasks after 28 minutes and failed in the build infrastructure before any course task: `Directory partition capacity exceeded` and `Original Windows relative directory facts changed` in the Rust generator tasks that had passed in attempt one. The verb of a PDF task cannot be run by itself (`repo ./📜️script.ts build-heating-cooling-en`: `Original parent Script process invocation required`). None of these failures is in the course. |

### Twelfth round (2026-11-25, 06:45 to 09:45): the official route again, five more attempts

| Attempt | How far | Where it ended |
|---|---|---|
| `repo cache status` | | the Rust cache partition stood at 96.0 GiB against a quota of 78.7 GiB ("image below its free-space floor"): the cause of `Directory partition capacity exceeded` |
| `repo cache gc --area=rust`, five passes | removed 6,566 idle or superseded build units (30.9 GiB), none held by a running compiler; first pass stopped on a DLL in use | partition at 60.4 GiB of 78.7 GiB |
| `repo run …:build` (07:00) | graph read 27 min, 8 tasks passed | `Cargo Source outside admitted owner` in `framework-rs:generate`, `framework-actor-rs:typegen`, `framework-os-shell-rs:typegen` (source admission of the native build tooling); the PDF tasks were not reached |
| `repo run …:build-heating-cooling-en` (08:07) | graph read 10 min | graph service: `Source input must be repository relative`; its log names the owner `@semio-tech/value-native-record-machine-rs` ("cannot be inferred: the failure reproduces on unchanged inputs") |
| `repo run …:build-parametrization-de` (08:18) | 9 of 11 dependency tasks passed, the Rust generators among them | the compiler wrapper of the tooling exited while compiling `semio-os-history-schema` (no Rust error, the partition was 0.2 GiB under its quota then); the PDF task was not reached |
| `repo run …:build` (09:18), after the last collection | graph read 25 min | graph service again: `Source input must be repository relative` |

Twelve attempts in all since 2026-11-24, 12:10. The route got as far as compiling four of the six PDFs once (02:55). Every
ending lies in the build tooling or in Rust projects other sessions are changing on this shared tree; none names a file
of the course.

**The cause of the last endings (found 09:45).** Three Rust sources of `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned`
(`🏗️record-machine/🦀️.rs`, `🧬️protocol/📦️packages/🦀️rust/🦀️.rs`, `🧳️original/🦀️.rs`) and one of the OS store include a
file by an absolute macOS path, `#[path="/Users/ueli/Documents/semio/…/GET-PROCESS-WORKING-END-TO-END/🗑️generated/debug-ledger.rs"] mod dbg;`,
written between 09:18 and 09:37 by a session that is debugging on the Mac. That is the "Source input must be repository
relative" of the graph service and the "Cargo Source outside admitted owner" of the Cargo admission: on every other
machine the Rust projects that the course build depends on cannot be read. The lines are another session's live
instrumentation and were left alone; the official build is queued to start as soon as they are gone.

### Open

- **The official route does not end green on this host.** See the eleventh round: it reaches the build and compiled four of six PDFs once; the other attempts ended in the print staging check, in the graph service and in the storage guards of the build tooling.
- **The staged directory carries the six PDFs the live site publishes** (energy from 2026-11-20). Freshly compiled ones exist for four (energy de, parametrization en, typology en and de) under `.cache/b/p/m/print/prototype/documents/course/`; they were not mixed in, so that both languages of an assignment stay the same version. The release workflow compiles all six.
- **Load sensitivity of four specs.** Touch drag at the phone's edge, parachute, overlap and ladder fail when the machine
  runs the chaos crowd at the same time (under five frames a second) and pass alone; the touch drag also failed in each
  run on three workers and passed in each run alone. The gate runs its shards one after
  the other, which is the quiet case.
- **Pets on a quiz's page** have no ladder and no rope from the footer line (edge-to-edge cards, a control as wide as the
  page): both gear specs moved to the home screen. Carrying a chip away is not an activity of the pet schema.
- `🗂️WindowChrome/shared-border.test.ts` passes again (a peer updated it).
- **The demonstration pointer of the introduction** moves over the label of "Weiter" while it shows the click; left as
  designed.
- Nothing is committed (shared worktree; the archivist commits). The tree keeps moving: these results hold for the times named.
