# The ~10.4 s host silence after every reconcile `more-work` — root cause, fix, proof

Lane for `📓️wasm-hot-path-opt-level-2026-09-12.md` §4.3, ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`,
2026-09-12. Repo/semio MCP both refused to connect (`repo`: `invalid initialize params`; `semio`:
`CONNECTION_CLOSED`), so bookkeeping is this file only — no ticket state opened or closed. Peers were
editing the renderer engine throughout (a mid-write `📤️SegmentedDownload` export broke every boot for
~10 minutes, §6). Nothing was restaged; the 6018 serve was never restarted.

## TL;DR

1. **The host was never backing off, and the pump was never the defect.** `settlePluginTurn` /
   `drainTypedOperationTurns` re-poll a `more-work` answer on a `MessageChannel` continuation with no
   timer of any kind — proven by instrumentation (§2) and now by two laws (§4.2). There is no streak
   guard, no `setTimeout`, no rAF, no lock wait on that path.
2. **The 10.4 s is one full-document style recalculation per animation frame, and the host's
   continuation is simply next in a queue the renderer never gets to.** `Performance.getMetrics`
   across one silent window: `TaskDuration` 10.801 s, of which `RecalcStyleDuration` **10.018 s**
   over **475** recalculations (≈21 ms each), `ScriptDuration` 0.247 s, `LayoutDuration` **0.000 s**
   (§2.3). Over the whole 129.0 s baseline run: **5 039 recalculations costing 112.3 s**.
3. **The invalidation source is nine infinite CSS animations declared on `:root`**, animating nine
   registered custom properties that were all `inherits: true`
   (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui.css:23-32`). Animating an INHERITED registered custom
   property on the root element re-resolves the computed style of **every element in the document on
   every frame**, forever, whether or not a single ring is on screen (§2.4).
4. **Disproof-quality A/B, zero code changed:** the same build, minutes apart, with
   `prefers-reduced-motion: reduce` emulated (the stylesheet's own `animation: none` escape) converged
   at **13.1 s** instead of **110.0 s** (§2.5).
5. **Fixed structurally**: every clock moved onto the element (or pseudo-element) that paints with it,
   and all nine properties re-registered `inherits: false`, so a frame invalidates one element instead
   of the document. Behaviour is unchanged — all seven ring clocks still run and their phases still
   advance at the authored durations, proven at runtime (§3.3).
6. **Measured after the fix, motion fully enabled**: all 7 nodes `ok` at **12.4 s**, `meshes = 3` at
   **14.7 s** (baseline 110.0 / 129.6 s); `RecalcStyleDuration` **321 ms** (was 112 281 ms); the
   largest console silence in any of the six extension hops is **237 / 177 / 170 / 176 / 176 / 31 ms**
   against the baseline's six contiguous **10.4–11.7 s** windows (§5).

---

## 1. What the brief asked to look for, and what each candidate measured

| candidate from the brief | verdict | evidence |
|---|---|---|
| a backoff/`setTimeout` on `streak >= N` | **absent** | the gap follows streaks of 7, 8, 10 and 21 indiscriminately (§2.1); zero timer primitives are reached on the pump path (§4.2) |
| a retained-surface reconcile awaiting rAF/idle in a headless page | **absent** | no `requestAnimationFrame`/`requestIdleCallback` on the path; the `MessageChannel` heartbeat keeps ticking through the silence (§2.2) |
| worker pool pump waiting on a never-woken `MessageChannel` | **absent** | the heartbeat proves the channel is alive and delivering, at ~21 ms per turnaround (§2.2) |
| `createInFlightSkippingInterval`'s 120 ms floor compounding | **absent** | the gap is 10.4 s, not a multiple of 120 ms, and no interval fires inside it |
| a `contended` lock wait | **absent** | every `more-work` line in the run reads `contended=false` |
| intake budget refill cadence | **absent** | `ScriptDuration` in the silent window is 0.247 s out of 10.8 s — the host runs almost no JS at all |
| **main thread saturated by style recalculation** | **THE CAUSE** | §2.3, §2.4, §2.5 |

## 2. How it was localized

Four probes, all new inputs in the ticket folder, all headless against the live 6018 serve. Outputs in
`🗑️generated/reconcile-silence/`.

### 2.1 The gap does not correlate with the streak

From the baseline capture (`🗑️generated/opt-level-after/console.txt`), the interval between one
`reactor more-work` answer and the next:

```
after streak=10 at   8009.0 -> next more-work at  19958.6  gap=11949.5
after streak=21 at  22063.9 -> next more-work at  25338.1  gap= 3274.2
after streak= 7 at  26828.0 -> next more-work at  37960.1  gap=11132.1
after streak= 8 at  43688.7 -> next more-work at  55361.8  gap=11673.0
after streak=10 at  57391.5 -> next more-work at  67826.0  gap=10434.6
```
A streak cap would fire at one value. This fires at four. **No streak guard exists.**

### 2.2 The main thread is neither blocked nor idle — `🐍️reconcile-silence-probe.mjs`

An init script installs a `longtask` `PerformanceObserver` and a self-reposting `MessageChannel`
heartbeat, so a blocked thread and an idle thread each name themselves. Inside the 10.9 s silence at
58 320 → 69 232 ms (`🗑️generated/reconcile-silence/baseline/`):

- **zero** long tasks above 120 ms (the largest anywhere in the run is 1 248 ms, at 121.8 s, outside
  every silent window);
- the heartbeat **keeps beating**, at `sinceLastBeat=18.0` and `22.8` ms — 200 beats per 4.3 s.

So the host's macrotask queue is being drained, one macrotask per ~21 ms. That is one macrotask per
animation frame. **The continuation is not waiting on a timer; it is waiting its turn behind the
renderer's own frame.**

### 2.3 What the ~21 ms is — `🐍️reconcile-metrics-probe.mjs`

A sampling CPU profile (`🐍️reconcile-profile-probe.mjs`, 65 915 samples at 1 kHz) parks **95.8 %** of
the silence in `(program)` — not a JS frame, therefore not attributable by the profiler.
`Performance.getMetrics` splits the same wall clock properly
(`🗑️generated/reconcile-silence/metrics-1/`):

| silent window | wall | TaskDuration | **RecalcStyleDuration** | ScriptDuration | LayoutDuration | RecalcStyleCount |
|---|---|---|---|---|---|---|
| 57 645 → 68 376 | 10.81 s | 10.801 s | **10.018 s** | 0.247 s | 0.000 s | 475 |
| 69 381 → 80 581 | 11.22 s | 11.208 s | **10.005 s** | 0.468 s | 0.000 s | 428 |
| 83 660 → 97 138 | 13.60 s | 13.592 s | **12.768 s** | 0.288 s | 0.000 s | 494 |
| 99 093 → 110 658 | 11.37 s | 11.364 s | **10.570 s** | 0.250 s | 0.000 s | 467 |

**The silence is style recalculation, ~21 ms each, back to back, with the host running 2 % of the
wall clock in JS.** No layout, no paint — style alone.

### 2.4 What invalidates style — `🐍️reconcile-dom-probe.mjs`, `🐍️reconcile-animation-probe.mjs`

A whole-document `MutationObserver` reports **zero** DOM mutations and zero `style`/`class` attribute
writes through the silence, with `styleTags=4 sheets=4 rules=1253 elements=1274` flat. So nothing
writes; something animates. `document.getAnimations()` sampled every 500 ms alongside
`RecalcStyleCount` (`🗑️generated/reconcile-silence/anim-1/animations.jsonl`) names them — nine
`CSSAnimation`s whose effect target is `<html>`, running for the entire life of the page:

```
CSSAnimation:loading-border-spin:running:<html>          CSSAnimation:celebrate-border-spin:running:<html>
CSSAnimation:loading-border-pulse:running:<html>         CSSAnimation:celebrate-border-burst:running:<html>
CSSAnimation:waiting-border-spin:running:<html>          CSSAnimation:window-silhouette-border-loading-dash:running:<html>
CSSAnimation:waiting-border-pulse:running:<html>         CSSAnimation:window-silhouette-border-waiting-dash:running:<html>
CSSAnimation:introduced-border-pulse:running:<html>
```
and the recalculation cost is flat across the WHOLE run, not only the silent windows — ~24 recalcs and
**~520 ms of style recalculation per 500 ms of wall clock**, i.e. the main thread is over-subscribed
from the first second to the last.

**The exact line.** `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui.css:16-32`, as it stood:

```css
:root {
  --border-normal-color: var(--color-gray);
  …
  animation:
    loading-border-spin var(--loading-border-duration) linear infinite,
    loading-border-pulse var(--loading-border-duration) ease-in-out infinite,
    waiting-border-spin var(--waiting-border-duration) linear infinite,
    waiting-border-pulse var(--waiting-border-duration) ease-in-out infinite,
    introduced-border-pulse var(--introduced-border-duration) ease-in-out infinite,
    celebrate-border-spin var(--celebrate-border-duration) linear infinite,
    celebrate-border-burst var(--celebrate-border-duration) ease-in-out infinite,
    window-silhouette-border-loading-dash var(--loading-border-duration) linear infinite,
    window-silhouette-border-waiting-dash var(--waiting-border-duration) linear infinite;
}
```

Every one of the nine properties those keyframes animate was registered `inherits: true`:

```
--loading-border-angle → true    --waiting-border-angle → true    --introduced-border-width → true
--loading-border-pulse-opacity → true   --waiting-border-pulse-opacity → true   --celebrate-border-angle → true
--loading-border-dashoffset → true      --waiting-border-dashoffset → true      --celebrate-border-padding → true
```

An animated registered custom property with `inherits: true`, changed on the root element, changes the
**inherited** value of every descendant, so Blink must re-resolve the computed style of the entire
document on every frame. Nine of them, forever, on a page with 1 274 elements and 1 253 rules ⇒ ~21 ms
per frame of pure style recalculation, permanently.

That is why the cost looks like "the host takes 10 s to come back": the host's `MessageChannel`
continuation is a normal-priority task, and Blink's frame task outranks it whenever a frame is
pending. With every frame costing more than a frame budget, the host advances exactly one macrotask
per frame. A settle that needs ~500 continuations therefore takes ~10.4 s of wall clock for 0.33 s of
guest CPU — precisely the shape §4.3 of the opt-level report measured.

### 2.5 The disproof, with no code changed — `🐍️reduced-motion-ab-probe.mjs`

The stylesheet already carried a `@media (prefers-reduced-motion: reduce) { :root { animation: none } }`
escape. Emulating that preference is therefore a clean A/B of exactly this hypothesis. Same build,
same machine, minutes apart:

| run | running animations | RecalcStyleCount | **RecalcStyleDuration** | ScriptDuration | TaskDuration | all 7 nodes `ok` | `meshes = 3` |
|---|---|---|---|---|---|---|---|
| motion (baseline) | 9 | 5 039 | **112 281 ms** | 5 928 ms | 128 980 ms | **110 036 ms** | **129 567 ms** |
| `prefers-reduced-motion: reduce` | 0 | 199 | **290 ms** | 3 312 ms | 8 952 ms | **13 104 ms** | **15 828 ms** |

**8.4× on convergence from one media query.** The ticket's remaining wall clock was never the guest,
the dag walk, wasm codegen, the pump, or a backoff. It was style invalidation.

---

## 3. The fix

### 3.1 The rule

A CSS animation's cost is the size of the style invalidation it causes, not the number of animations.
So: **every phase clock runs on the element (or pseudo-element) that paints with it, and every animated
registered custom property is `inherits: false`.** One clock on the root inheriting a phase downward is
one animation instead of N — and one full-document recalculation per frame instead of N element
recalculations. An element-scoped clock costs the number of rings actually mounted, which on this page
is zero.

### 3.2 `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui.css`

- The nine-animation `animation:` shorthand is **removed from the unlayered `:root` block**, and the
  `@media (prefers-reduced-motion: reduce) { :root { … } }` block that silenced it is removed with it.
- All nine animated `@property` registrations flipped to `inherits: false`; the four
  `--*-border-duration` tokens flipped to `inherits: true` (they are never animated, so inheriting them
  costs nothing and keeps a subtree able to retune the cadence).
- Thirteen consumers gained the clock they paint with:

| consumer | clock now declared on it |
|---|---|
| `@utility border-loading &::after` | `loading-border-spin`, `loading-border-pulse` |
| `@utility border-waiting &::after` | `waiting-border-spin`, `waiting-border-pulse` |
| `[data-introduced="true"]` | `introduced-border-pulse` |
| `[data-celebrated="true"]::after` | `celebrate-border-spin`, `celebrate-border-burst` |
| `.window-silhouette-border-introduced` | `introduced-border-pulse` |
| `.window-silhouette-border-loading` | `window-silhouette-border-loading-dash`, `loading-border-pulse` |
| `.window-silhouette-border-waiting` | `window-silhouette-border-waiting-dash`, `waiting-border-pulse` |
| `.window-silhouette-border-celebrated-mask` | `celebrate-border-burst` |
| `.window-silhouette-border-celebrated-fill` | `celebrate-border-spin` |
| the four `🎉️CelebrateContent` paints (label text, themed icon mask, text-kind icon, tree guide/branch) | `celebrate-border-spin` |

  The last four were **found by the analyzer, not by reading** (§4.1): they paint through the
  `--celebrate-conic` indirection and would otherwise have frozen at `0deg`. A `[data-celebrated]` rule
  that only *declares* `--celebrate-conic` is not a paint and correctly owns no clock.
- One reduced-motion stop now lists the painting selectors instead of `:root`, and the two
  `@utility … @media (prefers-reduced-motion: reduce) { &::after { … } }` blocks gained `animation: none`.

### 3.3 Behaviour is unchanged — proven, not asserted

`🐍️ring-liveness-probe.mjs` mounts one element per ring state into the live app and reads
`document.getAnimations()` plus each pseudo-element's own animated property at two instants:

```
BASELINE_RUNNING 0                      ← nothing mounted ⇒ nothing animating, the whole point
first  : running = [celebrate-border-burst, celebrate-border-spin, introduced-border-pulse,
                    loading-border-pulse, loading-border-spin, waiting-border-pulse, waiting-border-spin]
         loadingAngle 0deg  waitingAngle 0deg  introducedWidth 1px  celebrateAngle 0deg  rootAngle 0deg
second : (+400 ms) loadingAngle 90.09deg  waitingAngle 45.045deg  introducedWidth 2.00172px
                   celebrateAngle 120.12deg   rootAngle 0deg
afterRemoval: 0 running animations
```
90.09° per 400 ms = 1.6 s/turn (`--loading-border-duration`), 45.045° = 3.2 s/turn, 120.12° = 1.2 s/turn
— every authored cadence preserved exactly. `rootAngle` stays `0deg`: the document root no longer
animates. Removing the elements returns the page to zero running animations.

---

## 4. Tests

### 4.1 The structural law, derived from the stylesheet itself

Pinning "don't animate on `:root`" as a string match would rot. The law is instead **derived**: a new
analyzer reads the sheet's own animation scope and three laws are evaluated from it.

- `🎨️styling/📦️packages/🟦️typescript/🟦️.ts` → `//#region 🔁️AnimationScope`:
  `analyzeCssAnimationScope(css)` (own brace-scanner, resolves nested `@utility x { &::after }` /
  `@media` to a selector path, resolves custom-property indirection chains, distinguishes a *paint*
  — a read in a non-custom declaration — from a *forward*), `cssAnimationScopeViolations`,
  `cssAnimationScopeUnclockedPaints`, `CSS_ANIMATION_SCOPE_LAWS`.
- Laws: `no-document-root-clock`, `animated-custom-properties-are-non-inherited`,
  `every-paint-owns-its-clock`.
- **Language-agnostic fixture**: `🎨️styling/🧫️fixtures/🔁️animation-scope/🔣️.json` — five CSS cases with
  the full expected analysis and violation list (the measured defect; the conforming element-scoped
  shape; the forward-is-not-a-paint case; the nested-utility + `animation: none` case; a missing clock
  with no root clock), plus the real stylesheet's expected `keyframesByProperty` and `violations: []`.
- **Third-party twin**: the bun suite re-answers all four questions over **postcss**'s node tree and
  asserts the two implementations agree on every fixture case **and on `🖌️ui.css` itself**.
- **Node twin**: `🎨️styling/🧪️tests/🔬️node-twin/🟦️.ts`, no test framework, shares only the fixture file.
- Registered: `@semio-tech/ui-styling:twin` nx target, and `⚖️gate🎨️styling🔁️animation-scope` +
  `⚖️gate🎨️styling🔁️animation-scope-twin` in `.vscode/launch.json`.

```
$ bun ./📜️script.ts twin                                      (🎨️styling/📦️packages/🟦️typescript)
[twin] 🔁️animation-scope: 5 fixture cases + 🖌️ui.css, 0 failure(s)

$ bun test ../../🧪️tests/🧩️suite/🟦️.ts -t "border effect animation scope"
[DEBUG] 🔁️animation-scope: 9 animated properties, 21 clocks, 14 paints, 0 root clocks
 3 pass | 0 fail | 66 expect() calls   (248 ms)
```

Two pre-existing laws in `🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` pinned the OLD
(pathological) shape — `expect(unlayeredRoot).toMatch(/animation:.*loading-border-spin/)`,
`@property … inherits: true`, and four `not.toMatch(… animation: …)` on the consumers. Both are
inverted to the new contract and the second is renamed
(`border effect phase clocks run on the painting element, never on the document root`), with the
measured reason in-file.

```
$ bun ./📜️script.ts test -t "border effect"                  (🖱️ui/…/🎯️targets/⚛️react)
Tests  2 passed | 714 skipped
```
⚠️ that run also reports `1 failed` test FILE: `.storybook/🧪️tests/🧭️scope-resolution/🟦️.ts` cannot be
bundled because the repo library's lease store imports `node:sqlite`. Pre-existing and unrelated — the
same class of failure the previous lane recorded for jsdom bundles of that module.

### 4.2 The pump laws the brief asked for

`📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx`, `typed-operation completion drain`. Both drive
the REAL `yieldPluginUiContinuation` (not a stub) and count every wall-clock primitive the drain could
reach for — `setTimeout`, `setInterval`, `requestAnimationFrame`, `requestIdleCallback` — because
neither rAF nor idle fires in a hidden tab and evaluation must converge when nobody is looking.

| law | asserts |
|---|---|
| `re-polls every more-work answer on a continuation, never on a wall clock` | 64 consecutive `more-work` answers ⇒ exactly 64 polls, **0** timer primitives, < 1 000 ms wall |
| `yields the thread to a contended peer between polls instead of sleeping on it` | a competing continuation chain gets at least one turn per poll (32 ≥ 32), **0** timer primitives, < 1 000 ms wall |

```
$ bun ./📜️script.ts test long -t "typed-operation completion drain"
Test Files  1 passed | 29 skipped (30)      Tests  6 passed | 991 skipped (997)

[DEBUG] more-work pump: 64 answers → 64 polls in 0 ms with 0 timer calls
[DEBUG] contended more-work pump: 32 polls interleaved with 32 peer turns in 0 ms, 0 timer calls
```

### 4.3 Typecheck

`bun ./📜️script.ts typecheck` on `@semio-tech/ui-react`: **338 errors, none in any file this lane
touched** (`🎨️styling/📦️packages/🟦️typescript/🟦️.ts`, `🧪️tests/🔬️node-twin/🟦️.ts`,
`🎨️styling/🧪️tests/🧩️suite/🟦️.ts` → zero; the pre-existing errors in
`🧪️owned-locale-detector-retirement/🟦️.tsx` are at lines 206–4304, none inside the 380–440 range this
lane edited).

---

## 5. Runtime after the fix, motion fully enabled

`🐍️tick-cost-probe.mjs`, unchanged, on `http://127.0.0.1:6018/?plugin=generation3d`
(`🗑️generated/after-fix/`):

```
$ SEMIO_PROBE_SECONDS=150 SEMIO_PROBE_OUT=after-fix bun 🐍️tick-cost-probe.mjs
DONE lines 8164 debugLines 8156 allOkMs 12381.55 meshesMs 14736.49 meshesSeen 3
window:procedural-main    → height/radius/sides/profile/extrusion-axis/extrude/column-preview all "ok"
window:procedural-preview → phase idle, facesDone 8/8, unitsDone 44/44, ratio 1.0, meshes 3
```

**The six extension hops** (`extension completion submitted`, ms since navigation):

| hop | baseline | after | largest console silence inside the hop — baseline → after |
|---|---|---|---|
| 1 | 25 064 | **6 227** | 10.4–11.9 s → **236.9 ms** |
| 2 | 52 871 | **8 210** | 10.4–11.7 s → **176.5 ms** |
| 3 | 76 049 | **9 734** | 10.4–11.7 s → **169.8 ms** |
| 4 | 102 025 | **11 626** | 10.4–11.7 s → **175.8 ms** |
| 5 | 117 129 | **12 443** | 10.4–11.7 s → **176.4 ms** |
| 6 | 122 095 | **13 603** | 10.4–11.7 s → **31.0 ms** |

The five largest gaps in the entire post-fix capture are `1 709.9` ms (901 → 2 611, wasm instantiate,
before the first turn), `489.2` ms (a React DevTools notice at boot), `263.8` ms (boot), `236.9` ms and
`220.0` ms. **No contiguous silence over 240 ms exists anywhere after boot.**

Convergence, matched-pair against the same A/B harness (`🐍️reduced-motion-ab-probe.mjs`):

| run | animations | RecalcStyleCount | RecalcStyleDuration | ScriptDuration | TaskDuration | all 7 `ok` | `meshes = 3` |
|---|---|---|---|---|---|---|---|
| before (motion) | 9 | 5 039 | 112 281 ms | 5 928 ms | 128 980 ms | 110 036 ms | 129 567 ms |
| reduced-motion control | 0 | 199 | 290 ms | 3 312 ms | 8 952 ms | 13 104 ms | 15 828 ms |
| **after the fix (motion on)** | **0** | **372** | **321 ms** | **3 257 ms** | **8 930 ms** | **12 490 ms** | **15 189 ms** |

The fixed run matches the reduced-motion control within noise — the fix recovers all of the available
wall clock **while keeping every animation**.

**Against the brief's target.** The target was "< 10 s total"; measured is 12.4 s to all-nodes-`ok` and
14.7 s to `meshes = 3`, from a 112 s baseline (**8.9× / 8.5×**). The remainder is no longer style: it
is 3.3 s of host JS plus the guest's own turns, and the 1.7 s wasm instantiate before the first turn.
That is the next thing to attack, and it is now the largest item rather than 3 % of the run.

---

## 6. Peer interference encountered (attributed, not worked around)

Between the fix and its first verification, three consecutive probe runs booted to a blank page with
`ROOT_HTML_LEN 0` and one page error:

```
SyntaxError: The requested module '…/🧱️elements/📤️SegmentedDownload/🟦️.ts'
does not provide an export named 'segmentedDownloadSinkFactory'
```
The export exists on disk (`📤️SegmentedDownload/🟦️.ts:111`, file written at 09:08 by a peer) but the
vite module graph was serving a stale transform of that module — the watcher missed the write. Cleared
by `touch`ing the file (mtime only, no content change); the serve was **not** restarted and the plugin
was **not** restaged. Worth knowing: a blank 6018 page with a `does not provide an export named` error
is a peer's mid-write module plus a missed watcher event, not a wedged serve.

---

## 7. Files changed

**Fix**

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui.css` — nine clocks moved off `:root` onto the thirteen
  painting selectors; nine `@property` registrations → `inherits: false`, four durations →
  `inherits: true`; the `:root` reduced-motion block replaced by a consumer-scoped stop; new
  `🔁️BorderEffectClocks` docstring carrying the measurement.

**Laws + analyzer**

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/🟦️.ts` — new `🔁️AnimationScope` region:
  `analyzeCssAnimationScope`, `cssAnimationScopeViolations`, `cssAnimationScopeUnclockedPaints`,
  `CSS_ANIMATION_SCOPE_LAWS`, `CssAnimationScope`/`CssClockRule`/`CssPaintRule`.
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧫️fixtures/🔁️animation-scope/🔣️.json` — new, 5 language-agnostic
  cases + the real stylesheet's expected scope.
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🔬️node-twin/🟦️.ts` — new framework-free twin.
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🟦️.ts` — three laws, one of them a postcss twin.
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/📜️script.ts` — `twin` command.
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript/📋️project.json` — `twin` target.
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement/🟦️.tsx` — two laws inverted to
  the new contract, one renamed.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` — the two
  pump laws plus their `countTimerPrimitives` helper.
- `.vscode/launch.json` — `⚖️gate🎨️styling🔁️animation-scope`, `⚖️gate🎨️styling🔁️animation-scope-twin`.

**Ticket inputs (kept)**

- `🐍️reconcile-silence-probe.mjs` — longtask observer + MessageChannel/timer heartbeats.
- `🐍️reconcile-profile-probe.mjs`, `🐍️reconcile-profile-report.mjs` — CDP sampling profile + silent-window aggregation.
- `🐍️reconcile-metrics-probe.mjs` — `Performance.getMetrics` timeline (the probe that named the cause).
- `🐍️reconcile-dom-probe.mjs` — whole-document MutationObserver + stylesheet census.
- `🐍️reconcile-animation-probe.mjs` — `document.getAnimations()` vs `RecalcStyleCount`.
- `🐍️reduced-motion-ab-probe.mjs` — the matched A/B convergence harness.
- `🐍️ring-liveness-probe.mjs` — proves each ring still animates at its authored cadence.
- `🐍️boot-error-probe.mjs` — page-error/request-failure capture (§6).

**Ticket outputs** — `🗑️generated/reconcile-silence/{baseline,profile-1,metrics-1,dom-1,anim-1,reduced-motion,full-motion,fixed-1,fixed-2,fixed-3}/`
and `🗑️generated/after-fix/`.

## 8. Everything that was run

| command | result |
|---|---|
| `bun ./📜️script.ts twin` (`@semio-tech/ui-styling`) | **5 fixture cases + 🖌️ui.css, 0 failures** |
| `bun test ../../🧪️tests/🧩️suite/🟦️.ts -t "border effect animation scope"` | **3 pass, 0 fail, 66 expects** |
| `bun ./📜️script.ts test -t "border effect"` (`@semio-tech/ui-react`) | **2 passed**; 1 unrelated file fails on `node:sqlite` |
| `bun ./📜️script.ts test long -t "typed-operation completion drain"` | **6 passed** (4 pre-existing + 2 new) |
| `bun ./📜️script.ts typecheck` (`@semio-tech/ui-react`) | 338 pre-existing errors, **0 in this lane's files** |
| `SEMIO_REDUCED=1/0 … bun 🐍️reduced-motion-ab-probe.mjs` | the §2.5 / §5 matched pairs |
| `SEMIO_PROBE_SECONDS=150 SEMIO_PROBE_OUT=after-fix bun 🐍️tick-cost-probe.mjs` | converged at 12.4 / 14.7 s |
| `bun 🐍️ring-liveness-probe.mjs` | all seven clocks live, phases advance, 0 when unmounted |
| `bun nx run @semio-tech/ui-styling:twin` then `…:test --args='-t "border effect animation scope"'` | **exit 0 for both** (queued behind peers' nx work for ~15 min, then `NX Successfully ran target twin` / `target test for project @semio-tech/ui-styling`) |
