# 📓️ Work package A6 — gesture (press, click, hold, pick-up, circling, stroking, shaking, heat)

Ticket `2026/10/02/QUIZ-PETS`, second round, design-v2 §15.2, §17, §21, §23; research2 §1.2, §7, §8.4, §11, §13, appendix A.
`P = 🧰️framework/🛍️products/🐾️pets`, `TK` = this ticket folder, `TEST = 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`.
TypeScript is complete and green; the Rust twin and its adapter are phase D.

## 1. What exists

| File | Content |
|---|---|
| `P/🔨️modules/👆️gesture/🟦️.ts` | the pure module (511 lines): guards, press machine, heat, circle, stroke and shake detectors, the hover composite |
| `P/🔨️modules/👆️gesture/🧪️tests/🔬️unit/🟦️.ts` | vitest suite, 36 tests; amounts through `sampled` |
| `P/🧪️tests/👆️gesture-recognition/🥒️.feature` | Protocol v2 case, 9 scenarios (8 fundamental, `travel-hours` exhaustive), oracle `pets-scipy`, profile `pets-float-v1` |
| `P/🧪️tests/👆️gesture-recognition/🐍️.py` | the Python reference (numpy, scipy); never steps a state |
| `P/🧪️tests/👆️gesture-recognition/🟦️.ts` | TypeScript subject adapter; it also exports the replay helpers the unit suite and the ticket tools share |
| `P/🧫️fixtures/👆️gesture-recognition/🔣️.json` | 621 538 bytes, generated; a second generation is byte-identical |
| `TK/generate_gesture_vectors.py` | the hand model and corpus generator; `survey` mode for hours of independent traces |
| `TK/judge_gesture_traces.ts` | lets the TypeScript subject judge a corpus (called by the generator) |
| `TK/probe_gesture_variants.py` | ablation: judges a corpus with copies of the module whose constants are replaced (scratch only) |
| `TK/probe_circle_features.ts` | measures every lap of a corpus before any quality gate (how the gates were chosen) |
| `TK/compare_gesture_projections.py` | holds the subject's harness projections to the oracle's (what `parity` will do once Rust exists) |
| `TK/a6_typecheck.tsconfig.json` | `tsc` over module, suite, adapter and the two TypeScript ticket tools |

Anchored edits in shared files: `P/🔮️oracles/🔣️.json` (`pets-scipy`: capability `pets-gesture-recognition`, `scipy.signal.find_peaks` in the engine, one rationale sentence before "Surveyed and declined"); `P/📦️packages/🟦️typescript/🟦️.ts` (one line `export * from "../../🔨️modules/👆️gesture/🟦️.ts"` after `💗️feeling`; no export name collides with the schema or any module — checked over every `export const|function|type` of the schema and of the 24 other modules present on 2026-10-03). The directory names were copied from `TK/taxonomy_name_probe_v2.ts` (all three already registered by A2). `Cue` comes from `P/🧬️schema/🟦️.ts` (A2 exported `CUES`/`Cue`; the cue fields are `Extract<Cue, …>`).

## 2. Exports

Types (structural, ready to become schema types on `Actor`/`Stage`): `Guards`, `Pointer` (`"mouse" | "pen" | "touch"`), `PressPhase`, `Press { phase, x, y, since, slop }`, `PressInput` (the shell's `pressed`/`dragged`/`released`/`cancelled` events as they are, plus `{ kind: "ticked" }`), `PressSignal` (`"click" | "hold" | "unhold" | "lift" | "drop" | "abort"`), `PressStep { state, signal }`, `Tier` (`TIERS = ["hello","trick","purr","enough"]`), `Caress` (`"click" | "hold"`), `Warmth { heat, since, until, tier, run, tricks }`, `Circling` (15 numbers and a flag), `CircleStep`, `Stroking` (13 numbers and a flag), `StrokeStep`, `Shaking` (11 numbers and a flag), `ShakeStep`, `Hover { circling, stroking }`, `HoverStep`.

```ts
pressStep(press: Press, input: PressInput, tick: Ticks, guards: Guards): { state: Press; signal: PressSignal | null }
pressDue(press: Press): Ticks | null                 // tick at which an armed press becomes a hold (lull/wake horizon)
slopOf(pointer: Pointer): number
heatAt(heat: number, since: Ticks, tick: Ticks): number
heatAfter(heat: number, since: Ticks, tick: Ticks, caress: Caress): number
tierOf(heat: number): Tier
warmthAfter(warmth: Warmth, tick: Ticks, caress: Caress): Warmth      // refractory, forgiveness, run, running trick count
circleStep(circling: Circling, pointer: Point, body: Rect, tick: Ticks, guards: Guards): { state; cue: "circle" | "countercircle" | null }
strokeStep(stroking: Stroking, pointer: Point, body: Rect, tick: Ticks, guards: Guards): { state; cue: "stroke" | null }
shakeStep(shaking: Shaking, grip: Point, height: number, tick: Ticks, guards: Guards): { state; cue: "shake" | null }
hoverStep(hover: Hover, pointer: Point, body: Rect, tick: Ticks, guards: Guards): { state: Hover; cue: … | null }   // one gesture at a time
hoverBusy(hover: Hover): boolean                      // a gesture is under way: keep the full rate
noCircling(rest) / noStroking(rest) / noShaking(rest) / noHover(rest)   // the resets, deaf until `rest`
IDLE: Press, COLD: Warmth, UNGUARDED: Guards
```

Every threshold is an exported literal with a docstring (§3). Only `+ − × ÷`, `Math.abs/min/max` and comparisons; lengths are compared as squares; no `sqrt`, no angle, no velocity vector (a unit test reads the source).

## 3. Thresholds and why

| Constant | Value | Why |
|---|---|---|
| `SLOP_FINE` / `SLOP_COARSE` | 6 / 10 px | research §7.1 (desktop drag threshold; Android touch slop plus finger jitter) |
| `HOLD_TICKS` | 28 (0.44 s) | research §7.1 (long-press 400 ms); WCAG 2.5.2: nothing on the down event |
| `SCROLL_TICKS` | 16 | research §7 common gates (no wheel or scroll in the last 16 ticks) |
| `HEAT_CLICK` / `HEAT_HOLD` / `HEAT_LEAK` | 1 / 1/32 per tick / 0.5 per s | research §8.4; the hold is 1/32 (exact binary) instead of 0.05, so a hold from cold has enough after 4.7 s (measured 4.5–5 s in the suite) rather than 2.6 s |
| `HEAT_HELLO` / `HEAT_TRICK` / `HEAT_ENOUGH` | 1 / 3 / 7 | research §8.4 with its "giggle" tier folded into `purr` (design-v2 §17 has four tiers): a steady clicker (2 /s) gets hello, 2 tricks, 5 purrs, enough at click 9 |
| `ENOUGH_TICKS` / `HEAT_FORGIVEN` | 512 / 2 | research §8.4: 8 s of glances, then the next click is a trick |
| `CIRCLE_MARGIN` / `CIRCLE_REACH` | 8 px / 3.4 | band `max(w,h)/2 + 8 … 3.4·max(w,h)` = research's 32…163 px for the 48 px pet |
| `CIRCLE_HYSTERESIS` | 4 px | Schmitt trigger on both axes (research §7.2) |
| `CIRCLE_QUARTERS` | 5 | a lap is complete when the axis where it began is crossed again (≥ one full turn); research used 4 crossings (270°–360°) — see the ablation |
| `CIRCLE_FAST` / `CIRCLE_SLOW` | 4 / 40 ticks | ≤ 4 turns/s on average over the lap; a stall of more than 40 ticks between quarter turns restarts the lap |
| `CIRCLE_ROUND` | 2.5 | farthest ≤ 2.5 × nearest over the lap (research 3) |
| `CIRCLE_CLOSE` | 1.375 | new: the distance at the end of the lap within ×1.375 of the distance at its beginning (same axis, so independent of ellipse and centre offset) |
| `CIRCLE_AGAINST` | 1 | at most one quarter turn against the lap; the next one restarts the lap in the new direction |
| `CIRCLE_OUT_TICKS` / `CIRCLE_REST` | 8 / 128 | research §7.2 |
| `STROKE_MARGIN` / `STROKE_HYSTERESIS` / `STROKE_LENGTH` | 10 px / 0.15 w / 0.45 w | research §7.3 |
| `STROKE_SLOW` / `STROKE_FAST` | 60 / 900 px/s | research §7.3 (average over the stroke, the rest at its start included) |
| `STROKE_SLANT` | 0.5 | new: a stroke may wander up and down by at most half its length (26.6°) |
| `STROKE_SEGMENTS` / `STROKE_WINDOW` / `STROKE_PAUSE` | 3 / 96 / 48 ticks | research §7.3 |
| `SHAKE_AMPLITUDE` | 0.75 h | research's 36 px for h = 48, scaled with the pet |
| `SHAKE_HYSTERESIS` | 8 px | how far the grip comes back from its farthest point before a reversal is taken |
| `SHAKE_SPEED` | 190 px/s | average over a swing; a sine that peaks at the research's 300 px/s averages 191 |
| `SHAKE_REVERSALS` / `SHAKE_WINDOW` / `SHAKE_PAUSE` / `SHAKE_REST` | 4 / 64 / 48 / 128 | research §7.4 (rest = 2 s of dizziness) |

Rules beyond the research, each forced by a measured false trigger: a stroke must begin at a reversal (the run that led into the zone or follows a rest is no stroke — an overshoot, a correction and the next reach over a pet were three strokes); its middle must lie over the body; its slant is bounded. A circle must close.

## 4. Evidence on real-looking input

### 4.1 The hand model (`TK/generate_gesture_vectors.py`)

Minimum-jerk reaches (`10τ³ − 15τ⁴ + 6τ⁵`, Fitts durations, a slight bend), 30 % with overshoot and a corrective submovement; hover with low-passed (9 Hz) tremor of 0.2–1.6 px; exact rest; devices at 30, 60, 64, 120, 144 Hz read by sample and hold at 64 Hz; whole pixels (mouse) or quarter pixels. Ordinary kinds: `travel` (reaches between targets, half beside or on a pet, dwells of every length), `sweep` (wiping 50–220 px over, above or under a pet at 0.5–3 Hz), `read` (hops along lines with pauses, return sweeps), `select` (slow selection drags, adjusted back and forth, sometimes to the next line), `linger` (resting on a pet with tremor and drift), `wander` (idle persistent random walk, 80–400 px/s, in a 300–600 px box). Stress kind `fidget` (vigorous random walk at 150–900 px/s bouncing in a box of 40–440 px around a pet) is reported separately and is not part of the zero claim. Deliberate: circles (plain: radius 1.1–2.4 × size, 0.7–2.4 turns/s, axis ratio 0.75–1, offset ≤ 20 %, wobble ≤ 10 %, 2.2–3.5 turns; hard: radius 0.85–2.9 × size, 0.45–3.5 turns/s, ratio 0.55–1, offset ≤ 35 %, wobble ≤ 15 %, 1.5–3.5 turns, more tremor), strokes (plain: ±0.4–0.62 w at 1.2–3.4 Hz, slope ≤ 11°, arch ≤ 0.3; hard: ±0.3–0.7 w, 0.8–4.2 Hz, slope ≤ 23°), shakes (±32–70 px at 2.4–4.6 Hz; hard ±25–70 px at 2–5 Hz; 40 % in any direction, with drift), carries (reaches with overshoot and dwells, slow drags, sways at 0.4–1.2 Hz), presses (clicks, double clicks, long presses, slow and quick drags by mouse, pen and finger with jitter).

### 4.2 Survey of independent hours (outside the harness; TypeScript subject judges, 24 pets per page)

`survey 12 21` and `survey 12 22` (24 pointer hours in all), and `survey 6 3 fidget`:

| Ordinary kind | pointer hours | pet hours | cues |
|---|---|---|---|
| travel | 9.21 | 220.95 | 0 |
| sweep | 2.35 | 56.32 | 0 |
| read | 2.88 | 69.18 | 0 |
| select | 2.30 | 55.29 | 0 |
| linger | 2.43 | 58.34 | 0 |
| wander | 2.44 | 58.60 | 0 |
| **all ordinary** | **21.61** | **518.68** | **0** |
| fidget (stress, in the mix) | 2.41 | 57.87 | 3 circles |
| fidget only (stress) | 6.01 | 144.26 | 8 circles, 5 strokes |

| Deliberate group | recognised | answered with another gesture |
|---|---|---|
| circle, plain | 1199 / 1200 (99.9 %) | 0 |
| circle, hard | 949 / 1200 (79.1 %) | 0 |
| stroke, plain | 600 / 600 | 0 |
| stroke, hard | 565 / 600 (94.2 %) | 0 |
| shake, plain / hard | 600 / 600, 600 / 600 | 0 |
| carry (must not shake) | 0 / 1200 shaken | 0 |

The fidget cues are honest: each is a round, closed loop around a pet (ratio ≤ 2.5, ends within ×1.375 of where it began) or three level strokes over its body — continuous vigorous wiggling at a pet produces one about every 28 minutes.

### 4.3 Ablation (`TK/probe_gesture_variants.py`; corpus of seed 21 = 12 h mixed, and the 6 h fidget corpus)

| Variant | wander 1.22 h | fidget 1.2 h | fidget-only 6 h | circle plain | circle hard |
|---|---|---|---|---|---|
| as built | 0 | 1 | 8 circles + 5 strokes | 600/600 | 472/600 |
| research rule (4 crossings, round 3, no closure) | **7 circles** | 34 | **162** + 5 | 600 | 566 |
| 4 crossings only | 1 | 8 | 48 + 5 | 570 | 410 |
| no closure | 0 | 2 | 14 + 5 | 600 | 482 |
| round 3 | 0 | 3 | 19 + 5 | 600 | 520 |
| no Schmitt trigger | 0 | 0 | 9 + 5 | 600 | 469 |
| any number of steps against | 0 | 0 | 3 + 5 | **443** | 332 |
| no speed limit | 0 | 1 | 9 + 5 | 600 | 474 |
| no slant gate | 1 stroke | 1 | 8 + **13** | (strokes unchanged) | |
| shake window 200 ticks | — | — | — | carries shaken: 29 of 600 (34 in the other set) | |

Read: the research rule fires on idle wandering (7 in 1.2 h); the fifth crossing, the closure and the tighter roundness bring that to 0 and the fidget stress from 162 to 8 per 6 h, for 0.1 % plain and ~15 points hard circle detection. One step against is what lets a lap that began on the way in turn round (without it plain detection falls to 74 %). The Schmitt trigger and the speed limit show no measurable effect on these corpora; both stay (research §7.2: a sweep along an axis with tremor; scribbles). The shake window is what separates swaying a pet from shaking it.

Gates were chosen from measured lap features (`TK/probe_circle_features.ts`, 6 h fidget vs 1 200 circles): plain circles have roundness median 1.44 (p95 2.41), closure median 1.05; fidget laps roundness median 10, closure median 1.9. A path-length (tortuosity) gate would have cut the fidget stress further (8 → 3) but needs a square root per tick and is fragile under whole-pixel rounding; not adopted.

### 4.4 The committed vectors

62 presses (38 composed edge by edge, 24 synthesised with jitter — every synthesised one is classified as the hand meant it), 13 histories of attention (630 caresses), 48 circles (32 plain all recognised, 15 of them clean by the oracle; 16 hard, 12 recognised) + 4 guarded twins, 24 strokes (all recognised, 8 clean) + 4, 36 held paths (18 shakes all recognised, 3 clean; 18 carries none) + 4, 40 bodies, 18 stretches of ordinary travel (10.18 pointer minutes, the 6 sample stretches 3.43 minutes). In the four mirrored variants: circles 176/192 (91.7 %, floor 0.85), strokes 96/96 (floor 0.9), shakes 72/72 (floor 0.95). In the 16 exact variants (mirror ×2, transpose, reverse): 625 664 pointer ticks = 2.72 pointer hours, 25 026 560 pet ticks = 108.6 pet hours, 0 cues.

## 5. The Protocol v2 case

Scenarios: `constants` (conformance), `presses`, `warmth`, `circles`, `strokes`, `shakes` (differential), `detection`, `travel-sample` (property, fundamental), `travel-hours` (property, exhaustive). The oracle never steps a state: presses in closed form (`numpy.hypot`, `numpy.flatnonzero` against the tick of the hold), heat as Lindley's recursion (`numpy.cumsum` minus `numpy.minimum.accumulate`, tiers by `numpy.searchsorted`); circles by `numpy.unwrap(numpy.arctan2)`, strokes by `scipy.signal.find_peaks` on the horizontal offset, shakes by `find_peaks` along twelve directions and the principal axis (`numpy.linalg.eigh`). The path cues are recorded from the TypeScript subject at generation and answered as committed only after the oracle admitted each: soundness (every cue ends a full turn in its direction within a lap's time and band, round and closed; two prominent reversals before a stroke; three before a shake), completeness (every gesture the oracle finds clean — its own envelope — is cued, in its direction, by 2.4 turns, the fifth reversal, the sixth swing), guards (control/quiet/still/scrolled silence hover gestures; only still silences a shake), and no clean gesture hidden in ordinary travel or carrying. The subject adapter refuses any answer that differs from the committed one and any cue on ordinary travel, so `subject` alone already fails on drift.

## 6. Commands and real results (2026-10-03)

| Command | Result |
|---|---|
| repo root: `.venv/Scripts/python.exe TK/generate_gesture_vectors.py` | exit 0, summary as §4.4, `"bytes": 621538`; second and third runs `"changed": false` |
| in `TEST`: `bun ./📜️script.ts oracle fundamental --owner "…/🐾️pets" --case "👆️gesture-recognition"` | `[test] level=fundamental cases=1 executed=8 passed=8 failed=0 errored=0 parity=0/0` |
| `… subject exhaustive --implementation typescript …` | `executed=9 passed=9 failed=0` (twice; second run 50 s wall; `travel-hours` 4.99 s, every other scenario ≤ 0.15 s) |
| `… oracle exhaustive …` | `executed=9 passed=9 failed=0` (twice; 65 s wall; `travel-hours` 5.76 s) |
| repo root: `.venv/Scripts/python.exe TK/compare_gesture_projections.py` | all 9 scenarios `equal` (after both runs) |
| in `P/📦️packages/🟦️typescript`: `bun ./📜️script.ts test` | exit 0, `Test Files 15 passed`, `Tests 967 passed`, 8.3 s wall; last run after every edit: `Tests 968 passed` (a sibling added one), 7.3 s wall |
| `bun ./📜️script.ts test quick` | exit 0, `Tests 967 passed`, 21.0 s wall |
| `bun ./📜️script.ts typecheck` | exit 0 (4.5 s; last run 2.4 s) |
| gesture suite alone (`bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts gesture`) | 36 passed; test time 188 ms (fundamental), 1.07 s (quick), 4.76 s (exhaustive) |
| repo root: `bun x tsc -p TK/a6_typecheck.tsconfig.json` | exit 0, 0 errors |
| in `TEST`: `bun ./📜️script.ts contract --owner "…/🐾️pets"` | exit 1 with the repository backlog (6 252 high-priority breaches across 5 rules); no line and no entry of `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` names `👆️gesture-recognition` or `🔨️modules/👆️gesture` (the pets lines name a sibling's `🚧️clearance-proof` feature) |
| docstring check (node script over the 9 files) | every docstring starts with an emoji, none twice per file; no console, no `[DEBUG]` |

Not run, as instructed: `parity` (needs the Rust adapter), cargo, e2e, deploy checks.

## 7. Notes for the Rust twin

- Translate expression by expression; all state fields are numbers or flags (ticks may be `f64` or `i64`; `heatAt` clamps `tick − since` at 0, so unsigned ticks need a saturating subtraction). `Math.max/min/abs` ≙ `max/min/abs` on finite values. Squares are computed at the point of use (`CIRCLE_ROUND * CIRCLE_ROUND`, `CIRCLE_CLOSE * CIRCLE_CLOSE` = 1.890625 exact, `pace * pace`, `span * 64 * 64` left to right).
- `circleStep`: `quadrantOf` and `turned < 0 ? turned + 4 : turned` (no `%`); `0 - CIRCLE_HYSTERESIS`; the lap fields are computed from `fresh` exactly as written (keep the order of the ternaries; `way` is −1 when `step` is 0 and is then unused).
- `strokeStep`: `count` starts at −1 (the run that led in); `first`/`second` are a two-slot ring hard-wired to `STROKE_SEGMENTS = 3`.
- `shakeStep`: `mark1..mark3` are a three-slot ring hard-wired to `SHAKE_REVERSALS = 4`.
- Cues: the `Cue` enum of the schema twin; the adapter projects `[tick, cue]` rows, presses `[tick, signal]`, warmth `[heat, tier, run, tricks, until]` (heat is an exact binary fraction).
- The adapter must decode paths (`hold + n` repeats), build the 16 variants on integer quanta (mirror `page − x`, body `page − x − w`, transpose swaps width and height, reverse) and divide by `quantum` at the end — then every number is exact and the ticks equal the committed ones bit for bit. The mirror law (same ticks, circle ↔ countercircle) is exact and the `detection` scenario checks it.

## 8. Notes for the stage integrator (B2) and the shell (C1)

- **State**: `Stage` gains `press: Press`, the actor the press belongs to (the topmost body under the press point at `pressed`), and `shaking: Shaking` (the held pet); `Actor` gains `hover: Hover` and `warmth: Warmth`.
- **Order per tick**: fold `pressed`/`dragged`/`released`/`cancelled` through `pressStep` as they arrive (with the guards of that moment); on `ticked` call `pressStep(press, { kind: "ticked" }, tick, guards)` once (it alone turns an armed press into `hold`); then, while the press is idle, `hoverStep` for each actor with the sample-and-hold pointer and the actor's solid body (`bodyOf` of A5, or the frame's `body`); while lifted, `shakeStep` with the pointer as the grip and the held species' height. Reset with `noHover(rest)` when the pointer leaves the page (`unpointed`) and with `noShaking(tick)` on drop or abort.
- **Signals**: `click` → `warmthAfter(…, "click")` and react by its tier: `hello` → greet; `trick` → the species' click tricks in order, number `warmth.tricks − 1` (modulo their count); `purr` → purr; `enough` with `run === 1` → shrug and turn away, `run > 1` → only a glance. `hold` → purr loop; every tick while holding `warmthAfter(…, "hold")` (it answers `enough` when the heat says so: end the purr with a shrug). `lift` → footing `hand`, grip at the scruff (the press point is in `press.x/y` if wanted). `drop` → release with velocity. `abort` → undo by the phase it came from (armed: nothing; holding: stop purring; lifted: glide back). Cues `circle`/`countercircle`/`stroke`/`shake` → `Trick.cues`.
- **Guards**: `control` = the shell's `Pointed.over === "control"` and also while a button the pets did not take is down (text selection); `scrolled` = the page moved under the pointer since the last tick (design-v2 has no `scrolled` event yet — a scroll/wheel listener sending `stirred` with a flag, or a survey whose offset changed, can provide it); `quiet`/`still` from the mode. A press is not taken over a control or on a still stage; quiet does not affect presses.
- **Horizons (CORE X4)**: `lull`/wake must include `pressDue(press)`; `paceOf` must answer 64 while `hoverBusy(actor.hover)` for any actor, while a press is open, and while a pet is held. The detectors are verified with devices of 30 Hz (sample and hold every ~2 ticks) and a circle still works with a sample every 4th tick at 1 turn/s; the shell must deliver pointer moves at least once per animation frame (coalesced events are welcome but not needed) and every `pointerdown`/`pointerup`/`pointercancel` of the primary pointer with its `pointerType`; `Escape`, `lostpointercapture`, `blur` and `visibilitychange` → `cancelled`.
- **Not done here**: which actor a press hits, mood impulses, trick choice and clips — all stage business.

## 9. Deviations from the design and the research

1. Circle: five crossings (closure of the turn) instead of four; roundness 2.5 instead of 3; the new closure gate. Measured in §4.3.
2. Stroke: the entering run never counts; the slant gate and the "middle over the body" rule are new; after a cue the row starts again without a rest, so petting cues every three strokes.
3. Shake: reversals between the anchor and the farthest point (positions, not per-tick velocity), with an average-speed gate instead of a peak-speed gate (robust to sample and hold at 30 Hz); amplitude in body heights. The first swing from the pick-up point may count.
4. Heat: four tiers (giggle folded into purr), hold heat 1/32 per tick, and the answer is the state itself (`tier`, `run`, running `tricks`, `until`).
5. Press: no separate multi-click chain — the heat is the one model (research §7.1 says so); a hold that leaves the slop becomes a pick-up; a press arriving while one is open aborts it and arms anew; a scroll aborts only an unlifted press.
6. The press point is the "grip" of `lift`; the scruff grip of research §1.2 is the integrator's (species data).
7. Evidence: the exhaustive "hours" of the harness are 10.18 minutes of distinct committed travel in 16 exact variants past 40 bodies (2.72 pointer hours, 108.6 pet hours), because hours of raw traces would not fit a fixture; the independent hours (21.6 pointer hours, 519 pet hours) come from the survey of §4.2, which is reproducible from its seeds but not part of the harness.
8. The path cues of the vectors are recorded from the TypeScript subject and admitted by the oracle's readings (like `🎪️stage-trace`), not computed by the oracle; presses and heat are computed by the oracle and must equal the subject at generation.
9. The fixture is 621 KB, the largest of the product (paths dominate).

## 10. Open

- Phase D: `🦀️.rs` module, Rust unit suite and `P/🧪️tests/👆️gesture-recognition/🦀️.rs`; then `parity`.
- `scrolled` needs an input in the event contract (§8).
- Tuning by eye in the stories gallery once B2 lands; when a threshold changes: the literal in `🟦️.ts` (and `🦀️.rs`), the same literal in `🐍️.py`, rerun the generator (scenario `constants` fails if one side is forgotten).
- Scratch outputs of this package (`TK/🗑️generated/a6/`: survey corpora, verdicts, variant copies, harness logs) were deleted after the runs; the tools above recreate them.
