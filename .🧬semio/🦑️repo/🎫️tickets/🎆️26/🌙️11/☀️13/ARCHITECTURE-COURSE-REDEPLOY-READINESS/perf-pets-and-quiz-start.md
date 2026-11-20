# Pets survey tick and quiz start: two main-thread long tasks

Ports 8800 (proctor) and 6069 (site) only. Launchers: `perf2-stack.mjs`, `perf2-build.mjs`, `perf2-native-stack.ts` (copies of the intro and native stacks, new names). Profiling: `perf2-profile.mjs` (all 10 quizzes, CDP CPU throttling via `RATE`, long tasks, event timing, per-frame cost of the pets frame callback with the DOM reads inside it, nodes added per commit), `perf2-tasks.py`, `perf2-incl.py`, `perf2-show.py`.

The host was never quiet: load average 60 to 130 during every run, so wall-clock numbers at 1x are noisy and part of every "before" and "after" at 1x is host contention. Counts, contracts and the 20x runs (where the page's own work dominates the noise) are the evidence; the 1x numbers are given for the record.

## What the data say first (this changes the brief)

1. **The survey itself is small.** On the task screen a survey is about 75 `getBoundingClientRect`, 73 `getComputedStyle`, 49 `Range.getClientRects` and 227 `matches` calls, about 3 ms at 1x (frame `max` 3 to 6 ms with the survey in it). The cost the brief called `tick` is the pets frame callback as a whole: about 100 frames per 3 s, `p50` 0.3 to 0.6 ms, `p90` 1 to 3 ms, `p99` 3 to 6 ms at 1x. The 150 ms per 2 s is the sum of all frames, not one long task. In a quiet moment no pets frame at 1x exceeds 16 ms (before: `over16` 0 in 9 of 10 quiz-start windows, max 10.6 ms; the 31.8 ms and 158.7 ms outliers seen coincided with load average above 100).
2. **What a CPU profile at 20x attributes to the pets frame** (20 ms at 20x = 1 ms at 1x): `frameOf` (poses of every actor) 689 ms in the 2 s window, `survey` 34 to 46 ms per survey frame, `paint` 7 to 25 ms, `step2 > advance` rarely above 25 ms. The survey frame is therefore the one place where a single frame is several times larger than its neighbours, and it is the one that grows with the page (quiz pages with many text lines and controls, more `kept` elements).
3. **Quiz start is not dominated by item rendering.** The largest commit of a quiz start is 104 DOM nodes of `course-run` plus the pets layer arriving: 6 pets drawn at once are 290 to 340 SVG nodes in one commit (`div.pet-layer`, `svg:48 svg:38 svg:53 ...`). At 20x the profile of the start window is 7,672 ms `(program)` (browser layout, style, paint), 874 ms `removeChild` (React unmounting the previous screen, about 44 ms at 1x), 383 ms GC, 258 ms `getBoundingClientRect` (the shared border compositor's `snap`), 255 ms `renderRootSync`, and inside the first pets mount `menagerieIssues` 69 ms (the validation of the menagerie, about 3.5 ms at 1x, once) and `presenceIssues` 70 ms.
4. **Where the sheet was computed again and again.** `sheetOf(quiz, seed, challenge)` deals the whole sheet from the seed; it was called from `runView` (every fold of every accepted command: starts and answers), from `hints`, from the lifecycle decisions (`unknown-task`, answer checks, submit: three calls per decision) and from the deputy's `alike(view.sheet, sheetOf(...))`. Each call re-shuffled and re-sampled every task, with pools on top.

## Changes

| Item | File | Test (written first) |
| --- | --- | --- |
| The survey is a generator (`surveying`); `survey` runs it to the end, so the vectors, fixtures and the Python reading are unchanged. `surveyJob(root, options, now)` runs it in slices of a time budget, one element per step, at least one step per slice, all reads (no write) | `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/📡️survey/🟦️.ts` (exports in `🎯️targets/⚛️react/🟦️.tsx`) | `🧪️tests/📡️surface-survey/🟡️slices-across-frames/🟦️.tsx`: for every vector scene the slices add up to the survey of one go (`toEqual`); with a spent budget a slice reads at most one element and leaves the page untouched; one slice finishes when the budget is not spent |
| The layer surveys in slices of `SURVEY_SLICE_MILLISECONDS` (4 ms) per frame, wakes the pacer for the next slice, restarts the job on an urgent stale (`now`), keeps going when stale is `soon`, drops the job when the document hides or the layer goes inert, and stages the `surveyed` event when the last slice read | `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/🫧️layer/🟦️.tsx` | `🧪️tests/🫥️decorative-layer/⏱️sliced-survey/🟦️.tsx`: 40 surfaces, a clock that moves 1 ms per rect read: at most 8 reads in any frame, at least 6 frames with reads, nobody on stage after 3 frames, the pets stand once all of it was read |
| A menagerie is checked once per object, not once per mounted layer (`soundOf`, `WeakMap`) | same layer file | same new test file: a layer mounted three times reads the validator's `schema` field only the first time |
| `sheetOf` deals once per quiz object, seed and challenge (`SHEETS_KEPT` = 16 per quiz, first in first out); the Rust `sheet_of` and the vectors are untouched | `🧰️framework/🛍️products/📚️course/🔨️modules/🃏️sheet/🟦️.ts` | `🧪️tests/🎴️sheet-randomization/🟦️.ts`: same ask is the same object, other quiz object, seed or challenge is not, an evicted sheet is equal when dealt again |
| Tasks show their first item at once and the rest in chunks of `ITEM_CHUNK` (4) through `startTransition`; a task never starts over when answered; another task id starts again; a shorter list is shown whole | new `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🧱️gradual-items/🟦️.ts`, used by `🗂️classification`, `↕️sorting`, `🃏️matching` | `🧪️tests/🧱️gradual-items/🟦️.tsx` (listed in the course-react vitest config): hook contract (first commit 1, at most one chunk per commit, ends at the total) and, per task kind, 30 items: the first commit has 1 row, no commit adds more than a chunk, 30 rows at the end, an answer is one commit |

A list that waits for more items says so: `RowList` takes `aria-busy`, and the three task views set it while `shownCount` is below the item count. This was found by the e2e gate, not by the unit tests: the first quiz-runs run failed 5 of 15 (`balanced pools rotate ...`, `shownItems` read 1 item of 3 or 6 because the spec read the task the instant it opened). The e2e readers now wait for `[aria-busy="true"]` to vanish (`itemsSettled` in `🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🚶️learner/🟦️.ts`, called by `shownItems`, `keysHidden`, `shownHints`, `itemBoxes`, `sortedItems`; the same one-line wait in `🧪️tests/📐️adaptive-layout`, `📱️phone` and `⛰️challenge-levels` where they read all items at once), and the unit contract checks that a list is busy in every commit before the last and not after.

Small supporting edits: the harness of the survey suite got `optionsOf(scene, frame)` (used by the vector test and the slice test); `README.md` of the course names the sheet cache. Not done on purpose, with the reason:

- **No per-element geometry cache with ResizeObserver, MutationObserver or scroll invalidation.** A box moves when anything above or beside it reflows, and none of the three observers tells which elements moved: a cached rect for an element no observer touched can be wrong, which would change the survey (lines, open faces, stage-floor solids, `COURSE_PET_CONTROLS`), and the periodic patrol exists to catch exactly such unobserved moves. The survey stays exact; the work is spread instead. What is cacheable without risk (the validator result per menagerie, the sheet per quiz and seed) is cached.
- **Pets frames stay at about 1 ms at 1x on average**; making `frameOf` cheaper per frame would touch the pets core and its Rust and Python twins, which is outside what the data justify (no frame of the core is above 16 ms at 1x).

## Before and after

### Pets (frame callback of the layer, wall time, 1x)

Task screen, 3 s idle and 2 s around an answer, six alternating runs (before, after) at load 60 to 130: `p50` 0.3 to 0.6 ms in both; `p99` 3.4 to 6.6 ms in both; frames above 16 ms 0 to 2 per window in both and not tied to the change (host contention: the same runs show 50 to 200 ms long tasks with nobody touching the page). At 1x no pets frame is above 16 ms when the host lets the page run; this was already so before.

At 20x CPU throttle, idle on the task screen (3 s of pets time), frame callback:

| | frames | p50 | p90 | p99 | max | frames over 16 ms (= over 0.8 ms at 1x) |
| --- | --- | --- | --- | --- | --- | --- |
| before | 991 | 0.6 | 10.9 | 41.7 | 64.9 | 50 |
| after | 933 | 0.5 | 11.9 | 31.2 | 45.9 | 51 |

The worst frame at 20x fell by 30 percent (64.9 to 45.9 ms, 3.2 to 2.3 ms at 1x), the p99 by 25 percent. The survey frames are now at most one element past 4 ms; the remaining large frames are `frameOf` and `paint`, which are not survey work. Frame counts differ slightly because the run lasts the same wall time and a throttled frame takes longer.

### Quiz start, all 10 quizzes (`?preview=1`, window of 2 s after the start click, longtask observer = tasks over 50 ms)

At 1x (host load 60 to 130):

| quiz | before: tasks over 50 ms / longest / sum (ms) | after |
| --- | --- | --- |
| 0 physics | 2 / 86 / 164 | 5 / 98 / 351 (cold chunk load, load above 100) |
| 1 ventilation | 2 / 130 / 245 | 0 |
| 2 heating | 1 / 53 / 53 | 0 |
| 3 cooling | 2 / 132 / 214 | 1 / 54 / 54 |
| 4 demand | 3 / 121 / 233 | 0 |
| 5 parametric-design | 5 / 187 / 570 | 0 |
| 6 grasshopper-data | 3 / 176 / 297 | 1 / 65 / 65 |
| 7 excel-workbook | 2 / 108 / 163 | 1 / 94 / 94 |
| 8 bim-concepts | 1 / 51 / 51 | 1 / 57 / 57 |
| 9 optimization-decisions | 3 / 56 / 161 | 0 |
| quizzes with a task over 50 ms | 10 of 10 | 5 of 10 |
| sum of long-task time | 2,151 ms | 621 ms |

At 20x CPU throttle (an 50 ms task at 1x is 1,000 ms here, so the threshold of the observer, 50 ms, is 2.5 ms at 1x and everything counted is real work):

| quiz | before: tasks / longest / sum (ms) | after | pets frames over 16 ms, before to after |
| --- | --- | --- | --- |
| 0 | 23 / 944 / 3,658 | 20 / 535 / 2,027 | 15 to 18 |
| 1 | 17 / 539 / 2,843 | 2 / 145 / 257 | 9 to 3 |
| 2 | 24 / 512 / 2,540 | 3 / 110 / 275 | 11 to 2 |
| 3 | 24 / 343 / 2,807 | 5 / 208 / 488 | 10 to 10 |
| 4 | 26 / 299 / 2,953 | 9 / 414 / 1,075 | 9 to 9 |
| 5 | 21 / 631 / 2,673 | 7 / 141 / 632 | 10 to 4 |
| 6 | 23 / 788 / 4,500 | 11 / 275 / 1,161 | 10 to 6 |
| 7 | 18 / 517 / 2,447 | 10 / 178 / 1,132 | 10 to 6 |
| 8 | 26 / 925 / 3,114 | 9 / 167 / 752 | 21 to 11 |
| 9 | 21 / 249 / 2,099 | 9 / 282 / 925 | 7 to 5 |
| sum | 29,634 ms | 8,724 ms | |

Long-task time at quiz start fell by 71 percent at 20x and 71 percent at 1x; the longest task at 20x fell from 944 to at most 535 ms (47 ms at 1x). The first start of a session (quiz 0) still carries the lazy chunk loads and the first mount of the pets layer; that part is not item rendering.

Answering (events over 8 ms of handler time): none, before and after, at 1x and 20x; answering is one commit (test) and the sheet is not dealt again.

### What is still above the targets

- At 1x on this host, 5 of the 10 quiz starts still show a single task of 54 to 98 ms in some runs, while the other runs of the same quiz show none: the host's load average of 60 to 130 decides. The remaining work is not item rendering but the unmount of the previous screen (about 44 ms at 1x in the profile) and the first mount of the pets layer (6 pets drawn in one commit), plus the shared border compositor's `snap` reads. A confirmation at 1x needs a quiet host.
- At 20x the pets frame stays over 16 ms in about a twentieth of the frames (`frameOf`, `paint`), which is the same absolute work as 0.8 ms at 1x.

## Tests run

- E2E, through the same Playwright CLI as the other tickets' `pw-pets.sh` (`repo tools exec playwright` fails here with "did not expect test() to be called" because it resolves a second `@playwright/test`), on `perf2-native-stack.ts` (dev topology, 8800 and 6069): quiz-runs (`desktop` project, `🎯️quiz-runs`) 15 of 15 passed after the `aria-busy` fix (8.5 min); the pets project 24 of 24 in the final whole-project run (4.4 min; the first pass had 23 of 24 with `circling the pointer round a pet changes its state` failing under load average 100 with three workers; the same test alone passes in 12.6 s). The dev server runs with `TEACHING_ARCHITECTURE_COURSE_WATCH=off` and keeps serving modules it transformed before an edit, so the stack must be restarted after a source edit; the first quiz-runs rerun failed for that reason (stale view code without `aria-busy`).

- Typecheck: `tsc --noEmit` on the pets-react package: 0 errors; on course-react: 1 error in `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧾️typed/🟦️.ts` (`ContainerRole` has no `card`), a file I did not touch. `repo run <project>:typecheck` waited more than an hour on the graph service and was stopped.
- Pets-react (`@semio-tech/pets-react`, whole config, 9 files): 254 passed with the slice and sliced-layer tests included; the survey suite alone 72 passed.
- Pets core (`@semio-tech/pets`, 34 files): 1,166 passed. No Rust crate and no fixture changed, so no cargo run was needed.
- Course core (21 files): 620 passed (sheet-randomization 18 passed with the new case).
- Course-react (48 files): 1,131 passed and 15 failed in the first whole run under load average 100 (5 s per-test timeouts: keyboard, navigation, home grid, pets-on-stage, leaderboards, text size); the seven files that held those failures passed completely (208 tests) when rerun with `--testTimeout 120000`, and `task-keyboard` alone passed 21 of 21.

# Follow-up (coordinator): quiz-start residue and memory per learner page

Same ports (8800, 6069), `perf2-*` scripts, host load average 15 to 25 during every measurement below (quiet enough: the 50 to 200 ms noise tasks of the first part are gone).

## 1. Quiz-start residue, measured on a quiet host

Three full passes over the 10 quizzes before and after, `?preview=1`, 2 s window after the start click, longtask observer (tasks over 50 ms), unminified release build:

| | quiz 0 (first start of the session) | quizzes 1 to 9 |
| --- | --- | --- |
| before, pass 1 / 2 / 3 | 75 / 65 / 65 ms | no task over 50 ms, 3 of 3 passes |
| after the idle preload, pass 1 / 2 / 3 | 65 / 69 / 68 ms | no task over 50 ms, 3 of 3 passes |

So 9 of 10 quizzes were already clean on a quiet host (the 5 of 10 of the first part was host load), and the residue is a single task of 65 to 79 ms on the first quiz started in a page session. Starting quiz 5 first moves it to quiz 5 (66 ms, then quiz 6 clean): it is the first start, not a quiz. Pets frame callback in these 60 windows: no frame over 16 ms, worst 5 ms.

What the CPU profile of that first start shows (second start for comparison): a 138 ms stretch of native work (second start 41 ms), React's commit 69 ms of which `removeChild` of the home screen about 52 ms (second start 26 ms), render 20 ms, one GC 7 ms. The pets are not part of it: the pets chunk, the menagerie and its check are loaded and run while the home screen is shown (`CoursePetsProvider` fetches as soon as the choice is not `off`), 3.5 ms at 1x, and the pets rest during a run, so there is no first pets mount at quiz start to spread. The residue is first-run cost of the browser and V8 (lazy compilation of every function of the run screen on its first call, first style and layout of the new screen, first teardown of the 7,000-node home with its 477 compositor layers), which grows with nothing in the sheet.

Landed: `💤️idle-preload` (`whenIdle`, `usePreload`): while the home screen rests the shell asks for the run and results screen chunks in idle time, so a slow network no longer fetches and parses them at the click. Tests first: `🧪️tests/💤️idle-preload` (idle callback or timer fallback, once per activation, called off with the screen, nothing while inactive). On localhost it did not move the 65 to 69 ms (the chunks are cheap to fetch there); it is kept for real networks and costs one idle task.

Not landed, with the reason: the unmount of the previous screen is the home's 7,000 nodes and 477 layers torn down by the browser in one `removeChild`; React only issues that one call, so there is no synchronous cleanup (observers, effects that measure on unmount) in it to remove (the profile of the unmount shows no JS frames under `removeChild`). Making it cheaper means fewer nodes and layers on the home screen (next section), a design decision for the layered overview. The first-run V8 compilation cannot be spread: the navigation state arrives through `useSyncExternalStore`, which React always renders synchronously, so `startTransition` cannot slice it. The target (no task over 50 ms for all 10 quizzes) is therefore met for 9 of 10 and missed by 15 to 29 ms on the first start of a session.

## 2. Memory per learner page

Walk (`perf2-memory.mjs`): intro, identity, home, then each quiz started, one item answered, submitted, results shown, back to the grid; then the grid, the leaderboard and 15 s of rest. Every stop after three forced garbage collections: V8 heap (`Performance.getMetrics`), DOM nodes, documents and JS event listeners (`Memory.getDOMCounters`), and the resident memory of the renderer and of the whole browser tree (`SystemInfo.getProcessInfo` plus `ps`). Before and after are the same walk on the build without and with this follow-up's changes (none of them touches memory), 10 quizzes each, plus a 30-run walk (the ten quizzes three times over) to look for growth:

| stop | heap MB | DOM nodes | listeners | renderer MB | browser tree MB |
| --- | --- | --- | --- | --- | --- |
| intro | 20.1 / 20.3 | 7,641 / 7,648 | 1,255 | 386 / 391 | 673 / 665 |
| home | 19.9 / 20.0 | 7,138 / 7,197 | 1,283 | 422 / 434 | 704 / 701 |
| quiz 1 run | 16.7 / 17.1 | 1,093 / 1,309 | 302 | 365 / 396 | 631 / 680 |
| quiz 1 results | 18.1 / 18.1 | 2,598 / 2,615 | 445 | 342 / 379 | 603 / 682 |
| quiz 5 results | 19.7 / 19.9 | 1,781 / 1,783 | 436 | 384 / 379 | 659 / 661 |
| quiz 10 results | 20.4 / 20.7 | 1,103 / 1,106 | 436 | 452 / 346 | 755 / 582 |
| grid after all quizzes | 25.1 / 25.4 | 10,144 / 10,160 | 847 | 530 / 465 | 839 / 732 |
| leaderboard | 25.4 / 25.5 | 10,516 / 10,532 | 885 | 590 / 512 | 909 / 780 |
| steady, 15 s later | 25.6 / 25.7 | 10,516 / 10,532 | 885 | 444 / 441 | 634 / 656 |

(first figure before, second after.) The resident figures swing by 100 MB between runs of the same walk (macOS memory compression and tile eviction), so they carry no before and after signal; the heap, DOM and listener counts repeat to within 1 percent.

Findings:

- **There is no leak in the walk.** The 30-run walk: heap 18.2 MB after run 1 and 22.2 MB after run 30 (0.14 MB per run, the learner's own runs, answers and results, held by the session as they must be; flat from run 12 on at 21.3 to 21.4 MB until the last), DOM nodes after each results screen between 1,100 and 2,800 with no trend, listeners 432 to 460 with no trend, documents 4 to 5. Nothing is retained that was not asked for: the number of listeners after the leaderboard (885) and the grid (847) is the same after 10 and after 30 runs.
- **Caches are bounded.** `cn` empties each of its three tables at 20,000 entries (`SPLITS`, `GROUPS`, `COMPOSED`): new test, 45,000 distinct lists compose correctly across the bound and a composed list afterwards costs no pattern test (`bun test`, 3 of 3). `sheetOf`: 16 sheets per quiz object, first in first out, weak by quiz (test of the first part). Presence watches at most `MAX_WATCHED_ROOMS` (16) rooms. The solo walk exercises no peer buffers; the mixed-100 run is where those would show, and nothing in the code keeps an unbounded list.
- **The heap is not where the memory is.** The 330 to 470 MB of a live learner tab is the renderer's native memory: JS heap 20 to 27 MB. A Chrome memory-infra dump of the renderer at the home screen: `cc` (compositor tiles and layers) 160 to 175 MB, `shared_memory` 30 to 170 MB (software-raster bitmaps in headless Chromium, moving with tile eviction), `malloc` 70 MB, `blink_gc` 65 to 77 MB (the DOM of the home), `v8` 24 MB, `partition_alloc` 24 MB, `web_cache` 12 MB. At the run screen (700 nodes) `cc` is 70 MB and `blink_gc` 27 MB, so the home screen costs about 100 MB of compositor and 40 MB of DOM more than a quiz does, and a tab that sits on the home or the leaderboard is the expensive one.
- **Why the home is heavy.** The layered overview keeps every page of the strip as a live inert grid: about 7,000 to 10,000 nodes, 477 compositor layers (406 draw content) of 170 megapixels in all, the strip itself a 3,843 by 6,282 px layer of 24 megapixels, and 303 elements with `backdrop-filter` (115 window glass surfaces with `blur(32px)` over 29 megapixels, 10 veils, 178 chips), 84 of the 115 glass surfaces wholly off screen. Experiments on the built site with the CSS rewritten in flight, two rounds each: dropping every `backdrop-filter` takes `cc` from 171 to 100 MB and `shared_memory` from 76 to 39 MB (about 100 to 140 MB per tab); dropping every `will-change` makes it worse (`cc` 204 to 211 MB), because the strip is promoted by its `translate3d` anyway; clearing `backdrop-filter` per element from script, or only for off-screen elements, changes nothing (the CSS blur lives on more than those elements).
- **What would cut it, not done because it changes how the design system looks or is built, a decision for its owners:** a `backdrop-filter` only where the glass is visible (not on the 84 off-screen windows and the 178 chips), or panes beyond the neighbours of the revealed page as placeholders instead of live grids (the component's own comment says only pages near the view keep placeholders, but all 7,000 nodes are live), or a strip made of the visible cells instead of one 24-megapixel grid. Each would drop the home toward the quiz screen's footprint (about 100 MB of compositor and 40 MB of DOM).

## Tests run for the follow-up

- Course-react, whole config, 49 files: 1,150 passed on the quiet host (no timeout raised beyond 120 s per test, none needed). Course core 620, pets-react 253 (one slice test removed in the cleanup of the first part), pets core 1,166 (first part) passed. UI `class-name-composition/repeat-composition` (`bun test`): 3 of 3.
- E2E on the dev stack of `perf2-native-stack.ts` (restarted after the edits because the watcher is off): see the result lines at the end of this file.

E2E result lines: quiz-runs (`desktop` project) 15 of 15 passed (10.1 min), pets project 24 of 24 passed (5.6 min), both on a quiet host with three workers; the warm-up `boot` spec passed first.

# Follow-up 2 (coordinator): compositor cost of the home screen

Ports 8800 and 6069, host load average 12 to 25, `perf2-compositor.mjs` (layers and megapixels from `LayerTree`, allocator breakdown from memory-infra, a pointer pan across the overview with rAF intervals), `perf2-look.mjs` and `perf2-diff.mjs` (screenshots and a pixel difference), the unminified release build before and after, three alternating runs each.

## What was done (tests first)

| Change | Files (under `🧰️framework/🔨️modules/🖱️ui/`) | Tests |
| --- | --- | --- |
| **Off-screen culling.** One `IntersectionObserver` per root, margin `100% 100% 100% 100%` (a viewport on every side), threshold 0, marks `data-offscreen` on whatever is away, unmarks it when it comes near, ref-counts an element watched twice and disconnects with the last one. A root's margin is the only margin its clipped children get (an `overflow: clip` ancestor cuts a target before the margin counts), so the observer is per root and the overview's own box is the root. The overview watches its pages (`data-layered-pane`) and, in swipe mode, its card hosts (`data-layered-cell`; a strip-mode host is `display: contents` and has no box, so it is not watched). The framework stylesheet drops `backdrop-filter` of `.ui-glass` and `.ui-veil`, `will-change` and `box-shadow` under `[data-offscreen]`; fill, borders and geometry are untouched | `🔨️modules/👁️offscreen-culling/🟦️.ts` (new), `🧱️elements/🥞️LayeredOverview/🧵️pane/🟦️.tsx`, `🎨️styling/🖌️ui/🎨️.css` | `🔨️modules/👁️offscreen-culling/offscreen-culling.test.tsx` (7: one shared observer and its margin; nothing marked before the observer speaks, marked when away, unmarked when near; one observer per root; cleanup unmarks, unobserves and disconnects with the last element, and a later watch makes a new observer; double watch is counted; no observer, nothing marked; the hook watches against the root its caller names and lets go on unmount) and a LayeredOverview component test (one observer with the overview as root, every pane watched, the pane the observer calls away is marked) |
| **The 24-megapixel strip.** The strip was kept as one layer by `will-change: transform` and by its permanent `translate3d`. Every write of its transform (once per frame of a pan, glide or settle) now promotes it (`will-change: transform`, the 3D translation) and 250 ms after the last write the same translation is written in 2D and `will-change` is given up, so a resting strip belongs to its parent's layer. The geometry (`stripTransform`, its shared fixtures and twin) is unchanged; the flattening is local | `🧱️elements/🥞️LayeredOverview/🔼️pan-promotion/🟦️.ts` (new), `🔶️panes/🟦️.tsx`, `🟦️.tsx` (the two `will-change-transform` classes removed) | `🔼️pan-promotion/pan-promotion.test.ts` (4: 3D to 2D flattening; promoted while written to and released after rest; waits for the last write of a pan; promotes again and keeps a timer per element; no timer left) and the component test above (the strip rests with no `will-change` and a 2D translation). The 21 existing assertions on the strip's transform now compare the flattened translation (same position, same numbers) |

`shared-border.ts` (re-read first; another agent had rewritten it to paint imperatively) reads rects and computed clip styles only, never `backdrop-filter` or `will-change`, so culling cannot change its geometry; its 37 tests and the whole ui-react suite (57 files, 1,381 tests, via a copy of the repo config without the cache-folder policy this shell lacks: `perf2-ui-vitest.config.ts`) pass. `content-visibility: auto` on off-screen pages was not used: a skipped subtree has no layout, so the rects the shared borders and the container queries of the live inert pages read would go stale until the page returns, which trades a compositor saving for a risk to the geometry the brief says must stay correct.

## Before and after (three runs each, same machine state)

| | before | after |
| --- | --- | --- |
| compositor layers on the home | 477 / 477 / 476 | 118 / 118 / 118 |
| total layer area | 171.5 MP | 30.6 MP |
| biggest layer (the strip) | 24.1 MP | 1.8 MP |
| compositor memory `cc`, home | 134 / 132 / 135 MB | 96 / 95 / 96 MB (28 percent less) |
| compositor memory `cc`, leaderboard | 319 / 317 / 318 MB | 233 / 237 / 242 MB (25 percent less) |
| Blink GC and malloc, home | 73 and 72 MB | 73 and 72 MB (the DOM is unchanged) |
| pan, rAF interval p50 / p95 over 120 frames | 100 / 133, 99.9 / 133, 83.3 / 100 ms | 83.4 / 133, 83.3 / 100, 83.3 / 100 ms |
| first quiz-start task, three passes of ten quizzes | 75, 62, 64 ms (quiz 0), and 55 ms once on quiz 6 | 65, 65, 80 ms (quiz 0) |

Reading: the layers fell by three quarters and the compositor memory by a quarter; the pan did not regress (headless Chromium paints in software here, so its frame intervals are long in both builds, 83 ms at best; the median is the same or better). `shared_memory` and the renderer's resident size swing by 100 to 250 MB between runs of the same build (tile eviction timing), so they carry no signal and are not claimed. What is left on the home: 118 layers and 96 MB of `cc` for the visible pages, the cards, the veil and the pets, and a leaderboard that still holds 237 MB of `cc` (its page is a long scrolling table of surfaces that are on screen or within a viewport, which culling keeps by design).

The first quiz start did not move: 62 to 75 ms before, 65 to 80 ms after. The layers of the home are not what that task spends (that was the hypothesis behind removing them from the unmount); the residue of the first follow-up stands.

## Looks

Pixel difference of the home at rest and after a pan, in the default look, forced colours, reduced motion and dark, 1280 by 800 px: two runs of the same build already differ by 250,000 to 570,000 pixels at rest (animations, pointer state, the introduction's last frames), so the change is measured against that floor: before against after differs by 166,000 (default), 125,000 (dark), 571,000 (reduced) and 758 (forced) pixels at rest, within or below the floor of the same build against itself (248,000, 509,000, 567,000 and 1,068), and by 689, 863, 615 and 247 pixels after a pan against a floor of 644, 428, 500 and 353. No difference above the noise of the build itself. The forced-colours and reduced-transparency rules of the stylesheet already switch the blur off for everyone; the culling rules add nothing to them and do not touch `forced-color-adjust` or the fallbacks. Reduced motion is untouched: the pan, the glide and the settle run as before, and the promotion follows their writes.

## E2E

See the result lines below.

E2E on the dev stack of `perf2-native-stack.ts` (8800 and 6069), restarted after every source edit because its watcher is off, with my changes in the tree:

- quiz-runs (`desktop` project): 15 of 15 passed. pets: 24 of 24 passed. phone: 2 of 2 passed. `boot` passed.
- layered-home (`desktop` project, 6 tests): 5 passed, 1 failed: `home, topics and energy preserve the layered glass grid and immediate reading order` ("the card sits inside its grid cell": the card grid's band heights do not match the equal thirds the spec computes, a layout of the overview cards that the spec and the card grid disagree on). It fails the same way with one worker, and **the same way with my four edits (`🔶️panes`, `🟦️`, `🧵️pane`, the stylesheet block) reverted and the stack restarted** (checked once: 5 passed, 1 failed, `clips` instead of `badges` as the first card outside its cell), so it is not from the culling or the strip promotion; it needs its own ticket.
- adaptive-layout (`layout` project, 4 tests): fails with my changes (1 of 4 passes; the failures are a German introduction dialog that intercepts the click on the learner card in the first step of the spec, before any strip or glass is involved) and fails with them reverted too (0 of 4 pass), so it is also not from this change.
- The spec `🥞️layered-home` read the strip's resting position with a `translate3d` regexp (`panned()`); it now accepts `translate` and `translate3d` (the resting strip is the 2D translation). That check sits after the failing assertion of the test, so it could not be exercised here.
- Forced colours, reduced motion and dark were compared by screenshot (above); no e2e project of the gate covers them.
