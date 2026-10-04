# 📓️ Overview like play — the quiz overview is the overview of play and the demonstrator

Requirement (2026-10-02, second wording, which decides): *exactly like semio-tech play or the mit-bestand demonstrator.
Two layers. The foreground has fixed cards arranged on a grid. On the second layer behind the glass are the individual
apps that are the size of the screen, and when the mouse hovers over the card the background camera zooms to the screen
and makes it crisp. When moving the mouse between the cards the background moves but is below the glass. Exactly like
the other pages, not something new.* Normative text: `📓️design.md` §19.

## 1. What happened first, and why it was wrong

The first wording the same morning ("a 2D map of the specific pages that changes the camera on cursor movement … all
pages are always visible in the overview") was read as: keep the grid of all nine scaled pages behind the cards and add
a pointer camera that never loses a page. That was built (a magnifying camera over the grid), reported, and rejected:
it was something new, not play. It is removed again, together with the grid backdrop of 2026-09-29 it stood on. Nothing
of it is left in the code: no `rest="grid"`, no camera, no extra preference.

## 2. What the overview is now

Exactly the mechanism play and the demonstrator run, the same shared element with the same defaults:

| Layer | What |
|---|---|
| front | the nine cards, fixed on the card grid (leaderboard in the larger centre cell) |
| glass | one `ui-veil` |
| back | the nine real pages, each as large as the overview, side by side on a 3 × 3 strip (2 × 5 on tablets), all live |

| The mouse … | The background … |
|---|---|
| moves between the cards | pans under the glass: top-left corner → first page, bottom-right corner → last page |
| is on a card (or the keyboard focuses it) | glides to that card's page in 500 ms; the glass opens over it and is gone once the page fills the screen |
| moves on that card | stays on that page |
| leaves the card | glass back at once; the pan resumes with the next movement |
| clicks the card's heading or action | the page opens full size (unchanged) |

Narrow or short viewports list the pages one below the other, each under its own glass and card — what play and the
demonstrator do on phones (unchanged).

## 3. Changes

### Framework (`🧰️framework/🔨️modules/🖱️ui`)

| File | Change |
|---|---|
| `🧱️elements/🥞️LayeredOverview/🟦️.tsx` | `reducedMotion` defaults to `"never"` (§4); grid rest removed: props `rest`, `gridTracks` (and the short-lived `panZoom`), `data-rest`, the overlay variables `--layered-columns`/`--layered-rows`, the per-page camera painting and the zoom drive. Kept: `data-pan="pointer" \| "none"` on the root; the follow steps by elapsed time; a warm pace of zero boots the next page at once |
| `🔨️modules/🥞️layered-overview-geometry/🟦️.ts` | removed `LayeredRect`, `LayeredTracks`, `LAYERED_VIEW`, `trackSpans`, `trackTemplate`, `restRect`, `lerpRect`, `glideRect`, `viewRect`, `coverPlacement`, `spanAxisBounds`, `veilForRect` and the camera helpers; `veilClip` computes the hole itself again; kept `followFactor`, `LAYERED_FOLLOW_FRAME_MS`, `followStep(current, target, elapsedMs?)` |
| `🧫️fixtures/🥞️layered-overview/🔣️.json`, `🧬️schema/🥞️layered-overview/🔣️.json` | grid-rest and camera vectors and their schema removed; `followFactors` and two timed `follows` kept |
| `🔮️oracles/🔣️.json`, package `devDependencies`, `bun.lock` | `d3-zoom` (added for the camera) removed again; `three` `MathUtils.damp` judges the timed follow |
| `🎯️targets/⚛️react/🪟️chrome/🟦️.ts`, barrel | exports follow the above |
| `🧱️elements/🥞️LayeredOverview/📖️stories/🧪️.story.tsx` | grid stories removed |
| tests | grid-rest cases removed; added: no pan under reduced motion / `pan="none"` / touch and `data-pan`; the revealed page holds while the mouse moves on its card and the pan resumes after; the pan covers the same distance at 60 and at 10 frames a second; a zero pace boots at once within the budget |

Play and the demonstrator are untouched; what reaches them is the follow by elapsed time (same speed at 60 Hz, no
longer twice as fast at 120 Hz or crawling when frames are slow).

### Quiz (`🧰️framework/🛍️products/❓️quiz`)

| File | Change |
|---|---|
| `🎯️targets/⚛️react/🔨️modules/🏠️home/🟦️.tsx` | uses the element as play does (no `rest`, no `gridTracks`, default pan); lifecycle `{ budget: pages, warmStartMs: 0, warmIntervalMs: 0 }` so all nine pages are live at once; card tracks handed to the stylesheet as `--quiz-home-columns`/`--quiz-home-rows` (`homeTrackTemplate`) |
| `🎯️targets/⚛️react/🎨️.css` | `.quiz-home-grid` reads those variables |
| `🎯️targets/⚛️react/🔨️modules/🎛️preferences/🟦️.tsx`, `🌐️i18n/🟦️.ts` | the preference `moveBackground` and its two texts, added for the first reading, removed |
| `README.md` | section "Overview" describes the two layers |
| `🧪️tests/🏠️home-grid/🟦️.tsx` | pages screen-sized on the strip in the cells of the card grid, all live; pan between the cards, glide and clear glass on a card, hold on the card, pan again; reduced motion: no pan, page at once |
| `🧪️tests/🐾️pet-companions/🟦️.tsx`, `🧪️tests/📡️presence-client/🟦️.tsx` | preference literals without `moveBackground` |

### Site (`🎓️teaching/🏛️architecture/❓️quiz`)

`🧪️tests/🥞️layered-home/🟦️.ts`: new end-to-end test "between the cards the mouse pans the screen-sized pages under the
glass; on a card its page comes to the screen, clear; reduced motion keeps them still"; the first test no longer asks
for `data-rest`; the hover test checks that the page fills the overview. `README.md`: spec table.

## 4. The freeze on this workstation, and its fix

Third report of the day: "when moving the cursor between the cards the background stays frozen to the last page; not
smooth like play and the demonstrator". Reproduced in a browser inside the owner's session:
`matchMedia("(prefers-reduced-motion: reduce)").matches === true`, root `data-pan="none"`, the strip never moved, a
hovered card's page snapped in and stayed. Cause: this workstation runs in a **Remote Desktop session**
(`TerminalServerSession = True`, `SPI_GETCLIENTAREAANIMATION = False`; the stored Windows preference is "animate"),
every browser in such a session reports reduced motion, and the shared element's default `reducedMotion="auto"`
switched pan and glide off for it. The original landings of play and the demonstrator never read that signal, which is
why they are remembered as smooth; their current dev builds froze the same way.

Fix: `LayeredOverview` defaults to `reducedMotion="never"` — pan and glide run whatever the device says, for the quiz,
play and the demonstrator alike. `"auto"` and `"always"` stay for an app that wants them. After the fix, in the same
browser (still reporting reduced motion): `data-pan="pointer"` and the strip follows the pointer.

## 5. Evidence

| Check | Command | Result |
|---|---|---|
| element + geometry | `bun ./📜️script.ts test quick layered` (ui-react package) | **142 passed** (after the motion fix too) (fixture vectors, schema check, oracles `polygon-clipping`, `d3-ease`, `three`) |
| quiz renderer | `bun ./📜️script.ts test quick` (quiz-react package) | **640 passed**, 20 files |
| types | quiz-react `bun ./📜️script.ts typecheck`; ui-react `tsc --noEmit -p tsconfig.json` | 0 errors, 0 errors |
| taxonomy | `bun ./📜️script.ts verify taxonomy report --scope …` for `❓️quiz`, `🎓️teaching`, the geometry module, the element, the fixture and schema folders | `clean=true` each |
| end to end | `bun nx run @teaching/architecture-quiz:test-e2e -- layered-home --project=desktop` | **dev 6 passed, rehearsal 6 passed** (65 s, after the motion fix): the site boot, the four older tests of the spec and the pan/glide test, which now also emulates a device that reports reduced motion and requires the same pan and glide there |
| mutants | `bun overview_camera_mutants.ts` (on copies) | **11 of 11 killed** |
| real browser, measured | `bun overview_camera_shots.ts 1440 900` (Chromium, normal motion, the running dev site) | below |

Measured at 1440 × 900 (overview 1440 × 845): strip 300 % × 300 %, nine pages live, each 1440 × 845.

| State | Glass | Page on the screen |
|---|---|---|
| at rest | whole | learner (first page) |
| pointer between the cards, bottom-right corner | whole | preferences (last page) |
| pointer between the cards, top-right corner | whole | how it works |
| pointer between the cards, bottom centre | whole | energy demand |
| pointer on the leaderboard card | clear (hidden) | leaderboard, at (0, 0) |
| pointer back between the cards, left edge | whole | heating |
| device reports reduced motion, pointer between the cards, bottom-right corner | whole | preferences (`data-pan="pointer"`) |
| device reports reduced motion, pointer on the leaderboard card | clear (hidden) | leaderboard |

The only console errors in that run were three `400` answers to the leaderboard query: the dev proctor that was
running predates another session's period leaderboards (`unknown field period`); nothing of this revision.

Mutants killed: by default a device that reports reduced motion freezes the overview; the follow closes 12 % per frame however long the frame took; the element steps its follow per frame;
reduced motion still pans; `pan="none"` still pans; a touch pointer pans; the mouse pans away from a revealed page; the
root always says that the mouse pans; a pace of zero still waits for an idle moment; the glass has no hole for a page
partly on screen; a cell's bounds ignore the pan.

Not run: the other end-to-end projects (`phone`, `presence`, `shortage`, `away`, `pets`) and `deploy-check` — other
sessions held the gate's ports for most of the afternoon; those specs open pages or use the list and do not look at
where the pages lie behind the cards. The play and demonstrator suites were not run either (their files are
untouched; the element's own tests cover the mechanism they use).

Screenshots (`🗑️generated/overview-moves/`, deleted with the ticket's generated folder; `overview_camera_shots.ts`
re-creates them): `1440x900-rest`, `-between-cards-bottom-right`, `-between-cards-top-right`,
`-between-cards-bottom-middle`, `-hover-leaderboard-gliding`, `-hover-leaderboard-clear`, `-between-cards-left-again`,
`-reduced-motion-between-cards-bottom-right`, `-reduced-motion-hover-leaderboard`.

## 6. Scripts kept in the ticket

| Script | Purpose |
|---|---|
| `overview_camera_shots.ts` | screenshots and page positions of the overview under normal motion |
| `overview_camera_probe.ts` | the pointer events the overview receives and how fast the pan settles |
| `overview_camera_mutants.ts` | mutation checks on copies (the repository's sources are never mutated) |
| `overview_like_play_spec_rewrite.py` | the one-off rewrite of the end-to-end spec |
| `overview_camera_ticket.py` | reopen/close bookkeeping while the repo MCP is down |

## 7. Open

- The overview now moves for people who did ask their device for less motion as well. If that matters for the quiz,
  the place for it is an explicit choice in the preferences (as the pets have), passed to the element as
  `reducedMotion="always"`; nothing of the kind exists today, by the owner's "nothing new".
- Over Remote Desktop the picture itself arrives at the session's frame rate, so the pan can look less fluid there than
  on the machine's own screen; the pan is stepped by elapsed time, so it does not get slower, only coarser.
