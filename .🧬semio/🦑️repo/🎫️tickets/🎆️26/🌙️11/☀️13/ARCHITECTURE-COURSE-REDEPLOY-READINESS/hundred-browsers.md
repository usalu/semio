# A hundred real browser learners: what this 16 GB Mac sustains

Result: 100 simultaneous real browsers did not fit. The largest crowd measured was 49 connected browsers (presence count 52 while arrivals were still booting), 10.2 GB of browser memory, 55 renderers. Beyond about 45 to 50 live pages on this shared machine, new pages stopped answering (a click or a check hung for 150 s) and the run failed. Fully passing runs exist only at 10 and 12 browsers.

## What was built (mode of the chaos-crowd spec, no new spec)
`🎓️teaching/🏛️architecture/📚️course/🧪️tests/🌪️chaos-crowd/🟦️.ts`, env knobs:
- `CHAOS_PROTOCOL=0 CHAOS_BROWSERS=100 CHAOS_QUIZZES=1`: only real browsers, one quiz each, rotated by index so every quiz gets about the same number of players.
- `CHAOS_CONTEXTS_PER_BROWSER=N`: Chromium processes of N contexts each (headless shell); default keeps the fixture browser.
- `CHAOS_ARRIVAL_S`: arrival spread; `CHAOS_VIEWPORT=WxH`; `CHAOS_SAMPLE_S`.
- `CHAOS_CALM=1`: reduced motion plus the app's own preferences (pets off, icon animations off, shared cursors off) written to localStorage before boot; no code change in the app.
- `CHAOS_BOOT_SLOTS`, `CHAOS_MAX_BROWSERS_MB`, `CHAOS_ADMIT_PATIENCE_S`: admission control (concurrent cold boots, memory ceiling, learners not admitted are skipped and the assertions use the attending count).
- `CHAOS_RETRIES`, `CHAOS_INTENSITY`, `CHAOS_KILLS`, `CHAOS_GHOSTS` as before.
- New measurements in the report: peak connected browsers, presence seen, browser and proctor RSS counted from the runner's own process tree, renderer count, arrive / quiz-start / submit latencies (p50, p95, max; a "quiet" submit series excludes offline and slowed windows).
- Post-phase checks (settle, leaderboard pages, connection tone) now run in parallel.
- Launcher scripts take `CHAOS_WORK` for the output folder (`chaos-stack.mjs`, `chaos-build.mjs`, `chaos-run.sh`).

## Measurements
- Memory per real learner: about 113 MB on the introduction screen (JS heap 73 MB), 170 to 270 MB while playing, including a share of the browser, GPU and network processes. 100 learners therefore need 11 to 27 GB; the Mac has 16 GB and other agents' browsers share it (swap reached 8 to 10 GB, once exhausted).
- Quiet machine, 10 browsers, no chaos, one quiz each: passed in 4.7 min (about 90 s of play), 1.7 GB.
- 12 browsers, chaos intensity 1, proctor kill, ghosts: passed, but took 24 min at load average 140 (latencies meaningless: arrive p50 27 s).
- 100, 60, 30 and 20 browser attempts failed on stalls (page hung, selectOption or click timeouts), not on a wrong score; every failure happened while 30 to 50 pages were live and the machine load was often 80 to 290 from other tenants. No page error, duplicate submission or wrong total was ever reported by the assertions that ran.
- Presence: counts followed connections (52 seen at 49 connected); ghost sockets: closed and reset gone in 2.3 s, silent gone in 63.8 s (idle 60 s).

## Bugs and fragilities found and fixed (harness, tests first not applicable: they are the test)
- Introduction close click swallowed while the page boots under load: `readIntroduction` (shared learner kit) now repeats the click until the walkthrough is gone.
- Identity step: radio and name fill repeated until they stick.
- A reload or reopened tab left a step waiting on a page that no longer shows it for its full 120 s timeouts and livelocked slow learners: steps are now abandoned as soon as the page was taken away.
- A truth lookup during the proctor kill window threw an unguarded fetch error: guarded.
- Possible app issue, not fixed: under heavy boot load the introduction Skip click and the identity radio were occasionally lost or unresponsive; not reproduced on a quiet machine.

## To reach 100
Run on a machine with 24 GB or more (or alone on this one), `CHAOS_BOOT_SLOTS=6`, `CHAOS_CONTEXTS_PER_BROWSER=10`, `CHAOS_CALM=1`, `CHAOS_ARRIVAL_S=120`, `CHAOS_INTENSITY=0.3`, `CHAOS_RETRIES=20`. Chromium cannot share renderers between contexts, and single-process mode does not start under Playwright, so memory per learner cannot be cut further.
