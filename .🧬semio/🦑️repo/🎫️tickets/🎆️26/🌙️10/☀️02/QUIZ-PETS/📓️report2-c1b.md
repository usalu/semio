# 📓️ Work package C1b (second round): lifted copies of host elements, fixtures in the survey, the new drawings in the layer

Ticket `2026/10/02/QUIZ-PETS`, second round, phase C. `P` = `🧰️framework/🛍️products/🐾️pets`, `PR` = `P/🎯️targets/⚛️react`, `TK` = this ticket folder. Tool output: `TK/🗑️generated/c1b/`. Work 2026-10-03 01:20–01:55 and (after the rate-limit cut at ~03:30) 12:19–12:40, Windows, bun 1.4.2, while R0, B1–B3, C1a and H agents edited the same tree.

**State: done.** Lifting, fixtures in the survey and the depiction/scenery wiring are in; the React package typechecks and its 8 suites pass (166 tests); the quiz `🐾️pet-companions` suite and the site suite pass; the browser check passes on light and dark. The core does not emit lifts, ladders, particles or tools yet (`🎥️projection` returns `[]`, B3/B5), so in the running layer these paths are exercised by the unit suites and the harness only.

## 1. Files

| File | What |
|---|---|
| `PR/🔨️modules/🪞️lifting/🟦️.ts` | **new**: `liftFixtures`, `copyOf`, `LIFT_LOOK`, `LIFT_REACH`, `RECLAIMS`, types `Lifting`, `LiftingOptions` (lifts are the schema's `LiftFrame`) |
| `P/🧪️tests/🪞️fixture-lifting/🟦️.tsx` | **new** suite, 22 tests |
| `PR/🔨️modules/📡️survey/🟦️.ts` | fixtures: `PET_PROPS`, `PROP_KEY`, `FIXTURE_ELEMENTS` (80), `UNCOPYABLE`, `fixtureId`, `copyable`; `SurveyOptions.props`, `.pointer`; `survey()` fills `fixtures` (C1a did the walls and generalised `seen()`; I added the fourth selector) |
| `PR/🔨️modules/🖌️depiction/🟦️.ts` | `paint` draws the second-round actor frame: tilt about the pivot (mirrored with facing), state tint, spirits mouth; `tiltDegrees`, `tiltedPlacement` (moved from `🧰️gear`) and `tintPalette` (moved from `✨️effects`) live here now |
| `PR/🔨️modules/🧰️gear/🟦️.ts`, `PR/🔨️modules/✨️effects/🟦️.ts` | behaviour-free: the moved functions are imported from `🖌️depiction` |
| `PR/🔨️modules/🫧️layer/🟦️.tsx` | anchored edits in C1a's file: region `Scenery` (`stageScenery`, type `Scenery`) and the wiring of lifting and scenery into `startShow` (§3) |
| `PR/🟦️.tsx` | barrel: survey fixture names, lifting, `stageScenery`/`Scenery`, moved depiction names |
| `P/🧪️tests/🖌️pet-depiction/🟦️.tsx` | +3 tests (tilt with gl-matrix, state tint, scenery) |
| `P/🧪️tests/📡️surface-survey/🟦️.tsx` | scenes carry `pointer` and `options.props`, expected `fixtures`; +1 test (ids, frame, rule 7) |
| `TK/generate_survey_vectors.py`, `P/🧫️fixtures/📡️surface-survey/🔣️.json` | fixtures expectation + 3 fixture scenes (21 scenes, 8 fixtures); old scenes keep `fixtures: []` |
| `P/🔮️oracles/🔣️.json` | capability `pets-react-fixture-lifting` on `pets-react-gl-matrix` (placement) and `pets-react-aria-query` (accessibility tree), one sentence of rationale each |
| `TK/c1b_harness/{index.html, main.ts, vite.config.ts, pets_barrel.ts}`, `TK/c1b_browser_check.mjs`, `TK/c1b_code_rules.mjs`, `TK/c1b_module_names.ts` | harness (port 6255), Playwright check, code-rule check, taxonomy probe |
| `TK/render_species_preview.entry.ts`, `TK/wp_a9_gear_sheet.entry.ts`, `TK/wp_a9_mutants.ts` | A9's tools follow the moved functions (both entries still bundle with `bun build`; the mutants tool was not re-run) |

## 2. Lifting as built (`🪞️lifting`)

**Contract.** `liftFixtures(host, { props?, quiet?, reclaimed })` → `{ copies, realise(lifts, origin, size), measured(), release(), end() }`.

- `realise(frame.lifts, origin, size)` per frame. A lift seen for the first time finds its element among `props` (default `[data-pet-prop]`, never inside the layer) by `fixtureId`. It is **refused** (and `reclaimed` sent at once) when the element is gone, inert/hidden, not `copyable`, holds the focus, is `:hover`ed, or has no box.
- **Original**: keeps place, size, content, attributes, focusability and listeners. Only two inline CSSOM properties change: `transition-property: none !important` for the whole lift, and `opacity: 0 !important` exactly while the lift's opacity rounds to 1 (A8: the copy fades in and out lying on the element, so there is never a hole or a double). The author's declarations (value and priority) are restored. A `style` attribute that only the lift created is removed again, so `outerHTML` is identical afterwards.
- **Copy** (`copyOf`): `cloneNode(true)`. From every node it strips `id, name, for, form, style, autofocus, tabindex, accesskey, contenteditable, draggable, role, title, popover*, headers, list, slot, nonce`, every `data-*`, `on*` and `aria-*`, and `href` on `a`/`area`. It gets the computed look of its source node through `style.setProperty` for `LIFT_LOOK`: box/flow, flex/grid, gaps, alignment, margins, padding, borders, radii, backgrounds, box-shadow, colour, opacity, visibility, font longhands incl. `font-variant-numeric`, line-height, spacing, text decoration, lists, overflow, object-fit and SVG paints. `position: fixed` becomes `absolute` and `sticky` becomes `relative`. Every HTML box and `<svg>` is held to its source's on-screen size (`border-box`, `flex: 0 0 auto`). Every node also gets `pointer-events: none` and `animation-name: none`. The copy root is `inert`, `aria-hidden`, `position: absolute` at the layer corner, `margin: 0`, `user-select: none` and `transform-origin: 50% 50%`. Form state is carried over by property writes: the select index, field values and checkedness.
- **Placement**: `translate(box.left − origin.x + dx·size, box.top − origin.y + dy·size) rotate(tilt·360deg)`, opacity = author opacity × lift opacity. Writes only when the rounded values change (0.01 px, 0.01°, 0.001).
- **Reclaim**: while something is lifted (and only then) the document has capture-phase passive listeners for `pointerover pointerdown focusin keydown input change dragstart selectstart click` and `visibilitychange`, and the window one for `pagehide`. An event whose target or one of its 12 ancestors is a lifted original restores that original synchronously and calls `reclaimed(fixture)`. While `quiet()` is true any of these events anywhere restores everything; `hidden`/`pagehide` restore everything too.
  - A reclaimed fixture is **not lifted again until a frame without it arrives**, so the stage hears first.
  - `measured()` (after every survey) gives back a lift whose original moved by more than 0.5 px, was removed, or became inert/hidden.
  - `release()` gives back everything and tells the stage. `end()` gives back everything silently and removes the listeners.
  - Restore order: copy removed → opacity restored → `getComputedStyle(original).opacity` read (the style change happens while `transition-property` is still `none`) → `transition-property` restored. So neither the lift nor the undo starts a transition under any host rule.
- **Limits**: one listener set for all lifts; at most 80 elements per fixture; no table parts, canvas/video/audio/iframe/object/embed or custom elements (cloning a custom element would run its author's code). An original that scrolls is given back with the next survey rather than followed. Shadow DOM content is not copied (none in the quiz). Pseudo-elements appear only where the copied classes still match outside the original's ancestry.

**Decisions beyond the brief** (recorded here):
- `click` added to the reclaim set, for activation without a press, e.g. assistive technology or `element.click()`.
- `transition-property: none` on the original besides `opacity` (instant undo; R's reduced-motion storm, REACT §8.5).
- Additional stripped attributes (`tabindex`, `role`, `title`, `aria-*`, `href`…) so that no selector a host or an e2e spec uses (`button`-free checks aside) finds a hook in the layer.
- `reclaimed` is sent for **every** give-back while the show runs (user, survey, hidden, scene, `still`, mischief withdrawn): the reason is not part of the event; how the pet reacts (thrown off) is B5's.

## 3. Survey: fixtures (`📡️survey`)

`survey(root, { …, props, pointer, frame })` reports, in document order, `{ id: fixtureId(element), key: data-pet-prop, x, y, width, height }` for each element matching `props` that meets all of these:
- it has a non-empty key and is not inert or hidden;
- it is shown whole: no clipping ancestor cuts it, and it lies wholly on the stage, since a copy would be drawn beyond every clip;
- it is not, and does not contain, the focused element;
- the pointer is not over it (edges included; `pointer` is in viewport px);
- it is `copyable`;
- it is not inside `frame`.

Nothing but `data-pet-prop` is read (a test spies `getAttribute` and `dataset`: rule 7). The vectors generator models the same rules from text with tag tables (`UNCOPYABLE_TAGS`, `TABLE_PART_TAGS`, hyphenated tags, element counts). It runs three new scenes: rows with keys, a keyless row, a film, a custom element, 81 versus 80 elements, a table row and a surface that is also a fixture; a scrolling pane, off-stage, inert, hidden, focused and pointer-covered elements; a host selector and a framed stage. The existing 18 scenes keep `fixtures: []`. `margins.fixtureElements = 80`.

## 4. The depiction seam (`🖌️depiction` + layer `Scenery`)

- **Module graph kept acyclic.** `🧰️gear` and `✨️effects` draw with `🖌️depiction`'s rules (`maker`, `partShape`, `rounded`), so the depiction cannot import them. A9's wiring sketch (`tiltedPlacement` from gear inside `paint`) would have closed a cycle.
  - So `tiltDegrees`/`tiltedPlacement` (placement of the actor's `<svg>`) and `tintPalette` (the palette custom properties `depict` already wrote) moved into `🖌️depiction`.
  - The composition of tools, ladders and particles lives in the module that already imports all three: the layer, as the documented region **`Scenery`** (`stageScenery(host, kinds, copies)`).
- **`paint`** (successor in place): memo grows from 6 to 10 actor slots (tilt degrees, pivot x/y — 0 while upright —, state index).
  - The transform is `tiltedPlacement(...)`: exactly the old string while upright.
  - The tint is written only when the state index changes (an unknown state or one without a tint shows the palette).
  - The mouth reads `spirits`.
- **`Scenery`**:
  - `behind()`: the last of the copies and the ladder rack at the head of the layer; the actor walk starts after it.
  - `actor(depiction, actor)`: on the first frame equips the actor (gear `back` group prepended, `front` group appended, emitter pools stocked); every frame runs `paintTools`; when the state changes, `tintEffects`.
  - `stage(frame, size)`: the rack is inserted after the copies with the first ladder, the effects root is appended last with the first particle; then `paintLadders` and `paintEffects`.
  - `retire(species)` and `strike()`.
  - Lazy, so a stage without ladders and particles adds no element (T-DL's child counts and listener ledgers stay valid).
- **Layer wiring** (anchored edits in C1a's file, re-read before each):
  - `lifting` and `scenery` are created in `startShow`.
  - `show()`: `lifting.realise`, `scenery.stage`, the walk starts at `scenery.behind()`, `scenery.actor` after `paint`, `scenery.retire` for a departed species.
  - `measure()`: `survey(…, props, pointer)` (the pointer kept from `watchPointer` in viewport px), then `lifting.measured()`.
  - `rest()` (hidden or inert) → `lifting.release()`.
  - `direct()`: a new scene, entering `still` or mischief withdrawn → `release()`.
  - `stop()`: `lifting.end()` and `scenery.strike()`.
  - `reclaimed` → `waiting.push({ kind: "reclaimed", fixture })` + `pacer.wake()`.
  - The props prop (`props`) and `sameSurvey`/`staged` for fixtures were already in C1a's layer.

## 5. Tests and why they changed

| Suite | Change | Why |
|---|---|---|
| `🪞️fixture-lifting` (new, 22) | Copy fidelity: every `LIFT_LOOK` value equals the source's computed value, sizes, form state; no hooks or ids; inert/aria-hidden. Original untouched: `outerHTML` without the style is identical while lifted, and exactly identical (style declarations too) afterwards; listeners and tabIndex intact. Placement against gl-matrix corners. One test per `RECLAIMS` event on a node 12 levels deep (and none from unrelated targets). Also: focus, a filled field, a picked option, reach limit 12/13; no re-lift before the stage heard; quiet; hidden/pagehide/moved/inert/release; refusals (focus, 81 elements, canvas, `tr`, custom element, no box, unknown id). `getRoles` empty and `isInaccessible` for every copy node; listeners only while lifted, `{capture, passive}`; no writes on an unchanged frame; only CSSOM writes (the `cssText` calls are jsdom's echo of the cloned and the removed `style` attribute, asserted exactly). | brief §1, §4 |
| `🖌️pet-depiction` (+3) | Tilt about the pivot against gl-matrix for both facings and sizes (head leans the right way); no write for sub-0.005° tilts or pivot moves while upright; tint written only on a state change; `stageScenery` order, laziness, no writes, tint of particles, strike. | new frame fields; seam |
| `📡️surface-survey` (+1, scenes) | Expected `fixtures` from the vectors; `pointer`/`props` options; ids stable, frame excluded, only the key read. | fixtures |
| `🧰️gear-depiction`, `🎆️effect-painting` | unchanged (they import from the barrel). R0 had already moved the depiction suites onto the new frame fields; the pins "no `<defs>`/`<use>`, no ids" hold unchanged. | — |

## 6. Browser (harness on 6255, `TK/c1b_browser_check.mjs`, screenshots looked at)

The page is quiz-like: a card with three task rows marked `data-pet-prop`. One row has `letter-spacing` set by the CSSOM. Rows have a host `transition: opacity 300ms`. There is a field and a select. A CSP meta tag sets `style-src-attr 'none'`. Lifting is driven by hand along A8's real `liftAt`. Housy is drawn as the pusher.

- `lifted-{light,dark}-card.png`: the copy of row 2 lies 48 px (the pusher's width, travel at unit 1) to the right, beside its transparent original. Borders, fonts, glyph, `letter-spacing` and the muted kind label are faithful on both themes. `shove-*`: mid-shove with the slight rise.
- `elementFromPoint` at the original's middle returns the original (not the layer); `elementsFromPoint` on the copy never returns a copy node; a real mouse click there reaches the row's click listener (`clicks [0,1,0]`) and gives it back.
- Instant give-back verified for hover (pointer moved onto the row), `focus`, Playwright `fill`, a script-dispatched `input`, `selectOption`, and on the quiet page any key. After each give-back: 0 copies, computed opacity "1" at once, 0 running animations.
- Along the whole lift path (351 samples, ages 0–700) there is never a hole and never a half-transparent pair.
- No opacity transition ran on any row, also under `.app * { transition-duration: 0.01ms !important }`; the only transitions were the host's own hover/background and the harness's start-up letter-spacing.
- No CSP report, no console output, no sideways scroll, and the rows' inline styles afterwards are exactly the authored ones.
- `scenery-{light,dark}.png` (`--scenery`, whole core): housy tilted 14.4° about its scruff in the "hot" tint, gun → rope → hook on the card's top edge, a 14-rung ladder standing against the card behind it, rising rings in front. The layer order is `pet-ladders, pet, pet-effects`.

## 7. Commands and real results

| Command | Result |
|---|---|
| `bun ./📜️script.ts typecheck` in `PR/📦️packages/🟦️typescript` | exit 0 (12:35; earlier runs failed only in C1a's/R0's in-flight files) |
| `bun ./📜️script.ts test` (same) | `Test Files 8 passed (8)`, `Tests 166 passed (166)` |
| `bun ./📜️script.ts test pet-companions` in `QR/📦️packages/🟦️typescript` | `1 passed`, `Tests 33 passed (33)` |
| `bun ./📜️script.ts test` in `🎓️teaching/🏛️architecture/❓️quiz/📦️packages/🟦️typescript` | `Test Files 5 passed (5)`, `Tests 138 passed (138)` |
| `python -X utf8 TK/generate_survey_vectors.py` | `{"scenes": 21, "surfaces": 57, "keepouts": 101, "walls": 84, "fixtures": 8}` |
| `node TK/c1b_browser_check.mjs` (harness `C1B_PETS=subset`) and `… --scenery` (whole core) | `"failed": []` both |
| `node TK/c1b_code_rules.mjs` | every file `clean`, exit 0 |
| `bun build` of A9's two preview entries | both bundle (81.0 KB, 42.56 KB) |
| `bun TK/taxonomy_name_probe_v2.ts` | `🪞️lifting`, `🪞️fixture-lifting` already registered |

The harness server was started and stopped by its own PIDs (18988, 81960, 41924, 76108, 61576); 6255 is free.

## 8. Open, and notes for others

- **B5 / B3**: the core emits `lifts: []`. When it lifts, the layer path is already wired. `reclaimed` comes for every give-back (§2), and a fixture is refused and reported at once when it cannot be lifted. The survey never reports a fixture under the pointer or holding the focus.
- **C1a**: please keep the `Scenery` region and the wiring calls when you edit the layer next. The actor walk must start at `scenery.behind()`. `pet-pads` and the effects root both append at the end, which is fine for the walk.
- **C2 (stories) / C4 (pet-walk e2e)**: copies in the layer carry their source tags (`li`, `span`, `svg`, possibly `button`/`select` for run items). E2E's selector check (`a, button, input, … [role], [id]` inside the layer, E-PW:470) needs to skip `[inert]` copies.
- **C3 (quiz)**: put `data-pet-prop` keys on read-only rows only (design §22; never `tr`). The drag ghost of the quiz clones `data-pet-prop` (DOM §2.2): strip it in the ghost, or keep the `props` selector from matching `.quiz-drag-ghost`.
- **Not done**: P/README paragraph on lifting; A9's mutation tool not re-run after its two mutants moved to `DEPICTION`.
- **Deviation**: one sed edit of my own harness file `c1b_harness/main.ts` (a type rename, checked by grep afterwards) — all product files were changed with Write/Edit only.
