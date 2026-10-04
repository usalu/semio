# Quiz Pets, round 2: research brief for mechanics (drag and hang, parachute, grapple, ladder, wall, non-overlap, gestures, moods, effects, host displacement, accessibility, events)

Date: 2026-10-02. Companion of `📓️research-pet-animation.md` (round 1) and `📓️design.md` (§2.4 determinism, §4 core modules, §5 simulation rules, §13 as built). Nothing of round 1 is repeated here; names such as `springStep`, `perchesOf`, `landingOf`, `hopOf`, `GRAVITY = 1800`, `FALL_SPEED = 900`, `COMFORT_GAP = 8`, `MEET_GAP = 6`, `sinTurns`, `randomWords`, `STAGE_STREAM` are the existing ones.

Legend for evidence: **[V]** fetched in this session and the specific facts I rely on were extracted from the page (the fetch tool answers a prompt about a page with a model-written extract, it does not return the raw text, so even [V] is "targeted extraction", not verbatim reading; only D. Silver's paper, whose rendered pages I looked at, and the repository files were read directly); **[V-sum]** fetched, but the answer was vague, decoded from an encoded source file, or about a long document; **[S]** only seen in a search summary; **[M]** from my own knowledge, not re-verified; **[D]** design decision or starting value to tune by eye in the stories gallery; **[P]** measured in a throw-away Python/Node prototype that lives outside the repository (scratch directory of this session, not committed; the algorithms that matter are reproduced in appendix A so the experiment can be redone). Nothing was built in the repository and no repository test was run.

Pet size convention used for all starting values: height `h` = 48 px and width `w` = 40 px as the reference (the menagerie spans 40 to 56 px); every length is given as a multiple of `h` or `w` and the resulting pixels for the reference pet. Time: `dt = 1/64 s`, `dt² = 1/4096` (exact). Velocities in px/s in documents, divided by 64 once per use in the core.

---

## 0. Decisions at a glance

| # | Topic | Recommendation |
|---|---|---|
| 1 | Hanging | One shared constrained-Verlet primitive `swingStep` (SHAKE-style: one multiplier, one `sqrt`) used for the held pet, the parachute and the rope swing. **Do not use the usual "Verlet + project onto the circle" (position-based) step**: it is only first-order accurate and loses 10 % to 18 % of the amplitude in the first half swing at 64 Hz (24 % of the energy at 45 degrees) [P]. SHAKE matches an RK4 reference within 0.15 degrees over 3 s and does not drift over 60 s [P]. |
| 1 | Following | Anchor = critically damped spring on the pointer (halflife 0.06 s: `k = 534`, `c = 46`), body = rod pendulum under the anchor with a high "held gravity" (about 4 x) so that ordinary drags lean 25 to 50 degrees and flicks saturate at a 60-degree cone. |
| 1 | Release | Quadratic least-squares velocity over the last 7 tick samples (integer weights `[7,-2,-7,-8,-5,2,13]/28`), blended with the pendulum's own velocity; cap 640 px/s, dead zone 70 px/s, stale after 3 identical samples. |
| 2 | Parachute | Open when the predicted impact speed `sqrt(vy^2 + 2 g H)` exceeds 600 px/s (from rest: a drop of more than about two body heights) and `vy >= 240` px/s; 6-tick reflex delay, then exponential approach to terminal speed `2h px/s` (`tau = 0.18 s`, per-tick factor 0.91685535573); canopy swings by the same `swingStep`. From 100 px the chute cuts the impact speed from 600 to 233 px/s, from 150 px to 118, from 200 px to 98 [P]. |
| 3 | Grapple | Straight hook at 10 px/tick, clear-line test by segment-versus-rectangle (Liang-Barsky, comparisons only), rope length 0.8h to 3.2h, two outcomes: `zip` (reel straight, steep) or `swing` (SHAKE with a shortening length, which conserves angular momentum for free), then `mantle`. Reeling a SHAKE rope needs a tangential speed cap (520 px/s) or the swing winds up without bound [P]. |
| 4 | Ladder | A stage object with owner, lifetime and an occupant lane. Lean ratio `dx/H` in [0.14, 0.40] (default 0.25, the real-world 4:1 rule), height 0.8h to 3.6h. Climb phase is driven by distance (not time), 2.5 rungs/s up. Upper surface moves: ladder follows within 8 px and ratio range, else topples (rider falls or opens a parachute). |
| 5 | Wall | The survey must add `walls` (vertical box edges with free stretches). Climb 30 px/s up, grip budget 6 s, mantle over the corner in 28 ticks, slip when the wall moves more than 6 px. Species without limbs never climb (floaters change lanes instead). |
| 6 | Non-overlap | Invariant: **no two solid bodies overlap at the end of any tick.** Mechanism: sequential update with check-before-commit; swept corridors (claims) for everything that flies; heads of other pets are one-way platforms with a slide-off; held pets are projected out of other bodies; surveys re-seat with an isotonic projection (PAVA) executed as a guarded scoot; the only escape hatches are "ghost" (translucent, intangible, bounded) and "crowd out". In a prototype of 8 agents (24 seeds, 30 000 ticks each, brutal user and survey chaos) this produced **0 overlapping ticks in 5.76 million actor-ticks**; switching off any of the four core rules (walker corridor guard, hop planning check, held projection, ghost fallback) produced 4 580 to 96 097 overlapping ticks in 180 000 [P]. |
| 7 | Gestures | Circling: quadrant winding with a Schmitt trigger on both axes, radius band, per-quarter timing and "at most one opposite step": 98 % detected, **0 false triggers in 8 pet-hours of simulated ordinary pointer travel** [P]. Stroking: reversal segments of at least 0.45 w (94 % detected, 0 false). Shaking: reversal count with amplitude and peak-speed gates (99 % detected, 0 false in 1.4 h of ordinary drags) [P]. |
| 8 | Moods | Layers: physical mode (exclusive) > activity (exclusive) > mood (8-value enum with intensity, decay, priority) > needs (slow) > bonds. Pokes feed a leaky bucket (`heat`) that saturates into a refractory "enough", never into punishment. Influence rules are data: `when { actor, other, near, each } then effects[]`. |
| 9 | Effects | Emitters in the frame (numbers only), particles as pure functions `p(emitter, index, tick)`; births from `floor(i * period + jitter)`, jitter from a 32-bit hash; positions from `+ - * / floor` plus the existing `sinTurns` and Holden's `fastNegExp`. Batch particles into at most 3 `<path>` nodes per emitter (alpha buckets); cap 160 live particles and 96 nodes world-wide. |
| 10 | Displace host | `style.translate` only (CSSOM write, CSP-safe), at most 6 px, release on **hover** (before the press), not on press, because removing a transform between `pointerdown` and `pointerup` can swallow the click. Survey must subtract what it applied. Never on tables, sticky elements, ancestors of fixed descendants, controls under or near the pointer, coarse pointers, or during a run. |
| 11 | A11y | Layer stays `aria-hidden`; pointer interaction on window-level listeners for mouse and pen, hit pads only in an explicit "play" mode for touch; path-based tricks (circle, stroke, shake) and dragging need single-pointer and keyboard equivalents (click escalation, a "Play with the pets" panel, click-to-pick then click-to-place); Escape and `pointercancel` abort; a control under a pet always wins because pets only stand on cleared perches. |
| 12 | Events | Shell to core: `pressed`, `released`, `cancelled`, `dragged`, `reclaimed`, `played`, `permitted`, `scrolled`; `surveyed` gains `walls`. Core to renderer: `ActorFrame` gains `tilt`, `pivot`, `tools`, `feeling`; `Frame` gains `ladders`, `effects`, `nudges`, `grab`. |

### 0.1 New small math the mechanics need (all inside `+ - * / sqrt abs floor min max`)

- `atanTurns(y, x)`: a polynomial arctangent with octant reduction. Needed for ladder and rope angles, wall and mantle poses, and the lean of a hanging pet. Use Abramowitz and Stegun 4.4.49 on [0, 1] [M for the source, constants verified numerically here [P]]: `atan(z) ~ z (a1 + z2 (a3 + z2 (a5 + z2 (a7 + z2 a9))))`, `z2 = z z`, `a1 = 0.9998660`, `a3 = -0.3302995`, `a5 = 0.1801410`, `a7 = -0.0851330`, `a9 = 0.0208351`. Maximum error 1.15e-5 rad; the full-circle `atan2` in turns built from it (`ay <= ax ? atan(ay/ax) : 1/4 - atan(ax/ay)`, mirror for `x < 0`, negate for `y < 0`, all divided by `2 pi`) has a maximum error of 1.83e-6 turns (0.0007 degrees) [P]. Prefer vectors over angles wherever possible (the matrix of a leaning pet needs only `sin = u/|u|`, `cos = sqrt(1 - sin^2)`); use `atanTurns` only where the rig takes an angle.
- `fastNegExp(x) = 1 / (1 + x + 0.48 x^2 + 0.235 x^3)` (Holden, round 1 section 2.4) for drag-like saturation; `1 - fastNegExp` is monotone and saturating.
- `mix(a, b)`, a 32-bit hash for effects (section 9.2). `lowbias32` constants `0x7feb352d`, `0x846ca68b`, shifts 16, 15, 16 [V: https://nullprogram.com/blog/2018/07/31/].
- `segmentHitsRect(p, q, rect)` (Liang-Barsky clipping: only `+ - * /` and comparisons) for rope and ladder lines.
- `isotonicSeat` (pool-adjacent-violators) for re-seating (section 6.4).

### 0.2 Physical mode, activity, mood: the layering all sections assume

```
physical mode (exclusive, owned by the stage, decides which integrator and which claims apply)
   ground --------- walk / stand on a perch                       box: posture "stand"
   air ------------ hop / fall / thrown (ballistic, claim = swept corridor)   box: "stand" (+ tumble)
   chute ---------- parachute descent (swingStep under a canopy)   box: "stand" + canopy
   held ----------- dangling under the pointer (swingStep)         box: "stand" widened by lean
   rope ----------- aim, hook flight, zip / swing / climb-last     box: "stand" (+ gun)
   ladder --------- mount, climb, dismount                         box: "climb"
   wall ----------- grab, climb, hang, slide, mantle               box: "climb"
   head ----------- standing on another pet's head, sliding off    box: "stand"
   flags: ghost (intangible, translucent, bounded), solid
activity (exclusive label + intent; selects clip and weights): the existing eleven + held, thrown, chute, brace,
   aim, shoot, reel, swing, climb, mantle, slide, trick, purr, dizzy, shrug, scoot, place
mood (enum + intensity, section 8): modulates weights, face, tricks, contagion
needs (slow scalars, existing) and bonds / rapport (pairwise, existing)
```

Rule: **physics decides where a body can be; activity decides what it looks like; mood decides what it wants.** Only the physical-mode layer touches claims and the non-overlap invariant.

---

## 1. Dragging and hanging

### 1.1 Prior art for the feel

- **Shimeji-ee** [V-sum: https://raw.githubusercontent.com/TigerHix/shimeji-ee/master/conf/actions.xml]: `Dragged` is a sequence of `Pinched` (an embedded `Dragged` action that picks one of five sprites by the horizontal offset of the foot from the cursor: foot left of cursor by more than 50 px, 30 to 50, within 30, right by 30 to 50, more than 50) and `Resisting` (two sprites alternating, 250 ms). `Thrown` is `Falling` with `InitialVX = cursor.dx` and `InitialVY = cursor.dy`; `Falling` has `Gravity = 2`, `RegistanceX = 0.05`, `RegistanceY = 0.1` (per action tick). Lesson: five discrete dangle poses chosen by the foot-to-cursor offset already read as "swinging"; the throw is simply the cursor's last delta; there is no pendulum. A cheap fallback for a species without a rig that can lean.
- **Android `VelocityTracker`** [V-sum: https://android.googlesource.com/platform/frameworks/native/+/master/libs/input/VelocityTracker.cpp]: default strategy `LSQ2` (second-degree least squares), horizon 100 ms, a pointer that has not moved for 40 ms (`ASSUME_POINTER_STOPPED_TIME`) is treated as stopped, result clamped to a maximum fling velocity. A quadratic fit gives the velocity at the moment of lift-off better than a line or a two-point difference. Android `ViewConfiguration`: touch slop 8 dp, long-press 400 ms, double-tap timeout 300 ms, minimum / maximum fling 50 / 8000 dp/s [V-sum: https://android.googlesource.com/platform/frameworks/base/+/refs/heads/main/core/java/android/view/ViewConfiguration.java].
- **Jakobsen, "Advanced Character Physics"** [V-sum: https://github.com/krisives/advanced-character-physics]: Verlet integration `x += x - oldx + a dt^2`, stick constraints solved by relaxation, typically three to four relaxation passes per step (up to ten in his game). This is the origin of the usual position-based rope; section 1.3 shows why it is the wrong choice here.

### 1.2 Press, pick up, hold, click (state machine)

Thresholds are in section 7.1. The point is that **a press alone changes nothing** (WCAG 2.5.2, section 11):

```
            pressed(x,y)  on a hit circle, primary button, target not interactive, quiet off, play allowed
  idle ------------------------------------------------------------------> armed
                                                                      |     |       |
              released before HOLD_TICKS, moved < slop -> click       |     |       | moved >= slop
              (poked / escalation, section 7.1) <---------------------+     |       v
                                                                            | held >= HOLD_TICKS, moved < slop
                                          purr-while-pressed <--------------+      pickUp()  ---> HELD
   HELD --released--> release() --> air / chute (section 2)        HELD --Escape / pointercancel / blur--> cancel()
```

`pickUp(a)`: leave the perch (clear its claims), keep the grip at the **scruff**: `grip = (a.x, a.y - 0.9 h)` in the pet's frame, independent of where the press landed (the pet is picked up by its scruff, never by a foot); the first 8 ticks the anchor travels from the scruff to the pointer through the follow spring, which is the "lift" animation. Mood impulse `scared` 0.25 (small), `shy` for shy species (section 8).

### 1.3 The follow spring and the dangle

Anchor `A` (the grip point in the stage) follows the pointer target `T` (clamped into the stage) with the existing `springStep`, per axis, critically damped. Holden's halflife form [V round 1]: `omega = 2 ln 2 / halflife`, `k = omega^2`, `c = 2 omega`. Halflife 0.06 s gives `omega = 23.1`, **`k = 534`, `c = 46.2`**, `omega dt = 0.36` (well inside the semi-implicit Euler stability bound). At constant pointer speed the anchor lags by `2/omega` = 87 ms of travel. Halflife 0.03 s (`k = 2135`, `c = 92.4`, `omega dt = 0.72`) is stiffer; 0.10 s (`k = 192`, `c = 27.7`) is syrupy. [D]

Body: a rod pendulum under the anchor. State: `bob` (the feet) and `prev` (the feet one tick ago), rod length `Lh = 0.8 h` (38 px). **SHAKE-style step with a moving anchor** (the core of this brief, reused in sections 2 and 3), using only `+ - * / sqrt`:

```ts
/** One Verlet step of a point on a rod or rope of `length` below a moving anchor: the constraint force acts along the
 *  radius vector of the previous tick and its size is the smaller root of a quadratic (SHAKE, Ryckaert et al. 1977). */
function swingStep(a0: Point, a1: Point, bob: Point, prev: Point, length: number, gravity: number, damping: number, rope: boolean): Point {
  const ux = bob.x + (bob.x - prev.x) * damping;                       // free flight, damped
  const uy = bob.y + (bob.y - prev.y) * damping + gravity / 4096;      // gravity (px/s^2) times dt^2
  const rx = bob.x - a0.x, ry = bob.y - a0.y;                          // radius vector at tick n
  const wx = ux - a1.x, wy = uy - a1.y;                                // free position relative to the anchor at tick n+1
  const rr = rx * rx + ry * ry, wr = wx * rx + wy * ry, ww = wx * wx + wy * wy;
  if (rope && ww <= length * length) return { x: ux, y: uy };          // slack rope pulls nothing; a rod always does
  const lambda = (wr - Math.sqrt(Math.max(0, wr * wr - rr * (ww - length * length)))) / Math.max(rr, 1e-9);
  return { x: ux - lambda * rx, y: uy - lambda * ry };                  // |new - a1| = length exactly
}
```

Why SHAKE and not the usual projection (`p' = a + (p - a) L / |p - a|`, "position-based dynamics" / Jakobsen with one relaxation pass)? Measured against an RK4 reference of `theta'' = -(g/L) sin theta` at `dt = 1/64`, `L = 100`, 45 degrees start [P]:

| scheme | angle after 0.75 s (reference -44.853) | peak of the first half swing (start 45) | after 30 s |
|---|---|---|---|
| Verlet + radial projection, `dt = 1/64` | -38.80 | 38.97 (24 % of the energy lost) | amplitude 0.6 degrees |
| same, `dt = 1/128`, `1/256`, `1/1024` | -41.66, -43.21, -44.43 | | error halves with `dt`: first order |
| SHAKE (above) at `dt = 1/64` | -44.857 | | amplitude drift < 0.02 degrees over 60 s |

The projection scheme is implicit Euler for the constraint force, hence dissipative. SHAKE is second order and symplectic; a small explicit `damping` (per tick factor on the velocity) then gives full control of how fast a swing dies. [P]

Starting values (`h = 48`): `Lh = 0.8 h = 38`; **`gravity_held = 4 x GRAVITY = 7200 px/s^2`** (period `2 pi sqrt(Lh / g) = 0.46 s`); `damping = 0.95` per tick (half-life 13.5 ticks); cone: after the step, if `dy = bob.y - a1.y < Lh cos 60 degrees = 0.5 Lh`, set `dy = 0.5 Lh` and `dx = copysign(sqrt(Lh^2 - dy^2), dx)` (one `sqrt`, keeps the side). Why such a large held gravity: the pointer is a "giant hand"; ordinary drags accelerate at 1 000 to 30 000 px/s^2 (the peak of a minimum-jerk move is `5.77 d / T^2`: 1 150 for 200 px in 1 s, 4 300 for 120 px in 0.4 s, 6 900 for 300 px in 0.5 s, 28 000 for a 600 px flick in 0.35 s), compared with `GRAVITY = 1800`. With the real value the pet leans against the cone for every drag. In the simulation (minimum-jerk drags, anchor halflife 0.06) the peak lean for 120 px / 0.4 s, 300 px / 0.5 s, 600 px / 0.35 s and 200 px / 1.0 s was 40 to 51, 60, 60 and 14 to 24 degrees with `gscale = 3 to 5`, and the swing settled below 3 degrees after 1.1 to 2.3 s [P]. Tune `gscale` (3 to 6) and the cone by eye.

Rig mapping without trigonometry: `u = (bob - a1) / Lh` is the unit axis of the body (pointing from the scruff to the feet), `sin(phi) = u.x`, `cos(phi) = u.y`. Frame fields `tilt` (in turns, from `atanTurns(u.x, u.y)`) and `pivot = (0, -0.9 h)` (the scruff in feet coordinates) let a target draw `translate(feet) rotate(tilt about pivot)`; the feet position handed out is `bob`. Limbs get an additive spring lag (`springStep` per limb, target `-0.6 tilt`) for the follow-through; the face shows wide eyes and a small open mouth (mood `scared` 0.25 or `playful` 0.25 per species).

### 1.4 What the pet does to the others while held

- Held pets are **solid**. The follow target is not allowed to put the pet's box (stand box widened by `0.25 h` per side for the lean) into another solid body or into an active claim: iterate at most 4 times the minimum translation vector out of each overlapping obstacle, then re-check; if still overlapping, keep the previous position. **Shift anchor and bob together** by the vector so the rod stays consistent; the spring then pulls the anchor back toward the pointer each tick, which feels like bumping into the neighbour while the cursor goes deeper. Without this the held pets overlapped others in 41 673 of 180 000 ticks in the prototype [P] (section 6.6).
- Neighbours within 1.5 widths of the pet's projected drop column turn their heads to watch (gaze only; no collision response).

### 1.5 Release velocity

Keep a ring of the last 7 grip targets, one per tick (sample-and-hold of the latest `dragged` position; the core does the sampling, so it depends only on the event log). Least-squares quadratic velocity at the newest sample, exact integer weights (computed with rational arithmetic here [P]; they are the Savitzky-Golay edge-derivative weights [M]): oldest to newest

| N (ticks) | window | weights (divide by) |
|---|---|---|
| 5 | 78 ms | `[26, -27, -40, -13, 54] / 70` |
| 6 | 94 ms | `[85, -49, -108, -92, -1, 165] / 280` |
| **7** | **109 ms** | **`[7, -2, -7, -8, -5, 2, 13] / 28`** |
| 8 | 125 ms | `[35, -3, -27, -37, -33, -15, 17, 63] / 168` |

`v_ring = 64 * sum(w_i * p_i)` px/s per axis. Linear fits for comparison: N = 6 `[-5,-3,-1,1,3,5]/35`, N = 8 `[-7,-5,-3,-1,1,3,5,7]/84`. In a synthetic experiment (smoothstep flicks of 150 and 400 px over 0.15 to 0.4 s, events at 30, 60 or 120 Hz with 4 ms jitter, 1 px quantisation, held to 64 Hz ticks, 200 trials per cell) the mean combined bias plus spread, normalised by `A/D`, was for 60 Hz: two-point difference 0.57, LSQ1(6) 0.58, LSQ1(8) 0.69, LSQ2(6) 0.43, **LSQ2(7) 0.42**, LSQ2(8) 0.45; for 120 Hz 0.39 / 0.54 / 0.68 / 0.34 / **0.35** / 0.38; for 30 Hz everything is poor (1.26 for the two-point difference, 0.52 to 0.59 for LSQ2) [P]. Conclusion: LSQ2 over 6 or 7 ticks (they are within noise of each other; 7 is used below); at 30 Hz the shell should deliver coalesced samples (`PointerEvent.getCoalescedEvents()` [M, support not re-verified]) or interpolate them onto ticks.

Throw: `v_rel = v_bob + 0.5 (v_ring - v_anchor)` where `v_bob = (bob - prev) * 64` and `v_anchor` is the spring velocity (the part of the hand motion the follow spring has not yet delivered) [D]. Stale rule: if the last 3 ring samples are identical the pointer stopped, `v_ring = 0` (Android: 40 ms). Limits [D]: speed below `V_DROP_MIN = 70 px/s` means "just let go" (zero velocity); above `V_THROW_MAX = 640 px/s` scale down; upward component at most `520 px/s`. Consequences with `GRAVITY = 1800`: a vertical 640 px/s throw rises `640^2 / 3600 = 114 px` (2.4 h), a 45-degree throw travels `v^2 / g = 227 px`. Tumbling: tilt keeps its angular velocity for the first 10 ticks of flight then springs to zero (`k = 300`, `c = 30`).

Edge cases: `pointerleave` of the window while captured is a release with the ring as it is; `pointercancel`, `lostpointercapture`, window `blur`, `visibilitychange` and `hushed{quiet: true}` are cancels (zero velocity). **Escape** is a cancel with undo: glide back to the origin perch spot if `hopOf` succeeds and the spot is free (claims), else drop with zero velocity. Never a teleport.

### 1.6 What "too high" means

Defined in section 2.1 by the predicted impact speed (not by height alone), so that thrown pets and pets dropped from rest follow one rule.

---

## 2. Parachute

### 2.1 Trigger

Per tick while the actor is in `air` with `vy > 0` and not in a planned hop: let `H` = height of the first thing below the feet at the pet's x (the landing perch from `landingOf`, or the head of another pet), then

```
vHit = sqrt(vy^2 + 2 * GRAVITY * H)                  // impact speed without a chute
open when  vHit > V_CHUTE (600 px/s)  and  vy >= V_OPEN (240 px/s)  and  H >= H_MIN (56 px)
```

From rest `vHit > 600` means `H > 100 px`, about two body heights (`2.1 h` at `h = 48`); for a thrown pet the same rule holds with its initial `vy`. Written for scaling: `H_CHUTE = 2.0 h`. `V_OPEN = 240` is reached after 0.13 s (16 px) of free fall, so a pet that is merely dropped from a low perch never flashes a canopy. `vHit <= 600` (or `H < H_MIN`): plain landing with the existing `land` squash; **no damage anywhere in the game**. If `vHit > 600` but `H < H_MIN` (thrown downward at the floor): `brace`, a 0.5 s stagger with stars (no harm). Floaters (`gait: float`) do not fall at all; dropped, they sink at `vt = 0.6 h` px/s with no canopy.

Lemmings as prior art [S: https://lemmings.fandom.com/wiki/Floater; V-sum: https://giovanniviglietta.com/files/lemmings/Glitches.html]: without the umbrella a lemming dies from a fall of about 60 px or more (a single pixel decides between walking away and splatting; the Windows port's safe distance is 66 where the design says 63), with the umbrella it survives any height. Our variant replaces death by a soft-landing animation and moves the threshold to where the impact would look violent.

### 2.2 Descent

Timeline in ticks after the trigger: reflex `D = 6` (look up, tug the cord: anticipation), then inflation `I = 22`, then steady descent, then flare, touch, collapse.

```
after the reflex:     vy <- vt + (vy - vt) * F          F = exp(-1/(64 tau)) = 0.9168553557320289   (tau = 0.18 s, literal)
steady speed:         vt = 2.0 h px/s (96 px/s for h = 48);  lighter species 1.6 h, heavier 2.4 h
horizontal steering:  vxDes = clamp(kappa * (xTarget - x), -Vs, Vs),  kappa = 1.2 per s, Vs = 1.2 h px/s (56)
                      vx <- vx + (vxDes - vx) * 0.06                 per tick
wind (deterministic): wx = 10 * sinTurns(0.35 * tau_total + phi_actor)  px/s added to the canopy's drift
canopy anchor moves with (vx + wx, vy); the pet hangs below it via swingStep (rope = false, Lh = 0.9 h,
   gravity 0.5 * GRAVITY, damping 0.97); the sway is the pendulum answering steering and wind.
```

Why not a drag law `a = g - (g/vt^2) v |v|`? With `v = 600`, `vt = 96` the first tick would decelerate by `1800 * (39 - 1)` px/s^2, i.e. more than `v` per tick, so explicit integration is unstable; the exponential approach is exact for the linear-drag case, stable for every `v`, one multiply per tick. [P]

Measured impact speeds (px/s), `GRAVITY = 1800`, `V_OPEN = 240`, `D = 6`, `tau = 0.18`, `vt = 96`, flare ramp over the last 12 px [P]:

| drop H (px) | 36 | 50 | 80 | 100 | 150 | 200 | 300 | 500 |
|---|---|---|---|---|---|---|---|---|
| no chute | 360 | 424 | 537 | 600 | 735 | 849 | 900 | 900 |
| chute | 366 | 422 | 307 | 233 | 118 | 98 | 48 | 50 |

(Below 50 px the chute cannot help; 80 px is partially braked.) The smallest drop for which the chute yields an impact of at most 250 px/s is 95 px for `D = 6`, `tau = 0.18`, 155 px for `D = 10`, 70 px for `V_OPEN = 180`, 125 px for `V_OPEN = 300`; the reflex delay is therefore the most sensitive parameter: keep it at 6 ticks or less. Altitude used before the speed is under control from rest: about 200 px (`4 h`).

### 2.3 Landing spot

At the trigger tick pick the landing target: candidates are the free spots (claims, section 6) on the perches below within the horizontal reach `R = Vs * H / vt` (for `H = 200`: `56 * 2.1 = 117 px`), nearest to the current x first, preferring the perch the pointer was released over; reserve the landing box (`x +- (w/2 + 4)`, `[t_land - 8, t_land + 64]`) and steer to it. Re-target at most every 16 ticks when the reservation is lost. When nothing is free the pet keeps sinking onto a head (section 6.3) rather than hovering.

### 2.4 Flare, touch, collapse (no squash clip)

```
rem = H - travelled
flare (rem < 0.25 h = 12 px):   vy <- vt * (0.5 + 0.5 * rem / flare)          // linear ramp to half speed
                                legs extend (pose blend over 8 ticks), canopy tilts back 14 degrees, canopy scaleY -> 0.8
touch:   clip "touch" instead of "land": 3 % squash for 6 ticks (spring, k = 600, c = 24), no impact particles
collapse: canopy scaleY -> 0 over 18 ticks (critically damped, omega = 16), x-shear +-8 degrees, then the canopy
         is a ribbon lying on the perch for 0.6 s and fades (or retracts into the pack over 20 ticks)
```

### 2.5 Making it read with few bones

Four bones are enough: `pack` (child of the pet's root at the shoulders), `canopy` (child of `pack`, 0.9 h above), `lineL`, `lineR` (anchors of the two cords; the cords are `line` shapes between the canopy edge and the shoulders, redrawn every frame by the target). What sells it, in order of cost [M unless marked]:

1. **Anticipation and overshoot** (Disney principles, round 1): the 6-tick tug, then canopy `scaleY` 0 to 1.25 to 1.0 with a damped spring (`omega = 14.8` , `zeta = 0.4`: `k = 220`, `c = 11.9`, overshoot 25 %), `scaleX = 1 - 0.25 (scaleY - 1)` (area-preserving squash and stretch).
2. **Phase offset** between canopy and body: the canopy follows the anchor, the body the pendulum (`swingStep` gives this for free).
3. **Cloth edge**: the lower canopy edge is a `ribbon` through 5 points with a travelling wave `y_i = 1.5 * sinTurns(0.9 * tau - 0.12 i)` px.
4. **Face**: eyes look up during the reflex, down during the flare.
5. **Limbs** swing with a delayed spring (as in section 1.3).
6. Gusts: wind impulses squash the canopy 6 % for 10 ticks.

Box with canopy open (for claims): body box united with `[x - 0.85 w, x + 0.85 w] x [y - 1.55 h, y - 0.9 h]`.

---

## 3. Grappling gun

### 3.1 Phases

```
          decide (utility; no hop / ladder / wall reaches, a hook candidate exists, cooldown ok, species ability "grapple")
ready ---------------------------------------------------------------------------------------------> aim (14 ticks)
aim:   turn to face, raise the gun (clip), claim the corridor box, final re-test of the line
fire:  event "shot" at tick t0; hook(k) = M + u * min(S k, D)  with S = 10 px/tick (640 px/s), u = (P - M) / D
       rope drawn muzzle -> hook, straight (taut) , recoil clip 6 ticks
       each tick the shell's survey may cut the line (keep-out appeared, target moved > 8 px, vanished): -> miss
attach (k = ceil(D / S)): tug 4 ticks, then
       |dx| <= 0.35 |dy|  ->  zip      reel straight up the rope: v = 150 px/s * smoothstep(k / 10)
       else               ->  swing    swingStep(rope = true) with a shortening length, release window search
       both end at distance <= 0.9 h from P: climb-last (hand over hand, 0.35 s) -> mantle onto the perch (24 ticks)
miss:  hook returns at 20 px/tick, rope whips (slack wave), shrug 40 ticks, cooldown 4 s; two misses in a row: 60 s
```

### 3.2 Reachability (decision time, all `+ - * / sqrt` and comparisons)

For each perch `Q` above the feet by at least `0.8 h` (a hop reaches `HOP_HEIGHT - HOP_CLEARANCE = 72 px` at most, so a grapple is for higher perches), candidate hook points `P`: the perch's two ends inset by `w/2 + 4`, and the point of the top edge nearest to the pet's x, each at `P.y = Q.y - 1`. Muzzle `M = feet + (facing * 0.3 w, -0.65 h)`. Accept when

1. `D = |P - M|` within `[L_MIN, L_MAX] = [0.8 h, 3.2 h]` = [38, 154] px;
2. elevation `|dy| / D >= 0.42` (about 25 degrees: the rope must not scrape the card face);
3. the segment `M -> P` misses every keep-out inflated by 2 px (the keep-outs already contain every surface box grown sideways, design section 6.3) **except** the target surface's own, and misses the boxes of the other solid bodies (Liang-Barsky: parametric clipping in `t`, only comparisons and `/`);
4. the landing spot on `Q` (`P.x` moved inside by `w/2 + 8` toward the perch's interior) is free of claims.

Choose the candidate with the least `D + 0.3 |dx|`. A deliberate miss for charm: with probability 0.10 (a stage draw) the aim point is displaced 14 px beyond the edge, the hook flies past, and the segment test of the flight finds no contact: **a miss must be geometrically honest**, so it is the same flight code with a target that is not there. Failure by layout change (the line gets cut mid-flight) uses the same retract.

Worms' ninja rope has the same structure (distance constraint from the attach point, a winch changes the length) [S: https://www.hedgewars.org/node/2893 — read: it contains no formulas; https://create.roblox.com/docs/physics/constraints/rope for the winch idea]; rope wrapping around corners is out of scope.

### 3.3 Swing and reel with `swingStep`

Anchor `P` fixed, bob = the pet's centre of mass, rope length `L0 = D` at the attach. Reel: `L <- max(L_MIN_SWING (0.9 h), L - rho / 64)` with `rho = 60 px/s` (ramp 10 ticks). Because the constraint force of `swingStep` is central (along the radius vector), **angular momentum about the anchor is conserved while the length changes**, so shortening the rope speeds the swing up exactly like a pendulum with a shortened string. Check against the polar equation of motion with a prescribed `r(t) = L0 - s t`, `theta'' = -(g/r) sin theta - 2 (r'/r) theta'` [P]:

| case (`dt = 1/64`) | tick | reference (RK4) | SHAKE |
|---|---|---|---|
| L0 = 100, 45 degrees, `s = 60` | 16 / 32 / 48 | 20.24 / -40.69 / -67.67 | 20.22 / -40.76 / -67.64 |
| L0 = 160, 60 degrees, `s = 90` | 16 / 32 / 48 / 64 | 40.41 / -19.19 / -86.88 / -112.65 | 40.41 / -19.25 / -86.96 / -112.61 |

(degrees; agreement within 0.4 degrees until the length is down to about 0.4 of its start, after which the true motion spins up without bound: at tick 96 the reference of the first case is already at 467 degrees.) Hence the two guards that must exist in the implementation: a minimum swing length (`0.9 h`) and a tangential speed cap: after the step, `d = bob - old`; if `|d| > 520 / 64 = 8.1 px` scale `d` to that length and re-project with one more `sqrt`; plus `damping = 0.9965` (half-life 3 s) in the free flight. Without the cap the reeled swing reaches thousands of degrees per second [P].

Release window (the "Tarzan" use for crossing a gap, optional): from the attach tick simulate the same step forward up to 90 ticks; at each tick run the ballistic landing test (as `hopLanding`, with the velocity `(bob - prev) * 64`) and accept the first tick whose landing perch is free and whose `vHit <= 600` (or a parachute will open). This is a deterministic search of at most `90 x 48` steps at decision time. If none succeeds the swing is only a flourish ending in the climb.

Rope drawing: `ToolFrame` kind `rope` with `x0, y0` (muzzle), `x1, y1` (hook), `slack` in px; taut = straight line; `slack > 0` (flight start, retract, whip) = quadratic curve with the control point offset by `slack * sinTurns(2.5 * age)` along the normal.

### 3.4 Starting values (h = 48)

| name | value | note |
|---|---|---|
| hook speed `S` | 10 px/tick | 640 px/s; a 150 px shot takes 15 ticks (0.23 s) |
| retract speed | 20 px/tick | |
| `L_MIN`, `L_MAX` | 0.8 h, 3.2 h | 38, 154 px |
| aim, recoil, tug | 14, 6, 4 ticks | clips |
| zip speed | 150 px/s, ease 10 ticks | |
| reel in swing | 60 px/s | tangential cap 520 px/s, damping 0.9965 |
| climb-last, mantle | 0.35 s, 24 ticks | |
| miss chance, cooldown | 0.10, 4 s (60 s after two misses) | draw from the actor's stream |

---

## 4. Ladder

### 4.1 Geometry and placement

A ladder leans against a **wall** (the vertical edge of the upper box; section 5.1 adds `walls` to the survey) with its base on a lower perch. Let the wall be at `x = wx`, its top edge at `y_t` (the upper surface), the base point `B = (bx, by)` on the lower perch:

```
T  = (wx, y_t + 0.15 h)               contact point just under the rim; the rails extend 0.3 h beyond T along the axis
H  = by - T.y                          vertical rise, must be in [0.8 h, 3.6 h] = [38, 173] px
dx = |bx - wx| = r * H                 lean ratio r in [0.14, 0.40], default 0.25 (4 : 1, angle from vertical atan(0.25) = 14 degrees)
len = sqrt(H^2 + dx^2),  n_rungs = floor(len / (0.22 h)),  rung spacing 0.22 h = 10.5 px
```

Why `0.22 h` and why `r = 0.25`: real rungs are 0.305 m apart and people climb vertical ladders at about 0.4 m/s (a search summary of ladder-climbing studies, 305 mm vs 356 mm spacing, 0.38 to 0.43 m/s; the exact paper was not identified: [S] https://www.researchgate.net/publication/15498181_Biomechanical_analysis_in_ladder_climbing_the_effect_of_slant_angle_and_climbing_speed), i.e. 0.17 of a person's height, 1.3 rungs/s; the 4:1 setting rule for real ladders is [M]. A pet that climbs at 1.3 rungs/s would crawl (0.22 h x 1.3 = 14 px/s), so we use 2 to 3 rungs/s as a cartoon speed-up [D].

Placement test (decision): the base footprint `[bx - 10, bx + 10]` on the perch free of claims and keep-outs; the capsule of half-width 6 px around the segment `B -> T` misses all keep-outs and solid bodies (the wall's own rectangle excluded); the strip between base and wall on the lower perch becomes a **reserved zone** (nobody stands under a ladder, and nobody is placed there by the 1D seating of section 6). The ladder is chosen over a grapple when the wall exists and `H <= 3.6 h`, over a hop when the rise exceeds 72 px; species ability `ladder`.

### 4.2 The ladder as a stage object

```
Ladder { id, owner: speciesIndex, base: Point, top: Point, lowPerch: string, wall: string,
         state: "raising" | "standing" | "toppling", since: Ticks, lastUsed: Ticks,
         occupant: speciesIndex | -1, queue: speciesIndex[] (at most 2) }
```

- `raising` (owner animation `place`, 30 ticks: the ladder rotates up from lying on the perch about the base, `angle` from 90 degrees to `atan(r)`); then `standing`.
- Lifetime: renewed by every use; removed `LADDER_IDLE = 1280` ticks (20 s) after the last use and at most `LADDER_MAX = 7680` (120 s) after standing; removal = the owner walks to the base and lowers it (24 ticks) if idle and present, else the ladder folds and fades (16 ticks). At most 2 ladders on the stage, at most 1 per surface pair; none in `quiet` or `still`.
- Others may use it: it is an edge `lowPerch -> upperPerch` of the walking graph with cost `len / 26`; one occupant at a time, a second climber waits at distance `>= 0.7 w` from the base (a queue slot), never passes (no swapping on a ladder).

### 4.3 Climbing gait

Phase is driven by **distance**, not time, so hands and feet never slide on the rungs: `s` = distance climbed along the ladder, `phase = (s / (2 * rungSpacing)) mod 1` (two rungs, left hand and right hand alternating, is one cycle: `frac(s/21)` for 10.5 px rungs), `phase` selects the pose of a looping climb clip (`sampleClip` with `ticks = phase * clipTicks`). Speed up 26 px/s (2.5 rungs/s, 0.54 h/s), down 34 px/s, acceleration over 6 ticks. Mount: walk to within 2 px of the base facing it, `mount` 8 ticks; dismount: when `s >= len - 0.4 h`, `mantle` onto the perch at `P.x +- (w/2 + 8)` over 24 ticks (same path as section 5.3). Box while climbing: `(0.7 w, h)` around the axis.

### 4.4 Surfaces that move or vanish

Each `surveyed` re-solves the top: new `T'` and new `H', r'`. If `|T' - T| <= 8 px` and `H', r'` are in range and the line is clear, the top **follows** (critically damped spring, halflife 0.1 s) while the base stays; else the ladder `topples`: rotation about the base from `atan(r)` to 90 degrees away from the wall (spring `k = 150`, `c = 24`, about 20 ticks), lies flat for 10 ticks, then vanishes in a dust burst. A rider at rail height `s` is thrown clear: vertical rise above the perch `< 0.5 h` steps off; otherwise `fall` with `vx = +-70 px/s` away from the wall and the parachute rule of section 2 (a rider at 120 px opens a canopy and lands softly). If the lower perch vanishes or moves more than 12 px, the ladder falls the same way and the owner shows `surprised`. `reclaimed{wall}` (the user touched the element) = immediate topple without the follow tolerance.

---

## 5. Wall climbing

### 5.1 Survey addition and definitions

`surveyed` gains `walls: Wall[]`: `{ id, x, y0, y1, side: -1 | 1 }`, the left (air on the left, `side = -1`) and right (`+1`) vertical edges of every surface's element, from the top edge `y0` down to `y1 = min(bottom, stage floor)`. **Free stretches**: subtract from `[y0, y1]` the vertical extent of every keep-out that intersects the strip `[x - (w + 8), x]` (mirrored for `side = +1`) and keep stretches at least `1.5 h` long (the same interval subtraction `perchesOf` uses on x). Walls of `[inert]`, `[hidden]` elements and elements being nudged are excluded; the viewport edge is not a wall.

### 5.2 Entering, climbing, sliding

- **From a perch at the wall's foot**: the perch's end is within 6 px of the wall's x, the pet faces the wall, the free stretch above the perch reaches `1.5 h`; `grab` clip 8 ticks, then climb. **From the top corner** (a pet on the upper surface at its end): turn, `hang` over the edge (feet over, 10 ticks), climb down the face to the perch below or slide.
- Speeds [D], anchored on Celeste's `Player.cs` [V-sum: https://gist.github.com/Alexandria/bf563fd51aeb3ddd31f754ea9118e20f]: `ClimbUpSpeed = -45`, `ClimbDownSpeed = 80`, `ClimbSlipSpeed = 30` px/s, `ClimbMaxStamina = 110` with `ClimbUpCost = 100/2.2` per second and `ClimbStillCost = 100/10` per second, `ClimbHopY = -120`, `ClimbHopX = 100` for 0.2 s over the top corner, `ClimbAccel = 900`; Shimeji's `ClimbWall` moves 1 to 2 px per action tick [V-sum, same actions.xml as section 1.1]. For walking at 24 to 48 px/s: **up 30 px/s (0.62 h/s), down 40 px/s, slip start 30 px/s with acceleration 600 px/s^2 up to 160 px/s**; gait phase from distance with grip spacing `0.3 h`.
- **Grip budget** (species stamina, default 384 ticks = 6 s): costs 1 per tick climbing, 0.25 per tick hanging still, regenerates 2 per tick on a perch; wall stretches are limited to `3 h` (144 px, 4.8 s of climbing) so a healthy pet always reaches the top; at 0 the pet **slips**: slides down (sliding is a controlled `slide` clip while the grip is above 0, a `slip` otherwise) onto the perch below if `landingOf` finds one at that x within reach, else falls (parachute rule).
- A pet on a wall occupies the air side column `[x_w - (0.7 w + 2), x_w]` (mirrored): a vertical lane; one climber per stretch unless two are `h + 8` apart.

### 5.3 Rounding the corner (mantle)

When the hand point (`feet.y - 0.9 h`) passes the top edge (`feet.y <= y0 + 0.85 h`): 28 ticks, `s = k / 28`, start `S = (x_w - w/2 (side), y0 + 0.85 h)`, end `E = (x_w + side_in * (w/2 + 8), y0)`. Two-phase path, all easing is the existing `smoothstep`:

```
rise   = smoothstep(clamp(s / 0.65, 0, 1))        // up past the edge
across = smoothstep(clamp((s - 0.35) / 0.65, 0, 1))
x = S.x + (E.x - S.x) * across
y = S.y + (E.y - 0.1 h - S.y) * rise + 0.1 h * smoothstep(clamp((s - 0.65) / 0.35, 0, 1))   // settle onto the perch
```

with the clip `mantle` (arms push, body rolls over: rotation 0 to -25 to 0 degrees). The target spot must be free (claim made at the start of the last climb metre; if it is not free the pet waits hanging, spending grip).

### 5.4 When the wall moves, vanishes, or is taken back

`|dx| <= 6 px` between surveys: follow (spring). Greater, or vanished, or `reclaimed{surface}`: `grip = 0` immediately, the pet is **thrown out** away from the wall with `vx = side * 140 px/s`, `vy = -160 px/s`, a surprised face, then `air` (parachute rule, section 2). Species with `gait: float` never climb: they change altitude lanes (section 6.5).

---

## 6. Guaranteed non-overlap

### 6.1 Definitions

- **Box** of an actor: `[x - w/2 - m, x + w/2 + m] x [y - H_total - m, y + m]` with `m = COMFORT_GAP / 2 = 4` (so two boxes that just touch are 8 px apart; this is exactly the existing "comfortable gap"), `H_total = h + hover` plus held props: posture boxes in the species data (`stand`, `climb`, `hang`, `chute`): chute = body united with the canopy rectangle (section 2.5); hanging = width `w + 0.5 h * |sin(tilt)|`; aiming = plus 0.35 w toward the gun; climbing = `(0.7 w, h)`. Ropes and ladders are props drawn behind the pets; only their owners/riders are bodies. A shot is validated against other bodies at fire time (segment versus boxes).
- **Solid**: every body is solid except while `ghost` (translucent, intangible) or fading in or out.
- **Invariant INV**: at the end of every `advance` tick, for any two solid actors the open boxes are disjoint (margin 0), and for any two grounded actors on one perch the order by x never changes except by an explicit hop over.
- Strong form used for planning: margin `m = 4` (gap 8).

### 6.2 Why the usual crowd methods are not enough, and what to borrow

- Boids, social forces and ORCA give soft separation without a guarantee; fine for flavour, not for INV. ORCA's velocity obstacles need a linear program per agent and are overkill for 1D lanes [M].
- **Multi-agent path finding** [V on the core ideas: D. Silver, Cooperative Pathfinding, AIIDE 2005, https://cdn.aaai.org/ojs/18726/18726-52-22369-1-10-20210928.pdf]: *Cooperative A\** plans the agents one after another in space-time (two spatial dimensions plus time); after each route is found its cells are marked in a **reservation table**, and later agents treat marked cells as impassable; a `wait` action is part of every agent's action set. The paper stresses that **agents must keep cooperating after reaching their destination** (an agent parked in a corridor blocks the others), that results are sensitive to agent order, that a fixed order fails on some instances (its Figure 1 is a corridor with a side pocket) and that **dynamic priorities** (every agent gets the highest priority for a short period) solve cases a fixed order cannot (*Windowed Hierarchical Cooperative A\**, window `w`, searches staggered over time). Conflict types of the MAPF literature [S: Stern et al., "Multi-Agent Pathfinding: Definitions, Variants, and Benchmarks", SoCS 2019, https://ojs.aaai.org/index.php/SOCS/article/download/18510/18301/22026 ; definitions seen in a search summary]: *vertex* (same place at the same time step), *edge/swapping* (two agents trade places), *following* (an agent enters a place just vacated). Our world is continuous and small, so we reuse the **ideas**: a reservation table in space-time (claims), priority by order with aging, parked agents that yield, and the explicit swap case.
- Car-following models give the 1D rule: Gipps' "safe speed" is the largest speed from which the follower can still stop behind a leader that brakes at its maximum rate [S: https://www.researchgate.net/publication/31412592_An_analysis_of_Gipps'_car-following_model_of_highway_traffic]; with constant deceleration `a_b` and gap `g`: `v_safe = sqrt(2 a_b g)`.

### 6.3 Mechanisms, by situation

| situation | rule | guarantee |
|---|---|---|
| **grounded walker, same perch** | per tick `step = min(strideStep, vSafe(gap) / 64, gapToNeighbour - GAP_MIN)` (the `vSafe` term is for smooth braking, [D], not in the prototype; the clamp is the guarantee) ; the box at the new x is checked against every other solid body **and against the swept corridors of planned movers (rest of the plan, not just the next tick)**; blocked = stay | no entry into a body, no entry into a flight path |
| **1D order on a perch** | order by x is a state invariant; no passing | no swap conflict by construction |
| **swap by hop** | allowed only if a hop clears the neighbour's top by `GAP` (`rise >= h_neighbour + 4`, so the neighbour must be at most 68 px tall for `HOP_HEIGHT = 84`), the landing beyond the neighbour is free, and the corridor is claimed; else the lower-priority pet turns round or waits; after 40 ticks of mutual blocking both pick new goals | no overlap in flight (claimed), no deadlock (timeouts) |
| **hop, ladder, wall, rope, chute** | plan first: compute the whole path (deterministic from the decision state), test the swept boxes per tick (time aware) against all other bodies' positions at those ticks and the **final spot against everyone's future positions** (the hop planned earlier may arrive after me); delay up to 32 ticks or choose another target | claims = the plan itself |
| **fall** (perch vanished, release, slide-off) | plan the fall at once: (1) try the straight fall; (2) **steer** with air control `dvx` in `{+-30, 60, 90, 130, 180, 240}` px/s, nearest first; (3) **land on a head**: other pets' tops are one-way platforms (landing is detected like a perch crossing: the faller's feet cross the top of a box whose x range overlaps); the faller stands on the head and **slides off** at 32 to 128 px/s (acceleration `0.03 px/tick^2`, reverses at a stage edge or a blocked side, falls again when clear), checking support every tick (host gone: falls again); (4) only if the path crosses a body from the side and nothing above works: **ghost** | no interpenetration; ghosts are rare and bounded |
| **held** | projection out of obstacles (section 1.4) | INV holds for held pets |
| **survey** | see 6.4 | |
| **spawn** | only at a free place (or wait) | |
| **mode `still`** | freeze in place: valid, nothing moves | |

Priority and fairness: the stage processes actors in a rotating order (`start = tick mod n`) so nobody always wins; claims are first-come-first-served; a waiting actor gains priority by waiting (`waited` ticks) when two plans conflict at the same tick [D, from WHCA*'s dynamic priorities; not in the prototype]. Parked pets yield when a higher-priority plan needs their box: a `yield` request makes a standing pet `scoot` (guarded walk at 1.5x speed) out of the corridor before the plan starts (the mover waits until the corridor is clear).

**Ghost** (intangible, opacity 0.45, trailing particles): the faller's path crosses a body from the side and neither steering nor a head landing is possible. It continues to the landing perch, **then slides intangibly to the nearest free spot** (search along the perch in 2 px steps against the corridors of everyone), becomes solid only there; if no free spot exists the pet is **crowded out** (fades where it stands and arrives anew, as today). Ghost time is bounded (`<= 128` ticks) and counted as a quality metric.

### 6.4 Resolving overlaps created from outside

1. **A surface shrinks or moves** (`surveyed`): riders keep their offset from the surface (existing behaviour: `ride`), then the group of one perch is **re-seated by isotonic regression** and the move executed as a guarded scoot (never a teleport). Bodies of one perch, sorted by x, with widths `w_i` and required centre spacing `s_i = (w_i + w_{i+1})/2 + GAP`; define the offsets `o_1 = w_1/2`, `o_{i+1} = o_i + s_i`; with `q_i = x_i - o_i` the constraint "no overlap and order kept" becomes "`q` non-decreasing and inside `[x0, x1 - span]`", so the minimum-movement (least squares) seating is the isotonic regression of `q` followed by clamping: pool-adjacent-violators, `O(n)`, only `+ - * /` and comparisons:

```ts
function seat(bodies /* sorted by x */, perch): number[] {          // returns target centres; bodies that do not fit are dropped first (lowest priority first)
  // offsets o[i]; q[i] = x[i] - o[i]
  const blocks: { sum: number; n: number }[] = [];
  for (const v of q) {
    blocks.push({ sum: v, n: 1 });
    while (blocks.length > 1 && blocks.at(-2)!.sum / blocks.at(-2)!.n > blocks.at(-1)!.sum / blocks.at(-1)!.n) {
      const b = blocks.pop()!; blocks.at(-1)!.sum += b.sum; blocks.at(-1)!.n += b.n;
    }
  }
  // expand blocks to per-body means, clamp into [perch.x0, perch.x1 - span], add o[i]
}
```

Third-party check: `sklearn.isotonic.IsotonicRegression` (L2 isotonic regression with `y_min`/`y_max` bounds) gives the same targets [M]. **Linear interpolation between two valid seatings is itself valid** (the constraints are linear, so any convex combination of two feasible configurations is feasible); therefore a coordinated scoot where every body moves the same fraction `tau` of its displacement per tick cannot create an overlap inside the group. Bodies that do not fit leave: they step off the end and fall (rules above) or are crowded out.
2. **A surface jumps** (a layout shift moves a perch by more than the guarded scoot can follow): riders that would now overlap a body that did not move become **ghost** and slide to the nearest free spot; this was needed in the prototype (a jump of 60 px carried a walker into a neighbour) [P].
3. **The user drops a pet onto another** or throws it into one: the faller's plan finds the head (rule 3 above), lands on it, slides off. Thrown sideways into a standing pet: the flight is clipped to the top of the other's box (`rise` computed so the feet clear `top - 4`; an automatic hurdle), else it turns back with half the speed (a "bonk", no overlap).
4. **Spawn onto an occupied place**: wait (retried every second, as today).
5. **Mode change to `still`** and **quiet**: freeze in place.

### 6.5 Floaters and altitude lanes

Floaters (`sun`, `cloud`) hover `hover` px above their perch; their box is taller (`h + hover`). On a perch they obey the 1D order like walkers. When a floater glides between perches along a straight line (their "hop") it claims the swept corridor like a hopper; to avoid waiting on everything it has two lanes: its normal altitude and `+0.9 (h + 4)` higher; the planner tries the normal lane, then the high lane, then waits. Floaters never pass over a grounded pet's box (the vertical ranges would overlap).

### 6.6 How to prove it (and what was measured)

Proof obligations, in the order they are discharged:

- **P1 (guarded transitions)**: every transition into a solid state (landing, spawn, ghost becoming solid, hop landing, slide start) checks `free(box, others, claims)`. **P2 (guarded motion)**: every solid motion is validated per tick against bodies and the remaining swept corridors of planned movers, or is a plan that was vetted against the same at its start including its final spot against others' future positions. **P3 (external changes use the same machinery)**: surveys do not move bodies directly; they re-seat through guarded scoots or start planned falls. **P4 (liveness)**: waits have timeouts (40 ticks, then new goals), ghosts end in a free spot or a crowd-out.
- **Executable invariant**: `overlaps(stage): Pair[]` = all pairs of solid actors whose boxes intersect (`a.x0 < b.x1 && b.x0 < a.x1 && a.y0 < b.y1 && b.y0 < a.y1`, strict), asserted after **every** `advance` in the test harness and in `frameOf` debug builds; plus the order invariant for grounded pairs. Margin 0 is the hard check; margin 4 is counted as a near-miss metric.
- **Fuzz**: random traces of `surveyed` (shrink, grow, move, vanish, appear, jump), `summoned`, `tuned`, `hushed`, `pressed`/`dragged`/`released` (random pet, random path including over other pets, random velocities), `played`, in random order within a tick; 100 000 ticks, 6 to 10 actors, species sizes 28 to 56 px, many seeds; property-based tooling with shrinking: `fast-check` for TypeScript and `proptest` for Rust [M] so a failure reduces to a minimal event list; the TS and Rust traces are compared by per-tick hashes (existing practice), and the hash includes the `ghost` flags.
- **Small-scope exhaustive check** [M, the small-scope hypothesis]: 3 actors, 2 perches, positions on a 4 px grid, all state combinations, breadth-first over the planner's decisions, to find reachable states from which no overlap-free continuation exists (a "trap"); trivial to run in Rust.
- **Quality metrics to publish per run**: overlap ticks (must be 0), near-miss ticks (margin 4), ghost ticks and ghost events per million ticks, crowd-outs, mean waiting ticks.

Prototype evidence [P] (Python, not in the repository; model: 8 agents of widths 28 to 56 and heights 40 to 56 on a 960 x 540 stage with 7 perches; walking, planned hops, falls, head landing and slide, steering, ghost, PAVA scoot; user chaos: a pet is picked up on average every 120 ticks and thrown or dropped, a survey every 300 ticks changes one perch: delete, shrink, move 60 px, add; update order rotating; invariant checked after every tick). Result of the full rule set: **0 overlapping ticks in 24 seeds x 30 000 ticks x 8 actors (5.76 million actor-ticks)**; ghosts were 0.17 % of the actor-ticks (253 ghost events, one per about 2 850 ticks of the whole world), plus 3 819 head landings, 331 steered falls and 50 crowd-outs (all respawned). Ablations, each over 12 seeds x 15 000 ticks (180 000 world ticks): no walker corridor guard 96 097 overlapping ticks; no hop planning check 34 672; no held projection 41 673; no ghost fallback 4 580; no head platforms 0 overlaps but 18 721 ghost ticks instead of 2 831 (6.6 x more ghosting); no steering 0 overlaps and 6 759 ghost ticks (2.4 x); no PAVA 0 overlaps and 3 376 ghost ticks (the ghost repair covers the gap, PAVA mainly saves visible disruption: 19 % fewer ghost ticks here). The Gipps-style speed cap and the "waited ticks raise priority" rule of 6.3 were **not** in the prototype (walkers there simply stay when blocked). Eight bugs were found by running it and are exactly the traps to design against: a landing x clamped to the perch end (a teleport) into a neighbour; a rigid ride on a surface that jumped (a teleport) into a body that did not move; a mover that ignored the corridor of a faller whose plan had not started yet (include the current box as well as the rest of the plan); a faller's final spot not checked against the later arrival of an earlier hop; a ghost flag lost when a plan was installed; a ghost that landed on a head and became solid without a free-spot test; a head-landed pet left hanging when its host walked away (re-check support every tick); a slide beyond the stage edge followed by a clamping teleport. Caveats: 2D axis-aligned boxes, a toy world, no rope, ladder or wall, sequential update. The rules are the contribution, not the numbers.

---

## 7. Gesture recognition without trigonometry

All gesture state is per actor and per pointer, advanced once per tick on the sample-and-hold pointer position relative to the actor (`r = pointer - centre`, centre = feet + (0, -h/2)), inside the core. **Common gates** (every gesture): primary pointer; no other element under the pointer is interactive (`overInteractive` from the shell); no text selection in progress; no wheel or scroll in the last 16 ticks (`scrolled{}` resets all); not quiet, not `still`; the actor is idle, walking or sleeping-aware (a trick does not interrupt a ladder climb).

### 7.1 Click, long press, drag, multi-click

Thresholds (tick = 15.6 ms):

| name | value | basis |
|---|---|---|
| `SLOP` mouse and pen | 6 px | Windows `SM_CXDRAG` = pixels either side of the mouse-down point before a drag begins [V: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics ; default value not stated there]; GTK / macOS comparable [M] |
| `SLOP` touch | 10 px | Android touch slop 8 dp [V-sum] plus finger jitter |
| `HOLD_TICKS` | 28 (0.44 s) | Android long-press timeout 400 ms [V-sum]; WCAG 2.5.2 wants no execution on the down event, so hold = purr loop that ends on release |
| click | release before `HOLD_TICKS` with movement `< SLOP` | |
| chain gap | 48 ticks (0.75 s) and 12 px | Windows default double-click time 500 ms [V: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setdoubleclicktime], Android 300 ms [V-sum]; slower here because these are not text double clicks |

Escalation is decided by the leaky `heat` of section 8.4 (one model, not two): each click on the same actor within the chain gap adds 1 and `heat` leaks 0.5 per second, so for a steady clicker (one click per 0.5 s, +0.75 net per click) the tiers come out as: click 1 hello (the existing greet), clicks 2 to 3 the species trick, clicks 4 to 6 purr (a loop that lasts while clicks keep coming and 1 s after), clicks 7 to 8 giggle, from click 9 "enough". A click after a pause longer than the chain gap starts again near hello because `heat` has leaked. Drag: movement `>= SLOP` after `armed` is `pickUp` (section 1.2).

### 7.2 Circling around a pet (the sun trick)

Quadrant winding with hysteresis. Per tick, with `r = (rx, ry)` relative to the centre (y down, so increasing angle is clockwise on screen):

```
band:     RIN = 0.5 h + 8 = 32 px  <=  |r|  <=  ROUT = 3.4 h = 163 px   (squared compares); outside for > 8 ticks: reset
axis signs with a Schmitt trigger (H = 4 px):  sx = rx > H ? +1 : rx < -H ? -1 : sx   (same for sy)   [until both known: wait]
quadrant:  Q = (sx > 0) ? (sy < 0 ? 0 : 1) : (sy > 0 ? 2 : 3)      0 top-right, 1 bottom-right, 2 bottom-left, 3 top-left = clockwise order
dq = (Q - Qprev) mod 4:   1 -> +1,  3 -> -1,  2 -> ambiguous: +2 if cross > 0 else -2,  cross = prev.x * ry - prev.y * rx
q += dq;  remember the tick of each quarter step; reset q when > QMAX = 40 ticks pass without one
complete a lap when |q| >= 4 * turns:  accept iff
    span of the last 4*turns-1 steps in [QMIN * (4*turns-1), QMAX * (4*turns-1)],  QMIN = 4 ticks (4 turns/s)
    and rmax <= 3 * rmin over the lap   (circle-like, not a scribble)
    and at most 1 opposite step   (a back-and-forth is not a lap)
emit circled{ turns, direction } once, then reset; cooldown 128 ticks per actor
```

Needed turns by use: 1 = "notices" (gaze + glance), 2 = species trick (sun: rays flare, "shine"; the direction can pick `day` / `night` variants), 3 = encore. All designer data (`tricks[].trigger`).

Measured [P] (synthetic hands: ellipse axis ratio 0.6 to 1.0, random tilt, radius wobble to 15 %, 1.2 px noise, centre offset, speed 0.6 to 2.5 turns/s, events at 60 Hz held to 64 Hz ticks): **98.4 % of laps detected** for radius 50 to 130 px and centre offset up to 25 % of the radius; 89 % at 35 % offset (the band clips); 99.4 % at 2 to 3.5 turns/s; 94 % at 0.4 to 1.0 turns/s (one turn); two-turn tricks 96 % / 85 % / 99 % / 82 %. **False triggers: 0 in 8.0 pet-hours** of ordinary page travel (minimum-jerk moves to random targets with 0.8 px tremor and holds, 60 random pet centres), **0 in 0.9 h** of confined fidgeting (persistent random walks inside 220 px of the pet at 150 to 900 px/s), and **0 in 800 s** of horizontal sweeps over and under the pet at 0.5 to 3 Hz. Two lessons: a sweep through the horizontal axis with 0.8 px of noise **did** trigger (20 times in 800 s) before the Schmitt trigger and the "at most one opposite step" rule were added, because quadrant flips on the axis are a random walk that reaches +-4; and the plain "|q| >= 4" rule with a per-lap time limit measured at the first and last step had to count three intervals, not four (a bug that cut detection to 46 % at high speed).

Why this cannot fire on ordinary travel: a straight line past a point sweeps less than a half turn, i.e. at most two quarter steps; four are needed. Why it is cheap: three compares and a few adds per tick, no `atan2`.

Third-party oracle (test time): `numpy.unwrap(numpy.arctan2(ry, rx))` accumulated angle divided by `2 pi`; the integer winding must agree with `floor(|total turns| + 0.25)` on traces that stay in the band.

### 7.3 Stroking (petting)

Mouse and pen only (touch drags pick the pet up; a touch pet is petted by a long press). Zone `Z` = the stand box inflated by 10 px. Track horizontal reversals by extremes, not by velocity sign:

```
inside Z for the whole attempt (leaving resets); HYST = 0.15 w (6 px) to register a reversal from the running extreme
a segment = a monotone run between two reversals; it counts iff length >= 0.45 w (18 px) and speed in [60, 900] px/s
a segment that fails resets the chain;  chain gap > 48 ticks resets
petted{ strokes } after 3 consecutive counting segments within 96 ticks; while strokes continue each further 3 raise purr
```

Measured [P]: **94 % detected** over 1 to 4 Hz, +-14 to 28 px amplitude, 3 s of stroking (latency 0.85 s); **0 false triggers** in 13.4 pet-hours of ordinary travel crossing the zone and in 200 x 10 s of 1.5 px hand tremor. Oracle: `scipy.signal.find_peaks` on the x trace (prominence 0.45 w) counts the same segments.

### 7.4 Shaking while held

On the held grip velocity (per-tick differences, px/s): a reversal is `dot(v, v_ref) < 0` with `|v| > 40` and the distance from the last extreme `>= AMP = 36 px` and the peak speed since then `>= VPEAK = 300 px/s`; `NEED = 4` reversals within `WINDOW = 64` ticks emit `shaken{ level }`, `level` rising with further reversals; 48 ticks without a reversal reset. Result: dizzy (stars orbit, section 9), wobbling gaze, `scared`/`playful` per species, sticky for 2 s.

Measured [P]: 99.4 % detected over 2 to 5 Hz and +-25 to 70 px (the corner case 2 Hz at +-25 px has a peak speed of 314 px/s and is detected 73 %), **0 false triggers in 1.4 h** of ordinary dragging with overshoot and correction submovements. Oracle: `scipy.signal.find_peaks` on the projection of the position on its principal axis.

### 7.5 Guards, summary

Gates above; thresholds in pet units so they scale with the species; a gesture never fires while the pointer is over an interactive element; a successful gesture clears the other gesture states (one at a time); cooldowns per gesture and per actor; the circle needs the pointer to *stay* in the band, which ordinary travel (crossing from one side to the other in 0.3 to 1 s) does not.

---

## 8. Moods, states and escalation in virtual pets

### 8.1 How the references model it

| reference | model | take-away |
|---|---|---|
| **Tamagotchi** [S: https://tamagotchi.fandom.com/wiki/Care] | hunger and happiness hearts, a discipline bar, "attention calls" that count as care mistakes when ignored (about 15 minutes); evolution depends on care mistakes | needs plus neglect penalty: **not** what we want (no guilt, no death); only the visible hearts idea |
| **Nintendogs** [S: https://en.wikipedia.org/wiki/Nintendogs] | stroking on the touch screen is the main verb, reactions depend on where and how; rough taps make the dog yelp | continuous touch gesture as the verb (our stroking), gentle vs rough read from speed |
| **Animal Crossing** [S: https://animalcrossing.fandom.com/wiki/Bullying, Nookipedia] | pushing a villager for about 8 s, or hitting three times in a row, annoys them; talking too much gives warnings, then refusal for a while; apologising mends | escalation, saturation, refractory period, forgiveness: the template for our `heat` |
| **Neko Atsume** [S: https://en.wikipedia.org/wiki/Neko_Atsume] | visits are driven by items and randomness, no direct verb, rare visitors | charm from rarity and unpredictability; no obligation |
| **Shimeji-ee** [V round 1] | weighted behaviours with conditions, `Broadcast` / `Interact` pairs | data-authored weights and pair protocols |
| **Desktop Goose** [S: https://samperson.itch.io/desktop-goose (403 for the fetch tool), press summaries] | intentionally disruptive: steals the cursor, opens notes, drags windows [S]; aggression is a setting | the anti-pattern unless the user opted in; always an intensity knob |
| **Clippy / Microsoft Bob** [S: Reeves and Nass "Media Equation" lineage; retrospectives name interruption and the lack of any memory of the user as the core failures] | proactive help, no memory of the user, interrupts | never interrupt, never ask for attention; **remember** (our rapport persists within a session) |
| **Emotion contagion in crowds** [S: Bosse et al. ASCRIBE; Durupinar et al.; review https://link.springer.com/article/10.1007/s10458-022-09589-z (paywalled for the fetch tool)] | an agent's emotion intensity moves toward a weighted average of its neighbours', scaled by channel strength, sender expressiveness and receiver openness | the update rule of 8.3 |

### 8.2 Recommended compact model

- **Mood**: `Feeling = calm | happy | playful | curious | shy | annoyed | scared | sleepy` with `intensity in [0, 1]`, `since`, and a minimum hold of 128 ticks (2 s) to prevent flicker. Valence / arousal for the face (Russell's circumplex [M]): calm (0.1, 0.2), happy (0.8, 0.5), playful (0.7, 0.8), curious (0.3, 0.6), shy (-0.1, 0.3), annoyed (-0.6, 0.7), scared (-0.7, 0.9), sleepy (0.0, 0.0). The existing `Actor.mood` (valence, drives the mouth curve) becomes `intensity * valence(kind)`.
- **Priority**: `scared 5 > annoyed 4 > shy 3 > happy = playful 2 > curious 1 > calm 0`; `sleepy` is driven by the energy need and overrides when asleep.
- **Impulse** `(kind, amount, priority)` from events (appraisal table: poke 0.15 happy, hold 0.3 happy, pick up 0.25 scared (shy species: shy), shake 0.4 scared / playful by species, dropped from high 0.2 scared, parachute landing 0.3 happy, a pet lands on my head 0.3 annoyed, cuddle 0.4 happy, squabble 0.5 annoyed, tick-tock gesture circled 0.4 happy ...): if the kind equals the current one the intensity rises by `amount` (cap 1); else if `priority >= priority(current)` or `amount > 1.2 intensity` it replaces it (the old one is kept as a half-intensity residual for the decay phase); else it is ignored. **Decay**: `intensity -= decay(kind) / 64` per tick (happy 0.06/s, playful 0.10/s, curious 0.12/s, shy 0.08/s, annoyed 0.05/s, scared 0.15/s), return to the species baseline `calm` below 0.05.
- **Mood to behaviour**: multipliers on `activityWeights` (playful: fidget x2, hop x2, walk x1.5; shy: walk away x2, greet x0.3; annoyed: squabble x3 for rivals, greet x0.2; scared: hide / stay low, sleep x0; sleepy: sleep x4); the gaze (annoyed glares, shy looks away); tricks available per mood (`playful` or `happy` required for crowd-pleasing tricks).
- **State machine per species**: physical modes and activities as in 0.2; the table of transitions below is the same for all species; species data only enables/disables abilities (`climb`, `ladder`, `grapple`, `chute`, `float`) and supplies clips and tricks.

```
 ground --(hop / fall / throw)--> air --(vHit > 600 & room)--> chute --> ground
   |  \--(ladder placed)--> ladder --(top / bottom)--> ground
   |  \--(wall at foot / corner)--> wall --(mantle)--> ground ; wall --(slip / moved / reclaimed)--> air
   |  \--(rope target)--> rope --(zip / swing + mantle)--> ground ; rope --(miss)--> ground
   |--(pickUp)--> held --(release)--> air ; held --(cancel)--> ground (glide back) | air
   air --(lands on a head)--> head --(slides clear)--> air --> ground
   any solid --(unavoidable crossing)--> ghost --(free spot)--> solid
```

### 8.3 Contagion and the influence rule format

Contagion, evaluated for each ordered pair `(A, B)` within `R = 3 w` and the same or adjacent level, every 32 ticks, in pair order by (min, max) species index: for contagious kinds (`happy`, `playful`, `sleepy`, `scared`, `annoyed` with spread coefficients 0.5, 0.6, 0.8, 0.3, 0.2): `B.intensity += gamma * beta * bond * (A.intensity - B.intensity)^+` with `gamma = spread(kind)`, `beta = B.sociability`, `bond = 0.5 + 0.5 affinity` (affinity in `[-0.6, 1]` gives 0.2 to 1); if `B` is calm the kind is adopted at that intensity, never above `A`'s (no amplification: no runaway). Fear and anger spread weakly on purpose so that a squabble does not make the whole stage hostile. Yawn-like contagion for `sleepy` is the cutest and the strongest.

**Influence rules as data** (schema-first, validated; evaluated per pair, once every `each` ticks while `when` holds; deterministic order; a stage draw from the counter-based stream decides `chance`):

```jsonc
{
  "id": "cloud-rains-on-sun",
  "when": {
    "actor": { "species": "cloud", "activity": ["trick"], "trick": "rain" },
    "other": { "species": "sun", "feeling": { "not": ["scared"] } },
    "near":  { "within": 3.0, "unit": "heights", "other": "below" },
    "each": 32, "chance": 1.0, "cooldown": 640
  },
  "effects": [
    { "on": "other", "feeling": "annoyed", "amount": 0.2, "cap": 0.8 },
    { "on": "other", "request": { "activity": "idle", "clip": "shield", "priority": 2 } },
    { "on": "both",  "rapport": -0.02 },
    { "on": "actor", "emit": "rain-drops", "ticks": 160 }
  ]
}
```

Grammar: `when.actor|other` = `{ species?, activity[], trick?, feeling?, physical? }`; `near` = `{ within, unit: "px" | "heights" | "widths", sameSurface?, other: "left" | "right" | "above" | "below" | "any" }`; `effects[]` = `{ on: "actor" | "other" | "both", feeling+amount+cap | request{activity, clip, priority} | rapport | emit | trick }`. A `request` is honoured only if the target's current interruptibility (priority of its activity: `fall`/`held` > social > walk > fidget > idle) is lower; at most one active request per target. Because the actor's own tricks are species data too, a designer can author "sun circled -> shine -> warms the solar panel -> happy" without code.

### 8.4 Click escalation and saturation (the `heat` bucket)

Each poke / click / hold-tick on an actor adds to `heat` (poke 1.0, hold-purr 0.05 per tick), `heat` leaks at 0.5 per second; tiers: `heat <= 1` hello; `1 to 3` trick; `3 to 5` purr; `5 to 7` giggle / ticklish (the peak, no more reward); `>= 7` **enough**: the pet turns away or covers its face (`shy`/`annoyed` 0.3, not `scared`), pokes are ignored for a refractory 512 ticks (8 s) during which only a glance answers, then `heat` resets to 2. Releasing the pet resets nothing else: it forgives. This is the Animal Crossing shape (escalate, saturate, cool down, forgive [S]) without the penalty. There are no counters and no streak rewards on screen (round 1 section 6.4).

### 8.5 What keeps it charming

Rarity and variable timing (a trick has a cooldown and a species-specific chance, not a fixed rate), consistency of personality, anticipation and follow-through in every reaction, nothing that needs attention or punishes neglect (the Tamagotchi lesson, inverted), never interrupting (Clippy), no counters, small and bounded displacement, an intensity knob and an off switch (Desktop Goose lesson), reactions that remember within the session (rapport), and a few surprises that depend on the topic (the cast already fits the quiz).

---

## 9. Particles and effects as pure functions of time

### 9.1 Frame contract

The frame carries emitters, not particles: `EffectFrame { kind, x, y, start, until, key, a, b, c, d }` (numbers and an enum), at most 8 at a time. The core exports `particleCount(effect, tick)` and `particleOf(effect, index, tick)`; both are pure, twin-implemented and covered by the trace hash for a few ticks; the targets evaluate them (so no per-frame allocation in the core). Emitters are created by tricks, moods, landings and influence rules; an emitter that is past `until + life` is dropped from the frame.

### 9.2 Hash and birth times

A 32-bit mixing hash (Wellons' `lowbias32` [V: https://nullprogram.com/blog/2018/07/31/], bias 0.1735; `triple32` has bias 0.0209 if needed), with an additive constant so that `0` is not a fixed point (`lowbias32(0) = 0`):

```ts
const low = (x: number): number => { x >>>= 0; x ^= x >>> 16; x = Math.imul(x, 0x7feb352d); x ^= x >>> 15; x = Math.imul(x, 0x846ca68b); x ^= x >>> 16; return x >>> 0; };
const mix = (a: number, b: number): number => low(((a ^ 0x9e3779b9) + Math.imul((b + 1) | 0, 0x85ebca6b)) >>> 0);
const unit = (word: number): number => word / 4294967296;                  // exact, as the existing unitOf
// particle i, lane j of emitter key k:   unit(mix(mix(k, i), j))
```

Golden vectors (identical in Node and Python integer arithmetic [P]): `low(1) = 0x688990c0`, `low(0xdeadbeef) = 0xe628c683`, `mix(0,0) = 0xe577f3aa`, `mix(1,2) = 0xb111e030`, `mix(mix(7,3),5) = 0x5ab36a78`; the mean and variance of `unit(mix(12345, i))` over 200 000 indices were 0.4999 and 0.0831 (uniform: 0.5, 0.0833). Oracle: numpy `uint32` arithmetic. Alternative using only existing primitives: `randomWords([seed, EFFECT_STREAM, key * 65536 + index], 2)` (SeedSequence, validated against numpy already) at the cost of one array per call; at 200 draws a frame that is negligible CPU but allocates, hence `mix`.

Births without stored state: emitter start `s`, period `P` ticks (integer), life `Lf` ticks; particle `i` is born at `b_i = s + i * P + floor(unit(mix(mix(k, i), 0)) * P)` (the jitter is below `P`, so births are ordered by `i`); at tick `t`, candidates are `i in [max(0, floor((t - s - Lf) / P)), floor((t - s) / P)]`, at most `ceil(Lf / P) + 1` of them, each live iff `0 <= t - b_i < Lf`. Per particle: `age = t - b_i`, `s_ = age / Lf`, `tau = age / 64`. With integer ticks below 2^26 and small `P` the division `floor(a / b)` is exact in binary64.

### 9.3 Formulas (every one uses only `+ - * / floor` and the existing `sinTurns`, `cosTurns`, `smoothstep`, `fastNegExp`)

| kind | position | alpha and size |
|---|---|---|
| **rain** (cloud) | `x = e.x + (u1 - 0.5) * e.width`; `y = e.y + v * tau`, `v = 420 * (0.9 + 0.2 u2)` px/s; life ends at the floor: `Lf = ceil(64 * H / v)` (the splash is a second life stage: 3 droplets by the burst formula started at `b_i + Lf`, 8 ticks) | `alpha = min(1, age / 4) * (0.55 + 0.35 u3)`; streak length `6 + 6 u4` px along `v` |
| **burst** (sparks, confetti, pop) | `a = (j + u1) / N` turns (stratified); `sp = e.speed * (0.45 + 0.55 u2)`; `vx = sp * cosTurns(a)`, `vy = sp * sinTurns(a) - e.lift`; `x = e.x + vx * td * (1 - fastNegExp(tau / td))`, `y = e.y + vy * td * (1 - fastNegExp(tau / td)) + 0.5 * e.g * tau * tau` (`td` = drag time 0.4 s) | `alpha = 1 - s_^2`; `size = e.size * (1 - 0.6 s_)` |
| **rise** (hearts, steam, notes) | `y = e.y - e.rise * tau * (0.7 + 0.3 u1)`; `x = e.x + e.drift * sinTurns(0.35 * tau + u2) * (0.3 + s_)` | `scale = 0.5 + 0.5 * smoothstep(age / 8)` (steam: `0.6 + 0.8 s_`); `alpha = smoothstep(age / 6) * (1 - smoothstep((s_ - 0.6) / 0.4))` |
| **orbit** (dizzy stars, orbiting hearts) | fixed `N = 3 to 5`; `a_j = j / N + omega * tau_total`; `x = c.x + Rx * cosTurns(a_j)`, `y = c.y + Ry * sinTurns(a_j)` with `Rx = 0.45 w`, `Ry = 0.15 h`; depth order by `sinTurns(a_j)` sign | envelope `smoothstep((t - s) / 8) * (1 - smoothstep((t - until) / 8))` |
| **snow / dust motes** (persistent lanes) | `N` lanes `j`: `x = e.x0 + (u1 + 0.04 * sinTurns(0.1 * tau + u2)) * e.width`; `y = e.y0 + mod(v_j * tau + u3 * e.height, e.height)` with `mod(m, H) = m - floor(m / H) * H` | `alpha` constant, size `1 + 1.5 u4` |
| **beam** (sun, tractor) | not particles: a quad from the source to the target; width `w0 * (1 + 0.15 * sinTurns(2.5 * tau))` | envelope as orbit |

`fastNegExp` stays within about 0.1 % of `exp(-x)` for the range used [round 1]; the drag position `v td (1 - fastNegExp(tau / td))` saturates at `v td`. A Pade form `x / (1 + x/2)` is tempting but tends to `2 v td` (wrong limit): do not use it.

### 9.4 Caps, pooling, performance

- Counts: `rain` 24 live, `burst` 16, `hearts` 6, `steam` 10, `orbit` 5, `snow` 24; world-wide at most **160 live particles** and 8 emitters; `still` and reduced motion: no particles at all (an icon-like pose change instead, section 11).
- **Batching beats pooling nodes**: one `<path>` per (emitter, alpha bucket) with sub-paths `M x y l dx dy` for streaks and `M x y h0.01` with `stroke-linecap: round` and `stroke-width = size` for dots, three alpha buckets (1, 0.6, 0.3) per emitter, so an emitter costs at most 3 DOM nodes and 3 `setAttribute('d', ...)` per frame; a string of about 25 numbers per bucket formatted to 0.1 px. Hearts and stars, which need a real shape, use a small pool of `<use>` elements that reference one `<symbol>` (same-document reference, CSP-neutral): allocate `cap` elements per emitter once, never create or remove them during animation, hide unused ones with the `visibility` attribute.
- Node budget [S for the order of magnitude: SVG stays smooth to a few thousand elements, e.g. https://blog.logrocket.com/svg-vs-canvas/; blog benchmarks report about 2 ms and 60 fps at 100 animated SVG elements, degrading at hundreds to thousands: https://www.svgai.org/blog/research/svg-animation-encyclopedia-complete-guide; not measured here]: 10 pets at about 25 parts are about 500 nodes and 250 `transform` writes per frame already; effects add at most 24 nodes (8 emitters x 3). Budget [D] to be measured with the round-1 protocol: effects at most 0.3 ms main-thread per frame at 8 emitters, whole layer at most 2.5 ms at 4 x CPU throttle; zero writes when nothing is live.

---

## 10. Displacing host UI elements safely

### 10.1 What the platform guarantees

- **Individual transform properties**: `translate`, `rotate`, `scale` are Baseline widely available since August 2022 [V: https://developer.mozilla.org/en-US/docs/Web/CSS/translate]; they are animatable and transitionable. Composition [V: https://www.w3.org/TR/css-transforms-2/]: the matrix is `T(origin) * translate * rotate * scale * offset * (transform functions, left to right) * T(-origin)`, i.e. `translate` is applied **outside** the author's `transform`, so a displacement in the parent's axes is unaffected by an author rotation of the element. Use `translate` (and at most a tiny `rotate`), never `transform`, which the author may own.
- **CSP**: the CSSOM write `el.style.translate = "2px 1px"` or `el.style.setProperty("translate", ...)` is not blocked by `style-src`; inline `style` attributes, `setAttribute("style", ...)` and `cssText` are [V: MDN `style-src` / `style-src-attr`, round 1 section 5.1]. Restore with `el.style.removeProperty("translate")`.
- **Layout and rendering effects** [V: https://www.w3.org/TR/css-transforms-1/]: transforms leave the layout of other boxes alone and affect only overflow; they can extend the scrollable overflow area but never shrink it; any non-`none` value creates a **stacking context** and a **containing block for absolutely and fixed positioned descendants**, and the Transforms 2 text says the individual properties do the same when not `none` [V]; `getClientRects()` and `getBoundingClientRect()` return the **transformed** box [V]. Browsers disagree on the scrollable-overflow details (Chromium and Safari consider the transformed position only, Firefox both positions) [S: https://github.com/w3c/csswg-drafts/issues/9458]. Hit-testing follows the transformed geometry (pointer events land on the displaced element) [M, test it]. "Transformable element" excludes non-replaced inline boxes and table-column(-group) boxes; table rows and cells are transformable per the specification and work in Chromium and Firefox in practice [V spec; S for the engines: W3C bug thread "Various table-related elements are not transformable per spec"]. Sticky positioning with a transform between the sticky element and its scroller is under-specified [S: https://github.com/w3c/csswg-drafts/issues/3186].

### 10.2 Rules (what may be displaced and how much)

| rule | value |
|---|---|
| property | `translate` only; optional `rotate` at most 0.8 degrees for non-surfaces; never `scale` (blurs text) |
| magnitude | `abs(dx) <= 6 px` and `abs(dy) <= 6 px`; typical landing dip 1 to 3 px (a weight dip: the card sags and springs back, `omega = 18`, `zeta = 0.5`); bump 2 to 4 px away from a hit |
| positive direction | clamp by the free scroll slack of the nearest scrollable ancestor (`scrollWidth - clientWidth - scrollLeft`, same for y) minus 8 px, so a displacement never creates a scrollbar; negative (left, up) cannot extend overflow |
| count and rate | at most 2 elements displaced at once; at most one displacement per element per 4 s; each lasts under 600 ms |
| never | tables and table parts (`display: table*`), non-replaced inline boxes, `position: sticky` elements or their scrollers' sticky children, ancestors of `position: fixed` descendants (the transform changes their containing block), elements containing the focus or the pointer (within 48 px), elements with `[inert]`, `[hidden]`, controls themselves (only containers), anything in the top layer, any element during a run (`quiet`), `still`/reduced motion, `(pointer: coarse)` or viewport under 768 px |
| surveys | the shell reports the **rest rectangle**: `rect_rest = getBoundingClientRect() - applied` (translate only, which is why rotation is excluded from surfaces) so the surface does not drift; the core owns the displacement spring, so a pet's `y` is `rest.y + dy` and nothing feeds back |
| host transitions | if `getComputedStyle(el).transitionProperty` contains `all` or `translate` with a non-zero duration the element is not displaceable (a host transition would smooth every per-frame write, as round 1's WP-R found for the layer); under the quiz's reduced-motion rule (`.quiz-app * { transition-duration: 0.01ms !important }`) every write would still start a transition, so displacement is off there anyway; the survey's `transitionend` watcher must ignore events whose `propertyName` is `translate` or `rotate` (or it re-surveys on every displacement frame) |
| undo | `releaseAll()` synchronously on `visibilitychange: hidden`, `pagehide`, pause, layer unmount, `tuned{still}`, `hushed{quiet: true}`; a `WeakMap<Element, { prior: string }>` stores the author's inline `translate` before the first write and restores it (`removeProperty` when there was none). No CSS transition is used for the displacement (the core's spring is written per frame), so no `transitionend` or `transitioncancel` is needed to finish the undo; both events only exist as a nuisance to ignore, because `transitioncancel` replaces `transitionend` whenever a running transition is interrupted [V: MDN `transitioncancel`], and neither fires at all when the duration is 0 |

### 10.3 When the user takes an element back

The release must happen **before** the press whenever possible. Removing a transform between `pointerdown` and `pointerup` moves the element under a stationary pointer; the `click` event is dispatched to the common ancestor of the down and up targets, so a click on a displaced button can be lost. Therefore:

1. **Hover releases**: `pointerover` / `pointerenter` (mouse, pen) anywhere on a displaced element or its descendants starts a release with halflife 0.04 s (done in about 150 ms), long before a press lands. Touch has no hover, which is why displacement is off for coarse pointers.
2. Fail-safes, in one capture-phase passive set of listeners on `document`: `pointerdown`, `focusin`, `keydown`, `input`, `change`, `dragstart`, `selectstart`; the handler walks up from `event.target` (at most 12 steps) to the nearest displaced element and calls `reclaimed(surfaceId)` for the core and removes the `translate` synchronously.
3. **Not** treated as taking back: `wheel` and scroll (the page scrolls under the pets anyway), window resize (survey), focus leaving.
4. The core's answer to `reclaimed`: pets standing on that surface are **thrown out** (vx = +-(120 to 200) px/s away from the pointer or the centre, vy = -220 px/s, a surprised face, then `air` and the parachute rule); ladders against it topple; wall climbers are ejected (section 5.4).

### 10.4 Prior art and what makes it delightful rather than hostile

- Shimeji-ee mascots **carry and throw the browser window** (actions `WalkWithIe`, `RunWithIe`, `FallWithIe`, `ThrowIe`, `ClimbIEWall`, `JumpOnIELeftWall` ... [V-sum: actions.xml, section 1.1]); Desktop Goose is reported to drag windows around and steal the cursor [S]; Google Gravity (Mr.doob) detaches DOM elements and hands their positions to Box2D [S: https://www.dualmedia.fr/en/how-to-use-google-gravity-how-it-works/]; Google's "Do a barrel roll" rotated the whole results page by CSS [M]. All of them are things the user *asked for* (a bookmarklet, an easter egg, an installed app). The pets are unasked, so the limits are tighter: a few pixels, only with a visible physical cause (a pet's weight, a bump, a tug), never over something being read or typed, never in a run, gone the instant the pointer approaches, a setting ("Pets may nudge cards", default on in `lively`, off in `calm` and `still`), and an exact undo. The delight is in the *plausibility* of a 2 px sag when a pet lands, not in the magnitude.

---

## 11. Interactive pets without hurting accessibility

### 11.1 Success criteria that become relevant (all [V] from the Understanding documents under https://www.w3.org/WAI/WCAG22/Understanding/)

| SC | what it says (paraphrased) | consequence |
|---|---|---|
| **2.1.1 Keyboard** (A) | all functionality operable through a keyboard interface; "dragging objects to a location" is not path-dependent and must support keyboard operation | the "Play with the pets" panel (11.3) |
| **2.2.2 Pause, Stop, Hide** (A) | moving content that starts automatically, lasts over 5 s, in parallel with other content needs a mechanism | existing footer switch; it must also drop a held pet and release all nudges |
| **2.3.3 Animation from Interactions** (AAA) | interaction-triggered motion can be disabled unless essential; respect `prefers-reduced-motion` | gestures, tricks, nudges and effects off under `still`; the response may be a non-motion change (a 150 ms opacity cross-fade of an expression) |
| **2.4.11 Focus Not Obscured (Minimum)** (AA) | a focused component is not entirely hidden by author content | existing focused-element keep-out; never displace the focused element or its ancestor; a thrown or hanging pet never covers it (the held pet is projected out of keep-outs too) |
| **2.5.1 Pointer Gestures** (A) | multipoint or **path-based** gestures need a single-pointer alternative unless essential; simple dragging in any direction is not path-based | **circling and stroking are path-based**: every trick is also reachable by click escalation and by the panel |
| **2.5.2 Pointer Cancellation** (A) | one of: no down-event execution; abort or undo; up reversal; essential | a press only arms; the pick-up commits on movement; release drops; Escape, `pointercancel`, `lostpointercapture` abort with undo |
| **2.5.7 Dragging Movements** (AA) | functionality that uses dragging has another single-pointer mode (e.g. click the first corner, then the opposite one) unless essential | "click a pet, then click a destination card" (it walks / hops / climbs there), plus the panel |
| **2.5.8 Target Size (Minimum)** (AA) | pointer targets at least 24 x 24 CSS px (spacing exception, inline exception) | hit area `max(box, 24 px)`; pets are 32 to 56 px |
| **1.4.13 Content on Hover or Focus** (AA) | additional content on hover/focus must be dismissible, hoverable, persistent | no hover tooltips or name labels on pets; the names are in the panel |
| **2.3.1 Three Flashes** (A) | no flashing | effects never strobe; sun rays pulse slowly (below 3 Hz, small area) |

### 11.2 Keeping the layer out of the accessibility tree

`aria-hidden="true"` on the layer; SVGs `focusable="false"`; no roles, no tab stops, no live regions in the layer; `user-select: none`; no `title`. Mouse and pen: **window-level capture-phase passive listeners** (the layer keeps `pointer-events: none`), which means the browser delivers the event to whatever is under the pointer first and the pet code only *reads* it: if the target has an interactive ancestor (`PET_CONTROLS`), `event.defaultPrevented`, a non-collapsed selection, a non-primary button or a non-primary pointer, the pet code ignores the event, so **a control always wins**. Pets stand only on perches cleared of keep-outs (headroom band), so a *standing* pet never covers a control; an airborne pet may, and then the same rule applies because the layer is transparent to hit-testing. Touch cannot be handled this way: `touch-action` is determined by the event target (the intersection of the values of the touched element and its ancestors up to the scroller) and changing it after a gesture has started has no effect on that gesture [V: https://developer.mozilla.org/en-US/docs/Web/CSS/touch-action], so a window-level handler cannot stop the browser from scrolling or zooming. Therefore on touch, dragging a pet exists only in the explicit **play mode**, in which small hit pads (`pointer-events: auto`, `touch-action: none`, at least 24 x 24 CSS px, `user-select: none`, `-webkit-touch-callout: none`) are placed over **grounded, cleared-perch** pets only (never over a control by construction; removed while airborne), and the rest of the page keeps native scrolling. Outside play mode, touch gets taps (hello / trick / purr by escalation) through the same window listener, which needs no `touch-action`.

Pointer capture: on pick-up call `target.setPointerCapture(event.pointerId)` on `document.documentElement` (mouse and pen; touch has implicit capture) [V: https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture] so moves and the release arrive even outside the window; release on `pointerup`, `pointercancel`, `lostpointercapture`. A browser pan takeover delivers `pointercancel` [V, touch-action page]: treat it as a cancel.

### 11.3 The keyboard equivalent: "Play with the pets"

A switch in the existing pets preference group, "Pets can be played with" (default on for fine pointers, off for coarse pointers and in `still`), and, when on, a labelled group in the same place (not in the layer): per pet currently on stage (names from `Species.name`, both languages): buttons "Say hello", "Do a trick", "Pet it" (starts a purr for 3 s), and a menu "Go to ..." listing the surfaces by their heading text (the shell keeps `id -> label`) that sends `played{ species, deed: "go", surface }`; the core plans hop, ladder, wall or grapple to the nearest valid perch. Feedback: a polite status message for explicit actions only ("Sunny shines.", "Sunny moves to Question 3."; WCAG 4.1.3), never for autonomous behaviour. All of this works with the keyboard alone and with a screen reader, while the pets stay `aria-hidden`.

### 11.4 Cancelling and undo

`Escape` (document `keydown`, capture, passive) while a pet is held or armed cancels (glide back to the origin, section 1.5); no `preventDefault` unless the press was ours and a dialog is not open; `pointercancel`, `lostpointercapture`, window `blur`, `visibilitychange`, `hushed` cancel as well; the pause switch cancels everything. After a cancel the pet is where it started, not where the pointer is.

---

## 12. Events (shell to core) and frame additions (core to renderer)

In the existing style: past-tense kinds, plain objects, numbers and short enums only. The shell never supplies timestamps; the core stamps with its own tick, so the event log alone reproduces everything.

```ts
// Shell -> core (additions; existing: ticked, pointed, unpointed, glanced, surveyed, summoned, tuned, hushed, poked)
type Pressed    = { kind: "pressed";    x: number; y: number; pointer: "mouse" | "pen" | "touch" };   // primary button or contact went down on a candidate (no interactive ancestor)
type Dragged    = { kind: "dragged";    x: number; y: number };                                        // pointer position while a pet is armed or held, once per frame (coalesced samples welcome)
type Released   = { kind: "released";   x: number; y: number };
type Cancelled  = { kind: "cancelled";  reason: "escape" | "pointercancel" | "blur" | "hidden" | "paused" | "capture-lost" };
type Reclaimed  = { kind: "reclaimed";  surface: string };                                             // the user touched, focused or typed into a displaced or occupied element
type Scrolled   = { kind: "scrolled" };                                                                 // resets gesture state (the page moved under the pointer)
type Played     = { kind: "played";     species: Slug; deed: "hello" | "trick" | "pet" | "go"; surface?: string };   // keyboard / panel equivalent
type Permitted  = { kind: "permitted";  play: boolean; nudge: boolean; touch: boolean };               // preferences: pick-up, nudging of host elements, touch play mode
// Surveyed gains:  walls: readonly Wall[]  with  Wall = { id: string; x: number; y0: number; y1: number; side: -1 | 1 }
//                  and per surface  nudgeable: boolean  (the shell's verdict of section 10.2)
// pointed gains:   over: "none" | "control" | "text"   (so gestures can ignore a pointer over an interactive element)
```

Existing `poked{x, y}` stays as the keyboard-free tap shortcut; the click chain and escalation then live in the core (derived from `pressed` / `released`).

```ts
// Core -> renderer (additions)
type Feeling = "calm" | "happy" | "playful" | "curious" | "shy" | "annoyed" | "scared" | "sleepy";
type ToolFrame =
  | { kind: "chute";  open: number; sway: number }                               // open 0..1 (can overshoot to 1.25), sway in turns
  | { kind: "rope";   x0: number; y0: number; x1: number; y1: number; slack: number }
  | { kind: "gun";    aim: number }                                              // aim in turns, muzzle drawn at the rig's hand bone
  | { kind: "hook";   x: number; y: number; angle: number };
type ActorFrame = {
  /* existing: species, x, y, facing, activity, opacity, bones, eyes, mood */
  tilt: number;                  // turns, rotation of the whole drawing about `pivot`
  pivot: readonly [number, number];   // in feet coordinates, e.g. (0, -0.9 h) while held
  feeling: Feeling; intensity: number;
  tools: readonly ToolFrame[];
  ghost: boolean;                // draw translucent, trailing
  hit: readonly [number, number, number];   // centre x, y, radius for the shell's hover / pointer test
};
type LadderFrame  = { id: number; baseX: number; baseY: number; topX: number; topY: number; rungs: number; opacity: number; state: "raising" | "standing" | "toppling"; angle: number };
type EffectFrame  = { kind: "rain" | "burst" | "hearts" | "steam" | "orbit" | "snow" | "beam"; x: number; y: number; start: number; until: number; key: number; a: number; b: number; c: number; d: number };
type NudgeFrame   = { surface: string; dx: number; dy: number };               // the shell writes style.translate; 0,0 = release
type Frame = {
  /* existing: tick, actors, rate, wake */
  ladders: readonly LadderFrame[];
  effects: readonly EffectFrame[];
  nudges: readonly NudgeFrame[];
  grab: null | { actor: Slug; cursor: "grab" | "grabbing" };   // the core took the press: the shell captures the pointer
};
```

`ACTIVITIES` additions: `held, thrown, chute, brace, aim, shoot, reel, swing, climb, mantle, slide, trick, purr, dizzy, shrug, scoot, place`. Species data additions (schema first): `abilities: ("climb"|"ladder"|"grapple"|"chute")[]`, `bodies: { stand, climb, hang, chute: Size }` (posture boxes), `grip: number` (scruff height in `h`), `lightness: number` (scales `vt`), `tricks: { id, trigger{gesture, turns?, direction?, feeling?}, clip, effect?, feeling?, cooldown }[]`; menagerie additions: `influences: Influence[]`. Core modules (names to be given emoji from the registry): `swing` (swingStep, held, chute, rope), `climb` (ladder, wall, mantle), `clearance` (boxes, claims, seat, ghost), `gesture` (click chain, circle, stroke, shake), `feeling` (mood, heat, contagion, influences), `effects` (particles), `nudge` (host springs).

---

## 13. Tests and oracles per feature (each needs a language-agnostic case and a third-party reproduction)

| feature | oracle |
|---|---|
| `swingStep` (rod, rope, reel) | `scipy.integrate.solve_ivp` (DOP853, `rtol = 1e-12`) on `theta'' = -(g/r) sin theta - 2 (r'/r) theta'`; tolerance 0.4 degrees over 3 s, amplitude drift < 0.05 degrees over 60 s; negative control: the radial-projection step must fail the drift test |
| anchor spring, chute descent | `scipy.linalg.expm` / closed form of the first-order lag; sympy for `F = exp(-1/(64 tau))` |
| release velocity | `numpy.polyfit(deg = 2)` derivative at the last sample on the same window: weights `[7,-2,-7,-8,-5,2,13]/28` must reproduce it exactly (rational arithmetic) |
| `atanTurns` | mpmath `atan2` (50 digits), maximum error 1.9e-6 turns; golden bit patterns for the TS / Rust twin |
| circle, stroke, shake | numpy (`unwrap(arctan2)` winding; `scipy.signal.find_peaks`) on recorded traces; plus the false-positive corpus (minimum-jerk page travel, tremor, sweeps) with a zero-trigger assertion |
| clear-line test | `shapely.LineString.intersects(box)` |
| ladder / wall geometry | `shapely` and numpy for lean ratio, length, rung count; the mantle path against `scipy.interpolate` of the same keys |
| seating (PAVA) | `sklearn.isotonic.IsotonicRegression` (with bounds) and `scipy.optimize.minimize` (SLSQP) on the QP |
| non-overlap | the executable invariant plus `shapely.box(...).intersects` as an independent overlap test over recorded traces; `fast-check` / `proptest` for traces; TS-vs-Rust trace hashes include ghost flags |
| effects | numpy vectorised evaluation of the same formulas; `numpy` `uint32` hash; Holden's `fastNegExp` vs `numpy.exp` tolerance 1e-3 |
| host displacement | Playwright in Chromium, Firefox, WebKit: `getBoundingClientRect` after `style.translate`, a fixed-position descendant moves with the element, no CSP violation under `style-src-attr 'none'`, `elementFromPoint` hits the displaced box, `click` survives a hover-release, no scrollbar appears at the page edge, no `transitionend` loop |
| accessibility | axe-core; Playwright keyboard run of the panel; `emulateMedia({ reducedMotion: "reduce" })` gives no interaction motion; a control under a pet receives the click; Escape cancels |

---

## 14. Risks and open items

1. **Held gravity and lean mapping** are tuned numbers; a minimum-jerk simulation says ordinary drags saturate the cone unless `gscale >= 3`, but only the stories gallery can show whether the quick 0.46 s swing looks cute or nervous; a taller virtual rod and a lean gain below 1 are the alternatives.
2. **The reeled swing** needs its two guards (minimum length, tangential cap) or it winds up; verify with the ODE oracle at the extremes.
3. **Ghost** is a visible compromise: bounded and rare in the prototype (0.17 % of actor-ticks under brutal chaos), but a ghost crossing is a translucent overlap by design. If the owner finds that unacceptable, replace the ghost by crowd-out (fade where it stands, arrive anew) at the cost of more disappearances.
4. **Touch dragging** is only possible in an explicit play mode because of `touch-action`; this is a platform fact, not a design choice.
5. **Host displacement** relies on [M] behaviours (hit-testing, table rows, sticky, Firefox overflow); the Playwright matrix in section 13 is a precondition for enabling it, and the safe fallback is to restrict displacement to plain block containers.
6. **Evidence quality**: several prior-art claims are from search summaries or decoded sources ([S], [V-sum]); the Desktop Goose claims in particular could not be read from the primary page (HTTP 403).
7. **Prototype scope**: the crowd experiment is two-dimensional axis-aligned boxes in a toy world; ropes, ladders and walls were not simulated; the claims for them are by construction (planned corridors) and must be fuzzed in the real core.
8. **Art cost**: climb, hang, aim, mantle, shrug and trick clips per species, plus a held pose; floaters need only a tiny set. Do the crowd and drag / chute work first (they are mostly physics), gestures and moods next (data), then walls, ladders and rope (they need the survey's `walls`).

---

## 15. Sources

Verified by reading in this session ([V]): CSS `translate` https://developer.mozilla.org/en-US/docs/Web/CSS/translate ; CSS Transforms 1 https://www.w3.org/TR/css-transforms-1/ ; CSS Transforms 2 https://www.w3.org/TR/css-transforms-2/ ; MDN `touch-action` https://developer.mozilla.org/en-US/docs/Web/CSS/touch-action ; MDN `setPointerCapture` https://developer.mozilla.org/en-US/docs/Web/API/Element/setPointerCapture ; MDN `transitioncancel` https://developer.mozilla.org/en-US/docs/Web/API/Element/transitioncancel_event and `transitionend` https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionend_event ; WCAG 2.2 Understanding: 2.5.7 https://www.w3.org/WAI/WCAG22/Understanding/dragging-movements.html , 2.5.2 .../pointer-cancellation.html , 2.5.8 .../target-size-minimum.html , 2.5.1 .../pointer-gestures.html , 2.1.1 .../keyboard.html , 1.4.13 .../content-on-hover-or-focus.html , 2.3.3 .../animation-from-interactions.html ; D. Silver, Cooperative Pathfinding (AIIDE 2005; pages 1 to 4 read as images) https://cdn.aaai.org/ojs/18726/18726-52-22369-1-10-20210928.pdf ; Microsoft `GetSystemMetrics` https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getsystemmetrics and `SetDoubleClickTime` https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setdoubleclicktime ; C. Wellons, hash prospector https://nullprogram.com/blog/2018/07/31/ ; Hedgewars rope thread (no formulas) https://www.hedgewars.org/node/2893.

Fetched as summaries or decoded sources ([V-sum]): Shimeji-ee `actions.xml` https://raw.githubusercontent.com/TigerHix/shimeji-ee/master/conf/actions.xml ; Android `VelocityTracker.cpp` https://android.googlesource.com/platform/frameworks/native/+/master/libs/input/VelocityTracker.cpp and `ViewConfiguration.java` https://android.googlesource.com/platform/frameworks/base/+/refs/heads/main/core/java/android/view/ViewConfiguration.java ; Celeste `Player.cs` constants https://gist.github.com/Alexandria/bf563fd51aeb3ddd31f754ea9118e20f ; Jakobsen translation https://github.com/krisives/advanced-character-physics ; Lemmings glitches https://giovanniviglietta.com/files/lemmings/Glitches.html .

Search results only ([S]): MAPF definitions (Stern et al.) https://ojs.aaai.org/index.php/SOCS/article/download/18510/18301/22026 ; Gipps' model https://www.researchgate.net/publication/31412592_An_analysis_of_Gipps'_car-following_model_of_highway_traffic ; SHAKE (Ryckaert, Ciccotti, Berendsen 1977) http://www2.stat.duke.edu/~scs/Courses/Stat376/Papers/Constraints/Shake1977.pdf ; counter-based generators (Salmon et al., SC11) https://www.thesalmons.org/john/random123/papers/random123sc11.pdf (numpy `Philox` is the third-party implementation); Tamagotchi care https://tamagotchi.fandom.com/wiki/Care ; Animal Crossing annoyance https://animalcrossing.fandom.com/wiki/Bullying ; Nintendogs https://en.wikipedia.org/wiki/Nintendogs ; Neko Atsume https://en.wikipedia.org/wiki/Neko_Atsume ; emotion contagion in crowds https://link.springer.com/article/10.1007/s10458-022-09589-z ; Desktop Goose https://samperson.itch.io/desktop-goose ; Google Gravity https://www.dualmedia.fr/en/how-to-use-google-gravity-how-it-works/ ; ladder climbing study https://www.researchgate.net/publication/15498181_Biomechanical_analysis_in_ladder_climbing_the_effect_of_slant_angle_and_climbing_speed ; scrollable overflow of transforms https://github.com/w3c/csswg-drafts/issues/9458 ; sticky and transforms https://github.com/w3c/csswg-drafts/issues/3186 ; SVG performance orders of magnitude https://blog.logrocket.com/svg-vs-canvas/ and https://www.svgai.org/blog/research/svg-animation-encyclopedia-complete-guide ; Lemmings floater https://lemmings.fandom.com/wiki/Floater (HTTP 402 for the fetch tool, search summary only).

Recalled, not re-verified ([M]): Abramowitz and Stegun 4.4.49 coefficients (verified numerically here), Savitzky-Golay edge weights (verified numerically here), the 4:1 ladder rule, Russell's circumplex, ORCA, `getCoalescedEvents` support, hit-testing under transforms, the "Do a barrel roll" easter egg, fast-check / proptest / small-scope hypothesis.

---

## Appendix A. Prototype algorithms (the scratch code is not in the repository; these are the parts that carry the claims)

**A.1 SHAKE rod / rope step**: section 1.3 (`swingStep`). Numerical protocol: RK4 reference with `h = 1/20000`; compare the angle at ticks 16, 32, 48, 64, 96, 128, 160, 192; peaks over 30 to 60 s for energy.

**A.2 Circle detector**: section 7.2, with `Qprev` replaced by the Schmitt-trigger signs; state `{ q, sx, sy, last, steps[], rmin, rmax, out, opp, prev }`, reset on band exit for more than 8 ticks, stall beyond 40 ticks, or after a lap; the lap test `QMIN * (4t - 1) <= span <= QMAX * (4t - 1)`, `rmax <= 3 rmin`, `opp <= 1`.

**A.3 Non-overlap prototype outline** (sequential update, rotating order):

```
tick t: maybe survey (reseat: ride, PAVA targets -> walk(scoot) with guards; rider overlapping a non-moved body -> ghost slide to the nearest free spot)
        maybe user picks a standing pet (held) / releases (fall with v from the last displacement)
        for each actor in rotation:
          held:    follow target; project out of obstacles = (bodies' boxes + planned movers' remaining corridor slices, 8-tick bounding boxes
                   plus the current box); keep the old position if still overlapping; on release plan_fall
          planned: set position to plan[t - t0]; at the end finish_plan (land -> stand | ghost: nearest free spot; head -> slide)
          slide:   check support (host top == my feet, x overlap) else plan_fall; move away from the host at 0.5 + 0.03 k px/tick; reverse at edges / blocked
          stand:   after dwell: hop (pick a perch within |dy| < 100, plan_hop) or walk
          walk:    next x; blocked iff the box (margin 4) hits any obstacle; stall > 40 ticks -> stand
plan_hop: ballistic like hopOf; reject if any tick's box (margin 4) hits a stationary body or a planned body at that tick,
          or if the final spot hits any planned body's later positions (future_clear)
plan_fall: candidates dvx in {0, +-30, +-60, +-90, +-130, +-180, +-240}: simulate per tick; landing = perch crossing with the whole footprint
          on the perch; head = feet cross the top of a box with x overlap; side contact = conflict; final spot vs future positions;
          first conflict-free candidate wins; else ghost run (ignore bodies) ending in the free-spot search
```
