# 📓️ Work package C1a (second round): the learner's hand in the browser — grasp, walls in the survey, the layer's API

Ticket `2026/10/02/QUIZ-PETS`, second round, phase C; design-v2 §14 (decisions 1, 5, 6), §17, §21, §22, §23. `P` = `🧰️framework/🛍️products/🐾️pets`, `PR` = `P/🎯️targets/⚛️react`, `QR` = `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`, `TK` = this ticket folder. All results below are from runs on 2026-10-03 (Windows, bun 1.4.2); the work was interrupted by an API rate limit between ~02:22 and 12:19 and resumed. Evidence kept: `TK/🗑️generated/c1a/browser/*.png`, `TK/🗑️generated/c1a/drive-final.json`.

## 1. Files

| File | What |
|---|---|
| `PR/🔨️modules/🤏️grasp/🟦️.ts` | **new**: the hand (`watchGrasp`, `PET_CURSOR`, `STIR_MILLISECONDS`, types `Grasp`, `GraspHost`, `GraspOptions`, `Grasped`) |
| `PR/🔨️modules/📡️survey/🟦️.ts` | anchored: `PET_PRESS_CONTROLS`, `SurveyOptions.walls`, `wallId`, walls in `survey()`, `seen()` generalised to a list of selectors (C1b's `props` now rides on it), `PointerOptions.controls`, `Pointed.over` with the "busy" rule |
| `PR/🔨️modules/🫧️layer/🟦️.tsx` | rewritten once at 02:21 on R0's version (before C1b's Scenery region, which C1b added on top at ~12:27): new props, `PetLayerHandle`, `permitted`, the hand, touch pads, coalescing, survey cadence while held, teardown |
| `PR/🎨️.css` | anchored: `.pet-pad` rule; header sentence |
| `PR/🟦️.tsx` | barrel: grasp exports, `PET_PRESS_CONTROLS`, `wallId`, `PET_COARSE_POINTER`, `PetLayerHandle` |
| `PR/📦️packages/🟦️typescript/package.json`, `bun.lock` | devDependency `@testing-library/user-event` `^14.6.3` (already resolved in the lockfile for the quiz package; I added the same line to the pets-react workspace block of `bun.lock` by hand, exactly as `bun install` writes it — `bun install` was not run) |
| `P/🧪️tests/🤏️pet-handling/🟦️.tsx` | **new** suite, 26 tests (17 of the hand against user-event, 9 of the layer against a fake core) |
| `P/🧪️tests/🫥️decorative-layer/🟦️.tsx`, `P/🧪️tests/📡️surface-survey/🟦️.tsx`, `P/🧪️tests/🧰️gear-depiction/🟦️.tsx` | rewritten pins (§5) |
| `P/🧫️fixtures/📡️surface-survey/🔣️.json` | regenerated: every scene gains `expected.walls`, 3 new wall scenes (18 scenes) |
| `TK/generate_survey_vectors.py` | walls (`walls_of`, option `walls`), 3 scenes, docstring |
| `P/🔮️oracles/🔣️.json` | new oracle `pets-react-user-event` (capability `pets-react-pet-handling`) |
| `P/README.md` | React target: props table (`play`, `mischief`, `controls`, `walls`, `props`, `ref`), the hand paragraph |
| `TK/c1a_harness/{index.html, main.tsx, vite.config.ts, c1a_pets_recorder.ts}` | browser harness on port 6253 (records every stage event and the last frame) |
| `TK/c1a_drive.mjs` | Playwright drive of the hand (mouse, keyboard, touch via CDP) |
| `TK/c1a_probe_stage.ts` | feeds the core the layer's events outside React (told a broken in-flight core from a broken layer) |
| `TK/c1a_code_rules.mjs` | docstring emojis, no line comments, no console/debug logs, no forbidden writes |

## 2. The shell contract as built (`🤏️grasp`)

**Where it listens.** Window, capture phase, ahead of every listener of the page: `pointerdown` (not passive), `pointermove`, `pointerup`, `pointercancel`, `lostpointercapture`, `wheel`, `input` (passive), `keydown`, `click`, `selectstart`, `dragstart`, `contextmenu` (not passive); window `blur` in the bubbling phase (so an element's blur, which passes the window only while capturing, is not mistaken for the window's); document `scroll` (capture, passive). 14 listeners, all removed by `stop()`.

**When a press is the pets'** (all must hold): primary button (`button === 0`), primary pointer, not already cancelled, no Shift/Ctrl/Meta/Alt (they extend a selection, open a menu or a link), no interactive ancestor of the target (`controls()`: `PET_PRESS_CONTROLS` = `PET_CONTROLS` + `label`, plus the host's `controls`), no non-collapsed document selection, and `takes(x, y)` — in the layer: play permitted, mode not `still`, the show not resting (hidden document, inert/hidden layer), and the point inside the `body` of an actor of the last frame whose opacity is above 0 (stage units: layer pixels ÷ size).

**What it does with that press only:** `preventDefault()` (no compatible `mousedown`, so no focus change and no text selection; `selectstart`, `dragstart`, `contextmenu` are also cancelled while the press is open), `stopPropagation()` (no listener of the page hears the `pointerdown`), `document.documentElement.setPointerCapture(pointerId)` (released on end; failures of synthetic pointers are passed over), and the click it leads to is swallowed (`preventDefault` + `stopPropagation` at window capture) — also after the press was called off while the button was still down. A key typed after the button is up, a cancelled pointer, a window blur or the next press forgets the swallow, so a keyboard click always passes (tested).

**What it emits** (layer pixels; the layer divides by the size): `pressed {x, y, pointer}` (`mouse | pen | touch`; an unknown type counts as mouse), `dragged {x, y}` for every move of that pointer, `released {x, y}`, `cancelled` on Escape (that Escape is also `preventDefault` + `stopPropagation`: it belongs to the press; every other Escape passes), `pointercancel`, `lostpointercapture`, window `blur`, the host's `cancel()`; a new primary press while one is open cancels the old one first. `stirred` for every `pointerdown`, `keydown`, `wheel`, `input` anywhere, at most once per `STIR_MILLISECONDS` = 1000 (a finger is a pointer, so no `touchstart` listener is needed); `scrolled` for every scroll of the page or a pane. Whether a press was a click, a hold or a pick-up is not decided here (A6's press machine in the core).

**Cursor.** `data-pet-cursor` on the document element (an attribute, never a class: the survey's MutationObserver hears `class`): `grab` while a mouse/pen rests (no button) on a pet `takes` accepts with no control under it — re-judged after every frame, so a pet that walks away from a resting pointer takes the hand with it —, `grabbing` while a press is open and the frame holds a pet (`Frame.held !== null`). When the frame stops holding a pet during an open press (the stage let go by itself), the hand ends the press quietly (no event, capture released, the click still swallowed).

**Verified that the quiz does not need the stopped press** (read, not run in the quiz): presence (`QR/🔨️modules/👥️presence:811-814`, document capture `pointerdown`) only resets its keyboard flag and calls `presence.drag(draggedItem(target))`, which is `undefined` for anything but a grip — a pet press moves no focus, so a stale keyboard flag changes nothing; the panorama (`LayeredOverview` window `pointermove`) and the card reveal (`pointerenter/leave`) do not listen to `pointerdown` at all. So `stopPropagation` stays. Two effects that are not of the stopping but of the capture: while a pet is held, boundary events go to the document element, so the hovered home card gets `pointerleave` (it conceals while a pet is held, reveals again after the release); and C1b's reclaim listeners (document capture) never see a taken press — a lifted original is reclaimed by `pointerover` before any press lands on a pet over it.

## 3. Survey (my part)

- `walls`: the left and right sides of every surface element (in surface element order), then of every other element matching the host's `walls` selector (walls only: no surface, no solid, no keep-out), left before right. A side counts only where it can be seen (`region.left === box.left` / `region.right === box.right` after the clipping ancestors); it runs from the top of what one can see down to its bottom (`y0`, `y1` = the visible region), `x` = the box side, stage pixels. Surface elements narrower than 48 px or with their top scrolled away still have sides. Id `wallId(element, side)` = `"<surfaceId>:left|right"` (stable per element and side), `surface` = `surfaceId(element)`. Off-stage elements have none; inert/hidden ones none. Free stretches are the core's (`wallsOf`).
- `seen(root, selectors[])` (one walk or one query per selector) replaced the two-list version; C1b's `props` is its fourth selector now.
- `Pointed.over`: `"control"` while the target has an interactive ancestor (`controls()`, default `PET_PRESS_CONTROLS`) **or** while the learner is busy with a press the pets did not take (a text selection, a host's drag): from the event after its `pointerdown` until its `pointerup`/`pointercancel` or the first move without a button. A taken press never reaches this feed (stopped at window capture), so every press it hears is somebody else's.
- Fixture part: C1b's (`props`, `fixtures`, `pointer` option); I left `seen()` and the return statement as the seam.

## 4. The layer's API (for C3 and the quiz)

```tsx
<PetLayer
  menagerie scene mode quiet capacity surfaces keepouts glances scale seed zIndex onCast tempo   // as before
  play?: boolean        // default true  → `permitted.play`; false: no press is taken, no pads, a held pet is called off
  mischief?: boolean    // default true  → `permitted.mischief` (C1b releases lifts when it goes false)
  controls?: string     // more controls a press never takes away from, appended to PET_PRESS_CONTROLS
  walls?: string        // more elements whose sides are walls
  props?: string        // C1b: fixtures, default [data-pet-prop]
  ref?: Ref<PetLayerHandle>
/>
interface PetLayerHandle { play(species: Slug, deed: Deed): void }   // deed: "hello" | "trick" | "pet" | "toss"
```

- **Deeds: `ref` + `useImperativeHandle`** (React 19 passes `ref` as a prop) — chosen over a callback registration because it is the idiomatic channel for imperative commands into a component and keeps the props comparable. `play` before the show runs, after it ended or under forced colours does nothing; while it runs it queues `played {species, deed}` and wakes the pacer; the stage decides (still or play not permitted: no answer). The glue imports only the type, so the entry chunk stays free of the target: `const pets = useRef<PetLayerHandle>(null)` in `QuizPetsProvider`, handed to `<Layer ref={pets} …/>` and to the "Play with the pets" group.
- **Recommended quiz props (C3):** `controls="[data-quiz-grip]"` (the drag grip is a `span[aria-hidden]`, not a control by any default selector — without it a press on a pet flying over a grip would be the pet's), `play={preferences.petsPlay}`, `mischief={preferences.petsMischief}`.
- **At mount** the stage gets `tuned`, `hushed`, `permitted` (R0: the stage's `play`/`mischief` are false until then). A change of `play`/`mischief` sends `permitted` again; switching play off, going `still`, a hidden document or an inert layer call a held pet off (`cancelled`).
- **What reaches the stage of the hand:** every `pressed`, `released`, `cancelled`, `played`; of `dragged` the latest of each step (a newer one replaces a queued one unless a press/release/cancel came after it) — one per animation frame; `stirred` and `scrolled` once per step. On a `still` stage nothing of the hand but `cancelled` is queued (so a still stage still writes and schedules nothing).
- **Survey cadence:** as before, except that `rate === 64` forces a survey every 8 ticks only while no pet is held — a held pet keeps the stage at full rate without costing 8 surveys a second (it falls back to once a second; scroll and resize still survey at once).
- **Touch pads:** on `(pointer: coarse)` (`PET_COARSE_POINTER`, read live), while play is permitted and the stage is not still, one `div.pet-pad` per visible actor with `footing === "perch"` in a `div.pet-pads` appended to the layer: the actor's `body` × size, clipped at its feet (so it lies in the headroom band the perch was cleared in, never over a control), transform/width/height by CSSOM only when the rounded value changed; removed when the actor is in the air, in the hand, on a rope/ladder/wall, invisible, or play stops; the container goes with the show. CSS: `pointer-events: auto; touch-action: none; user-select: none; -webkit-touch-callout: none` — the only rule of the stylesheet that takes pointer events (pinned in two suites). Taps work everywhere without pads.
- **Teardown:** `stop()` ends the hand (listeners, capture, cursor attribute), removes pads, then C1b's lifting and scenery. Listener ledger of a running layer: window 20 (`pagehide, pageshow, resize`, pointer feed 4, hand 13), document 6 (`focusin, focusout, scroll` ×2, `transitionend, visibilitychange`), root 1 (`pointerleave`); StrictMode `[20, 6, 1]`.

## 5. Test changes and why

| Suite | Change | Reason |
|---|---|---|
| `🫥️decorative-layer` "keeps a calm stage alive…" | the poke part (a press on the body at a pet raised its mouth; `defaultPrevented` false for both presses) → a press on a button at a pet's place is not prevented and reaches the page; a press on the pet is prevented and does not reach a document listener | "pets never take a click" is replaced by the hand; how the pet answers a click is B2's (the core decides) |
| same, "never takes pointer events: the stylesheet…" | renamed; the regex "no `pointer-events` but `none`" → "only `.pet-pad` declares another value"; no pad on a fine pointer | touch pads |
| same, listener ledgers | `[7,5,1]` → `[20,6,1]`, the exact sets listed | the hand's listeners, honestly counted |
| `🧰️gear-depiction` "has the rules…" | same CSS regex change | it pinned the same claim |
| `📡️surface-survey` scenes | `walls` from the vectors (`wallId`, `surfaceId`), `options.walls` | walls |
| same, "sees nothing … behind the scenes" | `walls` of the card | walls |
| same, pointer feed | R0's interim version rewritten: `over` stays `control` during a press the pets did not take, ends with its release or a buttonless move; secondary/non-primary presses do not make it busy; a label is a control | the busy rule, `PET_PRESS_CONTROLS` |
| same, new | "asks the host which elements are controls at every event"; "names the sides of a surface as walls…" | `controls()`, `wallId`, host walls |
| `🤏️pet-handling` (new) | 17 hand cases with user-event as oracle (taken press: no `pointerdown`/`mousedown`/`click` on the page, no focus, no selection, capture set and released; control/label/host grip keep press, click, focus; text selection beside a pet, a selection holds back the next press, a taken press selects nothing; drag captured and followed beyond the window; quick press/release told as such; Escape; pointercancel/lostpointercapture/blur/host cancel, never twice; no press for another button, Shift, a second finger, a miss, play off, a pre-cancelled press; touch tap; keyboard click after a press without click passes; quiet let-go; a new press cancels the open one; cursor states by attribute, never class; stir throttle and scroll; selectstart/dragstart/contextmenu only during a press; listener set, options and teardown mid-press) + 9 layer cases with a fake core (`advance` records, `frameOf` shows test actors): menagerie valid; `permitted` at mount and on change; pressed/latest drag/released in stage pixels at size 2; control under a pet, still and play off take nothing; grabbing while held and `cancelled` on play off/still/hidden, nothing on unmount but a clean cursor; `played` via the handle and nothing after unmount; pads (coarse only, perch only, size, clip at the feet, `pointer-events: auto`, removed in the air/hand, on play off and still, gone with the layer); survey cadence while held; one stir and one scroll per step, none on a still stage | the brief's list |

## 6. What I saw in the browser (harness on 6253, headless Chromium 1280 × 800, sample menagerie `blobby`/`hoppy`/`floaty`)

`node TK/c1a_drive.mjs` → `failed: []`, console empty (the page's own console hook and Playwright's). Screenshots looked at: `1-hover` (three pets on the card tops; `data-pet-cursor="grab"` over floaty), `3-dragging` / `4-released` (the pet stays where it stood: B1 does not lift yet, `Frame.held` was `null`, so the cursor stayed `grab`/none — correct by the contract), `5b-button-under-a-resting-pet` (floaty drawn over the "Under" button with `keepouts=loose`: the button had `pointerdown`, `mousedown`, `click`; the stage nothing), `6-selected` (69 characters of the paragraph selected by a drag beside the pets), `8-touch-pads` (pads are invisible). Readings: click on a pet → `pressed, released` at the same point, the page heard nothing; hold and drag 160 × 120 px → `pressed`, 20 `dragged` (one per frame), `released` at the release point within 0.01 px, the page heard nothing; Escape mid-drag → `… cancelled`, no `released`, the page heard neither the Escape nor a click; `played {floaty, hello}` through `ref.current.play`; the last survey carried 6 walls (`s1:left` … `s3:right`); touch (`hasTouch`, `isMobile`): `(pointer: coarse)` true, 3 pads for 3 grounded pets, `pointer-events: auto`, `touch-action: none`, each pad is what the browser hits at its middle; a tap → `pressed(touch), released`; a CDP touch drag of 10 moves on a pad → `pressed`, 10 `dragged`, `released`, no `pointercancel`, the page heard nothing. Seen on the way: a pet moves away from a button that gets focus (the focus keep-out) and from a control placed under it (keep-outs) — the core's rules, not the shell's.

## 7. Commands and real results

| Command | Result |
|---|---|
| `PR/📦️packages/🟦️typescript`: `bun ./📜️script.ts typecheck` | exit 0 (12:41). Earlier runs during the day failed only in core files of B1/B2/R0 in flight (`🕰️clock` `slip`/`cheer`, `🎥️projection` `spirits`) |
| same: `bun ./📜️script.ts test` | exit 0, `Test Files 8 passed (8)`, `Tests 166 passed (166)`, 11.4 s (12:41). At 12:22 the decorative-layer's living-stage cases failed because the core could not load (`cheer` missing in `👀️attention`, B2 in flight; confirmed by `TK/c1a_probe_stage.ts`), green again once it was restored |
| same: `bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts pet-handling` | `Tests 26 passed (26)` |
| `QR/📦️packages/🟦️typescript`: `bun ./📜️script.ts test` | exit 0, `Test Files 22 passed (22)`, `Tests 801 passed (801)`, 49.8 s |
| same: `bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts pet-companions translation-completeness` | `Test Files 2 passed (2)`, `Tests 45 passed (45)` |
| `🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript`: `bun ./📜️script.ts test` | exit 0, `Test Files 5 passed (5)`, `Tests 138 passed (138)` |
| repo root: `.venv/Scripts/python.exe TK/generate_survey_vectors.py` | `{"scenes": 18, "surfaces": 53, "keepouts": 90}`; fixture `+1129` lines, insertions only (every old expectation unchanged) |
| repo root: `bun node_modules/vite/bin/vite.js --config TK/c1a_harness/vite.config.ts`, then `node TK/c1a_drive.mjs` | `failed: []` (output `TK/🗑️generated/c1a/drive-final.json`); server stopped by its PID afterwards |
| `node TK/c1a_code_rules.mjs` | all 14 files `clean` |
| `bun TK/taxonomy_name_probe_v2.ts` | `🤏️grasp` and `🤏️pet-handling` already registered (bytes checked: `f0 9f a4 8f ef b8 8f`) |

Not run, as instructed: e2e gate, deploy checks, `bun install`.

## 8. Decisions

1. Play and mischief default to `true` (the preferences' defaults, design §14.6); a host that wants pets without play passes `play={false}`.
2. Presses with a modifier key and presses while text is selected stay the page's (selection must not break).
3. `label` is a control for presses and for `over` but not in `PET_CONTROLS` (that selector stays exactly the WAI-ARIA widgets, as its suite pins).
4. Escape that cancels a held pet is kept from the page (it belongs to the press; otherwise the same Escape would also close the opened quiz page).
5. Pads are clipped at the feet and limited to `perch`; their DOM is a lazy `div.pet-pads` at the end of the layer.
6. `stirred` is throttled in the hand (1/s, by `performance.now`) and coalesced in the layer; it wakes the pacer so the core's idle clocks are never late by more than a second.
7. No `touchstart` listener: `pointerdown` covers fingers.
8. The harness is a temporary ticket tool and is not registered in `launch.json` (like the harnesses of the first round).

## 9. Open (for others)

- **B1/B2/B3:** `Frame.held` is always `null` today, so `grabbing`, the quiet let-go and the held survey cadence were proven only against the fake core; the shell sends everything the press machine needs (A6 §8). `pointed` keeps flowing during a drag (with `over: "free"`: the target is the document element under capture).
- **C3 (quiz):** pass `play`, `mischief`, `controls="[data-quiz-grip]"` and a `ref` for the settings group (§4); `T-PC`'s exact-props pin will change with that.
- **C4 (site e2e `🐕️pet-walk`):** `takenTargets` and "layer `pointer-events: none`" hold for fine pointers only; on the phone project (`hasTouch`) pads take touches over grounded pets. The home card under a held pet conceals during the hold (capture) — worth one assertion.
- **C1b:** a taken press never reaches the reclaim listeners (stopped at window capture); while quiet, "any input reclaims" therefore misses a press on a pet — the core sees that `pressed` and may drop the lift itself.
- The harness, drive and probe stay in `TK`; `TK/🗑️generated/c1a/` holds only the final drive JSON and the screenshots.
