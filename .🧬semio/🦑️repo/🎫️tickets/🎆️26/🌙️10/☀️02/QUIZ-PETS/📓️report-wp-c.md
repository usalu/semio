# 📓️ Report — work package C (animation)

Ticket `2026/10/02/QUIZ-PETS`, phase 1 (TypeScript). `P` = `🧰️framework/🛍️products/🐾️pets`, `TK` = this folder, `TEST` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`.

## 1. What exists

| File | Content |
|---|---|
| `P/🔨️modules/🎞️animation/🟦️.ts` | the module (design §4.4), 169 lines, no runtime dependency |
| `P/🔨️modules/🎞️animation/🧪️tests/🔬️unit/🟦️.ts` | vitest unit suite, 31 tests (d3-ease as JavaScript oracle; also holds the species and tracks of the vectors to the product's own `speciesIssues`, and the module source to the arithmetic of design §2.4) |
| `P/🧪️tests/🎞️animation-sampling/{🥒️.feature, 🐍️.py, 🟦️.ts}` | Protocol v2 case, 6 scenarios, oracle `pets-scipy`, profile `pets-float-v1` |
| `P/🧪️tests/🪀️spring-settling/{🥒️.feature, 🐍️.py, 🟦️.ts}` | Protocol v2 case, 5 scenarios, oracle `pets-numpy`, profile `pets-float-v1` |
| `P/🧫️fixtures/🎞️animation-sampling/🔣️.json` | generated vectors (1697 lines, 112 kB) |
| `P/🧫️fixtures/🪀️spring-settling/🔣️.json` | generated vectors (337 lines, 11 kB) |
| `TK/generate_animation_vectors.py` | the generator of both fixtures (loads the two `🐍️.py` oracles, idempotent: a second run leaves both files byte-identical) |
| `TK/measure_animation_deviation.py` | prints, per scenario, how far the TypeScript subject lies from the Python oracle in the last harness run |

No Rust file was created (phase 2). No file of a sibling package, of the quiz product or of the registries was edited.

## 2. The module — exported names

Fixed by the design: `easeBezier(ease, amount)`, `sampleTrack(track, phase)`, `clipTicks(clip)`, `sampleClip(species, clip, ticks): Pose`, `blendPose(from, to, amount): Pose`, `springStep(position, velocity, target, stiffness, damping): Spring`, `BLINK_TICKS = 12`, `lidAt(ticks)`.

Added by this package (owner's choice): `type Spring = { readonly position: number; readonly velocity: number }` (the return type of `springStep`), `GAZE_STIFFNESS = 512`, `GAZE_DAMPING = 32`.

Imports: `TICKS_PER_SECOND` and types from `🧬️schema`, `Pose`/`BonePose` (types only) from `🦴️rig`, `lerp`/`smoothstep` from `📐️trigonometry`. No export name collides with another module or the schema (checked over every `export` of `P/🔨️modules` and `P/🧬️schema` on 2026-10-02 05:00).

### Semantics decided here (all stated in the docstrings)

| Function | Decision |
|---|---|
| `easeBezier` | 0 at and below 0, 1 at and above 1 (no CSS extrapolation; phases never leave 0 … 1). Both coordinates are the Horner cubic `((a·s + b)·s + c)·s`, `c = 3·p1`, `b = 3·(p2 − p1) − c`, `a = 1 − c − b`. 48 halvings of `[0, 1]`, the lower half kept while `x(middle) < amount`; the parameter is the middle of the last interval. |
| `sampleTrack` | first key's value at and before its phase, last key's at and after its phase; the segment is the last one whose earlier key lies at or before the phase, so a phase on a key yields that key's value exactly and a stretch of zero width is never divided by; `lerp` by the local phase, eased by the earlier key. A track without keys yields the rest value of its channel (1 for scales, 0 otherwise). |
| `clipTicks` | `floor(seconds × 64 + 0.5)`, at least 1. |
| `sampleClip` | ticks before the beginning count as 0; looping: `(ticks mod length) ÷ length`; otherwise `min(ticks, length) ÷ length`. Tracks are written in clip order (the later of two tracks on one channel wins); a track on a bone the species lacks is skipped; untouched channels stay at rest. |
| `blendPose` | returns `from` itself at and below 0 and `to` itself at and above 1 (so a finished blend is exactly the clip pose), `lerp` per channel in between; rotations are plain degree offsets, never wrapped. Both poses must be of the same rig. |
| `springStep` | `velocity′ = velocity + (stiffness × (target − position) − damping × velocity) × 0.015625`, `position′ = position + velocity′ × 0.015625`. A spring resting on its target stays there exactly. Stable while `stiffness > 0`, `damping > 0`, `stiffness ÷ 4096 + damping ÷ 32 < 4` (Jury conditions of the one-tick matrix; confirmed by `numpy.linalg.eigvals`). |
| `lidAt` | 0 for `ticks ≤ 0` and `ticks ≥ 12`; closing over 4 ticks `smoothstep(ticks ÷ 4)`, shut at ticks 4 and 5, opening over 7 ticks `1 − smoothstep((ticks − 5) ÷ 7)`. A stage drawn at every other tick (rate 32) sees a fully shut lid whichever parity it draws. |

### Gaze spring (for package E: `springStep(gaze.x, gaze.vx, target.x, GAZE_STIFFNESS, GAZE_DAMPING)` per axis)

`GAZE_STIFFNESS = 512` (1/s²), `GAZE_DAMPING = 32` (1/s): natural frequency 22.6 rad/s, damping ratio 0.71 before discretisation. The one-tick matrix on `(position − target, velocity)` is `[[0.875, 0.0078125], [−8, 0.5]]`, exact in binary. Measured for a pupil at rest whose target jumps (shares of the way): 12.5 % after 1 tick, 62 % after 4, within 2 % from tick 9 (0.14 s), within 1 % from tick 14 (0.22 s), one overshoot of 1.01 % at tick 13, within 0.1 % from tick 21. The case `🪀️spring-settling` states and enforces: within 1 % from tick 16 at the latest, overshoot at most 2 %.

Candidates compared (unit step, same integrator): 576/36 overshoots by 0.15 % (within 2 % from tick 10), which is no spring at all; 384/28 overshoots by 1.2 % but has covered only 89 % of the way after 8 ticks (512/32: 95 %); 512/28 overshoots by 4.5 %, which is more than "slightly". 512/32 was chosen for its small overshoot and its exact matrix. Whether the overshoot is visible at pupil scale (a travel of one to two pixels) has not been judged by eye; that belongs to the stories gallery.

## 3. Verification — exact commands and real results

All run on 2026-10-02 between 04:50 and 05:12 on this host (bun 1.4.2, Windows).

| Command | Result |
|---|---|
| `.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/generate_animation_vectors.py` (repo root) | wrote both fixtures; a second run left the SHA-256 of both files unchanged |
| `cd P/📦️packages/🟦️typescript && bun ./📜️script.ts test quick animation` | `Test Files 1 passed (1)`, `Tests 31 passed (31)` |
| `cd P/📦️packages/🟦️typescript && bun ./📜️script.ts test quick` (whole package, for information) | `Test Files 6 passed (6)`, `Tests 258 passed (258)` |
| `cd TEST && bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/🐾️pets"` | exit 1 with `5868 high-priority breach(es) across 4 rule(s)` — the command prints the breaches of the whole repository. Three of them name pets, all in `P/🧪️tests/🧬️schema-conformance/🥒️.feature` (package A): `Step at line 14 / 19 / 26 is outside a Background or Scenario`. None names `🎞️animation-sampling` or `🪀️spring-settling`. |
| `cd TEST && bun ./📜️script.ts oracle exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "🎞️animation-sampling"` | `[test] level=exhaustive cases=1 executed=6 passed=6 failed=0 errored=0 parity=0/0` |
| `… parity exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "🎞️animation-sampling"` | `[test] level=exhaustive cases=1 executed=12 passed=12 failed=0 errored=0 parity=6/6` |
| `… oracle exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "🪀️spring-settling"` | `[test] level=exhaustive cases=1 executed=5 passed=5 failed=0 errored=0 parity=0/0` |
| `… parity exhaustive --owner "🧰️framework/🛍️products/🐾️pets" --case "🪀️spring-settling"` | `[test] level=exhaustive cases=1 executed=10 passed=10 failed=0 errored=0 parity=5/5` |
| `.venv/Scripts/python.exe TK/measure_animation_deviation.py` (after the parity runs) | see below |
| `tsc --noEmit` over the four TypeScript files (root options plus `noUncheckedIndexedAccess`) | no diagnostic in any pets file |
| `inventoryTaxonomy` over scope `P` | no violation in a path of this package (2 warnings in `🦴️rig-solving/🥒️.feature`, package B) |
| `TK/../QUIZ-PRODUCT-AND-TEACHING-PROCTOR/domain_docstring_emojis.py` over the eight files | `0 finding(s) in 8 file(s)`; every docstring emoji carries U+FE0F |

The subject is TypeScript only (no `P/📦️packages/🦀️rust` yet), so `parity` compares the oracle with one subject.

Oracle versus subject, largest absolute difference per scenario (tolerance 1e-9):

```
🎞️animation-sampling/bezier-easings: 573 numbers, largest difference 3.408e-14
🎞️animation-sampling/blink-lids: 17 numbers, largest difference 1.110e-16
🎞️animation-sampling/clip-lengths: 16 numbers, largest difference 0.000e+00
🎞️animation-sampling/clip-poses: 3150 numbers, largest difference 1.350e-13
🎞️animation-sampling/pose-blends: 1050 numbers, largest difference 3.553e-15
🎞️animation-sampling/track-samples: 301 numbers, largest difference 1.208e-13
🪀️spring-settling/gaze-settling: 12 numbers, largest difference 5.551e-15
🪀️spring-settling/resting: 8 numbers, largest difference 0.000e+00
🪀️spring-settling/single-steps: 14 numbers, largest difference 1.110e-16
🪀️spring-settling/stable-springs: 10 numbers, largest difference 5.329e-15
🪀️spring-settling/tick-runs: 132 numbers, largest difference 1.243e-14
```

The differences are small but not zero where the two sides compute differently, so the comparison is not a replay of the vectors.

### What the oracles are

| Scenario | Third-party evidence |
|---|---|
| `bezier-easings` | Bernstein form of the curve, inverted by `scipy.optimize.brentq` (the subject bisects a Horner form) |
| `track-samples` | `numpy.interp` over whole linear tracks; eased tracks through `numpy.searchsorted` segments whose arithmetic must reproduce `numpy.interp` with the eases removed |
| `clip-lengths` | `numpy.floor`, confirmed as round-half-up of the exact rational product (`fractions.Fraction`) |
| `clip-poses` | the above per track, `numpy.mod` / `min` for the phase; species = `TK/reference-species.json` plus one clip `lunge` (played once, eased `x` and scales) |
| `pose-blends` | numpy's convex combination `(1 − a)·from + a·to` |
| `blink-lids` | `scipy.interpolate.CubicHermiteSpline` through (0, 0), (4, 1), (5, 1), (12, 0) with flat tangents |
| `single-steps`, `tick-runs`, `resting`, `stable-springs` | `numpy.linalg.matrix_power` of the one-tick matrix (the oracle's own `n` single products must agree within 1e-9 before it answers); `numpy.linalg.eigvals` for the stability region, inside and outside |
| `gaze-settling` | settling tick and overshoot read off the matrix-power trajectory; the oracle refuses vectors beyond 16 ticks or 2 %; the subject projects its own `GAZE_STIFFNESS`/`GAZE_DAMPING`, so tuning them without rerunning the generator fails the case |

Both oracles hold the committed vectors to their recomputation (`agree`, floats within 1e-12 because a root finder may differ in the last bits between library builds).

Unit suite, JavaScript oracle: d3-ease 3.0.1. `easeLinear`, `easeCubicIn`, `easeCubicOut`, `easeQuadIn`, `easeQuadOut` equal `easeBezier` with the control points `(1/3, y1, 2/3, y2)` of the same polynomial within 1e-13 on 1025 amounts; CSS ease-in-out is held to the family of `easeCubicInOut` (symmetric, rising, same side of the diagonal, never 0.1 apart; they are different curves, the largest gap is 0.084).

## 4. Deviations from the design and the brief

1. `--case` takes the directory name with its emoji (`--case "🎞️animation-sampling"`); the bare slug selects nothing (`cases=0`).
2. Oracles beyond the two named in design §10: `scipy.interpolate.CubicHermiteSpline` (lid) and `numpy.linalg.eigvals` (stability region), both inside the registered packages; `fractions.Fraction` from the standard library for the rounding check.
3. The integer remainder `%` is used for the wrap of a looping clip. Ticks are whole numbers, so it is exact; the Rust twin uses the integer `%`.
4. Shared vectors carry no negative ticks, so the Rust twin is free to use unsigned ticks; "ticks before the beginning count as 0" is pinned in the TypeScript unit suite only.
5. The fixtures are written with one row of numbers per line instead of one number per line (same JSON; the generator proves that the rendered text reads back as the document).
6. Where the abscissa of an easing stands still inside the curve (`cubic-bezier(1, 0, 0, 1)` at exactly one half) every solver is ill-conditioned: the subject answers 0.4999971 instead of 0.5. This is deterministic and invisible, the vectors sample that easing on both sides of one half but not at it, and the unit suite pins the bound (1e-5). Art packages should not rely on that single phase.

## 5. Open

- **Phase 2 (Rust twin)**: `P/🔨️modules/🎞️animation/🦀️.rs`, its `🧪️tests/🔬️unit/🦀️.rs` and the two `🦀️.rs` adapters. Notes for the twin: evaluate `((a·s + b)·s + c)·s` exactly as written, never `mul_add`; `TICK_SECONDS = 0.015625`; the bisection loop runs exactly 48 times with `<`; `clip_ticks` is `(seconds * 64.0 + 0.5).floor()` clamped to at least 1; `lid_at` uses the literals 4, 5, 7, 12; `blend_pose` returns the unblended pose at the ends. The projections are objects keyed by vector id; a pose is an array of `{x, y, rotation, scaleX, scaleY}`; `gaze-settling` projects `{stiffness, damping, jumps: {id: {settled, overshoot}}}`; `blink-lids` projects `{blinkTicks, closures}`.
- **Package A**: `P/🔮️oracles/🔣️.json` names d3-ease in two rationales but has no entry for it. Proposed entry: id `pets-d3-ease`, `kind: third-party-library`, `ecosystem: javascript`, `package: d3-ease`, `version: 3.0.1`, capability `pets-animation-sampling`, profile `pets-float-v1`, `hostPath: 🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript`, rationale: judges `easeBezier` inside `🔨️modules/🎞️animation/🧪️tests/🔬️unit` for the polynomial eases that are Béziers with a linear abscissa. The engine text of `pets-scipy` could also name `scipy.interpolate.CubicHermiteSpline`, that of `pets-numpy` `numpy.linalg.eigvals`.
- **Package A**: the three contract breaches in `🧬️schema-conformance/🥒️.feature` (description lines that begin with a step keyword).
- **Package E**: use `GAZE_STIFFNESS`, `GAZE_DAMPING` and the `Spring` type by these names; the constants are starting values, to be tuned in the stories gallery, after which `TK/generate_animation_vectors.py` (its `GAZE_STIFFNESS`/`GAZE_DAMPING` rows) is rerun.
- Tool output of this package: `TK/🗑️generated/wp-c/` (six short result files); temporary probes were removed.
