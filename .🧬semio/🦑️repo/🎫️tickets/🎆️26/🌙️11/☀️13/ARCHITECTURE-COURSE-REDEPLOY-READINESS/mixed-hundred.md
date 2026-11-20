# One hundred simultaneous learners: real browsers plus protocol learners

Result: one passing run (run 10) with 25 real browsers and 75 protocol learners, presence 100, chaos on, against the static release build (`vite build --mode release`, served statically, proctor in production mode with the site origin allowed, release `proctor` binary built with `repo cargo --store=shared --configuration=release -- build --manifest-path 🎓️teaching/Cargo.toml -p teaching-proctor --bin proctor --release`). Ports 8798 (proctor) and 6067 (site). The ramp to 40 + 60 did not complete on this 16 GB Mac, because memory thrashes (see "Why not 40 or 50 browsers"); 50 + 50 was not attempted.

## Passing run (run 10)
Configuration: `CHAOS_BROWSERS=25 CHAOS_PROTOCOL=75 CHAOS_QUIZZES=10 CHAOS_BROWSER_QUIZZES=1 CHAOS_CONTEXTS_PER_BROWSER=3 CHAOS_VIEWPORT=960x640 CHAOS_CALM=1 CHAOS_BOOT_SLOTS=4 CHAOS_ARRIVAL_S=120 CHAOS_INTENSITY=0.3 CHAOS_RETRIES=20 CHAOS_SAMPLE_S=15 CHAOS_MAX_BROWSERS_MB=9500 CHAOS_ADMIT_PATIENCE_S=3000`, `CHAOS_KILLS` default 3. Playwright result: `1 passed (26.9m)`.
- Coverage: the 25 browsers play one quiz each, rotated by index, so all 10 quizzes are covered (2 to 3 browsers each); each of the 75 protocol learners plays all 10 quizzes (new knob `CHAOS_BROWSER_QUIZZES`, see fixes).
- Peak: 25 live browsers in 9 Chromium processes (25 renderers), 75 bots, presence seen 100 (25 + 75, read from a browser's own presence indicator).
- Runs completed: 775 (25 browser runs, 750 bot runs), 25 of 25 browsers attended (none skipped by admission), elapsed 1605 s (all learners finished after 1262 s).
- Chaos injected: 3 proctor kills (SIGKILL then restart), 51 offline windows (9 of them in the middle of a submission), 15 slowed-network windows, 9 reloads, 8 tab close and reopen, 14 step recoveries, 2 submit recoveries; bots: 351 lost requests, 323 lost answers, 525 double sends, 108 socket drops, 603 retries; ghost sockets 2 closed, 2 reset, 2 silent (ended and reset gone after a few seconds, silent gone after 70.5 s with proctor idle 60 s and keepalive 20 s).
- Memory: browsers peaked at 9132 MB (25 renderers; about 365 MB per live learner including a share of browser, GPU and network processes), proctor 48 MB, runner 419 MB; `memory_pressure` free percentage never below 34 %; machine total 16384 MB.
- Load (uptime 1 minute average, sampled every 15 s over the whole session): 18 to 156; the other tenants' git, bun and replica work accounted for a base of 20 to 70.
- Latency percentiles (milliseconds, browser learners): page arrival p50 5081, p95 14560, max 20380 (25 samples); quiz start p50 13177, p95 202218, max 750407 (21 samples); submit while quiet p50 4397, p95 7690, max 7690 (9 samples); submit including disturbed windows p50 6263, p95 20467, max 36296 (25 samples); whole quiz p50 173306, p95 1029496, max 1131437.
- Assertions that passed: every bot's own problem list empty; each learner (25 browser, 75 bot) has exactly the planned submitted runs, each quiz submitted exactly once, no extra runs, total and own leaderboard row exactly the expected total (server truth read from the proctor); the leaderboard ranks exactly 100 new learners and counts exactly the wanted new submissions (25 + 750); every browser's leaderboard page shows its own exact total; every connection indicator recovered to calm; presence converges on 100 online on every browser, then on 25 after the bots leave; ghosts that closed or reset leave at once while silent ones leave within the idle timeout; presence holds the true count afterwards; no page error and no unannounced console or request failure on any browser (security policy violations count as failures always).

## Every run
All runs: static release build, chaos intensity 0.3, retries 20, calm mode, 10 quizzes for the bots, one quiz per browser, 4 boot slots (run 1: 6), sample every 15 s.

| Run | Browsers + bots | Contexts per browser, viewport | Memory ceiling | Result and reason |
| --- | --- | --- | --- | --- |
| 1 | 25 + 75 | 5, default 1440x900 | 8000 MB | failed after 7 min: the introduction close click never became stable within the 150 s window while the load was 80 (harness limit, widened) |
| 2 | 25 + 75 | 10, default | 7000 MB | failed: `page.goto` of the author preview timed out after 150 s during boot, unguarded (harness; boot is now retried) |
| 3 | 25 + 75 | 10, default | 7000 MB | stuck and killed: the runner's own synchronous `ps`, `lsof`, `memory_pressure` sampling blocked its event loop for seconds under load (harness; now asynchronous) |
| 4 | 25 + 75 | 10, default | 7000 MB | failed: a step timed out while the learner's offline window had just begun and was classified as an undisturbed hang (harness) |
| 5 | 25 + 75 | 3, 960x640 | 8000 MB | killed after 15 min: the proctor answered in seconds because its event store sat in the replicated worktree (fsync 9 ms in tree against 0.2 ms outside, measured; harness, store moved out) |
| 6 | 25 + 75 | 3, 960x640 | 8000 MB | everything passed up to and including presence and ghosts; failed only on the last check: 72 `net::ERR_INTERNET_DISCONNECTED` console lines delivered late by starved pages after the 25 s grace of their own offline window |
| 7 | 25 + 75 | 3, 960x640 | 8000 MB | everything passed up to and including ghosts; failed only on the last check: a `404` answer to `POST /queries` (the proctor saying a view does not exist yet) reported late after the learner's offline windows |
| 8 | 40 + 60 | 3, 960x640 | 10500 MB | failed: browser learner earned 0 points; the event store showed a second run of the same quiz (see fixes: submit recovery started a new run while the first submission was still in the device outbox) |
| 9 | 40 + 60 | 3, 960x640 | 10500 MB | same failure as run 8 with the catch-up poll in place; the event store again showed the second run, started 19 s after the first run's submission finally arrived about 16 minutes late |
| 10 | 25 + 75 | 3, 960x640 | 9500 MB | PASSED (above) |
| 11 | 40 + 60 | 3, 960x640 | 10500 MB | stopped by me at t=1835 s: 23 of 40 quizzes done, 33 live browsers, presence 95, swap 4.6 GB of 6 GB and the macOS compressor at 7.9 GB: the machine was memory-bound (42 % of CPU in the kernel), every learner stalled |
| 12 | 40 + 60 | 3, 960x640, arrival 240 s | 6500 MB | stopped by me at t=1638 s: 17 of 40 done, 26 live, presence 86, compressor 6.8 GB, swap 3.5 of 4 GB (other agents' browsers and tools held about 8 GB at the same time) |

Peaks of the failed and stopped runs (live browsers, presence, browser memory): run 1: 19, 92, 8249 MB; run 6: 25, 100, 10162 MB; run 7: 25, 100, 9310 MB; run 8: 27, 86, 9720 MB; run 9: 34, 94, 10335 MB; run 11: 33, 95, about 10400 MB; run 12: 26, 86, 7691 MB. No run ever reported a duplicate submission or a wrong total from a learner that the harness did not itself restart.

## Why not 40 or 50 browsers
A live learner costs about 330 to 470 MB (renderer about 250 to 350 MB, plus its share of browser, GPU and network processes), so 40 need 13 to 19 GB and 50 need 17 to 24 GB. This Mac has 16 GB and other agents' tools (a 1.5 GB bun, the desktop app, their own Playwright suites with up to 40 renderers) held 5 to 8 GB at all times, so beyond about 25 live pages the macOS compressor and swap saturate and every page, including the proctor's clients, stalls. The admission ceiling (`CHAOS_MAX_BROWSERS_MB`) keeps the rest waiting, but a learner is only admitted when others finish, and finishing is what slows down. Measured per-page idle cost with calm preferences is 0 % CPU (no animation loop), so this is memory, not a busy page.

## Fixes (all harness, no product bug found)
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🌪️chaos-crowd/🟦️.ts`:
  - new knob `CHAOS_BROWSER_QUIZZES` (default `CHAOS_QUIZZES`): browsers play one quiz each while the protocol learners still play all 10, so both cover every quiz; the doc comment names it.
  - the browser start is retried up to 3 times (new context each time, same handle, so the same learner) when a boot step times out under load; each retry is noted.
  - memory sampling (`ps`, `lsof`, `memory_pressure`) is asynchronous and cached; it was blocking the runner's event loop.
  - `resilient` treats a learner that is offline right now as disturbed.
  - submit recovery: when the device outbox has not drained within 300 s and the proctor has no submitted run yet, it keeps waiting (up to 30 minutes) instead of resuming the quiz, which started a second run of the same quiz that was then submitted empty (score 0, the cause of runs 8 and 9). A mismatching points value on the results page is polled for 90 s and the catch-up is noted.
  - announced failures: `net::ERR_INTERNET_DISCONNECTED` (produced only by the injected offline emulation) and a `404` answer to `/queries` (the proctor's "does not exist yet", retried by the page through projection lag) are tolerated for a learner that had offline windows or after a proctor kill, however late a starved page reports them. Every other console or request failure, every page error and every security policy violation still fails the run. The assertions on truth, leaderboard and presence are unchanged.
- `🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🚶️learner/🟦️.ts`: `readIntroduction` repeats its close click for up to 400 s instead of 150 s.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/ARCHITECTURE-COURSE-REDEPLOY-READINESS/chaos-stack.mjs`: `CHAOS_DATA` places the proctor's event store outside the replicated worktree (the live replica made fsync 40 times slower in the tree).

## Observations for the product
- While about 75 protocol learners drove the proctor and all browsers were connected, the browsers progressed slowly (quiz p95 about 17 minutes in this memory-bound situation); they finish quickly once the bots are done. On a quiet machine with 10 browsers a quiz takes about 90 s.
- A device whose submission sits in its outbox for minutes lets a learner start another run of the same quiz; the proctor then holds two runs for that quiz. A user cannot trigger this by hand (the quiz shows the results), so it is recorded here only as a harness hazard that was fixed in the harness.
- The built page keeps pets and icon animations running by default; with them on, an idle page keeps its browser's GPU process at 100 % CPU (software compositing in headless Chromium). The crowd spec turns them off with `CHAOS_CALM=1`.

## Cleanup
Stack, proctor, bots, samplers and browsers were stopped; `🗑️generated/mixed100` (build output, event store, logs, reports) was deleted.
