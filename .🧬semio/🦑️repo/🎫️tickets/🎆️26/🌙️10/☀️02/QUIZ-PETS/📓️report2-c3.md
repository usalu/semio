# 📓️ Work package C3 (second round): the quiz connected to the hand, the lifted copies and the keyboard play

Ticket `2026/10/02/QUIZ-PETS`, second round, phase C; design-v2 §14 (decisions 4–7), §17, §20, §22 "Quiz". `Q` = `🧰️framework/🛍️products/❓️quiz`, `QR` = `Q/🎯️targets/⚛️react`, `M` = `QR/🔨️modules`, `PR` = `🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react`, `S` = `🎓️teaching/🏛️architecture/❓️quiz`, `TK` = this ticket folder. Work 2026-10-03 12:45–13:55 (Windows, bun 1.4.2) beside B1–B3, `QUIZ-ADAPTIVE-LAYOUT` and the challenge ticket, which edited quiz files at the same time. Tool output: `TK/🗑️generated/c3/`.

**State: done.** The glue hands the layer `play`, `mischief`, `controls`, `props` and a `ref`; the quiz marks its topics; the two preferences and the "Play with the pets" group exist in English and German; the suites, the typecheck and a real browser drive are green. **One finding that is not mine but blocks a release:** the site's entry script is 268 525 B gzip, over the 260 000 B budget — it was already 267 286 B without C3 (§9); C3 adds 1 239 B.

## 1. Shared files edited (all with small anchored `Edit`s, each file re-read right before each edit)

| File | Where (lines as they are now) | What |
|---|---|---|
| `M/🐾️pets/🟦️.tsx` (the glue) | header 1–31; imports 33–36; `petNames` 77–80 (now via `petPlayers`); new region `🎾️Play` 83–113 (`PET_DEEDS`, `PetDeed`, `PetPlayer`, `PetPlay`, `petPlayers`); region `🗺️Stage` 129–157 (`QUIZ_PET_CONTROLS`, `QUIZ_PET_PROPS`, `petProp`, `PetTopic`, `usePetTopic`); `ShownPets` + `QuizPetsNotes` 197–227 (`play`, `mischief`, `handle`, `playing`); `QuizPetsProvider` 229–291 (props `play`, `mischief`, the handle, `ask`, `playing`); `usePetPlay` 293–296; `QuizPets` 345–371 (the new layer props) | the glue |
| `M/🎛️preferences/🟦️.tsx` | header 7–16; imports 24–28 (`cn`, `PET_DEEDS`, `usePetPlay`, `useAnnouncement`, types); `PET_DEED_LABELS` 58–59, `PET_DEED_SAID` 61–63; `QuizPreferences.petsPlay/petsMischief` 80–81; `readPreferences` docstring 85–88, body 104–105; `PreferencesPanel` docstring 183–191 and body (`playing`, `resting`, `restingId`, `allowance` 195–206; the two checkboxes, the note and the play row 236–257); new `PetsPlayground` 276–312 | preferences, play group |
| `M/🌐️i18n/🟦️.ts` | EN 127–138, DE 580–591 (twelve keys each, after `petsShown`) | strings |
| `M/🪟️chrome/🟦️.tsx` | `QuizCard` docstring 32–33, prop `topic` 38, `data-pet-topic` in `data` 62 | topic of a card |
| `M/📖️quiz-page/🟦️.tsx` | import 21; `QuizCardView` docstring 85–86 and `topic={quiz.id}` 96; `QuizPage` docstring 160–162; task row 220 | marks |
| `M/🗂️classification/🟦️.tsx` | import 27; docstring 32; `usePetTopic()` 36; chip 68 | marks |
| `M/↕️sorting/🟦️.tsx` | import 34; docstring 70; `usePetTopic()` 74; row 159 | marks |
| `M/🃏️matching/🟦️.tsx` | import 30; docstring 40–41; `usePetTopic()` 45; item row 132 | marks |
| `M/▶️run/🟦️.tsx` | import 44; `TaskBody` docstring 312–313; `<PetTopic>` around `SteadyTaskView` 355–357 | topic of a run's task |
| `M/🏁️results/🟦️.tsx` | import 29; `SortingResult` `usePetTopic()` 131 and true-order row 174; `ResultsScreen` docstring 255–257; `<PetTopic>` around `TaskResultView` 332–334 | marks |
| `QR/🟦️.tsx` | re-exports 101–129; provider props `play`/`mischief` 557 | wiring |
| `Q/🧪️tests/🐾️pet-companions/🟦️.tsx` | header, imports, `fakeStage` (handle that records deeds), fixtures (`SHEET` = `icons-1`, `OPEN_RUN`, `HOME_CATALOG`, `submittedRun`, `homeAt`, `idleSession`, `marked`, `named`), exact-props test, 12 new tests (§7) | suite |
| `Q/🧪️tests/🏠️home-grid/🟦️.tsx` l. 71, `Q/🧪️tests/📡️presence-client/🟦️.tsx` l. 845 | `petsPlay: true, petsMischief: true` in the pinned preference objects | pins |
| `Q/README.md` | `### Pets`: the Scene bullet (quiet), new bullets **Play and mischief**, **The hand**, **Topics**, **Play with the pets** (708–735), last sentence of **Stage** | docs |

Not touched: `QR/🎨️.css` (only Tailwind utilities in markup), the pets product, the site package, launch files, taxonomy (no new module or suite directory), `📋️project.json`s.

Ticket tools (kept): `TK/c3_selector_probe.ts` (jsdom understands the props selector), `TK/c3_code_rules.mjs` (docstring emojis against a baseline, no line comments, no console), `TK/c3_budget.ts` + `TK/c3_budget.vite.ts` (the pre-C3 build of §9), `TK/c3_preview_check.mjs` (the browser drive); updated `TK/quiz_pets_preview/{main.tsx, vite.config.ts}`. None is a permanent command, so nothing is registered in `launch.json`.

## 2. The glue as built

`<Layer>` gets, besides what it had: `ref={handle}` (a `useRef<PetLayerHandle>` of the provider; `PetLayerHandle` and `Deed` are **type-only** imports — the glue still imports no value of `@semio-tech/pets-react` or `@semio-tech/pets`), `play`, `mischief`, `controls={QUIZ_PET_CONTROLS}`, `props={QUIZ_PET_PROPS}`.

- **`play` / `mischief`** = the learner's `petsPlay` / `petsMischief` while the effective mode is `calm` or `lively`, `false` for both while it is `still` (also the reduced-motion default) — design §14.6. In a run the glue passes `quiet: true` and the two values unchanged: the stage decides what of it fits a time of concentration (§14.4); the glue only reports what the learner allows (tested: run → `quiet: true, play: true, mischief: true`; results → `quiet: false`).
- **`QUIZ_PET_CONTROLS = "[data-quiz-grip], [data-quiz-drop], [data-layered-card]"`**, verified against the DOM: the grip is `span[data-quiz-grip][aria-hidden]` with `onPointerDown` (`M/🧩️task:119-124`) — no default selector covers it; drop zones are `[data-quiz-drop]` sections, sorting rows and matching item rows; the overview's cards sit in `div[data-layered-card].contents` hosts in both strip and list mode (`🥞️LayeredOverview:411-421,767,793`), and `QuizCard`'s wrapper opens the page on a click anywhere but a control (`M/🪟️chrome:84-86`). Card actions are `<button data-overview-card-action>` (`🃏️OverviewCard:149-152`): already controls, not repeated. Presence anchors act on nothing but the grips (`👥️presence` `draggedItem`) and the home cards: covered. Every other `onClick`/`onPointerDown` in `QR` is on a `<button>`/`<a>` (grep, 13 hits). Consequence (in the README): a pet standing beside the title tab of an overview card stands in the card's own box (the cap row), so it is not picked up there; the settings play with it instead.
- **`QUIZ_PET_PROPS = "[data-pet-prop]:not(.quiz-drag-ghost *)"`** — the drag ghost is a shallow clone of the list with a deep clone of the row (`M/🤏️drag:29-42`) and so carries `data-pet-prop`; the selector leaves it out (C1b's note). Proven in jsdom (`TK/c3_selector_probe.ts`, then the suite with the real `startPointerDrag`) and used by Chromium in the drive.
- **Laziness:** the render target is still only `import("@semio-tech/pets-react")` (§9 proves the entry chunk has none of it).

## 3. Topic marks per screen

| Screen | Element | Key | Never |
|---|---|---|---|
| Overview | `section[data-overview-card][data-card="quiz:<q>"]` | **`data-pet-topic="<q>"`** (not `data-pet-prop`) | lifted: C1b's survey reads only `data-pet-prop` and cannot tell a card (`📡️survey:295-307`), so cards carry the topic in another attribute, for topic matching only. **Nothing consumes `data-pet-topic` yet** (no core rule matches surfaces to topics). |
| Opened quiz page | `[data-card="quiz-tasks"] ol > li` (the question stack) | `<q>/<task>` | — |
| Run, classification | chip `li[data-quiz-item]` (pool and bins) | `<q>/<task>/<item>` | — |
| Run, sorting | row `li[data-quiz-item]` | `<q>/<task>/<item>` | — |
| Run, matching | item row `li.quiz-slot` (one per dimension; `QUIZ-ADAPTIVE-LAYOUT` replaced the table by a list, so these are no table rows any more) | `<q>/<task>/<item>` | the value cards `li[data-quiz-drag]` (about no item) |
| Results | sorting's "true order" `ol > li` | `<q>/<task>/<item>` | every `tr` (classification, sorting, matching tables) |
| Run/task/results cards, stepper | — | — | cards are never lifted; the stepper holds buttons and is about tasks |

The quiz id the run and results DOM lacked (DOM §2.2) is in every key: `PetTopic` (a context provider, no element) names `<quiz>/<task>` around the task view (`TaskBody`) and around each task's results (`ResultsScreen`); the views compose `petProp(topic, item)`. Outside a `PetTopic` (e.g. a view rendered alone, the learner pages) nothing is marked. A key is about the topic only, never a value or correctness (rule 7).

## 4. Preferences

`QuizPreferences.petsPlay` and `.petsMischief`, both `true` unless the store holds an explicit `false` (`"false"`, `0`, `null` read as `true`), written with the rest of the preferences; neither touches `petsChosen` (the `petsChosen` marker logic does not apply). Two checkboxes right under the pets' row — "Pets react to clicks and can be picked up" / "Tierchen reagieren auf Klicks und lassen sich hochheben", "Pets may play with the page" / "Tierchen dürfen mit der Seite spielen" — each changing only its own field. While the effective mode is `off` or `still` (including the reduced-motion default) both are unchecked, `disabled` and `aria-describedby` the note "Only calm and lively pets play." / "Nur ruhige und lebhafte Tierchen spielen."; the stored values stay and return when the pets move again. Hooks `data-pets-allow="play|mischief"`.

## 5. "Play with the pets"

A `.quiz-setting` row under the two checkboxes (name column "Play with the pets" / "Mit den Tierchen spielen") with `role="group"` of that name, present only while `usePetPlay()` is defined: pets calm or lively, play allowed, somebody on stage (from `onCast`, names via `petPlayers`), a site with pets. Per pet a `role="group"` labelled by the pet's name (`aria-labelledby`), with four native buttons Hello / Trick / Pet / Toss — Hallo / Kunststück / Streicheln / Hochwerfen (worded differently in both languages for `🗣️both-languages`), calling `handle.current.play(species, deed)`. The names form a column (subgrid), the buttons wrap beside them. One `role="status"` (`aria-live="polite"`, atomic) line says only what the learner just asked for — "Sunny says hello." / "Sonni sagt Hallo.", "… does a trick." / "… macht ein Kunststück.", "… is petted." / "… wird gestreichelt.", "… is tossed up." / "… wird hochgeworfen." — a repeated deed is said again (`useAnnouncement`'s serial). Focus stays on the pressed button. Hooks `data-pets-play`, `data-pets-player="<species>"`.

## 6. i18n

Twelve keys per bundle, all `quiz.preferences.*`, used literally (`PET_DEED_LABELS`, `PET_DEED_SAID` are literal key maps like `PET_CHOICE_LABELS`): `petsPlay`, `petsMischief`, `petsResting`, `petsPlayWith`, `petsHello`, `petsTrick`, `petsPet`, `petsToss`, `petsHelloSaid`, `petsTrickSaid`, `petsPetSaid`, `petsTossSaid`. German informal, "Tierchen" throughout. `🗣️translation-completeness` green.

## 7. Tests (`Q/🧪️tests/🐾️pet-companions`, 33 → 45)

- Changed: the exact props the layer gets (now with `play`, `mischief`, `controls`, `props`, `ref: { current: { play } }`, one `ref` identity across renders); the preference pins (also in `🏠️home-grid`, `📡️presence-client`); the fake layer gets a handle that records deeds.
- New, "what the learner allows" (4): defaults/persistence/only-`false`/never a choice of liveliness; the two checkboxes in both languages (only their own field changes; off, disabled, described by the note under off and still, clicks and Space do nothing — user-event); what the layer gets for calm/lively/still, in a run and on results; nothing plays under the reduced-motion default (no group, disabled boxes, note).
- New, "the hand and the topics" (4): grips and drop zones are `QUIZ_PET_CONTROLS`, a card heading is not; `QUIZ_PET_PROPS` keeps the two chips and leaves out the ghost of a real `startPointerDrag`; the real `HomeScreen`: quiz cards carry `data-pet-topic` = quiz id, no `data-pet-prop`, lie in their `[data-layered-card]` control, nothing inside them is a prop; the opened page's task rows are `heating/u-values`, `heating/heating-load-and-demand` in `quiz-tasks`; the real `RunScreen` over the shared sheet `icons-1` (classification → matching with two dimensions → sorting via the stepper): exactly the item `li`s keyed `icon-demo/<task>/<item>`, no value card, no section, no table, nothing nested; the real `ResultsScreen`: only the true order, in rank order, never a table row.
- New, "play with the pets" (3): `PET_DEEDS` equals the core's `DEEDS`, labels in both languages and all different between them, `petPlayers`; groups per pet named by the pet with the four buttons, pointer and keyboard (Enter, Space, Tab — user-event) reach the layer's handle, the status says what was asked, in both languages; the group is there only for calm/lively, play allowed, somebody on stage (none in a scene without cast, an unknown species is left out), and not on a site without pets.
- New, "the client" (1): the real `QuizApp` with the real layer and the sample menagerie: the group names pets of the menagerie, Hello/Toss reach the real handle and the status line, the play box hides the group and is stored (`petsChosen` stays `false`), both boxes survive a remount, play on again brings the group back.

## 8. Docs

`Q/README.md` `### Pets` (§1). The pets product README already documents the props (C1a).

## 9. Commands and real results

From `QR/📦️packages/🟦️typescript` unless said otherwise.

| When | Command | Result |
|---|---|---|
| 12:49 baseline | `bun ./📜️script.ts test` | 22 files, **801 passed**, exit 0 |
| 13:16 | `bun ./📜️script.ts typecheck` (after the glue) | exit 0 |
| 13:20 | same | exit 1: one error in `🐾️pets/🔨️modules/🎯️choice/🟦️.ts(35,17)` `needsAfter` — the core, B-integrators in flight; gone at 13:30 |
| 13:20–13:41 | `bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts pet-companions translation-completeness` (four runs while writing the suite; the second failed on a `fireEvent.click` that jsdom delivers to a disabled checkbox, replaced by user-event) | final: **2 files, 57 passed** (45 + 12), no console output |
| 13:29 | `bun ./📜️script.ts test` · `typecheck` | **22 files, 813 passed** · exit 0 |
| 13:45 | same | 3 failed of 813 (`🚶️learner-journey` 2, `🫡️deputy-decisions` 1: expert-run clock openings, e.g. `expected { climates: 1760000001000 } to deeply equal { climates: 1760000003000 }`, `answer-invalid` vs `time-up`) and typecheck `M/🧭️session/🟦️.ts(301,59) TS2554` — the challenge ticket's `🧭️session` (written 13:47:07) and `Q/🔨️modules/⛰️challenge` (13:45:10) were being edited; no pets test, no file of mine |
| **13:47–13:48 final** | `bun ./📜️script.ts typecheck` · `bun ./📜️script.ts test` | **exit 0** · **22 files, 813 passed**, exit 0 |
| 13:47 final | `bun ./📜️script.ts test` in `PR/📦️packages/🟦️typescript` | **8 files, 166 passed**, exit 0 |
| 13:47 final | `bun ./📜️script.ts test` in `S/📦️packages/🟦️typescript` | **5 files, 138 passed**, exit 0 |
| 13:47 final | `NX_PLUGIN_NO_TIMEOUTS=true bun ./📜️script.ts build --outDir TK/🗑️generated/c3/site-release --emptyOutDir` in `S/📦️packages/🟦️typescript` (the site's own Vite config, production proctor origin; the site's `dist` untouched) | exit 0 |
| | `bun TK/site_bundle_weight.ts TK/🗑️generated/c3/site-release` | linked: entry `assets/🌐️-UdqxJGEe.js` 936 026 B, **gzip 268 525 B > 260 000 B**; stylesheet gzip 40 764 B; lazy: pets target + core `🟦️-CcdgO0ON.js` 143 395 B (51 344 gz), menagerie `🟦️-BBMIVWNk.js` 943 529 B (109 362 gz), schema/validation `🟦️-CztY9-Wv.js` 14 746 B (4 978 gz), pets stylesheet 1 501 B |
| | `bun TK/c3_budget.ts` then `bun node_modules/vite/bin/vite.js build --config TK/c3_budget.vite.ts --configLoader bundle --outDir TK/🗑️generated/c3/site-pre --emptyOutDir` (the same build with the eleven quiz sources as they were before C3: index text read by `git show :<path>` where the difference to the index is C3's alone — checked, the script refuses an index that already holds C3's edits — and the i18n module minus C3's 24 lines, since it also holds another ticket's unstaged `alreadyOpened`) | entry 931 673 B, **gzip 267 286 B** — already 7 286 B over budget before C3 |
| | difference | **C3 adds 4 353 B raw, 1 239 B gzip** to the entry (first pair of builds at ~13:33, before the subgrid layout of the group: 935 851 vs 931 609 B, gzip 268 457 vs 267 267 B, i.e. 4 242 / 1 190 B) |
| | entry chunk content (`grep`) | none of `pet-layer`, `pet-pads`, `data-pet-cursor`, `liftFixtures`, `watchGrasp`, `surveyed`, `summoned`, `permitted`, `openStage`, `semio.pets.menagerie`; it holds `import("./🟦️-CcdgO0ON.js")` (pets) and `import("./🟦️-BBMIVWNk.js")` (menagerie); the document links only the entry script and stylesheet. **No pets code in the entry.** |
| | `node TK/c3_code_rules.mjs` (with the baseline of repeated emojis taken before my first edit) | every file clean, no new repeated docstring emoji, exit 0 |
| | real browser: preview `TK/quiz_pets_preview` on **6257** (no HMR, no watch), `node TK/c3_preview_check.mjs` (Chromium 1280 × 860, sample menagerie, real layer and stage) | `failed: []`, console empty. On stage blobby, hoppy, floaty = the three groups, four buttons each. Deeds on blobby, each watched 2.5 s on `data-pet-activity`: Hello → `greet`; Trick → `trick` → `idle`; Pet → `purr`; Toss → `hang` → `tumble` → `glide`; the status line said "Blobby, the blob says hello." … "… is tossed up."; Enter on floaty's focused Hello → `greet`, focus kept. Resting pointer on a pet: `data-pet-cursor="grab"` with play on, none with play off, none while still; the group goes with play and comes back; still → no group, both boxes disabled, the note. German: "Blobby, der Klecks: Hallo \| Kunststück \| Streicheln \| Hochwerfen", status "Blobby, der Klecks macht ein Kunststück.". At 390 px the preview has no room for a pet: nobody on stage, so no group (and no sideways scroll). Marks seen: two task rows `heating/…`, two chips `heating/u-values/…`, one `SECTION` topic. Screenshots `TK/🗑️generated/c3/browser/{1-play-group, 2-after-deeds, 3-still, 4-group-en, 5-settings-de-narrow}.png`, data `browser/report.json` |

The first drive had console errors ("createRoot() on a container that has already been passed", `removeChild`): Vite HMR re-ran the preview's `main.tsx` when other sessions edited shared UI modules at 13:35:44 (`preview-server.txt`); the preview now runs with `hmr: false, watch: null`, and the drive is clean. Not run, as instructed: e2e gate, deploy-check, `bun install`. Ports 6061, 6063, 6069, 6074, 6161, 6162, 8791, 8793, 8891, 8892 untouched; 6257 was started and stopped by its PID (free at the end).

## 10. Decisions

1. **Cards get `data-pet-topic`, not `data-pet-prop`.** The survey cannot tell a card (it reads only the key). A `:not(...)` in `props` would have hidden the rule in a selector; a separate attribute keeps "lifted" and "matched" apart for every reader (stories, e2e). It is unconsumed today.
2. **Matching item rows are marked** — the brief excluded table rows, and the adaptive-layout ticket made them list rows; the value cards stay unmarked (no item).
3. **Drop zones are `controls`** as the brief asked: a press over a zone is the learner's (their drag model owns the area); pets are not supposed to stand there anyway (keep-outs). Card actions are buttons and are not repeated in the selector.
4. **`play`/`mischief` are `false` while still**, but not touched in a run (the core's job, §14.4). Under forced colours nothing changes in the glue: nobody is on stage, so the group is absent and the existing note explains.
5. **Provider props `play` and `mischief` are required**, like `chosen`.
6. **The group lives in the preferences module**, the glue only exposes `usePetPlay()` (`{ players, play }`): UI where the settings are, state where the pets are.
7. **Status sentences say what was asked** ("is petted", "is tossed up"), never how the pet answered — the stage may answer nothing (cooldown, `quiet`).
8. **The deeds are duplicated** as `PET_DEEDS` (no value import of the pets core into the entry chunk); the suite holds them equal to `DEEDS`.
9. **Budget measured differentially** (§9) instead of guessing whose growth it is.
10. **No language-agnostic vector case**: the glue is React-only (as WP-I decided for the first round); the third-party references are `@testing-library/user-event` for every keyboard and pointer claim, jsdom's selector engine and Chromium for the props selector, and the core's own `DEEDS`.
11. **Generated files stay** for the coordinator: deleting files is outside what I may do on my own; `TK/🗑️generated/c3/` holds two site builds (3.5 MB each), the preview's Vite cache (41 MB), `pre-variant.json` and logs — delete with `🗑️generated` when the ticket closes.

## 11. Open

- **Release budget (owner / whoever gates the release):** entry 268 525 B gzip > 260 000 B (`QUIZ_SITE_BUDGET.scriptGzipBytes`, `S/🚀️deploy/🟦️.ts:100`, checked by `siteArtifactProblems`, l. 347); without C3 267 286 B (other tickets' growth since R's 250 475 B on 2026-10-02 15:02). C3's 1 239 B could only shrink by moving the play group out of the entry chunk.
- **Site package (C4):** nothing changed in `S`; `S/README.md` `### Pets` could name play, mischief and the group. The site suite needs no change (138 green).
- **E2E `S/🧪️tests/🐕️pet-walk` (C4):** assert the group in both languages (named per pet, four buttons, the polite line) and that a deed changes `data-pet-activity`; play off → no `data-pet-cursor="grab"` over a pet, still → disabled boxes and the note; the overview's cards carry `data-pet-topic` and are never lifted (no copy in `.pet-layer` of a card), the quiz page's rows carry `<q>/<task>`; a press on a grip, a drop zone or a home card is never a pet's. **`🗣️both-languages`** compares the number of controls per page in both languages: the settings now hold 4 buttons per pet on stage, so a cast rotation between its two readings of the settings page would change the count (a rotation takes 60–120 s; the readings are seconds apart) — its owner may want to count outside `[data-pets-play]`.
- **Core (B5):** `data-pet-topic` has no consumer; a rule that prefers perches on cards of the pet's topic would use it. Mischief itself could not be watched in the browser: the core lifts nothing yet (C1b), and the grounds of the sample menagerie are descriptions ("sample menagerie of the pets product: the walker"), not quiz keys.
- `TK/quiz_pets_preview` is updated to the current preference shape and provider API (it still had R's shape with `moveBackground`), shows a marked task list and a classification in its topic, and runs without HMR.
