# W3-LATENCY-HONEST (static, nothing compiled or run)

Every `[latency]` line is now `[latency] <name>/<scenario> p50_ms= p95_ms= budget_ms=`. Scenarios: `cold` (first evaluation, one sample, own budget 1000 ms; the debug build gets 20x), `drag` (input differs every step, 12 ms), `commit` (a different real edit per round, 400 ms), `paint`/`repaint` (cache hit, 5/12 ms). Declared budgets are unchanged (drag 12, commit 400). Root cause of the finding: warm-up with an identical input followed by identical rounds, so the fibre section memo (and pure-kernel calls with identical arguments) answered every timed round.

Paths relative to `✏️s/🔌️plugins/📕️norm/`.

## Audit and result per test
| test | before | now |
| --- | --- | --- |
| mechanics `tests/⏱️latency` slope grid, Mononobe-Okabe | identical input each round, 1 warm-up | `cold` (round 0), 3 discarded warm-ups, then 30 rounds with a different slope height / kh offset: `<name>/cold`, `<name>/drag` |
| mechanics fibre `characteristics`, `curve` (the "pushover" fibre gates of the review) | one section built once, warm-up filled its `SectionCache`, all timed rounds were cache hits | a fresh `rc_section(round)` per round (bar area and concrete strength differ, empty cache): `fibre-*/cold`, `fibre-*/commit` (400). Section construction is inside the timed region (it is what an edit repeats) |
| compliance `one-entity-commit` | commit of a changed load, true; cold open was untimed | added `one-entity-commit/cold` (open + full plan on 1000 members), `/commit` line renamed; `settled-paint/paint` line added |
| en1993 `frame-envelope` | timed the uncached `combination_results` and a cached repeat | `frame-envelope/cold` (first cached call on the fresh thread-local), `/commit` (edited frame through `combination_results_cached`, memo miss each round, plus `envelope`), `/repaint` (hit, 12 ms, must beat the solve) |
| 10 editors, `scene/tests/live` drag test | drag step 0 reused the stored placement x; no print; no commit | `<code>/scene-cold` (first projection), `/scene-drag` (x = step+1, 30 steps), `/scene-commit` (12 random real edits from the existing `edit` generator, each followed by the full checks and the incremental projection, 400); shared local helper `gate_scenario` per file |
| structural-scene live-scene 1000-member test | drag only, no print | `live-scene-1000-members/cold` and `/drag` lines |
| en1992 editor remedy search / member drag | same document each commit round, no print | `en1992-member-checks/cold`, `/commit` (member count differs per round), `/drag`; still asserted in release only, as before |
| app-surface `latency-bench` (all 10 editors) | 17 repeating drag values and commit values equal to drag values (both hit the retained caches after the first lap); warm-up consumed the cold cost | drag and commit values are distinct per tick/round and disjoint from each other; new `cold-evaluation` (first `live_report` + first scene) and `cold-drag` (first tick), budget `COLD_MS` = 1000 ms, asserted in release via `failures()`, written into the JSON report; lines `<variant>/cold-drag`, `/cold-evaluation`, `/drag`, `/commit` (was `drag-preview`, `commit-total`) |
| scene-memo `memo_diagrams` | untested | new test in live-scene tests: `diagram-memo/cold`, `/drag` (document differs per step, miss + store), `/repaint` (hit, `compute` must not run) |
| overlay preview cache (`selection-mirror`) | untested directly | covered by the app-surface `drag` scenario, now with a different write set every tick |
| unit tests "projecting 1000 walls/floors/members stays interactive" (en1990, 1991, 1993, 1995, 1996, 1997, 1999) | cold projection, no print | now print `<code>/scene-cold-1000` with the asserted bound as budget (4000 / 1000 / 2000 / 2000 / 20000 ms); asserts unchanged |

## Runner
`🗑️generated/build-runner/latency-kernels.py`: EXPECTED list now holds the scenario names (unlisted ones are still appended). Regex unchanged (`\S+` name).

## Caveats
- Nothing compiled. Risk spots: `scenarios`/`gate` helpers in mechanics (closure `time` borrowed mutably twice sequentially), `limit as f64` cast of an inferred integer in en1999, `memo_diagrams(.., Vec::new)` type inference for `Vec<Diagram2d>`, `f64::from(step)` on an inferred `i32` in the memo test.
- Cold figures are only cold for the first test in a process (nextest runs one process per test, as the runner does); with plain `cargo test` threads the pure kernels share nothing but CPU caches.
- The new cold budget (1000 ms) is new, not a raised one; if a cold figure exceeds it in release, that is a finding, not a reason to move the budget.
- Drag/commit values on handles with integer or clamped ranges can collapse to equal documents; the bench cannot detect that.
- Fibre `drag` (12 ms) is not asserted: it was never declared for fibre and the review said the kernel fits only the commit budget.
