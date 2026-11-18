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
