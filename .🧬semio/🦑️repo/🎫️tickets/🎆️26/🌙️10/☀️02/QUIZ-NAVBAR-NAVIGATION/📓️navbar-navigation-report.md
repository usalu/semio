# 🧭 Quiz Navbar Navigation

Request: move the description of the quizzes to the middle of the navbar, put the overview button on the navbar's left
and next to it a backward, a forward and an upward button.

## What the navbar is now

```
[▦ Overview][←][→][↑]          (logo) Architecture and Technology Quizzes          ✓ All answers saved  👥 Online: 3  [English|Deutsch]
```

- **Left — the ways** (`🔨️modules/🚏️navigation`, `NavigationControls`): overview, back, forward, up, one bordered
  group named "Go to". Each is a real button named with where it leads ("Back: Leaderboard", "Up: Heating"); the name
  is also the pointer's tooltip. A way that leads nowhere keeps its place and its tab stop and carries `aria-disabled`,
  so the bar never changes shape and focus is never lost. They show once a learner is identified.
- **Middle — what the quizzes are about**: the catalog title beside the site's logo, the navbar's `centered` item (true
  centre of the bar while it fits, else inside the free band). It is text, no longer the button that led home.
- **Right** — unchanged: connection, who is online, language.
- The overview button no longer floats over an opened page: `LayeredOverview` shows its own Overview button only when
  the app names one (`labels.overview` is optional now; play and the demonstrator still name it). The run card lost its
  "Overview" action for the same reason — one button, always in the same place.

## Decisions

| Question | Decision | Why |
|---|---|---|
| Whose history do back and forward walk? | The client's own trail (`QuizState.trail`, ephemeral local-only), folded by `evolveQuizState` | The layered overview has always replaced the address in place and written no browser history; a run has no address at all. A trail in the session is event-sourced, pure and testable, and it can drop steps that stop being places. |
| What happens to a closed run on the trail? | It leaves by itself: back and forward lead to the nearest step that *still stands* (overview and pages always, a run while open, results once submitted) | "Back" must never land on the answering screen of a submitted or voided run. No pruning pass is needed — a step that no longer stands is skipped and forgotten when the trail is walked. |
| Where does "up" lead? | Overview above its pages; the quiz's page above a run and its results (the overview when the catalog no longer lists the quiz) | The quiz page is where a run is started, resumed and its last result opened. |
| Who owns the address (`#board`)? | The client (`useAddress`), not `LayeredOverview` (`routing="none"`) | With two owners the hash went stale while a run showed, and returning home reopened whatever page it still named — "Overview" from a run would have landed on the quiz page. One owner: the hash names the page of the step and nothing else. |
| A page named by the address on arrival | Opens *instead of* the overview (`session.open(step, true)`): nothing lies behind it, "up" leads to the overview | As a deep link in a browser has no entry before it. |
| Browser back button | Unchanged: it leaves the site | Wiring the trail into the browser's history means addressable runs and results (deep links, loading states, stale ids after an identity switch) — a feature of its own. |
| Narrow navbars | The overview's word and the language names give way to icon and codes (EN, DE) below tablet width; the title ends in an ellipsis, then leaves the line, then the logo leaves — never cut, never over a control (`.quiz-brand`) | Four buttons take 110 px more than the logo did. Without this the largest text size on a 375 px phone left no room and the logo would have lain over its neighbours. |

## Files

| File | Change |
|---|---|
| `❓️quiz/🎯️targets/⚛️react/🔨️modules/🧭️session/🟦️.ts` | `QuizTrail`, `EMPTY_TRAIL`, `TRAIL_LIMIT`, `sameStep`, `stepBefore`, `stepAfter`, `stepAbove`; events `step-opened { instead }`, `step-retraced`; `QuizSession.back/forward/up`, `open(step, instead)` |
| `❓️quiz/🎯️targets/⚛️react/🔨️modules/🚏️navigation/🟦️.tsx` | new: `NavigationControls`, `navigationWays`, `placeName`, `addressPage`, `stepAddress`, `useAddress` |
| `❓️quiz/🎯️targets/⚛️react/🟦️.tsx` | navbar items (ways left, brand centred), `useAddress`, `screenTitle` on the shared labels, re-exports |
| `❓️quiz/🎯️targets/⚛️react/🔨️modules/🏠️home/🟦️.tsx` | `pageLabel` (one mapping for page names), `routing="none"`, no overview label |
| `❓️quiz/🎯️targets/⚛️react/🔨️modules/▶️run/🟦️.tsx` | the run card's "Overview" action removed |
| `❓️quiz/🎯️targets/⚛️react/🔨️modules/🪟️chrome/🟦️.tsx`, `🎛️preferences/🟦️.tsx` | `Segment.short`, `LanguageSwitch compact` |
| `❓️quiz/🎯️targets/⚛️react/🔨️modules/🌐️i18n/🟦️.ts` | `quiz.nav.ways`, `back`, `backTo`, `forward`, `forwardTo`, `up`, `upTo` (en, de) |
| `❓️quiz/🎯️targets/⚛️react/🎨️.css` | `.quiz-brand` |
| `🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx` | `LayeredLabels.overview` optional |
| `❓️quiz/🧫️fixtures/🚏️navigation/🔣️.json`, `❓️quiz/🧪️tests/🚏️navigation/🟦️.tsx` | new shared vectors and suite |
| `❓️quiz/🔮️oracles/🔣️.json` | oracle `quiz-react-jsdom-session-history` |
| `🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` | `🚏️navigation` as module, test and fixture member |
| tests touched | `🏠️home-grid`, `🚶️learner-journey`, `📡️presence-client`, `📇️learner-pages`, `💭️crowd-client` (state literals, moved assertions); `LayeredOverview` component suite (+1 case); site specs `🥞️layered-home` (+1 spec), `🐕️pet-walk`, e2e `goHome`/`way` |
| docs | quiz README "Finding the way", site README spec table |

## Tests

- Shared vectors (`🧫️fixtures/🚏️navigation`): eight trails (pages, run, arrival, results, submitted, closed elsewhere,
  closed ahead, voided), step addresses, hashes that name a page.
- Third-party oracle: every trail that closes no run is replayed on the session history of a fresh **jsdom** window
  (`pushState`, `replaceState`, `go`); step in front and the steps one entry behind and ahead must agree after every
  move.
- Client cases in jsdom: order and names of the ways, brand in the centred slot, no way before identification, the full
  walk (card → back → forward → Escape → up → run → up → back → results → up) with names, address and unchanged
  `history.length`, unavailable ways staying focusable, arrival by hash, a hash changed later, the address kept through
  a first visit.

## Verified

Other sessions were rewriting the crowd, preferences and leaderboard modules of the same working tree during this
ticket, so the tree was consistent only part of the time. What was run, and when it means what:

| Check | Result |
|---|---|
| `@semio-tech/quiz-react` suite, tree consistent (11:49) | 19 files, 539 tests passed — with the new `🚏️navigation` suite and the adjusted `🏠️home-grid`, `🚶️learner-journey` |
| `🚏️navigation` suite, latest tree (12:38) | 40 of 40 passed |
| `@semio-tech/quiz-react` suite, latest tree (12:35) | 20 failures in 5 other files, none in navigation code: `state.asked` missing in test states, crowd exports moved, a result table with two "Desert" row headers, a forced-colours rule the contrast suite's own selector parser cannot read — all from the concurrent refactor |
| `quiz-react` typecheck | exit 0 at 11:48; at 12:35 errors only in files of the concurrent refactor (`asked`, `showAnswers`, `Leaderboard.period`, crowd exports), none in the files of this ticket |
| `LayeredOverview` component suite | 32 of 32 passed (one new case: no Overview button for an app that names none) |
| `verify taxonomy report --scope 🧰️framework/🛍️products/❓️quiz` | the three `🚏️navigation` directories are accepted; the two remaining errors are the other session's `📊️plot` module and a file that changed under the verifier |
| Real browser, dev site, `navbar_shots.ts` (Chromium, 7 cases × 6 screens: 1440 and 1280 desktop, 820 tablet, 375 phone at normal, larger and largest text, 360 phone at largest) | no console problem, no overlap of the brand with a control, page and navbar never wider than the viewport; brand centre = navbar centre on desktop (720/720, 641/640); hash `#board` / `#heating` on pages, empty on the overview and in a run |
| Real browser, by hand in the app's pane | card → leaderboard (`#board`, Back "Back: Overview", Overview names Escape) → Back → Forward → Overview; Resume → run (the run card offers only "Submit quiz") → Up → quiz page `#heating` → Back → run → Overview lands on the overview; `history.length` stays 1 |
| End-to-end, `🥞️layered-home` specs matching "navbar" (dev topology, proctor built from the tree, 12:53) | 2 passed: the page by hash, card, Escape and the navbar's overview button; the walk overview → back → forward → run → up → back with names, address and the brand within 1 px of the navbar's centre |
| The same two specs against the dev proctor binary of that morning | every assertion of the specs held; the fixture's "no console problem" check failed on `400` answers to `/queries` — the client of the concurrent refactor asks a query the older binary does not know |
| End-to-end, whole dev topology (12:58, 7.3 min) | 10 passed, 8 failed, 16 did not run (the projects that wait for desktop and phone). Passed with this ticket's changes in them: both navbar specs, `🪪️first-visit` (4, `goHome` from a run by the navbar), `🏆️live-leaderboard` (`goHome` from results), hover/focus, the language specs. The 8 failures are the concurrent rework: result tables with twice the rows (`🎯️quiz-runs` ×3, `📱️phone`, `🗣️both-languages` ×2 — all fail in `expectFeedback` or the wording comparison of the results, before any navigation), and the leaderboard card that grew a period switch (`🥞️layered-home`: "centred down" off by 2.008 px instead of < 2, and the camera spec, where the taller card now lies under the pointer left by the identity step and reveals its page) |
| End-to-end, specs of the projects that did not run, alone | `🐕️pet-walk` "pets live on the home screen" (closes the page with the navbar's overview button): passed. `👥️shared-presence`: fails on "no item moved when the figure opened below the task", an assertion of the concurrent figure feature |

The gate's run directories of this ticket under `.🧬semio/🎓️teaching/architecture-quiz-e2e/` were removed again.

## Left for others

- The eight end-to-end failures and the jsdom failures listed above belong to the sessions reworking results, crowd,
  preferences and the leaderboard; the navigation code and its specs do not depend on them.
- The logo asset `🖼️assets/🪧️logos/🛡️emblem/🌘️dark-round/🖋️vector.svg` carries a stray `--&gt;` text node (invisible,
  inside the `aria-hidden` logo); flagged as a separate task.
- Browser history: the trail is the client's own. Making the browser's back button walk it needs addressable runs and
  results and is a feature of its own.

Phone widths at the larger text sizes are where the brand gives way: 375 px normal shows the logo and "Architectu…",
375 px larger the logo alone, 375 and 360 px largest neither — and the controls stay whole and apart in all of them.
