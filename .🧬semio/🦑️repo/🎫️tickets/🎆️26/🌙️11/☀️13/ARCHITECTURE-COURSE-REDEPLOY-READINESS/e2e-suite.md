# Architecture course e2e suite, run natively

Run on macOS arm64 (machine load 50 to 80 from other agents, which made every journey several times slower than on a quiet host). `repo run` is unusable here, so the suite is run by `native-stack.ts` in this folder.

## How to rerun

```
RUN_DIRECTORY=<dir> $HOME/.local/share/semio/node/v24.15.0/bin/node native-stack.ts <dev|rehearsal> <shared|exclusive|all|PROJECT,...> [playwright args]
```

- It builds nothing for the proctor: it starts the prototype `proctor` binary (build it first with `repo cargo --store=shared --configuration=prototype -- build --manifest-path 🎓️teaching/Cargo.toml -p teaching-proctor --bin proctor`) on 8791 with a temporary data folder, the control endpoint the away and faults specs use, the PDF runtime channel, and the site on 6061 (dev: vite dev server proxying the proctor; rehearsal: `vite build --mode release` served statically, proctor in production mode with the site origin allowed), runs the `boot` spec, then `playwright test`.
- Environment: `PDF_DIRECTORY`, `WORKERS` (shared shard, default 3), `SKIP_WARMUP=1`, `KEEP=1` (boot and keep the stack, prints the control URL, until SIGTERM), `PROCTOR_BINARY`.
- The exclusive shard must run on one worker and never beside other specs: its specs stop and start the shared proctor. Pass `--no-deps` to run `shortage,away,faults` when `presence` is red (otherwise Playwright skips them).

## Results (dev topology)

| Shard | Result |
|---|---|
| shared (74 specs, 3 workers) | 53 passed, 21 failed in the first full run |
| exclusive, `shortage` + `away` + `faults` | 6 of 6 passed |
| exclusive, `presence` | 5 of 6 passed (card scroll open) |

After fixes, reruns passed: all 14 `quiz-runs`, `phone` hard run, `both-languages` en and de, four of five `layered-home`, `presence` mobile swipe, ladder (perch above).

## Results (release rehearsal)

`challenge-levels` 13 of 13, `first-visit` 4 of 4, `live-leaderboard` 1 of 1, `shortage` 2 of 2, `away` 2 of 2, `faults` 2 of 2 (the latter three run serially on the kept stack). One apparent set of failures of the outage specs was my own invocation (they ran in parallel with other specs against one proctor and killed it under each other), not a product defect. Not run in rehearsal: `phone`, `layout`, `pets`, `presence`, `quiz-runs`, `layered-home`, `both-languages`. The grapple ladder spec failed 3 of 3 there (pets, see below).

## Fixes made

Stale specs fixed:
- `🧪️tests/🎯️quiz-runs/🟦️.ts`: Zipf pools draw fewer items than slots, so assert at most one variant per slot and every shown item belongs to a slot; the 4-item `final-energy` swap marks two items, so assert right >= wrong.
- `🧪️tests/🥞️layered-home/🟦️.ts`: grid measurement re-measures frame and card together (the footer resizes the grid as learners come and go) and allows the 24 px tab that reaches into the cell above; the pan spec moves the pointer back to the top-left before hovering a card that panned out of view.
- `🧪️tests/📱️phone/🟦️.ts`: cards now stretch to fill their page (`items-stretch`, as the component test and `home-text-and-viewport` pin), not centre.
- `🧪️tests/👥️shared-presence/🟦️.ts`: seven cards, so a swipe right from the learner wraps to `assignments` and a swipe down to `prefs`; card scroll reads the scrolling box inside the page (helper `scrolled`).
- `🧪️tests/🐕️pet-walk/🧰️pet-kit/🟦️.ts`: `expectCast` names the scene in its failure message.

Product: `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/👀️follow/🟦️.tsx`: `scroller` now returns the scrolling box inside the swipe page, because a card that fills its page scrolls itself and the page cell never scrolls, so followed scroll was not shared. Not verified: the card-scroll spec is still red (the follower stays at 0), so this change is incomplete or insufficient. It is owned by another agent now.

## Open failures (all owned by other agents)

- Pets ground regression (7 specs: topic casts en/de, cast follows the learner, "eight deeds" 320/375 en/de, phone small pets, grapple ladder): pages whose single card fills the whole viewport (single-quiz topics `parametric`, `grasshopper`, `excel`, `bim`, `optimization`, the `assignments` and `clips` pages, and the phone pages) leave a 24 px strip under the card body, so no pet has ground. Confirmed by probing: no pets in 24 s on those pages, pets present on `home`, `energy`, `physics`, `learner`, `badges`.
- `adaptive-layout` phone: `div.course-page-frame` scrolls sideways at 375 px on the page `board`.
- `layered-home` pan/zoom reveal: the glass does not clear on a press (`data-veil` stays `whole`); before my spec change, a locked topic card's tint intercepted the pointer over the energy zoom button once the strip had panned.
- `presence` card scroll (follow).
- `phone` home spec: expectation updated to `stretch`, but it was not rerun green (it failed again in the rerun; cause not examined).

## Flakes

- `both-languages` en/de: failed with the 10 minute limit under load 60 to 80 (shared shard), passed on rerun (6 min each, alone). Load flake, 2 of 2 failed once, 2 of 2 passed once.
- No spec was rerun three times except the grapple ladder (3 of 3 failed, not a flake: the pets regression). Flake rates for the rest are therefore not measured.

## Launcher notes

A first rehearsal build failed with `Cannot set property code of  which has only a getter` (a masked `AbortError`): the launcher ran `vite build` synchronously, which froze the process serving the PDFs. It now runs the build asynchronously.

## E2E Regressions (2026-11-18 evening)

Dev topology on ports 8801 and 6070 (`e2efix-stack.ts`, a copy of `native-stack.ts`; `e2efix-pw.sh` runs Playwright against a kept stack), site watcher off, stack restarted after every edit. Waves as the session table defines them: shared (3 workers), exclusive (1 worker), then `chaos` and `reconnect` (`--no-deps`, 10 browsers and 90 protocol learners). Host load 14 to 37.

### The two reported failures: both stale or racing specs, no product regression

- `layered-home`, "the card sits inside its grid cell": the card grid interlocks consecutive rows (`CardStack/bands.ts`, today 08:16): a lower card is lifted onto the bottom line of the card above by one band (24 px) plus two path insets (2 px), pinned by the `CardStack` and `bands` tests. Measured: 26 px (row 1 top 240 of a cell starting at 266, row 2 top 506 of 532); the spec allowed 24 px plus 1 px of tolerance. Spec fixed: the reach is `24 + 2` (`TAB_PX`).
- `adaptive-layout` (German walkthrough intercepting the learner card): the spec opened the site with `page.goto("/")` and `readIntroduction`, which counts the walkthrough, finds none (it is a lazy chunk that mounts after the overview, which is already clickable) and moves on; the walkthrough then appears over the learner card. Every other spec uses `arrive`, which waits for it. Spec fixed to `arrive({ page })` (`arrive` now takes `Pick<Device, "page">`). Not a product bug: the walkthrough shows exactly where it should (first visit) and dismisses. It failed 2 of 4 before the fix (a race), 4 of 4 and 12 of 12 after.

### Found by the whole run

- Product bug: `observerSession` (`👀️follow`) turned every function of the session, including `revived`, into an async no-op, so the leaderboard poll got a promise as its unsubscribe and threw `back is not a function` when the observed home unmounted, crashing `HomeScreen` of every follower. It broke four `presence` specs (reconnect/follow, footer and cursors, mobile swipe, card scroll). Test first (`👀️follow` unit test, red then green), then `revived` returns the idle subscription like `subscribe`. Presence then 6 of 6.
- Spec race: the layered-home pan test hovers the zoom control, waits for the glass, then presses; the footer (learners online) can move the grid in between, so the press missed the control and the glass stayed whole (failed 2 of 5 runs in loaded waves, never alone). The press is now hover, press, expect clear inside `toPass`; 30 of 30 with 5 workers and three repeats.
- Launcher gap: the control endpoint of `native-stack.ts` knew `stop` and `start` only; the reconnect and chaos specs freeze the proctor through `pause` and `resume` (`SIGSTOP`, `SIGCONT`) as the gate does. Added to `native-stack.ts` and `e2efix-stack.ts`. Without it the freeze cycles of `reconnect` failed; with it `reconnect` passed 1 of 1 on dev (17 cycles, 9.9 min).

### Results

| Project | Result |
| --- | --- |
| boot | 1 of 1 |
| desktop (first wave) | 43 of 44; the one red was the pan race above, then 6 of 6 layered-home three times over |
| phone | 2 of 2 |
| layout | 4 of 4 (first wave), 12 of 12 repeated |
| pets | 24 of 24 |
| presence | 2 of 6 before the follow fix, 6 of 6 after |
| shortage, away, faults | 2 of 2 each after the follow fix |
| reconnect | dev 1 of 1 with pause and resume; an earlier run behind `chaos` in one stack failed in every cycle after chaos (presence never returned), not reproduced alone |
| chaos | red, not attributed |

### Chaos (open, not a regression I could pin)

10 browsers and 90 protocol learners, three runs, each after the proctor truth (every run, total and submission) checked green. Dev: the final "presence converges on everyone online" saw 94 and 93 of 100. Release rehearsal passed that and failed the next step ("silent sockets leave within the idle timeout"): the watcher's roster socket closed and reopened every 25 to 80 s and showed 1 online instead of 10. The host had under 100 MB free memory in all runs (peaks 7 to 8 GB of browsers); `mixed-hundred.md` found the same memory-bound behaviour at this size earlier today. No change of today touches the presence socket or room (`🔌️socket` is committed from 13:37). To be rerun on a quiet host with a proctor binary built from the current source (the one used here is from 2026-11-17 16:26).

## Fresh Proctor Reruns (2026-11-18 night)

Ports 8802 and 6071 only, one stack at a time, host load 10 to 57 (memory free percentage 34 to 70 during the runs; free pages stay low by macOS design).

### Hypothesis: confirmed in part

The earlier chaos runs used the proctor built 2026-11-17 16:26, which sends no heartbeat frames, while the client watchdog (25 s) had already shipped. Confirmed by a websocket probe of both fresh binaries (prototype and release, built 22:17 and 22:20): `welcome` at 0 s, `heartbeat` at 10 s and 20 s; also through the dev server's websocket proxy on 6071 (heartbeats at 10.1 s and 20.1 s while batches flow), so the upgrade path does not swallow them (the client has no long-poll path, only the websocket). With the fresh binary the static rehearsal now passes the two steps that failed before ("presence converges on everyone online" and "silent sockets leave within the idle timeout").

### Build

`repo cargo --store=shared` first failed on both configurations with `Module not found .../sealed/<digest>/...unit-cache/buildscript/<ts>` from eight cached build scripts per configuration (`libc`, `quote`, `getrandom`, `generic-array`, `zmij`, `httparse`, `serde_json`, `zerocopy`): their wrapper `build_script_build` baked the path of a sealed source snapshot that the build had since pruned (digests `2077cbe5533b7c34` prototype, `d13de40fa8f15ba3` release). Not a deadline refusal (that did not recur: both builds ended `exit 0`). Fixed locally by removing those 16 stale build directories under `.cache/b/r/m/u/l/p001ec5/{prototype,release}/build/<crate>/<hash>`; cargo recompiled them. Open product bug for the owner of the unit cache: a cached build-script wrapper must not outlive the sealed snapshot it names (invalidate the entry when the snapshot is gone, or do not bake the path in).

### Harness root cause: stale launchers

Every launcher pinned `.../prototype/build/teaching-proctor/762af25000b726d1/out/proctor`, a content hash that never changes while the binary behind it is the one that was built on 2026-11-17: a new build lands under a new hash, so the launchers silently kept the old one. Fixed with `fresh-proctor.mjs` (laws in `fresh-proctor.test.mjs`, 3 tests, `node --test`):

- `freshProctor(configuration)` builds through `repo cargo` (incremental, cheap when nothing changed), picks the newest `out/proctor` of that configuration, and throws when it is older than the newest Rust/TOML/lock source of the proctor's crates (`PROCTOR_BUILD=off` skips the build, never the check).
- `native-stack.ts`, `e2efix-stack.ts` (`PROCTOR_CONFIGURATION`, default prototype), `chaos-stack.mjs` (release when `STATIC=1`, else prototype) and `reconnect-stack.mjs` (release) use it unless `PROCTOR_BINARY` / `PROCTOR_EXECUTABLE` names one explicitly.
- `chaos-build.mjs` and `reconnect-build.mjs` rebuild the static site unless the build is newer than every site source and was built for the same proctor origin (`.built-for` marker; `FORCE_BUILD=1` forces).
- `reconnect-stack.mjs`, `reconnect-build.mjs`, `reconnect-run.sh`, `reconnect-repeat.sh` take `PROCTOR_PORT`, `SITE_PORT` and `RECONNECT_WORK`.
- The e2e gate (`🎭️e2e/🟦️.ts`) was not changed: it resolves the receipt product through `acquireProctor`, which already refuses a stale product.

### Results (fresh proctor, 2026-11-18 22:17 prototype and 22:20 release)

| Run | Topology | Result |
| --- | --- | --- |
| reconnect seed 11 | static release, proctor release | passed (6.5 min), 11 cycles, 0 stuck, worst presence 12.0 s, worst calm 12.2 s, 20 of 20 silent sockets closed by the watchdog |
| reconnect seed 12 | same | passed (5.9 min), 11 cycles, 0 stuck, worst presence 13.4 s, worst calm 8.7 s, 20 of 20 |
| reconnect seed 13 | same | passed (6.3 min), 11 cycles, 0 stuck, worst presence 13.1 s, worst calm 14.2 s, 20 of 20 |
| chaos (10 browsers, 90 protocol learners, 10 quizzes) | static release rehearsal | passed (13.0 min), 1000 runs, presence 100, 3 proctor kills, ghosts gone after 66 s |
| chaos, run 1 | dev (vite on 6071, prototype proctor) | failed (22.4 min): proctor truth, leaderboard pages and calm indicators green; "presence converges on everyone online" saw 98 of 100 on `Chaos 2` for the whole 90 s poll. Two recoveries of that run were starved for over 120 s (`locator.click` timeouts of `Chaos 2` and `Chaos 4`), browsers peaked at 6.2 GB, free memory 41 MB at the lowest |
| chaos, run 2 | dev | passed (19.2 min), 1000 runs, presence 100, ghosts gone after 79 s |
| chaos, run 3 | dev | passed (26.1 min), 1000 runs, presence 100 |

So: `reconnect` 3 of 3, `chaos` static 1 of 1, `chaos` dev 2 of 3. The old "reopens every 25 to 80 s showing 1 online" symptom did not occur again, which is consistent with the heartbeat-less binary being the cause of the release failure. The 93 to 94 of 100 on dev (old binary) is not reproduced as such, but a 98 of 100 on dev once is open: the cause was not established (the run had no per-learner evidence).

### Step 4: not changed in the product

Checked: the server heartbeat is its own `select!` arm with the default burst missed-tick behaviour, independent of batch flushing (the proxied probe above shows it on schedule between dozens of batches); heartbeats reach a socket behind the dev proxy. Not checked beyond argument: a watcher whose page is blocked for more than 25 s may see its watchdog timer run before the queued frames are delivered and close a healthy socket; that is a hypothesis that fits run 1 (pages starved for minutes) and the earlier "reopens every 25 to 80 s", but no run proved it, so no product change was made. If the dev failure recurs, the next step is a stall-aware watchdog (when the timer fires later than `silenceMs` plus a margin, the page was not running during the silence; re-arm once instead of closing), test first in `🚪️presence-room` with `vi.setSystemTime`.

Diagnostic added for the next occurrence: `rosterAudit` in `🌪️chaos-crowd/🤖️protocol-learner/🟦️.ts` joins the roster with a silent observer, collects welcome and batches for 4 s and prints how many sessions and learners the proctor itself holds and which expected learners (browsers by tag, bots by index) are missing; `expectOnline` in `🌪️chaos-crowd/🟦️.ts` appends it to its failure message. Runs 2 and 3 above ran with it (it only speaks on failure).

### Files changed

- Ticket scripts: `fresh-proctor.mjs` and `fresh-proctor.test.mjs` (new), `native-stack.ts`, `e2efix-stack.ts`, `chaos-stack.mjs`, `chaos-build.mjs`, `reconnect-stack.mjs`, `reconnect-build.mjs`, `reconnect-run.sh`, `reconnect-repeat.sh`.
- Spec: `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🌪️chaos-crowd/🟦️.ts` and `🤖️protocol-learner/🟦️.ts` (the roster audit).
- Outside the repository tree tracked by git: 16 stale cargo build-script directories removed from the shared cache lane `p001ec5`.

### Rerun

`PROCTOR_PORT=8802 SITE_PORT=6071 STATIC=1 CHAOS_WORK=<dir> node chaos-stack.mjs` (builds the proctor itself), `CHAOS_WORK=<dir> PROCTOR_URL=http://127.0.0.1:8802 node chaos-build.mjs` (only rebuilds when stale), then `CHAOS_BROWSERS=10 CHAOS_PROTOCOL=90 CHAOS_QUIZZES=10 CHAOS_WORK=<dir> ./chaos-run.sh`; reconnect the same with `RECONNECT_WORK`, `reconnect-stack.mjs`, `RECONNECT_SEEDS="11 12 13" ./reconnect-repeat.sh`.
