# Quiz Pets: Research Brief for Skeletal, Procedural, Behavioural and Rendering Design

Date: 2026-10-02. Scope: decorative animated "pets" (sun, cloud, house, solar panel, radiator, heat pump, window, wall, battery, ...) in the React quiz site (static site, strict CSP, desktop > mobile > tablet). Constraints: no runtime libraries, schema-first, TypeScript implementation plus bit-exact Rust twin of the pure core, deterministic (MT19937 exists), event-driven (`step(world, events) -> world`), `prefers-reduced-motion`, near-zero hidden-tab CPU, decorative and non-blocking.

Verification legend used below: **[V]** = fetched and read in this session; **[S]** = only seen in a search summary; **[M]** = from my own knowledge, not re-verified; **[D]** = design decision / starting value to be tuned. Nothing here was built or benchmarked (read-only research); every performance number is an estimate with a stated measurement protocol (section 5.6).

---

## 0. Decisions at a glance

| Topic | Decision |
|---|---|
| Rig | Parent-first bone array, rest local transform `x,y,rotation,scaleX,scaleY,length,inherit`; FK with 2x3 affine in the SVG `matrix(a b c d e f)` convention; rotations stored in **turns**. |
| Skinning | Rigid attachments (one slot = one bone) + three small deformable attachment kinds: `limb` (capsule between two bones), `ribbon` (Catmull-Rom path through N bones), `morph` (blend-shape path). No mesh LBS in v1. |
| Draw order | Slot array order = back-to-front; slots can switch attachment / alpha; optional keyed `drawOrder` offsets (Spine style). |
| Math | Hot path uses **only** `+ - * / sqrt abs floor` and comparisons (all IEEE-754 exactly specified). Own `detmath` (sin/cos/atan2/exp) written *in terms of those ops* from fdlibm kernels, ported identically to TS and Rust. Forbid `Math.sin/cos/atan2/exp/pow/hypot/...` and Rust `f64::sin/...` in the core by lint. |
| Time | Fixed step `DT = 1/64 s` (power of two, so tick time is exact); integer ticks only cross the shell/core boundary. |
| Springs | Closed-form damped springs with coefficients precomputed per `(omega, zeta, DT)` at load (Juckett 4-coefficient form, Holden halflife parameterization). Semi-implicit Euler only for non-linear cases. |
| IK | Two-bone analytic (sqrt-based knee point, one `atan2` per bone) for legs; FABRIK with a fixed iteration count for tails/necks. |
| Walking | Foot "stepper" with step triggers and planted feet (procedural creature style); species gait is one of `walk`, `hop`, `float`. |
| Behaviour | Layered: continuous procedural layers (blink, breathe, gaze, secondary) + a small **utility-weighted FSM** (Shimeji-style weights/conditions, needs/mood modulate weights, hysteresis, global activity governor) + a pair-wise social protocol (broadcast / respond / script). |
| Surfaces | Opt-in `data-pet-*` surfaces and lanes, expressed as horizontal segments in viewport coordinates minus keep-out intervals; reducer replaces the surface set wholesale on `surfaces-changed`. |
| Rendering | Per-pet small inline `<svg>`; wrapper moved by CSSOM `transform` (compositor), bones via `setAttribute('transform', 'matrix(...)')`. Colours via CSS classes + custom properties in the external stylesheet (not `fill="var()"`). Canvas 2D is a documented alternate target behind the same draw list. |
| CSP | Only SVG attributes + CSSOM property writes (`el.style.x = v` / `setProperty`); never `style=""` markup, `setAttribute('style')`, `cssText`, `<style>` injection. Enforced by an e2e test with `style-src-attr 'none'`. |
| Loop | rAF accumulator -> `tick{n}`; render rate classes 60/30/15/0; at rate 0 no rAF at all, one timer for `nextWake`; stop on `visibilitychange`. |
| Reduced motion | OS `reduce` -> default mode `still` (static rest pose, no blinks, no gaze); explicit in-app choice may raise it to `calm`, never `lively`. |
| Exam context | During a timed run: `sleep`/`still` pets outside the task panel, no social events, no reactions to answers. |
| Oracles | numpy/scipy (`expm` for springs, `least_squares` for IK), mpmath (ULP accuracy of detmath), sympy (derivations), gl-matrix (FK), bezier-easing + Web Animations API (easing), `numpy.random.MT19937` (RNG), planck.js/pymunk (ballistics), pyfabrik / `@aminere/fullik` (FABRIK, tolerance only). Bit-exact TS-vs-Rust parity is proven by per-tick bit-pattern trace hashes, not by oracles. |

---

## 1. Skeletal model

### 1.1 How the three reference formats structure data

**Spine JSON** [V: http://esotericsoftware.com/spine-json-format, http://esotericsoftware.com/spine-runtime-skeletons]
- Top level: `skeleton` (metadata), `bones`, `slots`, `ik`/`transform`/`path`(/`physics` in 4.2 [S]) constraints, `skins`, `events`, `animations`.
- `bones[]`: `name`, `parent` (name; parents listed before children), `length`, `x`, `y`, `rotation` (degrees), `scaleX`, `scaleY`, `shearX`, `shearY`, `transform` (inherit mode: `normal | onlyTranslation | noRotationOrReflection | noScale | noScaleOrReflection`). Omitted values default to 0 (scale 1).
- `slots[]`: `name`, `bone`, `color`, `dark`, `attachment` (setup attachment name), `blend`. **Slot array order is the draw order.**
- `skins[]`: slot name -> attachment name -> attachment data (region, mesh, path, ...).
- Constraints have an explicit `order` and are interleaved with bone updates: IK: `bones[1|2]`, `target`, `mix`, `bendPositive`, `compress`, `stretch`.
- `animations.<name>`: `bones.<bone>.{rotate|translate|scale|shear}` arrays of `{time, value(s), curve}`; `slots.<slot>.{attachment|color}`; constraint timelines; `deform`; `events`; `drawOrder` (`offsets`). Curve: omitted = linear, `"stepped"`, or Bezier `[cx1, cy1, cx2, cy2]` (x = time unit, y = value).
- Runtime model: local transform (x, y, rotation, scale, shear) per bone, world = combine with parent; `updateWorldTransform()` processes bones in order with constraints. Animation values are **relative to the setup pose**.

**DragonBones JSON** [V: https://github.com/DragonBones/DragonBonesJS/blob/master/docs/DragonBones_4.5_data_format_zh.md]
- `armature[]` -> `bone[]` (`name`, `parent`, `length`, `transform{x,y,skX,skY,scX,scY}`), `slot[]` (`name`, `parent`, `displayIndex`, `blendMode`, color multipliers/offsets), `skin[]` (per slot a `display[]` of image/mesh/armature), `ik[]` (`name`, `bone`, `target`, `bendPositive`, `chain`, `weight`), `animation[]` (`name`, `duration`, `playTimes`, per-bone/slot `frame[]` with **durations** (not absolute times), `tweenEasing`, `curve`, `transform`), `ffd` (mesh deformation). Difference to Spine: skew (`skX/skY`) instead of shear, duration-based frames.

**Rive** [V (container only): https://rive.app/docs/runtimes/advanced-topic/format; S: https://dev.to/uianimation/engineering-interactive-mascots-with-rives-state-machine-and-runtime-architecture-4e2h, https://deepwiki.com/rive-app/rive-runtime]
- Binary `.riv`: objects typed by integer keys, properties keyed by integer with a table of contents so unknown properties can be skipped; objects reference parents by index within an artboard; animations are `LinearAnimation` -> keyed objects -> keyed properties -> keyframes with interpolators [M]; bones/skins/tendons and IK/transform/distance constraints [M]; **state machines with named inputs (bool, number, trigger)** and layered transitions [S]. Runtime = scene graph with dirty-flag component updates; `advance(elapsedSeconds)`.
- Lesson for us: the state-machine *input* boundary (bool/number/trigger set from outside) is exactly our event boundary; and "dirty flags" are our `needsFrames` signal.

### 1.2 Our conventions

- Coordinates: **y-down**, matching SVG/CSS. Angles are stored in **turns** (1.0 = 360 degrees), positive = clockwise on screen. Authoring JSON uses degrees (Spine-like); the loader converts `deg / 360` (one correctly rounded division, identical in both languages).
- Why turns: quadrant folding of a turn-valued angle is exact in binary floating point (`t - floor(t)`, `0.5 - t`, `t - 1` are exact), so no `pi` reduction error and no `pio2_hi/lo` machinery is needed (see 2.0).
- Bones are stored parent-first (`parent < index`), so FK is one linear pass.

### 1.3 Forward kinematics (exact formulas)

Column-vector convention, SVG matrix `matrix(a b c d e f)`:

```
x' = a*x + c*y + e
y' = b*x + d*y + f          // = [[a c e],[b d f],[0 0 1]] * [x y 1]^T  (CSS Transforms 1: SVG matrix mapping) [V]
```

Local matrix of bone `i` from pose `(x, y, rot[turns], sx, sy)`, i.e. `L = T(x,y) * R(rot) * S(sx,sy)` (same order as Spine: translate, rotate, scale):

```
(s, c) = sinCosTurns(rot)
a = c*sx    b = s*sx    c' = -s*sy    d = c*sy    e = x    f = y
```

World matrix `W_i = W_parent * L_i` (parent first applied last). With `P = (a1,b1,c1,d1,e1,f1)`, `L = (a2,b2,c2,d2,e2,f2)`:

```
a = a1*a2 + c1*b2        c = a1*c2 + c1*d2        e = a1*e2 + c1*f2 + e1
b = b1*a2 + d1*b2        d = b1*c2 + d1*d2        f = b1*e2 + d1*f2 + f1
```

This is exactly `gl-matrix` `mat2d.multiply(out, parent, local)` (layout `[a,b,c,d,tx,ty]`, `multiply(out,a,b)` = `a*b`, so `b` is applied first) [V: https://glmatrix.net/docs/module-mat2d.html], which makes it a direct test oracle.

Inverse (needed for gaze and surface-local queries), `det = a*d - b*c`:

```
a' =  d/det    b' = -b/det    c' = -c/det    d' = a/det
e' = -(a'*e + c'*f)           f' = -(b'*e + d'*f)
```

`inherit` modes (subset of Spine's): `normal` (above); `translation` (world linear part = local linear part, world translation = `P.apply(e2,f2)`; used for pupils that must not roll with a rotating head, shadows); `noScale` (replace `P`'s linear part by its column-normalised version: `(a,b)/sqrt(a^2+b^2)`, `(c,d)/sqrt(c^2+d^2)`; used so squash on the body does not squash limbs). Skip shear entirely (squash/stretch via scale is enough); no reflection: horizontal flip is a wrapper `scale(-1,1)`.

Pose = rest `(x0,y0,rot0,sx0,sy0)` plus deltas, Spine-style: `x = x0 + dx`, `rot = rot0 + drot`, `sx = sx0 * msx` (animation channels are deltas/multipliers relative to rest, which is what makes additive layering trivial).

### 1.4 Skinning choice: rigid attachments + three small deformables

Mesh skinning (linear blend skinning `v' = sum_k w_k * W_k * B_k^-1 * v`) needs triangulated meshes, weight painting tools and flattening of Beziers; SVG elements cannot be mesh-skinned without regenerating path data every frame. For flat vector mascots with few joints it buys little. Recommendation:

1. **Rigid attachment**: each slot holds one attachment under exactly one bone: `path` (SVG path data), `ellipse`, `rect` (rounded), `circle`. Attachment has a *static local transform* (x, y, rotation, scale) baked into the child SVG element once; the slot `<g>` receives the bone's world matrix per tick.
2. **`limb`**: tapered capsule between the origins of two bones (start radius, end radius). Gives smoothly bent legs/arms without skinning; built from the two bone world positions in the draw list.
3. **`ribbon`**: smooth path through N bone origins (tail, sun rays, antenna, curtain, smoke). Uniform Catmull-Rom to cubic Bezier: for consecutive points `p0..p3`, segment `p1->p2` has control points `c1 = p1 + (p2 - p0)/6`, `c2 = p2 - (p3 - p1)/6` (only `+ - * /`; endpoints duplicated).
4. **`morph`**: path with fixed topology and named blend shapes: `d = base + sum_k w_k * delta_k` (mouth open/smile, eyelid curve, window shutters). The core only emits `w_k`; building the `d` string is the render target's job (so no float-to-string in the bit-exact core).

Strings never appear in the core: it outputs numbers; targets format them.

### 1.5 Draw order and slots

- Slots in array order = draw order (back to front), per pet; pets are sorted among each other by foot `y` (lower on screen in front).
- Per-slot runtime state: `attachment` (index, -1 = hidden), `alpha` in [0,1]. Blink uses an eyelid slot (skin-coloured ellipse over the eye) whose bone `scaleY` goes 0 -> 1; pupil clip is optional because pupil travel is ellipse-bounded.
- `drawOrder` keyed offsets (Spine) only if a clip must swap two slots (e.g. arm in front/behind); likely unnecessary in v1.
- Skins = named attachment sets (species variants/themes, accessories). Colours are **not** in the rig: attachments carry class names (`fill-body`, `stroke-line`), resolved by the external stylesheet via CSS custom properties.

### 1.6 Minimal schema of our own (inspired by the above)

Schema-first (JSON Schema in the repo's `🔣️.json` convention); TS and Rust types generated/hand-mirrored. Illustrative instance:

```jsonc
{
  "species": "sun",
  "unit": "px",                       // 1 unit = 1 CSS px at scale 1
  "bones": [                          // parent-first
    { "name": "root",  "parent": -1, "length": 0,  "x": 0, "y": 0,   "rotation": 0, "scaleX": 1, "scaleY": 1, "inherit": "normal" },
    { "name": "hip",   "parent": 0,  "length": 0,  "x": 0, "y": -22, "rotation": 0 },
    { "name": "body",  "parent": 1,  "length": 24, "x": 0, "y": -4 },
    { "name": "eyeL",  "parent": 2,  "length": 0,  "x": -9, "y": -6 },
    { "name": "pupilL","parent": 3,  "length": 0,  "inherit": "translation" }
  ],
  "slots": [                          // draw order, back to front
    { "name": "rays",   "bone": "body",   "attachment": "rays" },
    { "name": "body",   "bone": "body",   "attachment": "disc" },
    { "name": "eyeL",   "bone": "eyeL",   "attachment": "white" },
    { "name": "pupilL", "bone": "pupilL", "attachment": "dot" },
    { "name": "lidL",   "bone": "eyeL",   "attachment": "lid" }
  ],
  "skins": { "default": { "body": { "disc": { "type": "ellipse", "rx": 24, "ry": 24, "class": "fill-body" } } } },
  "constraints": [                    // evaluated in array order, interleaved with bone updates
    { "type": "ik",     "bones": ["thighL","shinL"], "target": "footL", "bend": "forward", "mix": 1 },
    { "type": "look",   "eye": "eyeL", "pupil": "pupilL", "radii": [3, 2.5], "saturation": 120, "headParallax": 0.35 },
    { "type": "spring", "bone": "ray0", "omega": 13.8, "zeta": 0.35, "influence": 1, "gravity": 0.2 },
    { "type": "blink",  "lid": "lidL" }
  ],
  "animations": {                     // keyed deltas relative to rest
    "fidget-stretch": { "duration": 0.9, "loop": "once",
      "bones": { "body": { "scale": [ { "time": 0, "x": 1, "y": 1 }, { "time": 0.3, "x": 0.96, "y": 1.06, "curve": [0.3, 0, 0.2, 1] } ] } },
      "events": [ { "time": 0.45, "name": "emote:happy" } ] }
  },
  "locomotion": { "gait": "walk", "stride": 22, "stepLift": 6, "speed": [24, 48], "hopApex": [28, 48] },
  "personality": { "energy": 0.6, "sociability": 0.7, "curiosity": 0.5, "rivalry": { "cloud": 0.6 } }
}
```

Rules: defaults omitted as in Spine; `curve` = CSS-style normalised cubic Bezier `[x1,y1,x2,y2]` (easing of the *fraction* between two keys), absent = linear, `"stepped"` allowed; rotations in degrees at authoring; `inherit` in `{normal, translation, noScale}`; constraint `order` = array order. Compile step (outside the bit-exact core) flattens to struct-of-arrays `Float64Array` rig (bone parents `Int32Array`, rest transforms, curve tables). Species data (rig, personality, affinity) are *domain-specific extensions*; skeleton/motion/behaviour/world are *domain-neutral framework*.

---

## 2. Procedural animation with a deterministic, bit-exact twin

### 2.0 Which math is safe (research result)

What is guaranteed:
- **IEEE-754 binary64 `+ - * / sqrt` (and `fma`) are correctly rounded** and deterministic; Rust documents `sqrt` as "guaranteed to be the rounded infinite-precision result ... guaranteed not to change" [V: https://doc.rust-lang.org/std/primitive.f64.html]. ECMAScript Number arithmetic is IEEE binary64 round-to-nearest-even; `Math.sqrt`, `abs`, `floor`, `trunc` are exact (the implementation-approximated list in ECMA-262 covers acos..tanh, exp, log, pow, hypot, cbrt, sin, cos, tan, atan2 etc. but not sqrt) [S: https://github.com/jjgroenendijk/sunset-driver/issues/471 states "Only Math.sqrt, Math.abs, Math.round, Math.floor and plain arithmetic are exact by the standard"; my attempt to fetch the spec text returned only the table of contents, so the exact wording is **not** directly verified; https://tc39.es/ecma262/multipage/numbers-and-dates.html]. WebAssembly float arithmetic follows IEEE-754 with round-to-nearest-even; the only specified nondeterminism is NaN payload/sign and relaxed-SIMD [V: https://webassembly.github.io/spec/core/exec/numerics.html, https://github.com/WebAssembly/design/blob/main/Nondeterminism.md].

What is **not** guaranteed:
- MDN: "Many `Math` functions have a precision that's implementation-dependent ... different browsers can give a different result. Even the same JavaScript engine on a different OS or architecture can give different results!" [V: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math]. ECMA-262 permits "some latitude" for the approximated functions [S].
- Rust: `sin`, `cos`, `atan2`, `exp`, `powf`: "The precision of this function is non-deterministic. This means it varies by platform, Rust version, and can even differ within the same execution from one invocation to the next"; `atan2/exp/powf` currently map to the platform libc [V: Rust f64 docs]. On `wasm32-unknown-unknown` math comes from the `libm` crate (a MUSL port) through compiler-builtins [S: https://github.com/rust-lang/libm]; linking can silently swap libm for the bundled port [S: https://github.com/rust-lang/rust/issues/142119].
- Observed in the wild: Chromium 141 `sin/cos` differ from Node; Chrome for Testing 153 additionally differs in `tan, exp, log, atan, asin, acos, atan2, cbrt, sinh, cosh` [S: https://github.com/jjgroenendijk/sunset-driver/pull/472]; V8 `Math.pow` calls `std::pow` since V8 11 [S]; glibc/musl/macOS/UCRT are faithful (< 1 ULP) but not correctly rounded and disagree [S: https://zenn.dev/mod_poppo/articles/libm-precision?locale=en]. IEEE-754 only *recommends* correctly rounded transcendental functions.
- Two ports that solved exactly our problem: sunset-driver's `libm.ts` (fdlibm methods, only exact ops, accuracy 1-3 ULP) [S: PR 472] and thevesperbell's fdlibm 5.3 port with 400+ golden bit patterns, within 1 ULP of `Math` on 10 000 random inputs [S: https://github.com/davetashner/thevesperbell/pull/54]. fdlibm itself is the reference-quality (< 1 ULP) source [V: https://www.netlib.org/fdlibm/readme, https://www.netlib.org/fdlibm/k_sin.c (coefficients S1..S6, error bound 2^-58 on [-pi/4, pi/4])].

**Strategy (recommended):**

1. **Tier 0 (everything in the hot path):** `+ - * / sqrt abs floor`, comparisons, int<->f64 for integers < 2^53, bit casts. No FMA (`mul_add` forbidden; Rust/LLVM does not contract `a*b+c` without fast-math flags and JS cannot; still run the parity suite on **aarch64** where FMA hardware is baseline, and never build the twin with fast-math/`relaxed-simd`).
2. **Tier 1: `detmath`** implemented in terms of Tier 0 only, same source structure in TS and Rust, same coefficient literals, same operation order (explicit parentheses, no reassociation):
   - `sinCosTurns(t)`: `t -= floor(t)`; fold into [-1/8, 1/8] with exact subtractions and a sin/cos kernel swap; `x = TWO_PI * t_r` (one rounding); fdlibm `__kernel_sin` / `__kernel_cos` (S1..S6, C1..C6; copy from netlib `k_sin.c` / `k_cos.c`, including the `|x| > 0.3` branch of `k_cos`). Expected accuracy about 1 ULP.
   - `atan2Turns(y, x)`: port of fdlibm `s_atan.c` + `e_atan2.c` quadrant logic (inputs are never NaN, infinite or the zero vector by construction; guard zero explicitly); result in turns.
   - `expDet(x)` for |x| <= 60: fdlibm `e_exp.c` (`k = floor(x/ln2 + 0.5)`, `ln2HI/ln2LO`, P1..P5 rational), `2^k` built exactly from bits (`Float64Array`/`DataView` in TS, `f64::from_bits` in Rust). Used only at load/parameter-change time (spring coefficients), never per tick.
3. **Design the transcendental functions out where possible:** IK uses the sqrt-based knee point (no `acos`); look-at uses vector scaling (no `atan2`); breathing/blink phases advance as turn fractions; cubic-Bezier easing uses bisection (no `pow`); weighted random choice uses cumulative sums.
4. **Lint/CI guard:** ban `Math.(sin|cos|tan|asin|acos|atan|atan2|exp|expm1|log|log1p|log2|log10|pow|hypot|cbrt|sinh|cosh|tanh|random|fround|round|min|max|imul)` and the `**` operator in the core (TS), and `f64::(sin|cos|tan|asin|acos|atan|atan2|exp|ln|log|powf|powi|hypot|cbrt|mul_add|round|min|max|to_degrees|to_radians)` plus `.abs_sub` in the Rust core. Provide `min`/`max`/`clamp` helpers with defined NaN-free semantics (`a < b ? a : b`); `Math.round` (half toward +inf) and `f64::round` (half away from zero) differ, use `floor(x + 0.5)`; `as i32` saturates in Rust but `|0` wraps in JS, so use explicit range-checked integer conversion.
5. **Float hygiene:** guard every division and `sqrt` with explicit `max(eps, .)`; treat any non-finite value as a bug (debug assertion + property test); normalise `-0` at the output boundary (`x + 0.0`); compare floats as bit patterns (`Float64Array` view / `to_bits`), store golden vectors as 16-hex-digit strings, never decimals (shortest-round-trip printing differs between JS and Rust). Decimal literals in rig JSON should parse identically (both correctly rounded) but assert it with a fixture.
6. **Time and RNG:** `DT = 1/64` exactly; the world keeps an integer `tick`; the shell passes `tick{n}` only. RNG: existing `Mt19937` (`🧰️framework/🛍️products/❓️quiz/🔨️modules/🎲️randomness/🟦️.ts`, `init_genrand`, matches numpy `_legacy_seeding` + `random_raw` vectors in `mt19937-numpy-vectors.py`). Doubles via `genrand_res53`: `((a >>> 5) * 67108864 + (b >>> 6)) / 9007199254740992` (all intermediate values exact), which equals numpy's `random_sample` for the legacy stream. One stream per pet: `seed_i = fnv1a32(worldSeed + ":" + petId)`; pair decisions use the lower id's stream; draws are counted so replays can assert consumption.

### 2.1 Gaze: look-at / aim constraint (eyes, pupils clamped to an ellipse)

Per eye with socket bone `E` (world matrix `W_E`), pupil bone `Pu` (child, `inherit: translation`), ellipse radii `(Rx, Ry)`, saturation distance `S` (socket units, [D] 120 px):

```
d  = inverse(W_E) * target            // target = pointer or attention point, viewport coords
u  = d / S                            // componentwise
m  = sqrt(ux*ux + uy*uy)
k  = m > 1 ? 1/m : 1                  // hard clamp to the unit disc ...
off*= (Rx*ux*k, Ry*uy*k)              // ... which maps to the ellipse (inside-ellipse test u.u <= 1)
```

Soft variant without a clamp branch: `r = sqrt(dx*dx + dy*dy); g = r/(r + Dh); n = d/max(r, eps); off* = (Rx*g*nx, Ry*g*ny)` (strictly inside the ellipse; `Dh` ~ 40 px).

Then: pupil position follows `off*` with a critically damped spring (halflife [D] 0.05 s); target is only re-latched every 120-250 ms or if it moved > 6 px (saccade-like hold, avoids jitter); face features translate `headParallax * off` (0.3-0.5) and the head rotates `+- 0.012 turns * ux` (pseudo-3D from a flat character). Attention source priority: pointer moved within 3 s > other pet that started a social action > random "interesting point" (nearest surface end, a card). When no pointer (touch): last touch point for 2 s, then neutral. Under reduced motion see 5.5.

### 2.2 Blinking

Human data [S, search summaries, not independently verified; PMC4043155 is about *voluntary* blinks]: spontaneous rate roughly 11-19/min; blink 150-400 ms with the eye fully closed about 50 ms; closing about 82 ms, opening about 176 ms (closing is faster); inter-blink intervals right-skewed (https://pmc.ncbi.nlm.nih.gov/articles/PMC4043155/, https://journals.plos.org/plosone/article?id=10.1371%2Fjournal.pone.0196125, https://link.springer.com/article/10.3758/s13428-023-02333-9). The double-blink probability is my assumption.

```
next interval T = 1.6 + 5.4*u*u            // u = genrand_res53; right-skewed, mean 3.4 s            [D]
with p = 0.12: second blink after 0.22 s (double blink); also p = 0.3 blink on gaze jumps > 30 deg      [D]
phase times: close Tc = 0.09 s, hold Th = 0.04 s, open To = 0.18 s                                     [D]
closing p = t/Tc:   c = p*p*(3 - 2p)          opening q = (t-Tc-Th)/To:   c = 1 - q*q*(3 - 2q)
lid.scaleY = c                                 (or eye.scaleY = 1 - 0.9*c)
```

Only `* + - /` needed. Suppress blinks while sleeping (lids closed), during a squabble stare (longer interval), and in mode `still`.

### 2.3 Breathing, squash and stretch

Phase in turns: `phi += f*DT; phi -= floor(phi)`; `s = sinTurns(phi)`; `sy = 1 + A*s; sx = 1/sy` (area-preserving: squash widens, stretch narrows). Pivot at the base/feet bone so contact stays planted. [D] idle `f = 0.28 Hz, A = 0.02`, after activity `f = 0.6 Hz` decaying to idle over 3 s, sleeping `f = 0.15 Hz, A = 0.03`. At <= 15 fps this stays smooth (amplitude about 1.6 px over 4 s).

Impact squash: an underdamped spring on a scalar `squash` (goal 0) with initial velocity `-kappa * vyImpact` (kappa [D] chosen so peak squash <= 0.22), `zeta ~ 0.35, omega = 2*pi*3.2`; stretch in flight: `sy = 1 + 0.08 * min(|vy|/vmax, 1)`. Anticipation before hops (Disney principles: squash and stretch, anticipation, follow-through/overlap, slow in/slow out [S: https://www.creativebloq.com/advice/understand-the-12-principles-of-animation]): crouch to `squash = -0.15` over 0.12 s, then launch.

### 2.4 Critically damped and general damped springs

Closed-form (Juckett, "Damped Springs") [V: https://www.ryanjuckett.com/damped-springs/]. For state `(x, v)` relative to goal (`x := pos - goal`), angular frequency `omega`, damping ratio `zeta`, step `dt`:

```
x' = posPos*x + posVel*v
v' = velPos*x + velVel*v

zeta == 1 (|zeta-1| < 1e-6):  e = exp(-omega*dt)
   posPos = e*(1 + omega*dt)   posVel = e*dt
   velPos = -e*omega*omega*dt  velVel = e*(1 - omega*dt)

zeta < 1:  alpha = omega*sqrt(1 - zeta*zeta);  e = exp(-zeta*omega*dt);  c = cos(alpha*dt);  s = sin(alpha*dt);  g = zeta*omega/alpha
   posPos = e*(c + g*s)        posVel = e*s/alpha
   velPos = -e*s*(alpha + zeta*zeta*omega*omega/alpha)     velVel = e*(c - g*s)

zeta > 1:  z1 = -omega*zeta - omega*sqrt(zeta*zeta - 1);  z2 = -omega*zeta + omega*sqrt(zeta*zeta - 1);  e1 = exp(z1*dt);  e2 = exp(z2*dt);  k = 1/(z1 - z2)
   posPos = (z1*e2 - z2*e1)*k  posVel = (e1 - e2)*k
   velPos = z1*z2*(e2 - e1)*k  velVel = (z1*e1 - z2*e2)*k
```

(Derived from Juckett's `x(t)`/`v(t)` formulas; cross-checked algebraically here, and to be validated against `scipy.linalg.expm`, see 7.) Because `DT` is constant, the four coefficients are computed **once** per spring at load with `detmath` and the per-tick update is 4 multiplies and 2 adds: unconditionally stable for any `omega`, no `exp/sin/cos` in the loop.

Holden's halflife parameterization (critical damping) [V: https://theorangeduck.com/page/spring-roll-call]: `damping d = 4*ln2 / halflife`, `y = d/2`, `x(t) = e^{-y t} (j0 + j1 t) + goal`, `j0 = x - goal`, `j1 = v + j0*y`. Use halflife in the rig for intuitive tuning ("halflife 0.05 s = snappy eye, 0.25 s = lazy lean"). Holden's `fast_negexp(x) = 1/(1 + x + 0.48x^2 + 0.235x^3)` is a pure-`+ * /` approximation of `e^{-x}` (about 0.1 % class error); acceptable for visual springs and trivially bit-exact, but prefer `detmath.exp` coefficients so oracles can check against exact math.

Semi-implicit (symplectic) Euler, only where closed form is unavailable (contacts, clamped springs): `v += (omega^2*(goal - x) - 2*zeta*omega*v)*dt; x += v*dt;` stable for roughly `omega*dt < 2` and `zeta*omega*dt < 1` (at `dt = 1/64`: `omega < 128 rad/s`); it adds energy-drift/phase error and requires a limit on stiffness. Rule: closed form by default.

### 2.5 Two-bone analytic IK (legs) and FABRIK (tails, necks)

**Two-bone** (law of cosines) [V: https://www.ryanjuckett.com/analytic-two-bone-ik-in-2d/ gives the angle form `cos(theta2) = (x^2 + y^2 - d1^2 - d2^2)/(2 d1 d2)`, `theta1 = atan2(y, x) - atan2(d2 sin(theta2), d1 + d2 cos(theta2))`, clamp `cos` to [-1,1] when out of reach]. Sqrt-only form (no `acos`):

```
d = target - root;  D2 = d.d;  D = sqrt(D2)
u = D > eps ? d/D : (1, 0)
Dc = clamp(D, |l1 - l2| + eps, l1 + l2 - eps)
a  = (Dc*Dc + l1*l1 - l2*l2) / (2*Dc)          // knee projection on the axis
h  = sqrt(max(0, l1*l1 - a*a))                 // knee height off the axis
n  = (-u.y, u.x)                               // perpendicular (y-down)
knee = root + u*a + sigma*n*h                  // sigma = -facing for a leg hanging down (knee forward)
end  = root + u*Dc
theta1 = atan2Turns(knee - root);  theta2 = atan2Turns(end - knee)
local1 = wrap(theta1 - worldRot(parent));  local2 = wrap(theta2 - theta1);   wrap(x) = x - floor(x + 0.5)
```

`sigma` is fixed per leg from the facing direction (no flipping at full extension; add hysteresis if facing flips). IK parents must have uniform scale (horizontal flip lives on the wrapper). `end` is the planted foot position when reachable; if the target is unreachable the leg stretches straight toward it and the stepper forces a step.

**FABRIK** (Aristidou and Lasenby 2011, Graphical Models 73(5); https://www.andreasaristidou.com/publications/papers/FABRIK.pdf [S: abstract/metadata only]). Joint positions `p_0..p_n`, segment lengths `d_i`:

```
if |t - p_0| >= sum(d_i): straighten toward t
else repeat K times (fixed K = 6, no tolerance exit; deterministic cost):
  backward: p_n = t;      for i = n-1 .. 0: r = |p_{i+1} - p_i|; lam = d_i / max(r, eps); p_i = (1 - lam)*p_{i+1} + lam*p_i
  forward:  p_0 = root;   for i = 0 .. n-1: r = |p_{i+1} - p_i|; lam = d_i / max(r, eps); p_{i+1} = (1 - lam)*p_i + lam*p_{i+1}
then bone rotations from consecutive differences via atan2Turns.
```

Use FABRIK for tails/necks (3-6 bones) and ribbon-driving chains; two-bone for legs.

### 2.6 Procedural walk cycle (foot planting, step triggers)

Pattern used by procedural creature animation [V: https://blog.littlepolygon.com/posts/loco2/ (pinned feet, phase-offset legs, step arc `9.4815*(1-x)^3*x`, velocity extrapolation of foot target); S: https://weaverdev.io/projects/proc-anim-tutorial/ (`wantStepAtDistance`, `moveDuration`; page returned 403, search snippet only); S: Rain World process https://www.gamedeveloper.com/art/video-animating-i-rain-world-i-and-its-many-squishy-stretchy-creatures]:

```
per foot i:   home_i = hipWorld_i + facing*(stanceOffset_i, 0), projected on surface y;  plant_i (world, fixed while planted)
trigger:      planted && |plant_i.x - (home_i.x + lead)| > trigger (0.35*stride)  &&  partner is planted or its swing progress >= 0.6
start step:   from = plant_i;  to = (home_i.x + lead, surfaceY);  lead = clamp(v*Tstep*0.5, +-stride);  Tstep = 0.22 s
during step:  s = (tick - t0)*DT/Tstep;  e = s*s*(3 - 2s);  pos = from + (to - from)*e;  pos.y -= stepLift * (256/27)*(1 - s)^3 * s   // arc peaks at s = 1/4 with value 1, soft landing
end:          plant = to; fire "step" event
emergency:    if |plant - hip| > l1 + l2 - eps  -> forced step regardless of partner
body:         hip height = surfaceY - L0 + bobAmp * (arcL + arcR) * 0.5, spring-smoothed (halflife 0.08 s)
```

Species gaits ([D]): `walk` (legs: solar panel, heat pump, radiator, battery), `hop` (house, wall, window; anticipation -> ballistic launch -> stretch -> impact squash spring -> recover), `float` (sun, cloud: bob `sinTurns` at 0.2 Hz, drift along the lane with a lean spring; no feet). Calm speeds 24-48 px/s, lively up to ~90 px/s; acceleration limited by a spring on velocity (smooth onset, see 3.5).

### 2.7 Secondary motion (follow-through, spring bones)

Reference: VRM spring bones (stiffness, drag, gravity, verlet) [S: https://github.com/vrm-c/vrm-specification/blob/master/specification/VRMC_springBone-1.0/README.md]; Spine 4.2 physics constraint fields `inertia, strength, damping, massInverse, wind, gravity, mix, step` [S: https://github.com/EsotericSoftware/spine-runtimes/blob/4.2/spine-csharp/src/PhysicsConstraintData.cs]. Deterministic version ("tip chaser"):

```
tipRest = W_bone * (length, 0)                       // from the pose before the constraint
q  (tip position), v: spring toward goal = tipRest + (wind*..., gravity*length)  using 2.4 coefficients (x and y independently, zeta ~ 0.35)
q  = pivot + (q - pivot) * L / max(|q - pivot|, eps)   // keep length (or allow +-8 % stretch)
bone.rot = lerp(restRot, atan2Turns(q - pivot) - worldRot(parent), influence)
```

Applied to sun rays, antenna, house chimney smoke, cloud edges, panel cable. Inertia input comes for free because `tipRest` moves with the body.

### 2.8 Keyframe clips, easing, layering, blending

- Clip = tracks of keys `{time, value(s), curve}` per bone channel (`rotate` turns, `translate` px, `scale` multipliers), slot tracks (`attachment`, `alpha`), events, loop mode (`once|loop|pingpong`). Sampling: cursor per track; `f = (t - t0)/(t1 - t0)`; `v = v0 + (v1 - v0)*ease(f)`.
- **Cubic-Bezier easing** (CSS semantics, `P0=(0,0)`, `P3=(1,1)`, control `(x1,y1),(x2,y2)`, `x1,x2` in [0,1] so `x(s)` is monotone): `x(s) = 3(1-s)^2 s x1 + 3(1-s) s^2 x2 + s^3`, `y(s)` analogous; solve `x(s) = f` by **40 bisection steps** (deterministic, always converges, about 1e-12) then `ease = y(s)`. Newton is faster but needs the same fixed iteration discipline; not needed at this scale. Reference implementations: `bezier-easing` (Newton + subdivision + sample table, mirrors CSS `cubic-bezier`; so expect agreement only to roughly 1e-6 not bit level) [V: https://github.com/gre/bezier-easing].
- **Layers** evaluated in fixed order, each emitting delta channels with weight `w` and per-bone mask `m`:
  - additive: `x += w*m*dx; rot += w*m*drot; sx *= 1 + w*m*(msx - 1)`;
  - override: `x = lerp(x, L.x, w*m)`; rotation lerp via `x + w*m*wrap(L.rot - x)` (shortest arc in turns).
  Order: base clip (idle / locomotion pose) -> additive breathing + squash springs -> fidget clip (override on masked bones) -> procedural gaze/blink writes -> constraints (feet IK, spring tip chasers) -> FK. Spine-style update order list interleaves bones and constraints so each solver runs once its bones/targets have world matrices.
- **Transitions between states**: prefer **inertialization / dead blending** over cross-fading two evaluated clips: store the pose and velocity offset at the transition and decay it with a critically damped spring (or exponential decay `v *= exp(-0.693*dt/halflife)`), which needs one clip evaluated and composes with procedural layers [V: https://theorangeduck.com/page/dead-blending]. Fallback: smoothstep cross-fade `s*s*(3 - 2s)` over 0.15-0.25 s.

---

## 3. Behaviour model

### 3.1 How desktop pets do it

| Pet | Model (source) | Takeaways for us |
|---|---|---|
| **oneko / neko** | Cat chases the cursor; when it catches it runs an idle chain (sit, groom at wall, paw, scratch, yawn, curl up, sleep) and wakes with a startle when the cursor moves [S: https://manpages.debian.org/unstable/oneko/oneko.6.en.html, https://deepwiki.com/crgimenes/neko]. | Tiny state machine + a charming *idle chain*; sleep gated by input. Chasing is the "annoying" part: ours only looks. |
| **Shimeji-ee** | `behaviors.xml`: `BehaviorList` with nested `Condition` blocks ("On the Floor", ceiling, ...); each `Behavior` has `Frequency` (weight; 0 = only reachable via chaining, `Hidden`), `NextBehaviorList` with `Add="true|false"`; actions `Stay/Move/Animate/Sequence/Select/Embedded`; embedded classes `Broadcast`, `ScanMove`, `Interact`, `Look`; conditions are expressions (e.g. `#{mascot.totalCount < 50}` caps multiplication); environment = work area plus active windows [V: https://kilkakon.com/shimeji/affordances.php, https://git.dcsl.dk/General/shimeji-collection/src/tag/1.1/conf/behaviors.xml; S: https://github.com/TigerHix/shimeji-ee/blob/master/conf/behaviors.xml]. Sample floor weights: StandUp 200, SitDown 200, Walk 100, Run 100, Crawl 10, LieDown 30, SitWhileDanglingLegs 20. | Weighted random choice among condition-eligible behaviours is the right core; the **affordance handshake** (A broadcasts, B scans and walks over, both `Interact` and switch to new behaviours) is the right social protocol. |
| **eSheep / web-eSheep** | XML of sprite animations with spawn points, probabilistic `nextAnimation` graph, falls onto the taskbar and detects windows [S: https://github.com/Adrianotiger/desktopPet, https://github.com/Adrianotiger/web-esheep]. | Animation graph with probabilities; data-driven, shipped inside one file. |
| **VS Code Pets** | Per-pet `sequence { startingState, sequenceStates[ {state, possibleNextStates} ] }`; random next state; "friends" show a heart then chase each other; ball toy [S: https://github.com/tonybaloney/vscode-pets/blob/main/src/panel/pets/dog.ts, https://deepwiki.com/tonybaloney/vscode-pets]. | Per-species transition tables are enough for a lot of charm; friendship = a state pair. |
| **Desktop Goose** | Intentionally mischievous: steals the cursor, drags windows, leaves notes/memes; aggression level 1-3, `CanAttackMouse` toggle; widely flagged as "malware-like" [S: https://samperson.itch.io/desktop-goose, search summary]. | The anti-pattern: never take control of pointer, focus or content; always provide an intensity knob and an off switch. |

### 3.2 Recommendation: layered, utility-weighted FSM (not a behaviour tree, not pure utility)

- **FSM per pet** for explicit, testable, replayable states: `(state, event) -> (state', commands)` maps directly to CQRS/event sourcing and to a reducer; states carry `minDur/maxDur`, `cooldown`, `interruptPriority`, entry conditions. Behaviour trees add structure we do not need and keep per-node running state that complicates snapshots/replay; pure utility selection flaps without commitment.
- **Utility used only for choosing the next state** among eligible ones (Dave Mark's "considerations" with response curves and a weight; https://en.wikipedia.org/wiki/Utility_system, https://www.gdcvault.com/play/1018040/Architecture-Tricks-Managing-Behaviors-in): `score = baseWeight * mode(mood-mult) * prod(consideration_k(need_k)) * (current ? 1.3 : 1)`, then Shimeji-style weighted random pick (cumulative sum with `u*total`). Decisions happen only when a state ends or an event preempts, so there is no per-tick decision cost and a natural `nextWake`.
- **Continuous layers outside the FSM**: gaze (attention), blink, breathing, secondary motion. This keeps the FSM small (about 12 states).
- **Global activity governor** (the "calm default"): caps on simultaneously moving pets, moving duty cycle, and event rates (3.6).

### 3.3 States

Locomotion/pose FSM: `idle` (stand or sit; includes micro-motions), `fidget` (short clips: stretch, wiggle, shake, yawn, look-around), `walk`, `hop` (same-surface hop or jump to another surface), `fall`, `land` (recover), `sleep` (+ `wake`), `peek`/`hide` (optional), `dragged` (**reserved but disabled**: pets are `pointer-events: none`; dragging only via an explicit "play mode" toggle that enables hit-testing on pets, see 6.4).
Social overlay: `notice` -> `greet`, `cuddle` (like), `squabble` (dispute) -> `sulk` -> `reconcile`; domain extras (battery + solar panel "charging" duet).
Cursor: `look` (gaze only, always on), optional `approach-cursor` / `flee-from-cursor` for specific personalities in `lively` only (off by default; "flee" is a shy species trait that triggers only after the pointer lingers within 120 px for > 0.6 s).

Interrupt priority (high -> low): `fall` > `social response/greet` > `flee` > `walk/hop` > `fidget` > `idle/sleep`; surface loss, pokes, modal opening and mode changes preempt.

### 3.4 Needs, moods, affinity

- Per-pet scalars in [0,1], fixed-rate linear drift per tick (no `exp`): `energy` (-0.004/s active, +0.02/s asleep), `sociability` (+0.006/s alone, drops after an interaction), `curiosity` (+0.01/s idle, reset by attending to something new), `calm` (rises after a squabble's sulk, decays) [D].
- Response curves: `x^2`, `1 - x`, smoothstep, piecewise-linear (only `* + -`). Example: `sleep` considerations `(1 - energy)^2` and `userIdleSeconds/60` capped; `greet` considerations `sociability`, `(0.5 + 0.5*affinity)`, distance falloff.
- **Pairwise affinity** `A[i][j] in [-1, 1]`: species base value from extension data + per-instance seeded jitter + dynamic offset: `+0.10` after a hug/greet, `-0.15` after a squabble, `+0.10` after reconcile, drifting back to base with a ~10 min time constant. Asymmetric allowed (cloud annoys sun more than sun annoys cloud). Clamp so disputes stay playful: `A >= -0.6`, a squabble always resolves into `sulk` (<= 8 s) then `reconcile`.
- Example domain data ([D], energy-domain humour): sun-cloud (teasing rivals), solar panel-sun (adores), solar panel-battery (charging duet, high affinity), radiator-heat pump (friendly competition), window-wall (neighbours), house (welcoming host to all).
- Choice probabilities when two pets meet within 400 px and both are interruptible: `P(greet) ~ sociability * (0.5 + 0.5*A)`, `P(cuddle) ~ max(0, A)^2`, `P(squabble) ~ rivalry[a][b] * max(0, 0.3 - A)`; at most one social event in the whole world at a time.

### 3.5 Social protocol (event-driven, Shimeji affordance style)

`social-offer{from, to, kind}` (broadcast) -> target decides on its next decision point using its state interruptibility, mood and affinity -> `social-accept` / `social-decline` -> a **two-party script** with phases, sync barriers and a timeout (phases are clips plus approach/face constraints), e.g. squabble: `face-off (2 s) -> puff + shake fist (2 s) -> turn away -> sulk`; cuddle: `approach -> lean together (hearts, 4 s) -> separate`. If either party is preempted, the script aborts cleanly (both return to `idle`). All messages are ordinary events appended in deterministic order (tick, then pet id).

### 3.6 What makes pets charming rather than annoying (synthesis, [D] unless sourced)

1. **Calm default**: mostly idle with micro-motions; long dwell times (6-20 s) between locomotion bursts.
2. **Frequency caps** (starting values): `calm`: <= 1 pet moving at once, moving duty cycle <= 8 %, speeds <= 48 px/s, no social events; `lively`: <= 2 moving, duty <= 25 %, social event per pair >= 75 s apart and one active social event world-wide with >= 20 s gaps, each lasting 4-10 s.
3. **Smooth motion onset**: motion *onset* captures attention (Abrams and Christ 2003, https://journals.sagepub.com/doi/abs/10.1111/1467-9280.01458), but follow-up work finds onset does not capture attention when subsequent motion is smooth (https://link.springer.com/article/10.3758/s13423-011-0152-3). So every locomotion start has anticipation (a lean) and an ease-in of >= 250 ms; no jerky starts; no sudden appearances (fade 300 ms).
4. **Personality consistency** per species, rare surprises with cooldowns (variable-ratio feels alive), anticipation/follow-through (Disney principles).
5. **Responsive but unobtrusive**: acknowledge the pointer with a glance within 200-300 ms; never chase it across the page.
6. **Never block, never steal**: pointer-events none, no focus, no layout changes, no sound.
7. **Controllable**: off / still / calm / lively + count (6.3).

---

## 4. Walking on UI

### 4.1 How existing pets treat windows/elements as platforms

- Shimeji-ee models an environment with the screen work area plus (formerly) the active Internet Explorer window; behaviours are conditioned on being on the floor, a wall, the ceiling, or the window's top border (conditions like "On the Floor", ceiling crawl; `JumpFromBottomOfIE` etc.) [V: behaviors.xml listing above; S: https://github.com/DalekCraft2/Shimeji-Desktop (ported to modern JDK)]. eSheep detects windows and falls to the taskbar [S].
- The web has no window manager, so surfaces must be derived from the DOM.

### 4.2 Representation

A surface is a **horizontal segment in viewport coordinates**: `{id, x0, x1, y, clear (headroom px), flags}` (optional vertical walls are out of scope for v1). The shell publishes the *entire current set* with `surfaces-changed{surfaces[], keepOut[]}` (an epoch number guards against stale messages); the reducer reconciles by `id`. Pets store `(surfaceId, u)` with `u = x - x0` so a standing pet rides along when the surface scrolls or moves.

### 4.3 Sourcing surfaces (opt-in, not DOM-wide scanning)

- Authors mark `data-pet-surface="<id>"` on cards/panels and `data-pet-lane` on dedicated empty strips (recommended: a reserved lane below the header, so "walk on the header" means walking on the lane directly beneath its bottom edge, where the page margin provides headroom). Buttons are allowed only if they are explicitly marked and have headroom.
- Reasoning: a pet standing on a top edge occupies the space *above* the element; in a dense UI that space is the previous card's content. Hence **headroom test**: the rectangle `[x0..x1] x [y - petHeight, y]` must not intersect keep-out rectangles; subtract blocked x-intervals from the edge (1-D interval subtraction) and keep spans wider than `minWidth` (about 1.5 pet widths).
- Keep-out rectangles (inflated by 4 px): interactive controls (`a[href], button, input, select, textarea, summary, [role=button|link|checkbox|radio|tab|menuitem|slider|switch], [tabindex], [contenteditable]`), text blocks (`p, li, h1-h6, label, figcaption, td, th, legend, dt, dd, blockquote, pre`), media with alt, the **focused element** (plus 8 px halo), open popovers/menus/tooltips (`[popover]:popover-open`, `[role=menu|listbox|tooltip]`), the whole task panel while a run is active, and any `data-pet-keepout`. The surface's own element and its ancestors are excluded.

### 4.4 Observation and update cost

- Mark dirty on: `ResizeObserver` (registered surfaces + `documentElement`), `MutationObserver` (childList, subtree, attributes `class, hidden, open, aria-expanded, data-pet-*`, debounced), passive capture `scroll` listener (nested scrollers), `resize`, `visibilitychange`, `focusin/out`, `transitionend`, run/modal state events.
- Re-measure in **one rAF batch, reads before writes**: `getBoundingClientRect()` for registered surfaces + keep-out candidates; coalesce to <= 4 Hz while idle, immediately on scroll (scroll only changes `y`; optional fast path `scrolled{dx,dy}` shifts everything without layout reads). `IntersectionObserver` (`rootMargin` ~ 100 px) tells which surfaces are near the viewport so offscreen ones are skipped; note that IO fires on threshold crossings only and gives rects at observation time, so it cannot replace re-measurement [V: https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API].
- The pure core never touches the DOM; it receives snapshots.

### 4.5 Surfaces that move or vanish

- Surface id missing after an update: pet enters `fall` with gravity `g = 1800 px/s^2` (a 100 px drop takes about 0.33 s), terminal speed 900 px/s, lands on the highest surface with `y >= foot.y` whose `[x0,x1]` contains `x` (one-way platform, swept test: land when `prevFoot.y <= s.y < foot.y` and `x` inside), else the lowest lane/floor; no surface -> fade out and `relocate` to a random valid surface (alpha ramp 300 ms).
- Surface moved vertically by `dy`: carry the pet if `|dy| <= 24 px` per update (spring-smoothed), else detach and fall. Surface shrank so the pet is off-segment: walk to the nearest edge (if `< 0.5` pet widths) else fall. Layout shifts (CLS) are debounced 100 ms to avoid fall flicker. Surface off-viewport > 1.5 s: relocate.

### 4.6 Jumps between surfaces (ballistic arc with reachability check)

Launch `p0 = (x0, y0)` to landing `p1 = (x1, y1)` (y-down, `dy = y1 - y0`), gravity `g`:

```
hUp = max(hMin, -dy + clearance)               // apex above the higher endpoint; clearance ~ 10 px
vy  = -sqrt(2*g*hUp)                           // upward
T   = (-vy + sqrt(vy*vy + 2*g*dy)) / g         // y0 + vy*T + 0.5*g*T^2 = y1
vx  = (x1 - x0) / T
reachable = hUp <= hMax (~1.5*petHeight)  &&  |vx| <= vxMax (~140 px/s)  &&  T <= Tmax (0.9 s)
            && the sampled arc (8 fixed points) stays in the viewport and clears keep-out rects at the foot level
```

Per tick under semi-implicit Euler at `DT` the trajectory deviates from the analytic curve by O(g*DT) which is fine; landing uses the swept one-way test above (no tunnelling). If surfaces changed in flight, continue the same ballistic integration and land on whatever is actually there.

### 4.7 Occlusion, z-order, and not covering anything important

- Overlay: `position: fixed; inset: 0; pointer-events: none; z-index` below the app's modal layer; native `<dialog>.showModal()` and `[popover]` live in the top layer and therefore always render above the overlay. On `modal-changed{open}` the shell drops all surfaces in `inert`/behind-modal regions; pets with no valid surface fade (calm) or sit on the floor lane.
- Because the overlay is `pointer-events: none`, `pointermove`/`pointerdown` `event.target` is the element under the pet, so "is the pointer over an interactive control beneath a pet's box" costs one `closest(selector)` and no `elementsFromPoint`. If true for > 120 ms, the pet fades to opacity 0.25 (class toggle, CSS transition) and steps away after 300 ms. For keyboard users, `focus-moved{rect}` adds the focused element to keep-out and pets move off it (**WCAG 2.4.11 Focus Not Obscured (Minimum), AA: "not entirely hidden due to author-created content"** [V: https://www.w3.org/WAI/WCAG22/Understanding/focus-not-obscured-minimum.html]).
- Pets perch on edges: bodies sit in margins/lanes, never over content text; arcs may briefly cross content (< 0.9 s) but never end there.

---

## 5. Rendering and the loop

### 5.1 CSP facts (verified)

- MDN `style-src`: inline `style` attributes, `<style>` elements without hash/nonce, `setAttribute('style', ...)` and `el.style.cssText = ...` are **blocked**; "styles properties that are set directly on the element's `style` property will not be blocked", e.g. `el.style.display = "none"` [V: https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/style-src]; `style-src-attr` behaves the same, listing `setAttribute("style")` and `cssText` as blocked and `element.style.display = "inline"` as allowed [V: .../style-src-attr]. The CSP3 text governs inline `style` attributes and `<style>` blocks via its inline check and does not mention CSSOM property writes [V: https://www.w3.org/TR/CSP3/ (no statement found)]. So the user's claim is **correct for `el.style.prop = v`**; `el.style.setProperty(name, v)` goes through the same CSSOM path and is expected to behave identically, but MDN's example names only the property form, so assert it in a test.
- SVG presentation attributes (`transform`, `d`, `opacity`, `visibility`, `fill` ...) are plain attributes, not the `style` attribute, so `setAttribute('transform', 'matrix(...)')` is not subject to `style-src(-attr)`; SVG 2 defines `transform` as a presentation attribute mapped to the CSS `transform` property [V: https://www.w3.org/TR/css-transforms-1/].
- **Repo specifics**: the current site policy (built in `🎓️teaching/🏛️architecture/❓️quiz/🏗️builder/🌐️vite/🟦️.ts`) is `default-src 'self'; script-src 'self' <hashes>; style-src 'self' <hash>; style-src-attr 'unsafe-inline'; img-src 'self' data:; connect-src <proctor>; object-src 'none'; base-uri 'none'; form-action 'self'`. Inline style attributes are *currently* allowed via `style-src-attr 'unsafe-inline'`; the pet code must not depend on that so the policy can later be tightened to `'none'`. Pets make no network requests (rig JSON is bundled), and need no `worker-src`/`blob:`.
- `fill="var(--x)"` in presentation attributes is **unreliable**: presentation attributes are not parsed as CSS declarations in SVG 2, and support differs (renders in Firefox/Safari, not in Chromium) [S: https://css-tricks.com/svg-properties-and-css/, https://lists.w3.org/Archives/Public/public-svg-issues/2025Nov/0024.html]. Therefore: class names on SVG elements + rules in the **external** stylesheet using `fill: var(--pet-body)`; per-instance theming by writing custom properties with `el.style.setProperty('--pet-hue', ...)` on the pet root (CSSOM, allowed). React's client-side `style={{...}}` prop writes through the CSSOM and is fine in the browser; never emit pet markup as server/static HTML with `style=""`.

### 5.2 Options compared (3-6 characters, ~15-25 slots each)

| Option | Per-frame cost (estimate, unmeasured) | CSP | Crispness / DPR | Theming | A11y | Verdict |
|---|---|---|---|---|---|---|
| **Inline SVG, per-pet `<svg>`, `setAttribute('transform')` per slot; wrapper positioned by CSSOM `transform`** | ~100-180 attribute writes/frame, script < 0.3 ms; style-recalc scoped to tiny subtrees; paint only the pet's small layer; wrapper motion is compositor-only | OK | Vector, crisp at any DPR/zoom, no resize code | CSS classes + custom properties, `currentColor` | `aria-hidden`, no focusables | **Primary** |
| Canvas 2D (one overlay or per-pet), `Path2D(d)` | ~0.2-0.5 ms draw for 6x20 paths, clear+redraw dirty rect; stays on last frame when idle | OK | Needs DPR scaling and redraw on resize/zoom/DPR change | Colours read from `getComputedStyle` on theme change | Decorative | Alternate target behind the same draw list (useful if SVG paint ever shows up in traces) |
| CSS transforms on DOM (div per bone) | Many elements, layout-containing absolutely positioned boxes; vector shapes need borders/clip-path | OK via `style.transform` CSSOM | OK | OK | OK | Worse than SVG for paths; no advantage |
| WebGL / OffscreenCanvas in a worker | Shader/context setup, context-loss handling, worker messaging; sim + DOM measurement still on main thread | `worker-src` fallback to `default-src 'self'` is fine | Needs MSAA/DPR work | Awkward | Decorative | Overkill for ~100 shapes; battery cost of a GPU context; reject |

Generic guidance that SVG suffers at thousands of nodes and canvas wins there while SVG is fine below hundreds [S: https://blog.logrocket.com/svg-vs-canvas/]; we are about two orders of magnitude under that. **Measure** per 5.6 before locking.

### 5.3 React integration

- `<PetLayer />` renders one static `<div class="pet-layer" aria-hidden="true">` (portal or sibling root, outside re-rendering subtrees). A single `useEffect` creates the engine (`createElementNS`, observers, loop) and disposes everything on cleanup; idempotent for StrictMode double-invocation. React never re-renders per frame; preferences/mode/run-state go in via `engine.dispatch(event)`; read-only status (pet count) via `useSyncExternalStore` if the settings UI needs it.
- Shell = the only impure code: DOM measurement, observers, rAF/timer, SVG writes. Core = pure TS/Rust.

### 5.4 Fixed-timestep loop with render interpolation

Gaffer on Games "Fix Your Timestep" [V: https://gafferongames.com/post/fix_your_timestep/]: accumulator, clamp the frame time (spiral-of-death guard), interpolate with `alpha = accumulator/dt`. Our variant with integer ticks:

```
frame(now):                         // rAF timestamp (DOMHighResTimeStamp); always use it, never assume 60 Hz  [V: MDN rAF]
  dtMs = min(now - last, 250); last = now
  acc += dtMs;  n = floor(acc / 15.625);  acc -= n*15.625;  n = min(n, 8)   // excess is dropped (time dilation), not caught up
  if n > 0: world = step(world, [..pendingEvents, {type:'tick', n}])
  alpha = acc / 15.625
  render(lerp6(prevDraw, currDraw, alpha))   // lerp the 6 matrix entries; per-tick deltas are tiny so linear matrix interpolation is visually exact
```

Interpolating matrices is presentation only and need not be bit-exact. Write DOM attributes only when the quantised value changed (round to 1/64 px).

### 5.5 Throttling, pausing, quiescence

- **Core-driven scheduling**: `step` returns in `world` a `frameRate ∈ {display, 30, 15, 0}` and `nextWake` (absolute tick of the next scheduled change, e.g. next blink start in 3 s). Walking/hopping/social -> display rate; stationary micro-motion (blink, breath, gaze) -> 30; sleeping -> 15 or 0.
- Rate 30/15: skip rAF callbacks (accumulate; `tick{n=2|4}`). Rate 0 (nothing animating): **cancel rAF**, arm one `setTimeout` for `nextWake` (converted to ms) and wake on `pointermove`/`surfaces-changed`/`preference-changed`. rAF callbacks that do nothing still keep the frame pipeline waking at 60 Hz; the timer-based sleep avoids that. Use discrete-event "next-event time advance": `tick{n}` with large `n` jumps over quiescent stretches in O(1) in the reducer.
- **Hidden tab**: rAF is paused by browsers ("paused in most browsers when running in background tabs or hidden iframes" [V: MDN rAF]) and "doesn't use any CPU when the page is hidden", whereas timers are throttled to once per second and, after 5 min hidden with chain count >= 5, once per minute [V: https://developer.chrome.com/blog/timer-throttling-in-chrome-88]. On `visibilitychange` -> `hidden` [V: https://developer.mozilla.org/en-US/docs/Web/API/Document/visibilitychange_event]: dispatch `visibility-changed{hidden:true}`, cancel rAF **and** any wake timer; on `visible`: dispatch, reset `last = now` so hidden time is not simulated. Same for `pagehide`/`freeze`.
- **IntersectionObserver** on the pet layer is pointless (fixed full-viewport); IO is used only for surfaces (4.4).
- **`prefers-reduced-motion`** [V: https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/@media/prefers-reduced-motion]: "reduce" does *not* mean "no animation", it means removing motion that can trigger vestibular disorders; read with `matchMedia('(prefers-reduced-motion: reduce)')` and subscribe to `change`. Proposed behaviour (see also 6.3):
  - `reduce` -> default mode **still**: static rest pose at fixed lane positions, no walking/hopping/falling/breathing/squash, no gaze following (pointer-driven motion is interaction-triggered, SC 2.3.3), no blinking. Optional: opacity cross-fades only (e.g. eyes closed/open pose swap every ~10 s as a 250 ms cross-fade) are acceptable *only* when the user explicitly picks `calm`.
  - In-app explicit choice wins for `calm`, but `lively` is not offered while `reduce` is active.
- **`Save-Data`** (`navigator.connection.saveData`; Chromium-only, "not Baseline" [V: MDN NetworkInformation.saveData]): treat as a hint to default to `calm`; **Battery Status API**: limited, secure-context only, not in Baseline, privacy-sensitive [V: MDN Battery Status API]; do not use. Also hide pets under `forced-colors: active` (decorative colours would be overridden).

### 5.6 Measurement protocol (to replace the estimates)

Headless Chromium via Playwright + CDP `Performance.getMetrics` (`TaskDuration`, `ScriptDuration`, `LayoutDuration`, `RecalcStyleDuration`) and trace categories `devtools.timeline,blink`; scenarios: 1/3/6 pets walking, 6 pets idle (30 fps), 6 pets sleeping (rate 0), hidden tab 60 s, 4x CPU throttle, DPR 1/2/3. Budgets [D]: <= 1 ms main-thread per frame at 6 walking pets on a mid-range laptop, <= 2.5 ms at 4x throttle, idle (rate 0) < 0.1 % CPU, hidden tab: zero rAF callbacks and zero timers (assert via instrumentation counters), no layout reads outside the measure batch (assert `LayoutCount` flat while idle). Run the same scenario on the Canvas target for A/B.

---

## 6. Accessibility and UX

### 6.1 WCAG mapping [V: W3C Understanding documents]

- **2.2.2 Pause, Stop, Hide (Level A)**: moving/blinking/scrolling content that starts automatically, lasts > 5 s, and is presented in parallel with other content needs a mechanism to pause, stop or hide it (no exception for "decorative"; only "essential"). Pets qualify, so ship the control (6.3) in the persistent UI chrome, keyboard operable, reachable early in tab order, not hidden in a menu you must scroll to; state persists (local-only) [https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html].
- **2.3.3 Animation from Interactions (AAA)**: "Motion animation triggered by interaction can be disabled, unless the animation is essential"; technique C39: use `prefers-reduced-motion` to prevent motion. Gaze-following, hover reactions and petting are interaction-triggered, so they are the first things `still`/`off` disable [https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions.html].
- **2.4.11 Focus Not Obscured (Minimum, AA)**: pets must never hide a focused component (4.7).
- **2.3.1 Three Flashes**: no luminance flashing; blink = eyelid scale only; no strobing colour changes.
- Decorative semantics: container `aria-hidden="true"`, SVGs `focusable="false"` and no `role`/`title`, no tab stops, no live regions, no sound, `pointer-events: none`, `user-select: none`; never call `preventDefault`/`stopPropagation` on page events; never move focus; never change scroll or layout.
- Zoom/reflow: sized in CSS px (scales with browser zoom); small viewports (< 480 CSS px or `(pointer: coarse)`): max 1-2 pets, lanes only.

### 6.2 Cognitive load in a learning/exam context

- Decorative extras that are irrelevant to the learning goal ("seductive details") reduce retention and transfer (Harp and Mayer 1997/98; Rey 2012 meta-analysis: small-to-medium negative effect on retention, medium on transfer) [S: https://en.wikipedia.org/wiki/Seductive_details, https://www.researchgate.net/publication/257690772]. Motion onset captures attention (see 3.6). Therefore:
  - During an active run/timed task: mode forced to `quiet`: pets `sleep` (eyes closed, ultra-slow breath at <= 15 fps, or fully static), parked on lanes **outside** the task panel; no social events; no walking; no reactions to answers, scores, correctness or timers (no information leakage, no pressure).
  - Pets appear lively only on home, results, and idle screens; wake gently (fade-in, stretch) when the run ends.
  - Never overlay the question text, answer controls, timer, or proctor UI (keep-out covers the whole task panel).

### 6.3 User controls

- One control group in the preferences module: **Pets: Off / Still / Calm / Lively** (default `calm` unless `reduce`/Save-Data/coarse pointer, then `still`/`calm`), **Number of pets** (0-6, default 2-3 desktop, 1 mobile), and a visible quick toggle ("Pause pets") in the persistent chrome. Persisted local-only (`persisted local-only` class in the repo taxonomy); `off` destroys the engine (no observers, no loop, zero cost) and sets `data-pets="off"` for CSS.
- Multi-language labels (English first, German second, no default language), same pattern as the existing `🌐️i18n` module.

### 6.4 Pointer interaction etiquette

- Hover: a lingering pointer (>= 400 ms inside a pet's box) -> glance + tiny reaction; no flee unless the species trait says so and mode is `lively`.
- **Petting on click only if it never intercepts**: keep `pointer-events: none`; a global passive capture-phase `pointerdown` listener tests the pet hit-circle from the draw list *only when* `event.target` has no interactive ancestor, no active text selection (`getSelection().isCollapsed`), primary button, no drag within 6 px; the listener never calls `preventDefault`/`stopPropagation`. Reaction: a 1 s happy hop, cooldown 3 s; no counters, no streak mechanics, nothing that rewards frantic clicking.
- Touch: no hover; `pointerdown` sets a temporary gaze target for 2 s; taps always go to the UI.
- Dragging pets is out of scope unless the user enables an explicit "play mode" (which turns pet hit-testing on and is announced in the control), because it conflicts with "never block clicks".

---

## 7. Third-party oracles (test time only)

Rule from the repo: each feature needs a language-agnostic test (fixtures + Gherkin) whose expected output is reproduced by a third-party library. Existing pattern: `🧪️tests/<feature>/{🐍️.py, 🟦️.ts, 🥒️.feature, 🦀️.rs}` plus `🧫️fixtures/*.json`; the RNG already follows it (numpy vectors recorded by `mt19937-numpy-vectors.py`).

| Oracle | Validates exactly what | Tolerance / caveat |
|---|---|---|
| **numpy** (float64 scalar ops, not BLAS) | Third implementation of Tier-0 pieces: FK chain, affine inverse, two-bone IK knee point, spring coefficient application, layer blends. IEEE basic ops are the same, so FK/IK given identical inputs should match **bit-exactly** if written as explicit scalar `np.float64` expressions (no `@`, no `np.sum` pairwise, no FMA via `numexpr`). Also `numpy.random.MT19937()._legacy_seeding(seed)` + `random_raw` for the RNG stream and `RandomState(seed).random_sample()` for `genrand_res53` doubles. | Transcendentals only to ULP; numpy's int seeding is the legacy `init_genrand`-compatible one used by the repo vectors (the SeedSequence path of `MT19937(seed)` is different: [V: https://numpy.org/doc/stable/reference/random/bit_generators/mt19937.html]). Also cite the standard check value: default MT19937 (seed 5489) first output 3499211612 (already in repo vectors). |
| **scipy** `linalg.expm` | The exact discrete spring step: `expm([[0,1],[-omega^2,-2*zeta*omega]]*dt)` equals `[[posPos,posVel],[velPos,velVel]]` for all three damping regimes and the halflife form. `integrate.solve_ivp(rtol=1e-12)` for multi-step trajectories; `optimize.least_squares/fsolve` for IK (FK(theta) = target, both bend choices, unreachable targets -> closest point); `optimize.brentq` for `x(s) = f` in Bezier easing; `integrate` for ballistic arcs. | Tolerance about 1e-12 relative; never bit-level. |
| **sympy** | Symbolic derivation/verification of the spring coefficients (`dsolve`), the sqrt-based two-bone solution vs the angle form, Bezier polynomial coefficients, ballistic time-of-flight `T`; emits exact rational test vectors. | Derivations only (no float rounding claims). |
| **mpmath** | Correctly rounded references for `sin(2*pi*t)`, `cos`, `atan2/(2*pi)`, `exp` to 50 digits: measure max ULP error of `detmath` over 10^6 random + boundary points (multiples of 1/8 turn, tiny, near fold points); assert <= 2 ULP (goal <= 1). | Accuracy oracle, not bit-exact oracle. |
| **gl-matrix `mat2d`** (npm, TS tests) | `multiply` (FK composition and order: `out = a*b`), `invert`, `fromRotation/fromScaling/fromTranslation` composition vs our `T*R*S` local matrix and the 6-entry layout. Call `glMatrix.setMatrixArrayType(Array)` first: the default array type is `Float32Array` [M]. | `fromRotation` uses `Math.sin/cos`: compare to <= 1e-15 relative, not bits. [V layout/multiply: https://glmatrix.net/docs/module-mat2d.html] |
| **`bezier-easing`** (npm) + **Web Animations API in Chromium** | Cubic-Bezier easing parity: `BezierEasing(x1,y1,x2,y2)(f)` and, as a browser-grade oracle, a paused `KeyframeEffect` with `easing: 'cubic-bezier(...)'` sampled by setting `currentTime`; compare to our bisection result. | `bezier-easing` is table+Newton (tolerance about 1e-6 observed class) [V: https://github.com/gre/bezier-easing]; browser sampling precision about 1e-5. |
| **pyfabrik** (PyPI, 2D/3D) [S: https://github.com/saleone/pyfabrik]; **`@aminere/fullik` / `lo-th/fullik`** (npm, Caliko port) [S: https://www.npmjs.com/package/@aminere/fullik]; **`ikts`/IK.ts** [S: https://github.com/goldst/IK.ts]; **ikpy** (general chains via scipy) [S: https://github.com/maximkulkin/ikpy] | FABRIK: same iteration count/input must give the same joint positions; invariants: bone lengths preserved, root fixed, end effector at target or straight toward it. | Libraries differ in stop criteria/tolerance, so compare with `K` fixed to a high count and tolerance ~1e-6, or compare only invariants; none is a bit-level oracle. Two-bone: `scipy.optimize` / ikpy as the independent check. |
| **planck.js** (Box2D port) / **pymunk** (Chipmunk) | Ballistic jump sanity: analytic `T`, apex and landing point vs a physics step of a dynamic body under gravity; one-way platform landing (swept) vs a static edge shape: same landed surface id and time within O(g*DT). | Different integrators; assert tolerances, not equality. |
| **Playwright + axe-core** | Browser-level: no focusable/aria-visible pet nodes, `aria-hidden`, clicks reach underlying controls (`elementFromPoint` through the overlay), `securitypolicyviolation` listener reports **zero** violations under `style-src-attr 'none'`, `page.emulateMedia({reducedMotion:'reduce'})` yields static pets, hidden-tab test shows zero callbacks, no layout reads while idle. | Real engines (Chromium, Firefox, WebKit). |
| **Parity harness (ours, not third-party)** | TS vs Rust: run N = 10 000 ticks from a seed and a scripted event log; hash per-tick draw-list **bit patterns** (FNV-1a over `to_bits`/`Float64Array` words) and compare tick by tick; run on x86-64, aarch64, wasm32. Golden vectors for `detmath` as hex bit patterns (400+ covering all branches, as in the Vesper port). | This is what proves "bit-exact twin"; oracles above prove "correct". |

---

## 8. Recommended architecture

```
 SCHEMA (JSON Schema, source of truth)          rig.schema  clip.schema  behaviour.schema  species.schema  events.schema  drawlist.schema
        |  generate/mirror types, compile rigs to flat Float64Array tables (outside the bit-exact core)
        v
 PURE CORE (TS ⇄ Rust twin, Tier-0 math only)
   detmath       sinCosTurns, atan2Turns, expDet, wrap, clamp, smoothstep, bezierEase(bisection), spring coefficients
   skeleton      rig tables, pose (rest+deltas), FK (2x3 affine), inverse, inherit modes, update order with constraints
   motion        springs, look-at, blink, breath/squash, clips (sample, loop, events), layers + inertialization, two-bone IK, FABRIK, stepper, hop/ballistics, spring-tip chasers
   behaviour     needs/mood, utility+hysteresis, locomotion FSM, social protocol + pair scripts, activity governor (mode caps), RNG streams
   world         surfaces/keepOut reconciliation, one-way platform collision, fall/relocate, attention targets, scheduling (nextWake, frameRate)
   step(world, events) -> world        // events sorted (tick, kind, id); next-event time advance on tick{n}
   project(world) -> DrawList          // pure query side
        |
        v
 SHELL (impure, TS only)  events in:  pointer-moved {x,y,kind,overInteractive}  pointer-pressed {x,y,overInteractive,primary}  pet-poked {pet}
                                      surfaces-changed {epoch,surfaces[],keepOut[]}  viewport-resized {w,h,dpr}  tick {n}
                                      visibility-changed {hidden}  preference-changed {mode,count,reducedMotion,saveData,coarsePointer}
                                      focus-moved {rect|null}  modal-changed {open}  run-state-changed {active}  spawn/despawn {species,id,seed}
                          observers: Resize/Mutation/Intersection, scroll, visibilitychange, matchMedia(reduce, pointer:coarse, forced-colors)
                          loop: rAF accumulator (integer ticks), 60/30/15/0 rate classes, wake timer, hidden = cancel all
        |
        v
 RENDER TARGETS (interchangeable, consume DrawList only)
   svg      per-pet <svg>; wrapper CSSOM transform (translate, flipX, opacity); slot <g setAttribute('transform','matrix(...)')>; classes for colour
   canvas2d Path2D version (alternate)
```

**DrawList** (struct-of-arrays, per pet): `id, species, x, y, flipX (+-1), opacity, z (foot-y sort), bbox [x0 y0 x1 y1], hitCircle [cx cy r], state (enum, debug), bones: f64[6*B]` (world `a b c d e f`, pet-local, relative to anchor), `slots: [{attachment:int(-1 hidden), alpha}]`, `eyes: [{pupilX, pupilY, lidClosure}]`, `morphs: f64[]`, `ribbons`/`limbs` derive from bone matrices at the target. World also exposes `frameRate` and `nextWake`.

Module taxonomy (domain-driven, multi-language per repo conventions; emoji names to be assigned via the repo's emoji registry): framework modules `skeleton`, `motion`, `determinism` (detmath), `behaviour`, `ecology` (surfaces + social + world) each with `🟦️.ts`, `🦀️.rs`, `🧪️tests/`; product module `❓️quiz/🔨️modules/pets` (species rigs, personalities, affinity table, quiz-event mapping: run-state, modal, results); React target `🎯️targets/⚛️react/🔨️modules/pets` (layer, shell, SVG target). Schema first, then fixtures (`🧫️fixtures`), then Gherkin, then twins.

Test-first order: (1) `detmath` vs mpmath + golden bits, (2) FK vs gl-matrix/numpy, (3) springs vs `expm`, (4) easing vs bezier-easing/WAAPI, (5) IK vs scipy/pyfabrik, (6) RNG/behaviour determinism via trace hash TS=Rust, (7) surfaces/fall/jump vs planck/pymunk, (8) browser e2e (CSP, reduced motion, hidden tab, clicks).

---

## 9. Risks and open items

1. **ECMAScript wording for `Math.sqrt` exactness not directly verified** (spec fetch returned only the table of contents); every engine uses hardware sqrt in practice, and independent ports rely on it. Add a golden test of `sqrt` on awkward inputs in CI across Chromium/Firefox/WebKit and Node.
2. Rust twin on `wasm32` and `aarch64` must be exercised in CI early (FMA contraction and libm substitution are exactly the kind of bug that appears only there).
3. Surface derivation cost on large DOMs is the biggest runtime unknown; opt-in `data-pet-*` plus batching keeps it bounded; measure with 5.6.
4. All numeric tuning (speeds, caps, intervals, affinity deltas, blink distribution) is a starting point to be tuned by playtesting; blink statistics came from search summaries.
5. Rive's internal object model (bones, tendons, keyed properties) was taken from the container docs and secondary sources, not from its definition files (the `rive-runtime` `dev/defs` URLs were not retrievable).
6. Whether to offer pet dragging is a product question; recommended default is "no".

---

## 10. Sources

Skeletal formats: Spine JSON https://esotericsoftware.com/spine-json-format ; Spine skeletons http://esotericsoftware.com/spine-runtime-skeletons ; Spine physics (4.2) https://github.com/EsotericSoftware/spine-runtimes/blob/4.2/spine-csharp/src/PhysicsConstraintData.cs ; DragonBones https://github.com/DragonBones/DragonBonesJS/blob/master/docs/DragonBones_4.5_data_format_zh.md ; Rive format https://rive.app/docs/runtimes/advanced-topic/format ; Rive state machine/runtime https://dev.to/uianimation/engineering-interactive-mascots-with-rives-state-machine-and-runtime-architecture-4e2h , https://deepwiki.com/rive-app/rive-runtime ; VRM spring bone https://github.com/vrm-c/vrm-specification/blob/master/specification/VRMC_springBone-1.0/README.md

Math determinism: Rust f64 docs https://doc.rust-lang.org/std/primitive.f64.html ; rust-lang/libm https://github.com/rust-lang/libm ; Rust libm substitution issue https://github.com/rust-lang/rust/issues/142119 ; MDN Math https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math ; ECMA-262 Math https://tc39.es/ecma262/multipage/numbers-and-dates.html ; sunset-driver issue/PR https://github.com/jjgroenendijk/sunset-driver/issues/471 , https://github.com/jjgroenendijk/sunset-driver/pull/472 ; thevesperbell PR https://github.com/davetashner/thevesperbell/pull/54 ; libm precision survey https://zenn.dev/mod_poppo/articles/libm-precision?locale=en ; fdlibm https://www.netlib.org/fdlibm/readme , https://www.netlib.org/fdlibm/k_sin.c ; WebAssembly numerics https://webassembly.github.io/spec/core/exec/numerics.html , nondeterminism https://github.com/WebAssembly/design/blob/main/Nondeterminism.md ; numpy MT19937 https://numpy.org/doc/stable/reference/random/bit_generators/mt19937.html

Animation techniques: Juckett damped springs https://www.ryanjuckett.com/damped-springs/ ; Juckett two-bone IK https://www.ryanjuckett.com/analytic-two-bone-ik-in-2d/ ; Holden springs https://theorangeduck.com/page/spring-roll-call ; Holden dead blending https://theorangeduck.com/page/dead-blending ; FABRIK paper https://www.andreasaristidou.com/publications/papers/FABRIK.pdf (DOI 10.1016/j.gmod.2011.05.003) ; Little Polygon locomotion https://blog.littlepolygon.com/posts/loco2/ ; Procedural animation tutorial https://weaverdev.io/projects/proc-anim-tutorial/ ; Rain World animation https://www.gamedeveloper.com/art/video-animating-i-rain-world-i-and-its-many-squishy-stretchy-creatures ; Disney principles https://www.creativebloq.com/advice/understand-the-12-principles-of-animation ; blink physiology https://pmc.ncbi.nlm.nih.gov/articles/PMC4043155/ , https://journals.plos.org/plosone/article?id=10.1371%2Fjournal.pone.0196125

Behaviour/pets: oneko https://manpages.debian.org/unstable/oneko/oneko.6.en.html , https://deepwiki.com/crgimenes/neko ; Shimeji-ee affordances https://kilkakon.com/shimeji/affordances.php , behaviors https://git.dcsl.dk/General/shimeji-collection/src/tag/1.1/conf/behaviors.xml , https://github.com/TigerHix/shimeji-ee/blob/master/conf/behaviors.xml , https://github.com/gil/shimeji-ee/blob/master/readme.txt ; eSheep https://github.com/Adrianotiger/desktopPet , https://github.com/Adrianotiger/web-esheep ; VS Code Pets https://github.com/tonybaloney/vscode-pets , https://github.com/tonybaloney/vscode-pets/blob/main/src/panel/pets/dog.ts ; Desktop Goose https://samperson.itch.io/desktop-goose ; Utility AI https://en.wikipedia.org/wiki/Utility_system , https://www.gdcvault.com/play/1018040/Architecture-Tricks-Managing-Behaviors-in

Web platform: MDN style-src https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/style-src ; style-src-attr .../style-src-attr ; CSP3 https://www.w3.org/TR/CSP3/ ; CSS Transforms https://www.w3.org/TR/css-transforms-1/ ; CSS vars in SVG attributes https://css-tricks.com/svg-properties-and-css/ , https://lists.w3.org/Archives/Public/public-svg-issues/2025Nov/0024.html ; MDN rAF https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame ; visibilitychange https://developer.mozilla.org/en-US/docs/Web/API/Document/visibilitychange_event ; IntersectionObserver https://developer.mozilla.org/en-US/docs/Web/API/Intersection_Observer_API ; prefers-reduced-motion https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/At-rules/@media/prefers-reduced-motion ; saveData https://developer.mozilla.org/en-US/docs/Web/API/NetworkInformation/saveData ; Battery Status https://developer.mozilla.org/en-US/docs/Web/API/Battery_Status_API ; Chrome timer throttling https://developer.chrome.com/blog/timer-throttling-in-chrome-88 ; Fix Your Timestep https://gafferongames.com/post/fix_your_timestep/ ; SVG vs Canvas https://blog.logrocket.com/svg-vs-canvas/

Accessibility/UX: WCAG 2.2.2 https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html ; 2.3.3 https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions.html ; 2.4.11 https://www.w3.org/WAI/WCAG22/Understanding/focus-not-obscured-minimum.html ; seductive details https://en.wikipedia.org/wiki/Seductive_details , https://www.researchgate.net/publication/257690772_A_review_of_research_and_a_meta-analysis_of_the_seductive_detail_effect ; motion onset https://journals.sagepub.com/doi/abs/10.1111/1467-9280.01458 , https://link.springer.com/article/10.3758/s13423-011-0152-3

Oracles: gl-matrix mat2d https://glmatrix.net/docs/module-mat2d.html ; bezier-easing https://github.com/gre/bezier-easing ; pyfabrik https://github.com/saleone/pyfabrik ; ikpy https://github.com/maximkulkin/ikpy ; fullik https://www.npmjs.com/package/@aminere/fullik ; IK.ts https://github.com/goldst/IK.ts

Repo references read: `🎓️teaching/🏛️architecture/❓️quiz/🏗️builder/🌐️vite/🟦️.ts` (site CSP), `🧰️framework/🛍️products/❓️quiz/🔨️modules/🎲️randomness/🟦️.ts` (Mt19937 / FNV-1a / uniformIndex / shuffle), `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/mt19937-numpy-vectors.py` (numpy vector recorder), `🧰️framework/🛍️products/❓️quiz/🧪️tests/🌀️mt19937-generator/🟦️.ts`.
