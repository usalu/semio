# Work package Q — test budget headroom of the pets core suite

`P` = `🧰️framework/🛍️products/🐾️pets`, `TK` = this ticket folder, `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.
Package under work: `P/📦️packages/🟦️typescript` (`bun ./📜️script.ts test`, config `P/🧪️tests/🎚️config/🟦️.ts`).

## 1. Result

| | before | after |
| --- | --- | --- |
| `test` (fundamental), quiet host, wall | 6.2 s, 6.3 s (two runs; the brief measured 12.5 s) | 2.8 s – 3.8 s (section 5) |
| `test` (fundamental), sum of test time over the 8 files | 15.8 s | 2.5 s |
| `test` (fundamental), busy host (load mostly 60 – 100 %) | killed at 15 s (`[budget] … exceeded 15000ms`, exit 1) | exit 0 in 23 of 24 runs, 3.0 s – 16.1 s wall; the one kill came in a spike that starved everything |
| tests at fundamental / quick / long / exhaustive | 510 / 510 / 510 / 510 | 510 / 510 / 510 / 514 |
| `expect(` call sites in the 8 suites | 1098 | 1098 |

No assertion and no test was removed. Every test runs at every level; only how many samples a sweep takes, how many
seeds and how many seconds a statistical session runs, and the thresholds that are counts of those samples, depend on
the level. At `quick` and `long` every amount is exactly what it was before (for the stage suite: what its
`THOROUGH` branch was), so those two levels do the work the fundamental level did before, and more for the stage.

## 2. How the repo's levels work

* `TEST_LEVELS = ["fundamental", "quick", "long", "exhaustive"]` and the wall budgets `TEST_LEVEL_BUDGET_MS`
  = 15 s / 300 s / 900 s / 1800 s (`LIB/🟦️.ts:956-965`; `SEMIO_TEST_BUDGET_MS` overrides).
* `resolveTestLevel(segments)` (`LIB/🟦️.ts:1035`) takes the level from the first argument of the package script
  (`test quick`), else from `SEMIO_TEST_LEVEL`, else `fundamental`; it **writes `process.env.SEMIO_TEST_LEVEL`** and,
  at `exhaustive`, sets `SEMIO_COVERAGE=1`.
* `runVitest` (`LIB/🟦️.ts:2619`) spawns `vitest run --config <absolute config>` under `runTestBudgeted`
  (`LIB/🟦️.ts:1325`), which kills the process tree when the budget of the level expires. The level reaches the
  suites **only as the inherited environment variable**; there is no tag and no filter. The only level-dependent
  arguments are `--testTimeout/--hookTimeout/--teardownTimeout <budget>` (`vitestLevelArgs`, `LIB/🟦️.ts:1271`).
  With coverage on (exhaustive) the runtime is `node` with the v8 provider instead of `bun`.
* The library exports `testLevelAtLeast` and `atTestLevel` (`LIB/🟦️.ts:1051-1064`), but it is one module of 6 536
  lines (291 KB, 19 imports); a unit suite that imported it would pay more in transform time than it saves. Suites
  elsewhere read the variable inline (`🧰️framework/🔨️modules/⏪️time-travel/🧪️tests/🧪️conformance/🟦️.ts:386`, the os
  dev suites, the pets stage suite until now).
* The quiz product's unit suites and the site's tests do **not** read the level at all (no `SEMIO_TEST_LEVEL`,
  `testLevelAtLeast` or `atTestLevel` under `🧰️framework/🛍️products/❓️quiz` or `🎓️teaching`); only the pets stage
  suite did (`THOROUGH`).

## 3. The shared reader

`P/🧪️tests/🎚️config/🟦️.ts` (the existing, registered Vitest configuration of the core; no new directory, nothing to
register) now also exports

```ts
export const LEVEL: "fundamental" | "quick" | "long" | "exhaustive"   // SEMIO_TEST_LEVEL, fundamental when absent or unknown
export function sampled<Amount>(few: Amount, full: Amount, more: Amount): Amount   // fundamental | quick and long | exhaustive
```

Every unit suite imports `sampled` from `../../../../🧪️tests/🎚️config/🟦️.ts`. The stage suite's own `THOROUGH`
constant is gone. The import costs 1 ms per suite (Vitest's import breakdown).

## 4. What changed per suite

Amounts are `fundamental / quick and long / exhaustive`; the middle value is the former one.

**🏞️terrain** (3.71 s → 0.21 s)
* `perchesOf agrees with polygon-clipping`: layouts 30 / 300 / 3000; the floor `perches > layouts` (was `> 300`).
  Counted: 45 / 441 / 4251 perches.
* `strideTo arrives after ceil(distance ÷ step) ticks`: walks 6 / 200 / 600.
* `hopOf lands every granted hop`: mesh of the target grid ×4 / ×1 / ×0.5 (195 / 3120 / 12 376 targets); the floors
  `granted > 1500 ÷ mesh²` and `refused > 500 ÷ mesh²`. Counted: 156 and 39 (floors 93.75 and 31.25), 2547 and 573,
  10 139 and 2237 (floors 6000 and 2000).

**✅️validation** (3.45 s → 0.38 s)
* The four sweeps against ajv break `swept(document)` instead of `nodes(document)`: at fundamental about 24 nodes of a
  document, evenly spread from the root on (in the wrong-type sweep 46 – 62 mutated values of a species instead of
  482 – 575; the sample ensemble with its 17 nodes is still swept whole); every node at quick, long and exhaustive.
* At exhaustive the whole sample menagerie (783 nodes) is swept as a sixth document: four more test rows (514 tests).
  They pass.
* ajv is needed at every level (it is the third-party judge of all four sweeps and of the vectors), so it stays a
  top-level import: 96 ms to load, 146 ms to set up and compile the three definitions (measured in node).

**🎲️randomness** (0.62 s → 0.09 s)
* Draws of the two statistics 2000 / 20 000 / 100 000; their bounds are multiplied by `SLACK = √(20 000 ÷ draws)`
  (1 at quick and long), because the deviation of a mean goes with 1 ÷ √draws.
* Counters of the per-draw laws 200 / 2000 / 20 000, of the two 500-counter laws a quarter of that.

**📐️trigonometry** (0.62 s → 0.09 s): the dyadic angles are `k ÷ CUTS` with `CUTS` 64 / 1024 / 4096 (a power of
two, so every identity stays exact). The gl-matrix sweep over 65 537 angles has no assertion per angle and is unchanged.

**🦴️rig** (0.43 s → 0.08 s): `DRAWN` matrices 50 / 500 / 5000 against gl-matrix; laws a fifth, posed skeletons a
tenth per species, look offsets four times as many (200 / 2000 / 20 000).

**🎞️animation** (0.27 s → 0.15 s): steps of the d3-ease comparison 64 / 1024 / 4096. Everything else is committed
vectors, which run whole at every level.

**🧠️behavior** (0.28 s → 0.19 s): keyed draws 100 / 500 / 5000, idle-dwell steps 40 / 200 / 2000, monotony steps
16 / 64 / 512. The rest is committed vectors.

**🎪️stage** (6.39 s → 1.29 s; before this work package its fundamental form was already shorter than its quick form)

| test | fundamental | quick and long | exhaustive |
| --- | --- | --- | --- |
| newcomers a comfortable gap apart | 6 seeds | 20 | 200 |
| spreads newcomers out | 4 seeds | 12 | 120 |
| never inside a keep-out | 2 seeds × 10 s (was 3 × 15 s) | 10 × 30 s | 30 × 30 s |
| liveliness sessions | 120 s (was 300 s) | 600 s | 600 s |
| slightly active in calm | 1 seed; fidgets ≥ 2, walks ≥ 1 | 3; ≥ 6, ≥ 3 | 8; ≥ 16, ≥ 8 |
| activity graph | 180 s (the last scripted event is at 175 s) | 240 s | 240 s |
| blinking | 30 s (was 45 s) | 120 s | 600 s |
| chooses goals | 2 seeds × 120 s per mode; walks > 2 | 4 × 300 s; > 5 | 8 × 300 s; > 10 |
| outcomes of encounters (three tests) | 80; "mostly" > 44 | 200; > 110 | 2000; > 1100 |
| brings two actors together | 1 seed × 120 s per mode | 3 × 480 s | 8 × 480 s |
| waits out the gap | seed 5, three gaps of the mode (270 s calm, 90 s lively) | seed 5, 600 s | seeds 5, 1, 2, 3, 600 s |
| time of concentration | 60 s hushed, 30 s after (was 120 s, 120 s) | 300 s, 120 s | same |
| the tired fall asleep sooner | 40 s | 120 s | 120 s |
| comes back to life | 30 s | 120 s | 120 s |
| only as many arrive as fit; a perch shrinks | 2 seeds | 6 | 24 |
| never a goal beyond or inside a neighbour | 2 seeds × 60 s (was 2 × 120 s) | 6 × 300 s | 12 × 300 s |
| leaves a crowd | 1 seed × 60 s (was 2 × 240 s) | 4 × 240 s | 8 × 240 s |
| hopping between two shelves | seeds 1 – 4 × 60 s | seeds 1 – 3 × 300 s | seeds 1 – 8 × 300 s |
| hopping gait | 2 seeds × 120 s (was 2 × 300 s) | 4 × 300 s | 8 × 300 s |
| same frames for the same seed | 320 ticks, seed 42 (was 1600) | 3000 ticks, seed 42 | 3000 ticks, seeds 42, 142, 242 |
| however the ticks are cut | chunks up to 500, tail 500 | chunks up to 20 000, tail 3000 | chunks up to 50 000, tail 3000 |
| never changes the stage or the events | 12 rounds | 40 | 160 |
| highest rate, back to front | 600 ticks | 2000 | 10 000 |

The fundamental amounts were chosen from counted sessions, not guessed (`TK/wp_q_probe_stage.ts`, both companies,
eight seeds): for instance a calm session of seed 1 has 3 and 6 fidgets and 4 and 6 walks after 120 s; the second
approach of seed 5 begins at 252 s and 254 s in calm (hence three gaps = 270 s) and at 62 s and 66 s in lively; only
seed 4 hops between the shelves within a minute (hence seeds 1 – 4 there); a lonely walker begins 4 walks in
2 seeds × 2 modes × 120 s.

Changed assertion lines (all of them; each evaluates to the former value at quick and long):
`fidgets ≥ 2 × seeds`, `walks ≥ seeds`, `walks > sampled(2, 5, 10)`, three times `> MOSTLY` (11 in 20 of the
meetings, was 110), `met ≤ seeds × ceil(span ÷ gap)` (span was the literal 480 s) in the stage suite;
`perches > LAYOUTS`, `granted > 1500 ÷ MESH²`, `refused > 500 ÷ MESH²` in terrain; three bounds `× SLACK` in
randomness. The other differences are loop bounds. Verified against the original blobs of all eight files (found
among the loose objects of the repository): 1098 `expect(` call sites and 295 `it` / `it.each` registrations before
and after, file by file.

**Site test `🎓️teaching/🏛️architecture/❓️quiz/🧪️tests/🐾️pet-cast/🟦️.ts`**: left as it is. 80 tests, 0.31 s of test time, no
sweep and no session (each of the 20 species is judged once by ajv and once by the product). Its import takes 1.4 s:
0.5 s for the `@semio-tech/pets` barrel (the public entry a site has to use) and 0.65 s for the architecture
menagerie module with its 21 static JSON imports, which is the thing under test.

**README**: `P/README.md` names the three levels of the core's `test` command and the role of the config.

## 5. Measurements

The host was never idle: other agents ran end-to-end browsers and builds the whole time, and its load moved between
13 % and 100 % within seconds. Every figure is given with the processor load sampled just before the run
(`TK/wp_q_measure.sh`, `TK/wp_q_load.ps1`). Wall time is that of `bun ./📜️script.ts …` as a whole.

| command | result | wall | load before | budget |
| --- | --- | --- | --- | --- |
| `test` run 1 of 3 in a row | exit 0, 8 files, 510 tests | 4.99 s (Vitest 3.92 s) | 27 % | 15 s |
| `test` run 2 | exit 0, 510 tests | 3.31 s (Vitest 2.49 s) | 16 % | 15 s |
| `test` run 3 | exit 0, 510 tests | 3.78 s (Vitest 2.40 s) | 37 % | 15 s |
| `test --reporter=json` | exit 0, 510 tests | 2.78 s | 30 % | 15 s |
| `test` (later) | exit 0, 510 tests | 3.92 s | 61 % | 15 s |
| `test quick` | exit 0, 510 tests | 7.40 s; 18.39 s in a second run | 19 %; 100 % | 300 s |
| `test long` | exit 0, 510 tests | 6.87 s; 13.29 s in a second run | 17 %; 35 % | 900 s |
| `test exhaustive` (node, v8 coverage) | exit 0, 514 tests | 86.3 s; 79.1 s in a second run | 93 %; 65 % | 1800 s |
| `typecheck` | exit 0 | 12.0 s; 4.9 s in a second run | 100 %; 19 % | — |
| site `test` (`🎓️teaching/…/❓️quiz/📦️packages/🟦️typescript`) | exit 0, 5 files, 133 tests | 3.93 s | 31 % | 15 s |
| `verify taxonomy report --scope P` | `clean=true errors=0 warnings=0` | | | |
| repo test `contract --owner P` | no breach names the pets tree | | | |

Quietest runs seen (eight alternating runs, the last three of them with the host calm): 2.97 s, 3.29 s, 3.32 s wall,
Vitest 2.17 s – 2.45 s.

Per file at the fundamental level (`--reporter=json`, sum of test durations):

| file | tests | before, quiet | before, loaded | after |
| --- | --- | --- | --- | --- |
| 🎪️stage | 185 | 6.39 s | 12.57 s | 1.29 s |
| 🏞️terrain | 34 | 3.71 s | 5.97 s | 0.21 s |
| ✅️validation | 139 | 3.45 s | 4.05 s | 0.38 s |
| 🎲️randomness | 20 | 0.62 s | 0.95 s | 0.09 s |
| 📐️trigonometry | 15 | 0.62 s | 0.95 s | 0.09 s |
| 🦴️rig | 28 | 0.43 s | 0.51 s | 0.08 s |
| 🧠️behavior | 58 | 0.28 s | 0.55 s | 0.19 s |
| 🎞️animation | 31 | 0.27 s | 0.49 s | 0.15 s |
| all | 510 | 15.78 s | 26.02 s | 2.49 s |

The slowest test is now 81 ms (the frames of a story, troupe); before it was 1945 ms (`strideTo`).

Before, for the other levels: `test quick` 8.2 s, `test exhaustive` 24.1 s, 510 tests each.

Under load, after: three runs in a row at 74 – 100 % took 11.4 s, 14.4 s and 14.6 s wall and passed (Vitest 7.4 s –
10.0 s; the budget of 15 s counts Vitest alone, the wall also the start of the package script). Of 21 further runs
while the load swung between a quarter and all of the host, 20 passed in 3.0 s – 16.1 s and one was killed by the
budget, in a spike in which the neighbouring run reported 46 s of summed transform time.

The site suite at 100 % load was killed by its budget twice (once after all 133 tests had passed, Vitest 13.6 s); its
time there is import time of `📰️host-document`, `🧱️local-stack` and `🧪️deploy`, which are not ours.

## 6. Import and transform time

Nothing wasteful is imported by the core suites: every suite imports its module, the modules that module needs, the
schema twin, its fixtures (the largest, 543 KB, parses in 3 ms) and one third-party oracle that every level needs
(ajv 44 ms, gl-matrix 12 – 29 ms, polygon-clipping 8 ms, d3-ease 6 ms). Vitest's import breakdown
(`--experimental.importDurations.print`) puts almost all of the import time into the test files themselves
(validation 645 ms, behaviour 411 ms, stage 330 ms on a loaded host), which is the wait for Vite's transform of the
file; the product modules cost 1 – 22 ms each. So nothing was changed there.

## 7. Left over

1. **A starved host can still exceed 15 s.** What remains at the fundamental level is 2.5 s of tests and about as
   much start-up, transform and import; when the host starves, the latter grows without bound. Two levers exist
   outside the suites, measured but not applied:
   * `experimental.fsModuleCache` of Vitest 4.1 (a transform cache on disk, in `node_modules/.experimental-vitest-cache`):
     warm, the summed transform time fell to 0.05 s – 0.28 s (from 2.3 s – 12 s in the same minutes) and the import
     time to 0.6 s – 2.0 s. Not applied: it is an experimental option, and a stale cache in a gate is a wrong result,
     not a slow one. Worth a decision of its own.
   * `pool: "threads"`: no gain that the noise of this host lets one see (medians 3.8 s and 3.9 s over 8 runs each).
2. **Four wall-clock assertions** in the stage suite (`performance.now() - started < 200` in "jumps over a quiet
   hour" and "does not keep a resting stage awake", both companies) depend on the host. They did not fail in any run
   here; they were not touched.
3. **`reports the highest rate anyone needs`** asserts that at most one actor is absent in the watched window. With
   20 000 ticks that stops holding for the troupe (two actors are off stage together for 29 ticks from tick 14 242
   on, while relocating; `TK/wp_q_probe_rate.ts`). That is a property of the window, not a law of the stage, so the
   exhaustive level watches 10 000 ticks. Every other clause held over 20 000 ticks in both companies.
4. **The site suite** is not robust under load; the cause is not the pet-cast test (section 4).
5. Something stages files in the git index while agents work (the index followed my edits within minutes). I ran no
   modifying git command. The pets tree is still being edited by someone else: `P/README.md` (the paragraph on the
   decorative layer), `P/🎯️targets/⚛️react/🔨️modules/📡️survey/🟦️.ts` and `P/🧪️tests/📡️surface-survey/🟦️.tsx` changed
   during this work package; none of them is a file of mine except the README, where both edits stand side by side.
6. Not run, as instructed: cargo, end-to-end gates, deploy checks. The Rust twin is untouched.

## 8. Files

Changed: `P/🧪️tests/🎚️config/🟦️.ts`, the eight `P/🔨️modules/*/🧪️tests/🔬️unit/🟦️.ts`, `P/README.md`.

Ticket tools (kept): `wp_q_durations.mjs` (per-file and slowest tests of a JSON report), `wp_q_measure.sh` and
`wp_q_load.ps1` (a run with its wall time and the load before it), `wp_q_variants.sh` (alternating Vitest variants),
`wp_q_probe_stage.ts`, `wp_q_probe_terrain.ts`, `wp_q_probe_rate.ts` (the counts behind the fundamental amounts),
`wp_q_docstring_emojis.mjs` (docstring emojis unique per file, no console, no `[DEBUG]`: clean),
`wp_q_import_cost/` (what each import of the pet-cast test costs).

Tool output: `TK/🗑️generated/wp-q/` (the before and after JSON reports, the run logs, the session counts).
