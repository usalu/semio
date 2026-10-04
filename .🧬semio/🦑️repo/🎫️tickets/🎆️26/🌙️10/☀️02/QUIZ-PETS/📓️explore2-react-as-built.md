# 📓️ Explore 2 — the React target and the quiz glue, as built

Read-only survey for round two of the pets (clickable, draggable pets; ropes, ladders, parachutes; rain, beams, hearts; host elements that pets displace). Everything below was read from the files on disk on 2026-10-02 (nothing run, no browser): where a statement is about browser behaviour that nobody measured, it says so. Line numbers are of the files as they are now; other agents edit the quiz and site tree concurrently.

Abbreviations (as in `📓️design.md`): `P` = `🧰️framework/🛍️products/🐾️pets`, `PR` = `P/🎯️targets/⚛️react`, `Q` = `🧰️framework/🛍️products/❓️quiz`, `QR` = `Q/🎯️targets/⚛️react`, `S` = `🎓️teaching/🏛️architecture/❓️quiz`, `AP` = `🎓️teaching/🏛️architecture/🐾️pets`, `TK` = this ticket folder. Modules of `PR`: `LAYER` = `PR/🔨️modules/🫧️layer/🟦️.tsx`, `SURVEY` = `PR/🔨️modules/📡️survey/🟦️.ts`, `PACING` = `PR/🔨️modules/⏲️pacing/🟦️.ts`, `DEPICT` = `PR/🔨️modules/🖌️depiction/🟦️.ts`, `CSS` = `PR/🎨️.css`, `STORIES` = `PR/📖️stories/🟦️.tsx`. Quiz glue `GLUE` = `QR/🔨️modules/🐾️pets/🟦️.tsx`. Suites: `T-DL` = `P/🧪️tests/🫥️decorative-layer/🟦️.tsx`, `T-DP` = `P/🧪️tests/🖌️pet-depiction/🟦️.tsx`, `T-SV` = `P/🧪️tests/📡️surface-survey/🟦️.tsx`, `T-FP` = `P/🧪️tests/⏲️frame-pacing/🟦️.tsx`, `T-PC` = `Q/🧪️tests/🐾️pet-companions/🟦️.tsx`, `E-PW` = `S/🧪️tests/🐕️pet-walk/🟦️.ts`, `E-LN` = `S/🎭️e2e/🚶️learner/🟦️.ts`.

## 0. The ten facts that matter most for round two

1. **One actor per species, one depiction per species.** `LAYER:130,199-203` keys `depictions` by `actor.species`; the cast never holds a species twice (site test `S/🧪️tests/🐾️pet-cast/🟦️.ts:142-147`). Anything new that is not "a species on stage" (rope, ladder, drops, hearts, beams) has no home in `Frame`, `Depiction` or `show()` today.
2. **Pets are not hit targets by inheritance, not by their own rule.** `CSS:9` puts `pointer-events: none` on `.pet-layer`; `.pet` (`CSS:24-35`) declares nothing about it, so every descendant inherits `none`; the layer repeats it inline at runtime (`LAYER:348`). The CSS file may contain no `pointer-events` value but `none` (regex `T-DL:317`).
3. **Hit-testing is a rectangle in script, not the DOM.** `hit()` (`LAYER:272-282`) tests the species `size` box of the last frame; it decides only whether a primary `pointerdown` also becomes `poked` (`SURVEY:305-312`). `poked` → core `poke` (`P/🔨️modules/🎪️stage/🟦️.ts:1547-1570`) → `hail`.
4. **The quiz's own drag and the presence feed both ask the DOM who is under the pointer.** `QR/🔨️modules/🤏️drag/🟦️.ts:13-16` (`document.elementFromPoint` → `closest("[data-quiz-drop]")`) and `QR/🔨️modules/👥️presence/🟦️.tsx:791-793,807-810` (`event.target.closest("[data-presence-anchor]")`). A pet that is a real hit target hides drop zones and anchors under it from both (drops on it are not recognised, the shared cursor loses its anchor while over/capturing a pet). Not mentioned in any report so far.
5. **The survey cadence is coupled to `Frame.rate`, not to "actors travel".** `LAYER:234`: a survey every 8 ticks whenever the stage reports rate 64, once a second at any rate above 0. A held (dragged) pet or particles that force rate 64 therefore also cost 8 surveys + 8 `glances()` calls a second (0.15 – 3.4 ms each, `📓️report-wp-g.md`).
6. **The reduced-motion transition fix covers the inside of the layer only.** `CSS:17-20` and `SURVEY:243-246`. A host element that pets displace sits outside it: under reduced motion the quiz gives it `transition-duration: 0.01ms !important` (`QR/🎨️.css:597-603`), every write starts a transition, its `transitionend` is *not* ignored (`settled`, `SURVEY:243-246`, only passes over targets inside the layer) → `stale(false)` → survey + `pacer.wake()`: the storm R measured (485 `transitionrun` in 2 s, 172 live animations, possibly one renderer crash).
7. **Exact pins on the layer's moving parts.** Listener sets per target (`T-DL:785-789,829`), observer counts (`T-DL:790,830`), timer counts (`T-DL:527,570,791,824`), animation-frame counts (`T-DL:394,409,425,823`), the exact props the quiz hands the layer (`T-PC:441`), the exact `Surveyed` object (`T-SV:159,213-218`), "layer is the last child of `.quiz-app`" (`T-PC:797`). New listeners/observers/timers/props/event fields change those assertions.
8. **No `id` and no `<style>` in the pet DOM** (`T-DL:268-272`, `T-DP:118-121,266`), which rules out `<defs>`/`clipPath`/`marker`/`<use href="#…">` for parachutes and ropes unless that rule is revised. CSP allows same-document `url(#id)`; the ban is the repo's own.
9. **`Surveyed` is a closed schema in three twins.** `walls`/`fixtures` are changes in `P/🧬️schema/🟦️.ts:210`, `P/🧬️schema/🔣️.json`, `P/🧬️schema/🦀️.rs`, the Rust stage, the survey vectors generator `TK/generate_survey_vectors.py`, `P/🧫️fixtures/📡️surface-survey/🔣️.json` (15 scenes), `sameSurvey` (`LAYER:77-88`) and `staged` (`LAYER:91-100`).
10. **Entry budget headroom is ~9.5 kB gzip.** 250 475 B of 260 000 B at the last measurement (R, 15:02); the pets chunks are lazy and uncharged, but whatever the quiz glue gains is charged to the entry. The glue may not import values from `@semio-tech/pets-react` (it is type-only on purpose, `GLUE:25,120`).

---

## 1. Layer anatomy (`LAYER`)

### 1.1 `PetLayerProps` today (`LAYER:389-403`)

```tsx
export interface PetLayerProps {
  readonly menagerie: Menagerie;
  readonly scene: string;
  readonly mode: PetMode;                              // "still" | "calm" | "lively"; off = not rendering the layer
  readonly quiet?: boolean;
  readonly capacity?: number;                          // default petCapacity(width): <768 → 2, <1024 → 4, else 6   (LAYER:54-56)
  readonly surfaces?: string;                          // default PET_SURFACES = "[data-pet-surface]"             (SURVEY:18)
  readonly keepouts?: string;                          // default PET_KEEPOUTS = controls, texts, [data-pet-keepout] (SURVEY:28)
  readonly glances?: () => readonly Point[];           // sampled with every survey, viewport px
  readonly scale?: number;                             // default petScale(width): <768 → 0.8, else 1             (LAYER:59-61)
  readonly seed?: number;                              // default random per effect run: Math.floor(Math.random() * 2**32) (LAYER:430)
  readonly zIndex?: number;                            // default PET_LAYER_Z_INDEX = 35
  readonly onCast?: (species: readonly Slug[]) => void;
  readonly tempo?: number;                             // default 1; held to [1/8, 8]; a new tempo starts a new show
}
export function PetLayer(props: PetLayerProps): ReactElement;   // LAYER:422
```

There is no imperative handle (`forwardRef`/`ref`), no event/command prop and no render-prop: the only inputs are props and DOM events. Exports of the barrel `PR/🟦️.tsx:12-24`: `depict`, `depictionMarkup`, `paint`, the survey names, the pacing names, `PetLayer`, `petCapacity`, `petCast`, `petScale`, `PET_EDGE_INSET (6)`, `PET_HOME_SCENE`, `PET_LAYER_Z_INDEX (35)`, `PET_MEDIUM_WIDTH`, `PET_NARROW_WIDTH`, `PET_ROTATION_SECONDS = [60, 120]`, plus core re-exports (`MENAGERIE_SCHEMA`, `PET_MODES`, `TICKS_PER_SECOND`, types). The barrel imports `./🎨️.css` (`PR/🟦️.tsx:12`), so the stylesheet is part of the lazy chunk.

Render: `<div ref={host} className="pet-layer" aria-hidden="true" />` (`LAYER:440`), nothing else; React never re-renders per frame (`T-DL:529` `renders === 1`).

### 1.2 Engine lifecycle

- **Effect 1** (`LAYER:428-436`, deps `[menagerie, sound, seed, tempo, forced]`): returns early when forced colours are active (`LAYER:426,429`), when `menagerieIssues(menagerie)` is non-empty (`LAYER:427`, once per menagerie) or no host; else `startShow(host, menagerie, seed ?? random, tempo, {scene, mode, quiet, capacity, surfaces, keepouts, glances, scale, zIndex, onCast})`; cleanup `started.stop()`.
- **Effect 2** (`LAYER:437-439`, deps = all other props): `show.current?.direct({…})` — also runs once on mount with identical values (no-op: same references).
- **StrictMode**: `mountQuiz` wraps the app in `<StrictMode>` (`QR/🟦️.tsx:552`), so in dev effect 1 runs, cleans up and runs again: two `startShow`s, two animation-frame requests (`T-DL:823`), a different random seed each when `seed` is undefined (`LAYER:430`), listener counts `[7,5,1]` after the second (`T-DL:829`). The cleanup is complete (`stop`, `LAYER:335-345`: `ended`, `pacer.stop()`, the three unwatchers, all depiction elements removed, inline `z-index` and `pointer-events` removed, `announce([])`).
- `startShow(host, menagerie, seed, tempo, first): Show` (`LAYER:124-374`) owns a closure of mutable state: `stage` (opened with `tuned` + `hushed`, `LAYER:132-135`), `frame`, `waiting: StageEvent[]`, `stale: "now" | "soon" | null` (starts `"now"`), `surveyed`, `surveyedSize`, `surveyedAt` (tick), `touched`, `glanced`, `origin`, `width`, `epoch`, `rotateAt`, `wished`, `summoned`, `redraw`, `told`, `toldTo`, `ended` (`LAYER:136-153`). It sets `host.style.zIndex` and `host.style.pointerEvents = "none"` (`LAYER:347-348`), then `rest()` and `pacer.wake()` (`LAYER:349-350`).
- **The only writer is `step(ticks)`** (`LAYER:228-252`), run inside the pacer's animation-frame callback and wrapped in `try/catch` that calls `stop()` and returns `{tick: 0, rate: 0, wake: null}` — a fault ends the show silently (`LAYER:254-262`, `T-DL:860-878`). Order inside a step: `ticked` (if ticks > 0) → `measure(tick)` when due → waiting events → rotation bookkeeping → `summon()` → `advance(menagerie, stage, events)` + `frameOf` when there is any event or no frame yet → `show(frame)` when there is any event or `redraw` → return the frame as `Pace` (`{tick, rate, wake}`). Final event order is `[ticked, surveyed, glanced, …waiting, summoned]`.
- **Survey due** (`LAYER:234`): `stale === "now"` (resize/scroll/ResizeObserver, `resurvey`, scale change), or `stale === "soon"` and (rate 0 or ≥ 8 ticks since), or rate 64 and ≥ 8 ticks since, or rate > 0 and ≥ 64 ticks since.
- **`measure`** (`LAYER:170-190`): `stageBox(page, host)` (the layer's own box; viewport if no area), `survey(page, {surfaces, keepouts, frame: host})`, origin/width, `staged(seen, size)` (divide by the draw size, inset every surface 6 px from both layer edges, `LAYER:91-100`) pushed only if `!sameSurvey` (`LAYER:77-88`, compares width/height/surfaces/keepouts only), then `glances?.()` mapped into stage px and pushed as `glanced` only if the points changed (`LAYER:184-188`).
- **`show`** (`LAYER:192-216`): per actor of the frame, `depict(kind, page)` once per species, `paint(depiction, size === 1 ? actor : {...actor, x: actor.x*size, y: actor.y*size}, size)`, then keeps DOM order equal to frame order with `insertBefore` (the frame is sorted by `y`, then species id — painter's order), removes depictions of species no longer in the frame, then `announce(...)`.
- **`onCast`** (`LAYER:218-226`): called with the painted species ids in menagerie order when the joined string changes or the listener identity changes while somebody is on stage; `[]` on stop; a new listener while nobody is on stage is not called (`T-DL:573-597`).
- **Cast rotation**: `wish = "${scene} ${capacity} ${epoch}"`; `castOf(petCast(menagerie, scene), capacity, epoch, seed)` → `summoned` event when the species list changed (`LAYER:158-168`). Epoch: `if (rotateAt !== null && tick >= rotateAt) epoch += 1; if (rotateAt === null || tick >= rotateAt) rotateAt = tick + floor(randomBetween([seed, ROTATION_STREAM, epoch], 60, 120) * 64)` — only while `mode !== "still"` and not quiet (`LAYER:237-240`); a change of mode or quiet resets `rotateAt` (`LAYER:365`). `petCast` falls back to scene `"home"`, then `null` = nobody (`LAYER:64-66`). Rotation is in stage ticks, so `tempo` scales it.
- **`direct`** (`LAYER:353-372`): z-index; `tuned` on mode change (dropping queued pointer/poke events when entering `still`); `hushed` on quiet change; `rotateAt = null` and `touched = false` on those; `stale = "now"` on new `surfaces`/`keepouts` strings or `scale`; `"soon"` when `glances` appears/disappears; `redraw` on scale/`onCast` change; `pacer.wake()` when anything of that happened or scene/capacity changed.

### 1.3 Event flow: DOM → stage events

| DOM source | Registered where | Becomes | Notes |
|---|---|---|---|
| `pointermove` (window, passive) | `SURVEY:320,302-304` | `pointed {x,y}` in layer px (`clientX − origin`) | `LAYER:296-307` divides by draw size; coalesced: a newer `pointed`/`unpointed` replaces queued ones (`LAYER:304`) |
| `pointerdown` (window, passive) | `SURVEY:321,305-312` | `pointed`, plus `poked` if `button === 0`, `isPrimary !== false`, `target.closest(PET_CONTROLS) === null` and `hit(x, y)` | `poked`s are never coalesced; no `preventDefault`, no `stopPropagation` (`T-SV:489`) |
| `pointerleave` (documentElement) | `SURVEY:324,313-315` | `unpointed` for `pointerType !== "touch"` | does not bubble, hence on the root element |
| `pointerup`, `pointercancel` (window) | `SURVEY:322-323,316-318` | `unpointed` for touch only | a finger has no hover |
| `scroll` (document, capture), `resize` (window, capture) | `SURVEY:261-262` | `stale(true)` → survey with the next frame | |
| `focusin`, `focusout` (document, capture), `transitionend` (document, capture), MutationObserver (class/hidden/inert/open + childList on the whole subtree), ResizeObserver (documentElement; first greeting passed over) | `SURVEY:247-265` | `stale(false)` → survey within ≤ 8 ticks | records inside the layer are ignored except attributes of the layer itself (`SURVEY:256-258`); `transitionend` inside the layer ignored (`SURVEY:243-246`) |
| `visibilitychange` (document), `pagehide`, `pageshow` (window) | `PACING:145-157` | `rest(hidden)` → `pacer.hide()/show()` | |
| props | `LAYER:353-372` | `tuned`, `hushed`, `summoned`, resurvey | |
| `glances()` | each survey | `glanced` | |

- **Pointer in `still`** (`LAYER:296-307`): `poked` is always dropped; `pointed`/`unpointed` pass only while the pointer is within 8 px (`POINTER_REACH`, `near()` `LAYER:284-294`) of a pet or right after it was (`touched`), so the pet can turn see-through and come back; the pacer is woken only when actors are on stage (`LAYER:328`).
- **Hit-testing** (`hit`, `LAYER:272-282`): `x ∈ [actor.x·size ± width·size/2]`, `y ∈ [(actor.y − height)·size, actor.y·size]` of the *last frame*; no opacity, activity, `leaving` or perch check (the core checks grounded/not leaving in `poke`, within `POKE_REACH = 1.2` heights of the body middle, `P/🔨️modules/🎪️stage/🟦️.ts:1547-1563`; a poke is ignored while `still` or quiet, `:1548`). Floaters: `actor.y` is the bottom of the drawn body (the hover is already in `y`; e2e `misplaced` adds `hover·size` to reach the edge, `E-PW:450`).
- **See-through**: core `presenceOf` (stage): opacity eases to 0.35 while the pointer is inside the actor box grown by 4 px (1/32 per tick down, 1/16 up); on a `still` stage the frame shows 0.35 at once (`📓️report-wp-n.md` §1.5, `T-DL:414-441,443-461`). It does not change hit-testing.
- **Quiet** (a run): the layer still forwards pointer events; the core ignores pokes, turns, leans, greetings, encounters, walks and the rotation (layer: `LAYER:237`).

### 1.4 Tempo seam

`haste = clamp(tempo, 1/8, 8)` (`LAYER:128`, `FRAME_TICKS = 8`). The pacer gets `now: performance.now()·haste`, `requestFrame: raf(t => cb(t·haste))`, `setTimer: setTimeout(cb, ms / haste)` (`LAYER:263-269`). Pointer events and survey reads are *not* scaled; stage time, rotation and everything in ticks are. The quiz reads `data-pets-tempo` on the document root in every render of `QuizPets` (`GLUE:96-99,258`); nothing in the client sets it (test seam, `T-PC:584-600`).

### 1.5 Inert / hidden / forced colours / print

- `rest(hidden = false)` (`LAYER:309-312`): `pacer.hide()` when `hidden`, the document is hidden, or `host.closest("[inert], [hidden]") !== null`; else `pacer.show()` (restarts the clock, no catch-up). Called on visibility changes and on every `stale` callback, so an attribute change on the layer or an ancestor (`MutationObserver`, `SURVEY:256-259`) re-evaluates it. While resting: no frame, no timer, no survey, pets stay where they are (`T-DL:693-716,718-743,745-757`). The quiz `Dialog` makes every child of `.quiz-app` inert, the layer included (`📓️report-wp-i.md` §6).
- **Forced colours**: `useSyncExternalStore(watchForcedColors, forcedColors, () => false)` (`LAYER:426`), effect 1 returns early, the empty `<div>` stays and `CSS:81-85` hides it; zero frames, zero timers (`T-DL:759-773`, `E-PW:607-624`). Print: `CSS:87-91`. An invalid menagerie: layer rendered empty, no show, no `style` attribute (`T-DL:290-300`).

### 1.6 What a pointer-interactive pet needs from this layer (as-built gaps)

- Hit target, pointer capture, `touch-action`, `user-select`: see §3.4.
- `point()` (`LAYER:296-307`) and `watchPointer` (`SURVEY:296-332`) know only "pointer position" and "poke". Pointer ids, pointer type (except touch → unpointed), press/release pairs, capture, `lostpointercapture`, double click, long press and right button are not modelled; `pressed` returns early for non-primary buttons.
- There is no path from DOM to a stage event other than the nine kinds of `StageEvent` (`P/🧬️schema/🟦️.ts:225`): `ticked, pointed, unpointed, glanced, surveyed, summoned, tuned, hushed, poked`. New abilities (grab, release, throw, effect, command) need new event kinds in the three schema twins and both stages.

---

## 2. Survey anatomy (`SURVEY`)

### 2.1 How surfaces and keep-outs are found

`survey(root, options): Surveyed` (`SURVEY:174-209`), reads only:

1. `stage = stageBox(root, frame)` (`SURVEY:81-87`): the layer's box when it has an area, else `documentElement.clientWidth/Height` (or `innerWidth/Height`).
2. `seen(root, surfaces, keepouts)` (`SURVEY:111-122`): the elements matching each selector, document order, without anything `[inert]`/`[hidden]` or inside it. Fast path `querySelectorAll` when `root.querySelector("[inert], [hidden]") === null`; otherwise one `TreeWalker` that rejects inert/hidden subtrees at their root (the quiz always takes the walker: backdrop pages are inert).
3. **Surfaces** (`SURVEY:184-192`), per element matching `surfaces`: `box = getBoundingClientRect()`, `region = shown(box, clipOf(element))` (box cut to the boxes of scrolling/clipping ancestors, `body`/`html` never clip, `SURVEY:90-104,125-133`); skip when `region === null` or outside the stage; **always** push the element's visible box, grown 4 px left and right, as a *solid* keep-out (`SURVEY:189`); skip as a surface when `region.top > box.top` (top edge scrolled away) or visible width < 48 (`SURVEY:190`); else `{id: surfaceId(element), x0, x1, y: region.top}` in stage px. Ids are `s1, s2, …` issued once per element from a `WeakMap` (`SURVEY:62-73`) — nothing maps an id back to an element except calling `surfaceId(element)`.
4. **Floor**: `{id: "floor", x0: 0, x1: stage.width, y: stage.height}` is always the last surface (`SURVEY:193`).
5. **Keep-outs** (`SURVEY:194-207`): elements matching `keepouts` that are neither a surface nor an ancestor of one (`grounds`, `carriers`), each cut by its clipping ancestors, grown 4 px (`KEEPOUT_MARGIN`) with the rule that the margin never reaches over an edge the element lies under (`keepout()`, `SURVEY:145-152`); then the solids; then the focused element grown 8 px (`FOCUS_MARGIN`) unless it is `body`/`html`, a carrier, inert/hidden, outside the root; a focused surface keeps its whole margin.
6. Return `{kind: "surveyed", width, height, surfaces, keepouts}`.

Default selectors: `PET_SURFACES = "[data-pet-surface]"`; `PET_CONTROLS` = `a[href], button, input, select, textarea, summary, [role=button|link|checkbox|radio|tab|menuitem|option|slider|switch], [tabindex]:not([tabindex="-1"]), [contenteditable]:not([contenteditable="false"])`; `PET_TEXTS` = `p, li, h1–h6, label, figcaption, td, th, legend, dt, dd, blockquote, pre, code, output, progress, meter, [role=alert], [role=status]`; `PET_KEEPOUTS` = controls + texts + `[data-pet-keepout]` (`SURVEY:18-28`).

Several surfaces per card (the quiz): `QUIZ_PET_SURFACES = '#quiz-main [data-card] [data-slot="window-chrome-chip-cap"], #quiz-main [data-card] [data-slot="window-chrome-body-surface"], .quiz-app > footer'` (`GLUE:82`) — the title tab, the body edge beside the tab (the tab's solid blocks the body edge under it) and the footer line (whose solid also blocks the stage floor behind it). Measured: tab 72 × 26 px; first card 35/57 px under a 29 px navbar; later cards 30 px below the card above (`📓️report-wp-n.md`, `📓️report-wp-p.md` A5). `QUIZ_PET_KEEPOUTS = "[data-quiz-item], [data-quiz-drop], .quiz-app > header"` (`GLUE:87`) is appended to the layer's own `PET_KEEPOUTS` (`GLUE:183`).

### 2.2 Staleness (`watchSurvey`, `SURVEY:237-275`)

Two urgencies: `stale(true)` (scroll, resize, root ResizeObserver) → measure with the next frame; `stale(false)` (mutation of `class`/`hidden`/`inert`/`open` or child lists, `focusin`/`focusout`, `transitionend`) → within 8 ticks while running. **`style` and `data-*` attribute changes are not observed**, so inline-style writes (transforms, `translate`) on host elements never make a survey stale by themselves — and also never announce a moved card (the strip camera's `style.transform`, `LayeredOverview` `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx:486`, is only seen by the periodic re-survey: ≤ 8 ticks while rate 64, once a second while rate > 0). Toggling `class` inside the layer *is* recorded by the observer (the filter is by attribute name) and then discarded by `ignore.contains` — avoid per-frame `class` writes in new nodes.

### 2.3 Cost

Measured by G (`📓️report-wp-g.md` §2): 0.15 ms on the harness page; 0.6 – 0.9 ms on 1 700 elements with nine tenths inert; 2.8 – 3.4 ms on 1 800 elements with nothing inert (1 342 live keep-outs, box reads dominate). Cadence: see fact 5. One `getComputedStyle` per clipping candidate ancestor (cached per survey), one `getBoundingClientRect` per surface and per keep-out.

### 2.4 The `surveyed` payload (what the stage receives)

```ts
// P/🧬️schema/🟦️.ts:184-193,210
type Surface  = { id: string; x0: number; x1: number; y: number }          // top edge
type Rect     = { x: number; y: number; width: number; height: number }
type Surveyed = { kind: "surveyed"; width: number; height: number; surfaces: Surface[]; keepouts: Rect[] }
```

Stage-pixel numbers (layer origin), after the layer's `staged()`: divided by the draw size (0.8 on phones), surfaces inset 6 px. Example (`T-SV:216`, a card 200 × 100 at 100,300, a 200 × 40 paragraph at 400,300 and another at 400,500, a 1000 × 700 stage): `{kind:"surveyed", width:1000, height:700, surfaces:[{id:"s…", x0:100, x1:300, y:300}, {id:"floor", x0:0, x1:1000, y:700}], keepouts:[{x:396,y:296,width:208,height:48}, {x:396,y:496,width:208,height:48}, {x:96,y:300,width:208,height:100}]}` — the last box is the card's own solid, 4 px wider on each side.

### 2.5 What (a) walls, (b) fixtures, (c) touches would need

**(a) Walls — the left and right sides of cards with their free stretches.** The data is already in the loop at `SURVEY:184-192`: `box` and `region` of every surface element. A wall candidate is a side that is not clipped (`region.left === box.left` / `region.right === box.right`, the same test as `region.top > box.top` at `:190`), spanning `region.top … region.bottom`; for the silhouette of a quiz card there are the tab's sides (≈ 26 px high), the body's sides, and the sides of the footer line. Free stretches = the vertical span minus the keep-outs and the solids of *other* parts that touch the strip `[x − clearance, x + clearance]` (the mirror image of `perchesOf(surfaces, keepouts, width, height, clearance, minimum)`, `P/🔨️modules/🏞️terrain`, which subtracts keep-outs from the band above a horizontal edge). Open choices: report whole walls `{id, side: "left" | "right", x, y0, y1}` and let a new terrain function cut them (consistent with today), or cut in the survey. A new event field breaks `T-SV:159,213-218` (exact `toEqual`), the 15 vector scenes, the schema twins, `sameSurvey`/`staged` (inset applies to x of walls too). Room facts: desktop cards are 30 px apart vertically on non-overview pages; the overview grid has its own gaps (`--quiz-home-columns/rows`, `QR/🔨️modules/🏠️home/🟦️.tsx:251`).

**(b) Fixtures — host elements a pet may displace.** Needs: a selector option (e.g. `fixtures`, default `[data-pet-fixture]`), the key from an attribute value, and per fixture `{ id, key, x, y, width, height, room: {left, right, up, down} }`. Room = distance from the *rest* box to the nearest of: the visible box of the nearest clipping ancestor (`clipOf`, `SURVEY:90-104` already walks them), the stage edges (phone rule: `documentElement.scrollWidth − innerWidth ≤ 0`, `E-PW:481,765,838`, `S/🧪️tests/📱️phone/🟦️.ts:38,52-60`), and for scroll containers the scrollable overflow (a transformed child extends the scroll area to the right and downwards only — not measured here, validate in a browser). Two traps: (1) `getBoundingClientRect` includes whatever displacement the layer itself applied, so the survey must subtract its own writes (the layer would have to remember them) or measure layout boxes (`offsetLeft/Top`, which ignore transforms); (2) a displaced fixture that is also a surface or a keep-out (a card, a button) moves surfaces and keep-outs under the pets that stand on it — wanted for riding, but the survey must then use the displaced box consistently. The quiz already refuses cut cards: `S/🧪️tests/📱️phone/🟦️.ts:40-46` asserts `scrollHeight − clientHeight = 0` for every overview card and `sideways()` (`:15-24`) asserts that no scroll box in `#quiz-main` scrolls sideways.

**(c) Interaction with fixtures — "the user touched it".** Every candidate is already seen by a listener the layer owns, so no per-element listener is needed: `pointerdown`/`pointermove` on `window` (`SURVEY:320-321`; `event.target.closest(fixtureSelector)`), `focusin`/`focusout` on `document` capture (`SURVEY:263-264`, today only `stale(false)`), `transitionend` (`SURVEY:265`), `scroll` capture (`SURVEY:261`). The wanted signals, in order of how early they come: hover/approach (`pointermove`, `pointerover`, desktop only), press (`pointerdown`, mouse/touch/pen), keyboard focus (`focusin` — also screen readers), activation (`click`, `keydown` Enter/Space, `input`, `change`), `scroll` inside it. **Click retargeting hazard** (browser behaviour from the UI Events/Pointer Events specs, not measured here): if a fixture is displaced and is snapped back on `pointerdown`, `pointerup` lands where the element no longer is, and the `click` goes to the nearest common ancestor of the two targets instead of to the element — the intended click is lost. Touch has no hover, so for touch there is no earlier signal than `pointerdown`/`focusin`; displacement must stay inside the element's own box (so `pointerup` still hits it) or be undone by proximity before the finger lands. Other listeners that assume the DOM under the pointer is the page: quiz drag (`dropZoneAt`), presence feed — see fact 4.

---

## 3. Depiction anatomy (`DEPICT`, `CSS`)

### 3.1 Element structure per pet (`depict`, `DEPICT:151-190`)

```
<svg class="pet" data-pet="<species id>" focusable="false" overflow="visible"
     [data-pet-activity="<activity>"]  style="--pet-body:#…; --pet-accent:#…; --pet-detail:#…; transform:…; opacity:…">
  <g [transform="matrix(a b c d e f)"]>  <path|ellipse|rect|line class="pet-fill-<paint> pet-stroke-<paint>" [stroke-width] …>  </g>   // one per part, back to front
  …
  // the face after the part named by face.above, else last:
  <g [transform="matrix(bone)"]>  <g [transform="translate(x y) scale(1 open)"]>
        <circle class="pet-fill-paper pet-stroke-ink" stroke-width="1.5" r="…"/>  <circle class="pet-pupil" r="…" [cx] [cy]/>  </g></g>   // per eye
  <g [transform="matrix(bone)"]> <path class="pet-fill-none pet-stroke-ink" stroke-width="1.5" [d="M x−w/2 y Q x bend x+w/2 y"]/> </g>      // mouth
</svg>
```

The root attributes are `class, data-pet, focusable, overflow` (`DEPICT:91-98`); `data-pet-activity` is added by the first `paint` (`DEPICT:211-215`). The palette is set once, through the CSSOM, at `depict` time (`DEPICT:158-160`); `paint` never rewrites it. `Depiction` (`DEPICT:140-148`): `{species, element, followers: SVGGElement[][] per bone, sockets, pupils, mouth, painted: Float64Array}`.

### 3.2 What is written per frame (`paint`, `DEPICT:193-259`)

Memo `painted` (NaN-filled, so the first frame writes everything): slots `[x, y, flip, size, opacity, activityIndex]` (6), then 6 per bone, 3 per eye (pupil x, y, openness), 1 for the mouth. Writes, each only when the rounded value changed: `element.style.transform = "translate(Xpx, Ypx) scale(F, S)"` (x,y to 0.01, others to 0.001), `element.style.opacity`, `setAttribute("data-pet-activity")`, `setAttribute("transform", "matrix(…)")` on every follower group of a changed bone, pupil `cx`/`cy`, eye socket `transform`, mouth `d`. Only attribute and CSSOM writes (no `style` attribute, no `innerHTML`, `cssText`): `T-DP:234-247`. `depictionMarkup(species, frame)` states the same tree as text without place/opacity/palette (`DEPICT:274-290`, `T-DP:249-267`).

### 3.3 Classes and attribute hooks

Paint classes `pet-fill-{body,accent,detail,ink,paper,none}`, `pet-stroke-{…}`, `pet-pupil` (`CSS:39-79`): body/accent/detail from the per-element custom properties; ink `var(--pet-ink, var(--foreground, #001117))`, paper `var(--pet-paper, #f7f3e3)`, pupil `var(--pet-pupil, #001117)` — a host may override. `.pet { position:absolute; left:0; top:0; width:1px; height:1px; overflow:visible; transform-origin:0 0; will-change:transform; stroke-linejoin:round; stroke-linecap:round }` (`CSS:24-35`) — the actor is a 1 px box whose transform puts the feet in place; mirroring flips around the feet. `data-pet` and `data-pet-activity` are the only per-state hooks a host stylesheet has (activity written only on change).

### 3.4 Pointer events of layer and pets

- CSS: `.pet-layer { position:fixed; inset:0; overflow:hidden; pointer-events:none; contain:strict }` (`CSS:5-11`); pets inherit `none`; `.pet` sets nothing. Runtime: `host.style.pointerEvents = "none"` at start, removed at stop (`LAYER:348,343`); nothing in `DEPICT` touches it. Layer z-index 35: above cards, below the presence layer (`z-40`, `QR/🔨️modules/👥️presence/🟦️.tsx:1013`), dialogs (`z-50`, `QR/🔨️modules/🪟️chrome/🟦️.tsx:344`), and the quiz drag ghost (`1000`, `QR/🎨️.css:316`).
- The stories gallery's species view draws `svg.pet` *outside* any `.pet-layer`, so there they already are hit targets (harmless).
- **For real hit targets** (click, drag with pointer capture, touch) the following would be needed; none exists:
  - CSS on `.pet` (not on the layer, which stays `none`): `pointer-events: auto` (→ SVG shapes get `visiblePainted`, i.e. silhouette hit-testing; legs with `fill: none` are hit on their stroke only), `touch-action: none` (no scroll while dragging), `user-select: none`. The CSS regex `T-DL:317` forbids any value but `none` today. Whether an outer `<svg>` of 1 × 1 px with `overflow: visible` is hit-tested on its overflowing children is *not verified* in a browser.
  - `pointerdown` currently arrives passively on `window` (`SURVEY:321`): `preventDefault` is impossible there; capture via `target.closest(".pet")?.setPointerCapture(event.pointerId)` is possible from a passive listener; captured `pointermove`s still bubble to the same window listener.
  - Replace the rectangle `hit()` by `event.target.closest("[data-pet]")` (the species id is on the root).
  - The aria-hidden layer must stay free of focusable nodes (`T-DL:268-272`, `T-PC:798`): interaction would be pointer-only decoration; a keyboard alternative is a product decision, not a technical one.
  - `see-through` (0.35 opacity while the pointer rests on a pet) does not stop hit-testing; a pet being dragged is under the pointer all the time.
  - Hazards: quiz drag-and-drop and presence anchors (fact 4); `E-PW:233-252 takenTargets` asserts that the element under the middle of every pet is *not* in the layer.

### 3.5 Extra drawings attached to an actor, and free ones

- **Today**: a depiction is the species' parts + face only. `ActorFrame` (`P/🧬️schema/🟦️.ts:288-298`) is `{species, x, y, facing, activity, opacity, bones, eyes, mood}`; `Frame` is `{tick, actors, rate, wake}`. Nothing carries props, ropes or particles.
- **Attached to an actor without any React change**: author them as species parts on their own bone and animate the bone's `scaleX/scaleY` to 0 when absent (design §13.7: "a part cannot be hidden by opacity", H3 used sliding behind the body), selected by activity/clip. Costs nothing in `DEPICT`; changes only the art and the repertoire (and a new `Activity` if a state has no activity yet — `ACTIVITIES`, `P/🧬️schema/🟦️.ts:55`).
- **Attached but dynamic** (rope end, held ladder at a changing length/angle): needs frame data beyond bones: e.g. `ActorFrame.props: {kind, …numbers}[]`, drawn by `paint` into extra groups appended to the same `<svg class="pet">` (inherit mirroring and scale), with their own memo slots (`ACTOR_SLOTS` layout, `DEPICT:29,189`). Mirroring flips them with the pet — a rope end anchored in *world* coordinates must be counter-transformed.
- **Free ones** (ladder leaning on a card, rain, hearts, beams): need a second kind of drawable in `show()` (`LAYER:192-216`), e.g. a Map `props` next to `depictions`, ordered against actors in the same DOM sequence (`insertBefore` logic, `LAYER:205-207`). Draw them in sibling elements of the layer (not under a `.pet`), classes other than `.pet` (so `will-change: transform` does not promote hundreds of layers), coordinates ×`scale` like actors (`LAYER:204`). For many particles: pooled elements and only `transform`/`cx`/`cy`/`d` writes (no `class` writes, no child-list churn — both are seen by the survey's MutationObserver, §2.2), or one `<path d>` per effect. Canvas would not be blocked by the CSP (no `img-src` is declared and none is needed), but the repo's tests and e2e helpers read `svg.pet` children (`E-PW:245,314`).
- **Without ids**: no `<defs>`, `<use href>`, `clipPath`, `marker`, `filter url(#…)` under the current rule (fact 8) — a parachute canopy, rope and rungs are plain paths/lines.
- **Per-state looks**: (1) CSS only — `.pet[data-pet-activity="squabble"] .pet-fill-body { fill: color-mix(in srgb, var(--pet-body) 70%, red) }` in `CSS` (static stylesheet, `style-src 'self'`; `T-DP:151-154` allows no `transition`/`animation`/`@keyframes`/`@apply`/`@import`); (2) a new attribute written like `data-pet-activity` (only on change); (3) palette rewrites via `style.setProperty("--pet-body", …)` in `paint` (needs a memo slot). Extra parts per state: see above.

---

## 4. Pacing (`PACING`) and what drag/particles would mean

- `createPacer(step, seams): {wake, hide, show, stop}` (`PACING:59-131`). Accumulator over rAF timestamps; `TICK_MILLISECONDS = 15.625`, `FRAME_TICKS = 8` (max ticks per running step; the rest is dropped, never caught up). A step happens when `64 ÷ rate` ticks are due, or at once after `wake()`: rate 64 every frame, 32 every other, 16 every fourth, 0 no frame at all and one timer for `Frame.wake` (the step that ends the sleep carries every tick slept — the stage jumps over lulls in O(1)). `hide()` cancels frame and timer; `show()` restarts the clock without catching up.
- `Frame.rate` ∈ {0, 16, 32, 64}, from the core: 64 while any actor walks, hops, falls, lands, turns, appears, leaves, is see-through-easing or plays a non-looping clip; 32 for gaze springs/blinks/looping clips; 16 while every actor sleeps; 0 when nothing moves (`TK/📓️design.md` §5.7, §13.3).
- **Dragging**: the layer wakes the pacer on every accepted pointer event (`LAYER:328`), so a drag at rate 0 would still be painted once per move, but `ticks` after a rest are delivered unbounded (`PACING:101`) and the core "lull" logic would jump over time around a held actor: the core must know about held actors (rate 64, no lull). The pacer has no override for "keep running": `step()` returns `Pace` straight from `frameOf` (`LAYER:251`); a layer-side override would be `{...shown, rate: held ? 64 : shown.rate}`. Pointer coordinates are in real time while stage time scales with `tempo` (test seam only).
- **Particles**: either inside the core (deterministic, enters TS/Rust parity, `advance` copies drafts per event and `frameOf` allocates per frame — 100s of particles make that O(n) per step) or render-only on the React side (cheap, but outside parity and tests). Either way they need the pacer at 64/32 while alive, and with today's coupling that also means a survey every 8 ticks and a `glances()` call (fact 5) although no actor moves; a separate "effects rate" or decoupling the survey cadence from `rate` would avoid it. `tempo` scales particle time if it is tick-based.
- Pinned: `T-FP` (16 tests) pins the accumulator, rates 64/32/16/0, the single timer at rate 0, hide/show; `T-DL:389-412` pins "a still stage paints once and schedules nothing".

---

## 5. Stories gallery (`STORIES`, `PR/📖️stories/🌐️.html`, `PR/📖️stories/🎨️.css`, `PR/🏗️builder/🌐️vite/🟦️.ts`)

- Dev server only: `bun nx run @semio-tech/pets-react:dev`, port `PETS_STORIES_PORT` (default 6069), menagerie `PETS_MENAGERIE` (repo-relative path, default `P/🧫️fixtures/🧬️schema-conformance/🔣️.json`); `.claude/launch.json:626-642`: `pets-stories` (6069) and `architecture-pets-stories` (6074, `AP/🟦️.ts`). The page imports `{origin, source}` from the virtual module `pets-stories:menagerie` (`PR/📖️stories/🌐️.html:11-13`, `PR/🏗️builder/🌐️vite/🟦️.ts:20-47`) and `menagerieOf` finds the outermost valid menagerie among the exports (`STORIES:129-146`).
- **Species view** (`Gallery`, `STORIES:355-391`): controls `mood`, `lid`, blink, face left, speed (0–2), zoom (`Controls`, `:151`); a roster; per species on a light and a dark ground the rest pose and every clip looping (non-looping clips replay after a pause), captioned `id · seconds · loop/once · activities that use it` (`clipRoles`, `:321-325`). One rAF loop + one `IntersectionObserver` for all specimens (`openStudio`, `:168-219`); frames are hand-made by `specimenFrame` (`:222-235`): `sampleClip`/`solveRig`, `activity: "idle"` and `opacity: 1` hard-wired, pupils via `lookOffset`/`pupilReach`.
- **Sandbox** (`Sandbox`, `STORIES:416-467`): mock terrain — three cards (`data-pet-surface`), a `button.tab` on an edge, a `span.note[data-pet-keepout]` on an edge, a scrolling `.shelf` of five cards (`overflow: auto`, 260 px) — and `<PetLayer key={seed} menagerie scene mode quiet capacity scale seed />` (`:464`). Controls: scene (from the casts), mode incl. off, quiet, capacity (1–8), scale (0.6–2), seed + new seed, **Poke a pet** (`pokeSomePet`, `:396-402`: a synthetic touch `pointerdown` on `document.body` at a random pet, which reaches the window listener), scroll the shelf, rearrange the cards (`room-0/1` grid), take a card away. English/German labels (`LABELS`, `:24-109`), dark toggle.
- **Not exposed**: `surfaces`/`keepouts` selectors, `glances`, `onCast` read-out, `tempo`, forcing an activity/state/trick, drag, effects, fixtures, walls.
- **How it could exercise round two**: (1) sandbox terrain: cards that have free vertical sides (current mock cards are bordered boxes with `data-pet-surface`), a wall segment that is part covered by a `note`, and elements flagged as fixtures (`data-pet-fixture`) — a button and a card, one inside a clipping `.shelf`, one near the viewport edge — plus a switch for a simulated `prefers-reduced-motion` stylesheet (`.stories * {transition-duration: 0.01ms !important}`) to reproduce R's regression; (2) a *director*: the layer has no imperative seam, so forcing an activity/trick needs a new prop or event channel (e.g. `commands` prop or a core `commanded` event) — the gallery would only be a select + button; (3) drag: dispatch `pointerdown` → `pointermove`* → `pointerup` on the pet (synthetic like `pokeSomePet`) *and* a real-pointer mode once pets are targets; (4) species view: a state/palette select and prop toggles that feed new `ActorFrame` fields through `specimenFrame` into `paint`; activity select writing `frame.activity` (drives `data-pet-activity` CSS looks); (5) `tempo` slider, `onCast` read-out, a mock peer cursor via `glances`. The studio loop and the layer are independent, so effects can be previewed in the species view without the stage.

---

## 6. Quiz glue

### 6.1 Wiring (`GLUE`, `QR/🟦️.tsx`)

- Entry: `QuizOptions.pets?: QuizPetsSource` (`QR/🟦️.tsx:197`; `type QuizPetsSource = () => Promise<Menagerie>`, `GLUE:114`); the site passes `pets: () => import("../🐾️pets/🟦️.ts").then((module) => module.ARCHITECTURE_MENAGERIE)` (`S/🟦️.ts:33`). `QuizApp` wraps `Client` in `<QuizPetsProvider source choice={preferences.pets} chosen={preferences.petsChosen} state locale>` (`QR/🟦️.tsx:528-530`); inside `Client`: `const pets = effectivePetMode(preferences.pets, preferences.petsChosen, usePetsReduced())` (`:407`), `data-pets={pets}` on `.quiz-app` (`:454`), `<LegalFooter …><PetsSwitch …/></LegalFooter>` (`:476-478`), `<PresenceOverlay …/>` (`:479`), `<QuizPets />` last (`:480`). Exports `QR/🟦️.tsx:101-103`.
- `QuizPetsProvider` (`GLUE:160-199`): `reducedMotion = useMediaQuery("(prefers-reduced-motion: reduce)")`, `forcedColors = useMediaQuery("(forced-colors: active)")`; `mode = effectivePetMode(choice, chosen, reducedMotion)`; loads `source()` and `import("@semio-tech/pets-react")` together only when `mode !== "off"` and nothing is loaded (`wanted`, `:175-190`); a failure is swallowed, a fetch outliving the provider or the wish is dropped; `scene = petScene(state.step, state.runs, state.catalog)`; **`quiet = state.step.screen === "run"`** (`:192`); `shown = {…loaded, scene, mode, quiet, onCast: setOnStage}`; `names = petNames(menagerie, onStage, locale)`.
- `QuizPets` (`GLUE:252-261`) renders, inside an error boundary (`PetBoundary`, `:237-247`), exactly `<Layer menagerie scene mode quiet surfaces={QUIZ_PET_SURFACES} keepouts={pets.keepouts} glances={peerGlances} onCast={pets.onCast} tempo={petsTempo()} />` (`:258`). `keepouts` = `` `${PET_KEEPOUTS}, ${QUIZ_PET_KEEPOUTS}` `` (`:183`). The type-only import of `PetLayerProps` (`:25`) keeps the render target out of the entry chunk.

### 6.2 Mode rules

- `PET_CHOICES = ["off", "still", "calm", "lively"]` (`GLUE:31`); `effectivePetMode(choice, chosen, reducedMotion) = choice === "off" || chosen || !reducedMotion ? choice : "still"` (`:43-45`): the device's reduced-motion hint decides the **default only**; any choice the learner made (`QuizPreferences.petsChosen`, set by `withPets` only, `QR/🔨️modules/🎛️preferences/🟦️.tsx:105-107`; read as `true` only for a stored `true`, `:86`) holds on every device. `switchedPets(shown, before)` (`GLUE:49-51`): footer switch = `off` ↔ last liveliness (`petsLiveliness`). Forced colours: the layer renders nobody whatever the mode (`LAYER:426-429`), the preferences say why (`usePetsForced`, `PreferencesPanel:173,204`).
- `PreferencesPanel` row (`QR/🔨️modules/🎛️preferences/🟦️.tsx:199-207`): `Segments` pressing the effective mode; notes `petsForced` / `petsReduced`; the cast line `petsCast` ("Here right now: …" / "Gerade hier: …"). i18n keys `quiz.preferences.pets|petsOff|petsStill|petsCalm|petsLively|petsCast|petsReduced|petsForced|petsShown` in `QR/🔨️modules/🌐️i18n/🟦️.ts:117-125` (en) and `:500-508` (de).
- Footer switch `PetsSwitch` (`GLUE:223-232`): a native checkbox in a label, nothing on a site without pets (`offered`), rendered as `children` of `LegalFooter` before its `nav` (`QR/🔨️modules/⚖️legal/🟦️.tsx:95-104`); it sits inside `.quiz-app > footer`, which is itself a surface (so the switch is a control = keep-out on its own floor).
- **Where "quiet" suppresses interaction**: `quiet` → layer `direct` → `hushed` event → core: no walks, hops, fidgets, encounters, turns, leans, greetings, rotation; pokes ignored (`P/🔨️modules/🎪️stage/🟦️.ts:1548`; `T`-stage unit `:985-988`); pupils still follow the pointer. The layer itself does *not* filter by quiet (pointer events are still forwarded). `still` is filtered in the layer (`LAYER:299-303`) and in the core. Results are not quiet (a score is read, the switch is on that screen). Dialogs are not "quiet" but "inert": the whole show rests (`LAYER:309-312`).
- `data-pets-tempo` seam: `GLUE:93-99`.

### 6.3 Glances

`peerGlances()` (`GLUE:103-108`): the top-left corners of `[data-presence-layer] .quiz-peer:not([hidden])` (other learners' cursor marks) in viewport px; sampled with each survey, mapped to stage px (`LAYER:184-188`).

### 6.4 What the DOM carries about quiz / task / item today

Species `grounds` are `<quiz>`, `<quiz>/<task>`, `<quiz>/<task>/<item>` (`AP`, e.g. `heating/u-values/wall-geg`, `physics/power-or-energy/kettle`; checked by `S/🧪️tests/🐾️pet-cast/🟦️.ts:118-125`). The glue knows the **quiz** only through `scene` (`petScene`); the DOM hooks, by screen (all in `QR/🔨️modules`, `data-card`/`data-presence-anchor` set by `QuizCard`, `🪟️chrome/🟦️.tsx:60`):

| Screen | Element | Attributes (example values) | Carries |
|---|---|---|---|
| Overview (desktop `data-mode="strip"`, phone `"list"`) | `[data-layered-overview]`, section `[data-layered-section="heating"]` | `LayeredOverview 🟦️.tsx:756,760,779` | quiz id as page key |
| | `[data-layered-card="heating"]` (also `learner`, `intro`, `board`, `badges`, `prefs`), pane `[data-layered-pane="heating"][data-opened]` + `inert` while not opened | `LayeredOverview:413,370-376` | quiz id (`pane.id`) |
| | quiz card `section[data-overview-card][data-card="quiz:heating"][data-presence-anchor="home:quiz:heating"]`, heading link `href="#heating"`, `[data-overview-card-action="primary"]`, `span[data-learning="2"]` | `📖️quiz-page:84-86`, `OverviewCard:97,152`, `:62` | quiz id |
| | other cards: `data-card="learner"` (anchor `home:learner`), `introduction`, `board` (`home:board`), `badges`, `preferences` (`home:preferences`) | `📇️profile:106`, `👋️introduction:30,56`, `🏆️leaderboard:273`, `🏅️badges:49`, `🎛️preferences:244` | none |
| Quiz page (pane) | `div[data-page="quiz:heating"]`; cards `data-card="quiz"` (anchor `quiz:heating`), `"quiz-tasks"` (anchor `quiz:heating:tasks`; list items carry **no** task ids), `"quiz-crowd"` (no anchor) | `🪟️chrome:107`, `📖️quiz-page:85,162,168,121` | quiz id |
| Run | `section[data-card="run"][data-presence-anchor="run"]` (progress, nav buttons without ids, `[data-complete]`); `section[data-card="task"][data-presence-anchor="task:u-values"]` | `▶️run:117-118,171-172,152` | **task id**; quiz id only via state; **no task-kind attribute** |
| Run, classification | `li[data-quiz-drag][data-quiz-item="wall-geg"][data-presence-anchor="item:wall-geg"]`; pool `section[data-quiz-drop="pool"]`; bins `section[data-quiz-drop="category:<cat>"][data-presence-anchor="category:<cat>"]`; grip `[data-quiz-grip]` | `🗂️classification:50,78,93`, `🧩️task:94` | **item id**, category id |
| Run, sorting | `li[data-quiz-drag][data-quiz-item="<id>"][data-presence-anchor="item:<id>"][data-quiz-drop="item:<id>"]` | `↕️sorting:204-207` | item id |
| Run, matching | per card `li[data-quiz-drag][data-used]` (no id); per row `tr[data-quiz-drop="slot:u-value:wall-geg"][data-presence-anchor="item:wall-geg"]` | `🃏️matching:77-78,114` | item id + dimension id (**no `data-quiz-item`**) |
| Run, crowd | `div[data-crowd-task="<task>"]`, `figure[data-crowd-figure="answers"][data-task][data-dimension][data-runs][data-thinkers]`, `tr[data-crowd-item="<item>"][data-answers]`, `td[data-key][data-count][data-own][data-correct][data-live]`, `figure[data-crowd-figure="scores"]`, `li[data-bin]` | `🗳️crowd:438,313,346,356,395,416` | task, item |
| Results | `section[data-card="results"][data-presence-anchor="results"]`, per task `section[data-card="task-result"][data-presence-anchor="result:u-values"]`, `li[data-earned]` (badges); item rows are `th[scope=row]` with the label as text only | `🏁️results:214,229,262,246` | **task id**; item ids only in crowd rows |
| Presence | `[data-presence-layer] .quiz-peer[data-anchor][data-x][data-y][data-tag][data-drag]` | `👥️presence:952,957` | anchor keys (`PRESENCE_ANCHORS`, `:725-738`: `run`, `results`, `task:<id>`, `result:<id>`, `item:<id>`, `category:<id>`, `quiz:<id>`, `quiz:<id>:tasks`, `home:<card>`) |

`LayeredOverview` = `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview/🟦️.tsx`, `OverviewCard` = `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🃏️OverviewCard/🟦️.tsx`; the other module names are `QR/🔨️modules/<name>/🟦️.tsx`.

So a ground can be derived today without new attributes as `scene / (closest [data-card=task] anchor minus "task:") / (data-quiz-item | slot row item | data-presence-anchor "item:…")` for run items; results and the quiz page's task list would need an attribute (task id in the page's `li`, item id on result rows). The quiz must mark whatever the pets are to react to with a stable attribute and keep it in the quiz (the glue cannot import values from the pets chunk — `GLUE:25` and `📓️report-wp-i.md` §5.11 on the duplicated `PET_HOME_SCENE`; a test holds duplicates equal).

---

## 7. Tests that pin today's behaviour

(Test names as in the files; line = first line of the cited assertion or test.)

### 7.1 `T-DL` (`P/🧪️tests/🫥️decorative-layer/🟦️.tsx`, 29 `it`)

| Test | Lines | Pinned |
|---|---|---|
| afterEach | 214-225 | console silent (`log, info, warn, error, debug, trace`), 0 timers after every test, DOM cleaned |
| "renders one static element hidden from assistive technology…" | 233-253 | layer attribute names exactly `["aria-hidden","class","style"]` (239), class exactly `pet-layer` (240), no children before the first frame (242) |
| "exposes nothing to assistive technology and nothing to the keyboard" | 255-275 | no roles, all nodes `isInaccessible`, every node `tabIndex === -1`, no `tabindex/role/id/title/aria-label/aria-labelledby/aria-live/href/contenteditable` (270), not `a, button, input, select, textarea, summary, details, iframe, object, embed, audio, video, title, desc, style, script, text, foreignObject` (271), `focusable="false"` (273), no text (274) |
| "never takes pointer events without its stylesheet either…" | 277-288 | `layer.style.pointerEvents === "none"` (283), computed `none` on every descendant without CSS (285), style attribute empty after unmount (287) |
| "shows nothing of a menagerie the core rejects…" | 290-300 | attributes only `["aria-hidden","class"]` (298) |
| "never takes pointer events: the stylesheet says so…" | 302-319 | computed `none` on layer and every node (310-314); `expect(css).not.toMatch(/pointer-events:\s*(?!none)\S/)` (317); no `animation-name:` (318) |
| "never transitions…" | 321-345 | computed `transition-property: none` for layer and every node under a host rule `transition-duration: 0.01ms !important` (336-341); the CSS rule text (342) |
| "paints a still stage once and schedules nothing…" | 389-412 | one rAF in total (394, 409), no `setTimeout` (399, 410), pointer events away from pets leave `innerHTML` identical, `setAttribute` never called (400-408) — **writes nothing in `still`** |
| "lets a pet of a still stage turn see-through at once…" | 414-441 | one extra frame (425), only opacity changes (427), **a `pointerdown` on the pet in `still` does nothing more** (428-430), no `setAttribute` (438), no timer (439-440) |
| "lets a pet of a living stage turn see-through…" | 443-461 | easing 1 → 0.35 → 1 |
| "follows a still stage's surfaces when the page moves…" | 463-483 | exact positions +30/+50; rAF count 2 |
| "keeps a calm stage alive outside React…" | 485-530 | pupils follow (499-505); **poke**: a `pointerdown` dispatched on a `button` at a pet's position does nothing (510-513), on `body` raises that pet's mouth only (514-518); `defaultPrevented` false (528); `vi.getTimerCount() === 1` (527); `renders === 1` (529) |
| "turns changed props into events…" | 532-571 | same element reused (545, 556), timers 0 in `still` (566-568), 1 when lively again (570) |
| "tells its host who is on stage…" | 573-597 | `onCast` semantics |
| tempo, rotation, narrow stage | 599-617, 619-655, 668-691 | tempo clamps, rotation never on `still`/quiet |
| hidden / inert / hidden-start | 693-716, 718-743, 745-757 | 0 timers, `innerHTML` unchanged while resting |
| "stays empty and runs nothing under forced colours" | 759-773 | no frame requested |
| "leaves no listener, observer, frame, timer or element behind…" | 775-806 | listeners exactly: window `["pagehide","pageshow","pointercancel","pointerdown","pointermove","pointerup","resize"]`, document `["focusin","focusout","scroll","transitionend","visibilitychange"]`, root `["pointerleave"]` (785-789); observers `{resize:1, mutation:1}` (790); one timer (791); `zIndex ""` and removed after stop (797) |
| StrictMode | 808-840 | rAF 2 (823), listener counts `[7, 5, 1]` (829), observers 1/1 |
| seed, silent fault, glances | 842-889 | |

### 7.2 `T-DP` (`P/🧪️tests/🖌️pet-depiction/🟦️.tsx`, 11 `it`)

108-122 root attributes (`class, data-pet, focusable, overflow, data-pet-activity`), `querySelector("style, script, title, desc, a, [tabindex]")` null, no id, no `[style]`; 140-155 CSS rules: `.pet-layer` declarations `position: fixed; inset: 0; overflow: hidden; pointer-events: none; contain: strict;` (147), `.pet` declarations (148), forced-colours/print (149-150), **the only `transition…` declaration is `transition-property: none !important;` and no `animation|@keyframes|transition|@apply|@import|@tailwind`** (151-154); 157-167 CSSOM place/opacity/palette; 217-232 **writes nothing when nothing changes; exact mutation lists for single changes**; 234-247 no `setAttribute("style")`, `innerHTML`, `outerHTML`, `cssText`; 249-267 `depictionMarkup` equals the live tree; no `style|id=` (266).

### 7.3 `T-SV` (`P/🧪️tests/📡️surface-survey/🟦️.tsx`, 15 vector scenes + 12 other `it`)

136-140 vector margins; 142-165 each scene's `Surveyed` equals `{kind, width, height, surfaces, keepouts}` exactly (159-162) and `document.body.outerHTML` is unchanged (163: reads only); 167-179 root-relative; 213-218 exact objects; 228-249 stable ids; 251-274 `PET_CONTROLS` == WAI-ARIA widgets; 276-388 watch: observed options `{subtree, childList, attributes, attributeFilter: SURVEY_ATTRIBUTES}` (314), page listeners exactly `["focusin","focusout","scroll","transitionend"]` and window `["resize"]` (316-317), all `{capture, passive}` (318), urgencies (327-346), transitions inside the layer ignored (341-344), observer cleanup; 390-409 greeting passed over; 421-496 pointer: window listeners `["pointercancel","pointerdown","pointermove","pointerup"]` + root `["pointerleave"]`, `{passive: true}` (425-427), **poke rules** (450-486: `pointed`+`poked` on a primary press on a pet; not on interactive ancestors, secondary or non-primary pointers or misses; touch `pointerup`/`cancel` → `unpointed`), no `preventDefault` (489); 498-504 no `hit` → no poke.

### 7.4 `T-FP` (`P/🧪️tests/⏲️frame-pacing/🟦️.tsx`, 16 `it`)

Constants (95-99), one frame per wake (101-112), rate 64/32/16/0 behaviour, hide/show, browser seams. Unaffected unless the pacer changes.

### 7.5 `T-PC` (`Q/🧪️tests/🐾️pet-companions/🟦️.tsx`, 33 tests)

- **Exact props handed to the layer** `toEqual({menagerie, scene, mode, quiet, surfaces: QUIZ_PET_SURFACES, keepouts: "button, p, " + QUIZ_PET_KEEPOUTS, glances: peerGlances, onCast: any Function, tempo: 1})` (`:441`); one `onCast` identity (442).
- Selectors against real markup: `QUIZ_PET_SURFACES` matches exactly `[chip-cap, body-surface of the task card, FOOTER]` (569-575), `QUIZ_PET_KEEPOUTS` (577-581).
- Client: real layer `aria-hidden` (796), **is `.quiz-app`'s last child** (797), **contains no `a, button, input, select, textarea, [tabindex]`** (798), each card has only its `chip-cap` in jsdom (802), footer is last surface (803).
- Footer: buttons in the footer are exactly `["What is stored"]` (829); switch not inside `nav` (828).
- Preferences object shape: 112 (`PREFERENCES`), 261, 313 (`toHaveBeenLastCalledWith`), plus `Q/🧪️tests/🏠️home-grid/🟦️.tsx:71` and `Q/🧪️tests/📡️presence-client/🟦️.tsx:845` — a new preference field changes all of them; 284-299 every other control must call `onChange` with `petsChosen: false`.
- Tempo seam 584-600; peer glances 602-626; `translation-completeness` (quiz suite) needs both bundles for new keys.

### 7.6 `E-PW` (`S/🧪️tests/🐕️pet-walk/🟦️.ts`, 13 tests, project `pets`, last in the gate)

- **"No pet is a hit target"**: `takenTargets` (233-252) asserts that the element under the middle of every control/card **and the middle of every pet's drawing** is not inside `.pet-layer`; called at 473, 477, 491, 503, 574, 652, 670, 717, 750.
- Test 1 (463-482): `data-pets="calm"`, one layer, `aria-hidden`, `toHaveCSS("pointer-events","none")` (469), no `a, button, input, select, textarea, summary, [tabindex], [role], [id], style, script` inside (470), no horizontal overflow (481).
- **"Writes nothing"**: `watchStill`/`expectMotionless` (164-192; `places:false` = no mutation record of any kind in the layer for 90 frames) at 537 (`still`), 570 and 573 (reduced motion, also while the pointer crosses the window), 589; `places:true` during a run (501).
- Reduced motion (556-605): `layerTransitions` must be `{property: "none", running: 0}` (595; counts `document.getAnimations()` targeting `.pet-layer` only, 203-207); calm pets walk after choosing (591-603).
- Quiet/run: nobody changes place (501); cast follows (484-505).
- `misplaced` (439-461): every whole pet's feet are on an edge shown on screen within 1.5 px (452), no two bodies on one edge overlap (453-457), **no body overlaps an `a[href], button, input, select, textarea, summary` or a card tab/body by more than 1 px** (458); polled at 629-651, 687, 821-836 — a pet that climbs, hangs, is dragged, parachutes or leans on a wall fails 452/458 while it is in that state for as long as the poll sees it.
- Pointer attention (690-718): idle pets face and look at the pointer; see-through 0.35 only for the pointed pet (655-671, 667).
- Phone (804-841): ≤ 2 pets, size 0.8 (815-816), overflow ≤ 0 (838), `problems` empty.
- Footer switch (753-802): overflow ≤ 0 (765).
- `S/🧪️tests/🐾️pet-cast/🟦️.ts:96-99`: `Object.keys(architecturePets)` equals `["ARCHITECTURE_MENAGERIE"]`; 59-63 directories vs ensemble; 118-125 every ground exists in the quiz files; 127-161 casts.
- Phone spec `S/🧪️tests/📱️phone/🟦️.ts`: overflow 38, 52-60; `sideways()` 53, 56, 61; every overview card cut nowhere (40-46).

### 7.7 Core-level pins of poke/quiet/still (TS and Rust twins)

`P/🔨️modules/🎪️stage/🧪️tests/🔬️unit/🟦️.ts:954-1011` ("a poke in $name": greets towards the poke and cheers up 959-970; reach 1.2 heights 972-983; **ignored in a time of concentration** 985-988; wakes a sleeper and stops a walker 990-995; busy partner and sulk 997-1010), `:1315-1320`, trace stories `:822,1624,1676`; Rust twin `P/🔨️modules/🎪️stage/🧪️tests/🔬️unit/🦀️.rs` (14 mentions); stage-trace vectors `P/🧫️fixtures/🎪️stage-trace/🔣️.json` (23 mentions of `poked`), `P/🧪️tests/🎪️stage-trace/🥒️.feature:20`. Docs that state the rules: `P/README.md:195-204`, `Q/README.md:445-475` (`a click on a pet does nothing` during a run, `:465`; `never in the way of the pointer`, `:474-475`), `S/README.md:102,120`, `AP/README.md`.

---

## 8. Constraints

### 8.1 CSP (rehearsal and release; `S/🚀️deploy/🟦️.ts:311-340`)

`<meta http-equiv="Content-Security-Policy">` before the first script; `default-src`, `script-src`, `style-src` each only `'self'`, `'none'` or a `sha256-…` of an inline block; `connect-src` exactly the proctor origin and its `ws` twin; `object-src` and `base-uri` `'none'`; `form-action` restricted. Consequences for new rendering: no `eval`/`new Function`, no inline `<style>` and no `style="…"` *attribute* (`setAttribute("style", …)`), but CSSOM writes (`el.style.x = …`, `style.setProperty`) are allowed and are what the layer uses; SVG presentation attributes and same-document `url(#id)` references are fine; no `img-src`/`font-src` → fall back to `default-src 'self'`, so `data:`/`blob:` images and fonts are out; workers must be same-origin files; any network request other than the proctor is blocked (the layer makes none). The pets stylesheet is a hashed same-origin asset linked by the lazy chunk (`CSS` is imported by `PR/🟦️.tsx:12`). The e2e device records every `securitypolicyviolation` as a failure.

### 8.2 Release budget (`QUIZ_SITE_BUDGET`, `S/🚀️deploy/🟦️.ts:100`: script 260 000 B gzip, style 60 000 B gzip, artifact 4 000 000 B)

Latest numbers (R, 15:02, rehearsal build): entry script 865 859 B raw / **250 475 B gzip** (9 525 B headroom; it grew 22.5 kB between 14:00 and 14:24 through other tickets); lazy pets stylesheet 963 B. P's build (earlier that day): entry style 37 944 B gzip; lazy menagerie 331 822 B (36 199 gz), layer + core 54 156 B (20 163 gz), validation 8 979 B (3 267 gz); 62 files, 2 556 999 B. Only the scripts and styles the document links count against the 260 000/60 000 B budgets; the lazy chunks do not, but the glue in `QR` is part of the entry, and a value import of `@semio-tech/pets-react` from the glue would drag the whole target into it.

### 8.3 E2E device guard (`E-LN:150-235`)

Every `device` fails its test on: any console message of type `error` (`:173-175`; warnings are not caught), `pageerror` (176), `requestfailed` other than `net::ERR_ABORTED` (177-180), any response ≥ 400 (181-183), a websocket error other than "closed before the connection is established" (184-188), and every `securitypolicyviolation` (194-197), unless inside `expectingFailures`. The phone test in `E-PW:804-841` has its own reduced guard (console `error` + `pageerror`). The core of the pets must write nothing to the console (`T-DL:214-217`); the layer ends a faulty show silently (`LAYER:254-262`). Dev-topology cost: a fresh learner makes ~265 requests, 40 of them pets (22 menagerie documents, 17 modules); `net::ERR_NO_BUFFER_SPACE` on a loaded host fails whichever request it hits (`📓️report-wp-p.md` §3.4).

### 8.4 Phone overflow rule

`document.documentElement.scrollWidth − window.innerWidth ≤ 0` (asserted in `E-PW:481,765,838` and `S/🧪️tests/📱️phone/🟦️.ts:38,52,55,60`); no box in `#quiz-main` scrolls sideways (`sideways()`, `:15-24,53,56,61`); every overview card on the phone is "cut nowhere" (`:40-46`). The layer cannot cause any of it today (`position: fixed; inset: 0; overflow: hidden; contain: strict`, `CSS:5-11`); a displaced host element can (transforms extend the scrollable overflow).

### 8.5 The reduced-motion transition fix (R) — exactly

- **Cause**: `QR/🎨️.css:597-603` `@media (prefers-reduced-motion: reduce) { .quiz-app *, ::before, ::after { transition-duration: 0.01ms !important } }`. An element without its own `transition-property` transitions *all* properties, so every frame's write into a pet (`transform`, `opacity`, `cx`, `d`, …) started a transition; each `transitionend` was taken by `watchSurvey` for a reason to measure and to wake the pacer.
- **Fix 1** (`CSS:13-20`): `.pet-layer, .pet-layer * { transition-property: none !important }` — descendant selector, so any new element created *inside* the layer is covered (pseudo-elements are not; SVG elements have none).
- **Fix 2** (`SURVEY:219-221,243-246`): `watchSurvey(…, {ignore: host})` — a `transitionend` whose target is inside `ignore` does not call `stale`; every other `transitionend` still does.
- **Tests that hold it**: `T-DL:321-345`, `T-DP:151-154` (the CSS may contain exactly that one `transition` declaration), `T-SV:338-346`, `E-PW:595` and helper `layerTransitions` (`:203-207`, counts animations *inside the layer* only).
- **What new moving parts must not regress**:
  1. Anything moved *outside* the layer — displaced host elements, a ladder or rope drawn into a portal, outlines — is under the quiz's blanket rule and under the host's own transitions (`QuizCard` has `transition-transform duration-200` when it opens a page, `🪟️chrome/🟦️.tsx:73`; `.quiz-peer` has `transition: transform 120ms linear`, `QR/🎨️.css:374-387`). Each write then starts a transition, and `settled` does not ignore its `transitionend` → survey + wake storm. Remedy has to be part of the design: set `transition-property: none` (CSSOM, CSP-safe) on a fixture while the layer drives it, and make `settled` ignore `transitionend` of driven fixtures (it only knows the layer element today).
  2. Do not toggle `class`/`hidden`/`inert`/`open` on host elements to displace them: they are `SURVEY_ATTRIBUTES` and mark the survey stale. `style`/`data-*` writes are invisible to the observer (§2.2).
  3. Web Animations (`element.animate`) would be counted by `layerTransitions` (`document.getAnimations()`), and `T-DP:151-154` forbids `animation`/`@keyframes` in the stylesheet.
  4. `E-PW:595` must be extended to count animations/transitions of fixtures too, otherwise the next regression is invisible to it.
  5. R could not prove the renderer crash (access violation, once) was caused by the storm; none in ~350 exposures afterwards (`📓️report-wp-r.md` §6.1).

### 8.6 Other constraints worth knowing

- Two-language rule: every core change (schema, stage events, terrain) has a Rust twin and a Protocol v2 case with a third-party oracle (parity 183/183 at close); the React target and the glue are TypeScript only.
- Docstrings start with a unique emoji; no comments inside definitions; the tree edited by others concurrently — anchored edits only (design §12 rules).
- Labels in both languages (`translation-completeness`, `🗣️both-languages` fails on identical wording in both).
- The repo MCP server was down for the previous work; tickets were kept by hand.

## 9. Not verified (read-only session)

Hit-testing of an outer `<svg>` of 1 × 1 px with overflowing children; click retargeting after a snap-back; scroll-overflow behaviour of transformed descendants in the real quiz; whether `contain: strict` changes pointer hit-testing for overflowing SVG content; none of the test results cited here were re-run — they are the figures of reports N, P, R and of the closing summary.
